//! Gaussian Gibbs orchestration following BOOM StateSpacePosteriorSampler.
//! Copyright Google LLC and Steven L. Scott; LGPL-2.1-or-later.
use crate::{
    defaults::Scale,
    gaussian,
    prior::{Prior, Statistics, Update},
    state::{Component, Direct, Harmonics, Normal, Season, System, Variance},
    Error,
};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use nalgebra::{DMatrix, DVector};

#[derive(Clone)]
enum Observation {
    Gaussian(Prior),
    Regression {
        model: crate::regression::Regression,
        predictors: DMatrix<f64>,
        rows: Vec<usize>,
    },
}

#[derive(Clone)]
struct Parameter {
    update: Update,
    value: Variance,
}
impl Parameter {
    fn new(update: Update) -> Self {
        Self {
            value: update.initial(),
            update,
        }
    }
    fn sample(&mut self, stats: Statistics, rng: &mut Mt19937) -> Result<(), Error> {
        self.value = self.update.draw(stats, rng)?;
        Ok(())
    }
}

#[derive(Clone)]
enum Mechanism {
    Semilocal {
        level: Parameter,
        slope: crate::semilocal::Sampler,
        initial_level: Normal,
        initial_slope: Normal,
    },
    DynamicAr {
        predictors: crate::state::Predictors,
        samplers: Vec<crate::ar::Sampler>,
    },
    Dynamic {
        predictors: crate::state::Predictors,
        innovations: Vec<Parameter>,
    },
    SparseAr(crate::sparse_ar::Sampler),
    Ar(crate::ar::Sampler),
    Direct {
        cycle: Harmonics,
        innovations: Vec<Parameter>,
        initial: Normal,
    },
    Intercept {
        initial: Normal,
    },
    Harmonic {
        cycle: Harmonics,
        innovation: Parameter,
        initial: Normal,
    },
    Level {
        innovation: Parameter,
        initial: Normal,
    },
    Trend {
        level: Parameter,
        slope: Parameter,
        initial_level: Normal,
        initial_slope: Normal,
    },
    Seasonal {
        season: Season,
        innovation: Parameter,
        initial: Normal,
    },
}

