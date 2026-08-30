use hirmos_causal_core::{
    check_dag, holm_adjust, uniformity_test, Dag, DagImplication, ImplicationDecision,
};
use nalgebra::DMatrix;
use serde::Deserialize;

#[derive(Deserialize)]
struct ImplicationFixture {
    x: usize,
    y: usize,
    given: Vec<usize>,
    p_value: f64,
    adjusted_p_value: f64,
    contradicted: bool,
}

#[derive(Deserialize)]
struct UniformityFixture {
    statistic: f64,
    p_value: f64,
}

#[derive(Deserialize)]
struct FalsificationFixture {
    given_violations: usize,
    n_tests: usize,
    given_fraction: f64,
    permutation_lmc_fractions: Vec<f64>,
    permutation_tpa_fractions: Vec<f64>,
    p_value_lmc: f64,
    p_value_tpa: f64,
    in_mec: usize,
    falsifiable: bool,
    falsified: bool,
}

#[derive(Deserialize)]
struct Fixture {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
    implications: Vec<ImplicationFixture>,
    uniformity: UniformityFixture,
    falsification: FalsificationFixture,
    data: Vec<Vec<f64>>,
}

#[test]
fn holm_is_monotone_in_sorted_p_value_order() {
    let raw = [0.01, 0.04, 0.03, 0.9];
    let adjusted = holm_adjust(&raw);
    assert_eq!(adjusted, vec![0.04, 0.09, 0.09, 0.9]);
    assert!(adjusted
        .iter()
        .zip(raw)
        .all(|(adjusted, raw)| *adjusted >= raw));
}

#[test]
fn ks_uniformity_matches_scipy_exact_method() {
    let result = uniformity_test(&[0.02, 0.17, 0.31, 0.44, 0.68, 0.91]).unwrap();
    assert!((result.statistic - 0.22666666666666668).abs() < 1e-15);
    assert!((result.p_value - 0.8567933683990124).abs() < 2e-13);
}

#[test]
fn complete_lalonde_contract_matches_octopus_dowhy_worker() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../oracle/fixtures/graph_falsification_lalonde.json"
    ))
    .unwrap();
    let data = DMatrix::from_fn(fixture.data.len(), fixture.nodes.len(), |row, column| {
        fixture.data[row][column]
    });
    let dag = Dag::new(fixture.nodes.len(), &fixture.edges);
    let implications: Vec<DagImplication> = fixture
        .implications
        .iter()
        .map(|item| DagImplication {
            x: item.x,
            y: item.y,
            given: item.given.clone(),
        })
        .collect();
    let result = check_dag(&dag, &data, &implications, 500, 20, 0.05).unwrap();

    for (actual, expected) in result.implications.iter().zip(&fixture.implications) {
        assert!((actual.p_value - expected.p_value).abs() < 2e-8);
        assert!((actual.adjusted_p_value - expected.adjusted_p_value).abs() < 2e-7);
        assert_eq!(
            actual.decision == ImplicationDecision::Contradicted,
            expected.contradicted
        );
        assert!(actual.adjusted_p_value >= actual.p_value);
    }
    let uniformity = result.uniformity.unwrap();
    assert!((uniformity.statistic - fixture.uniformity.statistic).abs() < 2e-8);
    assert!((uniformity.p_value - fixture.uniformity.p_value).abs() < 2e-8);

    let actual = result.falsification;
    let expected = fixture.falsification;
    assert_eq!(actual.given_lmc_violations, expected.given_violations);
    assert_eq!(actual.given_lmc_tests, expected.n_tests);
    assert!((actual.given_lmc_violation_fraction - expected.given_fraction).abs() < 1e-15);
    assert_eq!(
        actual.permutation_lmc_violation_fractions,
        expected.permutation_lmc_fractions
    );
    assert_eq!(
        actual.permutation_tpa_violation_fractions,
        expected.permutation_tpa_fractions
    );
    assert_eq!(actual.p_value_lmc, expected.p_value_lmc);
    assert_eq!(actual.p_value_tpa, expected.p_value_tpa);
    assert_eq!(
        actual.permutations_in_markov_equivalence_class,
        expected.in_mec
    );
    assert_eq!(actual.falsifiable, expected.falsifiable);
    assert_eq!(actual.falsified, expected.falsified);
}
