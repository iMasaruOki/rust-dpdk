extern crate dpdk;
use dpdk::ffi;

unsafe extern "C" fn hello_thread(_arg: *mut std::os::raw::c_void) -> i32 {
    println!("Hello! lcore {}",  dpdk::lcore::id());
    0
}

fn main() {
    unsafe {
        let _ = dpdk::eal::init(std::env::args());
        let callback: ffi::lcore_function_t = Some(hello_thread);
        let callback_arg: *mut std::os::raw::c_void = std::mem::zeroed();
        ffi::rte_eal_mp_remote_launch(callback,
                                      callback_arg,
                                      ffi::rte_rmt_call_master_t::CALL_MASTER);
        ffi::rte_eal_mp_wait_lcore();
    }
}
