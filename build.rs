extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dpdk");
    let bindings = bindgen::Builder::default()
        .header("rte.h")
        .clang_arg(format!("-I{}/build/include", env::var("RTE_SDK").unwrap()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("dpdk.rs"))
        .expect("Cooudn't write bindings");
}