#[derive(Clone)]
pub struct Term(Mechanism);
impl Term {
    pub fn semilocal(
        level: Update,
        slope: crate::semilocal::Sampler,
        initial_level: Normal,
        initial_slope: Normal,
    ) -> Self {
        Self(Mechanism::Semilocal {
            level: Parameter::new(level),
            slope,
            initial_level,
            initial_slope,
        })
    }
    pub fn dynamic_ar(
        predictors: crate::state::Predictors,
        samplers: Vec<crate::ar::Sampler>,
    ) -> Result<Self, Error> {
        crate::state::DynamicAr::new(
            predictors.clone(),
            samplers
                .iter()
                .map(|s| (s.coefficients().clone(), s.variance()))
                .collect(),
        )?;
        Ok(Self(Mechanism::DynamicAr {
            predictors,
            samplers,
        }))
    }
    pub fn dynamic(
        predictors: crate::state::Predictors,
        priors: Vec<Prior>,
    ) -> Result<Self, Error> {
        if priors.len() != predictors.columns() {
            return Err(Error::Shape);
        }
        let innovations = priors
            .into_iter()
            .map(|prior| {
                Parameter::new(Update::Sample {
                    prior,
                    initial: Scale::new(1.0).expect("unit scale"),
                })
            })
            .collect();
        Ok(Self(Mechanism::Dynamic {
            predictors,
            innovations,
        }))
    }
    pub fn sparse_ar(sampler: crate::sparse_ar::Sampler) -> Self {
        Self(Mechanism::SparseAr(sampler))
    }
    pub fn ar_with_sampler(sampler: crate::ar::Sampler) -> Self {
        Self(Mechanism::Ar(sampler))
    }
    pub fn ar(order: std::num::NonZeroUsize, prior: Prior, seed: u32) -> Self {
        Self(Mechanism::Ar(crate::ar::Sampler::new(order, prior, seed)))
    }
    pub fn direct(cycle: Harmonics, prior: Prior, initial: Normal) -> Self {
        let innovations = vec![
            Parameter::new(Update::Sample {
                prior,
                initial: Scale::new(1.0).expect("unit scale")
            });
            cycle.dimension()
        ];
        Self(Mechanism::Direct {
            cycle,
            innovations,
            initial,
        })
    }
    pub fn intercept(initial: Normal) -> Self {
        Self(Mechanism::Intercept { initial })
    }
    pub fn harmonic(cycle: Harmonics, prior: Prior, initial: Normal) -> Self {
        // R's harmonic factory installs a sampler and leaves its initial SD at
        // one; it does not use SdPrior.fixed or SdPrior.initial.value.
        Self(Mechanism::Harmonic {
            cycle,
            innovation: Parameter::new(Update::Sample {
                prior,
                initial: Scale::new(1.0).expect("unit scale"),
            }),
            initial,
        })
    }
    pub fn level(innovation: Update, initial: Normal) -> Self {
        Self(Mechanism::Level {
            innovation: Parameter::new(innovation),
            initial,
        })
    }
    pub fn trend(
        level: Update,
        slope: Update,
        initial_level: Normal,
        initial_slope: Normal,
    ) -> Self {
        Self(Mechanism::Trend {
            level: Parameter::new(level),
            slope: Parameter::new(slope),
            initial_level,
            initial_slope,
        })
    }
    pub fn seasonal(season: Season, innovation: Update, initial: Normal) -> Self {
        Self(Mechanism::Seasonal {
            season,
            innovation: Parameter::new(innovation),
            initial,
        })
    }
    fn component(&self) -> Component {
        match &self.0 {
            Mechanism::Semilocal {
                level,
                slope,
                initial_level,
                initial_slope,
            } => Component::Semilocal {
                level: level.value,
                slope: slope.parameters(),
                initial_level: *initial_level,
                initial_slope: *initial_slope,
            },
            Mechanism::DynamicAr {
                predictors,
                samplers,
            } => Component::DynamicAr(
                crate::state::DynamicAr::new(
                    predictors.clone(),
                    samplers
                        .iter()
                        .map(|s| (s.coefficients().clone(), s.variance()))
                        .collect(),
                )
                .expect("validated dynamic AR layout"),
            ),
            Mechanism::Dynamic {
                predictors,
                innovations,
            } => Component::Dynamic(
                crate::state::Dynamic::new(
                    predictors.clone(),
                    innovations.iter().map(|p| p.value).collect(),
                )
                .expect("validated dynamic coefficient count"),
            ),
            Mechanism::SparseAr(sampler) => Component::SparseAr {
                coefficients: sampler.coefficients().clone(),
                innovation: sampler.variance(),
            },
            Mechanism::Ar(sampler) => Component::Ar {
                coefficients: sampler.coefficients().clone(),
                innovation: sampler.variance(),
            },
            Mechanism::Direct {
                cycle,
                innovations,
                initial,
            } => Component::Direct(
                Direct::new(
                    cycle.clone(),
                    innovations.iter().map(|p| p.value).collect(),
                    *initial,
                )
                .expect("parameters sized by cycle constructor"),
            ),
            Mechanism::Intercept { initial } => Component::Intercept { initial: *initial },
            Mechanism::Harmonic {
                cycle,
                innovation,
                initial,
            } => Component::Harmonic {
                cycle: cycle.clone(),
                innovation: innovation.value,
                initial: *initial,
            },
            Mechanism::Level {
                innovation,
                initial,
            } => Component::Level {
                innovation: innovation.value,
                initial: *initial,
            },
            Mechanism::Trend {
                level,
                slope,
                initial_level,
                initial_slope,
            } => Component::Trend {
                level: level.value,
                slope: slope.value,
                initial_level: *initial_level,
                initial_slope: *initial_slope,
            },
            Mechanism::Seasonal {
                season,
                innovation,
                initial,
            } => Component::Seasonal {
                season: *season,
                innovation: innovation.value,
                initial: *initial,
            },
        }
    }
    fn sample(
        &mut self,
        states: &[DVector<f64>],
        offset: usize,
        rng: &mut Mt19937,
    ) -> Result<(), Error> {
        let component = self.component();
        let dimension = component.dimension();
        let statistics = |residual: &dyn Fn(usize,&DVector<f64>,&DVector<f64>)->Option<f64>| -> Result<Statistics,Error> {
            let mut stats = Statistics::default();
            for t in 1..states.len() {
                if let Some(value) = residual(t,&states[t-1],&states[t]) { stats.add(value)?; }
            }
            Ok(stats)
        };
        match &mut self.0 {
            Mechanism::Semilocal { level, slope, .. } => {
                level.sample(
                    statistics(&|_, then, now| {
                        Some(now[offset] - then[offset] - then[offset + 1])
                    })?,
                    rng,
                )?;
                slope.step(
                    &states
                        .iter()
                        .map(|state| state[offset + 1])
                        .collect::<Vec<_>>(),
                )
            }
            Mechanism::DynamicAr { samplers, .. } => {
                let n = states.len().saturating_sub(1);
                let mut position = offset;
                for sampler in samplers {
                    let p = sampler.coefficients().order();
                    let x = DMatrix::from_fn(n, p, |i, j| states[i][position + j]);
                    let y = DVector::from_fn(n, |i, _| states[i + 1][position]);
                    sampler.step(&x, &y)?;
                    position += p;
                }
                Ok(())
            }
            Mechanism::Dynamic { innovations, .. } => {
                for (j, parameter) in innovations.iter_mut().enumerate() {
                    parameter.sample(
                        statistics(&|_, then, now| Some(now[offset + j] - then[offset + j]))?,
                        rng,
                    )?;
                }
                Ok(())
            }
            Mechanism::SparseAr(sampler) => {
                let n = states.len().saturating_sub(1);
                let x = DMatrix::from_fn(n, dimension, |i, j| states[i][offset + j]);
                let y = DVector::from_fn(n, |i, _| states[i + 1][offset]);
                sampler.step(&x, &y)
            }
            Mechanism::Ar(sampler) => {
                let n = states.len().saturating_sub(1);
                let x = DMatrix::from_fn(n, dimension, |i, j| states[i][offset + j]);
                let y = DVector::from_fn(n, |i, _| states[i + 1][offset]);
                sampler.step(&x, &y)
            }
            Mechanism::Intercept { .. } => Ok(()),
            Mechanism::Direct { innovations, .. } => {
                for (j, parameter) in innovations.iter_mut().enumerate() {
                    parameter.sample(
                        statistics(&|_, then, now| Some(now[offset + j] - then[offset + j]))?,
                        rng,
                    )?;
                }
                Ok(())
            }
            Mechanism::Harmonic { innovation, .. } => {
                let transition = System::new(vec![component])?.transition(0)?.0;
                let mut stats = Statistics::default();
                for t in 1..states.len() {
                    let rotated = &transition * states[t - 1].rows(offset, dimension);
                    for j in 0..dimension {
                        stats.add(states[t][offset + j] - rotated[j])?;
                    }
                }
                innovation.sample(stats, rng)
            }
            Mechanism::Level { innovation, .. } => innovation.sample(
                statistics(&|_, then, now| Some(now[offset] - then[offset]))?,
                rng,
            ),
            Mechanism::Seasonal {
                season, innovation, ..
            } => innovation.sample(
                statistics(&|t, then, now| {
                    season
                        .begins_at(t)
                        .then(|| now[offset] + then.rows(offset, dimension).iter().sum::<f64>())
                })?,
                rng,
            ),
            Mechanism::Trend { level, slope, .. } => {
                level.sample(
                    statistics(&|_, then, now| {
                        Some(now[offset] - then[offset] - then[offset + 1])
                    })?,
                    rng,
                )?;
                slope.sample(
                    statistics(&|_, then, now| Some(now[offset + 1] - then[offset + 1]))?,
                    rng,
                )
            }
        }
    }
    fn variances(&self) -> Vec<f64> {
        match &self.0 {
            Mechanism::Semilocal { level, slope, .. } => {
                vec![level.value.value(), slope.parameters().variance().value()]
            }
            Mechanism::DynamicAr { samplers, .. } => {
                samplers.iter().map(|s| s.variance().value()).collect()
            }
            Mechanism::Dynamic { innovations, .. } => {
                innovations.iter().map(|p| p.value.value()).collect()
            }
            Mechanism::SparseAr(sampler) => vec![sampler.variance().value()],
            Mechanism::Ar(sampler) => vec![sampler.variance().value()],
            Mechanism::Direct { innovations, .. } => {
                innovations.iter().map(|p| p.value.value()).collect()
            }
            Mechanism::Intercept { .. } => vec![],
            Mechanism::Level { innovation, .. }
            | Mechanism::Seasonal { innovation, .. }
            | Mechanism::Harmonic { innovation, .. } => vec![innovation.value.value()],
            Mechanism::Trend { level, slope, .. } => vec![level.value.value(), slope.value.value()],
        }
    }
}

