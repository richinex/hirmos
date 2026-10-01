# Golden fixtures for the within (unit fixed effects) regression against linearmodels PanelOLS.
# Run from the repository root in the panel oracle image:
#   docker run --rm --entrypoint python -v "$PWD:/w" -w /w hirmos-panel-oracle:py312 crates/causal-core/oracle/within_fixtures.py
import json
import warnings

import numpy as np
import pandas as pd
from linearmodels.panel import PanelOLS

rng = np.random.default_rng(311)


def panel(labels, sizes, periods=None):
    rows = []
    shocks = {t: rng.normal(0.0, 0.7) for t in range(max(sizes) if periods is None else periods)}
    for unit, size in zip(labels, sizes):
        alpha = rng.normal(0.0, 1.0)
        z = rng.normal(0.0, 1.0)
        # An unbalanced unit is observed in a window of periods, not always from the first.
        start = 0 if periods is None else int(rng.integers(0, periods - size + 1))
        for t in range(start, start + size):
            x1, x2, x3 = rng.normal(), rng.normal(), rng.uniform(-1.0, 1.0)
            e = rng.normal(0.0, 0.5 + abs(x1))
            y = 1.5 * x1 - 0.7 * x2 + 0.3 * x3 + 0.8 * z + alpha + shocks[t] + e
            # w is the same for every unit in a period, so time effects absorb it.
            rows.append(dict(unit=unit, t=t, y=y, x1=x1, x2=x2, z=z, x3=x3, w=shocks[t] * 2.0 + 1.0))
    return pd.DataFrame(rows)


def case(frame, regressors, time_effects=False, entity_effects=True, cluster_by=None, weighted=False):
    p = frame.set_index(["unit", "t"])
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        model = PanelOLS(p["y"], p[regressors], weights=p["weight"] if weighted else None, entity_effects=entity_effects, time_effects=time_effects, drop_absorbed=True)
        cluster_options = {"cluster_entity": True} if cluster_by is None else {"clusters": pd.Series(frame[cluster_by].to_numpy(), index=p.index)}
        fits = {
            "classical": model.fit(cov_type="unadjusted", debiased=True),
            "hc1": model.fit(cov_type="robust", debiased=True),
            "cluster": model.fit(cov_type="clustered", **cluster_options, group_debias=True, debiased=True),
        }
    kept = list(fits["classical"].params.index)
    out = {
        "y": frame["y"].tolist(),
        "weights": frame["weight"].tolist() if weighted else None,
        "x": frame[regressors].to_numpy().tolist(),
        "groups": frame["unit"].astype(int).tolist(),
        "clusters": frame["unit" if cluster_by is None else cluster_by].astype(int).tolist(),
        "entity_effects": entity_effects,
        "times": frame["t"].astype(int).tolist(),
        "periods": int(frame["t"].nunique()),
        "regressors": regressors,
        "kept": [regressors.index(name) for name in kept],
        "units": int(frame["unit"].nunique()),
        "df_resid": int(fits["classical"].df_resid),
        "resid_ss": float(fits["classical"].resid_ss),
        "residuals": fits["classical"].resids.tolist(),
        "rsquared_within": float(fits["classical"].rsquared_within),
    }
    for name, fit in fits.items():
        out[name] = {
            "covariance": fit.cov.to_numpy().tolist(),
            "params": fit.params.tolist(),
            "bse": fit.std_errors.tolist(),
            "pvalues": fit.pvalues.tolist(),
            "conf_int": fit.conf_int().to_numpy().tolist(),
        }
    return out


fixtures = {
    # Non-dense unit labels; z is constant within a unit and so absorbed.
    "balanced_absorbed": case(panel([13 * i + 5 for i in range(40)], [5] * 40), ["x1", "x2", "z", "x3"]),
    # Units of unequal size, nothing absorbed.
    "unbalanced": case(panel([7 * i + 2 for i in range(30)], [2 + i % 5 for i in range(30)]), ["x1", "x2", "x3"]),
    # Unit and time effects on a balanced panel; z is absorbed by the units and w by the periods.
    "two_way_balanced": case(panel([13 * i + 5 for i in range(40)], [5] * 40), ["x1", "x2", "z", "w", "x3"], time_effects=True),
    # Unit and time effects on an unbalanced panel observed over windows of 8 periods.
    "two_way_unbalanced": case(panel([7 * i + 2 for i in range(30)], [3 + i % 5 for i in range(30)], periods=8), ["x1", "w", "x2", "x3"], time_effects=True),
}
extra = panel(list(range(30)), [6] * 30)
extra["school"] = extra["unit"] // 3
fixtures.update({
    "time_only_cluster_unit": case(extra, ["x1", "x2", "x3"], time_effects=True, entity_effects=False),
    "unit_cluster_school": case(extra, ["x1", "x2", "x3"], cluster_by="school"),
    "unit_cluster_time": case(extra, ["x1", "x2", "x3"], cluster_by="t"),
    "two_way_cluster_school": case(extra, ["x1", "x2", "x3"], time_effects=True, cluster_by="school"),
})
extra["weight"] = rng.uniform(0.1, 4.0, len(extra))
unequal = panel(list(range(25)), [3 + i % 5 for i in range(25)], periods=9)
unequal["weight"] = rng.uniform(0.1, 4.0, len(unequal))
fixtures.update({
    "weighted_unit": case(extra, ["x1", "x2", "z", "x3"], weighted=True),
    "weighted_time": case(extra, ["x1", "x2", "x3"], entity_effects=False, time_effects=True, weighted=True),
    "weighted_both": case(extra, ["x1", "x2", "z", "w", "x3"], time_effects=True, weighted=True),
    "weighted_unbalanced": case(unequal, ["x1", "x2", "x3"], weighted=True),
    "weighted_both_unbalanced": case(unequal, ["x1", "x2", "x3"], time_effects=True, weighted=True),
    "weighted_school": case(extra, ["x1", "x2", "x3"], cluster_by="school", weighted=True),
})
with open("crates/causal-core/oracle/fixtures/within.json", "w") as handle:
    json.dump(fixtures, handle)
for name, fixture in fixtures.items():
    print(name, "kept", fixture["kept"], "df_resid", fixture["df_resid"], "cluster bse", [round(v, 5) for v in fixture["cluster"]["bse"]])
