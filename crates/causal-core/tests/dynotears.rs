// Parity for DYNOTEARS against causalnex: same nonzero pattern and weights at optimizer tolerance
// (the inner solver is an algorithmic L-BFGS-B port, not the bitwise Fortran).
use hirmos_causal_core::dynotears::dynotears;
use nalgebra::DMatrix;
use serde_json::Value;

#[test]
fn dynotears_matches_causalnex() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/dynotears.json")).unwrap();
    for fx in root.as_array().unwrap() {
        let name = fx["name"].as_str().unwrap();
        let series: Vec<Vec<f64>> = serde_json::from_value(fx["series"].clone()).unwrap();
        let p = fx["p"].as_u64().unwrap() as usize;
        let t = series.len();
        let d = series[0].len();
        let x = DMatrix::from_fn(t - p, d, |i, j| series[p + i][j]);
        let xlags = DMatrix::from_fn(t - p, d * p, |i, j| {
            let lag = j / d + 1;
            series[p + i - lag][j % d]
        });
        let res = dynotears(
            &x,
            &xlags,
            fx["lambda_w"].as_f64().unwrap(),
            fx["lambda_a"].as_f64().unwrap(),
        );
        let w_want: Vec<Vec<f64>> = serde_json::from_value(fx["w"].clone()).unwrap();
        let a_want: Vec<Vec<f64>> = serde_json::from_value(fx["a"].clone()).unwrap();
        let mut max_dev = 0.0f64;
        for i in 0..d {
            for j in 0..d {
                max_dev = max_dev.max((res.w[(i, j)] - w_want[i][j]).abs());
            }
        }
        for i in 0..d * p {
            for j in 0..d {
                max_dev = max_dev.max((res.a[(i, j)] - a_want[i][j]).abs());
            }
        }
        assert!(max_dev < 1e-4, "{name}: max weight deviation {max_dev}");
        // Thresholded edge patterns must agree at the workflow cutoffs.
        for thr in [0.3, 0.15] {
            for i in 0..d {
                for j in 0..d {
                    assert_eq!(
                        res.w[(i, j)].abs() >= thr,
                        w_want[i][j].abs() >= thr,
                        "{name} w pattern at {thr}"
                    );
                }
            }
            for i in 0..d * p {
                for j in 0..d {
                    assert_eq!(
                        res.a[(i, j)].abs() >= thr,
                        a_want[i][j].abs() >= thr,
                        "{name} a pattern at {thr}"
                    );
                }
            }
        }
        println!("{name}: weights within {max_dev:.2e} of causalnex, patterns identical");
    }
}
