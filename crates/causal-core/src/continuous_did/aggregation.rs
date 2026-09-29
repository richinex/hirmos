//! ptetools pte_aggte for the initial unweighted, varying-baseline slope route.
//! Its cohort proportions normalize over treated units, including inside wif.
use crate::continuous_did::{
    bootstrap::{self, Bands, Stream},
    panel::Fit,
};
pub use crate::staggered_did::EventWindow;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Error {
    Shape,
    TimeOverflow,
    EmptyEventWindow,
    UnavailableStandardError { stage: String },
    Bootstrap(bootstrap::Error),
}
impl From<bootstrap::Error> for Error {
    fn from(e: bootstrap::Error) -> Self {
        Self::Bootstrap(e)
    }
}
pub struct Summary {
    pub labels: Vec<i64>,
    pub estimates: Vec<f64>,
    pub influence: Vec<Vec<f64>>,
    pub standard_errors: Vec<f64>,
    pub critical: f64,
    pub overall: f64,
    pub overall_influence: Vec<f64>,
    pub overall_standard_error: f64,
}
pub struct Aggregation {
    pub group: Summary,
    pub event: Summary,
    /// Ordered stages at which R warns that a pointwise critical-value floor applied.
    pub floor_stages: Vec<String>,
}

fn weighted(
    values: &[f64],
    scores: &[Vec<f64>],
    weights: &[f64],
    members: &[Vec<bool>],
    estimated: bool,
) -> (f64, Vec<f64>) {
    let sum: f64 = weights.iter().sum();
    let mean = values.iter().zip(weights).map(|(v, w)| v * w).sum::<f64>() / sum;
    let influence = scores
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let base = row.iter().zip(weights).map(|(v, w)| v * w).sum::<f64>() / sum;
            // Algebraic form of wif %*% att: constants cancel, but the
            // denominator is the oracle's treated-normalized probability sum.
            base + if estimated {
                values
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| members[*j][i])
                    .map(|(_, v)| v - mean)
                    .sum::<f64>()
                    / sum
            } else {
                0.0
            }
        })
        .collect();
    (mean, influence)
}
pub(crate) fn boot(
    stream: &mut Stream,
    input: &[Vec<f64>],
    iterations: usize,
    alpha: f64,
    stage: &str,
    warnings: &mut Vec<String>,
) -> Result<bootstrap::Bootstrap, Error> {
    let result = stream.run(input, iterations, alpha)?;
    if result.pointwise_floor_applied {
        warnings.push(stage.to_string());
    }
    Ok(result)
}
fn finish(
    labels: Vec<i64>,
    parts: Vec<(f64, Vec<f64>)>,
    weights: Vec<f64>,
    members: Vec<Vec<bool>>,
    estimated: bool,
    bands: Bands,
    stream: &mut Stream,
    iterations: usize,
    alpha: f64,
    stage: &str,
    warnings: &mut Vec<String>,
) -> Result<Summary, Error> {
    let n = parts[0].1.len();
    let estimates: Vec<_> = parts.iter().map(|p| p.0).collect();
    let influence: Vec<Vec<f64>> = (0..n)
        .map(|i| parts.iter().map(|p| p.1[i]).collect())
        .collect();
    let mut standard_errors = Vec::new();
    for (j, p) in parts.iter().enumerate() {
        let column: Vec<_> = p.1.iter().map(|v| vec![*v]).collect();
        standard_errors.push(
            boot(
                stream,
                &column,
                iterations,
                alpha,
                &format!("{stage}/{j}"),
                warnings,
            )?
            .standard_errors[0],
        );
    }
    let critical = match bands {
        Bands::Pointwise => crate::continuous_did::r_rng::standard_normal_quantile(1.0 - alpha / 2.0),
        Bands::Simultaneous => {
            boot(
                stream,
                &influence,
                iterations,
                alpha,
                &format!("{stage}/joint"),
                warnings,
            )?
            .critical
        }
    };
    let (overall, overall_influence) =
        weighted(&estimates, &influence, &weights, &members, estimated);
    let column: Vec<_> = overall_influence.iter().map(|v| vec![*v]).collect();
    let overall_standard_error = boot(
        stream,
        &column,
        iterations,
        alpha,
        &format!("{stage}/overall"),
        warnings,
    )?
    .standard_errors[0];
    // pte_aggte marks these scales NA. Refuse rather than reporting a valid interval.
    if standard_errors
        .iter()
        .chain(std::iter::once(&overall_standard_error))
        .any(|se| *se <= f64::EPSILON.sqrt() * 10.0)
    {
        return Err(Error::UnavailableStandardError {
            stage: stage.to_string(),
        });
    }
    Ok(Summary {
        labels,
        estimates,
        influence,
        standard_errors,
        critical,
        overall,
        overall_influence,
        overall_standard_error,
    })
}

