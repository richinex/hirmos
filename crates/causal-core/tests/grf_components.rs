use hirmos_causal_core::grf::prediction::{predict, variance, CausalData, Moments};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Fixture {
    version: String,
    commit: String,
    cases: BTreeMap<String, Case>,
}
#[derive(Deserialize)]
struct Case {
    y: Vec<f64>,
    w: Vec<f64>,
    weights: Vec<f64>,
    samples: Vec<usize>,
    indices: Vec<Vec<usize>>,
    group_size: usize,
    expected: Expected,
}
#[derive(Deserialize)]
struct Expected {
    stopped: bool,
    responses: Option<Vec<f64>>,
    leaves: Vec<Option<Moments>>,
    average: Vec<Option<f64>>,
    prediction: Option<f64>,
    variance: Option<f64>,
}

fn close(actual: f64, expected: f64, context: &str) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= 2e-12 * expected.abs().max(1.0),
        "{context}: Rust {actual:.17e}, GRF {expected:.17e}"
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn pinned_cpp_component_parity() {
    let fixture: Fixture =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/components.json")).unwrap();
    assert_eq!(fixture.version, "2.6.1");
    assert_eq!(fixture.commit, "1564220351aa1c07e7d412f10d63b129db3e56b6");
    assert_eq!(fixture.cases.len(), 8);
    for (name, c) in fixture.cases {
        let data = CausalData::new(&c.y, &c.w, &c.weights).unwrap();
        let responses = data.relabel(&c.samples).unwrap();
        assert_eq!(responses.is_none(), c.expected.stopped, "{name}");
        match (responses, c.expected.responses) {
            (Some(a), Some(b)) => {
                assert_eq!(a.len(), b.len());
                for (a, b) in a.into_iter().zip(b) {
                    close(a, b, &name);
                }
            }
            (None, None) => {}
            _ => panic!("{name}: mismatched relabeling outcome"),
        }
        let leaves: Vec<_> = c
            .indices
            .iter()
            .map(|s| data.leaf_moments(s).unwrap())
            .collect();
        assert_eq!(leaves.len(), c.expected.leaves.len());
        for (a, b) in leaves.iter().zip(&c.expected.leaves) {
            match (a, b) {
                (Some(a), Some(b)) => {
                    for j in 0..7 {
                        close(a[j], b[j], &name);
                    }
                }
                (None, None) => {}
                _ => panic!("{name}: mismatched empty leaf"),
            }
        }
        let count = leaves.iter().flatten().count();
        if count == 0 {
            assert!(c.expected.prediction.is_none());
            assert!(c.expected.variance.is_none());
            continue;
        }
        let mut average = [0.0; 7];
        for leaf in leaves.iter().flatten() {
            for j in 0..7 {
                average[j] += leaf[j];
            }
        }
        for j in 0..7 {
            average[j] /= count as f64;
            close(average[j], c.expected.average[j].unwrap(), &name);
        }
        match c.expected.prediction {
            Some(x) => close(predict(&average), x, &name),
            None => assert!(!predict(&average).is_finite(), "{name}"),
        }
        let actual = variance(&average, &leaves, c.group_size).unwrap();
        match c.expected.variance {
            Some(x) => close(actual, x, &name),
            None => assert!(!actual.is_finite(), "{name}"),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_inputs_are_not_silently_repaired() {
    assert!(CausalData::new(&[], &[], &[]).is_err());
    assert!(CausalData::new(&[1.0], &[0.0, 1.0], &[1.0]).is_err());
    assert!(CausalData::new(&[f64::NAN], &[0.0], &[1.0]).is_err());
    assert!(CausalData::new(&[1.0], &[0.0], &[-1.0]).is_err());
    let data = CausalData::new(&[1.0], &[0.0], &[1.0]).unwrap();
    assert!(data.relabel(&[1]).is_err());
    assert!(data.leaf_moments(&[1]).is_err());
    assert!(data.leaf_moments(&[]).unwrap().is_none());
    assert!(variance(&[0.0; 7], &[], 1).is_err());
    assert!(variance(&[0.0; 7], &[None, None], 2).is_err());
}
