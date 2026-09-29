// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// Source-level port of GRF 2.6.1 RandomSampler / SamplingOptions and bundled
// nonstd uniform integer/real sampling and shuffle. See COPYING.
use crate::mt19937_64::Mt19937_64;
use std::collections::{BTreeMap, BTreeSet};

/// Preserve first-occurrence cluster IDs and source order within each cluster.
pub struct Clusters {
    groups: Vec<Vec<usize>>,
    per_cluster: usize,
}
impl Clusters {
    /// Transport sampling membership without exposing mutable cluster internals.
    pub fn membership(&self) -> (Vec<usize>, usize) {
        let mut labels = vec![0; self.row_count()];
        for (group, rows) in self.groups.iter().enumerate() {
            for &row in rows { labels[row] = group; }
        }
        (labels, self.per_cluster)
    }
    pub(crate) fn row_count(&self) -> usize {
        self.groups.iter().map(Vec::len).sum()
    }
    pub fn new(labels: &[usize], per_cluster: usize) -> Result<Self, &'static str> {
        if labels.is_empty() || per_cluster == 0 {
            return Err("Clusters and per-cluster count must be nonempty.");
        }
        let mut ids = BTreeMap::new();
        let mut groups = Vec::<Vec<usize>>::new();
        for (row, label) in labels.iter().enumerate() {
            let next = ids.len();
            let id = *ids.entry(*label).or_insert(next);
            if id == groups.len() {
                groups.push(Vec::new());
            }
            groups[id].push(row);
        }
        Ok(Self {
            groups,
            per_cluster,
        })
    }
}

pub struct Sampler {
    engine: Mt19937_64,
}
impl Sampler {
    pub fn new(seed: u32) -> Self {
        Self {
            engine: Mt19937_64::new(seed as u64),
        }
    }

