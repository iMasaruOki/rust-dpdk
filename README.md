# rust-dpdk

Rust bindings to [DPDK](http://dpdk.org/), currently at version 18.08.

## Requiement

perl

## Building

At first, build and install DPDK as shared library.
Below build instruction is for gcc.  If use clang, replace 'gcc' with 'clang'.

```
tar xzvf dpdk-18.08.tar.gz
cd dpdk-18.02
echo CONFIG_RTE_BUILD_SHARED_LIB=y >> config/defconfig_x86_64-native-linuxapp-gcc
make T=x86_64-native-linuxapp-gcc config
make
sudo make install
```

NOTE: So far, we need nightly build of Rust to use #[thread_local] feature.

To generate the bindings from scratch, use:

```
cargo +nightly build
```

To use within your own project, use:

```
[dependencies.rust-dpdk]
git = "https://github.com/iMasaruOki/rust-dpdk.git"
```
