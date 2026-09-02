use hirmos_causal_core::neural_granger::{
    clstm_initial_parameters, clstm_predict, clstm_proximal_update, cmlp_initial_parameters,
    cmlp_predict, cmlp_proximal_update, fit_clstm_from_parameters, fit_clstm_with, fit_cmlp,
    fit_cmlp_from_parameters, fit_cmlp_with, Activation, ClstmConfig, ClstmParameters, CmlpConfig,
    CmlpParameters, CmlpPenalty, Conv1dParameters, LstmNetworkParameters, NeuralGrangerError,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Fixture {
    source_revision: String,
    torch_version: String,
    cmlp: CmlpFixture,
    clstm: ClstmFixture,
    reference_cases: ReferenceCases,
}

#[derive(Deserialize)]
struct CmlpFixture {
    settings: CmlpSettings,
    data: Vec<f64>,
    initial_parameters: Vec<Parameter>,
    initial_prediction: Vec<f64>,
    activation_predictions: HashMap<String, Vec<f64>>,
    proximal_first_weight: ProximalWeights,
    loss: Vec<f64>,
    final_parameters: Vec<Parameter>,
    summary_scores: Vec<f64>,
    lag_scores: Vec<f64>,
}

#[derive(Deserialize)]
struct CmlpSettings {
    series: usize,
    lag: usize,
    hidden: Vec<usize>,
    lambda: f64,
    ridge_lambda: f64,
    learning_rate: f64,
    iterations: usize,
}

#[derive(Deserialize)]
struct ClstmFixture {
    settings: ClstmSettings,
    data: Vec<f64>,
    initial_parameters: Vec<Parameter>,
    initial_prediction: Vec<f64>,
    proximal_first_weight: Vec<f64>,
    loss: Vec<f64>,
    final_parameters: Vec<Parameter>,
    summary_scores: Vec<f64>,
}

#[derive(Deserialize)]
struct ClstmSettings {
    series: usize,
    context: usize,
    hidden: usize,
    lambda: f64,
    ridge_lambda: f64,
    learning_rate: f64,
    iterations: usize,
}

#[derive(Deserialize)]
struct ReferenceCases {
    lagged_var_cmlp: ReferenceCmlp,
    lorenz96_clstm: ReferenceClstm,
}

#[derive(Deserialize)]
struct ReferenceCmlp {
    settings: ReferenceCmlpSettings,
    data: Vec<f64>,
    initial_parameters: Vec<Parameter>,
    loss: Vec<f64>,
    final_parameters: Vec<Parameter>,
    summary_scores: Vec<f64>,
    lag_scores: Vec<f64>,
}

#[derive(Deserialize)]
struct ReferenceCmlpSettings {
    series: usize,
    lag: usize,
    hidden: Vec<usize>,
    lambda: f64,
    ridge_lambda: f64,
    learning_rate: f64,
    iterations: usize,
    check_every: usize,
}

#[derive(Deserialize)]
struct ReferenceClstm {
    settings: ReferenceClstmSettings,
    data: Vec<f64>,
    initial_parameters: Vec<Parameter>,
    loss: Vec<f64>,
    final_parameters: Vec<Parameter>,
    summary_scores: Vec<f64>,
}

#[derive(Deserialize)]
struct ReferenceClstmSettings {
    series: usize,
    context: usize,
    hidden: usize,
    lambda: f64,
    ridge_lambda: f64,
    learning_rate: f64,
    iterations: usize,
    check_every: usize,
}

#[derive(Deserialize)]
struct Parameter {
    name: String,
    shape: Vec<usize>,
    values: Vec<f32>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ProximalWeights {
    GL: Vec<f64>,
    GSGL: Vec<f64>,
    H: Vec<f64>,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../oracle/fixtures/neural_granger.json")).unwrap()
}

fn config(fixture: &CmlpFixture) -> CmlpConfig {
    CmlpConfig {
        lag: fixture.settings.lag,
        hidden: fixture.settings.hidden.clone(),
        activation: Activation::Relu,
        penalty: CmlpPenalty::Hierarchical,
        lambda: fixture.settings.lambda,
        ridge_lambda: fixture.settings.ridge_lambda,
        learning_rate: fixture.settings.learning_rate,
        max_iter: fixture.settings.iterations,
        check_every: 1,
        lookback: 100,
        seed: 0,
    }
}

fn cmlp_parameters(records: &[Parameter], series: usize, layers: usize) -> CmlpParameters {
    assert_eq!(records.len(), series * layers * 2);
    let mut networks = Vec::with_capacity(series);
    let mut position = 0;
    for output in 0..series {
        let mut network = Vec::with_capacity(layers);
        for layer in 0..layers {
            let weight = &records[position];
            let bias = &records[position + 1];
            assert_eq!(
                weight.name,
                format!("networks.{output}.layers.{layer}.weight")
            );
            assert_eq!(bias.name, format!("networks.{output}.layers.{layer}.bias"));
            network.push(Conv1dParameters {
                out_channels: weight.shape[0],
                in_channels: weight.shape[1],
                kernel_size: weight.shape[2],
                weight: weight.values.clone(),
                bias: bias.values.clone(),
            });
            position += 2;
        }
        networks.push(network);
    }
    CmlpParameters { networks }
}

fn assert_close(actual: &[f64], expected: &[f64], tolerance: f64) {
    assert_eq!(actual.len(), expected.len());
    let deviation = max_deviation(actual, expected);
    assert!(
        deviation <= tolerance,
        "maximum deviation {deviation:e} exceeds {tolerance:e}"
    );
}

fn max_deviation(actual: &[f64], expected: &[f64]) -> f64 {
    assert_eq!(actual.len(), expected.len());
    actual
        .iter()
        .zip(expected)
        .map(|(actual, expected)| (actual - expected).abs())
        .fold(0.0f64, f64::max)
}

fn flattened_parameters(parameters: &CmlpParameters) -> Vec<f64> {
    parameters
        .networks
        .iter()
        .flat_map(|network| network.iter())
        .flat_map(|layer| layer.weight.iter().chain(&layer.bias))
        .map(|value| *value as f64)
        .collect()
}

fn clstm_config(fixture: &ClstmFixture) -> ClstmConfig {
    ClstmConfig {
        context: fixture.settings.context,
        hidden: fixture.settings.hidden,
        lambda: fixture.settings.lambda,
        ridge_lambda: fixture.settings.ridge_lambda,
        learning_rate: fixture.settings.learning_rate,
        max_iter: fixture.settings.iterations,
        check_every: 1,
        lookback: 100,
        seed: 0,
    }
}

fn clstm_parameters(records: &[Parameter], series: usize, hidden: usize) -> ClstmParameters {
    assert_eq!(records.len(), series * 6);
    let mut networks = Vec::with_capacity(series);
    for output in 0..series {
        let offset = output * 6;
        let names = [
            "lstm.weight_ih_l0",
            "lstm.weight_hh_l0",
            "lstm.bias_ih_l0",
            "lstm.bias_hh_l0",
            "linear.weight",
            "linear.bias",
        ];
        for (record, suffix) in records[offset..offset + 6].iter().zip(names) {
            assert_eq!(record.name, format!("networks.{output}.{suffix}"));
        }
        networks.push(LstmNetworkParameters {
            weight_ih: records[offset].values.clone(),
            weight_hh: records[offset + 1].values.clone(),
            bias_ih: records[offset + 2].values.clone(),
            bias_hh: records[offset + 3].values.clone(),
            output_weight: records[offset + 4].values.clone(),
            output_bias: records[offset + 5].values[0],
        });
    }
    ClstmParameters {
        series,
        hidden,
        networks,
    }
}

fn flattened_clstm_parameters(parameters: &ClstmParameters) -> Vec<f64> {
    parameters
        .networks
        .iter()
        .flat_map(|network| {
            network
                .weight_ih
                .iter()
                .chain(&network.weight_hh)
                .chain(&network.bias_ih)
                .chain(&network.bias_hh)
                .chain(&network.output_weight)
                .copied()
                .chain(std::iter::once(network.output_bias))
        })
        .map(|value| value as f64)
        .collect()
}

#[test]
fn cmlp_forward_matches_neural_gc_torch_2_7() {
    let fixture = fixture();
    assert_eq!(
        fixture.source_revision,
        "c3263d40433aaf3acc94c27a0c1abf9b8e9fcedf"
    );
    assert_eq!(fixture.torch_version, "2.7.0");
    let config = config(&fixture.cmlp);
    let parameters = cmlp_parameters(
        &fixture.cmlp.initial_parameters,
        fixture.cmlp.settings.series,
        fixture.cmlp.settings.hidden.len() + 1,
    );
    let prediction = cmlp_predict(
        &fixture.cmlp.data,
        7,
        fixture.cmlp.settings.series,
        &config,
        &parameters,
    )
    .unwrap();
    assert_close(&prediction, &fixture.cmlp.initial_prediction, 2e-7);
}

#[test]
fn cmlp_activations_match_neural_gc() {
    let fixture = fixture();
    let parameters = cmlp_parameters(
        &fixture.cmlp.initial_parameters,
        fixture.cmlp.settings.series,
        fixture.cmlp.settings.hidden.len() + 1,
    );
    for (name, activation) in [
        ("sigmoid", Activation::Sigmoid),
        ("tanh", Activation::Tanh),
        ("relu", Activation::Relu),
        ("leakyrelu", Activation::LeakyRelu),
        ("identity", Activation::Identity),
    ] {
        let mut config = config(&fixture.cmlp);
        config.activation = activation;
        let prediction = cmlp_predict(
            &fixture.cmlp.data,
            7,
            fixture.cmlp.settings.series,
            &config,
            &parameters,
        )
        .unwrap();
        assert_close(
            &prediction,
            fixture.cmlp.activation_predictions.get(name).unwrap(),
            5e-7,
        );
    }
}

#[test]
fn cmlp_proximal_penalties_match_neural_gc() {
    let fixture = fixture();
    let initial = cmlp_parameters(
        &fixture.cmlp.initial_parameters,
        fixture.cmlp.settings.series,
        fixture.cmlp.settings.hidden.len() + 1,
    );
    for (penalty, expected) in [
        (
            CmlpPenalty::GroupLasso,
            &fixture.cmlp.proximal_first_weight.GL,
        ),
        (
            CmlpPenalty::GroupSparseGroupLasso,
            &fixture.cmlp.proximal_first_weight.GSGL,
        ),
        (
            CmlpPenalty::Hierarchical,
            &fixture.cmlp.proximal_first_weight.H,
        ),
    ] {
        let mut parameters = initial.clone();
        cmlp_proximal_update(
            &mut parameters,
            fixture.cmlp.settings.lambda,
            fixture.cmlp.settings.learning_rate,
            penalty,
        )
        .unwrap();
        let actual = parameters.networks[0].layers_first_weight();
        assert_close(&actual, expected, 3e-8);
    }
}

trait FirstWeight {
    fn layers_first_weight(&self) -> Vec<f64>;
}

impl FirstWeight for Vec<Conv1dParameters> {
    fn layers_first_weight(&self) -> Vec<f64> {
        self[0].weight.iter().map(|value| *value as f64).collect()
    }
}

#[test]
fn cmlp_ista_training_matches_neural_gc() {
    let fixture = fixture();
    let config = config(&fixture.cmlp);
    let initial = cmlp_parameters(
        &fixture.cmlp.initial_parameters,
        fixture.cmlp.settings.series,
        fixture.cmlp.settings.hidden.len() + 1,
    );
    let result = fit_cmlp_from_parameters(
        &fixture.cmlp.data,
        7,
        fixture.cmlp.settings.series,
        &config,
        &initial,
    )
    .unwrap();

    let expected_parameters = cmlp_parameters(
        &fixture.cmlp.final_parameters,
        fixture.cmlp.settings.series,
        fixture.cmlp.settings.hidden.len() + 1,
    );
    assert_close(&result.loss, &fixture.cmlp.loss, 2e-6);
    assert_close(&result.summary_scores, &fixture.cmlp.summary_scores, 2e-6);
    assert_close(&result.lag_scores, &fixture.cmlp.lag_scores, 2e-6);
    assert_eq!(
        result.summary_active,
        fixture
            .cmlp
            .summary_scores
            .iter()
            .map(|score| *score > 0.0)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result.lag_active,
        fixture
            .cmlp
            .lag_scores
            .iter()
            .map(|score| *score > 0.0)
            .collect::<Vec<_>>()
    );
    assert_eq!(result.lag_order, vec![2, 1]);
    assert_close(
        &flattened_parameters(&result.parameters),
        &flattened_parameters(&expected_parameters),
        2e-6,
    );
}

#[test]
fn progress_callbacks_report_reference_checkpoints() {
    let fixture = fixture();
    let cmlp_config = config(&fixture.cmlp);
    let cmlp_initial = cmlp_parameters(
        &fixture.cmlp.initial_parameters,
        fixture.cmlp.settings.series,
        fixture.cmlp.settings.hidden.len() + 1,
    );
    let mut cmlp_progress = Vec::new();
    fit_cmlp_with(
        &fixture.cmlp.data,
        7,
        2,
        &cmlp_config,
        Some(&cmlp_initial),
        |progress| cmlp_progress.push(progress),
    )
    .unwrap();
    assert_eq!(
        cmlp_progress
            .iter()
            .map(|progress| progress.iteration)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    assert_close(
        &cmlp_progress
            .iter()
            .map(|progress| progress.loss)
            .collect::<Vec<_>>(),
        &fixture.cmlp.loss,
        2e-6,
    );

    let clstm_config = clstm_config(&fixture.clstm);
    let clstm_initial = clstm_parameters(
        &fixture.clstm.initial_parameters,
        fixture.clstm.settings.series,
        fixture.clstm.settings.hidden,
    );
    let mut clstm_progress = Vec::new();
    fit_clstm_with(
        &fixture.clstm.data,
        7,
        2,
        &clstm_config,
        Some(&clstm_initial),
        |progress| clstm_progress.push(progress),
    )
    .unwrap();
    assert_eq!(
        clstm_progress
            .iter()
            .map(|progress| progress.iteration)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    assert_close(
        &clstm_progress
            .iter()
            .map(|progress| progress.loss)
            .collect::<Vec<_>>(),
        &fixture.clstm.loss,
        4e-6,
    );
}

#[test]
fn burn_initialization_is_seeded_and_reproducible() {
    let fixture = fixture();
    let mut cmlp_config = config(&fixture.cmlp);
    cmlp_config.seed = 805;
    let first = cmlp_initial_parameters(2, &cmlp_config).unwrap();
    let second = cmlp_initial_parameters(2, &cmlp_config).unwrap();
    assert_eq!(first, second);
    cmlp_config.seed = 806;
    assert_ne!(first, cmlp_initial_parameters(2, &cmlp_config).unwrap());

    let mut clstm_config = clstm_config(&fixture.clstm);
    clstm_config.seed = 805;
    let first = clstm_initial_parameters(2, &clstm_config).unwrap();
    let second = clstm_initial_parameters(2, &clstm_config).unwrap();
    assert_eq!(first, second);
    clstm_config.seed = 806;
    assert_ne!(first, clstm_initial_parameters(2, &clstm_config).unwrap());
}

#[test]
fn invalid_inputs_are_refused_before_training() {
    let fixture = fixture();
    let cmlp_config = config(&fixture.cmlp);
    let cmlp_initial = cmlp_parameters(
        &fixture.cmlp.initial_parameters,
        fixture.cmlp.settings.series,
        fixture.cmlp.settings.hidden.len() + 1,
    );
    assert_eq!(
        cmlp_predict(&fixture.cmlp.data, 8, 2, &cmlp_config, &cmlp_initial),
        Err(NeuralGrangerError::InvalidShape)
    );
    let mut invalid = fixture.cmlp.data.clone();
    invalid[0] = f64::NAN;
    assert_eq!(
        cmlp_predict(&invalid, 7, 2, &cmlp_config, &cmlp_initial),
        Err(NeuralGrangerError::NonFiniteData)
    );

    let mut clstm_config = clstm_config(&fixture.clstm);
    clstm_config.context = 7;
    let clstm_initial = clstm_parameters(
        &fixture.clstm.initial_parameters,
        fixture.clstm.settings.series,
        fixture.clstm.settings.hidden,
    );
    assert_eq!(
        clstm_predict(&fixture.clstm.data, 7, 2, &clstm_config, &clstm_initial),
        Err(NeuralGrangerError::InvalidLag)
    );

    let divergent = (0..16)
        .map(|index| if index % 2 == 0 { 1e100 } else { -1e100 })
        .collect::<Vec<_>>();
    let divergent_config = CmlpConfig {
        lag: 2,
        hidden: vec![2],
        activation: Activation::Relu,
        penalty: CmlpPenalty::Hierarchical,
        lambda: 0.005,
        ridge_lambda: 0.01,
        learning_rate: 0.01,
        max_iter: 1,
        check_every: 1,
        lookback: 1,
        seed: 0,
    };
    assert_eq!(
        fit_cmlp(&divergent, 8, 2, &divergent_config),
        Err(NeuralGrangerError::NonFiniteTraining)
    );
}

#[test]
fn clstm_forward_matches_neural_gc_torch_2_7() {
    let fixture = fixture();
    let config = clstm_config(&fixture.clstm);
    let parameters = clstm_parameters(
        &fixture.clstm.initial_parameters,
        fixture.clstm.settings.series,
        fixture.clstm.settings.hidden,
    );
    let prediction = clstm_predict(
        &fixture.clstm.data,
        7,
        fixture.clstm.settings.series,
        &config,
        &parameters,
    )
    .unwrap();
    assert_close(&prediction, &fixture.clstm.initial_prediction, 5e-7);
}

#[test]
fn clstm_proximal_update_matches_neural_gc() {
    let fixture = fixture();
    let mut parameters = clstm_parameters(
        &fixture.clstm.initial_parameters,
        fixture.clstm.settings.series,
        fixture.clstm.settings.hidden,
    );
    clstm_proximal_update(
        &mut parameters,
        fixture.clstm.settings.lambda,
        fixture.clstm.settings.learning_rate,
    )
    .unwrap();
    let weight = parameters.networks[0]
        .weight_ih
        .iter()
        .map(|value| *value as f64)
        .collect::<Vec<_>>();
    assert_close(&weight, &fixture.clstm.proximal_first_weight, 3e-8);
}

#[test]
fn clstm_ista_training_matches_neural_gc() {
    let fixture = fixture();
    let config = clstm_config(&fixture.clstm);
    let initial = clstm_parameters(
        &fixture.clstm.initial_parameters,
        fixture.clstm.settings.series,
        fixture.clstm.settings.hidden,
    );
    let result = fit_clstm_from_parameters(
        &fixture.clstm.data,
        7,
        fixture.clstm.settings.series,
        &config,
        &initial,
    )
    .unwrap();
    let expected_parameters = clstm_parameters(
        &fixture.clstm.final_parameters,
        fixture.clstm.settings.series,
        fixture.clstm.settings.hidden,
    );

    assert_close(&result.loss, &fixture.clstm.loss, 4e-6);
    assert_close(&result.summary_scores, &fixture.clstm.summary_scores, 4e-6);
    assert_eq!(
        result.summary_active,
        fixture
            .clstm
            .summary_scores
            .iter()
            .map(|score| *score > 0.0)
            .collect::<Vec<_>>()
    );
    assert_close(
        &flattened_clstm_parameters(&result.parameters),
        &flattened_clstm_parameters(&expected_parameters),
        4e-6,
    );
}

#[test]
fn cmlp_matches_neural_gc_lagged_var_example_shape() {
    let fixture = fixture();
    let case = &fixture.reference_cases.lagged_var_cmlp;
    let config = CmlpConfig {
        lag: case.settings.lag,
        hidden: case.settings.hidden.clone(),
        activation: Activation::Relu,
        penalty: CmlpPenalty::Hierarchical,
        lambda: case.settings.lambda,
        ridge_lambda: case.settings.ridge_lambda,
        learning_rate: case.settings.learning_rate,
        max_iter: case.settings.iterations,
        check_every: case.settings.check_every,
        lookback: 100,
        seed: 0,
    };
    let initial = cmlp_parameters(
        &case.initial_parameters,
        case.settings.series,
        case.settings.hidden.len() + 1,
    );
    let result =
        fit_cmlp_from_parameters(&case.data, 160, case.settings.series, &config, &initial).unwrap();
    let expected = cmlp_parameters(
        &case.final_parameters,
        case.settings.series,
        case.settings.hidden.len() + 1,
    );
    println!(
        "cMLP VAR deviations: loss={:.3e}, summary={:.3e}, lag={:.3e}, parameters={:.3e}",
        max_deviation(&result.loss, &case.loss),
        max_deviation(&result.summary_scores, &case.summary_scores),
        max_deviation(&result.lag_scores, &case.lag_scores),
        max_deviation(
            &flattened_parameters(&result.parameters),
            &flattened_parameters(&expected),
        ),
    );

    assert_close(&result.loss, &case.loss, 2e-5);
    assert_close(&result.summary_scores, &case.summary_scores, 2e-5);
    assert_close(&result.lag_scores, &case.lag_scores, 2e-5);
    assert_close(
        &flattened_parameters(&result.parameters),
        &flattened_parameters(&expected),
        2e-5,
    );
}

#[test]
fn clstm_matches_neural_gc_lorenz96_example_shape() {
    let fixture = fixture();
    let case = &fixture.reference_cases.lorenz96_clstm;
    let config = ClstmConfig {
        context: case.settings.context,
        hidden: case.settings.hidden,
        lambda: case.settings.lambda,
        ridge_lambda: case.settings.ridge_lambda,
        learning_rate: case.settings.learning_rate,
        max_iter: case.settings.iterations,
        check_every: case.settings.check_every,
        lookback: 100,
        seed: 0,
    };
    let initial = clstm_parameters(
        &case.initial_parameters,
        case.settings.series,
        case.settings.hidden,
    );
    let result =
        fit_clstm_from_parameters(&case.data, 120, case.settings.series, &config, &initial)
            .unwrap();
    let expected = clstm_parameters(
        &case.final_parameters,
        case.settings.series,
        case.settings.hidden,
    );
    println!(
        "cLSTM Lorenz-96 deviations: loss={:.3e}, summary={:.3e}, parameters={:.3e}",
        max_deviation(&result.loss, &case.loss),
        max_deviation(&result.summary_scores, &case.summary_scores),
        max_deviation(
            &flattened_clstm_parameters(&result.parameters),
            &flattened_clstm_parameters(&expected),
        ),
    );

    assert_close(&result.loss, &case.loss, 1e-4);
    assert_close(&result.summary_scores, &case.summary_scores, 1e-4);
    assert_close(
        &flattened_clstm_parameters(&result.parameters),
        &flattened_clstm_parameters(&expected),
        1e-4,
    );
}
