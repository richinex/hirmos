//! Arm math/log.c, N=128, polynomial orders 6/12, HAVE_FAST_FMA=1.
//! Copyright (c) 2018-2025, Arm Limited.
//! SPDX-License-Identifier: MIT OR Apache-2.0 WITH LLVM-exception
//! Positive finite path only. Explicit FMA, other operations uncontracted.
//! Used at the validated GRF tuning boundary, not as a global math replacement.
use crate::arm_log_data::{NEAR as B, POLY as A, TABLE};
use crate::arm_pow_data::{LN2_HI, LN2_LO};
pub(crate) fn log_positive(x: f64) -> f64 {
    debug_assert!(x.is_finite() && x > 0.);
    let mut ix = x.to_bits();
    let low = (1. - 1. / 16_f64).to_bits();
    let high = (1.0_f64 + (1. + 9. / 256.) / 16.).to_bits();
    if ix.wrapping_sub(low) < high - low {
        if x == 1. {
            return 0.;
        }
        let r = x - 1.;
        let r2 = r * r;
        let r3 = r * r2;
        let mut y = r3
            * (B[1]
                + r * B[2]
                + r2 * B[3]
                + r3 * (B[4]
                    + r * B[5]
                    + r2 * B[6]
                    + r3 * (B[7] + r * B[8] + r2 * B[9] + r3 * B[10])));
        let w = r * 134217728.;
        let rhi = r + w - w;
        let rlo = r - rhi;
        let w = rhi * rhi * B[0];
        let hi = r + w;
        let mut lo = r - hi + w;
        lo += B[0] * rlo * (rhi + r);
        y += lo;
        y += hi;
        return y;
    }
    if x < f64::MIN_POSITIVE {
        ix = (x * 4503599627370496.).to_bits().wrapping_sub(52_u64 << 52);
    }
    let tmp = ix.wrapping_sub(0x3fe6000000000000);
    let i = ((tmp >> (52 - 7)) % 128) as usize;
    let k = ((tmp as i64) >> 52) as f64;
    let z = f64::from_bits(ix.wrapping_sub(tmp & (0xfff_u64 << 52)));
    let [invc, logc] = TABLE[i];
    let r = z.mul_add(invc, -1.);
    let w = k * LN2_HI + logc;
    let hi = w + r;
    let lo = w - hi + r + k * LN2_LO;
    let r2 = r * r;
    lo + r2 * A[0] + r * r2 * (A[1] + r * A[2] + r2 * (A[3] + r * A[4])) + hi
}
