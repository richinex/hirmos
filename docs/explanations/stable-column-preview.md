# Stable column previews and compact table typography

Implemented 17 September 2026. This is a presentation/lifecycle change, not a numerical optimisation or a change to worker calculations.

## Why the preview flickered

Selecting a column changes the selected column immediately. Its worker-generated profile and numeric series arrive later. Previously, both preview panes replaced their content with a skeleton during that interval. The bottom pane could also briefly describe a numeric column as nonnumeric because the selected column and the previous response did not match.

Removing `EChart` from the React tree disposes its ECharts instance. Returning the result mounts a fresh chart. Even cached selections could pass through the mismatched state because the hook publishes the cached response in an effect.

## The fix

`ColumnSeriesPane` and `ColumnProfilePane` retain their last successfully displayed data together with its column identity. While the next request is pending, they keep that labelled content mounted and mark the surface `aria-busy`. They accept a ready response only when its column matches the current selection.

When the matching response arrives, the heading, figures and chart update together. A guarded state adjustment during render updates this presentation snapshot before React commits the children; an effect would introduce an extra committed frame. The snapshot retains references, not copies of the numeric arrays.

The existing `EChart` component already updates its live instance using `setOption`. No timeout, fade-out, global animation change or new chart renderer was needed. Numeric-to-numeric changes therefore preserve both the chart host and its SVG.

Both panes are keyed by **dataset profile identity**, not column identity. A different profile resets retained presentation data. Do not use a column key: that would reintroduce the remount on every selection.

The bottom plot associates its zoom window with the displayed series. A new series resets the controlled chart window and summary together. The old series keeps its existing zoom while the replacement loads.

Failures still display an error. Nonnumeric columns intentionally have no row-order line plot. Numeric-to-categorical changes in the inspector intentionally replace the histogram with the category summary. Keeping the old content during loading does not mean hiding failures or labelling old values as new values.

## Apply this pattern elsewhere

1. Inspect the loading branches and React keys before changing chart animation.
2. Check whether a selection first renders with the previous asynchronous response.
3. Keep the last successful **identity and data together** during loading.
4. Reject stale responses; reset retained content at the owning dataset/project boundary.
5. Update a mounted chart through its existing API.
6. Keep chart interaction state and derived summaries synchronised.
7. Test initial loading, cached switches, rapid switches, failures, different data types and different sources.

This is local presentation state, not another persisted copy of the dataset or a replacement for the workflow store.

## Verification

`tests/column-series-stability.spec.ts` uploads a CSV through the UI and changes preview headers. It checks that both the bottom chart and the inspector histogram retain the same DOM hosts and SVG elements, while their accessible labels and numerical summaries change.

A controlled browser harness covers delayed loading, mismatched replies, cached switches, errors, nonnumeric columns and source resets on desktop and mobile. It also checks stable plot geometry. The real preview-click test is desktop-only because the mobile sheet overlays the table.

Run:

```sh
npm run check
npx playwright test tests/column-series-stability.spec.ts tests/table-controls.spec.ts --reporter=line
```

Screenshots are written into the relevant Playwright `test-results` directories. Element identity is the regression assertion; screenshots alone cannot prove that a chart was not recreated.

## Typography

The preview table inherited the dashboard's 14px body size; the earlier 13px reduction was scoped to bottom-panel content. Shared tables now use `text-table`, backed by `--text-table: .8125rem` (13px at a 16px root). Schema column names use the same token. Headers retain `text-label`; controls and explanatory prose are not indiscriminately shrunk. Rem units preserve browser font-size preferences.

The follow-up source audit found 11 tables bypassing the shared recipe in `EstimationPanel`, `SensitivityPanel`, `CounterfactualPanel` and `EstimateHeadline`. Their explicit body-text classes now use the table token too. Shared evidence, time-series, preprocessing and history tables inherit the recipe. The compact DAG ledger intentionally remains label-sized. Empty-state explanations remain body text; inspector headings retain their separate hierarchy. This source audit does not claim that every estimator screen has been visually replayed.

## Pipeline previews and Script cards (18 September 2026)

The pipeline preview now retains the last successful table together with its block identity, title and run outcome. Pending requests cannot relabel an old table as a different block. Late responses are ignored. Waiting or failed blocks hide the retained table instead of destroying it; recovery updates the same table instance. Detailed errors appear once in the inspector, while the preview says no preview is available.

Pasting code exposed a separate geometry problem: replacing status text with the running orb increased the node height by about 14.5px. Pipeline cards now use the existing 86px card-height constant, with fixed header and status rows and a fixed-size orb slot. This fixes the changing geometry without delaying execution or suppressing updates.

`tests/pipeline-preview-stability.spec.ts` exercises a real Script block through execution, failure and recovery. It checks retained table and node DOM identity, one detailed error, stable preview-pane height, and less than 0.5px node-height variation during a rerun.

## References

- [React: preserving and resetting state](https://react.dev/learn/preserving-and-resetting-state)
- [React: adjusting state when a prop changes](https://react.dev/learn/you-might-not-need-an-effect#adjusting-some-state-when-a-prop-changes)
- [ECharts: dynamic data and setOption](https://echarts.apache.org/handbook/en/how-to/data/dynamic-data/)
- Local guidance used: `.claude/skills/react-best-practices`.
