// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
//! Staged GRF tune_forest operations. Candidate IDs preserve R's draw order even
//! when an external worker pool evaluates the forests out of order.
use super::{
    forest, regression,
    sampling::Clusters,
    tree::{Data, Honesty},
};
use crate::survival::r_rng::RRng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parameter {
    SampleFraction,
    Mtry,
    MinNodeSize,
    HonestyFraction,
    HonestyPrune,
    Alpha,
    ImbalancePenalty,
}
impl Parameter {
    pub fn from_name(name: &str) -> Result<Self, &'static str> {
        match name {
            "sample.fraction" => Ok(Self::SampleFraction),
            "mtry" => Ok(Self::Mtry),
            "min.node.size" => Ok(Self::MinNodeSize),
            "honesty.fraction" => Ok(Self::HonestyFraction),
            "honesty.prune.leaves" => Ok(Self::HonestyPrune),
            "alpha" => Ok(Self::Alpha),
            "imbalance.penalty" => Ok(Self::ImbalancePenalty),
            _ => Err("Unknown GRF tuning parameter."),
        }
    }
    pub fn transform(self, rows: usize, columns: usize, draw: f64) -> Result<f64, &'static str> {
        if rows == 0 || columns == 0 || !draw.is_finite() || draw <= 0.0 || draw >= 1.0 {
            return Err(
                "Tuning requires positive dimensions and a draw strictly between zero and one.",
            );
        }
        Ok(match self {
            Self::SampleFraction => 0.05 + 0.45 * draw,
            Self::Mtry => ((columns as f64).min((columns as f64).sqrt() + 20.0) * draw).ceil(),
            Self::MinNodeSize => 2_f64
                .powf(draw * ((rows as f64).ln() / 2_f64.ln() - 4.0))
                .floor(),
            Self::HonestyFraction => 0.5 + (0.8 - 0.5) * draw,
            Self::HonestyPrune => {
                if draw < 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Alpha => draw / 4.0,
            Self::ImbalancePenalty => -crate::arm_log::log_positive(draw),
        })
    }
}
#[derive(Clone, Debug)]
pub struct Candidate {
    pub index: usize,
    pub draws: Vec<f64>,
    pub parameters: Vec<(Parameter, f64)>,
}
/// R resets the seed separately for fitting draws and optimization draws. Call
/// this with each stage's row count, not with one continued RNG stream.
pub fn candidates(
    rows: usize,
    columns: usize,
    parameters: &[Parameter],
    count: usize,
    seed: u32,
) -> Result<Vec<Candidate>, &'static str> {
    if count == 0
        || parameters.is_empty()
        || parameters
            .iter()
            .enumerate()
            .any(|(i, p)| parameters[..i].contains(p))
    {
        return Err("Tuning requires candidates and unique parameter names.");
    }
    let mut rng = RRng::new(seed);
    let mut result: Vec<_> = (0..count)
        .map(|index| Candidate {
            index,
            draws: vec![],
            parameters: vec![],
        })
        .collect();
    // matrix(runif(n * p), n, p) fills columns, not candidate rows.
    for &parameter in parameters {
        for candidate in &mut result {
            let u = rng.uniform();
            candidate.draws.push(u);
            candidate
                .parameters
                .push((parameter, parameter.transform(rows, columns, u)?));
        }
    }
    Ok(result)
}
impl Candidate {
    pub fn forest_options(
        &self,
        base: &forest::Options,
        trees: u32,
    ) -> Result<forest::Options, &'static str> {
        let mut options = base.clone();
        options.trees = trees;
        options.group_size = 1;
        for &(p, v) in &self.parameters {
            match p {
                Parameter::SampleFraction => options.sample_fraction = v,
                Parameter::Mtry => options.tree.mtry = v as u32,
                Parameter::MinNodeSize => options.tree.min_node_size = v as usize,
                Parameter::Alpha => options.tree.alpha = v,
                Parameter::ImbalancePenalty => options.tree.imbalance_penalty = v,
                Parameter::HonestyFraction => {
                    if let Honesty::Enabled { fraction, .. } = &mut options.tree.honesty {
                        *fraction = v;
                    }
                }
                Parameter::HonestyPrune => {
                    if let Honesty::Enabled { prune, .. } = &mut options.tree.honesty {
                        *prune = v != 0.0;
                    }
                }
            }
        }
        Ok(options)
    }
}
/// Evaluate one independently scheduled mini-forest. An unavailable OOB mean
/// remains None; a malformed request or numerical failure is not converted to it.
pub fn causal_error(
    data: Data<'_>,
    clusters: &Clusters,
    options: &forest::Options,
    stabilize: bool,
) -> Result<Option<f64>, &'static str> {
    let fit = forest::train_with_stabilization(data, clusters, options, stabilize)?;
    let errors = fit.oob_errors(
        data.columns,
        &data.columns[data.outcome],
        &data.columns[data.treatment],
    )?;
    Ok(mean_available(errors.into_iter().map(|v| v.map(|v| v.0))))
}
pub fn regression_error(
    columns: &[Vec<f64>],
    outcome: usize,
    weight: Option<usize>,
    clusters: &Clusters,
    options: &forest::Options,
) -> Result<Option<f64>, &'static str> {
    let fit = regression::train(columns, outcome, weight, clusters, options)?;
    let errors = fit.oob_errors(columns, &columns[outcome])?;
    Ok(mean_available(errors.into_iter().map(|v| v.map(|v| v.0))))
}
fn mean_available(values: impl Iterator<Item = Option<f64>>) -> Option<f64> {
    let (mut sum, mut count) = (0.0, 0);
    for value in values.flatten() {
        sum += value;
        count += 1;
    }
    if count == 0 {
        None
    } else {
        Some(sum / count as f64)
    }
}

