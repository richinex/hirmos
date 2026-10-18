use hirmos_causal_core::nprandom::NpRng;
use hirmos_causal_core::reinforcement::sequential_evaluation::{Stage, Trajectory};
use hirmos_causal_core::reinforcement::treatment_regimes::{optimistic_expectation, CausalBounds, RegimeLearner, RegimePolicy, RegimeShape};

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn responder(s1: usize, x1: usize) -> f64 {
    sigmoid(0.1 + 0.1 * s1 as f64 + 0.1 * x1 as f64)
}

// Y = 1{U > c} with U uniform on [0, 1] (Eq. 78).
fn outcome(s1: usize, x1: usize, s2: usize, x2: usize) -> f64 {
    let c = (s1 + x1 + s1 * x1 + s2 * x2) as i64 - (x2 + x1 * x2) as i64;
    if c <= 0 { 1.0 } else { 0.0 }
}

fn chance(value: usize, probability_of_one: f64) -> f64 {
    if value == 1 { probability_of_one } else { 1.0 - probability_of_one }
}

/// The observational distribution of Example 12, integrating the shared confounder U by Simpson's rule.
fn observational_log(alpha1: f64, alpha2: f64) -> Vec<Trajectory> {
    let mut log = Vec::new();
    for s1 in 0..2 {
        for x1 in 0..2 {
            for s2 in 0..2 {
                for x2 in 0..2 {
                    let density = |u: f64| {
                        chance(x1, sigmoid(3.0 * s1 as f64 + alpha1 * u)) * chance(x2, sigmoid(3.0 * s2 as f64 + alpha2 * u))
                    };
                    let n = 4000;
                    let h = 1.0 / n as f64;
                    let integral = (0..=n)
                        .map(|i| {
                            let weight = if i == 0 || i == n { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
                            weight * density(i as f64 * h)
                        })
                        .sum::<f64>()
                        * h
                        / 3.0;
                    log.push(Trajectory {
                        stages: vec![Stage { state: s1, action: x1 }, Stage { state: s2, action: x2 }],
                        outcome: outcome(s1, x1, s2, x2),
                        weight: 0.5 * chance(s2, responder(s1, x1)) * integral,
                    });
                }
            }
        }
    }
    log
}

fn shape() -> RegimeShape {
    RegimeShape::new(vec![2, 2], vec![2, 2]).unwrap()
}

fn value(policy: &RegimePolicy) -> f64 {
    let mut total = 0.0;
    for s1 in 0..2 {
        let x1 = policy.action(&[s1]);
        for s2 in 0..2 {
            let x2 = policy.action(&[s1, x1, s2]);
            total += 0.5 * chance(s2, responder(s1, x1)) * outcome(s1, x1, s2, x2);
        }
    }
    total
}

fn logistic(rng: &mut NpRng) -> f64 {
    let u = rng.next_f64();
    (u / (1.0 - u)).ln()
}

fn episode(policy: &RegimePolicy, rng: &mut NpRng) -> Trajectory {
    let u = rng.next_f64();
    let s1 = usize::from(logistic(rng) > 0.0);
    let x1 = policy.action(&[s1]);
    let s2 = usize::from(0.1 + 0.1 * s1 as f64 + 0.1 * x1 as f64 + logistic(rng) > 0.0);
    let x2 = policy.action(&[s1, x1, s2]);
    let c = (s1 + x1 + s1 * x1 + s2 * x2) as f64 - (x2 + x1 * x2) as f64;
    Trajectory {
        stages: vec![Stage { state: s1, action: x1 }, Stage { state: s2, action: x2 }],
        outcome: if u > c { 1.0 } else { 0.0 },
        weight: 1.0,
    }
}

const OPTIMAL: f64 = 0.5 + 0.5 * (1.0 - 0.549833997312478);

fn cumulative_regret(bounds: CausalBounds, episodes: usize, seed: u64) -> Vec<f64> {
    let mut learner = RegimeLearner::new(bounds);
    let mut rng = NpRng::seeded(seed);
    let mut total = 0.0;
    (0..episodes)
        .map(|_| {
            let policy = learner.policy();
            total += OPTIMAL - value(&policy);
            learner.record(&episode(&policy, &mut rng)).unwrap();
            total
        })
        .collect()
}

#[test]
fn transition_bounds_reproduce_table_13a() {
    let bounds = CausalBounds::from_log(&shape(), &observational_log(3.0, -3.0)).unwrap();
    let table = [
        [0.1021, 0.8872], [0.1128, 0.8979], [0.3534, 0.5683], [0.4317, 0.6466],
        [0.0069, 0.9915], [0.0085, 0.9931], [0.4190, 0.4344], [0.5656, 0.5810],
    ];
    for (history, [low, high]) in table.iter().enumerate() {
        let (l, r) = bounds.transition(1, history);
        assert!((l - low).abs() <= 5e-5 && (r - high).abs() <= 5e-5, "history {history}: ({l}, {r})");
    }
}

#[test]
fn every_deterministic_regime_has_its_value_inside_its_causal_bounds() {
    for (alpha1, alpha2) in [(3.0, -3.0), (-12.0, -3.0), (0.0, 0.0)] {
        let bounds = CausalBounds::from_log(&shape(), &observational_log(alpha1, alpha2)).unwrap();
        let mut learner = RegimeLearner::new(CausalBounds::uninformative(&shape()));
        let mut rng = NpRng::seeded(5);
        for _ in 0..300 {
            let policy = learner.policy();
            let (low, high) = bounds.policy_value(&policy);
            let truth = value(&policy);
            assert!(low <= truth + 1e-9 && truth <= high + 1e-9, "alpha ({alpha1}, {alpha2}): {low} <= {truth} <= {high}");
            learner.record(&episode(&policy, &mut rng)).unwrap();
        }
    }
}

#[test]
fn the_optimistic_expectation_stays_between_the_estimate_and_the_best_distribution_in_the_bounds() {
    let mut rng = NpRng::seeded(11);
    for _ in 0..500 {
        let n = 2 + rng.bounded_uint64(4) as usize;
        let values: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let raw: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let estimate: Vec<f64> = raw.iter().map(|v| v / raw.iter().sum::<f64>()).collect();
        let open = vec![(0.0, 1.0); n];
        let at_estimate: f64 = estimate.iter().zip(&values).map(|(p, v)| p * v).sum();
        let best = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!((optimistic_expectation(&values, &estimate, 0.0, &open) - at_estimate).abs() < 1e-9);
        assert!((optimistic_expectation(&values, &estimate, 2.0, &open) - best).abs() < 1e-9);
        let narrow = optimistic_expectation(&values, &estimate, 0.1, &open);
        let wide = optimistic_expectation(&values, &estimate, 0.4, &open);
        assert!(at_estimate - 1e-9 <= narrow && narrow <= wide + 1e-9 && wide <= best + 1e-9);
        let spread: Vec<(f64, f64)> = estimate.iter().map(|p| ((p - 0.05).max(0.0), (p + 0.05).min(1.0))).collect();
        let clipped = optimistic_expectation(&values, &estimate, 2.0, &spread);
        assert!(clipped <= wide.max(at_estimate) + 0.05 * n as f64 + 1e-9 && clipped >= at_estimate - 1e-9);
    }
}

#[test]
fn regret_per_episode_falls_and_informative_causal_bounds_cut_the_total() {
    for (alpha1, alpha2) in [(3.0, -3.0), (-12.0, -3.0)] {
        let informed = CausalBounds::from_log(&shape(), &observational_log(alpha1, alpha2)).unwrap();
        for seed in 1..=3 {
            let plain = cumulative_regret(CausalBounds::uninformative(&shape()), 11_000, seed);
            let causal = cumulative_regret(informed.clone(), 11_000, seed);
            let (early, late) = (plain[999] / 1000.0, (plain[10_999] - plain[9_999]) / 1000.0);
            assert!(late < early / 10.0, "seed {seed}: regret per episode {early} early, {late} late");
            let factor = if alpha1 == 3.0 { 0.5 } else { 1.0 };
            assert!(causal[10_999] < factor * plain[10_999], "alpha ({alpha1}, {alpha2}) seed {seed}: causal {}, plain {}", causal[10_999], plain[10_999]);
        }
    }
}
