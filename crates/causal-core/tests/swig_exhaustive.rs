//! Independent active-path definition versus the reused moralized-ancestor engine.
use hirmos_causal_core::swig::{CausalDag, Node, Role, Variable as V};
use std::collections::BTreeSet;

fn active_path_exists(
    n: usize,
    edges: &[(usize, usize)],
    x: usize,
    y: usize,
    z: &BTreeSet<usize>,
) -> bool {
    let mut arrow = vec![vec![false; n]; n];
    for &(a, b) in edges {
        arrow[a][b] = true;
    }
    let mut reaches = arrow.clone();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                reaches[i][j] |= reaches[i][k] && reaches[k][j];
            }
        }
    }
    fn walk(
        arrow: &[Vec<bool>],
        reaches: &[Vec<bool>],
        path: &mut Vec<usize>,
        y: usize,
        z: &BTreeSet<usize>,
    ) -> bool {
        let b = *path.last().unwrap();
        if b == y {
            return true;
        }
        for c in 0..arrow.len() {
            if path.contains(&c) || !(arrow[b][c] || arrow[c][b]) {
                continue;
            }
            if path.len() > 1 {
                let a = path[path.len() - 2];
                let collider = arrow[a][b] && arrow[c][b];
                let open = if collider {
                    z.contains(&b) || z.iter().any(|&v| reaches[b][v])
                } else {
                    !z.contains(&b)
                };
                if !open {
                    continue;
                }
            }
            path.push(c);
            if walk(arrow, reaches, path, y, z) {
                return true;
            }
            path.pop();
        }
        false
    }
    walk(&arrow, &reaches, &mut vec![x], y, z)
}

#[test]
fn every_four_node_dag_all_intervention_subsets_all_queries() {
    let possible = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for mask in 0..64 {
        let edges: Vec<_> = possible
            .iter()
            .enumerate()
            .filter_map(|(i, &e)| (mask & (1 << i) != 0).then_some(e))
            .collect();
        let dag = CausalDag::new(
            vec![Role::Endogenous; 4],
            &edges.iter().map(|&(a, b)| (V(a), V(b))).collect::<Vec<_>>(),
        )
        .unwrap();
        for intervention in 0..16 {
            let settings: Vec<_> = (0..4)
                .filter(|i| intervention & (1 << i) != 0)
                .map(|i| (V(i), 0.0))
                .collect();
            let swig = dag.intervene(&settings).unwrap();
            let random_edges: Vec<_> = edges
                .iter()
                .copied()
                .filter(|&(a, _)| intervention & (1 << a) == 0)
                .collect();
            for x in 0..4 {
                for y in x + 1..4 {
                    let other: Vec<_> = (0..4).filter(|&i| i != x && i != y).collect();
                    for bits in 0..4 {
                        let given: Vec<_> = other
                            .iter()
                            .enumerate()
                            .filter_map(|(i, &v)| (bits & (1 << i) != 0).then_some(v))
                            .collect();
                        let expected = !active_path_exists(
                            4,
                            &random_edges,
                            x,
                            y,
                            &given.iter().copied().collect(),
                        );
                        assert_eq!(
                            swig.separated_nodes(x, y, &given).unwrap(),
                            expected,
                            "dag {mask} do {intervention} x{x} y{y} z{given:?}"
                        );
                        let mut interest = given.clone();
                        interest.extend([x, y]);
                        assert_eq!(
                            swig.prune_for(&interest)
                                .unwrap()
                                .separated_nodes(x, y, &given)
                                .unwrap(),
                            expected
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn appendix_b1_collider_conditioning_and_blocked_noncollider() {
    // X=0,Y=1,A=2,B=3,C=4,D=5. Follow Figure B.1 and Definition 1.
    let s = CausalDag::new(
        vec![Role::Endogenous; 6],
        &[
            (V(0), V(2)),
            (V(2), V(1)),
            (V(3), V(0)),
            (V(3), V(1)),
            (V(0), V(4)),
            (V(5), V(4)),
            (V(5), V(1)),
        ],
    )
    .unwrap()
    .intervene(&[])
    .unwrap();
    assert!(s.separated_nodes(0, 1, &[2, 3]).unwrap());
    // The draft prose says {C} blocks X->C<-D->Y. It opens it instead.
    assert!(!s.separated_nodes(0, 1, &[2, 3, 4]).unwrap());
    assert!(s.separated_nodes(0, 1, &[2, 3, 5]).unwrap());
    assert!(s.separated_nodes(0, 1, &[2, 3, 4, 5]).unwrap());
    assert!(!s.separated_nodes(0, 1, &[2]).unwrap());
    assert!(!s.separated_nodes(0, 1, &[3]).unwrap());
}

#[test]
fn figures2_and3_exact_split_graph_and_potential_labels() {
    // U,X,D,Y0,Y1,UY0,UY1,UX,UD. Explicit independent disturbances.
    let edges = [
        (0, 1),
        (7, 1),
        (0, 2),
        (1, 2),
        (8, 2),
        (0, 3),
        (1, 3),
        (5, 3),
        (0, 4),
        (1, 4),
        (2, 4),
        (6, 4),
    ];
    let roles = (0..9)
        .map(|i| {
            if i == 0 || i >= 5 {
                Role::Exogenous
            } else {
                Role::Endogenous
            }
        })
        .collect();
    let s = CausalDag::new(
        roles,
        &edges.iter().map(|&(a, b)| (V(a), V(b))).collect::<Vec<_>>(),
    )
    .unwrap()
    .intervene(&[(V(2), 0.0)])
    .unwrap();
    let expected: BTreeSet<_> = edges
        .iter()
        .map(|&(a, b)| (if a == 2 { 9 } else { a }, b))
        .collect();
    assert_eq!(s.edges().into_iter().collect::<BTreeSet<_>>(), expected);
    for i in 0..9 {
        match &s.nodes()[i] {
            Node::Random { interventions, .. } => {
                assert_eq!(interventions, &if i == 4 { vec![V(2)] } else { vec![] })
            }
            _ => panic!(),
        }
    }
    assert!(!s.separated_nodes(2, 4, &[1]).unwrap());
    assert!(s.separated_nodes(2, 4, &[0, 1]).unwrap());
    // Figure 3 has the same adjacency: alpha annotations alone are not surgery.
}
