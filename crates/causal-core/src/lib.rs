//! Hirmos-owned causal kernels copied module-for-module from the completed parity transpile.
//!
//! Only modules adopted by the application live here. The browser-facing contract remains in the
//! separate `hirmos-wasm` façade so scientific implementations do not acquire UI serialization.

pub mod grf;
mod mt19937_64;
mod arm_log;
mod arm_log_data;
mod arm_pow;
mod arm_pow_data;
pub mod backdoor;
pub mod bootstrap;
pub mod causal_effects;
pub mod causal_ts_preparation;
pub mod cdnots;
pub mod ci_samples;
pub mod constraint_discovery;
pub mod data_preparation;
pub mod crossfit;
pub mod design;
pub mod gps;
pub mod gradient_boosting;
pub mod discrete_bn;
pub mod dowhy_bootstrap;
pub mod estimation;
pub mod frontdoor;
pub mod iv;
pub mod joint_samples;
pub mod jpcmciplus;
mod lapack_cholesky;
mod lapack_dgeev;
mod lapack_dgelsd;
pub mod honest_did;
mod lapack_dgesdd;
mod lapack_dsyevd;
mod lapack_lu;
pub mod r_nelder_mead;
mod slsqp;
mod nelder_mead;
mod least_squares;
pub mod conditional_gaussian;
mod linalg;
pub mod mackinnon;
pub mod missing_data;
pub mod neural_granger;
pub mod normality;
pub mod ols;
pub mod panel;
pub mod parcorr;
pub mod pc_stable;
pub mod pcmci;
pub mod pcmciplus;
pub mod redundancy;
pub mod stationarity;
pub mod survival;
pub mod tsdiag;
pub mod zivot_andrews;

pub use backdoor::{
    backdoor_adjustment, backdoor_linear_ate, backdoor_variables, refute_data_subset,
    refute_placebo, refute_random_common_cause, Dag,
};
pub use dowhy_bootstrap::{BootstrapError, DowhyBootstrap};
pub use estimation::{durbin_watson, ols_cluster, ols_hac, ols_hc1, ols_two_way, ols_within, wls, HacOls, SandwichOls, WithinErrors, WithinOls, WlsFit};
pub use frontdoor::{
    frontdoor_two_stage, frontdoor_two_stage_with_progress, identify_frontdoor_set,
    FrontdoorBootstrap, FrontdoorError, FrontdoorIdentificationError, FrontdoorInput,
    FrontdoorOptions, FrontdoorResult,
};
pub use iv::{
    identify_instrument_set, instrumental_variable, instrumental_variable_with_progress, IvError,
    IvEstimator, IvIdentificationError, IvInput, IvOptions, IvResult,
};
pub use numpy_reduce::{numpy_mean, numpy_sum};

pub use ci_samples::{
    construct_array_tracked, ConstructOptions, ConstructedArray, SampleExclusion,
    SampleExclusionReason,
};
pub use mackinnon::Regression;
pub use missing_data::{
    forward_fill, linear_interpolate, longest_complete_interval, structural_zero, ImputationResult,
    MaskType, PreprocessingError, TigramiteFrame,
};
pub use parcorr::{
    run_test as parcorr_test, CiKind, CiSampleAudit, ParCorrCi, RoleAwareSamplePolicy, TimeSeries,
};
pub use pcmci::{
    run_bivci, run_pcmci, run_pcmci_filtered, run_pcmci_frame, run_pcmci_selected, run_pcmci_with,
    PcmciResult,
};
pub use pcmciplus::{
    fdr_bh, run_pcalg_standard, run_pcmciplus, run_pcmciplus_frame, run_sliding_window_pcmciplus,
    PcmciPlusResult,
};
pub use stationarity::{adfuller, kpss, AdfResult, KpssResult};
pub use zivot_andrews::{zivot_andrews, ZaModel, ZaResult};
pub mod dynotears;
pub mod expm;
pub mod fci;
pub mod lars;
pub mod lbfgsb;
pub mod lpcmci;
pub mod nprandom;
pub mod numpy_argsort;
pub mod numpy_reduce;
pub mod ocse;
pub mod parcorr_mult;
mod r_zeroin;
pub mod var_lingam;
pub mod ziggurat;

