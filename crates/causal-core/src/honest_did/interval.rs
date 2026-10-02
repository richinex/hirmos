use crate::honest_did::{flci::type_seven_quantile, r_rng::RRng, AffineProblem, BiasOptimum, Error};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SearchMethod {
    DerivativeBisection,
    Grid,
}

#[derive(Clone, Debug)]
pub struct FixedLengthInterval {
    pub lower: f64,
    pub upper: f64,
    pub coefficients: Vec<f64>,
    pub half_length: f64,
    pub magnitude: f64,
    pub search: SearchMethod,
    pub accuracy: crate::honest_did::optimization::Accuracy,
}

struct FoldedDraws(Vec<f64>);
impl FoldedDraws {
    fn new(seed: u32) -> Self {
        let mut rng = RRng::new(seed);
        Self((0..1_000_000).map(|_| rng.normal()).collect())
    }
    fn quantile(&self, probability: f64, mean: f64) -> f64 {
        let mut sample: Vec<f64> = self.0.iter().map(|z| (z + mean).abs()).collect();
        type_seven_quantile(&mut sample, probability)
    }
}

/// HonestDiD::findOptimalFLCI, including the authors' derivative-bisection
/// search and their grid-search branch. A failed robust calculation is an Err.
pub fn fixed_length_interval(
    problem: &AffineProblem,
    beta: &[f64],
    magnitude: f64,
    alpha: f64,
    points: usize,
    seed: u32,
) -> Result<FixedLengthInterval, Error> {
    let expected = problem.period_count();
    if beta.len() != expected
        || beta.iter().any(|v| !v.is_finite())
        || !magnitude.is_finite()
        || magnitude < 0.0
        || !alpha.is_finite()
        || alpha <= 0.0
        || alpha >= 1.0
        || points < 2
    {
        return Err(Error::InvalidInput("The interval requires matching finite coefficients, a nonnegative bound, a confidence level between zero and one and at least two search points."));
    }
    let h_min = problem.minimum_sd()?;
    let h_bias = problem.minimum_bias_sd()?;
    let draws = FoldedDraws::new(seed);
    let objective = |h: f64| -> Result<f64, Error> {
        let optimum = problem.minimum_bias_at_sd(h, 1.0)?;
        Ok(draws.quantile(1.0 - alpha, magnitude * optimum.bias / h) * h)
    };
    let h_star = derivative_bisection(h_min, h_bias, points, &objective);
    // In HonestDiD 0.2.8 the final quantile call (and the grid branch) omit
    // `seed`, hence use seed 0 even when the derivative search uses another seed.
    // Preserve the oracle's behavior rather than silently correcting upstream.
    let final_draws = if seed == 0 {
        draws
    } else {
        FoldedDraws::new(0)
    };
    let (optimum, half_length, search) = if let Some(h) = h_star {
        let optimum = problem.minimum_bias_at_sd(h, 1.0)?;
        let half = final_draws.quantile(1.0 - alpha, magnitude * optimum.bias / h) * h;
        (optimum, half, SearchMethod::DerivativeBisection)
    } else {
        let mut best: Option<(BiasOptimum, f64)> = None;
        let mut last_error = None;
        for i in 0..points {
            let h = h_min + (h_bias - h_min) * i as f64 / (points - 1) as f64;
            match problem.minimum_bias_at_sd(h, 1.0) {
                Ok(optimum) => {
                    let half = final_draws.quantile(1.0 - alpha, magnitude * optimum.bias / h) * h;
                    if half.is_finite() && best.as_ref().is_none_or(|(_, old)| half < *old) {
                        best = Some((optimum, half));
                    }
                }
                Err(error) => last_error = Some(error),
            }
        }
        let (optimum, half) = best.ok_or_else(|| {
            last_error.unwrap_or(Error::InvalidInput(
                "No finite fixed-length interval was obtained.",
            ))
        })?;
        (optimum, half, SearchMethod::Grid)
    };
    let accuracy = optimum.accuracy;
    let mut coefficients = optimum.coefficients;
    coefficients.extend_from_slice(problem.contrast());
    let center: f64 = coefficients.iter().zip(beta).map(|(l, b)| l * b).sum();
    if !half_length.is_finite() || half_length < 0.0 || !center.is_finite() {
        return Err(Error::InvalidInput("The fitted interval is not finite."));
    }
    Ok(FixedLengthInterval {
        lower: center - half_length,
        upper: center + half_length,
        coefficients,
        half_length,
        magnitude,
        search,
        accuracy,
    })
}

fn derivative_bisection<F>(mut a: f64, mut b: f64, points: usize, objective: &F) -> Option<f64>
where
    F: Fn(f64) -> Result<f64, Error>,
{
    let dif = ((b - a) / points as f64).min(b.abs() * f64::EPSILON.powf(1.0 / 3.0));
    if dif <= 0.0 || !dif.is_finite() {
        return None;
    }
    let fa = objective(a).ok()?;
    let fb = objective(b).ok()?;
    let fpa = (objective(a + dif).ok()? - fa) / dif;
    let fpb = (objective(b - dif).ok()? - fb) / -dif;
    if ![fa, fb, fpa, fpb].iter().all(|v| v.is_finite()) || fpa > fpb {
        return None;
    }
    if fpb < 0.0 {
        return Some(b);
    }
    if fpa > 0.0 {
        return Some(a);
    }
    let mut iteration = 1;
    let max_iterations = 10 * (((b - a).abs() / dif).log2().ceil() as usize);
    while (b - a).abs() > dif {
        iteration += 1;
        let x = (a + b) / 2.0;
        let slope = (objective(x + dif).ok()? - objective(x - dif).ok()?) / (2.0 * dif);
        if !slope.is_finite()
            || slope > fpb + f64::EPSILON.sqrt()
            || slope + f64::EPSILON.sqrt() < fpa
            || iteration > max_iterations
        {
            return None;
        }
        if slope > 0.0 {
            b = x;
        } else {
            a = x;
        }
    }
    Some((a + b) / 2.0)
}
