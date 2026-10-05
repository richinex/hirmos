use hirmos_causal_core::swig::{did::*, AdditiveTerm, CausalDag, Equation, Role, Variable as V};

#[derive(Clone, Copy)]
enum CovariateEffect {
    History,
    Contemporary,
    None,
}
#[derive(Clone, Copy)]
enum Feedback {
    None,
    Previous,
    Within,
}
fn assumptions() -> Assumptions {
    Assumptions {
        absorbing_binary_treatment: true,
        consistency: true,
        independent_disturbances: true,
    }
}
fn term(name: &str, arguments: Vec<V>) -> AdditiveTerm {
    AdditiveTerm {
        identity: name.into(),
        arguments,
    }
}
fn fixture(
    p: usize,
    feedback: Feedback,
    effect: CovariateEffect,
) -> (CausalDag, Vec<PanelRole>, Vec<Equation>, AdditiveTerm) {
    let x = |t| V(1 + t);
    let d = |t| V(p + t);
    let y = |t| V(2 * p + t);
    let e = |t| V(3 * p + t);
    let mut graph_roles = vec![Role::Endogenous; 4 * p];
    graph_roles[0] = Role::Exogenous;
    let mut roles = vec![PanelRole::Confounder; 4 * p];
    let mut edges = vec![];
    let alpha = term(
        "alpha",
        if matches!(effect, CovariateEffect::History) {
            vec![V(0), x(0)]
        } else {
            vec![V(0)]
        },
    );
    let mut equations = vec![];
    for t in 0..p {
        roles[x(t).0] = PanelRole::Covariate { period: t };
        roles[y(t).0] = PanelRole::Outcome { period: t };
        roles[e(t).0] = PanelRole::Disturbance { of: y(t) };
        graph_roles[e(t).0] = Role::Exogenous;
        edges.extend([(V(0), x(t)), (V(0), y(t)), (e(t), y(t))]);
        if t > 0 {
            roles[d(t).0] = PanelRole::Treatment { period: t };
            edges.push((V(0), d(t)));
            for s in 0..=t {
                if s > 0 {
                    edges.push((d(s), y(t)));
                }
                if !matches!(feedback, Feedback::Within) || s < t {
                    edges.push((x(s), d(t)));
                }
            }
            if t > 1 {
                edges.push((d(t - 1), d(t)));
            }
            match feedback {
                Feedback::Previous if t > 1 => edges.push((d(t - 1), x(t))),
                Feedback::Within => edges.push((d(t), x(t))),
                _ => {}
            }
        }
        let covs: Vec<_> = match effect {
            CovariateEffect::History => (0..=t).map(x).collect(),
            CovariateEffect::Contemporary => vec![x(t)],
            CovariateEffect::None => vec![],
        };
        edges.extend(covs.iter().map(|&x| (x, y(t))));
        equations.push(Equation {
            outcome: y(t),
            terms: vec![
                alpha.clone(),
                term(&format!("g{t}"), covs.into_iter().chain([e(t)]).collect()),
            ],
        });
    }
    (
        CausalDag::new(graph_roles, &edges).unwrap(),
        roles,
        equations,
        alpha,
    )
}
fn model(p: usize, feedback: Feedback, effect: CovariateEffect) -> StaggeredModel {
    let (dag, roles, equations, alpha) = fixture(p, feedback, effect);
    StaggeredModel::new(dag, roles, equations, alpha, assumptions()).unwrap()
}
fn family(
    model: &StaggeredModel,
    g: usize,
    t: usize,
    p: usize,
    comparison: Comparison,
) -> AdjustmentFamily {
    match model
        .adjustment(g, t, comparison, &(1..=p).map(V).collect::<Vec<_>>())
        .unwrap()
    {
        AdjustmentDecision::SupportedSubjectToOverlap(family) => family,
        other => panic!("unexpected decision: {other:?}"),
    }
}

#[test]
fn propositions_one_and_two_figure9_cohort_specific_feedback() {
    for feedback in [Feedback::None, Feedback::Previous] {
        let m = model(3, feedback, CovariateEffect::History);
        let late = family(&m, 2, 2, 3, Comparison::NeverTreated);
        assert_eq!(late.required(), vec![V(1), V(2), V(3)]);
        assert!(late.supports(&[V(3), V(1), V(2)]).unwrap());
        assert!(!late.supports(&[V(1), V(2)]).unwrap());
        let early = m
            .adjustment(1, 2, Comparison::NeverTreated, &[V(1), V(2), V(3)])
            .unwrap();
        assert_eq!(
            matches!(early, AdjustmentDecision::NotEstablished { .. }),
            matches!(feedback, Feedback::Previous)
        );
        let immediate = family(&m, 1, 1, 3, Comparison::NeverTreated);
        assert_eq!(immediate.required(), vec![V(1), V(2)]);
        // X2 is OPTIONAL as an observed variable even with D1 -> X2.
        assert!(immediate.supports(&[V(1), V(2), V(3)]).unwrap());
        assert!(immediate.supports(&[V(1), V(2), V(2)]).is_err());
        assert!(!immediate.supports(&[V(1), V(2), V(4)]).unwrap());
    }
}

