# rust-dpdk

Rust bindings to [DPDK](http://dpdk.org/), currently at version 17.08.

## Building

To generate the bindings from scratch, use:

```
env RTE_SDK=path_to_dpdk_top cargo build
```

To use within your own project, use:

```
[dependencies.rust-dpdk]
git = "https://github.com/iMasaruOki/rust-dpdk.git"
```
