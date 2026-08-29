// Parity for the synthetic control weights of 134. The reference is cvxpy with CLARABEL; the
// SCS answer 134 actually uses is carried alongside so its own gap is visible.
use hirmos_causal_core::synthetic_control::*;
use nalgebra::{DMatrix, DVector};
use serde_json::Value;

fn matrix(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |r, c| rows[r][c])
}

fn vector(v: &Value) -> DVector<f64> {
    let raw: Vec<f64> = serde_json::from_value(v.clone()).unwrap();
    DVector::from_row_slice(&raw)
}

#[test]
fn synthetic_control_matches() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/synthetic_control.json")).unwrap();
    for case in root["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (sc, effect) = synthetic_effect(
            &matrix(&case["y_pre_co"]),
            &vector(&case["y_pre_tr"]),
            &matrix(&case["y_post_co"]),
            &vector(&case["y_post_tr"]),
        );
        let want_w: Vec<f64> = serde_json::from_value(case["weights"].clone()).unwrap();
        let wdev = sc
            .weights
            .iter()
            .zip(&want_w)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max);
        // The loss is compared relatively: outside the convex hull it is large, so a weight
        // agreeing to 1e-9 still moves it by 1e-7 in absolute terms.
        let ldev = (sc.loss / case["loss_recomputed"].as_f64().unwrap() - 1.0).abs();
        let solver_ldev = (sc.loss / case["loss"].as_f64().unwrap() - 1.0).abs();
        let adev = (effect.att - case["att"].as_f64().unwrap()).abs();

        // The constraints must hold exactly, not merely to the solver's tolerance.
        let total: f64 = sc.weights.iter().sum();
        assert!(
            (total - 1.0).abs() <= 1e-12,
            "{name}: weights sum to {total}"
        );
        assert!(
            sc.weights.iter().all(|&v| v >= 0.0),
            "{name}: negative weight"
        );

        println!(
            "{name}: {} iterations, weights maxdev {wdev:.3e}, relative loss dev {ldev:.3e} \
             (against the solver's own objective {solver_ldev:.3e}), ATT dev {adev:.3e}",
            sc.iterations
        );
        println!(
            "    SCS's own gap from the same reference was {:.3e}, at a constraint violation of {:.3e}",
            case["scs_weight_gap"].as_f64().unwrap(),
            case["scs_violation"].as_f64().unwrap()
        );
        assert!(wdev <= 1e-7, "{name}: weight deviation {wdev}");
        assert!(ldev <= 1e-8, "{name}: relative loss deviation {ldev}");
        assert!(
            solver_ldev <= 1e-8,
            "{name}: solver objective deviation {solver_ldev}"
        );
        assert!(adev <= 1e-7, "{name}: ATT deviation {adev}");

        let want_pre: Vec<f64> = serde_json::from_value(case["pre_gap"].clone()).unwrap();
        let want_post: Vec<f64> = serde_json::from_value(case["post_gap"].clone()).unwrap();
        let gdev = effect
            .pre_gap
            .iter()
            .chain(&effect.post_gap)
            .zip(want_pre.iter().chain(&want_post))
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max);
        println!("    pre and post gaps maxdev {gdev:.3e}");
        assert!(gdev <= 1e-6, "{name}: gap deviation {gdev}");
    }
}

#[test]
fn no_feasible_point_beats_the_active_set_optimum() {
    // The active set method returns the exact optimum, so no feasible point may do better.
    // SCS can report a lower loss, but only by stopping short of feasibility, so its weights
    // are projected back onto the simplex before being compared.
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/synthetic_control.json")).unwrap();
    for case in root["cases"].as_array().unwrap() {
        let x = matrix(&case["y_pre_co"]);
        let y = vector(&case["y_pre_tr"]);
        let sc = fit_synthetic_control(&x, &y);

        let loss_of = |w: &[f64]| -> f64 {
            let r = &x * DVector::from_row_slice(w) - &y;
            r.iter().map(|v| v * v).sum()
        };
        let mut projected: Vec<f64> = serde_json::from_value(case["scs_weights"].clone()).unwrap();
        let raw_loss = loss_of(&projected);
        for v in projected.iter_mut() {
            *v = v.max(0.0);
        }
        let total: f64 = projected.iter().sum();
        for v in projected.iter_mut() {
            *v /= total;
        }
        println!(
            "{}: ours {:.10}, SCS as returned {:.10} (violation {:.3e}), SCS projected {:.10}",
            case["name"].as_str().unwrap(),
            sc.loss,
            raw_loss,
            case["scs_violation"].as_f64().unwrap(),
            loss_of(&projected)
        );
        assert!(
            sc.loss <= loss_of(&projected) + 1e-12,
            "a feasible point did better"
        );
        let reference: Vec<f64> = serde_json::from_value(case["weights"].clone()).unwrap();
        assert!(
            sc.loss <= loss_of(&reference) + 1e-12,
            "the reference point did better"
        );
    }
}
