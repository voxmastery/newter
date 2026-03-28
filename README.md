# Newt

[![Crates.io](https://img.shields.io/crates/v/newter-compiler.svg)](https://crates.io/crates/newter-compiler)
[![CI](https://github.com/voxmastery/newter/actions/workflows/ci.yml/badge.svg)](https://github.com/voxmastery/newter/actions/workflows/ci.yml)
[![Discord](https://img.shields.io/badge/Discord-Join-7c3aed?logo=discord&logoColor=white)](https://discord.gg/s5ZjeNN8H)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A UI language that compiles to canvas, HTML, and JSON.**

Describing a UI shouldn't require picking a framework first. Newt gives you 73 built-in elements, reactive state, and a syntax you can learn in 5 minutes. Write once, compile to a GPU canvas, static HTML, or a JSON layout tree.

> **Status: alpha (v0.1)** — functional but the language is evolving. Expect breaking changes.

## The idea

```
┌─────────────────┐     ┌──────────────┐
│                 │     │ GPU Canvas   │  newter-compiler run
│                 │     │ HTML Page    │  newter-compiler build --html
│   .newt file    │────▶│ React JSX    │  newter-compiler build --react
│                 │     │ JSON Tree    │  newter-compiler build --json
└─────────────────┘     └──────────────┘
```

One source file. Four output targets. No framework, no runtime, no build step.

## Same UI, less code

**Newt (8 lines):**
```newt
screen Main {
    column(gap: 16, padding: 24)(
        text("Hello!", fontSize: 24, fontWeight: "700")
        row(gap: 8)(
            button("Click me", fill: #7c3aed, radius: 8)
            button("Cancel", stroke: #e5e7eb, radius: 8)
        )
    )
}
```

**React equivalent (20+ lines):**
```jsx
function App() {
  return (
    <div style={{display:'flex', flexDirection:'column', gap:'16px', padding:'24px'}}>
      <h1 style={{fontSize:'24px', fontWeight:700}}>Hello!</h1>
      <div style={{display:'flex', gap:'8px'}}>
        <button style={{background:'#7c3aed', color:'#fff', borderRadius:'8px',
          border:'none', padding:'8px 16px'}}>Click me</button>
        <button style={{border:'1px solid #e5e7eb', borderRadius:'8px',
          background:'transparent', padding:'8px 16px'}}>Cancel</button>
      </div>
    </div>
  );
}
```

## Install

```bash
cargo install newter-compiler
```

Or download a pre-built binary from [GitHub Releases](https://github.com/voxmastery/newter/releases).

## Quick start

```bash
# Create hello.newt, then:
newter-compiler serve hello.newt
# Open http://localhost:3333
```

| Command | What it does |
|---------|-------------|
| `newter-compiler run app.newt` | GPU-rendered canvas window |
| `newter-compiler serve app.newt` | Browser IDE with hot reload |
| `newter-compiler build app.newt --html -o out.html` | Standalone HTML file |
| `newter-compiler build app.newt --react -o App.jsx` | React component |
| `newter-compiler build app.newt --json -o out.json` | JSON layout tree |
| `newter-compiler check app.newt` | Validate syntax (for CI) |

## What's in the box

- **73 built-in elements** — button, card, modal, grid, chart, carousel, and more
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

Full docs at **[newter.vercel.app](https://newter.vercel.app)**

- [Getting Started](https://newter.vercel.app/docs/getting-started) — zero to running UI in 60 seconds
- [Language Reference](https://newter.vercel.app/docs/language) — elements, props, components, state, themes
- [Compiler Guide](https://newter.vercel.app/docs/compiler) — pipeline, CLI, HTML export, Canvas IDE
- [Examples](https://newter.vercel.app/docs/examples) — counter, dashboard, form, landing page

## Community

- [Discord](https://discord.gg/s5ZjeNN8H) — questions, feedback, showcase
- [GitHub Issues](https://github.com/voxmastery/newter/issues) — bug reports and feature requests

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to build from source, run tests, and submit pull requests.

## License

MIT
