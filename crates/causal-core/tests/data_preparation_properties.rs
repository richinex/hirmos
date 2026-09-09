use hirmos_causal_core::data_preparation::{
    carry_forward_bounded, interpolate_bounded_linear, project_lagged, AnalysisExclusionPolicy,
    ColumnId, ExclusionReason, FeatureRole, GapInfluence, HistoryRequirement, ImputationOutcome,
    LagProjectionRequest, LaggedFeature, Lookback, MatrixLayout, NonEmptyVec, NullableDataset,
    PositiveUsize, ProjectionOutcome, ReferenceDomain, SampleDecision,
};
use proptest::collection::vec;
use proptest::prelude::*;

fn bool_rows(flat: &[bool], times: usize, columns: usize) -> Vec<Vec<bool>> {
    (0..times)
        .map(|time| flat[time * columns..(time + 1) * columns].to_vec())
        .collect()
}

fn value_rows(flat: &[i16], times: usize, columns: usize) -> Vec<Vec<f64>> {
    (0..times)
        .map(|time| {
            flat[time * columns..(time + 1) * columns]
                .iter()
                .map(|value| f64::from(*value))
                .collect()
        })
        .collect()
}

fn single_feature_request(
    column: ColumnId,
    lookback: usize,
    gap_influence: GapInfluence,
    layout: MatrixLayout,
) -> LagProjectionRequest {
    LagProjectionRequest {
        features: NonEmptyVec::try_from_vec(vec![LaggedFeature::new(
            column,
            Lookback::new(lookback),
            FeatureRole::TestedOutcome,
        )])
        .unwrap(),
        reference_domain: ReferenceDomain::AllAvailable,
        history: HistoryRequirement::MinimumForFeatures,
        gap_influence,
        analysis_exclusions: AnalysisExclusionPolicy::Ignore,
        layout,
    }
}

fn built(outcome: ProjectionOutcome) -> hirmos_causal_core::data_preparation::LaggedAnalysisMatrix {
    match outcome {
        ProjectionOutcome::Built(matrix) => matrix,
        ProjectionOutcome::Refused { reasons, .. } => panic!("unexpected refusal: {reasons:?}"),
    }
}

prop_compose! {
    fn nullable_matrix()
        (times in 1usize..16, columns in 1usize..5)
        (
            times in Just(times),
            columns in Just(columns),
            values in vec(any::<i16>(), times * columns),
            validity in vec(any::<bool>(), times * columns),
        ) -> (usize, usize, Vec<i16>, Vec<bool>)
    {
        (times, columns, values, validity)
    }
}

prop_compose! {
    fn dense_projection_case()
        (times in 2usize..20, columns in 1usize..5)
        (
            times in Just(times),
            columns in Just(columns),
            values in vec(any::<i16>(), times * columns),
            column in 0usize..columns,
            lookback in 0usize..times,
        ) -> (usize, usize, Vec<i16>, usize, usize)
    {
        (times, columns, values, column, lookback)
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: None,
        .. ProptestConfig::default()
    })]

    #[test]
    fn invalid_physical_storage_never_changes_the_dataset_or_projection(
        (times, columns, raw, validity) in nullable_matrix()
    ) {
        let mut first = value_rows(&raw, times, columns);
        let mut second = first.clone();
        for time in 0..times {
            for column in 0..columns {
                if !validity[time * columns + column] {
                    first[time][column] = f64::NAN;
                    second[time][column] = 1.0e250 + (time * columns + column) as f64;
                }
            }
        }
        let validity_rows = bool_rows(&validity, times, columns);
        let coordinates = (0..times).map(|time| time as f64).collect::<Vec<_>>();
        let a = NullableDataset::from_rows(
            first,
            validity_rows.clone(),
            None,
            coordinates.clone(),
        ).unwrap();
        let b = NullableDataset::from_rows(second, validity_rows, None, coordinates).unwrap();
        prop_assert_eq!(&a, &b);

        let column = ColumnId::new(0, columns).unwrap();
        let request = single_feature_request(
            column,
            0,
            GapInfluence::DirectOnly,
            MatrixLayout::FeaturesBySamples,
        );
        prop_assert_eq!(
            project_lagged(&a, &request).unwrap(),
            project_lagged(&b, &request).unwrap()
        );
    }

    #[test]
    fn dense_projection_maps_every_value_to_its_declared_source_and_layout(
        (times, columns, raw, selected_column, lookback) in dense_projection_case()
    ) {
        let rows = value_rows(&raw, times, columns);
        let dataset = NullableDataset::from_rows(
            rows.clone(),
            vec![vec![true; columns]; times],
            None,
            (0..times).map(|time| time as f64).collect(),
        ).unwrap();
        let column = ColumnId::new(selected_column, columns).unwrap();
        let feature_rows = built(project_lagged(
            &dataset,
            &single_feature_request(
                column,
                lookback,
                GapInfluence::DirectOnly,
                MatrixLayout::FeaturesBySamples,
            ),
        ).unwrap());
        let sample_rows = built(project_lagged(
            &dataset,
            &single_feature_request(
                column,
                lookback,
                GapInfluence::DirectOnly,
                MatrixLayout::SamplesByFeatures,
            ),
        ).unwrap());
        let expected_references = (lookback..times).collect::<Vec<_>>();
        let expected_values = expected_references
            .iter()
            .map(|reference| rows[reference - lookback][selected_column])
            .collect::<Vec<_>>();
        prop_assert_eq!(
            feature_rows.retained_references.as_slice(),
            expected_references.as_slice()
        );
        prop_assert_eq!(&feature_rows.values, &vec![expected_values.clone()]);
        prop_assert_eq!(
            sample_rows.values,
            expected_values.into_iter().map(|value| vec![value]).collect::<Vec<_>>()
        );
        prop_assert_eq!(feature_rows.decisions.len(), times);
        for decision in feature_rows.decisions.as_slice() {
            if let SampleDecision::Excluded { reasons, .. } = decision {
                prop_assert!(!reasons.is_empty());
            }
        }
    }

    #[test]
    fn following_guard_only_excludes_the_declared_following_candidates(
        times in 2usize..30,
        missing in vec(any::<bool>(), 2usize..30),
        guard_steps in 1usize..8,
    ) {
        let times = times.min(missing.len());
        let missing = &missing[..times];
        let validity = missing.iter().map(|is_missing| vec![!*is_missing]).collect();
        let dataset = NullableDataset::from_rows(
            (0..times).map(|time| vec![time as f64]).collect(),
            validity,
            None,
            (0..times).map(|time| time as f64).collect(),
        ).unwrap();
        let request = single_feature_request(
            ColumnId::new(0, 1).unwrap(),
            0,
            GapInfluence::FollowingGuard(PositiveUsize::new(guard_steps).unwrap()),
            MatrixLayout::FeaturesBySamples,
        );
        let outcome = project_lagged(&dataset, &request).unwrap();
        let decisions = match &outcome {
            ProjectionOutcome::Built(matrix) => matrix.decisions.as_slice(),
            ProjectionOutcome::Refused { decisions, .. } => decisions.as_slice(),
        };
        for decision in decisions {
            if let SampleDecision::Excluded { reference, reasons } = decision {
                for reason in reasons.as_slice() {
                    if let ExclusionReason::FollowingMissingnessGuard {
                        origin_reference,
                        steps_after,
                    } = reason
                    {
                        prop_assert!(missing[*origin_reference]);
                        prop_assert!(*steps_after >= 1 && *steps_after <= guard_steps);
                        prop_assert_eq!(*reference, origin_reference + steps_after);
                    }
                }
            }
        }
    }
}

