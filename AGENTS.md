# Project guidance

This is a Rust 2024 foundation for a parallel downloader MCP server. The current
scaffold has no dependencies. Keep the package name `mcp-tools` and the
single-crate structure.

## Dependencies

Dependencies are allowed. Prefer a reputable, maintained crate over a custom implementation when it meaningfully improves correctness, security, or maintainability.

Before recommending or adding a dependency, discover the current options from live, relevant sources—not pretrained knowledge alone. For Rust crates, check the registry and crate metadata (for example, with `cargo search` and `cargo info`, or the crates.io listing), then consult the crate’s official documentation. Verify that the candidate is compatible with this project’s Rust edition and toolchain, and check relevant features, maintenance signals, and known security advisories.

Explain the dependency’s purpose, why it fits better than practical alternatives, the version selected, and important trade-offs. Report which sources and checks were used, and what they showed. Phrase security findings narrowly—for example, “no relevant advisories found in the sources checked”—rather than claiming a dependency is vulnerability-free.

Choose a suitable stable version compatible with the project; do not chase updates simply to be on the newest release.

If a check cannot be completed, disclose that limitation; don’t present it as verified or replace a justified dependency with a fragile custom implementation just to avoid it.

Don’t treat the absence of prior authorization as a blanket prohibition on proposing or using a justified dependency.

## Architecture boundaries

- `src/lib.rs` exposes the downloader module and its shared types.
- `src/downloader.rs` holds download requests, byte-range segments, and progress.
  Byte ranges are start-inclusive and end-exclusive.
- `src/main.rs` currently reports scaffold status; it is not an MCP server yet.
- Future MCP integration must stay thin: translate tool requests into downloader
  operations and return results. The downloader core owns connection handling,
  scheduling, parallel workers, per-segment retries, and resume validation.
- Keep connection handling inside the Rust core initially. Choose resume storage
  and validation when implementing resume support.
- Introduce subsystem modules when they contain working behavior. Do not create
  empty stubs or panic-based unfinished implementations.

MCP protocol stdout must contain no diagnostic output. Send status messages and
diagnostics to stderr; reserve stdout for protocol messages once MCP is added.

## Development workflow

Before editing, run `git status --short` and inspect `git diff` and `git diff --cached`. Preserve changes that were already present.

Make changes in small, related steps. After each step, inspect `git diff` for the affected files; use the diff—not memory or a summary—as the record of what changed.

If a change causes a problem, inspect the diff and correct only the faulty changes. Use `git restore -- <file>` only when you have checked that **all** uncommitted changes to that file can be discarded. For a tracked file with both wanted and unwanted changes, restore only selected hunks with `git restore -p -- <file>` or edit the file directly. Never restore the whole working tree to fix one file.

Never use broad restore, reset, or clean commands.

Don’t make checkpoint commits unless the user has authorized commits. If a change cannot be safely separated from pre-existing work, stop and report the conflict rather than guessing.

After each task, run and report `git diff --stat` so the change set can be checked
against the claimed changes. Distinguish existing changes from the current task.

If a read or command fails, report the exact error and stop that step. Do not
infer file contents or claim a file was read unless the read returned its contents.

Before editing a file, read its current contents in this session. Prefer the
built-in file read tool. If that tool fails or is unavailable, you may read the
file with a shell command such as `cat` or `sed -n`, but state that you did so and
why. Never say a file was read unless a read succeeded, and never describe
contents you did not see. If both methods fail, stop, report the exact error,
and make no edits to that file. If an edit fails because the file changed, read
it again before retrying. Use shell commands for builds, tests, git, and hashes.

Do not use Python for project tooling, checks, or verification. Use `cargo`, `git`, and `shasum -a 256` (shipped with macOS). Do not depend on whichever `python3` is first on PATH.

Use the installed Rust toolchain with edition 2024 support. Local checks are
offline and require the rustfmt and Clippy components to be installed:

```sh
cargo fmt --check
cargo clippy --offline --all-targets -- -D warnings
cargo test --offline
cargo run --offline --quiet
```

Keep changes focused. Add tests for real behavior when behavior is introduced;
do not add placeholder tests or tests that merely assert the scaffold exists.
Keep `Cargo.lock` in version control and build artifacts under ignored `target/`.
Preserve the supplied architecture document as reference material.
