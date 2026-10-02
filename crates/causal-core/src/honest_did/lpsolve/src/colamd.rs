pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colamd_Col_struct {
    pub start: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub shared1: C2RustUnnamed_2,
    pub shared2: C2RustUnnamed_1,
    pub shared3: C2RustUnnamed_0,
    pub shared4: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub degree_next: ::core::ffi::c_int,
    pub hash_next: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub headhash: ::core::ffi::c_int,
    pub hash: ::core::ffi::c_int,
    pub prev: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub score: ::core::ffi::c_int,
    pub order: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub thickness: ::core::ffi::c_int,
    pub parent: ::core::ffi::c_int,
}
pub type Colamd_Col = Colamd_Col_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colamd_Row_struct {
    pub start: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub shared1: C2RustUnnamed_4,
    pub shared2: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub mark: ::core::ffi::c_int,
    pub first_column: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub degree: ::core::ffi::c_int,
    pub p: ::core::ffi::c_int,
}
pub type Colamd_Row = Colamd_Row_struct;
pub const COLAMD_KNOBS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const COLAMD_STATS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const COLAMD_DENSE_ROW: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COLAMD_DENSE_COL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLAMD_DEFRAG_COUNT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const COLAMD_STATUS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const COLAMD_INFO1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const COLAMD_INFO2: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const COLAMD_INFO3: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const COLAMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COLAMD_OK_BUT_JUMBLED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLAMD_ERROR_A_not_present: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const COLAMD_ERROR_p_not_present: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const COLAMD_ERROR_nrow_negative: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const COLAMD_ERROR_ncol_negative: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const COLAMD_ERROR_nnz_negative: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
pub const COLAMD_ERROR_p0_nonzero: ::core::ffi::c_int = -(6 as ::core::ffi::c_int);
pub const COLAMD_ERROR_A_too_small: ::core::ffi::c_int = -(7 as ::core::ffi::c_int);
pub const COLAMD_ERROR_col_length_negative: ::core::ffi::c_int = -(8 as ::core::ffi::c_int);
pub const COLAMD_ERROR_row_index_out_of_bounds: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const COLAMD_ERROR_out_of_memory: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
pub const COLAMD_ERROR_internal_error: ::core::ffi::c_int = -(999 as ::core::ffi::c_int);
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const ALIVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEAD: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const DEAD_PRINCIPAL: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const DEAD_NON_PRINCIPAL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
#[export_name="honest_lpsolve_colamd_recommended"]
pub unsafe extern "C" fn colamd_recommended(
    mut nnz: ::core::ffi::c_int,
    mut n_row: ::core::ffi::c_int,
    mut n_col: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if nnz < 0 as ::core::ffi::c_int
        || n_row < 0 as ::core::ffi::c_int
        || n_col < 0 as ::core::ffi::c_int
    {
        -(1 as ::core::ffi::c_int)
    } else {
        ((2 as ::core::ffi::c_int * nnz) as usize)
            .wrapping_add(
                ((n_col + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<Colamd_Col>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize),
            )
            .wrapping_add(
                ((n_row + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<Colamd_Row>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize),
            )
            .wrapping_add(n_col as usize)
            .wrapping_add((nnz / 5 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int
    };
}
#[export_name="honest_lpsolve_colamd_set_defaults"]
pub unsafe extern "C" fn colamd_set_defaults(mut knobs: *mut ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    if knobs.is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < COLAMD_KNOBS {
        *knobs.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    *knobs.offset(COLAMD_DENSE_ROW as isize) = 0.5f64;
    *knobs.offset(COLAMD_DENSE_COL as isize) = 0.5f64;
}
#[export_name="honest_lpsolve_symamd"]
pub unsafe extern "C" fn symamd(
    mut n: ::core::ffi::c_int,
    mut A: *mut ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_int,
    mut perm: *mut ::core::ffi::c_int,
    mut knobs: *mut ::core::ffi::c_double,
    mut stats: *mut ::core::ffi::c_int,
    mut allocate: Option<unsafe extern "C" fn(size_t, size_t) -> *mut ::core::ffi::c_void>,
    mut release: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    let mut count: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut mark: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut M: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Mlen: ::core::ffi::c_int = 0;
    let mut n_row: ::core::ffi::c_int = 0;
    let mut nnz: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut mnz: ::core::ffi::c_int = 0;
    let mut pp: ::core::ffi::c_int = 0;
    let mut last_row: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut cknobs: [::core::ffi::c_double; 20] = [0.; 20];
    let mut default_knobs: [::core::ffi::c_double; 20] = [0.; 20];
    let mut cstats: [::core::ffi::c_int; 20] = [0; 20];
    if stats.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < COLAMD_STATS {
        *stats.offset(i as isize) = 0 as ::core::ffi::c_int;
        i += 1;
    }
    *stats.offset(COLAMD_STATUS as isize) = COLAMD_OK;
    *stats.offset(COLAMD_INFO1 as isize) = -(1 as ::core::ffi::c_int);
    *stats.offset(COLAMD_INFO2 as isize) = -(1 as ::core::ffi::c_int);
    if A.is_null() {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_A_not_present;
        return 0 as ::core::ffi::c_int;
    }
    if p.is_null() {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_p_not_present;
        return 0 as ::core::ffi::c_int;
    }
    if n < 0 as ::core::ffi::c_int {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_ncol_negative;
        *stats.offset(COLAMD_INFO1 as isize) = n;
        return 0 as ::core::ffi::c_int;
    }
    nnz = *p.offset(n as isize);
    if nnz < 0 as ::core::ffi::c_int {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_nnz_negative;
        *stats.offset(COLAMD_INFO1 as isize) = nnz;
        return 0 as ::core::ffi::c_int;
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) != 0 as ::core::ffi::c_int {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_p0_nonzero;
        *stats.offset(COLAMD_INFO1 as isize) = *p.offset(0 as ::core::ffi::c_int as isize);
        return 0 as ::core::ffi::c_int;
    }
    if knobs.is_null() {
        colamd_set_defaults(&raw mut default_knobs as *mut ::core::ffi::c_double);
        knobs = &raw mut default_knobs as *mut ::core::ffi::c_double as *mut ::core::ffi::c_double;
    }
    count = Some(allocate.expect("non-null function pointer")).expect("non-null function pointer")(
        (n + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if count.is_null() {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_out_of_memory;
        return 0 as ::core::ffi::c_int;
    }
    mark = Some(allocate.expect("non-null function pointer")).expect("non-null function pointer")(
        (n + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if mark.is_null() {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_out_of_memory;
        Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
            count as *mut ::core::ffi::c_void,
        );
        return 0 as ::core::ffi::c_int;
    }
    *stats.offset(COLAMD_INFO3 as isize) = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *mark.offset(i as isize) = -(1 as ::core::ffi::c_int);
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < n {
        last_row = -(1 as ::core::ffi::c_int);
        length = *p.offset((j + 1 as ::core::ffi::c_int) as isize) - *p.offset(j as isize);
        if length < 0 as ::core::ffi::c_int {
            *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_col_length_negative;
            *stats.offset(COLAMD_INFO1 as isize) = j;
            *stats.offset(COLAMD_INFO2 as isize) = length;
            Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
                count as *mut ::core::ffi::c_void,
            );
            Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
                mark as *mut ::core::ffi::c_void,
            );
            return 0 as ::core::ffi::c_int;
        }
        pp = *p.offset(j as isize);
        while pp < *p.offset((j + 1 as ::core::ffi::c_int) as isize) {
            i = *A.offset(pp as isize);
            if i < 0 as ::core::ffi::c_int || i >= n {
                *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_row_index_out_of_bounds;
                *stats.offset(COLAMD_INFO1 as isize) = j;
                *stats.offset(COLAMD_INFO2 as isize) = i;
                *stats.offset(COLAMD_INFO3 as isize) = n;
                Some(release.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    count as *mut ::core::ffi::c_void
                );
                Some(release.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    mark as *mut ::core::ffi::c_void
                );
                return 0 as ::core::ffi::c_int;
            }
            if i <= last_row || *mark.offset(i as isize) == j {
                *stats.offset(COLAMD_STATUS as isize) = COLAMD_OK_BUT_JUMBLED;
                *stats.offset(COLAMD_INFO1 as isize) = j;
                *stats.offset(COLAMD_INFO2 as isize) = i;
                let ref mut fresh39 = *stats.offset(COLAMD_INFO3 as isize);
                *fresh39 += 1;
            }
            if i > j && *mark.offset(i as isize) != j {
                let ref mut fresh40 = *count.offset(i as isize);
                *fresh40 += 1;
                let ref mut fresh41 = *count.offset(j as isize);
                *fresh41 += 1;
            }
            *mark.offset(i as isize) = j;
            last_row = i;
            pp += 1;
        }
        j += 1;
    }
    if *stats.offset(COLAMD_STATUS as isize) == COLAMD_OK {
        Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
            mark as *mut ::core::ffi::c_void,
        );
    }
    *perm.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    while j <= n {
        *perm.offset(j as isize) = *perm.offset((j - 1 as ::core::ffi::c_int) as isize)
            + *count.offset((j - 1 as ::core::ffi::c_int) as isize);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < n {
        *count.offset(j as isize) = *perm.offset(j as isize);
        j += 1;
    }
    mnz = *perm.offset(n as isize);
    n_row = mnz / 2 as ::core::ffi::c_int;
    Mlen = colamd_recommended(mnz, n_row, n);
    M = Some(allocate.expect("non-null function pointer")).expect("non-null function pointer")(
        Mlen as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if M.is_null() {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_out_of_memory;
        Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
            count as *mut ::core::ffi::c_void,
        );
        Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
            mark as *mut ::core::ffi::c_void,
        );
        return 0 as ::core::ffi::c_int;
    }
    k = 0 as ::core::ffi::c_int;
    if *stats.offset(COLAMD_STATUS as isize) == COLAMD_OK {
        j = 0 as ::core::ffi::c_int;
        while j < n {
            pp = *p.offset(j as isize);
            while pp < *p.offset((j + 1 as ::core::ffi::c_int) as isize) {
                i = *A.offset(pp as isize);
                if i > j {
                    let ref mut fresh42 = *count.offset(i as isize);
                    let fresh43 = *fresh42;
                    *fresh42 = *fresh42 + 1;
                    *M.offset(fresh43 as isize) = k;
                    let ref mut fresh44 = *count.offset(j as isize);
                    let fresh45 = *fresh44;
                    *fresh44 = *fresh44 + 1;
                    *M.offset(fresh45 as isize) = k;
                    k += 1;
                }
                pp += 1;
            }
            j += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < n {
            *mark.offset(i as isize) = -(1 as ::core::ffi::c_int);
            i += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < n {
            pp = *p.offset(j as isize);
            while pp < *p.offset((j + 1 as ::core::ffi::c_int) as isize) {
                i = *A.offset(pp as isize);
                if i > j && *mark.offset(i as isize) != j {
                    let ref mut fresh46 = *count.offset(i as isize);
                    let fresh47 = *fresh46;
                    *fresh46 = *fresh46 + 1;
                    *M.offset(fresh47 as isize) = k;
                    let ref mut fresh48 = *count.offset(j as isize);
                    let fresh49 = *fresh48;
                    *fresh48 = *fresh48 + 1;
                    *M.offset(fresh49 as isize) = k;
                    k += 1;
                    *mark.offset(i as isize) = j;
                }
                pp += 1;
            }
            j += 1;
        }
        Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
            mark as *mut ::core::ffi::c_void,
        );
    }
    Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
        count as *mut ::core::ffi::c_void,
    );
    i = 0 as ::core::ffi::c_int;
    while i < COLAMD_KNOBS {
        cknobs[i as usize] = *knobs.offset(i as isize);
        i += 1;
    }
    cknobs[COLAMD_DENSE_ROW as usize] = 1.0f64;
    if n_row != 0 as ::core::ffi::c_int && n < n_row {
        cknobs[COLAMD_DENSE_COL as usize] = *knobs.offset(COLAMD_DENSE_ROW as isize)
            * n as ::core::ffi::c_double
            / n_row as ::core::ffi::c_double;
    } else {
        cknobs[COLAMD_DENSE_COL as usize] = 1.0f64;
    }
    if colamd(
        n_row,
        n,
        Mlen,
        M as *mut ::core::ffi::c_int,
        perm,
        &raw mut cknobs as *mut ::core::ffi::c_double,
        &raw mut cstats as *mut ::core::ffi::c_int,
    ) == 0
    {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_internal_error;
        Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
            M as *mut ::core::ffi::c_void,
        );
        return 0 as ::core::ffi::c_int;
    }
    *stats.offset(COLAMD_DENSE_ROW as isize) = cstats[COLAMD_DENSE_COL as usize];
    *stats.offset(COLAMD_DENSE_COL as isize) = cstats[COLAMD_DENSE_COL as usize];
    *stats.offset(COLAMD_DEFRAG_COUNT as isize) = cstats[COLAMD_DEFRAG_COUNT as usize];
    Some(release.expect("non-null function pointer")).expect("non-null function pointer")(
        M as *mut ::core::ffi::c_void,
    );
    return 1 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_colamd"]
pub unsafe extern "C" fn colamd(
    mut n_row: ::core::ffi::c_int,
    mut n_col: ::core::ffi::c_int,
    mut Alen: ::core::ffi::c_int,
    mut A: *mut ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_int,
    mut knobs: *mut ::core::ffi::c_double,
    mut stats: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nnz: ::core::ffi::c_int = 0;
    let mut Row_size: ::core::ffi::c_int = 0;
    let mut Col_size: ::core::ffi::c_int = 0;
    let mut need: ::core::ffi::c_int = 0;
    let mut Row: *mut Colamd_Row = ::core::ptr::null_mut::<Colamd_Row>();
    let mut Col: *mut Colamd_Col = ::core::ptr::null_mut::<Colamd_Col>();
    let mut n_col2: ::core::ffi::c_int = 0;
    let mut n_row2: ::core::ffi::c_int = 0;
    let mut ngarbage: ::core::ffi::c_int = 0;
    let mut max_deg: ::core::ffi::c_int = 0;
    let mut default_knobs: [::core::ffi::c_double; 20] = [0.; 20];
    if stats.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < COLAMD_STATS {
        *stats.offset(i as isize) = 0 as ::core::ffi::c_int;
        i += 1;
    }
    *stats.offset(COLAMD_STATUS as isize) = COLAMD_OK;
    *stats.offset(COLAMD_INFO1 as isize) = -(1 as ::core::ffi::c_int);
    *stats.offset(COLAMD_INFO2 as isize) = -(1 as ::core::ffi::c_int);
    if A.is_null() {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_A_not_present;
        return 0 as ::core::ffi::c_int;
    }
    if p.is_null() {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_p_not_present;
        return 0 as ::core::ffi::c_int;
    }
    if n_row < 0 as ::core::ffi::c_int {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_nrow_negative;
        *stats.offset(COLAMD_INFO1 as isize) = n_row;
        return 0 as ::core::ffi::c_int;
    }
    if n_col < 0 as ::core::ffi::c_int {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_ncol_negative;
        *stats.offset(COLAMD_INFO1 as isize) = n_col;
        return 0 as ::core::ffi::c_int;
    }
    nnz = *p.offset(n_col as isize);
    if nnz < 0 as ::core::ffi::c_int {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_nnz_negative;
        *stats.offset(COLAMD_INFO1 as isize) = nnz;
        return 0 as ::core::ffi::c_int;
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) != 0 as ::core::ffi::c_int {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_p0_nonzero;
        *stats.offset(COLAMD_INFO1 as isize) = *p.offset(0 as ::core::ffi::c_int as isize);
        return 0 as ::core::ffi::c_int;
    }
    if knobs.is_null() {
        colamd_set_defaults(&raw mut default_knobs as *mut ::core::ffi::c_double);
        knobs = &raw mut default_knobs as *mut ::core::ffi::c_double as *mut ::core::ffi::c_double;
    }
    Col_size = ((n_col + 1 as ::core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<Colamd_Col>() as usize)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
        as ::core::ffi::c_int;
    Row_size = ((n_row + 1 as ::core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<Colamd_Row>() as usize)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
        as ::core::ffi::c_int;
    need = 2 as ::core::ffi::c_int * nnz + n_col + Col_size + Row_size;
    if need > Alen {
        *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_A_too_small;
        *stats.offset(COLAMD_INFO1 as isize) = need;
        *stats.offset(COLAMD_INFO2 as isize) = Alen;
        return 0 as ::core::ffi::c_int;
    }
    Alen -= Col_size + Row_size;
    Col = A.offset(Alen as isize) as *mut ::core::ffi::c_int as *mut Colamd_Col;
    Row = A.offset((Alen + Col_size) as isize) as *mut ::core::ffi::c_int as *mut Colamd_Row;
    if init_rows_cols(
        n_row,
        n_col,
        Row as *mut Colamd_Row,
        Col as *mut Colamd_Col,
        A,
        p,
        stats,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    init_scoring(
        n_row,
        n_col,
        Row as *mut Colamd_Row,
        Col as *mut Colamd_Col,
        A,
        p,
        knobs,
        &raw mut n_row2,
        &raw mut n_col2,
        &raw mut max_deg,
    );
    ngarbage = find_ordering(
        n_row,
        n_col,
        Alen,
        Row as *mut Colamd_Row,
        Col as *mut Colamd_Col,
        A,
        p,
        n_col2,
        max_deg,
        2 as ::core::ffi::c_int * nnz,
    );
    order_children(n_col, Col as *mut Colamd_Col, p);
    *stats.offset(COLAMD_DENSE_ROW as isize) = n_row - n_row2;
    *stats.offset(COLAMD_DENSE_COL as isize) = n_col - n_col2;
    *stats.offset(COLAMD_DEFRAG_COUNT as isize) = ngarbage;
    return 1 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_colamd_report"]
pub unsafe extern "C" fn colamd_report(mut stats: *mut ::core::ffi::c_int) {
    print_report(
        b"colamd\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        stats,
    );
}
#[export_name="honest_lpsolve_symamd_report"]
pub unsafe extern "C" fn symamd_report(mut stats: *mut ::core::ffi::c_int) {
    print_report(
        b"symamd\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        stats,
    );
}
unsafe extern "C" fn init_rows_cols(
    mut n_row: ::core::ffi::c_int,
    mut n_col: ::core::ffi::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_int,
    mut stats: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut col: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut cp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut cp_end: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rp_end: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut last_row: ::core::ffi::c_int = 0;
    col = 0 as ::core::ffi::c_int;
    while col < n_col {
        (*Col.offset(col as isize)).start = *p.offset(col as isize);
        (*Col.offset(col as isize)).length =
            *p.offset((col + 1 as ::core::ffi::c_int) as isize) - *p.offset(col as isize);
        if (*Col.offset(col as isize)).length < 0 as ::core::ffi::c_int {
            *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_col_length_negative;
            *stats.offset(COLAMD_INFO1 as isize) = col;
            *stats.offset(COLAMD_INFO2 as isize) = (*Col.offset(col as isize)).length;
            return 0 as ::core::ffi::c_int;
        }
        (*Col.offset(col as isize)).shared1.thickness = 1 as ::core::ffi::c_int;
        (*Col.offset(col as isize)).shared2.score = 0 as ::core::ffi::c_int;
        (*Col.offset(col as isize)).shared3.prev = EMPTY;
        (*Col.offset(col as isize)).shared4.degree_next = EMPTY;
        col += 1;
    }
    *stats.offset(COLAMD_INFO3 as isize) = 0 as ::core::ffi::c_int;
    row = 0 as ::core::ffi::c_int;
    while row < n_row {
        (*Row.offset(row as isize)).length = 0 as ::core::ffi::c_int;
        (*Row.offset(row as isize)).shared2.mark = -(1 as ::core::ffi::c_int);
        row += 1;
    }
    col = 0 as ::core::ffi::c_int;
    while col < n_col {
        last_row = -(1 as ::core::ffi::c_int);
        cp = A.offset(*p.offset(col as isize) as isize) as *mut ::core::ffi::c_int;
        cp_end = A.offset(*p.offset((col + 1 as ::core::ffi::c_int) as isize) as isize)
            as *mut ::core::ffi::c_int;
        while cp < cp_end {
            let fresh26 = cp;
            cp = cp.offset(1);
            row = *fresh26;
            if row < 0 as ::core::ffi::c_int || row >= n_row {
                *stats.offset(COLAMD_STATUS as isize) = COLAMD_ERROR_row_index_out_of_bounds;
                *stats.offset(COLAMD_INFO1 as isize) = col;
                *stats.offset(COLAMD_INFO2 as isize) = row;
                *stats.offset(COLAMD_INFO3 as isize) = n_row;
                return 0 as ::core::ffi::c_int;
            }
            if row <= last_row || (*Row.offset(row as isize)).shared2.mark == col {
                *stats.offset(COLAMD_STATUS as isize) = COLAMD_OK_BUT_JUMBLED;
                *stats.offset(COLAMD_INFO1 as isize) = col;
                *stats.offset(COLAMD_INFO2 as isize) = row;
                let ref mut fresh27 = *stats.offset(COLAMD_INFO3 as isize);
                *fresh27 += 1;
            }
            if (*Row.offset(row as isize)).shared2.mark != col {
                let ref mut fresh28 = (*Row.offset(row as isize)).length;
                *fresh28 += 1;
            } else {
                let ref mut fresh29 = (*Col.offset(col as isize)).length;
                *fresh29 -= 1;
            }
            (*Row.offset(row as isize)).shared2.mark = col;
            last_row = row;
        }
        col += 1;
    }
    (*Row.offset(0 as ::core::ffi::c_int as isize)).start = *p.offset(n_col as isize);
    (*Row.offset(0 as ::core::ffi::c_int as isize)).shared1.p =
        (*Row.offset(0 as ::core::ffi::c_int as isize)).start;
    (*Row.offset(0 as ::core::ffi::c_int as isize)).shared2.mark = -(1 as ::core::ffi::c_int);
    row = 1 as ::core::ffi::c_int;
    while row < n_row {
        (*Row.offset(row as isize)).start = (*Row.offset((row - 1 as ::core::ffi::c_int) as isize))
            .start
            + (*Row.offset((row - 1 as ::core::ffi::c_int) as isize)).length;
        (*Row.offset(row as isize)).shared1.p = (*Row.offset(row as isize)).start;
        (*Row.offset(row as isize)).shared2.mark = -(1 as ::core::ffi::c_int);
        row += 1;
    }
    if *stats.offset(COLAMD_STATUS as isize) == COLAMD_OK_BUT_JUMBLED {
        col = 0 as ::core::ffi::c_int;
        while col < n_col {
            cp = A.offset(*p.offset(col as isize) as isize) as *mut ::core::ffi::c_int;
            cp_end = A.offset(*p.offset((col + 1 as ::core::ffi::c_int) as isize) as isize)
                as *mut ::core::ffi::c_int;
            while cp < cp_end {
                let fresh30 = cp;
                cp = cp.offset(1);
                row = *fresh30;
                if (*Row.offset(row as isize)).shared2.mark != col {
                    let ref mut fresh31 = (*Row.offset(row as isize)).shared1.p;
                    let fresh32 = *fresh31;
                    *fresh31 = *fresh31 + 1;
                    *A.offset(fresh32 as isize) = col;
                    (*Row.offset(row as isize)).shared2.mark = col;
                }
            }
            col += 1;
        }
    } else {
        col = 0 as ::core::ffi::c_int;
        while col < n_col {
            cp = A.offset(*p.offset(col as isize) as isize) as *mut ::core::ffi::c_int;
            cp_end = A.offset(*p.offset((col + 1 as ::core::ffi::c_int) as isize) as isize)
                as *mut ::core::ffi::c_int;
            while cp < cp_end {
                let fresh33 = cp;
                cp = cp.offset(1);
                let ref mut fresh34 = (*Row.offset(*fresh33 as isize)).shared1.p;
                let fresh35 = *fresh34;
                *fresh34 = *fresh34 + 1;
                *A.offset(fresh35 as isize) = col;
            }
            col += 1;
        }
    }
    row = 0 as ::core::ffi::c_int;
    while row < n_row {
        (*Row.offset(row as isize)).shared2.mark = 0 as ::core::ffi::c_int;
        (*Row.offset(row as isize)).shared1.degree = (*Row.offset(row as isize)).length;
        row += 1;
    }
    if *stats.offset(COLAMD_STATUS as isize) == COLAMD_OK_BUT_JUMBLED {
        (*Col.offset(0 as ::core::ffi::c_int as isize)).start = 0 as ::core::ffi::c_int;
        *p.offset(0 as ::core::ffi::c_int as isize) =
            (*Col.offset(0 as ::core::ffi::c_int as isize)).start;
        col = 1 as ::core::ffi::c_int;
        while col < n_col {
            (*Col.offset(col as isize)).start =
                (*Col.offset((col - 1 as ::core::ffi::c_int) as isize)).start
                    + (*Col.offset((col - 1 as ::core::ffi::c_int) as isize)).length;
            *p.offset(col as isize) = (*Col.offset(col as isize)).start;
            col += 1;
        }
        row = 0 as ::core::ffi::c_int;
        while row < n_row {
            rp = A.offset((*Row.offset(row as isize)).start as isize) as *mut ::core::ffi::c_int;
            rp_end = rp.offset((*Row.offset(row as isize)).length as isize);
            while rp < rp_end {
                let fresh36 = rp;
                rp = rp.offset(1);
                let ref mut fresh37 = *p.offset(*fresh36 as isize);
                let fresh38 = *fresh37;
                *fresh37 = *fresh37 + 1;
                *A.offset(fresh38 as isize) = row;
            }
            row += 1;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn init_scoring(
    mut n_row: ::core::ffi::c_int,
    mut n_col: ::core::ffi::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut ::core::ffi::c_int,
    mut head: *mut ::core::ffi::c_int,
    mut knobs: *mut ::core::ffi::c_double,
    mut p_n_row2: *mut ::core::ffi::c_int,
    mut p_n_col2: *mut ::core::ffi::c_int,
    mut p_max_deg: *mut ::core::ffi::c_int,
) {
    let mut c: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut cp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut deg: ::core::ffi::c_int = 0;
    let mut cp_end: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut new_cp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut col_length: ::core::ffi::c_int = 0;
    let mut score: ::core::ffi::c_int = 0;
    let mut n_col2: ::core::ffi::c_int = 0;
    let mut n_row2: ::core::ffi::c_int = 0;
    let mut dense_row_count: ::core::ffi::c_int = 0;
    let mut dense_col_count: ::core::ffi::c_int = 0;
    let mut min_score: ::core::ffi::c_int = 0;
    let mut max_deg: ::core::ffi::c_int = 0;
    let mut next_col: ::core::ffi::c_int = 0;
    dense_row_count = (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
        > (if (*knobs.offset(0 as ::core::ffi::c_int as isize) * n_col as ::core::ffi::c_double)
            < n_col as ::core::ffi::c_double
        {
            *knobs.offset(0 as ::core::ffi::c_int as isize) * n_col as ::core::ffi::c_double
        } else {
            n_col as ::core::ffi::c_double
        }) {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    } else if (*knobs.offset(0 as ::core::ffi::c_int as isize) * n_col as ::core::ffi::c_double)
        < n_col as ::core::ffi::c_double
    {
        *knobs.offset(0 as ::core::ffi::c_int as isize) * n_col as ::core::ffi::c_double
    } else {
        n_col as ::core::ffi::c_double
    }) as ::core::ffi::c_int;
    dense_col_count = (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
        > (if (*knobs.offset(1 as ::core::ffi::c_int as isize) * n_row as ::core::ffi::c_double)
            < n_row as ::core::ffi::c_double
        {
            *knobs.offset(1 as ::core::ffi::c_int as isize) * n_row as ::core::ffi::c_double
        } else {
            n_row as ::core::ffi::c_double
        }) {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    } else if (*knobs.offset(1 as ::core::ffi::c_int as isize) * n_row as ::core::ffi::c_double)
        < n_row as ::core::ffi::c_double
    {
        *knobs.offset(1 as ::core::ffi::c_int as isize) * n_row as ::core::ffi::c_double
    } else {
        n_row as ::core::ffi::c_double
    }) as ::core::ffi::c_int;
    max_deg = 0 as ::core::ffi::c_int;
    n_col2 = n_col;
    n_row2 = n_row;
    c = n_col - 1 as ::core::ffi::c_int;
    while c >= 0 as ::core::ffi::c_int {
        deg = (*Col.offset(c as isize)).length;
        if deg == 0 as ::core::ffi::c_int {
            n_col2 -= 1;
            (*Col.offset(c as isize)).shared2.order = n_col2;
            (*Col.offset(c as isize)).start = DEAD_PRINCIPAL;
        }
        c -= 1;
    }
    c = n_col - 1 as ::core::ffi::c_int;
    while c >= 0 as ::core::ffi::c_int {
        if !((*Col.offset(c as isize)).start < ALIVE) {
            deg = (*Col.offset(c as isize)).length;
            if deg > dense_col_count {
                n_col2 -= 1;
                (*Col.offset(c as isize)).shared2.order = n_col2;
                cp = A.offset((*Col.offset(c as isize)).start as isize) as *mut ::core::ffi::c_int;
                cp_end = cp.offset((*Col.offset(c as isize)).length as isize);
                while cp < cp_end {
                    let fresh22 = cp;
                    cp = cp.offset(1);
                    let ref mut fresh23 = (*Row.offset(*fresh22 as isize)).shared1.degree;
                    *fresh23 -= 1;
                }
                (*Col.offset(c as isize)).start = DEAD_PRINCIPAL;
            }
        }
        c -= 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < n_row {
        deg = (*Row.offset(r as isize)).shared1.degree;
        if deg > dense_row_count || deg == 0 as ::core::ffi::c_int {
            (*Row.offset(r as isize)).shared2.mark = DEAD;
            n_row2 -= 1;
        } else {
            max_deg = if max_deg > deg { max_deg } else { deg };
        }
        r += 1;
    }
    c = n_col - 1 as ::core::ffi::c_int;
    while c >= 0 as ::core::ffi::c_int {
        if !((*Col.offset(c as isize)).start < ALIVE) {
            score = 0 as ::core::ffi::c_int;
            cp = A.offset((*Col.offset(c as isize)).start as isize) as *mut ::core::ffi::c_int;
            new_cp = cp;
            cp_end = cp.offset((*Col.offset(c as isize)).length as isize);
            while cp < cp_end {
                let fresh24 = cp;
                cp = cp.offset(1);
                row = *fresh24;
                if (*Row.offset(row as isize)).shared2.mark < ALIVE {
                    continue;
                }
                let fresh25 = new_cp;
                new_cp = new_cp.offset(1);
                *fresh25 = row;
                score += (*Row.offset(row as isize)).shared1.degree - 1 as ::core::ffi::c_int;
                score = if score < n_col { score } else { n_col };
            }
            col_length =
                new_cp
                    .offset_from(A.offset((*Col.offset(c as isize)).start as isize)
                        as *mut ::core::ffi::c_int) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            if col_length == 0 as ::core::ffi::c_int {
                n_col2 -= 1;
                (*Col.offset(c as isize)).shared2.order = n_col2;
                (*Col.offset(c as isize)).start = DEAD_PRINCIPAL;
            } else {
                (*Col.offset(c as isize)).length = col_length;
                (*Col.offset(c as isize)).shared2.score = score;
            }
        }
        c -= 1;
    }
    c = 0 as ::core::ffi::c_int;
    while c <= n_col {
        *head.offset(c as isize) = EMPTY;
        c += 1;
    }
    min_score = n_col;
    c = n_col - 1 as ::core::ffi::c_int;
    while c >= 0 as ::core::ffi::c_int {
        if (*Col.offset(c as isize)).start >= ALIVE {
            score = (*Col.offset(c as isize)).shared2.score;
            next_col = *head.offset(score as isize);
            (*Col.offset(c as isize)).shared3.prev = EMPTY;
            (*Col.offset(c as isize)).shared4.degree_next = next_col;
            if next_col != EMPTY {
                (*Col.offset(next_col as isize)).shared3.prev = c;
            }
            *head.offset(score as isize) = c;
            min_score = if min_score < score { min_score } else { score };
        }
        c -= 1;
    }
    *p_n_col2 = n_col2;
    *p_n_row2 = n_row2;
    *p_max_deg = max_deg;
}
unsafe extern "C" fn find_ordering(
    mut n_row: ::core::ffi::c_int,
    mut n_col: ::core::ffi::c_int,
    mut Alen: ::core::ffi::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut ::core::ffi::c_int,
    mut head: *mut ::core::ffi::c_int,
    mut n_col2: ::core::ffi::c_int,
    mut max_deg: ::core::ffi::c_int,
    mut pfree: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut pivot_col: ::core::ffi::c_int = 0;
    let mut cp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut pivot_row: ::core::ffi::c_int = 0;
    let mut new_cp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut new_rp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut pivot_row_start: ::core::ffi::c_int = 0;
    let mut pivot_row_degree: ::core::ffi::c_int = 0;
    let mut pivot_row_length: ::core::ffi::c_int = 0;
    let mut pivot_col_score: ::core::ffi::c_int = 0;
    let mut needed_memory: ::core::ffi::c_int = 0;
    let mut cp_end: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rp_end: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut max_score: ::core::ffi::c_int = 0;
    let mut cur_score: ::core::ffi::c_int = 0;
    let mut hash: ::core::ffi::c_uint = 0;
    let mut head_column: ::core::ffi::c_int = 0;
    let mut first_col: ::core::ffi::c_int = 0;
    let mut tag_mark: ::core::ffi::c_int = 0;
    let mut row_mark: ::core::ffi::c_int = 0;
    let mut set_difference: ::core::ffi::c_int = 0;
    let mut min_score: ::core::ffi::c_int = 0;
    let mut col_thickness: ::core::ffi::c_int = 0;
    let mut max_mark: ::core::ffi::c_int = 0;
    let mut pivot_col_thickness: ::core::ffi::c_int = 0;
    let mut prev_col: ::core::ffi::c_int = 0;
    let mut next_col: ::core::ffi::c_int = 0;
    let mut ngarbage: ::core::ffi::c_int = 0;
    max_mark = INT_MAX - n_col;
    tag_mark = clear_mark(n_row, Row);
    min_score = 0 as ::core::ffi::c_int;
    ngarbage = 0 as ::core::ffi::c_int;
    k = 0 as ::core::ffi::c_int;
    while k < n_col2 {
        while *head.offset(min_score as isize) == EMPTY && min_score < n_col {
            min_score += 1;
        }
        pivot_col = *head.offset(min_score as isize);
        next_col = (*Col.offset(pivot_col as isize)).shared4.degree_next;
        *head.offset(min_score as isize) = next_col;
        if next_col != EMPTY {
            (*Col.offset(next_col as isize)).shared3.prev = EMPTY;
        }
        pivot_col_score = (*Col.offset(pivot_col as isize)).shared2.score;
        (*Col.offset(pivot_col as isize)).shared2.order = k;
        pivot_col_thickness = (*Col.offset(pivot_col as isize)).shared1.thickness;
        k += pivot_col_thickness;
        needed_memory = if pivot_col_score < n_col - k {
            pivot_col_score
        } else {
            n_col - k
        };
        if pfree + needed_memory >= Alen {
            pfree = garbage_collection(
                n_row,
                n_col,
                Row,
                Col,
                A,
                A.offset(pfree as isize) as *mut ::core::ffi::c_int,
            );
            ngarbage += 1;
            tag_mark = clear_mark(n_row, Row);
        }
        pivot_row_start = pfree;
        pivot_row_degree = 0 as ::core::ffi::c_int;
        (*Col.offset(pivot_col as isize)).shared1.thickness = -pivot_col_thickness;
        cp = A.offset((*Col.offset(pivot_col as isize)).start as isize) as *mut ::core::ffi::c_int;
        cp_end = cp.offset((*Col.offset(pivot_col as isize)).length as isize);
        while cp < cp_end {
            let fresh1 = cp;
            cp = cp.offset(1);
            row = *fresh1;
            if (*Row.offset(row as isize)).shared2.mark < ALIVE {
                continue;
            }
            rp = A.offset((*Row.offset(row as isize)).start as isize) as *mut ::core::ffi::c_int;
            rp_end = rp.offset((*Row.offset(row as isize)).length as isize);
            while rp < rp_end {
                let fresh2 = rp;
                rp = rp.offset(1);
                col = *fresh2;
                col_thickness = (*Col.offset(col as isize)).shared1.thickness;
                if col_thickness > 0 as ::core::ffi::c_int
                    && (*Col.offset(col as isize)).start >= ALIVE
                {
                    (*Col.offset(col as isize)).shared1.thickness = -col_thickness;
                    let fresh3 = pfree;
                    pfree = pfree + 1;
                    *A.offset(fresh3 as isize) = col;
                    pivot_row_degree += col_thickness;
                }
            }
        }
        (*Col.offset(pivot_col as isize)).shared1.thickness = pivot_col_thickness;
        max_deg = if max_deg > pivot_row_degree {
            max_deg
        } else {
            pivot_row_degree
        };
        cp = A.offset((*Col.offset(pivot_col as isize)).start as isize) as *mut ::core::ffi::c_int;
        cp_end = cp.offset((*Col.offset(pivot_col as isize)).length as isize);
        while cp < cp_end {
            let fresh4 = cp;
            cp = cp.offset(1);
            row = *fresh4;
            (*Row.offset(row as isize)).shared2.mark = DEAD;
        }
        pivot_row_length = pfree - pivot_row_start;
        if pivot_row_length > 0 as ::core::ffi::c_int {
            pivot_row = *A.offset((*Col.offset(pivot_col as isize)).start as isize);
        } else {
            pivot_row = EMPTY;
        }
        rp = A.offset(pivot_row_start as isize) as *mut ::core::ffi::c_int;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh5 = rp;
            rp = rp.offset(1);
            col = *fresh5;
            col_thickness = -(*Col.offset(col as isize)).shared1.thickness;
            (*Col.offset(col as isize)).shared1.thickness = col_thickness;
            cur_score = (*Col.offset(col as isize)).shared2.score;
            prev_col = (*Col.offset(col as isize)).shared3.prev;
            next_col = (*Col.offset(col as isize)).shared4.degree_next;
            if prev_col == EMPTY {
                *head.offset(cur_score as isize) = next_col;
            } else {
                (*Col.offset(prev_col as isize)).shared4.degree_next = next_col;
            }
            if next_col != EMPTY {
                (*Col.offset(next_col as isize)).shared3.prev = prev_col;
            }
            cp = A.offset((*Col.offset(col as isize)).start as isize) as *mut ::core::ffi::c_int;
            cp_end = cp.offset((*Col.offset(col as isize)).length as isize);
            while cp < cp_end {
                let fresh6 = cp;
                cp = cp.offset(1);
                row = *fresh6;
                row_mark = (*Row.offset(row as isize)).shared2.mark;
                if row_mark < ALIVE {
                    continue;
                }
                set_difference = row_mark - tag_mark;
                if set_difference < 0 as ::core::ffi::c_int {
                    set_difference = (*Row.offset(row as isize)).shared1.degree;
                }
                set_difference -= col_thickness;
                if set_difference == 0 as ::core::ffi::c_int {
                    (*Row.offset(row as isize)).shared2.mark = DEAD;
                } else {
                    (*Row.offset(row as isize)).shared2.mark = set_difference + tag_mark;
                }
            }
        }
        rp = A.offset(pivot_row_start as isize) as *mut ::core::ffi::c_int;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh7 = rp;
            rp = rp.offset(1);
            col = *fresh7;
            hash = 0 as ::core::ffi::c_uint;
            cur_score = 0 as ::core::ffi::c_int;
            cp = A.offset((*Col.offset(col as isize)).start as isize) as *mut ::core::ffi::c_int;
            new_cp = cp;
            cp_end = cp.offset((*Col.offset(col as isize)).length as isize);
            while cp < cp_end {
                let fresh8 = cp;
                cp = cp.offset(1);
                row = *fresh8;
                row_mark = (*Row.offset(row as isize)).shared2.mark;
                if row_mark < ALIVE {
                    continue;
                }
                let fresh9 = new_cp;
                new_cp = new_cp.offset(1);
                *fresh9 = row;
                hash = hash.wrapping_add(row as ::core::ffi::c_uint);
                cur_score += row_mark - tag_mark;
                cur_score = if cur_score < n_col { cur_score } else { n_col };
            }
            (*Col.offset(col as isize)).length =
                new_cp
                    .offset_from(A.offset((*Col.offset(col as isize)).start as isize)
                        as *mut ::core::ffi::c_int) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            if (*Col.offset(col as isize)).length == 0 as ::core::ffi::c_int {
                (*Col.offset(col as isize)).start = DEAD_PRINCIPAL;
                pivot_row_degree -= (*Col.offset(col as isize)).shared1.thickness;
                (*Col.offset(col as isize)).shared2.order = k;
                k += (*Col.offset(col as isize)).shared1.thickness;
            } else {
                (*Col.offset(col as isize)).shared2.score = cur_score;
                hash = hash.wrapping_rem((n_col + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                head_column = *head.offset(hash as isize);
                if head_column > EMPTY {
                    first_col = (*Col.offset(head_column as isize)).shared3.headhash;
                    (*Col.offset(head_column as isize)).shared3.headhash = col;
                } else {
                    first_col = -(head_column + 2 as ::core::ffi::c_int);
                    *head.offset(hash as isize) = -(col + 2 as ::core::ffi::c_int);
                }
                (*Col.offset(col as isize)).shared4.hash_next = first_col;
                (*Col.offset(col as isize)).shared3.hash = hash as ::core::ffi::c_int;
            }
        }
        detect_super_cols(Col, A, head, pivot_row_start, pivot_row_length);
        (*Col.offset(pivot_col as isize)).start = DEAD_PRINCIPAL;
        tag_mark += max_deg + 1 as ::core::ffi::c_int;
        if tag_mark >= max_mark {
            tag_mark = clear_mark(n_row, Row);
        }
        rp = A.offset(pivot_row_start as isize) as *mut ::core::ffi::c_int;
        new_rp = rp;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh10 = rp;
            rp = rp.offset(1);
            col = *fresh10;
            if (*Col.offset(col as isize)).start < ALIVE {
                continue;
            }
            let fresh11 = new_rp;
            new_rp = new_rp.offset(1);
            *fresh11 = col;
            let ref mut fresh12 = (*Col.offset(col as isize)).length;
            let fresh13 = *fresh12;
            *fresh12 = *fresh12 + 1;
            *A.offset(((*Col.offset(col as isize)).start + fresh13) as isize) = pivot_row;
            cur_score = (*Col.offset(col as isize)).shared2.score + pivot_row_degree;
            max_score = n_col - k - (*Col.offset(col as isize)).shared1.thickness;
            cur_score -= (*Col.offset(col as isize)).shared1.thickness;
            cur_score = if cur_score < max_score {
                cur_score
            } else {
                max_score
            };
            (*Col.offset(col as isize)).shared2.score = cur_score;
            next_col = *head.offset(cur_score as isize);
            (*Col.offset(col as isize)).shared4.degree_next = next_col;
            (*Col.offset(col as isize)).shared3.prev = EMPTY;
            if next_col != EMPTY {
                (*Col.offset(next_col as isize)).shared3.prev = col;
            }
            *head.offset(cur_score as isize) = col;
            min_score = if min_score < cur_score {
                min_score
            } else {
                cur_score
            };
        }
        if pivot_row_degree > 0 as ::core::ffi::c_int {
            (*Row.offset(pivot_row as isize)).start = pivot_row_start;
            (*Row.offset(pivot_row as isize)).length =
                new_rp.offset_from(A.offset(pivot_row_start as isize) as *mut ::core::ffi::c_int)
                    as ::core::ffi::c_long as ::core::ffi::c_int;
            (*Row.offset(pivot_row as isize)).shared1.degree = pivot_row_degree;
            (*Row.offset(pivot_row as isize)).shared2.mark = 0 as ::core::ffi::c_int;
        }
    }
    return ngarbage;
}
unsafe extern "C" fn order_children(
    mut n_col: ::core::ffi::c_int,
    mut Col: *mut Colamd_Col,
    mut p: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut parent: ::core::ffi::c_int = 0;
    let mut order: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < n_col {
        if !((*Col.offset(i as isize)).start == DEAD_PRINCIPAL)
            && (*Col.offset(i as isize)).shared2.order == EMPTY
        {
            parent = i;
            loop {
                parent = (*Col.offset(parent as isize)).shared1.parent;
                if (*Col.offset(parent as isize)).start == DEAD_PRINCIPAL {
                    break;
                }
            }
            c = i;
            order = (*Col.offset(parent as isize)).shared2.order;
            loop {
                let fresh0 = order;
                order = order + 1;
                (*Col.offset(c as isize)).shared2.order = fresh0;
                (*Col.offset(c as isize)).shared1.parent = parent;
                c = (*Col.offset(c as isize)).shared1.parent;
                if !((*Col.offset(c as isize)).shared2.order == EMPTY) {
                    break;
                }
            }
            (*Col.offset(parent as isize)).shared2.order = order;
        }
        i += 1;
    }
    c = 0 as ::core::ffi::c_int;
    while c < n_col {
        *p.offset((*Col.offset(c as isize)).shared2.order as isize) = c;
        c += 1;
    }
}
unsafe extern "C" fn detect_super_cols(
    mut Col: *mut Colamd_Col,
    mut A: *mut ::core::ffi::c_int,
    mut head: *mut ::core::ffi::c_int,
    mut row_start: ::core::ffi::c_int,
    mut row_length: ::core::ffi::c_int,
) {
    let mut hash: ::core::ffi::c_int = 0;
    let mut rp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut c: ::core::ffi::c_int = 0;
    let mut super_c: ::core::ffi::c_int = 0;
    let mut cp1: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut cp2: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut length: ::core::ffi::c_int = 0;
    let mut prev_c: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut rp_end: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut col: ::core::ffi::c_int = 0;
    let mut head_column: ::core::ffi::c_int = 0;
    let mut first_col: ::core::ffi::c_int = 0;
    rp = A.offset(row_start as isize) as *mut ::core::ffi::c_int;
    rp_end = rp.offset(row_length as isize);
    while rp < rp_end {
        let fresh14 = rp;
        rp = rp.offset(1);
        col = *fresh14;
        if (*Col.offset(col as isize)).start < ALIVE {
            continue;
        }
        hash = (*Col.offset(col as isize)).shared3.hash;
        head_column = *head.offset(hash as isize);
        if head_column > EMPTY {
            first_col = (*Col.offset(head_column as isize)).shared3.headhash;
        } else {
            first_col = -(head_column + 2 as ::core::ffi::c_int);
        }
        super_c = first_col;
        while super_c != EMPTY {
            length = (*Col.offset(super_c as isize)).length;
            prev_c = super_c;
            c = (*Col.offset(super_c as isize)).shared4.hash_next;
            while c != EMPTY {
                if (*Col.offset(c as isize)).length != length
                    || (*Col.offset(c as isize)).shared2.score
                        != (*Col.offset(super_c as isize)).shared2.score
                {
                    prev_c = c;
                } else {
                    cp1 = A.offset((*Col.offset(super_c as isize)).start as isize)
                        as *mut ::core::ffi::c_int;
                    cp2 = A.offset((*Col.offset(c as isize)).start as isize)
                        as *mut ::core::ffi::c_int;
                    i = 0 as ::core::ffi::c_int;
                    while i < length {
                        let fresh15 = cp1;
                        cp1 = cp1.offset(1);
                        let fresh16 = cp2;
                        cp2 = cp2.offset(1);
                        if *fresh15 != *fresh16 {
                            break;
                        }
                        i += 1;
                    }
                    if i != length {
                        prev_c = c;
                    } else {
                        (*Col.offset(super_c as isize)).shared1.thickness +=
                            (*Col.offset(c as isize)).shared1.thickness;
                        (*Col.offset(c as isize)).shared1.parent = super_c;
                        (*Col.offset(c as isize)).start = DEAD_NON_PRINCIPAL;
                        (*Col.offset(c as isize)).shared2.order = EMPTY;
                        (*Col.offset(prev_c as isize)).shared4.hash_next =
                            (*Col.offset(c as isize)).shared4.hash_next;
                    }
                }
                c = (*Col.offset(c as isize)).shared4.hash_next;
            }
            super_c = (*Col.offset(super_c as isize)).shared4.hash_next;
        }
        if head_column > EMPTY {
            (*Col.offset(head_column as isize)).shared3.headhash = EMPTY;
        } else {
            *head.offset(hash as isize) = EMPTY;
        }
    }
}
unsafe extern "C" fn garbage_collection(
    mut n_row: ::core::ffi::c_int,
    mut n_col: ::core::ffi::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut ::core::ffi::c_int,
    mut pfree: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut psrc: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut pdest: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut j: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    pdest = A.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    c = 0 as ::core::ffi::c_int;
    while c < n_col {
        if (*Col.offset(c as isize)).start >= ALIVE {
            psrc = A.offset((*Col.offset(c as isize)).start as isize) as *mut ::core::ffi::c_int;
            (*Col.offset(c as isize)).start = pdest
                .offset_from(A.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int)
                as ::core::ffi::c_long
                as ::core::ffi::c_int;
            length = (*Col.offset(c as isize)).length;
            j = 0 as ::core::ffi::c_int;
            while j < length {
                let fresh17 = psrc;
                psrc = psrc.offset(1);
                r = *fresh17;
                if (*Row.offset(r as isize)).shared2.mark >= ALIVE {
                    let fresh18 = pdest;
                    pdest = pdest.offset(1);
                    *fresh18 = r;
                }
                j += 1;
            }
            (*Col.offset(c as isize)).length =
                pdest
                    .offset_from(A.offset((*Col.offset(c as isize)).start as isize)
                        as *mut ::core::ffi::c_int) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
        }
        c += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < n_row {
        if (*Row.offset(r as isize)).shared2.mark >= ALIVE {
            if (*Row.offset(r as isize)).length == 0 as ::core::ffi::c_int {
                (*Row.offset(r as isize)).shared2.mark = DEAD;
            } else {
                psrc =
                    A.offset((*Row.offset(r as isize)).start as isize) as *mut ::core::ffi::c_int;
                (*Row.offset(r as isize)).shared2.first_column = *psrc;
                *psrc = -r - 1 as ::core::ffi::c_int;
            }
        }
        r += 1;
    }
    psrc = pdest;
    while psrc < pfree {
        let fresh19 = psrc;
        psrc = psrc.offset(1);
        if *fresh19 < 0 as ::core::ffi::c_int {
            psrc = psrc.offset(-1);
            r = -*psrc - 1 as ::core::ffi::c_int;
            *psrc = (*Row.offset(r as isize)).shared2.first_column;
            (*Row.offset(r as isize)).start = pdest
                .offset_from(A.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int)
                as ::core::ffi::c_long
                as ::core::ffi::c_int;
            length = (*Row.offset(r as isize)).length;
            j = 0 as ::core::ffi::c_int;
            while j < length {
                let fresh20 = psrc;
                psrc = psrc.offset(1);
                c = *fresh20;
                if (*Col.offset(c as isize)).start >= ALIVE {
                    let fresh21 = pdest;
                    pdest = pdest.offset(1);
                    *fresh21 = c;
                }
                j += 1;
            }
            (*Row.offset(r as isize)).length =
                pdest
                    .offset_from(A.offset((*Row.offset(r as isize)).start as isize)
                        as *mut ::core::ffi::c_int) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
        }
    }
    return pdest.offset_from(A.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int)
        as ::core::ffi::c_long as ::core::ffi::c_int;
}
unsafe extern "C" fn clear_mark(
    mut n_row: ::core::ffi::c_int,
    mut Row: *mut Colamd_Row,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < n_row {
        if (*Row.offset(r as isize)).shared2.mark >= ALIVE {
            (*Row.offset(r as isize)).shared2.mark = 0 as ::core::ffi::c_int;
        }
        r += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn print_report(
    mut method: *mut ::core::ffi::c_char,
    mut stats: *mut ::core::ffi::c_int,
) {
    let mut i1: ::core::ffi::c_int = 0;
    let mut i2: ::core::ffi::c_int = 0;
    let mut i3: ::core::ffi::c_int = 0;
    if stats.is_null() {
        return;
    }
    i1 = *stats.offset(COLAMD_INFO1 as isize);
    i2 = *stats.offset(COLAMD_INFO2 as isize);
    i3 = *stats.offset(COLAMD_INFO3 as isize);
    *stats.offset(COLAMD_STATUS as isize) >= 0 as ::core::ffi::c_int;
    match *stats.offset(COLAMD_STATUS as isize) {
        COLAMD_ERROR_internal_error => {}
        COLAMD_OK_BUT_JUMBLED
        | COLAMD_OK
        | COLAMD_ERROR_A_not_present
        | COLAMD_ERROR_p_not_present
        | COLAMD_ERROR_nrow_negative
        | COLAMD_ERROR_ncol_negative
        | COLAMD_ERROR_nnz_negative
        | COLAMD_ERROR_p0_nonzero
        | COLAMD_ERROR_A_too_small
        | COLAMD_ERROR_col_length_negative
        | COLAMD_ERROR_row_index_out_of_bounds
        | COLAMD_ERROR_out_of_memory
        | _ => {}
    };
}
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
