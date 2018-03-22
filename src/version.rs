use std::str;
use ffi;

pub fn string() -> String {
    if ffi::RTE_VER_SUFFIX[0] == 0 {
        format!("{} {}.{:02}.{}",
                str::from_utf8(ffi::RTE_VER_PREFIX).unwrap(),
                ffi::RTE_VER_YEAR,
                ffi::RTE_VER_MONTH,
                ffi::RTE_VER_MINOR)
    } else {
        format!("{} {}.{:02}.{}{}{}",
                str::from_utf8(ffi::RTE_VER_PREFIX).unwrap(),
                ffi::RTE_VER_YEAR,
                ffi::RTE_VER_MONTH,
                ffi::RTE_VER_MINOR,
                str::from_utf8(ffi::RTE_VER_SUFFIX).unwrap(),
                if ffi::RTE_VER_RELEASE < 16 {
                    ffi::RTE_VER_RELEASE
                } else {
                    (ffi::RTE_VER_RELEASE as i32 - 16) as u32
                })
    }
}
