# Show HN: Newt — A UI language that compiles to canvas, HTML, and JSON

Newt is a declarative UI language. You describe interfaces in `.newt` files using 73 built-in elements, and the compiler outputs GPU-accelerated canvas, static HTML, or structured JSON from the same source.

I built it because describing a UI shouldn't require picking a framework first. Every project starts with the same question — React? Svelte? Something else? — before you've written a single layout. Newt skips that. You declare what you want, then choose the output target at compile time.

What makes it different: one source file, three compilation targets. The canvas backend renders via GPU for performance-critical interfaces. The HTML backend produces static markup with no runtime. The JSON backend gives you a serializable UI tree for tooling, server-driven UI, or further transformation.

Current state: the compiler is written in Rust. There's a VS Code extension with syntax highlighting and diagnostics, and a Canvas IDE with hot reload for interactive development. It's early but functional.

Looking for feedback on the language design and compilation model. If you've thought about separating UI description from UI rendering, I'd like to hear your take.

GitHub: https://github.com/voxmastery/newter
Docs: https://newter.vercel.app
Discord: https://discord.gg/s5ZjeNN8H
VS Code Marketplace: search "Newt"
