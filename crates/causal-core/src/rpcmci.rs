//! Tigramite-compatible RPCMCI with seeded annealings over regime assignments, masked PCMCI per
//! regime, the default standardized linear prediction model, and the GLOP regime LP.

use crate::missing_data::TigramiteFrame;
use crate::nprandom::NpRng;
use crate::parcorr::{CiKind, Node, TimeSeries};
use crate::pcmci::run_pcmci_filtered;
use crate::simplex::optimize_gamma;
use crate::sklearn_linear::{fit_sklearn_linear_regression, SKLEARN_LINEAR_TOLERANCE};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq)]
pub struct RpcmciResult {
    /// Soft regime memberships, shape (num_regimes, T).
    pub regimes: Vec<Vec<f64>>,
    /// Per regime: string graph, val and p matrices from the final masked PCMCI.
    pub graphs: Vec<Vec<Vec<Vec<String>>>>,
    pub val_matrices: Vec<Vec<Vec<Vec<f64>>>>,
    pub p_matrices: Vec<Vec<Vec<Vec<f64>>>>,
    /// Per-regime PCMCI confidence matrices. The exposed analytic CI tests do not request
    /// confidence estimation, so these correspond to Tigramite's `conf_matrix=None` values.
    pub conf_matrices: Vec<Option<Vec<Vec<Vec<[f64; 2]>>>>>,
    /// Per-annealing convergence histories and the history of the winning annealing.
    /// This is Tigramite's `diff_g_f` tuple without changing its structure.
    pub diff_g_f: (Vec<Option<Vec<f64>>>, Vec<f64>),
    pub error_free_annealings: usize,
}

struct RegimeDiscovery {
    parents: Vec<Vec<Node>>,
    p_matrix: Vec<Vec<Vec<f64>>>,
    val_matrix: Vec<Vec<Vec<f64>>>,
    graph: Vec<Vec<Vec<String>>>,
}

