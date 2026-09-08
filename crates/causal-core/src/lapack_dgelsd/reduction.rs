//! Real Householder reductions used by `DGELSD`.
//!
//! This is a direct, column-major translation of the double-precision
//! routines in LAPACK 3.12.1.  The public entry points deliberately retain
//! LAPACK's explicit dimensions, leading dimensions, work arrays, workspace
//! queries, and negative `INFO` values.  Array bounds are asserted because a
//! too-short Rust slice is a programming error rather than a LAPACK argument.
//!
//! The unblocked kernels preserve the scalar update order of Netlib. The
//! drivers retain Netlib's `ILAENV` choices (`NB=32`, `NBMIN=2`, `NX=128`)
//! and blocked/unblocked branch structure. `dlarft` and `dlarfb` provide the
//! compact-WY path used above the crossover point.

use super::blas::{dgemm, dgemv, dlamch, dlapy2, dnrm2, dscal, dtrmm, Diag, Side, Transpose, Uplo};

const BLOCK_SIZE: usize = 32;

#[inline]
fn same(c: u8, expected: u8) -> bool {
    c.eq_ignore_ascii_case(&expected)
}

#[inline]
fn at(row: usize, col: usize, ld: usize) -> usize {
    row + col * ld
}

fn check_matrix(a: &[f64], rows: usize, cols: usize, lda: usize) {
    if rows != 0 && cols != 0 {
        assert!(a.len() >= at(rows - 1, cols - 1, lda) + 1);
    }
}

/// DLARFG: generate `H = I - tau * (1,v)^T (1,v)`.
pub(crate) fn dlarfg(n: usize, alpha: &mut f64, x: &mut [f64], incx: usize) -> f64 {
    assert!(incx > 0);
    if n <= 1 {
        return 0.0;
    }
    assert!(x.len() >= 1 + (n - 2) * incx);
    let mut xnorm = dnrm2(n - 1, x, incx as isize);
    if xnorm == 0.0 {
        return 0.0;
    }

    let safmin = dlamch('S') / dlamch('E');
    let mut beta = -dlapy2(*alpha, xnorm).copysign(*alpha);
    let mut knt = 0usize;
    if beta.abs() < safmin {
        let rsafmn = 1.0 / safmin;
        loop {
            knt += 1;
            dscal(n - 1, rsafmn, x, incx as isize);
            beta *= rsafmn;
            *alpha *= rsafmn;
            if beta.abs() >= safmin || knt >= 20 {
                break;
            }
        }
        xnorm = dnrm2(n - 1, x, incx as isize);
        beta = -dlapy2(*alpha, xnorm).copysign(*alpha);
    }
    let tau = (beta - *alpha) / beta;
    let scale = 1.0 / (*alpha - beta);
    dscal(n - 1, scale, x, incx as isize);
    for _ in 0..knt {
        beta *= safmin;
    }
    *alpha = beta;
    tau
}

/// DLARF1F. `v[0]` is implicit one and is never read.
pub(crate) fn dlarf1f(
    side: u8,
    m: usize,
    n: usize,
    v: &[f64],
    incv: usize,
    tau: f64,
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
) {
    let left = same(side, b'L');
    let nv = if left { m } else { n };
    assert!(incv > 0);
    if nv > 0 {
        assert!(v.len() >= 1 + (nv - 1) * incv);
    }
    check_matrix(c, m, n, ldc);
    assert!(work.len() >= if left { n } else { m });
    if tau == 0.0 || m == 0 || n == 0 {
        return;
    }

    let mut lastv = nv;
    // V(1)=1 is implicit, hence do not inspect it.
    while lastv > 1 && v[(lastv - 1) * incv] == 0.0 {
        lastv -= 1;
    }
    let mut lastc = 0;
    if left {
        'cols: for j in (0..n).rev() {
            for i in 0..lastv {
                if c[at(i, j, ldc)] != 0.0 {
                    lastc = j + 1;
                    break 'cols;
                }
            }
        }
        if lastc == 0 {
            return;
        }
        if lastv == 1 {
            for j in 0..lastc {
                c[at(0, j, ldc)] *= 1.0 - tau;
            }
            return;
        }
        // DGEMV('T') followed by DAXPY, matching DLARF1F.
        for j in 0..lastc {
            let mut sum = 0.0;
            for i in 1..lastv {
                sum += c[at(i, j, ldc)] * v[i * incv];
            }
            work[j] = sum + c[at(0, j, ldc)];
        }
        for j in 0..lastc {
            c[at(0, j, ldc)] += -tau * work[j];
        }
        for j in 0..lastc {
            let w = -tau * work[j];
            for i in 1..lastv {
                c[at(i, j, ldc)] += v[i * incv] * w;
            }
        }
    } else {
        'rows: for i in (0..m).rev() {
            for j in 0..lastv {
                if c[at(i, j, ldc)] != 0.0 {
                    lastc = i + 1;
                    break 'rows;
                }
            }
        }
        if lastc == 0 {
            return;
        }
        if lastv == 1 {
            for ci in c.iter_mut().take(lastc) {
                *ci *= 1.0 - tau;
            }
            return;
        }
        // DGEMV('N') followed by DAXPY, matching DLARF1F.
        for i in 0..lastc {
            let mut sum = 0.0;
            for j in 1..lastv {
                sum += c[at(i, j, ldc)] * v[j * incv];
            }
            work[i] = sum + c[i];
        }
        for i in 0..lastc {
            c[i] += -tau * work[i];
        }
        for j in 1..lastv {
            let q = -tau * v[j * incv];
            for i in 0..lastc {
                c[at(i, j, ldc)] += work[i] * q;
            }
        }
    }
}

fn reflector_from_column(a: &[f64], lda: usize, i: usize, len: usize) -> Vec<f64> {
    let mut v = vec![0.0; len];
    if len != 0 {
        v[0] = 1.0;
        for r in 1..len {
            v[r] = a[at(i + r, i, lda)];
        }
    }
    v
}

fn reflector_from_row(a: &[f64], lda: usize, i: usize, len: usize) -> Vec<f64> {
    let mut v = vec![0.0; len];
    if len != 0 {
        v[0] = 1.0;
        for j in 1..len {
            v[j] = a[at(i, i + j, lda)];
        }
    }
    v
}

/// DGEQR2: unblocked QR factorization.
pub(crate) fn dgeqr2(
    m: usize,
    n: usize,
    a: &mut [f64],
    lda: usize,
    tau: &mut [f64],
    work: &mut [f64],
) -> i32 {
    if lda < m.max(1) {
        return -4;
    }
    check_matrix(a, m, n, lda);
    let k = m.min(n);
    assert!(tau.len() >= k && work.len() >= n.saturating_sub(1));
    for i in 0..k {
        let ii = at(i, i, lda);
        let mut alpha = a[ii];
        tau[i] = if m - i <= 1 {
            dlarfg(1, &mut alpha, &mut [], 1)
        } else {
            dlarfg(m - i, &mut alpha, &mut a[ii + 1..], 1)
        };
        a[ii] = alpha;
        if i + 1 < n {
            let v = reflector_from_column(a, lda, i, m - i);
            dlarf1f(
                b'L',
                m - i,
                n - i - 1,
                &v,
                1,
                tau[i],
                &mut a[at(i, i + 1, lda)..],
                lda,
                work,
            );
        }
    }
    0
}

/// DGELQ2: unblocked LQ factorization.
pub(crate) fn dgelq2(
    m: usize,
    n: usize,
    a: &mut [f64],
    lda: usize,
    tau: &mut [f64],
    work: &mut [f64],
) -> i32 {
    if lda < m.max(1) {
        return -4;
    }
    check_matrix(a, m, n, lda);
    let k = m.min(n);
    assert!(tau.len() >= k && work.len() >= m.saturating_sub(1));
    for i in 0..k {
        let ii = at(i, i, lda);
        let mut alpha = a[ii];
        tau[i] = if n - i <= 1 {
            dlarfg(1, &mut alpha, &mut [], 1)
        } else {
            // A(i,i+1) is lda-1 positions after A(i,i) in this tail.
            dlarfg(n - i, &mut alpha, &mut a[ii + lda..], lda)
        };
        a[ii] = alpha;
        if i + 1 < m {
            let v = reflector_from_row(a, lda, i, n - i);
            dlarf1f(
                b'R',
                m - i - 1,
                n - i,
                &v,
                1,
                tau[i],
                &mut a[at(i + 1, i, lda)..],
                lda,
                work,
            );
        }
    }
    0
}

/// DGEQRF with the LAPACK 3.12.1 generic `ILAENV` tuning constants.
pub(crate) fn dgeqrf(
    m: usize,
    n: usize,
    a: &mut [f64],
    lda: usize,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: isize,
) -> i32 {
    let k = m.min(n);
    if lda < m.max(1) {
        return -4;
    }
    if lwork != -1 && (lwork <= 0 || (m > 0 && lwork < n.max(1) as isize)) {
        return -7;
    }
    assert!(!work.is_empty());
    if lwork == -1 {
        work[0] = if k == 0 { 1.0 } else { (n * BLOCK_SIZE) as f64 };
        return 0;
    }
    if k == 0 {
        work[0] = 1.0;
        return 0;
    }
    let mut nb = BLOCK_SIZE;
    let nbmin = 2usize;
    let mut nx = 0usize;
    let mut iws = n;
    if nb > 1 && nb < k {
        nx = 128;
        if nx < k {
            iws = n * nb;
            if (lwork as usize) < iws {
                nb = lwork as usize / n;
            }
        }
    }
    let mut i = 0usize;
    if nb >= nbmin && nb < k && nx < k {
        while i < k - nx {
            let ib = (k - i).min(nb);
            let info = dgeqr2(m - i, ib, &mut a[at(i, i, lda)..], lda, &mut tau[i..], work);
            if info != 0 {
                return info;
            }
            if i + ib < n {
                let mut t = vec![0.0; ib * ib];
                dlarft(
                    b'F',
                    b'C',
                    m - i,
                    ib,
                    &a[at(i, i, lda)..],
                    lda,
                    &tau[i..],
                    &mut t,
                    ib,
                );
                let mut w = vec![0.0; (n - i - ib).max(1) * ib];
                let ldw = (n - i - ib).max(1);
                let panel = a[at(i, i, lda)..].to_vec();
                dlarfb(
                    b'L',
                    b'T',
                    b'F',
                    b'C',
                    m - i,
                    n - i - ib,
                    ib,
                    &panel,
                    lda,
                    &t,
                    ib,
                    &mut a[at(i, i + ib, lda)..],
                    lda,
                    &mut w,
                    ldw,
                );
            }
            i += ib;
        }
    }
    let info = if i < k {
        dgeqr2(
            m - i,
            n - i,
            &mut a[at(i, i, lda)..],
            lda,
            &mut tau[i..],
            work,
        )
    } else {
        0
    };
    work[0] = iws as f64;
    info
}

