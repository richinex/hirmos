use hirmos_structural_timeseries::random::Random;
#[test]
fn invalid_truncation_preserves_the_stream() {
    for (shape, rate, cut) in [
        (0., 1., 1.),
        (1., 0., 1.),
        (1., 1., 0.),
        (f64::NAN, 1., 1.),
        (1., f64::INFINITY, 1.),
    ] {
        let mut rng = Random::new(17);
        let mut before = rng.clone();
        assert!(rng.truncated_gamma(shape, rate, cut).is_err());
        assert_eq!(rng.uniform(), before.uniform());
    }
}
#[test]
fn stream_matches_installed_boom() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/random-stream.json")).unwrap();
    for case in fixture.as_array().unwrap() {
        let seed = case["seed"].as_u64().unwrap();
        let mut rng = Random::new(seed);
        for (i, value) in case["uniform"].as_array().unwrap().iter().enumerate() {
            assert_eq!(
                rng.uniform(),
                value.as_f64().unwrap(),
                "uniform seed {seed} draw {i}"
            );
        }
        for (i, value) in case["normal"].as_array().unwrap().iter().enumerate() {
            let expected = value.as_f64().unwrap();
            assert!(
                (rng.normal() - expected).abs() < 1e-14,
                "normal seed {seed} draw {i}"
            );
        }
        for expected in case["shuffle"].as_array().unwrap() {
            let expected: Vec<usize> = serde_json::from_value(expected.clone()).unwrap();
            let mut actual: Vec<_> = (0..expected.len()).collect();
            rng.shuffle(&mut actual);
            assert_eq!(actual, expected, "shuffle seed {seed}");
        }
        for group in case["gamma"].as_array().unwrap() {
            let shape = group["shape"].as_f64().unwrap();
            for (i, value) in group["draws"].as_array().unwrap().iter().enumerate() {
                let expected = value.as_f64().unwrap();
                let actual = rng.gamma(shape, 0.5).unwrap();
                assert!(
                    (actual - expected).abs() <= 1e-13 * expected.abs().max(1e-300),
                    "gamma seed {seed}, shape {shape}, draw {i}: {actual} != {expected}"
                );
            }
        }
        for group in case["truncated"].as_array().unwrap() {
            let shape = group["shape"].as_f64().unwrap();
            let rate = group["rate"].as_f64().unwrap();
            let cut = group["cut"].as_f64().unwrap();
            for (i, value) in group["draws"].as_array().unwrap().iter().enumerate() {
                let expected = value.as_f64().unwrap();
                let actual = rng.truncated_gamma(shape, rate, cut).unwrap();
                assert!((actual-expected).abs()<=1e-11*expected.abs().max(1.),
                    "truncated seed {seed}, shape {shape}, cut {cut}, draw {i}: {actual} != {expected}");
            }
            assert_eq!(
                rng.uniform(),
                group["next"].as_f64().unwrap(),
                "truncated stream {seed} shape {shape} cut {cut}"
            );
        }
    }
}
