//! v0.14 `kernel_based` sampling wrapper for finite numeric/encoded inputs.

use super::shapley::Execution;
use crate::{kci::{kernel_conditional_independence_with_rng, KciError, KciResult}, nprandom::Mt19937};
use nalgebra::DMatrix;
use std::num::NonZeroUsize;

/// Default non-bootstrap KCI: random subset first, then a seeded kernel job.
pub fn kernel_test(x:&DMatrix<f64>,y:&DMatrix<f64>,z:Option<&DMatrix<f64>>,
    maximum:NonZeroUsize,execution:Execution,rng:&mut Mt19937,cache:&mut Option<f64>)->Result<KciResult,KciError> {
    if x.nrows()==0 {return Err(KciError::EmptySample);}
    if x.nrows()!=y.nrows() || z.is_some_and(|z|z.nrows()!=x.nrows()) {return Err(KciError::RowMismatch);}
    let count=x.nrows().min(maximum.get());
    let order=rng.permutation(x.nrows());
    let select=|values:&DMatrix<f64>|DMatrix::from_fn(count,values.ncols(),|r,c|values[(order[r],c)]);
    let xs=select(x);
    let ys=select(y);
    let zs=z.map(select);
    let seed=rng.randint(i32::MAX as u64) as u32;
    match execution {
        Execution::Serial=>{
            *rng=Mt19937::seeded(seed);
            *cache=None;
            kernel_conditional_independence_with_rng(&xs,&ys,zs.as_ref(),rng)
        }
        Execution::Isolated=>kernel_conditional_independence_with_rng(&xs,&ys,zs.as_ref(),&mut Mt19937::seeded(seed)),
    }
}

/// Two-sample mechanism test; string dataset labels become two indicator columns.
pub fn mechanism_change(old:&DMatrix<f64>,new:&DMatrix<f64>,parents:Option<(&DMatrix<f64>,&DMatrix<f64>)>,
    execution:Execution,rng:&mut Mt19937,cache:&mut Option<f64>)->Result<KciResult,KciError> {
    if old.nrows()==0 || new.nrows()==0 {return Err(KciError::EmptySample);}
    if old.ncols()!=new.ncols() || parents.is_some_and(|(a,b)|a.nrows()!=old.nrows() || b.nrows()!=new.nrows() || a.ncols()!=b.ncols()) {
        return Err(KciError::RowMismatch);
    }
    let count=old.nrows().min(new.nrows());
    let old_order=rng.permutation(old.nrows());
    let new_order=rng.permutation(new.nrows());
    let combine=|a:&DMatrix<f64>,b:&DMatrix<f64>|DMatrix::from_fn(2*count,a.ncols(),|r,c| {
        if r<count {a[(old_order[r],c)]} else {b[(new_order[r-count],c)]}
    });
    let target=combine(old,new);
    let labels=DMatrix::from_fn(2*count,2,|r,c| f64::from((r<count && c==1)||(r>=count && c==0)));
    let conditioning=parents.map(|(a,b)|combine(a,b));
    kernel_test(&target,&labels,conditioning.as_ref(),2000.try_into().unwrap(),execution,rng,cache)
}
