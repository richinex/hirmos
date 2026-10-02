use crate::runtime::{sqrt};
extern "C" {
    #[link_name = "honest_osqp_amd_postorder"]
    fn amd_postorder(
        nn: ::core::ffi::c_int,
        Parent: *mut ::core::ffi::c_int,
        Npiv: *mut ::core::ffi::c_int,
        Fsize: *mut ::core::ffi::c_int,
        Order: *mut ::core::ffi::c_int,
        Child: *mut ::core::ffi::c_int,
        Sibling: *mut ::core::ffi::c_int,
        Stack: *mut ::core::ffi::c_int,
    );
}
pub type OSQPFloat = ::core::ffi::c_double;
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_DENSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_DEFAULT_DENSE: ::core::ffi::c_double = 10.0f64;
pub const AMD_DEFAULT_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_STATUS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_NDENSE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const AMD_NCMPA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const AMD_LNZ: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const AMD_NDIV: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AMD_NMULTSUBS_LDL: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const AMD_NMULTSUBS_LU: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const AMD_DMAX: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Int_MAX: ::core::ffi::c_int = INT_MAX;
unsafe extern "C" fn clear_flag(
    mut wflg: ::core::ffi::c_int,
    mut wbig: ::core::ffi::c_int,
    mut W: *mut ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    if wflg < 2 as ::core::ffi::c_int || wflg >= wbig {
        x = 0 as ::core::ffi::c_int;
        while x < n {
            if *W.offset(x as isize) != 0 as ::core::ffi::c_int {
                *W.offset(x as isize) = 1 as ::core::ffi::c_int;
            }
            x += 1;
        }
        wflg = 2 as ::core::ffi::c_int;
    }
    return wflg;
}
#[export_name = "honest_osqp_amd_2"]
pub unsafe extern "C" fn amd_2(
    mut n: ::core::ffi::c_int,
    mut Pe: *mut ::core::ffi::c_int,
    mut Iw: *mut ::core::ffi::c_int,
    mut Len: *mut ::core::ffi::c_int,
    mut iwlen: ::core::ffi::c_int,
    mut pfree: ::core::ffi::c_int,
    mut Nv: *mut ::core::ffi::c_int,
    mut Next: *mut ::core::ffi::c_int,
    mut Last: *mut ::core::ffi::c_int,
    mut Head: *mut ::core::ffi::c_int,
    mut Elen: *mut ::core::ffi::c_int,
    mut Degree: *mut ::core::ffi::c_int,
    mut W: *mut ::core::ffi::c_int,
    mut Control: *mut OSQPFloat,
    mut Info: *mut OSQPFloat,
) {
    let mut deg: ::core::ffi::c_int = 0;
    let mut degme: ::core::ffi::c_int = 0;
    let mut dext: ::core::ffi::c_int = 0;
    let mut lemax: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut elenme: ::core::ffi::c_int = 0;
    let mut eln: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ilast: ::core::ffi::c_int = 0;
    let mut inext: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jlast: ::core::ffi::c_int = 0;
    let mut jnext: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut knt1: ::core::ffi::c_int = 0;
    let mut knt2: ::core::ffi::c_int = 0;
    let mut knt3: ::core::ffi::c_int = 0;
    let mut lenj: ::core::ffi::c_int = 0;
    let mut ln: ::core::ffi::c_int = 0;
    let mut me: ::core::ffi::c_int = 0;
    let mut mindeg: ::core::ffi::c_int = 0;
    let mut nel: ::core::ffi::c_int = 0;
    let mut nleft: ::core::ffi::c_int = 0;
    let mut nvi: ::core::ffi::c_int = 0;
    let mut nvj: ::core::ffi::c_int = 0;
    let mut nvpiv: ::core::ffi::c_int = 0;
    let mut slenme: ::core::ffi::c_int = 0;
    let mut wbig: ::core::ffi::c_int = 0;
    let mut we: ::core::ffi::c_int = 0;
    let mut wflg: ::core::ffi::c_int = 0;
    let mut wnvi: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = 0;
    let mut ndense: ::core::ffi::c_int = 0;
    let mut ncmpa: ::core::ffi::c_int = 0;
    let mut dense: ::core::ffi::c_int = 0;
    let mut aggressive: ::core::ffi::c_int = 0;
    let mut hash: ::core::ffi::c_uint = 0;
    let mut f: OSQPFloat = 0.;
    let mut r: OSQPFloat = 0.;
    let mut ndiv: OSQPFloat = 0.;
    let mut s: OSQPFloat = 0.;
    let mut nms_lu: OSQPFloat = 0.;
    let mut nms_ldl: OSQPFloat = 0.;
    let mut dmax: OSQPFloat = 0.;
    let mut alpha: OSQPFloat = 0.;
    let mut lnz: OSQPFloat = 0.;
    let mut lnzme: OSQPFloat = 0.;
    let mut p: ::core::ffi::c_int = 0;
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    let mut p3: ::core::ffi::c_int = 0;
    let mut p4: ::core::ffi::c_int = 0;
    let mut pdst: ::core::ffi::c_int = 0;
    let mut pend: ::core::ffi::c_int = 0;
    let mut pj: ::core::ffi::c_int = 0;
    let mut pme: ::core::ffi::c_int = 0;
    let mut pme1: ::core::ffi::c_int = 0;
    let mut pme2: ::core::ffi::c_int = 0;
    let mut pn: ::core::ffi::c_int = 0;
    let mut psrc: ::core::ffi::c_int = 0;
    lnz = 0 as ::core::ffi::c_int as OSQPFloat;
    ndiv = 0 as ::core::ffi::c_int as OSQPFloat;
    nms_lu = 0 as ::core::ffi::c_int as OSQPFloat;
    nms_ldl = 0 as ::core::ffi::c_int as OSQPFloat;
    dmax = 1 as ::core::ffi::c_int as OSQPFloat;
    me = EMPTY;
    mindeg = 0 as ::core::ffi::c_int;
    ncmpa = 0 as ::core::ffi::c_int;
    nel = 0 as ::core::ffi::c_int;
    lemax = 0 as ::core::ffi::c_int;
    if !Control.is_null() {
        alpha = *Control.offset(AMD_DENSE as isize);
        aggressive = (*Control.offset(AMD_AGGRESSIVE as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int;
    } else {
        alpha = AMD_DEFAULT_DENSE as OSQPFloat;
        aggressive = AMD_DEFAULT_AGGRESSIVE;
    }
    if alpha < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        dense = n - 2 as ::core::ffi::c_int;
    } else {
        dense = (alpha as ::core::ffi::c_double * sqrt(n as ::core::ffi::c_double))
            as ::core::ffi::c_int;
    }
    dense = if 16 as ::core::ffi::c_int > dense {
        16 as ::core::ffi::c_int
    } else {
        dense
    };
    dense = if n < dense { n } else { dense };
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *Last.offset(i as isize) = EMPTY;
        *Head.offset(i as isize) = EMPTY;
        *Next.offset(i as isize) = EMPTY;
        *Nv.offset(i as isize) = 1 as ::core::ffi::c_int;
        *W.offset(i as isize) = 1 as ::core::ffi::c_int;
        *Elen.offset(i as isize) = 0 as ::core::ffi::c_int;
        *Degree.offset(i as isize) = *Len.offset(i as isize);
        i += 1;
    }
    wbig = Int_MAX - n;
    wflg = clear_flag(0 as ::core::ffi::c_int, wbig, W, n);
    ndense = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        deg = *Degree.offset(i as isize);
        if deg == 0 as ::core::ffi::c_int {
            *Elen.offset(i as isize) = -(1 as ::core::ffi::c_int) - 2 as ::core::ffi::c_int;
            nel += 1;
            *Pe.offset(i as isize) = EMPTY;
            *W.offset(i as isize) = 0 as ::core::ffi::c_int;
        } else if deg > dense {
            ndense += 1;
            *Nv.offset(i as isize) = 0 as ::core::ffi::c_int;
            *Elen.offset(i as isize) = EMPTY;
            nel += 1;
            *Pe.offset(i as isize) = EMPTY;
        } else {
            inext = *Head.offset(deg as isize);
            if inext != EMPTY {
                *Last.offset(inext as isize) = i;
            }
            *Next.offset(i as isize) = inext;
            *Head.offset(deg as isize) = i;
        }
        i += 1;
    }
    while nel < n {
        deg = mindeg;
        while deg < n {
            me = *Head.offset(deg as isize);
            if me != EMPTY {
                break;
            }
            deg += 1;
        }
        mindeg = deg;
        inext = *Next.offset(me as isize);
        if inext != EMPTY {
            *Last.offset(inext as isize) = EMPTY;
        }
        *Head.offset(deg as isize) = inext;
        elenme = *Elen.offset(me as isize);
        nvpiv = *Nv.offset(me as isize);
        nel += nvpiv;
        *Nv.offset(me as isize) = -nvpiv;
        degme = 0 as ::core::ffi::c_int;
        if elenme == 0 as ::core::ffi::c_int {
            pme1 = *Pe.offset(me as isize);
            pme2 = pme1 - 1 as ::core::ffi::c_int;
            p = pme1;
            while p <= pme1 + *Len.offset(me as isize) - 1 as ::core::ffi::c_int {
                i = *Iw.offset(p as isize);
                nvi = *Nv.offset(i as isize);
                if nvi > 0 as ::core::ffi::c_int {
                    degme += nvi;
                    *Nv.offset(i as isize) = -nvi;
                    pme2 += 1;
                    *Iw.offset(pme2 as isize) = i;
                    ilast = *Last.offset(i as isize);
                    inext = *Next.offset(i as isize);
                    if inext != EMPTY {
                        *Last.offset(inext as isize) = ilast;
                    }
                    if ilast != EMPTY {
                        *Next.offset(ilast as isize) = inext;
                    } else {
                        *Head.offset(*Degree.offset(i as isize) as isize) = inext;
                    }
                }
                p += 1;
            }
        } else {
            p = *Pe.offset(me as isize);
            pme1 = pfree;
            slenme = *Len.offset(me as isize) - elenme;
            knt1 = 1 as ::core::ffi::c_int;
            while knt1 <= elenme + 1 as ::core::ffi::c_int {
                if knt1 > elenme {
                    e = me;
                    pj = p;
                    ln = slenme;
                } else {
                    let fresh0 = p;
                    p = p + 1;
                    e = *Iw.offset(fresh0 as isize);
                    pj = *Pe.offset(e as isize);
                    ln = *Len.offset(e as isize);
                }
                knt2 = 1 as ::core::ffi::c_int;
                while knt2 <= ln {
                    let fresh1 = pj;
                    pj = pj + 1;
                    i = *Iw.offset(fresh1 as isize);
                    nvi = *Nv.offset(i as isize);
                    if nvi > 0 as ::core::ffi::c_int {
                        if pfree >= iwlen {
                            *Pe.offset(me as isize) = p;
                            *Len.offset(me as isize) -= knt1;
                            if *Len.offset(me as isize) == 0 as ::core::ffi::c_int {
                                *Pe.offset(me as isize) = EMPTY;
                            }
                            *Pe.offset(e as isize) = pj;
                            *Len.offset(e as isize) = ln - knt2;
                            if *Len.offset(e as isize) == 0 as ::core::ffi::c_int {
                                *Pe.offset(e as isize) = EMPTY;
                            }
                            ncmpa += 1;
                            j = 0 as ::core::ffi::c_int;
                            while j < n {
                                pn = *Pe.offset(j as isize);
                                if pn >= 0 as ::core::ffi::c_int {
                                    *Pe.offset(j as isize) = *Iw.offset(pn as isize);
                                    *Iw.offset(pn as isize) = -j - 2 as ::core::ffi::c_int;
                                }
                                j += 1;
                            }
                            psrc = 0 as ::core::ffi::c_int;
                            pdst = 0 as ::core::ffi::c_int;
                            pend = pme1 - 1 as ::core::ffi::c_int;
                            while psrc <= pend {
                                let fresh2 = psrc;
                                psrc = psrc + 1;
                                j = -*Iw.offset(fresh2 as isize) - 2 as ::core::ffi::c_int;
                                if j >= 0 as ::core::ffi::c_int {
                                    *Iw.offset(pdst as isize) = *Pe.offset(j as isize);
                                    let fresh3 = pdst;
                                    pdst = pdst + 1;
                                    *Pe.offset(j as isize) = fresh3;
                                    lenj = *Len.offset(j as isize);
                                    knt3 = 0 as ::core::ffi::c_int;
                                    while knt3 <= lenj - 2 as ::core::ffi::c_int {
                                        let fresh4 = psrc;
                                        psrc = psrc + 1;
                                        let fresh5 = pdst;
                                        pdst = pdst + 1;
                                        *Iw.offset(fresh5 as isize) = *Iw.offset(fresh4 as isize);
                                        knt3 += 1;
                                    }
                                }
                            }
                            p1 = pdst;
                            psrc = pme1;
                            while psrc <= pfree - 1 as ::core::ffi::c_int {
                                let fresh6 = pdst;
                                pdst = pdst + 1;
                                *Iw.offset(fresh6 as isize) = *Iw.offset(psrc as isize);
                                psrc += 1;
                            }
                            pme1 = p1;
                            pfree = pdst;
                            pj = *Pe.offset(e as isize);
                            p = *Pe.offset(me as isize);
                        }
                        degme += nvi;
                        *Nv.offset(i as isize) = -nvi;
                        let fresh7 = pfree;
                        pfree = pfree + 1;
                        *Iw.offset(fresh7 as isize) = i;
                        ilast = *Last.offset(i as isize);
                        inext = *Next.offset(i as isize);
                        if inext != EMPTY {
                            *Last.offset(inext as isize) = ilast;
                        }
                        if ilast != EMPTY {
                            *Next.offset(ilast as isize) = inext;
                        } else {
                            *Head.offset(*Degree.offset(i as isize) as isize) = inext;
                        }
                    }
                    knt2 += 1;
                }
                if e != me {
                    *Pe.offset(e as isize) = -me - 2 as ::core::ffi::c_int;
                    *W.offset(e as isize) = 0 as ::core::ffi::c_int;
                }
                knt1 += 1;
            }
            pme2 = pfree - 1 as ::core::ffi::c_int;
        }
        *Degree.offset(me as isize) = degme;
        *Pe.offset(me as isize) = pme1;
        *Len.offset(me as isize) = pme2 - pme1 + 1 as ::core::ffi::c_int;
        *Elen.offset(me as isize) = -(nvpiv + degme) - 2 as ::core::ffi::c_int;
        wflg = clear_flag(wflg, wbig, W, n);
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            eln = *Elen.offset(i as isize);
            if eln > 0 as ::core::ffi::c_int {
                nvi = -*Nv.offset(i as isize);
                wnvi = wflg - nvi;
                p = *Pe.offset(i as isize);
                while p <= *Pe.offset(i as isize) + eln - 1 as ::core::ffi::c_int {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we >= wflg {
                        we -= nvi;
                    } else if we != 0 as ::core::ffi::c_int {
                        we = *Degree.offset(e as isize) + wnvi;
                    }
                    *W.offset(e as isize) = we;
                    p += 1;
                }
            }
            pme += 1;
        }
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            p1 = *Pe.offset(i as isize);
            p2 = p1 + *Elen.offset(i as isize) - 1 as ::core::ffi::c_int;
            pn = p1;
            hash = 0 as ::core::ffi::c_uint;
            deg = 0 as ::core::ffi::c_int;
            if aggressive != 0 {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as ::core::ffi::c_int {
                        dext = we - wflg;
                        if dext > 0 as ::core::ffi::c_int {
                            deg += dext;
                            let fresh8 = pn;
                            pn = pn + 1;
                            *Iw.offset(fresh8 as isize) = e;
                            hash = hash.wrapping_add(e as ::core::ffi::c_uint);
                        } else {
                            *Pe.offset(e as isize) = -me - 2 as ::core::ffi::c_int;
                            *W.offset(e as isize) = 0 as ::core::ffi::c_int;
                        }
                    }
                    p += 1;
                }
            } else {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as ::core::ffi::c_int {
                        dext = we - wflg;
                        deg += dext;
                        let fresh9 = pn;
                        pn = pn + 1;
                        *Iw.offset(fresh9 as isize) = e;
                        hash = hash.wrapping_add(e as ::core::ffi::c_uint);
                    }
                    p += 1;
                }
            }
            *Elen.offset(i as isize) = pn - p1 + 1 as ::core::ffi::c_int;
            p3 = pn;
            p4 = p1 + *Len.offset(i as isize);
            p = p2 + 1 as ::core::ffi::c_int;
            while p < p4 {
                j = *Iw.offset(p as isize);
                nvj = *Nv.offset(j as isize);
                if nvj > 0 as ::core::ffi::c_int {
                    deg += nvj;
                    let fresh10 = pn;
                    pn = pn + 1;
                    *Iw.offset(fresh10 as isize) = j;
                    hash = hash.wrapping_add(j as ::core::ffi::c_uint);
                }
                p += 1;
            }
            if *Elen.offset(i as isize) == 1 as ::core::ffi::c_int && p3 == pn {
                *Pe.offset(i as isize) = -me - 2 as ::core::ffi::c_int;
                nvi = -*Nv.offset(i as isize);
                degme -= nvi;
                nvpiv += nvi;
                nel += nvi;
                *Nv.offset(i as isize) = 0 as ::core::ffi::c_int;
                *Elen.offset(i as isize) = EMPTY;
            } else {
                *Degree.offset(i as isize) = if *Degree.offset(i as isize) < deg {
                    *Degree.offset(i as isize)
                } else {
                    deg
                };
                *Iw.offset(pn as isize) = *Iw.offset(p3 as isize);
                *Iw.offset(p3 as isize) = *Iw.offset(p1 as isize);
                *Iw.offset(p1 as isize) = me;
                *Len.offset(i as isize) = pn - p1 + 1 as ::core::ffi::c_int;
                hash = hash.wrapping_rem(n as ::core::ffi::c_uint);
                j = *Head.offset(hash as isize);
                if j <= EMPTY {
                    *Next.offset(i as isize) = -j - 2 as ::core::ffi::c_int;
                    *Head.offset(hash as isize) = -i - 2 as ::core::ffi::c_int;
                } else {
                    *Next.offset(i as isize) = *Last.offset(j as isize);
                    *Last.offset(j as isize) = i;
                }
                *Last.offset(i as isize) = hash as ::core::ffi::c_int;
            }
            pme += 1;
        }
        *Degree.offset(me as isize) = degme;
        lemax = if lemax > degme { lemax } else { degme };
        wflg += lemax;
        wflg = clear_flag(wflg, wbig, W, n);
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            if *Nv.offset(i as isize) < 0 as ::core::ffi::c_int {
                hash = *Last.offset(i as isize) as ::core::ffi::c_uint;
                j = *Head.offset(hash as isize);
                if j == EMPTY {
                    i = EMPTY;
                } else if j < EMPTY {
                    i = -j - 2 as ::core::ffi::c_int;
                    *Head.offset(hash as isize) = EMPTY;
                } else {
                    i = *Last.offset(j as isize);
                    *Last.offset(j as isize) = EMPTY;
                }
                while i != EMPTY && *Next.offset(i as isize) != EMPTY {
                    ln = *Len.offset(i as isize);
                    eln = *Elen.offset(i as isize);
                    p = *Pe.offset(i as isize) + 1 as ::core::ffi::c_int;
                    while p <= *Pe.offset(i as isize) + ln - 1 as ::core::ffi::c_int {
                        *W.offset(*Iw.offset(p as isize) as isize) = wflg;
                        p += 1;
                    }
                    jlast = i;
                    j = *Next.offset(i as isize);
                    while j != EMPTY {
                        ok = (*Len.offset(j as isize) == ln && *Elen.offset(j as isize) == eln)
                            as ::core::ffi::c_int;
                        p = *Pe.offset(j as isize) + 1 as ::core::ffi::c_int;
                        while ok != 0 && p <= *Pe.offset(j as isize) + ln - 1 as ::core::ffi::c_int
                        {
                            if *W.offset(*Iw.offset(p as isize) as isize) != wflg {
                                ok = 0 as ::core::ffi::c_int;
                            }
                            p += 1;
                        }
                        if ok != 0 {
                            *Pe.offset(j as isize) = -i - 2 as ::core::ffi::c_int;
                            *Nv.offset(i as isize) += *Nv.offset(j as isize);
                            *Nv.offset(j as isize) = 0 as ::core::ffi::c_int;
                            *Elen.offset(j as isize) = EMPTY;
                            j = *Next.offset(j as isize);
                            *Next.offset(jlast as isize) = j;
                        } else {
                            jlast = j;
                            j = *Next.offset(j as isize);
                        }
                    }
                    wflg += 1;
                    i = *Next.offset(i as isize);
                }
            }
            pme += 1;
        }
        p = pme1;
        nleft = n - nel;
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            nvi = -*Nv.offset(i as isize);
            if nvi > 0 as ::core::ffi::c_int {
                *Nv.offset(i as isize) = nvi;
                deg = *Degree.offset(i as isize) + degme - nvi;
                deg = if deg < nleft - nvi { deg } else { nleft - nvi };
                inext = *Head.offset(deg as isize);
                if inext != EMPTY {
                    *Last.offset(inext as isize) = i;
                }
                *Next.offset(i as isize) = inext;
                *Last.offset(i as isize) = EMPTY;
                *Head.offset(deg as isize) = i;
                mindeg = if mindeg < deg { mindeg } else { deg };
                *Degree.offset(i as isize) = deg;
                let fresh11 = p;
                p = p + 1;
                *Iw.offset(fresh11 as isize) = i;
            }
            pme += 1;
        }
        *Nv.offset(me as isize) = nvpiv;
        *Len.offset(me as isize) = p - pme1;
        if *Len.offset(me as isize) == 0 as ::core::ffi::c_int {
            *Pe.offset(me as isize) = EMPTY;
            *W.offset(me as isize) = 0 as ::core::ffi::c_int;
        }
        if elenme != 0 as ::core::ffi::c_int {
            pfree = p;
        }
        if !Info.is_null() {
            f = nvpiv as OSQPFloat;
            r = (degme + ndense) as OSQPFloat;
            dmax = (if dmax > f + r {
                dmax as ::core::ffi::c_double
            } else {
                f as ::core::ffi::c_double + r as ::core::ffi::c_double
            }) as OSQPFloat;
            lnzme = (f as ::core::ffi::c_double * r as ::core::ffi::c_double
                + (f as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    * f as ::core::ffi::c_double
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                as OSQPFloat;
            lnz += lnzme as ::core::ffi::c_double;
            ndiv += lnzme as ::core::ffi::c_double;
            s = (f as ::core::ffi::c_double
                * r as ::core::ffi::c_double
                * r as ::core::ffi::c_double
                + r as ::core::ffi::c_double
                    * (f as ::core::ffi::c_double
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    * f as ::core::ffi::c_double
                + (f as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    * f as ::core::ffi::c_double
                    * (2 as ::core::ffi::c_int as ::core::ffi::c_double
                        * f as ::core::ffi::c_double
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    / 6 as ::core::ffi::c_int as ::core::ffi::c_double)
                as OSQPFloat;
            nms_lu += s as ::core::ffi::c_double;
            nms_ldl += (s as ::core::ffi::c_double + lnzme as ::core::ffi::c_double)
                / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    if !Info.is_null() {
        f = ndense as OSQPFloat;
        dmax = (if dmax > ndense as OSQPFloat {
            dmax as ::core::ffi::c_double
        } else {
            ndense as ::core::ffi::c_double
        }) as OSQPFloat;
        lnzme = ((f as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            * f as ::core::ffi::c_double
            / 2 as ::core::ffi::c_int as ::core::ffi::c_double) as OSQPFloat;
        lnz += lnzme as ::core::ffi::c_double;
        ndiv += lnzme as ::core::ffi::c_double;
        s = ((f as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            * f as ::core::ffi::c_double
            * (2 as ::core::ffi::c_int as ::core::ffi::c_double * f as ::core::ffi::c_double
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            / 6 as ::core::ffi::c_int as ::core::ffi::c_double) as OSQPFloat;
        nms_lu += s as ::core::ffi::c_double;
        nms_ldl += (s as ::core::ffi::c_double + lnzme as ::core::ffi::c_double)
            / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *Info.offset(AMD_LNZ as isize) = lnz;
        *Info.offset(AMD_NDIV as isize) = ndiv;
        *Info.offset(AMD_NMULTSUBS_LDL as isize) = nms_ldl;
        *Info.offset(AMD_NMULTSUBS_LU as isize) = nms_lu;
        *Info.offset(AMD_NDENSE as isize) = ndense as OSQPFloat;
        *Info.offset(AMD_DMAX as isize) = dmax;
        *Info.offset(AMD_NCMPA as isize) = ncmpa as OSQPFloat;
        *Info.offset(AMD_STATUS as isize) = AMD_OK as OSQPFloat;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *Pe.offset(i as isize) = -*Pe.offset(i as isize) - 2 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *Elen.offset(i as isize) = -*Elen.offset(i as isize) - 2 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if *Nv.offset(i as isize) == 0 as ::core::ffi::c_int {
            j = *Pe.offset(i as isize);
            if !(j == EMPTY) {
                while *Nv.offset(j as isize) == 0 as ::core::ffi::c_int {
                    j = *Pe.offset(j as isize);
                }
                e = j;
                j = i;
                while *Nv.offset(j as isize) == 0 as ::core::ffi::c_int {
                    jnext = *Pe.offset(j as isize);
                    *Pe.offset(j as isize) = e;
                    j = jnext;
                }
            }
        }
        i += 1;
    }
    amd_postorder(n, Pe, Nv, Elen, W, Head, Next, Last);
    k = 0 as ::core::ffi::c_int;
    while k < n {
        *Head.offset(k as isize) = EMPTY;
        *Next.offset(k as isize) = EMPTY;
        k += 1;
    }
    e = 0 as ::core::ffi::c_int;
    while e < n {
        k = *W.offset(e as isize);
        if k != EMPTY {
            *Head.offset(k as isize) = e;
        }
        e += 1;
    }
    nel = 0 as ::core::ffi::c_int;
    k = 0 as ::core::ffi::c_int;
    while k < n {
        e = *Head.offset(k as isize);
        if e == EMPTY {
            break;
        }
        *Next.offset(e as isize) = nel;
        nel += *Nv.offset(e as isize);
        k += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if *Nv.offset(i as isize) == 0 as ::core::ffi::c_int {
            e = *Pe.offset(i as isize);
            if e != EMPTY {
                *Next.offset(i as isize) = *Next.offset(e as isize);
                let ref mut fresh12 = *Next.offset(e as isize);
                *fresh12 += 1;
            } else {
                let fresh13 = nel;
                nel = nel + 1;
                *Next.offset(i as isize) = fresh13;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        k = *Next.offset(i as isize);
        *Last.offset(k as isize) = i;
        i += 1;
    }
}
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
