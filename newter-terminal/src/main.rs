//! Frozen ice-glass native terminal for Newter.

mod ui;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::process::Command;
use std::time::Duration;

struct App {
    input: String,
    output: Vec<String>,
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            output: vec![
                "newter terminal (voiceos-inspired frozen nav)".to_string(),
                "type a command and press Enter. 'run <file>' runs the compiler.".to_string(),
                "'.quit' or Ctrl+C to exit.".to_string(),
                "".to_string(),
            ],
            exit: false,
        }
    }

    fn submit(&mut self) {
        let line = self.input.trim();
        if line.is_empty() {
            return;
        }
        self.output.push(format!("> {}", line));
        if line == ".quit" || line == "quit" || line == "exit" {
            self.exit = true;
            return;
        }
        if line.starts_with("run ") {
            let file = line.strip_prefix("run ").unwrap_or("").trim();
            self.output.push(format!("  [run] newter-compiler {}", file));
            match Command::new("cargo")
                .args(["run", "-p", "newter-compiler", "--release", "--", file])
                .spawn()
            {
                Ok(child) => self.output.push(format!("  started compiler (PID {})", child.id())),
                Err(e) => self.output.push(format!("  error: {}", e)),
            }
        } else {
            self.output.push(format!("  unknown: {}", line));
        }
        self.output.push("".to_string());
        self.input.clear();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                        app.exit = true;
                    }
                    KeyCode::Char('q') => app.exit = true,
                    KeyCode::Enter => app.submit(),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                }
            }
        }

        if app.exit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
