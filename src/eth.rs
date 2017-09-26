// rte_ethdev.h
use ffi;

pub type tx_buffer = ffi::rte_eth_dev_tx_buffer;

pub unsafe fn devices_get(port_id: u8) -> *const ffi::rte_eth_dev {
    ffi::rte_eth_devices.as_ptr().offset(port_id as isize)
}

pub unsafe fn rx_burst(port_id: u8, queue_id: u16,
                       rx_pkts: *mut *mut ffi::rte_mbuf,
                       nb_pkts: u16) -> i16 {
    let dev = *devices_get(port_id);
    let queue = *(*dev.data).rx_queues.offset(queue_id as isize)
        as *mut ::std::os::raw::c_void;
    let nb_rx = (dev.rx_pkt_burst.unwrap())(queue, rx_pkts, nb_pkts);
    nb_rx as i16
}

pub unsafe fn tx_burst(port_id: u8, queue_id: u16,
                       tx_pkts: *mut *mut ffi::rte_mbuf,
                       nb_pkts: u16) -> u16 {
    let dev = *devices_get(port_id);
    let queue = *(*dev.data).tx_queues.offset(queue_id as isize)
        as *mut ::std::os::raw::c_void;
    let nb_tx = (dev.tx_pkt_burst.unwrap())(queue, tx_pkts, nb_pkts);
    nb_tx
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
