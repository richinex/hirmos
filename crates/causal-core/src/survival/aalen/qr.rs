//! R's dqrdc2 rank rule and the coefficient branch of dqrsl.

pub(crate) struct Qr {
    a: Vec<f64>,
    aux: Vec<f64>,
    pivot: Vec<usize>,
    pub rank: usize,
    n: usize,
    columns: usize,
}

impl Qr {
    pub fn new(a: Vec<f64>, n: usize, tolerance: f64) -> Self {
        Self::rectangular(a, n, n, tolerance)
    }

    /// Same R QR operations for an overdetermined design; square callers are unchanged.
    pub fn rectangular(mut a: Vec<f64>, n: usize, columns: usize, tolerance: f64) -> Self {
        assert!(n >= columns && a.len() == n * columns);
        let mut aux: Vec<f64> = (0..columns).map(|j| norm(&a[j * n..(j + 1) * n])).collect();
        let mut original: Vec<f64> = aux
            .iter()
            .map(|v| if *v == 0.0 { 1.0 } else { *v })
            .collect();
        let mut pivot: Vec<usize> = (0..columns).collect();
        let mut rank = columns;
        for l in 0..columns {
            while l < rank && aux[l] < original[l] * tolerance {
                for row in 0..n {
                    let saved = a[row + l * n];
                    for j in l + 1..columns {
                        a[row + (j - 1) * n] = a[row + j * n];
                    }
                    a[row + (columns - 1) * n] = saved;
                }
                aux[l..].rotate_left(1);
                original[l..].rotate_left(1);
                pivot[l..].rotate_left(1);
                rank -= 1;
            }
            if l == n - 1 {
                continue;
            }
            let mut length = norm(&a[l + l * n..(l + 1) * n]);
            if length == 0.0 {
                continue;
            }
            if a[l + l * n] != 0.0 {
                length = length.copysign(a[l + l * n]);
            }
            let reciprocal = 1.0 / length;
            for row in l..n {
                a[row + l * n] *= reciprocal;
            }
            a[l + l * n] += 1.0;
            for j in l + 1..columns {
                let t = -(l..n).fold(0.0, |sum, row| a[row + l * n].mul_add(a[row + j * n], sum))
                    / a[l + l * n];
                for row in l..n {
                    a[row + j * n] = t.mul_add(a[row + l * n], a[row + j * n]);
                }
                if aux[j] == 0.0 {
                    continue;
                }
                let ratio = a[l + j * n].abs() / aux[j];
                let fraction = (-ratio).mul_add(ratio, 1.0).max(0.0);
                aux[j] = if fraction.abs() >= 1e-6 {
                    aux[j] * fraction.sqrt()
                } else {
                    norm(&a[l + 1 + j * n..(j + 1) * n])
                };
            }
            aux[l] = a[l + l * n];
            a[l + l * n] = -length;
        }
        Self {
            a,
            aux,
            pivot,
            rank,
            n,
            columns,
        }
    }

    /// Full-rank (X'X)^-1 from the existing R factor, in original column order.
    pub fn inverse_crossproduct(&self) -> Option<Vec<Vec<f64>>> {
        if self.rank != self.columns {
            return None;
        }
        let p = self.columns;
        let mut inverse = vec![vec![0.0; p]; p];
        for column in 0..p {
            for row in (0..=column).rev() {
                let diagonal = self.a[row + row * self.n];
                if diagonal == 0.0 {
                    return None;
                }
                let mut value = if row == column { 1.0 } else { 0.0 };
                for j in row + 1..=column {
                    value -= self.a[row + j * self.n] * inverse[j][column];
                }
                inverse[row][column] = value / diagonal;
            }
        }
        let mut result = vec![vec![0.0; p]; p];
        for i in 0..p {
            for j in 0..p {
                result[self.pivot[i]][self.pivot[j]] =
                    (0..p).map(|k| inverse[i][k] * inverse[j][k]).sum();
            }
        }
        Some(result)
    }

