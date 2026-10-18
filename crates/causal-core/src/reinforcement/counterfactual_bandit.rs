//! Bandits that use a person's intended action as information, with the Greedy Casino as the standard example.

use std::collections::BTreeMap;
use std::fmt;

use crate::nprandom::NpRng;

#[derive(Clone, Debug, PartialEq)]
pub enum CounterfactualBanditError {
    NoArms,
    InvalidProbability(f64),
}

impl fmt::Display for CounterfactualBanditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArms => write!(f, "at least one arm is required"),
            Self::InvalidProbability(p) => write!(f, "payout probability {p} is outside [0, 1]"),
        }
    }
}

impl std::error::Error for CounterfactualBanditError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Machine {
    Zero,
    One,
}

impl Machine {
    pub fn index(self) -> usize {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Zero),
            1 => Some(Self::One),
            _ => None,
        }
    }
}

/// Payout probabilities indexed by machine, then by the blinking coin, then by the drinking coin.
pub type PayoutTable = [[[f64; 2]; 2]; 2];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CasinoStep {
    pub natural: Machine,
    pub played: Machine,
    pub won: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GreedyCasino {
    payouts: PayoutTable,
}

impl Default for GreedyCasino {
    fn default() -> Self {
        Self {
            payouts: [[[0.10, 0.50], [0.40, 0.20]], [[0.50, 0.10], [0.20, 0.40]]],
        }
    }
}

impl GreedyCasino {
    pub fn new(payouts: PayoutTable) -> Result<Self, CounterfactualBanditError> {
        if let Some(p) = payouts.iter().flatten().flatten().find(|p| !(0.0..=1.0).contains(*p)) {
            return Err(CounterfactualBanditError::InvalidProbability(*p));
        }
        Ok(Self { payouts })
    }

    /// Draw the hidden coins for one gambler from a generator seeded with `seed`.
    pub fn episode(&self, seed: u64) -> CasinoEpisode {
        let mut rng = NpRng::seeded(seed);
        let blinking = fair_coin(&mut rng);
        let drunk = fair_coin(&mut rng);
        CasinoEpisode { payouts: self.payouts, blinking, drunk, rng }
    }
}

fn fair_coin(rng: &mut NpRng) -> bool {
    rng.binomial(1, 0.5).expect("a single draw uses the inversion sampler") == 1
}

/// One gambler at the machines, with hidden coins already drawn.
#[derive(Clone, Debug)]
pub struct CasinoEpisode {
    payouts: PayoutTable,
    blinking: bool,
    drunk: bool,
    rng: NpRng,
}

impl CasinoEpisode {
    /// The machine the gambler would choose unaided: machine one when exactly one coin is set.
    pub fn natural(&self) -> Machine {
        if self.blinking ^ self.drunk {
            Machine::One
        } else {
            Machine::Zero
        }
    }

    pub fn blinking(&self) -> bool {
        self.blinking
    }

    pub fn drunk(&self) -> bool {
        self.drunk
    }

    fn play(mut self, played: Machine) -> CasinoStep {
        let p = self.payouts[played.index()][usize::from(self.blinking)][usize::from(self.drunk)];
        let won = self.rng.binomial(1, p).expect("a single draw uses the inversion sampler") == 1;
        CasinoStep { natural: self.natural(), played, won }
    }

    /// Let the gambler play the machine they would choose unaided.
    pub fn see(self) -> CasinoStep {
        let natural = self.natural();
        self.play(natural)
    }

    /// Make the gambler play `machine` whatever they intended.
    pub fn intervene(self, machine: Machine) -> CasinoStep {
        self.play(machine)
    }

    /// Show the policy the gambler's intended machine and play the machine it returns.
    pub fn counterfactual(self, policy: impl FnOnce(Machine) -> Machine) -> CasinoStep {
        let played = policy(self.natural());
        self.play(played)
    }
}

/// The running mean reward of an arm and the number of pulls behind it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ArmEstimate {
    pub pulls: u64,
    pub mean: f64,
}

impl ArmEstimate {
    fn record(&mut self, reward: f64) {
        self.mean += (reward - self.mean) / (self.pulls as f64 + 1.0);
        self.pulls += 1;
    }
}

/// An upper confidence bound learner whose estimates are kept separately for each intention.
#[derive(Clone, Debug)]
pub struct CounterfactualUcb {
    arms: usize,
    exploration: f64,
    estimates: BTreeMap<(usize, usize), ArmEstimate>,
    rounds: u64,
}

impl CounterfactualUcb {
    pub fn new(arms: usize, exploration: f64) -> Result<Self, CounterfactualBanditError> {
        if arms == 0 {
            return Err(CounterfactualBanditError::NoArms);
        }
        Ok(Self { arms, exploration, estimates: BTreeMap::new(), rounds: 0 })
    }

    pub fn select_arm(&mut self, intention: usize) -> usize {
        self.rounds += 1;
        let ln_rounds = (self.rounds as f64).ln();
        let mut best_arm = 0;
        let mut best_score = f64::NEG_INFINITY;
        for arm in 0..self.arms {
            let estimate = self.estimate(intention, arm).unwrap_or_default();
            if estimate.pulls == 0 {
                return arm;
            }
            let score = estimate.mean + self.exploration * (ln_rounds / estimate.pulls as f64).sqrt();
            if score > best_score {
                best_score = score;
                best_arm = arm;
            }
        }
        best_arm
    }

    pub fn update(&mut self, intention: usize, arm: usize, reward: f64) {
        self.estimates.entry((intention, arm)).or_default().record(reward);
    }

    pub fn estimate(&self, intention: usize, arm: usize) -> Option<ArmEstimate> {
        self.estimates.get(&(intention, arm)).copied()
    }
}

/// An upper confidence bound learner that ignores intentions, the experimental baseline.
#[derive(Clone, Debug)]
pub struct Ucb {
    exploration: f64,
    estimates: Vec<ArmEstimate>,
    rounds: u64,
}

impl Ucb {
    pub fn new(arms: usize, exploration: f64) -> Result<Self, CounterfactualBanditError> {
        if arms == 0 {
            return Err(CounterfactualBanditError::NoArms);
        }
        Ok(Self { exploration, estimates: vec![ArmEstimate::default(); arms], rounds: 0 })
    }

    pub fn select_arm(&mut self) -> usize {
        self.rounds += 1;
        if let Some(arm) = self.estimates.iter().position(|estimate| estimate.pulls == 0) {
            return arm;
        }
        let ln_rounds = (self.rounds as f64).ln();
        let mut best_arm = 0;
        let mut best_score = f64::NEG_INFINITY;
        for (arm, estimate) in self.estimates.iter().enumerate() {
            let score = estimate.mean + self.exploration * (ln_rounds / estimate.pulls as f64).sqrt();
            if score > best_score {
                best_score = score;
                best_arm = arm;
            }
        }
        best_arm
    }

    pub fn update(&mut self, arm: usize, reward: f64) {
        self.estimates[arm].record(reward);
    }

    pub fn estimate(&self, arm: usize) -> ArmEstimate {
        self.estimates[arm]
    }
}
