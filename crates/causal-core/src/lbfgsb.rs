//! L-BFGS-B ported 1:1 from the Fortran v3.0 sources that scipy wraps (Byrd, Lu, Nocedal, Zhu;
//! Morales, Nocedal), with 1-based indexing preserved throughout. The reverse-communication
//! driver is folded into a closure-driven loop with identical control flow.

const ONE: f64 = 1.0;
const ZERO: f64 = 0.0;

/// 1-based column-major matrix view.
#[derive(Clone)]
struct Mat {
    d: Vec<f64>,
    ld: usize,
}

impl Mat {
    fn new(ld: usize, cols: usize) -> Self {
        Mat {
            d: vec![0.0; ld * cols + 1],
            ld,
        }
    }
    #[inline]
    fn g(&self, i: usize, j: usize) -> f64 {
        self.d[(j - 1) * self.ld + i]
    }
    #[inline]
    fn s(&mut self, i: usize, j: usize, v: f64) {
        self.d[(j - 1) * self.ld + i] = v;
    }
}

fn ddot(n: usize, a: &[f64], b: &[f64]) -> f64 {
    let mut s = 0.0;
    for k in 1..=n {
        s += a[k] * b[k];
    }
    s
}

/// LINPACK dpofa on the leading n x n block, upper-triangular factor in place.
fn dpofa(a: &mut Mat, n: usize) -> i32 {
    for j in 1..=n {
        let mut s = 0.0;
        for k in 1..j {
            let mut t = a.g(k, j);
            for l in 1..k {
                t -= a.g(l, k) * a.g(l, j);
            }
            t /= a.g(k, k);
            a.s(k, j, t);
            s += t * t;
        }
        let s = a.g(j, j) - s;
        if s <= ZERO {
            return j as i32;
        }
        a.s(j, j, s.sqrt());
    }
    0
}

/// LINPACK dtrsl on the leading n x n block of t, solving into b[off+1 ..= off+n].
/// job 01: t upper, t*x = b. job 11: t upper, trans(t)*x = b.
fn dtrsl(t: &Mat, n: usize, b: &mut [f64], off: usize, job: u8) -> i32 {
    for k in 1..=n {
        if t.g(k, k) == ZERO {
            return k as i32;
        }
    }
    match job {
        11 => {
            b[off + 1] /= t.g(1, 1);
            for j in 2..=n {
                let mut s = b[off + j];
                for k in 1..j {
                    s -= t.g(k, j) * b[off + k];
                }
                b[off + j] = s / t.g(j, j);
            }
        }
        1 => {
            b[off + n] /= t.g(n, n);
            for jj in 2..=n {
                let j = n - jj + 1;
                let mut s = b[off + j];
                for k in j + 1..=n {
                    s -= t.g(j, k) * b[off + k];
                }
                b[off + j] = s / t.g(j, j);
            }
        }
        _ => unreachable!(),
    }
    0
}

/// bmv: solve the middle system of the compact representation.
fn bmv(m: usize, sy: &Mat, wt: &Mat, col: usize, v: &[f64], p: &mut [f64]) -> i32 {
    let _ = m;
    if col == 0 {
        return 0;
    }
    p[col + 1] = v[col + 1];
    for i in 2..=col {
        let i2 = col + i;
        let mut sum = 0.0;
        for k in 1..i {
            sum += sy.g(i, k) * v[k] / sy.g(k, k);
        }
        p[i2] = v[i2] + sum;
    }
    let info = dtrsl(wt, col, p, col, 11);
    if info != 0 {
        return info;
    }
    for i in 1..=col {
        p[i] = v[i] / sy.g(i, i).sqrt();
    }
    let info = dtrsl(wt, col, p, col, 1);
    if info != 0 {
        return info;
    }
    for i in 1..=col {
        p[i] = -p[i] / sy.g(i, i).sqrt();
    }
    for i in 1..=col {
        let mut sum = 0.0;
        for k in i + 1..=col {
            sum += sy.g(k, i) * p[col + k] / sy.g(i, i);
        }
        p[i] += sum;
    }
    0
}

fn projgr(n: usize, l: &[f64], u: &[f64], nbd: &[i32], x: &[f64], g: &[f64]) -> f64 {
    let mut sbgnrm = ZERO;
    for i in 1..=n {
        let mut gi = g[i];
        if gi.is_nan() {
            return gi;
        }
        if nbd[i] != 0 {
            if gi < ZERO {
                if nbd[i] >= 2 {
                    gi = (x[i] - u[i]).max(gi);
                }
            } else if nbd[i] <= 2 {
                gi = (x[i] - l[i]).min(gi);
            }
        }
        sbgnrm = sbgnrm.max(gi.abs());
    }
    sbgnrm
}

fn hpsolb(n: usize, t: &mut [f64], iorder: &mut [usize], iheap: i32) {
    if iheap == 0 {
        for k in 2..=n {
            let ddum = t[k];
            let indxin = iorder[k];
            let mut i = k;
            while i > 1 {
                let j = i / 2;
                if ddum < t[j] {
                    t[i] = t[j];
                    iorder[i] = iorder[j];
                    i = j;
                } else {
                    break;
                }
            }
            t[i] = ddum;
            iorder[i] = indxin;
        }
    }
    if n > 1 {
        let mut i = 1;
        let out = t[1];
        let indxou = iorder[1];
        let ddum = t[n];
        let indxin = iorder[n];
        loop {
            let mut j = i + i;
            if j <= n - 1 {
                if t[j + 1] < t[j] {
                    j += 1;
                }
                if t[j] < ddum {
                    t[i] = t[j];
                    iorder[i] = iorder[j];
                    i = j;
                    continue;
                }
            }
            break;
        }
        t[i] = ddum;
        iorder[i] = indxin;
        t[n] = out;
        iorder[n] = indxou;
    }
}

pub(crate) struct Dcsrch {
    gtol: f64,
    xtol: f64,
    brackt: bool,
    stage: i32,
    finit: f64,
    ginit: f64,
    gtest: f64,
    width: f64,
    width1: f64,
    stx: f64,
    fx: f64,
    gx: f64,
    sty: f64,
    fy: f64,
    gy: f64,
    stmin: f64,
    stmax: f64,
}

#[derive(PartialEq)]
pub(crate) enum SrchTask {
    Fg,
    Convergence,
    Warning,
    Error,
}