/// DGELQF with the LAPACK 3.12.1 generic `ILAENV` tuning constants.
pub(crate) fn dgelqf(
    m: usize,
    n: usize,
    a: &mut [f64],
    lda: usize,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: isize,
) -> i32 {
    let k = m.min(n);
    if lda < m.max(1) {
        return -4;
    }
    if lwork != -1 && (lwork <= 0 || (n > 0 && lwork < m.max(1) as isize)) {
        return -7;
    }
    assert!(!work.is_empty());
    if lwork == -1 {
        work[0] = if k == 0 { 1.0 } else { (m * BLOCK_SIZE) as f64 };
        return 0;
    }
    if k == 0 {
        work[0] = 1.0;
        return 0;
    }
    let mut nb = BLOCK_SIZE;
    let nbmin = 2usize;
    let mut nx = 0usize;
    let mut iws = m;
    if nb > 1 && nb < k {
        nx = 128;
        if nx < k {
            iws = m * nb;
            if (lwork as usize) < iws {
                nb = lwork as usize / m;
            }
        }
    }
    let mut i = 0usize;
    if nb >= nbmin && nb < k && nx < k {
        while i < k - nx {
            let ib = (k - i).min(nb);
            let info = dgelq2(ib, n - i, &mut a[at(i, i, lda)..], lda, &mut tau[i..], work);
            if info != 0 {
                return info;
            }
            if i + ib < m {
                let mut t = vec![0.0; ib * ib];
                dlarft(
                    b'F',
                    b'R',
                    n - i,
                    ib,
                    &a[at(i, i, lda)..],
                    lda,
                    &tau[i..],
                    &mut t,
                    ib,
                );
                let ldw = (m - i - ib).max(1);
                let mut w = vec![0.0; ldw * ib];
                let panel = a[at(i, i, lda)..].to_vec();
                dlarfb(
                    b'R',
                    b'N',
                    b'F',
                    b'R',
                    m - i - ib,
                    n - i,
                    ib,
                    &panel,
                    lda,
                    &t,
                    ib,
                    &mut a[at(i + ib, i, lda)..],
                    lda,
                    &mut w,
                    ldw,
                );
            }
            i += ib;
        }
    }
    let info = if i < k {
        dgelq2(
            m - i,
            n - i,
            &mut a[at(i, i, lda)..],
            lda,
            &mut tau[i..],
            work,
        )
    } else {
        0
    };
    work[0] = iws as f64;
    info
}

/// DORM2R: multiply by the Q from DGEQR2.
pub(crate) fn dorm2r(
    side: u8,
    trans: u8,
    m: usize,
    n: usize,
    k: usize,
    a: &[f64],
    lda: usize,
    tau: &[f64],
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
) -> i32 {
    let left = same(side, b'L');
    let notran = same(trans, b'N');
    let nq = if left { m } else { n };
    if !left && !same(side, b'R') {
        return -1;
    }
    if !notran && !same(trans, b'T') {
        return -2;
    }
    if k > nq {
        return -5;
    }
    if lda < nq.max(1) {
        return -7;
    }
    if ldc < m.max(1) {
        return -10;
    }
    check_matrix(c, m, n, ldc);
    assert!(tau.len() >= k);
    if m == 0 || n == 0 || k == 0 {
        return 0;
    }
    let forward = (left && !notran) || (!left && notran);
    let iter: Box<dyn Iterator<Item = usize>> = if forward {
        Box::new(0..k)
    } else {
        Box::new((0..k).rev())
    };
    for i in iter {
        let (mi, ni, ic, jc) = if left {
            (m - i, n, i, 0)
        } else {
            (m, n - i, 0, i)
        };
        let v = reflector_from_column(a, lda, i, nq - i);
        dlarf1f(
            side,
            mi,
            ni,
            &v,
            1,
            tau[i],
            &mut c[at(ic, jc, ldc)..],
            ldc,
            work,
        );
    }
    0
}

/// DORML2: multiply by the Q from DGELQ2.
pub(crate) fn dorml2(
    side: u8,
    trans: u8,
    m: usize,
    n: usize,
    k: usize,
    a: &[f64],
    lda: usize,
    tau: &[f64],
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
) -> i32 {
    let left = same(side, b'L');
    let notran = same(trans, b'N');
    let nq = if left { m } else { n };
    if !left && !same(side, b'R') {
        return -1;
    }
    if !notran && !same(trans, b'T') {
        return -2;
    }
    if k > nq {
        return -5;
    }
    if lda < k.max(1) {
        return -7;
    }
    if ldc < m.max(1) {
        return -10;
    }
    check_matrix(c, m, n, ldc);
    assert!(tau.len() >= k);
    if m == 0 || n == 0 || k == 0 {
        return 0;
    }
    let forward = (left && notran) || (!left && !notran);
    let iter: Box<dyn Iterator<Item = usize>> = if forward {
        Box::new(0..k)
    } else {
        Box::new((0..k).rev())
    };
    for i in iter {
        let (mi, ni, ic, jc) = if left {
            (m - i, n, i, 0)
        } else {
            (m, n - i, 0, i)
        };
        let v = reflector_from_row(a, lda, i, nq - i);
        dlarf1f(
            side,
            mi,
            ni,
            &v,
            1,
            tau[i],
            &mut c[at(ic, jc, ldc)..],
            ldc,
            work,
        );
    }
    0
}

fn dorm_workspace(side: u8, m: usize, n: usize) -> usize {
    if same(side, b'L') {
        n.max(1)
    } else {
        m.max(1)
    }
}

/// DORMQR workspace-query wrapper around DORM2R.
pub(crate) fn dormqr(
    side: u8,
    trans: u8,
    m: usize,
    n: usize,
    k: usize,
    a: &[f64],
    lda: usize,
    tau: &[f64],
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
    lwork: isize,
) -> i32 {
    let left = same(side, b'L');
    let notran = same(trans, b'N');
    let nq = if left { m } else { n };
    let nw = dorm_workspace(side, m, n);
    if !left && !same(side, b'R') {
        return -1;
    }
    if !notran && !same(trans, b'T') {
        return -2;
    }
    if k > nq {
        return -5;
    }
    if lda < nq.max(1) {
        return -7;
    }
    if ldc < m.max(1) {
        return -10;
    }
    if lwork != -1 && lwork < nw as isize {
        return -12;
    }
    assert!(!work.is_empty());
    const TSIZE: usize = 65 * 64;
    let optimum = nw * BLOCK_SIZE + TSIZE;
    work[0] = optimum as f64;
    if lwork == -1 {
        return 0;
    }
    if m == 0 || n == 0 || k == 0 {
        work[0] = 1.0;
        return 0;
    }
    let mut nb = BLOCK_SIZE.min(64);
    if nb > 1 && nb < k && (lwork as usize) < optimum {
        nb = if lwork as usize > TSIZE {
            (lwork as usize - TSIZE) / nw
        } else {
            0
        };
    }
    let info = if nb < 2 || nb >= k {
        dorm2r(side, trans, m, n, k, a, lda, tau, c, ldc, work)
    } else {
        let ascending = (left && !notran) || (!left && notran);
        let mut starts = Vec::new();
        if ascending {
            let mut i = 0;
            while i < k {
                starts.push(i);
                i += nb;
            }
        } else {
            let mut i = ((k - 1) / nb) * nb;
            loop {
                starts.push(i);
                if i < nb {
                    break;
                }
                i -= nb;
            }
        }
        for i in starts {
            let ib = nb.min(k - i);
            let mut t = vec![0.0; ib * ib];
            dlarft(
                b'F',
                b'C',
                nq - i,
                ib,
                &a[at(i, i, lda)..],
                lda,
                &tau[i..],
                &mut t,
                ib,
            );
            let (mi, ni, co) = if left {
                (m - i, n, i)
            } else {
                (m, n - i, i * ldc)
            };
            let ldw = if left { ni.max(1) } else { mi.max(1) };
            let mut w = vec![0.0; ldw * ib];
            dlarfb(
                side,
                trans,
                b'F',
                b'C',
                mi,
                ni,
                ib,
                &a[at(i, i, lda)..],
                lda,
                &t,
                ib,
                &mut c[co..],
                ldc,
                &mut w,
                ldw,
            );
        }
        0
    };
    work[0] = optimum as f64;
    info
}

/// DORMLQ workspace-query wrapper around DORML2.
pub(crate) fn dormlq(
    side: u8,
    trans: u8,
    m: usize,
    n: usize,
    k: usize,
    a: &[f64],
    lda: usize,
    tau: &[f64],
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
    lwork: isize,
) -> i32 {
    let left = same(side, b'L');
    let notran = same(trans, b'N');
    let nq = if left { m } else { n };
    let nw = dorm_workspace(side, m, n);
    if !left && !same(side, b'R') {
        return -1;
    }
    if !notran && !same(trans, b'T') {
        return -2;
    }
    if k > nq {
        return -5;
    }
    if lda < k.max(1) {
        return -7;
    }
    if ldc < m.max(1) {
        return -10;
    }
    if lwork != -1 && lwork < nw as isize {
        return -12;
    }
    assert!(!work.is_empty());
    const TSIZE: usize = 65 * 64;
    let optimum = nw * BLOCK_SIZE + TSIZE;
    work[0] = optimum as f64;
    if lwork == -1 {
        return 0;
    }
    if m == 0 || n == 0 || k == 0 {
        work[0] = 1.0;
        return 0;
    }
    let mut nb = BLOCK_SIZE.min(64);
    if nb > 1 && nb < k && (lwork as usize) < optimum {
        nb = if lwork as usize > TSIZE {
            (lwork as usize - TSIZE) / nw
        } else {
            0
        };
    }
    let info = if nb < 2 || nb >= k {
        dorml2(side, trans, m, n, k, a, lda, tau, c, ldc, work)
    } else {
        let ascending = (left && notran) || (!left && !notran);
        let mut starts = Vec::new();
        if ascending {
            let mut i = 0;
            while i < k {
                starts.push(i);
                i += nb;
            }
        } else {
            let mut i = ((k - 1) / nb) * nb;
            loop {
                starts.push(i);
                if i < nb {
                    break;
                }
                i -= nb;
            }
        }
        let block_trans = if notran { b'T' } else { b'N' };
        for i in starts {
            let ib = nb.min(k - i);
            let mut t = vec![0.0; ib * ib];
            dlarft(
                b'F',
                b'R',
                nq - i,
                ib,
                &a[at(i, i, lda)..],
                lda,
                &tau[i..],
                &mut t,
                ib,
            );
            let (mi, ni, co) = if left {
                (m - i, n, i)
            } else {
                (m, n - i, i * ldc)
            };
            let ldw = if left { ni.max(1) } else { mi.max(1) };
            let mut w = vec![0.0; ldw * ib];
            dlarfb(
                side,
                block_trans,
                b'F',
                b'R',
                mi,
                ni,
                ib,
                &a[at(i, i, lda)..],
                lda,
                &t,
                ib,
                &mut c[co..],
                ldc,
                &mut w,
                ldw,
            );
        }
        0
    };
    work[0] = optimum as f64;
    info
}

