//! Continuous-dose DiD. Imported from the pinned continuous-did oracle port.
//! Shared numerical dependencies are aliases, not duplicate implementations.
pub mod aggregation;
pub mod bootstrap;
pub mod cck;
pub mod cell;
pub mod defaults;
pub mod dose;
pub mod panel;
pub mod response_curve;
pub mod spline;
pub mod window_contrast;
use crate::lapack_dgesdd;
use crate::survival::aalen::qr as r_qr;
use crate::survival::r_rng;
mod quantile {
    pub(crate) use crate::survival::flexsurv::uncertainty::type_seven_quantile;
}
