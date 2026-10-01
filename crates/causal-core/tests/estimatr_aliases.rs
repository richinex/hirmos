use hirmos_causal_core::estimation::aliases::{select, select_clustered, ColumnStatus, Error};
use nalgebra::DMatrix;

#[test]
fn oracle_kept_columns() {
    let root: serde_json::Value =
        serde_json::from_str(include_str!("../oracle/fixtures/mixtape_estimatr.json")).unwrap();
    for (name, c) in root["cases"].as_object().unwrap() {
        let x: Vec<Vec<f64>> = serde_json::from_value(c["full_x"].clone()).unwrap();
        let w: Vec<f64> = serde_json::from_value(c["weights"].clone()).unwrap();
        let cluster: Vec<u64> = serde_json::from_value(c["clusters"].clone()).unwrap();
        let n = x.len();
        let p = x[0].len();
        let mut order: Vec<_> = (0..n).collect();
        order.sort_by_key(|i| cluster[*i]);
        let mean = w.iter().sum::<f64>() / n as f64;
        let weighted = DMatrix::from_fn(n, p, |i, j| x[order[i]][j] * (w[order[i]] / mean).sqrt());
        let actual = select(&weighted).unwrap();
        let expected: Vec<usize> = serde_json::from_value(c["keep"].clone()).unwrap();
        let dropped: Vec<_> = actual
            .columns()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| matches!(c, ColumnStatus::Aliased).then_some(i))
            .collect();
        eprintln!("{name}: rank={} dropped={dropped:?}", actual.kept().len());
        assert_eq!(actual.kept(), expected, "{name}");
    }
}

#[test]
fn rank_states_and_invalid_inputs() {
    let x = DMatrix::from_row_slice(4, 3, &[1., 0., 0., 1., 1., 2., 1., 2., 4., 1., 3., 6.]);
    let selection = select(&x).unwrap();
    assert_eq!(selection.kept().len(), 2);
    assert_eq!(
        selection
            .columns()
            .iter()
            .filter(|c| matches!(c, ColumnStatus::Aliased))
            .count(),
        1
    );
    for (coefficient, &j) in selection.kept().iter().enumerate() {
        assert_eq!(
            selection.columns()[j],
            ColumnStatus::Retained { coefficient }
        );
    }
    assert_eq!(select(&DMatrix::zeros(0, 2)).unwrap_err(), Error::Empty);
    assert_eq!(
        select(&DMatrix::from_element(2, 2, f64::NAN)).unwrap_err(),
        Error::NonFinite
    );
    let zero = select(&DMatrix::zeros(3, 2)).unwrap();
    assert_eq!(
        zero.columns(),
        &[ColumnStatus::Aliased, ColumnStatus::Aliased]
    );
}

#[test]
fn clustered_selection_boundaries() {
    let x = DMatrix::from_row_slice(4, 2, &[1., 0., 1., 1., 1., 2., 1., 3.]);
    let clusters = [2, 1, 2, 1];
    assert_eq!(select_clustered(&x, &[1.; 3], &clusters).unwrap_err(), Error::Shape);
    assert_eq!(select_clustered(&x, &[1.; 4], &[1; 3]).unwrap_err(), Error::Shape);
    for weights in [[0.; 4], [1., -1., 1., 1.], [1., f64::NAN, 1., 1.]] {
        assert_eq!(select_clustered(&x, &weights, &clusters).unwrap_err(), Error::InvalidWeights);
    }
    let selected = select_clustered(&x, &[0., 1., 1., 1.], &clusters).unwrap();
    assert_eq!(selected.kept(), &[0, 1]);
    let scaled = select_clustered(&x, &[0., 7., 7., 7.], &clusters).unwrap();
    assert_eq!(selected.columns(), scaled.columns());
}
