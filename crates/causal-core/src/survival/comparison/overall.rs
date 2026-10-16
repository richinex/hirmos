//! ComparisonSurv's aggregate `Overall.test` orchestration.

use std::num::NonZeroUsize;

use super::cox::{proportional_hazards_check, CoxError, ProportionalHazardsCheck};
use super::data::{ComparisonData, Group};
use super::descriptive::{
    describe, ComparisonTime, DescriptiveError, RmstContrast, RmstWindow, SignificanceLevel,
};
use super::methods::{
    breslow_tarone_ware, lin_wang, lin_xu, log_rank, weighted_kaplan_meier, ComparisonError,
    LinWangResult, LinXuResult, Side, Weight, WeightedKaplanMeierResult, WeightedRankResult,
};
use super::tshrc::{two_stage_with_rng, TwoStageConfiguration, TwoStageError, TwoStageResult};
use crate::survival::r_rng::RRng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverallConfiguration {
    truncation_time: f64,
    permutations: NonZeroUsize,
    seed: u32,
}

impl OverallConfiguration {
    pub fn new(
        truncation_time: f64,
        permutations: NonZeroUsize,
        seed: u32,
    ) -> Result<Self, OverallError> {
        if !truncation_time.is_finite() || truncation_time < 0.0 {
            return Err(OverallError::InvalidTruncationTime);
        }
        Ok(Self {
            truncation_time,
            permutations,
            seed,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogRankResult {
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OverallResult {
    pub proportional_hazards: ProportionalHazardsCheck,
    pub log_rank: LogRankResult,
    pub gehan_wilcoxon: WeightedRankResult,
    pub tarone_ware: WeightedRankResult,
    pub weighted_kaplan_meier: WeightedKaplanMeierResult,
    pub absolute_difference: LinXuResult,
    pub absolute_difference_permutation_p_value: f64,
    pub two_stage: TwoStageResult,
    pub squared_differences: LinWangResult,
    pub restricted_mean_difference: RmstContrast,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverallError {
    InvalidTruncationTime,
    Comparison(ComparisonError),
    Description(DescriptiveError),
    TwoStage(TwoStageError),
    Cox(CoxError),
}

impl From<ComparisonError> for OverallError {
    fn from(value: ComparisonError) -> Self {
        Self::Comparison(value)
    }
}

impl From<DescriptiveError> for OverallError {
    fn from(value: DescriptiveError) -> Self {
        Self::Description(value)
    }
}

impl From<TwoStageError> for OverallError {
    fn from(value: TwoStageError) -> Self {
        Self::TwoStage(value)
    }
}

impl From<CoxError> for OverallError {
    fn from(value: CoxError) -> Self {
        Self::Cox(value)
    }
}

pub fn overall_test(
    data: &ComparisonData,
    configuration: OverallConfiguration,
) -> Result<OverallResult, OverallError> {
    let mut rng = RRng::new(configuration.seed);
    let two_stage = two_stage_with_rng(
        data,
        TwoStageConfiguration::new(configuration.permutations, 0.05, 0.1, configuration.seed)?,
        &mut rng,
    )?;
    let absolute_difference = lin_xu(data, Side::TwoSided, 0.5)?;
    let zero_size = data.group_samples(Group::Zero).len();
    let mut labels = vec![Group::Zero; zero_size];
    labels.extend(vec![Group::One; data.group_samples(Group::One).len()]);
    let mut at_least_as_extreme = 0;
    for _ in 0..configuration.permutations.get() {
        let permutation = rng.permutation(labels.len());
        let samples = data
            .samples()
            .iter()
            .zip(permutation)
            .map(|(sample, label_index)| sample.with_group(labels[label_index]))
            .collect::<Vec<_>>();
        let permuted = ComparisonData::new(samples)
            .map_err(|_| OverallError::Comparison(ComparisonError::UndefinedStatistic))?;
        if lin_xu(&permuted, Side::TwoSided, 0.5)?.statistic.abs()
            > absolute_difference.statistic.abs()
        {
            at_least_as_extreme += 1;
        }
    }
    let (log_rank_statistic, log_rank_p_value) = log_rank(data)?;
    let restricted_mean = describe(
        data,
        RmstWindow::At(ComparisonTime::new(configuration.truncation_time)?),
        SignificanceLevel::new(0.05)?,
    )?;
    Ok(OverallResult {
        proportional_hazards: proportional_hazards_check(data)?,
        log_rank: LogRankResult {
            statistic: log_rank_statistic,
            p_value: log_rank_p_value,
        },
        gehan_wilcoxon: breslow_tarone_ware(data, Weight::GehanWilcoxon),
        tarone_ware: breslow_tarone_ware(data, Weight::TaroneWare),
        weighted_kaplan_meier: weighted_kaplan_meier(data)?,
        absolute_difference,
        absolute_difference_permutation_p_value: at_least_as_extreme as f64
            / configuration.permutations.get() as f64,
        two_stage,
        squared_differences: lin_wang(data),
        restricted_mean_difference: restricted_mean
            .restricted_mean_comparison
            .group_one_minus_zero,
    })
}
