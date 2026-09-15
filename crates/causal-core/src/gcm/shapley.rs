//! DoWhy v0.14 exact and default Halton-permutation Shapley estimators.

use super::coalitions::Coalitions;
use crate::{halton::Halton, nprandom::Mt19937, numpy_argsort::argsort};
use std::{collections::HashMap, num::NonZeroUsize};

#[derive(Clone, Copy, Debug)]
pub enum Method {
    Auto,
    Exact,
    Permutation { count: NonZeroUsize },
}

/// Joblib's in-process jobs reseed the caller; process-isolated jobs do not.
#[derive(Clone, Copy, Debug)]
pub enum Execution {
    Serial,
    Isolated,
}

enum Scope { Caller, Serial, Isolated(Mt19937) }

/// SciPy distribution objects retain a separate RNG after process serialization.
pub enum DistributionRandom<'a> {
    Shared,
    Isolated { rng: &'a mut Mt19937, cache: &'a mut Option<f64> },
}

/// Observed joblib batches, with each coalition assigned exactly once.
pub struct ProcessBatches {
    jobs: HashMap<Vec<bool>,usize>,
}

impl ProcessBatches {
    pub fn new(batches:Vec<Vec<Vec<bool>>>)->Result<Self,ShapleyError> {
        let width=batches.first().and_then(|b|b.first()).map(Vec::len).filter(|&n|n>0)
            .ok_or(ShapleyError::InvalidBatches)?;
        let mut jobs=HashMap::new();
        for (index,batch) in batches.into_iter().enumerate() {
            if batch.is_empty() {return Err(ShapleyError::InvalidBatches);}
            for subset in batch {
                if subset.len()!=width || jobs.insert(subset,index).is_some() {return Err(ShapleyError::InvalidBatches);}
            }
        }
        Ok(Self {jobs})
    }
}

#[derive(Clone,Copy)]
pub enum DistributionExecution<'a> {
    Serial,
    IndependentJobs,
    RecordedBatches(&'a ProcessBatches),
}

