pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn amd_l_post_tree(
    mut root: i64,
    mut k: i64,
    mut Child: *mut i64,
    mut Sibling: *const i64,
    mut Order: *mut i64,
    mut Stack: *mut i64,
) -> i64 {
    let mut f: i64 = 0;
    let mut head: i64 = 0;
    let mut h: i64 = 0;
    let mut i: i64 = 0;
    head = 0 as i64;
    *Stack.offset(0 as ::core::ffi::c_int as isize) = root;
    while head >= 0 as i64 {
        i = *Stack.offset(head as isize);
        if *Child.offset(i as isize) != EMPTY as i64 {
            f = *Child.offset(i as isize);
            while f != EMPTY as i64 {
                head += 1;
                f = *Sibling.offset(f as isize);
            }
            h = head;
            f = *Child.offset(i as isize);
            while f != EMPTY as i64 {
                let fresh0 = h;
                h = h - 1;
                *Stack.offset(fresh0 as isize) = f;
                f = *Sibling.offset(f as isize);
            }
            *Child.offset(i as isize) = EMPTY as i64;
        } else {
            head -= 1;
            let fresh1 = k;
            k = k + 1;
            *Order.offset(i as isize) = fresh1;
        }
    }
    return k;
}
