use serde_json::Value;
use hirmos_causal_core::staggered_did::{inference::*, *};

#[test]
fn adjusted_event_study_and_clustered_intervals_match_the_r_chain() {
    let read = |name: &str| {
        serde_json::from_str::<Value>(
            &std::fs::read_to_string(format!(
                "{}/fixtures/staggered-did/{name}.json",
                env!("CARGO_MANIFEST_DIR")
            ))
            .unwrap(),
        )
        .unwrap()
    };
    let input = read("mpdta-input");
    let reference = read("inference");
    let input = input.as_array().unwrap();
    let rows: Vec<_> = input
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
    let p = Panel::new(&rows).unwrap();
    let adjusted = Adjusted::new(
        &p,
        input
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
    let result = adjusted
        .dynamic(Controls::NeverTreated, Baseline::Varying)
        .unwrap();
    let points: Vec<_> = reference["event"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| Point::Estimated(&result.events[&e.as_i64().unwrap()]))
        .collect();
    for c in reference["cases"].as_array().unwrap() {
        let clusters: Vec<_> = c["clusters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();
        for (label, method) in [
            (
                "pointwise",
                Method::BootstrapPointwise {
                    iterations: 999,
                    seed: 731,
                },
            ),
            (
                "simultaneous",
                Method::BootstrapSimultaneous {
                    iterations: 999,
                    seed: 731,
                },
            ),
        ] {
            let output = infer(&points, &clusters, Confidence::new(0.95).unwrap(), method).unwrap();
            for (i, interval) in output.intervals.iter().enumerate() {
                let Interval::Estimated {
                    se, lower, upper, ..
                } = interval
                else {
                    panic!()
                };
                for (actual, expected) in [
                    (*se, &c["se"][i]),
                    (*lower, &c[format!("{label}_lower")][i]),
                    (*upper, &c[format!("{label}_upper")][i]),
                ] {
                    assert!(
                        (actual - expected.as_f64().unwrap()).abs() < 1e-9,
                        "{label}: {actual} != {expected}"
                    );
                }
            }
        }
    }
}
