//! Hirmos-owned causal kernels copied module-for-module from the completed parity transpile.
//!
//! Only modules adopted by the application live here. The browser-facing contract remains in the
//! separate `hirmos-wasm` façade so scientific implementations do not acquire UI serialization.

pub mod mackinnon;
pub mod ols;
pub mod parcorr;
pub mod pcmci;
pub mod pcmciplus;
pub mod preprocessing;
pub mod stationarity;
pub mod tsdiag;
pub mod zivot_andrews;

pub use mackinnon::Regression;
pub use parcorr::{run_test as parcorr_test, CiKind, ParCorrCi, TimeSeries};
pub use pcmci::{
    run_bivci, run_pcmci, run_pcmci_filtered, run_pcmci_selected, run_pcmci_with, PcmciResult,
};
pub use pcmciplus::{
    fdr_bh, run_pcalg_standard, run_pcmciplus, run_sliding_window_pcmciplus, PcmciPlusResult,
};
pub use preprocessing::{
    construct_array_tracked, forward_fill, linear_interpolate, longest_complete_interval,
    structural_zero, ConstructOptions, ConstructedArray, ImputationResult, MaskType,
    PreprocessingError, SampleExclusion, SampleExclusionReason, TigramiteFrame,
};
pub use stationarity::{adfuller, kpss, AdfResult, KpssResult};
pub use zivot_andrews::{zivot_andrews, ZaModel, ZaResult};
