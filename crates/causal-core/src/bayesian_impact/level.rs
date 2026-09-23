//! Proper-prior scalar local level. Existing UCM smoothing is diffuse and two-state.
#[derive(Debug, PartialEq)]
pub enum Error {
    Empty,
    NonFinite,
    InvalidVariance,
    Numerical,
}
pub struct Model {
    observation: f64,
    level: f64,
    initial_mean: f64,
    initial_variance: f64,
}
pub struct Marginals {
    pub means: Vec<f64>,
    pub variances: Vec<f64>,
}

impl Model {
    /// Durbin–Koopman conditional draw, using the existing NumPy-compatible RNG.
    /// Random streams differ from TensorFlow; the conditional distribution is identical.
    pub fn sample(&self, values: &[Option<f64>], rng: &mut crate::nprandom::NpRng) -> Result<Vec<f64>, Error> {
        if values.is_empty() { return Err(Error::Empty); }
        let mut level = self.initial_mean + self.initial_variance.sqrt() * rng.standard_normal();
        let mut generated = Vec::with_capacity(values.len());
        let mut differences = Vec::with_capacity(values.len());
        for value in values {
            generated.push(level);
            let observation = level + self.observation.sqrt() * rng.standard_normal();
            differences.push(value.map(|v| v - observation));
            level += self.level.sqrt() * rng.standard_normal();
        }
        let zero_mean = Self { initial_mean: 0.0, ..*self };
        let residual = zero_mean.smooth(&differences)?;
        Ok(generated.iter().zip(residual.means).map(|(a,b)| a+b).collect())
    }
    pub fn new(
        observation: f64,
        level: f64,
        initial_mean: f64,
        initial_variance: f64,
    ) -> Result<Self, Error> {
        if !initial_mean.is_finite() {
            return Err(Error::NonFinite);
        }
        if [observation, level, initial_variance]
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.0)
        {
            return Err(Error::InvalidVariance);
        }
        Ok(Self {
            observation,
            level,
            initial_mean,
            initial_variance,
        })
    }

    pub fn smooth(&self, values: &[Option<f64>]) -> Result<Marginals, Error> {
        if values.is_empty() {
            return Err(Error::Empty);
        }
        if values.iter().flatten().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut means = Vec::with_capacity(values.len());
        let mut variances = Vec::with_capacity(values.len());
        let mut mean = self.initial_mean;
        let mut variance = self.initial_variance;
        for value in values {
            if let Some(value) = value {
                let gain = variance / (variance + self.observation);
                mean += gain * (value - mean);
                variance *= 1.0 - gain;
            }
            means.push(mean);
            variances.push(variance);
            variance += self.level;
        }
        for t in (0..values.len() - 1).rev() {
            let predicted_variance = variances[t] + self.level;
            let gain = variances[t] / predicted_variance;
            means[t] += gain * (means[t + 1] - means[t]);
            variances[t] += gain * gain * (variances[t + 1] - predicted_variance);
        }
        if means.iter().any(|v| !v.is_finite())
            || variances.iter().any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(Error::Numerical);
        }
        Ok(Marginals { means, variances })
    }
}
