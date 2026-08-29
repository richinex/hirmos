//! `statsmodels.tsa.seasonal.STL`, the seasonal trend decomposition of Cleveland, Cleveland,
//! McRae and Terpenning (1990), ported from `statsmodels/tsa/stl/_stl.pyx`.

pub struct StlConfig {
    pub period: usize,
    pub seasonal: usize,
    pub trend: usize,
    pub low_pass: usize,
    pub seasonal_deg: i32,
    pub trend_deg: i32,
    pub low_pass_deg: i32,
    pub robust: bool,
    pub seasonal_jump: usize,
    pub trend_jump: usize,
    pub low_pass_jump: usize,
}

impl StlConfig {
    /// `STL(endog, period=period, robust=robust)`, filling in the defaults statsmodels
    /// derives from the period.
    pub fn new(period: usize, robust: bool) -> Self {
        assert!(period >= 2, "period must be a positive integer >= 2");
        let seasonal = 7usize;
        let mut trend = (1.5 * period as f64 / (1.0 - 1.5 / seasonal as f64)).ceil() as usize;
        trend += usize::from(trend % 2 == 0);
        let mut low_pass = period + 1;
        low_pass += usize::from(low_pass % 2 == 0);
        Self {
            period,
            seasonal,
            trend,
            low_pass,
            seasonal_deg: 1,
            trend_deg: 1,
            low_pass_deg: 1,
            robust,
            seasonal_jump: 1,
            trend_jump: 1,
            low_pass_jump: 1,
        }
    }
}

pub struct StlResult {
    pub trend: Vec<f64>,
    pub seasonal: Vec<f64>,
    pub resid: Vec<f64>,
    /// `robust_weight`, the bisquare weights of the outer loop.
    pub weights: Vec<f64>,
}

/// One local regression estimate at `xs`, over the window `[nleft, nright]` in one-based
/// positions. Returns NaN where the weights sum to zero, which the callers replace.
#[allow(clippy::too_many_arguments)]
fn est(
    y: &[f64],
    n: usize,
    len: usize,
    ideg: i32,
    xs: f64,
    nleft: usize,
    nright: usize,
    w: &mut [f64],
    userw: bool,
    rw: &[f64],
) -> f64 {
    let rng = n as f64 - 1.0;
    let mut h = (xs - nleft as f64).max(nright as f64 - xs);
    if len > n {
        h += ((len - n) as f64 / 2.0).floor();
    }
    let h9 = 0.999 * h;
    let h1 = 0.001 * h;
    let mut a = 0.0;
    for j in nleft - 1..nright {
        w[j] = 0.0;
        let r = ((j + 1) as f64 - xs).abs();
        if r <= h9 {
            w[j] = if r <= h1 {
                1.0
            } else {
                (1.0 - (r / h).powi(3)).powi(3)
            };
            if userw {
                w[j] *= rw[j];
            }
            a += w[j];
        }
    }
    if a <= 0.0 {
        return f64::NAN;
    }
    for j in nleft - 1..nright {
        w[j] /= a;
    }
    if h > 0.0 && ideg > 0 {
        // Fit a local line rather than a local mean.
        let mut a = 0.0;
        for j in nleft - 1..nright {
            a += w[j] * (j + 1) as f64;
        }
        let mut b = xs - a;
        let mut c = 0.0;
        for j in nleft - 1..nright {
            c += w[j] * ((j + 1) as f64 - a).powi(2);
        }
        if c.sqrt() > 0.001 * rng {
            b /= c;
            for j in nleft - 1..nright {
                w[j] *= b * ((j + 1) as f64 - a) + 1.0;
            }
        }
    }
    (nleft - 1..nright).map(|j| w[j] * y[j]).sum()
}

