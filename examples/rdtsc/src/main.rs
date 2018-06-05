extern crate dpdk;
use std::{thread, time};

fn printtsc(hz: u64, now: u64, base: u64) {
    println!("{} ({})", now, (now as f64 - base as f64) / hz as f64);
}


fn main() {
    unsafe {
        let z: u64;
        let o: u64;
        let t: u64;
        let _ = dpdk::eal::init(std::env::args());
        let hz = dpdk::lcore::tsc_hz();
        assert_ne!(hz, 0);
        z = dpdk::cycles::rdtsc();
        printtsc(hz, z, z);
        thread::sleep(time::Duration::from_millis(1000));
        o = dpdk::cycles::rdtsc();
        printtsc(hz, o, z);
        thread::sleep(time::Duration::from_millis(3000));
        t = dpdk::cycles::rdtsc();
        printtsc(hz, t, z);
    }
}