/// Unrestricted event window and simultaneous bands. Matches the reference's
/// RNG-consuming call sequence, including its fixed 1000-draw initial stage.
pub fn slope(fit: &Fit, iterations: usize, alpha: f64, seed: u32) -> Result<Aggregation, Error> {
    run(
        fit,
        Target::Slope,
        Bands::Simultaneous,
        iterations,
        alpha,
        &mut Stream::new(seed),
    )
}

pub fn level(fit: &Fit, iterations: usize, alpha: f64, seed: u32) -> Result<Aggregation, Error> {
    run(
        fit,
        Target::Level,
        Bands::Simultaneous,
        iterations,
        alpha,
        &mut Stream::new(seed),
    )
}

#[derive(Clone, Copy)]
pub enum Target {
    Level,
    Slope,
}

pub fn estimate(
    fit: &Fit,
    target: Target,
    bands: Bands,
    iterations: usize,
    alpha: f64,
    seed: u32,
) -> Result<Aggregation, Error> {
    run(
        fit,
        target,
        bands,
        iterations,
        alpha,
        &mut Stream::new(seed),
    )
}

pub(crate) fn run(
    fit: &Fit,
    target: Target,
    bands: Bands,
    iterations: usize,
    alpha: f64,
    stream: &mut Stream,
) -> Result<Aggregation, Error> {
    run_window(
        fit,
        target,
        bands,
        EventWindow::default(),
        iterations,
        alpha,
        stream,
    )
}

pub fn estimate_window(
    fit: &Fit,
    target: Target,
    bands: Bands,
    window: EventWindow,
    iterations: usize,
    alpha: f64,
    seed: u32,
) -> Result<Aggregation, Error> {
    run_window(
        fit,
        target,
        bands,
        window,
        iterations,
        alpha,
        &mut Stream::new(seed),
    )
}

