# Golden fixtures for cointegration: coint_johansen across det orders and Engle-Granger pairs,
# on synthetic I(1) systems and macro data. Run: uv run python oracle/coint_fixtures.py
import json

import numpy as np
import statsmodels.api as sm
from statsmodels.tsa.stattools import coint
from statsmodels.tsa.vector_ar.vecm import coint_johansen

fixtures = {"johansen": [], "engle_granger": []}

# Cointegrated system: common stochastic trend plus stationary spreads.
rng = np.random.default_rng(71)
T = 300
trend = np.cumsum(rng.normal(0, 1, T))
data3 = np.column_stack([
    trend + rng.normal(0, 0.5, T),
    0.8 * trend + rng.normal(0, 0.7, T),
    np.cumsum(rng.normal(0, 1, T)),
])
# Independent random walks.
data_rw = np.column_stack([np.cumsum(rng.normal(0, 1, (T, 2)), axis=0)])
# Macro levels.
macro = sm.datasets.macrodata.load_pandas().data
levels = np.log(macro[["realgdp", "realcons"]].to_numpy())

for name, data, det, k in [
    ("coint3_det0_k1", data3, 0, 1),
    ("coint3_detm1_k1", data3, -1, 1),
    ("coint3_det1_k2", data3, 1, 2),
    ("rw2_det0_k1", data_rw, 0, 1),
    ("macro_det0_k1", levels, 0, 1),
]:
    res = coint_johansen(np.asarray(data, float), det, k)
    fixtures["johansen"].append({
        "name": name, "data": np.asarray(data, float).tolist(), "det_order": det, "k_ar_diff": k,
        "eig": res.eig.tolist(), "lr1": res.lr1.tolist(), "lr2": res.lr2.tolist(),
        "cvt": res.cvt.tolist(), "cvm": res.cvm.tolist(),
    })
    print(name, "trace:", [round(v, 3) for v in res.lr1])

for name, y0, y1 in [
    ("pair_cointegrated", data3[:, 0], data3[:, 1]),
    ("pair_independent", data_rw[:, 0], data_rw[:, 1]),
    ("pair_macro", levels[:, 0], levels[:, 1]),
]:
    stat, pval, crit = coint(y0, y1)
    fixtures["engle_granger"].append({
        "name": name, "y0": y0.tolist(), "y1": y1.tolist(),
        "stat": float(stat), "pval": float(pval), "crit": np.asarray(crit).tolist(),
    })
    print(name, "stat:", round(float(stat), 4), "p:", round(float(pval), 5))

with open("oracle/fixtures/coint.json", "w") as f:
    json.dump(fixtures, f, indent=1)
print("wrote oracle/fixtures/coint.json")
