//! Tigramite DataFrame's multiple-dataset/vector-variable projection.
//! Reuses the role-aware single-frame projector; datasets are pooled AFTER lagging.
use crate::ci_samples::{construct_array_tracked, ConstructOptions};
use crate::missing_data::{MaskType, PreprocessingError, TigramiteFrame};
use crate::parcorr::CutOff;
use crate::parcorr::{CleanedXyz, Node};
use crate::parcorr_mult::{AnalyticResult, Correlation, Role, SampleError, Samples};
use std::{cell::RefCell, collections::BTreeMap};

mod shuffle_ci;
pub use shuffle_ci::ShuffleCi;

#[derive(Debug)]
pub enum JointError {
    EmptyDatasets,
    Shape,
    EmptyVariable(usize),
    InvalidComponent(Node),
    InvalidNode(Node),
    LagOverflow,
    MissingZeroOffset,
    NoReferencePoints,
    InconsistentMasks,
    InconsistentTypes,
    MaskRequired,
    ResidualRecyclingSignature,
    EmptyBootstrap(Vec<(usize, crate::bootstrap::BlockSummary)>),
    Frame(PreprocessingError),
    Samples(SampleError),
}

/// Immutable data/mappings. Dataset positions retain Python dictionary order.
pub struct JointData {
    datasets: Vec<Dataset>,
    vectors: Vec<Vec<Node>>,
    reference_points: Vec<usize>,
    discarded_reference_points: Vec<i64>,
    mask_type: MaskType,
    propagate_missing: bool,
    bootstrap: Option<crate::bootstrap::Bootstrap>,
}

struct Dataset {
    frame: TigramiteFrame,
    offset: usize,
}

/// Transport input: the offset and cell metadata travel with their dataset.
pub struct DatasetInput {
    pub rows: Vec<Vec<f64>>,
    pub offset: usize,
    pub mask: Option<Vec<Vec<bool>>>,
    pub data_type: Option<Vec<Vec<bool>>>,
}

#[derive(Default)]
pub enum ReferencePoints {
    #[default]
    All,
    Selected(Vec<i64>),
}

#[derive(Default)]
pub struct JointOptions {
    pub reference_points: ReferencePoints,
    pub missing_flag: Option<f64>,
    pub mask_type: MaskType,
    pub remove_missing_upto_maxlag: bool,
}

pub struct JointArray {
    /// Per-dataset block counts, including the source's fewer-than-ten warning.
    pub bootstrap: Vec<(usize, crate::bootstrap::BlockSummary)>,
    pub values: Vec<Vec<f64>>,
    pub roles: Vec<Role>,
    pub cleaned: CleanedXyz,
    /// (dataset position, local reference point), never a fictitious global time.
    pub references: Vec<(usize, usize)>,
    pub data_type: Option<Vec<Vec<bool>>>,
}

/// Analytical CI configuration borrows validated data; it does not alter its
/// time alignment or mask metadata.
pub struct AnalyticCi<'a> {
    pub(crate) data: &'a JointData,
    pub(crate) correlation: Correlation,
    cache: RefCell<BTreeMap<[[u8; 20]; 3], AnalyticResult>>,
    residuals: Residuals,
}

#[derive(Clone, Copy, Default)]
pub(crate) enum Residuals {
    #[default]
    Fresh,
    SourceRecycling,
}

impl AnalyticCi<'_> {
    pub fn with_residual_recycling(mut self) -> Self {
        self.residuals = self.data.recycling_policy();
        self
    }
    pub fn cached_tests(&self) -> usize {
        self.cache.borrow().len()
    }

    pub fn evidence(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
    ) -> Result<AnalyticResult, JointError> {
        let array = self.data.test_array(x, y, z, tau)?;
        let key = array.ci_key();
        let mut cache = self.cache.borrow_mut();
        if let Some(result) = cache.get(&key) {
            return Ok(*result);
        }
        let samples = array.samples()?;
        self.data.check_recycling(&samples, self.residuals)?;
        let result = samples
            .analytic_result(self.correlation)
            .map_err(JointError::Samples)?;
        cache.insert(key, result);
        Ok(result)
    }

    pub fn test(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
    ) -> Result<(f64, f64), JointError> {
        Ok(self.evidence(x, y, z, tau)?.values())
    }
}

impl JointArray {
    pub fn samples(self) -> Result<Samples, JointError> {
        Samples::new(self.values, self.roles).map_err(JointError::Samples)
    }
}

