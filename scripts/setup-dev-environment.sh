#!/bin/sh

# Setup development environment for the project
main() {
  need_cmd curl
  curl_install "rustup" "https://sh.rustup.rs"
  curl_install "cargo-binstall" "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh"

  cargo_install "cargo-nextest" "cargo-nextest"
  cargo_install "just" "just"
  cargo_install "mdbook" "mdbook"
  cargo_install "mdbook-anchors-aweigh" "mdbook-anchors-aweigh"
}

#-----------------------------------------------------------
#--------------------- Helper functions --------------------
#-----------------------------------------------------------

ensure() {
  if ! "$@"; then
    echo "❌ Command failed: $*"
    exit 1
  fi
}

check_cmd() {
  command -v "$1" >/dev/null 2>&1
}

need_cmd() {
  if ! check_cmd "$1"; then
    echo "❌ Required command not found: $1"
    exit 1
  fi
}

confirm() {
  read -p "⁉️ [y/N] " response
  case "$response" in
  [yY][eE][sS] | [yY])
    return 0
    ;;
  *)
    echo "❌ Operation cancelled."
    exit 1
    ;;
  esac
}

cargo_install() {
  local cmd="$1"
  local package="$2"

  if check_cmd "$cmd"; then
    echo "✅ $cmd found."
    return
  fi

  echo "⁉️ $1 is required but not found."
  echo "⁉️ Install From:"
  echo "⁉️   cargo binstall $package"
  confirm
  echo "📥 Installing $package..."
  ensure cargo binstall "$package" -y
}

curl_install() {
  local cmd="$1"
  local url="$2"

  if check_cmd "$cmd"; then
    echo "✅ $cmd found."
    return
  fi

  echo "⁉️ $1 is required but not found."
  echo "⁉️ Install From:"
  echo "⁉️   $url"
  confirm
  echo "📥 Installing from $url..."
  ensure curl --proto '=https' --tlsv1.2 -LsSf $url | sh
}

main
