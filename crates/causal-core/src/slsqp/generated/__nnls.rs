extern "C" {
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    #[link_name = "slsqp_closure_nextafter"]
    fn nextafter(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn dgelsd_closure_f2c_ddot(
        n: *mut ::core::ffi::c_long,
        dx: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
        dy: *mut ::core::ffi::c_double,
        incy: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_double;
    fn dgeev_closure_dlarf_(
        side: *mut ::core::ffi::c_char,
        m: *mut ::core::ffi::c_long,
        n: *mut ::core::ffi::c_long,
        v: *mut ::core::ffi::c_double,
        incv: *mut ::core::ffi::c_long,
        tau: *mut ::core::ffi::c_double,
        c: *mut ::core::ffi::c_double,
        ldc: *mut ::core::ffi::c_long,
        work: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dlarfgp_(
        n: *mut ::core::ffi::c_long,
        alpha: *mut ::core::ffi::c_double,
        x: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
        tau: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn slsqp_closure_dlartgp_(
        f: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        cs: *mut ::core::ffi::c_double,
        sn: *mut ::core::ffi::c_double,
        r: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn dgelsd_closure_f2c_dnrm2(
        n: *mut ::core::ffi::c_long,
        x: *mut ::core::ffi::c_double,
        incx: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn slsqp_nnls(
    m: ::core::ffi::c_long,
    n: ::core::ffi::c_long,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
    mut w: *mut ::core::ffi::c_double,
    mut zz: *mut ::core::ffi::c_double,
    mut indices: *mut ::core::ffi::c_long,
    maxiter: ::core::ffi::c_long,
    mut rnorm: *mut ::core::ffi::c_double,
    mut info: *mut ::core::ffi::c_long,
) {
    let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut ii: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut ip: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut indz: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut iteration: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut iz: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut izmax: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut j: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut jj: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut k: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut one: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut tmpint: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut tau: ::core::ffi::c_double = 0.0f64;
    let mut unorm: ::core::ffi::c_double = 0.0f64;
    let mut ztest: ::core::ffi::c_double = 0.;
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut cc: ::core::ffi::c_double = 0.;
    let mut ss: ::core::ffi::c_double = 0.;
    let mut wmax: ::core::ffi::c_double = 0.;
    let mut T: ::core::ffi::c_double = 0.;
    let mut tmp_work: ::core::ffi::c_double = 0.;
    let mut pivot: ::core::ffi::c_double = 1.0f64;
    let mut pivot2: ::core::ffi::c_double = 0.0f64;
    let mut tmp: ::core::ffi::c_double = 0.0f64;
    let mut spacing: ::core::ffi::c_double = 0.0f64;
    *info = 1 as ::core::ffi::c_long;
    if m <= 0 as ::core::ffi::c_long || n <= 0 as ::core::ffi::c_long {
        *info = 2 as ::core::ffi::c_long;
        return;
    }
    i = 0 as ::core::ffi::c_long;
    while i < n {
        *indices.offset(i as isize) = i;
        i += 1;
    }
    i = 0 as ::core::ffi::c_long;
    while i < n {
        *x.offset(i as isize) = 0.0f64;
        i += 1;
    }
    's_61: while indz < (if m < n { m } else { n }) {
        i = indz;
        while i < n {
            j = *indices.offset(i as isize);
            tmpint = m - indz;
            *w.offset(j as isize) = dgelsd_closure_f2c_ddot(
                &raw mut tmpint,
                a.offset((indz + j * m) as isize) as *mut ::core::ffi::c_double,
                &raw mut one,
                b.offset(indz as isize) as *mut ::core::ffi::c_double,
                &raw mut one,
            );
            i += 1;
        }
        loop {
            wmax = 0.0f64;
            k = indz;
            while k < n {
                j = *indices.offset(k as isize);
                if *w.offset(j as isize) > wmax {
                    wmax = *w.offset(j as isize);
                    izmax = k;
                }
                k += 1;
            }
            if wmax <= 0.0f64 {
                break 's_61;
            }
            iz = izmax;
            j = *indices.offset(iz as isize);
            pivot = *a.offset((indz + j * m) as isize);
            tmpint = m - indz;
            slsqp_closure_dlarfgp_(
                &raw mut tmpint,
                &raw mut pivot,
                a.offset((indz + 1 as ::core::ffi::c_long + j * m) as isize)
                    as *mut ::core::ffi::c_double,
                &raw mut one,
                &raw mut tau,
            );
            unorm = if indz > 0 as ::core::ffi::c_long {
                dgelsd_closure_f2c_dnrm2(
                    &raw mut indz,
                    a.offset((j * m) as isize) as *mut ::core::ffi::c_double,
                    &raw mut one,
                )
            } else {
                0.0f64
            };
            spacing = if unorm > 0.0f64 {
                nextafter(
                    unorm,
                    2 as ::core::ffi::c_int as ::core::ffi::c_double * unorm,
                ) - unorm
            } else {
                0.0f64
            };
            if fabs(pivot) > 100.0f64 * spacing {
                i = 0 as ::core::ffi::c_long;
                while i < m {
                    *zz.offset(i as isize) = *b.offset(i as isize);
                    i += 1;
                }
                tmpint = m - indz;
                pivot2 = *a.offset((indz + j * m) as isize);
                *a.offset((indz + j * m) as isize) = 1.0f64;
                dgeev_closure_dlarf_(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut tmpint,
                    &raw mut one,
                    a.offset((indz + j * m) as isize) as *mut ::core::ffi::c_double,
                    &raw mut one,
                    &raw mut tau,
                    zz.offset(indz as isize) as *mut ::core::ffi::c_double,
                    &raw mut tmpint,
                    &raw mut tmp_work,
                );
                ztest = *zz.offset(indz as isize) / pivot;
                if ztest > 0.0f64 {
                    break;
                }
                *a.offset((indz + j * m) as isize) = pivot2;
            }
            *w.offset(j as isize) = 0.0f64;
        }
        i = 0 as ::core::ffi::c_long;
        while i < m {
            *b.offset(i as isize) = *zz.offset(i as isize);
            i += 1;
        }
        *indices.offset(iz as isize) = *indices.offset(indz as isize);
        *indices.offset(indz as isize) = j;
        indz += 1;
        if indz < n {
            tmpint = m - indz + 1 as ::core::ffi::c_long;
            k = indz;
            while k < n {
                jj = *indices.offset(k as isize);
                dgeev_closure_dlarf_(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut tmpint,
                    &raw mut one,
                    a.offset((indz - 1 as ::core::ffi::c_long + j * m) as isize)
                        as *mut ::core::ffi::c_double,
                    &raw mut one,
                    &raw mut tau,
                    a.offset((indz - 1 as ::core::ffi::c_long + jj * m) as isize)
                        as *mut ::core::ffi::c_double,
                    &raw mut tmpint,
                    &raw mut tmp_work,
                );
                k += 1;
            }
        }
        *a.offset((indz - 1 as ::core::ffi::c_long + j * m) as isize) = pivot;
        if indz < m {
            i = indz;
            while i < m {
                *a.offset((j * m + i) as isize) = 0.0f64;
                i += 1;
            }
        }
        *w.offset(j as isize) = 0.0f64;
        k = 0 as ::core::ffi::c_long;
        while k < indz {
            ip = indz - 1 as ::core::ffi::c_long - k;
            if k != 0 as ::core::ffi::c_long {
                i = 0 as ::core::ffi::c_long;
                while i <= ip {
                    *zz.offset(i as isize) = *zz.offset(i as isize)
                        - *a.offset((i + jj * m) as isize)
                            * *zz.offset((ip + 1 as ::core::ffi::c_long) as isize);
                    i += 1;
                }
            }
            jj = *indices.offset(ip as isize);
            *zz.offset(ip as isize) = *zz.offset(ip as isize) / *a.offset((ip + jj * m) as isize);
            k += 1;
        }
        loop {
            iteration += 1;
            if iteration >= maxiter {
                *info = 3 as ::core::ffi::c_long;
                break 's_61;
            } else {
                alpha = 2.0f64;
                ip = 0 as ::core::ffi::c_long;
                while ip < indz {
                    k = *indices.offset(ip as isize);
                    if *zz.offset(ip as isize) <= 0.0f64 {
                        T = -*x.offset(k as isize)
                            / (*zz.offset(ip as isize) - *x.offset(k as isize));
                        if alpha > T {
                            alpha = T;
                            jj = ip;
                        }
                    }
                    ip += 1;
                }
                if alpha == 2.0f64 {
                    break;
                }
                ip = 0 as ::core::ffi::c_long;
                while ip < indz {
                    k = *indices.offset(ip as isize);
                    *x.offset(k as isize) = *x.offset(k as isize)
                        + alpha * (*zz.offset(ip as isize) - *x.offset(k as isize));
                    ip += 1;
                }
                i = *indices.offset(jj as isize);
                loop {
                    *x.offset(i as isize) = 0.0f64;
                    if jj != indz - 1 as ::core::ffi::c_long {
                        jj += 1;
                        j = jj;
                        while j < indz {
                            ii = *indices.offset(j as isize);
                            *indices.offset((j - 1 as ::core::ffi::c_long) as isize) = ii;
                            slsqp_closure_dlartgp_(
                                a.offset((j - 1 as ::core::ffi::c_long + ii * m) as isize)
                                    as *mut ::core::ffi::c_double,
                                a.offset((j + ii * m) as isize) as *mut ::core::ffi::c_double,
                                &raw mut cc,
                                &raw mut ss,
                                a.offset((j - 1 as ::core::ffi::c_long + ii * m) as isize)
                                    as *mut ::core::ffi::c_double,
                            );
                            *a.offset((j + ii * m) as isize) = 0.0f64;
                            k = 0 as ::core::ffi::c_long;
                            while k < n {
                                if k != ii {
                                    tmp =
                                        *a.offset((j - 1 as ::core::ffi::c_long + k * m) as isize);
                                    *a.offset((j - 1 as ::core::ffi::c_long + k * m) as isize) =
                                        cc * tmp + ss * *a.offset((j + k * m) as isize);
                                    *a.offset((j + k * m) as isize) =
                                        -ss * tmp + cc * *a.offset((j + k * m) as isize);
                                }
                                k += 1;
                            }
                            tmp = *b.offset((j - 1 as ::core::ffi::c_long) as isize);
                            *b.offset((j - 1 as ::core::ffi::c_long) as isize) =
                                cc * tmp + ss * *b.offset(j as isize);
                            *b.offset(j as isize) = -ss * tmp + cc * *b.offset(j as isize);
                            j += 1;
                        }
                    }
                    indz -= 1;
                    *indices.offset(indz as isize) = i;
                    let mut nobreak: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                    jj = 0 as ::core::ffi::c_long;
                    while jj < indz {
                        i = *indices.offset(jj as isize);
                        if *x.offset(i as isize) <= 0.0f64 {
                            break;
                        }
                        if jj == indz - 1 as ::core::ffi::c_long {
                            nobreak = 1 as ::core::ffi::c_long;
                        }
                        jj += 1;
                    }
                    if nobreak != 0 {
                        break;
                    }
                }
                i = 0 as ::core::ffi::c_long;
                while i < m {
                    *zz.offset(i as isize) = *b.offset(i as isize);
                    i += 1;
                }
                k = 0 as ::core::ffi::c_long;
                while k < indz {
                    ip = indz - 1 as ::core::ffi::c_long - k;
                    if k != 0 as ::core::ffi::c_long {
                        i = 0 as ::core::ffi::c_long;
                        while i <= ip {
                            *zz.offset(i as isize) = *zz.offset(i as isize)
                                - *a.offset((i + jj * m) as isize)
                                    * *zz.offset((ip + 1 as ::core::ffi::c_long) as isize);
                            i += 1;
                        }
                    }
                    jj = *indices.offset(ip as isize);
                    *zz.offset(ip as isize) =
                        *zz.offset(ip as isize) / *a.offset((ip + jj * m) as isize);
                    k += 1;
                }
            }
        }
        k = 0 as ::core::ffi::c_long;
        while k < indz {
            i = *indices.offset(k as isize);
            *x.offset(i as isize) = *zz.offset(k as isize);
            k += 1;
        }
    }
    if indz < m {
        tmpint = m - indz;
        *rnorm = dgelsd_closure_f2c_dnrm2(
            &raw mut tmpint,
            b.offset(indz as isize) as *mut ::core::ffi::c_double,
            &raw mut one,
        );
    } else {
        i = 0 as ::core::ffi::c_long;
        while i < n {
            *w.offset(i as isize) = 0.0f64;
            i += 1;
        }
        *rnorm = 0.0f64;
    };
}
