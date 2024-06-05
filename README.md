# Rust File Demo

This is a simple demo of a Rust application that lists/finds files in a directory.

The aim of the demo is to only use the standard library and not use any external crates.

## Usage

```sh
# Find all files in the current directory and subdirectories that match the name "readme.md"
cargo run -- find readme.md

# List all files in the current directory and subdirectories with the depth of 2
cargo run -- list 2
```
