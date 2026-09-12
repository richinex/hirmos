//! Bootstrap survival trees with log-rank or randomized candidate splits.
use super::{random::Random, Error, Forest, Node, Split};
use crate::survival::coxph::{
    concordance::{prediction_concordance_with_ties, EventTimeTies},
    Event,
};

pub struct Data {
    x: Vec<Vec<f64>>,
    time: Vec<f64>,
    event: Vec<bool>,
    ordered: Vec<bool>,
    levels: Vec<Vec<f64>>,
}

impl Data {
    pub fn new(
        x: Vec<Vec<f64>>,
        time: Vec<f64>,
        event: Vec<bool>,
        ordered: Vec<bool>,
    ) -> Result<Self, Error> {
        if x.is_empty()
            || ordered.is_empty()
            || time.len() != x.len()
            || event.len() != x.len()
            || time.iter().any(|v| !v.is_finite() || *v < 0.0)
            || x.iter()
                .any(|r| r.len() != ordered.len() || r.iter().any(|v| !v.is_finite()))
        {
            return Err(Error::InvalidRow);
        }
        let mut levels = Vec::new();
        for (j, &is_ordered) in ordered.iter().enumerate() {
            let mut values: Vec<f64> = x.iter().map(|r| r[j]).collect();
            values.sort_by(f64::total_cmp);
            values.dedup();
            if !is_ordered
                && values
                    .iter()
                    .any(|v| *v < 1.0 || *v > 53.0 || v.fract() != 0.0)
            {
                return Err(Error::InvalidCategory);
            }
            levels.push(values);
        }
        Ok(Self {
            x,
            time,
            event,
            ordered,
            levels,
        })
    }
}

#[derive(Clone, Copy)]
pub enum SplitRule {
    LogRank,
    Auc,
    AucIgnoreTies,
    MaxStat { alpha: f64 },
    ExtraTrees { candidates: usize },
}

pub struct Settings {
    pub trees: usize,
    pub mtry: usize,
    pub seed: u32,
    pub min_node_size: usize,
    pub min_bucket: usize,
    pub max_depth: Option<usize>,
    pub rule: SplitRule,
}

pub struct Training {
    pub forest: Forest,
    /// Counts indexed by tree, then original input row.
    pub inbag: Vec<Vec<usize>>,
    pub permutation_importance: Vec<Option<f64>>,
    pub prediction_error: Option<f64>,
    pub split_statistics: Vec<Vec<f64>>,
    pub node_sample_counts: Vec<Vec<usize>>,
}

pub fn train(data: &Data, settings: &Settings) -> Result<Training, Error> {
    train_with_sampling(
        data,
        settings,
        super::Sampling::Uniform {
            fraction: 1.0,
            replacement: super::Replacement::With,
        },
    )
}

