from time import sleep

from tetris_engine import Tetris, Direction


def run_lib():
    """
    Tests the underlying rust library
    """
    t = Tetris()._game
    t.setup_game()
    while t.is_running():
        t.move_down()
        t.increment_frame()
        for row in t.grid:
            print(row)
        print()


def run_main():
    """
    Tests a simple running of the game
    """
    tetris = Tetris(debug=True)
    while tetris.is_game_running():
        tetris.move(direction=Direction.Down.value)
        sleep(1)


def run_multithreaded():
    tetris = Tetris(multithreaded=True, debug=True)
    while tetris.is_game_running():
        tetris.display()
        sleep(.5)


if __name__ == '__main__':
    # run_lib()
    run_main()
    # run_multithreaded()
