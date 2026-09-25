use hirmos_causal_core::model_selection::{candidates, grid_search, stratified_k_fold, Grid, SearchError};

const ROWS: usize = 400;
const COLUMNS: usize = 5;
const SCORE_TOLERANCE: f64 = 1e-12;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../oracle/fixtures/sklearn_grid_search_kernel.json"))
        .unwrap()
}

fn design() -> Vec<Vec<f64>> {
    (0..ROWS)
        .map(|row| {
            (0..COLUMNS)
                .map(|column| ((row * (column + 3) + column * column) % 29) as f64)
                .collect()
        })
        .collect()
}

fn labels(x: &[Vec<f64>]) -> Vec<f64> {
    x.iter()
        .map(|row| f64::from((row[0] * 2.0 + row[1] - row[3]).rem_euclid(11.0) > 5.0))
        .collect()
}

fn grid(oracle: &serde_json::Value) -> Grid {
    let axis = |name: &str| -> Vec<f64> {
        oracle["grid"][name].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
    };
    Grid {
        learning_rate: axis("learning_rate"),
        max_depth: axis("max_depth").into_iter().map(|v| v as usize).collect(),
        n_estimators: axis("n_estimators").into_iter().map(|v| v as usize).collect(),
    }
}

#[test]
fn stratified_folds_match_sklearn() {
    let oracle = fixture();
    let y = labels(&design());
    let ours = stratified_k_fold(&y, 5).expect("both classes are large enough");
    let theirs = oracle["folds"].as_array().unwrap();
    assert_eq!(ours.len(), theirs.len());
    for (fold, (mine, expected)) in ours.iter().zip(theirs).enumerate() {
        let want: Vec<usize> = expected.as_array().unwrap()
            .iter().map(|v| v.as_u64().unwrap() as usize).collect();
        assert_eq!(*mine, want, "fold {fold}");
    }
}

#[test]
fn candidate_order_matches_parameter_grid() {
    let oracle = fixture();
    let ours = candidates(&grid(&oracle));
    let theirs = oracle["candidate_order"].as_array().unwrap();
    assert_eq!(ours.len(), theirs.len());
    for (mine, expected) in ours.iter().zip(theirs) {
        assert_eq!(mine.learning_rate, expected["learning_rate"].as_f64().unwrap());
        assert_eq!(mine.max_depth, expected["max_depth"].as_u64().unwrap() as usize);
        assert_eq!(mine.n_estimators, expected["n_estimators"].as_u64().unwrap() as usize);
    }
}

#[test]
fn the_search_reproduces_sklearn_scores_and_selection() {
    let oracle = fixture();
    let x = design();
    let y = labels(&x);
    let result = grid_search(
        &x, &y, &grid(&oracle), 5,
        oracle["min_samples_leaf"].as_u64().unwrap() as usize, 2,
        oracle["random_state"].as_u64().unwrap() as u32,
    ).expect("the grid fits");

    let means: Vec<f64> = oracle["mean_test_score"].as_array().unwrap()
        .iter().map(|v| v.as_f64().unwrap()).collect();
    let worst = result.scores.iter().zip(&means)
        .map(|(ours, theirs)| (ours.mean_score - theirs).abs()).fold(0f64, f64::max);
    println!("worst mean score gap {worst:.3e}");
    assert!(worst < SCORE_TOLERANCE, "worst mean score gap {worst:.3e}");

    let splits = oracle["split_scores"].as_array().unwrap();
    let worst_fold = result.scores.iter().zip(splits)
        .flat_map(|(ours, theirs)| {
            ours.fold_scores.iter().zip(theirs.as_array().unwrap())
                .map(|(a, b)| (a - b.as_f64().unwrap()).abs()).collect::<Vec<_>>()
        })
        .fold(0f64, f64::max);
    println!("worst per-fold score gap {worst_fold:.3e}");
    assert!(worst_fold < SCORE_TOLERANCE, "worst per-fold gap {worst_fold:.3e}");

    let best = &oracle["best_params"];
    assert_eq!(result.best.learning_rate, best["learning_rate"].as_f64().unwrap());
    assert_eq!(result.best.max_depth, best["max_depth"].as_u64().unwrap() as usize);
    assert_eq!(result.best.n_estimators, best["n_estimators"].as_u64().unwrap() as usize);
    assert!((result.best_score - oracle["best_score"].as_f64().unwrap()).abs() < SCORE_TOLERANCE);
}

/// sklearn fits every candidate; the prefix form fits one model per (learning_rate, max_depth).
#[test]
fn prefix_scoring_cuts_the_fit_count() {
    let oracle = fixture();
    let x = design();
    let y = labels(&x);
    let selected = grid(&oracle);
    let result = grid_search(&x, &y, &selected, 5, 5, 2, 1234).expect("the grid fits");

    let sklearn_fits = selected.learning_rate.len() * selected.max_depth.len()
        * selected.n_estimators.len() * 5;
    println!("ours {} fits against sklearn's {sklearn_fits}", result.fits);
    assert_eq!(result.fits, selected.learning_rate.len() * selected.max_depth.len() * 5);
    assert!(result.fits < sklearn_fits);
}

#[test]
fn the_search_rejects_input_it_cannot_use() {
    let x = design();
    let y = labels(&x);
    let selected = Grid {
        learning_rate: vec![0.1],
        max_depth: vec![2],
        n_estimators: vec![5],
    };
    assert_eq!(grid_search(&[], &[], &selected, 5, 5, 2, 0).unwrap_err(), SearchError::EmptySample);
    assert_eq!(grid_search(&x, &y[..ROWS - 1], &selected, 5, 5, 2, 0).unwrap_err(),
        SearchError::RowMismatch);
    assert_eq!(
        grid_search(&x, &y, &Grid { learning_rate: vec![], ..selected.clone() }, 5, 5, 2, 0)
            .unwrap_err(),
        SearchError::EmptyGrid);
    assert_eq!(stratified_k_fold(&y, 1).unwrap_err(), SearchError::TooFewSplits);
    assert!(matches!(stratified_k_fold(&vec![0.0; 400], 5), Ok(_)));
}
