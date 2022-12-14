# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
from time import sleep
from rust_tetris import create_game, setup_game, run_frame, move_down


def main():

    t = create_game()
    t = setup_game(t)
    while True:
        t = move_down(t)
        t = run_frame(t)
        for row in t.grid:
            print(row)
        print()
        sleep(.5)

if __name__ == '__main__':
    main()