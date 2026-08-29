# Golden fixtures for the synthetic control weights of 134.
# 134 solves the problem with SCS, a first order solver whose default tolerance is 1e-4, so
# the fixture also carries a high accuracy reference. The Rust port is an exact active set
# method and is checked against the reference, with the SCS gap recorded separately.
# Run: uv run python oracle/synthetic_control.py
import json

import numpy as np
import cvxpy as cp


def fit(X, y, solver, **kw):
    w = cp.Variable(X.shape[1])
    problem = cp.Problem(cp.Minimize(cp.sum_squares(X @ w - y)), [cp.sum(w) == 1, w >= 0])
    loss = problem.solve(verbose=False, solver=solver, **kw)
    return np.asarray(w.value), float(loss)


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

with open("oracle/fixtures/synthetic_control.json", "w") as fh:
    json.dump({"cases": cases}, fh)

for c in cases:
    print(f"  {c['name']}")
    print(f"    loss {c['loss']:.8f} (SCS {c['scs_loss']:.8f}), "
          f"ATT {c['att']:+.6f} (SCS {c['scs_att']:+.6f})")
    print(f"    SCS weight gap from the reference: {c['scs_weight_gap']:.3e}, "
          f"SCS constraint violation {c['scs_violation']:.3e} "
          f"(reference {c['ref_violation']:.3e})")
    print(f"    weights: {np.round(c['weights'], 4).tolist()}")
