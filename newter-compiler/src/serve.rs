//! Canvas IDE server: HTTP + WebSocket for live-reload canvas.
//!
//! `cargo run -p newter-compiler -- serve path/to/file.newt`
//! Opens http://localhost:3333 with the canvas IDE.

use crate::{compile_with_state, get_screen, screen_names, value_to_json, Source};
use crate::value::{eval_expr, EvalContext, Value};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Query, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::CorsLayer;

/// Shared state across handlers.
#[derive(Clone)]
pub struct AppState {
    pub file_path: PathBuf,
    pub source: Arc<RwLock<String>>,
    pub layout_json: Arc<RwLock<String>>,
    pub default_screen: Option<String>,
    pub tx: broadcast::Sender<String>,
    pub current_state: Arc<std::sync::Mutex<HashMap<String, Value>>>,
}

/// Compile a .newt source to layout JSON for the given screen (None = first screen).
/// Payload includes "screens" list so the IDE can show a screen selector.
/// Accepts state overrides and returns (json_string, effective_state).
fn compile_to_json(
    source: &str,
    path: Option<&str>,
    screen_name: Option<&str>,
    state_overrides: &HashMap<String, Value>,
) -> Result<(String, HashMap<String, Value>), String> {
    let path_obj = path.map(std::path::Path::new);
    let (program, layout, effective_state) = compile_with_state(source, path_obj, screen_name, state_overrides).map_err(|e| {
        let src = Source::new(source.to_string(), path.map(String::from));
        crate::format_error(&src, &e)
    })?;
    let screens = screen_names(&program);
    let screen = get_screen(&program, screen_name).expect("compile guarantees a screen");

    // Serialize state for the client
    let state_json: serde_json::Map<String, serde_json::Value> = effective_state
        .iter()
        .map(|(k, v)| (k.clone(), value_to_json(v)))
        .collect();

    let payload = serde_json::json!({
        "type": "layout",
        "screens": screens,
        "screen": screen.name,
        "viewport": { "w": crate::DEFAULT_VIEWPORT_W, "h": crate::DEFAULT_VIEWPORT_H },
        "root": layout,
        "state": state_json,
    });
    Ok((serde_json::to_string(&payload).unwrap(), effective_state))
}

/// Find the position of `=` in an assignment expression, skipping `==`, `!=`, `<=`, `>=`.
fn find_assignment_split(expr: &str) -> Option<usize> {
    let bytes = expr.as_bytes();
    let len = bytes.len();
    for i in 0..len {
        if bytes[i] == b'=' {
            // Skip ==
            if i + 1 < len && bytes[i + 1] == b'=' {
                continue;
            }
            // Skip !=, <=, >=
            if i > 0 && (bytes[i - 1] == b'!' || bytes[i - 1] == b'<' || bytes[i - 1] == b'>') {
                continue;
            }
            return Some(i);
        }
    }
    None
}

