# Newter Project — Full Context for Claude

**Purpose:** This document provides a complete, code-level understanding of the Newter project so Claude can assist with development, debugging, refactoring, and feature work without needing to re-explore the codebase.

---

## 1. Project Overview

**Newter** is a Rust workspace for the **Newt** UI DSL — a declarative, call-style language for building UIs. Outputs: native wgpu canvas, static HTML, or JSON for a canvas IDE.

### Workspace Members

| Crate | Path | Purpose |
|-------|------|---------|
| **newter-compiler** | `newter-compiler/` | Parser, layout engine, wgpu renderer, HTML export, serve |
| **newter-terminal** | `newter-terminal/` | Neo-brutalism TUI (ratatui) that runs compiler commands |
| **newter-lsp** | `newter-lsp/` | LSP server for diagnostics, completion, hover, go-to-definition |

### Key Files (entry points)

| File | Role |
|------|------|
| `newter-compiler/src/main.rs` | CLI: `run`, `check`, `build`, `serve`, `watch` |
| `newter-compiler/src/lib.rs` | Library API, `compile()` entry point |
| `newter-lsp/src/main.rs` | LSP stdin/stdout loop |
| `newter-terminal/src/main.rs` | TUI event loop |

---

## 2. Compiler Architecture (Code-Level)

### 2.1 Single Compile Entry

**Always use** `compile(source, path?, screen_name?)` — never call `parse`, `resolve_imports`, `layout_tree` separately for production behavior.

```rust
// newter-compiler/src/lib.rs
pub fn compile(
    source: &str,
    path: Option<&std::path::Path>,
    screen_name: Option<&str>,
) -> Result<(Program, layout::LayoutNode), NewtError>
```

1. `parse(source, path?)` → `Program`
2. If `path` is `Some`: `resolve_imports(program, base_dir)` (loads `import "x.newt"` files, avoids cycles)
3. `get_screen(&program, screen_name)` → first screen or named screen
4. `layout_tree(&ctx, &screen.body, viewport_rect)` → `LayoutNode`

Viewport: `(0, 0, 960, 640)` (`DEFAULT_VIEWPORT_W`, `DEFAULT_VIEWPORT_H`).

### 2.2 Pipeline Data Flow

```
source (.newt)
  → Lexer::next_token()       [lexer.rs]
  → Parser::parse()           [parser.rs]
  → Program (ast.rs)
  → resolve_imports()         [lib.rs] (if path set)
  → EvalContext::from_program [value.rs]
  → get_screen()
  → layout_tree()             [layout.rs]
  → LayoutNode tree
  → renderer/state.rs draw_layout() OR html.rs layout_to_html()
```

### 2.3 Module-by-Module (newter-compiler)

| Module | File | Key Types | Key Functions |
|--------|------|-----------|---------------|
| **lexer** | `lexer.rs` | `Lexer`, `Token`, `TokenKind`, `TokenCategory` | `Lexer::new()`, `next_token()` |
| **parser** | `parser.rs` | `Parser` | `Parser::new()`, `parse()`, `parse_expr()` |
| **ast** | `ast.rs` | `Program`, `ProgramItem`, `Expr`, `ElementKind`, `Prop`, `Stmt` | — |
| **value** | `value.rs` | `Value`, `EvalContext` | `eval_expr()`, `EvalContext::from_program()` |
| **layout** | `layout.rs` | `Rect`, `LayoutNode`, `LayoutKind` | `layout_tree()`, `layout_kind_from_element()` |
| **error** | `error.rs` | `Span`, `Source`, `NewtError` | `format_error()` |
| **html** | `html.rs` | — | `layout_to_html()` |
| **renderer** | `renderer/state.rs` | `RendererState`, `DrawRect` | `draw_layout()`, `render()` |
| **serve** | `serve.rs` | `AppState` | `serve()`, `compile_to_json()` |
| **app** | `app.rs` | `App` (implements `ApplicationHandler`) | `load_and_layout()`, `rebuild_layout()` |

---

## 3. Lexer (lexer.rs)

### TokenKind (partial)

