#![allow(bad_style,
         missing_copy_implementations,
         improper_ctypes)]
#![feature(thread_local, core_intrinsics)]

pub mod ffi;

pub mod atomic;
pub mod eal;
pub mod eth;
pub mod lcore;
pub mod mbuf;
pub mod mempool;
pub mod pktmbuf;
pub mod ring;
pub mod socket;