fn run_window(
    fit: &Fit,
    target: Target,
    bands: Bands,
    window: EventWindow,
    iterations: usize,
    alpha: f64,
    stream: &mut Stream,
) -> Result<Aggregation, Error> {
    let n = fit.units.len();
    let k = fit.cells.len();
    let influence = match target {
        Target::Level => &fit.raw_level_influence,
        Target::Slope => &fit.raw_influence,
    };
    if n == 0 || k == 0 || influence.len() != n || influence.iter().any(|r| r.len() != k) {
        return Err(Error::Shape);
    }
    let mut groups = BTreeMap::new();
    let mut events = BTreeMap::new();
    let index: BTreeMap<_, _> = fit.units.iter().enumerate().map(|(i, u)| (*u, i)).collect();
    for (j, c) in fit.cells.iter().enumerate() {
        groups.entry(c.group.0).or_insert_with(Vec::new).push(j);
        events
            .entry(c.time.0.checked_sub(c.group.0).ok_or(Error::TimeOverflow)?)
            .or_insert_with(Vec::new)
            .push(j);
    }
    let (first, last, balance) = window.bounds();
    let max_time = fit
        .cells
        .iter()
        .map(|c| c.time.0)
        .max()
        .ok_or(Error::Shape)?;
    let min_time = fit
        .cells
        .iter()
        .map(|c| c.time.0)
        .min()
        .ok_or(Error::Shape)?;
    // pte passes event-window options only to dynamic aggregation. Group
    // summaries and their RNG-consuming computations remain unchanged.
    events.retain(|event, indices| {
        if first.is_some_and(|v| *event < v) || last.is_some_and(|v| *event > v) {
            return false;
        }
        if let Some(balance) = balance {
            let balance = i128::from(balance);
            if i128::from(*event) > balance
                || i128::from(*event) < balance - i128::from(max_time) + i128::from(min_time)
            {
                return false;
            }
            indices.retain(|j| i128::from(max_time) - i128::from(fit.cells[*j].group.0) >= balance);
        }
        !indices.is_empty()
    });
    if !events.keys().any(|e| *e >= 0) {
        return Err(Error::EmptyEventWindow);
    }
    let mut membership = BTreeMap::new();
    for (g, indices) in &groups {
        let c = &fit.cells[indices[0]];
        if c.units.len() != c.dose.len() {
            return Err(Error::Shape);
        }
        let mut member = vec![false; n];
        for (unit, dose) in c.units.iter().zip(&c.dose) {
            let i = *index.get(unit).ok_or(Error::Shape)?;
            if *dose > 0.0 {
                member[i] = true;
            }
        }
        membership.insert(*g, member);
    }
    let counts: BTreeMap<_, _> = membership
        .iter()
        .map(|(g, m)| (*g, m.iter().filter(|m| **m).count() as f64))
        .collect();
    let total: f64 = counts.values().sum();
    if total == 0.0 {
        return Err(Error::Shape);
    }
    let part = |indices: &[usize], estimated: bool| {
        let values: Vec<_> = indices
            .iter()
            .map(|j| match target {
                Target::Level => fit.cells[*j].binary_level,
                Target::Slope => fit.cells[*j].fit.average_slope,
            })
            .collect();
        let scores: Vec<Vec<_>> = influence
            .iter()
            .map(|r| indices.iter().map(|j| r[*j]).collect())
            .collect();
        let weights: Vec<_> = indices
            .iter()
            .map(|j| counts[&fit.cells[*j].group.0] / total)
            .collect();
        let members: Vec<_> = indices
            .iter()
            .map(|j| membership[&fit.cells[*j].group.0].clone())
            .collect();
        weighted(&values, &scores, &weights, &members, estimated)
    };
    let mut floor_stages = Vec::new();
    // process_att_gt does not forward biters to mboot2.
    boot(
        stream,
        influence,
        1000,
        alpha,
        "group-time",
        &mut floor_stages,
    )?;
    let group_parts: Vec<_> = groups
        .values()
        .map(|indices| {
            part(
                &indices
                    .iter()
                    .copied()
                    .filter(|j| fit.cells[*j].time >= fit.cells[*j].group)
                    .collect::<Vec<_>>(),
                false,
            )
        })
        .collect();
    let group = finish(
        groups.keys().copied().collect(),
        group_parts,
        groups.keys().map(|g| counts[g] / total).collect(),
        groups.keys().map(|g| membership[g].clone()).collect(),
        true,
        bands,
        stream,
        iterations,
        alpha,
        "group",
        &mut floor_stages,
    )?;
    let event_parts: Vec<_> = events.values().map(|indices| part(indices, true)).collect();
    let post = events.keys().filter(|e| **e >= 0).count();
    if post == 0 {
        return Err(Error::Shape);
    }
    let event = finish(
        events.keys().copied().collect(),
        event_parts,
        events
            .keys()
            .map(|e| if *e >= 0 { 1.0 / post as f64 } else { 0.0 })
            .collect(),
        vec![],
        false,
        bands,
        stream,
        iterations,
        alpha,
        "event",
        &mut floor_stages,
    )?;
    Ok(Aggregation {
        group,
        event,
        floor_stages,
    })
}
