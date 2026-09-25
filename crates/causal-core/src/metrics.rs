//! scikit-learn scaling and classification metrics. The correlation redundancy filter lives
//! in `preprocess::cluster_redundant`.

/// `sklearn.preprocessing.StandardScaler`: centre and scale by the population deviation,
/// leaving constant columns unscaled.
pub struct StandardScaler {
    pub mean: Vec<f64>,
    pub scale: Vec<f64>,
}

impl StandardScaler {
    pub fn fit(x: &[Vec<f64>]) -> Self {
        let n = x.len() as f64;
        let k = x[0].len();
        let mut mean = vec![0.0; k];
        let mut scale = vec![0.0; k];
        for j in 0..k {
            mean[j] = x.iter().map(|r| r[j]).sum::<f64>() / n;
            let var = x.iter().map(|r| (r[j] - mean[j]).powi(2)).sum::<f64>() / n;
            let sd = var.sqrt();
            // sklearn leaves a zero deviation as one so constant columns pass through.
            scale[j] = if sd < 10.0 * f64::EPSILON { 1.0 } else { sd };
        }
        StandardScaler { mean, scale }
    }

    pub fn transform(&self, x: &[Vec<f64>]) -> Vec<Vec<f64>> {
        x.iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(j, v)| (v - self.mean[j]) / self.scale[j])
                    .collect()
            })
            .collect()
    }
}

/// Counts as `confusion_matrix(y, pred, labels=[0, 1]).ravel()` returns them.
pub fn confusion(y_true: &[u8], y_pred: &[u8]) -> (usize, usize, usize, usize) {
    let mut counts = (0, 0, 0, 0);
    for (&t, &p) in y_true.iter().zip(y_pred) {
        match (t, p) {
            (0, 0) => counts.0 += 1,
            (0, _) => counts.1 += 1,
            (_, 0) => counts.2 += 1,
            _ => counts.3 += 1,
        }
    }
    counts
}

pub fn accuracy(y_true: &[u8], y_pred: &[u8]) -> f64 {
    let hits = y_true.iter().zip(y_pred).filter(|(a, b)| a == b).count();
    hits as f64 / y_true.len() as f64
}

/// Precision, recall and F1 for the positive class, with sklearn's `zero_division=0`.
pub fn precision_recall_f1(y_true: &[u8], y_pred: &[u8]) -> (f64, f64, f64) {
    let (_, fp, fn_, tp) = confusion(y_true, y_pred);
    let precision = if tp + fp == 0 {
        0.0
    } else {
        tp as f64 / (tp + fp) as f64
    };
    let recall = if tp + fn_ == 0 {
        0.0
    } else {
        tp as f64 / (tp + fn_) as f64
    };
    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };
    (precision, recall, f1)
}

pub fn brier_score(y_true: &[u8], probs: &[f64]) -> f64 {
    let n = y_true.len() as f64;
    y_true
        .iter()
        .zip(probs)
        .map(|(&t, p)| (p - t as f64).powi(2))
        .sum::<f64>()
        / n
}

/// `roc_auc_score` for binary labels, by the rank formulation with ties averaged.
pub fn roc_auc(y_true: &[u8], score: &[f64]) -> f64 {
    let n = score.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| score[a].partial_cmp(&score[b]).unwrap());
    // Midranks so tied scores share their average rank.
    let mut ranks = vec![0.0f64; n];
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j + 1 < n && score[order[j + 1]] == score[order[i]] {
            j += 1;
        }
        let avg = ((i + j) as f64) / 2.0 + 1.0;
        for &idx in &order[i..=j] {
            ranks[idx] = avg;
        }
        i = j + 1;
    }
    let n_pos = y_true.iter().filter(|&&t| t == 1).count() as f64;
    let n_neg = y_true.len() as f64 - n_pos;
    let sum_pos: f64 = y_true
        .iter()
        .zip(&ranks)
        .filter(|(&t, _)| t == 1)
        .map(|(_, r)| r)
        .sum();
    (sum_pos - n_pos * (n_pos + 1.0) / 2.0) / (n_pos * n_neg)
}

/// `sklearn.metrics.roc_curve` with its default `drop_intermediate=True`: thresholds
/// descend through the distinct scores, collinear interior points are dropped, and the
/// curve is prefixed with the origin at an infinite threshold.
pub fn roc_curve(y_true: &[u8], score: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = score.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| score[b].partial_cmp(&score[a]).unwrap());
    let mut tps: Vec<f64> = Vec::new();
    let mut fps: Vec<f64> = Vec::new();
    let mut thresholds: Vec<f64> = Vec::new();
    let (mut tp, mut fp) = (0.0f64, 0.0f64);
    let mut i = 0;
    while i < n {
        let value = score[order[i]];
        while i < n && score[order[i]] == value {
            if y_true[order[i]] == 1 {
                tp += 1.0;
            } else {
                fp += 1.0;
            }
            i += 1;
        }
        tps.push(tp);
        fps.push(fp);
        thresholds.push(value);
    }

    // Drop interior points that lie on a straight run: those whose second difference in
    // both counts is zero.
    if tps.len() > 2 {
        let mut keep = vec![true; tps.len()];
        for k in 1..tps.len() - 1 {
            let d2_fps = fps[k + 1] - 2.0 * fps[k] + fps[k - 1];
            let d2_tps = tps[k + 1] - 2.0 * tps[k] + tps[k - 1];
            keep[k] = d2_fps != 0.0 || d2_tps != 0.0;
        }
        let filter = |v: &Vec<f64>| -> Vec<f64> {
            v.iter()
                .zip(&keep)
                .filter(|(_, &k)| k)
                .map(|(x, _)| *x)
                .collect()
        };
        tps = filter(&tps);
        fps = filter(&fps);
        thresholds = filter(&thresholds);
    }

    let total_pos = *tps.last().unwrap();
    let total_neg = *fps.last().unwrap();
    let mut tpr = vec![0.0];
    let mut fpr = vec![0.0];
    let mut out_thresholds = vec![f64::INFINITY];
    for k in 0..tps.len() {
        tpr.push(tps[k] / total_pos);
        fpr.push(fps[k] / total_neg);
        out_thresholds.push(thresholds[k]);
    }
    (fpr, tpr, out_thresholds)
}
