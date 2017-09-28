use std::vec::Vec;
use std::option::Option;
use std::ffi::CString;
use ffi;

pub unsafe fn init(args: ::std::env::Args)
                   -> Option<Vec<String>> {
    let argstrs: Vec<String> = args.collect();
    let mut arg_iter = argstrs.split(|str| str == "--");
    let args: Vec<*mut i8> = arg_iter.next().unwrap().iter()
        .map(|arg| {
            CString::new(arg.as_bytes()).unwrap().into_raw()
        })
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
