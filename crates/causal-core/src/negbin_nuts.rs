//! The Bayesian Gamma-Poisson regression used by notebook 114i, evaluated in the same
//! unconstrained parameterization as Pyro: `[beta_0, beta_treatment, beta_confounder, log_r]`.

use crate::nprandom::NpRng;
use crate::nuts::{multinomial_nuts, NutsOptions, NutsResult};

const LOG_2_PI: f64 = 1.8378770664093453;

#[derive(Clone, Debug)]
pub struct PbcNegBinModel {
    pub treatment: Vec<f64>,
    pub confounder: Vec<f64>,
    pub outcome: Vec<f64>,
    /// Sample standard deviation of the unnormalised treatment, used by 114i's IRR.
    pub treatment_std: f64,
}

impl PbcNegBinModel {
    /// Apply Torch's default conceptual normalization: mean and sample standard deviation
    /// (`correction=1`). Arithmetic is f64 in this port.
    pub fn from_raw(treatment: &[f64], confounder: &[f64], outcome: &[f64]) -> Self {
        assert_eq!(treatment.len(), confounder.len());
        assert_eq!(treatment.len(), outcome.len());
        assert!(treatment.len() > 1);
        let n = treatment.len() as f64;
        let treatment_mean = treatment.iter().sum::<f64>() / n;
        let confounder_mean = confounder.iter().sum::<f64>() / n;
        let treatment_std = (treatment
            .iter()
            .map(|value| (value - treatment_mean).powi(2))
            .sum::<f64>()
            / (n - 1.0))
            .sqrt();
        let confounder_std = (confounder
            .iter()
            .map(|value| (value - confounder_mean).powi(2))
            .sum::<f64>()
            / (n - 1.0))
            .sqrt();
        assert!(treatment_std > 0.0 && confounder_std > 0.0);
        Self {
            treatment: treatment
                .iter()
                .map(|value| (value - treatment_mean) / treatment_std)
                .collect(),
            confounder: confounder
                .iter()
                .map(|value| (value - confounder_mean) / confounder_std)
                .collect(),
            outcome: outcome.to_vec(),
            treatment_std,
        }
    }

    pub fn from_normalized(
        treatment: Vec<f64>,
        confounder: Vec<f64>,
        outcome: Vec<f64>,
        treatment_std: f64,
    ) -> Self {
        assert_eq!(treatment.len(), confounder.len());
        assert_eq!(treatment.len(), outcome.len());
        assert!(treatment_std > 0.0);
        Self {
            treatment,
            confounder,
            outcome,
            treatment_std,
        }
    }

    /// Log density and gradient including the `r = exp(log_r)` transform Jacobian.
    pub fn log_prob_grad(&self, parameters: &[f64]) -> (f64, Vec<f64>) {
        assert_eq!(parameters.len(), 4);
        let (beta_0, beta_treatment, beta_confounder, log_r) =
            (parameters[0], parameters[1], parameters[2], parameters[3]);
        let r = log_r.exp();

        // Normal(3.5, 1), Normal(0, 1), Normal(0, 1), HalfNormal(10), and the
        // exponential-transform log-Jacobian for r.
        let mut log_prob = -0.5 * (beta_0 - 3.5).powi(2)
            - 0.5 * beta_treatment.powi(2)
            - 0.5 * beta_confounder.powi(2)
            - 1.5 * LOG_2_PI
            - 10.0_f64.ln()
            - 0.5 * std::f64::consts::PI.ln()
            + 0.5 * std::f64::consts::LN_2
            - r * r / 200.0
            + log_r;
        let mut gradient = vec![-(beta_0 - 3.5), -beta_treatment, -beta_confounder, 0.0];
        let mut r_gradient = -r / 100.0;

        for index in 0..self.outcome.len() {
            let y = self.outcome[index];
            let raw_log_mu = beta_0
                + beta_treatment * self.treatment[index]
                + beta_confounder * self.confounder[index];
            let active = raw_log_mu <= 10.0;
            let log_mu = raw_log_mu.min(10.0);
            let mu = log_mu.exp();
            let denominator = r + mu;
            log_prob += spec_math::cephes64::lgam(y + r)
                - spec_math::cephes64::lgam(r)
                - spec_math::cephes64::lgam(y + 1.0)
                + r * (r.ln() - denominator.ln())
                + y * (log_mu - denominator.ln());

            if active {
                let eta_gradient = y - (y + r) * mu / denominator;
                gradient[0] += eta_gradient;
                gradient[1] += eta_gradient * self.treatment[index];
                gradient[2] += eta_gradient * self.confounder[index];
            }
            r_gradient +=
                spec_math::cephes64::psi(y + r) - spec_math::cephes64::psi(r) + r.ln() + 1.0
                    - denominator.ln()
                    - (r + y) / denominator;
        }
        gradient[3] = r * r_gradient + 1.0;
        (log_prob, gradient)
    }

    pub fn sample(&self, options: NutsOptions, seed: u64) -> NutsResult {
        let mut rng = NpRng::seeded(seed);
        multinomial_nuts(
            &|parameters| self.log_prob_grad(parameters),
            &[3.5, 0.0, 0.0, 0.0],
            options,
            &mut rng,
        )
    }

    pub fn irr_draws(&self, result: &NutsResult) -> Vec<f64> {
        result
            .samples
            .iter()
            .map(|sample| (sample[1] / self.treatment_std).exp())
            .collect()
    }
}

pub fn quantile(values: &[f64], probability: f64) -> f64 {
    assert!(!values.is_empty());
    assert!((0.0..=1.0).contains(&probability));
    let mut sorted = values.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).expect("finite posterior draw"));
    let index = probability * (sorted.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    let weight = index - lower as f64;
    sorted[lower] * (1.0 - weight) + sorted[upper] * weight
}

#[derive(Clone, Debug)]
pub struct IrrSummary {
    pub median: f64,
    pub lower: f64,
    pub upper: f64,
}

pub fn irr_summary(draws: &[f64]) -> IrrSummary {
    assert!(!draws.is_empty());
    let mut sorted = draws.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).expect("finite posterior draw"));
    IrrSummary {
        // `fit_negbin_irr` calls `Tensor.median()`, which selects the lower middle
        // observation for an even number of draws rather than interpolating.
        median: sorted[(sorted.len() - 1) / 2],
        lower: quantile(draws, 0.025),
        upper: quantile(draws, 0.975),
    }
}