#[test]
fn propositions_three_and_four_not_yet_treated_and_invalid_timing() {
    let m = model(4, Feedback::Previous, CovariateEffect::Contemporary);
    for comparison in [
        Comparison::NeverTreated,
        Comparison::NotYetTreated { through: 2 },
        Comparison::NotYetTreated { through: 3 },
    ] {
        let f = family(&m, 2, 2, 4, comparison);
        assert_eq!(f.required(), vec![V(2), V(3)]);
        assert!(f.supports(&[V(1), V(2), V(3), V(4)]).unwrap());
        assert_eq!(f.comparison(), comparison);
    }
    for (g, t, s) in [
        (2, 2, 1),
        (2, 3, 2),
        (2, 0, 0),
        (1, 1, 4),
        (0, 1, 2),
        (4, 1, 3),
        (1, 4, 3),
    ] {
        assert_eq!(
            m.adjustment(g, t, Comparison::NotYetTreated { through: s }, &[]),
            Err(ModelError::InvalidComparison)
        );
    }
    // (E.16) allows the untreated-at-baseline mixture for pre-trends. By
    // Proposition 3 and total expectation, its conditional trend equals the
    // target cohort's, even though the populations overlap.
    assert_eq!(
        family(&m, 2, 0, 4, Comparison::NotYetTreated { through: 1 }).required(),
        vec![V(1), V(2)]
    );
}

#[test]
fn multiple_covariates_and_confounders_are_not_hardcoded_to_the_figures() {
    let (dag, mut roles, mut eq, mut alpha) = fixture(3, Feedback::None, CovariateEffect::History);
    let original = dag.intervene(&[]).unwrap();
    let mut edges: Vec<_> = original
        .edges()
        .into_iter()
        .map(|(a, b)| (V(a), V(b)))
        .collect();
    let mut graph_roles: Vec<_> = original
        .nodes()
        .iter()
        .map(|n| match n {
            hirmos_causal_core::swig::Node::Random { role, .. } => *role,
            _ => unreachable!(),
        })
        .collect();
    roles.extend([
        PanelRole::Confounder,
        PanelRole::Covariate { period: 1 },
        PanelRole::Covariate { period: 2 },
    ]);
    graph_roles.extend([Role::Exogenous, Role::Endogenous, Role::Endogenous]);
    alpha.arguments.push(V(12));
    for (i, equation) in eq.iter_mut().enumerate() {
        equation.terms[0] = alpha.clone();
        edges.push((V(12), equation.outcome));
        if i > 0 {
            edges.push((V(13), equation.outcome));
            equation.terms[1].arguments.push(V(13));
        }
        if i > 1 {
            edges.push((V(14), equation.outcome));
            equation.terms[1].arguments.push(V(14));
        }
    }
    edges.extend([
        (V(12), V(13)),
        (V(12), V(14)),
        (V(13), V(4)),
        (V(13), V(5)),
        (V(14), V(5)),
    ]);
    let m = StaggeredModel::new(
        CausalDag::new(graph_roles, &edges).unwrap(),
        roles,
        eq,
        alpha,
        assumptions(),
    )
    .unwrap();
    let result = m
        .adjustment(
            1,
            1,
            Comparison::NeverTreated,
            &[V(1), V(2), V(3), V(13), V(14)],
        )
        .unwrap();
    match result {
        AdjustmentDecision::SupportedSubjectToOverlap(f) => {
            assert_eq!(f.required(), vec![V(1), V(2), V(13)])
        }
        _ => panic!(),
    }
}

