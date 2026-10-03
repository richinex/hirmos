# Hirmos

Hirmos is a causal inference workbench that runs in a web browser. You prepare the observations for analysis and draw a graph describing how you believe the variables affect one another. Before estimation, Hirmos checks whether the effect you want to study is identifiable under that graph's assumptions, recording any variables you need to adjust for.

The current architecture slice provides the independent application shell, canonical DuckDB-Wasm
CSV/TSV/Parquet profiling, offline ICU timezone handling, validity-preserving numeric buffers, and
an owned Rust causal core behind one typed Hirmos analysis-Wasm façade. Later workflow chapters
remain unavailable until their artifact preconditions exist.

```sh
npm install
npm run dev
```

Hirmos includes methods ported to Rust from Tigramite, statsmodels and DoWhy, compiled to WebAssembly for use in the browser and tested against outputs from the original implementations. Hirmos supports panel data and independent observations alongside time series, with additional methods for survival analysis and for attributing unusual observations or distribution changes to variables in a causal model.

To inspect an example, open [Hirmos](https://hirmos-app.pages.dev) and select say 'Seat-belt law and road deaths' on the Projects page and start exploring.