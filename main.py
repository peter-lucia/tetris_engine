# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
from rust_tetris import get_well

well = get_well()
print(dir(well))