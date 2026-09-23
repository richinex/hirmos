//! R bsts 0.9.11 numeric default constructors for supported state components.
//! Sources include add.local.level.R, add.local.linear.trend.R, add.seasonal.R,
//! add.ar.R and add.dynamic.regression.R.
//! Copyright 2011 Google LLC. LGPL-2.1-or-later; original notices in vendor.
use crate::{
    state::{Component, Normal, Season, Variance},
    Error,
};

/// Positive, finite SD. Keeping this distinct from Variance prevents passing
/// R constructor scales directly to state covariance matrices.
#[derive(Clone, Copy, Debug)]
pub struct Scale(f64);
impl Scale {
    pub fn new(value: f64) -> Result<Self, Error> {
        if !value.is_finite() {
            return Err(Error::NonFinite);
        }
        if value <= 0.0 {
            return Err(Error::InvalidScale);
        }
        // The numerical state representation must also be finite and positive.
        if !value.powi(2).is_finite() || value.powi(2) == 0.0 {
            return Err(Error::InvalidScale);
        }
        Ok(Self(value))
    }
    pub fn value(self) -> f64 {
        self.0
    }
    pub fn variance(self) -> Variance {
        Variance::new(self.0 * self.0).expect("validated squared scale")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InnovationPrior {
    guess: Scale,
    upper: Scale,
}
impl InnovationPrior {
    pub fn update(self) -> Result<crate::prior::Update, Error> {
        Ok(crate::prior::Update::Sample {
            prior: crate::prior::Prior::new(
                self.guess,
                self.degrees_of_freedom(),
                crate::prior::Limit::At(self.upper),
            )?,
            initial: self.guess,
        })
    }
    pub fn guess(self) -> Scale {
        self.guess
    }
    pub fn upper(self) -> Scale {
        self.upper
    }
    pub fn degrees_of_freedom(self) -> f64 {
        0.01
    }
}

/// Defaults are not posterior draws. Their scales only initialize the state
/// model; fitting must subsequently apply the corresponding prior updates.
pub enum Specification {
    Level {
        prior: InnovationPrior,
        initial: Normal,
    },
    Trend {
        level: InnovationPrior,
        slope: InnovationPrior,
        initial_level: Normal,
        initial_slope: Normal,
    },
    Seasonal {
        season: Season,
        prior: InnovationPrior,
        initial: Normal,
    },
}
impl Specification {
    pub fn into_term(self) -> Result<crate::fit::Term, Error> {
        use crate::fit::Term;
        match self {
            Self::Level { prior, initial } => Ok(Term::level(prior.update()?, initial)),
            Self::Trend {
                level,
                slope,
                initial_level,
                initial_slope,
            } => Ok(Term::trend(
                level.update()?,
                slope.update()?,
                initial_level,
                initial_slope,
            )),
            Self::Seasonal {
                season,
                prior,
                initial,
            } => Ok(Term::seasonal(season, prior.update()?, initial)),
        }
    }
    pub fn initial_component(&self) -> Component {
        match *self {
            Self::Level { prior, initial } => Component::Level {
                innovation: prior.guess.variance(),
                initial,
            },
            Self::Trend {
                level,
                slope,
                initial_level,
                initial_slope,
            } => Component::Trend {
                level: level.guess.variance(),
                slope: slope.guess.variance(),
                initial_level,
                initial_slope,
            },
            Self::Seasonal {
                season,
                prior,
                initial,
            } => Component::Seasonal {
                season,
                innovation: prior.guess.variance(),
                initial,
            },
        }
    }
}

fn observed(y: &[Option<f64>]) -> Result<Vec<f64>, Error> {
    if y.iter().flatten().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let values: Vec<f64> = y.iter().flatten().copied().collect();
    if values.len() < 2 {
        return Err(Error::Empty);
    }
    Ok(values)
}

/// AddDynamicRegression's default random-walk priors, from
/// add.dynamic.regression.R (Google LLC, 2018, LGPL-2.1-or-later).
/// Accept training rows only: future predictors must not influence fitting.
/// This consumes an already expanded design matrix, not an R formula.
pub fn dynamic_regression(
    y: &[Option<f64>],
    training: &nalgebra::DMatrix<f64>,
) -> Result<Vec<crate::prior::Prior>, Error> {
    if training.nrows() != y.len() || training.ncols() == 0 {
        return Err(Error::Shape);
    }
    if training.iter().any(|x| !x.is_finite()) {
        return Err(Error::MissingPredictors);
    }
    let sd = |values: &[f64]| -> Result<Scale, Error> {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        Scale::new(
            (values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64)
                .sqrt(),
        )
    };
    let response_sd = sd(&observed(y)?)?;
    (0..training.ncols())
        .map(|j| {
            let predictor_sd = sd(training.column(j).as_slice())?;
            crate::prior::Prior::new(
                Scale::new(0.01 * response_sd.value() / predictor_sd.value())?,
                1.0,
                crate::prior::Limit::Unbounded,
            )
        })
        .collect()
}

/// AddAutoAr's constant-response fallback is visible to the caller, not logged
/// or silently substituted inside sampling. Fewer than two observations fail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoArNotice {
    None,
    ZeroStandardDeviation,
}

pub struct AutoArDefaults {
    pub prior: crate::sparse_ar::UnscaledSlab,
    pub notice: AutoArNotice,
}

pub fn auto_ar(y: &[Option<f64>], order: std::num::NonZeroUsize) -> Result<AutoArDefaults, Error> {
    let values = observed(y)?;
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let sd =
        (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64).sqrt();
    let (scale, notice) = if sd == 0.0 {
        (Scale::new(1.0)?, AutoArNotice::ZeroStandardDeviation)
    } else {
        (Scale::new(sd)?, AutoArNotice::None)
    };
    Ok(AutoArDefaults {
        prior: crate::sparse_ar::UnscaledSlab::r_default(order, scale)?,
        notice,
    })
}

fn prior(values: &[f64]) -> Result<InnovationPrior, Error> {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let sum = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>();
    let sd = Scale::new((sum / (values.len() - 1) as f64).sqrt())?;
    Ok(InnovationPrior {
        guess: Scale::new(0.01 * sd.value())?,
        upper: sd,
    })
}

/// AddAr uses SdPrior(0.01 * sdy), without the upper bound used by AddLocalLevel.
pub fn ar(y: &[Option<f64>]) -> Result<crate::prior::Prior, Error> {
    let values = observed(y)?;
    let scale = prior(&values)?.guess;
    crate::prior::Prior::new(scale, 0.01, crate::prior::Limit::Unbounded)
}

pub fn level(y: &[Option<f64>]) -> Result<Specification, Error> {
    let values = observed(y)?;
    let prior = prior(&values)?;
    // Unlike the trend constructor, R takes the first row, not first observed.
    // A missing first row needs an explicitly supplied initial prior.
    let initial = Normal::new(y[0].ok_or(Error::MissingInitial)?, prior.upper.variance())?;
    Ok(Specification::Level { prior, initial })
}

pub fn trend(y: &[Option<f64>]) -> Result<Specification, Error> {
    let values = observed(y)?;
    let prior = prior(&values)?;
    let initial_level = Normal::new(values[0], prior.upper.variance())?;
    let slope = (values[values.len() - 1] - values[0]) / y.len() as f64;
    let initial_slope = Normal::new(slope, prior.upper.variance())?;
    Ok(Specification::Trend {
        level: prior,
        slope: prior,
        initial_level,
        initial_slope,
    })
}

/// Numeric defaults of AddSemilocalLinearTrend. The deprecated generalized
/// trend constructor delegates to this same specification in R.
pub struct SemilocalDefaults {
    pub level: InnovationPrior,
    pub slope: InnovationPrior,
    pub mean: Normal,
    pub phi: Normal,
    pub initial_level: Normal,
    pub initial_slope: Normal,
}
impl SemilocalDefaults {
    pub fn into_term(self, seed: u32) -> Result<crate::fit::Term, Error> {
        let slope_prior = crate::prior::Prior::new(
            self.slope.guess,
            self.slope.degrees_of_freedom(),
            crate::prior::Limit::At(self.slope.upper),
        )?;
        let slope = crate::semilocal::Sampler::new(
            self.mean,
            self.phi,
            slope_prior,
            crate::semilocal::Support::Stationary,
            crate::semilocal::Parameters::new(self.mean.mean(), self.phi.mean(), self.slope.guess)?,
            seed,
        )?;
        Ok(crate::fit::Term::semilocal(
            self.level.update()?,
            slope,
            self.initial_level,
            self.initial_slope,
        ))
    }
}
pub fn semilocal(y: &[Option<f64>]) -> Result<SemilocalDefaults, Error> {
    let values = observed(y)?;
    let prior = prior(&values)?;
    let zero = Normal::new(0., prior.upper.variance())?;
    Ok(SemilocalDefaults {
        level: prior,
        slope: prior,
        mean: zero,
        phi: Normal::new(0., Variance::new(1.)?)?,
        initial_level: Normal::new(y[0].ok_or(Error::MissingInitial)?, prior.upper.variance())?,
        initial_slope: zero,
    })
}

pub fn seasonal(y: &[Option<f64>], season: Season) -> Result<Specification, Error> {
    let values = observed(y)?;
    let prior = prior(&values)?;
    Ok(Specification::Seasonal {
        season,
        prior,
        initial: Normal::new(0.0, prior.upper.variance())?,
    })
}

/// AddStudentLocalLinearTrend forwards its Gaussian prior construction to
/// AddLocalLinearTrend. Its C++ factory starts both scales at one and nu at ten,
/// ignoring SD-prior initial/fixed flags, unlike the Gaussian trend factory.
pub fn student_trend(y: Vec<Option<f64>>, seed: u32) -> Result<crate::student::Trend, Error> {
    let Specification::Trend {
        level,
        slope,
        initial_level,
        initial_slope,
    } = trend(&y)?
    else {
        unreachable!("trend constructor")
    };
    let parameters = crate::student::Parameters::new(Scale::new(1.)?, 10.)?;
    let tail = crate::student::UniformTail::new(1., 500.)?;
    let innovation = |prior: InnovationPrior, stream: u32| {
        crate::student::Innovation::new(
            parameters,
            crate::prior::Prior::new(
                prior.guess,
                prior.degrees_of_freedom(),
                crate::prior::Limit::At(prior.upper),
            )?,
            tail,
            y.len() - 1,
            stream,
        )
    };
    let level = innovation(level, seed.wrapping_add(2))?;
    let slope = innovation(slope, seed.wrapping_add(3))?;
    let observation = gaussian(&y)?;
    crate::student::Trend::new(
        level,
        slope,
        initial_level,
        initial_slope,
        observation,
        y,
        seed,
    )
}

/// SpikeSlabPrior defaults use the first coefficient for the response mean.
/// BSTS's own regression default instead supplies an explicit zero vector.
/// This selects only the mean convention. BSTS also imposes a residual SD
/// ceiling, which this unbounded SpikeSlabPrior constructor does not supply.
pub enum RegressionMean {
    FirstResponseMean,
    Zero,
    Coefficients(nalgebra::DVector<f64>),
}

/// bsts 0.9.11 Gaussian observation default without regression.
pub fn gaussian(y: &[Option<f64>]) -> Result<crate::prior::Prior, Error> {
    let values = observed(y)?;
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance =
        values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
    let sd = Scale::new(variance.sqrt())?;
    crate::prior::Prior::new(
        sd,
        0.01,
        crate::prior::Limit::At(Scale::new(1.2 * sd.value())?),
    )
}

enum SelectionPrior {
    ExpectedSize(f64),
    Probabilities(Vec<crate::regression::Inclusion>),
}

/// Validated scalar hyperparameters for R's proper Gaussian SpikeSlabPrior.
/// Design-dependent dimensions and positive definiteness are checked by build.
pub struct RegressionPrior {
    r2: f64,
    df: f64,
    weight: f64,
    shrinkage: f64,
    selection: SelectionPrior,
}
impl RegressionPrior {
    pub fn new(r2: f64, df: f64, weight: f64, shrinkage: f64) -> Result<Self, Error> {
        if !r2.is_finite() || !(0.0..1.0).contains(&r2) || r2 == 0.0 {
            return Err(Error::InvalidProbability);
        }
        if !df.is_finite() || df <= 0.0 || !weight.is_finite() || weight <= 0.0 {
            return Err(Error::InvalidScale);
        }
        if !shrinkage.is_finite() || !(0.0..=1.0).contains(&shrinkage) {
            return Err(Error::InvalidProbability);
        }
        Ok(Self {
            r2,
            df,
            weight,
            shrinkage,
            selection: SelectionPrior::ExpectedSize(1.0),
        })
    }
    pub fn with_expected_size(mut self, size: f64) -> Result<Self, Error> {
        if !size.is_finite() || size <= 0.0 {
            return Err(Error::InvalidScale);
        }
        self.selection = SelectionPrior::ExpectedSize(size);
        Ok(self)
    }
    pub fn with_probabilities(mut self, probabilities: Vec<crate::regression::Inclusion>) -> Self {
        self.selection = SelectionPrior::Probabilities(probabilities);
        self
    }
    pub fn build(
        &self,
        x: &nalgebra::DMatrix<f64>,
        y: &[Option<f64>],
        mean: RegressionMean,
    ) -> Result<crate::regression::Slab, Error> {
        use crate::regression::{Inclusion, Slab};
        use nalgebra::{DMatrix, DVector};
        if x.nrows() != y.len() || x.ncols() == 0 {
            return Err(Error::Shape);
        }
        if x.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        let values = observed(y)?;
        let center = values.iter().sum::<f64>() / values.len() as f64;
        let sum = values.iter().map(|x| (x - center).powi(2)).sum::<f64>();
        let sigma = Scale::new(((1.0 - self.r2) * sum / (values.len() - 1) as f64).sqrt())?;
        let xtx = x.transpose() * x / x.nrows() as f64;
        let p = x.ncols();
        let precision = DMatrix::from_fn(p, p, |i, j| {
            self.weight
                * if i == j {
                    xtx[(i, j)]
                } else {
                    (1.0 - self.shrinkage) * xtx[(i, j)]
                }
        });
        let mu = match mean {
            RegressionMean::FirstResponseMean => {
                let mut m = DVector::zeros(p);
                m[0] = center;
                m
            }
            RegressionMean::Zero => DVector::zeros(p),
            RegressionMean::Coefficients(m) => m,
        };
        let probabilities = match &self.selection {
            SelectionPrior::ExpectedSize(size) => {
                vec![Inclusion::new((size / p as f64).min(1.0))?; p]
            }
            SelectionPrior::Probabilities(values) => values.clone(),
        };
        Slab::new(mu, precision, probabilities, sigma, self.df)
    }
}
impl Default for RegressionPrior {
    fn default() -> Self {
        Self {
            r2: 0.5,
            df: 0.01,
            weight: 0.01,
            shrinkage: 0.5,
            selection: SelectionPrior::ExpectedSize(1.0),
        }
    }
}

/// Gaussian SSVS defaults from bsts 0.9.11 .SetDefaultPrior.
pub fn bsts_regression(
    x: &nalgebra::DMatrix<f64>,
    y: &[Option<f64>],
) -> Result<crate::regression::Slab, Error> {
    let values = observed(y)?;
    let center = values.iter().sum::<f64>() / values.len() as f64;
    let variance =
        values.iter().map(|v| (v - center).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
    Ok(regression(x, y, RegressionMean::Zero)?
        .with_residual_ceiling(Scale::new(1.2 * variance.sqrt())?))
}

/// Gaussian ODA defaults as consumed by BSTS's C++ prior adapter. That adapter
/// scales the coefficient variances by residual variance regardless of R's
/// scale.by.residual.variance field (Boom/src/spike_slab_prior.cpp).
pub fn bsts_oda(
    x: &nalgebra::DMatrix<f64>,
    y: &[Option<f64>],
) -> Result<crate::regression::Slab, Error> {
    use crate::regression::{Inclusion, Slab};
    use nalgebra::{DMatrix, DVector};
    if x.nrows() != y.len() || x.ncols() == 0 {
        return Err(Error::Shape);
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let values = observed(y)?;
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance =
        values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
    let mut precision = DVector::zeros(x.ncols());
    for j in 0..x.ncols() {
        let mean = x.column(j).sum() / x.nrows() as f64;
        let v =
            x.column(j).iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (x.nrows() - 1) as f64;
        precision[j] = (if v == 0.0 { 1.0 } else { v }) / (100.0 * variance);
    }
    Ok(Slab::new(
        DVector::zeros(x.ncols()),
        DMatrix::from_diagonal(&precision),
        vec![Inclusion::new(1.0 / x.ncols() as f64)?; x.ncols()],
        Scale::new((0.5 * variance).sqrt())?,
        0.01,
    )?
    .with_residual_ceiling(Scale::new(1.2 * variance.sqrt())?))
}

pub fn regression(
    x: &nalgebra::DMatrix<f64>,
    y: &[Option<f64>],
    mean: RegressionMean,
) -> Result<crate::regression::Slab, Error> {
    RegressionPrior::default().build(x, y, mean)
}
