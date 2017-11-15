// rte_mbuf.h
use ffi;
use ffi::rte_mbuf;
use mempool;
use atomic;
use std::os::raw::c_void;

impl rte_mbuf {

    pub unsafe fn detach(&self) {
        // not implemented yet
    }

    pub unsafe fn indirect(&self) -> bool {
        self.ol_flags & ffi::IND_ATTACHED_MBUF != 0
    }

    pub unsafe fn raw_free(&mut self) {
        mempool::put(self.pool, (&mut *self as *mut rte_mbuf) as *mut c_void);
    }

    pub unsafe fn refcnt(&self) -> u16 {
        self.__bindgen_anon_2.refcnt
    }

    pub unsafe fn refcnt_set(&mut self, new_value: u16) {
        atomic::set16(&mut self.__bindgen_anon_2.refcnt_atomic, new_value)
    }

    pub unsafe fn refcnt_update(&mut self, value: u16) -> u16 {
        if self.refcnt() == 1 {
            self.refcnt_set(1 + value);
            return 1 + value;
        }
        atomic::add16_return(&mut self.__bindgen_anon_2.refcnt_atomic, value)
    }

    pub unsafe fn refcnt_inc(&mut self) -> u16 {
        self.refcnt_update(1)
    }
}
