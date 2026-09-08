//! PCMCI (PC1 lagged parent selection + MCI) ported 1:1 from tigramite's run_pcmci with ParCorr
//! and analytic significance: same candidate order, condition choice, and symmetrisation.

use crate::missing_data::{PreprocessingError, TigramiteFrame};
use crate::parcorr::{CiKind, Node, ParCorrCi, RoleAwareSamplePolicy, TimeSeries};

pub struct PcmciResult {
    pub parents: Vec<Vec<Node>>,
    /// Indexed [i][j][tau]: link i at -tau onto j.
    pub val_matrix: Vec<Vec<Vec<f64>>>,
    pub p_matrix: Vec<Vec<Vec<f64>>>,
}

/// Stable descending sort by |val|, matching Python's sorted over dict insertion order.
fn sort_parents(vals: &[(Node, f64)]) -> Vec<Node> {
    let mut order: Vec<usize> = (0..vals.len()).collect();
    order.sort_by(|&a, &b| vals[b].1.abs().partial_cmp(&vals[a].1.abs()).unwrap());
    order.into_iter().map(|k| vals[k].0).collect()
}

pub(crate) struct Pc1Single {
    pub parents: Vec<Node>,
    /// Max p-value per initial link, with the test statistic at that maximum.
    pub pval_max: Vec<(Node, f64, f64)>,
}

pub(crate) fn pc_stable_single(
    ci: &mut ParCorrCi,
    data: &TimeSeries,
    j: usize,
    n: usize,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
) -> Pc1Single {
    // PC1 only considers lagged parents. Tigramite first maps tau_min=0 to 1, then creates
    // candidates in variable-major, lag-minor order.
    let tau_min = tau_min.max(1);
    let parents: Vec<Node> = (0..n)
        .flat_map(|i| (tau_min..=tau_max).map(move |tau| (i, -(tau as i32))))
        .collect();
    pc_stable_candidates(
        parents,
        n * (tau_max + 1).saturating_sub(tau_min),
        pc_alpha,
        |parent, z| {
            Ok::<_, std::convert::Infallible>(ci.run_test(data, &[parent], &[(j, 0)], z, tau_max))
        },
    )
    .unwrap()
}

/// Same ordered PC1 search with caller-supplied candidates and CI routine.
pub(crate) fn pc_stable_candidates<E>(
    parents: Vec<Node>,
    max_conds_dim: usize,
    pc_alpha: f64,
    test: impl FnMut(Node, &[Node]) -> Result<(f64, f64), E>,
) -> Result<Pc1Single, E> {
    pc_stable_search(parents, max_conds_dim, 1, pc_alpha, test)
}

pub(crate) fn pc_stable_search<E>(
    parents: Vec<Node>,
    max_conds_dim: usize,
    max_combinations: usize,
    pc_alpha: f64,
    mut test: impl FnMut(Node, &[Node]) -> Result<(f64, f64), E>,
) -> Result<Pc1Single, E> {
    pc_stable_decisions(parents, max_conds_dim, max_combinations, |node, z| {
        let (value, p) = test(node, z)?;
        Ok((value, p, p <= pc_alpha))
    })
}

