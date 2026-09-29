use hirmos_causal_core::grf::{causal, forest, inference, rank, regression, sampling::Clusters, tree};
use nalgebra::DMatrix;

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn inference_and_ranking_boundaries() {
    let y = [0., 1., 2., 3.];
    let w = [0., 1., 0., 1.];
    let p = [0.5; 4];
    let tau = [1.; 4];
    let data = inference::Observations {
        y: &y,
        w: &w,
        y_hat: &y,
        w_hat: &p,
        tau: &tau,
    };
    let labels = [0, 1, 2, 3];
    assert!(inference::scores(&data, Some(&[1.])).is_err());
    let edge = inference::Observations {
        w_hat: &[0., 1., 0.5, 0.5],
        ..data
    };
    assert!(inference::scores(&edge, None).is_err());
    assert!(inference::tmle(&edge, inference::Target::All).is_err());
    let continuous = inference::Observations {
        w: &[0., 1., 2., 3.],
        ..data
    };
    assert!(inference::scores(&continuous, None).is_err());
    assert!(inference::tmle(&continuous, inference::Target::All).is_err());
    assert!(inference::aipw(
        &continuous,
        &[1.; 4],
        &labels,
        inference::Target::Treated,
        None
    )
    .is_err());
    assert!(inference::average_scores(&y, &[0.; 4], &labels).is_err());
    assert!(inference::average_scores(&y, &[1.; 4], &[0; 4]).is_err());
    assert!(inference::observation_weights(&labels, Some(&[1.; 4]), true).is_err());
    let ew = inference::observation_weights(&[0, 0, 0, 1], None, true).unwrap();
    assert!((ew[0] - 1. / 6.).abs() < 1e-15 && (ew[3] - 0.5).abs() < 1e-15);
    let duplicate = DMatrix::from_element(4, 2, 1.);
    assert!(inference::linear_summary(
        &duplicate,
        &y,
        &[1.; 4],
        &labels,
        inference::Covariance::Hc3,
        false
    )
    .is_err());
    let nan = DMatrix::from_element(4, 1, f64::NAN);
    assert!(inference::linear_summary(
        &nan,
        &y,
        &[1.; 4],
        &labels,
        inference::Covariance::Hc0,
        false
    )
    .is_err());
    let priorities = vec![vec![1.; 4]];
    for q in [
        vec![],
        vec![0., 1.],
        vec![0.5],
        vec![1., 0.5, 1.],
        vec![f64::NAN, 1.],
    ] {
        assert!(rank::fit(
            &y,
            &priorities,
            None,
            &labels,
            &q,
            rank::Target::Autoc,
            10,
            7
        )
        .is_err());
    }
    assert!(rank::fit(
        &y,
        &priorities,
        Some(&[0.; 4]),
        &labels,
        &[1.],
        rank::Target::Autoc,
        10,
        7
    )
    .is_err());
    assert!(rank::fit(
        &y,
        &priorities,
        None,
        &[0; 4],
        &[1.],
        rank::Target::Autoc,
        10,
        7
    )
    .is_err());
    for reps in [0, 1, 10] {
        let result = rank::fit(
            &y,
            &priorities,
            None,
            &labels,
            &[0.5, 1.],
            rank::Target::Autoc,
            reps,
            7,
        )
        .unwrap();
        assert_eq!(result.rules[0].estimate, 0.);
        assert_eq!(result.rules[0].standard_error, 0.);
        assert_eq!(result.rules[0].toc, vec![0., 0.]);
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn nuisance_boundaries_and_supplied_predictions() {
    let n = 40;
    let x = vec![(0..n).map(|i| i as f64).collect::<Vec<_>>()];
    let y: Vec<_> = (0..n).map(|i| (i % 2) as f64 + (i as f64).sin()).collect();
    let w: Vec<_> = (0..n).map(|i| (i % 2) as f64).collect();
    let clusters = Clusters::new(&(0..n).collect::<Vec<_>>(), 1).unwrap();
    let options = forest::Options {
        trees: 12,
        group_size: 2,
        sample_fraction: 0.5,
        seed: 7,
        seed_mode: forest::SeedMode::Indexed,
        batches: 1,
        tree: tree::Options {
            mtry: 1,
            min_node_size: 5,
            honesty: tree::Honesty::Enabled {
                fraction: 0.5,
                prune: true,
            },
            alpha: 0.05,
            imbalance_penalty: 0.,
        },
    };
    let model = causal::fit(
        &x,
        &y,
        &w,
        None,
        &clusters,
        &options,
        Some(&[0.]),
        Some(&[0.5]),
    )
    .unwrap();
    assert_eq!(model.outcome_hat, vec![0.; n]);
    assert_eq!(model.treatment_hat, vec![0.5; n]);
    assert!(causal::fit(&x, &y, &w, None, &clusters, &options, Some(&[0., 0.]), None).is_err());
    assert!(causal::fit(
        &x,
        &y,
        &w,
        None,
        &clusters,
        &options,
        Some(&[f64::NAN]),
        None
    )
    .is_err());
    assert!(causal::fit(
        &x,
        &y,
        &w,
        Some(&vec![-1.; n]),
        &clusters,
        &options,
        None,
        None
    )
    .is_err());
    assert!(causal::continuous_debiasing(
        &x,
        &w,
        &[],
        None,
        &clusters,
        7,
        1,
        forest::SeedMode::Indexed,
        50
    )
    .is_err());
    let mut cols = x.clone();
    cols.push(y.clone());
    assert!(regression::train(&cols, 0, Some(0), &clusters, &options).is_err());
    cols[1][0] = f64::NAN;
    assert!(regression::train(&cols, 1, None, &clusters, &options).is_err());
    assert!(model.forest.predict(&[], false, false).is_err());
    assert!(model.forest.predict(&[vec![1.; 3]], true, false).is_err());
    assert!(model.forest.split_frequencies(0, 4).is_err());
    assert!(model.forest.variable_importance(1, 4, f64::NAN).is_err());
}
