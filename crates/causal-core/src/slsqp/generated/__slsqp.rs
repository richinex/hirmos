extern "C" {
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn hypot(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmax(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn dgelsd_closure_f2c_ddot(
        n: *mut ::core::ffi::c_long,
        dx: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
        dy: *mut ::core::ffi::c_double,
        incy: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_double;
    fn dgelsd_closure_f2c_dnrm2(
        n: *mut ::core::ffi::c_long,
        x: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_double;
    fn slsqp_nnls(
        m: ::core::ffi::c_long,
        n: ::core::ffi::c_long,
        a: *mut ::core::ffi::c_double,
        b: *mut ::core::ffi::c_double,
        x: *mut ::core::ffi::c_double,
        w: *mut ::core::ffi::c_double,
        zz: *mut ::core::ffi::c_double,
        indices: *mut ::core::ffi::c_long,
        maxiter: ::core::ffi::c_long,
        rnorm: *mut ::core::ffi::c_double,
        info: *mut ::core::ffi::c_long,
    );
    fn dsyevd_closure_f2c_daxpy(
        n: *mut ::core::ffi::c_long,
        sa: *mut ::core::ffi::c_double,
        sx: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
        sy: *mut ::core::ffi::c_double,
        incy: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dgelsy_(
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        nrhs: *mut ::core::ffi::c_long,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        b: *mut ::core::ffi::c_double,
        ldb: *mut ::core::ffi::c_long,
        jpvt: *mut ::core::ffi::c_long,
        rcond: *mut ::core::ffi::c_double,
        rank: *mut ::core::ffi::c_long,
        work: *mut ::core::ffi::c_double,
        lwork: *mut ::core::ffi::c_long,
        info: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn dgelsd_closure_f2c_dgemv(
        trans: *mut ::core::ffi::c_char,
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        alpha: *mut ::core::ffi::c_double,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        x: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
        beta: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
        incy: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn dgesdd_closure_dgeqr2_(
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        tau: *mut ::core::ffi::c_double,
        work: *mut ::core::ffi::c_double,
        info: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dgerq2_(
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        tau: *mut ::core::ffi::c_double,
        work: *mut ::core::ffi::c_double,
        info: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn dgeev_closure_dorm2r_(
        side: *mut ::core::ffi::c_char,
        trans: *mut ::core::ffi::c_char,
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        k: *mut ::core::ffi::c_long,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        tau: *mut ::core::ffi::c_double,
        c: *mut ::core::ffi::c_double,
        ldc: *mut ::core::ffi::c_long,
        work: *mut ::core::ffi::c_double,
        info: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dormr2_(
        side: *mut ::core::ffi::c_char,
        trans: *mut ::core::ffi::c_char,
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        k: *mut ::core::ffi::c_long,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        tau: *mut ::core::ffi::c_double,
        c: *mut ::core::ffi::c_double,
        ldc: *mut ::core::ffi::c_long,
        work: *mut ::core::ffi::c_double,
        info: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn dgelsd_closure_f2c_dscal(
        n: *mut ::core::ffi::c_long,
        da: *mut ::core::ffi::c_double,
        dx: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dtpmv(
        uplo: *mut ::core::ffi::c_char,
        trans: *mut ::core::ffi::c_char,
        diag: *mut ::core::ffi::c_char,
        n: *mut ::core::ffi::c_long,
        ap: *mut ::core::ffi::c_double,
        x: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dtpsv(
        uplo: *mut ::core::ffi::c_char,
        trans: *mut ::core::ffi::c_char,
        diag: *mut ::core::ffi::c_char,
        n: *mut ::core::ffi::c_long,
        ap: *mut ::core::ffi::c_double,
        x: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dtrsm_(
        side: *mut ::core::ffi::c_char,
        uplo: *mut ::core::ffi::c_char,
        transa: *mut ::core::ffi::c_char,
        diag: *mut ::core::ffi::c_char,
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        alpha: *mut ::core::ffi::c_double,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        b: *mut ::core::ffi::c_double,
        ldb: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dtrsv(
        uplo: *mut ::core::ffi::c_char,
        trans: *mut ::core::ffi::c_char,
        diag: *mut ::core::ffi::c_char,
        n: *mut ::core::ffi::c_long,
        a: *mut ::core::ffi::c_double,
        lda: *mut ::core::ffi::c_long,
        x: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SLSQP_vars {
    pub acc: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub f0: ::core::ffi::c_double,
    pub gs: ::core::ffi::c_double,
    pub h1: ::core::ffi::c_double,
    pub h2: ::core::ffi::c_double,
    pub h3: ::core::ffi::c_double,
    pub h4: ::core::ffi::c_double,
    pub t: ::core::ffi::c_double,
    pub t0: ::core::ffi::c_double,
    pub tol: ::core::ffi::c_double,
    pub exact: ::core::ffi::c_long,
    pub inconsistent: ::core::ffi::c_long,
    pub reset: ::core::ffi::c_long,
    pub iter: ::core::ffi::c_long,
    pub itermax: ::core::ffi::c_long,
    pub line: ::core::ffi::c_long,
    pub m: ::core::ffi::c_long,
    pub meq: ::core::ffi::c_long,
    pub mode: ::core::ffi::c_long,
    pub n: ::core::ffi::c_long,
}
#[no_mangle]
pub unsafe extern "C" fn slsqp_body(
    mut S: *mut SLSQP_vars,
    mut funx: *mut ::core::ffi::c_double,
    mut gradx: *mut ::core::ffi::c_double,
    mut C: *mut ::core::ffi::c_double,
    mut d: *mut ::core::ffi::c_double,
    mut sol: *mut ::core::ffi::c_double,
    mut mult: *mut ::core::ffi::c_double,
    mut xl: *mut ::core::ffi::c_double,
    mut xu: *mut ::core::ffi::c_double,
    mut buffer: *mut ::core::ffi::c_double,
    mut indices: *mut ::core::ffi::c_long,
) {
    let mut current_block: u64;
    let mut one: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut lda: ::core::ffi::c_long = if (*S).m > 0 as ::core::ffi::c_long {
        (*S).m
    } else {
        1 as ::core::ffi::c_long
    };
    let mut j: ::core::ffi::c_long = 0;
    let mut done: ::core::ffi::c_double = 1.0f64;
    let mut dmone: ::core::ffi::c_double = -1.0f64;
    let mut alfmin: ::core::ffi::c_double = 0.1f64;
    let mut n: ::core::ffi::c_long = (*S).n;
    let mut m: ::core::ffi::c_long = (*S).m;
    let mut n1: ::core::ffi::c_long = n + 1 as ::core::ffi::c_long;
    let mut n2: ::core::ffi::c_long = n1 * n / 2 as ::core::ffi::c_long;
    let mut bfgs: *mut ::core::ffi::c_double =
        buffer.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    let mut x0: *mut ::core::ffi::c_double =
        buffer.offset(n2 as isize) as *mut ::core::ffi::c_double;
    let mut mu: *mut ::core::ffi::c_double =
        buffer.offset((n2 + n) as isize) as *mut ::core::ffi::c_double;
    let mut s: *mut ::core::ffi::c_double =
        buffer.offset((n2 + n + m) as isize) as *mut ::core::ffi::c_double;
    let mut u: *mut ::core::ffi::c_double =
        buffer.offset((n2 + n + m + n1) as isize) as *mut ::core::ffi::c_double;
    let mut v: *mut ::core::ffi::c_double =
        buffer.offset((n2 + n + m + n1 + n1) as isize) as *mut ::core::ffi::c_double;
    let mut lsq_buffer: *mut ::core::ffi::c_double =
        buffer.offset((n2 + n + m + n1 + n1 + n1) as isize) as *mut ::core::ffi::c_double;
    let mut badlin: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if (*S).mode == 0 as ::core::ffi::c_long {
        current_block = 13413499668543719749;
    } else if (*S).mode == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        let mut i_8: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_8 < n {
            *u.offset(i_8 as isize) = *gradx.offset(i_8 as isize);
            i_8 += 1;
        }
        dgelsd_closure_f2c_dgemv(
            b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut m,
            &raw mut n,
            &raw mut dmone,
            C,
            &raw mut lda,
            mult,
            &raw mut one,
            &raw mut done,
            u,
            &raw mut one,
        );
        let mut i_9: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_9 < n {
            *u.offset(i_9 as isize) = *u.offset(i_9 as isize) - *v.offset(i_9 as isize);
            i_9 += 1;
        }
        let mut i_10: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_10 < n {
            *v.offset(i_10 as isize) = *s.offset(i_10 as isize);
            i_10 += 1;
        }
        slsqp_closure_dtpmv(
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut n,
            bfgs,
            v,
            &raw mut one,
        );
        j = 0 as ::core::ffi::c_long;
        let mut i_11: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_11 < n {
            *v.offset(i_11 as isize) = *bfgs.offset(j as isize) * *v.offset(i_11 as isize);
            j += n - i_11;
            i_11 += 1;
        }
        slsqp_closure_dtpmv(
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut n,
            bfgs,
            v,
            &raw mut one,
        );
        (*S).h1 = dgelsd_closure_f2c_ddot(&raw mut n, s, &raw mut one, u, &raw mut one);
        (*S).h2 = dgelsd_closure_f2c_ddot(&raw mut n, s, &raw mut one, v, &raw mut one);
        (*S).h3 = 0.2f64 * (*S).h2;
        if (*S).h1 < (*S).h3 {
            (*S).h4 = ((*S).h2 - (*S).h3) / ((*S).h2 - (*S).h1);
            (*S).h1 = (*S).h3;
            let mut tmp_dbl: ::core::ffi::c_double = 1.0f64 - (*S).h4;
            dgelsd_closure_f2c_dscal(&raw mut n, &raw mut (*S).h4, u, &raw mut one);
            dsyevd_closure_f2c_daxpy(
                &raw mut n,
                &raw mut tmp_dbl,
                v,
                &raw mut one,
                u,
                &raw mut one,
            );
        }
        if (*S).h1 == 0.0f64 || (*S).h2 == 0.0f64 {
            current_block = 1826393108436451184;
        } else {
            ldl_update(n, bfgs, u, 1.0f64 / (*S).h1, v);
            ldl_update(n, bfgs, v, -1.0f64 / (*S).h2, u);
            current_block = 8771614158763595177;
        }
    } else if (*S).mode == 1 as ::core::ffi::c_long {
        (*S).t = *funx;
        let mut j_2: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while j_2 < m {
            if j_2 < (*S).meq {
                (*S).h1 = *d.offset(j_2 as isize);
            } else {
                (*S).h1 = 0.0f64;
            }
            (*S).t = (*S).t + *mu.offset(j_2 as isize) * fmax(-*d.offset(j_2 as isize), (*S).h1);
            j_2 += 1;
        }
        (*S).h1 = (*S).t - (*S).t0;
        if !((*S).h1 <= (*S).h3 / 10.0f64 || (*S).line > 10 as ::core::ffi::c_long) {
            (*S).alpha = fmax((*S).h3 / (2.0f64 * ((*S).h3 - (*S).h1)), alfmin);
        } else {
            (*S).h3 = 0.0f64;
            let mut j_3: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while j_3 < m {
                if j_3 < (*S).meq {
                    (*S).h1 = *d.offset(j_3 as isize);
                } else {
                    (*S).h1 = 0.0f64;
                }
                (*S).h3 = (*S).h3 + fmax(-*d.offset(j_3 as isize), (*S).h1);
                j_3 += 1;
            }
            if (fabs(*funx - (*S).f0) < (*S).acc
                || dgelsd_closure_f2c_dnrm2(&raw mut n, s, &raw mut one) < (*S).acc)
                && (*S).h3 < (*S).acc
                && badlin == 0
                && *funx == *funx
            {
                (*S).mode = 0 as ::core::ffi::c_long;
                return;
            } else {
                (*S).mode = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
            }
            return;
        }
        current_block = 8677990145982646489;
    } else {
        current_block = 13413499668543719749;
    }
    match current_block {
        13413499668543719749 => {
            (*S).exact = 0 as ::core::ffi::c_long;
            (*S).acc = fabs((*S).acc);
            (*S).tol = 10 as ::core::ffi::c_int as ::core::ffi::c_double * (*S).acc;
            (*S).iter = 0 as ::core::ffi::c_long;
            (*S).reset = 0 as ::core::ffi::c_long;
            let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i < n {
                *s.offset(i as isize) = 0.0f64;
                i += 1;
            }
            let mut i_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_0 < m {
                *mu.offset(i_0 as isize) = 0.0f64;
                i_0 += 1;
            }
            current_block = 1826393108436451184;
        }
        _ => {}
    }
    loop {
        match current_block {
            8771614158763595177 => {
                (*S).mode = 9 as ::core::ffi::c_long;
                if (*S).iter >= (*S).itermax {
                    return;
                }
                (*S).iter += 1;
                let mut i_3: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                while i_3 < n {
                    *u.offset(i_3 as isize) = -*sol.offset(i_3 as isize) + *xl.offset(i_3 as isize);
                    *v.offset(i_3 as isize) = -*sol.offset(i_3 as isize) + *xu.offset(i_3 as isize);
                    i_3 += 1;
                }
                (*S).h4 = 1.0f64;
                lsq(
                    m,
                    (*S).meq,
                    n,
                    0 as ::core::ffi::c_long,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    bfgs,
                    gradx,
                    C,
                    d,
                    u,
                    v,
                    s,
                    mult,
                    lsq_buffer,
                    indices,
                    &raw mut (*S).mode,
                );
                badlin = 0 as ::core::ffi::c_long;
                if (*S).mode == 6 as ::core::ffi::c_long && n == (*S).meq {
                    (*S).mode = 4 as ::core::ffi::c_long;
                }
                if (*S).mode == 4 as ::core::ffi::c_long {
                    badlin = 1 as ::core::ffi::c_long;
                    let mut i_4: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                    while i_4 < n {
                        *s.offset(i_4 as isize) = 0.0f64;
                        i_4 += 1;
                    }
                    (*S).h3 = 0.0f64;
                    let mut rho: ::core::ffi::c_double = 100.0f64;
                    (*S).inconsistent = 0 as ::core::ffi::c_long;
                    loop {
                        lsq(
                            m,
                            (*S).meq,
                            n,
                            1 as ::core::ffi::c_long,
                            rho,
                            bfgs,
                            gradx,
                            C,
                            d,
                            u,
                            v,
                            s,
                            mult,
                            lsq_buffer,
                            indices,
                            &raw mut (*S).mode,
                        );
                        (*S).h4 = 1.0f64 - *s.offset(n as isize);
                        if (*S).mode == 4 as ::core::ffi::c_long {
                            rho *= 10.0f64;
                            (*S).inconsistent += 1;
                            if (*S).inconsistent > 5 as ::core::ffi::c_long {
                                return;
                            }
                        } else {
                            if (*S).mode != 1 as ::core::ffi::c_long {
                                return;
                            }
                            break;
                        }
                    }
                } else if (*S).mode != 1 as ::core::ffi::c_long {
                    return;
                }
                let mut i_5: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                while i_5 < n {
                    *v.offset(i_5 as isize) = *gradx.offset(i_5 as isize);
                    i_5 += 1;
                }
                dgelsd_closure_f2c_dgemv(
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut m,
                    &raw mut n,
                    &raw mut dmone,
                    C,
                    &raw mut lda,
                    mult,
                    &raw mut one,
                    &raw mut done,
                    v,
                    &raw mut one,
                );
                (*S).f0 = *funx;
                let mut i_6: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                while i_6 < n {
                    *x0.offset(i_6 as isize) = *sol.offset(i_6 as isize);
                    i_6 += 1;
                }
                (*S).gs = dgelsd_closure_f2c_ddot(&raw mut n, gradx, &raw mut one, s, &raw mut one);
                (*S).h1 = fabs((*S).gs);
                (*S).h2 = 0.0f64;
                let mut j_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                while j_0 < m {
                    if j_0 < (*S).meq {
                        (*S).h3 = *d.offset(j_0 as isize);
                    } else {
                        (*S).h3 = 0.0f64;
                    }
                    (*S).h2 = (*S).h2 + fmax(-*d.offset(j_0 as isize), (*S).h3);
                    (*S).h3 = fabs(*mult.offset(j_0 as isize));
                    *mu.offset(j_0 as isize) =
                        fmax((*S).h3, (*mu.offset(j_0 as isize) + (*S).h3) / 2.0f64);
                    (*S).h1 = (*S).h1 + (*S).h3 * fabs(*d.offset(j_0 as isize));
                    j_0 += 1;
                }
                (*S).mode = 0 as ::core::ffi::c_long;
                if (*S).h1 < (*S).acc && (*S).h2 < (*S).acc && badlin == 0 && *funx == *funx {
                    return;
                }
                (*S).h1 = 0.0f64;
                let mut j_1: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                while j_1 < m {
                    if j_1 < (*S).meq {
                        (*S).h3 = *d.offset(j_1 as isize);
                    } else {
                        (*S).h3 = 0.0f64;
                    }
                    (*S).h1 += *mu.offset(j_1 as isize) * fmax(-*d.offset(j_1 as isize), (*S).h3);
                    j_1 += 1;
                }
                (*S).t0 = *funx + (*S).h1;
                (*S).h3 = (*S).gs - (*S).h1 * (*S).h4;
                (*S).mode = 8 as ::core::ffi::c_long;
                if (*S).h3 >= 0.0f64 {
                    current_block = 1826393108436451184;
                    continue;
                }
                (*S).line = 0 as ::core::ffi::c_long;
                (*S).alpha = 1.0f64;
                current_block = 8677990145982646489;
            }
            1826393108436451184 => {
                (*S).reset += 1;
                if (*S).reset > 5 as ::core::ffi::c_long {
                    (*S).h3 = 0.0f64;
                    let mut j_4: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                    while j_4 < m {
                        if j_4 < (*S).meq {
                            (*S).h1 = *d.offset(j_4 as isize);
                        } else {
                            (*S).h1 = 0.0f64;
                        }
                        (*S).h3 = (*S).h3 + fmax(-*d.offset(j_4 as isize), (*S).h1);
                        j_4 += 1;
                    }
                    if (fabs(*funx - (*S).f0) < (*S).tol
                        || dgelsd_closure_f2c_dnrm2(&raw mut n, s, &raw mut one) < (*S).tol)
                        && (*S).h3 < (*S).tol
                        && badlin == 0
                        && *funx == *funx
                    {
                        (*S).mode = 0 as ::core::ffi::c_long;
                    } else {
                        (*S).mode = 8 as ::core::ffi::c_long;
                    }
                    return;
                } else {
                    let mut i_1: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                    while i_1 < n2 {
                        *bfgs.offset(i_1 as isize) = 0.0f64;
                        i_1 += 1;
                    }
                    j = 0 as ::core::ffi::c_long;
                    let mut i_2: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                    while i_2 < n {
                        *bfgs.offset(j as isize) = 1.0f64;
                        j += n - i_2;
                        i_2 += 1;
                    }
                    current_block = 8771614158763595177;
                }
            }
            _ => {
                (*S).line += 1;
                (*S).h3 = (*S).alpha * (*S).h3;
                dgelsd_closure_f2c_dscal(&raw mut n, &raw mut (*S).alpha, s, &raw mut one);
                let mut i_7: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                while i_7 < n {
                    *sol.offset(i_7 as isize) = *x0.offset(i_7 as isize);
                    i_7 += 1;
                }
                dsyevd_closure_f2c_daxpy(
                    &raw mut n,
                    &raw mut done,
                    s,
                    &raw mut one,
                    sol,
                    &raw mut one,
                );
                (*S).mode = 1 as ::core::ffi::c_long;
                return;
            }
        }
    }
}
unsafe extern "C" fn lsq(
    mut m: ::core::ffi::c_long,
    mut meq: ::core::ffi::c_long,
    mut n: ::core::ffi::c_long,
    mut augment: ::core::ffi::c_long,
    mut aug_weight: ::core::ffi::c_double,
    mut Lf: *mut ::core::ffi::c_double,
    mut gradx: *mut ::core::ffi::c_double,
    mut C: *mut ::core::ffi::c_double,
    mut d: *mut ::core::ffi::c_double,
    mut xl: *mut ::core::ffi::c_double,
    mut xu: *mut ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
    mut buffer: *mut ::core::ffi::c_double,
    mut jw: *mut ::core::ffi::c_long,
    mut mode: *mut ::core::ffi::c_long,
) {
    let mut one: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut orign: ::core::ffi::c_long = n;
    let mut mineq: ::core::ffi::c_long = m - meq;
    let mut xnorm: ::core::ffi::c_double = 0.0f64;
    let mut cursor: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut ld: ::core::ffi::c_long = n;
    let mut n_wG_rows: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if augment != 0 {
        ld = n + 1 as ::core::ffi::c_long;
        *x.offset(n as isize) = 1.0f64;
        *xl.offset(n as isize) = 0.0f64;
        *xu.offset(n as isize) = 1.0f64;
    }
    let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i < (ld + 2 as ::core::ffi::c_long) * ld {
        *buffer.offset(i as isize) = 0.0f64;
        i += 1;
    }
    let mut wA: *mut ::core::ffi::c_double = buffer;
    let mut wb: *mut ::core::ffi::c_double = buffer
        .offset((ld * (ld + 1 as ::core::ffi::c_long)) as isize)
        as *mut ::core::ffi::c_double;
    let mut j: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while j < n {
        let fresh0 = cursor;
        cursor = cursor + 1;
        let mut diag: ::core::ffi::c_double = sqrt(*Lf.offset(fresh0 as isize));
        *wA.offset((j + j * ld) as isize) = diag;
        let mut i_0: ::core::ffi::c_long = j + 1 as ::core::ffi::c_long;
        while i_0 < n {
            let fresh1 = cursor;
            cursor = cursor + 1;
            *wA.offset((j + i_0 * ld) as isize) = *Lf.offset(fresh1 as isize) * diag;
            i_0 += 1;
        }
        j += 1;
    }
    let mut i_1: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_1 < n {
        *wb.offset(i_1 as isize) = *gradx.offset(i_1 as isize);
        i_1 += 1;
    }
    slsqp_closure_dtpsv(
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut n,
        Lf,
        wb,
        &raw mut one,
    );
    cursor = 0 as ::core::ffi::c_long;
    let mut i_2: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_2 < n {
        *wb.offset(i_2 as isize) /= -sqrt(*Lf.offset(cursor as isize));
        cursor += n - i_2;
        i_2 += 1;
    }
    if augment != 0 {
        *wA.offset((ld * ld - 1 as ::core::ffi::c_long) as isize) = aug_weight;
    }
    if augment != 0 {
        n += 1;
    }
    let mut wE: *mut ::core::ffi::c_double = buffer
        .offset((n * (n + 1 as ::core::ffi::c_long) + n) as isize)
        as *mut ::core::ffi::c_double;
    let mut wf: *mut ::core::ffi::c_double = buffer
        .offset((n * (n + 1 as ::core::ffi::c_long) + n + n * meq) as isize)
        as *mut ::core::ffi::c_double;
    if meq > 0 as ::core::ffi::c_long {
        let mut j_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while j_0 < n - 1 as ::core::ffi::c_long {
            let mut i_3: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_3 < meq {
                *wE.offset((i_3 + j_0 * meq) as isize) = *C.offset((i_3 + j_0 * m) as isize);
                i_3 += 1;
            }
            j_0 += 1;
        }
        if augment != 0 {
            let mut i_4: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_4 < meq {
                *wE.offset((i_4 + (n - 1 as ::core::ffi::c_long) * meq) as isize) =
                    -*d.offset(i_4 as isize);
                i_4 += 1;
            }
        } else {
            let mut i_5: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_5 < meq {
                *wE.offset((i_5 + (n - 1 as ::core::ffi::c_long) * meq) as isize) =
                    *C.offset((i_5 + (n - 1 as ::core::ffi::c_long) * m) as isize);
                i_5 += 1;
            }
        }
        let mut i_6: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_6 < meq {
            *wf.offset(i_6 as isize) = -*d.offset(i_6 as isize);
            i_6 += 1;
        }
    }
    let mut wG: *mut ::core::ffi::c_double = buffer
        .offset((n * (n + 1 as ::core::ffi::c_long) + n + n * meq + meq) as isize)
        as *mut ::core::ffi::c_double;
    let mut wh: *mut ::core::ffi::c_double = buffer.offset(
        (n * (n + 1 as ::core::ffi::c_long)
            + n
            + n * meq
            + meq
            + (mineq + 2 as ::core::ffi::c_long * n) * ld) as isize,
    ) as *mut ::core::ffi::c_double;
    let mut i_7: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_7 < (mineq + 2 as ::core::ffi::c_long * n) * (ld + 1 as ::core::ffi::c_long) {
        *wG.offset(i_7 as isize) = 0.0f64;
        i_7 += 1;
    }
    let mut nancount: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut nrow: ::core::ffi::c_long = mineq;
    if m > meq {
        let mut i_8: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_8 < mineq {
            *wh.offset(i_8 as isize) = -*d.offset((meq + i_8) as isize);
            i_8 += 1;
        }
    }
    let mut i_9: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_9 < n {
        if *xl.offset(i_9 as isize) != *xl.offset(i_9 as isize) {
            nancount += 1;
        } else {
            let fresh2 = nrow;
            nrow = nrow + 1;
            *wh.offset(fresh2 as isize) = *xl.offset(i_9 as isize);
        }
        i_9 += 1;
    }
    let mut i_10: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_10 < n {
        if *xu.offset(i_10 as isize) != *xu.offset(i_10 as isize) {
            nancount += 1;
        } else {
            let fresh3 = nrow;
            nrow = nrow + 1;
            *wh.offset(fresh3 as isize) = -*xu.offset(i_10 as isize);
        }
        i_10 += 1;
    }
    n_wG_rows = mineq + 2 as ::core::ffi::c_long * n - nancount;
    if m > meq {
        let mut j_1: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while j_1 < orign {
            let mut i_11: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_11 < mineq {
                *wG.offset((i_11 + j_1 * n_wG_rows) as isize) =
                    *C.offset((meq + i_11 + j_1 * m) as isize);
                i_11 += 1;
            }
            j_1 += 1;
        }
    }
    if augment != 0 {
        let mut i_12: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_12 < mineq {
            *wG.offset((i_12 + orign * n_wG_rows) as isize) =
                fmax(-*d.offset((meq + i_12) as isize), 0.0f64);
            i_12 += 1;
        }
    }
    nrow = mineq;
    let mut i_13: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_13 < n {
        if !(*xl.offset(i_13 as isize) != *xl.offset(i_13 as isize)) {
            *wG.offset((nrow + i_13 * n_wG_rows) as isize) = 1.0f64;
            nrow += 1;
        }
        i_13 += 1;
    }
    let mut i_14: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_14 < n {
        if !(*xu.offset(i_14 as isize) != *xu.offset(i_14 as isize)) {
            *wG.offset((nrow + i_14 * n_wG_rows) as isize) = -1.0f64;
            nrow += 1;
        }
        i_14 += 1;
    }
    let mut lsei_scratch: *mut ::core::ffi::c_double =
        wh.offset((mineq + 2 as ::core::ffi::c_long * n) as isize) as *mut ::core::ffi::c_double;
    lsei(
        ld,
        meq,
        n_wG_rows,
        n,
        wA,
        wb,
        wE,
        wf,
        wG,
        wh,
        x,
        lsei_scratch,
        jw,
        &raw mut xnorm,
        mode,
    );
    if *mode == 1 as ::core::ffi::c_long {
        let mut i_15: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_15 < meq {
            *y.offset(i_15 as isize) = *lsei_scratch.offset((i_15 + n_wG_rows) as isize);
            i_15 += 1;
        }
        let mut i_16: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_16 < mineq {
            *y.offset((meq + i_16) as isize) = *lsei_scratch.offset(i_16 as isize);
            i_16 += 1;
        }
        let mut i_17: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_17 < 2 as ::core::ffi::c_long * n {
            *y.offset((m + i_17) as isize) = ::core::f32::NAN as ::core::ffi::c_double;
            i_17 += 1;
        }
    }
    let mut i_18: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_18 < n {
        if !(*xl.offset(i_18 as isize) != *xl.offset(i_18 as isize))
            && *x.offset(i_18 as isize) < *xl.offset(i_18 as isize)
        {
            *x.offset(i_18 as isize) = *xl.offset(i_18 as isize);
        } else if !(*xu.offset(i_18 as isize) != *xu.offset(i_18 as isize))
            && *x.offset(i_18 as isize) > *xu.offset(i_18 as isize)
        {
            *x.offset(i_18 as isize) = *xu.offset(i_18 as isize);
        }
        i_18 += 1;
    }
}
unsafe extern "C" fn lsei(
    mut ma: ::core::ffi::c_long,
    mut me: ::core::ffi::c_long,
    mut mg: ::core::ffi::c_long,
    mut n: ::core::ffi::c_long,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut e: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut h: *mut ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
    mut buffer: *mut ::core::ffi::c_double,
    mut jw: *mut ::core::ffi::c_long,
    mut xnorm: *mut ::core::ffi::c_double,
    mut mode: *mut ::core::ffi::c_long,
) {
    let mut one: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut nvars: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut info: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut lde: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut ldg: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut done: ::core::ffi::c_double = 1.0f64;
    let mut dmone: ::core::ffi::c_double = -1.0f64;
    let mut dzero: ::core::ffi::c_double = 0.0f64;
    let mut t: ::core::ffi::c_double = 0.0f64;
    let epsmach: ::core::ffi::c_double = 2.220446049250313e-16f64;
    let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i < n {
        *x.offset(i as isize) = 0.0f64;
        i += 1;
    }
    if me > n {
        *mode = 2 as ::core::ffi::c_long;
        return;
    }
    nvars = n - me;
    let mut gmults: *mut ::core::ffi::c_double =
        buffer.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    let mut emults: *mut ::core::ffi::c_double =
        buffer.offset(mg as isize) as *mut ::core::ffi::c_double;
    let mut wb: *mut ::core::ffi::c_double =
        buffer.offset((me + mg) as isize) as *mut ::core::ffi::c_double;
    let mut tau: *mut ::core::ffi::c_double =
        buffer.offset((me + mg + ma) as isize) as *mut ::core::ffi::c_double;
    let mut a2: *mut ::core::ffi::c_double = buffer
        .offset((mg + 2 as ::core::ffi::c_long * me + ma) as isize)
        as *mut ::core::ffi::c_double;
    let mut g2: *mut ::core::ffi::c_double = buffer
        .offset((mg + 2 as ::core::ffi::c_long * me + ma + ma * nvars) as isize)
        as *mut ::core::ffi::c_double;
    let mut lsi_scratch: *mut ::core::ffi::c_double = buffer
        .offset((mg + 2 as ::core::ffi::c_long * me + ma + (ma + mg) * nvars) as isize)
        as *mut ::core::ffi::c_double;
    lde = if me > 0 as ::core::ffi::c_long {
        me
    } else {
        1 as ::core::ffi::c_long
    };
    ldg = if mg > 0 as ::core::ffi::c_long {
        mg
    } else {
        1 as ::core::ffi::c_long
    };
    slsqp_closure_dgerq2_(
        &raw mut me,
        &raw mut n,
        e,
        &raw mut lde,
        tau,
        lsi_scratch,
        &raw mut info,
    );
    slsqp_closure_dormr2_(
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut ma,
        &raw mut n,
        &raw mut me,
        e,
        &raw mut lde,
        tau,
        a,
        &raw mut ma,
        lsi_scratch,
        &raw mut info,
    );
    slsqp_closure_dormr2_(
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut mg,
        &raw mut n,
        &raw mut me,
        e,
        &raw mut lde,
        tau,
        g,
        &raw mut ldg,
        lsi_scratch,
        &raw mut info,
    );
    let mut i_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_0 < me {
        if !(fabs(*e.offset((i_0 + (nvars + i_0) * me) as isize)) >= epsmach) {
            *mode = 6 as ::core::ffi::c_long;
            return;
        }
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_1 < me {
        *x.offset((nvars + i_1) as isize) = *f.offset(i_1 as isize);
        i_1 += 1;
    }
    slsqp_closure_dtrsv(
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut me,
        e.offset((nvars * me) as isize) as *mut ::core::ffi::c_double,
        &raw mut lde,
        x.offset(nvars as isize) as *mut ::core::ffi::c_double,
        &raw mut one,
    );
    *mode = 1 as ::core::ffi::c_long;
    let mut i_2: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_2 < mg {
        *gmults.offset(i_2 as isize) = 0.0f64;
        i_2 += 1;
    }
    if !(me == n) {
        let mut i_3: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_3 < ma {
            *wb.offset(i_3 as isize) = *b.offset(i_3 as isize);
            i_3 += 1;
        }
        dgelsd_closure_f2c_dgemv(
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut ma,
            &raw mut me,
            &raw mut dmone,
            a.offset((ma * nvars) as isize) as *mut ::core::ffi::c_double,
            &raw mut ma,
            x.offset(nvars as isize) as *mut ::core::ffi::c_double,
            &raw mut one,
            &raw mut done,
            wb,
            &raw mut one,
        );
        let mut j: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while j < nvars {
            let mut i_4: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_4 < ma {
                *a2.offset((i_4 + j * ma) as isize) = *a.offset((i_4 + j * ma) as isize);
                i_4 += 1;
            }
            let mut i_5: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_5 < mg {
                *g2.offset((i_5 + j * mg) as isize) = *g.offset((i_5 + j * mg) as isize);
                i_5 += 1;
            }
            j += 1;
        }
        if mg == 0 as ::core::ffi::c_long {
            let mut lwork: ::core::ffi::c_long =
                ma * nvars + 3 as ::core::ffi::c_long * nvars + 1 as ::core::ffi::c_long;
            let mut wb_orig: *mut ::core::ffi::c_double =
                lsi_scratch.offset(lwork as isize) as *mut ::core::ffi::c_double;
            let mut i_6: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_6 < ma {
                *wb_orig.offset(i_6 as isize) = *wb.offset(i_6 as isize);
                i_6 += 1;
            }
            let mut krank: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            t = sqrt(epsmach);
            slsqp_closure_dgelsy_(
                &raw mut ma,
                &raw mut nvars,
                &raw mut one,
                a2,
                &raw mut ma,
                wb,
                &raw mut ma,
                jw,
                &raw mut t,
                &raw mut krank,
                lsi_scratch,
                &raw mut lwork,
                &raw mut info,
            );
            let mut i_7: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_7 < nvars {
                *x.offset(i_7 as isize) = *wb.offset(i_7 as isize);
                i_7 += 1;
            }
            dgelsd_closure_f2c_dgemv(
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut ma,
                &raw mut nvars,
                &raw mut done,
                a,
                &raw mut ma,
                x,
                &raw mut one,
                &raw mut dmone,
                wb_orig,
                &raw mut one,
            );
            *xnorm = dgelsd_closure_f2c_dnrm2(&raw mut ma, wb_orig, &raw mut one);
            *mode = 7 as ::core::ffi::c_long;
            if krank < nvars {
                return;
            }
            *mode = 1 as ::core::ffi::c_long;
        } else {
            dgelsd_closure_f2c_dgemv(
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut mg,
                &raw mut me,
                &raw mut dmone,
                g.offset((mg * nvars) as isize) as *mut ::core::ffi::c_double,
                &raw mut ldg,
                x.offset(nvars as isize) as *mut ::core::ffi::c_double,
                &raw mut one,
                &raw mut done,
                h,
                &raw mut one,
            );
            lsi(
                ma,
                mg,
                nvars,
                a2,
                wb,
                g2,
                h,
                x,
                lsi_scratch,
                jw,
                xnorm,
                mode,
            );
            let mut i_8: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            while i_8 < mg {
                *gmults.offset(i_8 as isize) = *lsi_scratch.offset(i_8 as isize);
                i_8 += 1;
            }
            if me == 0 as ::core::ffi::c_long {
                return;
            }
            t = dgelsd_closure_f2c_dnrm2(
                &raw mut me,
                x.offset(nvars as isize) as *mut ::core::ffi::c_double,
                &raw mut one,
            );
            *xnorm = hypot(*xnorm, t);
            if *mode != 1 as ::core::ffi::c_long {
                return;
            }
        }
    }
    dgelsd_closure_f2c_dgemv(
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut ma,
        &raw mut n,
        &raw mut done,
        a,
        &raw mut ma,
        x,
        &raw mut one,
        &raw mut dmone,
        b,
        &raw mut one,
    );
    dgelsd_closure_f2c_dgemv(
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut ma,
        &raw mut me,
        &raw mut done,
        a.offset((nvars * ma) as isize) as *mut ::core::ffi::c_double,
        &raw mut ma,
        b,
        &raw mut one,
        &raw mut dzero,
        f,
        &raw mut one,
    );
    dgelsd_closure_f2c_dgemv(
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut mg,
        &raw mut me,
        &raw mut dmone,
        g.offset((nvars * mg) as isize) as *mut ::core::ffi::c_double,
        &raw mut ldg,
        gmults,
        &raw mut one,
        &raw mut done,
        f,
        &raw mut one,
    );
    slsqp_closure_dormr2_(
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut n,
        &raw mut one,
        &raw mut me,
        e,
        &raw mut lde,
        tau,
        x,
        &raw mut n,
        lsi_scratch,
        &raw mut info,
    );
    let mut i_9: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_9 < me {
        *emults.offset(i_9 as isize) = *f.offset(i_9 as isize);
        i_9 += 1;
    }
    slsqp_closure_dtrsv(
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut me,
        e.offset(((n - me) * me) as isize) as *mut ::core::ffi::c_double,
        &raw mut lde,
        emults,
        &raw mut one,
    );
}
unsafe extern "C" fn lsi(
    mut ma: ::core::ffi::c_long,
    mut mg: ::core::ffi::c_long,
    mut n: ::core::ffi::c_long,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut h: *mut ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
    mut buffer: *mut ::core::ffi::c_double,
    mut jw: *mut ::core::ffi::c_long,
    mut xnorm: *mut ::core::ffi::c_double,
    mut mode: *mut ::core::ffi::c_long,
) {
    let mut one: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut tmp_int: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut info: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut done: ::core::ffi::c_double = 1.0f64;
    let mut dmone: ::core::ffi::c_double = -1.0f64;
    let mut tmp_dbl: ::core::ffi::c_double = 0.0f64;
    let epsmach: ::core::ffi::c_double = 2.220446049250313e-16f64;
    tmp_int = if ma < n { ma } else { n };
    dgesdd_closure_dgeqr2_(
        &raw mut ma,
        &raw mut n,
        a,
        &raw mut ma,
        buffer,
        buffer.offset(tmp_int as isize) as *mut ::core::ffi::c_double,
        &raw mut info,
    );
    dgeev_closure_dorm2r_(
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut ma,
        &raw mut one,
        &raw mut tmp_int,
        a,
        &raw mut ma,
        buffer,
        b,
        &raw mut ma,
        buffer.offset(tmp_int as isize) as *mut ::core::ffi::c_double,
        &raw mut info,
    );
    *mode = 5 as ::core::ffi::c_long;
    *xnorm = 0.0f64;
    let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i < tmp_int {
        if !(fabs(*a.offset((i + i * ma) as isize)) >= epsmach) {
            return;
        }
        i += 1;
    }
    slsqp_closure_dtrsm_(
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut mg,
        &raw mut n,
        &raw mut done,
        a,
        &raw mut ma,
        g,
        &raw mut mg,
    );
    dgelsd_closure_f2c_dgemv(
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut mg,
        &raw mut n,
        &raw mut dmone,
        g,
        &raw mut mg,
        b,
        &raw mut one,
        &raw mut done,
        h,
        &raw mut one,
    );
    ldp(mg, n, g, h, x, buffer, jw, xnorm, mode);
    if *mode != 1 as ::core::ffi::c_long {
        return;
    }
    dsyevd_closure_f2c_daxpy(&raw mut n, &raw mut done, b, &raw mut one, x, &raw mut one);
    slsqp_closure_dtrsv(
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut n,
        a,
        &raw mut ma,
        x,
        &raw mut one,
    );
    tmp_int = ma - n;
    tmp_dbl = dgelsd_closure_f2c_dnrm2(
        &raw mut tmp_int,
        b.offset(
            ((if n + 1 as ::core::ffi::c_long > ma {
                ma
            } else {
                n + 1 as ::core::ffi::c_long
            }) - 1 as ::core::ffi::c_long) as isize,
        ) as *mut ::core::ffi::c_double,
        &raw mut one,
    );
    *xnorm = hypot(*xnorm, tmp_dbl);
}
unsafe extern "C" fn ldp(
    mut m: ::core::ffi::c_long,
    mut n: ::core::ffi::c_long,
    mut g: *mut ::core::ffi::c_double,
    mut h: *mut ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
    mut buffer: *mut ::core::ffi::c_double,
    mut indices: *mut ::core::ffi::c_long,
    mut xnorm: *mut ::core::ffi::c_double,
    mut mode: *mut ::core::ffi::c_long,
) {
    let mut one: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut dzero: ::core::ffi::c_double = 0.0f64;
    let mut rnorm: ::core::ffi::c_double = 0.0f64;
    if n <= 0 as ::core::ffi::c_long {
        *mode = 2 as ::core::ffi::c_long;
        return;
    }
    let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i < n {
        *x.offset(i as isize) = 0.0f64;
        i += 1;
    }
    if m == 0 as ::core::ffi::c_long {
        *mode = 1 as ::core::ffi::c_long;
        return;
    }
    let mut a: *mut ::core::ffi::c_double =
        buffer.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    let mut b: *mut ::core::ffi::c_double =
        buffer.offset((m * (n + 1 as ::core::ffi::c_long)) as isize) as *mut ::core::ffi::c_double;
    let mut zz: *mut ::core::ffi::c_double = buffer
        .offset(((m + 1 as ::core::ffi::c_long) * (n + 1 as ::core::ffi::c_long)) as isize)
        as *mut ::core::ffi::c_double;
    let mut y: *mut ::core::ffi::c_double = buffer
        .offset(((m + 2 as ::core::ffi::c_long) * (n + 1 as ::core::ffi::c_long)) as isize)
        as *mut ::core::ffi::c_double;
    let mut w: *mut ::core::ffi::c_double = buffer
        .offset(((m + 2 as ::core::ffi::c_long) * (n + 1 as ::core::ffi::c_long) + m) as isize)
        as *mut ::core::ffi::c_double;
    let mut j: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while j < m {
        let mut i_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_0 < n {
            *a.offset((i_0 + j * (n + 1 as ::core::ffi::c_long)) as isize) =
                *g.offset((j + i_0 * m) as isize);
            i_0 += 1;
        }
        *a.offset((n + j * (n + 1 as ::core::ffi::c_long)) as isize) = *h.offset(j as isize);
        j += 1;
    }
    let mut i_1: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_1 < n {
        *b.offset(i_1 as isize) = 0.0f64;
        i_1 += 1;
    }
    *b.offset(n as isize) = 1.0f64;
    slsqp_nnls(
        n + 1 as ::core::ffi::c_long,
        m,
        a,
        b,
        y,
        w,
        zz,
        indices,
        3 as ::core::ffi::c_long * m,
        &raw mut rnorm,
        mode,
    );
    if *mode != 1 as ::core::ffi::c_long {
        return;
    }
    *mode = 4 as ::core::ffi::c_long;
    if rnorm <= 0.0f64 {
        return;
    }
    let mut fac: ::core::ffi::c_double =
        1.0f64 - dgelsd_closure_f2c_ddot(&raw mut m, h, &raw mut one, y, &raw mut one);
    if !(1.0f64 + fac - 1.0f64 > 0.0f64) {
        return;
    }
    *mode = 1 as ::core::ffi::c_long;
    fac = 1.0f64 / fac;
    dgelsd_closure_f2c_dgemv(
        b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut m,
        &raw mut n,
        &raw mut fac,
        g,
        &raw mut m,
        y,
        &raw mut one,
        &raw mut dzero,
        x,
        &raw mut one,
    );
    *xnorm = dgelsd_closure_f2c_dnrm2(&raw mut n, x, &raw mut one);
    let mut i_2: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_2 < m {
        *buffer.offset(i_2 as isize) = fac * *y.offset(i_2 as isize);
        i_2 += 1;
    }
}
unsafe extern "C" fn ldl_update(
    mut n: ::core::ffi::c_long,
    mut a: *mut ::core::ffi::c_double,
    mut z: *mut ::core::ffi::c_double,
    mut sigma: ::core::ffi::c_double,
    mut w: *mut ::core::ffi::c_double,
) {
    let mut j: ::core::ffi::c_long = 0;
    let mut ij: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let epsmach: ::core::ffi::c_double = 2.220446049250313e-16f64;
    if sigma == 0.0f64 {
        return;
    }
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut beta: ::core::ffi::c_double = 0.;
    let mut delta: ::core::ffi::c_double = 0.;
    let mut gamma: ::core::ffi::c_double = 0.;
    let mut u: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    let mut tp: ::core::ffi::c_double = 0.;
    let mut t: ::core::ffi::c_double = 1.0f64 / sigma;
    if sigma <= 0.0f64 {
        let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i < n {
            *w.offset(i as isize) = *z.offset(i as isize);
            i += 1;
        }
        let mut i_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_0 < n {
            v = *w.offset(i_0 as isize);
            t = t + v * v / *a.offset(ij as isize);
            let mut j_0: ::core::ffi::c_long = i_0 + 1 as ::core::ffi::c_long;
            while j_0 < n {
                ij += 1;
                *w.offset(j_0 as isize) = *w.offset(j_0 as isize) - v * *a.offset(ij as isize);
                j_0 += 1;
            }
            ij += 1;
            i_0 += 1;
        }
        if t >= 0.0f64 {
            t = epsmach / sigma;
        }
        let mut i_1: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        while i_1 < n {
            j = n - i_1 - 1 as ::core::ffi::c_long;
            ij -= i_1 + 1 as ::core::ffi::c_long;
            u = *w.offset(j as isize);
            *w.offset(j as isize) = t;
            t = t - u * u / *a.offset(ij as isize);
            i_1 += 1;
        }
    }
    let mut i_2: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while i_2 < n {
        v = *z.offset(i_2 as isize);
        delta = v / *a.offset(ij as isize);
        tp = if sigma < 0.0f64 {
            *w.offset(i_2 as isize)
        } else {
            t + delta * v
        };
        alpha = tp / t;
        *a.offset(ij as isize) = alpha * *a.offset(ij as isize);
        if i_2 == n - 1 as ::core::ffi::c_long {
            return;
        }
        beta = delta / tp;
        if alpha <= 4.0f64 {
            let mut j_1: ::core::ffi::c_long = i_2 + 1 as ::core::ffi::c_long;
            while j_1 < n {
                ij += 1;
                *z.offset(j_1 as isize) = *z.offset(j_1 as isize) - v * *a.offset(ij as isize);
                *a.offset(ij as isize) = *a.offset(ij as isize) + beta * *z.offset(j_1 as isize);
                j_1 += 1;
            }
        } else {
            gamma = t / tp;
            let mut j_2: ::core::ffi::c_long = i_2 + 1 as ::core::ffi::c_long;
            while j_2 < n {
                ij += 1;
                u = *a.offset(ij as isize);
                *a.offset(ij as isize) = gamma * u + beta * *z.offset(j_2 as isize);
                *z.offset(j_2 as isize) = *z.offset(j_2 as isize) - v * u;
                j_2 += 1;
            }
        }
        ij += 1;
        t = tp;
        i_2 += 1;
    }
}
