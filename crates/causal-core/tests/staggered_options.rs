use serde_json::Value;
use hirmos_causal_core::staggered_did::*;
use std::collections::BTreeMap;

fn close(actual: f64, expected: &Value, label: &str) {
    if expected.is_null() {
        assert!(actual.abs() < 1e-7, "{label}: {actual}");
        return;
    }
    let expected = expected.as_f64().unwrap();
    assert!(
        (actual - expected).abs() < 1e-9,
        "{label}: {actual} != {expected}"
    );
}

#[test]
fn weighted_anticipation_and_event_windows_match_r() {
    let oracle: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/staggered-did/options.json"
        ))
        .unwrap(),
    )
    .unwrap();
    let input = oracle["input"].as_array().unwrap();
    let rows: Vec<_> = input
        .iter()
        .map(|r| Row {
            unit: Unit(r["id"].as_u64().unwrap()),
            period: Period(r["time"].as_i64().unwrap()),
            adoption: match r["cohort"].as_i64().unwrap() {
                0 => Adoption::Never,
                g => Adoption::At(Period(g)),
            },
            outcome: r["y"].as_f64().unwrap(),
        })
        .collect();
    for (case_index, c) in oracle["cases"].as_array().unwrap().iter().enumerate() {
        let mut panel = Panel::new(&rows).unwrap();
        if c["weighted"].as_bool().unwrap() {
            let weights: BTreeMap<_, _> = input
                .iter()
                .map(|r| {
                    (
                        Unit(r["id"].as_u64().unwrap()),
                        r["weight"].as_f64().unwrap(),
                    )
                })
                .collect();
            panel = panel.with_weights(weights.into_iter().collect()).unwrap();
        }
        let adjusted = Adjusted::new(
            &panel,
            input
                .iter()
                .map(|r| {
                    (
                        Unit(r["id"].as_u64().unwrap()),
                        Period(r["time"].as_i64().unwrap()),
                        vec![r["x"].as_f64().unwrap()],
                    )
                })
                .collect(),
        )
        .unwrap();
        let controls = if c["control"] == "nevertreated" {
            Controls::NeverTreated
        } else {
            Controls::NotYetTreated
        };
        let baseline = if c["baseline"] == "varying" {
            Baseline::Varying
        } else {
            Baseline::Universal
        };
        let anticipation = Anticipation(c["anticipation"].as_u64().unwrap() as u32);
        let positions: BTreeMap<_, _> = panel.units().enumerate().map(|(i, u)| (u, i)).collect();
        for (w, window) in [
            EventWindow::default(),
            EventWindow::new(Some(-2), Some(2), None).unwrap(),
            EventWindow::new(Some(-2), Some(2), Some(2)).unwrap(),
        ]
        .into_iter()
        .enumerate()
        {
            let result = if c["adjusted"].as_bool().unwrap() {
                adjusted.event_study(controls, baseline, anticipation, window)
            } else {
                event_study(&panel, controls, baseline, anticipation, window)
            }
            .unwrap();
            let expected = &c["windows"][w];
            let label = format!("case {case_index}, window {w}");
            assert_eq!(
                result.events.len(),
                expected["event"].as_array().unwrap().len()
            );
            for (j, event) in expected["event"].as_array().unwrap().iter().enumerate() {
                let s = &result.events[&event.as_i64().unwrap()];
                close(s.att, &expected["att"][j], &label);
                close(s.se, &expected["se"][j], &label);
                for (i, id) in c["units"].as_array().unwrap().iter().enumerate() {
                    close(
                        s.influence[positions[&Unit(id.as_str().unwrap().parse().unwrap())]],
                        &expected["influence"][i][j],
                        &label,
                    );
                }
            }
            close(result.overall.att, &expected["overall_att"], &label);
            close(result.overall.se, &expected["overall_se"], &label);
            for (j, s) in [
                &result.simple,
                &result.group_overall,
                &result.calendar_overall,
            ]
            .iter()
            .enumerate()
            {
                close(s.att, &c["aggregations"][j]["att"], &label);
                close(s.se, &c["aggregations"][j]["se"], &label);
            }
            for (j, g) in c["group"].as_array().unwrap().iter().enumerate() {
                let cell = &result.cells[&(
                    Period(g.as_i64().unwrap()),
                    Period(c["time"][j].as_i64().unwrap()),
                )];
                match cell {
                    Cell::Reference => close(0.0, &c["att"][j], &label),
                    Cell::Estimated(e) => {
                        close(e.att, &c["att"][j], &label);
                        close(e.se, &c["se"][j], &label);
                        for (i, id) in c["units"].as_array().unwrap().iter().enumerate() {
                            close(
                                e.influence
                                    [positions[&Unit(id.as_str().unwrap().parse().unwrap())]],
                                &c["influence"][i][j],
                                &label,
                            );
                        }
                    }
                }
            }
        }
    }
}
