//! Online learning by intervention that uses a confounded log's natural bounds without inheriting its bias.

use std::fmt;

use crate::nprandom::NpRng;

#[derive(Clone, Debug, PartialEq)]
pub enum OfflineToOnlineError {
    NoArms,
    ArmOutOfRange { arm: usize, arms: usize },
    UnloggedArm(usize),
    RewardOutsideUnitInterval(f64),
    IntervalCount { intervals: usize, arms: usize },
    UnrandomisedArm(usize),
    EmptyLog,
}

impl fmt::Display for OfflineToOnlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArms => write!(f, "at least one arm is required"),
            Self::ArmOutOfRange { arm, arms } => write!(f, "arm {arm} is outside 0..{arms}"),
            Self::UnloggedArm(arm) => write!(f, "arm {arm} never appears in the log"),
            Self::RewardOutsideUnitInterval(value) => {
                write!(f, "reward {value} is outside [0, 1], which the natural bounds require")
            }
            Self::IntervalCount { intervals, arms } => {
                write!(f, "{intervals} intervals were given for {arms} arms")
            }
            Self::UnrandomisedArm(arm) => write!(f, "arm {arm} has no randomised rows to replay"),
            Self::EmptyLog => write!(f, "the log has no rows to replay"),
        }
    }
}

impl std::error::Error for OfflineToOnlineError {}

/// A reward in [0, 1], the range the natural bounds are derived for.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct UnitReward(f64);

impl UnitReward {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);

    pub fn new(value: f64) -> Result<Self, OfflineToOnlineError> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(OfflineToOnlineError::RewardOutsideUnitInterval(value))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// One row of a confounded log: the arm the behaviour policy chose and the reward that followed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoggedPull {
    pub arm: usize,
    pub reward: UnitReward,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

/// What the log says about one arm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArmBounds {
    pub pulls: u64,
    pub reward_sum: f64,
    /// Contains the arm's interventional mean E[Y | do(arm)] whatever the confounding.
    pub interval: Interval,
}

/// Logged rows that share an arm and a reward, and how many rows they stand for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoggedCell {
    pub arm: usize,
    pub reward: UnitReward,
    pub count: u64,
}

/// Bound each arm's interventional mean from a confounded log.
pub fn natural_bounds(arms: usize, log: &[LoggedPull]) -> Result<Vec<ArmBounds>, OfflineToOnlineError> {
    let cells: Vec<LoggedCell> = log.iter().map(|row| LoggedCell { arm: row.arm, reward: row.reward, count: 1 }).collect();
    natural_bounds_from_cells(arms, &cells)
}

/// Bound each arm's interventional mean from a confounded log summarised as counted cells.
pub fn natural_bounds_from_cells(arms: usize, log: &[LoggedCell]) -> Result<Vec<ArmBounds>, OfflineToOnlineError> {
    if arms == 0 {
        return Err(OfflineToOnlineError::NoArms);
    }
    let mut pulls = vec![0u64; arms];
    let mut reward_sums = vec![0.0; arms];
    for cell in log {
        if cell.arm >= arms {
            return Err(OfflineToOnlineError::ArmOutOfRange { arm: cell.arm, arms });
        }
        pulls[cell.arm] += cell.count;
        reward_sums[cell.arm] += cell.reward.value() * cell.count as f64;
    }
    let samples = pulls.iter().sum::<u64>() as f64;
    (0..arms)
        .map(|arm| {
            if pulls[arm] == 0 {
                return Err(OfflineToOnlineError::UnloggedArm(arm));
            }
            let expected = reward_sums[arm] / pulls[arm] as f64;
            let share = pulls[arm] as f64 / samples;
            let lower = expected * share;
            Ok(ArmBounds {
                pulls: pulls[arm],
                reward_sum: reward_sums[arm],
                interval: Interval { lower, upper: lower + (1.0 - share) },
            })
        })
        .collect()
}

/// A bandit that can both reveal its behaviour policy's choice and accept an intervention.
pub trait ConfoundedBandit {
    fn arms(&self) -> usize;
    /// Start a new episode; each episode allows one `see` or one `intervene`.
    fn reset(&mut self);
    /// Let the behaviour policy choose, as the log was produced.
    fn see(&mut self) -> LoggedPull;
    /// Force an arm regardless of the behaviour policy.
    fn intervene(&mut self, arm: usize) -> UnitReward;
}

/// How an online learner uses offline data.
#[derive(Clone, Debug, PartialEq)]
pub enum OfflineUse {
    Ignore,
    /// Clip every upper confidence bound into its arm's interval.
    Clip(Vec<Interval>),
    /// Treat this many observational episodes as if they were experiments.
    CopyObservations { episodes: usize },
}

// log(10_000): the bonus uses a fixed horizon, not log t.
const LOG_HORIZON: f64 = 9.210_340_371_976_184;

/// The arm an online learner pulled at one step and the reward it received.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OnlineStep {
    pub arm: usize,
    pub reward: f64,
}