- **Literals:** `Number(f64)`, `String(String)`, `Ident(String)`, `True`, `False`, `HexColor(u8,u8,u8,u8)`
- **Keywords:** `Let`, `Component`, `Screen`, `If`, `Else`, `For`, `In`, `Theme`, `Use`, `Import`
- **Elements:** `Header`, `Footer`, `Container`, `Box`, `Text`, `Row`, `Column`, `Grid`, `Stack`, `Center`, `Spacer`, `Image`, `Button`, `Input`, `Card`, `Modal`, `Tabs`, …
- **Props (lexed as Ident):** `width`, `height`, `fill`, `padding` etc. — so `let padding = 24` works
- **Delimiters:** `LeftBrace` `{`, `RightBrace` `}`, `LeftParen` `(`, `Comma`, `Colon`, `Semicolon`, `Eq`
- **Operators:** `EqEq`, `NotEq`, `Le`, `Ge`, `And`, `Or`, `Arrow` `->`

### TokenCategory (for highlighting)

`Keyword`, `String`, `Number`, `Ident`, `Operator`, `Punctuation`, `Comment`, `Eof`

---

## 4. Parser (parser.rs)

- **Recursive descent** with precedence: `or` → `and` → `equality` → `comparison` → `term` → `factor` → `unary` → `primary`
- **Primary:** literal, `Ident`, `Call`, `Element`, `Block`, `If`, `For`
- **Elements:** `box { props } { children }` — both blocks optional; shorthand `text("Hi")` = `text { content: "Hi" }`
- **Call:** `Card("title", #ff0000)` — callee, positional args, optional `slot_args`
- **Typo hints:** `screan` → suggests `screen`

---

## 5. AST (ast.rs)

### ProgramItem

`Variable(VariableDecl)` | `Component(ComponentDecl)` | `Screen(ScreenDecl)` | `Theme(ThemeDecl)` | `Import(ImportDecl)` | `UseTheme(String)`

### Expr

`Literal` | `Ident` | `Binary` | `Unary` | `Call` | `Element` | `Block` | `If` | `For`

### ElementKind → LayoutKind (layout.rs: layout_kind_from_element)

| ElementKind | LayoutKind |
|-------------|------------|
| Header, Footer, Container, Sidebar, Section, Widget | Column |
| Accordion, Bento, Tabs, Nav, Form, Feed, Carousel | Column |
| Modal | Modal |
| Card, Box, ConfirmDialog, Toast, etc. | Box |
| Text | Text |
| Row | Row |
| Column | Column |
| Grid | Grid |
| Stack | Stack |
| Center | Center |
| Spacer | Spacer |
| Image | Image |
| Button | Button |
| Input, Password, Search | Input |
| Checkbox, Radio, Dropdown, etc. | Box |

---

## 6. Value & EvalContext (value.rs)

### Value

`Number(f64)` | `String(String)` | `Bool(bool)` | `Color { r, g, b, a }` | `Array(Vec<Value>)`

### EvalContext

- `variables: HashMap<String, Value>` — from `VariableDecl`s
- `components: HashMap<String, ComponentDecl>` — from `ComponentDecl`s
- `theme_vars: Vec<HashMap<String, Value>>` — from `use theme X`

### eval_expr

Supports: literals, ident (lookup), binary, unary, call (builtins: `range(n)`), element (no-op, layout handles), block (evaluates stmts, returns last expr), if/else, for (returns `Value::Array`).

---

## 7. Layout (layout.rs)

### layout_tree(ctx, expr, rect) → LayoutNode

- **Element:** resolve props (padding, gap, fill, stroke, radius, fontSize, content), compute inner rect, dispatch by LayoutKind
- **Row:** split width by children + gap; supports `width` on children
- **Column:** split height by children + gap; supports `height` on children
- **Grid:** `columns` prop (e.g. `"1fr 1fr"`), rows inferred
- **Stack/Center/Box/Button/Input:** all children get same inner rect
- **Text/Spacer/Image:** no children
- **Call:** resolve component, bind params, recurse on body
- **Block:** layout last expr
- **For:** layout body for each iteration; children collected
- **If:** layout then or else branch

### Visibility

`minWidth`, `maxWidth`, `minHeight`, `maxHeight` — compared to viewport; if fails, empty node.

### Props (get_prop_*)

`width`, `height`, `fill`, `stroke`, `strokeWidth`, `radius`, `padding`, `gap`, `fontSize`, `content`, `minWidth`, `maxWidth`, `minHeight`, `maxHeight`, `shadow`, `transition`, `role`, `ariaLabel`, `focusOrder`, `onClick`, `href`, `name`, `aspectRatio`, `columns`, `rows`, `src`

