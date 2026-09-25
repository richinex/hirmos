//! dowhy's add_unobserved_common_cause direct simulation for the linear lane, 1:1: kappa ranges
//! inferred from the observed common causes, then a grid of simulated confounders that flip the
//! treatment and shift the outcome cumulatively on the shared frame, refitting the linear
//! backdoor estimate per cell on the legacy global RandomState stream.

use crate::backdoor::backdoor_linear_ate;
use crate::logistic::fit_logistic;
use crate::nprandom::Mt19937;
use nalgebra::DMatrix;

/// StandardScaler on the backdoor columns: population variance, zero scales become one.
fn standardize(occ: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = occ.len();
    let p = occ[0].len();
    let mut out = vec![vec![0.0; p]; n];
    for j in 0..p {
        let mean = occ.iter().map(|r| r[j]).sum::<f64>() / n as f64;
        let var = occ
            .iter()
            .map(|r| (r[j] - mean) * (r[j] - mean))
            .sum::<f64>()
            / n as f64;
        let scale = if var == 0.0 { 1.0 } else { var.sqrt() };
        for i in 0..n {
            out[i][j] = (occ[i][j] - mean) / scale;
        }
    }
    out
}

/// np.arange(start, stop, step) for floats, or the single max when the range is empty.
fn arange(start: f64, stop: f64, step: f64) -> Vec<f64> {
    if stop == start {
        return vec![stop];
    }
    let len = ((stop - start) / step).ceil() as usize;
    (0..len).map(|k| start + k as f64 * step).collect()
}

/// _infer_default_kappa_t with binary_flip: logistic flip fractions per zeroed column.
pub fn infer_kappa_t(occ: &[Vec<f64>], t: &[f64]) -> Vec<f64> {
    let scaled = standardize(occ);
    let model = fit_logistic(&scaled, t, crate::logistic::SKLEARN_DEFAULT_MAX_ITER);
    let base = model.predict(&scaled);
    let n = t.len();
    let p = occ[0].len();
    let mut flips = Vec::with_capacity(p);
    for j in 0..p {
        let mut zeroed = scaled.clone();
        for row in zeroed.iter_mut() {
            row[j] = 0.0;
        }
        let pred = model.predict(&zeroed);
        let diff: f64 = pred.iter().zip(&base).map(|(a, b)| (a - b).abs()).sum();
        flips.push(diff / n as f64);
    }
    let min = flips.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = flips.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    arange(min, max, (max - min) / 10.0)
}

/// _infer_default_kappa_y with a linear effect: correlation with the outcome times its std.
pub fn infer_kappa_y(occ: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let scaled = standardize(occ);
    let n = y.len() as f64;
    let ym = y.iter().sum::<f64>() / n;
    let ys = (y.iter().map(|v| (v - ym) * (v - ym)).sum::<f64>() / n).sqrt();
    let p = occ[0].len();
    let mut coeffs = Vec::with_capacity(p);
    for j in 0..p {
        let cm = scaled.iter().map(|r| r[j]).sum::<f64>() / n;
        let mut num = 0.0;
        let mut cden = 0.0;
        let mut yden = 0.0;
        for (row, &yv) in scaled.iter().zip(y) {
            num += (row[j] - cm) * (yv - ym);
            cden += (row[j] - cm) * (row[j] - cm);
            yden += (yv - ym) * (yv - ym);
        }
        coeffs.push(num / (cden.sqrt() * yden.sqrt()) * ys);
    }
    let min = coeffs.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = coeffs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    arange(min, max, (max - min) / 10.0)
}

/// sensitivity_simulation over the kappa grid with binary_flip treatment and linear outcome.
/// The frame mutates cumulatively across cells exactly as dowhy's shared orig_data does, and
/// the stream is the legacy global RandomState with its persistent gaussian cache.
pub fn unobserved_common_cause_grid(
    t: &[f64],
    y: &[f64],
    occ: &[Vec<f64>],
    kappa_t: &[f64],
    kappa_y: &[f64],
    mt: &mut Mt19937,
) -> Vec<Vec<f64>> {
    let n = t.len();
    let p = occ[0].len();
    // results_matrix = np.random.rand(...) advances the stream before the loop.
    for _ in 0..kappa_t.len() * kappa_y.len() {
        mt.next_f64();
    }
    let mut cur_t = t.to_vec();
    let mut cur_y = y.to_vec();
    let mut cache: Option<f64> = None;
    let mut effects = vec![vec![0.0; kappa_y.len()]; kappa_t.len()];
    for (i, &kt) in kappa_t.iter().enumerate() {
        for (j, &ky) in kappa_y.iter().enumerate() {
            let w: Vec<f64> = (0..n).map(|_| mt.standard_normal(&mut cache)).collect();
            let alpha = if kt >= 0.5 {
                2.0 * kt - 1.0
            } else {
                1.0 - 2.0 * kt
            };
            let lo = spec_math::cephes64::ndtri((1.0 - alpha) / 2.0);
            let hi = spec_math::cephes64::ndtri((1.0 + alpha) / 2.0);
            let rel = if kt >= 0.5 { lo } else { hi };
            for (k, &wv) in w.iter().enumerate() {
                if rel <= wv {
                    cur_t[k] = 1.0 - cur_t[k];
                }
                cur_y[k] += -ky * wv;
            }
            let mut m = DMatrix::<f64>::zeros(n, 2 + p);
            for r in 0..n {
                m[(r, 0)] = cur_t[r];
                m[(r, 1)] = cur_y[r];
                for c in 0..p {
                    m[(r, 2 + c)] = occ[r][c];
                }
            }
            let adjust: Vec<usize> = (2..2 + p).collect();
            effects[i][j] = backdoor_linear_ate(&m, 0, 1, &adjust);
        }
    }
    effects
}
