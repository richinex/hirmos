//! Empirical roots and forest mechanisms for forward interventions.
//!
//! This model deliberately has no noise-inversion API: a classifier FCM is not invertible.

use super::model::{Graph, ModelError};
use crate::{
    nprandom::Mt19937,
    sktree::{fit_forest_with_rng, RandomForest},
};
use nalgebra::DMatrix;
use std::{collections::BTreeMap, num::NonZeroUsize};

pub struct Forest {
    pub trees: NonZeroUsize,
    pub min_leaf: NonZeroUsize,
}

pub enum MechanismSpec {
    Empirical,
    Regression(Forest),
    /// Encoded classes follow the source's sorted string-label order.
    Classifier {
        forest: Forest,
        classes: NonZeroUsize,
    },
}

enum Mechanism {
    Empirical(Vec<f64>),
    Regression {
        forest: RandomForest,
        residuals: Vec<f64>,
    },
    Classifier(RandomForest),
}

#[derive(Debug)]
pub enum Error {
    Model(ModelError),
    EmptyData,
    Specification,
    UnsupportedEncoding { node: usize },
    InvalidIntervention,
}

/// Finite intervention values. Uniform choices cannot be empty.
pub struct Intervention {
    operation: Operation,
}

enum Operation {
    Shift(f64),
    Set(f64),
    Uniform(Vec<f64>),
}

impl Intervention {
    pub fn shift(value: f64) -> Option<Self> {
        value.is_finite().then_some(Self {
            operation: Operation::Shift(value),
        })
    }
    pub fn set(value: f64) -> Option<Self> {
        value.is_finite().then_some(Self {
            operation: Operation::Set(value),
        })
    }
    pub fn uniform(values: Vec<f64>) -> Option<Self> {
        (!values.is_empty() && values.iter().all(|v| v.is_finite())).then_some(Self {
            operation: Operation::Uniform(values),
        })
    }
    fn apply(&self, value: f64, rng: &mut Mt19937) -> f64 {
        match &self.operation {
            Operation::Shift(shift) => value + shift,
            Operation::Set(value) => *value,
            Operation::Uniform(values) => values[rng.randint(values.len() as u64) as usize],
        }
    }
}

enum Encoding {
    Numeric,
    OneHot(NonZeroUsize),
}

pub struct Model {
    graph: Graph,
    mechanisms: Vec<Mechanism>,
    encodings: Vec<Encoding>,
}

fn parents(
    graph: &Graph,
    node: usize,
    data: &DMatrix<f64>,
    encodings: &[Encoding],
) -> Vec<Vec<f64>> {
    let columns = graph.parents(node).expect("validated graph node");
    (0..data.nrows())
        .map(|row| {
            let mut values = Vec::new();
            for &column in columns {
                match encodings[column] {
                    Encoding::Numeric => values.push(data[(row, column)]),
                    Encoding::OneHot(count) => values.extend(
                        (0..count.get())
                            .map(|class| f64::from(data[(row, column)] == class as f64)),
                    ),
                }
            }
            values
        })
        .collect()
}

