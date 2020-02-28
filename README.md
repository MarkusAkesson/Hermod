Secure File Transfer Protocol

Hermod is a research project and should be treated as such.
We take no responsibility for any security flaws or any data lost due to using Hermod.

## Build

Hermod is written in rust and uses cargo for building and installing.

In order to build Hermod a working installation of Rust is needed
Hermod only sypports Rust 1.41 and newer.

```shell
# Check that you have the latest version of rust
rustup update

# Build Hermod
cargo build

# Run debug version
cargo run -- ARGUMENTS

# Install
cargo install --path .
```

## Using Hermod
```
Hermod File Transefer util
Hermod is a file transfer util that utilises the Hermod file transfer protocol

USAGE:
    hermod <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    download     Download a file or files from the remote server
    gen-key      Generate static keys for the client and a new client-token
    help         Prints this message or the help of the given subcommand(s)
    server       Start a server
    share-key    Generate and immediately share keys and id with the specified host
    upload       Upload a file or files to the remote server
```

## Testing

Testing uses docker for automated testing.
```shell
# Build docker images and run tests
./test.sh build

# Run tests
./test.sh
```