#[test]
fn appendix_h_endpoints_pretrends_and_no_covariate_effects() {
    for feedback in [Feedback::None, Feedback::Previous] {
        let m = model(4, feedback, CovariateEffect::Contemporary);
        assert_eq!(
            family(&m, 2, 2, 4, Comparison::NeverTreated).required(),
            vec![V(2), V(3)]
        );
        assert_eq!(
            family(&m, 2, 0, 4, Comparison::NeverTreated).required(),
            vec![V(1), V(2)]
        );
        let dynamic = m
            .adjustment(2, 3, Comparison::NeverTreated, &[V(1), V(2), V(3), V(4)])
            .unwrap();
        match feedback {
            Feedback::None => assert_eq!(
                family(&m, 2, 3, 4, Comparison::NeverTreated).required(),
                vec![V(2), V(4)]
            ),
            _ => assert!(matches!(dynamic, AdjustmentDecision::NotEstablished { .. })),
        }
        assert_eq!(
            m.adjustment(2, 1, Comparison::NeverTreated, &[]).unwrap(),
            AdjustmentDecision::BaselineIdentity
        );
        let r = m.restrictions();
        assert!(r.no_within_period_treatment_covariate_effect);
        assert_eq!(
            r.no_future_treatment_covariate_effect,
            matches!(feedback, Feedback::None)
        );
        assert!(r.no_direct_covariate_outcome_dynamics);
        assert!(!r.no_within_period_covariate_outcome_effect);
    }
    let m = model(4, Feedback::Within, CovariateEffect::None);
    for g in 1..4 {
        for t in 0..4 {
            if t != g - 1 {
                assert!(family(&m, g, t, 4, Comparison::NeverTreated)
                    .required()
                    .is_empty());
            }
        }
    }
    let m = model(4, Feedback::Within, CovariateEffect::Contemporary);
    assert!(matches!(
        m.adjustment(2, 2, Comparison::NeverTreated, &[V(1), V(2), V(3), V(4)])
            .unwrap(),
        AdjustmentDecision::NotEstablished { .. }
    ));
    assert_eq!(
        family(&m, 2, 0, 4, Comparison::NeverTreated).required(),
        vec![V(1), V(2)]
    );
}

#[test]
fn missing_measurements_are_not_confused_with_graphical_failure() {
    let m = model(3, Feedback::None, CovariateEffect::History);
    match m
        .adjustment(1, 1, Comparison::NeverTreated, &[V(1)])
        .unwrap()
    {
        AdjustmentDecision::NotEstablished {
            treated_blockers,
            comparison_blockers,
            ..
        } => {
            assert_eq!(treated_blockers.len(), 1);
            assert_eq!(comparison_blockers, treated_blockers);
        }
        _ => panic!("missing required column accepted"),
    }
    let f = match m
        .adjustment(1, 1, Comparison::NeverTreated, &[V(1), V(2)])
        .unwrap()
    {
        AdjustmentDecision::SupportedSubjectToOverlap(f) => f,
        _ => panic!(),
    };
    assert_eq!(f.available(), vec![V(1), V(2)]);
}

#[test]
fn assumptions_and_functional_restrictions_are_enforced() {
    for missing in 0..3 {
        let (dag, roles, eq, alpha) = fixture(3, Feedback::None, CovariateEffect::History);
        let mut a = assumptions();
        match missing {
            0 => a.absorbing_binary_treatment = false,
            1 => a.consistency = false,
            _ => a.independent_disturbances = false,
        };
        assert!(matches!(
            StaggeredModel::new(dag, roles, eq, alpha, a),
            Err(ModelError::MissingAssumption(_))
        ));
    }
    for change in 0..4 {
        let (dag, roles, mut eq, mut alpha) = fixture(3, Feedback::None, CovariateEffect::History);
        match change {
            0 => eq[1].terms[0].identity = "different alpha".into(),
            1 => eq[1].terms[1].arguments.push(V(0)),
            2 => alpha.arguments.clear(),
            _ => eq[1].terms[1].arguments.pop().map(|_| ()).unwrap(),
        }
        assert!(StaggeredModel::new(dag, roles, eq, alpha, assumptions()).is_err());
    }
}

#[test]
fn incompatible_structural_models_are_refused() {
    for change in 0..4 {
        let (dag, mut roles, eq, alpha) = fixture(3, Feedback::None, CovariateEffect::History);
        let mut edges: Vec<_> = dag_edges_for_fixture();
        match change {
            0 => edges.push((V(6), V(7))), // Y0 -> Y1, R-Y fails
            1 => edges.push((V(3), V(2))), // X2 -> X1, chronology fails
            2 => edges.push((V(9), V(4))), // outcome disturbance also affects treatment
            _ => roles[4] = PanelRole::Treatment { period: 0 },
        }
        let _ = dag;
        let mut gr = vec![Role::Endogenous; 12];
        gr[0] = Role::Exogenous;
        gr[9..].fill(Role::Exogenous);
        let dag = CausalDag::new(gr, &edges).unwrap();
        assert!(StaggeredModel::new(dag, roles, eq, alpha, assumptions()).is_err());
    }
}
fn dag_edges_for_fixture() -> Vec<(V, V)> {
    // Export through a zero-intervention SWIG; reuse the public graph interface.
    fixture(3, Feedback::None, CovariateEffect::History)
        .0
        .intervene(&[])
        .unwrap()
        .edges()
        .into_iter()
        .map(|(a, b)| (V(a), V(b)))
        .collect()
}
