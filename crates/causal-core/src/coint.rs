//! Cointegration tests ported 1:1 from statsmodels: coint_johansen's trace and max-eigenvalue
//! statistics with the Osterwald-Lenum critical value tables, and the augmented Engle-Granger
//! two-step test with MacKinnon N=2 surfaces.

use crate::mackinnon::{mackinnoncrit_coint2, mackinnonp_coint2};
use crate::ols::Ols;
use crate::stationarity::adfuller;
use crate::Regression;
use nalgebra::{DMatrix, DVector};

const C_SJA: [[[f64; 3]; 12]; 3] = [
    [
        [2.9762, 4.1296, 6.9406],
        [9.4748, 11.2246, 15.0923],
        [15.7175, 17.7961, 22.2519],
        [21.837, 24.1592, 29.0609],
        [27.916, 30.4428, 35.7359],
        [33.9271, 36.6301, 42.2333],
        [39.9085, 42.7679, 48.6606],
        [45.893, 48.8795, 55.0335],
        [51.8528, 54.9629, 61.3449],
        [57.7954, 61.0404, 67.6415],
        [63.7248, 67.0756, 73.8856],
        [69.6513, 73.0946, 80.0937],
    ],
    [
        [2.7055, 3.8415, 6.6349],
        [12.2971, 14.2639, 18.52],
        [18.8928, 21.1314, 25.865],
        [25.1236, 27.5858, 32.7172],
        [31.2379, 33.8777, 39.3693],
        [37.2786, 40.0763, 45.8662],
        [43.2947, 46.2299, 52.3069],
        [49.2855, 52.3622, 58.6634],
        [55.2412, 58.4332, 64.996],
        [61.2041, 64.504, 71.2525],
        [67.1307, 70.5392, 77.4877],
        [73.0563, 76.5734, 83.7105],
    ],
    [
        [2.7055, 3.8415, 6.6349],
        [15.0006, 17.1481, 21.7465],
        [21.8731, 24.2522, 29.2631],
        [28.2398, 30.8151, 36.193],
        [34.4202, 37.1646, 42.8612],
        [40.5244, 43.4183, 49.4095],
        [46.5583, 49.5875, 55.8171],
        [52.5858, 55.7302, 62.1741],
        [58.5316, 61.8051, 68.503],
        [64.5292, 67.904, 74.7434],
        [70.463, 73.9355, 81.0678],
        [76.4081, 79.9878, 87.2395],
    ],
];
const C_SJT: [[[f64; 3]; 12]; 3] = [
    [
        [2.9762, 4.1296, 6.9406],
        [10.4741, 12.3212, 16.364],
        [21.7781, 24.2761, 29.5147],
        [37.0339, 40.1749, 46.5716],
        [56.2839, 60.0627, 67.6367],
        [79.5329, 83.9383, 92.7136],
        [106.7351, 111.7797, 121.7375],
        [137.9954, 143.6691, 154.7977],
        [173.2292, 179.5199, 191.8122],
        [212.4721, 219.4051, 232.8291],
        [255.6732, 263.2603, 277.9962],
        [302.9054, 311.1288, 326.9716],
    ],
    [
        [2.7055, 3.8415, 6.6349],
        [13.4294, 15.4943, 19.9349],
        [27.0669, 29.7961, 35.4628],
        [44.4929, 47.8545, 54.6815],
        [65.8202, 69.8189, 77.8202],
        [91.109, 95.7542, 104.9637],
        [120.3673, 125.6185, 135.9825],
        [153.6341, 159.529, 171.0905],
        [190.8714, 197.3772, 210.0366],
        [232.103, 239.2468, 253.2526],
        [277.374, 285.1402, 300.2821],
        [326.5354, 334.9795, 351.215],
    ],
    [
        [2.7055, 3.8415, 6.6349],
        [16.1619, 18.3985, 23.1485],
        [32.0645, 35.0116, 41.0815],
        [51.6492, 55.2459, 62.5202],
        [75.1027, 79.3422, 87.7748],
        [102.4674, 107.3429, 116.9829],
        [133.7852, 139.278, 150.0778],
        [169.0618, 175.1584, 187.1891],
        [208.3582, 215.1268, 228.2226],
        [251.6293, 259.0267, 273.3838],
        [298.8836, 306.8988, 322.4264],
        [350.1125, 358.719, 375.3203],
    ],
];

pub struct JohansenResult {
    /// Eigenvalues, descending.
    pub eig: Vec<f64>,
    /// Trace statistics per rank hypothesis.
    pub lr1: Vec<f64>,
    /// Maximum-eigenvalue statistics per rank hypothesis.
    pub lr2: Vec<f64>,
    /// Trace critical values (90/95/99).
    pub cvt: Vec<[f64; 3]>,
    /// Max-eig critical values (90/95/99).
    pub cvm: Vec<[f64; 3]>,
}

impl JohansenResult {
    /// The 902 workflow's rank walk: count leading hypotheses whose trace exceeds the critical
    /// value at the given level column (0=90%, 1=95%, 2=99%), stopping at the first failure.
    pub fn trace_rank(&self, level: usize) -> usize {
        let mut rank = 0;
        for r in 0..self.lr1.len() {
            if self.lr1[r] > self.cvt[r][level] {
                rank = r + 1;
            } else {
                break;
            }
        }
        rank
    }
}

