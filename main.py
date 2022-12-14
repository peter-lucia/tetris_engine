# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
import threading
from time import sleep

from rust_tetris import MyTetris


def start(tetris: MyTetris):
    tetris.start_game()

def status(tetris: MyTetris):
    while True:
        for row in tetris.grid:
            print(row)
        print("")
        sleep(1)

def main():
    tetris = MyTetris()
    t1 = threading.Thread(target=start, args=(tetris,))
    t2 = threading.Thread(target=status, args=(tetris,))

    t1.start()
    t2.start()

    # wait until thread 1 is completely executed
    t1.join()
    # wait until thread 2 is completely executed
    t2.join()


if __name__ == '__main__':
    # asyncio.run(main())
    main()