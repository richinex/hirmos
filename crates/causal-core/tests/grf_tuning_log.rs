use hirmos_causal_core::grf::tuning::Parameter;
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn penalty_transform_matches_pinned_r_bits() {
    let f: serde_json::Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/tuning-log.json")).unwrap();
    assert_eq!(f["input"].as_array().unwrap().len(), 11_003);
    assert_eq!(f["expected"].as_array().unwrap().len(), 11_003);
    for (input, expected) in f["input"]
        .as_array()
        .unwrap()
        .iter()
        .zip(f["expected"].as_array().unwrap())
    {
        let x: f64 = input.as_str().unwrap().parse().unwrap();
        let y: f64 = expected.as_str().unwrap().parse().unwrap();
        let actual = Parameter::ImbalancePenalty.transform(1000, 10, x).unwrap();
        assert_eq!(
            actual.to_bits(),
            y.to_bits(),
            "-log({x}) = {actual}, R = {y}"
        );
    }
}
