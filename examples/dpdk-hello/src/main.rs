extern crate dpdk;
use std::os::raw::c_void;

unsafe extern "C" fn hello_thread(_arg: *mut c_void) -> i32 {
    println!("Hello! lcore {}", dpdk::lcore::id());
    0
}

fn main() {
    unsafe {
        let _ = dpdk::eal::init(std::env::args());
        let callback_arg: *mut c_void = std::mem::zeroed();
        dpdk::eal::mp_remote_launch(hello_thread, callback_arg, true);
        dpdk::eal::mp_wait_lcore();
    }
}
