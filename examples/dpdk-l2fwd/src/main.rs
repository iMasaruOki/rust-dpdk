extern crate libc;
extern crate rust_dpdk;

use rust_dpdk::dpdk;
use std::ffi::CString;
use std::vec::Vec;

static force_quit: bool = false;

const MAX_PKT_BURST:u16 = 32;

unsafe extern "C" fn l2fwd_main_loop(arg: *mut std::os::raw::c_void) -> i32 {
    let lcore_id = dpdk::per_lcore__lcore_id;
    let mut pkts: [*mut dpdk::rte_mbuf; MAX_PKT_BURST as usize] = std::mem::zeroed();
    let mut buffer: dpdk::rte_eth_dev_tx_buffer = std::mem::zeroed();
    let in_port = 0u8;
    let out_port = 1u8;

    dpdk::rte_eth_tx_buffer_init(&mut buffer, MAX_PKT_BURST);

    while force_quit != true {
        let nb_rx = dpdk::rte_eth_rx_burst(in_port, 0,
                                           pkts.as_mut_ptr(),
                                           MAX_PKT_BURST);
        for i in 0..nb_rx as usize {
            let sent = dpdk::rte_eth_tx_buffer(out_port, 0, &mut buffer, pkts[i]);
        }
    }
    0
}

static mut pktmbuf_pool: *mut dpdk::rte_mempool = 0 as *mut dpdk::rte_mempool;

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
        let nparam = dpdk::rte_eal_init(argc, argv);
        if nparam < 0 {
            dpdk::rte_exit(dpdk::EXIT_FAILURE as i32,
                           "Invalid EAL arguments".as_ptr() as *const i8);
        }
        pktmbuf_pool = dpdk::rte_pktmbuf_pool_create("mbufpool".as_ptr() as *const i8,
                                                     8192,
                                                     256,
                                                     0,
                                                     dpdk::RTE_MBUF_DEFAULT_BUF_SIZE as u16,
                                                     dpdk::rte_socket_id() as i32);
        for portid in 0..1 {
            dpdk::rte_eth_dev_configure(portid, 1, 1, &port_conf);
            let mut nb_rxd: u16 = 128;
            let mut nb_txd: u16 = 512;
            dpdk::rte_eth_dev_adjust_nb_rx_tx_desc(portid, &mut nb_rxd, &mut nb_txd);
            dpdk::rte_eth_rx_queue_setup(portid, 0, nb_rxd, dpdk::rte_eth_dev_socket_id(portid) as u32, 0 as *mut dpdk::rte_eth_rxconf, pktmbuf_pool);
            dpdk::rte_eth_tx_queue_setup(portid, 0, nb_txd, dpdk::rte_eth_dev_socket_id(portid) as u32, 0 as *mut dpdk::rte_eth_txconf);
        }
        let callback: dpdk::lcore_function_t = Some(l2fwd_main_loop);
        let callback_arg: *mut std::os::raw::c_void = std::mem::zeroed();
        dpdk::rte_eal_mp_remote_launch(callback,
                                       callback_arg,
                                       dpdk::rte_rmt_call_master_t::CALL_MASTER);
        dpdk::rte_eal_mp_wait_lcore();
    }
}
