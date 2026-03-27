# Contributing to Newt

Thanks for your interest in contributing!

## Development setup

```bash
git clone https://github.com/voxmastery/newter.git
cd newter
cargo build --release
cargo test --workspace
```

### Prerequisites

- Rust 1.75+ ([rustup.rs](https://rustup.rs))
- A C linker (gcc, clang, or MSVC)

## Project structure

```
newter/
  newter-compiler/       # The compiler (Rust)
    src/
      lexer.rs           # Tokenizer
      parser.rs          # Recursive descent parser
      ast.rs             # AST types (ElementKind, PropName)
      layout.rs          # Flex/grid layout engine
      html.rs            # HTML export backend
      serve.rs           # Canvas IDE (Axum + WebSocket)
      renderer/          # GPU renderer (wgpu)
  newter-lsp/            # Language Server Protocol
  ide/vscode-extension/  # VS Code extension
  docs/website/          # Documentation site (Docusaurus)
  examples/              # Example .newt programs
```

## Adding a new element

See `docs/CLAUDE_CONTEXT.md` section 14 for the full checklist.

## Code style

```bash
cargo fmt
cargo clippy
```

## Submitting changes

1. Fork the repo and create a branch from `main`
2. Make your changes
3. Run `cargo test --workspace` and `cargo clippy`
4. Submit a pull request

## Reporting bugs

Open a [GitHub issue](https://github.com/voxmastery/newter/issues/new) with:
- What you expected
- What actually happened
- The `.newt` code that triggered the issue
- Your OS and compiler version (`newter-compiler --help`)
