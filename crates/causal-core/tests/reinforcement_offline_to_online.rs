use hirmos_causal_core::reinforcement::offline_to_online::{
    learn_online, natural_bounds, natural_bounds_from_cells, ConfoundedBandit, Interval, LoggedCell,
    LoggedPull, OfflineUse, ReplayedBandit, ThresholdBandit, UnitReward,
};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_offline_to_online.json")).unwrap()
}

fn floats(value: &Value) -> Vec<f64> {
    serde_json::from_value(value.clone()).unwrap()
}

fn seed(case: &Value, name: &str) -> u64 {
    case["seeds"][name].as_u64().unwrap()
}

fn log(case: &Value) -> Vec<LoggedPull> {
    let delta = case["delta"].as_f64().unwrap();
    let first = seed(case, "bounds");
    (0..case["samples"].as_u64().unwrap())
        .map(|k| {
            let mut bandit = ThresholdBandit::new(delta, first + k);
            bandit.reset();
            bandit.see()
        })
        .collect()
}

fn intervals(case: &Value) -> Vec<Interval> {
    floats(&case["lower"])
        .into_iter()
        .zip(floats(&case["upper"]))
        .map(|(lower, upper)| Interval { lower, upper })
        .collect()
}

#[test]
fn natural_bounds_match_the_recorded_bounds_bit_for_bit() {
    for case in fixture()["cases"].as_array().unwrap() {
        let bounds = natural_bounds(2, &log(case)).unwrap();
        let pulls: Vec<f64> = bounds.iter().map(|arm| arm.pulls as f64).collect();
        let sums: Vec<f64> = bounds.iter().map(|arm| arm.reward_sum).collect();
        let found: Vec<Interval> = bounds.iter().map(|arm| arm.interval).collect();
        assert_eq!(pulls, floats(&case["observational_counts"]));
        assert_eq!(sums, floats(&case["observational_rewards"]));
        assert_eq!(found, intervals(case));
    }
}

#[test]
fn learners_replay_the_reference_trajectories() {
    for case in fixture()["cases"].as_array().unwrap() {
        let delta = case["delta"].as_f64().unwrap();
        let timesteps = case["timesteps"].as_u64().unwrap() as usize;
        let episodes = case["samples"].as_u64().unwrap() as usize;
        let runs = [
            ("plain", OfflineUse::Ignore, "ucb_rewards"),
            ("clipped", OfflineUse::Clip(intervals(case)), "clipped_ucb_rewards"),
            ("transfer", OfflineUse::CopyObservations { episodes }, "direct_transfer_rewards"),
        ];
        for (seed_name, offline, expected) in runs {
            let mut bandit = ThresholdBandit::new(delta, seed(case, seed_name));
            let rewards: Vec<f64> = learn_online(&mut bandit, &offline, timesteps).unwrap().iter().map(|step| step.reward).collect();
            assert_eq!(rewards, floats(&case[expected]), "{expected}, delta {delta}");
        }
    }
}

#[test]
fn estimated_bounds_sit_within_sampling_error_of_population_bounds_that_contain_the_truth() {
    for case in fixture()["cases"].as_array().unwrap() {
        let delta = case["delta"].as_f64().unwrap();
        let samples = case["samples"].as_f64().unwrap();
        let truth = [0.4, 0.4 - delta];
        let population = [Interval { lower: 0.0, upper: 0.8 }, Interval { lower: 0.4 - delta, upper: 0.6 - delta }];
        let tolerance = 4.0 * (0.25 / samples).sqrt();
        for (arm, estimate) in intervals(case).iter().enumerate() {
            assert!(population[arm].lower <= truth[arm] && truth[arm] <= population[arm].upper);
            assert!((estimate.lower - population[arm].lower).abs() <= tolerance, "lower, arm {arm}, delta {delta}");
            assert!((estimate.upper - population[arm].upper).abs() <= tolerance, "upper, arm {arm}, delta {delta}");
        }
    }
}

fn cell(arm: usize, reward: f64, count: u64) -> LoggedCell {
    LoggedCell { arm, reward: UnitReward::new(reward).unwrap(), count }
}

#[test]
fn counted_cells_give_the_bounds_of_the_rows_they_stand_for() {
    let cells = [cell(0, 1.0, 3), cell(0, 0.0, 5), cell(1, 1.0, 1), cell(1, 0.0, 11), cell(2, 1.0, 0), cell(2, 0.0, 4)];
    let rows: Vec<LoggedPull> = cells
        .iter()
        .flat_map(|c| (0..c.count).map(move |_| LoggedPull { arm: c.arm, reward: c.reward }))
        .collect();
    assert_eq!(natural_bounds_from_cells(3, &cells).unwrap(), natural_bounds(3, &rows).unwrap());
}

#[test]
fn replayed_interventions_reproduce_each_arms_randomised_mean() {
    let logged = [cell(0, 1.0, 90), cell(1, 0.0, 10)];
    let randomised = [cell(0, 1.0, 2), cell(0, 0.0, 8), cell(1, 1.0, 7), cell(1, 0.0, 3)];
    let mut bandit = ReplayedBandit::new(2, &logged, &randomised, 11).unwrap();
    let draws = 200_000;
    for (arm, mean) in [(0, 0.2), (1, 0.7)] {
        let total: f64 = (0..draws).map(|_| bandit.intervene(arm).value()).sum();
        // Four binomial standard errors at p = 0.5 is about 0.0045.
        assert!((total / draws as f64 - mean).abs() < 0.0045, "arm {arm}");
    }
    let seen = (0..draws).filter(|_| bandit.see().arm == 0).count() as f64 / draws as f64;
    assert!((seen - 0.9).abs() < 0.0045);
}

#[test]
fn an_arm_without_randomised_rows_is_refused() {
    assert!(ReplayedBandit::new(2, &[cell(0, 1.0, 1)], &[cell(0, 1.0, 1), cell(1, 0.0, 0)], 1).is_err());
}

#[test]
fn clipping_to_bounds_spends_fewer_pulls_on_the_worse_arm_than_learning_from_scratch() {
    // The worse arm's upper bound, 0.55, lies below the better arm's randomised mean, 0.9.
    let logged = [cell(0, 1.0, 50), cell(1, 1.0, 5), cell(1, 0.0, 45)];
    let randomised = [cell(0, 1.0, 9), cell(0, 0.0, 1), cell(1, 1.0, 1), cell(1, 0.0, 9)];
    let intervals: Vec<Interval> = natural_bounds_from_cells(2, &logged).unwrap().iter().map(|b| b.interval).collect();
    assert!(intervals[1].upper < 0.9);
    let worse_pulls = |offline: OfflineUse| {
        let mut bandit = ReplayedBandit::new(2, &logged, &randomised, 3).unwrap();
        learn_online(&mut bandit, &offline, 2_000).unwrap().iter().filter(|step| step.arm == 1).count()
    };
    let scratch = worse_pulls(OfflineUse::Ignore);
    let clipped = worse_pulls(OfflineUse::Clip(intervals));
    assert!(clipped < scratch, "clipped {clipped}, scratch {scratch}");
}
