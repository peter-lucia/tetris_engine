#!/bin/bash

# Source: https://pyo3.rs/latest/building_and_distribution.html#manual-builds

#ln -s target/debug/librust_tetris.so rust_tetris.so
#cargo install wasm-pack
#python setup.py build

# install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# install npm
echo "run ~/projects/tools/scripts/install_npm.sh"

echo "run wasm-pack build"
