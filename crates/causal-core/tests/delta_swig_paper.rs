//! Appendix F.1--F.3 structural equations, not inferred from plot coordinates.
use hirmos_causal_core::swig::{
    AdditiveTerm, AvailabilityBlocker as Blocker, CausalDag,
    ConditioningAvailability as SetAvailability, CovariateAvailability as Availability, Equation,
    Error, Role::*, Swig, Variable as V,
};

fn term(name: &str, args: &[usize]) -> AdditiveTerm {
    AdditiveTerm {
        identity: name.into(),
        arguments: args.iter().copied().map(V).collect(),
    }
}
// U=0, X0=1, X1=2, D=3, Y0=4, Y1=5, UY0=6, UY1=7,
// UX0=8, UX1=9, UD=10. Include independent explicit disturbances.
fn model(dynamics: bool, outcome_treatment: bool, feedback: bool) -> (Swig, usize) {
    model_with_covariate_feedback(dynamics, outcome_treatment, feedback, false)
}
fn model_with_covariate_feedback(
    dynamics: bool,
    outcome_treatment: bool,
    feedback: bool,
    outcome_covariate: bool,
) -> (Swig, usize) {
    let mut edges = vec![
        (0, 1),
        (8, 1),
        (0, 2),
        (1, 2),
        (9, 2),
        (0, 3),
        (1, 3),
        (10, 3),
        (0, 4),
        (1, 4),
        (6, 4),
        (0, 5),
        (1, 5),
        (2, 5),
        (3, 5),
        (7, 5),
    ];
    if feedback {
        edges.push((3, 2));
    } else {
        edges.push((2, 3));
    }
    if dynamics {
        edges.push((4, 5));
    }
    if outcome_treatment {
        edges.push((4, 3));
    }
    if outcome_covariate {
        edges.push((4, 2));
    }
    let roles = vec![
        Exogenous, Endogenous, Endogenous, Endogenous, Endogenous, Endogenous, Exogenous,
        Exogenous, Exogenous, Exogenous, Exogenous,
    ];
    let mut s = CausalDag::new(
        roles,
        &edges.iter().map(|&(a, b)| (V(a), V(b))).collect::<Vec<_>>(),
    )
    .unwrap()
    .intervene(&[(V(3), 0.0)])
    .unwrap();
    let mut args = vec![1, 2, 7];
    if dynamics {
        args.push(4);
    }
    let delta = s
        .add_difference(
            Equation {
                outcome: V(5),
                terms: vec![term("alpha", &[0, 1]), term("g1", &args)],
            },
            Equation {
                outcome: V(4),
                terms: vec![term("alpha", &[0, 1]), term("g0", &[1, 6])],
            },
            &[(V(3), 0.0)],
        )
        .unwrap();
    (s, delta)
}

#[test]
fn figure6d_outcome_covariate_feedback_uses_main_text_graph() {
    // Figure 6(d) explicitly draws Y0 -> X1. F.2(d)'s printed assignment
    // mistakenly includes X1 on its own RHS; do not implement that cycle.
    let (s, delta) = model_with_covariate_feedback(false, false, false, true);
    for mask in 0..8 {
        let given: Vec<_> = [1, 2, 4]
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| (mask & (1 << i) != 0).then_some(v))
            .collect();
        assert!(
            !s.separated_nodes(3, delta, &given).unwrap(),
            "unexpected separator {given:?}"
        );
    }
}

#[test]
fn figure5_time_varying_covariates_require_both_periods() {
    let (s, delta) = model(false, false, false);
    for set in [vec![], vec![1], vec![2]] {
        assert!(!s.separated_nodes(3, delta, &set).unwrap());
    }
    assert!(s.separated_nodes(3, delta, &[1, 2]).unwrap());
    assert_eq!(
        s.covariate_availability(V(2), &[V(1), V(2)]).unwrap(),
        Availability::Observed
    );
}

#[test]
fn figure6_state_dependence_leaves_unblocked_path_through_baseline_outcome() {
    let (s, delta) = model(true, false, false);
    assert!(!s.separated_nodes(3, delta, &[1, 2]).unwrap());
    // Conditioning on Y0 opens D <- U -> Y0 <- UY0 -> delta.
    assert!(!s.separated_nodes(3, delta, &[1, 2, 4]).unwrap());
}

#[test]
fn figure6_outcome_treatment_feedback_leaves_disturbance_path() {
    let (s, delta) = model(false, true, false);
    assert!(!s.separated_nodes(3, delta, &[1, 2]).unwrap());
    assert!(!s.separated_nodes(3, delta, &[1, 2, 4]).unwrap());
}

#[test]
fn figure7_separating_potential_covariate_is_not_observed_for_everyone() {
    let (s, delta) = model(false, false, true);
    assert!(!s.separated_nodes(3, delta, &[1]).unwrap());
    assert!(s.separated_nodes(3, delta, &[1, 2]).unwrap());
    let measured = [V(1), V(2), V(3), V(4), V(5)];
    assert_eq!(
        s.covariate_availability(V(1), &measured).unwrap(),
        Availability::Observed
    );
    assert_eq!(
        s.covariate_availability(V(2), &measured).unwrap(),
        Availability::UnderConsistency {
            assignments: vec![(V(3), 0.0)]
        }
    );
    assert_eq!(
        s.covariate_availability(V(0), &measured).unwrap(),
        Availability::Unmeasured
    );
    assert_eq!(
        s.covariate_availability(V(2), &[V(1)]).unwrap(),
        Availability::Unmeasured
    );
}

