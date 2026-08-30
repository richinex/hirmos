//! Pyodide-free DAG implication checks and whole-graph falsification.
//!
//! The contract follows Octopus's executed DoWhy 0.14 worker: KCI per observed local-Markov
//! implication, Holm family-wise correction, a one-sample KS check of raw p-value uniformity,
//! and Eulig et al.'s node-relabeling permutation baseline (`falsify_graph`).

use crate::backdoor::Dag;
use crate::kci::{kernel_conditional_independence, KciError};
use crate::nprandom::Mt19937;
use nalgebra::DMatrix;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CiKey {
    pair: (usize, usize),
    given: Vec<usize>,
}

impl CiKey {
    fn new(x: usize, y: usize, given: &[usize]) -> Self {
        let pair = if x <= y { (x, y) } else { (y, x) };
        let mut given = given.to_vec();
        given.sort_unstable();
        given.dedup();
        Self { pair, given }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DagImplication {
    pub x: usize,
    pub y: usize,
    pub given: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImplicationDecision {
    Contradicted,
    NotRefuted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DagImplicationTest {
    pub implication: DagImplication,
    pub p_value: f64,
    pub adjusted_p_value: f64,
    pub observations: usize,
    pub decision: ImplicationDecision,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformityTest {
    pub statistic: f64,
    pub p_value: f64,
    pub tests: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphFalsificationResult {
    pub given_lmc_violations: usize,
    pub given_lmc_tests: usize,
    pub given_lmc_violation_fraction: f64,
    pub permutation_lmc_violation_fractions: Vec<f64>,
    pub permutation_tpa_violation_fractions: Vec<f64>,
    pub p_value_lmc: f64,
    pub p_value_tpa: f64,
    pub permutations: usize,
    pub permutations_in_markov_equivalence_class: usize,
    pub falsifiable: bool,
    pub falsified: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DagCheckResult {
    pub implications: Vec<DagImplicationTest>,
    pub uniformity: Option<UniformityTest>,
    pub falsification: GraphFalsificationResult,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GraphCheckError {
    EmptyImplicationFamily,
    InvalidSignificanceLevel,
    InvalidPermutationCount,
    UnknownNode(usize),
    Kci(KciError),
}

impl From<KciError> for GraphCheckError {
    fn from(value: KciError) -> Self {
        Self::Kci(value)
    }
}

/// Holm's step-down family-wise error correction, in the input order.
pub fn holm_adjust(p_values: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..p_values.len()).collect();
    order.sort_by(|&left, &right| p_values[left].total_cmp(&p_values[right]));
    let mut adjusted = vec![0.0; p_values.len()];
    let mut running: f64 = 0.0;
    for (rank, index) in order.into_iter().enumerate() {
        let candidate = ((p_values.len() - rank) as f64 * p_values[index]).min(1.0);
        running = running.max(candidate);
        adjusted[index] = running;
    }
    adjusted
}

fn matrix_power(mut base: DMatrix<f64>, mut exponent: usize) -> DMatrix<f64> {
    let mut result = DMatrix::identity(base.nrows(), base.ncols());
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = &result * &base;
        }
        exponent >>= 1;
        if exponent > 0 {
            base = &base * &base;
        }
    }
    result
}

/// Exact two-sided one-sample KS CDF using the Marsaglia-Tsang-Wang matrix method.
fn ks_two_sided_cdf(n: usize, statistic: f64) -> f64 {
    if statistic <= 0.0 {
        return 0.0;
    }
    if statistic >= 1.0 {
        return 1.0;
    }
    let nd = n as f64 * statistic;
    let k = nd.floor() as usize + 1;
    let size = 2 * k - 1;
    let h = k as f64 - nd;
    let mut factorial = vec![1.0; size + 1];
    for index in 1..=size {
        factorial[index] = factorial[index - 1] * index as f64;
    }
    let mut matrix = DMatrix::from_fn(size, size, |row, column| {
        let degree = row as isize - column as isize + 1;
        if degree >= 0 {
            1.0 / factorial[degree as usize]
        } else {
            0.0
        }
    });
    for row in 0..size {
        matrix[(row, 0)] -= h.powi((row + 1) as i32) / factorial[row + 1];
    }
    for column in 0..size {
        let degree = size - column;
        matrix[(size - 1, column)] -= h.powi(degree as i32) / factorial[degree];
    }
    if 2.0 * h - 1.0 > 0.0 {
        matrix[(size - 1, 0)] += (2.0 * h - 1.0).powi(size as i32) / factorial[size];
    }
    let powered = matrix_power(matrix, n);
    let mut probability = powered[(k - 1, k - 1)];
    for index in 1..=n {
        probability *= index as f64 / n as f64;
    }
    probability.clamp(0.0, 1.0)
}

/// SciPy-compatible two-sided KS test against Uniform(0, 1).
pub fn uniformity_test(p_values: &[f64]) -> Option<UniformityTest> {
    if p_values.is_empty()
        || p_values
            .iter()
            .any(|p| !p.is_finite() || !(0.0..=1.0).contains(p))
    {
        return None;
    }
    let mut sorted = p_values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    let nf = n as f64;
    let statistic = sorted
        .iter()
        .enumerate()
        .fold(0.0_f64, |maximum, (index, &value)| {
            let upper = (index + 1) as f64 / nf - value;
            let lower = value - index as f64 / nf;
            maximum.max(upper).max(lower)
        });
    Some(UniformityTest {
        statistic,
        p_value: (1.0 - ks_two_sided_cdf(n, statistic)).clamp(0.0, 1.0),
        tests: n,
    })
}

fn select_columns(data: &DMatrix<f64>, columns: &[usize]) -> Result<DMatrix<f64>, GraphCheckError> {
    if let Some(&unknown) = columns.iter().find(|&&column| column >= data.ncols()) {
        return Err(GraphCheckError::UnknownNode(unknown));
    }
    Ok(DMatrix::from_fn(
        data.nrows(),
        columns.len(),
        |row, column| data[(row, columns[column])],
    ))
}

fn select_rows(data: &DMatrix<f64>, rows: &[usize]) -> DMatrix<f64> {
    DMatrix::from_fn(rows.len(), data.ncols(), |row, column| {
        data[(rows[row], column)]
    })
}

/// Match the worker's per-implication observation cap and `np.random.seed(7 + index)`.
pub fn test_implications(
    data: &DMatrix<f64>,
    implications: &[DagImplication],
    significance_level: f64,
    maximum_observations: usize,
) -> Result<Vec<DagImplicationTest>, GraphCheckError> {
    if implications.is_empty() {
        return Err(GraphCheckError::EmptyImplicationFamily);
    }
    if !(0.0..1.0).contains(&significance_level) {
        return Err(GraphCheckError::InvalidSignificanceLevel);
    }
    let mut raw = Vec::with_capacity(implications.len());
    for (index, implication) in implications.iter().enumerate() {
        let mut rng = Mt19937::seeded(7 + index as u32);
        let order = rng.permutation(data.nrows());
        let rows = &order[..data.nrows().min(maximum_observations)];
        let sample = select_rows(data, rows);
        let x = select_columns(&sample, &[implication.x])?;
        let y = select_columns(&sample, &[implication.y])?;
        let z = select_columns(&sample, &implication.given)?;
        let result =
            kernel_conditional_independence(&x, &y, (!implication.given.is_empty()).then_some(&z))?;
        raw.push(result.p_value);
    }
    let adjusted = holm_adjust(&raw);
    Ok(implications
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, implication)| DagImplicationTest {
            implication,
            p_value: raw[index],
            adjusted_p_value: adjusted[index],
            observations: data.nrows().min(maximum_observations),
            decision: if adjusted[index] <= significance_level {
                ImplicationDecision::Contradicted
            } else {
                ImplicationDecision::NotRefuted
            },
        })
        .collect())
}

fn parental_triples(dag: &Dag, include_unconditional: bool) -> Vec<DagImplication> {
    let mut triples = Vec::new();
    for node in 0..dag.n {
        let parents: BTreeSet<usize> = dag.parents[node].iter().copied().collect();
        if parents.is_empty() && !include_unconditional {
            continue;
        }
        let descendants = dag.descendants(node);
        for non_descendant in 0..dag.n {
            if non_descendant != node
                && !descendants.contains(&non_descendant)
                && !parents.contains(&non_descendant)
            {
                triples.push(DagImplication {
                    x: node,
                    y: non_descendant,
                    given: parents.iter().copied().collect(),
                });
            }
        }
    }
    triples
}

fn p_value_for(data: &DMatrix<f64>, implication: &DagImplication) -> Result<f64, GraphCheckError> {
    let x = select_columns(data, &[implication.x])?;
    let y = select_columns(data, &[implication.y])?;
    let z = select_columns(data, &implication.given)?;
    Ok(
        kernel_conditional_independence(&x, &y, (!implication.given.is_empty()).then_some(&z))?
            .p_value,
    )
}

fn validate_lmc(
    dag: &Dag,
    data: &DMatrix<f64>,
    significance_level: f64,
    memory: &mut BTreeMap<CiKey, f64>,
    rng: &mut Mt19937,
) -> Result<(usize, usize), GraphCheckError> {
    let triples = parental_triples(dag, true);
    let missing: Vec<(CiKey, DagImplication)> = triples
        .iter()
        .filter_map(|implication| {
            let key = CiKey::new(implication.x, implication.y, &implication.given);
            (!memory.contains_key(&key)).then_some((key, implication.clone()))
        })
        .collect::<BTreeMap<_, _>>()
        .into_iter()
        .collect();
    // DoWhy draws one int32 seed per uncached test before executing any test.
    for _ in &missing {
        let _ = rng.randint(i32::MAX as u64);
    }
    for (key, implication) in missing {
        memory.insert(key, p_value_for(data, &implication)?);
    }
    let violations = triples
        .iter()
        .filter(|implication| {
            let key = CiKey::new(implication.x, implication.y, &implication.given);
            memory[&key] <= significance_level
        })
        .count();
    Ok((triples.len(), violations))
}

fn validate_tpa(permuted: &Dag, reference: &Dag) -> (usize, usize) {
    let triples = parental_triples(permuted, true);
    let violations = triples
        .iter()
        .filter(|implication| {
            let given: BTreeSet<usize> = implication.given.iter().copied().collect();
            !reference.d_separated(implication.x, implication.y, &given)
        })
        .count();
    (triples.len(), violations)
}

fn permute_dag(dag: &Dag, permutation: &[usize]) -> Dag {
    let mut edges = Vec::new();
    for cause in 0..dag.n {
        for &effect in &dag.children[cause] {
            edges.push((permutation[cause], permutation[effect]));
        }
    }
    Dag::new(dag.n, &edges)
}

/// DoWhy 0.14 / Eulig et al. whole-graph node-permutation falsification.
pub fn falsify_graph(
    dag: &Dag,
    data: &DMatrix<f64>,
    permutations: usize,
    significance_level: f64,
    significance_ci: f64,
    random_seed: u32,
) -> Result<GraphFalsificationResult, GraphCheckError> {
    falsify_graph_with_progress(
        dag,
        data,
        permutations,
        significance_level,
        significance_ci,
        random_seed,
        |_, _| {},
    )
}

/// Falsification with one progress event after each evaluated node relabeling.
pub fn falsify_graph_with_progress<F>(
    dag: &Dag,
    data: &DMatrix<f64>,
    permutations: usize,
    significance_level: f64,
    significance_ci: f64,
    random_seed: u32,
    mut progress: F,
) -> Result<GraphFalsificationResult, GraphCheckError>
where
    F: FnMut(usize, usize),
{
    if permutations == 0 {
        return Err(GraphCheckError::InvalidPermutationCount);
    }
    if !(0.0..1.0).contains(&significance_level) || !(0.0..1.0).contains(&significance_ci) {
        return Err(GraphCheckError::InvalidSignificanceLevel);
    }
    if data.ncols() < dag.n {
        return Err(GraphCheckError::UnknownNode(data.ncols()));
    }
    let mut rng = Mt19937::seeded(random_seed);
    let mut memory = BTreeMap::new();
    let (given_tests, given_violations) =
        validate_lmc(dag, data, significance_ci, &mut memory, &mut rng)?;
    let given_fraction = given_violations as f64 / given_tests.max(1) as f64;

    let mut permutation_lmc = Vec::with_capacity(permutations);
    let mut permutation_tpa = Vec::with_capacity(permutations);
    progress(0, permutations);
    for completed in 0..permutations {
        let ordering = rng.permutation(dag.n);
        let permuted = permute_dag(dag, &ordering);
        let (lmc_tests, lmc_violations) =
            validate_lmc(&permuted, data, significance_ci, &mut memory, &mut rng)?;
        let (tpa_tests, tpa_violations) = validate_tpa(&permuted, dag);
        permutation_lmc.push(lmc_violations as f64 / lmc_tests.max(1) as f64);
        permutation_tpa.push(tpa_violations as f64 / tpa_tests.max(1) as f64);
        progress(completed + 1, permutations);
    }
    let p_value_lmc = permutation_lmc
        .iter()
        .filter(|&&fraction| fraction <= given_fraction)
        .count() as f64
        / permutations as f64;
    let permutations_in_markov_equivalence_class = permutation_tpa
        .iter()
        .filter(|&&fraction| fraction == 0.0)
        .count();
    let p_value_tpa = permutations_in_markov_equivalence_class as f64 / permutations as f64;
    let can_evaluate = given_tests > 0;
    let falsifiable = can_evaluate && p_value_tpa <= significance_level;
    let falsified =
        can_evaluate && p_value_lmc > significance_level && p_value_tpa < significance_level;
    Ok(GraphFalsificationResult {
        given_lmc_violations: given_violations,
        given_lmc_tests: given_tests,
        given_lmc_violation_fraction: given_fraction,
        permutation_lmc_violation_fractions: permutation_lmc,
        permutation_tpa_violation_fractions: permutation_tpa,
        p_value_lmc,
        p_value_tpa,
        permutations,
        permutations_in_markov_equivalence_class,
        falsifiable,
        falsified,
    })
}

/// Execute the complete Octopus DAG-check contract on one dense numeric dataset.
pub fn check_dag(
    dag: &Dag,
    data: &DMatrix<f64>,
    implications: &[DagImplication],
    maximum_observations: usize,
    permutations: usize,
    significance_level: f64,
) -> Result<DagCheckResult, GraphCheckError> {
    let implications =
        test_implications(data, implications, significance_level, maximum_observations)?;
    let raw: Vec<f64> = implications.iter().map(|test| test.p_value).collect();
    let uniformity = uniformity_test(&raw);
    let falsification_data = if data.nrows() <= maximum_observations {
        data.clone()
    } else {
        let mut rng = Mt19937::seeded(7);
        let rows = rng.permutation(data.nrows());
        select_rows(data, &rows[..maximum_observations])
    };
    let falsification = falsify_graph(
        dag,
        &falsification_data,
        permutations,
        significance_level,
        significance_level,
        11,
    )?;
    Ok(DagCheckResult {
        implications,
        uniformity,
        falsification,
    })
}
