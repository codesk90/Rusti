#!/usr/bin/env bash
set -euo pipefail

REPO="codesk90/Rusti"
BIN_NAME="rusti"
INSTALL_DIR="${RUSTI_INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${RUSTI_VERSION:-latest}"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    exit 1
  fi
}

platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Darwin) os="apple-darwin" ;;
    Linux) os="unknown-linux-gnu" ;;
    *) echo "error: unsupported OS: $os" >&2; exit 1 ;;
  esac

  case "$arch" in
    arm64|aarch64) arch="aarch64" ;;
    x86_64|amd64) arch="x86_64" ;;
    *) echo "error: unsupported architecture: $arch" >&2; exit 1 ;;
  esac

  echo "${arch}-${os}"
}

release_url() {
  local target tag asset
  target="$1"
  tag="$VERSION"
  asset="rusti-${target}.tar.gz"

  if [ "$tag" = "latest" ]; then
    echo "https://github.com/${REPO}/releases/latest/download/${asset}"
  else
    echo "https://github.com/${REPO}/releases/download/${tag}/${asset}"
  fi
}

main() {
  need curl
  need tar
  need install

  local target url tmp
  target="$(platform)"
  url="$(release_url "$target")"
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  echo "Installing Rusti ${VERSION} for ${target}..."
  echo "Downloading ${url}"

  curl -fL "$url" -o "$tmp/rusti.tar.gz"
  tar -xzf "$tmp/rusti.tar.gz" -C "$tmp"

  mkdir -p "$INSTALL_DIR"
  install -m 0755 "$tmp/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"

  echo "Rusti installed: $INSTALL_DIR/$BIN_NAME"
  "$INSTALL_DIR/$BIN_NAME" --version

  case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
      echo ""
      echo "Note: $INSTALL_DIR is not on PATH. Add this to your shell profile:"
      echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
      ;;
  esac
}

main "$@"
