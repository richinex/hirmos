//! Opt-in workload tracing; absent from production builds.
use super::*;
use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

static RECORD: AtomicBool = AtomicBool::new(false);
type Shape = (bool, bool, usize, usize, usize);
static SHAPES: Mutex<BTreeMap<Shape, usize>> = Mutex::new(BTreeMap::new());

#[allow(clippy::too_many_arguments)]
fn reference_product(
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
) {
    for j in 0..n {
        for i in 0..m {
            let mut sum = 0.0;
            for l in 0..k {
                sum += a[l + i * lda] * b[l + j * ldb];
            }
            c[i + j * ldc] = if beta == 0.0 {
                alpha * sum
            } else {
                alpha * sum + beta * c[i + j * ldc]
            };
        }
    }
}
#[test]
#[ignore = "compares DGEMM on shapes recorded from KCI"]
fn benchmark_transposed_product() {
    use std::{hint::black_box, time::Instant};
    for (m, n, k) in [(7, 5, 9), (1200, 32, 1167), (600, 32, 567), (32, 1200, 32)] {
        let (lda, ldb, ldc) = (k + 3, k + 2, m + 1);
        let a: Vec<_> = (0..lda * m).map(|i| (i as f64 * 0.31).sin()).collect();
        let b: Vec<_> = (0..ldb * n).map(|i| (i as f64 * 0.17).cos()).collect();
        for beta in [0.0, 1.0, -0.5] {
            let mut original = vec![0.37; ldc * n];
            let mut candidate = original.clone();
            reference_product(m, n, k, 0.73, &a, lda, &b, ldb, beta, &mut original, ldc);
            dgemm(
                Transpose::Transpose,
                Transpose::None,
                m,
                n,
                k,
                0.73,
                &a,
                lda,
                &b,
                ldb,
                beta,
                &mut candidate,
                ldc,
            )
            .unwrap();
            assert!(original
                .iter()
                .zip(&candidate)
                .all(|(a, b)| a.to_bits() == b.to_bits()));
        }
        let mut c = vec![0.37; ldc * n];
        for round in 0..6 {
            for candidate in if round % 2 == 0 {
                [false, true]
            } else {
                [true, false]
            } {
                let start = Instant::now();
                if candidate {
                    dgemm(
                        Transpose::Transpose,
                        Transpose::None,
                        m,
                        n,
                        k,
                        0.73,
                        black_box(&a),
                        lda,
                        &b,
                        ldb,
                        0.0,
                        &mut c,
                        ldc,
                    )
                    .unwrap()
                } else {
                    reference_product(m, n, k, 0.73, black_box(&a), lda, &b, ldb, 0.0, &mut c, ldc)
                }
                black_box(&c);
                if round > 0 {
                    println!(
                        "{}",
                        serde_json::json!({"m":m,"n":n,"k":k,"candidate":candidate,"seconds":start.elapsed().as_secs_f64()})
                    );
                }
            }
        }
    }
}

#[test]
fn transposed_product_preserves_reference_bits() {
    for m in [1, 2, 3, 4, 5, 7, 8, 9, 31, 32, 33] {
        for k in [0, 1, 2, 7, 32, 129] {
            let n = 5;
            let (lda, ldb, ldc) = (k + 3, k + 2, m + 1);
            let a: Vec<_> = (0..if k == 0 { 0 } else { (m - 1) * lda + k })
                .map(|i| (i as f64 * 0.31).sin())
                .collect();
            let b: Vec<_> = (0..if k == 0 { 0 } else { (n - 1) * ldb + k })
                .map(|i| (i as f64 * 0.17).cos())
                .collect();
            for alpha in [0.73, 1.0, -1.0] {
                for beta in [0.0, -0.0, 1.0, -0.5] {
                    let mut expected = vec![0.37; ldc * n];
                    let mut actual = expected.clone();
                    reference_product(m, n, k, alpha, &a, lda, &b, ldb, beta, &mut expected, ldc);
                    dgemm(
                        Transpose::Transpose,
                        Transpose::None,
                        m,
                        n,
                        k,
                        alpha,
                        &a,
                        lda,
                        &b,
                        ldb,
                        beta,
                        &mut actual,
                        ldc,
                    )
                    .unwrap();
                    assert!(
                        expected
                            .iter()
                            .zip(&actual)
                            .all(|(a, b)| a.to_bits() == b.to_bits()),
                        "m={m} k={k} alpha={alpha} beta={beta}"
                    );
                }
            }
        }
    }
}

