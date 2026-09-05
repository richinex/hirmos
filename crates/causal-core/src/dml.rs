//! DoubleML's PLR and IRM estimators ported 1:1 at the causal worker's settings: shuffled KFold
//! from the global MT19937 stream, cross-fitted random-forest nuisances, linear score solving,
//! and the sandwich standard error.

use crate::fminbound::fminbound;
use crate::nprandom::Mt19937;
use crate::sktree::fit_forest;

pub struct DmlResult {
    pub coef: f64,
    pub se: f64,
    pub ci_low: f64,
    pub ci_high: f64,
    /// psi / mean(psi_a), the framework's normalised score.
    scaled_psi: Vec<f64>,
    /// sqrt(sigma2 * nu2) and its influence function, DoubleML's confounding bias bound.
    max_bias: f64,
    psi_max_bias: Vec<f64>,
    /// What DoubleML's `gate` regresses on group dummies; absent for the ATTE score, which it refuses.
    group_signal: Option<GroupSignal>,
}

/// The per-observation quantities behind DoubleML's best linear predictor of group effects: for
/// IRM the orthogonal signal `psi_b` against a plain dummy basis, for PLR the partialled-out outcome
/// against a dummy basis scaled by the partialled-out treatment.
pub struct GroupSignal {
    pub(crate) signal: Vec<f64>,
    pub(crate) scale: Vec<f64>,
}

impl DmlResult {
    pub(crate) fn group_signal(&self) -> Option<&GroupSignal> {
        self.group_signal.as_ref()
    }
}

pub struct SensitivityScenario {
    pub confounding: f64,
    pub effect_lower: f64,
    pub effect_upper: f64,
    pub ci_lower: f64,
    pub ci_upper: f64,
}

pub struct SensitivityResult {
    pub scenarios: Vec<SensitivityScenario>,
    pub robustness_value: f64,
    pub robustness_value_ci: f64,
}

const Z975: f64 = 1.959963984540054;

impl DmlResult {
    /// theta and CI bounds under equal confounding shares cf in outcome and treatment.
    fn bounds(&self, cf: f64, rho: f64) -> (f64, f64, f64, f64) {
        let n = self.scaled_psi.len() as f64;
        let strength = rho.abs() * (cf * (cf / (1.0 - cf))).sqrt();
        let theta_lower = self.coef - strength * self.max_bias;
        let theta_upper = self.coef + strength * self.max_bias;
        let sigma = |sign: f64| -> f64 {
            let gamma = self
                .scaled_psi
                .iter()
                .zip(&self.psi_max_bias)
                .map(|(p, pb)| (p + sign * strength * pb).powi(2))
                .sum::<f64>()
                / n;
            (gamma / n).sqrt()
        };
        let quant = spec_math::cephes64::ndtri(0.95);
        let ci_lower = theta_lower - quant * sigma(-1.0);
        let ci_upper = theta_upper + quant * sigma(1.0);
        (theta_lower, theta_upper, ci_lower, ci_upper)
    }

    /// The worker's sensitivity block: scenarios at cf 0.02/0.05/0.10 with rho 1.0, and the
    /// robustness values against a zero null via scipy's bounded minimiser.
    pub fn sensitivity(&self) -> SensitivityResult {
        let scenarios = [0.02, 0.05, 0.10]
            .iter()
            .map(|&cf| {
                let (tl, tu, cl, cu) = self.bounds(cf, 1.0);
                SensitivityScenario {
                    confounding: cf,
                    effect_lower: tl,
                    effect_upper: tu,
                    ci_lower: cl,
                    ci_upper: cu,
                }
            })
            .collect();
        let upper = 0.0 > self.coef;
        let pick = |theta_bound: bool, cf: f64| -> f64 {
            let (tl, tu, cl, cu) = self.bounds(cf, 1.0);
            let v = match (theta_bound, upper) {
                (true, true) => tu,
                (true, false) => tl,
                (false, true) => cu,
                (false, false) => cl,
            };
            v * v
        };
        let rv = fminbound(|cf| pick(true, cf), 0.0, 0.9999, 1e-5, 500);
        let rva = fminbound(|cf| pick(false, cf), 0.0, 0.9999, 1e-5, 500);
        SensitivityResult {
            scenarios,
            robustness_value: rv,
            robustness_value_ci: rva,
        }
    }
}

