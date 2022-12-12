# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
from time import sleep

from rust_tetris import MyTetris

tetris = MyTetris()
tetris.start_game()
print("got here")
while True:
    # tetris.rotate_tetromino()
    for row in tetris.grid:
        print(row)
    print()
    tetris.move_piece("Down")
    sleep(1)