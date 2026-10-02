extern "C" {
    fn amd_l_valid(
        n_row: i64,
        n_col: i64,
        Ap: *const i64,
        Ai: *const i64,
    ) -> i64;
    static amd_malloc: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
    static amd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    fn amd_l_aat(
        n: i64,
        Ap: *const i64,
        Ai: *const i64,
        Len: *mut i64,
        Tp: *mut i64,
        Info: *mut ::core::ffi::c_double,
    ) -> size_t;
    fn amd_l1(
        n: i64,
        Ap: *const i64,
        Ai: *const i64,
        P: *mut i64,
        Pinv: *mut i64,
        Len: *mut i64,
        slen: i64,
        S: *mut i64,
        Control: *mut ::core::ffi::c_double,
        Info: *mut ::core::ffi::c_double,
    );
    fn amd_l_preprocess(
        n: i64,
        Ap: *const i64,
        Ai: *const i64,
        Rp: *mut i64,
        Ri: *mut i64,
        W: *mut i64,
        Flag: *mut i64,
    );
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SuiteSparse_long_max: i64 = LONG_MAX;
pub const AMD_INFO: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const AMD_STATUS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_N: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_NZ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AMD_MEMORY: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_OUT_OF_MEMORY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const AMD_INVALID: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const AMD_OK_BUT_JUMBLED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Int_MAX: i64 = SuiteSparse_long_max;
#[no_mangle]
pub unsafe extern "C" fn amd_l_order(
    mut n: i64,
    mut Ap: *const i64,
    mut Ai: *const i64,
    mut P: *mut i64,
    mut Control: *mut ::core::ffi::c_double,
    mut Info: *mut ::core::ffi::c_double,
) -> i64 {
    let mut Len: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut S: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut nz: i64 = 0;
    let mut i: i64 = 0;
    let mut Pinv: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut info: i64 = 0;
    let mut status: i64 = 0;
    let mut Rp: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Ri: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Cp: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Ci: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut ok: i64 = 0;
    let mut nzaat: size_t = 0;
    let mut slen: size_t = 0;
    let mut mem: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    info = (Info != ::core::ptr::null_mut::<::core::ffi::c_double>()) as ::core::ffi::c_int
        as i64;
    if info != 0 {
        i = 0 as i64;
        while i < AMD_INFO as i64 {
            *Info.offset(i as isize) = EMPTY as ::core::ffi::c_double;
            i += 1;
        }
        *Info.offset(AMD_N as isize) = n as ::core::ffi::c_double;
        *Info.offset(AMD_STATUS as isize) = AMD_OK as ::core::ffi::c_double;
    }
    if Ai.is_null() || Ap.is_null() || P.is_null() || n < 0 as i64 {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_INVALID as ::core::ffi::c_double;
        }
        return -(2 as ::core::ffi::c_int) as i64;
    }
    if n == 0 as i64 {
        return 0 as i64;
    }
    nz = *Ap.offset(n as isize);
    if info != 0 {
        *Info.offset(AMD_NZ as isize) = nz as ::core::ffi::c_double;
    }
    if nz < 0 as i64 {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_INVALID as ::core::ffi::c_double;
        }
        return -(2 as ::core::ffi::c_int) as i64;
    }
    if n as size_t
        >= (SIZE_T_MAX as usize)
            .wrapping_div(::core::mem::size_of::<i64>() as usize)
        || nz as size_t
            >= (SIZE_T_MAX as usize)
                .wrapping_div(::core::mem::size_of::<i64>() as usize)
    {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as ::core::ffi::c_double;
        }
        return -(1 as ::core::ffi::c_int) as i64;
    }
    status = amd_l_valid(n, n, Ap, Ai);
    if status == AMD_INVALID as i64 {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_INVALID as ::core::ffi::c_double;
        }
        return -(2 as ::core::ffi::c_int) as i64;
    }
    Len = amd_malloc.expect("non-null function pointer")(
        (n as size_t).wrapping_mul(::core::mem::size_of::<i64>() as size_t),
    ) as *mut i64;
    Pinv = amd_malloc.expect("non-null function pointer")(
        (n as size_t).wrapping_mul(::core::mem::size_of::<i64>() as size_t),
    ) as *mut i64;
    mem += n as ::core::ffi::c_double;
    mem += n as ::core::ffi::c_double;
    if Len.is_null() || Pinv.is_null() {
        amd_free.expect("non-null function pointer")(Len as *mut ::core::ffi::c_void);
        amd_free.expect("non-null function pointer")(Pinv as *mut ::core::ffi::c_void);
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as ::core::ffi::c_double;
        }
        return -(1 as ::core::ffi::c_int) as i64;
    }
    if status == AMD_OK_BUT_JUMBLED as i64 {
        Rp = amd_malloc.expect("non-null function pointer")(
            ((n + 1 as i64) as size_t)
                .wrapping_mul(::core::mem::size_of::<i64>() as size_t),
        ) as *mut i64;
        Ri = amd_malloc.expect("non-null function pointer")(
            ((if nz > 1 as i64 {
                nz
            } else {
                1 as i64
            }) as size_t)
                .wrapping_mul(::core::mem::size_of::<i64>() as size_t),
        ) as *mut i64;
        mem += (n + 1 as i64) as ::core::ffi::c_double;
        mem += (if nz > 1 as i64 {
            nz
        } else {
            1 as i64
        }) as ::core::ffi::c_double;
        if Rp.is_null() || Ri.is_null() {
            amd_free.expect("non-null function pointer")(Rp as *mut ::core::ffi::c_void);
            amd_free.expect("non-null function pointer")(Ri as *mut ::core::ffi::c_void);
            amd_free.expect("non-null function pointer")(Len as *mut ::core::ffi::c_void);
            amd_free.expect("non-null function pointer")(Pinv as *mut ::core::ffi::c_void);
            if info != 0 {
                *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as ::core::ffi::c_double;
            }
            return -(1 as ::core::ffi::c_int) as i64;
        }
        amd_l_preprocess(
            n,
            Ap,
            Ai,
            Rp as *mut i64,
            Ri as *mut i64,
            Len as *mut i64,
            Pinv as *mut i64,
        );
        Cp = Rp;
        Ci = Ri;
    } else {
        Rp = ::core::ptr::null_mut::<i64>();
        Ri = ::core::ptr::null_mut::<i64>();
        Cp = Ap as *mut i64;
        Ci = Ai as *mut i64;
    }
    nzaat = amd_l_aat(
        n,
        Cp as *const i64,
        Ci as *const i64,
        Len as *mut i64,
        P,
        Info,
    );
    S = ::core::ptr::null_mut::<i64>();
    slen = nzaat;
    ok = (slen.wrapping_add(nzaat.wrapping_div(5 as size_t)) >= slen) as ::core::ffi::c_int
        as i64;
    slen = (slen as u64)
        .wrapping_add(nzaat.wrapping_div(5 as size_t) as u64) as size_t
        as size_t;
    i = 0 as i64;
    while ok != 0 && i < 7 as i64 {
        ok = (slen.wrapping_add(n as size_t) > slen) as ::core::ffi::c_int as i64;
        slen = (slen as u64).wrapping_add(n as u64) as size_t
            as size_t;
        i += 1;
    }
    mem += slen as ::core::ffi::c_double;
    ok = (ok != 0
        && slen
            < (SIZE_T_MAX as usize)
                .wrapping_div(::core::mem::size_of::<i64>() as usize))
        as ::core::ffi::c_int as i64;
    ok = (ok != 0 && slen < Int_MAX as size_t) as ::core::ffi::c_int as i64;
    if ok != 0 {
        S = amd_malloc.expect("non-null function pointer")(
            slen.wrapping_mul(::core::mem::size_of::<i64>() as size_t),
        ) as *mut i64;
    }
    if S.is_null() {
        amd_free.expect("non-null function pointer")(Rp as *mut ::core::ffi::c_void);
        amd_free.expect("non-null function pointer")(Ri as *mut ::core::ffi::c_void);
        amd_free.expect("non-null function pointer")(Len as *mut ::core::ffi::c_void);
        amd_free.expect("non-null function pointer")(Pinv as *mut ::core::ffi::c_void);
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as ::core::ffi::c_double;
        }
        return -(1 as ::core::ffi::c_int) as i64;
    }
    if info != 0 {
        *Info.offset(AMD_MEMORY as isize) =
            mem * ::core::mem::size_of::<i64>() as ::core::ffi::c_double;
    }
    amd_l1(
        n,
        Cp as *const i64,
        Ci as *const i64,
        P,
        Pinv as *mut i64,
        Len as *mut i64,
        slen as i64,
        S as *mut i64,
        Control,
        Info,
    );
    amd_free.expect("non-null function pointer")(Rp as *mut ::core::ffi::c_void);
    amd_free.expect("non-null function pointer")(Ri as *mut ::core::ffi::c_void);
    amd_free.expect("non-null function pointer")(Len as *mut ::core::ffi::c_void);
    amd_free.expect("non-null function pointer")(Pinv as *mut ::core::ffi::c_void);
    amd_free.expect("non-null function pointer")(S as *mut ::core::ffi::c_void);
    if info != 0 {
        *Info.offset(AMD_STATUS as isize) = status as ::core::ffi::c_double;
    }
    return status;
}
pub const ULONG_MAX: u64 = 0xffffffffffffffff as u64;
pub const LONG_MAX: i64 = 0x7fffffffffffffff as i64;
pub const SIZE_T_MAX: u64 = ULONG_MAX;
