extern "C" {
    fn amd_l_post_tree(
        root: i64,
        k: i64,
        Child: *mut i64,
        Sibling: *const i64,
        Order: *mut i64,
        Stack: *mut i64,
    ) -> i64;
}
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn amd_l_postorder(
    mut nn: i64,
    mut Parent: *mut i64,
    mut Nv: *mut i64,
    mut Fsize: *mut i64,
    mut Order: *mut i64,
    mut Child: *mut i64,
    mut Sibling: *mut i64,
    mut Stack: *mut i64,
) {
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut k: i64 = 0;
    let mut parent: i64 = 0;
    let mut frsize: i64 = 0;
    let mut f: i64 = 0;
    let mut fprev: i64 = 0;
    let mut maxfrsize: i64 = 0;
    let mut bigfprev: i64 = 0;
    let mut bigf: i64 = 0;
    let mut fnext: i64 = 0;
    j = 0 as i64;
    while j < nn {
        *Child.offset(j as isize) = EMPTY as i64;
        *Sibling.offset(j as isize) = EMPTY as i64;
        j += 1;
    }
    j = nn - 1 as i64;
    while j >= 0 as i64 {
        if *Nv.offset(j as isize) > 0 as i64 {
            parent = *Parent.offset(j as isize);
            if parent != EMPTY as i64 {
                *Sibling.offset(j as isize) = *Child.offset(parent as isize);
                *Child.offset(parent as isize) = j;
            }
        }
        j -= 1;
    }
    i = 0 as i64;
    while i < nn {
        if *Nv.offset(i as isize) > 0 as i64
            && *Child.offset(i as isize) != EMPTY as i64
        {
            fprev = EMPTY as i64;
            maxfrsize = EMPTY as i64;
            bigfprev = EMPTY as i64;
            bigf = EMPTY as i64;
            f = *Child.offset(i as isize);
            while f != EMPTY as i64 {
                frsize = *Fsize.offset(f as isize);
                if frsize >= maxfrsize {
                    maxfrsize = frsize;
                    bigfprev = fprev;
                    bigf = f;
                }
                fprev = f;
                f = *Sibling.offset(f as isize);
            }
            fnext = *Sibling.offset(bigf as isize);
            if fnext != EMPTY as i64 {
                if bigfprev == EMPTY as i64 {
                    *Child.offset(i as isize) = fnext;
                } else {
                    *Sibling.offset(bigfprev as isize) = fnext;
                }
                *Sibling.offset(bigf as isize) = EMPTY as i64;
                *Sibling.offset(fprev as isize) = bigf;
            }
        }
        i += 1;
    }
    i = 0 as i64;
    while i < nn {
        *Order.offset(i as isize) = EMPTY as i64;
        i += 1;
    }
    k = 0 as i64;
    i = 0 as i64;
    while i < nn {
        if *Parent.offset(i as isize) == EMPTY as i64
            && *Nv.offset(i as isize) > 0 as i64
        {
            k = amd_l_post_tree(
                i,
                k,
                Child,
                Sibling as *const i64,
                Order,
                Stack,
            );
        }
        i += 1;
    }
}
