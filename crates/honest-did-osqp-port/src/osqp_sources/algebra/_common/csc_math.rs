pub type OSQPInt = ::core::ffi::c_int;
pub type OSQPFloat = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPCscMatrix {
    pub m: OSQPInt,
    pub n: OSQPInt,
    pub p: *mut OSQPInt,
    pub i: *mut OSQPInt,
    pub x: *mut OSQPFloat,
    pub nzmax: OSQPInt,
    pub nz: OSQPInt,
    pub owned: OSQPInt,
}
#[export_name = "honest_osqp_vec_set_scalar"]
pub unsafe extern "C" fn vec_set_scalar(mut v: *mut OSQPFloat, mut val: OSQPFloat, mut n: OSQPInt) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        *v.offset(i as isize) = val;
        i += 1;
    }
}
#[export_name = "honest_osqp_vec_mult_scalar"]
pub unsafe extern "C" fn vec_mult_scalar(
    mut v: *mut OSQPFloat,
    mut val: OSQPFloat,
    mut n: OSQPInt,
) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        let ref mut fresh0 = *v.offset(i as isize);
        *fresh0 *= val as ::core::ffi::c_double;
        i += 1;
    }
}
#[export_name = "honest_osqp_vec_negate"]
pub unsafe extern "C" fn vec_negate(mut v: *mut OSQPFloat, mut n: OSQPInt) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        *v.offset(i as isize) = -*v.offset(i as isize);
        i += 1;
    }
}
#[export_name = "honest_osqp_csc_update_values"]
pub unsafe extern "C" fn csc_update_values(
    mut M: *mut OSQPCscMatrix,
    mut Mx_new: *const OSQPFloat,
    mut Mx_new_idx: *const OSQPInt,
    mut M_new_n: OSQPInt,
) {
    let mut i: OSQPInt = 0;
    if !Mx_new_idx.is_null() {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < M_new_n {
            *(*M).x.offset(*Mx_new_idx.offset(i as isize) as isize) = *Mx_new.offset(i as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < M_new_n {
            *(*M).x.offset(i as isize) = *Mx_new.offset(i as isize);
            i += 1;
        }
    };
}
#[export_name = "honest_osqp_csc_scale"]
pub unsafe extern "C" fn csc_scale(mut A: *mut OSQPCscMatrix, mut sc: OSQPFloat) {
    let mut i: OSQPInt = 0;
    let mut nnzA: OSQPInt = 0;
    nnzA = *(*A).p.offset((*A).n as isize);
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < nnzA {
        let ref mut fresh1 = *(*A).x.offset(i as isize);
        *fresh1 *= sc as ::core::ffi::c_double;
        i += 1;
    }
}
#[export_name = "honest_osqp_csc_lmult_diag"]
pub unsafe extern "C" fn csc_lmult_diag(mut A: *mut OSQPCscMatrix, mut d: *const OSQPFloat) {
    let mut j: OSQPInt = 0;
    let mut i: OSQPInt = 0;
    let mut n: OSQPInt = (*A).n;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ai: *mut OSQPInt = (*A).i;
    let mut Ax: *mut OSQPFloat = (*A).x;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        i = *Ap.offset(j as isize);
        while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            let ref mut fresh2 = *Ax.offset(i as isize);
            *fresh2 *= *d.offset(*Ai.offset(i as isize) as isize) as ::core::ffi::c_double;
            i += 1;
        }
        j += 1;
    }
}
#[export_name = "honest_osqp_csc_rmult_diag"]
pub unsafe extern "C" fn csc_rmult_diag(mut A: *mut OSQPCscMatrix, mut d: *const OSQPFloat) {
    let mut j: OSQPInt = 0;
    let mut i: OSQPInt = 0;
    let mut n: OSQPInt = (*A).n;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ax: *mut OSQPFloat = (*A).x;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        i = *Ap.offset(j as isize);
        while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            let ref mut fresh3 = *Ax.offset(i as isize);
            *fresh3 *= *d.offset(j as isize) as ::core::ffi::c_double;
            i += 1;
        }
        j += 1;
    }
}
#[export_name = "honest_osqp_csc_AtDA_extract_diag"]
pub unsafe extern "C" fn csc_AtDA_extract_diag(
    mut A: *const OSQPCscMatrix,
    mut D: *const OSQPFloat,
    mut d: *mut OSQPFloat,
) {
    let mut j: OSQPInt = 0;
    let mut i: OSQPInt = 0;
    let mut n: OSQPInt = (*A).n;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ai: *mut OSQPInt = (*A).i;
    let mut Ax: *mut OSQPFloat = (*A).x;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        *d.offset(j as isize) = 0 as ::core::ffi::c_int as OSQPFloat;
        i = *Ap.offset(j as isize);
        while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            let ref mut fresh4 = *d.offset(j as isize);
            *fresh4 += (*Ax.offset(i as isize)
                * *Ax.offset(i as isize)
                * *D.offset(*Ai.offset(i as isize) as isize))
                as ::core::ffi::c_double;
            i += 1;
        }
        j += 1;
    }
}
#[export_name = "honest_osqp_csc_Axpy_sym_triu"]
pub unsafe extern "C" fn csc_Axpy_sym_triu(
    mut A: *const OSQPCscMatrix,
    mut x: *const OSQPFloat,
    mut y: *mut OSQPFloat,
    mut alpha: OSQPFloat,
    mut beta: OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ai: *mut OSQPInt = (*A).i;
    let mut An: OSQPInt = (*A).n;
    let mut Am: OSQPInt = (*A).m;
    let mut Ax: *mut OSQPFloat = (*A).x;
    if beta == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        vec_set_scalar(y, 0.0f64, Am);
    } else if !(beta == 1 as ::core::ffi::c_int as ::core::ffi::c_double) {
        if beta == -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
            vec_negate(y, Am);
        } else {
            vec_mult_scalar(y, beta, Am);
        }
    }
    if *Ap.offset(An as isize) == 0 as ::core::ffi::c_int || alpha == 0.0f64 {
        return;
    }
    if alpha == -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < An {
            i = *Ap.offset(j as isize);
            while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh5 = *y.offset(*Ai.offset(i as isize) as isize);
                *fresh5 -=
                    (*Ax.offset(i as isize) * *x.offset(j as isize)) as ::core::ffi::c_double;
                if *Ai.offset(i as isize) != j {
                    let ref mut fresh6 = *y.offset(j as isize);
                    *fresh6 -= (*Ax.offset(i as isize) * *x.offset(*Ai.offset(i as isize) as isize))
                        as ::core::ffi::c_double;
                }
                i += 1;
            }
            j += 1;
        }
    } else if alpha == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < An {
            i = *Ap.offset(j as isize);
            while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh7 = *y.offset(*Ai.offset(i as isize) as isize);
                *fresh7 +=
                    (*Ax.offset(i as isize) * *x.offset(j as isize)) as ::core::ffi::c_double;
                if *Ai.offset(i as isize) != j {
                    let ref mut fresh8 = *y.offset(j as isize);
                    *fresh8 += (*Ax.offset(i as isize) * *x.offset(*Ai.offset(i as isize) as isize))
                        as ::core::ffi::c_double;
                }
                i += 1;
            }
            j += 1;
        }
    } else {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < An {
            i = *Ap.offset(j as isize);
            while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh9 = *y.offset(*Ai.offset(i as isize) as isize);
                *fresh9 += (alpha * *Ax.offset(i as isize) * *x.offset(j as isize))
                    as ::core::ffi::c_double;
                if *Ai.offset(i as isize) != j {
                    let ref mut fresh10 = *y.offset(j as isize);
                    *fresh10 += (alpha
                        * *Ax.offset(i as isize)
                        * *x.offset(*Ai.offset(i as isize) as isize))
                        as ::core::ffi::c_double;
                }
                i += 1;
            }
            j += 1;
        }
    };
}
#[export_name = "honest_osqp_csc_Axpy"]
pub unsafe extern "C" fn csc_Axpy(
    mut A: *const OSQPCscMatrix,
    mut x: *const OSQPFloat,
    mut y: *mut OSQPFloat,
    mut alpha: OSQPFloat,
    mut beta: OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ai: *mut OSQPInt = (*A).i;
    let mut An: OSQPInt = (*A).n;
    let mut Am: OSQPInt = (*A).m;
    let mut Ax: *mut OSQPFloat = (*A).x;
    if beta == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        vec_set_scalar(y, 0.0f64, Am);
    } else if !(beta == 1 as ::core::ffi::c_int as ::core::ffi::c_double) {
        if beta == -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
            vec_negate(y, Am);
        } else {
            vec_mult_scalar(y, beta, Am);
        }
    }
    if *Ap.offset(An as isize) == 0 as ::core::ffi::c_int || alpha == 0.0f64 {
        return;
    }
    if alpha == -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < An {
            i = *Ap.offset(j as isize);
            while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh11 = *y.offset(*Ai.offset(i as isize) as isize);
                *fresh11 -=
                    (*Ax.offset(i as isize) * *x.offset(j as isize)) as ::core::ffi::c_double;
                i += 1;
            }
            j += 1;
        }
    } else if alpha == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < An {
            i = *Ap.offset(j as isize);
            while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh12 = *y.offset(*Ai.offset(i as isize) as isize);
                *fresh12 +=
                    (*Ax.offset(i as isize) * *x.offset(j as isize)) as ::core::ffi::c_double;
                i += 1;
            }
            j += 1;
        }
    } else {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < An {
            i = *Ap.offset(j as isize);
            while i < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh13 = *y.offset(*Ai.offset(i as isize) as isize);
                *fresh13 += (alpha * *Ax.offset(i as isize) * *x.offset(j as isize))
                    as ::core::ffi::c_double;
                i += 1;
            }
            j += 1;
        }
    };
}
#[export_name = "honest_osqp_csc_Atxpy"]
pub unsafe extern "C" fn csc_Atxpy(
    mut A: *const OSQPCscMatrix,
    mut x: *const OSQPFloat,
    mut y: *mut OSQPFloat,
    mut alpha: OSQPFloat,
    mut beta: OSQPFloat,
) {
    let mut j: OSQPInt = 0;
    let mut k: OSQPInt = 0;
    let mut An: OSQPInt = (*A).n;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ai: *mut OSQPInt = (*A).i;
    let mut Ax: *mut OSQPFloat = (*A).x;
    if beta == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        vec_set_scalar(y, 0.0f64, An);
    } else if !(beta == 1 as ::core::ffi::c_int as ::core::ffi::c_double) {
        if beta == -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
            vec_negate(y, An);
        } else {
            vec_mult_scalar(y, beta, An);
        }
    }
    if *Ap.offset(An as isize) == 0 as ::core::ffi::c_int || alpha == 0.0f64 {
        return;
    }
    if alpha == -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < (*A).n {
            k = *Ap.offset(j as isize);
            while k < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh14 = *y.offset(j as isize);
                *fresh14 -= (*Ax.offset(k as isize) * *x.offset(*Ai.offset(k as isize) as isize))
                    as ::core::ffi::c_double;
                k += 1;
            }
            j += 1;
        }
    } else if alpha == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < (*A).n {
            k = *Ap.offset(j as isize);
            while k < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh15 = *y.offset(j as isize);
                *fresh15 += (*Ax.offset(k as isize) * *x.offset(*Ai.offset(k as isize) as isize))
                    as ::core::ffi::c_double;
                k += 1;
            }
            j += 1;
        }
    } else {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < (*A).n {
            k = *Ap.offset(j as isize);
            while k < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                let ref mut fresh16 = *y.offset(j as isize);
                *fresh16 +=
                    (alpha * *Ax.offset(k as isize) * *x.offset(*Ai.offset(k as isize) as isize))
                        as ::core::ffi::c_double;
                k += 1;
            }
            j += 1;
        }
    };
}
#[export_name = "honest_osqp_csc_col_norm_inf"]
pub unsafe extern "C" fn csc_col_norm_inf(mut M: *const OSQPCscMatrix, mut E: *mut OSQPFloat) {
    let mut j: OSQPInt = 0;
    let mut ptr: OSQPInt = 0;
    let mut Mp: *mut OSQPInt = (*M).p;
    let mut Mn: OSQPInt = (*M).n;
    let mut Mx: *mut OSQPFloat = (*M).x;
    vec_set_scalar(E, 0.0f64, Mn);
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < Mn {
        ptr = *Mp.offset(j as isize);
        while ptr < *Mp.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            *E.offset(j as isize) = (if (if *Mx.offset(ptr as isize)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -(*Mx.offset(ptr as isize) as ::core::ffi::c_double)
            } else {
                *Mx.offset(ptr as isize) as ::core::ffi::c_double
            }) > *E.offset(j as isize)
            {
                if *Mx.offset(ptr as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(*Mx.offset(ptr as isize) as ::core::ffi::c_double)
                } else {
                    *Mx.offset(ptr as isize) as ::core::ffi::c_double
                }
            } else {
                *E.offset(j as isize) as ::core::ffi::c_double
            }) as OSQPFloat;
            ptr += 1;
        }
        j += 1;
    }
}
#[export_name = "honest_osqp_csc_row_norm_inf"]
pub unsafe extern "C" fn csc_row_norm_inf(mut M: *const OSQPCscMatrix, mut E: *mut OSQPFloat) {
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    let mut ptr: OSQPInt = 0;
    let mut Mp: *mut OSQPInt = (*M).p;
    let mut Mi: *mut OSQPInt = (*M).i;
    let mut Mn: OSQPInt = (*M).n;
    let mut Mm: OSQPInt = (*M).m;
    let mut Mx: *mut OSQPFloat = (*M).x;
    vec_set_scalar(E, 0.0f64, Mm);
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < Mn {
        ptr = *Mp.offset(j as isize);
        while ptr < *Mp.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            i = *Mi.offset(ptr as isize);
            *E.offset(i as isize) = (if (if *Mx.offset(ptr as isize)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -(*Mx.offset(ptr as isize) as ::core::ffi::c_double)
            } else {
                *Mx.offset(ptr as isize) as ::core::ffi::c_double
            }) > *E.offset(i as isize)
            {
                if *Mx.offset(ptr as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(*Mx.offset(ptr as isize) as ::core::ffi::c_double)
                } else {
                    *Mx.offset(ptr as isize) as ::core::ffi::c_double
                }
            } else {
                *E.offset(i as isize) as ::core::ffi::c_double
            }) as OSQPFloat;
            ptr += 1;
        }
        j += 1;
    }
}
#[export_name = "honest_osqp_csc_row_norm_inf_sym_triu"]
pub unsafe extern "C" fn csc_row_norm_inf_sym_triu(
    mut M: *const OSQPCscMatrix,
    mut E: *mut OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    let mut ptr: OSQPInt = 0;
    let mut Mp: *mut OSQPInt = (*M).p;
    let mut Mi: *mut OSQPInt = (*M).i;
    let mut Mn: OSQPInt = (*M).n;
    let mut Mm: OSQPInt = (*M).m;
    let mut Mx: *mut OSQPFloat = (*M).x;
    let mut abs_x: OSQPFloat = 0.;
    vec_set_scalar(E, 0.0f64, Mm);
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < Mn {
        ptr = *Mp.offset(j as isize);
        while ptr < *Mp.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            i = *Mi.offset(ptr as isize);
            abs_x = (if *Mx.offset(ptr as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -(*Mx.offset(ptr as isize) as ::core::ffi::c_double)
            } else {
                *Mx.offset(ptr as isize) as ::core::ffi::c_double
            }) as OSQPFloat;
            *E.offset(j as isize) = (if abs_x > *E.offset(j as isize) {
                abs_x as ::core::ffi::c_double
            } else {
                *E.offset(j as isize) as ::core::ffi::c_double
            }) as OSQPFloat;
            if i != j {
                *E.offset(i as isize) = (if abs_x > *E.offset(i as isize) {
                    abs_x as ::core::ffi::c_double
                } else {
                    *E.offset(i as isize) as ::core::ffi::c_double
                }) as OSQPFloat;
            }
            ptr += 1;
        }
        j += 1;
    }
}
