extern crate bindgen;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=dpdk");
    let bindings = bindgen::Builder::default()
        .header("rte.h")
        .generate_inline_functions(true)
        .clang_arg(format!("-I{}/build/include", env::var("RTE_SDK").unwrap()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("tmp.rs"))
        .expect("Cooudn't write bindings");
    let mut child = Command::new("perl")
        .arg("-pi")
        .arg("-e")
        .arg("s/(^.*link_name.*per_lcore__)/    #[thread_local]\n$1/")
        .arg(out_path.join("tmp.rs").as_os_str())
        .spawn()
        .expect("failed to execute perl");
    let exitcode = child.wait();
    let output = Command::new("/bin/cat")
        .arg(out_path.join("tmp.rs").as_os_str())
        .arg(out_path.join("inline.rs").as_os_str())
        .output()
        .expect("failed to execute cat");
    let mut file = File::create(out_path.join("dpdk.rs").as_os_str());
    file.ok().unwrap().write_all(&output.stdout);
    fs::remove_file(out_path.join("tmp.rs").as_os_str());
}
