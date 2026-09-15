//! Source default geometric-median summary of bootstrap replicate vectors.

use crate::{bfgs::{fmin_bfgs_complete, BfgsResult}, numpy_reduce::numpy_sum};
use nalgebra::DMatrix;
use std::cell::{Cell, RefCell};

#[derive(Debug, PartialEq, Eq)]
pub enum SummaryError {
    Empty,
    NonFinite,
}

pub struct GeometricMedian {
    pub optimizer: BfgsResult,
    pub objective_evaluations: usize,
}

pub fn geometric_median(samples: &DMatrix<f64>) -> Result<GeometricMedian, SummaryError> {
    if samples.nrows() == 0 || samples.ncols() == 0 {
        return Err(SummaryError::Empty);
    }
    if samples.iter().any(|v| !v.is_finite()) {
        return Err(SummaryError::NonFinite);
    }
    let initial: Vec<_>=(0..samples.ncols()).map(|c| samples.column(c).iter().sum::<f64>()/samples.nrows() as f64).collect();
    let evaluations=Cell::new(0);
    let objective=|point: &[f64]| {
        evaluations.set(evaluations.get()+1);
        let distances: Vec<_>=(0..samples.nrows()).map(|r| {
            let squares: Vec<_>=(0..samples.ncols()).map(|c| (point[c]-samples[(r,c)]).powi(2)).collect();
            numpy_sum(&squares).sqrt()
        }).collect();
        numpy_sum(&distances)
    };
    let cache=RefCell::new(None::<(Vec<f64>,f64)>);
    let function=|point: &[f64]| {
        if let Some((x,value))=cache.borrow().as_ref() {
            if x==point {return *value;}
        }
        let value=objective(point);
        *cache.borrow_mut()=Some((point.to_vec(),value));
        value
    };
    let gradient=|point: &[f64]| {
        let value=function(point);
        (0..point.len()).map(|c| {
            let mut shifted=point.to_vec();
            shifted[c]+=f64::EPSILON.sqrt();
            let mut step=shifted[c]-point[c];
            if step==0.0 {
                shifted[c]=point[c]+f64::EPSILON.sqrt()*point[c].signum()*point[c].abs().max(1.0);
                step=shifted[c]-point[c];
            }
            (objective(&shifted)-value)/step
        }).collect()
    };
    let optimizer=fmin_bfgs_complete(function,gradient,&initial,1e-5,200*samples.ncols());
    Ok(GeometricMedian {optimizer,objective_evaluations:evaluations.get()})
}
