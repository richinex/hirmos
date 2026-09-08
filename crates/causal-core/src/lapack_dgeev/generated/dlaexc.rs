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
pub const FALSE_: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut c__1: integer = 1 as integer;
static mut c__4: integer = 4 as integer;
static mut c_false: logical = FALSE_ as logical;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c__2: integer = 2 as integer;
static mut c__3: integer = 3 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlaexc_(
    mut wantq: *mut logical,
    mut n: *mut integer,
    mut t: *mut doublereal,
    mut ldt: *mut integer,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut j1: *mut integer,
    mut n1: *mut integer,
    mut n2: *mut integer,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut t_dim1: integer = 0;
    let mut t_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__: [doublereal; 16] = [0.; 16];
    let mut k: integer = 0;
    let mut u: [doublereal; 3] = [0.; 3];
    let mut x: [doublereal; 4] = [0.; 4];
    let mut j2: integer = 0;
    let mut j3: integer = 0;
    let mut j4: integer = 0;
    let mut u1: [doublereal; 3] = [0.; 3];
    let mut u2: [doublereal; 3] = [0.; 3];
    let mut nd: integer = 0;
    let mut cs: doublereal = 0.;
    let mut t11: doublereal = 0.;
    let mut t22: doublereal = 0.;
    let mut t33: doublereal = 0.;
    let mut sn: doublereal = 0.;
    let mut wi1: doublereal = 0.;
    let mut wi2: doublereal = 0.;
    let mut wr1: doublereal = 0.;
    let mut wr2: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut tau: doublereal = 0.;
    let mut tau1: doublereal = 0.;
    let mut tau2: doublereal = 0.;
    let mut ierr: integer = 0;
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
    let mut scale: doublereal = 0.;
    let mut dnorm: doublereal = 0.;
    let mut xnorm: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_dlanv2_"]
        fn dlanv2__0(
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
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlasy2_"]
        fn dlasy2__0(
            _: *mut logical,
            _: *mut logical,
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
            _: *mut doublereal,
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
        #[link_name = "dgeev_closure_dlange_"]
        fn dlange__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlarfg_"]
        fn dlarfg__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
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
        #[link_name = "dgeev_closure_dlarfx_"]
        fn dlarfx__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    let mut thresh: doublereal = 0.;
    let mut smlnum: doublereal = 0.;
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    work = work.offset(-1);
    *info = 0 as integer;
    if *n == 0 as ::core::ffi::c_long
        || *n1 == 0 as ::core::ffi::c_long
        || *n2 == 0 as ::core::ffi::c_long
    {
        return 0 as ::core::ffi::c_int;
    }
    if *j1 + *n1 > *n {
        return 0 as ::core::ffi::c_int;
    }
    j2 = (*j1 + 1 as ::core::ffi::c_long) as integer;
    j3 = (*j1 + 2 as ::core::ffi::c_long) as integer;
    j4 = (*j1 + 3 as ::core::ffi::c_long) as integer;
    if *n1 == 1 as ::core::ffi::c_long && *n2 == 1 as ::core::ffi::c_long {
        t11 = *t.offset((*j1 + *j1 * t_dim1) as isize);
        t22 = *t.offset((j2 + j2 * t_dim1) as isize);
        d__1 = t22 - t11;
        dlartg__0(
            t.offset((*j1 + j2 * t_dim1) as isize) as *mut doublereal,
            &raw mut d__1,
            &raw mut cs,
            &raw mut sn,
            &raw mut temp,
        );
        if j3 <= *n {
            i__1 = (*n - *j1 - 1 as ::core::ffi::c_long) as integer;
            f2c_drot_0(
                &raw mut i__1,
                t.offset((*j1 + j3 * t_dim1) as isize) as *mut doublereal,
                ldt,
                t.offset((j2 + j3 * t_dim1) as isize) as *mut doublereal,
                ldt,
                &raw mut cs,
                &raw mut sn,
            );
        }
        i__1 = (*j1 - 1 as ::core::ffi::c_long) as integer;
        f2c_drot_0(
            &raw mut i__1,
            t.offset((*j1 * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            &raw mut c__1,
            t.offset(
                (j2 as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
            &raw mut cs,
            &raw mut sn,
        );
        *t.offset((*j1 + *j1 * t_dim1) as isize) = t22;
        *t.offset((j2 + j2 * t_dim1) as isize) = t11;
        if *wantq != 0 {
            f2c_drot_0(
                n,
                q.offset((*j1 * q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
                q.offset(
                    (j2 as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                &raw mut cs,
                &raw mut sn,
            );
        }
    } else {
        nd = *n1 + *n2;
        dlacpy__0(
            b"Full\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut nd,
            &raw mut nd,
            t.offset((*j1 + *j1 * t_dim1) as isize) as *mut doublereal,
            ldt,
            &raw mut d__ as *mut doublereal,
            &raw mut c__4,
        );
        dnorm = dlange__0(
            b"Max\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut nd,
            &raw mut nd,
            &raw mut d__ as *mut doublereal,
            &raw mut c__4,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        eps = dlamch__0(
            b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        smlnum = dlamch__0(
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) / eps;
        d__1 = eps * 10.0f64 * dnorm;
        thresh = (if d__1 >= smlnum {
            d__1 as ::core::ffi::c_double
        } else {
            smlnum as ::core::ffi::c_double
        }) as doublereal;
        dlasy2__0(
            &raw mut c_false,
            &raw mut c_false,
            &raw mut c_n1,
            n1,
            n2,
            &raw mut d__ as *mut doublereal,
            &raw mut c__4,
            (&raw mut d__ as *mut doublereal).offset(
                (*n1 + 1 as ::core::ffi::c_long
                    + ((*n1 + 1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__4,
            (&raw mut d__ as *mut doublereal).offset(
                (((*n1 + 1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - 4 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__4,
            &raw mut scale,
            &raw mut x as *mut doublereal,
            &raw mut c__2,
            &raw mut xnorm,
            &raw mut ierr,
        );
        k = (*n1 + *n1 + *n2 - 3 as ::core::ffi::c_long) as integer;
        match k {
            2 => {
                u[0 as ::core::ffi::c_int as usize] = -x[0 as ::core::ffi::c_int as usize];
                u[1 as ::core::ffi::c_int as usize] = -x[1 as ::core::ffi::c_int as usize];
                u[2 as ::core::ffi::c_int as usize] = scale;
                dlarfg__0(
                    &raw mut c__3,
                    &raw mut u as *mut doublereal,
                    (&raw mut u as *mut doublereal).offset(1 as ::core::ffi::c_int as isize)
                        as *mut doublereal,
                    &raw mut c__1,
                    &raw mut tau,
                );
                u[0 as ::core::ffi::c_int as usize] = 1.0f64 as doublereal;
                t33 = *t.offset((j3 + j3 * t_dim1) as isize);
                dlarfx__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__3,
                    &raw mut c__3,
                    &raw mut u as *mut doublereal,
                    &raw mut tau,
                    &raw mut d__ as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                dlarfx__0(
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__3,
                    &raw mut c__3,
                    &raw mut u as *mut doublereal,
                    &raw mut tau,
                    &raw mut d__ as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                d__2 = (if d__[1 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[1 as ::core::ffi::c_int as usize]
                } else {
                    -d__[1 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__3 = (if d__[2 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[2 as ::core::ffi::c_int as usize]
                } else {
                    -d__[2 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__2 = (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) as doublereal;
                d__1 = d__[0 as ::core::ffi::c_int as usize] - t33;
                d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                if (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) > thresh
                {
                    current_block = 12950274254465297480;
                } else {
                    dlarfx__0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut j3,
                        &raw mut c__3,
                        &raw mut u as *mut doublereal,
                        &raw mut tau,
                        t.offset(
                            (*j1 * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                as isize,
                        ) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    i__1 = *n - *j1;
                    dlarfx__0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__3,
                        &raw mut i__1,
                        &raw mut u as *mut doublereal,
                        &raw mut tau,
                        t.offset((*j1 + j2 * t_dim1) as isize) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    *t.offset((*j1 + *j1 * t_dim1) as isize) = t33;
                    *t.offset((j2 + *j1 * t_dim1) as isize) = 0.0f64 as doublereal;
                    *t.offset((j3 + *j1 * t_dim1) as isize) = 0.0f64 as doublereal;
                    if *wantq != 0 {
                        dlarfx__0(
                            b"R\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            &raw mut c__3,
                            &raw mut u as *mut doublereal,
                            &raw mut tau,
                            q.offset(
                                (*j1 * q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldq,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        );
                    }
                    current_block = 12366136100333888408;
                }
            }
            3 => {
                u1[0 as ::core::ffi::c_int as usize] = -x[0 as ::core::ffi::c_int as usize];
                u1[1 as ::core::ffi::c_int as usize] = -x[1 as ::core::ffi::c_int as usize];
                u1[2 as ::core::ffi::c_int as usize] = scale;
                dlarfg__0(
                    &raw mut c__3,
                    &raw mut u1 as *mut doublereal,
                    (&raw mut u1 as *mut doublereal).offset(1 as ::core::ffi::c_int as isize)
                        as *mut doublereal,
                    &raw mut c__1,
                    &raw mut tau1,
                );
                u1[0 as ::core::ffi::c_int as usize] = 1.0f64 as doublereal;
                temp = -tau1
                    * (x[2 as ::core::ffi::c_int as usize]
                        + u1[1 as ::core::ffi::c_int as usize]
                            * x[3 as ::core::ffi::c_int as usize]);
                u2[0 as ::core::ffi::c_int as usize] = -temp * u1[1 as ::core::ffi::c_int as usize]
                    - x[3 as ::core::ffi::c_int as usize];
                u2[1 as ::core::ffi::c_int as usize] = -temp * u1[2 as ::core::ffi::c_int as usize];
                u2[2 as ::core::ffi::c_int as usize] = scale;
                dlarfg__0(
                    &raw mut c__3,
                    &raw mut u2 as *mut doublereal,
                    (&raw mut u2 as *mut doublereal).offset(1 as ::core::ffi::c_int as isize)
                        as *mut doublereal,
                    &raw mut c__1,
                    &raw mut tau2,
                );
                u2[0 as ::core::ffi::c_int as usize] = 1.0f64 as doublereal;
                dlarfx__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__3,
                    &raw mut c__4,
                    &raw mut u1 as *mut doublereal,
                    &raw mut tau1,
                    &raw mut d__ as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                dlarfx__0(
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__4,
                    &raw mut c__3,
                    &raw mut u1 as *mut doublereal,
                    &raw mut tau1,
                    &raw mut d__ as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                dlarfx__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__3,
                    &raw mut c__4,
                    &raw mut u2 as *mut doublereal,
                    &raw mut tau2,
                    (&raw mut d__ as *mut doublereal).offset(1 as ::core::ffi::c_int as isize)
                        as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                dlarfx__0(
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__4,
                    &raw mut c__3,
                    &raw mut u2 as *mut doublereal,
                    &raw mut tau2,
                    (&raw mut d__ as *mut doublereal).offset(4 as ::core::ffi::c_int as isize)
                        as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                d__1 = (if d__[2 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[2 as ::core::ffi::c_int as usize]
                } else {
                    -d__[2 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__2 = (if d__[6 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[6 as ::core::ffi::c_int as usize]
                } else {
                    -d__[6 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__1 = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = (if d__[3 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[3 as ::core::ffi::c_int as usize]
                } else {
                    -d__[3 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__1 = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = (if d__[7 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[7 as ::core::ffi::c_int as usize]
                } else {
                    -d__[7 as ::core::ffi::c_int as usize]
                }) as doublereal;
                if (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) > thresh
                {
                    current_block = 12950274254465297480;
                } else {
                    i__1 = (*n - *j1 + 1 as ::core::ffi::c_long) as integer;
                    dlarfx__0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__3,
                        &raw mut i__1,
                        &raw mut u1 as *mut doublereal,
                        &raw mut tau1,
                        t.offset((*j1 + *j1 * t_dim1) as isize) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    dlarfx__0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut j4,
                        &raw mut c__3,
                        &raw mut u1 as *mut doublereal,
                        &raw mut tau1,
                        t.offset(
                            (*j1 * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                as isize,
                        ) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    i__1 = (*n - *j1 + 1 as ::core::ffi::c_long) as integer;
                    dlarfx__0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__3,
                        &raw mut i__1,
                        &raw mut u2 as *mut doublereal,
                        &raw mut tau2,
                        t.offset((j2 + *j1 * t_dim1) as isize) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    dlarfx__0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut j4,
                        &raw mut c__3,
                        &raw mut u2 as *mut doublereal,
                        &raw mut tau2,
                        t.offset(
                            (j2 as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    *t.offset((j3 + *j1 * t_dim1) as isize) = 0.0f64 as doublereal;
                    *t.offset((j3 + j2 * t_dim1) as isize) = 0.0f64 as doublereal;
                    *t.offset((j4 + *j1 * t_dim1) as isize) = 0.0f64 as doublereal;
                    *t.offset((j4 + j2 * t_dim1) as isize) = 0.0f64 as doublereal;
                    if *wantq != 0 {
                        dlarfx__0(
                            b"R\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            &raw mut c__3,
                            &raw mut u1 as *mut doublereal,
                            &raw mut tau1,
                            q.offset(
                                (*j1 * q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldq,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        );
                        dlarfx__0(
                            b"R\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            &raw mut c__3,
                            &raw mut u2 as *mut doublereal,
                            &raw mut tau2,
                            q.offset(
                                (j2 as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldq,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        );
                    }
                    current_block = 12366136100333888408;
                }
            }
            1 | _ => {
                u[0 as ::core::ffi::c_int as usize] = scale;
                u[1 as ::core::ffi::c_int as usize] = x[0 as ::core::ffi::c_int as usize];
                u[2 as ::core::ffi::c_int as usize] = x[2 as ::core::ffi::c_int as usize];
                dlarfg__0(
                    &raw mut c__3,
                    (&raw mut u as *mut doublereal).offset(2 as ::core::ffi::c_int as isize)
                        as *mut doublereal,
                    &raw mut u as *mut doublereal,
                    &raw mut c__1,
                    &raw mut tau,
                );
                u[2 as ::core::ffi::c_int as usize] = 1.0f64 as doublereal;
                t11 = *t.offset((*j1 + *j1 * t_dim1) as isize);
                dlarfx__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__3,
                    &raw mut c__3,
                    &raw mut u as *mut doublereal,
                    &raw mut tau,
                    &raw mut d__ as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                dlarfx__0(
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__3,
                    &raw mut c__3,
                    &raw mut u as *mut doublereal,
                    &raw mut tau,
                    &raw mut d__ as *mut doublereal,
                    &raw mut c__4,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                d__2 = (if d__[2 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[2 as ::core::ffi::c_int as usize]
                } else {
                    -d__[2 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__3 = (if d__[6 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    d__[6 as ::core::ffi::c_int as usize]
                } else {
                    -d__[6 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__2 = (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) as doublereal;
                d__1 = d__[10 as ::core::ffi::c_int as usize] - t11;
                d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                if (if d__2 >= d__3 {
                    d__2 as ::core::ffi::c_double
                } else {
                    d__3 as ::core::ffi::c_double
                }) > thresh
                {
                    current_block = 12950274254465297480;
                } else {
                    i__1 = (*n - *j1 + 1 as ::core::ffi::c_long) as integer;
                    dlarfx__0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__3,
                        &raw mut i__1,
                        &raw mut u as *mut doublereal,
                        &raw mut tau,
                        t.offset((*j1 + *j1 * t_dim1) as isize) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    dlarfx__0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut j2,
                        &raw mut c__3,
                        &raw mut u as *mut doublereal,
                        &raw mut tau,
                        t.offset(
                            (*j1 * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                as isize,
                        ) as *mut doublereal,
                        ldt,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    );
                    *t.offset((j3 + *j1 * t_dim1) as isize) = 0.0f64 as doublereal;
                    *t.offset((j3 + j2 * t_dim1) as isize) = 0.0f64 as doublereal;
                    *t.offset((j3 + j3 * t_dim1) as isize) = t11;
                    if *wantq != 0 {
                        dlarfx__0(
                            b"R\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            &raw mut c__3,
                            &raw mut u as *mut doublereal,
                            &raw mut tau,
                            q.offset(
                                (*j1 * q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldq,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        );
                    }
                    current_block = 12366136100333888408;
                }
            }
        }
        match current_block {
            12950274254465297480 => {
                *info = 1 as integer;
                return 0 as ::core::ffi::c_int;
            }
            _ => {
                if *n2 == 2 as ::core::ffi::c_long {
                    dlanv2__0(
                        t.offset((*j1 + *j1 * t_dim1) as isize) as *mut doublereal,
                        t.offset((*j1 + j2 * t_dim1) as isize) as *mut doublereal,
                        t.offset((j2 + *j1 * t_dim1) as isize) as *mut doublereal,
                        t.offset((j2 + j2 * t_dim1) as isize) as *mut doublereal,
                        &raw mut wr1,
                        &raw mut wi1,
                        &raw mut wr2,
                        &raw mut wi2,
                        &raw mut cs,
                        &raw mut sn,
                    );
                    i__1 = (*n - *j1 - 1 as ::core::ffi::c_long) as integer;
                    f2c_drot_0(
                        &raw mut i__1,
                        t.offset((*j1 + (*j1 + 2 as integer) * t_dim1) as isize) as *mut doublereal,
                        ldt,
                        t.offset((j2 + (*j1 + 2 as integer) * t_dim1) as isize) as *mut doublereal,
                        ldt,
                        &raw mut cs,
                        &raw mut sn,
                    );
                    i__1 = (*j1 - 1 as ::core::ffi::c_long) as integer;
                    f2c_drot_0(
                        &raw mut i__1,
                        t.offset(
                            (*j1 * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        t.offset(
                            (j2 as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut cs,
                        &raw mut sn,
                    );
                    if *wantq != 0 {
                        f2c_drot_0(
                            n,
                            q.offset(
                                (*j1 * q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            q.offset(
                                (j2 as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            &raw mut cs,
                            &raw mut sn,
                        );
                    }
                }
                if *n1 == 2 as ::core::ffi::c_long {
                    j3 = *j1 + *n2;
                    j4 = (j3 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    dlanv2__0(
                        t.offset((j3 + j3 * t_dim1) as isize) as *mut doublereal,
                        t.offset((j3 + j4 * t_dim1) as isize) as *mut doublereal,
                        t.offset((j4 + j3 * t_dim1) as isize) as *mut doublereal,
                        t.offset((j4 + j4 * t_dim1) as isize) as *mut doublereal,
                        &raw mut wr1,
                        &raw mut wi1,
                        &raw mut wr2,
                        &raw mut wi2,
                        &raw mut cs,
                        &raw mut sn,
                    );
                    if j3 as ::core::ffi::c_long + 2 as ::core::ffi::c_long <= *n {
                        i__1 =
                            (*n - j3 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        f2c_drot_0(
                            &raw mut i__1,
                            t.offset((j3 + (j3 + 2 as integer) * t_dim1) as isize)
                                as *mut doublereal,
                            ldt,
                            t.offset((j4 + (j3 + 2 as integer) * t_dim1) as isize)
                                as *mut doublereal,
                            ldt,
                            &raw mut cs,
                            &raw mut sn,
                        );
                    }
                    i__1 = (j3 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    f2c_drot_0(
                        &raw mut i__1,
                        t.offset(
                            (j3 as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        t.offset(
                            (j4 as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut cs,
                        &raw mut sn,
                    );
                    if *wantq != 0 {
                        f2c_drot_0(
                            n,
                            q.offset(
                                (j3 as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            q.offset(
                                (j4 as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            &raw mut cs,
                            &raw mut sn,
                        );
                    }
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
