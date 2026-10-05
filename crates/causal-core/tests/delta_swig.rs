use hirmos_causal_core::swig::{
    AdditiveTerm, CausalDag, Equation, Error, Node, Role::*, Swig, Variable as V,
};

// Paper Section 2.5 equations (1'), (2') and Figure 4:
// U=0, X=1, D=2, Y0=3, Y1=4, UY0=5, UY1=6.
fn figure4(value: f64) -> Swig {
    CausalDag::new(
        vec![
            Exogenous, Endogenous, Endogenous, Endogenous, Endogenous, Exogenous, Exogenous,
        ],
        &[
            (V(0), V(1)),
            (V(0), V(2)),
            (V(1), V(2)),
            (V(0), V(3)),
            (V(1), V(3)),
            (V(5), V(3)),
            (V(0), V(4)),
            (V(1), V(4)),
            (V(2), V(4)),
            (V(6), V(4)),
        ],
    )
    .unwrap()
    .intervene(&[(V(2), value)])
    .unwrap()
}
fn term(name: &str, args: &[usize]) -> AdditiveTerm {
    AdditiveTerm {
        identity: name.into(),
        arguments: args.iter().copied().map(V).collect(),
    }
}
fn equations() -> (Equation, Equation) {
    (
        Equation {
            outcome: V(4),
            terms: vec![term("alpha", &[0, 1]), term("g1", &[1, 6])],
        },
        Equation {
            outcome: V(3),
            terms: vec![term("alpha", &[0, 1]), term("g0", &[1, 5])],
        },
    )
}

#[test]
fn figure4_cancels_unit_effect_not_covariates_or_disturbances() {
    let mut s = figure4(0.0);
    let (later, earlier) = equations();
    let delta = s.add_difference(later, earlier, &[(V(2), 0.0)]).unwrap();
    let parents: Vec<_> = s
        .edges()
        .iter()
        .filter(|(_, b)| *b == delta)
        .map(|(a, _)| *a)
        .collect();
    assert_eq!(parents, vec![1, 5, 6]);
    assert!(!s.separated_nodes(2, delta, &[]).unwrap());
    assert!(s.separated_nodes(2, delta, &[1]).unwrap());
    // Differencing, not an ordinary back-door adjustment on outcome levels.
    assert!(!s.separated(V(2), V(4), &[V(1)]).unwrap());
    assert!(s.separated(V(2), V(4), &[V(0), V(1)]).unwrap());
    assert!(
        matches!(&s.nodes()[delta],Node::Difference {cancelled_terms,..} if cancelled_terms==&vec!["alpha".to_owned()])
    );
}

#[test]
fn figure4_pruning_preserves_query_and_conditioning_variables() {
    let mut s = figure4(0.0);
    let (later, earlier) = equations();
    let delta = s.add_difference(later, earlier, &[(V(2), 0.0)]).unwrap();
    let pruned = s.prune_for(&[2, delta, 1]).unwrap();
    assert!(!pruned.retained_nodes().contains(&3));
    assert!(!pruned.retained_nodes().contains(&4));
    assert!(pruned.retained_nodes().contains(&5));
    assert!(pruned.retained_nodes().contains(&6));
    for given in [vec![], vec![1], vec![0], vec![0, 1]] {
        assert_eq!(
            s.separated_nodes(2, delta, &given),
            pruned.separated_nodes(2, delta, &given)
        );
    }
    assert_eq!(
        pruned.separated_nodes(2, delta, &[3]),
        Err(Error::UnknownVariable)
    );
    let protected = s.prune_for(&[2, delta, 3]).unwrap();
    assert!(protected.retained_nodes().contains(&3));
    assert_eq!(
        s.separated_nodes(2, delta, &[3]),
        protected.separated_nodes(2, delta, &[3])
    );
}

#[test]
fn identical_parent_sets_do_not_imply_identical_functions() {
    let mut s = figure4(0.0);
    let (mut later, earlier) = equations();
    later.terms[0].identity = "time-varying-alpha".into();
    let delta = s.add_difference(later, earlier, &[(V(2), 0.0)]).unwrap();
    assert!(s.edges().contains(&(0, delta)));
    assert!(!s.separated_nodes(2, delta, &[1]).unwrap());
}

#[test]
fn unit_effect_used_in_remaining_term_cannot_disappear() {
    let mut s = figure4(0.0);
    let (mut later, earlier) = equations();
    later.terms[1].arguments.push(V(0));
    let delta = s.add_difference(later, earlier, &[(V(2), 0.0)]).unwrap();
    assert!(s.edges().contains(&(0, delta)));
    assert!(!s.separated_nodes(2, delta, &[1]).unwrap());
}

#[test]
fn assumptions_cannot_be_reused_in_another_intervention_world() {
    let mut s = figure4(1.0);
    let (later, earlier) = equations();
    assert_eq!(
        s.add_difference(later, earlier, &[(V(2), 0.0)]),
        Err(Error::WrongInterventionWorld)
    );
    assert_eq!(s.nodes().len(), 8);
}

#[test]
fn missing_noise_and_inconsistent_shared_functions_are_errors_without_mutation() {
    let mut s = figure4(0.0);
    let (mut later, earlier) = equations();
    later.terms[1].arguments = vec![V(1)];
    assert_eq!(
        s.add_difference(later, earlier, &[(V(2), 0.0)]),
        Err(Error::InvalidEquation)
    );
    let (mut later, earlier) = equations();
    later.terms[0].arguments.reverse();
    assert_eq!(
        s.add_difference(later, earlier, &[(V(2), 0.0)]),
        Err(Error::ConflictingMechanism)
    );
    assert_eq!(s.nodes().len(), 8);
    let (later, earlier) = equations();
    assert!(s.add_difference(later, earlier, &[(V(2), 0.0)]).is_ok());
}

#[test]
fn repeated_differences_retain_consistent_equations_and_are_not_implicitly_conditioned() {
    let mut s = figure4(0.0);
    let (later, earlier) = equations();
    let a = s.add_difference(later, earlier, &[(V(2), 0.0)]).unwrap();
    let (later, earlier) = equations();
    let b = s.add_difference(earlier, later, &[(V(2), 0.0)]).unwrap();
    assert!(!s.separated_nodes(a, b, &[]).unwrap());
    assert!(s.separated_nodes(2, b, &[1]).unwrap());
    let (mut later, earlier) = equations();
    later.terms[1].identity = "replacement-g1".into();
    assert_eq!(
        s.add_difference(later, earlier, &[(V(2), 0.0)]),
        Err(Error::ConflictingMechanism)
    );
}
