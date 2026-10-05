//! DTO adapter for the core's sufficient DiD adjustment rules.
use super::*;
use hirmos_causal_core::swig::{did as core, AvailabilityBlocker};

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PanelRole {
    Confounder {},
    Disturbance { of: usize },
    Covariate { period: usize },
    Treatment { period: usize },
    Outcome { period: usize },
}
impl PanelRole {
    fn core(&self) -> core::PanelRole {
        match *self {
            Self::Confounder {} => core::PanelRole::Confounder,
            Self::Disturbance { of } => core::PanelRole::Disturbance { of: Variable(of) },
            Self::Covariate { period } => core::PanelRole::Covariate { period },
            Self::Treatment { period } => core::PanelRole::Treatment { period },
            Self::Outcome { period } => core::PanelRole::Outcome { period },
        }
    }
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum Comparison {
    NeverTreated {},
    NotYetTreated { through: usize },
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Assumptions {
    pub absorbing_binary_treatment: bool,
    pub consistency: bool,
    pub independent_disturbances: bool,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Design {
    pub panel_roles: Vec<PanelRole>,
    pub untreated: Vec<StructuralEquation>,
    pub alpha: Term,
    pub assumptions: Assumptions,
    pub adoption: usize,
    pub outcome: usize,
    pub comparison: Comparison,
    pub measured: Vec<usize>,
    pub selected: Vec<usize>,
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Blocker {
    Unmeasured {
        variable: usize,
    },
    UnestablishedAssignment {
        variable: usize,
        treatment: usize,
        required: f64,
    },
    ConflictingAssignment {
        variable: usize,
        treatment: usize,
        required: f64,
        actual: f64,
    },
}
impl From<AvailabilityBlocker> for Blocker {
    fn from(value: AvailabilityBlocker) -> Self {
        match value {
            AvailabilityBlocker::Unmeasured { variable } => Self::Unmeasured {
                variable: variable.0,
            },
            AvailabilityBlocker::UnestablishedAssignment {
                variable,
                treatment,
                required,
            } => Self::UnestablishedAssignment {
                variable: variable.0,
                treatment: treatment.0,
                required,
            },
            AvailabilityBlocker::ConflictingAssignment {
                variable,
                treatment,
                required,
                actual,
            } => Self::ConflictingAssignment {
                variable: variable.0,
                treatment: treatment.0,
                required,
                actual,
            },
        }
    }
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Decision {
    BaselineIdentity {},
    SupportedSubjectToOverlap {
        required: Vec<usize>,
        available: Vec<usize>,
        selected_supported: bool,
    },
    NotEstablished {
        potential_set: Vec<usize>,
        treated_blockers: Vec<Blocker>,
        comparison_blockers: Vec<Blocker>,
    },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Assessment {
    pub decision: Decision,
    pub no_within_period_treatment_covariate_effect: bool,
    pub no_future_treatment_covariate_effect: bool,
    pub no_direct_covariate_outcome_dynamics: bool,
    pub no_within_period_covariate_outcome_effect: bool,
}
fn problem(value: core::ModelError, names: &[String]) -> String {
    let name = |v: Variable| names[v.0].as_str();
    match value {
        core::ModelError::Graph(error) => super::describe(error),
        core::ModelError::MissingAssumption(name) => format!("This DiD rule requires an explicit assertion of {name}."),
        core::ModelError::InvalidPanel => "Classify every node, with one outcome at each period from 0, one treatment at each period from 1, and an explicit disturbance for every outcome.".into(),
        core::ModelError::InvalidRole(v) => format!("The role declared for {} is incompatible with its graph position.", name(v)),
        core::ModelError::BackwardTime { from, to } => format!("The arrow from {} to {} runs backwards in the declared periods.", name(from), name(to)),
        core::ModelError::OutcomeDynamics(v) => format!("The outcome {} has children. The supported DiD rules require no outcome dynamics (R-Y). This refusal does not establish nonidentification.", name(v)),
        core::ModelError::InvalidDisturbance(v) => format!("The disturbance {} must be exogenous and have only its declared variable as a child.", name(v)),
        core::ModelError::InvalidAdditiveSeparability(v) => format!("The equation for the outcome {} does not satisfy the declared common additive component (R-alpha).", name(v)),
        core::ModelError::InvalidComparison => "Choose an adoption period from 1 and a comparison group untreated through both the baseline and outcome periods.".into(),
    }
}
pub fn analyse(mut spec: Specification, design: Design, names: &[String]) -> Result<Evidence, String> {
    if !matches!(spec.query, Query::Graph {}) {
        return Err("A qualified DiD assessment is distinct from a separation query.".into());
    }
    let mut world: Vec<_> = design
        .panel_roles
        .iter()
        .enumerate()
        .filter_map(|(i, role)| matches!(role, PanelRole::Treatment { .. }).then_some((i, 0.0)))
        .collect();
    world.sort_by_key(|x| x.0);
    let mut supplied = spec.interventions.clone();
    supplied.sort_by_key(|x| x.0);
    if world != supplied {
        return Err("The DiD assessment uses the all-untreated intervention world.".into());
    }
    let dag = CausalDag::new(
        spec.roles
            .iter()
            .map(|r| match r {
                Role::Endogenous => swig::Role::Endogenous,
                Role::Exogenous => swig::Role::Exogenous,
            })
            .collect(),
        &spec
            .edges
            .iter()
            .map(|&(a, b)| (Variable(a), Variable(b)))
            .collect::<Vec<_>>(),
    )
    .map_err(super::describe)?;
    let model = core::StaggeredModel::new(
        dag,
        design.panel_roles.iter().map(PanelRole::core).collect(),
        design
            .untreated
            .iter()
            .cloned()
            .map(super::equation)
            .collect(),
        AdditiveTerm {
            identity: design.alpha.identity,
            arguments: design.alpha.arguments.into_iter().map(Variable).collect(),
        },
        core::Assumptions {
            absorbing_binary_treatment: design.assumptions.absorbing_binary_treatment,
            consistency: design.assumptions.consistency,
            independent_disturbances: design.assumptions.independent_disturbances,
        },
    )
    .map_err(|error| problem(error, names))?;
    let comparison = match design.comparison {
        Comparison::NeverTreated {} => core::Comparison::NeverTreated,
        Comparison::NotYetTreated { through } => core::Comparison::NotYetTreated { through },
    };
    let decision = match model
        .adjustment(
            design.adoption,
            design.outcome,
            comparison,
            &design
                .measured
                .iter()
                .copied()
                .map(Variable)
                .collect::<Vec<_>>(),
        )
        .map_err(|error| problem(error, names))?
    {
        core::AdjustmentDecision::BaselineIdentity => Decision::BaselineIdentity {},
        core::AdjustmentDecision::SupportedSubjectToOverlap(family) => {
            Decision::SupportedSubjectToOverlap {
                selected_supported: family
                    .supports(
                        &design
                            .selected
                            .iter()
                            .copied()
                            .map(Variable)
                            .collect::<Vec<_>>(),
                    )
                    .map_err(super::describe)?,
                required: family.required().iter().map(|v| v.0).collect(),
                available: family.available().iter().map(|v| v.0).collect(),
            }
        }
        core::AdjustmentDecision::NotEstablished {
            potential_set,
            treated_blockers,
            comparison_blockers,
        } => Decision::NotEstablished {
            potential_set: potential_set.iter().map(|v| v.0).collect(),
            treated_blockers: treated_blockers.into_iter().map(Blocker::from).collect(),
            comparison_blockers: comparison_blockers.into_iter().map(Blocker::from).collect(),
        },
    };
    let restrictions = model.restrictions();
    let mut equations = design.untreated;
    equations.sort_by_key(|e| match design.panel_roles[e.outcome] {
        PanelRole::Outcome { period } => period,
        _ => unreachable!(),
    });
    spec.construction = Construction::Differences {
        pairs: equations
            .windows(2)
            .map(|pair| Difference {
                earlier: pair[0].clone(),
                later: pair[1].clone(),
            })
            .collect(),
    };
    let mut evidence = super::analyse(spec, names)?;
    evidence.conclusion = Conclusion::Did {
        assessment: Assessment {
            decision,
            no_within_period_treatment_covariate_effect: restrictions
                .no_within_period_treatment_covariate_effect,
            no_future_treatment_covariate_effect: restrictions.no_future_treatment_covariate_effect,
            no_direct_covariate_outcome_dynamics: restrictions.no_direct_covariate_outcome_dynamics,
            no_within_period_covariate_outcome_effect: restrictions
                .no_within_period_covariate_outcome_effect,
        },
    };
    Ok(evidence)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Specification {
        serde_json::from_value(serde_json::json!({
   "roles":["endogenous","endogenous","endogenous","endogenous","exogenous","exogenous"],
   "edges":[[0,1],[0,2],[0,3],[1,3],[4,2],[5,3]],"interventions":[[1,0]],
   "construction":{"kind":"did","design":{
    "panelRoles":[{"kind":"confounder"},{"kind":"treatment","period":1},{"kind":"outcome","period":0},{"kind":"outcome","period":1},{"kind":"disturbance","of":2},{"kind":"disturbance","of":3}],
    "untreated":[{"outcome":2,"terms":[{"identity":"alpha","arguments":[0]},{"identity":"g0","arguments":[4]}]},{"outcome":3,"terms":[{"identity":"alpha","arguments":[0]},{"identity":"g1","arguments":[5]}]}],
    "alpha":{"identity":"alpha","arguments":[0]},
    "assumptions":{"absorbingBinaryTreatment":true,"consistency":true,"independentDisturbances":true},
    "adoption":1,"outcome":1,"comparison":{"kind":"neverTreated"},"measured":[1,2,3],"selected":[]
   }},"query":{"kind":"graph"}
  })).unwrap()
    }
    #[test]
    fn assessment_and_browser_serialization_match_core_rule() {
        let result = super::super::tests::analyse(fixture()).unwrap();
        let json = serde_json::to_value(result).unwrap();
        assert_eq!(
            json["conclusion"]["assessment"]["decision"]["kind"],
            "supportedSubjectToOverlap"
        );
        assert_eq!(
            json["conclusion"]["assessment"]["decision"]["selectedSupported"],
            true
        );
        assert_eq!(json["nodes"][7]["cancelled"], serde_json::json!(["alpha"]));
    }
    #[test]
    fn baseline_is_identity_not_parallel_trends_evidence() {
        let mut spec = fixture();
        if let Construction::Did { design } = &mut spec.construction {
            design.outcome = 0;
        }
        let json = serde_json::to_value(super::super::tests::analyse(spec).unwrap()).unwrap();
        assert_eq!(
            json["conclusion"]["assessment"]["decision"]["kind"],
            "baselineIdentity"
        );
    }
    #[test]
    fn outcome_feedback_and_missing_assumptions_are_refused() {
        let mut spec = fixture();
        spec.edges.push((2, 3));
        assert!(super::super::tests::analyse(spec).is_err());
        let mut spec = fixture();
        if let Construction::Did { design } = &mut spec.construction {
            design.assumptions.consistency = false;
        }
        assert!(super::super::tests::analyse(spec).is_err());
    }
}
