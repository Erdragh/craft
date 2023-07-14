# craft

[![Build](https://github.com/Erdragh/clifp/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/Erdragh/crafp/actions/workflows/build.yml)

**c**li - **r**ead - **a**nd - **f**ilter - **t**ool

This is a simple grep-like command line utility for finding a specific keyword in a file.

**NOT REALLY INTENDED FOR USE**, it's just my little try at creating a command line utility.

This is an overview on what this tool can do:

- [x] Filter by RegEx
- [x] Output the line and number of character where a match was found
- [x] Highlight these various pieces of information
- [x] Support for piping (`|`) the output from other command line programs. This utilizes the standard input.
- [x] Show a help page

# Usage
If you really want to use this, clone this repository and use cargo to build it:
```
git clone https://github.com/Erdragh/craft
```
```
cd craft
```
```
cargo build --release
```
The binary will be in `target/release`.

# Contributing

You'll have to be able to run `cargo` to build this project, which means having the Rust toolchain installed. The easiest way to do this is to use rustup.

Any Pull Request is welcome as long as it doesn't change basic functionality.
