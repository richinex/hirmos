//! Source-level port of GRF 2.6.1 causal-forest components.
//!
//! Includes nuisance regression forests, causal training, prediction, average
//! effects and heterogeneity diagnostics. See oracle/grf/README.md for the
//! pinned oracle, verification commands and explicit scope exclusions.
pub mod causal;
pub mod forest;
pub mod inference;
pub mod prediction;
pub mod rank;
pub mod regression;
pub mod sampling;
pub mod splitting;
pub mod tree;
pub mod tuning;
pub mod kriging;
pub mod moderation;
