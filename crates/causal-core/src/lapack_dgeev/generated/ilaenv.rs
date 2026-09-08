#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type integer = ::core::ffi::c_long;
pub type real = ::core::ffi::c_float;
pub type logical = ::core::ffi::c_long;
pub type ftnlen = ::core::ffi::c_long;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
static mut c__1: integer = 1 as integer;
static mut c_b163: real = 0.0f32;
static mut c_b164: real = 1.0f32;
static mut c__0: integer = 0 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_ilaenv_(
    mut ispec: *mut integer,
    mut name__: *mut ::core::ffi::c_char,
    mut opts: *mut ::core::ffi::c_char,
    mut n1: *mut integer,
    mut n2: *mut integer,
    mut n3: *mut integer,
    mut n4: *mut integer,
) -> integer {
    let mut ret_val: integer = 0;
    extern "C" {
        #[link_name = "dgeev_closure_s_copy"]
        fn s_copy_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: ftnlen,
            _: ftnlen,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_s_cmp"]
        fn s_cmp_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: ftnlen,
            _: ftnlen,
        ) -> integer;
    }
    let mut i__: integer = 0;
    let mut c1: [::core::ffi::c_char; 1] = [0; 1];
    let mut c2: [::core::ffi::c_char; 1] = [0; 1];
    let mut c3: [::core::ffi::c_char; 1] = [0; 1];
    let mut c4: [::core::ffi::c_char; 1] = [0; 1];
    let mut ic: integer = 0;
    let mut nb: integer = 0;
    let mut iz: integer = 0;
    let mut nx: integer = 0;
    let mut cname: logical = 0;
    let mut nbmin: integer = 0;
    let mut sname: logical = 0;
    extern "C" {
        #[link_name = "dgeev_closure_ieeeck_"]
        fn ieeeck__0(_: *mut integer, _: *mut real, _: *mut real) -> integer;
    }
    let mut subnam: [::core::ffi::c_char; 1] = [0; 1];
    extern "C" {
        #[link_name = "dgeev_closure_iparmq_"]
        fn iparmq__0(
            _: *mut integer,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> integer;
    }
    let mut name_len: ftnlen = 0;
    let mut opts_len: ftnlen = 0;
    name_len = strlen(name__) as ftnlen;
    opts_len = strlen(opts) as ftnlen;
    match *ispec {
        1 | 2 | 3 => {
            ret_val = 1 as integer;
            s_copy_0(
                &raw mut subnam as *mut ::core::ffi::c_char,
                name__,
                1 as ::core::ffi::c_int as ftnlen,
                name_len,
            );
            ic = *(&raw mut subnam as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                as integer;
            iz = 'Z' as i32 as integer;
            if iz == 90 as ::core::ffi::c_long || iz == 122 as ::core::ffi::c_long {
                if ic >= 97 as ::core::ffi::c_long && ic <= 122 as ::core::ffi::c_long {
                    *(&raw mut subnam as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                        (ic as ::core::ffi::c_long - 32 as ::core::ffi::c_long)
                            as ::core::ffi::c_char as ::core::ffi::c_uchar;
                    i__ = 2 as integer;
                    while i__ <= 6 as ::core::ffi::c_long {
                        ic = *((&raw mut subnam as *mut ::core::ffi::c_char).offset(
                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar) as integer;
                        if ic >= 97 as ::core::ffi::c_long && ic <= 122 as ::core::ffi::c_long {
                            *((&raw mut subnam as *mut ::core::ffi::c_char).offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar) = (ic as ::core::ffi::c_long
                                - 32 as ::core::ffi::c_long)
                                as ::core::ffi::c_char
                                as ::core::ffi::c_uchar;
                        }
                        i__ += 1;
                    }
                }
            } else if iz == 233 as ::core::ffi::c_long || iz == 169 as ::core::ffi::c_long {
                if ic >= 129 as ::core::ffi::c_long && ic <= 137 as ::core::ffi::c_long
                    || ic >= 145 as ::core::ffi::c_long && ic <= 153 as ::core::ffi::c_long
                    || ic >= 162 as ::core::ffi::c_long && ic <= 169 as ::core::ffi::c_long
                {
                    *(&raw mut subnam as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                        (ic as ::core::ffi::c_long + 64 as ::core::ffi::c_long)
                            as ::core::ffi::c_char as ::core::ffi::c_uchar;
                    i__ = 2 as integer;
                    while i__ <= 6 as ::core::ffi::c_long {
                        ic = *((&raw mut subnam as *mut ::core::ffi::c_char).offset(
                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar) as integer;
                        if ic >= 129 as ::core::ffi::c_long && ic <= 137 as ::core::ffi::c_long
                            || ic >= 145 as ::core::ffi::c_long && ic <= 153 as ::core::ffi::c_long
                            || ic >= 162 as ::core::ffi::c_long && ic <= 169 as ::core::ffi::c_long
                        {
                            *((&raw mut subnam as *mut ::core::ffi::c_char).offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar) = (ic as ::core::ffi::c_long
                                + 64 as ::core::ffi::c_long)
                                as ::core::ffi::c_char
                                as ::core::ffi::c_uchar;
                        }
                        i__ += 1;
                    }
                }
            } else if iz == 218 as ::core::ffi::c_long || iz == 250 as ::core::ffi::c_long {
                if ic >= 225 as ::core::ffi::c_long && ic <= 250 as ::core::ffi::c_long {
                    *(&raw mut subnam as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                        (ic as ::core::ffi::c_long - 32 as ::core::ffi::c_long)
                            as ::core::ffi::c_char as ::core::ffi::c_uchar;
                    i__ = 2 as integer;
                    while i__ <= 6 as ::core::ffi::c_long {
                        ic = *((&raw mut subnam as *mut ::core::ffi::c_char).offset(
                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar) as integer;
                        if ic >= 225 as ::core::ffi::c_long && ic <= 250 as ::core::ffi::c_long {
                            *((&raw mut subnam as *mut ::core::ffi::c_char).offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar) = (ic as ::core::ffi::c_long
                                - 32 as ::core::ffi::c_long)
                                as ::core::ffi::c_char
                                as ::core::ffi::c_uchar;
                        }
                        i__ += 1;
                    }
                }
            }
            *(&raw mut c1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                *(&raw mut subnam as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar);
            sname = (*(&raw mut c1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                as ::core::ffi::c_int
                == 'S' as i32
                || *(&raw mut c1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                    as ::core::ffi::c_int
                    == 'D' as i32) as ::core::ffi::c_int as logical;
            cname = (*(&raw mut c1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                as ::core::ffi::c_int
                == 'C' as i32
                || *(&raw mut c1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                    as ::core::ffi::c_int
                    == 'Z' as i32) as ::core::ffi::c_int as logical;
            if !(cname != 0 || sname != 0) {
                return ret_val;
            }
            s_copy_0(
                &raw mut c2 as *mut ::core::ffi::c_char,
                (&raw mut subnam as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                1 as ::core::ffi::c_int as ftnlen,
                2 as ::core::ffi::c_int as ftnlen,
            );
            s_copy_0(
                &raw mut c3 as *mut ::core::ffi::c_char,
                (&raw mut subnam as *mut ::core::ffi::c_char)
                    .offset(3 as ::core::ffi::c_int as isize),
                1 as ::core::ffi::c_int as ftnlen,
                3 as ::core::ffi::c_int as ftnlen,
            );
            s_copy_0(
                &raw mut c4 as *mut ::core::ffi::c_char,
                (&raw mut c3 as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
                1 as ::core::ffi::c_int as ftnlen,
                2 as ::core::ffi::c_int as ftnlen,
            );
            match *ispec {
                2 => {
                    nbmin = 2 as integer;
                    if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"GE\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"QRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"RQF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"LQF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"QLF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nbmin = 2 as integer;
                            } else {
                                nbmin = 2 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"HRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nbmin = 2 as integer;
                            } else {
                                nbmin = 2 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"BRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nbmin = 2 as integer;
                            } else {
                                nbmin = 2 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRI\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nbmin = 2 as integer;
                            } else {
                                nbmin = 2 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"SY\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nbmin = 8 as integer;
                            } else {
                                nbmin = 8 as integer;
                            }
                        } else if sname != 0
                            && s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"TRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                        {
                            nbmin = 2 as integer;
                        }
                    } else if cname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"HE\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            nbmin = 2 as integer;
                        }
                    } else if sname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"OR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if *(&raw mut c3 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'G' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nbmin = 2 as integer;
                            }
                        } else if *(&raw mut c3 as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'M' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nbmin = 2 as integer;
                            }
                        }
                    } else if cname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"UN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if *(&raw mut c3 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'G' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nbmin = 2 as integer;
                            }
                        } else if *(&raw mut c3 as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'M' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nbmin = 2 as integer;
                            }
                        }
                    }
                    ret_val = nbmin;
                    return ret_val;
                }
                3 => {
                    nx = 0 as integer;
                    if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"GE\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"QRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"RQF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"LQF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"QLF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nx = 128 as integer;
                            } else {
                                nx = 128 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"HRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nx = 128 as integer;
                            } else {
                                nx = 128 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"BRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nx = 128 as integer;
                            } else {
                                nx = 128 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"SY\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if sname != 0
                            && s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"TRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                        {
                            nx = 32 as integer;
                        }
                    } else if cname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"HE\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            nx = 32 as integer;
                        }
                    } else if sname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"OR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if *(&raw mut c3 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'G' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nx = 128 as integer;
                            }
                        }
                    } else if cname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"UN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if *(&raw mut c3 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'G' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nx = 128 as integer;
                            }
                        }
                    }
                    ret_val = nx;
                    return ret_val;
                }
                1 | _ => {
                    nb = 1 as integer;
                    if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"GE\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 64 as integer;
                            } else {
                                nb = 64 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"QRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"RQF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"LQF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                            || s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"QLF\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 32 as integer;
                            } else {
                                nb = 32 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"HRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 32 as integer;
                            } else {
                                nb = 32 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"BRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 32 as integer;
                            } else {
                                nb = 32 as integer;
                            }
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRI\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 64 as integer;
                            } else {
                                nb = 64 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"PO\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 64 as integer;
                            } else {
                                nb = 64 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"SY\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 64 as integer;
                            } else {
                                nb = 64 as integer;
                            }
                        } else if sname != 0
                            && s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"TRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                        {
                            nb = 32 as integer;
                        } else if sname != 0
                            && s_cmp_0(
                                &raw mut c3 as *mut ::core::ffi::c_char,
                                b"GST\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                3 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                        {
                            nb = 64 as integer;
                        }
                    } else if cname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"HE\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            nb = 64 as integer;
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            nb = 32 as integer;
                        } else if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"GST\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            nb = 64 as integer;
                        }
                    } else if sname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"OR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if *(&raw mut c3 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'G' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nb = 32 as integer;
                            }
                        } else if *(&raw mut c3 as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'M' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nb = 32 as integer;
                            }
                        }
                    } else if cname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"UN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if *(&raw mut c3 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'G' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nb = 32 as integer;
                            }
                        } else if *(&raw mut c3 as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_int
                            == 'M' as i32
                        {
                            if s_cmp_0(
                                &raw mut c4 as *mut ::core::ffi::c_char,
                                b"QR\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                1 as ::core::ffi::c_int as ftnlen,
                                2 as ::core::ffi::c_int as ftnlen,
                            ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"RQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"LQ\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"QL\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"HR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"TR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                                || s_cmp_0(
                                    &raw mut c4 as *mut ::core::ffi::c_char,
                                    b"BR\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int as ftnlen,
                                    2 as ::core::ffi::c_int as ftnlen,
                                ) == 0 as ::core::ffi::c_long
                            {
                                nb = 32 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"GB\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                if *n4 <= 64 as ::core::ffi::c_long {
                                    nb = 1 as integer;
                                } else {
                                    nb = 32 as integer;
                                }
                            } else if *n4 <= 64 as ::core::ffi::c_long {
                                nb = 1 as integer;
                            } else {
                                nb = 32 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"PB\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                if *n2 <= 64 as ::core::ffi::c_long {
                                    nb = 1 as integer;
                                } else {
                                    nb = 32 as integer;
                                }
                            } else if *n2 <= 64 as ::core::ffi::c_long {
                                nb = 1 as integer;
                            } else {
                                nb = 32 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"TR\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"TRI\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 64 as integer;
                            } else {
                                nb = 64 as integer;
                            }
                        }
                    } else if s_cmp_0(
                        &raw mut c2 as *mut ::core::ffi::c_char,
                        b"LA\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int as ftnlen,
                        2 as ::core::ffi::c_int as ftnlen,
                    ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"UUM\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            if sname != 0 {
                                nb = 64 as integer;
                            } else {
                                nb = 64 as integer;
                            }
                        }
                    } else if sname != 0
                        && s_cmp_0(
                            &raw mut c2 as *mut ::core::ffi::c_char,
                            b"ST\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            2 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                    {
                        if s_cmp_0(
                            &raw mut c3 as *mut ::core::ffi::c_char,
                            b"EBZ\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            1 as ::core::ffi::c_int as ftnlen,
                            3 as ::core::ffi::c_int as ftnlen,
                        ) == 0 as ::core::ffi::c_long
                        {
                            nb = 1 as integer;
                        }
                    }
                    ret_val = nb;
                    return ret_val;
                }
            }
        }
        4 => {
            ret_val = 6 as integer;
            return ret_val;
        }
        5 => {
            ret_val = 2 as integer;
            return ret_val;
        }
        6 => {
            ret_val =
                ((if *n1 <= *n2 { *n1 } else { *n2 }) as ::core::ffi::c_float * 1.6f32) as integer;
            return ret_val;
        }
        7 => {
            ret_val = 1 as integer;
            return ret_val;
        }
        8 => {
            ret_val = 50 as integer;
            return ret_val;
        }
        9 => {
            ret_val = 25 as integer;
            return ret_val;
        }
        10 => {
            ret_val = 1 as integer;
            if ret_val == 1 as ::core::ffi::c_long {
                ret_val = ieeeck__0(&raw mut c__1, &raw mut c_b163, &raw mut c_b164);
            }
            return ret_val;
        }
        11 => {
            ret_val = 1 as integer;
            if ret_val == 1 as ::core::ffi::c_long {
                ret_val = ieeeck__0(&raw mut c__0, &raw mut c_b163, &raw mut c_b164);
            }
            return ret_val;
        }
        12 | 13 | 14 | 15 | 16 => {
            ret_val = iparmq__0(ispec, name__, opts, n1, n2, n3, n4);
            return ret_val;
        }
        _ => {
            ret_val = -(1 as ::core::ffi::c_int) as integer;
            return ret_val;
        }
    };
}
