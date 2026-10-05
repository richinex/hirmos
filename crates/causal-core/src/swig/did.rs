//! Knaus & Pfleiderer (2026), Propositions 1--4. These are sufficient
//! model-class adjustment rules, not a general identification algorithm.
//! A graph cannot verify consistency, positivity, or an asserted SCM function.
use super::{
    AdditiveTerm, AvailabilityBlocker, CausalDag, ConditioningAvailability, Equation, Error, Role,
    Swig, Variable,
};
use std::collections::{BTreeMap, BTreeSet};

/// Every node is classified exactly once. Period zero is untreated baseline;
/// treatments exist in periods 1..=T. Multiple covariates per period are allowed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelRole {
    Confounder,
    Disturbance { of: Variable },
    Covariate { period: usize },
    Treatment { period: usize },
    Outcome { period: usize },
}
impl PanelRole {
    fn period(self) -> Option<usize> {
        match self {
            Self::Covariate { period } | Self::Treatment { period } | Self::Outcome { period } => {
                Some(period)
            }
            _ => None,
        }
    }
}

/// Assertions about mechanisms and probability semantics, not empirical findings.
/// R-alpha's functional part is checked separately against declared equations.
#[derive(Clone, Debug)]
pub struct Assumptions {
    pub absorbing_binary_treatment: bool,
    pub consistency: bool,
    pub independent_disturbances: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelError {
    Graph(Error),
    MissingAssumption(&'static str),
    InvalidPanel,
    InvalidRole(Variable),
    BackwardTime { from: Variable, to: Variable },
    OutcomeDynamics(Variable),
    InvalidDisturbance(Variable),
    InvalidAdditiveSeparability(Variable),
    InvalidComparison,
}
impl From<Error> for ModelError {
    fn from(error: Error) -> Self {
        Self::Graph(error)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    NeverTreated,
    /// Untreated through this period, through >= max(g-1,t), as in (E.16).
    /// When through=g-1, this is the at-risk mixture for a pre-trend query;
    /// the target cohort is also part of that comparison population.
    NotYetTreated {
        through: usize,
    },
}

/// Structural restrictions verified on the DAG, not supplied as checkboxes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Restrictions {
    pub no_within_period_treatment_covariate_effect: bool,
    pub no_future_treatment_covariate_effect: bool,
    pub no_direct_covariate_outcome_dynamics: bool,
    pub no_within_period_covariate_outcome_effect: bool,
}

pub struct StaggeredModel {
    swig: Swig,
    outcomes: Vec<Variable>,
    treatments: Vec<Variable>,
    covariates: BTreeSet<Variable>,
    restrictions: Restrictions,
}

/// Results stay qualified: positivity still has to hold on the chosen observed
/// covariates. Failure of this rule does not rule out other identification methods.
#[derive(Clone, Debug, PartialEq)]
pub enum AdjustmentDecision {
    /// The comparison is Y_(g-1) minus itself, identically zero.
    BaselineIdentity,
    SupportedSubjectToOverlap(AdjustmentFamily),
    NotEstablished {
        potential_set: Vec<Variable>,
        treated_blockers: Vec<AvailabilityBlocker>,
        comparison_blockers: Vec<AvailabilityBlocker>,
    },
}

/// Private fields preserve the checked rule's bounds. The minimum is the paper's
/// parent-union requirement, not necessarily the smallest separator for a more
/// restricted graph. Optional members are OBSERVED columns, not their potentials.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdjustmentFamily {
    required: BTreeSet<Variable>,
    available: BTreeSet<Variable>,
    comparison: Comparison,
}
impl AdjustmentFamily {
    pub fn required(&self) -> Vec<Variable> {
        self.required.iter().copied().collect()
    }
    pub fn available(&self) -> Vec<Variable> {
        self.available.iter().copied().collect()
    }
    pub fn comparison(&self) -> Comparison {
        self.comparison
    }
    /// Implements the subset bounds in (39)/(E.17), without exponential listing.
    /// False means not justified by this family, not universally invalid.
    pub fn supports(&self, selected: &[Variable]) -> Result<bool, Error> {
        let count = selected.len();
        let selected: BTreeSet<_> = selected.iter().copied().collect();
        if selected.len() != count {
            return Err(Error::InvalidQuery);
        }
        Ok(self.required.is_subset(&selected) && selected.is_subset(&self.available))
    }
}

impl StaggeredModel {
    /// The untreated equations declare alpha(U,C)+g_t(...,UY_t) in the all-zero
    /// world. Exact equation coverage and mechanism identities use the same
    /// validator as ordinary delta construction. No guessed cancellation.
    pub fn new(
        dag: CausalDag,
        roles: Vec<PanelRole>,
        untreated: Vec<Equation>,
        alpha: AdditiveTerm,
        assumptions: Assumptions,
    ) -> Result<Self, ModelError> {
        for (holds, name) in [
            (
                assumptions.absorbing_binary_treatment,
                "ST: untreated baseline and absorbing binary treatment",
            ),
            (assumptions.consistency, "consistency"),
            (
                assumptions.independent_disturbances,
                "independent exogenous disturbances",
            ),
        ] {
            if !holds {
                return Err(ModelError::MissingAssumption(name));
            }
        }
        if roles.len() != dag.graph.n {
            return Err(ModelError::InvalidPanel);
        }
        let mut outcomes = BTreeMap::new();
        let mut treatments = BTreeMap::new();
        let mut covariates = BTreeSet::new();
        let mut confounders = BTreeSet::new();
        let mut noise = BTreeMap::new();
        for (i, &role) in roles.iter().enumerate() {
            let variable = Variable(i);
            match role {
                PanelRole::Outcome { period } => {
                    if outcomes.insert(period, variable).is_some() {
                        return Err(ModelError::InvalidPanel);
                    }
                    if !dag.graph.children[i].is_empty() {
                        return Err(ModelError::OutcomeDynamics(variable));
                    }
                }
                PanelRole::Treatment { period } => {
                    if period == 0 || treatments.insert(period, variable).is_some() {
                        return Err(ModelError::InvalidPanel);
                    }
                }
                PanelRole::Covariate { .. } => {
                    covariates.insert(variable);
                }
                PanelRole::Confounder => {
                    confounders.insert(variable);
                }
                PanelRole::Disturbance { of } => {
                    if of.0 >= roles.len()
                        || matches!(roles[of.0], PanelRole::Disturbance { .. })
                        || dag.roles[i] != Role::Exogenous
                        || dag.graph.children[i] != vec![of.0]
                        || noise.insert(of, variable).is_some()
                    {
                        return Err(ModelError::InvalidDisturbance(variable));
                    }
                }
            }
            if role.period().is_some() && dag.roles[i] != Role::Endogenous {
                return Err(ModelError::InvalidRole(variable));
            }
            for &p in &dag.graph.parents[i] {
                if let (Some(from), Some(to)) = (roles[p].period(), role.period()) {
                    if from > to {
                        return Err(ModelError::BackwardTime {
                            from: Variable(p),
                            to: variable,
                        });
                    }
                }
                if matches!(role, PanelRole::Confounder)
                    && !matches!(
                        roles[p],
                        PanelRole::Confounder | PanelRole::Disturbance { .. }
                    )
                {
                    return Err(ModelError::InvalidRole(variable));
                }
            }
        }
        let periods = outcomes.len();
        if periods < 2
            || outcomes.keys().copied().ne(0..periods)
            || treatments.keys().copied().ne(1..periods)
            || roles
                .iter()
                .filter_map(|r| r.period())
                .any(|t| t >= periods)
        {
            return Err(ModelError::InvalidPanel);
        }
        let outcomes: Vec<_> = outcomes.into_values().collect();
        let treatments: Vec<_> = treatments.into_values().collect();
        if outcomes.iter().any(|y| !noise.contains_key(y)) {
            return Err(ModelError::InvalidPanel);
        }
        let mut common = covariates.clone();
        for &y in &outcomes {
            common.retain(|v| dag.graph.parents[y.0].contains(&v.0));
        }
        common.extend(confounders.iter().copied());
        if alpha.identity.trim().is_empty()
            || alpha.arguments.iter().copied().collect::<BTreeSet<_>>() != common
            || alpha.arguments.len() != common.len()
        {
            return Err(ModelError::InvalidAdditiveSeparability(outcomes[0]));
        }
        let mut equations = BTreeMap::new();
        for equation in untreated {
            let y = equation.outcome;
            if !outcomes.contains(&y) || equations.insert(y, equation).is_some() {
                return Err(ModelError::InvalidPanel);
            }
        }
        if equations.len() != periods {
            return Err(ModelError::InvalidPanel);
        }
        for (&y, equation) in &equations {
            if equation.terms.iter().filter(|term| *term == &alpha).count() != 1
                || equation
                    .terms
                    .iter()
                    .filter(|term| term.identity != alpha.identity)
                    .any(|term| term.arguments.iter().any(|a| confounders.contains(a)))
            {
                return Err(ModelError::InvalidAdditiveSeparability(y));
            }
        }
        let mut restrictions = Restrictions {
            no_within_period_treatment_covariate_effect: true,
            no_future_treatment_covariate_effect: true,
            no_direct_covariate_outcome_dynamics: true,
            no_within_period_covariate_outcome_effect: true,
        };
        for &d in &treatments {
            let t = roles[d.0].period().unwrap();
            for child in dag.graph.descendants(d.0) {
                if let PanelRole::Covariate { period } = roles[child] {
                    if period == t {
                        restrictions.no_within_period_treatment_covariate_effect = false;
                    }
                    if period > t {
                        restrictions.no_future_treatment_covariate_effect = false;
                    }
                }
            }
        }
        for &x in &covariates {
            let t = roles[x.0].period().unwrap();
            for &child in &dag.graph.children[x.0] {
                if let PanelRole::Outcome { period } = roles[child] {
                    if period == t {
                        restrictions.no_within_period_covariate_outcome_effect = false;
                    }
                    if period > t {
                        restrictions.no_direct_covariate_outcome_dynamics = false;
                    }
                }
            }
        }
        let world: Vec<_> = treatments.iter().map(|&d| (d, 0.0)).collect();
        let mut swig = dag.intervene(&world)?;
        for pair in outcomes.windows(2) {
            swig.add_difference(
                equations[&pair[1]].clone(),
                equations[&pair[0]].clone(),
                &world,
            )?;
        }
        Ok(Self {
            swig,
            outcomes,
            treatments,
            covariates,
            restrictions,
        })
    }