/// Loess smoothing of a whole series, evaluating every `njump`th point and interpolating
/// between them.
#[allow(clippy::too_many_arguments)]
fn ess(
    y: &[f64],
    n: usize,
    len: usize,
    ideg: i32,
    njump: usize,
    userw: bool,
    rw: &[f64],
    ys: &mut [f64],
    res: &mut [f64],
) {
    if n < 2 {
        ys[0] = y[0];
        return;
    }
    let newnj = njump.min(n - 1);
    let mut nleft = 1usize;
    let mut nright = n;
    if len >= n {
        let mut i = 0;
        while i < n {
            ys[i] = est(
                y,
                n,
                len,
                ideg,
                (i + 1) as f64,
                nleft,
                nright,
                res,
                userw,
                rw,
            );
            if ys[i].is_nan() {
                ys[i] = y[i];
            }
            i += newnj;
        }
    } else if newnj == 1 {
        let nsh = (len + 2) / 2;
        nleft = 1;
        nright = len;
        for i in 0..n {
            if i + 1 > nsh && nright != n {
                nleft += 1;
                nright += 1;
            }
            ys[i] = est(
                y,
                n,
                len,
                ideg,
                (i + 1) as f64,
                nleft,
                nright,
                res,
                userw,
                rw,
            );
            if ys[i].is_nan() {
                ys[i] = y[i];
            }
        }
    } else {
        let nsh = (len + 1) / 2;
        let mut i = 0;
        while i < n {
            if i + 1 < nsh {
                nleft = 1;
                nright = len;
            } else if i + 1 >= n - nsh + 1 {
                nleft = n - len + 1;
                nright = n;
            } else {
                nleft = i + 1 - nsh + 1;
                nright = len + i + 1 - nsh;
            }
            ys[i] = est(
                y,
                n,
                len,
                ideg,
                (i + 1) as f64,
                nleft,
                nright,
                res,
                userw,
                rw,
            );
            if ys[i].is_nan() {
                ys[i] = y[i];
            }
            i += newnj;
        }
    }
    if newnj == 1 {
        return;
    }
    let mut i = 0;
    while i + newnj < n {
        let delta = (ys[i + newnj] - ys[i]) / newnj as f64;
        for j in i..i + newnj {
            ys[j] = ys[i] + delta * ((j + 1) as f64 - (i + 1) as f64);
        }
        i += newnj;
    }
    let k = ((n - 1) / newnj) * newnj + 1;
    if k != n {
        ys[n - 1] = est(y, n, len, ideg, n as f64, nleft, nright, res, userw, rw);
        if ys[n - 1].is_nan() {
            ys[n - 1] = y[n - 1];
        }
        if k != n - 1 {
            let delta = (ys[n - 1] - ys[k - 1]) / (n - k) as f64;
            for j in k..n {
                ys[j] = ys[k - 1] + delta * ((j + 1) as f64 - k as f64);
            }
        }
    }
}

/// Moving average of length `len` over the first `n` entries.
fn ma(x: &[f64], n: usize, len: usize, ave: &mut [f64]) {
    let newn = n - len + 1;
    let flen = len as f64;
    let mut v: f64 = x[..len].iter().sum();
    ave[0] = v / flen;
    let (mut k, mut m) = (len, 0usize);
    for a in ave.iter_mut().take(newn).skip(1) {
        v += x[k] - x[m];
        *a = v / flen;
        k += 1;
        m += 1;
    }
}

struct Stl<'a> {
    y: &'a [f64],
    nobs: usize,
    cfg: &'a StlConfig,
    trend: Vec<f64>,
    season: Vec<f64>,
    rw: Vec<f64>,
    work: Vec<Vec<f64>>,
    use_rw: bool,
}

