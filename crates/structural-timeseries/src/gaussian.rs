//! Proper-prior Gaussian filtering and smoothing for composed state components.
//! Reuses Hirmos LAPACK solves. No matrix inverse or independent solver is added.
use crate::lapack_cholesky::{dpotrf, dpotrs, Triangle};
use crate::{
    state::{System, Variance},
    Error,
};
use crate::random::NormalDraw;
use nalgebra::{DMatrix, DVector};

pub struct Filter<'a> {
    system: &'a System,
    pub errors: Vec<Option<f64>>,
    pub variances: Vec<f64>,
    pub log_likelihood: f64,
    predicted_means: Vec<DVector<f64>>,
    predicted_covariances: Vec<DMatrix<f64>>,
    filtered_means: Vec<DVector<f64>>,
    filtered_covariances: Vec<DMatrix<f64>>,
}
pub struct Smoothed {
    pub means: Vec<DVector<f64>>,
    pub covariances: Vec<DMatrix<f64>>,
}
fn symmetric(matrix: DMatrix<f64>) -> DMatrix<f64> {
    (&matrix + matrix.transpose()) * 0.5
}

#[derive(Clone, Copy)]
enum Noise<'a> {
    Constant(Variance),
    Rows(&'a [Variance]),
}
impl Noise<'_> {
    fn validate(self, rows: usize) -> Result<(), Error> {
        match self {
            Self::Rows(values) if values.len() != rows => Err(Error::Shape),
            _ => Ok(()),
        }
    }
    fn at(self, row: usize) -> Variance {
        match self {
            Self::Constant(value) => value,
            Self::Rows(values) => values[row],
        }
    }
}

pub fn filter<'a>(
    system: &'a System,
    noise: Variance,
    y: &[Option<f64>],
) -> Result<Filter<'a>, Error> {
    filter_impl(system, Noise::Constant(noise), y)
}

/// Conditional observation variances, for latent-weight observation families.
/// State transitions and both smoothing recursions are shared with filter().
pub fn filter_with_noise<'a>(
    system: &'a System,
    noise: &[Variance],
    y: &[Option<f64>],
) -> Result<Filter<'a>, Error> {
    filter_impl(system, Noise::Rows(noise), y)
}
fn filter_impl<'a>(
    system: &'a System,
    noise: Noise<'_>,
    y: &[Option<f64>],
) -> Result<Filter<'a>, Error> {
    noise.validate(y.len())?;
    if y.is_empty() {
        return Err(Error::Empty);
    }
    if y.iter().flatten().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let (mut mean, mut covariance) = system.initial();
    let mut result = Filter {
        system,
        errors: Vec::new(),
        variances: Vec::new(),
        log_likelihood: 0.0,
        predicted_means: Vec::new(),
        predicted_covariances: Vec::new(),
        filtered_means: Vec::new(),
        filtered_covariances: Vec::new(),
    };
    for (t, observed) in y.iter().enumerate() {
        let z = system.observation_at(t)?;
        result.predicted_means.push(mean.clone());
        result.predicted_covariances.push(covariance.clone());
        let cross = &covariance * &z;
        let variance = z.dot(&cross) + noise.at(t).value();
        if !variance.is_finite() || variance <= 0.0 {
            return Err(Error::Singular);
        }
        let error = observed.map(|value| value - z.dot(&mean));
        if let Some(error) = error {
            mean += &cross * (error / variance);
            covariance = symmetric(covariance - &cross * cross.transpose() / variance);
            result.log_likelihood -=
                0.5 * ((2.0 * std::f64::consts::PI * variance).ln() + error * error / variance);
            if !result.log_likelihood.is_finite() {
                return Err(Error::NonFinite);
            }
        }
        result.errors.push(error);
        result.variances.push(variance);
        result.filtered_means.push(mean.clone());
        result.filtered_covariances.push(covariance.clone());
        let (transition, innovation) = system.transition(t)?;
        mean = &transition * mean;
        covariance = symmetric(&transition * covariance * transition.transpose() + innovation);
        if mean.iter().chain(covariance.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
    }
    Ok(result)
}

