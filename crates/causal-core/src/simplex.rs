//! A dense bounded-variable primal simplex for the RPCMCI regime LP: minimise c'x subject to
//! equality and <= rows with box bounds, two phases with artificials on the equalities.
//! Requires a start at lower bounds with non-negative row activity, which the regime LP satisfies.

const EPS: f64 = 1e-9;

pub struct Lp {
    /// Sparse rows as (column, coefficient) lists.
    pub rows: Vec<Vec<(usize, f64)>>,
    pub rhs: Vec<f64>,
    /// True for equality rows, false for <=.
    pub eq: Vec<bool>,
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
    pub cost: Vec<f64>,
}

pub fn solve(lp: &Lp) -> Option<(Vec<f64>, f64)> {
    let m = lp.rows.len();
    let nvar = lp.cost.len();
    let nslack = lp.eq.iter().filter(|e| !**e).count();
    let nart = m - nslack;
    let ntot = nvar + nslack + nart;

    let mut lower = vec![0.0; ntot];
    let mut upper = vec![f64::INFINITY; ntot];
    lower[..nvar].copy_from_slice(&lp.lower);
    upper[..nvar].copy_from_slice(&lp.upper);

    let mut a = vec![vec![0.0; ntot]; m];
    let mut rhs_t = lp.rhs.clone();
    let mut basis = vec![0usize; m];
    let mut in_basis = vec![false; ntot];
    {
        let mut slack_i = 0usize;
        let mut art_i = 0usize;
        for r in 0..m {
            for &(c, v) in &lp.rows[r] {
                a[r][c] = v;
            }
            let col = if lp.eq[r] {
                let c = nvar + nslack + art_i;
                art_i += 1;
                c
            } else {
                let c = nvar + slack_i;
                slack_i += 1;
                c
            };
            a[r][col] = 1.0;
            basis[r] = col;
            in_basis[col] = true;
        }
    }

    let mut at_upper = vec![false; ntot];
    let val = |j: usize, at_upper: &[bool], lower: &[f64], upper: &[f64]| -> f64 {
        if at_upper[j] {
            upper[j]
        } else {
            lower[j]
        }
    };

    let recompute_xb = |a: &Vec<Vec<f64>>,
                        rhs_t: &Vec<f64>,
                        basis: &Vec<usize>,
                        at_upper: &Vec<bool>,
                        lower: &Vec<f64>,
                        upper: &Vec<f64>|
     -> Vec<f64> {
        let mut xb = vec![0.0; m];
        for r in 0..m {
            let mut v = rhs_t[r];
            for j in 0..ntot {
                if j != basis[r] && a[r][j] != 0.0 {
                    v -= a[r][j] * val(j, at_upper, lower, upper);
                }
            }
            xb[r] = v;
        }
        xb
    };

    let mut xb = recompute_xb(&a, &rhs_t, &basis, &at_upper, &lower, &upper);
    if xb.iter().any(|v| *v < -1e-7) {
        return None; // start is infeasible for the slack basis; not expected for the regime LP
    }

    for phase in 0..2 {
        let mut cost = vec![0.0; ntot];
        if phase == 0 {
            if nart == 0 {
                continue;
            }
            for c in cost.iter_mut().take(ntot).skip(nvar + nslack) {
                *c = 1.0;
            }
        } else {
            cost[..nvar].copy_from_slice(&lp.cost);
            for j in nvar + nslack..ntot {
                upper[j] = 0.0;
            }
        }

        // Reduced-cost row maintained incrementally through the pivots.
        let mut red = cost.clone();
        for r in 0..m {
            let cb = cost[basis[r]];
            if cb != 0.0 {
                for j in 0..ntot {
                    if a[r][j] != 0.0 {
                        red[j] -= cb * a[r][j];
                    }
                }
            }
        }
        let mut iterations = 0usize;
        loop {
            iterations += 1;
            if iterations > 200_000 {
                return None;
            }
            let mut enter = None;
            let mut best = EPS;
            for j in 0..ntot {
                if in_basis[j] || upper[j] <= lower[j] {
                    continue;
                }
                let score = if at_upper[j] { red[j] } else { -red[j] };
                if score > best {
                    best = score;
                    enter = Some(j);
                }
            }
            let Some(je) = enter else { break };
            let dir: f64 = if at_upper[je] { -1.0 } else { 1.0 };

            let span = upper[je] - lower[je];
            let mut limit = span;
            let mut leave: Option<(usize, bool)> = None;
            for r in 0..m {
                let delta = -dir * a[r][je];
                let jb = basis[r];
                if delta < -EPS {
                    let ratio = (xb[r] - lower[jb]) / (-delta);
                    if ratio < limit - EPS {
                        limit = ratio;
                        leave = Some((r, false));
                    }
                } else if delta > EPS && upper[jb].is_finite() {
                    let ratio = (upper[jb] - xb[r]) / delta;
                    if ratio < limit - EPS {
                        limit = ratio;
                        leave = Some((r, true));
                    }
                }
            }
            if limit.is_infinite() {
                return None;
            }
            match leave {
                None => {
                    // Bound flip: basics shift by the full span along the entering column.
                    for r in 0..m {
                        xb[r] -= dir * a[r][je] * span;
                    }
                    at_upper[je] = !at_upper[je];
                }
                Some((rl, to_upper)) => {
                    let jl = basis[rl];
                    let enter_val = val(je, &at_upper, &lower, &upper) + dir * limit;
                    for r in 0..m {
                        if r != rl {
                            xb[r] -= dir * a[r][je] * limit;
                        }
                    }
                    let piv = a[rl][je];
                    let inv = 1.0 / piv;
                    for v in a[rl].iter_mut() {
                        *v *= inv;
                    }
                    rhs_t[rl] *= inv;
                    let row_l = a[rl].clone();
                    let rhs_l = rhs_t[rl];
                    // Eliminations only touch the pivot row's nonzero columns; subtracting
                    // factor * 0.0 elsewhere is a no-op, so this is bit-identical and fast.
                    let nz: Vec<usize> = (0..ntot).filter(|&c| row_l[c] != 0.0).collect();
                    for r in 0..m {
                        if r != rl {
                            let factor = a[r][je];
                            if factor != 0.0 {
                                for &c in &nz {
                                    a[r][c] -= factor * row_l[c];
                                }
                                rhs_t[r] -= factor * rhs_l;
                            }
                        }
                    }
                    let rfac = red[je];
                    if rfac != 0.0 {
                        for &c in &nz {
                            red[c] -= rfac * row_l[c];
                        }
                    }
                    in_basis[jl] = false;
                    in_basis[je] = true;
                    basis[rl] = je;
                    at_upper[jl] = to_upper;
                    at_upper[je] = false;
                    xb[rl] = enter_val;
                }
            }
        }
        if phase == 0 {
            let mut infeas = 0.0;
            for r in 0..m {
                if basis[r] >= nvar + nslack {
                    infeas += xb[r].abs();
                }
            }
            if infeas > 1e-7 {
                return None;
            }
        }
    }

    let mut x = vec![0.0; nvar];
    for j in 0..nvar {
        if !in_basis[j] {
            x[j] = val(j, &at_upper, &lower, &upper);
        }
    }
    for r in 0..m {
        if basis[r] < nvar {
            x[basis[r]] = xb[r];
        }
    }
    let obj = lp.cost.iter().zip(&x).map(|(c, v)| c * v).sum();
    Some((x, obj))
}

