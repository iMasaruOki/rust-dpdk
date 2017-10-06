# dpdk-l2fwd, rust-dpdk example like as dpdk/src/examples/l2fwd

## Building

To generate the bindings from scratch, use:

```
rustup run nightly cargo build
```
## Running

Run ./target/debug/dpdk-l2fwd as root with DPDK l2fwd parameter.

e.g.

```
sudo ./target/debug/dpdk-l2fwd -cf -n2 -- -p3
```
