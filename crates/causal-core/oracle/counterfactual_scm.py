# The linear Gaussian SCM counterfactual of 202w_counterfactual_ness.py.
#
# The script does Pearl's three steps with Pyro: abduction by SVI over the four exogenous
# noise terms, action by pyro.do, prediction by forward sampling. Because every structural
# equation is linear with additive noise and the endogenous nodes are emitted as
# PseudoDelta = Normal(mu, 0.01), the whole system is linear Gaussian, so the posterior over
# the noise is available in closed form and the counterfactual with it. This oracle records
# the exact answer and, alongside it, what 5000 steps of SVI actually reach.
# Run: uv run python oracle/counterfactual_scm.py
import json

import numpy as np
import torch
import pyro
import pyro.distributions as dist
from pyro.infer import SVI, Trace_ELBO
from pyro.optim import Adam as PyroAdam

PSEUDO_SCALE = 0.01

# Coefficients standing in for the OLS fits in the script, fixed so the case is portable.
COEF = {
    "commits_mean": 42.0, "commits_std": 9.0,
    "bugs_intercept": 1.4, "bugs_beta_commits": 0.08, "bugs_residual_std": 1.1,
    "sec_intercept": 0.6, "sec_beta_commits": 0.05, "sec_beta_bugs": 0.35, "sec_residual_std": 0.9,
    "inc_intercept": 0.2, "inc_beta_security": 1.25, "inc_beta_commits": 0.02,
    "inc_beta_bugs": 0.18, "inc_residual_std": 1.3,
}
# The observed week to explain, and the intervention on security.
OBS = {"commits": 55.0, "bugs": 7.2, "security": 4.1, "incidents": 6.8}
DO_SECURITY = 1.5

c = COEF
# y = A n + b, in the node order commits, bugs, security, incidents.
A = np.zeros((4, 4))
b = np.zeros(4)
A[0, 0] = c["commits_std"]
b[0] = c["commits_mean"]
A[1, 0] = c["bugs_beta_commits"] * A[0, 0]
A[1, 1] = c["bugs_residual_std"]
b[1] = c["bugs_intercept"] + c["bugs_beta_commits"] * b[0]
A[2] = c["sec_beta_commits"] * A[0] + c["sec_beta_bugs"] * A[1]
A[2, 2] = c["sec_residual_std"]
b[2] = c["sec_intercept"] + c["sec_beta_commits"] * b[0] + c["sec_beta_bugs"] * b[1]
A[3] = c["inc_beta_security"] * A[2] + c["inc_beta_commits"] * A[0] + c["inc_beta_bugs"] * A[1]
A[3, 3] = c["inc_residual_std"]
b[3] = (c["inc_intercept"] + c["inc_beta_security"] * b[2]
        + c["inc_beta_commits"] * b[0] + c["inc_beta_bugs"] * b[1])

y_obs = np.array([OBS["commits"], OBS["bugs"], OBS["security"], OBS["incidents"]])

# Exact abduction with true deltas: the noise is point identified by triangular inversion.
n_exact = np.linalg.solve(A, y_obs - b)

# Exact abduction under the pseudo delta observation noise: a linear Gaussian posterior.
prec = np.eye(4) + A.T @ A / PSEUDO_SCALE ** 2
post_cov = np.linalg.inv(prec)
post_mean = post_cov @ (A.T @ (y_obs - b) / PSEUDO_SCALE ** 2)


def counterfactual(n):
    """Action and prediction: do(security = DO_SECURITY), then propagate."""
    commits = c["commits_mean"] + c["commits_std"] * n[0]
    bugs = c["bugs_intercept"] + c["bugs_beta_commits"] * commits + c["bugs_residual_std"] * n[1]
    security = DO_SECURITY
    incidents = (c["inc_intercept"] + c["inc_beta_security"] * security
                 + c["inc_beta_commits"] * commits + c["inc_beta_bugs"] * bugs
                 + c["inc_residual_std"] * n[3])
    return commits, bugs, security, incidents


cf_exact = counterfactual(n_exact)
cf_posterior = counterfactual(post_mean)

# What the script's SVI actually reaches.
pyro.clear_param_store()
pyro.util.set_rng_seed(42)
tA = torch.tensor(A, dtype=torch.float64)
tb = torch.tensor(b, dtype=torch.float64)
ty = torch.tensor(y_obs, dtype=torch.float64)


def model():
    n = torch.stack([pyro.sample(f"N{i}", dist.Normal(torch.tensor(0.0, dtype=torch.float64),
                                                      torch.tensor(1.0, dtype=torch.float64)))
                     for i in range(4)])
    mu = tA @ n + tb
    for i in range(4):
        pyro.sample(f"y{i}", dist.Normal(mu[i], torch.tensor(PSEUDO_SCALE, dtype=torch.float64)),
                    obs=ty[i])


def guide():
    for i in range(4):
        loc = pyro.param(f"loc{i}", torch.tensor(0.0, dtype=torch.float64))
        scale = pyro.param(f"scale{i}", torch.tensor(0.1, dtype=torch.float64),
                           constraint=dist.constraints.positive)
        pyro.sample(f"N{i}", dist.Normal(loc, scale))


svi = SVI(model, guide, PyroAdam({"lr": 0.003}), loss=Trace_ELBO())
for _ in range(5000):
    svi.step()
n_svi = np.array([float(pyro.param(f"loc{i}")) for i in range(4)])
cf_svi = counterfactual(n_svi)

out = {
    "coefficients": COEF,
    "observation": OBS,
    "do_security": DO_SECURITY,
    "pseudo_scale": PSEUDO_SCALE,
    "A": A.tolist(),
    "b": b.tolist(),
    "abduction_exact": n_exact.tolist(),
    "abduction_pseudo_delta": post_mean.tolist(),
    "counterfactual_exact": list(cf_exact),
    "counterfactual_pseudo_delta": list(cf_posterior),
    "abduction_svi": n_svi.tolist(),
    "counterfactual_svi": list(cf_svi),
    "observed_incidents": OBS["incidents"],
}
with open("oracle/fixtures/counterfactual_scm.json", "w") as fh:
    json.dump(out, fh)
print("exact abduction        :", np.round(n_exact, 6))
print("pseudo delta posterior :", np.round(post_mean, 6))
print("SVI after 5000 steps   :", np.round(n_svi, 6))
print("counterfactual incidents: exact %.6f | posterior %.6f | SVI %.6f"
      % (cf_exact[3], cf_posterior[3], cf_svi[3]))
print("observed incidents      : %.6f" % OBS["incidents"])
