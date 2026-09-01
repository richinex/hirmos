# Golden fixtures for the synthetic control weights of 134.
# 134 solves the problem with SCS, a first order solver whose default tolerance is 1e-4, so
# the fixture also carries a high accuracy reference. The Rust port is an exact active set
# method and is checked against the reference, with the SCS gap recorded separately.
# Run: uv run python oracle/synthetic_control.py
import json
import csv

import numpy as np
import cvxpy as cp
from scipy.stats import norm, t as student_t


def fit(X, y, solver, **kw):
    w = cp.Variable(X.shape[1])
    problem = cp.Problem(cp.Minimize(cp.sum_squares(X @ w - y)), [cp.sum(w) == 1, w >= 0])
    loss = problem.solve(verbose=False, solver=solver, **kw)
    return np.asarray(w.value), float(loss)


def debiased_sc(case, folds=3):
    """The 134 cross-fitting procedure, using CLARABEL for its SC subproblems."""
    y_pre_co = np.asarray(case["y_pre_co"], dtype=float)
    y_pre_tr = np.asarray(case["y_pre_tr"], dtype=float)
    y_post_co = np.asarray(case["y_post_co"], dtype=float)
    y_post_tr = np.asarray(case["y_post_tr"], dtype=float)
    pre_periods = len(y_pre_co)
    post_periods = len(y_post_co)
    block_size = int(min(np.floor(pre_periods / folds), post_periods))
    blocks = np.split(np.arange(pre_periods)[-folds * block_size:], folds)
    fitted = []
    for held_out in blocks:
        training = np.setdiff1d(np.arange(pre_periods), held_out)
        weights, _ = fit(y_pre_co[training], y_pre_tr[training], cp.CLARABEL)
        bias = float(np.mean(y_pre_tr[held_out] - y_pre_co[held_out] @ weights))
        fold_att = float(np.mean(y_post_tr - y_post_co @ weights) - bias)
        fitted.append({
            "held_out": held_out.tolist(),
            "weights": weights.tolist(),
            "bias": bias,
            "att": fold_att,
        })
    fold_atts = np.asarray([fold["att"] for fold in fitted])
    att = float(np.mean(fold_atts))
    standard_error = float(
        np.sqrt(1 + folds * block_size / post_periods)
        * np.std(fold_atts, ddof=1)
        / np.sqrt(folds)
    )
    t_statistic = float(att / standard_error if standard_error > 1e-10 else 0.0)
    degrees_of_freedom = folds - 1
    p_value = float(2 * student_t.cdf(-abs(t_statistic), degrees_of_freedom))
    critical = float(student_t.ppf(0.975, degrees_of_freedom))
    return {
        "fold_count": folds,
        "block_size": block_size,
        "att": att,
        "standard_error": standard_error,
        "t_statistic": t_statistic,
        "degrees_of_freedom": degrees_of_freedom,
        "p_value": p_value,
        "confidence_interval": [
            att - critical * standard_error,
            att + critical * standard_error,
        ],
        "folds": fitted,
    }


def donor_mspe_summary(match_controls, match_treated, outcomes_controls, outcomes_treated, pre):
    weights, _ = fit(match_controls, match_treated, cp.CLARABEL)
    with np.errstate(over="ignore", divide="ignore", invalid="ignore"):
        gap = outcomes_treated - outcomes_controls @ weights
    pre_mspe = float(np.mean(np.square(gap[:pre])))
    post_mspe = float(np.mean(np.square(gap[pre:])))
    return {
        "weights": weights.tolist(),
        "gap": gap.tolist(),
        "pre_mspe": pre_mspe,
        "post_mspe": post_mspe,
        "mspe_ratio": float(post_mspe / pre_mspe) if pre_mspe != 0 else float("inf"),
    }


