# Copilot Instructions

This repository is a Rust 2024, single-crate foundation for a parallel
downloader MCP server. Keep the package name `mcp-tools`.

## Architecture

- Keep the downloader core in Rust. It owns connection handling, scheduling,
  parallel workers, per-segment retries, and resume validation.
- Keep any future MCP layer thin: translate tool requests to downloader
  operations and return their results.
- Keep shared downloader types in `src/downloader.rs` and re-export them from
  `src/lib.rs`.
- Byte ranges are start-inclusive and end-exclusive.
- Add modules only with working behavior; do not add empty stubs or
  panic-based placeholders.

## Output and quality

- Reserve stdout for MCP protocol messages. Send diagnostics and status to
  stderr.
- Preserve existing behavior and add focused tests when introducing behavior.
- Prefer maintained crates when they meaningfully improve correctness, but
  verify the current version, compatibility, documentation, and relevant
  advisories before adding one.
- Treat lower-bound version constraints as compatibility floors, not selected
  or exclusive versions; state the separate rationale for any chosen pin.
- Keep `Cargo.lock` committed and build artifacts under ignored `target/`.

## Checks

For Rust changes, run:

```sh
cargo build --offline
cargo fmt --check
cargo clippy --offline --all-targets -- -D warnings
cargo test --offline
cargo run --offline --quiet
```

Alternatively, run `mise run rust-check`, which runs the same commands. For
Markdown or Mermaid changes, use the direct Mermaid commands from the README
or `mise run docs-check`, not both. `mise run check` runs both task groups.
