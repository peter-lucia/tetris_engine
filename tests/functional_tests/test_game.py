from tetris_engine import Tetris


def test_alt():
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
    assert True, "Success"


def test_main():
    """
    Tests a simple running of the game
    """
    tetris = Tetris(debug=True)
    while tetris.is_game_running():
        tetris.move()
    assert True, "Success"
