#!/usr/bin/env bash
set -euo pipefail

REPO_URL="https://github.com/codesk90/Rusti"
BIN_NAME="rusti"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    return 1
  fi
}

main() {
  echo "Installing Rusti..."

  if ! command -v cargo >/dev/null 2>&1; then
    cat >&2 <<'MSG'
error: Rust/Cargo is required to install Rusti.
Install Rust first, then retry:
  https://rustup.rs/
MSG
    exit 1
  fi

  need git

  cargo install --git "$REPO_URL" --locked --force

  if command -v "$BIN_NAME" >/dev/null 2>&1; then
    echo "Rusti installed successfully: $(command -v "$BIN_NAME")"
    "$BIN_NAME" --version
  else
    cat >&2 <<'MSG'
Rusti was built, but the binary is not on PATH.
Cargo usually installs to ~/.cargo/bin.
Add this to your shell profile:
  export PATH="$HOME/.cargo/bin:$PATH"
MSG
    exit 1
  fi
}

main "$@"
