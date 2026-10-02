use crate::honest_did::Error;
use nalgebra::DMatrix;

#[derive(Clone, Copy, Debug)]
pub struct Periods {
    pre: usize,
    post: usize,
}

impl Periods {
    pub fn new(pre: usize, post: usize) -> Result<Self, Error> {
        if pre == 0
            || post == 0
            || pre
                .checked_add(post)
                .and_then(|v| v.checked_add(1))
                .is_none()
        {
            return Err(Error::InvalidInput(
                "At least one pre-treatment and one post-treatment coefficient are required.",
            ));
        }
        Ok(Self { pre, post })
    }
    pub fn pre(self) -> usize {
        self.pre
    }
    pub fn post(self) -> usize {
        self.post
    }
    pub fn total(self) -> usize {
        self.pre + self.post
    }
}

/// HonestDiD::.create_A_SD. The omitted reference coefficient is fixed at zero.
pub fn smoothness_constraints(periods: Periods, post_only: bool) -> DMatrix<f64> {
    let n = periods.total();
    let rows: Vec<usize> = (0..n - 1)
        .filter(|r| !post_only || r + 2 > periods.pre)
        .collect();
    let mut a = DMatrix::zeros(2 * rows.len(), n);
    for (i, &row) in rows.iter().enumerate() {
        for (offset, coefficient) in [1.0, -2.0, 1.0].into_iter().enumerate() {
            let column = row + offset;
            if column == periods.pre {
                continue;
            }
            let retained = if column > periods.pre {
                column - 1
            } else {
                column
            };
            a[(i, retained)] = coefficient;
            a[(i + rows.len(), retained)] = -coefficient;
        }
    }
    a
}

/// HonestDiD::.create_A_RM, with a zero-based index for the candidate maximum
/// pre-treatment first difference. The family is the union over index and sign.
pub fn relative_magnitude_constraints(
    periods: Periods,
    bound: f64,
    max_difference: usize,
    positive: bool,
) -> Result<DMatrix<f64>, Error> {
    if !bound.is_finite() || bound < 0.0 || max_difference >= periods.pre {
        return Err(Error::InvalidInput("The relative-magnitude bound must be nonnegative and the maximum difference must be a pre-treatment difference."));
    }
    let n = periods.total();
    let sign = if positive { 1.0 } else { -1.0 };
    let mut rows = Vec::new();
    for orientation in [1.0, -1.0] {
        for r in 0..n {
            let mut row = vec![0.0; n + 1];
            row[r] -= orientation;
            row[r + 1] += orientation;
            let scale = if r < periods.pre { 1.0 } else { bound };
            row[max_difference] += scale * sign;
            row[max_difference + 1] -= scale * sign;
            // R removes zero rows before dropping the reference column.
            if row.iter().map(|v| v * v).sum::<f64>() > 1e-10 {
                row.remove(periods.pre);
                rows.extend(row);
            }
        }
    }
    Ok(DMatrix::from_row_slice(rows.len() / n, n, &rows))
}
