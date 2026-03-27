# newter-terminal

**Frozen ice-glass** native terminal for the Newter project. Built with Rust, **crossterm** and **ratatui**.

## Run

From the **workspace root** (`newter/`):

```bash
cargo run -p newter-terminal
```

Or from this directory:

```bash
cargo run
```

## Commands

- **run &lt;file&gt;** — Spawns the Newt compiler with the given file (e.g. `run examples/hello.newt`). Run from workspace root so `cargo run -p newter-compiler` works.
- **.quit** / **quit** / **exit** — Exit the terminal.
- **q** — Quit (single key).

## Frozen ice-glass style

- Deep blue background
- Frosted blue panels
- Rounded glass-like borders
- Cold cyan accent text
- Context-aware log colors for commands, run events, and errors
