test *PATTERN:
  cargo nextest run --workspace {{PATTERN}}
  cargo test --doc
