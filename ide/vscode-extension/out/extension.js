"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = require("vscode");
const path = require("path");
const child_process_1 = require("child_process");
const node_1 = require("vscode-languageclient/node");
const LANGUAGE_ID = "newt";
function getCompilerPath() {
    return vscode.workspace.getConfiguration("newt").get("compilerPath") ?? "newter-compiler";
}
function getLspPath() {
    return vscode.workspace.getConfiguration("newt").get("lsp.path") ?? "newter-lsp";
}
function getActiveNewtFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor)
        return null;
    const file = editor.document.uri.fsPath;
    if (!file.endsWith(".newt"))
        return null;
    return file;
}
function runCompiler(args, cwd, token) {
    const cmd = getCompilerPath();
    return new Promise((resolve) => {
        const proc = (0, child_process_1.spawn)(cmd, args, {
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
function activate(context) {
    try {
        registerCommands(context);
        startLspDeferred(context);
    }
    catch (e) {
        console.error("Newt extension failed to activate", e);
    }
}
function registerCommands(context) {
    context.subscriptions.push(vscode.commands.registerCommand("newt.run", async () => {
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
    }));
    context.subscriptions.push(vscode.commands.registerCommand("newt.serve", async () => {
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
        vscode.window.showInformationMessage("Newt Canvas IDE starting at http://localhost:3333 — opening in browser...");
    }));
    context.subscriptions.push(vscode.commands.registerCommand("newt.check", async () => {
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
        }
        else {
            vscode.window.showErrorMessage("Check failed.");
            const doc = await vscode.workspace.openTextDocument(file);
            const channel = vscode.window.createOutputChannel("Newt");
            channel.clear();
            channel.append(out || stderr);
            channel.show();
        }
    }));
    context.subscriptions.push(vscode.commands.registerCommand("newt.exportHtml", async () => {
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
        }
        else {
            vscode.window.showErrorMessage("Export failed.");
            const channel = vscode.window.createOutputChannel("Newt");
            channel.clear();
            channel.append([stdout, stderr].filter(Boolean).join("\n"));
            channel.show();
        }
    }));
}
function startLspDeferred(context) {
    const lspEnabled = vscode.workspace.getConfiguration("newt").get("lsp.enabled") ?? true;
    if (!lspEnabled)
        return;
    setImmediate(() => {
        try {
            const lspPath = getLspPath();
            const serverOptions = {
                run: { command: lspPath, args: [], options: { env: process.env } },
                debug: { command: lspPath, args: [], options: { env: process.env } },
            };
            const clientOptions = {
                documentSelector: [{ scheme: "file", language: LANGUAGE_ID }],
            };
            const lspClient = new node_1.LanguageClient("newterLsp", "Newt Language Server", serverOptions, clientOptions);
            context.subscriptions.push(lspClient);
            void lspClient.start();
        }
        catch (e) {
            console.warn("Newt: LSP failed to start", e);
        }
    });
}
function deactivate() { }
//# sourceMappingURL=extension.js.map