    pub fn solve(&self, rhs: &[f64]) -> Option<Vec<f64>> {
        if self.rank != self.columns || rhs.len() != self.n {
            return None;
        }
        let n = self.n;
        let mut b = rhs.to_vec();
        for j in 0..self.columns.min(n.saturating_sub(1)) {
            if self.aux[j] == 0.0 {
                continue;
            }
            let mut dot = self.aux[j] * b[j];
            for i in j + 1..n {
                dot = self.a[i + j * n].mul_add(b[i], dot);
            }
            let t = -dot / self.aux[j];
            b[j] = t.mul_add(self.aux[j], b[j]);
            for i in j + 1..n {
                b[i] = t.mul_add(self.a[i + j * n], b[i]);
            }
        }
        for j in (0..self.columns).rev() {
            if self.a[j + j * n] == 0.0 {
                return None;
            }
            b[j] /= self.a[j + j * n];
            for i in 0..j {
                b[i] = (-b[j]).mul_add(self.a[i + j * n], b[i]);
            }
        }
        let mut result = vec![0.0; self.columns];
        for j in 0..self.columns {
            result[self.pivot[j]] = b[j];
        }
        Some(result)
    }
}

fn norm(values: &[f64]) -> f64 {
    // R's linked BLAS uses the older scale/ssq DNRM2 algorithm, not
    // the three-accumulator implementation used by our LAPACK 3.12 port.
    let mut scale = 0.0_f64;
    let mut squares = 1.0_f64;
    for value in values {
        let x = value.abs();
        if x == 0.0 {
            continue;
        }
        if x > scale {
            let ratio = scale / x;
            squares = squares.mul_add(ratio * ratio, 1.0);
            scale = x;
        } else {
            let ratio = x / scale;
            squares = ratio.mul_add(ratio, squares);
        }
    }
    scale * squares.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_coefficient_solves() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../../oracle/fixtures/aalen.json")).unwrap();
        let case = fixture["cases"].as_array().unwrap().last().unwrap();
        for (call, reference) in case["solves"].as_array().unwrap().iter().enumerate() {
            if reference["rhs"][0].is_array() {
                continue;
            }
            let n = reference["rhs"].as_array().unwrap().len();
            let mut a = vec![0.0; n * n];
            for i in 0..n {
                for j in 0..n {
                    a[i + j * n] = reference["factor"][i][j].as_f64().unwrap();
                }
            }
            let qr = Qr {
                a,
                n,
                columns: n,
                rank: n,
                aux: serde_json::from_value(reference["aux"].clone()).unwrap(),
                pivot: reference["pivot"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap() as usize - 1)
                    .collect(),
            };
            let rhs: Vec<f64> = serde_json::from_value(reference["rhs"].clone()).unwrap();
            let result = qr.solve(&rhs).unwrap();
            for (i, actual) in result.iter().enumerate() {
                let expected = reference["solution"][i].as_f64().unwrap();
                assert!(
                    (actual - expected).abs() <= 1e-9 + 1e-8 * expected.abs(),
                    "call {call} [{i}]: {actual} != {expected}"
                );
            }
        }
    }

    #[test]
    fn r_qr_factors() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../../oracle/fixtures/aalen.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            for (call, reference) in case["qr"].as_array().unwrap().iter().enumerate() {
                let rows = reference["input"].as_array().unwrap();
                let n = rows.len();
                let mut a = vec![0.0; n * n];
                for i in 0..n {
                    for j in 0..n {
                        a[i + j * n] = rows[i][j].as_f64().unwrap();
                    }
                }
                let qr = Qr::new(a, n, 1e-7);
                assert_eq!(
                    qr.rank,
                    reference["rank"].as_u64().unwrap() as usize,
                    "{} call {call}",
                    case["name"]
                );
                for i in 0..n {
                    for j in 0..n {
                        let expected = reference["factor"][i][j].as_f64().unwrap();
                        if case["name"] == "veteran" && call == 74 {
                            assert_eq!(qr.a[i + j * n], expected, "tail QR [{i},{j}]");
                        }
                        assert!(
                            (qr.a[i + j * n] - expected).abs() < 1e-11 * expected.abs().max(1.0),
                            "{} call {call} [{i},{j}]: {} != {expected}",
                            case["name"],
                            qr.a[i + j * n]
                        );
                    }
                }
            }
        }
    }
}