/// Proper-prior information recursion, generalized from ucm::smoothed_state.
/// No inverse state covariance is needed, including for deterministic states.
pub fn smooth(filtered: &Filter<'_>) -> Result<Smoothed, Error> {
    let system = filtered.system;
    let n = filtered.predicted_means.len();
    let d = system.dimension();
    let mut r = DVector::zeros(d);
    let mut information = DMatrix::zeros(d, d);
    let mut means = filtered.predicted_means.clone();
    let mut covariances = filtered.predicted_covariances.clone();
    for t in (0..n).rev() {
        let z = system.observation_at(t)?;
        let p = &filtered.predicted_covariances[t];
        let (transition, _) = system.transition(t)?;
        match filtered.errors[t] {
            Some(error) => {
                let f = filtered.variances[t];
                let gain = &transition * p * &z / f;
                let residual_transition = transition - gain * z.transpose();
                r = &z * (error / f) + residual_transition.transpose() * r;
                information = &z * z.transpose() / f
                    + residual_transition.transpose() * information * residual_transition;
            }
            None => {
                r = transition.transpose() * r;
                information = transition.transpose() * information * transition;
            }
        }
        means[t] += p * &r;
        covariances[t] = symmetric(p - p * &information * p);
        if means[t]
            .iter()
            .chain(covariances[t].iter())
            .any(|x| !x.is_finite())
        {
            return Err(Error::NonFinite);
        }
    }
    Ok(Smoothed { means, covariances })
}

/// Independent covariance-form cross-check using the existing LAPACK solves.
/// Unlike smooth(), this path requires nonsingular predicted covariances.
pub fn smooth_rts(filtered: &Filter<'_>) -> Result<Smoothed, Error> {
    let system = filtered.system;
    let n = filtered.filtered_means.len();
    let d = system.dimension();
    let mut means = filtered.filtered_means.clone();
    let mut covariances = filtered.filtered_covariances.clone();
    for t in (0..n - 1).rev() {
        let (transition, _) = system.transition(t)?;
        let cross = &filtered.filtered_covariances[t] * transition.transpose();
        let mut factor = filtered.predicted_covariances[t + 1].as_slice().to_vec();
        if dpotrf(Triangle::Lower, d, &mut factor, d).map_err(|_| Error::Shape)? != 0 {
            return Err(Error::Singular);
        }
        let mut solved = cross.transpose();
        dpotrs(Triangle::Lower, d, d, &factor, d, solved.as_mut_slice(), d)
            .map_err(|_| Error::Shape)?;
        let gain = solved.transpose();
        let adjustment = &gain * (&means[t + 1] - &filtered.predicted_means[t + 1]);
        means[t] += adjustment;
        covariances[t] = symmetric(
            &filtered.filtered_covariances[t]
                + &gain
                    * (&covariances[t + 1] - &filtered.predicted_covariances[t + 1])
                    * gain.transpose(),
        );
    }
    Ok(Smoothed { means, covariances })
}

/// Gaussian conditional simulation by prior-draw correction. This generalizes
/// the existing UCM simulation-smoother construction to proper initial priors
/// and composed components. It does not claim BOOM random-stream equality.
pub fn draw_states(
    system: &System,
    noise: Variance,
    y: &[Option<f64>],
    rng: &mut impl NormalDraw,
) -> Result<Vec<DVector<f64>>, Error> {
    draw_states_impl(system, Noise::Constant(noise), y, rng)
}

pub fn draw_states_with_noise(
    system: &System,
    noise: &[Variance],
    y: &[Option<f64>],
    rng: &mut impl NormalDraw,
) -> Result<Vec<DVector<f64>>, Error> {
    draw_states_impl(system, Noise::Rows(noise), y, rng)
}
fn draw_states_impl(
    system: &System,
    noise: Noise<'_>,
    y: &[Option<f64>],
    rng: &mut impl NormalDraw,
) -> Result<Vec<DVector<f64>>, Error> {
    let conditional = smooth(&filter_impl(system, noise, y)?)?;
    let mut state = system.initial_distribution().draw(rng);
    let mut generated = Vec::with_capacity(y.len());
    let mut simulated_y = Vec::with_capacity(y.len());
    for (t, observed) in y.iter().enumerate() {
        let z = system.observation_at(t)?;
        let value = z.dot(&state) + noise.at(t).value().sqrt() * rng.standard_normal();
        simulated_y.push(observed.map(|_| value));
        generated.push(state.clone());
        if t + 1 < y.len() {
            let (transition, innovation) = system.transition(t)?;
            state = transition * state;
            for j in 0..state.len() {
                if innovation[(j, j)] > 0. {
                    state[j] += innovation[(j, j)].sqrt() * rng.standard_normal();
                }
            }
        }
    }
    let simulated_conditional = smooth(&filter_impl(system, noise, &simulated_y)?)?;
    Ok(generated
        .into_iter()
        .zip(conditional.means)
        .zip(simulated_conditional.means)
        .map(|((prior, actual), simulated)| prior + actual - simulated)
        .collect())
}
