use crate::honest_did::lpsolve::runtime::{malloc,calloc,free,realloc,sqrt,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_init_BLAS"]
    fn init_BLAS();
    #[link_name="honest_lpsolve_is_nativeBLAS"]
    fn is_nativeBLAS() -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_unload_BLAS"]
    fn unload_BLAS() -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_lps_dscal"]
    fn lps_dscal(
        n: ::core::ffi::c_int,
        da: ::core::ffi::c_double,
        dx: *mut ::core::ffi::c_double,
        incx: ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_lps_daxpy"]
    fn lps_daxpy(
        n: ::core::ffi::c_int,
        da: ::core::ffi::c_double,
        dx: *mut ::core::ffi::c_double,
        incx: ::core::ffi::c_int,
        dy: *mut ::core::ffi::c_double,
        incy: ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_lps_idamax"]
    fn lps_idamax(
        n: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_double,
        is: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn log10(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    #[link_name="honest_lpsolve_blockWriteINT"]
    fn blockWriteINT(
        output: *mut FILE,
        label: *mut ::core::ffi::c_char,
        myvector: *mut ::core::ffi::c_int,
        first: ::core::ffi::c_int,
        last: ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_blockWriteREAL"]
    fn blockWriteREAL(
        output: *mut FILE,
        label: *mut ::core::ffi::c_char,
        myvector: *mut ::core::ffi::c_double,
        first: ::core::ffi::c_int,
        last: ::core::ffi::c_int,
    );
}
pub type __int64_t = i64;
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
pub type LUSOLlogfunc = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_char,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LUSOLmat {
    pub a: *mut ::core::ffi::c_double,
    pub lenx: *mut ::core::ffi::c_int,
    pub indr: *mut ::core::ffi::c_int,
    pub indc: *mut ::core::ffi::c_int,
    pub indx: *mut ::core::ffi::c_int,
}
pub type LUSOLmat = _LUSOLmat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LUSOLrec {
    pub outstream: *mut FILE,
    pub writelog: Option<LUSOLlogfunc>,
    pub loghandle: *mut ::core::ffi::c_void,
    pub debuginfo: Option<LUSOLlogfunc>,
    pub luparm: [::core::ffi::c_int; 33],
    pub parmlu: [::core::ffi::c_double; 21],
    pub lena: ::core::ffi::c_int,
    pub nelem: ::core::ffi::c_int,
    pub indc: *mut ::core::ffi::c_int,
    pub indr: *mut ::core::ffi::c_int,
    pub a: *mut ::core::ffi::c_double,
    pub maxm: ::core::ffi::c_int,
    pub m: ::core::ffi::c_int,
    pub lenr: *mut ::core::ffi::c_int,
    pub ip: *mut ::core::ffi::c_int,
    pub iqloc: *mut ::core::ffi::c_int,
    pub ipinv: *mut ::core::ffi::c_int,
    pub locr: *mut ::core::ffi::c_int,
    pub maxn: ::core::ffi::c_int,
    pub n: ::core::ffi::c_int,
    pub lenc: *mut ::core::ffi::c_int,
    pub iq: *mut ::core::ffi::c_int,
    pub iploc: *mut ::core::ffi::c_int,
    pub iqinv: *mut ::core::ffi::c_int,
    pub locc: *mut ::core::ffi::c_int,
    pub w: *mut ::core::ffi::c_double,
    pub vLU6L: *mut ::core::ffi::c_double,
    pub isingular: *mut ::core::ffi::c_int,
    pub Ha: *mut ::core::ffi::c_double,
    pub diagU: *mut ::core::ffi::c_double,
    pub Hj: *mut ::core::ffi::c_int,
    pub Hk: *mut ::core::ffi::c_int,
    pub amaxr: *mut ::core::ffi::c_double,
    pub L0: *mut LUSOLmat,
    pub U: *mut LUSOLmat,
    pub expanded_a: ::core::ffi::c_int,
    pub replaced_c: ::core::ffi::c_int,
    pub replaced_r: ::core::ffi::c_int,
}
pub type LUSOLrec = _LUSOLrec;
#[export_name="honest_lpsolve_clean_realloc"]
pub unsafe extern "C" fn clean_realloc(
    mut oldptr: *mut ::core::ffi::c_void,
    mut width: ::core::ffi::c_int,
    mut newsize: ::core::ffi::c_int,
    mut oldsize: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    newsize *= width;
    oldsize *= width;
    if newsize <= 0 as ::core::ffi::c_int {
        free(oldptr);
        oldptr = NULL;
        return oldptr;
    }
    oldptr = realloc(oldptr, newsize as size_t);
    if newsize > oldsize {
        memset(
            (oldptr as *mut ::core::ffi::c_char).offset(oldsize as isize)
                as *mut ::core::ffi::c_void,
            '\0' as i32,
            (newsize - oldsize) as size_t,
        );
    }
    return oldptr;
}
#[export_name="honest_lpsolve_LUSOL_realloc_a"]
pub unsafe extern "C" fn LUSOL_realloc_a(
    mut LUSOL: *mut LUSOLrec,
    mut newsize: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut oldsize: ::core::ffi::c_int = 0;
    if newsize < 0 as ::core::ffi::c_int {
        newsize = (*LUSOL).lena
            + (if abs(newsize) > 10000 as ::core::ffi::c_int {
                abs(newsize)
            } else {
                10000 as ::core::ffi::c_int
            });
    }
    oldsize = (*LUSOL).lena;
    (*LUSOL).lena = newsize;
    if newsize > 0 as ::core::ffi::c_int {
        newsize += 1;
    }
    if oldsize > 0 as ::core::ffi::c_int {
        oldsize += 1;
    }
    (*LUSOL).a = clean_realloc(
        (*LUSOL).a as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_double;
    (*LUSOL).indc = clean_realloc(
        (*LUSOL).indc as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).indr = clean_realloc(
        (*LUSOL).indr as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    if newsize == 0 as ::core::ffi::c_int
        || !(*LUSOL).a.is_null() && !(*LUSOL).indc.is_null() && !(*LUSOL).indr.is_null()
    {
        return 1 as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_LUSOL_expand_a"]
pub unsafe extern "C" fn LUSOL_expand_a(
    mut LUSOL: *mut LUSOLrec,
    mut delta_lena: *mut ::core::ffi::c_int,
    mut right_shift: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut LENA: ::core::ffi::c_int = 0;
    let mut NFREE: ::core::ffi::c_int = 0;
    let mut LFREE: ::core::ffi::c_int = 0;
    LENA = (*LUSOL).lena;
    *delta_lena = (*delta_lena as ::core::ffi::c_double
        * (if 1.33f64
            < pow(
                1.5f64,
                fabs(*delta_lena as ::core::ffi::c_double)
                    / (LENA + *delta_lena + 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            )
        {
            1.33f64
        } else {
            pow(
                1.5f64,
                fabs(*delta_lena as ::core::ffi::c_double)
                    / (LENA + *delta_lena + 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            )
        })) as ::core::ffi::c_int;
    if *delta_lena <= 0 as ::core::ffi::c_int || LUSOL_realloc_a(LUSOL, LENA + *delta_lena) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    *delta_lena = (*LUSOL).lena - LENA;
    LFREE = *right_shift;
    NFREE = LFREE + *delta_lena;
    LENA -= LFREE - 1 as ::core::ffi::c_int;
    memmove(
        (*LUSOL).a.offset(NFREE as isize) as *mut ::core::ffi::c_void,
        (*LUSOL).a.offset(LFREE as isize) as *const ::core::ffi::c_void,
        (LENA as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    memmove(
        (*LUSOL).indr.offset(NFREE as isize) as *mut ::core::ffi::c_void,
        (*LUSOL).indr.offset(LFREE as isize) as *const ::core::ffi::c_void,
        (LENA as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    memmove(
        (*LUSOL).indc.offset(NFREE as isize) as *mut ::core::ffi::c_void,
        (*LUSOL).indc.offset(LFREE as isize) as *const ::core::ffi::c_void,
        (LENA as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    *right_shift = NFREE;
    (*LUSOL).expanded_a += 1;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_LUSOL_realloc_r"]
pub unsafe extern "C" fn LUSOL_realloc_r(
    mut LUSOL: *mut LUSOLrec,
    mut newsize: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut oldsize: ::core::ffi::c_int = 0;
    if newsize < 0 as ::core::ffi::c_int {
        newsize = (*LUSOL).maxm
            + (if abs(newsize) > 1000 as ::core::ffi::c_int {
                abs(newsize)
            } else {
                1000 as ::core::ffi::c_int
            });
    }
    oldsize = (*LUSOL).maxm;
    (*LUSOL).maxm = newsize;
    if newsize > 0 as ::core::ffi::c_int {
        newsize += 1;
    }
    if oldsize > 0 as ::core::ffi::c_int {
        oldsize += 1;
    }
    (*LUSOL).lenr = clean_realloc(
        (*LUSOL).lenr as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).ip = clean_realloc(
        (*LUSOL).ip as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).iqloc = clean_realloc(
        (*LUSOL).iqloc as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).ipinv = clean_realloc(
        (*LUSOL).ipinv as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).locr = clean_realloc(
        (*LUSOL).locr as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    if newsize == 0 as ::core::ffi::c_int
        || !(*LUSOL).lenr.is_null()
            && !(*LUSOL).ip.is_null()
            && !(*LUSOL).iqloc.is_null()
            && !(*LUSOL).ipinv.is_null()
            && !(*LUSOL).locr.is_null()
    {
        (*LUSOL).amaxr = clean_realloc(
            (*LUSOL).amaxr as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
            newsize,
            oldsize,
        ) as *mut ::core::ffi::c_double;
        if newsize > 0 as ::core::ffi::c_int && (*LUSOL).amaxr.is_null() {
            return 0 as ::core::ffi::c_uchar;
        }
        return 1 as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_LUSOL_realloc_c"]
pub unsafe extern "C" fn LUSOL_realloc_c(
    mut LUSOL: *mut LUSOLrec,
    mut newsize: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut oldsize: ::core::ffi::c_int = 0;
    if newsize < 0 as ::core::ffi::c_int {
        newsize = (*LUSOL).maxn
            + (if abs(newsize) > 1000 as ::core::ffi::c_int {
                abs(newsize)
            } else {
                1000 as ::core::ffi::c_int
            });
    }
    oldsize = (*LUSOL).maxn;
    (*LUSOL).maxn = newsize;
    if newsize > 0 as ::core::ffi::c_int {
        newsize += 1;
    }
    if oldsize > 0 as ::core::ffi::c_int {
        oldsize += 1;
    }
    (*LUSOL).lenc = clean_realloc(
        (*LUSOL).lenc as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).iq = clean_realloc(
        (*LUSOL).iq as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).iploc = clean_realloc(
        (*LUSOL).iploc as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).iqinv = clean_realloc(
        (*LUSOL).iqinv as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).locc = clean_realloc(
        (*LUSOL).locc as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_int;
    (*LUSOL).w = clean_realloc(
        (*LUSOL).w as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_double;
    (*LUSOL).vLU6L = clean_realloc(
        (*LUSOL).vLU6L as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
        newsize,
        oldsize,
    ) as *mut ::core::ffi::c_double;
    if newsize == 0 as ::core::ffi::c_int
        || !(*LUSOL).w.is_null()
            && !(*LUSOL).lenc.is_null()
            && !(*LUSOL).iq.is_null()
            && !(*LUSOL).iploc.is_null()
            && !(*LUSOL).iqinv.is_null()
            && !(*LUSOL).locc.is_null()
    {
        if (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] == LUSOL_PIVMOD_TCP {
            (*LUSOL).Ha = clean_realloc(
                (*LUSOL).Ha as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
                newsize,
                oldsize,
            ) as *mut ::core::ffi::c_double;
            (*LUSOL).Hj = clean_realloc(
                (*LUSOL).Hj as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
                newsize,
                oldsize,
            ) as *mut ::core::ffi::c_int;
            (*LUSOL).Hk = clean_realloc(
                (*LUSOL).Hk as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
                newsize,
                oldsize,
            ) as *mut ::core::ffi::c_int;
            if newsize > 0 as ::core::ffi::c_int
                && ((*LUSOL).Ha.is_null() || (*LUSOL).Hj.is_null() || (*LUSOL).Hk.is_null())
            {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        if (*LUSOL).luparm[LUSOL_IP_KEEPLU as usize] == FALSE {
            (*LUSOL).diagU = clean_realloc(
                (*LUSOL).diagU as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
                newsize,
                oldsize,
            ) as *mut ::core::ffi::c_double;
            if newsize > 0 as ::core::ffi::c_int && (*LUSOL).diagU.is_null() {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        return 1 as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_LUSOL_create"]
pub unsafe extern "C" fn LUSOL_create(
    mut outstream: *mut FILE,
    mut msgfil: ::core::ffi::c_int,
    mut pivotmodel: ::core::ffi::c_int,
    mut updatelimit: ::core::ffi::c_int,
) -> *mut LUSOLrec {
    let mut newLU: *mut LUSOLrec = ::core::ptr::null_mut::<LUSOLrec>();
    newLU = calloc(1 as size_t, ::core::mem::size_of::<LUSOLrec>() as size_t) as *mut LUSOLrec;
    if newLU.is_null() {
        return newLU;
    }
    (*newLU).luparm[LUSOL_IP_SCALAR_NZA as usize] = LUSOL_MULT_nz_a;
    (*newLU).outstream = outstream;
    (*newLU).luparm[LUSOL_IP_PRINTUNIT as usize] = msgfil;
    (*newLU).luparm[LUSOL_IP_PRINTLEVEL as usize] = LUSOL_MSG_SINGULARITY;
    LUSOL_setpivotmodel(newLU, pivotmodel, LUSOL_PIVTOL_DEFAULT);
    (*newLU).parmlu[LUSOL_RP_GAMMA as usize] = LUSOL_DEFAULT_GAMMA;
    (*newLU).parmlu[LUSOL_RP_ZEROTOLERANCE as usize] = 3.0e-13f64;
    (*newLU).parmlu[LUSOL_RP_EPSDIAG_U as usize] = 3.7e-11f64;
    (*newLU).parmlu[LUSOL_RP_SMALLDIAG_U as usize] = (*newLU).parmlu[LUSOL_RP_EPSDIAG_U as usize];
    (*newLU).parmlu[LUSOL_RP_COMPSPACE_U as usize] = 3.0e+0f64;
    (*newLU).luparm[LUSOL_IP_MARKOWITZ_MAXCOL as usize] = 5 as ::core::ffi::c_int;
    (*newLU).parmlu[LUSOL_RP_MARKOWITZ_CONLY as usize] = 0.3e+0f64;
    (*newLU).parmlu[LUSOL_RP_MARKOWITZ_DENSE as usize] = 0.5e+0f64;
    (*newLU).parmlu[LUSOL_RP_SMARTRATIO as usize] = LUSOL_DEFAULT_SMARTRATIO;
    (*newLU).luparm[LUSOL_IP_KEEPLU as usize] = TRUE;
    (*newLU).luparm[LUSOL_IP_UPDATELIMIT as usize] = updatelimit;
    init_BLAS();
    return newLU;
}
#[export_name="honest_lpsolve_LUSOL_sizeto"]
pub unsafe extern "C" fn LUSOL_sizeto(
    mut LUSOL: *mut LUSOLrec,
    mut init_r: ::core::ffi::c_int,
    mut init_c: ::core::ffi::c_int,
    mut init_a: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if init_c == 0 as ::core::ffi::c_int {
        free((*LUSOL).isingular as *mut ::core::ffi::c_void);
        (*LUSOL).isingular = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if LUSOL_realloc_a(LUSOL, init_a) as ::core::ffi::c_int != 0
        && LUSOL_realloc_r(LUSOL, init_r) as ::core::ffi::c_int != 0
        && LUSOL_realloc_c(LUSOL, init_c) as ::core::ffi::c_int != 0
    {
        return 1 as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_LUSOL_pivotLabel"]
pub unsafe extern "C" fn LUSOL_pivotLabel(mut LUSOL: *mut LUSOLrec) -> *mut ::core::ffi::c_char {
    static mut pivotText: [*mut ::core::ffi::c_char; 4] = [
        b"TPP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"TRP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"TCP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"TSP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    return pivotText[(*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] as usize];
}
#[export_name="honest_lpsolve_LUSOL_setpivotmodel"]
pub unsafe extern "C" fn LUSOL_setpivotmodel(
    mut LUSOL: *mut LUSOLrec,
    mut pivotmodel: ::core::ffi::c_int,
    mut initlevel: ::core::ffi::c_int,
) {
    let mut newFM: ::core::ffi::c_double = 0.;
    let mut newUM: ::core::ffi::c_double = 0.;
    if pivotmodel > LUSOL_PIVMOD_NOCHANGE {
        if pivotmodel <= LUSOL_PIVMOD_DEFAULT || pivotmodel > LUSOL_PIVMOD_MAX {
            pivotmodel = LUSOL_PIVMOD_TPP;
        }
        (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] = pivotmodel;
    }
    if initlevel <= LUSOL_PIVTOL_NOCHANGE || initlevel > LUSOL_PIVTOL_MAX {
        return;
    }
    if initlevel == LUSOL_PIVTOL_BAGGY {
        newFM = 500.0f64;
        newUM = newFM / 20 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if initlevel == LUSOL_PIVTOL_LOOSE {
        newFM = 100.0f64;
        newUM = newFM / 10 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if initlevel == LUSOL_PIVTOL_NORMAL {
        newFM = 28.0f64;
        newUM = newFM / 4 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if initlevel == LUSOL_PIVTOL_SLIM {
        newFM = 10.0f64;
        newUM = newFM / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if initlevel == LUSOL_PIVTOL_TIGHT {
        newFM = 5.0f64;
        newUM = newFM / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if initlevel == LUSOL_PIVTOL_SUPER {
        newFM = 2.5f64;
        newUM = 1.99f64;
    } else {
        newFM = 1.99f64;
        newUM = newFM / 1.49f64;
    }
    (*LUSOL).parmlu[LUSOL_RP_FACTORMAX_Lij as usize] = newFM;
    (*LUSOL).parmlu[LUSOL_RP_UPDATEMAX_Lij as usize] = newUM;
}
#[export_name="honest_lpsolve_LUSOL_tightenpivot"]
pub unsafe extern "C" fn LUSOL_tightenpivot(mut LUSOL: *mut LUSOLrec) -> ::core::ffi::c_uchar {
    if (if (*LUSOL).parmlu[1 as ::core::ffi::c_int as usize]
        < (*LUSOL).parmlu[2 as ::core::ffi::c_int as usize]
    {
        (*LUSOL).parmlu[1 as ::core::ffi::c_int as usize]
    } else {
        (*LUSOL).parmlu[2 as ::core::ffi::c_int as usize]
    }) < 1.1f64
    {
        if (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] >= LUSOL_PIVMOD_TRP {
            return 0 as ::core::ffi::c_uchar;
        }
        LUSOL_setpivotmodel(
            LUSOL,
            (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] + 1 as ::core::ffi::c_int,
            LUSOL_PIVTOL_DEFAULT + 1 as ::core::ffi::c_int,
        );
        return 2 as ::core::ffi::c_uchar;
    }
    (*LUSOL).parmlu[LUSOL_RP_FACTORMAX_Lij as usize] =
        1.0f64 + (*LUSOL).parmlu[LUSOL_RP_FACTORMAX_Lij as usize] / 3.0f64;
    (*LUSOL).parmlu[LUSOL_RP_UPDATEMAX_Lij as usize] =
        1.0f64 + (*LUSOL).parmlu[LUSOL_RP_UPDATEMAX_Lij as usize] / 3.0f64;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_LUSOL_addSingularity"]
pub unsafe extern "C" fn LUSOL_addSingularity(
    mut LUSOL: *mut LUSOLrec,
    mut singcol: ::core::ffi::c_int,
    mut inform: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut NSING: ::core::ffi::c_int = (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize];
    let mut ASING: ::core::ffi::c_int = (*LUSOL).luparm[LUSOL_IP_SINGULARLISTSIZE as usize];
    if NSING > 0 as ::core::ffi::c_int && NSING >= ASING {
        ASING +=
            (10.0f64 * (log10((*LUSOL).m as ::core::ffi::c_double) + 1.0f64)) as ::core::ffi::c_int;
        (*LUSOL).isingular = realloc(
            (*LUSOL).isingular as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                .wrapping_mul((ASING + 1 as ::core::ffi::c_int) as size_t),
        ) as *mut ::core::ffi::c_int;
        if (*LUSOL).isingular.is_null() {
            (*LUSOL).luparm[LUSOL_IP_SINGULARLISTSIZE as usize] = 0 as ::core::ffi::c_int;
            *inform = LUSOL_INFORM_NOMEMLEFT;
            return 0 as ::core::ffi::c_uchar;
        }
        (*LUSOL).luparm[LUSOL_IP_SINGULARLISTSIZE as usize] = ASING;
        if NSING == 1 as ::core::ffi::c_int {
            *(*LUSOL).isingular.offset(NSING as isize) =
                (*LUSOL).luparm[LUSOL_IP_SINGULARINDEX as usize];
        }
    }
    NSING += 1;
    if NSING > 1 as ::core::ffi::c_int {
        *(*LUSOL).isingular.offset(0 as ::core::ffi::c_int as isize) = NSING;
        *(*LUSOL).isingular.offset(NSING as isize) = singcol;
    }
    (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize] = NSING;
    (*LUSOL).luparm[LUSOL_IP_SINGULARINDEX as usize] = singcol;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_LUSOL_getSingularity"]
pub unsafe extern "C" fn LUSOL_getSingularity(
    mut LUSOL: *mut LUSOLrec,
    mut singitem: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if singitem > (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize]
        || singitem < 0 as ::core::ffi::c_int
    {
        singitem = -(1 as ::core::ffi::c_int);
    } else if singitem == 0 as ::core::ffi::c_int {
        singitem = (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize];
    } else if singitem > 1 as ::core::ffi::c_int {
        singitem = *(*LUSOL).isingular.offset(singitem as isize);
    } else {
        singitem = (*LUSOL).luparm[LUSOL_IP_SINGULARINDEX as usize];
    }
    return singitem;
}
#[export_name="honest_lpsolve_LUSOL_findSingularityPosition"]
pub unsafe extern "C" fn LUSOL_findSingularityPosition(
    mut LUSOL: *mut LUSOLrec,
    mut singcol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    singcol = *(*LUSOL).iqinv.offset(singcol as isize);
    return *(*LUSOL).ip.offset(singcol as isize);
}
#[export_name="honest_lpsolve_LUSOL_informstr"]
pub unsafe extern "C" fn LUSOL_informstr(
    mut LUSOL: *mut LUSOLrec,
    mut inform: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    static mut informText: [*mut ::core::ffi::c_char; 12] = [
        b"LUSOL_RANKLOSS: Lost rank\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_LUSUCCESS: Success\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_LUSINGULAR: Singular A\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_LUUNSTABLE: Unstable factorization\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_ADIMERR: Row or column count exceeded\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_ADUPLICATE: Duplicate A matrix entry found\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"(Undefined message)\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"(Undefined message)\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_ANEEDMEM: Insufficient memory for factorization\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"LUSOL_FATALERR: Fatal internal error\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_NOPIVOT: Found no suitable pivot\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"LUSOL_NOMEMLEFT: Could not obtain more memory\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    if inform < LUSOL_INFORM_MIN || inform > LUSOL_INFORM_MAX {
        inform = (*LUSOL).luparm[LUSOL_IP_INFORM as usize];
    }
    return informText[(inform - LUSOL_INFORM_MIN) as usize];
}
#[export_name="honest_lpsolve_LUSOL_clear"]
pub unsafe extern "C" fn LUSOL_clear(mut LUSOL: *mut LUSOLrec, mut nzonly: ::core::ffi::c_uchar) {
    let mut len: ::core::ffi::c_int = 0;
    (*LUSOL).nelem = 0 as ::core::ffi::c_int;
    if nzonly == 0 {
        len = (*LUSOL).lena + LUSOL_ARRAYOFFSET;
        memset(
            (*LUSOL).a as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        memset(
            (*LUSOL).indc as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).indr as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        len = (*LUSOL).maxm + LUSOL_ARRAYOFFSET;
        memset(
            (*LUSOL).lenr as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).ip as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).iqloc as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).ipinv as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).locr as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        if !(*LUSOL).amaxr.is_null() {
            memset(
                (*LUSOL).amaxr as *mut ::core::ffi::c_void,
                '\0' as i32,
                (len as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
        len = (*LUSOL).maxn + LUSOL_ARRAYOFFSET;
        memset(
            (*LUSOL).lenc as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).iq as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).iploc as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).iqinv as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).locc as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*LUSOL).w as *mut ::core::ffi::c_void,
            '\0' as i32,
            (len as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        if (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] == LUSOL_PIVMOD_TCP {
            memset(
                (*LUSOL).Ha as *mut ::core::ffi::c_void,
                '\0' as i32,
                (len as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            memset(
                (*LUSOL).Hj as *mut ::core::ffi::c_void,
                '\0' as i32,
                (len as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            memset(
                (*LUSOL).Hk as *mut ::core::ffi::c_void,
                '\0' as i32,
                (len as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
        }
        if (*LUSOL).luparm[LUSOL_IP_KEEPLU as usize] == FALSE {
            memset(
                (*LUSOL).diagU as *mut ::core::ffi::c_void,
                '\0' as i32,
                (len as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
    }
}
#[export_name="honest_lpsolve_LUSOL_assign"]
pub unsafe extern "C" fn LUSOL_assign(
    mut LUSOL: *mut LUSOLrec,
    mut iA: *mut ::core::ffi::c_int,
    mut jA: *mut ::core::ffi::c_int,
    mut Aij: *mut ::core::ffi::c_double,
    mut nzcount: ::core::ffi::c_int,
    mut istriplet: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ij: ::core::ffi::c_int = 0;
    let mut kol: ::core::ffi::c_int = 0;
    if nzcount > (*LUSOL).lena / (*LUSOL).luparm[LUSOL_IP_SCALAR_NZA as usize]
        && LUSOL_realloc_a(
            LUSOL,
            nzcount * (*LUSOL).luparm[LUSOL_IP_SCALAR_NZA as usize],
        ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    m = 0 as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    kol = 1 as ::core::ffi::c_int;
    k = 1 as ::core::ffi::c_int;
    while k <= nzcount {
        ij = *iA.offset(k as isize);
        if ij > m {
            m = ij;
            if m > (*LUSOL).maxm
                && LUSOL_realloc_r(
                    LUSOL,
                    -(m / LUSOL_MINDELTA_FACTOR + 1 as ::core::ffi::c_int),
                ) == 0
            {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        *(*LUSOL).indc.offset(k as isize) = ij;
        if istriplet != 0 {
            ij = *jA.offset(k as isize);
        } else {
            if k >= *jA.offset(kol as isize) {
                kol += 1;
            }
            ij = kol;
        }
        if ij > n {
            n = ij;
            if n > (*LUSOL).maxn
                && LUSOL_realloc_c(
                    LUSOL,
                    -(n / LUSOL_MINDELTA_FACTOR + 1 as ::core::ffi::c_int),
                ) == 0
            {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        *(*LUSOL).indr.offset(k as isize) = ij;
        *(*LUSOL).a.offset(k as isize) = *Aij.offset(k as isize);
        k += 1;
    }
    (*LUSOL).m = m;
    (*LUSOL).n = n;
    (*LUSOL).nelem = nzcount;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_LUSOL_loadColumn"]
pub unsafe extern "C" fn LUSOL_loadColumn(
    mut LUSOL: *mut LUSOLrec,
    mut iA: *mut ::core::ffi::c_int,
    mut jA: ::core::ffi::c_int,
    mut Aij: *mut ::core::ffi::c_double,
    mut nzcount: ::core::ffi::c_int,
    mut offset1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    nz = (*LUSOL).nelem;
    i = nz + nzcount;
    if i > (*LUSOL).lena / (*LUSOL).luparm[LUSOL_IP_SCALAR_NZA as usize]
        && LUSOL_realloc_a(LUSOL, i * (*LUSOL).luparm[LUSOL_IP_SCALAR_NZA as usize]) == 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    k = 0 as ::core::ffi::c_int;
    ii = 1 as ::core::ffi::c_int;
    while ii <= nzcount {
        i = ii + offset1;
        if !(*Aij.offset(i as isize) == 0 as ::core::ffi::c_int as ::core::ffi::c_double) {
            if *iA.offset(i as isize) <= 0 as ::core::ffi::c_int
                || *iA.offset(i as isize) > (*LUSOL).m
                || jA <= 0 as ::core::ffi::c_int
                || jA > (*LUSOL).n
            {
                LUSOL_report(
                    LUSOL,
                    0 as ::core::ffi::c_int,
                    b"Variable index outside of set bounds (r:%d/%d, c:%d/%d)\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                k += 1;
                nz += 1;
                *(*LUSOL).a.offset(nz as isize) = *Aij.offset(i as isize);
                *(*LUSOL).indc.offset(nz as isize) = *iA.offset(i as isize);
                *(*LUSOL).indr.offset(nz as isize) = jA;
            }
        }
        ii += 1;
    }
    (*LUSOL).nelem = nz;
    return k;
}
#[export_name="honest_lpsolve_LUSOL_free"]
pub unsafe extern "C" fn LUSOL_free(mut LUSOL: *mut LUSOLrec) {
    LUSOL_realloc_a(LUSOL, 0 as ::core::ffi::c_int);
    LUSOL_realloc_r(LUSOL, 0 as ::core::ffi::c_int);
    LUSOL_realloc_c(LUSOL, 0 as ::core::ffi::c_int);
    if !(*LUSOL).L0.is_null() {
        LUSOL_matfree(&raw mut (*LUSOL).L0);
    }
    if !(*LUSOL).U.is_null() {
        LUSOL_matfree(&raw mut (*LUSOL).U);
    }
    if is_nativeBLAS() == 0 {
        unload_BLAS();
    }
    free(LUSOL as *mut ::core::ffi::c_void);
    LUSOL = ::core::ptr::null_mut::<LUSOLrec>();
}
#[export_name="honest_lpsolve_LUSOL_report"]
pub unsafe extern "C" fn LUSOL_report(
    mut LUSOL: *mut LUSOLrec,
    mut msglevel: ::core::ffi::c_int,
    mut format: *mut ::core::ffi::c_char,
) {
}
#[export_name="honest_lpsolve_LUSOL_timer"]
pub unsafe extern "C" fn LUSOL_timer(
    mut LUSOL: *mut LUSOLrec,
    mut timerid: ::core::ffi::c_int,
    mut text: *mut ::core::ffi::c_char,
) {
    LUSOL_report(
        LUSOL,
        -(1 as ::core::ffi::c_int),
        b"TimerID %d at %s - %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
}
#[export_name="honest_lpsolve_LUSOL_factorize"]
pub unsafe extern "C" fn LUSOL_factorize(mut LUSOL: *mut LUSOLrec) -> ::core::ffi::c_int {
    let mut inform: ::core::ffi::c_int = 0;
    LU1FAC(LUSOL, &raw mut inform);
    return inform;
}
#[export_name="honest_lpsolve_LUSOL_ftran"]
pub unsafe extern "C" fn LUSOL_ftran(
    mut LUSOL: *mut LUSOLrec,
    mut b: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
    mut prepareupdate: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut inform: ::core::ffi::c_int = 0;
    let mut vector: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if prepareupdate != 0 {
        vector = (*LUSOL).vLU6L;
    } else {
        vector = (*LUSOL).w;
    }
    memcpy(
        vector.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        b.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ((*LUSOL).n as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    if !vector.is_null() {
        *vector.offset(0 as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    LU6SOL(
        LUSOL,
        LUSOL_SOLVE_Aw_v,
        vector as *mut ::core::ffi::c_double,
        b,
        NZidx,
        &raw mut inform,
    );
    (*LUSOL).luparm[LUSOL_IP_FTRANCOUNT as usize] += 1;
    return inform;
}
#[export_name="honest_lpsolve_LUSOL_btran"]
pub unsafe extern "C" fn LUSOL_btran(
    mut LUSOL: *mut LUSOLrec,
    mut b: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut inform: ::core::ffi::c_int = 0;
    memcpy(
        (*LUSOL).w.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        b.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ((*LUSOL).m as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    if !(*LUSOL).w.is_null() {
        *(*LUSOL).w.offset(0 as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    LU6SOL(
        LUSOL,
        LUSOL_SOLVE_Atv_w,
        b,
        (*LUSOL).w as *mut ::core::ffi::c_double,
        NZidx,
        &raw mut inform,
    );
    (*LUSOL).luparm[LUSOL_IP_BTRANCOUNT as usize] += 1;
    return inform;
}
#[export_name="honest_lpsolve_LUSOL_replaceColumn"]
pub unsafe extern "C" fn LUSOL_replaceColumn(
    mut LUSOL: *mut LUSOLrec,
    mut jcol: ::core::ffi::c_int,
    mut v: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut inform: ::core::ffi::c_int = 0;
    let mut DIAG: ::core::ffi::c_double = 0.;
    let mut VNORM: ::core::ffi::c_double = 0.;
    LU8RPC(
        LUSOL,
        LUSOL_UPDATE_OLDNONEMPTY,
        LUSOL_UPDATE_NEWNONEMPTY,
        jcol,
        v,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
        &raw mut inform,
        &raw mut DIAG,
        &raw mut VNORM,
    );
    (*LUSOL).replaced_c += 1;
    return inform;
}
#[export_name="honest_lpsolve_LUSOL_vecdensity"]
pub unsafe extern "C" fn LUSOL_vecdensity(
    mut LUSOL: *mut LUSOLrec,
    mut V: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut I: ::core::ffi::c_int = 0;
    let mut N: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    I = 1 as ::core::ffi::c_int;
    while I <= (*LUSOL).m {
        if fabs(*V.offset(I as isize)) > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            N += 1;
        }
        I += 1;
    }
    return N as ::core::ffi::c_double / (*LUSOL).m as ::core::ffi::c_double;
}
#[export_name="honest_lpsolve_relationChar"]
pub unsafe extern "C" fn relationChar(
    mut left: ::core::ffi::c_double,
    mut right: ::core::ffi::c_double,
) -> ::core::ffi::c_char {
    if left > right {
        return '>' as i32 as ::core::ffi::c_char;
    } else if left == right {
        return '=' as i32 as ::core::ffi::c_char;
    } else {
        return '<' as i32 as ::core::ffi::c_char;
    };
}
#[export_name="honest_lpsolve_HDOWN"]
pub unsafe extern "C" fn HDOWN(
    mut HA: *mut ::core::ffi::c_double,
    mut HJ: *mut ::core::ffi::c_int,
    mut HK: *mut ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut K: ::core::ffi::c_int,
    mut HOPS: *mut ::core::ffi::c_int,
) {
    let mut J: ::core::ffi::c_int = 0;
    let mut JJ: ::core::ffi::c_int = 0;
    let mut JV: ::core::ffi::c_int = 0;
    let mut N2: ::core::ffi::c_int = 0;
    let mut V: ::core::ffi::c_double = 0.;
    *HOPS = 0 as ::core::ffi::c_int;
    V = *HA.offset(K as isize);
    JV = *HJ.offset(K as isize);
    N2 = N / 2 as ::core::ffi::c_int;
    while !(K > N2) {
        *HOPS += 1;
        J = K + K;
        if J < N {
            if *HA.offset(J as isize) < *HA.offset((J + 1 as ::core::ffi::c_int) as isize) {
                J += 1;
            }
        }
        if V >= *HA.offset(J as isize) {
            break;
        }
        *HA.offset(K as isize) = *HA.offset(J as isize);
        JJ = *HJ.offset(J as isize);
        *HJ.offset(K as isize) = JJ;
        *HK.offset(JJ as isize) = K;
        K = J;
    }
    *HA.offset(K as isize) = V;
    *HJ.offset(K as isize) = JV;
    *HK.offset(JV as isize) = K;
}
#[export_name="honest_lpsolve_HUP"]
pub unsafe extern "C" fn HUP(
    mut HA: *mut ::core::ffi::c_double,
    mut HJ: *mut ::core::ffi::c_int,
    mut HK: *mut ::core::ffi::c_int,
    mut K: ::core::ffi::c_int,
    mut HOPS: *mut ::core::ffi::c_int,
) {
    let mut J: ::core::ffi::c_int = 0;
    let mut JV: ::core::ffi::c_int = 0;
    let mut K2: ::core::ffi::c_int = 0;
    let mut V: ::core::ffi::c_double = 0.;
    *HOPS = 0 as ::core::ffi::c_int;
    V = *HA.offset(K as isize);
    JV = *HJ.offset(K as isize);
    while !(K < 2 as ::core::ffi::c_int) {
        K2 = K / 2 as ::core::ffi::c_int;
        if V < *HA.offset(K2 as isize) {
            break;
        }
        *HOPS += 1;
        *HA.offset(K as isize) = *HA.offset(K2 as isize);
        J = *HJ.offset(K2 as isize);
        *HJ.offset(K as isize) = J;
        *HK.offset(J as isize) = K;
        K = K2;
    }
    *HA.offset(K as isize) = V;
    *HJ.offset(K as isize) = JV;
    *HK.offset(JV as isize) = K;
}
#[export_name="honest_lpsolve_HINSERT"]
pub unsafe extern "C" fn HINSERT(
    mut HA: *mut ::core::ffi::c_double,
    mut HJ: *mut ::core::ffi::c_int,
    mut HK: *mut ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut V: ::core::ffi::c_double,
    mut JV: ::core::ffi::c_int,
    mut HOPS: *mut ::core::ffi::c_int,
) {
    *HA.offset(N as isize) = V;
    *HJ.offset(N as isize) = JV;
    *HK.offset(JV as isize) = N;
    HUP(HA, HJ, HK, N, HOPS);
}
#[export_name="honest_lpsolve_HCHANGE"]
pub unsafe extern "C" fn HCHANGE(
    mut HA: *mut ::core::ffi::c_double,
    mut HJ: *mut ::core::ffi::c_int,
    mut HK: *mut ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut K: ::core::ffi::c_int,
    mut V: ::core::ffi::c_double,
    mut JV: ::core::ffi::c_int,
    mut HOPS: *mut ::core::ffi::c_int,
) {
    let mut V1: ::core::ffi::c_double = 0.;
    V1 = *HA.offset(K as isize);
    *HA.offset(K as isize) = V;
    *HJ.offset(K as isize) = JV;
    *HK.offset(JV as isize) = K;
    if V1 < V {
        HUP(HA, HJ, HK, K, HOPS);
    } else {
        HDOWN(HA, HJ, HK, N, K, HOPS);
    };
}
#[export_name="honest_lpsolve_HDELETE"]
pub unsafe extern "C" fn HDELETE(
    mut HA: *mut ::core::ffi::c_double,
    mut HJ: *mut ::core::ffi::c_int,
    mut HK: *mut ::core::ffi::c_int,
    mut N: *mut ::core::ffi::c_int,
    mut K: ::core::ffi::c_int,
    mut HOPS: *mut ::core::ffi::c_int,
) {
    let mut JV: ::core::ffi::c_int = 0;
    let mut NX: ::core::ffi::c_int = 0;
    let mut V: ::core::ffi::c_double = 0.;
    NX = *N;
    V = *HA.offset(NX as isize);
    JV = *HJ.offset(NX as isize);
    *N -= 1;
    *HOPS = 0 as ::core::ffi::c_int;
    if K < NX {
        HCHANGE(HA, HJ, HK, NX, K, V, JV, HOPS);
    }
}
#[export_name="honest_lpsolve_HBUILD"]
pub unsafe extern "C" fn HBUILD(
    mut HA: *mut ::core::ffi::c_double,
    mut HJ: *mut ::core::ffi::c_int,
    mut HK: *mut ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut HOPS: *mut ::core::ffi::c_int,
) {
    let mut H: ::core::ffi::c_int = 0;
    let mut JV: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut KK: ::core::ffi::c_int = 0;
    let mut V: ::core::ffi::c_double = 0.;
    *HOPS = 0 as ::core::ffi::c_int;
    K = 1 as ::core::ffi::c_int;
    while K <= N {
        KK = K;
        V = *HA.offset(K as isize);
        JV = *HJ.offset(K as isize);
        HINSERT(HA, HJ, HK, KK, V, JV, &raw mut H);
        *HOPS += H;
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU6CHK"]
pub unsafe extern "C" fn LU6CHK(
    mut LUSOL: *mut LUSOLrec,
    mut MODE: ::core::ffi::c_int,
    mut LENA2: ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
) {
    let mut KEEPLU: ::core::ffi::c_uchar = 0;
    let mut TRP: ::core::ffi::c_uchar = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut JUMIN: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut LENL: ::core::ffi::c_int = 0;
    let mut LDIAGU: ::core::ffi::c_int = 0;
    let mut LPRINT: ::core::ffi::c_int = 0;
    let mut NDEFIC: ::core::ffi::c_int = 0;
    let mut NRANK: ::core::ffi::c_int = 0;
    let mut AIJ: ::core::ffi::c_double = 0.;
    let mut DIAG: ::core::ffi::c_double = 0.;
    let mut DUMAX: ::core::ffi::c_double = 0.;
    let mut DUMIN: ::core::ffi::c_double = 0.;
    let mut LMAX: ::core::ffi::c_double = 0.;
    let mut UMAX: ::core::ffi::c_double = 0.;
    let mut UTOL1: ::core::ffi::c_double = 0.;
    let mut UTOL2: ::core::ffi::c_double = 0.;
    LPRINT = (*LUSOL).luparm[LUSOL_IP_PRINTLEVEL as usize];
    KEEPLU = ((*LUSOL).luparm[LUSOL_IP_KEEPLU as usize] != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    TRP = ((*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] == LUSOL_PIVMOD_TRP) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    NRANK = (*LUSOL).luparm[LUSOL_IP_RANK_U as usize];
    LENL = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize];
    UTOL1 = (*LUSOL).parmlu[LUSOL_RP_SMALLDIAG_U as usize];
    UTOL2 = (*LUSOL).parmlu[LUSOL_RP_EPSDIAG_U as usize];
    *INFORM = LUSOL_INFORM_LUSUCCESS;
    LMAX = ZERO as ::core::ffi::c_double;
    UMAX = ZERO as ::core::ffi::c_double;
    (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize] = 0 as ::core::ffi::c_int;
    (*LUSOL).luparm[LUSOL_IP_SINGULARINDEX as usize] = 0 as ::core::ffi::c_int;
    JUMIN = 0 as ::core::ffi::c_int;
    DUMAX = ZERO as ::core::ffi::c_double;
    DUMIN = LUSOL_BIGNUM;
    memset(
        (*LUSOL).w.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        '\0' as i32,
        ((*LUSOL).n as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    if KEEPLU != 0 {
        L = LENA2 + 1 as ::core::ffi::c_int - LENL;
        while L <= LENA2 {
            if LMAX < fabs(*(*LUSOL).a.offset(L as isize)) {
                LMAX = fabs(*(*LUSOL).a.offset(L as isize));
            }
            L += 1;
        }
        K = 1 as ::core::ffi::c_int;
        while K <= NRANK {
            I = *(*LUSOL).ip.offset(K as isize);
            L1 = *(*LUSOL).locr.offset(I as isize);
            L2 = L1 + *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
            L = L1;
            while L <= L2 {
                J = *(*LUSOL).indr.offset(L as isize);
                AIJ = fabs(*(*LUSOL).a.offset(L as isize));
                if *(*LUSOL).w.offset(J as isize) < AIJ {
                    *(*LUSOL).w.offset(J as isize) = AIJ;
                }
                if UMAX < AIJ {
                    UMAX = AIJ;
                }
                L += 1;
            }
            K += 1;
        }
        (*LUSOL).parmlu[LUSOL_RP_MAXMULT_L as usize] = LMAX;
        (*LUSOL).parmlu[LUSOL_RP_MAXELEM_U as usize] = UMAX;
        K = 1 as ::core::ffi::c_int;
        while K <= NRANK {
            J = *(*LUSOL).iq.offset(K as isize);
            I = *(*LUSOL).ip.offset(K as isize);
            L1 = *(*LUSOL).locr.offset(I as isize);
            DIAG = fabs(*(*LUSOL).a.offset(L1 as isize));
            if DUMAX < DIAG {
                DUMAX = DIAG;
            }
            if DUMIN > DIAG {
                DUMIN = DIAG;
                JUMIN = J;
            }
            K += 1;
        }
    } else {
        LDIAGU = LENA2 - (*LUSOL).n;
        K = 1 as ::core::ffi::c_int;
        while K <= NRANK {
            J = *(*LUSOL).iq.offset(K as isize);
            DIAG = fabs(*(*LUSOL).a.offset((LDIAGU + J) as isize));
            *(*LUSOL).w.offset(J as isize) = DIAG;
            if DUMAX < DIAG {
                DUMAX = DIAG;
            }
            if DUMIN > DIAG {
                DUMIN = DIAG;
                JUMIN = J;
            }
            K += 1;
        }
    }
    if MODE == 1 as ::core::ffi::c_int && TRP as ::core::ffi::c_int != 0 {
        if UTOL1 < UTOL2 * DUMAX {
            UTOL1 = UTOL2 * DUMAX;
        }
    }
    if KEEPLU != 0 {
        K = 1 as ::core::ffi::c_int;
        while K <= (*LUSOL).n {
            J = *(*LUSOL).iq.offset(K as isize);
            if K > NRANK {
                DIAG = ZERO as ::core::ffi::c_double;
            } else {
                I = *(*LUSOL).ip.offset(K as isize);
                L1 = *(*LUSOL).locr.offset(I as isize);
                DIAG = fabs(*(*LUSOL).a.offset(L1 as isize));
            }
            if DIAG <= UTOL1 || DIAG <= UTOL2 * *(*LUSOL).w.offset(J as isize) {
                LUSOL_addSingularity(LUSOL, J, INFORM);
                *(*LUSOL).w.offset(J as isize) = -*(*LUSOL).w.offset(J as isize);
            }
            K += 1;
        }
    } else {
        K = 1 as ::core::ffi::c_int;
        while K <= (*LUSOL).n {
            J = *(*LUSOL).iq.offset(K as isize);
            DIAG = *(*LUSOL).w.offset(J as isize);
            if DIAG <= UTOL1 {
                LUSOL_addSingularity(LUSOL, J, INFORM);
                *(*LUSOL).w.offset(J as isize) = -*(*LUSOL).w.offset(J as isize);
            }
            K += 1;
        }
    }
    if JUMIN == 0 as ::core::ffi::c_int {
        DUMIN = ZERO as ::core::ffi::c_double;
    }
    (*LUSOL).luparm[LUSOL_IP_COLINDEX_DUMIN as usize] = JUMIN;
    (*LUSOL).parmlu[LUSOL_RP_MAXELEM_DIAGU as usize] = DUMAX;
    (*LUSOL).parmlu[LUSOL_RP_MINELEM_DIAGU as usize] = DUMIN;
    if (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize] > 0 as ::core::ffi::c_int {
        *INFORM = LUSOL_INFORM_LUSINGULAR;
        NDEFIC = (*LUSOL).n - NRANK;
        if !(*LUSOL).outstream.is_null() && LPRINT >= LUSOL_MSG_SINGULARITY {
            LUSOL_report(
                LUSOL,
                0 as ::core::ffi::c_int,
                b"Singular(m%cn)  rank:%9d  n-rank:%8d  nsing:%9d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
}
#[export_name="honest_lpsolve_LU1L0"]
pub unsafe extern "C" fn LU1L0(
    mut LUSOL: *mut LUSOLrec,
    mut mat: *mut *mut LUSOLmat,
    mut inform: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LL: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut LENL0: ::core::ffi::c_int = 0;
    let mut NUML0: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut lsumr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    *inform = LUSOL_INFORM_LUSUCCESS;
    if mat.is_null() {
        return status;
    }
    if !(*mat).is_null() {
        LUSOL_matfree(mat);
    }
    NUML0 = (*LUSOL).luparm[LUSOL_IP_COLCOUNT_L0 as usize];
    LENL0 = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L0 as usize];
    if NUML0 == 0 as ::core::ffi::c_int
        || LENL0 == 0 as ::core::ffi::c_int
        || (*LUSOL).luparm[LUSOL_IP_ACCELERATION as usize] == LUSOL_BASEORDER
        || (*LUSOL).luparm[LUSOL_IP_ACCELERATION as usize] & LUSOL_ACCELERATE_L0
            == 0 as ::core::ffi::c_int
    {
        return status;
    }
    lsumr = calloc(
        ((*LUSOL).m + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if lsumr.is_null() {
        *inform = LUSOL_INFORM_NOMEMLEFT;
        return status;
    }
    K = 0 as ::core::ffi::c_int;
    L2 = (*LUSOL).lena;
    L1 = L2 - LENL0 + 1 as ::core::ffi::c_int;
    L = L1;
    while L <= L2 {
        I = *(*LUSOL).indc.offset(L as isize);
        let ref mut fresh8 = *lsumr.offset(I as isize);
        *fresh8 += 1;
        if *lsumr.offset(I as isize) == 1 as ::core::ffi::c_int {
            K += 1;
        }
        L += 1;
    }
    (*LUSOL).luparm[LUSOL_IP_ROWCOUNT_L0 as usize] = K;
    if !((*LUSOL).luparm[LUSOL_IP_ACCELERATION as usize] & LUSOL_AUTOORDER != 0
        && (*LUSOL).luparm[LUSOL_IP_ROWCOUNT_L0 as usize] as ::core::ffi::c_double
            / (*LUSOL).m as ::core::ffi::c_double
            > (*LUSOL).parmlu[LUSOL_RP_SMARTRATIO as usize])
    {
        *mat = LUSOL_matcreate((*LUSOL).m, LENL0);
        if (*mat).is_null() {
            *inform = LUSOL_INFORM_NOMEMLEFT;
        } else {
            *(**mat).lenx.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
            K = 1 as ::core::ffi::c_int;
            while K <= (*LUSOL).m {
                *(**mat).lenx.offset(K as isize) =
                    *(**mat).lenx.offset((K - 1 as ::core::ffi::c_int) as isize)
                        + *lsumr.offset(K as isize);
                *lsumr.offset(K as isize) =
                    *(**mat).lenx.offset((K - 1 as ::core::ffi::c_int) as isize);
                K += 1;
            }
            L2 = (*LUSOL).lena;
            L1 = L2 - LENL0 + 1 as ::core::ffi::c_int;
            L = L1;
            while L <= L2 {
                I = *(*LUSOL).indc.offset(L as isize);
                let ref mut fresh9 = *lsumr.offset(I as isize);
                let fresh10 = *fresh9;
                *fresh9 = *fresh9 + 1;
                LL = fresh10;
                *(**mat).a.offset(LL as isize) = *(*LUSOL).a.offset(L as isize);
                *(**mat).indr.offset(LL as isize) = *(*LUSOL).indr.offset(L as isize);
                *(**mat).indc.offset(LL as isize) = I;
                L += 1;
            }
            I = 0 as ::core::ffi::c_int;
            L = 1 as ::core::ffi::c_int;
            while L <= (*LUSOL).m {
                K = *(*LUSOL).ip.offset(L as isize);
                if *(**mat).lenx.offset(K as isize)
                    > *(**mat).lenx.offset((K - 1 as ::core::ffi::c_int) as isize)
                {
                    I += 1;
                    *(**mat).indx.offset(I as isize) = K;
                }
                L += 1;
            }
            status = TRUE as ::core::ffi::c_uchar;
        }
    }
    if !(lsumr as *mut ::core::ffi::c_void).is_null() {
        free(lsumr as *mut ::core::ffi::c_void);
        lsumr = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return status;
}
#[export_name="honest_lpsolve_LU6L0T_v"]
pub unsafe extern "C" fn LU6L0T_v(
    mut LUSOL: *mut LUSOLrec,
    mut mat: *mut LUSOLmat,
    mut V: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
) {
    let mut LEN: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut KK: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut NUML0: ::core::ffi::c_int = 0;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut VPIV: ::core::ffi::c_double = 0.;
    let mut aptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut jptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    NUML0 = (*LUSOL).luparm[LUSOL_IP_ROWCOUNT_L0 as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    K = NUML0;
    while K > 0 as ::core::ffi::c_int {
        KK = *(*mat).indx.offset(K as isize);
        L = *(*mat).lenx.offset(KK as isize);
        L1 = *(*mat).lenx.offset((KK - 1 as ::core::ffi::c_int) as isize);
        LEN = L - L1;
        if !(LEN == 0 as ::core::ffi::c_int) {
            VPIV = *V.offset(KK as isize);
            if fabs(VPIV) > SMALL {
                L -= 1;
                aptr = (*mat).a.offset(L as isize);
                jptr = (*mat).indr.offset(L as isize);
                while LEN > 0 as ::core::ffi::c_int {
                    *V.offset(*jptr as isize) += VPIV * *aptr;
                    LEN -= 1;
                    aptr = aptr.offset(-1);
                    jptr = jptr.offset(-1);
                }
            }
        }
        K -= 1;
    }
}
#[export_name="honest_lpsolve_LU6L"]
pub unsafe extern "C" fn LU6L(
    mut LUSOL: *mut LUSOLrec,
    mut INFORM: *mut ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
) {
    let mut JPIV: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut LEN: ::core::ffi::c_int = 0;
    let mut LENL: ::core::ffi::c_int = 0;
    let mut LENL0: ::core::ffi::c_int = 0;
    let mut NUML: ::core::ffi::c_int = 0;
    let mut NUML0: ::core::ffi::c_int = 0;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut VPIV: ::core::ffi::c_double = 0.;
    let mut aptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut iptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut jptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    NUML0 = (*LUSOL).luparm[LUSOL_IP_COLCOUNT_L0 as usize];
    LENL0 = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L0 as usize];
    LENL = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    *INFORM = LUSOL_INFORM_LUSUCCESS;
    L1 = (*LUSOL).lena + 1 as ::core::ffi::c_int;
    K = 1 as ::core::ffi::c_int;
    while K <= NUML0 {
        LEN = *(*LUSOL).lenc.offset(K as isize);
        L = L1;
        L1 -= LEN;
        JPIV = *(*LUSOL).indr.offset(L1 as isize);
        VPIV = *V.offset(JPIV as isize);
        if fabs(VPIV) > SMALL {
            L -= 1;
            aptr = (*LUSOL).a.offset(L as isize);
            iptr = (*LUSOL).indc.offset(L as isize);
            while LEN > 0 as ::core::ffi::c_int {
                *V.offset(*iptr as isize) += *aptr * VPIV;
                LEN -= 1;
                aptr = aptr.offset(-1);
                iptr = iptr.offset(-1);
            }
        }
        K += 1;
    }
    L = (*LUSOL).lena - LENL0 + 1 as ::core::ffi::c_int;
    NUML = LENL - LENL0;
    L -= 1;
    aptr = (*LUSOL).a.offset(L as isize);
    jptr = (*LUSOL).indr.offset(L as isize);
    iptr = (*LUSOL).indc.offset(L as isize);
    while NUML > 0 as ::core::ffi::c_int {
        if fabs(*V.offset(*jptr as isize)) > SMALL {
            *V.offset(*iptr as isize) += *aptr * *V.offset(*jptr as isize);
        }
        NUML -= 1;
        aptr = aptr.offset(-1);
        jptr = jptr.offset(-1);
        iptr = iptr.offset(-1);
    }
    (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
}
#[export_name="honest_lpsolve_LU6LD"]
pub unsafe extern "C" fn LU6LD(
    mut LUSOL: *mut LUSOLrec,
    mut INFORM: *mut ::core::ffi::c_int,
    mut MODE: ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
) {
    let mut IPIV: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut LEN: ::core::ffi::c_int = 0;
    let mut NUML0: ::core::ffi::c_int = 0;
    let mut DIAG: ::core::ffi::c_double = 0.;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut VPIV: ::core::ffi::c_double = 0.;
    let mut aptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut jptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    NUML0 = (*LUSOL).luparm[LUSOL_IP_COLCOUNT_L0 as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    *INFORM = LUSOL_INFORM_LUSUCCESS;
    L1 = (*LUSOL).lena + 1 as ::core::ffi::c_int;
    K = 1 as ::core::ffi::c_int;
    while K <= NUML0 {
        LEN = *(*LUSOL).lenc.offset(K as isize);
        L = L1;
        L1 -= LEN;
        IPIV = *(*LUSOL).indr.offset(L1 as isize);
        VPIV = *V.offset(IPIV as isize);
        if fabs(VPIV) > SMALL {
            L -= 1;
            aptr = (*LUSOL).a.offset(L as isize);
            jptr = (*LUSOL).indc.offset(L as isize);
            while LEN > 0 as ::core::ffi::c_int {
                *V.offset(*jptr as isize) += *aptr * VPIV;
                LEN -= 1;
                aptr = aptr.offset(-1);
                jptr = jptr.offset(-1);
            }
            L = *(*LUSOL).locr.offset(IPIV as isize);
            DIAG = *(*LUSOL).a.offset(L as isize);
            if MODE == 2 as ::core::ffi::c_int {
                DIAG = fabs(DIAG);
            }
            *V.offset(IPIV as isize) = VPIV / DIAG;
        }
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU6LT"]
pub unsafe extern "C" fn LU6LT(
    mut LUSOL: *mut LUSOLrec,
    mut INFORM: *mut ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
) {
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut LEN: ::core::ffi::c_int = 0;
    let mut LENL: ::core::ffi::c_int = 0;
    let mut LENL0: ::core::ffi::c_int = 0;
    let mut NUML0: ::core::ffi::c_int = 0;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut SUM: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
    let mut HOLD: ::core::ffi::c_double = 0.;
    let mut aptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut iptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut jptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    NUML0 = (*LUSOL).luparm[LUSOL_IP_COLCOUNT_L0 as usize];
    LENL0 = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L0 as usize];
    LENL = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    *INFORM = LUSOL_INFORM_LUSUCCESS;
    L1 = (*LUSOL).lena - LENL + 1 as ::core::ffi::c_int;
    L2 = (*LUSOL).lena - LENL0;
    L = L1;
    aptr = (*LUSOL).a.offset(L1 as isize);
    iptr = (*LUSOL).indr.offset(L1 as isize);
    jptr = (*LUSOL).indc.offset(L1 as isize);
    while L <= L2 {
        HOLD = *V.offset(*jptr as isize);
        if fabs(HOLD) > SMALL {
            *V.offset(*iptr as isize) += *aptr * HOLD;
        }
        L += 1;
        aptr = aptr.offset(1);
        iptr = iptr.offset(1);
        jptr = jptr.offset(1);
    }
    if !(*LUSOL).L0.is_null()
        || (*LUSOL).luparm[LUSOL_IP_BTRANCOUNT as usize] == 0 as ::core::ffi::c_int
            && LU1L0(LUSOL, &raw mut (*LUSOL).L0, INFORM) as ::core::ffi::c_int != 0
    {
        LU6L0T_v(LUSOL, (*LUSOL).L0, V, NZidx, INFORM);
    } else {
        K = NUML0;
        while K >= 1 as ::core::ffi::c_int {
            SUM = crate::honest_did::lpsolve::extended::Extended::new(ZERO);
            LEN = *(*LUSOL).lenc.offset(K as isize);
            L1 = L2 + 1 as ::core::ffi::c_int;
            L2 += LEN;
            L = L1;
            aptr = (*LUSOL).a.offset(L1 as isize);
            jptr = (*LUSOL).indc.offset(L1 as isize);
            while L <= L2 {
                SUM += crate::honest_did::lpsolve::extended::Extended::new(*aptr * *V.offset(*jptr as isize));
                L += 1;
                aptr = aptr.offset(1);
                jptr = jptr.offset(1);
            }
            *V.offset(*(*LUSOL).indr.offset(L1 as isize) as isize) += SUM.to_f64().unwrap();
            K -= 1;
        }
    }
    (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
}
#[export_name="honest_lpsolve_print_L0"]
pub unsafe extern "C" fn print_L0(mut LUSOL: *mut LUSOLrec) {
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut LEN: ::core::ffi::c_int = 0;
    let mut LENL0: ::core::ffi::c_int = 0;
    let mut NUML0: ::core::ffi::c_int = 0;
    let mut denseL0: *mut ::core::ffi::c_double = calloc(
        ((*LUSOL).m + 1 as ::core::ffi::c_int) as size_t,
        (((*LUSOL).n + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    ) as *mut ::core::ffi::c_double;
    NUML0 = (*LUSOL).luparm[LUSOL_IP_COLCOUNT_L0 as usize];
    LENL0 = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L0 as usize];
    L2 = (*LUSOL).lena - LENL0;
    K = NUML0;
    while K >= 1 as ::core::ffi::c_int {
        LEN = *(*LUSOL).lenc.offset(K as isize);
        L1 = L2 + 1 as ::core::ffi::c_int;
        L2 += LEN;
        L = L1;
        while L <= L2 {
            I = *(*LUSOL).indc.offset(L as isize);
            I = *(*LUSOL).ipinv.offset(I as isize);
            J = *(*LUSOL).indr.offset(L as isize);
            *denseL0.offset(
                (((*LUSOL).n + 1 as ::core::ffi::c_int) * (J - 1 as ::core::ffi::c_int) + I)
                    as isize,
            ) = *(*LUSOL).a.offset(L as isize);
            L += 1;
        }
        K -= 1;
    }
    I = 1 as ::core::ffi::c_int;
    while I <= (*LUSOL).n {
        J = 1 as ::core::ffi::c_int;
        while J <= (*LUSOL).m {
            J += 1;
        }
        I += 1;
    }
    free(denseL0 as *mut ::core::ffi::c_void);
    denseL0 = ::core::ptr::null_mut::<::core::ffi::c_double>();
}
#[export_name="honest_lpsolve_LU1U0"]
pub unsafe extern "C" fn LU1U0(
    mut LUSOL: *mut LUSOLrec,
    mut mat: *mut *mut LUSOLmat,
    mut inform: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LL: ::core::ffi::c_int = 0;
    let mut LENU: ::core::ffi::c_int = 0;
    let mut NUMU: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut lsumc: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    *inform = LUSOL_INFORM_LUSUCCESS;
    if mat.is_null() {
        return status;
    }
    if !(*mat).is_null() {
        LUSOL_matfree(mat);
    }
    NUMU = (*LUSOL).luparm[LUSOL_IP_RANK_U as usize];
    LENU = (*LUSOL).luparm[LUSOL_IP_NONZEROS_U as usize];
    if NUMU == 0 as ::core::ffi::c_int
        || LENU == 0 as ::core::ffi::c_int
        || (*LUSOL).luparm[LUSOL_IP_ACCELERATION as usize] == LUSOL_BASEORDER
        || (*LUSOL).luparm[LUSOL_IP_ACCELERATION as usize] & LUSOL_ACCELERATE_U
            == 0 as ::core::ffi::c_int
    {
        return status;
    }
    lsumc = calloc(
        ((*LUSOL).n + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if lsumc.is_null() {
        *inform = LUSOL_INFORM_NOMEMLEFT;
        return status;
    }
    L = 1 as ::core::ffi::c_int;
    while L <= LENU {
        J = *(*LUSOL).indr.offset(L as isize);
        let ref mut fresh11 = *lsumc.offset(J as isize);
        *fresh11 += 1;
        L += 1;
    }
    if !((*LUSOL).luparm[LUSOL_IP_ACCELERATION as usize] & LUSOL_AUTOORDER != 0
        && sqrt(NUMU as ::core::ffi::c_double / LENU as ::core::ffi::c_double)
            > (*LUSOL).parmlu[LUSOL_RP_SMARTRATIO as usize])
    {
        *mat = LUSOL_matcreate((*LUSOL).n, LENU);
        if (*mat).is_null() {
            *inform = LUSOL_INFORM_NOMEMLEFT;
        } else {
            *(**mat).lenx.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
            K = 1 as ::core::ffi::c_int;
            while K <= (*LUSOL).n {
                *(**mat).lenx.offset(K as isize) =
                    *(**mat).lenx.offset((K - 1 as ::core::ffi::c_int) as isize)
                        + *lsumc.offset(K as isize);
                *lsumc.offset(K as isize) =
                    *(**mat).lenx.offset((K - 1 as ::core::ffi::c_int) as isize);
                K += 1;
            }
            L = 1 as ::core::ffi::c_int;
            while L <= LENU {
                J = *(*LUSOL).indr.offset(L as isize);
                let ref mut fresh12 = *lsumc.offset(J as isize);
                let fresh13 = *fresh12;
                *fresh12 = *fresh12 + 1;
                LL = fresh13;
                *(**mat).a.offset(LL as isize) = *(*LUSOL).a.offset(L as isize);
                *(**mat).indr.offset(LL as isize) = J;
                *(**mat).indc.offset(LL as isize) = *(*LUSOL).indc.offset(L as isize);
                L += 1;
            }
            J = 0 as ::core::ffi::c_int;
            L = 1 as ::core::ffi::c_int;
            while L <= (*LUSOL).n {
                K = *(*LUSOL).iq.offset(L as isize);
                if *(**mat).lenx.offset(K as isize)
                    > *(**mat).lenx.offset((K - 1 as ::core::ffi::c_int) as isize)
                {
                    J += 1;
                    *(**mat).indx.offset(J as isize) = K;
                }
                L += 1;
            }
            status = TRUE as ::core::ffi::c_uchar;
        }
    }
    if !(lsumc as *mut ::core::ffi::c_void).is_null() {
        free(lsumc as *mut ::core::ffi::c_void);
        lsumc = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return status;
}
#[export_name="honest_lpsolve_LU6U0_v"]
pub unsafe extern "C" fn LU6U0_v(
    mut LUSOL: *mut LUSOLrec,
    mut mat: *mut LUSOLmat,
    mut V: *mut ::core::ffi::c_double,
    mut W: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
) {
    let mut LEN: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut NRANK: ::core::ffi::c_int = 0;
    let mut NRANK1: ::core::ffi::c_int = 0;
    let mut KLAST: ::core::ffi::c_int = 0;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut T: ::core::ffi::c_double = 0.;
    let mut J: ::core::ffi::c_int = 0;
    NRANK = (*LUSOL).luparm[LUSOL_IP_RANK_U as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    *INFORM = LUSOL_INFORM_LUSUCCESS;
    NRANK1 = NRANK + 1 as ::core::ffi::c_int;
    KLAST = NRANK;
    while KLAST >= 1 as ::core::ffi::c_int {
        I = *(*LUSOL).ip.offset(KLAST as isize);
        if fabs(*V.offset(I as isize)) > SMALL {
            break;
        }
        KLAST -= 1;
    }
    L = (*LUSOL).n;
    K = KLAST + 1 as ::core::ffi::c_int;
    while K <= L {
        J = *(*LUSOL).iq.offset(K as isize);
        *W.offset(J as isize) = ZERO as ::core::ffi::c_double;
        K += 1;
    }
    K = NRANK;
    while K > 0 as ::core::ffi::c_int {
        I = *(*mat).indx.offset(K as isize);
        L = *(*mat).lenx.offset(I as isize);
        L1 = *(*mat).lenx.offset((I - 1 as ::core::ffi::c_int) as isize);
        LEN = L - L1;
        T = *V.offset(I as isize);
        if fabs(T) <= SMALL {
            *W.offset(K as isize) = ZERO as ::core::ffi::c_double;
        } else {
            T /= *(*mat).a.offset(L1 as isize);
            *W.offset(K as isize) = T;
            LEN -= 1;
            while LEN > 0 as ::core::ffi::c_int {
                L -= 1;
                J = *(*mat).indc.offset(L as isize);
                *V.offset(J as isize) -= T * *(*mat).a.offset(L as isize);
                LEN -= 1;
            }
        }
        K -= 1;
    }
    T = ZERO as ::core::ffi::c_double;
    K = NRANK1;
    while K <= (*LUSOL).m {
        I = *(*LUSOL).ip.offset(K as isize);
        T += fabs(*V.offset(I as isize));
        K += 1;
    }
    if T > ZERO as ::core::ffi::c_double {
        *INFORM = LUSOL_INFORM_LUSINGULAR;
    }
    (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
    (*LUSOL).parmlu[LUSOL_RP_RESIDUAL_U as usize] = T;
}
#[export_name="honest_lpsolve_LU6U"]
pub unsafe extern "C" fn LU6U(
    mut LUSOL: *mut LUSOLrec,
    mut INFORM: *mut ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut W: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
) {
    if !(*LUSOL).U.is_null()
        || (*LUSOL).luparm[LUSOL_IP_FTRANCOUNT as usize] == 0 as ::core::ffi::c_int
            && LU1U0(LUSOL, &raw mut (*LUSOL).U, INFORM) as ::core::ffi::c_int != 0
    {
        LU6U0_v(LUSOL, (*LUSOL).U, V, W, NZidx, INFORM);
    } else {
        let mut I: ::core::ffi::c_int = 0;
        let mut J: ::core::ffi::c_int = 0;
        let mut K: ::core::ffi::c_int = 0;
        let mut KLAST: ::core::ffi::c_int = 0;
        let mut L: ::core::ffi::c_int = 0;
        let mut L1: ::core::ffi::c_int = 0;
        let mut L2: ::core::ffi::c_int = 0;
        let mut L3: ::core::ffi::c_int = 0;
        let mut NRANK: ::core::ffi::c_int = 0;
        let mut NRANK1: ::core::ffi::c_int = 0;
        let mut SMALL: ::core::ffi::c_double = 0.;
        let mut T: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
        let mut aptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut jptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        NRANK = (*LUSOL).luparm[LUSOL_IP_RANK_U as usize];
        SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
        *INFORM = LUSOL_INFORM_LUSUCCESS;
        NRANK1 = NRANK + 1 as ::core::ffi::c_int;
        KLAST = NRANK;
        while KLAST >= 1 as ::core::ffi::c_int {
            I = *(*LUSOL).ip.offset(KLAST as isize);
            if fabs(*V.offset(I as isize)) > SMALL {
                break;
            }
            KLAST -= 1;
        }
        L = (*LUSOL).n;
        K = KLAST + 1 as ::core::ffi::c_int;
        jptr = (*LUSOL).iq.offset(K as isize);
        while K <= L {
            *W.offset(*jptr as isize) = ZERO as ::core::ffi::c_double;
            K += 1;
            jptr = jptr.offset(1);
        }
        K = KLAST;
        while K >= 1 as ::core::ffi::c_int {
            I = *(*LUSOL).ip.offset(K as isize);
            T = crate::honest_did::lpsolve::extended::Extended::new(*V.offset(I as isize));
            L1 = *(*LUSOL).locr.offset(I as isize);
            L2 = L1 + 1 as ::core::ffi::c_int;
            L3 = L1 + *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
            L = L2;
            aptr = (*LUSOL).a.offset(L2 as isize);
            jptr = (*LUSOL).indr.offset(L2 as isize);
            while L <= L3 {
                T -= crate::honest_did::lpsolve::extended::Extended::new(*aptr * *W.offset(*jptr as isize));
                L += 1;
                aptr = aptr.offset(1);
                jptr = jptr.offset(1);
            }
            J = *(*LUSOL).iq.offset(K as isize);
            if fabs(T.to_f64().unwrap()) <= SMALL {
                T = crate::honest_did::lpsolve::extended::Extended::new(ZERO);
            } else {
                T /= crate::honest_did::lpsolve::extended::Extended::new(*(*LUSOL).a.offset(L1 as isize));
            }
            *W.offset(J as isize) = T.to_f64().unwrap();
            K -= 1;
        }
        T = crate::honest_did::lpsolve::extended::Extended::new(ZERO);
        K = NRANK1;
        while K <= (*LUSOL).m {
            I = *(*LUSOL).ip.offset(K as isize);
            T += crate::honest_did::lpsolve::extended::Extended::new(fabs(*V.offset(I as isize)));
            K += 1;
        }
        if T > crate::honest_did::lpsolve::extended::Extended::new(ZERO) {
            *INFORM = LUSOL_INFORM_LUSINGULAR;
        }
        (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
        (*LUSOL).parmlu[LUSOL_RP_RESIDUAL_U as usize] = T.to_f64().unwrap();
    };
}
#[export_name="honest_lpsolve_LU6UT"]
pub unsafe extern "C" fn LU6UT(
    mut LUSOL: *mut LUSOLrec,
    mut INFORM: *mut ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut W: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
) {
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut NRANK: ::core::ffi::c_int = 0;
    let mut NRANK1: ::core::ffi::c_int = 0;
    let mut ip: *mut ::core::ffi::c_int = (*LUSOL).ip.offset(1 as ::core::ffi::c_int as isize);
    let mut iq: *mut ::core::ffi::c_int = (*LUSOL).iq.offset(1 as ::core::ffi::c_int as isize);
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut T: ::core::ffi::c_double = 0.;
    let mut aptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut jptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    NRANK = (*LUSOL).luparm[LUSOL_IP_RANK_U as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    *INFORM = LUSOL_INFORM_LUSUCCESS;
    NRANK1 = NRANK + 1 as ::core::ffi::c_int;
    L = (*LUSOL).m;
    K = NRANK1;
    jptr = (*LUSOL).ip.offset(K as isize);
    while K <= L {
        *V.offset(*jptr as isize) = ZERO as ::core::ffi::c_double;
        K += 1;
        jptr = jptr.offset(1);
    }
    K = 1 as ::core::ffi::c_int;
    while K <= NRANK {
        I = *ip;
        J = *iq;
        T = *W.offset(J as isize);
        if fabs(T) <= SMALL {
            *V.offset(I as isize) = ZERO as ::core::ffi::c_double;
        } else {
            L1 = *(*LUSOL).locr.offset(I as isize);
            T /= *(*LUSOL).a.offset(L1 as isize);
            *V.offset(I as isize) = T;
            L2 = L1 + *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
            L1 += 1;
            L = L1;
            aptr = (*LUSOL).a.offset(L1 as isize);
            jptr = (*LUSOL).indr.offset(L1 as isize);
            while L <= L2 {
                *W.offset(*jptr as isize) -= T * *aptr;
                L += 1;
                aptr = aptr.offset(1);
                jptr = jptr.offset(1);
            }
        }
        K += 1;
        ip = ip.offset(1);
        iq = iq.offset(1);
    }
    T = ZERO as ::core::ffi::c_double;
    K = NRANK1;
    while K <= (*LUSOL).n {
        J = *(*LUSOL).iq.offset(K as isize);
        T += fabs(*W.offset(J as isize));
        K += 1;
    }
    if T > ZERO as ::core::ffi::c_double {
        *INFORM = LUSOL_INFORM_LUSINGULAR;
    }
    (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
    (*LUSOL).parmlu[LUSOL_RP_RESIDUAL_U as usize] = T;
}
#[export_name="honest_lpsolve_LU6SOL"]
pub unsafe extern "C" fn LU6SOL(
    mut LUSOL: *mut LUSOLrec,
    mut MODE: ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut W: *mut ::core::ffi::c_double,
    mut NZidx: *mut ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
) {
    if MODE == LUSOL_SOLVE_Lv_v {
        LU6L(LUSOL, INFORM, V, NZidx);
    } else if MODE == LUSOL_SOLVE_Ltv_v {
        LU6LT(LUSOL, INFORM, V, NZidx);
    } else if MODE == LUSOL_SOLVE_Uw_v {
        LU6U(LUSOL, INFORM, V, W, NZidx);
    } else if MODE == LUSOL_SOLVE_Utv_w {
        LU6UT(LUSOL, INFORM, V, W, NZidx);
    } else if MODE == LUSOL_SOLVE_Aw_v {
        LU6L(LUSOL, INFORM, V, NZidx);
        LU6U(
            LUSOL,
            INFORM,
            V,
            W,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    } else if MODE == LUSOL_SOLVE_Atv_w {
        LU6UT(LUSOL, INFORM, V, W, NZidx);
        LU6LT(
            LUSOL,
            INFORM,
            V,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    } else if MODE == LUSOL_SOLVE_Av_v {
        LU6LD(LUSOL, INFORM, 1 as ::core::ffi::c_int, V, NZidx);
        LU6LT(
            LUSOL,
            INFORM,
            V,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    } else if MODE == LUSOL_SOLVE_LDLtv_v {
        LU6LD(LUSOL, INFORM, 2 as ::core::ffi::c_int, V, NZidx);
        LU6LT(
            LUSOL,
            INFORM,
            V,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    }
}
#[export_name="honest_lpsolve_LU1DCP"]
pub unsafe extern "C" fn LU1DCP(
    mut LUSOL: *mut LUSOLrec,
    mut DA: *mut ::core::ffi::c_double,
    mut LDA: ::core::ffi::c_int,
    mut M: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut SMALL: ::core::ffi::c_double,
    mut NSING: *mut ::core::ffi::c_int,
    mut IPVT: *mut ::core::ffi::c_int,
    mut IX: *mut ::core::ffi::c_int,
) {
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut KP1: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LAST: ::core::ffi::c_int = 0;
    let mut LENCOL: ::core::ffi::c_int = 0;
    let mut IMAX: ::core::ffi::c_int = 0;
    let mut JMAX: ::core::ffi::c_int = 0;
    let mut JLAST: ::core::ffi::c_int = 0;
    let mut JNEW: ::core::ffi::c_int = 0;
    let mut AIJMAX: ::core::ffi::c_double = 0.;
    let mut AJMAX: ::core::ffi::c_double = 0.;
    let mut T: ::core::ffi::c_double = 0.;
    let mut DA1: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut DA2: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut IDA1: ::core::ffi::c_int = 0;
    let mut IDA2: ::core::ffi::c_int = 0;
    *NSING = 0 as ::core::ffi::c_int;
    LENCOL = M + 1 as ::core::ffi::c_int;
    LAST = N;
    K = 1 as ::core::ffi::c_int;
    while K <= N {
        KP1 = K + 1 as ::core::ffi::c_int;
        LENCOL -= 1;
        AIJMAX = ZERO as ::core::ffi::c_double;
        IMAX = K;
        JMAX = K;
        JLAST = LAST;
        J = K;
        's_48: while J <= JLAST {
            loop {
                L = lps_idamax(
                    LENCOL,
                    DA.offset((K + (J - 1 as ::core::ffi::c_int) * LDA) as isize)
                        .offset(-(LUSOL_ARRAYOFFSET as isize)),
                    1 as ::core::ffi::c_int,
                ) + K
                    - 1 as ::core::ffi::c_int;
                AJMAX = fabs(*DA.offset((L + (J - 1 as ::core::ffi::c_int) * LDA) as isize));
                if !(AJMAX <= SMALL) {
                    break;
                }
                *NSING += 1;
                JNEW = *IX.offset(LAST as isize);
                *IX.offset(LAST as isize) = *IX.offset(J as isize);
                *IX.offset(J as isize) = JNEW;
                DA1 = DA.offset(
                    (0 as ::core::ffi::c_int + (LAST - 1 as ::core::ffi::c_int) * LDA) as isize,
                ) as *mut ::core::ffi::c_double;
                DA2 = DA.offset(
                    (0 as ::core::ffi::c_int + (J - 1 as ::core::ffi::c_int) * LDA) as isize,
                ) as *mut ::core::ffi::c_double;
                I = 1 as ::core::ffi::c_int;
                while I <= K - 1 as ::core::ffi::c_int {
                    DA1 = DA1.offset(1);
                    DA2 = DA2.offset(1);
                    T = *DA1;
                    *DA1 = *DA2;
                    *DA2 = T;
                    I += 1;
                }
                I = K;
                while I <= M {
                    DA1 = DA1.offset(1);
                    DA2 = DA2.offset(1);
                    T = *DA1;
                    *DA1 = ZERO as ::core::ffi::c_double;
                    *DA2 = T;
                    I += 1;
                }
                LAST -= 1;
                if !(J <= LAST) {
                    break 's_48;
                }
            }
            if AIJMAX < AJMAX {
                AIJMAX = AJMAX;
                IMAX = L;
                JMAX = J;
            }
            if J >= LAST {
                break;
            }
            J += 1;
        }
        *IPVT.offset(K as isize) = IMAX;
        if JMAX != K {
            JNEW = *IX.offset(JMAX as isize);
            *IX.offset(JMAX as isize) = *IX.offset(K as isize);
            *IX.offset(K as isize) = JNEW;
            DA1 = DA
                .offset((0 as ::core::ffi::c_int + (JMAX - 1 as ::core::ffi::c_int) * LDA) as isize)
                as *mut ::core::ffi::c_double;
            DA2 = DA
                .offset((0 as ::core::ffi::c_int + (K - 1 as ::core::ffi::c_int) * LDA) as isize)
                as *mut ::core::ffi::c_double;
            I = 1 as ::core::ffi::c_int;
            while I <= M {
                DA1 = DA1.offset(1);
                DA2 = DA2.offset(1);
                T = *DA1;
                *DA1 = *DA2;
                *DA2 = T;
                I += 1;
            }
        }
        if !(M > K) {
            break;
        }
        if IMAX != K {
            IDA1 = IMAX + (K - 1 as ::core::ffi::c_int) * LDA;
            IDA2 = K + (K - 1 as ::core::ffi::c_int) * LDA;
            T = *DA.offset(IDA1 as isize);
            *DA.offset(IDA1 as isize) = *DA.offset(IDA2 as isize);
            *DA.offset(IDA2 as isize) = T;
        }
        T = -ONE as ::core::ffi::c_double
            / *DA.offset((K + (K - 1 as ::core::ffi::c_int) * LDA) as isize);
        lps_dscal(
            M - K,
            T,
            DA.offset((KP1 + (K - 1 as ::core::ffi::c_int) * LDA) as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
            1 as ::core::ffi::c_int,
        );
        J = KP1;
        while J <= LAST {
            IDA1 = IMAX + (J - 1 as ::core::ffi::c_int) * LDA;
            T = *DA.offset(IDA1 as isize);
            if IMAX != K {
                IDA2 = K + (J - 1 as ::core::ffi::c_int) * LDA;
                *DA.offset(IDA1 as isize) = *DA.offset(IDA2 as isize);
                *DA.offset(IDA2 as isize) = T;
            }
            lps_daxpy(
                M - K,
                T,
                DA.offset((KP1 + (K - 1 as ::core::ffi::c_int) * LDA) as isize)
                    .offset(-(LUSOL_ARRAYOFFSET as isize)),
                1 as ::core::ffi::c_int,
                DA.offset((KP1 + (J - 1 as ::core::ffi::c_int) * LDA) as isize)
                    .offset(-(LUSOL_ARRAYOFFSET as isize)),
                1 as ::core::ffi::c_int,
            );
            J += 1;
        }
        if K >= LAST {
            break;
        }
        K += 1;
    }
    K = LAST + 1 as ::core::ffi::c_int;
    while K <= M {
        *IPVT.offset(K as isize) = K;
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU1DPP"]
pub unsafe extern "C" fn LU1DPP(
    mut LUSOL: *mut LUSOLrec,
    mut DA: *mut ::core::ffi::c_double,
    mut LDA: ::core::ffi::c_int,
    mut M: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut SMALL: ::core::ffi::c_double,
    mut NSING: *mut ::core::ffi::c_int,
    mut IPVT: *mut ::core::ffi::c_int,
    mut IX: *mut ::core::ffi::c_int,
) {
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut KP1: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LAST: ::core::ffi::c_int = 0;
    let mut LENCOL: ::core::ffi::c_int = 0;
    let mut T: ::core::ffi::c_double = 0.;
    let mut DA1: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut DA2: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut IDA1: ::core::ffi::c_int = 0;
    let mut IDA2: ::core::ffi::c_int = 0;
    *NSING = 0 as ::core::ffi::c_int;
    K = 1 as ::core::ffi::c_int;
    LAST = N;
    loop {
        KP1 = K + 1 as ::core::ffi::c_int;
        LENCOL = M - K + 1 as ::core::ffi::c_int;
        L = lps_idamax(
            LENCOL,
            DA.offset((K + (K - 1 as ::core::ffi::c_int) * LDA) as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
            1 as ::core::ffi::c_int,
        ) + K
            - 1 as ::core::ffi::c_int;
        *IPVT.offset(K as isize) = L;
        if fabs(*DA.offset((L + (K - 1 as ::core::ffi::c_int) * LDA) as isize)) <= SMALL {
            *NSING += 1;
            J = *IX.offset(LAST as isize);
            *IX.offset(LAST as isize) = *IX.offset(K as isize);
            *IX.offset(K as isize) = J;
            DA1 = DA
                .offset((0 as ::core::ffi::c_int + (LAST - 1 as ::core::ffi::c_int) * LDA) as isize)
                as *mut ::core::ffi::c_double;
            DA2 = DA
                .offset((0 as ::core::ffi::c_int + (K - 1 as ::core::ffi::c_int) * LDA) as isize)
                as *mut ::core::ffi::c_double;
            I = 1 as ::core::ffi::c_int;
            while I <= K - 1 as ::core::ffi::c_int {
                DA1 = DA1.offset(1);
                DA2 = DA2.offset(1);
                T = *DA1;
                *DA1 = *DA2;
                *DA2 = T;
                I += 1;
            }
            I = K;
            while I <= M {
                DA1 = DA1.offset(1);
                DA2 = DA2.offset(1);
                T = *DA1;
                *DA1 = ZERO as ::core::ffi::c_double;
                *DA2 = T;
                I += 1;
            }
            LAST = LAST - 1 as ::core::ffi::c_int;
            if !(K <= LAST) {
                break;
            }
        } else {
            if !(M > K) {
                break;
            }
            if L != K {
                IDA1 = L + (K - 1 as ::core::ffi::c_int) * LDA;
                IDA2 = K + (K - 1 as ::core::ffi::c_int) * LDA;
                T = *DA.offset(IDA1 as isize);
                *DA.offset(IDA1 as isize) = *DA.offset(IDA2 as isize);
                *DA.offset(IDA2 as isize) = T;
            }
            T = -ONE as ::core::ffi::c_double
                / *DA.offset((K + (K - 1 as ::core::ffi::c_int) * LDA) as isize);
            lps_dscal(
                M - K,
                T,
                DA.offset((KP1 + (K - 1 as ::core::ffi::c_int) * LDA) as isize)
                    .offset(-(LUSOL_ARRAYOFFSET as isize)),
                1 as ::core::ffi::c_int,
            );
            J = KP1;
            while J <= LAST {
                IDA1 = L + (J - 1 as ::core::ffi::c_int) * LDA;
                T = *DA.offset(IDA1 as isize);
                if L != K {
                    IDA2 = K + (J - 1 as ::core::ffi::c_int) * LDA;
                    *DA.offset(IDA1 as isize) = *DA.offset(IDA2 as isize);
                    *DA.offset(IDA2 as isize) = T;
                }
                lps_daxpy(
                    M - K,
                    T,
                    DA.offset((KP1 + (K - 1 as ::core::ffi::c_int) * LDA) as isize)
                        .offset(-(LUSOL_ARRAYOFFSET as isize)),
                    1 as ::core::ffi::c_int,
                    DA.offset((KP1 + (J - 1 as ::core::ffi::c_int) * LDA) as isize)
                        .offset(-(LUSOL_ARRAYOFFSET as isize)),
                    1 as ::core::ffi::c_int,
                );
                J += 1;
            }
            K += 1;
            if !(K <= LAST) {
                break;
            }
        }
    }
    K = LAST + 1 as ::core::ffi::c_int;
    while K <= M {
        *IPVT.offset(K as isize) = K;
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU1PQ1"]
pub unsafe extern "C" fn LU1PQ1(
    mut LUSOL: *mut LUSOLrec,
    mut M: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
    mut LEN: *mut ::core::ffi::c_int,
    mut IPERM: *mut ::core::ffi::c_int,
    mut LOC: *mut ::core::ffi::c_int,
    mut INV: *mut ::core::ffi::c_int,
    mut NUM: *mut ::core::ffi::c_int,
) {
    let mut NZEROS: ::core::ffi::c_int = 0;
    let mut NZ: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    NZEROS = 0 as ::core::ffi::c_int;
    NZ = 1 as ::core::ffi::c_int;
    while NZ <= N {
        *NUM.offset(NZ as isize) = 0 as ::core::ffi::c_int;
        *LOC.offset(NZ as isize) = 0 as ::core::ffi::c_int;
        NZ += 1;
    }
    I = 1 as ::core::ffi::c_int;
    while I <= M {
        NZ = *LEN.offset(I as isize);
        if NZ == 0 as ::core::ffi::c_int {
            NZEROS += 1;
        } else {
            let ref mut fresh3 = *NUM.offset(NZ as isize);
            *fresh3 += 1;
        }
        I += 1;
    }
    L = NZEROS + 1 as ::core::ffi::c_int;
    NZ = 1 as ::core::ffi::c_int;
    while NZ <= N {
        *LOC.offset(NZ as isize) = L;
        L += *NUM.offset(NZ as isize);
        *NUM.offset(NZ as isize) = 0 as ::core::ffi::c_int;
        NZ += 1;
    }
    NZEROS = 0 as ::core::ffi::c_int;
    I = 1 as ::core::ffi::c_int;
    while I <= M {
        NZ = *LEN.offset(I as isize);
        if NZ == 0 as ::core::ffi::c_int {
            NZEROS += 1;
            *IPERM.offset(NZEROS as isize) = I;
        } else {
            L = *LOC.offset(NZ as isize) + *NUM.offset(NZ as isize);
            *IPERM.offset(L as isize) = I;
            let ref mut fresh4 = *NUM.offset(NZ as isize);
            *fresh4 += 1;
        }
        I += 1;
    }
    L = 1 as ::core::ffi::c_int;
    while L <= M {
        I = *IPERM.offset(L as isize);
        *INV.offset(I as isize) = L;
        L += 1;
    }
}
#[export_name="honest_lpsolve_LU1PQ2"]
pub unsafe extern "C" fn LU1PQ2(
    mut LUSOL: *mut LUSOLrec,
    mut NZPIV: ::core::ffi::c_int,
    mut NZCHNG: *mut ::core::ffi::c_int,
    mut IND: *mut ::core::ffi::c_int,
    mut LENOLD: *mut ::core::ffi::c_int,
    mut LENNEW: *mut ::core::ffi::c_int,
    mut IXLOC: *mut ::core::ffi::c_int,
    mut IX: *mut ::core::ffi::c_int,
    mut IXINV: *mut ::core::ffi::c_int,
) {
    let mut LR: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut NZ: ::core::ffi::c_int = 0;
    let mut NZNEW: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut NEXT: ::core::ffi::c_int = 0;
    let mut LNEW: ::core::ffi::c_int = 0;
    let mut JNEW: ::core::ffi::c_int = 0;
    *NZCHNG = 0 as ::core::ffi::c_int;
    LR = 1 as ::core::ffi::c_int;
    while LR <= NZPIV {
        J = *IND.offset(LR as isize);
        *IND.offset(LR as isize) = 0 as ::core::ffi::c_int;
        NZ = *LENOLD.offset(LR as isize);
        NZNEW = *LENNEW.offset(J as isize);
        if NZ != NZNEW {
            L = *IXINV.offset(J as isize);
            *NZCHNG = *NZCHNG + NZNEW - NZ;
            if NZ < NZNEW {
                loop {
                    NEXT = NZ + 1 as ::core::ffi::c_int;
                    LNEW = *IXLOC.offset(NEXT as isize) - 1 as ::core::ffi::c_int;
                    if LNEW != L {
                        JNEW = *IX.offset(LNEW as isize);
                        *IX.offset(L as isize) = JNEW;
                        *IXINV.offset(JNEW as isize) = L;
                    }
                    L = LNEW;
                    *IXLOC.offset(NEXT as isize) = LNEW;
                    NZ = NEXT;
                    if !(NZ < NZNEW) {
                        break;
                    }
                }
            } else {
                loop {
                    LNEW = *IXLOC.offset(NZ as isize);
                    if LNEW != L {
                        JNEW = *IX.offset(LNEW as isize);
                        *IX.offset(L as isize) = JNEW;
                        *IXINV.offset(JNEW as isize) = L;
                    }
                    L = LNEW;
                    *IXLOC.offset(NZ as isize) = LNEW + 1 as ::core::ffi::c_int;
                    NZ = NZ - 1 as ::core::ffi::c_int;
                    if !(NZ > NZNEW) {
                        break;
                    }
                }
            }
            *IX.offset(LNEW as isize) = J;
            *IXINV.offset(J as isize) = LNEW;
        }
        LR += 1;
    }
}
#[export_name="honest_lpsolve_LU1PQ3"]
pub unsafe extern "C" fn LU1PQ3(
    mut LUSOL: *mut LUSOLrec,
    mut MN: ::core::ffi::c_int,
    mut LEN: *mut ::core::ffi::c_int,
    mut IPERM: *mut ::core::ffi::c_int,
    mut IW: *mut ::core::ffi::c_int,
    mut NRANK: *mut ::core::ffi::c_int,
) {
    let mut NZEROS: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    *NRANK = 0 as ::core::ffi::c_int;
    NZEROS = 0 as ::core::ffi::c_int;
    K = 1 as ::core::ffi::c_int;
    while K <= MN {
        I = *IPERM.offset(K as isize);
        if *LEN.offset(I as isize) == 0 as ::core::ffi::c_int {
            NZEROS += 1;
            *IW.offset(NZEROS as isize) = I;
        } else {
            *NRANK += 1;
            *IPERM.offset(*NRANK as isize) = I;
        }
        K += 1;
    }
    K = 1 as ::core::ffi::c_int;
    while K <= NZEROS {
        *IPERM.offset((*NRANK + K) as isize) = *IW.offset(K as isize);
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU1REC"]
pub unsafe extern "C" fn LU1REC(
    mut LUSOL: *mut LUSOLrec,
    mut N: ::core::ffi::c_int,
    mut REALS: ::core::ffi::c_uchar,
    mut LTOP: *mut ::core::ffi::c_int,
    mut IND: *mut ::core::ffi::c_int,
    mut LEN: *mut ::core::ffi::c_int,
    mut LOC: *mut ::core::ffi::c_int,
) {
    let mut NEMPTY: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LENI: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LEND: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut KLAST: ::core::ffi::c_int = 0;
    let mut ILAST: ::core::ffi::c_int = 0;
    let mut LPRINT: ::core::ffi::c_int = 0;
    NEMPTY = 0 as ::core::ffi::c_int;
    I = 1 as ::core::ffi::c_int;
    while I <= N {
        LENI = *LEN.offset(I as isize);
        if LENI > 0 as ::core::ffi::c_int {
            L = *LOC.offset(I as isize) + LENI - 1 as ::core::ffi::c_int;
            *LEN.offset(I as isize) = *IND.offset(L as isize);
            *IND.offset(L as isize) = -(N + I);
        } else if LENI == 0 as ::core::ffi::c_int {
            NEMPTY += 1;
        }
        I += 1;
    }
    K = 0 as ::core::ffi::c_int;
    KLAST = 0 as ::core::ffi::c_int;
    ILAST = 0 as ::core::ffi::c_int;
    LEND = *LTOP;
    L = 1 as ::core::ffi::c_int;
    while L <= LEND {
        I = *IND.offset(L as isize);
        if I > 0 as ::core::ffi::c_int {
            K += 1;
            *IND.offset(K as isize) = I;
            if REALS != 0 {
                *(*LUSOL).a.offset(K as isize) = *(*LUSOL).a.offset(L as isize);
            }
        } else if I < -N {
            I = -(N + I);
            ILAST = I;
            K += 1;
            *IND.offset(K as isize) = *LEN.offset(I as isize);
            if REALS != 0 {
                *(*LUSOL).a.offset(K as isize) = *(*LUSOL).a.offset(L as isize);
            }
            *LOC.offset(I as isize) = KLAST + 1 as ::core::ffi::c_int;
            *LEN.offset(I as isize) = K - KLAST;
            KLAST = K;
        }
        L += 1;
    }
    if NEMPTY > 0 as ::core::ffi::c_int {
        I = 1 as ::core::ffi::c_int;
        while I <= N {
            if *LEN.offset(I as isize) == 0 as ::core::ffi::c_int {
                K += 1;
                *LOC.offset(I as isize) = K;
                *IND.offset(K as isize) = 0 as ::core::ffi::c_int;
                ILAST = I;
            }
            I += 1;
        }
    }
    LPRINT = (*LUSOL).luparm[LUSOL_IP_PRINTLEVEL as usize];
    if LPRINT >= LUSOL_MSG_PIVOT {
        LUSOL_report(
            LUSOL,
            0 as ::core::ffi::c_int,
            b"lu1rec.  File compressed from %d to %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    (*LUSOL).luparm[LUSOL_IP_COMPRESSIONS_LU as usize] += 1;
    *LTOP = K;
    *IND.offset((*LTOP + 1 as ::core::ffi::c_int) as isize) = ILAST;
}
#[export_name="honest_lpsolve_LU1SLK"]
pub unsafe extern "C" fn LU1SLK(mut LUSOL: *mut LUSOLrec) {
    let mut J: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LQ: ::core::ffi::c_int = 0;
    let mut LQ1: ::core::ffi::c_int = 0;
    let mut LQ2: ::core::ffi::c_int = 0;
    J = 1 as ::core::ffi::c_int;
    while J <= (*LUSOL).n {
        *(*LUSOL).w.offset(J as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        J += 1;
    }
    LQ1 = if !(*LUSOL).iqloc.is_null() {
        *(*LUSOL).iqloc.offset(1 as ::core::ffi::c_int as isize)
    } else {
        (*LUSOL).n + 1 as ::core::ffi::c_int
    };
    LQ2 = (*LUSOL).n;
    if (*LUSOL).m > 1 as ::core::ffi::c_int {
        LQ2 = *(*LUSOL).iqloc.offset(2 as ::core::ffi::c_int as isize) - 1 as ::core::ffi::c_int;
    }
    LQ = LQ1;
    while LQ <= LQ2 {
        J = *(*LUSOL).iq.offset(LQ as isize);
        LC1 = *(*LUSOL).locc.offset(J as isize);
        if fabs(*(*LUSOL).a.offset(LC1 as isize))
            == 1 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *(*LUSOL).w.offset(J as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        LQ += 1;
    }
}
#[export_name="honest_lpsolve_LU1GAU"]
pub unsafe extern "C" fn LU1GAU(
    mut LUSOL: *mut LUSOLrec,
    mut MELIM: ::core::ffi::c_int,
    mut NSPARE: ::core::ffi::c_int,
    mut SMALL: ::core::ffi::c_double,
    mut LPIVC1: ::core::ffi::c_int,
    mut LPIVC2: ::core::ffi::c_int,
    mut LFIRST: *mut ::core::ffi::c_int,
    mut LPIVR2: ::core::ffi::c_int,
    mut LFREE: ::core::ffi::c_int,
    mut MINFRE: ::core::ffi::c_int,
    mut ILAST: ::core::ffi::c_int,
    mut JLAST: *mut ::core::ffi::c_int,
    mut LROW: *mut ::core::ffi::c_int,
    mut LCOL: *mut ::core::ffi::c_int,
    mut LU: *mut ::core::ffi::c_int,
    mut NFILL: *mut ::core::ffi::c_int,
    mut MARK: *mut ::core::ffi::c_int,
    mut AL: *mut ::core::ffi::c_double,
    mut MARKL: *mut ::core::ffi::c_int,
    mut AU: *mut ::core::ffi::c_double,
    mut IFILL: *mut ::core::ffi::c_int,
    mut JFILL: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut ATEND: ::core::ffi::c_uchar = 0;
    let mut LR: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut LENJ: ::core::ffi::c_int = 0;
    let mut NFREE: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LC2: ::core::ffi::c_int = 0;
    let mut NDONE: ::core::ffi::c_int = 0;
    let mut NDROP: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LL: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut LR1: ::core::ffi::c_int = 0;
    let mut LAST: ::core::ffi::c_int = 0;
    let mut LREP: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut LENI: ::core::ffi::c_int = 0;
    let mut UJ: ::core::ffi::c_double = 0.;
    let mut AIJ: ::core::ffi::c_double = 0.;
    LR = *LFIRST;
    loop {
        if !(LR <= LPIVR2) {
            current_block = 11869735117417356968;
            break;
        }
        J = *(*LUSOL).indr.offset(LR as isize);
        LENJ = *(*LUSOL).lenc.offset(J as isize);
        NFREE = LFREE - *LCOL;
        if NFREE < MINFRE {
            current_block = 1929036051270020115;
            break;
        }
        *LU += 1;
        UJ = *AU.offset(*LU as isize);
        LC1 = *(*LUSOL).locc.offset(J as isize);
        LC2 = LC1 + LENJ - 1 as ::core::ffi::c_int;
        ATEND = (J == *JLAST) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        NDONE = 0 as ::core::ffi::c_int;
        if !(LENJ == 0 as ::core::ffi::c_int) {
            NDROP = 0 as ::core::ffi::c_int;
            L = LC1;
            while L <= LC2 {
                I = *(*LUSOL).indc.offset(L as isize);
                LL = -*MARK.offset(I as isize);
                if LL > 0 as ::core::ffi::c_int {
                    NDONE += 1;
                    *MARKL.offset(LL as isize) = J;
                    *(*LUSOL).a.offset(L as isize) += *AL.offset(LL as isize) * UJ;
                    if fabs(*(*LUSOL).a.offset(L as isize)) <= SMALL {
                        NDROP += 1;
                    }
                }
                L += 1;
            }
            if !(NDROP == 0 as ::core::ffi::c_int) {
                K = LC1;
                L = LC1;
                while L <= LC2 {
                    I = *(*LUSOL).indc.offset(L as isize);
                    if fabs(*(*LUSOL).a.offset(L as isize)) <= SMALL {
                        LENJ -= 1;
                        let ref mut fresh1 = *(*LUSOL).lenr.offset(I as isize);
                        *fresh1 -= 1;
                        LR1 = *(*LUSOL).locr.offset(I as isize);
                        LAST = LR1 + *(*LUSOL).lenr.offset(I as isize);
                        LREP = LR1;
                        while LREP <= LAST {
                            if *(*LUSOL).indr.offset(LREP as isize) == J {
                                break;
                            }
                            LREP += 1;
                        }
                        *(*LUSOL).indr.offset(LREP as isize) = *(*LUSOL).indr.offset(LAST as isize);
                        *(*LUSOL).indr.offset(LAST as isize) = 0 as ::core::ffi::c_int;
                        if I == ILAST {
                            *LROW -= 1;
                        }
                    } else {
                        *(*LUSOL).a.offset(K as isize) = *(*LUSOL).a.offset(L as isize);
                        *(*LUSOL).indc.offset(K as isize) = I;
                        K += 1;
                    }
                    L += 1;
                }
                memset(
                    (*LUSOL).indc.offset(K as isize) as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ((LC2 - K + 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                );
                if ATEND != 0 {
                    *LCOL = K - 1 as ::core::ffi::c_int;
                }
            }
        }
        if !(NDONE == MELIM) {
            if !(ATEND != 0) {
                LAST = LC1 + LENJ - 1 as ::core::ffi::c_int;
                L1 = LAST + 1 as ::core::ffi::c_int;
                L2 = LAST + MELIM - NDONE;
                if L2 >= *LCOL {
                    current_block = 12118498384613381784;
                } else {
                    L = L1;
                    loop {
                        if !(L <= L2) {
                            current_block = 3484393783448207349;
                            break;
                        }
                        if *(*LUSOL).indc.offset(L as isize) != 0 as ::core::ffi::c_int {
                            current_block = 12118498384613381784;
                            break;
                        }
                        L += 1;
                    }
                }
                match current_block {
                    3484393783448207349 => {}
                    _ => {
                        L1 = *LCOL + 1 as ::core::ffi::c_int;
                        L2 = *LCOL + NSPARE;
                        *LCOL = L2;
                        L = L1;
                        while L <= L2 {
                            *(*LUSOL).indc.offset(L as isize) = 0 as ::core::ffi::c_int;
                            L += 1;
                        }
                        ATEND = TRUE as ::core::ffi::c_uchar;
                        *JLAST = J;
                        L1 = LC1;
                        L2 = *LCOL;
                        LC1 = L2 + 1 as ::core::ffi::c_int;
                        *(*LUSOL).locc.offset(J as isize) = LC1;
                        L = L1;
                        while L <= LAST {
                            L2 += 1;
                            *(*LUSOL).a.offset(L2 as isize) = *(*LUSOL).a.offset(L as isize);
                            *(*LUSOL).indc.offset(L2 as isize) = *(*LUSOL).indc.offset(L as isize);
                            *(*LUSOL).indc.offset(L as isize) = 0 as ::core::ffi::c_int;
                            L += 1;
                        }
                        *LCOL = L2;
                    }
                }
            }
            LAST = LC1 + LENJ - 1 as ::core::ffi::c_int;
            LL = 0 as ::core::ffi::c_int;
            LC = LPIVC1;
            while LC <= LPIVC2 {
                LL += 1;
                if !(*MARKL.offset(LL as isize) == J) {
                    AIJ = *AL.offset(LL as isize) * UJ;
                    if !(fabs(AIJ) <= SMALL) {
                        LENJ += 1;
                        LAST += 1;
                        *(*LUSOL).a.offset(LAST as isize) = AIJ;
                        I = *(*LUSOL).indc.offset(LC as isize);
                        *(*LUSOL).indc.offset(LAST as isize) = I;
                        LENI = *(*LUSOL).lenr.offset(I as isize);
                        L = *(*LUSOL).locr.offset(I as isize) + LENI;
                        if L >= *LROW {
                            current_block = 3071460294893645972;
                        } else if *(*LUSOL).indr.offset(L as isize) > 0 as ::core::ffi::c_int {
                            current_block = 3071460294893645972;
                        } else {
                            *(*LUSOL).indr.offset(L as isize) = J;
                            *(*LUSOL).lenr.offset(I as isize) = LENI + 1 as ::core::ffi::c_int;
                            current_block = 10067844863897285902;
                        }
                        match current_block {
                            10067844863897285902 => {}
                            _ => {
                                if *IFILL.offset(LL as isize) == 0 as ::core::ffi::c_int {
                                    *NFILL += LENI + NSPARE;
                                }
                                if *JFILL.offset(*LU as isize) == 0 as ::core::ffi::c_int {
                                    *JFILL.offset(*LU as isize) = LENJ;
                                }
                                *NFILL += 1;
                                let ref mut fresh2 = *IFILL.offset(LL as isize);
                                *fresh2 += 1;
                                *(*LUSOL).indc.offset(LAST as isize) = (*LUSOL).m + I;
                            }
                        }
                    }
                }
                LC += 1;
            }
            if ATEND != 0 {
                *LCOL = LAST;
            }
        }
        *(*LUSOL).lenc.offset(J as isize) = LENJ;
        LR += 1;
    }
    match current_block {
        1929036051270020115 => {
            *LFIRST = LR;
            return;
        }
        _ => {
            *LFIRST = 0 as ::core::ffi::c_int;
            return;
        }
    };
}
#[export_name="honest_lpsolve_LU1MAR"]
pub unsafe extern "C" fn LU1MAR(
    mut LUSOL: *mut LUSOLrec,
    mut MAXMN: ::core::ffi::c_int,
    mut TCP: ::core::ffi::c_uchar,
    mut AIJTOL: ::core::ffi::c_double,
    mut LTOL: ::core::ffi::c_double,
    mut MAXCOL: ::core::ffi::c_int,
    mut MAXROW: ::core::ffi::c_int,
    mut IBEST: *mut ::core::ffi::c_int,
    mut JBEST: *mut ::core::ffi::c_int,
    mut MBEST: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut KBEST: ::core::ffi::c_int = 0;
    let mut NCOL: ::core::ffi::c_int = 0;
    let mut NROW: ::core::ffi::c_int = 0;
    let mut NZ1: ::core::ffi::c_int = 0;
    let mut NZ: ::core::ffi::c_int = 0;
    let mut LQ1: ::core::ffi::c_int = 0;
    let mut LQ2: ::core::ffi::c_int = 0;
    let mut LQ: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LC2: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LEN1: ::core::ffi::c_int = 0;
    let mut MERIT: ::core::ffi::c_int = 0;
    let mut LP1: ::core::ffi::c_int = 0;
    let mut LP2: ::core::ffi::c_int = 0;
    let mut LP: ::core::ffi::c_int = 0;
    let mut LR1: ::core::ffi::c_int = 0;
    let mut LR2: ::core::ffi::c_int = 0;
    let mut LR: ::core::ffi::c_int = 0;
    let mut ABEST: ::core::ffi::c_double = 0.;
    let mut LBEST: ::core::ffi::c_double = 0.;
    let mut AMAX: ::core::ffi::c_double = 0.;
    let mut AIJ: ::core::ffi::c_double = 0.;
    let mut CMAX: ::core::ffi::c_double = 0.;
    ABEST = ZERO as ::core::ffi::c_double;
    LBEST = ZERO as ::core::ffi::c_double;
    *IBEST = 0 as ::core::ffi::c_int;
    *MBEST = -(1 as ::core::ffi::c_int);
    KBEST = MAXMN + 1 as ::core::ffi::c_int;
    NCOL = 0 as ::core::ffi::c_int;
    NROW = 0 as ::core::ffi::c_int;
    NZ1 = 0 as ::core::ffi::c_int;
    NZ = 1 as ::core::ffi::c_int;
    's_30: while NZ <= MAXMN {
        if KBEST <= NZ1 {
            break;
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NCOL >= MAXCOL {
                current_block = 15110427245365641410;
            } else {
                current_block = 2868539653012386629;
            }
        } else {
            current_block = 2868539653012386629;
        }
        match current_block {
            2868539653012386629 => {
                if !(NZ > (*LUSOL).m) {
                    LQ1 = *(*LUSOL).iqloc.offset(NZ as isize);
                    LQ2 = (*LUSOL).n;
                    if NZ < (*LUSOL).m {
                        LQ2 = *(*LUSOL)
                            .iqloc
                            .offset((NZ + 1 as ::core::ffi::c_int) as isize)
                            - 1 as ::core::ffi::c_int;
                    }
                    LQ = LQ1;
                    while LQ <= LQ2 {
                        NCOL = NCOL + 1 as ::core::ffi::c_int;
                        J = *(*LUSOL).iq.offset(LQ as isize);
                        LC1 = *(*LUSOL).locc.offset(J as isize);
                        LC2 = LC1 + NZ1;
                        AMAX = fabs(*(*LUSOL).a.offset(LC1 as isize));
                        if TCP != 0 {
                            if AMAX < AIJTOL {
                                current_block = 7976072742316086414;
                            } else {
                                current_block = 5783071609795492627;
                            }
                        } else {
                            current_block = 5783071609795492627;
                        }
                        match current_block {
                            5783071609795492627 => {
                                LC = LC1;
                                while LC <= LC2 {
                                    I = *(*LUSOL).indc.offset(LC as isize);
                                    LEN1 =
                                        *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
                                    if !(LEN1 > KBEST) {
                                        if LC == LC1 {
                                            AIJ = AMAX;
                                            CMAX = ONE as ::core::ffi::c_double;
                                            current_block = 11385396242402735691;
                                        } else {
                                            AIJ = fabs(*(*LUSOL).a.offset(LC as isize));
                                            if TCP != 0 {
                                                if AIJ < AIJTOL {
                                                    current_block = 18317007320854588510;
                                                } else {
                                                    current_block = 17500079516916021833;
                                                }
                                            } else if AIJ * LTOL < AMAX {
                                                current_block = 18317007320854588510;
                                            } else {
                                                current_block = 17500079516916021833;
                                            }
                                            match current_block {
                                                18317007320854588510 => {}
                                                _ => {
                                                    CMAX = AMAX / AIJ;
                                                    current_block = 11385396242402735691;
                                                }
                                            }
                                        }
                                        match current_block {
                                            18317007320854588510 => {}
                                            _ => {
                                                MERIT = NZ1 * LEN1;
                                                if MERIT == *MBEST {
                                                    if LBEST
                                                        <= (*LUSOL).parmlu[LUSOL_RP_GAMMA as usize]
                                                        && CMAX
                                                            <= (*LUSOL).parmlu
                                                                [LUSOL_RP_GAMMA as usize]
                                                    {
                                                        if ABEST >= AIJ {
                                                            current_block = 18317007320854588510;
                                                        } else {
                                                            current_block = 7427571413727699167;
                                                        }
                                                    } else if LBEST <= CMAX {
                                                        current_block = 18317007320854588510;
                                                    } else {
                                                        current_block = 7427571413727699167;
                                                    }
                                                } else {
                                                    current_block = 7427571413727699167;
                                                }
                                                match current_block {
                                                    18317007320854588510 => {}
                                                    _ => {
                                                        *IBEST = I;
                                                        *JBEST = J;
                                                        KBEST = LEN1;
                                                        *MBEST = MERIT;
                                                        ABEST = AIJ;
                                                        LBEST = CMAX;
                                                        if NZ == 1 as ::core::ffi::c_int {
                                                            break 's_30;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    LC += 1;
                                }
                                if *IBEST > 0 as ::core::ffi::c_int {
                                    if NCOL >= MAXCOL {
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                        LQ += 1;
                    }
                }
            }
            _ => {}
        }
        if KBEST <= NZ {
            break;
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NROW >= MAXROW {
                current_block = 15748585554211589791;
            } else {
                current_block = 17747245473264231573;
            }
        } else {
            current_block = 17747245473264231573;
        }
        match current_block {
            17747245473264231573 => {
                if !(NZ > (*LUSOL).n) {
                    LP1 = *(*LUSOL).iploc.offset(NZ as isize);
                    LP2 = (*LUSOL).m;
                    if NZ < (*LUSOL).n {
                        LP2 = *(*LUSOL)
                            .iploc
                            .offset((NZ + 1 as ::core::ffi::c_int) as isize)
                            - 1 as ::core::ffi::c_int;
                    }
                    LP = LP1;
                    while LP <= LP2 {
                        NROW += 1;
                        I = *(*LUSOL).ip.offset(LP as isize);
                        LR1 = *(*LUSOL).locr.offset(I as isize);
                        LR2 = LR1 + NZ1;
                        LR = LR1;
                        while LR <= LR2 {
                            J = *(*LUSOL).indr.offset(LR as isize);
                            LEN1 = *(*LUSOL).lenc.offset(J as isize) - 1 as ::core::ffi::c_int;
                            if !(LEN1 > KBEST) {
                                LC1 = *(*LUSOL).locc.offset(J as isize);
                                LC2 = LC1 + LEN1;
                                AMAX = fabs(*(*LUSOL).a.offset(LC1 as isize));
                                LC = LC1;
                                while LC <= LC2 {
                                    if *(*LUSOL).indc.offset(LC as isize) == I {
                                        break;
                                    }
                                    LC += 1;
                                }
                                AIJ = fabs(*(*LUSOL).a.offset(LC as isize));
                                if TCP != 0 {
                                    if AIJ < AIJTOL {
                                        current_block = 1623252117315916725;
                                    } else {
                                        current_block = 9437375157805982253;
                                    }
                                } else {
                                    current_block = 9437375157805982253;
                                }
                                match current_block {
                                    1623252117315916725 => {}
                                    _ => {
                                        if LC == LC1 {
                                            CMAX = ONE as ::core::ffi::c_double;
                                            current_block = 2754258178208450300;
                                        } else {
                                            if TCP != 0 {
                                                current_block = 851619935621435220;
                                            } else if AIJ * LTOL < AMAX {
                                                current_block = 1623252117315916725;
                                            } else {
                                                current_block = 851619935621435220;
                                            }
                                            match current_block {
                                                1623252117315916725 => {}
                                                _ => {
                                                    CMAX = AMAX / AIJ;
                                                    current_block = 2754258178208450300;
                                                }
                                            }
                                        }
                                        match current_block {
                                            1623252117315916725 => {}
                                            _ => {
                                                MERIT = NZ1 * LEN1;
                                                if MERIT == *MBEST {
                                                    if LBEST
                                                        <= (*LUSOL).parmlu[LUSOL_RP_GAMMA as usize]
                                                        && CMAX
                                                            <= (*LUSOL).parmlu
                                                                [LUSOL_RP_GAMMA as usize]
                                                    {
                                                        if ABEST >= AIJ {
                                                            current_block = 1623252117315916725;
                                                        } else {
                                                            current_block = 10393716428851982524;
                                                        }
                                                    } else if LBEST <= CMAX {
                                                        current_block = 1623252117315916725;
                                                    } else {
                                                        current_block = 10393716428851982524;
                                                    }
                                                } else {
                                                    current_block = 10393716428851982524;
                                                }
                                                match current_block {
                                                    1623252117315916725 => {}
                                                    _ => {
                                                        *IBEST = I;
                                                        *JBEST = J;
                                                        *MBEST = MERIT;
                                                        KBEST = LEN1;
                                                        ABEST = AIJ;
                                                        LBEST = CMAX;
                                                        if NZ == 1 as ::core::ffi::c_int {
                                                            break 's_30;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            LR += 1;
                        }
                        if *IBEST > 0 as ::core::ffi::c_int {
                            if NROW >= MAXROW {
                                break;
                            }
                        }
                        LP += 1;
                    }
                }
            }
            _ => {}
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NROW >= MAXROW && NCOL >= MAXCOL {
                break;
            }
        }
        NZ1 = NZ;
        if *IBEST > 0 as ::core::ffi::c_int {
            KBEST = *MBEST / NZ1;
        }
        NZ += 1;
    }
}
#[export_name="honest_lpsolve_LU1MCP"]
pub unsafe extern "C" fn LU1MCP(
    mut LUSOL: *mut LUSOLrec,
    mut AIJTOL: ::core::ffi::c_double,
    mut IBEST: *mut ::core::ffi::c_int,
    mut JBEST: *mut ::core::ffi::c_int,
    mut MBEST: *mut ::core::ffi::c_int,
    mut HLEN: ::core::ffi::c_int,
    mut HA: *mut ::core::ffi::c_double,
    mut HJ: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut J: ::core::ffi::c_int = 0;
    let mut KHEAP: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LC2: ::core::ffi::c_int = 0;
    let mut LENJ: ::core::ffi::c_int = 0;
    let mut MAXCOL: ::core::ffi::c_int = 0;
    let mut NCOL: ::core::ffi::c_int = 0;
    let mut NZ1: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LEN1: ::core::ffi::c_int = 0;
    let mut MERIT: ::core::ffi::c_int = 0;
    let mut ABEST: ::core::ffi::c_double = 0.;
    let mut AIJ: ::core::ffi::c_double = 0.;
    let mut AMAX: ::core::ffi::c_double = 0.;
    let mut CMAX: ::core::ffi::c_double = 0.;
    let mut LBEST: ::core::ffi::c_double = 0.;
    ABEST = ZERO as ::core::ffi::c_double;
    LBEST = ZERO as ::core::ffi::c_double;
    *IBEST = 0 as ::core::ffi::c_int;
    *JBEST = *HJ.offset(1 as ::core::ffi::c_int as isize);
    LENJ = *(*LUSOL).lenc.offset(*JBEST as isize);
    *MBEST = LENJ * HLEN;
    MAXCOL = 40 as ::core::ffi::c_int;
    NCOL = 0 as ::core::ffi::c_int;
    KHEAP = 1 as ::core::ffi::c_int;
    's_30: while KHEAP <= HLEN {
        AMAX = *HA.offset(KHEAP as isize);
        if !(AMAX < AIJTOL) {
            NCOL += 1;
            J = *HJ.offset(KHEAP as isize);
            LENJ = *(*LUSOL).lenc.offset(J as isize);
            NZ1 = LENJ - 1 as ::core::ffi::c_int;
            LC1 = *(*LUSOL).locc.offset(J as isize);
            LC2 = LC1 + NZ1;
            LC = LC1;
            while LC <= LC2 {
                I = *(*LUSOL).indc.offset(LC as isize);
                LEN1 = *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
                MERIT = NZ1 * LEN1;
                if !(MERIT > *MBEST) {
                    if LC == LC1 {
                        AIJ = AMAX;
                        CMAX = ONE as ::core::ffi::c_double;
                        current_block = 1109700713171191020;
                    } else {
                        AIJ = fabs(*(*LUSOL).a.offset(LC as isize));
                        if AIJ < AIJTOL {
                            current_block = 13586036798005543211;
                        } else {
                            CMAX = AMAX / AIJ;
                            current_block = 1109700713171191020;
                        }
                    }
                    match current_block {
                        13586036798005543211 => {}
                        _ => {
                            if MERIT == *MBEST {
                                if LBEST <= (*LUSOL).parmlu[LUSOL_RP_GAMMA as usize]
                                    && CMAX <= (*LUSOL).parmlu[LUSOL_RP_GAMMA as usize]
                                {
                                    if ABEST >= AIJ {
                                        current_block = 13586036798005543211;
                                    } else {
                                        current_block = 3437258052017859086;
                                    }
                                } else if LBEST <= CMAX {
                                    current_block = 13586036798005543211;
                                } else {
                                    current_block = 3437258052017859086;
                                }
                            } else {
                                current_block = 3437258052017859086;
                            }
                            match current_block {
                                13586036798005543211 => {}
                                _ => {
                                    *IBEST = I;
                                    *JBEST = J;
                                    *MBEST = MERIT;
                                    ABEST = AIJ;
                                    LBEST = CMAX;
                                    if MERIT == 0 as ::core::ffi::c_int {
                                        break 's_30;
                                    }
                                }
                            }
                        }
                    }
                }
                LC += 1;
            }
            if NCOL >= MAXCOL {
                break;
            }
        }
        KHEAP += 1;
    }
}
#[export_name="honest_lpsolve_LU1MRP"]
pub unsafe extern "C" fn LU1MRP(
    mut LUSOL: *mut LUSOLrec,
    mut MAXMN: ::core::ffi::c_int,
    mut LTOL: ::core::ffi::c_double,
    mut MAXCOL: ::core::ffi::c_int,
    mut MAXROW: ::core::ffi::c_int,
    mut IBEST: *mut ::core::ffi::c_int,
    mut JBEST: *mut ::core::ffi::c_int,
    mut MBEST: *mut ::core::ffi::c_int,
    mut AMAXR: *mut ::core::ffi::c_double,
) {
    let mut current_block: u64;
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut KBEST: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LC2: ::core::ffi::c_int = 0;
    let mut LEN1: ::core::ffi::c_int = 0;
    let mut LP: ::core::ffi::c_int = 0;
    let mut LP1: ::core::ffi::c_int = 0;
    let mut LP2: ::core::ffi::c_int = 0;
    let mut LQ: ::core::ffi::c_int = 0;
    let mut LQ1: ::core::ffi::c_int = 0;
    let mut LQ2: ::core::ffi::c_int = 0;
    let mut LR: ::core::ffi::c_int = 0;
    let mut LR1: ::core::ffi::c_int = 0;
    let mut LR2: ::core::ffi::c_int = 0;
    let mut MERIT: ::core::ffi::c_int = 0;
    let mut NCOL: ::core::ffi::c_int = 0;
    let mut NROW: ::core::ffi::c_int = 0;
    let mut NZ: ::core::ffi::c_int = 0;
    let mut NZ1: ::core::ffi::c_int = 0;
    let mut ABEST: ::core::ffi::c_double = 0.;
    let mut AIJ: ::core::ffi::c_double = 0.;
    let mut AMAX: ::core::ffi::c_double = 0.;
    let mut ATOLI: ::core::ffi::c_double = 0.;
    let mut ATOLJ: ::core::ffi::c_double = 0.;
    ABEST = ZERO as ::core::ffi::c_double;
    *IBEST = 0 as ::core::ffi::c_int;
    KBEST = MAXMN + 1 as ::core::ffi::c_int;
    *MBEST = -(1 as ::core::ffi::c_int);
    NCOL = 0 as ::core::ffi::c_int;
    NROW = 0 as ::core::ffi::c_int;
    NZ1 = 0 as ::core::ffi::c_int;
    NZ = 1 as ::core::ffi::c_int;
    's_27: while NZ <= MAXMN {
        if KBEST <= NZ1 {
            break;
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NCOL >= MAXCOL {
                current_block = 998969450222430575;
            } else {
                current_block = 11812396948646013369;
            }
        } else {
            current_block = 11812396948646013369;
        }
        match current_block {
            11812396948646013369 => {
                if !(NZ > (*LUSOL).m) {
                    LQ1 = *(*LUSOL).iqloc.offset(NZ as isize);
                    LQ2 = (*LUSOL).n;
                    if NZ < (*LUSOL).m {
                        LQ2 = *(*LUSOL)
                            .iqloc
                            .offset((NZ + 1 as ::core::ffi::c_int) as isize)
                            - 1 as ::core::ffi::c_int;
                    }
                    LQ = LQ1;
                    while LQ <= LQ2 {
                        NCOL = NCOL + 1 as ::core::ffi::c_int;
                        J = *(*LUSOL).iq.offset(LQ as isize);
                        LC1 = *(*LUSOL).locc.offset(J as isize);
                        LC2 = LC1 + NZ1;
                        AMAX = fabs(*(*LUSOL).a.offset(LC1 as isize));
                        ATOLJ = AMAX / LTOL;
                        LC = LC1;
                        while LC <= LC2 {
                            I = *(*LUSOL).indc.offset(LC as isize);
                            LEN1 = *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
                            if !(LEN1 > KBEST) {
                                AIJ = fabs(*(*LUSOL).a.offset(LC as isize));
                                if !(AIJ < ATOLJ) {
                                    if !(AIJ * LTOL < *AMAXR.offset(I as isize)) {
                                        MERIT = NZ1 * LEN1;
                                        if MERIT == *MBEST {
                                            if ABEST >= AIJ {
                                                current_block = 13797916685926291137;
                                            } else {
                                                current_block = 14359455889292382949;
                                            }
                                        } else {
                                            current_block = 14359455889292382949;
                                        }
                                        match current_block {
                                            13797916685926291137 => {}
                                            _ => {
                                                *IBEST = I;
                                                *JBEST = J;
                                                KBEST = LEN1;
                                                *MBEST = MERIT;
                                                ABEST = AIJ;
                                                if NZ == 1 as ::core::ffi::c_int {
                                                    break 's_27;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            LC += 1;
                        }
                        if *IBEST > 0 as ::core::ffi::c_int {
                            if NCOL >= MAXCOL {
                                break;
                            }
                        }
                        LQ += 1;
                    }
                }
            }
            _ => {}
        }
        if KBEST <= NZ {
            break;
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NROW >= MAXROW {
                current_block = 5531478931527916286;
            } else {
                current_block = 3934796541983872331;
            }
        } else {
            current_block = 3934796541983872331;
        }
        match current_block {
            3934796541983872331 => {
                if !(NZ > (*LUSOL).n) {
                    LP1 = *(*LUSOL).iploc.offset(NZ as isize);
                    LP2 = (*LUSOL).m;
                    if NZ < (*LUSOL).n {
                        LP2 = *(*LUSOL)
                            .iploc
                            .offset((NZ + 1 as ::core::ffi::c_int) as isize)
                            - 1 as ::core::ffi::c_int;
                    }
                    LP = LP1;
                    while LP <= LP2 {
                        NROW = NROW + 1 as ::core::ffi::c_int;
                        I = *(*LUSOL).ip.offset(LP as isize);
                        LR1 = *(*LUSOL).locr.offset(I as isize);
                        LR2 = LR1 + NZ1;
                        ATOLI = *AMAXR.offset(I as isize) / LTOL;
                        LR = LR1;
                        while LR <= LR2 {
                            J = *(*LUSOL).indr.offset(LR as isize);
                            LEN1 = *(*LUSOL).lenc.offset(J as isize) - 1 as ::core::ffi::c_int;
                            if !(LEN1 > KBEST) {
                                LC1 = *(*LUSOL).locc.offset(J as isize);
                                LC2 = LC1 + LEN1;
                                AMAX = fabs(*(*LUSOL).a.offset(LC1 as isize));
                                LC = LC1;
                                while LC <= LC2 {
                                    if *(*LUSOL).indc.offset(LC as isize) == I {
                                        break;
                                    }
                                    LC += 1;
                                }
                                AIJ = fabs(*(*LUSOL).a.offset(LC as isize));
                                if !(AIJ < ATOLI) {
                                    if !(AIJ * LTOL < AMAX) {
                                        MERIT = NZ1 * LEN1;
                                        if MERIT == *MBEST {
                                            if ABEST >= AIJ {
                                                current_block = 13678349939556791712;
                                            } else {
                                                current_block = 5181772461570869434;
                                            }
                                        } else {
                                            current_block = 5181772461570869434;
                                        }
                                        match current_block {
                                            13678349939556791712 => {}
                                            _ => {
                                                *IBEST = I;
                                                *JBEST = J;
                                                KBEST = LEN1;
                                                *MBEST = MERIT;
                                                ABEST = AIJ;
                                                if NZ == 1 as ::core::ffi::c_int {
                                                    break 's_27;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            LR += 1;
                        }
                        if *IBEST > 0 as ::core::ffi::c_int {
                            if NROW >= MAXROW {
                                break;
                            }
                        }
                        LP += 1;
                    }
                }
            }
            _ => {}
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NROW >= MAXROW && NCOL >= MAXCOL {
                break;
            }
        }
        NZ1 = NZ;
        if *IBEST > 0 as ::core::ffi::c_int {
            KBEST = *MBEST / NZ1;
        }
        NZ += 1;
    }
}
#[export_name="honest_lpsolve_LU1MSP"]
pub unsafe extern "C" fn LU1MSP(
    mut LUSOL: *mut LUSOLrec,
    mut MAXMN: ::core::ffi::c_int,
    mut LTOL: ::core::ffi::c_double,
    mut MAXCOL: ::core::ffi::c_int,
    mut IBEST: *mut ::core::ffi::c_int,
    mut JBEST: *mut ::core::ffi::c_int,
    mut MBEST: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut KBEST: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LC2: ::core::ffi::c_int = 0;
    let mut LQ: ::core::ffi::c_int = 0;
    let mut LQ1: ::core::ffi::c_int = 0;
    let mut LQ2: ::core::ffi::c_int = 0;
    let mut MERIT: ::core::ffi::c_int = 0;
    let mut NCOL: ::core::ffi::c_int = 0;
    let mut NZ: ::core::ffi::c_int = 0;
    let mut NZ1: ::core::ffi::c_int = 0;
    let mut ABEST: ::core::ffi::c_double = 0.;
    let mut AIJ: ::core::ffi::c_double = 0.;
    let mut AMAX: ::core::ffi::c_double = 0.;
    let mut ATOLJ: ::core::ffi::c_double = 0.;
    ABEST = ZERO as ::core::ffi::c_double;
    *IBEST = 0 as ::core::ffi::c_int;
    *MBEST = -(1 as ::core::ffi::c_int);
    KBEST = MAXMN + 1 as ::core::ffi::c_int;
    NCOL = 0 as ::core::ffi::c_int;
    NZ1 = 0 as ::core::ffi::c_int;
    NZ = 1 as ::core::ffi::c_int;
    's_24: while NZ <= MAXMN {
        if KBEST <= NZ1 {
            break;
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NCOL >= MAXCOL {
                current_block = 9087588081680396315;
            } else {
                current_block = 13183875560443969876;
            }
        } else {
            current_block = 13183875560443969876;
        }
        match current_block {
            13183875560443969876 => {
                if !(NZ > (*LUSOL).m) {
                    LQ1 = *(*LUSOL).iqloc.offset(NZ as isize);
                    LQ2 = (*LUSOL).n;
                    if NZ < (*LUSOL).m {
                        LQ2 = *(*LUSOL)
                            .iqloc
                            .offset((NZ + 1 as ::core::ffi::c_int) as isize)
                            - 1 as ::core::ffi::c_int;
                    }
                    LQ = LQ1;
                    while LQ <= LQ2 {
                        NCOL += 1;
                        J = *(*LUSOL).iq.offset(LQ as isize);
                        LC1 = *(*LUSOL).locc.offset(J as isize);
                        LC2 = LC1 + NZ1;
                        AMAX = fabs(*(*LUSOL).a.offset(LC1 as isize));
                        ATOLJ = AMAX / LTOL;
                        LC = LC1;
                        while LC <= LC2 {
                            I = *(*LUSOL).indc.offset(LC as isize);
                            if !(I != J) {
                                if !(NZ1 > KBEST) {
                                    AIJ = fabs(*(*LUSOL).a.offset(LC as isize));
                                    if !(AIJ < ATOLJ) {
                                        MERIT = NZ1 * NZ1;
                                        if MERIT == *MBEST {
                                            if ABEST >= AIJ {
                                                current_block = 7175849428784450219;
                                            } else {
                                                current_block = 15925075030174552612;
                                            }
                                        } else {
                                            current_block = 15925075030174552612;
                                        }
                                        match current_block {
                                            7175849428784450219 => {}
                                            _ => {
                                                *IBEST = I;
                                                *JBEST = J;
                                                KBEST = NZ1;
                                                *MBEST = MERIT;
                                                ABEST = AIJ;
                                                if NZ == 1 as ::core::ffi::c_int {
                                                    break 's_24;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            LC += 1;
                        }
                        if *IBEST > 0 as ::core::ffi::c_int {
                            if NCOL >= MAXCOL {
                                break;
                            }
                        }
                        LQ += 1;
                    }
                }
            }
            _ => {}
        }
        if *IBEST > 0 as ::core::ffi::c_int {
            if NCOL >= MAXCOL {
                break;
            }
        }
        NZ1 = NZ;
        if *IBEST > 0 as ::core::ffi::c_int {
            KBEST = *MBEST / NZ1;
        }
        NZ += 1;
    }
}
#[export_name="honest_lpsolve_LU1MXC"]
pub unsafe extern "C" fn LU1MXC(
    mut LUSOL: *mut LUSOLrec,
    mut K1: ::core::ffi::c_int,
    mut K2: ::core::ffi::c_int,
    mut IX: *mut ::core::ffi::c_int,
) {
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut LENJ: ::core::ffi::c_int = 0;
    let mut AMAX: ::core::ffi::c_double = 0.;
    K = K1;
    while K <= K2 {
        J = *IX.offset(K as isize);
        LC = *(*LUSOL).locc.offset(J as isize);
        LENJ = *(*LUSOL).lenc.offset(J as isize);
        if !(LENJ == 0 as ::core::ffi::c_int) {
            L = lps_idamax(
                *(*LUSOL).lenc.offset(J as isize),
                (*LUSOL)
                    .a
                    .offset(LC as isize)
                    .offset(-(LUSOL_ARRAYOFFSET as isize)),
                1 as ::core::ffi::c_int,
            ) + LC
                - 1 as ::core::ffi::c_int;
            if L > LC {
                AMAX = *(*LUSOL).a.offset(L as isize);
                *(*LUSOL).a.offset(L as isize) = *(*LUSOL).a.offset(LC as isize);
                *(*LUSOL).a.offset(LC as isize) = AMAX;
                I = *(*LUSOL).indc.offset(L as isize);
                *(*LUSOL).indc.offset(L as isize) = *(*LUSOL).indc.offset(LC as isize);
                *(*LUSOL).indc.offset(LC as isize) = I;
            }
        }
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU1MXR"]
pub unsafe extern "C" fn LU1MXR(
    mut LUSOL: *mut LUSOLrec,
    mut K1: ::core::ffi::c_int,
    mut K2: ::core::ffi::c_int,
    mut IX: *mut ::core::ffi::c_int,
    mut AMAXR: *mut ::core::ffi::c_double,
) {
    static mut I: ::core::ffi::c_int = 0;
    static mut J: *mut ::core::ffi::c_int =
        ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
    static mut IC: *mut ::core::ffi::c_int =
        ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
    static mut K: ::core::ffi::c_int = 0;
    static mut LC: ::core::ffi::c_int = 0;
    static mut LC1: ::core::ffi::c_int = 0;
    static mut LC2: ::core::ffi::c_int = 0;
    static mut LR: ::core::ffi::c_int = 0;
    static mut LR1: ::core::ffi::c_int = 0;
    static mut LR2: ::core::ffi::c_int = 0;
    static mut AMAX: ::core::ffi::c_double = 0.;
    K = K1;
    while K <= K2 {
        AMAX = ZERO as ::core::ffi::c_double;
        I = *IX.offset(K as isize);
        LR1 = *(*LUSOL).locr.offset(I as isize);
        LR2 = LR1 + *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
        LR = LR1;
        J = (*LUSOL).indr.offset(LR1 as isize);
        while LR <= LR2 {
            LC1 = *(*LUSOL).locc.offset(*J as isize);
            LC2 = LC1 + *(*LUSOL).lenc.offset(*J as isize);
            LC = LC1;
            IC = (*LUSOL).indc.offset(LC1 as isize);
            while LC < LC2 {
                if *IC == I {
                    break;
                }
                LC += 1;
                IC = IC.offset(1);
            }
            if AMAX < fabs(*(*LUSOL).a.offset(LC as isize)) {
                AMAX = fabs(*(*LUSOL).a.offset(LC as isize));
            }
            LR += 1;
            J = J.offset(1);
        }
        *AMAXR.offset(I as isize) = AMAX;
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU1FUL"]
pub unsafe extern "C" fn LU1FUL(
    mut LUSOL: *mut LUSOLrec,
    mut LEND: ::core::ffi::c_int,
    mut LU1: ::core::ffi::c_int,
    mut TPP: ::core::ffi::c_uchar,
    mut MLEFT: ::core::ffi::c_int,
    mut NLEFT: ::core::ffi::c_int,
    mut NRANK: ::core::ffi::c_int,
    mut NROWU: ::core::ffi::c_int,
    mut LENL: *mut ::core::ffi::c_int,
    mut LENU: *mut ::core::ffi::c_int,
    mut NSING: *mut ::core::ffi::c_int,
    mut KEEPLU: ::core::ffi::c_uchar,
    mut SMALL: ::core::ffi::c_double,
    mut D: *mut ::core::ffi::c_double,
    mut IPVT: *mut ::core::ffi::c_int,
) {
    let mut L: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut IPBASE: ::core::ffi::c_int = 0;
    let mut LDBASE: ::core::ffi::c_int = 0;
    let mut LQ: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LC2: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut LD: ::core::ffi::c_int = 0;
    let mut LKK: ::core::ffi::c_int = 0;
    let mut LKN: ::core::ffi::c_int = 0;
    let mut LU: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut IBEST: ::core::ffi::c_int = 0;
    let mut JBEST: ::core::ffi::c_int = 0;
    let mut LA: ::core::ffi::c_int = 0;
    let mut LL: ::core::ffi::c_int = 0;
    let mut NROWD: ::core::ffi::c_int = 0;
    let mut NCOLD: ::core::ffi::c_int = 0;
    let mut AI: ::core::ffi::c_double = 0.;
    let mut AJ: ::core::ffi::c_double = 0.;
    if NRANK < (*LUSOL).m {
        L = 1 as ::core::ffi::c_int;
        while L <= (*LUSOL).m {
            I = *(*LUSOL).ip.offset(L as isize);
            *(*LUSOL).ipinv.offset(I as isize) = L;
            L += 1;
        }
    }
    memset(
        D.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        '\0' as i32,
        (LEND as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    IPBASE = NROWU - 1 as ::core::ffi::c_int;
    LDBASE = 1 as ::core::ffi::c_int - NROWU;
    LQ = NROWU;
    while LQ <= (*LUSOL).n {
        J = *(*LUSOL).iq.offset(LQ as isize);
        LC1 = *(*LUSOL).locc.offset(J as isize);
        LC2 = LC1 + *(*LUSOL).lenc.offset(J as isize) - 1 as ::core::ffi::c_int;
        LC = LC1;
        while LC <= LC2 {
            I = *(*LUSOL).indc.offset(LC as isize);
            LD = LDBASE + *(*LUSOL).ipinv.offset(I as isize);
            *D.offset(LD as isize) = *(*LUSOL).a.offset(LC as isize);
            LC += 1;
        }
        LDBASE += MLEFT;
        LQ += 1;
    }
    if TPP != 0 {
        LU1DPP(
            LUSOL,
            D,
            MLEFT,
            MLEFT,
            NLEFT,
            SMALL,
            NSING,
            IPVT,
            (*LUSOL)
                .iq
                .offset(NROWU as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
        );
    } else {
        LU1DCP(
            LUSOL,
            D,
            MLEFT,
            MLEFT,
            NLEFT,
            SMALL,
            NSING,
            IPVT,
            (*LUSOL)
                .iq
                .offset(NROWU as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
        );
    }
    memcpy(
        (*LUSOL).a.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        D.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        (LEND as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    LKK = 1 as ::core::ffi::c_int;
    LKN = LEND - MLEFT + 1 as ::core::ffi::c_int;
    LU = LU1;
    K = 1 as ::core::ffi::c_int;
    while K <= (if MLEFT < NLEFT { MLEFT } else { NLEFT }) {
        L1 = IPBASE + K;
        L2 = IPBASE + *IPVT.offset(K as isize);
        if L1 != L2 {
            I = *(*LUSOL).ip.offset(L1 as isize);
            *(*LUSOL).ip.offset(L1 as isize) = *(*LUSOL).ip.offset(L2 as isize);
            *(*LUSOL).ip.offset(L2 as isize) = I;
        }
        IBEST = *(*LUSOL).ip.offset(L1 as isize);
        JBEST = *(*LUSOL).iq.offset(L1 as isize);
        if KEEPLU != 0 {
            LA = LKK;
            LL = LU;
            NROWD = 1 as ::core::ffi::c_int;
            I = K + 1 as ::core::ffi::c_int;
            while I <= MLEFT {
                LA += 1;
                AI = *(*LUSOL).a.offset(LA as isize);
                if fabs(AI) > SMALL {
                    NROWD = NROWD + 1 as ::core::ffi::c_int;
                    LL -= 1;
                    *(*LUSOL).a.offset(LL as isize) = AI;
                    *(*LUSOL).indc.offset(LL as isize) = *(*LUSOL).ip.offset((IPBASE + I) as isize);
                    *(*LUSOL).indr.offset(LL as isize) = IBEST;
                }
                I += 1;
            }
            LA = LKN + MLEFT;
            LU = LL;
            NCOLD = 0 as ::core::ffi::c_int;
            J = NLEFT;
            while J >= K {
                LA = LA - MLEFT;
                AJ = *(*LUSOL).a.offset(LA as isize);
                if fabs(AJ) > SMALL || J == K {
                    NCOLD += 1;
                    LU -= 1;
                    *(*LUSOL).a.offset(LU as isize) = AJ;
                    *(*LUSOL).indr.offset(LU as isize) = *(*LUSOL).iq.offset((IPBASE + J) as isize);
                }
                J -= 1;
            }
            *(*LUSOL).lenr.offset(IBEST as isize) = -NCOLD;
            *(*LUSOL).lenc.offset(JBEST as isize) = -NROWD;
            *LENL = *LENL + NROWD - 1 as ::core::ffi::c_int;
            *LENU = *LENU + NCOLD;
            LKN += 1;
        } else {
            *(*LUSOL).diagU.offset(JBEST as isize) = *(*LUSOL).a.offset(LKK as isize);
        }
        LKK += MLEFT + 1 as ::core::ffi::c_int;
        K += 1;
    }
}
#[export_name="honest_lpsolve_LU1OR1"]
pub unsafe extern "C" fn LU1OR1(
    mut LUSOL: *mut LUSOLrec,
    mut SMALL: ::core::ffi::c_double,
    mut AMAX: *mut ::core::ffi::c_double,
    mut NUMNZ: *mut ::core::ffi::c_int,
    mut LERR: *mut ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LDUMMY: ::core::ffi::c_int = 0;
    memset(
        (*LUSOL).lenr.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        '\0' as i32,
        ((*LUSOL).m as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    memset(
        (*LUSOL).lenc.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        '\0' as i32,
        ((*LUSOL).n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    *AMAX = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    *NUMNZ = (*LUSOL).nelem;
    L = (*LUSOL).nelem + 1 as ::core::ffi::c_int;
    LDUMMY = 1 as ::core::ffi::c_int;
    loop {
        if !(LDUMMY <= (*LUSOL).nelem) {
            current_block = 4956146061682418353;
            break;
        }
        L -= 1;
        if fabs(*(*LUSOL).a.offset(L as isize)) > SMALL {
            I = *(*LUSOL).indc.offset(L as isize);
            J = *(*LUSOL).indr.offset(L as isize);
            if *AMAX < fabs(*(*LUSOL).a.offset(L as isize)) {
                *AMAX = fabs(*(*LUSOL).a.offset(L as isize));
            }
            if I < 1 as ::core::ffi::c_int || I > (*LUSOL).m {
                current_block = 7125413649291604652;
                break;
            }
            if J < 1 as ::core::ffi::c_int || J > (*LUSOL).n {
                current_block = 7125413649291604652;
                break;
            }
            let ref mut fresh6 = *(*LUSOL).lenr.offset(I as isize);
            *fresh6 += 1;
            let ref mut fresh7 = *(*LUSOL).lenc.offset(J as isize);
            *fresh7 += 1;
        } else {
            *(*LUSOL).a.offset(L as isize) = *(*LUSOL).a.offset(*NUMNZ as isize);
            *(*LUSOL).indc.offset(L as isize) = *(*LUSOL).indc.offset(*NUMNZ as isize);
            *(*LUSOL).indr.offset(L as isize) = *(*LUSOL).indr.offset(*NUMNZ as isize);
            *NUMNZ -= 1;
        }
        LDUMMY += 1;
    }
    match current_block {
        7125413649291604652 => {
            *LERR = L;
            *INFORM = LUSOL_INFORM_LUSINGULAR;
            return;
        }
        _ => {
            *LERR = 0 as ::core::ffi::c_int;
            *INFORM = LUSOL_INFORM_LUSUCCESS;
            return;
        }
    };
}
#[export_name="honest_lpsolve_LU1OR2"]
pub unsafe extern "C" fn LU1OR2(mut LUSOL: *mut LUSOLrec) {
    let mut ACE: ::core::ffi::c_double = 0.;
    let mut ACEP: ::core::ffi::c_double = 0.;
    let mut L: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut JCE: ::core::ffi::c_int = 0;
    let mut ICE: ::core::ffi::c_int = 0;
    let mut ICEP: ::core::ffi::c_int = 0;
    let mut JCEP: ::core::ffi::c_int = 0;
    let mut JA: ::core::ffi::c_int = 0;
    let mut JB: ::core::ffi::c_int = 0;
    L = 1 as ::core::ffi::c_int;
    J = 1 as ::core::ffi::c_int;
    while J <= (*LUSOL).n {
        *(*LUSOL).locc.offset(J as isize) = L;
        L += *(*LUSOL).lenc.offset(J as isize);
        J += 1;
    }
    I = 1 as ::core::ffi::c_int;
    while I <= (*LUSOL).nelem {
        JCE = *(*LUSOL).indr.offset(I as isize);
        if !(JCE == 0 as ::core::ffi::c_int) {
            ACE = *(*LUSOL).a.offset(I as isize);
            ICE = *(*LUSOL).indc.offset(I as isize);
            *(*LUSOL).indr.offset(I as isize) = 0 as ::core::ffi::c_int;
            J = 1 as ::core::ffi::c_int;
            while J <= (*LUSOL).nelem {
                L = *(*LUSOL).locc.offset(JCE as isize);
                let ref mut fresh5 = *(*LUSOL).locc.offset(JCE as isize);
                *fresh5 += 1;
                ACEP = *(*LUSOL).a.offset(L as isize);
                ICEP = *(*LUSOL).indc.offset(L as isize);
                JCEP = *(*LUSOL).indr.offset(L as isize);
                *(*LUSOL).a.offset(L as isize) = ACE;
                *(*LUSOL).indc.offset(L as isize) = ICE;
                *(*LUSOL).indr.offset(L as isize) = 0 as ::core::ffi::c_int;
                if JCEP == 0 as ::core::ffi::c_int {
                    break;
                }
                ACE = ACEP;
                ICE = ICEP;
                JCE = JCEP;
                J += 1;
            }
        }
        I += 1;
    }
    JA = 1 as ::core::ffi::c_int;
    J = 1 as ::core::ffi::c_int;
    while J <= (*LUSOL).n {
        JB = *(*LUSOL).locc.offset(J as isize);
        *(*LUSOL).locc.offset(J as isize) = JA;
        JA = JB;
        J += 1;
    }
}
#[export_name="honest_lpsolve_LU1OR3"]
pub unsafe extern "C" fn LU1OR3(
    mut LUSOL: *mut LUSOLrec,
    mut LERR: *mut ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut I: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    memset(
        (*LUSOL).ip.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        '\0' as i32,
        ((*LUSOL).m as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    J = 1 as ::core::ffi::c_int;
    's_7: loop {
        if !(J <= (*LUSOL).n) {
            current_block = 11650488183268122163;
            break;
        }
        if *(*LUSOL).lenc.offset(J as isize) > 0 as ::core::ffi::c_int {
            L1 = *(*LUSOL).locc.offset(J as isize);
            L2 = L1 + *(*LUSOL).lenc.offset(J as isize) - 1 as ::core::ffi::c_int;
            L = L1;
            while L <= L2 {
                I = *(*LUSOL).indc.offset(L as isize);
                if *(*LUSOL).ip.offset(I as isize) == J {
                    current_block = 3005070403850004692;
                    break 's_7;
                }
                *(*LUSOL).ip.offset(I as isize) = J;
                L += 1;
            }
        }
        J += 1;
    }
    match current_block {
        3005070403850004692 => {
            *LERR = L;
            *INFORM = LUSOL_INFORM_LUSINGULAR;
            return;
        }
        _ => {
            *INFORM = LUSOL_INFORM_LUSUCCESS;
            return;
        }
    };
}
#[export_name="honest_lpsolve_LU1OR4"]
pub unsafe extern "C" fn LU1OR4(mut LUSOL: *mut LUSOLrec) {
    let mut L: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut JDUMMY: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut LR: ::core::ffi::c_int = 0;
    L = 1 as ::core::ffi::c_int;
    I = 1 as ::core::ffi::c_int;
    while I <= (*LUSOL).m {
        L += *(*LUSOL).lenr.offset(I as isize);
        *(*LUSOL).locr.offset(I as isize) = L;
        I += 1;
    }
    L2 = (*LUSOL).nelem;
    J = (*LUSOL).n + 1 as ::core::ffi::c_int;
    JDUMMY = 1 as ::core::ffi::c_int;
    while JDUMMY <= (*LUSOL).n {
        J = J - 1 as ::core::ffi::c_int;
        if *(*LUSOL).lenc.offset(J as isize) > 0 as ::core::ffi::c_int {
            L1 = *(*LUSOL).locc.offset(J as isize);
            L = L1;
            while L <= L2 {
                I = *(*LUSOL).indc.offset(L as isize);
                LR = *(*LUSOL).locr.offset(I as isize) - 1 as ::core::ffi::c_int;
                *(*LUSOL).locr.offset(I as isize) = LR;
                *(*LUSOL).indr.offset(LR as isize) = J;
                L += 1;
            }
            L2 = L1 - 1 as ::core::ffi::c_int;
        }
        JDUMMY += 1;
    }
}
#[export_name="honest_lpsolve_LU1PEN"]
pub unsafe extern "C" fn LU1PEN(
    mut LUSOL: *mut LUSOLrec,
    mut NSPARE: ::core::ffi::c_int,
    mut ILAST: *mut ::core::ffi::c_int,
    mut LPIVC1: ::core::ffi::c_int,
    mut LPIVC2: ::core::ffi::c_int,
    mut LPIVR1: ::core::ffi::c_int,
    mut LPIVR2: ::core::ffi::c_int,
    mut LROW: *mut ::core::ffi::c_int,
    mut IFILL: *mut ::core::ffi::c_int,
    mut JFILL: *mut ::core::ffi::c_int,
) {
    let mut LL: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LR1: ::core::ffi::c_int = 0;
    let mut LR2: ::core::ffi::c_int = 0;
    let mut LR: ::core::ffi::c_int = 0;
    let mut LU: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LC2: ::core::ffi::c_int = 0;
    let mut LAST: ::core::ffi::c_int = 0;
    LL = 0 as ::core::ffi::c_int;
    LC = LPIVC1;
    while LC <= LPIVC2 {
        LL += 1;
        if !(*IFILL.offset(LL as isize) == 0 as ::core::ffi::c_int) {
            LC1 = *LROW + 1 as ::core::ffi::c_int;
            LC2 = *LROW + NSPARE;
            *LROW = LC2;
            L = LC1;
            while L <= LC2 {
                *(*LUSOL).indr.offset(L as isize) = 0 as ::core::ffi::c_int;
                L += 1;
            }
            I = *(*LUSOL).indc.offset(LC as isize);
            *ILAST = I;
            LR1 = *(*LUSOL).locr.offset(I as isize);
            LR2 = LR1 + *(*LUSOL).lenr.offset(I as isize) - 1 as ::core::ffi::c_int;
            *(*LUSOL).locr.offset(I as isize) = *LROW + 1 as ::core::ffi::c_int;
            LR = LR1;
            while LR <= LR2 {
                *LROW += 1;
                *(*LUSOL).indr.offset(*LROW as isize) = *(*LUSOL).indr.offset(LR as isize);
                *(*LUSOL).indr.offset(LR as isize) = 0 as ::core::ffi::c_int;
                LR += 1;
            }
            *LROW += *IFILL.offset(LL as isize);
        }
        LC += 1;
    }
    LU = 1 as ::core::ffi::c_int;
    LR = LPIVR1;
    while LR <= LPIVR2 {
        LU += 1;
        if !(*JFILL.offset(LU as isize) == 0 as ::core::ffi::c_int) {
            J = *(*LUSOL).indr.offset(LR as isize);
            LC1 = *(*LUSOL).locc.offset(J as isize) + *JFILL.offset(LU as isize)
                - 1 as ::core::ffi::c_int;
            LC2 = *(*LUSOL).locc.offset(J as isize) + *(*LUSOL).lenc.offset(J as isize)
                - 1 as ::core::ffi::c_int;
            LC = LC1;
            while LC <= LC2 {
                I = *(*LUSOL).indc.offset(LC as isize) - (*LUSOL).m;
                if I > 0 as ::core::ffi::c_int {
                    *(*LUSOL).indc.offset(LC as isize) = I;
                    LAST = *(*LUSOL).locr.offset(I as isize) + *(*LUSOL).lenr.offset(I as isize);
                    *(*LUSOL).indr.offset(LAST as isize) = J;
                    let ref mut fresh0 = *(*LUSOL).lenr.offset(I as isize);
                    *fresh0 += 1;
                }
                LC += 1;
            }
        }
        LR += 1;
    }
}
#[export_name="honest_lpsolve_LU1FAD"]
pub unsafe extern "C" fn LU1FAD(
    mut LUSOL: *mut LUSOLrec,
    mut INFORM: *mut ::core::ffi::c_int,
    mut LENL: *mut ::core::ffi::c_int,
    mut LENU: *mut ::core::ffi::c_int,
    mut MINLEN: *mut ::core::ffi::c_int,
    mut MERSUM: *mut ::core::ffi::c_int,
    mut NUTRI: *mut ::core::ffi::c_int,
    mut NLTRI: *mut ::core::ffi::c_int,
    mut NDENS1: *mut ::core::ffi::c_int,
    mut NDENS2: *mut ::core::ffi::c_int,
    mut NRANK: *mut ::core::ffi::c_int,
    mut LMAX: *mut ::core::ffi::c_double,
    mut UMAX: *mut ::core::ffi::c_double,
    mut DUMAX: *mut ::core::ffi::c_double,
    mut DUMIN: *mut ::core::ffi::c_double,
    mut AKMAX: *mut ::core::ffi::c_double,
) {
    let mut current_block: u64;
    let mut UTRI: ::core::ffi::c_uchar = 0;
    let mut LTRI: ::core::ffi::c_uchar = 0;
    let mut SPARS1: ::core::ffi::c_uchar = 0;
    let mut SPARS2: ::core::ffi::c_uchar = 0;
    let mut DENSE: ::core::ffi::c_uchar = 0;
    let mut DENSLU: ::core::ffi::c_uchar = 0;
    let mut KEEPLU: ::core::ffi::c_uchar = 0;
    let mut TCP: ::core::ffi::c_uchar = 0;
    let mut TPP: ::core::ffi::c_uchar = 0;
    let mut TRP: ::core::ffi::c_uchar = 0;
    let mut TSP: ::core::ffi::c_uchar = 0;
    let mut HLEN: ::core::ffi::c_int = 0;
    let mut HOPS: ::core::ffi::c_int = 0;
    let mut H: ::core::ffi::c_int = 0;
    let mut LPIV: ::core::ffi::c_int = 0;
    let mut LPRINT: ::core::ffi::c_int = 0;
    let mut MAXCOL: ::core::ffi::c_int = 0;
    let mut MAXROW: ::core::ffi::c_int = 0;
    let mut ILAST: ::core::ffi::c_int = 0;
    let mut JLAST: ::core::ffi::c_int = 0;
    let mut LFILE: ::core::ffi::c_int = 0;
    let mut LROW: ::core::ffi::c_int = 0;
    let mut LCOL: ::core::ffi::c_int = 0;
    let mut MINMN: ::core::ffi::c_int = 0;
    let mut MAXMN: ::core::ffi::c_int = 0;
    let mut NZLEFT: ::core::ffi::c_int = 0;
    let mut NSPARE: ::core::ffi::c_int = 0;
    let mut LU1: ::core::ffi::c_int = 0;
    let mut KK: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut LC: ::core::ffi::c_int = 0;
    let mut MLEFT: ::core::ffi::c_int = 0;
    let mut NLEFT: ::core::ffi::c_int = 0;
    let mut NROWU: ::core::ffi::c_int = 0;
    let mut LQ1: ::core::ffi::c_int = 0;
    let mut LQ2: ::core::ffi::c_int = 0;
    let mut JBEST: ::core::ffi::c_int = 0;
    let mut LQ: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut IBEST: ::core::ffi::c_int = 0;
    let mut MBEST: ::core::ffi::c_int = 0;
    let mut LEND: ::core::ffi::c_int = 0;
    let mut NFREE: ::core::ffi::c_int = 0;
    let mut LD: ::core::ffi::c_int = 0;
    let mut NCOLD: ::core::ffi::c_int = 0;
    let mut NROWD: ::core::ffi::c_int = 0;
    let mut MELIM: ::core::ffi::c_int = 0;
    let mut NELIM: ::core::ffi::c_int = 0;
    let mut JMAX: ::core::ffi::c_int = 0;
    let mut IMAX: ::core::ffi::c_int = 0;
    let mut LL1: ::core::ffi::c_int = 0;
    let mut LSAVE: ::core::ffi::c_int = 0;
    let mut LFREE: ::core::ffi::c_int = 0;
    let mut LIMIT: ::core::ffi::c_int = 0;
    let mut MINFRE: ::core::ffi::c_int = 0;
    let mut LPIVR: ::core::ffi::c_int = 0;
    let mut LPIVR1: ::core::ffi::c_int = 0;
    let mut LPIVR2: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut LPIVC: ::core::ffi::c_int = 0;
    let mut LPIVC1: ::core::ffi::c_int = 0;
    let mut LPIVC2: ::core::ffi::c_int = 0;
    let mut KBEST: ::core::ffi::c_int = 0;
    let mut LU: ::core::ffi::c_int = 0;
    let mut LR: ::core::ffi::c_int = 0;
    let mut LENJ: ::core::ffi::c_int = 0;
    let mut LC1: ::core::ffi::c_int = 0;
    let mut LAST: ::core::ffi::c_int = 0;
    let mut LL: ::core::ffi::c_int = 0;
    let mut LS: ::core::ffi::c_int = 0;
    let mut LENI: ::core::ffi::c_int = 0;
    let mut LR1: ::core::ffi::c_int = 0;
    let mut LFIRST: ::core::ffi::c_int = 0;
    let mut NFILL: ::core::ffi::c_int = 0;
    let mut NZCHNG: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut MRANK: ::core::ffi::c_int = 0;
    let mut NSING: ::core::ffi::c_int = 0;
    let mut LIJ: ::core::ffi::c_double = 0.;
    let mut LTOL: ::core::ffi::c_double = 0.;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut USPACE: ::core::ffi::c_double = 0.;
    let mut DENS1: ::core::ffi::c_double = 0.;
    let mut DENS2: ::core::ffi::c_double = 0.;
    let mut AIJMAX: ::core::ffi::c_double = 0.;
    let mut AIJTOL: ::core::ffi::c_double = 0.;
    let mut AMAX: ::core::ffi::c_double = 0.;
    let mut ABEST: ::core::ffi::c_double = 0.;
    let mut DIAG: ::core::ffi::c_double = 0.;
    let mut V: ::core::ffi::c_double = 0.;
    let mut LENA2: ::core::ffi::c_int = (*LUSOL).lena;
    AIJMAX = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    AIJTOL = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    HLEN = 0 as ::core::ffi::c_int;
    JBEST = 0 as ::core::ffi::c_int;
    IBEST = 0 as ::core::ffi::c_int;
    MBEST = 0 as ::core::ffi::c_int;
    LEND = 0 as ::core::ffi::c_int;
    LD = 0 as ::core::ffi::c_int;
    LPRINT = (*LUSOL).luparm[LUSOL_IP_PRINTLEVEL as usize];
    MAXCOL = (*LUSOL).luparm[LUSOL_IP_MARKOWITZ_MAXCOL as usize];
    LPIV = (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize];
    KEEPLU = ((*LUSOL).luparm[LUSOL_IP_KEEPLU as usize] != FALSE) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    TPP = (LPIV == LUSOL_PIVMOD_TPP) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    TRP = (LPIV == LUSOL_PIVMOD_TRP) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    TCP = (LPIV == LUSOL_PIVMOD_TCP) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    TSP = (LPIV == LUSOL_PIVMOD_TSP) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    DENSLU = FALSE as ::core::ffi::c_uchar;
    MAXROW = MAXCOL - 1 as ::core::ffi::c_int;
    ILAST = (*LUSOL).m;
    JLAST = (*LUSOL).n;
    LFILE = (*LUSOL).nelem;
    LROW = (*LUSOL).nelem;
    LCOL = (*LUSOL).nelem;
    MINMN = if (*LUSOL).m < (*LUSOL).n {
        (*LUSOL).m
    } else {
        (*LUSOL).n
    };
    MAXMN = if (*LUSOL).m > (*LUSOL).n {
        (*LUSOL).m
    } else {
        (*LUSOL).n
    };
    NZLEFT = (*LUSOL).nelem;
    NSPARE = 1 as ::core::ffi::c_int;
    if KEEPLU != 0 {
        LU1 = LENA2 + 1 as ::core::ffi::c_int;
    } else {
        LU1 = LENA2 + 1 as ::core::ffi::c_int;
    }
    LTOL = (*LUSOL).parmlu[LUSOL_RP_FACTORMAX_Lij as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    USPACE = (*LUSOL).parmlu[LUSOL_RP_COMPSPACE_U as usize];
    DENS1 = (*LUSOL).parmlu[LUSOL_RP_MARKOWITZ_CONLY as usize];
    DENS2 = (*LUSOL).parmlu[LUSOL_RP_MARKOWITZ_DENSE as usize];
    UTRI = TRUE as ::core::ffi::c_uchar;
    LTRI = FALSE as ::core::ffi::c_uchar;
    SPARS1 = FALSE as ::core::ffi::c_uchar;
    SPARS2 = FALSE as ::core::ffi::c_uchar;
    DENSE = FALSE as ::core::ffi::c_uchar;
    if LTOL < 1.0001E+0f64 {
        LTOL = 1.0001E+0f64;
    }
    if DENS1 > DENS2 {
        DENS1 = DENS2;
    }
    *LMAX = ZERO as ::core::ffi::c_double;
    *UMAX = ZERO as ::core::ffi::c_double;
    *DUMAX = ZERO as ::core::ffi::c_double;
    *DUMIN = LUSOL_BIGNUM;
    if (*LUSOL).nelem == 0 as ::core::ffi::c_int {
        *DUMIN = ZERO as ::core::ffi::c_double;
    }
    *AKMAX = ZERO as ::core::ffi::c_double;
    HOPS = 0 as ::core::ffi::c_int;
    if TPP as ::core::ffi::c_int != 0 || TSP as ::core::ffi::c_int != 0 {
        AIJMAX = ZERO as ::core::ffi::c_double;
        AIJTOL = ZERO as ::core::ffi::c_double;
        HLEN = 1 as ::core::ffi::c_int;
    } else {
        LU1MXC(
            LUSOL,
            1 as ::core::ffi::c_int,
            (*LUSOL).n,
            (*LUSOL).iq as *mut ::core::ffi::c_int,
        );
        LU1SLK(LUSOL);
    }
    if TRP != 0 {
        LU1MXR(
            LUSOL,
            1 as ::core::ffi::c_int,
            (*LUSOL).m,
            (*LUSOL).ip as *mut ::core::ffi::c_int,
            (*LUSOL).amaxr as *mut ::core::ffi::c_double,
        );
    }
    if TCP != 0 {
        HLEN = 0 as ::core::ffi::c_int;
        KK = 1 as ::core::ffi::c_int;
        while KK <= (*LUSOL).n {
            HLEN += 1;
            J = *(*LUSOL).iq.offset(KK as isize);
            LC = *(*LUSOL).locc.offset(J as isize);
            *(*LUSOL).Ha.offset(HLEN as isize) = fabs(*(*LUSOL).a.offset(LC as isize));
            *(*LUSOL).Hj.offset(HLEN as isize) = J;
            *(*LUSOL).Hk.offset(J as isize) = HLEN;
            KK += 1;
        }
        HBUILD(
            (*LUSOL).Ha as *mut ::core::ffi::c_double,
            (*LUSOL).Hj as *mut ::core::ffi::c_int,
            (*LUSOL).Hk as *mut ::core::ffi::c_int,
            HLEN,
            &raw mut HOPS,
        );
    }
    MLEFT = (*LUSOL).m + 1 as ::core::ffi::c_int;
    NLEFT = (*LUSOL).n + 1 as ::core::ffi::c_int;
    NROWU = 1 as ::core::ffi::c_int;
    's_270: loop {
        if !(NROWU <= MINMN) {
            current_block = 2832009447382049603;
            break;
        }
        MLEFT -= 1;
        NLEFT -= 1;
        if *(*LUSOL).iploc.offset(1 as ::core::ffi::c_int as isize) > (*LUSOL).m {
            current_block = 2832009447382049603;
            break;
        }
        if TCP != 0 {
            AIJMAX = *(*LUSOL).Ha.offset(1 as ::core::ffi::c_int as isize);
            if *AKMAX < AIJMAX {
                *AKMAX = AIJMAX;
            }
            AIJTOL = AIJMAX / LTOL;
        }
        if UTRI != 0 {
            LQ1 = *(*LUSOL).iqloc.offset(1 as ::core::ffi::c_int as isize);
            LQ2 = (*LUSOL).n;
            if (*LUSOL).m > 1 as ::core::ffi::c_int {
                LQ2 = *(*LUSOL).iqloc.offset(2 as ::core::ffi::c_int as isize)
                    - 1 as ::core::ffi::c_int;
            }
            if LQ1 <= LQ2 {
                if TPP as ::core::ffi::c_int != 0 || TSP as ::core::ffi::c_int != 0 {
                    JBEST = *(*LUSOL).iq.offset(LQ1 as isize);
                } else {
                    JBEST = 0 as ::core::ffi::c_int;
                    LQ = LQ1;
                    while LQ <= LQ2 {
                        J = *(*LUSOL).iq.offset(LQ as isize);
                        if *(*LUSOL).w.offset(J as isize) > ZERO as ::core::ffi::c_double {
                            JBEST = J;
                            break;
                        } else {
                            LC = *(*LUSOL).locc.offset(J as isize);
                            AMAX = fabs(*(*LUSOL).a.offset(LC as isize));
                            if TRP != 0 {
                                I = *(*LUSOL).indc.offset(LC as isize);
                                AIJTOL = *(*LUSOL).amaxr.offset(I as isize) / LTOL;
                            }
                            if AMAX >= AIJTOL {
                                JBEST = J;
                                break;
                            } else {
                                LQ += 1;
                            }
                        }
                    }
                }
                if JBEST > 0 as ::core::ffi::c_int {
                    LC = *(*LUSOL).locc.offset(JBEST as isize);
                    IBEST = *(*LUSOL).indc.offset(LC as isize);
                    MBEST = 0 as ::core::ffi::c_int;
                    current_block = 2319049091803029761;
                } else {
                    current_block = 5722677567366458307;
                }
            } else {
                current_block = 5722677567366458307;
            }
            match current_block {
                2319049091803029761 => {}
                _ => {
                    if LPRINT >= LUSOL_MSG_PIVOT {
                        LUSOL_report(
                            LUSOL,
                            0 as ::core::ffi::c_int,
                            b"Utri ended.  spars1 = TRUE\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    UTRI = FALSE as ::core::ffi::c_uchar;
                    LTRI = TRUE as ::core::ffi::c_uchar;
                    SPARS1 = TRUE as ::core::ffi::c_uchar;
                    *NUTRI = NROWU - 1 as ::core::ffi::c_int;
                    if TPP as ::core::ffi::c_int != 0 || TSP as ::core::ffi::c_int != 0 {
                        LU1MXC(
                            LUSOL,
                            LQ1,
                            (*LUSOL).n,
                            (*LUSOL).iq as *mut ::core::ffi::c_int,
                        );
                    }
                    current_block = 7072655752890836508;
                }
            }
        } else {
            current_block = 7072655752890836508;
        }
        match current_block {
            7072655752890836508 => {
                if SPARS1 != 0 {
                    if TPP as ::core::ffi::c_int != 0 || TCP as ::core::ffi::c_int != 0 {
                        LU1MAR(
                            LUSOL,
                            MAXMN,
                            TCP,
                            AIJTOL,
                            LTOL,
                            MAXCOL,
                            MAXROW,
                            &raw mut IBEST,
                            &raw mut JBEST,
                            &raw mut MBEST,
                        );
                    } else if TRP != 0 {
                        LU1MRP(
                            LUSOL,
                            MAXMN,
                            LTOL,
                            MAXCOL,
                            MAXROW,
                            &raw mut IBEST,
                            &raw mut JBEST,
                            &raw mut MBEST,
                            (*LUSOL).amaxr as *mut ::core::ffi::c_double,
                        );
                    } else if TSP != 0 {
                        LU1MSP(
                            LUSOL,
                            MAXMN,
                            LTOL,
                            MAXCOL,
                            &raw mut IBEST,
                            &raw mut JBEST,
                            &raw mut MBEST,
                        );
                        if IBEST == 0 as ::core::ffi::c_int {
                            current_block = 17242067349324868145;
                            break;
                        }
                    }
                    if LTRI != 0 {
                        if MBEST > 0 as ::core::ffi::c_int {
                            LTRI = FALSE as ::core::ffi::c_uchar;
                            *NLTRI = NROWU - 1 as ::core::ffi::c_int - *NUTRI;
                            if LPRINT >= LUSOL_MSG_PIVOT {
                                LUSOL_report(
                                    LUSOL,
                                    0 as ::core::ffi::c_int,
                                    b"Ltri ended.\n\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                        }
                    } else if NZLEFT as ::core::ffi::c_double
                        >= DENS1 * MLEFT as ::core::ffi::c_double * NLEFT as ::core::ffi::c_double
                    {
                        SPARS1 = FALSE as ::core::ffi::c_uchar;
                        SPARS2 = TRUE as ::core::ffi::c_uchar;
                        *NDENS1 = NLEFT;
                        MAXROW = 0 as ::core::ffi::c_int;
                        if LPRINT >= LUSOL_MSG_PIVOT {
                            LUSOL_report(
                                LUSOL,
                                0 as ::core::ffi::c_int,
                                b"spars1 ended.  spars2 = TRUE\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                } else if SPARS2 as ::core::ffi::c_int != 0 || DENSE as ::core::ffi::c_int != 0 {
                    if TPP as ::core::ffi::c_int != 0 || TCP as ::core::ffi::c_int != 0 {
                        LU1MAR(
                            LUSOL,
                            MAXMN,
                            TCP,
                            AIJTOL,
                            LTOL,
                            MAXCOL,
                            MAXROW,
                            &raw mut IBEST,
                            &raw mut JBEST,
                            &raw mut MBEST,
                        );
                    } else if TRP != 0 {
                        LU1MRP(
                            LUSOL,
                            MAXMN,
                            LTOL,
                            MAXCOL,
                            MAXROW,
                            &raw mut IBEST,
                            &raw mut JBEST,
                            &raw mut MBEST,
                            (*LUSOL).amaxr as *mut ::core::ffi::c_double,
                        );
                    } else if TSP != 0 {
                        LU1MSP(
                            LUSOL,
                            MAXMN,
                            LTOL,
                            MAXCOL,
                            &raw mut IBEST,
                            &raw mut JBEST,
                            &raw mut MBEST,
                        );
                        if IBEST == 0 as ::core::ffi::c_int {
                            current_block = 7753837810170075804;
                            break;
                        }
                    }
                    if SPARS2 != 0 {
                        if NZLEFT as ::core::ffi::c_double
                            >= DENS2
                                * MLEFT as ::core::ffi::c_double
                                * NLEFT as ::core::ffi::c_double
                        {
                            SPARS2 = FALSE as ::core::ffi::c_uchar;
                            DENSE = TRUE as ::core::ffi::c_uchar;
                            *NDENS2 = NLEFT;
                            MAXCOL = 1 as ::core::ffi::c_int;
                            if LPRINT >= LUSOL_MSG_PIVOT {
                                LUSOL_report(
                                    LUSOL,
                                    0 as ::core::ffi::c_int,
                                    b"spars2 ended.  dense = TRUE\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                        }
                    }
                }
                if DENSE != 0 {
                    LEND = MLEFT * NLEFT;
                    NFREE = LU1 - 1 as ::core::ffi::c_int;
                    if NFREE >= 2 as ::core::ffi::c_int * LEND {
                        DENSLU = TRUE as ::core::ffi::c_uchar;
                        *NDENS2 = NLEFT;
                        LD = LU1 - LEND;
                        if LCOL >= LD {
                            LU1REC(
                                LUSOL,
                                (*LUSOL).n,
                                TRUE as ::core::ffi::c_uchar,
                                &raw mut LCOL,
                                (*LUSOL).indc as *mut ::core::ffi::c_int,
                                (*LUSOL).lenc as *mut ::core::ffi::c_int,
                                (*LUSOL).locc as *mut ::core::ffi::c_int,
                            );
                            LFILE = LCOL;
                            JLAST = *(*LUSOL)
                                .indc
                                .offset((LCOL + 1 as ::core::ffi::c_int) as isize);
                        }
                        current_block = 2832009447382049603;
                        break;
                    }
                }
            }
            _ => {}
        }
        NCOLD = *(*LUSOL).lenr.offset(IBEST as isize);
        NROWD = *(*LUSOL).lenc.offset(JBEST as isize);
        MELIM = NROWD - 1 as ::core::ffi::c_int;
        NELIM = NCOLD - 1 as ::core::ffi::c_int;
        *MERSUM += MBEST;
        *LENL += MELIM;
        *LENU += NCOLD;
        if LPRINT >= LUSOL_MSG_PIVOT {
            if NROWU == 1 as ::core::ffi::c_int {
                LUSOL_report(
                    LUSOL,
                    0 as ::core::ffi::c_int,
                    b"lu1fad debug:\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if TPP as ::core::ffi::c_int != 0
                || TRP as ::core::ffi::c_int != 0
                || TSP as ::core::ffi::c_int != 0
            {
                LUSOL_report(
                    LUSOL,
                    0 as ::core::ffi::c_int,
                    b"nrowu:%7d   i,jbest:%7d,%7d   nrowd,ncold:%6d,%6d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                JMAX = *(*LUSOL).Hj.offset(1 as ::core::ffi::c_int as isize);
                IMAX = *(*LUSOL)
                    .indc
                    .offset(*(*LUSOL).locc.offset(JMAX as isize) as isize);
                LUSOL_report(
                    LUSOL,
                    0 as ::core::ffi::c_int,
                    b"nrowu:%7d   i,jbest:%7d,%7d   nrowd,ncold:%6d,%6d   i,jmax:%7d,%7d   aijmax:%g\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        if KEEPLU == 0 {
            LU1 = LENA2 + 1 as ::core::ffi::c_int;
        }
        LL1 = LU1 - MELIM;
        LU1 = LL1 - NCOLD;
        LSAVE = LU1 - NROWD;
        LFREE = LSAVE - NCOLD;
        L = if KEEPLU as ::core::ffi::c_int != 0 {
            (if LROW > LCOL { LROW } else { LCOL })
                + 2 as ::core::ffi::c_int * ((*LUSOL).m + (*LUSOL).n)
        } else {
            0 as ::core::ffi::c_int
        };
        L *= LUSOL_MULT_nz_a;
        if L < NROWD * NCOLD {
            L = NROWD * NCOLD;
        }
        if L > LFREE - LCOL
            && LUSOL_expand_a(LUSOL, &raw mut L, &raw mut LFREE) as ::core::ffi::c_int != 0
        {
            LL1 += L;
            LU1 += L;
            LSAVE += L;
        }
        LIMIT = (USPACE * LFILE as ::core::ffi::c_double) as ::core::ffi::c_int
            + (*LUSOL).m
            + (*LUSOL).n
            + 1000 as ::core::ffi::c_int;
        MINFRE = NROWD * NCOLD;
        NFREE = LFREE - LCOL;
        if NFREE < MINFRE || LCOL > LIMIT {
            LU1REC(
                LUSOL,
                (*LUSOL).n,
                TRUE as ::core::ffi::c_uchar,
                &raw mut LCOL,
                (*LUSOL).indc as *mut ::core::ffi::c_int,
                (*LUSOL).lenc as *mut ::core::ffi::c_int,
                (*LUSOL).locc as *mut ::core::ffi::c_int,
            );
            LFILE = LCOL;
            JLAST = *(*LUSOL)
                .indc
                .offset((LCOL + 1 as ::core::ffi::c_int) as isize);
            NFREE = LFREE - LCOL;
            if NFREE < MINFRE {
                current_block = 16770362701224144535;
                break;
            }
        }
        MINFRE = NROWD * NCOLD;
        NFREE = LFREE - LROW;
        if NFREE < MINFRE || LROW > LIMIT {
            LU1REC(
                LUSOL,
                (*LUSOL).m,
                FALSE as ::core::ffi::c_uchar,
                &raw mut LROW,
                (*LUSOL).indr as *mut ::core::ffi::c_int,
                (*LUSOL).lenr as *mut ::core::ffi::c_int,
                (*LUSOL).locr as *mut ::core::ffi::c_int,
            );
            LFILE = LROW;
            ILAST = *(*LUSOL)
                .indr
                .offset((LROW + 1 as ::core::ffi::c_int) as isize);
            NFREE = LFREE - LROW;
            if NFREE < MINFRE {
                current_block = 16770362701224144535;
                break;
            }
        }
        LPIVR = *(*LUSOL).locr.offset(IBEST as isize);
        LPIVR1 = LPIVR + 1 as ::core::ffi::c_int;
        LPIVR2 = LPIVR + NELIM;
        L = LPIVR;
        while L <= LPIVR2 {
            if *(*LUSOL).indr.offset(L as isize) == JBEST {
                break;
            }
            L += 1;
        }
        *(*LUSOL).indr.offset(L as isize) = *(*LUSOL).indr.offset(LPIVR as isize);
        *(*LUSOL).indr.offset(LPIVR as isize) = JBEST;
        LPIVC = *(*LUSOL).locc.offset(JBEST as isize);
        LPIVC1 = LPIVC + 1 as ::core::ffi::c_int;
        LPIVC2 = LPIVC + MELIM;
        L = LPIVC;
        while L <= LPIVC2 {
            if *(*LUSOL).indc.offset(L as isize) == IBEST {
                break;
            }
            L += 1;
        }
        *(*LUSOL).indc.offset(L as isize) = *(*LUSOL).indc.offset(LPIVC as isize);
        *(*LUSOL).indc.offset(LPIVC as isize) = IBEST;
        ABEST = *(*LUSOL).a.offset(L as isize);
        *(*LUSOL).a.offset(L as isize) = *(*LUSOL).a.offset(LPIVC as isize);
        *(*LUSOL).a.offset(LPIVC as isize) = ABEST;
        if KEEPLU == 0 {
            *(*LUSOL).diagU.offset(JBEST as isize) = ABEST;
        }
        if TCP != 0 {
            KBEST = *(*LUSOL).Hk.offset(JBEST as isize);
            HDELETE(
                (*LUSOL).Ha as *mut ::core::ffi::c_double,
                (*LUSOL).Hj as *mut ::core::ffi::c_int,
                (*LUSOL).Hk as *mut ::core::ffi::c_int,
                &raw mut HLEN,
                KBEST,
                &raw mut H,
            );
            HOPS += H;
        }
        *(*LUSOL).a.offset(LU1 as isize) = ABEST;
        *(*LUSOL).indr.offset(LU1 as isize) = JBEST;
        *(*LUSOL).indc.offset(LU1 as isize) = NROWD;
        LU = LU1;
        DIAG = fabs(ABEST);
        if *UMAX < DIAG {
            *UMAX = DIAG;
        }
        if *DUMAX < DIAG {
            *DUMAX = DIAG;
        }
        if *DUMIN > DIAG {
            *DUMIN = DIAG;
        }
        LR = LPIVR1;
        while LR <= LPIVR2 {
            LU += 1;
            J = *(*LUSOL).indr.offset(LR as isize);
            LENJ = *(*LUSOL).lenc.offset(J as isize);
            *(*LUSOL).lenc.offset(J as isize) = LENJ - 1 as ::core::ffi::c_int;
            LC1 = *(*LUSOL).locc.offset(J as isize);
            LAST = LC1 + *(*LUSOL).lenc.offset(J as isize);
            L = LC1;
            while L <= LAST {
                if *(*LUSOL).indc.offset(L as isize) == IBEST {
                    break;
                }
                L += 1;
            }
            *(*LUSOL).a.offset(LU as isize) = *(*LUSOL).a.offset(L as isize);
            *(*LUSOL).indr.offset(LU as isize) = 0 as ::core::ffi::c_int;
            *(*LUSOL).indc.offset(LU as isize) = LENJ;
            if *UMAX < fabs(*(*LUSOL).a.offset(LU as isize)) {
                *UMAX = fabs(*(*LUSOL).a.offset(LU as isize));
            }
            *(*LUSOL).a.offset(L as isize) = *(*LUSOL).a.offset(LAST as isize);
            *(*LUSOL).indc.offset(L as isize) = *(*LUSOL).indc.offset(LAST as isize);
            *(*LUSOL).indc.offset(LAST as isize) = 0 as ::core::ffi::c_int;
            LR += 1;
        }
        *(*LUSOL).indc.offset(LSAVE as isize) = NCOLD;
        if !(MELIM == 0 as ::core::ffi::c_int) {
            LL = LL1 - 1 as ::core::ffi::c_int;
            LS = LSAVE;
            ABEST = ONE as ::core::ffi::c_double / ABEST;
            LC = LPIVC1;
            while LC <= LPIVC2 {
                LL += 1;
                LS += 1;
                I = *(*LUSOL).indc.offset(LC as isize);
                LENI = *(*LUSOL).lenr.offset(I as isize);
                *(*LUSOL).lenr.offset(I as isize) = LENI - 1 as ::core::ffi::c_int;
                LR1 = *(*LUSOL).locr.offset(I as isize);
                LAST = LR1 + *(*LUSOL).lenr.offset(I as isize);
                L = LR1;
                while L <= LAST {
                    if *(*LUSOL).indr.offset(L as isize) == JBEST {
                        break;
                    }
                    L += 1;
                }
                *(*LUSOL).indr.offset(L as isize) = *(*LUSOL).indr.offset(LAST as isize);
                *(*LUSOL).indr.offset(LAST as isize) = 0 as ::core::ffi::c_int;
                *(*LUSOL).a.offset(LL as isize) = -*(*LUSOL).a.offset(LC as isize) * ABEST;
                LIJ = fabs(*(*LUSOL).a.offset(LL as isize));
                if *LMAX < LIJ {
                    *LMAX = LIJ;
                }
                *(*LUSOL).indc.offset(LL as isize) = 0 as ::core::ffi::c_int;
                *(*LUSOL).indr.offset(LL as isize) = 0 as ::core::ffi::c_int;
                *(*LUSOL).indc.offset(LS as isize) = LENI;
                *(*LUSOL).indr.offset(LS as isize) = *(*LUSOL).iqloc.offset(I as isize);
                *(*LUSOL).iqloc.offset(I as isize) = LSAVE - LS;
                LC += 1;
            }
            if !(NELIM == 0 as ::core::ffi::c_int) {
                LFIRST = LPIVR1;
                MINFRE = MLEFT + NSPARE;
                LU = 1 as ::core::ffi::c_int;
                NFILL = 0 as ::core::ffi::c_int;
                loop {
                    LU1GAU(
                        LUSOL,
                        MELIM,
                        NSPARE,
                        SMALL,
                        LPIVC1,
                        LPIVC2,
                        &raw mut LFIRST,
                        LPIVR2,
                        LFREE,
                        MINFRE,
                        ILAST,
                        &raw mut JLAST,
                        &raw mut LROW,
                        &raw mut LCOL,
                        &raw mut LU,
                        &raw mut NFILL,
                        (*LUSOL).iqloc as *mut ::core::ffi::c_int,
                        (*LUSOL)
                            .a
                            .offset(LL1 as isize)
                            .offset(-(LUSOL_ARRAYOFFSET as isize)),
                        (*LUSOL)
                            .indc
                            .offset(LL1 as isize)
                            .offset(-(LUSOL_ARRAYOFFSET as isize)),
                        (*LUSOL)
                            .a
                            .offset(LU1 as isize)
                            .offset(-(LUSOL_ARRAYOFFSET as isize)),
                        (*LUSOL)
                            .indr
                            .offset(LL1 as isize)
                            .offset(-(LUSOL_ARRAYOFFSET as isize)),
                        (*LUSOL)
                            .indr
                            .offset(LU1 as isize)
                            .offset(-(LUSOL_ARRAYOFFSET as isize)),
                    );
                    if !(LFIRST > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    LU1REC(
                        LUSOL,
                        (*LUSOL).n,
                        TRUE as ::core::ffi::c_uchar,
                        &raw mut LCOL,
                        (*LUSOL).indc as *mut ::core::ffi::c_int,
                        (*LUSOL).lenc as *mut ::core::ffi::c_int,
                        (*LUSOL).locc as *mut ::core::ffi::c_int,
                    );
                    LFILE = LCOL;
                    JLAST = *(*LUSOL)
                        .indc
                        .offset((LCOL + 1 as ::core::ffi::c_int) as isize);
                    LPIVC = *(*LUSOL).locc.offset(JBEST as isize);
                    LPIVC1 = LPIVC + 1 as ::core::ffi::c_int;
                    LPIVC2 = LPIVC + MELIM;
                    NFREE = LFREE - LCOL;
                    if NFREE < MINFRE {
                        current_block = 16770362701224144535;
                        break 's_270;
                    }
                }
                if NFILL > 0 as ::core::ffi::c_int {
                    MINFRE = NFILL;
                    NFREE = LFREE - LROW;
                    if NFREE < MINFRE {
                        LU1REC(
                            LUSOL,
                            (*LUSOL).m,
                            FALSE as ::core::ffi::c_uchar,
                            &raw mut LROW,
                            (*LUSOL).indr as *mut ::core::ffi::c_int,
                            (*LUSOL).lenr as *mut ::core::ffi::c_int,
                            (*LUSOL).locr as *mut ::core::ffi::c_int,
                        );
                        LFILE = LROW;
                        ILAST = *(*LUSOL)
                            .indr
                            .offset((LROW + 1 as ::core::ffi::c_int) as isize);
                        LPIVR = *(*LUSOL).locr.offset(IBEST as isize);
                        LPIVR1 = LPIVR + 1 as ::core::ffi::c_int;
                        LPIVR2 = LPIVR + NELIM;
                        NFREE = LFREE - LROW;
                        if NFREE < MINFRE {
                            current_block = 16770362701224144535;
                            break;
                        }
                    }
                    LU1PEN(
                        LUSOL,
                        NSPARE,
                        &raw mut ILAST,
                        LPIVC1,
                        LPIVC2,
                        LPIVR1,
                        LPIVR2,
                        &raw mut LROW,
                        (*LUSOL)
                            .indr
                            .offset(LL1 as isize)
                            .offset(-(LUSOL_ARRAYOFFSET as isize)),
                        (*LUSOL)
                            .indr
                            .offset(LU1 as isize)
                            .offset(-(LUSOL_ARRAYOFFSET as isize)),
                    );
                }
            }
        }
        *(*LUSOL).lenr.offset(IBEST as isize) = 0 as ::core::ffi::c_int;
        *(*LUSOL).lenc.offset(JBEST as isize) = 0 as ::core::ffi::c_int;
        LL = LL1 - 1 as ::core::ffi::c_int;
        LS = LSAVE;
        LC = LPIVC1;
        while LC <= LPIVC2 {
            LL += 1;
            LS += 1;
            I = *(*LUSOL).indc.offset(LC as isize);
            *(*LUSOL).iqloc.offset(I as isize) = *(*LUSOL).indr.offset(LS as isize);
            *(*LUSOL).indc.offset(LL as isize) = I;
            *(*LUSOL).indr.offset(LL as isize) = IBEST;
            LC += 1;
        }
        LU = LU1 - 1 as ::core::ffi::c_int;
        LR = LPIVR;
        while LR <= LPIVR2 {
            LU += 1;
            *(*LUSOL).indr.offset(LU as isize) = *(*LUSOL).indr.offset(LR as isize);
            LR += 1;
        }
        LU1PQ2(
            LUSOL,
            NCOLD,
            &raw mut NZCHNG,
            (*LUSOL)
                .indr
                .offset(LPIVR as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
            (*LUSOL)
                .indc
                .offset(LU1 as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
            (*LUSOL).lenc as *mut ::core::ffi::c_int,
            (*LUSOL).iqloc as *mut ::core::ffi::c_int,
            (*LUSOL).iq as *mut ::core::ffi::c_int,
            (*LUSOL).iqinv as *mut ::core::ffi::c_int,
        );
        LU1PQ2(
            LUSOL,
            NROWD,
            &raw mut NZCHNG,
            (*LUSOL)
                .indc
                .offset(LPIVC as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
            (*LUSOL)
                .indc
                .offset(LSAVE as isize)
                .offset(-(LUSOL_ARRAYOFFSET as isize)),
            (*LUSOL).lenr as *mut ::core::ffi::c_int,
            (*LUSOL).iploc as *mut ::core::ffi::c_int,
            (*LUSOL).ip as *mut ::core::ffi::c_int,
            (*LUSOL).ipinv as *mut ::core::ffi::c_int,
        );
        NZLEFT += NZCHNG;
        if !(UTRI as ::core::ffi::c_int != 0 && TPP as ::core::ffi::c_int != 0) {
            if TRP as ::core::ffi::c_int != 0 && MELIM > 0 as ::core::ffi::c_int {
                LU1MXR(
                    LUSOL,
                    LL1,
                    LL,
                    (*LUSOL).indc as *mut ::core::ffi::c_int,
                    (*LUSOL).amaxr as *mut ::core::ffi::c_double,
                );
            }
            if NELIM > 0 as ::core::ffi::c_int {
                LU1MXC(
                    LUSOL,
                    LU1 + 1 as ::core::ffi::c_int,
                    LU,
                    (*LUSOL).indr as *mut ::core::ffi::c_int,
                );
                if TCP != 0 {
                    KK = LU1 + 1 as ::core::ffi::c_int;
                    while KK <= LU {
                        J = *(*LUSOL).indr.offset(KK as isize);
                        K = *(*LUSOL).Hk.offset(J as isize);
                        V = fabs(
                            *(*LUSOL)
                                .a
                                .offset(*(*LUSOL).locc.offset(J as isize) as isize),
                        );
                        HCHANGE(
                            (*LUSOL).Ha as *mut ::core::ffi::c_double,
                            (*LUSOL).Hj as *mut ::core::ffi::c_int,
                            (*LUSOL).Hk as *mut ::core::ffi::c_int,
                            HLEN,
                            K,
                            V,
                            J,
                            &raw mut H,
                        );
                        HOPS += H;
                        KK += 1;
                    }
                }
            }
        }
        *(*LUSOL).lenr.offset(IBEST as isize) = -NCOLD;
        *(*LUSOL).lenc.offset(JBEST as isize) = -NROWD;
        if LROW > LSAVE || LCOL > LSAVE {
            current_block = 1516748902077283103;
            break;
        }
        if IBEST == ILAST {
            LROW = *(*LUSOL).locr.offset(IBEST as isize);
        }
        if JBEST == JLAST {
            LCOL = *(*LUSOL).locc.offset(JBEST as isize);
        }
        NROWU += 1;
    }
    match current_block {
        2832009447382049603 => {
            *INFORM = LUSOL_INFORM_LUSUCCESS;
            LU1PQ3(
                LUSOL,
                (*LUSOL).m,
                (*LUSOL).lenr as *mut ::core::ffi::c_int,
                (*LUSOL).ip as *mut ::core::ffi::c_int,
                (*LUSOL).ipinv as *mut ::core::ffi::c_int,
                &raw mut MRANK,
            );
            LU1PQ3(
                LUSOL,
                (*LUSOL).n,
                (*LUSOL).lenc as *mut ::core::ffi::c_int,
                (*LUSOL).iq as *mut ::core::ffi::c_int,
                (*LUSOL).iqinv as *mut ::core::ffi::c_int,
                NRANK,
            );
            if *NRANK > MRANK {
                *NRANK = MRANK;
            }
            if DENSLU != 0 {
                LU1FUL(
                    LUSOL,
                    LEND,
                    LU1,
                    TPP,
                    MLEFT,
                    NLEFT,
                    *NRANK,
                    NROWU,
                    LENL,
                    LENU,
                    &raw mut NSING,
                    KEEPLU,
                    SMALL,
                    (*LUSOL)
                        .a
                        .offset(LD as isize)
                        .offset(-(LUSOL_ARRAYOFFSET as isize)),
                    (*LUSOL).locr as *mut ::core::ffi::c_int,
                );
                *NRANK = MINMN - NSING;
            }
            *MINLEN = *LENL + *LENU + 2 as ::core::ffi::c_int * ((*LUSOL).m + (*LUSOL).n);
        }
        16770362701224144535 => {
            *INFORM = LUSOL_INFORM_ANEEDMEM;
            *MINLEN = LENA2 + LFILE + 2 as ::core::ffi::c_int * ((*LUSOL).m + (*LUSOL).n);
        }
        1516748902077283103 => {
            *INFORM = LUSOL_INFORM_FATALERR;
        }
        7753837810170075804 => {
            *INFORM = LUSOL_INFORM_NOPIVOT;
        }
        _ => {}
    };
}
#[export_name="honest_lpsolve_LU1FAC"]
pub unsafe extern "C" fn LU1FAC(mut LUSOL: *mut LUSOLrec, mut INFORM: *mut ::core::ffi::c_int) {
    let mut current_block: u64;
    let mut KEEPLU: ::core::ffi::c_uchar = 0;
    let mut TPP: ::core::ffi::c_uchar = 0;
    let mut LPIV: ::core::ffi::c_int = 0;
    let mut NELEM0: ::core::ffi::c_int = 0;
    let mut LPRINT: ::core::ffi::c_int = 0;
    let mut MINLEN: ::core::ffi::c_int = 0;
    let mut NUML0: ::core::ffi::c_int = 0;
    let mut LENL: ::core::ffi::c_int = 0;
    let mut LENU: ::core::ffi::c_int = 0;
    let mut LROW: ::core::ffi::c_int = 0;
    let mut MERSUM: ::core::ffi::c_int = 0;
    let mut NUTRI: ::core::ffi::c_int = 0;
    let mut NLTRI: ::core::ffi::c_int = 0;
    let mut NDENS1: ::core::ffi::c_int = 0;
    let mut NDENS2: ::core::ffi::c_int = 0;
    let mut NRANK: ::core::ffi::c_int = 0;
    let mut NSING: ::core::ffi::c_int = 0;
    let mut JSING: ::core::ffi::c_int = 0;
    let mut JUMIN: ::core::ffi::c_int = 0;
    let mut NUMNZ: ::core::ffi::c_int = 0;
    let mut LERR: ::core::ffi::c_int = 0;
    let mut LU: ::core::ffi::c_int = 0;
    let mut LL: ::core::ffi::c_int = 0;
    let mut LM: ::core::ffi::c_int = 0;
    let mut LTOPL: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LENUK: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut LENLK: ::core::ffi::c_int = 0;
    let mut IDUMMY: ::core::ffi::c_int = 0;
    let mut LLSAVE: ::core::ffi::c_int = 0;
    let mut NMOVE: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut NCP: ::core::ffi::c_int = 0;
    let mut NBUMP: ::core::ffi::c_int = 0;
    let mut LMAX: ::core::ffi::c_double = 0.;
    let mut LTOL: ::core::ffi::c_double = 0.;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut AMAX: ::core::ffi::c_double = 0.;
    let mut UMAX: ::core::ffi::c_double = 0.;
    let mut DUMAX: ::core::ffi::c_double = 0.;
    let mut DUMIN: ::core::ffi::c_double = 0.;
    let mut AKMAX: ::core::ffi::c_double = 0.;
    let mut DM: ::core::ffi::c_double = 0.;
    let mut DN: ::core::ffi::c_double = 0.;
    let mut DELEM: ::core::ffi::c_double = 0.;
    let mut DENSTY: ::core::ffi::c_double = 0.;
    let mut AGRWTH: ::core::ffi::c_double = 0.;
    let mut UGRWTH: ::core::ffi::c_double = 0.;
    let mut GROWTH: ::core::ffi::c_double = 0.;
    let mut CONDU: ::core::ffi::c_double = 0.;
    let mut DINCR: ::core::ffi::c_double = 0.;
    let mut AVGMER: ::core::ffi::c_double = 0.;
    if !(*LUSOL).L0.is_null() {
        LUSOL_matfree(&raw mut (*LUSOL).L0);
    }
    NELEM0 = (*LUSOL).nelem;
    LPRINT = (*LUSOL).luparm[LUSOL_IP_PRINTLEVEL as usize];
    LPIV = (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize];
    KEEPLU = ((*LUSOL).luparm[LUSOL_IP_KEEPLU as usize] != FALSE) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    LTOL = (*LUSOL).parmlu[LUSOL_RP_FACTORMAX_Lij as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    TPP = (LPIV == LUSOL_PIVMOD_TPP) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    *INFORM = LUSOL_INFORM_LUSUCCESS;
    LERR = 0 as ::core::ffi::c_int;
    MINLEN = (*LUSOL).nelem + 2 as ::core::ffi::c_int * ((*LUSOL).m + (*LUSOL).n);
    NUML0 = 0 as ::core::ffi::c_int;
    LENL = 0 as ::core::ffi::c_int;
    LENU = 0 as ::core::ffi::c_int;
    LROW = 0 as ::core::ffi::c_int;
    MERSUM = 0 as ::core::ffi::c_int;
    NUTRI = (*LUSOL).m;
    NLTRI = 0 as ::core::ffi::c_int;
    NDENS1 = 0 as ::core::ffi::c_int;
    NDENS2 = 0 as ::core::ffi::c_int;
    NRANK = 0 as ::core::ffi::c_int;
    NSING = 0 as ::core::ffi::c_int;
    JSING = 0 as ::core::ffi::c_int;
    JUMIN = 0 as ::core::ffi::c_int;
    AMAX = ZERO as ::core::ffi::c_double;
    LMAX = ZERO as ::core::ffi::c_double;
    UMAX = ZERO as ::core::ffi::c_double;
    DUMAX = ZERO as ::core::ffi::c_double;
    DUMIN = ZERO as ::core::ffi::c_double;
    AKMAX = ZERO as ::core::ffi::c_double;
    DM = (*LUSOL).m as ::core::ffi::c_double;
    DN = (*LUSOL).n as ::core::ffi::c_double;
    DELEM = (*LUSOL).nelem as ::core::ffi::c_double;
    (*LUSOL).luparm[LUSOL_IP_COMPRESSIONS_LU as usize] = 0 as ::core::ffi::c_int;
    if (*LUSOL).lena < MINLEN {
        if LUSOL_realloc_a(LUSOL, MINLEN) == 0 {
            current_block = 7783087613731627617;
        } else {
            current_block = 2719512138335094285;
        }
    } else {
        current_block = 2719512138335094285;
    }
    match current_block {
        2719512138335094285 => {
            LU1OR1(
                LUSOL,
                SMALL,
                &raw mut AMAX,
                &raw mut NUMNZ,
                &raw mut LERR,
                INFORM,
            );
            if LPRINT >= LUSOL_MSG_STATISTICS {
                DENSTY = 100 as ::core::ffi::c_int as ::core::ffi::c_double * DELEM / (DM * DN);
                LUSOL_report(
                    LUSOL,
                    0 as ::core::ffi::c_int,
                    b"m:%6d %c n:%6d  nzcount:%9d  Amax:%g  Density:%g\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if *INFORM != LUSOL_INFORM_LUSUCCESS {
                *INFORM = LUSOL_INFORM_ADIMERR;
                if LPRINT >= LUSOL_MSG_SINGULARITY {
                    LUSOL_report(
                        LUSOL,
                        0 as ::core::ffi::c_int,
                        b"lu1fac  error...\nentry  a[%d]  has an illegal row (%d) or column (%d) index\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                current_block = 1208239530801147038;
            } else {
                (*LUSOL).nelem = NUMNZ;
                LU1OR2(LUSOL);
                LU1OR3(LUSOL, &raw mut LERR, INFORM);
                if *INFORM != LUSOL_INFORM_LUSUCCESS {
                    *INFORM = LUSOL_INFORM_ADUPLICATE;
                    if LPRINT >= LUSOL_MSG_SINGULARITY {
                        LUSOL_report(
                            LUSOL,
                            0 as ::core::ffi::c_int,
                            b"lu1fac  error...\nentry  a[%d]  is a duplicate with indeces indc=%d, indr=%d\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    current_block = 1208239530801147038;
                } else {
                    LU1OR4(LUSOL);
                    LU1PQ1(
                        LUSOL,
                        (*LUSOL).m,
                        (*LUSOL).n,
                        (*LUSOL).lenr as *mut ::core::ffi::c_int,
                        (*LUSOL).ip as *mut ::core::ffi::c_int,
                        (*LUSOL).iploc as *mut ::core::ffi::c_int,
                        (*LUSOL).ipinv as *mut ::core::ffi::c_int,
                        (*LUSOL).indc.offset((*LUSOL).nelem as isize),
                    );
                    LU1PQ1(
                        LUSOL,
                        (*LUSOL).n,
                        (*LUSOL).m,
                        (*LUSOL).lenc as *mut ::core::ffi::c_int,
                        (*LUSOL).iq as *mut ::core::ffi::c_int,
                        (*LUSOL).iqloc as *mut ::core::ffi::c_int,
                        (*LUSOL).iqinv as *mut ::core::ffi::c_int,
                        (*LUSOL).indc.offset((*LUSOL).nelem as isize),
                    );
                    LU1FAD(
                        LUSOL,
                        INFORM,
                        &raw mut LENL,
                        &raw mut LENU,
                        &raw mut MINLEN,
                        &raw mut MERSUM,
                        &raw mut NUTRI,
                        &raw mut NLTRI,
                        &raw mut NDENS1,
                        &raw mut NDENS2,
                        &raw mut NRANK,
                        &raw mut LMAX,
                        &raw mut UMAX,
                        &raw mut DUMAX,
                        &raw mut DUMIN,
                        &raw mut AKMAX,
                    );
                    (*LUSOL).luparm[LUSOL_IP_RANK_U as usize] = NRANK;
                    (*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize] = LENL;
                    if *INFORM == LUSOL_INFORM_ANEEDMEM {
                        current_block = 7783087613731627617;
                    } else {
                        if *INFORM == LUSOL_INFORM_NOPIVOT {
                            *INFORM = LUSOL_INFORM_NOPIVOT;
                            if LPRINT >= LUSOL_MSG_SINGULARITY {
                                LUSOL_report(
                                    LUSOL,
                                    0 as ::core::ffi::c_int,
                                    b"lu1fac  error...\nTSP used but diagonal pivot could not be found\n\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                        } else if *INFORM > LUSOL_INFORM_LUSUCCESS {
                            *INFORM = LUSOL_INFORM_FATALERR;
                            if LPRINT >= LUSOL_MSG_SINGULARITY {
                                LUSOL_report(
                                    LUSOL,
                                    0 as ::core::ffi::c_int,
                                    b"lu1fac  error...\nfatal bug   (sorry --- this should never happen)\n\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                        } else if KEEPLU != 0 {
                            LU = 0 as ::core::ffi::c_int;
                            LL = (*LUSOL).lena + 1 as ::core::ffi::c_int;
                            LM = LL;
                            LTOPL = LL - LENL - LENU;
                            LROW = LENU;
                            K = 1 as ::core::ffi::c_int;
                            while K <= NRANK {
                                I = *(*LUSOL).ip.offset(K as isize);
                                LENUK = -*(*LUSOL).lenr.offset(I as isize);
                                *(*LUSOL).lenr.offset(I as isize) = LENUK;
                                J = *(*LUSOL).iq.offset(K as isize);
                                LENLK =
                                    -*(*LUSOL).lenc.offset(J as isize) - 1 as ::core::ffi::c_int;
                                if LENLK > 0 as ::core::ffi::c_int {
                                    NUML0 += 1;
                                    *(*LUSOL).iqloc.offset(NUML0 as isize) = LENLK;
                                }
                                if LU + LENUK < LTOPL {
                                    IDUMMY = 1 as ::core::ffi::c_int;
                                    while IDUMMY <= LENLK {
                                        LL -= 1;
                                        LM -= 1;
                                        *(*LUSOL).a.offset(LL as isize) =
                                            *(*LUSOL).a.offset(LM as isize);
                                        *(*LUSOL).indc.offset(LL as isize) =
                                            *(*LUSOL).indc.offset(LM as isize);
                                        *(*LUSOL).indr.offset(LL as isize) =
                                            *(*LUSOL).indr.offset(LM as isize);
                                        IDUMMY += 1;
                                    }
                                } else {
                                    LLSAVE = LL - LENLK;
                                    NMOVE = LM - LTOPL;
                                    IDUMMY = 1 as ::core::ffi::c_int;
                                    while IDUMMY <= NMOVE {
                                        LL -= 1;
                                        LM -= 1;
                                        *(*LUSOL).a.offset(LL as isize) =
                                            *(*LUSOL).a.offset(LM as isize);
                                        *(*LUSOL).indc.offset(LL as isize) =
                                            *(*LUSOL).indc.offset(LM as isize);
                                        *(*LUSOL).indr.offset(LL as isize) =
                                            *(*LUSOL).indr.offset(LM as isize);
                                        IDUMMY += 1;
                                    }
                                    LTOPL = LL;
                                    LL = LLSAVE;
                                    LM = LL;
                                }
                                *(*LUSOL).locr.offset(I as isize) = LU + 1 as ::core::ffi::c_int;
                                L2 = LM - 1 as ::core::ffi::c_int;
                                LM = LM - LENUK;
                                L = LM;
                                while L <= L2 {
                                    LU = LU + 1 as ::core::ffi::c_int;
                                    *(*LUSOL).a.offset(LU as isize) =
                                        *(*LUSOL).a.offset(L as isize);
                                    *(*LUSOL).indr.offset(LU as isize) =
                                        *(*LUSOL).indr.offset(L as isize);
                                    L += 1;
                                }
                                K += 1;
                            }
                            K = 1 as ::core::ffi::c_int;
                            while K <= NUML0 {
                                *(*LUSOL).lenc.offset(K as isize) =
                                    *(*LUSOL).iqloc.offset(K as isize);
                                K += 1;
                            }
                            J = 1 as ::core::ffi::c_int;
                            while J <= (*LUSOL).n {
                                *(*LUSOL).locc.offset(J as isize) = 0 as ::core::ffi::c_int;
                                J += 1;
                            }
                            LU6CHK(LUSOL, 1 as ::core::ffi::c_int, (*LUSOL).lena, INFORM);
                            NSING = (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize];
                            JSING = (*LUSOL).luparm[LUSOL_IP_SINGULARINDEX as usize];
                            JUMIN = (*LUSOL).luparm[LUSOL_IP_COLINDEX_DUMIN as usize];
                            LMAX = (*LUSOL).parmlu[LUSOL_RP_MAXMULT_L as usize];
                            UMAX = (*LUSOL).parmlu[LUSOL_RP_MAXELEM_U as usize];
                            DUMAX = (*LUSOL).parmlu[LUSOL_RP_MAXELEM_DIAGU as usize];
                            DUMIN = (*LUSOL).parmlu[LUSOL_RP_MINELEM_DIAGU as usize];
                        } else {
                            LU6CHK(LUSOL, 1 as ::core::ffi::c_int, (*LUSOL).lena, INFORM);
                            NSING = (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize];
                            JSING = (*LUSOL).luparm[LUSOL_IP_SINGULARINDEX as usize];
                            JUMIN = (*LUSOL).luparm[LUSOL_IP_COLINDEX_DUMIN as usize];
                            DUMAX = (*LUSOL).parmlu[LUSOL_RP_MAXELEM_DIAGU as usize];
                            DUMIN = (*LUSOL).parmlu[LUSOL_RP_MINELEM_DIAGU as usize];
                        }
                        current_block = 1208239530801147038;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        7783087613731627617 => {
            *INFORM = LUSOL_INFORM_ANEEDMEM;
            if LPRINT >= LUSOL_MSG_SINGULARITY {
                LUSOL_report(
                    LUSOL,
                    0 as ::core::ffi::c_int,
                    b"lu1fac  error...\ninsufficient storage; increase  lena  from %d to at least %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    (*LUSOL).nelem = NELEM0;
    (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize] = NSING;
    (*LUSOL).luparm[LUSOL_IP_SINGULARINDEX as usize] = JSING;
    (*LUSOL).luparm[LUSOL_IP_MINIMUMLENA as usize] = MINLEN;
    (*LUSOL).luparm[LUSOL_IP_UPDATECOUNT as usize] = 0 as ::core::ffi::c_int;
    (*LUSOL).luparm[LUSOL_IP_RANK_U as usize] = NRANK;
    (*LUSOL).luparm[LUSOL_IP_COLCOUNT_DENSE1 as usize] = NDENS1;
    (*LUSOL).luparm[LUSOL_IP_COLCOUNT_DENSE2 as usize] = NDENS2;
    (*LUSOL).luparm[LUSOL_IP_COLINDEX_DUMIN as usize] = JUMIN;
    (*LUSOL).luparm[LUSOL_IP_COLCOUNT_L0 as usize] = NUML0;
    (*LUSOL).luparm[LUSOL_IP_ROWCOUNT_L0 as usize] = 0 as ::core::ffi::c_int;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_L0 as usize] = LENL;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_U0 as usize] = LENU;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize] = LENL;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_U as usize] = LENU;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_ROW as usize] = LROW;
    (*LUSOL).luparm[LUSOL_IP_MARKOWITZ_MERIT as usize] = MERSUM;
    (*LUSOL).luparm[LUSOL_IP_TRIANGROWS_U as usize] = NUTRI;
    (*LUSOL).luparm[LUSOL_IP_TRIANGROWS_L as usize] = NLTRI;
    (*LUSOL).parmlu[LUSOL_RP_MAXELEM_A as usize] = AMAX;
    (*LUSOL).parmlu[LUSOL_RP_MAXMULT_L as usize] = LMAX;
    (*LUSOL).parmlu[LUSOL_RP_MAXELEM_U as usize] = UMAX;
    (*LUSOL).parmlu[LUSOL_RP_MAXELEM_DIAGU as usize] = DUMAX;
    (*LUSOL).parmlu[LUSOL_RP_MINELEM_DIAGU as usize] = DUMIN;
    (*LUSOL).parmlu[LUSOL_RP_MAXELEM_TCP as usize] = AKMAX;
    AGRWTH = AKMAX / (AMAX + LUSOL_SMALLNUM);
    UGRWTH = UMAX / (AMAX + LUSOL_SMALLNUM);
    if TPP != 0 {
        GROWTH = UGRWTH;
    } else {
        GROWTH = AGRWTH;
    }
    (*LUSOL).parmlu[LUSOL_RP_GROWTHRATE as usize] = GROWTH;
    (*LUSOL).luparm[LUSOL_IP_FTRANCOUNT as usize] = 0 as ::core::ffi::c_int;
    (*LUSOL).luparm[LUSOL_IP_BTRANCOUNT as usize] = 0 as ::core::ffi::c_int;
    (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
    if *INFORM == LUSOL_INFORM_NOMEMLEFT {
        LUSOL_report(
            LUSOL,
            0 as ::core::ffi::c_int,
            b"lu1fac  error...\ninsufficient memory available\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    NCP = (*LUSOL).luparm[LUSOL_IP_COMPRESSIONS_LU as usize];
    CONDU = DUMAX
        / (if DUMIN > 1.0e-20f64 {
            DUMIN
        } else {
            1.0e-20f64
        });
    DINCR = (LENL + LENU - (*LUSOL).nelem) as ::core::ffi::c_double;
    DINCR = DINCR * 100 as ::core::ffi::c_int as ::core::ffi::c_double
        / (if DELEM > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            DELEM
        } else {
            1 as ::core::ffi::c_int as ::core::ffi::c_double
        });
    AVGMER = MERSUM as ::core::ffi::c_double;
    AVGMER = AVGMER / DM;
    NBUMP = (*LUSOL).m - NUTRI - NLTRI;
    if LPRINT >= LUSOL_MSG_STATISTICS {
        if TPP != 0 {
            LUSOL_report(
                LUSOL,
                0 as ::core::ffi::c_int,
                b"Merit %g %d %d %d %g %d %d %g %g %d %d %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            LUSOL_report(
                LUSOL,
                0 as ::core::ffi::c_int,
                b"Merit %s %g %d %d %d %g %d %d %g %g %d %d %d %g %g\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        LUSOL_report(
            LUSOL,
            0 as ::core::ffi::c_int,
            b"bump%9d  dense2%7d  DUmax%g DUmin%g  conDU%g\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
}
#[export_name="honest_lpsolve_LU7ADD"]
pub unsafe extern "C" fn LU7ADD(
    mut LUSOL: *mut LUSOLrec,
    mut JADD: ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut LENL: ::core::ffi::c_int,
    mut LENU: *mut ::core::ffi::c_int,
    mut LROW: *mut ::core::ffi::c_int,
    mut NRANK: ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
    mut KLAST: *mut ::core::ffi::c_int,
    mut VNORM: *mut ::core::ffi::c_double,
) {
    let mut current_block: u64;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut K: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LENI: ::core::ffi::c_int = 0;
    let mut MINFRE: ::core::ffi::c_int = 0;
    let mut NFREE: ::core::ffi::c_int = 0;
    let mut LR1: ::core::ffi::c_int = 0;
    let mut LR2: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    *VNORM = ZERO as ::core::ffi::c_double;
    *KLAST = 0 as ::core::ffi::c_int;
    K = 1 as ::core::ffi::c_int;
    loop {
        if !(K <= NRANK) {
            current_block = 16924917904204750491;
            break;
        }
        I = *(*LUSOL).ip.offset(K as isize);
        if !(fabs(*V.offset(I as isize)) <= SMALL) {
            *KLAST = K;
            *VNORM += fabs(*V.offset(I as isize));
            LENI = *(*LUSOL).lenr.offset(I as isize);
            MINFRE = LENI + 1 as ::core::ffi::c_int;
            NFREE = (*LUSOL).lena - LENL - *LROW;
            if NFREE < MINFRE {
                LU1REC(
                    LUSOL,
                    (*LUSOL).m,
                    TRUE as ::core::ffi::c_uchar,
                    LROW,
                    (*LUSOL).indr as *mut ::core::ffi::c_int,
                    (*LUSOL).lenr as *mut ::core::ffi::c_int,
                    (*LUSOL).locr as *mut ::core::ffi::c_int,
                );
                NFREE = (*LUSOL).lena - LENL - *LROW;
                if NFREE < MINFRE {
                    current_block = 3201846850007821067;
                    break;
                }
            }
            if LENI == 0 as ::core::ffi::c_int {
                *(*LUSOL).locr.offset(I as isize) = *LROW + 1 as ::core::ffi::c_int;
            }
            LR1 = *(*LUSOL).locr.offset(I as isize);
            LR2 = LR1 + LENI - 1 as ::core::ffi::c_int;
            if LR2 == *LROW {
                current_block = 950617515269983550;
            } else if *(*LUSOL)
                .indr
                .offset((LR2 + 1 as ::core::ffi::c_int) as isize)
                == 0 as ::core::ffi::c_int
            {
                current_block = 8634978415505398161;
            } else {
                *(*LUSOL).locr.offset(I as isize) = *LROW + 1 as ::core::ffi::c_int;
                L = LR2 - LR1 + 1 as ::core::ffi::c_int;
                if L > 0 as ::core::ffi::c_int {
                    LR2 = *LROW + 1 as ::core::ffi::c_int;
                    memmove(
                        (*LUSOL).a.offset(LR2 as isize) as *mut ::core::ffi::c_void,
                        (*LUSOL).a.offset(LR1 as isize) as *const ::core::ffi::c_void,
                        (L as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                            ),
                    );
                    memmove(
                        (*LUSOL).indr.offset(LR2 as isize) as *mut ::core::ffi::c_void,
                        (*LUSOL).indr.offset(LR1 as isize) as *const ::core::ffi::c_void,
                        (L as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                    );
                    memset(
                        (*LUSOL).indr.offset(LR1 as isize) as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (L as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                    );
                    *LROW += L;
                }
                current_block = 950617515269983550;
            }
            match current_block {
                950617515269983550 => {
                    LR2 = *LROW;
                    *LROW += 1;
                }
                _ => {}
            }
            LR2 += 1;
            *(*LUSOL).a.offset(LR2 as isize) = *V.offset(I as isize);
            *(*LUSOL).indr.offset(LR2 as isize) = JADD;
            *(*LUSOL).lenr.offset(I as isize) = LENI + 1 as ::core::ffi::c_int;
            *LENU += 1;
        }
        K += 1;
    }
    match current_block {
        3201846850007821067 => {
            *INFORM = LUSOL_INFORM_ANEEDMEM;
        }
        _ => {
            *INFORM = LUSOL_INFORM_LUSUCCESS;
        }
    };
}
#[export_name="honest_lpsolve_LU7CYC"]
pub unsafe extern "C" fn LU7CYC(
    mut LUSOL: *mut LUSOLrec,
    mut KFIRST: ::core::ffi::c_int,
    mut KLAST: ::core::ffi::c_int,
    mut IX: *mut ::core::ffi::c_int,
) {
    if KFIRST < KLAST {
        let mut IFIRST: ::core::ffi::c_int = 0;
        let mut K: ::core::ffi::c_int = 0;
        IFIRST = *IX.offset(KFIRST as isize);
        K = KLAST - KFIRST;
        memmove(
            IX.offset(KFIRST as isize) as *mut ::core::ffi::c_void,
            IX.offset(KFIRST as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (K as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        *IX.offset(KLAST as isize) = IFIRST;
    }
}
#[export_name="honest_lpsolve_LU7ELM"]
pub unsafe extern "C" fn LU7ELM(
    mut LUSOL: *mut LUSOLrec,
    mut JELM: ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut LENL: *mut ::core::ffi::c_int,
    mut LROW: *mut ::core::ffi::c_int,
    mut NRANK: ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
    mut DIAG: *mut ::core::ffi::c_double,
) {
    let mut current_block: u64;
    let mut VI: ::core::ffi::c_double = 0.;
    let mut VMAX: ::core::ffi::c_double = 0.;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut NRANK1: ::core::ffi::c_int = 0;
    let mut MINFRE: ::core::ffi::c_int = 0;
    let mut NFREE: ::core::ffi::c_int = 0;
    let mut KMAX: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LMAX: ::core::ffi::c_int = 0;
    let mut IMAX: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    LMAX = 0 as ::core::ffi::c_int;
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    NRANK1 = NRANK + 1 as ::core::ffi::c_int;
    *DIAG = ZERO as ::core::ffi::c_double;
    MINFRE = (*LUSOL).m - NRANK;
    NFREE = (*LUSOL).lena - *LENL - *LROW;
    if NFREE >= MINFRE {
        current_block = 8904642319578368429;
    } else {
        LU1REC(
            LUSOL,
            (*LUSOL).m,
            TRUE as ::core::ffi::c_uchar,
            LROW,
            (*LUSOL).indr as *mut ::core::ffi::c_int,
            (*LUSOL).lenr as *mut ::core::ffi::c_int,
            (*LUSOL).locr as *mut ::core::ffi::c_int,
        );
        NFREE = (*LUSOL).lena - *LENL - *LROW;
        if NFREE < MINFRE {
            *INFORM = LUSOL_INFORM_ANEEDMEM;
            current_block = 9007357115414505193;
        } else {
            current_block = 8904642319578368429;
        }
    }
    match current_block {
        8904642319578368429 => {
            VMAX = ZERO as ::core::ffi::c_double;
            KMAX = 0 as ::core::ffi::c_int;
            L = (*LUSOL).lena - *LENL + 1 as ::core::ffi::c_int;
            K = NRANK1;
            while K <= (*LUSOL).m {
                I = *(*LUSOL).ip.offset(K as isize);
                VI = fabs(*V.offset(I as isize));
                if !(VI <= SMALL) {
                    L -= 1;
                    *(*LUSOL).a.offset(L as isize) = *V.offset(I as isize);
                    *(*LUSOL).indc.offset(L as isize) = I;
                    if !(VMAX >= VI) {
                        VMAX = VI;
                        KMAX = K;
                        LMAX = L;
                    }
                }
                K += 1;
            }
            if KMAX == 0 as ::core::ffi::c_int {
                *INFORM = LUSOL_INFORM_LUSUCCESS;
            } else {
                IMAX = *(*LUSOL).ip.offset(KMAX as isize);
                VMAX = *(*LUSOL).a.offset(LMAX as isize);
                *(*LUSOL).a.offset(LMAX as isize) = *(*LUSOL).a.offset(L as isize);
                *(*LUSOL).indc.offset(LMAX as isize) = *(*LUSOL).indc.offset(L as isize);
                L1 = L + 1 as ::core::ffi::c_int;
                L2 = (*LUSOL).lena - *LENL;
                *LENL = *LENL + L2 - L;
                L = L1;
                while L <= L2 {
                    *(*LUSOL).a.offset(L as isize) /= -VMAX;
                    *(*LUSOL).indr.offset(L as isize) = IMAX;
                    L += 1;
                }
                *(*LUSOL).ip.offset(KMAX as isize) = *(*LUSOL).ip.offset(NRANK1 as isize);
                *(*LUSOL).ip.offset(NRANK1 as isize) = IMAX;
                *DIAG = VMAX;
                if JELM > 0 as ::core::ffi::c_int {
                    *LROW += 1;
                    *(*LUSOL).locr.offset(IMAX as isize) = *LROW;
                    *(*LUSOL).lenr.offset(IMAX as isize) = 1 as ::core::ffi::c_int;
                    *(*LUSOL).a.offset(*LROW as isize) = VMAX;
                    *(*LUSOL).indr.offset(*LROW as isize) = JELM;
                }
                *INFORM = LUSOL_INFORM_LUSINGULAR;
            }
        }
        _ => {}
    };
}
#[export_name="honest_lpsolve_LU7FOR"]
pub unsafe extern "C" fn LU7FOR(
    mut LUSOL: *mut LUSOLrec,
    mut KFIRST: ::core::ffi::c_int,
    mut KLAST: ::core::ffi::c_int,
    mut LENL: *mut ::core::ffi::c_int,
    mut LENU: *mut ::core::ffi::c_int,
    mut LROW: *mut ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
    mut DIAG: *mut ::core::ffi::c_double,
) {
    let mut current_block: u64;
    let mut SWAPPD: ::core::ffi::c_uchar = 0;
    let mut KBEGIN: ::core::ffi::c_int = 0;
    let mut IW: ::core::ffi::c_int = 0;
    let mut LENW: ::core::ffi::c_int = 0;
    let mut LW1: ::core::ffi::c_int = 0;
    let mut LW2: ::core::ffi::c_int = 0;
    let mut JFIRST: ::core::ffi::c_int = 0;
    let mut MINFRE: ::core::ffi::c_int = 0;
    let mut NFREE: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut J: ::core::ffi::c_int = 0;
    let mut KSTART: ::core::ffi::c_int = 0;
    let mut KSTOP: ::core::ffi::c_int = 0;
    let mut K: ::core::ffi::c_int = 0;
    let mut LFIRST: ::core::ffi::c_int = 0;
    let mut IV: ::core::ffi::c_int = 0;
    let mut LENV: ::core::ffi::c_int = 0;
    let mut LV1: ::core::ffi::c_int = 0;
    let mut JLAST: ::core::ffi::c_int = 0;
    let mut LV2: ::core::ffi::c_int = 0;
    let mut LV3: ::core::ffi::c_int = 0;
    let mut LV: ::core::ffi::c_int = 0;
    let mut JV: ::core::ffi::c_int = 0;
    let mut LW: ::core::ffi::c_int = 0;
    let mut LDIAG: ::core::ffi::c_int = 0;
    let mut LIMIT: ::core::ffi::c_int = 0;
    let mut AMULT: ::core::ffi::c_double = 0.;
    let mut LTOL: ::core::ffi::c_double = 0.;
    let mut USPACE: ::core::ffi::c_double = 0.;
    let mut SMALL: ::core::ffi::c_double = 0.;
    let mut VJ: ::core::ffi::c_double = 0.;
    let mut WJ: ::core::ffi::c_double = 0.;
    LTOL = (*LUSOL).parmlu[LUSOL_RP_UPDATEMAX_Lij as usize];
    SMALL = (*LUSOL).parmlu[LUSOL_RP_ZEROTOLERANCE as usize];
    USPACE = (*LUSOL).parmlu[LUSOL_RP_COMPSPACE_U as usize];
    KBEGIN = KFIRST;
    SWAPPD = FALSE as ::core::ffi::c_uchar;
    '_x100: loop {
        IW = *(*LUSOL).ip.offset(KLAST as isize);
        LENW = *(*LUSOL).lenr.offset(IW as isize);
        if LENW == 0 as ::core::ffi::c_int {
            current_block = 10011980152082598750;
            break;
        }
        LW1 = *(*LUSOL).locr.offset(IW as isize);
        LW2 = LW1 + LENW - 1 as ::core::ffi::c_int;
        JFIRST = *(*LUSOL).iq.offset(KBEGIN as isize);
        if !(KBEGIN >= KLAST) {
            MINFRE = (*LUSOL).n + 1 as ::core::ffi::c_int;
            NFREE = (*LUSOL).lena - *LENL - *LROW;
            if NFREE < MINFRE {
                LU1REC(
                    LUSOL,
                    (*LUSOL).m,
                    TRUE as ::core::ffi::c_uchar,
                    LROW,
                    (*LUSOL).indr as *mut ::core::ffi::c_int,
                    (*LUSOL).lenr as *mut ::core::ffi::c_int,
                    (*LUSOL).locr as *mut ::core::ffi::c_int,
                );
                LW1 = *(*LUSOL).locr.offset(IW as isize);
                LW2 = LW1 + LENW - 1 as ::core::ffi::c_int;
                NFREE = (*LUSOL).lena - *LENL - *LROW;
                if NFREE < MINFRE {
                    *INFORM = LUSOL_INFORM_ANEEDMEM;
                    current_block = 10468276026569382870;
                    break;
                }
            }
            L = LW1;
            while L <= LW2 {
                J = *(*LUSOL).indr.offset(L as isize);
                *(*LUSOL).locc.offset(J as isize) = L;
                L += 1;
            }
            KSTART = KBEGIN;
            KSTOP = if KLAST < (*LUSOL).n {
                KLAST
            } else {
                (*LUSOL).n
            };
            K = KSTART;
            while K <= KSTOP {
                JFIRST = *(*LUSOL).iq.offset(K as isize);
                LFIRST = *(*LUSOL).locc.offset(JFIRST as isize);
                if !(LFIRST == 0 as ::core::ffi::c_int) {
                    WJ = *(*LUSOL).a.offset(LFIRST as isize);
                    if !(K == KLAST) {
                        IV = *(*LUSOL).ip.offset(K as isize);
                        LENV = *(*LUSOL).lenr.offset(IV as isize);
                        LV1 = *(*LUSOL).locr.offset(IV as isize);
                        VJ = ZERO as ::core::ffi::c_double;
                        if LENV == 0 as ::core::ffi::c_int {
                            current_block = 7298750357305569613;
                        } else if *(*LUSOL).indr.offset(LV1 as isize) != JFIRST {
                            current_block = 7298750357305569613;
                        } else {
                            VJ = *(*LUSOL).a.offset(LV1 as isize);
                            if SWAPPD != 0 {
                                current_block = 3344612223147050765;
                            } else if LTOL * fabs(WJ) < fabs(VJ) {
                                current_block = 3344612223147050765;
                            } else if LTOL * fabs(VJ) < fabs(WJ) {
                                current_block = 7298750357305569613;
                            } else if LENV <= LENW {
                                current_block = 3344612223147050765;
                            } else {
                                current_block = 7298750357305569613;
                            }
                            match current_block {
                                7298750357305569613 => {}
                                _ => {
                                    *(*LUSOL).a.offset(LFIRST as isize) =
                                        *(*LUSOL).a.offset(LW2 as isize);
                                    JLAST = *(*LUSOL).indr.offset(LW2 as isize);
                                    *(*LUSOL).indr.offset(LFIRST as isize) = JLAST;
                                    *(*LUSOL).indr.offset(LW2 as isize) = 0 as ::core::ffi::c_int;
                                    *(*LUSOL).locc.offset(JLAST as isize) = LFIRST;
                                    *(*LUSOL).locc.offset(JFIRST as isize) =
                                        0 as ::core::ffi::c_int;
                                    LENW -= 1;
                                    *LENU -= 1;
                                    if *LROW == LW2 {
                                        *LROW -= 1;
                                    }
                                    LW2 = LW2 - 1 as ::core::ffi::c_int;
                                    if fabs(WJ) <= SMALL {
                                        current_block = 16684891175374439920;
                                    } else {
                                        AMULT = -WJ / VJ;
                                        L = (*LUSOL).lena - *LENL;
                                        *(*LUSOL).a.offset(L as isize) = AMULT;
                                        *(*LUSOL).indr.offset(L as isize) = IV;
                                        *(*LUSOL).indc.offset(L as isize) = IW;
                                        *LENL += 1;
                                        if LENV == 1 as ::core::ffi::c_int {
                                            current_block = 16684891175374439920;
                                        } else {
                                            LV2 = LV1 + 1 as ::core::ffi::c_int;
                                            LV3 = LV1 + LENV - 1 as ::core::ffi::c_int;
                                            if LW2 == *LROW {
                                                current_block = 15249426545244455969;
                                            } else {
                                                LV = LV2;
                                                loop {
                                                    if !(LV <= LV3) {
                                                        current_block = 16684891175374439920;
                                                        break;
                                                    }
                                                    JV = *(*LUSOL).indr.offset(LV as isize);
                                                    LW = *(*LUSOL).locc.offset(JV as isize);
                                                    if LW > 0 as ::core::ffi::c_int {
                                                        *(*LUSOL).a.offset(LW as isize) +=
                                                            AMULT * *(*LUSOL).a.offset(LV as isize);
                                                        if fabs(*(*LUSOL).a.offset(LW as isize))
                                                            <= SMALL
                                                        {
                                                            *(*LUSOL).a.offset(LW as isize) =
                                                                *(*LUSOL).a.offset(LW2 as isize);
                                                            J = *(*LUSOL).indr.offset(LW2 as isize);
                                                            *(*LUSOL).indr.offset(LW as isize) = J;
                                                            *(*LUSOL).indr.offset(LW2 as isize) =
                                                                0 as ::core::ffi::c_int;
                                                            *(*LUSOL).locc.offset(J as isize) = LW;
                                                            *(*LUSOL).locc.offset(JV as isize) =
                                                                0 as ::core::ffi::c_int;
                                                            *LENU -= 1;
                                                            LENW -= 1;
                                                            LW2 -= 1;
                                                        }
                                                    } else {
                                                        if *(*LUSOL).indr.offset(
                                                            (LW2 + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        ) != 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 1374326499789186306;
                                                            break;
                                                        }
                                                        *LENU += 1;
                                                        LENW += 1;
                                                        LW2 += 1;
                                                        *(*LUSOL).a.offset(LW2 as isize) =
                                                            AMULT * *(*LUSOL).a.offset(LV as isize);
                                                        *(*LUSOL).indr.offset(LW2 as isize) = JV;
                                                        *(*LUSOL).locc.offset(JV as isize) = LW2;
                                                    }
                                                    LV += 1;
                                                }
                                                match current_block {
                                                    16684891175374439920 => {}
                                                    _ => {
                                                        LV2 = LV;
                                                        *(*LUSOL).locr.offset(IW as isize) =
                                                            *LROW + 1 as ::core::ffi::c_int;
                                                        L = LW2 - LW1 + 1 as ::core::ffi::c_int;
                                                        if L > 0 as ::core::ffi::c_int {
                                                            let mut loci: ::core::ffi::c_int = 0;
                                                            let mut locp: *mut ::core::ffi::c_int =
                                                                ::core::ptr::null_mut::<
                                                                    ::core::ffi::c_int,
                                                                >(
                                                                );
                                                            loci = LW1;
                                                            locp =
                                                                (*LUSOL).indr.offset(LW1 as isize);
                                                            while loci <= LW2 {
                                                                *LROW += 1;
                                                                *(*LUSOL)
                                                                    .locc
                                                                    .offset(*locp as isize) = *LROW;
                                                                loci += 1;
                                                                locp = locp.offset(1);
                                                            }
                                                            LW2 =
                                                                *LROW - L + 1 as ::core::ffi::c_int;
                                                            memmove(
                                                                (*LUSOL).a.offset(LW2 as isize)
                                                                    as *mut ::core::ffi::c_void,
                                                                (*LUSOL).a.offset(LW1 as isize)
                                                                    as *const ::core::ffi::c_void,
                                                                (L as size_t).wrapping_mul(
                                                                    ::core::mem::size_of::<
                                                                        ::core::ffi::c_double,
                                                                    >(
                                                                    )
                                                                        as size_t,
                                                                ),
                                                            );
                                                            memmove(
                                                                (*LUSOL).indr.offset(LW2 as isize)
                                                                    as *mut ::core::ffi::c_void,
                                                                (*LUSOL).indr.offset(LW1 as isize)
                                                                    as *const ::core::ffi::c_void,
                                                                (L as size_t).wrapping_mul(
                                                                    ::core::mem::size_of::<
                                                                        ::core::ffi::c_int,
                                                                    >(
                                                                    )
                                                                        as size_t,
                                                                ),
                                                            );
                                                            memset(
                                                                (*LUSOL).indr.offset(LW1 as isize)
                                                                    as *mut ::core::ffi::c_void,
                                                                '\0' as i32,
                                                                (L as size_t).wrapping_mul(
                                                                    ::core::mem::size_of::<
                                                                        ::core::ffi::c_int,
                                                                    >(
                                                                    )
                                                                        as size_t,
                                                                ),
                                                            );
                                                        }
                                                        LW1 = *(*LUSOL).locr.offset(IW as isize);
                                                        LW2 = *LROW;
                                                        current_block = 15249426545244455969;
                                                    }
                                                }
                                            }
                                            match current_block {
                                                16684891175374439920 => {}
                                                _ => {
                                                    LV = LV2;
                                                    while LV <= LV3 {
                                                        JV = *(*LUSOL).indr.offset(LV as isize);
                                                        LW = *(*LUSOL).locc.offset(JV as isize);
                                                        if LW > 0 as ::core::ffi::c_int {
                                                            *(*LUSOL).a.offset(LW as isize) += AMULT
                                                                * *(*LUSOL).a.offset(LV as isize);
                                                            if fabs(*(*LUSOL).a.offset(LW as isize))
                                                                <= SMALL
                                                            {
                                                                *(*LUSOL).a.offset(LW as isize) =
                                                                    *(*LUSOL)
                                                                        .a
                                                                        .offset(LW2 as isize);
                                                                J = *(*LUSOL)
                                                                    .indr
                                                                    .offset(LW2 as isize);
                                                                *(*LUSOL)
                                                                    .indr
                                                                    .offset(LW as isize) = J;
                                                                *(*LUSOL)
                                                                    .indr
                                                                    .offset(LW2 as isize) =
                                                                    0 as ::core::ffi::c_int;
                                                                *(*LUSOL).locc.offset(J as isize) =
                                                                    LW;
                                                                *(*LUSOL)
                                                                    .locc
                                                                    .offset(JV as isize) =
                                                                    0 as ::core::ffi::c_int;
                                                                *LENU -= 1;
                                                                LENW -= 1;
                                                                LW2 -= 1;
                                                            }
                                                        } else {
                                                            *LENU += 1;
                                                            LENW += 1;
                                                            LW2 += 1;
                                                            *(*LUSOL).a.offset(LW2 as isize) = AMULT
                                                                * *(*LUSOL).a.offset(LV as isize);
                                                            *(*LUSOL).indr.offset(LW2 as isize) =
                                                                JV;
                                                            *(*LUSOL).locc.offset(JV as isize) =
                                                                LW2;
                                                        }
                                                        LV += 1;
                                                    }
                                                    *LROW = LW2;
                                                    current_block = 16684891175374439920;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            16684891175374439920 => {}
                            _ => {
                                *(*LUSOL).ip.offset(KLAST as isize) = IV;
                                *(*LUSOL).ip.offset(K as isize) = IW;
                                KBEGIN = K;
                                SWAPPD = TRUE as ::core::ffi::c_uchar;
                                break;
                            }
                        }
                    }
                }
                SWAPPD = FALSE as ::core::ffi::c_uchar;
                K += 1;
            }
            *(*LUSOL).lenr.offset(IW as isize) = LENW;
            if LENW == 0 as ::core::ffi::c_int {
                current_block = 10011980152082598750;
                break;
            }
            L = LW1;
            while L <= LW2 {
                J = *(*LUSOL).indr.offset(L as isize);
                *(*LUSOL).locc.offset(J as isize) = 0 as ::core::ffi::c_int;
                L += 1;
            }
        }
        L = LW1;
        loop {
            if !(L <= LW2) {
                current_block = 10011980152082598750;
                break '_x100;
            }
            LDIAG = L;
            if *(*LUSOL).indr.offset(L as isize) == JFIRST {
                break;
            }
            L += 1;
        }
        *DIAG = *(*LUSOL).a.offset(LDIAG as isize);
        *(*LUSOL).a.offset(LDIAG as isize) = *(*LUSOL).a.offset(LW1 as isize);
        *(*LUSOL).a.offset(LW1 as isize) = *DIAG;
        *(*LUSOL).indr.offset(LDIAG as isize) = *(*LUSOL).indr.offset(LW1 as isize);
        *(*LUSOL).indr.offset(LW1 as isize) = JFIRST;
        if SWAPPD != 0 {
            continue;
        }
        *INFORM = LUSOL_INFORM_LUSUCCESS;
        current_block = 16866181720925481797;
        break;
    }
    match current_block {
        10011980152082598750 => {
            *DIAG = ZERO as ::core::ffi::c_double;
            *INFORM = LUSOL_INFORM_LUSINGULAR;
            current_block = 16866181720925481797;
        }
        _ => {}
    }
    match current_block {
        16866181720925481797 => {
            LIMIT = (USPACE * *LENU as ::core::ffi::c_double) as ::core::ffi::c_int
                + (*LUSOL).m
                + (*LUSOL).n
                + 1000 as ::core::ffi::c_int;
            if *LROW > LIMIT {
                LU1REC(
                    LUSOL,
                    (*LUSOL).m,
                    TRUE as ::core::ffi::c_uchar,
                    LROW,
                    (*LUSOL).indr as *mut ::core::ffi::c_int,
                    (*LUSOL).lenr as *mut ::core::ffi::c_int,
                    (*LUSOL).locr as *mut ::core::ffi::c_int,
                );
            }
        }
        _ => {}
    };
}
#[export_name="honest_lpsolve_LU7RNK"]
pub unsafe extern "C" fn LU7RNK(
    mut LUSOL: *mut LUSOLrec,
    mut JSING: ::core::ffi::c_int,
    mut LENU: *mut ::core::ffi::c_int,
    mut LROW: *mut ::core::ffi::c_int,
    mut NRANK: *mut ::core::ffi::c_int,
    mut INFORM: *mut ::core::ffi::c_int,
    mut DIAG: *mut ::core::ffi::c_double,
) {
    let mut UTOL1: ::core::ffi::c_double = 0.;
    let mut UMAX: ::core::ffi::c_double = 0.;
    let mut IW: ::core::ffi::c_int = 0;
    let mut LENW: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut L2: ::core::ffi::c_int = 0;
    let mut LMAX: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    let mut JMAX: ::core::ffi::c_int = 0;
    let mut KMAX: ::core::ffi::c_int = 0;
    L1 = 0 as ::core::ffi::c_int;
    L2 = 0 as ::core::ffi::c_int;
    UTOL1 = (*LUSOL).parmlu[LUSOL_RP_SMALLDIAG_U as usize];
    *DIAG = ZERO as ::core::ffi::c_double;
    IW = *(*LUSOL).ip.offset(*NRANK as isize);
    LENW = *(*LUSOL).lenr.offset(IW as isize);
    if !(LENW == 0 as ::core::ffi::c_int) {
        L1 = *(*LUSOL).locr.offset(IW as isize);
        L2 = L1 + LENW - 1 as ::core::ffi::c_int;
        UMAX = ZERO as ::core::ffi::c_double;
        LMAX = L1;
        L = L1;
        while L <= L2 {
            if UMAX < fabs(*(*LUSOL).a.offset(L as isize)) {
                UMAX = fabs(*(*LUSOL).a.offset(L as isize));
                LMAX = L;
            }
            L += 1;
        }
        *DIAG = *(*LUSOL).a.offset(LMAX as isize);
        JMAX = *(*LUSOL).indr.offset(LMAX as isize);
        KMAX = *NRANK;
        while KMAX <= (*LUSOL).n {
            if *(*LUSOL).iq.offset(KMAX as isize) == JMAX {
                break;
            }
            KMAX += 1;
        }
        *(*LUSOL).iq.offset(KMAX as isize) = *(*LUSOL).iq.offset(*NRANK as isize);
        *(*LUSOL).iq.offset(*NRANK as isize) = JMAX;
        *(*LUSOL).a.offset(LMAX as isize) = *(*LUSOL).a.offset(L1 as isize);
        *(*LUSOL).a.offset(L1 as isize) = *DIAG;
        *(*LUSOL).indr.offset(LMAX as isize) = *(*LUSOL).indr.offset(L1 as isize);
        *(*LUSOL).indr.offset(L1 as isize) = JMAX;
        if !(UMAX <= UTOL1) {
            if !(JMAX == JSING) {
                *INFORM = LUSOL_INFORM_LUSUCCESS;
                return;
            }
        }
    }
    *INFORM = LUSOL_INFORM_RANKLOSS;
    *NRANK -= 1;
    if LENW > 0 as ::core::ffi::c_int {
        LENU = LENU.offset(-(LENW as isize));
        *(*LUSOL).lenr.offset(IW as isize) = 0 as ::core::ffi::c_int;
        L = L1;
        while L <= L2 {
            *(*LUSOL).indr.offset(L as isize) = 0 as ::core::ffi::c_int;
            L += 1;
        }
        if L2 == *LROW {
            L = 1 as ::core::ffi::c_int;
            while L <= L2 {
                if *(*LUSOL).indr.offset(*LROW as isize) > 0 as ::core::ffi::c_int {
                    break;
                }
                *LROW -= 1;
                L += 1;
            }
        }
    }
}
#[export_name="honest_lpsolve_LU7ZAP"]
pub unsafe extern "C" fn LU7ZAP(
    mut LUSOL: *mut LUSOLrec,
    mut JZAP: ::core::ffi::c_int,
    mut KZAP: *mut ::core::ffi::c_int,
    mut LENU: *mut ::core::ffi::c_int,
    mut LROW: *mut ::core::ffi::c_int,
    mut NRANK: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut K: ::core::ffi::c_int = 0;
    let mut I: ::core::ffi::c_int = 0;
    let mut LENI: ::core::ffi::c_int = 0;
    let mut LR1: ::core::ffi::c_int = 0;
    let mut LR2: ::core::ffi::c_int = 0;
    let mut L: ::core::ffi::c_int = 0;
    K = 1 as ::core::ffi::c_int;
    loop {
        if !(K <= NRANK) {
            current_block = 17407779659766490442;
            break;
        }
        I = *(*LUSOL).ip.offset(K as isize);
        LENI = *(*LUSOL).lenr.offset(I as isize);
        if !(LENI == 0 as ::core::ffi::c_int) {
            LR1 = *(*LUSOL).locr.offset(I as isize);
            LR2 = LR1 + LENI - 1 as ::core::ffi::c_int;
            L = LR1;
            loop {
                if !(L <= LR2) {
                    current_block = 5787028501905255630;
                    break;
                }
                if *(*LUSOL).indr.offset(L as isize) == JZAP {
                    current_block = 18361454741133303765;
                    break;
                }
                L += 1;
            }
            match current_block {
                5787028501905255630 => {}
                _ => {
                    *(*LUSOL).a.offset(L as isize) = *(*LUSOL).a.offset(LR2 as isize);
                    *(*LUSOL).indr.offset(L as isize) = *(*LUSOL).indr.offset(LR2 as isize);
                    *(*LUSOL).indr.offset(LR2 as isize) = 0 as ::core::ffi::c_int;
                    *(*LUSOL).lenr.offset(I as isize) = LENI - 1 as ::core::ffi::c_int;
                    *LENU -= 1;
                }
            }
        }
        *KZAP = K;
        if *(*LUSOL).iq.offset(K as isize) == JZAP {
            current_block = 7497738638356087663;
            break;
        }
        K += 1;
    }
    match current_block {
        17407779659766490442 => {
            L = (*LUSOL).n;
            K = NRANK + 1 as ::core::ffi::c_int;
            while K <= L {
                *KZAP = K;
                if *(*LUSOL).iq.offset(K as isize) == JZAP {
                    break;
                }
                K += 1;
            }
        }
        _ => {}
    }
    if *LROW > 0 as ::core::ffi::c_int {
        if *(*LUSOL).indr.offset(*LROW as isize) == 0 as ::core::ffi::c_int {
            *LROW -= 1;
        }
    }
}
#[export_name="honest_lpsolve_LU8RPC"]
pub unsafe extern "C" fn LU8RPC(
    mut LUSOL: *mut LUSOLrec,
    mut MODE1: ::core::ffi::c_int,
    mut MODE2: ::core::ffi::c_int,
    mut JREP: ::core::ffi::c_int,
    mut V: *mut ::core::ffi::c_double,
    mut W: *mut ::core::ffi::c_double,
    mut INFORM: *mut ::core::ffi::c_int,
    mut DIAG: *mut ::core::ffi::c_double,
    mut VNORM: *mut ::core::ffi::c_double,
) {
    let mut current_block: u64;
    let mut SINGLR: ::core::ffi::c_uchar = 0;
    let mut LPRINT: ::core::ffi::c_int = 0;
    let mut NRANK: ::core::ffi::c_int = 0;
    let mut LENL: ::core::ffi::c_int = 0;
    let mut LENU: ::core::ffi::c_int = 0;
    let mut LROW: ::core::ffi::c_int = 0;
    let mut NRANK0: ::core::ffi::c_int = 0;
    let mut KREP: ::core::ffi::c_int = 0;
    let mut KLAST: ::core::ffi::c_int = 0;
    let mut IW: ::core::ffi::c_int = 0;
    let mut L1: ::core::ffi::c_int = 0;
    let mut J1: ::core::ffi::c_int = 0;
    let mut JSING: ::core::ffi::c_int = 0;
    let mut UTOL1: ::core::ffi::c_double = 0.;
    let mut UTOL2: ::core::ffi::c_double = 0.;
    LPRINT = (*LUSOL).luparm[LUSOL_IP_PRINTLEVEL as usize];
    NRANK = (*LUSOL).luparm[LUSOL_IP_RANK_U as usize];
    LENL = (*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize];
    LENU = (*LUSOL).luparm[LUSOL_IP_NONZEROS_U as usize];
    LROW = (*LUSOL).luparm[LUSOL_IP_NONZEROS_ROW as usize];
    UTOL1 = (*LUSOL).parmlu[LUSOL_RP_SMALLDIAG_U as usize];
    UTOL2 = (*LUSOL).parmlu[LUSOL_RP_EPSDIAG_U as usize];
    NRANK0 = NRANK;
    *DIAG = ZERO as ::core::ffi::c_double;
    *VNORM = ZERO as ::core::ffi::c_double;
    if JREP < 1 as ::core::ffi::c_int {
        current_block = 7893921082977597171;
    } else if JREP > (*LUSOL).n {
        current_block = 7893921082977597171;
    } else {
        if MODE1 == LUSOL_UPDATE_OLDEMPTY {
            KREP = (*LUSOL).n + 1 as ::core::ffi::c_int;
            loop {
                KREP -= 1;
                if !(*(*LUSOL).iq.offset(KREP as isize) != JREP) {
                    break;
                }
            }
        } else {
            LU7ZAP(
                LUSOL,
                JREP,
                &raw mut KREP,
                &raw mut LENU,
                &raw mut LROW,
                NRANK,
            );
        }
        if MODE2 == LUSOL_UPDATE_NEWEMPTY {
            KLAST = 0 as ::core::ffi::c_int;
            current_block = 14401909646449704462;
        } else {
            if MODE2 == LUSOL_UPDATE_NEWNONEMPTY {
                LU6SOL(
                    LUSOL,
                    LUSOL_SOLVE_Lv_v,
                    V,
                    W,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    INFORM,
                );
            } else if V.is_null() {
                V = (*LUSOL).vLU6L as *mut ::core::ffi::c_double;
            }
            LU7ADD(
                LUSOL,
                JREP,
                V,
                LENL,
                &raw mut LENU,
                &raw mut LROW,
                NRANK,
                INFORM,
                &raw mut KLAST,
                VNORM,
            );
            if *INFORM == LUSOL_INFORM_ANEEDMEM {
                current_block = 1851266092312843941;
            } else {
                current_block = 14401909646449704462;
            }
        }
        match current_block {
            14401909646449704462 => {
                if MODE2 == LUSOL_UPDATE_NEWEMPTY {
                    if KREP > NRANK {
                        current_block = 2108160683830896987;
                    } else {
                        current_block = 15897653523371991391;
                    }
                } else if NRANK < (*LUSOL).m {
                    LU7ELM(
                        LUSOL,
                        JREP,
                        V,
                        &raw mut LENL,
                        &raw mut LROW,
                        NRANK,
                        INFORM,
                        DIAG,
                    );
                    if *INFORM == LUSOL_INFORM_ANEEDMEM {
                        current_block = 1851266092312843941;
                    } else {
                        if *INFORM == LUSOL_INFORM_LUSINGULAR {
                            NRANK += 1;
                            KLAST = NRANK;
                        }
                        current_block = 15897653523371991391;
                    }
                } else {
                    current_block = 15897653523371991391;
                }
                match current_block {
                    1851266092312843941 => {}
                    _ => {
                        match current_block {
                            15897653523371991391 => {
                                if NRANK < (*LUSOL).n {
                                    if KREP < NRANK {
                                        KLAST = NRANK;
                                    } else {
                                        *(*LUSOL).iq.offset(KREP as isize) =
                                            *(*LUSOL).iq.offset(NRANK as isize);
                                        *(*LUSOL).iq.offset(NRANK as isize) = JREP;
                                        KREP = NRANK;
                                    }
                                }
                                if KREP <= KLAST {
                                    LU7CYC(
                                        LUSOL,
                                        KREP,
                                        KLAST,
                                        (*LUSOL).ip as *mut ::core::ffi::c_int,
                                    );
                                    LU7CYC(
                                        LUSOL,
                                        KREP,
                                        KLAST,
                                        (*LUSOL).iq as *mut ::core::ffi::c_int,
                                    );
                                    LU7FOR(
                                        LUSOL,
                                        KREP,
                                        KLAST,
                                        &raw mut LENL,
                                        &raw mut LENU,
                                        &raw mut LROW,
                                        INFORM,
                                        DIAG,
                                    );
                                    if *INFORM == LUSOL_INFORM_ANEEDMEM {
                                        current_block = 1851266092312843941;
                                    } else {
                                        KREP = KLAST;
                                        SINGLR = (*VNORM < UTOL2 * fabs(*DIAG))
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uchar;
                                        if SINGLR != 0 {
                                            *INFORM = LUSOL_INFORM_LUUNSTABLE;
                                            if LPRINT >= LUSOL_MSG_SINGULARITY {
                                                LUSOL_report(
                                                    LUSOL,
                                                    0 as ::core::ffi::c_int,
                                                    b"lu8rpc  warning...\nInstability after replacing column.    jrep=%8d    diag=%g\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            }
                                            current_block = 13912628294591860825;
                                        } else {
                                            current_block = 3938820862080741272;
                                        }
                                    }
                                } else {
                                    current_block = 3938820862080741272;
                                }
                                match current_block {
                                    13912628294591860825 => {}
                                    1851266092312843941 => {}
                                    _ => {
                                        *DIAG = ZERO as ::core::ffi::c_double;
                                        IW = *(*LUSOL).ip.offset(KREP as isize);
                                        SINGLR = (*(*LUSOL).lenr.offset(IW as isize)
                                            == 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uchar;
                                        if SINGLR == 0 {
                                            L1 = *(*LUSOL).locr.offset(IW as isize);
                                            J1 = *(*LUSOL).indr.offset(L1 as isize);
                                            SINGLR = (J1 != JREP) as ::core::ffi::c_int
                                                as ::core::ffi::c_uchar;
                                            if SINGLR == 0 {
                                                *DIAG = *(*LUSOL).a.offset(L1 as isize);
                                                SINGLR = (fabs(*DIAG) <= UTOL1
                                                    || fabs(*DIAG) <= UTOL2 * *VNORM)
                                                    as ::core::ffi::c_int
                                                    as ::core::ffi::c_uchar;
                                            }
                                        }
                                        if SINGLR as ::core::ffi::c_int != 0 && KREP < NRANK {
                                            LU7CYC(
                                                LUSOL,
                                                KREP,
                                                NRANK,
                                                (*LUSOL).ip as *mut ::core::ffi::c_int,
                                            );
                                            LU7CYC(
                                                LUSOL,
                                                KREP,
                                                (*LUSOL).n,
                                                (*LUSOL).iq as *mut ::core::ffi::c_int,
                                            );
                                            LU7FOR(
                                                LUSOL,
                                                KREP,
                                                NRANK,
                                                &raw mut LENL,
                                                &raw mut LENU,
                                                &raw mut LROW,
                                                INFORM,
                                                DIAG,
                                            );
                                            if *INFORM == LUSOL_INFORM_ANEEDMEM {
                                                current_block = 1851266092312843941;
                                            } else {
                                                current_block = 16415152177862271243;
                                            }
                                        } else {
                                            current_block = 16415152177862271243;
                                        }
                                        match current_block {
                                            1851266092312843941 => {}
                                            _ => {
                                                if SINGLR as ::core::ffi::c_int != 0
                                                    || NRANK < (*LUSOL).n
                                                {
                                                    JSING = 0 as ::core::ffi::c_int;
                                                    if SINGLR != 0 {
                                                        JSING = JREP;
                                                    }
                                                    LU7RNK(
                                                        LUSOL,
                                                        JSING,
                                                        &raw mut LENU,
                                                        &raw mut LROW,
                                                        &raw mut NRANK,
                                                        INFORM,
                                                        DIAG,
                                                    );
                                                }
                                                current_block = 2108160683830896987;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            13912628294591860825 => {}
                            1851266092312843941 => {}
                            _ => {
                                if NRANK == NRANK0 {
                                    *INFORM = LUSOL_INFORM_LUSUCCESS;
                                } else if NRANK < NRANK0 {
                                    *INFORM = LUSOL_INFORM_RANKLOSS;
                                    if NRANK0 == (*LUSOL).n {
                                        if LPRINT >= LUSOL_MSG_SINGULARITY {
                                            LUSOL_report(
                                                LUSOL,
                                                0 as ::core::ffi::c_int,
                                                b"lu8rpc  warning...\nSingularity after replacing column.    jrep=%8d    diag=%g\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                } else {
                                    *INFORM = LUSOL_INFORM_LUSINGULAR;
                                }
                                current_block = 13912628294591860825;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            13912628294591860825 => {}
            _ => {
                *INFORM = LUSOL_INFORM_ANEEDMEM;
                if LPRINT >= LUSOL_MSG_SINGULARITY {
                    LUSOL_report(
                        LUSOL,
                        0 as ::core::ffi::c_int,
                        b"lu8rpc  error...\nInsufficient memory.    lena=%8d\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                current_block = 13912628294591860825;
            }
        }
    }
    match current_block {
        7893921082977597171 => {
            *INFORM = LUSOL_INFORM_FATALERR;
            if LPRINT >= LUSOL_MSG_SINGULARITY {
                LUSOL_report(
                    LUSOL,
                    0 as ::core::ffi::c_int,
                    b"lu8rpc  error...\njrep  is out of range.    m=%8d    n=%8d    jrep=%8d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    (*LUSOL).luparm[LUSOL_IP_UPDATECOUNT as usize] += 1;
    (*LUSOL).luparm[LUSOL_IP_RANK_U as usize] = NRANK;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize] = LENL;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_U as usize] = LENU;
    (*LUSOL).luparm[LUSOL_IP_NONZEROS_ROW as usize] = LROW;
    (*LUSOL).luparm[LUSOL_IP_INFORM as usize] = *INFORM;
}
#[export_name="honest_lpsolve_LUSOL_dump"]
pub unsafe extern "C" fn LUSOL_dump(mut output: *mut FILE, mut LUSOL: *mut LUSOLrec) {
    let mut userfile: ::core::ffi::c_uchar =
        (output != NULL as *mut FILE) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if userfile == 0 {
        output = native_only!(fopen,
            b"LUSOL.dbg\0" as *const u8 as *const ::core::ffi::c_char,
            b"w\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    blockWriteREAL(
        output,
        b"a\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).a,
        1 as ::core::ffi::c_int,
        (*LUSOL).lena,
    );
    blockWriteINT(
        output,
        b"indc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).indc,
        1 as ::core::ffi::c_int,
        (*LUSOL).lena,
    );
    blockWriteINT(
        output,
        b"indr\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).indr,
        1 as ::core::ffi::c_int,
        (*LUSOL).lena,
    );
    blockWriteINT(
        output,
        b"ip\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).ip,
        1 as ::core::ffi::c_int,
        (*LUSOL).m,
    );
    blockWriteINT(
        output,
        b"iq\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).iq,
        1 as ::core::ffi::c_int,
        (*LUSOL).n,
    );
    blockWriteINT(
        output,
        b"lenc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).lenc,
        1 as ::core::ffi::c_int,
        (*LUSOL).n,
    );
    blockWriteINT(
        output,
        b"lenr\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).lenr,
        1 as ::core::ffi::c_int,
        (*LUSOL).m,
    );
    blockWriteINT(
        output,
        b"locc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).locc,
        1 as ::core::ffi::c_int,
        (*LUSOL).n,
    );
    blockWriteINT(
        output,
        b"locr\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).locr,
        1 as ::core::ffi::c_int,
        (*LUSOL).m,
    );
    blockWriteINT(
        output,
        b"iploc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).iploc,
        1 as ::core::ffi::c_int,
        (*LUSOL).n,
    );
    blockWriteINT(
        output,
        b"iqloc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).iqloc,
        1 as ::core::ffi::c_int,
        (*LUSOL).m,
    );
    blockWriteINT(
        output,
        b"ipinv\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).ipinv,
        1 as ::core::ffi::c_int,
        (*LUSOL).m,
    );
    blockWriteINT(
        output,
        b"iqinv\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*LUSOL).iqinv,
        1 as ::core::ffi::c_int,
        (*LUSOL).n,
    );
    if userfile == 0 {
        native_only!(fclose,output);
    }
}
#[export_name="honest_lpsolve_LUSOL_matcreate"]
pub unsafe extern "C" fn LUSOL_matcreate(
    mut dim: ::core::ffi::c_int,
    mut nz: ::core::ffi::c_int,
) -> *mut LUSOLmat {
    let mut newm: *mut LUSOLmat = ::core::ptr::null_mut::<LUSOLmat>();
    newm = calloc(1 as size_t, ::core::mem::size_of::<LUSOLmat>() as size_t) as *mut LUSOLmat;
    if !newm.is_null() {
        (*newm).a = malloc(
            ((nz + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        ) as *mut ::core::ffi::c_double;
        (*newm).lenx = malloc(
            ((dim + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        ) as *mut ::core::ffi::c_int;
        (*newm).indx = malloc(
            ((dim + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        ) as *mut ::core::ffi::c_int;
        (*newm).indr = malloc(
            ((nz + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        ) as *mut ::core::ffi::c_int;
        (*newm).indc = malloc(
            ((nz + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        ) as *mut ::core::ffi::c_int;
        if (*newm).a.is_null()
            || (*newm).lenx.is_null()
            || (*newm).indx.is_null()
            || (*newm).indr.is_null()
            || (*newm).indc.is_null()
        {
            LUSOL_matfree(&raw mut newm);
        }
    }
    return newm;
}
#[export_name="honest_lpsolve_LUSOL_matfree"]
pub unsafe extern "C" fn LUSOL_matfree(mut mat: *mut *mut LUSOLmat) {
    if mat.is_null() || (*mat).is_null() {
        return;
    }
    free((**mat).a as *mut ::core::ffi::c_void);
    (**mat).a = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((**mat).indc as *mut ::core::ffi::c_void);
    (**mat).indc = ::core::ptr::null_mut::<::core::ffi::c_int>();
    free((**mat).indr as *mut ::core::ffi::c_void);
    (**mat).indr = ::core::ptr::null_mut::<::core::ffi::c_int>();
    free((**mat).lenx as *mut ::core::ffi::c_void);
    (**mat).lenx = ::core::ptr::null_mut::<::core::ffi::c_int>();
    free((**mat).indx as *mut ::core::ffi::c_void);
    (**mat).indx = ::core::ptr::null_mut::<::core::ffi::c_int>();
    free(*mat as *mut ::core::ffi::c_void);
    *mat = ::core::ptr::null_mut::<LUSOLmat>();
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_ARRAYOFFSET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ZERO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ONE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_DEFAULT_GAMMA: ::core::ffi::c_double = 2.0f64;
pub const LUSOL_SMALLNUM: ::core::ffi::c_double = 1.0e-20f64;
pub const LUSOL_BIGNUM: ::core::ffi::c_double = 1.0e+20f64;
pub const LUSOL_MINDELTA_FACTOR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_MULT_nz_a: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_DEFAULT_SMARTRATIO: ::core::ffi::c_double = 0.667f64;
pub const LUSOL_RP_SMARTRATIO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_RP_FACTORMAX_Lij: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_RP_UPDATEMAX_Lij: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_RP_ZEROTOLERANCE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LUSOL_RP_SMALLDIAG_U: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_RP_EPSDIAG_U: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LUSOL_RP_COMPSPACE_U: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LUSOL_RP_MARKOWITZ_CONLY: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LUSOL_RP_MARKOWITZ_DENSE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LUSOL_RP_GAMMA: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const LUSOL_RP_MAXELEM_A: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LUSOL_RP_MAXMULT_L: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const LUSOL_RP_MAXELEM_U: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LUSOL_RP_MAXELEM_DIAGU: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const LUSOL_RP_MINELEM_DIAGU: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const LUSOL_RP_MAXELEM_TCP: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const LUSOL_RP_GROWTHRATE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LUSOL_RP_RESIDUAL_U: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const LUSOL_IP_PRINTUNIT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_IP_PRINTLEVEL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_IP_MARKOWITZ_MAXCOL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LUSOL_IP_SCALAR_NZA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_IP_UPDATELIMIT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LUSOL_IP_PIVOTTYPE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LUSOL_IP_ACCELERATION: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LUSOL_IP_KEEPLU: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LUSOL_IP_SINGULARLISTSIZE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const LUSOL_IP_INFORM: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LUSOL_IP_SINGULARITIES: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const LUSOL_IP_SINGULARINDEX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LUSOL_IP_MINIMUMLENA: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const LUSOL_IP_UPDATECOUNT: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const LUSOL_IP_RANK_U: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LUSOL_IP_COLCOUNT_DENSE1: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const LUSOL_IP_COLCOUNT_DENSE2: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const LUSOL_IP_COLINDEX_DUMIN: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const LUSOL_IP_COLCOUNT_L0: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_L0: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_U0: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_L: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_U: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_ROW: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const LUSOL_IP_COMPRESSIONS_LU: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const LUSOL_IP_MARKOWITZ_MERIT: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const LUSOL_IP_TRIANGROWS_U: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const LUSOL_IP_TRIANGROWS_L: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const LUSOL_IP_FTRANCOUNT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const LUSOL_IP_BTRANCOUNT: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const LUSOL_IP_ROWCOUNT_L0: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const LUSOL_MSG_SINGULARITY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_MSG_STATISTICS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LUSOL_MSG_PIVOT: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const LUSOL_BASEORDER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_AUTOORDER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_ACCELERATE_L0: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_ACCELERATE_U: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_NOCHANGE: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const LUSOL_PIVMOD_DEFAULT: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const LUSOL_PIVMOD_TPP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_TRP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_TCP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_TSP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_MAX: ::core::ffi::c_int = LUSOL_PIVMOD_TSP;
pub const LUSOL_PIVTOL_NOCHANGE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_BAGGY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_LOOSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_NORMAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_SLIM: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_TIGHT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_SUPER: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_CORSET: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_DEFAULT: ::core::ffi::c_int = LUSOL_PIVTOL_SLIM;
pub const LUSOL_PIVTOL_MAX: ::core::ffi::c_int = LUSOL_PIVTOL_CORSET;
pub const LUSOL_UPDATE_OLDEMPTY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_UPDATE_OLDNONEMPTY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_UPDATE_NEWEMPTY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_UPDATE_NEWNONEMPTY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_Lv_v: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_Ltv_v: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_Uw_v: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_Utv_w: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_Aw_v: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_Atv_w: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_Av_v: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LUSOL_SOLVE_LDLtv_v: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LUSOL_INFORM_RANKLOSS: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const LUSOL_INFORM_LUSUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_INFORM_LUSINGULAR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_INFORM_LUUNSTABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_INFORM_ADIMERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LUSOL_INFORM_ADUPLICATE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_INFORM_ANEEDMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LUSOL_INFORM_FATALERR: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LUSOL_INFORM_NOPIVOT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const LUSOL_INFORM_NOMEMLEFT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LUSOL_INFORM_MIN: ::core::ffi::c_int = LUSOL_INFORM_RANKLOSS;
pub const LUSOL_INFORM_MAX: ::core::ffi::c_int = LUSOL_INFORM_NOMEMLEFT;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
