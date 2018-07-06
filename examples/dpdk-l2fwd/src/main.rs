extern crate getopts;
extern crate dpdk;

#[macro_use]
extern crate lazy_static;

use std::vec::Vec;
use getopts::Options;
use std::sync::Mutex;
use dpdk::ffi;
use std::os::raw::c_void;

static mut FORCE_QUIT: bool = false;
static mut DUMP_FLAG: bool = false;

const MAX_PKT_BURST: u16 = 32;

lazy_static! {
    static ref PORTS: Mutex<Vec<dpdk::eth::Port>> = Mutex::new(vec![]);
}

fn dump_packet_type(ptype: u32) {
    match ptype & ffi::RTE_PTYPE_L2_MASK {
        ffi::RTE_PTYPE_L2_ETHER => print!("Ether,"),
        ffi::RTE_PTYPE_L2_ETHER_VLAN => print!("Ether+VLAN,"),
        ffi::RTE_PTYPE_L2_ETHER_QINQ => print!("Ether+QinQ,"),
        ffi::RTE_PTYPE_L2_ETHER_ARP => print!("Ether+ARP,"),
        _ => print!("Other L2 ({}),", ptype & ffi::RTE_PTYPE_L2_MASK),
    }
    match ptype & ffi::RTE_PTYPE_L3_MASK {
        ffi::RTE_PTYPE_L3_IPV4 => print!("IPv4,"),
        ffi::RTE_PTYPE_L3_IPV4_EXT => print!("IPv4-Ext,"),
        ffi::RTE_PTYPE_L3_IPV6 => print!("IPv6,"),
        ffi::RTE_PTYPE_L3_IPV6_EXT => print!("IPv6-Ext,"),
        _ => print!("Other L3 ({}),", ptype & ffi::RTE_PTYPE_L3_MASK),
    }
    match ptype & ffi::RTE_PTYPE_L4_MASK {
        ffi::RTE_PTYPE_L4_UDP => println!("UDP"),
        ffi::RTE_PTYPE_L4_TCP => println!("TCP"),
        ffi::RTE_PTYPE_L4_SCTP => println!("SCTP"),
        ffi::RTE_PTYPE_L4_ICMP => println!("ICMP"),
        ffi::RTE_PTYPE_L4_FRAG => println!("Fragment"),
        _ => println!("Other L4 ({})", ptype & ffi::RTE_PTYPE_L4_MASK),
    }
}

unsafe fn dump_mbuf(m: &ffi::rte_mbuf) {
    let mut hdr_lens: ffi::rte_net_hdr_lens = std::mem::zeroed();
    let ptype = dpdk::net::get_ptype(m, &mut hdr_lens, ffi::RTE_PTYPE_ALL_MASK);
    dump_packet_type(ptype);
    print!("l2_len {},", hdr_lens.l2_len);
    print!("l3_len {},", hdr_lens.l3_len);
    println!("l4_len {}", hdr_lens.l4_len);
}

unsafe extern "C" fn l2fwd_main_loop(arg: *mut c_void) -> i32 {
    let lcore_id = dpdk::lcore::id();
    let mut pkts: [&mut ffi::rte_mbuf; MAX_PKT_BURST as usize];
    let mut buffers: [dpdk::eth::tx_buffer; ffi::RTE_MAX_ETHPORTS as usize];
    let in_port = dpdk::eth::Port { port_id: arg as u16 };

    pkts = std::mem::zeroed();
    buffers = std::mem::zeroed();
    for buffer in buffers.iter_mut() {
        buffer.init(MAX_PKT_BURST);
    }

    println!("lcore{}: loop start", lcore_id);
    while FORCE_QUIT != true {
        let nb_rx = in_port.rx_burst(0, pkts.as_mut_ptr(), MAX_PKT_BURST);
        if nb_rx == 0 {
            continue;
        }
        for out_port in PORTS.lock().unwrap().iter() {
            if out_port.port_id == in_port.port_id {
                continue;
            }
            let buffer = &mut buffers[out_port.port_id as usize];
            for i in 0..nb_rx as usize {
                if DUMP_FLAG == true {
                    dump_mbuf(pkts[i]);
                }
                pkts[i].refcnt_update(1);
                let sent = buffer.tx(out_port, 0, pkts[i]);
                if sent < 1 {
                    let new_refcnt = pkts[i].refcnt() - 1;
                    pkts[i].refcnt_set(new_refcnt);
                }
            }
            buffer.flush(out_port, 0);
        }
        for pkt in 0..nb_rx {
            dpdk::pktmbuf::free(pkts[pkt as usize]);
        }
    }
    0
}


