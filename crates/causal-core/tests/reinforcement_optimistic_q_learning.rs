use hirmos_causal_core::reinforcement::optimistic_q_learning::{UcbQ};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_optimistic_q_learning.json")).unwrap()
}

fn floats(value: &Value) -> Vec<f64> {
    serde_json::from_value(value.clone()).unwrap()
}

fn usize_of(value: &Value) -> usize {
    value.as_u64().unwrap() as usize
}

#[test]
fn ucb_q_replays_every_decision_and_value() {
    for case in fixture()["ucbq"].as_array().unwrap() {
        let mut learner = UcbQ::new(usize_of(&case["states"]), usize_of(&case["actions"]), case["delta"].as_f64().unwrap()).unwrap();
        for step in case["steps"].as_array().unwrap() {
            let state = usize_of(&step[0]);
            let action = learner.act(state);
            assert_eq!(action, usize_of(&step[1]));
            learner.update(state, action, step[2].as_f64().unwrap(), usize_of(&step[3]));
        }
        assert_eq!(learner.action_values(), floats(&case["Q"]).as_slice());
        assert_eq!(learner.visits(), floats(&case["N"]).as_slice());
    }
}
