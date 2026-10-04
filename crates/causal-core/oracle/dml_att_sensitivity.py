"""ATT sensitivity oracle, including rare treatment and the NSW placebo sample.

Run with the pinned hirmos-did-oracle:py312 image (DoubleML 0.10.1).
External nuisance predictions isolate score/bound parity from forest fitting.
"""
import json
import sys
import numpy as np
import pandas as pd
import doubleml
from sklearn.ensemble import RandomForestClassifier, RandomForestRegressor

assert doubleml.__version__ == "0.10.1"

def evaluate(name, frame, covariates):
    np.random.seed(7)
    model = doubleml.DoubleMLIRM(
        doubleml.DoubleMLData(frame, y_col="y", d_cols="d", x_cols=covariates),
        ml_g=RandomForestRegressor(n_estimators=200, min_samples_leaf=5, random_state=7),
        ml_m=RandomForestClassifier(n_estimators=200, min_samples_leaf=5, random_state=7),
        score="ATTE", n_folds=5, trimming_threshold=0.01,
    ).fit()
    scenarios = []
    for share in [0.02, 0.05, 0.10]:
        model.sensitivity_analysis(cf_y=share, cf_d=share, rho=1.0)
        result = model.sensitivity_params
        scenarios.append({"share": share, **{
            f"{quantity}_{side}": float(result[quantity][side][0])
            for quantity in ["theta", "ci"] for side in ["lower", "upper"]
        }})
    out = {"name": name, "y": frame.y.tolist(), "d": frame.d.tolist(),
           "predictions": {key: value.ravel().tolist() for key, value in model.predictions.items()},
           "coef": float(model.coef[0]), "se": float(model.se[0]), "scenarios": scenarios,
           "sigma2": float(model.sensitivity_elements["sigma2"].ravel()[0]),
           "nu2": float(model.sensitivity_elements["nu2"].ravel()[0])}
    print(name, "n", len(frame), "treated", frame.d.sum(), "estimate", out["coef"], "se", out["se"], "2%", scenarios[0], flush=True)
    return out

rng = np.random.default_rng(41)
n = 1200
x = rng.normal(size=n)
d = np.zeros(n)
d[rng.choice(n, 32, replace=False)] = 1
synthetic = pd.DataFrame({"x": x, "d": d, "y": 0.7*x + 0.5*d + rng.normal(size=n)})
cases = [evaluate("rare_att", synthetic, ["x"])]
raw = pd.read_csv(sys.argv[1])
raw = raw[(raw["sample"] == 2) | ((raw["sample"] == 1) & (raw.treated == 0))]
base = raw[raw.year == 1975].set_index("id").copy()
post = raw[raw.year == 1978].set_index("id")
base["y"] = post.re - base.re
base["d"] = base.experimental.astype(float)
covariates = ["age", "educ", "black", "married", "nodegree", "hisp", "re74"]
assert len(base) == 16417 and base.d.sum() == 425
cases.append(evaluate("nsw_placebo_att", base.reset_index(), covariates))
with open(sys.argv[2], "w") as output:
    json.dump({"oracle": "DoubleML 0.10.1", "cases": cases}, output, allow_nan=False)
