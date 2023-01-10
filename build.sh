#!/usr/bin/zsh

# default build, (builds for wasm)
cargo build

# explicit build for wasm
cargo build --features "wasm"

# build for python-lib
cargo build --features "python-lib"
