// Copyright (c) 2024 GRF Contributors.
// Rust translation of GRF 2.6.1 InstrumentalRelabelingStrategy,
// InstrumentalPredictionStrategy and ObjectiveBayesDebiaser, restricted to Z=W.
// SPDX-License-Identifier: GPL-3.0-or-later
// Distributed WITHOUT ANY WARRANTY. See reference/grf-2.6.1/COPYING.

/// Seven sufficient statistics in upstream order: Y, W, Z, YZ, WZ, ZZ, weight.
/// Keep upstream ordering and arithmetic even though Z=W for causal forests.
pub type Moments = [f64; 7];

/// Validated, already residualized observations. Original treatment need not be
/// binary: GRF also supports a continuous treatment. Residuals are not restricted
/// to [0,1]. No fitting, centering or imputation is silently performed here.
pub struct CausalData<'a> {
    y: &'a [f64],
    w: &'a [f64],
    weights: &'a [f64],
}

impl<'a> CausalData<'a> {
    pub fn new(y: &'a [f64], w: &'a [f64], weights: &'a [f64]) -> Result<Self, &'static str> {
        if y.is_empty() || y.len() != w.len() || y.len() != weights.len() {
            return Err("Outcome, treatment and weights must have the same positive length.");
        }
        if y.iter().chain(w).chain(weights).any(|v| !v.is_finite())
            || weights.iter().any(|v| *v < 0.0)
        {
            return Err("Responses must be finite and weights finite and nonnegative.");
        }
        Ok(Self { y, w, weights })
    }

    fn check(&self, samples: &[usize]) -> Result<(), &'static str> {
        if samples.iter().any(|&i| i >= self.y.len()) {
            return Err("Sample index is outside the data.");
        }
        Ok(())
    }

    /// `None` is upstream's stop-splitting result for zero mass or effectively
    /// zero within-node treatment variance. Responses follow sample order.
    pub fn relabel(&self, samples: &[usize]) -> Result<Option<Vec<f64>>, &'static str> {
        self.check(samples)?;
        let (mut mass, mut sy, mut sw) = (0.0, 0.0, 0.0);
        // Explicit contraction matches the pinned GCC/aarch64 oracle. A few
        // ulps in relabeling can reverse a near-tied split in small nodes.
        for &i in samples {
            sy = self.weights[i].mul_add(self.y[i], sy);
            sw = self.weights[i].mul_add(self.w[i], sw);
            mass += self.weights[i];
        }
        if mass.abs() <= 1e-16 {
            return Ok(None);
        }
        let (my, mw) = (sy / mass, sw / mass);
        let (mut numerator, mut denominator) = (0.0, 0.0);
        for &i in samples {
            let weighted = self.weights[i] * (self.w[i] - mw);
            numerator = weighted.mul_add(self.y[i] - my, numerator);
            denominator = weighted.mul_add(self.w[i] - mw, denominator);
        }
        if denominator.abs() < 1e-10 {
            return Ok(None);
        }
        let effect = numerator / denominator;
        Ok(Some(
            samples
                .iter()
                .map(|&i| {
                    let residual = (-effect).mul_add(self.w[i] - mw, self.y[i] - my);
                    (self.w[i] - mw) * residual
                })
                .collect(),
        ))
    }

    /// Upstream divides by leaf size, NOT total weight. Prediction combines
    /// these moments before taking the ratio. Normalizing each leaf by mass
    /// instead would change the weighted forest estimator.
    pub fn leaf_moments(&self, samples: &[usize]) -> Result<Option<Moments>, &'static str> {
        self.check(samples)?;
        if samples.is_empty() {
            return Ok(None);
        }
        let mut v = [0.0; 7];
        for &i in samples {
            let (g, y, w) = (self.weights[i], self.y[i], self.w[i]);
            v[0] += g * y;
            v[1] += g * w;
            v[2] += g * w;
            v[3] += g * y * w;
            v[4] += g * w * w;
            v[5] += g * w * w;
            v[6] += g;
        }
        if v[6].abs() <= 1e-16 {
            return Ok(None);
        }
        for value in &mut v {
            *value /= samples.len() as f64;
        }
        Ok(Some(v))
    }
}

