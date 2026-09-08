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
static mut c__2: integer = 2 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dtrexc_(
    mut compq: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut t: *mut doublereal,
    mut ldt: *mut integer,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut ifst: *mut integer,
    mut ilst: *mut integer,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut t_dim1: integer = 0;
    let mut t_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut nbf: integer = 0;
    let mut nbl: integer = 0;
    let mut here: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut wantq: logical = 0;
    extern "C" {
        #[link_name = "dgeev_closure_dlaexc_"]
        fn dlaexc__0(
            _: *mut logical,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut nbnext: integer = 0;
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    work = work.offset(-1);
    *info = 0 as integer;
    wantq = lsame__0(
        compq,
        b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if wantq == 0
        && lsame__0(
            compq,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *ldt
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ldq < 1 as ::core::ffi::c_long
        || wantq != 0
            && *ldq
                < (if 1 as ::core::ffi::c_long >= *n {
                    1 as ::core::ffi::c_long
                } else {
                    *n
                })
    {
        *info = -(6 as ::core::ffi::c_int) as integer;
    } else if *ifst < 1 as ::core::ffi::c_long || *ifst > *n {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *ilst < 1 as ::core::ffi::c_long || *ilst > *n {
        *info = -(8 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DTREXC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n <= 1 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *ifst > 1 as ::core::ffi::c_long {
        if *t.offset((*ifst + (*ifst - 1 as integer) * t_dim1) as isize) != 0.0f64 {
            *ifst -= 1;
        }
    }
    nbf = 1 as integer;
    if *ifst < *n {
        if *t.offset((*ifst + 1 as integer + *ifst * t_dim1) as isize) != 0.0f64 {
            nbf = 2 as integer;
        }
    }
    if *ilst > 1 as ::core::ffi::c_long {
        if *t.offset((*ilst + (*ilst - 1 as integer) * t_dim1) as isize) != 0.0f64 {
            *ilst -= 1;
        }
    }
    nbl = 1 as integer;
    if *ilst < *n {
        if *t.offset((*ilst + 1 as integer + *ilst * t_dim1) as isize) != 0.0f64 {
            nbl = 2 as integer;
        }
    }
    if *ifst == *ilst {
        return 0 as ::core::ffi::c_int;
    }
    if *ifst < *ilst {
        if nbf == 2 as ::core::ffi::c_long && nbl == 1 as ::core::ffi::c_long {
            *ilst -= 1;
        }
        if nbf == 1 as ::core::ffi::c_long && nbl == 2 as ::core::ffi::c_long {
            *ilst += 1;
        }
        here = *ifst;
        loop {
            if nbf == 1 as ::core::ffi::c_long || nbf == 2 as ::core::ffi::c_long {
                nbnext = 1 as integer;
                if here as ::core::ffi::c_long
                    + nbf as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long
                    <= *n
                {
                    if *t.offset((here + nbf + 1 as integer + (here + nbf) * t_dim1) as isize)
                        != 0.0f64
                    {
                        nbnext = 2 as integer;
                    }
                }
                dlaexc__0(
                    &raw mut wantq,
                    n,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    q.offset(q_offset as isize) as *mut doublereal,
                    ldq,
                    &raw mut here,
                    &raw mut nbf,
                    &raw mut nbnext,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
                if *info != 0 as ::core::ffi::c_long {
                    *ilst = here;
                    return 0 as ::core::ffi::c_int;
                }
                here += nbnext as ::core::ffi::c_long;
                if nbf == 2 as ::core::ffi::c_long {
                    if *t.offset((here + 1 as integer + here * t_dim1) as isize) == 0.0f64 {
                        nbf = 3 as integer;
                    }
                }
            } else {
                nbnext = 1 as integer;
                if here as ::core::ffi::c_long + 3 as ::core::ffi::c_long <= *n {
                    if *t.offset((here + 3 as integer + (here + 2 as integer) * t_dim1) as isize)
                        != 0.0f64
                    {
                        nbnext = 2 as integer;
                    }
                }
                i__1 = (here as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dlaexc__0(
                    &raw mut wantq,
                    n,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    q.offset(q_offset as isize) as *mut doublereal,
                    ldq,
                    &raw mut i__1,
                    &raw mut c__1,
                    &raw mut nbnext,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
                if *info != 0 as ::core::ffi::c_long {
                    *ilst = here;
                    return 0 as ::core::ffi::c_int;
                }
                if nbnext == 1 as ::core::ffi::c_long {
                    dlaexc__0(
                        &raw mut wantq,
                        n,
                        t.offset(t_offset as isize) as *mut doublereal,
                        ldt,
                        q.offset(q_offset as isize) as *mut doublereal,
                        ldq,
                        &raw mut here,
                        &raw mut c__1,
                        &raw mut nbnext,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        info,
                    );
                    here += 1;
                } else {
                    if *t.offset((here + 2 as integer + (here + 1 as integer) * t_dim1) as isize)
                        == 0.0f64
                    {
                        nbnext = 1 as integer;
                    }
                    if nbnext == 2 as ::core::ffi::c_long {
                        dlaexc__0(
                            &raw mut wantq,
                            n,
                            t.offset(t_offset as isize) as *mut doublereal,
                            ldt,
                            q.offset(q_offset as isize) as *mut doublereal,
                            ldq,
                            &raw mut here,
                            &raw mut c__1,
                            &raw mut nbnext,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            info,
                        );
                        if *info != 0 as ::core::ffi::c_long {
                            *ilst = here;
                            return 0 as ::core::ffi::c_int;
                        }
                        here += 2 as ::core::ffi::c_long;
                    } else {
                        dlaexc__0(
                            &raw mut wantq,
                            n,
                            t.offset(t_offset as isize) as *mut doublereal,
                            ldt,
                            q.offset(q_offset as isize) as *mut doublereal,
                            ldq,
                            &raw mut here,
                            &raw mut c__1,
                            &raw mut c__1,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            info,
                        );
                        i__1 = (here as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        dlaexc__0(
                            &raw mut wantq,
                            n,
                            t.offset(t_offset as isize) as *mut doublereal,
                            ldt,
                            q.offset(q_offset as isize) as *mut doublereal,
                            ldq,
                            &raw mut i__1,
                            &raw mut c__1,
                            &raw mut c__1,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            info,
                        );
                        here += 2 as ::core::ffi::c_long;
                    }
                }
            }
            if !(here < *ilst) {
                break;
            }
        }
    } else {
        here = *ifst;
        loop {
            if nbf == 1 as ::core::ffi::c_long || nbf == 2 as ::core::ffi::c_long {
                nbnext = 1 as integer;
                if here >= 3 as ::core::ffi::c_long {
                    if *t.offset((here - 1 as integer + (here - 2 as integer) * t_dim1) as isize)
                        != 0.0f64
                    {
                        nbnext = 2 as integer;
                    }
                }
                i__1 = here - nbnext;
                dlaexc__0(
                    &raw mut wantq,
                    n,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    q.offset(q_offset as isize) as *mut doublereal,
                    ldq,
                    &raw mut i__1,
                    &raw mut nbnext,
                    &raw mut nbf,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
                if *info != 0 as ::core::ffi::c_long {
                    *ilst = here;
                    return 0 as ::core::ffi::c_int;
                }
                here -= nbnext as ::core::ffi::c_long;
                if nbf == 2 as ::core::ffi::c_long {
                    if *t.offset((here + 1 as integer + here * t_dim1) as isize) == 0.0f64 {
                        nbf = 3 as integer;
                    }
                }
            } else {
                nbnext = 1 as integer;
                if here >= 3 as ::core::ffi::c_long {
                    if *t.offset((here - 1 as integer + (here - 2 as integer) * t_dim1) as isize)
                        != 0.0f64
                    {
                        nbnext = 2 as integer;
                    }
                }
                i__1 = here - nbnext;
                dlaexc__0(
                    &raw mut wantq,
                    n,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    q.offset(q_offset as isize) as *mut doublereal,
                    ldq,
                    &raw mut i__1,
                    &raw mut nbnext,
                    &raw mut c__1,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
                if *info != 0 as ::core::ffi::c_long {
                    *ilst = here;
                    return 0 as ::core::ffi::c_int;
                }
                if nbnext == 1 as ::core::ffi::c_long {
                    dlaexc__0(
                        &raw mut wantq,
                        n,
                        t.offset(t_offset as isize) as *mut doublereal,
                        ldt,
                        q.offset(q_offset as isize) as *mut doublereal,
                        ldq,
                        &raw mut here,
                        &raw mut nbnext,
                        &raw mut c__1,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        info,
                    );
                    here -= 1;
                } else {
                    if *t.offset((here + (here - 1 as integer) * t_dim1) as isize) == 0.0f64 {
                        nbnext = 1 as integer;
                    }
                    if nbnext == 2 as ::core::ffi::c_long {
                        i__1 = (here as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        dlaexc__0(
                            &raw mut wantq,
                            n,
                            t.offset(t_offset as isize) as *mut doublereal,
                            ldt,
                            q.offset(q_offset as isize) as *mut doublereal,
                            ldq,
                            &raw mut i__1,
                            &raw mut c__2,
                            &raw mut c__1,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            info,
                        );
                        if *info != 0 as ::core::ffi::c_long {
                            *ilst = here;
                            return 0 as ::core::ffi::c_int;
                        }
                        here += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    } else {
                        dlaexc__0(
                            &raw mut wantq,
                            n,
                            t.offset(t_offset as isize) as *mut doublereal,
                            ldt,
                            q.offset(q_offset as isize) as *mut doublereal,
                            ldq,
                            &raw mut here,
                            &raw mut c__1,
                            &raw mut c__1,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            info,
                        );
                        i__1 = (here as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        dlaexc__0(
                            &raw mut wantq,
                            n,
                            t.offset(t_offset as isize) as *mut doublereal,
                            ldt,
                            q.offset(q_offset as isize) as *mut doublereal,
                            ldq,
                            &raw mut i__1,
                            &raw mut c__1,
                            &raw mut c__1,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            info,
                        );
                        here += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    }
                }
            }
            if !(here > *ilst) {
                break;
            }
        }
    }
    *ilst = here;
    return 0 as ::core::ffi::c_int;
}