impl Dcsrch {
    const XTRAPL: f64 = 1.1;
    const XTRAPU: f64 = 4.0;
    // L-BFGS-B's own tolerances; scipy's line_search_wolfe1 passes its c1, c2 and xtol.
    const FTOL: f64 = 1.0e-3;
    const GTOL: f64 = 0.9;
    const XTOL: f64 = 0.1;

    fn start(f: f64, g: f64, stp: f64, stpmax: f64) -> Result<Dcsrch, SrchTask> {
        Self::start_with(f, g, stp, stpmax, Self::FTOL, Self::GTOL, Self::XTOL)
    }

    pub(crate) fn start_with(
        f: f64,
        g: f64,
        stp: f64,
        stpmax: f64,
        ftol: f64,
        gtol: f64,
        xtol: f64,
    ) -> Result<Dcsrch, SrchTask> {
        if g >= ZERO || stp > stpmax {
            return Err(SrchTask::Error);
        }
        Ok(Dcsrch {
            gtol,
            xtol,
            brackt: false,
            stage: 1,
            finit: f,
            ginit: g,
            gtest: ftol * g,
            width: stpmax,
            width1: stpmax / 0.5,
            stx: ZERO,
            fx: f,
            gx: g,
            sty: ZERO,
            fy: f,
            gy: g,
            stmin: ZERO,
            stmax: stp + Self::XTRAPU * stp,
        })
    }

    pub(crate) fn step(
        &mut self,
        f: f64,
        g: f64,
        stp: &mut f64,
        stpmin: f64,
        stpmax_arg: f64,
    ) -> SrchTask {
        let ftest = self.finit + *stp * self.gtest;
        if self.stage == 1 && f <= ftest && g >= ZERO {
            self.stage = 2;
        }
        let mut task = SrchTask::Fg;
        if self.brackt && (*stp <= self.stmin || *stp >= self.stmax) {
            task = SrchTask::Warning;
        }
        if self.brackt && self.stmax - self.stmin <= self.xtol * self.stmax {
            task = SrchTask::Warning;
        }
        if *stp == stpmax_arg && f <= ftest && g <= self.gtest {
            task = SrchTask::Warning;
        }
        if *stp == stpmin && (f > ftest || g >= self.gtest) {
            task = SrchTask::Warning;
        }
        if f <= ftest && g.abs() <= self.gtol * (-self.ginit) {
            task = SrchTask::Convergence;
        }
        if task == SrchTask::Warning || task == SrchTask::Convergence {
            return task;
        }

        if self.stage == 1 && f <= self.fx && f > ftest {
            let fm = f - *stp * self.gtest;
            let mut fxm = self.fx - self.stx * self.gtest;
            let mut fym = self.fy - self.sty * self.gtest;
            let gm = g - self.gtest;
            let mut gxm = self.gx - self.gtest;
            let mut gym = self.gy - self.gtest;
            dcstep(
                &mut self.stx,
                &mut fxm,
                &mut gxm,
                &mut self.sty,
                &mut fym,
                &mut gym,
                stp,
                fm,
                gm,
                &mut self.brackt,
                self.stmin,
                self.stmax,
            );
            self.fx = fxm + self.stx * self.gtest;
            self.fy = fym + self.sty * self.gtest;
            self.gx = gxm + self.gtest;
            self.gy = gym + self.gtest;
        } else {
            dcstep(
                &mut self.stx,
                &mut self.fx,
                &mut self.gx,
                &mut self.sty,
                &mut self.fy,
                &mut self.gy,
                stp,
                f,
                g,
                &mut self.brackt,
                self.stmin,
                self.stmax,
            );
        }
        if self.brackt {
            if (self.sty - self.stx).abs() >= 0.66 * self.width1 {
                *stp = self.stx + 0.5 * (self.sty - self.stx);
            }
            self.width1 = self.width;
            self.width = (self.sty - self.stx).abs();
        }
        if self.brackt {
            self.stmin = self.stx.min(self.sty);
            self.stmax = self.stx.max(self.sty);
        } else {
            self.stmin = *stp + Self::XTRAPL * (*stp - self.stx);
            self.stmax = *stp + Self::XTRAPU * (*stp - self.stx);
        }
        *stp = stp.max(stpmin);
        *stp = stp.min(stpmax_arg);
        if (self.brackt && (*stp <= self.stmin || *stp >= self.stmax))
            || (self.brackt && self.stmax - self.stmin <= self.xtol * self.stmax)
        {
            *stp = self.stx;
        }
        SrchTask::Fg
    }
}