/// Run an upper confidence bound learner by intervention and return every step.
pub fn learn_online(
    bandit: &mut impl ConfoundedBandit,
    offline: &OfflineUse,
    timesteps: usize,
) -> Result<Vec<OnlineStep>, OfflineToOnlineError> {
    let arms = bandit.arms();
    if arms == 0 {
        return Err(OfflineToOnlineError::NoArms);
    }
    if let OfflineUse::Clip(intervals) = offline {
        if intervals.len() != arms {
            return Err(OfflineToOnlineError::IntervalCount { intervals: intervals.len(), arms });
        }
    }
    let mut reward_sums = vec![0.0; arms];
    let mut counts = vec![1.0; arms];
    if let OfflineUse::CopyObservations { episodes } = offline {
        for _ in 0..*episodes {
            bandit.reset();
            let row = bandit.see();
            reward_sums[row.arm] += row.reward.value();
            counts[row.arm] += 1.0;
        }
    }
    let mut steps = Vec::with_capacity(timesteps);
    let mut scores = vec![0.0; arms];
    for _ in 0..timesteps {
        bandit.reset();
        for arm in 0..arms {
            let score = reward_sums[arm] / counts[arm] + (LOG_HORIZON / counts[arm]).sqrt();
            scores[arm] = match offline {
                OfflineUse::Clip(intervals) => score.max(intervals[arm].lower).min(intervals[arm].upper),
                _ => score,
            };
        }
        let arm = first_maximum(&scores);
        let reward = bandit.intervene(arm).value();
        steps.push(OnlineStep { arm, reward });
        reward_sums[arm] += reward;
        counts[arm] += 1.0;
    }
    Ok(steps)
}

fn first_maximum(values: &[f64]) -> usize {
    let mut best = 0;
    for (i, value) in values.iter().enumerate().skip(1) {
        if *value > values[best] {
            best = i;
        }
    }
    best
}

/// A two-armed bandit with a uniform latent confounder U.
#[derive(Clone, Debug)]
pub struct ThresholdBandit {
    delta: f64,
    next_seed: u64,
    rng: NpRng,
}

impl ThresholdBandit {
    /// Build the bandit; episode k draws U from a generator seeded with `first_seed + k`.
    pub fn new(delta: f64, first_seed: u64) -> Self {
        Self { delta, next_seed: first_seed, rng: NpRng::seeded(first_seed) }
    }

    fn reward(&self, arm: usize, latent: f64) -> UnitReward {
        if latent < 0.4 - self.delta * arm as f64 {
            UnitReward::ONE
        } else {
            UnitReward::ZERO
        }
    }
}

impl ConfoundedBandit for ThresholdBandit {
    fn arms(&self) -> usize {
        2
    }

    fn reset(&mut self) {
        self.rng = NpRng::seeded(self.next_seed);
        self.next_seed += 1;
    }

    fn see(&mut self) -> LoggedPull {
        let latent = self.rng.next_f64();
        let arm = usize::from(latent < 0.8);
        LoggedPull { arm, reward: self.reward(arm, latent) }
    }

    fn intervene(&mut self, arm: usize) -> UnitReward {
        let latent = self.rng.next_f64();
        self.reward(arm, latent)
    }
}

/// Counted cells laid out for drawing one row uniformly at random.
#[derive(Clone, Debug, PartialEq)]
struct CellDraw {
    cells: Vec<LoggedCell>,
    ends: Vec<u64>,
}

impl CellDraw {
    fn new(cells: Vec<LoggedCell>) -> Self {
        let mut total = 0;
        let ends = cells
            .iter()
            .map(|cell| {
                total += cell.count;
                total
            })
            .collect();
        Self { cells, ends }
    }

    fn rows(&self) -> u64 {
        self.ends.last().copied().unwrap_or(0)
    }

    fn draw(&self, rng: &mut NpRng) -> LoggedCell {
        let row = rng.bounded_uint64(self.rows());
        self.cells[self.ends.partition_point(|&end| end <= row)]
    }
}

/// A bandit replayed from data: `see` draws a logged row and `intervene` draws a randomised row of the chosen arm.
#[derive(Clone, Debug)]
pub struct ReplayedBandit {
    logged: CellDraw,
    randomised: Vec<CellDraw>,
    rng: NpRng,
}

impl ReplayedBandit {
    pub fn new(arms: usize, logged: &[LoggedCell], randomised: &[LoggedCell], seed: u64) -> Result<Self, OfflineToOnlineError> {
        if arms == 0 {
            return Err(OfflineToOnlineError::NoArms);
        }
        let mut by_arm = vec![Vec::new(); arms];
        for cell in randomised.iter().filter(|cell| cell.count > 0) {
            by_arm.get_mut(cell.arm).ok_or(OfflineToOnlineError::ArmOutOfRange { arm: cell.arm, arms })?.push(*cell);
        }
        if let Some(arm) = by_arm.iter().position(Vec::is_empty) {
            return Err(OfflineToOnlineError::UnrandomisedArm(arm));
        }
        let logged: Vec<LoggedCell> = logged.iter().filter(|cell| cell.count > 0).copied().collect();
        if logged.is_empty() {
            return Err(OfflineToOnlineError::EmptyLog);
        }
        if let Some(cell) = logged.iter().find(|cell| cell.arm >= arms) {
            return Err(OfflineToOnlineError::ArmOutOfRange { arm: cell.arm, arms });
        }
        Ok(Self {
            logged: CellDraw::new(logged),
            randomised: by_arm.into_iter().map(CellDraw::new).collect(),
            rng: NpRng::seeded(seed),
        })
    }
}

impl ConfoundedBandit for ReplayedBandit {
    fn arms(&self) -> usize {
        self.randomised.len()
    }

    fn reset(&mut self) {}

    fn see(&mut self) -> LoggedPull {
        let cell = self.logged.draw(&mut self.rng);
        LoggedPull { arm: cell.arm, reward: cell.reward }
    }

    fn intervene(&mut self, arm: usize) -> UnitReward {
        self.randomised[arm].draw(&mut self.rng).reward
    }
}