/// max_bias and its influence function from the model's sensitivity elements.
fn attach_sensitivity(
    res: &mut DmlResult,
    psi_a: &[f64],
    psi_b: &[f64],
    sigma2: f64,
    psi_sigma2: &[f64],
    nu2: f64,
    psi_nu2: &[f64],
) {
    let n = psi_a.len() as f64;
    let mean_a = psi_a.iter().sum::<f64>() / n;
    res.scaled_psi = psi_a
        .iter()
        .zip(psi_b)
        .map(|(a, b)| (a * res.coef + b) / mean_a)
        .collect();
    res.max_bias = (sigma2 * nu2).sqrt();
    res.psi_max_bias = psi_sigma2
        .iter()
        .zip(psi_nu2)
        .map(|(ps, pn)| (sigma2 * pn + nu2 * ps) / (2.0 * res.max_bias))
        .collect();
}

/// sklearn KFold(shuffle=True) fed by the given MT19937 stream: test folds are consecutive
/// chunks of the shuffled index array, train folds the sorted complement.
pub fn kfold_shuffled(n: usize, k: usize, mt: &mut Mt19937) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut indices: Vec<usize> = (0..n).collect();
    mt.shuffle(&mut indices);
    let mut folds = Vec::with_capacity(k);
    let base = n / k;
    let extra = n % k;
    let mut at = 0usize;
    for f in 0..k {
        let size = base + usize::from(f < extra);
        let mut test: Vec<usize> = indices[at..at + size].to_vec();
        test.sort_unstable();
        at += size;
        let mut is_test = vec![false; n];
        for &t in &test {
            is_test[t] = true;
        }
        let train: Vec<usize> = (0..n).filter(|&i| !is_test[i]).collect();
        folds.push((train, test));
    }
    folds
}

/// sklearn StratifiedKFold(shuffle=True): per-class fold allocations shuffled on the stream,
/// classes encoded by order of appearance.
pub fn kfold_stratified(
    strata: &[f64],
    k: usize,
    mt: &mut Mt19937,
) -> Vec<(Vec<usize>, Vec<usize>)> {
    let n = strata.len();
    let mut sorted_classes: Vec<f64> = strata.to_vec();
    sorted_classes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted_classes.dedup();
    let n_classes = sorted_classes.len();
    let class_of = |v: f64| sorted_classes.iter().position(|&c| c == v).unwrap();
    let mut first_idx = vec![usize::MAX; n_classes];
    for (i, &v) in strata.iter().enumerate() {
        let c = class_of(v);
        if first_idx[c] == usize::MAX {
            first_idx[c] = i;
        }
    }
    let mut order: Vec<usize> = (0..n_classes).collect();
    order.sort_by_key(|&c| first_idx[c]);
    let mut appearance = vec![0usize; n_classes];
    for (rank, &c) in order.iter().enumerate() {
        appearance[c] = rank;
    }
    let y_encoded: Vec<usize> = strata.iter().map(|&v| appearance[class_of(v)]).collect();

    let mut y_order = y_encoded.clone();
    y_order.sort_unstable();
    let mut allocation = vec![vec![0usize; n_classes]; k];
    for f in 0..k {
        let mut p = f;
        while p < n {
            allocation[f][y_order[p]] += 1;
            p += k;
        }
    }
    let mut test_folds = vec![0usize; n];
    for cls in 0..n_classes {
        let mut folds_for_class: Vec<usize> = Vec::new();
        for (f, alloc) in allocation.iter().enumerate() {
            folds_for_class.extend(std::iter::repeat_n(f, alloc[cls]));
        }
        mt.shuffle(&mut folds_for_class);
        let mut at = 0usize;
        for i in 0..n {
            if y_encoded[i] == cls {
                test_folds[i] = folds_for_class[at];
                at += 1;
            }
        }
    }
    (0..k)
        .map(|f| {
            let test: Vec<usize> = (0..n).filter(|&i| test_folds[i] == f).collect();
            let train: Vec<usize> = (0..n).filter(|&i| test_folds[i] != f).collect();
            (train, test)
        })
        .collect()
}

fn subset(rows: &[Vec<f64>], idx: &[usize]) -> Vec<Vec<f64>> {
    idx.iter().map(|&i| rows[i].clone()).collect()
}

fn subset1(v: &[f64], idx: &[usize]) -> Vec<f64> {
    idx.iter().map(|&i| v[i]).collect()
}

