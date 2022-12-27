# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
# https://pyo3.rs/v0.4.1/
# https://pyo3.rs/main/building_and_distribution.html#manual-builds
from time import sleep
from rust_tetris import create_game


def main():
    t = create_game()
    t.setup_game()
    while True:
        t.move_down()
        t.increment_frame()
        for row in t.grid:
            print(row)
        print()
        sleep(.5)

if __name__ == '__main__':
    main()