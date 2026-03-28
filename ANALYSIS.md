# Newter Codebase — In-Depth Analysis

## 1. Project Overview

**Newter** is a Rust workspace containing:

| Crate | Purpose |
|-------|--------|
| **newter-compiler** | Design canvas compiler for the **Newt** UI DSL: parse → layout → render (wgpu) or export HTML |
| **newter-terminal** | Neo-brutalism TUI (crossterm + ratatui) that can spawn the compiler |

- **Language**: Newt — UI/visual DSL with variables, screens, layout primitives, and elements.
- **Outputs**: Live wgpu window, or static HTML via `--html` / `-o`.
- **Stack**: Rust 2021, wgpu 22, winit 0.30, glyphon (text, unused), notify (unused).

---

## 2. Architecture

### 2.1 Pipeline (newter-compiler)

```
.newt source → Lexer → Parser → AST (Program)
                    → EvalContext (variables, components)
                    → layout_tree(ctx, screen.body, viewport Rect) → LayoutNode tree
                    → RendererState.draw_layout() → GPU rects
                    → OR layout_to_html() → standalone HTML
```

- **main.rs**: CLI (args, optional `--html`/`-o`), default inline source or file; creates `App` and runs winit event loop.
- **app.rs**: `ApplicationHandler` — window creation, renderer init, `load_and_layout` / `rebuild_layout`, resize, redraw. No file watching despite `reload_rx` and `notify` dependency.
- **lib.rs**: Public API: `parse`, `parse_file`, `layout_tree`, `layout_to_html`, `EvalContext`, re-exports.

### 2.2 newter-compiler Modules

| Module | Role |
|--------|------|
| **lexer** | Tokenizer: literals (number, string, hex color), keywords (let, screen, component, if/else, for/in), element tokens (box, text, row, column, …), prop-like names (width, fill, …) emitted as **Ident** so e.g. `let padding = 24` works. Line/column spans. |
| **parser** | Recursive descent: program (variable/component/screen), expressions (binary, unary, call, element, block, if, for), element props + children. Prop names: Ident or keyword (Width, Fill, …). |
| **ast** | `Program`, `ProgramItem` (Variable, Component, Screen), `Expr` (Literal, Ident, Binary, Unary, Call, Element, Block, If, For), `ElementKind`, `Prop`/`PropName`/`PropValue`, `Stmt`. |
| **value** | `Value` (Number, String, Bool, Color), `EvalContext` (variables + components from program), `eval_expr` for layout-time expression evaluation. Block scope: `Stmt::Let` in blocks does not mutate context (no block-scoped variables). |
| **layout** | `Rect`, `LayoutNode` (kind, rect, fill, stroke, radius, text, font_size, children), `layout_tree`: from AST element tree to layout tree. Row/column split space by child count + gap; stack/center/box give same rect to children. Text/spacer/image: no children. |
| **renderer** | `RendererState`: wgpu surface, device, queue, rect pipeline (rect.wgsl), vertex buffer. `draw_layout` → `collect_rects` → NDC vertices; `render` clears and draws. **No rounded rects in shader** (radius passed but not used in WGSL). **No glyphon** usage — text only as rect background or in HTML. |
| **html** | `layout_to_html`: single root div, emit_node recursive; position/size from `LayoutNode.rect`, styles for fill, stroke, radius, font_size. |
| **error** | `Span`, `Source`, `NewtError` (Lexer/Parse/Semantic/Io/Other), `format_error` for pretty printing. |

### 2.3 newter-terminal

