//! Pearl's three step counterfactual for a linear structural causal model with additive
//! Gaussian noise, in closed form.
//!
//! The noise to observation map is affine, so abduction is a linear solve. Two variants:
//! exact inversion, and the posterior under the notebook's small observation noise.

use nalgebra::{DMatrix, DVector};

/// One structural equation: `value = intercept + sum(coef * parent) + noise_scale * noise`.
pub struct Equation {
    pub intercept: f64,
    /// Coefficients on earlier nodes, indexed by node position.
    pub parents: Vec<(usize, f64)>,
    pub noise_scale: f64,
}

/// A linear SCM whose nodes are listed in topological order.
pub struct LinearScm {
    pub equations: Vec<Equation>,
}

impl LinearScm {
    /// The affine map from exogenous noise to node values: values = A n + b.
    pub fn affine_map(&self) -> (DMatrix<f64>, DVector<f64>) {
        let k = self.equations.len();
        let mut a = DMatrix::<f64>::zeros(k, k);
        let mut b = DVector::<f64>::zeros(k);
        for (i, eq) in self.equations.iter().enumerate() {
            b[i] = eq.intercept;
            for &(p, coef) in &eq.parents {
                b[i] += coef * b[p];
                for c in 0..k {
                    a[(i, c)] += coef * a[(p, c)];
                }
            }
            a[(i, i)] += eq.noise_scale;
        }
        (a, b)
    }

    /// Forward evaluation of the model from a noise vector, honouring interventions.
    /// An intervened node takes its fixed value and ignores its equation.
    pub fn evaluate(&self, noise: &[f64], interventions: &[(usize, f64)]) -> Vec<f64> {
        let mut values = vec![0.0; self.equations.len()];
        for (i, eq) in self.equations.iter().enumerate() {
            if let Some(&(_, fixed)) = interventions.iter().find(|(node, _)| *node == i) {
                values[i] = fixed;
                continue;
            }
            let mut v = eq.intercept + eq.noise_scale * noise[i];
            for &(p, coef) in &eq.parents {
                v += coef * values[p];
            }
            values[i] = v;
        }
        values
    }

    /// Abduction with deterministic structural equations: the noise is point identified,
    /// recovered by solving A n = y - b.
    pub fn abduct_exact(&self, observed: &[f64]) -> Vec<f64> {
        let (a, b) = self.affine_map();
        let y = DVector::from_column_slice(observed);
        let rhs = y - b;
        let rhs = DMatrix::from_column_slice(rhs.len(), 1, rhs.as_slice());
        let solved =
            crate::linalg::solve(&a, &rhs).expect("the affine map is triangular and invertible");
        solved.iter().copied().collect()
    }

    /// Abduction when the observed nodes carry Gaussian noise of the given scale, as the
    /// notebook's pseudo delta does. With a standard normal prior on the noise the posterior
    /// is Gaussian with precision I + A'A / s^2, and this returns its mean.
    pub fn abduct_with_observation_noise(&self, observed: &[f64], scale: f64) -> Vec<f64> {
        let (a, b) = self.affine_map();
        let k = self.equations.len();
        let s2 = scale * scale;
        let precision = DMatrix::identity(k, k) + a.transpose() * &a / s2;
        let y = DVector::from_column_slice(observed);
        let rhs = a.transpose() * (y - b) / s2;
        let rhs = DMatrix::from_column_slice(rhs.len(), 1, rhs.as_slice());
        let mean = crate::linalg::solve(&precision, &rhs)
            .expect("the posterior precision is positive definite");
        mean.iter().copied().collect()
    }

    /// The full three steps: abduct from the observation, act, and predict.
    pub fn counterfactual(
        &self,
        observed: &[f64],
        interventions: &[(usize, f64)],
        observation_noise: Option<f64>,
    ) -> Vec<f64> {
        let noise = match observation_noise {
            Some(scale) => self.abduct_with_observation_noise(observed, scale),
            None => self.abduct_exact(observed),
        };
        self.evaluate(&noise, interventions)
    }
}
