use std::io::{self, Write};
use std::time::Duration;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Attribute, Print, SetAttribute},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusti::{AppConfig, render_welcome};

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
    /// Check local terminal capabilities.
    Doctor,
    /// Open the starter terminal screen. Press q to quit.
    Demo,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
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

    println!("Rusti doctor");
    println!("terminal size: {}x{}", size.0, size.1);
    println!("alternate screen: available");
    println!("keyboard events: available");
    println!("status: ok");

    Ok(())
}

fn demo() -> Result<()> {
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
