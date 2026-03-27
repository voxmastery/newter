---
title: HTML Export
description: Export Newt programs as self-contained HTML files for deployment or sharing.
keywords: [newt, html, export, build, static, deploy]
sidebar_position: 3
---

# HTML Export

The `build --html` command compiles a Newt program into a single, self-contained HTML file that can be opened in any browser or deployed to any static hosting service.

## Basic usage

```bash
newter-compiler build dashboard.newt --html -o dashboard.html
```

This produces `dashboard.html` -- a complete file with no external dependencies.

## What the output looks like

Each Newt element becomes a positioned `<div>` with inline styles computed by the layout engine. The output structure follows this pattern:

- A root container sized to the viewport (960 x 640 pixels by default)
- Nested divs for every element, each with `position: absolute`, `left`, `top`, `width`, and `height` set in pixels
- Inline styles for all visual props: `background-color` for `fill`, `border` for `stroke`, `border-radius` for `radius`, `font-size` for `fontSize`, and so on
- Theme variables are included as CSS custom properties on the root element

## Static vs reactive output

The compiler produces different output depending on whether the program uses state:

### Static programs (no state)

Programs that use only `let` variables and have no `onClick` handlers produce pure HTML with no JavaScript. The output is a static snapshot of the UI.

```newt
let title = "Hello";

screen Main {
    center(fill: #f9fafb)(
        text { content: title, fontSize: 32, fontWeight: "700" }
    )
}
```

This compiles to simple HTML and CSS -- no scripts, no runtime.

### Reactive programs (with state)

Programs that declare `state` variables and use `onClick` handlers produce HTML with embedded JavaScript. The JavaScript handles:

- State variable storage
- Event listener registration for `onClick` handlers
- DOM updates when state changes
- String interpolation re-evaluation

```newt
state count = 0;

screen Main {
    column(gap: 16, padding: 48)(
        text("Count: {count}", fontSize: 24)
        button("+1", fill: #2563eb, radius: 8, onClick: { count = count + 1 })
    )
}
```

This compiles to HTML with a small JavaScript runtime that manages the counter state and updates the text element when the button is clicked.

## Selecting a screen

If your file defines multiple screens, use `--screen` to pick which one to export:

```bash
newter-compiler build app.newt --html --screen Dashboard -o dashboard.html
newter-compiler build app.newt --html --screen Settings -o settings.html
```

If `--screen` is omitted, the first screen in the file is used.

## Viewport size

The default viewport is 960 x 640 pixels. The exported HTML file sets the root container to these dimensions. Elements are positioned absolutely within this viewport.

## Auto-rebuild with watch

For iterative development, use `watch` instead of `build` to automatically regenerate the HTML file every time you save:

```bash
newter-compiler watch dashboard.newt --html -o dashboard.html
```

This watches the source file and runs the build pipeline on every change, so you can keep a browser tab open and refresh to see updates.

## Deployment

The output file is fully self-contained. To deploy it:

1. Upload `dashboard.html` to any static hosting service (Netlify, Vercel, GitHub Pages, S3, or a simple web server).
2. No build step is needed on the server -- the file is ready to serve as-is.
3. For programs with state, the JavaScript runtime is embedded inline, so there are no external script dependencies.

## Next steps

- [Canvas IDE](/docs/compiler/canvas-ide) — preview your UI live before exporting.
- [CLI Reference](/docs/compiler/cli) — use `build --html` to generate HTML output.
- [Getting Started](/docs/getting-started) — build your first Newt program from scratch.
