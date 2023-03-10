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
        if self.debug:
            self.display()

    def restart(self):
        """
        Creates a new game, overwriting the existing game
        """
        self._game = create_game()

    def end_game(self):
        """
        Ends the current game
        """
        self._game.exit()

    def is_game_running(self):
        """
        A check to determine if the current game is still active
        :return: True if the current game is active, False otherwise
        """
        return self._game.is_running()

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

        # does not move the tetromino
        self._game.increment_frame()
        if self.debug:
            self.display()

    def display(self):
        """
        Display the current state of the game
        """
        for row in self._game.grid:
            print(row)
        print()


