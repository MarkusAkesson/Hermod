# Hermod
Secure File Transfer Protocol

Hermod is research project and should be treated as such.
We take no responsibility for any security flaws or any lost data due to using Hermod.

## Build

Hermod is written in rust and uses cargo for building and testing.

In order to build Hermod a working installation of Rust is needed.
Hermod only supports Rust 1.41 and newer.

```shell
# Check that you have the latest version of rust
rustup update

# Build Hermod
cargo build

# Test
cargo test

# Run
cargo run -- ARGUMENTS

# Install
cargo install --path .
```

The program is still under development and the current version does NOT support any transfers or key generation.
