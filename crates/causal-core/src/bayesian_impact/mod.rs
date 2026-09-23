//! Google's nonseasonal local-level CausalImpact model, with spike-and-slab controls.
//! TFP 0.25 conditional kernels; NumPy-compatible random streams, not TF bit streams.
//! Source attribution and oracle: oracle/time-related-events/BAYESIAN.md.
pub mod level;
pub mod regression;
pub mod summary;

use crate::lapack_cholesky::{dpotrf, dpotrs, Triangle};
use crate::nprandom::{Mt19937, NpRng};
use nalgebra::{DMatrix, DVector};
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Error { Shape, NonFinite, Window, ConstantOutcome, ConstantControl, Settings, Singular, Numerical }

pub struct Plan {
    y: Vec<f64>, x: DMatrix<f64>, pre: usize, post: Range<usize>,
    mean: f64, sd: f64, draws: usize, warmup: usize, seed: u32, prior_level_sd: f64,
}
pub struct Draw { pub level: Vec<f64>, pub weights: Vec<f64>, pub observation_variance: f64, pub level_variance: f64 }
pub struct Fit {
    pub means: Vec<f64>,
    /// Time-major predictive draws on the original outcome scale.
    pub paths: Vec<Vec<f64>>,
    pub latent: Vec<Draw>,
    pub inclusion_probabilities: Vec<f64>,
    pub summaries: summary::Summaries,
    pub post_path: Vec<summary::PostPoint>,
}

impl Plan {
    pub fn new(y: &[f64], x: &DMatrix<f64>, pre: usize, post: Range<usize>, draws: usize, warmup: usize, seed: u32, prior_level_sd: f64) -> Result<Self, Error> {
        if y.len() != x.nrows() { return Err(Error::Shape); }
        if pre < 2 || pre > y.len() || post.start < pre || post.start >= post.end || post.end > y.len() { return Err(Error::Window); }
        if y.iter().chain(x.iter()).any(|v| !v.is_finite()) { return Err(Error::NonFinite); }
        if draws < 2 || draws.checked_add(warmup).is_none() || !prior_level_sd.is_finite() || prior_level_sd <= 0.0 { return Err(Error::Settings); }
        let scale = |values: &[f64]| {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let sd = (values.iter().map(|v| (v-mean).powi(2)).sum::<f64>() / (values.len()-1) as f64).sqrt();
            (mean, sd)
        };
        let (mean, sd) = scale(&y[..pre]);
        if !sd.is_finite() || sd <= 0.0 { return Err(Error::ConstantOutcome); }
        let standardized_y: Vec<f64> = y.iter().map(|v| (v-mean)/sd).collect();
        let mut standardized_x = x.clone();
        for j in 0..x.ncols() {
            let values: Vec<f64> = (0..pre).map(|t| x[(t,j)]).collect();
            let (m, s) = scale(&values);
            if !s.is_finite() || s <= 0.0 { return Err(Error::ConstantControl); }
            for t in 0..x.nrows() { standardized_x[(t,j)] = (x[(t,j)]-m)/s; }
        }
        if standardized_y.iter().chain(standardized_x.iter()).any(|v| !v.is_finite()) { return Err(Error::NonFinite); }
        // CausalImpactData appends an unstandardized intercept after scaling controls.
        // It participates in spike-and-slab selection and the dimension-dependent prior.
        let standardized_x = if x.ncols() == 0 { standardized_x } else {
            DMatrix::from_fn(x.nrows(), x.ncols()+1, |i,j| {
                if j == x.ncols() { 1.0 } else { standardized_x[(i,j)] }
            })
        };
        Ok(Self { y: standardized_y, x: standardized_x, pre, post, mean, sd, draws, warmup, seed, prior_level_sd })
    }
}