/// A live chain owns its parameters, data and RNGs. It cannot borrow a view's
/// transient settings or silently resume with a different series.
pub struct Chain {
    terms: Vec<Term>,
    initial: crate::initial::Initial,
    y: Vec<Option<f64>>,
    observation_model: Observation,
    observation: Variance,
    states: Vec<DVector<f64>>,
    state_rng: NpRng,
    variance_rng: Mt19937,
}

pub struct Draw {
    pub semilocal: Vec<Option<crate::semilocal::Parameters>>,
    pub dynamic_ar: Vec<Option<Vec<crate::ar::Draw>>>,
    pub sparse_ar: Vec<Option<crate::sparse_ar::Draw>>,
    pub ar: Vec<Option<crate::ar::Draw>>,
    pub regression: Option<crate::regression::Draw>,
    pub observation_variance: f64,
    pub component_variances: Vec<Vec<f64>>,
    pub states: Vec<Vec<f64>>,
    pub log_likelihood: f64,
}

impl Chain {
    pub fn with_regression(
        terms: Vec<Term>,
        slab: crate::regression::Slab,
        predictors: DMatrix<f64>,
        y: Vec<Option<f64>>,
        seed: u32,
    ) -> Result<Self, Error> {
        Self::with_regression_sweep(
            terms,
            slab,
            predictors,
            y,
            seed,
            crate::regression::FlipSweep::All,
        )
    }
    pub fn with_regression_sweep(
        terms: Vec<Term>,
        slab: crate::regression::Slab,
        predictors: DMatrix<f64>,
        y: Vec<Option<f64>>,
        seed: u32,
        sweep: crate::regression::FlipSweep,
    ) -> Result<Self, Error> {
        Self::with_regression_sampling(
            terms,
            slab,
            predictors,
            y,
            seed,
            crate::regression::Sampling::Ssvs(sweep),
        )
    }
    pub fn with_regression_sampling(
        terms: Vec<Term>,
        slab: crate::regression::Slab,
        predictors: DMatrix<f64>,
        y: Vec<Option<f64>>,
        seed: u32,
        sampling: crate::regression::Sampling,
    ) -> Result<Self, Error> {
        if predictors.nrows() != y.len() {
            return Err(Error::Shape);
        }
        if predictors.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        let rows: Vec<_> = y
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|_| i))
            .collect();
        let x = DMatrix::from_fn(rows.len(), predictors.ncols(), |i, j| {
            predictors[(rows[i], j)]
        });
        let response = DVector::from_iterator(
            rows.len(),
            rows.iter().map(|i| y[*i].expect("observed row")),
        );
        let prior = slab.residual_prior();
        let model = crate::regression::Regression::new(x, response, slab, seed.wrapping_add(2))?
            .with_sampling(sampling)?;
        let mut chain = Self::new(terms, prior, y, seed)?;
        chain.observation_model = Observation::Regression {
            model,
            predictors,
            rows,
        };
        Ok(chain)
    }
    pub fn new(
        terms: Vec<Term>,
        observation_prior: Prior,
        y: Vec<Option<f64>>,
        seed: u32,
    ) -> Result<Self, Error> {
        let system = System::new(terms.iter().map(Term::component).collect())?;
        Self::start(terms, observation_prior, y, seed, system)
    }
    pub fn with_initial(
        terms: Vec<Term>,
        observation_prior: Prior,
        y: Vec<Option<f64>>,
        seed: u32,
        initial: crate::initial::Initial,
    ) -> Result<Self, Error> {
        let system =
            System::new(terms.iter().map(Term::component).collect())?.with_initial(initial)?;
        Self::start(terms, observation_prior, y, seed, system)
    }
    fn start(
        terms: Vec<Term>,
        observation_prior: Prior,
        y: Vec<Option<f64>>,
        seed: u32,
        system: System,
    ) -> Result<Self, Error> {
        if !y.iter().any(Option::is_some) {
            return Err(Error::Empty);
        }
        let mut state_rng = NpRng::seeded(seed as u64);
        // The R Gaussian manager leaves the observation model's starting SD at
        // one, ignoring SdPrior.initial.value as well as SdPrior.fixed.
        let observation = Variance::new(1.0)?;
        let states = gaussian::draw_states(&system, observation, &y, &mut state_rng)?;
        Ok(Self {
            terms,
            initial: system.initial_distribution().clone(),
            y,
            observation_model: Observation::Gaussian(observation_prior),
            observation,
            states,
            state_rng,
            variance_rng: Mt19937::seeded(seed.wrapping_add(1)),
        })
    }
    pub fn step(&mut self) -> Result<Draw, Error> {
        // Commit only after a complete Gibbs sweep, so a numerical failure
        // cannot leave parameters and states from different iterations.
        let mut terms = self.terms.clone();
        let mut variance_rng = self.variance_rng.clone();
        let mut state_rng = self.state_rng.clone();
        let mut stats = Statistics::default();
        let system = System::new(self.terms.iter().map(Term::component).collect())?;
        for (t, (value, state)) in self.y.iter().zip(&self.states).enumerate() {
            let z = system.observation_at(t)?;
            if let Some(value) = value {
                stats.add(value - z.dot(state))?;
            }
        }
        let mut observation_model = self.observation_model.clone();
        let (observation, regression, adjusted_y) = match &mut observation_model {
            Observation::Gaussian(prior) => (
                prior.conditional(stats)?.draw(&mut variance_rng)?,
                None,
                self.y.clone(),
            ),
            Observation::Regression {
                model,
                predictors,
                rows,
            } => {
                let residuals = DVector::from_vec(
                    rows.iter()
                        .map(|t| {
                            Ok(self.y[*t].expect("observed row")
                                - system.observation_at(*t)?.dot(&self.states[*t]))
                        })
                        .collect::<Result<Vec<_>, Error>>()?,
                );
                model.set_response(residuals)?;
                let draw = model.step()?;
                let predicted = model.predict(predictors)?;
                let adjusted = self
                    .y
                    .iter()
                    .zip(predicted.iter())
                    .map(|(y, x)| y.map(|y| y - x))
                    .collect::<Vec<_>>();
                (Variance::new(draw.variance)?, Some(draw), adjusted)
            }
        };
        let mut offset = 0;
        for term in &mut terms {
            term.sample(&self.states, offset, &mut variance_rng)?;
            offset += term.component().dimension();
        }
        let mut initial = self.initial.clone();
        let mut offset = 0;
        for term in &terms {
            if let Mechanism::Semilocal { slope, .. } = &term.0 {
                initial = initial.with_known_value(offset + 2, slope.parameters().mean())?;
            }
            offset += term.component().dimension();
        }
        let system =
            System::new(terms.iter().map(Term::component).collect())?.with_initial(initial)?;
        let states = gaussian::draw_states(&system, observation, &adjusted_y, &mut state_rng)?;
        let filtered = gaussian::filter(&system, observation, &adjusted_y)?;
        let draw = Draw {
            semilocal: terms
                .iter()
                .map(|term| match &term.0 {
                    Mechanism::Semilocal { slope, .. } => Some(slope.parameters()),
                    _ => None,
                })
                .collect(),
            dynamic_ar: terms
                .iter()
                .map(|term| match &term.0 {
                    Mechanism::DynamicAr { samplers, .. } => {
                        Some(samplers.iter().map(|s| s.snapshot()).collect())
                    }
                    _ => None,
                })
                .collect(),
            sparse_ar: terms
                .iter()
                .map(|term| match &term.0 {
                    Mechanism::SparseAr(sampler) => Some(sampler.snapshot()),
                    _ => None,
                })
                .collect(),
            ar: terms
                .iter()
                .map(|term| match &term.0 {
                    Mechanism::Ar(sampler) => Some(sampler.snapshot()),
                    _ => None,
                })
                .collect(),
            regression,
            observation_variance: observation.value(),
            component_variances: terms.iter().map(Term::variances).collect(),
            states: states.iter().map(|x| x.as_slice().to_vec()).collect(),
            log_likelihood: filtered.log_likelihood,
        };
        self.terms = terms;
        self.observation = observation;
        self.observation_model = observation_model;
        self.states = states;
        self.variance_rng = variance_rng;
        self.state_rng = state_rng;
        Ok(draw)
    }
    /// Seasonal time continues from the training series, not from zero.
    pub fn forecast(&self, horizon: usize, seed: u64) -> Result<Vec<f64>, Error> {
        match &self.observation_model {
            Observation::Gaussian(_) => self.forecast_offsets(&vec![0.0; horizon], seed),
            Observation::Regression { .. } => Err(Error::MissingPredictors),
        }
    }
    pub fn forecast_with_predictors(
        &self,
        predictors: &DMatrix<f64>,
        seed: u64,
    ) -> Result<Vec<f64>, Error> {
        match &self.observation_model {
            Observation::Regression { model, .. } => {
                self.forecast_offsets(model.predict(predictors)?.as_slice(), seed)
            }
            Observation::Gaussian(_) => Err(Error::Shape),
        }
    }
    fn forecast_offsets(&self, offsets: &[f64], seed: u64) -> Result<Vec<f64>, Error> {
        self.forecast_snapshot()?.forecast(offsets, seed)
    }
    pub fn forecast_snapshot(&self) -> Result<crate::forecast::Snapshot, Error> {
        crate::forecast::Snapshot::new(
            self.terms.iter().map(Term::component).collect(),
            self.observation,
            self.states.last().ok_or(Error::Empty)?.as_slice().to_vec(),
            self.y.len().try_into().map_err(|_| Error::Empty)?,
        )
    }
}
