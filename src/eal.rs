use std::vec::Vec;
use std::option::Option;
use std::ffi::CString;
use ffi;

pub unsafe fn init(args: ::std::env::Args) -> Option<Vec<String>> {
    let argstrs: Vec<String> = args.collect();
    let mut arg_iter = argstrs.split(|str| str == "--");
    let args: Vec<*mut i8> = arg_iter
        .next()
        .unwrap()
        .iter()
        .map(|arg| CString::new(arg.as_bytes()).unwrap().into_raw())
        .collect();
    let argc = args.len() as i32;
    let argv = args.as_ptr() as *mut *mut i8;
    let nparam = ffi::rte_eal_init(argc, argv);
    assert!(nparam >= 0, "Invalid EAL arguments");
    let next = arg_iter.next();
    if next != None {
        return Some(next.unwrap().to_vec());
    }
    None
}

type lcore_func_t = unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> i32;

pub unsafe fn remote_launch(
    callback: lcore_func_t,
    arg: *mut ::std::os::raw::c_void,
    lcore: u32,
) -> i32 {
    ffi::rte_eal_remote_launch(Some(callback), arg, lcore)
}

pub unsafe fn mp_remote_launch(
    callback: lcore_func_t,
    arg: *mut ::std::os::raw::c_void,
    call_master: bool,
) -> i32 {
    let cm;
    if call_master == true {
        cm = ffi::rte_rmt_call_master_t_CALL_MASTER;
    } else {
        cm = ffi::rte_rmt_call_master_t_SKIP_MASTER;
    }
    ffi::rte_eal_mp_remote_launch(Some(callback), arg, cm)
}

pub unsafe fn mp_wait_lcore() {
    ffi::rte_eal_mp_wait_lcore()
}