fn main() {
    unsafe {
        let pool: *mut ffi::rte_mempool;
        let mut opts = Options::new();
        opts.optopt("p", "portmap", "set port bitmap", "PORTMAP");
        opts.optflag("d", "dump", "show packet mbuf dump");
        opts.optflag("v", "version", "show version and exit");

        let exargs = dpdk::eal::init(std::env::args());
        if exargs.is_none() == true {
            println!("parameter required.");
            return;
        }
        let matches = match opts.parse(exargs.unwrap()) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };
        if matches.opt_present("v") {
            println!("dpdk-l2fwd {} with {}",
                     env!("CARGO_PKG_VERSION"),
                     dpdk::version::string());
            return
        }
        if matches.opt_present("d") {
            DUMP_FLAG = true;
        }
        let mut portmap = matches.opt_str("p").unwrap().parse::<u32>().unwrap();
        // lcore and port assignment
        let mut lcores: Vec<u32> = Vec::new();
        let mut n = 0u16;
        let mut lc = dpdk::lcore::get_first(true);
        while portmap > 0 {
            if portmap & 1 != 0 {
                if lc == ffi::RTE_MAX_LCORE {
                    panic!("Not enough logical core.");
                }
                println!("portid {}: lcore {}", n, lc);
                PORTS.lock().unwrap().push(dpdk::eth::Port { port_id: n });
                lcores.push(lc);
                lc = dpdk::lcore::get_next(lc, false, false);
            }
            portmap /= 2;
            n += 1;
        }
        pool = dpdk::pktmbuf::pool_create(
            "mbufpool",
            8192,
            256,
            0,
            ffi::RTE_MBUF_DEFAULT_BUF_SIZE as u16,
            dpdk::socket::id(),
        );
        assert!(pool.is_null() == false);
        let mut port_conf: ffi::rte_eth_conf = std::mem::zeroed();
        port_conf.rxmode.set_hw_strip_crc(1);
        for port in PORTS.lock().unwrap().clone() {
            let mut info: ffi::rte_eth_dev_info = std::mem::zeroed();
            port.info(&mut info);
            let device = port.devices();
            println!("Initializing port {}: name {}", port.port_id, device.name());
            if device.is_intr_lsc_enable() == true {
                port_conf.intr_conf.set_lsc(1);
            } else {
                port_conf.intr_conf.set_lsc(0);
            }
            let rv = port.configure(1, 1, &port_conf);
            assert!(
                rv == 0,
                "configure failed: portid {}, rv: {}",
                port.port_id,
                rv
            );
            let nb_rxd = port.adjust_rx_desc(128);
            let nb_txd = port.adjust_tx_desc(512);
            let rv =
                port.rx_queue_setup(0, nb_rxd, port.socket_id(), &mut info.default_rxconf, pool);
            assert!(
                rv == 0,
                "rx queue setup failed: portid {}, rv: {}",
                port.port_id,
                rv
            );
            let rv = port.tx_queue_setup(0, nb_txd, port.socket_id(), &mut info.default_txconf);
            assert!(
                rv == 0,
                "tx queue setup failed: portid {}, rv: {}",
                port.port_id,
                rv
            );
            let rv = port.start();
            assert!(
                rv == 0,
                "ethernet devvice not started: portid {}, rv: {}",
                port.port_id,
                rv
            );
            port.promiscuous_set(true);
        }
        let callback = l2fwd_main_loop;
        for n in 0..lcores.len() {
            let callback_arg = PORTS.lock().unwrap()[n].port_id as *mut c_void;
            dpdk::eal::remote_launch(callback, callback_arg, lcores[n]);
        }
        dpdk::eal::mp_wait_lcore();
    }
}
