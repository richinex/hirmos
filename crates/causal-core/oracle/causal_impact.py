# Causal impact without the CausalImpact package.
#
# The published `causalimpact` 0.2.6 constructs
# `UnobservedComponents(endog=y, level='llevel', exog=controls)` on standardised data and
# fits it by maximum likelihood; the package itself cannot run on pandas 2, and the notebook
# calls it in a way that raises regardless. This oracle builds that same model directly in
# statsmodels: fit on the pre-intervention window, forecast the counterfactual over the post
# window, and report the pointwise and cumulative impact.
# Run: uv run python oracle/causal_impact.py
import json

import numpy as np
import statsmodels.api as sm

rng = np.random.default_rng(17)
n, n_pre = 90, 60
x1 = np.cumsum(rng.normal(0, 0.4, n)) + 10.0
x2 = rng.normal(5, 1.0, n)
level = np.cumsum(rng.normal(0, 0.25, n))
y = level + 1.1 * x1 - 0.4 * x2 + rng.normal(0, 0.5, n)
y[n_pre:] += 2.5  # the intervention

exog = np.column_stack([x1, x2])
y_pre, exog_pre = y[:n_pre], exog[:n_pre]
exog_post = exog[n_pre:]

model = sm.tsa.UnobservedComponents(y_pre, level="llevel", exog=exog_pre)
start = np.asarray(model.start_params, dtype=float)
model.update(start)
res_start = model.ssm.filter()

fitted = model.fit(disp=False)
forecast = fitted.get_forecast(steps=n - n_pre, exog=exog_post)
counterfactual = np.asarray(forecast.predicted_mean)
se = np.asarray(forecast.se_mean)

pointwise = y[n_pre:] - counterfactual
out = {
    "y": y.tolist(),
    "exog": exog.tolist(),
    "n_pre": n_pre,
    "param_names": list(model.param_names),
    "start_params": start.tolist(),
    "llf_start": float(model.loglike(start)),
    "loglikeobs_start": model.loglikeobs(start).tolist(),
    "forecasts_error_start": np.squeeze(np.asarray(res_start.forecasts_error)).tolist(),
    "fit_params": fitted.params.tolist(),
    "fit_llf": float(fitted.llf),
    "counterfactual": counterfactual.tolist(),
    "counterfactual_se": se.tolist(),
    "pointwise_impact": pointwise.tolist(),
    "cumulative_impact": float(np.sum(pointwise)),
    "average_impact": float(np.mean(pointwise)),
}
with open("oracle/fixtures/causal_impact.json", "w") as fh:
    json.dump(out, fh)
print("param_names :", out["param_names"])
print("start_params:", np.round(start, 6))
print("fit params  :", np.round(fitted.params, 6), "llf", round(float(fitted.llf), 6))
print("average impact %.6f | cumulative %.6f" % (out["average_impact"], out["cumulative_impact"]))
