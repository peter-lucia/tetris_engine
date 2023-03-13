from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="tetris-engine",
    version="0.1.0",
    rust_extensions=[RustExtension("rust_tetris", binding=Binding.PyO3)],
    packages=["tetris_engine"],
    zip_safe=False,  # rust extensions are not zip safe, just like C-extensions.
)