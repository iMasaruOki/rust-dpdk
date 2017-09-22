pub unsafe extern "C" fn rte_lcore_count() -> u32 {
    (*rte_eal_get_configuration()).lcore_count
}

pub unsafe fn rte_get_master_lcore()-> u32 {
    (*rte_eal_get_configuration()).master_lcore
}

pub unsafe fn rte_lcore_is_enabled(lcore_id: u32) -> bool {
    if lcore_id >= RTE_MAX_LCORE {
        return false;
    }
    let role = (*rte_eal_get_configuration()).lcore_role[lcore_id as usize];
    role == rte_lcore_role_t::ROLE_RTE
}

pub unsafe fn rte_get_first_lcore(skip_master: bool) -> u32 {
    let mut i = 0u32;
    while i < RTE_MAX_LCORE {
        if rte_lcore_is_enabled(i) == false ||
            (skip_master == true && i == rte_get_master_lcore()) {
                i += 1;
            }
        break;
    }
    i
}

pub unsafe fn rte_get_next_lcore(prev: u32, skip_master: bool,
                                 wrap: bool) -> u32 {
    let mut i = prev + 1;
    if wrap == true {
        i %= RTE_MAX_LCORE;
    }
    while i < RTE_MAX_LCORE {
        if rte_lcore_is_enabled(i) == false ||
            (skip_master == true && i == rte_get_master_lcore()) {
                i += 1;
                if wrap == true {
                    i %= RTE_MAX_LCORE;
                }
            }
        break;
    }
    i
}

pub unsafe extern "C" fn rte_eth_rx_burst(port_id: u8, queue_id: u16,
                                          rx_pkts: *mut *mut rte_mbuf,
                                          nb_pkts: u16) -> i16 {
    let dev = rte_eth_devices[port_id as usize];
    let queue = (*dev.data).rx_queues.offset(queue_id as isize)
        as *mut ::std::os::raw::c_void;
    let nb_rx = (dev.rx_pkt_burst.unwrap())(queue, rx_pkts, nb_pkts);
    nb_rx as i16
}

pub unsafe fn rte_eth_tx_burst(port_id: u8, queue_id: u16,
                               tx_pkts: *mut *mut rte_mbuf,
                               nb_pkts: u16) -> u16 {
    let dev = *rte_eth_devices.as_ptr().offset(port_id as isize);
    let queue = (*dev.data).tx_queues.offset(queue_id as isize)
        as *mut ::std::os::raw::c_void;
    let nb_tx = (dev.tx_pkt_burst.unwrap())(queue, tx_pkts, nb_pkts);
    nb_tx
}

pub unsafe fn rte_eth_tx_buffer_flush(port_id: u8, queue_id: u16,
                                      buffer: *mut rte_eth_dev_tx_buffer) -> u16 {
    let to_send = (*buffer).length;
    if to_send == 0 {
        return 0;
    }
    let sent = rte_eth_tx_burst(port_id, queue_id,
                                (*buffer).pkts.as_mut_ptr(), to_send);
    (*buffer).length = 0;
    return sent;
}

pub unsafe fn rte_eth_tx_buffer(port_id: u8, queue_id: u16,
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
