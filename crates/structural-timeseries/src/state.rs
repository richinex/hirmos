//! State equations from BOOM 0.9.16 LocalLevelStateModel, LocalLinearTrend,
//! and SeasonalStateModel. R constructor defaults live separately from equations.
//! Copyright 2018 Google LLC; Copyright 2005-2011 Steven L. Scott.
//! Derived equations follow the LGPL-2.1-or-later sources in vendor/Boom_0.9.16.tar.gz.
use crate::Error;
use nalgebra::{DMatrix, DVector};
use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug)]
pub struct Variance(f64);
impl Variance {
    pub fn new(value: f64) -> Result<Self, Error> {
        if !value.is_finite() {
            return Err(Error::NonFinite);
        }
        if value < 0.0 {
            return Err(Error::InvalidScale);
        }
        Ok(Self(value))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Normal {
    mean: f64,
    variance: Variance,
}
impl Normal {
    pub fn mean(self) -> f64 {
        self.mean
    }
    pub fn variance(self) -> Variance {
        self.variance
    }
    pub fn new(mean: f64, variance: Variance) -> Result<Self, Error> {
        if !mean.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(Self { mean, variance })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Season {
    count: NonZeroUsize,
    clock: SeasonClock,
}
#[derive(Clone, Copy, Debug)]
enum SeasonClock {
    Rows(NonZeroUsize),
    Months(jiff::civil::Date),
}
impl Season {
    pub fn dimension(self) -> usize {
        self.count.get() - 1
    }
    pub fn begins_at(self, row: usize) -> bool {
        match self.clock {
            SeasonClock::Rows(duration) => row % duration.get() == 0,
            SeasonClock::Months(first) => {
                // Gregorian month boundaries repeat every 400 years. The
                // constructor normalizes the origin to 2000..2399, so adding
                // fewer than 146097 days cannot leave Jiff's date range.
                first
                    .checked_add(jiff::Span::new().days((row % 146097) as i64))
                    .expect("bounded Gregorian-cycle offset")
                    .day()
                    == 1
            }
        }
    }
    fn begins_after(self, row: usize) -> bool {
        match self.clock {
            SeasonClock::Rows(duration) => row % duration.get() == duration.get() - 1,
            SeasonClock::Months(_) => self.begins_at(row % 146097 + 1),
        }
    }
    /// BOOM MonthlyAnnualCycle: daily rows; rotate on the first of each month.
    pub fn monthly(first: jiff::civil::Date) -> Self {
        let year = 2000 + first.year().rem_euclid(400);
        let first = jiff::civil::Date::new(year, first.month(), first.day())
            .expect("same Gregorian-cycle year preserves valid month/day");
        Self {
            count: NonZeroUsize::new(12).unwrap(),
            clock: SeasonClock::Months(first),
        }
    }
    pub fn new(count: usize, duration: usize) -> Result<Self, Error> {
        if count < 2 {
            return Err(Error::InvalidSeason);
        }
        Ok(Self {
            count: NonZeroUsize::new(count).ok_or(Error::InvalidSeason)?,
            clock: SeasonClock::Rows(NonZeroUsize::new(duration).ok_or(Error::InvalidSeason)?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Harmonics(Vec<f64>);
impl Harmonics {
    pub fn new(period: f64, frequencies: &[f64]) -> Result<Self, Error> {
        if !period.is_finite() || period <= 0.0 || frequencies.is_empty() {
            return Err(Error::InvalidSeason);
        }
        frequencies.len().checked_mul(2).ok_or(Error::Shape)?;
        let mut rotations = Vec::with_capacity(frequencies.len());
        for frequency in frequencies {
            if !frequency.is_finite() || *frequency <= 0.0 {
                return Err(Error::InvalidSeason);
            }
            let angle = 2.0 * std::f64::consts::PI * frequency / period;
            if !angle.is_finite() {
                return Err(Error::NonFinite);
            }
            rotations.push(angle);
        }
        Ok(Self(rotations))
    }
    pub fn dimension(&self) -> usize {
        2 * self.0.len()
    }
}

#[derive(Clone, Debug)]
pub struct Direct {
    cycle: Harmonics,
    innovations: Vec<Variance>,
    initial: Normal,
}
impl Direct {
    pub fn new(
        cycle: Harmonics,
        innovations: Vec<Variance>,
        initial: Normal,
    ) -> Result<Self, Error> {
        if innovations.len() != cycle.dimension() {
            return Err(Error::Shape);
        }
        Ok(Self {
            cycle,
            innovations,
            initial,
        })
    }
}

/// Ordered finite predictor rows, optionally including declared future rows.
/// Sharing the immutable schedule avoids copying data during every Gibbs sweep.
#[derive(Clone, Debug)]
pub struct Predictors(std::sync::Arc<DMatrix<f64>>);
impl Predictors {
    pub fn new(rows: DMatrix<f64>) -> Result<Self, Error> {
        if rows.nrows() == 0 || rows.ncols() == 0 {
            return Err(Error::Empty);
        }
        if rows.iter().any(|v| !v.is_finite()) {
            return Err(Error::MissingPredictors);
        }
        Ok(Self(std::sync::Arc::new(rows)))
    }
    pub fn columns(&self) -> usize {
        self.0.ncols()
    }
    pub fn rows(&self) -> usize {
        self.0.nrows()
    }
}

#[derive(Clone, Debug)]
pub struct Dynamic {
    predictors: Predictors,
    innovations: Vec<Variance>,
}
impl Dynamic {
    pub fn new(predictors: Predictors, innovations: Vec<Variance>) -> Result<Self, Error> {
        if predictors.columns() != innovations.len() {
            return Err(Error::Shape);
        }
        Ok(Self {
            predictors,
            innovations,
        })
    }
}

#[derive(Clone, Debug)]
pub struct DynamicAr {
    predictors: Predictors,
    coefficients: Vec<(crate::ar::Coefficients, Variance)>,
    dimension: usize,
}
impl DynamicAr {
    pub fn new(
        predictors: Predictors,
        coefficients: Vec<(crate::ar::Coefficients, Variance)>,
    ) -> Result<Self, Error> {
        if predictors.columns() != coefficients.len() {
            return Err(Error::Shape);
        }
        let order = coefficients.first().ok_or(Error::Empty)?.0.order();
        if coefficients.iter().any(|(c, _)| c.order() != order) {
            return Err(Error::Shape);
        }
        let dimension = order.checked_mul(coefficients.len()).ok_or(Error::Shape)?;
        Ok(Self {
            predictors,
            coefficients,
            dimension,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Component {
    Semilocal {
        level: Variance,
        slope: crate::semilocal::Parameters,
        initial_level: Normal,
        initial_slope: Normal,
    },
    DynamicAr(DynamicAr),
    Dynamic(Dynamic),
    SparseAr {
        coefficients: crate::sparse_ar::Coefficients,
        innovation: Variance,
    },
    Ar {
        coefficients: crate::ar::Coefficients,
        innovation: Variance,
    },
    Direct(Direct),
    Intercept {
        initial: Normal,
    },
    Harmonic {
        cycle: Harmonics,
        innovation: Variance,
        initial: Normal,
    },
    Level {
        innovation: Variance,
        initial: Normal,
    },
    Trend {
        level: Variance,
        slope: Variance,
        initial_level: Normal,
        initial_slope: Normal,
    },
    Seasonal {
        season: Season,
        innovation: Variance,
        initial: Normal,
    },
}
impl Component {
    pub fn dimension(&self) -> usize {
        match self {
            Self::Semilocal { .. } => 3,
            Self::DynamicAr(model) => model.dimension,
            Self::Dynamic(model) => model.predictors.columns(),
            Self::SparseAr { coefficients, .. } => coefficients.order(),
            Self::Ar { coefficients, .. } => coefficients.order(),
            Self::Direct(direct) => direct.cycle.dimension(),
            Self::Intercept { .. } => 1,
            Self::Harmonic { cycle, .. } => cycle.dimension(),
            Self::Level { .. } => 1,
            Self::Trend { .. } => 2,
            Self::Seasonal { season, .. } => season.count.get() - 1,
        }
    }
}

pub struct System {
    components: Vec<Component>,
    dimension: usize,
    initial: crate::initial::Initial,
    innovations: Option<Vec<Vec<Variance>>>,
}
impl System {
    pub fn new(components: Vec<Component>) -> Result<Self, Error> {
        if components.is_empty() {
            return Err(Error::Empty);
        }
        let dimension = components
            .iter()
            .try_fold(0usize, |n, c| n.checked_add(c.dimension()))
            .ok_or(Error::Shape)?;
        dimension.checked_mul(dimension).ok_or(Error::Shape)?;
        let (mean, covariance) = Self::component_initial(&components, dimension);
        let initial = crate::initial::Initial::new(mean, covariance)?;
        Ok(Self {
            components,
            dimension,
            initial,
            innovations: None,
        })
    }
    /// Conditional diagonal process noise, e.g. Student latent-scale mixtures.
    /// The schedule is finite; querying beyond it returns an error.
    pub fn with_innovations(mut self, innovations: Vec<Vec<Variance>>) -> Result<Self, Error> {
        if innovations.is_empty() {
            return Err(Error::Empty);
        }
        if innovations.iter().any(|row| row.len() != self.dimension) {
            return Err(Error::Shape);
        }
        self.innovations = Some(innovations);
        Ok(self)
    }
    pub fn with_initial(mut self, initial: crate::initial::Initial) -> Result<Self, Error> {
        if initial.dimension() != self.dimension {
            return Err(Error::Shape);
        }
        let mut offset = 0;
        for component in &self.components {
            if let Component::Semilocal { slope, .. } = component {
                let i = offset + 2;
                if initial.known_value(i)? != slope.mean() {
                    return Err(Error::InvalidScale);
                }
            }
            offset += component.dimension();
        }
        self.initial = initial;
        Ok(self)
    }
    pub fn initial_distribution(&self) -> &crate::initial::Initial {
        &self.initial
    }
    pub fn dimension(&self) -> usize {
        self.dimension
    }
    pub fn observation(&self) -> Result<DVector<f64>, Error> {
        self.observation_at(0)
    }
    pub fn observation_at(&self, row: usize) -> Result<DVector<f64>, Error> {
        let mut z = DVector::zeros(self.dimension);
        let mut offset = 0;
        for component in &self.components {
            match component {
                Component::DynamicAr(model) => {
                    if row >= model.predictors.rows() {
                        return Err(Error::MissingPredictors);
                    }
                    let mut position = offset;
                    for (j, (coefficients, _)) in model.coefficients.iter().enumerate() {
                        z[position] = model.predictors.0[(row, j)];
                        position += coefficients.order();
                    }
                }
                Component::Dynamic(model) => {
                    if row >= model.predictors.rows() {
                        return Err(Error::MissingPredictors);
                    }
                    for j in 0..model.predictors.columns() {
                        z[offset + j] = model.predictors.0[(row, j)];
                    }
                }
                Component::Direct(direct) => {
                    for (j, angle) in direct.cycle.0.iter().enumerate() {
                        z[offset + 2 * j] = (angle * row as f64).cos();
                        z[offset + 2 * j + 1] = (angle * row as f64).sin();
                    }
                }
                Component::Harmonic { cycle, .. } => {
                    for j in (0..cycle.dimension()).step_by(2) {
                        z[offset + j] = 1.0;
                    }
                }
                Component::SparseAr { .. }
                | Component::Semilocal { .. }
                | Component::Ar { .. }
                | Component::Intercept { .. }
                | Component::Level { .. }
                | Component::Trend { .. }
                | Component::Seasonal { .. } => z[offset] = 1.0,
            }
            offset += component.dimension();
        }
        Ok(z)
    }
    pub fn initial(&self) -> (DVector<f64>, DMatrix<f64>) {
        self.initial.moments()
    }
    fn component_initial(
        components: &[Component],
        dimension: usize,
    ) -> (DVector<f64>, DMatrix<f64>) {
        let mut mean = DVector::zeros(dimension);
        let mut covariance = DMatrix::zeros(dimension, dimension);
        let mut offset = 0;
        let ar_initial = Normal::new(0.0, Variance::new(1.0).unwrap()).unwrap();
        for component in components {
            let known_mean = match component {
                Component::Semilocal { slope, .. } => slope.mean(),
                _ => 0.,
            };
            let known = Normal::new(known_mean, Variance::new(0.).unwrap()).unwrap();
            for j in 0..component.dimension() {
                let initial = match component {
                    Component::Semilocal {
                        initial_level,
                        initial_slope,
                        ..
                    } => match j {
                        0 => initial_level,
                        1 => initial_slope,
                        _ => &known,
                    },
                    Component::DynamicAr(_)
                    | Component::Dynamic(_)
                    | Component::Ar { .. }
                    | Component::SparseAr { .. } => &ar_initial,
                    Component::Direct(direct) => &direct.initial,
                    Component::Intercept { initial } | Component::Harmonic { initial, .. } => {
                        initial
                    }
                    Component::Level { initial, .. } | Component::Seasonal { initial, .. } => {
                        initial
                    }
                    Component::Trend {
                        initial_level,
                        initial_slope,
                        ..
                    } => {
                        if j == 0 {
                            initial_level
                        } else {
                            initial_slope
                        }
                    }
                };
                mean[offset + j] = initial.mean;
                covariance[(offset + j, offset + j)] = initial.variance.value();
            }
            offset += component.dimension();
        }
        (mean, covariance)
    }
    /// Maps state at zero-based row t to state at t+1. Season duration is in rows.
    pub fn transition(&self, t: usize) -> Result<(DMatrix<f64>, DMatrix<f64>), Error> {
        let mut transition = DMatrix::identity(self.dimension, self.dimension);
        let mut noise = DMatrix::zeros(self.dimension, self.dimension);
        let mut offset = 0;
        for component in &self.components {
            match component {
                Component::Semilocal { level, slope, .. } => {
                    transition[(offset, offset + 1)] = 1.;
                    transition[(offset + 1, offset + 1)] = slope.phi();
                    transition[(offset + 1, offset + 2)] = 1. - slope.phi();
                    noise[(offset, offset)] = level.value();
                    noise[(offset + 1, offset + 1)] = slope.variance().value();
                }
                Component::DynamicAr(model) => {
                    let mut position = offset;
                    for (coefficients, innovation) in &model.coefficients {
                        companion(&mut transition, position, coefficients.values());
                        noise[(position, position)] = innovation.value();
                        position += coefficients.order();
                    }
                }
                Component::Dynamic(model) => {
                    for (j, variance) in model.innovations.iter().enumerate() {
                        noise[(offset + j, offset + j)] = variance.value();
                    }
                }
                Component::SparseAr {
                    coefficients,
                    innovation,
                } => {
                    companion(&mut transition, offset, coefficients.values());
                    noise[(offset, offset)] = innovation.value();
                }
                Component::Ar {
                    coefficients,
                    innovation,
                } => {
                    companion(&mut transition, offset, coefficients.values());
                    noise[(offset, offset)] = innovation.value();
                }
                Component::Direct(direct) => {
                    for (j, variance) in direct.innovations.iter().enumerate() {
                        noise[(offset + j, offset + j)] = variance.value();
                    }
                }
                Component::Intercept { .. } => {}
                Component::Harmonic {
                    cycle, innovation, ..
                } => {
                    for (j, angle) in cycle.0.iter().enumerate() {
                        let cosine = angle.cos();
                        let sine = angle.sin();
                        let i = offset + 2 * j;
                        transition[(i, i)] = cosine;
                        transition[(i, i + 1)] = sine;
                        transition[(i + 1, i)] = -sine;
                        transition[(i + 1, i + 1)] = cosine;
                        noise[(i, i)] = innovation.value();
                        noise[(i + 1, i + 1)] = innovation.value();
                    }
                }
                Component::Level { innovation, .. } => noise[(offset, offset)] = innovation.value(),
                Component::Trend { level, slope, .. } => {
                    transition[(offset, offset + 1)] = 1.0;
                    noise[(offset, offset)] = level.value();
                    noise[(offset + 1, offset + 1)] = slope.value();
                }
                Component::Seasonal {
                    season, innovation, ..
                } => {
                    // Avoid t+1 overflowing when querying an arbitrary row index.
                    if season.begins_after(t) {
                        let n = component.dimension();
                        for i in 0..n {
                            for j in 0..n {
                                transition[(offset + i, offset + j)] = if i == 0 {
                                    -1.0
                                } else if i == j + 1 {
                                    1.0
                                } else {
                                    0.0
                                };
                            }
                        }
                        noise[(offset, offset)] = innovation.value();
                    }
                }
            }
            offset += component.dimension();
        }
        if let Some(innovations) = &self.innovations {
            let row = innovations.get(t).ok_or(Error::Shape)?;
            noise.fill(0.);
            for (i, variance) in row.iter().enumerate() {
                noise[(i, i)] = variance.value();
            }
        }
        Ok((transition, noise))
    }
}

fn companion(transition: &mut DMatrix<f64>, offset: usize, coefficients: &[f64]) {
    for i in 0..coefficients.len() {
        for j in 0..coefficients.len() {
            transition[(offset + i, offset + j)] = if i == 0 {
                coefficients[j]
            } else if i == j + 1 {
                1.
            } else {
                0.
            };
        }
    }
}
