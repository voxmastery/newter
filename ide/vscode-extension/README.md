# Newt UI Language

Full language support for **Newt** — a declarative UI DSL for building layouts and screens.

## Features

- **Syntax highlighting** — rich TextMate grammar for `.newt` files
- **LSP diagnostics** — real-time error checking powered by `newter-lsp`
- **Run & Check** — compile or validate files directly from VS Code
- **Live preview** — launch a dev server with hot reload via Canvas IDE
- **HTML export** — export any `.newt` file to standalone HTML

## Requirements

- The `newter-compiler` binary must be on your `PATH` (or configure `newt.compilerPath`)
- For LSP diagnostics, install `newter-lsp` (or configure `newt.lsp.path`)

## Commands

| Command | Description | Keybinding |
|---------|-------------|------------|
| **Newt: Run current file** | Compile and run the active `.newt` file | `Ctrl+Shift+N` (`Cmd+Shift+N` on Mac) |
| **Newt: Check current file** | Validate the active file without running | — |
| **Newt: Export to HTML** | Export the current file to HTML | — |
| **Newt: Open Canvas IDE (live preview)** | Start a live-preview dev server | `Ctrl+Shift+L` (`Cmd+Shift+L` on Mac) |

## Extension Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `newt.compilerPath` | `string` | `"newter-compiler"` | Path to the newter-compiler binary |
| `newt.lsp.enabled` | `boolean` | `true` | Enable LSP diagnostics (requires newter-lsp) |
| `newt.lsp.path` | `string` | `"newter-lsp"` | Path to the newter-lsp binary |
