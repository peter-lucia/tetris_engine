# Tetris
* A tetris game engine built with rust

Capabilities

* Use with your Python or Rust GUI library of choice

### Python Library - Getting started
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

### Rust Crate (Binary)

Simulate a game

```bash
cargo run --bin rust_tetris
```

### Rust Crate (Library)

Build the library only

```bash
cargo build --lib rust_tetris
```

### TODO
- [ ] Game engine available for rust
- [ ] Crate available for rust
- [ ] Python package available from pypi
- [x] Build pipeline - cross-compile for mac, windows, linux
- [x] Python package available from git
