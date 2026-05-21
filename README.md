# Rusti

Rusti is an open-source Rust terminal workbench for AI coding agents.

The first goal is deliberately small: a durable CLI/TUI foundation that can grow into panes, agent sessions, logs, approvals, and command review without becoming hard to maintain.

## Install

From GitHub with curl:

```bash
curl -fsSL https://raw.githubusercontent.com/codesk90/Rusti/main/install.sh | bash
```

Or from a local checkout:

```bash
cargo install --path .
```

## Run

```bash
cargo run
cargo run -- doctor
cargo run -- demo
```

In the demo screen, press `q` or `Esc` to quit.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Roadmap

- Pane layout
- Agent session launcher
- Command/log stream viewer
- Human approval queue
- Config file support
- Cross-platform packaging

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
