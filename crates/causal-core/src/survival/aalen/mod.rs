//! Aalen additive regression, following survival 3.8-3's aareg.

mod curves;
mod influence;
pub(crate) mod qr;
mod summary;
pub use curves::{coefficient_curves, CoefficientPoint};
pub use influence::Influence;
pub use summary::{summarize, summarize_through, Summary, SummaryTable};

use qr::Qr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestWeight {
    Aalen,
    AtRisk,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub test: TestWeight,
    pub nmin: Option<usize>,
    pub tolerance: f64,
    pub taper: Vec<f64>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            test: TestWeight::Aalen,
            nmin: None,
            tolerance: 1e-7,
            taper: vec![1.0],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidShape,
    InvalidValue,
    InvalidInterval,
    InvalidOptions,
    TooFewEvents,
    SingularRiskSet,
}

/// Validated counting-process rows; covariates do not include the intercept.
pub struct Data {
    x: Vec<Vec<f64>>,
    intervals: Vec<[f64; 3]>,
    weights: Vec<f64>,
    clusters: Option<Vec<usize>>,
}

impl Data {
    pub fn new(
        x: Vec<Vec<f64>>,
        intervals: Vec<[f64; 3]>,
        weights: Vec<f64>,
    ) -> Result<Self, Error> {
        let n = x.len();
        let p = x.first().map_or(0, Vec::len);
        if n == 0
            || p == 0
            || intervals.len() != n
            || weights.len() != n
            || x.iter().any(|row| row.len() != p)
        {
            return Err(Error::InvalidShape);
        }
        if x.iter().flatten().any(|v| !v.is_finite())
            || weights.iter().any(|w| !w.is_finite() || *w <= 0.0)
        {
            return Err(Error::InvalidValue);
        }
        if intervals.iter().any(|row| {
            row.iter().any(|v| !v.is_finite())
                || row[0] >= row[1]
                || (row[2] != 0.0 && row[2] != 1.0)
        }) {
            return Err(Error::InvalidInterval);
        }
        Ok(Self {
            x,
            intervals,
            weights,
            clusters: None,
        })
    }

