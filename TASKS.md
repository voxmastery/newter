# Newter — Task List

Prioritized, actionable tasks derived from the full codebase analysis. See **ANALYSIS.md** for architecture and details.

---

## P0 — Critical / Completeness

| ID | Task | Location | Notes |
|----|------|----------|------|
| P0-1 | **Implement rounded rects in GPU shader** | `newter-compiler/src/renderer/rect.wgsl`, `state.rs` | `DrawRect.r` and `LayoutNode.radius` are set but WGSL draws only quads. Add fragment shader logic (e.g. SDF or 4-quarter circles) to clip by radius. |
| P0-2 | **Wire text rendering with glyphon** | `newter-compiler/src/renderer/state.rs`, `app.rs` | Glyphon is in Cargo.toml but unused. For each `LayoutNode` with `text` + `font_size`, render text at `rect` with glyphon; integrate in same render pass or a second pass. |
| P0-3 | **Implement file watcher hot-reload** | `newter-compiler/src/app.rs` | `notify` is dependency; `reload_rx` exists but no watcher. When `source_path` is set, start notify watcher; on event, re-read file, `load_and_layout()`, request redraw. |

---

## P1 — Correctness & Behavior

| ID | Task | Location | Notes |
|----|------|----------|-------|
| P1-1 | **Support block-scoped variables** | `newter-compiler/src/value.rs` | In `Expr::Block`, for `Stmt::Let` mutate a local scope (e.g. clone context, insert, use for subsequent stmts and return value). |
| P1-2 | **Handle `Expr::For` in layout and eval** | `newter-compiler/src/layout.rs`, `value.rs` | Currently `layout_tree` and `eval_expr` ignore `For`. Define semantics (e.g. iter as list of values); in layout, produce multiple child nodes; in eval, return last iteration or list. |
| P1-3 | **Use stroke in wgpu renderer** | `newter-compiler/src/renderer/` | Stroke is in `DrawRect` but shader ignores it. Option A: second pass or overlay for stroke quads; Option B: fragment shader that draws inner fill and outer stroke by distance. |
| P1-4 | **True Stack/Center layout** | `newter-compiler/src/layout.rs` | Stack: children overlay same rect (already same rect). Center: size child by content (or min size), center within inner rect. May require two-pass layout (measure then place). |

---

## P2 — Quality & Maintainability

| ID | Task | Location | Notes |
|----|------|----------|-------|
| P2-1 | **Add unit tests for lexer** | `newter-compiler/src/lexer.rs` | Tests for tokens: numbers, strings, hex colors, keywords, identifiers, comments, error cases (unterminated string, invalid hex). |
| P2-2 | **Add unit tests for layout** | `newter-compiler/src/layout.rs` | Tests: row/column gap and padding, single child, multiple children; optional integration test with small AST. |
| P2-3 | **Add unit tests for value (eval)** | `newter-compiler/src/value.rs` | Tests: literals, ident, binary ops, unary, if/else, component call with args; block with expr; undefined variable/component errors. |
| P2-4 | **Remove or use dead parser prop keywords** | `newter-compiler/src/parser.rs`, `lexer.rs` | Either: (a) make lexer return `TokenKind::Width` etc. for prop names and keep parser keyword path, or (b) remove `is_prop_keyword` and keyword arms in `parse_prop` and rely only on `Ident`. |
| P2-5 | **Optional: remove unused `glam`** | `newter-compiler/Cargo.toml` | If no plan to use it, drop dependency; or use for Rect/vec2 in layout and renderer for clarity. |

---

## P3 — Features & UX

| ID | Task | Location | Notes |
|----|------|----------|-------|
| P3-1 | **Multiple screens / screen selector** | `newter-compiler` | Allow selecting which screen to show (e.g. CLI flag `--screen Name`, or first by default). Layout and render use that screen. |
| P3-2 | **Pan/zoom on canvas** | `newter-compiler/src/app.rs`, `renderer` | Track viewport transform (pan + scale); apply in layout-to-NDC or in vertex shader; handle mouse drag and scroll. |
| P3-3 | **Terminal: run from any cwd** | `newter-terminal` | `run <file>` uses `cargo run -p newter-compiler` which assumes workspace root. Resolve path to compiler binary or document that user must run from workspace root. |
| P3-4 | **Improve redraw strategy** | `newter-compiler/src/app.rs` | Only request redraw when layout/source changes or on resize; avoid requesting every frame in `about_to_wait` if nothing changed (saves CPU/GPU). |

---

## P4 — Documentation & Polish

| ID | Task | Location | Notes |
|----|------|----------|-------|
| P4-1 | **Language spec (short)** | `newter-compiler/` or repo root | Document grammar, evaluation order, layout rules (row/column/stack/center), and list of supported props per element. |
| P4-2 | **README: list known limitations** | `newter-compiler/README.md` | State: no GPU text yet, no rounded corners/stroke on GPU, no hot-reload, single screen, block scope and for not fully supported. |
| P4-3 | **More examples** | `newter-compiler/examples/` | Add .newt examples: components with params, conditional layout, nested rows/columns. |

---

## Quick reference

- **Analysis**: `ANALYSIS.md`
- **Build & run compiler**: `cargo run -p newter-compiler --release [-- examples/hello.newt]`
- **HTML export**: `cargo run -p newter-compiler --release -- --html out.html examples/hello.newt`
- **Terminal**: `cargo run -p newter-terminal` (from workspace root)