    /// GRF uses low-bit rejection, not ranger's multiply-high distribution.
    pub fn index(&mut self, count: usize) -> Result<usize, &'static str> {
        if count == 0 {
            return Err("Integer sampling requires a positive range.");
        }
        if count == 1 {
            return Ok(0);
        }
        let bits = usize::BITS - (count - 1).leading_zeros();
        let mask = u64::MAX >> (64 - bits);
        loop {
            let draw = self.engine.next() & mask;
            if draw < count as u64 {
                return Ok(draw as usize);
            }
        }
    }

    fn unit(&mut self) -> f64 {
        self.engine.next() as f64 / 18446744073709551616.0
    }

    // GRF constructs a fresh normal distribution for every Poisson draw.
    // Its second polar variate is discarded, not cached across calls.
    fn normal(&mut self) -> f64 {
        loop {
            let u = 2.0 * self.unit() - 1.0;
            let v = 2.0 * self.unit() - 1.0;
            let s = u * u + v * v;
            if s <= 1.0 && s != 0.0 {
                return u * (-2.0 * s.ln() / s).sqrt();
            }
        }
    }

    /// Bundled nonstd Poisson distribution used for the candidate feature count.
    pub fn poisson(&mut self, mean: u32) -> usize {
        let mean = mean as f64;
        if mean < 10.0 {
            let limit = (-mean).exp();
            let mut x = 0;
            let mut p = self.unit();
            while p > limit {
                x += 1;
                p *= self.unit();
            }
            return x;
        }
        let s = mean.sqrt();
        let d = 6.0 * mean * mean;
        let l = (mean - 1.1484).trunc();
        let omega = 0.3989423 / s;
        let b1 = 0.4166667E-1 / mean;
        let b2 = 0.3 * b1 * b1;
        let c3 = 0.1428571 * b1 * b2;
        let c2 = b2 - 15.0 * c3;
        let c1 = b1 - 6.0 * b2 + 45.0 * c3;
        let c0 = 1.0 - b1 + 3.0 * b2 - 15.0 * c3;
        let c = 0.1069 / mean;
        let g = mean + s * self.normal();
        let mut x = 0;
        let mut dif = 0.0;
        let mut u = 0.0;
        if g > 0.0 {
            x = g as usize;
            if x as f64 >= l {
                return x;
            }
            dif = mean - x as f64;
            u = self.unit();
            if d * u >= dif * dif * dif {
                return x;
            }
        }
        let mut using_exp = false;
        loop {
            let mut e = 0.0;
            if using_exp || g <= 0.0 {
                let t = loop {
                    e = -(1.0 - self.unit()).ln();
                    u = self.unit();
                    u += u - 1.0;
                    let t = 1.8 + if u < 0.0 { -e } else { e };
                    if t > -0.6744 {
                        break t;
                    }
                };
                x = (mean + s * t) as usize;
                dif = mean - x as f64;
                using_exp = true;
            }
            let xf = x as f64;
            let (px, py) = if x < 10 {
                let factorial = [
                    1.0, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0, 5040.0, 40320.0, 362880.0,
                ];
                (-mean, mean.powf(xf) / factorial[x])
            } else {
                let mut del = 0.8333333E-1 / xf;
                del -= 4.8 * del * del * del;
                let v = dif / xf;
                let px = if v.abs() > 0.25 {
                    xf * (1.0 + v).ln() - dif - del
                } else {
                    xf * v
                        * v
                        * (((((((0.1250060 * v - 0.1384794) * v + 0.1421878) * v - 0.1661269)
                            * v
                            + 0.2000118)
                            * v
                            - 0.2500068)
                            * v
                            + 0.3333333)
                            * v
                            - 0.5)
                        - del
                };
                (px, 0.3989423 / xf.sqrt())
            };
            let r = (0.5 - dif) / s;
            let r2 = r * r;
            let fx = -0.5 * r2;
            let fy = omega * (((c3 * r2 + c2) * r2 + c1) * r2 + c0);
            let accepted = if using_exp {
                c * u.abs() <= py * (px + e).exp() - fy * (fx + e).exp()
            } else {
                fy - u * fy <= py * (px - fx).exp()
            };
            if accepted {
                return x;
            }
            using_exp = true;
        }
    }

    pub fn shuffle<T>(&mut self, values: &mut [T]) {
        for i in 0..values.len().saturating_sub(1) {
            let offset = self
                .index(values.len() - i)
                .expect("positive remaining range");
            if offset != 0 {
                values.swap(i, i + offset);
            }
        }
    }

    pub fn sample(&mut self, rows: usize, fraction: f64) -> Result<Vec<usize>, &'static str> {
        check_fraction(fraction)?;
        let mut samples: Vec<_> = (0..rows).collect();
        self.shuffle(&mut samples);
        samples.truncate((rows as f64 * fraction) as usize); // upstream floor
        Ok(samples)
    }

    pub fn subsample(
        &mut self,
        samples: &[usize],
        fraction: f64,
    ) -> Result<(Vec<usize>, Vec<usize>), &'static str> {
        check_fraction(fraction)?;
        let mut selected = samples.to_vec();
        self.shuffle(&mut selected);
        let rest = selected.split_off((samples.len() as f64 * fraction).ceil() as usize);
        Ok((selected, rest))
    }

    pub fn draw(
        &mut self,
        max: usize,
        skip: &BTreeSet<usize>,
        count: usize,
    ) -> Result<Vec<usize>, &'static str> {
        if skip.iter().any(|&i| i >= max) || count > max - skip.len() {
            return Err("Invalid feature sampling range or exclusions.");
        }
        if count < max / 10 {
            let mut selected = vec![false; max];
            let mut result = Vec::with_capacity(count);
            while result.len() < count {
                let mut draw = self.index(max - skip.len())?;
                for &excluded in skip {
                    if draw >= excluded {
                        draw += 1;
                    }
                }
                if !selected[draw] {
                    selected[draw] = true;
                    result.push(draw);
                }
            }
            Ok(result)
        } else {
            let mut result: Vec<_> = (0..max).filter(|i| !skip.contains(i)).collect();
            for i in 0..count {
                // Match the upstream addition before truncation.
                let j = (i as f64 + self.unit() * (max - skip.len() - i) as f64) as usize;
                if j >= result.len() {
                    return Err("Uniform draw rounded outside feature range.");
                }
                result.swap(i, j);
            }
            result.truncate(count);
            Ok(result)
        }
    }

    pub fn sample_clusters(
        &mut self,
        clusters: &Clusters,
        fraction: f64,
    ) -> Result<Vec<usize>, &'static str> {
        self.sample(clusters.groups.len(), fraction)
    }

    pub fn rows_in_clusters(
        &mut self,
        clusters: &Clusters,
        ids: &[usize],
        subsample: bool,
    ) -> Result<Vec<usize>, &'static str> {
        if ids.iter().any(|&id| id >= clusters.groups.len()) {
            return Err("Unknown cluster ID.");
        }
        let mut rows = Vec::new();
        for &id in ids {
            let group = &clusters.groups[id];
            if subsample && group.len() > clusters.per_cluster {
                let mut selected = group.clone();
                self.shuffle(&mut selected);
                rows.extend_from_slice(&selected[..clusters.per_cluster]);
            } else {
                rows.extend_from_slice(group);
            }
        }
        Ok(rows)
    }
}
fn check_fraction(value: f64) -> Result<(), &'static str> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        Err("Sampling fraction must lie between zero and one.")
    } else {
        Ok(())
    }
}
