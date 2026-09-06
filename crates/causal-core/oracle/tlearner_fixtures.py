# Golden fixtures for the EconML T-learner port at the causal worker's forest settings.
# Run: uv run python oracle/tlearner_fixtures.py
import json

import numpy as np
from econml.metalearners import TLearner
from sklearn.ensemble import RandomForestRegressor

rng = np.random.default_rng(157)
n = 400
w1 = rng.normal(0, 1, n)
w2 = rng.normal(0, 1, n)
d = (0.8 * w1 - 0.5 * w2 + rng.normal(0, 1, n) > 0).astype(float)
# The effect varies with w1, so the per-row effect is not one number.
y = (0.6 + 0.5 * w1) * d + 0.9 * w1 + 0.4 * w2 + rng.normal(0, 1, n)
x = np.column_stack([w1, w2])
query = np.array([[-1.0, 0.0], [0.0, 0.0], [1.0, 0.0], [0.5, -0.5], [2.0, 1.0]])

learner = TLearner(models=RandomForestRegressor(n_estimators=200, min_samples_leaf=5, random_state=7))
learner.fit(y, d, X=x)
effects = learner.effect(x)
query_effects = learner.effect(query)
print("ate", float(learner.ate(X=x)), "effects", effects[:3])
with open("oracle/fixtures/tlearner.json", "w") as f:
    json.dump({
        "w1": w1.tolist(), "w2": w2.tolist(), "d": d.tolist(), "y": y.tolist(),
        "effects": effects.tolist(), "ate": float(learner.ate(X=x)),
        "query": query.tolist(), "query_effects": query_effects.tolist(),
        "control_rows": int((d == 0).sum()), "treated_rows": int((d == 1).sum()),
    }, f, indent=1)
print("wrote oracle/fixtures/tlearner.json")
