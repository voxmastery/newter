//! Application loop: winit window, file watch, and render.

use crate::layout::{layout_tree, LayoutNode, Rect};
use crate::renderer::RendererState;
use crate::value::EvalContext;
use crate::{compile, get_screen, Program};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    window: Option<Window>,
    renderer: Option<RendererState>,
    program: Option<Program>,
    root_layout: Option<LayoutNode>,
    source_path: Option<PathBuf>,
    source_code: String,
    screen_name: Option<String>,
    reload_rx: Option<mpsc::Receiver<()>>,
}

impl App {
    pub fn new(source_path: Option<PathBuf>, source_code: String, screen_name: Option<String>) -> Self {
        Self {
            window: None,
            renderer: None,
            program: None,
            root_layout: None,
            source_path,
            source_code,
            screen_name,
            reload_rx: None,
        }
    }

    fn load_and_layout(&mut self) {
        let source = self.source_code.trim();
        if source.is_empty() {
            log::error!("No source code to parse");
            return;
        }
        let path = self.source_path.as_ref().map(|p| p.as_path());
        match compile(source, path, self.screen_name.as_deref()) {
            Ok((program, _layout)) => {
                self.program = Some(program);
                self.rebuild_layout();
            }
            Err(e) => {
                log::error!("Compile error: {}", e);
            }
        }
    }

    fn rebuild_layout(&mut self) {
        let program = match &self.program {
            Some(p) => p,
            None => return,
        };
        let ctx = EvalContext::from_program(program);
        let screen = get_screen(program, self.screen_name.as_deref());
        let root_layout = match (screen, &self.renderer) {
            (Some(screen), Some(renderer)) => {
                let viewport_w = renderer.config.width as f32;
                let viewport_h = renderer.config.height as f32;
                let rect = Rect::new(0.0, 0.0, viewport_w, viewport_h);
                match layout_tree(&ctx, &screen.body, rect) {
                    Ok(layout) => Some(layout),
                    Err(e) => {
                        log::error!("Layout error: {}", e);
                        None
                    }
                }
            }
            _ => None,
        };
        self.root_layout = root_layout;
    }

    fn start_file_watcher(&mut self, path: &PathBuf) {
        use notify::{RecommendedWatcher, RecursiveMode, Watcher, EventKind};
        let (tx, rx) = mpsc::channel();
        self.reload_rx = Some(rx);
        let path = path.clone();
        std::thread::spawn(move || {
            let (notify_tx, notify_rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = match RecommendedWatcher::new(
                move |res| { let _ = notify_tx.send(res); },
                notify::Config::default(),
            ) {
                Ok(w) => w,
                Err(e) => {
                    log::error!("File watcher: {}", e);
                    return;
                }
            };
            if let Err(e) = watcher.watch(path.as_path(), RecursiveMode::NonRecursive) {
                log::error!("Watch {}: {}", path.display(), e);
                return;
            }
            // Block on notify events — watcher stays alive as long as this thread runs
            while let Ok(event) = notify_rx.recv() {
                if let Ok(event) = event {
                    if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                        std::thread::sleep(Duration::from_millis(50));
                        let _ = tx.send(());
                    }
                }
            }
        });
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = winit::window::Window::default_attributes()
            .with_title("Newt — Design Canvas")
            .with_inner_size(winit::dpi::LogicalSize::new(960.0, 640.0));
        let window = match event_loop.create_window(attrs) {
            Ok(w) => w,
            Err(e) => {
                log::error!("Failed to create window: {}", e);
                return;
            }
        };
        let renderer = match pollster::block_on(RendererState::new(&window)) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Failed to create renderer: {}", e);
                return;
            }
        };
        self.window = Some(window);
        self.renderer = Some(renderer);
        self.load_and_layout();
        if let Some(path) = self.source_path.clone() {
            self.start_file_watcher(&path);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(ref rx) = self.reload_rx {
            if rx.try_recv().is_ok() {
                if let Some(ref path) = self.source_path {
                    if let Ok(code) = std::fs::read_to_string(path) {
                        self.source_code = code;
                        self.load_and_layout();
                    }
                }
                if let Some(ref window) = self.window {
                    window.request_redraw();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(ref mut r) = self.renderer {
                    r.resize(size.width, size.height);
                    self.rebuild_layout();
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(renderer), Some(ref root)) = (&mut self.renderer, &self.root_layout) {
                    renderer.draw_layout(root);
                    if let Err(e) = renderer.render() {
                        log::error!("Render error: {:?}", e);
                    }
                }
            }
            _ => {}
        }
    }
}