/// Cross-fitted predictions: per fold, a fresh seed-fixed forest on the train rows.
#[allow(clippy::too_many_arguments)]
fn cv_predict(
    x: &[Vec<f64>],
    target: &[f64],
    smpls: &[(Vec<usize>, Vec<usize>)],
    n_estimators: usize,
    min_samples_leaf: usize,
    classifier: bool,
    seed: u32,
) -> Vec<f64> {
    let mut out = vec![f64::NAN; x.len()];
    for (train, test) in smpls {
        let xt = subset(x, train);
        let yt = subset1(target, train);
        let forest = fit_forest(
            &xt,
            &yt,
            n_estimators,
            min_samples_leaf,
            if classifier { Some(2) } else { None },
            seed,
        );
        let x_test = subset(x, test);
        if classifier {
            let proba = forest.predict_proba(&x_test);
            for (k, &t) in test.iter().enumerate() {
                out[t] = proba[k][1];
            }
        } else {
            let preds = forest.predict(&x_test);
            for (k, &t) in test.iter().enumerate() {
                out[t] = preds[k];
            }
        }
    }
    out
}

fn solve_score(psi_a: &[f64], psi_b: &[f64]) -> DmlResult {
    let n = psi_a.len() as f64;
    let mean_a = psi_a.iter().sum::<f64>() / n;
    let mean_b = psi_b.iter().sum::<f64>() / n;
    let coef = -mean_b / mean_a;
    let gamma: f64 = psi_a
        .iter()
        .zip(psi_b)
        .map(|(a, b)| (a * coef + b).powi(2))
        .sum::<f64>()
        / n;
    let se = (gamma / (mean_a * mean_a) / n).sqrt();
    DmlResult {
        coef,
        se,
        ci_low: coef - Z975 * se,
        ci_high: coef + Z975 * se,
        scaled_psi: Vec::new(),
        max_bias: 0.0,
        psi_max_bias: Vec::new(),
        group_signal: None,
    }
}

/// DoubleMLPLR with the partialling-out score.
#[allow(clippy::too_many_arguments)]
pub fn dml_plr(
    x: &[Vec<f64>],
    y: &[f64],
    d: &[f64],
    treat_binary: bool,
    n_folds: usize,
    n_estimators: usize,
    min_samples_leaf: usize,
    learner_seed: u32,
    fold_stream: &mut Mt19937,
) -> DmlResult {
    let smpls = kfold_shuffled(x.len(), n_folds, fold_stream);
    let l_hat = cv_predict(
        x,
        y,
        &smpls,
        n_estimators,
        min_samples_leaf,
        false,
        learner_seed,
    );
    let m_hat = cv_predict(
        x,
        d,
        &smpls,
        n_estimators,
        min_samples_leaf,
        treat_binary,
        learner_seed,
    );
    let psi_a: Vec<f64> = d
        .iter()
        .zip(&m_hat)
        .map(|(di, mi)| -(di - mi) * (di - mi))
        .collect();
    let psi_b: Vec<f64> = d
        .iter()
        .zip(&m_hat)
        .zip(y.iter().zip(&l_hat))
        .map(|((di, mi), (yi, li))| (di - mi) * (yi - li))
        .collect();
    let mut res = solve_score(&psi_a, &psi_b);

    let n = x.len() as f64;
    let sigma2_el: Vec<f64> = (0..x.len())
        .map(|i| (y[i] - l_hat[i] - res.coef * (d[i] - m_hat[i])).powi(2))
        .collect();
    let sigma2 = sigma2_el.iter().sum::<f64>() / n;
    let psi_sigma2: Vec<f64> = sigma2_el.iter().map(|v| v - sigma2).collect();
    let tr: Vec<f64> = d.iter().zip(&m_hat).map(|(di, mi)| di - mi).collect();
    let nu2 = 1.0 / (tr.iter().map(|v| v * v).sum::<f64>() / n);
    let psi_nu2: Vec<f64> = tr.iter().map(|v| nu2 - v * v * nu2 * nu2).collect();
    attach_sensitivity(&mut res, &psi_a, &psi_b, sigma2, &psi_sigma2, nu2, &psi_nu2);
    res.group_signal = Some(GroupSignal {
        signal: y.iter().zip(&l_hat).map(|(yi, li)| yi - li).collect(),
        scale: tr,
    });
    res
}

