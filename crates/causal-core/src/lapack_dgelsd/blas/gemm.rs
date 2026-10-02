//! Matrix products with independent accumulators and reference summation order.
//! The DGEMM dispatcher validates dimensions and storage before calling here.
#[allow(clippy::too_many_arguments)]
pub(super) fn transposed_product(
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
    // With no inner dimension BLAS does not require storage for A or B.
    if k == 0 {
        for j in 0..n {
            for i in 0..m {
                c[i + j * ldc] = if beta == 0.0 {
                    alpha * 0.0
                } else {
                    alpha * 0.0 + beta * c[i + j * ldc]
                };
            }
        }
        return;
    }
    for j in 0..n {
        let b = &b[j * ldb..j * ldb + k];
        let c = &mut c[j * ldc..j * ldc + m];
        let end = m / 4 * 4;
        for i in (0..end).step_by(4) {
            let a0 = &a[i * lda..i * lda + k];
            let a1 = &a[(i + 1) * lda..(i + 1) * lda + k];
            let a2 = &a[(i + 2) * lda..(i + 2) * lda + k];
            let a3 = &a[(i + 3) * lda..(i + 3) * lda + k];
            let mut sums = [0.0; 4];
            for l in 0..k {
                sums[0] += a0[l] * b[l];
                sums[1] += a1[l] * b[l];
                sums[2] += a2[l] * b[l];
                sums[3] += a3[l] * b[l];
            }
            for r in 0..4 {
                c[i + r] = if beta == 0.0 {
                    alpha * sums[r]
                } else {
                    alpha * sums[r] + beta * c[i + r]
                };
            }
        }
        for i in end..m {
            let mut sum = 0.0;
            for l in 0..k {
                sum += a[l + i * lda] * b[l];
            }
            c[i] = if beta == 0.0 {
                alpha * sum
            } else {
                alpha * sum + beta * c[i]
            };
        }
    }
}
