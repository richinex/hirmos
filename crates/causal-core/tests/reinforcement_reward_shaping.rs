use hirmos_causal_core::reinforcement::reward_shaping::{
    optimal_value_upper_bound, shaped_q_ucb, EpisodicEnvironment, LoggedStep, Shaping,
};
use hirmos_causal_core::reinforcement::windy_grid::{Layout, WindyGrid};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_reward_shaping.json")).unwrap()
}

fn flat(value: &Value) -> Vec<f64> {
    match value {
        Value::Array(items) => items.iter().flat_map(flat).collect(),
        number => vec![number.as_f64().unwrap()],
    }
}

fn cell(value: &Value) -> usize {
    value[0].as_u64().unwrap() as usize * 9 + value[1].as_u64().unwrap() as usize
}

#[test]
fn seeded_episodes_follow_the_recorded_rollouts_on_every_layout() {
    let fixture = fixture();
    for (name, layout) in [
        ("easy", Layout::Easy),
        ("hard", Layout::Hard),
        ("extreme", Layout::Extreme),
        ("maze", Layout::Maze),
        ("maze2", Layout::MazeComplex),
    ] {
        let mut grid = WindyGrid::new(layout);
        let recorded = &fixture["layouts"][name];
        for x in 0..9 {
            for y in 0..9 {
                assert_eq!(grid.wind_distribution(x, y).to_vec(), flat(&recorded["wind"][x][y]), "{name} wind at ({x}, {y})");
            }
        }
        for rollout in recorded["rollouts"].as_array().unwrap() {
            assert_eq!(grid.reset(rollout["seed"].as_u64().unwrap()), cell(&rollout["start"]), "{name} start");
            assert_eq!(grid.direction() as u64, rollout["direction"].as_u64().unwrap());
            assert_eq!(grid.wind() as u64, rollout["wind"].as_u64().unwrap());
            for expected in rollout["steps"].as_array().unwrap() {
                let step = grid.step(expected["action"].as_u64().unwrap() as usize);
                assert_eq!(step.next, cell(&expected["state"]), "{name} seed {}", rollout["seed"]);
                assert_eq!(step.reward, expected["reward"].as_f64().unwrap());
                assert_eq!(step.terminated, expected["terminated"].as_bool().unwrap());
                assert_eq!(step.truncated, expected["truncated"].as_bool().unwrap());
                assert_eq!(grid.wind() as u64, expected["wind"].as_u64().unwrap());
            }
        }
    }
}

#[test]
fn value_iteration_reproduces_the_published_optimal_values() {
    let fixture = fixture();
    let (values, action_values) = WindyGrid::new(Layout::Extreme).optimal_values();
    assert_eq!(values, flat(&fixture["optimal_values"]));
    assert_eq!(action_values, flat(&fixture["optimal_qvalues"]));
}

#[test]
fn bounds_from_each_behaviour_log_match_the_recorded_bounds() {
    let fixture = fixture();
    let actionable = WindyGrid::new(Layout::Extreme).actionable();
    for case in fixture["bounds"].as_array().unwrap() {
        let log: Vec<LoggedStep> = case["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| LoggedStep {
                state: cell(&row[0]),
                action: row[1].as_u64().unwrap() as usize,
                reward: row[2].as_f64().unwrap(),
                next: cell(&row[3]),
            })
            .collect();
        let bound = optimal_value_upper_bound(&log, &actionable, 5, 20, 0.0);
        assert_eq!(bound, flat(&case["bound"]), "{} policy", case["policy"]);
    }
}

#[test]
fn every_shaping_variant_matches_the_recorded_learning_run() {
    let fixture = fixture();
    let optimal_values = flat(&fixture["optimal_values"]);
    let optimal_action_values = flat(&fixture["optimal_qvalues"]);
    for run in fixture["runs"].as_array().unwrap() {
        let mode = run["mode"].as_u64().unwrap();
        let shaping = Shaping { potential: mode == 1 || mode == 5, clip_to_bound: mode == 1 || mode == 4 };
        let mut grid = WindyGrid::new(Layout::Extreme);
        let result = shaped_q_ucb(&mut grid, &flat(&run["upper_value"]), 5, 20, 300, 1234, 3, shaping);
        assert_eq!(result.action_values, flat(&run["q_table"]), "mode {mode}");
        assert_eq!(result.visits, flat(&run["visits"]), "mode {mode}");
        assert_eq!(result.rewards, flat(&run["rewards"]), "mode {mode}");
        let regret = result.cumulative_regret(&optimal_values, &optimal_action_values, 5);
        assert_eq!(regret, flat(&run["regrets"]), "mode {mode}");
        assert_eq!(result.steps.len() as u64, run["total_steps"].as_u64().unwrap());
    }
}

#[test]
fn twenty_thousand_episodes_reproduce_the_notebook_rewards_and_regrets() {
    let fixture = fixture();
    let optimal_values = flat(&fixture["optimal_values"]);
    let optimal_action_values = flat(&fixture["optimal_qvalues"]);
    let bound = flat(&fixture["runs"][1]["upper_value"]);
    let unshaped = Shaping { potential: false, clip_to_bound: false };
    let shaped = Shaping { potential: true, clip_to_bound: true };
    for (shaping, upper, reward, regret) in [(unshaped, vec![0.0; 81], -1.783, 35342.837), (shaped, bound, -0.458, 696.588)] {
        let run = shaped_q_ucb(&mut WindyGrid::new(Layout::Extreme), &upper, 5, 20, 20_000, 1234, 3, shaping);
        let last_hundred = run.rewards[19_900..].iter().sum::<f64>() / 100.0;
        let total = run.cumulative_regret(&optimal_values, &optimal_action_values, 5)[19_999];
        assert_eq!(((last_hundred * 1000.0).round() / 1000.0, (total * 1000.0).round() / 1000.0), (reward, regret));
    }
}