pub fn train_with_sampling(
    data: &Data,
    settings: &Settings,
    sampling: super::Sampling,
) -> Result<Training, Error> {
    if settings.trees == 0
        || settings.mtry == 0
        || settings.mtry > data.ordered.len()
        || settings.min_node_size == 0
        || settings.min_bucket == 0
        || settings.seed == 0
        || matches!(settings.rule, SplitRule::ExtraTrees { candidates: 0 })
    {
        return Err(Error::InvalidSettings);
    }
    if let SplitRule::MaxStat { alpha } = settings.rule {
        if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) || data.ordered.iter().any(|v| !v) {
            return Err(Error::InvalidSettings);
        }
    }
    if matches!(settings.rule, SplitRule::Auc | SplitRule::AucIgnoreTies)
        && data.ordered.iter().any(|v| !v)
    {
        return Err(Error::InvalidSettings);
    }
    let sampling = super::sampling::Plan::new(sampling, data.x.len(), settings.trees)?;
    let mut times: Vec<f64> = data
        .time
        .iter()
        .zip(&data.event)
        .filter_map(|(&t, &e)| e.then_some(t))
        .collect();
    times.sort_by(f64::total_cmp);
    times.dedup();
    if times.is_empty() {
        return Err(Error::InvalidTimes);
    }
    let time_ids: Vec<usize> = data
        .time
        .iter()
        .map(|t| times.partition_point(|s| s < t))
        .collect();
    let context = Context {
        data,
        settings,
        times: &times,
        time_ids,
    };
    let mut trees = Vec::with_capacity(settings.trees);
    let mut inbag = Vec::with_capacity(settings.trees);
    let mut importance = vec![Some(0.0); data.ordered.len()];
    let mut split_statistics = Vec::new();
    let mut node_sample_counts = Vec::new();
    for i in 0..settings.trees {
        let mut random = Random::new((i as u32 + 1).wrapping_mul(settings.seed) as u64);
        let (samples, oob) = sampling.draw(i, &mut random);
        let mut counts = vec![0; data.x.len()];
        for &id in &samples {
            counts[id] += 1;
        }
        let (tree, statistics, node_counts) = context.grow(samples, &mut random);
        split_statistics.push(statistics);
        node_sample_counts.push(node_counts);
        let risks: Vec<f64> = tree
            .iter()
            .map(|node| match node {
                Node::Terminal { hazard } => -hazard.iter().sum::<f64>(),
                Node::Branch { .. } => 0.0,
            })
            .collect();
        let original = accuracy(data, &oob, &tree, &risks, None);
        let mut permutation = oob.clone();
        for (column, sum) in importance.iter_mut().enumerate() {
            // Terminal split IDs are zero in ranger, so column zero consumes a shuffle too.
            let used = column == 0
                || tree.iter().any(|node| match node {
                    Node::Branch { split, .. } => match split {
                        Split::Ordered { column: c, .. } | Split::Categorical { column: c, .. } => {
                            *c == column
                        }
                    },
                    Node::Terminal { .. } => false,
                });
            if !used {
                continue;
            }
            random.shuffle(&mut permutation);
            let permuted = accuracy(data, &oob, &tree, &risks, Some((column, &permutation)));
            *sum = sum
                .zip(original.zip(permuted))
                .map(|(sum, (a, b))| sum + (a - b));
        }
        trees.push(tree);
        inbag.push(counts);
    }
    for sum in &mut importance {
        *sum = sum.map(|v| v / settings.trees as f64);
    }
    let forest = Forest::new(times, data.ordered.len(), trees)?;
    let mut duration = Vec::new();
    let mut event = Vec::new();
    let mut predictions = Vec::new();
    for (i, row) in data.x.iter().enumerate() {
        let counts: Vec<usize> = inbag.iter().map(|tree| tree[i]).collect();
        if let Some(result) = forest.predict_out_of_bag(row, &counts)? {
            duration.push(data.time[i]);
            event.push(if data.event[i] {
                Event::Observed
            } else {
                Event::Censored
            });
            predictions.push(-result.hazard.iter().sum::<f64>());
        }
    }
    let prediction_error = prediction_concordance_with_ties(
        &duration,
        &event,
        &predictions,
        None,
        EventTimeTies::Unordered,
    )
    .map(|v| 1.0 - v.value());
    Ok(Training {
        forest,
        inbag,
        permutation_importance: importance,
        prediction_error,
        split_statistics,
        node_sample_counts,
    })
}

fn accuracy(
    data: &Data,
    rows: &[usize],
    tree: &[Node],
    risks: &[f64],
    permutation: Option<(usize, &[usize])>,
) -> Option<f64> {
    let mut predictions = Vec::with_capacity(rows.len());
    let mut duration = Vec::with_capacity(rows.len());
    let mut event = Vec::with_capacity(rows.len());
    for (i, &r) in rows.iter().enumerate() {
        let mut row = data.x[r].clone();
        if let Some((column, shuffled)) = permutation {
            row[column] = data.x[shuffled[i]][column];
        }
        let node = super::terminal_node(tree, &row).expect("validated training row");
        predictions.push(risks[node]);
        duration.push(data.time[r]);
        event.push(if data.event[r] {
            Event::Observed
        } else {
            Event::Censored
        });
    }
    prediction_concordance_with_ties(
        &duration,
        &event,
        &predictions,
        None,
        EventTimeTies::Unordered,
    )
    .map(|v| v.value())
}

struct Counts {
    risk: Vec<usize>,
    deaths: Vec<usize>,
    exits: Vec<usize>,
    n: usize,
}

struct Context<'a> {
    data: &'a Data,
    settings: &'a Settings,
    times: &'a [f64],
    time_ids: Vec<usize>,
}

