use hirmos_causal_core::grf::sampling::{Clusters, Sampler};
use serde_json::Value;
use std::collections::BTreeSet;

fn ids(value: &Value) -> Vec<usize> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect()
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn exact_upstream_sampling_streams() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/sampling.json")).unwrap();
    assert_eq!(fixture["version"], "2.6.1");
    for case in fixture["cases"].as_array().unwrap() {
        let seed = case["seed"].as_u64().unwrap() as u32;
        let mut rng = Sampler::new(seed);
        let actual: Vec<_> = (0..1000)
            .flat_map(|_| [0, 1, 4, 9, 10, 11, 20, 100, 1000])
            .map(|mean| rng.poisson(mean))
            .collect();
        assert_eq!(
            actual,
            ids(&case["expected"]["poisson"]),
            "Poisson seed {seed}"
        );
        let mut rng = Sampler::new(seed);
        for (round, expected) in case["expected"]["rounds"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            let sample = rng.sample(37, 0.51).unwrap();
            assert_eq!(
                sample,
                ids(&expected["sample"]),
                "seed {seed} round {round}"
            );
            let (split, rest) = rng.subsample(&sample, 0.55).unwrap();
            assert_eq!(split, ids(&expected["split"]));
            assert_eq!(rest, ids(&expected["rest"]));
            assert_eq!(
                rng.draw(100, &BTreeSet::from([0, 2, 41, 99]), 7).unwrap(),
                ids(&expected["sparse"])
            );
            assert_eq!(
                rng.draw(19, &BTreeSet::from([1, 7, 9]), 10).unwrap(),
                ids(&expected["dense"])
            );
            assert_eq!(rng.draw(1, &BTreeSet::new(), 1).unwrap(), vec![0]);
        }
        let clusters =
            Clusters::new(&[9, 9, 2, 7, 2, 9, 7, 7, 7, 1, 9, 2, 1, 9, 7, 1, 2], 2).unwrap();
        let mut rng = Sampler::new(seed);
        for expected in case["expected"]["clusters"].as_array().unwrap() {
            let selected = rng.sample_clusters(&clusters, 0.75).unwrap();
            assert_eq!(selected, ids(&expected["ids"]));
            assert_eq!(
                rng.rows_in_clusters(&clusters, &selected, true).unwrap(),
                ids(&expected["selected"])
            );
            assert_eq!(
                rng.rows_in_clusters(&clusters, &selected, false).unwrap(),
                ids(&expected["all"])
            );
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn sampling_boundaries() {
    let mut rng = Sampler::new(7);
    assert!(rng.index(0).is_err());
    assert!(rng.sample(8, f64::NAN).is_err());
    assert!(rng.subsample(&[0, 1], 1.01).is_err());
    assert!(rng.draw(2, &BTreeSet::from([2]), 1).is_err());
    assert!(rng.draw(2, &BTreeSet::from([1]), 2).is_err());
    assert!(rng.sample(0, 0.5).unwrap().is_empty());
    assert_eq!(rng.subsample(&[], 0.5).unwrap(), (vec![], vec![]));
    assert_eq!(rng.index(1).unwrap(), 0);
    assert!(Clusters::new(&[], 2).is_err());
    assert!(Clusters::new(&[0], 0).is_err());
    let clusters = Clusters::new(&[0], 1).unwrap();
    assert!(rng.rows_in_clusters(&clusters, &[1], true).is_err());
}
