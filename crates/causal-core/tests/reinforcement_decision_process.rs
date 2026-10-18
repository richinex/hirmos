use hirmos_causal_core::discrete_bn::{Cpd, Dag, DiscreteBn};
use hirmos_causal_core::reinforcement::decision_process::{
    bellman_operator, evaluate_finite_policy, finite_horizon, interventional_mdp_from_discrete_bn,
    q_learning, value_iteration, TabularMdp,
};
use serde_json::Value;
use std::collections::HashMap;

const TOLERANCE: f64 = 5e-13;
const CHECKPOINTS: [usize; 7] = [1, 2, 3, 10, 100, 1_000, 10_000];
const CASES: [&str; 4] = [
    "small",
    "forest",
    "small_transition_rewards",
    "random_dense_seed17",
];

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_decision_process.json")).unwrap()
}

fn floats(value: &Value) -> Vec<f64> {
    serde_json::from_value(value.clone()).unwrap()
}

fn integers(value: &Value) -> Vec<usize> {
    serde_json::from_value(value.clone()).unwrap()
}

fn flatten_3d(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|matrix| {
            matrix
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|row| floats(row))
        })
        .collect()
}

fn model(case: &Value) -> TabularMdp {
    let states = case["states"].as_u64().unwrap() as usize;
    let actions = case["actions"].as_u64().unwrap() as usize;
    let transitions = flatten_3d(&case["transitions"]);
    let discount = case["discount"].as_f64().unwrap();
    if case["reward_kind"] == "transition" {
        TabularMdp::with_transition_rewards(
            states,
            actions,
            transitions,
            flatten_3d(&case["rewards"]),
            discount,
        )
        .unwrap()
    } else {
        TabularMdp::new(
            states,
            actions,
            transitions,
            case["rewards"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(floats)
                .collect(),
            discount,
        )
        .unwrap()
    }
}

fn maximum_deviation(actual: &[f64], expected: &[f64]) -> f64 {
    assert_eq!(actual.len(), expected.len());
    actual
        .iter()
        .zip(expected)
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, f64::max)
}

#[test]
fn finite_horizon_matches_the_recorded_values_and_its_policy_evaluates_to_the_same_value() {
    let root = fixture();
    for name in CASES {
        let mdp = model(&root[name]);
        for case in root[name]["finite_horizon"].as_array().unwrap() {
            let terminal = floats(&case["terminal"]);
            let result =
                finite_horizon(&mdp, case["horizon"].as_u64().unwrap() as usize, &terminal)
                    .unwrap();
            let expected: Vec<Vec<usize>> = serde_json::from_value(case["policy"].clone()).unwrap();
            assert_eq!(result.policy, expected, "{name}");
            for (actual, expected) in result.values.iter().zip(case["values"].as_array().unwrap()) {
                assert!(
                    maximum_deviation(actual, &floats(expected)) < TOLERANCE,
                    "{name}"
                );
            }
            let evaluated = evaluate_finite_policy(&mdp, &result.policy, &terminal).unwrap();
            assert_eq!(evaluated, result.values);
        }
    }
}

#[test]
fn every_small_three_stage_policy_matches_explicit_trajectory_enumeration() {
    let mdp = model(&fixture()["small"]);
    let horizon = 3;
    let terminal = [-2.0, 3.0];
    let optimal = finite_horizon(&mdp, horizon, &terminal).unwrap();
    for code in 0..64 {
        let policy: Vec<Vec<usize>> = (0..horizon)
            .map(|t| (0..2).map(|s| (code >> (2 * t + s)) & 1).collect())
            .collect();
        let evaluated = evaluate_finite_policy(&mdp, &policy, &terminal).unwrap();
        for initial in 0..2 {
            let mut expected = 0.0;
            for path in 0..8 {
                let mut probability = 1.0;
                let mut reward = 0.0;
                let mut discount = 1.0;
                let mut state = initial;
                for (t, actions) in policy.iter().enumerate() {
                    let action = actions[state];
                    let next = (path >> t) & 1;
                    reward += discount * mdp.reward(state, action);
                    probability *= mdp.transition(action, state, next);
                    discount *= mdp.discount();
                    state = next;
                }
                expected += probability * (reward + discount * terminal[state]);
            }
            assert!((evaluated[0][initial] - expected).abs() < TOLERANCE);
            assert!(optimal.values[0][initial] + TOLERANCE >= expected);
        }
    }
}

