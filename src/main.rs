use std::io::{self, IsTerminal, Read, Write};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Attribute, Print, SetAttribute},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use rusti::{AppConfig, TerminalConfig, render_welcome};

#[derive(Debug, Parser)]
#[command(name = "rusti")]
#[command(about = "A Rust terminal workbench for AI coding agents")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Open a real terminal session backed by your local shell.
    #[command(alias = "term")]
    Terminal,
    /// Check local terminal capabilities.
    Doctor,
    /// Open the starter terminal screen. Press q to quit.
    Demo,
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> Result<Self> {
        terminal::enable_raw_mode().context("failed to enable raw terminal mode")?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Terminal) => terminal_session(),
        Some(Command::Doctor) => doctor(),
        Some(Command::Demo) => demo(),
        None => {
            print!("{}", render_welcome(&AppConfig::default()));
            Ok(())
        }
    }
}

fn doctor() -> Result<()> {
    let size = terminal::size().unwrap_or((0, 0));
    let config = TerminalConfig::from_shell_env(std::env::var("SHELL").ok().as_deref());

    println!("Rusti doctor");
    println!("terminal size: {}x{}", size.0, size.1);
    println!("shell: {}", config.shell);
    println!("pty backend: available");
    println!("alternate screen: available");
    println!("keyboard events: available");
    println!("status: ok");

    Ok(())
}

fn terminal_session() -> Result<()> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        println!(
            "Rusti terminal requires an interactive terminal. Try running `rusti terminal` directly in your shell."
        );
        return Ok(());
    }

    let config = TerminalConfig::from_shell_env(std::env::var("SHELL").ok().as_deref());
    let (cols, rows) = terminal::size().unwrap_or((80, 24));
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("failed to open pseudo-terminal")?;

    let mut command = CommandBuilder::new(&config.shell);
    if let Ok(current_dir) = std::env::current_dir() {
        command.cwd(current_dir);
    }
    command.env(
        "TERM",
        std::env::var("TERM").unwrap_or_else(|_| "xterm-256color".to_string()),
    );
    command.env("RUSTI", "1");
    command.env("RUSTI_SHELL", config.shell_name());

    let mut child = pair
        .slave
        .spawn_command(command)
        .with_context(|| format!("failed to start shell `{}`", config.shell))?;
    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .context("failed to clone pty reader")?;
    let mut writer = pair
        .master
        .take_writer()
        .context("failed to open pty writer")?;

    let _raw_mode = RawModeGuard::new()?;
    eprintln!(
        "Rusti terminal: {}. Type `exit` or press Ctrl-D to quit.",
        config.shell
    );

    let output_thread = thread::spawn(move || -> io::Result<()> {
        let mut stdout = io::stdout();
        io::copy(&mut reader, &mut stdout)?;
        stdout.flush()
    });

    let input_thread = thread::spawn(move || -> io::Result<()> {
        let mut stdin = io::stdin();
        let mut buffer = [0_u8; 1024];

        loop {
            let bytes_read = stdin.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            writer.write_all(&buffer[..bytes_read])?;
            writer.flush()?;
        }

        Ok(())
    });

    let status = child.wait().context("terminal child process failed")?;
    drop(pair.master);

    if let Err(error) = output_thread.join().unwrap_or(Ok(())) {
        eprintln!("Rusti output stream ended: {error}");
    }
    drop(input_thread);

    if !status.success() {
        eprintln!("Rusti terminal exited with status: {status:?}");
    }

    Ok(())
}

fn demo() -> Result<()> {
    if !io::stdout().is_terminal() {
        println!(
            "Rusti demo requires an interactive terminal. Try running `rusti demo` directly in your shell."
        );
        return Ok(());
    }

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let result = run_demo_loop(&mut stdout);

    execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}

fn run_demo_loop(stdout: &mut impl Write) -> Result<()> {
    loop {
        let (cols, rows) = terminal::size()?;
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(2, 1),
            SetAttribute(Attribute::Bold),
            Print("Rusti"),
            SetAttribute(Attribute::Reset),
            cursor::MoveTo(2, 3),
            Print("Rust terminal workbench for AI coding agents"),
            cursor::MoveTo(2, 5),
            Print(format!("viewport: {}x{}", cols, rows)),
            cursor::MoveTo(2, 7),
            Print("Next: panes, agent sessions, logs, and command review."),
            cursor::MoveTo(2, rows.saturating_sub(2)),
            Print("Press q to quit")
        )?;
        stdout.flush()?;

        if !event::poll(Duration::from_millis(250))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
            break;
        }
    }

    Ok(())
}
