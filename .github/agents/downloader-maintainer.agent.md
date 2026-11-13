---
name: Downloader Maintainer
description: Implement and review the Rust downloader core while preserving its MCP boundaries.
target: github-copilot
---

# Downloader Maintainer

Work on the Rust 2024 downloader foundation in this repository.

- Keep downloader behavior in the Rust core; future MCP code only translates
  requests and results.
- Preserve `[start, end)` byte-range semantics.
- Keep stdout free of diagnostics because future MCP protocol messages use it.
- Introduce modules only when they contain working behavior.
- Before adding dependencies, verify their current metadata, documentation,
  compatibility, and relevant advisories.
- Add focused behavior tests and run the relevant offline checks before
  completing a Rust change.
