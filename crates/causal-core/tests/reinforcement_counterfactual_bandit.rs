use hirmos_causal_core::reinforcement::counterfactual_bandit::{
    CounterfactualUcb, GreedyCasino, Machine, Ucb,
};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_counterfactual_bandit.json")).unwrap()
}

fn integers(value: &Value) -> Vec<u64> {
    serde_json::from_value(value.clone()).unwrap()
}

struct CounterfactualRun {
    wins: Vec<u64>,
    natural: Vec<u64>,
    played: Vec<u64>,
    seen: Vec<u64>,
    learner: CounterfactualUcb,
}

fn counterfactual_run(episodes: u64, exploration: f64, seed: u64) -> CounterfactualRun {
    let casino = GreedyCasino::default();
    let mut learner = CounterfactualUcb::new(2, exploration).unwrap();
    let (mut wins, mut natural, mut played, mut seen) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for k in 0..episodes {
        seen.push(u64::from(casino.episode(seed + k).see().won));
        let step = casino.episode(seed + k).counterfactual(|intention| {
            Machine::from_index(learner.select_arm(intention.index())).unwrap()
        });
        learner.update(step.natural.index(), step.played.index(), f64::from(u8::from(step.won)));
        wins.push(u64::from(step.won));
        natural.push(step.natural.index() as u64);
        played.push(step.played.index() as u64);
    }
    CounterfactualRun { wins, natural, played, seen, learner }
}

fn baseline_run(episodes: u64, exploration: f64, seed: u64) -> (Vec<u64>, Vec<u64>, Ucb) {
    let casino = GreedyCasino::default();
    let mut learner = Ucb::new(2, exploration).unwrap();
    let (mut wins, mut played) = (Vec::new(), Vec::new());
    for k in 0..episodes {
        let arm = learner.select_arm();
        let step = casino.episode(seed + k).intervene(Machine::from_index(arm).unwrap());
        learner.update(arm, f64::from(u8::from(step.won)));
        wins.push(u64::from(step.won));
        played.push(arm as u64);
    }
    (wins, played, learner)
}

#[test]
fn counterfactual_and_baseline_ucb_replay_the_reference_runs() {
    let fixture = fixture();
    for run in fixture["runs"].as_array().unwrap() {
        let config = &run["config"];
        let episodes = config["episodes"].as_u64().unwrap();
        let exploration = config["alpha"].as_f64().unwrap();
        let seed = config["seed"].as_u64().unwrap();

        let ctf = counterfactual_run(episodes, exploration, seed);
        let reference = &run["ctf_ucb"];
        assert_eq!(ctf.seen, integers(&reference["see_rewards"]), "observational rewards, seed {seed}");
        assert_eq!(ctf.natural, integers(&reference["natural_actions"]), "intentions, seed {seed}");
        assert_eq!(ctf.played, integers(&reference["chosen_actions"]), "CTF-UCB arms, seed {seed}");
        assert_eq!(ctf.wins, integers(&reference["rewards"]), "CTF-UCB rewards, seed {seed}");
        for (key, mean) in reference["values"].as_object().unwrap() {
            let (intention, arm) = key.split_once(',').unwrap();
            let estimate = ctf.learner.estimate(intention.parse().unwrap(), arm.parse().unwrap()).unwrap();
            assert_eq!(estimate.mean, mean.as_f64().unwrap(), "mean {key}");
            assert_eq!(estimate.pulls, reference["counts"][key].as_u64().unwrap(), "pulls {key}");
        }

        let (wins, played, learner) = baseline_run(episodes, exploration, seed);
        let reference = &run["baseline_ucb"];
        assert_eq!(played, integers(&reference["chosen_actions"]), "UCB arms, seed {seed}");
        assert_eq!(wins, integers(&reference["rewards"]), "UCB rewards, seed {seed}");
        for arm in 0..2usize {
            let estimate = learner.estimate(arm);
            assert_eq!(estimate.mean, reference["values"][arm.to_string()].as_f64().unwrap());
            assert_eq!(estimate.pulls, reference["counts"][arm.to_string()].as_u64().unwrap());
        }
    }
}

#[test]
fn natural_choice_wins_less_than_any_machine_and_switching_wins_most() {
    let casino = GreedyCasino::default();
    let episodes = 200_000u64;
    let rate = |play: &dyn Fn(u64) -> bool| (0..episodes).filter(|k| play(*k)).count() as f64 / episodes as f64;
    let natural = rate(&|k| casino.episode(k).see().won);
    let zero = rate(&|k| casino.episode(k).intervene(Machine::Zero).won);
    let one = rate(&|k| casino.episode(k).intervene(Machine::One).won);
    let switch = rate(&|k| {
        casino
            .episode(k)
            .counterfactual(|intention| if intention == Machine::Zero { Machine::One } else { Machine::Zero })
            .won
    });
    let tolerance = 4.0 * (0.25 / episodes as f64).sqrt();
    for (found, expected) in [(natural, 0.15), (zero, 0.30), (one, 0.30), (switch, 0.45)] {
        assert!((found - expected).abs() <= tolerance, "{found} against {expected}");
    }
}
