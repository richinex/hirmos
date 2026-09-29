//! process_dose_gt inference order, reusing checked cell, aggregation and RNG stages.
use crate::continuous_did::{
    aggregation::{self, Error, Target},
    bootstrap::{Bands, Bootstrap, Stream},
    panel::Fit,
};

pub struct Inference {
    pub bands: Bands,
    pub level_critical: f64,
    pub slope_critical: f64,
    pub overall_level: f64,
    pub overall_level_influence: Vec<f64>,
    pub overall_level_standard_error: f64,
    pub overall_slope_standard_error: f64,
    pub level_curve: Bootstrap,
    pub slope_curve: Bootstrap,
    pub floor_stages: Vec<String>,
}

/// Initial specification: simultaneous bands, varying baseline, no anticipation.
pub fn infer(fit: &Fit, iterations: usize, alpha: f64, seed: u32) -> Result<Inference, Error> {
    infer_with_bands(fit, Bands::Simultaneous, iterations, alpha, seed)
}

pub fn infer_with_bands(
    fit: &Fit,
    bands: Bands,
    iterations: usize,
    alpha: f64,
    seed: u32,
) -> Result<Inference, Error> {
    let mut stream = Stream::new(seed);
    let mut floor_stages = Vec::new();
    // Continuous process_att_gt precedes the complete binary pte_default call.
    aggregation::boot(
        &mut stream,
        &fit.raw_influence,
        1000,
        alpha,
        "continuous/group-time",
        &mut floor_stages,
    )?;
    // process_dose_gt does not forward cband to its internal pte_default call.
    let binary = aggregation::run(
        fit,
        Target::Level,
        Bands::Simultaneous,
        iterations,
        alpha,
        &mut stream,
    )?;
    floor_stages.extend(
        binary
            .floor_stages
            .into_iter()
            .map(|s| format!("binary/{s}")),
    );
    let column: Vec<_> = fit
        .average_slope_influence
        .iter()
        .map(|v| vec![*v])
        .collect();
    let overall_slope_standard_error = aggregation::boot(
        &mut stream,
        &column,
        iterations,
        alpha,
        "overall-slope",
        &mut floor_stages,
    )?
    .standard_errors[0];
    let level_curve = aggregation::boot(
        &mut stream,
        &fit.level_influence,
        iterations,
        alpha,
        "level-curve",
        &mut floor_stages,
    )?;
    let slope_curve = aggregation::boot(
        &mut stream,
        &fit.slope_influence,
        iterations,
        alpha,
        "slope-curve",
        &mut floor_stages,
    )?;
    Ok(Inference {
        bands,
        level_critical: match bands {
            Bands::Pointwise => crate::continuous_did::r_rng::standard_normal_quantile(1.0 - alpha / 2.0),
            Bands::Simultaneous => level_curve.critical,
        },
        slope_critical: match bands {
            Bands::Pointwise => crate::continuous_did::r_rng::standard_normal_quantile(1.0 - alpha / 2.0),
            Bands::Simultaneous => slope_curve.critical,
        },
        overall_level: binary.group.overall,
        overall_level_influence: binary.group.overall_influence,
        overall_level_standard_error: binary.group.overall_standard_error,
        overall_slope_standard_error,
        level_curve,
        slope_curve,
        floor_stages,
    })
}
