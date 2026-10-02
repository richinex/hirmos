use crate::honest_did::lpsolve::runtime::{strlen,strcmp,strncmp};
use crate::honest_did::lpsolve::runtime::{malloc};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
extern "C" {
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    static mut __stdinp: *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        __size: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fscanf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type MM_typecode = [::core::ffi::c_char; 4];
#[inline]
unsafe extern "C" fn tolower(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return native_only!(__tolower,_c as __darwin_ct_rune_t) as ::core::ffi::c_int;
}
pub const MM_MAX_LINE_LENGTH: ::core::ffi::c_int = 1025 as ::core::ffi::c_int;
pub const MatrixMarketBanner: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"%%MatrixMarket\0") };
pub const MM_COULD_NOT_READ_FILE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MM_PREMATURE_EOF: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MM_NO_HEADER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const MM_UNSUPPORTED_TYPE: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const MM_COULD_NOT_WRITE_FILE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const MM_MTX_STR: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"matrix\0") };
pub const MM_DENSE_STR: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"array\0") };
pub const MM_SPARSE_STR: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"coordinate\0") };
pub const MM_COMPLEX_STR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"complex\0") };
pub const MM_REAL_STR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"real\0") };
pub const MM_INT_STR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"integer\0") };
pub const MM_GENERAL_STR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"general\0") };
pub const MM_SYMM_STR: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"symmetric\0") };
pub const MM_HERM_STR: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"hermitian\0") };
pub const MM_SKEW_STR: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"skew-symmetric\0") };
pub const MM_PATTERN_STR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"pattern\0") };
#[export_name="honest_lpsolve_mm_is_valid"]
pub unsafe extern "C" fn mm_is_valid(mut matcode: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    if !(*matcode.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'M' as i32) {
        return 0 as ::core::ffi::c_int;
    }
    if *matcode.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'A' as i32
        && *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'P' as i32
    {
        return 0 as ::core::ffi::c_int;
    }
    if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'R' as i32
        && *matcode.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'H' as i32
    {
        return 0 as ::core::ffi::c_int;
    }
    if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'P' as i32
        && (*matcode.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'H' as i32
            || *matcode.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'K' as i32)
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_read_banner"]
pub unsafe extern "C" fn mm_read_banner(
    mut f: *mut FILE,
    mut matcode: *mut MM_typecode,
) -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut banner: [::core::ffi::c_char; 64] = [0; 64];
    let mut mtx: [::core::ffi::c_char; 64] = [0; 64];
    let mut crd: [::core::ffi::c_char; 64] = [0; 64];
    let mut data_type: [::core::ffi::c_char; 64] = [0; 64];
    let mut storage_scheme: [::core::ffi::c_char; 64] = [0; 64];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*matcode)[2 as ::core::ffi::c_int as usize] = ' ' as i32 as ::core::ffi::c_char;
    (*matcode)[1 as ::core::ffi::c_int as usize] = (*matcode)[2 as ::core::ffi::c_int as usize];
    (*matcode)[0 as ::core::ffi::c_int as usize] = (*matcode)[1 as ::core::ffi::c_int as usize];
    (*matcode)[3 as ::core::ffi::c_int as usize] = 'G' as i32 as ::core::ffi::c_char;
    if native_only!(fgets,
        &raw mut line as *mut ::core::ffi::c_char,
        MM_MAX_LINE_LENGTH,
        f,
    )
    .is_null()
    {
        return MM_PREMATURE_EOF;
    }
    if native_only!(sscanf,
        &raw mut line as *mut ::core::ffi::c_char,
        b"%s %s %s %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut banner as *mut ::core::ffi::c_char,
        &raw mut mtx as *mut ::core::ffi::c_char,
        &raw mut crd as *mut ::core::ffi::c_char,
        &raw mut data_type as *mut ::core::ffi::c_char,
        &raw mut storage_scheme as *mut ::core::ffi::c_char,
    ) != 5 as ::core::ffi::c_int
    {
        return MM_PREMATURE_EOF;
    }
    p = &raw mut mtx as *mut ::core::ffi::c_char;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        *p = tolower(*p as ::core::ffi::c_int) as ::core::ffi::c_char;
        p = p.offset(1);
    }
    p = &raw mut crd as *mut ::core::ffi::c_char;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        *p = tolower(*p as ::core::ffi::c_int) as ::core::ffi::c_char;
        p = p.offset(1);
    }
    p = &raw mut data_type as *mut ::core::ffi::c_char;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        *p = tolower(*p as ::core::ffi::c_int) as ::core::ffi::c_char;
        p = p.offset(1);
    }
    p = &raw mut storage_scheme as *mut ::core::ffi::c_char;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        *p = tolower(*p as ::core::ffi::c_int) as ::core::ffi::c_char;
        p = p.offset(1);
    }
    if strncmp(
        &raw mut banner as *mut ::core::ffi::c_char,
        MatrixMarketBanner.as_ptr(),
        strlen(MatrixMarketBanner.as_ptr()),
    ) != 0 as ::core::ffi::c_int
    {
        return MM_NO_HEADER;
    }
    if strcmp(
        &raw mut mtx as *mut ::core::ffi::c_char,
        MM_MTX_STR.as_ptr(),
    ) != 0 as ::core::ffi::c_int
    {
        return MM_UNSUPPORTED_TYPE;
    }
    (*matcode)[0 as ::core::ffi::c_int as usize] = 'M' as i32 as ::core::ffi::c_char;
    if strcmp(
        &raw mut crd as *mut ::core::ffi::c_char,
        MM_SPARSE_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[1 as ::core::ffi::c_int as usize] = 'C' as i32 as ::core::ffi::c_char;
    } else if strcmp(
        &raw mut crd as *mut ::core::ffi::c_char,
        MM_DENSE_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[1 as ::core::ffi::c_int as usize] = 'A' as i32 as ::core::ffi::c_char;
    } else {
        return MM_UNSUPPORTED_TYPE;
    }
    if strcmp(
        &raw mut data_type as *mut ::core::ffi::c_char,
        MM_REAL_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[2 as ::core::ffi::c_int as usize] = 'R' as i32 as ::core::ffi::c_char;
    } else if strcmp(
        &raw mut data_type as *mut ::core::ffi::c_char,
        MM_COMPLEX_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[2 as ::core::ffi::c_int as usize] = 'C' as i32 as ::core::ffi::c_char;
    } else if strcmp(
        &raw mut data_type as *mut ::core::ffi::c_char,
        MM_PATTERN_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[2 as ::core::ffi::c_int as usize] = 'P' as i32 as ::core::ffi::c_char;
    } else if strcmp(
        &raw mut data_type as *mut ::core::ffi::c_char,
        MM_INT_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[2 as ::core::ffi::c_int as usize] = 'I' as i32 as ::core::ffi::c_char;
    } else {
        return MM_UNSUPPORTED_TYPE;
    }
    if strcmp(
        &raw mut storage_scheme as *mut ::core::ffi::c_char,
        MM_GENERAL_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[3 as ::core::ffi::c_int as usize] = 'G' as i32 as ::core::ffi::c_char;
    } else if strcmp(
        &raw mut storage_scheme as *mut ::core::ffi::c_char,
        MM_SYMM_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[3 as ::core::ffi::c_int as usize] = 'S' as i32 as ::core::ffi::c_char;
    } else if strcmp(
        &raw mut storage_scheme as *mut ::core::ffi::c_char,
        MM_HERM_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[3 as ::core::ffi::c_int as usize] = 'H' as i32 as ::core::ffi::c_char;
    } else if strcmp(
        &raw mut storage_scheme as *mut ::core::ffi::c_char,
        MM_SKEW_STR.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*matcode)[3 as ::core::ffi::c_int as usize] = 'K' as i32 as ::core::ffi::c_char;
    } else {
        return MM_UNSUPPORTED_TYPE;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_write_mtx_crd_size"]
pub unsafe extern "C" fn mm_write_mtx_crd_size(
    mut f: *mut FILE,
    mut M: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut nz: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if native_only!(fprintf,
        f,
        b"%d %d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        M,
        N,
        nz,
    ) < 0 as ::core::ffi::c_int
    {
        return MM_COULD_NOT_WRITE_FILE;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_mm_read_mtx_crd_size"]
pub unsafe extern "C" fn mm_read_mtx_crd_size(
    mut f: *mut FILE,
    mut M: *mut ::core::ffi::c_int,
    mut N: *mut ::core::ffi::c_int,
    mut nz: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut num_items_read: ::core::ffi::c_int = 0;
    *nz = 0 as ::core::ffi::c_int;
    *N = *nz;
    *M = *N;
    loop {
        if native_only!(fgets,
            &raw mut line as *mut ::core::ffi::c_char,
            MM_MAX_LINE_LENGTH,
            f,
        )
        .is_null()
        {
            return MM_PREMATURE_EOF;
        }
        if !(line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '%' as i32) {
            break;
        }
    }
    if native_only!(sscanf,
        &raw mut line as *mut ::core::ffi::c_char,
        b"%d %d %d\0" as *const u8 as *const ::core::ffi::c_char,
        M,
        N,
        nz,
    ) >= 2 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    } else {
        loop {
            num_items_read = native_only!(fscanf,
                f,
                b"%d %d %d\0" as *const u8 as *const ::core::ffi::c_char,
                M,
                N,
                nz,
            );
            if num_items_read == EOF {
                return MM_PREMATURE_EOF;
            }
            if !(num_items_read < 2 as ::core::ffi::c_int) {
                break;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_read_mtx_array_size"]
pub unsafe extern "C" fn mm_read_mtx_array_size(
    mut f: *mut FILE,
    mut M: *mut ::core::ffi::c_int,
    mut N: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut num_items_read: ::core::ffi::c_int = 0;
    *N = 0 as ::core::ffi::c_int;
    *M = *N;
    loop {
        if native_only!(fgets,
            &raw mut line as *mut ::core::ffi::c_char,
            MM_MAX_LINE_LENGTH,
            f,
        )
        .is_null()
        {
            return MM_PREMATURE_EOF;
        }
        if !(line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '%' as i32) {
            break;
        }
    }
    if native_only!(sscanf,
        &raw mut line as *mut ::core::ffi::c_char,
        b"%d %d\0" as *const u8 as *const ::core::ffi::c_char,
        M,
        N,
    ) == 2 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    } else {
        loop {
            num_items_read = native_only!(fscanf,
                f,
                b"%d %d\0" as *const u8 as *const ::core::ffi::c_char,
                M,
                N,
            );
            if num_items_read == EOF {
                return MM_PREMATURE_EOF;
            }
            if !(num_items_read != 2 as ::core::ffi::c_int) {
                break;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_write_mtx_array_size"]
pub unsafe extern "C" fn mm_write_mtx_array_size(
    mut f: *mut FILE,
    mut M: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if native_only!(fprintf,
        f,
        b"%d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        M,
        N,
    ) < 0 as ::core::ffi::c_int
    {
        return MM_COULD_NOT_WRITE_FILE;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_mm_read_mtx_crd_data"]
pub unsafe extern "C" fn mm_read_mtx_crd_data(
    mut f: *mut FILE,
    mut M: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut nz: ::core::ffi::c_int,
    mut I: *mut ::core::ffi::c_int,
    mut J: *mut ::core::ffi::c_int,
    mut val: *mut ::core::ffi::c_double,
    mut matcode: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'C' as i32 {
        i = 0 as ::core::ffi::c_int;
        while i < nz {
            if native_only!(fscanf,
                f,
                b"%d %d %lg %lg\0" as *const u8 as *const ::core::ffi::c_char,
                I.offset(i as isize) as *mut ::core::ffi::c_int,
                J.offset(i as isize) as *mut ::core::ffi::c_int,
                val.offset((2 as ::core::ffi::c_int * i) as isize) as *mut ::core::ffi::c_double,
                val.offset((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) != 4 as ::core::ffi::c_int
            {
                return MM_PREMATURE_EOF;
            }
            i += 1;
        }
    } else if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'R' as i32
    {
        i = 0 as ::core::ffi::c_int;
        while i < nz {
            if native_only!(fscanf,
                f,
                b"%d %d %lg\n\0" as *const u8 as *const ::core::ffi::c_char,
                I.offset(i as isize) as *mut ::core::ffi::c_int,
                J.offset(i as isize) as *mut ::core::ffi::c_int,
                val.offset(i as isize) as *mut ::core::ffi::c_double,
            ) != 3 as ::core::ffi::c_int
            {
                return MM_PREMATURE_EOF;
            }
            i += 1;
        }
    } else if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'P' as i32
    {
        i = 0 as ::core::ffi::c_int;
        while i < nz {
            if native_only!(fscanf,
                f,
                b"%d %d\0" as *const u8 as *const ::core::ffi::c_char,
                I.offset(i as isize) as *mut ::core::ffi::c_int,
                J.offset(i as isize) as *mut ::core::ffi::c_int,
            ) != 2 as ::core::ffi::c_int
            {
                return MM_PREMATURE_EOF;
            }
            i += 1;
        }
    } else {
        return MM_UNSUPPORTED_TYPE;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_read_mtx_crd_entry"]
pub unsafe extern "C" fn mm_read_mtx_crd_entry(
    mut f: *mut FILE,
    mut I: *mut ::core::ffi::c_int,
    mut J: *mut ::core::ffi::c_int,
    mut real: *mut ::core::ffi::c_double,
    mut imag: *mut ::core::ffi::c_double,
    mut matcode: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'C' as i32 {
        if native_only!(fscanf,
            f,
            b"%d %d %lg %lg\0" as *const u8 as *const ::core::ffi::c_char,
            I,
            J,
            real,
            imag,
        ) != 4 as ::core::ffi::c_int
        {
            return MM_PREMATURE_EOF;
        }
    } else if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'R' as i32
    {
        if native_only!(fscanf,
            f,
            b"%d %d %lg\n\0" as *const u8 as *const ::core::ffi::c_char,
            I,
            J,
            real,
        ) != 3 as ::core::ffi::c_int
        {
            return MM_PREMATURE_EOF;
        }
    } else if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'P' as i32
    {
        if native_only!(fscanf,
            f,
            b"%d %d\0" as *const u8 as *const ::core::ffi::c_char,
            I,
            J,
        ) != 2 as ::core::ffi::c_int
        {
            return MM_PREMATURE_EOF;
        }
    } else {
        return MM_UNSUPPORTED_TYPE;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_read_mtx_crd"]
pub unsafe extern "C" fn mm_read_mtx_crd(
    mut fname: *mut ::core::ffi::c_char,
    mut M: *mut ::core::ffi::c_int,
    mut N: *mut ::core::ffi::c_int,
    mut nz: *mut ::core::ffi::c_int,
    mut I: *mut *mut ::core::ffi::c_int,
    mut J: *mut *mut ::core::ffi::c_int,
    mut val: *mut *mut ::core::ffi::c_double,
    mut matcode: *mut MM_typecode,
) -> ::core::ffi::c_int {
    let mut ret_code: ::core::ffi::c_int = 0;
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    if strcmp(fname, b"stdin\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        f = __stdinp;
    } else {
        f = native_only!(fopen,fname, b"r\0" as *const u8 as *const ::core::ffi::c_char);
        if f.is_null() {
            return MM_COULD_NOT_READ_FILE;
        }
    }
    ret_code = mm_read_banner(f, matcode);
    if ret_code != 0 as ::core::ffi::c_int {
        return ret_code;
    }
    if !(mm_is_valid(&raw mut *matcode as *mut ::core::ffi::c_char) != 0
        && (*matcode)[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'C' as i32
        && (*matcode)[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'M' as i32)
    {
        return MM_UNSUPPORTED_TYPE;
    }
    ret_code = mm_read_mtx_crd_size(f, M, N, nz);
    if ret_code != 0 as ::core::ffi::c_int {
        return ret_code;
    }
    *I = malloc(
        (*nz as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    *J = malloc(
        (*nz as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    *val = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if (*matcode)[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'C' as i32 {
        *val = malloc(
            ((*nz * 2 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        ) as *mut ::core::ffi::c_double;
        ret_code = mm_read_mtx_crd_data(
            f,
            *M,
            *N,
            *nz,
            *I,
            *J,
            *val,
            &raw mut *matcode as *mut ::core::ffi::c_char,
        );
        if ret_code != 0 as ::core::ffi::c_int {
            return ret_code;
        }
    } else if (*matcode)[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'R' as i32 {
        *val = malloc(
            (*nz as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        ) as *mut ::core::ffi::c_double;
        ret_code = mm_read_mtx_crd_data(
            f,
            *M,
            *N,
            *nz,
            *I,
            *J,
            *val,
            &raw mut *matcode as *mut ::core::ffi::c_char,
        );
        if ret_code != 0 as ::core::ffi::c_int {
            return ret_code;
        }
    } else if (*matcode)[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'P' as i32 {
        ret_code = mm_read_mtx_crd_data(
            f,
            *M,
            *N,
            *nz,
            *I,
            *J,
            *val,
            &raw mut *matcode as *mut ::core::ffi::c_char,
        );
        if ret_code != 0 as ::core::ffi::c_int {
            return ret_code;
        }
    }
    if f != __stdinp {
        native_only!(fclose,f);
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_write_banner"]
pub unsafe extern "C" fn mm_write_banner(
    mut f: *mut FILE,
    mut matcode: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut str: *mut ::core::ffi::c_char = mm_typecode_to_str(matcode);
    let mut ret_code: ::core::ffi::c_int = 0;
    ret_code = native_only!(fprintf,
        f,
        b"%s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        MatrixMarketBanner.as_ptr(),
        str,
    );
    if ret_code < 0 as ::core::ffi::c_int {
        return MM_COULD_NOT_WRITE_FILE;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_mm_write_mtx_crd"]
pub unsafe extern "C" fn mm_write_mtx_crd(
    mut fname: *mut ::core::ffi::c_char,
    mut M: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut nz: ::core::ffi::c_int,
    mut I: *mut ::core::ffi::c_int,
    mut J: *mut ::core::ffi::c_int,
    mut val: *mut ::core::ffi::c_double,
    mut matcode: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mm_typecode_to_str"]
pub unsafe extern "C" fn mm_typecode_to_str(
    mut matcode: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut buffer: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut types: [*mut ::core::ffi::c_char; 4] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 4];
    if *matcode.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'M' as i32 {
        types[0 as ::core::ffi::c_int as usize] = MM_MTX_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *matcode.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'C' as i32 {
        types[1 as ::core::ffi::c_int as usize] =
            MM_SPARSE_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else if *matcode.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'A' as i32
    {
        types[1 as ::core::ffi::c_int as usize] = MM_DENSE_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'R' as i32 {
        types[2 as ::core::ffi::c_int as usize] = MM_REAL_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'C' as i32
    {
        types[2 as ::core::ffi::c_int as usize] =
            MM_COMPLEX_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'P' as i32
    {
        types[2 as ::core::ffi::c_int as usize] =
            MM_PATTERN_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else if *matcode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'I' as i32
    {
        types[2 as ::core::ffi::c_int as usize] = MM_INT_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *matcode.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'G' as i32 {
        types[3 as ::core::ffi::c_int as usize] =
            MM_GENERAL_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else if *matcode.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'S' as i32
    {
        types[3 as ::core::ffi::c_int as usize] = MM_SYMM_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else if *matcode.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'H' as i32
    {
        types[3 as ::core::ffi::c_int as usize] = MM_HERM_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else if *matcode.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'K' as i32
    {
        types[3 as ::core::ffi::c_int as usize] = MM_SKEW_STR.as_ptr() as *mut ::core::ffi::c_char;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    native_only!(snprintf,
        &raw mut buffer as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1025]>() as size_t,
        b"%s %s %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        types[0 as ::core::ffi::c_int as usize],
        types[1 as ::core::ffi::c_int as usize],
        types[2 as ::core::ffi::c_int as usize],
        types[3 as ::core::ffi::c_int as usize],
    );
    return (&raw mut buffer as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
