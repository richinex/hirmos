# Golden fixtures for the two count estimators of 805 step 3f: the Poisson GLM fitted by
# IRLS, and NegativeBinomialP fitted by BFGS.
# Run: uv run python oracle/glm.py
import json

import numpy as np
import statsmodels.api as sm
from scipy.optimize import fmin_bfgs
from statsmodels.discrete.discrete_model import NegativeBinomialP

rng = np.random.default_rng(5)
n = 300
x1 = rng.normal(size=n)
x2 = rng.normal(size=n)
mu = np.exp(1.2 + 0.45 * x1 - 0.3 * x2)
y = rng.negative_binomial(1 / 0.6, 1 / (1 + 0.6 * mu)).astype(float)
X = sm.add_constant(np.column_stack([x1, x2]))

pois = sm.GLM(y, X, family=sm.families.Poisson()).fit()
poi_mle = sm.Poisson(y, X).fit(disp=0)
nb = NegativeBinomialP(y, X, p=2).fit(disp=0)

# The 805 read: the incidence rate ratio on the treatment column and the additive effect at
# the mean rate.
treat = 1
out = {
    "y": y.tolist(),
    "x": X.tolist(),
    "poisson_glm": {
        "params": pois.params.tolist(),
        "bse": pois.bse.tolist(),
        "tvalues": pois.tvalues.tolist(),
        "pvalues": pois.pvalues.tolist(),
        "fittedvalues": pois.fittedvalues.tolist(),
        "deviance": float(pois.deviance),
        "df_resid": float(pois.df_resid),
        "scale": float(pois.scale),
        "llf": float(pois.llf),
        "iterations": int(pois.fit_history["iteration"]),
        "converged": bool(pois.converged),
        "irr": float(np.exp(pois.params[treat])),
        "mean_rate": float(pois.fittedvalues.mean()),
        "dev_over_df": float(pois.deviance / pois.df_resid),
    },
    "poisson_mle": {
        "params": poi_mle.params.tolist(),
        "df_resid": float(poi_mle.df_resid),
        "resid_head": poi_mle.resid[:5].tolist(),
    },
    "negative_binomial_p": {
        "params": nb.params.tolist(),
        "bse": nb.bse.tolist(),
        "tvalues": nb.tvalues.tolist(),
        "pvalues": nb.pvalues.tolist(),
        "fittedvalues": nb.fittedvalues.tolist(),
        "llf": float(nb.llf),
        "converged": bool(nb.mle_retvals["converged"]),
        "warnflag": int(nb.mle_retvals["warnflag"]),
        "fcalls": int(nb.mle_retvals["fcalls"]),
        "gcalls": int(nb.mle_retvals["gcalls"]),
        "gopt_maxabs": float(np.max(np.abs(nb.mle_retvals["gopt"]))),
        "irr": float(np.exp(nb.params[treat])),
        "mean_rate": float(np.exp(nb.fittedvalues).mean()),
    },
}

# The starting values NegativeBinomialP builds before handing over to BFGS. Note that this
# class overrides the base dispersion estimate.
mod = NegativeBinomialP(y, X, p=2)
res_poi = sm.Poisson(y, X).fit(disp=0, skip_hessian=True, warn_convergence=False)
a = mod._estimate_dispersion(res_poi.predict(), res_poi.resid, df_resid=res_poi.df_resid)
out["negative_binomial_p"]["start_params"] = np.append(
    res_poi.params, max(0.05, a)).tolist()
out["negative_binomial_p"]["dispersion_estimate"] = float(a)

# Record every function and gradient evaluation in the scipy BFGS path. Matching counts alone
# cannot prove the intermediate evaluation points were identical; this ordered trace can.
bfgs_trace = []
nobs = float(len(y))


def traced_objective(params):
    value = float(-mod.loglike(params) / nobs)
    recorded = value if np.isfinite(value) else str(value)
    bfgs_trace.append({"kind": "f", "x": params.tolist(), "value": recorded})
    return value


def traced_gradient(params):
    value = np.asarray(-mod.score(params) / nobs)
    bfgs_trace.append({"kind": "g", "x": params.tolist(), "value": value.tolist()})
    return value