struct Conditional { indices: Vec<usize>, factor: Vec<f64>, mean: Vec<f64>, shape: f64, scale: f64, log_prob: f64 }
struct Controls { x: DMatrix<f64>, prior: DMatrix<f64>, precision: DMatrix<f64>, probability: f64 }
impl Controls {
    fn new(x: &DMatrix<f64>, pre: usize) -> Result<Self, Error> {
        let gram = x.transpose()*x;
        let p=x.ncols();
        let prior=DMatrix::from_fn(p,p,|i,j| 0.01*gram[(i,j)]*(if i==j {1.0} else {0.5})/x.nrows() as f64);
        let observed=x.rows(0,pre).into_owned();
        let precision=observed.transpose()*&observed+&prior;
        let mut factor=prior.as_slice().to_vec();
        if dpotrf(Triangle::Lower,p,&mut factor,p).map_err(|_| Error::Shape)? != 0 {return Err(Error::Singular);}
        Ok(Self { x:observed, prior, precision, probability:(3.0/p as f64).min(1.0) })
    }
    fn conditional(&self, target:&[f64], included:&[bool]) -> Result<Conditional,Error> {
        let indices:Vec<usize>=included.iter().enumerate().filter_map(|(i,b)| b.then_some(i)).collect();
        let k=indices.len();
        let xty=self.x.transpose()*DVector::from_column_slice(target);
        let shape=25.0+target.len() as f64/2.0;
        let mut factor=DMatrix::from_fn(k,k,|i,j| self.precision[(indices[i],indices[j])]).as_slice().to_vec();
        let mut prior=DMatrix::from_fn(k,k,|i,j| self.prior[(indices[i],indices[j])]).as_slice().to_vec();
        let mut mean:Vec<f64>=indices.iter().map(|&i| xty[i]).collect();
        if k>0 {
            for a in [&mut factor,&mut prior] { if dpotrf(Triangle::Lower,k,a,k).map_err(|_|Error::Shape)?!=0 {return Err(Error::Singular);} }
            dpotrs(Triangle::Lower,k,1,&factor,k,&mut mean,k).map_err(|_|Error::Shape)?;
        }
        let scale=5.0+(target.iter().map(|v|v*v).sum::<f64>()-mean.iter().zip(&indices).map(|(m,&i)|m*xty[i]).sum::<f64>())/2.0;
        if !scale.is_finite() || scale<=0.0 {return Err(Error::Numerical);}
        let log_prior=if self.probability==1.0 {if k==included.len(){0.0}else{f64::NEG_INFINITY}} else {
            k as f64*self.probability.ln()+(included.len()-k) as f64*(1.0-self.probability).ln()
        };
        let log_prob=(0..k).map(|i|prior[i+i*k].ln()-factor[i+i*k].ln()).sum::<f64>()+log_prior-(shape-1.0)*(2.0*scale).ln();
        Ok(Conditional { indices,factor,mean,shape,scale,log_prob })
    }
    fn sample(&self,target:&[f64],included:&mut [bool],rng:&mut NpRng,gamma:&mut Mt19937) -> Result<(f64,Vec<f64>),Error> {
        let mut state=self.conditional(target,included)?;
        if self.probability<1.0 {
            // Sorting independent uniforms matches the reference random scan distribution.
            let mut order:Vec<(f64,usize)>=(0..included.len()).map(|i|(rng.next_f64(),i)).collect();
            order.sort_by(|a,b|a.0.total_cmp(&b.0));
            for (_,i) in order {
                included[i]=!included[i];
                let proposed=self.conditional(target,included)?;
                let delta=proposed.log_prob-state.log_prob;
                let p=if delta>=0.0 {1.0/(1.0+(-delta).exp())} else {delta.exp()/(1.0+delta.exp())};
                if rng.next_f64()<p {state=proposed;} else {included[i]=!included[i];}
            }
        }
        let variance=crate::ucm::invgamma_rvs(state.shape,state.scale,gamma).min(1.44);
        let k=state.indices.len();
        let z:Vec<f64>=(0..k).map(|_|rng.standard_normal()).collect();
        // P^-1 L z = L^-T z; use the existing LAPACK solve rather than another triangular solver.
        let mut noise:Vec<f64>=(0..k).map(|i|(0..=i).map(|j|state.factor[i+j*k]*z[j]).sum()).collect();
        if k>0 {dpotrs(Triangle::Lower,k,1,&state.factor,k,&mut noise,k).map_err(|_|Error::Shape)?;}
        let mut weights=vec![0.0;included.len()];
        for (j,&i) in state.indices.iter().enumerate() {weights[i]=state.mean[j]+variance.sqrt()*noise[j];}
        Ok((variance,weights))
    }
}

