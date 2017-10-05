extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=dpdk");
    let bindings = bindgen::Builder::default()
        .header("rte.h")
        .generate_inline_functions(true)
        .clang_arg(format!("-I{}/build/include", env::var("RTE_SDK").unwrap()))
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
        .arg("s/(^.*link_name.*per_lcore__)/    #[thread_local]\n$1/")
        .arg(out_path.join("ffi.rs").as_os_str())
        .spawn()
        .expect("failed to execute perl");
    let _ = child.wait();
}
