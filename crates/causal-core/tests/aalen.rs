use hirmos_causal_core::survival::aalen::{
    coefficient_curves, fit, fit_with_influence, summarize, summarize_through, Data, Options,
    SummaryTable, TestWeight,
};
use serde_json::Value;

fn compare(actual: &Value, expected: &Value, path: &str) {
    if let Some(values) = actual.as_array() {
        let reference = expected.as_array().unwrap();
        assert_eq!(values.len(), reference.len(), "{path}");
        for (i, (a, b)) in values.iter().zip(reference).enumerate() {
            compare(a, b, &format!("{path}[{i}]"));
        }
    } else {
        let a = actual.as_f64().unwrap();
        let b = expected.as_f64().unwrap();
        assert!((a - b).abs() <= 1e-9 + 1e-8 * b.abs(), "{path}: {a} != {b}");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn r_aareg_event_outputs() {
    let fixtures: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/aalen.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let input = &case["input"];
        let x: Vec<Vec<f64>> = input["x"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                if row.is_array() {
                    serde_json::from_value(row.clone()).unwrap()
                } else {
                    vec![row.as_f64().unwrap()]
                }
            })
            .collect();
        let mut data = Data::new(
            x,
            serde_json::from_value(input["y"].clone()).unwrap(),
            serde_json::from_value(input["weights"].clone()).unwrap(),
        )
        .unwrap();
        if input["cluster"].is_array() {
            data = data
                .with_clusters(serde_json::from_value(input["cluster"].clone()).unwrap())
                .unwrap();
        }
        let taper = if input["taper"].is_array() {
            serde_json::from_value(input["taper"].clone()).unwrap()
        } else {
            vec![input["taper"].as_f64().unwrap()]
        };
        let options = Options {
            nmin: Some(input["nmin"].as_u64().unwrap() as usize),
            taper,
            test: if input["test"] == "aalen" {
                TestWeight::Aalen
            } else {
                TestWeight::AtRisk
            },
            ..Options::default()
        };
        let fitted = if input["dfbeta"] == true {
            fit_with_influence(&data, &options)
        } else {
            fit(&data, &options)
        }
        .unwrap();
        if let Some(influence) = &fitted.influence {
            compare(
                &serde_json::json!(influence.increments),
                &case["expected"]["dfbeta"],
                &format!("{}.dfbeta", case["name"]),
            );
            compare(
                &serde_json::json!(influence.variance),
                &case["expected"]["robust_variance"],
                &format!("{}.robust_variance", case["name"]),
            );
        }
        let curves = coefficient_curves(&fitted);
        for (term, curve) in curves.iter().enumerate() {
            for (i, point) in curve.iter().enumerate() {
                for (key, value) in [
                    ("estimate", point.estimate),
                    ("lower", point.lower),
                    ("upper", point.upper),
                ] {
                    compare(
                        &serde_json::json!(value),
                        &case["expected"]["plotted_curve"][key][i][term],
                        &format!("{}.curve.{key}[{i},{term}]", case["name"]),
                    );
                }
                compare(
                    &serde_json::json!(point.time),
                    &case["expected"]["plotted_curve"]["time"][i],
                    "curve time",
                );
            }
        }
        {
            let summary = summarize(&fitted).unwrap();
            let table = match summary.table {
                SummaryTable::Model(v) => serde_json::json!(v),
                SummaryTable::Robust(v) => serde_json::json!(v),
            };
            compare(
                &table,
                &case["expected"]["summary"],
                &format!("{}.summary", case["name"]),
            );
            compare(
                &serde_json::json!(summary.chisq),
                &case["expected"]["chisq"],
                &format!("{}.chisq", case["name"]),
            );
        }
        for expected in case["expected"]["cutoff_summaries"].as_array().unwrap() {
            let weight = if expected["test"] == "aalen" {
                TestWeight::Aalen
            } else {
                TestWeight::AtRisk
            };
            let summary = summarize_through(
                &fitted,
                expected["time"].as_f64().unwrap(),
                weight,
                expected["scale"].as_f64().unwrap(),
            )
            .unwrap();
            let table = match summary.table {
                SummaryTable::Model(v) => serde_json::json!(v),
                SummaryTable::Robust(v) => serde_json::json!(v),
            };
            compare(
                &table,
                &expected["table"],
                &format!("{}.cutoff", case["name"]),
            );
            compare(
                &serde_json::json!(summary.chisq),
                &expected["chisq"],
                "cutoff chi-square",
            );
        }
        let result = serde_json::json!({"n": fitted.n, "times": fitted.times,
            "nrisk": fitted.nrisk, "coefficient": fitted.coefficient,
            "tweight": fitted.tweight, "statistic": fitted.statistic, "variance": fitted.variance});
        for key in [
            "n",
            "times",
            "nrisk",
            "coefficient",
            "tweight",
            "statistic",
            "variance",
        ] {
            compare(
                &result[key],
                &case["expected"][key],
                &format!("{}.{key}", case["name"]),
            );
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_aalen_inputs_are_rejected() {
    use hirmos_causal_core::survival::aalen::Error;
    assert!(matches!(
        Data::new(vec![vec![0.0]], vec![[1.0, 1.0, 1.0]], vec![1.0]),
        Err(Error::InvalidInterval)
    ));
    assert!(matches!(
        Data::new(vec![vec![f64::NAN]], vec![[0.0, 1.0, 1.0]], vec![1.0]),
        Err(Error::InvalidValue)
    ));
    let rows: Vec<Vec<f64>> = (0..20).map(|i| vec![(i as f64).sin()]).collect();
    let intervals: Vec<[f64; 3]> = (0..20).map(|i| [-1.0, (i + 1) as f64, 0.0]).collect();
    let data = Data::new(rows, intervals, vec![1.0; 20]).unwrap();
    assert!(matches!(
        fit(&data, &Options::default()),
        Err(Error::TooFewEvents)
    ));
    assert!(matches!(
        fit(
            &data,
            &Options {
                taper: vec![0.0],
                ..Options::default()
            }
        ),
        Err(Error::InvalidOptions)
    ));
    assert!(data.with_clusters(vec![1]).is_err());
}
