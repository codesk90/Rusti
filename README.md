# Rusti

Rusti is an open-source Rust terminal workbench for AI coding agents.

It now includes a real terminal mode: `rusti terminal` opens your local shell inside a pseudo-terminal. On macOS it uses your built-in shell from `$SHELL`, falling back to `/bin/zsh`.

## Install

From GitHub with curl:

```bash
curl -fsSL https://raw.githubusercontent.com/codesk90/Rusti/main/install.sh | bash
```

This downloads the latest prebuilt release binary for your OS/architecture and installs it to `~/.local/bin` by default.

Override install location:

```bash
RUSTI_INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/codesk90/Rusti/main/install.sh | bash
```

Install a specific version:

```bash
RUSTI_VERSION=v0.2.0 curl -fsSL https://raw.githubusercontent.com/codesk90/Rusti/main/install.sh | bash
```

Or from a local checkout:

```bash
cargo install --path .
```

## Run

```bash
cargo run -- terminal
cargo run -- doctor
cargo run -- demo
```

`rusti terminal` launches a real interactive shell session. Type `exit` or press Ctrl-D to quit.

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
- DMG packaging
- Intel macOS release asset

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
