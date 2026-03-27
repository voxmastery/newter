//! LSP backend: didOpen / didChange → compile (parse + resolve_imports + layout) → publishDiagnostics.
//! Supports completion, hover, and go-to-definition via symbol table from AST.

use newter_compiler::{
    compile, parse, check_all, symbol_table, completion_keywords, completion_element_names,
    completion_prop_names, NewtError, SymbolKind,
};
use std::collections::HashMap;
use std::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::lsp_types::OneOf;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: RwLock<HashMap<Url, DocumentState>>,
}

#[derive(Debug, Clone)]
struct DocumentState {
    version: i32,
    text: String,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::new(HashMap::new()),
        }
    }

    fn diagnostics_for(&self, uri: &Url, text: &str, _version: i32) -> Vec<Diagnostic> {
        let path = uri.to_file_path().ok();
        let path_ref = path.as_ref().map(|p| p.as_path());
        let errors = check_all(text, path_ref);
        errors.into_iter().map(error_to_diagnostic).collect()
    }
}

fn error_to_diagnostic(err: NewtError) -> Diagnostic {
    let (range, message) = match &err {
        NewtError::Lexer { span, message, .. } | NewtError::Parse { span, message, .. } | NewtError::Semantic { span, message, .. } => {
            let start = Position {
                line: span.line.saturating_sub(1) as u32,
                character: span.column.saturating_sub(1) as u32,
            };
            let end_len = (span.end - span.start).min(1) as u32;
            let end = Position {
                line: start.line,
                character: start.character.saturating_add(end_len),
            };
            let mut msg = message.clone();
            if let Some(hint) = err.suggestion() {
                msg.push_str("\n  hint: ");
                msg.push_str(hint);
            }
            (Range { start, end }, msg)
        }
        _ => (
            Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: 0 },
            },
            err.to_string(),
        ),
    };
    Diagnostic {
        range,
        message,
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("newt".into()),
        related_information: None,
        tags: None,
        data: None,
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: None,
                    trigger_characters: None,
                    all_commit_characters: None,
                    work_done_progress_options: Default::default(),
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "newter-lsp".into(),
                version: Some("0.1.0".into()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Newt LSP server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        let text = params.text_document.text;
        {
            let mut docs = self.documents.write().unwrap();
            docs.insert(
                uri.clone(),
                DocumentState {
                    version,
                    text: text.clone(),
                },
            );
        }
        let diags = self.diagnostics_for(&uri, &text, version);
        self.client.publish_diagnostics(uri, diags, Some(version)).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        let mut full_text = self
            .documents
            .read()
            .unwrap()
            .get(&uri)
            .map(|s| s.text.clone())
            .unwrap_or_default();

        for event in params.content_changes {
            match event {
                TextDocumentContentChangeEvent {
                    range: Some(range),
                    range_length: _,
                    text,
                } => {
                    let start = offset_from_position(&full_text, range.start);
                    let end = offset_from_position(&full_text, range.end);
                    if start <= end && end <= full_text.len() {
                        full_text.replace_range(start..end, &text);
                    }
                }
                TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text,
                } => full_text = text,
                _ => {}
            }
        }

        {
            let mut docs = self.documents.write().unwrap();
            docs.insert(
                uri.clone(),
                DocumentState {
                    version,
                    text: full_text.clone(),
                },
            );
        }
        let diags = self.diagnostics_for(&uri, &full_text, version);
        self.client.publish_diagnostics(uri, diags, Some(version)).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.write().unwrap().remove(&params.text_document.uri);
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let text = self
            .documents
            .read()
            .unwrap()
            .get(&uri)
            .map(|d| d.text.clone())
            .unwrap_or_default();
        let path = uri.to_file_path().ok();
        let path_ref = path.as_ref().map(|p| p.as_path());
        let offset = offset_from_position(&text, position);

        let tokens = match newter_compiler::tokenize(&text) {
            Ok(t) => t,
            Err(_) => return Ok(Some(CompletionResponse::Array(vec![]))),
        };
        let (_current, prefix_offset, prev, prev_prev) = token_at_offset(&tokens, offset);

        let prefix = prefix_offset
            .map(|s| text.get(s..offset).unwrap_or("").to_lowercase())
            .unwrap_or_default();

        let mut items: Vec<CompletionItem> = Vec::new();

        let in_prop_context = prev.as_ref().and_then(|(span, _)| {
            let t = text.get(span.start..span.end)?;
            if t != "(" && t != "{" && t != "," {
                return None;
            }
            let prev_span = prev_prev.as_ref()?.0;
            let prev_text = text.get(prev_span.start..prev_span.end)?.to_lowercase();
            let elements: std::collections::HashSet<_> =
                completion_element_names().into_iter().map(|s| s.to_lowercase()).collect();
            if elements.contains(prev_text.as_str()) {
                Some(())
            } else {
                None
            }
        }).is_some();

        if in_prop_context {
            for name in completion_prop_names() {
                if prefix.is_empty() || name.to_lowercase().starts_with(prefix.as_str()) {
                    items.push(CompletionItem {
                        label: name.to_string(),
                        label_details: None,
                        kind: Some(CompletionItemKind::PROPERTY),
                        detail: None,
                        documentation: None,
                        deprecated: None,
                        preselect: None,
                        sort_text: None,
                        filter_text: None,
                        insert_text: None,
                        insert_text_format: None,
                        insert_text_mode: None,
                        text_edit: None,
                        additional_text_edits: None,
                        command: None,
                        commit_characters: None,
                        data: None,
                        tags: None,
                    });
                }
            }
        } else {
            for kw in completion_keywords() {
                if prefix.is_empty() || kw.starts_with(prefix.as_str()) {
                    items.push(CompletionItem {
                        label: kw.to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        detail: None,
                        documentation: Some(Documentation::String(
                            keyword_hover_label(kw).to_string(),
                        )),
                        ..default_completion_item()
                    });
                }
            }
            for el in completion_element_names() {
                if prefix.is_empty() || el.to_lowercase().starts_with(prefix.as_str()) {
                    items.push(CompletionItem {
                        label: el.to_string(),
                        kind: Some(CompletionItemKind::CLASS),
                        detail: Some("element".to_string()),
                        ..default_completion_item()
                    });
                }
            }
            if let Some(program) = get_program(&text, path_ref) {
                let symbols = symbol_table(&program);
                for (name, (_span, kind)) in &symbols {
                    let ok = prefix.is_empty()
                        || name.to_lowercase().starts_with(prefix.as_str());
                    if !ok {
                        continue;
                    }
                    let (detail, kind_lsp) = match kind {
                        SymbolKind::Variable => ("variable".to_string(), CompletionItemKind::VARIABLE),
                        SymbolKind::Component { params } => (
                            format!("component({})", params.join(", ")),
                            CompletionItemKind::FUNCTION,
                        ),
                        SymbolKind::Screen => ("screen".to_string(), CompletionItemKind::CLASS),
                        SymbolKind::Theme => ("theme".to_string(), CompletionItemKind::MODULE),
                    };
                    items.push(CompletionItem {
                        label: name.clone(),
                        kind: Some(kind_lsp),
                        detail: Some(detail),
                        ..default_completion_item()
                    });
                }
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let text = self
            .documents
            .read()
            .unwrap()
            .get(&uri)
            .map(|d| d.text.clone())
            .unwrap_or_default();
        let path = uri.to_file_path().ok();
        let path_ref = path.as_ref().map(|p| p.as_path());
        let offset = offset_from_position(&text, position);

        let tokens = match newter_compiler::tokenize(&text) {
            Ok(t) => t,
            Err(_) => return Ok(None),
        };
        let (current, _, _, _) = token_at_offset(&tokens, offset);

        let (span, _cat) = match &current {
            Some(s) => s,
            None => return Ok(None),
        };
        let name = match text.get(span.start..span.end) {
            Some(s) => s,
            None => return Ok(None),
        };

        if let Some(program) = get_program(&text, path_ref) {
            let symbols = symbol_table(&program);
            if let Some((def_span, kind)) = symbols.get(name) {
                let (label, doc) = match kind {
                    SymbolKind::Variable => ("variable".to_string(), None),
                    SymbolKind::Component { params } => (
                        format!("component {}({})", name, params.join(", ")),
                        Some("Reusable component.".to_string()),
                    ),
                    SymbolKind::Screen => (
                        format!("screen {}", name),
                        Some("Root view.".to_string()),
                    ),
                    SymbolKind::Theme => (
                        format!("theme {}", name),
                        Some("Design tokens.".to_string()),
                    ),
                };
                let mut md = format!("**{}**", label);
                if let Some(d) = doc {
                    md.push_str("\n\n");
                    md.push_str(&d);
                }
                return Ok(Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(md)),
                    range: Some(span_to_range(def_span, &text)),
                }));
            }
        }

        let kw_lower = name.to_lowercase();
        if completion_keywords().iter().any(|k| *k == kw_lower) {
            return Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    keyword_hover_label(&kw_lower).to_string(),
                )),
                range: Some(span_to_range(span, &text)),
            }));
        }

        if completion_element_names().iter().any(|e| e.to_lowercase() == kw_lower) {
            return Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(format!(
                    "**element** `{}`\n\nLayout or content element.",
                    name
                ))),
                range: Some(span_to_range(span, &text)),
            }));
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let text = self
            .documents
            .read()
            .unwrap()
            .get(&uri)
            .map(|d| d.text.clone())
            .unwrap_or_default();
        let path = uri.to_file_path().ok();
        let path_ref = path.as_ref().map(|p| p.as_path());
        let offset = offset_from_position(&text, position);

        let tokens = match newter_compiler::tokenize(&text) {
            Ok(t) => t,
            Err(_) => return Ok(None),
        };
        let (current, _, _, _) = token_at_offset(&tokens, offset);

        let (span, _cat) = match &current {
            Some(s) => s,
            None => return Ok(None),
        };
        let name = match text.get(span.start..span.end) {
            Some(s) => s,
            None => return Ok(None),
        };

        let program = match get_program(&text, path_ref) {
            Some(p) => p,
            None => return Ok(None),
        };
        let symbols = symbol_table(&program);
        let (def_span, _) = match symbols.get(name) {
            Some(x) => x,
            None => return Ok(None),
        };
        let range = span_to_range(def_span, &text);
        Ok(Some(GotoDefinitionResponse::Scalar(Location { uri, range })))
    }
}