#[allow(clippy::too_many_arguments)]
fn dcstep(
    stx: &mut f64,
    fx: &mut f64,
    dx: &mut f64,
    sty: &mut f64,
    fy: &mut f64,
    dy: &mut f64,
    stp: &mut f64,
    fp: f64,
    dp: f64,
    brackt: &mut bool,
    stpmin: f64,
    stpmax: f64,
) {
    let sgnd = dp * (*dx / dx.abs());
    let stpf;
    if fp > *fx {
        let theta = 3.0 * (*fx - fp) / (*stp - *stx) + *dx + dp;
        let s = theta.abs().max(dx.abs()).max(dp.abs());
        let mut gamma = s * ((theta / s).powi(2) - (*dx / s) * (dp / s)).sqrt();
        if *stp < *stx {
            gamma = -gamma;
        }
        let p = (gamma - *dx) + theta;
        let q = ((gamma - *dx) + gamma) + dp;
        let r = p / q;
        let stpc = *stx + r * (*stp - *stx);
        let stpq = *stx + ((*dx / ((*fx - fp) / (*stp - *stx) + *dx)) / 2.0) * (*stp - *stx);
        if (stpc - *stx).abs() < (stpq - *stx).abs() {
            stpf = stpc;
        } else {
            stpf = stpc + (stpq - stpc) / 2.0;
        }
        *brackt = true;
    } else if sgnd < ZERO {
        let theta = 3.0 * (*fx - fp) / (*stp - *stx) + *dx + dp;
        let s = theta.abs().max(dx.abs()).max(dp.abs());
        let mut gamma = s * ((theta / s).powi(2) - (*dx / s) * (dp / s)).sqrt();
        if *stp > *stx {
            gamma = -gamma;
        }
        let p = (gamma - dp) + theta;
        let q = ((gamma - dp) + gamma) + *dx;
        let r = p / q;
        let stpc = *stp + r * (*stx - *stp);
        let stpq = *stp + (dp / (dp - *dx)) * (*stx - *stp);
        if (stpc - *stp).abs() > (stpq - *stp).abs() {
            stpf = stpc;
        } else {
            stpf = stpq;
        }
        *brackt = true;
    } else if dp.abs() < dx.abs() {
        let theta = 3.0 * (*fx - fp) / (*stp - *stx) + *dx + dp;
        let s = theta.abs().max(dx.abs()).max(dp.abs());
        let mut gamma = s * (ZERO.max((theta / s).powi(2) - (*dx / s) * (dp / s))).sqrt();
        if *stp > *stx {
            gamma = -gamma;
        }
        let p = (gamma - dp) + theta;
        let q = (gamma + (*dx - dp)) + gamma;
        let r = p / q;
        let stpc = if r < ZERO && gamma != ZERO {
            *stp + r * (*stx - *stp)
        } else if *stp > *stx {
            stpmax
        } else {
            stpmin
        };
        let stpq = *stp + (dp / (dp - *dx)) * (*stx - *stp);
        if *brackt {
            let mut f = if (stpc - *stp).abs() < (stpq - *stp).abs() {
                stpc
            } else {
                stpq
            };
            if *stp > *stx {
                f = f.min(*stp + 0.66 * (*sty - *stp));
            } else {
                f = f.max(*stp + 0.66 * (*sty - *stp));
            }
            stpf = f;
        } else {
            let mut f = if (stpc - *stp).abs() > (stpq - *stp).abs() {
                stpc
            } else {
                stpq
            };
            f = f.min(stpmax);
            f = f.max(stpmin);
            stpf = f;
        }
    } else if *brackt {
        let theta = 3.0 * (fp - *fy) / (*sty - *stp) + *dy + dp;
        let s = theta.abs().max(dy.abs()).max(dp.abs());
        let mut gamma = s * ((theta / s).powi(2) - (*dy / s) * (dp / s)).sqrt();
        if *stp > *sty {
            gamma = -gamma;
        }
        let p = (gamma - dp) + theta;
        let q = ((gamma - dp) + gamma) + *dy;
        let r = p / q;
        stpf = *stp + r * (*sty - *stp);
    } else if *stp > *stx {
        stpf = stpmax;
    } else {
        stpf = stpmin;
    }

    if fp > *fx {
        *sty = *stp;
        *fy = fp;
        *dy = dp;
    } else {
        if sgnd < ZERO {
            *sty = *stx;
            *fy = *fx;
            *dy = *dx;
        }
        *stx = *stp;
        *fx = fp;
        *dx = dp;
    }
    *stp = stpf;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LbfgsbTermination {
    ProjectedGradient,
    FunctionTolerance,
    IterationLimit,
    LineSearchFailed,
}

impl LbfgsbTermination {
    pub fn converged(self) -> bool {
        matches!(self, Self::ProjectedGradient | Self::FunctionTolerance)
    }
}

pub struct LbfgsbResult {
    pub x: Vec<f64>,
    pub f: f64,
    pub iterations: usize,
    pub termination: LbfgsbTermination,
}

/// The mainlb driver at scipy defaults semantics: closure evaluates (f, g); bounds are encoded
/// via nbd exactly as the Fortran expects (0 free, 1 lower, 2 both, 3 upper).
#[allow(clippy::too_many_arguments)]
pub fn lbfgsb<F>(
    x0: &[f64],
    l0: &[f64],
    u0: &[f64],
    nbd0: &[i32],
    m: usize,
    factr: f64,
    pgtol: f64,
    maxls: usize,
    max_iter: usize,
    f_and_grad: F,
) -> LbfgsbResult
where
    F: FnMut(&[f64]) -> (f64, Vec<f64>),
{
    lbfgsb_driver(
        x0, l0, u0, nbd0, m, factr, pgtol, maxls, max_iter, false, f_and_grad,
    )
}

/// R's pre-3.0 feasible-subspace step, sharing the remaining solver machinery.
/// This entry point does not change the SciPy-compatible solver above.
/// Subspace adaptation follows R src/appl/lbfgsb.c::subsm (R Core Team,
/// GPL-2.0-or-later), rather than the 2011 projected-step safeguard.
#[allow(clippy::too_many_arguments)]
pub fn lbfgsb_r<F>(
    x0: &[f64],
    l0: &[f64],
    u0: &[f64],
    nbd0: &[i32],
    m: usize,
    factr: f64,
    pgtol: f64,
    maxls: usize,
    max_iter: usize,
    f_and_grad: F,
) -> LbfgsbResult
where
    F: FnMut(&[f64]) -> (f64, Vec<f64>),
{
    lbfgsb_driver(
        x0, l0, u0, nbd0, m, factr, pgtol, maxls, max_iter, true, f_and_grad,
    )
}

#[allow(clippy::too_many_arguments)]
fn lbfgsb_driver<F>(
    x0: &[f64],
    l0: &[f64],
    u0: &[f64],
    nbd0: &[i32],
    m: usize,
    factr: f64,
    pgtol: f64,
    maxls: usize,
    max_iter: usize,
    r_subspace: bool,
    mut f_and_grad: F,
) -> LbfgsbResult
where
    F: FnMut(&[f64]) -> (f64, Vec<f64>),
{
    let n = x0.len();
    // 1-based copies.
    let mut x = vec![0.0; n + 1];
    let mut l = vec![0.0; n + 1];
    let mut u = vec![0.0; n + 1];
    let mut nbd = vec![0i32; n + 1];
    x[1..].copy_from_slice(x0);
    l[1..].copy_from_slice(l0);
    u[1..].copy_from_slice(u0);
    nbd[1..].copy_from_slice(nbd0);

    let epsmch = f64::EPSILON;
    let tol = factr * epsmch;

    let mut ws = Mat::new(n, m);
    let mut wy = Mat::new(n, m);
    let mut sy = Mat::new(m, m);
    let mut ss = Mat::new(m, m);
    let mut wt = Mat::new(m, m);
    let mut wn = Mat::new(2 * m, 2 * m);
    let mut snd = Mat::new(2 * m, 2 * m);
    let mut z = vec![0.0; n + 1];
    let mut r = vec![0.0; n + 1];
    let mut d = vec![0.0; n + 1];
    let mut t = vec![0.0; n + 1];
    let mut xp = vec![0.0; n + 1];
    let mut wa = vec![0.0; 8 * m + 1];
    let mut index = vec![0usize; n + 1];
    let mut iwhere = vec![0i32; n + 1];
    let mut indx2 = vec![0usize; n + 1];

    let mut col = 0usize;
    let mut head = 1usize;
    let mut theta = ONE;
    let mut iupdat = 0usize;
    let mut updatd = false;
    let mut itail = 0usize;
    let mut nact;
    let mut ileave = 0usize;
    let mut nenter = 0usize;
    let mut iter = 0usize;
    let mut nfree = n;
    let mut cnstnd = false;
    let mut boxed = true;

    // active: project x into the box, set iwhere.
    for i in 1..=n {
        if nbd[i] > 0 {
            if nbd[i] <= 2 && x[i] <= l[i] {
                if x[i] < l[i] {
                    x[i] = l[i];
                }
            } else if nbd[i] >= 2 && x[i] >= u[i] && x[i] > u[i] {
                x[i] = u[i];
            }
        }
    }
    for i in 1..=n {
        if nbd[i] != 2 {
            boxed = false;
        }
        if nbd[i] == 0 {
            iwhere[i] = -1;
        } else {
            cnstnd = true;
            if nbd[i] == 2 && u[i] - l[i] <= ZERO {
                iwhere[i] = 3;
            } else {
                iwhere[i] = 0;
            }
        }
    }

    let (mut f, mut gv) = f_and_grad(&x[1..]);
    let mut g = vec![0.0; n + 1];
    g[1..].copy_from_slice(&gv);

    let mut sbgnrm = projgr(n, &l, &u, &nbd, &x, &g);
    if sbgnrm <= pgtol {
        return LbfgsbResult {
            x: x[1..].to_vec(),
            f,
            iterations: iter,
            termination: LbfgsbTermination::ProjectedGradient,
        };
    }

    'main: loop {
        // 222
        let mut nseg = 0usize;
        let wrk;
        if !cnstnd && col > 0 {
            z[1..].copy_from_slice(&x[1..]);
            wrk = updatd;
        } else {
            // cauchy
            let info = cauchy(
                n,
                &x,
                &l,
                &u,
                &nbd,
                &g,
                &mut indx2,
                &mut iwhere,
                &mut t,
                &mut d,
                &mut z,
                m,
                &wy,
                &ws,
                &sy,
                &wt,
                theta,
                col,
                head,
                &mut wa,
                &mut nseg,
                sbgnrm,
                epsmch,
            );
            if info != 0 {
                col = 0;
                head = 1;
                theta = ONE;
                iupdat = 0;
                updatd = false;
                continue 'main;
            }
            // freev
            nenter = 0;
            ileave = n + 1;
            if iter > 0 && cnstnd {
                for i in 1..=nfree {
                    let k = index[i];
                    if iwhere[k] > 0 {
                        ileave -= 1;
                        indx2[ileave] = k;
                    }
                }
                for i in 1 + nfree..=n {
                    let k = index[i];
                    if iwhere[k] <= 0 {
                        nenter += 1;
                        indx2[nenter] = k;
                    }
                }
            }
            wrk = (ileave < n + 1) || (nenter > 0) || updatd;
            nfree = 0;
            let mut iact = n + 1;
            for i in 1..=n {
                if iwhere[i] <= 0 {
                    nfree += 1;
                    index[nfree] = i;
                } else {
                    iact -= 1;
                    index[iact] = i;
                }
            }
            nact = n - nfree;
            let _ = nact;
        }

        // 333
        if nfree != 0 && col != 0 {
            let mut info = 0;
            if wrk {
                info = formk(
                    n, nfree, &index, nenter, ileave, &indx2, iupdat, updatd, &mut wn, &mut snd, m,
                    &ws, &wy, &sy, theta, col, head,
                );
            }
            if info != 0 {
                col = 0;
                head = 1;
                theta = ONE;
                iupdat = 0;
                updatd = false;
                continue 'main;
            }
            let mut info = cmprlb(
                n, m, &x, &g, &ws, &wy, &sy, &wt, &z, &mut r, &mut wa, &index, theta, col, head,
                nfree, cnstnd,
            );
            if info == 0 {
                info = subsm(
                    n, m, nfree, &index, &l, &u, &nbd, &mut z, &mut r, &mut xp, &ws, &wy, theta,
                    r_subspace, &x, &g, col, head, &mut wa, &wn,
                );
            }
            if info != 0 {
                col = 0;
                head = 1;
                theta = ONE;
                iupdat = 0;
                updatd = false;
                continue 'main;
            }
        }

        // 555
        for i in 1..=n {
            d[i] = z[i] - x[i];
        }

        // Line search (lnsrlb + dcsrch), driven with the closure.
        let dtd = ddot(n, &d, &d);
        let dnorm = dtd.sqrt();
        let mut stpmx = 1.0e10;
        if cnstnd {
            if iter == 0 {
                stpmx = ONE;
            } else {
                for i in 1..=n {
                    let a1 = d[i];
                    if nbd[i] != 0 {
                        if a1 < ZERO && nbd[i] <= 2 {
                            let a2 = l[i] - x[i];
                            if a2 >= ZERO {
                                stpmx = ZERO;
                            } else if a1 * stpmx < a2 {
                                stpmx = a2 / a1;
                            }
                        } else if a1 > ZERO && nbd[i] >= 2 {
                            let a2 = u[i] - x[i];
                            if a2 <= ZERO {
                                stpmx = ZERO;
                            } else if a1 * stpmx > a2 {
                                stpmx = a2 / a1;
                            }
                        }
                    }
                }
            }
        }
        let mut stp = if iter == 0 && !boxed {
            (ONE / dnorm).min(stpmx)
        } else {
            ONE
        };
        t[1..].copy_from_slice(&x[1..]);
        r[1..].copy_from_slice(&g[1..]);
        let fold = f;
        let mut ifun = 0usize;
        let mut iback;
        let mut gd = ddot(n, &g, &d);
        let gdold = gd;
        let mut info = 0i32;
        let mut srch: Option<Dcsrch> = None;
        let mut ls_failed = false;
        loop {
            if ifun == 0 {
                if gd >= ZERO {
                    info = -4;
                    ls_failed = true;
                    break;
                }
                match Dcsrch::start(f, gd, stp, stpmx) {
                    Ok(s) => srch = Some(s),
                    Err(_) => {
                        info = -4;
                        ls_failed = true;
                        break;
                    }
                }
                // task FG on first entry with unchanged stp.
            } else {
                let task = srch.as_mut().unwrap().step(f, gd, &mut stp, ZERO, stpmx);
                match task {
                    SrchTask::Convergence | SrchTask::Warning => break,
                    SrchTask::Error => {
                        info = -4;
                        ls_failed = true;
                        break;
                    }
                    SrchTask::Fg => {}
                }
            }
            // FG_LNSRCH
            ifun += 1;
            iback = ifun - 1;
            let _ = iback;
            if stp == ONE {
                x[1..].copy_from_slice(&z[1..]);
            } else {
                for i in 1..=n {
                    x[i] = stp * d[i] + t[i];
                    if nbd[i] == 1 || nbd[i] == 2 {
                        x[i] = x[i].max(l[i]);
                    }
                    if nbd[i] == 2 || nbd[i] == 3 {
                        x[i] = x[i].min(u[i]);
                    }
                }
            }
            if iback >= maxls {
                ls_failed = true;
                break;
            }
            let eval = f_and_grad(&x[1..]);
            f = eval.0;
            gv = eval.1;
            g[1..].copy_from_slice(&gv);
            gd = ddot(n, &g, &d);
        }

        if info != 0 || ls_failed {
            // Restore and restart or abort.
            x[1..].copy_from_slice(&t[1..]);
            g[1..].copy_from_slice(&r[1..]);
            f = fold;
            if col == 0 {
                iter += 1;
                return LbfgsbResult {
                    x: x[1..].to_vec(),
                    f,
                    iterations: iter,
                    termination: LbfgsbTermination::LineSearchFailed,
                };
            }
            col = 0;
            head = 1;
            theta = ONE;
            iupdat = 0;
            updatd = false;
            continue 'main;
        }

        iter += 1;
        sbgnrm = projgr(n, &l, &u, &nbd, &x, &g);

        // 777 convergence tests
        if sbgnrm <= pgtol {
            return LbfgsbResult {
                x: x[1..].to_vec(),
                f,
                iterations: iter,
                termination: LbfgsbTermination::ProjectedGradient,
            };
        }
        let ddum = fold.abs().max(f.abs()).max(ONE);
        if (fold - f) <= tol * ddum {
            return LbfgsbResult {
                x: x[1..].to_vec(),
                f,
                iterations: iter,
                termination: LbfgsbTermination::FunctionTolerance,
            };
        }
        if iter >= max_iter {
            return LbfgsbResult {
                x: x[1..].to_vec(),
                f,
                iterations: iter,
                termination: LbfgsbTermination::IterationLimit,
            };
        }

        // Compute y = g - r and update the memory.
        for i in 1..=n {
            r[i] = g[i] - r[i];
        }
        let rr = ddot(n, &r, &r);
        let dr;
        let ddum2;
        if stp == ONE {
            dr = gd - gdold;
            ddum2 = -gdold;
        } else {
            dr = (gd - gdold) * stp;
            for i in 1..=n {
                d[i] *= stp;
            }
            ddum2 = -gdold * stp;
        }
        if dr <= epsmch * ddum2 {
            updatd = false;
            continue 'main;
        }
        updatd = true;
        iupdat += 1;
        matupd(
            n, m, &mut ws, &mut wy, &mut sy, &mut ss, &d, &r, &mut itail, iupdat, &mut col,
            &mut head, &mut theta, rr, dr, stp, dtd,
        );
        let info = formt(m, &mut wt, &sy, &ss, col, theta);
        if info != 0 {
            col = 0;
            head = 1;
            theta = ONE;
            iupdat = 0;
            updatd = false;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn cauchy(
    n: usize,
    x: &[f64],
    l: &[f64],
    u: &[f64],
    nbd: &[i32],
    g: &[f64],
    iorder: &mut [usize],
    iwhere: &mut [i32],
    t: &mut [f64],
    d: &mut [f64],
    xcp: &mut [f64],
    m: usize,
    wy: &Mat,
    ws: &Mat,
    sy: &Mat,
    wt: &Mat,
    theta: f64,
    col: usize,
    head: usize,
    wa: &mut [f64],
    nseg: &mut usize,
    sbgnrm: f64,
    epsmch: f64,
) -> i32 {
    // wa is partitioned as p (2m), c (2m), wbp (2m), v (2m), 1-based blocks.
    if sbgnrm <= ZERO {
        xcp[1..=n].copy_from_slice(&x[1..=n]);
        return 0;
    }
    let mut bnded = true;
    let mut nfree = n + 1;
    let mut nbreak = 0usize;
    let mut ibkmin = 0usize;
    let mut tl = 0.0;
    let mut tu = 0.0;
    let mut bkmin = ZERO;
    let col2 = 2 * col;
    let mut f1 = ZERO;
    for i in 1..=col2 {
        wa[i] = ZERO;
    }
    for i in 1..=n {
        let neggi = -g[i];
        if iwhere[i] != 3 && iwhere[i] != -1 {
            if nbd[i] <= 2 {
                tl = x[i] - l[i];
            }
            if nbd[i] >= 2 {
                tu = u[i] - x[i];
            }
            let xlower = nbd[i] <= 2 && tl <= ZERO;
            let xupper = nbd[i] >= 2 && tu <= ZERO;
            iwhere[i] = 0;
            if xlower {
                if neggi <= ZERO {
                    iwhere[i] = 1;
                }
            } else if xupper {
                if neggi >= ZERO {
                    iwhere[i] = 2;
                }
            } else if neggi.abs() <= ZERO {
                iwhere[i] = -3;
            }
        }
        let mut pointr = head;
        if iwhere[i] != 0 && iwhere[i] != -1 {
            d[i] = ZERO;
        } else {
            d[i] = neggi;
            f1 -= neggi * neggi;
            for j in 1..=col {
                wa[j] += wy.g(i, pointr) * neggi;
                wa[col + j] += ws.g(i, pointr) * neggi;
                pointr = pointr % m + 1;
            }
            if nbd[i] <= 2 && nbd[i] != 0 && neggi < ZERO {
                nbreak += 1;
                iorder[nbreak] = i;
                t[nbreak] = tl / (-neggi);
                if nbreak == 1 || t[nbreak] < bkmin {
                    bkmin = t[nbreak];
                    ibkmin = nbreak;
                }
            } else if nbd[i] >= 2 && neggi > ZERO {
                nbreak += 1;
                iorder[nbreak] = i;
                t[nbreak] = tu / neggi;
                if nbreak == 1 || t[nbreak] < bkmin {
                    bkmin = t[nbreak];
                    ibkmin = nbreak;
                }
            } else {
                nfree -= 1;
                iorder[nfree] = i;
                if neggi.abs() > ZERO {
                    bnded = false;
                }
            }
        }
    }
    if theta != ONE {
        for j in col + 1..=col2 {
            wa[j] *= theta;
        }
    }
    xcp[1..=n].copy_from_slice(&x[1..=n]);
    if nbreak == 0 && nfree == n + 1 {
        return 0;
    }
    for j in 1..=col2 {
        wa[2 * m + j] = ZERO;
    }
    let mut f2 = -theta * f1;
    let f2_org = f2;
    if col > 0 {
        let mut vsrc = vec![0.0; 2 * col + 1];
        vsrc[1..=col2].copy_from_slice(&wa[1..=col2]);
        let mut vout = vec![0.0; 2 * col + 1];
        let info = bmv(m, sy, wt, col, &vsrc, &mut vout);
        if info != 0 {
            return info;
        }
        wa[6 * m + 1..6 * m + 1 + col2].copy_from_slice(&vout[1..=col2]);
        let mut dot = 0.0;
        for j in 1..=col2 {
            dot += vout[j] * wa[j];
        }
        f2 -= dot;
    }
    let mut dtm = -f1 / f2;
    let mut tsum = ZERO;
    *nseg = 1;
    if nbreak != 0 {
        let mut nleft = nbreak;
        let mut iter_c = 1usize;
        let mut tj = ZERO;
        loop {
            let tj0 = tj;
            let ibp;
            if iter_c == 1 {
                tj = bkmin;
                ibp = iorder[ibkmin];
            } else {
                if iter_c == 2 {
                    if ibkmin != nbreak {
                        t[ibkmin] = t[nbreak];
                        iorder[ibkmin] = iorder[nbreak];
                    }
                }
                hpsolb(nleft, t, iorder, (iter_c - 2) as i32);
                tj = t[nleft];
                ibp = iorder[nleft];
            }
            let dt = tj - tj0;
            if dtm < dt {
                break;
            }
            tsum += dt;
            nleft -= 1;
            iter_c += 1;
            let dibp = d[ibp];
            d[ibp] = ZERO;
            let zibp;
            if dibp > ZERO {
                zibp = u[ibp] - x[ibp];
                xcp[ibp] = u[ibp];
                iwhere[ibp] = 2;
            } else {
                zibp = l[ibp] - x[ibp];
                xcp[ibp] = l[ibp];
                iwhere[ibp] = 1;
            }
            if nleft == 0 && nbreak == n {
                dtm = dt;
                // 999
                if col > 0 {
                    for j in 1..=col2 {
                        wa[2 * m + j] += dtm * wa[j];
                    }
                }
                return 0;
            }
            *nseg += 1;
            let dibp2 = dibp * dibp;
            f1 += dt * f2 + dibp2 - theta * dibp * zibp;
            f2 -= theta * dibp2;
            if col > 0 {
                for j in 1..=col2 {
                    wa[2 * m + j] += dt * wa[j];
                }
                let mut pointr = head;
                for j in 1..=col {
                    wa[4 * m + j] = wy.g(ibp, pointr);
                    wa[4 * m + col + j] = theta * ws.g(ibp, pointr);
                    pointr = pointr % m + 1;
                }
                let mut wbp = vec![0.0; 2 * col + 1];
                wbp[1..=col2].copy_from_slice(&wa[4 * m + 1..4 * m + 1 + col2]);
                let mut v = vec![0.0; 2 * col + 1];
                let info = bmv(m, sy, wt, col, &wbp, &mut v);
                if info != 0 {
                    return info;
                }
                wa[6 * m + 1..6 * m + 1 + col2].copy_from_slice(&v[1..=col2]);
                let mut wmc = 0.0;
                let mut wmp = 0.0;
                let mut wmw = 0.0;
                for j in 1..=col2 {
                    wmc += wa[2 * m + j] * v[j];
                    wmp += wa[j] * v[j];
                    wmw += wbp[j] * v[j];
                }
                for j in 1..=col2 {
                    wa[j] -= dibp * wbp[j];
                }
                f1 += dibp * wmc;
                f2 += 2.0 * dibp * wmp - dibp2 * wmw;
            }
            f2 = f2.max(epsmch * f2_org);
            if nleft > 0 {
                dtm = -f1 / f2;
                continue;
            } else if bnded {
                dtm = ZERO;
                break;
            } else {
                dtm = -f1 / f2;
                break;
            }
        }
    }
    // 888
    if dtm <= ZERO {
        dtm = ZERO;
    }
    tsum += dtm;
    for i in 1..=n {
        xcp[i] += tsum * d[i];
    }
    if col > 0 {
        for j in 1..=col2 {
            wa[2 * m + j] += dtm * wa[j];
        }
    }
    0
}

#[allow(clippy::too_many_arguments)]
fn formk(
    n: usize,
    nsub: usize,
    ind: &[usize],
    nenter: usize,
    ileave: usize,
    indx2: &[usize],
    iupdat: usize,
    updatd: bool,
    wn: &mut Mat,
    wn1: &mut Mat,
    m: usize,
    ws: &Mat,
    wy: &Mat,
    sy: &Mat,
    theta: f64,
    col: usize,
    head: usize,
) -> i32 {
    let upcl;
    if updatd {
        if iupdat > m {
            for jy in 1..=m - 1 {
                let js = m + jy;
                for k in 0..m - jy {
                    let v = wn1.g(jy + 1 + k, jy + 1);
                    wn1.s(jy + k, jy, v);
                    let v = wn1.g(js + 1 + k, js + 1);
                    wn1.s(js + k, js, v);
                }
                for k in 0..m - 1 {
                    let v = wn1.g(m + 2 + k, jy + 1);
                    wn1.s(m + 1 + k, jy, v);
                }
            }
        }
        let pbegin = 1;
        let pend = nsub;
        let dbegin = nsub + 1;
        let dend = n;
        let iy = col;
        let is = m + col;
        let mut ipntr = head + col - 1;
        if ipntr > m {
            ipntr -= m;
        }
        let mut jpntr = head;
        for jy in 1..=col {
            let js = m + jy;
            let mut temp1 = ZERO;
            let mut temp2 = ZERO;
            let mut temp3 = ZERO;
            for k in pbegin..=pend {
                let k1 = ind[k];
                temp1 += wy.g(k1, ipntr) * wy.g(k1, jpntr);
            }
            for k in dbegin..=dend {
                let k1 = ind[k];
                temp2 += ws.g(k1, ipntr) * ws.g(k1, jpntr);
                temp3 += ws.g(k1, ipntr) * wy.g(k1, jpntr);
            }
            wn1.s(iy, jy, temp1);
            wn1.s(is, js, temp2);
            wn1.s(is, jy, temp3);
            jpntr = jpntr % m + 1;
        }
        let jy = col;
        let mut jpntr = head + col - 1;
        if jpntr > m {
            jpntr -= m;
        }
        let mut ipntr = head;
        for i in 1..=col {
            let is = m + i;
            let mut temp3 = ZERO;
            for k in pbegin..=pend {
                let k1 = ind[k];
                temp3 += ws.g(k1, ipntr) * wy.g(k1, jpntr);
            }
            ipntr = ipntr % m + 1;
            wn1.s(is, jy, temp3);
        }
        upcl = col - 1;
    } else {
        upcl = col;
    }

    let mut ipntr = head;
    for iy in 1..=upcl {
        let is = m + iy;
        let mut jpntr = head;
        for jy in 1..=iy {
            let js = m + jy;
            let mut temp1 = ZERO;
            let mut temp2 = ZERO;
            let mut temp3 = ZERO;
            let mut temp4 = ZERO;
            for k in 1..=nenter {
                let k1 = indx2[k];
                temp1 += wy.g(k1, ipntr) * wy.g(k1, jpntr);
                temp2 += ws.g(k1, ipntr) * ws.g(k1, jpntr);
            }
            for k in ileave..=n {
                let k1 = indx2[k];
                temp3 += wy.g(k1, ipntr) * wy.g(k1, jpntr);
                temp4 += ws.g(k1, ipntr) * ws.g(k1, jpntr);
            }
            let v = wn1.g(iy, jy) + temp1 - temp3;
            wn1.s(iy, jy, v);
            let v = wn1.g(is, js) - temp2 + temp4;
            wn1.s(is, js, v);
            jpntr = jpntr % m + 1;
        }
        ipntr = ipntr % m + 1;
    }
    let mut ipntr = head;
    for is in m + 1..=m + upcl {
        let mut jpntr = head;
        for jy in 1..=upcl {
            let mut temp1 = ZERO;
            let mut temp3 = ZERO;
            for k in 1..=nenter {
                let k1 = indx2[k];
                temp1 += ws.g(k1, ipntr) * wy.g(k1, jpntr);
            }
            for k in ileave..=n {
                let k1 = indx2[k];
                temp3 += ws.g(k1, ipntr) * wy.g(k1, jpntr);
            }
            if is <= jy + m {
                let v = wn1.g(is, jy) + temp1 - temp3;
                wn1.s(is, jy, v);
            } else {
                let v = wn1.g(is, jy) - temp1 + temp3;
                wn1.s(is, jy, v);
            }
            jpntr = jpntr % m + 1;
        }
        ipntr = ipntr % m + 1;
    }

    let m2 = 2 * m;
    for iy in 1..=col {
        let is = col + iy;
        let is1 = m + iy;
        for jy in 1..=iy {
            let js = col + jy;
            let js1 = m + jy;
            wn.s(jy, iy, wn1.g(iy, jy) / theta);
            wn.s(js, is, wn1.g(is1, js1) * theta);
        }
        for jy in 1..iy {
            wn.s(jy, is, -wn1.g(is1, jy));
        }
        for jy in iy..=col {
            wn.s(jy, is, wn1.g(is1, jy));
        }
        let v = wn.g(iy, iy) + sy.g(iy, iy);
        wn.s(iy, iy, v);
    }
    let info = dpofa(wn, col);
    if info != 0 {
        return -1;
    }
    let col2 = 2 * col;
    for js in col + 1..=col2 {
        // solve trans(upper) * x = wn[1..col, js]
        let mut b = vec![0.0; col + 1];
        for k in 1..=col {
            b[k] = wn.g(k, js);
        }
        let info = dtrsl(wn, col, &mut b, 0, 11);
        if info != 0 {
            return -1;
        }
        for k in 1..=col {
            wn.s(k, js, b[k]);
        }
    }
    for is in col + 1..=col2 {
        for js in is..=col2 {
            let mut dot = 0.0;
            for k in 1..=col {
                dot += wn.g(k, is) * wn.g(k, js);
            }
            let v = wn.g(is, js) + dot;
            wn.s(is, js, v);
        }
    }
    // dpofa on the trailing block wn[col+1..2col, col+1..2col]
    {
        let mut sub = Mat::new(col, col);
        for i in 1..=col {
            for j in 1..=col {
                sub.s(i, j, wn.g(col + i, col + j));
            }
        }
        let info = dpofa(&mut sub, col);
        if info != 0 {
            return -2;
        }
        for i in 1..=col {
            for j in 1..=col {
                wn.s(col + i, col + j, sub.g(i, j));
            }
        }
    }
    let _ = m2;
    0
}

#[allow(clippy::too_many_arguments)]
fn cmprlb(
    n: usize,
    m: usize,
    x: &[f64],
    g: &[f64],
    ws: &Mat,
    wy: &Mat,
    sy: &Mat,
    wt: &Mat,
    z: &[f64],
    r: &mut [f64],
    wa: &mut [f64],
    index: &[usize],
    theta: f64,
    col: usize,
    head: usize,
    nfree: usize,
    cnstnd: bool,
) -> i32 {
    if !cnstnd && col > 0 {
        for i in 1..=n {
            r[i] = -g[i];
        }
    } else {
        for i in 1..=nfree {
            let k = index[i];
            r[i] = -theta * (z[k] - x[k]) - g[k];
        }
        let mut vsrc = vec![0.0; 2 * col + 1];
        vsrc[1..=2 * col].copy_from_slice(&wa[2 * m + 1..2 * m + 1 + 2 * col]);
        let mut vout = vec![0.0; 2 * col + 1];
        let info = bmv(m, sy, wt, col, &vsrc, &mut vout);
        if info != 0 {
            return -8;
        }
        wa[1..=2 * col].copy_from_slice(&vout[1..=2 * col]);
        let mut pointr = head;
        for j in 1..=col {
            let a1 = wa[j];
            let a2 = theta * wa[col + j];
            for i in 1..=nfree {
                let k = index[i];
                r[i] += wy.g(k, pointr) * a1 + ws.g(k, pointr) * a2;
            }
            pointr = pointr % m + 1;
        }
    }
    0
}

#[allow(clippy::too_many_arguments)]
fn subsm(
    n: usize,
    m: usize,
    nsub: usize,
    ind: &[usize],
    l: &[f64],
    u: &[f64],
    nbd: &[i32],
    x: &mut [f64],
    d: &mut [f64],
    xp: &mut [f64],
    ws: &Mat,
    wy: &Mat,
    theta: f64,
    r_subspace: bool,
    xx: &[f64],
    gg: &[f64],
    col: usize,
    head: usize,
    wv: &mut [f64],
    wn: &Mat,
) -> i32 {
    if nsub == 0 {
        return 0;
    }
    let mut pointr = head;
    for i in 1..=col {
        let mut temp1 = ZERO;
        let mut temp2 = ZERO;
        for j in 1..=nsub {
            let k = ind[j];
            temp1 += wy.g(k, pointr) * d[j];
            temp2 += ws.g(k, pointr) * d[j];
        }
        wv[i] = temp1;
        wv[col + i] = theta * temp2;
        pointr = pointr % m + 1;
    }
    let col2 = 2 * col;
    let info = dtrsl(wn, col2, wv, 0, 11);
    if info != 0 {
        return info;
    }
    for i in 1..=col {
        wv[i] = -wv[i];
    }
    let info = dtrsl(wn, col2, wv, 0, 1);
    if info != 0 {
        return info;
    }
    let mut pointr = head;
    for jy in 1..=col {
        let js = col + jy;
        for i in 1..=nsub {
            let k = ind[i];
            d[i] += wy.g(k, pointr) * wv[jy] / theta + ws.g(k, pointr) * wv[js];
        }
        pointr = pointr % m + 1;
    }
    for i in 1..=nsub {
        if r_subspace {
            d[i] /= theta;
        } else {
            d[i] *= ONE / theta;
        }
    }

    if !r_subspace {
        let mut iword = 0;
        xp[1..=n].copy_from_slice(&x[1..=n]);
        for i in 1..=nsub {
            let k = ind[i];
            let dk = d[i];
            let xk = x[k];
            if nbd[k] != 0 {
                if nbd[k] == 1 {
                    x[k] = l[k].max(xk + dk);
                    if x[k] == l[k] {
                        iword = 1;
                    }
                } else if nbd[k] == 2 {
                    let xk2 = l[k].max(xk + dk);
                    x[k] = u[k].min(xk2);
                    if x[k] == l[k] || x[k] == u[k] {
                        iword = 1;
                    }
                } else if nbd[k] == 3 {
                    x[k] = u[k].min(xk + dk);
                    if x[k] == u[k] {
                        iword = 1;
                    }
                }
            } else {
                x[k] = xk + dk;
            }
        }
        if iword == 0 {
            return 0;
        }
        let mut dd_p = ZERO;
        for i in 1..=n {
            dd_p += (x[i] - xx[i]) * gg[i];
        }
        if dd_p > ZERO {
            x[1..=n].copy_from_slice(&xp[1..=n]);
        } else {
            return 0;
        }
    }
    let mut alpha = ONE;
    let mut temp1 = alpha;
    let mut ibd = 0usize;
    for i in 1..=nsub {
        let k = ind[i];
        let dk = d[i];
        if nbd[k] != 0 {
            if dk < ZERO && nbd[k] <= 2 {
                let temp2 = l[k] - x[k];
                if temp2 >= ZERO {
                    temp1 = ZERO;
                } else if dk * alpha < temp2 {
                    temp1 = temp2 / dk;
                }
            } else if dk > ZERO && nbd[k] >= 2 {
                let temp2 = u[k] - x[k];
                if temp2 <= ZERO {
                    temp1 = ZERO;
                } else if dk * alpha > temp2 {
                    temp1 = temp2 / dk;
                }
            }
            if temp1 < alpha {
                alpha = temp1;
                ibd = i;
            }
        }
    }
    if alpha < ONE {
        let dk = d[ibd];
        let k = ind[ibd];
        if dk > ZERO {
            x[k] = u[k];
            d[ibd] = ZERO;
        } else if dk < ZERO {
            x[k] = l[k];
            d[ibd] = ZERO;
        }
    }
    for i in 1..=nsub {
        let k = ind[i];
        x[k] += alpha * d[i];
    }
    0
}

#[allow(clippy::too_many_arguments)]
fn matupd(
    n: usize,
    m: usize,
    ws: &mut Mat,
    wy: &mut Mat,
    sy: &mut Mat,
    ss: &mut Mat,
    d: &[f64],
    r: &[f64],
    itail: &mut usize,
    iupdat: usize,
    col: &mut usize,
    head: &mut usize,
    theta: &mut f64,
    rr: f64,
    dr: f64,
    stp: f64,
    dtd: f64,
) {
    if iupdat <= m {
        *col = iupdat;
        *itail = (*head + iupdat - 2) % m + 1;
    } else {
        *itail = *itail % m + 1;
        *head = *head % m + 1;
    }
    for i in 1..=n {
        ws.s(i, *itail, d[i]);
        wy.s(i, *itail, r[i]);
    }
    *theta = rr / dr;
    if iupdat > m {
        for j in 1..=*col - 1 {
            for k in 0..j {
                let v = ss.g(2 + k, j + 1);
                ss.s(1 + k, j, v);
            }
            for k in 0..*col - j {
                let v = sy.g(j + 1 + k, j + 1);
                sy.s(j + k, j, v);
            }
        }
    }
    let mut pointr = *head;
    for j in 1..=*col - 1 {
        let mut dot1 = 0.0;
        let mut dot2 = 0.0;
        for i in 1..=n {
            dot1 += d[i] * wy.g(i, pointr);
            dot2 += ws.g(i, pointr) * d[i];
        }
        sy.s(*col, j, dot1);
        ss.s(j, *col, dot2);
        pointr = pointr % m + 1;
    }
    if stp == ONE {
        ss.s(*col, *col, dtd);
    } else {
        ss.s(*col, *col, stp * stp * dtd);
    }
    sy.s(*col, *col, dr);
}

fn formt(m: usize, wt: &mut Mat, sy: &Mat, ss: &Mat, col: usize, theta: f64) -> i32 {
    let _ = m;
    for j in 1..=col {
        wt.s(1, j, theta * ss.g(1, j));
    }
    for i in 2..=col {
        for j in i..=col {
            let k1 = i.min(j) - 1;
            let mut ddum = ZERO;
            for k in 1..=k1 {
                ddum += sy.g(i, k) * sy.g(j, k) / sy.g(k, k);
            }
            wt.s(i, j, ddum + theta * ss.g(i, j));
        }
    }
    let info = dpofa(wt, col);
    if info != 0 {
        return -3;
    }
    0
}
