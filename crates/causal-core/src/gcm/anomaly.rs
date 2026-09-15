//! DoWhy v0.14 attribute_anomaly_scores composition.

use super::shapley::{self, Execution, Method, ShapleyError};
use super::{
    model::{FittedModel, ModelError},
    scoring::MedianCdf,
};

#[derive(Debug)]
pub enum AnomalyError {
    Model(ModelError),
    Shapley(ShapleyError),
}

pub struct AnomalyAttributions {
    pub nodes: Vec<usize>,
    pub values: Vec<Vec<f64>>,
}

/// Default information-theoretic attribution with an explicit distribution sample count.
pub fn attribute_anomalies(
    model: &FittedModel,
    target: usize,
    anomalies: &DMatrix<f64>,
    count: std::num::NonZeroUsize,
    execution: Execution,
    rng: &mut Mt19937,
    normal_cache: &mut Option<f64>,
) -> Result<AnomalyAttributions, AnomalyError> {
    let observed_noise = model
        .noise_from_data(anomalies)
        .map_err(AnomalyError::Model)?;
    let (sampled, noise) = model
        .sample_ancestors(target, count.get(), rng, normal_cache)
        .map_err(AnomalyError::Model)?;
    let nodes = model.ancestor_order(target).map_err(AnomalyError::Model)?;
    let scorer = MedianCdf::fit(sampled.column(target).as_slice());
    let selected_anomalies = DMatrix::from_fn(anomalies.nrows(), nodes.len(), |r, c| {
        observed_noise[(r, nodes[c])]
    });
    let selected_noise = DMatrix::from_fn(count.get(), nodes.len(), |r, c| noise[(r, nodes[c])]);
    let values = attribute_scores(
        &selected_anomalies,
        &selected_noise,
        Attribution::InformationTheoretic,
        Method::Auto,
        execution,
        rng,
        |features| {
            let mut noise = DMatrix::zeros(features.nrows(), model.graph().names().len());
            for (column, &node) in nodes.iter().enumerate() {
                noise.set_column(node, &features.column(column));
            }
            let data = model
                .data_from_noise(&noise)
                .map_err(|_| ShapleyError::NonFinite)?;
            Ok(data
                .column(target)
                .iter()
                .map(|&v| scorer.score(v))
                .collect())
        },
    )
    .map_err(AnomalyError::Shapley)?;
    Ok(AnomalyAttributions { nodes, values })
}
use crate::{nprandom::Mt19937, numpy_reduce::numpy_mean};
use nalgebra::DMatrix;

#[derive(Clone, Copy, Debug)]
pub enum Attribution {
    InformationTheoretic,
    MeanDeviation,
}

/// Attribute each sample's score to its noise features. Output rows follow anomaly rows.
pub fn attribute_scores<F>(
    anomalies: &DMatrix<f64>,
    distribution: &DMatrix<f64>,
    attribution: Attribution,
    method: Method,
    execution: Execution,
    rng: &mut Mt19937,
    score: F,
) -> Result<Vec<Vec<f64>>, ShapleyError>
where
    F: Fn(&DMatrix<f64>) -> Result<Vec<f64>, ShapleyError>,
{
    if anomalies.ncols() != distribution.ncols()
        || anomalies.ncols() == 0
        || anomalies.nrows() == 0
        || distribution.nrows() == 0
    {
        return Err(ShapleyError::OutputShape);
    }
    enum Baseline {
        Scores(Vec<f64>),
        Mean(f64),
    }
    let checked_score = |data: &DMatrix<f64>| {
        let values = score(data)?;
        if values.len() != data.nrows() {
            return Err(ShapleyError::OutputShape);
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(ShapleyError::NonFinite);
        }
        Ok(values)
    };
    let baseline = match attribution {
        Attribution::InformationTheoretic => Baseline::Scores(checked_score(anomalies)?),
        Attribution::MeanDeviation => Baseline::Mean(numpy_mean(&checked_score(distribution)?)),
    };
    shapley::estimate(
        anomalies.ncols().try_into().unwrap(),
        method,
        execution,
        rng,
        |subset, stream| {
            // The source consumes a permutation even for the full coalition.
            let order = stream.permutation(distribution.nrows());
            let mut features =
                DMatrix::from_fn(distribution.nrows(), distribution.ncols(), |r, c| {
                    distribution[(if subset[c] { r } else { order[r] }, c)]
                });
            let mut results = Vec::with_capacity(anomalies.nrows());
            for row in 0..anomalies.nrows() {
                for column in 0..anomalies.ncols() {
                    if subset[column] {
                        features.column_mut(column).fill(anomalies[(row, column)]);
                    }
                }
                let values = checked_score(&features)?;
                results.push(match &baseline {
                    Baseline::Mean(mean) => numpy_mean(&values) - mean,
                    Baseline::Scores(scores) => {
                        let count = values.iter().filter(|&&v| v >= scores[row]).count();
                        ((count as f64 + 0.5) / (values.len() as f64 + 0.5)).ln()
                    }
                });
            }
            Ok(results)
        },
    )
}
