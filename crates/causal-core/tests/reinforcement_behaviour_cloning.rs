use hirmos_causal_core::reinforcement::behaviour_cloning::{clone_continuous, clone_discrete, CloningSettings, Demonstrations, DiscreteClone};
use hirmos_causal_core::nprandom::NpRng;

#[derive(Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(self, a: u8, b: u8) -> u8 {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

fn flip(value: u8, keep: bool) -> u8 {
    if keep { value } else { 1 - value }
}

/// One random model from the four families of Experiment 9.10, over three binary confounders.
struct Model {
    graph: usize,
    p_one: [f64; 3],
    signs: [bool; 4],
    ops: [Op; 4],
}

impl Model {
    fn random(graph: usize, rng: &mut NpRng) -> Self {
        let mut coin = || rng.next_f64() < 0.5;
        let signs = [coin(), coin(), coin(), coin()];
        let ops = [0; 4].map(|_| [Op::And, Op::Or, Op::Xor][rng.bounded_uint64(3) as usize]);
        let p_one = [0; 3].map(|_| rng.next_f64());
        Self { graph, p_one, signs, ops }
    }

    /// (state Z0, expert X0, expert X1, reward Y) for confounders `u`, with `x0` and `x1` overriding the expert when given.
    fn run(&self, u: [u8; 3], x0: Option<u8>, x1: impl Fn(u8, u8) -> Option<u8>) -> (u8, u8, u8, u8) {
        let [s0, s1, s2, s3] = self.signs;
        let [o0, o1, o2, o3] = self.ops;
        match self.graph {
            1 => {
                let z = flip(o1.apply(o0.apply(u[0], u[1]), u[2]), s0);
                let a0 = x0.unwrap_or(flip(u[0], s1));
                let a1 = x1(z, a0).unwrap_or(flip(o2.apply(a0, u[1]), s2));
                (z, a0, a1, flip(o3.apply(a1, u[2]), s3))
            }
            2 | 3 => {
                let z = flip(u[0], if self.graph == 2 { !s0 } else { s0 });
                let a0 = x0.unwrap_or(flip(u[0], s1));
                let a1 = x1(z, a0).unwrap_or(flip(a0, s2));
                (z, a0, a1, flip(o3.apply(a1, u[0]), s3))
            }
            _ => {
                let z = flip(o0.apply(u[0], u[2]), s0);
                let a0 = x0.unwrap_or(flip(u[0], s1));
                let a1 = x1(z, a0).unwrap_or(flip(z, s2));
                (z, a0, a1, flip(o2.apply(o1.apply(a0, a1), u[2]), s3))
            }
        }
    }

    fn confounders(&self) -> impl Iterator<Item = ([u8; 3], f64)> + '_ {
        (0..8u8).map(move |bits| {
            let u = [bits & 1, (bits >> 1) & 1, (bits >> 2) & 1];
            let p = (0..3).map(|i| if u[i] == 1 { self.p_one[i] } else { 1.0 - self.p_one[i] }).product();
            (u, p)
        })
    }

    fn expert_value(&self) -> f64 {
        self.confounders().map(|(u, p)| p * self.run(u, None, |_, _| None).3 as f64).sum()
    }
}

/// Inputs an action reads: Z0 and, for the second action, X0.
#[derive(Clone, Copy)]
struct Reads {
    z: bool,
    x0: bool,
}

fn features(reads: Reads, z: u8, x0: u8) -> Vec<f32> {
    let mut row = Vec::new();
    if reads.x0 {
        row.push(x0 as f32);
    }
    if reads.z {
        row.push(z as f32);
    }
    row
}

fn imitator_value(model: &Model, first: &DiscreteClone, first_reads: Reads, second: &DiscreteClone, second_reads: Reads) -> f64 {
    let mut total = 0.0;
    for (u, p) in model.confounders() {
        let z = model.run(u, None, |_, _| None).0;
        let p0 = first.probabilities(&features(first_reads, z, 0));
        for x0 in 0..2u8 {
            let p1 = second.probabilities(&features(second_reads, z, x0));
            for x1 in 0..2u8 {
                let y = model.run(u, Some(x0), |_, _| Some(x1)).3;
                total += p * p0[x0 as usize] * p1[x1 as usize] * y as f64;
            }
        }
    }
    total
}

fn gap(graph: usize, seed: u64, first_reads: Reads, second_reads: Reads) -> f64 {
    let mut rng = NpRng::seeded(seed);
    let model = Model::random(graph, &mut rng);
    let (mut first_x, mut second_x, mut first_a, mut second_a) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for _ in 0..1000 {
        let u = [0, 1, 2].map(|i| u8::from(rng.next_f64() < model.p_one[i]));
        let (z, x0, x1, _) = model.run(u, None, |_, _| None);
        first_x.extend(features(first_reads, z, 0));
        first_a.push(x0 as usize);
        second_x.extend(features(second_reads, z, x0));
        second_a.push(x1 as usize);
    }
    let settings = CloningSettings { seed, ..CloningSettings::default() };
    let width = |r: Reads| usize::from(r.z) + usize::from(r.x0);
    let first = clone_discrete(Demonstrations { inputs: &first_x, width: width(first_reads) }, &first_a, 2, &settings).unwrap();
    let second = clone_discrete(Demonstrations { inputs: &second_x, width: width(second_reads) }, &second_a, 2, &settings).unwrap();
    (imitator_value(&model, &first, first_reads, &second, second_reads) - model.expert_value()).abs()
}

fn mean_gap(graph: usize, first: Reads, second: Reads) -> f64 {
    (0..10).map(|k| gap(graph, 100 * graph as u64 + k, first, second)).sum::<f64>() / 10.0
}

const NONE: Reads = Reads { z: false, x0: false };
const Z: Reads = Reads { z: true, x0: false };
const X0: Reads = Reads { z: false, x0: true };

#[test]
fn cloning_on_the_causal_sets_matches_the_expert_and_cloning_on_observed_parents_does_not() {
    let causal = [mean_gap(1, NONE, X0), mean_gap(2, Z, Z), mean_gap(3, NONE, Z)];
    let parents = [mean_gap(2, NONE, X0), mean_gap(3, NONE, X0), mean_gap(4, NONE, Z)];
    assert!(causal.iter().all(|g| *g < 0.02), "causal sets: {causal:?}");
    assert!(parents[0] > 0.1 && parents[1] > 0.1 && parents[2] > 0.02, "observed parents: {parents:?}");
}

#[test]
fn a_continuous_rule_learns_a_linear_target() {
    let mut rng = NpRng::seeded(3);
    let inputs: Vec<f32> = (0..1000).map(|_| rng.next_f64() as f32).collect();
    let targets: Vec<f32> = inputs.iter().map(|x| 0.5 * x - 0.2).collect();
    let rule = clone_continuous(Demonstrations { inputs: &inputs, width: 1 }, &targets, 1, None, &CloningSettings::default()).unwrap();
    let error = (0..=10).map(|i| (rule.act(&[i as f32 / 10.0])[0] - (0.05 * i as f64 - 0.2)).abs()).fold(0.0, f64::max);
    assert!(error < 0.05, "largest error {error}");
}
