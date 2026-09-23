use serde_json::Value;
use hirmos_causal_core::staggered_did::*;
use std::{collections::BTreeMap, fs, path::PathBuf};

fn fixture(name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/staggered-did")
        .join(name);
    serde_json::from_str(
        &fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{}: {e}; run the Docker oracle first", path.display())),
    )
    .unwrap()
}

fn close(actual: f64, expected: &Value, label: &str) {
    let expected = expected.as_f64().unwrap();
    assert!(
        (actual - expected).abs() < 1e-9,
        "{label}: Rust={actual:.16}, R={expected:.16}"
    );
}

#[test]
fn cells_and_all_aggregations_match_r() {
    let input = fixture("mpdta-input.json");
    let rows: Vec<_> = input
        .as_array()
        .unwrap()
        .iter()
        .map(|r| Row {
            unit: Unit(r["countyreal"].as_u64().unwrap()),
            period: Period(r["year"].as_i64().unwrap()),
            adoption: match r["first.treat"].as_i64().unwrap() {
                0 => Adoption::Never,
                g => Adoption::At(Period(g)),
            },
            outcome: r["lemp"].as_f64().unwrap(),
        })
        .collect();
    let panel = Panel::new(&rows).unwrap();
    let adjusted = Adjusted::new(
        &panel,
        input
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    Unit(r["countyreal"].as_u64().unwrap()),
                    Period(r["year"].as_i64().unwrap()),
                    vec![r["lpop"].as_f64().unwrap()],
                )
            })
            .collect(),
    )
    .unwrap();
    let positions: BTreeMap<_, _> = panel.units().enumerate().map(|(i, u)| (u, i)).collect();
    let reference = fixture("mpdta.json");
    for (control_name, control) in [
        ("nevertreated", Controls::NeverTreated),
        ("notyettreated", Controls::NotYetTreated),
    ] {
        for (base_name, base) in [
            ("varying", Baseline::Varying),
            ("universal", Baseline::Universal),
        ] {
            for adjustment in [false, true] {
                let suffix = if adjustment { "adjusted" } else { "unadjusted" };
                let key = format!("{control_name}_{base_name}_{suffix}");
                let case = &reference["cases"][&key];
                let mut cell_influence = Vec::new();
                for (column, g) in case["group"].as_array().unwrap().iter().enumerate() {
                    let t = Period(case["time"][column].as_i64().unwrap());
                    let estimate = if adjustment {
                        adjusted.cell(Period(g.as_i64().unwrap()), t, control, base)
                    } else {
                        cell(&panel, Period(g.as_i64().unwrap()), t, control, base)
                    }
                    .unwrap();
                    let (att, se, influence) = match estimate {
                        Cell::Reference => (0.0, 0.0, vec![0.0; positions.len()]),
                        Cell::Estimated(e) => (e.att, e.se, e.influence),
                    };
                    close(att, &case["att"][column], &key);
                    if !case["se"][column].is_null() {
                        close(se, &case["se"][column], &key);
                    } else {
                        assert!(se < 1e-7);
                    }
                    for (row, id) in case["unit_ids"].as_array().unwrap().iter().enumerate() {
                        let unit = Unit(id.as_str().unwrap().parse().unwrap());
                        close(
                            influence[positions[&unit]],
                            &case["influence"][row][column],
                            &key,
                        );
                    }
                    cell_influence.push(influence);
                }
                for (j, a) in cell_influence.iter().enumerate() {
                    for (k, b) in cell_influence.iter().enumerate() {
                        let covariance = a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>()
                            / (positions.len() as f64).powi(2);
                        close(covariance, &case["covariance"][j][k], &key);
                    }
                }
                let aggregate = if adjustment {
                    adjusted.dynamic(control, base)
                } else {
                    dynamic(&panel, control, base)
                }
                .unwrap();
                assert_eq!(
                    aggregate.events.len(),
                    case["event"].as_array().unwrap().len()
                );
                for (index, e) in case["event"].as_array().unwrap().iter().enumerate() {
                    let actual = &aggregate.events[&e.as_i64().unwrap()];
                    close(actual.att, &case["event_att"][index], &key);
                    if !case["event_se"][index].is_null() {
                        close(actual.se, &case["event_se"][index], &key);
                    } else {
                        assert!(actual.se < 1e-7);
                    }
                    for (row, id) in case["unit_ids"].as_array().unwrap().iter().enumerate() {
                        let unit = Unit(id.as_str().unwrap().parse().unwrap());
                        close(
                            actual.influence[positions[&unit]],
                            &case["event_influence"][row][index],
                            &key,
                        );
                    }
                }
                close(aggregate.overall.att, &case["overall_att"], &key);
                close(aggregate.overall.se, &case["overall_se"], &key);
                let aggregations = &case["aggregations"];
                close(
                    aggregate.simple.att,
                    &aggregations["simple"]["overall_att"],
                    &key,
                );
                close(
                    aggregate.simple.se,
                    &aggregations["simple"]["overall_se"],
                    &key,
                );
                for (kind, values, overall) in [
                    ("group", &aggregate.by_group, &aggregate.group_overall),
                    (
                        "calendar",
                        &aggregate.by_calendar,
                        &aggregate.calendar_overall,
                    ),
                ] {
                    close(overall.att, &aggregations[kind]["overall_att"], &key);
                    close(overall.se, &aggregations[kind]["overall_se"], &key);
                    for (index, t) in aggregations[kind]["axis"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .enumerate()
                    {
                        let actual = &values[&Period(t.as_i64().unwrap())];
                        close(actual.att, &aggregations[kind]["att"][index], &key);
                        close(actual.se, &aggregations[kind]["se"][index], &key);
                    }
                }
                for (row, id) in case["unit_ids"].as_array().unwrap().iter().enumerate() {
                    let unit = Unit(id.as_str().unwrap().parse().unwrap());
                    close(
                        aggregate.overall.influence[positions[&unit]],
                        &case["overall_influence"][row],
                        &key,
                    );
                }
            }
        }
    }
}
