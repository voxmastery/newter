---
title: Canvas IDE
description: The Canvas IDE provides live-reload development with a browser-based preview and API endpoints.
keywords: [newt, canvas ide, serve, live reload, websocket, api]
sidebar_position: 4
---

# Canvas IDE

The Canvas IDE is a browser-based development environment that renders your Newt UI with live-reload. Start it with the `serve` command and open it in your browser.

## Starting the IDE

```bash
newter-compiler serve dashboard.newt
```

This starts an Axum web server at [http://localhost:3333](http://localhost:3333) and opens your default browser automatically.

## Configuration

| Flag       | Default    | Description                 |
|------------|------------|-----------------------------|
| `--port`   | `3333`     | Port to listen on           |
| `--host`   | `0.0.0.0`  | Host address to bind to    |
| `--screen` | first screen | Screen to render          |

```bash
newter-compiler serve app.newt --port 8080 --host 127.0.0.1 --screen Settings
```

## Live reload

The IDE watches your `.newt` file for changes using a file system watcher. When you save the file in your editor:

1. The watcher detects the modification.
2. The compiler re-parses and re-evaluates the source.
3. The updated layout is sent to the browser via WebSocket.
4. The browser redraws the UI without a full page reload.

This feedback loop is fast enough that changes appear within milliseconds of saving.

## API endpoints

The Canvas IDE exposes several HTTP endpoints for programmatic access.

### POST /api/compile

Triggers a recompile with optional state overrides. Send a JSON body with state values to preview different states without modifying the source file.

```bash
curl -X POST http://localhost:3333/api/compile \
  -H "Content-Type: application/json" \
  -d '{"state": {"count": 42, "darkMode": true}}'
```

The response contains the compiled layout tree as JSON.

### GET /api/source

Returns the current source code of the loaded `.newt` file as plain text.

```bash
curl http://localhost:3333/api/source
```

### GET /api/layout

Returns the computed layout tree as JSON. Each element has its resolved position (`x`, `y`) and size (`w`, `h`), along with all evaluated props.

```bash
curl http://localhost:3333/api/layout
```

This is useful for testing and debugging layout behavior programmatically.

## WebSocket connection

The IDE serves a WebSocket endpoint that the browser client connects to for live updates. When the file changes, the server pushes a message containing the new layout data. The browser-side JavaScript receives this message and updates the canvas without a page refresh.

You do not need to interact with the WebSocket directly -- it is handled automatically by the IDE's built-in client.

## Multiple screens

If your file defines multiple screens, you can switch between them using the `--screen` flag:

```bash
newter-compiler serve app.newt --screen Dashboard
```

To switch screens without restarting the server, update the `--screen` value in the serve command. A future version will support screen switching from within the IDE interface.

## Using with the VS Code extension

For the best development experience, use the Canvas IDE alongside the [VS Code extension](../tooling/vscode.md). The extension provides syntax highlighting, error diagnostics, and completions in your editor, while the Canvas IDE shows the live preview in your browser. Save in VS Code, and the preview updates instantly.

## Next steps

- [CLI Reference](/docs/compiler/cli) — all flags for `serve` and other commands.
- [HTML Export](/docs/compiler/html-export) — export your UI as a standalone HTML file.
- [VS Code Extension](/docs/tooling/vscode) — get syntax highlighting and diagnostics in your editor.
