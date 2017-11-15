# dpdk-hello, rust-dpdk example

## Building

To generate the bindings from scratch, use:

```
rustup run nightly cargo build
```
## Running

Run ./target/debug/dpdk-hello with DPDK parameter as root.

e.g.

```
sudo ./target/debug/dpdk-hello -cf -n2
```
