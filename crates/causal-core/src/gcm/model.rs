//! Numeric half-normal-root/linear-empirical ANMs used by the v0.14 notebook.

use crate::{
    backdoor::Dag,
    nprandom::Mt19937,
    numpy_reduce::numpy_mean,
    sklearn_linear::{fit_sklearn_fortran, predict_fortran, SKLEARN_LINEAR_TOLERANCE},
};
use nalgebra::{DMatrix, DVector};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use super::shapley::DistributionRandom;

#[derive(Debug, PartialEq, Eq)]
pub enum ModelError {
    Graph,
    Shape,
    NonFinite,
    Regression,
    UnknownNode,
}

/// Graph order is supplied separately from column labels; parent columns are label-sorted.
#[derive(Clone)]
pub struct Graph {
    names: Vec<String>,
    dag: Dag,
    order: Vec<usize>,
}

impl Graph {
    pub fn new(names: Vec<String>, edges: &[(usize, usize)]) -> Result<Self, ModelError> {
        if names.is_empty()
            || names.iter().collect::<BTreeSet<_>>().len() != names.len()
            || edges
                .iter()
                .any(|&(a, b)| a >= names.len() || b >= names.len())
        {
            return Err(ModelError::Graph);
        }
        let mut seen = BTreeSet::new();
        let edges: Vec<_> = edges.iter().copied().filter(|e| seen.insert(*e)).collect();
        let mut dag = Dag::new(names.len(), &edges);
        for parents in &mut dag.parents {
            parents.sort_by(|&a, &b| names[a].cmp(&names[b]));
        }
        let mut degrees: Vec<_> = dag.parents.iter().map(Vec::len).collect();
        let mut queue: VecDeque<_> = (0..names.len()).filter(|&n| degrees[n] == 0).collect();
        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &child in &dag.children[node] {
                degrees[child] -= 1;
                if degrees[child] == 0 {
                    queue.push_back(child);
                }
            }
        }
        if order.len() != names.len() {
            return Err(ModelError::Graph);
        }
        Ok(Self { names, dag, order })
    }
    pub fn names(&self) -> &[String] {
        &self.names
    }
    pub fn order(&self) -> &[usize] {
        &self.order
    }
    pub fn parents(&self, node: usize) -> Option<&[usize]> {
        self.dag.parents.get(node).map(Vec::as_slice)
    }
    /// Target and ancestors in graph insertion order, before numerical fitting.
    pub fn ancestors(&self, target:usize)->Result<Vec<usize>,ModelError> {
        if target>=self.names.len() {return Err(ModelError::UnknownNode);}
        let included=self.dag.ancestors_of(&[target]);
        Ok((0..self.names.len()).filter(|n|included.contains(n)).collect())
    }

    pub fn ancestor_graph(&self,target:usize)->Result<(Self,Vec<usize>),ModelError> {
        let nodes=self.ancestors(target)?;
        let mut mapping=vec![None;self.names.len()];
        for (i,&node) in nodes.iter().enumerate() {mapping[node]=Some(i);}
        let mut edges=Vec::new();
        for (i,&node) in nodes.iter().enumerate() {
            for &child in &self.dag.children[node] {
                if let Some(j)=mapping[child] {edges.push((i,j));}
            }
        }
        Ok((Self::new(nodes.iter().map(|&n|self.names[n].clone()).collect(),&edges)?,nodes))
    }
    pub(crate) fn validate(&self, data: &DMatrix<f64>) -> Result<(), ModelError> {
        if data.ncols() != self.names.len() {
            return Err(ModelError::Shape);
        }
        if data.iter().any(|v| !v.is_finite()) {
            return Err(ModelError::NonFinite);
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) enum Mechanism {
    HalfNormal {
        location: f64,
        scale: f64,
    },
    Linear {
        coefficients: DVector<f64>,
        intercept: f64,
        residuals: Vec<f64>,
    },
}

impl Mechanism {
    pub(crate) fn draw_noise(&self,count:usize,rng:&mut Mt19937,cache:&mut Option<f64>)->DVector<f64> {
        match self {
            Self::HalfNormal {location,scale}=>DVector::from_iterator(count,(0..count).map(|_| {
                if *scale==0.0 {*location} else {rng.standard_normal(cache).abs()*scale+location}
            })),
            Self::Linear {residuals,..}=>DVector::from_iterator(count,(0..count).map(|_|residuals[rng.randint(residuals.len() as u64) as usize])),
        }
    }

    pub(crate) fn predict(&self,parents:&DMatrix<f64>)->DVector<f64> {
        match self {
            Self::HalfNormal {..}=>DVector::zeros(parents.nrows()),
            Self::Linear {coefficients,intercept,..}=>predict_fortran(parents,coefficients,*intercept),
        }
    }
}

/// All mechanisms are fitted and aligned to the privately owned validated graph.
#[derive(Clone)]
pub struct FittedModel {
    graph: Graph,
    mechanisms: Vec<Mechanism>,
}

impl FittedModel {
    fn draw_noise(
        &self,
        node: usize,
        count: usize,
        rng: &mut Mt19937,
        cache: &mut Option<f64>,
    ) -> DVector<f64> {
        self.mechanisms[node].draw_noise(count,rng,cache)
    }

    /// Shift interventions with fresh conditional draws, not fixed-noise counterfactuals.
    pub fn interventional_samples(
        &self,
        observed: &DMatrix<f64>,
        shifts: &BTreeMap<usize, f64>,
        rng: &mut Mt19937,
    ) -> Result<DMatrix<f64>, ModelError> {
        self.graph.validate(observed)?;
        if shifts.keys().any(|&n| n >= self.mechanisms.len()) {
            return Err(ModelError::UnknownNode);
        }
        if shifts.values().any(|v| !v.is_finite()) {
            return Err(ModelError::NonFinite);
        }
        let mut affected: BTreeSet<_> = shifts.keys().copied().collect();
        for &node in shifts.keys() {
            affected.extend(self.graph.dag.descendants(node));
        }
        let mut samples = observed.clone();
        for &node in &self.graph.order {
            if !affected.contains(&node) {
                continue;
            }
            let mut values = match self.mechanisms[node] {
                Mechanism::HalfNormal { .. } => samples.column(node).into_owned(),
                Mechanism::Linear { .. } => {
                    self.prediction(node, &samples)
                        + self.draw_noise(node, samples.nrows(), rng, &mut None)
                }
            };
            if let Some(shift) = shifts.get(&node) {
                values.add_scalar_mut(*shift);
            }
            samples.set_column(node, &values);
        }
        Ok(samples)
    }
    pub fn fit(graph: Graph, data: &DMatrix<f64>) -> Result<Self, ModelError> {
        graph.validate(data)?;
        if data.nrows() == 0 {
            return Err(ModelError::Shape);
        }
        let mut mechanisms = Vec::with_capacity(graph.names.len());
        for node in 0..graph.names.len() {
            mechanisms.push(Self::fit_mechanism(&graph,node,data)?);
        }
        Ok(Self { graph, mechanisms })
    }

    pub(crate) fn fit_mechanism(graph:&Graph,node:usize,data:&DMatrix<f64>)->Result<Mechanism,ModelError> {
            let target = data.column(node).into_owned();
            let parents = &graph.dag.parents[node];
            let mechanism = if parents.is_empty() {
                let location = target.iter().copied().fold(f64::INFINITY, f64::min);
                let squared: Vec<_> = target
                    .iter()
                    .map(|&v| (v - location) * (v - location))
                    .collect();
                let scale = numpy_mean(&squared).sqrt();
                if !scale.is_finite() {
                    return Err(ModelError::NonFinite);
                }
                Mechanism::HalfNormal { location, scale }
            } else {
                let x = DMatrix::from_fn(data.nrows(), parents.len(), |r, c| data[(r, parents[c])]);
                let fit = fit_sklearn_fortran(&x, &target, SKLEARN_LINEAR_TOLERANCE)
                    .map_err(|_| ModelError::Regression)?;
                Mechanism::Linear {
                    coefficients: fit.coefficients,
                    intercept: fit.intercept,
                    residuals: fit.residuals.as_slice().to_vec(),
                }
            };
        Ok(mechanism)
    }

    /// Unchanged mechanisms share a fit to pooled rows; changed ones are fitted separately.
    pub(crate) fn fit_changed(graph:Graph,old:&DMatrix<f64>,new:&DMatrix<f64>,changed:&[bool])
        ->Result<(Self,Self),ModelError> {
        graph.validate(old)?;
        graph.validate(new)?;
        if old.nrows()==0 || new.nrows()==0 || changed.len()!=graph.names.len() {return Err(ModelError::Shape);}
        let joint=DMatrix::from_fn(old.nrows()+new.nrows(),old.ncols(),|r,c| {
            if r<old.nrows() {old[(r,c)]} else {new[(r-old.nrows(),c)]}
        });
        let mut before=Vec::new();
        let mut after=Vec::new();
        for (node,&changed) in changed.iter().enumerate() {
            if changed {
                before.push(Self::fit_mechanism(&graph,node,old)?);
                after.push(Self::fit_mechanism(&graph,node,new)?);
            } else {
                let pooled=Self::fit_mechanism(&graph,node,&joint)?;
                before.push(pooled.clone());
                after.push(pooled);
            }
        }
        Ok((Self {graph:graph.clone(),mechanisms:before},Self {graph,mechanisms:after}))
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    fn prediction(&self, node: usize, data: &DMatrix<f64>) -> DVector<f64> {
        match &self.mechanisms[node] {
            Mechanism::HalfNormal { .. } => DVector::zeros(data.nrows()),
            Mechanism::Linear {
                coefficients,
                intercept,
                ..
            } => {
                let parents = &self.graph.dag.parents[node];
                let x = DMatrix::from_fn(data.nrows(), parents.len(), |r, c| data[(r, parents[c])]);
                predict_fortran(&x, coefficients, *intercept)
            }
        }
    }

    /// Recover noise while retaining the graph's explicit column order.
    pub fn noise_from_data(&self, data: &DMatrix<f64>) -> Result<DMatrix<f64>, ModelError> {
        self.graph.validate(data)?;
        let mut noise = data.clone();
        for &node in &self.graph.order {
            if let Mechanism::Linear { .. } = self.mechanisms[node] {
                noise.set_column(node, &(data.column(node) - self.prediction(node, data)));
            }
        }
        Ok(noise)
    }

    pub fn data_from_noise(&self, noise: &DMatrix<f64>) -> Result<DMatrix<f64>, ModelError> {
        self.graph.validate(noise)?;
        let mut data = noise.clone();
        for &node in &self.graph.order {
            if let Mechanism::Linear { .. } = self.mechanisms[node] {
                data.set_column(node, &(self.prediction(node, &data) + noise.column(node)));
            }
        }
        Ok(data)
    }

    /// Draw ancestor noise in topological order. The caller owns NumPy's normal cache.
    pub fn sample_ancestors(
        &self,
        target: usize,
        count: usize,
        rng: &mut Mt19937,
        normal_cache: &mut Option<f64>,
    ) -> Result<(DMatrix<f64>, DMatrix<f64>), ModelError> {
        self.sample_using(target,count,rng,normal_cache,DistributionRandom::Shared,|_|self)
    }

    pub(crate) fn sample_mixed_ancestors<'a>(&'a self,other:&'a Self,selected:&[bool],target:usize,
        count:usize,rng:&mut Mt19937,cache:&mut Option<f64>,distribution_random:DistributionRandom<'_>)->Result<DMatrix<f64>,ModelError> {
        if self.graph.names!=other.graph.names || self.graph.dag.parents!=other.graph.dag.parents
            || self.graph.order!=other.graph.order || selected.len()!=self.mechanisms.len() {
            return Err(ModelError::Graph);
        }
        self.sample_using(target,count,rng,cache,distribution_random,|n|if selected[n] {other} else {self}).map(|(data,_)|data)
    }

    fn sample_using<'a>(&'a self,target:usize,count:usize,rng:&mut Mt19937,normal_cache:&mut Option<f64>,
        distribution_random:DistributionRandom<'_>,
        choose:impl Fn(usize)->&'a Self)->Result<(DMatrix<f64>,DMatrix<f64>),ModelError> {
        if target >= self.mechanisms.len() {
            return Err(ModelError::UnknownNode);
        }
        let ancestors = self.graph.dag.ancestors_of(&[target]);
        self.sample_nodes(&ancestors,count,rng,normal_cache,distribution_random,choose)
    }

    pub fn draw_samples(&self,count:usize,rng:&mut Mt19937,cache:&mut Option<f64>)->Result<DMatrix<f64>,ModelError> {
        self.sample_nodes(&(0..self.graph.names.len()).collect(),count,rng,cache,DistributionRandom::Shared,|_|self)
            .map(|(data,_)|data)
    }

    fn sample_nodes<'a>(&'a self,ancestors:&BTreeSet<usize>,count:usize,rng:&mut Mt19937,normal_cache:&mut Option<f64>,
        mut distribution_random:DistributionRandom<'_>,choose:impl Fn(usize)->&'a Self)
        ->Result<(DMatrix<f64>,DMatrix<f64>),ModelError> {
        let mut noise = DMatrix::zeros(count, self.mechanisms.len());
        let mut data = noise.clone();
        for &node in &self.graph.order {
            if !ancestors.contains(&node) {
                continue;
            }
            let model=choose(node);
            let drawn=match (&model.mechanisms[node],&mut distribution_random) {
                (Mechanism::HalfNormal {..},DistributionRandom::Isolated {rng,cache})=>model.draw_noise(node,count,rng,cache),
                _=>model.draw_noise(node,count,rng,normal_cache),
            };
            noise.set_column(node, &drawn);
            match &model.mechanisms[node] {
                Mechanism::HalfNormal { .. } => {
                    data.set_column(node, &noise.column(node));
                }
                Mechanism::Linear { .. } => {
                    data.set_column(node, &(model.prediction(node, &data) + noise.column(node)));
                }
            }
        }
        Ok((data, noise))
    }

    pub fn ancestor_order(&self, target: usize) -> Result<Vec<usize>, ModelError> {
        if target >= self.mechanisms.len() {
            return Err(ModelError::UnknownNode);
        }
        let ancestors = self.graph.dag.ancestors_of(&[target]);
        Ok(self
            .graph
            .order
            .iter()
            .copied()
            .filter(|node| ancestors.contains(node))
            .collect())
    }
}
