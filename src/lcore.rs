// from rte_lcore.h
use ffi;

pub unsafe fn id() -> u32 {
    ffi::per_lcore__lcore_id
}

pub unsafe fn count() -> u32 {
    (*ffi::rte_eal_get_configuration()).lcore_count
}

pub unsafe fn get_master()-> u32 {
    (*ffi::rte_eal_get_configuration()).master_lcore
}

pub unsafe fn is_enabled(lcore_id: u32) -> bool {
    if lcore_id >= ffi::RTE_MAX_LCORE {
        return false;
    }
    let role = (*ffi::rte_eal_get_configuration()).lcore_role[lcore_id as usize];
    role == ffi::rte_lcore_role_t::ROLE_RTE
}

pub unsafe fn get_first(skip_master: bool) -> u32 {
    let mut i = 0u32;
    while i < ffi::RTE_MAX_LCORE {
        if is_enabled(i) == false ||
            (skip_master == true && i == get_master()) {
                i += 1;
            }
        break;
    }
    i
}

pub unsafe fn get_next(prev: u32, skip_master: bool,
                       wrap: bool) -> u32 {
    let mut i = prev + 1;
    if wrap == true {
        i %= ffi::RTE_MAX_LCORE;
    }
    while i < ffi::RTE_MAX_LCORE {
        if is_enabled(i) == false ||
            (skip_master == true && i == get_master()) {
                i += 1;
                if wrap == true {
                    i %= ffi::RTE_MAX_LCORE;
                }
            }
        break;
    }
    i
}