def california_donor_placebos():
    by_state = {}
    with open("reference/synthdid/data/california_prop99.csv", newline="") as source:
        for row in csv.DictReader(source, delimiter=";"):
            by_state.setdefault(row["State"], []).append(
                (int(row["Year"]), float(row["PacksPerCapita"]), int(row["treated"]))
            )
    for observations in by_state.values():
        observations.sort()
    treated_name = next(
        name for name, observations in by_state.items()
        if any(treatment == 1 for _, _, treatment in observations)
    )
    donor_names = sorted(name for name in by_state if name != treated_name)
    years = [year for year, _, _ in by_state[treated_name]]
    treatment_year = next(
        year for year, _, treatment in by_state[treated_name] if treatment == 1
    )
    pre_periods = years.index(treatment_year)
    treated_outcomes = np.asarray([value for _, value, _ in by_state[treated_name]])
    donor_outcomes = np.column_stack([
        [value for _, value, _ in by_state[name]] for name in donor_names
    ])
    matching_controls = donor_outcomes[:pre_periods]
    matching_treated = treated_outcomes[:pre_periods]
    treated = donor_mspe_summary(
        matching_controls, matching_treated, donor_outcomes, treated_outcomes, pre_periods
    )
    placebos = []
    for donor in range(len(donor_names)):
        swapped_matching_controls = matching_controls.copy()
        swapped_matching_controls[:, donor] = matching_treated
        swapped_outcomes = donor_outcomes.copy()
        swapped_outcomes[:, donor] = treated_outcomes
        summary = donor_mspe_summary(
            swapped_matching_controls,
            matching_controls[:, donor],
            swapped_outcomes,
            donor_outcomes[:, donor],
            pre_periods,
        )
        placebos.append({"donor": donor, "name": donor_names[donor], **summary})
    valid = [item for item in placebos if not np.isnan(item["mspe_ratio"])]
    extreme = 1 + sum(item["mspe_ratio"] >= treated["mspe_ratio"] for item in valid)
    alpha = 0.05
    pre_gap = np.asarray(treated["gap"][:pre_periods])
    rank = int(np.ceil((pre_periods + 1) * (1 - alpha)))
    conformal_q = float(np.sort(np.abs(pre_gap))[rank - 1]) if rank <= pre_periods else float("inf")
    sigma_pre = float(np.std(pre_gap, ddof=1))
    normal_half_width = float(norm.ppf(1 - alpha / 2) * sigma_pre)
    synthetic = treated_outcomes - np.asarray(treated["gap"])
    prediction_bands = {
        "alpha": alpha,
        "conformal_q": conformal_q,
        "conformal_intervals": np.column_stack([
            synthetic - conformal_q, synthetic + conformal_q
        ]).tolist(),
        "sigma_pre": sigma_pre,
        "parametric_half_width": normal_half_width,
        "parametric_intervals": np.column_stack([
            synthetic - normal_half_width, synthetic + normal_half_width
        ]).tolist(),
    }
    return {
        "reference": "Synth::generate_placebos donor-swap semantics; CLARABEL weight solver",
        "treated_name": treated_name,
        "donor_names": donor_names,
        "years": years,
        "treatment_year": treatment_year,
        "pre_periods": pre_periods,
        "matching_controls": matching_controls.tolist(),
        "matching_treated": matching_treated.tolist(),
        "outcome_controls": donor_outcomes.tolist(),
        "outcome_treated": treated_outcomes.tolist(),
        "treated": treated,
        "placebos": placebos,
        "n_valid_placebos": len(valid),
        "p_value": extreme / (len(valid) + 1),
        "prediction_bands": prediction_bands,
    }


rng = np.random.default_rng(3)
cases = []

# Case 1: an interior optimum with more pre periods than controls, and a treated series that
# really is a convex combination of three of them.
n_pre, n_post, n_co = 40, 12, 8
factors = rng.normal(size=(n_pre + n_post, 3))
loadings = rng.uniform(0.2, 1.5, size=(3, n_co))
panel = factors @ loadings + rng.normal(scale=0.15, size=(n_pre + n_post, n_co))
true_w = np.zeros(n_co)
true_w[[1, 4, 6]] = [0.5, 0.3, 0.2]
treated = panel @ true_w + rng.normal(scale=0.2, size=n_pre + n_post)
treated[n_pre:] += 2.5  # the intervention

cases.append({
    "name": "well posed, 8 controls",
    "y_pre_co": panel[:n_pre].tolist(), "y_pre_tr": treated[:n_pre].tolist(),
    "y_post_co": panel[n_pre:].tolist(), "y_post_tr": treated[n_pre:].tolist(),
})