#[test]
fn transposed_product_preserves_cancellation_and_signed_zero() {
    let values = [0.0, -0.0, 1e16, 1.0, -1e16, 1e-200, -1e-200, 1e200, -1e200];
    let (m, n, k) = (9, 7, 129);
    let (lda, ldb, ldc) = (k + 3, k + 2, m + 1);
    let a: Vec<_> = (0..lda * m).map(|i| values[i % values.len()]).collect();
    let b: Vec<_> = (0..ldb * n)
        .map(|i| [1.0, -1.0, 0.5, 1e-100][i % 4])
        .collect();
    for alpha in [0.73, 1.0, -1.0, 1e-100] {
        for beta in [0.0, -0.0, 1.0, -1.0] {
            let mut expected: Vec<_> = (0..ldc * n).map(|i| values[i % 2]).collect();
            let mut actual = expected.clone();
            reference_product(m, n, k, alpha, &a, lda, &b, ldb, beta, &mut expected, ldc);
            dgemm(
                Transpose::Transpose,
                Transpose::None,
                m,
                n,
                k,
                alpha,
                &a,
                lda,
                &b,
                ldb,
                beta,
                &mut actual,
                ldc,
            )
            .unwrap();
            assert!(expected
                .iter()
                .zip(&actual)
                .all(|(a, b)| a.to_bits() == b.to_bits()));
        }
    }
}

pub(super) fn record(a: Transpose, b: Transpose, m: usize, n: usize, k: usize) {
    if !RECORD.load(Ordering::Relaxed) {
        return;
    }
    *SHAPES
        .lock()
        .unwrap()
        .entry((
            a == Transpose::Transpose,
            b == Transpose::Transpose,
            m,
            n,
            k,
        ))
        .or_default() += 1;
}

#[allow(clippy::too_many_arguments)]
fn tiled_product(
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
    transb: Transpose,
) {
    for j in (0..n).step_by(4) {
        for i in (0..m).step_by(4) {
            let rows = (m - i).min(4);
            let cols = (n - j).min(4);
            if rows == 4 && cols == 4 {
                let mut sums = [[0.0; 4]; 4];
                for q in 0..4 {
                    for r in 0..4 {
                        sums[q][r] = if beta == 0.0 {
                            0.0
                        } else if beta == 1.0 {
                            c[i + r + (j + q) * ldc]
                        } else {
                            beta * c[i + r + (j + q) * ldc]
                        };
                    }
                }
                for l in 0..k {
                    let av = &a[i + l * lda..i + l * lda + 4];
                    for q in 0..4 {
                        let temp = alpha
                            * b[if transb == Transpose::None {
                                l + (j + q) * ldb
                            } else {
                                j + q + l * ldb
                            }];
                        for r in 0..4 {
                            sums[q][r] += temp * av[r];
                        }
                    }
                }
                for q in 0..4 {
                    for r in 0..4 {
                        c[i + r + (j + q) * ldc] = sums[q][r];
                    }
                }
                continue;
            }
            let mut sums = [[0.0; 4]; 4];
            for q in 0..cols {
                for r in 0..rows {
                    sums[q][r] = if beta == 0.0 {
                        0.0
                    } else if beta == 1.0 {
                        c[i + r + (j + q) * ldc]
                    } else {
                        beta * c[i + r + (j + q) * ldc]
                    };
                }
            }
            for l in 0..k {
                let av = &a[i + l * lda..i + l * lda + rows];
                for q in 0..cols {
                    let temp = alpha
                        * b[if transb == Transpose::None {
                            l + (j + q) * ldb
                        } else {
                            j + q + l * ldb
                        }];
                    for r in 0..rows {
                        sums[q][r] += temp * av[r];
                    }
                }
            }
            for q in 0..cols {
                for r in 0..rows {
                    c[i + r + (j + q) * ldc] = sums[q][r];
                }
            }
        }
    }
}

