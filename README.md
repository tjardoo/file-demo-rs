# Rust File Demo

This is a simple Rust application used to learn specific concepts.

- Command line arguments
- Recursive file search
- State machine pattern
- ANSI terminal colors
- SQLite database
- Pipelines
- Filtering
- Container
- Traits
- Macros

## Usage

```sh
# Find all files with the given name in the given directory with a depth of 2
cargo run find --path=/var/www --name=readme.md --depth=2

# List all files in the given directory with a depth of 2
cargo run list --path=/var/www --depth=2 --type=file

# Filter users by name and age (from SQLite database)
cargo run users --name=Judy --min=18 --max=25
```
