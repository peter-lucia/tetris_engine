# Tetris
* A tetris game engine built with rust and made available as a python library
* Can be used with your python GUI library of choice

### Getting started
```bash
pip install git+ssh://git@github.com/peter-lucia/rust_tetris@python-lib
```

```python
def show_board(t):
    for row in t.grid:
        print(row)
    print()

    
def main():
    t = create_game()
    show_board(t)
    t.setup_game()
    show_board(t)
    t.move_down()
    t.increment_frame()
    show_board(t)


if __name__ == '__main__':
    main()
```

