use serde_json::Value;
use hirmos_causal_core::staggered_did::*;
use std::collections::BTreeMap;

#[test]
fn source_preprocessing_matches_r_without_hiding_changes() {
    let cases: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/staggered-did/preparation.json"
        ))
        .unwrap(),
    )
    .unwrap();
    for c in cases.as_array().unwrap() {
        let rows: Vec<_> = c["input"]
            .as_array()
            .unwrap()
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
        let control = if c["control"] == "nevertreated" {
            Controls::NeverTreated
        } else {
            Controls::NotYetTreated
        };
        let anticipation = Anticipation(c["anticipation"].as_u64().unwrap() as u32);
        let prepared = preparation::prepare(&rows, control, anticipation).unwrap();
        assert!(!prepared.changes.is_empty());
        let panel = &prepared.panel;
        assert_eq!(panel.units().count(), c["n"].as_u64().unwrap() as usize);
        let positions: BTreeMap<_, _> = panel.units().enumerate().map(|(i, u)| (u, i)).collect();
        let result = with_anticipation(panel, control, Baseline::Varying, anticipation).unwrap();
        assert_eq!(result.cells.len(), c["att"].as_array().unwrap().len());
        for (j, g) in c["group"].as_array().unwrap().iter().enumerate() {
            let Cell::Estimated(e) = &result.cells[&(
                Period(g.as_i64().unwrap()),
                Period(c["time"][j].as_i64().unwrap()),
            )] else {
                panic!()
            };
            assert!((e.att - c["att"][j].as_f64().unwrap()).abs() < 1e-9);
            assert!((e.se - c["se"][j].as_f64().unwrap()).abs() < 1e-9);
            for (i, id) in c["units"].as_array().unwrap().iter().enumerate() {
                assert!(
                    (e.influence[positions[&Unit(id.as_str().unwrap().parse().unwrap())]]
                        - c["influence"][i][j].as_f64().unwrap())
                    .abs()
                        < 1e-9
                );
            }
        }
        assert!((result.overall.att - c["overall"].as_f64().unwrap()).abs() < 1e-9);
    }
}
