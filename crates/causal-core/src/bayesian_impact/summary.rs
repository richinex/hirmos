//! Posterior reporting, separated from sampling so it can be checked draw-for-draw.
//! Follows Google's tfp-causalimpact `_compute_summary` (Apache-2.0).
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    InvalidWindow,
    InvalidAlpha,
    UndefinedRelativeEffect,
}

pub struct Predictions<'a> {
    observed: &'a [f64],
    means: &'a [f64],
    paths: &'a [Vec<f64>],
    post: Range<usize>,
    alpha: f64,
}

impl<'a> Predictions<'a> {
    /// Paths are time-major; means are conditional means, not noisy draw averages.
    pub fn new(
        observed: &'a [f64],
        means: &'a [f64],
        paths: &'a [Vec<f64>],
        post: Range<usize>,
        alpha: f64,
    ) -> Result<Self, Error> {
        if observed.len() != means.len()
            || paths.len() != observed.len()
            || paths.is_empty()
            || paths[0].len() < 2
            || paths.iter().any(|row| row.len() != paths[0].len())
        {
            return Err(Error::Shape);
        }
        if observed
            .iter()
            .chain(means)
            .chain(paths.iter().flatten())
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        if post.start >= post.end || post.end > observed.len() {
            return Err(Error::InvalidWindow);
        }
        if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
            return Err(Error::InvalidAlpha);
        }
        Ok(Self {
            observed,
            means,
            paths,
            post,
            alpha,
        })
    }
}

#[derive(Debug)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}
#[derive(Debug)]
pub struct Quantity {
    pub mean: f64,
    pub interval: Interval,
    pub sd: f64,
}
#[derive(Debug)]
pub struct Summary {
    pub actual: f64,
    pub predicted: Quantity,
    pub absolute: Quantity,
    pub relative: Quantity,
    pub tail_probability: f64,
}
pub struct Summaries {
    pub average: Summary,
    pub cumulative: Summary,
}
pub struct PostPoint {
    pub index: usize,
    pub predicted: Quantity,
    pub effect: Quantity,
    pub cumulative: Quantity,
}

fn quantity(mean: f64, samples: &[f64], alpha: f64) -> Quantity {
    let sample_mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let sd = (samples
        .iter()
        .map(|v| (v - sample_mean).powi(2))
        .sum::<f64>()
        / (samples.len() - 1) as f64)
        .sqrt();
    Quantity {
        mean,
        sd,
        interval: Interval {
            lower: crate::causal_effects::numpy_percentile(samples, alpha / 2.0),
            upper: crate::causal_effects::numpy_percentile(samples, 1.0 - alpha / 2.0),
        },
    }
}

pub fn summarize(input: &Predictions<'_>) -> Result<Summaries, Error> {
    let actual: f64 = input.observed[input.post.clone()].iter().sum();
    let predicted: f64 = input.means[input.post.clone()].iter().sum();
    let n = input.post.len() as f64;
    let sums: Vec<f64> = (0..input.paths[0].len())
        .map(|j| {
            input.paths[input.post.clone()]
                .iter()
                .map(|row| row[j])
                .sum()
        })
        .collect();
    if sums.iter().any(|v| !v.is_finite()) || !actual.is_finite() || !predicted.is_finite() {
        return Err(Error::NonFinite);
    }
    let relative: Vec<f64> = sums.iter().map(|v| actual / v - 1.0).collect();
    if relative.iter().any(|v| !v.is_finite()) {
        return Err(Error::UndefinedRelativeEffect);
    }
    let below = 1 + sums.iter().filter(|&&v| actual <= v).count();
    let above = 1 + sums.iter().filter(|&&v| actual >= v).count();
    let tail_probability = below.min(above) as f64 / (sums.len() + 1) as f64;
    let aggregate = |divisor: f64| {
        let predictions: Vec<f64> = sums.iter().map(|v| v / divisor).collect();
        let effects: Vec<f64> = sums.iter().map(|v| (actual - v) / divisor).collect();
        Summary {
            actual: actual / divisor,
            predicted: quantity(predicted / divisor, &predictions, input.alpha),
            absolute: quantity((actual - predicted) / divisor, &effects, input.alpha),
            relative: quantity(
                relative.iter().sum::<f64>() / relative.len() as f64,
                &relative,
                input.alpha,
            ),
            tail_probability,
        }
    };
    Ok(Summaries {
        average: aggregate(n),
        cumulative: aggregate(1.0),
    })
}

/// Cumulative intervals aggregate each draw first, never marginal bounds.
pub fn post_path(input: &Predictions<'_>) -> Result<Vec<PostPoint>, Error> {
    let mut cumulative_draws = vec![0.0; input.paths[0].len()];
    let mut cumulative_mean = 0.0;
    let mut result = Vec::with_capacity(input.post.len());
    for t in input.post.clone() {
        let effects: Vec<f64> = input.paths[t]
            .iter()
            .map(|prediction| input.observed[t] - prediction)
            .collect();
        for (sum, effect) in cumulative_draws.iter_mut().zip(&effects) {
            *sum += effect;
        }
        let effect = input.observed[t] - input.means[t];
        cumulative_mean += effect;
        if !cumulative_mean.is_finite() || cumulative_draws.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        result.push(PostPoint {
            index: t,
            predicted: quantity(input.means[t], &input.paths[t], input.alpha),
            effect: quantity(effect, &effects, input.alpha),
            cumulative: quantity(cumulative_mean, &cumulative_draws, input.alpha),
        });
    }
    Ok(result)
}
