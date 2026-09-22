# Golden fixtures for regression with ARMA errors: statsmodels SARIMAX(y, exog, order=(p, 0, q),
# trend='n') at its defaults, the error-model half of the dynamic harmonic regression of Hyndman
# and Athanasopoulos (FPP3 §10.5). The designs are the Sicily rate on the interrupted-series
# columns (constant, time, step, two harmonic pairs), the same exog the continuous fit uses with
# Newey-West errors. Fitted at the library defaults the port follows: L-BFGS with forward
# differences at epsilon 1e-5 on the average log likelihood, maxiter 50, stationary initialisation,
# outer-product-of-gradients covariance from complex-step scores.
# Run: uv run python oracle/arma_regression.py
import csv
import json
import warnings
from pathlib import Path

import numpy as np
import statsmodels.api as sm
from statsmodels.tsa.statespace.sarimax import SARIMAX

warnings.filterwarnings("ignore")
here = Path(__file__).parent
rows = list(csv.DictReader(open(here / "sicily.csv")))
aces = np.array([float(r["aces"]) for r in rows])
time = np.array([float(r["time"]) for r in rows])
month = np.array([float(r["month"]) for r in rows])
smokban = np.array([float(r["smokban"]) for r in rows])
stdpop = np.array([float(r["stdpop"]) for r in rows])
rate = aces / stdpop * 1e5


def harmonic(x, nfreq, period):
    k = np.arange(1, nfreq + 1) * 2 * np.pi / period
    m = np.outer(x, k)
    return np.column_stack([np.sin(m), np.cos(m)])


x3 = np.column_stack([np.ones_like(time), time, smokban, harmonic(month, 2, 12)])
x1 = np.column_stack([np.ones_like(time), time, smokban])


def case(y, exog, p, q, maxiter=50):
    model = SARIMAX(y, exog=exog, order=(p, 0, q), trend="n")
    start = model.start_params
    fit = model.fit(disp=False, maxiter=maxiter)
    # The score per observation at the optimum by complex step, the OPG matrix the covariance
    # inverts, so the port's finite-difference scores can be checked against the exact ones.
    score_obs = model.score_obs(fit.params)
    return {
        "p": p,
        "q": q,
        "maxiter": maxiter,
        "start_params": start.tolist(),
        "unconstrained_start": model.untransform_params(start).tolist(),
        "params": fit.params.tolist(),
        "llf": float(fit.llf),
        "llf_obs": fit.llf_obs.tolist(),
        "bse": fit.bse.tolist(),
        "pvalues": fit.pvalues.tolist(),
        "conf_int": fit.conf_int().tolist(),
        "resid": fit.resid.tolist(),
        "forecasts_error_cov": fit.filter_results.forecasts_error_cov[0, 0, :].tolist(),
        "standardized_forecasts_error": fit.standardized_forecasts_error[0].tolist(),
        "aic": float(fit.aic),
        "bic": float(fit.bic),
        "iterations": int(fit.mle_retvals["iterations"]),
        "converged": bool(fit.mle_retvals["converged"]),
        "score_obs": score_obs.tolist(),
    }


fixtures = {
    "y": rate.tolist(),
    "x3": x3.tolist(),
    "x1": x1.tolist(),
    "x3_ar1": case(rate, x3, 1, 0),
    "x3_ma1": case(rate, x3, 0, 1),
    "x3_arma11": case(rate, x3, 1, 1),
    "x3_ar2": case(rate, x3, 2, 0),
    "x1_arma21": case(rate, x1, 2, 1),
    # The same three at an iteration limit they reach convergence under, for a tight parity check.
    "x3_arma11_converged": case(rate, x3, 1, 1, maxiter=500),
    "x3_ar2_converged": case(rate, x3, 2, 0, maxiter=500),
    "x1_arma21_converged": case(rate, x1, 2, 1, maxiter=500),
}
# The stationary initial state covariance for the fitted ARMA(1,1), read back from the model so
# the port's Lyapunov solve is checked directly.
model = SARIMAX(rate, exog=x3, order=(1, 0, 1), trend="n")
model.update(np.array(fixtures["x3_arma11"]["params"]))
_, _, p0 = model.ssm.initialization(model=model.ssm)
fixtures["x3_arma11"]["initial_state_cov"] = p0.tolist()
fixtures["x3_arma11"]["transition"] = model.ssm["transition"].tolist()
fixtures["x3_arma11"]["selection"] = model.ssm["selection"][:, 0].tolist()
out = here / "fixtures" / "arma_regression.json"
out.write_text(json.dumps(fixtures, indent=1))
for k, v in fixtures.items():
    if isinstance(v, dict):
        print(k, "params", np.round(v["params"], 6).tolist(), "llf", round(v["llf"], 6), "iterations", v["iterations"], "converged", v["converged"])
