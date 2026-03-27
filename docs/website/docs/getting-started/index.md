---
title: Getting Started
description: Learn what Newt is and build your first UI in under a minute.
keywords: [newt, getting started, quickstart, ui language]
sidebar_position: 1
---

# Getting Started

Newt is a UI language that compiles to canvas, HTML, and JSON. It gives you a concise, readable syntax for describing user interfaces without the overhead of a full framework. You write what you want to see, and the compiler handles rendering.

## Quickstart

Get from zero to a running UI in three steps.

### 1. Install the compiler

```bash
cargo install newter-compiler
```

### 2. Create a file called `hello.newt`

```newt
screen Main {
    center(fill: #f9fafb)(
        column(gap: 16)(
            text("Hello, Newt!", fontSize: 32, fontWeight: "700")
            text("Your first UI in 3 lines")
        )
    )
}
```

### 3. Run it

```bash
newter-compiler serve hello.newt
```

Open [http://localhost:3333](http://localhost:3333) to see your UI.

You should see a centered column with a bold heading and a subtitle, rendered on a light gray background. The Canvas IDE provides live-reload, so any edits you save will appear instantly in the browser.

## What just happened?

- `screen Main` declared a named screen. Every Newt program needs at least one screen.
- `center(fill: #f9fafb)` created a centered container with a background color.
- `column(gap: 16)` stacked its children vertically with 16 pixels of spacing.
- `text(...)` rendered text with the specified font size and weight.

## Next steps

- [Installation](./installation.md) -- alternative install methods including binary downloads and VS Code setup.
- [Language Overview](../language/index.md) -- learn the full syntax: elements, props, components, state, and more.
- [Examples](../examples/index.md) -- complete programs you can copy and modify.
- [CLI Reference](../compiler/cli.md) -- all compiler commands and flags.
