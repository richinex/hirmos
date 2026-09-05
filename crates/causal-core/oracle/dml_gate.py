# Golden fixtures for DoubleML's group average treatment effects on the DML port's own cases.
# Run: uv run python oracle/dml_gate.py
import json

import numpy as np
import pandas as pd
import doubleml as dml_lib
from sklearn.ensemble import RandomForestClassifier, RandomForestRegressor

root = json.load(open("oracle/fixtures/dml.json"))
w1 = np.array(root["w1"])
w2 = np.array(root["w2"])
cases = {case["name"]: case for case in root["cases"]}

# Two groupings the workbench would draw: terciles of one covariate, and a sign split of the other.
terciles = pd.qcut(w1, 3, labels=False).astype(int)
sign = (w2 > 0).astype(int)
groupings = {"w1_terciles": terciles, "w2_sign": sign}


def fit(case):
    frame = pd.DataFrame({"y": case["y"], "d": case["d"], "w1": w1, "w2": w2})
    np.random.seed(7)
    booked = dml_lib.DoubleMLData(frame, y_col="y", d_cols="d", x_cols=["w1", "w2"])
    forest = dict(n_estimators=200, min_samples_leaf=5, random_state=7)
    treat_binary = set(np.unique(case["d"])) <= {0.0, 1.0}
    if case["kind"] == "irm":
        model = dml_lib.DoubleMLIRM(
            booked, ml_g=RandomForestRegressor(**forest), ml_m=RandomForestClassifier(**forest),
            score=case["score"], n_folds=5, trimming_threshold=0.01,
        )
    else:
        model = dml_lib.DoubleMLPLR(
            booked, ml_l=RandomForestRegressor(**forest),
            ml_m=RandomForestClassifier(**forest) if treat_binary else RandomForestRegressor(**forest),
            n_folds=5,
        )
    model.fit()
    return model


def dummies(labels):
    count = int(labels.max()) + 1
    return pd.DataFrame({f"Group_{g}": labels == g for g in range(count)})


output = {"reference": {"doubleml_version": dml_lib.__version__}, "groupings": {k: v.tolist() for k, v in groupings.items()}, "cases": []}
for name in ["irm_ate", "plr_binary", "plr_continuous"]:
    model = fit(cases[name])
    record = {"name": name, "coef": float(model.coef[0]), "groupings": {}}
    for grouping, labels in groupings.items():
        gate = model.gate(dummies(labels))
        ci = gate.confint(level=0.95)
        record["groupings"][grouping] = {
            "effects": [float(v) for v in gate.coef],
            "standard_errors": [float(v) for v in gate.se],
            "ci_low": [float(v) for v in ci.iloc[:, 0]],
            "ci_high": [float(v) for v in ci.iloc[:, 2]],
            "observations": [int(v) for v in np.bincount(labels)],
        }
        print(name, grouping, [round(float(v), 4) for v in gate.coef])
    output["cases"].append(record)

atte = fit(cases["irm_atte"])
try:
    atte.gate(dummies(sign))
    output["atte_refuses"] = False
except ValueError as error:
    output["atte_refuses"] = True
    output["atte_message"] = str(error)
    print("irm_atte refuses:", error)

with open("oracle/fixtures/dml_gate.json", "w") as f:
    json.dump(output, f, indent=1)
print("wrote oracle/fixtures/dml_gate.json")