/// Same PC-stable search, retaining the CI test's own dependence decision.
/// Fixed-threshold decisions cannot be reconstructed from their 0/1 marker
/// by comparing it with alpha (in particular when alpha is one).
pub(crate) fn pc_stable_decisions<E>(
    mut parents: Vec<Node>,
    max_conds_dim: usize,
    max_combinations: usize,
    mut test: impl FnMut(Node, &[Node]) -> Result<(f64, f64, bool), E>,
) -> Result<Pc1Single, E> {
    // Minimum |val| per surviving link, in insertion (test) order.
    let mut val_min: Vec<(Node, f64)> = Vec::new();
    let mut pval_max: Vec<(Node, f64, f64)> = Vec::new();

    for conds_dim in 0..=max_conds_dim {
        if parents.len() < conds_dim + 1 {
            break;
        }
        let mut removed: Vec<Node> = Vec::new();
        for &parent in &parents {
            let pool: Vec<Node> = parents.iter().copied().filter(|p| *p != parent).collect();
            // Default PC1 needs only the strongest subset; avoid generating an
            // exponential family when max_combinations is one.
            let conditions = if max_combinations == 1 {
                vec![pool.iter().copied().take(conds_dim).collect()]
            } else {
                crate::pcmciplus::combinations(&pool, conds_dim)
            };
            for z in conditions.into_iter().take(max_combinations) {
                let (val, pval, dependent) = test(parent, &z)?;
                match val_min.iter_mut().find(|(node, _)| *node == parent) {
                    Some(entry) => entry.1 = entry.1.min(val.abs()),
                    None => val_min.push((parent, val.abs())),
                }
                match pval_max.iter_mut().find(|(node, _, _)| *node == parent) {
                    Some(entry) => {
                        if pval > entry.1 {
                            entry.1 = pval;
                            entry.2 = val;
                        }
                    }
                    None => pval_max.push((parent, pval, val)),
                }
                if !dependent {
                    removed.push(parent);
                    break;
                }
            }
        }
        val_min.retain(|(node, _)| !removed.contains(node));
        parents = sort_parents(&val_min);
    }
    Ok(Pc1Single { parents, pval_max })
}

pub fn run_pcmci(data: &TimeSeries, tau_max: usize, pc_alpha: f64) -> PcmciResult {
    run_pcmci_with(data, tau_max, pc_alpha, CiKind::ParCorr)
}

pub fn run_pcmci_with(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
) -> PcmciResult {
    run_pcmci_filtered(data, 0, tau_max, pc_alpha, kind, None)
}

