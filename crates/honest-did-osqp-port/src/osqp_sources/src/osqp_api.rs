#[repr(C)] pub struct OSQPVectorf_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPVectori_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPMatrix_ { _opaque: [u8; 0] }
use crate::runtime::{malloc,calloc,free};
extern "C" {
    #[link_name = "honest_osqp_OSQP_ERROR_MESSAGE"]
    static mut OSQP_ERROR_MESSAGE: [*const ::core::ffi::c_char; 0];
    #[link_name = "honest_osqp_OSQPVectorf_malloc"]
    fn OSQPVectorf_malloc(length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_calloc"]
    fn OSQPVectorf_calloc(length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectori_malloc"]
    fn OSQPVectori_malloc(length: OSQPInt) -> *mut OSQPVectori;
    #[link_name = "honest_osqp_OSQPVectori_calloc"]
    fn OSQPVectori_calloc(length: OSQPInt) -> *mut OSQPVectori;
    #[link_name = "honest_osqp_OSQPVectorf_new"]
    fn OSQPVectorf_new(a: *const OSQPFloat, length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_free"]
    fn OSQPVectorf_free(a: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectori_free"]
    fn OSQPVectori_free(a: *mut OSQPVectori);
    #[link_name = "honest_osqp_OSQPVectorf_view"]
    fn OSQPVectorf_view(a: *const OSQPVectorf, head: OSQPInt, length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_view_free"]
    fn OSQPVectorf_view_free(a: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_from_raw"]
    fn OSQPVectorf_from_raw(b: *mut OSQPVectorf, a: *const OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_set_scalar"]
    fn OSQPVectorf_set_scalar(a: *mut OSQPVectorf, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_set_scalar_conditional"]
    fn OSQPVectorf_set_scalar_conditional(
        a: *mut OSQPVectorf,
        test: *const OSQPVectori,
        val_if_neg: OSQPFloat,
        val_if_zero: OSQPFloat,
        val_if_pos: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPVectorf_mult_scalar"]
    fn OSQPVectorf_mult_scalar(a: *mut OSQPVectorf, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_ew_prod"]
    fn OSQPVectorf_ew_prod(c: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_all_leq"]
    fn OSQPVectorf_all_leq(l: *const OSQPVectorf, u: *const OSQPVectorf) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPVectorf_ew_reciprocal"]
    fn OSQPVectorf_ew_reciprocal(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPMatrix_new_from_csc"]
    fn OSQPMatrix_new_from_csc(A: *const OSQPCscMatrix, is_triu: OSQPInt) -> *mut OSQPMatrix;
    #[link_name = "honest_osqp_OSQPMatrix_update_values"]
    fn OSQPMatrix_update_values(
        M: *mut OSQPMatrix,
        Mx_new: *const OSQPFloat,
        Mx_new_idx: *const OSQPInt,
        M_new_n: OSQPInt,
    );
    #[link_name = "honest_osqp_OSQPMatrix_get_nz"]
    fn OSQPMatrix_get_nz(M: *const OSQPMatrix) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPMatrix_Axpy"]
    fn OSQPMatrix_Axpy(
        A: *const OSQPMatrix,
        x: *const OSQPVectorf,
        y: *mut OSQPVectorf,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPMatrix_free"]
    fn OSQPMatrix_free(M: *mut OSQPMatrix);
    #[link_name = "honest_osqp_compute_rho_estimate"]
    fn compute_rho_estimate(solver: *const OSQPSolver) -> OSQPFloat;
    #[link_name = "honest_osqp_adapt_rho"]
    fn adapt_rho(solver: *mut OSQPSolver) -> OSQPInt;
    #[link_name = "honest_osqp_set_rho_vec"]
    fn set_rho_vec(solver: *mut OSQPSolver) -> OSQPInt;
    #[link_name = "honest_osqp_update_rho_vec"]
    fn update_rho_vec(solver: *mut OSQPSolver) -> OSQPInt;
    #[link_name = "honest_osqp_swap_vectors"]
    fn swap_vectors(a: *mut *mut OSQPVectorf, b: *mut *mut OSQPVectorf);
    #[link_name = "honest_osqp_update_xz_tilde"]
    fn update_xz_tilde(solver: *mut OSQPSolver, admm_iter: OSQPInt);
    #[link_name = "honest_osqp_update_x"]
    fn update_x(solver: *mut OSQPSolver);
    #[link_name = "honest_osqp_update_z"]
    fn update_z(solver: *mut OSQPSolver);
    #[link_name = "honest_osqp_update_y"]
    fn update_y(solver: *mut OSQPSolver);
    #[link_name = "honest_osqp_compute_obj_val_dual_gap"]
    fn compute_obj_val_dual_gap(
        solver: *const OSQPSolver,
        x: *const OSQPVectorf,
        y: *const OSQPVectorf,
        prim_obj_val: *mut OSQPFloat,
        dual_obj_val: *mut OSQPFloat,
        duality_gap: *mut OSQPFloat,
    );
    #[link_name = "honest_osqp_has_solution"]
    fn has_solution(info: *const OSQPInfo) -> OSQPInt;
    #[link_name = "honest_osqp_store_solution"]
    fn store_solution(solver: *mut OSQPSolver, solution: *mut OSQPSolution);
    #[link_name = "honest_osqp_update_info"]
    fn update_info(solver: *mut OSQPSolver, iter: OSQPInt, polishing: OSQPInt);
    #[link_name = "honest_osqp_reset_info"]
    fn reset_info(info: *mut OSQPInfo);
    #[link_name = "honest_osqp_update_status"]
    fn update_status(info: *mut OSQPInfo, status_val: OSQPInt);
    #[link_name = "honest_osqp_check_termination"]
    fn check_termination(solver: *mut OSQPSolver, approximate: OSQPInt) -> OSQPInt;
    #[link_name = "honest_osqp_validate_data"]
    fn validate_data(
        P: *const OSQPCscMatrix,
        q: *const OSQPFloat,
        A: *const OSQPCscMatrix,
        l: *const OSQPFloat,
        u: *const OSQPFloat,
        m: OSQPInt,
        n: OSQPInt,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_validate_settings"]
    fn validate_settings(settings: *const OSQPSettings, from_setup: OSQPInt) -> OSQPInt;
    #[link_name = "honest_osqp_copy_settings"]
    fn copy_settings(settings: *const OSQPSettings) -> *mut OSQPSettings;
    #[link_name = "honest_osqp_osqp_algebra_linsys_supported"]
    fn osqp_algebra_linsys_supported() -> OSQPInt;
    #[link_name = "honest_osqp_osqp_algebra_default_linsys"]
    fn osqp_algebra_default_linsys() -> osqp_linsys_solver_type;
    #[link_name = "honest_osqp_osqp_algebra_init_libs"]
    fn osqp_algebra_init_libs(device: OSQPInt) -> OSQPInt;
    #[link_name = "honest_osqp_osqp_algebra_free_libs"]
    fn osqp_algebra_free_libs();
    #[link_name = "honest_osqp_osqp_algebra_init_linsys_solver"]
    fn osqp_algebra_init_linsys_solver(
        s: *mut *mut LinSysSolver,
        P: *const OSQPMatrix,
        A: *const OSQPMatrix,
        rho_vec: *const OSQPVectorf,
        settings: *const OSQPSettings,
        scaled_prim_res: *mut OSQPFloat,
        scaled_dual_res: *mut OSQPFloat,
        polishing: OSQPInt,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_scale_data"]
    fn scale_data(solver: *mut OSQPSolver) -> OSQPInt;
    #[link_name = "honest_osqp_unscale_data"]
    fn unscale_data(solver: *mut OSQPSolver) -> OSQPInt;
    #[link_name = "honest_osqp__osqp_error"]
    fn _osqp_error(
        error_code: osqp_error_type,
        function_name: *const ::core::ffi::c_char,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_polish"]
    fn polish(solver: *mut OSQPSolver) -> OSQPInt;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type osqp_capabilities_type = ::core::ffi::c_uint;
pub const OSQP_CAPABILITY_DERIVATIVES: osqp_capabilities_type = 16;
pub const OSQP_CAPABILITY_UPDATE_MATRICES: osqp_capabilities_type = 8;
pub const OSQP_CAPABILITY_CODEGEN: osqp_capabilities_type = 4;
pub const OSQP_CAPABILITY_INDIRECT_SOLVER: osqp_capabilities_type = 2;
pub const OSQP_CAPABILITY_DIRECT_SOLVER: osqp_capabilities_type = 1;
pub type osqp_status_type = ::core::ffi::c_uint;
pub const OSQP_UNSOLVED: osqp_status_type = 11;
pub const OSQP_SIGINT: osqp_status_type = 10;
pub const OSQP_NON_CVX: osqp_status_type = 9;
pub const OSQP_TIME_LIMIT_REACHED: osqp_status_type = 8;
pub const OSQP_MAX_ITER_REACHED: osqp_status_type = 7;
pub const OSQP_DUAL_INFEASIBLE_INACCURATE: osqp_status_type = 6;
pub const OSQP_DUAL_INFEASIBLE: osqp_status_type = 5;
pub const OSQP_PRIMAL_INFEASIBLE_INACCURATE: osqp_status_type = 4;
pub const OSQP_PRIMAL_INFEASIBLE: osqp_status_type = 3;
pub const OSQP_SOLVED_INACCURATE: osqp_status_type = 2;
pub const OSQP_SOLVED: osqp_status_type = 1;
pub type osqp_polish_status_type = ::core::ffi::c_int;
pub const OSQP_POLISH_NO_ACTIVE_SET_FOUND: osqp_polish_status_type = 2;
pub const OSQP_POLISH_SUCCESS: osqp_polish_status_type = 1;
pub const OSQP_POLISH_NOT_PERFORMED: osqp_polish_status_type = 0;
pub const OSQP_POLISH_FAILED: osqp_polish_status_type = -1;
pub const OSQP_POLISH_LINSYS_ERROR: osqp_polish_status_type = -2;
pub type osqp_linsys_solver_type = ::core::ffi::c_uint;
pub const OSQP_INDIRECT_SOLVER: osqp_linsys_solver_type = 2;
pub const OSQP_DIRECT_SOLVER: osqp_linsys_solver_type = 1;
pub const OSQP_UNKNOWN_SOLVER: osqp_linsys_solver_type = 0;
pub type osqp_precond_type = ::core::ffi::c_uint;
pub const OSQP_DIAGONAL_PRECONDITIONER: osqp_precond_type = 1;
pub const OSQP_NO_PRECONDITIONER: osqp_precond_type = 0;
pub type osqp_error_type = ::core::ffi::c_uint;
pub const OSQP_LAST_ERROR_PLACE: osqp_error_type = 12;
pub const OSQP_FUNC_NOT_IMPLEMENTED: osqp_error_type = 11;
pub const OSQP_DATA_NOT_INITIALIZED: osqp_error_type = 10;
pub const OSQP_CODEGEN_DEFINES_ERROR: osqp_error_type = 9;
pub const OSQP_FOPEN_ERROR: osqp_error_type = 8;
pub const OSQP_ALGEBRA_LOAD_ERROR: osqp_error_type = 7;
pub const OSQP_WORKSPACE_NOT_INIT_ERROR: osqp_error_type = 6;
pub const OSQP_MEM_ALLOC_ERROR: osqp_error_type = 5;
pub const OSQP_NONCVX_ERROR: osqp_error_type = 4;
pub const OSQP_LINSYS_SOLVER_INIT_ERROR: osqp_error_type = 3;
pub const OSQP_SETTINGS_VALIDATION_ERROR: osqp_error_type = 2;
pub const OSQP_DATA_VALIDATION_ERROR: osqp_error_type = 1;
pub const OSQP_NO_ERROR: osqp_error_type = 0;
pub type OSQPInt = ::core::ffi::c_int;
pub type OSQPFloat = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPCscMatrix {
    pub m: OSQPInt,
    pub n: OSQPInt,
    pub p: *mut OSQPInt,
    pub i: *mut OSQPInt,
    pub x: *mut OSQPFloat,
    pub nzmax: OSQPInt,
    pub nz: OSQPInt,
    pub owned: OSQPInt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPSettings {
    pub device: OSQPInt,
    pub linsys_solver: osqp_linsys_solver_type,
    pub allocate_solution: OSQPInt,
    pub verbose: OSQPInt,
    pub profiler_level: OSQPInt,
    pub warm_starting: OSQPInt,
    pub scaling: OSQPInt,
    pub polishing: OSQPInt,
    pub rho: OSQPFloat,
    pub rho_is_vec: OSQPInt,
    pub sigma: OSQPFloat,
    pub alpha: OSQPFloat,
    pub cg_max_iter: OSQPInt,
    pub cg_tol_reduction: OSQPInt,
    pub cg_tol_fraction: OSQPFloat,
    pub cg_precond: osqp_precond_type,
    pub adaptive_rho: OSQPInt,
    pub adaptive_rho_interval: OSQPInt,
    pub adaptive_rho_fraction: OSQPFloat,
    pub adaptive_rho_tolerance: OSQPFloat,
    pub max_iter: OSQPInt,
    pub eps_abs: OSQPFloat,
    pub eps_rel: OSQPFloat,
    pub eps_prim_inf: OSQPFloat,
    pub eps_dual_inf: OSQPFloat,
    pub scaled_termination: OSQPInt,
    pub check_termination: OSQPInt,
    pub check_dualgap: OSQPInt,
    pub time_limit: OSQPFloat,
    pub delta: OSQPFloat,
    pub polish_refine_iter: OSQPInt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPInfo {
    pub status: [::core::ffi::c_char; 32],
    pub status_val: OSQPInt,
    pub status_polish: OSQPInt,
    pub obj_val: OSQPFloat,
    pub dual_obj_val: OSQPFloat,
    pub prim_res: OSQPFloat,
    pub dual_res: OSQPFloat,
    pub duality_gap: OSQPFloat,
    pub iter: OSQPInt,
    pub rho_updates: OSQPInt,
    pub rho_estimate: OSQPFloat,
    pub setup_time: OSQPFloat,
    pub solve_time: OSQPFloat,
    pub update_time: OSQPFloat,
    pub polish_time: OSQPFloat,
    pub run_time: OSQPFloat,
    pub primdual_int: OSQPFloat,
    pub rel_kkt_error: OSQPFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPSolution {
    pub x: *mut OSQPFloat,
    pub y: *mut OSQPFloat,
    pub prim_inf_cert: *mut OSQPFloat,
    pub dual_inf_cert: *mut OSQPFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPWorkspace_ {
    pub data: *mut OSQPData,
    pub linsys_solver: *mut LinSysSolver,
    pub pol: *mut OSQPPolish,
    pub rho_vec: *mut OSQPVectorf,
    pub rho_inv_vec: *mut OSQPVectorf,
    pub constr_type: *mut OSQPVectori,
    pub x: *mut OSQPVectorf,
    pub y: *mut OSQPVectorf,
    pub z: *mut OSQPVectorf,
    pub xz_tilde: *mut OSQPVectorf,
    pub xtilde_view: *mut OSQPVectorf,
    pub ztilde_view: *mut OSQPVectorf,
    pub x_prev: *mut OSQPVectorf,
    pub z_prev: *mut OSQPVectorf,
    pub Ax: *mut OSQPVectorf,
    pub Px: *mut OSQPVectorf,
    pub Aty: *mut OSQPVectorf,
    pub xtPx: OSQPFloat,
    pub qtx: OSQPFloat,
    pub SC: OSQPFloat,
    pub scaled_dual_gap: OSQPFloat,
    pub delta_y: *mut OSQPVectorf,
    pub Atdelta_y: *mut OSQPVectorf,
    pub delta_x: *mut OSQPVectorf,
    pub Pdelta_x: *mut OSQPVectorf,
    pub Adelta_x: *mut OSQPVectorf,
    pub D_temp: *mut OSQPVectorf,
    pub D_temp_A: *mut OSQPVectorf,
    pub E_temp: *mut OSQPVectorf,
    pub scaling: *mut OSQPScaling,
    pub scaled_prim_res: OSQPFloat,
    pub scaled_dual_res: OSQPFloat,
    pub rho_inv: OSQPFloat,
    pub rho_updated: OSQPInt,
    pub last_rel_kkt: OSQPFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPScaling {
    pub c: OSQPFloat,
    pub D: *mut OSQPVectorf,
    pub E: *mut OSQPVectorf,
    pub cinv: OSQPFloat,
    pub Dinv: *mut OSQPVectorf,
    pub Einv: *mut OSQPVectorf,
}
pub type OSQPVectorf = OSQPVectorf_;
pub type OSQPVectori = OSQPVectori_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPPolish {
    pub Ared: *mut OSQPMatrix,
    pub n_active: OSQPInt,
    pub active_flags: *mut OSQPVectori,
    pub x: *mut OSQPVectorf,
    pub z: *mut OSQPVectorf,
    pub y: *mut OSQPVectorf,
    pub obj_val: OSQPFloat,
    pub dual_obj_val: OSQPFloat,
    pub duality_gap: OSQPFloat,
    pub prim_res: OSQPFloat,
    pub dual_res: OSQPFloat,
}
pub type OSQPMatrix = OSQPMatrix_;
pub type LinSysSolver = linsys_solver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linsys_solver {
    pub type_0: osqp_linsys_solver_type,
    pub name: Option<unsafe extern "C" fn(*mut LinSysSolver) -> *const ::core::ffi::c_char>,
    pub solve:
        Option<unsafe extern "C" fn(*mut LinSysSolver, *mut OSQPVectorf, OSQPInt) -> OSQPInt>,
    pub update_settings: Option<unsafe extern "C" fn(*mut LinSysSolver, *const OSQPSettings) -> ()>,
    pub warm_start: Option<unsafe extern "C" fn(*mut LinSysSolver, *const OSQPVectorf) -> ()>,
    pub adjoint_derivative: Option<unsafe extern "C" fn(*mut LinSysSolver) -> OSQPInt>,
    pub free: Option<unsafe extern "C" fn(*mut LinSysSolver) -> ()>,
    pub update_matrices: Option<
        unsafe extern "C" fn(
            *mut LinSysSolver,
            *const OSQPMatrix,
            *const OSQPInt,
            OSQPInt,
            *const OSQPMatrix,
            *const OSQPInt,
            OSQPInt,
        ) -> OSQPInt,
    >,
    pub update_rho_vec:
        Option<unsafe extern "C" fn(*mut LinSysSolver, *const OSQPVectorf, OSQPFloat) -> OSQPInt>,
    pub nthreads: OSQPInt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPData {
    pub n: OSQPInt,
    pub m: OSQPInt,
    pub P: *mut OSQPMatrix,
    pub A: *mut OSQPMatrix,
    pub q: *mut OSQPVectorf,
    pub l: *mut OSQPVectorf,
    pub u: *mut OSQPVectorf,
}
pub type OSQPWorkspace = OSQPWorkspace_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPSolver {
    pub settings: *mut OSQPSettings,
    pub solution: *mut OSQPSolution,
    pub info: *mut OSQPInfo,
    pub work: *mut OSQPWorkspace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPCodegenDefines {
    pub embedded_mode: OSQPInt,
    pub float_type: OSQPInt,
    pub printing_enable: OSQPInt,
    pub profiling_enable: OSQPInt,
    pub interrupt_enable: OSQPInt,
    pub derivatives_enable: OSQPInt,
}
pub const OSQP_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"1.0.0\0") };
pub const OSQP_VERBOSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OSQP_WARM_STARTING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OSQP_SCALING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OSQP_POLISHING: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OSQP_RHO: ::core::ffi::c_double = 0.1f64;
pub const OSQP_SIGMA: ::core::ffi::c_double = 1E-06f64;
pub const OSQP_ALPHA: ::core::ffi::c_double = 1.6f64;
pub const OSQP_RHO_MIN: ::core::ffi::c_double = 1e-06f64;
pub const OSQP_RHO_EQ_OVER_RHO_INEQ: ::core::ffi::c_double = 1e03f64;
pub const OSQP_RHO_IS_VEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OSQP_CG_MAX_ITER: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OSQP_CG_TOL_REDUCTION: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OSQP_CG_TOL_FRACTION: ::core::ffi::c_double = 0.15f64;
pub const OSQP_ADAPTIVE_RHO_UPDATE_DISABLED: ::core::ffi::c_int = 0;
pub const OSQP_ADAPTIVE_RHO_UPDATE_ITERATIONS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OSQP_ADAPTIVE_RHO_UPDATE_TIME: ::core::ffi::c_int = 2;
pub const OSQP_ADAPTIVE_RHO_UPDATE_KKT_ERROR: ::core::ffi::c_int = 3;
pub const OSQP_ADAPTIVE_RHO_UPDATE_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OSQP_ADAPTIVE_RHO_INTERVAL: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const OSQP_ADAPTIVE_RHO_TOLERANCE: ::core::ffi::c_double = 5.0f64;
pub const OSQP_ADAPTIVE_RHO_FRACTION: ::core::ffi::c_double = 0.4f64;
pub const OSQP_ADAPTIVE_RHO_MULTIPLE_TERMINATION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OSQP_ADAPTIVE_RHO_FIXED: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const OSQP_MAX_ITER: ::core::ffi::c_int = 4000 as ::core::ffi::c_int;
pub const OSQP_EPS_ABS: ::core::ffi::c_double = 1E-3f64;
pub const OSQP_EPS_REL: ::core::ffi::c_double = 1E-3f64;
pub const OSQP_EPS_PRIM_INF: ::core::ffi::c_double = 1E-4f64;
pub const OSQP_EPS_DUAL_INF: ::core::ffi::c_double = 1E-4f64;
pub const OSQP_SCALED_TERMINATION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OSQP_TIME_LIMIT: ::core::ffi::c_double = 1e10f64;
pub const OSQP_CHECK_DUALGAP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OSQP_CHECK_TERMINATION: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const OSQP_DELTA: ::core::ffi::c_double = 1E-6f64;
pub const OSQP_POLISH_REFINE_ITER: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OSQP_INFTY: OSQPFloat = 1e30f64;
#[export_name = "honest_osqp_OSQPCscMatrix_new"]
pub unsafe extern "C" fn OSQPCscMatrix_new(
    mut m: OSQPInt,
    mut n: OSQPInt,
    mut nzmax: OSQPInt,
    mut x: *mut OSQPFloat,
    mut i: *mut OSQPInt,
    mut p: *mut OSQPInt,
) -> *mut OSQPCscMatrix {
    let mut mat: *mut OSQPCscMatrix = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPCscMatrix>() as size_t,
    ) as *mut OSQPCscMatrix;
    if mat.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    OSQPCscMatrix_set_data(mat, m, n, nzmax, x, i, p);
    return mat;
}
#[export_name = "honest_osqp_OSQPCscMatrix_identity"]
pub unsafe extern "C" fn OSQPCscMatrix_identity(mut m: OSQPInt) -> *mut OSQPCscMatrix {
    return OSQPCscMatrix_diag_scalar(m, m, 1.0f64);
}
#[export_name = "honest_osqp_OSQPCscMatrix_diag_scalar"]
pub unsafe extern "C" fn OSQPCscMatrix_diag_scalar(
    mut m: OSQPInt,
    mut n: OSQPInt,
    mut scalar: OSQPFloat,
) -> *mut OSQPCscMatrix {
    let mut i: OSQPInt = 0;
    let mut min_elem: OSQPInt = if m < n { m } else { n };
    let mut mat: *mut OSQPCscMatrix = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPCscMatrix>() as size_t,
    ) as *mut OSQPCscMatrix;
    if mat.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    (*mat).m = m;
    (*mat).n = n;
    (*mat).nz = -(1 as ::core::ffi::c_int) as OSQPInt;
    (*mat).p = calloc(
        (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<OSQPInt>() as size_t,
    ) as *mut OSQPInt;
    if m < n {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < n {
            if i < m {
                *(*mat).p.offset(i as isize) = i;
            } else {
                *(*mat).p.offset(i as isize) = m;
            }
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < n {
            *(*mat).p.offset(i as isize) = i;
            i += 1;
        }
    }
    (*mat).nzmax = min_elem;
    (*mat).i =
        malloc((min_elem as size_t).wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t))
            as *mut OSQPInt;
    (*mat).x =
        malloc((min_elem as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
            as *mut OSQPFloat;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < min_elem {
        *(*mat).i.offset(i as isize) = i;
        *(*mat).x.offset(i as isize) = scalar;
        i += 1;
    }
    *(*mat).p.offset(n as isize) = min_elem;
    (*mat).owned = 1 as ::core::ffi::c_int as OSQPInt;
    return mat;
}
#[export_name = "honest_osqp_OSQPCscMatrix_diag_vec"]
pub unsafe extern "C" fn OSQPCscMatrix_diag_vec(
    mut m: OSQPInt,
    mut n: OSQPInt,
    mut vec: *mut OSQPFloat,
) -> *mut OSQPCscMatrix {
    let mut i: OSQPInt = 0;
    let mut min_elem: OSQPInt = if m < n { m } else { n };
    let mut mat: *mut OSQPCscMatrix = OSQPCscMatrix_diag_scalar(m, n, 1.0f64);
    if mat.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < min_elem {
        *(*mat).x.offset(i as isize) = *vec.offset(i as isize);
        i += 1;
    }
    return mat;
}
#[export_name = "honest_osqp_OSQPCscMatrix_zeros"]
pub unsafe extern "C" fn OSQPCscMatrix_zeros(mut m: OSQPInt, mut n: OSQPInt) -> *mut OSQPCscMatrix {
    let mut mat: *mut OSQPCscMatrix = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPCscMatrix>() as size_t,
    ) as *mut OSQPCscMatrix;
    if mat.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    (*mat).m = m;
    (*mat).n = n;
    (*mat).nz = -(1 as ::core::ffi::c_int) as OSQPInt;
    (*mat).nzmax = 0 as ::core::ffi::c_int as OSQPInt;
    (*mat).x = ::core::ptr::null_mut::<OSQPFloat>();
    (*mat).i = ::core::ptr::null_mut::<OSQPInt>();
    (*mat).p = calloc(
        (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<OSQPInt>() as size_t,
    ) as *mut OSQPInt;
    (*mat).owned = 1 as ::core::ffi::c_int as OSQPInt;
    return mat;
}
#[export_name = "honest_osqp_OSQPCscMatrix_free"]
pub unsafe extern "C" fn OSQPCscMatrix_free(mut mat: *mut OSQPCscMatrix) {
    if !mat.is_null() {
        if (*mat).owned != 0 {
            if !(*mat).p.is_null() {
                free((*mat).p as *mut ::core::ffi::c_void);
            }
            if !(*mat).i.is_null() {
                free((*mat).i as *mut ::core::ffi::c_void);
            }
            if !(*mat).x.is_null() {
                free((*mat).x as *mut ::core::ffi::c_void);
            }
        }
        free(mat as *mut ::core::ffi::c_void);
    }
}
#[export_name = "honest_osqp_OSQPCscMatrix_set_data"]
pub unsafe extern "C" fn OSQPCscMatrix_set_data(
    mut M: *mut OSQPCscMatrix,
    mut m: OSQPInt,
    mut n: OSQPInt,
    mut nzmax: OSQPInt,
    mut x: *mut OSQPFloat,
    mut i: *mut OSQPInt,
    mut p: *mut OSQPInt,
) {
    (*M).m = m;
    (*M).n = n;
    (*M).nz = -(1 as ::core::ffi::c_int) as OSQPInt;
    (*M).nzmax = nzmax;
    (*M).x = x;
    (*M).i = i;
    (*M).p = p;
    (*M).owned = 0 as ::core::ffi::c_int as OSQPInt;
}
#[export_name = "honest_osqp_OSQPSettings_new"]
pub unsafe extern "C" fn OSQPSettings_new() -> *mut OSQPSettings {
    let mut settings: *mut OSQPSettings = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPSettings>() as size_t,
    ) as *mut OSQPSettings;
    if settings.is_null() {
        return ::core::ptr::null_mut::<OSQPSettings>();
    }
    osqp_set_default_settings(settings);
    return settings;
}
#[export_name = "honest_osqp_OSQPSettings_free"]
pub unsafe extern "C" fn OSQPSettings_free(mut settings: *mut OSQPSettings) {
    if !settings.is_null() {
        free(settings as *mut ::core::ffi::c_void);
    }
}
#[export_name = "honest_osqp_OSQPCodegenDefines_new"]
pub unsafe extern "C" fn OSQPCodegenDefines_new() -> *mut OSQPCodegenDefines {
    let mut defs: *mut OSQPCodegenDefines = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPCodegenDefines>() as size_t,
    ) as *mut OSQPCodegenDefines;
    if defs.is_null() {
        return ::core::ptr::null_mut::<OSQPCodegenDefines>();
    }
    osqp_set_default_codegen_defines(defs);
    return defs;
}
#[export_name = "honest_osqp_OSQPCodegenDefines_free"]
pub unsafe extern "C" fn OSQPCodegenDefines_free(mut defs: *mut OSQPCodegenDefines) {
    if !defs.is_null() {
        free(defs as *mut ::core::ffi::c_void);
    }
}
#[export_name = "honest_osqp_osqp_capabilities"]
pub unsafe extern "C" fn osqp_capabilities() -> OSQPInt {
    let mut capabilities: OSQPInt = 0 as OSQPInt;
    capabilities |= osqp_algebra_linsys_supported() as ::core::ffi::c_int;
    capabilities |= OSQP_CAPABILITY_UPDATE_MATRICES as ::core::ffi::c_int;
    return capabilities;
}
#[export_name = "honest_osqp_osqp_version"]
pub unsafe extern "C" fn osqp_version() -> *const ::core::ffi::c_char {
    return OSQP_VERSION.as_ptr();
}
#[export_name = "honest_osqp_osqp_error_message"]
pub unsafe extern "C" fn osqp_error_message(mut error_flag: OSQPInt) -> *const ::core::ffi::c_char {
    if error_flag >= OSQP_LAST_ERROR_PLACE as ::core::ffi::c_int {
        return *(&raw mut OSQP_ERROR_MESSAGE as *mut *const ::core::ffi::c_char).offset(
            (OSQP_LAST_ERROR_PLACE as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
        );
    }
    return *(&raw mut OSQP_ERROR_MESSAGE as *mut *const ::core::ffi::c_char)
        .offset((error_flag as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
}
#[export_name = "honest_osqp_osqp_get_dimensions"]
pub unsafe extern "C" fn osqp_get_dimensions(
    mut solver: *mut OSQPSolver,
    mut m: *mut OSQPInt,
    mut n: *mut OSQPInt,
) {
    if solver.is_null() || (*solver).work.is_null() || (*(*solver).work).data.is_null() {
        *m = -(1 as ::core::ffi::c_int) as OSQPInt;
        *n = -(1 as ::core::ffi::c_int) as OSQPInt;
    } else {
        *m = (*(*(*solver).work).data).m;
        *n = (*(*(*solver).work).data).n;
    };
}
#[export_name = "honest_osqp_osqp_set_default_codegen_defines"]
pub unsafe extern "C" fn osqp_set_default_codegen_defines(mut defines: *mut OSQPCodegenDefines) {
    if defines.is_null() {
        return;
    }
    (*defines).embedded_mode = 1 as ::core::ffi::c_int as OSQPInt;
    (*defines).float_type = 0 as ::core::ffi::c_int as OSQPInt;
    (*defines).printing_enable = 0 as ::core::ffi::c_int as OSQPInt;
    (*defines).profiling_enable = 0 as ::core::ffi::c_int as OSQPInt;
    (*defines).interrupt_enable = 0 as ::core::ffi::c_int as OSQPInt;
    (*defines).derivatives_enable = 0 as ::core::ffi::c_int as OSQPInt;
}
#[export_name = "honest_osqp_osqp_set_default_settings"]
pub unsafe extern "C" fn osqp_set_default_settings(mut settings: *mut OSQPSettings) {
    if settings.is_null() {
        return;
    }
    (*settings).device = 0 as ::core::ffi::c_int as OSQPInt;
    (*settings).linsys_solver = osqp_algebra_default_linsys();
    (*settings).allocate_solution = 1 as ::core::ffi::c_int as OSQPInt;
    (*settings).profiler_level = 0 as ::core::ffi::c_int as OSQPInt;
    (*settings).verbose = OSQP_VERBOSE as OSQPInt;
    (*settings).warm_starting = OSQP_WARM_STARTING as OSQPInt;
    (*settings).scaling = OSQP_SCALING as OSQPInt;
    (*settings).polishing = OSQP_POLISHING as OSQPInt;
    (*settings).rho = OSQP_RHO;
    (*settings).rho_is_vec = OSQP_RHO_IS_VEC as OSQPInt;
    (*settings).sigma = OSQP_SIGMA;
    (*settings).alpha = OSQP_ALPHA;
    (*settings).cg_max_iter = OSQP_CG_MAX_ITER as OSQPInt;
    (*settings).cg_tol_reduction = OSQP_CG_TOL_REDUCTION as OSQPInt;
    (*settings).cg_tol_fraction = OSQP_CG_TOL_FRACTION as OSQPFloat;
    (*settings).cg_precond = OSQP_DIAGONAL_PRECONDITIONER;
    (*settings).adaptive_rho = OSQP_ADAPTIVE_RHO_UPDATE_DEFAULT as OSQPInt;
    (*settings).adaptive_rho_interval = OSQP_ADAPTIVE_RHO_INTERVAL as OSQPInt;
    (*settings).adaptive_rho_fraction = OSQP_ADAPTIVE_RHO_FRACTION;
    (*settings).adaptive_rho_tolerance = OSQP_ADAPTIVE_RHO_TOLERANCE;
    (*settings).max_iter = OSQP_MAX_ITER as OSQPInt;
    (*settings).eps_abs = OSQP_EPS_ABS;
    (*settings).eps_rel = OSQP_EPS_REL;
    (*settings).eps_prim_inf = OSQP_EPS_PRIM_INF;
    (*settings).eps_dual_inf = OSQP_EPS_DUAL_INF;
    (*settings).scaled_termination = OSQP_SCALED_TERMINATION as OSQPInt;
    (*settings).check_termination = OSQP_CHECK_TERMINATION as OSQPInt;
    (*settings).check_dualgap = OSQP_CHECK_DUALGAP as OSQPInt;
    (*settings).time_limit = OSQP_TIME_LIMIT as OSQPFloat;
    (*settings).delta = OSQP_DELTA as OSQPFloat;
    (*settings).polish_refine_iter = OSQP_POLISH_REFINE_ITER as OSQPInt;
}
#[export_name = "honest_osqp_osqp_setup"]
pub unsafe extern "C" fn osqp_setup(
    mut solverp: *mut *mut OSQPSolver,
    mut P: *const OSQPCscMatrix,
    mut q: *const OSQPFloat,
    mut A: *const OSQPCscMatrix,
    mut l: *const OSQPFloat,
    mut u: *const OSQPFloat,
    mut m: OSQPInt,
    mut n: OSQPInt,
    mut settings: *const OSQPSettings,
) -> OSQPInt {
    let mut exitflag: OSQPInt = 0;
    let mut solver: *mut OSQPSolver = ::core::ptr::null_mut::<OSQPSolver>();
    let mut work: *mut OSQPWorkspace = ::core::ptr::null_mut::<OSQPWorkspace>();
    if validate_data(P, q, A, l, u, m, n) != 0 {
        return _osqp_error(
            OSQP_DATA_VALIDATION_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if validate_settings(settings, 1 as OSQPInt) != 0 {
        return _osqp_error(
            OSQP_SETTINGS_VALIDATION_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    solver = calloc(1 as size_t, ::core::mem::size_of::<OSQPSolver>() as size_t) as *mut OSQPSolver;
    if solver.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *solverp = solver;
    work = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPWorkspace>() as size_t,
    ) as *mut OSQPWorkspace;
    if work.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*solver).work = work;
    (*solver).info =
        calloc(1 as size_t, ::core::mem::size_of::<OSQPInfo>() as size_t) as *mut OSQPInfo;
    if (*solver).info.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    exitflag = osqp_algebra_init_libs((*settings).device);
    if exitflag != 0 {
        return _osqp_error(
            OSQP_ALGEBRA_LOAD_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*work).data =
        calloc(1 as size_t, ::core::mem::size_of::<OSQPData>() as size_t) as *mut OSQPData;
    if (*work).data.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*work).data).m = m;
    (*(*work).data).n = n;
    (*(*work).data).P = OSQPMatrix_new_from_csc(P, 1 as OSQPInt);
    (*(*work).data).q = OSQPVectorf_new(q, n);
    if (*(*work).data).P.is_null() || (*(*work).data).q.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*work).data).A = OSQPMatrix_new_from_csc(A, 0 as OSQPInt);
    if (*(*work).data).A.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*work).data).l = OSQPVectorf_new(l, m);
    (*(*work).data).u = OSQPVectorf_new(u, m);
    if (*(*work).data).l.is_null() || (*(*work).data).u.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*settings).rho_is_vec != 0 {
        (*work).rho_vec = OSQPVectorf_malloc(m);
        (*work).rho_inv_vec = OSQPVectorf_malloc(m);
        if (*work).rho_vec.is_null() || (*work).rho_inv_vec.is_null() {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*work).constr_type = OSQPVectori_calloc(m);
        if (*work).constr_type.is_null() {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        (*work).rho_vec = ::core::ptr::null_mut::<OSQPVectorf>();
        (*work).rho_inv_vec = ::core::ptr::null_mut::<OSQPVectorf>();
    }
    (*work).x = OSQPVectorf_calloc(n);
    (*work).z = OSQPVectorf_calloc(m);
    (*work).xz_tilde = OSQPVectorf_calloc(n + m);
    (*work).xtilde_view = OSQPVectorf_view((*work).xz_tilde, 0 as OSQPInt, n);
    (*work).ztilde_view = OSQPVectorf_view((*work).xz_tilde, n, m);
    (*work).x_prev = OSQPVectorf_calloc(n);
    (*work).z_prev = OSQPVectorf_calloc(m);
    (*work).y = OSQPVectorf_calloc(m);
    if (*work).x.is_null() || (*work).z.is_null() || (*work).xz_tilde.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*work).xtilde_view.is_null() || (*work).ztilde_view.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*work).x_prev.is_null() || (*work).z_prev.is_null() || (*work).y.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*work).Ax = OSQPVectorf_calloc(m);
    (*work).Px = OSQPVectorf_calloc(n);
    (*work).Aty = OSQPVectorf_calloc(n);
    (*work).delta_y = OSQPVectorf_calloc(m);
    (*work).Atdelta_y = OSQPVectorf_calloc(n);
    (*work).delta_x = OSQPVectorf_calloc(n);
    (*work).Pdelta_x = OSQPVectorf_calloc(n);
    (*work).Adelta_x = OSQPVectorf_calloc(m);
    if (*work).Ax.is_null() || (*work).Px.is_null() || (*work).Aty.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*work).delta_y.is_null() || (*work).Atdelta_y.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*work).delta_x.is_null() || (*work).Pdelta_x.is_null() || (*work).Adelta_x.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*solver).settings = copy_settings(settings);
    if (*solver).settings.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*settings).scaling != 0 {
        (*work).scaling =
            malloc(::core::mem::size_of::<OSQPScaling>() as size_t) as *mut OSQPScaling;
        if (*work).scaling.is_null() {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*work).scaling).D = OSQPVectorf_calloc(n);
        (*(*work).scaling).Dinv = OSQPVectorf_calloc(n);
        (*(*work).scaling).E = OSQPVectorf_calloc(m);
        (*(*work).scaling).Einv = OSQPVectorf_calloc(m);
        if (*(*work).scaling).D.is_null()
            || (*(*work).scaling).Dinv.is_null()
            || (*(*work).scaling).E.is_null()
            || (*(*work).scaling).Einv.is_null()
        {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*work).D_temp = OSQPVectorf_calloc(n);
        (*work).D_temp_A = OSQPVectorf_calloc(n);
        (*work).E_temp = OSQPVectorf_calloc(m);
        if (*work).D_temp.is_null() || (*work).D_temp_A.is_null() || (*work).E_temp.is_null() {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        scale_data(solver);
    } else {
        (*work).scaling = ::core::ptr::null_mut::<OSQPScaling>();
        (*work).D_temp = ::core::ptr::null_mut::<OSQPVectorf>();
        (*work).D_temp_A = ::core::ptr::null_mut::<OSQPVectorf>();
        (*work).E_temp = ::core::ptr::null_mut::<OSQPVectorf>();
    }
    if (*settings).rho_is_vec != 0 {
        set_rho_vec(solver);
    } else {
        (*(*solver).settings).rho = (if (if (*settings).rho > 1e-06f64 {
            (*settings).rho as ::core::ffi::c_double
        } else {
            1e-06f64
        }) < 1e06f64
        {
            if (*settings).rho > 1e-06f64 {
                (*settings).rho as ::core::ffi::c_double
            } else {
                1e-06f64
            }
        } else {
            1e06f64
        }) as OSQPFloat;
        (*work).rho_inv = 1.0f64 / (*settings).rho;
    }
    exitflag = osqp_algebra_init_linsys_solver(
        &raw mut (*work).linsys_solver,
        (*(*work).data).P,
        (*(*work).data).A,
        (*work).rho_vec,
        (*solver).settings,
        &raw mut (*work).scaled_prim_res,
        &raw mut (*work).scaled_dual_res,
        0 as OSQPInt,
    );
    if exitflag == OSQP_NONCVX_ERROR as ::core::ffi::c_int {
        update_status(
            (*solver).info,
            OSQP_NON_CVX as ::core::ffi::c_int as OSQPInt,
        );
        return _osqp_error(
            exitflag as osqp_error_type,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if exitflag != 0 {
        return _osqp_error(
            exitflag as osqp_error_type,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    osqp_cold_start(solver);
    (*work).pol = malloc(::core::mem::size_of::<OSQPPolish>() as size_t) as *mut OSQPPolish;
    if (*work).pol.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*work).pol).active_flags = OSQPVectori_malloc(m);
    (*(*work).pol).x = OSQPVectorf_malloc(n);
    (*(*work).pol).z = OSQPVectorf_malloc(m);
    (*(*work).pol).y = OSQPVectorf_malloc(m);
    if (*(*work).pol).x.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*(*work).pol).active_flags.is_null()
        || (*(*work).pol).z.is_null()
        || (*(*work).pol).y.is_null()
    {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*settings).allocate_solution != 0 {
        (*solver).solution = calloc(
            1 as size_t,
            ::core::mem::size_of::<OSQPSolution>() as size_t,
        ) as *mut OSQPSolution;
        if (*solver).solution.is_null() {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*solver).solution).x = calloc(
            1 as size_t,
            (n as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t),
        ) as *mut OSQPFloat;
        (*(*solver).solution).y = calloc(
            1 as size_t,
            (m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t),
        ) as *mut OSQPFloat;
        (*(*solver).solution).prim_inf_cert = calloc(
            1 as size_t,
            (m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t),
        ) as *mut OSQPFloat;
        (*(*solver).solution).dual_inf_cert = calloc(
            1 as size_t,
            (n as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t),
        ) as *mut OSQPFloat;
        if (*(*solver).solution).x.is_null() || (*(*solver).solution).dual_inf_cert.is_null() {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if m != 0
            && ((*(*solver).solution).y.is_null() || (*(*solver).solution).prim_inf_cert.is_null())
        {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"osqp_setup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        (*solver).solution = ::core::ptr::null_mut::<OSQPSolution>();
    }
    (*(*solver).info).status_polish = OSQP_POLISH_NOT_PERFORMED as ::core::ffi::c_int as OSQPInt;
    update_status(
        (*solver).info,
        OSQP_UNSOLVED as ::core::ffi::c_int as OSQPInt,
    );
    (*(*solver).info).rho_updates = 0 as ::core::ffi::c_int as OSQPInt;
    (*(*solver).info).rho_estimate = (*(*solver).settings).rho;
    (*(*solver).info).obj_val = OSQP_INFTY;
    (*(*solver).info).prim_res = OSQP_INFTY;
    (*(*solver).info).dual_res = OSQP_INFTY;
    (*(*solver).info).rel_kkt_error = OSQP_INFTY;
    (*work).last_rel_kkt = OSQP_INFTY;
    (*work).rho_updated = 0 as ::core::ffi::c_int as OSQPInt;
    match (*(*solver).settings).adaptive_rho {
        OSQP_ADAPTIVE_RHO_UPDATE_ITERATIONS => {
            if (*(*solver).settings).adaptive_rho_interval == 0 as ::core::ffi::c_int {
                if (*(*solver).settings).check_termination != 0 {
                    (*(*solver).settings).adaptive_rho_interval =
                        OSQP_ADAPTIVE_RHO_MULTIPLE_TERMINATION
                            * (*(*solver).settings).check_termination;
                } else {
                    (*(*solver).settings).adaptive_rho_interval =
                        OSQP_ADAPTIVE_RHO_FIXED as OSQPInt;
                }
            }
        }
        OSQP_ADAPTIVE_RHO_UPDATE_TIME => {}
        OSQP_ADAPTIVE_RHO_UPDATE_KKT_ERROR => {
            if (*(*solver).settings).adaptive_rho_interval == 0 as ::core::ffi::c_int {
                (*(*solver).settings).adaptive_rho_interval = 1 as ::core::ffi::c_int as OSQPInt;
            }
        }
        OSQP_ADAPTIVE_RHO_UPDATE_DISABLED | _ => {}
    }
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_osqp_solve"]
pub unsafe extern "C" fn osqp_solve(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut current_block: u64;
    let mut exitflag: OSQPInt = 0;
    let mut iter: OSQPInt = 0;
    let mut max_iter: OSQPInt = 0;
    let mut can_print: OSQPInt = 0 as OSQPInt;
    let mut can_adapt_rho: OSQPInt = 0 as OSQPInt;
    let mut can_check_termination: OSQPInt = 0 as OSQPInt;
    let mut work: *mut OSQPWorkspace = ::core::ptr::null_mut::<OSQPWorkspace>();
    let mut settings: *mut OSQPSettings = ::core::ptr::null_mut::<OSQPSettings>();
    if solver.is_null() || (*solver).work.is_null() {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_solve\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    work = (*solver).work;
    settings = (*solver).settings;
    exitflag = 0 as ::core::ffi::c_int as OSQPInt;
    can_check_termination = 0 as ::core::ffi::c_int as OSQPInt;
    if (*settings).warm_starting == 0 {
        osqp_cold_start(solver);
    }
    max_iter = (*settings).max_iter;
    iter = 1 as ::core::ffi::c_int as OSQPInt;
    loop {
        if !(iter <= max_iter) {
            current_block = 3689906465960840878;
            break;
        }
        swap_vectors(&raw mut (*work).x, &raw mut (*work).x_prev);
        swap_vectors(&raw mut (*work).z, &raw mut (*work).z_prev);
        update_xz_tilde(solver, iter);
        update_x(solver);
        update_z(solver);
        update_y(solver);
        can_check_termination = ((*settings).check_termination != 0
            && iter % (*settings).check_termination == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as OSQPInt;
        can_print = 0 as ::core::ffi::c_int as OSQPInt;
        match (*settings).adaptive_rho {
            OSQP_ADAPTIVE_RHO_UPDATE_DISABLED => {
                can_adapt_rho = 0 as ::core::ffi::c_int as OSQPInt;
            }
            OSQP_ADAPTIVE_RHO_UPDATE_TIME => {
                can_adapt_rho = 0 as ::core::ffi::c_int as OSQPInt;
            }
            OSQP_ADAPTIVE_RHO_UPDATE_KKT_ERROR | OSQP_ADAPTIVE_RHO_UPDATE_ITERATIONS => {
                if (*settings).adaptive_rho_interval != 0
                    && iter % (*settings).adaptive_rho_interval == 0 as ::core::ffi::c_int
                {
                    can_adapt_rho = 1 as ::core::ffi::c_int as OSQPInt;
                } else {
                    can_adapt_rho = 0 as ::core::ffi::c_int as OSQPInt;
                }
            }
            _ => {}
        }
        if can_check_termination != 0
            || can_print != 0
            || can_adapt_rho != 0
            || iter == 1 as ::core::ffi::c_int
        {
            update_info(solver, iter, 0 as OSQPInt);
        }
        if can_check_termination != 0 {
            if check_termination(solver, 0 as OSQPInt) != 0 {
                current_block = 3689906465960840878;
                break;
            }
        }
        (*work).rho_updated = 0 as ::core::ffi::c_int as OSQPInt;
        if can_adapt_rho != 0 && (*settings).adaptive_rho == OSQP_ADAPTIVE_RHO_UPDATE_KKT_ERROR {
            if (*(*solver).info).rel_kkt_error
                <= (*settings).adaptive_rho_fraction * (*work).last_rel_kkt
            {
                can_adapt_rho = 1 as ::core::ffi::c_int as OSQPInt;
            } else {
                can_adapt_rho = 0 as ::core::ffi::c_int as OSQPInt;
            }
        }
        if can_adapt_rho != 0 {
            if adapt_rho(solver) != 0 {
                exitflag = 1 as ::core::ffi::c_int as OSQPInt;
                current_block = 15970011996474399071;
                break;
            }
        }
        if (*work).rho_updated != 0 {
            (*work).last_rel_kkt = (*(*solver).info).rel_kkt_error;
        }
        iter += 1;
    }
    match current_block {
        3689906465960840878 => {
            if can_check_termination == 0 {
                update_info(solver, iter - 1 as OSQPInt, 0 as OSQPInt);
                check_termination(solver, 0 as OSQPInt);
            }
            if has_solution((*solver).info) != 0 {
                compute_obj_val_dual_gap(
                    solver,
                    (*work).x,
                    (*work).y,
                    &raw mut (*(*solver).info).obj_val,
                    &raw mut (*(*solver).info).dual_obj_val,
                    &raw mut (*(*solver).info).duality_gap,
                );
            }
            if (*(*solver).info).status_val == OSQP_UNSOLVED as ::core::ffi::c_int {
                if check_termination(solver, 1 as OSQPInt) == 0 {
                    update_status(
                        (*solver).info,
                        OSQP_MAX_ITER_REACHED as ::core::ffi::c_int as OSQPInt,
                    );
                }
            }
            (*(*solver).info).rho_estimate = compute_rho_estimate(solver);
            if (*settings).polishing != 0
                && (*(*solver).info).status_val == OSQP_SOLVED as ::core::ffi::c_int
            {
                exitflag = polish(solver);
                if exitflag > 0 as ::core::ffi::c_int {
                    current_block = 15970011996474399071;
                } else {
                    current_block = 307447392441238883;
                }
            } else {
                current_block = 307447392441238883;
            }
            match current_block {
                15970011996474399071 => {}
                _ => {
                    store_solution(solver, (*solver).solution);
                }
            }
        }
        _ => {}
    }
    return exitflag;
}
#[export_name = "honest_osqp_osqp_get_solution"]
pub unsafe extern "C" fn osqp_get_solution(
    mut solver: *mut OSQPSolver,
    mut solution: *mut OSQPSolution,
) -> OSQPInt {
    if solver.is_null()
        || (*solver).work.is_null()
        || (*solver).settings.is_null()
        || (*solver).info.is_null()
    {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_get_solution\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if solution.is_null() {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_get_solution\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    store_solution(solver, solution);
    return OSQP_NO_ERROR as ::core::ffi::c_int as OSQPInt;
}
#[export_name = "honest_osqp_osqp_cleanup"]
pub unsafe extern "C" fn osqp_cleanup(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut exitflag: OSQPInt = 0 as OSQPInt;
    let mut work: *mut OSQPWorkspace = ::core::ptr::null_mut::<OSQPWorkspace>();
    if solver.is_null() {
        return 0 as OSQPInt;
    }
    work = (*solver).work;
    if !work.is_null() {
        osqp_algebra_free_libs();
        if !(*work).data.is_null() {
            OSQPMatrix_free((*(*work).data).P);
            OSQPMatrix_free((*(*work).data).A);
            OSQPVectorf_free((*(*work).data).q);
            OSQPVectorf_free((*(*work).data).l);
            OSQPVectorf_free((*(*work).data).u);
            free((*work).data as *mut ::core::ffi::c_void);
        }
        if !(*work).scaling.is_null() {
            OSQPVectorf_free((*(*work).scaling).D);
            OSQPVectorf_free((*(*work).scaling).Dinv);
            OSQPVectorf_free((*(*work).scaling).E);
            OSQPVectorf_free((*(*work).scaling).Einv);
        }
        free((*work).scaling as *mut ::core::ffi::c_void);
        OSQPVectorf_free((*work).D_temp);
        OSQPVectorf_free((*work).D_temp_A);
        OSQPVectorf_free((*work).E_temp);
        if !(*work).linsys_solver.is_null() {
            if (*(*work).linsys_solver).free.is_some() {
                (*(*work).linsys_solver)
                    .free
                    .expect("non-null function pointer")((*work).linsys_solver);
            }
        }
        if !(*work).pol.is_null() {
            OSQPVectori_free((*(*work).pol).active_flags);
            OSQPVectorf_free((*(*work).pol).x);
            OSQPVectorf_free((*(*work).pol).z);
            OSQPVectorf_free((*(*work).pol).y);
            free((*work).pol as *mut ::core::ffi::c_void);
        }
        OSQPVectorf_free((*work).rho_vec);
        OSQPVectorf_free((*work).rho_inv_vec);
        OSQPVectori_free((*work).constr_type);
        OSQPVectorf_free((*work).x);
        OSQPVectorf_free((*work).z);
        OSQPVectorf_free((*work).xz_tilde);
        OSQPVectorf_view_free((*work).xtilde_view);
        OSQPVectorf_view_free((*work).ztilde_view);
        OSQPVectorf_free((*work).x_prev);
        OSQPVectorf_free((*work).z_prev);
        OSQPVectorf_free((*work).y);
        OSQPVectorf_free((*work).Ax);
        OSQPVectorf_free((*work).Px);
        OSQPVectorf_free((*work).Aty);
        OSQPVectorf_free((*work).delta_y);
        OSQPVectorf_free((*work).Atdelta_y);
        OSQPVectorf_free((*work).delta_x);
        OSQPVectorf_free((*work).Pdelta_x);
        OSQPVectorf_free((*work).Adelta_x);
        if !(*solver).settings.is_null() {
            free((*solver).settings as *mut ::core::ffi::c_void);
        }
        if !(*solver).solution.is_null() {
            free((*(*solver).solution).x as *mut ::core::ffi::c_void);
            free((*(*solver).solution).y as *mut ::core::ffi::c_void);
            free((*(*solver).solution).prim_inf_cert as *mut ::core::ffi::c_void);
            free((*(*solver).solution).dual_inf_cert as *mut ::core::ffi::c_void);
            free((*solver).solution as *mut ::core::ffi::c_void);
        }
        if !(*solver).info.is_null() {
            free((*solver).info as *mut ::core::ffi::c_void);
        }
        free(work as *mut ::core::ffi::c_void);
    }
    free(solver as *mut ::core::ffi::c_void);
    return exitflag;
}
#[export_name = "honest_osqp_osqp_update_data_vec"]
pub unsafe extern "C" fn osqp_update_data_vec(
    mut solver: *mut OSQPSolver,
    mut q_new: *const OSQPFloat,
    mut l_new: *const OSQPFloat,
    mut u_new: *const OSQPFloat,
) -> OSQPInt {
    let mut exitflag: OSQPInt = 0 as OSQPInt;
    let mut l_tmp: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut u_tmp: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut work: *mut OSQPWorkspace = ::core::ptr::null_mut::<OSQPWorkspace>();
    if solver.is_null() || (*solver).work.is_null() {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_update_data_vec\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    work = (*solver).work;
    if !l_new.is_null() || !u_new.is_null() {
        l_tmp = (*work).z_prev;
        u_tmp = (*work).delta_y;
        if !l_new.is_null() {
            OSQPVectorf_from_raw(l_tmp, l_new);
        }
        if !u_new.is_null() {
            OSQPVectorf_from_raw(u_tmp, u_new);
        }
        if (*(*solver).settings).scaling != 0 {
            if !l_new.is_null() {
                OSQPVectorf_ew_prod(l_tmp, l_tmp, (*(*work).scaling).E);
            }
            if !u_new.is_null() {
                OSQPVectorf_ew_prod(u_tmp, u_tmp, (*(*work).scaling).E);
            }
        }
        if !l_new.is_null() && !u_new.is_null() {
            exitflag = (OSQPVectorf_all_leq(l_tmp, u_tmp) == 0) as ::core::ffi::c_int as OSQPInt;
        } else if !l_new.is_null() {
            exitflag = (OSQPVectorf_all_leq(l_tmp, (*(*work).data).u) == 0) as ::core::ffi::c_int
                as OSQPInt;
        } else {
            exitflag = (OSQPVectorf_all_leq((*(*work).data).l, u_tmp) == 0) as ::core::ffi::c_int
                as OSQPInt;
        }
        if exitflag != 0 {
            return _osqp_error(
                OSQP_DATA_VALIDATION_ERROR,
                b"osqp_update_data_vec\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !l_new.is_null() {
            swap_vectors(&raw mut (*work).z_prev, &raw mut (*(*work).data).l);
        }
        if !u_new.is_null() {
            swap_vectors(&raw mut (*work).delta_y, &raw mut (*(*work).data).u);
        }
        if (*(*solver).settings).rho_is_vec != 0 {
            exitflag = update_rho_vec(solver);
        }
    }
    if !q_new.is_null() {
        OSQPVectorf_from_raw((*(*work).data).q, q_new);
        if (*(*solver).settings).scaling != 0 {
            OSQPVectorf_ew_prod((*(*work).data).q, (*(*work).data).q, (*(*work).scaling).D);
            OSQPVectorf_mult_scalar((*(*work).data).q, (*(*work).scaling).c);
        }
    }
    reset_info((*solver).info);
    return exitflag;
}
#[export_name = "honest_osqp_osqp_warm_start"]
pub unsafe extern "C" fn osqp_warm_start(
    mut solver: *mut OSQPSolver,
    mut x: *const OSQPFloat,
    mut y: *const OSQPFloat,
) -> OSQPInt {
    let mut work: *mut OSQPWorkspace = ::core::ptr::null_mut::<OSQPWorkspace>();
    if solver.is_null() || (*solver).work.is_null() {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_warm_start\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    work = (*solver).work;
    if (*(*solver).settings).warm_starting == 0 {
        (*(*solver).settings).warm_starting = 1 as ::core::ffi::c_int as OSQPInt;
    }
    if !x.is_null() {
        OSQPVectorf_from_raw((*work).x, x);
    }
    if !y.is_null() {
        OSQPVectorf_from_raw((*work).y, y);
    }
    if (*(*solver).settings).scaling != 0 {
        if !x.is_null() {
            OSQPVectorf_ew_prod((*work).x, (*work).x, (*(*work).scaling).Dinv);
        }
        if !y.is_null() {
            OSQPVectorf_ew_prod((*work).y, (*work).y, (*(*work).scaling).Einv);
            OSQPVectorf_mult_scalar((*work).y, (*(*work).scaling).c);
        }
    }
    if !x.is_null() {
        OSQPMatrix_Axpy((*(*work).data).A, (*work).x, (*work).z, 1.0f64, 0.0f64);
    }
    (*(*work).linsys_solver)
        .warm_start
        .expect("non-null function pointer")((*work).linsys_solver, (*work).x);
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_osqp_cold_start"]
pub unsafe extern "C" fn osqp_cold_start(mut solver: *mut OSQPSolver) {
    let mut work: *mut OSQPWorkspace = (*solver).work;
    OSQPVectorf_set_scalar((*work).x, 0.0f64);
    OSQPVectorf_set_scalar((*work).z, 0.0f64);
    OSQPVectorf_set_scalar((*work).y, 0.0f64);
    (*(*work).linsys_solver)
        .warm_start
        .expect("non-null function pointer")((*work).linsys_solver, (*work).x);
}
#[export_name = "honest_osqp_osqp_update_data_mat"]
pub unsafe extern "C" fn osqp_update_data_mat(
    mut solver: *mut OSQPSolver,
    mut Px_new: *const OSQPFloat,
    mut Px_new_idx: *const OSQPInt,
    mut P_new_n: OSQPInt,
    mut Ax_new: *const OSQPFloat,
    mut Ax_new_idx: *const OSQPInt,
    mut A_new_n: OSQPInt,
) -> OSQPInt {
    let mut exitflag: OSQPInt = 0;
    let mut nnzP: OSQPInt = 0;
    let mut nnzA: OSQPInt = 0;
    let mut work: *mut OSQPWorkspace = ::core::ptr::null_mut::<OSQPWorkspace>();
    if solver.is_null() || (*solver).work.is_null() {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_update_data_mat\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    work = (*solver).work;
    nnzP = OSQPMatrix_get_nz((*(*work).data).P);
    nnzA = OSQPMatrix_get_nz((*(*work).data).A);
    if P_new_n > nnzP || P_new_n < 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if Px_new_idx.is_null() && P_new_n != 0 as ::core::ffi::c_int && P_new_n != nnzP {
        return 1 as OSQPInt;
    }
    if P_new_n == 0 as ::core::ffi::c_int {
        P_new_n = nnzP;
    }
    if A_new_n > nnzA || A_new_n < 0 as ::core::ffi::c_int {
        return 2 as OSQPInt;
    }
    if Ax_new_idx.is_null() && A_new_n != 0 as ::core::ffi::c_int && A_new_n != nnzA {
        return 2 as OSQPInt;
    }
    if A_new_n == 0 as ::core::ffi::c_int {
        A_new_n = nnzA;
    }
    if (*(*solver).settings).scaling != 0 {
        unscale_data(solver);
    }
    if !Px_new.is_null() {
        OSQPMatrix_update_values((*(*work).data).P, Px_new, Px_new_idx, P_new_n);
    }
    if !Ax_new.is_null() {
        OSQPMatrix_update_values((*(*work).data).A, Ax_new, Ax_new_idx, A_new_n);
    }
    if (*(*solver).settings).scaling != 0 {
        scale_data(solver);
    }
    if (*(*solver).settings).scaling != 0 {
        exitflag = (*(*work).linsys_solver)
            .update_matrices
            .expect("non-null function pointer")(
            (*work).linsys_solver,
            (*(*work).data).P,
            ::core::ptr::null::<OSQPInt>(),
            nnzP,
            (*(*work).data).A,
            ::core::ptr::null::<OSQPInt>(),
            nnzA,
        );
    } else {
        exitflag = (*(*work).linsys_solver)
            .update_matrices
            .expect("non-null function pointer")(
            (*work).linsys_solver,
            (*(*work).data).P,
            Px_new_idx,
            P_new_n,
            (*(*work).data).A,
            Ax_new_idx,
            A_new_n,
        );
    }
    reset_info((*solver).info);
    exitflag != 0 as ::core::ffi::c_int;
    return exitflag;
}
#[export_name = "honest_osqp_osqp_update_rho"]
pub unsafe extern "C" fn osqp_update_rho(
    mut solver: *mut OSQPSolver,
    mut rho_new: OSQPFloat,
) -> OSQPInt {
    let mut exitflag: OSQPInt = 0;
    let mut work: *mut OSQPWorkspace = ::core::ptr::null_mut::<OSQPWorkspace>();
    if solver.is_null() || (*solver).work.is_null() {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_update_rho\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    work = (*solver).work;
    if rho_new <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return 1 as OSQPInt;
    }
    (*(*solver).settings).rho = (if (if rho_new > 1e-06f64 {
        rho_new as ::core::ffi::c_double
    } else {
        1e-06f64
    }) < 1e06f64
    {
        if rho_new > 1e-06f64 {
            rho_new as ::core::ffi::c_double
        } else {
            1e-06f64
        }
    } else {
        1e06f64
    }) as OSQPFloat;
    if (*(*solver).settings).rho_is_vec != 0 {
        OSQPVectorf_set_scalar_conditional(
            (*work).rho_vec,
            (*work).constr_type,
            OSQP_RHO_MIN,
            (*(*solver).settings).rho,
            OSQP_RHO_EQ_OVER_RHO_INEQ * (*(*solver).settings).rho,
        );
        OSQPVectorf_ew_reciprocal((*work).rho_inv_vec, (*work).rho_vec);
    } else {
        (*work).rho_inv = 1.0f64 / (*(*solver).settings).rho;
    }
    exitflag = (*(*work).linsys_solver)
        .update_rho_vec
        .expect("non-null function pointer")(
        (*work).linsys_solver,
        (*work).rho_vec,
        (*(*solver).settings).rho,
    );
    return exitflag;
}
#[export_name = "honest_osqp_osqp_update_settings"]
pub unsafe extern "C" fn osqp_update_settings(
    mut solver: *mut OSQPSolver,
    mut new_settings: *const OSQPSettings,
) -> OSQPInt {
    let mut settings: *mut OSQPSettings = (*solver).settings;
    if solver.is_null() || (*solver).work.is_null() {
        return _osqp_error(
            OSQP_WORKSPACE_NOT_INIT_ERROR,
            b"osqp_update_settings\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if validate_settings(new_settings, 0 as OSQPInt) != 0 {
        return _osqp_error(
            OSQP_SETTINGS_VALIDATION_ERROR,
            b"osqp_update_settings\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*settings).profiler_level = (*new_settings).profiler_level;
    (*settings).verbose = (*new_settings).verbose;
    (*settings).warm_starting = (*new_settings).warm_starting;
    (*settings).polishing = (*new_settings).polishing;
    (*settings).alpha = (*new_settings).alpha;
    (*settings).cg_max_iter = (*new_settings).cg_max_iter;
    (*settings).cg_tol_reduction = (*new_settings).cg_tol_reduction;
    (*settings).cg_tol_fraction = (*new_settings).cg_tol_fraction;
    (*settings).cg_precond = (*new_settings).cg_precond;
    (*settings).max_iter = (*new_settings).max_iter;
    (*settings).eps_abs = (*new_settings).eps_abs;
    (*settings).eps_rel = (*new_settings).eps_rel;
    (*settings).eps_prim_inf = (*new_settings).eps_prim_inf;
    (*settings).eps_dual_inf = (*new_settings).eps_dual_inf;
    (*settings).scaled_termination = (*new_settings).scaled_termination;
    (*settings).check_termination = (*new_settings).check_termination;
    (*settings).check_dualgap = (*new_settings).check_dualgap;
    (*settings).time_limit = (*new_settings).time_limit;
    (*settings).delta = (*new_settings).delta;
    (*settings).polish_refine_iter = (*new_settings).polish_refine_iter;
    (*(*(*solver).work).linsys_solver)
        .update_settings
        .expect("non-null function pointer")((*(*solver).work).linsys_solver, settings);
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_osqp_codegen"]
pub unsafe extern "C" fn osqp_codegen(
    mut solver: *mut OSQPSolver,
    mut output_dir: *const ::core::ffi::c_char,
    mut file_prefix: *const ::core::ffi::c_char,
    mut defines: *mut OSQPCodegenDefines,
) -> OSQPInt {
    let mut exitflag: OSQPInt = 0 as OSQPInt;
    exitflag = OSQP_FUNC_NOT_IMPLEMENTED as ::core::ffi::c_int as OSQPInt;
    return exitflag;
}
#[export_name = "honest_osqp_osqp_adjoint_derivative_compute"]
pub unsafe extern "C" fn osqp_adjoint_derivative_compute(
    mut solver: *mut OSQPSolver,
    mut dx: *mut OSQPFloat,
    mut dy: *mut OSQPFloat,
) -> OSQPInt {
    let mut status: OSQPInt = 0 as OSQPInt;
    status = OSQP_FUNC_NOT_IMPLEMENTED as ::core::ffi::c_int as OSQPInt;
    return status;
}
#[export_name = "honest_osqp_osqp_adjoint_derivative_get_mat"]
pub unsafe extern "C" fn osqp_adjoint_derivative_get_mat(
    mut solver: *mut OSQPSolver,
    mut dP: *mut OSQPCscMatrix,
    mut dA: *mut OSQPCscMatrix,
) -> OSQPInt {
    let mut status: OSQPInt = 0 as OSQPInt;
    status = OSQP_FUNC_NOT_IMPLEMENTED as ::core::ffi::c_int as OSQPInt;
    return status;
}
#[export_name = "honest_osqp_osqp_adjoint_derivative_get_vec"]
pub unsafe extern "C" fn osqp_adjoint_derivative_get_vec(
    mut solver: *mut OSQPSolver,
    mut dq: *mut OSQPFloat,
    mut dl: *mut OSQPFloat,
    mut du: *mut OSQPFloat,
) -> OSQPInt {
    let mut status: OSQPInt = 0 as OSQPInt;
    status = OSQP_FUNC_NOT_IMPLEMENTED as ::core::ffi::c_int as OSQPInt;
    return status;
}