pub fn fit(plan:&Plan) -> Result<Fit,Error> {
    let n=plan.y.len(); let p=plan.x.ncols();
    let controls=if p==0 {None} else {Some(Controls::new(&plan.x,plan.pre)?)};
    let mut rng=NpRng::seeded(plan.seed as u64);
    let mut gamma=Mt19937::seeded(plan.seed.wrapping_add(1));
    let mut predictive_rng=NpRng::seeded((plan.seed as u64)^0x9e3779b97f4a7c15);
    let mut included=vec![p<=3;p];
    let mut inclusions=vec![0usize;p];
    let mut level=vec![0.0;n]; let mut weights=vec![0.0;p];
    let mut observation_variance=if p>0 {0.2} else {1.0};
    let mut level_variance=plan.prior_level_sd.powi(2);
    let mut latent=Vec::with_capacity(plan.draws);
    let mut means=vec![0.0;n]; let mut paths=vec![Vec::with_capacity(plan.draws);n];
    for iteration in 0..plan.warmup+plan.draws {
        if let Some(controls)=&controls {
            let target:Vec<f64>=(0..plan.pre).map(|t|plan.y[t]-level[t]).collect();
            (observation_variance,weights)=controls.sample(&target,&mut included,&mut rng,&mut gamma)?;
        }
        let residuals:Vec<Option<f64>>=(0..n).map(|t| if t<plan.pre {Some(plan.y[t]-(0..p).map(|j|plan.x[(t,j)]*weights[j]).sum::<f64>())}else{None}).collect();
        let model=level::Model::new(observation_variance,level_variance,plan.y[0],1.0).map_err(|_|Error::Numerical)?;
        level=model.sample(&residuals,&mut rng).map_err(|_|Error::Numerical)?;
        let ss=level.windows(2).map(|w|(w[1]-w[0]).powi(2)).sum::<f64>();
        // TFP's compiled _resample_scale reconstructs its distribution argument
        // without the ad-hoc upper_bound attribute. Match that executed kernel,
        // not its eager-only cap (see bayesian_caps.py).
        level_variance=crate::ucm::invgamma_rvs(16.0+(n-1) as f64/2.0,16.0*plan.prior_level_sd.powi(2)+ss/2.0,&mut gamma);
        if p==0 {
            let ss=(0..plan.pre).map(|t|(plan.y[t]-level[t]).powi(2)).sum::<f64>();
            observation_variance=crate::ucm::invgamma_rvs(0.005+plan.pre as f64/2.0,0.005+ss/2.0,&mut gamma);
        }
        if iteration>=plan.warmup {
            for (count, selected) in inclusions.iter_mut().zip(&included) { *count += usize::from(*selected); }
            for t in 0..n {
                let mean=level[t]+(0..p).map(|j|plan.x[(t,j)]*weights[j]).sum::<f64>();
                means[t]+=(mean*plan.sd+plan.mean)/plan.draws as f64;
                paths[t].push((mean+observation_variance.sqrt()*predictive_rng.standard_normal())*plan.sd+plan.mean);
            }
            latent.push(Draw {level:level.clone(),weights:weights.clone(),observation_variance,level_variance});
        }
    }
    let observed:Vec<f64>=plan.y.iter().map(|v|v*plan.sd+plan.mean).collect();
    let predictions=summary::Predictions::new(&observed,&means,&paths,plan.post.clone(),0.05).map_err(|_|Error::Numerical)?;
    let summaries=summary::summarize(&predictions).map_err(|_|Error::Numerical)?;
    let post_path=summary::post_path(&predictions).map_err(|_|Error::Numerical)?;
    let inclusion_probabilities=inclusions.into_iter().map(|n|n as f64/plan.draws as f64).collect();
    Ok(Fit {means,paths,latent,summaries,post_path,inclusion_probabilities})
}
