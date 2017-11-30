// from rte_net.h
use ffi;

pub unsafe fn get_ptype(
    m: &ffi::rte_mbuf,
    hdr_lens: &mut ffi::rte_net_hdr_lens,
    layers: u32,
) -> u32 {
    ffi::rte_net_get_ptype(
        m as *const ffi::rte_mbuf,
        hdr_lens as *mut ffi::rte_net_hdr_lens,
        layers,
    )
}
