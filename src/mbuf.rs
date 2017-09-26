// rte_mbuf.h
use ffi;
use ffi::rte_mbuf;
use mempool;
use atomic;

pub type mbuf = rte_mbuf;

pub unsafe fn indirect(m: *const mbuf) -> bool {
    (*m).ol_flags & ffi::IND_ATTACHED_MBUF != 0
}

pub unsafe fn raw_free(m: *mut mbuf) {
    mempool::put((*m).pool, m as *mut ::std::os::raw::c_void);
}

pub unsafe fn refcnt_read(m: *const rte_mbuf) -> u16 {
    (*m).__bindgen_anon_1.refcnt
}

pub unsafe fn refcnt_set(m: *mut rte_mbuf, new_value: u16) {
    atomic::set16(&mut (*m).__bindgen_anon_1.refcnt_atomic, new_value)
}

pub unsafe fn refcnt_update(m: *mut rte_mbuf, value: u16) -> u16 {
    if refcnt_read(m) == 1 {
        refcnt_set(m, 1 + value);
        return 1 + value;
    }
    atomic::add16_return(&mut (*m).__bindgen_anon_1.refcnt_atomic, value)
}
