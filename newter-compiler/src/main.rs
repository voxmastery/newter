//! Newt design canvas compiler — entry point.
//!
//! Subcommands: run (default), check, build, serve, watch.
//! Usage: newter-compiler [options] [file]
//!        newter-compiler serve [options] <file>
//!        newter-compiler build [options] <file>
//!        newter-compiler check <file>
//!        newter-compiler watch [options] <file>

use clap::Parser;
use newter_compiler::{
    compile, format_error, has_state_vars, layout_to_html, layout_to_reactive_html,
    theme_css_vars, Source, DEFAULT_SERVE_PORT, DEFAULT_VIEWPORT_H, DEFAULT_VIEWPORT_W,
};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(1);
        }
    };

    match cli.command {
        None => run_run(cli.file, cli.screen),
        Some(Command::Serve { file, port, host, screen }) => {
            let path = file.unwrap_or_else(|| {
                eprintln!("error: serve requires <file>");
                eprintln!("usage: newter-compiler serve <file.newt> [--port PORT] [--host HOST] [--screen NAME]");
                std::process::exit(1);
            });
            if !path.exists() {
                eprintln!("error: file not found: {}", path.display());
                return ExitCode::from(1);
            }
            let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
            if let Err(e) = rt.block_on(newter_compiler::serve::serve(path, port, host, screen)) {
                eprintln!("serve error: {}", e);
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Some(Command::Build {
            file,
            html,
            output,
            screen,
        }) => {
            let path = file.unwrap_or_else(|| {
                eprintln!("error: build requires <file>");
                eprintln!("usage: newter-compiler build <file.newt> [--html] [-o OUT] [--screen NAME]");
                std::process::exit(1);
            });
            run_build(path, html, output, screen)
        }
        Some(Command::Check { file }) => {
            let path = file.unwrap_or_else(|| {
                eprintln!("error: check requires <file>");
                eprintln!("usage: newter-compiler check <file.newt>");
                std::process::exit(1);
            });
            run_check(path)
        }
        Some(Command::Watch {
            file,
            html,
            output,
            screen,
        }) => {
            let path = file.unwrap_or_else(|| {
                eprintln!("error: watch requires <file>");
                std::process::exit(1);
            });
            run_watch(path, html, output, screen)
        }
    }
}

fn run_check(path: PathBuf) -> ExitCode {
    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to read file: {}", e);
            return ExitCode::from(1);
        }
    };
    let path_str = path.to_str();
    let source_obj = Source::new(source.clone(), path_str.map(String::from));
    match compile(source.trim(), Some(path.as_path()), None) {
        Ok(_) => {
            println!("check ok");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", format_error(&source_obj, &e));
            ExitCode::from(1)
        }
    }
}

fn run_build(
    path: PathBuf,
    html: bool,
    output: Option<PathBuf>,
    screen: Option<String>,
) -> ExitCode {
    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to read file: {}", e);
            return ExitCode::from(1);
        }
    };
    let source_obj = Source::new(source.clone(), path.to_str().map(String::from));
    let (program, layout) = match compile(source.trim(), Some(path.as_path()), screen.as_deref()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", format_error(&source_obj, &e));
            return ExitCode::from(1);
        }
    };
    if !html {
        eprintln!("build: use --html to output HTML (e.g. newter-compiler build --html out.html file.newt)");
        return ExitCode::SUCCESS;
    }
    let out_path = output.unwrap_or_else(|| PathBuf::from("out.html"));
    let vars = theme_css_vars(&program);
    let vars_ref = if vars.is_empty() {
        None
    } else {
        Some(vars.as_slice())
    };
    let html_out = if has_state_vars(&program) {
        layout_to_reactive_html(
            &program,
            &layout,
            DEFAULT_VIEWPORT_W as u32,
            DEFAULT_VIEWPORT_H as u32,
            vars_ref,
        )
    } else {
        layout_to_html(
            &layout,
            DEFAULT_VIEWPORT_W as u32,
            DEFAULT_VIEWPORT_H as u32,
            vars_ref,
        )
    };
    if let Err(e) = std::fs::write(&out_path, html_out) {
        eprintln!("error: write {}: {}", out_path.display(), e);
        return ExitCode::from(1);
    }
    println!("Wrote {}", out_path.display());
    ExitCode::SUCCESS
}

fn run_watch(
    path: PathBuf,
    html: bool,
    output: Option<PathBuf>,
    screen: Option<String>,
) -> ExitCode {
    use notify_debouncer_full::{new_debouncer, notify::RecursiveMode};
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();
    let mut debouncer = match new_debouncer(Duration::from_millis(300), None, move |_| {
        let _ = tx.send(());
    }) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("watch error: {}", e);
            return ExitCode::from(1);
        }
    };
    if debouncer.watch(path.as_path(), RecursiveMode::NonRecursive).is_err() {
        eprintln!("error: could not watch {}", path.display());
        return ExitCode::from(1);
    }
    println!("Watching {} (Ctrl+C to stop)", path.display());
    loop {
        if rx.recv().is_ok() {
            if run_build(path.clone(), html, output.clone(), screen.clone()) == ExitCode::SUCCESS {
                println!("Build ok");
            }
        }
    }
}

fn run_run(file: Option<PathBuf>, screen: Option<String>) -> ExitCode {
    let (source_path, source_code) = if let Some(path) = file {
        match std::fs::read_to_string(&path) {
            Ok(code) => (Some(path), code),
            Err(e) => {
                eprintln!("error: failed to read file: {}", e);
                return ExitCode::from(1);
            }
        }
    } else {
        let default = r#"
let padding = 24;
let fill = #f0f0f0;

screen Main {
  column { gap: 16, padding: padding } {
    box { fill: #ffffff, radius: 8, padding: 16 } {
      text { content: "Newt Canvas", fontSize: 24 }
    }
    row { gap: 12 } {
      box { fill: #e0e0e0, radius: 4 } { text { content: "One" } }
      box { fill: #e0e0e0, radius: 4 } { text { content: "Two" } }
      box { fill: #e0e0e0, radius: 4 } { text { content: "Three" } }
    }
  }
}
"#;
        (None, default.to_string())
    };

    let source = Source::new(
        source_code.clone(),
        source_path.as_ref().and_then(|p| p.to_str().map(String::from)),
    );

    if let Err(e) = compile(
        &source_code,
        source_path.as_ref().map(|p| p.as_path()),
        screen.as_deref(),
    ) {
        eprintln!("{}", format_error(&source, &e));
        return ExitCode::from(1);
    }

    let mut app = newter_compiler::App::new(source_path, source_code, screen);
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let _ = event_loop.run_app(&mut app);
    ExitCode::SUCCESS
}

#[derive(clap::Parser)]
#[command(name = "newter-compiler")]
#[command(about = "Newt design canvas compiler")]
struct Cli {
    /// .newt file (for run: default demo when omitted)
    file: Option<PathBuf>,
    #[arg(long, global = true)]
    screen: Option<String>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Parse and layout only; exit 0 on success
    Check {
        file: Option<PathBuf>,
    },
    /// Build to HTML
    Build {
        file: Option<PathBuf>,
        #[arg(long)]
        html: bool,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        screen: Option<String>,
    },
    /// Start the Canvas IDE server
    Serve {
        file: Option<PathBuf>,
        #[arg(long, default_value_t = DEFAULT_SERVE_PORT)]
        port: u16,
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        #[arg(long)]
        screen: Option<String>,
    },
    /// Rebuild on file change
    Watch {
        file: Option<PathBuf>,
        #[arg(long)]
        html: bool,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        screen: Option<String>,
    },
}

