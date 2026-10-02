//! R ARDL 0.2.5's stored asymptotic bounds and I(1) p-value interpolation.
//! This route is separate from statsmodels' PSS calibration.
#[path="bounds_tables.rs"]
mod tables;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Statistic { F(f64), T(f64) }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Finding { BelowRejectionRegion, Inconclusive, RejectNoLevelRelationship }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Error { InvalidStatistic, UnsupportedCase, UnsupportedPredictorCount, UnsupportedAlpha, TNotApplicable }
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Calibration {pub alpha:f64,pub i0:f64,pub i1:f64,pub p_value:f64,pub finding:Finding}
pub const LEVELS:[f64;8]=[0.005,0.01,0.025,0.05,0.075,0.1,0.15,0.2];

pub fn calibrate(statistic:Statistic,k:usize,case:usize,alpha:f64)->Result<Calibration,Error> {
    if !(1..=5).contains(&case){return Err(Error::UnsupportedCase);}
    if !(1..=10).contains(&k){return Err(Error::UnsupportedPredictorCount);}
    let table=tables::TABLES.iter().find(|t|t.case==case&&t.k==k).ok_or(Error::UnsupportedPredictorCount)?;
    let index=table.alpha.iter().position(|a|(*a-alpha).abs()<1e-12).ok_or(Error::UnsupportedAlpha)?;
    let (value,i0,i1,curve,increasing)=match statistic {
        Statistic::F(value) if value.is_finite()&&value>=0.0=>(value,table.f_i0[index],table.f_i1[index],table.p_f,false),
        Statistic::T(value) if value.is_finite()=>{
            if ![1,3,5].contains(&case){return Err(Error::TNotApplicable);}
            (value,table.t_i0[index],table.t_i1[index],table.p_t,true)
        }
        _=>return Err(Error::InvalidStatistic),
    };
    // bounds_test.R uses the I(1) distribution, not the I(0) distribution,
    // and its recorded endpoints 0.000001 / 0.999999 outside the table.
    let minimum=curve.iter().copied().fold(f64::INFINITY,f64::min);
    let maximum=curve.iter().copied().fold(f64::NEG_INFINITY,f64::max);
    let p_value=if value<minimum {if increasing{0.000001}else{0.999999}}
    else if value>maximum {if increasing{0.999999}else{0.000001}}
    else if let Some(i)=curve.iter().position(|v|*v==value){table.p_alpha[i]}
    else {
        let lower=curve.iter().enumerate().filter(|(_,v)|**v<value).max_by(|a,b|a.1.total_cmp(b.1)).unwrap().0;
        let upper=curve.iter().enumerate().filter(|(_,v)|**v>value).min_by(|a,b|a.1.total_cmp(b.1)).unwrap().0;
        table.p_alpha[lower]+(table.p_alpha[upper]-table.p_alpha[lower])*(value-curve[lower])/(curve[upper]-curve[lower])
    };
    let finding=if increasing {
        if value<i1{Finding::RejectNoLevelRelationship}else if value>i0{Finding::BelowRejectionRegion}else{Finding::Inconclusive}
    }else if value>i1{Finding::RejectNoLevelRelationship}else if value<i0{Finding::BelowRejectionRegion}else{Finding::Inconclusive};
    Ok(Calibration{alpha,i0,i1,p_value,finding})
}