#[derive(Debug, PartialEq)]
pub enum SurfaceReadiness {
    Ready,
    TooFewUsableForests,
    NearlyConstantErrors,
}
pub fn surface_readiness(errors: &[Option<f64>]) -> Result<SurfaceReadiness, &'static str> {
    let finite: Vec<_> = errors.iter().flatten().copied().collect();
    if finite.iter().any(|v| !v.is_finite()) {
        return Err("Non-finite tuning error.");
    }
    if finite.len() < 10 {
        return Ok(SurfaceReadiness::TooFewUsableForests);
    }
    let mean = finite.iter().sum::<f64>() / finite.len() as f64;
    let sd =
        (finite.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (finite.len() - 1) as f64).sqrt();
    // Preserve upstream's signed sd/mean check, including negative means.
    Ok(if sd == 0.0 || sd / mean < 1e-10 {
        SurfaceReadiness::NearlyConstantErrors
    } else {
        SurfaceReadiness::Ready
    })
}

/// Restore candidate order after independent worker jobs. Missing and duplicate
/// replies are protocol errors, not unavailable OOB estimates.
pub fn ordered_errors(
    count: usize,
    replies: &[(usize, Option<f64>)],
) -> Result<Vec<Option<f64>>, &'static str> {
    if count == 0 || replies.len() != count {
        return Err("Incomplete tuning replies.");
    }
    let mut seen = vec![false; count];
    let mut errors = vec![None; count];
    for &(index, error) in replies {
        if index >= count || seen[index] {
            return Err("Duplicate or invalid tuning candidate identity.");
        }
        if error.is_some_and(|v| !v.is_finite()) {
            return Err("Non-finite tuning error.");
        }
        seen[index] = true;
        errors[index] = error;
    }
    Ok(errors)
}