/// The MCI stage: conditions from the target's parents and the source's shifted parents.
fn mci_stage(
    ci: &mut ParCorrCi,
    data: &TimeSeries,
    parents: &[Vec<Node>],
    tau_min: usize,
    tau_max: usize,
) -> (Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>) {
    let n = data.n;
    let mut val_matrix = vec![vec![vec![0.0; tau_max + 1]; n]; n];
    let mut p_matrix = vec![vec![vec![1.0; tau_max + 1]; n]; n];
    for j in 0..n {
        let conds_y = &parents[j];
        for i in 0..n {
            for tau in tau_min as i32..=tau_max as i32 {
                if i == j && tau == 0 {
                    continue;
                }
                let x: Node = (i, -tau);
                let mut z: Vec<Node> = conds_y.iter().copied().filter(|node| *node != x).collect();
                for &(k, k_tau) in &parents[i] {
                    let shifted = (k, -tau + k_tau);
                    if !z.contains(&shifted) {
                        z.push(shifted);
                    }
                }
                let (val, pval) = ci.run_test(data, &[x], &[(j, 0)], &z, tau_max);
                val_matrix[i][j][tau as usize] = val;
                p_matrix[i][j][tau as usize] = pval;
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if i != j && p_matrix[i][j][0] >= p_matrix[j][i][0] {
                p_matrix[j][i][0] = p_matrix[i][j][0];
                val_matrix[j][i][0] = val_matrix[i][j][0];
            }
        }
    }
    (val_matrix, p_matrix)
}

/// run_pcmci with pc_alpha=None: the alpha level is chosen per variable over tigramite's
/// default list by the AIC model selection score, then MCI runs with the winning parents.
/// Returns the result, the per-variable optimal alphas, and the alpha_level graph.
pub fn run_pcmci_selected(
    data: &TimeSeries,
    tau_max: usize,
    alpha_level: f64,
) -> (PcmciResult, Vec<f64>, Vec<Vec<Vec<String>>>) {
    let alphas = [0.05, 0.1, 0.2, 0.3, 0.4, 0.5];
    let n = data.n;
    let mut ci = ParCorrCi::new();
    let mut parents: Vec<Vec<Node>> = Vec::with_capacity(n);
    let mut optimal = Vec::with_capacity(n);
    for j in 0..n {
        let mut best: Option<(f64, usize, Vec<Node>)> = None;
        for (idx, &alpha) in alphas.iter().enumerate() {
            let cand = pc_stable_single(&mut ci, data, j, n, 1, tau_max, alpha).parents;
            let score = crate::parcorr::model_selection_aic(data, j, &cand, tau_max);
            // np.argmin keeps the first minimum.
            if best.as_ref().map(|(s, _, _)| score < *s).unwrap_or(true) {
                best = Some((score, idx, cand));
            }
        }
        let (_, idx, cand) = best.unwrap();
        optimal.push(alphas[idx]);
        parents.push(cand);
    }
    let (val_matrix, p_matrix) = mci_stage(&mut ci, data, &parents, 0, tau_max);
    let mut graph = vec![vec![vec![String::new(); tau_max + 1]; n]; n];
    for i in 0..n {
        for j in 0..n {
            for tau in 0..=tau_max {
                if p_matrix[i][j][tau] <= alpha_level && !(i == j && tau == 0) {
                    if tau > 0 {
                        graph[i][j][tau] = "-->".into();
                    } else if p_matrix[j][i][0] <= alpha_level {
                        graph[i][j][0] = "o-o".into();
                    } else {
                        graph[i][j][0] = "-->".into();
                        graph[j][i][0] = "<--".into();
                    }
                }
            }
        }
    }
    (
        PcmciResult {
            parents,
            val_matrix,
            p_matrix,
        },
        optimal,
        graph,
    )
}

/// run_bivci: each pair tested conditional on the target's own past, val_only.
pub fn run_bivci(data: &TimeSeries, tau_max: usize) -> Vec<Vec<Vec<f64>>> {
    let n = data.n;
    let mut ci = ParCorrCi::new();
    let auto_past: Vec<Vec<Node>> = (0..n)
        .map(|j| (1..=tau_max as i32).map(|tau| (j, -tau)).collect())
        .collect();
    let mut val_matrix = vec![vec![vec![0.0; tau_max + 1]; n]; n];
    for j in 0..n {
        for i in 0..n {
            for tau in 0..=tau_max as i32 {
                if i == j && tau == 0 {
                    continue;
                }
                let x: Node = (i, -tau);
                let z: Vec<Node> = auto_past[j]
                    .iter()
                    .copied()
                    .filter(|node| *node != x)
                    .collect();
                let (val, _pval) = ci.run_test(data, &[x], &[(j, 0)], &z, tau_max);
                val_matrix[i][j][tau as usize] = val;
            }
        }
    }
    // val_only returns before the symmetrization step, so both lag zero directions keep
    // their own value.
    val_matrix
}

/// run_pcmci with a tau_min and an optional sample keep-filter (RPCMCI's regime masking).
pub fn run_pcmci_filtered(
    data: &TimeSeries,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
    filter: Option<Vec<bool>>,
) -> PcmciResult {
    // One CI cache across the PC and MCI stages, as one cond_ind_test serves both in the oracle.
    let mut ci = ParCorrCi::with_kind(kind);
    ci.sample_filter = filter;
    run_pcmci_with_ci(data, tau_min, tau_max, pc_alpha, &mut ci)
}

/// PCMCI over a nullable frame using Tigramite's role-aware sample construction.
///
/// Missing selected values are always excluded. The analysis mask is applied only to the
/// configured X/Y/Z roles. No partial result is returned if any CI test has no usable sample.
pub fn run_pcmci_frame(
    frame: TigramiteFrame,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
    sample_policy: RoleAwareSamplePolicy,
) -> Result<PcmciResult, PreprocessingError> {
    let data = frame.data.clone();
    let mut ci = ParCorrCi::with_frame(kind, frame, sample_policy);
    let result = run_pcmci_with_ci(&data, tau_min, tau_max, pc_alpha, &mut ci);
    match ci.preprocessing_error().cloned() {
        Some(error) => Err(error),
        None => Ok(result),
    }
}

fn run_pcmci_with_ci(
    data: &TimeSeries,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    ci: &mut ParCorrCi,
) -> PcmciResult {
    let n = data.n;
    let parents: Vec<Vec<Node>> = (0..n)
        .map(|j| pc_stable_single(ci, data, j, n, tau_min, tau_max, pc_alpha).parents)
        .collect();
    let (val_matrix, p_matrix) = mci_stage(ci, data, &parents, tau_min, tau_max);
    PcmciResult {
        parents,
        val_matrix,
        p_matrix,
    }
}
