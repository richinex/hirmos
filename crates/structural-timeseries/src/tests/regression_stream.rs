use super::*;
use crate::defaults;
use nalgebra::{DMatrix, DVector};
#[test]
fn consecutive_ssvs_draws_match_boom_from_identical_sampler_seeds() {
    let f: serde_json::Value =
        serde_json::from_str(include_str!("../../fixtures/regression-stream.json")).unwrap();
    let y: Vec<f64> = serde_json::from_value(f["y"].clone()).unwrap();
    let x = DMatrix::from_fn(y.len(), 2, |i, j| f["x"][i][j].as_f64().unwrap());
    let observed: Vec<_> = y.iter().copied().map(Some).collect();
    let slab = defaults::bsts_regression(&x, &observed).unwrap();
    for chain in f["chains"].as_array().unwrap() {
        let seed = chain["seed"].as_u64().unwrap();
        let mut sampler = Regression::new(
            x.clone(),
            DVector::from_vec(y.clone()),
            slab.clone(),
            seed as u32,
        )
        .unwrap();
        for (i, expected) in chain["draws"].as_array().unwrap().iter().enumerate() {
            let draw = sampler.step().unwrap();
            let mask = u64::from(draw.included[0]) + 2 * u64::from(draw.included[1]);
            assert_eq!(
                mask,
                expected[3].as_u64().unwrap(),
                "mask seed {seed} draw {i}"
            );
            // jsonlite writes the R double with fewer than 17 significant digits.
            assert!(
                (sampler.rng.clone().uniform() - expected[4].as_f64().unwrap()).abs() < 1e-15,
                "stream seed {seed} draw {i}"
            );
            for (j, actual) in draw
                .coefficients
                .iter()
                .chain([draw.variance].iter())
                .enumerate()
            {
                let reference = expected[j].as_f64().unwrap();
                assert!(
                    (actual - reference).abs() < 1e-9 * reference.abs().max(1.),
                    "seed {seed} draw {i} field {j}: {actual} != {reference}"
                );
            }
        }
    }
}