---

## 8. Renderer (renderer/)

### state.rs

- `RendererState::new(window)` — wgpu device/queue, rect pipeline
- `draw_layout(layout)` — `collect_rects` → vertices (6 per rect, NDC)
- `render()` — clear, draw, present
- **Note:** `rect.wgsl` draws solid quads; radius and stroke in `DrawRect` are **not used** in the shader (no rounded corners, no stroke)

### rect.wgsl

Vertex: position (vec2), color (vec4). Fragment: output color. No radius/stroke in shader.

---

## 9. HTML Export (html.rs)

`layout_to_html(root, w, h, theme_css_vars?)` → full HTML string. Emits divs with `position:absolute`, `left`, `top`, `width`, `height`, `background-color`, `border`, `border-radius`, `font-size`. Text escaped for XSS.

---

## 10. Serve (serve.rs)

- **`serve(file_path, port, screen_name?)`** — Axum HTTP server
- **Routes:** `/` (canvas IDE HTML), `/ws` (WebSocket), `/api/layout`, `/api/compile`, `/api/source`
- **File watcher:** notify on change; recompiles and broadcasts JSON via WebSocket
- **Payload:** `{ type: "layout", screens, screen, viewport, root }` or `{ type: "error", message }`

---

## 11. LSP (newter-lsp)

### server.rs

- **Backend** implements `LanguageServer`
- **textDocument/didOpen, didChange:** `compile()` → publishDiagnostics (errors)
- **completion:** keywords, element names, prop names, symbols (variables, components, screens, themes)
- **hover:** symbol description from symbol_table
- **definition:** span from symbol_table

### main.rs

`LspService::new(server::Backend::new)` → stdin/stdout loop

---

## 12. Terminal (newter-terminal)

- **main.rs:** ratatui loop; `run <file>` spawns `cargo run -p newter-compiler --release -- <file>`
- **ui.rs:** FrozenGlass theme; title bar, scrollable output, input line

---

## 13. IDE / VS Code

- **ide/vscode-extension/:** Language ID `newt`, LSP client, commands `newt.run`, `newt.serve`, `newt.check`, `newt.exportHtml`
- **Syntax:** `syntaxes/newt.tmLanguage.json`
- **Config:** `newt.compilerPath`, `newt.lsp.path`, `newt.lsp.enabled`

---

## 14. Adding a New Element (Checklist)

1. **lexer.rs:** Add `TokenKind` variant and match in `read_ident_or_keyword`
2. **ast.rs:** Add `ElementKind` variant and `from_token_kind` mapping
3. **layout.rs:** Add `layout_kind_from_element` mapping (`LayoutKind` = Box, Row, Column, etc.)
4. **lib.rs:** Add to `completion_element_names()` if desired

---

## 15. Known Gaps / Inconsistencies

- **Rounded corners:** `LayoutNode.radius` set but shader ignores it
- **Stroke:** stroke in `DrawRect` not rendered
- **Text on GPU:** glyphon in Cargo.toml but unused; text only in HTML
- **Block scope:** `Stmt::Let` in blocks not persisted to context
- **Single screen:** only first/named screen used; no runtime switch
- **notify in app:** `reload_rx` exists but no watcher started in wgpu app (serve uses notify)

---

## 16. Constants

| Name | Value |
|------|-------|
| `DEFAULT_VIEWPORT_W` | 960 |
| `DEFAULT_VIEWPORT_H` | 640 |
| `DEFAULT_SERVE_PORT` | 3333 |

---

## 17. Test Coverage (lib.rs tests)

- Lexer: number, string, hex, operators, keywords vs idents, errors
- Parser: variable, component, theme/use, if, for, nested elements
- Eval: binary ops, comparison, block, for, theme vars, if/else
- Layout: row/column split, padding, fixed width, for loop, visibility
- HTML: text, XSS escape, theme vars
- Compile: full pipeline, missing screen, named screen, component call
- Error: span info, format

---

## 18. Documentation Cross-References

- **PROJECT_MAP.md** — File tree, module map, quick reference
- **LANGUAGE_SPEC.md** — Newt syntax, elements, props
- **COMPILER_GUIDE.md** — Pipeline stages, Mermaid diagrams
- **ANALYSIS.md** — Gaps, inconsistencies, unused deps
