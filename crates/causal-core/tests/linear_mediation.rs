use hirmos_causal_core::causal_effects::BootstrapBlockLength;
use hirmos_causal_core::dynamic_counterfactual::{
    DynamicEquation, DynamicLinearScm, DynamicParent, DynamicParentSpec, InterventionTiming,
};
use hirmos_causal_core::linear_mediation::{
    LinearMediationBootstrap, LinearMediationBootstrapOptions, LinearMediationModel,
};
use hirmos_causal_core::nprandom::NpRng;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    data: Vec<Vec<f64>>,
    parents: Vec<Vec<(usize, usize)>>,
    treatment: usize,
    outcome: usize,
    interventions: (f64, f64),
    horizon: usize,
    phi: Vec<Vec<Vec<f64>>>,
    point: Vec<f64>,
    persistent: Vec<f64>,
    bootstrap: BootstrapFixture,
}

#[derive(Deserialize)]
struct BootstrapFixture {
    samples: usize,
    block_length: usize,
    seed: u64,
    confidence_level: f64,
    block_starts: Vec<Vec<usize>>,
    phi_draws: Vec<Vec<Vec<Vec<f64>>>>,
    point_draws: Vec<Vec<f64>>,
    persistent_draws: Vec<Vec<f64>>,
    point_interval: [Vec<f64>; 2],
    persistent_interval: [Vec<f64>; 2],
    point_average_draws: Vec<f64>,
    persistent_average_draws: Vec<f64>,
    point_cumulative_draws: Vec<f64>,
    persistent_cumulative_draws: Vec<f64>,
    point_average_interval: [f64; 2],
    persistent_average_interval: [f64; 2],
    point_cumulative_interval: [f64; 2],
    persistent_cumulative_interval: [f64; 2],
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!(
        "../oracle/fixtures/linear_mediation_counterfactual.json"
    ))
    .unwrap()
}

fn parents(raw: &[Vec<(usize, usize)>]) -> Vec<Vec<DynamicParentSpec>> {
    raw.iter()
        .map(|list| {
            list.iter()
                .map(|&(variable, lag)| DynamicParentSpec { variable, lag })
                .collect()
        })
        .collect()
}

fn maxdev(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max)
}

fn phi_maxdev(model: &LinearMediationModel, expected: &[Vec<Vec<f64>>]) -> f64 {
    model
        .phi
        .iter()
        .enumerate()
        .flat_map(|(lag, matrix)| {
            (0..matrix.nrows()).flat_map(move |child| {
                (0..matrix.ncols()).map(move |parent| {
                    (matrix[(child, parent)] - expected[lag][child][parent]).abs()
                })
            })
        })
        .fold(0.0, f64::max)
}

#[test]
fn linear_mediation_responses_match_tigramite_and_explicit_replay() {
    let expected = fixture();
    let parents = parents(&expected.parents);
    let model = LinearMediationModel::fit(&expected.data, &parents).unwrap();
    let point = model
        .contrast_response(
            expected.treatment,
            expected.outcome,
            InterventionTiming::Point { time: 20 },
            expected.horizon,
            expected.interventions.0,
            expected.interventions.1,
        )
        .unwrap();
    let persistent = model
        .contrast_response(
            expected.treatment,
            expected.outcome,
            InterventionTiming::Persistent { start: 20 },
            expected.horizon,
            expected.interventions.0,
            expected.interventions.1,
        )
        .unwrap();
    let point_replay = model
        .scm
        .counterfactual_pair(
            &expected.data,
            expected.treatment,
            expected.outcome,
            InterventionTiming::Point { time: 20 },
            expected.horizon,
            expected.interventions.0,
            expected.interventions.1,
        )
        .unwrap();
    let persistent_replay = model
        .scm
        .counterfactual_pair(
            &expected.data,
            expected.treatment,
            expected.outcome,
            InterventionTiming::Persistent { start: 20 },
            expected.horizon,
            expected.interventions.0,
            expected.interventions.1,
        )
        .unwrap();
    let phi_deviation = phi_maxdev(&model, &expected.phi);
    let response_deviation = maxdev(&point, &expected.point)
        .max(maxdev(&persistent, &expected.persistent))
        .max(maxdev(&point, &point_replay.contrast))
        .max(maxdev(&persistent, &persistent_replay.contrast));
    println!(
        "LinearMediation original phi {phi_deviation:.3e}, response/replay {response_deviation:.3e}"
    );
    assert!(phi_deviation <= 3e-14);
    assert!(response_deviation <= 3e-14);
}

