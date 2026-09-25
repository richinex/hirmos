use hirmos_causal_core::gcm::boosting::{classifier::Classifier, regressor::{Options, Stopping}};
use hirmos_causal_core::nprandom::Mt19937;
use nalgebra::DMatrix;

const ROWS: usize = 900;
const COLUMNS: usize = 6;
const DECISION_TOLERANCE: f64 = 1e-12;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../oracle/fixtures/sklearn_hist_classifier_kernel.json"))
        .unwrap()
}

fn numbers(value: &serde_json::Value) -> Vec<f64> {
    value.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
}

fn design() -> DMatrix<f64> {
    DMatrix::from_fn(ROWS, COLUMNS, |row, column| {
        ((row * (column + 3) + column * column) % 37) as f64
    })
}

fn labels(x: &DMatrix<f64>) -> Vec<f64> {
    (0..ROWS)
        .map(|row| {
            let value = x[(row, 0)] * 2.0 + x[(row, 1)] - x[(row, 3)];
            f64::from(value.rem_euclid(13.0) > 6.0)
        })
        .collect()
}

fn options(case: &serde_json::Value) -> Options {
    let option = &case["options"];
    Options {
        iterations: (option["max_iter"].as_u64().unwrap() as usize).try_into().unwrap(),
        leaves: (option["max_leaf_nodes"].as_u64().unwrap() as usize).try_into().unwrap(),
        min_leaf: (option["min_samples_leaf"].as_u64().unwrap() as usize).try_into().unwrap(),
        bins: 255,
        learning_rate: option["learning_rate"].as_f64().unwrap(),
        l2: 0.0,
        stopping: Stopping::Disabled,
    }
}

#[test]
fn the_design_and_labels_match_the_oracle() {
    let oracle = fixture();
    let x = design();
    let y = labels(&x);
    assert_eq!(x.nrows(), oracle["rows"].as_u64().unwrap() as usize);
    assert_eq!(x.ncols(), oracle["columns"].as_u64().unwrap() as usize);
    assert_eq!(y.iter().sum::<f64>() as u64, oracle["positives"].as_u64().unwrap());
}

#[test]
fn the_baseline_is_the_log_odds_of_the_label_mean() {
    let oracle = fixture();
    let x = design();
    let y = labels(&x);
    let theirs = oracle["cases"][0]["baseline"].as_f64().unwrap();
    let mean = y.iter().sum::<f64>() / ROWS as f64;
    let ours = (mean / (1.0 - mean)).ln();
    println!("baseline ours={ours:.17} oracle={theirs:.17}");
    assert!((ours - theirs).abs() < 1e-15, "baseline gap {:.3e}", (ours - theirs).abs());
}

#[test]
fn the_port_reproduces_sklearn_at_every_setting() {
    let oracle = fixture();
    let x = design();
    let y = labels(&x);
    for case in oracle["cases"].as_array().unwrap() {
        let model = Classifier::fit(&x, &y, &options(case), &mut Mt19937::seeded(0))
            .expect("the kernel design fits");
        assert_eq!(model.iterations(), case["iterations"].as_u64().unwrap() as usize);

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

#[test]
fn the_classifier_rejects_input_it_cannot_use() {
    use hirmos_causal_core::gcm::boosting::regressor::BoostError;
    let x = design();
    let y = labels(&x);
    let settings = options(&fixture()["cases"][0]);
    let mut rng = Mt19937::seeded(0);

    assert!(matches!(
        Classifier::fit(&x, &y[..ROWS - 1], &settings, &mut rng),
        Err(BoostError::Shape)));

    let mut not_binary = y.clone();
    not_binary[3] = 0.5;
    assert!(matches!(
        Classifier::fit(&x, &not_binary, &settings, &mut rng),
        Err(BoostError::Shape)));

    let mut infinite = x.clone();
    infinite[(2, 2)] = f64::INFINITY;
    assert!(matches!(
        Classifier::fit(&infinite, &y, &settings, &mut rng),
        Err(BoostError::NonFinite)));
}
