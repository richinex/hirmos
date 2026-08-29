# The Gibbs sampler behind the Bayesian structural time series work (Fulton's SciPy 2022
# recipe, as used in 109_bayesian_structural_ts.py), made reproducible.
#
# The script as written calls sim.simulate() with no random_state. statsmodels then builds a
# fresh entropy seeded Generator on every call, so the chain cannot be reproduced from run to
# run, in Python or anywhere else. Passing one Generator through fixes that and is what this
# oracle does, so the port can be checked draw for draw.
# Run: uv run python oracle/bsts_gibbs.py
import json

import numpy as np
import statsmodels.api as sm
from scipy import stats

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
sim = model.ssm.simulation_smoother(simulate_state=True)
priors = [stats.invgamma(0.001, scale=0.001)] * model.k_params

NITER = 200
STATE_SEED = 21
INVGAMMA_SEED = 5
params = np.zeros((NITER + 1, 3))
params[0] = np.asarray(model.start_params, dtype=float)

state_gen = np.random.default_rng(STATE_SEED)
np.random.seed(INVGAMMA_SEED)


def draw_ig(resid, prior):
    post_shape = np.sum(~np.isnan(resid)) / 2 + prior.args[0]
    post_scale = np.nansum(resid ** 2) / 2 + prior.kwds["scale"]
    return float(stats.invgamma(post_shape, scale=post_scale).rvs())


first_states = None
for i in range(1, NITER + 1):
    model.update(params[i - 1])
    sim.simulate(random_state=state_gen)
    states = np.asarray(sim.simulated_state).T  # nobs x 2
    if i == 1:
        first_states = states.copy()
    params[i, 0] = draw_ig(y - states[:, 0], priors[0])
    params[i, 1] = draw_ig(states[1:, 0] - (states[:-1, 0] + states[:-1, 1]), priors[1])
    params[i, 2] = draw_ig(states[1:, 1] - states[:-1, 1], priors[2])

out = {
    "y": y.tolist(),
    "niter": NITER,
    "state_seed": STATE_SEED,
    "invgamma_seed": INVGAMMA_SEED,
    "start_params": params[0].tolist(),
    "params": params.tolist(),
    "first_states": first_states.tolist(),
}
with open("oracle/fixtures/bsts_gibbs.json", "w") as fh:
    json.dump(out, fh)
print("first draw :", np.round(params[1], 6))
print("last draw  :", np.round(params[-1], 6))
print("posterior median:", np.round(np.median(params[50:], axis=0), 6))
