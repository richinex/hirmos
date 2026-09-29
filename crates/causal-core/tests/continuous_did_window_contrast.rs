use hirmos_causal_core::continuous_did::{
    panel::{Adoption, Observation, Period, Row, Unit},
    window_contrast::{prepare, Eligibility, Error},
};
use std::collections::BTreeMap;

#[test]
fn archived_windows_and_row_order() {
    let f: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/window-contrast.json")).unwrap();
    let mut dates = BTreeMap::new();
    let curves: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/response-curve.json")).unwrap();
    let mut rows: Vec<_> = f["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let unit = Unit(r["i"].as_u64().unwrap());
            let g = Period(r["G"].as_i64().unwrap());
            dates.insert(unit, Some(g));
            let dose = r["d"].as_f64().unwrap();
            Row {
                dose,
                observation: Observation {
                    unit,
                    period: Period(r["t"].as_i64().unwrap()),
                    adoption: if dose == 0.0 {
                        Adoption::Never
                    } else {
                        Adoption::At(g)
                    },
                    outcome: r["y"].as_f64().unwrap(),
                },
            }
        })
        .collect();
    for _ in 0..2 {
        for case in f["cases"].as_array().unwrap() {
            let result = prepare(
                &rows,
                case["first"].as_i64().unwrap(),
                case["last"].as_i64().unwrap(),
                Eligibility::WindowEndWithinPanel(&dates),
            )
            .unwrap();
            let expected = case["expected"].as_array().unwrap();
            assert_eq!(result.rows.len(), expected.len());
            for (got, want) in result.rows.iter().zip(expected) {
                assert_eq!(got.unit.0, want["i"].as_u64().unwrap());
                assert!((got.contrast - want["ddY"].as_f64().unwrap()).abs() < 1e-12);
                assert_eq!(got.dose, want["d"].as_f64().unwrap());
            }
            // Compose preparation and curve fitting, not just isolated stages.
            let key = match case["name"].as_str().unwrap() {
                "short" => "SR",
                "long" => "LR",
                "pooled" => "all",
                _ => panic!("unknown window"),
            };
            let reference = &curves[key];
            let numbers = |key: &str| {
                reference[key]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect::<Vec<_>>()
            };
            let fitted = hirmos_causal_core::continuous_did::response_curve::fit(
                &result.rows.iter().map(|r| r.dose).collect::<Vec<_>>(),
                &result.rows.iter().map(|r| r.contrast).collect::<Vec<_>>(),
                &numbers("grid"),
                reference["degree"].as_u64().unwrap() as usize,
                numbers("knots"),
            )
            .unwrap();
            for (got, want) in fitted.curve.iter().zip(numbers("curve")) {
                assert!((got - want).abs() < 1e-9);
            }
            for (got, want) in fitted.standard_errors.iter().zip(numbers("se")) {
                assert!((got - want).abs() < 1e-9);
            }
        }
        rows.reverse();
    }
}

fn independent() -> Vec<Row> {
    (1..=3)
        .flat_map(|u| {
            (0..=6).map(move |t| Row {
                dose: if u == 1 { 0.0 } else { u as f64 },
                observation: Observation {
                    unit: Unit(u),
                    period: Period(t),
                    adoption: if u == 1 {
                        Adoption::Never
                    } else {
                        Adoption::At(Period(2))
                    },
                    outcome: 10.0 * u as f64
                        + 3.0 * t as f64
                        + if u > 1 && t >= 2 {
                            (u as f64) * (t - 1) as f64
                        } else {
                            0.0
                        },
                },
            })
        })
        .collect()
}

#[test]
fn independent_changes_and_refusals() {
    let rows = independent();
    let fit = prepare(&rows, 1, 3, Eligibility::All).unwrap();
    assert_eq!(fit.rows.len(), 2);
    for row in fit.rows {
        assert_eq!(row.contrast, 3.0 * row.dose);
        assert_eq!(row.comparison_units, 1);
        assert_eq!(row.periods, 3);
    }
    assert!(matches!(
        prepare(&rows, -1, 2, Eligibility::All),
        Err(Error::InvalidWindow)
    ));
    let gapped: Vec<_> = rows
        .iter()
        .copied()
        .filter(|r| r.observation.period != Period(4))
        .collect();
    assert!(matches!(
        prepare(&gapped, 0, 2, Eligibility::All),
        Err(Error::IrregularPeriods)
    ));
    assert!(matches!(
        prepare(&rows, 0, 9, Eligibility::All),
        Err(Error::NoRetainedTreated)
    ));
    assert!(matches!(
        prepare(
            &rows,
            0,
            2,
            Eligibility::WindowEndWithinPanel(&BTreeMap::new())
        ),
        Err(Error::EligibilityKeys)
    ));
    let no_controls: Vec<_> = rows.iter().copied().filter(|r| r.dose > 0.0).collect();
    assert!(matches!(
        prepare(&no_controls, 0, 2, Eligibility::All),
        Err(Error::MissingControls(_))
    ));
}
