# Golden fixtures for HAC OLS, WLS, Durbin-Watson. Run: uv run python oracle/estimation_fixtures.py
import json

import numpy as np
import statsmodels.api as sm

fixtures = {}
rng = np.random.default_rng(107)
n = 150
x = rng.normal(0, 1, (n, 3))
trend = np.cumsum(rng.normal(0, 0.3, n))
y = 2.0 + 0.8 * x[:, 0] - 0.5 * x[:, 1] + 0.0 * x[:, 2] + 0.4 * trend + rng.normal(0, 1, n)
X = sm.add_constant(x)

cases = []
for maxlags in (8, 16):
    r = sm.OLS(y, X).fit(cov_type="HAC", cov_kwds={"maxlags": maxlags})
    naive = sm.OLS(y, X).fit()
    cases.append({
        "maxlags": maxlags,
        "covariance": r.cov_params().tolist(),
        "params": r.params.tolist(),
        "bse": r.bse.tolist(),
        "pvalues": r.pvalues.tolist(),
        "conf_int": r.conf_int().tolist(),
        "dw": float(sm.stats.stattools.durbin_watson(naive.resid)),
        "rsquared": float(naive.rsquared),
    })
fixtures["hac"] = {"y": y.tolist(), "x": X.tolist(), "cases": cases}

w = rng.uniform(0.2, 2.0, n)
wr = sm.WLS(y, X, weights=w).fit()
fixtures["wls"] = {"weights": w.tolist(), "params": wr.params.tolist(), "pvalues": wr.pvalues.tolist()}

# Heteroskedastic errors on independent rows, and rows that repeat within clusters.
xh = rng.normal(0, 1, (n, 2))
yh = 1.0 + 0.6 * xh[:, 0] - 0.3 * xh[:, 1] + rng.normal(0, 1, n) * (0.5 + np.abs(xh[:, 0]))
Xh = sm.add_constant(xh)
hc1 = sm.OLS(yh, Xh).fit(cov_type="HC1")
fixtures["hc1"] = {"y": yh.tolist(), "x": Xh.tolist(), "params": hc1.params.tolist(), "bse": hc1.bse.tolist(),
                   "pvalues": hc1.pvalues.tolist(), "conf_int": hc1.conf_int().tolist()}

groups = np.repeat(np.arange(30), 5)
shocks = rng.normal(0, 0.8, 30)[groups]
xc = rng.normal(0, 1, (n, 2))
yc = 0.5 + 0.7 * xc[:, 0] + 0.2 * xc[:, 1] + shocks + rng.normal(0, 0.5, n)
Xc = sm.add_constant(xc)
# Labels that are not dense, as a unit column read from a file need not be.
labels = (groups * 7 + 3).tolist()
cluster = sm.OLS(yc, Xc).fit(cov_type="cluster", cov_kwds={"groups": groups})
fixtures["cluster"] = {"y": yc.tolist(), "x": Xc.tolist(), "groups": labels, "params": cluster.params.tolist(),
                       "bse": cluster.bse.tolist(), "pvalues": cluster.pvalues.tolist(), "conf_int": cluster.conf_int().tolist()}
print("hc1 bse:", [round(v, 4) for v in hc1.bse])
print("cluster bse:", [round(v, 4) for v in cluster.bse])
print("hac params:", [round(v, 4) for v in cases[0]["params"]])
print("wls params:", [round(v, 4) for v in wr.params])

for name, fit in [("wls", wr), ("hc1", hc1), ("cluster", cluster)]:
    fixtures[name]["covariance"] = fit.cov_params().tolist()

# Explicit reference choices, nonzero nulls and mixed-coefficient restrictions.
restrictions = []
for name, fit in [("wls", wr), ("hc1", hc1), ("cluster", cluster)]:
    p = len(fit.params)
    R = np.zeros((2, p))
    R[0, 1], R[0, 2], R[1, 0], R[1, 2] = 1, -0.5, 1, 0.25
    q = np.array([0.2, -0.1])
    for use_t in [False, True]:
        scalar = fit.t_test((R[:1], q[:1]), use_t=use_t)
        joint = fit.wald_test((R, q), use_f=use_t, scalar=True)
        restrictions.append(dict(name=name, params=fit.params.tolist(),
            covariance=fit.cov_params().tolist(), R=R.tolist(), q=q.tolist(),
            use_t=use_t, df=float(getattr(fit, "df_resid_inference", fit.df_resid)),
            estimate=float(scalar.effect.item()), se=float(scalar.sd.item()),
            statistic=float(scalar.tvalue.item()), p=float(scalar.pvalue.item()),
            interval=scalar.conf_int(alpha=0.1)[0].tolist(),
            joint_statistic=float(joint.statistic), joint_p=float(joint.pvalue)))
fixtures["restrictions"] = restrictions

with open("oracle/fixtures/estimation.json", "w") as f:
    json.dump(fixtures, f, indent=1)
print("wrote oracle/fixtures/estimation.json")
