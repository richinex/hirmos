//! Hirmos-owned causal kernels copied module-for-module from the completed parity transpile.
//!
//! Only modules adopted by the application live here. The browser-facing contract remains in the
//! separate `hirmos-wasm` façade so scientific implementations do not acquire UI serialization.

pub mod mackinnon;
pub mod backdoor;
pub mod estimation;
pub mod causal_effects;
pub mod discrete_bn;
pub mod ols;
pub mod panel;
pub mod parcorr;
pub mod pcmci;
pub mod pcmciplus;
pub mod preprocessing;
pub mod data_preparation;
pub mod stationarity;
pub mod tsdiag;
pub mod zivot_andrews;

pub use backdoor::{
    backdoor_adjustment, backdoor_linear_ate, backdoor_variables, refute_data_subset, refute_placebo,
    refute_random_common_cause, Dag,
};
pub use estimation::{durbin_watson, ols_hac, wls, HacOls, WlsFit};

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
pub mod dynotears;
pub mod expm;
pub mod lars;
pub mod lbfgsb;
pub mod lpcmci;
pub mod nprandom;
pub mod ocse;
pub mod var_lingam;
pub mod ziggurat;

pub use var_lingam::{direct_lingam, run_var_lingam, var_lingam_bootstrap, VarBootstrapResult, VarLingamResult};

pub mod glm;
pub mod graph_falsification;
pub mod kci;
pub mod bfgs;
pub mod causal_impact;
pub mod ucm;
pub mod dml;
pub mod sktree;
pub mod fminbound;
pub mod logistic;
pub mod refute_dml;
pub mod unobserved;
pub mod counterfactual;
pub mod synthetic_control;
pub mod ardl;
pub mod pss_tables;
pub mod vecm;
pub mod coint;
pub mod nuts;
pub mod negbin_nuts;
pub mod bayesian_gaussian;
pub mod stl;
pub mod pelt;
pub mod preprocess;

pub use coint::{coint, coint_johansen, CointResult, JohansenResult};
pub use dml::{dml_irm, dml_plr, DmlResult, SensitivityResult, SensitivityScenario};
pub use preprocess::{cluster_redundant, shapiro, vif_redundant};
pub use graph_falsification::{
    check_dag, falsify_graph, falsify_graph_with_progress, holm_adjust, test_implications,
    uniformity_test, DagCheckResult,
    DagImplication, DagImplicationTest, GraphCheckError, GraphFalsificationResult,
    ImplicationDecision, UniformityTest,
};
pub use kci::{kernel_conditional_independence, KciError, KciResult};
pub use refute_dml::{
    placebo_refute, random_common_cause_refute, unobserved_refute, worker_fit, RefutationOutcome,
    WorkerStudy,
};
pub use unobserved::{infer_kappa_t, infer_kappa_y, unobserved_common_cause_grid};
pub use vecm::{chow_break, select_coint_rank, vecm_fit, vecm_select_order, VecmResult};
mod adjustment_sets;
pub use adjustment_sets::{
    dagitty_adjustment_sets, AdjustmentSetAnalysis, AdjustmentSetError,
};