#[test]
#[ignore = "benchmarks reference-order tiles on recorded DGEMM shapes"]
fn benchmark_tiled_product() {
    use std::{hint::black_box, time::Instant};
    for transb in [Transpose::None, Transpose::Transpose] {
        for (m, n, k) in [(7, 5, 9), (1167, 1200, 32), (1200, 32, 1167)] {
            let (lda, ldb, ldc) = (
                m + 3,
                if transb == Transpose::None {
                    k + 2
                } else {
                    n + 2
                },
                m + 1,
            );
            let a: Vec<_> = (0..lda * k).map(|i| (i as f64 * 0.31).sin()).collect();
            let b: Vec<_> = (0..ldb * if transb == Transpose::None { n } else { k })
                .map(|i| (i as f64 * 0.17).cos())
                .collect();
            for beta in [0.0, 1.0, -0.5] {
                let mut expected = vec![0.37; ldc * n];
                let mut actual = expected.clone();
                dgemm(
                    Transpose::None,
                    transb,
                    m,
                    n,
                    k,
                    0.73,
                    &a,
                    lda,
                    &b,
                    ldb,
                    beta,
                    &mut expected,
                    ldc,
                )
                .unwrap();
                tiled_product(
                    m,
                    n,
                    k,
                    0.73,
                    &a,
                    lda,
                    &b,
                    ldb,
                    beta,
                    &mut actual,
                    ldc,
                    transb,
                );
                assert!(expected
                    .iter()
                    .zip(&actual)
                    .all(|(a, b)| a.to_bits() == b.to_bits()));
            }
            let mut c = vec![0.37; ldc * n];
            for round in 0..6 {
                for candidate in if round % 2 == 0 {
                    [false, true]
                } else {
                    [true, false]
                } {
                    let start = Instant::now();
                    if candidate {
                        tiled_product(
                            m,
                            n,
                            k,
                            0.73,
                            black_box(&a),
                            lda,
                            &b,
                            ldb,
                            0.0,
                            &mut c,
                            ldc,
                            transb,
                        )
                    } else {
                        dgemm(
                            Transpose::None,
                            transb,
                            m,
                            n,
                            k,
                            0.73,
                            black_box(&a),
                            lda,
                            &b,
                            ldb,
                            0.0,
                            &mut c,
                            ldc,
                        )
                        .unwrap()
                    }
                    black_box(&c);
                    if round > 0 {
                        println!(
                            "{}",
                            serde_json::json!({"m":m,"n":n,"k":k,"transpose_b":transb==Transpose::Transpose,"candidate":candidate,"seconds":start.elapsed().as_secs_f64()})
                        );
                    }
                }
            }
        }
    }
}

pub(crate) fn trace(action: impl FnOnce()) {
    SHAPES.lock().unwrap().clear();
    RECORD.store(true, Ordering::Relaxed);
    action();
    RECORD.store(false, Ordering::Relaxed);
    let mut shapes: Vec<_> = SHAPES
        .lock()
        .unwrap()
        .iter()
        .map(|(shape, count)| (*shape, *count))
        .collect();
    shapes.sort_by_key(|((_, _, m, n, k), count)| std::cmp::Reverse(m * n * k * count));
    for ((a, b, m, n, k), count) in shapes {
        println!(
            "{}",
            serde_json::json!({"transpose_a":a,"transpose_b":b,"m":m,"n":n,"k":k,"calls":count,"multiply_adds":m*n*k*count})
        );
    }
}
