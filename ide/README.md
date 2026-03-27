# Newt IDE support

## VS Code extension

- **Location**: `vscode-extension/`
- **Features**: Language id `newt`, syntax highlighting, Run / Check / Export HTML commands, optional LSP diagnostics.

### Do it (build + package + install)

From repo root:

```bash
# 1. Build compiler and LSP
cargo build -p newter-compiler -p newter-lsp

# 2. Package the extension (creates ide/vscode-extension/newt-language.vsix)
cd ide/vscode-extension && npx @vscode/vsce package --no-dependencies -o newt-language.vsix && cd ../..

# 3. Install the .vsix in VS Code: Command Palette → "Extensions: Install from VSIX..." → choose ide/vscode-extension/newt-language.vsix
```

Put `target/debug` on your `PATH`, or set in VS Code: `newt.compilerPath` and `newt.lsp.path` to the full paths of the binaries.

### Setup (reference)

1. **Compiler and LSP** (from repo root):
   ```bash
   cargo build -p newter-compiler -p newter-lsp
   ```
   Ensure `target/debug/newter-compiler` and `target/debug/newter-lsp` are on your `PATH`, or set in VS Code:
   - `newt.compilerPath`: path to `newter-compiler`
   - `newt.lsp.path`: path to `newter-lsp`

2. **Install the extension**:
   - Open `ide/vscode-extension` in VS Code, press F5 to run the Extension Development Host, or
   - Package: `cd ide/vscode-extension && npx @vscode/vsce package -o newt-language.vsix`, then install the `.vsix` via Command Palette → "Extensions: Install from VSIX...".

### Commands

- **Newt: Run current file** — runs `newter-compiler run <file>` in a terminal (opens canvas).
- **Newt: Check current file** — runs `newter-compiler check <file>`, shows errors in a message and the Output channel.
- **Newt: Export to HTML** — runs `newter-compiler --html out.html <file>`.

### LSP

If `newt.lsp.enabled` is true (default), the extension starts `newter-lsp` and shows parse/layout diagnostics in the editor. Set `newt.lsp.path` to the full path of the LSP binary if it is not on `PATH`.

### What next?

1. **Open a `.newt` file**  
   e.g. the example: `newter-compiler/examples/screen-header-container.newt` in this repo.

2. **Ensure the tools are on PATH**  
   From repo root run `cargo build -p newter-compiler -p newter-lsp`, then either:
   - Add `target/debug` to your `PATH`, or
   - In VS Code: Settings → search "newt" → set **Newt: Compiler Path** and **Newt: Lsp Path** to the full paths (e.g. `/path/to/newter/target/debug/newter-compiler`).

3. **Try the commands** (Command Palette `Ctrl+Shift+P` / `Cmd+Shift+P`):
   - **Newt: Run current file** — opens the wgpu canvas with live preview; save the file to hot-reload.
   - **Newt: Check current file** — parse + layout only; shows "Check ok" or errors in Output.
   - **Newt: Export to HTML** — writes `out.html` next to the current file.

4. **Edit and see diagnostics**  
   If LSP is running, parse/layout errors show as red squiggles in the editor.
