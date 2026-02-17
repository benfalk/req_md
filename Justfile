# lists all recipes
default:
  @just --list

# launch interactive menu
menu:
  @just --choose

# Runs dev CLI, rebuilding when needed
[group('dev')]
dev-cli COMMAND *ARGS:
  @cargo run -q --package reqmd_cli -- {{COMMAND}} {{ARGS}}

# Runs dev TUI, rebuilding when needed
[group('dev')]
dev-tui *ARGS:
  @cargo run -q --package reqmd_tui -- {{ARGS}}

# Builds all crates in the workspace
[group('build')]
build:
  cargo build --workspace

# Builds and opens rust crate documentation
[group('build')]
build-rust-docs:
  cargo doc --workspace --no-deps --open

# Builds and installs the CLI crate
[group('build')]
build-install-cli:
  @just _crate_install reqmd_cli

# Builds and installs experimental TUI crate
[group('build')]
build-install-tui:
  @just _crate_install reqmd_tui

# runs all quick tests or specific functions
[group('test')]
test *TEST_CASES:
  @{{if TEST_CASES != "" { "just _run test-funcs " + TEST_CASES } \
     else { "just _run test-short" } \
  }}

# runs code comment examples as tests
[group('test')]
test-docs:
  cargo test --doc

# test code comments generate documentation
[group('test')]
test-doc-gen:
  RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# run all or specific tests functions
[group('test')]
test-funcs *TEST_CASES:
  cargo nextest run --workspace {{TEST_CASES}}

# checks crates are ready for crates.io
[group('test')]
test-publish:
  cargo publish --dry-run --workspace

# ensures code is formatted correctly
[group('test')]
test-format:
  cargo fmt --all --check

# runs quick tests for development
[group('test')]
test-short:
  @just _run test-funcs
  @just _run test-docs
  @just _run test-format

# runs all tests, including publish checks
[group('test')]
test-all:
  @just _run test-funcs
  @just _run test-docs
  @just _run test-format
  @just _run test-doc-gen
  @just _run test-publish

# runs a recipe with a bit of flare
_run recipe *ARGS:
  @echo "⚙️{{BLUE + BOLD}} {{recipe}} {{YELLOW}}{{ARGS}}{{NORMAL}}"
  @just {{recipe}} {{ARGS}}

# installs a crate binary
[no-cd]
_crate_install crate:
  cargo install --path=./crates/{{crate}}
