# Golden fixtures for the preprocessing helpers: shapiro, cluster redundancy, VIF.
# Run: uv run python oracle/preprocess_fixtures.py
import json

import numpy as np
from scipy.cluster.hierarchy import fcluster, linkage
from scipy.spatial.distance import squareform
from scipy.stats import shapiro
from statsmodels.stats.outliers_influence import variance_inflation_factor

fixtures = {}

rng = np.random.default_rng(101)
shapiro_cases = []
for name, series in [
    ("normal", rng.normal(0, 1, 120)),
    ("uniform", rng.uniform(-1, 1, 80)),
    ("student_t", rng.standard_t(3, 200)),
    ("small_n8", rng.normal(0, 1, 8)),
    ("lognormal", np.exp(rng.normal(0, 0.5, 60))),
]:
    w, p = shapiro(series)
    shapiro_cases.append({"name": name, "x": series.tolist(), "w": float(w), "p": float(p)})
    print(name, round(float(w), 6), float(p))
fixtures["shapiro"] = shapiro_cases


def _cluster_redundant(corr, threshold, linkage_method="complete"):
    dist = 1.0 - np.abs(corr)
    np.fill_diagonal(dist, 0.0)
    dist = np.clip((dist + dist.T) / 2.0, 0.0, None)
    Z = linkage(squareform(dist, checks=False), method=linkage_method)
    labels = fcluster(Z, t=1.0 - threshold, criterion="distance")
    clusters = {}
    for idx, lab in enumerate(labels):
        clusters.setdefault(int(lab), []).append(idx)
    keep_idx, drop_idx, cluster_lists = [], set(), [sorted(v) for v in clusters.values()]
    for members in cluster_lists:
        keep_idx.append(members[0])
        drop_idx.update(members[1:])
    return sorted(keep_idx), sorted(drop_idx), sorted(cluster_lists)


def _vif_redundant(arr, vif_threshold=10.0):
    n = arr.shape[1]
    keep = list(range(n))
    drop_idx = set()
    history = []
    while len(keep) >= 2:
        X = arr[:, keep]
        Xc = np.column_stack([np.ones(len(X)), X])
        vifs = [variance_inflation_factor(Xc, p + 1) for p in range(len(keep))]
        vifs = [v if np.isfinite(v) else np.inf for v in vifs]
        worst = max(range(len(keep)), key=lambda p: (vifs[p], keep[p]))
        if vifs[worst] < vif_threshold:
            break
        history.append((keep[worst], float(vifs[worst])))
        drop_idx.add(keep[worst])
        keep.pop(worst)
    return sorted(keep), sorted(drop_idx), history


# Correlated block structure.
T, base = 300, rng.normal(0, 1, (300, 3))
data = np.column_stack([
    base[:, 0],
    base[:, 0] + 0.05 * rng.normal(0, 1, T),
    base[:, 1],
    0.9 * base[:, 1] + 0.1 * rng.normal(0, 1, T),
    base[:, 2],
    base[:, 0] - base[:, 1] + 0.5 * rng.normal(0, 1, T),
])
corr = np.corrcoef(data.T)
keep, drop, clusters = _cluster_redundant(corr, 0.9)
fixtures["cluster"] = {"data": data.tolist(), "correlation": corr.tolist(), "threshold": 0.9, "keep": keep, "drop": drop, "clusters": clusters}
print("cluster keep:", keep, "drop:", drop)

keep_v, drop_v, hist = _vif_redundant(data, 10.0)
fixtures["vif"] = {"threshold": 10.0, "keep": keep_v, "drop": drop_v,
                   "history": [[int(i), v] for i, v in hist]}
print("vif keep:", keep_v, "history:", [(i, round(v, 2)) for i, v in hist])

with open("oracle/fixtures/preprocess.json", "w") as f:
    json.dump(fixtures, f, indent=1)
print("wrote oracle/fixtures/preprocess.json")
