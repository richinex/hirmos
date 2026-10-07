use hirmos_causal_core::causal_effects::*;

fn effects(edges: &[(usize, usize, usize)], lag: usize, x: Node) -> CausalEffects {
    let mut graph = StationaryGraph::new(4, lag);
    for &(a, b, l) in edges {
        graph.set(a, b, l, mark("-->"));
        if l == 0 {
            graph.set(b, a, 0, mark("<--"));
        }
    }
    CausalEffects::new(graph, &[x], &[(2, 0)], &[], &[])
}

#[test]
fn explicit_mediator_and_descendants_are_refused() {
    for (edges, z) in [
        (vec![(0, 1, 0), (1, 2, 0)], (1, 0)),
        (vec![(0, 1, 0), (1, 2, 0), (1, 3, 0)], (3, 0)),
        (vec![(0, 2, 0), (2, 3, 0)], (3, 0)),
    ] {
        let ce = effects(&edges, 0, (0, 0));
        assert!(ce
            .resolve_adjustment_set(&AdjustmentSetSelection::Explicit(vec![]))
            .is_ok());
        assert_eq!(
            ce.resolve_adjustment_set(&AdjustmentSetSelection::Explicit(vec![z])),
            Err(AdjustmentSetError::InvalidExplicitSet {
                problems: vec![ExplicitAdjustmentProblem::ForbiddenNode(z)]
            })
        );
    }
}

#[test]
fn lagged_mediator_is_refused_but_unrelated_treatment_child_is_allowed() {
    let ce = effects(&[(0, 1, 1), (1, 2, 1)], 1, (0, -2));
    assert!(ce
        .resolve_adjustment_set(&AdjustmentSetSelection::Explicit(vec![(1, -1)]))
        .is_err());
    let ce = effects(&[(0, 2, 0), (0, 3, 0)], 0, (0, 0));
    assert_eq!(
        ce.resolve_adjustment_set(&AdjustmentSetSelection::Explicit(vec![(3, 0)])),
        Ok(vec![(3, 0)])
    );
}

#[test]
fn query_endpoints_are_refused_even_without_confounding() {
    let ce = effects(&[(0, 2, 0)], 0, (0, 0));
    for z in [(0, 0), (2, 0)] {
        assert!(ce
            .resolve_adjustment_set(&AdjustmentSetSelection::Explicit(vec![z]))
            .is_err());
    }
}