/// DoubleMLIRM with the ATE or ATTE score, truncation-trimmed propensities.
#[allow(clippy::too_many_arguments)]
pub fn dml_irm(
    x: &[Vec<f64>],
    y: &[f64],
    d: &[f64],
    att: bool,
    n_folds: usize,
    n_estimators: usize,
    min_samples_leaf: usize,
    trimming_threshold: f64,
    learner_seed: u32,
    fold_stream: &mut Mt19937,
) -> DmlResult {
    let n = x.len();
    // DoubleMLIRM first draws (and discards) a plain split at base init, then redraws
    // stratified on the treatment once _strata is set.
    let _ = kfold_shuffled(n, n_folds, fold_stream);
    let smpls = kfold_stratified(d, n_folds, fold_stream);
    let cond_smpls = |value: f64| -> Vec<(Vec<usize>, Vec<usize>)> {
        smpls
            .iter()
            .map(|(train, test)| {
                (
                    train.iter().copied().filter(|&i| d[i] == value).collect(),
                    test.clone(),
                )
            })
            .collect()
    };
    let g0_hat = cv_predict(
        x,
        y,
        &cond_smpls(0.0),
        n_estimators,
        min_samples_leaf,
        false,
        learner_seed,
    );
    let g1_hat = cv_predict(
        x,
        y,
        &cond_smpls(1.0),
        n_estimators,
        min_samples_leaf,
        false,
        learner_seed,
    );
    let m_raw = cv_predict(
        x,
        d,
        &smpls,
        n_estimators,
        min_samples_leaf,
        true,
        learner_seed,
    );
    let m_hat: Vec<f64> = m_raw
        .iter()
        .map(|&m| m.clamp(trimming_threshold, 1.0 - trimming_threshold))
        .collect();

    let (weights, weights_bar): (Vec<f64>, Vec<f64>) = if att {
        let p = d.iter().sum::<f64>() / n as f64;
        (
            d.iter().map(|&di| di / p).collect(),
            m_hat.iter().map(|&m| m / p).collect(),
        )
    } else {
        (vec![1.0; n], vec![1.0; n])
    };
    let mean_w = weights.iter().sum::<f64>() / n as f64;
    let mut psi_a = vec![0.0; n];
    let mut psi_b = vec![0.0; n];
    for i in 0..n {
        let u0 = y[i] - g0_hat[i];
        let u1 = y[i] - g1_hat[i];
        psi_b[i] = weights[i] * (g1_hat[i] - g0_hat[i])
            + weights_bar[i] * (d[i] * u1 / m_hat[i] - (1.0 - d[i]) * u0 / (1.0 - m_hat[i]));
        psi_a[i] = -weights[i] / mean_w;
    }
    let mut res = solve_score(&psi_a, &psi_b);

    let nf = n as f64;
    let sigma2_el: Vec<f64> = (0..n)
        .map(|i| (y[i] - d[i] * g1_hat[i] - (1.0 - d[i]) * g0_hat[i]).powi(2))
        .collect();
    let sigma2 = sigma2_el.iter().sum::<f64>() / nf;
    let psi_sigma2: Vec<f64> = sigma2_el.iter().map(|v| v - sigma2).collect();
    let rr: Vec<f64> = (0..n)
        .map(|i| weights_bar[i] * (d[i] / m_hat[i] - (1.0 - d[i]) / (1.0 - m_hat[i])))
        .collect();
    let nu2_el: Vec<f64> = (0..n)
        .map(|i| {
            let m_alpha = weights[i] * weights_bar[i] * (1.0 / m_hat[i] + 1.0 / (1.0 - m_hat[i]));
            2.0 * m_alpha - rr[i] * rr[i]
        })
        .collect();
    let mut nu2 = nu2_el.iter().sum::<f64>() / nf;
    let mut psi_nu2: Vec<f64> = nu2_el.iter().map(|v| v - nu2).collect();
    if nu2 <= 0.0 {
        // DoubleML falls back to the non-orthogonal Riesz representer estimate.
        psi_nu2 = rr.iter().map(|v| v * v).collect();
        nu2 = psi_nu2.iter().sum::<f64>() / nf;
    }
    attach_sensitivity(&mut res, &psi_a, &psi_b, sigma2, &psi_sigma2, nu2, &psi_nu2);
    if !att {
        res.group_signal = Some(GroupSignal {
            signal: psi_b,
            scale: vec![1.0; n],
        });
    }
    res
}