impl Model {
    pub fn fit(
        graph: Graph,
        data: &DMatrix<f64>,
        specs: Vec<MechanismSpec>,
        rng: &mut Mt19937,
    ) -> Result<Self, Error> {
        graph.validate(data).map_err(Error::Model)?;
        if data.nrows() == 0 {
            return Err(Error::EmptyData);
        }
        if specs.len() != graph.names().len() {
            return Err(Error::Specification);
        }
        let encodings: Vec<_> = specs
            .iter()
            .map(|spec| match spec {
                MechanismSpec::Classifier { classes, .. } => Encoding::OneHot(*classes),
                _ => Encoding::Numeric,
            })
            .collect();
        let mut fitted: BTreeMap<usize, Mechanism> = BTreeMap::new();
        for &node in graph.order() {
            // Above this threshold DoWhy chooses CatBoost encoding, outside this one-hot model.
            let categories = graph
                .parents(node)
                .unwrap()
                .iter()
                .map(|&parent| match encodings[parent] {
                    Encoding::Numeric => 0,
                    Encoding::OneHot(count) => count.get(),
                })
                .sum::<usize>();
            if categories > 7 {
                return Err(Error::UnsupportedEncoding { node });
            }
            let x = parents(&graph, node, data, &encodings);
            let y: Vec<_> = data.column(node).iter().copied().collect();
            let root = graph.parents(node).unwrap().is_empty();
            let mechanism = match &specs[node] {
                MechanismSpec::Empirical if root => Mechanism::Empirical(y),
                MechanismSpec::Regression(spec) if !root => {
                    let forest = fit_forest_with_rng(
                        &x,
                        &y,
                        spec.trees.get(),
                        spec.min_leaf.get(),
                        None,
                        rng,
                    );
                    let residuals = y
                        .iter()
                        .zip(forest.predict(&x))
                        .map(|(y, predicted)| y - predicted)
                        .collect();
                    Mechanism::Regression { forest, residuals }
                }
                MechanismSpec::Classifier {
                    forest: spec,
                    classes,
                } if !root => {
                    if y.iter().any(|&value| {
                        value < 0.0 || value.fract() != 0.0 || value >= classes.get() as f64
                    }) || (0..classes.get()).any(|class| !y.contains(&(class as f64)))
                    {
                        return Err(Error::Specification);
                    }
                    Mechanism::Classifier(fit_forest_with_rng(
                        &x,
                        &y,
                        spec.trees.get(),
                        spec.min_leaf.get(),
                        Some(classes.get()),
                        rng,
                    ))
                }
                _ => return Err(Error::Specification),
            };
            fitted.insert(node, mechanism);
        }
        Ok(Self {
            graph,
            mechanisms: fitted.into_values().collect(),
            encodings,
        })
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    fn draw(&self, node: usize, data: &DMatrix<f64>, rng: &mut Mt19937) -> Vec<f64> {
        match &self.mechanisms[node] {
            Mechanism::Empirical(values) => (0..data.nrows())
                .map(|_| values[rng.randint(values.len() as u64) as usize])
                .collect(),
            Mechanism::Regression { forest, residuals } => forest
                .predict(&parents(&self.graph, node, data, &self.encodings))
                .into_iter()
                .map(|prediction| {
                    prediction + residuals[rng.randint(residuals.len() as u64) as usize]
                })
                .collect(),
            Mechanism::Classifier(forest) => {
                // The source draws all uniform noises before evaluating class probabilities.
                let noise: Vec<_> = (0..data.nrows()).map(|_| rng.next_f64()).collect();
                forest
                    .predict_proba(&parents(&self.graph, node, data, &self.encodings))
                    .into_iter()
                    .zip(noise)
                    .map(|(probabilities, noise)| {
                        let mut cumulative = 0.0;
                        probabilities
                            .iter()
                            .position(|p| {
                                cumulative += p;
                                cumulative >= noise
                            })
                            .unwrap_or(0) as f64
                    })
                    .collect()
            }
        }
    }

    pub fn interventional_samples(
        &self,
        observed: &DMatrix<f64>,
        interventions: &BTreeMap<usize, Intervention>,
        rng: &mut Mt19937,
    ) -> Result<DMatrix<f64>, Error> {
        self.graph.validate(observed).map_err(Error::Model)?;
        if observed.nrows() == 0 {
            return Err(Error::EmptyData);
        }
        if interventions
            .keys()
            .any(|&node| node >= self.mechanisms.len())
        {
            return Err(Error::InvalidIntervention);
        }
        let mut affected = vec![false; self.mechanisms.len()];
        let mut samples = observed.clone();
        for &node in self.graph.order() {
            affected[node] = interventions.contains_key(&node)
                || self
                    .graph
                    .parents(node)
                    .unwrap()
                    .iter()
                    .any(|&parent| affected[parent]);
            if !affected[node] {
                continue;
            }
            let values = if self.graph.parents(node).unwrap().is_empty() {
                samples.column(node).iter().copied().collect()
            } else {
                self.draw(node, &samples, rng)
            };
            for (row, value) in values.into_iter().enumerate() {
                samples[(row, node)] = match interventions.get(&node) {
                    Some(operation) => operation.apply(value, rng),
                    None => value,
                };
            }
        }
        self.graph.validate(&samples).map_err(Error::Model)?;
        Ok(samples)
    }
}
