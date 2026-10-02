use crate::runtime::{sqrt};
extern "C" {
    fn amd_l_postorder(
        nn: i64,
        Parent: *mut i64,
        Npiv: *mut i64,
        Fsize: *mut i64,
        Order: *mut i64,
        Child: *mut i64,
        Sibling: *mut i64,
        Stack: *mut i64,
    );
}
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SuiteSparse_long_max: i64 = LONG_MAX;
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
pub const Int_MAX: i64 = SuiteSparse_long_max;
unsafe extern "C" fn clear_flag(
    mut wflg: i64,
    mut wbig: i64,
    mut W: *mut i64,
    mut n: i64,
) -> i64 {
    let mut x: i64 = 0;
    if wflg < 2 as i64 || wflg >= wbig {
        x = 0 as i64;
        while x < n {
            if *W.offset(x as isize) != 0 as i64 {
                *W.offset(x as isize) = 1 as i64;
            }
            x += 1;
        }
        wflg = 2 as i64;
    }
    return wflg;
}
#[no_mangle]
pub unsafe extern "C" fn amd_l2(
    mut n: i64,
    mut Pe: *mut i64,
    mut Iw: *mut i64,
    mut Len: *mut i64,
    mut iwlen: i64,
    mut pfree: i64,
    mut Nv: *mut i64,
    mut Next: *mut i64,
    mut Last: *mut i64,
    mut Head: *mut i64,
    mut Elen: *mut i64,
    mut Degree: *mut i64,
    mut W: *mut i64,
    mut Control: *mut ::core::ffi::c_double,
    mut Info: *mut ::core::ffi::c_double,
) {
    let mut deg: i64 = 0;
    let mut degme: i64 = 0;
    let mut dext: i64 = 0;
    let mut lemax: i64 = 0;
    let mut e: i64 = 0;
    let mut elenme: i64 = 0;
    let mut eln: i64 = 0;
    let mut i: i64 = 0;
    let mut ilast: i64 = 0;
    let mut inext: i64 = 0;
    let mut j: i64 = 0;
    let mut jlast: i64 = 0;
    let mut jnext: i64 = 0;
    let mut k: i64 = 0;
    let mut knt1: i64 = 0;
    let mut knt2: i64 = 0;
    let mut knt3: i64 = 0;
    let mut lenj: i64 = 0;
    let mut ln: i64 = 0;
    let mut me: i64 = 0;
    let mut mindeg: i64 = 0;
    let mut nel: i64 = 0;
    let mut nleft: i64 = 0;
    let mut nvi: i64 = 0;
    let mut nvj: i64 = 0;
    let mut nvpiv: i64 = 0;
    let mut slenme: i64 = 0;
    let mut wbig: i64 = 0;
    let mut we: i64 = 0;
    let mut wflg: i64 = 0;
    let mut wnvi: i64 = 0;
    let mut ok: i64 = 0;
    let mut ndense: i64 = 0;
    let mut ncmpa: i64 = 0;
    let mut dense: i64 = 0;
    let mut aggressive: i64 = 0;
    let mut hash: u64 = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    let mut ndiv: ::core::ffi::c_double = 0.;
    let mut s: ::core::ffi::c_double = 0.;
    let mut nms_lu: ::core::ffi::c_double = 0.;
    let mut nms_ldl: ::core::ffi::c_double = 0.;
    let mut dmax: ::core::ffi::c_double = 0.;
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut lnz: ::core::ffi::c_double = 0.;
    let mut lnzme: ::core::ffi::c_double = 0.;
    let mut p: i64 = 0;
    let mut p1: i64 = 0;
    let mut p2: i64 = 0;
    let mut p3: i64 = 0;
    let mut p4: i64 = 0;
    let mut pdst: i64 = 0;
    let mut pend: i64 = 0;
    let mut pj: i64 = 0;
    let mut pme: i64 = 0;
    let mut pme1: i64 = 0;
    let mut pme2: i64 = 0;
    let mut pn: i64 = 0;
    let mut psrc: i64 = 0;
    lnz = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    ndiv = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    nms_lu = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    nms_ldl = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    dmax = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    me = EMPTY as i64;
    mindeg = 0 as i64;
    ncmpa = 0 as i64;
    nel = 0 as i64;
    lemax = 0 as i64;
    if !Control.is_null() {
        alpha = *Control.offset(AMD_DENSE as isize);
        aggressive = (*Control.offset(AMD_AGGRESSIVE as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int as i64;
    } else {
        alpha = AMD_DEFAULT_DENSE;
        aggressive = AMD_DEFAULT_AGGRESSIVE as i64;
    }
    if alpha < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        dense = n - 2 as i64;
    } else {
        dense = (alpha * sqrt(n as ::core::ffi::c_double)) as i64;
    }
    dense = if 16 as i64 > dense {
        16 as i64
    } else {
        dense
    };
    dense = if n < dense { n } else { dense };
    i = 0 as i64;
    while i < n {
        *Last.offset(i as isize) = EMPTY as i64;
        *Head.offset(i as isize) = EMPTY as i64;
        *Next.offset(i as isize) = EMPTY as i64;
        *Nv.offset(i as isize) = 1 as i64;
        *W.offset(i as isize) = 1 as i64;
        *Elen.offset(i as isize) = 0 as i64;
        *Degree.offset(i as isize) = *Len.offset(i as isize);
        i += 1;
    }
    wbig = Int_MAX - n;
    wflg = clear_flag(0 as i64, wbig, W, n);
    ndense = 0 as i64;
    i = 0 as i64;
    while i < n {
        deg = *Degree.offset(i as isize);
        if deg == 0 as i64 {
            *Elen.offset(i as isize) =
                (-(1 as ::core::ffi::c_int) - 2 as ::core::ffi::c_int) as i64;
            nel += 1;
            *Pe.offset(i as isize) = EMPTY as i64;
            *W.offset(i as isize) = 0 as i64;
        } else if deg > dense {
            ndense += 1;
            *Nv.offset(i as isize) = 0 as i64;
            *Elen.offset(i as isize) = EMPTY as i64;
            nel += 1;
            *Pe.offset(i as isize) = EMPTY as i64;
        } else {
            inext = *Head.offset(deg as isize);
            if inext != EMPTY as i64 {
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
            if me != EMPTY as i64 {
                break;
            }
            deg += 1;
        }
        mindeg = deg;
        inext = *Next.offset(me as isize);
        if inext != EMPTY as i64 {
            *Last.offset(inext as isize) = EMPTY as i64;
        }
        *Head.offset(deg as isize) = inext;
        elenme = *Elen.offset(me as isize);
        nvpiv = *Nv.offset(me as isize);
        nel += nvpiv;
        *Nv.offset(me as isize) = -nvpiv;
        degme = 0 as i64;
        if elenme == 0 as i64 {
            pme1 = *Pe.offset(me as isize);
            pme2 = pme1 - 1 as i64;
            p = pme1;
            while p <= pme1 + *Len.offset(me as isize) - 1 as i64 {
                i = *Iw.offset(p as isize);
                nvi = *Nv.offset(i as isize);
                if nvi > 0 as i64 {
                    degme += nvi;
                    *Nv.offset(i as isize) = -nvi;
                    pme2 += 1;
                    *Iw.offset(pme2 as isize) = i;
                    ilast = *Last.offset(i as isize);
                    inext = *Next.offset(i as isize);
                    if inext != EMPTY as i64 {
                        *Last.offset(inext as isize) = ilast;
                    }
                    if ilast != EMPTY as i64 {
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
            knt1 = 1 as i64;
            while knt1 <= elenme + 1 as i64 {
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
                knt2 = 1 as i64;
                while knt2 <= ln {
                    let fresh1 = pj;
                    pj = pj + 1;
                    i = *Iw.offset(fresh1 as isize);
                    nvi = *Nv.offset(i as isize);
                    if nvi > 0 as i64 {
                        if pfree >= iwlen {
                            *Pe.offset(me as isize) = p;
                            *Len.offset(me as isize) -= knt1;
                            if *Len.offset(me as isize) == 0 as i64 {
                                *Pe.offset(me as isize) = EMPTY as i64;
                            }
                            *Pe.offset(e as isize) = pj;
                            *Len.offset(e as isize) = ln - knt2;
                            if *Len.offset(e as isize) == 0 as i64 {
                                *Pe.offset(e as isize) = EMPTY as i64;
                            }
                            ncmpa += 1;
                            j = 0 as i64;
                            while j < n {
                                pn = *Pe.offset(j as isize);
                                if pn >= 0 as i64 {
                                    *Pe.offset(j as isize) = *Iw.offset(pn as isize);
                                    *Iw.offset(pn as isize) = -j - 2 as i64;
                                }
                                j += 1;
                            }
                            psrc = 0 as i64;
                            pdst = 0 as i64;
                            pend = pme1 - 1 as i64;
                            while psrc <= pend {
                                let fresh2 = psrc;
                                psrc = psrc + 1;
                                j = -*Iw.offset(fresh2 as isize) - 2 as i64;
                                if j >= 0 as i64 {
                                    *Iw.offset(pdst as isize) = *Pe.offset(j as isize);
                                    let fresh3 = pdst;
                                    pdst = pdst + 1;
                                    *Pe.offset(j as isize) = fresh3;
                                    lenj = *Len.offset(j as isize);
                                    knt3 = 0 as i64;
                                    while knt3 <= lenj - 2 as i64 {
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
                            while psrc <= pfree - 1 as i64 {
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
                        if inext != EMPTY as i64 {
                            *Last.offset(inext as isize) = ilast;
                        }
                        if ilast != EMPTY as i64 {
                            *Next.offset(ilast as isize) = inext;
                        } else {
                            *Head.offset(*Degree.offset(i as isize) as isize) = inext;
                        }
                    }
                    knt2 += 1;
                }
                if e != me {
                    *Pe.offset(e as isize) = -me - 2 as i64;
                    *W.offset(e as isize) = 0 as i64;
                }
                knt1 += 1;
            }
            pme2 = pfree - 1 as i64;
        }
        *Degree.offset(me as isize) = degme;
        *Pe.offset(me as isize) = pme1;
        *Len.offset(me as isize) = pme2 - pme1 + 1 as i64;
        *Elen.offset(me as isize) = -(nvpiv + degme) - 2 as i64;
        wflg = clear_flag(wflg, wbig, W, n);
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            eln = *Elen.offset(i as isize);
            if eln > 0 as i64 {
                nvi = -*Nv.offset(i as isize);
                wnvi = wflg - nvi;
                p = *Pe.offset(i as isize);
                while p <= *Pe.offset(i as isize) + eln - 1 as i64 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we >= wflg {
                        we -= nvi;
                    } else if we != 0 as i64 {
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
            p2 = p1 + *Elen.offset(i as isize) - 1 as i64;
            pn = p1;
            hash = 0 as u64;
            deg = 0 as i64;
            if aggressive != 0 {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as i64 {
                        dext = we - wflg;
                        if dext > 0 as i64 {
                            deg += dext;
                            let fresh8 = pn;
                            pn = pn + 1;
                            *Iw.offset(fresh8 as isize) = e;
                            hash = hash.wrapping_add(e as u64);
                        } else {
                            *Pe.offset(e as isize) = -me - 2 as i64;
                            *W.offset(e as isize) = 0 as i64;
                        }
                    }
                    p += 1;
                }
            } else {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as i64 {
                        dext = we - wflg;
                        deg += dext;
                        let fresh9 = pn;
                        pn = pn + 1;
                        *Iw.offset(fresh9 as isize) = e;
                        hash = hash.wrapping_add(e as u64);
                    }
                    p += 1;
                }
            }
            *Elen.offset(i as isize) = pn - p1 + 1 as i64;
            p3 = pn;
            p4 = p1 + *Len.offset(i as isize);
            p = p2 + 1 as i64;
            while p < p4 {
                j = *Iw.offset(p as isize);
                nvj = *Nv.offset(j as isize);
                if nvj > 0 as i64 {
                    deg += nvj;
                    let fresh10 = pn;
                    pn = pn + 1;
                    *Iw.offset(fresh10 as isize) = j;
                    hash = hash.wrapping_add(j as u64);
                }
                p += 1;
            }
            if *Elen.offset(i as isize) == 1 as i64 && p3 == pn {
                *Pe.offset(i as isize) = -me - 2 as i64;
                nvi = -*Nv.offset(i as isize);
                degme -= nvi;
                nvpiv += nvi;
                nel += nvi;
                *Nv.offset(i as isize) = 0 as i64;
                *Elen.offset(i as isize) = EMPTY as i64;
            } else {
                *Degree.offset(i as isize) = if *Degree.offset(i as isize) < deg {
                    *Degree.offset(i as isize)
                } else {
                    deg
                };
                *Iw.offset(pn as isize) = *Iw.offset(p3 as isize);
                *Iw.offset(p3 as isize) = *Iw.offset(p1 as isize);
                *Iw.offset(p1 as isize) = me;
                *Len.offset(i as isize) = pn - p1 + 1 as i64;
                hash = hash.wrapping_rem(n as u64);
                j = *Head.offset(hash as isize);
                if j <= EMPTY as i64 {
                    *Next.offset(i as isize) = -j - 2 as i64;
                    *Head.offset(hash as isize) = -i - 2 as i64;
                } else {
                    *Next.offset(i as isize) = *Last.offset(j as isize);
                    *Last.offset(j as isize) = i;
                }
                *Last.offset(i as isize) = hash as i64;
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
            if *Nv.offset(i as isize) < 0 as i64 {
                hash = *Last.offset(i as isize) as u64;
                j = *Head.offset(hash as isize);
                if j == EMPTY as i64 {
                    i = EMPTY as i64;
                } else if j < EMPTY as i64 {
                    i = -j - 2 as i64;
                    *Head.offset(hash as isize) = EMPTY as i64;
                } else {
                    i = *Last.offset(j as isize);
                    *Last.offset(j as isize) = EMPTY as i64;
                }
                while i != EMPTY as i64
                    && *Next.offset(i as isize) != EMPTY as i64
                {
                    ln = *Len.offset(i as isize);
                    eln = *Elen.offset(i as isize);
                    p = *Pe.offset(i as isize) + 1 as i64;
                    while p <= *Pe.offset(i as isize) + ln - 1 as i64 {
                        *W.offset(*Iw.offset(p as isize) as isize) = wflg;
                        p += 1;
                    }
                    jlast = i;
                    j = *Next.offset(i as isize);
                    while j != EMPTY as i64 {
                        ok = (*Len.offset(j as isize) == ln && *Elen.offset(j as isize) == eln)
                            as ::core::ffi::c_int
                            as i64;
                        p = *Pe.offset(j as isize) + 1 as i64;
                        while ok != 0 && p <= *Pe.offset(j as isize) + ln - 1 as i64
                        {
                            if *W.offset(*Iw.offset(p as isize) as isize) != wflg {
                                ok = 0 as i64;
                            }
                            p += 1;
                        }
                        if ok != 0 {
                            *Pe.offset(j as isize) = -i - 2 as i64;
                            *Nv.offset(i as isize) += *Nv.offset(j as isize);
                            *Nv.offset(j as isize) = 0 as i64;
                            *Elen.offset(j as isize) = EMPTY as i64;
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
            if nvi > 0 as i64 {
                *Nv.offset(i as isize) = nvi;
                deg = *Degree.offset(i as isize) + degme - nvi;
                deg = if deg < nleft - nvi { deg } else { nleft - nvi };
                inext = *Head.offset(deg as isize);
                if inext != EMPTY as i64 {
                    *Last.offset(inext as isize) = i;
                }
                *Next.offset(i as isize) = inext;
                *Last.offset(i as isize) = EMPTY as i64;
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
        if *Len.offset(me as isize) == 0 as i64 {
            *Pe.offset(me as isize) = EMPTY as i64;
            *W.offset(me as isize) = 0 as i64;
        }
        if elenme != 0 as i64 {
            pfree = p;
        }
        if !Info.is_null() {
            f = nvpiv as ::core::ffi::c_double;
            r = (degme + ndense) as ::core::ffi::c_double;
            dmax = if dmax > f + r { dmax } else { f + r };
            lnzme = f * r
                + (f - 1 as ::core::ffi::c_int as ::core::ffi::c_double) * f
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            lnz += lnzme;
            ndiv += lnzme;
            s = f * r * r
                + r * (f - 1 as ::core::ffi::c_int as ::core::ffi::c_double) * f
                + (f - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    * f
                    * (2 as ::core::ffi::c_int as ::core::ffi::c_double * f
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    / 6 as ::core::ffi::c_int as ::core::ffi::c_double;
            nms_lu += s;
            nms_ldl += (s + lnzme) / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    if !Info.is_null() {
        f = ndense as ::core::ffi::c_double;
        dmax = if dmax > ndense as ::core::ffi::c_double {
            dmax
        } else {
            ndense as ::core::ffi::c_double
        };
        lnzme = (f - 1 as ::core::ffi::c_int as ::core::ffi::c_double) * f
            / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        lnz += lnzme;
        ndiv += lnzme;
        s = (f - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            * f
            * (2 as ::core::ffi::c_int as ::core::ffi::c_double * f
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            / 6 as ::core::ffi::c_int as ::core::ffi::c_double;
        nms_lu += s;
        nms_ldl += (s + lnzme) / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *Info.offset(AMD_LNZ as isize) = lnz;
        *Info.offset(AMD_NDIV as isize) = ndiv;
        *Info.offset(AMD_NMULTSUBS_LDL as isize) = nms_ldl;
        *Info.offset(AMD_NMULTSUBS_LU as isize) = nms_lu;
        *Info.offset(AMD_NDENSE as isize) = ndense as ::core::ffi::c_double;
        *Info.offset(AMD_DMAX as isize) = dmax;
        *Info.offset(AMD_NCMPA as isize) = ncmpa as ::core::ffi::c_double;
        *Info.offset(AMD_STATUS as isize) = AMD_OK as ::core::ffi::c_double;
    }
    i = 0 as i64;
    while i < n {
        *Pe.offset(i as isize) = -*Pe.offset(i as isize) - 2 as i64;
        i += 1;
    }
    i = 0 as i64;
    while i < n {
        *Elen.offset(i as isize) = -*Elen.offset(i as isize) - 2 as i64;
        i += 1;
    }
    i = 0 as i64;
    while i < n {
        if *Nv.offset(i as isize) == 0 as i64 {
            j = *Pe.offset(i as isize);
            if !(j == EMPTY as i64) {
                while *Nv.offset(j as isize) == 0 as i64 {
                    j = *Pe.offset(j as isize);
                }
                e = j;
                j = i;
                while *Nv.offset(j as isize) == 0 as i64 {
                    jnext = *Pe.offset(j as isize);
                    *Pe.offset(j as isize) = e;
                    j = jnext;
                }
            }
        }
        i += 1;
    }
    amd_l_postorder(n, Pe, Nv, Elen, W, Head, Next, Last);
    k = 0 as i64;
    while k < n {
        *Head.offset(k as isize) = EMPTY as i64;
        *Next.offset(k as isize) = EMPTY as i64;
        k += 1;
    }
    e = 0 as i64;
    while e < n {
        k = *W.offset(e as isize);
        if k != EMPTY as i64 {
            *Head.offset(k as isize) = e;
        }
        e += 1;
    }
    nel = 0 as i64;
    k = 0 as i64;
    while k < n {
        e = *Head.offset(k as isize);
        if e == EMPTY as i64 {
            break;
        }
        *Next.offset(e as isize) = nel;
        nel += *Nv.offset(e as isize);
        k += 1;
    }
    i = 0 as i64;
    while i < n {
        if *Nv.offset(i as isize) == 0 as i64 {
            e = *Pe.offset(i as isize);
            if e != EMPTY as i64 {
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
    i = 0 as i64;
    while i < n {
        k = *Next.offset(i as isize);
        *Last.offset(k as isize) = i;
        i += 1;
    }
}
pub const LONG_MAX: i64 = 0x7fffffffffffffff as i64;
