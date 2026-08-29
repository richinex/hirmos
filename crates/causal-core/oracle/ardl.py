# Golden fixtures for the ARDL bounds test step of 805: ardl_select_order, UECM, the
# cointegrating vector and the PSS bounds test.
# Run: uv run python oracle/ardl.py
import json

import numpy as np
import pandas as pd
from statsmodels.tsa.ardl import ardl_select_order, UECM

rng = np.random.default_rng(7)
n = 160

# A cointegrated pair: x is a random walk, y tracks a long-run multiple of it and corrects
# back towards equilibrium.
x = np.cumsum(rng.normal(scale=0.7, size=n))
y = np.zeros(n)
for t in range(1, n):
    gap = y[t - 1] - (1.5 + 0.8 * x[t - 1] + 0.03 * t)
    y[t] = y[t - 1] - 0.35 * gap + 0.5 * (x[t] - x[t - 1]) + rng.normal(scale=0.4)

df = pd.DataFrame({"outcome": y, "treatment": x})
y_lr = df["outcome"]
x_lr = df[["treatment"]]
maxlag = 4

sel = ardl_select_order(y_lr, maxlag, x_lr, maxlag, ic="aic", trend="ct")
p_ar = max(1, max(sel.model.ar_lags or [1]))
_dl = [max(v) for v in (sel.model.dl_lags or {}).values() if len(v)]
q_dl = max(1, (max(_dl) if _dl else 1))

# The whole AIC grid, so every candidate can be checked and not just the winner. The series
# is indexed by the criterion value and holds the order as its value.
grid = []
for aic, order in sel.aic.items():
    ar, exog = order[0], order[1]
    dl = exog["treatment"] if isinstance(exog, dict) else exog[1]
    grid.append({"ar": 0 if not ar else int(ar),
                 "dl": None if dl is None else int(dl),
                 "aic": float(aic)})

res = UECM(y_lr, p_ar, x_lr, q_dl, trend="ct").fit()
ci = res.ci_conf_int()
bt = res.bounds_test(case=4)

out = {
    "y": y.tolist(), "x": x.tolist(), "maxlag": maxlag,
    "selected": {"p": int(p_ar), "q": int(q_dl)},
    "aic_grid": grid,
    "uecm": {
        "exog_names": list(res.model.exog_names),
        "params": np.asarray(res.params).tolist(),
        "bse": np.asarray(res.bse).tolist(),
        "cov_params": np.asarray(res.cov_params()).tolist(),
        "nobs": int(res.nobs),
        "df_resid": float(res.df_resid),
        "resid": np.asarray(res.resid).tolist(),
    },
    "cointegrating_vector": {
        "index": [str(v) for v in res.ci_params.index],
        "params": np.asarray(res.ci_params).tolist(),
        "bse": np.asarray(res.ci_bse).tolist(),
        "tvalues": [None if np.isnan(v) else float(v) for v in np.asarray(res.ci_tvalues)],
        "pvalues": [None if np.isnan(v) else float(v) for v in np.asarray(res.ci_pvalues)],
        "conf_int_lower": np.asarray(ci["lower"]).tolist(),
        "conf_int_upper": np.asarray(ci["upper"]).tolist(),
    },
    "bounds_test": {
        "case": 4,
        "stat": float(bt.stat),
        "p_lower": float(bt.p_values["lower"]),
        "p_upper": float(bt.p_values["upper"]),
        "crit_lower": np.asarray(bt.crit_vals["lower"]).tolist(),
        "crit_upper": np.asarray(bt.crit_vals["upper"]).tolist(),
    },
}

# The long-run read as 805 forms it: the cointegrating vector is normalised on the outcome,
# so the effect is the negated coefficient and the interval negates and swaps.
key = "treatment"
out["long_run"] = {
    "beta": -float(res.ci_params[key]),
    "p": float(res.ci_pvalues[key]),
    "ci_lo": -float(ci.loc[key].iloc[1]),
    "ci_hi": -float(ci.loc[key].iloc[0]),
}

# A second configuration, so the trend and order handling are pinned at more than one point.
res_c = UECM(y_lr, 2, x_lr, 3, trend="c").fit()
bt_c = res_c.bounds_test(case=3)
out["trend_c"] = {
    "p": 2, "q": 3, "case": 3,
    "exog_names": list(res_c.model.exog_names),
    "params": np.asarray(res_c.params).tolist(),
    "ci_params": np.asarray(res_c.ci_params).tolist(),
    "ci_bse": np.asarray(res_c.ci_bse).tolist(),
    "stat": float(bt_c.stat),
    "p_lower": float(bt_c.p_values["lower"]),
    "p_upper": float(bt_c.p_values["upper"]),
}

with open("oracle/fixtures/ardl.json", "w") as fh:
    json.dump(out, fh)

print(f"ARDL order (AIC): p={p_ar}, q={q_dl}, grid size {len(grid)}")
print("UECM regressors:", list(res.model.exog_names))
print(f"long-run beta={out['long_run']['beta']:+.4f} p={out['long_run']['p']:.4g} "
      f"CI [{out['long_run']['ci_lo']:+.4f}, {out['long_run']['ci_hi']:+.4f}]")
print(f"bounds F (case 4): stat={bt.stat:.4f} p_lower={out['bounds_test']['p_lower']:.4g} "
      f"p_upper={out['bounds_test']['p_upper']:.4g}")
print(f"trend c (case 3): stat={bt_c.stat:.4f}")
