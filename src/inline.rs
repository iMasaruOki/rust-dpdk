// rte_lcore.h
pub unsafe fn rte_lcore_id() -> u32 {
    per_lcore__lcore_id
}

pub unsafe fn rte_lcore_count() -> u32 {
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

// rte_atomic.h
pub unsafe fn rte_atomic16_set(v: *mut rte_atomic16_t, new_value: u16) {
    ::std::intrinsics::atomic_store(v as *mut u16, new_value)
}

pub unsafe fn rte_atomic16_add_return(v: *mut rte_atomic16_t, inc: u16)
                                      -> u16 {
    ::std::intrinsics::atomic_xadd(v as *mut u16, inc) + inc
}

pub unsafe fn rte_atomic16_sub_return(v: *mut rte_atomic16_t, dec: u16)
                                      -> u16 {
    ::std::intrinsics::atomic_xsub(v as *mut u16, dec) - dec
}

// rte_ethdev.h
pub unsafe fn rte_eth_devices_get(port_id: u8) -> *const rte_eth_dev {
    rte_eth_devices.as_ptr().offset(port_id as isize)
}

pub unsafe fn rte_eth_rx_burst(port_id: u8, queue_id: u16,
                               rx_pkts: *mut *mut rte_mbuf,
                               nb_pkts: u16) -> i16 {
    let dev = *rte_eth_devices_get(port_id);
    let queue = *(*dev.data).rx_queues.offset(queue_id as isize)
        as *mut ::std::os::raw::c_void;
    let nb_rx = (dev.rx_pkt_burst.unwrap())(queue, rx_pkts, nb_pkts);
    nb_rx as i16
}

pub unsafe fn rte_eth_tx_burst(port_id: u8, queue_id: u16,
                               tx_pkts: *mut *mut rte_mbuf,
                               nb_pkts: u16) -> u16 {
    let dev = *rte_eth_devices_get(port_id);
    let queue = *(*dev.data).tx_queues.offset(queue_id as isize)
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
    rte_eth_tx_buffer_flush(port_id, queue_id, buffer)
}

// rte_mempool.h
pub unsafe fn rte_mempool_get_ops(ops_index: usize) -> *const rte_mempool_ops {
    &rte_mempool_ops_table.ops[ops_index]
}

pub unsafe fn rte_mempool_ops_enqueue_bulk(mp: *mut rte_mempool,
                                           obj_table: &[*mut ::std::os::raw::c_void],
                                           len: usize) {
    let ops = rte_mempool_get_ops((*mp).ops_index as usize);
    (*ops).enqueue.unwrap()(mp, obj_table.as_ptr() as *const *const ::std::os::raw::c_void, len as u32);
}

pub unsafe fn rte_mempool_generic_put(mp: *mut rte_mempool,
                                      obj_table: &[*mut ::std::os::raw::c_void],
                                      cache: *mut rte_mempool_cache) {
    if cache.is_null() || obj_table.len() > RTE_MEMPOOL_CACHE_MAX_SIZE as usize {
        rte_mempool_ops_enqueue_bulk(mp, obj_table, obj_table.len());
    } else {
        let mut cache_objs = (*cache).objs[(*cache).len as usize];
        memcpy(cache_objs,
               obj_table.as_ptr() as *mut ::std::os::raw::c_void,
               ::std::mem::size_of::<*mut u32>() * obj_table.len());
        (*cache).len += obj_table.len() as u32;
        if (*cache).len >= (*cache).flushthresh {
            rte_mempool_ops_enqueue_bulk(mp, &[(*cache).objs[(*cache).size as usize]],
                                         ((*cache).len - (*cache).size) as usize);
            (*cache).len = (*cache).size;
        }
    }
}

pub unsafe fn rte_mempool_default_cache(mp: *mut rte_mempool, lcore_id: u32)
                                        -> *mut rte_mempool_cache {
    (*mp).local_cache.offset(lcore_id as isize)
}

pub unsafe fn rte_mempool_put_bulk(mp: *mut rte_mempool, obj_table: &[*mut ::std::os::raw::c_void]) {
    let mut cache = rte_mempool_default_cache(mp, rte_lcore_id());
    rte_mempool_generic_put(mp, obj_table, cache);
}

pub unsafe fn rte_mempool_put(mp: *mut rte_mempool, obj: *mut ::std::os::raw::c_void) {
    let mut obj_table: [*mut ::std::os::raw::c_void; 1] = [obj];

    rte_mempool_put_bulk(mp, &obj_table);
}

// rte_mbuf.h
pub unsafe fn rte_mbuf_indirect(m: *const rte_mbuf) -> bool {
    (*m).ol_flags & IND_ATTACHED_MBUF != 0
}

pub unsafe fn rte_mbuf_raw_free(m: *mut rte_mbuf) {
    rte_mempool_put((*m).pool, m as *mut ::std::os::raw::c_void);
}

pub unsafe fn rte_pktmbuf_detach(_m: *mut rte_mbuf) {
    // not implemented yet
}

pub unsafe fn rte_mbuf_refcnt_read(m: *const rte_mbuf) -> u16 {
    (*m).__bindgen_anon_1.refcnt
}

pub unsafe fn rte_mbuf_refcnt_set(m: *mut rte_mbuf, new_value: u16) {
    rte_atomic16_set(&mut (*m).__bindgen_anon_1.refcnt_atomic,
                     new_value)
}

pub unsafe fn rte_mbuf_refcnt_update(m: *mut rte_mbuf, value: u16) -> u16 {
    if rte_mbuf_refcnt_read(m) == 1 {
        rte_mbuf_refcnt_set(m, 1 + value);
        return 1 + value;
    }
    rte_atomic16_add_return(&mut (*m).__bindgen_anon_1.refcnt_atomic, value)
}

pub unsafe fn rte_pktmbuf_prefree_seg(m: *mut rte_mbuf) -> *mut rte_mbuf {
    if rte_mbuf_refcnt_read(m) == 1 {
        if rte_mbuf_indirect(m) == true {
            rte_pktmbuf_detach(m);
        }
        if (*m).next.is_null() != true {
            (*m).next = 0 as *mut rte_mbuf;
            (*m).nb_segs = 1;
        }
        return m;
    } else if rte_atomic16_sub_return(&mut (*m).__bindgen_anon_1.refcnt_atomic, 1) == 0 {
    }
    0 as *mut rte_mbuf
}

pub unsafe fn rte_pktmbuf_free_seg(m: *mut rte_mbuf) {
    let mut m = rte_pktmbuf_prefree_seg(m);
    if m.is_null() != true {
        rte_mbuf_raw_free(m);
    }
}

pub unsafe fn rte_pktmbuf_free(m: *mut rte_mbuf) {
    let mut n = m;
    while n.is_null() != true {
        let mut m_next = (*n).next;
        rte_pktmbuf_free_seg(n);
        n = m_next;
    }
}
