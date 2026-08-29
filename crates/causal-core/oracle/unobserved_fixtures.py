# Golden fixtures for dowhy's add_unobserved_common_cause direct simulation as the worker's
# linear lane runs it: automatic kappa ranges, binary_flip on treatment, linear on outcome.
# Run: uv run python oracle/unobserved_fixtures.py
import json

import numpy as np
import pandas as pd
from dowhy import CausalModel
from dowhy.causal_refuters.add_unobserved_common_cause import (
    _infer_default_kappa_t,
    _infer_default_kappa_y,
)

rng = np.random.default_rng(131)
n = 500
w1 = rng.normal(0, 1, n)
w2 = rng.normal(0, 1, n)
ps = 1.0 / (1.0 + np.exp(-(0.9 * w1 - 0.6 * w2)))
t = (rng.uniform(size=n) < ps).astype(float)
y = 0.5 * t + 0.7 * w1 + 0.3 * w2 + rng.normal(0, 1, n)
df = pd.DataFrame({"T": t, "Y": y, "W1": w1, "W2": w2})

model = CausalModel(data=df, treatment="T", outcome="Y",
                    graph="digraph {W1 -> T; W1 -> Y; W2 -> T; W2 -> Y; T -> Y}")
identified = model.identify_effect(proceed_when_unidentifiable=False)
estimate = model.estimate_effect(identified, method_name="backdoor.linear_regression")

treatment_strengths = np.atleast_1d(np.asarray(_infer_default_kappa_t(
    df, identified, ["T"], "binary_flip", 1,
), dtype=float))
outcome_strengths = np.atleast_1d(np.asarray(_infer_default_kappa_y(
    df, identified, ["Y"], "linear", 1,
), dtype=float))

np.random.seed(700)
sensitivity = model.refute_estimate(
    identified, estimate, method_name="add_unobserved_common_cause",
    confounders_effect_on_treatment="binary_flip",
    confounders_effect_on_outcome="linear",
    effect_strength_on_treatment=treatment_strengths,
    effect_strength_on_outcome=outcome_strengths,
    plotmethod=None, random_state=7, n_jobs=1, show_progress_bar=False,
)
matrix = np.asarray(sensitivity.new_effect_array, dtype=float)

out = {
    "data": {"t": t.tolist(), "y": y.tolist(), "w1": w1.tolist(), "w2": w2.tolist()},
    "ate": float(estimate.value),
    "seed": 700,
    "kappa_t": treatment_strengths.tolist(),
    "kappa_y": outcome_strengths.tolist(),
    "matrix": matrix.tolist(),
}
with open("oracle/fixtures/unobserved.json", "w") as fh:
    json.dump(out, fh)
print("kappa_t", treatment_strengths)
print("kappa_y", outcome_strengths)
print("matrix corners", matrix[0, 0], matrix[-1, -1], "shape", matrix.shape)