#[test]
fn linear_mediation_bootstrap_matches_tigramite_draw_for_draw() {
    let expected = fixture();
    let parents = parents(&expected.parents);
    let bootstrap = LinearMediationBootstrap::fit(
        &expected.data,
        &parents,
        LinearMediationBootstrapOptions {
            samples: expected.bootstrap.samples,
            block_length: BootstrapBlockLength::Fixed(expected.bootstrap.block_length),
            seed: expected.bootstrap.seed,
        },
    )
    .unwrap();
    assert_eq!(bootstrap.block_starts, expected.bootstrap.block_starts);

    let mut phi_deviation: f64 = 0.0;
    let mut replay_deviation: f64 = 0.0;
    for (draw, model) in bootstrap.bootstrap_models.iter().enumerate() {
        phi_deviation = phi_deviation.max(phi_maxdev(model, &expected.bootstrap.phi_draws[draw]));
        for (timing, expected_path) in [
            (
                InterventionTiming::Point { time: 20 },
                &expected.bootstrap.point_draws[draw],
            ),
            (
                InterventionTiming::Persistent { start: 20 },
                &expected.bootstrap.persistent_draws[draw],
            ),
        ] {
            let response = model
                .contrast_response(
                    expected.treatment,
                    expected.outcome,
                    timing,
                    expected.horizon,
                    expected.interventions.0,
                    expected.interventions.1,
                )
                .unwrap();
            let replay = model
                .scm
                .counterfactual_pair(
                    &expected.data,
                    expected.treatment,
                    expected.outcome,
                    timing,
                    expected.horizon,
                    expected.interventions.0,
                    expected.interventions.1,
                )
                .unwrap();
            replay_deviation = replay_deviation
                .max(maxdev(&response, expected_path))
                .max(maxdev(&response, &replay.contrast));
        }
    }
    println!(
        "LinearMediation bootstrap phi {phi_deviation:.3e}, response/replay {replay_deviation:.3e}"
    );
    assert!(phi_deviation <= 5e-14);
    assert!(replay_deviation <= 5e-14);
}

#[test]
fn dynamic_counterfactual_intervals_match_tigramite_percentiles() {
    let expected = fixture();
    let parents = parents(&expected.parents);
    let bootstrap = LinearMediationBootstrap::fit(
        &expected.data,
        &parents,
        LinearMediationBootstrapOptions {
            samples: expected.bootstrap.samples,
            block_length: BootstrapBlockLength::Fixed(expected.bootstrap.block_length),
            seed: expected.bootstrap.seed,
        },
    )
    .unwrap();
    for (
        timing,
        expected_draws,
        expected_pointwise,
        expected_average_draws,
        expected_average_interval,
        expected_cumulative_draws,
        expected_cumulative_interval,
    ) in [
        (
            InterventionTiming::Point { time: 20 },
            &expected.bootstrap.point_draws,
            &expected.bootstrap.point_interval,
            &expected.bootstrap.point_average_draws,
            &expected.bootstrap.point_average_interval,
            &expected.bootstrap.point_cumulative_draws,
            &expected.bootstrap.point_cumulative_interval,
        ),
        (
            InterventionTiming::Persistent { start: 20 },
            &expected.bootstrap.persistent_draws,
            &expected.bootstrap.persistent_interval,
            &expected.bootstrap.persistent_average_draws,
            &expected.bootstrap.persistent_average_interval,
            &expected.bootstrap.persistent_cumulative_draws,
            &expected.bootstrap.persistent_cumulative_interval,
        ),
    ] {
        let uncertainty = bootstrap
            .counterfactual_uncertainty(
                expected.treatment,
                expected.outcome,
                timing,
                expected.horizon,
                expected.interventions.0,
                expected.interventions.1,
                expected.bootstrap.confidence_level,
            )
            .unwrap();
        let draw_deviation = uncertainty
            .effect_draws
            .iter()
            .zip(expected_draws)
            .map(|(actual, oracle)| maxdev(actual, oracle))
            .fold(0.0, f64::max);
        let interval_deviation = maxdev(&uncertainty.pointwise_interval[0], &expected_pointwise[0])
            .max(maxdev(
                &uncertainty.pointwise_interval[1],
                &expected_pointwise[1],
            ))
            .max(maxdev(&uncertainty.average_draws, expected_average_draws))
            .max(maxdev(
                &uncertainty.average_interval,
                expected_average_interval,
            ))
            .max(maxdev(
                &uncertainty.cumulative_draws,
                expected_cumulative_draws,
            ))
            .max(maxdev(
                &uncertainty.cumulative_interval,
                expected_cumulative_interval,
            ));
        println!(
            "dynamic bootstrap draws {draw_deviation:.3e}, intervals {interval_deviation:.3e}"
        );
        assert!(draw_deviation <= 5e-14);
        assert!(interval_deviation <= 5e-14);
    }
}

