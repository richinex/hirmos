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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Target {
    pub eval: Option<
        unsafe extern "C" fn(
            i64,
            *const ::core::ffi::c_double,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_double,
    >,
    pub context: *mut ::core::ffi::c_void,
}
pub unsafe extern "C" fn newuoa_(
    mut target: *mut Target,
    mut n: *mut i64,
    mut npt: *mut i64,
    mut x: *mut ::core::ffi::c_double,
    mut rhobeg: *mut ::core::ffi::c_double,
    mut rhoend: *mut ::core::ffi::c_double,
    mut iprint: *mut i64,
    mut maxfun: *mut i64,
    mut w: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut id: i64 = 0;
    let mut np: i64 = 0;
    let mut iw: i64 = 0;
    let mut igq: i64 = 0;
    let mut ihq: i64 = 0;
    let mut ixb: i64 = 0;
    let mut ifv: i64 = 0;
    let mut ipq: i64 = 0;
    let mut ivl: i64 = 0;
    let mut ixn: i64 = 0;
    let mut ixo: i64 = 0;
    let mut ixp: i64 = 0;
    let mut ndim: i64 = 0;
    let mut nptm: i64 = 0;
    let mut ibmat: i64 = 0;
    let mut izmat: i64 = 0;
    w = w.offset(-1);
    x = x.offset(-1);
    np = *n + 1 as i64;
    nptm = *npt - np;
    if !(*npt < *n + 2 as i64 || *npt > (*n + 2 as i64) * np / 2 as i64) {
        ndim = *npt + *n;
        ixb = 1 as i64;
        ixo = ixb + *n;
        ixn = ixo + *n;
        ixp = ixn + *n;
        ifv = ixp + *n * *npt;
        igq = ifv + *npt;
        ihq = igq + *n;
        ipq = ihq + *n * np / 2 as i64;
        ibmat = ipq + *npt;
        izmat = ibmat + ndim * *n;
        id = izmat + *npt * nptm;
        ivl = id + *n;
        iw = ivl + ndim;
        newuob_(
            target,
            n,
            npt,
            x.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
            rhobeg,
            rhoend,
            iprint,
            maxfun,
            w.offset(ixb as isize) as *mut ::core::ffi::c_double,
            w.offset(ixo as isize) as *mut ::core::ffi::c_double,
            w.offset(ixn as isize) as *mut ::core::ffi::c_double,
            w.offset(ixp as isize) as *mut ::core::ffi::c_double,
            w.offset(ifv as isize) as *mut ::core::ffi::c_double,
            w.offset(igq as isize) as *mut ::core::ffi::c_double,
            w.offset(ihq as isize) as *mut ::core::ffi::c_double,
            w.offset(ipq as isize) as *mut ::core::ffi::c_double,
            w.offset(ibmat as isize) as *mut ::core::ffi::c_double,
            w.offset(izmat as isize) as *mut ::core::ffi::c_double,
            &raw mut ndim,
            w.offset(id as isize) as *mut ::core::ffi::c_double,
            w.offset(ivl as isize) as *mut ::core::ffi::c_double,
            w.offset(iw as isize) as *mut ::core::ffi::c_double,
        );
    }
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn newuob_(
    mut target: *mut Target,
    mut n: *mut i64,
    mut npt: *mut i64,
    mut x: *mut ::core::ffi::c_double,
    mut rhobeg: *mut ::core::ffi::c_double,
    mut rhoend: *mut ::core::ffi::c_double,
    mut iprint: *mut i64,
    mut maxfun: *mut i64,
    mut xbase: *mut ::core::ffi::c_double,
    mut xopt: *mut ::core::ffi::c_double,
    mut xnew: *mut ::core::ffi::c_double,
    mut xpt: *mut ::core::ffi::c_double,
    mut fval: *mut ::core::ffi::c_double,
    mut gq: *mut ::core::ffi::c_double,
    mut hq: *mut ::core::ffi::c_double,
    mut pq: *mut ::core::ffi::c_double,
    mut bmat: *mut ::core::ffi::c_double,
    mut zmat: *mut ::core::ffi::c_double,
    mut ndim: *mut i64,
    mut d__: *mut ::core::ffi::c_double,
    mut vlag: *mut ::core::ffi::c_double,
    mut w: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut xpt_dim1: i64 = 0;
    let mut xpt_offset: i64 = 0;
    let mut bmat_dim1: i64 = 0;
    let mut bmat_offset: i64 = 0;
    let mut zmat_dim1: i64 = 0;
    let mut zmat_offset: i64 = 0;
    let mut i__1: i64 = 0;
    let mut i__2: i64 = 0;
    let mut i__3: i64 = 0;
    let mut d__1: ::core::ffi::c_double = 0.;
    let mut d__2: ::core::ffi::c_double = 0.;
    let mut d__3: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut i__: i64 = 0;
    let mut j: i64 = 0;
    let mut k: i64 = 0;
    let mut ih: i64 = 0;
    let mut nf: i64 = 0;
    let mut nh: i64 = 0;
    let mut ip: i64 = 0;
    let mut jp: i64 = 0;
    let mut dx: ::core::ffi::c_double = 0.;
    let mut np: i64 = 0;
    let mut nfm: i64 = 0;
    let mut one: ::core::ffi::c_double = 0.;
    let mut idz: i64 = 0;
    let mut dsq: ::core::ffi::c_double = 0.;
    let mut rho: ::core::ffi::c_double = 0.;
    let mut ipt: i64 = 0;
    let mut jpt: i64 = 0;
    let mut sum: ::core::ffi::c_double = 0.;
    let mut fbeg: ::core::ffi::c_double = 0.;
    let mut diff: ::core::ffi::c_double = 0.;
    let mut half: ::core::ffi::c_double = 0.;
    let mut beta: ::core::ffi::c_double = 0.;
    let mut nfmm: i64 = 0;
    let mut gisq: ::core::ffi::c_double = 0.;
    let mut knew: i64 = 0;
    let mut temp: ::core::ffi::c_double = 0.;
    let mut suma: ::core::ffi::c_double = 0.;
    let mut sumb: ::core::ffi::c_double = 0.;
    let mut fopt: ::core::ffi::c_double = 0.;
    let mut bsum: ::core::ffi::c_double = 0.;
    let mut gqsq: ::core::ffi::c_double = 0.;
    let mut kopt: i64 = 0;
    let mut nptm: i64 = 0;
    let mut zero: ::core::ffi::c_double = 0.;
    let mut xipt: ::core::ffi::c_double = 0.;
    let mut xjpt: ::core::ffi::c_double = 0.;
    let mut sumz: ::core::ffi::c_double = 0.;
    let mut diffa: ::core::ffi::c_double = 0.;
    let mut diffb: ::core::ffi::c_double = 0.;
    let mut diffc: ::core::ffi::c_double = 0.;
    let mut hdiag: ::core::ffi::c_double = 0.;
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut delta: ::core::ffi::c_double = 0.;
    let mut recip: ::core::ffi::c_double = 0.;
    let mut reciq: ::core::ffi::c_double = 0.;
    let mut fsave: ::core::ffi::c_double = 0.;
    let mut ksave: i64 = 0;
    let mut nfsav: i64 = 0;
    let mut itemp: i64 = 0;
    let mut dnorm: ::core::ffi::c_double = 0.;
    let mut ratio: ::core::ffi::c_double = 0.;
    let mut dstep: ::core::ffi::c_double = 0.;
    let mut tenth: ::core::ffi::c_double = 0.;
    let mut vquad: ::core::ffi::c_double = 0.;
    let mut ktemp: i64 = 0;
    let mut tempq: ::core::ffi::c_double = 0.;
    let mut itest: i64 = 0;
    let mut rhosq: ::core::ffi::c_double = 0.;
    let mut detrat: ::core::ffi::c_double = 0.;
    let mut crvmin: ::core::ffi::c_double = 0.;
    let mut nftest: i64 = 0;
    let mut distsq: ::core::ffi::c_double = 0.;
    #[export_name = "trsapp_"]
    pub unsafe extern "C" fn trsapp__0(
        mut n_0: *mut i64,
        mut npt_0: *mut i64,
        mut xopt_0: *mut ::core::ffi::c_double,
        mut xpt_0: *mut ::core::ffi::c_double,
        mut gq_0: *mut ::core::ffi::c_double,
        mut hq_0: *mut ::core::ffi::c_double,
        mut pq_0: *mut ::core::ffi::c_double,
        mut delta_0: *mut ::core::ffi::c_double,
        mut step: *mut ::core::ffi::c_double,
        mut d___0: *mut ::core::ffi::c_double,
        mut g: *mut ::core::ffi::c_double,
        mut hd: *mut ::core::ffi::c_double,
        mut hs: *mut ::core::ffi::c_double,
        mut crvmin_0: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int {
        let mut current_block: u64;
        let mut xpt_dim1_0: i64 = 0;
        let mut xpt_offset_0: i64 = 0;
        let mut i__1_0: i64 = 0;
        let mut i__2_0: i64 = 0;
        let mut d__1_0: ::core::ffi::c_double = 0.;
        let mut d__2_0: ::core::ffi::c_double = 0.;
        let mut i___0: i64 = 0;
        let mut j_0: i64 = 0;
        let mut k_0: i64 = 0;
        let mut dd: ::core::ffi::c_double = 0.;
        let mut cf: ::core::ffi::c_double = 0.;
        let mut dg: ::core::ffi::c_double = 0.;
        let mut gg: ::core::ffi::c_double = 0.;
        let mut ih_0: i64 = 0;
        let mut ds: ::core::ffi::c_double = 0.;
        let mut sg: ::core::ffi::c_double = 0.;
        let mut iu: i64 = 0;
        let mut ss: ::core::ffi::c_double = 0.;
        let mut dhd: ::core::ffi::c_double = 0.;
        let mut dhs: ::core::ffi::c_double = 0.;
        let mut cth: ::core::ffi::c_double = 0.;
        let mut sgk: ::core::ffi::c_double = 0.;
        let mut shs: ::core::ffi::c_double = 0.;
        let mut sth: ::core::ffi::c_double = 0.;
        let mut qadd: ::core::ffi::c_double = 0.;
        let mut half_0: ::core::ffi::c_double = 0.;
        let mut qbeg: ::core::ffi::c_double = 0.;
        let mut qred: ::core::ffi::c_double = 0.;
        let mut qmin: ::core::ffi::c_double = 0.;
        let mut temp_0: ::core::ffi::c_double = 0.;
        let mut qsav: ::core::ffi::c_double = 0.;
        let mut qnew: ::core::ffi::c_double = 0.;
        let mut zero_0: ::core::ffi::c_double = 0.;
        let mut ggbeg: ::core::ffi::c_double = 0.;
        let mut alpha_0: ::core::ffi::c_double = 0.;
        let mut angle: ::core::ffi::c_double = 0.;
        let mut reduc: ::core::ffi::c_double = 0.;
        let mut iterc: i64 = 0;
        let mut ggsav: ::core::ffi::c_double = 0.;
        let mut delsq: ::core::ffi::c_double = 0.;
        let mut tempa: ::core::ffi::c_double = 0.;
        let mut tempb: ::core::ffi::c_double = 0.;
        let mut isave: i64 = 0;
        let mut bstep: ::core::ffi::c_double = 0.;
        let mut ratio_0: ::core::ffi::c_double = 0.;
        let mut twopi: ::core::ffi::c_double = 0.;
        let mut itersw: i64 = 0;
        let mut angtest: ::core::ffi::c_double = 0.;
        let mut itermax: i64 = 0;
        xpt_dim1_0 = *npt_0;
        xpt_offset_0 = 1 as i64 + xpt_dim1_0;
        xpt_0 = xpt_0.offset(-(xpt_offset_0 as isize));
        xopt_0 = xopt_0.offset(-1);
        gq_0 = gq_0.offset(-1);
        hq_0 = hq_0.offset(-1);
        pq_0 = pq_0.offset(-1);
        step = step.offset(-1);
        d___0 = d___0.offset(-1);
        g = g.offset(-1);
        hd = hd.offset(-1);
        hs = hs.offset(-1);
        half_0 = 0.5f64;
        zero_0 = 0.0f64;
        twopi = atan(1.0f64) * 8.0f64;
        delsq = *delta_0 * *delta_0;
        iterc = 0 as i64;
        itermax = *n_0;
        itersw = itermax;
        i__1_0 = *n_0;
        i___0 = 1 as i64;
        while i___0 <= i__1_0 {
            *d___0.offset(i___0 as isize) = *xopt_0.offset(i___0 as isize);
            i___0 += 1;
        }
        loop {
            i__1_0 = *n_0;
            i___0 = 1 as i64;
            while i___0 <= i__1_0 {
                *hd.offset(i___0 as isize) = zero_0;
                i___0 += 1;
            }
            i__1_0 = *npt_0;
            k_0 = 1 as i64;
            while k_0 <= i__1_0 {
                temp_0 = zero_0;
                i__2_0 = *n_0;
                j_0 = 1 as i64;
                while j_0 <= i__2_0 {
                    temp_0 += *xpt_0.offset((k_0 + j_0 * xpt_dim1_0) as isize)
                        * *d___0.offset(j_0 as isize);
                    j_0 += 1;
                }
                temp_0 *= *pq_0.offset(k_0 as isize);
                i__2_0 = *n_0;
                i___0 = 1 as i64;
                while i___0 <= i__2_0 {
                    *hd.offset(i___0 as isize) +=
                        temp_0 * *xpt_0.offset((k_0 + i___0 * xpt_dim1_0) as isize);
                    i___0 += 1;
                }
                k_0 += 1;
            }
            ih_0 = 0 as i64;
            i__2_0 = *n_0;
            j_0 = 1 as i64;
            while j_0 <= i__2_0 {
                i__1_0 = j_0;
                i___0 = 1 as i64;
                while i___0 <= i__1_0 {
                    ih_0 += 1;
                    if i___0 < j_0 {
                        *hd.offset(j_0 as isize) +=
                            *hq_0.offset(ih_0 as isize) * *d___0.offset(i___0 as isize);
                    }
                    *hd.offset(i___0 as isize) +=
                        *hq_0.offset(ih_0 as isize) * *d___0.offset(j_0 as isize);
                    i___0 += 1;
                }
                j_0 += 1;
            }
            if iterc == 0 as i64 {
                qred = zero_0;
                dd = zero_0;
                i__1_0 = *n_0;
                i___0 = 1 as i64;
                while i___0 <= i__1_0 {
                    *step.offset(i___0 as isize) = zero_0;
                    *hs.offset(i___0 as isize) = zero_0;
                    *g.offset(i___0 as isize) =
                        *gq_0.offset(i___0 as isize) + *hd.offset(i___0 as isize);
                    *d___0.offset(i___0 as isize) = -*g.offset(i___0 as isize);
                    d__1_0 = *d___0.offset(i___0 as isize);
                    dd += d__1_0 * d__1_0;
                    i___0 += 1;
                }
                *crvmin_0 = zero_0;
                if dd == zero_0 {
                    break;
                }
                ds = zero_0;
                ss = zero_0;
                gg = dd;
                ggbeg = gg;
            } else {
                if iterc <= itersw {
                    dhd = zero_0;
                    i__1_0 = *n_0;
                    j_0 = 1 as i64;
                    while j_0 <= i__1_0 {
                        dhd += *d___0.offset(j_0 as isize) * *hd.offset(j_0 as isize);
                        j_0 += 1;
                    }
                    alpha_0 = bstep;
                    if dhd > zero_0 {
                        temp_0 = dhd / dd;
                        if iterc == 1 as i64 {
                            *crvmin_0 = temp_0;
                        }
                        *crvmin_0 = if *crvmin_0 < temp_0 {
                            *crvmin_0
                        } else {
                            temp_0
                        };
                        d__1_0 = alpha_0;
                        d__2_0 = gg / dhd;
                        alpha_0 = if d__1_0 < d__2_0 { d__1_0 } else { d__2_0 };
                    }
                    qadd = alpha_0 * (gg - half_0 * alpha_0 * dhd);
                    qred += qadd;
                    ggsav = gg;
                    gg = zero_0;
                    i__1_0 = *n_0;
                    i___0 = 1 as i64;
                    while i___0 <= i__1_0 {
                        *step.offset(i___0 as isize) += alpha_0 * *d___0.offset(i___0 as isize);
                        *hs.offset(i___0 as isize) += alpha_0 * *hd.offset(i___0 as isize);
                        d__1_0 = *g.offset(i___0 as isize) + *hs.offset(i___0 as isize);
                        gg += d__1_0 * d__1_0;
                        i___0 += 1;
                    }
                    if alpha_0 < bstep {
                        if qadd <= qred * 0.01f64 {
                            break;
                        }
                        if gg <= ggbeg * 1e-4f64 {
                            break;
                        }
                        if iterc == itermax {
                            break;
                        }
                        temp_0 = gg / ggsav;
                        dd = zero_0;
                        ds = zero_0;
                        ss = zero_0;
                        i__1_0 = *n_0;
                        i___0 = 1 as i64;
                        while i___0 <= i__1_0 {
                            *d___0.offset(i___0 as isize) = temp_0 * *d___0.offset(i___0 as isize)
                                - *g.offset(i___0 as isize)
                                - *hs.offset(i___0 as isize);
                            d__1_0 = *d___0.offset(i___0 as isize);
                            dd += d__1_0 * d__1_0;
                            ds += *d___0.offset(i___0 as isize) * *step.offset(i___0 as isize);
                            d__1_0 = *step.offset(i___0 as isize);
                            ss += d__1_0 * d__1_0;
                            i___0 += 1;
                        }
                        if ds <= zero_0 {
                            break;
                        }
                        if ss < delsq {
                            current_block = 7811003389375813934;
                        } else {
                            current_block = 17441561948628420366;
                        }
                    } else {
                        current_block = 17441561948628420366;
                    }
                    match current_block {
                        7811003389375813934 => {}
                        _ => {
                            *crvmin_0 = zero_0;
                            itersw = iterc;
                            current_block = 17200749577435373778;
                        }
                    }
                } else {
                    dg = zero_0;
                    dhd = zero_0;
                    dhs = zero_0;
                    i__1_0 = *n_0;
                    i___0 = 1 as i64;
                    while i___0 <= i__1_0 {
                        dg += *d___0.offset(i___0 as isize) * *g.offset(i___0 as isize);
                        dhd += *hd.offset(i___0 as isize) * *d___0.offset(i___0 as isize);
                        dhs += *hd.offset(i___0 as isize) * *step.offset(i___0 as isize);
                        i___0 += 1;
                    }
                    cf = half_0 * (shs - dhd);
                    qbeg = sg + cf;
                    qsav = qbeg;
                    qmin = qbeg;
                    isave = 0 as i64;
                    iu = 49 as i64;
                    temp_0 = twopi / (iu + 1 as i64) as ::core::ffi::c_double;
                    i__1_0 = iu;
                    i___0 = 1 as i64;
                    while i___0 <= i__1_0 {
                        angle = i___0 as ::core::ffi::c_double * temp_0;
                        cth = cos(angle);
                        sth = sin(angle);
                        qnew = (sg + cf * cth) * cth + (dg + dhs * cth) * sth;
                        if qnew < qmin {
                            qmin = qnew;
                            isave = i___0;
                            tempa = qsav;
                        } else if i___0 == isave + 1 as i64 {
                            tempb = qnew;
                        }
                        qsav = qnew;
                        i___0 += 1;
                    }
                    if isave as ::core::ffi::c_double == zero_0 {
                        tempa = qnew;
                    }
                    if isave == iu {
                        tempb = qbeg;
                    }
                    angle = zero_0;
                    if tempa != tempb {
                        tempa -= qmin;
                        tempb -= qmin;
                        angle = half_0 * (tempa - tempb) / (tempa + tempb);
                    }
                    angle = temp_0 * (isave as ::core::ffi::c_double + angle);
                    cth = cos(angle);
                    sth = sin(angle);
                    reduc = qbeg - (sg + cf * cth) * cth - (dg + dhs * cth) * sth;
                    gg = zero_0;
                    i__1_0 = *n_0;
                    i___0 = 1 as i64;
                    while i___0 <= i__1_0 {
                        *step.offset(i___0 as isize) = cth * *step.offset(i___0 as isize)
                            + sth * *d___0.offset(i___0 as isize);
                        *hs.offset(i___0 as isize) =
                            cth * *hs.offset(i___0 as isize) + sth * *hd.offset(i___0 as isize);
                        d__1_0 = *g.offset(i___0 as isize) + *hs.offset(i___0 as isize);
                        gg += d__1_0 * d__1_0;
                        i___0 += 1;
                    }
                    qred += reduc;
                    ratio_0 = reduc / qred;
                    if !(iterc < itermax && ratio_0 > 0.01f64) {
                        break;
                    }
                    current_block = 17200749577435373778;
                }
                match current_block {
                    7811003389375813934 => {}
                    _ => {
                        if gg <= ggbeg * 1e-4f64 {
                            break;
                        }
                        sg = zero_0;
                        shs = zero_0;
                        i__1_0 = *n_0;
                        i___0 = 1 as i64;
                        while i___0 <= i__1_0 {
                            sg += *step.offset(i___0 as isize) * *g.offset(i___0 as isize);
                            shs += *step.offset(i___0 as isize) * *hs.offset(i___0 as isize);
                            i___0 += 1;
                        }
                        sgk = sg + shs;
                        angtest = sgk / sqrt(gg * delsq);
                        if angtest <= -0.99f64 {
                            break;
                        }
                        iterc += 1;
                        temp_0 = sqrt(delsq * gg - sgk * sgk);
                        tempa = delsq / temp_0;
                        tempb = sgk / temp_0;
                        i__1_0 = *n_0;
                        i___0 = 1 as i64;
                        while i___0 <= i__1_0 {
                            *d___0.offset(i___0 as isize) = tempa
                                * (*g.offset(i___0 as isize) + *hs.offset(i___0 as isize))
                                - tempb * *step.offset(i___0 as isize);
                            i___0 += 1;
                        }
                        continue;
                    }
                }
            }
            iterc += 1;
            temp_0 = delsq - ss;
            bstep = temp_0 / (ds + sqrt(ds * ds + dd * temp_0));
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut xoptsq: ::core::ffi::c_double = 0.;
    zmat_dim1 = *npt;
    zmat_offset = 1 as i64 + zmat_dim1;
    zmat = zmat.offset(-(zmat_offset as isize));
    xpt_dim1 = *npt;
    xpt_offset = 1 as i64 + xpt_dim1;
    xpt = xpt.offset(-(xpt_offset as isize));
    x = x.offset(-1);
    xbase = xbase.offset(-1);
    xopt = xopt.offset(-1);
    xnew = xnew.offset(-1);
    fval = fval.offset(-1);
    gq = gq.offset(-1);
    hq = hq.offset(-1);
    pq = pq.offset(-1);
    bmat_dim1 = *ndim;
    bmat_offset = 1 as i64 + bmat_dim1;
    bmat = bmat.offset(-(bmat_offset as isize));
    d__ = d__.offset(-1);
    vlag = vlag.offset(-1);
    w = w.offset(-1);
    half = 0.5f64;
    one = 1.0f64;
    tenth = 0.1f64;
    zero = 0.0f64;
    np = *n + 1 as i64;
    nh = *n * np / 2 as i64;
    nptm = *npt - np;
    nftest = if *maxfun > 1 as i64 {
        *maxfun
    } else {
        1 as i64
    };
    i__1 = *n;
    j = 1 as i64;
    while j <= i__1 {
        *xbase.offset(j as isize) = *x.offset(j as isize);
        i__2 = *npt;
        k = 1 as i64;
        while k <= i__2 {
            *xpt.offset((k + j * xpt_dim1) as isize) = zero;
            k += 1;
        }
        i__2 = *ndim;
        i__ = 1 as i64;
        while i__ <= i__2 {
            *bmat.offset((i__ + j * bmat_dim1) as isize) = zero;
            i__ += 1;
        }
        j += 1;
    }
    i__2 = nh;
    ih = 1 as i64;
    while ih <= i__2 {
        *hq.offset(ih as isize) = zero;
        ih += 1;
    }
    i__2 = *npt;
    k = 1 as i64;
    while k <= i__2 {
        *pq.offset(k as isize) = zero;
        i__1 = nptm;
        j = 1 as i64;
        while j <= i__1 {
            *zmat.offset((k + j * zmat_dim1) as isize) = zero;
            j += 1;
        }
        k += 1;
    }
    rhosq = *rhobeg * *rhobeg;
    recip = one / rhosq;
    reciq = sqrt(half) / rhosq;
    nf = 0 as i64;
    '_L50: loop {
        nfm = nf;
        nfmm = nf - *n;
        nf += 1;
        if nfm <= *n << 1 as ::core::ffi::c_int {
            if nfm >= 1 as i64 && nfm <= *n {
                *xpt.offset((nf + nfm * xpt_dim1) as isize) = *rhobeg;
            } else if nfm > *n {
                *xpt.offset((nf + nfmm * xpt_dim1) as isize) = -*rhobeg;
            }
        } else {
            itemp = (nfmm - 1 as i64) / *n;
            jpt = nfm - itemp * *n - *n;
            ipt = jpt + itemp;
            if ipt > *n {
                itemp = jpt;
                jpt = ipt - *n;
                ipt = itemp;
            }
            xipt = *rhobeg;
            if *fval.offset((ipt + np) as isize) < *fval.offset((ipt + 1 as i64) as isize) {
                xipt = -xipt;
            }
            xjpt = *rhobeg;
            if *fval.offset((jpt + np) as isize) < *fval.offset((jpt + 1 as i64) as isize) {
                xjpt = -xjpt;
            }
            *xpt.offset((nf + ipt * xpt_dim1) as isize) = xipt;
            *xpt.offset((nf + jpt * xpt_dim1) as isize) = xjpt;
        }
        i__1 = *n;
        j = 1 as i64;
        while j <= i__1 {
            *x.offset(j as isize) =
                *xpt.offset((nf + j * xpt_dim1) as isize) + *xbase.offset(j as isize);
            j += 1;
        }
        while !(nf > nftest) {
            f = (*target).eval.expect("non-null function pointer")(
                *n,
                x.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                (*target).context,
            );
            if nf <= *npt {
                *fval.offset(nf as isize) = f;
                if nf == 1 as i64 {
                    fbeg = f;
                    fopt = f;
                    kopt = 1 as i64;
                } else if f < fopt {
                    fopt = f;
                    kopt = nf;
                }
                if nfm <= *n << 1 as ::core::ffi::c_int {
                    if nfm >= 1 as i64 && nfm <= *n {
                        *gq.offset(nfm as isize) = (f - fbeg) / *rhobeg;
                        if *npt < nf + *n {
                            *bmat.offset((nfm * bmat_dim1 + 1 as i64) as isize) = -one / *rhobeg;
                            *bmat.offset((nf + nfm * bmat_dim1) as isize) = one / *rhobeg;
                            *bmat.offset((*npt + nfm + nfm * bmat_dim1) as isize) = -half * rhosq;
                        }
                    } else if nfm > *n {
                        *bmat.offset((nf - *n + nfmm * bmat_dim1) as isize) = half / *rhobeg;
                        *bmat.offset((nf + nfmm * bmat_dim1) as isize) = -half / *rhobeg;
                        *zmat.offset((nfmm * zmat_dim1 + 1 as i64) as isize) = -reciq - reciq;
                        *zmat.offset((nf - *n + nfmm * zmat_dim1) as isize) = reciq;
                        *zmat.offset((nf + nfmm * zmat_dim1) as isize) = reciq;
                        ih = nfmm * (nfmm + 1 as i64) / 2 as i64;
                        temp = (fbeg - f) / *rhobeg;
                        *hq.offset(ih as isize) = (*gq.offset(nfmm as isize) - temp) / *rhobeg;
                        *gq.offset(nfmm as isize) = half * (*gq.offset(nfmm as isize) + temp);
                    }
                } else {
                    ih = ipt * (ipt - 1 as i64) / 2 as i64 + jpt;
                    if xipt < zero {
                        ipt += *n;
                    }
                    if xjpt < zero {
                        jpt += *n;
                    }
                    *zmat.offset((nfmm * zmat_dim1 + 1 as i64) as isize) = recip;
                    *zmat.offset((nf + nfmm * zmat_dim1) as isize) = recip;
                    *zmat.offset((ipt + 1 as i64 + nfmm * zmat_dim1) as isize) = -recip;
                    *zmat.offset((jpt + 1 as i64 + nfmm * zmat_dim1) as isize) = -recip;
                    *hq.offset(ih as isize) = (fbeg
                        - *fval.offset((ipt + 1 as i64) as isize)
                        - *fval.offset((jpt + 1 as i64) as isize)
                        + f)
                        / (xipt * xjpt);
                }
                if nf < *npt {
                    continue '_L50;
                }
                rho = *rhobeg;
                delta = rho;
                idz = 1 as i64;
                diffa = zero;
                diffb = zero;
                itest = 0 as i64;
                xoptsq = zero;
                i__1 = *n;
                i__ = 1 as i64;
                while i__ <= i__1 {
                    *xopt.offset(i__ as isize) = *xpt.offset((kopt + i__ * xpt_dim1) as isize);
                    d__1 = *xopt.offset(i__ as isize);
                    xoptsq += d__1 * d__1;
                    i__ += 1;
                }
                current_block = 11957425925709070238;
            } else {
                if knew == -(1 as ::core::ffi::c_int) as i64 {
                    break '_L50;
                }
                vquad = zero;
                ih = 0 as i64;
                i__2 = *n;
                j = 1 as i64;
                while j <= i__2 {
                    vquad += *d__.offset(j as isize) * *gq.offset(j as isize);
                    i__1 = j;
                    i__ = 1 as i64;
                    while i__ <= i__1 {
                        ih += 1;
                        temp = *d__.offset(i__ as isize) * *xnew.offset(j as isize)
                            + *d__.offset(j as isize) * *xopt.offset(i__ as isize);
                        if i__ == j {
                            temp = half * temp;
                        }
                        vquad += temp * *hq.offset(ih as isize);
                        i__ += 1;
                    }
                    j += 1;
                }
                i__1 = *npt;
                k = 1 as i64;
                while k <= i__1 {
                    vquad += *pq.offset(k as isize) * *w.offset(k as isize);
                    k += 1;
                }
                diff = f - fopt - vquad;
                diffc = diffb;
                diffb = diffa;
                diffa = fabs(diff);
                if dnorm > rho {
                    nfsav = nf;
                }
                fsave = fopt;
                if f < fopt {
                    fopt = f;
                    xoptsq = zero;
                    i__1 = *n;
                    i__ = 1 as i64;
                    while i__ <= i__1 {
                        *xopt.offset(i__ as isize) = *xnew.offset(i__ as isize);
                        d__1 = *xopt.offset(i__ as isize);
                        xoptsq += d__1 * d__1;
                        i__ += 1;
                    }
                }
                ksave = knew;
                if knew > 0 as i64 {
                    current_block = 2973113667853626903;
                } else {
                    if vquad >= zero {
                        break '_L50;
                    }
                    ratio = (f - fsave) / vquad;
                    if ratio <= tenth {
                        delta = half * dnorm;
                    } else if ratio <= 0.7f64 {
                        d__1 = half * delta;
                        delta = if d__1 > dnorm { d__1 } else { dnorm };
                    } else {
                        d__1 = half * delta;
                        d__2 = dnorm + dnorm;
                        delta = if d__1 > d__2 { d__1 } else { d__2 };
                    }
                    if delta <= rho * 1.5f64 {
                        delta = rho;
                    }
                    d__2 = tenth * delta;
                    d__1 = if d__2 > rho { d__2 } else { rho };
                    rhosq = d__1 * d__1;
                    ktemp = 0 as i64;
                    detrat = zero;
                    if f >= fsave {
                        ktemp = kopt;
                        detrat = one;
                    }
                    i__1 = *npt;
                    k = 1 as i64;
                    while k <= i__1 {
                        hdiag = zero;
                        i__2 = nptm;
                        j = 1 as i64;
                        while j <= i__2 {
                            temp = one;
                            if j < idz {
                                temp = -one;
                            }
                            d__1 = *zmat.offset((k + j * zmat_dim1) as isize);
                            hdiag += temp * (d__1 * d__1);
                            j += 1;
                        }
                        d__2 = *vlag.offset(k as isize);
                        d__1 = beta * hdiag + d__2 * d__2;
                        temp = fabs(d__1);
                        distsq = zero;
                        i__2 = *n;
                        j = 1 as i64;
                        while j <= i__2 {
                            d__1 =
                                *xpt.offset((k + j * xpt_dim1) as isize) - *xopt.offset(j as isize);
                            distsq += d__1 * d__1;
                            j += 1;
                        }
                        if distsq > rhosq {
                            d__1 = distsq / rhosq;
                            temp *= d__1 * (d__1 * d__1);
                        }
                        if temp > detrat && k != ktemp {
                            detrat = temp;
                            knew = k;
                        }
                        k += 1;
                    }
                    if knew == 0 as i64 {
                        current_block = 16033908980573495747;
                    } else {
                        current_block = 2973113667853626903;
                    }
                }
                match current_block {
                    16033908980573495747 => {}
                    _ => {
                        update_(
                            n,
                            npt,
                            bmat.offset(bmat_offset as isize) as *mut ::core::ffi::c_double,
                            zmat.offset(zmat_offset as isize) as *mut ::core::ffi::c_double,
                            &raw mut idz,
                            ndim,
                            vlag.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            &raw mut beta,
                            &raw mut knew,
                            w.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                        );
                        *fval.offset(knew as isize) = f;
                        ih = 0 as i64;
                        i__1 = *n;
                        i__ = 1 as i64;
                        while i__ <= i__1 {
                            temp = *pq.offset(knew as isize)
                                * *xpt.offset((knew + i__ * xpt_dim1) as isize);
                            i__2 = i__;
                            j = 1 as i64;
                            while j <= i__2 {
                                ih += 1;
                                *hq.offset(ih as isize) +=
                                    temp * *xpt.offset((knew + j * xpt_dim1) as isize);
                                j += 1;
                            }
                            i__ += 1;
                        }
                        *pq.offset(knew as isize) = zero;
                        i__2 = nptm;
                        j = 1 as i64;
                        while j <= i__2 {
                            temp = diff * *zmat.offset((knew + j * zmat_dim1) as isize);
                            if j < idz {
                                temp = -temp;
                            }
                            i__1 = *npt;
                            k = 1 as i64;
                            while k <= i__1 {
                                *pq.offset(k as isize) +=
                                    temp * *zmat.offset((k + j * zmat_dim1) as isize);
                                k += 1;
                            }
                            j += 1;
                        }
                        gqsq = zero;
                        i__1 = *n;
                        i__ = 1 as i64;
                        while i__ <= i__1 {
                            *gq.offset(i__ as isize) +=
                                diff * *bmat.offset((knew + i__ * bmat_dim1) as isize);
                            d__1 = *gq.offset(i__ as isize);
                            gqsq += d__1 * d__1;
                            *xpt.offset((knew + i__ * xpt_dim1) as isize) =
                                *xnew.offset(i__ as isize);
                            i__ += 1;
                        }
                        if ksave == 0 as i64 && delta == rho {
                            if fabs(ratio) > 0.01f64 {
                                itest = 0 as i64;
                            } else {
                                i__1 = *npt;
                                k = 1 as i64;
                                while k <= i__1 {
                                    *vlag.offset(k as isize) =
                                        *fval.offset(k as isize) - *fval.offset(kopt as isize);
                                    k += 1;
                                }
                                gisq = zero;
                                i__1 = *n;
                                i__ = 1 as i64;
                                while i__ <= i__1 {
                                    sum = zero;
                                    i__2 = *npt;
                                    k = 1 as i64;
                                    while k <= i__2 {
                                        sum += *bmat.offset((k + i__ * bmat_dim1) as isize)
                                            * *vlag.offset(k as isize);
                                        k += 1;
                                    }
                                    gisq += sum * sum;
                                    *w.offset(i__ as isize) = sum;
                                    i__ += 1;
                                }
                                itest += 1;
                                if gqsq < gisq * 100.0f64 {
                                    itest = 0 as i64;
                                }
                                if itest >= 3 as i64 {
                                    i__1 = *n;
                                    i__ = 1 as i64;
                                    while i__ <= i__1 {
                                        *gq.offset(i__ as isize) = *w.offset(i__ as isize);
                                        i__ += 1;
                                    }
                                    i__1 = nh;
                                    ih = 1 as i64;
                                    while ih <= i__1 {
                                        *hq.offset(ih as isize) = zero;
                                        ih += 1;
                                    }
                                    i__1 = nptm;
                                    j = 1 as i64;
                                    while j <= i__1 {
                                        *w.offset(j as isize) = zero;
                                        i__2 = *npt;
                                        k = 1 as i64;
                                        while k <= i__2 {
                                            *w.offset(j as isize) += *vlag.offset(k as isize)
                                                * *zmat.offset((k + j * zmat_dim1) as isize);
                                            k += 1;
                                        }
                                        if j < idz {
                                            *w.offset(j as isize) = -*w.offset(j as isize);
                                        }
                                        j += 1;
                                    }
                                    i__1 = *npt;
                                    k = 1 as i64;
                                    while k <= i__1 {
                                        *pq.offset(k as isize) = zero;
                                        i__2 = nptm;
                                        j = 1 as i64;
                                        while j <= i__2 {
                                            *pq.offset(k as isize) += *zmat
                                                .offset((k + j * zmat_dim1) as isize)
                                                * *w.offset(j as isize);
                                            j += 1;
                                        }
                                        k += 1;
                                    }
                                    itest = 0 as i64;
                                }
                            }
                        }
                        if f < fsave {
                            kopt = knew;
                        }
                        if f <= fsave + tenth * vquad {
                            current_block = 16373612197937702507;
                        } else if ksave > 0 as i64 {
                            current_block = 16373612197937702507;
                        } else {
                            knew = 0 as i64;
                            current_block = 16033908980573495747;
                        }
                    }
                }
            }
            loop {
                match current_block {
                    11957425925709070238 => {
                        nfsav = nf;
                        current_block = 16373612197937702507;
                        continue;
                    }
                    16373612197937702507 => {
                        knew = 0 as i64;
                        trsapp__0(
                            n,
                            npt,
                            xopt.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            xpt.offset(xpt_offset as isize) as *mut ::core::ffi::c_double,
                            gq.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            hq.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            pq.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            &raw mut delta,
                            d__.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            w.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            w.offset(np as isize) as *mut ::core::ffi::c_double,
                            w.offset((np + *n) as isize) as *mut ::core::ffi::c_double,
                            w.offset((np + (*n << 1 as ::core::ffi::c_int)) as isize)
                                as *mut ::core::ffi::c_double,
                            &raw mut crvmin,
                        );
                        dsq = zero;
                        i__1 = *n;
                        i__ = 1 as i64;
                        while i__ <= i__1 {
                            d__1 = *d__.offset(i__ as isize);
                            dsq += d__1 * d__1;
                            i__ += 1;
                        }
                        d__1 = delta;
                        d__2 = sqrt(dsq);
                        dnorm = if d__1 < d__2 { d__1 } else { d__2 };
                        if !(dnorm < half * rho) {
                            current_block = 2877612503323235335;
                            break;
                        }
                        knew = -(1 as ::core::ffi::c_int) as i64;
                        delta = tenth * delta;
                        ratio = -1.0f64;
                        if delta <= rho * 1.5f64 {
                            delta = rho;
                        }
                        if nf <= nfsav + 2 as i64 {
                            current_block = 16033908980573495747;
                            continue;
                        }
                        temp = crvmin * 0.125f64 * rho * rho;
                        d__1 = if diffa > diffb { diffa } else { diffb };
                        if temp <= (if d__1 > diffc { d__1 } else { diffc }) {
                            current_block = 16033908980573495747;
                            continue;
                        }
                    }
                    _ => {
                        distsq = delta * 4.0f64 * delta;
                        i__2 = *npt;
                        k = 1 as i64;
                        while k <= i__2 {
                            sum = zero;
                            i__1 = *n;
                            j = 1 as i64;
                            while j <= i__1 {
                                d__1 = *xpt.offset((k + j * xpt_dim1) as isize)
                                    - *xopt.offset(j as isize);
                                sum += d__1 * d__1;
                                j += 1;
                            }
                            if sum > distsq {
                                knew = k;
                                distsq = sum;
                            }
                            k += 1;
                        }
                        if knew > 0 as i64 {
                            d__2 = tenth * sqrt(distsq);
                            d__3 = half * delta;
                            d__1 = if d__2 < d__3 { d__2 } else { d__3 };
                            dstep = if d__1 > rho { d__1 } else { rho };
                            dsq = dstep * dstep;
                            current_block = 2877612503323235335;
                            break;
                        } else {
                            if ratio > zero {
                                current_block = 16373612197937702507;
                                continue;
                            }
                            if (if delta > dnorm { delta } else { dnorm }) > rho {
                                current_block = 16373612197937702507;
                                continue;
                            }
                        }
                    }
                }
                if rho > *rhoend {
                    delta = half * rho;
                    ratio = rho / *rhoend;
                    if ratio <= 16.0f64 {
                        rho = *rhoend;
                    } else if ratio <= 250.0f64 {
                        rho = sqrt(ratio) * *rhoend;
                    } else {
                        rho = tenth * rho;
                    }
                    delta = if delta > rho { delta } else { rho };
                    current_block = 11957425925709070238;
                } else if knew == -(1 as ::core::ffi::c_int) as i64 {
                    current_block = 6857986714728531959;
                    break;
                } else {
                    break '_L50;
                }
            }
            match current_block {
                2877612503323235335 => {
                    if dsq <= xoptsq * 0.001f64 {
                        tempq = xoptsq * 0.25f64;
                        i__1 = *npt;
                        k = 1 as i64;
                        while k <= i__1 {
                            sum = zero;
                            i__2 = *n;
                            i__ = 1 as i64;
                            while i__ <= i__2 {
                                sum += *xpt.offset((k + i__ * xpt_dim1) as isize)
                                    * *xopt.offset(i__ as isize);
                                i__ += 1;
                            }
                            temp = *pq.offset(k as isize) * sum;
                            sum -= half * xoptsq;
                            *w.offset((*npt + k) as isize) = sum;
                            i__2 = *n;
                            i__ = 1 as i64;
                            while i__ <= i__2 {
                                *gq.offset(i__ as isize) +=
                                    temp * *xpt.offset((k + i__ * xpt_dim1) as isize);
                                *xpt.offset((k + i__ * xpt_dim1) as isize) -=
                                    half * *xopt.offset(i__ as isize);
                                *vlag.offset(i__ as isize) =
                                    *bmat.offset((k + i__ * bmat_dim1) as isize);
                                *w.offset(i__ as isize) = sum
                                    * *xpt.offset((k + i__ * xpt_dim1) as isize)
                                    + tempq * *xopt.offset(i__ as isize);
                                ip = *npt + i__;
                                i__3 = i__;
                                j = 1 as i64;
                                while j <= i__3 {
                                    *bmat.offset((ip + j * bmat_dim1) as isize) = *bmat
                                        .offset((ip + j * bmat_dim1) as isize)
                                        + *vlag.offset(i__ as isize) * *w.offset(j as isize)
                                        + *w.offset(i__ as isize) * *vlag.offset(j as isize);
                                    j += 1;
                                }
                                i__ += 1;
                            }
                            k += 1;
                        }
                        i__3 = nptm;
                        k = 1 as i64;
                        while k <= i__3 {
                            sumz = zero;
                            i__2 = *npt;
                            i__ = 1 as i64;
                            while i__ <= i__2 {
                                sumz += *zmat.offset((i__ + k * zmat_dim1) as isize);
                                *w.offset(i__ as isize) = *w.offset((*npt + i__) as isize)
                                    * *zmat.offset((i__ + k * zmat_dim1) as isize);
                                i__ += 1;
                            }
                            i__2 = *n;
                            j = 1 as i64;
                            while j <= i__2 {
                                sum = tempq * sumz * *xopt.offset(j as isize);
                                i__1 = *npt;
                                i__ = 1 as i64;
                                while i__ <= i__1 {
                                    sum += *w.offset(i__ as isize)
                                        * *xpt.offset((i__ + j * xpt_dim1) as isize);
                                    i__ += 1;
                                }
                                *vlag.offset(j as isize) = sum;
                                if k < idz {
                                    sum = -sum;
                                }
                                i__1 = *npt;
                                i__ = 1 as i64;
                                while i__ <= i__1 {
                                    *bmat.offset((i__ + j * bmat_dim1) as isize) +=
                                        sum * *zmat.offset((i__ + k * zmat_dim1) as isize);
                                    i__ += 1;
                                }
                                j += 1;
                            }
                            i__1 = *n;
                            i__ = 1 as i64;
                            while i__ <= i__1 {
                                ip = i__ + *npt;
                                temp = *vlag.offset(i__ as isize);
                                if k < idz {
                                    temp = -temp;
                                }
                                i__2 = i__;
                                j = 1 as i64;
                                while j <= i__2 {
                                    *bmat.offset((ip + j * bmat_dim1) as isize) +=
                                        temp * *vlag.offset(j as isize);
                                    j += 1;
                                }
                                i__ += 1;
                            }
                            k += 1;
                        }
                        ih = 0 as i64;
                        i__2 = *n;
                        j = 1 as i64;
                        while j <= i__2 {
                            *w.offset(j as isize) = zero;
                            i__1 = *npt;
                            k = 1 as i64;
                            while k <= i__1 {
                                *w.offset(j as isize) += *pq.offset(k as isize)
                                    * *xpt.offset((k + j * xpt_dim1) as isize);
                                *xpt.offset((k + j * xpt_dim1) as isize) -=
                                    half * *xopt.offset(j as isize);
                                k += 1;
                            }
                            i__1 = j;
                            i__ = 1 as i64;
                            while i__ <= i__1 {
                                ih += 1;
                                if i__ < j {
                                    *gq.offset(j as isize) +=
                                        *hq.offset(ih as isize) * *xopt.offset(i__ as isize);
                                }
                                *gq.offset(i__ as isize) +=
                                    *hq.offset(ih as isize) * *xopt.offset(j as isize);
                                *hq.offset(ih as isize) = *hq.offset(ih as isize)
                                    + *w.offset(i__ as isize) * *xopt.offset(j as isize)
                                    + *xopt.offset(i__ as isize) * *w.offset(j as isize);
                                *bmat.offset((*npt + i__ + j * bmat_dim1) as isize) =
                                    *bmat.offset((*npt + j + i__ * bmat_dim1) as isize);
                                i__ += 1;
                            }
                            j += 1;
                        }
                        i__1 = *n;
                        j = 1 as i64;
                        while j <= i__1 {
                            *xbase.offset(j as isize) += *xopt.offset(j as isize);
                            *xopt.offset(j as isize) = zero;
                            j += 1;
                        }
                        xoptsq = zero;
                    }
                    if knew > 0 as i64 {
                        biglag_(
                            n,
                            npt,
                            xopt.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            xpt.offset(xpt_offset as isize) as *mut ::core::ffi::c_double,
                            bmat.offset(bmat_offset as isize) as *mut ::core::ffi::c_double,
                            zmat.offset(zmat_offset as isize) as *mut ::core::ffi::c_double,
                            &raw mut idz,
                            ndim,
                            &raw mut knew,
                            &raw mut dstep,
                            d__.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            &raw mut alpha,
                            vlag.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            vlag.offset((*npt + 1 as i64) as isize) as *mut ::core::ffi::c_double,
                            w.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_double,
                            w.offset(np as isize) as *mut ::core::ffi::c_double,
                            w.offset((np + *n) as isize) as *mut ::core::ffi::c_double,
                        );
                    }
                    i__1 = *npt;
                    k = 1 as i64;
                    while k <= i__1 {
                        suma = zero;
                        sumb = zero;
                        sum = zero;
                        i__2 = *n;
                        j = 1 as i64;
                        while j <= i__2 {
                            suma +=
                                *xpt.offset((k + j * xpt_dim1) as isize) * *d__.offset(j as isize);
                            sumb +=
                                *xpt.offset((k + j * xpt_dim1) as isize) * *xopt.offset(j as isize);
                            sum += *bmat.offset((k + j * bmat_dim1) as isize)
                                * *d__.offset(j as isize);
                            j += 1;
                        }
                        *w.offset(k as isize) = suma * (half * suma + sumb);
                        *vlag.offset(k as isize) = sum;
                        k += 1;
                    }
                    beta = zero;
                    i__1 = nptm;
                    k = 1 as i64;
                    while k <= i__1 {
                        sum = zero;
                        i__2 = *npt;
                        i__ = 1 as i64;
                        while i__ <= i__2 {
                            sum += *zmat.offset((i__ + k * zmat_dim1) as isize)
                                * *w.offset(i__ as isize);
                            i__ += 1;
                        }
                        if k < idz {
                            beta += sum * sum;
                            sum = -sum;
                        } else {
                            beta -= sum * sum;
                        }
                        i__2 = *npt;
                        i__ = 1 as i64;
                        while i__ <= i__2 {
                            *vlag.offset(i__ as isize) +=
                                sum * *zmat.offset((i__ + k * zmat_dim1) as isize);
                            i__ += 1;
                        }
                        k += 1;
                    }
                    bsum = zero;
                    dx = zero;
                    i__2 = *n;
                    j = 1 as i64;
                    while j <= i__2 {
                        sum = zero;
                        i__1 = *npt;
                        i__ = 1 as i64;
                        while i__ <= i__1 {
                            sum += *w.offset(i__ as isize)
                                * *bmat.offset((i__ + j * bmat_dim1) as isize);
                            i__ += 1;
                        }
                        bsum += sum * *d__.offset(j as isize);
                        jp = *npt + j;
                        i__1 = *n;
                        k = 1 as i64;
                        while k <= i__1 {
                            sum += *bmat.offset((jp + k * bmat_dim1) as isize)
                                * *d__.offset(k as isize);
                            k += 1;
                        }
                        *vlag.offset(jp as isize) = sum;
                        bsum += sum * *d__.offset(j as isize);
                        dx += *d__.offset(j as isize) * *xopt.offset(j as isize);
                        j += 1;
                    }
                    beta = dx * dx + dsq * (xoptsq + dx + dx + half * dsq) + beta - bsum;
                    *vlag.offset(kopt as isize) += one;
                    if knew > 0 as i64 {
                        d__1 = *vlag.offset(knew as isize);
                        temp = one + alpha * beta / (d__1 * d__1);
                        if fabs(temp) <= 0.8f64 {
                            bigden_(
                                n,
                                npt,
                                xopt.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_double,
                                xpt.offset(xpt_offset as isize) as *mut ::core::ffi::c_double,
                                bmat.offset(bmat_offset as isize) as *mut ::core::ffi::c_double,
                                zmat.offset(zmat_offset as isize) as *mut ::core::ffi::c_double,
                                &raw mut idz,
                                ndim,
                                &raw mut kopt,
                                &raw mut knew,
                                d__.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_double,
                                w.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_double,
                                vlag.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_double,
                                &raw mut beta,
                                xnew.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_double,
                                w.offset((*ndim + 1 as i64) as isize) as *mut ::core::ffi::c_double,
                                w.offset((*ndim * 6 as i64 + 1 as i64) as isize)
                                    as *mut ::core::ffi::c_double,
                            );
                        }
                    }
                }
                _ => {}
            }
            i__2 = *n;
            i__ = 1 as i64;
            while i__ <= i__2 {
                *xnew.offset(i__ as isize) = *xopt.offset(i__ as isize) + *d__.offset(i__ as isize);
                *x.offset(i__ as isize) = *xbase.offset(i__ as isize) + *xnew.offset(i__ as isize);
                i__ += 1;
            }
            nf += 1;
        }
        nf -= 1;
        break;
    }
    if fopt <= f {
        i__2 = *n;
        i__ = 1 as i64;
        while i__ <= i__2 {
            *x.offset(i__ as isize) = *xbase.offset(i__ as isize) + *xopt.offset(i__ as isize);
            i__ += 1;
        }
        f = fopt;
    }
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn bigden_(
    mut n: *mut i64,
    mut npt: *mut i64,
    mut xopt: *mut ::core::ffi::c_double,
    mut xpt: *mut ::core::ffi::c_double,
    mut bmat: *mut ::core::ffi::c_double,
    mut zmat: *mut ::core::ffi::c_double,
    mut idz: *mut i64,
    mut ndim: *mut i64,
    mut kopt: *mut i64,
    mut knew: *mut i64,
    mut d__: *mut ::core::ffi::c_double,
    mut w: *mut ::core::ffi::c_double,
    mut vlag: *mut ::core::ffi::c_double,
    mut beta: *mut ::core::ffi::c_double,
    mut s: *mut ::core::ffi::c_double,
    mut wvec: *mut ::core::ffi::c_double,
    mut prod: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut xpt_dim1: i64 = 0;
    let mut xpt_offset: i64 = 0;
    let mut bmat_dim1: i64 = 0;
    let mut bmat_offset: i64 = 0;
    let mut zmat_dim1: i64 = 0;
    let mut zmat_offset: i64 = 0;
    let mut wvec_dim1: i64 = 0;
    let mut wvec_offset: i64 = 0;
    let mut prod_dim1: i64 = 0;
    let mut prod_offset: i64 = 0;
    let mut i__1: i64 = 0;
    let mut i__2: i64 = 0;
    let mut d__1: ::core::ffi::c_double = 0.;
    let mut i__: i64 = 0;
    let mut j: i64 = 0;
    let mut k: i64 = 0;
    let mut dd: ::core::ffi::c_double = 0.;
    let mut jc: i64 = 0;
    let mut ds: ::core::ffi::c_double = 0.;
    let mut ip: i64 = 0;
    let mut iu: i64 = 0;
    let mut nw: i64 = 0;
    let mut ss: ::core::ffi::c_double = 0.;
    let mut den: [::core::ffi::c_double; 9] = [0.; 9];
    let mut one: ::core::ffi::c_double = 0.;
    let mut par: [::core::ffi::c_double; 9] = [0.; 9];
    let mut tau: ::core::ffi::c_double = 0.;
    let mut sum: ::core::ffi::c_double = 0.;
    let mut two: ::core::ffi::c_double = 0.;
    let mut diff: ::core::ffi::c_double = 0.;
    let mut half: ::core::ffi::c_double = 0.;
    let mut temp: ::core::ffi::c_double = 0.;
    let mut ksav: i64 = 0;
    let mut step: ::core::ffi::c_double = 0.;
    let mut nptm: i64 = 0;
    let mut zero: ::core::ffi::c_double = 0.;
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut angle: ::core::ffi::c_double = 0.;
    let mut denex: [::core::ffi::c_double; 9] = [0.; 9];
    let mut iterc: i64 = 0;
    let mut tempa: ::core::ffi::c_double = 0.;
    let mut tempb: ::core::ffi::c_double = 0.;
    let mut tempc: ::core::ffi::c_double = 0.;
    let mut isave: i64 = 0;
    let mut ssden: ::core::ffi::c_double = 0.;
    let mut dtest: ::core::ffi::c_double = 0.;
    let mut quart: ::core::ffi::c_double = 0.;
    let mut xoptd: ::core::ffi::c_double = 0.;
    let mut twopi: ::core::ffi::c_double = 0.;
    let mut xopts: ::core::ffi::c_double = 0.;
    let mut denold: ::core::ffi::c_double = 0.;
    let mut denmax: ::core::ffi::c_double = 0.;
    let mut densav: ::core::ffi::c_double = 0.;
    let mut dstemp: ::core::ffi::c_double = 0.;
    let mut sumold: ::core::ffi::c_double = 0.;
    let mut sstemp: ::core::ffi::c_double = 0.;
    let mut xoptsq: ::core::ffi::c_double = 0.;
    zmat_dim1 = *npt;
    zmat_offset = 1 as i64 + zmat_dim1;
    zmat = zmat.offset(-(zmat_offset as isize));
    xpt_dim1 = *npt;
    xpt_offset = 1 as i64 + xpt_dim1;
    xpt = xpt.offset(-(xpt_offset as isize));
    xopt = xopt.offset(-1);
    prod_dim1 = *ndim;
    prod_offset = 1 as i64 + prod_dim1;
    prod = prod.offset(-(prod_offset as isize));
    wvec_dim1 = *ndim;
    wvec_offset = 1 as i64 + wvec_dim1;
    wvec = wvec.offset(-(wvec_offset as isize));
    bmat_dim1 = *ndim;
    bmat_offset = 1 as i64 + bmat_dim1;
    bmat = bmat.offset(-(bmat_offset as isize));
    d__ = d__.offset(-1);
    w = w.offset(-1);
    vlag = vlag.offset(-1);
    s = s.offset(-1);
    half = 0.5f64;
    one = 1.0f64;
    quart = 0.25f64;
    two = 2.0f64;
    zero = 0.0f64;
    twopi = atan(one) * 8.0f64;
    nptm = *npt - *n - 1 as i64;
    i__1 = *npt;
    k = 1 as i64;
    while k <= i__1 {
        *w.offset((*n + k) as isize) = zero;
        k += 1;
    }
    i__1 = nptm;
    j = 1 as i64;
    while j <= i__1 {
        temp = *zmat.offset((*knew + j * zmat_dim1) as isize);
        if j < *idz {
            temp = -temp;
        }
        i__2 = *npt;
        k = 1 as i64;
        while k <= i__2 {
            *w.offset((*n + k) as isize) += temp * *zmat.offset((k + j * zmat_dim1) as isize);
            k += 1;
        }
        j += 1;
    }
    alpha = *w.offset((*n + *knew) as isize);
    dd = zero;
    ds = zero;
    ss = zero;
    xoptsq = zero;
    i__2 = *n;
    i__ = 1 as i64;
    while i__ <= i__2 {
        d__1 = *d__.offset(i__ as isize);
        dd += d__1 * d__1;
        *s.offset(i__ as isize) =
            *xpt.offset((*knew + i__ * xpt_dim1) as isize) - *xopt.offset(i__ as isize);
        ds += *d__.offset(i__ as isize) * *s.offset(i__ as isize);
        d__1 = *s.offset(i__ as isize);
        ss += d__1 * d__1;
        d__1 = *xopt.offset(i__ as isize);
        xoptsq += d__1 * d__1;
        i__ += 1;
    }
    if ds * ds > dd * 0.99f64 * ss {
        ksav = *knew;
        dtest = ds * ds / ss;
        i__2 = *npt;
        k = 1 as i64;
        while k <= i__2 {
            if k != *kopt {
                dstemp = zero;
                sstemp = zero;
                i__1 = *n;
                i__ = 1 as i64;
                while i__ <= i__1 {
                    diff = *xpt.offset((k + i__ * xpt_dim1) as isize) - *xopt.offset(i__ as isize);
                    dstemp += *d__.offset(i__ as isize) * diff;
                    sstemp += diff * diff;
                    i__ += 1;
                }
                if dstemp * dstemp / sstemp < dtest {
                    ksav = k;
                    dtest = dstemp * dstemp / sstemp;
                    ds = dstemp;
                    ss = sstemp;
                }
            }
            k += 1;
        }
        i__2 = *n;
        i__ = 1 as i64;
        while i__ <= i__2 {
            *s.offset(i__ as isize) =
                *xpt.offset((ksav + i__ * xpt_dim1) as isize) - *xopt.offset(i__ as isize);
            i__ += 1;
        }
    }
    ssden = dd * ss - ds * ds;
    iterc = 0 as i64;
    densav = zero;
    loop {
        iterc += 1;
        temp = one / sqrt(ssden);
        xoptd = zero;
        xopts = zero;
        i__2 = *n;
        i__ = 1 as i64;
        while i__ <= i__2 {
            *s.offset(i__ as isize) =
                temp * (dd * *s.offset(i__ as isize) - ds * *d__.offset(i__ as isize));
            xoptd += *xopt.offset(i__ as isize) * *d__.offset(i__ as isize);
            xopts += *xopt.offset(i__ as isize) * *s.offset(i__ as isize);
            i__ += 1;
        }
        tempa = half * xoptd * xoptd;
        tempb = half * xopts * xopts;
        den[0 as ::core::ffi::c_int as usize] = dd * (xoptsq + half * dd) + tempa + tempb;
        den[1 as ::core::ffi::c_int as usize] = two * xoptd * dd;
        den[2 as ::core::ffi::c_int as usize] = two * xopts * dd;
        den[3 as ::core::ffi::c_int as usize] = tempa - tempb;
        den[4 as ::core::ffi::c_int as usize] = xoptd * xopts;
        i__ = 6 as i64;
        while i__ <= 9 as i64 {
            den[(i__ - 1 as i64) as usize] = zero;
            i__ += 1;
        }
        i__2 = *npt;
        k = 1 as i64;
        while k <= i__2 {
            tempa = zero;
            tempb = zero;
            tempc = zero;
            i__1 = *n;
            i__ = 1 as i64;
            while i__ <= i__1 {
                tempa += *xpt.offset((k + i__ * xpt_dim1) as isize) * *d__.offset(i__ as isize);
                tempb += *xpt.offset((k + i__ * xpt_dim1) as isize) * *s.offset(i__ as isize);
                tempc += *xpt.offset((k + i__ * xpt_dim1) as isize) * *xopt.offset(i__ as isize);
                i__ += 1;
            }
            *wvec.offset((k + wvec_dim1) as isize) = quart * (tempa * tempa + tempb * tempb);
            *wvec.offset((k + (wvec_dim1 << 1 as ::core::ffi::c_int)) as isize) = tempa * tempc;
            *wvec.offset((k + wvec_dim1 * 3 as i64) as isize) = tempb * tempc;
            *wvec.offset((k + (wvec_dim1 << 2 as ::core::ffi::c_int)) as isize) =
                quart * (tempa * tempa - tempb * tempb);
            *wvec.offset((k + wvec_dim1 * 5 as i64) as isize) = half * tempa * tempb;
            k += 1;
        }
        i__2 = *n;
        i__ = 1 as i64;
        while i__ <= i__2 {
            ip = i__ + *npt;
            *wvec.offset((ip + wvec_dim1) as isize) = zero;
            *wvec.offset((ip + (wvec_dim1 << 1 as ::core::ffi::c_int)) as isize) =
                *d__.offset(i__ as isize);
            *wvec.offset((ip + wvec_dim1 * 3 as i64) as isize) = *s.offset(i__ as isize);
            *wvec.offset((ip + (wvec_dim1 << 2 as ::core::ffi::c_int)) as isize) = zero;
            *wvec.offset((ip + wvec_dim1 * 5 as i64) as isize) = zero;
            i__ += 1;
        }
        jc = 1 as i64;
        while jc <= 5 as i64 {
            nw = *npt;
            if jc == 2 as i64 || jc == 3 as i64 {
                nw = *ndim;
            }
            i__2 = *npt;
            k = 1 as i64;
            while k <= i__2 {
                *prod.offset((k + jc * prod_dim1) as isize) = zero;
                k += 1;
            }
            i__2 = nptm;
            j = 1 as i64;
            while j <= i__2 {
                sum = zero;
                i__1 = *npt;
                k = 1 as i64;
                while k <= i__1 {
                    sum += *zmat.offset((k + j * zmat_dim1) as isize)
                        * *wvec.offset((k + jc * wvec_dim1) as isize);
                    k += 1;
                }
                if j < *idz {
                    sum = -sum;
                }
                i__1 = *npt;
                k = 1 as i64;
                while k <= i__1 {
                    *prod.offset((k + jc * prod_dim1) as isize) +=
                        sum * *zmat.offset((k + j * zmat_dim1) as isize);
                    k += 1;
                }
                j += 1;
            }
            if nw == *ndim {
                i__1 = *npt;
                k = 1 as i64;
                while k <= i__1 {
                    sum = zero;
                    i__2 = *n;
                    j = 1 as i64;
                    while j <= i__2 {
                        sum += *bmat.offset((k + j * bmat_dim1) as isize)
                            * *wvec.offset((*npt + j + jc * wvec_dim1) as isize);
                        j += 1;
                    }
                    *prod.offset((k + jc * prod_dim1) as isize) += sum;
                    k += 1;
                }
            }
            i__1 = *n;
            j = 1 as i64;
            while j <= i__1 {
                sum = zero;
                i__2 = nw;
                i__ = 1 as i64;
                while i__ <= i__2 {
                    sum += *bmat.offset((i__ + j * bmat_dim1) as isize)
                        * *wvec.offset((i__ + jc * wvec_dim1) as isize);
                    i__ += 1;
                }
                *prod.offset((*npt + j + jc * prod_dim1) as isize) = sum;
                j += 1;
            }
            jc += 1;
        }
        i__1 = *ndim;
        k = 1 as i64;
        while k <= i__1 {
            sum = zero;
            i__ = 1 as i64;
            while i__ <= 5 as i64 {
                par[(i__ - 1 as i64) as usize] = half
                    * *prod.offset((k + i__ * prod_dim1) as isize)
                    * *wvec.offset((k + i__ * wvec_dim1) as isize);
                sum += par[(i__ - 1 as i64) as usize];
                i__ += 1;
            }
            den[0 as ::core::ffi::c_int as usize] =
                den[0 as ::core::ffi::c_int as usize] - par[0 as ::core::ffi::c_int as usize] - sum;
            tempa = *prod.offset((k + prod_dim1) as isize)
                * *wvec.offset((k + (wvec_dim1 << 1 as ::core::ffi::c_int)) as isize)
                + *prod.offset((k + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize)
                    * *wvec.offset((k + wvec_dim1) as isize);
            tempb = *prod.offset((k + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize)
                * *wvec.offset((k + (wvec_dim1 << 2 as ::core::ffi::c_int)) as isize)
                + *prod.offset((k + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize)
                    * *wvec.offset((k + (wvec_dim1 << 1 as ::core::ffi::c_int)) as isize);
            tempc = *prod.offset((k + prod_dim1 * 3 as i64) as isize)
                * *wvec.offset((k + wvec_dim1 * 5 as i64) as isize)
                + *prod.offset((k + prod_dim1 * 5 as i64) as isize)
                    * *wvec.offset((k + wvec_dim1 * 3 as i64) as isize);
            den[1 as ::core::ffi::c_int as usize] =
                den[1 as ::core::ffi::c_int as usize] - tempa - half * (tempb + tempc);
            den[5 as ::core::ffi::c_int as usize] -= half * (tempb - tempc);
            tempa = *prod.offset((k + prod_dim1) as isize)
                * *wvec.offset((k + wvec_dim1 * 3 as i64) as isize)
                + *prod.offset((k + prod_dim1 * 3 as i64) as isize)
                    * *wvec.offset((k + wvec_dim1) as isize);
            tempb = *prod.offset((k + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize)
                * *wvec.offset((k + wvec_dim1 * 5 as i64) as isize)
                + *prod.offset((k + prod_dim1 * 5 as i64) as isize)
                    * *wvec.offset((k + (wvec_dim1 << 1 as ::core::ffi::c_int)) as isize);
            tempc = *prod.offset((k + prod_dim1 * 3 as i64) as isize)
                * *wvec.offset((k + (wvec_dim1 << 2 as ::core::ffi::c_int)) as isize)
                + *prod.offset((k + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize)
                    * *wvec.offset((k + wvec_dim1 * 3 as i64) as isize);
            den[2 as ::core::ffi::c_int as usize] =
                den[2 as ::core::ffi::c_int as usize] - tempa - half * (tempb - tempc);
            den[6 as ::core::ffi::c_int as usize] -= half * (tempb + tempc);
            tempa = *prod.offset((k + prod_dim1) as isize)
                * *wvec.offset((k + (wvec_dim1 << 2 as ::core::ffi::c_int)) as isize)
                + *prod.offset((k + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize)
                    * *wvec.offset((k + wvec_dim1) as isize);
            den[3 as ::core::ffi::c_int as usize] = den[3 as ::core::ffi::c_int as usize]
                - tempa
                - par[1 as ::core::ffi::c_int as usize]
                + par[2 as ::core::ffi::c_int as usize];
            tempa = *prod.offset((k + prod_dim1) as isize)
                * *wvec.offset((k + wvec_dim1 * 5 as i64) as isize)
                + *prod.offset((k + prod_dim1 * 5 as i64) as isize)
                    * *wvec.offset((k + wvec_dim1) as isize);
            tempb = *prod.offset((k + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize)
                * *wvec.offset((k + wvec_dim1 * 3 as i64) as isize)
                + *prod.offset((k + prod_dim1 * 3 as i64) as isize)
                    * *wvec.offset((k + (wvec_dim1 << 1 as ::core::ffi::c_int)) as isize);
            den[4 as ::core::ffi::c_int as usize] =
                den[4 as ::core::ffi::c_int as usize] - tempa - half * tempb;
            den[7 as ::core::ffi::c_int as usize] = den[7 as ::core::ffi::c_int as usize]
                - par[3 as ::core::ffi::c_int as usize]
                + par[4 as ::core::ffi::c_int as usize];
            tempa = *prod.offset((k + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize)
                * *wvec.offset((k + wvec_dim1 * 5 as i64) as isize)
                + *prod.offset((k + prod_dim1 * 5 as i64) as isize)
                    * *wvec.offset((k + (wvec_dim1 << 2 as ::core::ffi::c_int)) as isize);
            den[8 as ::core::ffi::c_int as usize] -= half * tempa;
            k += 1;
        }
        sum = zero;
        i__ = 1 as i64;
        while i__ <= 5 as i64 {
            d__1 = *prod.offset((*knew + i__ * prod_dim1) as isize);
            par[(i__ - 1 as i64) as usize] = half * (d__1 * d__1);
            sum += par[(i__ - 1 as i64) as usize];
            i__ += 1;
        }
        denex[0 as ::core::ffi::c_int as usize] = alpha * den[0 as ::core::ffi::c_int as usize]
            + par[0 as ::core::ffi::c_int as usize]
            + sum;
        tempa = two
            * *prod.offset((*knew + prod_dim1) as isize)
            * *prod.offset((*knew + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize);
        tempb = *prod.offset((*knew + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize)
            * *prod.offset((*knew + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize);
        tempc = *prod.offset((*knew + prod_dim1 * 3 as i64) as isize)
            * *prod.offset((*knew + prod_dim1 * 5 as i64) as isize);
        denex[1 as ::core::ffi::c_int as usize] =
            alpha * den[1 as ::core::ffi::c_int as usize] + tempa + tempb + tempc;
        denex[5 as ::core::ffi::c_int as usize] =
            alpha * den[5 as ::core::ffi::c_int as usize] + tempb - tempc;
        tempa = two
            * *prod.offset((*knew + prod_dim1) as isize)
            * *prod.offset((*knew + prod_dim1 * 3 as i64) as isize);
        tempb = *prod.offset((*knew + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize)
            * *prod.offset((*knew + prod_dim1 * 5 as i64) as isize);
        tempc = *prod.offset((*knew + prod_dim1 * 3 as i64) as isize)
            * *prod.offset((*knew + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize);
        denex[2 as ::core::ffi::c_int as usize] =
            alpha * den[2 as ::core::ffi::c_int as usize] + tempa + tempb - tempc;
        denex[6 as ::core::ffi::c_int as usize] =
            alpha * den[6 as ::core::ffi::c_int as usize] + tempb + tempc;
        tempa = two
            * *prod.offset((*knew + prod_dim1) as isize)
            * *prod.offset((*knew + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize);
        denex[3 as ::core::ffi::c_int as usize] = alpha * den[3 as ::core::ffi::c_int as usize]
            + tempa
            + par[1 as ::core::ffi::c_int as usize]
            - par[2 as ::core::ffi::c_int as usize];
        tempa = two
            * *prod.offset((*knew + prod_dim1) as isize)
            * *prod.offset((*knew + prod_dim1 * 5 as i64) as isize);
        denex[4 as ::core::ffi::c_int as usize] = alpha * den[4 as ::core::ffi::c_int as usize]
            + tempa
            + *prod.offset((*knew + (prod_dim1 << 1 as ::core::ffi::c_int)) as isize)
                * *prod.offset((*knew + prod_dim1 * 3 as i64) as isize);
        denex[7 as ::core::ffi::c_int as usize] = alpha * den[7 as ::core::ffi::c_int as usize]
            + par[3 as ::core::ffi::c_int as usize]
            - par[4 as ::core::ffi::c_int as usize];
        denex[8 as ::core::ffi::c_int as usize] = alpha * den[8 as ::core::ffi::c_int as usize]
            + *prod.offset((*knew + (prod_dim1 << 2 as ::core::ffi::c_int)) as isize)
                * *prod.offset((*knew + prod_dim1 * 5 as i64) as isize);
        sum = denex[0 as ::core::ffi::c_int as usize]
            + denex[1 as ::core::ffi::c_int as usize]
            + denex[3 as ::core::ffi::c_int as usize]
            + denex[5 as ::core::ffi::c_int as usize]
            + denex[7 as ::core::ffi::c_int as usize];
        denold = sum;
        denmax = sum;
        isave = 0 as i64;
        iu = 49 as i64;
        temp = twopi / (iu + 1 as i64) as ::core::ffi::c_double;
        par[0 as ::core::ffi::c_int as usize] = one;
        i__1 = iu;
        i__ = 1 as i64;
        while i__ <= i__1 {
            angle = i__ as ::core::ffi::c_double * temp;
            par[1 as ::core::ffi::c_int as usize] = cos(angle);
            par[2 as ::core::ffi::c_int as usize] = sin(angle);
            j = 4 as i64;
            while j <= 8 as i64 {
                par[(j - 1 as i64) as usize] = par[1 as ::core::ffi::c_int as usize]
                    * par[(j - 3 as i64) as usize]
                    - par[2 as ::core::ffi::c_int as usize] * par[(j - 2 as i64) as usize];
                par[j as usize] = par[1 as ::core::ffi::c_int as usize]
                    * par[(j - 2 as i64) as usize]
                    + par[2 as ::core::ffi::c_int as usize] * par[(j - 3 as i64) as usize];
                j += 2 as i64;
            }
            sumold = sum;
            sum = zero;
            j = 1 as i64;
            while j <= 9 as i64 {
                sum += denex[(j - 1 as i64) as usize] * par[(j - 1 as i64) as usize];
                j += 1;
            }
            if fabs(sum) > fabs(denmax) {
                denmax = sum;
                isave = i__;
                tempa = sumold;
            } else if i__ == isave + 1 as i64 {
                tempb = sum;
            }
            i__ += 1;
        }
        if isave == 0 as i64 {
            tempa = sum;
        }
        if isave == iu {
            tempb = denold;
        }
        step = zero;
        if tempa != tempb {
            tempa -= denmax;
            tempb -= denmax;
            step = half * (tempa - tempb) / (tempa + tempb);
        }
        angle = temp * (isave as ::core::ffi::c_double + step);
        par[1 as ::core::ffi::c_int as usize] = cos(angle);
        par[2 as ::core::ffi::c_int as usize] = sin(angle);
        j = 4 as i64;
        while j <= 8 as i64 {
            par[(j - 1 as i64) as usize] = par[1 as ::core::ffi::c_int as usize]
                * par[(j - 3 as i64) as usize]
                - par[2 as ::core::ffi::c_int as usize] * par[(j - 2 as i64) as usize];
            par[j as usize] = par[1 as ::core::ffi::c_int as usize] * par[(j - 2 as i64) as usize]
                + par[2 as ::core::ffi::c_int as usize] * par[(j - 3 as i64) as usize];
            j += 2 as i64;
        }
        *beta = zero;
        denmax = zero;
        j = 1 as i64;
        while j <= 9 as i64 {
            *beta += den[(j - 1 as i64) as usize] * par[(j - 1 as i64) as usize];
            denmax += denex[(j - 1 as i64) as usize] * par[(j - 1 as i64) as usize];
            j += 1;
        }
        i__1 = *ndim;
        k = 1 as i64;
        while k <= i__1 {
            *vlag.offset(k as isize) = zero;
            j = 1 as i64;
            while j <= 5 as i64 {
                *vlag.offset(k as isize) +=
                    *prod.offset((k + j * prod_dim1) as isize) * par[(j - 1 as i64) as usize];
                j += 1;
            }
            k += 1;
        }
        tau = *vlag.offset(*knew as isize);
        dd = zero;
        tempa = zero;
        tempb = zero;
        i__1 = *n;
        i__ = 1 as i64;
        while i__ <= i__1 {
            *d__.offset(i__ as isize) = par[1 as ::core::ffi::c_int as usize]
                * *d__.offset(i__ as isize)
                + par[2 as ::core::ffi::c_int as usize] * *s.offset(i__ as isize);
            *w.offset(i__ as isize) = *xopt.offset(i__ as isize) + *d__.offset(i__ as isize);
            d__1 = *d__.offset(i__ as isize);
            dd += d__1 * d__1;
            tempa += *d__.offset(i__ as isize) * *w.offset(i__ as isize);
            tempb += *w.offset(i__ as isize) * *w.offset(i__ as isize);
            i__ += 1;
        }
        if iterc >= *n {
            break;
        }
        if iterc > 1 as i64 {
            densav = if densav > denold { densav } else { denold };
        }
        if fabs(denmax) <= fabs(densav) * 1.1f64 {
            break;
        }
        densav = denmax;
        i__1 = *n;
        i__ = 1 as i64;
        while i__ <= i__1 {
            temp = tempa * *xopt.offset(i__ as isize) + tempb * *d__.offset(i__ as isize)
                - *vlag.offset((*npt + i__) as isize);
            *s.offset(i__ as isize) =
                tau * *bmat.offset((*knew + i__ * bmat_dim1) as isize) + alpha * temp;
            i__ += 1;
        }
        i__1 = *npt;
        k = 1 as i64;
        while k <= i__1 {
            sum = zero;
            i__2 = *n;
            j = 1 as i64;
            while j <= i__2 {
                sum += *xpt.offset((k + j * xpt_dim1) as isize) * *w.offset(j as isize);
                j += 1;
            }
            temp = (tau * *w.offset((*n + k) as isize) - alpha * *vlag.offset(k as isize)) * sum;
            i__2 = *n;
            i__ = 1 as i64;
            while i__ <= i__2 {
                *s.offset(i__ as isize) += temp * *xpt.offset((k + i__ * xpt_dim1) as isize);
                i__ += 1;
            }
            k += 1;
        }
        ss = zero;
        ds = zero;
        i__2 = *n;
        i__ = 1 as i64;
        while i__ <= i__2 {
            d__1 = *s.offset(i__ as isize);
            ss += d__1 * d__1;
            ds += *d__.offset(i__ as isize) * *s.offset(i__ as isize);
            i__ += 1;
        }
        ssden = dd * ss - ds * ds;
        if !(ssden >= dd * 1e-8f64 * ss) {
            break;
        }
    }
    i__2 = *ndim;
    k = 1 as i64;
    while k <= i__2 {
        *w.offset(k as isize) = zero;
        j = 1 as i64;
        while j <= 5 as i64 {
            *w.offset(k as isize) +=
                *wvec.offset((k + j * wvec_dim1) as isize) * par[(j - 1 as i64) as usize];
            j += 1;
        }
        k += 1;
    }
    *vlag.offset(*kopt as isize) += one;
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn biglag_(
    mut n: *mut i64,
    mut npt: *mut i64,
    mut xopt: *mut ::core::ffi::c_double,
    mut xpt: *mut ::core::ffi::c_double,
    mut bmat: *mut ::core::ffi::c_double,
    mut zmat: *mut ::core::ffi::c_double,
    mut idz: *mut i64,
    mut ndim: *mut i64,
    mut knew: *mut i64,
    mut delta: *mut ::core::ffi::c_double,
    mut d__: *mut ::core::ffi::c_double,
    mut alpha: *mut ::core::ffi::c_double,
    mut hcol: *mut ::core::ffi::c_double,
    mut gc: *mut ::core::ffi::c_double,
    mut gd: *mut ::core::ffi::c_double,
    mut s: *mut ::core::ffi::c_double,
    mut w: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut xpt_dim1: i64 = 0;
    let mut xpt_offset: i64 = 0;
    let mut bmat_dim1: i64 = 0;
    let mut bmat_offset: i64 = 0;
    let mut zmat_dim1: i64 = 0;
    let mut zmat_offset: i64 = 0;
    let mut i__1: i64 = 0;
    let mut i__2: i64 = 0;
    let mut d__1: ::core::ffi::c_double = 0.;
    let mut i__: i64 = 0;
    let mut j: i64 = 0;
    let mut k: i64 = 0;
    let mut dd: ::core::ffi::c_double = 0.;
    let mut gg: ::core::ffi::c_double = 0.;
    let mut iu: i64 = 0;
    let mut sp: ::core::ffi::c_double = 0.;
    let mut ss: ::core::ffi::c_double = 0.;
    let mut cf1: ::core::ffi::c_double = 0.;
    let mut cf2: ::core::ffi::c_double = 0.;
    let mut cf3: ::core::ffi::c_double = 0.;
    let mut cf4: ::core::ffi::c_double = 0.;
    let mut cf5: ::core::ffi::c_double = 0.;
    let mut dhd: ::core::ffi::c_double = 0.;
    let mut cth: ::core::ffi::c_double = 0.;
    let mut one: ::core::ffi::c_double = 0.;
    let mut tau: ::core::ffi::c_double = 0.;
    let mut sth: ::core::ffi::c_double = 0.;
    let mut sum: ::core::ffi::c_double = 0.;
    let mut half: ::core::ffi::c_double = 0.;
    let mut temp: ::core::ffi::c_double = 0.;
    let mut step: ::core::ffi::c_double = 0.;
    let mut nptm: i64 = 0;
    let mut zero: ::core::ffi::c_double = 0.;
    let mut angle: ::core::ffi::c_double = 0.;
    let mut scale: ::core::ffi::c_double = 0.;
    let mut denom: ::core::ffi::c_double = 0.;
    let mut iterc: i64 = 0;
    let mut isave: i64 = 0;
    let mut delsq: ::core::ffi::c_double = 0.;
    let mut tempa: ::core::ffi::c_double = 0.;
    let mut tempb: ::core::ffi::c_double = 0.;
    let mut twopi: ::core::ffi::c_double = 0.;
    let mut taubeg: ::core::ffi::c_double = 0.;
    let mut tauold: ::core::ffi::c_double = 0.;
    let mut taumax: ::core::ffi::c_double = 0.;
    zmat_dim1 = *npt;
    zmat_offset = 1 as i64 + zmat_dim1;
    zmat = zmat.offset(-(zmat_offset as isize));
    xpt_dim1 = *npt;
    xpt_offset = 1 as i64 + xpt_dim1;
    xpt = xpt.offset(-(xpt_offset as isize));
    xopt = xopt.offset(-1);
    bmat_dim1 = *ndim;
    bmat_offset = 1 as i64 + bmat_dim1;
    bmat = bmat.offset(-(bmat_offset as isize));
    d__ = d__.offset(-1);
    hcol = hcol.offset(-1);
    gc = gc.offset(-1);
    gd = gd.offset(-1);
    s = s.offset(-1);
    w = w.offset(-1);
    half = 0.5f64;
    one = 1.0f64;
    zero = 0.0f64;
    twopi = atan(one) * 8.0f64;
    delsq = *delta * *delta;
    nptm = *npt - *n - 1 as i64;
    iterc = 0 as i64;
    i__1 = *npt;
    k = 1 as i64;
    while k <= i__1 {
        *hcol.offset(k as isize) = zero;
        k += 1;
    }
    i__1 = nptm;
    j = 1 as i64;
    while j <= i__1 {
        temp = *zmat.offset((*knew + j * zmat_dim1) as isize);
        if j < *idz {
            temp = -temp;
        }
        i__2 = *npt;
        k = 1 as i64;
        while k <= i__2 {
            *hcol.offset(k as isize) += temp * *zmat.offset((k + j * zmat_dim1) as isize);
            k += 1;
        }
        j += 1;
    }
    *alpha = *hcol.offset(*knew as isize);
    dd = zero;
    i__2 = *n;
    i__ = 1 as i64;
    while i__ <= i__2 {
        *d__.offset(i__ as isize) =
            *xpt.offset((*knew + i__ * xpt_dim1) as isize) - *xopt.offset(i__ as isize);
        *gc.offset(i__ as isize) = *bmat.offset((*knew + i__ * bmat_dim1) as isize);
        *gd.offset(i__ as isize) = zero;
        d__1 = *d__.offset(i__ as isize);
        dd += d__1 * d__1;
        i__ += 1;
    }
    i__2 = *npt;
    k = 1 as i64;
    while k <= i__2 {
        temp = zero;
        sum = zero;
        i__1 = *n;
        j = 1 as i64;
        while j <= i__1 {
            temp += *xpt.offset((k + j * xpt_dim1) as isize) * *xopt.offset(j as isize);
            sum += *xpt.offset((k + j * xpt_dim1) as isize) * *d__.offset(j as isize);
            j += 1;
        }
        temp = *hcol.offset(k as isize) * temp;
        sum = *hcol.offset(k as isize) * sum;
        i__1 = *n;
        i__ = 1 as i64;
        while i__ <= i__1 {
            *gc.offset(i__ as isize) += temp * *xpt.offset((k + i__ * xpt_dim1) as isize);
            *gd.offset(i__ as isize) += sum * *xpt.offset((k + i__ * xpt_dim1) as isize);
            i__ += 1;
        }
        k += 1;
    }
    gg = zero;
    sp = zero;
    dhd = zero;
    i__1 = *n;
    i__ = 1 as i64;
    while i__ <= i__1 {
        d__1 = *gc.offset(i__ as isize);
        gg += d__1 * d__1;
        sp += *d__.offset(i__ as isize) * *gc.offset(i__ as isize);
        dhd += *d__.offset(i__ as isize) * *gd.offset(i__ as isize);
        i__ += 1;
    }
    scale = *delta / sqrt(dd);
    if sp * dhd < zero {
        scale = -scale;
    }
    temp = zero;
    if sp * sp > dd * 0.99f64 * gg {
        temp = one;
    }
    tau = scale * (fabs(sp) + half * scale * fabs(dhd));
    if gg * delsq < tau * 0.01f64 * tau {
        temp = one;
    }
    i__1 = *n;
    i__ = 1 as i64;
    while i__ <= i__1 {
        *d__.offset(i__ as isize) = scale * *d__.offset(i__ as isize);
        *gd.offset(i__ as isize) = scale * *gd.offset(i__ as isize);
        *s.offset(i__ as isize) = *gc.offset(i__ as isize) + temp * *gd.offset(i__ as isize);
        i__ += 1;
    }
    loop {
        iterc += 1;
        dd = zero;
        sp = zero;
        ss = zero;
        i__1 = *n;
        i__ = 1 as i64;
        while i__ <= i__1 {
            d__1 = *d__.offset(i__ as isize);
            dd += d__1 * d__1;
            sp += *d__.offset(i__ as isize) * *s.offset(i__ as isize);
            d__1 = *s.offset(i__ as isize);
            ss += d__1 * d__1;
            i__ += 1;
        }
        temp = dd * ss - sp * sp;
        if temp <= dd * 1e-8f64 * ss {
            break;
        }
        denom = sqrt(temp);
        i__1 = *n;
        i__ = 1 as i64;
        while i__ <= i__1 {
            *s.offset(i__ as isize) =
                (dd * *s.offset(i__ as isize) - sp * *d__.offset(i__ as isize)) / denom;
            *w.offset(i__ as isize) = zero;
            i__ += 1;
        }
        i__1 = *npt;
        k = 1 as i64;
        while k <= i__1 {
            sum = zero;
            i__2 = *n;
            j = 1 as i64;
            while j <= i__2 {
                sum += *xpt.offset((k + j * xpt_dim1) as isize) * *s.offset(j as isize);
                j += 1;
            }
            sum = *hcol.offset(k as isize) * sum;
            i__2 = *n;
            i__ = 1 as i64;
            while i__ <= i__2 {
                *w.offset(i__ as isize) += sum * *xpt.offset((k + i__ * xpt_dim1) as isize);
                i__ += 1;
            }
            k += 1;
        }
        cf1 = zero;
        cf2 = zero;
        cf3 = zero;
        cf4 = zero;
        cf5 = zero;
        i__2 = *n;
        i__ = 1 as i64;
        while i__ <= i__2 {
            cf1 += *s.offset(i__ as isize) * *w.offset(i__ as isize);
            cf2 += *d__.offset(i__ as isize) * *gc.offset(i__ as isize);
            cf3 += *s.offset(i__ as isize) * *gc.offset(i__ as isize);
            cf4 += *d__.offset(i__ as isize) * *gd.offset(i__ as isize);
            cf5 += *s.offset(i__ as isize) * *gd.offset(i__ as isize);
            i__ += 1;
        }
        cf1 = half * cf1;
        cf4 = half * cf4 - cf1;
        taubeg = cf1 + cf2 + cf4;
        taumax = taubeg;
        tauold = taubeg;
        isave = 0 as i64;
        iu = 49 as i64;
        temp = twopi / (iu + 1 as i64) as ::core::ffi::c_double;
        i__2 = iu;
        i__ = 1 as i64;
        while i__ <= i__2 {
            angle = i__ as ::core::ffi::c_double * temp;
            cth = cos(angle);
            sth = sin(angle);
            tau = cf1 + (cf2 + cf4 * cth) * cth + (cf3 + cf5 * cth) * sth;
            if fabs(tau) > fabs(taumax) {
                taumax = tau;
                isave = i__;
                tempa = tauold;
            } else if i__ == isave + 1 as i64 {
                tempb = tau;
            }
            tauold = tau;
            i__ += 1;
        }
        if isave == 0 as i64 {
            tempa = tau;
        }
        if isave == iu {
            tempb = taubeg;
        }
        step = zero;
        if tempa != tempb {
            tempa -= taumax;
            tempb -= taumax;
            step = half * (tempa - tempb) / (tempa + tempb);
        }
        angle = temp * (isave as ::core::ffi::c_double + step);
        cth = cos(angle);
        sth = sin(angle);
        tau = cf1 + (cf2 + cf4 * cth) * cth + (cf3 + cf5 * cth) * sth;
        i__2 = *n;
        i__ = 1 as i64;
        while i__ <= i__2 {
            *d__.offset(i__ as isize) =
                cth * *d__.offset(i__ as isize) + sth * *s.offset(i__ as isize);
            *gd.offset(i__ as isize) =
                cth * *gd.offset(i__ as isize) + sth * *w.offset(i__ as isize);
            *s.offset(i__ as isize) = *gc.offset(i__ as isize) + *gd.offset(i__ as isize);
            i__ += 1;
        }
        if fabs(tau) <= fabs(taubeg) * 1.1f64 {
            break;
        }
        if !(iterc < *n) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn update_(
    mut n: *mut i64,
    mut npt: *mut i64,
    mut bmat: *mut ::core::ffi::c_double,
    mut zmat: *mut ::core::ffi::c_double,
    mut idz: *mut i64,
    mut ndim: *mut i64,
    mut vlag: *mut ::core::ffi::c_double,
    mut beta: *mut ::core::ffi::c_double,
    mut knew: *mut i64,
    mut w: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut bmat_dim1: i64 = 0;
    let mut bmat_offset: i64 = 0;
    let mut zmat_dim1: i64 = 0;
    let mut zmat_offset: i64 = 0;
    let mut i__1: i64 = 0;
    let mut i__2: i64 = 0;
    let mut d__1: ::core::ffi::c_double = 0.;
    let mut d__2: ::core::ffi::c_double = 0.;
    let mut i__: i64 = 0;
    let mut j: i64 = 0;
    let mut ja: i64 = 0;
    let mut jb: i64 = 0;
    let mut jl: i64 = 0;
    let mut jp: i64 = 0;
    let mut one: ::core::ffi::c_double = 0.;
    let mut tau: ::core::ffi::c_double = 0.;
    let mut temp: ::core::ffi::c_double = 0.;
    let mut nptm: i64 = 0;
    let mut zero: ::core::ffi::c_double = 0.;
    let mut iflag: i64 = 0;
    let mut scala: ::core::ffi::c_double = 0.;
    let mut scalb: ::core::ffi::c_double = 0.;
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut denom: ::core::ffi::c_double = 0.;
    let mut tempa: ::core::ffi::c_double = 0.;
    let mut tempb: ::core::ffi::c_double = 0.;
    let mut tausq: ::core::ffi::c_double = 0.;
    zmat_dim1 = *npt;
    zmat_offset = 1 as i64 + zmat_dim1;
    zmat = zmat.offset(-(zmat_offset as isize));
    bmat_dim1 = *ndim;
    bmat_offset = 1 as i64 + bmat_dim1;
    bmat = bmat.offset(-(bmat_offset as isize));
    vlag = vlag.offset(-1);
    w = w.offset(-1);
    one = 1.0f64;
    zero = 0.0f64;
    nptm = *npt - *n - 1 as i64;
    jl = 1 as i64;
    i__1 = nptm;
    j = 2 as i64;
    while j <= i__1 {
        if j == *idz {
            jl = *idz;
        } else if *zmat.offset((*knew + j * zmat_dim1) as isize) != zero {
            d__1 = *zmat.offset((*knew + jl * zmat_dim1) as isize);
            d__2 = *zmat.offset((*knew + j * zmat_dim1) as isize);
            temp = sqrt(d__1 * d__1 + d__2 * d__2);
            tempa = *zmat.offset((*knew + jl * zmat_dim1) as isize) / temp;
            tempb = *zmat.offset((*knew + j * zmat_dim1) as isize) / temp;
            i__2 = *npt;
            i__ = 1 as i64;
            while i__ <= i__2 {
                temp = tempa * *zmat.offset((i__ + jl * zmat_dim1) as isize)
                    + tempb * *zmat.offset((i__ + j * zmat_dim1) as isize);
                *zmat.offset((i__ + j * zmat_dim1) as isize) = tempa
                    * *zmat.offset((i__ + j * zmat_dim1) as isize)
                    - tempb * *zmat.offset((i__ + jl * zmat_dim1) as isize);
                *zmat.offset((i__ + jl * zmat_dim1) as isize) = temp;
                i__ += 1;
            }
            *zmat.offset((*knew + j * zmat_dim1) as isize) = zero;
        }
        j += 1;
    }
    tempa = *zmat.offset((*knew + zmat_dim1) as isize);
    if *idz >= 2 as i64 {
        tempa = -tempa;
    }
    if jl > 1 as i64 {
        tempb = *zmat.offset((*knew + jl * zmat_dim1) as isize);
    }
    i__1 = *npt;
    i__ = 1 as i64;
    while i__ <= i__1 {
        *w.offset(i__ as isize) = tempa * *zmat.offset((i__ + zmat_dim1) as isize);
        if jl > 1 as i64 {
            *w.offset(i__ as isize) += tempb * *zmat.offset((i__ + jl * zmat_dim1) as isize);
        }
        i__ += 1;
    }
    alpha = *w.offset(*knew as isize);
    tau = *vlag.offset(*knew as isize);
    tausq = tau * tau;
    denom = alpha * *beta + tausq;
    *vlag.offset(*knew as isize) -= one;
    iflag = 0 as i64;
    if jl == 1 as i64 {
        temp = sqrt(fabs(denom));
        tempb = tempa / temp;
        tempa = tau / temp;
        i__1 = *npt;
        i__ = 1 as i64;
        while i__ <= i__1 {
            *zmat.offset((i__ + zmat_dim1) as isize) = tempa
                * *zmat.offset((i__ + zmat_dim1) as isize)
                - tempb * *vlag.offset(i__ as isize);
            i__ += 1;
        }
        if *idz == 1 as i64 && temp < zero {
            *idz = 2 as i64;
        }
        if *idz >= 2 as i64 && temp >= zero {
            iflag = 1 as i64;
        }
    } else {
        ja = 1 as i64;
        if *beta >= zero {
            ja = jl;
        }
        jb = jl + 1 as i64 - ja;
        temp = *zmat.offset((*knew + jb * zmat_dim1) as isize) / denom;
        tempa = temp * *beta;
        tempb = temp * tau;
        temp = *zmat.offset((*knew + ja * zmat_dim1) as isize);
        scala = one / sqrt(fabs(*beta) * temp * temp + tausq);
        scalb = scala * sqrt(fabs(denom));
        i__1 = *npt;
        i__ = 1 as i64;
        while i__ <= i__1 {
            *zmat.offset((i__ + ja * zmat_dim1) as isize) = scala
                * (tau * *zmat.offset((i__ + ja * zmat_dim1) as isize)
                    - temp * *vlag.offset(i__ as isize));
            *zmat.offset((i__ + jb * zmat_dim1) as isize) = scalb
                * (*zmat.offset((i__ + jb * zmat_dim1) as isize)
                    - tempa * *w.offset(i__ as isize)
                    - tempb * *vlag.offset(i__ as isize));
            i__ += 1;
        }
        if denom <= zero {
            if *beta < zero {
                *idz += 1;
            }
            if *beta >= zero {
                iflag = 1 as i64;
            }
        }
    }
    if iflag == 1 as i64 {
        *idz -= 1;
        i__1 = *npt;
        i__ = 1 as i64;
        while i__ <= i__1 {
            temp = *zmat.offset((i__ + zmat_dim1) as isize);
            *zmat.offset((i__ + zmat_dim1) as isize) =
                *zmat.offset((i__ + *idz * zmat_dim1) as isize);
            *zmat.offset((i__ + *idz * zmat_dim1) as isize) = temp;
            i__ += 1;
        }
    }
    i__1 = *n;
    j = 1 as i64;
    while j <= i__1 {
        jp = *npt + j;
        *w.offset(jp as isize) = *bmat.offset((*knew + j * bmat_dim1) as isize);
        tempa = (alpha * *vlag.offset(jp as isize) - tau * *w.offset(jp as isize)) / denom;
        tempb = (-*beta * *w.offset(jp as isize) - tau * *vlag.offset(jp as isize)) / denom;
        i__2 = jp;
        i__ = 1 as i64;
        while i__ <= i__2 {
            *bmat.offset((i__ + j * bmat_dim1) as isize) = *bmat
                .offset((i__ + j * bmat_dim1) as isize)
                + tempa * *vlag.offset(i__ as isize)
                + tempb * *w.offset(i__ as isize);
            if i__ > *npt {
                *bmat.offset((jp + (i__ - *npt) * bmat_dim1) as isize) =
                    *bmat.offset((i__ + j * bmat_dim1) as isize);
            }
            i__ += 1;
        }
        j += 1;
    }
    return 0 as ::core::ffi::c_int;
}
