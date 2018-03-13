extern crate bindgen;

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=dpdk");
    let bindings = bindgen::Builder::default()
        .header("rte.h")
        .generate_inline_functions(true)
        .clang_arg("-I/usr/local/include/dpdk")
        .clang_arg("-Wno-error=implicit-function-declaration")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Cooudn't write bindings");
    let mut child = Command::new("perl")
        .arg("-pi")
        .arg("-e")
        .arg("s/(pub static mut per_lcore__lcore_id)/    #[thread_local] $1/")
        .arg(out_path.join("ffi.rs").as_os_str())
        .spawn()
        .expect("failed to execute perl");
    let _ = child.wait();
}
