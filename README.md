# aoc2023-cpp

Advent of Code 2023 solutions, written in Rust.

## Build

This project uses [`cargo`](https://github.com/rust-lang/cargo) to build and manage dependencies. It is recommended to use [`rustup`](https://www.rust-lang.org/learn/get-started) to manage the Rust toolchain.

Run `cargo build` to build the project.

## Tools

### `generate.py`

This is a simple python script to generate a templated solution module.

Example usage:

```sh
cd tools
python generate.py -n 1 -t 'Trebuchet?!'
```

This will generate `day1.rs` in the `src` directory, and update `main.rs` and `lib.rs` to include this new module.

Use the `--help` flag to display all available options.

## Testing

All tests can be executed with `cargo test`.
