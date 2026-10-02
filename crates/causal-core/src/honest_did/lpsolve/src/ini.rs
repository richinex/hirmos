use crate::honest_did::lpsolve::runtime::{strlen,strchr};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
extern "C" {
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn fputs(_: *const ::core::ffi::c_char, _: *mut FILE) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
}
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
pub type __darwin_size_t = usize;
pub type __darwin_wchar_t = ::core::ffi::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: ::core::ffi::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [::core::ffi::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [::core::ffi::c_char; 8],
    pub __encoding: [::core::ffi::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            __darwin_size_t,
            *mut *const ::core::ffi::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut ::core::ffi::c_char,
            __darwin_size_t,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut ::core::ffi::c_void,
    pub __variable_len: ::core::ffi::c_int,
    pub __ncharclasses: ::core::ffi::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
pub const _CTYPE_S: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
#[inline]
unsafe extern "C" fn isascii(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __istype(
    mut _c: __darwin_ct_rune_t,
    mut _f: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    return if isascii(_c as ::core::ffi::c_int) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as ::core::ffi::c_ulong & _f != 0)
            as ::core::ffi::c_int
    } else {
        (native_only!(__maskrune,_c, _f) != 0) as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong);
}
#[export_name="honest_lpsolve_ini_create"]
pub unsafe extern "C" fn ini_create(mut filename: *mut ::core::ffi::c_char) -> *mut FILE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fp = native_only!(fopen,filename, b"w\0" as *const u8 as *const ::core::ffi::c_char);
    return fp;
}
#[export_name="honest_lpsolve_ini_open"]
pub unsafe extern "C" fn ini_open(mut filename: *mut ::core::ffi::c_char) -> *mut FILE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fp = native_only!(fopen,filename, b"r\0" as *const u8 as *const ::core::ffi::c_char);
    return fp;
}
#[export_name="honest_lpsolve_ini_writecomment"]
pub unsafe extern "C" fn ini_writecomment(
    mut fp: *mut FILE,
    mut comment: *mut ::core::ffi::c_char,
) {
    native_only!(fprintf,
        fp,
        b"; %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        comment,
    );
}
#[export_name="honest_lpsolve_ini_writeheader"]
pub unsafe extern "C" fn ini_writeheader(
    mut fp: *mut FILE,
    mut header: *mut ::core::ffi::c_char,
    mut addnewline: ::core::ffi::c_int,
) {
    if addnewline != 0 && native_only!(ftell,fp) > 0 as ::core::ffi::c_long {
        native_only!(fputs,b"\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
    }
    native_only!(fprintf,
        fp,
        b"[%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        header,
    );
}
#[export_name="honest_lpsolve_ini_writedata"]
pub unsafe extern "C" fn ini_writedata(
    mut fp: *mut FILE,
    mut name: *mut ::core::ffi::c_char,
    mut data: *mut ::core::ffi::c_char,
) {
    if !name.is_null() {
        native_only!(fprintf,
            fp,
            b"%s=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
            data,
        );
    } else {
        native_only!(fprintf,
            fp,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            data,
        );
    };
}
#[export_name="honest_lpsolve_ini_readdata"]
pub unsafe extern "C" fn ini_readdata(
    mut fp: *mut FILE,
    mut data: *mut ::core::ffi::c_char,
    mut szdata: ::core::ffi::c_int,
    mut withcomment: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = 0;
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if native_only!(fgets,data, szdata, fp).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if withcomment == 0 {
        ptr = strchr(data, ';' as i32);
        if !ptr.is_null() {
            *ptr = 0 as ::core::ffi::c_char;
        }
    }
    l = strlen(data) as ::core::ffi::c_int;
    while l > 0 as ::core::ffi::c_int
        && isspace(*data.offset((l - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int) != 0
    {
        l -= 1;
    }
    *data.offset(l as isize) = 0 as ::core::ffi::c_char;
    if l >= 2 as ::core::ffi::c_int
        && *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32
        && *data.offset((l - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == ']' as i32
    {
        memcpy(
            data as *mut ::core::ffi::c_void,
            data.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (l - 2 as ::core::ffi::c_int) as size_t,
        );
        *data.offset((l - 2 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_char;
        return 1 as ::core::ffi::c_int;
    }
    return 2 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_ini_close"]
pub unsafe extern "C" fn ini_close(mut fp: *mut FILE) {
    native_only!(fclose,fp);
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
