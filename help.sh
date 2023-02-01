#!/bin/bash


# Cross compilation
# https://pyo3.rs/v0.4.1/
rustup target list
# rustup target add <specific target from list>
#rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-unknown-linux-gnu

cargo --version
rustup update
cargo --version

#cargo build --lib --target x86_64-apple-darwin
#cargo build --bin rust_tetris --target x86_64-unknown-linux-musl
cargo build --bin rust_tetris --release --target x86_64-unknown-linux-gnu

cargo install cross --git https://github.com/cross-rs/cross
# Ensure docker desktop is running
cross build --target x86_64-unknown-linux-gnu --release --bin rust_tetris