use super::*;
use crate::{
    defaults::Scale,
    regression::{Inclusion, Slab},
};
use serde_json::Value;
fn summary(x: &[f64]) -> (f64, f64) {
    let m = x.iter().sum::<f64>() / x.len() as f64;
    let b: Vec<_> = x
        .chunks_exact(100)
        .map(|z| z.iter().sum::<f64>() / 100.)
        .collect();
    (
        m,
        (b.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (b.len() - 1) as f64 / b.len() as f64)
            .sqrt(),
    )
}
fn compare(x: &[f64], reference: &Value) {
    let (m, se) = summary(x);
    let r = reference["mean"].as_f64().unwrap();
    let rs = reference["mcse"].as_f64().unwrap();
    assert!(
        (m - r).abs() <= 6. * se.hypot(rs) + 1e-12,
        "{m} vs {r}, SE {se}/{rs}"
    );
    for i in 0..3 {
        let q = reference["quantiles"][i].as_f64().unwrap();
        let z: Vec<_> = x.iter().map(|v| if *v <= q { 1. } else { 0. }).collect();
        let (p, se) = summary(&z);
        let rp = reference["cdf"][i].as_f64().unwrap();
        let rs = reference["cdf_mcse"][i].as_f64().unwrap();
        assert!(
            (p - rp).abs() <= 6. * se.hypot(rs) + 1. / x.len() as f64,
            "cdf {p} vs {rp}"
        );
    }
}
fn sampler(p: usize, restricted: bool, support: Support) -> Sampler {
    let mean = DVector::from_fn(p, |j, _| 0.1 / (j + 1) as f64);
    let precision = DMatrix::from_diagonal(&DVector::from_fn(p, |j, _| {
        1. / (0.5 * 0.8_f64.powi(j as i32)).powi(2)
    }));
    let probs = (0..p)
        .map(|j| {
            Inclusion::new(if restricted && j == 0 {
                1.
            } else if restricted && j == p - 1 {
                0.
            } else {
                0.8 * 0.8_f64.powi(j as i32)
            })
            .unwrap()
        })
        .collect();
    let mut prior = Slab::new(mean, precision, probs, Scale::new(0.5).unwrap(), 10.).unwrap();
    if restricted {
        prior = prior.with_residual_ceiling(Scale::new(0.5).unwrap());
    }
    let mut sampler = Sampler::new(UnscaledSlab::new(prior), support, 314159);
    if restricted {
        sampler =
            sampler.with_selection(Selection::Sweep(FlipSweep::AtMost(1.try_into().unwrap())));
    }
    sampler
}
#[test]
fn full_sparse_sampler_matches_original_boom_including_forced_fallback() {
    let f: Value =
        serde_json::from_str(include_str!("../../fixtures/sparse-ar-sampler.json")).unwrap();
    for c in f["sampling"].as_array().unwrap() {
        let p = c["order"].as_u64().unwrap() as usize;
        let coordinate = c["coordinate"].as_bool().unwrap();
        let support = if c["truncate"].as_bool().unwrap() {
            Support::ReferenceStationary
        } else {
            Support::Unrestricted
        };
        let mut s = sampler(p, c["restricted"].as_bool().unwrap(), support);
        if coordinate {
            s = s.with_selection(Selection::Fixed);
        }
        let x = DMatrix::from_fn(80, p, |i, j| (i as f64 * (0.3 + j as f64 * 0.41)).sin());
        let y = DVector::from_fn(80, |i, _| {
            0.8 * (i as f64 * 1.71).cos()
                + (0..p)
                    .map(|j| 0.4 / (j + 1) as f64 * x[(i, j)])
                    .sum::<f64>()
        });
        let mut columns = vec![vec![]; 1 + 2 * p];
        for i in 0..40000 {
            s.step_with_budget(&x, &y, if coordinate { 0 } else { 100 })
                .unwrap();
            if i < 10000 {
                continue;
            }
            columns[0].push(s.variance.value());
            for j in 0..p {
                columns[j + 1].push(s.coefficients.values[j]);
                columns[j + p + 1].push(if s.coefficients.included[j] { 1. } else { 0. });
            }
        }
        for (j, x) in columns.iter().enumerate() {
            compare(x, &c["metrics"][j]);
        }
    }
}
#[test]
fn sparse_failure_preserves_both_rngs_and_mask() {
    let mut a = sampler(2, false, Support::ReferenceStationary);
    let mut b = a.clone();
    let x = DMatrix::from_fn(40, 2, |i, j| (i as f64 * (0.3 + j as f64)).sin());
    let y = DVector::from_fn(40, |i, _| (i as f64 * 0.7).cos());
    assert!(a.step(&x, &DVector::repeat(40, f64::MAX)).is_err());
    for _ in 0..20 {
        a.step(&x, &y).unwrap();
        b.step(&x, &y).unwrap();
        assert_eq!(a.coefficients.values, b.coefficients.values);
        assert_eq!(a.coefficients.included, b.coefficients.included);
        assert_eq!(a.variance.value(), b.variance.value());
    }
}