/// DGEBD2: unblocked reduction to upper/lower bidiagonal form.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dgebd2(
    m: usize,
    n: usize,
    a: &mut [f64],
    lda: usize,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [f64],
    taup: &mut [f64],
    work: &mut [f64],
) -> i32 {
    if lda < m.max(1) {
        return -4;
    }
    check_matrix(a, m, n, lda);
    let k = m.min(n);
    assert!(d.len() >= k && e.len() >= k.saturating_sub(1));
    assert!(tauq.len() >= k && taup.len() >= k);
    assert!(work.len() >= m.max(n).max(1));
    if m >= n {
        for i in 0..n {
            let ii = at(i, i, lda);
            let mut alpha = a[ii];
            tauq[i] = if m - i == 1 {
                dlarfg(1, &mut alpha, &mut [], 1)
            } else {
                dlarfg(m - i, &mut alpha, &mut a[ii + 1..], 1)
            };
            a[ii] = alpha;
            d[i] = alpha;
            if i + 1 < n {
                let v = reflector_from_column(a, lda, i, m - i);
                dlarf1f(
                    b'L',
                    m - i,
                    n - i - 1,
                    &v,
                    1,
                    tauq[i],
                    &mut a[at(i, i + 1, lda)..],
                    lda,
                    work,
                );

                let ij = at(i, i + 1, lda);
                let mut alpha = a[ij];
                taup[i] = if n - i - 1 == 1 {
                    dlarfg(1, &mut alpha, &mut [], 1)
                } else {
                    dlarfg(n - i - 1, &mut alpha, &mut a[ij + lda..], lda)
                };
                a[ij] = alpha;
                e[i] = alpha;
                let v = {
                    let len = n - i - 1;
                    let mut q = vec![0.0; len];
                    q[0] = 1.0;
                    for j in 1..len {
                        q[j] = a[at(i, i + 1 + j, lda)];
                    }
                    q
                };
                dlarf1f(
                    b'R',
                    m - i - 1,
                    n - i - 1,
                    &v,
                    1,
                    taup[i],
                    &mut a[at(i + 1, i + 1, lda)..],
                    lda,
                    work,
                );
            } else {
                taup[i] = 0.0;
            }
        }
    } else {
        for i in 0..m {
            let ii = at(i, i, lda);
            let mut alpha = a[ii];
            taup[i] = if n - i == 1 {
                dlarfg(1, &mut alpha, &mut [], 1)
            } else {
                dlarfg(n - i, &mut alpha, &mut a[ii + lda..], lda)
            };
            a[ii] = alpha;
            d[i] = alpha;
            if i + 1 < m {
                let v = reflector_from_row(a, lda, i, n - i);
                dlarf1f(
                    b'R',
                    m - i - 1,
                    n - i,
                    &v,
                    1,
                    taup[i],
                    &mut a[at(i + 1, i, lda)..],
                    lda,
                    work,
                );

                let ji = at(i + 1, i, lda);
                let mut alpha = a[ji];
                tauq[i] = if m - i - 1 == 1 {
                    dlarfg(1, &mut alpha, &mut [], 1)
                } else {
                    dlarfg(m - i - 1, &mut alpha, &mut a[ji + 1..], 1)
                };
                a[ji] = alpha;
                e[i] = alpha;
                let len = m - i - 1;
                let mut v = vec![0.0; len];
                v[0] = 1.0;
                for r in 1..len {
                    v[r] = a[at(i + 1 + r, i, lda)];
                }
                dlarf1f(
                    b'L',
                    m - i - 1,
                    n - i - 1,
                    &v,
                    1,
                    tauq[i],
                    &mut a[at(i + 1, i + 1, lda)..],
                    lda,
                    work,
                );
            } else {
                tauq[i] = 0.0;
            }
        }
    }
    0
}

/// DGEBRD with the LAPACK 3.12.1 generic `ILAENV` tuning constants.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dgebrd(
    m: usize,
    n: usize,
    a: &mut [f64],
    lda: usize,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [f64],
    taup: &mut [f64],
    work: &mut [f64],
    lwork: isize,
) -> i32 {
    if lda < m.max(1) {
        return -4;
    }
    let minwork = m.max(n).max(1);
    if lwork != -1 && lwork < minwork as isize {
        return -10;
    }
    assert!(!work.is_empty());
    work[0] = ((m + n) * BLOCK_SIZE).max(1) as f64;
    if lwork == -1 {
        return 0;
    }
    let minmn = m.min(n);
    if minmn == 0 {
        work[0] = 1.0;
        return 0;
    }
    let mut nb = BLOCK_SIZE;
    let mut nx = minmn;
    let mut ws = minwork;
    if nb > 1 && nb < minmn {
        nx = nb.max(128);
        if nx < minmn {
            ws = (m + n) * nb;
            if (lwork as usize) < ws {
                let nbmin = 2;
                if lwork as usize >= (m + n) * nbmin {
                    nb = lwork as usize / (m + n);
                } else {
                    nb = 1;
                    nx = minmn;
                }
            }
        }
    }
    let mut i = 0usize;
    while nb > 1 && nx < minmn && i < minmn - nx {
        let mut xbuf = vec![0.0; m * nb];
        let mut ybuf = vec![0.0; n * nb];
        dlabrd(
            m - i,
            n - i,
            nb,
            &mut a[at(i, i, lda)..],
            lda,
            &mut d[i..],
            &mut e[i..],
            &mut tauq[i..],
            &mut taup[i..],
            &mut xbuf,
            m,
            &mut ybuf,
            n,
        );

        let mr = m - i - nb;
        let nr = n - i - nb;
        if mr != 0 && nr != 0 {
            // A22 -= V * Y2^T.
            let ac = a.to_vec();
            dgemm_view(
                Transpose::None,
                Transpose::Transpose,
                mr,
                nr,
                nb,
                -1.0,
                &ac[at(i + nb, i, lda)..],
                lda,
                &ybuf[nb..],
                n,
                1.0,
                &mut a[at(i + nb, i + nb, lda)..],
                lda,
            );
            // A22 -= X2 * U^T (U^T is stored in the panel rows).
            let ac = a.to_vec();
            dgemm_view(
                Transpose::None,
                Transpose::None,
                mr,
                nr,
                nb,
                -1.0,
                &xbuf[nb..],
                m,
                &ac[at(i, i + nb, lda)..],
                lda,
                1.0,
                &mut a[at(i + nb, i + nb, lda)..],
                lda,
            );
        }
        if m >= n {
            for j in i..i + nb {
                a[at(j, j, lda)] = d[j];
                a[at(j, j + 1, lda)] = e[j];
            }
        } else {
            for j in i..i + nb {
                a[at(j, j, lda)] = d[j];
                a[at(j + 1, j, lda)] = e[j];
            }
        }
        i += nb;
    }
    let e_start = i.min(e.len());
    let info = dgebd2(
        m - i,
        n - i,
        &mut a[at(i, i, lda)..],
        lda,
        &mut d[i..],
        &mut e[e_start..],
        &mut tauq[i..],
        &mut taup[i..],
        work,
    );
    work[0] = ws as f64;
    info
}

// Materialize the K elementary reflectors represented by V.  This helper is
// intentionally used only by DLARFT/DLARFB; the factorization kernels above
// never materialize their reflectors.
fn compact_vectors(direct: u8, storev: u8, n: usize, k: usize, v: &[f64], ldv: usize) -> Vec<f64> {
    let columnwise = same(storev, b'C');
    if columnwise {
        check_matrix(v, n, k, ldv);
    } else {
        check_matrix(v, k, n, ldv);
    }
    let forward = same(direct, b'F');
    let mut u = vec![0.0; n * k];
    for j in 0..k {
        let pivot = if forward { j } else { n - k + j };
        u[at(pivot, j, n)] = 1.0;
        if forward {
            for r in pivot + 1..n {
                u[at(r, j, n)] = if columnwise {
                    v[at(r, j, ldv)]
                } else {
                    v[at(j, r, ldv)]
                };
            }
        } else {
            for r in 0..pivot {
                u[at(r, j, n)] = if columnwise {
                    v[at(r, j, ldv)]
                } else {
                    v[at(j, r, ldv)]
                };
            }
        }
    }
    u
}

