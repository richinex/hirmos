use hirmos_causal_core::panel_design::{
    bacon::{Comparison, Error},
    Panel,
};
use nalgebra::DMatrix;
use serde::Deserialize;
use std::collections::BTreeMap;
#[derive(Deserialize)]
struct Row {
    id: u64,
    time: i64,
    y: f64,
    treated: f64,
}
#[derive(Deserialize)]
struct Expected {
    treated: i64,
    untreated: Option<i64>,
    estimate: f64,
    weight: f64,
}
#[derive(Deserialize)]
struct Case {
    data: Vec<Row>,
    comparisons: Vec<Expected>,
    twfe: f64,
}
#[derive(Deserialize)]
struct Fixture {
    cases: BTreeMap<String, Case>,
}
fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-8 * b.abs().max(1.), "{a} != {b}");
}
fn panel(rows: &[Row]) -> Panel {
    Panel::new(
        &rows.iter().map(|r| (r.id, r.time)).collect::<Vec<_>>(),
        DMatrix::from_fn(rows.len(), 2, |i, j| {
            if j == 0 {
                rows[i].y
            } else {
                rows[i].treated
            }
        }),
    )
    .unwrap()
}
#[test]
fn pinned_bacondecomp() {
    let f: Fixture = serde_json::from_str(include_str!("../oracle/fixtures/bacon.json")).unwrap();
    for (name, mut c) in f.cases {
        for reversed in [false, true] {
            if reversed {
                c.data.reverse();
            }
            let fit = panel(&c.data).bacon_unadjusted(0, 1).unwrap();
            near(fit.twfe, c.twfe);
            near(fit.reconstructed, c.twfe);
            near(fit.components.iter().map(|c| c.weight).sum(), 1.);
            assert_eq!(fit.components.len(), c.comparisons.len());
            let first = c.data.iter().map(|r| r.time).min().unwrap();
            for e in &c.comparisons {
                let kind = match e.untreated {
                    None => Comparison::TreatedVsNever {
                        adoption: e.treated,
                    },
                    Some(h) if h == first => Comparison::LaterVsAlways {
                        adoption: e.treated,
                    },
                    Some(h) if e.treated < h => Comparison::EarlierVsLater {
                        earlier: e.treated,
                        later: h,
                    },
                    Some(h) => Comparison::LaterVsEarlier {
                        earlier: h,
                        later: e.treated,
                    },
                };
                let a = fit
                    .components
                    .iter()
                    .find(|a| a.comparison == kind)
                    .unwrap();
                near(a.estimate, e.estimate);
                near(a.weight, e.weight);
            }
        }
        eprintln!("{name}: comparisons and TWFE reconstruction pass");
    }
}
#[test]
fn unsupported_inputs_are_named() {
    let mut rows: Vec<_> = (0..4)
        .flat_map(|id| {
            (0..4).map(move |time| Row {
                id,
                time,
                y: (id + time as u64) as f64,
                treated: 0.,
            })
        })
        .collect();
    assert_eq!(
        panel(&rows).bacon_unadjusted(0, 1).unwrap_err(),
        Error::NoComparisons
    );
    assert_eq!(
        panel(&rows).bacon_unadjusted(0, 0).unwrap_err(),
        Error::InvalidColumns
    );
    rows[2].treated = 0.5;
    assert_eq!(
        panel(&rows).bacon_unadjusted(0, 1).unwrap_err(),
        Error::NonBinaryTreatment { row: 2 }
    );
    rows[2].treated = 1.;
    assert_eq!(
        panel(&rows).bacon_unadjusted(0, 1).unwrap_err(),
        Error::TreatmentReversal { unit: 0, period: 3 }
    );
}
