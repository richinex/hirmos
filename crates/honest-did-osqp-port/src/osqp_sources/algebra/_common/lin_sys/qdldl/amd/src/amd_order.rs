extern "C" {
    #[link_name = "honest_osqp_SuiteSparse_malloc"]
    fn SuiteSparse_malloc(nitems: size_t, size_of_item: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "honest_osqp_SuiteSparse_free"]
    fn SuiteSparse_free(p: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[link_name = "honest_osqp_amd_valid"]
    fn amd_valid(
        n_row: ::core::ffi::c_int,
        n_col: ::core::ffi::c_int,
        Ap: *const ::core::ffi::c_int,
        Ai: *const ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "honest_osqp_amd_aat"]
    fn amd_aat(
        n: ::core::ffi::c_int,
        Ap: *const ::core::ffi::c_int,
        Ai: *const ::core::ffi::c_int,
        Len: *mut ::core::ffi::c_int,
        Tp: *mut ::core::ffi::c_int,
        Info: *mut OSQPFloat,
    ) -> size_t;
    #[link_name = "honest_osqp_amd_1"]
    fn amd_1(
        n: ::core::ffi::c_int,
        Ap: *const ::core::ffi::c_int,
        Ai: *const ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        Pinv: *mut ::core::ffi::c_int,
        Len: *mut ::core::ffi::c_int,
        slen: ::core::ffi::c_int,
        S: *mut ::core::ffi::c_int,
        Control: *mut OSQPFloat,
        Info: *mut OSQPFloat,
    );
    #[link_name = "honest_osqp_amd_preprocess"]
    fn amd_preprocess(
        n: ::core::ffi::c_int,
        Ap: *const ::core::ffi::c_int,
        Ai: *const ::core::ffi::c_int,
        Rp: *mut ::core::ffi::c_int,
        Ri: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        Flag: *mut ::core::ffi::c_int,
    );
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type OSQPFloat = ::core::ffi::c_double;
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_INFO: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const AMD_STATUS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_N: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_NZ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AMD_MEMORY: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_OUT_OF_MEMORY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const AMD_INVALID: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const AMD_OK_BUT_JUMBLED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Int_MAX: ::core::ffi::c_int = INT_MAX;
#[export_name = "honest_osqp_amd_order"]
pub unsafe extern "C" fn amd_order(
    mut n: ::core::ffi::c_int,
    mut Ap: *const ::core::ffi::c_int,
    mut Ai: *const ::core::ffi::c_int,
    mut P: *mut ::core::ffi::c_int,
    mut Control: *mut OSQPFloat,
    mut Info: *mut OSQPFloat,
) -> ::core::ffi::c_int {
    let mut Len: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut S: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nz: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut Pinv: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut info: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut Rp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Ri: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Cp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Ci: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut ok: ::core::ffi::c_int = 0;
    let mut nzaat: size_t = 0;
    let mut slen: size_t = 0;
    let mut mem: OSQPFloat = 0 as ::core::ffi::c_int as OSQPFloat;
    info = (Info != ::core::ptr::null_mut::<OSQPFloat>()) as ::core::ffi::c_int;
    if info != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < AMD_INFO {
            *Info.offset(i as isize) = EMPTY as OSQPFloat;
            i += 1;
        }
        *Info.offset(AMD_N as isize) = n as OSQPFloat;
        *Info.offset(AMD_STATUS as isize) = AMD_OK as OSQPFloat;
    }
    if Ai.is_null() || Ap.is_null() || P.is_null() || n < 0 as ::core::ffi::c_int {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_INVALID as OSQPFloat;
        }
        return -(2 as ::core::ffi::c_int);
    }
    if n == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    nz = *Ap.offset(n as isize);
    if info != 0 {
        *Info.offset(AMD_NZ as isize) = nz as OSQPFloat;
    }
    if nz < 0 as ::core::ffi::c_int {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_INVALID as OSQPFloat;
        }
        return -(2 as ::core::ffi::c_int);
    }
    if n as size_t
        >= (SIZE_T_MAX as usize).wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
        || nz as size_t
            >= (SIZE_T_MAX as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
    {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as OSQPFloat;
        }
        return -(1 as ::core::ffi::c_int);
    }
    status = amd_valid(n, n, Ap, Ai);
    if status == AMD_INVALID {
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_INVALID as OSQPFloat;
        }
        return -(2 as ::core::ffi::c_int);
    }
    Len = SuiteSparse_malloc(
        n as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    Pinv = SuiteSparse_malloc(
        n as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    mem += n as ::core::ffi::c_double;
    mem += n as ::core::ffi::c_double;
    if Len.is_null() || Pinv.is_null() {
        SuiteSparse_free(Len as *mut ::core::ffi::c_void);
        SuiteSparse_free(Pinv as *mut ::core::ffi::c_void);
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as OSQPFloat;
        }
        return -(1 as ::core::ffi::c_int);
    }
    if status == AMD_OK_BUT_JUMBLED {
        Rp = SuiteSparse_malloc(
            (n + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        ) as *mut ::core::ffi::c_int;
        Ri = SuiteSparse_malloc(
            nz as size_t,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        ) as *mut ::core::ffi::c_int;
        mem += (n + 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        mem += (if nz > 1 as ::core::ffi::c_int {
            nz
        } else {
            1 as ::core::ffi::c_int
        }) as ::core::ffi::c_double;
        if Rp.is_null() || Ri.is_null() {
            SuiteSparse_free(Rp as *mut ::core::ffi::c_void);
            SuiteSparse_free(Ri as *mut ::core::ffi::c_void);
            SuiteSparse_free(Len as *mut ::core::ffi::c_void);
            SuiteSparse_free(Pinv as *mut ::core::ffi::c_void);
            if info != 0 {
                *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as OSQPFloat;
            }
            return -(1 as ::core::ffi::c_int);
        }
        amd_preprocess(
            n,
            Ap,
            Ai,
            Rp as *mut ::core::ffi::c_int,
            Ri as *mut ::core::ffi::c_int,
            Len as *mut ::core::ffi::c_int,
            Pinv as *mut ::core::ffi::c_int,
        );
        Cp = Rp;
        Ci = Ri;
    } else {
        Rp = ::core::ptr::null_mut::<::core::ffi::c_int>();
        Ri = ::core::ptr::null_mut::<::core::ffi::c_int>();
        Cp = Ap as *mut ::core::ffi::c_int;
        Ci = Ai as *mut ::core::ffi::c_int;
    }
    nzaat = amd_aat(
        n,
        Cp as *const ::core::ffi::c_int,
        Ci as *const ::core::ffi::c_int,
        Len as *mut ::core::ffi::c_int,
        P,
        Info,
    );
    S = ::core::ptr::null_mut::<::core::ffi::c_int>();
    slen = nzaat;
    ok = (slen.wrapping_add(nzaat.wrapping_div(5 as size_t)) >= slen) as ::core::ffi::c_int;
    slen = (slen as u64)
        .wrapping_add(nzaat.wrapping_div(5 as size_t) as u64) as size_t
        as size_t;
    i = 0 as ::core::ffi::c_int;
    while ok != 0 && i < 7 as ::core::ffi::c_int {
        ok = (slen.wrapping_add(n as size_t) > slen) as ::core::ffi::c_int;
        slen = (slen as u64).wrapping_add(n as u64) as size_t
            as size_t;
        i += 1;
    }
    mem += slen as ::core::ffi::c_double;
    ok = (ok != 0
        && slen
            < (SIZE_T_MAX as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize))
        as ::core::ffi::c_int;
    ok = (ok != 0 && slen < Int_MAX as size_t) as ::core::ffi::c_int;
    if ok != 0 {
        S = SuiteSparse_malloc(slen, ::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            as *mut ::core::ffi::c_int;
    }
    if S.is_null() {
        SuiteSparse_free(Rp as *mut ::core::ffi::c_void);
        SuiteSparse_free(Ri as *mut ::core::ffi::c_void);
        SuiteSparse_free(Len as *mut ::core::ffi::c_void);
        SuiteSparse_free(Pinv as *mut ::core::ffi::c_void);
        if info != 0 {
            *Info.offset(AMD_STATUS as isize) = AMD_OUT_OF_MEMORY as OSQPFloat;
        }
        return -(1 as ::core::ffi::c_int);
    }
    if info != 0 {
        *Info.offset(AMD_MEMORY as isize) = (mem as ::core::ffi::c_double
            * ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_double)
            as OSQPFloat;
    }
    amd_1(
        n,
        Cp as *const ::core::ffi::c_int,
        Ci as *const ::core::ffi::c_int,
        P,
        Pinv as *mut ::core::ffi::c_int,
        Len as *mut ::core::ffi::c_int,
        slen as ::core::ffi::c_int,
        S as *mut ::core::ffi::c_int,
        Control,
        Info,
    );
    SuiteSparse_free(Rp as *mut ::core::ffi::c_void);
    SuiteSparse_free(Ri as *mut ::core::ffi::c_void);
    SuiteSparse_free(Len as *mut ::core::ffi::c_void);
    SuiteSparse_free(Pinv as *mut ::core::ffi::c_void);
    SuiteSparse_free(S as *mut ::core::ffi::c_void);
    if info != 0 {
        *Info.offset(AMD_STATUS as isize) = status as OSQPFloat;
    }
    return status;
}
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const ULONG_MAX: u64 = 0xffffffffffffffff as u64;
pub const SIZE_T_MAX: u64 = ULONG_MAX;