/// DLARFT: form the triangular factor T of a block reflector.
///
/// The implementation follows the defining recurrence rather than the
/// recursive DGEMM implementation introduced in LAPACK 3.12.  Both produce
/// the same compact WY reflector; the scalar recurrence is also the classic
/// CLAPACK 3.2.1 mechanical translation.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dlarft(
    direct: u8,
    storev: u8,
    n: usize,
    k: usize,
    v: &[f64],
    ldv: usize,
    tau: &[f64],
    t: &mut [f64],
    ldt: usize,
) {
    if n == 0 || k == 0 {
        return;
    }
    assert!(k <= n && tau.len() >= k && ldt >= k.max(1));
    check_matrix(t, k, k, ldt);
    for j in 0..k {
        for i in 0..k {
            t[at(i, j, ldt)] = 0.0;
        }
    }
    if same(direct, b'F') {
        if n == 1 || k == 1 {
            t[0] = tau[0];
            return;
        }
        let l = k / 2;
        let columnwise = same(storev, b'C');
        dlarft(direct, storev, n, l, v, ldv, tau, t, ldt);
        let voff = at(l, l, ldv);
        let toff = at(l, l, ldt);
        dlarft(
            direct,
            storev,
            n - l,
            k - l,
            &v[voff..],
            ldv,
            &tau[l..],
            &mut t[toff..],
            ldt,
        );
        if columnwise {
            // LAPACK 3.12.1 QR split: T12 = -T11 * V1^T*V2 * T22.
            for j in 0..k - l {
                for i in 0..l {
                    t[at(i, l + j, ldt)] = v[at(l + j, i, ldv)];
                }
            }
            dtrmm(
                Side::Right,
                Uplo::Lower,
                Transpose::None,
                Diag::Unit,
                l,
                k - l,
                1.0,
                &v[voff..],
                ldv,
                &mut t[l * ldt..],
                ldt,
            )
            .expect("validated DLARFT DTRMM");
            if n > k {
                dgemm_view(
                    Transpose::Transpose,
                    Transpose::None,
                    l,
                    k - l,
                    n - k,
                    1.0,
                    &v[k..],
                    ldv,
                    &v[at(k, l, ldv)..],
                    ldv,
                    1.0,
                    &mut t[l * ldt..],
                    ldt,
                );
            }
        } else {
            // LAPACK 3.12.1 LQ split.
            for j in 0..k - l {
                for i in 0..l {
                    t[at(i, l + j, ldt)] = v[at(i, l + j, ldv)];
                }
            }
            dtrmm(
                Side::Right,
                Uplo::Upper,
                Transpose::Transpose,
                Diag::Unit,
                l,
                k - l,
                1.0,
                &v[voff..],
                ldv,
                &mut t[l * ldt..],
                ldt,
            )
            .expect("validated DLARFT DTRMM");
            if n > k {
                dgemm_view(
                    Transpose::None,
                    Transpose::Transpose,
                    l,
                    k - l,
                    n - k,
                    1.0,
                    &v[k * ldv..],
                    ldv,
                    &v[at(l, k, ldv)..],
                    ldv,
                    1.0,
                    &mut t[l * ldt..],
                    ldt,
                );
            }
        }
        let t11 = t.to_vec();
        dtrmm(
            Side::Left,
            Uplo::Upper,
            Transpose::None,
            Diag::NonUnit,
            l,
            k - l,
            -1.0,
            &t11,
            ldt,
            &mut t[l * ldt..],
            ldt,
        )
        .expect("validated DLARFT DTRMM");
        let t22 = t[toff..].to_vec();
        dtrmm(
            Side::Right,
            Uplo::Upper,
            Transpose::None,
            Diag::NonUnit,
            l,
            k - l,
            1.0,
            &t22,
            ldt,
            &mut t[l * ldt..],
            ldt,
        )
        .expect("validated DLARFT DTRMM");
    } else {
        let u = compact_vectors(direct, storev, n, k, v, ldv);
        for i in (0..k).rev() {
            if tau[i] != 0.0 {
                let mut z = vec![0.0; k - i - 1];
                for j in i + 1..k {
                    let mut dot = 0.0;
                    for r in 0..n {
                        dot += u[at(r, j, n)] * u[at(r, i, n)];
                    }
                    z[j - i - 1] = -tau[i] * dot;
                }
                // T(i+1:k,i) = T(i+1:k,i+1:k) * z.
                for row in i + 1..k {
                    let mut sum = 0.0;
                    for col in i + 1..=row {
                        sum += t[at(row, col, ldt)] * z[col - i - 1];
                    }
                    t[at(row, i, ldt)] = sum;
                }
            }
            t[at(i, i, ldt)] = tau[i];
        }
    }
}

fn last_nonzero_row(m: usize, n: usize, a: &[f64], lda: usize) -> usize {
    for i in (0..m).rev() {
        for j in 0..n {
            if a[at(i, j, lda)] != 0.0 {
                return i + 1;
            }
        }
    }
    0
}

fn last_nonzero_col(m: usize, n: usize, a: &[f64], lda: usize) -> usize {
    for j in (0..n).rev() {
        for i in 0..m {
            if a[at(i, j, lda)] != 0.0 {
                return j + 1;
            }
        }
    }
    0
}

// Infallible adapter for already-validated LAPACK submatrix views.
#[allow(clippy::too_many_arguments)]
fn dgemm_view(
    transa: Transpose,
    transb: Transpose,
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
    dgemm(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
        .expect("validated LAPACK DGEMM view");
}

/// DLARFB: apply a compact WY block reflector.
///
/// `DIRECT='F'`, the only form required by DGELSD's QR/LQ/bidiagonal
/// closure, follows Netlib's staged DTRMM/DGEMM update order. The backward
/// forms retain the defining dense fallback below.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dlarfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: usize,
    n: usize,
    k: usize,
    v: &[f64],
    ldv: usize,
    t: &[f64],
    ldt: usize,
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
    ldwork: usize,
) {
    if m == 0 || n == 0 || k == 0 {
        return;
    }
    if !same(direct, b'F') {
        dlarfb_dense(
            side, trans, direct, storev, m, n, k, v, ldv, t, ldt, c, ldc, work, ldwork,
        );
        return;
    }
    let left = same(side, b'L');
    let columnwise = same(storev, b'C');
    let order = if left { m } else { n };
    if columnwise {
        check_matrix(v, order, k, ldv);
    } else {
        check_matrix(v, k, order, ldv);
    }
    check_matrix(c, m, n, ldc);
    check_matrix(t, k, k, ldt);
    let wr = if left { n } else { m };
    assert!(ldwork >= wr.max(1) && work.len() >= ldwork * k);
    let transt = if same(trans, b'N') {
        Transpose::Transpose
    } else {
        Transpose::None
    };
    let trans_direct = if same(trans, b'N') {
        Transpose::None
    } else {
        Transpose::Transpose
    };

    if columnwise {
        let lastv = k.max(last_nonzero_row(order, k, v, ldv));
        let lastc = if left {
            last_nonzero_col(lastv, n, c, ldc)
        } else {
            last_nonzero_row(m, lastv, c, ldc)
        };
        if lastc == 0 {
            return;
        }
        // W starts as C1^T (left) or C1 (right).
        for j in 0..k {
            for i in 0..lastc {
                work[at(i, j, ldwork)] = if left {
                    c[at(j, i, ldc)]
                } else {
                    c[at(i, j, ldc)]
                };
            }
        }
        dtrmm(
            Side::Right,
            Uplo::Lower,
            Transpose::None,
            Diag::Unit,
            lastc,
            k,
            1.0,
            v,
            ldv,
            work,
            ldwork,
        )
        .expect("validated DLARFB DTRMM");
        if lastv > k {
            if left {
                dgemm_view(
                    Transpose::Transpose,
                    Transpose::None,
                    lastc,
                    k,
                    lastv - k,
                    1.0,
                    &c[k..],
                    ldc,
                    &v[k..],
                    ldv,
                    1.0,
                    work,
                    ldwork,
                );
            } else {
                dgemm_view(
                    Transpose::None,
                    Transpose::None,
                    lastc,
                    k,
                    lastv - k,
                    1.0,
                    &c[k * ldc..],
                    ldc,
                    &v[k..],
                    ldv,
                    1.0,
                    work,
                    ldwork,
                );
            }
        }
        dtrmm(
            Side::Right,
            Uplo::Upper,
            if left { transt } else { trans_direct },
            Diag::NonUnit,
            lastc,
            k,
            1.0,
            t,
            ldt,
            work,
            ldwork,
        )
        .expect("validated DLARFB DTRMM");
        if lastv > k {
            if left {
                dgemm_view(
                    Transpose::None,
                    Transpose::Transpose,
                    lastv - k,
                    lastc,
                    k,
                    -1.0,
                    &v[k..],
                    ldv,
                    work,
                    ldwork,
                    1.0,
                    &mut c[k..],
                    ldc,
                );
            } else {
                dgemm_view(
                    Transpose::None,
                    Transpose::Transpose,
                    lastc,
                    lastv - k,
                    k,
                    -1.0,
                    work,
                    ldwork,
                    &v[k..],
                    ldv,
                    1.0,
                    &mut c[k * ldc..],
                    ldc,
                );
            }
        }
        dtrmm(
            Side::Right,
            Uplo::Lower,
            Transpose::Transpose,
            Diag::Unit,
            lastc,
            k,
            1.0,
            v,
            ldv,
            work,
            ldwork,
        )
        .expect("validated DLARFB DTRMM");
        for j in 0..k {
            for i in 0..lastc {
                if left {
                    c[at(j, i, ldc)] -= work[at(i, j, ldwork)];
                } else {
                    c[at(i, j, ldc)] -= work[at(i, j, ldwork)];
                }
            }
        }
    } else {
        let lastv = k.max(last_nonzero_col(k, order, v, ldv));
        let lastc = if left {
            last_nonzero_col(lastv, n, c, ldc)
        } else {
            last_nonzero_row(m, lastv, c, ldc)
        };
        if lastc == 0 {
            return;
        }
        for j in 0..k {
            for i in 0..lastc {
                work[at(i, j, ldwork)] = if left {
                    c[at(j, i, ldc)]
                } else {
                    c[at(i, j, ldc)]
                };
            }
        }
        dtrmm(
            Side::Right,
            Uplo::Upper,
            Transpose::Transpose,
            Diag::Unit,
            lastc,
            k,
            1.0,
            v,
            ldv,
            work,
            ldwork,
        )
        .expect("validated DLARFB DTRMM");
        if lastv > k {
            if left {
                dgemm_view(
                    Transpose::Transpose,
                    Transpose::Transpose,
                    lastc,
                    k,
                    lastv - k,
                    1.0,
                    &c[k..],
                    ldc,
                    &v[k * ldv..],
                    ldv,
                    1.0,
                    work,
                    ldwork,
                );
            } else {
                dgemm_view(
                    Transpose::None,
                    Transpose::Transpose,
                    lastc,
                    k,
                    lastv - k,
                    1.0,
                    &c[k * ldc..],
                    ldc,
                    &v[k * ldv..],
                    ldv,
                    1.0,
                    work,
                    ldwork,
                );
            }
        }
        dtrmm(
            Side::Right,
            Uplo::Upper,
            if left { transt } else { trans_direct },
            Diag::NonUnit,
            lastc,
            k,
            1.0,
            t,
            ldt,
            work,
            ldwork,
        )
        .expect("validated DLARFB DTRMM");
        if lastv > k {
            if left {
                dgemm_view(
                    Transpose::Transpose,
                    Transpose::Transpose,
                    lastv - k,
                    lastc,
                    k,
                    -1.0,
                    &v[k * ldv..],
                    ldv,
                    work,
                    ldwork,
                    1.0,
                    &mut c[k..],
                    ldc,
                );
            } else {
                dgemm_view(
                    Transpose::None,
                    Transpose::None,
                    lastc,
                    lastv - k,
                    k,
                    -1.0,
                    work,
                    ldwork,
                    &v[k * ldv..],
                    ldv,
                    1.0,
                    &mut c[k * ldc..],
                    ldc,
                );
            }
        }
        dtrmm(
            Side::Right,
            Uplo::Upper,
            Transpose::None,
            Diag::Unit,
            lastc,
            k,
            1.0,
            v,
            ldv,
            work,
            ldwork,
        )
        .expect("validated DLARFB DTRMM");
        for j in 0..k {
            for i in 0..lastc {
                if left {
                    c[at(j, i, ldc)] -= work[at(i, j, ldwork)];
                } else {
                    c[at(i, j, ldc)] -= work[at(i, j, ldwork)];
                }
            }
        }
    }
}

