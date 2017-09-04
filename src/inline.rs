pub unsafe extern "C" fn rte_eth_rx_burst(port_id: u8, queue_id: u16,
                                          rx_pkts: *mut *mut rte_mbuf,
                                          nb_pkts: u16) -> i16 {
    let dev = rte_eth_devices[port_id as usize];
    let queue = (*dev.data).rx_queues.offset(queue_id as isize);
    let nb_rx = (dev.rx_pkt_burst.unwrap())(queue as *mut ::std::os::raw::c_void,
                                            rx_pkts, nb_pkts);
    nb_rx as i16
}

pub unsafe extern "C" fn rte_eth_tx_burst(port_id: u8, queue_id: u16,
                                          tx_pkts: *mut *mut rte_mbuf,
                                          nb_pkts: u16) -> u16 {
    let dev = rte_eth_devices[port_id as usize];
    let queue = (*dev.data).tx_queues.offset(queue_id as isize);
    let nb_tx = (dev.tx_pkt_burst.unwrap())(queue as *mut ::std::os::raw::c_void,
                                            tx_pkts, nb_pkts);
    nb_tx
}

pub unsafe extern "C" fn rte_eth_tx_buffer_flush(port_id: u8, queue_id: u16,
                                                 buffer: *mut rte_eth_dev_tx_buffer) -> u16 {
    let to_send = (*buffer).length;
    if to_send == 0 {
        return 0;
    }
    let sent = rte_eth_tx_burst(port_id, queue_id, (*buffer).pkts.as_mut_ptr(), to_send);
    (*buffer).length = 0;
    return sent;
}

pub unsafe extern "C" fn rte_eth_tx_buffer(port_id: u8, queue_id: u16,
                                           buffer: *mut rte_eth_dev_tx_buffer,
                                           tx_pkt: *mut rte_mbuf) -> u16 {
    *(*buffer).pkts.as_mut_ptr().offset((*buffer).length as isize) = tx_pkt;
    (*buffer).length += 1;
    if (*buffer).length < (*buffer).size {
        return 0;
    }
    let nb_tx = rte_eth_tx_buffer_flush(port_id, queue_id, buffer);
    return nb_tx;
}
