#!/bin/bash

# OLD:
# Source: https://pyo3.rs/latest/building_and_distribution.html#manual-builds
#ln -s target/debug/librust_tetris.so rust_tetris.so

# NEW:
# Just build
python setup.py build

# Build and install all at once:
python setup.py install