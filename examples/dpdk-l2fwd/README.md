# dpdk-l2fwd, rust-dpdk example like as dpdk/src/examples/l2fwd

## Requirement

DPDK 17.11 built as shared library.
Instruction, see ../../README.md

## Building

To generate the bindings from scratch, use:

```
cargo +nightly build
```
## Running

Run ./target/debug/dpdk-l2fwd as root with DPDK l2fwd parameter.

e.g.

```
sudo ./target/debug/dpdk-l2fwd -cf -n2 -- -p3
```
