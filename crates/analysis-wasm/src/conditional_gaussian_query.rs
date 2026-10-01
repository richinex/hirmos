use super::*;
use hirmos_causal_core::conditional_gaussian::{Error, Model, Variable};

#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct Assignment { variable:usize, value:f64 }
#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase")]
pub enum Kind { Continuous, Discrete }
#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct Query {
    rows:usize, columns:usize, names:Vec<String>, edges:Vec<(usize,usize)>,
    variables:Vec<Kind>, outcome:usize, observations:Vec<Assignment>, interventions:Vec<Assignment>,
}
pub fn run(values:&[f64],q:Query)->Result<AnalysisResult,String>{
    validate_dense_matrix("conditional Gaussian",values,q.rows,q.columns)?;
    if q.names.len()!=q.columns||q.names.iter().any(|n|n.trim().is_empty())||q.names.iter().collect::<std::collections::BTreeSet<_>>().len()!=q.columns{return Err("Each column needs a distinct name.".into())}
    let kinds=q.variables.iter().map(|v|match v{Kind::Continuous=>Variable::Continuous,Kind::Discrete=>Variable::Discrete}).collect::<Vec<_>>();
    let describe=|error:Error|match error{
        Error::InvalidData=>"Use finite numeric values and assign a type to every variable.".into(),
        Error::InvalidGraph=>"Use an acyclic graph without repeated arrows.".into(),
        Error::InvalidRoles=>"Choose one continuous outcome and distinct variables for observations and interventions.".into(),
        Error::ContinuousParentOfDiscrete{parent,child}=>format!("{} is continuous and points to the discrete variable {}. Conditional-Gaussian models require discrete nodes to have only discrete parents.",q.names[parent],q.names[child]),
        Error::MissingParent{node,parent}=>format!("To read {}, supply an observed or intervention value for its parent {}.",q.names[node],q.names[parent]),
        Error::UnsupportedEvidence{node}=>format!("{} is not a direct parent of this outcome. This local conditional-Gaussian query does not support that evidence or intervention.",q.names[node]),
        Error::InvalidState{node}=>format!("The supplied value is not an observed category of {}.",q.names[node]),
        Error::UnseenConfiguration{node}=>format!("No rows support this combination of discrete parents for {}. No other configuration was substituted.",q.names[node]),
        Error::Numerical=>"The Gaussian fit could not produce finite parameters.".into(),
    };
    let data=nalgebra::DMatrix::from_column_slice(q.rows,q.columns,values);
    let model=Model::fit(&data,&kinds,&q.edges).map_err(&describe)?;
    let observations=q.observations.iter().map(|a|(a.variable,a.value)).collect::<Vec<_>>();
    let interventions=q.interventions.iter().map(|a|(a.variable,a.value)).collect::<Vec<_>>();
    let result=model.query(&[q.outcome],&observations,&interventions).map_err(describe)?;
    let r=&result[0];
    Ok(AnalysisResult::ConditionalGaussianQuery{observations:q.rows,outcome:r.node,mean:r.mean,std:r.std,configuration_rows:r.rows})
}
