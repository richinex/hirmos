# Hirmos

Hirmos is a browser-first causal inference workbench. The product follows this repository's
authoritative `DESIGN.md`.

The current architecture slice provides the independent application shell, canonical DuckDB-Wasm
CSV/TSV/Parquet profiling, offline ICU timezone handling, validity-preserving numeric buffers, and
an owned Rust causal core behind one typed Hirmos analysis-Wasm façade. Later workflow chapters
remain unavailable until their artifact preconditions exist.

```sh
npm install
npm run dev
```

The scientific interface presents method and model output as recommendations and evidence for
judgment, never as authoritative conclusions.