# Case 2: more controls than pre periods, so X'X is rank deficient and many weight vectors
# attain the same loss.
n_pre2, n_co2 = 10, 16
panel2 = rng.normal(size=(n_pre2 + 6, n_co2))
treated2 = panel2 @ (np.ones(n_co2) / n_co2) + rng.normal(scale=0.3, size=n_pre2 + 6)
cases.append({
    "name": "rank deficient, 16 controls over 10 periods",
    "y_pre_co": panel2[:n_pre2].tolist(), "y_pre_tr": treated2[:n_pre2].tolist(),
    "y_post_co": panel2[n_pre2:].tolist(), "y_post_tr": treated2[n_pre2:].tolist(),
})

# Case 3: the treated series sits outside the convex hull, so the optimum is on a vertex.
n_pre3, n_co3 = 30, 5
panel3 = rng.normal(size=(n_pre3 + 8, n_co3))
treated3 = panel3[:, 0] * 3.0 + rng.normal(scale=0.1, size=n_pre3 + 8)
cases.append({
    "name": "outside the hull, vertex solution",
    "y_pre_co": panel3[:n_pre3].tolist(), "y_pre_tr": treated3[:n_pre3].tolist(),
    "y_post_co": panel3[n_pre3:].tolist(), "y_post_tr": treated3[n_pre3:].tolist(),
})

for case in cases:
    X = np.array(case["y_pre_co"], dtype=float)
    y = np.array(case["y_pre_tr"], dtype=float)
    w_ref, loss_ref = fit(X, y, cp.CLARABEL)
    w_scs, loss_scs = fit(X, y, cp.SCS)
    case["weights"] = w_ref.tolist()
    # The solver reports its own objective estimate, so the loss is also recomputed from the
    # returned weights for an exact comparison.
    case["loss"] = loss_ref
    case["loss_recomputed"] = float(np.sum((X @ w_ref - y) ** 2))
    case["scs_weights"] = w_scs.tolist()
    case["scs_loss"] = loss_scs
    case["scs_weight_gap"] = float(np.max(np.abs(w_ref - w_scs)))
    # SCS stops at a slightly infeasible point, which is why its loss can read lower.
    case["ref_violation"] = float(max(abs(w_ref.sum() - 1), -min(w_ref.min(), 0.0)))
    case["scs_violation"] = float(max(abs(w_scs.sum() - 1), -min(w_scs.min(), 0.0)))

    post_co = np.array(case["y_post_co"], dtype=float)
    post_tr = np.array(case["y_post_tr"], dtype=float)
    case["pre_gap"] = (y - X @ w_ref).tolist()
    case["post_gap"] = (post_tr - post_co @ w_ref).tolist()
    case["att"] = float(np.mean(post_tr - post_co @ w_ref))
    case["scs_att"] = float(np.mean(post_tr - post_co @ w_scs))

debiased_cases = [
    {"name": case["name"], **debiased_sc(case)}
    for case in (cases[0], cases[2])
]
donor_placebo = california_donor_placebos()

with open("oracle/fixtures/synthetic_control.json", "w") as fh:
    json.dump({
        "cases": cases,
        "debiased_cases": debiased_cases,
        "donor_placebo": donor_placebo,
    }, fh)

for c in cases:
    print(f"  {c['name']}")
    print(f"    loss {c['loss']:.8f} (SCS {c['scs_loss']:.8f}), "
          f"ATT {c['att']:+.6f} (SCS {c['scs_att']:+.6f})")
    print(f"    SCS weight gap from the reference: {c['scs_weight_gap']:.3e}, "
          f"SCS constraint violation {c['scs_violation']:.3e} "
          f"(reference {c['ref_violation']:.3e})")
    print(f"    weights: {np.round(c['weights'], 4).tolist()}")

for case in debiased_cases:
    print(
        f"  debiased {case['name']}: ATT {case['att']:+.8f}, "
        f"SE {case['standard_error']:.8f}, p={case['p_value']:.8f}"
    )

print(
    f"  California outcome-path donor placebos: treated MSPE ratio "
    f"{donor_placebo['treated']['mspe_ratio']:.8f}, "
    f"p={donor_placebo['p_value']:.8f} "
    f"({donor_placebo['n_valid_placebos']} donor refits)"
)
