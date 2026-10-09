# Hirmos

## Source and licences

Copyright (C) 2026 Richard Chukwu.

Hirmos is free software under [GPL-3.0-or-later](LICENSE). You may use, modify
and redistribute it under those terms. It is provided without warranty.
Third-party code retains its applicable licences and attribution: see
[NOTICE](NOTICE), [licence texts](THIRD_PARTY_LICENSES), and the
[source and release checklist](licenses/README.md).

The app's GitHub link points to the build's commit. A release must publish that
commit and all corresponding source, including the Rust kernels, patches and
build scripts. The numerical implementation is not just the frontend source.


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