#[test]
fn bellman_operator_matches_the_recorded_values() {
    let root = fixture();
    let mut worst = 0.0_f64;

    for name in CASES {
        let case = &root[name];
        let mdp = model(case);
        let result = bellman_operator(&mdp, &vec![0.0; mdp.states()]).unwrap();
        assert_eq!(result.policy, integers(&case["bellman_zero"]["policy"]));
        worst = worst.max(maximum_deviation(
            &result.value,
            &floats(&case["bellman_zero"]["value"]),
        ));
    }
    println!("Bellman-operator maximum deviation {worst:.3e}");
    assert!(worst <= TOLERANCE, "maximum deviation {worst}");
}

#[test]
fn value_iteration_matches_every_recorded_iteration() {
    let root = fixture();
    let mut worst = 0.0_f64;

    for name in CASES {
        let case = &root[name];
        let expected = &case["value_iteration"];
        let result = value_iteration(
            &model(case),
            expected["epsilon"].as_f64().unwrap(),
            1_000,
            None,
        )
        .unwrap();

        assert_eq!(result.policy, integers(&expected["policy"]));
        assert_eq!(
            result.iterations,
            expected["iterations"].as_u64().unwrap() as usize
        );
        assert_eq!(
            result.computed_max_iterations,
            expected["computed_max_iter"].as_u64().unwrap() as usize
        );
        worst = worst.max((result.threshold - expected["threshold"].as_f64().unwrap()).abs());
        worst = worst.max(maximum_deviation(
            &result.value,
            &floats(&expected["value"]),
        ));

        let expected_trace = expected["trace"].as_array().unwrap();
        assert_eq!(result.trace.len(), expected_trace.len());
        for (actual, oracle) in result.trace.iter().zip(expected_trace) {
            assert_eq!(
                actual.iteration,
                oracle["iteration"].as_u64().unwrap() as usize
            );
            assert_eq!(actual.policy, integers(&oracle["policy"]));
            worst = worst.max(maximum_deviation(&actual.value, &floats(&oracle["value"])));
            worst = worst.max((actual.variation - oracle["variation"].as_f64().unwrap()).abs());
        }
    }

    println!("value-iteration maximum deviation {worst:.3e}");
    assert!(worst <= TOLERANCE, "maximum deviation {worst}");
}

#[test]
fn q_learning_matches_the_seeded_update_trajectory() {
    let root = fixture();
    let mut worst = 0.0_f64;

    for name in CASES {
        let case = &root[name];
        let expected = &case["q_learning"];
        let result = q_learning(
            &model(case),
            expected["iterations"].as_u64().unwrap() as usize,
            expected["seed"].as_u64().unwrap() as u32,
            &CHECKPOINTS,
        )
        .unwrap();

        assert_eq!(
            result.initial_state,
            expected["initial_state"].as_u64().unwrap() as usize
        );
        assert_eq!(result.policy, integers(&expected["policy"]));
        worst = worst.max(maximum_deviation(&result.q, &flatten_2d(&expected["q"])));
        worst = worst.max(maximum_deviation(
            &result.value,
            &floats(&expected["value"]),
        ));
        worst = worst.max(maximum_deviation(
            &result.mean_discrepancy,
            &floats(&expected["mean_discrepancy"]),
        ));

        let expected_updates = expected["updates"].as_array().unwrap();
        assert_eq!(result.checkpoints.len(), expected_updates.len());
        for (actual, oracle) in result.checkpoints.iter().zip(expected_updates) {
            assert_eq!(
                actual.iteration,
                oracle["iteration"].as_u64().unwrap() as usize
            );
            assert_eq!(actual.state, oracle["state"].as_u64().unwrap() as usize);
            assert_eq!(actual.action, oracle["action"].as_u64().unwrap() as usize);
            assert_eq!(
                actual.next_state,
                oracle["next_state"].as_u64().unwrap() as usize
            );
            worst = worst.max((actual.reward - oracle["reward"].as_f64().unwrap()).abs());
            worst = worst.max((actual.delta - oracle["delta"].as_f64().unwrap()).abs());
            worst = worst.max((actual.update - oracle["update"].as_f64().unwrap()).abs());
            worst =
                worst.max((actual.updated_value - oracle["updated_value"].as_f64().unwrap()).abs());
            worst = worst.max(maximum_deviation(&actual.q, &flatten_2d(&oracle["q"])));
        }
    }

    println!("Q-learning maximum deviation {worst:.3e}");
    assert!(worst <= TOLERANCE, "maximum deviation {worst}");
}

