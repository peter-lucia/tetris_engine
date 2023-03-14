from tetris_engine import Tetris, Direction


def test_rust_lib():
    """
    Tests the underlying rust library
    """
    t = Tetris()._game
    t.setup_game()
    while t.is_running():
        t.move_down()
        t.increment_frame()

    assert True


def test_singlethreaded():
    """
    Tests a simple running of the game
    """
    tetris = Tetris()
    while tetris.is_game_running():
        tetris.move(direction=Direction.Down.value)
    assert True


def test_multithreaded():
    tetris = Tetris(multithreaded=True)
    old_grid = tetris._game.grid
    test_passes = False
    while tetris.is_game_running():
        tetris.read_game()
        new_grid = tetris._game.grid
        if old_grid != new_grid:
            test_passes = True
            for row in old_grid:
                print(row)
            print()

            for row in new_grid:
                print(row)
            print()
            break
    assert test_passes


if __name__ == '__main__':
    test_multithreaded()