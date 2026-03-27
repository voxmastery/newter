---
title: CLI Reference
description: Complete reference for all newter-compiler commands, flags, and options.
keywords: [newt, cli, commands, serve, build, check, watch]
sidebar_position: 2
---

# CLI Reference

The `newter-compiler` command-line tool provides five commands for working with Newt files.

## Commands

### Run (default)

```bash
newter-compiler [file]
```

Opens a native wgpu window and renders the UI. If no file is specified, a built-in demo is shown.

| Argument    | Description                           |
|-------------|---------------------------------------|
| `[file]`    | Path to a `.newt` file (optional)     |
| `--screen`  | Name of the screen to render          |

Example:

```bash
newter-compiler dashboard.newt
newter-compiler dashboard.newt --screen Settings
```

### Serve

```bash
newter-compiler serve <file> [--port PORT] [--host HOST] [--screen NAME]
```

Starts the Canvas IDE -- an Axum-based web server that renders your UI in the browser with live-reload.

| Flag       | Default     | Description                        |
|------------|-------------|------------------------------------|
| `<file>`   | required    | Path to a `.newt` file             |
| `--port`   | `3333`      | Port to listen on                  |
| `--host`   | `0.0.0.0`   | Host address to bind to            |
| `--screen` | first screen | Name of the screen to render      |

Example:

```bash
newter-compiler serve app.newt
newter-compiler serve app.newt --port 8080
newter-compiler serve app.newt --screen Dashboard
```

The server watches the file for changes and pushes updates to the browser via WebSocket. Every time you save, the UI refreshes automatically.

### Build

```bash
newter-compiler build <file> --html [-o OUTPUT] [--screen NAME]
```

Compiles a `.newt` file and exports it as a self-contained HTML file.

| Flag       | Default      | Description                        |
|------------|-------------|-------------------------------------|
| `<file>`   | required    | Path to a `.newt` file              |
| `--html`   | required    | Output format (currently only HTML) |
| `-o`       | `out.html`  | Output file path                    |
| `--screen` | first screen | Name of the screen to render       |

Example:

```bash
newter-compiler build dashboard.newt --html
newter-compiler build dashboard.newt --html -o dashboard.html
newter-compiler build dashboard.newt --html --screen Overview -o overview.html
```

### Check

```bash
newter-compiler check <file>
```

Parses and validates a `.newt` file without rendering. Exits with code 0 if the file is valid, or prints errors and exits with a non-zero code.

| Argument  | Description                      |
|-----------|----------------------------------|
| `<file>`  | Path to a `.newt` file           |

Example:

```bash
newter-compiler check app.newt
```

This is useful in CI pipelines or pre-commit hooks to catch syntax errors early.

### Watch

```bash
newter-compiler watch <file> --html [-o OUTPUT] [--screen NAME]
```

Watches a `.newt` file for changes and rebuilds the HTML output on every save. Equivalent to running `build` automatically whenever the file is modified.

| Flag       | Default      | Description                        |
|------------|-------------|-------------------------------------|
| `<file>`   | required    | Path to a `.newt` file              |
| `--html`   | required    | Output format                       |
| `-o`       | `out.html`  | Output file path                    |
| `--screen` | first screen | Name of the screen to render       |

Example:

```bash
newter-compiler watch dashboard.newt --html -o dashboard.html
```

## Global flag

### --screen NAME

All commands that render output accept `--screen` to select which screen to display. If omitted, the first screen defined in the file is used.

```bash
newter-compiler serve app.newt --screen Settings
newter-compiler build app.newt --html --screen Settings -o settings.html
```

## Exit codes

| Code | Meaning              |
|------|----------------------|
| `0`  | Success              |
| `1`  | Compilation error    |
| `2`  | File not found       |
| `3`  | Invalid arguments    |

## Next steps

- [HTML Export](/docs/compiler/html-export) — details on the `--html` build output.
- [Canvas IDE](/docs/compiler/canvas-ide) — how `serve` powers the live preview.
- [Installation](/docs/getting-started/installation) — install the compiler and get started.