/// Defining fallback for backward DLARFB forms (unused by DGELSD).
#[allow(clippy::too_many_arguments)]
fn dlarfb_dense(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: usize,
    n: usize,
    k: usize,
    v: &[f64],
    ldv: usize,
    t: &[f64],
    ldt: usize,
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
    ldwork: usize,
) {
    if m == 0 || n == 0 || k == 0 {
        return;
    }
    let left = same(side, b'L');
    let order = if left { m } else { n };
    let u = compact_vectors(direct, storev, order, k, v, ldv);
    check_matrix(t, k, k, ldt);
    check_matrix(c, m, n, ldc);
    let wrows = if left { n } else { m };
    assert!(ldwork >= wrows.max(1) && work.len() >= ldwork * k);
    let transpose = same(trans, b'T');

    if left {
        // W = C^T U.
        for j in 0..k {
            for row in 0..n {
                let mut sum = 0.0;
                for q in 0..m {
                    sum += c[at(q, row, ldc)] * u[at(q, j, m)];
                }
                work[at(row, j, ldwork)] = sum;
            }
        }
        // C <- C - U op(T) U^T C; W is transposed, so W <- W op(T)^T.
        let old = work[..ldwork * k].to_vec();
        for j in 0..k {
            for row in 0..n {
                let mut sum = 0.0;
                for q in 0..k {
                    let tqj = if transpose {
                        t[at(q, j, ldt)]
                    } else {
                        t[at(j, q, ldt)]
                    };
                    sum += old[at(row, q, ldwork)] * tqj;
                }
                work[at(row, j, ldwork)] = sum;
            }
        }
        for col in 0..n {
            for row in 0..m {
                let mut sum = 0.0;
                for q in 0..k {
                    sum += u[at(row, q, m)] * work[at(col, q, ldwork)];
                }
                c[at(row, col, ldc)] -= sum;
            }
        }
    } else {
        // W = C U.
        for j in 0..k {
            for row in 0..m {
                let mut sum = 0.0;
                for q in 0..n {
                    sum += c[at(row, q, ldc)] * u[at(q, j, n)];
                }
                work[at(row, j, ldwork)] = sum;
            }
        }
        // W <- W op(T).
        let old = work[..ldwork * k].to_vec();
        for j in 0..k {
            for row in 0..m {
                let mut sum = 0.0;
                for q in 0..k {
                    let tqj = if transpose {
                        t[at(j, q, ldt)]
                    } else {
                        t[at(q, j, ldt)]
                    };
                    sum += old[at(row, q, ldwork)] * tqj;
                }
                work[at(row, j, ldwork)] = sum;
            }
        }
        for col in 0..n {
            for row in 0..m {
                let mut sum = 0.0;
                for q in 0..k {
                    sum += work[at(row, q, ldwork)] * u[at(col, q, n)];
                }
                c[at(row, col, ldc)] -= sum;
            }
        }
    }
}

/// DORMBR: apply Q or P from DGEBRD.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dormbr(
    vect: u8,
    side: u8,
    trans: u8,
    m: usize,
    n: usize,
    k: usize,
    a: &[f64],
    lda: usize,
    tau: &[f64],
    c: &mut [f64],
    ldc: usize,
    work: &mut [f64],
    lwork: isize,
) -> i32 {
    let applyq = same(vect, b'Q');
    let left = same(side, b'L');
    let notran = same(trans, b'N');
    let nq = if left { m } else { n };
    let nw = dorm_workspace(side, m, n);
    if !applyq && !same(vect, b'P') {
        return -1;
    }
    if !left && !same(side, b'R') {
        return -2;
    }
    if !notran && !same(trans, b'T') {
        return -3;
    }
    if (applyq && lda < nq.max(1)) || (!applyq && lda < nq.min(k).max(1)) {
        return -8;
    }
    if ldc < m.max(1) {
        return -11;
    }
    if lwork != -1 && lwork < nw as isize {
        return -13;
    }
    assert!(!work.is_empty());
    let optimum = nw * BLOCK_SIZE + 65 * 64;
    work[0] = optimum as f64;
    if lwork == -1 {
        return 0;
    }
    if m == 0 || n == 0 {
        work[0] = 1.0;
        return 0;
    }

    let info = if applyq {
        if nq >= k {
            dormqr(side, trans, m, n, k, a, lda, tau, c, ldc, work, lwork)
        } else if nq > 1 {
            let (mi, ni, co) = if left { (m - 1, n, 1) } else { (m, n - 1, ldc) };
            dormqr(
                side,
                trans,
                mi,
                ni,
                nq - 1,
                &a[1..],
                lda,
                tau,
                &mut c[co..],
                ldc,
                work,
                lwork,
            )
        } else {
            0
        }
    } else {
        // P is represented as a product of row reflectors; DORMBR applies
        // P for TRANS='N', whereas DORMLQ's Q is P^T.
        let transt = if notran { b'T' } else { b'N' };
        if nq > k {
            dormlq(side, transt, m, n, k, a, lda, tau, c, ldc, work, lwork)
        } else if nq > 1 {
            let (mi, ni, co) = if left { (m - 1, n, 1) } else { (m, n - 1, ldc) };
            dormlq(
                side,
                transt,
                mi,
                ni,
                nq - 1,
                &a[lda..],
                lda,
                tau,
                &mut c[co..],
                ldc,
                work,
                lwork,
            )
        } else {
            0
        }
    };
    work[0] = optimum as f64;
    info
}

#[allow(clippy::too_many_arguments)]
fn gemv(
    trans: bool,
    m: usize,
    n: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    x: &[f64],
    incx: usize,
    beta: f64,
    y: &mut [f64],
    incy: usize,
) {
    dgemv(
        if trans {
            Transpose::Transpose
        } else {
            Transpose::None
        },
        m,
        n,
        alpha,
        a,
        lda,
        x,
        incx as isize,
        beta,
        y,
        incy as isize,
    )
    .expect("validated LAPACK DGEMV view");
}

fn scal(n: usize, alpha: f64, x: &mut [f64], incx: usize) {
    dscal(n, alpha, x, incx as isize);
}

