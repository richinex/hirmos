//! Layout parity for the two encoders. Full-data value parity lives in the transpile, where the
//! chapter CSVs already sit; here the fixtures supply the real column names, kinds and levels.
use hirmos_causal_core::design::{get_dummies, patsy_dmatrix, Column, DesignError, Encoding, Values};
use serde_json::Value;

fn pandas() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/pandas_get_dummies_rhc.json")).unwrap()
}

fn strings(value: &Value) -> Vec<String> {
    value.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()
}

/// Rebuilds the confounder frame from the recorded kinds and levels. Each text column cycles its
/// levels so every one is present, and each numeric column holds distinct values.
fn rhc_columns() -> Vec<Column> {
    let oracle = pandas();
    let rows = 40usize;
    strings(&oracle["confounders"])
        .into_iter()
        .map(|name| {
            if oracle["kinds"][&name].as_str().unwrap() == "numeric" {
                Column {
                    name,
                    values: Values::Numeric((0..rows).map(|row| row as f64).collect()),
                    encoding: Encoding::Numeric,
                }
            } else {
                let levels = strings(&oracle["levels"][&name]);
                Column {
                    values: Values::Text(
                        (0..rows).map(|row| levels[row % levels.len()].clone()).collect(),
                    ),
                    name,
                    encoding: Encoding::Indicators,
                }
            }
        })
        .collect()
}

#[test]
fn get_dummies_reproduces_the_pandas_column_layout() {
    let oracle = pandas();
    let design = get_dummies(&rhc_columns()).expect("the confounders encode");
    assert_eq!(design.names, strings(&oracle["names"]), "column names and their order");
    assert_eq!(design.names.len(), oracle["columns"].as_u64().unwrap() as usize);
}

#[test]
fn patsy_puts_the_intercept_first_then_contrasts_then_numerics() {
    let layout: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/patsy_layout_management.json"))
            .unwrap();
    // The chapter's own level sets: n_of_reports 1 to 8, gender 1 and 2, role 0 to 4.
    let rows = 8usize;
    let cycle = |levels: &[f64]| -> Values {
        Values::Numeric((0..rows).map(|row| levels[row % levels.len()]).collect())
    };
    let columns = vec![
        Column { name: "tenure".into(), values: cycle(&[1.0, 2.0, 3.0]),
            encoding: Encoding::Numeric },
        Column { name: "last_engagement_score".into(), values: cycle(&[0.1, 0.2]),
            encoding: Encoding::Numeric },
        Column { name: "department_score".into(), values: cycle(&[0.5, 0.6]),
            encoding: Encoding::Numeric },
        Column { name: "n_of_reports".into(),
            values: cycle(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
            encoding: Encoding::TreatmentContrast },
        Column { name: "gender".into(), values: cycle(&[1.0, 2.0]),
            encoding: Encoding::TreatmentContrast },
        Column { name: "role".into(), values: cycle(&[0.0, 1.0, 2.0, 3.0, 4.0]),
            encoding: Encoding::TreatmentContrast },
    ];
    let design = patsy_dmatrix(&columns).expect("the chapter formula encodes");
    assert_eq!(design.names[0], "Intercept");
    assert!(design.rows.iter().all(|row| row[0] == 1.0));

    assert_eq!(design.names, strings(&layout["layouts"]["chapter"]["names"]),
        "the chapter's own column names and their order");
}

/// The chapter reads `role` as five unordered levels. Entering it as one numeric column asserts
/// that role 4 is twice role 2, which is a different model.
#[test]
fn one_column_takes_a_different_shape_under_each_encoding() {
    let values = Values::Numeric(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    let numeric = get_dummies(&[Column {
        name: "role".into(), values: values.clone(), encoding: Encoding::Numeric }]).unwrap();
    assert_eq!(numeric.names, vec!["role"]);

    let contrast = get_dummies(&[Column {
        name: "role".into(), values, encoding: Encoding::TreatmentContrast }]).unwrap();
    assert_eq!(contrast.names,
        vec!["C(role)[T.1]", "C(role)[T.2]", "C(role)[T.3]", "C(role)[T.4]"]);
    assert_eq!(contrast.rows[0], vec![0.0, 0.0, 0.0, 0.0], "the baseline row is all zeros");
}

#[test]
fn encoding_rejects_input_it_cannot_use() {
    assert_eq!(get_dummies(&[]).unwrap_err(), DesignError::EmptyFrame);
    assert_eq!(
        get_dummies(&[Column { name: "a".into(),
            values: Values::Text(vec!["x".into(), "x".into()]), encoding: Encoding::Indicators }])
            .unwrap_err(),
        DesignError::SingleLevel { column: "a".into() });
    assert_eq!(
        get_dummies(&[Column { name: "a".into(),
            values: Values::Text(vec!["x".into(), "y".into()]), encoding: Encoding::Numeric }])
            .unwrap_err(),
        DesignError::NoLevels { column: "a".into() });
}
