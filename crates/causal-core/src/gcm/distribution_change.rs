//! v0.14 distribution-change attribution with the notebook's mean difference.

use super::{independence::mechanism_change,model::{FittedModel,Graph,ModelError},
    shapley::{estimate_with_distributions,DistributionExecution,Method,ShapleyError}};
use crate::{kci::KciError,nprandom::Mt19937,numpy_reduce::numpy_mean,pcmciplus::bh_values};
use nalgebra::DMatrix;
use std::num::NonZeroUsize;

#[derive(Debug)]
pub enum ChangeError { Model(ModelError), Independence(KciError), Shapley(ShapleyError) }

pub struct MeanChange {
    pub nodes: Vec<usize>,
    pub contributions: Vec<f64>,
}

pub struct MechanismChange {
    pub node:usize,
    pub p_value:f64,
    pub adjusted_p_value:f64,
    pub changed:bool,
}

pub struct DistributionChange {
    pub attribution:MeanChange,
    pub mechanisms:Vec<MechanismChange>,
}

/// The executed notebook uses default KCI, BH at .05, and no invariant-node override.
pub fn distribution_change(graph:&Graph,old:&DMatrix<f64>,new:&DMatrix<f64>,target:usize,
    count:NonZeroUsize,execution:DistributionExecution<'_>,rng:&mut Mt19937,cache:&mut Option<f64>)
    ->Result<DistributionChange,ChangeError> {
    if old.ncols()!=graph.names().len() || new.ncols()!=graph.names().len() {
        return Err(ChangeError::Model(ModelError::Shape));
    }
    let (graph,nodes)=graph.ancestor_graph(target).map_err(ChangeError::Model)?;
    let target=nodes.iter().position(|&n|n==target).ok_or(ChangeError::Model(ModelError::UnknownNode))?;
    let select=|values:&DMatrix<f64>|DMatrix::from_fn(values.nrows(),nodes.len(),|r,c|values[(r,nodes[c])]);
    let old=select(old);
    let new=select(new);
    let mut p_values=Vec::new();
    for node in 0..nodes.len() {
        let parents=graph.parents(node).unwrap();
        let columns=|values:&DMatrix<f64>|DMatrix::from_fn(values.nrows(),parents.len(),|r,c|values[(r,parents[c])]);
        let old_parents=columns(&old);
        let new_parents=columns(&new);
        let conditioning=(!parents.is_empty()).then_some((&old_parents,&new_parents));
        let result=mechanism_change(&old.columns(node,1).into_owned(),&new.columns(node,1).into_owned(),
            conditioning,execution.jobs(),rng,cache).map_err(ChangeError::Independence)?;
        p_values.push(result.p_value);
    }
    let adjusted=bh_values(&p_values);
    let order=crate::numpy_argsort::argsort(&p_values);
    let last=order.iter().enumerate().filter_map(|(rank,&node)|
        (p_values[node]<=((rank+1) as f64/nodes.len() as f64)*0.05).then_some(rank)).last();
    let mut changed=vec![false;nodes.len()];
    if let Some(last)=last {for &node in &order[..=last] {changed[node]=true;}}
    let (before,after)=FittedModel::fit_changed(graph,&old,&new,&changed).map_err(ChangeError::Model)?;
    let mut attribution=mean_change_of_graphs(&before,&after,target,count,execution,rng,cache)?;
    for node in &mut attribution.nodes {*node=nodes[*node];}
    let mechanisms=nodes.iter().enumerate().map(|(i,&node)|MechanismChange {
        node,p_value:p_values[i],adjusted_p_value:adjusted[i],changed:changed[i],
    }).collect();
    Ok(DistributionChange {attribution,mechanisms})
}

/// Compare fitted mechanisms; coalition players follow sorted node labels.
pub fn mean_change_of_graphs(old:&FittedModel,new:&FittedModel,target:usize,count:NonZeroUsize,
    execution:DistributionExecution<'_>,rng:&mut Mt19937,cache:&mut Option<f64>)->Result<MeanChange,ChangeError> {
    if old.graph().names()!=new.graph().names() || old.graph().order()!=new.graph().order()
        || (0..old.graph().names().len()).any(|n|old.graph().parents(n)!=new.graph().parents(n)) {
        return Err(ChangeError::Model(ModelError::Graph));
    }
    let mut nodes=old.graph().ancestors(target).map_err(ChangeError::Model)?;
    nodes.sort_by(|&a,&b|old.graph().names()[a].cmp(&old.graph().names()[b]));
    let (baseline,_)=old.sample_ancestors(target,count.get(),rng,cache).map_err(ChangeError::Model)?;
    let baseline_mean=numpy_mean(baseline.column(target).as_slice());
    let contributions=estimate_with_distributions(nodes.len().try_into().unwrap(),Method::Auto,execution,rng,cache,|subset,rng,cache,distribution_random| {
        if !subset.iter().any(|&v|v) {return Ok(vec![0.0]);}
        let mut selected=vec![false;old.graph().names().len()];
        for (&node,&value) in nodes.iter().zip(subset) {selected[node]=value;}
        let samples=old.sample_mixed_ancestors(new,&selected,target,count.get(),rng,cache,distribution_random)
            .map_err(|_|ShapleyError::NonFinite)?;
        Ok(vec![numpy_mean(samples.column(target).as_slice())-baseline_mean])
    }).map_err(ChangeError::Shapley)?.remove(0);
    Ok(MeanChange {nodes,contributions})
}