impl Context<'_> {
    fn counts(&self, samples: &[usize]) -> Counts {
        let mut exits = vec![0; self.times.len()];
        let mut deaths = exits.clone();
        for &id in samples {
            let t = self.time_ids[id];
            if t < exits.len() {
                exits[t] += 1;
                deaths[t] += usize::from(self.data.event[id]);
            }
        }
        let mut remaining = samples.len();
        let risk = exits
            .iter()
            .map(|&n| {
                let r = remaining;
                remaining -= n;
                r
            })
            .collect();
        Counts {
            risk,
            deaths,
            exits,
            n: samples.len(),
        }
    }

    fn grow(
        &self,
        mut samples: Vec<usize>,
        random: &mut Random,
    ) -> (Vec<Node>, Vec<f64>, Vec<usize>) {
        let mut ranges = vec![(0, samples.len(), 0)];
        let mut nodes = Vec::new();
        let mut statistics = Vec::new();
        let mut node_counts = Vec::new();
        let mut id = 0;
        while id < ranges.len() {
            let (start, end, depth) = ranges[id];
            // Terminal nodes also consume the feature-subset draws.
            let features = random.features(self.data.ordered.len(), self.settings.mtry);
            let rows = &samples[start..end];
            node_counts.push(rows.len());
            let counts = self.counts(rows);
            let first = rows[0];
            let pure = rows.iter().all(|&r| {
                self.data.time[r] == self.data.time[first]
                    && self.data.event[r] == self.data.event[first]
            });
            let stopped = pure
                || rows.len() <= self.settings.min_node_size
                || (!matches!(self.settings.rule, SplitRule::MaxStat { .. })
                    && rows.len() / 2 < self.settings.min_bucket)
                || self.settings.max_depth.is_some_and(|limit| depth >= limit);
            let split = if stopped {
                None
            } else {
                self.best_split(rows, &counts, features, random)
            };
            match split {
                None => {
                    statistics.push(0.0);
                    let mut sum = 0.0;
                    let hazard = counts
                        .risk
                        .iter()
                        .zip(&counts.deaths)
                        .map(|(&r, &d)| {
                            if r > 0 {
                                sum += d as f64 / r as f64;
                            }
                            sum
                        })
                        .collect();
                    nodes.push(Node::Terminal { hazard });
                }
                Some((split, score)) => {
                    statistics.push(score);
                    let mut pos = start;
                    let mut middle = end;
                    while pos < middle {
                        if right(&split, &self.data.x[samples[pos]]) {
                            middle -= 1;
                            samples.swap(pos, middle);
                        } else {
                            pos += 1;
                        }
                    }
                    let left = ranges.len();
                    ranges.push((start, middle, depth + 1));
                    ranges.push((middle, end, depth + 1));
                    nodes.push(Node::Branch {
                        split,
                        left,
                        right: left + 1,
                    });
                }
            }
            id += 1;
        }
        (nodes, statistics, node_counts)
    }

    fn best_split(
        &self,
        rows: &[usize],
        total: &Counts,
        features: Vec<usize>,
        random: &mut Random,
    ) -> Option<(Split, f64)> {
        if let SplitRule::MaxStat { alpha } = self.settings.rule {
            return super::maxstat::split(
                &self.data.x,
                &self.data.time,
                &self.data.event,
                rows,
                &features,
                alpha,
            );
        }
        let mut best = None;
        let mut best_score = -1.0;
        for column in features {
            if self.data.ordered[column]
                && matches!(
                    self.settings.rule,
                    SplitRule::Auc | SplitRule::AucIgnoreTies
                )
            {
                for (split, left, score) in self.auc_splits(rows, column) {
                    if left < self.settings.min_bucket
                        || rows.len() - left < self.settings.min_bucket
                    {
                        continue;
                    }
                    if score > best_score {
                        best_score = score;
                        best = Some(split);
                    }
                }
                continue;
            }
            let mut values: Vec<f64> = rows.iter().map(|&r| self.data.x[r][column]).collect();
            values.sort_by(f64::total_cmp);
            values.dedup();
            let candidates = self.candidates(column, &values, random);
            for split in candidates {
                let selected: Vec<usize> = rows
                    .iter()
                    .copied()
                    .filter(|&r| right(&split, &self.data.x[r]))
                    .collect();
                if selected.len() < self.settings.min_bucket
                    || rows.len() - selected.len() < self.settings.min_bucket
                {
                    continue;
                }
                let score = logrank(total, &self.counts(&selected));
                if score > best_score {
                    best_score = score;
                    best = Some(split);
                }
            }
        }
        best.map(|split| (split, best_score))
    }

    fn auc_splits(&self, rows: &[usize], column: usize) -> Vec<(Split, usize, f64)> {
        let mut ordered = rows.to_vec();
        ordered.sort_by(|&a, &b| self.data.time[a].total_cmp(&self.data.time[b]));
        let mut prior_events = 0i64;
        let mut pairs = 0i64;
        let mut contributions = Vec::with_capacity(rows.len());
        let mut start = 0;
        while start < ordered.len() {
            let time = self.data.time[ordered[start]];
            let mut end = start + 1;
            while end < ordered.len() && self.data.time[ordered[end]] == time {
                end += 1;
            }
            let later = (ordered.len() - end) as i64;
            let mut events = 0;
            for &row in &ordered[start..end] {
                let observed = i64::from(self.data.event[row]);
                contributions.push((self.data.x[row][column], observed * later - prior_events));
                pairs += observed * later;
                events += observed;
            }
            prior_events += events;
            start = end;
        }
        // computeAucSplit leaves status_smaller=0 for tied times, so both
        // upstream AUC modes exclude them. Prefix contributions replace its pair loop.
        contributions.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut delta = 0i64;
        let mut result = Vec::new();
        for i in 0..contributions.len() - 1 {
            delta += contributions[i].1;
            let a = contributions[i].0;
            let b = contributions[i + 1].0;
            if a == b {
                continue;
            }
            let midpoint = (a + b) / 2.0;
            let threshold = if midpoint == b { a } else { midpoint };
            let score = (((pairs + delta) as f64 / 2.0) / pairs as f64 - 0.5).abs();
            result.push((Split::Ordered { column, threshold }, i + 1, score));
        }
        result
    }

    fn candidates(&self, column: usize, values: &[f64], random: &mut Random) -> Vec<Split> {
        if self.data.ordered[column] {
            if values.len() < 2 {
                return Vec::new();
            }
            let thresholds: Vec<f64> = match self.settings.rule {
                SplitRule::MaxStat { .. } => unreachable!("maxstat chooses cutpoints separately"),
                SplitRule::LogRank | SplitRule::Auc | SplitRule::AucIgnoreTies => values
                    .windows(2)
                    .map(|w| {
                        let midpoint = (w[0] + w[1]) / 2.0;
                        if midpoint == w[1] {
                            w[0]
                        } else {
                            midpoint
                        }
                    })
                    .collect(),
                SplitRule::ExtraTrees { candidates } => {
                    let min = values[0];
                    let max = values[values.len() - 1];
                    let mut thresholds: Vec<f64> = (0..candidates)
                        .map(|_| random.unit().mul_add(max - min, min))
                        .collect();
                    thresholds.sort_by(f64::total_cmp);
                    thresholds
                }
            };
            return thresholds
                .into_iter()
                .map(|threshold| Split::Ordered { column, threshold })
                .collect();
        }
        let masks: Vec<u64> = match self.settings.rule {
            SplitRule::MaxStat { .. } => unreachable!("maxstat accepts ordered variables"),
            SplitRule::LogRank | SplitRule::Auc | SplitRule::AucIgnoreTies => {
                (1..(1_u64 << values.len()) / 2)
                    .map(|mask| level_mask(values, mask))
                    .collect()
            }
            SplitRule::ExtraTrees { candidates } => {
                let present: Vec<f64> = self.data.levels[column]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| values.contains(v).then_some((i + 1) as f64))
                    .collect();
                let absent: Vec<f64> = self.data.levels[column]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| (!values.contains(v)).then_some((i + 1) as f64))
                    .collect();
                (0..candidates)
                    .map(|_| {
                        let inside = if values.len() > 1 {
                            level_mask(&present, 1 + random.index64((1_u64 << values.len()) - 2))
                        } else {
                            0
                        };
                        let outside = if absent.len() > 1 {
                            level_mask(&absent, random.index64(1_u64 << absent.len()))
                        } else {
                            0
                        };
                        inside | outside
                    })
                    .collect()
            }
        };
        masks
            .into_iter()
            .map(|right_levels| Split::Categorical {
                column,
                right_levels,
            })
            .collect()
    }
}