/// Low-level upstream arithmetic: a singular prediction remains non-finite.
/// A future public estimator boundary must represent this explicitly, not zero it.
pub fn predict(average: &Moments) -> f64 {
    let numerator = average[3] * average[6] - average[0] * average[2];
    let denominator = average[4] * average[6] - average[1] * average[2];
    numerator / denominator
}

/// InstrumentalPredictionStrategy's leave-one-tree-out error correction, Z=W.
pub fn error(
    outcome: f64,
    treatment: f64,
    average: &Moments,
    leaves: &[Option<Moments>],
) -> Option<(f64, f64)> {
    let count = leaves.iter().flatten().count();
    if count <= 5 {
        return None;
    }
    let residual = |v: &Moments| {
        let effect = (v[3] * v[6] - v[0] * v[2]) / (v[5] * v[6] - v[2] * v[2]);
        outcome - (treatment - v[2] / v[6]) * effect - v[0] / v[6]
    };
    let r = residual(average);
    let mut bias = 0.0;
    for leaf in leaves.iter().flatten() {
        let mut adjusted = [0.0; 7];
        for i in 0..7 {
            adjusted[i] = (count as f64 * average[i] - leaf[i]) / (count - 1) as f64;
        }
        let delta = residual(&adjusted) - r;
        bias += delta * delta;
    }
    bias *= (count - 1) as f64 / count as f64;
    let debiased = r * r - bias;
    (debiased.is_finite() && bias.is_finite()).then_some((debiased, bias))
}

pub fn debias_variance(between: f64, noise: f64, groups: f64) -> f64 {
    let initial = between - noise;
    let se = between.max(noise) * (2.0 / groups).sqrt();
    if se.abs() < 1e-10 {
        return 0.0;
    }
    let ratio = initial / se;
    // Preserve the decimal constants in ObjectiveBayesDebiaser.h. Replacing
    // them with higher-precision constants changes the oracle's calculation.
    let numerator = (-ratio * ratio / 2.0).exp() * 0.3989422804;
    let denominator = 0.5 * libm::erfc(-ratio * 0.70710678118);
    initial + se * numerator / denominator
}

/// Little-bag variance. Preserve empty-tree positions: compacting them would
/// silently regroup trees and change inference. Only complete groups contribute.
pub fn variance(
    average: &Moments,
    leaves: &[Option<Moments>],
    group_size: usize,
) -> Result<f64, &'static str> {
    if group_size < 2 {
        return Err("Variance requires groups of at least two trees.");
    }
    let denominator = average[4] * average[6] - average[1] * average[2];
    let effect = predict(average);
    let main = (average[0] - average[1] * effect) / average[6];
    let (mut groups, mut squared, mut grouped_squared) = (0.0, 0.0, 0.0);
    for group in leaves.chunks_exact(group_size) {
        if group.iter().any(Option::is_none) {
            continue;
        }
        groups += 1.0;
        let mut group_rho = 0.0;
        for leaf in group.iter().flatten() {
            let psi1 = leaf[3] - leaf[4] * effect - leaf[2] * main;
            let psi2 = leaf[0] - leaf[1] * effect - leaf[6] * main;
            let rho = (average[6] * psi1 - average[2] * psi2) / denominator;
            squared += rho * rho;
            group_rho += rho;
        }
        group_rho /= group_size as f64;
        grouped_squared += group_rho * group_rho;
    }
    if groups == 0.0 {
        return Err("No complete tree groups for variance estimation.");
    }
    let between = grouped_squared / groups;
    let total = squared / (groups * group_size as f64);
    Ok(debias_variance(
        between,
        (total - between) / (group_size - 1) as f64,
        groups,
    ))
}
