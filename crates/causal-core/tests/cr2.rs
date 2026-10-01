use hirmos_causal_core::design::{hierarchical_design, Column, Encoding, Term, Values};
use hirmos_causal_core::estimation::{cr2, inference};
use nalgebra::DMatrix;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Case {
    input: Vec<Input>,
    term_inputs: Vec<Vec<usize>>,
    full_x: Vec<Vec<f64>>,
    keep: Vec<usize>,
    x: Vec<Vec<f64>>,
    y: Vec<f64>,
    weights: Vec<f64>,
    clusters: Vec<u64>,
    params: Vec<f64>,
    covariance: Vec<Vec<f64>>,
    se: Vec<f64>,
    df: Vec<f64>,
    pvalues: Vec<f64>,
    intervals: Vec<[f64; 2]>,
    terms: Vec<String>,
}
#[derive(Deserialize)]
struct Input {
    name: String,
    categorical: bool,
    numeric: bool,
    values: serde_json::Value,
    reference: String,
}
#[derive(Deserialize)]
struct Fixture {
    cases: BTreeMap<String, Case>,
}
fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= 1e-8 * b.abs().max(1.),
        "{label}: {a:.15e} != {b:.15e}, delta={:.6e}",
        (a - b).abs()
    );
}
#[test]
fn estimatr_oracle() {
    let fixtures: Fixture =
        serde_json::from_str(include_str!("../oracle/fixtures/mixtape_estimatr.json")).unwrap();
    for (name, c) in fixtures.cases {
        let n = c.y.len();
        let p = c.params.len();
        let columns: Vec<_> = c
            .input
            .iter()
            .map(|v| Column {
                name: v.name.clone(),
                values: if v.numeric {
                    Values::Numeric(serde_json::from_value(v.values.clone()).unwrap())
                } else {
                    Values::Text(serde_json::from_value(v.values.clone()).unwrap())
                },
                encoding: if v.categorical {
                    Encoding::TreatmentContrast
                } else {
                    Encoding::Numeric
                },
            })
            .collect();
        let references = c
            .input
            .iter()
            .enumerate()
            .filter(|(_, v)| v.categorical)
            .map(|(i, v)| (i, v.reference.clone()))
            .collect();
        let terms: Vec<_> = c
            .term_inputs
            .iter()
            .map(|v| match v.as_slice() {
                [a] => Term::Main(*a),
                [a, b] => Term::Pair(*a, *b),
                [a, b, c] => Term::Triple(*a, *b, *c),
                _ => panic!("unsupported oracle term"),
            })
            .collect();
        let design = hierarchical_design(&columns, &terms, &references).unwrap();
        assert_eq!(design.rows.len(), c.full_x.len());
        for (i, (actual, expected)) in design.rows.iter().zip(&c.full_x).enumerate() {
            assert_eq!(actual.len(), expected.len());
            for (j, (a, b)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(a, b, "{name}: full design [{i},{j}]");
            }
        }
        let full = DMatrix::from_fn(n, design.names.len(), |i, j| design.rows[i][j]);
        let selected = cr2::fit_selecting_aliases(&c.y, &full, &c.weights, &c.clusters).unwrap();
        assert_eq!(
            selected.selection.kept(),
            c.keep,
            "{name}: automatic alias selection"
        );
        let x = DMatrix::from_fn(n, p, |i, j| full[(i, selected.selection.kept()[j])]);
        assert_eq!(x, DMatrix::from_fn(n, p, |i, j| c.x[i][j]));
        let fit = selected.fit;
        assert_eq!(
            fit.observations,
            c.weights.iter().filter(|w| **w > 0.).count()
        );
        for j in 0..p {
            let label = format!("{name}/{}", c.terms[j]);
            near(fit.params[j], c.params[j], &format!("{label}/beta"));
            near(fit.degrees_of_freedom[j], c.df[j], &format!("{label}/df"));
            near(
                fit.covariance[(j, j)].sqrt(),
                c.se[j],
                &format!("{label}/se"),
            );
            for k in 0..p {
                near(
                    fit.covariance[(j, k)],
                    c.covariance[j][k],
                    &format!("{label}/cov{k}"),
                );
            }
            // Reuse inference with a scalar projection; covariance parity is checked above.
            let contrast = inference::contrast(
                &[fit.params[j]],
                &DMatrix::from_element(1, 1, fit.covariance[(j, j)]),
                &[1.],
                0.,
                0.95,
                inference::Reference::Student {
                    degrees_of_freedom: fit.degrees_of_freedom[j],
                },
            )
            .unwrap();
            near(contrast.p_value, c.pvalues[j], &format!("{label}/p"));
            for k in 0..2 {
                near(
                    contrast.interval[k],
                    c.intervals[j][k],
                    &format!("{label}/ci{k}"),
                );
            }
        }
        eprintln!("{name}: {n} rows, {p} retained terms passed");
    }
}

