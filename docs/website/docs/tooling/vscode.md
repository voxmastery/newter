---
title: VS Code Extension
description: Install and use the Newt VS Code extension for syntax highlighting, diagnostics, completions, and more.
keywords: [newt, vscode, extension, syntax highlighting, diagnostics, completions]
sidebar_position: 1
---

# VS Code Extension

The Newt extension for Visual Studio Code provides a full editing experience for `.newt` files, powered by the Newt LSP server.

## Installation

### From the marketplace

Open VS Code, go to the Extensions panel (Ctrl+Shift+X / Cmd+Shift+X), and search for **Newt**. Click **Install**.

### From the command line

```bash
code --install-extension voxmastery.newt
```

### From the VS Code quick open

Press Ctrl+P / Cmd+P and paste:

```
ext install voxmastery.newt
```

## Features

### Syntax highlighting

The extension provides full syntax highlighting for `.newt` files:

- **Keywords** like `screen`, `component`, `state`, `let`, `theme`, `use`, `import`, `if`, `else`, `for` are highlighted distinctly.
- **Element names** like `row`, `column`, `text`, `button`, `card` are recognized and colored.
- **Props** are highlighted as property names.
- **Strings**, **numbers**, and **hex colors** each have their own highlighting.
- **Comments** starting with `//` are dimmed.

### Real-time diagnostics

As you type, the extension checks your code for errors and displays them inline:

- Syntax errors (missing braces, unexpected tokens)
- Undefined variable references
- Unknown element names
- Invalid prop values
- Missing required arguments in component calls

Errors appear as red underlines in the editor and in the Problems panel (Ctrl+Shift+M / Cmd+Shift+M).

### Auto-completions

The extension offers context-aware completions:

- **Element names**: start typing and see suggestions for all 58 built-in elements.
- **Prop names**: inside an element's prop list, get suggestions for valid props (e.g., `fill`, `radius`, `padding`, `gap`).
- **Variables**: references to `let` and `state` variables are suggested.
- **Components**: your custom components appear in the completion list alongside built-in elements.

### Hover information

Hover over any element, prop, variable, or component to see a tooltip with:

- For elements: a brief description and list of supported props.
- For props: the expected type and default value.
- For variables: the declared value.
- For components: the parameter list.

### Go-to-definition

Ctrl+Click (Cmd+Click on macOS) on a variable name, component name, or imported file path to jump to its definition. This works for:

- `let` and `state` variable declarations
- `component` definitions
- `import` file paths (opens the imported file)

## Configuration

The extension requires no configuration. It bundles the Newt LSP server and starts it automatically when you open a `.newt` file. The LSP server shuts down when you close the last `.newt` file.

## Recommended workflow

1. Open your project folder in VS Code.
2. Start the Canvas IDE in a terminal: `newter-compiler serve app.newt`
3. Edit `.newt` files in VS Code -- the extension provides diagnostics and completions as you type.
4. Save (Ctrl+S / Cmd+S) -- the Canvas IDE in your browser updates instantly via live-reload.

This gives you editor-quality tooling and a live preview side by side.

## Next steps

- [LSP](/docs/tooling/lsp) — technical details of the language server protocol implementation.
- [Getting Started](/docs/getting-started) — set up your first Newt project with VS Code.
- [Elements](/docs/language/elements) — browse all built-in elements with examples.