impl DistributionExecution<'_> {
    pub fn jobs(self)->Execution {
        match self {Self::Serial=>Execution::Serial,Self::IndependentJobs|Self::RecordedBatches(_)=>Execution::Isolated}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ShapleyError {
    OutputShape,
    NonFinite,
    Capacity,
    InvalidBatches,
}

fn check(value: &[f64], width: usize) -> Result<(), ShapleyError> {
    if width == 0 || value.len() != width {
        return Err(ShapleyError::OutputShape);
    }
    if value.iter().any(|v| !v.is_finite()) {
        return Err(ShapleyError::NonFinite);
    }
    Ok(())
}

fn evaluate<F>(
    subsets: Vec<Vec<bool>>,
    rng: &mut Mt19937,
    execution: Execution,
    function: &mut F,
) -> Result<HashMap<Vec<bool>, Vec<f64>>, ShapleyError>
where
    F: FnMut(&[bool], &mut Mt19937, Scope) -> Result<Vec<f64>, ShapleyError>,
{
    let seeds: Vec<_> = subsets
        .iter()
        .map(|_| rng.randint(i32::MAX as u64) as u32)
        .collect();
    let mut values = HashMap::new();
    let distribution_rng = rng.clone();
    for (subset, seed) in subsets.into_iter().zip(seeds) {
        let value = match execution {
            Execution::Serial => {
                *rng = Mt19937::seeded(seed);
                function(&subset, rng, Scope::Serial)?
            }
            Execution::Isolated => function(&subset, &mut Mt19937::seeded(seed), Scope::Isolated(distribution_rng.clone()))?,
        };
        check(&value, value.len())?;
        values.insert(subset, value);
    }
    Ok(values)
}

fn binary(index: usize, size: usize) -> Vec<bool> {
    (0..size)
        .map(|bit| index & (1 << (size - bit - 1)) != 0)
        .collect()
}

/// Return one row per output and one column per player, following NumPy's transpose.
pub fn estimate<F>(
    players: NonZeroUsize,
    method: Method,
    execution: Execution,
    rng: &mut Mt19937,
    mut function: F,
) -> Result<Vec<Vec<f64>>, ShapleyError>
where
    F: FnMut(&[bool], &mut Mt19937) -> Result<Vec<f64>, ShapleyError>,
{
    estimate_core(players,method,execution,rng,|subset,rng,_|function(subset,rng))
}

/// Preserve cached normal draws across caller evaluations and joblib reseeding.
pub fn estimate_with_cache<F>(players:NonZeroUsize,method:Method,execution:Execution,
    rng:&mut Mt19937,cache:&mut Option<f64>,mut function:F)->Result<Vec<Vec<f64>>,ShapleyError>
where F:FnMut(&[bool],&mut Mt19937,&mut Option<f64>)->Result<Vec<f64>,ShapleyError> {
    estimate_core(players,method,execution,rng,|subset,rng,scope|match scope {
        Scope::Caller=>function(subset,rng,cache),
        Scope::Serial=>{*cache=None; function(subset,rng,cache)},
        Scope::Isolated(_)=>function(subset,rng,&mut None),
    })
}

/// Distinguish the worker's NumPy RNG from its serialized SciPy distribution RNG.
pub fn estimate_with_distributions<F>(players:NonZeroUsize,method:Method,execution:DistributionExecution<'_>,
    rng:&mut Mt19937,cache:&mut Option<f64>,mut function:F)->Result<Vec<Vec<f64>>,ShapleyError>
where F:FnMut(&[bool],&mut Mt19937,&mut Option<f64>,DistributionRandom<'_>)->Result<Vec<f64>,ShapleyError> {
    let mut batch_states=HashMap::new();
    let mut visited=0;
    let result=estimate_core(players,method,execution.jobs(),rng,|subset,rng,scope|match scope {
        Scope::Caller=>function(subset,rng,cache,DistributionRandom::Shared),
        Scope::Serial=>{*cache=None; function(subset,rng,cache,DistributionRandom::Shared)},
        Scope::Isolated(mut distribution_rng)=>{
            visited+=1;
            if let DistributionExecution::RecordedBatches(plan)=execution {
                let batch=plan.jobs.get(subset).ok_or(ShapleyError::InvalidBatches)?;
                let (root_rng,root_cache)=batch_states.entry(*batch).or_insert_with(||(distribution_rng.clone(),*cache));
                return function(subset,rng,&mut None,DistributionRandom::Isolated {rng:root_rng,cache:root_cache});
            }
            let mut distribution_cache=*cache;
            function(subset,rng,&mut None,DistributionRandom::Isolated {rng:&mut distribution_rng,cache:&mut distribution_cache})
        },
    })?;
    if let DistributionExecution::RecordedBatches(plan)=execution {
        if visited!=plan.jobs.len() {return Err(ShapleyError::InvalidBatches);}
    }
    Ok(result)
}

fn estimate_core<F>(players:NonZeroUsize,method:Method,execution:Execution,rng:&mut Mt19937,mut function:F)
    ->Result<Vec<Vec<f64>>,ShapleyError>
where F:FnMut(&[bool],&mut Mt19937,Scope)->Result<Vec<f64>,ShapleyError> {
    let players = players.get();
    match method {
        Method::Auto if players <= 5 => exact(players, execution, rng, &mut function),
        Method::Auto => permutation(players, 25, execution, rng, &mut function),
        Method::Exact => exact(players, execution, rng, &mut function),
        Method::Permutation { count } => {
            permutation(players, count.get(), execution, rng, &mut function)
        }
    }
}

fn exact<F>(
    players: usize,
    execution: Execution,
    rng: &mut Mt19937,
    function: &mut F,
) -> Result<Vec<Vec<f64>>, ShapleyError>
where
    F: FnMut(&[bool], &mut Mt19937, Scope) -> Result<Vec<f64>, ShapleyError>,
{
    let count = 1usize
        .checked_shl(players.try_into().map_err(|_| ShapleyError::Capacity)?)
        .ok_or(ShapleyError::Capacity)?;
    let values = evaluate(
        (0..count).map(|i| binary(i, players)).collect(),
        rng,
        execution,
        function,
    )?;
    let width = values[&vec![false; players]].len();
    for value in values.values() {
        check(value, width)?;
    }
    let mut result = vec![vec![0.0; players]; width];
    for player in 0..players {
        for index in 0..count / 2 {
            let mut without = binary(index, players - 1);
            without.insert(player, false);
            let length = without.iter().filter(|&&v| v).count();
            let mut combination = 1.0;
            for i in 1..=length.min(players - 1 - length) {
                combination *= (players - i) as f64 / i as f64;
            }
            let weight = 1.0 / (players as f64 * combination);
            let mut with = without.clone();
            with[player] = true;
            for output in 0..width {
                result[output][player] +=
                    weight * (values[&with][output] - values[&without][output]);
            }
        }
    }
    Ok(result)
}

fn permutation<F>(
    players: usize,
    count: usize,
    execution: Execution,
    rng: &mut Mt19937,
    function: &mut F,
) -> Result<Vec<Vec<f64>>, ShapleyError>
where
    F: FnMut(&[bool], &mut Mt19937, Scope) -> Result<Vec<f64>, ShapleyError>,
{
    let full = function(&vec![true; players], rng,Scope::Caller)?;
    let empty = function(&vec![false; players], rng,Scope::Caller)?;
    let width = full.len();
    check(&full, width)?;
    check(&empty, width)?;
    let factorial = (1..=players).fold(1usize, usize::saturating_mul);
    let count = count.min(factorial);
    let mut halton = Halton::seeded(players, rng.randint(i32::MAX as u64));
    let permutations: Vec<_> = halton
        .draw(count)
        .map_err(|_| ShapleyError::Capacity)?
        .iter()
        .map(|v| argsort(v))
        .collect();
    let mut subsets = Coalitions::new();
    for order in &permutations {
        let mut subset = vec![false; players];
        for &player in order.iter().take(players - 1) {
            subset[player] = true;
            subsets.insert(subset.clone());
        }
    }
    let values = evaluate(subsets.into_order(), rng, execution, function)?;
    for value in values.values() {
        check(value, width)?;
    }
    let mut result = vec![vec![0.0; players]; width];
    for order in &permutations {
        let mut subset = vec![false; players];
        let mut previous = &empty;
        for (index, &player) in order.iter().enumerate() {
            subset[player] = true;
            let current = if index + 1 == players {
                &full
            } else {
                &values[&subset]
            };
            for output in 0..width {
                result[output][player] += current[output] - previous[output];
            }
            previous = current;
        }
    }
    for row in &mut result {
        for value in row {
            *value /= count as f64;
        }
    }
    Ok(result)
}
