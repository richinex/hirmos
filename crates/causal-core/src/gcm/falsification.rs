//! Ordered, cached graph checks for the pinned v0.14 notebook execution.

use super::{
    independence::kernel_test,
    model::{Graph, ModelError},
    node_order,
    shapley::Execution,
};
use crate::{
    backdoor::Dag,
    graph_falsification::{CiKey, DagImplication},
    kci::KciError,
    nprandom::Mt19937,
};
use nalgebra::DMatrix;
use std::{
    collections::{BTreeMap, BTreeSet},
    num::NonZeroUsize,
};

/// The absence of testable implications has no reject/retain decision.
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    NoImplications,
    NotInformative,
    Retained,
    Rejected,
}

#[derive(Debug)]
pub enum FalsificationError {
    Model(ModelError),
    Kernel(KciError),
}

/// Test order and p-values are retained for source trace comparisons.
pub struct Validation {
    pub order: Vec<usize>,
    pub implications: Vec<DagImplication>,
    pub p_values: Vec<f64>,
    pub lmc_violations: usize,
    pub tpa_violations: usize,
}

pub struct Falsification {
    pub given: Validation,
    pub permutations: Vec<Validation>,
    pub p_value_lmc: f64,
    pub p_value_tpa: f64,
    pub verdict: Verdict,
}

/// Notebook defaults: KCI cap 500, CI threshold .05, graph threshold .2.
/// Label-set iteration is defined by CPython 3.12 with PYTHONHASHSEED=0.
pub fn evaluate(
    graph: &Graph,
    data: &DMatrix<f64>,
    permutations: NonZeroUsize,
    execution: Execution,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
) -> Result<Falsification, FalsificationError> {
    graph.validate(data).map_err(FalsificationError::Model)?;
    let edges: Vec<_> = (0..graph.names().len())
        .flat_map(|child| {
            graph
                .parents(child)
                .unwrap()
                .iter()
                .map(move |&parent| (parent, child))
        })
        .collect();
    let reference = Dag::new(graph.names().len(), &edges);
    let mut memory = BTreeMap::new();
    let identity: Vec<_> = (0..graph.names().len()).collect();
    let given = validate(
        graph.names(),
        &reference,
        &reference,
        identity.clone(),
        data,
        execution,
        &mut memory,
        rng,
        cache,
    )?;
    let factorial = (1..=graph.names().len()).try_fold(1usize, |a, b| a.checked_mul(b));
    let exhaustive = factorial.is_some_and(|n| permutations.get() > n);
    let count = if exhaustive {
        factorial.unwrap()
    } else {
        permutations.get()
    };
    let mut results = Vec::with_capacity(count);
    let mut order = identity;
    for index in 0..count {
        match exhaustive {
            true if index > 0 => next_permutation(&mut order),
            true => {}
            false => order = rng.permutation(graph.names().len()),
        }
        let mapped: Vec<_> = edges.iter().map(|&(a, b)| (order[a], order[b])).collect();
        let permuted = Dag::new(graph.names().len(), &mapped);
        results.push(validate(
            graph.names(),
            &permuted,
            &reference,
            order.clone(),
            data,
            execution,
            &mut memory,
            rng,
            cache,
        )?);
    }
    let fraction = |n: usize, tests: usize| n as f64 / tests.max(1) as f64;
    let lmc = fraction(given.lmc_violations, given.implications.len());
    let tpa = fraction(given.tpa_violations, given.implications.len());
    let p_value_lmc = results
        .iter()
        .filter(|v| fraction(v.lmc_violations, v.implications.len()) <= lmc)
        .count() as f64
        / count as f64;
    let p_value_tpa = results
        .iter()
        .filter(|v| fraction(v.tpa_violations, v.implications.len()) <= tpa)
        .count() as f64
        / count as f64;
    let verdict = match (
        given.implications.is_empty(),
        p_value_lmc > 0.2,
        p_value_tpa.partial_cmp(&0.2).unwrap(),
    ) {
        (true, _, _) => Verdict::NoImplications,
        (false, true, std::cmp::Ordering::Less) => Verdict::Rejected,
        (false, _, std::cmp::Ordering::Greater) => Verdict::NotInformative,
        _ => Verdict::Retained,
    };
    Ok(Falsification {
        given,
        permutations: results,
        p_value_lmc,
        p_value_tpa,
        verdict,
    })
}

