use std::collections::BTreeSet;

use super::model::{
    DataPreparationError, ExclusionReason, GapInfluence, HistoryRequirement, LagProjectionRequest,
    LaggedAnalysisMatrix, LaggedFeature, MatrixLayout, NonEmptyVec, NullableDataset,
    PreparationRefusal, ProjectionOutcome, ReferenceDomain, SampleDecision,
};

#[derive(Clone, Debug)]
struct PendingDecision {
    reference: usize,
    reasons: Vec<ExclusionReason>,
}

fn push_unique(reasons: &mut Vec<ExclusionReason>, reason: ExclusionReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn deduplicate_features(features: &[LaggedFeature]) -> Vec<LaggedFeature> {
    let mut seen = BTreeSet::new();
    features
        .iter()
        .copied()
        .filter(|feature| seen.insert((feature.column, feature.lookback, feature.role)))
        .collect()
}

fn references(
    dataset: &NullableDataset,
    domain: &ReferenceDomain,
) -> Result<Vec<PendingDecision>, DataPreparationError> {
    match domain {
        ReferenceDomain::AllAvailable => Ok((0..dataset.time_count())
            .map(|reference| PendingDecision {
                reference,
                reasons: Vec::new(),
            })
            .collect()),
        ReferenceDomain::Window {
            start,
            end_exclusive,
        } => {
            if start >= end_exclusive || *end_exclusive > dataset.time_count() {
                return Err(DataPreparationError::InvalidWindow {
                    start: *start,
                    end_exclusive: *end_exclusive,
                });
            }
            Ok((0..dataset.time_count())
                .map(|reference| PendingDecision {
                    reference,
                    reasons: if reference < *start || reference >= *end_exclusive {
                        vec![ExclusionReason::OutsideWindow]
                    } else {
                        Vec::new()
                    },
                })
                .collect())
        }
        ReferenceDomain::Explicit(points) => {
            let mut values = points.as_slice().to_vec();
            values.sort_unstable();
            if let Some(pair) = values.windows(2).find(|pair| pair[0] == pair[1]) {
                return Err(DataPreparationError::DuplicateReferencePoint { reference: pair[0] });
            }
            Ok(values
                .into_iter()
                .map(|reference| PendingDecision {
                    reference,
                    reasons: if reference >= dataset.time_count() {
                        vec![ExclusionReason::OutsideDataset]
                    } else {
                        Vec::new()
                    },
                })
                .collect())
        }
    }
}

pub fn project_lagged(
    dataset: &NullableDataset,
    request: &LagProjectionRequest,
) -> Result<ProjectionOutcome, DataPreparationError> {
    let features = deduplicate_features(request.features.as_slice());
    for feature in &features {
        if feature.column.index() >= dataset.column_count() {
            return Err(DataPreparationError::InvalidColumn {
                column: feature.column.index(),
                column_count: dataset.column_count(),
            });
        }
    }
    let maximum_lookback = features
        .iter()
        .map(|feature| feature.lookback.steps())
        .max()
        .unwrap_or(0);
    let warmup = match request.history {
        HistoryRequirement::MinimumForFeatures => maximum_lookback,
        HistoryRequirement::FixedWarmup(warmup) => {
            if warmup.steps() < maximum_lookback {
                return Err(DataPreparationError::WarmupShorterThanFeatureHistory {
                    warmup: warmup.steps(),
                    required: maximum_lookback,
                });
            }
            warmup.steps()
        }
    };

    let mut pending = references(dataset, &request.reference_domain)?;
    for decision in &mut pending {
        if decision.reference < warmup {
            push_unique(
                &mut decision.reasons,
                ExclusionReason::InsufficientHistory { required: warmup },
            );
        }
    }

    let candidate_positions: Vec<usize> = pending
        .iter()
        .enumerate()
        .filter_map(|(index, decision)| decision.reasons.is_empty().then_some(index))
        .collect();
    let mut directly_missing = Vec::new();
    for (candidate_position, &pending_index) in candidate_positions.iter().enumerate() {
        let reference = pending[pending_index].reference;
        for (feature_index, feature) in features.iter().enumerate() {
            let source_time = reference - feature.lookback.steps();
            if !dataset.is_valid(source_time, feature.column) {
                push_unique(
                    &mut pending[pending_index].reasons,
                    ExclusionReason::MissingSourceCell {
                        feature: feature_index,
                        source_time,
                        column: feature.column,
                    },
                );
            }
        }
        if pending[pending_index]
            .reasons
            .iter()
            .any(|reason| matches!(reason, ExclusionReason::MissingSourceCell { .. }))
        {
            directly_missing.push(candidate_position);
        }
    }

    if let GapInfluence::FollowingGuard(steps) = request.gap_influence {
        for origin_position in directly_missing {
            let origin_reference = pending[candidate_positions[origin_position]].reference;
            let end =
                (origin_position + steps.get()).min(candidate_positions.len().saturating_sub(1));
            for following_position in (origin_position + 1)..=end {
                let pending_index = candidate_positions[following_position];
                push_unique(
                    &mut pending[pending_index].reasons,
                    ExclusionReason::FollowingMissingnessGuard {
                        origin_reference,
                        steps_after: following_position - origin_position,
                    },
                );
            }
        }
    }

    for &pending_index in &candidate_positions {
        let reference = pending[pending_index].reference;
        for (feature_index, feature) in features.iter().enumerate() {
            if !request.analysis_exclusions.applies_to(feature.role) {
                continue;
            }
            let source_time = reference - feature.lookback.steps();
            if dataset.is_excluded(source_time, feature.column) {
                push_unique(
                    &mut pending[pending_index].reasons,
                    ExclusionReason::AnalysisExcluded {
                        feature: feature_index,
                        source_time,
                        column: feature.column,
                        role: feature.role,
                    },
                );
            }
        }
    }

    let retained: Vec<usize> = pending
        .iter()
        .filter_map(|decision| decision.reasons.is_empty().then_some(decision.reference))
        .collect();
    let decisions = NonEmptyVec::try_from_vec(
        pending
            .into_iter()
            .map(|decision| {
                if decision.reasons.is_empty() {
                    SampleDecision::Retained {
                        reference: decision.reference,
                    }
                } else {
                    SampleDecision::Excluded {
                        reference: decision.reference,
                        reasons: NonEmptyVec::try_from_vec(decision.reasons)
                            .expect("the excluded branch contains at least one reason"),
                    }
                }
            })
            .collect(),
    )?;

    if retained.is_empty() {
        return Ok(ProjectionOutcome::Refused {
            decisions,
            reasons: NonEmptyVec::try_from_vec(vec![PreparationRefusal::NoRetainedSamples])
                .expect("one refusal reason is present"),
        });
    }

    let rows: Vec<Vec<f64>> = features
        .iter()
        .map(|feature| {
            retained
                .iter()
                .map(|reference| {
                    dataset.value(reference - feature.lookback.steps(), feature.column)
                })
                .collect()
        })
        .collect();
    let type_rows = dataset.data_type.as_ref().map(|_| {
        features
            .iter()
            .map(|feature| {
                retained
                    .iter()
                    .map(|reference| {
                        dataset
                            .is_discrete(reference - feature.lookback.steps(), feature.column)
                            .expect("data type exists for every source cell")
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>()
    });
    let imputed_rows: Vec<Vec<bool>> = features
        .iter()
        .map(|feature| {
            retained
                .iter()
                .map(|reference| {
                    dataset.is_imputed(reference - feature.lookback.steps(), feature.column)
                })
                .collect()
        })
        .collect();
    let (values, data_type, imputed) = match request.layout {
        MatrixLayout::FeaturesBySamples => (rows, type_rows, imputed_rows),
        MatrixLayout::SamplesByFeatures => {
            let values = (0..retained.len())
                .map(|sample| rows.iter().map(|row| row[sample]).collect())
                .collect();
            let data_type = type_rows.map(|rows| {
                (0..retained.len())
                    .map(|sample| rows.iter().map(|row| row[sample]).collect())
                    .collect()
            });
            let imputed = (0..retained.len())
                .map(|sample| imputed_rows.iter().map(|row| row[sample]).collect())
                .collect();
            (values, data_type, imputed)
        }
    };

    Ok(ProjectionOutcome::Built(LaggedAnalysisMatrix {
        values,
        data_type,
        imputed,
        layout: request.layout,
        features: NonEmptyVec::try_from_vec(features)
            .expect("deduplicating a non-empty feature list remains non-empty"),
        retained_references: NonEmptyVec::try_from_vec(retained)
            .expect("the empty retained case returned a refusal"),
        decisions,
    }))
}
