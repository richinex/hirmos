//! LAPACK 3.12.1 `DGELSD` driver.
//!
//! This file follows `reference/lapack-3.12.1/SRC/dgelsd.f` branch for branch:
//! QR pre-reduction (path 1a), direct overdetermined reduction (path 1), LQ
//! pre-reduction (path 2a), and direct underdetermined reduction (path 2).
//! CLAPACK 3.2.1 was used only to check the conversion from one-based workspace
//! offsets to Rust ranges.

use core::fmt;

use super::blas::dlamch;
use super::divide_conquer::{dlalsd_, dlascl_};
use super::reduction::{dgebrd, dgelqf, dgeqrf, dormbr, dormlq, dormqr};

const SMLSIZ: usize = 25;
const BLOCK_SIZE: usize = 32;

/// Real and integer workspace sizes reported by LAPACK's query path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DgelsdWorkspace {
    pub minimum_real: usize,
    pub optimal_real: usize,
    pub minimum_integer: usize,
}

/// Errors added by the safe Rust boundary, plus LAPACK's positive `INFO`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DgelsdError {
    InvalidLeadingDimension {
        argument: usize,
        minimum: usize,
        actual: usize,
    },
    InvalidWorkspace {
        minimum: usize,
        actual: isize,
    },
    BufferTooShort {
        argument: usize,
        minimum: usize,
        actual: usize,
    },
    InternalRoutine {
        routine: &'static str,
        info: i32,
    },
    DidNotConverge {
        unconverged: usize,
    },
}

impl fmt::Display for DgelsdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLeadingDimension {
                argument,
                minimum,
                actual,
            } => write!(
                f,
                "DGELSD argument {argument} must be at least {minimum}, received {actual}"
            ),
            Self::InvalidWorkspace { minimum, actual } => write!(
                f,
                "DGELSD argument 12 must be -1 or at least {minimum}, received {actual}"
            ),
            Self::BufferTooShort {
                argument,
                minimum,
                actual,
            } => write!(
                f,
                "DGELSD argument {argument} needs {minimum} elements, received {actual}"
            ),
            Self::InternalRoutine { routine, info } => {
                write!(f, "DGELSD's {routine} call returned INFO={info}")
            }
            Self::DidNotConverge { unconverged } => write!(
                f,
                "DGELSD bidiagonal SVD did not converge ({unconverged} off-diagonal elements)"
            ),
        }
    }
}

impl std::error::Error for DgelsdError {}

#[inline]
fn matrix_len(rows: usize, columns: usize, leading_dimension: usize) -> usize {
    if rows == 0 || columns == 0 {
        0
    } else {
        (columns - 1) * leading_dimension + rows
    }
}

#[inline]
fn at(row: usize, column: usize, leading_dimension: usize) -> usize {
    row + column * leading_dimension
}

fn levels(minimum_dimension: usize) -> usize {
    let minimum_dimension = minimum_dimension.max(1);
    ((minimum_dimension as f64 / (SMLSIZ + 1) as f64).log2() as isize + 1).max(0) as usize
}

fn wlalsd(dimension: usize, right_hand_sides: usize, nlvl: usize) -> usize {
    9 * dimension
        + 2 * dimension * SMLSIZ
        + 8 * dimension * nlvl
        + dimension * right_hand_sides
        + (SMLSIZ + 1) * (SMLSIZ + 1)
}