/// The RPCMCI gamma optimisation: minimise sum res_sq * gamma with per-time simplex rows and a
/// per-regime transition budget, exactly the LP the oracle hands to GLOP.
pub fn optimize_gamma(res_sq: &[Vec<f64>], max_transitions: usize) -> Option<(Vec<Vec<f64>>, f64)> {
    let k_regimes = res_sq.len();
    let t_len = res_sq[0].len();
    let n_gamma = k_regimes * t_len;
    let n_eta = k_regimes * t_len - 1;
    let nvar = n_gamma + n_eta;

    let mut lp = Lp {
        rows: Vec::new(),
        rhs: Vec::new(),
        eq: Vec::new(),
        lower: vec![0.0; nvar],
        upper: vec![f64::INFINITY; nvar],
        cost: vec![0.0; nvar],
    };
    for k in 0..k_regimes {
        for t in 0..t_len {
            lp.cost[k * t_len + t] = res_sq[k][t];
            lp.upper[k * t_len + t] = 1.0;
        }
    }
    for t in 0..t_len {
        let row: Vec<(usize, f64)> = (0..k_regimes).map(|k| (k * t_len + t, 1.0)).collect();
        lp.rows.push(row);
        lp.rhs.push(1.0);
        lp.eq.push(true);
    }
    for k in 0..k_regimes {
        for t in 0..t_len - 1 {
            let g0 = k * t_len + t;
            let g1 = k * t_len + t + 1;
            let e = n_gamma + k * t_len + t;
            lp.rows.push(vec![(g1, 1.0), (g0, -1.0), (e, -1.0)]);
            lp.rhs.push(0.0);
            lp.eq.push(false);
            lp.rows.push(vec![(g1, -1.0), (g0, 1.0), (e, -1.0)]);
            lp.rhs.push(0.0);
            lp.eq.push(false);
        }
        let row: Vec<(usize, f64)> = (0..t_len - 1)
            .map(|t| (n_gamma + k * t_len + t, 1.0))
            .collect();
        lp.rows.push(row);
        lp.rhs.push(max_transitions as f64);
        lp.eq.push(false);
    }
    let (x, obj) = solve(&lp)?;
    let gamma = (0..k_regimes)
        .map(|k| x[k * t_len..k * t_len + t_len].to_vec())
        .collect();
    Some((gamma, obj))
}
