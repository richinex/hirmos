//! Numeric model-evaluation metrics used by the preserved GCM notebook.

use crate::numpy_reduce::{numpy_mean,numpy_sum};

#[derive(Debug,PartialEq,Eq)]
pub enum MetricError { Empty, Shape, NonFinite }

fn validate(values:&[f64])->Result<(),MetricError> {
    if values.is_empty() {return Err(MetricError::Empty);}
    if values.iter().any(|v|!v.is_finite()) {return Err(MetricError::NonFinite);}
    Ok(())
}

pub fn variance(values:&[f64])->Result<f64,MetricError> {
    validate(values)?;
    let mean=numpy_mean(values);
    Ok(numpy_mean(&values.iter().map(|v|(v-mean).powi(2)).collect::<Vec<_>>()))
}

pub struct PredictionMetrics { pub mse:f64, pub nmse:f64, pub r2:f64 }

pub fn prediction_metrics(observed:&[f64],predicted:&[f64])->Result<PredictionMetrics,MetricError> {
    validate(observed)?;
    validate(predicted)?;
    if observed.len()!=predicted.len() {return Err(MetricError::Shape);}
    let errors:Vec<_>=observed.iter().zip(predicted).map(|(a,b)|(a-b).powi(2)).collect();
    let mse=numpy_mean(&errors);
    let variance=variance(observed)?;
    let nmse=if variance==0.0 {mse.sqrt()} else {mse.sqrt()/variance.sqrt()};
    let r2=match (mse==0.0,variance==0.0) {(true,_)=>1.0,(false,true)=>0.0,(false,false)=>1.0-mse/variance};
    Ok(PredictionMetrics {mse,nmse,r2})
}

/// Empirical CRPS; the pairwise sample differences use NumPy's reduction order.
pub fn empirical_crps(samples:&[f64],observed:f64)->Result<f64,MetricError> {
    validate(samples)?;
    if !observed.is_finite() {return Err(MetricError::NonFinite);}
    let errors:Vec<_>=samples.iter().map(|v|(v-observed).abs()).collect();
    let pairs:Vec<_>=samples.iter().flat_map(|a|samples.iter().map(move|b|(a-b).abs())).collect();
    Ok(numpy_mean(&errors)-0.5*numpy_mean(&pairs))
}

/// One-dimensional default k=1 KL estimate after removing shared observations.
pub fn marginal_kl(x:&[f64],y:&[f64])->Result<f64,MetricError> {
    validate(x)?;
    validate(y)?;
    let mut y=y.to_vec();
    y.sort_by(f64::total_cmp);
    let mut x=x.to_vec();
    x.sort_by(f64::total_cmp);
    x.dedup_by(|a,b|*a==*b);
    x.retain(|v|y.binary_search_by(|other|other.partial_cmp(v).unwrap()).is_err());
    if x.len()<2 {return Ok(0.0);}
    let mut terms=Vec::with_capacity(x.len());
    for (index,&value) in x.iter().enumerate() {
        let left=if index==0 {f64::INFINITY} else {distance(value,x[index-1],x.len()<=5)};
        let right=x.get(index+1).map(|&v|distance(value,v,x.len()<=5)).unwrap_or(f64::INFINITY);
        let position=y.partition_point(|&v|v<value);
        let below=if position==0 {f64::INFINITY} else {distance(value,y[position-1],y.len()<=3)};
        let above=y.get(position).map(|&v|distance(value,v,y.len()<=3)).unwrap_or(f64::INFINITY);
        terms.push((1.0/x.len() as f64)*(below.min(above)/left.min(right)).ln());
    }
    let result=numpy_sum(&terms)+(y.len() as f64/(x.len()-1) as f64).ln();
    if !result.is_finite() {return Err(MetricError::NonFinite);}
    Ok(result.max(0.0))
}

fn distance(a:f64,b:f64,brute:bool)->f64 {
    if brute {(((-2.0*a)*b+a*a)+b*b).max(0.0).sqrt()} else {(a-b).abs()}
}
