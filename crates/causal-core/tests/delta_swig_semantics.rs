//! Exact finite-distribution check, independent of graphical separation code.
//! Integer cell counts avoid Monte Carlo error and floating-point normalization.
use std::collections::BTreeMap;

// (X,D,delta,Y1(0),Y0), each row one equally likely exogenous configuration.
fn distribution(shared_alpha: bool) -> Vec<[i32; 5]> {
    let mut rows = vec![];
    for u in 0..2 {
        for ux in 0..4 {
            for ud in 0..8 {
                for e0 in 0..2 {
                    for e1 in 0..2 {
                        let x = i32::from(ux < 1 + 2 * u);
                        let d = i32::from(ud < 1 + 2 * u + x);
                        // Y0=(1+X)(U+e0)+X permits U/e0 ambiguity, so
                        // conditioning on Y0 actually activates the collider.
                        let alpha = u + x + u * x;
                        let y0 = alpha + (1 + x) * e0;
                        let y1 = alpha + 2 * x + (1 + x) * e1 + if shared_alpha { 0 } else { u };
                        rows.push([x, d, y1 - y0, y1, y0]);
                    }
                }
            }
        }
    }
    rows
}
fn independent(rows: &[[i32; 5]], a: usize, b: usize, given: &[usize]) -> bool {
    let mut joint = BTreeMap::<(Vec<i32>, i32, i32), i64>::new();
    let mut left = BTreeMap::<(Vec<i32>, i32), i64>::new();
    let mut right = left.clone();
    let mut totals = BTreeMap::<Vec<i32>, i64>::new();
    for row in rows {
        let z: Vec<_> = given.iter().map(|&i| row[i]).collect();
        *joint.entry((z.clone(), row[a], row[b])).or_default() += 1;
        *left.entry((z.clone(), row[a])).or_default() += 1;
        *right.entry((z.clone(), row[b])).or_default() += 1;
        *totals.entry(z).or_default() += 1;
    }
    joint
        .iter()
        .all(|((z, a, b), n)| n * totals[z] == left[&(z.clone(), *a)] * right[&(z.clone(), *b)])
}

#[test]
fn finite_scm_confirms_difference_independence_but_not_level_independence() {
    let rows = distribution(true);
    assert!(independent(&rows, 1, 2, &[0]));
    assert!(!independent(&rows, 1, 2, &[]));
    assert!(!independent(&rows, 1, 3, &[0]));
    // Conditioning on a level can activate its outcome-disturbance collider.
    assert!(!independent(&rows, 1, 2, &[0, 4]));
}

#[test]
fn changing_the_shared_function_breaks_the_claim() {
    assert!(!independent(&distribution(false), 1, 2, &[0]));
}
