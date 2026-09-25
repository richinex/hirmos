use hirmos_causal_core::gradient_boosting::{GradientBoostingClassifier, GradientBoostingError, Options};

const ROWS: usize = 900;
const COLUMNS: usize = 6;
const DECISION_TOLERANCE: f64 = 1e-12;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../oracle/fixtures/sklearn_gradient_boosting_kernel.json"))
        .unwrap()
}

fn numbers(value: &serde_json::Value) -> Vec<f64> {
    value.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
}

fn design() -> Vec<Vec<f64>> {
    (0..ROWS)
        .map(|row| {
            (0..COLUMNS)
                .map(|column| ((row * (column + 3) + column * column) % 37) as f64)
                .collect()
        })
        .collect()
}

fn labels(x: &[Vec<f64>]) -> Vec<f64> {
    x.iter()
        .map(|row| f64::from((row[0] * 2.0 + row[1] - row[3]).rem_euclid(13.0) > 6.0))
        .collect()
}

fn options(option: &serde_json::Value, seed: u32) -> Options {
    Options {
        n_estimators: option["n_estimators"].as_u64().unwrap() as usize,
        learning_rate: option["learning_rate"].as_f64().unwrap(),
        max_depth: option["max_depth"].as_u64().unwrap() as usize,
        min_samples_leaf: option["min_samples_leaf"].as_u64().unwrap() as usize,
        min_samples_split: 2,
        random_state: seed,
    }
}

#[test]
fn the_design_and_labels_match_the_oracle() {
    let oracle = fixture();
    let x = design();
    let y = labels(&x);
    assert_eq!(x.len(), oracle["rows"].as_u64().unwrap() as usize);
    assert_eq!(x[0].len(), oracle["columns"].as_u64().unwrap() as usize);
    assert_eq!(y.iter().sum::<f64>() as u64, oracle["positives"].as_u64().unwrap());
}

#[test]
fn the_port_reproduces_sklearn_at_every_setting() {
    let oracle = fixture();
    let seed = oracle["random_state"].as_u64().unwrap() as u32;
    let x = design();
    let y = labels(&x);
    for case in oracle["cases"].as_array().unwrap() {
        let model = GradientBoostingClassifier::fit(&x, &y, &options(&case["options"], seed))
            .expect("the kernel design fits");
        let decision = model.decision_function(&x).unwrap();
        let theirs = numbers(&case["decision"]);
        let worst = decision.iter().zip(&theirs)
            .map(|(a, b)| (a - b).abs()).fold(0f64, f64::max);
        let probability = model.predict_probability(&x).unwrap();
        let their_probability = numbers(&case["probability"]);
        let worst_probability = probability.iter().zip(&their_probability)
            .map(|(a, b)| (a - b).abs()).fold(0f64, f64::max);
        println!("{}: worst decision {worst:.3e}, worst probability {worst_probability:.3e}",
            case["options"]);
        assert!(worst < DECISION_TOLERANCE, "worst decision gap {worst:.3e}");
        assert!(worst_probability < DECISION_TOLERANCE,
            "worst probability gap {worst_probability:.3e}");
    }
}

/// A prefix of one fit is the shorter fit, which is what lets a grid over `n_estimators` be scored
/// without refitting. Checked against sklearn's own `staged_decision_function`.
#[test]
fn prefixes_of_one_fit_match_the_shorter_fits() {
    let oracle = fixture();
    let seed = oracle["random_state"].as_u64().unwrap() as u32;
    let x = design();
    let y = labels(&x);
    let staged = &oracle["staged"];
    let model = GradientBoostingClassifier::fit(&x, &y, &options(&staged["options"], seed))
        .expect("the kernel design fits");

    let mut stages: Vec<usize> = staged["stages"].as_object().unwrap()
        .keys().map(|k| k.parse().unwrap()).collect();
    stages.sort_unstable();
    let ours = model.staged_decision_function(&x, &stages).unwrap();
    for (stage, values) in stages.iter().zip(&ours) {
        let theirs = numbers(&staged["stages"][stage.to_string()]);
        let worst = values.iter().zip(&theirs)
            .map(|(a, b)| (a - b).abs()).fold(0f64, f64::max);
        println!("stage {stage}: worst {worst:.3e}");
        assert!(worst < DECISION_TOLERANCE, "stage {stage} gap {worst:.3e}");
    }

    let full = model.decision_function(&x).unwrap();
    let last = ours.last().unwrap();
    assert!(full.iter().zip(last).all(|(a, b)| a == b));
}

#[test]
fn the_classifier_rejects_input_it_cannot_use() {
    let x = design();
    let y = labels(&x);
    let settings = Options::default();

    assert_eq!(GradientBoostingClassifier::fit(&[], &[], &settings).unwrap_err(),
        GradientBoostingError::EmptySample);
    assert_eq!(GradientBoostingClassifier::fit(&x, &y[..ROWS - 1], &settings).unwrap_err(),
        GradientBoostingError::RowMismatch);
    assert_eq!(GradientBoostingClassifier::fit(&[vec![], vec![]], &[0., 1.], &settings).unwrap_err(),
        GradientBoostingError::NoColumns);

    let mut not_binary = y.clone();
    not_binary[7] = 0.5;
    assert_eq!(GradientBoostingClassifier::fit(&x, &not_binary, &settings).unwrap_err(),
        GradientBoostingError::NotBinary { row: 7 });

    assert_eq!(GradientBoostingClassifier::fit(&x, &vec![0.0; ROWS], &settings).unwrap_err(),
        GradientBoostingError::MissingArm { positive: true });

    let mut infinite = x.clone();
    infinite[2][2] = f64::NAN;
    assert_eq!(GradientBoostingClassifier::fit(&infinite, &y, &settings).unwrap_err(),
        GradientBoostingError::NonFiniteValue);

    assert_eq!(
        GradientBoostingClassifier::fit(&x, &y, &Options { n_estimators: 0, ..settings })
            .unwrap_err(),
        GradientBoostingError::Options);
}
