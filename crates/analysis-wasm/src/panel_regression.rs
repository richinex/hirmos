//! Browser boundary for pooled event regressions and hierarchical CR2 regressions.
//! These are regression specifications, not cohort ATT aggregations.
use hirmos_causal_core::{design, estimation::{self, cr2, inference}, panel_design};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Deserialize, Serialize)]
#[serde(tag="kind", rename_all="camelCase", deny_unknown_fields)]
pub(crate) enum Coding { Numeric {}, Categorical { reference: String } }
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub(crate) struct Predictor { column: usize, name: String, coding: Coding }
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub(crate) struct Window { first: i64, last: i64, reference: i64, tails: Tails }
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) enum Tails { Reference, Bin }
#[derive(Deserialize, Serialize)]
#[serde(tag="kind", rename_all="camelCase", rename_all_fields="camelCase", deny_unknown_fields)]
pub(crate) enum Specification {
    EventStudy { keys: Vec<(u64,i64)>, adoption: Vec<(u64,Option<i64>)>, covariates: Vec<usize>, window: Window },
    Interactions { predictors: Vec<Predictor>, terms: Vec<Vec<usize>> },
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    rows: usize, columns: usize, outcome: usize, names: Vec<String>,
    weights: Option<usize>, cluster: usize, confidence: f64, specification: Specification,
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Coefficient {
    index: usize, name: String, estimate: f64, standard_error: f64,
    degrees_of_freedom: f64, p_value: f64, lower: f64, upper: f64,
}
#[derive(Serialize)]
#[serde(tag="kind", rename_all="camelCase", rename_all_fields="camelCase")]
pub(crate) enum LeadTest {
    Unavailable { reason: String },
    Recorded { statistic: f64, numerator_df: usize, denominator_df: f64, p_value: f64 },
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Event { index: usize, period: i64, support: usize }
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Evidence {
    version: u8, request: Request, observations: usize, clusters: usize,
    terms: Vec<Coefficient>, omitted: Vec<String>, covariance: Vec<Vec<f64>>,
    events: Vec<Event>, lead_test: LeadTest,
}

fn coefficient(index: usize, name: String, beta: f64, variance: f64, df: f64, confidence: f64) -> Result<Coefficient,String> {
    let c=inference::contrast(&[beta], &DMatrix::from_element(1,1,variance), &[1.], 0., confidence,
        inference::Reference::Student { degrees_of_freedom:df })
        .map_err(|e|format!("Coefficient uncertainty could not be calculated: {e:?}."))?;
    Ok(Coefficient {index,name,estimate:c.estimate,standard_error:c.standard_error,
        degrees_of_freedom:df,p_value:c.p_value,lower:c.interval[0],upper:c.interval[1]})
}
pub(crate) fn run(values:&[f64], r:Request)->Result<Evidence,String> {
    if r.rows==0 || r.columns==0 || r.rows.checked_mul(r.columns)!=Some(values.len()) ||
        r.names.len()!=r.columns || r.names.iter().any(|n|n.trim().is_empty()) ||
        r.names.iter().collect::<BTreeSet<_>>().len()!=r.columns ||
        values.iter().any(|v|!v.is_finite()) || r.outcome>=r.columns || r.cluster>=r.columns ||
        r.weights.is_some_and(|w|w>=r.columns || w==r.outcome || w==r.cluster) || r.outcome==r.cluster || !(0.<r.confidence && r.confidence<1.) {
        return Err("Choose finite model columns, matching dimensions and a confidence level between zero and one.".into());
    }
    let matrix=DMatrix::from_column_slice(r.rows,r.columns,values);
    let y:Vec<_>=matrix.column(r.outcome).iter().copied().collect();
    let weights:Vec<_>=(0..r.rows).map(|i|r.weights.map_or(1.,|w|matrix[(i,w)])).collect();
    let mut labels:Vec<_>=matrix.column(r.cluster).iter().copied().collect();
    labels.sort_by(f64::total_cmp); labels.dedup();
    let clusters:Vec<_>=(0..r.rows).map(|i|labels.binary_search_by(|v|v.total_cmp(&matrix[(i,r.cluster)])).unwrap() as u64).collect();
    let reserved=|column:usize| column==r.outcome || column==r.cluster || r.weights==Some(column);
    let (terms,omitted,covariance,observations,cluster_count,events,lead_test)=match &r.specification {
        Specification::EventStudy{keys,adoption,covariates,window}=>{
            if keys.len()!=r.rows || covariates.iter().any(|c|*c>=r.columns || reserved(*c)) {
                return Err("Event-study keys and covariates must match the model rows and have distinct roles.".into());
            }
            let adoption_map:BTreeMap<_,_>=adoption.iter().copied().collect();
            if adoption_map.len()!=adoption.len() {return Err("Each unit must have one adoption record.".into());}
            let panel=panel_design::Panel::new(keys,matrix).map_err(|e|format!("Event-study panel could not be constructed: {e:?}."))?;
            let design=panel.pooled_events(&adoption_map,covariates,panel_design::EventWindow{
                first:window.first,last:window.last,reference:window.reference,tails:match window.tails {Tails::Reference=>panel_design::EventTails::Reference,Tails::Bin=>panel_design::EventTails::Bin},
            }).map_err(|e|format!("Event window is invalid or lacks observed support: {e:?}."))?;
            let ordered=|v:&[f64]|design.original_rows.iter().map(|i|v[*i]).collect::<Vec<_>>();
            let units:Vec<_>=design.keys.iter().map(|k|k.0).collect();
            let first=design.keys.iter().map(|k|k.1).min().unwrap();
            let times:Vec<_>=design.keys.iter().map(|k|(k.1-first) as u64).collect();
            let groups:Vec<_>=design.original_rows.iter().map(|i|clusters[*i]).collect();
            let fit=estimation::weighted_within_convention(&ordered(&y),&design.matrix,&units,Some(&times),&ordered(&weights),
                estimation::WithinErrors::ClusterBy(&groups),estimation::WithinConvention::Lfe).map_err(|e|e.to_string())?;
            let names:Vec<_>=covariates.iter().map(|c|r.names[*c].clone()).chain(design.event_periods.iter().map(|e|format!("Event {e}"))).collect();
            let terms=fit.kept.iter().enumerate().map(|(j,i)|coefficient(*i,names[*i].clone(),fit.params[j],fit.covariance[(j,j)],fit.inference_df as f64,r.confidence)).collect::<Result<Vec<_>,_>>()?;
            let events:Vec<_>=design.event_periods.iter().zip(&design.support).enumerate().map(|(j,(period,support))|Event{index:covariates.len()+j,period:*period,support:*support}).collect();
            let leads:Vec<_>=events.iter().filter(|e|e.period<0).collect();
            let lead_test=if leads.is_empty() { LeadTest::Unavailable{reason:"No pre-adoption coefficients were requested.".into()} }
                else if leads.iter().any(|e|!fit.kept.contains(&e.index)) {LeadTest::Unavailable{reason:"At least one requested lead was absorbed by the fixed effects.".into()}}
                else {
                    let restrictions=DMatrix::from_fn(leads.len(),fit.params.len(),|i,j|f64::from(leads[i].index==fit.kept[j]));
                    match inference::joint(&fit.params,&fit.covariance,&restrictions,&vec![0.;leads.len()],inference::Reference::Student{degrees_of_freedom:fit.inference_df as f64}) {
                        Ok(inference::JointTest::F{statistic,numerator_df,denominator_df,p_value})=>LeadTest::Recorded{statistic,numerator_df,denominator_df,p_value},
                        Ok(inference::JointTest::ChiSquared{..})=>return Err("The joint test returned an unexpected reference distribution.".into()),
                        Err(e)=>LeadTest::Unavailable{reason:format!("The joint restriction could not be evaluated: {e:?}.")},
                    }
                };
            (terms,fit.dropped.iter().map(|i|names[*i].clone()).collect(),fit.covariance,r.rows,labels.len(),events,lead_test)
        }
        Specification::Interactions{predictors,terms}=>{
            if predictors.is_empty() || predictors.iter().any(|p|p.column>=r.columns || p.name!=r.names[p.column] || p.column==r.outcome || r.weights==Some(p.column)) ||
                predictors.iter().map(|p|p.column).collect::<BTreeSet<_>>().len()!=predictors.len() || terms.is_empty() {
                return Err("Choose distinct predictor columns and at least one model term.".into());
            }
            let columns:Vec<_>=predictors.iter().map(|p|design::Column{name:p.name.clone(),values:design::Values::Numeric(matrix.column(p.column).iter().copied().collect()),encoding:match p.coding {Coding::Numeric {}=>design::Encoding::Numeric,Coding::Categorical{..}=>design::Encoding::TreatmentContrast}}).collect();
            let references:BTreeMap<_,_>=predictors.iter().enumerate().filter_map(|(i,p)|match &p.coding{Coding::Numeric {}=>None,Coding::Categorical{reference}=>Some((i,reference.clone()))}).collect();
            let terms=terms.iter().map(|t|match t.as_slice(){[a]=>Ok(design::Term::Main(*a)),[a,b]=>Ok(design::Term::Pair(*a,*b)),[a,b,c]=>Ok(design::Term::Triple(*a,*b,*c)),_=>Err("Each term needs one, two or three distinct predictors.")}).collect::<Result<Vec<_>,_>>()?;
            let design=design::hierarchical_design(&columns,&terms,&references).map_err(|e|format!("The interaction design could not be constructed: {e:?}."))?;
            let x=DMatrix::from_fn(r.rows,design.names.len(),|i,j|design.rows[i][j]);
            let selected=cr2::fit_selecting_aliases(&y,&x,&weights,&clusters).map_err(|e|format!("Clustered regression could not be fitted: {e:?}."))?;
            let fit=selected.fit;let kept=selected.selection.kept();
            let terms=kept.iter().enumerate().map(|(j,i)|coefficient(*i,design.names[*i].clone(),fit.params[j],fit.covariance[(j,j)],fit.degrees_of_freedom[j],r.confidence)).collect::<Result<Vec<_>,_>>()?;
            let omitted=design.names.into_iter().enumerate().filter_map(|(i,n)|(!kept.contains(&i)).then_some(n)).collect();
            (terms,omitted,fit.covariance,fit.observations,fit.clusters,vec![],LeadTest::Unavailable{reason:"This specification has no event-time lead test.".into()})
        }
    };
    let covariance=(0..covariance.nrows()).map(|i|(0..covariance.ncols()).map(|j|covariance[(i,j)]).collect()).collect();
    Ok(Evidence{version:1,request:r,observations,clusters:cluster_count,terms,omitted,covariance,events,lead_test})
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value,json};
    fn near(a:f64,b:f64){assert!((a-b).abs()<=1e-8*b.abs().max(1.),"{a} != {b}");}
    fn numbers(v:&Value)->Vec<f64>{serde_json::from_value(v.clone()).unwrap()}
    #[test]
    fn interaction_boundary_matches_numeric_estimatr_fixtures(){
        let root:Value=serde_json::from_str(include_str!("../../causal-core/oracle/fixtures/mixtape_estimatr.json")).unwrap();
        let mut tested=0;
        for (name,c) in root["cases"].as_object().unwrap(){
            let input=c["input"].as_array().unwrap();
            // The core separately verifies text factors; prepared matrices carry numeric columns.
            if input.iter().any(|p|p["numeric"]!=true){continue}
            let y=numbers(&c["y"]);let n=y.len();let mut values=y;
            let mut names=vec!["Outcome".to_string()];let mut predictors=vec![];
            for p in input {let column=names.len();let label=p["name"].as_str().unwrap().to_owned();names.push(label.clone());values.extend(numbers(&p["values"]));predictors.push(Predictor{column,name:label,coding:if p["categorical"]==true{Coding::Categorical{reference:p["reference"].as_str().unwrap().into()}}else{Coding::Numeric {}}});}
            let weight=names.len();names.push("Observation weights".into());values.extend(numbers(&c["weights"]));
            let cluster=names.len();names.push("Cluster".into());values.extend(numbers(&c["clusters"]));
            let request=Request{rows:n,columns:names.len(),names,outcome:0,weights:Some(weight),cluster,confidence:0.95,specification:Specification::Interactions{predictors,terms:serde_json::from_value(c["term_inputs"].clone()).unwrap()}};
            let result=run(&values,request).unwrap_or_else(|e|panic!("{name}: {e}"));
            assert_eq!(result.terms.len(),c["keep"].as_array().unwrap().len(),"{name}");
            for (j,t) in result.terms.iter().enumerate(){
                assert_eq!(t.index,c["keep"][j].as_u64().unwrap() as usize);
                near(t.estimate,c["params"][j].as_f64().unwrap());near(t.standard_error,c["se"][j].as_f64().unwrap());
                near(t.degrees_of_freedom,c["df"][j].as_f64().unwrap());near(t.p_value,c["pvalues"][j].as_f64().unwrap());
                near(t.lower,c["intervals"][j][0].as_f64().unwrap());near(t.upper,c["intervals"][j][1].as_f64().unwrap());
                for k in 0..result.terms.len(){near(result.covariance[j][k],c["covariance"][j][k].as_f64().unwrap());}
            } tested+=1;
        }assert_eq!(tested,9);
    }
    #[test]
    fn event_boundary_matches_lfe_with_source_rows_reversed(){
        let root:Value=serde_json::from_str(include_str!("../../causal-core/oracle/fixtures/mixtape_lfe.json")).unwrap();
        for name in ["castle_events","synthetic_events_reference","synthetic_events_bin"] {
            let c=&root[name];let y=numbers(&c["y"]);let n=y.len();let labels:Vec<String>=serde_json::from_value(c["terms"].clone()).unwrap();
            let covariates:Vec<String>=serde_json::from_value(c["covariates"].clone()).unwrap();
            let x:Vec<Vec<f64>>=serde_json::from_value(c["x"].clone()).unwrap();
            let mut values:Vec<_>=y.into_iter().rev().collect();let mut names=vec!["Outcome".to_string()];
            for cov in &covariates {let j=labels.iter().position(|v|v==cov).unwrap();values.extend(x.iter().rev().map(|r|r[j]));names.push(cov.clone());}
            let weight=names.len();names.push("Weights".into());values.extend(numbers(&c["weights"]).into_iter().rev());
            let cluster=names.len();names.push("Cluster".into());values.extend(numbers(&c["cluster_ids"]).into_iter().rev());
            let units:Vec<u64>=serde_json::from_value(c["units"].clone()).unwrap();let times:Vec<i64>=serde_json::from_value(c["times"].clone()).unwrap();
            let keys=units.into_iter().zip(times).rev().collect();let adoption=c["adoption"].as_array().unwrap().iter().map(|a|(a["unit"].as_u64().unwrap(),a["time"].as_i64())).collect();
            let request=Request{rows:n,columns:names.len(),names,outcome:0,weights:Some(weight),cluster,confidence:0.95,specification:Specification::EventStudy{keys,adoption,covariates:(1..=covariates.len()).collect(),window:serde_json::from_value(c["window"].clone()).unwrap()}};
            let result=run(&values,request).unwrap_or_else(|e|panic!("{name}: {e}"));
            let expected:Vec<_>=result.terms.iter().map(|t|{let label=if let Some(event)=result.events.iter().find(|e|e.index==t.index){if event.period<0{format!("lead{}",-event.period)}else{format!("lag{}",event.period)}}else{t.name.clone()};labels.iter().position(|v|*v==label).unwrap()}).collect();
            assert_eq!(expected.len(),labels.len());
            for (j,t) in result.terms.iter().enumerate(){let r=expected[j];near(t.estimate,c["params"][r].as_f64().unwrap());near(t.standard_error,c["se"][r].as_f64().unwrap());near(t.p_value,c["pvalues"][r].as_f64().unwrap());near(t.lower,c["interval"][r][0].as_f64().unwrap());near(t.upper,c["interval"][r][1].as_f64().unwrap());for k in 0..expected.len(){near(result.covariance[j][k],c["covariance"][r][expected[k]].as_f64().unwrap());}}
        }
    }
    #[test]
    fn boundary_rejects_unknown_options_and_out_of_bounds_roles(){
        let r=json!({"rows":2,"columns":2,"names":["y","x"],"outcome":3,"weights":null,"cluster":1,"confidence":0.95,"specification":{"kind":"interactions","predictors":[{"column":1,"name":"x","coding":{"kind":"numeric"}}],"terms":[[0]]}});
        assert!(run(&[1.,2.,3.,4.],serde_json::from_value(r.clone()).unwrap()).is_err());
        let mut bad=r;bad["fallback"]=json!(true);assert!(serde_json::from_value::<Request>(bad).is_err());
    }
}
