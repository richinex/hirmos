"""Pyro oracle for notebook 114i's Gamma-Poisson NUTS model.

The original resource CSV is not checked into this repository, so this uses both a deterministic
48-month, 114i-shaped input and R's real 192-month Seatbelts count series. The model, float32
inputs, priors, normalization, NUTS defaults, 500 warmup steps, 1000 draws, and IRR
transformation are literal copies of ``fit_negbin_irr``. Fixed-point density checks are also
emitted in float64 so arithmetic can be separated from sampler/RNG differences.

Run: uv run python oracle/nuts_114i.py
"""

import hashlib
import io
import json
import urllib.request

import numpy as np
import pandas as pd
import pyro
import pyro.distributions as dist
import torch
from pyro.infer import MCMC, NUTS


rng = np.random.default_rng(114)
n = 48
treatment = (3.0 + 0.55 * rng.normal(size=n) + 0.25 * np.sin(np.arange(n) / 4)).astype(
    np.float32
)
confounder = (95.0 + 18.0 * rng.normal(size=n) + 8.0 * np.cos(np.arange(n) / 6)).astype(
    np.float32
)
t_norm_generation = (treatment - treatment.mean()) / treatment.std(ddof=1)
c_norm_generation = (confounder - confounder.mean()) / confounder.std(ddof=1)
mu = np.exp(3.35 + 0.32 * t_norm_generation + 0.24 * c_norm_generation)
true_r = 6.0
outcome = rng.negative_binomial(true_r, true_r / (true_r + mu)).astype(np.float32)


def normalized(dtype, treatment_values, confounder_values, outcome_values):
    treatment_tensor = torch.tensor(treatment_values, dtype=dtype)
    confounder_tensor = torch.tensor(confounder_values, dtype=dtype)
    return (
        treatment_tensor,
        confounder_tensor,
        (treatment_tensor - treatment_tensor.mean()) / treatment_tensor.std(),
        (confounder_tensor - confounder_tensor.mean()) / confounder_tensor.std(),
        torch.tensor(outcome_values, dtype=dtype),
    )


def model(treatment_normalized, confounder_normalized, observed=None):
    beta_0 = pyro.sample("beta_0", dist.Normal(3.5, 1.0))
    beta_t = pyro.sample("beta_treatment", dist.Normal(0.0, 1.0))
    beta_c = pyro.sample("beta_confounder", dist.Normal(0.0, 1.0))
    r = pyro.sample("r", dist.HalfNormal(10.0))
    log_mu = beta_0 + beta_t * treatment_normalized + beta_c * confounder_normalized
    mean = torch.exp(torch.clamp(log_mu, max=10.0))
    with pyro.plate("data", len(treatment_normalized)):
        pyro.sample("outcome", dist.GammaPoisson(r, r / mean), obs=observed)


def fixed_density(dtype, point, treatment_values, confounder_values, outcome_values):
    _, _, treatment_normalized, confounder_normalized, observed = normalized(
        dtype, treatment_values, confounder_values, outcome_values
    )
    parameters = torch.tensor(point, dtype=dtype, requires_grad=True)
    beta_0, beta_t, beta_c, log_r = parameters.unbind()
    r = log_r.exp()
    log_mu = beta_0 + beta_t * treatment_normalized + beta_c * confounder_normalized
    mean = torch.exp(torch.clamp(log_mu, max=10.0))
    log_prob = (
        dist.Normal(torch.as_tensor(3.5, dtype=dtype), torch.as_tensor(1.0, dtype=dtype))
        .log_prob(beta_0)
        + dist.Normal(torch.as_tensor(0.0, dtype=dtype), torch.as_tensor(1.0, dtype=dtype))
        .log_prob(beta_t)
        + dist.Normal(torch.as_tensor(0.0, dtype=dtype), torch.as_tensor(1.0, dtype=dtype))
        .log_prob(beta_c)
        + dist.HalfNormal(torch.as_tensor(10.0, dtype=dtype)).log_prob(r)
        + dist.GammaPoisson(r, r / mean).log_prob(observed).sum()
        + log_r
    )
    gradient = torch.autograd.grad(log_prob, parameters)[0]
    return {
        "parameters": point,
        "log_prob": float(log_prob.detach()),
        "gradient": [float(value) for value in gradient.detach()],
    }


def summary(values):
    return {
        "mean": float(values.mean()),
        "std": float(values.std()),
        "q025": float(values.quantile(0.025)),
        "median": float(values.median()),
        "q975": float(values.quantile(0.975)),
    }


