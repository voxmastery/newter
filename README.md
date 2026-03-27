# Newt

**A UI language that compiles to canvas, HTML, and JSON.**

Describing a UI shouldn't require picking a framework first. Newt gives you 58 built-in elements, reactive state, and a syntax you can learn in 5 minutes. Write once, compile to a GPU canvas, static HTML, or a JSON layout tree.

> **Status: alpha (v0.1)** — functional but the language is evolving. Expect breaking changes.

```newt
state count = 0

screen Counter {
    column(gap: 24, padding: 32)(
        text("Count: {count}", fontSize: 32)
        row(gap: 12)(
            button("+ Add", fill: #2563eb, radius: 8, onClick: { count = count + 1 })
            button("Reset", fill: #6b7280, radius: 8, onClick: { count = 0 })
        )
    )
}
```

## Install

```bash
cargo install newter-compiler
```

Or download a pre-built binary from [GitHub Releases](https://github.com/voxmastery/newter/releases).

## Quick start

```bash
# Create hello.newt with the code above, then:
newter-compiler serve hello.newt
# Open http://localhost:3333
```

Three commands, three output targets:

| Command | What it does |
|---------|-------------|
| `newter-compiler run app.newt` | GPU-rendered canvas window |
| `newter-compiler serve app.newt` | Browser IDE with hot reload |
| `newter-compiler build app.newt --html -o out.html` | Standalone HTML file |

## What's in the box

- **58 built-in elements** — button, card, modal, grid, chart, carousel, and more
- **Reactive state** — `state count = 0` with automatic re-rendering
- **Components** — reusable UI blocks with parameters
- **Themes** — named variable sets applied with `use theme`
- **String interpolation** — `"Hello {name}"`
- **Control flow** — `if/else`, `for` loops
- **Multi-file** — `import "components.newt"`

## VS Code extension

Search **"Newt"** in the VS Code marketplace, or:

```bash
code --install-extension voxmastery.newt
```

Syntax highlighting, diagnostics, completions, hover info, and go-to-definition.

## Documentation

Full docs at [newter.vercel.app](https://newter.vercel.app):

- [Getting Started](https://newter.vercel.app/docs/getting-started) — install + hello world in 60 seconds
- [Language Reference](https://newter.vercel.app/docs/language) — elements, props, components, state, themes
- [Compiler Guide](https://newter.vercel.app/docs/compiler) — pipeline, CLI, HTML export, Canvas IDE
- [Examples](https://newter.vercel.app/docs/examples) — counter, dashboard, form, landing page

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to build from source, run tests, and submit pull requests.

## License

MIT
