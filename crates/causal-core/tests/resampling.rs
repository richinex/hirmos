use hirmos_causal_core::{
    pandas_resample_daily, IncompleteBins, ResampleFrequency, ResamplingAggregation,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    pandas_version: String,
    pandas_revision: String,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    target: String,
    incomplete: String,
    timestamps_ms: Vec<i64>,
    columns: Vec<String>,
    aggregations: Vec<String>,
    values: Vec<f64>,
    expected: Expected,
}

#[derive(Deserialize)]
struct Expected {
    timestamps_ms: Vec<i64>,
    values: Vec<f64>,
    source_rows: usize,
    output_rows: usize,
    incomplete_bins: usize,
    bins_dropped: usize,
    source_rows_dropped: usize,
}

fn aggregation(name: &str) -> ResamplingAggregation {
    match name {
        "mean" => ResamplingAggregation::Mean,
        "sum" => ResamplingAggregation::Sum,
        "median" => ResamplingAggregation::Median,
        "min" => ResamplingAggregation::Minimum,
        "max" => ResamplingAggregation::Maximum,
        "first" => ResamplingAggregation::First,
        "last" => ResamplingAggregation::Last,
        other => panic!("unknown aggregation {other}"),
    }
}

#[test]
fn daily_downsampling_matches_pandas_2_3_3() {
    let fixture: Fixture = serde_json::from_str(include_str!("../oracle/fixtures/resampling.json"))
        .expect("valid pandas fixture");
    assert_eq!(fixture.pandas_version, "2.3.3");
    assert_eq!(
        fixture.pandas_revision,
        "9c8bc3e55188c8aff37207a74f1dd144980b8874"
    );

    for case in fixture.cases {
        let frequency = match case.target.as_str() {
            "weekly" => ResampleFrequency::WeeklyMonday,
            "monthly" => ResampleFrequency::MonthStart,
            other => panic!("unknown target {other}"),
        };
        let incomplete = match case.incomplete.as_str() {
            "keep" => IncompleteBins::Keep,
            "drop" => IncompleteBins::Drop,
            other => panic!("unknown incomplete-bin policy {other}"),
        };
        let aggregations: Vec<_> = case
            .aggregations
            .iter()
            .map(|name| aggregation(name))
            .collect();
        let actual = pandas_resample_daily(
            &case.values,
            case.timestamps_ms.len(),
            case.columns.len(),
            &case.timestamps_ms,
            &aggregations,
            &[],
            frequency,
            incomplete,
        )
        .expect("supported fixture resamples");

        assert_eq!(actual.timestamps_ms, case.expected.timestamps_ms);
        assert_eq!(actual.source_rows, case.expected.source_rows);
        assert_eq!(actual.output_rows, case.expected.output_rows);
        assert_eq!(actual.incomplete_bins, case.expected.incomplete_bins);
        assert_eq!(actual.bins_dropped, case.expected.bins_dropped);
        assert_eq!(
            actual.source_rows_dropped,
            case.expected.source_rows_dropped
        );
        assert_eq!(actual.values.len(), case.expected.values.len());
        for (index, (&got, &want)) in actual.values.iter().zip(&case.expected.values).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{} {} value {index}: got {got:?}, want {want:?}",
                case.target,
                case.incomplete
            );
        }
    }
}
