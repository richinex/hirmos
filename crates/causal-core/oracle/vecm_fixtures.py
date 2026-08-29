# Golden fixtures for VECM: order selection, rank selection, ML fit, Chow test.
# Run: uv run python oracle/vecm_fixtures.py
import json

import numpy as np
from scipy import stats
from statsmodels.tsa.vector_ar.vecm import VECM, select_coint_rank, select_order

fixtures = {}
rng = np.random.default_rng(113)
T = 250
trend = np.cumsum(rng.normal(0, 1, T))
endog = np.column_stack([
    trend + rng.normal(0, 0.6, T) + 0.01 * np.arange(T),
    0.7 * trend + rng.normal(0, 0.8, T),
])

cases = []
for det, det_order in [("coli", 1), ("ci", 0)]:
    sel = select_order(endog, maxlags=6, deterministic=det)
    k_try = int(max(1, sel.aic))
    rank = select_coint_rank(endog, det_order=det_order, k_ar_diff=k_try, method="trace", signif=0.05)
    res = VECM(endog, k_ar_diff=k_try, coint_rank=1, deterministic=det).fit()
    cases.append({
        "deterministic": det,
        "det_order": det_order,
        "aic_k": int(sel.aic),
        "rank": int(rank.rank),
        "alpha": np.asarray(res.alpha).tolist(),
        "beta": np.asarray(res.beta).tolist(),
        "det_coef_coint": np.asarray(res.det_coef_coint).tolist(),
        "gamma": np.asarray(res.gamma).tolist(),
        "pvalues_alpha": np.asarray(res.pvalues_alpha).tolist(),
    })
    print(det, "aic_k:", sel.aic, "rank:", rank.rank, "beta:", np.round(np.asarray(res.beta).ravel(), 4).tolist())
fixtures["vecm"] = {"endog": endog.tolist(), "maxlags": 6, "cases": cases}

# Chow test on a synthetic break.
y = np.concatenate([1.0 + 0.5 * np.arange(60), 40.0 - 0.3 * np.arange(60)]) + rng.normal(0, 2, 120)
x = np.arange(120.0)
k = 60
X = np.column_stack([np.ones(120), x])
b, *_ = np.linalg.lstsq(X, y, rcond=None)
rss_p = ((y - X @ b) ** 2).sum()
rss_s = 0.0
for seg in (slice(0, k), slice(k, 120)):
    Xs = X[seg]
    bs, *_ = np.linalg.lstsq(Xs, y[seg], rcond=None)
    rss_s += ((y[seg] - Xs @ bs) ** 2).sum()
F = ((rss_p - rss_s) / 2) / (rss_s / (120 - 4))
p = stats.f.sf(F, 2, 120 - 4)
fixtures["chow"] = {"y": y.tolist(), "x": x.tolist(), "k": k, "f": float(F), "p": float(p)}
print("chow F:", round(float(F), 4), "p:", float(p))

with open("oracle/fixtures/vecm.json", "w") as f:
    json.dump(fixtures, f, indent=1)
print("wrote oracle/fixtures/vecm.json")
