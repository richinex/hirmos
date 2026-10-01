use hirmos_causal_core::panel_design::{bacon::adjusted::Comparison, Panel};
use nalgebra::DMatrix;
use serde::Deserialize;
use std::collections::BTreeMap;
#[derive(Deserialize)]
struct Row {
    id: u64,
    time: i64,
    y: f64,
    treated: f64,
    x1: f64,
    x2: f64,
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
    omega: f64,
    within: f64,
}
#[derive(Deserialize)]
struct Fixture {
    cases: BTreeMap<String, Case>,
}
fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= 1e-8 * b.abs().max(1.),
        "{label}: {a} != {b}"
    );
}
#[test]
fn adjusted_oracle() {
    let f: Fixture =
        serde_json::from_str(include_str!("../oracle/fixtures/bacon-adjusted.json")).unwrap();
    for (name, mut c) in f.cases {
        for reverse in [false, true] {
            if reverse {
                c.data.reverse();
            }
            let keys: Vec<_> = c.data.iter().map(|r| (r.id, r.time)).collect();
            let values = DMatrix::from_fn(c.data.len(), 4, |i, j| {
                let r = &c.data[i];
                [r.y, r.treated, r.x1, r.x2][j]
            });
            let p = Panel::new(&keys, values).unwrap();
            let fit = p.bacon_adjusted(0, 1, &[2, 3]).unwrap();
            near(fit.twfe, c.twfe, &name);
            near(fit.reconstructed, c.twfe, "reconstruction");
            near(fit.within_estimate, c.within, "within");
            near(fit.within_weight, c.omega, "omega");
            assert_eq!(fit.between.len(), c.comparisons.len());
            let first = keys.iter().map(|k| k.1).min().unwrap();
            for e in &c.comparisons {
                let kind = match e.untreated {
                    None => Comparison::TreatedVsNever {
                        adoption: e.treated,
                    },
                    Some(h) if e.treated == first => Comparison::LaterVsAlways { adoption: h },
                    Some(h) => Comparison::BothTreated {
                        earlier: e.treated,
                        later: h,
                    },
                };
                let a = fit.between.iter().find(|a| a.comparison == kind).unwrap();
                near(a.estimate, e.estimate, "estimate");
                near(a.weight, e.weight, "weight");
            }
        }
        eprintln!("{name}: adjusted decomposition passed");
    }
}

#[test]
fn adjusted_boundaries() {
    use hirmos_causal_core::panel_design::bacon::Error;
    let keys: Vec<_> = (0..6).flat_map(|u| (0..6).map(move |t| (u, t))).collect();
    let matrix = DMatrix::from_fn(keys.len(), 4, |i, j| {
        let (u, t) = keys[i];
        match j {
            0 => (u as f64 * 0.4 + t as f64 * 0.7).sin(),
            1 => f64::from(u < 3 && t >= 3),
            2 => u as f64,
            _ => (u as f64 * 0.7 + t as f64 * 0.3).cos(),
        }
    });
    let p = Panel::new(&keys, matrix).unwrap();
    for controls in [vec![], vec![0], vec![1], vec![4], vec![3, 3]] {
        assert_eq!(
            p.bacon_adjusted(0, 1, &controls).unwrap_err(),
            Error::InvalidColumns
        );
    }
    // A time-invariant control is absorbed by unit effects, not silently dropped.
    assert_eq!(
        p.bacon_adjusted(0, 1, &[2]).unwrap_err(),
        Error::RankDeficientProjection
    );
}
