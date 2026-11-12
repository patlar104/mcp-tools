# Rust Parallel Downloader MCP: Core Architecture

This is the groundwork sketch for a Rust MCP server that owns its connection logic: it splits a file into byte-range segments, fetches them in parallel, retries each segment on its own, and can resume after a drop.

## The core, layer by layer

The MCP layer stays thin and only exposes tools. The downloader core does the real work: the scheduler hands segments to a worker pool, each worker retries its own segment, and resume state is what lets a dropped connection continue instead of restarting.

&#91;embedded content: downloader architecture · 3 layers, 5 core parts\]

## Open decisions

- Whether the stable-connection layer stays inside the Rust core or later becomes its own process.
- How resume state is stored and validated.
