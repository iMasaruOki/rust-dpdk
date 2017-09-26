// rte_atomic.h
use ffi::rte_atomic16_t;

pub unsafe fn set16(v: *mut rte_atomic16_t, new_value: u16) {
    ::std::intrinsics::atomic_store(v as *mut u16, new_value)
}

pub unsafe fn add16_return(v: *mut rte_atomic16_t, inc: u16)
                           -> u16 {
    ::std::intrinsics::atomic_xadd(v as *mut u16, inc) + inc
}

pub unsafe fn sub16_return(v: *mut rte_atomic16_t, dec: u16)
                           -> u16 {
    ::std::intrinsics::atomic_xsub(v as *mut u16, dec) - dec
}