fn discover(
    data: &TimeSeries,
    filter: &[bool],
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    alpha_level: f64,
    kind: CiKind,
) -> RegimeDiscovery {
    let n = data.n;
    let res = run_pcmci_filtered(
        data,
        tau_min,
        tau_max,
        pc_alpha,
        kind,
        Some(filter.to_vec()),
    );
    // Graph: p <= alpha_level; lagged links become -->, contemporaneous mutual links o-o.
    let mut graph = vec![vec![vec![String::new(); tau_max + 1]; n]; n];
    for i in 0..n {
        for j in 0..n {
            for tau in 1..=tau_max {
                if res.p_matrix[i][j][tau] <= alpha_level {
                    graph[i][j][tau] = "-->".into();
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if i != j && res.p_matrix[i][j][0] <= alpha_level {
                if res.p_matrix[j][i][0] <= alpha_level {
                    graph[i][j][0] = "o-o".into();
                } else {
                    graph[i][j][0] = "-->".into();
                    graph[j][i][0] = "<--".into();
                }
            }
        }
    }
    // Parents: lagged --> links sorted by |val| descending, stable over (i, tau) insertion.
    let mut parents = Vec::with_capacity(n);
    for j in 0..n {
        let mut links: Vec<(Node, f64)> = Vec::new();
        for i in 0..n {
            for tau in 1..=tau_max {
                if graph[i][j][tau] == "-->" {
                    links.push(((i, -(tau as i32)), res.val_matrix[i][j][tau].abs()));
                }
            }
        }
        let mut order: Vec<usize> = (0..links.len()).collect();
        order.sort_by(|&a, &b| links[b].1.partial_cmp(&links[a].1).unwrap());
        parents.push(order.into_iter().map(|k| links[k].0).collect());
    }
    RegimeDiscovery {
        parents,
        p_matrix: res.p_matrix,
        val_matrix: res.val_matrix,
        graph,
    }
}

/// tigramite Prediction with StandardScaler + LinearRegression: train on the regime samples,
/// predict over all samples in the transformed scale, residual per target.
fn prediction_residuals(
    data: &TimeSeries,
    filter: &[bool],
    parents: &[Vec<Node>],
    tau_max: usize,
) -> Option<Vec<Vec<f64>>> {
    let t_len = data.t;
    let n = data.n;
    let test_len = t_len - tau_max;
    let mut residuals = vec![vec![0.0; test_len]; n];
    let at = |t: usize, v: usize| data.values[t * n + v];

    for j in 0..n {
        let pars = &parents[j];
        let train: Vec<usize> = (tau_max..t_len).filter(|&t| filter[t]).collect();
        if train.is_empty() {
            return None;
        }
        // Rows: [X dummy (j,0), Y (j,0), parents...]; scaler over train samples per row.
        let mut rows: Vec<Vec<f64>> = Vec::with_capacity(2 + pars.len());
        rows.push(train.iter().map(|&t| at(t, j)).collect());
        rows.push(train.iter().map(|&t| at(t, j)).collect());
        for &(v, lag) in pars {
            rows.push(
                train
                    .iter()
                    .map(|&t| at((t as i64 + lag as i64) as usize, v))
                    .collect(),
            );
        }
        let stats: Vec<(f64, f64)> = rows
            .iter()
            .map(|row| {
                let m = row.iter().sum::<f64>() / row.len() as f64;
                let s =
                    (row.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / row.len() as f64).sqrt();
                (m, if s == 0.0 { 1.0 } else { s })
            })
            .collect();

        let ns = train.len();
        let mut x_t = DMatrix::<f64>::zeros(ns, pars.len());
        for (column, row) in rows[2..].iter().enumerate() {
            for (sample, value) in row.iter().enumerate() {
                x_t[(sample, column)] = (value - stats[2 + column].0) / stats[2 + column].1;
            }
        }
        let y_t = DVector::from_iterator(
            ns,
            rows[1]
                .iter()
                .map(|value| (value - stats[1].0) / stats[1].1),
        );
        let fit = fit_sklearn_linear_regression(&x_t, &y_t, SKLEARN_LINEAR_TOLERANCE).ok()?;

        // Predict over all test samples with the train-fitted scaler.
        for (idx, t) in (tau_max..t_len).enumerate() {
            let y_obs = (at(t, j) - stats[1].0) / stats[1].1;
            let mut pred = fit.intercept;
            for (c, &(v, lag)) in pars.iter().enumerate() {
                let raw = at((t as i64 + lag as i64) as usize, v);
                pred += fit.coefficients[c] * (raw - stats[2 + c].0) / stats[2 + c].1;
            }
            residuals[j][idx] = y_obs - pred;
        }
    }
    Some(residuals)
}

#[derive(Clone, Copy)]
struct RpcmciOptions {
    num_regimes: usize,
    max_transitions: usize,
    switch_thres: f64,
    num_iterations: usize,
    max_anneal: usize,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    alpha_level: f64,
    seed: u64,
    kind: CiKind,
}

fn run_rpcmci_inner<F>(
    data: TimeSeries,
    options: RpcmciOptions,
    mut progress: F,
) -> Option<RpcmciResult>
where
    F: FnMut(&'static str, usize, usize),
{
    let RpcmciOptions {
        num_regimes,
        max_transitions,
        switch_thres,
        num_iterations,
        max_anneal,
        tau_min,
        tau_max,
        pc_alpha,
        alpha_level,
        seed,
        kind,
    } = options;
    let t_len = data.t;
    if num_regimes == 0
        || max_anneal == 0
        || tau_min > tau_max
        || tau_max >= t_len
        || !(0.0..=1.0).contains(&pc_alpha)
    {
        return None;
    }
    let q_break_cycle = 5;

    struct Annealing {
        objective: f64,
        gamma: Vec<Vec<f64>>,
        results: Vec<RegimeDiscovery>,
        diff_g: Vec<f64>,
    }

    let mut annealings: Vec<Option<Annealing>> = Vec::with_capacity(max_anneal);
    for a in 0..max_anneal {
        let mut rng = NpRng::seeded(seed + a as u64);
        let mut gamma_opt: Vec<Vec<f64>> = (0..num_regimes)
            .map(|_| (0..t_len).map(|_| rng.next_f64()).collect())
            .collect();
        let mut results_opt: Vec<Option<RegimeDiscovery>> =
            (0..num_regimes).map(|_| None).collect();
        let mut objective_opt = 0.0;
        let mut error_flag = false;
        let mut diff_last;
        let mut diff_g = Vec::with_capacity(num_iterations);

        for q in 0..num_iterations {
            let gamma_temp = gamma_opt.clone();
            let mut res_sq = vec![vec![0.0; t_len]; num_regimes];

            for k in 0..num_regimes {
                let filter: Vec<bool> = (0..t_len)
                    .map(|t| gamma_temp[k][t] > switch_thres)
                    .collect();
                if filter.iter().filter(|s| **s).count() <= 5 {
                    error_flag = true;
                    break;
                }
                let disc = discover(
                    &data,
                    &filter,
                    tau_min,
                    tau_max,
                    pc_alpha,
                    alpha_level,
                    kind,
                );
                let resid = match prediction_residuals(&data, &filter, &disc.parents, tau_max) {
                    Some(r) => r,
                    None => {
                        error_flag = true;
                        break;
                    }
                };
                results_opt[k] = Some(disc);
                for (idx, t) in (tau_max..t_len).enumerate() {
                    let mut sq = 0.0;
                    for row in resid.iter() {
                        sq += row[idx] * row[idx];
                    }
                    res_sq[k][t] = sq;
                }
            }
            if error_flag {
                break;
            }

            let Some((gamma_new, obj)) = optimize_gamma(&res_sq, max_transitions) else {
                error_flag = true;
                break;
            };
            gamma_opt = gamma_new;
            objective_opt = obj;

            let diff: f64 = gamma_opt
                .iter()
                .zip(&gamma_temp)
                .flat_map(|(a, b)| a.iter().zip(b).map(|(x, y)| (x - y).abs()))
                .sum();
            diff_last = diff;
            diff_g.push(diff);
            progress(
                "annealing",
                a * num_iterations + q + 1,
                max_anneal * num_iterations,
            );
            if diff_last == 0.0 {
                break;
            }
            if q >= q_break_cycle && diff_last <= (2 * num_regimes * t_len / 100) as f64 {
                break;
            }
        }
        if error_flag {
            annealings.push(None);
        } else {
            annealings.push(Some(Annealing {
                objective: objective_opt,
                gamma: gamma_opt,
                results: results_opt
                    .into_iter()
                    .map(|r| r.expect("regime results"))
                    .collect(),
                diff_g,
            }));
        }
    }

    let error_free = annealings.iter().filter(|a| a.is_some()).count();
    if error_free == 0 {
        return None;
    }
    let mut best = None;
    let mut best_obj = f64::INFINITY;
    for (a, ann) in annealings.iter().enumerate() {
        if let Some(ann) = ann {
            if ann.objective < best_obj {
                best_obj = ann.objective;
                best = Some(a);
            }
        }
    }
    let diff_g_all: Vec<Option<Vec<f64>>> = annealings
        .iter()
        .map(|annealing| annealing.as_ref().map(|value| value.diff_g.clone()))
        .collect();
    let diff_g_best = annealings[best?].as_ref()?.diff_g.clone();
    let best = annealings.into_iter().nth(best?).flatten()?;
    progress(
        "complete",
        max_anneal * num_iterations,
        max_anneal * num_iterations,
    );
    let conf_matrices = vec![None; best.results.len()];
    Some(RpcmciResult {
        regimes: best.gamma,
        graphs: best.results.iter().map(|r| r.graph.clone()).collect(),
        val_matrices: best.results.iter().map(|r| r.val_matrix.clone()).collect(),
        p_matrices: best.results.iter().map(|r| r.p_matrix.clone()).collect(),
        conf_matrices,
        diff_g_f: (diff_g_all, diff_g_best),
        error_free_annealings: error_free,
    })
}

/// Tigramite RPCMCI with its default `StandardScaler` and `LinearRegression` prediction model.
/// `kind` selects the conditional-independence test; it does not replace the prediction model.
#[allow(clippy::too_many_arguments)]
pub fn run_rpcmci(
    data_rows: &[Vec<f64>],
    num_regimes: usize,
    max_transitions: usize,
    switch_thres: f64,
    num_iterations: usize,
    max_anneal: usize,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    alpha_level: f64,
    seed: u64,
    kind: CiKind,
) -> Option<RpcmciResult> {
    if data_rows.is_empty()
        || data_rows[0].is_empty()
        || data_rows.iter().any(|row| row.len() != data_rows[0].len())
        || data_rows.iter().flatten().any(|value| value.is_nan())
    {
        return None;
    }
    run_rpcmci_inner(
        TimeSeries::new(data_rows.to_vec()),
        RpcmciOptions {
            num_regimes,
            max_transitions,
            switch_thres,
            num_iterations,
            max_anneal,
            tau_min,
            tau_max,
            pc_alpha,
            alpha_level,
            seed,
            kind,
        },
        |_, _, _| {},
    )
}

/// RPCMCI with progress reports that do not alter annealing or random-number order.
#[allow(clippy::too_many_arguments)]
pub fn run_rpcmci_with_progress<F>(
    data_rows: &[Vec<f64>],
    num_regimes: usize,
    max_transitions: usize,
    switch_thres: f64,
    num_iterations: usize,
    max_anneal: usize,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    alpha_level: f64,
    seed: u64,
    kind: CiKind,
    progress: F,
) -> Option<RpcmciResult>
where
    F: FnMut(&'static str, usize, usize),
{
    if data_rows.is_empty()
        || data_rows[0].is_empty()
        || data_rows.iter().any(|row| row.len() != data_rows[0].len())
        || data_rows.iter().flatten().any(|value| value.is_nan())
    {
        return None;
    }
    run_rpcmci_inner(
        TimeSeries::new(data_rows.to_vec()),
        RpcmciOptions {
            num_regimes,
            max_transitions,
            switch_thres,
            num_iterations,
            max_anneal,
            tau_min,
            tau_max,
            pc_alpha,
            alpha_level,
            seed,
            kind,
        },
        progress,
    )
}

/// Nullable RPCMCI entry point corresponding to Tigramite's `missing_flag` constructor path.
/// Tigramite 5.2.10.1 fails that path when prediction reconstructs a DataFrame from its internal
/// NaNs, so an invalid cell makes the run fail rather than being silently treated as a number.
#[allow(clippy::too_many_arguments)]
pub fn run_rpcmci_frame(
    frame: &TigramiteFrame,
    num_regimes: usize,
    max_transitions: usize,
    switch_thres: f64,
    num_iterations: usize,
    max_anneal: usize,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    alpha_level: f64,
    seed: u64,
    kind: CiKind,
) -> Option<RpcmciResult> {
    if frame.validity.iter().any(|valid| !valid) {
        return None;
    }
    run_rpcmci_inner(
        frame.data.clone(),
        RpcmciOptions {
            num_regimes,
            max_transitions,
            switch_thres,
            num_iterations,
            max_anneal,
            tau_min,
            tau_max,
            pc_alpha,
            alpha_level,
            seed,
            kind,
        },
        |_, _, _| {},
    )
}
