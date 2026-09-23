//! BOOM 0.9.16 SpikeSlabDaRegressionSampler, Gaussian ODA path.
//! Copyright Google LLC and Steven L. Scott; LGPL-2.1-or-later.
use super::*;

#[derive(Clone, Copy)]
pub struct OdaOptions {
    fudge: f64,
    fallback: f64,
}
impl OdaOptions {
    pub fn new(fudge: f64, fallback: f64) -> Result<Self, Error> {
        if !fudge.is_finite() || fudge < 0.0 {
            return Err(Error::InvalidScale);
        }
        if !fallback.is_finite() || !(0.0..=1.0).contains(&fallback) {
            return Err(Error::InvalidProbability);
        }
        Ok(Self { fudge, fallback })
    }
}
impl Default for OdaOptions {
    fn default() -> Self {
        Self {
            fudge: 0.001,
            fallback: 0.0,
        }
    }
}
#[derive(Clone)]
pub(super) struct Augmentation {
    missing: DMatrix<f64>,
    diagonal: DVector<f64>,
    center: DVector<f64>,
    options: OdaOptions,
}
impl Augmentation {
    fn new(x: &DMatrix<f64>, options: OdaOptions) -> Result<Self, Error> {
        let p = x.ncols();
        let n = x.nrows() as f64;
        let center = DVector::from_fn(p, |j, _| x.column(j).sum() / n);
        let xtx = x.transpose() * x;
        let mut centered = &xtx - &center * center.transpose() * n;
        let scale = centered.diagonal().map(f64::sqrt);
        if scale.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let epsilon = f64::EPSILON.sqrt();
        let intercept = (n - xtx[(0, 0)]).abs() < epsilon && centered[(0, 0)].abs() < epsilon;
        for i in usize::from(intercept)..p {
            for j in usize::from(intercept)..p {
                let product = scale[i] * scale[j];
                centered[(i, j)] /= if product == 0.0 { 1.0 } else { product };
            }
        }
        let (values, _) = crate::eigen::symmetric(&centered)?;
        let mut diagonal = DVector::repeat(p, values[p - 1] * (1.0 + options.fudge));
        if intercept {
            diagonal[0] = 0.0;
        }
        let mut missing_xtx = -centered;
        for i in 0..p {
            missing_xtx[(i, i)] += diagonal[i];
        }
        missing_xtx.apply(|v| {
            if v.abs() < epsilon {
                *v = 0.0;
            }
        });
        let (values, vectors) = crate::eigen::symmetric(&missing_xtx)?;
        if values.iter().any(|v| *v < 0.0) {
            return Err(Error::Singular);
        }
        let mut missing = DMatrix::from_diagonal(&values.map(f64::sqrt)) * vectors.transpose();
        if intercept {
            missing.column_mut(0).fill(0.0);
        }
        for j in 0..p {
            missing.column_mut(j).scale_mut(scale[j]);
            diagonal[j] *= scale[j].powi(2);
        }
        if missing
            .iter()
            .chain(diagonal.iter())
            .any(|x| !x.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(Self {
            missing,
            diagonal,
            center,
            options,
        })
    }
}
impl Regression {
    fn complete_inclusion_probability(
        &self,
        augmentation: &Augmentation,
        xty: &DVector<f64>,
        j: usize,
    ) -> Result<f64, Error> {
        let information = self.prior.precision[(j, j)];
        let posterior = augmentation.diagonal[j] + information;
        let mean = (xty[j] + information * self.prior.mean[j]) / posterior;
        let sse = mean.powi(2) * augmentation.diagonal[j] - 2.0 * mean * xty[j];
        let ssb = (mean - self.prior.mean[j]).powi(2) * information;
        let pi = self.prior.inclusion[j].0;
        let log_in =
            pi.ln() + 0.5 * (information.ln() - posterior.ln() - (sse + ssb) / self.variance);
        let log_out = (-pi).ln_1p();
        let max = log_in.max(log_out);
        let a = (log_in - max).exp();
        let b = (log_out - max).exp();
        let probability = a / (a + b);
        if !probability.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(probability)
    }
    /// ODA requires an independent conditional Gaussian slab, not a dense slab.
    pub fn with_oda(mut self, options: OdaOptions) -> Result<Self, Error> {
        let p = self.prior.mean.len();
        if (0..p).any(|i| (0..p).any(|j| i != j && self.prior.precision[(i, j)] != 0.0)) {
            return Err(Error::InvalidScale);
        }
        self.algorithm = Algorithm::Oda(Augmentation::new(&self.x, options)?);
        Ok(self)
    }
    pub(super) fn step_oda(&mut self, augmentation: &Augmentation) -> Result<Draw, Error> {
        if augmentation.options.fallback > 0.0
            && self.rng.next_f64() < augmentation.options.fallback
        {
            return self.step_ssvs();
        }
        let mut xty = self.x.transpose() * &self.y - &augmentation.center * self.y.sum();
        let mut missing_y = &augmentation.missing * &self.coefficients;
        for i in 0..missing_y.len() {
            missing_y[i] += self.variance.sqrt() * self.rng.standard_normal();
        }
        xty += augmentation.missing.transpose() * missing_y;
        for j in 1..self.included.len() {
            let probability = self.complete_inclusion_probability(augmentation, &xty, j)?;
            self.included[j] = self.rng.next_f64() < probability;
        }
        let pi = self.prior.inclusion[0].0;
        if pi == 1.0 {
            self.included[0] = true;
        } else if pi == 0.0 {
            self.included[0] = false;
        } else {
            let current = self.conditional(&self.included)?.log_weight;
            self.included[0] = !self.included[0];
            let proposed = self.conditional(&self.included)?.log_weight;
            if self.rng.next_f64().ln() > proposed - current {
                self.included[0] = !self.included[0];
            }
        }
        let conditional = self.conditional(&self.included)?;
        let k = conditional.indices.len();
        self.coefficients.fill(0.0);
        if k > 0 {
            let beta = draw_coefficients(&conditional.mean, &conditional.precision,
                self.variance, || self.rng.standard_normal())?;
            for (i, j) in conditional.indices.iter().enumerate() {
                self.coefficients[*j] = beta[i];
            }
        }
        // The reference ODA branch conditions on drawn beta and uses observed
        // residual SSE here, unlike the collapsed SSVS variance update.
        let residual = &self.y - &self.x * &self.coefficients;
        self.variance = self
            .prior
            .residual
            .conditional(Statistics::from_summary(
                self.y.len(),
                residual.norm_squared(),
            )?)?
            .draw(&mut self.rng)?
            .value();
        Ok(Draw {
            coefficients: self.coefficients.as_slice().to_vec(),
            included: self.included.clone(),
            variance: self.variance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    fn model(case: &Value) -> Regression {
        let design = case["design"].as_u64().unwrap();
        let p = if design == 3 { 1 } else { 3 };
        let x = DMatrix::from_fn(40, p, |i, j| match j {
            0 if design == 1 => (i as f64 * 0.17).sin(),
            0 => 1.,
            1 => (i as f64 * 0.3).sin(),
            2 if design == 2 => 2.0 * (i as f64 * 0.3).sin(),
            _ => (i as f64 * 0.71).cos(),
        });
        let y = DVector::from_fn(40, |i, _| {
            0.5 + 0.4 * (i as f64 * 0.3).sin() + 0.8 * (i as f64 * 1.7).sin()
        });
        let restricted = case["restricted"].as_bool().unwrap();
        let pi = if restricted {
            [1., 0.4, 0.]
        } else {
            [0.6, 0.4, 0.7]
        };
        let prior = Slab::new(
            DVector::from_vec(vec![0.2, -0.1, 0.3][..p].to_vec()),
            DMatrix::from_diagonal(&DVector::from_vec(vec![0.5, 1. / 3., 0.25][..p].to_vec())),
            pi.into_iter()
                .take(p)
                .map(|p| Inclusion::new(p).unwrap())
                .collect(),
            Scale::new(0.5).unwrap(),
            10.,
        )
        .unwrap();
        let prior = if case["bounded"].as_bool().unwrap() {
            prior.with_residual_ceiling(Scale::new(0.5).unwrap())
        } else {
            prior
        };
        Regression::new(x, y, prior, 20260924)
            .unwrap()
            .with_oda(OdaOptions::new(0.001, case["fallback"].as_f64().unwrap()).unwrap())
            .unwrap()
    }
    #[test]
    fn oda_boundaries_and_failed_sweeps_preserve_state() {
        for (fudge, fallback) in [
            (-0.1, 0.),
            (f64::NAN, 0.),
            (0.001, -0.1),
            (0.001, 1.1),
            (0.001, f64::NAN),
        ] {
            assert!(OdaOptions::new(fudge, fallback).is_err());
        }
        let fixture: Value = serde_json::from_str(include_str!("../../fixtures/oda.json")).unwrap();
        let mut chain = model(&fixture["cases"][0]);
        let original = chain.clone();
        chain.prior.precision[(0, 1)] = 0.1;
        chain.prior.precision[(1, 0)] = 0.1;
        assert!(chain.with_oda(OdaOptions::default()).is_err());
        let mut chain = original.clone();
        chain.set_response(DVector::repeat(40, 1e308)).unwrap();
        assert!(chain.step().is_err());
        assert_eq!(chain.coefficients, original.coefficients);
        assert_eq!(chain.included, original.included);
        assert_eq!(chain.variance, original.variance);
        // Restore valid observations and compare to an untouched chain. This
        // also proves both RNG streams rolled back after the failed sweep.
        chain.set_response(original.y.clone()).unwrap();
        let mut reference = original;
        for _ in 0..10 {
            let actual = chain.step().unwrap();
            let expected = reference.step().unwrap();
            assert_eq!(actual.coefficients, expected.coefficients);
            assert_eq!(actual.included, expected.included);
            assert_eq!(actual.variance, expected.variance);
        }
        let switched = chain.with_sampling(Sampling::Ssvs(FlipSweep::All)).unwrap();
        assert!(matches!(switched.algorithm, Algorithm::Ssvs));
    }
    #[test]
    fn augmentation_and_inclusion_match_compiled_boom() {
        let fixture: Value = serde_json::from_str(include_str!("../../fixtures/oda.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let model = model(case);
            let Algorithm::Oda(ref a) = model.algorithm else {
                panic!()
            };
            let gram = a.missing.transpose() * &a.missing;
            let p = model.x.ncols();
            let xty = DVector::from_fn(p, |i, _| case["conditional"][i][1].as_f64().unwrap());
            for i in 0..p {
                assert!(
                    (a.diagonal[i] - case["conditional"][i][0].as_f64().unwrap()).abs() < 1e-10
                );
                assert!(
                    (model.complete_inclusion_probability(a, &xty, i).unwrap()
                        - case["conditional"][i][2].as_f64().unwrap())
                    .abs()
                        < 1e-12
                );
                for j in 0..p {
                    assert!((gram[(i, j)] - case["gram"][i][j].as_f64().unwrap()).abs() < 1e-10);
                }
            }
        }
    }
    #[test]
    fn oda_draws_match_compiled_boom_with_fallback_and_bounds() {
        let fixture: Value = serde_json::from_str(include_str!("../../fixtures/oda.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let mut model = model(case);
            let p = model.x.ncols();
            let mut samples = vec![Vec::new(); 1 + 2 * p];
            for i in 0..60000 {
                let draw = model.step().unwrap();
                if case["bounded"].as_bool().unwrap() {
                    assert!(draw.variance <= 0.25);
                }
                if case["restricted"].as_bool().unwrap() {
                    assert!(draw.included[0]);
                    if p > 1 {
                        assert!(!draw.included[2]);
                    }
                }
                if i < 10000 {
                    continue;
                }
                samples[0].push(draw.variance);
                for j in 0..p {
                    samples[1 + j].push(draw.coefficients[j]);
                    samples[1 + p + j].push(if draw.included[j] { 1. } else { 0. });
                }
            }
            for (j, x) in samples.iter().enumerate() {
                let mean = x.iter().sum::<f64>() / x.len() as f64;
                let batches: Vec<_> = x
                    .chunks_exact(100)
                    .map(|b| b.iter().sum::<f64>() / 100.)
                    .collect();
                let mcse = (batches.iter().map(|b| (b - mean).powi(2)).sum::<f64>()
                    / ((batches.len() - 1) * batches.len()) as f64)
                    .sqrt();
                let expected = case["metrics"][j]["mean"].as_f64().unwrap();
                let se = mcse.hypot(case["metrics"][j]["mcse"].as_f64().unwrap());
                assert!((mean-expected).abs()<=6.*se+1e-12,"fallback {} restricted {} bounded {} metric {j}: {mean} != {expected}, SE {se}",case["fallback"],case["restricted"],case["bounded"]);
            }
        }
    }
}