- **main.rs**: Ratatui loop, 50ms poll, key handling (Enter = submit, q/Ctrl+C = exit). App holds `input`, `output` lines, `exit`. `run <file>` spawns `cargo run -p newter-compiler --release -- <file>` (assumes workspace root).
- **ui.rs**: Neo-brutalism theme (cream #FEFBE7, black borders, yellow accent). Three chunks: title bar, scrollable output, input line with cursor.

---

## 3. Data Flow Details

### 3.1 Parsing and evaluation

- Variables and components are registered in `EvalContext` from the program; variables are evaluated in order (no forward refs).
- Layout uses `get_prop_number`, `get_prop_color`, `get_prop_string` over element props; expressions are evaluated via `eval_expr(ctx, e)`.
- Component calls: new context with params bound, then `layout_tree(&new_ctx, &comp.body, rect)`.

### 3.2 Layout semantics

- **Row**: horizontal strip; equal width per child after gap.
- **Column**: vertical strip; equal height per child after gap.
- **Stack / Center / Box / Button / Input**: all children get the same inner rect (no true stacking or centering of sizes).
- **Text / Spacer / Image**: no child layout; text content and font size stored for HTML; spacer has no visual in wgpu (only in HTML as empty div).

### 3.3 Rendering (wgpu)

- `collect_rects` walks layout tree, pushes `DrawRect` for Box/Row/Column/Stack/Center/Button/Input; for Text only if it has fill (background rect only).
- Vertex format: position (NDC), color (fill). Stroke and radius are in `DrawRect` but **not used in rect.wgsl** (no stroke, no rounded corners).

---

## 4. Dependencies and Unused Code

- **glyphon**: In Cargo.toml, never imported or used. README lists “Text rendering (glyphon)” as next step.
- **notify**: In Cargo.toml; `App` has `reload_rx: Option<mpsc::Receiver<()>>` but no watcher is ever started, so hot-reload is not implemented.
- **glam**: In Cargo.toml but not used in current source (layout uses plain f32 and tuples).

---

## 5. Gaps and Inconsistencies

1. **Rounded corners**: `LayoutNode.radius` and `DrawRect.r` are set but the WGSL shader draws only axis-aligned quads; radius has no effect in GPU output.
2. **Stroke**: Stroke color is collected and passed to `DrawRect` but the shader has no stroke path; stroke has no effect in wgpu.
3. **Text on GPU**: Text is not rendered with glyphon; only colored rects (and HTML export shows text in divs).
4. **Block scope**: Block `Stmt::Let` is parsed and evaluated but the result is not inserted into context, so block-local variables do not work.
5. **For loops**: AST and parser support `for var in iter { body }` but `layout_tree` and `eval_expr` do not handle `Expr::For` (layout falls through to default empty box).
6. **Parser prop keywords**: Lexer maps "width", "fill", etc. to `Ident`; parser `is_prop_keyword()` checks `TokenKind::Width` etc., which never appear. Prop parsing still works via `Ident` and `PropName::Ident`; layout matches by string name. So behavior is correct but the keyword path in the parser is dead.
7. **Single screen**: Only the first screen in the program is used for layout/canvas; no way to select or switch screens.
8. **Resize**: On resize, layout is recomputed with new viewport; redraw is requested in `about_to_wait` every frame (no dirty flag).

---

## 6. File Inventory (source only)

```
newter/
├── Cargo.toml                 # workspace: newter-compiler, newter-terminal
├── newter-compiler/
│   ├── Cargo.toml
│   ├── README.md
│   ├── examples/
│   │   └── hello.newt
│   └── src/
│       ├── main.rs            # CLI, --html, App + event loop
│       ├── lib.rs             # parse, parse_file, re-exports
│       ├── app.rs             # winit ApplicationHandler, load_and_layout
│       ├── ast.rs             # Program, Expr, ElementKind, Prop, Stmt
│       ├── lexer.rs           # Lexer, Token, TokenKind
│       ├── parser.rs          # Parser, recursive descent
│       ├── value.rs           # Value, EvalContext, eval_expr
│       ├── layout.rs          # Rect, LayoutNode, layout_tree
│       ├── error.rs           # Span, Source, NewtError, format_error
│       ├── html.rs            # layout_to_html
│       └── renderer/
│           ├── mod.rs
│           ├── state.rs       # RendererState, DrawRect, rect pipeline
│           └── rect.wgsl      # vs_main, fs_main (no radius/stroke)
└── newter-terminal/
    ├── Cargo.toml
    ├── README.md
    └── src/
        ├── main.rs            # event loop, run command
        └── ui.rs              # Neo-brutalism draw
```

---

## 7. Test Coverage

- **lib.rs**: One test `parse_default_program` — parses a small program and checks `!program.items.is_empty()`.
- No tests for lexer, layout, value (eval), renderer, or HTML export.

---

## 8. Summary

The codebase is a clear pipeline from a custom UI DSL to either a wgpu canvas or HTML. The compiler core (lexer, parser, AST, eval, layout) is implemented and wired; the main gaps are GPU text (glyphon unused), rounded rects and stroke in the shader, hot-reload (notify unused), and full support for blocks (block scoping) and for-loops in layout/eval. The terminal is a minimal TUI that can launch the compiler; it does not share the Newt language or compiler logic.