#[test]
fn imputation_is_idempotent_and_preserves_original_values() {
    let dataset = NullableDataset::from_rows(
        vec![
            vec![1.0, 10.0],
            vec![999.0, 11.0],
            vec![3.0, 12.0],
            vec![999.0, 13.0],
            vec![5.0, 14.0],
        ],
        vec![
            vec![true, true],
            vec![false, true],
            vec![true, true],
            vec![false, true],
            vec![true, true],
        ],
        None,
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
    )
    .unwrap();
    let gap = PositiveUsize::new(1).unwrap();
    let linear = interpolate_bounded_linear(&dataset, gap);
    assert_eq!(linear.dataset().values()[0], 1.0);
    assert_eq!(linear.dataset().values()[4], 3.0);
    assert_eq!(linear.dataset().values()[8], 5.0);
    assert!(matches!(
        interpolate_bounded_linear(linear.dataset(), gap),
        ImputationOutcome::NoChange { .. }
    ));
    assert_eq!(
        linear.dataset().imputed(),
        &[false, false, true, false, false, false, true, false, false, false]
    );
    let projected = built(
        project_lagged(
            linear.dataset(),
            &single_feature_request(
                ColumnId::new(0, 2).unwrap(),
                0,
                GapInfluence::DirectOnly,
                MatrixLayout::FeaturesBySamples,
            ),
        )
        .unwrap(),
    );
    assert_eq!(
        projected.imputed,
        vec![vec![false, true, false, true, false]],
        "projection carries transformation details"
    );

    let forward = carry_forward_bounded(&dataset, gap);
    assert_eq!(forward.dataset().values()[0], 1.0);
    assert_eq!(forward.dataset().values()[2], 1.0);
    assert!(matches!(
        carry_forward_bounded(forward.dataset(), gap),
        ImputationOutcome::NoChange { .. }
    ));
}

#[test]
fn translating_or_positively_scaling_time_does_not_change_interpolation() {
    let rows = vec![vec![0.0], vec![999.0], vec![999.0], vec![9.0]];
    let validity = vec![vec![true], vec![false], vec![false], vec![true]];
    let build = |coordinates| {
        NullableDataset::from_rows(rows.clone(), validity.clone(), None, coordinates).unwrap()
    };
    let base = build(vec![0.0, 1.0, 2.0, 3.0]);
    let translated = build(vec![100.0, 101.0, 102.0, 103.0]);
    let scaled = build(vec![0.0, 7.0, 14.0, 21.0]);
    let gap = PositiveUsize::new(2).unwrap();
    assert_eq!(
        interpolate_bounded_linear(&base, gap).dataset().values(),
        interpolate_bounded_linear(&translated, gap)
            .dataset()
            .values()
    );
    assert_eq!(
        interpolate_bounded_linear(&base, gap).dataset().values(),
        interpolate_bounded_linear(&scaled, gap).dataset().values()
    );
}
