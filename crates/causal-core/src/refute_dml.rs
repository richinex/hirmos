//! The causal worker's DML refuter block ported 1:1: seeded light refits for the placebo and
//! random-common-cause probes on all retained rows. Random-common-cause fits share the
//! baseline's fold assignments, plus DoubleML's confounding-share sensitivity bounds.

use crate::dml::{dml_irm, dml_plr, DmlResult, SensitivityResult};
use crate::nprandom::{Mt19937, NpRng};

pub struct WorkerStudy<'a> {
    pub x: &'a [Vec<f64>],
    pub y: &'a [f64],
    pub d: &'a [f64],
    /// dml_aipw (IRM) when true, dml_plr otherwise.
    pub aipw: bool,
    pub att: bool,
    pub treat_binary: bool,
}

pub struct RefutationOutcome {
    pub original_effect: f64,
    pub refuted_effect: f64,
    pub p_value: f64,
}

fn norm_cdf(x: f64) -> f64 {
    0.5 * libm::erfc(-x / std::f64::consts::SQRT_2)
}

fn fit_effect(
    study: &WorkerStudy,
    x: &[Vec<f64>],
    y: &[f64],
    d: &[f64],
    light: bool,
    fold_stream: &mut Mt19937,
) -> f64 {
    let (trees, folds) = if light { (80, 2) } else { (200, 5) };
    if study.aipw {
        dml_irm(x, y, d, study.att, folds, trees, 5, 0.01, 7, fold_stream).coef
    } else {
        dml_plr(x, y, d, study.treat_binary, folds, trees, 5, 7, fold_stream).coef
    }
}

/// The worker's full-precision fit at 200 trees and 5 folds.
pub fn worker_fit(study: &WorkerStudy, fold_stream: &mut Mt19937) -> DmlResult {
    if study.aipw {
        dml_irm(
            study.x,
            study.y,
            study.d,
            study.att,
            5,
            200,
            5,
            0.01,
            7,
            fold_stream,
        )
    } else {
        dml_plr(
            study.x,
            study.y,
            study.d,
            study.treat_binary,
            5,
            200,
            5,
            7,
            fold_stream,
        )
    }
}

fn z_test(values: &[f64]) -> (f64, f64) {
    let k = values.len() as f64;
    let mean = values.iter().sum::<f64>() / k;
    let spread = (values.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / (k - 1.0)).sqrt();
    let z = if spread > 0.0 {
        mean / (spread / k.sqrt())
    } else {
        0.0
    };
    (mean, 2.0 * (1.0 - norm_cdf(z.abs())))
}

/// A permuted treatment should read as no effect: 8 light refits with Generator(7) shuffles.
pub fn placebo_refute(
    study: &WorkerStudy,
    effect: f64,
    fold_stream: &mut Mt19937,
) -> RefutationOutcome {
    let mut rng = NpRng::seeded(7);
    let (bx, by, bd) = (study.x, study.y, study.d);
    let sims: Vec<f64> = (0..8)
        .map(|_| {
            let shuffled = rng.permutation_of(&bd);
            fit_effect(study, &bx, &by, &shuffled, true, fold_stream)
        })
        .collect();
    let (mean, p_value) = z_test(&sims);
    RefutationOutcome {
        original_effect: effect,
        refuted_effect: mean,
        p_value,
    }
}

/// Six independent-noise refits against a light baseline on all retained rows, using
/// identical fold assignments and Generator(11) standard normal columns.
pub fn random_common_cause_refute(
    study: &WorkerStudy,
    fold_stream: &mut Mt19937,
) -> RefutationOutcome {
    let mut rng = NpRng::seeded(11);
    let (bx, by, bd) = (study.x, study.y, study.d);
    let paired_folds = fold_stream.clone();
    let baseline = fit_effect(study, &bx, &by, &bd, true, fold_stream);
    let sims: Vec<f64> = (0..6)
        .map(|_| {
            let augmented: Vec<Vec<f64>> = bx
                .iter()
                .map(|row| {
                    let mut r = row.clone();
                    r.push(rng.standard_normal());
                    r
                })
                .collect();
            fit_effect(study, &augmented, &by, &bd, true, &mut paired_folds.clone())
        })
        .collect();
    let shifts: Vec<f64> = sims.iter().map(|s| s - baseline).collect();
    let (_, p_value) = z_test(&shifts);
    let refuted = sims.iter().sum::<f64>() / sims.len() as f64;
    RefutationOutcome {
        original_effect: baseline,
        refuted_effect: refuted,
        p_value,
    }
}

/// The worker's unobserved-confounder outcome: sensitivity scenarios plus clamped robustness.
pub fn unobserved_refute(full: &DmlResult) -> SensitivityResult {
    let mut s = full.sensitivity();
    s.robustness_value = s.robustness_value.clamp(0.0, 1.0);
    s.robustness_value_ci = s.robustness_value_ci.clamp(0.0, 1.0);
    s
}
