# Golden fixtures for statsmodels' STL, the seasonal trend decomposition 805_dag runs as a
# preprocessing step at period 52 with robust weights.
# Run: uv run python oracle/stl.py
import json

import numpy as np
import pandas as pd
from statsmodels.tsa.seasonal import STL

rng = np.random.default_rng(17)


def strength(comp, resid):
    v = float((comp + resid).var())
    return max(0.0, 1.0 - float(resid.var()) / v) if v > 1e-12 else 0.0


cases = []

# Weekly data with a yearly cycle, a drifting trend, and a few outliers for the robust loop.
n, period = 260, 52
t = np.arange(n)
series = (
    10.0
    + 0.02 * t
    + 3.0 * np.sin(2 * np.pi * t / period)
    + 1.2 * np.cos(4 * np.pi * t / period)
    + rng.normal(scale=0.6, size=n)
)
series[[40, 137, 200]] += 12.0
cases.append({"name": "weekly, period 52, robust", "y": series.tolist(),
              "period": period, "robust": True})

# The same series without the robustness loop, so the inner loop is pinned on its own.
cases.append({"name": "weekly, period 52, not robust", "y": series.tolist(),
              "period": period, "robust": False})

# A short monthly series, where the trend window exceeds the sample and est's h correction
# for len_ > n fires.
n2, period2 = 60, 12
t2 = np.arange(n2)
s2 = 5.0 + 0.05 * t2 + 2.0 * np.sin(2 * np.pi * t2 / period2) + rng.normal(scale=0.4, size=n2)
cases.append({"name": "monthly, period 12, robust", "y": s2.tolist(),
              "period": period2, "robust": True})

# A quarterly series, so the period is small relative to the sample.
n3, period3 = 120, 4
t3 = np.arange(n3)
s3 = 2.0 + 0.01 * t3 + 1.5 * np.sin(2 * np.pi * t3 / period3) + rng.normal(scale=0.3, size=n3)
cases.append({"name": "quarterly, period 4, not robust", "y": s3.tolist(),
              "period": period3, "robust": False})

for case in cases:
    y = pd.Series(np.asarray(case["y"], dtype=float))
    mod = STL(y, period=case["period"], robust=case["robust"])
    fit = mod.fit()
    case["config"] = {k: (int(v) if isinstance(v, (int, np.integer)) else v)
                      for k, v in mod.config.items()}
    case["trend"] = fit.trend.tolist()
    case["seasonal"] = fit.seasonal.tolist()
    case["resid"] = fit.resid.tolist()
    case["weights"] = np.asarray(fit.weights).tolist()
    case["trend_strength"] = strength(fit.trend, fit.resid)
    case["seasonal_strength"] = strength(fit.seasonal, fit.resid)

# A jump setting too, since the interpolation branch of ess is otherwise never exercised.
y = pd.Series(np.asarray(cases[0]["y"], dtype=float))
mod = STL(y, period=52, robust=True, seasonal_jump=3, trend_jump=5, low_pass_jump=2)
fit = mod.fit()
cases.append({
    "name": "weekly, period 52, robust, with jumps",
    "y": cases[0]["y"], "period": 52, "robust": True,
    "jumps": {"seasonal": 3, "trend": 5, "low_pass": 2},
    "config": {k: (int(v) if isinstance(v, (int, np.integer)) else v)
               for k, v in mod.config.items()},
    "trend": fit.trend.tolist(), "seasonal": fit.seasonal.tolist(),
    "resid": fit.resid.tolist(), "weights": np.asarray(fit.weights).tolist(),
    "trend_strength": strength(fit.trend, fit.resid),
    "seasonal_strength": strength(fit.seasonal, fit.resid),
})

with open("oracle/fixtures/stl.json", "w") as fh:
    json.dump({"cases": cases}, fh)

for c in cases:
    cfg = c["config"]
    print(f"  {c['name']}")
    print(f"    seasonal={cfg['seasonal']} trend={cfg['trend']} low_pass={cfg['low_pass']} "
          f"robust={cfg['robust']}")
    print(f"    trend strength {c['trend_strength']:.4f}, "
          f"seasonal strength {c['seasonal_strength']:.4f}, "
          f"min weight {min(c['weights']):.4f}")
