use hirmos_causal_core::parcorr::{CutOff, Node};
use hirmos_causal_core::{
    construct_array_tracked, forward_fill, linear_interpolate, longest_complete_interval,
    structural_zero, ConstructOptions, MaskType, PreprocessingError, SampleExclusionReason,
    TigramiteFrame,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    tigramite: Vec<TigramiteCase>,
    imputation: ImputationFixture,
}

#[derive(Deserialize)]
struct TigramiteCase {
    name: String,
    values: Vec<Vec<f64>>,
    missing_flag: Option<f64>,
    analysis_mask: Option<Vec<Vec<bool>>>,
    data_type: Vec<Vec<bool>>,
    x: Vec<Node>,
    y: Vec<Node>,
    z: Vec<Node>,
    extra_z: Vec<Node>,
    tau_max: usize,
    cutoff: String,
    mask_type: Option<String>,
    propagate: bool,
    reference_points: Vec<usize>,
    array: Vec<Vec<f64>>,
    xyz: Vec<u8>,
    cleaned: Vec<Vec<Node>>,
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

fn cutoff(value: &str) -> CutOff {
    match value {
        "2xtau_max" => CutOff::TwoTauMax,
        "tau_max" => CutOff::TauMax,
        "max_lag" => CutOff::MaxLag,
        "max_lag_or_tau_max" => CutOff::MaxLagOrTauMax,
        "2xtau_max_future" => CutOff::TwoTauMaxFuture,
        other => panic!("unknown cutoff {other}"),
    }
}

fn mask_type(value: Option<&str>) -> MaskType {
    match value {
        None => MaskType::NONE,
        Some("x") => MaskType::X,
        Some("y") => MaskType::Y,
        Some("z") => MaskType::Z,
        Some("xy") => MaskType::XY,
        Some("xz") => MaskType::XZ,
        Some("yz") => MaskType::YZ,
        Some("xyz") => MaskType::XYZ,
        Some(other) => panic!("unknown mask type {other}"),
    }
}

fn assert_matrix_close(label: &str, got: &[Vec<f64>], want: &[Vec<f64>]) {
    assert_eq!(got.len(), want.len(), "{label}: rows");
    for (row, (got_row, want_row)) in got.iter().zip(want).enumerate() {
        assert_eq!(got_row.len(), want_row.len(), "{label}: row {row} length");
        for (column, (&got_value, &want_value)) in got_row.iter().zip(want_row).enumerate() {
            assert!(
                (got_value - want_value).abs() <= 1e-14,
                "{label}[{row},{column}]: got {got_value}, want {want_value}"
            );
        }
    }
}

fn rows_f64(flat: &[f64], t: usize, n: usize) -> Vec<Vec<f64>> {
    (0..t)
        .map(|time| flat[time * n..(time + 1) * n].to_vec())
        .collect()
}

fn rows_bool(flat: &[bool], t: usize, n: usize) -> Vec<Vec<bool>> {
    (0..t)
        .map(|time| flat[time * n..(time + 1) * n].to_vec())
        .collect()
}

#[test]
fn nullable_construct_array_matches_tigramite_for_all_policies() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../oracle/fixtures/missing_preprocessing.json"
    ))
    .unwrap();
    for case in fixture.tigramite {
        let frame =
            TigramiteFrame::from_missing_flag(case.values, case.missing_flag, case.analysis_mask)
                .unwrap()
                .with_data_type(case.data_type)
                .unwrap();
        let result = construct_array_tracked(
            &frame,
            &case.x,
            &case.y,
            &case.z,
            &case.extra_z,
            case.tau_max,
            ConstructOptions {
                cut_off: cutoff(&case.cutoff),
                remove_missing_upto_maxlag: case.propagate,
                mask_type: mask_type(case.mask_type.as_deref()),
                reference_points: Some(&case.reference_points),
                ..ConstructOptions::default()
            },
        )
        .unwrap_or_else(|error| panic!("{}: {error}", case.name));

        assert_eq!(
            result.retained_reference_points, case.retained_reference_points,
            "{} retained reference points",
            case.name
        );
        assert_matrix_close(&case.name, &result.values, &case.array);
        assert_eq!(result.xyz, case.xyz, "{} xyz", case.name);
        assert_eq!(result.cleaned.x, case.cleaned[0], "{} cleaned X", case.name);
        assert_eq!(result.cleaned.y, case.cleaned[1], "{} cleaned Y", case.name);
        assert_eq!(result.cleaned.z, case.cleaned[2], "{} cleaned Z", case.name);
        let got_types: Vec<Vec<u8>> = result
            .data_type
            .unwrap()
            .into_iter()
            .map(|row| row.into_iter().map(u8::from).collect())
            .collect();
        assert_eq!(got_types, case.type_array, "{} data type", case.name);

        let direct_missing: Vec<usize> = result
            .exclusions
            .iter()
            .filter(|entry| {
                entry
                    .reasons
                    .contains(&SampleExclusionReason::MissingSelectedValue)
            })
            .map(|entry| entry.reference_point)
            .collect();
        if case.missing_flag.is_some() {
            assert!(!direct_missing.is_empty(), "{} missing ledger", case.name);
        }
    }
}

