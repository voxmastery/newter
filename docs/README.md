# Newter documentation

- **[COMPILER_GUIDE.md](./COMPILER_GUIDE.md)** — Deep dive on the Newt compiler: flowcharts (Mermaid), stages (lexer → parser → context → layout → render), data structures, and an end-to-end example.

- **[LANGUAGE_SPEC.md](./LANGUAGE_SPEC.md)** — Newt as a **UI-specialized language**: call-style syntax only (`screen(mobile) { header(...) container(...) }`). No plain English — screens, sections (header, footer, container, sidebar), layout (row, column, card), and controls (button, input, …). Run the program → it compiles and shows on the canvas.

For high-level architecture and task list, see the repo root: `ANALYSIS.md` and `TASKS.md`.