#[test]
fn block_bootstrap_has_reasonable_known_sem_coverage() {
    const REPLICATIONS: usize = 100;
    const BOOTSTRAPS: usize = 200;
    const ROWS: usize = 120;
    const HORIZON: usize = 6;
    const CONFIDENCE: f64 = 0.90;
    let parents = vec![
        vec![DynamicParentSpec {
            variable: 0,
            lag: 1,
        }],
        vec![
            DynamicParentSpec {
                variable: 1,
                lag: 1,
            },
            DynamicParentSpec {
                variable: 0,
                lag: 0,
            },
        ],
    ];
    let truth = LinearMediationModel::from_scm(
        DynamicLinearScm::from_equations(vec![
            DynamicEquation {
                intercept: 0.0,
                parents: vec![DynamicParent {
                    variable: 0,
                    lag: 1,
                    coefficient: 0.45,
                }],
                residual_scale: 1.0,
            },
            DynamicEquation {
                intercept: 0.0,
                parents: vec![
                    DynamicParent {
                        variable: 1,
                        lag: 1,
                        coefficient: 0.35,
                    },
                    DynamicParent {
                        variable: 0,
                        lag: 0,
                        coefficient: 0.70,
                    },
                ],
                residual_scale: 1.0,
            },
        ])
        .unwrap(),
    )
    .unwrap();
    let true_effect = truth
        .contrast_response(
            0,
            1,
            InterventionTiming::Point { time: 20 },
            HORIZON,
            0.0,
            1.0,
        )
        .unwrap();
    let mut covered = [0usize; HORIZON];
    for replication in 0..REPLICATIONS {
        let mut rng = NpRng::seeded(8000 + replication as u64);
        let mut data = vec![vec![0.0; ROWS]; 2];
        for time in 1..ROWS {
            data[0][time] = 0.45 * data[0][time - 1] + rng.standard_normal();
            data[1][time] =
                0.35 * data[1][time - 1] + 0.70 * data[0][time] + 0.8 * rng.standard_normal();
        }
        let bootstrap = LinearMediationBootstrap::fit(
            &data,
            &parents,
            LinearMediationBootstrapOptions {
                samples: BOOTSTRAPS,
                block_length: BootstrapBlockLength::CubeRoot,
                seed: 900_000 + replication as u64 * BOOTSTRAPS as u64,
            },
        )
        .unwrap();
        let uncertainty = bootstrap
            .counterfactual_uncertainty(
                0,
                1,
                InterventionTiming::Point { time: 20 },
                HORIZON,
                0.0,
                1.0,
                CONFIDENCE,
            )
            .unwrap();
        for step in 0..HORIZON {
            if uncertainty.pointwise_interval[0][step] <= true_effect[step]
                && true_effect[step] <= uncertainty.pointwise_interval[1][step]
            {
                covered[step] += 1;
            }
        }
    }
    let rates: Vec<f64> = covered
        .iter()
        .map(|&count| count as f64 / REPLICATIONS as f64)
        .collect();
    println!("known-SEM pointwise 90% coverage {rates:?}");
    assert!(rates.iter().all(|&rate| (0.78..=0.98).contains(&rate)));
}

#[test]
fn seatbelts_step_parent_does_not_abort_a_rank_deficient_bootstrap_draw() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../oracle/fixtures/nuts_114i.json")).unwrap();
    let seatbelts = &fixture["seatbelts"];
    let column = |name: &str| {
        seatbelts[name]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>()
    };
    let data = vec![column("outcome"), column("confounder"), column("treatment")];
    let parents = vec![
        vec![
            DynamicParentSpec {
                variable: 0,
                lag: 1,
            },
            DynamicParentSpec {
                variable: 1,
                lag: 0,
            },
            DynamicParentSpec {
                variable: 2,
                lag: 0,
            },
        ],
        Vec::new(),
        Vec::new(),
    ];
    let bootstrap = LinearMediationBootstrap::fit(
        &data,
        &parents,
        LinearMediationBootstrapOptions {
            samples: 24,
            block_length: BootstrapBlockLength::CubeRoot,
            seed: 0,
        },
    )
    .unwrap();
    let deficient: Vec<usize> = bootstrap
        .bootstrap_models
        .iter()
        .enumerate()
        .filter_map(|(draw, model)| {
            let diagnostics = model.scm.fit_diagnostics[0].as_ref().unwrap();
            (diagnostics.numerical_rank < diagnostics.predictors).then_some(draw)
        })
        .collect();
    assert_eq!(deficient, vec![23]);
    assert_eq!(
        bootstrap.bootstrap_models[23].scm.equations[0].parents[2].coefficient,
        0.0
    );
}