/// Return the same real and integer workspace sizes as a `LWORK=-1` call.
pub fn dgelsd_workspace(m: usize, n: usize, nrhs: usize) -> DgelsdWorkspace {
    let minmn_actual = m.min(n);
    let nlvl = levels(minmn_actual);
    let minimum_integer = 3 * minmn_actual.max(1) * nlvl + 11 * minmn_actual.max(1);
    let mnthr = (minmn_actual as f64 * 1.6) as usize;

    let mut minimum_real = 1isize;
    let mut optimal_real = 1isize;
    let mut mm = m;

    if m >= n && m >= mnthr {
        mm = n;
        optimal_real = optimal_real.max((n + n * BLOCK_SIZE) as isize);
        optimal_real = optimal_real.max((n + nrhs * BLOCK_SIZE) as isize);
    }

    if m >= n {
        let wlalsd = wlalsd(n, nrhs, nlvl);
        optimal_real = optimal_real.max((3 * n + (mm + n) * BLOCK_SIZE) as isize);
        optimal_real = optimal_real.max((3 * n + nrhs * BLOCK_SIZE) as isize);
        // Preserve the signed `(N-1)*NB` expression for the N=0 query.
        optimal_real = optimal_real.max(3 * n as isize + (n as isize - 1) * BLOCK_SIZE as isize);
        optimal_real = optimal_real.max((3 * n + wlalsd) as isize);
        minimum_real = (3 * n + mm).max(3 * n + nrhs).max(3 * n + wlalsd) as isize;
    }

    if n > m {
        let wlalsd = wlalsd(m, nrhs, nlvl);
        if n >= mnthr {
            optimal_real = (m + m * BLOCK_SIZE) as isize;
            optimal_real = optimal_real.max((m * m + 4 * m + 2 * m * BLOCK_SIZE) as isize);
            optimal_real = optimal_real.max((m * m + 4 * m + nrhs * BLOCK_SIZE) as isize);
            optimal_real = optimal_real.max(
                m as isize * m as isize + 4 * m as isize + (m as isize - 1) * BLOCK_SIZE as isize,
            );
            optimal_real = optimal_real.max(if nrhs > 1 {
                (m * m + m + m * nrhs) as isize
            } else {
                (m * m + 2 * m) as isize
            });
            optimal_real = optimal_real.max((m + nrhs * BLOCK_SIZE) as isize);
            optimal_real = optimal_real.max((m * m + 4 * m + wlalsd) as isize);
            let scratch = (m as isize)
                .max(2 * m as isize - 4)
                .max(nrhs as isize)
                .max(n as isize - 3 * m as isize);
            optimal_real = optimal_real.max(4 * m as isize + (m * m) as isize + scratch);
        } else {
            optimal_real = (3 * m + (n + m) * BLOCK_SIZE) as isize;
            optimal_real = optimal_real.max((3 * m + nrhs * BLOCK_SIZE) as isize);
            optimal_real = optimal_real.max((3 * m + m * BLOCK_SIZE) as isize);
            optimal_real = optimal_real.max((3 * m + wlalsd) as isize);
        }
        minimum_real = (3 * m + nrhs).max(3 * m + m).max(3 * m + wlalsd) as isize;
    }

    minimum_real = minimum_real.min(optimal_real);
    DgelsdWorkspace {
        minimum_real: minimum_real.max(1) as usize,
        optimal_real: optimal_real.max(1) as usize,
        minimum_integer: minimum_integer.max(1),
    }
}

fn maximum_norm(rows: usize, columns: usize, a: &[f64], lda: usize) -> f64 {
    let mut norm = 0.0_f64;
    for column in 0..columns {
        for row in 0..rows {
            // `max` has LAPACK's desired NaN propagation when the matrix
            // element is the second operand.
            let value = a[at(row, column, lda)].abs();
            norm = if value.is_nan() {
                value
            } else {
                norm.max(value)
            };
        }
    }
    norm
}

fn set_zero(rows: usize, columns: usize, a: &mut [f64], lda: usize) {
    for column in 0..columns {
        for row in 0..rows {
            a[at(row, column, lda)] = 0.0;
        }
    }
}

fn copy_lower(order: usize, source: &[f64], lda: usize, target: &mut [f64], ldt: usize) {
    for column in 0..order {
        for row in 0..order {
            target[at(row, column, ldt)] = if row >= column {
                source[at(row, column, lda)]
            } else {
                0.0
            };
        }
    }
}

fn check_internal(routine: &'static str, info: i32) -> Result<(), DgelsdError> {
    if info == 0 {
        Ok(())
    } else if info > 0 {
        Err(DgelsdError::DidNotConverge {
            unconverged: info as usize,
        })
    } else {
        Err(DgelsdError::InternalRoutine { routine, info })
    }
}

#[allow(clippy::too_many_arguments)]
fn call_dlascl(
    cfrom: f64,
    cto: f64,
    rows: usize,
    columns: usize,
    a: &mut [f64],
    lda: usize,
) -> i32 {
    let mut kind = b'G' as core::ffi::c_char;
    let mut kl = 0 as core::ffi::c_long;
    let mut ku = 0 as core::ffi::c_long;
    let mut cfrom = cfrom;
    let mut cto = cto;
    let mut rows = rows as core::ffi::c_long;
    let mut columns = columns as core::ffi::c_long;
    let mut lda = lda as core::ffi::c_long;
    let mut info = 0 as core::ffi::c_long;
    unsafe {
        dlascl_(
            &mut kind,
            &mut kl,
            &mut ku,
            &mut cfrom,
            &mut cto,
            &mut rows,
            &mut columns,
            a.as_mut_ptr(),
            &mut lda,
            &mut info,
        );
    }
    info as i32
}