#[derive(Clone)]
pub struct Config {
    pub parameters: Vec<Parameter>,
    pub mini_trees: u32,
    pub repetitions: usize,
    pub draws: usize,
}
impl Config {
    pub fn causal_defaults(parameters: Vec<Parameter>) -> Self {
        Self {
            parameters,
            mini_trees: 200,
            repetitions: 50,
            draws: 1000,
        }
    }
    pub fn regression_defaults(parameters: Vec<Parameter>) -> Self {
        Self {
            parameters,
            mini_trees: 50,
            repetitions: 100,
            draws: 1000,
        }
    }
    pub fn defaults(&self, columns: usize) -> Vec<(Parameter, f64)> {
        self.parameters
            .iter()
            .map(|p| {
                (
                    *p,
                    match p {
                        Parameter::SampleFraction | Parameter::HonestyFraction => 0.5,
                        Parameter::Mtry => {
                            ((columns as f64).sqrt() + 20.).ceil().min(columns as f64)
                        }
                        Parameter::MinNodeSize => 5.,
                        Parameter::HonestyPrune => 1.,
                        Parameter::Alpha => 0.05,
                        Parameter::ImbalancePenalty => 0.,
                    },
                )
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum Failure {
    TooFewUsableForests,
    NearlyConstantErrors,
    ResponseSurface(&'static str),
}
#[derive(Debug)]
pub enum Status {
    /// Upstream keeps defaults, but the failed tuning attempt remains explicit.
    Failed(Failure),
    Default {
        error: Option<f64>,
    },
    Tuned {
        candidate: Candidate,
        error: f64,
        surface: Vec<f64>,
    },
}
pub struct Output {
    pub status: Status,
    pub options: forest::Options,
}

/// Complete tune_forest control flow. The supplied evaluator also allows the
/// same implementation to tune outcome, treatment and causal forests.
/// `defaults` contains upstream defaults for the selected parameters only;
/// untuned options continue to come from `base`.
pub fn tune(
    rows: usize,
    columns: usize,
    base: &forest::Options,
    defaults: &[(Parameter, f64)],
    config: &Config,
    mut evaluate: impl FnMut(&forest::Options) -> Result<Option<f64>, &'static str>,
) -> Result<Output, &'static str> {
    let confirmation_trees = config
        .mini_trees
        .checked_mul(4)
        .filter(|v| *v > 0)
        .ok_or("Invalid tuning tree count.")?;
    if config.draws == 0
        || defaults.len() != config.parameters.len()
        || config
            .parameters
            .iter()
            .any(|p| defaults.iter().filter(|(d, _)| d == p).count() != 1)
        || defaults.iter().any(|(_, v)| !v.is_finite())
    {
        return Err("Invalid tuning defaults or prediction draw count.");
    }
    let fit = candidates(
        rows,
        columns,
        &config.parameters,
        config.repetitions,
        base.seed,
    )?;
    let default_candidate = Candidate {
        index: 0,
        draws: vec![],
        parameters: defaults.to_vec(),
    };
    let mut default_options = default_candidate.forest_options(base, base.trees)?;
    default_options.group_size = base.group_size;
    let mut replies = Vec::with_capacity(fit.len());
    for c in &fit {
        replies.push((
            c.index,
            evaluate(&c.forest_options(base, config.mini_trees)?)?,
        ));
    }
    let errors = ordered_errors(fit.len(), &replies)?;
    let failed = |reason| Output {
        status: Status::Failed(reason),
        options: default_options.clone(),
    };
    match surface_readiness(&errors)? {
        SurfaceReadiness::TooFewUsableForests => return Ok(failed(Failure::TooFewUsableForests)),
        SurfaceReadiness::NearlyConstantErrors => return Ok(failed(Failure::NearlyConstantErrors)),
        SurfaceReadiness::Ready => (),
    }
    let design: Vec<_> = fit
        .iter()
        .zip(&errors)
        .filter(|(_, e)| e.is_some())
        .map(|(c, _)| c.draws.clone())
        .collect();
    let response: Vec<_> = errors.iter().flatten().copied().collect();
    let model = match super::kriging::fit(&design, &response, base.seed) {
        Ok(s) => s,
        Err(e) => return Ok(failed(Failure::ResponseSurface(e))),
    };
    let grid = candidates(rows, columns, &config.parameters, config.draws, base.seed)?;
    let surface = model
        .surface
        .predict(&grid.iter().map(|c| c.draws.clone()).collect::<Vec<_>>())?;
    if surface.iter().any(|v| !v.is_finite()) {
        return Err("Non-finite tuning surface prediction.");
    }
    let mut best = 0;
    for i in 1..surface.len() {
        if surface[i] < surface[best] {
            best = i;
        }
    }
    let candidate = grid[best].clone();
    let chosen_error = evaluate(&candidate.forest_options(base, confirmation_trees)?)?;
    let default_error = evaluate(&default_candidate.forest_options(base, confirmation_trees)?)?;
    if chosen_error
        .into_iter()
        .chain(default_error)
        .any(|v| !v.is_finite())
    {
        return Err("Non-finite confirmation error.");
    }
    // R cannot evaluate `if (NA)` when only the default confirmation is missing.
    // Report that condition instead of silently inventing a successful choice.
    match (chosen_error, default_error) {
        (None, _) => Ok(Output {
            status: Status::Default {
                error: default_error,
            },
            options: default_options,
        }),
        (Some(_), None) => Err("Default confirmation forest has no OOB error."),
        (Some(error), Some(default)) if default < error => Ok(Output {
            status: Status::Default {
                error: Some(default),
            },
            options: default_options,
        }),
        (Some(error), Some(_)) => {
            let mut options = candidate.forest_options(base, base.trees)?;
            options.group_size = base.group_size;
            Ok(Output {
                status: Status::Tuned {
                    candidate,
                    error,
                    surface,
                },
                options,
            })
        }
    }
}
