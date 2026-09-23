// Generated from preserved BOOM 0.9.16; see numerics-sources.json.
// Original source copyright Google LLC / Steven L. Scott, LGPL-2.1-or-later.
// NEWUOA: M. J. D. Powell; QUADPACK: Piessens / de Doncker, via R.
#![allow(
    unused,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_assignments
)]

fn sqrt(x: f64) -> f64 {
    x.sqrt()
}
fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}
fn fabs(x: f64) -> f64 {
    x.abs()
}
fn atan(x: f64) -> f64 {
    x.atan()
}
fn cos(x: f64) -> f64 {
    x.cos()
}
fn sin(x: f64) -> f64 {
    x.sin()
}

pub type bool_0 = bool;
pub type integr_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_double,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> (),
>;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = 2.220446049250313080847263336181640625e-16f64;
pub const DBL_MIN: ::core::ffi::c_double = 2.225073858507201383090232717332404064e-308f64;
pub const DBL_MAX: ::core::ffi::c_double = 1.797693134862315708145274237317043568e308f64;
static mut c_b6: ::core::ffi::c_double = 0.0f64;
static mut c_b7: ::core::ffi::c_double = 1.0f64;
pub unsafe extern "C" fn Rdqagi(
    mut f: integr_fn,
    mut ex: *mut ::core::ffi::c_void,
    mut bound: *mut ::core::ffi::c_double,
    mut inf: *mut ::core::ffi::c_int,
    mut epsabs: *mut ::core::ffi::c_double,
    mut epsrel: *mut ::core::ffi::c_double,
    mut result: *mut ::core::ffi::c_double,
    mut abserr: *mut ::core::ffi::c_double,
    mut neval: *mut ::core::ffi::c_int,
    mut ier: *mut ::core::ffi::c_int,
    mut limit: *mut ::core::ffi::c_int,
    mut lenw: *mut ::core::ffi::c_int,
    mut last: *mut ::core::ffi::c_int,
    mut iwork: *mut ::core::ffi::c_int,
    mut work: *mut ::core::ffi::c_double,
) {
    let mut l1: ::core::ffi::c_int = 0;
    let mut l2: ::core::ffi::c_int = 0;
    let mut l3: ::core::ffi::c_int = 0;
    iwork = iwork.offset(-1);
    work = work.offset(-1);
    *ier = 6 as ::core::ffi::c_int;
    *neval = 0 as ::core::ffi::c_int;
    *last = 0 as ::core::ffi::c_int;
    *result = 0.0f64;
    *abserr = 0.0f64;
    if *limit < 1 as ::core::ffi::c_int || *lenw < *limit << 2 as ::core::ffi::c_int {
        return;
    }
    l1 = *limit + 1 as ::core::ffi::c_int;
    l2 = *limit + l1;
    l3 = *limit + l2;
    rdqagie(
        f,
        ex,
        bound,
        inf,
        epsabs,
        epsrel,
        limit,
        result,
        abserr,
        neval,
        ier,
        work.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
        work.offset(l1 as isize) as *mut ::core::ffi::c_double,
        work.offset(l2 as isize) as *mut ::core::ffi::c_double,
        work.offset(l3 as isize) as *mut ::core::ffi::c_double,
        iwork.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int,
        last,
    );
}
unsafe extern "C" fn rdqagie(
    mut f: integr_fn,
    mut ex: *mut ::core::ffi::c_void,
    mut bound: *mut ::core::ffi::c_double,
    mut inf: *mut ::core::ffi::c_int,
    mut epsabs: *mut ::core::ffi::c_double,
    mut epsrel: *mut ::core::ffi::c_double,
    mut limit: *mut ::core::ffi::c_int,
    mut result: *mut ::core::ffi::c_double,
    mut abserr: *mut ::core::ffi::c_double,
    mut neval: *mut ::core::ffi::c_int,
    mut ier: *mut ::core::ffi::c_int,
    mut alist__: *mut ::core::ffi::c_double,
    mut blist: *mut ::core::ffi::c_double,
    mut rlist: *mut ::core::ffi::c_double,
    mut elist: *mut ::core::ffi::c_double,
    mut iord: *mut ::core::ffi::c_int,
    mut last: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut d__1: ::core::ffi::c_double = 0.;
    let mut d__2: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    let mut dres: ::core::ffi::c_double = 0.;
    let mut ksgn: ::core::ffi::c_int = 0;
    let mut boun: ::core::ffi::c_double = 0.;
    let mut nres: ::core::ffi::c_int = 0;
    let mut area1: ::core::ffi::c_double = 0.;
    let mut area2: ::core::ffi::c_double = 0.;
    let mut area12: ::core::ffi::c_double = 0.;
    let mut k: ::core::ffi::c_int = 0;
    let mut small: ::core::ffi::c_double = 0.0f64;
    let mut erro12: ::core::ffi::c_double = 0.;
    let mut ierro: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut b1: ::core::ffi::c_double = 0.;
    let mut b2: ::core::ffi::c_double = 0.;
    let mut defab1: ::core::ffi::c_double = 0.;
    let mut defab2: ::core::ffi::c_double = 0.;
    let mut oflow: ::core::ffi::c_double = 0.;
    let mut ktmin: ::core::ffi::c_int = 0;
    let mut nrmax: ::core::ffi::c_int = 0;
    let mut uflow: ::core::ffi::c_double = 0.;
    let mut noext: bool_0 = false;
    let mut iroff1: ::core::ffi::c_int = 0;
    let mut iroff2: ::core::ffi::c_int = 0;
    let mut iroff3: ::core::ffi::c_int = 0;
    let mut res3la: [::core::ffi::c_double; 3] = [0.; 3];
    let mut error1: ::core::ffi::c_double = 0.;
    let mut error2: ::core::ffi::c_double = 0.;
    let mut id: ::core::ffi::c_int = 0;
    let mut rlist2: [::core::ffi::c_double; 52] = [0.; 52];
    let mut numrl2: ::core::ffi::c_int = 0;
    let mut defabs: ::core::ffi::c_double = 0.;
    let mut epmach: ::core::ffi::c_double = 0.;
    let mut erlarg: ::core::ffi::c_double = 0.0f64;
    let mut abseps: ::core::ffi::c_double = 0.;
    let mut correc: ::core::ffi::c_double = 0.0f64;
    let mut errbnd: ::core::ffi::c_double = 0.;
    let mut resabs: ::core::ffi::c_double = 0.;
    let mut jupbnd: ::core::ffi::c_int = 0;
    let mut erlast: ::core::ffi::c_double = 0.;
    let mut errmax: ::core::ffi::c_double = 0.;
    let mut maxerr: ::core::ffi::c_int = 0;
    let mut reseps: ::core::ffi::c_double = 0.;
    let mut extrap: bool_0 = false;
    let mut ertest: ::core::ffi::c_double = 0.0f64;
    let mut errsum: ::core::ffi::c_double = 0.;
    iord = iord.offset(-1);
    elist = elist.offset(-1);
    rlist = rlist.offset(-1);
    blist = blist.offset(-1);
    alist__ = alist__.offset(-1);
    epmach = DBL_EPSILON;
    *ier = 0 as ::core::ffi::c_int;
    *neval = 0 as ::core::ffi::c_int;
    *last = 0 as ::core::ffi::c_int;
    *result = 0.0f64;
    *abserr = 0.0f64;
    *alist__.offset(1 as ::core::ffi::c_int as isize) = 0.0f64;
    *blist.offset(1 as ::core::ffi::c_int as isize) = 1.0f64;
    *rlist.offset(1 as ::core::ffi::c_int as isize) = 0.0f64;
    *elist.offset(1 as ::core::ffi::c_int as isize) = 0.0f64;
    *iord.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    if *epsabs <= 0.0f64
        && *epsrel
            < (if epmach * 50.0f64 > 5e-29f64 {
                epmach * 50.0f64
            } else {
                5e-29f64
            })
    {
        *ier = 6 as ::core::ffi::c_int;
    }
    if *ier == 6 as ::core::ffi::c_int {
        return;
    }
    boun = *bound;
    if *inf == 2 as ::core::ffi::c_int {
        boun = 0.0f64;
    }
    rdqk15i(
        f,
        ex,
        &raw mut boun,
        inf,
        &raw mut c_b6,
        &raw mut c_b7,
        result,
        abserr,
        &raw mut defabs,
        &raw mut resabs,
    );
    *last = 1 as ::core::ffi::c_int;
    *rlist.offset(1 as ::core::ffi::c_int as isize) = *result;
    *elist.offset(1 as ::core::ffi::c_int as isize) = *abserr;
    *iord.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
    dres = fabs(*result);
    errbnd = if *epsabs > *epsrel * dres {
        *epsabs
    } else {
        *epsrel * dres
    };
    if *abserr <= epmach * 100.0f64 * defabs && *abserr > errbnd {
        *ier = 2 as ::core::ffi::c_int;
    }
    if *limit == 1 as ::core::ffi::c_int {
        *ier = 1 as ::core::ffi::c_int;
    }
    if !(*ier != 0 as ::core::ffi::c_int
        || *abserr <= errbnd && *abserr != resabs
        || *abserr == 0.0f64)
    {
        uflow = DBL_MIN;
        oflow = DBL_MAX;
        rlist2[0 as ::core::ffi::c_int as usize] = *result;
        errmax = *abserr;
        maxerr = 1 as ::core::ffi::c_int;
        area = *result;
        errsum = *abserr;
        *abserr = oflow;
        nrmax = 1 as ::core::ffi::c_int;
        nres = 0 as ::core::ffi::c_int;
        ktmin = 0 as ::core::ffi::c_int;
        numrl2 = 2 as ::core::ffi::c_int;
        extrap = false_0 != 0;
        noext = false_0 != 0;
        ierro = 0 as ::core::ffi::c_int;
        iroff1 = 0 as ::core::ffi::c_int;
        iroff2 = 0 as ::core::ffi::c_int;
        iroff3 = 0 as ::core::ffi::c_int;
        ksgn = -(1 as ::core::ffi::c_int);
        if dres >= (1.0f64 - epmach * 50.0f64) * defabs {
            ksgn = 1 as ::core::ffi::c_int;
        }
        *last = 2 as ::core::ffi::c_int;
        loop {
            if !(*last <= *limit) {
                current_block = 11394243526202761407;
                break;
            }
            a1 = *alist__.offset(maxerr as isize);
            b1 = (*alist__.offset(maxerr as isize) + *blist.offset(maxerr as isize)) * 0.5f64;
            a2 = b1;
            b2 = *blist.offset(maxerr as isize);
            erlast = errmax;
            rdqk15i(
                f,
                ex,
                &raw mut boun,
                inf,
                &raw mut a1,
                &raw mut b1,
                &raw mut area1,
                &raw mut error1,
                &raw mut resabs,
                &raw mut defab1,
            );
            rdqk15i(
                f,
                ex,
                &raw mut boun,
                inf,
                &raw mut a2,
                &raw mut b2,
                &raw mut area2,
                &raw mut error2,
                &raw mut resabs,
                &raw mut defab2,
            );
            area12 = area1 + area2;
            erro12 = error1 + error2;
            errsum = errsum + erro12 - errmax;
            area = area + area12 - *rlist.offset(maxerr as isize);
            if !(defab1 == error1 || defab2 == error2) {
                if !(fabs(*rlist.offset(maxerr as isize) - area12) > fabs(area12) * 1e-5f64
                    || erro12 < errmax * 0.99f64)
                {
                    if extrap {
                        iroff2 += 1;
                    }
                    if !extrap {
                        iroff1 += 1;
                    }
                }
                if *last > 10 as ::core::ffi::c_int && erro12 > errmax {
                    iroff3 += 1;
                }
            }
            *rlist.offset(maxerr as isize) = area1;
            *rlist.offset(*last as isize) = area2;
            errbnd = if *epsabs > *epsrel * fabs(area) {
                *epsabs
            } else {
                *epsrel * fabs(area)
            };
            if iroff1 + iroff2 >= 10 as ::core::ffi::c_int || iroff3 >= 20 as ::core::ffi::c_int {
                *ier = 2 as ::core::ffi::c_int;
            }
            if iroff2 >= 5 as ::core::ffi::c_int {
                ierro = 3 as ::core::ffi::c_int;
            }
            if *last == *limit {
                *ier = 1 as ::core::ffi::c_int;
            }
            if (if fabs(a1) > fabs(b2) {
                fabs(a1)
            } else {
                fabs(b2)
            }) <= (epmach * 100.0f64 + 1.0f64) * (fabs(a2) + uflow * 1e3f64)
            {
                *ier = 4 as ::core::ffi::c_int;
            }
            if error2 > error1 {
                *alist__.offset(maxerr as isize) = a2;
                *alist__.offset(*last as isize) = a1;
                *blist.offset(*last as isize) = b1;
                *rlist.offset(maxerr as isize) = area2;
                *rlist.offset(*last as isize) = area1;
                *elist.offset(maxerr as isize) = error2;
                *elist.offset(*last as isize) = error1;
            } else {
                *alist__.offset(*last as isize) = a2;
                *blist.offset(maxerr as isize) = b1;
                *blist.offset(*last as isize) = b2;
                *elist.offset(maxerr as isize) = error1;
                *elist.offset(*last as isize) = error2;
            }
            rdqpsrt(
                limit,
                last,
                &raw mut maxerr,
                &raw mut errmax,
                elist.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                iord.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int,
                &raw mut nrmax,
            );
            if errsum <= errbnd {
                current_block = 13172448320796252212;
                break;
            }
            if *ier != 0 as ::core::ffi::c_int {
                current_block = 11394243526202761407;
                break;
            }
            if *last == 2 as ::core::ffi::c_int {
                small = 0.375f64;
                erlarg = errsum;
                ertest = errbnd;
                rlist2[1 as ::core::ffi::c_int as usize] = area;
            } else if !noext {
                erlarg -= erlast;
                if fabs(b1 - a1) > small {
                    erlarg += erro12;
                }
                if extrap {
                    current_block = 11921903885145342101;
                } else if fabs(*blist.offset(maxerr as isize) - *alist__.offset(maxerr as isize))
                    > small
                {
                    current_block = 2989495919056355252;
                } else {
                    extrap = true_0 != 0;
                    nrmax = 2 as ::core::ffi::c_int;
                    current_block = 11921903885145342101;
                }
                match current_block {
                    2989495919056355252 => {}
                    _ => {
                        if ierro == 3 as ::core::ffi::c_int || erlarg <= ertest {
                            current_block = 16006245776795492197;
                        } else {
                            id = nrmax;
                            jupbnd = *last;
                            if *last > *limit / 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int {
                                jupbnd = *limit + 3 as ::core::ffi::c_int - *last;
                            }
                            k = id;
                            loop {
                                if !(k <= jupbnd) {
                                    current_block = 16006245776795492197;
                                    break;
                                }
                                maxerr = *iord.offset(nrmax as isize);
                                errmax = *elist.offset(maxerr as isize);
                                if fabs(
                                    *blist.offset(maxerr as isize)
                                        - *alist__.offset(maxerr as isize),
                                ) > small
                                {
                                    current_block = 2989495919056355252;
                                    break;
                                }
                                nrmax += 1;
                                k += 1;
                            }
                        }
                        match current_block {
                            2989495919056355252 => {}
                            _ => {
                                numrl2 += 1;
                                rlist2[(numrl2 - 1 as ::core::ffi::c_int) as usize] = area;
                                rdqelg(
                                    &raw mut numrl2,
                                    &raw mut rlist2 as *mut ::core::ffi::c_double,
                                    &raw mut reseps,
                                    &raw mut abseps,
                                    &raw mut res3la as *mut ::core::ffi::c_double,
                                    &raw mut nres,
                                );
                                ktmin += 1;
                                if ktmin > 5 as ::core::ffi::c_int && *abserr < errsum * 0.001f64 {
                                    *ier = 5 as ::core::ffi::c_int;
                                }
                                if !(abseps >= *abserr) {
                                    ktmin = 0 as ::core::ffi::c_int;
                                    *abserr = abseps;
                                    *result = reseps;
                                    correc = erlarg;
                                    d__1 = *epsabs;
                                    d__2 = *epsrel * fabs(reseps);
                                    ertest = if d__1 > d__2 { d__1 } else { d__2 };
                                    if *abserr <= ertest {
                                        current_block = 11394243526202761407;
                                        break;
                                    }
                                }
                                if numrl2 == 1 as ::core::ffi::c_int {
                                    noext = true_0 != 0;
                                }
                                if *ier == 5 as ::core::ffi::c_int {
                                    current_block = 11394243526202761407;
                                    break;
                                }
                                maxerr = *iord.offset(1 as ::core::ffi::c_int as isize);
                                errmax = *elist.offset(maxerr as isize);
                                nrmax = 1 as ::core::ffi::c_int;
                                extrap = false_0 != 0;
                                small *= 0.5f64;
                                erlarg = errsum;
                            }
                        }
                    }
                }
            }
            *last += 1;
        }
        match current_block {
            11394243526202761407 => {
                if *abserr == oflow {
                    current_block = 13172448320796252212;
                } else {
                    if *ier + ierro == 0 as ::core::ffi::c_int {
                        current_block = 4116408444231823212;
                    } else {
                        if ierro == 3 as ::core::ffi::c_int {
                            *abserr += correc;
                        }
                        if *ier == 0 as ::core::ffi::c_int {
                            *ier = 3 as ::core::ffi::c_int;
                        }
                        if *result != 0.0f64 && area != 0.0f64 {
                            if *abserr / fabs(*result) > errsum / fabs(area) {
                                current_block = 13172448320796252212;
                            } else {
                                current_block = 4116408444231823212;
                            }
                        } else if *abserr > errsum {
                            current_block = 13172448320796252212;
                        } else if area == 0.0f64 {
                            current_block = 1503115706466962743;
                        } else {
                            current_block = 4116408444231823212;
                        }
                    }
                    match current_block {
                        13172448320796252212 => {}
                        1503115706466962743 => {}
                        _ => {
                            d__1 = fabs(*result);
                            d__2 = fabs(area);
                            if ksgn == -(1 as ::core::ffi::c_int)
                                && (if d__1 > d__2 { d__1 } else { d__2 }) <= defabs * 0.01f64
                            {
                                current_block = 1503115706466962743;
                            } else {
                                if 0.01f64 > *result / area
                                    || *result / area > 100.0f64
                                    || errsum > fabs(area)
                                {
                                    *ier = 6 as ::core::ffi::c_int;
                                }
                                current_block = 1503115706466962743;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            1503115706466962743 => {}
            _ => {
                *result = 0.0f64;
                k = 1 as ::core::ffi::c_int;
                while k <= *last {
                    *result += *rlist.offset(k as isize);
                    k += 1;
                }
                *abserr = errsum;
            }
        }
    }
    *neval = *last * 30 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    if *inf == 2 as ::core::ffi::c_int {
        *neval <<= 1 as ::core::ffi::c_int;
    }
    if *ier > 2 as ::core::ffi::c_int {
        *ier -= 1;
    }
}
pub unsafe extern "C" fn Rdqags(
    mut f: integr_fn,
    mut ex: *mut ::core::ffi::c_void,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut epsabs: *mut ::core::ffi::c_double,
    mut epsrel: *mut ::core::ffi::c_double,
    mut result: *mut ::core::ffi::c_double,
    mut abserr: *mut ::core::ffi::c_double,
    mut neval: *mut ::core::ffi::c_int,
    mut ier: *mut ::core::ffi::c_int,
    mut limit: *mut ::core::ffi::c_int,
    mut lenw: *mut ::core::ffi::c_int,
    mut last: *mut ::core::ffi::c_int,
    mut iwork: *mut ::core::ffi::c_int,
    mut work: *mut ::core::ffi::c_double,
) {
    let mut l1: ::core::ffi::c_int = 0;
    let mut l2: ::core::ffi::c_int = 0;
    let mut l3: ::core::ffi::c_int = 0;
    iwork = iwork.offset(-1);
    work = work.offset(-1);
    *ier = 6 as ::core::ffi::c_int;
    *neval = 0 as ::core::ffi::c_int;
    *last = 0 as ::core::ffi::c_int;
    *result = 0.0f64;
    *abserr = 0.0f64;
    if *limit < 1 as ::core::ffi::c_int || *lenw < *limit * 4 as ::core::ffi::c_int {
        return;
    }
    l1 = *limit + 1 as ::core::ffi::c_int;
    l2 = *limit + l1;
    l3 = *limit + l2;
    rdqagse(
        f,
        ex,
        a,
        b,
        epsabs,
        epsrel,
        limit,
        result,
        abserr,
        neval,
        ier,
        work.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
        work.offset(l1 as isize) as *mut ::core::ffi::c_double,
        work.offset(l2 as isize) as *mut ::core::ffi::c_double,
        work.offset(l3 as isize) as *mut ::core::ffi::c_double,
        iwork.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int,
        last,
    );
}
unsafe extern "C" fn rdqagse(
    mut f: integr_fn,
    mut ex: *mut ::core::ffi::c_void,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut epsabs: *mut ::core::ffi::c_double,
    mut epsrel: *mut ::core::ffi::c_double,
    mut limit: *mut ::core::ffi::c_int,
    mut result: *mut ::core::ffi::c_double,
    mut abserr: *mut ::core::ffi::c_double,
    mut neval: *mut ::core::ffi::c_int,
    mut ier: *mut ::core::ffi::c_int,
    mut alist__: *mut ::core::ffi::c_double,
    mut blist: *mut ::core::ffi::c_double,
    mut rlist: *mut ::core::ffi::c_double,
    mut elist: *mut ::core::ffi::c_double,
    mut iord: *mut ::core::ffi::c_int,
    mut last: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut noext: bool_0 = false;
    let mut extrap: bool_0 = false;
    let mut k: ::core::ffi::c_int = 0;
    let mut ksgn: ::core::ffi::c_int = 0;
    let mut nres: ::core::ffi::c_int = 0;
    let mut ierro: ::core::ffi::c_int = 0;
    let mut ktmin: ::core::ffi::c_int = 0;
    let mut nrmax: ::core::ffi::c_int = 0;
    let mut iroff1: ::core::ffi::c_int = 0;
    let mut iroff2: ::core::ffi::c_int = 0;
    let mut iroff3: ::core::ffi::c_int = 0;
    let mut id: ::core::ffi::c_int = 0;
    let mut numrl2: ::core::ffi::c_int = 0;
    let mut jupbnd: ::core::ffi::c_int = 0;
    let mut maxerr: ::core::ffi::c_int = 0;
    let mut res3la: [::core::ffi::c_double; 3] = [0.; 3];
    let mut rlist2: [::core::ffi::c_double; 52] = [0.; 52];
    let mut abseps: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    let mut area1: ::core::ffi::c_double = 0.;
    let mut area2: ::core::ffi::c_double = 0.;
    let mut area12: ::core::ffi::c_double = 0.;
    let mut dres: ::core::ffi::c_double = 0.;
    let mut epmach: ::core::ffi::c_double = 0.;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut b1: ::core::ffi::c_double = 0.;
    let mut b2: ::core::ffi::c_double = 0.;
    let mut defabs: ::core::ffi::c_double = 0.;
    let mut defab1: ::core::ffi::c_double = 0.;
    let mut defab2: ::core::ffi::c_double = 0.;
    let mut oflow: ::core::ffi::c_double = 0.;
    let mut uflow: ::core::ffi::c_double = 0.;
    let mut resabs: ::core::ffi::c_double = 0.;
    let mut reseps: ::core::ffi::c_double = 0.;
    let mut error1: ::core::ffi::c_double = 0.;
    let mut error2: ::core::ffi::c_double = 0.;
    let mut erro12: ::core::ffi::c_double = 0.;
    let mut errbnd: ::core::ffi::c_double = 0.;
    let mut erlast: ::core::ffi::c_double = 0.;
    let mut errmax: ::core::ffi::c_double = 0.;
    let mut errsum: ::core::ffi::c_double = 0.;
    let mut correc: ::core::ffi::c_double = 0.0f64;
    let mut erlarg: ::core::ffi::c_double = 0.0f64;
    let mut ertest: ::core::ffi::c_double = 0.0f64;
    let mut small: ::core::ffi::c_double = 0.0f64;
    iord = iord.offset(-1);
    elist = elist.offset(-1);
    rlist = rlist.offset(-1);
    blist = blist.offset(-1);
    alist__ = alist__.offset(-1);
    epmach = DBL_EPSILON;
    *ier = 0 as ::core::ffi::c_int;
    *neval = 0 as ::core::ffi::c_int;
    *last = 0 as ::core::ffi::c_int;
    *result = 0.0f64;
    *abserr = 0.0f64;
    *alist__.offset(1 as ::core::ffi::c_int as isize) = *a;
    *blist.offset(1 as ::core::ffi::c_int as isize) = *b;
    *rlist.offset(1 as ::core::ffi::c_int as isize) = 0.0f64;
    *elist.offset(1 as ::core::ffi::c_int as isize) = 0.0f64;
    if *epsabs <= 0.0f64
        && *epsrel
            < (if epmach * 50.0f64 > 5e-29f64 {
                epmach * 50.0f64
            } else {
                5e-29f64
            })
    {
        *ier = 6 as ::core::ffi::c_int;
        return;
    }
    uflow = DBL_MIN;
    oflow = DBL_MAX;
    ierro = 0 as ::core::ffi::c_int;
    rdqk21(
        f,
        ex,
        a,
        b,
        result,
        abserr,
        &raw mut defabs,
        &raw mut resabs,
    );
    dres = fabs(*result);
    errbnd = if *epsabs > *epsrel * dres {
        *epsabs
    } else {
        *epsrel * dres
    };
    *last = 1 as ::core::ffi::c_int;
    *rlist.offset(1 as ::core::ffi::c_int as isize) = *result;
    *elist.offset(1 as ::core::ffi::c_int as isize) = *abserr;
    *iord.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
    if *abserr <= epmach * 100.0f64 * defabs && *abserr > errbnd {
        *ier = 2 as ::core::ffi::c_int;
    }
    if *limit == 1 as ::core::ffi::c_int {
        *ier = 1 as ::core::ffi::c_int;
    }
    if *ier != 0 as ::core::ffi::c_int
        || *abserr <= errbnd && *abserr != resabs
        || *abserr == 0.0f64
    {
        current_block = 15180873175769413469;
    } else {
        rlist2[0 as ::core::ffi::c_int as usize] = *result;
        errmax = *abserr;
        maxerr = 1 as ::core::ffi::c_int;
        area = *result;
        errsum = *abserr;
        *abserr = oflow;
        nrmax = 1 as ::core::ffi::c_int;
        nres = 0 as ::core::ffi::c_int;
        numrl2 = 2 as ::core::ffi::c_int;
        ktmin = 0 as ::core::ffi::c_int;
        extrap = false_0 != 0;
        noext = false_0 != 0;
        iroff1 = 0 as ::core::ffi::c_int;
        iroff2 = 0 as ::core::ffi::c_int;
        iroff3 = 0 as ::core::ffi::c_int;
        ksgn = -(1 as ::core::ffi::c_int);
        if dres >= (1.0f64 - epmach * 50.0f64) * defabs {
            ksgn = 1 as ::core::ffi::c_int;
        }
        *last = 2 as ::core::ffi::c_int;
        loop {
            if !(*last <= *limit) {
                current_block = 18082114152826926066;
                break;
            }
            a1 = *alist__.offset(maxerr as isize);
            b1 = (*alist__.offset(maxerr as isize) + *blist.offset(maxerr as isize)) * 0.5f64;
            a2 = b1;
            b2 = *blist.offset(maxerr as isize);
            erlast = errmax;
            rdqk21(
                f,
                ex,
                &raw mut a1,
                &raw mut b1,
                &raw mut area1,
                &raw mut error1,
                &raw mut resabs,
                &raw mut defab1,
            );
            rdqk21(
                f,
                ex,
                &raw mut a2,
                &raw mut b2,
                &raw mut area2,
                &raw mut error2,
                &raw mut resabs,
                &raw mut defab2,
            );
            area12 = area1 + area2;
            erro12 = error1 + error2;
            errsum = errsum + erro12 - errmax;
            area = area + area12 - *rlist.offset(maxerr as isize);
            if !(defab1 == error1 || defab2 == error2) {
                if !(fabs(*rlist.offset(maxerr as isize) - area12) > fabs(area12) * 1e-5f64
                    || erro12 < errmax * 0.99f64)
                {
                    if extrap {
                        iroff2 += 1;
                    }
                    if !extrap {
                        iroff1 += 1;
                    }
                }
                if *last > 10 as ::core::ffi::c_int && erro12 > errmax {
                    iroff3 += 1;
                }
            }
            *rlist.offset(maxerr as isize) = area1;
            *rlist.offset(*last as isize) = area2;
            errbnd = if *epsabs > *epsrel * fabs(area) {
                *epsabs
            } else {
                *epsrel * fabs(area)
            };
            if iroff1 + iroff2 >= 10 as ::core::ffi::c_int || iroff3 >= 20 as ::core::ffi::c_int {
                *ier = 2 as ::core::ffi::c_int;
            }
            if iroff2 >= 5 as ::core::ffi::c_int {
                ierro = 3 as ::core::ffi::c_int;
            }
            if *last == *limit {
                *ier = 1 as ::core::ffi::c_int;
            }
            if (if fabs(a1) > fabs(b2) {
                fabs(a1)
            } else {
                fabs(b2)
            }) <= (epmach * 100.0f64 + 1.0f64) * (fabs(a2) + uflow * 1e3f64)
            {
                *ier = 4 as ::core::ffi::c_int;
            }
            if error2 > error1 {
                *alist__.offset(maxerr as isize) = a2;
                *alist__.offset(*last as isize) = a1;
                *blist.offset(*last as isize) = b1;
                *rlist.offset(maxerr as isize) = area2;
                *rlist.offset(*last as isize) = area1;
                *elist.offset(maxerr as isize) = error2;
                *elist.offset(*last as isize) = error1;
            } else {
                *alist__.offset(*last as isize) = a2;
                *blist.offset(maxerr as isize) = b1;
                *blist.offset(*last as isize) = b2;
                *elist.offset(maxerr as isize) = error1;
                *elist.offset(*last as isize) = error2;
            }
            rdqpsrt(
                limit,
                last,
                &raw mut maxerr,
                &raw mut errmax,
                elist.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                iord.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int,
                &raw mut nrmax,
            );
            if errsum <= errbnd {
                current_block = 2429925007462148094;
                break;
            }
            if *ier != 0 as ::core::ffi::c_int {
                current_block = 18082114152826926066;
                break;
            }
            if *last == 2 as ::core::ffi::c_int {
                small = fabs(*b - *a) * 0.375f64;
                erlarg = errsum;
                ertest = errbnd;
                rlist2[1 as ::core::ffi::c_int as usize] = area;
            } else if !noext {
                erlarg -= erlast;
                if fabs(b1 - a1) > small {
                    erlarg += erro12;
                }
                if extrap {
                    current_block = 13229369251729066864;
                } else if fabs(*blist.offset(maxerr as isize) - *alist__.offset(maxerr as isize))
                    > small
                {
                    current_block = 7226443171521532240;
                } else {
                    extrap = true_0 != 0;
                    nrmax = 2 as ::core::ffi::c_int;
                    current_block = 13229369251729066864;
                }
                match current_block {
                    7226443171521532240 => {}
                    _ => {
                        if ierro == 3 as ::core::ffi::c_int || erlarg <= ertest {
                            current_block = 267174800429061794;
                        } else {
                            id = nrmax;
                            jupbnd = *last;
                            if *last > *limit / 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int {
                                jupbnd = *limit + 3 as ::core::ffi::c_int - *last;
                            }
                            k = id;
                            loop {
                                if !(k <= jupbnd) {
                                    current_block = 267174800429061794;
                                    break;
                                }
                                maxerr = *iord.offset(nrmax as isize);
                                errmax = *elist.offset(maxerr as isize);
                                if fabs(
                                    *blist.offset(maxerr as isize)
                                        - *alist__.offset(maxerr as isize),
                                ) > small
                                {
                                    current_block = 7226443171521532240;
                                    break;
                                }
                                nrmax += 1;
                                k += 1;
                            }
                        }
                        match current_block {
                            7226443171521532240 => {}
                            _ => {
                                numrl2 += 1;
                                rlist2[(numrl2 - 1 as ::core::ffi::c_int) as usize] = area;
                                rdqelg(
                                    &raw mut numrl2,
                                    &raw mut rlist2 as *mut ::core::ffi::c_double,
                                    &raw mut reseps,
                                    &raw mut abseps,
                                    &raw mut res3la as *mut ::core::ffi::c_double,
                                    &raw mut nres,
                                );
                                ktmin += 1;
                                if ktmin > 5 as ::core::ffi::c_int && *abserr < errsum * 0.001f64 {
                                    *ier = 5 as ::core::ffi::c_int;
                                }
                                if !(abseps >= *abserr) {
                                    ktmin = 0 as ::core::ffi::c_int;
                                    *abserr = abseps;
                                    *result = reseps;
                                    correc = erlarg;
                                    ertest = if *epsabs > *epsrel * fabs(reseps) {
                                        *epsabs
                                    } else {
                                        *epsrel * fabs(reseps)
                                    };
                                    if *abserr <= ertest {
                                        current_block = 18082114152826926066;
                                        break;
                                    }
                                }
                                if numrl2 == 1 as ::core::ffi::c_int {
                                    noext = true_0 != 0;
                                }
                                if *ier == 5 as ::core::ffi::c_int {
                                    current_block = 18082114152826926066;
                                    break;
                                }
                                maxerr = *iord.offset(1 as ::core::ffi::c_int as isize);
                                errmax = *elist.offset(maxerr as isize);
                                nrmax = 1 as ::core::ffi::c_int;
                                extrap = false_0 != 0;
                                small *= 0.5f64;
                                erlarg = errsum;
                            }
                        }
                    }
                }
            }
            *last += 1;
        }
        match current_block {
            18082114152826926066 => {
                if *abserr == oflow {
                    current_block = 2429925007462148094;
                } else {
                    if *ier + ierro == 0 as ::core::ffi::c_int {
                        current_block = 5568477525302686547;
                    } else {
                        if ierro == 3 as ::core::ffi::c_int {
                            *abserr += correc;
                        }
                        if *ier == 0 as ::core::ffi::c_int {
                            *ier = 3 as ::core::ffi::c_int;
                        }
                        if *result != 0.0f64 && area != 0.0f64 {
                            if *abserr / fabs(*result) > errsum / fabs(area) {
                                current_block = 2429925007462148094;
                            } else {
                                current_block = 5568477525302686547;
                            }
                        } else if *abserr > errsum {
                            current_block = 2429925007462148094;
                        } else if area == 0.0f64 {
                            current_block = 18385021481934029905;
                        } else {
                            current_block = 5568477525302686547;
                        }
                    }
                    match current_block {
                        2429925007462148094 => {}
                        18385021481934029905 => {}
                        _ => {
                            if ksgn == -(1 as ::core::ffi::c_int)
                                && (if fabs(*result) > fabs(area) {
                                    fabs(*result)
                                } else {
                                    fabs(area)
                                }) <= defabs * 0.01f64
                            {
                                current_block = 18385021481934029905;
                            } else {
                                if 0.01f64 > *result / area
                                    || *result / area > 100.0f64
                                    || errsum > fabs(area)
                                {
                                    *ier = 5 as ::core::ffi::c_int;
                                }
                                current_block = 18385021481934029905;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            2429925007462148094 => {
                *result = 0.0f64;
                k = 1 as ::core::ffi::c_int;
                while k <= *last {
                    *result += *rlist.offset(k as isize);
                    k += 1;
                }
                *abserr = errsum;
            }
            _ => {}
        }
        if *ier > 2 as ::core::ffi::c_int {
            current_block = 15180873175769413469;
        } else {
            current_block = 12099607619007264150;
        }
    }
    match current_block {
        15180873175769413469 => {
            *neval = *last * 42 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        }
        _ => {}
    };
}
unsafe extern "C" fn rdqk15i(
    mut f: integr_fn,
    mut ex: *mut ::core::ffi::c_void,
    mut boun: *mut ::core::ffi::c_double,
    mut inf: *mut ::core::ffi::c_int,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut result: *mut ::core::ffi::c_double,
    mut abserr: *mut ::core::ffi::c_double,
    mut resabs: *mut ::core::ffi::c_double,
    mut resasc: *mut ::core::ffi::c_double,
) {
    const wg: [::core::ffi::c_double; 8] = [
        0.0f64,
        0.129484966168869693270611432679082f64,
        0.0f64,
        0.27970539148927666790146777142378f64,
        0.0f64,
        0.381830050505118944950369775488975f64,
        0.0f64,
        0.417959183673469387755102040816327f64,
    ];
    const xgk: [::core::ffi::c_double; 8] = [
        0.991455371120812639206854697526329f64,
        0.949107912342758524526189684047851f64,
        0.864864423359769072789712788640926f64,
        0.741531185599394439863864773280788f64,
        0.58608723546769113029414483825873f64,
        0.405845151377397166906606412076961f64,
        0.207784955007898467600689403773245f64,
        0.0f64,
    ];
    const wgk: [::core::ffi::c_double; 8] = [
        0.02293532201052922496373200805897f64,
        0.063092092629978553290700663189204f64,
        0.104790010322250183839876322541518f64,
        0.140653259715525918745189590510238f64,
        0.16900472663926790282658342659855f64,
        0.190350578064785409913256402421014f64,
        0.204432940075298892414161999234649f64,
        0.209482141084727828012999174891714f64,
    ];
    let mut absc: ::core::ffi::c_double = 0.;
    let mut dinf: ::core::ffi::c_double = 0.;
    let mut resg: ::core::ffi::c_double = 0.;
    let mut resk: ::core::ffi::c_double = 0.;
    let mut fsum: ::core::ffi::c_double = 0.;
    let mut absc1: ::core::ffi::c_double = 0.;
    let mut absc2: ::core::ffi::c_double = 0.;
    let mut fval1: ::core::ffi::c_double = 0.;
    let mut fval2: ::core::ffi::c_double = 0.;
    let mut j: ::core::ffi::c_int = 0;
    let mut hlgth: ::core::ffi::c_double = 0.;
    let mut centr: ::core::ffi::c_double = 0.;
    let mut reskh: ::core::ffi::c_double = 0.;
    let mut uflow: ::core::ffi::c_double = 0.;
    let mut tabsc1: ::core::ffi::c_double = 0.;
    let mut tabsc2: ::core::ffi::c_double = 0.;
    let mut fc: ::core::ffi::c_double = 0.;
    let mut epmach: ::core::ffi::c_double = 0.;
    let mut fv1: [::core::ffi::c_double; 7] = [0.; 7];
    let mut fv2: [::core::ffi::c_double; 7] = [0.; 7];
    let mut vec: [::core::ffi::c_double; 15] = [0.; 15];
    let mut vec2: [::core::ffi::c_double; 15] = [0.; 15];
    epmach = DBL_EPSILON;
    uflow = DBL_MIN;
    dinf = (if (1 as ::core::ffi::c_int) < *inf {
        1 as ::core::ffi::c_int
    } else {
        *inf
    }) as ::core::ffi::c_double;
    centr = (*a + *b) * 0.5f64;
    hlgth = (*b - *a) * 0.5f64;
    tabsc1 = *boun + dinf * (1.0f64 - centr) / centr;
    vec[0 as ::core::ffi::c_int as usize] = tabsc1;
    if *inf == 2 as ::core::ffi::c_int {
        vec2[0 as ::core::ffi::c_int as usize] = -tabsc1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= 7 as ::core::ffi::c_int {
        absc = hlgth * xgk[(j - 1 as ::core::ffi::c_int) as usize];
        absc1 = centr - absc;
        absc2 = centr + absc;
        tabsc1 = *boun + dinf * (1.0f64 - absc1) / absc1;
        tabsc2 = *boun + dinf * (1.0f64 - absc2) / absc2;
        vec[((j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as usize] = tabsc1;
        vec[(j * 2 as ::core::ffi::c_int) as usize] = tabsc2;
        if *inf == 2 as ::core::ffi::c_int {
            vec2[((j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as usize] = -tabsc1;
            vec2[(j * 2 as ::core::ffi::c_int) as usize] = -tabsc2;
        }
        j += 1;
    }
    f.expect("non-null function pointer")(
        &raw mut vec as *mut ::core::ffi::c_double,
        15 as ::core::ffi::c_int,
        ex,
    );
    if *inf == 2 as ::core::ffi::c_int {
        f.expect("non-null function pointer")(
            &raw mut vec2 as *mut ::core::ffi::c_double,
            15 as ::core::ffi::c_int,
            ex,
        );
    }
    fval1 = vec[0 as ::core::ffi::c_int as usize];
    if *inf == 2 as ::core::ffi::c_int {
        fval1 += vec2[0 as ::core::ffi::c_int as usize];
    }
    fc = fval1 / centr / centr;
    resg = wg[7 as ::core::ffi::c_int as usize] * fc;
    resk = wgk[7 as ::core::ffi::c_int as usize] * fc;
    *resabs = fabs(resk);
    j = 1 as ::core::ffi::c_int;
    while j <= 7 as ::core::ffi::c_int {
        absc = hlgth * xgk[(j - 1 as ::core::ffi::c_int) as usize];
        absc1 = centr - absc;
        absc2 = centr + absc;
        tabsc1 = *boun + dinf * (1.0f64 - absc1) / absc1;
        tabsc2 = *boun + dinf * (1.0f64 - absc2) / absc2;
        fval1 = vec[((j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as usize];
        fval2 = vec[(j * 2 as ::core::ffi::c_int) as usize];
        if *inf == 2 as ::core::ffi::c_int {
            fval1 += vec2[((j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as usize];
        }
        if *inf == 2 as ::core::ffi::c_int {
            fval2 += vec2[(j * 2 as ::core::ffi::c_int) as usize];
        }
        fval1 = fval1 / absc1 / absc1;
        fval2 = fval2 / absc2 / absc2;
        fv1[(j - 1 as ::core::ffi::c_int) as usize] = fval1;
        fv2[(j - 1 as ::core::ffi::c_int) as usize] = fval2;
        fsum = fval1 + fval2;
        resg += wg[(j - 1 as ::core::ffi::c_int) as usize] * fsum;
        resk += wgk[(j - 1 as ::core::ffi::c_int) as usize] * fsum;
        *resabs += wgk[(j - 1 as ::core::ffi::c_int) as usize] * (fabs(fval1) + fabs(fval2));
        j += 1;
    }
    reskh = resk * 0.5f64;
    *resasc = wgk[7 as ::core::ffi::c_int as usize] * fabs(fc - reskh);
    j = 1 as ::core::ffi::c_int;
    while j <= 7 as ::core::ffi::c_int {
        *resasc += wgk[(j - 1 as ::core::ffi::c_int) as usize]
            * (fabs(fv1[(j - 1 as ::core::ffi::c_int) as usize] - reskh)
                + fabs(fv2[(j - 1 as ::core::ffi::c_int) as usize] - reskh));
        j += 1;
    }
    *result = resk * hlgth;
    *resasc *= hlgth;
    *resabs *= hlgth;
    *abserr = fabs((resk - resg) * hlgth);
    if *resasc != 0.0f64 && *abserr != 0.0f64 {
        *abserr = *resasc
            * (if 1.0f64 < pow(*abserr * 200.0f64 / *resasc, 1.5f64) {
                1.0f64
            } else {
                pow(*abserr * 200.0f64 / *resasc, 1.5f64)
            });
    }
    if *resabs > uflow / (epmach * 50.0f64) {
        *abserr = if epmach * 50.0f64 * *resabs > *abserr {
            epmach * 50.0f64 * *resabs
        } else {
            *abserr
        };
    }
}
unsafe extern "C" fn rdqelg(
    mut n: *mut ::core::ffi::c_int,
    mut epstab: *mut ::core::ffi::c_double,
    mut result: *mut ::core::ffi::c_double,
    mut abserr: *mut ::core::ffi::c_double,
    mut res3la: *mut ::core::ffi::c_double,
    mut nres: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut i__: ::core::ffi::c_int = 0;
    let mut indx: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ib2: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut k1: ::core::ffi::c_int = 0;
    let mut k2: ::core::ffi::c_int = 0;
    let mut k3: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0;
    let mut newelm: ::core::ffi::c_int = 0;
    let mut limexp: ::core::ffi::c_int = 0;
    let mut delta1: ::core::ffi::c_double = 0.;
    let mut delta2: ::core::ffi::c_double = 0.;
    let mut delta3: ::core::ffi::c_double = 0.;
    let mut e0: ::core::ffi::c_double = 0.;
    let mut e1: ::core::ffi::c_double = 0.;
    let mut e1abs: ::core::ffi::c_double = 0.;
    let mut e2: ::core::ffi::c_double = 0.;
    let mut e3: ::core::ffi::c_double = 0.;
    let mut epmach: ::core::ffi::c_double = 0.;
    let mut epsinf: ::core::ffi::c_double = 0.;
    let mut oflow: ::core::ffi::c_double = 0.;
    let mut ss: ::core::ffi::c_double = 0.;
    let mut res: ::core::ffi::c_double = 0.;
    let mut errA: ::core::ffi::c_double = 0.;
    let mut err1: ::core::ffi::c_double = 0.;
    let mut err2: ::core::ffi::c_double = 0.;
    let mut err3: ::core::ffi::c_double = 0.;
    let mut tol1: ::core::ffi::c_double = 0.;
    let mut tol2: ::core::ffi::c_double = 0.;
    let mut tol3: ::core::ffi::c_double = 0.;
    res3la = res3la.offset(-1);
    epstab = epstab.offset(-1);
    epmach = DBL_EPSILON;
    oflow = DBL_MAX;
    *nres += 1;
    *abserr = oflow;
    *result = *epstab.offset(*n as isize);
    if !(*n < 3 as ::core::ffi::c_int) {
        limexp = 50 as ::core::ffi::c_int;
        *epstab.offset((*n + 2 as ::core::ffi::c_int) as isize) = *epstab.offset(*n as isize);
        newelm = (*n - 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
        *epstab.offset(*n as isize) = oflow;
        num = *n;
        k1 = *n;
        i__ = 1 as ::core::ffi::c_int;
        loop {
            if !(i__ <= newelm) {
                current_block = 10350621302084027428;
                break;
            }
            k2 = k1 - 1 as ::core::ffi::c_int;
            k3 = k1 - 2 as ::core::ffi::c_int;
            res = *epstab.offset((k1 + 2 as ::core::ffi::c_int) as isize);
            e0 = *epstab.offset(k3 as isize);
            e1 = *epstab.offset(k2 as isize);
            e2 = res;
            e1abs = fabs(e1);
            delta2 = e2 - e1;
            err2 = fabs(delta2);
            tol2 = (if fabs(e2) > e1abs { fabs(e2) } else { e1abs }) * epmach;
            delta3 = e1 - e0;
            err3 = fabs(delta3);
            tol3 = (if e1abs > fabs(e0) { e1abs } else { fabs(e0) }) * epmach;
            if err2 <= tol2 && err3 <= tol3 {
                *result = res;
                *abserr = err2 + err3;
                current_block = 11207080037060097641;
                break;
            } else {
                e3 = *epstab.offset(k1 as isize);
                *epstab.offset(k1 as isize) = e1;
                delta1 = e1 - e3;
                err1 = fabs(delta1);
                tol1 = (if e1abs > fabs(e3) { e1abs } else { fabs(e3) }) * epmach;
                if err1 > tol1 && err2 > tol2 && err3 > tol3 {
                    ss = 1.0f64 / delta1 + 1.0f64 / delta2 - 1.0f64 / delta3;
                    epsinf = fabs(ss * e1);
                    if epsinf > 1e-4f64 {
                        res = e1 + 1.0f64 / ss;
                        *epstab.offset(k1 as isize) = res;
                        k1 += -(2 as ::core::ffi::c_int);
                        errA = err2 + fabs(res - e2) + err3;
                        if errA <= *abserr {
                            *abserr = errA;
                            *result = res;
                        }
                        i__ += 1;
                        continue;
                    }
                }
                *n = i__ + i__ - 1 as ::core::ffi::c_int;
                current_block = 10350621302084027428;
                break;
            }
        }
        match current_block {
            11207080037060097641 => {}
            _ => {
                if *n == limexp {
                    *n = ((limexp / 2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int;
                }
                if (num / 2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == num {
                    ib = 2 as ::core::ffi::c_int;
                } else {
                    ib = 1 as ::core::ffi::c_int;
                }
                ie = newelm + 1 as ::core::ffi::c_int;
                i__ = 1 as ::core::ffi::c_int;
                while i__ <= ie {
                    ib2 = ib + 2 as ::core::ffi::c_int;
                    *epstab.offset(ib as isize) = *epstab.offset(ib2 as isize);
                    ib = ib2;
                    i__ += 1;
                }
                if num != *n {
                    indx = num - *n + 1 as ::core::ffi::c_int;
                    i__ = 1 as ::core::ffi::c_int;
                    while i__ <= *n {
                        *epstab.offset(i__ as isize) = *epstab.offset(indx as isize);
                        indx += 1;
                        i__ += 1;
                    }
                }
                if *nres >= 4 as ::core::ffi::c_int {
                    *abserr = fabs(*result - *res3la.offset(3 as ::core::ffi::c_int as isize))
                        + fabs(*result - *res3la.offset(2 as ::core::ffi::c_int as isize))
                        + fabs(*result - *res3la.offset(1 as ::core::ffi::c_int as isize));
                    *res3la.offset(1 as ::core::ffi::c_int as isize) =
                        *res3la.offset(2 as ::core::ffi::c_int as isize);
                    *res3la.offset(2 as ::core::ffi::c_int as isize) =
                        *res3la.offset(3 as ::core::ffi::c_int as isize);
                    *res3la.offset(3 as ::core::ffi::c_int as isize) = *result;
                } else {
                    *res3la.offset(*nres as isize) = *result;
                    *abserr = oflow;
                }
            }
        }
    }
    *abserr = if *abserr > epmach * 5.0f64 * fabs(*result) {
        *abserr
    } else {
        epmach * 5.0f64 * fabs(*result)
    };
}
unsafe extern "C" fn rdqk21(
    mut f: integr_fn,
    mut ex: *mut ::core::ffi::c_void,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut result: *mut ::core::ffi::c_double,
    mut abserr: *mut ::core::ffi::c_double,
    mut resabs: *mut ::core::ffi::c_double,
    mut resasc: *mut ::core::ffi::c_double,
) {
    const wg: [::core::ffi::c_double; 5] = [
        0.066671344308688137593568809893332f64,
        0.149451349150580593145776339657697f64,
        0.219086362515982043995534934228163f64,
        0.269266719309996355091226921569469f64,
        0.295524224714752870173892994651338f64,
    ];
    const xgk: [::core::ffi::c_double; 11] = [
        0.995657163025808080735527280689003f64,
        0.973906528517171720077964012084452f64,
        0.930157491355708226001207180059508f64,
        0.865063366688984510732096688423493f64,
        0.780817726586416897063717578345042f64,
        0.679409568299024406234327365114874f64,
        0.562757134668604683339000099272694f64,
        0.433395394129247190799265943165784f64,
        0.294392862701460198131126603103866f64,
        0.14887433898163121088482600112972f64,
        0.0f64,
    ];
    const wgk: [::core::ffi::c_double; 11] = [
        0.011694638867371874278064396062192f64,
        0.03255816230796472747881897245939f64,
        0.05475589657435199603138130024458f64,
        0.07503967481091995276704314091619f64,
        0.093125454583697605535065465083366f64,
        0.109387158802297641899210590325805f64,
        0.123491976262065851077958109831074f64,
        0.134709217311473325928054001771707f64,
        0.142775938577060080797094273138717f64,
        0.147739104901338491374841515972068f64,
        0.149445554002916905664936468389821f64,
    ];
    let mut fv1: [::core::ffi::c_double; 10] = [0.; 10];
    let mut fv2: [::core::ffi::c_double; 10] = [0.; 10];
    let mut vec: [::core::ffi::c_double; 21] = [0.; 21];
    let mut absc: ::core::ffi::c_double = 0.;
    let mut resg: ::core::ffi::c_double = 0.;
    let mut resk: ::core::ffi::c_double = 0.;
    let mut fsum: ::core::ffi::c_double = 0.;
    let mut fval1: ::core::ffi::c_double = 0.;
    let mut fval2: ::core::ffi::c_double = 0.;
    let mut hlgth: ::core::ffi::c_double = 0.;
    let mut centr: ::core::ffi::c_double = 0.;
    let mut reskh: ::core::ffi::c_double = 0.;
    let mut uflow: ::core::ffi::c_double = 0.;
    let mut fc: ::core::ffi::c_double = 0.;
    let mut epmach: ::core::ffi::c_double = 0.;
    let mut dhlgth: ::core::ffi::c_double = 0.;
    let mut j: ::core::ffi::c_int = 0;
    let mut jtw: ::core::ffi::c_int = 0;
    let mut jtwm1: ::core::ffi::c_int = 0;
    epmach = DBL_EPSILON;
    uflow = DBL_MIN;
    centr = (*a + *b) * 0.5f64;
    hlgth = (*b - *a) * 0.5f64;
    dhlgth = fabs(hlgth);
    resg = 0.0f64;
    vec[0 as ::core::ffi::c_int as usize] = centr;
    j = 1 as ::core::ffi::c_int;
    while j <= 5 as ::core::ffi::c_int {
        jtw = j << 1 as ::core::ffi::c_int;
        absc = hlgth * xgk[(jtw - 1 as ::core::ffi::c_int) as usize];
        vec[((j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as usize] = centr - absc;
        vec[(j * 2 as ::core::ffi::c_int) as usize] = centr + absc;
        j += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= 5 as ::core::ffi::c_int {
        jtwm1 = (j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
        absc = hlgth * xgk[(jtwm1 - 1 as ::core::ffi::c_int) as usize];
        vec[((j << 1 as ::core::ffi::c_int) + 9 as ::core::ffi::c_int) as usize] = centr - absc;
        vec[((j << 1 as ::core::ffi::c_int) + 10 as ::core::ffi::c_int) as usize] = centr + absc;
        j += 1;
    }
    f.expect("non-null function pointer")(
        &raw mut vec as *mut ::core::ffi::c_double,
        21 as ::core::ffi::c_int,
        ex,
    );
    fc = vec[0 as ::core::ffi::c_int as usize];
    resk = wgk[10 as ::core::ffi::c_int as usize] * fc;
    *resabs = fabs(resk);
    j = 1 as ::core::ffi::c_int;
    while j <= 5 as ::core::ffi::c_int {
        jtw = j << 1 as ::core::ffi::c_int;
        absc = hlgth * xgk[(jtw - 1 as ::core::ffi::c_int) as usize];
        fval1 = vec[((j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as usize];
        fval2 = vec[(j * 2 as ::core::ffi::c_int) as usize];
        fv1[(jtw - 1 as ::core::ffi::c_int) as usize] = fval1;
        fv2[(jtw - 1 as ::core::ffi::c_int) as usize] = fval2;
        fsum = fval1 + fval2;
        resg += wg[(j - 1 as ::core::ffi::c_int) as usize] * fsum;
        resk += wgk[(jtw - 1 as ::core::ffi::c_int) as usize] * fsum;
        *resabs += wgk[(jtw - 1 as ::core::ffi::c_int) as usize] * (fabs(fval1) + fabs(fval2));
        j += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= 5 as ::core::ffi::c_int {
        jtwm1 = (j << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
        absc = hlgth * xgk[(jtwm1 - 1 as ::core::ffi::c_int) as usize];
        fval1 = vec[((j << 1 as ::core::ffi::c_int) + 9 as ::core::ffi::c_int) as usize];
        fval2 = vec[((j << 1 as ::core::ffi::c_int) + 10 as ::core::ffi::c_int) as usize];
        fv1[(jtwm1 - 1 as ::core::ffi::c_int) as usize] = fval1;
        fv2[(jtwm1 - 1 as ::core::ffi::c_int) as usize] = fval2;
        fsum = fval1 + fval2;
        resk += wgk[(jtwm1 - 1 as ::core::ffi::c_int) as usize] * fsum;
        *resabs += wgk[(jtwm1 - 1 as ::core::ffi::c_int) as usize] * (fabs(fval1) + fabs(fval2));
        j += 1;
    }
    reskh = resk * 0.5f64;
    *resasc = wgk[10 as ::core::ffi::c_int as usize] * fabs(fc - reskh);
    j = 1 as ::core::ffi::c_int;
    while j <= 10 as ::core::ffi::c_int {
        *resasc += wgk[(j - 1 as ::core::ffi::c_int) as usize]
            * (fabs(fv1[(j - 1 as ::core::ffi::c_int) as usize] - reskh)
                + fabs(fv2[(j - 1 as ::core::ffi::c_int) as usize] - reskh));
        j += 1;
    }
    *result = resk * hlgth;
    *resabs *= dhlgth;
    *resasc *= dhlgth;
    *abserr = fabs((resk - resg) * hlgth);
    if *resasc != 0.0f64 && *abserr != 0.0f64 {
        *abserr = *resasc
            * (if 1.0f64 < pow(*abserr * 200.0f64 / *resasc, 1.5f64) {
                1.0f64
            } else {
                pow(*abserr * 200.0f64 / *resasc, 1.5f64)
            });
    }
    if *resabs > uflow / (epmach * 50.0f64) {
        *abserr = if epmach * 50.0f64 * *resabs > *abserr {
            epmach * 50.0f64 * *resabs
        } else {
            *abserr
        };
    }
}
unsafe extern "C" fn rdqpsrt(
    mut limit: *mut ::core::ffi::c_int,
    mut last: *mut ::core::ffi::c_int,
    mut maxerr: *mut ::core::ffi::c_int,
    mut ermax: *mut ::core::ffi::c_double,
    mut elist: *mut ::core::ffi::c_double,
    mut iord: *mut ::core::ffi::c_int,
    mut nrmax: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ido: ::core::ffi::c_int = 0;
    let mut jbnd: ::core::ffi::c_int = 0;
    let mut isucc: ::core::ffi::c_int = 0;
    let mut jupbn: ::core::ffi::c_int = 0;
    let mut errmin: ::core::ffi::c_double = 0.;
    let mut errmax: ::core::ffi::c_double = 0.;
    iord = iord.offset(-1);
    elist = elist.offset(-1);
    if *last <= 2 as ::core::ffi::c_int {
        *iord.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
        *iord.offset(2 as ::core::ffi::c_int as isize) = 2 as ::core::ffi::c_int;
    } else {
        errmax = *elist.offset(*maxerr as isize);
        if *nrmax > 1 as ::core::ffi::c_int {
            ido = *nrmax - 1 as ::core::ffi::c_int;
            i = 1 as ::core::ffi::c_int;
            while i <= ido {
                isucc = *iord.offset((*nrmax - 1 as ::core::ffi::c_int) as isize);
                if errmax <= *elist.offset(isucc as isize) {
                    break;
                }
                *iord.offset(*nrmax as isize) = isucc;
                *nrmax -= 1;
                i += 1;
            }
        }
        if *last > *limit / 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int {
            jupbn = *limit + 3 as ::core::ffi::c_int - *last;
        } else {
            jupbn = *last;
        }
        errmin = *elist.offset(*last as isize);
        jbnd = jupbn - 1 as ::core::ffi::c_int;
        i = *nrmax + 1 as ::core::ffi::c_int;
        's_86: loop {
            if !(i <= jbnd) {
                current_block = 14359455889292382949;
                break;
            }
            isucc = *iord.offset(i as isize);
            if errmax >= *elist.offset(isucc as isize) {
                *iord.offset((i - 1 as ::core::ffi::c_int) as isize) = *maxerr;
                j = i;
                k = jbnd;
                while j <= jbnd {
                    isucc = *iord.offset(k as isize);
                    if errmin < *elist.offset(isucc as isize) {
                        *iord.offset((k + 1 as ::core::ffi::c_int) as isize) = *last;
                        current_block = 14149539463053991836;
                        break 's_86;
                    } else {
                        *iord.offset((k + 1 as ::core::ffi::c_int) as isize) = isucc;
                        j += 1;
                        k -= 1;
                    }
                }
                *iord.offset(i as isize) = *last;
                current_block = 14149539463053991836;
                break;
            } else {
                *iord.offset((i - 1 as ::core::ffi::c_int) as isize) = isucc;
                i += 1;
            }
        }
        match current_block {
            14149539463053991836 => {}
            _ => {
                *iord.offset(jbnd as isize) = *maxerr;
                *iord.offset(jupbn as isize) = *last;
            }
        }
    }
    *maxerr = *iord.offset(*nrmax as isize);
    *ermax = *elist.offset(*maxerr as isize);
}
