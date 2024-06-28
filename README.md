# ESP32 Hello World Example (with defmt)

## Setup

 -  Rust `=1.77` : The `rustup` command to install toolchains. ([link][install-rust])
 -  `espup` : The ESP32 toolchain installation ([link][install-espup])

> This crate compiles for `no_std` environments.



## Compile

```sh
$ rustup run esp cargo build --release;
```



## Problem

In the Cargo.toml, if the `patch.crates-io` is :
 - **not commented** : it compiles with the commands above
 - **commented** : it does not compile with the following error :

    ```
    --> /home/ferris/.cargo/registry/src/index.crates.io-6f17d22bba15001f/embedded-hal-1.0.0/src/lib.rs:21:5
    |
    21 | use defmt as defmt;
    |     ^^^^^^^^^^^^^^ no external crate `defmt`

    For more information about this error, try `rustc --explain E0432`.
    error: could not compile `embedded-hal` (lib) due to 1 previous error
    warning: build failed, waiting for other jobs to finish...
    ```




[install-rust]: https://www.rust-lang.org/tools/install
[install-espup]: https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html