    /// Request clustered uncertainty using one group identifier per input row.
    pub fn with_clusters(mut self, clusters: Vec<usize>) -> Result<Self, Error> {
        if clusters.len() != self.x.len() {
            return Err(Error::InvalidShape);
        }
        self.clusters = Some(clusters);
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct Fit {
    test: TestWeight,
    pub n: [usize; 3],
    pub times: Vec<f64>,
    pub nrisk: Vec<f64>,
    pub coefficient: Vec<Vec<f64>>,
    pub tweight: Vec<Vec<f64>>,
    pub statistic: Vec<f64>,
    pub variance: Vec<Vec<f64>>,
    pub influence: Option<Influence>,
}

struct RiskSet {
    time: f64,
    weight: f64,
    mean: Vec<f64>,
    covariance: Vec<f64>,
    deaths: Vec<usize>,
}

/// Fit event-specific additive coefficients and the weighted omnibus-test inputs.
pub fn fit(data: &Data, options: &Options) -> Result<Fit, Error> {
    fit_internal(data, options, data.clusters.is_some())
}

/// Include individual influence increments when no clustering was requested.
pub fn fit_with_influence(data: &Data, options: &Options) -> Result<Fit, Error> {
    fit_internal(data, options, true)
}

fn fit_internal(data: &Data, options: &Options, robust: bool) -> Result<Fit, Error> {
    if !options.tolerance.is_finite()
        || options.tolerance <= 0.0
        || options.taper.is_empty()
        || options.taper.iter().any(|v| !v.is_finite() || *v <= 0.0)
    {
        return Err(Error::InvalidOptions);
    }
    let n = data.x.len();
    let p = data.x[0].len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        data.intervals[a][1]
            .total_cmp(&data.intervals[b][1])
            .then_with(|| data.intervals[b][2].total_cmp(&data.intervals[a][2]))
    });
    let center: Vec<f64> = (0..p)
        .map(|j| data.x.iter().map(|row| row[j]).sum::<f64>() / n as f64)
        .collect();
    let mut sets = risk_sets(data, &order, &center);
    // A vanished predictor leaves cancellation noise in coxdetail. Reproduce
    // its accumulation order before applying R's relative QR rank decision.
    for set in &mut sets {
        if (0..p)
            .any(|j| set.covariance[j + j * p].abs() < 1e-12 * center[j].abs().max(1.0).powi(2))
        {
            source_moments(set, data, &order, &center);
        }
    }
    let count = sets.len();
    if sets.iter().any(|set| {
        !set.weight.is_finite()
            || set
                .mean
                .iter()
                .chain(&set.covariance)
                .any(|v| !v.is_finite())
    }) {
        return Err(Error::InvalidValue);
    }
    let raw: Vec<Vec<f64>> = sets.iter().map(|set| set.covariance.clone()).collect();
    let taper = &options.taper[..options.taper.len().min(count)];
    for (i, set) in sets.iter_mut().enumerate() {
        let length = taper.len().min(i + 1);
        let weights = &taper[taper.len() - length..];
        let sum: f64 = weights.iter().sum();
        for cell in 0..p * p {
            set.covariance[cell] = (0..length)
                .map(|k| raw[i + 1 - length + k][cell] * weights[k] / sum)
                .sum();
        }
    }
    let nmin = options.nmin.unwrap_or(3 * p) as f64;
    let mut kept = sets
        .iter()
        .filter(|set| set.weight >= nmin && (p != 1 || set.covariance[0] > 0.0))
        .count();
    while kept > 0 && Qr::new(sets[kept - 1].covariance.clone(), p, options.tolerance).rank < p {
        kept -= 1;
    }
    if kept <= 1 {
        return Err(Error::TooFewEvents);
    }
    let mut result = Fit {
        test: options.test,
        n: [n, kept, count],
        times: Vec::new(),
        nrisk: Vec::new(),
        coefficient: Vec::new(),
        tweight: Vec::new(),
        statistic: vec![0.0; p + 1],
        variance: vec![vec![0.0; p + 1]; p + 1],
        influence: None,
    };
    for set in &sets[..kept] {
        let qr = Qr::new(set.covariance.clone(), p, options.tolerance);
        let inverse: Vec<Vec<f64>> = (0..p)
            .map(|j| {
                let mut unit = vec![0.0; p];
                unit[j] = 1.0;
                qr.solve(&unit).ok_or(Error::SingularRiskSet)
            })
            .collect::<Result<_, _>>()?;
        let mut twt = vec![0.0; p + 1];
        let quadratic: f64 = (0..p)
            .map(|i| {
                (0..p)
                    .map(|j| set.mean[i] * inverse[j][i] * set.mean[j])
                    .sum::<f64>()
            })
            .sum();
        twt[0] = set.weight / (1.0 + quadratic);
        for i in 0..p {
            twt[i + 1] = set.weight / inverse[i][i];
        }
        for &row in &set.deaths {
            let rhs: Vec<f64> = (0..p)
                .map(|j| data.weights[row] * (data.x[row][j] - set.mean[j]))
                .collect();
            let slopes = qr.solve(&rhs).ok_or(Error::SingularRiskSet)?;
            let mut coef = vec![0.0; p + 1];
            for j in 0..p {
                coef[j + 1] = slopes[j] / set.weight;
            }
            coef[0] = data.weights[row] / set.weight
                - (0..p).map(|j| set.mean[j] * coef[j + 1]).sum::<f64>();
            let tested: Vec<f64> = (0..=p)
                .map(|j| {
                    coef[j]
                        * match options.test {
                            TestWeight::Aalen => twt[j],
                            TestWeight::AtRisk => set.weight,
                        }
                })
                .collect();
            for i in 0..=p {
                result.statistic[i] += tested[i];
                for j in 0..=p {
                    result.variance[i][j] += tested[i] * tested[j];
                }
            }
            result.times.push(set.time);
            result.nrisk.push(set.weight);
            result.coefficient.push(coef);
            result.tweight.push(twt.clone());
        }
    }
    if robust {
        result.influence = Some(influence::calculate(
            data,
            options,
            &order,
            &sets[..kept],
            &result,
        )?);
    }
    Ok(result)
}

#[derive(Clone, Copy, Default)]
struct Sum {
    value: f64,
    correction: f64,
}

impl Sum {
    fn add(&mut self, value: f64) {
        let adjusted = value - self.correction;
        let next = self.value + adjusted;
        self.correction = (next - self.value) - adjusted;
        self.value = next;
    }
}

