// Parity for tigramite's CausalEffects on stationary DAGs: the latent projection into a time
// series ADMG, and the optimal adjustment set of Runge (NeurIPS 2021).
use hirmos_causal_core::causal_effects::*;
use hirmos_causal_core::parcorr::TimeSeries;
use serde_json::Value;

fn nodes(v: &Value) -> Vec<Node> {
    let raw: Vec<Vec<i64>> = serde_json::from_value(v.clone()).unwrap();
    raw.iter().map(|p| (p[0] as usize, p[1] as i32)).collect()
}

fn graph(n: usize, stat_lag: usize, edges: &[(usize, usize, usize)]) -> StationaryGraph {
    let mut graph = StationaryGraph::new(n, stat_lag);
    for &(cause, effect, lag) in edges {
        graph.set(cause, effect, lag, mark("-->"));
        if lag == 0 && graph.get(effect, cause, 0) == EMPTY {
            graph.set(effect, cause, 0, mark("<--"));
        }
    }
    graph
}

#[test]
fn adjustment_choices_match_tigramite_tutorial_admg() {
    let stationary = graph(
        8,
        0,
        &[
            (0, 1, 0),
            (1, 2, 0),
            (1, 3, 0),
            (2, 3, 0),
            (5, 1, 0),
            (5, 2, 0),
            (5, 4, 0),
            (6, 3, 0),
            (7, 3, 0),
            (7, 4, 0),
        ],
    );
    let effects = CausalEffects::new(stationary, &[(0, 0), (1, 0)], &[(3, 0)], &[], &[(7, 0)]);

    assert_eq!(
        effects.get_optimal_set(),
        Some(vec![(4, 0), (5, 0), (6, 0)])
    );
    assert_eq!(
        effects.get_optimal_set_with_minimization(OptimalSetMinimization::CollidersOnly),
        Some(vec![(5, 0), (6, 0)])
    );
    assert_eq!(
        effects.get_optimal_set_with_minimization(OptimalSetMinimization::All),
        Some(vec![(5, 0)])
    );
    assert!(effects.is_valid_adjustment_set(&[(5, 0)]));
    assert!(!effects.is_valid_adjustment_set(&[]));
    assert_eq!(
        effects.resolve_adjustment_set(&AdjustmentSetSelection::Explicit(vec![])),
        Err(AdjustmentSetError::InvalidExplicitSet {
            problems: vec![ExplicitAdjustmentProblem::OpenNonCausalPath],
        })
    );
}

#[test]
fn invalid_explicit_set_names_time_indexed_query_members() {
    let stationary = graph(3, 1, &[(2, 0, 0), (0, 1, 1), (2, 1, 1)]);
    let effects = CausalEffects::new(stationary, &[(0, -1)], &[(1, 0)], &[], &[]);

    assert_eq!(
        effects.explicit_adjustment_problems(&[(0, 0)]),
        vec![
            ExplicitAdjustmentProblem::LaterTreatmentOccurrence((0, 0)),
            ExplicitAdjustmentProblem::OpenNonCausalPath,
        ]
    );
}

