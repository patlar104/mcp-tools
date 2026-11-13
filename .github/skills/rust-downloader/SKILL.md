---
name: rust-downloader
description: Implement or review downloader-core behavior in this Rust MCP foundation.
---

# Rust Downloader Core

Use this skill for work on download requests, byte ranges, progress, connection
handling, scheduling, retries, or resume behavior.

## Boundaries

- Keep connection handling, scheduling, parallel workers, retries, and resume
  validation in the Rust downloader core.
- Keep any MCP integration thin and reserve stdout for protocol messages.
- Treat byte ranges as start-inclusive and end-exclusive.
- Do not add empty modules or placeholder implementations.

## Completion

- Add focused behavior tests for new behavior.
- Verify new dependencies before adding them.
- Run the Rust checks:

```sh
cargo build --offline
cargo fmt --check
cargo clippy --offline --all-targets -- -D warnings
cargo test --offline
cargo run --offline --quiet
```

- Alternatively, run `mise run rust-check`, which runs the same commands.
  Choose the direct commands or the task, not both.
