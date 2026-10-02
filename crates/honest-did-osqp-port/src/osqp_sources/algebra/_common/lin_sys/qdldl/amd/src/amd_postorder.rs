extern "C" {
    #[link_name = "honest_osqp_amd_post_tree"]
    fn amd_post_tree(
        root: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        Child: *mut ::core::ffi::c_int,
        Sibling: *const ::core::ffi::c_int,
        Order: *mut ::core::ffi::c_int,
        Stack: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[export_name = "honest_osqp_amd_postorder"]
pub unsafe extern "C" fn amd_postorder(
    mut nn: ::core::ffi::c_int,
    mut Parent: *mut ::core::ffi::c_int,
    mut Nv: *mut ::core::ffi::c_int,
    mut Fsize: *mut ::core::ffi::c_int,
    mut Order: *mut ::core::ffi::c_int,
    mut Child: *mut ::core::ffi::c_int,
    mut Sibling: *mut ::core::ffi::c_int,
    mut Stack: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut parent: ::core::ffi::c_int = 0;
    let mut frsize: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_int = 0;
    let mut fprev: ::core::ffi::c_int = 0;
    let mut maxfrsize: ::core::ffi::c_int = 0;
    let mut bigfprev: ::core::ffi::c_int = 0;
    let mut bigf: ::core::ffi::c_int = 0;
    let mut fnext: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < nn {
        *Child.offset(j as isize) = EMPTY;
        *Sibling.offset(j as isize) = EMPTY;
        j += 1;
    }
    j = nn - 1 as ::core::ffi::c_int;
    while j >= 0 as ::core::ffi::c_int {
        if *Nv.offset(j as isize) > 0 as ::core::ffi::c_int {
            parent = *Parent.offset(j as isize);
            if parent != EMPTY {
                *Sibling.offset(j as isize) = *Child.offset(parent as isize);
                *Child.offset(parent as isize) = j;
            }
        }
        j -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nn {
        if *Nv.offset(i as isize) > 0 as ::core::ffi::c_int && *Child.offset(i as isize) != EMPTY {
            fprev = EMPTY;
            maxfrsize = EMPTY;
            bigfprev = EMPTY;
            bigf = EMPTY;
            f = *Child.offset(i as isize);
            while f != EMPTY {
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
            if fnext != EMPTY {
                if bigfprev == EMPTY {
                    *Child.offset(i as isize) = fnext;
                } else {
                    *Sibling.offset(bigfprev as isize) = fnext;
                }
                *Sibling.offset(bigf as isize) = EMPTY;
                *Sibling.offset(fprev as isize) = bigf;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nn {
        *Order.offset(i as isize) = EMPTY;
        i += 1;
    }
    k = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nn {
        if *Parent.offset(i as isize) == EMPTY && *Nv.offset(i as isize) > 0 as ::core::ffi::c_int {
            k = amd_post_tree(
                i,
                k,
                Child,
                Sibling as *const ::core::ffi::c_int,
                Order,
                Stack,
            );
        }
        i += 1;
    }
}
