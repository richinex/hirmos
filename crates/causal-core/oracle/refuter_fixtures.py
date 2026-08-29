# Golden fixtures for the dowhy refuters under a seeded global RandomState.
# Run: uv run python oracle/refuter_fixtures.py
import json

import numpy as np
import pandas as pd
from dowhy import CausalModel

rng = np.random.default_rng(127)
n = 400
w = rng.normal(0, 1, n)
t = 0.8 * w + rng.normal(0, 1, n)
y = 0.5 * t + 0.7 * w + rng.normal(0, 1, n)
df = pd.DataFrame({"T": t, "Y": y, "W": w})
model = CausalModel(data=df, treatment="T", outcome="Y", graph="digraph {W -> T; W -> Y; T -> Y}")
ident = model.identify_effect(proceed_when_unidentifiable=True)
est = model.estimate_effect(ident, method_name="backdoor.linear_regression")

results = {}
np.random.seed(555)
ref = model.refute_estimate(ident, est, method_name="placebo_treatment_refuter",
                            placebo_type="permute", num_simulations=50)
results["placebo"] = float(ref.new_effect)
np.random.seed(556)
ref = model.refute_estimate(ident, est, method_name="data_subset_refuter",
                            subset_fraction=0.8, num_simulations=50)
results["subset"] = float(ref.new_effect)
np.random.seed(557)
ref = model.refute_estimate(ident, est, method_name="random_common_cause", num_simulations=50)
results["random_cc"] = float(ref.new_effect)

fixture = {"data": np.column_stack([t, y, w]).tolist(), "ate": float(est.value),
           "num_simulations": 50, "seeds": {"placebo": 555, "subset": 556, "random_cc": 557},
           "results": results}
with open("oracle/fixtures/refuters.json", "w") as f:
    json.dump(fixture, f, indent=1)
print(results)
