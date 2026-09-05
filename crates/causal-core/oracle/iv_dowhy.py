"""Freeze DoWhy v0.11's instrumental-variable identifier and estimator.

Run this against ``reference/dowhy`` at commit
e66ed36f1b37c94c015a53f46442805b2d3403f8. The first block replays DoWhy's own
``tests/causal_estimators/test_instrumental_variable_estimator.py`` at its seed and
configuration order, so the fixture records the estimate, the refusal or the data
snapshot that each configuration produced. The second block seeds the generic bootstrap
that DoWhy falls back to for confidence intervals, once per estimator branch.
"""

from __future__ import annotations

import itertools
import json
import random
from pathlib import Path

import numpy as np
import pandas as pd
import statsmodels
from statsmodels.sandbox.regression.gmm import IV2SLS

import dowhy.datasets
from dowhy import EstimandType, identify_effect_auto
from dowhy.causal_estimators.instrumental_variable_estimator import InstrumentalVariableEstimator
from dowhy.graph import build_graph_from_str


ROOT = Path(__file__).resolve().parents[1]
OUTPUT_PATH = ROOT / "oracle" / "fixtures" / "iv_dowhy.json"

DOWHY_COMMIT = "e66ed36f1b37c94c015a53f46442805b2d3403f8"
BETA = 10
ROWS = 100_000
ERROR_TOLERANCE = 0.4
SNAPSHOT_ROWS = 5
BOOTSTRAP_SEED = 0


def values(series) -> list[float]:
    return [float(value) for value in np.asarray(series, dtype=np.float64).reshape(-1)]


def snapshot(frame: pd.DataFrame) -> dict:
    numeric = frame.astype(np.float64)
    return {
        "columns": list(frame.columns),
        "head": [values(row) for row in numeric.head(SNAPSHOT_ROWS).to_numpy()],
        "tail": [values(row) for row in numeric.tail(SNAPSHOT_ROWS).to_numpy()],
        "column_sums": values(numeric.to_numpy().sum(axis=0)),
    }


def branch_of(frame: pd.DataFrame, instruments: list[str], treatments: list[str]) -> str:
    if len(instruments) == 1 and len(treatments) == 1:
        if len(np.unique(frame[instruments[0]])) <= 2:
            return "wald-ratio"
        return "covariance-ratio"
    return "two-stage-least-squares"


def two_stage_params(frame: pd.DataFrame, instruments: list[str], treatments: list[str]) -> list[float]:
    """The per-treatment coefficients the estimator sums; DoWhy does not retain them."""
    outcome = frame["y"].astype(np.float32)
    treatment = frame[treatments].astype(np.float32)
    return values(IV2SLS(outcome, treatment, frame[instruments]).fit().params)


def run_case(data: dict, confidence_intervals: bool) -> dict:
    frame = data["df"]
    estimand = identify_effect_auto(
        build_graph_from_str(data["gml_graph"]),
        observed_nodes=list(frame.columns),
        action_nodes=data["treatment_name"],
        outcome_nodes=data["outcome_name"],
        estimand_type=EstimandType.NONPARAMETRIC_ATE,
    )
    estimand.set_identifier_method("iv")
    instruments = list(estimand.instrumental_variables)
    record = {
        "true_ate": float(data["ate"]),
        "instruments": sorted(instruments),
        **snapshot(frame),
    }
    estimator = InstrumentalVariableEstimator(identified_estimand=estimand)
    try:
        estimator.fit(frame, effect_modifier_names=data["effect_modifier_names"])
    except ValueError as error:
        record["result"] = {"kind": "refusal", "message": str(error)}
        return record
    estimate = estimator.estimate_effect(
        frame,
        control_value=0,
        treatment_value=1,
        test_significance=False,
        evaluate_effect_strength=False,
        confidence_intervals=confidence_intervals,
        target_units="ate",
    )
    str(estimate)
    branch = branch_of(frame, instruments, data["treatment_name"])
    result = {
        "kind": "estimate",
        "estimator": branch,
        "value": float(estimate.value),
        "params": two_stage_params(frame, instruments, data["treatment_name"])
        if branch == "two-stage-least-squares"
        else [float(estimate.value)],
        "within_tolerance": bool(abs(estimate.value - data["ate"]) < abs(data["ate"]) * ERROR_TOLERANCE),
    }
    if confidence_intervals:
        # DoWhy's generic bootstrap uses sklearn.utils.resample without a random_state and
        # therefore consumes NumPy's global RandomState stream.
        np.random.seed(BOOTSTRAP_SEED)
        interval = estimate.get_confidence_intervals()
        result["confidence_interval"] = values(interval)
        result["standard_error"] = float(estimate.get_standard_error())
        result["bootstrap_estimates"] = values(estimator._bootstrap_estimates.estimates)
    record["result"] = result
    return record


