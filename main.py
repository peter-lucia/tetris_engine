# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
# https://pyo3.rs/v0.4.1/
# https://pyo3.rs/main/building_and_distribution.html#manual-builds
from time import sleep
from rust_tetris import create_game


def show_board(t):
    for row in t.grid:
        print(row)
    print()
    sleep(0.5)


def main():
    t = create_game()
    show_board(t)
    t.setup_game()
    show_board(t)
    t.move_down()
    t.increment_frame()
    show_board(t)
    t.rotate_left()
    t.increment_frame()
    show_board(t)
    t.rotate_right()
    t.increment_frame()
    show_board(t)
    t.move_down()
    t.increment_frame()
    show_board(t)
    while True:
        show_board(t)
        t.increment_frame()


if __name__ == '__main__':
    main()