/// OLS residual of every column of y on the detrending design of the given order.
fn detrend(y: DMatrix<f64>, order: i32) -> DMatrix<f64> {
    if order == -1 {
        return y;
    }
    let n = y.nrows();
    let cols = (order + 1) as usize;
    let mut design = DMatrix::<f64>::zeros(n, cols);
    for i in 0..n {
        let x = if n == 1 {
            -1.0
        } else {
            -1.0 + 2.0 * i as f64 / (n - 1) as f64
        };
        for (c, cell) in (0..cols).rev().enumerate() {
            design[(i, cell)] = x.powi(c as i32);
        }
    }
    let qr = design.clone().qr();
    let beta = qr
        .r()
        .solve_upper_triangular(&(qr.q().transpose() * &y))
        .expect("detrend design is rank deficient");
    y - design * beta
}

/// Least-squares residual of y on x, both multivariate.
fn resid_on(y: DMatrix<f64>, x: &DMatrix<f64>) -> DMatrix<f64> {
    if x.ncols() == 0 {
        return y;
    }
    let qr = x.clone().qr();
    let beta = qr
        .r()
        .solve_upper_triangular(&(qr.q().transpose() * &y))
        .expect("regressors are rank deficient");
    y - x * beta
}

/// statsmodels coint_johansen: eigenvalues, trace and max-eig statistics with critical values.
/// Eigenvectors are not reproduced; the workflow reads only statistics and tables.
pub fn coint_johansen(endog: &[Vec<f64>], det_order: i32, k_ar_diff: usize) -> JohansenResult {
    let nobs = endog.len();
    let neqs = endog[0].len();
    let y = DMatrix::from_fn(nobs, neqs, |i, j| endog[i][j]);

    let f = if det_order > -1 { 0 } else { det_order };
    let y = detrend(y, det_order);

    let mut dx = DMatrix::<f64>::zeros(nobs - 1, neqs);
    for i in 0..nobs - 1 {
        for j in 0..neqs {
            dx[(i, j)] = y[(i + 1, j)] - y[(i, j)];
        }
    }

    // lagmat with forward trim, rows past the zero padding: row t holds dx[t-1], dx[t-2], ...
    let zrows = nobs - 1 - k_ar_diff;
    let mut z = DMatrix::<f64>::zeros(zrows, neqs * k_ar_diff);
    for t in 0..zrows {
        for lag in 1..=k_ar_diff {
            for j in 0..neqs {
                z[(t, (lag - 1) * neqs + j)] = dx[(t + k_ar_diff - lag, j)];
            }
        }
    }
    let z = detrend(z, f);

    let dx_cut = detrend(dx.rows(k_ar_diff, zrows).into_owned(), f);
    let r0t = resid_on(dx_cut, &z);
    let lx = detrend(y.rows(1, nobs - k_ar_diff - 1).into_owned(), f);
    let rkt = resid_on(lx, &z);

    let t = rkt.nrows() as f64;
    let skk = rkt.transpose() * &rkt / t;
    let sk0 = rkt.transpose() * &r0t / t;
    let s00 = r0t.transpose() * &r0t / t;
    let s00_inv = s00.try_inverse().expect("s00 is singular");
    let sig = &sk0 * s00_inv * sk0.transpose();

    // Generalized symmetric-definite eigenproblem sig v = lambda skk v via Cholesky whitening.
    let chol = skk.cholesky().expect("skk is not positive definite");
    let l_inv = chol.l().try_inverse().expect("cholesky factor is singular");
    let b = &l_inv * sig * l_inv.transpose();
    let sym = nalgebra::SymmetricEigen::new((&b + &b.transpose()) * 0.5);
    let mut eig: Vec<f64> = sym.eigenvalues.iter().copied().collect();
    eig.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut lr1 = vec![0.0; neqs];
    let mut lr2 = vec![0.0; neqs];
    let mut cvt = Vec::with_capacity(neqs);
    let mut cvm = Vec::with_capacity(neqs);
    let det_idx = (det_order + 1) as usize;
    for i in 0..neqs {
        lr1[i] = -t * eig[i..].iter().map(|a| (1.0 - a).ln()).sum::<f64>();
        lr2[i] = -t * (1.0 - eig[i]).ln();
        cvm.push(C_SJA[det_idx][neqs - i - 1]);
        cvt.push(C_SJT[det_idx][neqs - i - 1]);
    }

    JohansenResult {
        eig,
        lr1,
        lr2,
        cvt,
        cvm,
    }
}

pub struct CointResult {
    pub stat: f64,
    pub pvalue: f64,
    /// 1%, 5%, 10%.
    pub crit: [f64; 3],
}

/// statsmodels coint with trend "c": OLS of y0 on [y1, const], ADF("n", aic) on the residual,
/// MacKinnon N=2 p-value and critical values.
pub fn coint(y0: &[f64], y1: &[f64]) -> CointResult {
    let nobs = y0.len();
    let mut design = DMatrix::<f64>::zeros(nobs, 2);
    for i in 0..nobs {
        design[(i, 0)] = y1[i];
        design[(i, 1)] = 1.0;
    }
    let yv = DVector::from_column_slice(y0);
    let fit = Ols::fit(&design, &yv);
    let ssr: f64 = fit.resid.iter().map(|r| r * r).sum();
    let ymean = y0.iter().sum::<f64>() / nobs as f64;
    let tss: f64 = y0.iter().map(|v| (v - ymean) * (v - ymean)).sum();
    let rsquared = 1.0 - ssr / tss;
    if rsquared > 1.0 - 100.0 * f64::EPSILON.sqrt() {
        return CointResult {
            stat: f64::NEG_INFINITY,
            pvalue: 0.0,
            crit: [f64::NAN; 3],
        };
    }
    let resid: Vec<f64> = fit.resid.iter().copied().collect();
    let adf = adfuller(&resid, Regression::N);
    CointResult {
        stat: adf.stat,
        pvalue: mackinnonp_coint2(adf.stat),
        crit: mackinnoncrit_coint2(nobs - 1),
    }
}
