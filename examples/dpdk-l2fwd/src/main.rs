extern crate getopts;
extern crate dpdk;

#[macro_use]
extern crate lazy_static;

use std::vec::Vec;
use getopts::Options;
use std::sync::Mutex;
use dpdk::ffi;
use std::os::raw::c_void;

static force_quit: bool = false;

const MAX_PKT_BURST:u16 = 32;

lazy_static! {
    static ref PORTS: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

unsafe extern "C" fn l2fwd_main_loop(arg: *mut c_void) -> i32 {
    let lcore_id = dpdk::lcore::id();
    let mut pkts: [*mut dpdk::ffi::rte_mbuf; MAX_PKT_BURST as usize];
    let mut buffers: [dpdk::eth::tx_buffer; ffi::RTE_MAX_ETHPORTS as usize];
    let in_port = arg as u8;

    pkts = std::mem::zeroed();
    buffers =  std::mem::zeroed();
    for buffer in buffers.iter_mut() {
        buffer.init(MAX_PKT_BURST);
    }

    println!("lcore{}: loop start", lcore_id);
    while force_quit != true {
        let nb_rx = dpdk::eth::rx_burst(in_port, 0,
                                        pkts.as_mut_ptr(),
                                        MAX_PKT_BURST);
        if nb_rx == 0 {
            continue;
        }
        for out_port in PORTS.lock().unwrap().iter() {
            if *out_port == in_port {
                continue;
            }
            let buffer = &mut buffers[*out_port as usize];
            for i in 0..nb_rx as usize {
                (*pkts[i]).refcnt_update(1);
                let sent = buffer.tx(*out_port, 0, pkts[i]);
                if sent < 1 {
                    (*pkts[i]).refcnt_set((*pkts[i]).refcnt() - 1);
                }
            }
            buffer.flush(*out_port, 0);
        }
        for pkt in pkts.iter() {
            dpdk::pktmbuf::free(*pkt);
        }
    }
    0
}


fn main() {
    unsafe {
        let pktmbuf_pool: *mut ffi::rte_mempool;
        let mut opts = Options::new();
        opts.optopt("p", "", "set port bitmap", "PORT");

        let exargs = dpdk::eal::init(std::env::args());
        if exargs.is_none() == true {
            println!("parameter required.");
            return;
        }
        let matches = match opts.parse(exargs.unwrap()) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()) }
        };
        let mut portmap = matches.opt_str("p")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        // lcore and port assignment
        let mut lcores: Vec<u32> = Vec::new();
        let mut n = 0u8;
        let mut lc = dpdk::lcore::get_first(true);
        while portmap > 0 {
            if portmap & 1 != 0 {
                if lc == ffi::RTE_MAX_LCORE {
                    panic!("Not enough logical core.");
                }
                println!("portid {}: lcore {}", n, lc);
                PORTS.lock().unwrap().push(n);
                lcores.push(lc);
                lc = dpdk::lcore::get_next(lc, false, false);
            }
            portmap /= 2;
            n += 1;
        }
        pktmbuf_pool = dpdk::pktmbuf::pool_create("mbufpool",
                                                  8192,
                                                  256,
                                                  0,
                                                  ffi::RTE_MBUF_DEFAULT_BUF_SIZE as u16,
                                                  ffi::rte_socket_id() as i32);
        assert!(pktmbuf_pool.is_null() == false);
        let mut port_conf: ffi::rte_eth_conf = std::mem::zeroed();
        port_conf.rxmode.set_hw_strip_crc(1);
        for portid in PORTS.lock().unwrap().clone() {
            let mut info: ffi::rte_eth_dev_info = std::mem::zeroed();
            ffi::rte_eth_dev_info_get(portid, &mut info as *mut ffi::rte_eth_dev_info);
            let device = dpdk::eth::devices(portid);
            println!("Initializing port {}: name {}", portid, device.name());
            if device.is_intr_lsc_enable() == true {
                port_conf.intr_conf.set_lsc(1);
            } else {
                port_conf.intr_conf.set_lsc(0);
            }
            let rv = dpdk::eth::configure(portid, 1, 1, &port_conf);
            assert!(rv == 0,
                    "configure failed: portid {}, rv: {}", portid, rv);
            let mut nb_rxd: u16 = 128;
            let mut nb_txd: u16 = 512;
            let rv = ffi::rte_eth_dev_adjust_nb_rx_tx_desc(portid, &mut nb_rxd, &mut nb_txd);
            assert!(rv == 0,
                    "rte_eth_dev_adjust_nb_rx_tx_desc failed: portid {}, rv: {}", portid, rv);
            let rv = dpdk::eth::rx_queue_setup(portid, 0, nb_rxd,
                                               dpdk::eth::socket_id(portid),
                                               0 as *mut ffi::rte_eth_rxconf,
                                               pktmbuf_pool);
            assert!(rv == 0,
                    "rx queue setup failed: portid {}, rv: {}", portid, rv);
            let rv = dpdk::eth::tx_queue_setup(portid, 0, nb_txd,
                                               dpdk::eth::socket_id(portid),
                                               0 as *mut ffi::rte_eth_txconf);
            assert!(rv == 0,
                    "tx queue setup failed: portid {}, rv: {}", portid, rv);
            let rv = dpdk::eth::start(portid);
            assert!(rv == 0,
                    "ethernet devvice not started: portid {}, rv: {}",
                    portid, rv);
            dpdk::eth::promiscuous_set(portid, true);
        }
        let callback: ffi::lcore_function_t = Some(l2fwd_main_loop);
        for n in 0..lcores.len() {
            let callback_arg = PORTS.lock().unwrap()[n] as *mut c_void;
            ffi::rte_eal_remote_launch(callback, callback_arg,lcores[n]);
        }
        ffi::rte_eal_mp_wait_lcore();
    }
}