traced = fmin_bfgs(
    traced_objective,
    np.asarray(out["negative_binomial_p"]["start_params"]),
    fprime=traced_gradient,
    gtol=1e-5,
    norm=np.inf,
    maxiter=35,
    full_output=True,
    disp=False,
)
assert np.allclose(traced[0], nb.params, rtol=0, atol=1e-14)
assert traced[4] == nb.mle_retvals["fcalls"]
assert traced[5] == nb.mle_retvals["gcalls"]
out["negative_binomial_p"]["bfgs_trace"] = bfgs_trace

# A second dataset, closer to Poisson, so the dispersion path is pinned at two points.
mu2 = np.exp(0.8 + 0.25 * x1)
y2 = rng.poisson(mu2).astype(float)
X2 = sm.add_constant(x1.reshape(-1, 1))
pois2 = sm.GLM(y2, X2, family=sm.families.Poisson()).fit()
nb2 = NegativeBinomialP(y2, X2, p=2).fit(disp=0)
mod2 = NegativeBinomialP(y2, X2, p=2)
res_poi2 = sm.Poisson(y2, X2).fit(disp=0, skip_hessian=True, warn_convergence=False)
a2 = mod2._estimate_dispersion(
    res_poi2.predict(), res_poi2.resid, df_resid=res_poi2.df_resid
)
start2 = np.append(res_poi2.params, max(0.05, a2))
bfgs_trace2 = []


def traced_objective2(params):
    value = float(-mod2.loglike(params) / len(y2))
    recorded = value if np.isfinite(value) else str(value)
    bfgs_trace2.append({"kind": "f", "x": params.tolist(), "value": recorded})
    return value


def traced_gradient2(params):
    value = np.asarray(-mod2.score(params) / len(y2))
    bfgs_trace2.append({"kind": "g", "x": params.tolist(), "value": value.tolist()})
    return value


traced2 = fmin_bfgs(
    traced_objective2,
    start2,
    fprime=traced_gradient2,
    gtol=1e-5,
    norm=np.inf,
    maxiter=35,
    full_output=True,
    disp=False,
)
assert np.allclose(traced2[0], nb2.params, rtol=0, atol=1e-12)
assert traced2[4] == nb2.mle_retvals["fcalls"]
assert traced2[5] == nb2.mle_retvals["gcalls"]
out["second"] = {
    "y": y2.tolist(), "x": X2.tolist(),
    "poisson_params": pois2.params.tolist(),
    "poisson_bse": pois2.bse.tolist(),
    "poisson_deviance": float(pois2.deviance),
    "nb_params": nb2.params.tolist(),
    "nb_bse": nb2.bse.tolist(),
    "nb_llf": float(nb2.llf),
    "nb_fcalls": int(nb2.mle_retvals["fcalls"]),
    "nb_gcalls": int(nb2.mle_retvals["gcalls"]),
    "nb_bfgs_trace": bfgs_trace2,
}

with open("oracle/fixtures/glm.json", "w") as fh:
    json.dump(out, fh)

p = out["poisson_glm"]
print(f"Poisson GLM: {p['iterations']} IRLS iterations, deviance {p['deviance']:.6f}, "
      f"dev/df {p['dev_over_df']:.4f}")
print(f"  params {np.round(p['params'], 6).tolist()}  IRR x{p['irr']:.4f}")
q = out["negative_binomial_p"]
print(f"NegBinP: BFGS {q['fcalls']} fcalls / {q['gcalls']} gcalls, warnflag {q['warnflag']}, "
      f"max |grad| at stop {q['gopt_maxabs']:.3e}")
print(f"  start  {np.round(q['start_params'], 6).tolist()}")
print(f"  params {np.round(q['params'], 6).tolist()}  llf {q['llf']:.6f}")
print(f"second dataset: nb {out['second']['nb_fcalls']} fcalls / "
      f"{out['second']['nb_gcalls']} gcalls, params {np.round(out['second']['nb_params'], 6).tolist()}")
