## Summary

<!-- Explain the behavior or documentation change. -->

## Validation

<!-- List the commands run and their results. -->

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --offline --all-targets -- -D warnings`
- [ ] `cargo test --offline`
- [ ] `cargo run --offline --quiet`

## Downloader considerations

<!-- Complete when applicable. -->

- [ ] Byte ranges retain `[start, end)` semantics.
- [ ] Diagnostics do not write to stdout.
- [ ] New behavior has focused tests.
- [ ] Dependencies were evaluated before being added.
