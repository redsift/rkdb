# rkdb

kdb+ bindings for Rust. Originally forked from and substantially based on the Krust cargo package by [adwhit](https://github.com/adwhit/krust) via [robsmith11](https://github.com/robsmith11/krust).

[![Circle CI](https://img.shields.io/circleci/project/redsift/rkdb.svg?logo=circleci)](https://circleci.com/gh/redsift/rkdb)
[![Latest version](https://img.shields.io/crates/v/rkdb.svg)](https://crates.io/crates/rkdb)
![License](https://img.shields.io/crates/l/rkdb.svg)

These bindings enable Rust to be used as inside Q to add additional functionality.
More generally they also enable Rust to communicate with kdb+ in a memory-safe way.

In addition to Krust, this variant features:
- Support for macOS
- Support for the GUID type and sd0x function call
- Fix for kerror strings when exported from Rust to Q
- Fixes for embedding
- Performance features
- Dynamic export for symbols to be weak linked
- Helpers for querying attributes on types

This variant incorporates [robsmith11](https://github.com/robsmith11/krust) updates for the current rust nightly.

For an example of how to embed Rust code within Q, see `demos/embed`.
For an example of how to perform IPC between Rust and Q, see `demos/ipc`.

## Building for Embedding

When built for embedding, the symbols are provided by the hosting Q process and the library must **not** include the duplicated functions. This requires weak linking which currently needs to be manually archived on macOS by specifying all the missing symbols that should be ignored at link time. This symbols are generated at build time as the constant `rkdb::SYMBOLS` and is currently used by setting `.cargo/config` in the library target to:

```
[target.'cfg(target_os="macos")']
rustflags = ["-Clink-args=-Wl,-U,_ktn -Wl,-U,_knk -Wl,-U,_ku -Wl,-U,_ka -Wl,-U,_kb -Wl,-U,_kg -Wl,-U,_kh -Wl,-U,_ki -Wl,-U,_kj -Wl,-U,_ke -Wl,-U,_kf -Wl,-U,_kc -Wl,-U,_ks -Wl,-U,_kd -Wl,-U,_kz -Wl,-U,_kt -Wl,-U,_ktj -Wl,-U,_kp -Wl,-U,_kpn -Wl,-U,_xT -Wl,-U,_xD -Wl,-U,_ktd -Wl,-U,_ss -Wl,-U,_sn -Wl,-U,_ymd -Wl,-U,_dj -Wl,-U,_setm -Wl,-U,_r1 -Wl,-U,_r0 -Wl,-U,_m9 -Wl,-U,_sd1 -Wl,-U,_sd0 -Wl,-U,_sd0x -Wl,-U,_k -Wl,-U,_dl -Wl,-U,_ja -Wl,-U,_js -Wl,-U,_jk -Wl,-U,_jv -Wl,-U,_krr -Wl,-U,_orr -Wl,-U,_dot -Wl,-U,_okx -Wl,-U,_b9 -Wl,-U,_d9"]

```
### Example

Reference [kdb-rs-hash](https://github.com/redsift/kdb-rs-hash) for an example of embedding.

## Building for IPC

To use this library for IPC, you will first need to
compile a static library from the kx-supplied object file `c.o` using `ar`. 
The static library should be placed in `src/c/libkdb.a`.

```
ar rcs libkdb.a c.o
mv libkdb.so src/c
```

`c.o` can be found [here](http://code.kx.com/wsvn/code).

## Compatibility

- 64 bit i.e. commercial version of Q >= 3.6.
- macOS (with XCode >=  9.4.1) and Linux.
- rustc >= 1.31.0

## License

Based substantially on [adwhit](https://github.com/adwhit/krust) and licensed under the same MIT license.
