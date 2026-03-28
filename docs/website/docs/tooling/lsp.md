---
title: LSP Server
description: The Newt Language Server Protocol implementation for editor integration.
keywords: [newt, lsp, language server, editor, neovim, emacs, helix]
sidebar_position: 2
---

# LSP Server

The Newt LSP server (`newter-lsp`) implements the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) to provide IDE features for Newt files in any compatible editor.

## Installation

### Via Cargo

```bash
cargo install newter-lsp
```

This builds and installs the `newter-lsp` binary to your Cargo bin directory.

### Via the VS Code extension

If you use VS Code, the [Newt extension](./vscode.md) bundles the LSP server. No separate installation is needed.

## Capabilities

The Newt LSP server supports the following LSP methods:

### textDocument/completion

Provides context-aware completions for:

- Built-in element names (all 73 elements)
- Prop names within element declarations
- Variable names (`let` and `state`)
- Component names
- Keywords (`screen`, `component`, `state`, `let`, `theme`, `use`, `import`, `if`, `else`, `for`)

### textDocument/hover

Returns documentation on hover for:

- Elements: description and list of accepted props
- Props: type, default value, and description
- Variables: declared type and value
- Components: name and parameter list

### textDocument/definition

Jumps to the definition of:

- Variables (navigates to the `let` or `state` declaration)
- Components (navigates to the `component` definition)
- Imported files (opens the referenced `.newt` file)

### textDocument/publishDiagnostics

Publishes real-time diagnostics as you edit. The server re-parses the file on every change and reports:

- Syntax errors with line and column positions
- Undefined variable references
- Unknown element names
- Type mismatches in prop values
- Circular import detection

## Editor configuration

### Neovim (nvim-lspconfig)

Add the following to your Neovim LSP configuration:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

configs.newt = {
  default_config = {
    cmd = { 'newter-lsp' },
    filetypes = { 'newt' },
    root_dir = lspconfig.util.find_git_ancestor,
  },
}

lspconfig.newt.setup({})
```

You also need to associate the `.newt` file extension with the `newt` filetype:

```lua
vim.filetype.add({
  extension = {
    newt = 'newt',
  },
})
```

### Helix

Add to your `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "newt"
scope = "source.newt"
file-types = ["newt"]
language-servers = ["newter-lsp"]
comment-token = "//"

[language-server.newter-lsp]
command = "newter-lsp"
```

### Emacs (lsp-mode)

Add to your Emacs configuration:

```elisp
(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(newt-mode . "newt"))
  (lsp-register-client
   (make-lsp-client
    :new-connection (lsp-stdio-connection '("newter-lsp"))
    :activation-fn (lsp-activate-on "newt")
    :server-id 'newter-lsp)))
```

### Other editors

Any editor that supports LSP can use the Newt language server. Point the editor's LSP client to the `newter-lsp` binary and associate it with the `.newt` file extension.

## Troubleshooting

### The LSP server is not starting

Verify that `newter-lsp` is on your `PATH`:

```bash
which newter-lsp
newter-lsp --help
```

If not found, check that `~/.cargo/bin` is in your `PATH`.

### Diagnostics are not appearing

The LSP server only activates for files with the `.newt` extension. Make sure your file is saved with the correct extension and that your editor associates `.newt` files with the Newt language.

### Completions are slow

The LSP server re-parses the entire file on each keystroke. For very large files (thousands of lines), consider splitting the code into smaller files using [imports](../language/imports.md).

## Next steps

- [VS Code Extension](/docs/tooling/vscode) — the recommended editor with built-in LSP support.
- [CLI Reference](/docs/compiler/cli) — compile and preview from the command line.
- [Getting Started](/docs/getting-started) — install Newt and write your first program.
