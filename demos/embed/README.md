Embed Example
=============

This crate will create an shared object that will add extra
functions to q:

* `sleep <val>`: sleep thread for `<val>` ms
* `mean <array>`: same as `avg`
* `dictex`: example create dictionary
* `tableex`: example create table
* `printer <val>`: print debug info about K object


Usage
-----
Ensure that the main rkdb crate is building correctly.

Run `cargo build --release` to create `target/release/libembed.so`.

Copy `libembed.so` to project root.

Run `q embed.q`. The above functions will be defined in the root namespace.

Note: Rust version and q version must be same 'bit' i.e. 64 bit.
