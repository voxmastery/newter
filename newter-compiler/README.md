# newter-compiler

[![Crates.io](https://img.shields.io/crates/v/newter-compiler.svg)](https://crates.io/crates/newter-compiler)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/voxmastery/newter/blob/main/LICENSE)

**The compiler for [Newt](https://github.com/voxmastery/newter) — a UI language that compiles to canvas, HTML, and JSON.**

Describe a UI in a `.newt` file. The compiler outputs a GPU-rendered canvas, a standalone HTML page, or a JSON layout tree — from the same source.

## Install

```bash
cargo install newter-compiler
```

## Hello World

Create `hello.newt`:

```newt
screen Main {
    center(fill: #f9fafb)(
        column(gap: 16)(
            text("Hello, Newt!", fontSize: 32, fontWeight: "700")
            text("Your first UI in 3 lines")
        )
    )
}
```

Run it:

```bash
newter-compiler serve hello.newt
# Open http://localhost:3333
```

## What you get

```bash
newter-compiler run app.newt                          # GPU canvas window
newter-compiler serve app.newt                        # Browser IDE + hot reload
newter-compiler build app.newt --html -o out.html     # Standalone HTML file
newter-compiler check app.newt                        # Validate syntax
```

## The language

73 built-in elements. Reactive state. Components. Themes. Imports. A syntax you can learn in 5 minutes.

```newt
state count = 0

screen Counter {
    column(gap: 24, padding: 32)(
        text("Count: {count}", fontSize: 32)
        row(gap: 12)(
            button("+1", fill: #2563eb, radius: 8, onClick: { count = count + 1 })
            button("Reset", fill: #ef4444, radius: 8, onClick: { count = 0 })
        )
    )
}
```

**Same UI, less code:**

| | Newt | React |
|---|---|---|
| A button with a counter | 8 lines | 20+ lines |
| No build step | Yes | No |
| No runtime | Yes (HTML export) | No |

## Why

Describing a UI shouldn't require picking a framework first. Newt separates *what you want to see* from *how it gets rendered*. One source, multiple targets.

## Docs

- [Getting Started](https://newter.vercel.app/docs/getting-started)
- [Language Reference](https://newter.vercel.app/docs/language)
- [73 Built-in Elements](https://newter.vercel.app/docs/language/elements)
- [CLI Reference](https://newter.vercel.app/docs/compiler/cli)
- [Examples](https://newter.vercel.app/docs/examples)

## Status

**Alpha (v0.1)** — functional but evolving. Expect breaking changes.

## License

MIT