#[test]
fn imputation_matches_python_oracle_and_preserves_provenance() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../oracle/fixtures/missing_preprocessing.json"
    ))
    .unwrap();
    let expected = fixture.imputation;
    let frame = TigramiteFrame::from_validity(expected.values, expected.valid, None).unwrap();
    let t = frame.data.t;
    let n = frame.data.n;

    let linear = linear_interpolate(&frame, expected.max_gap, Some(&expected.coordinates)).unwrap();
    assert_matrix_close(
        "linear",
        &rows_f64(&linear.frame.data.values, t, n),
        &expected.linear.values,
    );
    assert_eq!(
        rows_bool(&linear.frame.validity, t, n),
        expected.linear.valid
    );
    assert_eq!(rows_bool(&linear.imputed, t, n), expected.linear.imputed);

    let forward = forward_fill(&frame, expected.max_gap).unwrap();
    assert_matrix_close(
        "forward fill",
        &rows_f64(&forward.frame.data.values, t, n),
        &expected.forward_fill.values,
    );
    assert_eq!(
        rows_bool(&forward.frame.validity, t, n),
        expected.forward_fill.valid
    );
    assert_eq!(
        rows_bool(&forward.imputed, t, n),
        expected.forward_fill.imputed
    );

    let zero = structural_zero(&frame);
    assert_matrix_close(
        "structural zero",
        &rows_f64(&zero.frame.data.values, t, n),
        &expected.structural_zero.values,
    );
    assert_eq!(
        rows_bool(&zero.frame.validity, t, n),
        expected.structural_zero.valid
    );
    assert_eq!(
        rows_bool(&zero.imputed, t, n),
        expected.structural_zero.imputed
    );

    let interval = longest_complete_interval(&frame.validity, t, n, &[0, 1])
        .unwrap()
        .unwrap();
    assert_eq!(
        [interval.0, interval.1],
        expected.complete_interval.as_slice()
    );
}

#[test]
fn safe_constructor_and_bounded_gap_rules_are_explicit() {
    let literal_nan =
        TigramiteFrame::from_missing_flag(vec![vec![1.0], vec![f64::NAN]], Some(-999.0), None);
    assert!(matches!(
        literal_nan,
        Err(PreprocessingError::IncomingNan { .. })
    ));

    let frame = TigramiteFrame::from_validity(
        vec![
            vec![f64::NAN, 0.0],
            vec![1.0, 1.0],
            vec![2.0, 2.0],
            vec![3.0, 3.0],
            vec![4.0, 4.0],
        ],
        vec![
            vec![false, true],
            vec![true, true],
            vec![false, true],
            vec![false, true],
            vec![false, true],
        ],
        None,
    )
    .unwrap();
    assert_eq!(
        frame.data.values[0], 0.0,
        "invalid Arrow storage is normalized"
    );
    assert!(frame.validity[1], "a real zero remains valid");

    let linear = linear_interpolate(&frame, 2, None).unwrap();
    assert!(
        !linear.frame.validity[0],
        "leading gaps are not extrapolated"
    );
    assert!(
        !linear.frame.validity[2 * frame.data.n],
        "a run longer than max_gap remains unresolved"
    );
    let forward = forward_fill(&frame, 2).unwrap();
    assert!(
        !forward.frame.validity[0],
        "forward fill cannot invent a leading state"
    );
    assert!(
        !forward.frame.validity[2 * frame.data.n],
        "forward fill refuses an entire over-budget run"
    );
}
