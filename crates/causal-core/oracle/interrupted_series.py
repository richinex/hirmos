# Golden fixtures for the interrupted time series of Lopez Bernal, Cummins and Gasparrini
# (International Journal of Epidemiology 2017): the Sicily smoking-ban data, the harmonic
# seasonal terms of tsModel::harmonic, the paper's quasi-Poisson models 3 and 4 with the
# standardised population as an offset, and the same designs fitted to the rate by OLS with
# Newey-West errors. The R values come from the paper's own code run under R 4.5.1; the
# statsmodels values are the fits the Rust port follows line by line.
# Run: uv run python oracle/interrupted_series.py
import csv
import json
from pathlib import Path

import numpy as np
import statsmodels.api as sm

here = Path(__file__).parent
rows = list(csv.DictReader(open(here / "sicily.csv")))
aces = np.array([float(r["aces"]) for r in rows])
time = np.array([float(r["time"]) for r in rows])
month = np.array([float(r["month"]) for r in rows])
smokban = np.array([float(r["smokban"]) for r in rows])
stdpop = np.array([float(r["stdpop"]) for r in rows])
rate = aces / stdpop * 1e5


def harmonic(x, nfreq, period):
    # tsModel::harmonic: outer(x, seq(1, nfreq) * 2 * pi / period), sines then cosines.
    k = np.arange(1, nfreq + 1) * 2 * np.pi / period
    m = np.outer(x, k)
    return np.column_stack([np.sin(m), np.cos(m)])


seasonal = harmonic(month, 2, 12)
# Model 3: intercept, time, smokban, harmonics, in the port's column order (the fit does not
# depend on it). The intervention starts at time 37; the paper centres the slope change on time
# 36, the last pre-intervention month.
x3 = np.column_stack([np.ones_like(time), time, smokban, seasonal])
x4 = np.column_stack([np.ones_like(time), time, smokban, (time - 36) * smokban, seasonal])
offset = np.log(stdpop)


def quasi(y, x):
    fit = sm.GLM(y, x, family=sm.families.Poisson(), offset=offset).fit(scale="X2")
    # The paper's plotted curves: predictions at the mean population (the standardised count),
    # the counterfactual with the ban's columns at zero, and the "same month" (June) prediction
    # that draws the deseasonalised trend.
    at_mean = lambda design: np.exp(design @ fit.params + np.log(stdpop.mean()))
    june = harmonic(np.full_like(month, 6.0), 2, 12)
    fixed = x.copy(); fixed[:, -4:] = june
    counter = x.copy(); counter[:, 2] = 0.0
    if x.shape[1] == 8: counter[:, 3] = 0.0
    counter_fixed = counter.copy(); counter_fixed[:, -4:] = june
    return {
        "standardised": at_mean(x).tolist(),
        "standardised_counterfactual": at_mean(counter).tolist(),
        "deseasonalised": at_mean(fixed).tolist(),
        "deseasonalised_counterfactual": at_mean(counter_fixed).tolist(),
        "params": fit.params.tolist(),
        "bse": fit.bse.tolist(),
        "pvalues": fit.pvalues.tolist(),
        "scale": float(fit.scale),
        "deviance": float(fit.deviance),
        "fittedvalues": fit.fittedvalues.tolist(),
        "resid_deviance": fit.resid_deviance.tolist(),
        "resid_pearson": fit.resid_pearson.tolist(),
        "iterations": int(fit.fit_history["iteration"]),
        "converged": bool(fit.converged),
        "df_resid": float(fit.df_resid),
    }


def hac(y, x, maxlags):
    fit = sm.OLS(y, x).fit(cov_type="HAC", cov_kwds={"maxlags": maxlags})
    return {
        "params": fit.params.tolist(),
        "bse": fit.bse.tolist(),
        "pvalues": fit.pvalues.tolist(),
        "conf_int": fit.conf_int().tolist(),
        "resid": fit.resid.tolist(),
        "rsquared": float(fit.rsquared),
    }


r = json.load(open(here.parent.parent.parent / "docs/2026-09-21-interrupted-series/reference/oracle.json"))


def r_model(m, names):
    # R lists an interaction after the main effects; the port keeps the slope change beside the step.
    by = lambda field: [m[field][name] for name in names]
    return {
        "coefficients": by("coefficients"),
        "standardErrors": by("standardErrors"),
        "dispersion": m["dispersion"],
        "deviance": m["deviance"],
        "rateRatios": by("rateRatios"),
        "rateRatioLower": by("rateRatioLower"),
        "rateRatioUpper": by("rateRatioUpper"),
        "fitted": m["fitted"],
        "devianceResiduals": m["devianceResiduals"],
    }


HARMONICS = [f"harmonic(month, 2, 12){i}" for i in range(1, 5)]
NAMES3 = ["(Intercept)", "time", "smokban", *HARMONICS]
NAMES4 = ["(Intercept)", "time", "smokban", "smokban:I(time - 36)", *HARMONICS]


out = {
    "aces": aces.tolist(),
    "time": time.tolist(),
    "month": month.tolist(),
    "smokban": smokban.tolist(),
    "stdpop": stdpop.tolist(),
    "rate": rate.tolist(),
    "intervention_row": 36,
    "harmonic": seasonal.tolist(),
    "model3": {"statsmodels": quasi(aces, x3), "r": r_model(r["model3"], NAMES3)},
    "model4": {"statsmodels": quasi(aces, x4), "r": r_model(r["model4"], NAMES4), "anova_f": r["anova34"]},
    "trend_per_year_model3": r["trendPerYearModel3"],
    "rate_model3_hac2": hac(rate, x3, 2),
    "rate_model4_hac2": hac(rate, x4, 2),
}
json.dump(out, open(here / "fixtures/interrupted_series.json", "w"), indent=1)
print("model3 smokban", out["model3"]["statsmodels"]["params"][2], "R", out["model3"]["r"]["coefficients"][2])
print("model3 scale", out["model3"]["statsmodels"]["scale"], "R", out["model3"]["r"]["dispersion"])