    pub fn restrictions(&self) -> &Restrictions {
        &self.restrictions
    }

    pub fn adjustment(
        &self,
        adoption: usize,
        outcome: usize,
        comparison: Comparison,
        measured: &[Variable],
    ) -> Result<AdjustmentDecision, ModelError> {
        let last = self.outcomes.len() - 1;
        if adoption == 0 || adoption > last || outcome > last {
            return Err(ModelError::InvalidComparison);
        }
        let through = match comparison {
            Comparison::NeverTreated => last,
            Comparison::NotYetTreated { through }
                if through >= (adoption - 1).max(outcome) && through <= last =>
            {
                through
            }
            _ => return Err(ModelError::InvalidComparison),
        };
        // Validate even an empty/identity query's supplied measurement inventory.
        self.swig.conditioning_availability(&[], measured, &[])?;
        if outcome == adoption - 1 {
            return Ok(AdjustmentDecision::BaselineIdentity);
        }
        let required: BTreeSet<_> = [self.outcomes[adoption - 1], self.outcomes[outcome]]
            .iter()
            .flat_map(|v| self.swig.graph.parents[v.0].iter().copied().map(Variable))
            .filter(|v| self.covariates.contains(v))
            .collect();
        let potential_set: Vec<_> = required.iter().copied().collect();
        let treated_history: Vec<_> = self
            .treatments
            .iter()
            .enumerate()
            .map(|(t, &d)| (d, if t + 1 < adoption { 0.0 } else { 1.0 }))
            .collect();
        let comparison_history: Vec<_> = self
            .treatments
            .iter()
            .take(through)
            .map(|&d| (d, 0.0))
            .collect();
        let blockers = |history: &[(Variable, f64)]| -> Result<Vec<AvailabilityBlocker>, Error> {
            Ok(
                match self
                    .swig
                    .conditioning_availability(&potential_set, measured, history)?
                {
                    ConditioningAvailability::Available => vec![],
                    ConditioningAvailability::Unavailable { blockers } => blockers,
                },
            )
        };
        let treated_blockers = blockers(&treated_history)?;
        let comparison_blockers = blockers(&comparison_history)?;
        if !treated_blockers.is_empty() || !comparison_blockers.is_empty() {
            return Ok(AdjustmentDecision::NotEstablished {
                potential_set,
                treated_blockers,
                comparison_blockers,
            });
        }
        // Proposition 2/4 allows OBSERVED optional covariates, even if their
        // all-zero-world versions differ in treated units. Do not apply the
        // minimum-set consistency gate to those optional observed columns.
        let available = self
            .covariates
            .intersection(&measured.iter().copied().collect())
            .copied()
            .collect();
        Ok(AdjustmentDecision::SupportedSubjectToOverlap(
            AdjustmentFamily {
                required,
                available,
                comparison,
            },
        ))
    }
}
