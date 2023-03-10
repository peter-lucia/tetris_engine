# the name of the module must match the name of the .so or .pyd file in target/debug or target/release
# https://pyo3.rs/v0.14.5/module.html
# https://pyo3.rs/v0.4.1/
# https://pyo3.rs/main/building_and_distribution.html#manual-builds
import enum
from rust_tetris import create_game

class Direction(enum.Enum):
    Down = enum.auto()
    Left = enum.auto()
    Right = enum.auto()
    RightRotate = enum.auto()
    LeftRotate = enum.auto()

class Tetris:
    """
    A tetris engine powered by rust
    """

    def __init__(self, debug: bool = False):
        """
        Creates a tetris game
        """
        self._game = create_game()
        self.debug = debug

    def new_game(self):
        self._game = create_game()

    def end_game(self):
        self._game.exit()

    def move(self, direction: Direction = Direction.Down):
        """
        Moves the tetromino
        :param direction: The direction or rotation to apply. Only down, left, right, right rotate, and left rotate
        are supported
        """
        if direction == Direction.Left:
            self._game.move_left()
        elif direction == Direction.Right:
            self._game.move_right()
        elif direction == Direction.Down:
            self._game.move_down()
        elif direction == Direction.RightRotate:
            self._game.rotate(reverse=False)
        elif direction == Direction.LeftRotate:
            self._game.rotate(reverse=True)
        else:
            raise ValueError("Invalid direction!")
        self._game.increment_frame()
        if self.debug:
            for row in self._game.grid:
                print(row)
            print()