#[test]
fn figures8_and9_two_treatments_three_periods_and_feedback() {
    // Appendix F.4: U, X0, X1, X2, D1, D2, Y0, Y1, Y2,
    // UY0, UY1, UY2, UX0, UX1, UX2, UD1, UD2.
    for feedback in [false, true] {
        let mut edges = vec![
            (0, 1),
            (12, 1),
            (0, 2),
            (1, 2),
            (13, 2),
            (0, 3),
            (2, 3),
            (14, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (15, 4),
            (0, 5),
            (1, 5),
            (2, 5),
            (3, 5),
            (4, 5),
            (16, 5),
            (0, 6),
            (1, 6),
            (9, 6),
            (0, 7),
            (1, 7),
            (2, 7),
            (4, 7),
            (10, 7),
            (0, 8),
            (1, 8),
            (2, 8),
            (3, 8),
            (4, 8),
            (5, 8),
            (11, 8),
        ];
        if feedback {
            edges.push((4, 3));
        }
        let roles = (0..17)
            .map(|i| {
                if i == 0 || i >= 9 {
                    Exogenous
                } else {
                    Endogenous
                }
            })
            .collect();
        let mut s = CausalDag::new(
            roles,
            &edges.iter().map(|&(a, b)| (V(a), V(b))).collect::<Vec<_>>(),
        )
        .unwrap()
        .intervene(&[(V(4), 0.0), (V(5), 0.0)])
        .unwrap();
        let equation = |outcome, name, args: &[usize]| Equation {
            outcome: V(outcome),
            terms: vec![term("alpha", &[0, 1]), term(name, args)],
        };
        let world = [(V(4), 0.0), (V(5), 0.0)];
        let first = s
            .add_difference(
                equation(7, "g1", &[1, 2, 10]),
                equation(6, "g0", &[1, 9]),
                &world,
            )
            .unwrap();
        let second = s
            .add_difference(
                equation(8, "g2", &[1, 2, 3, 11]),
                equation(7, "g1", &[1, 2, 10]),
                &world,
            )
            .unwrap();
        assert!(s.separated_nodes(4, first, &[1, 2]).unwrap());
        assert!(!s.separated_nodes(4, second, &[1, 2]).unwrap());
        assert!(s.separated_nodes(4, second, &[1, 2, 3]).unwrap());
        assert!(s.separated_nodes(5, second, &[1, 2, 3]).unwrap());
        let observed = [V(1), V(2), V(3), V(4), V(5), V(6), V(7), V(8)];
        // Section 5.3.1: the same potential set has different availability
        // for early adopters, late adopters, and never-treated comparisons.
        for history in [
            vec![(V(4), 0.0), (V(5), 1.0)],
            vec![(V(4), 0.0), (V(5), 0.0)],
        ] {
            assert_eq!(
                s.conditioning_availability(&[V(1), V(2), V(3)], &observed, &history)
                    .unwrap(),
                SetAvailability::Available
            );
        }
        assert_eq!(
            s.conditioning_availability(
                &[V(1), V(2), V(3)],
                &observed,
                &[(V(4), 1.0), (V(5), 1.0)]
            )
            .unwrap(),
            if feedback {
                SetAvailability::Unavailable {
                    blockers: vec![Blocker::ConflictingAssignment {
                        variable: V(3),
                        treatment: V(4),
                        required: 0.0,
                        actual: 1.0,
                    }],
                }
            } else {
                SetAvailability::Available
            }
        );
        assert_eq!(
            s.covariate_availability(V(3), &observed).unwrap(),
            if feedback {
                Availability::UnderConsistency {
                    assignments: vec![(V(4), 0.0)],
                }
            } else {
                Availability::Observed
            }
        );
        // Natural second-period assignment is D2(0), not generally observed D2.
        assert_eq!(
            s.covariate_availability(V(5), &observed).unwrap(),
            Availability::UnderConsistency {
                assignments: vec![(V(4), 0.0)]
            }
        );
    }
}

#[test]
fn conditioning_availability_does_not_invent_history_or_measurements() {
    let (s, _) = model(false, false, true);
    assert_eq!(
        s.conditioning_availability(&[V(2), V(0)], &[V(1), V(2)], &[])
            .unwrap(),
        SetAvailability::Unavailable {
            blockers: vec![
                Blocker::Unmeasured { variable: V(0) },
                Blocker::UnestablishedAssignment {
                    variable: V(2),
                    treatment: V(3),
                    required: 0.0
                },
            ]
        }
    );
    assert_eq!(
        s.conditioning_availability(&[], &[], &[]).unwrap(),
        SetAvailability::Available
    );
    for history in [
        vec![(V(3), f64::NAN)],
        vec![(V(3), 0.0), (V(3), 0.0)],
        vec![(V(1), 0.0)],
    ] {
        assert_eq!(
            s.conditioning_availability(&[], &[], &history),
            Err(Error::InvalidQuery)
        );
    }
    assert_eq!(
        s.conditioning_availability(&[V(1), V(1)], &[V(1)], &[]),
        Err(Error::InvalidQuery)
    );
    assert_eq!(
        s.conditioning_availability(&[], &[V(1), V(1)], &[]),
        Err(Error::InvalidQuery)
    );
    assert_eq!(
        s.conditioning_availability(&[V(100)], &[], &[]),
        Err(Error::UnknownVariable)
    );
}
