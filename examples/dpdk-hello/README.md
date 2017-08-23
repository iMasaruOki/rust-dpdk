# dpdk-hello, rust-dpdk example

## Building

To generate the bindings from scratch, use:

```
env RTE_SDK=path_to_dpdk_top cargo build
```
## Running

Run ./target/debug/dpdk-hello with DPDK parameter as root.

e.g.

```
sudo ./target/debug/dpdk-hello -cf -n2
```
