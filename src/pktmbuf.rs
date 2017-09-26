// rte_mbuf.h
use ffi;
use ffi::rte_mbuf;
use mbuf;
use atomic;
use std::ffi::CString;

pub unsafe fn pool_create(name: &'static str,
                          n: u32,
                          cache_size: u32,
                          priv_size: u16,
                          data_room_size: u16,
                          socket_id: i32) -> *mut ffi::rte_mempool {
    ffi::rte_pktmbuf_pool_create(CString::new(name).unwrap().into_raw(),
                                 n,
                                 cache_size,
                                 priv_size,
                                 data_room_size,
                                 socket_id)
}

pub unsafe fn detach(_m: *mut rte_mbuf) {
    // not implemented yet
}

pub unsafe fn prefree_seg(m: *mut rte_mbuf) -> *mut rte_mbuf {
    if mbuf::refcnt_read(m) == 1 {
        if mbuf::indirect(m) == true {
            detach(m);
        }
        if (*m).next.is_null() != true {
            (*m).next = 0 as *mut rte_mbuf;
            (*m).nb_segs = 1;
        }
        return m;
    } else if atomic::sub16_return(&mut (*m).__bindgen_anon_1.refcnt_atomic, 1) == 0 {
    }
    0 as *mut rte_mbuf
}

pub unsafe fn free_seg(m: *mut rte_mbuf) {
    let mut m = prefree_seg(m);
    if m.is_null() != true {
        mbuf::raw_free(m);
    }
}

pub unsafe fn free(m: *mut rte_mbuf) {
    let mut n = m;
    while n.is_null() != true {
        let mut m_next = (*n).next;
        free_seg(n);
        n = m_next;
    }
}
