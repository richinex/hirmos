# Golden fixtures for the causal worker's DML refuter block: the full fit, the placebo and
# random-common-cause light refits on the shared global stream, and the sensitivity bounds.
# Run: uv run python oracle/refute_dml_fixtures.py
import json
import math

import numpy as np
import pandas as pd
import doubleml as dml_lib
from scipy import stats as _stats
from sklearn.ensemble import RandomForestClassifier, RandomForestRegressor


def make_case(seed, n):
    gen = np.random.default_rng(seed)
    w1 = gen.normal(size=n)
    w2 = gen.normal(size=n)
    ps = 1.0 / (1.0 + np.exp(-(0.7 * w1 - 0.4 * w2)))
    dv = (gen.uniform(size=n) < ps).astype(float)
    yv = 0.6 * dv + 0.9 * w1 + 0.4 * w2 + gen.normal(size=n)
    return pd.DataFrame({"y": yv, "d": dv, "w1": w1, "w2": w2})


def run_case(frame, estimator_kind, estimand_kind):
    outcome, treatment, backdoor_vars = "y", "d", ["w1", "w2"]
    treat_binary = set(float(v) for v in np.unique(frame[treatment])) <= {0.0, 1.0}
    np.random.seed(7)

    def fit_model(current_frame, light=False, columns=None):
        columns = backdoor_vars if columns is None else columns
        booked = dml_lib.DoubleMLData(current_frame, y_col=outcome, d_cols=treatment, x_cols=columns)
        forest = dict(n_estimators=80 if light else 200, min_samples_leaf=5, random_state=7)
        folds = 2 if light else 5
        if estimator_kind == "dml_aipw":
            fitted = dml_lib.DoubleMLIRM(
                booked, ml_g=RandomForestRegressor(**forest), ml_m=RandomForestClassifier(**forest),
                score="ATTE" if estimand_kind == "att" else "ATE",
                n_folds=folds, trimming_threshold=0.01,
            )
        else:
            fitted = dml_lib.DoubleMLPLR(
                booked, ml_l=RandomForestRegressor(**forest),
                ml_m=RandomForestClassifier(**forest) if treat_binary else RandomForestRegressor(**forest),
                n_folds=folds,
            )
        fitted.fit()
        return fitted

    fitted = fit_model(frame)
    effect = float(fitted.coef[0])
    ci = fitted.confint(level=0.95)

    rng = np.random.default_rng(7)
    base = frame
    sims = []
    for _ in range(8):
        shuffled = base.copy()
        shuffled[treatment] = rng.permutation(shuffled[treatment].to_numpy())
        sims.append(float(fit_model(shuffled, light=True).coef[0]))
    sims = np.asarray(sims, dtype=float)
    spread = float(sims.std(ddof=1))
    z = float(sims.mean()) / (spread / math.sqrt(len(sims))) if spread > 0 else 0.0
    placebo = {
        "sims": sims.tolist(),
        "refutedEffect": float(sims.mean()),
        "pValue": float(2 * (1 - _stats.norm.cdf(abs(z)))),
    }

    rng = np.random.default_rng(11)
    base = frame
    paired_folds = np.random.get_state()
    baseline = float(fit_model(base, light=True).coef[0])
    after_baseline = np.random.get_state()
    sims = []
    for _ in range(6):
        augmented = base.copy()
        augmented["_random_cause"] = rng.standard_normal(len(augmented))
        np.random.set_state(paired_folds)
        sims.append(float(fit_model(augmented, light=True, columns=backdoor_vars + ["_random_cause"]).coef[0]))
    np.random.set_state(after_baseline)
    sims = np.asarray(sims, dtype=float)
    shifts = sims - baseline
    spread = float(shifts.std(ddof=1))
    z = float(shifts.mean()) / (spread / math.sqrt(len(shifts))) if spread > 0 else 0.0
    rcc = {
        "baseline": baseline,
        "sims": sims.tolist(),
        "refutedEffect": float(sims.mean()),
        "pValue": float(2 * (1 - _stats.norm.cdf(abs(z)))),
    }

    scenarios = []
    robustness = None
    robustness_ci = None
    for cf in (0.02, 0.05, 0.10):
        fitted.sensitivity_analysis(cf_y=cf, cf_d=cf, rho=1.0)
        params = fitted.sensitivity_params
        theta = params["theta"]
        ci_bounds = params["ci"]
        if robustness is None:
            robustness = float(np.ravel(params["rv"])[0])
            robustness_ci = float(np.ravel(params["rva"])[0])
        scenarios.append({
            "confounding": cf,
            "effectLower": float(np.ravel(theta["lower"])[0]),
            "effectUpper": float(np.ravel(theta["upper"])[0]),
            "ciLower": float(np.ravel(ci_bounds["lower"])[0]),
            "ciUpper": float(np.ravel(ci_bounds["upper"])[0]),
        })

    return {
        "effect": effect,
        "ciLow": float(ci.iloc[0, 0]),
        "ciHigh": float(ci.iloc[0, 1]),
        "placebo": placebo,
        "randomCommonCause": rcc,
        "sensitivity": {
            "robustnessValue": max(0.0, min(1.0, robustness)),
            "robustnessValueCi": max(0.0, min(1.0, robustness_ci)),
            "scenarios": scenarios,
        },
    }


def main():
    out = {}
    frame = make_case(21, 1200)
    out["data"] = {c: frame[c].tolist() for c in frame.columns}
    out["plr"] = run_case(frame, "dml_plr", "ate")
    out["aipw"] = run_case(frame, "dml_aipw", "ate")
    # Unit probes for the Generator port.
    out["standard_normal_11"] = np.random.default_rng(11).standard_normal(1000).tolist()
    gen = np.random.default_rng(7)
    out["permutation_7"] = gen.permutation(np.arange(12, dtype=float)).tolist()
    out["sample_rows_7"] = np.random.RandomState(7).permutation(1200)[:20].tolist()
    with open("oracle/fixtures/refute_dml.json", "w") as fh:
        json.dump(out, fh)
    print("plr effect", out["plr"]["effect"], "aipw effect", out["aipw"]["effect"])
    print("plr placebo p", out["plr"]["placebo"]["pValue"], "rv", out["plr"]["sensitivity"]["robustnessValue"])


if __name__ == "__main__":
    main()
