# Newter Project Map

**Purpose**: Master navigation and structure of the Newter workspace for humans and AI assistants.

---

## Quick Reference

| Artifact | Path | Purpose |
|----------|------|---------|
| **Compiler** | `newter-compiler/` | Parse Newt → layout tree → wgpu/HTML |
| **Terminal** | `newter-terminal/` | Neo‑brutalism TUI; runs compiler via `cargo run -p newter-compiler` |
| **LSP** | `newter-lsp/` | Language server (diagnostics, completion, hover, goto-def) |
| **IDE** | `ide/vscode-extension/` | VS Code extension (syntax, LSP client) |
| **Docs** | `docs/` | Guides, specs, architecture |

---

## Workspace Layout

```
newter/
├── Cargo.toml                    # resolver = "2"
│   members: newter-compiler, newter-terminal, newter-lsp
├── newter-compiler/              # Compiler + renderer
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs               # CLI: run, check, build, serve, watch
│   │   ├── lib.rs                # Public API (parse, compile, tokenize, symbol_table)
│   │   ├── app.rs                # winit ApplicationHandler, load_and_layout
│   │   ├── ast.rs                # Program, Expr, ElementKind, Prop, Stmt
│   │   ├── lexer.rs              # Lexer, Token, TokenKind
│   │   ├── parser.rs             # Parser, recursive descent
│   │   ├── value.rs              # Value, EvalContext, eval_expr
│   │   ├── layout.rs             # Rect, LayoutNode, layout_tree
│   │   ├── error.rs              # Span, Source, NewtError
│   │   ├── html.rs               # layout_to_html
│   │   ├── serve.rs              # Canvas IDE HTTP + WebSocket + file watcher
│   │   └── renderer/
│   │       ├── mod.rs
│   │       ├── state.rs          # wgpu RendererState, DrawRect
│   │       ├── rect.wgsl         # Vertex/fragment shader
│   │       └── canvas/index.html # IDE client
│   └── examples/
│       ├── hello.newt
│       ├── screen-header-container.newt
│       └── ...
├── newter-terminal/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs               # Event loop, run command
│       └── ui.rs                 # Neo‑brutalism draw (ratatui)
├── newter-lsp/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs               # stdin/stdout LSP loop
│       └── server.rs             # Backend: diagnostics, completion, hover, definition
├── ide/
│   ├── README.md
│   └── vscode-extension/
│       ├── package.json
│       ├── src/extension.ts
│       ├── syntaxes/newt.tmLanguage.json
│       └── language-configuration.json
└── docs/
    ├── PROJECT_MAP.md            # This file
    ├── LANGUAGE_SPEC.md
    ├── COMPILER_GUIDE.md
    └── CLAUDE_CONTEXT.md
```

---

## Build and Run

```bash
# Build workspace
cargo build

# Run compiler (wgpu window)
cargo run -p newter-compiler -- examples/hello.newt

# Export to HTML
cargo run -p newter-compiler -- build examples/hello.newt --html -o out.html

# Serve (live-reload canvas IDE)
cargo run -p newter-compiler -- serve examples/hello.newt

# Terminal TUI
cargo run -p newter-terminal

# LSP (for VS Code / editors)
cargo run -p newter-lsp
```

---

## Pipeline Summary

```
.newt source
    → lexer (lexer.rs)
    → tokens
    → parser (parser.rs)
    → AST (Program)
    → resolve_imports (when path given)
    → EvalContext (value.rs)
    → get_screen
    → layout_tree (layout.rs)
    → LayoutNode
    → renderer (wgpu) / layout_to_html (html) / JSON (serve)
```

---

## Key Constants

| Constant | Value | Location |
|----------|-------|----------|
| `DEFAULT_VIEWPORT_W` | 960 | lib.rs |
| `DEFAULT_VIEWPORT_H` | 640 | lib.rs |
| `DEFAULT_SERVE_PORT` | 3333 | lib.rs |

---

## Cross-Crate Dependencies

- **newter-lsp** → **newter-compiler** (compile, parse, symbol_table, completion helpers)
- **newter-terminal** → runs `cargo run -p newter-compiler` (no library dependency)
