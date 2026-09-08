//! PCMCI's `_optimize_pcmciplus_alpha`, inherited by J-PCMCI+ (GPL-3.0).
use super::*;

/// Nonempty, finite significance levels. The order determines score tie-breaking.
pub struct AlphaGrid(Vec<f64>);

impl AlphaGrid {
    pub fn new(values: Vec<f64>) -> Result<Self, RunError> {
        if values.is_empty()
            || values
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
        {
            return Err(RunError::Configuration);
        }
        Ok(Self(values))
    }
}

impl Default for AlphaGrid {
    fn default() -> Self {
        Self(vec![0.001, 0.005, 0.01, 0.025, 0.05])
    }
}

pub enum AlphaAssumptions {
    Unrestricted,
    Explicit(Vec<NodeInput>),
}

/// Context/dummy parents retained by a completed source JPCMCIplus instance.
pub struct AlphaHistory {
    pub(super) contexts: Vec<Vec<Node>>,
    pub(super) dummies: Vec<Vec<Node>>,
}

impl AlphaHistory {
    pub fn new(contexts: Vec<Vec<Node>>, dummies: Vec<Vec<Node>>) -> Result<Self, RunError> {
        let n = contexts.len();
        if n == 0 || dummies.len() != n {
            return Err(RunError::Shape);
        }
        if contexts
            .iter()
            .chain(&dummies)
            .flatten()
            .any(|&(i, lag)| i >= n || lag > 0)
        {
            return Err(RunError::Configuration);
        }
        Ok(Self { contexts, dummies })
    }

    pub fn from_discovery(discovery: &Discovery) -> Result<Self, RunError> {
        Self::new(
            discovery.context_parents.clone(),
            discovery.dummy_parents.clone(),
        )
    }
}

pub struct AlphaSelection {
    pub optimal_alpha: f64,
    pub scores: Vec<(f64, f64)>,
    pub discovery: Discovery,
}

/// Source behavior on a fresh JPCMCIplus instance: a list/None alpha invokes
/// inherited PCMCI+, NOT the context/dummy/system J-PCMCI+ pipeline. Consequently
/// classifications do not restrict this search, even for vector dummy variables.
/// Complete search parity is verified with a shared deterministic residual
/// primitive; the raw NumPy/Accelerate exact-fit boundary is audited separately
/// in `parity/jpcmciplus.md`. Hirmos transfer remains a separate task.
pub fn optimize_source_alpha(
    data: &dyn ConditionalIndependence,
    assumptions: AlphaAssumptions,
    tau: usize,
    options: Options,
    grid: AlphaGrid,
) -> Result<AlphaSelection, RunError> {
    optimize(data, assumptions, tau, options, grid, None)
}

/// Inherited alpha selection after a scalar run on the same source instance.
pub fn optimize_reused_alpha(
    data: &dyn ConditionalIndependence,
    assumptions: AlphaAssumptions,
    tau: usize,
    options: Options,
    grid: AlphaGrid,
    history: &AlphaHistory,
) -> Result<AlphaSelection, RunError> {
    optimize(data, assumptions, tau, options, grid, Some(history))
}

fn optimize(
    data: &dyn ConditionalIndependence,
    assumptions: AlphaAssumptions,
    tau: usize,
    options: Options,
    grid: AlphaGrid,
    history: Option<&AlphaHistory>,
) -> Result<AlphaSelection, RunError> {
    let n = data.variables();
    if history.is_some_and(|h| h.contexts.len() != n) {
        return Err(RunError::Shape);
    }
    let classes = vec![NodeClass::System; n];
    let nodes = match assumptions {
        AlphaAssumptions::Unrestricted => None,
        AlphaAssumptions::Explicit(nodes) => Some(
            nodes
                .into_iter()
                .map(|node| NodeInput {
                    class: NodeClass::System,
                    parents: node.parents,
                })
                .collect::<Vec<_>>(),
        ),
    };
    let mut fits = Vec::new();
    let mut scores = Vec::new();
    for alpha in grid.0 {
        let fit = match &nodes {
            None => super::search::run_with_history(data, &classes, tau, alpha, options, history)?,
            Some(nodes) => super::search::run_assumptions_with_history(
                data,
                nodes.clone(),
                tau,
                alpha,
                options,
                history,
            )?,
        };
        let strength: Vec<f64> = (0..n)
            .map(|j| {
                crate::numpy_reduce::numpy_sum_rows(
                    fit.val_matrix
                        .iter()
                        .map(|row| row[j].iter().map(|v| v.abs()).collect::<Vec<_>>()),
                )
            })
            .collect();
        let mut order = crate::numpy_argsort::argsort(&strength);
        order.reverse();
        let dag = dag_from_cpdag(&fit.graph, &order);
        let mut score = 0.0;
        for j in 0..n {
            let parents: Vec<_> = (0..n)
                .flat_map(|i| {
                    let dag = &dag;
                    (0..=tau).filter_map(move |lag| {
                        (dag[i][j][lag] == "-->").then_some((i, -(lag as i32)))
                    })
                })
                .collect();
            score += data.model_score(j, &parents, tau)?;
        }
        scores.push((alpha, score / n as f64));
        fits.push(fit);
    }
    // np.argmin: first minimum, or first NaN if any score is NaN.
    let mut best = 0;
    for i in 1..scores.len() {
        if !scores[best].1.is_nan() && (scores[i].1.is_nan() || scores[i].1 < scores[best].1) {
            best = i;
        }
    }
    let optimal_alpha = scores[best].0;
    // Source results are keyed by alpha: a repeated key overwrites its earlier
    // fit, even when the first occurrence supplied the minimum score.
    let selected = scores
        .iter()
        .rposition(|&(alpha, _)| alpha == optimal_alpha)
        .expect("the selected finite alpha occurs in the nonempty grid");
    Ok(AlphaSelection {
        optimal_alpha,
        scores,
        discovery: fits.remove(selected),
    })
}

/// PCMCIbase._get_dag_from_cpdag's simplicial-vertex elimination. Its source
/// leaves conflict marks in the returned graph; only directed edges enter scores.
fn dag_from_cpdag(graph: &[Vec<Vec<String>>], order: &[usize]) -> Vec<Vec<Vec<String>>> {
    let n = graph.len();
    let mut dag = graph.to_vec();
    let mut circle: Vec<Vec<String>> = graph
        .iter()
        .map(|row| {
            row.iter()
                .map(|lags| match lags[0].as_str() {
                    "x-x" | "-->" => String::new(),
                    other => other.to_owned(),
                })
                .collect()
        })
        .collect();
    loop {
        let node = order.iter().find_map(|&j| {
            let adj: Vec<_> = (0..n)
                .filter(|&i| circle[i][j] == "o-o" || circle[i][j] == "o?o")
                .collect();
            let clique = !adj.is_empty()
                && adj
                    .iter()
                    .enumerate()
                    .all(|(k, &i)| adj[k + 1..].iter().all(|&v| !circle[i][v].is_empty()));
            clique.then_some((j, adj))
        });
        let Some((j, adj)) = node else {
            break;
        };
        for i in adj {
            dag[i][j][0] = "-->".into();
            dag[j][i][0] = "<--".into();
            circle[i][j].clear();
            circle[j][i].clear();
        }
    }
    dag
}
