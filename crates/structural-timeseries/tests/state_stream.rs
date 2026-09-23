use hirmos_structural_timeseries::{
    gaussian,
    state::{Component, Normal, Season, System, Variance},
};
use serde_json::Value;

#[test]
fn frozen_state_update_replays_compiled_boom_draw() {
    let cases: Value = serde_json::from_str(include_str!("../fixtures/impact-state.json")).unwrap();
    let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-9 * b.abs().max(1.), "{a} != {b}");
    for case in cases.as_array().unwrap() {
        let p: Vec<_> = case["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        let variance = |v| Variance::new(v).unwrap();
        let system = System::new(vec![
            Component::Level {
                innovation: variance(p[0]),
                initial: Normal::new(p[3], variance(1.)).unwrap(),
            },
            Component::Seasonal {
                season: Season::new(12, case["duration"].as_u64().unwrap() as usize).unwrap(),
                innovation: variance(p[1]),
                initial: Normal::new(0., variance(1.)).unwrap(),
            },
        ])
        .unwrap();
        let y: Vec<_> = case["target"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64())
            .collect();
        let mut rng = hirmos_structural_timeseries::random::Random::new(7129);
        let replay = gaussian::draw_states(&system, variance(p[2]), &y, &mut rng).unwrap();
        for t in 0..y.len() {
            for j in 0..12 {
                near(replay[t][j], case["draw"][t][j].as_f64().unwrap());
            }
        }
        let filtered = gaussian::filter(&system, variance(p[2]), &y).unwrap();
        near(
            filtered.log_likelihood,
            case["likelihood"].as_f64().unwrap(),
        );
        let result = gaussian::smooth(&filtered).unwrap();
        let simulated: Vec<_> = case["simulated"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64())
            .collect();
        let simulated_filter = gaussian::filter(&system, variance(p[2]), &simulated).unwrap();
        let simulated_result = gaussian::smooth(&simulated_filter).unwrap();
        for t in 0..y.len() {
            for j in 0..12 {
                let actual = case["prior"][t][j].as_f64().unwrap() + result.means[t][j]
                    - simulated_result.means[t][j];
                near(actual, case["draw"][t][j].as_f64().unwrap());
            }
        }
    }
}