#[allow(clippy::too_many_arguments)]
fn call_dlalsd(
    upper: bool,
    n: usize,
    nrhs: usize,
    d: &mut [f64],
    e: &mut [f64],
    b: &mut [f64],
    ldb: usize,
    rcond: f64,
    rank: &mut usize,
    work: &mut [f64],
    iwork: &mut [core::ffi::c_long],
) -> i32 {
    call_dlalsd_with_smlsiz(
        upper, SMLSIZ, n, nrhs, d, e, b, ldb, rcond, rank, work, iwork,
    )
}

#[allow(clippy::too_many_arguments)]
fn call_dlalsd_with_smlsiz(
    upper: bool,
    smlsiz: usize,
    n: usize,
    nrhs: usize,
    d: &mut [f64],
    e: &mut [f64],
    b: &mut [f64],
    ldb: usize,
    rcond: f64,
    rank: &mut usize,
    work: &mut [f64],
    iwork: &mut [core::ffi::c_long],
) -> i32 {
    let mut uplo = if upper { b'U' } else { b'L' } as core::ffi::c_char;
    let mut smlsiz = smlsiz as core::ffi::c_long;
    let mut n = n as core::ffi::c_long;
    let mut nrhs = nrhs as core::ffi::c_long;
    let mut ldb = ldb as core::ffi::c_long;
    let mut rcond = rcond;
    let mut raw_rank = 0 as core::ffi::c_long;
    let mut info = 0 as core::ffi::c_long;
    unsafe {
        dlalsd_(
            &mut uplo,
            &mut smlsiz,
            &mut n,
            &mut nrhs,
            d.as_mut_ptr(),
            e.as_mut_ptr(),
            b.as_mut_ptr(),
            &mut ldb,
            &mut rcond,
            &mut raw_rank,
            work.as_mut_ptr(),
            iwork.as_mut_ptr(),
            &mut info,
        );
    }
    *rank = raw_rank.max(0) as usize;
    info as i32
}

