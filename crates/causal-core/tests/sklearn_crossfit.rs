use hirmos_causal_core::crossfit::{cross_fitted_propensity, cross_fitted_tlearner};
use hirmos_causal_core::model_selection::{Candidate, Grid, SearchError};

const ROWS: usize = 400;
const COLUMNS: usize = 5;
const SCORE_TOLERANCE: f64 = 1e-12;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../oracle/fixtures/sklearn_crossfit_kernel.json")).unwrap()
}

fn numbers(value: &serde_json::Value) -> Vec<f64> {
    value.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
}

fn rows_of(value: &serde_json::Value) -> Vec<usize> {
    value.as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect()
}

fn confounders() -> Vec<Vec<f64>> {
    (0..ROWS)
        .map(|row| {
            (0..COLUMNS)
                .map(|column| ((row * (column + 3) + column * column) % 29) as f64)
                .collect()
        })
        .collect()
}

fn treatment(c: &[Vec<f64>]) -> Vec<f64> {
    c.iter().map(|row| f64::from((row[0] + row[2]).rem_euclid(7.0) > 3.0)).collect()
}

fn death(c: &[Vec<f64>], t: &[f64]) -> Vec<f64> {
    c.iter()
        .zip(t)
        .map(|(row, &treated)| f64::from((row[1] * 2.0 + treated * 5.0).rem_euclid(11.0) > 4.0))
        .collect()
}

fn design(c: &[Vec<f64>], t: &[f64]) -> Vec<Vec<f64>> {
    c.iter()
        .zip(t)
        .map(|(row, &treated)| {
            let mut out = vec![treated];
            out.extend(row);
            out
        })
        .collect()
}

fn grid(oracle: &serde_json::Value) -> Grid {
    let axis = |name: &str| numbers(&oracle["grid"][name]);
    Grid {
        learning_rate: axis("learning_rate"),
        max_depth: axis("max_depth").into_iter().map(|v| v as usize).collect(),
        n_estimators: axis("n_estimators").into_iter().map(|v| v as usize).collect(),
    }
}

fn assert_selected(ours: &[Candidate], theirs: &serde_json::Value, label: &str) {
    let expected = theirs.as_array().unwrap();
    assert_eq!(ours.len(), expected.len(), "{label}: model count");
    for (index, (mine, want)) in ours.iter().zip(expected).enumerate() {
        assert_eq!(mine.learning_rate, want["learning_rate"].as_f64().unwrap(),
            "{label} model {index} learning_rate");
        assert_eq!(mine.max_depth, want["max_depth"].as_u64().unwrap() as usize,
            "{label} model {index} max_depth");
        assert_eq!(mine.n_estimators, want["n_estimators"].as_u64().unwrap() as usize,
            "{label} model {index} n_estimators");
    }
}

#[test]
fn the_data_matches_the_oracle() {
    let oracle = fixture();
    let c = confounders();
    let t = treatment(&c);
    let d = death(&c, &t);
    assert_eq!(t.iter().sum::<f64>() as u64, oracle["treated"].as_u64().unwrap());
    assert_eq!(d.iter().sum::<f64>() as u64, oracle["deaths"].as_u64().unwrap());
}

#[test]
fn cross_fitted_propensity_reproduces_the_oracle() {
    let oracle = fixture();
    let c = confounders();
    let t = treatment(&c);
    let seed = oracle["seed"].as_u64().unwrap() as u32;
    let leaf = oracle["min_samples_leaf"].as_u64().unwrap() as usize;

    let got = cross_fitted_propensity(&c, &t, &grid(&oracle), 5, leaf, 2, seed)
        .expect("both halves fit");
    let section = &oracle["propensity"];
    assert_eq!(got.rows, rows_of(&section["rows"]), "concatenated row order");
    assert_selected(&got.selected, &section["selected"], "propensity");

    let theirs = numbers(&section["scores"]);
    let worst = got.scores.iter().zip(&theirs)
        .map(|(a, b)| (a - b).abs()).fold(0f64, f64::max);
    println!("worst cross-fitted score gap {worst:.3e}");
    assert!(worst < SCORE_TOLERANCE, "worst score gap {worst:.3e}");

    let labels: Vec<u8> = got.rows.iter().map(|&row| t[row] as u8).collect();
    let auc = hirmos_causal_core::metrics::roc_auc(&labels, &got.scores);
    assert!((auc - section["auc"].as_f64().unwrap()).abs() < SCORE_TOLERANCE);
}

#[test]
fn the_cross_fitted_tlearner_reproduces_the_oracle() {
    let oracle = fixture();
    let c = confounders();
    let t = treatment(&c);
    let d = death(&c, &t);
    let x = design(&c, &t);
    let seed = oracle["seed"].as_u64().unwrap() as u32;
    let leaf = oracle["min_samples_leaf"].as_u64().unwrap() as usize;

    let got = cross_fitted_tlearner(&x, &t, &d, &grid(&oracle), 5, leaf, 2, seed)
        .expect("both halves fit both arms");
    let section = &oracle["tlearner"];
    assert_eq!(got.rows, rows_of(&section["rows"]), "concatenated row order");
    assert_selected(&got.selected, &section["selected"], "tlearner");

    let theirs = numbers(&section["effects"]);
    let worst = got.effects.iter().zip(&theirs)
        .map(|(a, b)| (a - b).abs()).fold(0f64, f64::max);
    let ate = section["ate"].as_f64().unwrap();
    println!("worst per-row effect gap {worst:.3e}, ate ours={:.16} oracle={ate:.16}", got.ate);
    assert!(worst < SCORE_TOLERANCE, "worst effect gap {worst:.3e}");
    assert!((got.ate - ate).abs() < SCORE_TOLERANCE);
}

#[test]
fn no_row_is_scored_by_a_model_that_saw_it() {
    let oracle = fixture();
    let c = confounders();
    let t = treatment(&c);
    let got = cross_fitted_propensity(&c, &t, &grid(&oracle), 5, 5, 2, 1234)
        .expect("both halves fit");
    let mut seen = got.rows.clone();
    seen.sort_unstable();
    assert_eq!(seen, (0..ROWS).collect::<Vec<_>>(), "every row is scored exactly once");
    assert_eq!(got.selected.len(), 2);
}

#[test]
fn the_estimators_reject_input_they_cannot_use() {
    let oracle = fixture();
    let c = confounders();
    let t = treatment(&c);
    let d = death(&c, &t);
    let selected = grid(&oracle);

    assert_eq!(cross_fitted_propensity(&[], &[], &selected, 5, 5, 2, 0).unwrap_err(),
        SearchError::EmptySample);
    assert_eq!(cross_fitted_propensity(&c, &t[..ROWS - 1], &selected, 5, 5, 2, 0).unwrap_err(),
        SearchError::RowMismatch);
    assert_eq!(
        cross_fitted_tlearner(&c, &t, &d[..ROWS - 1], &selected, 5, 5, 2, 0).unwrap_err(),
        SearchError::RowMismatch);
}
