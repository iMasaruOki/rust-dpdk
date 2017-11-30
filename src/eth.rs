// rte_ethdev.h
use ffi;
use std::ffi::CString;

#[derive(Copy)]
pub struct Port {
    pub port_id: u16,
}

impl Clone for Port {
    fn clone(&self) -> Port {
        *self
    }
}

pub type tx_buffer = ffi::rte_eth_dev_tx_buffer;

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

impl Port {
    pub unsafe fn devices(&self) -> &'static ffi::rte_eth_dev {
        &*ffi::rte_eth_devices.as_ptr().offset(self.port_id as isize)
    }

    pub unsafe fn info(&self, info: &mut ffi::rte_eth_dev_info) {
        ffi::rte_eth_dev_info_get(self.port_id, info as *mut ffi::rte_eth_dev_info);
    }

    pub unsafe fn configure(&self, nb_rxd: u16, nb_txd: u16, port_conf: &ffi::rte_eth_conf) -> i32 {
        ffi::rte_eth_dev_configure(
            self.port_id,
            nb_rxd,
            nb_txd,
            port_conf as *const ffi::rte_eth_conf,
        )
    }

    pub unsafe fn socket_id(&self) -> u32 {
        ffi::rte_eth_dev_socket_id(self.port_id) as u32
    }

    pub unsafe fn adjust_rx_desc(&self, nb_rxd: u16) -> u16 {
        let mut nbr = nb_rxd;
        let mut nbt = nb_rxd;
        let rv = ffi::rte_eth_dev_adjust_nb_rx_tx_desc(self.port_id, &mut nbr, &mut nbt);
        assert_eq!(rv, 0);
        nbr
    }

    pub unsafe fn adjust_tx_desc(&self, nb_txd: u16) -> u16 {
        let mut nbr = nb_txd;
        let mut nbt = nb_txd;
        let rv = ffi::rte_eth_dev_adjust_nb_rx_tx_desc(self.port_id, &mut nbr, &mut nbt);
        assert_eq!(rv, 0);
        nbt
    }

    pub unsafe fn start(&self) -> i32 {
        ffi::rte_eth_dev_start(self.port_id)
    }

    pub unsafe fn stop(&self) {
        ffi::rte_eth_dev_stop(self.port_id);
    }

    pub unsafe fn promiscuous_set(&self, onoff: bool) {
        if onoff == true {
            ffi::rte_eth_promiscuous_enable(self.port_id);
        } else {
            ffi::rte_eth_promiscuous_disable(self.port_id);
        }
    }

    pub unsafe fn rx_queue_setup(
        &self,
        queue_id: u16,
        nb_rxd: u16,
        socket_id: u32,
        rxconf: *mut ffi::rte_eth_rxconf,
        pool: *mut ffi::rte_mempool,
    ) -> i32 {
        ffi::rte_eth_rx_queue_setup(self.port_id, queue_id, nb_rxd, socket_id, rxconf, pool)
    }

    pub unsafe fn tx_queue_setup(
        &self,
        queue_id: u16,
        nb_txd: u16,
        socket_id: u32,
        txconf: *mut ffi::rte_eth_txconf,
    ) -> i32 {
        ffi::rte_eth_tx_queue_setup(self.port_id, queue_id, nb_txd, socket_id, txconf)
    }

    pub unsafe fn rx_burst(
        &self,
        queue_id: u16,
        rx_pkts: *mut &mut ffi::rte_mbuf,
        nb_pkts: u16,
    ) -> i16 {
        let dev = self.devices();
        let queue = *(*dev.data).rx_queues.offset(queue_id as isize) as *mut ::std::os::raw::c_void;
        let nb_rx = (dev.rx_pkt_burst.unwrap())(queue, rx_pkts as *mut *mut ffi::rte_mbuf, nb_pkts);
        nb_rx as i16
    }

    pub unsafe fn tx_burst(
        &self,
        queue_id: u16,
        tx_pkts: *mut *mut ffi::rte_mbuf,
        nb_pkts: u16,
    ) -> u16 {
        let dev = self.devices();
        let queue = *(*dev.data).tx_queues.offset(queue_id as isize) as *mut ::std::os::raw::c_void;
        (dev.tx_pkt_burst.unwrap())(queue, tx_pkts, nb_pkts)
    }
}

impl tx_buffer {
    pub unsafe fn init(&mut self, size: u16) {
        ffi::rte_eth_tx_buffer_init(self as *mut tx_buffer, size);
    }

    pub unsafe fn flush(&mut self, port: &Port, queue_id: u16) -> u16 {
        let to_send = self.length;
        if to_send == 0 {
            return 0;
        }
        let sent = port.tx_burst(queue_id, self.pkts.as_mut_ptr(), to_send);
        self.length = 0;
        sent
    }

    pub unsafe fn tx(&mut self, port: &Port, queue_id: u16, tx_pkt: *mut ffi::rte_mbuf) -> u16 {
        *self.pkts.as_mut_ptr().offset(self.length as isize) = tx_pkt;
        self.length += 1;
        if self.length < self.size {
            return 0;
        }
        self.flush(port, queue_id)
    }
}