// Each row enters and leaves once. There is no subjects-by-event-times matrix.
fn risk_sets(data: &Data, stops: &[usize], center: &[f64]) -> Vec<RiskSet> {
    let n = stops.len();
    let p = center.len();
    let mut starts = stops.to_vec();
    starts.sort_by(|&a, &b| data.intervals[a][0].total_cmp(&data.intervals[b][0]));
    let mut sums = vec![Sum::default(); 1 + p + p * p];
    let mut entered = 0;
    let mut exited = 0;
    let mut position = 0;
    let mut sets = Vec::new();
    while position < n {
        let time = data.intervals[stops[position]][1];
        let mut end = position + 1;
        while end < n && data.intervals[stops[end]][1] == time {
            end += 1;
        }
        let deaths: Vec<usize> = stops[position..end]
            .iter()
            .copied()
            .filter(|&row| data.intervals[row][2] == 1.0)
            .collect();
        position = end;
        if deaths.is_empty() {
            continue;
        }
        while entered < n && data.intervals[starts[entered]][0] < time {
            update_sums(&mut sums, data, starts[entered], center, 1.0);
            entered += 1;
        }
        while exited < n && data.intervals[stops[exited]][1] < time {
            update_sums(&mut sums, data, stops[exited], center, -1.0);
            exited += 1;
        }
        let weight = sums[0].value;
        let mut mean: Vec<f64> = (0..p).map(|j| sums[1 + j].value / weight).collect();
        let mut covariance = vec![0.0; p * p];
        for i in 0..p {
            for j in 0..=i {
                let v = sums[1 + p + i + j * p].value / weight - mean[i] * mean[j];
                covariance[i + j * p] = v;
                covariance[j + i * p] = v;
            }
        }
        for j in 0..p {
            mean[j] += center[j];
        }
        sets.push(RiskSet {
            time,
            weight,
            mean,
            covariance,
            deaths,
        });
    }
    sets
}

fn update_sums(sums: &mut [Sum], data: &Data, row: usize, center: &[f64], sign: f64) {
    let p = center.len();
    let weight = sign * data.weights[row];
    sums[0].add(weight);
    for i in 0..p {
        let xi = data.x[row][i] - center[i];
        sums[1 + i].add(weight * xi);
        for j in 0..=i {
            sums[1 + p + i + j * p].add(weight * xi * (data.x[row][j] - center[j]));
        }
    }
}

fn source_moments(set: &mut RiskSet, data: &Data, order: &[usize], center: &[f64]) {
    let p = center.len();
    let mut weight = 0.0;
    let mut first = vec![0.0; p];
    let mut second = vec![0.0; p * p];
    let first_at_risk = order.partition_point(|&row| data.intervals[row][1] < set.time);
    for &row in &order[first_at_risk..] {
        if data.intervals[row][0] >= set.time || data.intervals[row][1] < set.time {
            continue;
        }
        let w = data.weights[row];
        weight += w;
        for i in 0..p {
            let xi = data.x[row][i] - center[i];
            first[i] = w.mul_add(xi, first[i]);
            for j in 0..=i {
                second[i + j * p] = (w * xi).mul_add(data.x[row][j] - center[j], second[i + j * p]);
            }
        }
    }
    let event_weight: f64 = set.deaths.iter().map(|&row| data.weights[row]).sum();
    let mean_weight = event_weight / set.deaths.len() as f64;
    set.weight = weight;
    set.mean.fill(0.0);
    set.covariance.fill(0.0);
    for _ in &set.deaths {
        for i in 0..p {
            let mean = first[i] / weight;
            set.mean[i] += (center[i] + mean) / set.deaths.len() as f64;
            for j in 0..=i {
                let value = ((-mean).mul_add(first[j], second[i + j * p]) / weight) * mean_weight;
                set.covariance[i + j * p] += value;
                if i != j {
                    set.covariance[j + i * p] += value;
                }
            }
        }
    }
    for value in &mut set.covariance {
        *value /= event_weight;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn veteran_tail_moments() {
        let fixtures: serde_json::Value =
            serde_json::from_str(include_str!("../../../oracle/fixtures/aalen.json")).unwrap();
        let case = fixtures["cases"].as_array().unwrap().last().unwrap();
        let data = Data::new(
            serde_json::from_value(case["input"]["x"].clone()).unwrap(),
            serde_json::from_value(case["input"]["y"].clone()).unwrap(),
            serde_json::from_value(case["input"]["weights"].clone()).unwrap(),
        )
        .unwrap();
        let mut order: Vec<usize> = (0..data.x.len()).collect();
        order.sort_by(|&a, &b| {
            data.intervals[a][1]
                .total_cmp(&data.intervals[b][1])
                .then_with(|| data.intervals[b][2].total_cmp(&data.intervals[a][2]))
        });
        let p = data.x[0].len();
        let center: Vec<f64> = (0..p)
            .map(|j| data.x.iter().map(|row| row[j]).sum::<f64>() / data.x.len() as f64)
            .collect();
        let mut sets = risk_sets(&data, &order, &center);
        for (k, set) in sets.iter_mut().take(75).enumerate().skip(73) {
            source_moments(set, &data, &order, &center);
            let reference = &case["qr"][k + 1]["input"];
            for i in 0..p {
                for j in 0..p {
                    let expected = reference[i][j].as_f64().unwrap();
                    assert_eq!(
                        set.covariance[i + j * p],
                        expected,
                        "time={} [{i},{j}]",
                        set.time
                    );
                }
            }
        }
    }
}
