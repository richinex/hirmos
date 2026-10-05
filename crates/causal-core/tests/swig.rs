use hirmos_causal_core::swig::{CausalDag, Error, Node, Role::*, Variable as V};

#[test]
fn confounded_training_example_splits_assignment_from_intervention() {
    let g = CausalDag::new(
        vec![Endogenous; 3],
        &[(V(0), V(1)), (V(0), V(2)), (V(1), V(2))],
    )
    .unwrap();
    let s = g.intervene(&[(V(1), 1.0)]).unwrap();
    assert_eq!(s.edges(), vec![(0, 1), (0, 2), (3, 2)]);
    assert!(!s.separated(V(1), V(2), &[]).unwrap());
    assert!(s.separated(V(1), V(2), &[V(0)]).unwrap());
    assert_eq!(
        s.nodes()[2],
        Node::Random {
            original: V(2),
            role: Endogenous,
            interventions: vec![V(1)]
        }
    );
    assert_eq!(
        s.nodes()[1],
        Node::Random {
            original: V(1),
            role: Endogenous,
            interventions: vec![]
        }
    );
}

#[test]
fn intervention_on_mediator_blocks_upstream_label_propagation() {
    let g = CausalDag::new(vec![Endogenous; 3], &[(V(0), V(1)), (V(1), V(2))]).unwrap();
    let s = g.intervene(&[(V(1), 2.0), (V(0), 1.0)]).unwrap();
    assert_eq!(
        s.nodes()[1],
        Node::Random {
            original: V(1),
            role: Endogenous,
            interventions: vec![V(0)]
        }
    );
    assert_eq!(
        s.nodes()[2],
        Node::Random {
            original: V(2),
            role: Endogenous,
            interventions: vec![V(1)]
        }
    );
    assert!(s.separated(V(0), V(2), &[]).unwrap());
}

#[test]
fn shared_fixed_parent_does_not_create_random_dependence() {
    let g = CausalDag::new(vec![Endogenous; 3], &[(V(0), V(1)), (V(0), V(2))]).unwrap();
    let s = g.intervene(&[(V(0), 0.0)]).unwrap();
    assert!(s.separated(V(1), V(2), &[]).unwrap());
    let observational = g.intervene(&[]).unwrap();
    assert!(!observational.separated(V(1), V(2), &[]).unwrap());
}

#[test]
fn malformed_graphs_interventions_and_queries_are_rejected() {
    assert!(matches!(
        CausalDag::new(vec![], &[]),
        Err(Error::EmptyGraph)
    ));
    assert!(matches!(
        CausalDag::new(vec![Endogenous; 2], &[(V(0), V(1)), (V(1), V(0))]),
        Err(Error::Cycle)
    ));
    assert!(matches!(
        CausalDag::new(vec![Endogenous], &[(V(0), V(1))]),
        Err(Error::UnknownVariable)
    ));
    assert!(matches!(
        CausalDag::new(vec![Endogenous; 2], &[(V(0), V(1)), (V(0), V(1))]),
        Err(Error::DuplicateEdge)
    ));
    assert!(matches!(
        CausalDag::new(vec![Endogenous, Exogenous], &[(V(0), V(1))]),
        Err(Error::ExogenousHasParents)
    ));
    let g = CausalDag::new(vec![Exogenous, Endogenous], &[(V(0), V(1))]).unwrap();
    assert!(matches!(
        g.intervene(&[(V(0), 1.0)]),
        Err(Error::InvalidIntervention)
    ));
    assert!(matches!(
        g.intervene(&[(V(1), f64::NAN)]),
        Err(Error::InvalidIntervention)
    ));
    assert!(matches!(
        g.intervene(&[(V(1), 0.0), (V(1), 1.0)]),
        Err(Error::DuplicateIntervention)
    ));
    let s = g.intervene(&[]).unwrap();
    assert_eq!(s.separated(V(0), V(1), &[V(0)]), Err(Error::InvalidQuery));
}