/// DLABRD: reduce the first `nb` rows and columns to bidiagonal form and
/// compute X and Y for the blocked trailing update
/// `A <- A - V*Y^T - X*U^T`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dlabrd(
    m: usize,
    n: usize,
    nb: usize,
    a: &mut [f64],
    lda: usize,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [f64],
    taup: &mut [f64],
    x: &mut [f64],
    ldx: usize,
    y: &mut [f64],
    ldy: usize,
) {
    if m == 0 || n == 0 {
        return;
    }
    assert!(nb <= m.min(n));
    check_matrix(a, m, n, lda);
    check_matrix(x, m, nb, ldx);
    check_matrix(y, n, nb, ldy);
    assert!(d.len() >= nb && e.len() >= nb && tauq.len() >= nb && taup.len() >= nb);

    if m >= n {
        for i in 0..nb {
            // A(i:m,i) -= A(i:m,0:i) * Y(i,0:i)^T.
            let ac = a.to_vec();
            gemv(
                false,
                m - i,
                i,
                -1.0,
                &ac[at(i, 0, lda)..],
                lda,
                &y[i..],
                ldy,
                1.0,
                &mut a[at(i, i, lda)..],
                1,
            );
            // A(i:m,i) -= X(i:m,0:i) * A(0:i,i).
            let ac = a.to_vec();
            let xc = x.to_vec();
            gemv(
                false,
                m - i,
                i,
                -1.0,
                &xc[at(i, 0, ldx)..],
                ldx,
                &ac[at(0, i, lda)..],
                1,
                1.0,
                &mut a[at(i, i, lda)..],
                1,
            );

            let ii = at(i, i, lda);
            let mut alpha = a[ii];
            tauq[i] = if m - i == 1 {
                dlarfg(1, &mut alpha, &mut [], 1)
            } else {
                dlarfg(m - i, &mut alpha, &mut a[ii + 1..], 1)
            };
            a[ii] = alpha;
            d[i] = alpha;
            if i < n - 1 {
                a[ii] = 1.0;
                let ac = a.to_vec();
                gemv(
                    true,
                    m - i,
                    n - i - 1,
                    1.0,
                    &ac[at(i, i + 1, lda)..],
                    lda,
                    &ac[ii..],
                    1,
                    0.0,
                    &mut y[at(i + 1, i, ldy)..],
                    1,
                );
                let ac = a.to_vec();
                gemv(
                    true,
                    m - i,
                    i,
                    1.0,
                    &ac[at(i, 0, lda)..],
                    lda,
                    &ac[ii..],
                    1,
                    0.0,
                    &mut y[at(0, i, ldy)..],
                    1,
                );
                let yc = y.to_vec();
                gemv(
                    false,
                    n - i - 1,
                    i,
                    -1.0,
                    &yc[at(i + 1, 0, ldy)..],
                    ldy,
                    &yc[at(0, i, ldy)..],
                    1,
                    1.0,
                    &mut y[at(i + 1, i, ldy)..],
                    1,
                );
                let ac = a.to_vec();
                let xc = x.to_vec();
                gemv(
                    true,
                    m - i,
                    i,
                    1.0,
                    &xc[at(i, 0, ldx)..],
                    ldx,
                    &ac[ii..],
                    1,
                    0.0,
                    &mut y[at(0, i, ldy)..],
                    1,
                );
                let ac = a.to_vec();
                let yc = y.to_vec();
                gemv(
                    true,
                    i,
                    n - i - 1,
                    -1.0,
                    &ac[at(0, i + 1, lda)..],
                    lda,
                    &yc[at(0, i, ldy)..],
                    1,
                    1.0,
                    &mut y[at(i + 1, i, ldy)..],
                    1,
                );
                scal(n - i - 1, tauq[i], &mut y[at(i + 1, i, ldy)..], 1);

                let ac = a.to_vec();
                let yc = y.to_vec();
                gemv(
                    false,
                    n - i - 1,
                    i + 1,
                    -1.0,
                    &yc[at(i + 1, 0, ldy)..],
                    ldy,
                    &ac[at(i, 0, lda)..],
                    lda,
                    1.0,
                    &mut a[at(i, i + 1, lda)..],
                    lda,
                );
                let ac = a.to_vec();
                let xc = x.to_vec();
                gemv(
                    true,
                    i,
                    n - i - 1,
                    -1.0,
                    &ac[at(0, i + 1, lda)..],
                    lda,
                    &xc[at(i, 0, ldx)..],
                    ldx,
                    1.0,
                    &mut a[at(i, i + 1, lda)..],
                    lda,
                );

                let ij = at(i, i + 1, lda);
                let mut alpha = a[ij];
                taup[i] = if n - i - 1 == 1 {
                    dlarfg(1, &mut alpha, &mut [], 1)
                } else {
                    dlarfg(n - i - 1, &mut alpha, &mut a[ij + lda..], lda)
                };
                a[ij] = alpha;
                e[i] = alpha;
                a[ij] = 1.0;

                let ac = a.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    n - i - 1,
                    1.0,
                    &ac[at(i + 1, i + 1, lda)..],
                    lda,
                    &ac[ij..],
                    lda,
                    0.0,
                    &mut x[at(i + 1, i, ldx)..],
                    1,
                );
                let ac = a.to_vec();
                let yc = y.to_vec();
                gemv(
                    true,
                    n - i - 1,
                    i + 1,
                    1.0,
                    &yc[at(i + 1, 0, ldy)..],
                    ldy,
                    &ac[ij..],
                    lda,
                    0.0,
                    &mut x[at(0, i, ldx)..],
                    1,
                );
                let ac = a.to_vec();
                let xc = x.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    i + 1,
                    -1.0,
                    &ac[at(i + 1, 0, lda)..],
                    lda,
                    &xc[at(0, i, ldx)..],
                    1,
                    1.0,
                    &mut x[at(i + 1, i, ldx)..],
                    1,
                );
                let ac = a.to_vec();
                gemv(
                    false,
                    i,
                    n - i - 1,
                    1.0,
                    &ac[at(0, i + 1, lda)..],
                    lda,
                    &ac[ij..],
                    lda,
                    0.0,
                    &mut x[at(0, i, ldx)..],
                    1,
                );
                let xc = x.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    i,
                    -1.0,
                    &xc[at(i + 1, 0, ldx)..],
                    ldx,
                    &xc[at(0, i, ldx)..],
                    1,
                    1.0,
                    &mut x[at(i + 1, i, ldx)..],
                    1,
                );
                scal(m - i - 1, taup[i], &mut x[at(i + 1, i, ldx)..], 1);
            }
        }
    } else {
        for i in 0..nb {
            let ac = a.to_vec();
            let yc = y.to_vec();
            gemv(
                false,
                n - i,
                i,
                -1.0,
                &yc[at(i, 0, ldy)..],
                ldy,
                &ac[at(i, 0, lda)..],
                lda,
                1.0,
                &mut a[at(i, i, lda)..],
                lda,
            );
            let ac = a.to_vec();
            let xc = x.to_vec();
            gemv(
                true,
                i,
                n - i,
                -1.0,
                &ac[at(0, i, lda)..],
                lda,
                &xc[at(i, 0, ldx)..],
                ldx,
                1.0,
                &mut a[at(i, i, lda)..],
                lda,
            );

            let ii = at(i, i, lda);
            let mut alpha = a[ii];
            taup[i] = if n - i == 1 {
                dlarfg(1, &mut alpha, &mut [], 1)
            } else {
                dlarfg(n - i, &mut alpha, &mut a[ii + lda..], lda)
            };
            a[ii] = alpha;
            d[i] = alpha;
            if i < m - 1 {
                a[ii] = 1.0;
                let ac = a.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    n - i,
                    1.0,
                    &ac[at(i + 1, i, lda)..],
                    lda,
                    &ac[ii..],
                    lda,
                    0.0,
                    &mut x[at(i + 1, i, ldx)..],
                    1,
                );
                let ac = a.to_vec();
                let yc = y.to_vec();
                gemv(
                    true,
                    n - i,
                    i,
                    1.0,
                    &yc[at(i, 0, ldy)..],
                    ldy,
                    &ac[ii..],
                    lda,
                    0.0,
                    &mut x[at(0, i, ldx)..],
                    1,
                );
                let ac = a.to_vec();
                let xc = x.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    i,
                    -1.0,
                    &ac[at(i + 1, 0, lda)..],
                    lda,
                    &xc[at(0, i, ldx)..],
                    1,
                    1.0,
                    &mut x[at(i + 1, i, ldx)..],
                    1,
                );
                let ac = a.to_vec();
                gemv(
                    false,
                    i,
                    n - i,
                    1.0,
                    &ac[at(0, i, lda)..],
                    lda,
                    &ac[ii..],
                    lda,
                    0.0,
                    &mut x[at(0, i, ldx)..],
                    1,
                );
                let xc = x.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    i,
                    -1.0,
                    &xc[at(i + 1, 0, ldx)..],
                    ldx,
                    &xc[at(0, i, ldx)..],
                    1,
                    1.0,
                    &mut x[at(i + 1, i, ldx)..],
                    1,
                );
                scal(m - i - 1, taup[i], &mut x[at(i + 1, i, ldx)..], 1);

                let ac = a.to_vec();
                let yc = y.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    i,
                    -1.0,
                    &ac[at(i + 1, 0, lda)..],
                    lda,
                    &yc[at(i, 0, ldy)..],
                    ldy,
                    1.0,
                    &mut a[at(i + 1, i, lda)..],
                    1,
                );
                let ac = a.to_vec();
                let xc = x.to_vec();
                gemv(
                    false,
                    m - i - 1,
                    i + 1,
                    -1.0,
                    &xc[at(i + 1, 0, ldx)..],
                    ldx,
                    &ac[at(0, i, lda)..],
                    1,
                    1.0,
                    &mut a[at(i + 1, i, lda)..],
                    1,
                );

                let ji = at(i + 1, i, lda);
                let mut alpha = a[ji];
                tauq[i] = if m - i - 1 == 1 {
                    dlarfg(1, &mut alpha, &mut [], 1)
                } else {
                    dlarfg(m - i - 1, &mut alpha, &mut a[ji + 1..], 1)
                };
                a[ji] = alpha;
                e[i] = alpha;
                a[ji] = 1.0;

                let ac = a.to_vec();
                gemv(
                    true,
                    m - i - 1,
                    n - i - 1,
                    1.0,
                    &ac[at(i + 1, i + 1, lda)..],
                    lda,
                    &ac[ji..],
                    1,
                    0.0,
                    &mut y[at(i + 1, i, ldy)..],
                    1,
                );
                let ac = a.to_vec();
                gemv(
                    true,
                    m - i - 1,
                    i,
                    1.0,
                    &ac[at(i + 1, 0, lda)..],
                    lda,
                    &ac[ji..],
                    1,
                    0.0,
                    &mut y[at(0, i, ldy)..],
                    1,
                );
                let yc = y.to_vec();
                gemv(
                    false,
                    n - i - 1,
                    i,
                    -1.0,
                    &yc[at(i + 1, 0, ldy)..],
                    ldy,
                    &yc[at(0, i, ldy)..],
                    1,
                    1.0,
                    &mut y[at(i + 1, i, ldy)..],
                    1,
                );
                let ac = a.to_vec();
                let xc = x.to_vec();
                gemv(
                    true,
                    m - i - 1,
                    i + 1,
                    1.0,
                    &xc[at(i + 1, 0, ldx)..],
                    ldx,
                    &ac[ji..],
                    1,
                    0.0,
                    &mut y[at(0, i, ldy)..],
                    1,
                );
                let ac = a.to_vec();
                let yc = y.to_vec();
                gemv(
                    true,
                    i + 1,
                    n - i - 1,
                    -1.0,
                    &ac[at(0, i + 1, lda)..],
                    lda,
                    &yc[at(0, i, ldy)..],
                    1,
                    1.0,
                    &mut y[at(i + 1, i, ldy)..],
                    1,
                );
                scal(n - i - 1, tauq[i], &mut y[at(i + 1, i, ldy)..], 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    #[link(name = "Accelerate", kind = "framework")]
    unsafe extern "C" {
        fn dgeqrf_(
            m: *const i32,
            n: *const i32,
            a: *mut f64,
            lda: *const i32,
            tau: *mut f64,
            work: *mut f64,
            lwork: *const i32,
            info: *mut i32,
        );
        fn dgebrd_(
            m: *const i32,
            n: *const i32,
            a: *mut f64,
            lda: *const i32,
            d: *mut f64,
            e: *mut f64,
            tauq: *mut f64,
            taup: *mut f64,
            work: *mut f64,
            lwork: *const i32,
            info: *mut i32,
        );
        fn dormqr_(
            side: *const u8,
            trans: *const u8,
            m: *const i32,
            n: *const i32,
            k: *const i32,
            a: *const f64,
            lda: *const i32,
            tau: *const f64,
            c: *mut f64,
            ldc: *const i32,
            work: *mut f64,
            lwork: *const i32,
            info: *mut i32,
        );
        fn dormbr_(
            vect: *const u8,
            side: *const u8,
            trans: *const u8,
            m: *const i32,
            n: *const i32,
            k: *const i32,
            a: *const f64,
            lda: *const i32,
            tau: *const f64,
            c: *mut f64,
            ldc: *const i32,
            work: *mut f64,
            lwork: *const i32,
            info: *mut i32,
        );
    }

    fn identity(n: usize) -> Vec<f64> {
        let mut a = vec![0.0; n * n];
        for i in 0..n {
            a[at(i, i, n)] = 1.0;
        }
        a
    }

    fn matmul(
        m: usize,
        n: usize,
        k: usize,
        a: &[f64],
        lda: usize,
        b: &[f64],
        ldb: usize,
    ) -> Vec<f64> {
        let mut c = vec![0.0; m * n];
        for j in 0..n {
            for i in 0..m {
                let mut sum = 0.0;
                for q in 0..k {
                    sum += a[at(i, q, lda)] * b[at(q, j, ldb)];
                }
                c[at(i, j, m)] = sum;
            }
        }
        c
    }

    fn max_abs_diff(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b)
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, f64::max)
    }

    fn seeded_matrix(m: usize, n: usize) -> Vec<f64> {
        let mut out = vec![0.0; m * n];
        let mut state = 0x1234_5678_9abc_def0u64;
        for j in 0..n {
            for i in 0..m {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                out[at(i, j, m)] = ((state >> 11) as f64) * (1.0 / ((1u64 << 53) as f64)) - 0.5
                    + 0.25 * (i == j) as u8 as f64;
            }
        }
        out
    }

    #[test]
    fn larfg_annihilates_tail() {
        let mut alpha = 4.0;
        let mut x = [3.0, 0.0];
        let tau = dlarfg(3, &mut alpha, &mut x, 1);
        assert_eq!(alpha, -5.0);
        assert_eq!(tau, 1.8);
        assert!((x[0] - 1.0 / 3.0).abs() < 2e-16);

        let mut c = vec![4.0, 3.0, 0.0];
        let v = [0.0, x[0], x[1]];
        let mut work = [0.0];
        dlarf1f(b'L', 3, 1, &v, 1, tau, &mut c, 3, &mut work);
        assert!((c[0] + 5.0).abs() < 1e-15);
        assert!(c[1].abs() < 1e-15 && c[2].abs() < 1e-15);
    }

    #[test]
    fn qr_factorization_reconstructs_rectangular_input() {
        let (m, n) = (4, 3);
        let original = vec![1.0, 4.0, 7.0, 2.0, 2.0, 5.0, 8.0, 3.0, 3.0, 6.0, 10.0, 5.0];
        let mut a = original.clone();
        let mut tau = vec![0.0; n];
        let mut work = vec![0.0; 256];
        assert_eq!(dgeqrf(m, n, &mut a, m, &mut tau, &mut work, 256), 0);
        let mut q = identity(m);
        assert_eq!(
            dormqr(b'L', b'N', m, m, n, &a, m, &tau, &mut q, m, &mut work, 256),
            0
        );
        let mut r = vec![0.0; m * n];
        for j in 0..n {
            for i in 0..=j.min(m - 1) {
                r[at(i, j, m)] = a[at(i, j, m)];
            }
        }
        let got = matmul(m, n, m, &q, m, &r, m);
        assert!(
            max_abs_diff(&got, &original) < 8e-15,
            "{}",
            max_abs_diff(&got, &original)
        );
    }

    #[test]
    fn lq_factorization_reconstructs_rectangular_input() {
        let (m, n) = (3, 5);
        let original = vec![
            1.0, 6.0, 11.0, 2.0, 7.0, 13.0, 3.0, 8.0, 15.0, 4.0, 9.0, 17.0, 5.0, 10.0, 19.0,
        ];
        let mut a = original.clone();
        let mut tau = vec![0.0; m];
        let mut work = vec![0.0; 256];
        assert_eq!(dgelqf(m, n, &mut a, m, &mut tau, &mut work, 256), 0);
        let mut q = identity(n);
        assert_eq!(
            dormlq(b'R', b'N', n, n, m, &a, m, &tau, &mut q, n, &mut work, 256),
            0
        );
        let mut l = vec![0.0; m * n];
        for j in 0..m {
            for i in j..m {
                l[at(i, j, m)] = a[at(i, j, m)];
            }
        }
        let got = matmul(m, n, n, &l, m, &q, n);
        assert!(
            max_abs_diff(&got, &original) < 2e-14,
            "{}",
            max_abs_diff(&got, &original)
        );
    }

    #[test]
    fn compact_wy_matches_individual_qr_reflectors() {
        let (m, n) = (5, 3);
        let mut a = vec![
            1.0, 4.0, 7.0, 2.0, 1.0, 2.0, 5.0, 8.0, 3.0, 0.0, 3.0, 6.0, 10.0, 5.0, 2.0,
        ];
        let mut tau = vec![0.0; n];
        let mut scratch = vec![0.0; 256];
        assert_eq!(dgeqr2(m, n, &mut a, m, &mut tau, &mut scratch), 0);
        let mut t = vec![0.0; n * n];
        dlarft(b'F', b'C', m, n, &a, m, &tau, &mut t, n);
        let mut blocked = identity(m);
        let mut w = vec![0.0; m * n];
        dlarfb(
            b'L',
            b'N',
            b'F',
            b'C',
            m,
            m,
            n,
            &a,
            m,
            &t,
            n,
            &mut blocked,
            m,
            &mut w,
            m,
        );
        let mut scalar = identity(m);
        assert_eq!(
            dorm2r(
                b'L',
                b'N',
                m,
                m,
                n,
                &a,
                m,
                &tau,
                &mut scalar,
                m,
                &mut scratch
            ),
            0
        );
        assert!(
            max_abs_diff(&blocked, &scalar) < 3e-15,
            "{}",
            max_abs_diff(&blocked, &scalar)
        );
    }

    #[test]
    fn compact_wy_matches_individual_lq_reflectors() {
        let (m, n) = (3, 5);
        let mut a = vec![
            1.0, 6.0, 11.0, 2.0, 7.0, 13.0, 3.0, 8.0, 15.0, 4.0, 9.0, 17.0, 5.0, 10.0, 19.0,
        ];
        let mut tau = vec![0.0; m];
        let mut scratch = vec![0.0; 256];
        assert_eq!(dgelq2(m, n, &mut a, m, &mut tau, &mut scratch), 0);
        let mut t = vec![0.0; m * m];
        dlarft(b'F', b'R', n, m, &a, m, &tau, &mut t, m);
        let mut blocked = identity(n);
        let mut w = vec![0.0; n * m];
        // DORML2(...,'R','N') applies Q; DLARFB sees the transposed compact
        // representation for rowwise reflectors, exactly as DORMLQ does.
        dlarfb(
            b'R',
            b'T',
            b'F',
            b'R',
            n,
            n,
            m,
            &a,
            m,
            &t,
            m,
            &mut blocked,
            n,
            &mut w,
            n,
        );
        let mut scalar = identity(n);
        assert_eq!(
            dorml2(
                b'R',
                b'N',
                n,
                n,
                m,
                &a,
                m,
                &tau,
                &mut scalar,
                n,
                &mut scratch
            ),
            0
        );
        assert!(
            max_abs_diff(&blocked, &scalar) < 4e-15,
            "{}",
            max_abs_diff(&blocked, &scalar)
        );
    }

    #[test]
    fn blocked_qr_branch_reconstructs_input() {
        let (m, n) = (131, 129);
        let mut original = vec![0.0; m * n];
        for j in 0..n {
            for i in 0..m {
                original[at(i, j, m)] =
                    ((17 * i + 31 * j + 3) as f64).sin() + (i == j) as u8 as f64;
            }
        }
        let mut a = original.clone();
        let mut tau = vec![0.0; n];
        let mut work = vec![0.0; (m + n) * BLOCK_SIZE + 5000];
        let lwork = work.len() as isize;
        assert_eq!(dgeqrf(m, n, &mut a, m, &mut tau, &mut work, lwork), 0);
        let mut q = identity(m);
        assert_eq!(
            dormqr(b'L', b'N', m, m, n, &a, m, &tau, &mut q, m, &mut work, lwork),
            0
        );
        let mut r = vec![0.0; m * n];
        for j in 0..n {
            for i in 0..=j {
                r[at(i, j, m)] = a[at(i, j, m)];
            }
        }
        let got = matmul(m, n, m, &q, m, &r, m);
        assert!(
            max_abs_diff(&got, &original) < 2e-12,
            "{}",
            max_abs_diff(&got, &original)
        );
    }

    #[test]
    fn blocked_bidiagonal_branch_reconstructs_b() {
        let (m, n) = (130, 129);
        let mut original = vec![0.0; m * n];
        for j in 0..n {
            for i in 0..m {
                original[at(i, j, m)] = ((11 * i + 19 * j + 1) as f64).cos()
                    + 0.25 * ((7 * i + 5 * j + 2) as f64).sin()
                    + (i == j) as u8 as f64;
            }
        }
        let mut a = original.clone();
        let mut d = vec![0.0; n];
        let mut e = vec![0.0; n - 1];
        let mut tq = vec![0.0; n];
        let mut tp = vec![0.0; n];
        let mut work = vec![0.0; (m + n) * BLOCK_SIZE + 5000];
        let lwork = work.len() as isize;
        assert_eq!(
            dgebrd(m, n, &mut a, m, &mut d, &mut e, &mut tq, &mut tp, &mut work, lwork),
            0
        );
        let mut transformed = original.clone();
        assert_eq!(
            dormbr(
                b'Q',
                b'L',
                b'T',
                m,
                n,
                n,
                &a,
                m,
                &tq,
                &mut transformed,
                m,
                &mut work,
                lwork
            ),
            0
        );
        assert_eq!(
            dormbr(
                b'P',
                b'R',
                b'N',
                m,
                n,
                n,
                &a,
                m,
                &tp,
                &mut transformed,
                m,
                &mut work,
                lwork
            ),
            0
        );
        let mut expected = vec![0.0; m * n];
        for i in 0..n {
            expected[at(i, i, m)] = d[i];
            if i + 1 < n {
                expected[at(i, i + 1, m)] = e[i];
            }
        }
        assert!(
            max_abs_diff(&transformed, &expected) < 2e-11,
            "{}",
            max_abs_diff(&transformed, &expected)
        );
    }

    #[test]
    fn gebrd_104x34_respects_signed_crossover_guard() {
        let (m, n) = (104, 34);
        let mut blocked_driver = vec![0.0; m * n];
        for j in 0..n {
            for i in 0..m {
                blocked_driver[at(i, j, m)] =
                    ((13 * i + 7 * j + 1) as f64).sin() + (i == j) as u8 as f64;
            }
        }
        let mut unblocked = blocked_driver.clone();
        let mut d1 = vec![0.0; n];
        let mut e1 = vec![0.0; n - 1];
        let mut q1 = vec![0.0; n];
        let mut p1 = vec![0.0; n];
        let mut d2 = vec![0.0; n];
        let mut e2 = vec![0.0; n - 1];
        let mut q2 = vec![0.0; n];
        let mut p2 = vec![0.0; n];
        let mut work = vec![0.0; (m + n) * BLOCK_SIZE];
        let lwork = work.len() as isize;
        assert_eq!(
            dgebrd(
                m,
                n,
                &mut blocked_driver,
                m,
                &mut d1,
                &mut e1,
                &mut q1,
                &mut p1,
                &mut work,
                lwork
            ),
            0
        );
        assert_eq!(
            dgebd2(
                m,
                n,
                &mut unblocked,
                m,
                &mut d2,
                &mut e2,
                &mut q2,
                &mut p2,
                &mut work
            ),
            0
        );
        assert_eq!(blocked_driver, unblocked);
        assert_eq!(d1, d2);
        assert_eq!(e1, e2);
        assert_eq!(q1, q2);
        assert_eq!(p1, p2);
    }

    #[test]
    fn bidiagonal_reduction_and_dormbr_produce_b() {
        let (m, n) = (5, 3);
        let original = vec![
            1.0, 4.0, 7.0, 2.0, 1.0, 2.0, 5.0, 8.0, 3.0, 0.0, 3.0, 6.0, 10.0, 5.0, 2.0,
        ];
        let mut a = original.clone();
        let mut d = vec![0.0; n];
        let mut e = vec![0.0; n - 1];
        let mut tq = vec![0.0; n];
        let mut tp = vec![0.0; n];
        let mut work = vec![0.0; 8192];
        assert_eq!(
            dgebrd(m, n, &mut a, m, &mut d, &mut e, &mut tq, &mut tp, &mut work, 8192),
            0
        );
        let mut transformed = original.clone();
        assert_eq!(
            dormbr(
                b'Q',
                b'L',
                b'T',
                m,
                n,
                n,
                &a,
                m,
                &tq,
                &mut transformed,
                m,
                &mut work,
                8192
            ),
            0
        );
        assert_eq!(
            dormbr(
                b'P',
                b'R',
                b'N',
                m,
                n,
                n,
                &a,
                m,
                &tp,
                &mut transformed,
                m,
                &mut work,
                8192
            ),
            0
        );
        for j in 0..n {
            for i in 0..m {
                let expected = if i == j {
                    d[i]
                } else if j == i + 1 {
                    e[i]
                } else {
                    0.0
                };
                assert!(
                    (transformed[at(i, j, m)] - expected).abs() < 2e-14,
                    "({i},{j}) {} != {expected}",
                    transformed[at(i, j, m)]
                );
            }
        }
    }

    #[test]
    fn lower_bidiagonal_reduction_and_dormbr_produce_b() {
        let (m, n) = (3, 5);
        let original = vec![
            1.0, 6.0, 11.0, 2.0, 7.0, 13.0, 3.0, 8.0, 15.0, 4.0, 9.0, 17.0, 5.0, 10.0, 19.0,
        ];
        let mut a = original.clone();
        let mut d = vec![0.0; m];
        let mut e = vec![0.0; m - 1];
        let mut tq = vec![0.0; m];
        let mut tp = vec![0.0; m];
        let mut work = vec![0.0; 8192];
        assert_eq!(
            dgebrd(m, n, &mut a, m, &mut d, &mut e, &mut tq, &mut tp, &mut work, 8192),
            0
        );
        let mut transformed = original.clone();
        assert_eq!(
            dormbr(
                b'Q',
                b'L',
                b'T',
                m,
                n,
                n,
                &a,
                m,
                &tq,
                &mut transformed,
                m,
                &mut work,
                8192
            ),
            0
        );
        assert_eq!(
            dormbr(
                b'P',
                b'R',
                b'N',
                m,
                n,
                m,
                &a,
                m,
                &tp,
                &mut transformed,
                m,
                &mut work,
                8192
            ),
            0
        );
        for j in 0..n {
            for i in 0..m {
                let expected = if i == j {
                    d[i]
                } else if i == j + 1 {
                    e[j]
                } else {
                    0.0
                };
                assert!(
                    (transformed[at(i, j, m)] - expected).abs() < 3e-14,
                    "({i},{j}) {} != {expected}",
                    transformed[at(i, j, m)]
                );
            }
        }
    }

    #[test]
    fn workspace_queries_and_info_match_lapack_positions() {
        let mut work = [0.0];
        assert_eq!(dgeqrf(4, 3, &mut [], 4, &mut [], &mut work, -1), 0);
        assert_eq!(work[0], 96.0);
        assert_eq!(dgeqrf(4, 3, &mut [], 3, &mut [], &mut work, -1), -4);
        assert_eq!(
            dormqr(b'?', b'N', 0, 0, 0, &[], 1, &[], &mut [], 1, &mut work, -1),
            -1
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn accelerate_oracle_qr_and_apply_104x34() {
        let (m, n, nrhs) = (104usize, 34usize, 4usize);
        let mut rust = seeded_matrix(m, n);
        let mut oracle = rust.clone();
        let mut rtau = vec![0.0; n];
        let mut otau = vec![0.0; n];
        let mut rw = vec![0.0; 200_000];
        let mut ow = vec![0.0; 200_000];
        let (mi, ni, li, ri, lwi) = (m as i32, n as i32, m as i32, nrhs as i32, rw.len() as i32);
        let mut info = 0;
        assert_eq!(
            dgeqrf(m, n, &mut rust, m, &mut rtau, &mut rw, lwi as isize),
            0
        );
        unsafe {
            dgeqrf_(
                &mi,
                &ni,
                oracle.as_mut_ptr(),
                &li,
                otau.as_mut_ptr(),
                ow.as_mut_ptr(),
                &lwi,
                &mut info,
            );
        }
        assert_eq!(info, 0);
        assert!(max_abs_diff(&rust, &oracle) < 2e-14);
        assert!(max_abs_diff(&rtau, &otau) < 2e-15);
        let mut rb = seeded_matrix(m, nrhs);
        let mut ob = rb.clone();
        assert_eq!(
            dormqr(
                b'L',
                b'T',
                m,
                nrhs,
                n,
                &rust,
                m,
                &rtau,
                &mut rb,
                m,
                &mut rw,
                lwi as isize
            ),
            0
        );
        unsafe {
            dormqr_(
                b"L".as_ptr(),
                b"T".as_ptr(),
                &mi,
                &ri,
                &ni,
                oracle.as_ptr(),
                &li,
                otau.as_ptr(),
                ob.as_mut_ptr(),
                &li,
                ow.as_mut_ptr(),
                &lwi,
                &mut info,
            );
        }
        assert_eq!(info, 0);
        assert!(max_abs_diff(&rb, &ob) < 3e-14);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn accelerate_oracle_bidiagonal_and_apply_34x34() {
        let (n, nrhs) = (34usize, 4usize);
        let m = n;
        let mut rust = seeded_matrix(m, n);
        let mut oracle = rust.clone();
        let mut rd = vec![0.0; n];
        let mut od = vec![0.0; n];
        let mut re = vec![0.0; n - 1];
        let mut oe = vec![0.0; n - 1];
        let mut rq = vec![0.0; n];
        let mut oq = vec![0.0; n];
        let mut rp = vec![0.0; n];
        let mut op = vec![0.0; n];
        let mut rw = vec![0.0; 200_000];
        let mut ow = vec![0.0; 200_000];
        let (mi, ni, li, ri, lwi) = (m as i32, n as i32, m as i32, nrhs as i32, rw.len() as i32);
        let mut info = 0;
        assert_eq!(
            dgebrd(
                m,
                n,
                &mut rust,
                m,
                &mut rd,
                &mut re,
                &mut rq,
                &mut rp,
                &mut rw,
                lwi as isize
            ),
            0
        );
        unsafe {
            dgebrd_(
                &mi,
                &ni,
                oracle.as_mut_ptr(),
                &li,
                od.as_mut_ptr(),
                oe.as_mut_ptr(),
                oq.as_mut_ptr(),
                op.as_mut_ptr(),
                ow.as_mut_ptr(),
                &lwi,
                &mut info,
            );
        }
        assert_eq!(info, 0);
        assert!(max_abs_diff(&rust, &oracle) < 2e-12);
        assert!(max_abs_diff(&rd, &od) < 2e-12 && max_abs_diff(&re, &oe) < 2e-12);
        assert!(max_abs_diff(&rq, &oq) < 2e-12 && max_abs_diff(&rp, &op) < 2e-12);
        for (vect, trans, rtau, otau, rows) in [
            (b'Q', b'T', rq.as_slice(), oq.as_slice(), m),
            (b'P', b'N', rp.as_slice(), op.as_slice(), n),
        ] {
            let mut rb = seeded_matrix(rows, nrhs);
            let mut ob = rb.clone();
            let rowsi = rows as i32;
            assert_eq!(
                dormbr(
                    vect,
                    b'L',
                    trans,
                    rows,
                    nrhs,
                    n,
                    &rust,
                    m,
                    rtau,
                    &mut rb,
                    rows,
                    &mut rw,
                    lwi as isize
                ),
                0
            );
            unsafe {
                dormbr_(
                    &vect,
                    &b'L',
                    &trans,
                    &rowsi,
                    &ri,
                    &ni,
                    oracle.as_ptr(),
                    &li,
                    otau.as_ptr(),
                    ob.as_mut_ptr(),
                    &rowsi,
                    ow.as_mut_ptr(),
                    &lwi,
                    &mut info,
                );
            }
            assert_eq!(info, 0);
            assert!(max_abs_diff(&rb, &ob) < 2e-12);
        }
    }
}
