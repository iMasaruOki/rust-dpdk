extern crate rust_dpdk;

use rust_dpdk::dpdk;

unsafe extern "C" fn hello_thread(_arg: *mut std::os::raw::c_void) -> i32 {
    println!("Hello! lcore {}",  dpdk::rte_lcore_id());
    0
}

fn main() {
    unsafe {
        let _ = dpdk::eal_init(std::env::args());
        let callback: dpdk::lcore_function_t = Some(hello_thread);
        let callback_arg: *mut std::os::raw::c_void = std::mem::zeroed();
        dpdk::rte_eal_mp_remote_launch(callback,
                                       callback_arg,
                                       dpdk::rte_rmt_call_master_t::CALL_MASTER);
        dpdk::rte_eal_mp_wait_lcore();
    }
}
