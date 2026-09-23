//! R-specific constructor rules, distinct from general state equations.
//! bsts 0.9.11 create_state_model.cpp / set_initial_state_prior.
//! Copyright Google LLC; LGPL-2.1-or-later sources preserved in vendor.
use crate::{
    fit::{Chain, Term},
    initial::Initial,
    prior::{Prior, Update},
    state::{Normal, Season},
    Error,
};
use nalgebra::{DMatrix, DVector};

pub enum SeasonalPrior {
    /// R transfers the scalar variance but leaves the seasonal mean at zero.
    Shared(Normal),
    /// MvnPrior and MvnDiagonalPrior transfer their actual vector means.
    Vector(Initial),
}

/// A term bound to its dimension-checked initial prior. Composition retains
/// independence between components, as in the R state-model factory.
pub struct Component {
    term: Term,
    initial: Initial,
}
impl Component {
    pub fn seasonal(
        season: Season,
        innovation: Update,
        prior: SeasonalPrior,
    ) -> Result<Self, Error> {
        let n = season.dimension();
        let initial = match prior {
            SeasonalPrior::Shared(prior) => Initial::new(
                DVector::zeros(n),
                DMatrix::identity(n, n) * prior.variance().value(),
            )?,
            SeasonalPrior::Vector(prior) => prior,
        };
        if initial.dimension() != n {
            return Err(Error::Shape);
        }
        let placeholder = Normal::new(0., crate::state::Variance::new(0.)?)?;
        Ok(Self {
            term: Term::seasonal(season, innovation, placeholder),
            initial,
        })
    }
    pub fn initial(&self) -> &Initial {
        &self.initial
    }
}

pub fn chain(
    components: Vec<Component>,
    observation: Prior,
    y: Vec<Option<f64>>,
    seed: u32,
) -> Result<Chain, Error> {
    let n = components
        .iter()
        .try_fold(0usize, |n, c| n.checked_add(c.initial.dimension()))
        .ok_or(Error::Shape)?;
    n.checked_mul(n).ok_or(Error::Shape)?;
    let mut mean = DVector::zeros(n);
    let mut covariance = DMatrix::zeros(n, n);
    let mut offset = 0;
    let mut terms = Vec::with_capacity(components.len());
    for component in components {
        let (m, c) = component.initial.moments();
        let d = m.len();
        mean.rows_mut(offset, d).copy_from(&m);
        covariance.view_mut((offset, offset), (d, d)).copy_from(&c);
        offset += d;
        terms.push(component.term);
    }
    Chain::with_initial(terms, observation, y, seed, Initial::new(mean, covariance)?)
}