pub use var_lingam::{
    direct_lingam, direct_lingam_with_progress, run_var_lingam, var_lingam_bootstrap,
    VarBootstrapResult, VarLingamResult,
};

pub mod ardl;
pub mod arma_regression;
pub mod bayesian_gaussian;
pub mod bayesian_impact;
pub mod did;
pub mod did_sensitivity;
pub mod staggered_did;
pub mod did_regression;
pub mod rd;
pub mod bfgs;
pub mod causal_impact;
pub mod coint;
pub mod counterfactual;
pub mod counterfactual_evaluator;
pub mod counterfactual_graph;
pub mod counterfactual_query;
pub mod dml;
pub mod dml_groups;
pub mod do_calculus;
pub mod dynamic_counterfactual;
pub mod fminbound;
pub mod glm;
pub mod grace;
pub mod graph_falsification;
pub mod gcm;
pub mod halton;
pub mod id_star;
pub mod idc_star;
pub mod identified_expression;
pub mod ingarch;
pub mod interrupted_series;
pub mod kci;
pub mod linear_mediation;
pub mod logistic;
pub mod logit;
pub mod metrics;
pub mod model_selection;
pub mod propensity;
pub mod negbin_nuts;
pub mod nuts;
pub mod pelt;
pub mod pss_tables;
pub mod refute_dml;
pub mod resampling;
pub mod calendar;
pub mod rpcmci;
pub mod simplex;
pub mod sklearn_linear;
pub mod sktree;
pub mod stl;
pub mod synthetic_control;
pub mod tlearner;
pub mod ucm;
pub mod unobserved;
pub mod vecm;

pub use coint::{coint, coint_johansen, CointResult, JohansenResult};
pub use dml::{dml_irm, dml_plr, DmlResult, SensitivityResult, SensitivityScenario};
pub use dml_groups::{group_effects, GroupEffect, GroupEffectError, GroupEffects};
pub use do_calculus::{
    identify_conditional_outcomes, identify_outcomes, latent_projection, Admg,
    Expression as IdentifiedExpression, GraphError as AdmgError, Hedge, IdentificationError,
};
pub use graph_falsification::{
    check_dag, falsify_graph, falsify_graph_with_progress, holm_adjust, test_implications,
    uniformity_test, DagCheckResult, DagImplication, DagImplicationTest, GraphCheckError,
    GraphFalsificationResult, ImplicationDecision, UniformityTest,
};
pub use kci::{
    causal_learn_kernel_conditional_independence, kernel_conditional_independence, KciError,
    KciResult,
};
pub use normality::shapiro;
pub use redundancy::{cluster_redundant, correlation_matrix, vif_redundant};
pub use refute_dml::{
    placebo_refute, random_common_cause_refute, unobserved_refute, worker_fit, RefutationOutcome,
    WorkerStudy,
};
pub use resampling::{
    pandas_resample_daily, Aggregation as ResamplingAggregation, IncompleteBins, ResampleError,
    ResampleFrequency, ResampleResult,
};
pub use rpcmci::{run_rpcmci, run_rpcmci_frame, run_rpcmci_with_progress, RpcmciResult};
pub use tlearner::{fit_tlearner, TLearnerError, TLearnerFit};
pub use unobserved::{infer_kappa_t, infer_kappa_y, unobserved_common_cause_grid};
pub use vecm::{chow_break, select_coint_rank, vecm_fit, vecm_select_order, VecmResult};
mod adjustment_sets;
pub use adjustment_sets::{dagitty_adjustment_sets, AdjustmentSetAnalysis, AdjustmentSetError};

pub mod panel_glm;

pub mod panel_design;

pub mod continuous_did;
pub mod sun_abraham;
pub mod sun_abraham_fit;
pub mod augmented_synth;
pub mod surrogate_index;
pub mod swig;
