# Rust File Demo

This is a simple demo of a Rust application that lists/finds files in a directory.

The aim of the demo is to only use the standard library and not use any external crates.

## Usage

```sh
# Find all files with the given name in the given directory with a depth of 2
cargo run find --directory=/var/www --name=readme.md --depth=2

# List all files in the given directory with a depth of 2
cargo run list --directory=/var/www --depth=2 --type=file
```