/// Evaluate a state action expression (e.g., "count = count + 1; label = \"hi\"").
/// Returns the updated state map.
fn evaluate_state_action(
    source: &str,
    path: Option<&str>,
    current_state: &HashMap<String, Value>,
    action_expr: &str,
) -> Result<HashMap<String, Value>, String> {
    let path_obj = path.map(std::path::Path::new);
    let trimmed = source.trim();
    let program = crate::parse(trimmed, path.map(|s| s)).map_err(|e| format!("{:?}", e))?;
    let program = if let Some(p) = path_obj {
        let base = p.parent().unwrap_or_else(|| std::path::Path::new("."));
        crate::resolve_imports(program, base).map_err(|e| format!("{:?}", e))?
    } else {
        program
    };

    let mut ctx = EvalContext::from_program(&program);

    // Apply current state overrides
    for (name, val) in current_state {
        ctx.variables.insert(name.clone(), val.clone());
    }

    let mut new_state = current_state.clone();

    // Process each semicolon-separated statement
    for stmt in action_expr.split(';') {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }

        if let Some(eq_pos) = find_assignment_split(stmt) {
            let var_name = stmt[..eq_pos].trim().to_string();
            let rhs = stmt[eq_pos + 1..].trim();

            // Parse and evaluate RHS using a dummy program wrapper
            let wrapper = format!("let __result = {}; screen __X {{ box {{}} }}", rhs);
            match crate::parse(&wrapper, None) {
                Ok(wrapped_program) => {
                    // The first item should be the let binding — evaluate it
                    let rhs_ctx = EvalContext {
                        variables: ctx.variables.clone(),
                        components: ctx.components.clone(),
                    };
                    // Find the let binding and eval its value
                    for item in &wrapped_program.items {
                        if let crate::ProgramItem::Variable(v) = item {
                            if v.name == "__result" {
                                if let Ok(val) = eval_expr(&rhs_ctx, &v.value) {
                                    new_state.insert(var_name.clone(), val.clone());
                                    ctx.variables.insert(var_name.clone(), val);
                                }
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("state_action parse error for '{}': {:?}", stmt, e);
                }
            }
        }
    }

    Ok(new_state)
}

/// Start the canvas IDE server.
pub async fn serve(file_path: PathBuf, port: u16, host: String, screen_name: Option<String>) -> anyhow::Result<()> {
    let source_code = std::fs::read_to_string(&file_path)?;
    let path_str = file_path.to_str().map(|s| s.to_string());

    let (layout_json, initial_state) = match compile_to_json(&source_code, path_str.as_deref(), screen_name.as_deref(), &HashMap::new()) {
        Ok((j, s)) => (j, s),
        Err(e) => (serde_json::json!({ "type": "error", "message": e }).to_string(), HashMap::new()),
    };

    let (tx, _rx) = broadcast::channel::<String>(64);

    let state = AppState {
        file_path: file_path.clone(),
        source: Arc::new(RwLock::new(source_code)),
        layout_json: Arc::new(RwLock::new(layout_json)),
        default_screen: screen_name,
        tx: tx.clone(),
        current_state: Arc::new(std::sync::Mutex::new(initial_state)),
    };

    // File watcher
    let watch_state = state.clone();
    let watch_path = file_path.clone();
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();
        let (notify_tx, notify_rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut watcher = RecommendedWatcher::new(
            move |res| { let _ = notify_tx.send(res); },
            notify::Config::default(),
        ).expect("watcher");
        watcher.watch(watch_path.as_ref(), RecursiveMode::NonRecursive).expect("watch");

        loop {
            match notify_rx.recv() {
                Ok(Ok(event)) => {
                    if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        if let Ok(new_source) = std::fs::read_to_string(&watch_state.file_path) {
                            let path_str = watch_state.file_path.to_str().map(|s| s.to_string());
                            let screen = watch_state.default_screen.as_deref();
                            // Preserve current state on file save
                            let overrides = watch_state.current_state.lock().unwrap().clone();
                            let json = match compile_to_json(&new_source, path_str.as_deref(), screen, &overrides) {
                                Ok((j, new_state)) => {
                                    *watch_state.current_state.lock().unwrap() = new_state;
                                    j
                                }
                                Err(e) => serde_json::json!({ "type": "error", "message": e }).to_string(),
                            };
                            rt.block_on(async {
                                *watch_state.source.write().await = new_source;
                                *watch_state.layout_json.write().await = json.clone();
                            });
                            let _ = watch_state.tx.send(json);
                        }
                    }
                }
                Ok(Err(e)) => eprintln!("watch error: {:?}", e),
                Err(_) => break,
            }
        }
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .route("/api/source", get(source_handler))
        .route("/api/layout", get(layout_handler))
        .route("/api/compile", axum::routing::post(compile_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    println!("Newt Canvas IDE running at http://localhost:{}", port);
    println!("  File: {}", file_path.display());
    println!("  Press Ctrl+C to stop.\n");

    let _ = open::that(format!("http://localhost:{}", port));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// Serve the canvas IDE HTML.
async fn index_handler() -> Html<&'static str> {
    Html(include_str!("canvas/index.html"))
}

/// Return current source.
async fn source_handler(State(state): State<AppState>) -> impl IntoResponse {
    let src = state.source.read().await;
    src.clone()
}

#[derive(Deserialize)]
struct LayoutQuery {
    screen: Option<String>,
}

/// Return layout JSON. ?screen=Name to get a specific screen (for multi-screen apps).
async fn layout_handler(State(state): State<AppState>, Query(q): Query<LayoutQuery>) -> impl IntoResponse {
    let json = if let Some(ref name) = q.screen {
        let src = state.source.read().await;
        let path_str = state.file_path.to_str().map(|s| s.to_string());
        let overrides = state.current_state.lock().unwrap().clone();
        match compile_to_json(&src, path_str.as_deref(), Some(name), &overrides) {
            Ok((j, _)) => j,
            Err(e) => serde_json::json!({ "type": "error", "message": e }).to_string(),
        }
    } else {
        state.layout_json.read().await.clone()
    };
    ([("content-type", "application/json")], json)
}

/// Compile source posted from the editor. Optional JSON body: { "code": "...", "screen": "Name" }.
async fn compile_handler(
    State(state): State<AppState>,
    body: String,
) -> impl IntoResponse {
    let path_str = state.file_path.to_str().map(|s| s.to_string());
    let (code, screen) = if let Ok(v) = serde_json::from_str::<serde_json::Value>(&body) {
        let code = v.get("code").and_then(|c| c.as_str()).unwrap_or(body.as_str()).to_string();
        let screen = v.get("screen").and_then(|s| s.as_str()).map(String::from);
        (code, screen.or(state.default_screen.clone()))
    } else {
        (body.clone(), state.default_screen.clone())
    };
    // Code changed — reset state
    *state.current_state.lock().unwrap() = HashMap::new();
    let json = match compile_to_json(&code, path_str.as_deref(), screen.as_deref(), &HashMap::new()) {
        Ok((j, new_state)) => {
            *state.current_state.lock().unwrap() = new_state;
            j
        }
        Err(e) => serde_json::json!({ "type": "error", "message": e }).to_string(),
    };
    *state.source.write().await = code;
    *state.layout_json.write().await = json.clone();
    let _ = state.tx.send(json.clone());
    ([("content-type", "application/json")], json)
}

/// WebSocket handler for live updates.
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: AppState) {
    // Send current layout immediately
    {
        let json = state.layout_json.read().await;
        let _ = socket.send(Message::Text(json.clone())).await;
    }
    // Send current source + filename
    {
        let src = state.source.read().await;
        let filename = state.file_path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("untitled.newt");
        let msg = serde_json::json!({
            "type": "source",
            "code": *src,
            "filename": filename,
            "path": state.file_path.to_str().unwrap_or(""),
        });
        let _ = socket.send(Message::Text(msg.to_string())).await;
    }

    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(json) => {
                        if socket.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Check for state_action message first
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                            if v.get("type").and_then(|t| t.as_str()) == Some("state_action") {
                                if let Some(expr) = v.get("expr").and_then(|e| e.as_str()) {
                                    let path_str = state.file_path.to_str().map(|s| s.to_string());
                                    let src = state.source.read().await.clone();
                                    let cur_state = state.current_state.lock().unwrap().clone();
                                    match evaluate_state_action(&src, path_str.as_deref(), &cur_state, expr) {
                                        Ok(new_state) => {
                                            *state.current_state.lock().unwrap() = new_state.clone();
                                            let screen = state.default_screen.as_deref();
                                            let json = match compile_to_json(&src, path_str.as_deref(), screen, &new_state) {
                                                Ok((j, effective)) => {
                                                    *state.current_state.lock().unwrap() = effective;
                                                    j
                                                }
                                                Err(e) => serde_json::json!({ "type": "error", "message": e }).to_string(),
                                            };
                                            *state.layout_json.write().await = json.clone();
                                            let _ = state.tx.send(json);
                                        }
                                        Err(e) => {
                                            eprintln!("state_action error: {}", e);
                                        }
                                    }
                                }
                                continue;
                            }
                        }

                        // Client sent source code (or JSON { code, screen }) — compile it
                        let path_str = state.file_path.to_str().map(|s| s.to_string());
                        let (code, screen) = if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                            let code = v.get("code").and_then(|c| c.as_str()).unwrap_or(text.as_str()).to_string();
                            let screen = v.get("screen").and_then(|s| s.as_str()).map(String::from);
                            (code, screen.or(state.default_screen.clone()))
                        } else {
                            (text.clone(), state.default_screen.clone())
                        };
                        // Code changed — reset state
                        *state.current_state.lock().unwrap() = HashMap::new();
                        let json = match compile_to_json(&code, path_str.as_deref(), screen.as_deref(), &HashMap::new()) {
                            Ok((j, new_state)) => {
                                *state.current_state.lock().unwrap() = new_state;
                                j
                            }
                            Err(e) => serde_json::json!({ "type": "error", "message": e }).to_string(),
                        };
                        *state.source.write().await = code;
                        *state.layout_json.write().await = json.clone();
                        let _ = state.tx.send(json);
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}
