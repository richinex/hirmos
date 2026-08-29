use hirmos_causal_core::data_preparation::{
    carry_forward_bounded, fill_confirmed_structural_zero, interpolate_bounded_linear,
    longest_complete_interval, project_lagged, AnalysisExclusionPolicy, ColumnId,
    CompleteIntervalOutcome, ConfirmationId, DataPreparationError, FeatureRole, GapInfluence,
    HistoryRequirement, ImputationOutcome, LagProjectionRequest, LaggedAnalysisMatrix,
    LaggedFeature, Lookback, MatrixLayout, NonEmptyVec, NullableDataset, NumericNullPolicy,
    PositiveUsize, ProjectionOutcome, ReferenceDomain, SampleDecision,
};
use serde::Deserialize;

type OracleNode = (usize, i32);

#[derive(Deserialize)]
struct Fixture {
    tigramite: Vec<OracleCase>,
    imputation: ImputationFixture,
}

#[derive(Deserialize)]
struct OracleCase {
    name: String,
    values: Vec<Vec<f64>>,
    missing_flag: Option<f64>,
    analysis_mask: Option<Vec<Vec<bool>>>,
    data_type: Vec<Vec<bool>>,
    x: Vec<OracleNode>,
    y: Vec<OracleNode>,
    z: Vec<OracleNode>,
    extra_z: Vec<OracleNode>,
    tau_max: usize,
    cutoff: String,
    mask_type: Option<String>,
    propagate: bool,
    reference_points: Vec<usize>,
    array: Vec<Vec<f64>>,
    xyz: Vec<u8>,
    cleaned: Vec<Vec<OracleNode>>,
    type_array: Vec<Vec<u8>>,
    retained_reference_points: Vec<usize>,
}

#[derive(Deserialize)]
struct ImputationFixture {
    values: Vec<Vec<f64>>,
    valid: Vec<Vec<bool>>,
    coordinates: Vec<f64>,
    max_gap: usize,
    linear: ImputedExpected,
    forward_fill: ImputedExpected,
    structural_zero: ImputedExpected,
    complete_interval: Vec<usize>,
}

#[derive(Deserialize)]
struct ImputedExpected {
    values: Vec<Vec<f64>>,
    valid: Vec<Vec<bool>>,
    imputed: Vec<Vec<bool>>,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!(
        "../oracle/fixtures/missing_preprocessing.json"
    ))
    .expect("the frozen oracle fixture is valid JSON")
}

fn deduplicate(nodes: &[OracleNode]) -> Vec<OracleNode> {
    let mut result = Vec::new();
    for node in nodes {
        if !result.contains(node) {
            result.push(*node);
        }
    }
    result
}

fn compatibility_features(case: &OracleCase) -> Vec<(OracleNode, FeatureRole)> {
    let x = deduplicate(&case.x);
    let y = deduplicate(&case.y);
    let z: Vec<_> = deduplicate(&case.z)
        .into_iter()
        .filter(|node| !x.contains(node) && !y.contains(node))
        .collect();
    let extra: Vec<_> = deduplicate(&case.extra_z)
        .into_iter()
        .filter(|node| !x.contains(node) && !y.contains(node) && !z.contains(node))
        .collect();
    x.into_iter()
        .map(|node| (node, FeatureRole::CandidateCause))
        .chain(y.into_iter().map(|node| (node, FeatureRole::TestedOutcome)))
        .chain(z.into_iter().map(|node| (node, FeatureRole::Conditioner)))
        .chain(extra.into_iter().map(|node| (node, FeatureRole::Auxiliary)))
        .collect()
}

fn roles_for_mask(value: Option<&str>) -> AnalysisExclusionPolicy {
    let roles = match value {
        None => return AnalysisExclusionPolicy::Ignore,
        Some("x") => vec![FeatureRole::CandidateCause],
        Some("y") => vec![FeatureRole::TestedOutcome],
        Some("z") => vec![FeatureRole::Conditioner],
        Some("xy") => vec![FeatureRole::CandidateCause, FeatureRole::TestedOutcome],
        Some("xz") => vec![FeatureRole::CandidateCause, FeatureRole::Conditioner],
        Some("yz") => vec![FeatureRole::TestedOutcome, FeatureRole::Conditioner],
        Some("xyz") => vec![
            FeatureRole::CandidateCause,
            FeatureRole::TestedOutcome,
            FeatureRole::Conditioner,
        ],
        Some(other) => panic!("unknown frozen mask policy {other}"),
    };
    AnalysisExclusionPolicy::ApplyTo(
        NonEmptyVec::try_from_vec(roles).expect("every named mask has at least one role"),
    )
}

