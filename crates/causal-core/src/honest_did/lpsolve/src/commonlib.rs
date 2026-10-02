use crate::honest_did::lpsolve::runtime::{strlen,strcpy,strcmp,strncmp,strcat,strrchr};
use crate::honest_did::lpsolve::runtime::{malloc,free};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
extern "C" {
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union QSORTrec {
    pub pvoid2: QSORTrec1,
    pub pvoidreal: QSORTrec2,
    pub pvoidint2: QSORTrec3,
    pub realint2: QSORTrec4,
    pub reallong: QSORTrec5,
    pub real2: QSORTrec6,
    pub int4: QSORTrec7,
}
pub type QSORTrec7 = _QSORTrec7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec7 {
    pub intval: ::core::ffi::c_int,
    pub intpar1: ::core::ffi::c_int,
    pub intpar2: ::core::ffi::c_int,
    pub intpar3: ::core::ffi::c_int,
}
pub type QSORTrec6 = _QSORTrec6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec6 {
    pub realval: ::core::ffi::c_double,
    pub realpar1: ::core::ffi::c_double,
}
pub type QSORTrec5 = _QSORTrec5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec5 {
    pub realval: ::core::ffi::c_double,
    pub longval: ::core::ffi::c_long,
}
pub type QSORTrec4 = _QSORTrec4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec4 {
    pub realval: ::core::ffi::c_double,
    pub intval: ::core::ffi::c_int,
    pub intpar1: ::core::ffi::c_int,
}
pub type QSORTrec3 = _QSORTrec3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec3 {
    pub ptr: *mut ::core::ffi::c_void,
    pub intval: ::core::ffi::c_int,
    pub intpar1: ::core::ffi::c_int,
}
pub type QSORTrec2 = _QSORTrec2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec2 {
    pub ptr: *mut ::core::ffi::c_void,
    pub realval: ::core::ffi::c_double,
}
pub type QSORTrec1 = _QSORTrec1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec1 {
    pub ptr: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
pub type findCompare_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn tolower(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return native_only!(__tolower,_c as __darwin_ct_rune_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toupper(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return native_only!(__toupper,_c as __darwin_ct_rune_t) as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_intpow"]
pub unsafe extern "C" fn intpow(
    mut base: ::core::ffi::c_int,
    mut exponent: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while exponent > 0 as ::core::ffi::c_int {
        result *= base;
        exponent -= 1;
    }
    while exponent < 0 as ::core::ffi::c_int {
        result /= base;
        exponent += 1;
    }
    return result;
}
#[export_name = "mod"]
pub unsafe extern "C" fn mod_0(
    mut n: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return n % d;
}
#[export_name="honest_lpsolve_strtoup"]
pub unsafe extern "C" fn strtoup(mut s: *mut ::core::ffi::c_char) {
    if !s.is_null() {
        while *s != 0 {
            *s = toupper(*s as ::core::ffi::c_int) as ::core::ffi::c_char;
            s = s.offset(1);
        }
    }
}
#[export_name="honest_lpsolve_strtolo"]
pub unsafe extern "C" fn strtolo(mut s: *mut ::core::ffi::c_char) {
    if !s.is_null() {
        while *s != 0 {
            *s = tolower(*s as ::core::ffi::c_int) as ::core::ffi::c_char;
            s = s.offset(1);
        }
    }
}
#[export_name="honest_lpsolve_strcpyup"]
pub unsafe extern "C" fn strcpyup(
    mut t: *mut ::core::ffi::c_char,
    mut s: *mut ::core::ffi::c_char,
) {
    if !s.is_null() && !t.is_null() {
        while *s != 0 {
            *t = toupper(*s as ::core::ffi::c_int) as ::core::ffi::c_char;
            t = t.offset(1);
            s = s.offset(1);
        }
        *t = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[export_name="honest_lpsolve_strcpylo"]
pub unsafe extern "C" fn strcpylo(
    mut t: *mut ::core::ffi::c_char,
    mut s: *mut ::core::ffi::c_char,
) {
    if !s.is_null() && !t.is_null() {
        while *s != 0 {
            *t = tolower(*s as ::core::ffi::c_int) as ::core::ffi::c_char;
            t = t.offset(1);
            s = s.offset(1);
        }
        *t = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[export_name="honest_lpsolve_so_stdname"]
pub unsafe extern "C" fn so_stdname(
    mut stdname: *mut ::core::ffi::c_char,
    mut descname: *mut ::core::ffi::c_char,
    mut buflen: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if descname.is_null()
        || stdname.is_null()
        || strlen(descname) as ::core::ffi::c_int >= buflen - 6 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_uchar;
    }
    strcpy(stdname, descname);
    ptr = strrchr(descname, '/' as i32);
    if ptr.is_null() {
        ptr = descname;
    } else {
        ptr = ptr.offset(1);
    }
    *stdname
        .offset(ptr.offset_from(descname) as ::core::ffi::c_long as ::core::ffi::c_int as isize) =
        0 as ::core::ffi::c_char;
    if strncmp(
        ptr,
        b"lib\0" as *const u8 as *const ::core::ffi::c_char,
        3 as size_t,
    ) != 0
    {
        strcat(stdname, b"lib\0" as *const u8 as *const ::core::ffi::c_char);
    }
    strcat(stdname, ptr);
    if strcmp(
        stdname
            .offset(strlen(stdname) as isize)
            .offset(-(3 as ::core::ffi::c_int as isize)),
        b".so\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        strcat(stdname, b".so\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_gcd"]
pub unsafe extern "C" fn gcd(
    mut a: ::core::ffi::c_longlong,
    mut b: ::core::ffi::c_longlong,
    mut c: *mut ::core::ffi::c_int,
    mut d: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut q: ::core::ffi::c_longlong = 0;
    let mut r: ::core::ffi::c_longlong = 0;
    let mut t: ::core::ffi::c_longlong = 0;
    let mut cret: ::core::ffi::c_int = 0;
    let mut dret: ::core::ffi::c_int = 0;
    let mut C: ::core::ffi::c_int = 0;
    let mut D: ::core::ffi::c_int = 0;
    let mut rval: ::core::ffi::c_int = 0;
    let mut sgn_a: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut sgn_b: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut swap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if a == 0 as ::core::ffi::c_longlong || b == 0 as ::core::ffi::c_longlong {
        return -(1 as ::core::ffi::c_int);
    }
    if c.is_null() {
        c = &raw mut cret;
    }
    if d.is_null() {
        d = &raw mut dret;
    }
    if a < 0 as ::core::ffi::c_longlong {
        a = -a;
        sgn_a = -(1 as ::core::ffi::c_int);
    }
    if b < 0 as ::core::ffi::c_longlong {
        b = -b;
        sgn_b = -(1 as ::core::ffi::c_int);
    }
    if b < a {
        t = b;
        b = a;
        a = t;
        swap = 1 as ::core::ffi::c_int;
    }
    q = b / a;
    r = b - a * q;
    if r == 0 as ::core::ffi::c_longlong {
        if swap != 0 {
            *d = 1 as ::core::ffi::c_int;
            *c = 0 as ::core::ffi::c_int;
        } else {
            *c = 1 as ::core::ffi::c_int;
            *d = 0 as ::core::ffi::c_int;
        }
        *c = sgn_a * *c;
        *d = sgn_b * *d;
        return a as ::core::ffi::c_int;
    }
    rval = gcd(a, r, &raw mut C, &raw mut D);
    if swap != 0 {
        *d =
            (C as ::core::ffi::c_longlong - D as ::core::ffi::c_longlong * q) as ::core::ffi::c_int;
        *c = D;
    } else {
        *d = D;
        *c =
            (C as ::core::ffi::c_longlong - D as ::core::ffi::c_longlong * q) as ::core::ffi::c_int;
    }
    *c = sgn_a * *c;
    *d = sgn_b * *d;
    return rval;
}
#[export_name="honest_lpsolve_findIndex"]
pub unsafe extern "C" fn findIndex(
    mut target: ::core::ffi::c_int,
    mut attributes: *mut ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut focusPos: ::core::ffi::c_int = 0;
    let mut beginPos: ::core::ffi::c_int = 0;
    let mut endPos: ::core::ffi::c_int = 0;
    let mut focusAttrib: ::core::ffi::c_int = 0;
    let mut beginAttrib: ::core::ffi::c_int = 0;
    let mut endAttrib: ::core::ffi::c_int = 0;
    beginPos = offset;
    endPos = beginPos + count - 1 as ::core::ffi::c_int;
    if endPos < beginPos {
        return -(1 as ::core::ffi::c_int);
    }
    focusPos = (beginPos + endPos) / 2 as ::core::ffi::c_int;
    beginAttrib = *attributes.offset(beginPos as isize);
    focusAttrib = *attributes.offset(focusPos as isize);
    endAttrib = *attributes.offset(endPos as isize);
    while endPos - beginPos > LINEARSEARCH {
        if beginAttrib == target {
            focusAttrib = beginAttrib;
            endPos = beginPos;
        } else if endAttrib == target {
            focusAttrib = endAttrib;
            beginPos = endPos;
        } else if focusAttrib < target {
            beginPos = focusPos + 1 as ::core::ffi::c_int;
            beginAttrib = *attributes.offset(beginPos as isize);
            focusPos = (beginPos + endPos) / 2 as ::core::ffi::c_int;
            focusAttrib = *attributes.offset(focusPos as isize);
        } else if focusAttrib > target {
            endPos = focusPos - 1 as ::core::ffi::c_int;
            endAttrib = *attributes.offset(endPos as isize);
            focusPos = (beginPos + endPos) / 2 as ::core::ffi::c_int;
            focusAttrib = *attributes.offset(focusPos as isize);
        } else {
            beginPos = focusPos;
            endPos = focusPos;
        }
    }
    if endPos - beginPos <= LINEARSEARCH {
        let mut attptr: *mut ::core::ffi::c_int = attributes.offset(beginPos as isize);
        while beginPos < endPos && *attptr < target {
            beginPos += 1;
            attptr = attptr.offset(1);
        }
        focusAttrib = *attptr;
    }
    if focusAttrib == target {
        return beginPos;
    } else if focusAttrib > target {
        return -beginPos;
    } else if beginPos > offset + count - 1 as ::core::ffi::c_int {
        return -(endPos + 1 as ::core::ffi::c_int);
    } else {
        return -(beginPos + 1 as ::core::ffi::c_int);
    };
}
#[export_name="honest_lpsolve_findIndexEx"]
pub unsafe extern "C" fn findIndexEx(
    mut target: *mut ::core::ffi::c_void,
    mut attributes: *mut ::core::ffi::c_void,
    mut count: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut recsize: ::core::ffi::c_int,
    mut findCompare: Option<findCompare_func>,
    mut ascending: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut focusPos: ::core::ffi::c_int = 0;
    let mut beginPos: ::core::ffi::c_int = 0;
    let mut endPos: ::core::ffi::c_int = 0;
    let mut compare: ::core::ffi::c_int = 0;
    let mut order: ::core::ffi::c_int = 0;
    let mut focusAttrib: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut beginAttrib: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut endAttrib: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    beginPos = offset;
    endPos = beginPos + count - 1 as ::core::ffi::c_int;
    if endPos < beginPos {
        return -(1 as ::core::ffi::c_int);
    }
    order = if ascending as ::core::ffi::c_int != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    focusPos = (beginPos + endPos) / 2 as ::core::ffi::c_int;
    beginAttrib = (attributes as *mut ::core::ffi::c_char).offset((beginPos * recsize) as isize)
        as *mut ::core::ffi::c_void;
    focusAttrib = (attributes as *mut ::core::ffi::c_char).offset((focusPos * recsize) as isize)
        as *mut ::core::ffi::c_void;
    endAttrib = (attributes as *mut ::core::ffi::c_char).offset((endPos * recsize) as isize)
        as *mut ::core::ffi::c_void;
    compare = 0 as ::core::ffi::c_int;
    while endPos - beginPos > LINEARSEARCH {
        if findCompare.expect("non-null function pointer")(target, beginAttrib)
            == 0 as ::core::ffi::c_int
        {
            focusAttrib = beginAttrib;
            endPos = beginPos;
        } else if findCompare.expect("non-null function pointer")(target, endAttrib)
            == 0 as ::core::ffi::c_int
        {
            focusAttrib = endAttrib;
            beginPos = endPos;
        } else {
            compare = findCompare.expect("non-null function pointer")(target, focusAttrib) * order;
            if compare < 0 as ::core::ffi::c_int {
                beginPos = focusPos + 1 as ::core::ffi::c_int;
                beginAttrib = (attributes as *mut ::core::ffi::c_char)
                    .offset((beginPos * recsize) as isize)
                    as *mut ::core::ffi::c_void;
                focusPos = (beginPos + endPos) / 2 as ::core::ffi::c_int;
                focusAttrib = (attributes as *mut ::core::ffi::c_char)
                    .offset((focusPos * recsize) as isize)
                    as *mut ::core::ffi::c_void;
            } else if compare > 0 as ::core::ffi::c_int {
                endPos = focusPos - 1 as ::core::ffi::c_int;
                endAttrib = (attributes as *mut ::core::ffi::c_char)
                    .offset((endPos * recsize) as isize)
                    as *mut ::core::ffi::c_void;
                focusPos = (beginPos + endPos) / 2 as ::core::ffi::c_int;
                focusAttrib = (attributes as *mut ::core::ffi::c_char)
                    .offset((focusPos * recsize) as isize)
                    as *mut ::core::ffi::c_void;
            } else {
                beginPos = focusPos;
                endPos = focusPos;
            }
        }
    }
    if endPos - beginPos <= LINEARSEARCH {
        focusAttrib = (attributes as *mut ::core::ffi::c_char).offset((beginPos * recsize) as isize)
            as *mut ::core::ffi::c_void;
        if beginPos == endPos {
            compare = findCompare.expect("non-null function pointer")(target, focusAttrib) * order;
        } else {
            while beginPos < endPos && {
                compare =
                    findCompare.expect("non-null function pointer")(target, focusAttrib) * order;
                compare < 0 as ::core::ffi::c_int
            } {
                beginPos += 1;
                focusAttrib = (attributes as *mut ::core::ffi::c_char)
                    .offset((beginPos * recsize) as isize)
                    as *mut ::core::ffi::c_void;
            }
        }
    }
    if compare == 0 as ::core::ffi::c_int {
        return beginPos;
    } else if compare > 0 as ::core::ffi::c_int {
        return -beginPos;
    } else if beginPos > offset + count - 1 as ::core::ffi::c_int {
        return -(endPos + 1 as ::core::ffi::c_int);
    } else {
        return -(beginPos + 1 as ::core::ffi::c_int);
    };
}
#[export_name="honest_lpsolve_compareCHAR"]
pub unsafe extern "C" fn compareCHAR(
    mut current: *const ::core::ffi::c_void,
    mut candidate: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return if (*(current as *mut ::core::ffi::c_char) as ::core::ffi::c_int)
        < *(candidate as *mut ::core::ffi::c_char) as ::core::ffi::c_int
    {
        -(1 as ::core::ffi::c_int)
    } else if *(current as *mut ::core::ffi::c_char) as ::core::ffi::c_int
        > *(candidate as *mut ::core::ffi::c_char) as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[export_name="honest_lpsolve_compareINT"]
pub unsafe extern "C" fn compareINT(
    mut current: *const ::core::ffi::c_void,
    mut candidate: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return if *(current as *mut ::core::ffi::c_int) < *(candidate as *mut ::core::ffi::c_int) {
        -(1 as ::core::ffi::c_int)
    } else if *(current as *mut ::core::ffi::c_int) > *(candidate as *mut ::core::ffi::c_int) {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[export_name="honest_lpsolve_compareREAL"]
pub unsafe extern "C" fn compareREAL(
    mut current: *const ::core::ffi::c_void,
    mut candidate: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return if *(current as *mut ::core::ffi::c_double) < *(candidate as *mut ::core::ffi::c_double)
    {
        -(1 as ::core::ffi::c_int)
    } else if *(current as *mut ::core::ffi::c_double) > *(candidate as *mut ::core::ffi::c_double)
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[export_name="honest_lpsolve_hpsort"]
pub unsafe extern "C" fn hpsort(
    mut attributes: *mut ::core::ffi::c_void,
    mut count: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut recsize: ::core::ffi::c_int,
    mut descending: ::core::ffi::c_uchar,
    mut findCompare: Option<findCompare_func>,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ir: ::core::ffi::c_int = 0;
    let mut order: ::core::ffi::c_int = 0;
    let mut hold: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut base: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut save: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if count < 2 as ::core::ffi::c_int {
        return;
    }
    offset -= 1 as ::core::ffi::c_int;
    attributes = (attributes as *mut ::core::ffi::c_char).offset((offset * recsize) as isize)
        as *mut ::core::ffi::c_void;
    base = (attributes as *mut ::core::ffi::c_char)
        .offset((1 as ::core::ffi::c_int * recsize) as isize);
    save = malloc(recsize as size_t) as *mut ::core::ffi::c_char;
    if descending != 0 {
        order = -(1 as ::core::ffi::c_int);
    } else {
        order = 1 as ::core::ffi::c_int;
    }
    k = (count >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int;
    ir = count;
    loop {
        if k > 1 as ::core::ffi::c_int {
            k -= 1;
            memcpy(
                save as *mut ::core::ffi::c_void,
                (attributes as *mut ::core::ffi::c_char).offset((k * recsize) as isize)
                    as *const ::core::ffi::c_void,
                (recsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
        } else {
            hold = (attributes as *mut ::core::ffi::c_char).offset((ir * recsize) as isize);
            memcpy(
                save as *mut ::core::ffi::c_void,
                hold as *const ::core::ffi::c_void,
                (recsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
            memcpy(
                hold as *mut ::core::ffi::c_void,
                base as *const ::core::ffi::c_void,
                (recsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
            ir -= 1;
            if ir == 1 as ::core::ffi::c_int {
                memcpy(
                    base as *mut ::core::ffi::c_void,
                    save as *const ::core::ffi::c_void,
                    (recsize as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                );
                break;
            }
        }
        i = k;
        j = k << 1 as ::core::ffi::c_int;
        while j <= ir {
            hold = (attributes as *mut ::core::ffi::c_char).offset((j * recsize) as isize);
            if j < ir
                && findCompare.expect("non-null function pointer")(
                    hold as *const ::core::ffi::c_void,
                    (attributes as *mut ::core::ffi::c_char)
                        .offset(((j + 1 as ::core::ffi::c_int) * recsize) as isize)
                        as *const ::core::ffi::c_void,
                ) * order
                    < 0 as ::core::ffi::c_int
            {
                hold = hold.offset(recsize as isize);
                j += 1;
            }
            if !(findCompare.expect("non-null function pointer")(
                save as *const ::core::ffi::c_void,
                hold as *const ::core::ffi::c_void,
            ) * order
                < 0 as ::core::ffi::c_int)
            {
                break;
            }
            memcpy(
                (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                    as *mut ::core::ffi::c_void,
                hold as *const ::core::ffi::c_void,
                (recsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
            i = j;
            j <<= 1 as ::core::ffi::c_int;
        }
        memcpy(
            (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                as *mut ::core::ffi::c_void,
            save as *const ::core::ffi::c_void,
            (recsize as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
    }
    if !(save as *mut ::core::ffi::c_void).is_null() {
        free(save as *mut ::core::ffi::c_void);
        save = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[export_name="honest_lpsolve_hpsortex"]
pub unsafe extern "C" fn hpsortex(
    mut attributes: *mut ::core::ffi::c_void,
    mut count: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut recsize: ::core::ffi::c_int,
    mut descending: ::core::ffi::c_uchar,
    mut findCompare: Option<findCompare_func>,
    mut tags: *mut ::core::ffi::c_int,
) {
    if count < 2 as ::core::ffi::c_int {
        return;
    }
    if tags.is_null() {
        hpsort(attributes, count, offset, recsize, descending, findCompare);
        return;
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut ir: ::core::ffi::c_int = 0;
        let mut order: ::core::ffi::c_int = 0;
        let mut hold: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut base: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut save: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut savetag: ::core::ffi::c_int = 0;
        offset -= 1 as ::core::ffi::c_int;
        attributes = (attributes as *mut ::core::ffi::c_char).offset((offset * recsize) as isize)
            as *mut ::core::ffi::c_void;
        tags = tags.offset(offset as isize);
        base = (attributes as *mut ::core::ffi::c_char)
            .offset((1 as ::core::ffi::c_int * recsize) as isize);
        save = malloc(recsize as size_t) as *mut ::core::ffi::c_char;
        if descending != 0 {
            order = -(1 as ::core::ffi::c_int);
        } else {
            order = 1 as ::core::ffi::c_int;
        }
        k = (count >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int;
        ir = count;
        loop {
            if k > 1 as ::core::ffi::c_int {
                k -= 1;
                memcpy(
                    save as *mut ::core::ffi::c_void,
                    (attributes as *mut ::core::ffi::c_char).offset((k * recsize) as isize)
                        as *const ::core::ffi::c_void,
                    (recsize as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                );
                savetag = *tags.offset(k as isize);
            } else {
                hold = (attributes as *mut ::core::ffi::c_char).offset((ir * recsize) as isize);
                memcpy(
                    save as *mut ::core::ffi::c_void,
                    hold as *const ::core::ffi::c_void,
                    (recsize as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                );
                memcpy(
                    hold as *mut ::core::ffi::c_void,
                    base as *const ::core::ffi::c_void,
                    (recsize as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                );
                savetag = *tags.offset(ir as isize);
                *tags.offset(ir as isize) = *tags.offset(1 as ::core::ffi::c_int as isize);
                ir -= 1;
                if ir == 1 as ::core::ffi::c_int {
                    memcpy(
                        base as *mut ::core::ffi::c_void,
                        save as *const ::core::ffi::c_void,
                        (recsize as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                    );
                    *tags.offset(1 as ::core::ffi::c_int as isize) = savetag;
                    break;
                }
            }
            i = k;
            j = k << 1 as ::core::ffi::c_int;
            while j <= ir {
                hold = (attributes as *mut ::core::ffi::c_char).offset((j * recsize) as isize);
                if j < ir
                    && findCompare.expect("non-null function pointer")(
                        hold as *const ::core::ffi::c_void,
                        (attributes as *mut ::core::ffi::c_char)
                            .offset(((j + 1 as ::core::ffi::c_int) * recsize) as isize)
                            as *const ::core::ffi::c_void,
                    ) * order
                        < 0 as ::core::ffi::c_int
                {
                    hold = hold.offset(recsize as isize);
                    j += 1;
                }
                if !(findCompare.expect("non-null function pointer")(
                    save as *const ::core::ffi::c_void,
                    hold as *const ::core::ffi::c_void,
                ) * order
                    < 0 as ::core::ffi::c_int)
                {
                    break;
                }
                memcpy(
                    (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                        as *mut ::core::ffi::c_void,
                    hold as *const ::core::ffi::c_void,
                    (recsize as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                );
                *tags.offset(i as isize) = *tags.offset(j as isize);
                i = j;
                j <<= 1 as ::core::ffi::c_int;
            }
            memcpy(
                (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                    as *mut ::core::ffi::c_void,
                save as *const ::core::ffi::c_void,
                (recsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
            *tags.offset(i as isize) = savetag;
        }
        if !(save as *mut ::core::ffi::c_void).is_null() {
            free(save as *mut ::core::ffi::c_void);
            save = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    };
}
pub const QS_IS_switch: ::core::ffi::c_int = LINEARSEARCH;
#[export_name="honest_lpsolve_qsortex_swap"]
pub unsafe extern "C" fn qsortex_swap(
    mut attributes: *mut ::core::ffi::c_void,
    mut l: ::core::ffi::c_int,
    mut r: ::core::ffi::c_int,
    mut recsize: ::core::ffi::c_int,
    mut tags: *mut ::core::ffi::c_void,
    mut tagsize: ::core::ffi::c_int,
    mut save: *mut ::core::ffi::c_char,
    mut savetag: *mut ::core::ffi::c_char,
) {
    memcpy(
        save as *mut ::core::ffi::c_void,
        (attributes as *mut ::core::ffi::c_char).offset((l * recsize) as isize)
            as *const ::core::ffi::c_void,
        (recsize as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    );
    memcpy(
        (attributes as *mut ::core::ffi::c_char).offset((l * recsize) as isize)
            as *mut ::core::ffi::c_void,
        (attributes as *mut ::core::ffi::c_char).offset((r * recsize) as isize)
            as *const ::core::ffi::c_void,
        (recsize as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    );
    memcpy(
        (attributes as *mut ::core::ffi::c_char).offset((r * recsize) as isize)
            as *mut ::core::ffi::c_void,
        save as *const ::core::ffi::c_void,
        (recsize as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    );
    if !tags.is_null() {
        memcpy(
            savetag as *mut ::core::ffi::c_void,
            (tags as *mut ::core::ffi::c_char).offset((l * tagsize) as isize)
                as *const ::core::ffi::c_void,
            (tagsize as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        memcpy(
            (tags as *mut ::core::ffi::c_char).offset((l * tagsize) as isize)
                as *mut ::core::ffi::c_void,
            (tags as *mut ::core::ffi::c_char).offset((r * tagsize) as isize)
                as *const ::core::ffi::c_void,
            (tagsize as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        memcpy(
            (tags as *mut ::core::ffi::c_char).offset((r * tagsize) as isize)
                as *mut ::core::ffi::c_void,
            savetag as *const ::core::ffi::c_void,
            (tagsize as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
    }
}
#[export_name="honest_lpsolve_qsortex_sort"]
pub unsafe extern "C" fn qsortex_sort(
    mut attributes: *mut ::core::ffi::c_void,
    mut l: ::core::ffi::c_int,
    mut r: ::core::ffi::c_int,
    mut recsize: ::core::ffi::c_int,
    mut sortorder: ::core::ffi::c_int,
    mut findCompare: Option<findCompare_func>,
    mut tags: *mut ::core::ffi::c_void,
    mut tagsize: ::core::ffi::c_int,
    mut save: *mut ::core::ffi::c_char,
    mut savetag: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nmove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if r - l > QS_IS_switch {
        i = (r + l) / 2 as ::core::ffi::c_int;
        if sortorder
            * findCompare.expect("non-null function pointer")(
                (attributes as *mut ::core::ffi::c_char).offset((l * recsize) as isize)
                    as *const ::core::ffi::c_void,
                (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                    as *const ::core::ffi::c_void,
            )
            > 0 as ::core::ffi::c_int
        {
            nmove += 1;
            qsortex_swap(attributes, l, i, recsize, tags, tagsize, save, savetag);
        }
        if sortorder
            * findCompare.expect("non-null function pointer")(
                (attributes as *mut ::core::ffi::c_char).offset((l * recsize) as isize)
                    as *const ::core::ffi::c_void,
                (attributes as *mut ::core::ffi::c_char).offset((r * recsize) as isize)
                    as *const ::core::ffi::c_void,
            )
            > 0 as ::core::ffi::c_int
        {
            nmove += 1;
            qsortex_swap(attributes, l, r, recsize, tags, tagsize, save, savetag);
        }
        if sortorder
            * findCompare.expect("non-null function pointer")(
                (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                    as *const ::core::ffi::c_void,
                (attributes as *mut ::core::ffi::c_char).offset((r * recsize) as isize)
                    as *const ::core::ffi::c_void,
            )
            > 0 as ::core::ffi::c_int
        {
            nmove += 1;
            qsortex_swap(attributes, i, r, recsize, tags, tagsize, save, savetag);
        }
        j = r - 1 as ::core::ffi::c_int;
        qsortex_swap(attributes, i, j, recsize, tags, tagsize, save, savetag);
        i = l;
        v = (attributes as *mut ::core::ffi::c_char).offset((j * recsize) as isize);
        loop {
            loop {
                i += 1;
                if !(sortorder
                    * findCompare.expect("non-null function pointer")(
                        (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                            as *const ::core::ffi::c_void,
                        v as *const ::core::ffi::c_void,
                    )
                    < 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            loop {
                j -= 1;
                if !(sortorder
                    * findCompare.expect("non-null function pointer")(
                        (attributes as *mut ::core::ffi::c_char).offset((j * recsize) as isize)
                            as *const ::core::ffi::c_void,
                        v as *const ::core::ffi::c_void,
                    )
                    > 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if j < i {
                break;
            }
            nmove += 1;
            qsortex_swap(attributes, i, j, recsize, tags, tagsize, save, savetag);
        }
        nmove += 1;
        qsortex_swap(
            attributes,
            i,
            r - 1 as ::core::ffi::c_int,
            recsize,
            tags,
            tagsize,
            save,
            savetag,
        );
        nmove += qsortex_sort(
            attributes,
            l,
            j,
            recsize,
            sortorder,
            findCompare,
            tags,
            tagsize,
            save,
            savetag,
        );
        nmove += qsortex_sort(
            attributes,
            i + 1 as ::core::ffi::c_int,
            r,
            recsize,
            sortorder,
            findCompare,
            tags,
            tagsize,
            save,
            savetag,
        );
    }
    return nmove;
}
#[export_name="honest_lpsolve_qsortex_finish"]
pub unsafe extern "C" fn qsortex_finish(
    mut attributes: *mut ::core::ffi::c_void,
    mut lo0: ::core::ffi::c_int,
    mut hi0: ::core::ffi::c_int,
    mut recsize: ::core::ffi::c_int,
    mut sortorder: ::core::ffi::c_int,
    mut findCompare: Option<findCompare_func>,
    mut tags: *mut ::core::ffi::c_void,
    mut tagsize: ::core::ffi::c_int,
    mut save: *mut ::core::ffi::c_char,
    mut savetag: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nmove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = lo0 + 1 as ::core::ffi::c_int;
    while i <= hi0 {
        memcpy(
            save as *mut ::core::ffi::c_void,
            (attributes as *mut ::core::ffi::c_char).offset((i * recsize) as isize)
                as *const ::core::ffi::c_void,
            (recsize as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        if !tags.is_null() {
            memcpy(
                savetag as *mut ::core::ffi::c_void,
                (tags as *mut ::core::ffi::c_char).offset((i * tagsize) as isize)
                    as *const ::core::ffi::c_void,
                (tagsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
        }
        j = i;
        while j > lo0
            && sortorder
                * findCompare.expect("non-null function pointer")(
                    (attributes as *mut ::core::ffi::c_char)
                        .offset(((j - 1 as ::core::ffi::c_int) * recsize) as isize)
                        as *const ::core::ffi::c_void,
                    save as *const ::core::ffi::c_void,
                )
                > 0 as ::core::ffi::c_int
        {
            memcpy(
                (attributes as *mut ::core::ffi::c_char).offset((j * recsize) as isize)
                    as *mut ::core::ffi::c_void,
                (attributes as *mut ::core::ffi::c_char)
                    .offset(((j - 1 as ::core::ffi::c_int) * recsize) as isize)
                    as *const ::core::ffi::c_void,
                (recsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
            if !tags.is_null() {
                memcpy(
                    (tags as *mut ::core::ffi::c_char).offset((j * tagsize) as isize)
                        as *mut ::core::ffi::c_void,
                    (tags as *mut ::core::ffi::c_char)
                        .offset(((j - 1 as ::core::ffi::c_int) * tagsize) as isize)
                        as *const ::core::ffi::c_void,
                    (tagsize as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                );
            }
            j -= 1;
            nmove += 1;
        }
        memcpy(
            (attributes as *mut ::core::ffi::c_char).offset((j * recsize) as isize)
                as *mut ::core::ffi::c_void,
            save as *const ::core::ffi::c_void,
            (recsize as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        if !tags.is_null() {
            memcpy(
                (tags as *mut ::core::ffi::c_char).offset((j * tagsize) as isize)
                    as *mut ::core::ffi::c_void,
                savetag as *const ::core::ffi::c_void,
                (tagsize as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
        }
        i += 1;
    }
    return nmove;
}
#[export_name="honest_lpsolve_qsortex"]
pub unsafe extern "C" fn qsortex(
    mut attributes: *mut ::core::ffi::c_void,
    mut count: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut recsize: ::core::ffi::c_int,
    mut descending: ::core::ffi::c_uchar,
    mut findCompare: Option<findCompare_func>,
    mut tags: *mut ::core::ffi::c_void,
    mut tagsize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iswaps: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sortorder: ::core::ffi::c_int = if descending as ::core::ffi::c_int != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    let mut save: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut savetag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(count <= 1 as ::core::ffi::c_int) {
        attributes = (attributes as *mut ::core::ffi::c_char).offset((offset * recsize) as isize)
            as *mut ::core::ffi::c_void;
        save = malloc(recsize as size_t) as *mut ::core::ffi::c_char;
        if tagsize <= 0 as ::core::ffi::c_int && !tags.is_null() {
            tags = NULL;
        } else if !tags.is_null() {
            tags = (tags as *mut ::core::ffi::c_char).offset((offset * tagsize) as isize)
                as *mut ::core::ffi::c_void;
            savetag = malloc(tagsize as size_t) as *mut ::core::ffi::c_char;
        }
        count -= 1;
        iswaps = qsortex_sort(
            attributes,
            0 as ::core::ffi::c_int,
            count,
            recsize,
            sortorder,
            findCompare,
            tags,
            tagsize,
            save,
            savetag,
        );
    }
    if !(save as *mut ::core::ffi::c_void).is_null() {
        free(save as *mut ::core::ffi::c_void);
        save = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(savetag as *mut ::core::ffi::c_void).is_null() {
        free(savetag as *mut ::core::ffi::c_void);
        savetag = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return iswaps;
}
pub const QS_IS_switch_0: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[export_name="honest_lpsolve_QS_swap"]
pub unsafe extern "C" fn QS_swap(
    mut a: *mut QSORTrec,
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
) {
    let mut T: QSORTrec = *a.offset(i as isize);
    *a.offset(i as isize) = *a.offset(j as isize);
    *a.offset(j as isize) = T;
}
#[export_name="honest_lpsolve_QS_addfirst"]
pub unsafe extern "C" fn QS_addfirst(
    mut a: *mut QSORTrec,
    mut mydata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let ref mut fresh0 = (*a.offset(0 as ::core::ffi::c_int as isize)).pvoid2.ptr;
    *fresh0 = mydata;
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_QS_append"]
pub unsafe extern "C" fn QS_append(
    mut a: *mut QSORTrec,
    mut ipos: ::core::ffi::c_int,
    mut mydata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if ipos <= 0 as ::core::ffi::c_int {
        ipos = QS_addfirst(a, mydata);
    } else {
        let ref mut fresh1 = (*a.offset(ipos as isize)).pvoid2.ptr;
        *fresh1 = mydata;
    }
    return ipos;
}
#[export_name="honest_lpsolve_QS_replace"]
pub unsafe extern "C" fn QS_replace(
    mut a: *mut QSORTrec,
    mut ipos: ::core::ffi::c_int,
    mut mydata: *mut ::core::ffi::c_void,
) {
    let ref mut fresh2 = (*a.offset(ipos as isize)).pvoid2.ptr;
    *fresh2 = mydata;
}
#[export_name="honest_lpsolve_QS_insert"]
pub unsafe extern "C" fn QS_insert(
    mut a: *mut QSORTrec,
    mut ipos: ::core::ffi::c_int,
    mut mydata: *mut ::core::ffi::c_void,
    mut epos: ::core::ffi::c_int,
) {
    while epos > ipos {
        *a.offset(epos as isize) = *a.offset((epos - 1 as ::core::ffi::c_int) as isize);
        epos -= 1;
    }
    let ref mut fresh3 = (*a.offset(ipos as isize)).pvoid2.ptr;
    *fresh3 = mydata;
}
#[export_name="honest_lpsolve_QS_delete"]
pub unsafe extern "C" fn QS_delete(
    mut a: *mut QSORTrec,
    mut ipos: ::core::ffi::c_int,
    mut epos: ::core::ffi::c_int,
) {
    while epos > ipos {
        *a.offset(epos as isize) = *a.offset((epos - 1 as ::core::ffi::c_int) as isize);
        epos -= 1;
    }
}
#[export_name="honest_lpsolve_QS_sort"]
pub unsafe extern "C" fn QS_sort(
    mut a: *mut QSORTrec,
    mut l: ::core::ffi::c_int,
    mut r: ::core::ffi::c_int,
    mut findCompare: Option<findCompare_func>,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nmove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v: QSORTrec = QSORTrec {
        pvoid2: QSORTrec1 {
            ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ptr2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    };
    if r - l > QS_IS_switch_0 {
        i = (r + l) / 2 as ::core::ffi::c_int;
        if findCompare.expect("non-null function pointer")(
            a.offset(l as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            a.offset(i as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
        ) > 0 as ::core::ffi::c_int
        {
            nmove += 1;
            QS_swap(a, l, i);
        }
        if findCompare.expect("non-null function pointer")(
            a.offset(l as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            a.offset(r as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
        ) > 0 as ::core::ffi::c_int
        {
            nmove += 1;
            QS_swap(a, l, r);
        }
        if findCompare.expect("non-null function pointer")(
            a.offset(i as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            a.offset(r as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
        ) > 0 as ::core::ffi::c_int
        {
            nmove += 1;
            QS_swap(a, i, r);
        }
        j = r - 1 as ::core::ffi::c_int;
        QS_swap(a, i, j);
        i = l;
        v = *a.offset(j as isize);
        loop {
            loop {
                i += 1;
                if !(findCompare.expect("non-null function pointer")(
                    a.offset(i as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    &raw mut v as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ) < 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            loop {
                j -= 1;
                if !(findCompare.expect("non-null function pointer")(
                    a.offset(j as isize) as *mut QSORTrec as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    &raw mut v as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ) > 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if j < i {
                break;
            }
            nmove += 1;
            QS_swap(a, i, j);
        }
        nmove += 1;
        QS_swap(a, i, r - 1 as ::core::ffi::c_int);
        nmove += QS_sort(a, l, j, findCompare);
        nmove += QS_sort(a, i + 1 as ::core::ffi::c_int, r, findCompare);
    }
    return nmove;
}
#[export_name="honest_lpsolve_QS_finish"]
pub unsafe extern "C" fn QS_finish(
    mut a: *mut QSORTrec,
    mut lo0: ::core::ffi::c_int,
    mut hi0: ::core::ffi::c_int,
    mut findCompare: Option<findCompare_func>,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nmove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v: QSORTrec = QSORTrec {
        pvoid2: QSORTrec1 {
            ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ptr2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    };
    i = lo0 + 1 as ::core::ffi::c_int;
    while i <= hi0 {
        v = *a.offset(i as isize);
        j = i;
        while j > lo0
            && findCompare.expect("non-null function pointer")(
                a.offset((j - 1 as ::core::ffi::c_int) as isize) as *mut QSORTrec
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                &raw mut v as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ) > 0 as ::core::ffi::c_int
        {
            *a.offset(j as isize) = *a.offset((j - 1 as ::core::ffi::c_int) as isize);
            j -= 1;
            nmove += 1;
        }
        *a.offset(j as isize) = v;
        i += 1;
    }
    return nmove;
}
#[export_name="honest_lpsolve_QS_execute"]
pub unsafe extern "C" fn QS_execute(
    mut a: *mut QSORTrec,
    mut count: ::core::ffi::c_int,
    mut findCompare: Option<findCompare_func>,
    mut nswaps: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut iswaps: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(count <= 1 as ::core::ffi::c_int) {
        count -= 1;
        iswaps = QS_sort(a, 0 as ::core::ffi::c_int, count, findCompare);
        iswaps += QS_finish(a, 0 as ::core::ffi::c_int, count, findCompare);
    }
    if !nswaps.is_null() {
        *nswaps = iswaps;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_sortByREAL"]
pub unsafe extern "C" fn sortByREAL(
    mut item: *mut ::core::ffi::c_int,
    mut weight: *mut ::core::ffi::c_double,
    mut size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut unique: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut saveI: ::core::ffi::c_int = 0;
    let mut saveW: ::core::ffi::c_double = 0.;
    i = 1 as ::core::ffi::c_int;
    while i < size {
        ii = i + offset - 1 as ::core::ffi::c_int;
        while ii >= offset
            && *weight.offset(ii as isize)
                >= *weight.offset((ii + 1 as ::core::ffi::c_int) as isize)
        {
            if *weight.offset(ii as isize)
                == *weight.offset((ii + 1 as ::core::ffi::c_int) as isize)
            {
                if unique != 0 {
                    return *item.offset(ii as isize);
                }
            } else {
                saveI = *item.offset(ii as isize);
                saveW = *weight.offset(ii as isize);
                *item.offset(ii as isize) = *item.offset((ii + 1 as ::core::ffi::c_int) as isize);
                *weight.offset(ii as isize) =
                    *weight.offset((ii + 1 as ::core::ffi::c_int) as isize);
                *item.offset((ii + 1 as ::core::ffi::c_int) as isize) = saveI;
                *weight.offset((ii + 1 as ::core::ffi::c_int) as isize) = saveW;
            }
            ii -= 1;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_sortByINT"]
pub unsafe extern "C" fn sortByINT(
    mut item: *mut ::core::ffi::c_int,
    mut weight: *mut ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut unique: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut saveI: ::core::ffi::c_int = 0;
    let mut saveW: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    while i < size {
        ii = i + offset - 1 as ::core::ffi::c_int;
        while ii >= offset
            && *weight.offset(ii as isize)
                >= *weight.offset((ii + 1 as ::core::ffi::c_int) as isize)
        {
            if *weight.offset(ii as isize)
                == *weight.offset((ii + 1 as ::core::ffi::c_int) as isize)
            {
                if unique != 0 {
                    return *item.offset(ii as isize);
                }
            } else {
                saveI = *item.offset(ii as isize);
                saveW = *weight.offset(ii as isize);
                *item.offset(ii as isize) = *item.offset((ii + 1 as ::core::ffi::c_int) as isize);
                *weight.offset(ii as isize) =
                    *weight.offset((ii + 1 as ::core::ffi::c_int) as isize);
                *item.offset((ii + 1 as ::core::ffi::c_int) as isize) = saveI;
                *weight.offset((ii + 1 as ::core::ffi::c_int) as isize) = saveW;
            }
            ii -= 1;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_sortREALByINT"]
pub unsafe extern "C" fn sortREALByINT(
    mut item: *mut ::core::ffi::c_double,
    mut weight: *mut ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut unique: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut saveW: ::core::ffi::c_int = 0;
    let mut saveI: ::core::ffi::c_double = 0.;
    i = 1 as ::core::ffi::c_int;
    while i < size {
        ii = i + offset - 1 as ::core::ffi::c_int;
        while ii >= offset
            && *weight.offset(ii as isize)
                >= *weight.offset((ii + 1 as ::core::ffi::c_int) as isize)
        {
            if *weight.offset(ii as isize)
                == *weight.offset((ii + 1 as ::core::ffi::c_int) as isize)
            {
                if unique != 0 {
                    return *item.offset(ii as isize);
                }
            } else {
                saveI = *item.offset(ii as isize);
                saveW = *weight.offset(ii as isize);
                *item.offset(ii as isize) = *item.offset((ii + 1 as ::core::ffi::c_int) as isize);
                *weight.offset(ii as isize) =
                    *weight.offset((ii + 1 as ::core::ffi::c_int) as isize);
                *item.offset((ii + 1 as ::core::ffi::c_int) as isize) = saveI;
                *weight.offset((ii + 1 as ::core::ffi::c_int) as isize) = saveW;
            }
            ii -= 1;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[export_name="honest_lpsolve_timeNow"]
pub unsafe extern "C" fn timeNow() -> ::core::ffi::c_double {
    return 0.0f64;
}
#[export_name="honest_lpsolve_blockWriteINT"]
pub unsafe extern "C" fn blockWriteINT(
    mut output: *mut FILE,
    mut label: *mut ::core::ffi::c_char,
    mut myvector: *mut ::core::ffi::c_int,
    mut first: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    native_only!(fprintf,
        output,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        label,
    );
    native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = first;
    while i <= last {
        native_only!(fprintf,
            output,
            b" %5d\0" as *const u8 as *const ::core::ffi::c_char,
            *myvector.offset(i as isize),
        );
        k += 1;
        if k % 12 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            k = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if k % 12 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[export_name="honest_lpsolve_blockWriteBOOL"]
pub unsafe extern "C" fn blockWriteBOOL(
    mut output: *mut FILE,
    mut label: *mut ::core::ffi::c_char,
    mut myvector: *mut ::core::ffi::c_uchar,
    mut first: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
    mut asRaw: ::core::ffi::c_uchar,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    native_only!(fprintf,
        output,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        label,
    );
    native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = first;
    while i <= last {
        if asRaw != 0 {
            native_only!(fprintf,
                output,
                b" %1d\0" as *const u8 as *const ::core::ffi::c_char,
                *myvector.offset(i as isize) as ::core::ffi::c_int,
            );
        } else {
            native_only!(fprintf,
                output,
                b" %5s\0" as *const u8 as *const ::core::ffi::c_char,
                if *myvector.offset(i as isize) == 0 {
                    b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
        }
        k += 1;
        if k % 36 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            k = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if k % 36 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[export_name="honest_lpsolve_blockWriteREAL"]
pub unsafe extern "C" fn blockWriteREAL(
    mut output: *mut FILE,
    mut label: *mut ::core::ffi::c_char,
    mut myvector: *mut ::core::ffi::c_double,
    mut first: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    native_only!(fprintf,
        output,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        label,
    );
    native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = first;
    while i <= last {
        native_only!(fprintf,
            output,
            b" %18g\0" as *const u8 as *const ::core::ffi::c_char,
            *myvector.offset(i as isize),
        );
        k += 1;
        if k % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            k = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if k % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[export_name="honest_lpsolve_printvec"]
pub unsafe extern "C" fn printvec(
    mut n: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
    mut modulo: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    if modulo <= 0 as ::core::ffi::c_int {
        modulo = 5 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        mod_0(i, modulo) == 1 as ::core::ffi::c_int;
        i += 1;
    }
    i % modulo != 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_printmatUT"]
pub unsafe extern "C" fn printmatUT(
    mut size: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut U: *mut ::core::ffi::c_double,
    mut modulo: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ll: ::core::ffi::c_int = 0;
    ll = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        printvec(
            n - i + 1 as ::core::ffi::c_int,
            U.offset(ll as isize) as *mut ::core::ffi::c_double,
            modulo,
        );
        ll += size - i + 1 as ::core::ffi::c_int;
        i += 1;
    }
}
#[export_name="honest_lpsolve_printmatSQ"]
pub unsafe extern "C" fn printmatSQ(
    mut size: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut X: *mut ::core::ffi::c_double,
    mut modulo: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ll: ::core::ffi::c_int = 0;
    ll = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        printvec(
            n,
            X.offset(ll as isize) as *mut ::core::ffi::c_double,
            modulo,
        );
        ll += size;
        i += 1;
    }
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LINEARSEARCH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
