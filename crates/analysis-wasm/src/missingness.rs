//! Browser serialization adapter for the method-neutral data-preparation core.

use hirmos_causal_core::data_preparation::{
    carry_forward_bounded, fill_confirmed_structural_zero, interpolate_bounded_linear,
    longest_complete_interval, ColumnId, CompleteIntervalOutcome, ConfirmationId,
    ImputationOutcome, NonEmptyVec, NullableDataset, PositiveUsize,
};
use serde::Serialize;

use super::AnalysisResult;

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum MissingnessResolution {
    CompleteInterval,
    Imputation {
        method: ImputationMethod,
        max_gap: usize,
        /// The recorded confirmation that missing means zero; required for structural zero.
        confirmation: Option<String>,
    },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum ImputationMethod {
    LinearInterior,
    ForwardFill,
    StructuralZero,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum MissingnessExecution {
    Completed {
        /// Dense column-major values of the retained rows.
        values: Vec<f64>,
        /// Rows retained, as a half-open window over the source rows.
        window_start: usize,
        window_end: usize,
        imputed_cells: Vec<(usize, usize)>,
    },
    Refused {
        reasons: Vec<MissingnessRefusal>,
        imputed_cells: Vec<(usize, usize)>,
        unresolved_runs: Vec<UnresolvedRun>,
        remaining_missing: usize,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum MissingnessRefusal {
    NoCompleteInterval,
    UnresolvedCells { count: usize },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UnresolvedRun {
    column: usize,
    start: usize,
    end: usize,
}

pub(super) fn resolve_missingness(
    values: &[f64],
    rows: usize,
    columns: usize,
    validity: &[u8],
    resolution: MissingnessResolution,
) -> Result<AnalysisResult, String> {
    let expected = rows
        .checked_mul(columns)
        .ok_or_else(|| "missingness matrix dimensions overflowed".to_owned())?;
    if values.len() != expected || validity.len() != expected || rows == 0 || columns == 0 {
        return Err(format!(
            "missingness received {} values and {} validity flags for a {rows} by {columns} matrix",
            values.len(),
            validity.len()
        ));
    }

    let mut row_values = vec![vec![0.0; columns]; rows];
    let mut row_validity = vec![vec![false; columns]; rows];
    for column in 0..columns {
        for row in 0..rows {
            let index = column * rows + row;
            let valid = validity[index] != 0;
            if valid && !values[index].is_finite() {
                return Err(format!(
                    "observed cell at row {row}, column {column} is not finite"
                ));
            }
            row_values[row][column] = if valid { values[index] } else { 0.0 };
            row_validity[row][column] = valid;
        }
    }

    let coordinates: Vec<f64> = (0..rows).map(|row| row as f64).collect();
    let dataset = NullableDataset::from_rows(row_values, row_validity, None, coordinates)
        .map_err(|error| format!("missingness dataset refused: {error:?}"))?;
    let dense = |set: &NullableDataset, start: usize, end: usize| -> Vec<f64> {
        let mut output = Vec::with_capacity((end - start) * columns);
        for column in 0..columns {
            let id = ColumnId::new(column, columns).expect("column in range");
            for row in start..end {
                output.push(set.value(row, id));
            }
        }
        output
    };

    match resolution.clone() {
        MissingnessResolution::CompleteInterval => {
            let all: Vec<ColumnId> = (0..columns)
                .map(|column| ColumnId::new(column, columns).expect("column in range"))
                .collect();
            let outcome = longest_complete_interval(
                &dataset,
                &NonEmptyVec::try_from_vec(all).expect("at least one column"),
            )
            .map_err(|error| format!("complete interval refused: {error:?}"))?;
            match outcome {
                CompleteIntervalOutcome::Found {
                    start,
                    end_exclusive,
                } => Ok(AnalysisResult::MissingnessResolved {
                    rows,
                    columns,
                    resolution,
                    outcome: MissingnessExecution::Completed {
                        values: dense(&dataset, start, end_exclusive),
                        window_start: start,
                        window_end: end_exclusive,
                        imputed_cells: Vec::new(),
                    },
                }),
                CompleteIntervalOutcome::Refused(_) => Ok(AnalysisResult::MissingnessResolved {
                    rows,
                    columns,
                    resolution,
                    outcome: MissingnessExecution::Refused {
                        reasons: vec![MissingnessRefusal::NoCompleteInterval],
                        imputed_cells: Vec::new(),
                        unresolved_runs: Vec::new(),
                        remaining_missing: expected,
                    },
                }),
            }
        }
        MissingnessResolution::Imputation {
            method,
            max_gap,
            confirmation,
        } => {
            let outcome: ImputationOutcome = match method {
                ImputationMethod::LinearInterior => interpolate_bounded_linear(
                    &dataset,
                    PositiveUsize::new(max_gap)
                        .map_err(|error| format!("max gap refused: {error:?}"))?,
                ),
                ImputationMethod::ForwardFill => carry_forward_bounded(
                    &dataset,
                    PositiveUsize::new(max_gap)
                        .map_err(|error| format!("max gap refused: {error:?}"))?,
                ),
                ImputationMethod::StructuralZero => {
                    let confirmed = confirmation.ok_or_else(|| {
                        "structural zero needs a recorded confirmation that missing means zero"
                            .to_owned()
                    })?;
                    let id = ConfirmationId::new(confirmed)
                        .map_err(|error| format!("confirmation refused: {error:?}"))?;
                    fill_confirmed_structural_zero(&dataset, &id)
                }
            };
            let resolved = outcome.dataset();
            let remaining_missing = resolved.validity().iter().filter(|valid| !**valid).count();
            let imputed_cells = outcome
                .imputed_cells()
                .iter()
                .map(|cell| (cell.time, cell.column.index()))
                .collect();
            let outcome = if remaining_missing == 0 {
                MissingnessExecution::Completed {
                    values: dense(resolved, 0, rows),
                    window_start: 0,
                    window_end: rows,
                    imputed_cells,
                }
            } else {
                MissingnessExecution::Refused {
                    reasons: vec![MissingnessRefusal::UnresolvedCells {
                        count: remaining_missing,
                    }],
                    imputed_cells,
                    unresolved_runs: outcome
                        .unresolved_runs()
                        .iter()
                        .map(|run| UnresolvedRun {
                            column: run.column.index(),
                            start: run.start,
                            end: run.end_exclusive,
                        })
                        .collect(),
                    remaining_missing,
                }
            };
            Ok(AnalysisResult::MissingnessResolved {
                rows,
                columns,
                resolution,
                outcome,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missingness_resolutions_impute_within_the_gap_and_find_the_complete_window() {
        // Two columns, six rows; column 0 has an interior gap of two rows, column 1 a leading gap.
        let values = vec![
            1.0,
            f64::NAN,
            f64::NAN,
            4.0,
            5.0,
            6.0,
            f64::NAN,
            2.0,
            2.0,
            2.0,
            2.0,
            2.0,
        ];
        let validity = vec![1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1];
        let linear = resolve_missingness(
            &values,
            6,
            2,
            &validity,
            MissingnessResolution::Imputation {
                method: ImputationMethod::LinearInterior,
                max_gap: 2,
                confirmation: None,
            },
        )
        .unwrap();
        let value: serde_json::Value = serde_json::to_value(linear).unwrap();
        assert_eq!(value["kind"], "missingnessResolved");
        assert_eq!(value["outcome"]["kind"], "refused");
        assert_eq!(
            value["outcome"]["imputedCells"].as_array().unwrap().len(),
            2
        );
        assert_eq!(value["outcome"]["remainingMissing"], 1);
        assert_eq!(value["outcome"]["reasons"][0]["kind"], "unresolvedCells");
        assert_eq!(value["outcome"]["unresolvedRuns"][0]["column"], 1);

        let window = resolve_missingness(
            &values,
            6,
            2,
            &validity,
            MissingnessResolution::CompleteInterval,
        )
        .unwrap();
        let value: serde_json::Value = serde_json::to_value(window).unwrap();
        assert_eq!(value["outcome"]["kind"], "completed");
        assert_eq!(value["outcome"]["windowStart"], 3);
        assert_eq!(value["outcome"]["windowEnd"], 6);
        assert_eq!(value["outcome"]["values"].as_array().unwrap().len(), 6);
        assert_eq!(value["outcome"]["values"][0], 4.0);

        let zero = resolve_missingness(
            &values,
            6,
            2,
            &validity,
            MissingnessResolution::Imputation {
                method: ImputationMethod::StructuralZero,
                max_gap: 1,
                confirmation: Some("absence means zero".to_owned()),
            },
        )
        .unwrap();
        let value: serde_json::Value = serde_json::to_value(zero).unwrap();
        assert_eq!(value["outcome"]["kind"], "completed");
        assert_eq!(
            value["outcome"]["imputedCells"].as_array().unwrap().len(),
            3
        );
        assert!(resolve_missingness(
            &values,
            6,
            2,
            &validity,
            MissingnessResolution::Imputation {
                method: ImputationMethod::StructuralZero,
                max_gap: 1,
                confirmation: None
            }
        )
        .is_err());
    }
}
