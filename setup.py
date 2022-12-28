from setuptools import setup
from setuptools_rust import Binding, RustExtension

# docs: https://setuptools-rust.readthedocs.io/en/latest/reference.html

setup(
    name="rust_tetris",
    version="0.1.0",
    rust_extensions=[RustExtension(target="rust_tetris",
                                   path="Cargo.toml",
                                   binding=Binding.PyO3)],
    # packages=["hello_rust"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)