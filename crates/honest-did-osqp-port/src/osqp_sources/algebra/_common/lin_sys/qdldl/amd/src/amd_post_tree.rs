pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[export_name = "honest_osqp_amd_post_tree"]
pub unsafe extern "C" fn amd_post_tree(
    mut root: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut Child: *mut ::core::ffi::c_int,
    mut Sibling: *const ::core::ffi::c_int,
    mut Order: *mut ::core::ffi::c_int,
    mut Stack: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut f: ::core::ffi::c_int = 0;
    let mut head: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    head = 0 as ::core::ffi::c_int;
    *Stack.offset(0 as ::core::ffi::c_int as isize) = root;
    while head >= 0 as ::core::ffi::c_int {
        i = *Stack.offset(head as isize);
        if *Child.offset(i as isize) != EMPTY {
            f = *Child.offset(i as isize);
            while f != EMPTY {
                head += 1;
                f = *Sibling.offset(f as isize);
            }
            h = head;
            f = *Child.offset(i as isize);
            while f != EMPTY {
                let fresh0 = h;
                h = h - 1;
                *Stack.offset(fresh0 as isize) = f;
                f = *Sibling.offset(f as isize);
            }
            *Child.offset(i as isize) = EMPTY;
        } else {
            head -= 1;
            let fresh1 = k;
            k = k + 1;
            *Order.offset(i as isize) = fresh1;
        }
    }
    return k;
}
