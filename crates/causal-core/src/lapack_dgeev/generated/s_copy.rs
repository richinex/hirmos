#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type ftnlen = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_s_copy(
    mut a: *mut ::core::ffi::c_char,
    mut b: *mut ::core::ffi::c_char,
    mut la: ftnlen,
    mut lb: ftnlen,
) {
    let mut aend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    aend = a.offset(la as isize);
    if la <= lb {
        if a <= b || a >= b.offset(la as isize) {
            while a < aend {
                let fresh0 = b;
                b = b.offset(1);
                let fresh1 = a;
                a = a.offset(1);
                *fresh1 = *fresh0;
            }
        } else {
            b = b.offset(la as isize);
            while a < aend {
                b = b.offset(-1);
                aend = aend.offset(-1);
                *aend = *b;
            }
        }
    } else {
        bend = b.offset(lb as isize);
        if a <= b || a >= bend {
            while b < bend {
                let fresh2 = b;
                b = b.offset(1);
                let fresh3 = a;
                a = a.offset(1);
                *fresh3 = *fresh2;
            }
        } else {
            a = a.offset(lb as isize);
            while b < bend {
                bend = bend.offset(-1);
                a = a.offset(-1);
                *a = *bend;
            }
            a = a.offset(lb as isize);
        }
        while a < aend {
            let fresh4 = a;
            a = a.offset(1);
            *fresh4 = ' ' as i32 as ::core::ffi::c_char;
        }
    };
}
