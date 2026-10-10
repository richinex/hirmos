//! Gaussian smoothing of group variances on a caller-supplied common axis.

use rustfft::{num_complex::Complex, FftPlanner};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooFewGroups,
    InvalidValues,
    InvalidRange,
    NumericalRange,
}

pub struct Density {
    pub bandwidth: f64,
    pub points: Vec<[f64; 2]>,
}

/// R's `bw.nrd0`: 0.9 times the smaller of the standard deviation and the interquartile range
/// over 1.34, times n to the power of minus one fifth.
pub fn bandwidth(variances: &[f64]) -> Result<f64, Error> {
    if variances.len() < 2 {
        return Err(Error::TooFewGroups);
    }
    if variances.iter().any(|v| !v.is_finite() || *v < 0.0) {
        return Err(Error::InvalidValues);
    }
    let mut sorted = variances.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len() as f64;
    let mean = sorted.iter().map(|v| v - sorted[0]).sum::<f64>() / n;
    let sd = (sorted
        .iter()
        .map(|v| (v - sorted[0] - mean).powi(2))
        .sum::<f64>()
        / (n - 1.0))
        .sqrt();
    let quantile = |p: f64| {
        let index = (n - 1.0) * p;
        let low = index.floor() as usize;
        sorted[low] + (index - low as f64) * (sorted[index.ceil() as usize] - sorted[low])
    };
    let spread = sd.min((quantile(0.75) - quantile(0.25)) / 1.34);
    let scale = if spread > 0.0 {
        spread
    } else if sd > 0.0 {
        sd
    } else if sorted[0] > 0.0 {
        sorted[0]
    } else {
        1.0
    };
    let bandwidth = 0.9 * scale * n.powf(-0.2);
    if bandwidth.is_finite() && bandwidth > 0.0 {
        Ok(bandwidth)
    } else {
        Err(Error::NumericalRange)
    }
}

pub fn gaussian(variances: &[f64], from: f64, to: f64) -> Result<Density, Error> {
    let bandwidth = bandwidth(variances)?;
    if !from.is_finite() || !to.is_finite() || from >= to {
        return Err(Error::InvalidRange);
    }
    let mut sorted = variances.to_vec();
    sorted.sort_by(f64::total_cmp);
    if sorted[0] < from || *sorted.last().unwrap() > to {
        return Err(Error::InvalidRange);
    }
    let n = sorted.len() as f64;
    let low = from - 4.0 * bandwidth;
    let high = to + 4.0 * bandwidth;
    let step = (high - low) / 511.0;
    if !step.is_finite() || step <= 0.0 {
        return Err(Error::NumericalRange);
    }
    let mut mass = vec![Complex::new(0.0, 0.0); 1024];
    for value in sorted {
        let position = (value - low) / step;
        let bin = position.floor() as usize;
        if bin >= 511 {
            return Err(Error::NumericalRange);
        }
        let fraction = position - bin as f64;
        mass[bin].re += (1.0 - fraction) / n;
        mass[bin + 1].re += fraction / n;
    }
    let normalizer = bandwidth * (2.0 * std::f64::consts::PI).sqrt();
    let mut kernel = (0..1024)
        .map(|i| {
            let distance = if i <= 512 {
                i as f64 * step
            } else {
                -((1024 - i) as f64) * step
            };
            Complex::new(
                (-0.5 * (distance / bandwidth).powi(2)).exp() / normalizer,
                0.0,
            )
        })
        .collect::<Vec<_>>();
    let mut planner = FftPlanner::<f64>::new();
    let forward = planner.plan_fft_forward(1024);
    forward.process(&mut mass);
    forward.process(&mut kernel);
    for (m, k) in mass.iter_mut().zip(kernel) {
        *m *= k.conj();
    }
    planner.plan_fft_inverse(1024).process(&mut mass);
    let ordinates = mass[..512]
        .iter()
        .map(|v| (v.re / 1024.0).max(0.0))
        .collect::<Vec<_>>();
    let mut points = Vec::with_capacity(512);
    for i in 0..512 {
        let x = from + i as f64 * (to - from) / 511.0;
        let position = (x - low) / step;
        let bin = (position.floor() as usize).min(510);
        let fraction = position - bin as f64;
        let y = ordinates[bin] + fraction * (ordinates[bin + 1] - ordinates[bin]);
        if !x.is_finite() || !y.is_finite() {
            return Err(Error::NumericalRange);
        }
        points.push([x, y]);
    }
    Ok(Density { bandwidth, points })
}
