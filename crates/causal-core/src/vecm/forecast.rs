//! statsmodels VECMResults.predict: VAR representation and innovation forecast covariance.

use super::VecmResult;
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq, Eq)]
pub enum ForecastError {
    EmptyHorizon,
    InvalidConfidence,
    UnsupportedDeterministic,
    NonFinite,
}

pub struct Forecast {
    pub mean: Vec<Vec<f64>>,
    pub covariance: Vec<DMatrix<f64>>,
    pub lower: Vec<Vec<f64>>,
    pub upper: Vec<Vec<f64>>,
}

impl VecmResult {
    pub fn forecast(&self, steps: usize, confidence: f64) -> Result<Forecast, ForecastError> {
        if steps == 0 {
            return Err(ForecastError::EmptyHorizon);
        }
        if !(0.0 < confidence && confidence < 1.0) {
            return Err(ForecastError::InvalidConfidence);
        }
        if !matches!(
            self.deterministic.as_str(),
            "n" | "ci" | "co" | "li" | "lo" | "cili" | "cilo" | "coli" | "colo"
        ) {
            return Err(ForecastError::UnsupportedDeterministic);
        }
        let k = self.alpha.nrows();
        let p = self.forecast_history.len();
        let mut ar = vec![DMatrix::zeros(k, k); p];
        ar[0] = &self.alpha * self.beta.transpose() + DMatrix::identity(k, k);
        if p > 1 {
            ar[0] += self.gamma.columns(0, k);
            ar[p - 1] = -self.gamma.columns(k * (p - 2), k);
            for lag in 1..p - 1 {
                ar[lag] = self.gamma.columns(k * lag, k) - self.gamma.columns(k * (lag - 1), k);
            }
        }
        let mut history: Vec<DVector<f64>> = self
            .forecast_history
            .iter()
            .map(|r| DVector::from_column_slice(r))
            .collect();
        let mut responses = vec![DMatrix::identity(k, k)];
        let mut covariance = DMatrix::zeros(k, k);
        let mut result = Forecast {
            mean: Vec::with_capacity(steps),
            covariance: Vec::with_capacity(steps),
            lower: Vec::with_capacity(steps),
            upper: Vec::with_capacity(steps),
        };
        let critical = spec_math::cephes64::ndtri(0.5 + confidence / 2.0);
        for step in 0..steps {
            let mut mean = DVector::zeros(k);
            for lag in 0..p {
                mean += &ar[lag] * &history[history.len() - lag - 1];
            }
            let mut outside = k * (p - 1);
            if self.deterministic.contains("co") {
                mean += self.gamma.column(outside);
                outside += 1;
            }
            if self.deterministic.contains("lo") {
                mean += self.gamma.column(outside) * (self.sample_length + step + 1) as f64;
            }
            let mut inside = 0;
            if self.deterministic.contains("ci") {
                mean += &self.alpha * self.det_coef_coint.row(inside).transpose();
                inside += 1;
            }
            if self.deterministic.contains("li") {
                mean += &self.alpha
                    * self.det_coef_coint.row(inside).transpose()
                    * (self.sample_length + step) as f64;
            }
            if step > 0 {
                let mut response = DMatrix::zeros(k, k);
                for lag in 1..=p.min(step) {
                    response += &ar[lag - 1] * &responses[step - lag];
                }
                responses.push(response);
            }
            covariance += &responses[step] * &self.sigma_u * responses[step].transpose();
            let margins =
                DVector::from_iterator(k, (0..k).map(|i| critical * covariance[(i, i)].sqrt()));
            if !mean
                .iter()
                .chain(covariance.iter())
                .chain(margins.iter())
                .all(|v| v.is_finite())
            {
                return Err(ForecastError::NonFinite);
            }
            result.mean.push(mean.iter().copied().collect());
            result
                .lower
                .push((&mean - &margins).iter().copied().collect());
            result
                .upper
                .push((&mean + &margins).iter().copied().collect());
            result.covariance.push(covariance.clone());
            history.push(mean);
        }
        Ok(result)
    }
}
