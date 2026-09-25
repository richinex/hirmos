//! Histogram accumulation from scikit-learn 1.9.0 (BSD-3-Clause).
//! See reference/sklearn-1.9.0-gcm/licenses/COPYING.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Bin {
    pub gradient: f64,
    pub hessian: f64,
    pub count: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HistogramError {
    Shape,
    NonFinite,
    Index,
    Count,
}

/// Constant curvature omits per-row storage and histogram Hessian accumulation.
pub enum Hessians {
    Constant,
    Variable(Vec<f32>),
}

pub struct Derivatives {
    gradients: Vec<f32>,
    hessians: Hessians,
}

/// A checked view for trees whose Hessian is one for every observation.
pub struct UnitDerivatives<'a>(&'a Derivatives);
impl<'a> TryFrom<&'a Derivatives> for UnitDerivatives<'a> {
    type Error = HistogramError;
    fn try_from(value: &'a Derivatives) -> Result<Self, Self::Error> {
        match value.hessians {
            Hessians::Constant => Ok(Self(value)),
            Hessians::Variable(_) => Err(HistogramError::Shape),
        }
    }
}
impl UnitDerivatives<'_> {
    pub fn derivatives(&self) -> &Derivatives {
        self.0
    }
}

impl Derivatives {
    pub fn new(gradients: Vec<f32>, hessians: Hessians) -> Result<Self, HistogramError> {
        if gradients.is_empty() {
            return Err(HistogramError::Shape);
        }
        if gradients.iter().any(|v| !v.is_finite()) {
            return Err(HistogramError::NonFinite);
        }
        if let Hessians::Variable(values) = &hessians {
            if values.len() != gradients.len() {
                return Err(HistogramError::Shape);
            }
            if values.iter().any(|v| !v.is_finite()) {
                return Err(HistogramError::NonFinite);
            }
        }
        Ok(Self {
            gradients,
            hessians,
        })
    }
    /// CyHalfSquaredError gradient and constant Hessian, cast to the boosting dtype.
    pub fn squared_error(observed: &[f64], predicted: &[f64]) -> Result<Self, HistogramError> {
        if observed.len() != predicted.len() {
            return Err(HistogramError::Shape);
        }
        if observed.iter().chain(predicted).any(|v| !v.is_finite()) {
            return Err(HistogramError::NonFinite);
        }
        Self::new(
            observed
                .iter()
                .zip(predicted)
                .map(|(&y, &p)| (p - y) as f32)
                .collect(),
            Hessians::Constant,
        )
    }
    /// CyHalfBinomialLoss gradient and Hessian, with the Taylor branch at -37.
    pub fn half_binomial(observed: &[f64], raw: &[f64]) -> Result<Self, HistogramError> {
        if observed.len() != raw.len() {
            return Err(HistogramError::Shape);
        }
        if observed.iter().chain(raw).any(|v| !v.is_finite()) {
            return Err(HistogramError::NonFinite);
        }
        let mut gradients = Vec::with_capacity(observed.len());
        let mut hessians = Vec::with_capacity(observed.len());
        for (&y, &value) in observed.iter().zip(raw) {
            let (gradient, hessian) = if value > -37.0 {
                let exponential = (-value).exp();
                (
                    ((1.0 - y) - y * exponential) / (1.0 + exponential),
                    exponential / (1.0 + exponential).powi(2),
                )
            } else {
                let exponential = value.exp();
                (exponential - y, exponential)
            };
            gradients.push(gradient as f32);
            hessians.push(hessian as f32);
        }
        Self::new(gradients, Hessians::Variable(hessians))
    }
    pub fn gradients(&self) -> &[f32] {
        &self.gradients
    }
    pub fn curvature(&self) -> crate::gcm::boosting::split::Curvature {
        match self.hessians {
            Hessians::Constant => crate::gcm::boosting::split::Curvature::Unit,
            Hessians::Variable(_) => crate::gcm::boosting::split::Curvature::Variable,
        }
    }
}

/// A feature's bins and derivatives are validated once, before node scans.
pub struct Feature<'a> {
    bins: &'a [u8],
    derivatives: &'a Derivatives,
    width: usize,
}

impl<'a> Feature<'a> {
    pub fn new(
        bins: &'a [u8],
        derivatives: &'a Derivatives,
        width: usize,
    ) -> Result<Self, HistogramError> {
        if width == 0 || width > 256 || bins.len() != derivatives.gradients.len() {
            return Err(HistogramError::Shape);
        }
        if bins.iter().any(|&b| b as usize >= width) {
            return Err(HistogramError::Index);
        }
        if bins.len() > u32::MAX as usize {
            return Err(HistogramError::Count);
        }
        Ok(Self {
            bins,
            derivatives,
            width,
        })
    }
    pub fn root(&self) -> Vec<Bin> {
        self.accumulate(0..self.bins.len())
    }
    pub fn node(&self, rows: &[usize]) -> Result<Vec<Bin>, HistogramError> {
        if rows.len() > u32::MAX as usize {
            return Err(HistogramError::Count);
        }
        if rows.iter().any(|&r| r >= self.bins.len()) {
            return Err(HistogramError::Index);
        }
        // The source's full-length fast path ignores sample_indices.
        if rows.len() == self.bins.len() {
            return Ok(self.root());
        }
        Ok(self.accumulate(rows.iter().copied()))
    }
    fn accumulate(&self, rows: impl Iterator<Item = usize>) -> Vec<Bin> {
        let mut result = vec![Bin::default(); self.width];
        for row in rows {
            let bin = &mut result[self.bins[row] as usize];
            bin.gradient += self.derivatives.gradients[row] as f64;
            if let Hessians::Variable(values) = &self.derivatives.hessians {
                bin.hessian += values[row] as f64;
            }
            bin.count += 1;
        }
        result
    }
}

/// Reuse the parent's buffer to obtain the sibling histogram in O(number of bins).
pub fn subtract(parent: &mut [Bin], child: &[Bin]) -> Result<(), HistogramError> {
    if parent.len() != child.len() {
        return Err(HistogramError::Shape);
    }
    if parent.iter().zip(child).any(|(p, c)| c.count > p.count) {
        return Err(HistogramError::Count);
    }
    for (parent, child) in parent.iter_mut().zip(child) {
        parent.gradient -= child.gradient;
        parent.hessian -= child.hessian;
        parent.count -= child.count;
    }
    Ok(())
}
