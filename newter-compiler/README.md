# Newt — Design Canvas Compiler

A **Rust**-based compiler and live canvas for the **Newt** UI language. Architecture B: parse source → layout → render on a **wgpu** canvas (S-tier stack).

## Prerequisites

- **Rust** (1.70+): [rustup](https://rustup.rs)
- **GPU** with Vulkan, Metal, or D3D12 support

## Build & Run

```bash
cd newter-compiler
cargo build --release
cargo run --release
```

Run with a specific `.newt` file:

```bash
cargo run --release -- examples/hello.newt
```

## Newt Language (UI/visual DSL)

- **Variables:** `let name = value;` (numbers, strings, colors `#RRGGBB`)
- **Screens:** `screen Main { ... }` — root UI tree
- **Layout:** `column`, `row`, `stack`, `center` with `gap`, `padding`
- **Elements:** `box`, `text`, `button`, `input`, `spacer`, `image`
- **Props:** `fill`, `stroke`, `radius`, `width`, `height`, `fontSize`, `content`, etc.

Example:

```newt
let padding = 24;

screen Main {
  column { gap: 16, padding: padding } {
    box { fill: #ffffff, radius: 8, padding: 16 } {
      text { content: "Hello", fontSize: 24 }
    }
    row { gap: 12 } {
      box { fill: #e0e0e0, radius: 4 } { text { content: "A" } }
      box { fill: #e0e0e0, radius: 4 } { text { content: "B" } }
    }
  }
}
```

## Project layout

- `src/lexer.rs` — tokenizer
- `src/parser.rs` — recursive descent parser
- `src/ast.rs` — AST types
- `src/value.rs` — interpreter (eval) for expressions
- `src/layout.rs` — flex-like layout → `LayoutNode` + `Rect`
- `src/renderer/` — wgpu canvas (rects; text can be added via glyphon)
- `src/app.rs` — winit event loop, window, redraw
- `src/main.rs` — CLI entry, optional file path

## Next steps

- Text rendering (glyphon)
- Hot-reload (notify) when the `.newt` file changes
- Rounded rects in the shader
- Pan/zoom on the canvas
