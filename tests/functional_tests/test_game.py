from tetris_engine import Tetris
from rust_tetris import create_game

def test_alt():
    t = create_game()
    t.setup_game()
    while True:
        t.move_down()
        t.increment_frame()
        for row in t.grid:
            print(row)
        print()
    assert True, "Success"

def test_main():
    tetris = Tetris()
    while True:
        tetris.move()
    assert True, "Success"
