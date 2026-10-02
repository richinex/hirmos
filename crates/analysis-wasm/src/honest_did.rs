//! Validated browser boundary over the shared HonestDiD port.
use hirmos_causal_core::honest_did as kernel;
use nalgebra::DMatrix;
use serde::{Deserialize,Serialize};
use crate::protocol::AnalysisResult;
#[derive(Deserialize,Serialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub(crate) struct EventStudy {pre:usize,post:usize,events:Vec<i32>,coefficients:Vec<f64>,covariance:Vec<Vec<f64>>,contrast:Vec<f64>}
#[derive(Deserialize,Serialize)]
#[serde(tag="kind",rename_all="camelCase",deny_unknown_fields)]
enum Grid {ReferenceDefault{points:usize},Explicit{lower:f64,upper:f64,points:usize}}
#[derive(Deserialize,Serialize)]
#[serde(tag="kind",rename_all="camelCase",deny_unknown_fields)]
enum Method {Conditional,LeastFavorableHybrid{kappa:f64}}
#[derive(Deserialize,Serialize)]
#[serde(rename_all="camelCase")]
enum Moments {PostTreatment,AllPeriods}
#[derive(Deserialize,Serialize)]
#[serde(tag="kind",rename_all="camelCase",deny_unknown_fields)]
enum Restriction {Smoothness{points:usize},RelativeMagnitude{method:Method,moments:Moments,grid:Grid}}
#[derive(Deserialize,Serialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct Configuration {confidence:f64,bounds:Vec<f64>,seed:u32,restriction:Restriction}
#[derive(Deserialize,Serialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub(crate) struct Request {event_study:EventStudy,configuration:Configuration}
#[derive(Serialize)]
#[serde(tag="kind",rename_all="camelCase",rename_all_fields="camelCase")]
enum Interval {FixedLength{lower:f64,upper:f64,coefficients:Vec<f64>,half_length:f64,search:&'static str,accuracy:&'static str},ConfidenceGrid{grid:Vec<f64>,accepted:Vec<bool>,open_lower:bool,open_upper:bool}}
#[derive(Serialize)]
struct Row {bound:f64,interval:Interval}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Evidence {kind:&'static str,version:u32,request:Request,estimate:f64,standard_error:f64,conventional:(f64,f64),results:Vec<Row>}
pub(crate) fn fit(request:Request)->Result<AnalysisResult,String>{
    let s=&request.event_study;let c=&request.configuration;
    let n=s.pre.checked_add(s.post).ok_or("The event-study dimensions are invalid.")?;
    if n>100||s.coefficients.len()!=n||s.events.len()!=n||s.covariance.len()!=n||s.covariance.iter().any(|r|r.len()!=n)||s.events.iter().enumerate().any(|(i,t)|*t!=if i<s.pre{i as i32-s.pre as i32-1}else{i as i32-s.pre as i32}){return Err("Use consecutive event periods with period -1 omitted and matching coefficient and covariance dimensions.".into())}
    if !c.confidence.is_finite()||c.confidence<=0.0||c.confidence>=1.0||c.bounds.is_empty()||c.bounds.len()>25||c.bounds.iter().enumerate().any(|(i,b)|!b.is_finite()||*b<0.0||i>0&&*b<=c.bounds[i-1]){return Err("Choose increasing nonnegative sensitivity bounds and a confidence level between zero and one.".into())}
    let points=match &c.restriction{Restriction::Smoothness{points}=>*points,Restriction::RelativeMagnitude{grid,..}=>match grid{Grid::ReferenceDefault{points}|Grid::Explicit{points,..}=>*points}};
    if !(2..=10001).contains(&points){return Err("Choose between 2 and 10,001 numerical search points.".into())}
    if matches!(c.restriction,Restriction::RelativeMagnitude{..})&&s.post==1&&s.contrast!=vec![1.0]{return Err("The single-post-period conditional route requires a unit target weight.".into())}
    let periods=kernel::Periods::new(s.pre,s.post).map_err(|e|e.to_string())?;
    let covariance=DMatrix::from_fn(n,n,|i,j|s.covariance[i][j]);
    let study=kernel::EventStudy::new(periods,s.coefficients.clone(),covariance.clone(),s.contrast.clone()).map_err(|e|e.to_string())?;
    let estimate=s.coefficients[s.pre..].iter().zip(&s.contrast).map(|(b,l)|b*l).sum::<f64>();
    let variance=(0..s.post).flat_map(|i|(0..s.post).map(move|j|(i,j))).map(|(i,j)|s.contrast[i]*s.contrast[j]*covariance[(s.pre+i,s.pre+j)]).sum::<f64>();
    if !variance.is_finite()||variance<=0.0{return Err("The target contrast has no positive finite sampling variance.".into())}
    let standard_error=variance.sqrt();let alpha=1.0-c.confidence;
    let critical=spec_math::cephes64::ndtri(1.0-alpha/2.0);
    let mut results=Vec::new();
    for bound in &c.bounds{
        let interval=match &c.restriction{
            Restriction::Smoothness{points}=>{
                let r=kernel::fixed_length_interval(study.affine_problem(),&s.coefficients,*bound,alpha,*points,c.seed).map_err(|e|e.to_string())?;
                Interval::FixedLength{lower:r.lower,upper:r.upper,coefficients:r.coefficients,half_length:r.half_length,
                    search:match r.search{kernel::SearchMethod::DerivativeBisection=>"derivativeBisection",kernel::SearchMethod::Grid=>"grid"},
                    accuracy:match r.accuracy{kernel::optimization::Accuracy::Solved=>"solved",kernel::optimization::Accuracy::ResidualQualified=>"residualQualified",kernel::optimization::Accuracy::ReferenceInaccurate=>"referenceInaccurate"}}
            },
            Restriction::RelativeMagnitude{method,moments,grid}=>{
                let specification=kernel::RelativeMagnitudeSpecification::new(*bound,match method{Method::Conditional=>kernel::ConditionalMethod::Conditional,Method::LeastFavorableHybrid{kappa}=>kernel::ConditionalMethod::LeastFavorableHybrid{kappa:*kappa}},match moments{Moments::PostTreatment=>kernel::MomentSelection::PostTreatment,Moments::AllPeriods=>kernel::MomentSelection::AllPeriods},match grid{Grid::ReferenceDefault{points}=>kernel::GridSpecification::ReferenceDefault{points:*points},Grid::Explicit{lower,upper,points}=>kernel::GridSpecification::Explicit{lower:*lower,upper:*upper,points:*points}},c.seed,alpha).map_err(|e|e.to_string())?;
                let r=kernel::relative_magnitude_confidence_set(&study,specification).map_err(|e|e.to_string())?;
                let(open_lower,open_upper)=r.open_endpoints();
                Interval::ConfidenceGrid{grid:r.grid().to_vec(),accepted:r.accepted().to_vec(),open_lower,open_upper}
            }
        };
        results.push(Row{bound:*bound,interval});
    }
    Ok(AnalysisResult::HonestDid{evidence:Evidence{kind:"honestDid",version:1,request,estimate,standard_error,conventional:(estimate-critical*standard_error,estimate+critical*standard_error),results}})
}
