// rte_mempool.h
extern crate libc;

use std::os::raw::c_void;
use ffi;
use lcore;

pub unsafe fn get_ops(ops_index: usize) -> *const ffi::rte_mempool_ops {
    &ffi::rte_mempool_ops_table.ops[ops_index]
}

pub unsafe fn ops_enqueue_bulk(mp: *mut ffi::rte_mempool, obj_table: &[*mut c_void], len: usize) {
    let ops = get_ops((*mp).ops_index as usize);
    (*ops).enqueue.unwrap()(mp, obj_table.as_ptr() as *const *const c_void, len as u32);
}

pub unsafe fn generic_put(
    mp: *mut ffi::rte_mempool,
    obj_table: &[*mut c_void],
    cache: *mut ffi::rte_mempool_cache,
) {
    if cache.is_null() || obj_table.len() > ffi::RTE_MEMPOOL_CACHE_MAX_SIZE as usize {
        ops_enqueue_bulk(mp, obj_table, obj_table.len());
    } else {
        libc::memcpy(
            (*cache).objs.as_mut_ptr().offset((*cache).len as isize) as *mut libc::c_void,
            obj_table.as_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<*mut u32>() * obj_table.len(),
        );
        (*cache).len += obj_table.len() as u32;
        if (*cache).len >= (*cache).flushthresh {
            ops_enqueue_bulk(
                mp,
                &[(*cache).objs[(*cache).size as usize]],
                ((*cache).len - (*cache).size) as usize,
            );
            (*cache).len = (*cache).size;
        }
    }
}

pub unsafe fn default_cache(
    mp: *mut ffi::rte_mempool,
    lcore_id: u32,
) -> *mut ffi::rte_mempool_cache {
    (*mp).local_cache.offset(lcore_id as isize)
}

pub unsafe fn put_bulk(mp: *mut ffi::rte_mempool, obj_table: &[*mut c_void]) {
    let cache = default_cache(mp, lcore::id());
    generic_put(mp, obj_table, cache);
}

pub unsafe fn put(mp: *mut ffi::rte_mempool, obj: *mut c_void) {
    let obj_table: [*mut c_void; 1] = [obj];

    put_bulk(mp, &obj_table);
}
