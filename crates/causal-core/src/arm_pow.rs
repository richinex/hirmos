//! Arm `math/pow.c` normal-positive path for DataFrame's integer sample counts.
//! Copyright (c) 2018-2026, Arm Limited.
//! SPDX-License-Identifier: MIT OR Apache-2.0 WITH LLVM-exception
//!
//! This is not a general pow API: u64 input and exponent 1/3 cannot reach the
//! negative, subnormal, overflow or nonfinite branches. Explicit FMAs follow
//! HAVE_FAST_FMA=1; other expressions follow C with -ffp-contract=off.
use crate::arm_pow_data::*;

pub(crate) fn cube_power(n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let ix = (n as f64).to_bits();
    let tmp = ix.wrapping_sub(0x3fe6955500000000);
    let i = ((tmp >> (52 - 7)) % 128) as usize;
    let k = ((tmp as i64) >> 52) as f64;
    let iz = ix.wrapping_sub(tmp & (0xfff_u64 << 52));
    let z = f64::from_bits(iz);
    let [invc, logc, logctail] = LOG[i];
    let r = z.mul_add(invc, -1.0);
    let t1 = k * LN2_HI + logc;
    let t2 = t1 + r;
    let lo1 = k * LN2_LO + logctail;
    let lo2 = t1 - t2 + r;
    let a = LOG_POLY;
    let ar = a[0] * r;
    let ar2 = r * ar;
    let ar3 = r * ar2;
    let hi = t2 + ar2;
    let lo3 = ar.mul_add(r, -ar2);
    let lo4 = t2 - hi + ar2;
    let p = ar3 * (a[1] + r * a[2] + ar2 * (a[3] + r * a[4] + ar2 * (a[5] + r * a[6])));
    let lo = lo1 + lo2 + lo3 + lo4 + p;
    let log = hi + lo;
    let tail = hi - log + lo;

    let exponent = 1.0_f64 / 3.0;
    let ehi = exponent * log;
    let elo = exponent * tail + exponent.mul_add(log, -ehi);
    if ehi.abs() < f64::from_bits((1023 - 54) << 52) {
        return 1.0 + ehi;
    }
    let kd = (INV_LN2_N * ehi).round();
    let ki = kd as u64;
    let r = ehi + kd * NEG_LN2_HI + kd * NEG_LN2_LO + elo;
    let idx = 2 * (ki % 128) as usize;
    let tail = f64::from_bits(EXP[idx]);
    let sbits = EXP[idx + 1].wrapping_add(ki << (52 - 7));
    let r2 = r * r;
    let [c2, c3, c4, c5] = EXP_POLY;
    let tmp = tail + r + r2 * (c2 + r * c3) + r2 * r2 * (c4 + r * c5);
    let scale = f64::from_bits(sbits);
    scale + scale * tmp
}
