import * as vscode from "vscode";
import * as path from "path";
import { spawn } from "child_process";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

const LANGUAGE_ID = "newt";

function getCompilerPath(): string {
  return vscode.workspace.getConfiguration("newt").get<string>("compilerPath") ?? "newter-compiler";
}

function getLspPath(): string {
  return vscode.workspace.getConfiguration("newt").get<string>("lsp.path") ?? "newter-lsp";
}

function getActiveNewtFile(): string | null {
  const editor = vscode.window.activeTextEditor;
  if (!editor) return null;
  const file = editor.document.uri.fsPath;
  if (!file.endsWith(".newt")) return null;
  return file;
}

function runCompiler(
  args: string[],
  cwd: string,
  token: vscode.CancellationToken
): Promise<{ stdout: string; stderr: string; code: number }> {
  const cmd = getCompilerPath();
  return new Promise((resolve) => {
    const proc = spawn(cmd, args, {
      cwd,
      shell: true,
    });
    let stdout = "";
    let stderr = "";
    proc.stdout?.on("data", (d) => (stdout += d.toString()));
    proc.stderr?.on("data", (d) => (stderr += d.toString()));
    proc.on("close", (code) => resolve({ stdout, stderr, code: code ?? 1 }));
    token.onCancellationRequested(() => proc.kill());
  });
}

export function activate(context: vscode.ExtensionContext) {
  try {
    registerCommands(context);
    startLspDeferred(context);
  } catch (e) {
    console.error("Newt extension failed to activate", e);
  }
}

function registerCommands(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand("newt.run", async () => {
      const file = getActiveNewtFile();
      if (!file) {
        vscode.window.showWarningMessage("Open a .newt file to run.");
        return;
      }
      const cwd = path.dirname(file);
      const term = vscode.window.createTerminal({
        name: "Newt Run",
        cwd,
        hideFromUser: false,
      });
      const cmd = getCompilerPath();
      term.show();
      term.sendText(`${cmd} run "${file}"`);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("newt.serve", async () => {
      const file = getActiveNewtFile();
      if (!file) {
        vscode.window.showWarningMessage("Open a .newt file to serve.");
        return;
      }
      const cwd = path.dirname(file);
      const term = vscode.window.createTerminal({
        name: "Newt Canvas IDE",
        cwd,
        hideFromUser: false,
      });
      const cmd = getCompilerPath();
      term.show();
      term.sendText(`${cmd} serve "${file}"`);
      vscode.window.showInformationMessage(
        "Newt Canvas IDE starting at http://localhost:3333 — opening in browser..."
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("newt.check", async () => {
      const file = getActiveNewtFile();
      if (!file) {
        vscode.window.showWarningMessage("Open a .newt file to check.");
        return;
      }
      const cwd = path.dirname(file);
      const token = new vscode.CancellationTokenSource().token;
      const { stdout, stderr, code } = await runCompiler(["check", file], cwd, token);
      const out = [stdout, stderr].filter(Boolean).join("\n").trim();
      if (code === 0) {
        vscode.window.showInformationMessage(out || "Check ok.");
      } else {
        vscode.window.showErrorMessage("Check failed.");
        const doc = await vscode.workspace.openTextDocument(file);
        const channel = vscode.window.createOutputChannel("Newt");
        channel.clear();
        channel.append(out || stderr);
        channel.show();
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("newt.exportHtml", async () => {
      const file = getActiveNewtFile();
      if (!file) {
        vscode.window.showWarningMessage("Open a .newt file to export.");
        return;
      }
      const outPath = path.join(path.dirname(file), "out.html");
      const cwd = path.dirname(file);
      const token = new vscode.CancellationTokenSource().token;
      const { stdout, stderr, code } = await runCompiler(["--html", outPath, file], cwd, token);
      if (code === 0) {
        vscode.window.showInformationMessage(`Exported to ${outPath}`);
      } else {
        vscode.window.showErrorMessage("Export failed.");
        const channel = vscode.window.createOutputChannel("Newt");
        channel.clear();
        channel.append([stdout, stderr].filter(Boolean).join("\n"));
        channel.show();
      }
    })
  );
}

function startLspDeferred(context: vscode.ExtensionContext) {
  const lspEnabled = vscode.workspace.getConfiguration("newt").get<boolean>("lsp.enabled") ?? true;
  if (!lspEnabled) return;
  setImmediate(() => {
    try {
      const lspPath = getLspPath();
      const serverOptions: ServerOptions = {
        run: { command: lspPath, args: [], options: { env: process.env } },
        debug: { command: lspPath, args: [], options: { env: process.env } },
      };
      const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: LANGUAGE_ID }],
      };
      const lspClient = new LanguageClient(
        "newterLsp",
        "Newt Language Server",
        serverOptions,
        clientOptions
      );
      context.subscriptions.push(lspClient);
      void lspClient.start();
    } catch (e) {
      console.warn("Newt: LSP failed to start", e);
    }
  });
}

export function deactivate() {}
