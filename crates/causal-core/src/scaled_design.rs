//! Ordered column selection on a scaled design, without forming cross-products.

use nalgebra::{DMatrix, DVector};

pub struct ScaledDesign {
    pub kept: Vec<usize>,
    pub scales: Vec<f64>,
    pub matrix: DMatrix<f64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DesignError {
    InvalidDimensions,
    NonFinite,
}

impl ScaledDesign {
    /// `source_scales` are maximum absolute column values before projection,
    /// with the same row weights as `projected`. Projection noise is not a regressor.
    pub fn new(projected: &DMatrix<f64>, source_scales: &[f64]) -> Result<Self, DesignError> {
        let (n, p) = projected.shape();
        if n == 0 || source_scales.len() != p {
            return Err(DesignError::InvalidDimensions);
        }
        if projected.iter().any(|v| !v.is_finite())
            || source_scales.iter().any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(DesignError::NonFinite);
        }
        let tolerance = n.max(p) as f64 * f64::EPSILON;
        let scales: Vec<f64> = (0..p)
            .map(|j| {
                projected
                    .column(j)
                    .iter()
                    .map(|v| v.abs())
                    .fold(0.0, f64::max)
            })
            .collect();
        let candidates: Vec<usize> = (0..p)
            .filter(|&j| scales[j] > 0.0 && scales[j] / source_scales[j] > tolerance)
            .collect();
        if candidates.is_empty() {
            return Ok(Self {
                kept: vec![],
                scales: vec![],
                matrix: DMatrix::zeros(n, 0),
            });
        }
        let normalized = DMatrix::from_fn(n, candidates.len(), |i, j| {
            projected[(i, candidates[j])] / scales[candidates[j]]
        });
        // Householder QR compresses the tall design without squaring its condition number.
        let r = normalized.clone().qr().r();
        let mut basis: Vec<DVector<f64>> = Vec::new();
        let mut selected = Vec::new();
        for j in 0..r.ncols() {
            let mut residual = r.column(j).into_owned();
            let norm = residual.norm();
            // Reorthogonalize so an earlier near-dependency does not contaminate later columns.
            for _ in 0..2 {
                for q in &basis {
                    residual.axpy(-q.dot(&residual), q, 1.0);
                }
            }
            let remaining = residual.norm();
            if remaining > tolerance * norm {
                basis.push(residual / remaining);
                selected.push(j);
            }
        }
        let kept: Vec<usize> = selected.iter().map(|&j| candidates[j]).collect();
        Ok(Self {
            scales: kept.iter().map(|&j| scales[j]).collect(),
            matrix: DMatrix::from_fn(n, kept.len(), |i, j| normalized[(i, selected[j])]),
            kept,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare(x: &DMatrix<f64>) -> ScaledDesign {
        let scales: Vec<f64> = (0..x.ncols())
            .map(|j| x.column(j).iter().map(|v| v.abs()).fold(0.0, f64::max))
            .collect();
        ScaledDesign::new(x, &scales).unwrap()
    }

    #[test]
    fn retains_order_and_resolves_dependencies_independently_of_units() {
        let x = DMatrix::from_fn(40, 5, |i, j| {
            let t = i as f64 / 40.0;
            match j {
                0 => t,
                1 => 2.0 * t,
                2 => 0.0,
                3 => t + 1e-7 * t * t,
                _ => t * t * t,
            }
        });
        let baseline = prepare(&x);
        assert_eq!(baseline.kept, vec![0, 3, 4]);
        let factors = [1e-100, -1e100, 1.0, -1e-50, 1e50];
        let changed = prepare(&DMatrix::from_fn(40, 5, |i, j| x[(i, j)] * factors[j]));
        assert_eq!(changed.kept, baseline.kept);
        for j in 0..baseline.kept.len() {
            let sign = factors[baseline.kept[j]].signum();
            for i in 0..40 {
                assert!((changed.matrix[(i, j)] * sign - baseline.matrix[(i, j)]).abs() < 1e-14);
            }
        }
    }

    #[test]
    fn projection_noise_and_zero_columns_are_not_predictors() {
        let x =
            DMatrix::from_column_slice(4, 2, &[1e-17, -1e-17, 1e-17, -1e-17, 0.0, 0.0, 0.0, 0.0]);
        let result = ScaledDesign::new(&x, &[0.1, 0.0]).unwrap();
        assert!(result.kept.is_empty());
        assert_eq!(result.matrix.shape(), (4, 0));
    }

    #[test]
    fn rejects_invalid_boundaries() {
        assert!(matches!(
            ScaledDesign::new(&DMatrix::zeros(2, 1), &[]),
            Err(DesignError::InvalidDimensions)
        ));
        assert!(matches!(
            ScaledDesign::new(&DMatrix::from_element(2, 1, f64::NAN), &[1.0]),
            Err(DesignError::NonFinite)
        ));
    }
}
