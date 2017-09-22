extern crate libc;
extern crate rust_dpdk;

use rust_dpdk::dpdk;
use std::ffi::CString;
use std::vec::Vec;

unsafe extern "C" fn hello_thread(arg: *mut std::os::raw::c_void) -> i32 {
    println!("Hello! lcore {}",  dpdk::rte_lcore_id());
    0
}

fn main() {
    unsafe {
        let mut port_conf: dpdk::rte_eth_conf = std::mem::zeroed();
        port_conf.intr_conf.set_lsc(1);
        let args: Vec<*mut i8> = std::env::args()
            .map(|arg| {
                CString::from_vec_unchecked(arg.into_bytes()).into_raw()
            })
            .collect();
        let argc = args.len() as i32;
        let argv = args.as_ptr() as *mut *mut i8;
        dpdk::rte_eal_init(argc, argv);
        let callback: dpdk::lcore_function_t = Some(hello_thread);
        let callback_arg: *mut std::os::raw::c_void = std::mem::zeroed();
        dpdk::rte_eal_mp_remote_launch(callback,
                                       callback_arg,
                                       dpdk::rte_rmt_call_master_t::CALL_MASTER);
        dpdk::rte_eal_mp_wait_lcore();
    }
}
