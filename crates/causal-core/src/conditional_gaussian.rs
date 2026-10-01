//! Configuration-specific Gaussian CPDs, matching bnlearn 1.0.0 local fits.
//! Queries are local conditionals with every parent supplied. No imputation of
//! parent configurations, ignored evidence, or mean-plug-in network propagation.
use nalgebra::DMatrix;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Variable { Continuous, Discrete }
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidData, InvalidGraph, ContinuousParentOfDiscrete { parent: usize, child: usize },
    InvalidRoles, InvalidState { node: usize }, MissingParent { node: usize, parent: usize },
    UnsupportedEvidence { node: usize }, UnseenConfiguration { node: usize }, Numerical,
}
#[derive(Clone, Debug)]
pub struct Configuration { pub states: Vec<f64>, pub coefficients: Vec<f64>, pub std: f64, pub rows: usize }
#[derive(Clone, Debug)]
pub struct Local { pub node: usize, pub discrete_parents: Vec<usize>, pub continuous_parents: Vec<usize>, pub configurations: Vec<Configuration> }
#[derive(Clone, Debug)]
pub struct Summary { pub node: usize, pub mean: f64, pub std: f64, pub rows: usize }
pub struct Model { pub locals: Vec<Local>, variables: Vec<Variable>, states: Vec<Vec<f64>> }

impl Model {
    pub fn fit(data: &DMatrix<f64>, variables: &[Variable], edges: &[(usize,usize)]) -> Result<Self,Error> {
        let n=data.nrows(); let p=data.ncols();
        if n==0 || p==0 || variables.len()!=p || data.iter().any(|x|!x.is_finite()) {return Err(Error::InvalidData)}
        if edges.iter().any(|&(a,b)|a>=p||b>=p||a==b) || edges.iter().collect::<BTreeSet<_>>().len()!=edges.len() {return Err(Error::InvalidGraph)}
        let mut degree=vec![0;p]; for &(a,b) in edges {
            if variables[a]==Variable::Continuous && variables[b]==Variable::Discrete {return Err(Error::ContinuousParentOfDiscrete{parent:a,child:b})}
            degree[b]+=1;
        }
        let mut queue=(0..p).filter(|&i|degree[i]==0).collect::<Vec<_>>(); let mut count=0;
        while let Some(a)=queue.pop(){count+=1;for &(u,v) in edges{if a==u{degree[v]-=1;if degree[v]==0{queue.push(v)}}}}
        if count!=p{return Err(Error::InvalidGraph)}
        let states=(0..p).map(|j|{let mut s=data.column(j).iter().copied().collect::<Vec<_>>();s.sort_by(f64::total_cmp);s.dedup();s}).collect::<Vec<_>>();
        let mut locals=Vec::new();
        for node in 0..p {
            if variables[node]!=Variable::Continuous{continue}
            // Adjacency-matrix order, as used by the reference's parent map.
            let parents=(0..p).filter(|&a|edges.contains(&(a,node))).collect::<Vec<_>>();
            let discrete_parents=parents.iter().copied().filter(|&a|variables[a]==Variable::Discrete).collect::<Vec<_>>();
            let continuous_parents=parents.iter().copied().filter(|&a|variables[a]==Variable::Continuous).collect::<Vec<_>>();
            let mut groups:BTreeMap<Vec<usize>,Vec<usize>>=BTreeMap::new();
            for row in 0..n {
                let key=discrete_parents.iter().map(|&a|states[a].iter().position(|v|*v==data[(row,a)]).unwrap()).collect();
                groups.entry(key).or_default().push(row);
            }
            let mut configurations=Vec::new();
            for (key,rows) in groups {
                let width=continuous_parents.len()+1;
                let x=DMatrix::from_fn(rows.len(),width,|r,c|if c==0{1.}else{data[(rows[r],continuous_parents[c-1])]});
                let y=DMatrix::from_fn(rows.len(),1,|r,_|data[(rows[r],node)]);
                let coefficients=if width==1 {vec![rows.iter().map(|&r|data[(r,node)]).sum::<f64>()/rows.len() as f64]} else {
                    crate::least_squares::solve(&x,&y,f64::EPSILON*rows.len().max(width) as f64).map_err(|_|Error::Numerical)?.coefficients.column(0).iter().copied().collect()
                };
                let rss=rows.iter().enumerate().map(|(r,_)|{let e=y[(r,0)]-(0..width).map(|c|x[(r,c)]*coefficients[c]).sum::<f64>();e*e}).sum::<f64>();
                let std=(rss/rows.len() as f64).sqrt().max(1e-12);
                if !std.is_finite()||coefficients.iter().any(|x|!x.is_finite()){return Err(Error::Numerical)}
                configurations.push(Configuration{states:key.iter().enumerate().map(|(i,&k)|states[discrete_parents[i]][k]).collect(),coefficients,std,rows:rows.len()});
            }
            locals.push(Local{node,discrete_parents,continuous_parents,configurations});
        }
        Ok(Self{locals,variables:variables.to_vec(),states})
    }

    pub fn query(&self,outcomes:&[usize],observations:&[(usize,f64)],interventions:&[(usize,f64)]) -> Result<Vec<Summary>,Error> {
        let roles=outcomes.iter().copied().chain(observations.iter().chain(interventions).map(|x|x.0)).collect::<Vec<_>>();
        if outcomes.len()!=1||roles.iter().any(|&i|i>=self.variables.len())||roles.iter().collect::<BTreeSet<_>>().len()!=roles.len()||outcomes.iter().any(|&i|self.variables[i]!=Variable::Continuous){return Err(Error::InvalidRoles)}
        let known=observations.iter().chain(interventions).copied().collect::<BTreeMap<_,_>>();
        for (&node,&value) in &known {
            if !value.is_finite(){return Err(Error::InvalidData)}
            if self.variables[node]==Variable::Discrete&&!self.states[node].contains(&value){return Err(Error::InvalidState{node})}
        }
        let mut used=BTreeSet::new();let mut result=Vec::new();
        for &node in outcomes {
            let local=self.locals.iter().find(|l|l.node==node).ok_or(Error::InvalidRoles)?;
            for &parent in local.discrete_parents.iter().chain(&local.continuous_parents){
                if !known.contains_key(&parent){return Err(Error::MissingParent{node,parent})}used.insert(parent);
            }
            let config=local.configurations.iter().find(|c|local.discrete_parents.iter().zip(&c.states).all(|(p,v)|known[p]==*v)).ok_or(Error::UnseenConfiguration{node})?;
            let mean=config.coefficients[0]+local.continuous_parents.iter().enumerate().map(|(i,p)|config.coefficients[i+1]*known[p]).sum::<f64>();
            if !mean.is_finite(){return Err(Error::Numerical)}
            result.push(Summary{node,mean,std:config.std,rows:config.rows});
        }
        for &node in known.keys(){if !used.contains(&node){return Err(Error::UnsupportedEvidence{node})}}
        Ok(result)
    }
}
