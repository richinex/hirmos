//! Shapiro–Wilk normality diagnostic, ported from SciPy's AS R94 implementation.

fn poly(c: &[f64], x: f64) -> f64 {
    let nord = c.len();
    let res = c[0];
    if nord == 1 {
        return res;
    }
    let mut p = x * c[nord - 1];
    if nord == 2 {
        return res + p;
    }
    for ind in (1..nord - 1).rev() {
        p = (p + c[ind]) * x;
    }
    res + p
}

fn ppnd(p: f64) -> f64 {
    const A: [f64; 4] = [
        2.50662823884,
        -18.61500062529,
        41.39119773534,
        -25.44106049637,
    ];
    const B: [f64; 4] = [
        -8.47351093090,
        23.08336743743,
        -21.06224101826,
        3.13082909833,
    ];
    const C: [f64; 4] = [-2.78718931138, -2.29796479134, 4.85014127135, 2.32121276858];
    const D: [f64; 2] = [3.54388924762, 1.63706781897];
    const SPLIT: f64 = 0.42;
    let q = p - 0.5;
    if q.abs() <= SPLIT {
        let r = q * q;
        let temp = q * (((A[3] * r + A[2]) * r + A[1]) * r + A[0]);
        return temp / ((((B[3] * r + B[2]) * r + B[1]) * r + B[0]) * r + 1.0);
    }
    let mut r = p;
    if q > 0.0 {
        r = 1.0 - p;
    }
    if r > 0.0 {
        r = (-r.ln()).sqrt();
    } else {
        return 0.0;
    }
    let temp = (((C[3] * r + C[2]) * r + C[1]) * r + C[0]) / ((D[1] * r + D[0]) * r + 1.0);
    if q < 0.0 {
        -temp
    } else {
        temp
    }
}

fn alnorm(x: f64, upper: bool) -> f64 {
    const LTONE: f64 = 7.0;
    const UTZERO: f64 = 38.0;
    const CON: f64 = 1.28;
    let mut upper = upper;
    let mut z = x;
    if !(z > 0.0) {
        upper = false;
        z = -z;
    }
    if !(z <= LTONE || (upper && z <= UTZERO)) {
        return if upper { 0.0 } else { 1.0 };
    }
    let y = 0.5 * z * z;
    let temp = if z <= CON {
        0.5 - z
            * (0.398942280444
                - 0.399903438504 * y
                    / (y + 5.75885480458
                        - 29.8213557808
                            / (y + 2.62433121679 + 48.6959930692 / (y + 5.92885724438))))
    } else {
        0.398942280385 * (-y).exp()
            / (z - 3.8052e-8
                + 1.00000615302
                    / (z + 3.98064794e-4
                        + 1.98615381364
                            / (z - 0.151679116635
                                + 5.29330324926
                                    / (z + 4.8385912808
                                        - 15.1508972451
                                            / (z + 0.742380924027
                                                + 30.789933034 / (z + 3.99019417011))))))
    };
    if upper {
        temp
    } else {
        1.0 - temp
    }
}

/// scipy.stats.shapiro: sorts, centres on the unsorted middle element, then AS R94 swilk.
pub fn shapiro(x: &[f64]) -> (f64, f64) {
    let n = x.len();
    assert!(n >= 3, "Data must be at least length 3");
    let mut y: Vec<f64> = x.to_vec();
    y.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let shift = x[n / 2];
    for v in &mut y {
        *v -= shift;
    }
    swilk(&y)
}

