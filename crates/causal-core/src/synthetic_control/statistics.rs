//! R-style finite-sample summaries, shared by scaling and dataprep.
//! Uses the existing qd dependency, not a native-only long-double implementation.
pub(crate) fn mean(values: &[f64]) -> f64 {
    let q = qd::Quad::from_f64;
    let count = q(values.len() as f64);
    let initial = values.iter().fold(q(0.), |sum, &v| sum + q(v)) / count;
    let correction = values.iter().fold(q(0.), |sum, &v| sum + (q(v) - initial));
    (initial + correction / count).0
}
pub(crate) fn variance(values: &[f64]) -> f64 {
    let q = qd::Quad::from_f64;
    let center = mean(values);
    let ss = values.iter().fold(q(0.), |sum, &v| {
        let difference = q(v - center);
        sum + difference * difference
    });
    (ss / q((values.len() - 1) as f64)).0
}
pub(crate) fn sum(values: &[f64]) -> f64 {
    values
        .iter()
        .fold(qd::Quad::ZERO, |sum, &v| sum + qd::Quad::from_f64(v))
        .0
}
