use hirmos_causal_core::model_selection::{stratified_shuffle_split, SearchError};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../oracle/fixtures/sklearn_stratified_split.json")).unwrap()
}

fn indices(value: &serde_json::Value) -> Vec<usize> {
    value.as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect()
}

#[test]
fn the_split_matches_sklearn_row_for_row() {
    for case in fixture()["cases"].as_array().unwrap() {
        let rows = case["rows"].as_u64().unwrap() as usize;
        let period = case["period"].as_u64().unwrap() as usize;
        let seed = case["seed"].as_u64().unwrap() as u32;
        let y: Vec<f64> = (0..rows).map(|row| f64::from(row % period == 0)).collect();
        assert_eq!(y.iter().sum::<f64>() as u64, case["positives"].as_u64().unwrap());

        let (train, test) = stratified_shuffle_split(&y, 0.5, seed).expect("both classes present");
        assert_eq!(train, indices(&case["train"]), "train for {rows} rows");
        assert_eq!(test, indices(&case["test"]), "test for {rows} rows");
    }
}

#[test]
fn the_two_halves_partition_the_sample_and_keep_the_prevalence() {
    for case in fixture()["cases"].as_array().unwrap() {
        let rows = case["rows"].as_u64().unwrap() as usize;
        let period = case["period"].as_u64().unwrap() as usize;
        let y: Vec<f64> = (0..rows).map(|row| f64::from(row % period == 0)).collect();
        let (train, test) = stratified_shuffle_split(&y, 0.5, 1234).expect("both classes present");

        let mut all: Vec<usize> = train.iter().chain(&test).copied().collect();
        all.sort_unstable();
        assert_eq!(all, (0..rows).collect::<Vec<_>>(), "the halves partition {rows} rows");

        let positives = |half: &[usize]| half.iter().filter(|&&row| y[row] == 1.0).count();
        let difference = positives(&train).abs_diff(positives(&test));
        assert!(difference <= 1, "{rows} rows: prevalence differs by {difference}");
    }
}

#[test]
fn the_split_rejects_input_it_cannot_use() {
    let y: Vec<f64> = (0..100).map(|row| f64::from(row % 3 == 0)).collect();
    assert_eq!(stratified_shuffle_split(&[], 0.5, 0).unwrap_err(), SearchError::EmptySample);
    assert_eq!(stratified_shuffle_split(&y, 0.0, 0).unwrap_err(), SearchError::TooFewSplits);
    assert_eq!(stratified_shuffle_split(&y, 1.0, 0).unwrap_err(), SearchError::TooFewSplits);

    let mut lonely = vec![0.0; 100];
    lonely[7] = 1.0;
    assert_eq!(stratified_shuffle_split(&lonely, 0.5, 0).unwrap_err(),
        SearchError::ClassTooSmall { encoded: 1, count: 1 });
}
