# Golden fixtures for HAC OLS, WLS, Durbin-Watson. Run: uv run python oracle/estimation_fixtures.py
import json

import numpy as np
import statsmodels.api as sm

fixtures = {}
rng = np.random.default_rng(107)
n = 150
x = rng.normal(0, 1, (n, 3))
trend = np.cumsum(rng.normal(0, 0.3, n))
y = 2.0 + 0.8 * x[:, 0] - 0.5 * x[:, 1] + 0.0 * x[:, 2] + 0.4 * trend + rng.normal(0, 1, n)
X = sm.add_constant(x)

cases = []
for maxlags in (8, 16):
    r = sm.OLS(y, X).fit(cov_type="HAC", cov_kwds={"maxlags": maxlags})
    naive = sm.OLS(y, X).fit()
    cases.append({
        "maxlags": maxlags,
        "params": r.params.tolist(),
        "bse": r.bse.tolist(),
        "pvalues": r.pvalues.tolist(),
        "conf_int": r.conf_int().tolist(),
        "dw": float(sm.stats.stattools.durbin_watson(naive.resid)),
        "rsquared": float(naive.rsquared),
    })
fixtures["hac"] = {"y": y.tolist(), "x": X.tolist(), "cases": cases}

w = rng.uniform(0.2, 2.0, n)
wr = sm.WLS(y, X, weights=w).fit()
fixtures["wls"] = {"weights": w.tolist(), "params": wr.params.tolist(), "pvalues": wr.pvalues.tolist()}
print("hac params:", [round(v, 4) for v in cases[0]["params"]])
print("wls params:", [round(v, 4) for v in wr.params])

with open("oracle/fixtures/estimation.json", "w") as f:
    json.dump(fixtures, f, indent=1)
print("wrote oracle/fixtures/estimation.json")
