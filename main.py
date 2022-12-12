# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
import asyncio
from time import sleep

from rust_tetris import MyTetris, start

async def main():
    tetris = MyTetris()
    start(tetris)
    while True:
        for row in tetris.grid:
            print(row)
        print()
        sleep(1)

asyncio.run(main())