fn validate(
    names: &[String],
    dag: &Dag,
    reference: &Dag,
    order: Vec<usize>,
    data: &DMatrix<f64>,
    execution: Execution,
    memory: &mut BTreeMap<CiKey, f64>,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
) -> Result<Validation, FalsificationError> {
    let labels: Vec<_> = order.iter().map(|&i| names[i].clone()).collect();
    let mut implications = Vec::new();
    for &node in &order {
        let mut parents = dag.parents[node].clone();
        parents.sort_by(|&a, &b| names[a].cmp(&names[b]));
        let mut excluded = dag.descendants(node);
        excluded.insert(node);
        excluded.extend(parents.iter().copied());
        let positions: Vec<_> = order
            .iter()
            .enumerate()
            .filter_map(|(i, n)| excluded.contains(n).then_some(i))
            .collect();
        for position in node_order::difference_order(&labels, &positions) {
            implications.push(DagImplication {
                x: node,
                y: order[position],
                given: parents.clone(),
            });
        }
    }
    let mut pending = BTreeSet::new();
    let tests: Vec<_> = implications
        .iter()
        .filter(|i| {
            let key = CiKey::new(i.x, i.y, &i.given);
            !memory.contains_key(&key) && pending.insert(key)
        })
        .collect();
    let seeds: Vec<_> = tests
        .iter()
        .map(|_| rng.randint(i32::MAX as u64) as u32)
        .collect();
    for (test, seed) in tests.into_iter().zip(seeds) {
        let p = match execution {
            Execution::Serial => {
                *rng = Mt19937::seeded(seed);
                *cache = None;
                test_value(test, data, rng, cache)
            }
            Execution::Isolated => test_value(test, data, &mut Mt19937::seeded(seed), &mut None),
        }
        .map_err(FalsificationError::Kernel)?;
        memory.insert(CiKey::new(test.x, test.y, &test.given), p);
    }
    let p_values: Vec<_> = implications
        .iter()
        .map(|i| memory[&CiKey::new(i.x, i.y, &i.given)])
        .collect();
    let lmc_violations = p_values.iter().filter(|&&p| p <= 0.05).count();
    let tpa_violations = implications
        .iter()
        .filter(|i| !reference.d_separated(i.x, i.y, &i.given.iter().copied().collect()))
        .count();
    Ok(Validation {
        order,
        implications,
        p_values,
        lmc_violations,
        tpa_violations,
    })
}

fn test_value(
    test: &DagImplication,
    data: &DMatrix<f64>,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
) -> Result<f64, KciError> {
    let x = data.columns(test.x, 1).into_owned();
    let y = data.columns(test.y, 1).into_owned();
    let z = (!test.given.is_empty()).then(|| {
        DMatrix::from_fn(data.nrows(), test.given.len(), |r, c| {
            data[(r, test.given[c])]
        })
    });
    kernel_test(
        &x,
        &y,
        z.as_ref(),
        NonZeroUsize::new(500).unwrap(),
        Execution::Serial,
        rng,
        cache,
    )
    .map(|r| r.p_value)
}

fn next_permutation(order: &mut [usize]) {
    let pivot = (0..order.len() - 1)
        .rev()
        .find(|&i| order[i] < order[i + 1])
        .unwrap();
    let swap = (pivot + 1..order.len())
        .rev()
        .find(|&i| order[i] > order[pivot])
        .unwrap();
    order.swap(pivot, swap);
    order[pivot + 1..].reverse();
}
