use super::model::{
    CellRef, ColumnId, CompleteIntervalOutcome, CompleteIntervalRefusal, ConfirmationId,
    DataPreparationError, ImputationOutcome, MissingRun, NonEmptyVec, NullableDataset,
    PositiveUsize,
};

fn missing_runs(dataset: &NullableDataset) -> Vec<MissingRun> {
    let mut runs = Vec::new();
    for column_index in 0..dataset.column_count {
        let column = ColumnId::new(column_index, dataset.column_count)
            .expect("the loop only visits in-range columns");
        let mut cursor = 0;
        while cursor < dataset.time_count {
            if dataset.is_valid(cursor, column) {
                cursor += 1;
                continue;
            }
            let start = cursor;
            while cursor < dataset.time_count && !dataset.is_valid(cursor, column) {
                cursor += 1;
            }
            runs.push(MissingRun {
                column,
                start,
                end_exclusive: cursor,
            });
        }
    }
    runs
}

fn outcome(dataset: NullableDataset, imputed: Vec<CellRef>) -> ImputationOutcome {
    let unresolved_runs = missing_runs(&dataset);
    if imputed.is_empty() {
        ImputationOutcome::NoChange {
            dataset,
            unresolved_runs,
        }
    } else {
        ImputationOutcome::Changed {
            dataset,
            imputed_cells: NonEmptyVec::try_from_vec(imputed)
                .expect("the changed branch contains at least one imputed cell"),
            unresolved_runs,
        }
    }
}

pub fn interpolate_bounded_linear(
    dataset: &NullableDataset,
    maximum_gap: PositiveUsize,
) -> ImputationOutcome {
    let mut result = dataset.clone();
    let mut imputed = Vec::new();
    for column_index in 0..dataset.column_count {
        let column = ColumnId::new(column_index, dataset.column_count)
            .expect("the loop only visits in-range columns");
        let mut cursor = 0;
        while cursor < dataset.time_count {
            if result.is_valid(cursor, column) {
                cursor += 1;
                continue;
            }
            let start = cursor;
            while cursor < dataset.time_count && !result.is_valid(cursor, column) {
                cursor += 1;
            }
            let end_exclusive = cursor;
            if start == 0
                || end_exclusive == dataset.time_count
                || end_exclusive - start > maximum_gap.get()
            {
                continue;
            }
            let left = start - 1;
            let right = end_exclusive;
            let left_value = result.value(left, column);
            let right_value = result.value(right, column);
            let coordinate_width = result.coordinates[right] - result.coordinates[left];
            for time in start..end_exclusive {
                let weight =
                    (result.coordinates[time] - result.coordinates[left]) / coordinate_width;
                let index = time * result.column_count + column.index();
                result.values[index] = left_value + weight * (right_value - left_value);
                result.validity[index] = true;
                result.imputed[index] = true;
                imputed.push(CellRef { time, column });
            }
        }
    }
    outcome(result, imputed)
}

pub fn carry_forward_bounded(
    dataset: &NullableDataset,
    maximum_gap: PositiveUsize,
) -> ImputationOutcome {
    let mut result = dataset.clone();
    let mut imputed = Vec::new();
    for column_index in 0..dataset.column_count {
        let column = ColumnId::new(column_index, dataset.column_count)
            .expect("the loop only visits in-range columns");
        let mut cursor = 0;
        while cursor < dataset.time_count {
            if result.is_valid(cursor, column) {
                cursor += 1;
                continue;
            }
            let start = cursor;
            while cursor < dataset.time_count && !result.is_valid(cursor, column) {
                cursor += 1;
            }
            let end_exclusive = cursor;
            if start == 0 || end_exclusive - start > maximum_gap.get() {
                continue;
            }
            let previous = result.value(start - 1, column);
            for time in start..end_exclusive {
                let index = time * result.column_count + column.index();
                result.values[index] = previous;
                result.validity[index] = true;
                result.imputed[index] = true;
                imputed.push(CellRef { time, column });
            }
        }
    }
    outcome(result, imputed)
}

pub fn fill_confirmed_structural_zero(
    dataset: &NullableDataset,
    _confirmation: &ConfirmationId,
) -> ImputationOutcome {
    let mut result = dataset.clone();
    let mut imputed = Vec::new();
    for time in 0..dataset.time_count {
        for column_index in 0..dataset.column_count {
            let column = ColumnId::new(column_index, dataset.column_count)
                .expect("the loop only visits in-range columns");
            let index = time * dataset.column_count + column_index;
            if !result.validity[index] {
                result.values[index] = 0.0;
                result.validity[index] = true;
                result.imputed[index] = true;
                imputed.push(CellRef { time, column });
            }
        }
    }
    outcome(result, imputed)
}

pub fn longest_complete_interval(
    dataset: &NullableDataset,
    columns: &NonEmptyVec<ColumnId>,
) -> Result<CompleteIntervalOutcome, DataPreparationError> {
    for column in columns.as_slice() {
        if column.index() >= dataset.column_count {
            return Err(DataPreparationError::InvalidColumn {
                column: column.index(),
                column_count: dataset.column_count,
            });
        }
    }
    let complete = |time: usize| {
        columns
            .as_slice()
            .iter()
            .all(|column| dataset.is_valid(time, *column))
    };
    let mut best: Option<(usize, usize)> = None;
    let mut cursor = 0;
    while cursor < dataset.time_count {
        if !complete(cursor) {
            cursor += 1;
            continue;
        }
        let start = cursor;
        while cursor < dataset.time_count && complete(cursor) {
            cursor += 1;
        }
        let candidate = (start, cursor);
        if best.is_none_or(|current| candidate.1 - candidate.0 > current.1 - current.0) {
            best = Some(candidate);
        }
    }
    Ok(match best {
        Some((start, end_exclusive)) => CompleteIntervalOutcome::Found {
            start,
            end_exclusive,
        },
        None => CompleteIntervalOutcome::Refused(CompleteIntervalRefusal::NoCompletePoint),
    })
}
