//! MacKinnon 1994 p-value surfaces and 2010 critical values for the ADF tau statistic, N = 1,
//! constants transcribed from statsmodels/tsa/adfvalues.py.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Regression {
    N,
    C,
    Ct,
}

const TAU_STAR: [f64; 3] = [-1.04, -1.61, -2.89];
const TAU_MIN: [f64; 3] = [-19.04, -18.83, -16.18];
const TAU_MAX: [f64; 3] = [f64::INFINITY, 2.74, 0.7];

// Ascending polynomial coefficients; small already scaled by [1, 1, 1e-2], large by [1, 1e-1, 1e-1, 1e-2].
const TAU_NC_SMALLP: [f64; 3] = [0.6344, 1.2378, 3.2496e-2];
const TAU_C_SMALLP: [f64; 3] = [2.1659, 1.4412, 3.8269e-2];
const TAU_CT_SMALLP: [f64; 3] = [3.2512, 1.6047, 4.9588e-2];
const TAU_NC_LARGEP: [f64; 4] = [0.4797, 9.3557e-1, -6.999e-2, 3.3066e-2];
const TAU_C_LARGEP: [f64; 4] = [1.7339, 9.3202e-1, -1.2745e-1, -1.0368e-2];
const TAU_CT_LARGEP: [f64; 4] = [2.5261, 6.1654e-1, -3.7956e-1, -6.0285e-2];

// MacKinnon 2010, N = 1, rows 1% / 5% / 10%, ascending in 1/nobs.
const TAU_NC_2010: [[f64; 4]; 3] = [
    [-2.56574, -2.2358, -3.627, 0.0],
    [-1.941, -0.2686, -3.365, 31.223],
    [-1.61682, 0.2656, -2.714, 25.364],
];
const TAU_C_2010: [[f64; 4]; 3] = [
    [-3.43035, -6.5393, -16.786, -79.433],
    [-2.86154, -2.8903, -4.234, -40.040],
    [-2.56677, -1.5384, -2.809, 0.0],
];
const TAU_CT_2010: [[f64; 4]; 3] = [
    [-3.95877, -9.0531, -28.428, -134.155],
    [-3.41049, -4.3904, -9.036, -45.374],
    [-3.12705, -2.5856, -3.925, -22.380],
];

// The N = 2 'c' surfaces for the Engle-Granger cointegration statistic.
const TAU_C2_STAR: f64 = -2.62;
const TAU_C2_MIN: f64 = -18.86;
const TAU_C2_MAX: f64 = 0.92;
const TAU_C2_SMALLP: [f64; 3] = [2.92, 1.5012, 3.9796e-2];
const TAU_C2_LARGEP: [f64; 4] = [2.1945, 6.4695e-1, -2.9198e-1, -4.2377e-2];
const TAU_C2_2010: [[f64; 4]; 3] = [
    [-3.89644, -10.9519, -33.527, 0.0],
    [-3.33613, -6.1101, -6.823, 0.0],
    [-3.04445, -4.2412, -2.72, 0.0],
];

fn index(regression: Regression) -> usize {
    match regression {
        Regression::N => 0,
        Regression::C => 1,
        Regression::Ct => 2,
    }
}

fn norm_cdf(x: f64) -> f64 {
    0.5 * libm::erfc(-x / std::f64::consts::SQRT_2)
}

pub fn mackinnonp(teststat: f64, regression: Regression) -> f64 {
    let i = index(regression);
    if teststat > TAU_MAX[i] {
        return 1.0;
    }
    if teststat < TAU_MIN[i] {
        return 0.0;
    }
    let value = if teststat <= TAU_STAR[i] {
        let c = match regression {
            Regression::N => &TAU_NC_SMALLP[..],
            Regression::C => &TAU_C_SMALLP[..],
            Regression::Ct => &TAU_CT_SMALLP[..],
        };
        c.iter().rev().fold(0.0, |acc, &coef| acc * teststat + coef)
    } else {
        let c = match regression {
            Regression::N => &TAU_NC_LARGEP[..],
            Regression::C => &TAU_C_LARGEP[..],
            Regression::Ct => &TAU_CT_LARGEP[..],
        };
        c.iter().rev().fold(0.0, |acc, &coef| acc * teststat + coef)
    };
    norm_cdf(value)
}

/// MacKinnon p for the two-variable Engle-Granger statistic with a constant trend.
pub fn mackinnonp_coint2(teststat: f64) -> f64 {
    if teststat > TAU_C2_MAX {
        return 1.0;
    }
    if teststat < TAU_C2_MIN {
        return 0.0;
    }
    let c = if teststat <= TAU_C2_STAR {
        &TAU_C2_SMALLP[..]
    } else {
        &TAU_C2_LARGEP[..]
    };
    norm_cdf(c.iter().rev().fold(0.0, |acc, &coef| acc * teststat + coef))
}

/// Critical values for the two-variable Engle-Granger statistic with a constant trend.
pub fn mackinnoncrit_coint2(nobs: usize) -> [f64; 3] {
    let inv = 1.0 / nobs as f64;
    let eval = |row: &[f64; 4]| row.iter().rev().fold(0.0, |acc, &coef| acc * inv + coef);
    [
        eval(&TAU_C2_2010[0]),
        eval(&TAU_C2_2010[1]),
        eval(&TAU_C2_2010[2]),
    ]
}

/// Critical values at 1%, 5%, 10% as a polynomial in 1/nobs.
pub fn mackinnoncrit(regression: Regression, nobs: usize) -> [f64; 3] {
    let table = match regression {
        Regression::N => &TAU_NC_2010,
        Regression::C => &TAU_C_2010,
        Regression::Ct => &TAU_CT_2010,
    };
    let inv = 1.0 / nobs as f64;
    let eval = |row: &[f64; 4]| row.iter().rev().fold(0.0, |acc, &coef| acc * inv + coef);
    [eval(&table[0]), eval(&table[1]), eval(&table[2])]
}