fn swilk(x: &[f64]) -> (f64, f64) {
    const C1: [f64; 6] = [0.0, 0.221157, -0.147981, -2.07119, 4.434685, -2.706056];
    const C2: [f64; 6] = [0.0, 0.042981, -0.293762, -1.752461, 5.682633, -3.582633];
    const C3: [f64; 4] = [0.5440, -0.39978, 0.025054, -6.714e-4];
    const C4: [f64; 4] = [1.3822, -0.77857, 0.062767, -2.0322e-3];
    const C5: [f64; 4] = [-1.5861, -0.31082, -0.083751, 3.8915e-3];
    const C6: [f64; 3] = [-0.4803, -0.082676, 3.0302e-3];
    const G: [f64; 2] = [-2.273, 0.459];
    const SMALL: f64 = 1e-19;

    let n = x.len();
    let n2 = n / 2;
    let an = n as f64;

    let mut a = vec![0.0; n2];
    if n == 3 {
        a[0] = std::f64::consts::FRAC_1_SQRT_2;
    } else {
        let an25 = an + 0.25;
        let mut summ2 = 0.0;
        for (ind1, item) in a.iter_mut().enumerate() {
            let temp = ppnd((ind1 as f64 + 1.0 - 0.375) / an25);
            *item = temp;
            summ2 += temp * temp;
        }
        summ2 *= 2.0;
        let ssumm2 = summ2.sqrt();
        let rsn = 1.0 / an.sqrt();
        let a1 = poly(&C1, rsn) - a[0] / ssumm2;
        let (i1, fac) = if n > 5 {
            let a2 = -a[1] / ssumm2 + poly(&C2, rsn);
            let fac = ((summ2 - 2.0 * a[0] * a[0] - 2.0 * a[1] * a[1])
                / (1.0 - 2.0 * a1 * a1 - 2.0 * a2 * a2))
                .sqrt();
            a[1] = a2;
            (2, fac)
        } else {
            (
                1,
                ((summ2 - 2.0 * a[0] * a[0]) / (1.0 - 2.0 * a1 * a1)).sqrt(),
            )
        };
        a[0] = a1;
        for item in a.iter_mut().take(n2).skip(i1) {
            *item *= -1.0 / fac;
        }
    }

    let range = x[n - 1] - x[0];
    if range < SMALL {
        return (1.0, 1.0);
    }

    let mut xx = x[0] / range;
    let mut sx = xx;
    let mut sa = -a[0];
    let mut ind2 = n - 2;
    for (ind1, &xi_raw) in x.iter().enumerate().take(n).skip(1) {
        let xi = xi_raw / range;
        sx += xi;
        if ind1 != ind2 {
            let sign = if ind1 < ind2 { -1.0 } else { 1.0 };
            sa += sign * a[ind1.min(ind2)];
        }
        xx = xi;
        ind2 = ind2.wrapping_sub(1);
    }
    let _ = xx;

    let sa = sa / n as f64;
    let sx = sx / n as f64;
    let (mut ssa, mut ssx, mut sax) = (0.0, 0.0, 0.0);
    let mut ind2 = n - 1;
    for (ind1, &xv) in x.iter().enumerate() {
        let asa = if ind1 != ind2 {
            let sign = if ind1 < ind2 { -1.0 } else { 1.0 };
            sign * a[ind1.min(ind2)] - sa
        } else {
            -sa
        };
        let xsx = xv / range - sx;
        ssa += asa * asa;
        ssx += xsx * xsx;
        sax += asa * xsx;
        ind2 = ind2.wrapping_sub(1);
    }

    let ssassx = (ssa * ssx).sqrt();
    let w1 = (ssassx - sax) * (ssassx + sax) / (ssa * ssx);
    let w = 1.0 - w1;

    if n == 3 {
        if w < 0.75 {
            return (0.75, 0.0);
        }
        let pi6 = 6.0 / std::f64::consts::PI;
        return (w, 1.0 - pi6 * w.sqrt().acos());
    }

    let mut y = w1.ln();
    let xx_log = an.ln();
    let (m, s);
    if n <= 11 {
        let gamma = poly(&G, an);
        if y >= gamma {
            return (w, SMALL);
        }
        y = -(gamma - y).ln();
        m = poly(&C3, an);
        s = poly(&C4, an).exp();
    } else {
        m = poly(&C5, xx_log);
        s = poly(&C6, xx_log).exp();
    }

    (w, alnorm((y - m) / s, true))
}
