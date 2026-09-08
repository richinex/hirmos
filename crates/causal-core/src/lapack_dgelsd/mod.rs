//! Safe, source-traceable translation of LAPACK 3.12.1's `DGELSD` closure.
//!
//! The modules retain LAPACK's column-major storage and leading dimensions.
//! [`dgelsd`] is the driver entry point; [`dgelsd_workspace`] implements the
//! corresponding workspace query without mutating matrix inputs.

#![allow(dead_code, unused_imports, unused_parens)]

mod blas;
mod divide_conquer;
pub(crate) mod reduction;

mod driver;

pub use driver::{dgelsd, dgelsd_workspace, DgelsdError, DgelsdWorkspace};

#[cfg(test)]
pub(crate) use driver::{probe_path_1a, probe_path_1a_with_smlsiz};
