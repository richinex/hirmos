# Golden fixtures for the DoubleML PLR/IRM port at the causal worker's exact settings.
# Run: uv run python oracle/dml_fixtures.py
import json

import numpy as np
import pandas as pd
import doubleml as dml_lib
from sklearn.ensemble import RandomForestClassifier, RandomForestRegressor

fixtures = []
rng = np.random.default_rng(157)
n = 400
w1 = rng.normal(0, 1, n)
w2 = rng.normal(0, 1, n)
d_bin = (0.8 * w1 - 0.5 * w2 + rng.normal(0, 1, n) > 0).astype(float)
d_cont = 0.8 * w1 - 0.5 * w2 + rng.normal(0, 1, n)
y_bin = 0.6 * d_bin + 0.9 * w1 + 0.4 * w2 + rng.normal(0, 1, n)
y_cont = 0.6 * d_cont + 0.9 * w1 + 0.4 * w2 + rng.normal(0, 1, n)


def run(name, kind, dvals, yvals, score="ATE", light=False):
    frame = pd.DataFrame({"y": yvals, "d": dvals, "w1": w1, "w2": w2})
    np.random.seed(7)
    booked = dml_lib.DoubleMLData(frame, y_col="y", d_cols="d", x_cols=["w1", "w2"])
    forest = dict(n_estimators=80 if light else 200, min_samples_leaf=5, random_state=7)
    folds = 2 if light else 5
    treat_binary = set(np.unique(dvals)) <= {0.0, 1.0}
    if kind == "irm":
        fitted = dml_lib.DoubleMLIRM(
            booked, ml_g=RandomForestRegressor(**forest), ml_m=RandomForestClassifier(**forest),
            score=score, n_folds=folds, trimming_threshold=0.01,
        )
    else:
        fitted = dml_lib.DoubleMLPLR(
            booked, ml_l=RandomForestRegressor(**forest),
            ml_m=RandomForestClassifier(**forest) if treat_binary else RandomForestRegressor(**forest),
            n_folds=folds,
        )
    fitted.fit()
    ci = fitted.confint(level=0.95)
    fixtures.append({
        "name": name, "kind": kind, "score": score, "light": light,
        "d": list(map(float, dvals)), "y": list(map(float, yvals)),
        "coef": float(fitted.coef[0]), "se": float(fitted.se[0]),
        "ci_low": float(ci.iloc[0, 0]), "ci_high": float(ci.iloc[0, 1]),
    })
    print(name, round(float(fitted.coef[0]), 5), round(float(fitted.se[0]), 5))


run("plr_binary", "plr", d_bin, y_bin)
run("plr_continuous", "plr", d_cont, y_cont)
run("plr_light", "plr", d_cont, y_cont, light=True)
run("irm_ate", "irm", d_bin, y_bin)
run("irm_atte", "irm", d_bin, y_bin, score="ATTE")

with open("oracle/fixtures/dml.json", "w") as f:
    json.dump({"w1": w1.tolist(), "w2": w2.tolist(), "cases": fixtures}, f, indent=1)
print("wrote oracle/fixtures/dml.json")
