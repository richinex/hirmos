# Discovery variable counts

Hirmos previously rejected more than 12 or 32 selected variables in browser discovery. These were application integration limits, not variable-count restrictions inherited from the preserved numerical references.

The CD-NOTS cap entered Hirmos in commit `cacf10c2cf2237dc2870b00e753379cc708fe674`. The preserved causal-ts reference is version 0.26.0, commit `cd20a0d54054f7e852e584c9a0db31191ad56477`. Its `causalts/cdnots/phase3_utils.py` builds graph dimensions from the input dataframe; neither `cdnots_discovery` nor the plus implementation sets a 32-variable ceiling. The local Rust core also accepts variable-sized input. No benchmark justification for choosing 32 was found in the introducing commit.

## What changed

Removed fixed discovery variable-count ceilings from readiness, worker-command parsing, evidence schemas and WASM adapters for:

- CD-NOTS, CD-NOTS+ and GRACE
- PCMCI+, J-PCMCI+, LPCMCI and RPCMCI
- PC-stable and FCI, including the separate 12-variable KCI restriction
- DirectLiNGAM and VAR-LiNGAM
- DYNOTEARS and oCSE
- cMLP and cLSTM

J-PCMCI+'s derived 34-variable result ceiling (32 observed plus two dummy variables) was removed too. Updating only the button would have left commands or results rejected downstream.

The other preserved implementations derive dimensions from input arrays: Tigramite's PCMCI-family entry points, causal-learn's PC/FCI entry points, LiNGAM's fit methods, Neural-GC's input-sized models, causal-ts GRACE, and the existing dynamic-matrix DYNOTEARS/oCSE ports. Their resource requirements still grow with the requested problem.

This change does not alter numerical kernels, defaults, test thresholds, lag limits, training budgets, sample-size checks or missing-data policies. The existing lower bound of two variables, shape checks, finite-value requirements, panel/role validation and cancellation remain. Unchanged limits have not all been established as mathematical oracle requirements; they are separate audit work.

## What this does not promise

No fixed column cap does not mean unlimited browser memory, unlimited runtime, or statistically adequate data. A high-dimensional graph can be unstable with short histories, sparse variables, deterministic derived metrics or differing missingness patterns. Do not split and recombine graphs as though conditioning on different variable sets produced one jointly estimated graph.

## Verification

`tests/discovery-dimensions.spec.ts` checks all discovery dimension schemas beyond their former caps, all worker command types with 54 variables, invalid dimensions/buffers, and actual 54-variable CD-NOTS/CD-NOTS+ runs through rebuilt WASM. It also cancels a larger run after progress and verifies that a fresh worker succeeds.

Native regression command:

```sh
cargo test --manifest-path crates/analysis-wasm/Cargo.toml discovery::tests
```

Browser and type checks:

```sh
npm run check
npm run build:wasm
npx playwright test tests/discovery-dimensions.spec.ts tests/analysis-worker.spec.ts tests/discovery-run-lifecycle.spec.ts --project=chromium
```

The same regression file verifies the import-route controls remain on one row at 320, 360, 393 and 412 pixels. Their labels retain the shared typography and their touch targets retain the component's coarse-pointer sizing.

Both methods also completed through the UI with all 54 varying metrics for Allegro and Maestro. Local study evidence is in `docs/2026-09-17-richdata01/studies/broad-metric-discovery/`. This establishes execution, not causal validity.

The existing stationarity worker smoke test used only a trend and two sinusoids, yielding a rank-deficient Zivot–Andrews auxiliary regression. Its success fixture now includes reproducible seeded noise. The numerical implementation and its rank-deficiency refusal were not changed.
