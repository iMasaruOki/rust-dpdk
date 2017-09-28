// rte_ethdev.h
use ffi;
use std::ffi::CString;

pub type tx_buffer = ffi::rte_eth_dev_tx_buffer;

pub unsafe fn devices(port_id: u8) -> &'static ffi::rte_eth_dev {
    &*ffi::rte_eth_devices.as_ptr().offset(port_id as isize)
}

impl ffi::rte_eth_dev {
    pub unsafe fn is_intr_lsc_enable(&self) -> bool {
        ((*self.data).dev_flags & ffi::RTE_ETH_DEV_INTR_LSC) != 0
    }
    pub unsafe fn name(&self) -> String {
        CString::from_raw((*self.data).name.as_mut_ptr())
            .into_string()
            .unwrap()
    }
}

pub unsafe fn configure(port_id: u8, nb_rxd: u16, nb_txd: u16,
                        port_conf: &ffi::rte_eth_conf) -> i32 {
    ffi::rte_eth_dev_configure(port_id, nb_rxd, nb_txd,
                               port_conf as *const ffi::rte_eth_conf)
}

pub unsafe fn socket_id(port_id: u8) -> u32 {
    ffi::rte_eth_dev_socket_id(port_id) as u32
}

pub unsafe fn rx_queue_setup(port_id: u8, queue_id: u16, nb_rxd: u16,
                             socket_id: u32, rxconf: *mut ffi::rte_eth_rxconf,
                             pool: *mut ffi::rte_mempool)
                             -> i32 {
    ffi::rte_eth_rx_queue_setup(port_id, queue_id, nb_rxd, socket_id,
                                rxconf, pool)
}

pub unsafe fn tx_queue_setup(port_id: u8, queue_id: u16, nb_txd: u16,
                             socket_id: u32, txconf: *mut ffi::rte_eth_txconf)
                             -> i32 {
    ffi::rte_eth_tx_queue_setup(port_id, queue_id, nb_txd, socket_id, txconf)
}

pub unsafe fn start(port_id: u8) -> i32 {
    ffi::rte_eth_dev_start(port_id)
}

pub unsafe fn stop(port_id: u8) {
    ffi::rte_eth_dev_stop(port_id);
}

pub unsafe fn promiscuous_set(port_id: u8, onoff: bool) {
    if onoff == true {
        ffi::rte_eth_promiscuous_enable(port_id);
    } else {
        ffi::rte_eth_promiscuous_disable(port_id);
    }
}

pub unsafe fn rx_burst(port_id: u8, queue_id: u16,
                       rx_pkts: *mut *mut ffi::rte_mbuf,
                       nb_pkts: u16) -> i16 {
    let dev = devices(port_id);
    let queue = *(*dev.data).rx_queues.offset(queue_id as isize)
        as *mut ::std::os::raw::c_void;
    let nb_rx = (dev.rx_pkt_burst.unwrap())(queue, rx_pkts, nb_pkts);
    nb_rx as i16
}

pub unsafe fn tx_burst(port_id: u8, queue_id: u16,
                       tx_pkts: *mut *mut ffi::rte_mbuf,
                       nb_pkts: u16) -> u16 {
    let dev = devices(port_id);
    let queue = *(*dev.data).tx_queues.offset(queue_id as isize)
        as *mut ::std::os::raw::c_void;
    (dev.tx_pkt_burst.unwrap())(queue, tx_pkts, nb_pkts)
}

impl tx_buffer {
    pub unsafe fn init(&mut self, size: u16) {
        ffi::rte_eth_tx_buffer_init(self as *mut tx_buffer, size);
    }

    pub unsafe fn flush(&mut self, port_id: u8, queue_id: u16) -> u16 {
        let to_send = self.length;
        if to_send == 0 {
            return 0;
        }
        let sent = tx_burst(port_id, queue_id,
                            self.pkts.as_mut_ptr(), to_send);
        self.length = 0;
        sent
    }

    pub unsafe fn tx(&mut self, port_id: u8, queue_id: u16,
                     tx_pkt: *mut ffi::rte_mbuf) -> u16 {
        *self.pkts.as_mut_ptr().offset(self.length as isize) = tx_pkt;
        self.length += 1;
        if self.length < self.size {
            return 0;
        }
        self.flush(port_id, queue_id)
    }
}
