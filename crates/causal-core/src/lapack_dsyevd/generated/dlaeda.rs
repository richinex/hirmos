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
static mut c__2: integer = 2 as integer;
static mut c__1: integer = 1 as integer;
static mut c_b24: doublereal = 1.0f64;
static mut c_b26: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaeda_(
    mut n: *mut integer,
    mut tlvls: *mut integer,
    mut curlvl: *mut integer,
    mut curpbm: *mut integer,
    mut prmptr: *mut integer,
    mut perm: *mut integer,
    mut givptr: *mut integer,
    mut givcol: *mut integer,
    mut givnum: *mut doublereal,
    mut q: *mut doublereal,
    mut qptr: *mut integer,
    mut z__: *mut doublereal,
    mut ztemp: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_pow_ii"]
        fn pow_ii_0(_: *mut integer, _: *mut integer) -> integer;
    }
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut mid: integer = 0;
    let mut ptr: integer = 0;
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
    let mut curr: integer = 0;
    let mut bsiz1: integer = 0;
    let mut bsiz2: integer = 0;
    let mut psiz1: integer = 0;
    let mut psiz2: integer = 0;
    let mut zptr1: integer = 0;
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
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    ztemp = ztemp.offset(-1);
    z__ = z__.offset(-1);
    qptr = qptr.offset(-1);
    q = q.offset(-1);
    givnum = givnum.offset(-(3 as ::core::ffi::c_int as isize));
    givcol = givcol.offset(-(3 as ::core::ffi::c_int as isize));
    givptr = givptr.offset(-1);
    perm = perm.offset(-1);
    prmptr = prmptr.offset(-1);
    *info = 0 as integer;
    if *n < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAEDA\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    mid = (*n / 2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    ptr = 1 as integer;
    i__1 = (*curlvl - 1 as ::core::ffi::c_long) as integer;
    curr = (ptr as ::core::ffi::c_long
        + *curpbm * pow_ii_0(&raw mut c__2, curlvl) as ::core::ffi::c_long
        + pow_ii_0(&raw mut c__2, &raw mut i__1) as ::core::ffi::c_long
        - 1 as ::core::ffi::c_long) as integer;
    bsiz1 = (sqrt(
        (*qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            - *qptr.offset(curr as isize)) as doublereal,
    ) + 0.5f64) as integer;
    bsiz2 = (sqrt(
        (*qptr.offset((curr as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            - *qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize))
            as doublereal,
    ) + 0.5f64) as integer;
    i__1 = (mid as ::core::ffi::c_long - bsiz1 as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
        as integer;
    k = 1 as integer;
    while k <= i__1 {
        *z__.offset(k as isize) = 0.0f64 as doublereal;
        k += 1;
    }
    f2c_dcopy_0(
        &raw mut bsiz1,
        q.offset(
            (*qptr.offset(curr as isize) as ::core::ffi::c_long + bsiz1 as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as isize,
        ) as *mut doublereal,
        &raw mut bsiz1,
        z__.offset((mid - bsiz1) as isize) as *mut doublereal,
        &raw mut c__1,
    );
    f2c_dcopy_0(
        &raw mut bsiz2,
        q.offset(
            *qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as isize,
        ) as *mut doublereal,
        &raw mut bsiz2,
        z__.offset(mid as isize) as *mut doublereal,
        &raw mut c__1,
    );
    i__1 = *n;
    k = mid + bsiz2;
    while k <= i__1 {
        *z__.offset(k as isize) = 0.0f64 as doublereal;
        k += 1;
    }
    ptr = (pow_ii_0(&raw mut c__2, tlvls) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
        as integer;
    i__1 = (*curlvl - 1 as ::core::ffi::c_long) as integer;
    k = 1 as integer;
    while k <= i__1 {
        i__2 = *curlvl - k;
        i__3 = (*curlvl - k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        curr = (ptr as ::core::ffi::c_long
            + *curpbm * pow_ii_0(&raw mut c__2, &raw mut i__2) as ::core::ffi::c_long
            + pow_ii_0(&raw mut c__2, &raw mut i__3) as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long) as integer;
        psiz1 = *prmptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            - *prmptr.offset(curr as isize);
        psiz2 = *prmptr.offset((curr as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            - *prmptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
        zptr1 = mid - psiz1;
        i__2 = (*givptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long) as integer;
        i__ = *givptr.offset(curr as isize);
        while i__ <= i__2 {
            f2c_drot_0(
                &raw mut c__1,
                z__.offset(
                    (zptr1 as ::core::ffi::c_long
                        + *givcol.offset(
                            (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                z__.offset(
                    (zptr1 as ::core::ffi::c_long
                        + *givcol.offset(
                            (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 2 as ::core::ffi::c_long) as isize,
                        ) as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                givnum.offset(
                    (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                givnum.offset(
                    (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
            );
            i__ += 1;
        }
        i__2 = (*givptr.offset((curr as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long) as integer;
        i__ = *givptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
        while i__ <= i__2 {
            f2c_drot_0(
                &raw mut c__1,
                z__.offset(
                    (mid - 1 as integer
                        + *givcol.offset(
                            (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_long) as isize,
                        )) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                z__.offset(
                    (mid - 1 as integer
                        + *givcol.offset(
                            (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 2 as ::core::ffi::c_long) as isize,
                        )) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                givnum.offset(
                    (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                givnum.offset(
                    (((i__ as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
            );
            i__ += 1;
        }
        psiz1 = *prmptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            - *prmptr.offset(curr as isize);
        psiz2 = *prmptr.offset((curr as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            - *prmptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
        i__2 = (psiz1 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        i__ = 0 as integer;
        while i__ <= i__2 {
            *ztemp.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) = *z__
                .offset(
                    (zptr1 as ::core::ffi::c_long
                        + *perm.offset((*prmptr.offset(curr as isize) + i__) as isize)
                            as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as isize,
                );
            i__ += 1;
        }
        i__2 = (psiz2 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        i__ = 0 as integer;
        while i__ <= i__2 {
            *ztemp.offset(
                (psiz1 as ::core::ffi::c_long
                    + i__ as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) = *z__.offset(
                (mid as ::core::ffi::c_long
                    + *perm.offset(
                        (*prmptr.offset(
                            (curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        ) + i__) as isize,
                    ) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as isize,
            );
            i__ += 1;
        }
        bsiz1 = (sqrt(
            (*qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                - *qptr.offset(curr as isize)) as doublereal,
        ) + 0.5f64) as integer;
        bsiz2 = (sqrt(
            (*qptr.offset((curr as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                - *qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize))
                as doublereal,
        ) + 0.5f64) as integer;
        if bsiz1 > 0 as ::core::ffi::c_long {
            f2c_dgemv_0(
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut bsiz1,
                &raw mut bsiz1,
                &raw mut c_b24,
                q.offset(*qptr.offset(curr as isize) as isize) as *mut doublereal,
                &raw mut bsiz1,
                ztemp.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b26,
                z__.offset(zptr1 as isize) as *mut doublereal,
                &raw mut c__1,
            );
        }
        i__2 = psiz1 - bsiz1;
        f2c_dcopy_0(
            &raw mut i__2,
            ztemp.offset((bsiz1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            &raw mut c__1,
            z__.offset((zptr1 + bsiz1) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        if bsiz2 > 0 as ::core::ffi::c_long {
            f2c_dgemv_0(
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut bsiz2,
                &raw mut bsiz2,
                &raw mut c_b24,
                q.offset(
                    *qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        as isize,
                ) as *mut doublereal,
                &raw mut bsiz2,
                ztemp.offset((psiz1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b26,
                z__.offset(mid as isize) as *mut doublereal,
                &raw mut c__1,
            );
        }
        i__2 = psiz2 - bsiz2;
        f2c_dcopy_0(
            &raw mut i__2,
            ztemp.offset(
                (psiz1 as ::core::ffi::c_long
                    + bsiz2 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
            z__.offset((mid + bsiz2) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__2 = *tlvls - k;
        ptr += pow_ii_0(&raw mut c__2, &raw mut i__2) as ::core::ffi::c_long;
        k += 1;
    }
    return 0 as ::core::ffi::c_int;
}
