use hirmos_causal_core::propensity::Normalization;
use hirmos_causal_core::reinforcement::sequential_evaluation::{
    plug_in_value, weighting_value, SequentialEvaluationError, Stage, Trajectory,
};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_sequential_evaluation.json")).unwrap()
}

fn two_stage(rows: &[[f64; 6]]) -> Vec<Trajectory> {
    rows.iter()
        .map(|row| Trajectory {
            stages: vec![
                Stage { state: row[0] as usize, action: row[1] as usize },
                Stage { state: row[2] as usize, action: row[3] as usize },
            ],
            outcome: row[4],
            weight: row[5],
        })
        .collect()
}

fn deterministic<'a>(rule: impl Fn(usize, &[usize]) -> usize + 'a) -> impl Fn(usize, &[usize], usize) -> f64 + 'a {
    move |stage, history, action| f64::from(u8::from(rule(stage, history) == action))
}

fn table_six() -> Vec<Trajectory> {
    let rows: Vec<[f64; 6]> = serde_json::from_value(fixture()["table"].clone()).unwrap();
    two_stage(&rows)
}

#[test]
fn both_estimators_match_the_recorded_estimates_on_the_two_stage_table() {
    let fixture = fixture();
    let trajectories = table_six();
    let policies: [(&str, Box<dyn Fn(usize, &[usize]) -> usize>); 3] = [
        ("x1_0_x2_1", Box::new(|stage, _: &[usize]| if stage == 0 { 0 } else { 1 })),
        ("x1_s1_x2_not_s2", Box::new(|stage, h: &[usize]| if stage == 0 { h[0] } else { 1 - h[2] })),
        ("x1_1_x2_x1", Box::new(|stage, h: &[usize]| if stage == 0 { 1 } else { h[1] })),
    ];
    for (name, rule) in policies {
        let policy = deterministic(rule);
        let weighting = weighting_value(&trajectories, 2, &policy, Normalization::HorvitzThompson).unwrap();
        let plug_in = plug_in_value(&trajectories, 2, &policy).unwrap().value;
        let expected = &fixture["estimates"][name];
        assert!((weighting - expected["ipw"].as_f64().unwrap()).abs() < 1e-12, "{name}: {weighting}");
        assert!((plug_in - expected["dp"].as_f64().unwrap()).abs() < 1e-12, "{name}: {plug_in}");
        assert!((weighting - plug_in).abs() < 1e-12, "{name}: weighting and plug-in differ");
        let hajek = weighting_value(&trajectories, 2, &policy, Normalization::Hajek).unwrap();
        assert!((hajek - weighting).abs() < 1e-12, "{name}: Hajek and Horvitz-Thompson differ");
    }
}

// P(S1, X1, X2, Y) with no second state; every second stage records state 0.
const TABLE_EIGHT: [[f64; 5]; 16] = [
    [0.0, 0.0, 0.0, 0.0, 0.2025], [0.0, 0.0, 0.0, 1.0, 0.0225], [0.0, 0.0, 1.0, 0.0, 0.0225], [0.0, 0.0, 1.0, 1.0, 0.2025],
    [0.0, 1.0, 0.0, 0.0, 0.2025], [0.0, 1.0, 0.0, 1.0, 0.0225], [0.0, 1.0, 1.0, 0.0, 0.0225], [0.0, 1.0, 1.0, 1.0, 0.2025],
    [1.0, 0.0, 0.0, 0.0, 0.0025], [1.0, 0.0, 0.0, 1.0, 0.0225], [1.0, 0.0, 1.0, 0.0, 0.0225], [1.0, 0.0, 1.0, 1.0, 0.0025],
    [1.0, 1.0, 0.0, 0.0, 0.0025], [1.0, 1.0, 0.0, 1.0, 0.0225], [1.0, 1.0, 1.0, 0.0, 0.0225], [1.0, 1.0, 1.0, 1.0, 0.0025],
];

#[test]
fn the_backdoor_admissible_example_is_worth_nine_tenths_by_both_estimators() {
    let rows: Vec<[f64; 6]> = TABLE_EIGHT.iter().map(|r| [r[0], r[1], 0.0, r[2], r[3], r[4]]).collect();
    let trajectories = two_stage(&rows);
    let policy = deterministic(|stage, h: &[usize]| if stage == 0 { h[0] } else { 1 - h[0] });
    let weighting = weighting_value(&trajectories, 2, &policy, Normalization::HorvitzThompson).unwrap();
    let plug_in = plug_in_value(&trajectories, 2, &policy).unwrap();
    assert!((weighting - 0.9).abs() < 1e-12);
    assert!((plug_in.value - 0.9).abs() < 1e-12);
    for s1 in 0..2usize {
        for x1 in 0..2usize {
            assert!((plug_in.action_values[0][&vec![s1, x1]] - 0.9).abs() < 1e-12);
            for x2 in 0..2usize {
                let expected = if x2 == 1 - s1 { 0.9 } else { 0.1 };
                assert!((plug_in.action_values[1][&vec![s1, x1, 0, x2]] - expected).abs() < 1e-12);
            }
        }
    }
}

#[test]
fn a_policy_outside_the_logged_support_is_refused() {
    let rows = [[0.0, 0.0, 0.0, 0.0, 1.0, 1.0], [0.0, 1.0, 0.0, 0.0, 0.0, 1.0]];
    let trajectories = two_stage(&rows);
    let policy = deterministic(|stage, _: &[usize]| if stage == 0 { 0 } else { 1 });
    let refusal = SequentialEvaluationError::OutsideSupport { stage: 1, history: vec![0, 0, 0], action: 1 };
    assert_eq!(weighting_value(&trajectories, 2, &policy, Normalization::HorvitzThompson).unwrap_err(), refusal);
    assert_eq!(plug_in_value(&trajectories, 2, &policy).unwrap_err(), refusal);
}
