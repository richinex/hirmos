//! Mechanical Rust translation of the LAPACK bidiagonal divide-and-conquer
//! dependency group.  The arithmetic/control flow below comes from the
//! CLAPACK f2c translation and is checked against LAPACK 3.12.1/SRC.
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#![allow(unused_assignments, unused_mut, unused_variables, improper_ctypes)]
#![allow(unsafe_op_in_unsafe_fn, static_mut_refs, clippy::all)]

type FInt = core::ffi::c_long;
type FDouble = core::ffi::c_double;
type FChar = core::ffi::c_char;
type FLogical = core::ffi::c_long;

#[inline]
unsafe fn char_upper(p: *mut FChar) -> u8 {
    if p.is_null() {
        0
    } else {
        (*p as u8).to_ascii_uppercase()
    }
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_lsame_(a: *mut FChar, b: *mut FChar) -> FLogical {
    (char_upper(a) == char_upper(b)) as FLogical
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_xerbla_(
    _name: *mut FChar,
    _info: *mut FInt,
) -> core::ffi::c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_dlamch_(cmach: *mut FChar) -> FDouble {
    super::blas::dlamch(char_upper(cmach) as char)
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_disnan_(x: *mut FDouble) -> FLogical {
    super::blas::disnan(*x) as FLogical
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_dlamc3_(a: *mut FDouble, b: *mut FDouble) -> FDouble {
    super::blas::dlamc3(core::ptr::read_volatile(a), core::ptr::read_volatile(b))
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_dlapy2_(x: *mut FDouble, y: *mut FDouble) -> FDouble {
    super::blas::dlapy2(*x, *y)
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_d_sign(a: *mut FDouble, b: *mut FDouble) -> FDouble {
    (*a).abs().copysign(*b)
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_pow_di(a: *mut FDouble, b: *mut FInt) -> FDouble {
    let mut x = *a;
    let mut n = *b;
    if n == 0 {
        return 1.0;
    }
    let invert = n < 0;
    if invert {
        n = -n;
    }
    let mut r = 1.0;
    while n != 0 {
        if n & 1 != 0 {
            r *= x;
        }
        n >>= 1;
        if n != 0 {
            x *= x;
        }
    }
    if invert {
        1.0 / r
    } else {
        r
    }
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_pow_dd(a: *mut FDouble, b: *mut FDouble) -> FDouble {
    (*a).powf(*b)
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_pow_ii(a: *mut FInt, b: *mut FInt) -> FInt {
    let mut x = *a;
    let mut n = *b;
    if n < 0 {
        return if x == 1 {
            1
        } else if x == -1 {
            if n & 1 == 0 {
                1
            } else {
                -1
            }
        } else {
            0
        };
    }
    let mut r = 1;
    while n != 0 {
        if n & 1 != 0 {
            r *= x;
        }
        n >>= 1;
        if n != 0 {
            x *= x;
        }
    }
    r
}

// Calls in the mechanical translation are rewritten to these Rust operations;
// exporting a C symbol named `sqrt` would recurse on native LLVM backends.
#[inline]
fn lapack_sqrt(x: FDouble) -> FDouble {
    x.sqrt()
}
#[inline]
fn lapack_log(x: FDouble) -> FDouble {
    x.ln()
}

#[inline]
unsafe fn step_ptr<T>(p: *mut T, i: FInt, inc: FInt) -> *mut T {
    p.offset((i * inc) as isize)
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_dcopy(
    n: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
    y: *mut FDouble,
    incy: *mut FInt,
) -> core::ffi::c_int {
    if *n <= 0 {
        return 0;
    }
    let mut ix = if *incx < 0 { (1 - *n) * *incx } else { 0 };
    let mut iy = if *incy < 0 { (1 - *n) * *incy } else { 0 };
    for _ in 0..*n {
        *y.offset(iy as isize) = *x.offset(ix as isize);
        ix += *incx;
        iy += *incy;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_dswap(
    n: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
    y: *mut FDouble,
    incy: *mut FInt,
) -> core::ffi::c_int {
    if *n <= 0 {
        return 0;
    }
    let mut ix = if *incx < 0 { (1 - *n) * *incx } else { 0 };
    let mut iy = if *incy < 0 { (1 - *n) * *incy } else { 0 };
    for _ in 0..*n {
        core::ptr::swap(x.offset(ix as isize), y.offset(iy as isize));
        ix += *incx;
        iy += *incy;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_dscal(
    n: *mut FInt,
    alpha: *mut FDouble,
    x: *mut FDouble,
    incx: *mut FInt,
) -> core::ffi::c_int {
    if *n <= 0 || *incx <= 0 {
        return 0;
    }
    let mut ix = 0;
    for _ in 0..*n {
        *x.offset(ix as isize) *= *alpha;
        ix += *incx;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_drot(
    n: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
    y: *mut FDouble,
    incy: *mut FInt,
    c: *mut FDouble,
    s: *mut FDouble,
) -> core::ffi::c_int {
    if *n <= 0 {
        return 0;
    }
    let mut ix = if *incx < 0 { (1 - *n) * *incx } else { 0 };
    let mut iy = if *incy < 0 { (1 - *n) * *incy } else { 0 };
    for _ in 0..*n {
        let xv = *x.offset(ix as isize);
        let yv = *y.offset(iy as isize);
        *x.offset(ix as isize) = *c * xv + *s * yv;
        *y.offset(iy as isize) = *c * yv - *s * xv;
        ix += *incx;
        iy += *incy;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_ddot(
    n: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
    y: *mut FDouble,
    incy: *mut FInt,
) -> FDouble {
    if *n <= 0 {
        return 0.0;
    }
    let nn = *n as usize;
    let xlen = 1 + (nn - 1) * (*incx).unsigned_abs() as usize;
    let ylen = 1 + (nn - 1) * (*incy).unsigned_abs() as usize;
    super::blas::ddot(
        nn,
        core::slice::from_raw_parts(x, xlen),
        *incx as isize,
        core::slice::from_raw_parts(y, ylen),
        *incy as isize,
    )
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_dnrm2(
    n: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
) -> FDouble {
    if *n < 1 || *incx < 1 {
        return 0.0;
    }
    let nn = *n as usize;
    let len = 1 + (nn - 1) * (*incx as usize);
    super::blas::dnrm2(nn, core::slice::from_raw_parts(x, len), *incx as isize)
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_idamax(
    n: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
) -> FInt {
    if *n < 1 || *incx <= 0 {
        return 0;
    }
    let nn = *n as usize;
    let len = 1 + (nn - 1) * (*incx as usize);
    super::blas::idamax(nn, core::slice::from_raw_parts(x, len), *incx as isize) as FInt
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_dgemv(
    trans: *mut FChar,
    m: *mut FInt,
    n: *mut FInt,
    alpha: *mut FDouble,
    a: *mut FDouble,
    lda: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
    beta: *mut FDouble,
    y: *mut FDouble,
    incy: *mut FInt,
) -> core::ffi::c_int {
    if *m < 0 || *n < 0 || *lda < (*m).max(1) || *incx == 0 || *incy == 0 {
        return -1;
    }
    let trans = match super::blas::Transpose::from_char(char_upper(trans) as char) {
        Some(x) => x,
        None => return -1,
    };
    let mm = *m as usize;
    let nn = *n as usize;
    let lda = *lda as usize;
    let (lenx, leny) = if trans == super::blas::Transpose::None {
        (nn, mm)
    } else {
        (mm, nn)
    };
    let xspan = if lenx == 0 {
        0
    } else {
        1 + (lenx - 1) * (*incx).unsigned_abs() as usize
    };
    let yspan = if leny == 0 {
        0
    } else {
        1 + (leny - 1) * (*incy).unsigned_abs() as usize
    };
    let alen = if nn == 0 { 0 } else { lda * nn };
    let result = super::blas::dgemv(
        trans,
        mm,
        nn,
        *alpha,
        core::slice::from_raw_parts(a, alen),
        lda,
        core::slice::from_raw_parts(x, xspan),
        *incx as isize,
        *beta,
        core::slice::from_raw_parts_mut(y, yspan),
        *incy as isize,
    );
    if result.is_ok() {
        0
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_f2c_dgemm(
    ta: *mut FChar,
    tb: *mut FChar,
    m: *mut FInt,
    n: *mut FInt,
    k: *mut FInt,
    alpha: *mut FDouble,
    a: *mut FDouble,
    lda: *mut FInt,
    b: *mut FDouble,
    ldb: *mut FInt,
    beta: *mut FDouble,
    c: *mut FDouble,
    ldc: *mut FInt,
) -> core::ffi::c_int {
    if *m < 0 || *n < 0 || *k < 0 {
        return -1;
    }
    let transa = match super::blas::Transpose::from_char(char_upper(ta) as char) {
        Some(x) => x,
        None => return -1,
    };
    let transb = match super::blas::Transpose::from_char(char_upper(tb) as char) {
        Some(x) => x,
        None => return -1,
    };
    let mm = *m as usize;
    let nn = *n as usize;
    let kk = *k as usize;
    let lda = *lda as usize;
    let ldb = *ldb as usize;
    let ldc = *ldc as usize;
    let acols = if transa == super::blas::Transpose::None {
        kk
    } else {
        mm
    };
    let bcols = if transb == super::blas::Transpose::None {
        nn
    } else {
        kk
    };
    let alen = lda.saturating_mul(acols);
    let blen = ldb.saturating_mul(bcols);
    let clen = ldc.saturating_mul(nn);
    let result = super::blas::dgemm(
        transa,
        transb,
        mm,
        nn,
        kk,
        *alpha,
        core::slice::from_raw_parts(a, alen),
        lda,
        core::slice::from_raw_parts(b, blen),
        ldb,
        *beta,
        core::slice::from_raw_parts_mut(c, clen),
        ldc,
    );
    if result.is_ok() {
        0
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_dlacpy_(
    uplo: *mut FChar,
    m: *mut FInt,
    n: *mut FInt,
    a: *mut FDouble,
    lda: *mut FInt,
    b: *mut FDouble,
    ldb: *mut FInt,
) -> core::ffi::c_int {
    let up = char_upper(uplo);
    for j in 0..*n {
        for i in 0..*m {
            if (up != b'U' && up != b'L') || (up == b'U' && i <= j) || (up == b'L' && i >= j) {
                *b.offset((i + j * *ldb) as isize) = *a.offset((i + j * *lda) as isize);
            }
        }
    }
    0
}

#[cfg(test)]
mod dlacpy_bridge_tests {
    use super::dgelsd_closure_dlacpy_;

    #[test]
    fn full_selector_copies_every_logical_entry() {
        let mut selector = b'F' as core::ffi::c_char;
        let mut rows = 2;
        let mut columns = 2;
        let mut leading_dimension = 3;
        let mut source = [1.0, 2.0, 99.0, 3.0, 4.0, 99.0];
        let mut destination = [-1.0; 6];

        unsafe {
            dgelsd_closure_dlacpy_(
                &mut selector,
                &mut rows,
                &mut columns,
                source.as_mut_ptr(),
                &mut leading_dimension,
                destination.as_mut_ptr(),
                &mut leading_dimension,
            );
        }

        assert_eq!(destination, [1.0, 2.0, -1.0, 3.0, 4.0, -1.0]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_dlassq_(
    n: *mut FInt,
    x: *mut FDouble,
    incx: *mut FInt,
    scale: *mut FDouble,
    sumsq: *mut FDouble,
) -> core::ffi::c_int {
    if *n <= 0 {
        return 0;
    }
    let nn = *n as usize;
    let len = 1 + (nn - 1) * (*incx).unsigned_abs() as usize;
    super::blas::dlassq(
        nn,
        core::slice::from_raw_parts(x, len),
        *incx as isize,
        &mut *scale,
        &mut *sumsq,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_ilaenv_(
    ispec: *mut FInt,
    _name: *mut FChar,
    _opts: *mut FChar,
    _n1: *mut FInt,
    _n2: *mut FInt,
    _n3: *mut FInt,
    _n4: *mut FInt,
) -> FInt {
    match *ispec {
        1 => 32,
        2 => 2,
        3 => 128,
        9 => 25,
        10 | 11 => 1,
        _ => 1,
    }
}

pub mod raw_dbdsqr {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    static mut c_b15: doublereal = -0.125f64;
    static mut c__1: integer = 1 as integer;
    static mut c_b49: doublereal = 1.0f64;
    static mut c_b72: doublereal = -1.0f64;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dbdsqr_(
        mut uplo: *mut ::core::ffi::c_char,
        mut n: *mut integer,
        mut ncvt: *mut integer,
        mut nru: *mut integer,
        mut ncc: *mut integer,
        mut d__: *mut doublereal,
        mut e: *mut doublereal,
        mut vt: *mut doublereal,
        mut ldvt: *mut integer,
        mut u: *mut doublereal,
        mut ldu: *mut integer,
        mut c__: *mut doublereal,
        mut ldc: *mut integer,
        mut work: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut current_block: u64;
        let mut c_dim1: integer = 0;
        let mut c_offset: integer = 0;
        let mut u_dim1: integer = 0;
        let mut u_offset: integer = 0;
        let mut vt_dim1: integer = 0;
        let mut vt_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut d__3: doublereal = 0.;
        let mut d__4: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_pow_dd"]
            fn pow_dd_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_d_sign"]
            fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
        }
        let mut f: doublereal = 0.;
        let mut g: doublereal = 0.;
        let mut h__: doublereal = 0.;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut m: integer = 0;
        let mut r__: doublereal = 0.;
        let mut cs: doublereal = 0.;
        let mut ll: integer = 0;
        let mut sn: doublereal = 0.;
        let mut mu: doublereal = 0.;
        let mut nm1: integer = 0;
        let mut nm12: integer = 0;
        let mut nm13: integer = 0;
        let mut lll: integer = 0;
        let mut eps: doublereal = 0.;
        let mut sll: doublereal = 0.;
        let mut tol: doublereal = 0.;
        let mut abse: doublereal = 0.;
        let mut idir: integer = 0;
        let mut abss: doublereal = 0.;
        let mut oldm: integer = 0;
        let mut cosl: doublereal = 0.;
        let mut isub: integer = 0;
        let mut iter: integer = 0;
        let mut unfl: doublereal = 0.;
        let mut sinl: doublereal = 0.;
        let mut cosr: doublereal = 0.;
        let mut smin: doublereal = 0.;
        let mut smax: doublereal = 0.;
        let mut sinr: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_drot"]
            fn f2c_drot_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlas2_"]
            fn dlas2__0(
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dscal"]
            fn f2c_dscal_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_lsame_"]
            fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
        }
        let mut oldcs: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlasr_"]
            fn dlasr__0(
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut oldll: integer = 0;
        let mut shift: doublereal = 0.;
        let mut sigmn: doublereal = 0.;
        let mut oldsn: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dswap"]
            fn f2c_dswap_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut iterdivn: integer = 0;
        let mut maxitdivn: integer = 0;
        let mut sminl: doublereal = 0.;
        let mut sigmx: doublereal = 0.;
        let mut lower: logical = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlasq1_"]
            fn dlasq1__0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasv2_"]
            fn dlasv2__0(
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlartg_"]
            fn dlartg__0(
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut sminoa: doublereal = 0.;
        let mut thresh: doublereal = 0.;
        let mut rotate: logical = 0;
        let mut tolmul: doublereal = 0.;
        d__ = d__.wrapping_offset(-1);
        e = e.wrapping_offset(-1);
        vt_dim1 = *ldvt;
        vt_offset = 1 as integer + vt_dim1;
        vt = vt.wrapping_offset(-(vt_offset as isize));
        u_dim1 = *ldu;
        u_offset = 1 as integer + u_dim1;
        u = u.wrapping_offset(-(u_offset as isize));
        c_dim1 = *ldc;
        c_offset = 1 as integer + c_dim1;
        c__ = c__.wrapping_offset(-(c_offset as isize));
        work = work.wrapping_offset(-1);
        *info = 0 as integer;
        lower = lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
            && lower == 0
        {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *n < 0 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *ncvt < 0 as ::core::ffi::c_long {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *nru < 0 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if *ncc < 0 as ::core::ffi::c_long {
            *info = -(5 as ::core::ffi::c_int) as integer;
        } else if *ncvt == 0 as ::core::ffi::c_long && *ldvt < 1 as ::core::ffi::c_long
            || *ncvt > 0 as ::core::ffi::c_long
                && *ldvt
                    < (if 1 as ::core::ffi::c_long >= *n {
                        1 as ::core::ffi::c_long
                    } else {
                        *n
                    })
        {
            *info = -(9 as ::core::ffi::c_int) as integer;
        } else if *ldu
            < (if 1 as ::core::ffi::c_long >= *nru {
                1 as ::core::ffi::c_long
            } else {
                *nru
            })
        {
            *info = -(11 as ::core::ffi::c_int) as integer;
        } else if *ncc == 0 as ::core::ffi::c_long && *ldc < 1 as ::core::ffi::c_long
            || *ncc > 0 as ::core::ffi::c_long
                && *ldc
                    < (if 1 as ::core::ffi::c_long >= *n {
                        1 as ::core::ffi::c_long
                    } else {
                        *n
                    })
        {
            *info = -(13 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DBDSQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        if *n == 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        if *n == 1 as ::core::ffi::c_long {
            current_block = 7225948839355843149;
        } else {
            rotate = (*ncvt > 0 as ::core::ffi::c_long
                || *nru > 0 as ::core::ffi::c_long
                || *ncc > 0 as ::core::ffi::c_long) as ::core::ffi::c_int
                as logical;
            if rotate == 0 {
                dlasq1__0(
                    n,
                    d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
                return 0 as ::core::ffi::c_int;
            }
            nm1 = (*n - 1 as ::core::ffi::c_long) as integer;
            nm12 = nm1 + nm1;
            nm13 = nm12 + nm1;
            idir = 0 as integer;
            eps = dlamch__0(
                b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            unfl = dlamch__0(
                b"Safe minimum\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            if lower != 0 {
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    dlartg__0(
                        d__.offset(i__ as isize) as *mut doublereal,
                        e.offset(i__ as isize) as *mut doublereal,
                        &raw mut cs,
                        &raw mut sn,
                        &raw mut r__,
                    );
                    *d__.offset(i__ as isize) = r__;
                    *e.offset(i__ as isize) = sn
                        * *d__.offset(
                            (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        );
                    *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                        cs * *d__.offset(
                            (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        );
                    *work.offset(i__ as isize) = cs;
                    *work.offset((nm1 + i__) as isize) = sn;
                    i__ += 1;
                }
                if *nru > 0 as ::core::ffi::c_long {
                    dlasr__0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        nru,
                        n,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        work.offset(*n as isize) as *mut doublereal,
                        u.offset(u_offset as isize) as *mut doublereal,
                        ldu,
                    );
                }
                if *ncc > 0 as ::core::ffi::c_long {
                    dlasr__0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        n,
                        ncc,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        work.offset(*n as isize) as *mut doublereal,
                        c__.offset(c_offset as isize) as *mut doublereal,
                        ldc,
                    );
                }
            }
            d__3 = 100.0f64 as doublereal;
            d__4 = pow_dd_0(&raw mut eps, &raw mut c_b15) as doublereal;
            d__1 = 10.0f64 as doublereal;
            d__2 = (if d__3 <= d__4 {
                d__3 as ::core::ffi::c_double
            } else {
                d__4 as ::core::ffi::c_double
            }) as doublereal;
            tolmul = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            tol = tolmul * eps;
            smax = 0.0f64 as doublereal;
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                d__2 = smax;
                d__1 = *d__.offset(i__ as isize);
                d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                smax = (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) as doublereal;
                i__ += 1;
            }
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__1 {
                d__2 = smax;
                d__1 = *e.offset(i__ as isize);
                d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                smax = (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) as doublereal;
                i__ += 1;
            }
            sminl = 0.0f64 as doublereal;
            if tol >= 0.0f64 {
                sminoa = (if *d__.offset(1 as ::core::ffi::c_int as isize)
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                } else {
                    -(*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                }) as doublereal;
                if !(sminoa == 0.0f64) {
                    mu = sminoa;
                    i__1 = *n;
                    i__ = 2 as integer;
                    while i__ <= i__1 {
                        d__2 = *d__.offset(i__ as isize);
                        d__1 = *e.offset(
                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        );
                        mu = ((if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) * (mu as ::core::ffi::c_double
                            / (mu as ::core::ffi::c_double
                                + (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                })))) as doublereal;
                        sminoa = (if sminoa <= mu {
                            sminoa as ::core::ffi::c_double
                        } else {
                            mu as ::core::ffi::c_double
                        }) as doublereal;
                        if sminoa == 0.0f64 {
                            break;
                        }
                        i__ += 1;
                    }
                }
                sminoa /= super::lapack_sqrt(*n as doublereal);
                d__1 = tol * sminoa;
                d__2 = (*n * 6 as integer * *n) as doublereal * unfl;
                thresh = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
            } else {
                d__1 = (if tol >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    tol
                } else {
                    -tol
                }) * smax;
                d__2 = (*n * 6 as integer * *n) as doublereal * unfl;
                thresh = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
            }
            // LAPACK 3.12.1's overflow-safe form of MAXITR*N**2. The
            // pre-2017 CLAPACK translation multiplied all three terms in one
            // integer and could overflow for N > 18,919.
            maxitdivn = 6 as integer * *n;
            iterdivn = 0 as integer;
            iter = -(1 as integer);
            oldll = -(1 as ::core::ffi::c_int) as integer;
            oldm = -(1 as ::core::ffi::c_int) as integer;
            m = *n;
            '_L60: loop {
                if m <= 1 as ::core::ffi::c_long {
                    current_block = 7225948839355843149;
                    break;
                }
                if iter >= *n {
                    iter -= *n;
                    iterdivn += 1;
                    if iterdivn >= maxitdivn {
                        *info = 0 as integer;
                        i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                        i__ = 1 as integer;
                        while i__ <= i__1 {
                            if *e.offset(i__ as isize) != 0.0f64 {
                                *info += 1;
                            }
                            i__ += 1;
                        }
                        current_block = 4690563065387989088;
                        break;
                    }
                }
                {
                    if tol < 0.0f64 && {
                        d__1 = *d__.offset(m as isize);
                        (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) <= thresh
                    } {
                        *d__.offset(m as isize) = 0.0f64 as doublereal;
                    }
                    d__1 = *d__.offset(m as isize);
                    smax = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                    smin = smax;
                    i__1 = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    lll = 1 as integer;
                    loop {
                        if !(lll <= i__1) {
                            current_block = 2956972668325154207;
                            break;
                        }
                        ll = m - lll;
                        d__1 = *d__.offset(ll as isize);
                        abss = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__1 = *e.offset(ll as isize);
                        abse = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        if tol < 0.0f64 && abss <= thresh {
                            *d__.offset(ll as isize) = 0.0f64 as doublereal;
                        }
                        if abse <= thresh {
                            current_block = 9046956821254779806;
                            break;
                        }
                        smin = (if smin <= abss {
                            smin as ::core::ffi::c_double
                        } else {
                            abss as ::core::ffi::c_double
                        }) as doublereal;
                        d__1 = (if smax >= abss {
                            smax as ::core::ffi::c_double
                        } else {
                            abss as ::core::ffi::c_double
                        }) as doublereal;
                        smax = (if d__1 >= abse {
                            d__1 as ::core::ffi::c_double
                        } else {
                            abse as ::core::ffi::c_double
                        }) as doublereal;
                        lll += 1;
                    }
                    match current_block {
                        2956972668325154207 => {
                            ll = 0 as integer;
                        }
                        _ => {
                            *e.offset(ll as isize) = 0.0f64 as doublereal;
                            if ll == m as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                                m -= 1;
                                continue;
                            }
                        }
                    }
                    ll += 1;
                    if ll == m as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                        dlasv2__0(
                            d__.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as *mut doublereal,
                            e.offset((m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                                as *mut doublereal,
                            d__.offset(m as isize) as *mut doublereal,
                            &raw mut sigmn,
                            &raw mut sigmx,
                            &raw mut sinr,
                            &raw mut cosr,
                            &raw mut sinl,
                            &raw mut cosl,
                        );
                        *d__.offset(
                            (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) = sigmx;
                        *e.offset((m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                            0.0f64 as doublereal;
                        *d__.offset(m as isize) = sigmn;
                        if *ncvt > 0 as ::core::ffi::c_long {
                            f2c_drot_0(
                                ncvt,
                                vt.offset((m - 1 as integer + vt_dim1) as isize) as *mut doublereal,
                                ldvt,
                                vt.offset((m + vt_dim1) as isize) as *mut doublereal,
                                ldvt,
                                &raw mut cosr,
                                &raw mut sinr,
                            );
                        }
                        if *nru > 0 as ::core::ffi::c_long {
                            f2c_drot_0(
                                nru,
                                u.offset(
                                    ((m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        * u_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                                u.offset(
                                    (m as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                                &raw mut cosl,
                                &raw mut sinl,
                            );
                        }
                        if *ncc > 0 as ::core::ffi::c_long {
                            f2c_drot_0(
                                ncc,
                                c__.offset((m - 1 as integer + c_dim1) as isize) as *mut doublereal,
                                ldc,
                                c__.offset((m + c_dim1) as isize) as *mut doublereal,
                                ldc,
                                &raw mut cosl,
                                &raw mut sinl,
                            );
                        }
                        m += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    } else {
                        if ll > oldm || m < oldll {
                            d__1 = *d__.offset(ll as isize);
                            d__2 = *d__.offset(m as isize);
                            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) >= (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__2 as ::core::ffi::c_double
                            } else {
                                -(d__2 as ::core::ffi::c_double)
                            }) {
                                idir = 1 as integer;
                            } else {
                                idir = 2 as integer;
                            }
                        }
                        if idir == 1 as ::core::ffi::c_long {
                            d__2 = *e.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            d__1 = *d__.offset(m as isize);
                            if (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__2 as ::core::ffi::c_double
                            } else {
                                -(d__2 as ::core::ffi::c_double)
                            }) <= (if tol >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                tol as ::core::ffi::c_double
                            } else {
                                -(tol as ::core::ffi::c_double)
                            }) * (if d__1
                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) || tol < 0.0f64 && {
                                d__3 = *e.offset(
                                    (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                );
                                (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__3 as ::core::ffi::c_double
                                } else {
                                    -(d__3 as ::core::ffi::c_double)
                                }) <= thresh
                            } {
                                *e.offset(
                                    (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) = 0.0f64 as doublereal;
                                continue;
                            } else if tol >= 0.0f64 {
                                d__1 = *d__.offset(ll as isize);
                                mu = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                }) as doublereal;
                                sminl = mu;
                                i__1 = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                    as integer;
                                lll = ll;
                                while lll <= i__1 {
                                    d__1 = *e.offset(lll as isize);
                                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    }) <= tol * mu
                                    {
                                        *e.offset(lll as isize) = 0.0f64 as doublereal;
                                        continue '_L60;
                                    } else {
                                        d__2 = *d__.offset(
                                            (lll as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                        d__1 = *e.offset(lll as isize);
                                        mu = ((if d__2
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            d__2 as ::core::ffi::c_double
                                        } else {
                                            -(d__2 as ::core::ffi::c_double)
                                        }) * (mu as ::core::ffi::c_double
                                            / (mu as ::core::ffi::c_double
                                                + (if d__1
                                                    >= 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                {
                                                    d__1 as ::core::ffi::c_double
                                                } else {
                                                    -(d__1 as ::core::ffi::c_double)
                                                }))))
                                            as doublereal;
                                        sminl = (if sminl <= mu {
                                            sminl as ::core::ffi::c_double
                                        } else {
                                            mu as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        lll += 1;
                                    }
                                }
                            }
                        } else {
                            d__2 = *e.offset(ll as isize);
                            d__1 = *d__.offset(ll as isize);
                            if (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__2 as ::core::ffi::c_double
                            } else {
                                -(d__2 as ::core::ffi::c_double)
                            }) <= (if tol >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                tol as ::core::ffi::c_double
                            } else {
                                -(tol as ::core::ffi::c_double)
                            }) * (if d__1
                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) || tol < 0.0f64 && {
                                d__3 = *e.offset(ll as isize);
                                (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__3 as ::core::ffi::c_double
                                } else {
                                    -(d__3 as ::core::ffi::c_double)
                                }) <= thresh
                            } {
                                *e.offset(ll as isize) = 0.0f64 as doublereal;
                                continue;
                            } else if tol >= 0.0f64 {
                                d__1 = *d__.offset(m as isize);
                                mu = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                }) as doublereal;
                                sminl = mu;
                                i__1 = ll;
                                lll = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                    as integer;
                                while lll >= i__1 {
                                    d__1 = *e.offset(lll as isize);
                                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    }) <= tol * mu
                                    {
                                        *e.offset(lll as isize) = 0.0f64 as doublereal;
                                        continue '_L60;
                                    } else {
                                        d__2 = *d__.offset(lll as isize);
                                        d__1 = *e.offset(lll as isize);
                                        mu = ((if d__2
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            d__2 as ::core::ffi::c_double
                                        } else {
                                            -(d__2 as ::core::ffi::c_double)
                                        }) * (mu as ::core::ffi::c_double
                                            / (mu as ::core::ffi::c_double
                                                + (if d__1
                                                    >= 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                {
                                                    d__1 as ::core::ffi::c_double
                                                } else {
                                                    -(d__1 as ::core::ffi::c_double)
                                                }))))
                                            as doublereal;
                                        sminl = (if sminl <= mu {
                                            sminl as ::core::ffi::c_double
                                        } else {
                                            mu as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        lll -= 1;
                                    }
                                }
                            }
                        }
                        oldll = ll;
                        oldm = m;
                        d__1 = eps;
                        d__2 = (tol as ::core::ffi::c_double * 0.01f64) as doublereal;
                        if tol >= 0.0f64
                            && *n as doublereal * tol * (sminl / smax)
                                <= (if d__1 >= d__2 {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    d__2 as ::core::ffi::c_double
                                })
                        {
                            shift = 0.0f64 as doublereal;
                        } else {
                            if idir == 1 as ::core::ffi::c_long {
                                d__1 = *d__.offset(ll as isize);
                                sll = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                }) as doublereal;
                                dlas2__0(
                                    d__.offset(
                                        (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    e.offset(
                                        (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    d__.offset(m as isize) as *mut doublereal,
                                    &raw mut shift,
                                    &raw mut r__,
                                );
                            } else {
                                d__1 = *d__.offset(m as isize);
                                sll = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                }) as doublereal;
                                dlas2__0(
                                    d__.offset(ll as isize) as *mut doublereal,
                                    e.offset(ll as isize) as *mut doublereal,
                                    d__.offset(
                                        (ll as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut shift,
                                    &raw mut r__,
                                );
                            }
                            if sll > 0.0f64 {
                                d__1 = shift / sll;
                                if d__1 * d__1 < eps {
                                    shift = 0.0f64 as doublereal;
                                }
                            }
                        }
                        iter = iter + m - ll;
                        if shift == 0.0f64 {
                            if idir == 1 as ::core::ffi::c_long {
                                cs = 1.0f64 as doublereal;
                                oldcs = 1.0f64 as doublereal;
                                i__1 = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                    as integer;
                                i__ = ll;
                                while i__ <= i__1 {
                                    d__1 = *d__.offset(i__ as isize) * cs;
                                    dlartg__0(
                                        &raw mut d__1,
                                        e.offset(i__ as isize) as *mut doublereal,
                                        &raw mut cs,
                                        &raw mut sn,
                                        &raw mut r__,
                                    );
                                    if i__ > ll {
                                        *e.offset(
                                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                                as isize,
                                        ) = oldsn * r__;
                                    }
                                    d__1 = oldcs * r__;
                                    d__2 = *d__.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) * sn;
                                    dlartg__0(
                                        &raw mut d__1,
                                        &raw mut d__2,
                                        &raw mut oldcs,
                                        &raw mut oldsn,
                                        d__.offset(i__ as isize) as *mut doublereal,
                                    );
                                    *work.offset(
                                        (i__ as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = cs;
                                    *work.offset((i__ - ll + 1 as integer + nm1) as isize) = sn;
                                    *work.offset((i__ - ll + 1 as integer + nm12) as isize) = oldcs;
                                    *work.offset((i__ - ll + 1 as integer + nm13) as isize) = oldsn;
                                    i__ += 1;
                                }
                                h__ = *d__.offset(m as isize) * cs;
                                *d__.offset(m as isize) = h__ * oldcs;
                                *e.offset(
                                    (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) = h__ * oldsn;
                                if *ncvt > 0 as ::core::ffi::c_long {
                                    i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as integer;
                                    dlasr__0(
                                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        &raw mut i__1,
                                        ncvt,
                                        work.offset(1 as ::core::ffi::c_int as isize)
                                            as *mut doublereal,
                                        work.offset(*n as isize) as *mut doublereal,
                                        vt.offset((ll + vt_dim1) as isize) as *mut doublereal,
                                        ldvt,
                                    );
                                }
                                if *nru > 0 as ::core::ffi::c_long {
                                    i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as integer;
                                    dlasr__0(
                                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        nru,
                                        &raw mut i__1,
                                        work.offset(
                                            (nm12 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        work.offset(
                                            (nm13 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        u.offset(
                                            (ll as ::core::ffi::c_long
                                                * u_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        ldu,
                                    );
                                }
                                if *ncc > 0 as ::core::ffi::c_long {
                                    i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as integer;
                                    dlasr__0(
                                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        &raw mut i__1,
                                        ncc,
                                        work.offset(
                                            (nm12 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        work.offset(
                                            (nm13 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        c__.offset((ll + c_dim1) as isize) as *mut doublereal,
                                        ldc,
                                    );
                                }
                                d__1 = *e.offset(
                                    (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                );
                                if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                }) <= thresh
                                {
                                    *e.offset(
                                        (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = 0.0f64 as doublereal;
                                }
                            } else {
                                cs = 1.0f64 as doublereal;
                                oldcs = 1.0f64 as doublereal;
                                i__1 = (ll as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                i__ = m;
                                while i__ >= i__1 {
                                    d__1 = *d__.offset(i__ as isize) * cs;
                                    dlartg__0(
                                        &raw mut d__1,
                                        e.offset(
                                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut cs,
                                        &raw mut sn,
                                        &raw mut r__,
                                    );
                                    if i__ < m {
                                        *e.offset(i__ as isize) = oldsn * r__;
                                    }
                                    d__1 = oldcs * r__;
                                    d__2 = *d__.offset(
                                        (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) * sn;
                                    dlartg__0(
                                        &raw mut d__1,
                                        &raw mut d__2,
                                        &raw mut oldcs,
                                        &raw mut oldsn,
                                        d__.offset(i__ as isize) as *mut doublereal,
                                    );
                                    *work.offset((i__ - ll) as isize) = cs;
                                    *work.offset((i__ - ll + nm1) as isize) = -sn;
                                    *work.offset((i__ - ll + nm12) as isize) = oldcs;
                                    *work.offset((i__ - ll + nm13) as isize) = -oldsn;
                                    i__ -= 1;
                                }
                                h__ = *d__.offset(ll as isize) * cs;
                                *d__.offset(ll as isize) = h__ * oldcs;
                                *e.offset(ll as isize) = h__ * oldsn;
                                if *ncvt > 0 as ::core::ffi::c_long {
                                    i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as integer;
                                    dlasr__0(
                                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"B\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        &raw mut i__1,
                                        ncvt,
                                        work.offset(
                                            (nm12 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        work.offset(
                                            (nm13 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        vt.offset((ll + vt_dim1) as isize) as *mut doublereal,
                                        ldvt,
                                    );
                                }
                                if *nru > 0 as ::core::ffi::c_long {
                                    i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as integer;
                                    dlasr__0(
                                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"B\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        nru,
                                        &raw mut i__1,
                                        work.offset(1 as ::core::ffi::c_int as isize)
                                            as *mut doublereal,
                                        work.offset(*n as isize) as *mut doublereal,
                                        u.offset(
                                            (ll as ::core::ffi::c_long
                                                * u_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        ldu,
                                    );
                                }
                                if *ncc > 0 as ::core::ffi::c_long {
                                    i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as integer;
                                    dlasr__0(
                                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"B\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        &raw mut i__1,
                                        ncc,
                                        work.offset(1 as ::core::ffi::c_int as isize)
                                            as *mut doublereal,
                                        work.offset(*n as isize) as *mut doublereal,
                                        c__.offset((ll + c_dim1) as isize) as *mut doublereal,
                                        ldc,
                                    );
                                }
                                d__1 = *e.offset(ll as isize);
                                if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                }) <= thresh
                                {
                                    *e.offset(ll as isize) = 0.0f64 as doublereal;
                                }
                            }
                        } else if idir == 1 as ::core::ffi::c_long {
                            d__1 = *d__.offset(ll as isize);
                            f = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }) - shift)
                                * (d_sign_0(
                                    &raw mut c_b49,
                                    d__.offset(ll as isize) as *mut doublereal,
                                ) as doublereal
                                    + shift / *d__.offset(ll as isize));
                            g = *e.offset(ll as isize);
                            i__1 = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            i__ = ll;
                            while i__ <= i__1 {
                                dlartg__0(
                                    &raw mut f,
                                    &raw mut g,
                                    &raw mut cosr,
                                    &raw mut sinr,
                                    &raw mut r__,
                                );
                                if i__ > ll {
                                    *e.offset(
                                        (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = r__;
                                }
                                f = cosr * *d__.offset(i__ as isize)
                                    + sinr * *e.offset(i__ as isize);
                                *e.offset(i__ as isize) = cosr * *e.offset(i__ as isize)
                                    - sinr * *d__.offset(i__ as isize);
                                g = sinr
                                    * *d__.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    );
                                *d__.offset(
                                    (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = cosr
                                    * *d__.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    );
                                dlartg__0(
                                    &raw mut f,
                                    &raw mut g,
                                    &raw mut cosl,
                                    &raw mut sinl,
                                    &raw mut r__,
                                );
                                *d__.offset(i__ as isize) = r__;
                                f = cosl * *e.offset(i__ as isize)
                                    + sinl
                                        * *d__.offset(
                                            (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                *d__.offset(
                                    (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) =
                                    cosl * *d__.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) - sinl * *e.offset(i__ as isize);
                                if i__ < m as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                                    g = sinl
                                        * *e.offset(
                                            (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                    *e.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = cosl
                                        * *e.offset(
                                            (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                }
                                *work.offset(
                                    (i__ as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = cosr;
                                *work.offset((i__ - ll + 1 as integer + nm1) as isize) = sinr;
                                *work.offset((i__ - ll + 1 as integer + nm12) as isize) = cosl;
                                *work.offset((i__ - ll + 1 as integer + nm13) as isize) = sinl;
                                i__ += 1;
                            }
                            *e.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) = f;
                            if *ncvt > 0 as ::core::ffi::c_long {
                                i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as integer;
                                dlasr__0(
                                    b"L\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"V\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"F\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    &raw mut i__1,
                                    ncvt,
                                    work.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut doublereal,
                                    work.offset(*n as isize) as *mut doublereal,
                                    vt.offset((ll + vt_dim1) as isize) as *mut doublereal,
                                    ldvt,
                                );
                            }
                            if *nru > 0 as ::core::ffi::c_long {
                                i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as integer;
                                dlasr__0(
                                    b"R\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"V\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"F\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    nru,
                                    &raw mut i__1,
                                    work.offset(
                                        (nm12 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    work.offset(
                                        (nm13 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    u.offset(
                                        (ll as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    ldu,
                                );
                            }
                            if *ncc > 0 as ::core::ffi::c_long {
                                i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as integer;
                                dlasr__0(
                                    b"L\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"V\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"F\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    &raw mut i__1,
                                    ncc,
                                    work.offset(
                                        (nm12 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    work.offset(
                                        (nm13 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    c__.offset((ll + c_dim1) as isize) as *mut doublereal,
                                    ldc,
                                );
                            }
                            d__1 = *e.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) <= thresh
                            {
                                *e.offset(
                                    (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) = 0.0f64 as doublereal;
                            }
                        } else {
                            d__1 = *d__.offset(m as isize);
                            f = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }) - shift)
                                * (d_sign_0(
                                    &raw mut c_b49,
                                    d__.offset(m as isize) as *mut doublereal,
                                ) as doublereal
                                    + shift / *d__.offset(m as isize));
                            g = *e.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            i__1 =
                                (ll as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                            i__ = m;
                            while i__ >= i__1 {
                                dlartg__0(
                                    &raw mut f,
                                    &raw mut g,
                                    &raw mut cosr,
                                    &raw mut sinr,
                                    &raw mut r__,
                                );
                                if i__ < m {
                                    *e.offset(i__ as isize) = r__;
                                }
                                f = cosr * *d__.offset(i__ as isize)
                                    + sinr
                                        * *e.offset(
                                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                *e.offset(
                                    (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) =
                                    cosr * *e.offset(
                                        (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) - sinr * *d__.offset(i__ as isize);
                                g = sinr
                                    * *d__.offset(
                                        (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    );
                                *d__.offset(
                                    (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = cosr
                                    * *d__.offset(
                                        (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    );
                                dlartg__0(
                                    &raw mut f,
                                    &raw mut g,
                                    &raw mut cosl,
                                    &raw mut sinl,
                                    &raw mut r__,
                                );
                                *d__.offset(i__ as isize) = r__;
                                f =
                                    cosl * *e.offset(
                                        (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) + sinl
                                        * *d__.offset(
                                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                *d__.offset(
                                    (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) =
                                    cosl * *d__.offset(
                                        (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) - sinl
                                        * *e.offset(
                                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                if i__ > ll as ::core::ffi::c_long + 1 as ::core::ffi::c_long {
                                    g = sinl
                                        * *e.offset(
                                            (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                    *e.offset(
                                        (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                            as isize,
                                    ) = cosl
                                        * *e.offset(
                                            (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                }
                                *work.offset((i__ - ll) as isize) = cosr;
                                *work.offset((i__ - ll + nm1) as isize) = -sinr;
                                *work.offset((i__ - ll + nm12) as isize) = cosl;
                                *work.offset((i__ - ll + nm13) as isize) = -sinl;
                                i__ -= 1;
                            }
                            *e.offset(ll as isize) = f;
                            d__1 = *e.offset(ll as isize);
                            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) <= thresh
                            {
                                *e.offset(ll as isize) = 0.0f64 as doublereal;
                            }
                            if *ncvt > 0 as ::core::ffi::c_long {
                                i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as integer;
                                dlasr__0(
                                    b"L\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"V\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"B\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    &raw mut i__1,
                                    ncvt,
                                    work.offset(
                                        (nm12 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    work.offset(
                                        (nm13 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    vt.offset((ll + vt_dim1) as isize) as *mut doublereal,
                                    ldvt,
                                );
                            }
                            if *nru > 0 as ::core::ffi::c_long {
                                i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as integer;
                                dlasr__0(
                                    b"R\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"V\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"B\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    nru,
                                    &raw mut i__1,
                                    work.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut doublereal,
                                    work.offset(*n as isize) as *mut doublereal,
                                    u.offset(
                                        (ll as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    ldu,
                                );
                            }
                            if *ncc > 0 as ::core::ffi::c_long {
                                i__1 = (m as ::core::ffi::c_long - ll as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as integer;
                                dlasr__0(
                                    b"L\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"V\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"B\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    &raw mut i__1,
                                    ncc,
                                    work.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut doublereal,
                                    work.offset(*n as isize) as *mut doublereal,
                                    c__.offset((ll + c_dim1) as isize) as *mut doublereal,
                                    ldc,
                                );
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            7225948839355843149 => {
                i__1 = *n;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    if *d__.offset(i__ as isize) < 0.0f64 {
                        *d__.offset(i__ as isize) = -*d__.offset(i__ as isize);
                        if *ncvt > 0 as ::core::ffi::c_long {
                            f2c_dscal_0(
                                ncvt,
                                &raw mut c_b72,
                                vt.offset((i__ + vt_dim1) as isize) as *mut doublereal,
                                ldvt,
                            );
                        }
                    }
                    i__ += 1;
                }
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    isub = 1 as integer;
                    smin = *d__.offset(1 as ::core::ffi::c_int as isize);
                    i__2 = *n + 1 as integer - i__;
                    j = 2 as integer;
                    while j <= i__2 {
                        if *d__.offset(j as isize) <= smin {
                            isub = j;
                            smin = *d__.offset(j as isize);
                        }
                        j += 1;
                    }
                    if isub != *n + 1 as integer - i__ {
                        *d__.offset(isub as isize) =
                            *d__.offset((*n + 1 as integer - i__) as isize);
                        *d__.offset((*n + 1 as integer - i__) as isize) = smin;
                        if *ncvt > 0 as ::core::ffi::c_long {
                            f2c_dswap_0(
                                ncvt,
                                vt.offset((isub + vt_dim1) as isize) as *mut doublereal,
                                ldvt,
                                vt.offset((*n + 1 as integer - i__ + vt_dim1) as isize)
                                    as *mut doublereal,
                                ldvt,
                            );
                        }
                        if *nru > 0 as ::core::ffi::c_long {
                            f2c_dswap_0(
                                nru,
                                u.offset(
                                    (isub as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                                u.offset(
                                    ((*n + 1 as ::core::ffi::c_long - i__ as ::core::ffi::c_long)
                                        * u_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                            );
                        }
                        if *ncc > 0 as ::core::ffi::c_long {
                            f2c_dswap_0(
                                ncc,
                                c__.offset((isub + c_dim1) as isize) as *mut doublereal,
                                ldc,
                                c__.offset((*n + 1 as integer - i__ + c_dim1) as isize)
                                    as *mut doublereal,
                                ldc,
                            );
                        }
                    }
                    i__ += 1;
                }
            }
            _ => {}
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dbdsqr::dgelsd_closure_dbdsqr_;

pub mod raw_dlalsd {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn log(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c__1: integer = 1 as integer;
    static mut c_b6: doublereal = 0.0f64;
    static mut c__0: integer = 0 as integer;
    static mut c_b11: doublereal = 1.0f64;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlalsd_(
        mut uplo: *mut ::core::ffi::c_char,
        mut smlsiz: *mut integer,
        mut n: *mut integer,
        mut nrhs: *mut integer,
        mut d__: *mut doublereal,
        mut e: *mut doublereal,
        mut b: *mut doublereal,
        mut ldb: *mut integer,
        mut rcond: *mut doublereal,
        mut rank: *mut integer,
        mut work: *mut doublereal,
        mut iwork: *mut integer,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut b_dim1: integer = 0;
        let mut b_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut d__1: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_d_sign"]
            fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
        }
        let mut c__: integer = 0;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut k: integer = 0;
        let mut r__: doublereal = 0.;
        let mut s: integer = 0;
        let mut u: integer = 0;
        let mut z__: integer = 0;
        let mut cs: doublereal = 0.;
        let mut bx: integer = 0;
        let mut sn: doublereal = 0.;
        let mut st: integer = 0;
        let mut vt: integer = 0;
        let mut nm1: integer = 0;
        let mut st1: integer = 0;
        let mut eps: doublereal = 0.;
        let mut iwk: integer = 0;
        let mut tol: doublereal = 0.;
        let mut difl: integer = 0;
        let mut difr: integer = 0;
        let mut rcnd: doublereal = 0.;
        let mut perm: integer = 0;
        let mut nsub: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_drot"]
            fn f2c_drot_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        let mut nlvl: integer = 0;
        let mut sqre: integer = 0;
        let mut bxst: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dgemm"]
            fn f2c_dgemm_0(
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut poles: integer = 0;
        let mut sizei: integer = 0;
        let mut nsize: integer = 0;
        let mut nwork: integer = 0;
        let mut icmpq1: integer = 0;
        let mut icmpq2: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasda_"]
            fn dlasda__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlalsa_"]
            fn dlalsa__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlascl_"]
            fn dlascl__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_idamax"]
            fn f2c_idamax_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> integer;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasdq_"]
            fn dlasdq__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlacpy_"]
            fn dlacpy__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlartg_"]
            fn dlartg__0(
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlaset_"]
            fn dlaset__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut givcol: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlanst_"]
            fn dlanst__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasrt_"]
            fn dlasrt__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut orgnrm: doublereal = 0.;
        let mut givnum: integer = 0;
        let mut givptr: integer = 0;
        let mut smlszp: integer = 0;
        d__ = d__.wrapping_offset(-1);
        e = e.wrapping_offset(-1);
        b_dim1 = *ldb;
        b_offset = 1 as integer + b_dim1;
        b = b.wrapping_offset(-(b_offset as isize));
        work = work.wrapping_offset(-1);
        iwork = iwork.wrapping_offset(-1);
        *info = 0 as integer;
        if *n < 0 as ::core::ffi::c_long {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *nrhs < 1 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if *ldb < 1 as ::core::ffi::c_long || *ldb < *n {
            *info = -(8 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLALSD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        eps = dlamch__0(
            b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if *rcond <= 0.0f64 || *rcond >= 1.0f64 {
            rcnd = eps;
        } else {
            rcnd = *rcond;
        }
        *rank = 0 as integer;
        if *n == 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        } else if *n == 1 as ::core::ffi::c_long {
            if *d__.offset(1 as ::core::ffi::c_int as isize) == 0.0f64 {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__1,
                    nrhs,
                    &raw mut c_b6,
                    &raw mut c_b6,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                );
            } else {
                *rank = 1 as integer;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c_b11,
                    &raw mut c__1,
                    nrhs,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                    info,
                );
                *d__.offset(1 as ::core::ffi::c_int as isize) = (if *d__
                    .offset(1 as ::core::ffi::c_int as isize)
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                } else {
                    -(*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                }) as doublereal;
            }
            return 0 as ::core::ffi::c_int;
        }
        if *(uplo as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == 'L' as i32 {
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__1 {
                dlartg__0(
                    d__.offset(i__ as isize) as *mut doublereal,
                    e.offset(i__ as isize) as *mut doublereal,
                    &raw mut cs,
                    &raw mut sn,
                    &raw mut r__,
                );
                *d__.offset(i__ as isize) = r__;
                *e.offset(i__ as isize) = sn
                    * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) = cs
                    * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                if *nrhs == 1 as ::core::ffi::c_long {
                    f2c_drot_0(
                        &raw mut c__1,
                        b.offset((i__ + b_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                        b.offset((i__ + 1 as integer + b_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut cs,
                        &raw mut sn,
                    );
                } else {
                    *work.offset(
                        (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as isize,
                    ) = cs;
                    *work
                        .offset((i__ as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize) =
                        sn;
                }
                i__ += 1;
            }
            if *nrhs > 1 as ::core::ffi::c_long {
                i__1 = *nrhs;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
                    j = 1 as integer;
                    while j <= i__2 {
                        cs = *work.offset(
                            (((j as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        );
                        sn = *work
                            .offset((j as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize);
                        f2c_drot_0(
                            &raw mut c__1,
                            b.offset((j + i__ * b_dim1) as isize) as *mut doublereal,
                            &raw mut c__1,
                            b.offset((j + 1 as integer + i__ * b_dim1) as isize) as *mut doublereal,
                            &raw mut c__1,
                            &raw mut cs,
                            &raw mut sn,
                        );
                        j += 1;
                    }
                    i__ += 1;
                }
            }
        }
        nm1 = (*n - 1 as ::core::ffi::c_long) as integer;
        orgnrm = dlanst__0(
            b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        if orgnrm == 0.0f64 {
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                nrhs,
                &raw mut c_b6,
                &raw mut c_b6,
                b.offset(b_offset as isize) as *mut doublereal,
                ldb,
            );
            return 0 as ::core::ffi::c_int;
        }
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut orgnrm,
            &raw mut c_b11,
            n,
            &raw mut c__1,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            n,
            info,
        );
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut orgnrm,
            &raw mut c_b11,
            &raw mut nm1,
            &raw mut c__1,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut nm1,
            info,
        );
        if *n <= *smlsiz {
            nwork = (*n * *n + 1 as ::core::ffi::c_long) as integer;
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b6,
                &raw mut c_b11,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
            );
            dlasdq__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                n,
                n,
                &raw mut c__0,
                nrhs,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
                b.offset(b_offset as isize) as *mut doublereal,
                ldb,
                work.offset(nwork as isize) as *mut doublereal,
                info,
            );
            if *info != 0 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            d__1 = *d__.offset(f2c_idamax_0(
                n,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            ) as isize);
            tol = (rcnd as ::core::ffi::c_double
                * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                })) as doublereal;
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                if *d__.offset(i__ as isize) <= tol {
                    dlaset__0(
                        b"A\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__1,
                        nrhs,
                        &raw mut c_b6,
                        &raw mut c_b6,
                        b.offset((i__ + b_dim1) as isize) as *mut doublereal,
                        ldb,
                    );
                } else {
                    dlascl__0(
                        b"G\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        d__.offset(i__ as isize) as *mut doublereal,
                        &raw mut c_b11,
                        &raw mut c__1,
                        nrhs,
                        b.offset((i__ + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        info,
                    );
                    *rank += 1;
                }
                i__ += 1;
            }
            f2c_dgemm_0(
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                nrhs,
                n,
                &raw mut c_b11,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
                b.offset(b_offset as isize) as *mut doublereal,
                ldb,
                &raw mut c_b6,
                work.offset(nwork as isize) as *mut doublereal,
                n,
            );
            dlacpy__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                nrhs,
                work.offset(nwork as isize) as *mut doublereal,
                n,
                b.offset(b_offset as isize) as *mut doublereal,
                ldb,
            );
            dlascl__0(
                b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                &raw mut c__0,
                &raw mut c_b11,
                &raw mut orgnrm,
                n,
                &raw mut c__1,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
                info,
            );
            dlasrt__0(
                b"D\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                info,
            );
            dlascl__0(
                b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                &raw mut c__0,
                &raw mut orgnrm,
                &raw mut c_b11,
                n,
                nrhs,
                b.offset(b_offset as isize) as *mut doublereal,
                ldb,
                info,
            );
            return 0 as ::core::ffi::c_int;
        }
        nlvl = ((super::lapack_log(
            *n as doublereal / (*smlsiz + 1 as ::core::ffi::c_long) as doublereal,
        ) / super::lapack_log(2.0f64)) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as integer;
        smlszp = (*smlsiz + 1 as ::core::ffi::c_long) as integer;
        u = 1 as integer;
        vt = (*smlsiz * *n + 1 as ::core::ffi::c_long) as integer;
        difl = vt + smlszp * *n;
        difr = difl + nlvl * *n;
        z__ = difr + (nlvl * *n << 1 as ::core::ffi::c_int);
        c__ = z__ + nlvl * *n;
        s = c__ + *n;
        poles = s + *n;
        givnum = poles + (nlvl << 1 as ::core::ffi::c_int) * *n;
        bx = givnum + (nlvl << 1 as ::core::ffi::c_int) * *n;
        nwork = bx + *n * *nrhs;
        sizei = (*n + 1 as ::core::ffi::c_long) as integer;
        k = sizei + *n;
        givptr = k + *n;
        perm = givptr + *n;
        givcol = perm + nlvl * *n;
        iwk = givcol + (nlvl * *n << 1 as ::core::ffi::c_int);
        st = 1 as integer;
        sqre = 0 as integer;
        icmpq1 = 1 as integer;
        icmpq2 = 0 as integer;
        nsub = 0 as integer;
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *d__.offset(i__ as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) < eps
            {
                *d__.offset(i__ as isize) =
                    d_sign_0(&raw mut eps, d__.offset(i__ as isize) as *mut doublereal)
                        as doublereal;
            }
            i__ += 1;
        }
        i__1 = nm1;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *e.offset(i__ as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) < eps
                || i__ == nm1
            {
                nsub += 1;
                *iwork.offset(nsub as isize) = st;
                if i__ < nm1 {
                    nsize = (i__ as ::core::ffi::c_long - st as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    *iwork.offset(
                        (sizei as ::core::ffi::c_long + nsub as ::core::ffi::c_long
                            - 1 as ::core::ffi::c_long) as isize,
                    ) = nsize;
                } else {
                    d__1 = *e.offset(i__ as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) >= eps
                    {
                        nsize =
                            (*n - st as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        *iwork.offset(
                            (sizei as ::core::ffi::c_long + nsub as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = nsize;
                    } else {
                        nsize = (i__ as ::core::ffi::c_long - st as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as integer;
                        *iwork.offset(
                            (sizei as ::core::ffi::c_long + nsub as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = nsize;
                        nsub += 1;
                        *iwork.offset(nsub as isize) = *n;
                        *iwork.offset(
                            (sizei as ::core::ffi::c_long + nsub as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = 1 as integer;
                        f2c_dcopy_0(
                            nrhs,
                            b.offset((*n + b_dim1) as isize) as *mut doublereal,
                            ldb,
                            work.offset((bx + nm1) as isize) as *mut doublereal,
                            n,
                        );
                    }
                }
                st1 = (st as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                if nsize == 1 as ::core::ffi::c_long {
                    f2c_dcopy_0(
                        nrhs,
                        b.offset((st + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        work.offset((bx + st1) as isize) as *mut doublereal,
                        n,
                    );
                } else if nsize <= *smlsiz {
                    dlaset__0(
                        b"A\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut nsize,
                        &raw mut nsize,
                        &raw mut c_b6,
                        &raw mut c_b11,
                        work.offset((vt + st1) as isize) as *mut doublereal,
                        n,
                    );
                    dlasdq__0(
                        b"U\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut nsize,
                        &raw mut nsize,
                        &raw mut c__0,
                        nrhs,
                        d__.offset(st as isize) as *mut doublereal,
                        e.offset(st as isize) as *mut doublereal,
                        work.offset((vt + st1) as isize) as *mut doublereal,
                        n,
                        work.offset(nwork as isize) as *mut doublereal,
                        n,
                        b.offset((st + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        work.offset(nwork as isize) as *mut doublereal,
                        info,
                    );
                    if *info != 0 as ::core::ffi::c_long {
                        return 0 as ::core::ffi::c_int;
                    }
                    dlacpy__0(
                        b"A\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut nsize,
                        nrhs,
                        b.offset((st + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        work.offset((bx + st1) as isize) as *mut doublereal,
                        n,
                    );
                } else {
                    dlasda__0(
                        &raw mut icmpq1,
                        smlsiz,
                        &raw mut nsize,
                        &raw mut sqre,
                        d__.offset(st as isize) as *mut doublereal,
                        e.offset(st as isize) as *mut doublereal,
                        work.offset((u + st1) as isize) as *mut doublereal,
                        n,
                        work.offset((vt + st1) as isize) as *mut doublereal,
                        iwork.offset((k + st1) as isize) as *mut integer,
                        work.offset((difl + st1) as isize) as *mut doublereal,
                        work.offset((difr + st1) as isize) as *mut doublereal,
                        work.offset((z__ + st1) as isize) as *mut doublereal,
                        work.offset((poles + st1) as isize) as *mut doublereal,
                        iwork.offset((givptr + st1) as isize) as *mut integer,
                        iwork.offset((givcol + st1) as isize) as *mut integer,
                        n,
                        iwork.offset((perm + st1) as isize) as *mut integer,
                        work.offset((givnum + st1) as isize) as *mut doublereal,
                        work.offset((c__ + st1) as isize) as *mut doublereal,
                        work.offset((s + st1) as isize) as *mut doublereal,
                        work.offset(nwork as isize) as *mut doublereal,
                        iwork.offset(iwk as isize) as *mut integer,
                        info,
                    );
                    if *info != 0 as ::core::ffi::c_long {
                        return 0 as ::core::ffi::c_int;
                    }
                    bxst = bx + st1;
                    dlalsa__0(
                        &raw mut icmpq2,
                        smlsiz,
                        &raw mut nsize,
                        nrhs,
                        b.offset((st + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        work.offset(bxst as isize) as *mut doublereal,
                        n,
                        work.offset((u + st1) as isize) as *mut doublereal,
                        n,
                        work.offset((vt + st1) as isize) as *mut doublereal,
                        iwork.offset((k + st1) as isize) as *mut integer,
                        work.offset((difl + st1) as isize) as *mut doublereal,
                        work.offset((difr + st1) as isize) as *mut doublereal,
                        work.offset((z__ + st1) as isize) as *mut doublereal,
                        work.offset((poles + st1) as isize) as *mut doublereal,
                        iwork.offset((givptr + st1) as isize) as *mut integer,
                        iwork.offset((givcol + st1) as isize) as *mut integer,
                        n,
                        iwork.offset((perm + st1) as isize) as *mut integer,
                        work.offset((givnum + st1) as isize) as *mut doublereal,
                        work.offset((c__ + st1) as isize) as *mut doublereal,
                        work.offset((s + st1) as isize) as *mut doublereal,
                        work.offset(nwork as isize) as *mut doublereal,
                        iwork.offset(iwk as isize) as *mut integer,
                        info,
                    );
                    if *info != 0 as ::core::ffi::c_long {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                st = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            }
            i__ += 1;
        }
        d__1 = *d__.offset(f2c_idamax_0(
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        ) as isize);
        tol = (rcnd as ::core::ffi::c_double
            * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            })) as doublereal;
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *d__.offset(i__ as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) <= tol
            {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__1,
                    nrhs,
                    &raw mut c_b6,
                    &raw mut c_b6,
                    work.offset(
                        (bx as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                            - 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    n,
                );
            } else {
                *rank += 1;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    d__.offset(i__ as isize) as *mut doublereal,
                    &raw mut c_b11,
                    &raw mut c__1,
                    nrhs,
                    work.offset(
                        (bx as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                            - 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    n,
                    info,
                );
            }
            d__1 = *d__.offset(i__ as isize);
            *d__.offset(i__ as isize) = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            i__ += 1;
        }
        icmpq2 = 1 as integer;
        i__1 = nsub;
        i__ = 1 as integer;
        while i__ <= i__1 {
            st = *iwork.offset(i__ as isize);
            st1 = (st as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            nsize = *iwork.offset(
                (sizei as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as isize,
            );
            bxst = bx + st1;
            if nsize == 1 as ::core::ffi::c_long {
                f2c_dcopy_0(
                    nrhs,
                    work.offset(bxst as isize) as *mut doublereal,
                    n,
                    b.offset((st + b_dim1) as isize) as *mut doublereal,
                    ldb,
                );
            } else if nsize <= *smlsiz {
                f2c_dgemm_0(
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nsize,
                    nrhs,
                    &raw mut nsize,
                    &raw mut c_b11,
                    work.offset((vt + st1) as isize) as *mut doublereal,
                    n,
                    work.offset(bxst as isize) as *mut doublereal,
                    n,
                    &raw mut c_b6,
                    b.offset((st + b_dim1) as isize) as *mut doublereal,
                    ldb,
                );
            } else {
                dlalsa__0(
                    &raw mut icmpq2,
                    smlsiz,
                    &raw mut nsize,
                    nrhs,
                    work.offset(bxst as isize) as *mut doublereal,
                    n,
                    b.offset((st + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    work.offset((u + st1) as isize) as *mut doublereal,
                    n,
                    work.offset((vt + st1) as isize) as *mut doublereal,
                    iwork.offset((k + st1) as isize) as *mut integer,
                    work.offset((difl + st1) as isize) as *mut doublereal,
                    work.offset((difr + st1) as isize) as *mut doublereal,
                    work.offset((z__ + st1) as isize) as *mut doublereal,
                    work.offset((poles + st1) as isize) as *mut doublereal,
                    iwork.offset((givptr + st1) as isize) as *mut integer,
                    iwork.offset((givcol + st1) as isize) as *mut integer,
                    n,
                    iwork.offset((perm + st1) as isize) as *mut integer,
                    work.offset((givnum + st1) as isize) as *mut doublereal,
                    work.offset((c__ + st1) as isize) as *mut doublereal,
                    work.offset((s + st1) as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(iwk as isize) as *mut integer,
                    info,
                );
                if *info != 0 as ::core::ffi::c_long {
                    return 0 as ::core::ffi::c_int;
                }
            }
            i__ += 1;
        }
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut c_b11,
            &raw mut orgnrm,
            n,
            &raw mut c__1,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            n,
            info,
        );
        dlasrt__0(
            b"D\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut orgnrm,
            &raw mut c_b11,
            n,
            nrhs,
            b.offset(b_offset as isize) as *mut doublereal,
            ldb,
            info,
        );
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlalsd::dgelsd_closure_dlalsd_;

pub mod raw_dlalsa {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c_b7: doublereal = 1.0f64;
    static mut c_b8: doublereal = 0.0f64;
    static mut c__2: integer = 2 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlalsa_(
        mut icompq: *mut integer,
        mut smlsiz: *mut integer,
        mut n: *mut integer,
        mut nrhs: *mut integer,
        mut b: *mut doublereal,
        mut ldb: *mut integer,
        mut bx: *mut doublereal,
        mut ldbx: *mut integer,
        mut u: *mut doublereal,
        mut ldu: *mut integer,
        mut vt: *mut doublereal,
        mut k: *mut integer,
        mut difl: *mut doublereal,
        mut difr: *mut doublereal,
        mut z__: *mut doublereal,
        mut poles: *mut doublereal,
        mut givptr: *mut integer,
        mut givcol: *mut integer,
        mut ldgcol: *mut integer,
        mut perm: *mut integer,
        mut givnum: *mut doublereal,
        mut c__: *mut doublereal,
        mut s: *mut doublereal,
        mut work: *mut doublereal,
        mut iwork: *mut integer,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut givcol_dim1: integer = 0;
        let mut givcol_offset: integer = 0;
        let mut perm_dim1: integer = 0;
        let mut perm_offset: integer = 0;
        let mut b_dim1: integer = 0;
        let mut b_offset: integer = 0;
        let mut bx_dim1: integer = 0;
        let mut bx_offset: integer = 0;
        let mut difl_dim1: integer = 0;
        let mut difl_offset: integer = 0;
        let mut difr_dim1: integer = 0;
        let mut difr_offset: integer = 0;
        let mut givnum_dim1: integer = 0;
        let mut givnum_offset: integer = 0;
        let mut poles_dim1: integer = 0;
        let mut poles_offset: integer = 0;
        let mut u_dim1: integer = 0;
        let mut u_offset: integer = 0;
        let mut vt_dim1: integer = 0;
        let mut vt_offset: integer = 0;
        let mut z_dim1: integer = 0;
        let mut z_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_pow_ii"]
            fn pow_ii_0(_: *mut integer, _: *mut integer) -> integer;
        }
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut i1: integer = 0;
        let mut ic: integer = 0;
        let mut lf: integer = 0;
        let mut nd: integer = 0;
        let mut ll: integer = 0;
        let mut nl: integer = 0;
        let mut nr: integer = 0;
        let mut im1: integer = 0;
        let mut nlf: integer = 0;
        let mut nrf: integer = 0;
        let mut lvl: integer = 0;
        let mut ndb1: integer = 0;
        let mut nlp1: integer = 0;
        let mut lvl2: integer = 0;
        let mut nrp1: integer = 0;
        let mut nlvl: integer = 0;
        let mut sqre: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dgemm"]
            fn f2c_dgemm_0(
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut inode: integer = 0;
        let mut ndiml: integer = 0;
        let mut ndimr: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlals0_"]
            fn dlals0__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasdt_"]
            fn dlasdt__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        b_dim1 = *ldb;
        b_offset = 1 as integer + b_dim1;
        b = b.wrapping_offset(-(b_offset as isize));
        bx_dim1 = *ldbx;
        bx_offset = 1 as integer + bx_dim1;
        bx = bx.wrapping_offset(-(bx_offset as isize));
        givnum_dim1 = *ldu;
        givnum_offset = 1 as integer + givnum_dim1;
        givnum = givnum.wrapping_offset(-(givnum_offset as isize));
        poles_dim1 = *ldu;
        poles_offset = 1 as integer + poles_dim1;
        poles = poles.wrapping_offset(-(poles_offset as isize));
        z_dim1 = *ldu;
        z_offset = 1 as integer + z_dim1;
        z__ = z__.wrapping_offset(-(z_offset as isize));
        difr_dim1 = *ldu;
        difr_offset = 1 as integer + difr_dim1;
        difr = difr.wrapping_offset(-(difr_offset as isize));
        difl_dim1 = *ldu;
        difl_offset = 1 as integer + difl_dim1;
        difl = difl.wrapping_offset(-(difl_offset as isize));
        vt_dim1 = *ldu;
        vt_offset = 1 as integer + vt_dim1;
        vt = vt.wrapping_offset(-(vt_offset as isize));
        u_dim1 = *ldu;
        u_offset = 1 as integer + u_dim1;
        u = u.wrapping_offset(-(u_offset as isize));
        k = k.wrapping_offset(-1);
        givptr = givptr.wrapping_offset(-1);
        perm_dim1 = *ldgcol;
        perm_offset = 1 as integer + perm_dim1;
        perm = perm.wrapping_offset(-(perm_offset as isize));
        givcol_dim1 = *ldgcol;
        givcol_offset = 1 as integer + givcol_dim1;
        givcol = givcol.wrapping_offset(-(givcol_offset as isize));
        c__ = c__.wrapping_offset(-1);
        s = s.wrapping_offset(-1);
        work = work.wrapping_offset(-1);
        iwork = iwork.wrapping_offset(-1);
        *info = 0 as integer;
        if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *smlsiz < 3 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *n < *smlsiz {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *nrhs < 1 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if *ldb < *n {
            *info = -(6 as ::core::ffi::c_int) as integer;
        } else if *ldbx < *n {
            *info = -(8 as ::core::ffi::c_int) as integer;
        } else if *ldu < *n {
            *info = -(10 as ::core::ffi::c_int) as integer;
        } else if *ldgcol < *n {
            *info = -(19 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLALSA\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        inode = 1 as integer;
        ndiml = inode + *n;
        ndimr = ndiml + *n;
        dlasdt__0(
            n,
            &raw mut nlvl,
            &raw mut nd,
            iwork.offset(inode as isize) as *mut integer,
            iwork.offset(ndiml as isize) as *mut integer,
            iwork.offset(ndimr as isize) as *mut integer,
            smlsiz,
        );
        if *icompq == 1 as ::core::ffi::c_long {
            j = 0 as integer;
            i__1 = nlvl;
            lvl = 1 as integer;
            while lvl <= i__1 {
                lvl2 = (((lvl as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_long) as integer;
                if lvl == 1 as ::core::ffi::c_long {
                    lf = 1 as integer;
                    ll = 1 as integer;
                } else {
                    i__2 = (lvl as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    lf = pow_ii_0(&raw mut c__2, &raw mut i__2);
                    ll = (((lf as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_long) as integer;
                }
                i__2 = lf;
                i__ = ll;
                while i__ >= i__2 {
                    im1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    ic = *iwork.offset((inode + im1) as isize);
                    nl = *iwork.offset((ndiml + im1) as isize);
                    nr = *iwork.offset((ndimr + im1) as isize);
                    nlf = ic - nl;
                    nrf = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    if i__ == ll {
                        sqre = 0 as integer;
                    } else {
                        sqre = 1 as integer;
                    }
                    j += 1;
                    dlals0__0(
                        icompq,
                        &raw mut nl,
                        &raw mut nr,
                        &raw mut sqre,
                        nrhs,
                        b.offset((nlf + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        bx.offset((nlf + bx_dim1) as isize) as *mut doublereal,
                        ldbx,
                        perm.offset((nlf + lvl * perm_dim1) as isize) as *mut integer,
                        givptr.offset(j as isize) as *mut integer,
                        givcol.offset((nlf + lvl2 * givcol_dim1) as isize) as *mut integer,
                        ldgcol,
                        givnum.offset((nlf + lvl2 * givnum_dim1) as isize) as *mut doublereal,
                        ldu,
                        poles.offset((nlf + lvl2 * poles_dim1) as isize) as *mut doublereal,
                        difl.offset((nlf + lvl * difl_dim1) as isize) as *mut doublereal,
                        difr.offset((nlf + lvl2 * difr_dim1) as isize) as *mut doublereal,
                        z__.offset((nlf + lvl * z_dim1) as isize) as *mut doublereal,
                        k.offset(j as isize) as *mut integer,
                        c__.offset(j as isize) as *mut doublereal,
                        s.offset(j as isize) as *mut doublereal,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        info,
                    );
                    i__ -= 1;
                }
                lvl += 1;
            }
            ndb1 = ((nd as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                / 2 as ::core::ffi::c_long) as integer;
            i__1 = nd;
            i__ = ndb1;
            while i__ <= i__1 {
                i1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                ic = *iwork.offset((inode + i1) as isize);
                nl = *iwork.offset((ndiml + i1) as isize);
                nr = *iwork.offset((ndimr + i1) as isize);
                nlp1 = (nl as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                if i__ == nd {
                    nrp1 = nr;
                } else {
                    nrp1 = (nr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                }
                nlf = ic - nl;
                nrf = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                f2c_dgemm_0(
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nlp1,
                    nrhs,
                    &raw mut nlp1,
                    &raw mut c_b7,
                    vt.offset((nlf + vt_dim1) as isize) as *mut doublereal,
                    ldu,
                    b.offset((nlf + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    &raw mut c_b8,
                    bx.offset((nlf + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
                f2c_dgemm_0(
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nrp1,
                    nrhs,
                    &raw mut nrp1,
                    &raw mut c_b7,
                    vt.offset((nrf + vt_dim1) as isize) as *mut doublereal,
                    ldu,
                    b.offset((nrf + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    &raw mut c_b8,
                    bx.offset((nrf + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
                i__ += 1;
            }
        } else {
            ndb1 = ((nd as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                / 2 as ::core::ffi::c_long) as integer;
            i__1 = nd;
            i__ = ndb1;
            while i__ <= i__1 {
                i1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                ic = *iwork.offset((inode + i1) as isize);
                nl = *iwork.offset((ndiml + i1) as isize);
                nr = *iwork.offset((ndimr + i1) as isize);
                nlf = ic - nl;
                nrf = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                f2c_dgemm_0(
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nl,
                    nrhs,
                    &raw mut nl,
                    &raw mut c_b7,
                    u.offset((nlf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                    b.offset((nlf + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    &raw mut c_b8,
                    bx.offset((nlf + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
                f2c_dgemm_0(
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nr,
                    nrhs,
                    &raw mut nr,
                    &raw mut c_b7,
                    u.offset((nrf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                    b.offset((nrf + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    &raw mut c_b8,
                    bx.offset((nrf + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
                i__ += 1;
            }
            i__1 = nd;
            i__ = 1 as integer;
            while i__ <= i__1 {
                ic = *iwork.offset(
                    (inode as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as isize,
                );
                f2c_dcopy_0(
                    nrhs,
                    b.offset((ic + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    bx.offset((ic + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
                i__ += 1;
            }
            j = pow_ii_0(&raw mut c__2, &raw mut nlvl);
            sqre = 0 as integer;
            lvl = nlvl;
            while lvl >= 1 as ::core::ffi::c_long {
                lvl2 = (((lvl as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_long) as integer;
                if lvl == 1 as ::core::ffi::c_long {
                    lf = 1 as integer;
                    ll = 1 as integer;
                } else {
                    i__1 = (lvl as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    lf = pow_ii_0(&raw mut c__2, &raw mut i__1);
                    ll = (((lf as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_long) as integer;
                }
                i__1 = ll;
                i__ = lf;
                while i__ <= i__1 {
                    im1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    ic = *iwork.offset((inode + im1) as isize);
                    nl = *iwork.offset((ndiml + im1) as isize);
                    nr = *iwork.offset((ndimr + im1) as isize);
                    nlf = ic - nl;
                    nrf = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    j -= 1;
                    dlals0__0(
                        icompq,
                        &raw mut nl,
                        &raw mut nr,
                        &raw mut sqre,
                        nrhs,
                        bx.offset((nlf + bx_dim1) as isize) as *mut doublereal,
                        ldbx,
                        b.offset((nlf + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        perm.offset((nlf + lvl * perm_dim1) as isize) as *mut integer,
                        givptr.offset(j as isize) as *mut integer,
                        givcol.offset((nlf + lvl2 * givcol_dim1) as isize) as *mut integer,
                        ldgcol,
                        givnum.offset((nlf + lvl2 * givnum_dim1) as isize) as *mut doublereal,
                        ldu,
                        poles.offset((nlf + lvl2 * poles_dim1) as isize) as *mut doublereal,
                        difl.offset((nlf + lvl * difl_dim1) as isize) as *mut doublereal,
                        difr.offset((nlf + lvl2 * difr_dim1) as isize) as *mut doublereal,
                        z__.offset((nlf + lvl * z_dim1) as isize) as *mut doublereal,
                        k.offset(j as isize) as *mut integer,
                        c__.offset(j as isize) as *mut doublereal,
                        s.offset(j as isize) as *mut doublereal,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        info,
                    );
                    i__ += 1;
                }
                lvl -= 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlalsa::dgelsd_closure_dlalsa_;

pub mod raw_dlals0 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c_b5: doublereal = -1.0f64;
    static mut c__1: integer = 1 as integer;
    static mut c_b11: doublereal = 1.0f64;
    static mut c_b13: doublereal = 0.0f64;
    static mut c__0: integer = 0 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlals0_(
        mut icompq: *mut integer,
        mut nl: *mut integer,
        mut nr: *mut integer,
        mut sqre: *mut integer,
        mut nrhs: *mut integer,
        mut b: *mut doublereal,
        mut ldb: *mut integer,
        mut bx: *mut doublereal,
        mut ldbx: *mut integer,
        mut perm: *mut integer,
        mut givptr: *mut integer,
        mut givcol: *mut integer,
        mut ldgcol: *mut integer,
        mut givnum: *mut doublereal,
        mut ldgnum: *mut integer,
        mut poles: *mut doublereal,
        mut difl: *mut doublereal,
        mut difr: *mut doublereal,
        mut z__: *mut doublereal,
        mut k: *mut integer,
        mut c__: *mut doublereal,
        mut s: *mut doublereal,
        mut work: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut givcol_dim1: integer = 0;
        let mut givcol_offset: integer = 0;
        let mut b_dim1: integer = 0;
        let mut b_offset: integer = 0;
        let mut bx_dim1: integer = 0;
        let mut bx_offset: integer = 0;
        let mut difr_dim1: integer = 0;
        let mut difr_offset: integer = 0;
        let mut givnum_dim1: integer = 0;
        let mut givnum_offset: integer = 0;
        let mut poles_dim1: integer = 0;
        let mut poles_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut m: integer = 0;
        let mut n: integer = 0;
        let mut dj: doublereal = 0.;
        let mut nlp1: integer = 0;
        let mut temp: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_drot"]
            fn f2c_drot_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dnrm2"]
            fn f2c_dnrm2_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dscal"]
            fn f2c_dscal_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut diflj: doublereal = 0.;
        let mut difrj: doublereal = 0.;
        let mut dsigj: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dgemv"]
            fn f2c_dgemv_0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamc3_"]
            fn dlamc3__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlascl_"]
            fn dlascl__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlacpy_"]
            fn dlacpy__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut dsigjp: doublereal = 0.;
        b_dim1 = *ldb;
        b_offset = 1 as integer + b_dim1;
        b = b.wrapping_offset(-(b_offset as isize));
        bx_dim1 = *ldbx;
        bx_offset = 1 as integer + bx_dim1;
        bx = bx.wrapping_offset(-(bx_offset as isize));
        perm = perm.wrapping_offset(-1);
        givcol_dim1 = *ldgcol;
        givcol_offset = 1 as integer + givcol_dim1;
        givcol = givcol.wrapping_offset(-(givcol_offset as isize));
        difr_dim1 = *ldgnum;
        difr_offset = 1 as integer + difr_dim1;
        difr = difr.wrapping_offset(-(difr_offset as isize));
        poles_dim1 = *ldgnum;
        poles_offset = 1 as integer + poles_dim1;
        poles = poles.wrapping_offset(-(poles_offset as isize));
        givnum_dim1 = *ldgnum;
        givnum_offset = 1 as integer + givnum_dim1;
        givnum = givnum.wrapping_offset(-(givnum_offset as isize));
        difl = difl.wrapping_offset(-1);
        z__ = z__.wrapping_offset(-1);
        work = work.wrapping_offset(-1);
        *info = 0 as integer;
        if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *nl < 1 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *nr < 1 as ::core::ffi::c_long {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *sqre < 0 as ::core::ffi::c_long || *sqre > 1 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        }
        n = (*nl + *nr + 1 as ::core::ffi::c_long) as integer;
        if *nrhs < 1 as ::core::ffi::c_long {
            *info = -(5 as ::core::ffi::c_int) as integer;
        } else if *ldb < n {
            *info = -(7 as ::core::ffi::c_int) as integer;
        } else if *ldbx < n {
            *info = -(9 as ::core::ffi::c_int) as integer;
        } else if *givptr < 0 as ::core::ffi::c_long {
            *info = -(11 as ::core::ffi::c_int) as integer;
        } else if *ldgcol < n {
            *info = -(13 as ::core::ffi::c_int) as integer;
        } else if *ldgnum < n {
            *info = -(15 as ::core::ffi::c_int) as integer;
        } else if *k < 1 as ::core::ffi::c_long {
            *info = -(20 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLALS0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        m = n + *sqre;
        nlp1 = (*nl + 1 as ::core::ffi::c_long) as integer;
        if *icompq == 0 as ::core::ffi::c_long {
            i__1 = *givptr;
            i__ = 1 as integer;
            while i__ <= i__1 {
                f2c_drot_0(
                    nrhs,
                    b.offset(
                        (*givcol.offset((i__ + (givcol_dim1 << 1 as ::core::ffi::c_int)) as isize)
                            + b_dim1) as isize,
                    ) as *mut doublereal,
                    ldb,
                    b.offset((*givcol.offset((i__ + givcol_dim1) as isize) + b_dim1) as isize)
                        as *mut doublereal,
                    ldb,
                    givnum.offset((i__ + (givnum_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        as *mut doublereal,
                    givnum.offset((i__ + givnum_dim1) as isize) as *mut doublereal,
                );
                i__ += 1;
            }
            f2c_dcopy_0(
                nrhs,
                b.offset((nlp1 + b_dim1) as isize) as *mut doublereal,
                ldb,
                bx.offset((bx_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldbx,
            );
            i__1 = n;
            i__ = 2 as integer;
            while i__ <= i__1 {
                f2c_dcopy_0(
                    nrhs,
                    b.offset((*perm.offset(i__ as isize) + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    bx.offset((i__ + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
                i__ += 1;
            }
            if *k == 1 as ::core::ffi::c_long {
                f2c_dcopy_0(
                    nrhs,
                    bx.offset(bx_offset as isize) as *mut doublereal,
                    ldbx,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                );
                if *z__.offset(1 as ::core::ffi::c_int as isize) < 0.0f64 {
                    f2c_dscal_0(
                        nrhs,
                        &raw mut c_b5,
                        b.offset(b_offset as isize) as *mut doublereal,
                        ldb,
                    );
                }
            } else {
                i__1 = *k;
                j = 1 as integer;
                while j <= i__1 {
                    diflj = *difl.offset(j as isize);
                    dj = *poles.offset((j + poles_dim1) as isize);
                    dsigj = -*poles.offset((j + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    if j < *k {
                        difrj = -*difr.offset((j + difr_dim1) as isize);
                        dsigjp = -*poles.offset(
                            (j + 1 as integer + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize,
                        );
                    }
                    if *z__.offset(j as isize) == 0.0f64
                        || *poles.offset((j + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize)
                            == 0.0f64
                    {
                        *work.offset(j as isize) = 0.0f64 as doublereal;
                    } else {
                        *work.offset(j as isize) = -*poles
                            .offset((j + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize)
                            * *z__.offset(j as isize)
                            / diflj
                            / (*poles
                                .offset((j + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize)
                                + dj);
                    }
                    i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        if *z__.offset(i__ as isize) == 0.0f64
                            || *poles
                                .offset((i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize)
                                == 0.0f64
                        {
                            *work.offset(i__ as isize) = 0.0f64 as doublereal;
                        } else {
                            *work.offset(i__ as isize) = *poles
                                .offset((i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize)
                                * *z__.offset(i__ as isize)
                                / (dlamc3__0(
                                    poles.offset(
                                        (i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                    ) as *mut doublereal,
                                    &raw mut dsigj,
                                ) - diflj)
                                / (*poles.offset(
                                    (i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                ) + dj);
                        }
                        i__ += 1;
                    }
                    i__2 = *k;
                    i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__2 {
                        if *z__.offset(i__ as isize) == 0.0f64
                            || *poles
                                .offset((i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize)
                                == 0.0f64
                        {
                            *work.offset(i__ as isize) = 0.0f64 as doublereal;
                        } else {
                            *work.offset(i__ as isize) = *poles
                                .offset((i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize)
                                * *z__.offset(i__ as isize)
                                / (dlamc3__0(
                                    poles.offset(
                                        (i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                    ) as *mut doublereal,
                                    &raw mut dsigjp,
                                ) + difrj)
                                / (*poles.offset(
                                    (i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                ) + dj);
                        }
                        i__ += 1;
                    }
                    *work.offset(1 as ::core::ffi::c_int as isize) = -1.0f64 as doublereal;
                    temp = f2c_dnrm2_0(
                        k,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        &raw mut c__1,
                    );
                    f2c_dgemv_0(
                        b"T\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        k,
                        nrhs,
                        &raw mut c_b11,
                        bx.offset(bx_offset as isize) as *mut doublereal,
                        ldbx,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut c_b13,
                        b.offset((j + b_dim1) as isize) as *mut doublereal,
                        ldb,
                    );
                    dlascl__0(
                        b"G\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        &raw mut temp,
                        &raw mut c_b11,
                        &raw mut c__1,
                        nrhs,
                        b.offset((j + b_dim1) as isize) as *mut doublereal,
                        ldb,
                        info,
                    );
                    j += 1;
                }
            }
            if *k
                < (if m >= n {
                    m as ::core::ffi::c_long
                } else {
                    n as ::core::ffi::c_long
                })
            {
                i__1 = n - *k;
                dlacpy__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    nrhs,
                    bx.offset((*k + 1 as integer + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                    b.offset((*k + 1 as integer + b_dim1) as isize) as *mut doublereal,
                    ldb,
                );
            }
        } else {
            if *k == 1 as ::core::ffi::c_long {
                f2c_dcopy_0(
                    nrhs,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                    bx.offset(bx_offset as isize) as *mut doublereal,
                    ldbx,
                );
            } else {
                i__1 = *k;
                j = 1 as integer;
                while j <= i__1 {
                    dsigj = *poles.offset((j + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    if *z__.offset(j as isize) == 0.0f64 {
                        *work.offset(j as isize) = 0.0f64 as doublereal;
                    } else {
                        *work.offset(j as isize) = -*z__.offset(j as isize)
                            / *difl.offset(j as isize)
                            / (dsigj + *poles.offset((j + poles_dim1) as isize))
                            / *difr.offset((j + (difr_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    }
                    i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        if *z__.offset(j as isize) == 0.0f64 {
                            *work.offset(i__ as isize) = 0.0f64 as doublereal;
                        } else {
                            d__1 = -*poles.offset(
                                (i__ + 1 as integer + (poles_dim1 << 1 as ::core::ffi::c_int))
                                    as isize,
                            );
                            *work.offset(i__ as isize) = *z__.offset(j as isize)
                                / (dlamc3__0(&raw mut dsigj, &raw mut d__1)
                                    - *difr.offset((i__ + difr_dim1) as isize))
                                / (dsigj + *poles.offset((i__ + poles_dim1) as isize))
                                / *difr.offset(
                                    (i__ + (difr_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                );
                        }
                        i__ += 1;
                    }
                    i__2 = *k;
                    i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__2 {
                        if *z__.offset(j as isize) == 0.0f64 {
                            *work.offset(i__ as isize) = 0.0f64 as doublereal;
                        } else {
                            d__1 = -*poles
                                .offset((i__ + (poles_dim1 << 1 as ::core::ffi::c_int)) as isize);
                            *work.offset(i__ as isize) = *z__.offset(j as isize)
                                / (dlamc3__0(&raw mut dsigj, &raw mut d__1)
                                    - *difl.offset(i__ as isize))
                                / (dsigj + *poles.offset((i__ + poles_dim1) as isize))
                                / *difr.offset(
                                    (i__ + (difr_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                );
                        }
                        i__ += 1;
                    }
                    f2c_dgemv_0(
                        b"T\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        k,
                        nrhs,
                        &raw mut c_b11,
                        b.offset(b_offset as isize) as *mut doublereal,
                        ldb,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut c_b13,
                        bx.offset((j + bx_dim1) as isize) as *mut doublereal,
                        ldbx,
                    );
                    j += 1;
                }
            }
            if *sqre == 1 as ::core::ffi::c_long {
                f2c_dcopy_0(
                    nrhs,
                    b.offset((m + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    bx.offset((m + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
                f2c_drot_0(
                    nrhs,
                    bx.offset((bx_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    ldbx,
                    bx.offset((m + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                    c__,
                    s,
                );
            }
            if *k
                < (if m >= n {
                    m as ::core::ffi::c_long
                } else {
                    n as ::core::ffi::c_long
                })
            {
                i__1 = n - *k;
                dlacpy__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    nrhs,
                    b.offset((*k + 1 as integer + b_dim1) as isize) as *mut doublereal,
                    ldb,
                    bx.offset((*k + 1 as integer + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                );
            }
            f2c_dcopy_0(
                nrhs,
                bx.offset((bx_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldbx,
                b.offset((nlp1 + b_dim1) as isize) as *mut doublereal,
                ldb,
            );
            if *sqre == 1 as ::core::ffi::c_long {
                f2c_dcopy_0(
                    nrhs,
                    bx.offset((m + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                    b.offset((m + b_dim1) as isize) as *mut doublereal,
                    ldb,
                );
            }
            i__1 = n;
            i__ = 2 as integer;
            while i__ <= i__1 {
                f2c_dcopy_0(
                    nrhs,
                    bx.offset((i__ + bx_dim1) as isize) as *mut doublereal,
                    ldbx,
                    b.offset((*perm.offset(i__ as isize) + b_dim1) as isize) as *mut doublereal,
                    ldb,
                );
                i__ += 1;
            }
            i__ = *givptr;
            while i__ >= 1 as ::core::ffi::c_long {
                d__1 = -*givnum.offset((i__ + givnum_dim1) as isize);
                f2c_drot_0(
                    nrhs,
                    b.offset(
                        (*givcol.offset((i__ + (givcol_dim1 << 1 as ::core::ffi::c_int)) as isize)
                            + b_dim1) as isize,
                    ) as *mut doublereal,
                    ldb,
                    b.offset((*givcol.offset((i__ + givcol_dim1) as isize) + b_dim1) as isize)
                        as *mut doublereal,
                    ldb,
                    givnum.offset((i__ + (givnum_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        as *mut doublereal,
                    &raw mut d__1,
                );
                i__ -= 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlals0::dgelsd_closure_dlals0_;

pub mod raw_dlasda {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c__0: integer = 0 as integer;
    static mut c_b11: doublereal = 0.0f64;
    static mut c_b12: doublereal = 1.0f64;
    static mut c__1: integer = 1 as integer;
    static mut c__2: integer = 2 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasda_(
        mut icompq: *mut integer,
        mut smlsiz: *mut integer,
        mut n: *mut integer,
        mut sqre: *mut integer,
        mut d__: *mut doublereal,
        mut e: *mut doublereal,
        mut u: *mut doublereal,
        mut ldu: *mut integer,
        mut vt: *mut doublereal,
        mut k: *mut integer,
        mut difl: *mut doublereal,
        mut difr: *mut doublereal,
        mut z__: *mut doublereal,
        mut poles: *mut doublereal,
        mut givptr: *mut integer,
        mut givcol: *mut integer,
        mut ldgcol: *mut integer,
        mut perm: *mut integer,
        mut givnum: *mut doublereal,
        mut c__: *mut doublereal,
        mut s: *mut doublereal,
        mut work: *mut doublereal,
        mut iwork: *mut integer,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut givcol_dim1: integer = 0;
        let mut givcol_offset: integer = 0;
        let mut perm_dim1: integer = 0;
        let mut perm_offset: integer = 0;
        let mut difl_dim1: integer = 0;
        let mut difl_offset: integer = 0;
        let mut difr_dim1: integer = 0;
        let mut difr_offset: integer = 0;
        let mut givnum_dim1: integer = 0;
        let mut givnum_offset: integer = 0;
        let mut poles_dim1: integer = 0;
        let mut poles_offset: integer = 0;
        let mut u_dim1: integer = 0;
        let mut u_offset: integer = 0;
        let mut vt_dim1: integer = 0;
        let mut vt_offset: integer = 0;
        let mut z_dim1: integer = 0;
        let mut z_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_pow_ii"]
            fn pow_ii_0(_: *mut integer, _: *mut integer) -> integer;
        }
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut m: integer = 0;
        let mut i1: integer = 0;
        let mut ic: integer = 0;
        let mut lf: integer = 0;
        let mut nd: integer = 0;
        let mut ll: integer = 0;
        let mut nl: integer = 0;
        let mut vf: integer = 0;
        let mut nr: integer = 0;
        let mut vl: integer = 0;
        let mut im1: integer = 0;
        let mut ncc: integer = 0;
        let mut nlf: integer = 0;
        let mut nrf: integer = 0;
        let mut vfi: integer = 0;
        let mut iwk: integer = 0;
        let mut vli: integer = 0;
        let mut lvl: integer = 0;
        let mut nru: integer = 0;
        let mut ndb1: integer = 0;
        let mut nlp1: integer = 0;
        let mut lvl2: integer = 0;
        let mut nrp1: integer = 0;
        let mut beta: doublereal = 0.;
        let mut idxq: integer = 0;
        let mut nlvl: integer = 0;
        let mut alpha: doublereal = 0.;
        let mut inode: integer = 0;
        let mut ndiml: integer = 0;
        let mut ndimr: integer = 0;
        let mut idxqi: integer = 0;
        let mut itemp: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut sqrei: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlasd6_"]
            fn dlasd6__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut nwork1: integer = 0;
        let mut nwork2: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlasdq_"]
            fn dlasdq__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasdt_"]
            fn dlasdt__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlaset_"]
            fn dlaset__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut smlszp: integer = 0;
        d__ = d__.wrapping_offset(-1);
        e = e.wrapping_offset(-1);
        givnum_dim1 = *ldu;
        givnum_offset = 1 as integer + givnum_dim1;
        givnum = givnum.wrapping_offset(-(givnum_offset as isize));
        poles_dim1 = *ldu;
        poles_offset = 1 as integer + poles_dim1;
        poles = poles.wrapping_offset(-(poles_offset as isize));
        z_dim1 = *ldu;
        z_offset = 1 as integer + z_dim1;
        z__ = z__.wrapping_offset(-(z_offset as isize));
        difr_dim1 = *ldu;
        difr_offset = 1 as integer + difr_dim1;
        difr = difr.wrapping_offset(-(difr_offset as isize));
        difl_dim1 = *ldu;
        difl_offset = 1 as integer + difl_dim1;
        difl = difl.wrapping_offset(-(difl_offset as isize));
        vt_dim1 = *ldu;
        vt_offset = 1 as integer + vt_dim1;
        vt = vt.wrapping_offset(-(vt_offset as isize));
        u_dim1 = *ldu;
        u_offset = 1 as integer + u_dim1;
        u = u.wrapping_offset(-(u_offset as isize));
        k = k.wrapping_offset(-1);
        givptr = givptr.wrapping_offset(-1);
        perm_dim1 = *ldgcol;
        perm_offset = 1 as integer + perm_dim1;
        perm = perm.wrapping_offset(-(perm_offset as isize));
        givcol_dim1 = *ldgcol;
        givcol_offset = 1 as integer + givcol_dim1;
        givcol = givcol.wrapping_offset(-(givcol_offset as isize));
        c__ = c__.wrapping_offset(-1);
        s = s.wrapping_offset(-1);
        work = work.wrapping_offset(-1);
        iwork = iwork.wrapping_offset(-1);
        *info = 0 as integer;
        if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *smlsiz < 3 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *n < 0 as ::core::ffi::c_long {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *sqre < 0 as ::core::ffi::c_long || *sqre > 1 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if *ldu < *n + *sqre {
            *info = -(8 as ::core::ffi::c_int) as integer;
        } else if *ldgcol < *n {
            *info = -(17 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLASDA\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        m = *n + *sqre;
        if *n <= *smlsiz {
            if *icompq == 0 as ::core::ffi::c_long {
                dlasdq__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    sqre,
                    n,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut c__0,
                    d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldu,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
            } else {
                dlasdq__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    sqre,
                    n,
                    &raw mut m,
                    n,
                    &raw mut c__0,
                    d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldu,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
            }
            return 0 as ::core::ffi::c_int;
        }
        inode = 1 as integer;
        ndiml = inode + *n;
        ndimr = ndiml + *n;
        idxq = ndimr + *n;
        iwk = idxq + *n;
        ncc = 0 as integer;
        nru = 0 as integer;
        smlszp = (*smlsiz + 1 as ::core::ffi::c_long) as integer;
        vf = 1 as integer;
        vl = vf + m;
        nwork1 = vl + m;
        nwork2 = nwork1 + smlszp * smlszp;
        dlasdt__0(
            n,
            &raw mut nlvl,
            &raw mut nd,
            iwork.offset(inode as isize) as *mut integer,
            iwork.offset(ndiml as isize) as *mut integer,
            iwork.offset(ndimr as isize) as *mut integer,
            smlsiz,
        );
        ndb1 = ((nd as ::core::ffi::c_long + 1 as ::core::ffi::c_long) / 2 as ::core::ffi::c_long)
            as integer;
        i__1 = nd;
        i__ = ndb1;
        while i__ <= i__1 {
            i1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            ic = *iwork.offset((inode + i1) as isize);
            nl = *iwork.offset((ndiml + i1) as isize);
            nlp1 = (nl as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            nr = *iwork.offset((ndimr + i1) as isize);
            nlf = ic - nl;
            nrf = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            idxqi = (idxq as ::core::ffi::c_long + nlf as ::core::ffi::c_long
                - 2 as ::core::ffi::c_long) as integer;
            vfi = (vf as ::core::ffi::c_long + nlf as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            vli = (vl as ::core::ffi::c_long + nlf as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            sqrei = 1 as integer;
            if *icompq == 0 as ::core::ffi::c_long {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nlp1,
                    &raw mut nlp1,
                    &raw mut c_b11,
                    &raw mut c_b12,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    &raw mut smlszp,
                );
                dlasdq__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut sqrei,
                    &raw mut nl,
                    &raw mut nlp1,
                    &raw mut nru,
                    &raw mut ncc,
                    d__.offset(nlf as isize) as *mut doublereal,
                    e.offset(nlf as isize) as *mut doublereal,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    &raw mut smlszp,
                    work.offset(nwork2 as isize) as *mut doublereal,
                    &raw mut nl,
                    work.offset(nwork2 as isize) as *mut doublereal,
                    &raw mut nl,
                    work.offset(nwork2 as isize) as *mut doublereal,
                    info,
                );
                itemp = nwork1 + nl * smlszp;
                f2c_dcopy_0(
                    &raw mut nlp1,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vfi as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                f2c_dcopy_0(
                    &raw mut nlp1,
                    work.offset(itemp as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vli as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            } else {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nl,
                    &raw mut nl,
                    &raw mut c_b11,
                    &raw mut c_b12,
                    u.offset((nlf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                );
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nlp1,
                    &raw mut nlp1,
                    &raw mut c_b11,
                    &raw mut c_b12,
                    vt.offset((nlf + vt_dim1) as isize) as *mut doublereal,
                    ldu,
                );
                dlasdq__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut sqrei,
                    &raw mut nl,
                    &raw mut nlp1,
                    &raw mut nl,
                    &raw mut ncc,
                    d__.offset(nlf as isize) as *mut doublereal,
                    e.offset(nlf as isize) as *mut doublereal,
                    vt.offset((nlf + vt_dim1) as isize) as *mut doublereal,
                    ldu,
                    u.offset((nlf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                    u.offset((nlf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    info,
                );
                f2c_dcopy_0(
                    &raw mut nlp1,
                    vt.offset((nlf + vt_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vfi as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                f2c_dcopy_0(
                    &raw mut nlp1,
                    vt.offset((nlf + nlp1 * vt_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vli as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            if *info != 0 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            i__2 = nl;
            j = 1 as integer;
            while j <= i__2 {
                *iwork.offset((idxqi + j) as isize) = j;
                j += 1;
            }
            if i__ == nd && *sqre == 0 as ::core::ffi::c_long {
                sqrei = 0 as integer;
            } else {
                sqrei = 1 as integer;
            }
            idxqi += nlp1 as ::core::ffi::c_long;
            vfi += nlp1 as ::core::ffi::c_long;
            vli += nlp1 as ::core::ffi::c_long;
            nrp1 = nr + sqrei;
            if *icompq == 0 as ::core::ffi::c_long {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nrp1,
                    &raw mut nrp1,
                    &raw mut c_b11,
                    &raw mut c_b12,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    &raw mut smlszp,
                );
                dlasdq__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut sqrei,
                    &raw mut nr,
                    &raw mut nrp1,
                    &raw mut nru,
                    &raw mut ncc,
                    d__.offset(nrf as isize) as *mut doublereal,
                    e.offset(nrf as isize) as *mut doublereal,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    &raw mut smlszp,
                    work.offset(nwork2 as isize) as *mut doublereal,
                    &raw mut nr,
                    work.offset(nwork2 as isize) as *mut doublereal,
                    &raw mut nr,
                    work.offset(nwork2 as isize) as *mut doublereal,
                    info,
                );
                itemp = nwork1 + (nrp1 - 1 as integer) * smlszp;
                f2c_dcopy_0(
                    &raw mut nrp1,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vfi as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                f2c_dcopy_0(
                    &raw mut nrp1,
                    work.offset(itemp as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vli as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            } else {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nr,
                    &raw mut nr,
                    &raw mut c_b11,
                    &raw mut c_b12,
                    u.offset((nrf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                );
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut nrp1,
                    &raw mut nrp1,
                    &raw mut c_b11,
                    &raw mut c_b12,
                    vt.offset((nrf + vt_dim1) as isize) as *mut doublereal,
                    ldu,
                );
                dlasdq__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut sqrei,
                    &raw mut nr,
                    &raw mut nrp1,
                    &raw mut nr,
                    &raw mut ncc,
                    d__.offset(nrf as isize) as *mut doublereal,
                    e.offset(nrf as isize) as *mut doublereal,
                    vt.offset((nrf + vt_dim1) as isize) as *mut doublereal,
                    ldu,
                    u.offset((nrf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                    u.offset((nrf + u_dim1) as isize) as *mut doublereal,
                    ldu,
                    work.offset(nwork1 as isize) as *mut doublereal,
                    info,
                );
                f2c_dcopy_0(
                    &raw mut nrp1,
                    vt.offset((nrf + vt_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vfi as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                f2c_dcopy_0(
                    &raw mut nrp1,
                    vt.offset((nrf + nrp1 * vt_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    work.offset(vli as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            if *info != 0 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            i__2 = nr;
            j = 1 as integer;
            while j <= i__2 {
                *iwork.offset((idxqi + j) as isize) = j;
                j += 1;
            }
            i__ += 1;
        }
        j = pow_ii_0(&raw mut c__2, &raw mut nlvl);
        lvl = nlvl;
        while lvl >= 1 as ::core::ffi::c_long {
            lvl2 = (((lvl as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as integer;
            if lvl == 1 as ::core::ffi::c_long {
                lf = 1 as integer;
                ll = 1 as integer;
            } else {
                i__1 = (lvl as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                lf = pow_ii_0(&raw mut c__2, &raw mut i__1);
                ll = (((lf as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_long) as integer;
            }
            i__1 = ll;
            i__ = lf;
            while i__ <= i__1 {
                im1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                ic = *iwork.offset((inode + im1) as isize);
                nl = *iwork.offset((ndiml + im1) as isize);
                nr = *iwork.offset((ndimr + im1) as isize);
                nlf = ic - nl;
                nrf = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                if i__ == ll {
                    sqrei = *sqre;
                } else {
                    sqrei = 1 as integer;
                }
                vfi = (vf as ::core::ffi::c_long + nlf as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as integer;
                vli = (vl as ::core::ffi::c_long + nlf as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as integer;
                idxqi = (idxq as ::core::ffi::c_long + nlf as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as integer;
                alpha = *d__.offset(ic as isize);
                beta = *e.offset(ic as isize);
                if *icompq == 0 as ::core::ffi::c_long {
                    dlasd6__0(
                        icompq,
                        &raw mut nl,
                        &raw mut nr,
                        &raw mut sqrei,
                        d__.offset(nlf as isize) as *mut doublereal,
                        work.offset(vfi as isize) as *mut doublereal,
                        work.offset(vli as isize) as *mut doublereal,
                        &raw mut alpha,
                        &raw mut beta,
                        iwork.offset(idxqi as isize) as *mut integer,
                        perm.offset(perm_offset as isize) as *mut integer,
                        givptr.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                        givcol.offset(givcol_offset as isize) as *mut integer,
                        ldgcol,
                        givnum.offset(givnum_offset as isize) as *mut doublereal,
                        ldu,
                        poles.offset(poles_offset as isize) as *mut doublereal,
                        difl.offset(difl_offset as isize) as *mut doublereal,
                        difr.offset(difr_offset as isize) as *mut doublereal,
                        z__.offset(z_offset as isize) as *mut doublereal,
                        k.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                        c__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        work.offset(nwork1 as isize) as *mut doublereal,
                        iwork.offset(iwk as isize) as *mut integer,
                        info,
                    );
                } else {
                    j -= 1;
                    dlasd6__0(
                        icompq,
                        &raw mut nl,
                        &raw mut nr,
                        &raw mut sqrei,
                        d__.offset(nlf as isize) as *mut doublereal,
                        work.offset(vfi as isize) as *mut doublereal,
                        work.offset(vli as isize) as *mut doublereal,
                        &raw mut alpha,
                        &raw mut beta,
                        iwork.offset(idxqi as isize) as *mut integer,
                        perm.offset((nlf + lvl * perm_dim1) as isize) as *mut integer,
                        givptr.offset(j as isize) as *mut integer,
                        givcol.offset((nlf + lvl2 * givcol_dim1) as isize) as *mut integer,
                        ldgcol,
                        givnum.offset((nlf + lvl2 * givnum_dim1) as isize) as *mut doublereal,
                        ldu,
                        poles.offset((nlf + lvl2 * poles_dim1) as isize) as *mut doublereal,
                        difl.offset((nlf + lvl * difl_dim1) as isize) as *mut doublereal,
                        difr.offset((nlf + lvl2 * difr_dim1) as isize) as *mut doublereal,
                        z__.offset((nlf + lvl * z_dim1) as isize) as *mut doublereal,
                        k.offset(j as isize) as *mut integer,
                        c__.offset(j as isize) as *mut doublereal,
                        s.offset(j as isize) as *mut doublereal,
                        work.offset(nwork1 as isize) as *mut doublereal,
                        iwork.offset(iwk as isize) as *mut integer,
                        info,
                    );
                }
                if *info != 0 as ::core::ffi::c_long {
                    return 0 as ::core::ffi::c_int;
                }
                i__ += 1;
            }
            lvl -= 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasda::dgelsd_closure_dlasda_;

pub mod raw_dlasd4 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    pub const TRUE_: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const FALSE_: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasd4_(
        mut n: *mut integer,
        mut i__: *mut integer,
        mut d__: *mut doublereal,
        mut z__: *mut doublereal,
        mut delta: *mut doublereal,
        mut rho: *mut doublereal,
        mut sigma: *mut doublereal,
        mut work: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut current_block: u64;
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut a: doublereal = 0.;
        let mut b: doublereal = 0.;
        let mut c__: doublereal = 0.;
        let mut j: integer = 0;
        let mut w: doublereal = 0.;
        let mut dd: [doublereal; 3] = [0.; 3];
        let mut ii: integer = 0;
        let mut dw: doublereal = 0.;
        let mut zz: [doublereal; 3] = [0.; 3];
        let mut ip1: integer = 0;
        let mut eta: doublereal = 0.;
        let mut phi: doublereal = 0.;
        let mut eps: doublereal = 0.;
        let mut tau: doublereal = 0.;
        let mut psi: doublereal = 0.;
        let mut iim1: integer = 0;
        let mut iip1: integer = 0;
        let mut dphi: doublereal = 0.;
        let mut dpsi: doublereal = 0.;
        let mut iter: integer = 0;
        let mut temp: doublereal = 0.;
        let mut prew: doublereal = 0.;
        let mut sg2lb: doublereal = 0.;
        let mut sg2ub: doublereal = 0.;
        let mut temp1: doublereal = 0.;
        let mut temp2: doublereal = 0.;
        let mut dtiim: doublereal = 0.;
        let mut delsq: doublereal = 0.;
        let mut dtiip: doublereal = 0.;
        let mut niter: integer = 0;
        let mut dtisq: doublereal = 0.;
        let mut swtch: logical = 0;
        let mut dtnsq: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlaed6_"]
            fn dlaed6__0(
                _: *mut integer,
                _: *mut logical,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasd5_"]
            fn dlasd5__0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        let mut delsq2: doublereal = 0.;
        let mut dtnsq1: doublereal = 0.;
        let mut swtch3: logical = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        let mut orgati: logical = 0;
        let mut erretm: doublereal = 0.;
        let mut dtipsq: doublereal = 0.;
        let mut rhoinv: doublereal = 0.;
        work = work.wrapping_offset(-1);
        delta = delta.wrapping_offset(-1);
        z__ = z__.wrapping_offset(-1);
        d__ = d__.wrapping_offset(-1);
        *info = 0 as integer;
        if *n == 1 as ::core::ffi::c_long {
            *sigma = super::lapack_sqrt(
                *d__.offset(1 as ::core::ffi::c_int as isize)
                    * *d__.offset(1 as ::core::ffi::c_int as isize)
                    + *rho
                        * *z__.offset(1 as ::core::ffi::c_int as isize)
                        * *z__.offset(1 as ::core::ffi::c_int as isize),
            ) as doublereal;
            *delta.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
            *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
            return 0 as ::core::ffi::c_int;
        }
        if *n == 2 as ::core::ffi::c_long {
            dlasd5__0(
                i__,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                delta.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                rho,
                sigma,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            );
            return 0 as ::core::ffi::c_int;
        }
        eps = dlamch__0(
            b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        rhoinv = 1.0f64 / *rho;
        if *i__ == *n {
            ii = (*n - 1 as ::core::ffi::c_long) as integer;
            niter = 1 as integer;
            temp = (*rho / 2.0f64) as doublereal;
            temp1 = (temp as ::core::ffi::c_double
                / (*d__.offset(*n as isize) as ::core::ffi::c_double
                    + super::lapack_sqrt(
                        *d__.offset(*n as isize) * *d__.offset(*n as isize) + temp,
                    ))) as doublereal;
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                *work.offset(j as isize) =
                    *d__.offset(j as isize) + *d__.offset(*n as isize) + temp1;
                *delta.offset(j as isize) =
                    *d__.offset(j as isize) - *d__.offset(*n as isize) - temp1;
                j += 1;
            }
            psi = 0.0f64 as doublereal;
            i__1 = (*n - 2 as ::core::ffi::c_long) as integer;
            j = 1 as integer;
            while j <= i__1 {
                psi += (*z__.offset(j as isize) * *z__.offset(j as isize)
                    / (*delta.offset(j as isize) * *work.offset(j as isize)))
                    as ::core::ffi::c_double;
                j += 1;
            }
            c__ = rhoinv + psi;
            w = c__
                + *z__.offset(ii as isize) * *z__.offset(ii as isize)
                    / (*delta.offset(ii as isize) * *work.offset(ii as isize))
                + *z__.offset(*n as isize) * *z__.offset(*n as isize)
                    / (*delta.offset(*n as isize) * *work.offset(*n as isize));
            if w <= 0.0f64 {
                temp1 =
                    super::lapack_sqrt(*d__.offset(*n as isize) * *d__.offset(*n as isize) + *rho)
                        as doublereal;
                temp = *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    * *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    / ((*d__.offset((*n - 1 as ::core::ffi::c_long) as isize) + temp1)
                        * (*d__.offset(*n as isize)
                            - *d__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                            + *rho / (*d__.offset(*n as isize) + temp1)))
                    + *z__.offset(*n as isize) * *z__.offset(*n as isize) / *rho;
                if c__ <= temp {
                    tau = *rho;
                } else {
                    delsq = (*d__.offset(*n as isize)
                        - *d__.offset((*n - 1 as ::core::ffi::c_long) as isize))
                        * (*d__.offset(*n as isize)
                            + *d__.offset((*n - 1 as ::core::ffi::c_long) as isize));
                    a = -c__ * delsq
                        + *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                            * *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                        + *z__.offset(*n as isize) * *z__.offset(*n as isize);
                    b = *z__.offset(*n as isize) * *z__.offset(*n as isize) * delsq;
                    if a < 0.0f64 {
                        tau = b * 2.0f64
                            / (super::lapack_sqrt(a * a + b * 4.0f64 * c__) as doublereal - a);
                    } else {
                        tau = ((a as ::core::ffi::c_double
                            + super::lapack_sqrt(a * a + b * 4.0f64 * c__))
                            / (c__ as ::core::ffi::c_double * 2.0f64))
                            as doublereal;
                    }
                }
            } else {
                delsq = (*d__.offset(*n as isize)
                    - *d__.offset((*n - 1 as ::core::ffi::c_long) as isize))
                    * (*d__.offset(*n as isize)
                        + *d__.offset((*n - 1 as ::core::ffi::c_long) as isize));
                a = -c__ * delsq
                    + *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                        * *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    + *z__.offset(*n as isize) * *z__.offset(*n as isize);
                b = *z__.offset(*n as isize) * *z__.offset(*n as isize) * delsq;
                if a < 0.0f64 {
                    tau = b * 2.0f64
                        / (super::lapack_sqrt(a * a + b * 4.0f64 * c__) as doublereal - a);
                } else {
                    tau = ((a as ::core::ffi::c_double
                        + super::lapack_sqrt(a * a + b * 4.0f64 * c__))
                        / (c__ as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                }
            }
            eta = (tau as ::core::ffi::c_double
                / (*d__.offset(*n as isize) as ::core::ffi::c_double
                    + super::lapack_sqrt(
                        *d__.offset(*n as isize) * *d__.offset(*n as isize) + tau,
                    ))) as doublereal;
            *sigma = *d__.offset(*n as isize) + eta;
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                *delta.offset(j as isize) =
                    *d__.offset(j as isize) - *d__.offset(*i__ as isize) - eta;
                *work.offset(j as isize) =
                    *d__.offset(j as isize) + *d__.offset(*i__ as isize) + eta;
                j += 1;
            }
            dpsi = 0.0f64 as doublereal;
            psi = 0.0f64 as doublereal;
            erretm = 0.0f64 as doublereal;
            i__1 = ii;
            j = 1 as integer;
            while j <= i__1 {
                temp = *z__.offset(j as isize)
                    / (*delta.offset(j as isize) * *work.offset(j as isize));
                psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                dpsi += (temp * temp) as ::core::ffi::c_double;
                erretm += psi as ::core::ffi::c_double;
                j += 1;
            }
            erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                erretm as ::core::ffi::c_double
            } else {
                -(erretm as ::core::ffi::c_double)
            }) as doublereal;
            temp =
                *z__.offset(*n as isize) / (*delta.offset(*n as isize) * *work.offset(*n as isize));
            phi = *z__.offset(*n as isize) * temp;
            dphi = temp * temp;
            erretm = (-phi - psi) * 8.0f64 + erretm - phi
                + rhoinv
                + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    tau
                } else {
                    -tau
                }) * (dpsi + dphi);
            w = rhoinv + phi + psi;
            if !((if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                w as ::core::ffi::c_double
            } else {
                -(w as ::core::ffi::c_double)
            }) <= eps * erretm)
            {
                niter += 1;
                dtnsq1 = *work.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    * *delta.offset((*n - 1 as ::core::ffi::c_long) as isize);
                dtnsq = *work.offset(*n as isize) * *delta.offset(*n as isize);
                c__ = w - dtnsq1 * dpsi - dtnsq * dphi;
                a = (dtnsq + dtnsq1) * w - dtnsq * dtnsq1 * (dpsi + dphi);
                b = dtnsq * dtnsq1 * w;
                if c__ < 0.0f64 {
                    c__ = (if c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        c__ as ::core::ffi::c_double
                    } else {
                        -(c__ as ::core::ffi::c_double)
                    }) as doublereal;
                }
                if c__ == 0.0f64 {
                    eta = *rho - *sigma * *sigma;
                } else if a >= 0.0f64 {
                    d__1 = a * a - b * 4.0f64 * c__;
                    eta = ((a as ::core::ffi::c_double
                        + super::lapack_sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))
                        / (c__ as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                } else {
                    d__1 = a * a - b * 4.0f64 * c__;
                    eta = (b as ::core::ffi::c_double * 2.0f64
                        / (a as ::core::ffi::c_double
                            - super::lapack_sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))) as doublereal;
                }
                if w * eta > 0.0f64 {
                    eta = -w / (dpsi + dphi);
                }
                temp = eta - dtnsq;
                if temp > *rho {
                    eta = *rho + dtnsq;
                }
                tau += eta as ::core::ffi::c_double;
                eta /= *sigma + super::lapack_sqrt(eta + *sigma * *sigma);
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    let ref mut fresh0 = *delta.offset(j as isize);
                    *fresh0 -= eta as ::core::ffi::c_double;
                    let ref mut fresh1 = *work.offset(j as isize);
                    *fresh1 += eta as ::core::ffi::c_double;
                    j += 1;
                }
                *sigma += eta as ::core::ffi::c_double;
                dpsi = 0.0f64 as doublereal;
                psi = 0.0f64 as doublereal;
                erretm = 0.0f64 as doublereal;
                i__1 = ii;
                j = 1 as integer;
                while j <= i__1 {
                    temp = *z__.offset(j as isize)
                        / (*work.offset(j as isize) * *delta.offset(j as isize));
                    psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                    dpsi += (temp * temp) as ::core::ffi::c_double;
                    erretm += psi as ::core::ffi::c_double;
                    j += 1;
                }
                erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    erretm as ::core::ffi::c_double
                } else {
                    -(erretm as ::core::ffi::c_double)
                }) as doublereal;
                temp = *z__.offset(*n as isize)
                    / (*work.offset(*n as isize) * *delta.offset(*n as isize));
                phi = *z__.offset(*n as isize) * temp;
                dphi = temp * temp;
                erretm = (-phi - psi) * 8.0f64 + erretm - phi
                    + rhoinv
                    + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        tau
                    } else {
                        -tau
                    }) * (dpsi + dphi);
                w = rhoinv + phi + psi;
                iter = (niter as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                niter = iter;
                loop {
                    if !(niter <= 20 as ::core::ffi::c_long) {
                        current_block = 14883390358315838856;
                        break;
                    }
                    if (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        w as ::core::ffi::c_double
                    } else {
                        -(w as ::core::ffi::c_double)
                    }) <= eps * erretm
                    {
                        current_block = 10032952401960431757;
                        break;
                    }
                    dtnsq1 = *work.offset((*n - 1 as ::core::ffi::c_long) as isize)
                        * *delta.offset((*n - 1 as ::core::ffi::c_long) as isize);
                    dtnsq = *work.offset(*n as isize) * *delta.offset(*n as isize);
                    c__ = w - dtnsq1 * dpsi - dtnsq * dphi;
                    a = (dtnsq + dtnsq1) * w - dtnsq1 * dtnsq * (dpsi + dphi);
                    b = dtnsq1 * dtnsq * w;
                    if a >= 0.0f64 {
                        d__1 = a * a - b * 4.0f64 * c__;
                        eta = ((a as ::core::ffi::c_double
                            + super::lapack_sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))
                            / (c__ as ::core::ffi::c_double * 2.0f64))
                            as doublereal;
                    } else {
                        d__1 = a * a - b * 4.0f64 * c__;
                        eta = (b as ::core::ffi::c_double * 2.0f64
                            / (a as ::core::ffi::c_double
                                - super::lapack_sqrt(
                                    (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1
                                    } else {
                                        -d__1
                                    }),
                                ))) as doublereal;
                    }
                    if w * eta > 0.0f64 {
                        eta = -w / (dpsi + dphi);
                    }
                    temp = eta - dtnsq;
                    if temp <= 0.0f64 {
                        eta /= 2.0f64;
                    }
                    tau += eta as ::core::ffi::c_double;
                    eta /= *sigma + super::lapack_sqrt(eta + *sigma * *sigma);
                    i__1 = *n;
                    j = 1 as integer;
                    while j <= i__1 {
                        let ref mut fresh2 = *delta.offset(j as isize);
                        *fresh2 -= eta as ::core::ffi::c_double;
                        let ref mut fresh3 = *work.offset(j as isize);
                        *fresh3 += eta as ::core::ffi::c_double;
                        j += 1;
                    }
                    *sigma += eta as ::core::ffi::c_double;
                    dpsi = 0.0f64 as doublereal;
                    psi = 0.0f64 as doublereal;
                    erretm = 0.0f64 as doublereal;
                    i__1 = ii;
                    j = 1 as integer;
                    while j <= i__1 {
                        temp = *z__.offset(j as isize)
                            / (*work.offset(j as isize) * *delta.offset(j as isize));
                        psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                        dpsi += (temp * temp) as ::core::ffi::c_double;
                        erretm += psi as ::core::ffi::c_double;
                        j += 1;
                    }
                    erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        erretm as ::core::ffi::c_double
                    } else {
                        -(erretm as ::core::ffi::c_double)
                    }) as doublereal;
                    temp = *z__.offset(*n as isize)
                        / (*work.offset(*n as isize) * *delta.offset(*n as isize));
                    phi = *z__.offset(*n as isize) * temp;
                    dphi = temp * temp;
                    erretm = (-phi - psi) * 8.0f64 + erretm - phi
                        + rhoinv
                        + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            tau
                        } else {
                            -tau
                        }) * (dpsi + dphi);
                    w = rhoinv + phi + psi;
                    niter += 1;
                }
                match current_block {
                    10032952401960431757 => {}
                    _ => {
                        *info = 1 as integer;
                    }
                }
            }
        } else {
            niter = 1 as integer;
            ip1 = (*i__ + 1 as ::core::ffi::c_long) as integer;
            delsq = (*d__.offset(ip1 as isize) - *d__.offset(*i__ as isize))
                * (*d__.offset(ip1 as isize) + *d__.offset(*i__ as isize));
            delsq2 = (delsq as ::core::ffi::c_double / 2.0f64) as doublereal;
            temp = (delsq2 as ::core::ffi::c_double
                / (*d__.offset(*i__ as isize) as ::core::ffi::c_double
                    + super::lapack_sqrt(
                        *d__.offset(*i__ as isize) * *d__.offset(*i__ as isize) + delsq2,
                    ))) as doublereal;
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                *work.offset(j as isize) =
                    *d__.offset(j as isize) + *d__.offset(*i__ as isize) + temp;
                *delta.offset(j as isize) =
                    *d__.offset(j as isize) - *d__.offset(*i__ as isize) - temp;
                j += 1;
            }
            psi = 0.0f64 as doublereal;
            i__1 = (*i__ - 1 as ::core::ffi::c_long) as integer;
            j = 1 as integer;
            while j <= i__1 {
                psi += (*z__.offset(j as isize) * *z__.offset(j as isize)
                    / (*work.offset(j as isize) * *delta.offset(j as isize)))
                    as ::core::ffi::c_double;
                j += 1;
            }
            phi = 0.0f64 as doublereal;
            i__1 = (*i__ + 2 as ::core::ffi::c_long) as integer;
            j = *n;
            while j >= i__1 {
                phi += (*z__.offset(j as isize) * *z__.offset(j as isize)
                    / (*work.offset(j as isize) * *delta.offset(j as isize)))
                    as ::core::ffi::c_double;
                j -= 1;
            }
            c__ = rhoinv + psi + phi;
            w = c__
                + *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                    / (*work.offset(*i__ as isize) * *delta.offset(*i__ as isize))
                + *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize)
                    / (*work.offset(ip1 as isize) * *delta.offset(ip1 as isize));
            if w > 0.0f64 {
                orgati = TRUE_ as logical;
                sg2lb = 0.0f64 as doublereal;
                sg2ub = delsq2;
                a = c__ * delsq
                    + *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                    + *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize);
                b = *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize) * delsq;
                if a > 0.0f64 {
                    d__1 = a * a - b * 4.0f64 * c__;
                    tau = (b as ::core::ffi::c_double * 2.0f64
                        / (a as ::core::ffi::c_double
                            + super::lapack_sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))) as doublereal;
                } else {
                    d__1 = a * a - b * 4.0f64 * c__;
                    tau = ((a as ::core::ffi::c_double
                        - super::lapack_sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))
                        / (c__ as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                }
                eta = (tau as ::core::ffi::c_double
                    / (*d__.offset(*i__ as isize) as ::core::ffi::c_double
                        + super::lapack_sqrt(
                            *d__.offset(*i__ as isize) * *d__.offset(*i__ as isize) + tau,
                        ))) as doublereal;
            } else {
                orgati = FALSE_ as logical;
                sg2lb = -delsq2;
                sg2ub = 0.0f64 as doublereal;
                a = c__ * delsq
                    - *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                    - *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize);
                b = *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize) * delsq;
                if a < 0.0f64 {
                    d__1 = a * a + b * 4.0f64 * c__;
                    tau = (b as ::core::ffi::c_double * 2.0f64
                        / (a as ::core::ffi::c_double
                            - super::lapack_sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))) as doublereal;
                } else {
                    d__1 = a * a + b * 4.0f64 * c__;
                    tau = (-(a as ::core::ffi::c_double
                        + super::lapack_sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))
                        / (c__ as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                }
                d__1 = *d__.offset(ip1 as isize) * *d__.offset(ip1 as isize) + tau;
                eta = (tau as ::core::ffi::c_double
                    / (*d__.offset(ip1 as isize) as ::core::ffi::c_double
                        + super::lapack_sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))) as doublereal;
            }
            if orgati != 0 {
                ii = *i__;
                *sigma = *d__.offset(*i__ as isize) + eta;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    *work.offset(j as isize) =
                        *d__.offset(j as isize) + *d__.offset(*i__ as isize) + eta;
                    *delta.offset(j as isize) =
                        *d__.offset(j as isize) - *d__.offset(*i__ as isize) - eta;
                    j += 1;
                }
            } else {
                ii = (*i__ + 1 as ::core::ffi::c_long) as integer;
                *sigma = *d__.offset(ip1 as isize) + eta;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    *work.offset(j as isize) =
                        *d__.offset(j as isize) + *d__.offset(ip1 as isize) + eta;
                    *delta.offset(j as isize) =
                        *d__.offset(j as isize) - *d__.offset(ip1 as isize) - eta;
                    j += 1;
                }
            }
            iim1 = (ii as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            iip1 = (ii as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dpsi = 0.0f64 as doublereal;
            psi = 0.0f64 as doublereal;
            erretm = 0.0f64 as doublereal;
            i__1 = iim1;
            j = 1 as integer;
            while j <= i__1 {
                temp = *z__.offset(j as isize)
                    / (*work.offset(j as isize) * *delta.offset(j as isize));
                psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                dpsi += (temp * temp) as ::core::ffi::c_double;
                erretm += psi as ::core::ffi::c_double;
                j += 1;
            }
            erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                erretm as ::core::ffi::c_double
            } else {
                -(erretm as ::core::ffi::c_double)
            }) as doublereal;
            dphi = 0.0f64 as doublereal;
            phi = 0.0f64 as doublereal;
            i__1 = iip1;
            j = *n;
            while j >= i__1 {
                temp = *z__.offset(j as isize)
                    / (*work.offset(j as isize) * *delta.offset(j as isize));
                phi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                dphi += (temp * temp) as ::core::ffi::c_double;
                erretm += phi as ::core::ffi::c_double;
                j -= 1;
            }
            w = rhoinv + phi + psi;
            swtch3 = FALSE_ as logical;
            if orgati != 0 {
                if w < 0.0f64 {
                    swtch3 = TRUE_ as logical;
                }
            } else if w > 0.0f64 {
                swtch3 = TRUE_ as logical;
            }
            if ii == 1 as ::core::ffi::c_long || ii == *n {
                swtch3 = FALSE_ as logical;
            }
            temp =
                *z__.offset(ii as isize) / (*work.offset(ii as isize) * *delta.offset(ii as isize));
            dw = dpsi + dphi + temp * temp;
            temp = *z__.offset(ii as isize) * temp;
            w += temp as ::core::ffi::c_double;
            erretm = (phi - psi) * 8.0f64
                + erretm
                + rhoinv * 2.0f64
                + (if temp >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    temp
                } else {
                    -temp
                }) * 3.0f64
                + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    tau
                } else {
                    -tau
                }) * dw;
            if !((if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                w as ::core::ffi::c_double
            } else {
                -(w as ::core::ffi::c_double)
            }) <= eps * erretm)
            {
                if w <= 0.0f64 {
                    sg2lb = (if sg2lb >= tau {
                        sg2lb as ::core::ffi::c_double
                    } else {
                        tau as ::core::ffi::c_double
                    }) as doublereal;
                } else {
                    sg2ub = (if sg2ub <= tau {
                        sg2ub as ::core::ffi::c_double
                    } else {
                        tau as ::core::ffi::c_double
                    }) as doublereal;
                }
                niter += 1;
                if swtch3 == 0 {
                    dtipsq = *work.offset(ip1 as isize) * *delta.offset(ip1 as isize);
                    dtisq = *work.offset(*i__ as isize) * *delta.offset(*i__ as isize);
                    if orgati != 0 {
                        d__1 = *z__.offset(*i__ as isize) / dtisq;
                        c__ = w - dtipsq * dw + delsq * (d__1 * d__1);
                    } else {
                        d__1 = *z__.offset(ip1 as isize) / dtipsq;
                        c__ = w - dtisq * dw - delsq * (d__1 * d__1);
                    }
                    a = (dtipsq + dtisq) * w - dtipsq * dtisq * dw;
                    b = dtipsq * dtisq * w;
                    if c__ == 0.0f64 {
                        if a == 0.0f64 {
                            if orgati != 0 {
                                a = *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                                    + dtipsq * dtipsq * (dpsi + dphi);
                            } else {
                                a = *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize)
                                    + dtisq * dtisq * (dpsi + dphi);
                            }
                        }
                        eta = b / a;
                    } else if a <= 0.0f64 {
                        d__1 = a * a - b * 4.0f64 * c__;
                        eta = ((a as ::core::ffi::c_double
                            - super::lapack_sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))
                            / (c__ as ::core::ffi::c_double * 2.0f64))
                            as doublereal;
                    } else {
                        d__1 = a * a - b * 4.0f64 * c__;
                        eta = (b as ::core::ffi::c_double * 2.0f64
                            / (a as ::core::ffi::c_double
                                + super::lapack_sqrt(
                                    (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1
                                    } else {
                                        -d__1
                                    }),
                                ))) as doublereal;
                    }
                    current_block = 10008106309883251951;
                } else {
                    dtiim = *work.offset(iim1 as isize) * *delta.offset(iim1 as isize);
                    dtiip = *work.offset(iip1 as isize) * *delta.offset(iip1 as isize);
                    temp = rhoinv + psi + phi;
                    if orgati != 0 {
                        temp1 = *z__.offset(iim1 as isize) / dtiim;
                        temp1 *= temp1 as ::core::ffi::c_double;
                        c__ = temp
                            - dtiip * (dpsi + dphi)
                            - (*d__.offset(iim1 as isize) - *d__.offset(iip1 as isize))
                                * (*d__.offset(iim1 as isize) + *d__.offset(iip1 as isize))
                                * temp1;
                        zz[0 as ::core::ffi::c_int as usize] =
                            *z__.offset(iim1 as isize) * *z__.offset(iim1 as isize);
                        if dpsi < temp1 {
                            zz[2 as ::core::ffi::c_int as usize] = dtiip * dtiip * dphi;
                        } else {
                            zz[2 as ::core::ffi::c_int as usize] =
                                dtiip * dtiip * (dpsi - temp1 + dphi);
                        }
                    } else {
                        temp1 = *z__.offset(iip1 as isize) / dtiip;
                        temp1 *= temp1 as ::core::ffi::c_double;
                        c__ = temp
                            - dtiim * (dpsi + dphi)
                            - (*d__.offset(iip1 as isize) - *d__.offset(iim1 as isize))
                                * (*d__.offset(iim1 as isize) + *d__.offset(iip1 as isize))
                                * temp1;
                        if dphi < temp1 {
                            zz[0 as ::core::ffi::c_int as usize] = dtiim * dtiim * dpsi;
                        } else {
                            zz[0 as ::core::ffi::c_int as usize] =
                                dtiim * dtiim * (dpsi + (dphi - temp1));
                        }
                        zz[2 as ::core::ffi::c_int as usize] =
                            *z__.offset(iip1 as isize) * *z__.offset(iip1 as isize);
                    }
                    zz[1 as ::core::ffi::c_int as usize] =
                        *z__.offset(ii as isize) * *z__.offset(ii as isize);
                    dd[0 as ::core::ffi::c_int as usize] = dtiim;
                    dd[1 as ::core::ffi::c_int as usize] =
                        *delta.offset(ii as isize) * *work.offset(ii as isize);
                    dd[2 as ::core::ffi::c_int as usize] = dtiip;
                    dlaed6__0(
                        &raw mut niter,
                        &raw mut orgati,
                        &raw mut c__,
                        &raw mut dd as *mut doublereal,
                        &raw mut zz as *mut doublereal,
                        &raw mut w,
                        &raw mut eta,
                        info,
                    );
                    if *info != 0 as ::core::ffi::c_long {
                        current_block = 10032952401960431757;
                    } else {
                        current_block = 10008106309883251951;
                    }
                }
                match current_block {
                    10032952401960431757 => {}
                    _ => {
                        if w * eta >= 0.0f64 {
                            eta = -w / dw;
                        }
                        if orgati != 0 {
                            temp1 = *work.offset(*i__ as isize) * *delta.offset(*i__ as isize);
                            temp = eta - temp1;
                        } else {
                            temp1 = *work.offset(ip1 as isize) * *delta.offset(ip1 as isize);
                            temp = eta - temp1;
                        }
                        if temp > sg2ub || temp < sg2lb {
                            if w < 0.0f64 {
                                eta = ((sg2ub as ::core::ffi::c_double
                                    - tau as ::core::ffi::c_double)
                                    / 2.0f64) as doublereal;
                            } else {
                                eta = ((sg2lb as ::core::ffi::c_double
                                    - tau as ::core::ffi::c_double)
                                    / 2.0f64) as doublereal;
                            }
                        }
                        tau += eta as ::core::ffi::c_double;
                        eta /= *sigma + super::lapack_sqrt(*sigma * *sigma + eta);
                        prew = w;
                        *sigma += eta as ::core::ffi::c_double;
                        i__1 = *n;
                        j = 1 as integer;
                        while j <= i__1 {
                            let ref mut fresh4 = *work.offset(j as isize);
                            *fresh4 += eta as ::core::ffi::c_double;
                            let ref mut fresh5 = *delta.offset(j as isize);
                            *fresh5 -= eta as ::core::ffi::c_double;
                            j += 1;
                        }
                        dpsi = 0.0f64 as doublereal;
                        psi = 0.0f64 as doublereal;
                        erretm = 0.0f64 as doublereal;
                        i__1 = iim1;
                        j = 1 as integer;
                        while j <= i__1 {
                            temp = *z__.offset(j as isize)
                                / (*work.offset(j as isize) * *delta.offset(j as isize));
                            psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                            dpsi += (temp * temp) as ::core::ffi::c_double;
                            erretm += psi as ::core::ffi::c_double;
                            j += 1;
                        }
                        erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            erretm as ::core::ffi::c_double
                        } else {
                            -(erretm as ::core::ffi::c_double)
                        }) as doublereal;
                        dphi = 0.0f64 as doublereal;
                        phi = 0.0f64 as doublereal;
                        i__1 = iip1;
                        j = *n;
                        while j >= i__1 {
                            temp = *z__.offset(j as isize)
                                / (*work.offset(j as isize) * *delta.offset(j as isize));
                            phi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                            dphi += (temp * temp) as ::core::ffi::c_double;
                            erretm += phi as ::core::ffi::c_double;
                            j -= 1;
                        }
                        temp = *z__.offset(ii as isize)
                            / (*work.offset(ii as isize) * *delta.offset(ii as isize));
                        dw = dpsi + dphi + temp * temp;
                        temp = *z__.offset(ii as isize) * temp;
                        w = rhoinv + phi + psi + temp;
                        erretm = (phi - psi) * 8.0f64
                            + erretm
                            + rhoinv * 2.0f64
                            + (if temp >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                temp
                            } else {
                                -temp
                            }) * 3.0f64
                            + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                tau
                            } else {
                                -tau
                            }) * dw;
                        if w <= 0.0f64 {
                            sg2lb = (if sg2lb >= tau {
                                sg2lb as ::core::ffi::c_double
                            } else {
                                tau as ::core::ffi::c_double
                            }) as doublereal;
                        } else {
                            sg2ub = (if sg2ub <= tau {
                                sg2ub as ::core::ffi::c_double
                            } else {
                                tau as ::core::ffi::c_double
                            }) as doublereal;
                        }
                        swtch = FALSE_ as logical;
                        if orgati != 0 {
                            if -w
                                > (if prew >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    prew as ::core::ffi::c_double
                                } else {
                                    -(prew as ::core::ffi::c_double)
                                }) / 10.0f64
                            {
                                swtch = TRUE_ as logical;
                            }
                        } else if w
                            > (if prew >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                prew as ::core::ffi::c_double
                            } else {
                                -(prew as ::core::ffi::c_double)
                            }) / 10.0f64
                        {
                            swtch = TRUE_ as logical;
                        }
                        iter = (niter as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        niter = iter;
                        loop {
                            if !(niter <= 20 as ::core::ffi::c_long) {
                                current_block = 9256118611181159293;
                                break;
                            }
                            if (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                w as ::core::ffi::c_double
                            } else {
                                -(w as ::core::ffi::c_double)
                            }) <= eps * erretm
                            {
                                current_block = 10032952401960431757;
                                break;
                            }
                            if swtch3 == 0 {
                                dtipsq = *work.offset(ip1 as isize) * *delta.offset(ip1 as isize);
                                dtisq = *work.offset(*i__ as isize) * *delta.offset(*i__ as isize);
                                if swtch == 0 {
                                    if orgati != 0 {
                                        d__1 = *z__.offset(*i__ as isize) / dtisq;
                                        c__ = w - dtipsq * dw + delsq * (d__1 * d__1);
                                    } else {
                                        d__1 = *z__.offset(ip1 as isize) / dtipsq;
                                        c__ = w - dtisq * dw - delsq * (d__1 * d__1);
                                    }
                                } else {
                                    temp = *z__.offset(ii as isize)
                                        / (*work.offset(ii as isize) * *delta.offset(ii as isize));
                                    if orgati != 0 {
                                        dpsi += (temp * temp) as ::core::ffi::c_double;
                                    } else {
                                        dphi += (temp * temp) as ::core::ffi::c_double;
                                    }
                                    c__ = w - dtisq * dpsi - dtipsq * dphi;
                                }
                                a = (dtipsq + dtisq) * w - dtipsq * dtisq * dw;
                                b = dtipsq * dtisq * w;
                                if c__ == 0.0f64 {
                                    if a == 0.0f64 {
                                        if swtch == 0 {
                                            if orgati != 0 {
                                                a = *z__.offset(*i__ as isize)
                                                    * *z__.offset(*i__ as isize)
                                                    + dtipsq * dtipsq * (dpsi + dphi);
                                            } else {
                                                a = *z__.offset(ip1 as isize)
                                                    * *z__.offset(ip1 as isize)
                                                    + dtisq * dtisq * (dpsi + dphi);
                                            }
                                        } else {
                                            a = dtisq * dtisq * dpsi + dtipsq * dtipsq * dphi;
                                        }
                                    }
                                    eta = b / a;
                                } else if a <= 0.0f64 {
                                    d__1 = a * a - b * 4.0f64 * c__;
                                    eta = ((a as ::core::ffi::c_double
                                        - super::lapack_sqrt(
                                            (if d__1
                                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            {
                                                d__1
                                            } else {
                                                -d__1
                                            }),
                                        ))
                                        / (c__ as ::core::ffi::c_double * 2.0f64))
                                        as doublereal;
                                } else {
                                    d__1 = a * a - b * 4.0f64 * c__;
                                    eta = (b as ::core::ffi::c_double * 2.0f64
                                        / (a as ::core::ffi::c_double
                                            + super::lapack_sqrt(
                                                (if d__1
                                                    >= 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                {
                                                    d__1
                                                } else {
                                                    -d__1
                                                }),
                                            )))
                                        as doublereal;
                                }
                            } else {
                                dtiim = *work.offset(iim1 as isize) * *delta.offset(iim1 as isize);
                                dtiip = *work.offset(iip1 as isize) * *delta.offset(iip1 as isize);
                                temp = rhoinv + psi + phi;
                                if swtch != 0 {
                                    c__ = temp - dtiim * dpsi - dtiip * dphi;
                                    zz[0 as ::core::ffi::c_int as usize] = dtiim * dtiim * dpsi;
                                    zz[2 as ::core::ffi::c_int as usize] = dtiip * dtiip * dphi;
                                } else if orgati != 0 {
                                    temp1 = *z__.offset(iim1 as isize) / dtiim;
                                    temp1 *= temp1 as ::core::ffi::c_double;
                                    temp2 = (*d__.offset(iim1 as isize)
                                        - *d__.offset(iip1 as isize))
                                        * (*d__.offset(iim1 as isize) + *d__.offset(iip1 as isize))
                                        * temp1;
                                    c__ = temp - dtiip * (dpsi + dphi) - temp2;
                                    zz[0 as ::core::ffi::c_int as usize] =
                                        *z__.offset(iim1 as isize) * *z__.offset(iim1 as isize);
                                    if dpsi < temp1 {
                                        zz[2 as ::core::ffi::c_int as usize] = dtiip * dtiip * dphi;
                                    } else {
                                        zz[2 as ::core::ffi::c_int as usize] =
                                            dtiip * dtiip * (dpsi - temp1 + dphi);
                                    }
                                } else {
                                    temp1 = *z__.offset(iip1 as isize) / dtiip;
                                    temp1 *= temp1 as ::core::ffi::c_double;
                                    temp2 = (*d__.offset(iip1 as isize)
                                        - *d__.offset(iim1 as isize))
                                        * (*d__.offset(iim1 as isize) + *d__.offset(iip1 as isize))
                                        * temp1;
                                    c__ = temp - dtiim * (dpsi + dphi) - temp2;
                                    if dphi < temp1 {
                                        zz[0 as ::core::ffi::c_int as usize] = dtiim * dtiim * dpsi;
                                    } else {
                                        zz[0 as ::core::ffi::c_int as usize] =
                                            dtiim * dtiim * (dpsi + (dphi - temp1));
                                    }
                                    zz[2 as ::core::ffi::c_int as usize] =
                                        *z__.offset(iip1 as isize) * *z__.offset(iip1 as isize);
                                }
                                dd[0 as ::core::ffi::c_int as usize] = dtiim;
                                dd[1 as ::core::ffi::c_int as usize] =
                                    *delta.offset(ii as isize) * *work.offset(ii as isize);
                                dd[2 as ::core::ffi::c_int as usize] = dtiip;
                                dlaed6__0(
                                    &raw mut niter,
                                    &raw mut orgati,
                                    &raw mut c__,
                                    &raw mut dd as *mut doublereal,
                                    &raw mut zz as *mut doublereal,
                                    &raw mut w,
                                    &raw mut eta,
                                    info,
                                );
                                if *info != 0 as ::core::ffi::c_long {
                                    current_block = 10032952401960431757;
                                    break;
                                }
                            }
                            if w * eta >= 0.0f64 {
                                eta = -w / dw;
                            }
                            if orgati != 0 {
                                temp1 = *work.offset(*i__ as isize) * *delta.offset(*i__ as isize);
                                temp = eta - temp1;
                            } else {
                                temp1 = *work.offset(ip1 as isize) * *delta.offset(ip1 as isize);
                                temp = eta - temp1;
                            }
                            if temp > sg2ub || temp < sg2lb {
                                if w < 0.0f64 {
                                    eta = ((sg2ub as ::core::ffi::c_double
                                        - tau as ::core::ffi::c_double)
                                        / 2.0f64)
                                        as doublereal;
                                } else {
                                    eta = ((sg2lb as ::core::ffi::c_double
                                        - tau as ::core::ffi::c_double)
                                        / 2.0f64)
                                        as doublereal;
                                }
                            }
                            tau += eta as ::core::ffi::c_double;
                            eta /= *sigma + super::lapack_sqrt(*sigma * *sigma + eta);
                            *sigma += eta as ::core::ffi::c_double;
                            i__1 = *n;
                            j = 1 as integer;
                            while j <= i__1 {
                                let ref mut fresh6 = *work.offset(j as isize);
                                *fresh6 += eta as ::core::ffi::c_double;
                                let ref mut fresh7 = *delta.offset(j as isize);
                                *fresh7 -= eta as ::core::ffi::c_double;
                                j += 1;
                            }
                            prew = w;
                            dpsi = 0.0f64 as doublereal;
                            psi = 0.0f64 as doublereal;
                            erretm = 0.0f64 as doublereal;
                            i__1 = iim1;
                            j = 1 as integer;
                            while j <= i__1 {
                                temp = *z__.offset(j as isize)
                                    / (*work.offset(j as isize) * *delta.offset(j as isize));
                                psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                                dpsi += (temp * temp) as ::core::ffi::c_double;
                                erretm += psi as ::core::ffi::c_double;
                                j += 1;
                            }
                            erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                erretm as ::core::ffi::c_double
                            } else {
                                -(erretm as ::core::ffi::c_double)
                            }) as doublereal;
                            dphi = 0.0f64 as doublereal;
                            phi = 0.0f64 as doublereal;
                            i__1 = iip1;
                            j = *n;
                            while j >= i__1 {
                                temp = *z__.offset(j as isize)
                                    / (*work.offset(j as isize) * *delta.offset(j as isize));
                                phi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                                dphi += (temp * temp) as ::core::ffi::c_double;
                                erretm += phi as ::core::ffi::c_double;
                                j -= 1;
                            }
                            temp = *z__.offset(ii as isize)
                                / (*work.offset(ii as isize) * *delta.offset(ii as isize));
                            dw = dpsi + dphi + temp * temp;
                            temp = *z__.offset(ii as isize) * temp;
                            w = rhoinv + phi + psi + temp;
                            erretm = (phi - psi) * 8.0f64
                                + erretm
                                + rhoinv * 2.0f64
                                + (if temp >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    temp
                                } else {
                                    -temp
                                }) * 3.0f64
                                + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    tau
                                } else {
                                    -tau
                                }) * dw;
                            if w * prew > 0.0f64
                                && (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    w as ::core::ffi::c_double
                                } else {
                                    -(w as ::core::ffi::c_double)
                                }) > (if prew >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    prew as ::core::ffi::c_double
                                } else {
                                    -(prew as ::core::ffi::c_double)
                                }) / 10.0f64
                            {
                                swtch = (swtch == 0) as ::core::ffi::c_int as logical;
                            }
                            if w <= 0.0f64 {
                                sg2lb = (if sg2lb >= tau {
                                    sg2lb as ::core::ffi::c_double
                                } else {
                                    tau as ::core::ffi::c_double
                                }) as doublereal;
                            } else {
                                sg2ub = (if sg2ub <= tau {
                                    sg2ub as ::core::ffi::c_double
                                } else {
                                    tau as ::core::ffi::c_double
                                }) as doublereal;
                            }
                            niter += 1;
                        }
                        match current_block {
                            10032952401960431757 => {}
                            _ => {
                                *info = 1 as integer;
                            }
                        }
                    }
                }
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasd4::dgelsd_closure_dlasd4_;

pub mod raw_dlasd5 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasd5_(
        mut i__: *mut integer,
        mut d__: *mut doublereal,
        mut z__: *mut doublereal,
        mut delta: *mut doublereal,
        mut rho: *mut doublereal,
        mut dsigma: *mut doublereal,
        mut work: *mut doublereal,
    ) -> ::core::ffi::c_int {
        let mut d__1: doublereal = 0.;
        let mut b: doublereal = 0.;
        let mut c__: doublereal = 0.;
        let mut w: doublereal = 0.;
        let mut del: doublereal = 0.;
        let mut tau: doublereal = 0.;
        let mut delsq: doublereal = 0.;
        work = work.wrapping_offset(-1);
        delta = delta.wrapping_offset(-1);
        z__ = z__.wrapping_offset(-1);
        d__ = d__.wrapping_offset(-1);
        del = *d__.offset(2 as ::core::ffi::c_int as isize)
            - *d__.offset(1 as ::core::ffi::c_int as isize);
        delsq = del
            * (*d__.offset(2 as ::core::ffi::c_int as isize)
                + *d__.offset(1 as ::core::ffi::c_int as isize));
        if *i__ == 1 as ::core::ffi::c_long {
            w = (*rho
                * 4.0f64
                * (*z__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    * *z__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    / (*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                        + *d__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                            * 3.0f64)
                    - *z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                        * *z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                        / (*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                            * 3.0f64
                            + *d__.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_double))
                / del as ::core::ffi::c_double
                + 1.0f64) as doublereal;
            if w > 0.0f64 {
                b = delsq
                    + *rho
                        * (*z__.offset(1 as ::core::ffi::c_int as isize)
                            * *z__.offset(1 as ::core::ffi::c_int as isize)
                            + *z__.offset(2 as ::core::ffi::c_int as isize)
                                * *z__.offset(2 as ::core::ffi::c_int as isize));
                c__ = *rho
                    * *z__.offset(1 as ::core::ffi::c_int as isize)
                    * *z__.offset(1 as ::core::ffi::c_int as isize)
                    * delsq;
                d__1 = (b as ::core::ffi::c_double * b as ::core::ffi::c_double
                    - c__ as ::core::ffi::c_double * 4.0f64) as doublereal;
                tau = (c__ as ::core::ffi::c_double * 2.0f64
                    / (b as ::core::ffi::c_double
                        + super::lapack_sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))) as doublereal;
                tau /= *d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    + super::lapack_sqrt(
                        *d__.offset(1 as ::core::ffi::c_int as isize)
                            * *d__.offset(1 as ::core::ffi::c_int as isize)
                            + tau,
                    );
                *dsigma = *d__.offset(1 as ::core::ffi::c_int as isize) + tau;
                *delta.offset(1 as ::core::ffi::c_int as isize) = -tau;
                *delta.offset(2 as ::core::ffi::c_int as isize) = del - tau;
                *work.offset(1 as ::core::ffi::c_int as isize) =
                    *d__.offset(1 as ::core::ffi::c_int as isize) * 2.0f64 + tau;
                *work.offset(2 as ::core::ffi::c_int as isize) = *d__
                    .offset(1 as ::core::ffi::c_int as isize)
                    + tau
                    + *d__.offset(2 as ::core::ffi::c_int as isize);
            } else {
                b = -delsq
                    + *rho
                        * (*z__.offset(1 as ::core::ffi::c_int as isize)
                            * *z__.offset(1 as ::core::ffi::c_int as isize)
                            + *z__.offset(2 as ::core::ffi::c_int as isize)
                                * *z__.offset(2 as ::core::ffi::c_int as isize));
                c__ = *rho
                    * *z__.offset(2 as ::core::ffi::c_int as isize)
                    * *z__.offset(2 as ::core::ffi::c_int as isize)
                    * delsq;
                if b > 0.0f64 {
                    tau = (c__ as ::core::ffi::c_double * -2.0f64
                        / (b as ::core::ffi::c_double + super::lapack_sqrt(b * b + c__ * 4.0f64)))
                        as doublereal;
                } else {
                    tau = ((b as ::core::ffi::c_double - super::lapack_sqrt(b * b + c__ * 4.0f64))
                        / 2.0f64) as doublereal;
                }
                d__1 = *d__.offset(2 as ::core::ffi::c_int as isize)
                    * *d__.offset(2 as ::core::ffi::c_int as isize)
                    + tau;
                tau /= *d__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    + super::lapack_sqrt(
                        (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1
                        } else {
                            -d__1
                        }),
                    );
                *dsigma = *d__.offset(2 as ::core::ffi::c_int as isize) + tau;
                *delta.offset(1 as ::core::ffi::c_int as isize) = -(del + tau);
                *delta.offset(2 as ::core::ffi::c_int as isize) = -tau;
                *work.offset(1 as ::core::ffi::c_int as isize) = *d__
                    .offset(1 as ::core::ffi::c_int as isize)
                    + tau
                    + *d__.offset(2 as ::core::ffi::c_int as isize);
                *work.offset(2 as ::core::ffi::c_int as isize) =
                    *d__.offset(2 as ::core::ffi::c_int as isize) * 2.0f64 + tau;
            }
        } else {
            b = -delsq
                + *rho
                    * (*z__.offset(1 as ::core::ffi::c_int as isize)
                        * *z__.offset(1 as ::core::ffi::c_int as isize)
                        + *z__.offset(2 as ::core::ffi::c_int as isize)
                            * *z__.offset(2 as ::core::ffi::c_int as isize));
            c__ = *rho
                * *z__.offset(2 as ::core::ffi::c_int as isize)
                * *z__.offset(2 as ::core::ffi::c_int as isize)
                * delsq;
            if b > 0.0f64 {
                tau = ((b as ::core::ffi::c_double + super::lapack_sqrt(b * b + c__ * 4.0f64))
                    / 2.0f64) as doublereal;
            } else {
                tau = (c__ as ::core::ffi::c_double * 2.0f64
                    / (-(b as ::core::ffi::c_double) + super::lapack_sqrt(b * b + c__ * 4.0f64)))
                    as doublereal;
            }
            tau /= *d__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                + super::lapack_sqrt(
                    *d__.offset(2 as ::core::ffi::c_int as isize)
                        * *d__.offset(2 as ::core::ffi::c_int as isize)
                        + tau,
                );
            *dsigma = *d__.offset(2 as ::core::ffi::c_int as isize) + tau;
            *delta.offset(1 as ::core::ffi::c_int as isize) = -(del + tau);
            *delta.offset(2 as ::core::ffi::c_int as isize) = -tau;
            *work.offset(1 as ::core::ffi::c_int as isize) = *d__
                .offset(1 as ::core::ffi::c_int as isize)
                + tau
                + *d__.offset(2 as ::core::ffi::c_int as isize);
            *work.offset(2 as ::core::ffi::c_int as isize) =
                *d__.offset(2 as ::core::ffi::c_int as isize) * 2.0f64 + tau;
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasd5::dgelsd_closure_dlasd5_;

pub mod raw_dlasd6 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c__0: integer = 0 as integer;
    static mut c_b7: doublereal = 1.0f64;
    static mut c__1: integer = 1 as integer;
    static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasd6_(
        mut icompq: *mut integer,
        mut nl: *mut integer,
        mut nr: *mut integer,
        mut sqre: *mut integer,
        mut d__: *mut doublereal,
        mut vf: *mut doublereal,
        mut vl: *mut doublereal,
        mut alpha: *mut doublereal,
        mut beta: *mut doublereal,
        mut idxq: *mut integer,
        mut perm: *mut integer,
        mut givptr: *mut integer,
        mut givcol: *mut integer,
        mut ldgcol: *mut integer,
        mut givnum: *mut doublereal,
        mut ldgnum: *mut integer,
        mut poles: *mut doublereal,
        mut difl: *mut doublereal,
        mut difr: *mut doublereal,
        mut z__: *mut doublereal,
        mut k: *mut integer,
        mut c__: *mut doublereal,
        mut s: *mut doublereal,
        mut work: *mut doublereal,
        mut iwork: *mut integer,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut givcol_dim1: integer = 0;
        let mut givcol_offset: integer = 0;
        let mut givnum_dim1: integer = 0;
        let mut givnum_offset: integer = 0;
        let mut poles_dim1: integer = 0;
        let mut poles_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut i__: integer = 0;
        let mut m: integer = 0;
        let mut n: integer = 0;
        let mut n1: integer = 0;
        let mut n2: integer = 0;
        let mut iw: integer = 0;
        let mut idx: integer = 0;
        let mut idxc: integer = 0;
        let mut idxp: integer = 0;
        let mut ivfw: integer = 0;
        let mut ivlw: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasd7_"]
            fn dlasd7__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasd8_"]
            fn dlasd8__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlascl_"]
            fn dlascl__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamrg_"]
            fn dlamrg__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut isigma: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut orgnrm: doublereal = 0.;
        d__ = d__.wrapping_offset(-1);
        vf = vf.wrapping_offset(-1);
        vl = vl.wrapping_offset(-1);
        idxq = idxq.wrapping_offset(-1);
        perm = perm.wrapping_offset(-1);
        givcol_dim1 = *ldgcol;
        givcol_offset = 1 as integer + givcol_dim1;
        givcol = givcol.wrapping_offset(-(givcol_offset as isize));
        poles_dim1 = *ldgnum;
        poles_offset = 1 as integer + poles_dim1;
        poles = poles.wrapping_offset(-(poles_offset as isize));
        givnum_dim1 = *ldgnum;
        givnum_offset = 1 as integer + givnum_dim1;
        givnum = givnum.wrapping_offset(-(givnum_offset as isize));
        difl = difl.wrapping_offset(-1);
        difr = difr.wrapping_offset(-1);
        z__ = z__.wrapping_offset(-1);
        work = work.wrapping_offset(-1);
        iwork = iwork.wrapping_offset(-1);
        *info = 0 as integer;
        n = (*nl + *nr + 1 as ::core::ffi::c_long) as integer;
        m = n + *sqre;
        if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *nl < 1 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *nr < 1 as ::core::ffi::c_long {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *sqre < 0 as ::core::ffi::c_long || *sqre > 1 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if *ldgcol < n {
            *info = -(14 as ::core::ffi::c_int) as integer;
        } else if *ldgnum < n {
            *info = -(16 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLASD6\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        isigma = 1 as integer;
        iw = isigma + n;
        ivfw = iw + m;
        ivlw = ivfw + m;
        idx = 1 as integer;
        idxc = idx + n;
        idxp = idxc + n;
        d__1 = (if *alpha >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *alpha
        } else {
            -*alpha
        }) as doublereal;
        d__2 = (if *beta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *beta
        } else {
            -*beta
        }) as doublereal;
        orgnrm = (if d__1 >= d__2 {
            d__1 as ::core::ffi::c_double
        } else {
            d__2 as ::core::ffi::c_double
        }) as doublereal;
        *d__.offset((*nl + 1 as ::core::ffi::c_long) as isize) = 0.0f64 as doublereal;
        i__1 = n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *d__.offset(i__ as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) > orgnrm
            {
                d__1 = *d__.offset(i__ as isize);
                orgnrm = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
            }
            i__ += 1;
        }
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut orgnrm,
            &raw mut c_b7,
            &raw mut n,
            &raw mut c__1,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut n,
            info,
        );
        *alpha /= orgnrm as ::core::ffi::c_double;
        *beta /= orgnrm as ::core::ffi::c_double;
        dlasd7__0(
            icompq,
            nl,
            nr,
            sqre,
            k,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(iw as isize) as *mut doublereal,
            vf.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(ivfw as isize) as *mut doublereal,
            vl.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(ivlw as isize) as *mut doublereal,
            alpha,
            beta,
            work.offset(isigma as isize) as *mut doublereal,
            iwork.offset(idx as isize) as *mut integer,
            iwork.offset(idxp as isize) as *mut integer,
            idxq.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
            perm.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
            givptr,
            givcol.offset(givcol_offset as isize) as *mut integer,
            ldgcol,
            givnum.offset(givnum_offset as isize) as *mut doublereal,
            ldgnum,
            c__,
            s,
            info,
        );
        dlasd8__0(
            icompq,
            k,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vf.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vl.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            difl.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            difr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            ldgnum,
            work.offset(isigma as isize) as *mut doublereal,
            work.offset(iw as isize) as *mut doublereal,
            info,
        );
        if *icompq == 1 as ::core::ffi::c_long {
            f2c_dcopy_0(
                k,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                poles
                    .offset((poles_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
            );
            f2c_dcopy_0(
                k,
                work.offset(isigma as isize) as *mut doublereal,
                &raw mut c__1,
                poles.offset(
                    (((poles_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
            );
        }
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut c_b7,
            &raw mut orgnrm,
            &raw mut n,
            &raw mut c__1,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut n,
            info,
        );
        n1 = *k;
        n2 = n - *k;
        dlamrg__0(
            &raw mut n1,
            &raw mut n2,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            &raw mut c_n1,
            idxq.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        );
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasd6::dgelsd_closure_dlasd6_;

pub mod raw_dlasd7 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c__1: integer = 1 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasd7_(
        mut icompq: *mut integer,
        mut nl: *mut integer,
        mut nr: *mut integer,
        mut sqre: *mut integer,
        mut k: *mut integer,
        mut d__: *mut doublereal,
        mut z__: *mut doublereal,
        mut zw: *mut doublereal,
        mut vf: *mut doublereal,
        mut vfw: *mut doublereal,
        mut vl: *mut doublereal,
        mut vlw: *mut doublereal,
        mut alpha: *mut doublereal,
        mut beta: *mut doublereal,
        mut dsigma: *mut doublereal,
        mut idx: *mut integer,
        mut idxp: *mut integer,
        mut idxq: *mut integer,
        mut perm: *mut integer,
        mut givptr: *mut integer,
        mut givcol: *mut integer,
        mut ldgcol: *mut integer,
        mut givnum: *mut doublereal,
        mut ldgnum: *mut integer,
        mut c__: *mut doublereal,
        mut s: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut current_block: u64;
        let mut givcol_dim1: integer = 0;
        let mut givcol_offset: integer = 0;
        let mut givnum_dim1: integer = 0;
        let mut givnum_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut m: integer = 0;
        let mut n: integer = 0;
        let mut k2: integer = 0;
        let mut z1: doublereal = 0.;
        let mut jp: integer = 0;
        let mut eps: doublereal = 0.;
        let mut tau: doublereal = 0.;
        let mut tol: doublereal = 0.;
        let mut nlp1: integer = 0;
        let mut nlp2: integer = 0;
        let mut idxi: integer = 0;
        let mut idxj: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_drot"]
            fn f2c_drot_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        let mut idxjp: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut jprev: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlapy2_"]
            fn dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamrg_"]
            fn dlamrg__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut hlftol: doublereal = 0.;
        d__ = d__.wrapping_offset(-1);
        z__ = z__.wrapping_offset(-1);
        zw = zw.wrapping_offset(-1);
        vf = vf.wrapping_offset(-1);
        vfw = vfw.wrapping_offset(-1);
        vl = vl.wrapping_offset(-1);
        vlw = vlw.wrapping_offset(-1);
        dsigma = dsigma.wrapping_offset(-1);
        idx = idx.wrapping_offset(-1);
        idxp = idxp.wrapping_offset(-1);
        idxq = idxq.wrapping_offset(-1);
        perm = perm.wrapping_offset(-1);
        givcol_dim1 = *ldgcol;
        givcol_offset = 1 as integer + givcol_dim1;
        givcol = givcol.wrapping_offset(-(givcol_offset as isize));
        givnum_dim1 = *ldgnum;
        givnum_offset = 1 as integer + givnum_dim1;
        givnum = givnum.wrapping_offset(-(givnum_offset as isize));
        *info = 0 as integer;
        n = (*nl + *nr + 1 as ::core::ffi::c_long) as integer;
        m = n + *sqre;
        if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *nl < 1 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *nr < 1 as ::core::ffi::c_long {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *sqre < 0 as ::core::ffi::c_long || *sqre > 1 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if *ldgcol < n {
            *info = -(22 as ::core::ffi::c_int) as integer;
        } else if *ldgnum < n {
            *info = -(24 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLASD7\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        nlp1 = (*nl + 1 as ::core::ffi::c_long) as integer;
        nlp2 = (*nl + 2 as ::core::ffi::c_long) as integer;
        if *icompq == 1 as ::core::ffi::c_long {
            *givptr = 0 as integer;
        }
        z1 = *alpha * *vl.offset(nlp1 as isize);
        *vl.offset(nlp1 as isize) = 0.0f64 as doublereal;
        tau = *vf.offset(nlp1 as isize);
        i__ = *nl;
        while i__ >= 1 as ::core::ffi::c_long {
            *z__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                *alpha * *vl.offset(i__ as isize);
            *vl.offset(i__ as isize) = 0.0f64 as doublereal;
            *vf.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                *vf.offset(i__ as isize);
            *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                *d__.offset(i__ as isize);
            *idxq.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                (*idxq.offset(i__ as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    as integer;
            i__ -= 1;
        }
        *vf.offset(1 as ::core::ffi::c_int as isize) = tau;
        i__1 = m;
        i__ = nlp2;
        while i__ <= i__1 {
            *z__.offset(i__ as isize) = *beta * *vf.offset(i__ as isize);
            *vf.offset(i__ as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        i__1 = n;
        i__ = nlp2;
        while i__ <= i__1 {
            let ref mut fresh0 = *idxq.offset(i__ as isize);
            *fresh0 += nlp1 as ::core::ffi::c_long;
            i__ += 1;
        }
        i__1 = n;
        i__ = 2 as integer;
        while i__ <= i__1 {
            *dsigma.offset(i__ as isize) = *d__.offset(*idxq.offset(i__ as isize) as isize);
            *zw.offset(i__ as isize) = *z__.offset(*idxq.offset(i__ as isize) as isize);
            *vfw.offset(i__ as isize) = *vf.offset(*idxq.offset(i__ as isize) as isize);
            *vlw.offset(i__ as isize) = *vl.offset(*idxq.offset(i__ as isize) as isize);
            i__ += 1;
        }
        dlamrg__0(
            nl,
            nr,
            dsigma.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            &raw mut c__1,
            idx.offset(2 as ::core::ffi::c_int as isize) as *mut integer,
        );
        i__1 = n;
        i__ = 2 as integer;
        while i__ <= i__1 {
            idxi = (*idx.offset(i__ as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as integer;
            *d__.offset(i__ as isize) = *dsigma.offset(idxi as isize);
            *z__.offset(i__ as isize) = *zw.offset(idxi as isize);
            *vf.offset(i__ as isize) = *vfw.offset(idxi as isize);
            *vl.offset(i__ as isize) = *vlw.offset(idxi as isize);
            i__ += 1;
        }
        eps = dlamch__0(
            b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        d__1 = (if *alpha >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *alpha
        } else {
            -*alpha
        }) as doublereal;
        d__2 = (if *beta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *beta
        } else {
            -*beta
        }) as doublereal;
        tol = (if d__1 >= d__2 {
            d__1 as ::core::ffi::c_double
        } else {
            d__2 as ::core::ffi::c_double
        }) as doublereal;
        d__1 = *d__.offset(n as isize);
        d__2 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        }) as doublereal;
        tol = (eps as ::core::ffi::c_double
            * 64.0f64
            * (if d__2 >= tol {
                d__2 as ::core::ffi::c_double
            } else {
                tol as ::core::ffi::c_double
            })) as doublereal;
        *k = 1 as integer;
        k2 = (n as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__1 = n;
        j = 2 as integer;
        loop {
            if !(j <= i__1) {
                current_block = 15886382916857677249;
                break;
            }
            d__1 = *z__.offset(j as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) <= tol
            {
                k2 -= 1;
                *idxp.offset(k2 as isize) = j;
                if j == n {
                    current_block = 9355436837411392774;
                    break;
                }
                j += 1;
            } else {
                jprev = j;
                current_block = 15886382916857677249;
                break;
            }
        }
        match current_block {
            15886382916857677249 => {
                j = jprev;
                loop {
                    j += 1;
                    if j > n {
                        break;
                    }
                    d__1 = *z__.offset(j as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) <= tol
                    {
                        k2 -= 1;
                        *idxp.offset(k2 as isize) = j;
                    } else {
                        d__1 = *d__.offset(j as isize) - *d__.offset(jprev as isize);
                        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) <= tol
                        {
                            *s = *z__.offset(jprev as isize);
                            *c__ = *z__.offset(j as isize);
                            tau = dlapy2__0(c__, s);
                            *z__.offset(j as isize) = tau;
                            *z__.offset(jprev as isize) = 0.0f64 as doublereal;
                            *c__ /= tau as ::core::ffi::c_double;
                            *s = -*s / tau;
                            if *icompq == 1 as ::core::ffi::c_long {
                                *givptr += 1;
                                idxjp = *idxq.offset(
                                    (*idx.offset(jprev as isize) as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                );
                                idxj = *idxq.offset(
                                    (*idx.offset(j as isize) as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                );
                                if idxjp <= nlp1 {
                                    idxjp -= 1;
                                }
                                if idxj <= nlp1 {
                                    idxj -= 1;
                                }
                                *givcol.offset(
                                    (*givptr + (givcol_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                ) = idxjp;
                                *givcol.offset((*givptr + givcol_dim1) as isize) = idxj;
                                *givnum.offset(
                                    (*givptr + (givnum_dim1 << 1 as ::core::ffi::c_int)) as isize,
                                ) = *c__;
                                *givnum.offset((*givptr + givnum_dim1) as isize) = *s;
                            }
                            f2c_drot_0(
                                &raw mut c__1,
                                vf.offset(jprev as isize) as *mut doublereal,
                                &raw mut c__1,
                                vf.offset(j as isize) as *mut doublereal,
                                &raw mut c__1,
                                c__,
                                s,
                            );
                            f2c_drot_0(
                                &raw mut c__1,
                                vl.offset(jprev as isize) as *mut doublereal,
                                &raw mut c__1,
                                vl.offset(j as isize) as *mut doublereal,
                                &raw mut c__1,
                                c__,
                                s,
                            );
                            k2 -= 1;
                            *idxp.offset(k2 as isize) = jprev;
                            jprev = j;
                        } else {
                            *k += 1;
                            *zw.offset(*k as isize) = *z__.offset(jprev as isize);
                            *dsigma.offset(*k as isize) = *d__.offset(jprev as isize);
                            *idxp.offset(*k as isize) = jprev;
                            jprev = j;
                        }
                    }
                }
                *k += 1;
                *zw.offset(*k as isize) = *z__.offset(jprev as isize);
                *dsigma.offset(*k as isize) = *d__.offset(jprev as isize);
                *idxp.offset(*k as isize) = jprev;
            }
            _ => {}
        }
        i__1 = n;
        j = 2 as integer;
        while j <= i__1 {
            jp = *idxp.offset(j as isize);
            *dsigma.offset(j as isize) = *d__.offset(jp as isize);
            *vfw.offset(j as isize) = *vf.offset(jp as isize);
            *vlw.offset(j as isize) = *vl.offset(jp as isize);
            j += 1;
        }
        if *icompq == 1 as ::core::ffi::c_long {
            i__1 = n;
            j = 2 as integer;
            while j <= i__1 {
                jp = *idxp.offset(j as isize);
                *perm.offset(j as isize) = *idxq.offset(
                    (*idx.offset(jp as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                        as isize,
                );
                if *perm.offset(j as isize) <= nlp1 {
                    let ref mut fresh1 = *perm.offset(j as isize);
                    *fresh1 -= 1;
                }
                j += 1;
            }
        }
        i__1 = n - *k;
        f2c_dcopy_0(
            &raw mut i__1,
            dsigma.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut c__1,
            d__.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        *dsigma.offset(1 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
        hlftol = (tol as ::core::ffi::c_double / 2.0f64) as doublereal;
        if (if *dsigma.offset(2 as ::core::ffi::c_int as isize)
            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *dsigma.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
        } else {
            -(*dsigma.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
        }) <= hlftol
        {
            *dsigma.offset(2 as ::core::ffi::c_int as isize) = hlftol;
        }
        if m > n {
            *z__.offset(1 as ::core::ffi::c_int as isize) =
                dlapy2__0(&raw mut z1, z__.offset(m as isize) as *mut doublereal);
            if *z__.offset(1 as ::core::ffi::c_int as isize) <= tol {
                *c__ = 1.0f64 as doublereal;
                *s = 0.0f64 as doublereal;
                *z__.offset(1 as ::core::ffi::c_int as isize) = tol;
            } else {
                *c__ = z1 / *z__.offset(1 as ::core::ffi::c_int as isize);
                *s = -*z__.offset(m as isize) / *z__.offset(1 as ::core::ffi::c_int as isize);
            }
            f2c_drot_0(
                &raw mut c__1,
                vf.offset(m as isize) as *mut doublereal,
                &raw mut c__1,
                vf.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                c__,
                s,
            );
            f2c_drot_0(
                &raw mut c__1,
                vl.offset(m as isize) as *mut doublereal,
                &raw mut c__1,
                vl.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                c__,
                s,
            );
        } else if (if z1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            z1 as ::core::ffi::c_double
        } else {
            -(z1 as ::core::ffi::c_double)
        }) <= tol
        {
            *z__.offset(1 as ::core::ffi::c_int as isize) = tol;
        } else {
            *z__.offset(1 as ::core::ffi::c_int as isize) = z1;
        }
        i__1 = (*k - 1 as ::core::ffi::c_long) as integer;
        f2c_dcopy_0(
            &raw mut i__1,
            zw.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            z__.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__1 = (n as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        f2c_dcopy_0(
            &raw mut i__1,
            vfw.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            vf.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__1 = (n as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        f2c_dcopy_0(
            &raw mut i__1,
            vlw.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            vl.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasd7::dgelsd_closure_dlasd7_;

pub mod raw_dlasd8 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c__1: integer = 1 as integer;
    static mut c__0: integer = 0 as integer;
    static mut c_b8: doublereal = 1.0f64;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasd8_(
        mut icompq: *mut integer,
        mut k: *mut integer,
        mut d__: *mut doublereal,
        mut z__: *mut doublereal,
        mut vf: *mut doublereal,
        mut vl: *mut doublereal,
        mut difl: *mut doublereal,
        mut difr: *mut doublereal,
        mut lddifr: *mut integer,
        mut dsigma: *mut doublereal,
        mut work: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut difr_dim1: integer = 0;
        let mut difr_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_d_sign"]
            fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
        }
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut dj: doublereal = 0.;
        let mut rho: doublereal = 0.;
        let mut iwk1: integer = 0;
        let mut iwk2: integer = 0;
        let mut iwk3: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_ddot"]
            fn f2c_ddot_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> doublereal;
        }
        let mut temp: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dnrm2"]
            fn f2c_dnrm2_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> doublereal;
        }
        let mut iwk2i: integer = 0;
        let mut iwk3i: integer = 0;
        let mut diflj: doublereal = 0.;
        let mut difrj: doublereal = 0.;
        let mut dsigj: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamc3_"]
            fn dlamc3__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasd4_"]
            fn dlasd4__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlascl_"]
            fn dlascl__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlaset_"]
            fn dlaset__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut dsigjp: doublereal = 0.;
        d__ = d__.wrapping_offset(-1);
        z__ = z__.wrapping_offset(-1);
        vf = vf.wrapping_offset(-1);
        vl = vl.wrapping_offset(-1);
        difl = difl.wrapping_offset(-1);
        difr_dim1 = *lddifr;
        difr_offset = 1 as integer + difr_dim1;
        difr = difr.wrapping_offset(-(difr_offset as isize));
        dsigma = dsigma.wrapping_offset(-1);
        work = work.wrapping_offset(-1);
        *info = 0 as integer;
        if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *k < 1 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *lddifr < *k {
            *info = -(9 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLASD8\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        if *k == 1 as ::core::ffi::c_long {
            *d__.offset(1 as ::core::ffi::c_int as isize) = (if *z__
                .offset(1 as ::core::ffi::c_int as isize)
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                *z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            } else {
                -(*z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
            }) as doublereal;
            *difl.offset(1 as ::core::ffi::c_int as isize) =
                *d__.offset(1 as ::core::ffi::c_int as isize);
            if *icompq == 1 as ::core::ffi::c_long {
                *difl.offset(2 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
                *difr.offset(
                    (((difr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) = 1.0f64 as doublereal;
            }
            return 0 as ::core::ffi::c_int;
        }
        i__1 = *k;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *dsigma.offset(i__ as isize) = dlamc3__0(
                dsigma.offset(i__ as isize) as *mut doublereal,
                dsigma.offset(i__ as isize) as *mut doublereal,
            ) - *dsigma.offset(i__ as isize);
            i__ += 1;
        }
        iwk1 = 1 as integer;
        iwk2 = iwk1 + *k;
        iwk3 = iwk2 + *k;
        iwk2i = (iwk2 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        iwk3i = (iwk3 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        rho = f2c_dnrm2_0(
            k,
            z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut rho,
            &raw mut c_b8,
            k,
            &raw mut c__1,
            z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            k,
            info,
        );
        rho *= rho as ::core::ffi::c_double;
        dlaset__0(
            b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            k,
            &raw mut c__1,
            &raw mut c_b8,
            &raw mut c_b8,
            work.offset(iwk3 as isize) as *mut doublereal,
            k,
        );
        i__1 = *k;
        j = 1 as integer;
        while j <= i__1 {
            dlasd4__0(
                k,
                &raw mut j,
                dsigma.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(iwk1 as isize) as *mut doublereal,
                &raw mut rho,
                d__.offset(j as isize) as *mut doublereal,
                work.offset(iwk2 as isize) as *mut doublereal,
                info,
            );
            if *info != 0 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            *work.offset((iwk3i + j) as isize) = *work.offset((iwk3i + j) as isize)
                * *work.offset(j as isize)
                * *work.offset((iwk2i + j) as isize);
            *difl.offset(j as isize) = -*work.offset(j as isize);
            *difr.offset((j + difr_dim1) as isize) =
                -*work.offset((j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__2 {
                *work.offset((iwk3i + i__) as isize) = *work.offset((iwk3i + i__) as isize)
                    * *work.offset(i__ as isize)
                    * *work.offset((iwk2i + i__) as isize)
                    / (*dsigma.offset(i__ as isize) - *dsigma.offset(j as isize))
                    / (*dsigma.offset(i__ as isize) + *dsigma.offset(j as isize));
                i__ += 1;
            }
            i__2 = *k;
            i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while i__ <= i__2 {
                *work.offset((iwk3i + i__) as isize) = *work.offset((iwk3i + i__) as isize)
                    * *work.offset(i__ as isize)
                    * *work.offset((iwk2i + i__) as isize)
                    / (*dsigma.offset(i__ as isize) - *dsigma.offset(j as isize))
                    / (*dsigma.offset(i__ as isize) + *dsigma.offset(j as isize));
                i__ += 1;
            }
            j += 1;
        }
        i__1 = *k;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *work.offset((iwk3i + i__) as isize);
            d__2 = super::lapack_sqrt(
                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1
                } else {
                    -d__1
                }),
            ) as doublereal;
            *z__.offset(i__ as isize) =
                d_sign_0(&raw mut d__2, z__.offset(i__ as isize) as *mut doublereal) as doublereal;
            i__ += 1;
        }
        i__1 = *k;
        j = 1 as integer;
        while j <= i__1 {
            diflj = *difl.offset(j as isize);
            dj = *d__.offset(j as isize);
            dsigj = -*dsigma.offset(j as isize);
            if j < *k {
                difrj = -*difr.offset((j + difr_dim1) as isize);
                dsigjp =
                    -*dsigma.offset((j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            }
            *work.offset(j as isize) =
                -*z__.offset(j as isize) / diflj / (*dsigma.offset(j as isize) + dj);
            i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__2 {
                *work.offset(i__ as isize) = *z__.offset(i__ as isize)
                    / (dlamc3__0(
                        dsigma.offset(i__ as isize) as *mut doublereal,
                        &raw mut dsigj,
                    ) - diflj)
                    / (*dsigma.offset(i__ as isize) + dj);
                i__ += 1;
            }
            i__2 = *k;
            i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while i__ <= i__2 {
                *work.offset(i__ as isize) = *z__.offset(i__ as isize)
                    / (dlamc3__0(
                        dsigma.offset(i__ as isize) as *mut doublereal,
                        &raw mut dsigjp,
                    ) + difrj)
                    / (*dsigma.offset(i__ as isize) + dj);
                i__ += 1;
            }
            temp = f2c_dnrm2_0(
                k,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            *work.offset((iwk2i + j) as isize) = f2c_ddot_0(
                k,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                vf.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            ) / temp;
            *work.offset((iwk3i + j) as isize) = f2c_ddot_0(
                k,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                vl.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            ) / temp;
            if *icompq == 1 as ::core::ffi::c_long {
                *difr.offset((j + (difr_dim1 << 1 as ::core::ffi::c_int)) as isize) = temp;
            }
            j += 1;
        }
        f2c_dcopy_0(
            k,
            work.offset(iwk2 as isize) as *mut doublereal,
            &raw mut c__1,
            vf.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        f2c_dcopy_0(
            k,
            work.offset(iwk3 as isize) as *mut doublereal,
            &raw mut c__1,
            vl.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasd8::dgelsd_closure_dlasd8_;

pub mod raw_dlaed6 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
        fn log(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    pub const TRUE_: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const FALSE_: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlaed6_(
        mut kniter: *mut integer,
        mut orgati: *mut logical,
        mut rho: *mut doublereal,
        mut d__: *mut doublereal,
        mut z__: *mut doublereal,
        mut finit: *mut doublereal,
        mut tau: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut current_block: u64;
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut d__3: doublereal = 0.;
        let mut d__4: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_pow_di"]
            fn pow_di_0(_: *mut doublereal, _: *mut integer) -> ::core::ffi::c_double;
        }
        let mut a: doublereal = 0.;
        let mut b: doublereal = 0.;
        let mut c__: doublereal = 0.;
        let mut f: doublereal = 0.;
        let mut i__: integer = 0;
        let mut fc: doublereal = 0.;
        let mut df: doublereal = 0.;
        let mut ddf: doublereal = 0.;
        let mut lbd: doublereal = 0.;
        let mut eta: doublereal = 0.;
        let mut ubd: doublereal = 0.;
        let mut eps: doublereal = 0.;
        let mut base: doublereal = 0.;
        let mut iter: integer = 0;
        let mut temp: doublereal = 0.;
        let mut temp1: doublereal = 0.;
        let mut temp2: doublereal = 0.;
        let mut temp3: doublereal = 0.;
        let mut temp4: doublereal = 0.;
        let mut scale: logical = 0;
        let mut niter: integer = 0;
        let mut small1: doublereal = 0.;
        let mut small2: doublereal = 0.;
        let mut sminv1: doublereal = 0.;
        let mut sminv2: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        let mut dscale: [doublereal; 3] = [0.; 3];
        let mut sclfac: doublereal = 0.;
        let mut zscale: [doublereal; 3] = [0.; 3];
        let mut erretm: doublereal = 0.;
        let mut sclinv: doublereal = 0.;
        z__ = z__.wrapping_offset(-1);
        d__ = d__.wrapping_offset(-1);
        *info = 0 as integer;
        if *orgati != 0 {
            lbd = *d__.offset(2 as ::core::ffi::c_int as isize);
            ubd = *d__.offset(3 as ::core::ffi::c_int as isize);
        } else {
            lbd = *d__.offset(1 as ::core::ffi::c_int as isize);
            ubd = *d__.offset(2 as ::core::ffi::c_int as isize);
        }
        if *finit < 0.0f64 {
            lbd = 0.0f64 as doublereal;
        } else {
            ubd = 0.0f64 as doublereal;
        }
        niter = 1 as integer;
        *tau = 0.0f64 as doublereal;
        if *kniter == 2 as ::core::ffi::c_long {
            if *orgati != 0 {
                temp = ((*d__.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    - *d__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                    / 2.0f64) as doublereal;
                c__ = *rho
                    + *z__.offset(1 as ::core::ffi::c_int as isize)
                        / (*d__.offset(1 as ::core::ffi::c_int as isize)
                            - *d__.offset(2 as ::core::ffi::c_int as isize)
                            - temp);
                a = c__
                    * (*d__.offset(2 as ::core::ffi::c_int as isize)
                        + *d__.offset(3 as ::core::ffi::c_int as isize))
                    + *z__.offset(2 as ::core::ffi::c_int as isize)
                    + *z__.offset(3 as ::core::ffi::c_int as isize);
                b = c__
                    * *d__.offset(2 as ::core::ffi::c_int as isize)
                    * *d__.offset(3 as ::core::ffi::c_int as isize)
                    + *z__.offset(2 as ::core::ffi::c_int as isize)
                        * *d__.offset(3 as ::core::ffi::c_int as isize)
                    + *z__.offset(3 as ::core::ffi::c_int as isize)
                        * *d__.offset(2 as ::core::ffi::c_int as isize);
            } else {
                temp = ((*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    - *d__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                    / 2.0f64) as doublereal;
                c__ = *rho
                    + *z__.offset(3 as ::core::ffi::c_int as isize)
                        / (*d__.offset(3 as ::core::ffi::c_int as isize)
                            - *d__.offset(2 as ::core::ffi::c_int as isize)
                            - temp);
                a = c__
                    * (*d__.offset(1 as ::core::ffi::c_int as isize)
                        + *d__.offset(2 as ::core::ffi::c_int as isize))
                    + *z__.offset(1 as ::core::ffi::c_int as isize)
                    + *z__.offset(2 as ::core::ffi::c_int as isize);
                b = c__
                    * *d__.offset(1 as ::core::ffi::c_int as isize)
                    * *d__.offset(2 as ::core::ffi::c_int as isize)
                    + *z__.offset(1 as ::core::ffi::c_int as isize)
                        * *d__.offset(2 as ::core::ffi::c_int as isize)
                    + *z__.offset(2 as ::core::ffi::c_int as isize)
                        * *d__.offset(1 as ::core::ffi::c_int as isize);
            }
            d__1 = (if a >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                a as ::core::ffi::c_double
            } else {
                -(a as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = (if b >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                b as ::core::ffi::c_double
            } else {
                -(b as ::core::ffi::c_double)
            }) as doublereal;
            d__1 = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            d__2 = (if c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                c__ as ::core::ffi::c_double
            } else {
                -(c__ as ::core::ffi::c_double)
            }) as doublereal;
            temp = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            a /= temp as ::core::ffi::c_double;
            b /= temp as ::core::ffi::c_double;
            c__ /= temp as ::core::ffi::c_double;
            if c__ == 0.0f64 {
                *tau = b / a;
            } else if a <= 0.0f64 {
                d__1 = a * a - b * 4.0f64 * c__;
                *tau = ((a as ::core::ffi::c_double
                    - super::lapack_sqrt(
                        (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1
                        } else {
                            -d__1
                        }),
                    ))
                    / (c__ as ::core::ffi::c_double * 2.0f64)) as doublereal;
            } else {
                d__1 = a * a - b * 4.0f64 * c__;
                *tau = (b as ::core::ffi::c_double * 2.0f64
                    / (a as ::core::ffi::c_double
                        + super::lapack_sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))) as doublereal;
            }
            if *tau < lbd || *tau > ubd {
                *tau = ((lbd as ::core::ffi::c_double + ubd as ::core::ffi::c_double) / 2.0f64)
                    as doublereal;
            }
            if *d__.offset(1 as ::core::ffi::c_int as isize) == *tau
                || *d__.offset(2 as ::core::ffi::c_int as isize) == *tau
                || *d__.offset(3 as ::core::ffi::c_int as isize) == *tau
            {
                *tau = 0.0f64 as doublereal;
            } else {
                temp = *finit
                    + *tau * *z__.offset(1 as ::core::ffi::c_int as isize)
                        / (*d__.offset(1 as ::core::ffi::c_int as isize)
                            * (*d__.offset(1 as ::core::ffi::c_int as isize) - *tau))
                    + *tau * *z__.offset(2 as ::core::ffi::c_int as isize)
                        / (*d__.offset(2 as ::core::ffi::c_int as isize)
                            * (*d__.offset(2 as ::core::ffi::c_int as isize) - *tau))
                    + *tau * *z__.offset(3 as ::core::ffi::c_int as isize)
                        / (*d__.offset(3 as ::core::ffi::c_int as isize)
                            * (*d__.offset(3 as ::core::ffi::c_int as isize) - *tau));
                if temp <= 0.0f64 {
                    lbd = *tau;
                } else {
                    ubd = *tau;
                }
                if (if *finit >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    *finit
                } else {
                    -*finit
                }) <= (if temp >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    temp as ::core::ffi::c_double
                } else {
                    -(temp as ::core::ffi::c_double)
                }) {
                    *tau = 0.0f64 as doublereal;
                }
            }
        }
        eps = dlamch__0(
            b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        base = dlamch__0(
            b"Base\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        i__1 = (super::lapack_log(dlamch__0(
            b"SafMin\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        )) / super::lapack_log(base)
            / 3.0f64) as integer;
        small1 = pow_di_0(&raw mut base, &raw mut i__1) as doublereal;
        sminv1 = 1.0f64 / small1;
        small2 = small1 * small1;
        sminv2 = sminv1 * sminv1;
        if *orgati != 0 {
            d__1 = *d__.offset(2 as ::core::ffi::c_int as isize) - *tau;
            d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = *d__.offset(3 as ::core::ffi::c_int as isize) - *tau;
            d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) as doublereal;
            temp = (if d__3 <= d__4 {
                d__3 as ::core::ffi::c_double
            } else {
                d__4 as ::core::ffi::c_double
            }) as doublereal;
        } else {
            d__1 = *d__.offset(1 as ::core::ffi::c_int as isize) - *tau;
            d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = *d__.offset(2 as ::core::ffi::c_int as isize) - *tau;
            d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) as doublereal;
            temp = (if d__3 <= d__4 {
                d__3 as ::core::ffi::c_double
            } else {
                d__4 as ::core::ffi::c_double
            }) as doublereal;
        }
        scale = FALSE_ as logical;
        if temp <= small1 {
            scale = TRUE_ as logical;
            if temp <= small2 {
                sclfac = sminv2;
                sclinv = small2;
            } else {
                sclfac = sminv1;
                sclinv = small1;
            }
            i__ = 1 as integer;
            while i__ <= 3 as ::core::ffi::c_long {
                dscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] =
                    *d__.offset(i__ as isize) * sclfac;
                zscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] =
                    *z__.offset(i__ as isize) * sclfac;
                i__ += 1;
            }
            *tau *= sclfac as ::core::ffi::c_double;
            lbd *= sclfac as ::core::ffi::c_double;
            ubd *= sclfac as ::core::ffi::c_double;
        } else {
            i__ = 1 as integer;
            while i__ <= 3 as ::core::ffi::c_long {
                dscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] =
                    *d__.offset(i__ as isize);
                zscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] =
                    *z__.offset(i__ as isize);
                i__ += 1;
            }
        }
        fc = 0.0f64 as doublereal;
        df = 0.0f64 as doublereal;
        ddf = 0.0f64 as doublereal;
        i__ = 1 as integer;
        while i__ <= 3 as ::core::ffi::c_long {
            temp = 1.0f64
                / (dscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] - *tau);
            temp1 = zscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] * temp;
            temp2 = temp1 * temp;
            temp3 = temp2 * temp;
            fc += (temp1 / dscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize])
                as ::core::ffi::c_double;
            df += temp2 as ::core::ffi::c_double;
            ddf += temp3 as ::core::ffi::c_double;
            i__ += 1;
        }
        f = *finit + *tau * fc;
        if !((if f >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            f as ::core::ffi::c_double
        } else {
            -(f as ::core::ffi::c_double)
        }) <= 0.0f64)
        {
            if f <= 0.0f64 {
                lbd = *tau;
            } else {
                ubd = *tau;
            }
            iter = (niter as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            niter = iter;
            loop {
                if !(niter <= 40 as ::core::ffi::c_long) {
                    current_block = 3024573345131975588;
                    break;
                }
                if *orgati != 0 {
                    temp1 = dscale[1 as ::core::ffi::c_int as usize] - *tau;
                    temp2 = dscale[2 as ::core::ffi::c_int as usize] - *tau;
                } else {
                    temp1 = dscale[0 as ::core::ffi::c_int as usize] - *tau;
                    temp2 = dscale[1 as ::core::ffi::c_int as usize] - *tau;
                }
                a = (temp1 + temp2) * f - temp1 * temp2 * df;
                b = temp1 * temp2 * f;
                c__ = f - (temp1 + temp2) * df + temp1 * temp2 * ddf;
                d__1 = (if a >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    a as ::core::ffi::c_double
                } else {
                    -(a as ::core::ffi::c_double)
                }) as doublereal;
                d__2 = (if b >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    b as ::core::ffi::c_double
                } else {
                    -(b as ::core::ffi::c_double)
                }) as doublereal;
                d__1 = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = (if c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    c__ as ::core::ffi::c_double
                } else {
                    -(c__ as ::core::ffi::c_double)
                }) as doublereal;
                temp = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                a /= temp as ::core::ffi::c_double;
                b /= temp as ::core::ffi::c_double;
                c__ /= temp as ::core::ffi::c_double;
                if c__ == 0.0f64 {
                    eta = b / a;
                } else if a <= 0.0f64 {
                    d__1 = a * a - b * 4.0f64 * c__;
                    eta = ((a as ::core::ffi::c_double
                        - super::lapack_sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))
                        / (c__ as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                } else {
                    d__1 = a * a - b * 4.0f64 * c__;
                    eta = (b as ::core::ffi::c_double * 2.0f64
                        / (a as ::core::ffi::c_double
                            + super::lapack_sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))) as doublereal;
                }
                if f * eta >= 0.0f64 {
                    eta = -f / df;
                }
                *tau += eta as ::core::ffi::c_double;
                if *tau < lbd || *tau > ubd {
                    *tau = ((lbd as ::core::ffi::c_double + ubd as ::core::ffi::c_double) / 2.0f64)
                        as doublereal;
                }
                fc = 0.0f64 as doublereal;
                erretm = 0.0f64 as doublereal;
                df = 0.0f64 as doublereal;
                ddf = 0.0f64 as doublereal;
                i__ = 1 as integer;
                while i__ <= 3 as ::core::ffi::c_long {
                    temp = 1.0f64
                        / (dscale
                            [(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize]
                            - *tau);
                    temp1 = zscale
                        [(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize]
                        * temp;
                    temp2 = temp1 * temp;
                    temp3 = temp2 * temp;
                    temp4 = temp1
                        / dscale[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                    fc += temp4 as ::core::ffi::c_double;
                    erretm += if temp4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        temp4 as ::core::ffi::c_double
                    } else {
                        -(temp4 as ::core::ffi::c_double)
                    };
                    df += temp2 as ::core::ffi::c_double;
                    ddf += temp3 as ::core::ffi::c_double;
                    i__ += 1;
                }
                f = *finit + *tau * fc;
                erretm = ((if *finit >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    *finit
                } else {
                    -*finit
                }) + (if *tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    *tau
                } else {
                    -*tau
                }) * erretm)
                    * 8.0f64
                    + (if *tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        *tau
                    } else {
                        -*tau
                    }) * df;
                if (if f >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    f as ::core::ffi::c_double
                } else {
                    -(f as ::core::ffi::c_double)
                }) <= eps * erretm
                {
                    current_block = 16339904477235788145;
                    break;
                }
                if f <= 0.0f64 {
                    lbd = *tau;
                } else {
                    ubd = *tau;
                }
                niter += 1;
            }
            match current_block {
                16339904477235788145 => {}
                _ => {
                    *info = 1 as integer;
                }
            }
        }
        if scale != 0 {
            *tau *= sclinv as ::core::ffi::c_double;
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlaed6::dgelsd_closure_dlaed6_;

pub mod raw_dlamrg {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlamrg_(
        mut n1: *mut integer,
        mut n2: *mut integer,
        mut a: *mut doublereal,
        mut dtrd1: *mut integer,
        mut dtrd2: *mut integer,
        mut index: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut i__: integer = 0;
        let mut ind1: integer = 0;
        let mut ind2: integer = 0;
        let mut n1sv: integer = 0;
        let mut n2sv: integer = 0;
        index = index.wrapping_offset(-1);
        a = a.wrapping_offset(-1);
        n1sv = *n1;
        n2sv = *n2;
        if *dtrd1 > 0 as ::core::ffi::c_long {
            ind1 = 1 as integer;
        } else {
            ind1 = *n1;
        }
        if *dtrd2 > 0 as ::core::ffi::c_long {
            ind2 = (*n1 + 1 as ::core::ffi::c_long) as integer;
        } else {
            ind2 = *n1 + *n2;
        }
        i__ = 1 as integer;
        while n1sv > 0 as ::core::ffi::c_long && n2sv > 0 as ::core::ffi::c_long {
            if *a.offset(ind1 as isize) <= *a.offset(ind2 as isize) {
                *index.offset(i__ as isize) = ind1;
                i__ += 1;
                ind1 += *dtrd1 as ::core::ffi::c_long;
                n1sv -= 1;
            } else {
                *index.offset(i__ as isize) = ind2;
                i__ += 1;
                ind2 += *dtrd2 as ::core::ffi::c_long;
                n2sv -= 1;
            }
        }
        if n1sv == 0 as ::core::ffi::c_long {
            i__1 = n2sv;
            n1sv = 1 as integer;
            while n1sv <= i__1 {
                *index.offset(i__ as isize) = ind2;
                i__ += 1;
                ind2 += *dtrd2 as ::core::ffi::c_long;
                n1sv += 1;
            }
        } else {
            i__1 = n1sv;
            n2sv = 1 as integer;
            while n2sv <= i__1 {
                *index.offset(i__ as isize) = ind1;
                i__ += 1;
                ind1 += *dtrd1 as ::core::ffi::c_long;
                n2sv += 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlamrg::dgelsd_closure_dlamrg_;

pub mod raw_dlanst {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    static mut c__1: integer = 1 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlanst_(
        mut norm: *mut ::core::ffi::c_char,
        mut n: *mut integer,
        mut d__: *mut doublereal,
        mut e: *mut doublereal,
    ) -> doublereal {
        let mut i__1: integer = 0;
        let mut ret_val: doublereal = 0.;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut d__3: doublereal = 0.;
        let mut d__4: doublereal = 0.;
        let mut d__5: doublereal = 0.;
        let mut i__: integer = 0;
        let mut sum: doublereal = 0.;
        let mut scale: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_lsame_"]
            fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
        }
        let mut anorm: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlassq_"]
            fn dlassq__0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        e = e.wrapping_offset(-1);
        d__ = d__.wrapping_offset(-1);
        if *n <= 0 as ::core::ffi::c_long {
            anorm = 0.0f64 as doublereal;
        } else if lsame__0(
            norm,
            b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            d__1 = *d__.offset(*n as isize);
            anorm = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__1 {
                d__2 = anorm;
                d__1 = *d__.offset(i__ as isize);
                d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                anorm = (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = anorm;
                d__1 = *e.offset(i__ as isize);
                d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                anorm = (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) as doublereal;
                i__ += 1;
            }
        } else if lsame__0(
            norm,
            b"O\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
            || *(norm as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '1' as i32
            || lsame__0(
                norm,
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
        {
            if *n == 1 as ::core::ffi::c_long {
                anorm = (if *d__.offset(1 as ::core::ffi::c_int as isize)
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                } else {
                    -(*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                }) as doublereal;
            } else {
                d__3 = ((if *d__.offset(1 as ::core::ffi::c_int as isize)
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                } else {
                    -(*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                }) + (if *e.offset(1 as ::core::ffi::c_int as isize)
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *e.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                } else {
                    -(*e.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                })) as doublereal;
                d__1 = *e.offset((*n - 1 as ::core::ffi::c_long) as isize);
                d__2 = *d__.offset(*n as isize);
                d__4 = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__2 as ::core::ffi::c_double
                } else {
                    -(d__2 as ::core::ffi::c_double)
                })) as doublereal;
                anorm = (if d__3 >= d__4 {
                    d__3 as ::core::ffi::c_double
                } else {
                    d__4 as ::core::ffi::c_double
                }) as doublereal;
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__ = 2 as integer;
                while i__ <= i__1 {
                    d__4 = anorm;
                    d__1 = *d__.offset(i__ as isize);
                    d__2 = *e.offset(i__ as isize);
                    d__3 =
                        *e.offset((i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                    d__5 = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__2 as ::core::ffi::c_double
                    } else {
                        -(d__2 as ::core::ffi::c_double)
                    }) + (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__3 as ::core::ffi::c_double
                    } else {
                        -(d__3 as ::core::ffi::c_double)
                    })) as doublereal;
                    anorm = (if d__4 >= d__5 {
                        d__4 as ::core::ffi::c_double
                    } else {
                        d__5 as ::core::ffi::c_double
                    }) as doublereal;
                    i__ += 1;
                }
            }
        } else if lsame__0(
            norm,
            b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
            || lsame__0(
                norm,
                b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
        {
            scale = 0.0f64 as doublereal;
            sum = 1.0f64 as doublereal;
            if *n > 1 as ::core::ffi::c_long {
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                dlassq__0(
                    &raw mut i__1,
                    e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut scale,
                    &raw mut sum,
                );
                sum *= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            dlassq__0(
                n,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut scale,
                &raw mut sum,
            );
            anorm = (scale as ::core::ffi::c_double * super::lapack_sqrt(sum)) as doublereal;
        }
        ret_val = anorm;
        return ret_val;
    }
}
pub use raw_dlanst::dgelsd_closure_dlanst_;

pub mod raw_dlas2 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type doublereal = ::core::ffi::c_double;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlas2_(
        mut f: *mut doublereal,
        mut g: *mut doublereal,
        mut h__: *mut doublereal,
        mut ssmin: *mut doublereal,
        mut ssmax: *mut doublereal,
    ) -> ::core::ffi::c_int {
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut c__: doublereal = 0.;
        let mut fa: doublereal = 0.;
        let mut ga: doublereal = 0.;
        let mut ha: doublereal = 0.;
        let mut as_0: doublereal = 0.;
        let mut at: doublereal = 0.;
        let mut au: doublereal = 0.;
        let mut fhmn: doublereal = 0.;
        let mut fhmx: doublereal = 0.;
        fa = (if *f >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *f
        } else {
            -*f
        }) as doublereal;
        ga = (if *g >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *g
        } else {
            -*g
        }) as doublereal;
        ha = (if *h__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *h__
        } else {
            -*h__
        }) as doublereal;
        fhmn = (if fa <= ha {
            fa as ::core::ffi::c_double
        } else {
            ha as ::core::ffi::c_double
        }) as doublereal;
        fhmx = (if fa >= ha {
            fa as ::core::ffi::c_double
        } else {
            ha as ::core::ffi::c_double
        }) as doublereal;
        if fhmn == 0.0f64 {
            *ssmin = 0.0f64 as doublereal;
            if fhmx == 0.0f64 {
                *ssmax = ga;
            } else {
                d__1 = ((if fhmx <= ga {
                    fhmx as ::core::ffi::c_double
                } else {
                    ga as ::core::ffi::c_double
                }) / (if fhmx >= ga {
                    fhmx as ::core::ffi::c_double
                } else {
                    ga as ::core::ffi::c_double
                })) as doublereal;
                *ssmax = ((if fhmx >= ga {
                    fhmx as ::core::ffi::c_double
                } else {
                    ga as ::core::ffi::c_double
                }) * super::lapack_sqrt(d__1 * d__1 + 1.0f64))
                    as doublereal;
            }
        } else if ga < fhmx {
            as_0 = (fhmn as ::core::ffi::c_double / fhmx as ::core::ffi::c_double + 1.0f64)
                as doublereal;
            at = (fhmx - fhmn) / fhmx;
            d__1 = ga / fhmx;
            au = d__1 * d__1;
            c__ = (2.0f64
                / (super::lapack_sqrt(as_0 * as_0 + au) + super::lapack_sqrt(at * at + au)))
                as doublereal;
            *ssmin = fhmn * c__;
            *ssmax = fhmx / c__;
        } else {
            au = fhmx / ga;
            if au == 0.0f64 {
                *ssmin = fhmn * fhmx / ga;
                *ssmax = ga;
            } else {
                as_0 = (fhmn as ::core::ffi::c_double / fhmx as ::core::ffi::c_double + 1.0f64)
                    as doublereal;
                at = (fhmx - fhmn) / fhmx;
                d__1 = as_0 * au;
                d__2 = at * au;
                c__ = (1.0f64
                    / (super::lapack_sqrt(d__1 * d__1 + 1.0f64)
                        + super::lapack_sqrt(d__2 * d__2 + 1.0f64)))
                    as doublereal;
                *ssmin = fhmn * c__ * au;
                *ssmin += *ssmin as ::core::ffi::c_double;
                *ssmax = ga / (c__ + c__);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlas2::dgelsd_closure_dlas2_;

pub mod raw_dlascl {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    pub const TRUE_: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const FALSE_: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlascl_(
        mut type__: *mut ::core::ffi::c_char,
        mut kl: *mut integer,
        mut ku: *mut integer,
        mut cfrom: *mut doublereal,
        mut cto: *mut doublereal,
        mut m: *mut integer,
        mut n: *mut integer,
        mut a: *mut doublereal,
        mut lda: *mut integer,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut a_dim1: integer = 0;
        let mut a_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut i__3: integer = 0;
        let mut i__4: integer = 0;
        let mut i__5: integer = 0;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut k1: integer = 0;
        let mut k2: integer = 0;
        let mut k3: integer = 0;
        let mut k4: integer = 0;
        let mut mul: doublereal = 0.;
        let mut cto1: doublereal = 0.;
        let mut done: logical = 0;
        let mut ctoc: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_lsame_"]
            fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
        }
        let mut itype: integer = 0;
        let mut cfrom1: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        let mut cfromc: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_disnan_"]
            fn disnan__0(_: *mut doublereal) -> logical;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut bignum: doublereal = 0.;
        let mut smlnum: doublereal = 0.;
        a_dim1 = *lda;
        a_offset = 1 as integer + a_dim1;
        a = a.wrapping_offset(-(a_offset as isize));
        *info = 0 as integer;
        if lsame__0(
            type__,
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            itype = 0 as integer;
        } else if lsame__0(
            type__,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            itype = 1 as integer;
        } else if lsame__0(
            type__,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            itype = 2 as integer;
        } else if lsame__0(
            type__,
            b"H\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            itype = 3 as integer;
        } else if lsame__0(
            type__,
            b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            itype = 4 as integer;
        } else if lsame__0(
            type__,
            b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            itype = 5 as integer;
        } else if lsame__0(
            type__,
            b"Z\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            itype = 6 as integer;
        } else {
            itype = -(1 as ::core::ffi::c_int) as integer;
        }
        if itype == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *cfrom == 0.0f64 || disnan__0(cfrom) != 0 {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if disnan__0(cto) != 0 {
            *info = -(5 as ::core::ffi::c_int) as integer;
        } else if *m < 0 as ::core::ffi::c_long {
            *info = -(6 as ::core::ffi::c_int) as integer;
        } else if *n < 0 as ::core::ffi::c_long
            || itype == 4 as ::core::ffi::c_long && *n != *m
            || itype == 5 as ::core::ffi::c_long && *n != *m
        {
            *info = -(7 as ::core::ffi::c_int) as integer;
        } else if itype <= 3 as ::core::ffi::c_long
            && *lda
                < (if 1 as ::core::ffi::c_long >= *m {
                    1 as ::core::ffi::c_long
                } else {
                    *m
                })
        {
            *info = -(9 as ::core::ffi::c_int) as integer;
        } else if itype >= 4 as ::core::ffi::c_long {
            i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
            if *kl < 0 as ::core::ffi::c_long
                || *kl
                    > (if i__1 >= 0 as ::core::ffi::c_long {
                        i__1 as ::core::ffi::c_long
                    } else {
                        0 as ::core::ffi::c_long
                    })
            {
                *info = -(2 as ::core::ffi::c_int) as integer;
            } else {
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                if *ku < 0 as ::core::ffi::c_long
                    || *ku
                        > (if i__1 >= 0 as ::core::ffi::c_long {
                            i__1 as ::core::ffi::c_long
                        } else {
                            0 as ::core::ffi::c_long
                        })
                    || (itype == 4 as ::core::ffi::c_long || itype == 5 as ::core::ffi::c_long)
                        && *kl != *ku
                {
                    *info = -(3 as ::core::ffi::c_int) as integer;
                } else if itype == 4 as ::core::ffi::c_long && *lda < *kl + 1 as ::core::ffi::c_long
                    || itype == 5 as ::core::ffi::c_long && *lda < *ku + 1 as ::core::ffi::c_long
                    || itype == 6 as ::core::ffi::c_long
                        && *lda < (*kl << 1 as ::core::ffi::c_int) + *ku + 1 as ::core::ffi::c_long
                {
                    *info = -(9 as ::core::ffi::c_int) as integer;
                }
            }
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLASCL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        if *n == 0 as ::core::ffi::c_long || *m == 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        smlnum = dlamch__0(
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        bignum = 1.0f64 / smlnum;
        cfromc = *cfrom;
        ctoc = *cto;
        loop {
            cfrom1 = cfromc * smlnum;
            if cfrom1 == cfromc {
                mul = ctoc / cfromc;
                done = TRUE_ as logical;
                cto1 = ctoc;
            } else {
                cto1 = ctoc / bignum;
                if cto1 == ctoc {
                    mul = ctoc;
                    done = TRUE_ as logical;
                    cfromc = 1.0f64 as doublereal;
                } else if (if cfrom1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    cfrom1 as ::core::ffi::c_double
                } else {
                    -(cfrom1 as ::core::ffi::c_double)
                }) > (if ctoc >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    ctoc as ::core::ffi::c_double
                } else {
                    -(ctoc as ::core::ffi::c_double)
                }) && ctoc != 0.0f64
                {
                    mul = smlnum;
                    done = FALSE_ as logical;
                    cfromc = cfrom1;
                } else if (if cto1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    cto1 as ::core::ffi::c_double
                } else {
                    -(cto1 as ::core::ffi::c_double)
                }) > (if cfromc >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    cfromc as ::core::ffi::c_double
                } else {
                    -(cfromc as ::core::ffi::c_double)
                }) {
                    mul = bignum;
                    done = FALSE_ as logical;
                    ctoc = cto1;
                } else {
                    mul = ctoc / cfromc;
                    done = TRUE_ as logical;
                }
            }
            if itype == 0 as ::core::ffi::c_long {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = *m;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        let ref mut fresh0 = *a.offset((i__ + j * a_dim1) as isize);
                        *fresh0 *= mul as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
            } else if itype == 1 as ::core::ffi::c_long {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = *m;
                    i__ = j;
                    while i__ <= i__2 {
                        let ref mut fresh1 = *a.offset((i__ + j * a_dim1) as isize);
                        *fresh1 *= mul as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
            } else if itype == 2 as ::core::ffi::c_long {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = (if j <= *m {
                        j as ::core::ffi::c_long
                    } else {
                        *m
                    }) as integer;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        let ref mut fresh2 = *a.offset((i__ + j * a_dim1) as isize);
                        *fresh2 *= mul as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
            } else if itype == 3 as ::core::ffi::c_long {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__3 = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    i__2 = (if i__3 <= *m {
                        i__3 as ::core::ffi::c_long
                    } else {
                        *m
                    }) as integer;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        let ref mut fresh3 = *a.offset((i__ + j * a_dim1) as isize);
                        *fresh3 *= mul as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
            } else if itype == 4 as ::core::ffi::c_long {
                k3 = (*kl + 1 as ::core::ffi::c_long) as integer;
                k4 = (*n + 1 as ::core::ffi::c_long) as integer;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__3 = k3;
                    i__4 = k4 - j;
                    i__2 = (if i__3 <= i__4 {
                        i__3 as ::core::ffi::c_long
                    } else {
                        i__4 as ::core::ffi::c_long
                    }) as integer;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        let ref mut fresh4 = *a.offset((i__ + j * a_dim1) as isize);
                        *fresh4 *= mul as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
            } else if itype == 5 as ::core::ffi::c_long {
                k1 = (*ku + 2 as ::core::ffi::c_long) as integer;
                k3 = (*ku + 1 as ::core::ffi::c_long) as integer;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = k1 - j;
                    i__3 = k3;
                    i__ = (if i__2 >= 1 as ::core::ffi::c_long {
                        i__2 as ::core::ffi::c_long
                    } else {
                        1 as ::core::ffi::c_long
                    }) as integer;
                    while i__ <= i__3 {
                        let ref mut fresh5 = *a.offset((i__ + j * a_dim1) as isize);
                        *fresh5 *= mul as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
            } else if itype == 6 as ::core::ffi::c_long {
                k1 = (*kl + *ku + 2 as ::core::ffi::c_long) as integer;
                k2 = (*kl + 1 as ::core::ffi::c_long) as integer;
                k3 = ((*kl << 1 as ::core::ffi::c_int) + *ku + 1 as ::core::ffi::c_long) as integer;
                k4 = *kl + *ku + 1 as integer + *m;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__3 = k1 - j;
                    i__4 = k3;
                    i__5 = k4 - j;
                    i__2 = (if i__4 <= i__5 {
                        i__4 as ::core::ffi::c_long
                    } else {
                        i__5 as ::core::ffi::c_long
                    }) as integer;
                    i__ = (if i__3 >= k2 {
                        i__3 as ::core::ffi::c_long
                    } else {
                        k2 as ::core::ffi::c_long
                    }) as integer;
                    while i__ <= i__2 {
                        let ref mut fresh6 = *a.offset((i__ + j * a_dim1) as isize);
                        *fresh6 *= mul as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
            }
            if !(done == 0) {
                break;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlascl::dgelsd_closure_dlascl_;

pub mod raw_dlasdq {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    static mut c__1: integer = 1 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasdq_(
        mut uplo: *mut ::core::ffi::c_char,
        mut sqre: *mut integer,
        mut n: *mut integer,
        mut ncvt: *mut integer,
        mut nru: *mut integer,
        mut ncc: *mut integer,
        mut d__: *mut doublereal,
        mut e: *mut doublereal,
        mut vt: *mut doublereal,
        mut ldvt: *mut integer,
        mut u: *mut doublereal,
        mut ldu: *mut integer,
        mut c__: *mut doublereal,
        mut ldc: *mut integer,
        mut work: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut c_dim1: integer = 0;
        let mut c_offset: integer = 0;
        let mut u_dim1: integer = 0;
        let mut u_offset: integer = 0;
        let mut vt_dim1: integer = 0;
        let mut vt_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut r__: doublereal = 0.;
        let mut cs: doublereal = 0.;
        let mut sn: doublereal = 0.;
        let mut np1: integer = 0;
        let mut isub: integer = 0;
        let mut smin: doublereal = 0.;
        let mut sqre1: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_lsame_"]
            fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasr_"]
            fn dlasr__0(
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dswap"]
            fn f2c_dswap_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut iuplo: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlartg_"]
            fn dlartg__0(
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dbdsqr_"]
            fn dbdsqr__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut rotate: logical = 0;
        d__ = d__.wrapping_offset(-1);
        e = e.wrapping_offset(-1);
        vt_dim1 = *ldvt;
        vt_offset = 1 as integer + vt_dim1;
        vt = vt.wrapping_offset(-(vt_offset as isize));
        u_dim1 = *ldu;
        u_offset = 1 as integer + u_dim1;
        u = u.wrapping_offset(-(u_offset as isize));
        c_dim1 = *ldc;
        c_offset = 1 as integer + c_dim1;
        c__ = c__.wrapping_offset(-(c_offset as isize));
        work = work.wrapping_offset(-1);
        *info = 0 as integer;
        iuplo = 0 as integer;
        if lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            iuplo = 1 as integer;
        }
        if lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            iuplo = 2 as integer;
        }
        if iuplo == 0 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *sqre < 0 as ::core::ffi::c_long || *sqre > 1 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        } else if *n < 0 as ::core::ffi::c_long {
            *info = -(3 as ::core::ffi::c_int) as integer;
        } else if *ncvt < 0 as ::core::ffi::c_long {
            *info = -(4 as ::core::ffi::c_int) as integer;
        } else if *nru < 0 as ::core::ffi::c_long {
            *info = -(5 as ::core::ffi::c_int) as integer;
        } else if *ncc < 0 as ::core::ffi::c_long {
            *info = -(6 as ::core::ffi::c_int) as integer;
        } else if *ncvt == 0 as ::core::ffi::c_long && *ldvt < 1 as ::core::ffi::c_long
            || *ncvt > 0 as ::core::ffi::c_long
                && *ldvt
                    < (if 1 as ::core::ffi::c_long >= *n {
                        1 as ::core::ffi::c_long
                    } else {
                        *n
                    })
        {
            *info = -(10 as ::core::ffi::c_int) as integer;
        } else if *ldu
            < (if 1 as ::core::ffi::c_long >= *nru {
                1 as ::core::ffi::c_long
            } else {
                *nru
            })
        {
            *info = -(12 as ::core::ffi::c_int) as integer;
        } else if *ncc == 0 as ::core::ffi::c_long && *ldc < 1 as ::core::ffi::c_long
            || *ncc > 0 as ::core::ffi::c_long
                && *ldc
                    < (if 1 as ::core::ffi::c_long >= *n {
                        1 as ::core::ffi::c_long
                    } else {
                        *n
                    })
        {
            *info = -(14 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLASDQ\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        if *n == 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        rotate = (*ncvt > 0 as ::core::ffi::c_long
            || *nru > 0 as ::core::ffi::c_long
            || *ncc > 0 as ::core::ffi::c_long) as ::core::ffi::c_int as logical;
        np1 = (*n + 1 as ::core::ffi::c_long) as integer;
        sqre1 = *sqre;
        if iuplo == 1 as ::core::ffi::c_long && sqre1 == 1 as ::core::ffi::c_long {
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__1 {
                dlartg__0(
                    d__.offset(i__ as isize) as *mut doublereal,
                    e.offset(i__ as isize) as *mut doublereal,
                    &raw mut cs,
                    &raw mut sn,
                    &raw mut r__,
                );
                *d__.offset(i__ as isize) = r__;
                *e.offset(i__ as isize) = sn
                    * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) = cs
                    * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                if rotate != 0 {
                    *work.offset(i__ as isize) = cs;
                    *work.offset((*n + i__) as isize) = sn;
                }
                i__ += 1;
            }
            dlartg__0(
                d__.offset(*n as isize) as *mut doublereal,
                e.offset(*n as isize) as *mut doublereal,
                &raw mut cs,
                &raw mut sn,
                &raw mut r__,
            );
            *d__.offset(*n as isize) = r__;
            *e.offset(*n as isize) = 0.0f64 as doublereal;
            if rotate != 0 {
                *work.offset(*n as isize) = cs;
                *work.offset((*n + *n) as isize) = sn;
            }
            iuplo = 2 as integer;
            sqre1 = 0 as integer;
            if *ncvt > 0 as ::core::ffi::c_long {
                dlasr__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut np1,
                    ncvt,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(np1 as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                );
            }
        }
        if iuplo == 2 as ::core::ffi::c_long {
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__1 {
                dlartg__0(
                    d__.offset(i__ as isize) as *mut doublereal,
                    e.offset(i__ as isize) as *mut doublereal,
                    &raw mut cs,
                    &raw mut sn,
                    &raw mut r__,
                );
                *d__.offset(i__ as isize) = r__;
                *e.offset(i__ as isize) = sn
                    * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) = cs
                    * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                if rotate != 0 {
                    *work.offset(i__ as isize) = cs;
                    *work.offset((*n + i__) as isize) = sn;
                }
                i__ += 1;
            }
            if sqre1 == 1 as ::core::ffi::c_long {
                dlartg__0(
                    d__.offset(*n as isize) as *mut doublereal,
                    e.offset(*n as isize) as *mut doublereal,
                    &raw mut cs,
                    &raw mut sn,
                    &raw mut r__,
                );
                *d__.offset(*n as isize) = r__;
                if rotate != 0 {
                    *work.offset(*n as isize) = cs;
                    *work.offset((*n + *n) as isize) = sn;
                }
            }
            if *nru > 0 as ::core::ffi::c_long {
                if sqre1 == 0 as ::core::ffi::c_long {
                    dlasr__0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        nru,
                        n,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        work.offset(np1 as isize) as *mut doublereal,
                        u.offset(u_offset as isize) as *mut doublereal,
                        ldu,
                    );
                } else {
                    dlasr__0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        nru,
                        &raw mut np1,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        work.offset(np1 as isize) as *mut doublereal,
                        u.offset(u_offset as isize) as *mut doublereal,
                        ldu,
                    );
                }
            }
            if *ncc > 0 as ::core::ffi::c_long {
                if sqre1 == 0 as ::core::ffi::c_long {
                    dlasr__0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        n,
                        ncc,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        work.offset(np1 as isize) as *mut doublereal,
                        c__.offset(c_offset as isize) as *mut doublereal,
                        ldc,
                    );
                } else {
                    dlasr__0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut np1,
                        ncc,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        work.offset(np1 as isize) as *mut doublereal,
                        c__.offset(c_offset as isize) as *mut doublereal,
                        ldc,
                    );
                }
            }
        }
        dbdsqr__0(
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            ncvt,
            nru,
            ncc,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vt.offset(vt_offset as isize) as *mut doublereal,
            ldvt,
            u.offset(u_offset as isize) as *mut doublereal,
            ldu,
            c__.offset(c_offset as isize) as *mut doublereal,
            ldc,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            isub = i__;
            smin = *d__.offset(i__ as isize);
            i__2 = *n;
            j = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while j <= i__2 {
                if *d__.offset(j as isize) < smin {
                    isub = j;
                    smin = *d__.offset(j as isize);
                }
                j += 1;
            }
            if isub != i__ {
                *d__.offset(isub as isize) = *d__.offset(i__ as isize);
                *d__.offset(i__ as isize) = smin;
                if *ncvt > 0 as ::core::ffi::c_long {
                    f2c_dswap_0(
                        ncvt,
                        vt.offset((isub + vt_dim1) as isize) as *mut doublereal,
                        ldvt,
                        vt.offset((i__ + vt_dim1) as isize) as *mut doublereal,
                        ldvt,
                    );
                }
                if *nru > 0 as ::core::ffi::c_long {
                    f2c_dswap_0(
                        nru,
                        u.offset(
                            (isub as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        u.offset(
                            (i__ as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                }
                if *ncc > 0 as ::core::ffi::c_long {
                    f2c_dswap_0(
                        ncc,
                        c__.offset((isub + c_dim1) as isize) as *mut doublereal,
                        ldc,
                        c__.offset((i__ + c_dim1) as isize) as *mut doublereal,
                        ldc,
                    );
                }
            }
            i__ += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasdq::dgelsd_closure_dlasdq_;

pub mod raw_dlasdt {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn log(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasdt_(
        mut n: *mut integer,
        mut lvl: *mut integer,
        mut nd: *mut integer,
        mut inode: *mut integer,
        mut ndiml: *mut integer,
        mut ndimr: *mut integer,
        mut msub: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut i__: integer = 0;
        let mut il: integer = 0;
        let mut ir: integer = 0;
        let mut maxn: integer = 0;
        let mut temp: doublereal = 0.;
        let mut nlvl: integer = 0;
        let mut llst: integer = 0;
        let mut ncrnt: integer = 0;
        ndimr = ndimr.wrapping_offset(-1);
        ndiml = ndiml.wrapping_offset(-1);
        inode = inode.wrapping_offset(-1);
        maxn = (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        }) as integer;
        temp = (super::lapack_log(
            maxn as doublereal / (*msub + 1 as ::core::ffi::c_long) as doublereal,
        ) / super::lapack_log(2.0f64)) as doublereal;
        *lvl = (temp as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__ = (*n / 2 as ::core::ffi::c_long) as integer;
        *inode.offset(1 as ::core::ffi::c_int as isize) =
            (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        *ndiml.offset(1 as ::core::ffi::c_int as isize) = i__;
        *ndimr.offset(1 as ::core::ffi::c_int as isize) =
            (*n - i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        il = 0 as integer;
        ir = 1 as integer;
        llst = 1 as integer;
        i__1 = (*lvl - 1 as ::core::ffi::c_long) as integer;
        nlvl = 1 as integer;
        while nlvl <= i__1 {
            i__2 = (llst as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            i__ = 0 as integer;
            while i__ <= i__2 {
                il += 2 as ::core::ffi::c_long;
                ir += 2 as ::core::ffi::c_long;
                ncrnt = llst + i__;
                *ndiml.offset(il as isize) = (*ndiml.offset(ncrnt as isize) as ::core::ffi::c_long
                    / 2 as ::core::ffi::c_long)
                    as integer;
                *ndimr.offset(il as isize) = (*ndiml.offset(ncrnt as isize) as ::core::ffi::c_long
                    - *ndiml.offset(il as isize) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long)
                    as integer;
                *inode.offset(il as isize) = (*inode.offset(ncrnt as isize) as ::core::ffi::c_long
                    - *ndimr.offset(il as isize) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long)
                    as integer;
                *ndiml.offset(ir as isize) = (*ndimr.offset(ncrnt as isize) as ::core::ffi::c_long
                    / 2 as ::core::ffi::c_long)
                    as integer;
                *ndimr.offset(ir as isize) = (*ndimr.offset(ncrnt as isize) as ::core::ffi::c_long
                    - *ndiml.offset(ir as isize) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long)
                    as integer;
                *inode.offset(ir as isize) = (*inode.offset(ncrnt as isize) as ::core::ffi::c_long
                    + *ndiml.offset(ir as isize) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long)
                    as integer;
                i__ += 1;
            }
            llst <<= 1 as ::core::ffi::c_int;
            nlvl += 1;
        }
        *nd = (((llst as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_long) as integer;
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasdt::dgelsd_closure_dlasdt_;

pub mod raw_dlaset {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlaset_(
        mut uplo: *mut ::core::ffi::c_char,
        mut m: *mut integer,
        mut n: *mut integer,
        mut alpha: *mut doublereal,
        mut beta: *mut doublereal,
        mut a: *mut doublereal,
        mut lda: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut a_dim1: integer = 0;
        let mut a_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut i__3: integer = 0;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_lsame_"]
            fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
        }
        a_dim1 = *lda;
        a_offset = 1 as integer + a_dim1;
        a = a.wrapping_offset(-(a_offset as isize));
        if lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            i__1 = *n;
            j = 2 as integer;
            while j <= i__1 {
                i__3 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__2 = (if i__3 <= *m {
                    i__3 as ::core::ffi::c_long
                } else {
                    *m
                }) as integer;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    *a.offset((i__ + j * a_dim1) as isize) = *alpha;
                    i__ += 1;
                }
                j += 1;
            }
        } else if lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            i__1 = (if *m <= *n { *m } else { *n }) as integer;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                while i__ <= i__2 {
                    *a.offset((i__ + j * a_dim1) as isize) = *alpha;
                    i__ += 1;
                }
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    *a.offset((i__ + j * a_dim1) as isize) = *alpha;
                    i__ += 1;
                }
                j += 1;
            }
        }
        i__1 = (if *m <= *n { *m } else { *n }) as integer;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *a.offset((i__ + i__ * a_dim1) as isize) = *beta;
            i__ += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlaset::dgelsd_closure_dlaset_;

pub mod raw_dlasq1 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    static mut c__1: integer = 1 as integer;
    static mut c__2: integer = 2 as integer;
    static mut c__0: integer = 0 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasq1_(
        mut n: *mut integer,
        mut d__: *mut doublereal,
        mut e: *mut doublereal,
        mut work: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut d__3: doublereal = 0.;
        let mut i__: integer = 0;
        let mut eps: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlas2_"]
            fn dlas2__0(
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        let mut scale: doublereal = 0.;
        let mut iinfo: integer = 0;
        let mut sigmn: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_f2c_dcopy"]
            fn f2c_dcopy_0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut sigmx: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlasq2_"]
            fn dlasq2__0(
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlascl_"]
            fn dlascl__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        let mut safmin: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasrt_"]
            fn dlasrt__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        work = work.wrapping_offset(-1);
        e = e.wrapping_offset(-1);
        d__ = d__.wrapping_offset(-1);
        *info = 0 as integer;
        if *n < 0 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
            i__1 = -*info;
            xerbla__0(
                b"DLASQ1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        } else if *n == 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        } else if *n == 1 as ::core::ffi::c_long {
            *d__.offset(1 as ::core::ffi::c_int as isize) = (if *d__
                .offset(1 as ::core::ffi::c_int as isize)
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                *d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            } else {
                -(*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
            }) as doublereal;
            return 0 as ::core::ffi::c_int;
        } else if *n == 2 as ::core::ffi::c_long {
            dlas2__0(
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                d__.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut sigmn,
                &raw mut sigmx,
            );
            *d__.offset(1 as ::core::ffi::c_int as isize) = sigmx;
            *d__.offset(2 as ::core::ffi::c_int as isize) = sigmn;
            return 0 as ::core::ffi::c_int;
        }
        sigmx = 0.0f64 as doublereal;
        i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *d__.offset(i__ as isize);
            *d__.offset(i__ as isize) = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = sigmx;
            d__1 = *e.offset(i__ as isize);
            d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            sigmx = (if d__2 >= d__3 {
                d__2 as ::core::ffi::c_double
            } else {
                d__3 as ::core::ffi::c_double
            }) as doublereal;
            i__ += 1;
        }
        d__1 = *d__.offset(*n as isize);
        *d__.offset(*n as isize) = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        }) as doublereal;
        if sigmx == 0.0f64 {
            dlasrt__0(
                b"D\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut iinfo,
            );
            return 0 as ::core::ffi::c_int;
        }
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = sigmx;
            d__2 = *d__.offset(i__ as isize);
            sigmx = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            i__ += 1;
        }
        eps = dlamch__0(
            b"Precision\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        safmin = dlamch__0(
            b"Safe minimum\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        scale = super::lapack_sqrt(eps / safmin) as doublereal;
        f2c_dcopy_0(
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__2,
        );
        i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
        f2c_dcopy_0(
            &raw mut i__1,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            work.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__2,
        );
        i__1 = ((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as integer;
        i__2 = ((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as integer;
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut sigmx,
            &raw mut scale,
            &raw mut i__1,
            &raw mut c__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut i__2,
            &raw mut iinfo,
        );
        i__1 = ((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as integer;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *work.offset(i__ as isize);
            *work.offset(i__ as isize) = d__1 * d__1;
            i__ += 1;
        }
        *work.offset((*n * 2 as ::core::ffi::c_long) as isize) = 0.0f64 as doublereal;
        dlasq2__0(
            n,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
        if *info == 0 as ::core::ffi::c_long {
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                *d__.offset(i__ as isize) =
                    super::lapack_sqrt(*work.offset(i__ as isize)) as doublereal;
                i__ += 1;
            }
            dlascl__0(
                b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                &raw mut c__0,
                &raw mut scale,
                &raw mut sigmx,
                n,
                &raw mut c__1,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
                &raw mut iinfo,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasq1::dgelsd_closure_dlasq1_;

pub mod raw_dlasq2 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    static mut c__1: integer = 1 as integer;
    static mut c__2: integer = 2 as integer;
    static mut c__10: integer = 10 as integer;
    static mut c__3: integer = 3 as integer;
    static mut c__4: integer = 4 as integer;
    static mut c__11: integer = 11 as integer;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasq2_(
        mut n: *mut integer,
        mut z__: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut current_block: u64;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut i__3: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut d__: doublereal = 0.;
        let mut e: doublereal = 0.;
        let mut g: doublereal = 0.;
        let mut k: integer = 0;
        let mut s: doublereal = 0.;
        let mut t: doublereal = 0.;
        let mut i0: integer = 0;
        let mut i4: integer = 0;
        let mut n0: integer = 0;
        let mut dn: doublereal = 0.;
        let mut pp: integer = 0;
        let mut dn1: doublereal = 0.;
        let mut dn2: doublereal = 0.;
        let mut dee: doublereal = 0.;
        let mut eps: doublereal = 0.;
        let mut tau: doublereal = 0.;
        let mut tol: doublereal = 0.;
        let mut ipn4: integer = 0;
        let mut tol2: doublereal = 0.;
        let mut ieee: logical = 0;
        let mut nbig: integer = 0;
        let mut dmin__: doublereal = 0.;
        let mut emin: doublereal = 0.;
        let mut emax: doublereal = 0.;
        let mut kmin: integer = 0;
        let mut ndiv: integer = 0;
        let mut iter: integer = 0;
        let mut qmin: doublereal = 0.;
        let mut temp: doublereal = 0.;
        let mut qmax: doublereal = 0.;
        let mut zmax: doublereal = 0.;
        let mut splt: integer = 0;
        let mut dmin1: doublereal = 0.;
        let mut dmin2: doublereal = 0.;
        let mut nfail: integer = 0;
        let mut desig: doublereal = 0.;
        let mut trace: doublereal = 0.;
        let mut sigma: doublereal = 0.;
        let mut iinfo: integer = 0;
        let mut ttype: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_dlasq3_"]
            fn dlasq3__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut logical,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        let mut deemin: doublereal = 0.;
        let mut iwhila: integer = 0;
        let mut iwhilb: integer = 0;
        let mut oldemn: doublereal = 0.;
        let mut safmin: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_ilaenv_"]
            fn ilaenv__0(
                _: *mut integer,
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
                _: *mut integer,
            ) -> integer;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasrt_"]
            fn dlasrt__0(
                _: *mut ::core::ffi::c_char,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
            ) -> ::core::ffi::c_int;
        }
        z__ = z__.wrapping_offset(-1);
        *info = 0 as integer;
        eps = dlamch__0(
            b"Precision\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        safmin = dlamch__0(
            b"Safe minimum\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        tol = (eps as ::core::ffi::c_double * 100.0f64) as doublereal;
        d__1 = tol;
        tol2 = d__1 * d__1;
        if *n < 0 as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
            xerbla__0(
                b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__1,
            );
            return 0 as ::core::ffi::c_int;
        } else if *n == 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        } else if *n == 1 as ::core::ffi::c_long {
            if *z__.offset(1 as ::core::ffi::c_int as isize) < 0.0f64 {
                *info = -(201 as ::core::ffi::c_int) as integer;
                xerbla__0(
                    b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut c__2,
                );
            }
            return 0 as ::core::ffi::c_int;
        } else if *n == 2 as ::core::ffi::c_long {
            if *z__.offset(2 as ::core::ffi::c_int as isize) < 0.0f64
                || *z__.offset(3 as ::core::ffi::c_int as isize) < 0.0f64
            {
                *info = -(2 as ::core::ffi::c_int) as integer;
                xerbla__0(
                    b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut c__2,
                );
                return 0 as ::core::ffi::c_int;
            } else if *z__.offset(3 as ::core::ffi::c_int as isize)
                > *z__.offset(1 as ::core::ffi::c_int as isize)
            {
                d__ = *z__.offset(3 as ::core::ffi::c_int as isize);
                *z__.offset(3 as ::core::ffi::c_int as isize) =
                    *z__.offset(1 as ::core::ffi::c_int as isize);
                *z__.offset(1 as ::core::ffi::c_int as isize) = d__;
            }
            *z__.offset(5 as ::core::ffi::c_int as isize) = *z__
                .offset(1 as ::core::ffi::c_int as isize)
                + *z__.offset(2 as ::core::ffi::c_int as isize)
                + *z__.offset(3 as ::core::ffi::c_int as isize);
            if *z__.offset(2 as ::core::ffi::c_int as isize)
                > *z__.offset(3 as ::core::ffi::c_int as isize) * tol2
            {
                t = ((*z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    - *z__.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    + *z__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                    * 0.5f64) as doublereal;
                s = *z__.offset(3 as ::core::ffi::c_int as isize)
                    * (*z__.offset(2 as ::core::ffi::c_int as isize) / t);
                if s <= t {
                    s = (*z__.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                        * (*z__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                            / (t as ::core::ffi::c_double
                                * (super::lapack_sqrt(s / t + 1.0f64) + 1.0f64))))
                        as doublereal;
                } else {
                    s = (*z__.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                        * (*z__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                            / (t as ::core::ffi::c_double
                                + super::lapack_sqrt(t) * super::lapack_sqrt(t + s))))
                        as doublereal;
                }
                t = *z__.offset(1 as ::core::ffi::c_int as isize)
                    + (s + *z__.offset(2 as ::core::ffi::c_int as isize));
                let ref mut fresh0 = *z__.offset(3 as ::core::ffi::c_int as isize);
                *fresh0 *=
                    (*z__.offset(1 as ::core::ffi::c_int as isize) / t) as ::core::ffi::c_double;
                *z__.offset(1 as ::core::ffi::c_int as isize) = t;
            }
            *z__.offset(2 as ::core::ffi::c_int as isize) =
                *z__.offset(3 as ::core::ffi::c_int as isize);
            *z__.offset(6 as ::core::ffi::c_int as isize) = *z__
                .offset(2 as ::core::ffi::c_int as isize)
                + *z__.offset(1 as ::core::ffi::c_int as isize);
            return 0 as ::core::ffi::c_int;
        }
        *z__.offset((*n * 2 as ::core::ffi::c_long) as isize) = 0.0f64 as doublereal;
        emin = *z__.offset(2 as ::core::ffi::c_int as isize);
        qmax = 0.0f64 as doublereal;
        zmax = 0.0f64 as doublereal;
        d__ = 0.0f64 as doublereal;
        e = 0.0f64 as doublereal;
        i__1 = ((*n - 1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) as integer;
        k = 1 as integer;
        while k <= i__1 {
            if *z__.offset(k as isize) < 0.0f64 {
                *info = -(k as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as integer;
                xerbla__0(
                    b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut c__2,
                );
                return 0 as ::core::ffi::c_int;
            } else if *z__.offset((k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                < 0.0f64
            {
                *info = -(k as ::core::ffi::c_long + 201 as ::core::ffi::c_long) as integer;
                xerbla__0(
                    b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut c__2,
                );
                return 0 as ::core::ffi::c_int;
            }
            d__ += *z__.offset(k as isize) as ::core::ffi::c_double;
            e += *z__.offset((k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as ::core::ffi::c_double;
            d__1 = qmax;
            d__2 = *z__.offset(k as isize);
            qmax = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            d__1 = emin;
            d__2 = *z__.offset((k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            emin = (if d__1 <= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            d__1 = (if qmax >= zmax {
                qmax as ::core::ffi::c_double
            } else {
                zmax as ::core::ffi::c_double
            }) as doublereal;
            d__2 = *z__.offset((k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            zmax = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            k += 2 as ::core::ffi::c_long;
        }
        if *z__.offset(((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as isize)
            < 0.0f64
        {
            *info = -((*n << 1 as ::core::ffi::c_int) + 199 as ::core::ffi::c_long) as integer;
            xerbla__0(
                b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__2,
            );
            return 0 as ::core::ffi::c_int;
        }
        d__ += *z__.offset(((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as isize)
            as ::core::ffi::c_double;
        d__1 = qmax;
        d__2 = *z__.offset(((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as isize);
        qmax = (if d__1 >= d__2 {
            d__1 as ::core::ffi::c_double
        } else {
            d__2 as ::core::ffi::c_double
        }) as doublereal;
        zmax = (if qmax >= zmax {
            qmax as ::core::ffi::c_double
        } else {
            zmax as ::core::ffi::c_double
        }) as doublereal;
        if e == 0.0f64 {
            i__1 = *n;
            k = 2 as integer;
            while k <= i__1 {
                *z__.offset(k as isize) = *z__.offset(
                    (((k as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_long) as isize,
                );
                k += 1;
            }
            dlasrt__0(
                b"D\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut iinfo,
            );
            *z__.offset(((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as isize) =
                d__;
            return 0 as ::core::ffi::c_int;
        }
        trace = d__ + e;
        if trace == 0.0f64 {
            *z__.offset(((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as isize) =
                0.0f64 as doublereal;
            return 0 as ::core::ffi::c_int;
        }
        ieee = (ilaenv__0(
            &raw mut c__10,
            b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__1,
            &raw mut c__2,
            &raw mut c__3,
            &raw mut c__4,
        ) == 1 as ::core::ffi::c_long
            && ilaenv__0(
                &raw mut c__11,
                b"DLASQ2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__1,
                &raw mut c__2,
                &raw mut c__3,
                &raw mut c__4,
            ) == 1 as ::core::ffi::c_long) as ::core::ffi::c_int as logical;
        k = *n << 1 as ::core::ffi::c_int;
        while k >= 2 as ::core::ffi::c_long {
            *z__.offset((k as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize) =
                0.0f64 as doublereal;
            *z__.offset(
                (((k as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long)
                    as isize,
            ) = *z__.offset(k as isize);
            *z__.offset(
                (((k as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) - 2 as ::core::ffi::c_long)
                    as isize,
            ) = 0.0f64 as doublereal;
            *z__.offset(
                (((k as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) - 3 as ::core::ffi::c_long)
                    as isize,
            ) = *z__.offset((k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
            k += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
        }
        i0 = 1 as integer;
        n0 = *n;
        if *z__.offset(
            (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) - 3 as ::core::ffi::c_long)
                as isize,
        ) as ::core::ffi::c_double
            * 1.5f64
            < *z__.offset(
                (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - 3 as ::core::ffi::c_long) as isize,
            )
        {
            ipn4 = i0 + n0 << 2 as ::core::ffi::c_int;
            i__1 = ((i0 as ::core::ffi::c_long + n0 as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long)
                << 1 as ::core::ffi::c_int) as integer;
            i4 = i0 << 2 as ::core::ffi::c_int;
            while i4 <= i__1 {
                temp = *z__.offset((i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize);
                *z__.offset((i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize) = *z__
                    .offset(
                        (ipn4 as ::core::ffi::c_long
                            - i4 as ::core::ffi::c_long
                            - 3 as ::core::ffi::c_long) as isize,
                    );
                *z__.offset(
                    (ipn4 as ::core::ffi::c_long
                        - i4 as ::core::ffi::c_long
                        - 3 as ::core::ffi::c_long) as isize,
                ) = temp;
                temp = *z__.offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                *z__.offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) = *z__
                    .offset(
                        (ipn4 as ::core::ffi::c_long
                            - i4 as ::core::ffi::c_long
                            - 5 as ::core::ffi::c_long) as isize,
                    );
                *z__.offset(
                    (ipn4 as ::core::ffi::c_long
                        - i4 as ::core::ffi::c_long
                        - 5 as ::core::ffi::c_long) as isize,
                ) = temp;
                i4 += 4 as ::core::ffi::c_long;
            }
        }
        pp = 0 as integer;
        k = 1 as integer;
        while k <= 2 as ::core::ffi::c_long {
            d__ = *z__.offset(
                (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    + pp as ::core::ffi::c_long
                    - 3 as ::core::ffi::c_long) as isize,
            );
            i__1 = (i0 << 2 as ::core::ffi::c_int) + pp;
            i4 = ((n0 - 1 as integer) << 2 as ::core::ffi::c_int) + pp;
            while i4 >= i__1 {
                if *z__.offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                    <= tol2 * d__
                {
                    *z__.offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                        -0.0f64 as doublereal;
                    d__ = *z__
                        .offset((i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize);
                } else {
                    d__ = *z__
                        .offset((i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize)
                        * (d__
                            / (d__
                                + *z__.offset(
                                    (i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                )));
                }
                i4 += -(4 as ::core::ffi::c_int) as ::core::ffi::c_long;
            }
            emin = *z__.offset(
                (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    + pp as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            );
            d__ = *z__.offset(
                (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    + pp as ::core::ffi::c_long
                    - 3 as ::core::ffi::c_long) as isize,
            );
            i__1 = ((n0 - 1 as integer) << 2 as ::core::ffi::c_int) + pp;
            i4 = (i0 << 2 as ::core::ffi::c_int) + pp;
            while i4 <= i__1 {
                *z__.offset(
                    (i4 as ::core::ffi::c_long
                        - ((pp as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        - 2 as ::core::ffi::c_long) as isize,
                ) = d__
                    + *z__.offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                if *z__.offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                    <= tol2 * d__
                {
                    *z__.offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                        -0.0f64 as doublereal;
                    *z__.offset(
                        (i4 as ::core::ffi::c_long
                            - ((pp as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as isize,
                    ) = d__;
                    *z__.offset((i4 - (pp << 1 as ::core::ffi::c_int)) as isize) =
                        0.0f64 as doublereal;
                    d__ = *z__
                        .offset((i4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                } else if safmin
                    * *z__.offset((i4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    < *z__.offset(
                        (i4 as ::core::ffi::c_long
                            - ((pp as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as isize,
                    )
                    && safmin
                        * *z__.offset(
                            (i4 as ::core::ffi::c_long
                                - ((pp as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 2 as ::core::ffi::c_long) as isize,
                        )
                        < *z__
                            .offset((i4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                {
                    temp = *z__
                        .offset((i4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        / *z__.offset(
                            (i4 as ::core::ffi::c_long
                                - ((pp as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 2 as ::core::ffi::c_long) as isize,
                        );
                    *z__.offset((i4 - (pp << 1 as ::core::ffi::c_int)) as isize) = *z__
                        .offset((i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                        * temp;
                    d__ *= temp as ::core::ffi::c_double;
                } else {
                    *z__.offset((i4 - (pp << 1 as ::core::ffi::c_int)) as isize) = *z__
                        .offset((i4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        * (*z__.offset(
                            (i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) / *z__.offset(
                            (i4 as ::core::ffi::c_long
                                - ((pp as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 2 as ::core::ffi::c_long) as isize,
                        ));
                    d__ = *z__
                        .offset((i4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        * (d__
                            / *z__.offset(
                                (i4 as ::core::ffi::c_long
                                    - ((pp as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                    - 2 as ::core::ffi::c_long)
                                    as isize,
                            ));
                }
                d__1 = emin;
                d__2 = *z__.offset((i4 - (pp << 1 as ::core::ffi::c_int)) as isize);
                emin = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                i4 += 4 as ::core::ffi::c_long;
            }
            *z__.offset(
                (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - pp as ::core::ffi::c_long
                    - 2 as ::core::ffi::c_long) as isize,
            ) = d__;
            qmax = *z__.offset(
                (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - pp as ::core::ffi::c_long
                    - 2 as ::core::ffi::c_long) as isize,
            );
            i__1 = (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - pp as ::core::ffi::c_long
                - 2 as ::core::ffi::c_long) as integer;
            i4 = (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - pp as ::core::ffi::c_long
                + 2 as ::core::ffi::c_long) as integer;
            while i4 <= i__1 {
                d__1 = qmax;
                d__2 = *z__.offset(i4 as isize);
                qmax = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                i4 += 4 as ::core::ffi::c_long;
            }
            pp = 1 as integer - pp;
            k += 1;
        }
        ttype = 0 as integer;
        dmin1 = 0.0f64 as doublereal;
        dmin2 = 0.0f64 as doublereal;
        dn = 0.0f64 as doublereal;
        dn1 = 0.0f64 as doublereal;
        dn2 = 0.0f64 as doublereal;
        g = 0.0f64 as doublereal;
        tau = 0.0f64 as doublereal;
        iter = 2 as integer;
        nfail = 0 as integer;
        ndiv = n0 - i0 << 1 as ::core::ffi::c_int;
        i__1 = (*n + 1 as ::core::ffi::c_long) as integer;
        iwhila = 1 as integer;
        loop {
            if !(iwhila <= i__1) {
                current_block = 15696916892398440870;
                break;
            }
            if n0 < 1 as ::core::ffi::c_long {
                current_block = 13067763561185533911;
                break;
            }
            desig = 0.0f64 as doublereal;
            if n0 == *n {
                sigma = 0.0f64 as doublereal;
            } else {
                sigma = -*z__.offset(
                    (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_long) as isize,
                );
            }
            if sigma < 0.0f64 {
                *info = 1 as integer;
                return 0 as ::core::ffi::c_int;
            }
            emax = 0.0f64 as doublereal;
            if n0 > i0 {
                d__1 = *z__.offset(
                    (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 5 as ::core::ffi::c_long) as isize,
                );
                emin = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
            } else {
                emin = 0.0f64 as doublereal;
            }
            qmin = *z__.offset(
                (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - 3 as ::core::ffi::c_long) as isize,
            );
            qmax = qmin;
            i4 = n0 << 2 as ::core::ffi::c_int;
            loop {
                if !(i4 >= 8 as ::core::ffi::c_long) {
                    current_block = 9240481512215375588;
                    break;
                }
                if *z__.offset((i4 as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                    <= 0.0f64
                {
                    current_block = 2026672097358460402;
                    break;
                }
                if qmin >= emax as ::core::ffi::c_double * 4.0f64 {
                    d__1 = qmin;
                    d__2 = *z__
                        .offset((i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize);
                    qmin = (if d__1 <= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                    d__1 = emax;
                    d__2 = *z__
                        .offset((i4 as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize);
                    emax = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                }
                d__1 = qmax;
                d__2 = *z__.offset((i4 as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize)
                    + *z__.offset((i4 as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize);
                qmax = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__1 = emin;
                d__2 = *z__.offset((i4 as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize);
                emin = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                i4 += -(4 as ::core::ffi::c_int) as ::core::ffi::c_long;
            }
            match current_block {
                9240481512215375588 => {
                    i4 = 4 as integer;
                }
                _ => {}
            }
            i0 = (i4 as ::core::ffi::c_long / 4 as ::core::ffi::c_long) as integer;
            pp = 0 as integer;
            if n0 - i0 > 1 as ::core::ffi::c_long {
                dee = *z__.offset(
                    (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 3 as ::core::ffi::c_long) as isize,
                );
                deemin = dee;
                kmin = i0;
                i__2 = (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - 3 as ::core::ffi::c_long) as integer;
                i4 = (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as integer;
                while i4 <= i__2 {
                    dee = *z__.offset(i4 as isize)
                        * (dee
                            / (dee
                                + *z__.offset(
                                    (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                                )));
                    if dee <= deemin {
                        deemin = dee;
                        kmin = ((i4 as ::core::ffi::c_long + 3 as ::core::ffi::c_long)
                            / 4 as ::core::ffi::c_long) as integer;
                    }
                    i4 += 4 as ::core::ffi::c_long;
                }
                if (kmin - i0 << 1 as ::core::ffi::c_int) < n0 - kmin
                    && deemin
                        <= *z__.offset(
                            (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 3 as ::core::ffi::c_long) as isize,
                        ) as ::core::ffi::c_double
                            * 0.5f64
                {
                    ipn4 = i0 + n0 << 2 as ::core::ffi::c_int;
                    pp = 2 as integer;
                    i__2 = ((i0 as ::core::ffi::c_long + n0 as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long)
                        << 1 as ::core::ffi::c_int) as integer;
                    i4 = i0 << 2 as ::core::ffi::c_int;
                    while i4 <= i__2 {
                        temp = *z__.offset(
                            (i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                        );
                        *z__.offset(
                            (i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                        ) = *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 3 as ::core::ffi::c_long) as isize,
                        );
                        *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 3 as ::core::ffi::c_long) as isize,
                        ) = temp;
                        temp = *z__.offset(
                            (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        );
                        *z__.offset(
                            (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        ) = *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 2 as ::core::ffi::c_long) as isize,
                        );
                        *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 2 as ::core::ffi::c_long) as isize,
                        ) = temp;
                        temp = *z__.offset(
                            (i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        );
                        *z__.offset(
                            (i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) = *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 5 as ::core::ffi::c_long) as isize,
                        );
                        *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 5 as ::core::ffi::c_long) as isize,
                        ) = temp;
                        temp = *z__.offset(i4 as isize);
                        *z__.offset(i4 as isize) = *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 4 as ::core::ffi::c_long) as isize,
                        );
                        *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - i4 as ::core::ffi::c_long
                                - 4 as ::core::ffi::c_long) as isize,
                        ) = temp;
                        i4 += 4 as ::core::ffi::c_long;
                    }
                }
            }
            d__1 = 0.0f64 as doublereal;
            d__2 = (qmin as ::core::ffi::c_double
                - super::lapack_sqrt(qmin) * 2.0f64 * super::lapack_sqrt(emax))
                as doublereal;
            dmin__ = -if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            } as doublereal;
            nbig = ((n0 as ::core::ffi::c_long - i0 as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long)
                * 30 as ::core::ffi::c_long) as integer;
            i__2 = nbig;
            iwhilb = 1 as integer;
            loop {
                if !(iwhilb <= i__2) {
                    current_block = 10528013381497917728;
                    break;
                }
                if i0 > n0 {
                    current_block = 16974974966130203269;
                    break;
                }
                dlasq3__0(
                    &raw mut i0,
                    &raw mut n0,
                    z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut pp,
                    &raw mut dmin__,
                    &raw mut sigma,
                    &raw mut desig,
                    &raw mut qmax,
                    &raw mut nfail,
                    &raw mut iter,
                    &raw mut ndiv,
                    &raw mut ieee,
                    &raw mut ttype,
                    &raw mut dmin1,
                    &raw mut dmin2,
                    &raw mut dn,
                    &raw mut dn1,
                    &raw mut dn2,
                    &raw mut g,
                    &raw mut tau,
                );
                pp = 1 as integer - pp;
                if pp == 0 as ::core::ffi::c_long && n0 - i0 >= 3 as ::core::ffi::c_long {
                    if *z__.offset((n0 as ::core::ffi::c_long * 4 as ::core::ffi::c_long) as isize)
                        <= tol2 * qmax
                        || *z__.offset(
                            (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        ) <= tol2 * sigma
                    {
                        splt = (i0 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        qmax = *z__.offset(
                            (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 3 as ::core::ffi::c_long) as isize,
                        );
                        emin = *z__.offset(
                            (((i0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        );
                        oldemn = *z__.offset(
                            (i0 as ::core::ffi::c_long * 4 as ::core::ffi::c_long) as isize,
                        );
                        i__3 = ((n0 as ::core::ffi::c_long - 3 as ::core::ffi::c_long)
                            << 2 as ::core::ffi::c_int) as integer;
                        i4 = i0 << 2 as ::core::ffi::c_int;
                        while i4 <= i__3 {
                            if *z__.offset(i4 as isize)
                                <= tol2
                                    * *z__.offset(
                                        (i4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long)
                                            as isize,
                                    )
                                || *z__.offset(
                                    (i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) <= tol2 * sigma
                            {
                                *z__.offset(
                                    (i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) = -sigma;
                                splt = (i4 as ::core::ffi::c_long / 4 as ::core::ffi::c_long)
                                    as integer;
                                qmax = 0.0f64 as doublereal;
                                emin = *z__.offset(
                                    (i4 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize,
                                );
                                oldemn = *z__.offset(
                                    (i4 as ::core::ffi::c_long + 4 as ::core::ffi::c_long) as isize,
                                );
                            } else {
                                d__1 = qmax;
                                d__2 = *z__.offset(
                                    (i4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                                );
                                qmax = (if d__1 >= d__2 {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    d__2 as ::core::ffi::c_double
                                }) as doublereal;
                                d__1 = emin;
                                d__2 = *z__.offset(
                                    (i4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                );
                                emin = (if d__1 <= d__2 {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    d__2 as ::core::ffi::c_double
                                }) as doublereal;
                                d__1 = oldemn;
                                d__2 = *z__.offset(i4 as isize);
                                oldemn = (if d__1 <= d__2 {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    d__2 as ::core::ffi::c_double
                                }) as doublereal;
                            }
                            i4 += 4 as ::core::ffi::c_long;
                        }
                        *z__.offset(
                            (((n0 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = emin;
                        *z__.offset(
                            (n0 as ::core::ffi::c_long * 4 as ::core::ffi::c_long) as isize,
                        ) = oldemn;
                        i0 = (splt as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    }
                }
                iwhilb += 1;
            }
            match current_block {
                16974974966130203269 => {
                    iwhila += 1;
                }
                _ => {
                    *info = 2 as integer;
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        match current_block {
            13067763561185533911 => {
                i__1 = *n;
                k = 2 as integer;
                while k <= i__1 {
                    *z__.offset(k as isize) = *z__.offset(
                        (((k as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                            - 3 as ::core::ffi::c_long) as isize,
                    );
                    k += 1;
                }
                dlasrt__0(
                    b"D\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut iinfo,
                );
                e = 0.0f64 as doublereal;
                k = *n;
                while k >= 1 as ::core::ffi::c_long {
                    e += *z__.offset(k as isize) as ::core::ffi::c_double;
                    k -= 1;
                }
                *z__.offset(
                    ((*n << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long) as isize,
                ) = trace;
                *z__.offset(
                    ((*n << 1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_long) as isize,
                ) = e;
                *z__.offset(
                    ((*n << 1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_long) as isize,
                ) = iter as doublereal;
                i__1 = *n;
                *z__.offset(
                    ((*n << 1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_long) as isize,
                ) = ndiv as doublereal / (i__1 * i__1) as doublereal;
                *z__.offset(
                    ((*n << 1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_long) as isize,
                ) = nfail as doublereal * 100.0f64 / iter as doublereal;
                return 0 as ::core::ffi::c_int;
            }
            _ => {
                *info = 3 as integer;
                return 0 as ::core::ffi::c_int;
            }
        };
    }
}
pub use raw_dlasq2::dgelsd_closure_dlasq2_;

pub mod raw_dlasq3 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasq3_(
        mut i0: *mut integer,
        mut n0: *mut integer,
        mut z__: *mut doublereal,
        mut pp: *mut integer,
        mut dmin__: *mut doublereal,
        mut sigma: *mut doublereal,
        mut desig: *mut doublereal,
        mut qmax: *mut doublereal,
        mut nfail: *mut integer,
        mut iter: *mut integer,
        mut ndiv: *mut integer,
        mut ieee: *mut logical,
        mut ttype: *mut integer,
        mut dmin1: *mut doublereal,
        mut dmin2: *mut doublereal,
        mut dn: *mut doublereal,
        mut dn1: *mut doublereal,
        mut dn2: *mut doublereal,
        mut g: *mut doublereal,
        mut tau: *mut doublereal,
    ) -> ::core::ffi::c_int {
        let mut current_block: u64;
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut s: doublereal = 0.;
        let mut t: doublereal = 0.;
        let mut j4: integer = 0;
        let mut nn: integer = 0;
        let mut eps: doublereal = 0.;
        let mut tol: doublereal = 0.;
        let mut n0in: integer = 0;
        let mut ipn4: integer = 0;
        let mut tol2: doublereal = 0.;
        let mut temp: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlasq4_"]
            fn dlasq4__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasq5_"]
            fn dlasq5__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut logical,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlasq6_"]
            fn dlasq6__0(
                _: *mut integer,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut integer,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
                _: *mut doublereal,
            ) -> ::core::ffi::c_int;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        extern "C" {
            #[link_name = "dgelsd_closure_disnan_"]
            fn disnan__0(_: *mut doublereal) -> logical;
        }
        z__ = z__.wrapping_offset(-1);
        n0in = *n0;
        eps = dlamch__0(
            b"Precision\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        tol = (eps as ::core::ffi::c_double * 100.0f64) as doublereal;
        d__1 = tol;
        tol2 = d__1 * d__1;
        loop {
            if *n0 < *i0 {
                return 0 as ::core::ffi::c_int;
            }
            if !(*n0 == *i0) {
                nn = (*n0 << 2 as ::core::ffi::c_int) + *pp;
                if *n0 == *i0 + 1 as ::core::ffi::c_long {
                    current_block = 1336937715023389256;
                } else if *z__
                    .offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                    > tol2
                        * (*sigma
                            + *z__.offset(
                                (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ))
                    && *z__.offset(
                        (nn as ::core::ffi::c_long
                            - (*pp << 1 as ::core::ffi::c_int)
                            - 4 as ::core::ffi::c_long) as isize,
                    ) > tol2
                        * *z__
                            .offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize)
                {
                    if *z__.offset((nn as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as isize)
                        > tol2 * *sigma
                        && *z__.offset(
                            (nn as ::core::ffi::c_long
                                - (*pp << 1 as ::core::ffi::c_int)
                                - 8 as ::core::ffi::c_long) as isize,
                        ) > tol2
                            * *z__.offset(
                                (nn as ::core::ffi::c_long - 11 as ::core::ffi::c_long) as isize,
                            )
                    {
                        break;
                    }
                    current_block = 1336937715023389256;
                } else {
                    current_block = 9457664389812342180;
                }
                match current_block {
                    9457664389812342180 => {}
                    _ => {
                        if *z__
                            .offset((nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize)
                            > *z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            )
                        {
                            s = *z__.offset(
                                (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            );
                            *z__.offset(
                                (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ) = *z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            );
                            *z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            ) = s;
                        }
                        if *z__
                            .offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                            > *z__.offset(
                                (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ) * tol2
                        {
                            t = ((*z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            ) as ::core::ffi::c_double
                                - *z__.offset(
                                    (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                                ) as ::core::ffi::c_double
                                + *z__.offset(
                                    (nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize,
                                ) as ::core::ffi::c_double)
                                * 0.5f64) as doublereal;
                            s = *z__.offset(
                                (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ) * (*z__.offset(
                                (nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize,
                            ) / t);
                            if s <= t {
                                s = (*z__.offset(
                                    (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                                ) as ::core::ffi::c_double
                                    * (*z__.offset(
                                        (nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long)
                                            as isize,
                                    )
                                        as ::core::ffi::c_double
                                        / (t as ::core::ffi::c_double
                                            * (super::lapack_sqrt(s / t + 1.0f64) + 1.0f64))))
                                    as doublereal;
                            } else {
                                s = (*z__.offset(
                                    (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                                ) as ::core::ffi::c_double
                                    * (*z__.offset(
                                        (nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long)
                                            as isize,
                                    )
                                        as ::core::ffi::c_double
                                        / (t as ::core::ffi::c_double
                                            + super::lapack_sqrt(t) * super::lapack_sqrt(t + s))))
                                    as doublereal;
                            }
                            t = *z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            ) + (s + *z__.offset(
                                (nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize,
                            ));
                            let ref mut fresh0 = *z__.offset(
                                (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            );
                            *fresh0 *= (*z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            ) / t) as ::core::ffi::c_double;
                            *z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            ) = t;
                        }
                        *z__.offset(
                            ((*n0 << 2 as ::core::ffi::c_int) - 7 as ::core::ffi::c_long) as isize,
                        ) = *z__.offset(
                            (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                        ) + *sigma;
                        *z__.offset(
                            ((*n0 << 2 as ::core::ffi::c_int) - 3 as ::core::ffi::c_long) as isize,
                        ) = *z__.offset(
                            (nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                        ) + *sigma;
                        *n0 += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
                        continue;
                    }
                }
            }
            *z__.offset(((*n0 << 2 as ::core::ffi::c_int) - 3 as ::core::ffi::c_long) as isize) =
                *z__.offset(
                    ((*n0 << 2 as ::core::ffi::c_int) + *pp - 3 as ::core::ffi::c_long) as isize,
                ) + *sigma;
            *n0 -= 1;
        }
        if *pp == 2 as ::core::ffi::c_long {
            *pp = 0 as integer;
        }
        if *dmin__ <= 0.0f64 || *n0 < n0in {
            if *z__.offset(
                ((*i0 << 2 as ::core::ffi::c_int) + *pp - 3 as ::core::ffi::c_long) as isize,
            ) as ::core::ffi::c_double
                * 1.5f64
                < *z__.offset(
                    ((*n0 << 2 as ::core::ffi::c_int) + *pp - 3 as ::core::ffi::c_long) as isize,
                )
            {
                ipn4 = *i0 + *n0 << 2 as ::core::ffi::c_int;
                i__1 =
                    ((*i0 + *n0 - 1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) as integer;
                j4 = *i0 << 2 as ::core::ffi::c_int;
                while j4 <= i__1 {
                    temp = *z__
                        .offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize);
                    *z__.offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize) =
                        *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - j4 as ::core::ffi::c_long
                                - 3 as ::core::ffi::c_long) as isize,
                        );
                    *z__.offset(
                        (ipn4 as ::core::ffi::c_long
                            - j4 as ::core::ffi::c_long
                            - 3 as ::core::ffi::c_long) as isize,
                    ) = temp;
                    temp = *z__
                        .offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize);
                    *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
                        *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - j4 as ::core::ffi::c_long
                                - 2 as ::core::ffi::c_long) as isize,
                        );
                    *z__.offset(
                        (ipn4 as ::core::ffi::c_long
                            - j4 as ::core::ffi::c_long
                            - 2 as ::core::ffi::c_long) as isize,
                    ) = temp;
                    temp = *z__
                        .offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                    *z__.offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                        *z__.offset(
                            (ipn4 as ::core::ffi::c_long
                                - j4 as ::core::ffi::c_long
                                - 5 as ::core::ffi::c_long) as isize,
                        );
                    *z__.offset(
                        (ipn4 as ::core::ffi::c_long
                            - j4 as ::core::ffi::c_long
                            - 5 as ::core::ffi::c_long) as isize,
                    ) = temp;
                    temp = *z__.offset(j4 as isize);
                    *z__.offset(j4 as isize) = *z__.offset(
                        (ipn4 as ::core::ffi::c_long
                            - j4 as ::core::ffi::c_long
                            - 4 as ::core::ffi::c_long) as isize,
                    );
                    *z__.offset(
                        (ipn4 as ::core::ffi::c_long
                            - j4 as ::core::ffi::c_long
                            - 4 as ::core::ffi::c_long) as isize,
                    ) = temp;
                    j4 += 4 as ::core::ffi::c_long;
                }
                if *n0 - *i0 <= 4 as ::core::ffi::c_long {
                    *z__.offset(
                        ((*n0 << 2 as ::core::ffi::c_int) + *pp - 1 as ::core::ffi::c_long)
                            as isize,
                    ) = *z__.offset(
                        ((*i0 << 2 as ::core::ffi::c_int) + *pp - 1 as ::core::ffi::c_long)
                            as isize,
                    );
                    *z__.offset(((*n0 << 2 as ::core::ffi::c_int) - *pp) as isize) =
                        *z__.offset(((*i0 << 2 as ::core::ffi::c_int) - *pp) as isize);
                }
                d__1 = *dmin2;
                d__2 = *z__.offset(
                    ((*n0 << 2 as ::core::ffi::c_int) + *pp - 1 as ::core::ffi::c_long) as isize,
                );
                *dmin2 = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__1 = *z__.offset(
                    ((*n0 << 2 as ::core::ffi::c_int) + *pp - 1 as ::core::ffi::c_long) as isize,
                );
                d__2 = *z__.offset(
                    ((*i0 << 2 as ::core::ffi::c_int) + *pp - 1 as ::core::ffi::c_long) as isize,
                );
                d__1 = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = *z__.offset(
                    ((*i0 << 2 as ::core::ffi::c_int) + *pp + 3 as ::core::ffi::c_long) as isize,
                );
                *z__.offset(
                    ((*n0 << 2 as ::core::ffi::c_int) + *pp - 1 as ::core::ffi::c_long) as isize,
                ) = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__1 = *z__.offset(((*n0 << 2 as ::core::ffi::c_int) - *pp) as isize);
                d__2 = *z__.offset(((*i0 << 2 as ::core::ffi::c_int) - *pp) as isize);
                d__1 = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = *z__.offset(
                    ((*i0 << 2 as ::core::ffi::c_int) - *pp + 4 as ::core::ffi::c_long) as isize,
                );
                *z__.offset(((*n0 << 2 as ::core::ffi::c_int) - *pp) as isize) = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                })
                    as doublereal;
                d__1 = *qmax;
                d__2 = *z__.offset(
                    ((*i0 << 2 as ::core::ffi::c_int) + *pp - 3 as ::core::ffi::c_long) as isize,
                );
                d__1 = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = *z__.offset(
                    ((*i0 << 2 as ::core::ffi::c_int) + *pp + 1 as ::core::ffi::c_long) as isize,
                );
                *qmax = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                *dmin__ = -0.0f64 as doublereal;
            }
        }
        dlasq4__0(
            i0,
            n0,
            z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            pp,
            &raw mut n0in,
            dmin__,
            dmin1,
            dmin2,
            dn,
            dn1,
            dn2,
            tau,
            ttype,
            g,
        );
        loop {
            dlasq5__0(
                i0,
                n0,
                z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                pp,
                tau,
                dmin__,
                dmin1,
                dmin2,
                dn,
                dn1,
                dn2,
                ieee,
            );
            *ndiv += *n0 - *i0 + 2 as ::core::ffi::c_long;
            *iter += 1;
            if *dmin__ >= 0.0f64 && *dmin1 > 0.0f64 {
                current_block = 15841802212995987424;
                break;
            }
            if *dmin__ < 0.0f64
                && *dmin1 > 0.0f64
                && *z__.offset((((*n0 - 1 as integer) << 2 as ::core::ffi::c_int) - *pp) as isize)
                    < tol * (*sigma + *dn1)
                && (if *dn >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    *dn
                } else {
                    -*dn
                }) < tol * *sigma
            {
                *z__.offset(
                    (((*n0 - 1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) - *pp
                        + 2 as ::core::ffi::c_long) as isize,
                ) = 0.0f64 as doublereal;
                *dmin__ = 0.0f64 as doublereal;
                current_block = 15841802212995987424;
                break;
            } else if *dmin__ < 0.0f64 {
                *nfail += 1;
                if *ttype < -(22 as ::core::ffi::c_int) as ::core::ffi::c_long {
                    *tau = 0.0f64 as doublereal;
                } else if *dmin1 > 0.0f64 {
                    *tau = ((*tau + *dmin__) * (1.0f64 - eps as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                    *ttype += -(11 as ::core::ffi::c_int) as ::core::ffi::c_long;
                } else {
                    *tau *= 0.25f64;
                    *ttype += -(12 as ::core::ffi::c_int) as ::core::ffi::c_long;
                }
            } else if disnan__0(dmin__) != 0 {
                if *tau == 0.0f64 {
                    current_block = 17722651503145564436;
                    break;
                }
                *tau = 0.0f64 as doublereal;
            } else {
                current_block = 17722651503145564436;
                break;
            }
        }
        match current_block {
            17722651503145564436 => {
                dlasq6__0(
                    i0,
                    n0,
                    z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    pp,
                    dmin__,
                    dmin1,
                    dmin2,
                    dn,
                    dn1,
                    dn2,
                );
                *ndiv += *n0 - *i0 + 2 as ::core::ffi::c_long;
                *iter += 1;
                *tau = 0.0f64 as doublereal;
            }
            _ => {}
        }
        if *tau < *sigma {
            *desig += *tau as ::core::ffi::c_double;
            t = *sigma + *desig;
            *desig -= (t - *sigma) as ::core::ffi::c_double;
        } else {
            t = *sigma + *tau;
            *desig = *sigma - (t - *tau) + *desig;
        }
        *sigma = t;
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasq3::dgelsd_closure_dlasq3_;

pub mod raw_dlasq4 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasq4_(
        mut i0: *mut integer,
        mut n0: *mut integer,
        mut z__: *mut doublereal,
        mut pp: *mut integer,
        mut n0in: *mut integer,
        mut dmin__: *mut doublereal,
        mut dmin1: *mut doublereal,
        mut dmin2: *mut doublereal,
        mut dn: *mut doublereal,
        mut dn1: *mut doublereal,
        mut dn2: *mut doublereal,
        mut tau: *mut doublereal,
        mut ttype: *mut integer,
        mut g: *mut doublereal,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut s: doublereal = 0.;
        let mut a2: doublereal = 0.;
        let mut b1: doublereal = 0.;
        let mut b2: doublereal = 0.;
        let mut i4: integer = 0;
        let mut nn: integer = 0;
        let mut np: integer = 0;
        let mut gam: doublereal = 0.;
        let mut gap1: doublereal = 0.;
        let mut gap2: doublereal = 0.;
        z__ = z__.wrapping_offset(-1);
        if *dmin__ <= 0.0f64 {
            *tau = -*dmin__;
            *ttype = -(1 as ::core::ffi::c_int) as integer;
            return 0 as ::core::ffi::c_int;
        }
        nn = (*n0 << 2 as ::core::ffi::c_int) + *pp;
        if *n0in == *n0 {
            if *dmin__ == *dn || *dmin__ == *dn1 {
                b1 = (super::lapack_sqrt(
                    *z__.offset((nn as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize),
                ) * super::lapack_sqrt(
                    *z__.offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize),
                )) as doublereal;
                b2 = (super::lapack_sqrt(
                    *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize),
                ) * super::lapack_sqrt(
                    *z__.offset((nn as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as isize),
                )) as doublereal;
                a2 = *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize)
                    + *z__.offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize);
                if *dmin__ == *dn && *dmin1 == *dn1 {
                    gap2 = (*dmin2 - a2 as ::core::ffi::c_double - *dmin2 * 0.25f64) as doublereal;
                    if gap2 > 0.0f64 && gap2 > b2 {
                        gap1 = a2 - *dn - b2 / gap2 * b2;
                    } else {
                        gap1 = a2 - *dn - (b1 + b2);
                    }
                    if gap1 > 0.0f64 && gap1 > b1 {
                        d__1 = *dn - b1 / gap1 * b1;
                        d__2 = (*dmin__ * 0.5f64) as doublereal;
                        s = (if d__1 >= d__2 {
                            d__1 as ::core::ffi::c_double
                        } else {
                            d__2 as ::core::ffi::c_double
                        }) as doublereal;
                        *ttype = -(2 as ::core::ffi::c_int) as integer;
                    } else {
                        s = 0.0f64 as doublereal;
                        if *dn > b1 {
                            s = *dn - b1;
                        }
                        if a2 > b1 + b2 {
                            d__1 = s;
                            d__2 = a2 - (b1 + b2);
                            s = (if d__1 <= d__2 {
                                d__1 as ::core::ffi::c_double
                            } else {
                                d__2 as ::core::ffi::c_double
                            }) as doublereal;
                        }
                        d__1 = s;
                        d__2 = (*dmin__ * 0.333f64) as doublereal;
                        s = (if d__1 >= d__2 {
                            d__1 as ::core::ffi::c_double
                        } else {
                            d__2 as ::core::ffi::c_double
                        }) as doublereal;
                        *ttype = -(3 as ::core::ffi::c_int) as integer;
                    }
                } else {
                    *ttype = -(4 as ::core::ffi::c_int) as integer;
                    s = (*dmin__ * 0.25f64) as doublereal;
                    if *dmin__ == *dn {
                        gam = *dn;
                        a2 = 0.0f64 as doublereal;
                        if *z__
                            .offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                            > *z__.offset(
                                (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                            )
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        b2 = *z__.offset(
                            (nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize,
                        ) / *z__.offset(
                            (nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize,
                        );
                        np = (nn as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as integer;
                    } else {
                        np = nn - (*pp << 1 as ::core::ffi::c_int);
                        b2 = *z__.offset(
                            (np as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        );
                        gam = *dn1;
                        if *z__
                            .offset((np as ::core::ffi::c_long - 4 as ::core::ffi::c_long) as isize)
                            > *z__.offset(
                                (np as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        a2 = *z__.offset(
                            (np as ::core::ffi::c_long - 4 as ::core::ffi::c_long) as isize,
                        ) / *z__.offset(
                            (np as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        );
                        if *z__
                            .offset((nn as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as isize)
                            > *z__.offset(
                                (nn as ::core::ffi::c_long - 11 as ::core::ffi::c_long) as isize,
                            )
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        b2 = *z__.offset(
                            (nn as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as isize,
                        ) / *z__.offset(
                            (nn as ::core::ffi::c_long - 11 as ::core::ffi::c_long) as isize,
                        );
                        np = (nn as ::core::ffi::c_long - 13 as ::core::ffi::c_long) as integer;
                    }
                    a2 += b2 as ::core::ffi::c_double;
                    i__1 = (*i0 << 2 as ::core::ffi::c_int) - 1 as integer + *pp;
                    i4 = np;
                    while i4 >= i__1 {
                        if b2 == 0.0f64 {
                            break;
                        }
                        b1 = b2;
                        if *z__.offset(i4 as isize)
                            > *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        b2 *= (*z__.offset(i4 as isize)
                            / *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )) as ::core::ffi::c_double;
                        a2 += b2 as ::core::ffi::c_double;
                        if (if b2 >= b1 {
                            b2 as ::core::ffi::c_double
                        } else {
                            b1 as ::core::ffi::c_double
                        }) * 100.0f64
                            < a2
                            || 0.563f64 < a2
                        {
                            break;
                        }
                        i4 += -(4 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    }
                    a2 *= 1.05f64;
                    if a2 < 0.563f64 {
                        s = (gam as ::core::ffi::c_double * (1.0f64 - super::lapack_sqrt(a2))
                            / (a2 as ::core::ffi::c_double + 1.0f64))
                            as doublereal;
                    }
                }
            } else if *dmin__ == *dn2 {
                *ttype = -(5 as ::core::ffi::c_int) as integer;
                s = (*dmin__ * 0.25f64) as doublereal;
                np = nn - (*pp << 1 as ::core::ffi::c_int);
                b1 = *z__.offset((np as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize);
                b2 = *z__.offset((np as ::core::ffi::c_long - 6 as ::core::ffi::c_long) as isize);
                gam = *dn2;
                if *z__.offset((np as ::core::ffi::c_long - 8 as ::core::ffi::c_long) as isize) > b2
                    || *z__.offset((np as ::core::ffi::c_long - 4 as ::core::ffi::c_long) as isize)
                        > b1
                {
                    return 0 as ::core::ffi::c_int;
                }
                a2 = (*z__.offset((np as ::core::ffi::c_long - 8 as ::core::ffi::c_long) as isize)
                    as ::core::ffi::c_double
                    / b2 as ::core::ffi::c_double
                    * (*z__.offset((np as ::core::ffi::c_long - 4 as ::core::ffi::c_long) as isize)
                        as ::core::ffi::c_double
                        / b1 as ::core::ffi::c_double
                        + 1.0f64)) as doublereal;
                if *n0 - *i0 > 2 as ::core::ffi::c_long {
                    b2 = *z__
                        .offset((nn as ::core::ffi::c_long - 13 as ::core::ffi::c_long) as isize)
                        / *z__.offset(
                            (nn as ::core::ffi::c_long - 15 as ::core::ffi::c_long) as isize,
                        );
                    a2 += b2 as ::core::ffi::c_double;
                    i__1 = (*i0 << 2 as ::core::ffi::c_int) - 1 as integer + *pp;
                    i4 = (nn as ::core::ffi::c_long - 17 as ::core::ffi::c_long) as integer;
                    while i4 >= i__1 {
                        if b2 == 0.0f64 {
                            break;
                        }
                        b1 = b2;
                        if *z__.offset(i4 as isize)
                            > *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        b2 *= (*z__.offset(i4 as isize)
                            / *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )) as ::core::ffi::c_double;
                        a2 += b2 as ::core::ffi::c_double;
                        if (if b2 >= b1 {
                            b2 as ::core::ffi::c_double
                        } else {
                            b1 as ::core::ffi::c_double
                        }) * 100.0f64
                            < a2
                            || 0.563f64 < a2
                        {
                            break;
                        }
                        i4 += -(4 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    }
                    a2 *= 1.05f64;
                }
                if a2 < 0.563f64 {
                    s = (gam as ::core::ffi::c_double * (1.0f64 - super::lapack_sqrt(a2))
                        / (a2 as ::core::ffi::c_double + 1.0f64))
                        as doublereal;
                }
            } else {
                if *ttype == -(6 as ::core::ffi::c_int) as ::core::ffi::c_long {
                    *g += (1.0f64 - *g) * 0.333f64;
                } else if *ttype == -(18 as ::core::ffi::c_int) as ::core::ffi::c_long {
                    *g = 0.083250000000000005f64 as doublereal;
                } else {
                    *g = 0.25f64 as doublereal;
                }
                s = *g * *dmin__;
                *ttype = -(6 as ::core::ffi::c_int) as integer;
            }
        } else if *n0in == *n0 + 1 as ::core::ffi::c_long {
            if *dmin1 == *dn1 && *dmin2 == *dn2 {
                *ttype = -(7 as ::core::ffi::c_int) as integer;
                s = (*dmin1 * 0.333f64) as doublereal;
                if *z__.offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                    > *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize)
                {
                    return 0 as ::core::ffi::c_int;
                }
                b1 = *z__.offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                    / *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize);
                b2 = b1;
                if !(b2 == 0.0f64) {
                    i__1 = (*i0 << 2 as ::core::ffi::c_int) - 1 as integer + *pp;
                    i4 = (*n0 << 2 as ::core::ffi::c_int) - 9 as integer + *pp;
                    while i4 >= i__1 {
                        a2 = b1;
                        if *z__.offset(i4 as isize)
                            > *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        b1 *= (*z__.offset(i4 as isize)
                            / *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )) as ::core::ffi::c_double;
                        b2 += b1 as ::core::ffi::c_double;
                        if (if b1 >= a2 {
                            b1 as ::core::ffi::c_double
                        } else {
                            a2 as ::core::ffi::c_double
                        }) * 100.0f64
                            < b2
                        {
                            break;
                        }
                        i4 += -(4 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    }
                }
                b2 = super::lapack_sqrt(b2 * 1.05f64) as doublereal;
                d__1 = b2;
                a2 = (*dmin1
                    / (d__1 as ::core::ffi::c_double * d__1 as ::core::ffi::c_double + 1.0f64))
                    as doublereal;
                gap2 = *dmin2 * 0.5f64 - a2;
                if gap2 > 0.0f64 && gap2 > b2 * a2 {
                    d__1 = s;
                    d__2 = a2 * (1.0f64 - a2 * 1.01f64 * (b2 / gap2) * b2);
                    s = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                } else {
                    d__1 = s;
                    d__2 = (a2 as ::core::ffi::c_double
                        * (1.0f64 - b2 as ::core::ffi::c_double * 1.01f64))
                        as doublereal;
                    s = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                    *ttype = -(8 as ::core::ffi::c_int) as integer;
                }
            } else {
                s = (*dmin1 * 0.25f64) as doublereal;
                if *dmin1 == *dn1 {
                    s = (*dmin1 * 0.5f64) as doublereal;
                }
                *ttype = -(9 as ::core::ffi::c_int) as integer;
            }
        } else if *n0in == *n0 + 2 as ::core::ffi::c_long {
            if *dmin2 == *dn2
                && *z__.offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                    as ::core::ffi::c_double
                    * 2.0f64
                    < *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize)
            {
                *ttype = -(10 as ::core::ffi::c_int) as integer;
                s = (*dmin2 * 0.333f64) as doublereal;
                if *z__.offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                    > *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize)
                {
                    return 0 as ::core::ffi::c_int;
                }
                b1 = *z__.offset((nn as ::core::ffi::c_long - 5 as ::core::ffi::c_long) as isize)
                    / *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize);
                b2 = b1;
                if !(b2 == 0.0f64) {
                    i__1 = (*i0 << 2 as ::core::ffi::c_int) - 1 as integer + *pp;
                    i4 = (*n0 << 2 as ::core::ffi::c_int) - 9 as integer + *pp;
                    while i4 >= i__1 {
                        if *z__.offset(i4 as isize)
                            > *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        b1 *= (*z__.offset(i4 as isize)
                            / *z__.offset(
                                (i4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            )) as ::core::ffi::c_double;
                        b2 += b1 as ::core::ffi::c_double;
                        if b1 as ::core::ffi::c_double * 100.0f64 < b2 {
                            break;
                        }
                        i4 += -(4 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    }
                }
                b2 = super::lapack_sqrt(b2 * 1.05f64) as doublereal;
                d__1 = b2;
                a2 = (*dmin2
                    / (d__1 as ::core::ffi::c_double * d__1 as ::core::ffi::c_double + 1.0f64))
                    as doublereal;
                gap2 = *z__.offset((nn as ::core::ffi::c_long - 7 as ::core::ffi::c_long) as isize)
                    + *z__.offset((nn as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as isize)
                    - super::lapack_sqrt(
                        *z__.offset(
                            (nn as ::core::ffi::c_long - 11 as ::core::ffi::c_long) as isize,
                        ),
                    ) as doublereal
                        * super::lapack_sqrt(*z__.offset(
                            (nn as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as isize,
                        )) as doublereal
                    - a2;
                if gap2 > 0.0f64 && gap2 > b2 * a2 {
                    d__1 = s;
                    d__2 = a2 * (1.0f64 - a2 * 1.01f64 * (b2 / gap2) * b2);
                    s = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                } else {
                    d__1 = s;
                    d__2 = (a2 as ::core::ffi::c_double
                        * (1.0f64 - b2 as ::core::ffi::c_double * 1.01f64))
                        as doublereal;
                    s = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                }
            } else {
                s = (*dmin2 * 0.25f64) as doublereal;
                *ttype = -(11 as ::core::ffi::c_int) as integer;
            }
        } else if *n0in > *n0 + 2 as ::core::ffi::c_long {
            s = 0.0f64 as doublereal;
            *ttype = -(12 as ::core::ffi::c_int) as integer;
        }
        *tau = s;
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasq4::dgelsd_closure_dlasq4_;

pub mod raw_dlasq5 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasq5_(
        mut i0: *mut integer,
        mut n0: *mut integer,
        mut z__: *mut doublereal,
        mut pp: *mut integer,
        mut tau: *mut doublereal,
        mut dmin__: *mut doublereal,
        mut dmin1: *mut doublereal,
        mut dmin2: *mut doublereal,
        mut dn: *mut doublereal,
        mut dnm1: *mut doublereal,
        mut dnm2: *mut doublereal,
        mut ieee: *mut logical,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut d__: doublereal = 0.;
        let mut j4: integer = 0;
        let mut j4p2: integer = 0;
        let mut emin: doublereal = 0.;
        let mut temp: doublereal = 0.;
        z__ = z__.wrapping_offset(-1);
        if *n0 - *i0 - 1 as ::core::ffi::c_long <= 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        j4 = ((*i0 << 2 as ::core::ffi::c_int) + *pp - 3 as ::core::ffi::c_long) as integer;
        emin = *z__.offset((j4 as ::core::ffi::c_long + 4 as ::core::ffi::c_long) as isize);
        d__ = *z__.offset(j4 as isize) - *tau;
        *dmin__ = d__;
        *dmin1 = -*z__.offset(j4 as isize);
        if *ieee != 0 {
            if *pp == 0 as ::core::ffi::c_long {
                i__1 = ((*n0 - 3 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) as integer;
                j4 = *i0 << 2 as ::core::ffi::c_int;
                while j4 <= i__1 {
                    *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
                        d__ + *z__.offset(
                            (j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        );
                    temp = *z__
                        .offset((j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        );
                    d__ = d__ * temp - *tau;
                    *dmin__ = (if *dmin__ <= d__ {
                        *dmin__
                    } else {
                        d__ as ::core::ffi::c_double
                    }) as doublereal;
                    *z__.offset(j4 as isize) = *z__
                        .offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                        * temp;
                    d__1 = *z__.offset(j4 as isize);
                    emin = (if d__1 <= emin {
                        d__1 as ::core::ffi::c_double
                    } else {
                        emin as ::core::ffi::c_double
                    }) as doublereal;
                    j4 += 4 as ::core::ffi::c_long;
                }
            } else {
                i__1 = ((*n0 - 3 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) as integer;
                j4 = *i0 << 2 as ::core::ffi::c_int;
                while j4 <= i__1 {
                    *z__.offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize) =
                        d__ + *z__.offset(j4 as isize);
                    temp = *z__
                        .offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                        );
                    d__ = d__ * temp - *tau;
                    *dmin__ = (if *dmin__ <= d__ {
                        *dmin__
                    } else {
                        d__ as ::core::ffi::c_double
                    }) as doublereal;
                    *z__.offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                        *z__.offset(j4 as isize) * temp;
                    d__1 = *z__
                        .offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                    emin = (if d__1 <= emin {
                        d__1 as ::core::ffi::c_double
                    } else {
                        emin as ::core::ffi::c_double
                    }) as doublereal;
                    j4 += 4 as ::core::ffi::c_long;
                }
            }
            *dnm2 = d__;
            *dmin2 = *dmin__;
            j4 = ((*n0 - 2 as integer) << 2 as ::core::ffi::c_int) - *pp;
            j4p2 = (j4 as ::core::ffi::c_long + (*pp << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as integer;
            *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
                *dnm2 + *z__.offset(j4p2 as isize);
            *z__.offset(j4 as isize) = *z__
                .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*z__.offset(j4p2 as isize)
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize));
            *dnm1 = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*dnm2
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize))
                - *tau;
            *dmin__ = (if *dmin__ <= *dnm1 { *dmin__ } else { *dnm1 }) as doublereal;
            *dmin1 = *dmin__;
            j4 += 4 as ::core::ffi::c_long;
            j4p2 = (j4 as ::core::ffi::c_long + (*pp << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as integer;
            *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
                *dnm1 + *z__.offset(j4p2 as isize);
            *z__.offset(j4 as isize) = *z__
                .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*z__.offset(j4p2 as isize)
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize));
            *dn = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*dnm1
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize))
                - *tau;
            *dmin__ = (if *dmin__ <= *dn { *dmin__ } else { *dn }) as doublereal;
        } else {
            if *pp == 0 as ::core::ffi::c_long {
                i__1 = ((*n0 - 3 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) as integer;
                j4 = *i0 << 2 as ::core::ffi::c_int;
                while j4 <= i__1 {
                    *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
                        d__ + *z__.offset(
                            (j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        );
                    if d__ < 0.0f64 {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        *z__.offset(j4 as isize) = *z__.offset(
                            (j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        ) * (*z__.offset(
                            (j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        ));
                        d__ = *z__.offset(
                            (j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        ) * (d__
                            / *z__.offset(
                                (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            ))
                            - *tau;
                    }
                    *dmin__ = (if *dmin__ <= d__ {
                        *dmin__
                    } else {
                        d__ as ::core::ffi::c_double
                    }) as doublereal;
                    d__1 = emin;
                    d__2 = *z__.offset(j4 as isize);
                    emin = (if d__1 <= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                    j4 += 4 as ::core::ffi::c_long;
                }
            } else {
                i__1 = ((*n0 - 3 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) as integer;
                j4 = *i0 << 2 as ::core::ffi::c_int;
                while j4 <= i__1 {
                    *z__.offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize) =
                        d__ + *z__.offset(j4 as isize);
                    if d__ < 0.0f64 {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        *z__.offset(
                            (j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) = *z__.offset(
                            (j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                        ) * (*z__.offset(j4 as isize)
                            / *z__.offset(
                                (j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ));
                        d__ = *z__.offset(
                            (j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                        ) * (d__
                            / *z__.offset(
                                (j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ))
                            - *tau;
                    }
                    *dmin__ = (if *dmin__ <= d__ {
                        *dmin__
                    } else {
                        d__ as ::core::ffi::c_double
                    }) as doublereal;
                    d__1 = emin;
                    d__2 = *z__
                        .offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                    emin = (if d__1 <= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                    j4 += 4 as ::core::ffi::c_long;
                }
            }
            *dnm2 = d__;
            *dmin2 = *dmin__;
            j4 = ((*n0 - 2 as integer) << 2 as ::core::ffi::c_int) - *pp;
            j4p2 = (j4 as ::core::ffi::c_long + (*pp << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as integer;
            *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
                *dnm2 + *z__.offset(j4p2 as isize);
            if *dnm2 < 0.0f64 {
                return 0 as ::core::ffi::c_int;
            } else {
                *z__.offset(j4 as isize) = *z__
                    .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    * (*z__.offset(j4p2 as isize)
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        ));
                *dnm1 = *z__
                    .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    * (*dnm2
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        ))
                    - *tau;
            }
            *dmin__ = (if *dmin__ <= *dnm1 { *dmin__ } else { *dnm1 }) as doublereal;
            *dmin1 = *dmin__;
            j4 += 4 as ::core::ffi::c_long;
            j4p2 = (j4 as ::core::ffi::c_long + (*pp << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as integer;
            *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
                *dnm1 + *z__.offset(j4p2 as isize);
            if *dnm1 < 0.0f64 {
                return 0 as ::core::ffi::c_int;
            } else {
                *z__.offset(j4 as isize) = *z__
                    .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    * (*z__.offset(j4p2 as isize)
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        ));
                *dn = *z__
                    .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    * (*dnm1
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        ))
                    - *tau;
            }
            *dmin__ = (if *dmin__ <= *dn { *dmin__ } else { *dn }) as doublereal;
        }
        *z__.offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) = *dn;
        *z__.offset(((*n0 << 2 as ::core::ffi::c_int) - *pp) as isize) = emin;
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasq5::dgelsd_closure_dlasq5_;

pub mod raw_dlasq6 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasq6_(
        mut i0: *mut integer,
        mut n0: *mut integer,
        mut z__: *mut doublereal,
        mut pp: *mut integer,
        mut dmin__: *mut doublereal,
        mut dmin1: *mut doublereal,
        mut dmin2: *mut doublereal,
        mut dn: *mut doublereal,
        mut dnm1: *mut doublereal,
        mut dnm2: *mut doublereal,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        let mut d__: doublereal = 0.;
        let mut j4: integer = 0;
        let mut j4p2: integer = 0;
        let mut emin: doublereal = 0.;
        let mut temp: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        let mut safmin: doublereal = 0.;
        z__ = z__.wrapping_offset(-1);
        if *n0 - *i0 - 1 as ::core::ffi::c_long <= 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        safmin = dlamch__0(
            b"Safe minimum\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        j4 = ((*i0 << 2 as ::core::ffi::c_int) + *pp - 3 as ::core::ffi::c_long) as integer;
        emin = *z__.offset((j4 as ::core::ffi::c_long + 4 as ::core::ffi::c_long) as isize);
        d__ = *z__.offset(j4 as isize);
        *dmin__ = d__;
        if *pp == 0 as ::core::ffi::c_long {
            i__1 = ((*n0 - 3 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) as integer;
            j4 = *i0 << 2 as ::core::ffi::c_int;
            while j4 <= i__1 {
                *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) = d__
                    + *z__.offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                if *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize)
                    == 0.0f64
                {
                    *z__.offset(j4 as isize) = 0.0f64 as doublereal;
                    d__ = *z__
                        .offset((j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                    *dmin__ = d__;
                    emin = 0.0f64 as doublereal;
                } else if safmin
                    * *z__.offset((j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    < *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize)
                    && safmin
                        * *z__
                            .offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize)
                        < *z__
                            .offset((j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                {
                    temp = *z__
                        .offset((j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        );
                    *z__.offset(j4 as isize) = *z__
                        .offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                        * temp;
                    d__ *= temp as ::core::ffi::c_double;
                } else {
                    *z__.offset(j4 as isize) = *z__
                        .offset((j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        * (*z__.offset(
                            (j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) / *z__.offset(
                            (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                        ));
                    d__ = *z__
                        .offset((j4 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        * (d__
                            / *z__.offset(
                                (j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            ));
                }
                *dmin__ = (if *dmin__ <= d__ {
                    *dmin__
                } else {
                    d__ as ::core::ffi::c_double
                }) as doublereal;
                d__1 = emin;
                d__2 = *z__.offset(j4 as isize);
                emin = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                j4 += 4 as ::core::ffi::c_long;
            }
        } else {
            i__1 = ((*n0 - 3 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int) as integer;
            j4 = *i0 << 2 as ::core::ffi::c_int;
            while j4 <= i__1 {
                *z__.offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize) =
                    d__ + *z__.offset(j4 as isize);
                if *z__.offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize)
                    == 0.0f64
                {
                    *z__.offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                        0.0f64 as doublereal;
                    d__ = *z__
                        .offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                    *dmin__ = d__;
                    emin = 0.0f64 as doublereal;
                } else if safmin
                    * *z__.offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    < *z__.offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize)
                    && safmin
                        * *z__
                            .offset((j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize)
                        < *z__
                            .offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                {
                    temp = *z__
                        .offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                        / *z__.offset(
                            (j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                        );
                    *z__.offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                        *z__.offset(j4 as isize) * temp;
                    d__ *= temp as ::core::ffi::c_double;
                } else {
                    *z__.offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                        *z__.offset(
                            (j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                        ) * (*z__.offset(j4 as isize)
                            / *z__.offset(
                                (j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ));
                    d__ = *z__
                        .offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                        * (d__
                            / *z__.offset(
                                (j4 as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as isize,
                            ));
                }
                *dmin__ = (if *dmin__ <= d__ {
                    *dmin__
                } else {
                    d__ as ::core::ffi::c_double
                }) as doublereal;
                d__1 = emin;
                d__2 = *z__.offset((j4 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                emin = (if d__1 <= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                j4 += 4 as ::core::ffi::c_long;
            }
        }
        *dnm2 = d__;
        *dmin2 = *dmin__;
        j4 = ((*n0 - 2 as integer) << 2 as ::core::ffi::c_int) - *pp;
        j4p2 = (j4 as ::core::ffi::c_long + (*pp << 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_long) as integer;
        *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
            *dnm2 + *z__.offset(j4p2 as isize);
        if *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) == 0.0f64 {
            *z__.offset(j4 as isize) = 0.0f64 as doublereal;
            *dnm1 = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            *dmin__ = *dnm1;
            emin = 0.0f64 as doublereal;
        } else if safmin
            * *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            < *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize)
            && safmin * *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize)
                < *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
        {
            temp = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize);
            *z__.offset(j4 as isize) = *z__.offset(j4p2 as isize) * temp;
            *dnm1 = *dnm2 * temp;
        } else {
            *z__.offset(j4 as isize) = *z__
                .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*z__.offset(j4p2 as isize)
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize));
            *dnm1 = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*dnm2
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize));
        }
        *dmin__ = (if *dmin__ <= *dnm1 { *dmin__ } else { *dnm1 }) as doublereal;
        *dmin1 = *dmin__;
        j4 += 4 as ::core::ffi::c_long;
        j4p2 = (j4 as ::core::ffi::c_long + (*pp << 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_long) as integer;
        *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) =
            *dnm1 + *z__.offset(j4p2 as isize);
        if *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize) == 0.0f64 {
            *z__.offset(j4 as isize) = 0.0f64 as doublereal;
            *dn = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            *dmin__ = *dn;
            emin = 0.0f64 as doublereal;
        } else if safmin
            * *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            < *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize)
            && safmin * *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize)
                < *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
        {
            temp = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize);
            *z__.offset(j4 as isize) = *z__.offset(j4p2 as isize) * temp;
            *dn = *dnm1 * temp;
        } else {
            *z__.offset(j4 as isize) = *z__
                .offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*z__.offset(j4p2 as isize)
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize));
            *dn = *z__.offset((j4p2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                * (*dnm1
                    / *z__.offset((j4 as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize));
        }
        *dmin__ = (if *dmin__ <= *dn { *dmin__ } else { *dn }) as doublereal;
        *z__.offset((j4 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) = *dn;
        *z__.offset(((*n0 << 2 as ::core::ffi::c_int) - *pp) as isize) = emin;
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasq6::dgelsd_closure_dlasq6_;

pub mod raw_dlasr {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasr_(
        mut side: *mut ::core::ffi::c_char,
        mut pivot: *mut ::core::ffi::c_char,
        mut direct: *mut ::core::ffi::c_char,
        mut m: *mut integer,
        mut n: *mut integer,
        mut c__: *mut doublereal,
        mut s: *mut doublereal,
        mut a: *mut doublereal,
        mut lda: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut a_dim1: integer = 0;
        let mut a_offset: integer = 0;
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut info: integer = 0;
        let mut temp: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_lsame_"]
            fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
        }
        let mut ctemp: doublereal = 0.;
        let mut stemp: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        c__ = c__.wrapping_offset(-1);
        s = s.wrapping_offset(-1);
        a_dim1 = *lda;
        a_offset = 1 as integer + a_dim1;
        a = a.wrapping_offset(-(a_offset as isize));
        info = 0 as integer;
        if !(lsame__0(
            side,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
            || lsame__0(
                side,
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0)
        {
            info = 1 as integer;
        } else if !(lsame__0(
            pivot,
            b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
            || lsame__0(
                pivot,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            || lsame__0(
                pivot,
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0)
        {
            info = 2 as integer;
        } else if !(lsame__0(
            direct,
            b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
            || lsame__0(
                direct,
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0)
        {
            info = 3 as integer;
        } else if *m < 0 as ::core::ffi::c_long {
            info = 4 as integer;
        } else if *n < 0 as ::core::ffi::c_long {
            info = 5 as integer;
        } else if *lda
            < (if 1 as ::core::ffi::c_long >= *m {
                1 as ::core::ffi::c_long
            } else {
                *m
            })
        {
            info = 9 as integer;
        }
        if info != 0 as ::core::ffi::c_long {
            xerbla__0(
                b"DLASR \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut info,
            );
            return 0 as ::core::ffi::c_int;
        }
        if *m == 0 as ::core::ffi::c_long || *n == 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        if lsame__0(
            side,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            if lsame__0(
                pivot,
                b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                if lsame__0(
                    direct,
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
                    j = 1 as integer;
                    while j <= i__1 {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__2 = *n;
                            i__ = 1 as integer;
                            while i__ <= i__2 {
                                temp = *a.offset((j + 1 as integer + i__ * a_dim1) as isize);
                                *a.offset((j + 1 as integer + i__ * a_dim1) as isize) =
                                    ctemp * temp - stemp * *a.offset((j + i__ * a_dim1) as isize);
                                *a.offset((j + i__ * a_dim1) as isize) =
                                    stemp * temp + ctemp * *a.offset((j + i__ * a_dim1) as isize);
                                i__ += 1;
                            }
                        }
                        j += 1;
                    }
                } else if lsame__0(
                    direct,
                    b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    j = (*m - 1 as ::core::ffi::c_long) as integer;
                    while j >= 1 as ::core::ffi::c_long {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__1 = *n;
                            i__ = 1 as integer;
                            while i__ <= i__1 {
                                temp = *a.offset((j + 1 as integer + i__ * a_dim1) as isize);
                                *a.offset((j + 1 as integer + i__ * a_dim1) as isize) =
                                    ctemp * temp - stemp * *a.offset((j + i__ * a_dim1) as isize);
                                *a.offset((j + i__ * a_dim1) as isize) =
                                    stemp * temp + ctemp * *a.offset((j + i__ * a_dim1) as isize);
                                i__ += 1;
                            }
                        }
                        j -= 1;
                    }
                }
            } else if lsame__0(
                pivot,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                if lsame__0(
                    direct,
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    i__1 = *m;
                    j = 2 as integer;
                    while j <= i__1 {
                        ctemp = *c__
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        stemp = *s
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__2 = *n;
                            i__ = 1 as integer;
                            while i__ <= i__2 {
                                temp = *a.offset((j + i__ * a_dim1) as isize);
                                *a.offset((j + i__ * a_dim1) as isize) = ctemp * temp
                                    - stemp
                                        * *a.offset(
                                            (i__ as ::core::ffi::c_long
                                                * a_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                *a.offset(
                                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = stemp * temp
                                    + ctemp
                                        * *a.offset(
                                            (i__ as ::core::ffi::c_long
                                                * a_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                i__ += 1;
                            }
                        }
                        j += 1;
                    }
                } else if lsame__0(
                    direct,
                    b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    j = *m;
                    while j >= 2 as ::core::ffi::c_long {
                        ctemp = *c__
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        stemp = *s
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__1 = *n;
                            i__ = 1 as integer;
                            while i__ <= i__1 {
                                temp = *a.offset((j + i__ * a_dim1) as isize);
                                *a.offset((j + i__ * a_dim1) as isize) = ctemp * temp
                                    - stemp
                                        * *a.offset(
                                            (i__ as ::core::ffi::c_long
                                                * a_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                *a.offset(
                                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = stemp * temp
                                    + ctemp
                                        * *a.offset(
                                            (i__ as ::core::ffi::c_long
                                                * a_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                i__ += 1;
                            }
                        }
                        j -= 1;
                    }
                }
            } else if lsame__0(
                pivot,
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                if lsame__0(
                    direct,
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
                    j = 1 as integer;
                    while j <= i__1 {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__2 = *n;
                            i__ = 1 as integer;
                            while i__ <= i__2 {
                                temp = *a.offset((j + i__ * a_dim1) as isize);
                                *a.offset((j + i__ * a_dim1) as isize) =
                                    stemp * *a.offset((*m + i__ * a_dim1) as isize) + ctemp * temp;
                                *a.offset((*m + i__ * a_dim1) as isize) =
                                    ctemp * *a.offset((*m + i__ * a_dim1) as isize) - stemp * temp;
                                i__ += 1;
                            }
                        }
                        j += 1;
                    }
                } else if lsame__0(
                    direct,
                    b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    j = (*m - 1 as ::core::ffi::c_long) as integer;
                    while j >= 1 as ::core::ffi::c_long {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__1 = *n;
                            i__ = 1 as integer;
                            while i__ <= i__1 {
                                temp = *a.offset((j + i__ * a_dim1) as isize);
                                *a.offset((j + i__ * a_dim1) as isize) =
                                    stemp * *a.offset((*m + i__ * a_dim1) as isize) + ctemp * temp;
                                *a.offset((*m + i__ * a_dim1) as isize) =
                                    ctemp * *a.offset((*m + i__ * a_dim1) as isize) - stemp * temp;
                                i__ += 1;
                            }
                        }
                        j -= 1;
                    }
                }
            }
        } else if lsame__0(
            side,
            b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            if lsame__0(
                pivot,
                b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                if lsame__0(
                    direct,
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                    j = 1 as integer;
                    while j <= i__1 {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__2 = *m;
                            i__ = 1 as integer;
                            while i__ <= i__2 {
                                temp = *a.offset((i__ + (j + 1 as integer) * a_dim1) as isize);
                                *a.offset((i__ + (j + 1 as integer) * a_dim1) as isize) =
                                    ctemp * temp - stemp * *a.offset((i__ + j * a_dim1) as isize);
                                *a.offset((i__ + j * a_dim1) as isize) =
                                    stemp * temp + ctemp * *a.offset((i__ + j * a_dim1) as isize);
                                i__ += 1;
                            }
                        }
                        j += 1;
                    }
                } else if lsame__0(
                    direct,
                    b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    j = (*n - 1 as ::core::ffi::c_long) as integer;
                    while j >= 1 as ::core::ffi::c_long {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__1 = *m;
                            i__ = 1 as integer;
                            while i__ <= i__1 {
                                temp = *a.offset((i__ + (j + 1 as integer) * a_dim1) as isize);
                                *a.offset((i__ + (j + 1 as integer) * a_dim1) as isize) =
                                    ctemp * temp - stemp * *a.offset((i__ + j * a_dim1) as isize);
                                *a.offset((i__ + j * a_dim1) as isize) =
                                    stemp * temp + ctemp * *a.offset((i__ + j * a_dim1) as isize);
                                i__ += 1;
                            }
                        }
                        j -= 1;
                    }
                }
            } else if lsame__0(
                pivot,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                if lsame__0(
                    direct,
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    i__1 = *n;
                    j = 2 as integer;
                    while j <= i__1 {
                        ctemp = *c__
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        stemp = *s
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__2 = *m;
                            i__ = 1 as integer;
                            while i__ <= i__2 {
                                temp = *a.offset((i__ + j * a_dim1) as isize);
                                *a.offset((i__ + j * a_dim1) as isize) =
                                    ctemp * temp - stemp * *a.offset((i__ + a_dim1) as isize);
                                *a.offset((i__ + a_dim1) as isize) =
                                    stemp * temp + ctemp * *a.offset((i__ + a_dim1) as isize);
                                i__ += 1;
                            }
                        }
                        j += 1;
                    }
                } else if lsame__0(
                    direct,
                    b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    j = *n;
                    while j >= 2 as ::core::ffi::c_long {
                        ctemp = *c__
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        stemp = *s
                            .offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__1 = *m;
                            i__ = 1 as integer;
                            while i__ <= i__1 {
                                temp = *a.offset((i__ + j * a_dim1) as isize);
                                *a.offset((i__ + j * a_dim1) as isize) =
                                    ctemp * temp - stemp * *a.offset((i__ + a_dim1) as isize);
                                *a.offset((i__ + a_dim1) as isize) =
                                    stemp * temp + ctemp * *a.offset((i__ + a_dim1) as isize);
                                i__ += 1;
                            }
                        }
                        j -= 1;
                    }
                }
            } else if lsame__0(
                pivot,
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                if lsame__0(
                    direct,
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                    j = 1 as integer;
                    while j <= i__1 {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__2 = *m;
                            i__ = 1 as integer;
                            while i__ <= i__2 {
                                temp = *a.offset((i__ + j * a_dim1) as isize);
                                *a.offset((i__ + j * a_dim1) as isize) =
                                    stemp * *a.offset((i__ + *n * a_dim1) as isize) + ctemp * temp;
                                *a.offset((i__ + *n * a_dim1) as isize) =
                                    ctemp * *a.offset((i__ + *n * a_dim1) as isize) - stemp * temp;
                                i__ += 1;
                            }
                        }
                        j += 1;
                    }
                } else if lsame__0(
                    direct,
                    b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    j = (*n - 1 as ::core::ffi::c_long) as integer;
                    while j >= 1 as ::core::ffi::c_long {
                        ctemp = *c__.offset(j as isize);
                        stemp = *s.offset(j as isize);
                        if ctemp != 1.0f64 || stemp != 0.0f64 {
                            i__1 = *m;
                            i__ = 1 as integer;
                            while i__ <= i__1 {
                                temp = *a.offset((i__ + j * a_dim1) as isize);
                                *a.offset((i__ + j * a_dim1) as isize) =
                                    stemp * *a.offset((i__ + *n * a_dim1) as isize) + ctemp * temp;
                                *a.offset((i__ + *n * a_dim1) as isize) =
                                    ctemp * *a.offset((i__ + *n * a_dim1) as isize) - stemp * temp;
                                i__ += 1;
                            }
                        }
                        j -= 1;
                    }
                }
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasr::dgelsd_closure_dlasr_;

pub mod raw_dlasrt {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasrt_(
        mut id: *mut ::core::ffi::c_char,
        mut n: *mut integer,
        mut d__: *mut doublereal,
        mut info: *mut integer,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut i__2: integer = 0;
        let mut i__: integer = 0;
        let mut j: integer = 0;
        let mut d1: doublereal = 0.;
        let mut d2: doublereal = 0.;
        let mut d3: doublereal = 0.;
        let mut dir: integer = 0;
        let mut tmp: doublereal = 0.;
        let mut endd: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_lsame_"]
            fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
        }
        let mut stack: [integer; 64] = [0; 64];
        let mut dmnmx: doublereal = 0.;
        let mut start: integer = 0;
        extern "C" {
            #[link_name = "dgelsd_closure_xerbla_"]
            fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
        }
        let mut stkpnt: integer = 0;
        d__ = d__.wrapping_offset(-1);
        *info = 0 as integer;
        dir = -(1 as ::core::ffi::c_int) as integer;
        if lsame__0(
            id,
            b"D\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            dir = 0 as integer;
        } else if lsame__0(
            id,
            b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            dir = 1 as integer;
        }
        if dir == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
            *info = -(1 as ::core::ffi::c_int) as integer;
        } else if *n < 0 as ::core::ffi::c_long {
            *info = -(2 as ::core::ffi::c_int) as integer;
        }
        if *info != 0 as ::core::ffi::c_long {
            i__1 = -*info;
            xerbla__0(
                b"DLASRT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
            );
            return 0 as ::core::ffi::c_int;
        }
        if *n <= 1 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        stkpnt = 1 as integer;
        stack[0 as ::core::ffi::c_int as usize] = 1 as integer;
        stack[1 as ::core::ffi::c_int as usize] = *n;
        loop {
            start = stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                - 2 as ::core::ffi::c_long) as usize];
            endd = stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as usize];
            stkpnt -= 1;
            if endd - start <= 20 as ::core::ffi::c_long && endd - start > 0 as ::core::ffi::c_long
            {
                if dir == 0 as ::core::ffi::c_long {
                    i__1 = endd;
                    i__ = (start as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__1 {
                        i__2 = (start as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        j = i__;
                        while j >= i__2 {
                            if !(*d__.offset(j as isize)
                                > *d__.offset(
                                    (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ))
                            {
                                break;
                            }
                            dmnmx = *d__.offset(j as isize);
                            *d__.offset(j as isize) = *d__.offset(
                                (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            *d__.offset(
                                (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) = dmnmx;
                            j -= 1;
                        }
                        i__ += 1;
                    }
                } else {
                    i__1 = endd;
                    i__ = (start as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__1 {
                        i__2 = (start as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        j = i__;
                        while j >= i__2 {
                            if !(*d__.offset(j as isize)
                                < *d__.offset(
                                    (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ))
                            {
                                break;
                            }
                            dmnmx = *d__.offset(j as isize);
                            *d__.offset(j as isize) = *d__.offset(
                                (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            *d__.offset(
                                (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) = dmnmx;
                            j -= 1;
                        }
                        i__ += 1;
                    }
                }
            } else if endd - start > 20 as ::core::ffi::c_long {
                d1 = *d__.offset(start as isize);
                d2 = *d__.offset(endd as isize);
                i__ = ((start as ::core::ffi::c_long + endd as ::core::ffi::c_long)
                    / 2 as ::core::ffi::c_long) as integer;
                d3 = *d__.offset(i__ as isize);
                if d1 < d2 {
                    if d3 < d1 {
                        dmnmx = d1;
                    } else if d3 < d2 {
                        dmnmx = d3;
                    } else {
                        dmnmx = d2;
                    }
                } else if d3 < d2 {
                    dmnmx = d2;
                } else if d3 < d1 {
                    dmnmx = d3;
                } else {
                    dmnmx = d1;
                }
                if dir == 0 as ::core::ffi::c_long {
                    i__ = (start as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    j = (endd as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    loop {
                        j -= 1;
                        if *d__.offset(j as isize) < dmnmx {
                            continue;
                        }
                        loop {
                            i__ += 1;
                            if !(*d__.offset(i__ as isize) > dmnmx) {
                                break;
                            }
                        }
                        if !(i__ < j) {
                            break;
                        }
                        tmp = *d__.offset(i__ as isize);
                        *d__.offset(i__ as isize) = *d__.offset(j as isize);
                        *d__.offset(j as isize) = tmp;
                    }
                    if j - start
                        > endd as ::core::ffi::c_long
                            - j as ::core::ffi::c_long
                            - 1 as ::core::ffi::c_long
                    {
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] = start;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = j;
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] =
                            (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = endd;
                    } else {
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] =
                            (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = endd;
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] = start;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = j;
                    }
                } else {
                    i__ = (start as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    j = (endd as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    loop {
                        j -= 1;
                        if *d__.offset(j as isize) > dmnmx {
                            continue;
                        }
                        loop {
                            i__ += 1;
                            if !(*d__.offset(i__ as isize) < dmnmx) {
                                break;
                            }
                        }
                        if !(i__ < j) {
                            break;
                        }
                        tmp = *d__.offset(i__ as isize);
                        *d__.offset(i__ as isize) = *d__.offset(j as isize);
                        *d__.offset(j as isize) = tmp;
                    }
                    if j - start
                        > endd as ::core::ffi::c_long
                            - j as ::core::ffi::c_long
                            - 1 as ::core::ffi::c_long
                    {
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] = start;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = j;
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] =
                            (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = endd;
                    } else {
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] =
                            (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = endd;
                        stkpnt += 1;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_long) as usize] = start;
                        stack[(((stkpnt as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_long) as usize] = j;
                    }
                }
            }
            if !(stkpnt > 0 as ::core::ffi::c_long) {
                break;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasrt::dgelsd_closure_dlasrt_;

pub mod raw_dlasv2 {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    pub type logical = ::core::ffi::c_long;
    pub const TRUE_: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const FALSE_: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut c_b3: doublereal = 2.0f64;
    static mut c_b4: doublereal = 1.0f64;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlasv2_(
        mut f: *mut doublereal,
        mut g: *mut doublereal,
        mut h__: *mut doublereal,
        mut ssmin: *mut doublereal,
        mut ssmax: *mut doublereal,
        mut snr: *mut doublereal,
        mut csr: *mut doublereal,
        mut snl: *mut doublereal,
        mut csl: *mut doublereal,
    ) -> ::core::ffi::c_int {
        let mut d__1: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_d_sign"]
            fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
        }
        let mut a: doublereal = 0.;
        let mut d__: doublereal = 0.;
        let mut l: doublereal = 0.;
        let mut m: doublereal = 0.;
        let mut r__: doublereal = 0.;
        let mut s: doublereal = 0.;
        let mut t: doublereal = 0.;
        let mut fa: doublereal = 0.;
        let mut ga: doublereal = 0.;
        let mut ha: doublereal = 0.;
        let mut ft: doublereal = 0.;
        let mut gt: doublereal = 0.;
        let mut ht: doublereal = 0.;
        let mut mm: doublereal = 0.;
        let mut tt: doublereal = 0.;
        let mut clt: doublereal = 0.;
        let mut crt: doublereal = 0.;
        let mut slt: doublereal = 0.;
        let mut srt: doublereal = 0.;
        let mut pmax: integer = 0;
        let mut temp: doublereal = 0.;
        let mut swap: logical = 0;
        let mut tsign: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        let mut gasmal: logical = 0;
        ft = *f;
        fa = (if ft >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            ft as ::core::ffi::c_double
        } else {
            -(ft as ::core::ffi::c_double)
        }) as doublereal;
        ht = *h__;
        ha = (if *h__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *h__
        } else {
            -*h__
        }) as doublereal;
        pmax = 1 as integer;
        swap = (ha > fa) as ::core::ffi::c_int as logical;
        if swap != 0 {
            pmax = 3 as integer;
            temp = ft;
            ft = ht;
            ht = temp;
            temp = fa;
            fa = ha;
            ha = temp;
        }
        gt = *g;
        ga = (if gt >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            gt as ::core::ffi::c_double
        } else {
            -(gt as ::core::ffi::c_double)
        }) as doublereal;
        if ga == 0.0f64 {
            *ssmin = ha;
            *ssmax = fa;
            clt = 1.0f64 as doublereal;
            crt = 1.0f64 as doublereal;
            slt = 0.0f64 as doublereal;
            srt = 0.0f64 as doublereal;
        } else {
            gasmal = TRUE_ as logical;
            if ga > fa {
                pmax = 2 as integer;
                if fa / ga
                    < dlamch__0(
                        b"EPS\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    )
                {
                    gasmal = FALSE_ as logical;
                    *ssmax = ga;
                    if ha > 1.0f64 {
                        *ssmin = fa / (ga / ha);
                    } else {
                        *ssmin = fa / ga * ha;
                    }
                    clt = 1.0f64 as doublereal;
                    slt = ht / gt;
                    srt = 1.0f64 as doublereal;
                    crt = ft / gt;
                }
            }
            if gasmal != 0 {
                d__ = fa - ha;
                if d__ == fa {
                    l = 1.0f64 as doublereal;
                } else {
                    l = d__ / fa;
                }
                m = gt / ft;
                t = 2.0f64 - l;
                mm = m * m;
                tt = t * t;
                s = super::lapack_sqrt(tt + mm) as doublereal;
                if l == 0.0f64 {
                    r__ = (if m >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        m as ::core::ffi::c_double
                    } else {
                        -(m as ::core::ffi::c_double)
                    }) as doublereal;
                } else {
                    r__ = super::lapack_sqrt(l * l + mm) as doublereal;
                }
                a = ((s as ::core::ffi::c_double + r__ as ::core::ffi::c_double) * 0.5f64)
                    as doublereal;
                *ssmin = ha / a;
                *ssmax = fa * a;
                if mm == 0.0f64 {
                    if l == 0.0f64 {
                        t = (d_sign_0(&raw mut c_b3, &raw mut ft)
                            * d_sign_0(&raw mut c_b4, &raw mut gt))
                            as doublereal;
                    } else {
                        t = gt / d_sign_0(&raw mut d__, &raw mut ft) as doublereal + m / t;
                    }
                } else {
                    t = ((m as ::core::ffi::c_double
                        / (s as ::core::ffi::c_double + t as ::core::ffi::c_double)
                        + m as ::core::ffi::c_double
                            / (r__ as ::core::ffi::c_double + l as ::core::ffi::c_double))
                        * (a as ::core::ffi::c_double + 1.0f64))
                        as doublereal;
                }
                l = super::lapack_sqrt(t * t + 4.0f64) as doublereal;
                crt = 2.0f64 / l;
                srt = t / l;
                clt = (crt + srt * m) / a;
                slt = ht / ft * srt / a;
            }
        }
        if swap != 0 {
            *csl = srt;
            *snl = crt;
            *csr = slt;
            *snr = clt;
        } else {
            *csl = clt;
            *snl = slt;
            *csr = crt;
            *snr = srt;
        }
        if pmax == 1 as ::core::ffi::c_long {
            tsign = (d_sign_0(&raw mut c_b4, csr)
                * d_sign_0(&raw mut c_b4, csl)
                * d_sign_0(&raw mut c_b4, f)) as doublereal;
        }
        if pmax == 2 as ::core::ffi::c_long {
            tsign = (d_sign_0(&raw mut c_b4, snr)
                * d_sign_0(&raw mut c_b4, csl)
                * d_sign_0(&raw mut c_b4, g)) as doublereal;
        }
        if pmax == 3 as ::core::ffi::c_long {
            tsign = (d_sign_0(&raw mut c_b4, snr)
                * d_sign_0(&raw mut c_b4, snl)
                * d_sign_0(&raw mut c_b4, h__)) as doublereal;
        }
        *ssmax = d_sign_0(ssmax, &raw mut tsign) as doublereal;
        d__1 = (tsign as ::core::ffi::c_double
            * d_sign_0(&raw mut c_b4, f)
            * d_sign_0(&raw mut c_b4, h__)) as doublereal;
        *ssmin = d_sign_0(ssmin, &raw mut d__1) as doublereal;
        return 0 as ::core::ffi::c_int;
    }
}
pub use raw_dlasv2::dgelsd_closure_dlasv2_;

pub mod raw_dlartg {
    #![allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_assignments,
        unused_mut
    )]
    extern "C" {
        fn sqrt(_: doublereal) -> ::core::ffi::c_double;
        fn log(_: doublereal) -> ::core::ffi::c_double;
    }
    pub type integer = ::core::ffi::c_long;
    pub type doublereal = ::core::ffi::c_double;
    #[no_mangle]
    pub unsafe extern "C" fn dgelsd_closure_dlartg_old_(
        mut f: *mut doublereal,
        mut g: *mut doublereal,
        mut cs: *mut doublereal,
        mut sn: *mut doublereal,
        mut r__: *mut doublereal,
    ) -> ::core::ffi::c_int {
        let mut i__1: integer = 0;
        let mut d__1: doublereal = 0.;
        let mut d__2: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_pow_di"]
            fn pow_di_0(_: *mut doublereal, _: *mut integer) -> ::core::ffi::c_double;
        }
        let mut i__: integer = 0;
        let mut f1: doublereal = 0.;
        let mut g1: doublereal = 0.;
        let mut eps: doublereal = 0.;
        let mut scale: doublereal = 0.;
        let mut count: integer = 0;
        let mut safmn2: doublereal = 0.;
        let mut safmx2: doublereal = 0.;
        extern "C" {
            #[link_name = "dgelsd_closure_dlamch_"]
            fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
        }
        let mut safmin: doublereal = 0.;
        safmin = dlamch__0(
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        eps = dlamch__0(
            b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        d__1 = dlamch__0(
            b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        i__1 = (super::lapack_log(safmin / eps)
            / super::lapack_log(dlamch__0(
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ))
            / 2.0f64) as integer;
        safmn2 = pow_di_0(&raw mut d__1, &raw mut i__1) as doublereal;
        safmx2 = 1.0f64 / safmn2;
        if *g == 0.0f64 {
            *cs = 1.0f64 as doublereal;
            *sn = 0.0f64 as doublereal;
            *r__ = *f;
        } else if *f == 0.0f64 {
            *cs = 0.0f64 as doublereal;
            *sn = 1.0f64 as doublereal;
            *r__ = *g;
        } else {
            f1 = *f;
            g1 = *g;
            d__1 = (if f1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                f1 as ::core::ffi::c_double
            } else {
                -(f1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = (if g1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                g1 as ::core::ffi::c_double
            } else {
                -(g1 as ::core::ffi::c_double)
            }) as doublereal;
            scale = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            if scale >= safmx2 {
                count = 0 as integer;
                loop {
                    count += 1;
                    f1 *= safmn2 as ::core::ffi::c_double;
                    g1 *= safmn2 as ::core::ffi::c_double;
                    d__1 = (if f1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        f1 as ::core::ffi::c_double
                    } else {
                        -(f1 as ::core::ffi::c_double)
                    }) as doublereal;
                    d__2 = (if g1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        g1 as ::core::ffi::c_double
                    } else {
                        -(g1 as ::core::ffi::c_double)
                    }) as doublereal;
                    scale = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                    if !(scale >= safmx2) {
                        break;
                    }
                }
                d__1 = f1;
                d__2 = g1;
                *r__ = super::lapack_sqrt(d__1 * d__1 + d__2 * d__2) as doublereal;
                *cs = f1 / *r__;
                *sn = g1 / *r__;
                i__1 = count;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *r__ *= safmx2 as ::core::ffi::c_double;
                    i__ += 1;
                }
            } else if scale <= safmn2 {
                count = 0 as integer;
                loop {
                    count += 1;
                    f1 *= safmx2 as ::core::ffi::c_double;
                    g1 *= safmx2 as ::core::ffi::c_double;
                    d__1 = (if f1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        f1 as ::core::ffi::c_double
                    } else {
                        -(f1 as ::core::ffi::c_double)
                    }) as doublereal;
                    d__2 = (if g1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        g1 as ::core::ffi::c_double
                    } else {
                        -(g1 as ::core::ffi::c_double)
                    }) as doublereal;
                    scale = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                    if !(scale <= safmn2) {
                        break;
                    }
                }
                d__1 = f1;
                d__2 = g1;
                *r__ = super::lapack_sqrt(d__1 * d__1 + d__2 * d__2) as doublereal;
                *cs = f1 / *r__;
                *sn = g1 / *r__;
                i__1 = count;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *r__ *= safmn2 as ::core::ffi::c_double;
                    i__ += 1;
                }
            } else {
                d__1 = f1;
                d__2 = g1;
                *r__ = super::lapack_sqrt(d__1 * d__1 + d__2 * d__2) as doublereal;
                *cs = f1 / *r__;
                *sn = g1 / *r__;
            }
            if (if *f >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *f
            } else {
                -*f
            }) > (if *g >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *g
            } else {
                -*g
            }) && *cs < 0.0f64
            {
                *cs = -*cs;
                *sn = -*sn;
                *r__ = -*r__;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[allow(unused_imports)]
pub use raw_dlartg::dgelsd_closure_dlartg_old_;

/// LAPACK 3.12.1 `DLARTG` (February 2021 Anderson safe-scaling revision).
///
/// CLAPACK's generated routine above is retained under `dgelsd_closure_dlartg_old_` solely
/// as a mechanical audit trail; all translated callers resolve this current
/// source implementation through the `dgelsd_closure_dlartg_` symbol.
#[no_mangle]
pub unsafe extern "C" fn dgelsd_closure_dlartg_(
    f: *mut FDouble,
    g: *mut FDouble,
    c: *mut FDouble,
    s: *mut FDouble,
    r: *mut FDouble,
) -> core::ffi::c_int {
    let fv = *f;
    let gv = *g;
    let f1 = fv.abs();
    let g1 = gv.abs();
    let safmin = f64::MIN_POSITIVE;
    let safmax = 1.0 / safmin;
    let rtmin = safmin.sqrt();
    let rtmax = (safmax / 2.0).sqrt();
    if gv == 0.0 {
        *c = 1.0;
        *s = 0.0;
        *r = fv;
    } else if fv == 0.0 {
        *c = 0.0;
        *s = 1.0_f64.copysign(gv);
        *r = g1;
    } else if f1 > rtmin && f1 < rtmax && g1 > rtmin && g1 < rtmax {
        let d = (fv * fv + gv * gv).sqrt();
        *c = f1 / d;
        *r = d.copysign(fv);
        *s = gv / *r;
    } else {
        let u = safmax.min(safmin.max(f1.max(g1)));
        let fs = fv / u;
        let gs = gv / u;
        let d = (fs * fs + gs * gs).sqrt();
        *c = fs.abs() / d;
        *r = d.copysign(fv);
        *s = gs / *r;
        *r *= u;
    }
    0
}

#[cfg(test)]
mod oracle_tests {
    use super::*;

    fn close(actual: f64, expected: f64, tol: f64) {
        let scale = 1.0_f64.max(expected.abs());
        assert!(
            (actual - expected).abs() <= tol * scale,
            "actual={actual:.17e}, expected={expected:.17e}, delta={:.3e}",
            (actual - expected).abs()
        );
    }

    /// Fingerprints produced by the SciPy-bundled OpenBLAS `dgelsd_closure_dlalsd_`.
    /// Inputs use ordinary base pointers, exactly as the DGELSD driver does.
    fn dlalsd_fingerprint(
        n0: usize,
        nrhs0: usize,
        lower: bool,
        deficient: bool,
    ) -> (FInt, [f64; 10]) {
        let smlsiz0 = 25usize;
        let mut d = (0..n0).map(|i| 1.0 + i as f64 / 29.0).collect::<Vec<_>>();
        let mut e = (0..n0.saturating_sub(1))
            .map(|i| 0.03 * ((i % 5) as f64 - 2.0))
            .collect::<Vec<_>>();
        if deficient && n0 != 0 {
            d[n0 / 2] = 0.0;
            if n0 > 1 {
                e[(n0 / 2).saturating_sub(1)] = 0.0;
                if n0 / 2 < n0 - 1 {
                    e[n0 / 2] = 0.0;
                }
            }
        }
        let mut b = (0..n0 * nrhs0)
            .map(|i| ((i * 17 + 3) % 23) as f64 / 11.0 - 1.0)
            .collect::<Vec<_>>();
        let nlvl = (((n0.max(1) as f64 / (smlsiz0 + 1) as f64).ln() / 2.0_f64.ln()) as usize) + 1;
        let mut work = vec![
            0.0;
            9 * n0
                + 2 * n0 * smlsiz0
                + 8 * n0 * nlvl
                + n0 * nrhs0
                + (smlsiz0 + 1).pow(2)
                + 1024
        ];
        let mut iwork = vec![0 as FInt; 3 * n0 * nlvl + 11 * n0 + 1024];
        let mut n = n0 as FInt;
        let mut nrhs = nrhs0 as FInt;
        let mut smlsiz = smlsiz0 as FInt;
        let mut ldb = n.max(1);
        let mut rank = 0 as FInt;
        let mut info = 0 as FInt;
        let mut rcond = -1.0;
        let mut uplo = if lower { b'L' as FChar } else { b'U' as FChar };
        unsafe {
            dgelsd_closure_dlalsd_(
                &mut uplo,
                &mut smlsiz,
                &mut n,
                &mut nrhs,
                d.as_mut_ptr(),
                e.as_mut_ptr(),
                b.as_mut_ptr(),
                &mut ldb,
                &mut rcond,
                &mut rank,
                work.as_mut_ptr(),
                iwork.as_mut_ptr(),
                &mut info,
            );
        }
        assert_eq!(info, 0);
        let mid_d = n0 / 2;
        let mid_b = b.len() / 2;
        let sum_d = d.iter().sum();
        let weighted_d = d.iter().enumerate().map(|(i, x)| (i + 1) as f64 * x).sum();
        let sum_b = b.iter().sum();
        let weighted_b = b.iter().enumerate().map(|(i, x)| (i + 1) as f64 * x).sum();
        (
            rank,
            [
                d[0],
                d[mid_d],
                d[n0 - 1],
                b[0],
                b[mid_b],
                b[b.len() - 1],
                sum_d,
                weighted_d,
                sum_b,
                weighted_b,
            ],
        )
    }

    #[test]
    fn dlalsd_matches_openblas_oracle_across_leaf_and_divide_conquer_paths() {
        let cases: &[(usize, usize, bool, bool, FInt, [f64; 10])] = &[
            (
                1,
                1,
                false,
                false,
                1,
                [
                    1.0,
                    1.0,
                    1.0,
                    -0.7272727272727273,
                    -0.7272727272727273,
                    -0.7272727272727273,
                    1.0,
                    1.0,
                    -0.7272727272727273,
                    -0.7272727272727273,
                ],
            ),
            (
                2,
                2,
                false,
                false,
                2,
                [
                    1.0522851756724156,
                    0.9830821364176778,
                    0.9830821364176778,
                    -0.6798181818181818,
                    0.25690909090909086,
                    -0.2636363636363635,
                    2.0353673120900933,
                    3.0184494485077713,
                    0.10436363636363627,
                    0.6181818181818182,
                ],
            ),
            (
                25,
                2,
                false,
                false,
                25,
                [
                    1.8332600908380734,
                    1.421627452730879,
                    0.9824086357442022,
                    -0.6793742521994135,
                    0.25557730205278584,
                    -0.14922813036020582,
                    35.352460146115596,
                    414.5034635222375,
                    -0.42953216672973005,
                    7.091424088063098,
                ],
            ),
            (
                26,
                2,
                false,
                false,
                26,
                [
                    1.8803363224982657,
                    1.4216274527308797,
                    0.9824086357442022,
                    -0.6793742521994135,
                    -0.3189980058651027,
                    0.39057239057239057,
                    37.21501694643789,
                    451.69886932393604,
                    -0.5443113794017636,
                    4.076939255373875,
                ],
            ),
            (
                34,
                3,
                false,
                false,
                34,
                [
                    2.137931034482759,
                    1.565678730914699,
                    0.9824086357442022,
                    -0.6793742521994135,
                    0.458498023715415,
                    0.29765395894428154,
                    53.354650652694176,
                    820.5099808870459,
                    -0.9844675203551468,
                    -26.15456085635808,
                ],
            ),
            (
                70,
                2,
                false,
                false,
                70,
                [
                    3.3849555483504505,
                    2.1590648648369886,
                    0.9824086357442022,
                    -0.6793742521994137,
                    0.8335560703812317,
                    0.24211502782931349,
                    153.29162976075094,
                    4455.7486127213615,
                    -0.18321521199985866,
                    31.130561539258686,
                ],
            ),
            (
                34,
                2,
                true,
                false,
                34,
                [
                    2.137931034482759,
                    1.5656787309146985,
                    0.9824086357442023,
                    -0.7272727272727276,
                    -0.45454545454545464,
                    0.17008797653958938,
                    53.35465065269417,
                    820.5099808870458,
                    -0.3148973334827969,
                    16.25510351086799,
                ],
            ),
            (
                34,
                2,
                false,
                true,
                33,
                [
                    2.137931034482759,
                    1.5188330689320748,
                    2.3735802595434384e-16,
                    -0.6793742521994135,
                    -0.511657595307918,
                    0.17008797653958938,
                    51.76830032571112,
                    771.8551979597041,
                    -1.1811242127767079,
                    -16.331806084587274,
                ],
            ),
        ];
        for &(n, nrhs, lower, deficient, expected_rank, expected) in cases {
            let (rank, actual) = dlalsd_fingerprint(n, nrhs, lower, deficient);
            assert_eq!(
                rank, expected_rank,
                "n={n}, lower={lower}, deficient={deficient}"
            );
            for (a, e) in actual.into_iter().zip(expected) {
                close(a, e, 2.0e-12);
            }
        }
    }

    #[test]
    fn dlasd4_matches_openblas_oracle() {
        let expected = [
            0.3879588622508436,
            1.1020978909217016,
            3.0323291581272183,
            7.01354746450029,
        ];
        for i0 in 1..=4 {
            let mut n = 4 as FInt;
            let mut i = i0 as FInt;
            let mut d = [0.0, 1.0, 3.0, 7.0];
            let mut z = [0.5; 4];
            let mut delta = [0.0; 4];
            let mut rho = 0.75;
            let mut sigma = 0.0;
            let mut work = [0.0; 4];
            let mut info = 0 as FInt;
            unsafe {
                dgelsd_closure_dlasd4_(
                    &mut n,
                    &mut i,
                    d.as_mut_ptr(),
                    z.as_mut_ptr(),
                    delta.as_mut_ptr(),
                    &mut rho,
                    &mut sigma,
                    work.as_mut_ptr(),
                    &mut info,
                )
            };
            assert_eq!(info, 0);
            close(sigma, expected[i0 - 1], 4.0e-15);
        }
    }

    #[test]
    fn dlasd8_matches_openblas_oracle() {
        let mut icompq = 1 as FInt;
        let mut k = 4 as FInt;
        let mut d = [0.0; 4];
        let mut z = [0.2, -0.3, 0.4, -0.5];
        let mut vf = [0.1, 0.2, 0.3, 0.4];
        let mut vl = [-0.4, 0.3, -0.2, 0.1];
        let mut difl = [0.0; 4];
        let mut difr = [0.0; 8];
        let mut lddifr = 4 as FInt;
        let mut dsigma = [0.0, 1.0, 3.0, 7.0];
        let mut work = [0.0; 12];
        let mut info = 0 as FInt;
        unsafe {
            dgelsd_closure_dlasd8_(
                &mut icompq,
                &mut k,
                d.as_mut_ptr(),
                z.as_mut_ptr(),
                vf.as_mut_ptr(),
                vl.as_mut_ptr(),
                difl.as_mut_ptr(),
                difr.as_mut_ptr(),
                &mut lddifr,
                dsigma.as_mut_ptr(),
                work.as_mut_ptr(),
                &mut info,
            )
        };
        assert_eq!(info, 0);
        let expected_d = [
            0.18929522024575365,
            1.0445193888039406,
            3.0267924172450162,
            7.017953706666411,
        ];
        let expected_vf = [
            -0.10930934604730089,
            0.19744567742617775,
            -0.29989163127832147,
            0.39891312440386884,
        ];
        let expected_vl = [
            0.38088997341658964,
            0.3183306654845167,
            0.2074363915999329,
            0.10275484904807981,
        ];
        for (a, e) in d.into_iter().zip(expected_d) {
            close(a, e, 4.0e-15);
        }
        for (a, e) in vf.into_iter().zip(expected_vf) {
            close(a, e, 4.0e-15);
        }
        for (a, e) in vl.into_iter().zip(expected_vl) {
            close(a, e, 4.0e-15);
        }
    }
}