def run_chains(treatment_values, confounder_values, outcome_values, label):
    chains = []
    for seed in [41, 42, 43]:
        pyro.clear_param_store()
        pyro.set_rng_seed(seed)
        treatment_tensor, _, treatment_normalized, confounder_normalized, observed = normalized(
            torch.float32, treatment_values, confounder_values, outcome_values
        )
        kernel = NUTS(model, adapt_step_size=True)
        mcmc = MCMC(kernel, num_samples=1000, warmup_steps=500, disable_progbar=True)
        mcmc.run(treatment_normalized, confounder_normalized, observed)
        samples = mcmc.get_samples()
        diagnostics = mcmc.diagnostics()
        divergent_iterations = diagnostics.get("divergences", {}).get("chain 0", [])
        irr = torch.exp(samples["beta_treatment"] / treatment_tensor.std())
        chain = {
            "seed": seed,
            "beta_0": summary(samples["beta_0"]),
            "beta_treatment": summary(samples["beta_treatment"]),
            "beta_confounder": summary(samples["beta_confounder"]),
            "r": summary(samples["r"]),
            "irr": summary(irr),
            "first_draws": [
                [
                    float(samples["beta_0"][index]),
                    float(samples["beta_treatment"][index]),
                    float(samples["beta_confounder"][index]),
                    float(samples["r"][index]),
                ]
                for index in range(3)
            ],
            "step_size": float(kernel.step_size),
            "inverse_mass_matrix": [
                float(value)
                for value in next(iter(kernel.inverse_mass_matrix.values())).reshape(-1)
            ],
            "divergences": len(divergent_iterations),
        }
        chains.append(chain)
        print(
            f"Pyro {label} seed {seed}: IRR {chain['irr']['median']:.6f} "
            f"[{chain['irr']['q025']:.6f}, {chain['irr']['q975']:.6f}], "
            f"step {chain['step_size']:.4g}, divergences {chain['divergences']}"
        )
    return chains


points = [
    [3.5, 0.0, 0.0, float(np.log(6.0))],
    [3.2, 0.3, -0.2, float(np.log(5.0))],
    [4.0, -0.4, 0.5, float(np.log(12.0))],
]
output = {
    "description": "deterministic 48-month input shaped like 114i; original resource data unavailable",
    "pyro_version": pyro.__version__,
    "torch_version": torch.__version__,
    "treatment": treatment.tolist(),
    "confounder": confounder.tolist(),
    "outcome": outcome.tolist(),
    "fixed_float64": [
        fixed_density(torch.float64, point, treatment, confounder, outcome) for point in points
    ],
    "fixed_float32": [
        fixed_density(torch.float32, point, treatment, confounder, outcome) for point in points
    ],
}
output["pyro_chains"] = run_chains(treatment, confounder, outcome, "shaped")

seatbelts_url = (
    "https://raw.githubusercontent.com/vincentarelbundock/Rdatasets/"
    "1dcc2bf5f955cc1224a3e1307256e1fe86b68dae/csv/datasets/Seatbelts.csv"
)
with urllib.request.urlopen(seatbelts_url) as response:
    seatbelts_bytes = response.read()
seatbelts_frame = pd.read_csv(io.BytesIO(seatbelts_bytes))
seatbelts_treatment = seatbelts_frame["law"].to_numpy(dtype=np.float32)
seatbelts_confounder = seatbelts_frame["kms"].to_numpy(dtype=np.float32)
seatbelts_outcome = seatbelts_frame["DriversKilled"].to_numpy(dtype=np.float32)
seatbelts_points = [
    [4.5, 0.0, 0.0, float(np.log(6.0))],
    [4.8, -0.2, 0.1, float(np.log(10.0))],
    [4.6, -0.4, 0.2, float(np.log(20.0))],
]
output["seatbelts"] = {
    "description": "R datasets::Seatbelts, monthly Great Britain road casualties, 1969-1984",
    "source_url": seatbelts_url,
    "source_sha256": hashlib.sha256(seatbelts_bytes).hexdigest(),
    "treatment_name": "law",
    "confounder_name": "kms",
    "outcome_name": "DriversKilled",
    "treatment": seatbelts_treatment.tolist(),
    "confounder": seatbelts_confounder.tolist(),
    "outcome": seatbelts_outcome.tolist(),
    "fixed_float64": [
        fixed_density(
            torch.float64,
            point,
            seatbelts_treatment,
            seatbelts_confounder,
            seatbelts_outcome,
        )
        for point in seatbelts_points
    ],
    "fixed_float32": [
        fixed_density(
            torch.float32,
            point,
            seatbelts_treatment,
            seatbelts_confounder,
            seatbelts_outcome,
        )
        for point in seatbelts_points
    ],
    "pyro_chains": run_chains(
        seatbelts_treatment, seatbelts_confounder, seatbelts_outcome, "Seatbelts"
    ),
}

with open("oracle/fixtures/nuts_114i.json", "w") as handle:
    json.dump(output, handle)
