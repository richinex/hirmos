//! Covariate-adjusted decomposition from bacondecomp 0.1.1.
use super::{Error, Panel};
use crate::ols::Ols;
use nalgebra::{DMatrix, DVector};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    TreatedVsNever { adoption: i64 },
    BothTreated { earlier: i64, later: i64 },
    LaterVsAlways { adoption: i64 },
}
#[derive(Debug)]
pub struct Component {
    pub comparison: Comparison,
    pub estimate: f64,
    pub weight: f64,
}
#[derive(Debug)]
pub struct Decomposition {
    pub within_estimate: f64,
    pub within_weight: f64,
    /// Conditional between weights sum to one; total mass is 1-Omega.
    pub between: Vec<Component>,
    pub twfe: f64,
    pub reconstructed: f64,
}
fn means<K: Ord + Clone>(v: &[f64], keys: &[K]) -> Vec<f64> {
    let mut m = BTreeMap::<K, (f64, usize)>::new();
    for (v, k) in v.iter().zip(keys) {
        let e = m.entry(k.clone()).or_default();
        e.0 += v;
        e.1 += 1;
    }
    keys.iter().map(|k| m[k].0 / m[k].1 as f64).collect()
}
fn cov(a: &[f64], b: &[f64]) -> f64 {
    let am = a.iter().sum::<f64>() / a.len() as f64;
    let bm = b.iter().sum::<f64>() / b.len() as f64;
    a.iter()
        .zip(b)
        .map(|(a, b)| (a - am) * (b - bm))
        .sum::<f64>()
        / a.len() as f64
}
fn fitted(y: &[f64], f: &Ols) -> Vec<f64> {
    y.iter().zip(f.resid.iter()).map(|(y, r)| y - r).collect()
}
impl Panel {
    fn bacon_projection(&self, rows: &[usize], y: &[f64], x: &[Vec<f64>]) -> Result<Ols, Error> {
        let design = self
            .with_effects(rows, x, (0..x[0].len()).map(|j| format!("x{j}")).collect())
            .map_err(|_| Error::Numerical)?;
        let fit = Ols::try_fit(&design.matrix, &DVector::from_column_slice(y))
            .map_err(|_| Error::Numerical)?;
        if fit.rank != design.matrix.ncols() {
            return Err(Error::RankDeficientProjection);
        }
        Ok(fit)
    }
    pub fn bacon_adjusted(
        &self,
        outcome: usize,
        treatment: usize,
        controls: &[usize],
    ) -> Result<Decomposition, Error> {
        let n = self.keys.len();
        let columns: Vec<_> = [vec![outcome, treatment], controls.to_vec()].concat();
        if controls.is_empty()
            || columns.iter().any(|c| *c >= self.values.ncols())
            || columns.iter().collect::<BTreeSet<_>>().len() != columns.len()
        {
            return Err(Error::InvalidColumns);
        }
        let adoption = self.bacon_adoption(treatment)?;
        let group: Vec<_> = self.keys.iter().map(|k| adoption[&k.0]).collect();
        let units: Vec<_> = self.keys.iter().map(|k| k.0).collect();
        let times: Vec<_> = self.keys.iter().map(|k| k.1).collect();
        let gt: Vec<_> = group.iter().copied().zip(times.iter().copied()).collect();
        let rows: Vec<_> = (0..n).collect();
        let y: Vec<_> = (0..n).map(|i| self.values[(i, outcome)]).collect();
        let d: Vec<_> = (0..n).map(|i| self.values[(i, treatment)]).collect();
        let x: Vec<Vec<_>> = (0..n)
            .map(|i| controls.iter().map(|c| self.values[(i, *c)]).collect())
            .collect();
        let fwl = self.bacon_projection(&rows, &d, &x)?;
        let r: Vec<_> = fwl.resid.iter().copied().collect();
        let p = fitted(&d, &fwl);
        let ri = means(&r, &units);
        let rt = means(&r, &times);
        let rgt = means(&r, &gt);
        let rg = means(&r, &group);
        let grand = r.iter().sum::<f64>() / n as f64;
        let dw: Vec<_> = (0..n).map(|i| r[i] - ri[i] - rgt[i] + rg[i]).collect();
        let dt: Vec<_> = (0..n).map(|i| r[i] - ri[i] - rt[i] + grand).collect();
        let vw = cov(&dw, &dw);
        let vt = cov(&dt, &dt);
        if vw <= f64::EPSILON * vt || vt <= 0. {
            return Err(Error::DegenerateWithin);
        }
        let within_weight = vw / vt;
        let within_estimate = cov(&y, &dw) / vw;
        let gp = means(&p, &gt);
        let gx: Vec<Vec<_>> = (0..controls.len())
            .map(|j| means(&x.iter().map(|r| r[j]).collect::<Vec<_>>(), &gt))
            .collect();
        let groups: BTreeSet<_> = group.iter().copied().collect();
        let first = *times.iter().min().ok_or(Error::NoComparisons)?;
        let mut between = Vec::new();
        for g in groups.iter().flatten().copied() {
            for &h in &groups {
                if h.is_some_and(|h| h <= g) {
                    continue;
                }
                let rows: Vec<_> = (0..n)
                    .filter(|i| group[*i] == Some(g) || group[*i] == h)
                    .collect();
                let select = |v: &[f64]| rows.iter().map(|i| v[*i]).collect::<Vec<_>>();
                let dy = select(&y);
                let dd = select(&d);
                let empty = vec![vec![]; rows.len()];
                let dfit = self.bacon_projection(&rows, &dd, &empty)?;
                let dtil: Vec<_> = dfit.resid.iter().copied().collect();
                let vd = cov(&dtil, &dtil);
                let gxx: Vec<Vec<_>> = rows
                    .iter()
                    .map(|i| gx.iter().map(|c| c[*i]).collect())
                    .collect();
                let px: Vec<Vec<f64>> = gx
                    .iter()
                    .map(|c| {
                        self.bacon_projection(&rows, &select(c), &empty)
                            .map(|f| f.resid.iter().copied().collect())
                    })
                    .collect::<Result<_, _>>()?;
                let pm = DMatrix::from_fn(rows.len(), controls.len() + 1, |i, j| {
                    if j == 0 {
                        1.
                    } else {
                        px[j - 1][i]
                    }
                });
                let pf = Ols::try_fit(&pm, &DVector::from_column_slice(&dtil))
                    .map_err(|_| Error::Numerical)?;
                if pf.rank != pm.ncols() {
                    return Err(Error::RankDeficientProjection);
                }
                let pg = fitted(&dtil, &pf);
                let unexplained = pf.ssr / (rows.len() as f64 * vd);
                let pt = self.bacon_projection(&rows, &select(&gp), &empty)?;
                let dp: Vec<_> = pg.iter().zip(pt.resid.iter()).map(|(a, b)| a - b).collect();
                let vdp = cov(&dp, &dp);
                let bx: Vec<Vec<_>> = dd
                    .iter()
                    .zip(&gxx)
                    .map(|(d, x)| {
                        let mut r = vec![*d];
                        r.extend(x);
                        r
                    })
                    .collect();
                let bd = self.bacon_projection(&rows, &dy, &bx)?.params[1];
                let bb = self
                    .bacon_projection(&rows, &dy, &dp.iter().map(|v| vec![*v]).collect::<Vec<_>>())?
                    .params[1];
                let mass = unexplained * vd + vdp;
                let estimate = (unexplained * vd * bd + vdp * bb) / mass;
                let weight = (rows.len() as f64).powi(2) * mass;
                if !estimate.is_finite() || !weight.is_finite() || weight <= 0. {
                    return Err(Error::Numerical);
                }
                let comparison = match h {
                    None => Comparison::TreatedVsNever { adoption: g },
                    Some(h) if g == first => Comparison::LaterVsAlways { adoption: h },
                    Some(h) => Comparison::BothTreated {
                        earlier: g,
                        later: h,
                    },
                };
                between.push(Component {
                    comparison,
                    estimate,
                    weight,
                });
            }
        }
        if between.is_empty() {
            return Err(Error::NoComparisons);
        }
        let total: f64 = between.iter().map(|c| c.weight).sum();
        if !total.is_finite() || total <= 0. {
            return Err(Error::Numerical);
        }
        for c in &mut between {
            c.weight /= total;
        }
        let reconstructed = within_weight * within_estimate
            + (1. - within_weight) * between.iter().map(|c| c.weight * c.estimate).sum::<f64>();
        let full: Vec<Vec<_>> = d
            .iter()
            .zip(x)
            .map(|(d, x)| {
                let mut r = vec![*d];
                r.extend(x);
                r
            })
            .collect();
        let twfe = self.bacon_projection(&rows, &y, &full)?.params[1];
        if !within_weight.is_finite()
            || !within_estimate.is_finite()
            || !reconstructed.is_finite()
            || !twfe.is_finite()
        {
            return Err(Error::Numerical);
        }
        Ok(Decomposition {
            within_estimate,
            within_weight,
            between,
            twfe,
            reconstructed,
        })
    }
}