fn flatten_2d(value: &Value) -> Vec<f64> {
    value.as_array().unwrap().iter().flat_map(floats).collect()
}

fn confounded_two_slice_model() -> DiscreteBn {
    let binary = vec!["0".to_string(), "1".to_string()];
    DiscreteBn::from_cpds(
        Dag {
            nodes: ["U", "S", "A", "S_next"].map(str::to_string).to_vec(),
            edges: [("U", "A"), ("U", "S_next"), ("A", "S_next")]
                .map(|(cause, effect)| (cause.to_string(), effect.to_string()))
                .to_vec(),
        },
        vec![
            Cpd {
                variable: "U".to_string(),
                states: binary.clone(),
                parents: vec![],
                parent_states: vec![],
                values: vec![0.5, 0.5],
            },
            Cpd {
                variable: "S".to_string(),
                states: binary.clone(),
                parents: vec![],
                parent_states: vec![],
                values: vec![0.5, 0.5],
            },
            Cpd {
                variable: "A".to_string(),
                states: binary.clone(),
                parents: vec!["U".to_string()],
                parent_states: vec![binary.clone()],
                // P(A=0|U=0), P(A=0|U=1), followed by A=1.
                values: vec![0.9, 0.1, 0.1, 0.9],
            },
            Cpd {
                variable: "S_next".to_string(),
                states: binary.clone(),
                parents: vec!["A".to_string(), "U".to_string()],
                parent_states: vec![binary.clone(), binary],
                // The action is harmful at either U value, but A=1 is logged
                // mostly for the high-outcome U=1 group.
                values: vec![0.6, 0.2, 0.8, 0.4, 0.4, 0.8, 0.2, 0.6],
            },
        ],
    )
    .unwrap()
}

#[test]
fn causal_transition_rows_use_interventions_not_logged_conditioning() {
    let network = confounded_two_slice_model();
    let causal = interventional_mdp_from_discrete_bn(
        &network,
        "S",
        "A",
        "S_next",
        vec![0.0, 0.0, 1.0, 1.0],
        0.9,
    )
    .unwrap();

    assert_eq!(causal.state_labels, vec!["0", "1"]);
    assert_eq!(causal.action_labels, vec!["0", "1"]);
    for state in 0..2 {
        assert!((causal.model.transition(0, state, 1) - 0.6).abs() < 1e-14);
        assert!((causal.model.transition(1, state, 1) - 0.4).abs() < 1e-14);
    }

    // Conditioning on the logged action reverses the comparison because U is
    // a common cause of both A and S_next.
    let mut observational = vec![0.0; 8];
    for action in 0..2 {
        for state in 0..2 {
            let evidence = HashMap::from([
                ("S".to_string(), state.to_string()),
                ("A".to_string(), action.to_string()),
            ]);
            let distribution = network
                .query_checked(&["S_next".to_string()], &evidence)
                .unwrap();
            for (assignment, probability) in
                distribution.states.iter().zip(distribution.probabilities)
            {
                let next_state = assignment[0].parse::<usize>().unwrap();
                observational[(action * 2 + state) * 2 + next_state] = probability;
            }
        }
    }
    assert!((observational[(0 * 2) * 2 + 1] - 0.44).abs() < 1e-14);
    assert!((observational[(1 * 2) * 2 + 1] - 0.56).abs() < 1e-14);

    let observational =
        TabularMdp::new(2, 2, observational, vec![0.0, 0.0, 1.0, 1.0], 0.9).unwrap();
    let causal_policy = value_iteration(&causal.model, 0.01, 1_000, None)
        .unwrap()
        .policy;
    let observational_policy = value_iteration(&observational, 0.01, 1_000, None)
        .unwrap()
        .policy;
    assert_eq!(causal_policy, vec![0, 0]);
    assert_eq!(observational_policy, vec![1, 1]);
}