#[test]
fn causal_effects_matches() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/causal_effects.json")).unwrap();
    let marks: Vec<String> = serde_json::from_value(root["marks"].clone()).unwrap();
    let code_of = |e: Edge| -> usize {
        if e == EMPTY {
            0
        } else {
            let s = String::from_utf8(e.to_vec()).unwrap();
            marks.iter().position(|m| *m == s).expect("known edge mark")
        }
    };

    for case in root["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let n = case["n"].as_u64().unwrap() as usize;
        let stat_lag = case["stat_lag"].as_u64().unwrap() as usize;
        let mut g = StationaryGraph::new(n, stat_lag);
        let edges: Vec<Vec<i64>> = serde_json::from_value(case["edges"].clone()).unwrap();
        for e in &edges {
            let (i, j, lag) = (e[0] as usize, e[1] as usize, e[2] as usize);
            g.set(i, j, lag, mark("-->"));
            if lag == 0 && g.get(j, i, 0) == EMPTY {
                g.set(j, i, 0, mark("<--"));
            }
        }
        let hidden = case.get("hidden").map(nodes).unwrap_or_default();
        let ce = CausalEffects::new(
            g,
            &nodes(&case["X"]),
            &nodes(&case["Y"]),
            &nodes(&case["S"]),
            &hidden,
        );

        assert_eq!(
            ce.tau_max,
            case["tau_max"].as_u64().unwrap() as usize,
            "{name}: tau_max"
        );

        // The whole projected graph, cell by cell.
        let shape: Vec<usize> = serde_json::from_value(case["projected_shape"].clone()).unwrap();
        assert_eq!(
            shape,
            vec![n, n, ce.tau_max + 1, ce.tau_max + 1],
            "{name}: shape"
        );
        let want: Vec<usize> = serde_json::from_value(case["projected"].clone()).unwrap();
        let mut differing = 0;
        let t = ce.tau_max + 1;
        for i in 0..n {
            for j in 0..n {
                for ti in 0..t {
                    for tj in 0..t {
                        let idx = ((i * n + j) * t + ti) * t + tj;
                        if code_of(ce.graph.get(i, j, ti, tj)) != want[idx] {
                            differing += 1;
                        }
                    }
                }
            }
        }
        println!(
            "{name}: tau_max {}, projected graph {} cells, {differing} differing",
            ce.tau_max,
            want.len()
        );
        assert_eq!(differing, 0, "{name}: latent projection differs");

        assert_eq!(
            ce.no_causal_path,
            case["no_causal_path"].as_bool().unwrap(),
            "{name}: no_causal_path"
        );
        let got_med: Vec<Node> = ce.mediators.iter().copied().collect();
        assert_eq!(got_med, nodes(&case["mediators"]), "{name}: mediators");

        match ce.get_optimal_set() {
            Some(oset) => {
                let want_oset = nodes(&case["oset"]);
                println!("  mediators {got_med:?}");
                println!("  O-set {oset:?}");
                assert_eq!(oset, want_oset, "{name}: optimal set");
                let (parents, colliders, collider_parents, s_out) =
                    ce.optimal_set_parts().expect("identifiable");
                for (label, got, key) in [
                    ("parents", parents, "parents"),
                    ("colliders", colliders, "colliders"),
                    ("collider_parents", collider_parents, "collider_parents"),
                    ("S", s_out, "S_out"),
                ] {
                    let got: Vec<Node> = got.into_iter().collect();
                    assert_eq!(got, nodes(&case[key]), "{name}: {label}");
                }
            }
            None => assert!(case["oset"].is_null(), "{name}: expected an optimal set"),
        }

        // The total effect estimator, where the fixture carries data for it.
        if let Some(te) = case.get("total_effect") {
            let rows: Vec<Vec<f64>> = serde_json::from_value(case["data"].clone()).unwrap();
            let data = TimeSeries::new(rows);
            let iv: Vec<Vec<f64>> = serde_json::from_value(case["intervention"].clone()).unwrap();
            for (label, est) in [
                ("linear", Estimator::Linear),
                ("knn15", Estimator::KNeighbors { k: 15 }),
            ] {
                let model = ce.fit_total_effect(&data, est).expect("identifiable");
                assert_eq!(
                    model.n_obs,
                    case["n_obs"].as_u64().unwrap() as usize,
                    "{name}: sample count after the tau_max cut off"
                );
                // tigramite hands the adjustment set to the model as a list built from a
                // Python set, so its column order is arbitrary. Least squares and a
                // Euclidean nearest neighbour average are both invariant to that ordering,
                // so the members are compared and the predictions carry the rest.
                let mut got_adj: Vec<Node> = model.adjustment_set.clone();
                let mut want_adj = nodes(&case["fitted_adjustment_set"]);
                got_adj.sort();
                want_adj.sort();
                assert_eq!(got_adj, want_adj, "{name}: fitted adjustment set");
                let pred = model.predict_total_effect(&iv);
                let want: Vec<f64> = serde_json::from_value(te[label].clone()).unwrap();
                let dev = pred
                    .iter()
                    .zip(&want)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0f64, f64::max);
                let ate = pred[1] - pred[0];
                let want_ate = te[&format!("{label}_ate")].as_f64().unwrap();
                println!(
                    "  {label}: n_obs {}, prediction maxdev {dev:.3e}, ATE {ate:+.6} against {want_ate:+.6}",
                    model.n_obs
                );
                assert!(dev <= 1e-9, "{name}/{label}: prediction deviation {dev}");
                assert!((ate - want_ate).abs() <= 1e-9, "{name}/{label}: ATE");

                let collider_model = ce
                    .fit_total_effect_with_adjustment_set(
                        &data,
                        est,
                        None,
                        &AdjustmentSetSelection::CollidersMinimizedOptimal,
                    )
                    .expect("Tigramite-valid collider-minimized adjustment set");
                let collider_prediction = collider_model.predict_total_effect(&iv);
                let collider_deviation = collider_prediction
                    .iter()
                    .zip(&want)
                    .map(|(actual, expected)| (actual - expected).abs())
                    .fold(0.0f64, f64::max);
                assert!(
                    collider_deviation <= 1e-9,
                    "{name}/{label}/collider-minimized: prediction deviation {collider_deviation}"
                );
            }

            let minimized_oracle = match name {
                "mediated chain, 4 vars" => Some((
                    vec![(1, -1)],
                    [0.04530944771152285, 0.21821659481224817],
                    [0.13158401774622405, 0.12209228578989693],
                )),
                "dense, 5 vars, lag 3" => Some((
                    vec![(1, -1), (2, -3)],
                    [0.10641788854102334, 0.5331903819988829],
                    [0.16759843058980706, 0.45964687983785285],
                )),
                _ => None,
            };
            if let Some((adjustment_set, linear_oracle, knn_oracle)) = minimized_oracle {
                for (label, estimator, oracle) in [
                    ("linear", Estimator::Linear, linear_oracle),
                    ("knn15", Estimator::KNeighbors { k: 15 }, knn_oracle),
                ] {
                    for selection in [
                        AdjustmentSetSelection::MinimizedOptimal,
                        AdjustmentSetSelection::Explicit(adjustment_set.clone()),
                    ] {
                        let model = ce
                            .fit_total_effect_with_adjustment_set(
                                &data, estimator, None, &selection,
                            )
                            .expect("Tigramite-valid minimized adjustment set");
                        assert_eq!(model.adjustment_set, adjustment_set, "{name}/{label}");
                        let prediction = model.predict_total_effect(&iv);
                        let deviation = prediction
                            .iter()
                            .zip(oracle)
                            .map(|(actual, expected)| (actual - expected).abs())
                            .fold(0.0f64, f64::max);
                        assert!(
                            deviation <= 1e-9,
                            "{name}/{label}/{selection:?}: prediction deviation {deviation}"
                        );
                    }
                }
            }
        }

        // Non-empty S: Tigramite predicts the first-stage model across observational Z,
        // fits a fresh nested model of those predictions on observed S, then evaluates it
        // at the requested condition. Mixed pairs prove conditional_estimator is independent.
        if let Some(te) = case.get("conditional_total_effect") {
            let rows: Vec<Vec<f64>> = serde_json::from_value(case["data"].clone()).unwrap();
            let data = TimeSeries::new(rows);
            let iv: Vec<Vec<f64>> = serde_json::from_value(case["intervention"].clone()).unwrap();
            let conditions: Vec<Vec<f64>> =
                serde_json::from_value(case["conditions_data"].clone()).unwrap();
            let pairs = [
                ("linear_linear", Estimator::Linear, None),
                ("knn15_knn15", Estimator::KNeighbors { k: 15 }, None),
                (
                    "linear_knn15",
                    Estimator::Linear,
                    Some(Estimator::KNeighbors { k: 15 }),
                ),
                (
                    "knn15_linear",
                    Estimator::KNeighbors { k: 15 },
                    Some(Estimator::Linear),
                ),
            ];
            for (label, estimator, conditional_estimator) in pairs {
                let model = ce
                    .fit_total_effect_with_conditional_estimator(
                        &data,
                        estimator,
                        conditional_estimator,
                    )
                    .expect("identifiable conditional effect");
                assert_eq!(
                    model.n_obs,
                    case["n_obs"].as_u64().unwrap() as usize,
                    "{name}: conditional sample count"
                );
                let mut got_adj = model.adjustment_set.clone();
                let mut want_adj = nodes(&case["fitted_adjustment_set"]);
                got_adj.sort();
                want_adj.sort();
                assert_eq!(got_adj, want_adj, "{name}: conditional adjustment set");

                let prediction = model.predict_total_effect_with_conditions(&iv, &conditions);
                let want: Vec<f64> = serde_json::from_value(te[label].clone()).unwrap();
                let dev = prediction
                    .iter()
                    .zip(&want)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0f64, f64::max);
                let ate_low = prediction[1] - prediction[0];
                let ate_high = prediction[3] - prediction[2];
                let want_low = te[&format!("{label}_ate_s_neg_0_5")].as_f64().unwrap();
                let want_high = te[&format!("{label}_ate_s_0_75")].as_f64().unwrap();
                println!(
                    "  {label}: nested prediction maxdev {dev:.3e}, \
                     ATE(S=-0.5) {ate_low:+.6}, ATE(S=0.75) {ate_high:+.6}"
                );
                assert!(
                    dev <= 1e-9,
                    "{name}/{label}: conditional prediction deviation {dev}"
                );
                assert!(
                    (ate_low - want_low).abs() <= 1e-9,
                    "{name}/{label}: low-S ATE"
                );
                assert!(
                    (ate_high - want_high).abs() <= 1e-9,
                    "{name}/{label}: high-S ATE"
                );
            }
        }
    }
}
