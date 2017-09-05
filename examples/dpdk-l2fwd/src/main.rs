extern crate libc;
extern crate getopts;
extern crate rust_dpdk;

use rust_dpdk::dpdk;
use std::ffi::CString;
use std::vec::Vec;
use getopts::Options;

static force_quit: bool = false;

const MAX_PKT_BURST:u16 = 32;

unsafe extern "C" fn l2fwd_main_loop(arg: *mut std::os::raw::c_void) -> i32 {
    let lcore_id = dpdk::per_lcore__lcore_id;
    let mut pkts: [*mut dpdk::rte_mbuf; MAX_PKT_BURST as usize];
    let mut buffer: dpdk::rte_eth_dev_tx_buffer = std::mem::zeroed();
    let in_port = arg as u8;
    let out_port = in_port ^ 1;

    pkts = std::mem::zeroed();
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
        let mut argc = args.len() as i32;
        let mut argv = args.as_ptr() as *mut *mut i8;
        let nparam = dpdk::rte_eal_init(argc, argv);
        if nparam < 0 {
            panic!("Invalid EAL arguments");
        }
        let mut opts = Options::new();
        opts.optopt("p", "", "set port bitmap", "PORT");
        let args: Vec<String> = std::env::args().collect();
        let (_, exarg) = args.split_at(nparam as usize);
        let matches = match opts.parse(exarg) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()) }
        };
        let mut portmap = matches.opt_str("p").unwrap().parse::<u32>().unwrap();
        let mut ports: Vec<u8> = Vec::new();
        let mut lcores: Vec<u32> = Vec::new();
        let mut n = 0u8;
        let mut lc = dpdk::rte_get_first_lcore(false);
        while portmap > 0 {
            if portmap & 1 != 0 {
                if lc == dpdk::RTE_MAX_LCORE {
                    panic!("Not enough logical core.");
                }
                ports.push(n);
                lcores.push(lc);
                lc = dpdk::rte_get_next_lcore(lc, false, false);
            }
            portmap /= 2;
            n += 1;
        }
        if n % 2 != 0 {
            panic!("Cannot assign odd ports.");
        }
        pktmbuf_pool = dpdk::rte_pktmbuf_pool_create("mbufpool".as_ptr() as *const i8,
                                                     8192,
                                                     256,
                                                     0,
                                                     dpdk::RTE_MBUF_DEFAULT_BUF_SIZE as u16,
                                                     dpdk::rte_socket_id() as i32);
        for portid in ports.clone() {
            dpdk::rte_eth_dev_configure(portid, 1, 1, &port_conf);
            let mut nb_rxd: u16 = 128;
            let mut nb_txd: u16 = 512;
            dpdk::rte_eth_dev_adjust_nb_rx_tx_desc(portid, &mut nb_rxd, &mut nb_txd);
            dpdk::rte_eth_rx_queue_setup(portid, 0, nb_rxd, dpdk::rte_eth_dev_socket_id(portid) as u32, 0 as *mut dpdk::rte_eth_rxconf, pktmbuf_pool);
            dpdk::rte_eth_tx_queue_setup(portid, 0, nb_txd, dpdk::rte_eth_dev_socket_id(portid) as u32, 0 as *mut dpdk::rte_eth_txconf);
        }
        let callback: dpdk::lcore_function_t = Some(l2fwd_main_loop);
        for n in 0..lcores.len() {
            let callback_arg = ports[n] as *mut std::os::raw::c_void;
            dpdk::rte_eal_remote_launch(callback,
                                        callback_arg,
                                        lcores[n]);
        }
        dpdk::rte_eal_mp_wait_lcore();
    }
}
