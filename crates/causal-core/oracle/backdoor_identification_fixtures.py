# Golden fixtures for backdoor identification with unobserved nodes (DoWhy).
# Run: ../../../octopus/rust-causal-transpile/.venv/bin/python oracle/backdoor_ols_fixtures.py
import json

import numpy as np
import pandas as pd
from dowhy import CausalModel

rng = np.random.default_rng(311)
n = 240


def identify(name, nodes, edges, observed, gen):
    data = gen()
    df = pd.DataFrame(data, columns=observed)
    dot = "digraph {" + "; ".join(f"{a} -> {b}" for a, b in edges) + "}"
    model = CausalModel(data=df, treatment="T", outcome="Y", graph=dot)
    ident = model.identify_effect(proceed_when_unidentifiable=True)
    backdoor = ident.get_backdoor_variables()
    identified = ident.estimands.get("backdoor") is not None
    print(name, "identified:", identified, "backdoor:", backdoor)
    return {
        "name": name, "nodes": nodes, "edges": edges, "observed": observed,
        "identified": identified, "backdoor": sorted(backdoor or []),
    }


identifications = []


def gen_latent():
    u = rng.normal(0, 1, n)
    t = 0.8 * u + rng.normal(0, 1, n)
    y = 0.5 * t + 0.7 * u + rng.normal(0, 1, n)
    return np.column_stack([t, y])


identifications.append(identify(
    "latent-confounder", ["T", "Y", "U"], [("U", "T"), ("U", "Y"), ("T", "Y")], ["T", "Y"], gen_latent))


def gen_latent_with_observed():
    u = rng.normal(0, 1, n)
    w = rng.normal(0, 1, n)
    t = 0.8 * u + 0.5 * w + rng.normal(0, 1, n)
    y = 0.5 * t + 0.7 * u + 0.4 * w + rng.normal(0, 1, n)
    return np.column_stack([t, y, w])


identifications.append(identify(
    "latent-and-observed", ["T", "Y", "U", "W"],
    [("U", "T"), ("U", "Y"), ("W", "T"), ("W", "Y"), ("T", "Y")], ["T", "Y", "W"], gen_latent_with_observed))


def gen_latent_blocked():
    u = rng.normal(0, 1, n)
    w = 0.9 * u + rng.normal(0, 1, n)
    t = 0.8 * w + rng.normal(0, 1, n)
    y = 0.5 * t + 0.7 * u + rng.normal(0, 1, n)
    return np.column_stack([t, y, w])


identifications.append(identify(
    "latent-blocked-by-observed", ["T", "Y", "U", "W"],
    [("U", "W"), ("W", "T"), ("U", "Y"), ("T", "Y")], ["T", "Y", "W"], gen_latent_blocked))


def gen_observed_confounder():
    w = rng.normal(0, 1, n)
    t = 0.8 * w + rng.normal(0, 1, n)
    y = 0.5 * t + 0.7 * w + rng.normal(0, 1, n)
    return np.column_stack([t, y, w])


identifications.append(identify(
    "observed-confounder", ["T", "Y", "W"], [("W", "T"), ("W", "Y"), ("T", "Y")], ["T", "Y", "W"], gen_observed_confounder))

with open("oracle/fixtures/backdoor_identification.json", "w") as f:
    json.dump({"identifications": identifications}, f, indent=1)
print("wrote oracle/fixtures/backdoor_identification.json")