fn role_code(role: FeatureRole) -> u8 {
    match role {
        FeatureRole::CandidateCause => 0,
        FeatureRole::TestedOutcome => 1,
        FeatureRole::Conditioner => 2,
        FeatureRole::Auxiliary => 3,
    }
}

fn rows_f64(dataset: &NullableDataset) -> Vec<Vec<f64>> {
    (0..dataset.time_count())
        .map(|time| {
            let start = time * dataset.column_count();
            dataset.values()[start..start + dataset.column_count()].to_vec()
        })
        .collect()
}

fn rows_bool(values: &[bool], times: usize, columns: usize) -> Vec<Vec<bool>> {
    (0..times)
        .map(|time| values[time * columns..(time + 1) * columns].to_vec())
        .collect()
}

fn imputed_bitmap(outcome: &ImputationOutcome) -> Vec<Vec<bool>> {
    let dataset = outcome.dataset();
    let mut result = vec![vec![false; dataset.column_count()]; dataset.time_count()];
    for cell in outcome.imputed_cells() {
        result[cell.time][cell.column.index()] = true;
    }
    result
}

fn assert_matrix_close(label: &str, got: &[Vec<f64>], want: &[Vec<f64>]) {
    assert_eq!(got.len(), want.len(), "{label}: row count");
    for (row, (got_row, want_row)) in got.iter().zip(want).enumerate() {
        assert_eq!(got_row.len(), want_row.len(), "{label}: row {row} width");
        for (column, (&got_value, &want_value)) in got_row.iter().zip(want_row).enumerate() {
            assert!(
                (got_value - want_value).abs() <= 1e-14,
                "{label}[{row},{column}]: got {got_value}, expected {want_value}"
            );
        }
    }
}

fn build_case(case: &OracleCase) -> LaggedAnalysisMatrix {
    let times = case.values.len();
    let columns = case.values[0].len();
    let coordinates = (0..times).map(|time| time as f64).collect();
    let null_policy = match case.missing_flag {
        Some(value) => NumericNullPolicy::ConfirmedSentinel {
            value,
            confirmation: ConfirmationId::new("frozen-oracle-fixture")
                .expect("the fixture confirmation is non-empty"),
        },
        None => NumericNullPolicy::NoSentinel,
    };
    let dataset = NullableDataset::import_numeric_rows(
        case.values.clone(),
        null_policy,
        case.analysis_mask.clone(),
        coordinates,
    )
    .unwrap_or_else(|error| panic!("{} import: {error}", case.name))
    .with_data_type(case.data_type.clone())
    .unwrap_or_else(|error| panic!("{} data types: {error}", case.name));
    assert_eq!(dataset.column_count(), columns);

    let compatibility = compatibility_features(case);
    let features: Vec<_> = compatibility
        .iter()
        .map(|&((column, lag), role)| {
            assert!(lag <= 0, "the frozen fixture contains only lookbacks");
            LaggedFeature::new(
                ColumnId::new(column, columns).expect("fixture column is in range"),
                Lookback::new((-lag) as usize),
                role,
            )
        })
        .collect();
    let maximum_lookback = features
        .iter()
        .map(|feature| feature.lookback.steps())
        .max()
        .expect("the fixture contains features");
    let (history, guard_steps, reference_points) = match case.cutoff.as_str() {
        "max_lag" => (
            HistoryRequirement::MinimumForFeatures,
            maximum_lookback,
            case.reference_points.clone(),
        ),
        "tau_max" => (
            HistoryRequirement::FixedWarmup(Lookback::new(case.tau_max)),
            case.tau_max,
            case.reference_points.clone(),
        ),
        "2xtau_max" => (
            HistoryRequirement::FixedWarmup(Lookback::new(2 * case.tau_max)),
            2 * case.tau_max,
            case.reference_points.clone(),
        ),
        "max_lag_or_tau_max" => {
            let warmup = maximum_lookback.max(case.tau_max);
            (
                HistoryRequirement::FixedWarmup(Lookback::new(warmup)),
                warmup,
                case.reference_points.clone(),
            )
        }
        "2xtau_max_future" => {
            let target_count = case
                .reference_points
                .iter()
                .filter(|&&point| point >= 2 * case.tau_max && point < times)
                .count();
            let mut points: Vec<_> = case
                .reference_points
                .iter()
                .copied()
                .filter(|&point| point >= maximum_lookback && point < times)
                .collect();
            points.sort_unstable();
            points.truncate(target_count);
            (
                HistoryRequirement::MinimumForFeatures,
                maximum_lookback,
                points,
            )
        }
        other => panic!("unknown frozen history policy {other}"),
    };
    let gap_influence = if case.propagate {
        GapInfluence::FollowingGuard(
            PositiveUsize::new(guard_steps).expect("fixture propagation has positive history"),
        )
    } else {
        GapInfluence::DirectOnly
    };
    let request = LagProjectionRequest {
        features: NonEmptyVec::try_from_vec(features).expect("fixture features are non-empty"),
        reference_domain: ReferenceDomain::Explicit(
            NonEmptyVec::try_from_vec(reference_points)
                .expect("fixture reference points are non-empty"),
        ),
        history,
        gap_influence,
        analysis_exclusions: roles_for_mask(case.mask_type.as_deref()),
        layout: MatrixLayout::FeaturesBySamples,
    };
    match project_lagged(&dataset, &request)
        .unwrap_or_else(|error| panic!("{} projection: {error}", case.name))
    {
        ProjectionOutcome::Built(matrix) => matrix,
        ProjectionOutcome::Refused { reasons, .. } => {
            panic!("{} unexpectedly refused: {reasons:?}", case.name)
        }
    }
}