impl JointData {
    pub(crate) fn recycling_policy(&self) -> Residuals {
        if self.mask_type == MaskType::NONE {
            Residuals::SourceRecycling
        } else {
            Residuals::Fresh
        }
    }

    pub(crate) fn check_recycling(
        &self,
        samples: &Samples,
        policy: Residuals,
    ) -> Result<(), JointError> {
        if matches!(policy, Residuals::SourceRecycling)
            && samples
                .nonconstant()
                .map_err(JointError::Samples)?
                .is_some()
        {
            // Pinned ParCorrMult requires xyz; CondIndTest's recycling call omits it.
            return Err(JointError::ResidualRecyclingSignature);
        }
        Ok(())
    }
    /// Dense, zero-offset shorthand for the general source-compatible boundary.
    pub fn new(datasets: Vec<Vec<Vec<f64>>>, vectors: Vec<Vec<Node>>) -> Result<Self, JointError> {
        Self::from_datasets(
            datasets
                .into_iter()
                .map(|rows| DatasetInput {
                    rows,
                    offset: 0,
                    mask: None,
                    data_type: None,
                })
                .collect(),
            vectors,
            JointOptions::default(),
        )
    }

    pub fn from_datasets(
        datasets: Vec<DatasetInput>,
        vectors: Vec<Vec<Node>>,
        options: JointOptions,
    ) -> Result<Self, JointError> {
        if datasets.is_empty() {
            return Err(JointError::EmptyDatasets);
        }
        if !datasets.iter().any(|d| d.offset == 0) {
            return Err(JointError::MissingZeroOffset);
        }
        if datasets
            .iter()
            .any(|d| d.mask.is_some() != datasets[0].mask.is_some())
        {
            return Err(JointError::InconsistentMasks);
        }
        if datasets
            .iter()
            .any(|d| d.data_type.is_some() != datasets[0].data_type.is_some())
        {
            return Err(JointError::InconsistentTypes);
        }
        let datasets = datasets
            .into_iter()
            .map(|dataset| {
                let mut frame = TigramiteFrame::from_missing_flag(
                    dataset.rows,
                    options.missing_flag,
                    dataset.mask,
                )
                .map_err(JointError::Frame)?;
                if let Some(types) = dataset.data_type {
                    frame = frame.with_data_type(types).map_err(JointError::Frame)?;
                }
                Ok(Dataset {
                    frame,
                    offset: dataset.offset,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let n = datasets[0].frame.data.n;
        if datasets.iter().any(|d| d.frame.data.n != n) || vectors.is_empty() {
            return Err(JointError::Shape);
        }
        for (i, components) in vectors.iter().enumerate() {
            if components.is_empty() {
                return Err(JointError::EmptyVariable(i));
            }
            for &node in components {
                if node.0 >= n || node.1 > 0 {
                    return Err(JointError::InvalidComponent(node));
                }
            }
        }
        let end = datasets
            .iter()
            .map(|d| {
                d.offset
                    .checked_add(d.frame.data.t)
                    .ok_or(JointError::LagOverflow)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .max()
            .unwrap();
        let mut discarded_reference_points = Vec::new();
        let reference_points = match options.reference_points {
            ReferencePoints::All => (0..end).collect(),
            ReferencePoints::Selected(points) => points
                .into_iter()
                .filter_map(|p| match usize::try_from(p) {
                    Ok(p) if p < end => Some(p),
                    _ => {
                        discarded_reference_points.push(p);
                        None
                    }
                })
                .collect::<Vec<_>>(),
        };
        if reference_points.is_empty() {
            return Err(JointError::NoReferencePoints);
        }
        Ok(Self {
            datasets,
            vectors,
            reference_points,
            discarded_reference_points,
            mask_type: options.mask_type,
            propagate_missing: options.remove_missing_upto_maxlag,
            bootstrap: None,
        })
    }

    /// Consumes the data so a live CI cache cannot observe a changed sampler.
    pub fn with_bootstrap(mut self, bootstrap: crate::bootstrap::Bootstrap) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }

    /// Points removed by the source range check (reported as warnings in Python).
    pub fn discarded_reference_points(&self) -> &[i64] {
        &self.discarded_reference_points
    }

    pub fn variables(&self) -> usize {
        self.vectors.len()
    }

    pub fn analytic(&self, correlation: Correlation) -> AnalyticCi<'_> {
        AnalyticCi {
            data: self,
            correlation,
            cache: RefCell::new(BTreeMap::new()),
            residuals: Residuals::Fresh,
        }
    }

    fn vectorize(&self, nodes: &[Node]) -> Result<Vec<Node>, JointError> {
        let mut out = Vec::new();
        for &node in nodes {
            if node.1 > 0 {
                return Err(JointError::InvalidNode(node));
            }
            let components = self
                .vectors
                .get(node.0)
                .ok_or(JointError::InvalidNode(node))?;
            for &(i, lag) in components {
                out.push((i, lag.checked_add(node.1).ok_or(JointError::LagOverflow)?));
            }
        }
        Ok(out)
    }

    pub fn construct(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau_max: usize,
    ) -> Result<JointArray, JointError> {
        self.construct_with_cutoff(x, y, z, tau_max, CutOff::TwoTauMax)
    }

    pub fn construct_with_cutoff(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau_max: usize,
        cut_off: CutOff,
    ) -> Result<JointArray, JointError> {
        let x = self.vectorize(x)?;
        let y = self.vectorize(y)?;
        let z = self.vectorize(z)?;
        let mut result: Option<JointArray> = None;
        let mut bootstrap = Vec::new();
        for (dataset, Dataset { frame, offset }) in self.datasets.iter().enumerate() {
            let references: Vec<_> = self
                .reference_points
                .iter()
                .filter_map(|p| p.checked_sub(*offset))
                .collect();
            let a = match construct_array_tracked(
                frame,
                &x,
                &y,
                &z,
                &[],
                tau_max,
                ConstructOptions {
                    cut_off,
                    reference_points: Some(&references),
                    mask_type: self.mask_type,
                    remove_missing_upto_maxlag: self.propagate_missing,
                    bootstrap: self.bootstrap.as_ref(),
                    ..ConstructOptions::default()
                },
            ) {
                Ok(a) => a,
                Err(PreprocessingError::NoValidSamples) => continue,
                Err(PreprocessingError::EmptyBootstrap(summary)) => {
                    bootstrap.push((dataset, summary));
                    continue;
                }
                Err(error) => return Err(JointError::Frame(error)),
            };
            if let Some(summary) = a.bootstrap {
                bootstrap.push((dataset, summary));
            }
            let references = a
                .retained_reference_points
                .into_iter()
                .map(|t| (dataset, t));
            match &mut result {
                Some(out) => {
                    for (to, from) in out.values.iter_mut().zip(a.values) {
                        to.extend(from);
                    }
                    out.references.extend(references);
                    if let (Some(to), Some(from)) = (&mut out.data_type, a.data_type) {
                        for (to, from) in to.iter_mut().zip(from) {
                            to.extend(from);
                        }
                    }
                }
                None => {
                    result = Some(JointArray {
                        bootstrap: Vec::new(),
                        values: a.values,
                        roles: a
                            .xyz
                            .into_iter()
                            .map(|r| match r {
                                0 => Role::X,
                                1 => Role::Y,
                                2 => Role::Z,
                                _ => unreachable!(),
                            })
                            .collect(),
                        cleaned: a.cleaned,
                        references: references.collect(),
                        data_type: a.data_type,
                    })
                }
            }
        }
        match result {
            Some(mut out) => {
                out.bootstrap = bootstrap;
                Ok(out)
            }
            None if !bootstrap.is_empty() => Err(JointError::EmptyBootstrap(bootstrap)),
            None => Err(JointError::Frame(PreprocessingError::NoValidSamples)),
        }
    }

    pub fn test(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau_max: usize,
    ) -> Result<(f64, f64), JointError> {
        self.test_with(x, y, z, tau_max, Correlation::MaxCorrelation)
    }

    pub fn test_with(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau_max: usize,
        correlation: Correlation,
    ) -> Result<(f64, f64), JointError> {
        self.test_array(x, y, z, tau_max)?
            .samples()?
            .run_test_with(correlation)
            .map_err(JointError::Samples)
    }

    pub(crate) fn test_array(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau_max: usize,
    ) -> Result<JointArray, JointError> {
        if self.mask_type != MaskType::NONE && self.datasets[0].frame.analysis_mask.is_none() {
            return Err(JointError::MaskRequired);
        }
        self.construct(x, y, z, tau_max)
    }
}
