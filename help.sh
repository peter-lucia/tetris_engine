#!/bin/bash


# https://pyo3.rs/v0.4.1/
rustup target list
rustup target add <specific target from list>

cargo --version
rustup update
cargo --version

cargo build --lib --target x86_64-apple-darwin
