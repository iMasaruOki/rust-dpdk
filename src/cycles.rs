// from rte_cycles.h

#[cfg(target_arch = "arm")]
use libc;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn rdtsc() -> u64 {
    let tsc_lo32: u32;
    let tsc_hi32: u32;
    asm!("rdtsc" : "={eax}" (tsc_lo32), "={edx}" (tsc_hi32));
    (tsc_hi32 as u64) << 32 | tsc_lo32 as u64
}

#[cfg(target_arch = "arm")]
pub unsafe fn rdtsc() -> u64 {
    let mut tv: libc::timespec;
    libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, &tv);
    tv.tv_sec as u64 * 10000000000 + tv.tv_nsec as u64
}