fn level_mask(values: &[f64], mask: u64) -> u64 {
    values.iter().enumerate().fold(0, |bits, (i, &v)| {
        if mask & (1 << i) != 0 {
            bits | (1 << (v as u32 - 1))
        } else {
            bits
        }
    })
}

fn right(split: &Split, row: &[f64]) -> bool {
    super::goes_right(split, row).expect("validated training row")
}

fn logrank(total: &Counts, child: &Counts) -> f64 {
    let mut numerator = 0.0;
    let mut variance = 0.0;
    let mut at_risk = child.n;
    for t in 0..total.risk.len() {
        if total.risk[t] < 2 || at_risk < 1 {
            break;
        }
        if total.deaths[t] > 0 {
            let d = total.deaths[t] as f64;
            let y = total.risk[t] as f64;
            let y1 = at_risk as f64;
            // The pinned ARM ranger build contracts these expressions. Near-tied
            // split scores can select different variables without this rounding.
            numerator += (-y1).mul_add(d / y, child.deaths[t] as f64);
            variance = ((y1 / y) * (1.0 - y1 / y) * ((y - d) / (y - 1.0))).mul_add(d, variance);
        }
        at_risk -= child.exits[t];
    }
    if variance == 0.0 {
        -1.0
    } else {
        (numerator / variance.sqrt()).abs()
    }
}
