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
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dgebal_(
    mut job: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
    mut scale: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut c__: doublereal = 0.;
    let mut f: doublereal = 0.;
    let mut g: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut l: integer = 0;
    let mut m: integer = 0;
    let mut r__: doublereal = 0.;
    let mut s: doublereal = 0.;
    let mut ca: doublereal = 0.;
    let mut ra: doublereal = 0.;
    let mut ica: integer = 0;
    let mut ira: integer = 0;
    let mut iexc: integer = 0;
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
    let mut sfmin1: doublereal = 0.;
    let mut sfmin2: doublereal = 0.;
    let mut sfmax1: doublereal = 0.;
    let mut sfmax2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_idamax"]
        fn f2c_idamax_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> integer;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut noconv: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    scale = scale.offset(-1);
    *info = 0 as integer;
    if lsame__0(
        job,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && lsame__0(
            job,
            b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
        && lsame__0(
            job,
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
        && lsame__0(
            job,
            b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGEBAL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    k = 1 as integer;
    l = *n;
    if !(*n == 0 as ::core::ffi::c_long) {
        if lsame__0(
            job,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                *scale.offset(i__ as isize) = 1.0f64 as doublereal;
                i__ += 1;
            }
        } else {
            if lsame__0(
                job,
                b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                current_block = 4812236721216540944;
            } else {
                current_block = 3123434771885419771;
            }
            '_L120: loop {
                match current_block {
                    3123434771885419771 => {
                        j = l;
                        loop {
                            if !(j >= 1 as ::core::ffi::c_long) {
                                current_block = 14003328289249315753;
                                break;
                            }
                            i__1 = l;
                            i__ = 1 as integer;
                            loop {
                                if !(i__ <= i__1) {
                                    current_block = 10399321362245223758;
                                    break;
                                }
                                if !(i__ == j) {
                                    if *a.offset((j + i__ * a_dim1) as isize) != 0.0f64 {
                                        current_block = 13131896068329595644;
                                        break;
                                    }
                                }
                                i__ += 1;
                            }
                            match current_block {
                                13131896068329595644 => {
                                    j -= 1;
                                }
                                _ => {
                                    m = l;
                                    iexc = 1 as integer;
                                    current_block = 12676171416016431538;
                                    break;
                                }
                            }
                        }
                        loop {
                            match current_block {
                                12676171416016431538 => {
                                    *scale.offset(m as isize) = j as doublereal;
                                    if !(j == m) {
                                        f2c_dswap_0(
                                            &raw mut l,
                                            a.offset(
                                                (j as ::core::ffi::c_long
                                                    * a_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            a.offset(
                                                (m as ::core::ffi::c_long
                                                    * a_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                        i__1 = (*n - k as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as integer;
                                        f2c_dswap_0(
                                            &raw mut i__1,
                                            a.offset((j + k * a_dim1) as isize) as *mut doublereal,
                                            lda,
                                            a.offset((m + k * a_dim1) as isize) as *mut doublereal,
                                            lda,
                                        );
                                    }
                                    match iexc {
                                        2 => {}
                                        1 | _ => {
                                            break;
                                        }
                                    }
                                    k += 1;
                                    current_block = 14003328289249315753;
                                }
                                _ => {
                                    i__1 = l;
                                    j = k;
                                    loop {
                                        if !(j <= i__1) {
                                            current_block = 4812236721216540944;
                                            continue '_L120;
                                        }
                                        i__2 = l;
                                        i__ = k;
                                        loop {
                                            if !(i__ <= i__2) {
                                                current_block = 3392087639489470149;
                                                break;
                                            }
                                            if !(i__ == j) {
                                                if *a.offset((i__ + j * a_dim1) as isize) != 0.0f64
                                                {
                                                    current_block = 7018308795614528254;
                                                    break;
                                                }
                                            }
                                            i__ += 1;
                                        }
                                        match current_block {
                                            7018308795614528254 => {
                                                j += 1;
                                            }
                                            _ => {
                                                m = k;
                                                iexc = 2 as integer;
                                                current_block = 12676171416016431538;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if l == 1 as ::core::ffi::c_long {
                            current_block = 3716987630968956025;
                            break;
                        }
                        l -= 1;
                        current_block = 3123434771885419771;
                    }
                    _ => {
                        i__1 = l;
                        i__ = k;
                        while i__ <= i__1 {
                            *scale.offset(i__ as isize) = 1.0f64 as doublereal;
                            i__ += 1;
                        }
                        if lsame__0(
                            job,
                            b"P\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        ) != 0
                        {
                            current_block = 3716987630968956025;
                            break;
                        } else {
                            current_block = 7178192492338286402;
                            break;
                        }
                    }
                }
            }
            match current_block {
                3716987630968956025 => {}
                _ => {
                    sfmin1 = dlamch__0(
                        b"S\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ) / dlamch__0(
                        b"P\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    sfmax1 = 1.0f64 / sfmin1;
                    sfmin2 = (sfmin1 as ::core::ffi::c_double * 2.0f64) as doublereal;
                    sfmax2 = 1.0f64 / sfmin2;
                    loop {
                        noconv = FALSE_ as logical;
                        i__1 = l;
                        i__ = k;
                        while i__ <= i__1 {
                            c__ = 0.0f64 as doublereal;
                            r__ = 0.0f64 as doublereal;
                            i__2 = l;
                            j = k;
                            while j <= i__2 {
                                if !(j == i__) {
                                    d__1 = *a.offset((j + i__ * a_dim1) as isize);
                                    c__ += (if d__1
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    });
                                    d__1 = *a.offset((i__ + j * a_dim1) as isize);
                                    r__ += (if d__1
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    });
                                }
                                j += 1;
                            }
                            ica = f2c_idamax_0(
                                &raw mut l,
                                a.offset(
                                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                            );
                            d__1 = *a.offset((ica + i__ * a_dim1) as isize);
                            ca = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) as doublereal;
                            i__2 = (*n - k as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                as integer;
                            ira = f2c_idamax_0(
                                &raw mut i__2,
                                a.offset((i__ + k * a_dim1) as isize) as *mut doublereal,
                                lda,
                            );
                            d__1 = *a.offset((i__ + (ira + k - 1 as integer) * a_dim1) as isize);
                            ra = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) as doublereal;
                            if !(c__ == 0.0f64 || r__ == 0.0f64) {
                                g = (r__ as ::core::ffi::c_double / 2.0f64) as doublereal;
                                f = 1.0f64 as doublereal;
                                s = c__ + r__;
                                loop {
                                    d__1 = (if f >= c__ {
                                        f as ::core::ffi::c_double
                                    } else {
                                        c__ as ::core::ffi::c_double
                                    }) as doublereal;
                                    d__2 = (if r__ <= g {
                                        r__ as ::core::ffi::c_double
                                    } else {
                                        g as ::core::ffi::c_double
                                    }) as doublereal;
                                    if c__ >= g
                                        || (if d__1 >= ca {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            ca as ::core::ffi::c_double
                                        }) >= sfmax2
                                        || (if d__2 <= ra {
                                            d__2 as ::core::ffi::c_double
                                        } else {
                                            ra as ::core::ffi::c_double
                                        }) <= sfmin2
                                    {
                                        break;
                                    }
                                    f *= 2.0f64;
                                    c__ *= 2.0f64;
                                    ca *= 2.0f64;
                                    r__ /= 2.0f64;
                                    g /= 2.0f64;
                                    ra /= 2.0f64;
                                }
                                g = (c__ as ::core::ffi::c_double / 2.0f64) as doublereal;
                                loop {
                                    d__1 = (if f <= c__ {
                                        f as ::core::ffi::c_double
                                    } else {
                                        c__ as ::core::ffi::c_double
                                    }) as doublereal;
                                    d__1 = (if d__1 <= g {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        g as ::core::ffi::c_double
                                    }) as doublereal;
                                    if g < r__
                                        || (if r__ >= ra {
                                            r__ as ::core::ffi::c_double
                                        } else {
                                            ra as ::core::ffi::c_double
                                        }) >= sfmax2
                                        || (if d__1 <= ca {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            ca as ::core::ffi::c_double
                                        }) <= sfmin2
                                    {
                                        break;
                                    }
                                    f /= 2.0f64;
                                    c__ /= 2.0f64;
                                    g /= 2.0f64;
                                    ca /= 2.0f64;
                                    r__ *= 2.0f64;
                                    ra *= 2.0f64;
                                }
                                if !(c__ + r__ >= s as ::core::ffi::c_double * 0.95f64) {
                                    if f < 1.0f64 && *scale.offset(i__ as isize) < 1.0f64 {
                                        if f * *scale.offset(i__ as isize) <= sfmin1 {
                                            current_block = 17441561948628420366;
                                        } else {
                                            current_block = 6733407218104445560;
                                        }
                                    } else {
                                        current_block = 6733407218104445560;
                                    }
                                    match current_block {
                                        17441561948628420366 => {}
                                        _ => {
                                            if f > 1.0f64 && *scale.offset(i__ as isize) > 1.0f64 {
                                                if *scale.offset(i__ as isize) >= sfmax1 / f {
                                                    current_block = 17441561948628420366;
                                                } else {
                                                    current_block = 12079920068676227593;
                                                }
                                            } else {
                                                current_block = 12079920068676227593;
                                            }
                                            match current_block {
                                                17441561948628420366 => {}
                                                _ => {
                                                    g = 1.0f64 / f;
                                                    let ref mut fresh0 =
                                                        *scale.offset(i__ as isize);
                                                    *fresh0 *= f as ::core::ffi::c_double;
                                                    noconv = TRUE_ as logical;
                                                    i__2 = (*n - k as ::core::ffi::c_long
                                                        + 1 as ::core::ffi::c_long)
                                                        as integer;
                                                    f2c_dscal_0(
                                                        &raw mut i__2,
                                                        &raw mut g,
                                                        a.offset((i__ + k * a_dim1) as isize)
                                                            as *mut doublereal,
                                                        lda,
                                                    );
                                                    f2c_dscal_0(
                                                        &raw mut l,
                                                        &raw mut f,
                                                        a.offset(
                                                            (i__ as ::core::ffi::c_long
                                                                * a_dim1 as ::core::ffi::c_long
                                                                + 1 as ::core::ffi::c_long)
                                                                as isize,
                                                        )
                                                            as *mut doublereal,
                                                        &raw mut c__1,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            i__ += 1;
                        }
                        if !(noconv != 0) {
                            break;
                        }
                    }
                }
            }
        }
    }
    *ilo = k;
    *ihi = l;
    return 0 as ::core::ffi::c_int;
}
