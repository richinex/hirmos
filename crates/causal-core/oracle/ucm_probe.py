# Staged probes for the UnobservedComponents('lltrend', use_exact_diffuse=True) port that
# backs the Bayesian structural time series work: state space matrices, start parameters,
# the exact diffuse Kalman filter, and the MLE fit.
# Run: uv run python oracle/ucm_probe.py
import json

import numpy as np
import statsmodels.api as sm

# A local linear trend series with a clear level and slope, deterministic given the seed.
rng = np.random.default_rng(11)
T = 60
level = np.zeros(T)
trend = np.zeros(T)
trend[0] = 0.3
for t in range(1, T):
    trend[t] = trend[t - 1] + rng.normal(0, 0.05)
    level[t] = level[t - 1] + trend[t - 1] + rng.normal(0, 0.2)
y = level + rng.normal(0, 0.7, T)

model = sm.tsa.UnobservedComponents(y, "lltrend", use_exact_diffuse=True)
start = np.asarray(model.start_params, dtype=float)
model.update(start)

out = {
    "y": y.tolist(),
    "param_names": list(model.param_names),
    "start_params": start.tolist(),
    "matrices": {
        "design": np.squeeze(np.asarray(model.ssm["design"])).tolist(),
        "transition": np.squeeze(np.asarray(model.ssm["transition"])).tolist(),
        "selection": np.squeeze(np.asarray(model.ssm["selection"])).tolist(),
        "state_cov": np.squeeze(np.asarray(model.ssm["state_cov"])).tolist(),
        "obs_cov": float(np.squeeze(np.asarray(model.ssm["obs_cov"]))),
    },
}

# The filter at the start parameters: per observation likelihoods and filtered states.
res = model.ssm.filter()
out["filter_start"] = {
    "llf": float(model.loglike(start)),
    "loglikeobs": model.loglikeobs(start).tolist(),
    "nobs_diffuse": int(getattr(res, "nobs_diffuse", 0)),
    "forecasts_error": np.squeeze(np.asarray(res.forecasts_error)).tolist(),
    "forecasts_error_cov": np.squeeze(np.asarray(res.forecasts_error_cov)).tolist(),
    "filtered_state": np.asarray(res.filtered_state).tolist(),
}

# A few likelihood evaluations away from the start, to pin the objective surface.
probe_rng = np.random.default_rng(5)
points = [np.abs(start * np.exp(probe_rng.normal(0, 0.3, start.shape))) for _ in range(4)]
out["probe_points"] = [p.tolist() for p in points]
out["probe_loglikes"] = [float(model.loglike(p)) for p in points]

fitted = model.fit(disp=False)
out["fit"] = {
    "params": fitted.params.tolist(),
    "llf": float(fitted.llf),
    "smoothed_state": np.asarray(fitted.smoothed_state).tolist(),
}

# The smoother, and the Durbin-Koopman simulation smoother under an explicit seed.
# Note: without `random_state` the smoother draws from a fresh entropy-seeded generator
# every call, so a Gibbs chain built on it is not reproducible even in Python.
model.update(start)
smoothed = model.ssm.smooth()
out["smoothed_start"] = {
    "smoothed_state": np.asarray(smoothed.smoothed_state).tolist(),
}

sim = model.ssm.simulation_smoother(simulate_state=True)
out["simulation_smoother"] = []
for seed in (0, 7):
    sim.simulate(random_state=seed)
    gen = np.random.default_rng(seed)
    md = gen.standard_normal(T)
    sd = gen.standard_normal(2 * T)
    init = gen.standard_normal(2)
    out["simulation_smoother"].append({
        "seed": seed,
        "measurement_variates": md.tolist(),
        "state_variates": sd.tolist(),
        "initial_state_variates": init.tolist(),
        "simulated_state": np.asarray(sim.simulated_state).tolist(),
    })

with open("oracle/fixtures/ucm_probe.json", "w") as fh:
    json.dump(out, fh)
print("start_params", np.round(start, 6))
print("llf(start)", out["filter_start"]["llf"], "diffuse periods", out["filter_start"]["nobs_diffuse"])
print("fit params", np.round(fitted.params, 6), "llf", round(float(fitted.llf), 6))
