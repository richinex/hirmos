//! Translation of splines2 BSpline::get_basis_simple/get_derivative_simple.
//! Upstream: Wenjie Wang and Jun Yan, splines2, GPL-3.0-or-later.
//! Source commit 7a07442629e15006a239d9d6d60e2ca6f04af574.
//! Scope: interior knots (including multiplicity), finite evaluation points,
//! first derivatives, explicit boundaries. Unsupported domains are errors.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidBoundary,
    InvalidKnots,
    NonFinite,
    DimensionOverflow,
}

#[derive(Debug)]
pub struct Basis {
    degree: usize,
    interior: Vec<f64>,
    boundary: [f64; 2],
    sequence: Vec<f64>,
    columns: usize,
}

impl Basis {
    pub fn new(
        degree: usize,
        mut interior: Vec<f64>,
        mut boundary: [f64; 2],
    ) -> Result<Self, Error> {
        boundary.sort_by(f64::total_cmp);
        interior.sort_by(f64::total_cmp);
        if !boundary.iter().all(|x| x.is_finite()) || boundary[0] >= boundary[1] {
            return Err(Error::InvalidBoundary);
        }
        if interior
            .iter()
            .any(|x| !x.is_finite() || *x <= boundary[0] || *x >= boundary[1])
        {
            return Err(Error::InvalidKnots);
        }
        let order = degree.checked_add(1).ok_or(Error::DimensionOverflow)?;
        let columns = order
            .checked_add(interior.len())
            .ok_or(Error::DimensionOverflow)?;
        let length = columns.checked_add(order).ok_or(Error::DimensionOverflow)?;
        let mut sequence = Vec::new();
        sequence
            .try_reserve_exact(length)
            .map_err(|_| Error::DimensionOverflow)?;
        sequence.extend(std::iter::repeat(boundary[0]).take(order));
        sequence.extend_from_slice(&interior);
        sequence.extend(std::iter::repeat(boundary[1]).take(order));
        Ok(Self {
            degree,
            interior,
            boundary,
            sequence,
            columns,
        })
    }

    fn index(&self, x: f64) -> Result<usize, Error> {
        if !x.is_finite() {
            return Err(Error::NonFinite);
        }
        // splines2 extrapolates the edge polynomial using the same recurrence.
        Ok(self.interior.partition_point(|k| *k <= x))
    }

    fn complete(&self, x: f64) -> Result<Vec<f64>, Error> {
        let index = self.index(x)?;
        let mut row = vec![0.0; self.columns];
        row[index] = 1.0;
        for k in 1..=self.degree {
            let offset = self.degree - k;
            let mut saved = 0.0;
            for j in 0..k {
                let column = index + j;
                let left = self.sequence[column + offset + 1];
                let right = self.sequence[column + self.degree + 1];
                let term = row[column] / (right - left);
                row[column] = saved + (right - x) * term;
                saved = (x - left) * term;
            }
            row[index + k] = saved;
        }
        Ok(row)
    }

    /// splines2's intercept=FALSE omits the first basis column.
    pub fn evaluate(&self, x: f64, intercept: bool) -> Result<Vec<f64>, Error> {
        let row = self.complete(x)?;
        Ok(if intercept { row } else { row[1..].to_vec() })
    }

    pub fn derivative(&self, x: f64, intercept: bool) -> Result<Vec<f64>, Error> {
        let index = self.index(x)?;
        let mut row = if self.degree == 0 {
            vec![0.0; self.columns]
        } else {
            let lower = Self::new(self.degree - 1, self.interior.clone(), self.boundary)?;
            let mut row = lower.complete(x)?;
            row.push(0.0);
            let mut saved = 0.0;
            for j in 0..self.degree {
                let column = index + j;
                let left = self.sequence[column + 1];
                let right = self.sequence[column + self.degree + 1];
                let term = self.degree as f64 * row[column] / (right - left);
                row[column] = saved - term;
                saved = term;
            }
            row[index + self.degree] = saved;
            row
        };
        if !intercept {
            row.remove(0);
        }
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linear_values_and_derivative() {
        let b = Basis::new(1, vec![], [0.0, 1.0]).unwrap();
        assert_eq!(b.evaluate(0.25, true).unwrap(), vec![0.75, 0.25]);
        assert_eq!(b.derivative(0.25, true).unwrap(), vec![-1.0, 1.0]);
        assert_eq!(b.evaluate(1.0, false).unwrap(), vec![1.0]);
    }
    #[test]
    fn partition_and_derivative_sum() {
        for degree in 1..=3 {
            let b = Basis::new(degree, vec![0.31, 0.63], [0.05, 0.99]).unwrap();
            for x in [0.05, 0.10, 0.31, 0.48, 0.63, 0.99] {
                assert!((b.evaluate(x, true).unwrap().iter().sum::<f64>() - 1.0).abs() < 1e-14);
                assert!(b.derivative(x, true).unwrap().iter().sum::<f64>().abs() < 1e-13);
            }
        }
    }
    #[test]
    fn rejects_unsupported_inputs() {
        assert!(matches!(
            Basis::new(2, vec![0.5, 1.0], [0.0, 1.0]),
            Err(Error::InvalidKnots)
        ));
        let b = Basis::new(3, vec![], [0.0, 1.0]).unwrap();
        assert!(b.evaluate(-0.1, false).is_ok());
        assert_eq!(b.derivative(f64::NAN, false), Err(Error::NonFinite));
    }
}
