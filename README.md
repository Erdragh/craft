# clifp
**CLI** - **f**ilter - **p**rogram

This is a simple grep-like command line utility for finding a specific keyword in a file.

**NOT REALLY INTENDED FOR USE**, it's just my little try at creating a command line utility.

This is an overview on what this tool can and can't do:

- [x] Find a keyword in a file
- [x] Output the line and number of character where the keyword was found
- [x] Highlight these various pieces of information
- [ ] Support for piping (`|`) the output from other command line programs
- [ ] RegEx instead of simple keywords
- [ ] A help page

# Usage
If you really want to use this, clone this repository and use cargo to build it:
```
git clone https://github.com/Erdragh/clifp
```
```
cd clifp
```
```
cargo build --release
```
The binary will be in `target/release`. Put this wherever you want and you can execute it from there - I think.

# Contributing
You'll have to have rustup installed, so you have cargo. I use VSCode with various Rust extensions, but you can also use bare vim from the command line with `cargo run` to run your code. 

Any Pull Request is welcome as long as it doesn't compromise anything or change anything about this.

*Side Note:* I really don't expect any PRs as this is literally just a project for me to learn Rust and CLI programming a bit.