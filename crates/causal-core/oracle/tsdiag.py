# Golden fixtures for the time-series diagnostics port against statsmodels.
# Run from crates/causal-core with the transpile's Python environment.
import json

import numpy as np
from statsmodels.stats.diagnostic import acorr_ljungbox
from statsmodels.tsa.ar_model import AutoReg
from statsmodels.tsa.api import VAR
from statsmodels.tsa.stattools import acf, grangercausalitytests, pacf

rng = np.random.default_rng(41)
n = 120
a = np.zeros(n)
b = np.zeros(n)
for t in range(1, n):
    a[t] = 0.6 * a[t - 1] + rng.normal(0, 1)
    b[t] = 0.4 * b[t - 1] + 0.5 * a[t - 1] + rng.normal(0, 1)

out = {"a": a.tolist(), "b": b.tolist()}
max_lag = 8
out["acf"] = {
    "fft_true": acf(a, nlags=max_lag, fft=True).tolist(),
    "fft_false": acf(a, nlags=max_lag, fft=False).tolist(),
}
out["pacf_ywadjusted"] = pacf(a, nlags=max_lag).tolist()
out["pacf_ywmle"] = pacf(a, nlags=max_lag, method="ywm").tolist()
acf_with_interval = acf(a, nlags=max_lag, fft=False, alpha=0.05)
out["plot_limits"] = {
    "acf": (acf_with_interval[1][:, 1] - acf_with_interval[0]).tolist(),
    "pacf": [0.0] + [float(1.959963984540054 / np.sqrt(n))] * max_lag,
}

lb = acorr_ljungbox(a, lags=list(range(1, max_lag + 1)), return_df=True)
out["ljungbox"] = {"stat": lb["lb_stat"].tolist(), "pvalue": lb["lb_pvalue"].tolist()}
ar = AutoReg(a, lags=1, old_names=False).fit()
out["autoreg1"] = {"params": ar.params.tolist(), "pvalues": ar.pvalues.tolist(), "resid": ar.resid.tolist()}
data = np.column_stack([b, a])
gc = grangercausalitytests(data, maxlag=4)
out["granger_ssr_ftest"] = {
    str(lag): [float(gc[lag][0]["ssr_ftest"][0]), float(gc[lag][0]["ssr_ftest"][1])]
    for lag in range(1, 5)
}
endog = np.column_stack([a, b])
var_cases = []
for p in (1, 2, 3):
    result = VAR(endog).fit(maxlags=p, ic=None, trend="c")
    var_cases.append({
        "lags": p,
        "aic": float(result.aic),
        "bic": float(result.bic),
        "params": np.asarray(result.params).tolist(),
        "forecast": np.asarray(result.forecast(y=endog[-p:], steps=1)).tolist(),
    })
out["var_trend_c"] = var_cases
out["endog"] = endog.tolist()

with open("oracle/fixtures/tsdiag.json", "w") as fixture:
    json.dump(out, fixture)
