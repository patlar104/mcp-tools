---
name: project-documentation
description: Maintain project Markdown, Mermaid diagrams, and documentation sources.
---

# Project Documentation

Use this skill when creating or updating project documentation, including
Markdown, embedded Mermaid diagrams, and standalone Mermaid source files.

## Documentation layout

- Keep repository-level guidance and project overviews in the existing root
  Markdown files.
- Add `docs/` only for substantive, maintained documentation; do not create it
  for placeholders.
- Store standalone Mermaid sources as `.mmd` files next to the document that
  uses them or under `docs/diagrams/` when diagrams become reusable assets.

## Mermaid conventions

- Use fenced `mermaid` blocks for diagrams embedded in Markdown.
- Keep diagrams aligned with the implemented behavior; label planned components
  as planned.
- Prefer simple, portable Mermaid syntax and concise node labels.
- Update accompanying prose whenever a diagram changes meaning.

## Documentation quality

- Preserve byte-range `[start, end)` semantics and the MCP stdout constraint.
- Keep examples consistent with the current public API and commands.
- Do not add images or generated diagram outputs when the Mermaid source is
  sufficient.
- After changing Markdown or Mermaid content, run:

  ```sh
  mise exec -- npm ci
  mise exec -- npm run docs:install-browser
  mise exec -- npm run docs:check
  ```

- Alternatively, run `mise run docs-check`, which runs the same commands.
  Choose the direct commands or the task, not both.
