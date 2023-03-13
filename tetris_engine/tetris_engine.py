import enum

from rust_tetris import create_game, read_game_multithreaded, write_game_multithreaded, start_game_multithreaded


class Direction(enum.Enum):
    Down = enum.auto()
    Left = enum.auto()
    Right = enum.auto()
    RightRotate = enum.auto()
    LeftRotate = enum.auto()


class Tetris:
    """
    A tetris engine powered by rust
    * Supports single threaded operation where the caller controls the game speed
    * Supports multithreading where the engine runs in the background, updating
      the board automatically. The client just supplies the user's commands
    """

    def __init__(self, multithreaded: bool = False, debug: bool = False):
        """
        Creates a tetris game
        """
        self.multithreaded = multithreaded
        if self.multithreaded:
            print("Creating a multithreaded game")
            self._game = read_game_multithreaded()
            start_game_multithreaded()
        else:
            print("Creating a single threaded game")
            self._game = create_game()
            self._game.setup_game()
        self.debug = debug
        if self.debug:
            self.display()

    def restart(self):
        """
        Creates a new game, overwriting the existing game
        """
        if self.multithreaded:
            raise ValueError("Not yet supported!")
        else:
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
        if self.multithreaded:
            self._game = read_game_multithreaded()
            return self._game.is_running()
        return self._game.is_running()

    def move(self, direction: int = Direction.Down.value):
        """
        Moves the tetromino
        :param direction: The direction or rotation to apply. Down, left, right, right rotate, and left rotate
        are supported
        """

        if direction == Direction.Left.value:
            self._game.move_left()
        elif direction == Direction.Right.value:
            self._game.move_right()
        elif direction == Direction.Down.value:
            self._game.move_down()
        elif direction == Direction.RightRotate.value:
            self._game.rotate(False)
        elif direction == Direction.LeftRotate.value:
            self._game.rotate(True)
        else:
            raise ValueError("Invalid direction!")

        if self.multithreaded:
            write_game_multithreaded(self._game)

        # does not move the tetromino
        self._game.increment_frame()

        if self.debug:
            self.display()

    def display(self):
        """
        Display the current state of the game
        """
        if self.multithreaded:
            self._game = read_game_multithreaded()
        for row in self._game.grid:
            print(row)
        print()