fn default_completion_item() -> CompletionItem {
    CompletionItem {
        label: String::new(),
        label_details: None,
        kind: None,
        detail: None,
        documentation: None,
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: None,
        insert_text_format: None,
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}

fn offset_from_position(text: &str, pos: Position) -> usize {
    let mut offset = 0;
    for (i, line) in text.lines().enumerate() {
        if i == pos.line as usize {
            return offset + (pos.character as usize).min(line.len());
        }
        offset += line.len() + 1;
    }
    offset
}

fn offset_to_position(text: &str, offset: usize) -> Position {
    let mut o = 0;
    for (line_idx, line) in text.lines().enumerate() {
        let line_len = line.len() + 1;
        if o + line.len() >= offset {
            let col = (offset - o).min(line.len());
            return Position {
                line: line_idx as u32,
                character: col as u32,
            };
        }
        o += line_len;
    }
    let line_count = text.lines().count();
    Position {
        line: line_count.saturating_sub(1) as u32,
        character: 0,
    }
}

fn span_to_range(span: &newter_compiler::Span, source: &str) -> Range {
    let start = offset_to_position(source, span.start);
    let end = offset_to_position(source, span.end.min(source.len()));
    Range { start, end }
}

/// Get program for URI (compile with path so imports resolved, or parse only on failure).
fn get_program(text: &str, path: Option<&std::path::Path>) -> Option<newter_compiler::Program> {
    let trimmed = text.trim();
    match compile(trimmed, path, None) {
        Ok((program, _)) => Some(program),
        Err(_) => parse(trimmed, path.and_then(|p| p.to_str())).ok(),
    }
}

/// Find token at offset: (current token containing offset, prefix start, prev token, prev_prev token).
fn token_at_offset(
    tokens: &[(newter_compiler::Span, newter_compiler::TokenCategory)],
    offset: usize,
) -> (
    Option<(newter_compiler::Span, newter_compiler::TokenCategory)>,
    Option<usize>,
    Option<(newter_compiler::Span, newter_compiler::TokenCategory)>,
    Option<(newter_compiler::Span, newter_compiler::TokenCategory)>,
) {
    use newter_compiler::TokenCategory;
    let mut current = None;
    let mut prev = None;
    let mut prev_prev = None;
    for (span, cat) in tokens.iter() {
        if span.start <= offset && offset <= span.end && !matches!(cat, TokenCategory::Eof) {
            current = Some((*span, *cat));
        }
        if span.end <= offset {
            prev_prev = prev;
            prev = Some((*span, *cat));
        }
    }
    let prefix_offset = current.as_ref().map(|(span, _)| span.start);
    (current, prefix_offset, prev, prev_prev)
}

/// One-line descriptions for keywords (hover).
fn keyword_hover_label(kw: &str) -> &'static str {
    match kw {
        "screen" => "Screen (root): defines a named view.",
        "let" => "Variable: binds a value to a name.",
        "component" => "Component: reusable block with parameters.",
        "theme" => "Theme: named set of design tokens.",
        "use" => "Use theme: apply a theme's variables.",
        "import" => "Import: merge another .newt file.",
        "if" => "Conditional: if condition then else.",
        "for" => "Loop: for variable in range(n) or array.",
        "else" => "Else branch of if.",
        "in" => "In: used in for x in ...",
        _ => "Keyword",
    }
}
