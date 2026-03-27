---
title: Installation
description: Install the Newt compiler via Cargo, binary download, or set up the VS Code extension.
keywords: [newt, install, cargo, binary, vscode, setup]
sidebar_position: 2
---

# Installation

There are three ways to get the Newt compiler on your machine.

## Option 1: Install via Cargo (recommended)

If you have the Rust toolchain installed, this is the fastest path.

```bash
cargo install newter-compiler
```

This builds the compiler from source and places the `newter-compiler` binary in your Cargo bin directory (usually `~/.cargo/bin/`).

### System requirements

- Rust 1.75 or later (install from [rustup.rs](https://rustup.rs))
- A C linker (gcc, clang, or MSVC depending on your platform)

## Option 2: Download a pre-built binary

Grab the latest release for your platform from GitHub.

| Platform        | Architecture | Download link                                                                                     |
|-----------------|-------------|---------------------------------------------------------------------------------------------------|
| Linux           | x86_64      | [newter-compiler-linux-x86_64](https://github.com/voxmastery/newter/releases/latest/download/newter-compiler-linux-x86_64)       |
| macOS           | ARM64       | [newter-compiler-macos-arm64](https://github.com/voxmastery/newter/releases/latest/download/newter-compiler-macos-arm64)         |
| macOS           | x86_64      | [newter-compiler-macos-x86_64](https://github.com/voxmastery/newter/releases/latest/download/newter-compiler-macos-x86_64)       |
| Windows         | x86_64      | [newter-compiler-windows-x86_64.exe](https://github.com/voxmastery/newter/releases/latest/download/newter-compiler-windows-x86_64.exe) |

After downloading, make the binary executable (Linux/macOS) and move it to a directory on your `PATH`:

```bash
chmod +x newter-compiler-linux-x86_64
sudo mv newter-compiler-linux-x86_64 /usr/local/bin/newter-compiler
```

## Option 3: VS Code extension

The Newt extension for VS Code provides syntax highlighting, diagnostics, completions, and hover information for `.newt` files.

Install it from the VS Code marketplace:

```bash
code --install-extension voxmastery.newt
```

Or open VS Code, go to the Extensions panel, and search for **Newt**.

The extension bundles the Newt LSP server, so you get full language support without any additional setup.

## Verify your installation

Run the help command to confirm everything is working:

```bash
newter-compiler --help
```

You should see the list of available commands including `serve`, `build`, `check`, and `watch`.

## Next steps

Once installed, head to the [Getting Started](./index.md) guide to create your first Newt program, or jump straight to the [Language Overview](../language/index.md) to learn the syntax.