#[test]
fn boundaries_and_invariances() {
    let x = DMatrix::from_fn(
        24,
        2,
        |i, j| if j == 0 { 1. } else { (i as f64 * 0.7).sin() },
    );
    let y: Vec<f64> = (0..24)
        .map(|i| 2. + 0.3 * x[(i, 1)] + (i as f64 * 0.3).cos())
        .collect();
    let w: Vec<f64> = (0..24).map(|i| 1. + (i % 4) as f64).collect();
    let cl: Vec<u64> = (0..24).map(|i| (i / 3) as u64).collect();
    let a = cr2::fit(&y, &x, &w, &cl).unwrap();
    // A zero-weight row in a new cluster must not count or alter inference.
    let padded_x = DMatrix::from_fn(25, 2, |i, j| if i < 24 { x[(i, j)] } else { 17. });
    let mut padded_y = y.clone();
    padded_y.push(999.);
    let mut padded_w = w.clone();
    padded_w.push(0.);
    let mut padded_cl = cl.clone();
    padded_cl.push(999);
    let zero = cr2::fit(&padded_y, &padded_x, &padded_w, &padded_cl).unwrap();
    assert_eq!(zero.observations, 24);
    assert_eq!(zero.clusters, 8);
    assert_eq!(zero.residuals.len(), 25);
    let b = cr2::fit(&y, &x, &w.iter().map(|v| v * 137.).collect::<Vec<_>>(), &cl).unwrap();
    let reversed = DMatrix::from_fn(24, 2, |i, j| x[(23 - i, j)]);
    let c = cr2::fit(
        &y.iter().rev().copied().collect::<Vec<_>>(),
        &reversed,
        &w.iter().rev().copied().collect::<Vec<_>>(),
        &cl.iter().rev().map(|g| g + 99).collect::<Vec<_>>(),
    )
    .unwrap();
    for fit in [b, c, zero] {
        for j in 0..2 {
            near(fit.params[j], a.params[j], "beta invariant");
            near(
                fit.degrees_of_freedom[j],
                a.degrees_of_freedom[j],
                "df invariant",
            );
            for k in 0..2 {
                near(
                    fit.covariance[(j, k)],
                    a.covariance[(j, k)],
                    "cov invariant",
                );
            }
        }
    }
    assert_eq!(
        cr2::fit(&y, &x, &w[..23], &cl).unwrap_err(),
        cr2::Error::Shape
    );
    for bad in [-1., f64::NAN, f64::INFINITY] {
        let mut ws = w.clone();
        ws[0] = bad;
        assert_eq!(
            cr2::fit(&y, &x, &ws, &cl).unwrap_err(),
            cr2::Error::InvalidWeights
        );
    }
    assert_eq!(
        cr2::fit(&y, &x, &vec![0.; 24], &cl).unwrap_err(),
        cr2::Error::InvalidWeights
    );
    assert_eq!(
        cr2::fit(&y, &x, &w, &vec![0; 24]).unwrap_err(),
        cr2::Error::InsufficientClusters
    );
    assert_eq!(
        cr2::fit(&y, &DMatrix::from_element(24, 2, 1.), &w, &cl).unwrap_err(),
        cr2::Error::RankDeficient
    );
    let mut ys = y.clone();
    ys[0] = f64::NAN;
    assert_eq!(
        cr2::fit(&ys, &x, &w, &cl).unwrap_err(),
        cr2::Error::NonFinite
    );
}