/// Compute the minimum-norm solution with LAPACK 3.12.1 `DGELSD` semantics.
///
/// Matrices are column-major. `B` must have `max(m,n)` addressable rows; on
/// success its first `n` rows contain the solution. A workspace query uses
/// `lwork=-1`, needs one element in both workspace slices, and writes the
/// optimal real and minimum integer lengths into their first elements.
#[allow(clippy::too_many_arguments)]
pub fn dgelsd(
    m: usize,
    n: usize,
    nrhs: usize,
    a: &mut [f64],
    lda: usize,
    b: &mut [f64],
    ldb: usize,
    s: &mut [f64],
    rcond: f64,
    rank: &mut usize,
    work: &mut [f64],
    lwork: isize,
    iwork: &mut [core::ffi::c_long],
) -> Result<(), DgelsdError> {
    let minmn = m.min(n);
    let maxmn = m.max(n);
    let query = lwork == -1;
    let workspace = dgelsd_workspace(m, n, nrhs);

    if lda < m.max(1) {
        return Err(DgelsdError::InvalidLeadingDimension {
            argument: 5,
            minimum: m.max(1),
            actual: lda,
        });
    }
    if ldb < maxmn.max(1) {
        return Err(DgelsdError::InvalidLeadingDimension {
            argument: 7,
            minimum: maxmn.max(1),
            actual: ldb,
        });
    }
    if !query && lwork < workspace.minimum_real as isize {
        return Err(DgelsdError::InvalidWorkspace {
            minimum: workspace.minimum_real,
            actual: lwork,
        });
    }

    let required_a = matrix_len(m, n, lda);
    if a.len() < required_a {
        return Err(DgelsdError::BufferTooShort {
            argument: 4,
            minimum: required_a,
            actual: a.len(),
        });
    }
    let required_b = matrix_len(maxmn, nrhs, ldb);
    if b.len() < required_b {
        return Err(DgelsdError::BufferTooShort {
            argument: 6,
            minimum: required_b,
            actual: b.len(),
        });
    }
    if s.len() < minmn {
        return Err(DgelsdError::BufferTooShort {
            argument: 8,
            minimum: minmn,
            actual: s.len(),
        });
    }
    let required_work = if query { 1 } else { lwork as usize };
    if work.len() < required_work {
        return Err(DgelsdError::BufferTooShort {
            argument: 11,
            minimum: required_work,
            actual: work.len(),
        });
    }
    let required_iwork = if query { 1 } else { workspace.minimum_integer };
    if iwork.len() < required_iwork {
        return Err(DgelsdError::BufferTooShort {
            argument: 13,
            minimum: required_iwork,
            actual: iwork.len(),
        });
    }

    work[0] = workspace.optimal_real as f64;
    iwork[0] = workspace.minimum_integer as core::ffi::c_long;
    if query {
        return Ok(());
    }
    if m == 0 || n == 0 {
        *rank = 0;
        return Ok(());
    }

    let eps = dlamch('P');
    let sfmin = dlamch('S');
    let smlnum = sfmin / eps;
    let bignum = 1.0 / smlnum;

    let anrm = maximum_norm(m, n, a, lda);
    let iascl = if anrm > 0.0 && anrm < smlnum {
        check_internal("DLASCL(A up)", call_dlascl(anrm, smlnum, m, n, a, lda))?;
        1
    } else if anrm > bignum {
        check_internal("DLASCL(A down)", call_dlascl(anrm, bignum, m, n, a, lda))?;
        2
    } else if anrm == 0.0 {
        set_zero(maxmn, nrhs, b, ldb);
        s[..minmn].fill(0.0);
        *rank = 0;
        work[0] = workspace.optimal_real as f64;
        iwork[0] = workspace.minimum_integer as core::ffi::c_long;
        return Ok(());
    } else {
        0
    };

    let bnrm = maximum_norm(m, nrhs, b, ldb);
    let ibscl = if bnrm > 0.0 && bnrm < smlnum {
        check_internal("DLASCL(B up)", call_dlascl(bnrm, smlnum, m, nrhs, b, ldb))?;
        1
    } else if bnrm > bignum {
        check_internal("DLASCL(B down)", call_dlascl(bnrm, bignum, m, nrhs, b, ldb))?;
        2
    } else {
        0
    };

    if m < n {
        for rhs in 0..nrhs {
            for row in m..n {
                b[at(row, rhs, ldb)] = 0.0;
            }
        }
    }

    let mnthr = (minmn as f64 * 1.6) as usize;
    let nlvl = levels(minmn);
    let wlalsd = wlalsd(minmn, nrhs, nlvl);

    if m >= n {
        let mut mm = m;
        if m >= mnthr {
            mm = n;
            let (tau, subwork) = work.split_at_mut(n);
            check_internal(
                "DGEQRF",
                dgeqrf(m, n, a, lda, tau, subwork, subwork.len() as isize),
            )?;
            check_internal(
                "DORMQR",
                dormqr(
                    b'L',
                    b'T',
                    m,
                    nrhs,
                    n,
                    a,
                    lda,
                    tau,
                    b,
                    ldb,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;
            // DLASET('L',N-1,N-1,A(2,1)): clear strictly below R.
            for column in 0..n.saturating_sub(1) {
                for row in column + 1..n {
                    a[at(row, column, lda)] = 0.0;
                }
            }
        }

        let (e, tail) = work.split_at_mut(n);
        let (tauq, tail) = tail.split_at_mut(n);
        let (taup, subwork) = tail.split_at_mut(n);
        check_internal(
            "DGEBRD",
            dgebrd(
                mm,
                n,
                a,
                lda,
                &mut s[..n],
                e,
                tauq,
                taup,
                subwork,
                subwork.len() as isize,
            ),
        )?;
        check_internal(
            "DORMBR(Q)",
            dormbr(
                b'Q',
                b'L',
                b'T',
                mm,
                nrhs,
                n,
                a,
                lda,
                tauq,
                b,
                ldb,
                subwork,
                subwork.len() as isize,
            ),
        )?;
        let info = call_dlalsd(
            true,
            n,
            nrhs,
            &mut s[..n],
            &mut e[..n.saturating_sub(1)],
            b,
            ldb,
            rcond,
            rank,
            subwork,
            iwork,
        );
        check_internal("DLALSD", info)?;
        check_internal(
            "DORMBR(P)",
            dormbr(
                b'P',
                b'L',
                b'N',
                n,
                nrhs,
                n,
                a,
                lda,
                taup,
                b,
                ldb,
                subwork,
                subwork.len() as isize,
            ),
        )?;
    } else {
        let path_2a_need = 4 * m
            + m * m
            + m.max(2 * m.saturating_sub(2))
                .max(nrhs)
                .max(n.saturating_sub(3 * m))
                .max(wlalsd);
        if n >= mnthr && lwork as usize >= path_2a_need {
            let ldwork_need = (4 * m
                + m * lda
                + m.max(2 * m.saturating_sub(2))
                    .max(nrhs)
                    .max(n.saturating_sub(3 * m)))
            .max(m * lda + m + m * nrhs)
            .max(4 * m + m * lda + wlalsd);
            let ldwork = if lwork as usize >= ldwork_need {
                lda
            } else {
                m
            };

            {
                let (tau, subwork) = work.split_at_mut(m);
                check_internal(
                    "DGELQF",
                    dgelqf(m, n, a, lda, tau, subwork, subwork.len() as isize),
                )?;
            }

            let (_, after_tau) = work.split_at_mut(m);
            let (lower, tail) = after_tau.split_at_mut(ldwork * m);
            copy_lower(m, a, lda, lower, ldwork);
            let (e, tail) = tail.split_at_mut(m);
            let (tauq, tail) = tail.split_at_mut(m);
            let (taup, subwork) = tail.split_at_mut(m);
            check_internal(
                "DGEBRD",
                dgebrd(
                    m,
                    m,
                    lower,
                    ldwork,
                    &mut s[..m],
                    e,
                    tauq,
                    taup,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;
            check_internal(
                "DORMBR(Q)",
                dormbr(
                    b'Q',
                    b'L',
                    b'T',
                    m,
                    nrhs,
                    m,
                    lower,
                    ldwork,
                    tauq,
                    b,
                    ldb,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;
            check_internal(
                "DLALSD",
                call_dlalsd(
                    true,
                    m,
                    nrhs,
                    &mut s[..m],
                    &mut e[..m.saturating_sub(1)],
                    b,
                    ldb,
                    rcond,
                    rank,
                    subwork,
                    iwork,
                ),
            )?;
            check_internal(
                "DORMBR(P)",
                dormbr(
                    b'P',
                    b'L',
                    b'N',
                    m,
                    nrhs,
                    m,
                    lower,
                    ldwork,
                    taup,
                    b,
                    ldb,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;

            for rhs in 0..nrhs {
                for row in m..n {
                    b[at(row, rhs, ldb)] = 0.0;
                }
            }
            let (tau, subwork) = work.split_at_mut(m);
            check_internal(
                "DORMLQ",
                dormlq(
                    b'L',
                    b'T',
                    n,
                    nrhs,
                    m,
                    a,
                    lda,
                    tau,
                    b,
                    ldb,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;
        } else {
            let (e, tail) = work.split_at_mut(m);
            let (tauq, tail) = tail.split_at_mut(m);
            let (taup, subwork) = tail.split_at_mut(m);
            check_internal(
                "DGEBRD",
                dgebrd(
                    m,
                    n,
                    a,
                    lda,
                    &mut s[..m],
                    e,
                    tauq,
                    taup,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;
            check_internal(
                "DORMBR(Q)",
                dormbr(
                    b'Q',
                    b'L',
                    b'T',
                    m,
                    nrhs,
                    n,
                    a,
                    lda,
                    tauq,
                    b,
                    ldb,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;
            check_internal(
                "DLALSD",
                call_dlalsd(
                    false,
                    m,
                    nrhs,
                    &mut s[..m],
                    &mut e[..m.saturating_sub(1)],
                    b,
                    ldb,
                    rcond,
                    rank,
                    subwork,
                    iwork,
                ),
            )?;
            check_internal(
                "DORMBR(P)",
                dormbr(
                    b'P',
                    b'L',
                    b'N',
                    n,
                    nrhs,
                    m,
                    a,
                    lda,
                    taup,
                    b,
                    ldb,
                    subwork,
                    subwork.len() as isize,
                ),
            )?;
        }
    }

    if iascl == 1 {
        check_internal(
            "DLASCL(solution A up)",
            call_dlascl(anrm, smlnum, n, nrhs, b, ldb),
        )?;
        check_internal(
            "DLASCL(singular A up)",
            call_dlascl(smlnum, anrm, minmn, 1, s, minmn),
        )?;
    } else if iascl == 2 {
        check_internal(
            "DLASCL(solution A down)",
            call_dlascl(anrm, bignum, n, nrhs, b, ldb),
        )?;
        check_internal(
            "DLASCL(singular A down)",
            call_dlascl(bignum, anrm, minmn, 1, s, minmn),
        )?;
    }
    if ibscl == 1 {
        check_internal(
            "DLASCL(solution B up)",
            call_dlascl(smlnum, bnrm, n, nrhs, b, ldb),
        )?;
    } else if ibscl == 2 {
        check_internal(
            "DLASCL(solution B down)",
            call_dlascl(bignum, bnrm, n, nrhs, b, ldb),
        )?;
    }

    work[0] = workspace.optimal_real as f64;
    iwork[0] = workspace.minimum_integer as core::ffi::c_long;
    Ok(())
}

/// Stage capture for the overdetermined QR-pre-reduction path. This is kept
/// behind `cfg(test)` so parity failures can be assigned to QR, bidiagonal
/// reduction, DLALSD, or the final right-vector application without changing
/// the production API.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct Path1aProbe {
    pub after_qr_a: Vec<f64>,
    pub after_qr_b: Vec<f64>,
    pub bidiagonal_d: Vec<f64>,
    pub bidiagonal_e: Vec<f64>,
    pub after_left_b: Vec<f64>,
    pub after_dlalsd_b: Vec<f64>,
    pub after_right_b: Vec<f64>,
    pub rank: usize,
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn probe_path_1a(
    m: usize,
    n: usize,
    nrhs: usize,
    a: Vec<f64>,
    lda: usize,
    b: Vec<f64>,
    ldb: usize,
    rcond: f64,
) -> Result<Path1aProbe, DgelsdError> {
    probe_path_1a_with_smlsiz(m, n, nrhs, a, lda, b, ldb, rcond, SMLSIZ)
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn probe_path_1a_with_smlsiz(
    m: usize,
    n: usize,
    nrhs: usize,
    mut a: Vec<f64>,
    lda: usize,
    mut b: Vec<f64>,
    ldb: usize,
    rcond: f64,
    smlsiz: usize,
) -> Result<Path1aProbe, DgelsdError> {
    assert!(m >= n && m >= (n as f64 * 1.6) as usize);
    let workspace = dgelsd_workspace(m, n, nrhs);
    let mut work = vec![0.0; workspace.optimal_real];
    let mut iwork = vec![0; workspace.minimum_integer];
    let (tau, subwork) = work.split_at_mut(n);
    check_internal(
        "DGEQRF",
        dgeqrf(m, n, &mut a, lda, tau, subwork, subwork.len() as isize),
    )?;
    check_internal(
        "DORMQR",
        dormqr(
            b'L',
            b'T',
            m,
            nrhs,
            n,
            &a,
            lda,
            tau,
            &mut b,
            ldb,
            subwork,
            subwork.len() as isize,
        ),
    )?;
    let after_qr_a = a.clone();
    let after_qr_b = b.clone();
    for column in 0..n.saturating_sub(1) {
        for row in column + 1..n {
            a[at(row, column, lda)] = 0.0;
        }
    }

    let (e, tail) = work.split_at_mut(n);
    let (tauq, tail) = tail.split_at_mut(n);
    let (taup, subwork) = tail.split_at_mut(n);
    let mut d = vec![0.0; n];
    check_internal(
        "DGEBRD",
        dgebrd(
            n,
            n,
            &mut a,
            lda,
            &mut d,
            e,
            tauq,
            taup,
            subwork,
            subwork.len() as isize,
        ),
    )?;
    let bidiagonal_d = d.clone();
    let bidiagonal_e = e[..n.saturating_sub(1)].to_vec();
    check_internal(
        "DORMBR(Q)",
        dormbr(
            b'Q',
            b'L',
            b'T',
            n,
            nrhs,
            n,
            &a,
            lda,
            tauq,
            &mut b,
            ldb,
            subwork,
            subwork.len() as isize,
        ),
    )?;
    let after_left_b = b.clone();
    let mut rank = 0;
    check_internal(
        "DLALSD",
        call_dlalsd_with_smlsiz(
            true,
            smlsiz,
            n,
            nrhs,
            &mut d,
            &mut e[..n.saturating_sub(1)],
            &mut b,
            ldb,
            rcond,
            &mut rank,
            subwork,
            &mut iwork,
        ),
    )?;
    let after_dlalsd_b = b.clone();
    check_internal(
        "DORMBR(P)",
        dormbr(
            b'P',
            b'L',
            b'N',
            n,
            nrhs,
            n,
            &a,
            lda,
            taup,
            &mut b,
            ldb,
            subwork,
            subwork.len() as isize,
        ),
    )?;

    Ok(Path1aProbe {
        after_qr_a,
        after_qr_b,
        bidiagonal_d,
        bidiagonal_e,
        after_left_b,
        after_dlalsd_b,
        after_right_b: b,
        rank,
    })
}