impl Stl<'_> {
    /// The low pass filter: three moving averages then a loess pass.
    fn fts(&mut self) {
        let np = self.cfg.period;
        let n = self.nobs + 2 * np;
        let mut w2 = std::mem::take(&mut self.work[2]);
        let mut w0 = std::mem::take(&mut self.work[0]);
        ma(&self.work[1], n, np, &mut w2);
        ma(&w2, n - np + 1, np, &mut w0);
        ma(&w0, n - 2 * np + 2, 3, &mut w2);
        self.work[2] = w2;
        self.work[0] = w0;
    }

    /// Cycle-subseries smoothing: each phase of the period is smoothed as its own series,
    /// extended by one position at each end.
    fn ss(&mut self) {
        let (n, np, ns) = (self.nobs, self.cfg.period, self.cfg.seasonal);
        let mut season = std::mem::take(&mut self.work[1]);
        let mut work1 = std::mem::take(&mut self.work[2]);
        let mut work2 = std::mem::take(&mut self.work[3]);
        let mut work3 = std::mem::take(&mut self.work[4]);
        // The seasonal output array doubles as scratch here, as in the original.
        let mut work4 = std::mem::take(&mut self.season);

        for j in 0..np {
            let k = (n - (j + 1)) / np + 1;
            for i in 0..k {
                work1[i] = self.work[0][i * np + j];
            }
            if self.use_rw {
                for i in 0..k {
                    work3[i] = self.rw[i * np + j];
                }
            }
            ess(
                &work1,
                k,
                ns,
                self.cfg.seasonal_deg,
                self.cfg.seasonal_jump,
                self.use_rw,
                &work3,
                &mut work2[1..],
                &mut work4,
            );
            let nright = ns.min(k);
            work2[0] = est(
                &work1,
                k,
                ns,
                self.cfg.seasonal_deg,
                0.0,
                1,
                nright,
                &mut work4,
                self.use_rw,
                &work3,
            );
            if work2[0].is_nan() {
                work2[0] = work2[1];
            }
            let nleft = 1.max(k.saturating_sub(ns) + 1);
            work2[k + 1] = est(
                &work1,
                k,
                ns,
                self.cfg.seasonal_deg,
                (k + 1) as f64,
                nleft,
                k,
                &mut work4,
                self.use_rw,
                &work3,
            );
            if work2[k + 1].is_nan() {
                work2[k + 1] = work2[k];
            }
            for m in 0..k + 2 {
                season[m * np + j] = work2[m];
            }
        }
        self.work[1] = season;
        self.work[2] = work1;
        self.work[3] = work2;
        self.work[4] = work3;
        self.season = work4;
    }

    fn onestp(&mut self, inner_iter: usize) {
        let (n, np) = (self.nobs, self.cfg.period);
        for _ in 0..inner_iter {
            for i in 0..n {
                self.work[0][i] = self.y[i] - self.trend[i];
            }
            self.ss();
            self.fts();
            let mut w0 = std::mem::take(&mut self.work[0]);
            let mut w4 = std::mem::take(&mut self.work[4]);
            ess(
                &self.work[2],
                n,
                self.cfg.low_pass,
                self.cfg.low_pass_deg,
                self.cfg.low_pass_jump,
                false,
                &self.work[3],
                &mut w0,
                &mut w4,
            );
            self.work[0] = w0;
            self.work[4] = w4;

            for i in 0..n {
                self.season[i] = self.work[1][np + i] - self.work[0][i];
                self.work[0][i] = self.y[i] - self.season[i];
            }
            let mut trend = std::mem::take(&mut self.trend);
            let mut w2 = std::mem::take(&mut self.work[2]);
            ess(
                &self.work[0],
                n,
                self.cfg.trend,
                self.cfg.trend_deg,
                self.cfg.trend_jump,
                self.use_rw,
                &self.rw,
                &mut trend,
                &mut w2,
            );
            self.trend = trend;
            self.work[2] = w2;
        }
    }

    /// Bisquare robustness weights against six times the median absolute residual.
    fn rwts(&mut self) {
        let n = self.nobs;
        for i in 0..n {
            self.rw[i] = (self.y[i] - self.work[0][i]).abs();
        }
        let mut sorted: Vec<f64> = self.rw[..n].to_vec();
        let (lo, hi) = (n / 2, n - n / 2 - 1);
        sorted.sort_by(|a, b| a.partial_cmp(b).expect("no NaN residuals"));
        let cmad = 3.0 * (sorted[lo] + sorted[hi]);
        if cmad == 0.0 {
            self.rw[..n].fill(1.0);
            return;
        }
        let c9 = 0.999 * cmad;
        let c1 = 0.001 * cmad;
        for i in 0..n {
            self.rw[i] = if self.rw[i] <= c1 {
                1.0
            } else if self.rw[i] <= c9 {
                (1.0 - (self.rw[i] / cmad).powi(2)).powi(2)
            } else {
                0.0
            };
        }
    }
}

/// `STL(...).fit(inner_iter, outer_iter)`. Passing `None` uses statsmodels' defaults, which
/// are 2 inner and 15 outer iterations when robust and 5 and 0 when not.
pub fn stl(
    y: &[f64],
    cfg: &StlConfig,
    inner_iter: Option<usize>,
    outer_iter: Option<usize>,
) -> StlResult {
    let n = y.len();
    assert!(
        cfg.trend > cfg.period,
        "trend must be greater than the period"
    );
    assert!(
        cfg.low_pass > cfg.period,
        "low_pass must be greater than the period"
    );
    let inner = inner_iter.unwrap_or(if cfg.robust { 2 } else { 5 });
    let outer = outer_iter.unwrap_or(if cfg.robust { 15 } else { 0 });

    let mut model = Stl {
        y,
        nobs: n,
        cfg,
        trend: vec![0.0; n],
        season: vec![0.0; n],
        rw: vec![1.0; n],
        work: vec![vec![0.0; n + 2 * cfg.period]; 7],
        use_rw: false,
    };

    let mut k = 0;
    loop {
        model.onestp(inner);
        k += 1;
        if k > outer {
            break;
        }
        for i in 0..n {
            model.work[0][i] = model.trend[i] + model.season[i];
        }
        model.rwts();
        model.use_rw = true;
    }

    let resid = (0..n)
        .map(|i| y[i] - model.season[i] - model.trend[i])
        .collect();
    StlResult {
        trend: model.trend,
        seasonal: model.season,
        resid,
        weights: model.rw,
    }
}

/// The strength measure 805 reads off the decomposition:
/// `max(0, 1 - var(resid) / var(component + resid))`, using the sample variance pandas uses.
pub fn strength(component: &[f64], resid: &[f64]) -> f64 {
    let var = |v: &[f64]| {
        let n = v.len() as f64;
        let mean = v.iter().sum::<f64>() / n;
        v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
    };
    let combined: Vec<f64> = component.iter().zip(resid).map(|(a, b)| a + b).collect();
    let v = var(&combined);
    if v > 1e-12 {
        (1.0 - var(resid) / v).max(0.0)
    } else {
        0.0
    }
}
