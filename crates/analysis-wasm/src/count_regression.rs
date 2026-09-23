use hirmos_causal_core::{panel_design::Panel, panel_glm as glm, interrupted_series as its};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="kind", rename_all="camelCase", deny_unknown_fields)]
pub enum Family { NegativeBinomial { exposure: Option<usize> }, Binomial { trials: usize } }
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Term { pub column: usize, pub lag: usize, pub name: String }
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="kind", rename_all="camelCase", deny_unknown_fields)]
pub enum EventWindow { All, Finite { first:i64, last:i64 } }
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="kind", rename_all="camelCase", deny_unknown_fields)]
pub enum Design {
    Lags { keys: Vec<(u64,i64)>, terms: Vec<Term>, sum: Vec<usize> },
    Events { keys: Vec<(u64,i64)>, adoption: Vec<(u64,Option<i64>)>, cohort:i64, covariates:Vec<Term>, window:EventWindow },
    Summary { keys: Vec<(u64,i64)>, adoption: Vec<(u64,Option<i64>)>, cohort:i64, covariates:Vec<Term> },
    Interrupted { intervention:usize, horizon:usize, bandwidth:usize, covariates:Vec<Term> },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub struct Request { pub rows:usize, pub columns:usize, pub outcome:usize, pub family:Family, pub design:Design, pub confidence:f64, pub iterations:usize, pub tolerance:f64 }
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Estimate { pub name:String, pub estimate:f64, pub standard_error:f64, pub lower:f64, pub upper:f64, pub ratio:f64, pub ratio_lower:f64, pub ratio_upper:f64 }
#[derive(Serialize)]
#[serde(tag="kind", rename_all="camelCase")]
pub enum Status { Converged { iterations:usize }, IterationLimit { iterations:usize }, LineSearchFailed { iterations:usize } }
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Evidence {
    pub request:Request, pub observations:usize, pub input_rows:usize, pub parameters:usize,
    pub retained:Vec<usize>, pub omitted:Vec<usize>, pub terms:Vec<Estimate>, pub contrasts:Vec<Estimate>,
    pub fitted:Vec<f64>, pub observed:Vec<f64>, pub covariance:Vec<Vec<f64>>, pub status:Status,
    pub event_periods:Vec<(usize,i64)>,
}
fn problem(e:glm::Error)->String { use glm::Error::*; match e {
    Shape=>"The model columns have incompatible dimensions.", NonFinite=>"The model contains a non-finite value.",
    Count=>"The outcome must contain non-negative integer counts.", Trials=>"Trials must be positive integers and at least as large as successes.",
    Exposure=>"Exposure must be finite and strictly positive.", Rank=>"The design is rank deficient. A covariate may be constant within teams or shared by all teams within a period.",
    DegreesOfFreedom=>"The model needs more retained observations than fitted parameters.", Clusters=>"Clustered uncertainty needs at least two teams.",
    TimeOrder=>"HAC needs consecutive time periods and a bandwidth below the number of observations.",
    Separation=>"The outcome is perfectly predicted or contains no events; this model cannot estimate finite coefficients.",
    Numerical=>"The numerical fit could not produce finite estimates and an invertible information matrix.",
    Contrast=>"The requested contrast has no estimable uncertainty.", Confidence=>"Confidence must be strictly between zero and one.",
    Iterations=>"Iterations and tolerance must be positive.",
}.into() }
fn design_problem(e:hirmos_causal_core::panel_design::Error)->String {
    use hirmos_causal_core::panel_design::Error::*;
    match e {
        Empty|NoRows=>"No panel observations remain for this specification.",
        Shape=>"Panel keys and model columns must contain the same rows.",
        NonFinite=>"Resolve missing or non-finite panel values before fitting.",
        DuplicateKey=>"Each team must have one observation per period.",
        Gap=>"The panel has missing periods. Resolve the calendar grid before constructing lags.",
        Unbalanced=>"Teams must share the same observed periods for this model.",
        InvalidLag=>"Choose valid predictor columns and lags shorter than the observed panel.",
        InvalidEvent=>"Choose an observed adoption cohort with valid onset records and an event window spanning -1 and 0.",
        UnsupportedEvent=>"The cohort model needs never-adopting controls and observed pre- and post-onset periods.",
    }.into()
}
fn estimate(name:String, fit:&glm::Fit, covariance:&DMatrix<f64>, weights:&[f64], confidence:f64)->Result<Estimate,String> {
    let c=glm::contrast(&fit.coefficients,covariance,weights,confidence).map_err(problem)?;
    if [c.estimate,c.standard_error,c.lower,c.upper,c.ratio,c.ratio_lower,c.ratio_upper].iter().any(|v|!v.is_finite()) {return Err("The requested interval overflows the ratio scale.".into())}
    Ok(Estimate{name,estimate:c.estimate,standard_error:c.standard_error,lower:c.lower,upper:c.upper,ratio:c.ratio,ratio_lower:c.ratio_lower,ratio_upper:c.ratio_upper})
}
pub fn run(values:&[f64], request:Request)->Result<Evidence,String> {
    let r=&request;
    if !r.confidence.is_finite() || r.confidence<=0.0 || r.confidence>=1.0 {return Err(problem(glm::Error::Confidence))}
    if r.iterations==0 || !r.tolerance.is_finite() || r.tolerance<=0.0 {return Err(problem(glm::Error::Iterations))}
    if r.rows.checked_mul(r.columns)!=Some(values.len()) || r.rows==0 || r.columns==0 || r.outcome>=r.columns || values.iter().any(|v|!v.is_finite()) {return Err("Invalid count-regression matrix.".into())}
    let denominator=match r.family {Family::NegativeBinomial{exposure}=>exposure,Family::Binomial{trials}=>Some(trials)};
    if denominator.is_some_and(|c|c>=r.columns||c==r.outcome){return Err("Choose a distinct exposure or trials column.".into())}
    let raw=DMatrix::from_column_slice(r.rows,r.columns,values);
    let validate=|terms:&[Term]|->Result<(),String>{
        let mut seen=BTreeSet::new();
        for term in terms {
            if term.column>=r.columns || term.column==r.outcome || Some(term.column)==denominator || term.name.trim().is_empty() || !seen.insert((term.column,term.lag)) {return Err("Choose distinct, valid predictor columns and lags, separate from outcome and denominator.".into())}
        } Ok(())
    };
    let (matrix,names,retained,omitted,groups,bandwidth,contrast_columns)=match &r.design {
        Design::Lags{keys,terms,sum}=>{
            validate(terms)?;
            if sum.is_empty() || sum.iter().any(|i|*i>=terms.len()) || sum.iter().collect::<BTreeSet<_>>().len()!=sum.len(){return Err("Choose at least one distinct predictor term for the joint contrast.".into())}
            let panel=Panel::new(keys,raw.clone()).map_err(design_problem)?;
            let d=panel.distributed_lags(&terms.iter().map(|t|(t.column,t.lag,t.name.clone())).collect::<Vec<_>>()).map_err(design_problem)?;
            let groups=d.keys.iter().map(|(u,_)|*u).collect();
            (d.matrix,d.terms,d.original_rows,d.omitted_rows,groups,None,sum.iter().map(|i|i+1).collect::<Vec<_>>())
        },
        Design::Events{keys,adoption,cohort,covariates,..}|Design::Summary{keys,adoption,cohort,covariates}=>{
            validate(covariates)?;
            if !matches!(r.family,Family::NegativeBinomial{..}) || covariates.iter().any(|t|t.lag!=0){return Err("Cohort count models require NB2 and contemporaneous covariates.".into())}
            let map:BTreeMap<_,_>=adoption.iter().copied().collect();
            if map.len()!=adoption.len(){return Err("Each team must have one adoption record.".into())}
            let panel=Panel::new(keys,raw.clone()).map_err(design_problem)?;
            let indices=covariates.iter().map(|t|t.column).collect::<Vec<_>>();
            let d=match &r.design {
                Design::Events{window:EventWindow::All,..}=>panel.cohort_events(&map,*cohort,&indices),
                Design::Events{window:EventWindow::Finite{first,last},..}=>panel.cohort_events_in_window(&map,*cohort,&indices,*first,*last),
                _=>panel.cohort_summary(&map,*cohort,&indices)
            }.map_err(design_problem)?;
            let groups=d.keys.iter().map(|(u,_)|*u).collect();
            let contrast=if matches!(r.design,Design::Summary{..}){vec![1]}else{vec![]};
            (d.matrix,d.terms,d.original_rows,d.omitted_rows,groups,None,contrast)
        },
        Design::Interrupted{intervention,horizon,bandwidth,covariates}=>{
            validate(covariates)?;
            if !matches!(r.family,Family::NegativeBinomial{..}) || *intervention<2 || *intervention>=r.rows || *horizon>=r.rows-*intervention || *bandwidth>=r.rows || covariates.iter().any(|t|t.lag!=0) {return Err("NB2 interrupted series needs pre/post observations, an in-window contrast horizon and a valid HAC bandwidth.".into())}
            let d=its::design(r.rows,&its::InterruptedSeriesDesign{intervention_row:*intervention,lag:0,impact:its::ImpactModel::LevelAndSlope,harmonics:None});
            let mut names=d.names;names.extend(covariates.iter().map(|t|t.name.clone()));
            let matrix=DMatrix::from_fn(r.rows,names.len(),|i,j|if j<d.x.ncols(){d.x[(i,j)]}else{raw[(i,covariates[j-d.x.ncols()].column)]});
            (matrix,names,(0..r.rows).collect(),vec![],vec![],Some(*bandwidth),vec![2,3])
        }
    };
    let y:Vec<_>=retained.iter().map(|i|raw[(*i,r.outcome)]).collect();
    let exposure:Vec<_>=retained.iter().map(|i|denominator.map_or(1.0,|c|raw[(*i,c)])).collect();
    let family=match r.family {Family::NegativeBinomial{..}=>glm::Family::NegativeBinomial{counts:&y,exposure:&exposure},Family::Binomial{..}=>glm::Family::Binomial{successes:&y,trials:&exposure}};
    let fit=glm::fit(&matrix,family,r.iterations,r.tolerance).map_err(problem)?;
    let times:Vec<_>=(0..retained.len() as i64).collect();
    let uncertainty=match bandwidth {Some(lags)=>glm::Uncertainty::Hac{times:&times,lags,correction:true},None=>glm::Uncertainty::Cluster{ids:&groups,correction:true}};
    let covariance=glm::covariance(&fit,uncertainty).map_err(problem)?;
    let mut terms=Vec::new();
    for (j,name) in names.into_iter().enumerate(){let mut w=vec![0.;fit.coefficients.len()];w[j]=1.;terms.push(estimate(name,&fit,&covariance,&w,r.confidence)?);}
    let mut contrasts=Vec::new();
    if !contrast_columns.is_empty(){
        let mut w=vec![0.;fit.coefficients.len()];for j in contrast_columns{w[j]=1.;}
        let name=match r.design {Design::Interrupted{horizon,..}=>{w[3]=(horizon+1) as f64;format!("Intervention contrast at post-event period {}",horizon+1)},Design::Summary{..}=>"Treated by post".into(),_=>"Joint lag sum".into()};
        contrasts.push(estimate(name,&fit,&covariance,&w,r.confidence)?);
    }
    let status=if fit.converged{Status::Converged{iterations:fit.iterations}}else if fit.line_search_failed{Status::LineSearchFailed{iterations:fit.iterations}}else{Status::IterationLimit{iterations:fit.iterations}};
    let event_periods=match &r.design {
        Design::Events{keys,cohort,adoption,window,..}=>{
            let teams:BTreeSet<_>=adoption.iter().filter(|(_,t)|*t==Some(*cohort)).map(|(u,_)|*u).collect();
            let periods:BTreeSet<_>=keys.iter().filter(|(u,_)|teams.contains(u)).map(|(_,t)|t-cohort).filter(|t|*t != -1).filter(|t|match window{EventWindow::All=>true,EventWindow::Finite{first,last}=>t>=first&&t<=last}).collect();
            periods.into_iter().enumerate().map(|(j,t)|(j+1,t)).collect()
        },_=>vec![]
    };
    Ok(Evidence{input_rows:r.rows,observations:retained.len(),parameters:fit.coefficients.len(),request,retained,omitted,terms,contrasts,fitted:fit.fitted,observed:y,covariance:(0..covariance.nrows()).map(|i|(0..covariance.ncols()).map(|j|covariance[(i,j)]).collect()).collect(),status,event_periods})
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixtures()->serde_json::Value {serde_json::from_str(include_str!("../../causal-core/oracle/fixtures/panel_glm.json")).unwrap()}
    fn close(a:f64,b:f64){assert!(a.is_finite()&&b.is_finite());assert!((a-b).abs()/(1.+b.abs())<2e-5,"{a} versus {b}");}
    #[test]
    fn panel_facade_matches_source_covariance_and_contrasts(){
        for case in fixtures()["cases"].as_array().unwrap(){
            let name=case["name"].as_str().unwrap();
            if !["nb2_distributed_lags","nb2_cohort_events","nb2_cohort_summary","binomial_team_week"].contains(&name){continue}
            let y:Vec<f64>=serde_json::from_value(case["y"].clone()).unwrap();
            let exposure:Vec<f64>=serde_json::from_value(case["exposure"].clone()).unwrap();
            let (keys,raw,design)=if name=="binomial_team_week" {
                let keys:Vec<_>=(0..12).flat_map(|u|(0..18).map(move|t|(u,t))).collect();
                let raw:Vec<Vec<f64>>=case["x"].as_array().unwrap().iter().map(|row|vec![row[1].as_f64().unwrap(),row[2].as_f64().unwrap()]).collect();
                let terms=vec![Term{column:2,lag:0,name:"x1".into()},Term{column:3,lag:0,name:"x2".into()}];
                (keys.clone(),raw,Design::Lags{keys,terms,sum:vec![0,1]})
            }else{
                let spec=&case["design"];
                let keys:Vec<(u64,i64)>=serde_json::from_value(spec["keys"].clone()).unwrap();
                let raw:Vec<Vec<f64>>=serde_json::from_value(spec["values"].clone()).unwrap();
                let design=if name=="nb2_distributed_lags" {
                    let terms=spec["columns"].as_array().unwrap().iter().map(|t|Term{column:t[0].as_u64().unwrap() as usize+2,lag:t[1].as_u64().unwrap() as usize,name:t[2].as_str().unwrap().into()}).collect();
                    Design::Lags{keys:keys.clone(),terms,sum:vec![0,1,2,3]}
                }else {
                    let adoption:BTreeMap<u64,Option<i64>>=serde_json::from_value(spec["adoption"].clone()).unwrap();
                    if name=="nb2_cohort_events"{Design::Events{keys:keys.clone(),adoption:adoption.into_iter().collect(),cohort:7,covariates:vec![],window:EventWindow::All}}else{Design::Summary{keys:keys.clone(),adoption:adoption.into_iter().collect(),cohort:7,covariates:vec![]}}
                };
                (keys,raw,design)
            };
            let rows=keys.len();let columns=raw[0].len()+2;
            let mut values=vec![0.;rows*columns];let mut retained=0;
            for (i,(u,t)) in keys.iter().enumerate(){
                let keep=match name {"nb2_distributed_lags"=>*t>=4,"nb2_cohort_events"|"nb2_cohort_summary"=>*u<8,_=>true};
                values[rows+i]=1.;
                if keep{values[i]=y[retained];values[rows+i]=exposure[retained];retained+=1;}
                for(j,v)in raw[i].iter().enumerate(){values[(j+2)*rows+i]=*v;}
            }
            let family=if name=="binomial_team_week"{Family::Binomial{trials:1}}else{Family::NegativeBinomial{exposure:Some(1)}};
            let result=run(&values,Request{rows,columns,outcome:0,family,design,confidence:0.95,iterations:1000,tolerance:case["tolerance"].as_f64().unwrap()}).unwrap();
            for(i,term)in result.terms.iter().enumerate(){close(term.estimate,case["params"][i].as_f64().unwrap());close(term.standard_error,case["covariances"]["cluster"][i][i].as_f64().unwrap().sqrt());}
            for c in result.contrasts{close(c.estimate,case["contrasts"]["cluster"]["estimate"].as_f64().unwrap());close(c.standard_error,case["contrasts"]["cluster"]["se"].as_f64().unwrap());}
            assert_eq!(result.observations,y.len());
        }
    }
    #[test]
    fn interrupted_facade_preserves_the_existing_slope_convention(){
        let f=fixtures();let case=f["cases"].as_array().unwrap().iter().find(|c|c["name"]=="nb2_its_standard_tolerance").unwrap();
        let y:Vec<f64>=serde_json::from_value(case["y"].clone()).unwrap();let exposure:Vec<f64>=serde_json::from_value(case["exposure"].clone()).unwrap();
        let mut values=y.clone();values.extend(&exposure);values.extend(case["x"].as_array().unwrap().iter().map(|row|row[4].as_f64().unwrap()));
        let request=Request{rows:y.len(),columns:3,outcome:0,family:Family::NegativeBinomial{exposure:Some(1)},design:Design::Interrupted{intervention:20,horizon:7,bandwidth:3,covariates:vec![Term{column:2,lag:0,name:"holiday".into()}]},confidence:0.95,iterations:1000,tolerance:1e-5};
        let actual=run(&values,request).unwrap();
        let expected:Vec<f64>=serde_json::from_value(case["fitted"].clone()).unwrap();
        for(a,b)in actual.fitted.iter().zip(expected){assert!((a-b).abs()/(1.+b.abs())<1e-4);}
        // Shift of the design origin changes intercept/step coefficients, not fitted means.
        let slope=case["params"][3].as_f64().unwrap();
        assert!((actual.terms[2].estimate-(case["params"][2].as_f64().unwrap()-slope)).abs()<1e-4);
    }
}
