//! Graph-only analysis. No dataset, fitted model, or simulation convention enters this route.
use hirmos_causal_core::swig::{self, AdditiveTerm, CausalDag, Equation, Node, Variable};
use serde::{Deserialize, Serialize};
mod did;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Difference {
    pub earlier: StructuralEquation,
    pub later: StructuralEquation,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Endogenous,
    Exogenous,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Term {
    pub identity: String,
    pub arguments: Vec<usize>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StructuralEquation {
    pub outcome: usize,
    pub terms: Vec<Term>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum Construction {
    Swig {},
    Differences {
        pairs: Vec<Difference>,
    },
    Did {
        design: did::Design,
    },
    Difference {
        earlier: StructuralEquation,
        later: StructuralEquation,
    },
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum Query {
    Graph {},
    Separation {
        left: usize,
        right: usize,
        given: Vec<usize>,
    },
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Specification {
    pub roles: Vec<Role>,
    pub edges: Vec<(usize, usize)>,
    pub interventions: Vec<(usize, f64)>,
    pub construction: Construction,
    pub query: Query,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DerivedNode {
    Random {
        original: usize,
        role: Role,
        interventions: Vec<usize>,
    },
    Fixed {
        original: usize,
        value: f64,
    },
    Difference {
        earlier: usize,
        later: usize,
        cancelled: Vec<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Conclusion {
    NotRequested,
    Did {
        assessment: did::Assessment,
    },
    Separated {
        left: usize,
        right: usize,
        given: Vec<usize>,
    },
    Connected {
        left: usize,
        right: usize,
        given: Vec<usize>,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub nodes: Vec<DerivedNode>,
    pub edges: Vec<(usize, usize)>,
    pub conclusion: Conclusion,
}

fn describe(error: swig::Error) -> String {
    match error {
        swig::Error::EmptyGraph => "Choose a DAG containing at least one variable.",
        swig::Error::UnknownVariable => "The specification refers to a variable outside this DAG revision.",
        swig::Error::DuplicateEdge => "The graph contains a repeated arrow.",
        swig::Error::Cycle => "SWIG construction requires an acyclic directed graph.",
        swig::Error::ExogenousHasParents => "A variable marked as an external disturbance cannot have parents in the graph.",
        swig::Error::DuplicateIntervention => "Specify each intervention variable only once.",
        swig::Error::InvalidIntervention => "Intervene on an endogenous variable using a finite value.",
        swig::Error::InvalidQuery => "Choose distinct random nodes and a conditioning set that excludes both query nodes and all fixed nodes.",
        swig::Error::InvalidEquation => "Each outcome equation must cover all its random parents, including disturbances, with distinct named additive terms.",
        swig::Error::ConflictingMechanism => "A shared term must use the same function and the same ordered arguments in both equations.",
        swig::Error::WrongInterventionWorld => "Both outcome equations must describe the selected intervention world.",
    }.into()
}

fn equation(value: StructuralEquation) -> Equation {
    Equation {
        outcome: Variable(value.outcome),
        terms: value
            .terms
            .into_iter()
            .map(|term| AdditiveTerm {
                identity: term.identity,
                arguments: term.arguments.into_iter().map(Variable).collect(),
            })
            .collect(),
    }
}

/// Analyses a specification; refusals name variables using `names`, one per specification role.
pub fn analyse(specification: Specification, names: &[String]) -> Result<Evidence, String> {
    // Bound graph work at the browser boundary, not inside the reusable kernel.
    if specification.roles.len() > 256 || specification.edges.len() > 65_536 {
        return Err("Graph analysis supports up to 256 explicit variables.".into());
    }
    if names.len() != specification.roles.len() {
        return Err("Name every variable in the specification.".into());
    }
    if let Construction::Did { design } = &specification.construction {
        return did::analyse(specification.clone(), design.clone(), names);
    }
    let roles = specification
        .roles
        .iter()
        .map(|role| match role {
            Role::Endogenous => swig::Role::Endogenous,
            Role::Exogenous => swig::Role::Exogenous,
        })
        .collect();
    let edges = specification
        .edges
        .iter()
        .map(|&(a, b)| (Variable(a), Variable(b)))
        .collect::<Vec<_>>();
    let interventions = specification
        .interventions
        .iter()
        .map(|&(v, x)| (Variable(v), x))
        .collect::<Vec<_>>();
    if interventions.is_empty() {
        return Err("Specify at least one intervention for this single-world graph.".into());
    }
    let dag = CausalDag::new(roles, &edges).map_err(describe)?;
    let mut graph = dag.intervene(&interventions).map_err(describe)?;
    match specification.construction {
        Construction::Swig {} => {}
        Construction::Did { .. } => unreachable!("DiD dispatch precedes graph construction"),
        Construction::Differences { pairs } => {
            if pairs.is_empty() || pairs.len() > 32 {
                return Err("Specify between one and 32 outcome differences.".into());
            }
            for pair in pairs {
                graph
                    .add_difference(equation(pair.later), equation(pair.earlier), &interventions)
                    .map_err(describe)?;
            }
        }
        Construction::Difference { earlier, later } => {
            graph
                .add_difference(equation(later), equation(earlier), &interventions)
                .map_err(describe)?;
        }
    }
    let conclusion = match specification.query {
        Query::Graph {} => Conclusion::NotRequested,
        Query::Separation { left, right, given } => {
            if graph
                .separated_nodes(left, right, &given)
                .map_err(describe)?
            {
                Conclusion::Separated { left, right, given }
            } else {
                Conclusion::Connected { left, right, given }
            }
        }
    };
    let nodes = graph
        .nodes()
        .iter()
        .map(|node| match node {
            Node::Random {
                original,
                role,
                interventions,
            } => DerivedNode::Random {
                original: original.0,
                role: match role {
                    swig::Role::Endogenous => Role::Endogenous,
                    swig::Role::Exogenous => Role::Exogenous,
                },
                interventions: interventions.iter().map(|v| v.0).collect(),
            },
            Node::Fixed { original, value } => DerivedNode::Fixed {
                original: original.0,
                value: *value,
            },
            Node::Difference {
                earlier,
                later,
                cancelled_terms,
            } => DerivedNode::Difference {
                earlier: earlier.0,
                later: later.0,
                cancelled: cancelled_terms.clone(),
            },
        })
        .collect();
    Ok(Evidence {
        nodes,
        edges: graph.edges(),
        conclusion,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Analyses a fixture whose variables are named by position.
    pub(crate) fn analyse(specification: Specification) -> Result<Evidence, String> {
        let names: Vec<String> =
            (1..=specification.roles.len()).map(|position| format!("V{position}")).collect();
        super::analyse(specification, &names)
    }

    #[test]
    fn randomized_notebook_splits_assignment_and_intervention() {
        let result = analyse(Specification {
            roles: vec![Role::Endogenous, Role::Endogenous],
            edges: vec![(0, 1)],
            interventions: vec![(0, 1.)],
            construction: Construction::Swig {},
            query: Query::Separation {
                left: 0,
                right: 1,
                given: vec![],
            },
        })
        .unwrap();
        assert!(matches!(result.conclusion, Conclusion::Separated { .. }));
        assert_eq!(result.edges, vec![(2, 1)]);
        assert!(matches!(
            result.nodes[2],
            DerivedNode::Fixed {
                original: 0,
                value: 1.
            }
        ));
    }
    #[test]
    fn confounding_does_not_disappear_after_intervention() {
        let result = analyse(Specification {
            roles: vec![Role::Endogenous, Role::Endogenous, Role::Exogenous],
            edges: vec![(2, 0), (2, 1), (0, 1)],
            interventions: vec![(0, 1.)],
            construction: Construction::Swig {},
            query: Query::Separation {
                left: 0,
                right: 1,
                given: vec![],
            },
        })
        .unwrap();
        assert!(matches!(result.conclusion, Conclusion::Connected { .. }));
    }
    #[test]
    fn malformed_graph_and_unknown_fields_are_refused() {
        assert!(serde_json::from_value::<Specification>(serde_json::json!({
            "roles":["endogenous"],"edges":[],"interventions":[[0,1]],
            "construction":{"kind":"swig","earlier":0},"query":{"kind":"graph"}
        }))
        .is_err());
    }

    #[test]
    fn explicit_shared_term_cancels_but_disturbances_remain() {
        let specification: Specification = serde_json::from_value(serde_json::json!({
            "roles":["exogenous","endogenous","endogenous","endogenous","exogenous","exogenous"],
            "edges":[[0,1],[0,2],[0,3],[1,3],[4,2],[5,3]],
            "interventions":[[1,0]],
            "construction":{"kind":"difference",
                "earlier":{"outcome":2,"terms":[{"identity":"unit","arguments":[0]},{"identity":"noise0","arguments":[4]}]},
                "later":{"outcome":3,"terms":[{"identity":"unit","arguments":[0]},{"identity":"noise1","arguments":[5]}]}},
            "query":{"kind":"separation","left":1,"right":7,"given":[]}
        })).unwrap();
        let result = analyse(specification).unwrap();
        assert!(matches!(result.conclusion, Conclusion::Separated { .. }));
        assert!(result.edges.contains(&(4, 7)) && result.edges.contains(&(5, 7)));
        assert!(!result.edges.contains(&(0, 7)));
        assert!(
            matches!(&result.nodes[7], DerivedNode::Difference { cancelled, .. } if cancelled == &vec!["unit".to_owned()])
        );
    }

    #[test]
    fn matching_parents_alone_do_not_cancel() {
        let specification: Specification = serde_json::from_value(serde_json::json!({
            "roles":["exogenous","endogenous","endogenous","endogenous"],
            "edges":[[0,1],[0,2],[0,3],[1,3]],"interventions":[[1,0]],
            "construction":{"kind":"difference",
                "earlier":{"outcome":2,"terms":[{"identity":"earlier","arguments":[0]}]},
                "later":{"outcome":3,"terms":[{"identity":"later","arguments":[0]}]}},
            "query":{"kind":"separation","left":1,"right":5,"given":[]}
        }))
        .unwrap();
        let result = analyse(specification).unwrap();
        assert!(matches!(result.conclusion, Conclusion::Connected { .. }));
        assert!(result.edges.contains(&(0, 5)));
    }
}
