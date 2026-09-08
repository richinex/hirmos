#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type integer = ::core::ffi::c_long;
pub type real = ::core::ffi::c_float;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_ieeeck_(
    mut ispec: *mut integer,
    mut zero: *mut real,
    mut one: *mut real,
) -> integer {
    let mut ret_val: integer = 0;
    let mut nan1: real = 0.;
    let mut nan2: real = 0.;
    let mut nan3: real = 0.;
    let mut nan4: real = 0.;
    let mut nan5: real = 0.;
    let mut nan6: real = 0.;
    let mut neginf: real = 0.;
    let mut posinf: real = 0.;
    let mut negzro: real = 0.;
    let mut newzro: real = 0.;
    ret_val = 1 as integer;
    posinf = *one / *zero;
    if posinf <= *one {
        ret_val = 0 as integer;
        return ret_val;
    }
    neginf = -*one / *zero;
    if neginf >= *zero {
        ret_val = 0 as integer;
        return ret_val;
    }
    negzro = *one / (neginf + *one);
    if negzro != *zero {
        ret_val = 0 as integer;
        return ret_val;
    }
    neginf = *one / negzro;
    if neginf >= *zero {
        ret_val = 0 as integer;
        return ret_val;
    }
    newzro = negzro + *zero;
    if newzro != *zero {
        ret_val = 0 as integer;
        return ret_val;
    }
    posinf = *one / newzro;
    if posinf <= *one {
        ret_val = 0 as integer;
        return ret_val;
    }
    neginf *= posinf as ::core::ffi::c_float;
    if neginf >= *zero {
        ret_val = 0 as integer;
        return ret_val;
    }
    posinf *= posinf as ::core::ffi::c_float;
    if posinf <= *one {
        ret_val = 0 as integer;
        return ret_val;
    }
    if *ispec == 0 as ::core::ffi::c_long {
        return ret_val;
    }
    nan1 = posinf + neginf;
    nan2 = posinf / neginf;
    nan3 = posinf / posinf;
    nan4 = posinf * *zero;
    nan5 = neginf * negzro;
    nan6 = (nan5 as ::core::ffi::c_float * 0.0f32) as real;
    if nan1 == nan1 {
        ret_val = 0 as integer;
        return ret_val;
    }
    if nan2 == nan2 {
        ret_val = 0 as integer;
        return ret_val;
    }
    if nan3 == nan3 {
        ret_val = 0 as integer;
        return ret_val;
    }
    if nan4 == nan4 {
        ret_val = 0 as integer;
        return ret_val;
    }
    if nan5 == nan5 {
        ret_val = 0 as integer;
        return ret_val;
    }
    if nan6 == nan6 {
        ret_val = 0 as integer;
        return ret_val;
    }
    return ret_val;
}