#[test]
fn independent_projection_matches_all_18_frozen_reference_cases() {
    let fixture = fixture();
    assert_eq!(
        fixture.tigramite.len(),
        18,
        "the full frozen matrix is loaded"
    );
    for case in &fixture.tigramite {
        let matrix = build_case(case);
        assert_eq!(
            matrix.retained_references.as_slice(),
            case.retained_reference_points,
            "{} retained references",
            case.name
        );
        assert_eq!(
            matrix.values, case.array,
            "{} copied source values are bit-for-bit equal",
            case.name
        );
        let roles: Vec<_> = matrix
            .features
            .as_slice()
            .iter()
            .map(|feature| role_code(feature.role))
            .collect();
        assert_eq!(roles, case.xyz, "{} feature roles", case.name);
        let projected_types: Vec<Vec<u8>> = matrix
            .data_type
            .as_ref()
            .expect("the fixture declares data types")
            .iter()
            .map(|row| row.iter().copied().map(u8::from).collect())
            .collect();
        assert_eq!(projected_types, case.type_array, "{} data types", case.name);
        assert!(
            matrix
                .imputed
                .iter()
                .flatten()
                .all(|was_imputed| !was_imputed),
            "{} source fixture contains exclusions, not imputations",
            case.name
        );

        let projected_by_role = |role| {
            matrix
                .features
                .as_slice()
                .iter()
                .filter(|feature| feature.role == role)
                .map(|feature| (feature.column.index(), -(feature.lookback.steps() as i32)))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            projected_by_role(FeatureRole::CandidateCause),
            case.cleaned[0],
            "{} cleaned candidate causes",
            case.name
        );
        assert_eq!(
            projected_by_role(FeatureRole::TestedOutcome),
            case.cleaned[1],
            "{} cleaned outcomes",
            case.name
        );
        assert_eq!(
            projected_by_role(FeatureRole::Conditioner),
            case.cleaned[2],
            "{} cleaned conditioners",
            case.name
        );
        assert!(
            matrix
                .decisions
                .as_slice()
                .iter()
                .all(|decision| match decision {
                    SampleDecision::Retained { .. } => true,
                    SampleDecision::Excluded { reasons, .. } => !reasons.is_empty(),
                }),
            "{} every exclusion is explained",
            case.name
        );
    }
}

