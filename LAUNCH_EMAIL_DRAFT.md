# Show HN: Newt – A UI language that compiles to canvas, HTML, React, and JSON

8 lines of Newt replaces 20+ lines of React JSX — and compiles to four targets from one source.

I built Newt because describing a UI shouldn't require picking a framework first. You write what you want to see, then choose the output: GPU canvas, static HTML, a React component, or a JSON layout tree.

```
newter-compiler build app.newt --html     # standalone HTML
newter-compiler build app.newt --react    # React component with useState
newter-compiler build app.newt --json     # layout tree for server-driven UI
newter-compiler run app.newt              # GPU canvas window
```

The compiler is written in Rust. 73 built-in elements, reactive state, components, themes, string interpolation, and imports. There's a VS Code extension with full LSP support, and a Canvas IDE with hot reload.

It's alpha (v0.1) — functional but evolving. Looking for feedback on the language design and the multi-target compilation model.

GitHub: https://github.com/voxmastery/newter
Docs: https://newter.vercel.app
Discord: https://discord.gg/s5ZjeNN8H
LLM context doc: NEWT_FOR_LLMS.md (so Claude/GPT can write Newt code)
