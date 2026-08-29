# Golden fixtures for the PELT changepoint detector, at the settings the notebooks use:
# ruptures.Pelt(model="l2", min_size=4, jump=1) and Pelt(model="rbf", min_size=8) with the
# Wheeler style penalties those scripts compute.
# Run: uv run python oracle/pelt.py
import json

import numpy as np
import ruptures as rpt

cases = []
rng = np.random.default_rng(31)


def wheeler_penalty(values, calibration=4.0):
    """The penalty the notebooks use: cal * log(n) * sigma^2 from the moving range."""
    sigma = np.median(np.abs(np.diff(values))) / 1.128
    return float(calibration * np.log(len(values)) * sigma ** 2)


# Univariate mean shifts, the shape 114i, 134 and 301w feed in.
for name, segs in [
    ("two_shifts", [(0.0, 40), (3.0, 35), (-1.0, 45)]),
    ("single_shift", [(0.0, 50), (2.0, 50)]),
    ("noisy_four", [(0.0, 30), (1.5, 25), (0.2, 30), (2.8, 35)]),
]:
    parts = [rng.normal(mu, 1.0, n) for mu, n in segs]
    values = np.concatenate(parts)
    for min_size, jump in ((4, 1), (8, 1), (2, 5)):
        pen = wheeler_penalty(values)
        bkps = rpt.Pelt(model="l2", min_size=min_size, jump=jump).fit(
            values.reshape(-1, 1)).predict(pen=pen)
        cases.append({
            "name": f"{name}_l2_min{min_size}_jump{jump}",
            "signal": values.reshape(-1, 1).tolist(),
            "model": "l2", "min_size": min_size, "jump": jump, "pen": pen,
            "breakpoints": [int(b) for b in bkps],
        })

# The rbf kernel cost, as 1001 uses it on a factor score.
for name, segs in [
    ("rbf_two", [(0.0, 45), (2.5, 40)]),
    ("rbf_three", [(0.0, 35), (1.8, 30), (-1.2, 40)]),
]:
    values = np.concatenate([rng.normal(mu, 1.0, n) for mu, n in segs])
    n = len(values)
    for mult in (0.5, 1.0, 2.0):
        pen = float(mult * np.log(n) * np.var(values))
        bkps = rpt.Pelt(model="rbf", min_size=8).fit(values.reshape(-1, 1)).predict(pen=pen)
        cases.append({
            "name": f"{name}_rbf_pen{mult}",
            "signal": values.reshape(-1, 1).tolist(),
            "model": "rbf", "min_size": 8, "jump": 5, "pen": pen,
            "breakpoints": [int(b) for b in bkps],
        })

# A multivariate l2 case, since the cost sums across features.
multi = np.column_stack([
    np.concatenate([rng.normal(0, 1, 40), rng.normal(2.5, 1, 40)]),
    np.concatenate([rng.normal(1, 1, 40), rng.normal(-1.0, 1, 40)]),
])
pen = float(2.0 * np.log(len(multi)) * multi.var(axis=0).sum())
bkps = rpt.Pelt(model="l2", min_size=5, jump=1).fit(multi).predict(pen=pen)
cases.append({
    "name": "multivariate_l2", "signal": multi.tolist(),
    "model": "l2", "min_size": 5, "jump": 1, "pen": pen,
    "breakpoints": [int(b) for b in bkps],
})

with open("oracle/fixtures/pelt.json", "w") as fh:
    json.dump({"cases": cases}, fh)
for c in cases:
    print(f"{c['name']:28s} n={len(c['signal']):3d} pen={c['pen']:8.3f} -> {c['breakpoints']}")
