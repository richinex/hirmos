//! Dense column-major matrix checks and the design helpers every kernel façade shares.

use super::*;

pub(crate) fn validate_dense_matrix(
    label: &str,
    values: &[f64],
    rows: usize,
    columns: usize,
) -> Result<(), String> {
    let expected = rows
        .checked_mul(columns)
        .ok_or_else(|| format!("{label} matrix dimensions overflowed"))?;
    if values.len() != expected {
        return Err(format!(
            "{label} received {} values for a {rows} by {columns} matrix",
            values.len()
        ));
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err(format!(
            "{label} requires finite dense values; resolve missingness first"
        ));
    }
    Ok(())
}

pub(crate) fn time_series_from_column_major(
    values: &[f64],
    rows: usize,
    columns: usize,
) -> TimeSeries {
    let mut row_major = vec![0.0; rows * columns];
    for column in 0..columns {
        for row in 0..rows {
            row_major[row * columns + column] = values[column * rows + row];
        }
    }
    TimeSeries {
        values: row_major,
        t: rows,
        n: columns,
    }
}

pub(crate) fn validate_stationarity_values(values: &[f64]) -> Result<(), String> {
    if values.len() < MIN_STATIONARITY_OBSERVATIONS {
        return Err(format!(
            "stationarity battery requires at least {MIN_STATIONARITY_OBSERVATIONS} observations"
        ));
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err("stationarity battery requires finite dense values".to_owned());
    }
    let first = values[0];
    if values.iter().all(|value| *value == first) {
        return Err("stationarity battery is undefined for a constant series".to_owned());
    }
    Ok(())
}

pub(crate) fn design_columns(
    label: &str,
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
) -> Result<(DMatrix<f64>, DMatrix<f64>, Vec<f64>), String> {
    validate_dense_matrix(label, values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err(format!("{label} columns must index the numeric matrix"));
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err(format!(
            "{label} needs distinct treatment, outcome, and adjustment columns"
        ));
    }
    let parameters = 2 + adjustment.len();
    if rows <= parameters + 1 {
        return Err(format!(
            "{label} needs more than {} observations for {parameters} parameters",
            parameters + 1
        ));
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let design = DMatrix::<f64>::from_fn(rows, parameters, |row, column| match column {
        0 => 1.0,
        1 => data[(row, treatment)],
        _ => data[(row, adjustment[column - 2])],
    });
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    Ok((data, design, y))
}

/// Kahn's order over node positions; `None` when the edges hold a cycle.
pub(crate) fn topological_order(count: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut indegree = vec![0usize; count];
    for &(_, effect) in edges {
        indegree[effect] += 1;
    }
    let mut ready: Vec<usize> = (0..count).filter(|&node| indegree[node] == 0).collect();
    ready.reverse();
    let mut order = Vec::with_capacity(count);
    while let Some(node) = ready.pop() {
        order.push(node);
        let mut freed: Vec<usize> = Vec::new();
        for &(cause, effect) in edges {
            if cause == node {
                indegree[effect] -= 1;
                if indegree[effect] == 0 {
                    freed.push(effect);
                }
            }
        }
        freed.sort_unstable_by(|left, right| right.cmp(left));
        ready.extend(freed);
    }
    (order.len() == count).then_some(order)
}