def upstream_test() -> dict:
    """tests/causal_estimators/test_instrumental_variable_estimator.py, verbatim order."""
    random.seed(0)
    np.random.seed(0)
    args_dict = {
        "num_common_causes": [0, 1],
        "num_instruments": [1, 2],
        "num_effect_modifiers": [0],
        "num_treatments": [1, 2],
        "treatment_is_binary": [False, True],
        "outcome_is_binary": [False],
    }
    keys, options = zip(*args_dict.items())
    configs = [dict(zip(keys, chosen)) for chosen in itertools.product(*options)]
    # The upstream test mutates its first configuration after the sweep.
    configs.append({**configs[0], "num_instruments": 0})
    records = []
    for config in configs:
        data = dowhy.datasets.linear_dataset(
            beta=BETA,
            num_samples=ROWS,
            num_frontdoor_variables=0,
            treatment_is_category=False,
            **config,
        )
        expected = "estimate" if 0 < config["num_instruments"] and config["num_instruments"] >= config["num_treatments"] else "refusal"
        records.append({"config": config, "expected": expected, **run_case(data, confidence_intervals=False)})
    return {
        "seed": 0,
        "rows": ROWS,
        "beta": BETA,
        "error_tolerance": ERROR_TOLERANCE,
        "configs": records,
    }


def bootstrap_cases() -> list[dict]:
    cases = []

    np.random.seed(0)
    data = dowhy.datasets.linear_dataset(
        beta=BETA,
        num_samples=ROWS,
        num_common_causes=1,
        num_instruments=1,
        num_treatments=1,
        treatment_is_binary=True,
    )
    cases.append({"name": "wald-ratio", "generator": "linear_dataset", "seed": 0, **run_case(data, True)})

    np.random.seed(0)
    data = dowhy.datasets.simple_iv_dataset(beta=BETA, num_samples=ROWS, treatment_is_binary=False)
    cases.append({"name": "covariance-ratio", "generator": "simple_iv_dataset", "seed": 0, **run_case(data, True)})

    np.random.seed(0)
    data = dowhy.datasets.simple_iv_dataset(beta=BETA, num_samples=ROWS, treatment_is_binary=True)
    cases.append(
        {"name": "covariance-ratio-binary-treatment", "generator": "simple_iv_dataset", "seed": 0, **run_case(data, True)}
    )

    np.random.seed(0)
    data = dowhy.datasets.linear_dataset(
        beta=BETA,
        num_samples=ROWS,
        num_common_causes=1,
        num_instruments=2,
        num_treatments=2,
        treatment_is_binary=False,
    )
    cases.append({"name": "two-stage-least-squares", "generator": "linear_dataset", "seed": 0, **run_case(data, True)})
    return cases


def main() -> None:
    output = {
        "reference": {
            "dowhy_version": "0.11",
            "dowhy_commit": DOWHY_COMMIT,
            "numpy_version": np.__version__,
            "pandas_version": pd.__version__,
            "statsmodels_version": statsmodels.__version__,
            "bootstrap_seed": BOOTSTRAP_SEED,
            "bootstrap_simulations": InstrumentalVariableEstimator.DEFAULT_NUMBER_OF_SIMULATIONS_CI,
            "bootstrap_sample_fraction": InstrumentalVariableEstimator.DEFAULT_SAMPLE_SIZE_FRACTION,
            "confidence_level": InstrumentalVariableEstimator.DEFAULT_CONFIDENCE_LEVEL,
        },
        "upstream_test": upstream_test(),
        "bootstrap_cases": bootstrap_cases(),
    }
    OUTPUT_PATH.write_text(json.dumps(output, indent=2) + "\n")
    for record in output["upstream_test"]["configs"]:
        print(record["config"], record["expected"], record["result"].get("value", record["result"].get("message")))
    for case in output["bootstrap_cases"]:
        print(case["name"], case["result"]["value"], case["result"]["confidence_interval"], case["result"]["standard_error"])


if __name__ == "__main__":
    main()
