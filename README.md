# Tetris

[![Rust](https://github.com/peter-lucia/tetris_engine/actions/workflows/rust.yml/badge.svg)](https://github.com/peter-lucia/tetris_engine/actions/workflows/rust.yml)

A tetris engine python package powered by rust

* Handles the tetris game logic for you so you can focus on building a tetris interface
* Provides flexibility for the game loop, allowing you to define the game loop entirely
or run a built-in game loop thread in the background
* The built-in, multithreaded game loop capability makes use of rust's safe concurrency offering




![Tetris](images/game5.gif)

### Install the package

```bash
pip install http://www.github.com/peter-lucia/tetris_engine/archive/main.zip
```

```python
from time import sleep

from tetris_engine import Tetris, Direction

def run_singlethreaded():
    """
    You control the game loop as well as the game speed
    """
    tetris = Tetris(debug=True)
    while tetris.is_game_running():
        tetris.move(direction=Direction.Down.value)
        sleep(1)
        

def run_multithreaded():
    """
    You control the user controls of the game but the 
    game loop runs in a background thread 
    """
    tetris = Tetris(multithreaded=True, debug=True)
    while tetris.is_game_running():
        tetris.display()
        sleep(.5)


```
