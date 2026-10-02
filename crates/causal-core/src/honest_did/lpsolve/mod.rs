//! Reference simplex implementation. Matrix construction is the only public adapter.
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_mut,
    unused_assignments,
    unused_parens,
    static_mut_refs,
    unused_variables,
    unused_attributes,
    double_negations,
    improper_ctypes_definitions,
    clashing_extern_declarations,
    unused_must_use
)]
mod blas_bridge;
// Matrix inference must not call native file, text-parser or plugin operations.
// Preserve those calls on native targets and fail explicitly on WASM.
macro_rules! native_only {
    ($name:ident, $($argument:expr),* $(,)?) => {{
        #[cfg(not(target_arch="wasm32"))]
        { $name($($argument),*) }
        #[cfg(target_arch="wasm32")]
        { crate::honest_did::lpsolve::runtime::unavailable::<native_result_type!($name)>(stringify!($name)) }
    }};
}
macro_rules! native_result_type {
    (fopen) => {*mut FILE};
    (fgets) => {*mut core::ffi::c_char};
    (dlopen) => {*mut core::ffi::c_void};
    (dlsym) => {*mut core::ffi::c_void};
    (__error) => {*mut core::ffi::c_int};
    (strtod) => {core::ffi::c_double};
    (strtol) => {core::ffi::c_long};
    (ftell) => {core::ffi::c_long};
    (fwrite) => {usize};
    ($name:ident) => {core::ffi::c_int};
}
mod extended;
pub(crate) mod runtime;
thread_local! {
    static RNG: std::cell::RefCell<crate::honest_did::r_rng::RRng> = std::cell::RefCell::new(crate::honest_did::r_rng::RRng::new(0));
    static RANDOM_CALLS: std::cell::Cell<usize> = const {std::cell::Cell::new(0)};
}
// Upstream LUSOL contains static scratch storage. Serialize the bounded native
// solver entry point; separate browser workers own separate WASM instances.
static SOLVER: std::sync::Mutex<()> = std::sync::Mutex::new(());
unsafe fn GetRNGstate() {}
unsafe fn PutRNGstate() {}
unsafe fn unif_rand() -> f64 {
    RANDOM_CALLS.with(|c| c.set(c.get() + 1));
    RNG.with(|r| r.borrow_mut().uniform())
}
include!("generated.rs");

pub(crate) fn minimize(
    cost: &[f64],
    a: &nalgebra::DMatrix<f64>,
    b: &[f64],
) -> Result<(f64, Vec<f64>, Vec<f64>), crate::honest_did::Error> {
    use lp_lib::*;
    if a.ncols() != cost.len()
        || a.nrows() != b.len()
        || cost.is_empty()
        || b.is_empty()
        || cost.len() + b.len() > i32::MAX as usize
        || a.iter()
            .chain(cost.iter())
            .chain(b.iter())
            .any(|v| !v.is_finite())
    {
        return Err(crate::honest_did::Error::InvalidInput(
            "Invalid simplex matrix dimensions or values.",
        ));
    }
    let _guard = SOLVER.lock().map_err(|_| {
        crate::honest_did::Error::InvalidInput("The simplex workspace is unavailable after a failed call.")
    })?;
    RANDOM_CALLS.with(|c| c.set(0));
    unsafe {
        let lp = make_lp(0, cost.len() as i32);
        if lp.is_null() {
            return Err(crate::honest_did::Error::InvalidInput(
                "The simplex solver could not allocate a model.",
            ));
        }
        struct Model(*mut lprec);
        impl Drop for Model {
            fn drop(&mut self) {
                unsafe { delete_lp(self.0) }
            }
        }
        let _model = Model(lp);
        let mut row = vec![0.0; cost.len() + 1];
        row[1..].copy_from_slice(cost);
        if set_obj_fn(lp, row.as_mut_ptr()) == 0 {
            return Err(crate::honest_did::Error::InvalidInput(
                "The simplex objective was refused.",
            ));
        }
        for i in 0..a.nrows() {
            for j in 0..a.ncols() {
                row[j + 1] = a[(i, j)];
            }
            if add_constraint(lp, row.as_mut_ptr(), 1, b[i]) == 0 {
                return Err(crate::honest_did::Error::InvalidInput(
                    "A simplex constraint was refused.",
                ));
            }
        }
        for j in 1..=cost.len() {
            if set_lowbo(lp, j as i32, -1e30) == 0 {
                return Err(crate::honest_did::Error::InvalidInput("A simplex bound was refused."));
            }
        }
        set_minim(lp);
        set_simplextype(lp, 10);
        set_pivoting(lp, 1);
        set_verbose(lp, 0);
        let status = solve(lp);
        if RANDOM_CALLS.with(|c| c.get()) != 0 {
            return Err(crate::honest_did::Error::Optimization(crate::honest_did::optimization::Failure::DidNotConverge("Randomized simplex perturbation needs a shared reference RNG state; no interval is returned.".into())));
        }
        if status != 0 {
            return Err(crate::honest_did::Error::Optimization(match status {
                2 => crate::honest_did::optimization::Failure::Infeasible,
                3 => crate::honest_did::optimization::Failure::Unbounded,
                _ => crate::honest_did::optimization::Failure::DidNotConverge(format!(
                    "Reference simplex status {status}"
                )),
            }));
        }
        let mut variables = vec![0.0; cost.len()];
        let mut duals = vec![0.0; cost.len() + b.len() + 1];
        if get_variables(lp, variables.as_mut_ptr()) == 0
            || get_dual_solution(lp, duals.as_mut_ptr()) == 0
        {
            return Err(crate::honest_did::Error::InvalidInput(
                "The reference simplex solution is unavailable.",
            ));
        }
        Ok((
            get_objective(lp),
            variables,
            duals[1..=b.len()].iter().map(|v| -v).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;
    #[test]
    fn statuses_do_not_become_successful_solutions() {
        assert!(matches!(
            minimize(
                &[1.0],
                &DMatrix::from_row_slice(2, 1, &[1.0, -1.0]),
                &[0.0, -1.0]
            ),
            Err(crate::honest_did::Error::Optimization(
                crate::honest_did::optimization::Failure::Infeasible
            ))
        ));
        assert!(matches!(
            minimize(&[-1.0], &DMatrix::from_row_slice(1, 1, &[-1.0]), &[0.0]),
            Err(crate::honest_did::Error::Optimization(
                crate::honest_did::optimization::Failure::Unbounded
            ))
        ));
        assert!(minimize(&[f64::NAN], &DMatrix::identity(1, 1), &[0.0]).is_err());
        assert!(minimize(&[], &DMatrix::zeros(0, 0), &[]).is_err());
    }
    #[test]
    fn separate_native_calls_are_serialized() {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                std::thread::spawn(|| {
                    for _ in 0..20 {
                        let out =
                            minimize(&[1.0], &DMatrix::from_row_slice(1, 1, &[-1.0]), &[-2.0])
                                .unwrap();
                        assert!((out.0 - 2.0).abs() < 1e-12);
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
