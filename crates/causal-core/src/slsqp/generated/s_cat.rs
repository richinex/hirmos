pub type ftnlen = ::core::ffi::c_long;
pub type ftnint = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_s_cat(
    mut lp: *mut ::core::ffi::c_char,
    mut rpp: *mut *mut ::core::ffi::c_char,
    mut rnp: *mut ftnint,
    mut np: *mut ftnint,
    mut ll: ftnlen,
) {
    let mut i: ftnlen = 0;
    let mut nc: ftnlen = 0;
    let mut rp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ftnlen = *np;
    i = 0 as ftnlen;
    while i < n {
        nc = ll;
        if *rnp.offset(i as isize) < nc {
            nc = *rnp.offset(i as isize) as ftnlen;
        }
        ll -= nc as ::core::ffi::c_long;
        rp = *rpp.offset(i as isize);
        loop {
            nc -= 1;
            if !(nc >= 0 as ::core::ffi::c_long) {
                break;
            }
            let fresh0 = rp;
            rp = rp.offset(1);
            let fresh1 = lp;
            lp = lp.offset(1);
            *fresh1 = *fresh0;
        }
        i += 1;
    }
    loop {
        ll -= 1;
        if !(ll >= 0 as ::core::ffi::c_long) {
            break;
        }
        let fresh2 = lp;
        lp = lp.offset(1);
        *fresh2 = ' ' as i32 as ::core::ffi::c_char;
    }
}