#[test]
fn independent_imputation_matches_the_frozen_python_oracle() {
    let expected = fixture().imputation;
    let dataset =
        NullableDataset::from_rows(expected.values, expected.valid, None, expected.coordinates)
            .expect("the imputation fixture is a valid nullable dataset");
    let maximum_gap = PositiveUsize::new(expected.max_gap).expect("fixture gap is positive");

    let linear = interpolate_bounded_linear(&dataset, maximum_gap);
    assert_matrix_close(
        "linear interpolation",
        &rows_f64(linear.dataset()),
        &expected.linear.values,
    );
    assert_eq!(
        rows_bool(
            linear.dataset().validity(),
            linear.dataset().time_count(),
            linear.dataset().column_count(),
        ),
        expected.linear.valid
    );
    assert_eq!(imputed_bitmap(&linear), expected.linear.imputed);

    let forward = carry_forward_bounded(&dataset, maximum_gap);
    assert_matrix_close(
        "bounded carry-forward",
        &rows_f64(forward.dataset()),
        &expected.forward_fill.values,
    );
    assert_eq!(
        rows_bool(
            forward.dataset().validity(),
            forward.dataset().time_count(),
            forward.dataset().column_count(),
        ),
        expected.forward_fill.valid
    );
    assert_eq!(imputed_bitmap(&forward), expected.forward_fill.imputed);

    let confirmation =
        ConfirmationId::new("structural-zero-fixture").expect("confirmation is non-empty");
    let zero = fill_confirmed_structural_zero(&dataset, &confirmation);
    assert_matrix_close(
        "structural zero",
        &rows_f64(zero.dataset()),
        &expected.structural_zero.values,
    );
    assert_eq!(
        rows_bool(
            zero.dataset().validity(),
            zero.dataset().time_count(),
            zero.dataset().column_count(),
        ),
        expected.structural_zero.valid
    );
    assert_eq!(imputed_bitmap(&zero), expected.structural_zero.imputed);

    let selected = NonEmptyVec::try_from_vec(vec![
        ColumnId::new(0, dataset.column_count()).unwrap(),
        ColumnId::new(1, dataset.column_count()).unwrap(),
    ])
    .unwrap();
    assert_eq!(
        longest_complete_interval(&dataset, &selected).unwrap(),
        CompleteIntervalOutcome::Found {
            start: expected.complete_interval[0],
            end_exclusive: expected.complete_interval[1],
        }
    );
}

#[test]
fn invalid_physical_storage_is_unobservable_and_real_zero_survives() {
    let validity = vec![vec![true], vec![false], vec![true]];
    let first = NullableDataset::from_rows(
        vec![vec![0.0], vec![f64::NAN], vec![2.0]],
        validity.clone(),
        None,
        vec![0.0, 1.0, 2.0],
    )
    .unwrap();
    let second = NullableDataset::from_rows(
        vec![vec![0.0], vec![9.9e200], vec![2.0]],
        validity,
        None,
        vec![0.0, 1.0, 2.0],
    )
    .unwrap();
    assert_eq!(first, second, "invalid physical values are normalized away");
    assert_eq!(first.values()[0], 0.0);
    assert!(first.validity()[0], "the observed zero remains valid");
}

#[test]
fn projection_preserves_time_identity_and_returns_typed_refusal() {
    let dataset = NullableDataset::from_rows(
        vec![vec![10.0], vec![f64::NAN], vec![30.0]],
        vec![vec![true], vec![false], vec![true]],
        None,
        vec![100.0, 200.0, 300.0],
    )
    .unwrap();
    let column = ColumnId::new(0, 1).unwrap();
    let request = LagProjectionRequest {
        features: NonEmptyVec::try_from_vec(vec![LaggedFeature::new(
            column,
            Lookback::new(0),
            FeatureRole::TestedOutcome,
        )])
        .unwrap(),
        reference_domain: ReferenceDomain::AllAvailable,
        history: HistoryRequirement::MinimumForFeatures,
        gap_influence: GapInfluence::DirectOnly,
        analysis_exclusions: AnalysisExclusionPolicy::Ignore,
        layout: MatrixLayout::FeaturesBySamples,
    };
    let built = match project_lagged(&dataset, &request).unwrap() {
        ProjectionOutcome::Built(matrix) => matrix,
        ProjectionOutcome::Refused { .. } => panic!("two valid samples should remain"),
    };
    assert_eq!(built.retained_references.as_slice(), &[0, 2]);
    assert_eq!(built.values, vec![vec![10.0, 30.0]]);

    let only_missing = LagProjectionRequest {
        reference_domain: ReferenceDomain::Explicit(NonEmptyVec::try_from_vec(vec![1]).unwrap()),
        ..request
    };
    match project_lagged(&dataset, &only_missing).unwrap() {
        ProjectionOutcome::Refused { decisions, reasons } => {
            assert_eq!(decisions.len(), 1);
            assert_eq!(reasons.len(), 1);
        }
        ProjectionOutcome::Built(_) => panic!("a missing-only domain must be refused"),
    }
}

#[test]
fn constructors_reject_ambiguous_or_malformed_inputs() {
    let literal_nan = NullableDataset::import_numeric_rows(
        vec![vec![1.0], vec![f64::NAN]],
        NumericNullPolicy::NoSentinel,
        None,
        vec![0.0, 1.0],
    );
    assert!(matches!(
        literal_nan,
        Err(DataPreparationError::NonFiniteObservedValue { .. })
    ));
    assert!(matches!(
        ConfirmationId::new("   "),
        Err(DataPreparationError::EmptyConfirmation)
    ));
    assert!(PositiveUsize::new(0).is_err());
    assert!(NonEmptyVec::<usize>::try_from_vec(Vec::new()).is_err());
}
