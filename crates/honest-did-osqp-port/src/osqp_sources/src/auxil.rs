#[repr(C)] pub struct OSQPVectorf_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPVectori_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPMatrix_ { _opaque: [u8; 0] }
use crate::runtime::{sqrt};
extern "C" {
    #[link_name = "honest_osqp_osqp_cold_start"]
    fn osqp_cold_start(solver: *mut OSQPSolver);
    #[link_name = "honest_osqp_osqp_update_rho"]
    fn osqp_update_rho(solver: *mut OSQPSolver, rho_new: OSQPFloat) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPVectorf_copy"]
    fn OSQPVectorf_copy(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_to_raw"]
    fn OSQPVectorf_to_raw(bv: *mut OSQPFloat, a: *const OSQPVectorf);
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
    #[link_name = "honest_osqp_OSQPVectorf_round_to_zero"]
    fn OSQPVectorf_round_to_zero(a: *mut OSQPVectorf, tol: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_mult_scalar"]
    fn OSQPVectorf_mult_scalar(a: *mut OSQPVectorf, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_plus"]
    fn OSQPVectorf_plus(x: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_minus"]
    fn OSQPVectorf_minus(x: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_add_scaled"]
    fn OSQPVectorf_add_scaled(
        x: *mut OSQPVectorf,
        sca: OSQPFloat,
        a: *const OSQPVectorf,
        scb: OSQPFloat,
        b: *const OSQPVectorf,
    );
    #[link_name = "honest_osqp_OSQPVectorf_add_scaled3"]
    fn OSQPVectorf_add_scaled3(
        x: *mut OSQPVectorf,
        sca: OSQPFloat,
        a: *const OSQPVectorf,
        scb: OSQPFloat,
        b: *const OSQPVectorf,
        scc: OSQPFloat,
        c: *const OSQPVectorf,
    );
    #[link_name = "honest_osqp_OSQPVectorf_norm_inf"]
    fn OSQPVectorf_norm_inf(v: *const OSQPVectorf) -> OSQPFloat;
    #[link_name = "honest_osqp_OSQPVectorf_scaled_norm_inf"]
    fn OSQPVectorf_scaled_norm_inf(S: *const OSQPVectorf, v: *const OSQPVectorf) -> OSQPFloat;
    #[link_name = "honest_osqp_OSQPVectorf_dot_prod"]
    fn OSQPVectorf_dot_prod(a: *const OSQPVectorf, b: *const OSQPVectorf) -> OSQPFloat;
    #[link_name = "honest_osqp_OSQPVectorf_dot_prod_signed"]
    fn OSQPVectorf_dot_prod_signed(
        a: *const OSQPVectorf,
        b: *const OSQPVectorf,
        sign: OSQPInt,
    ) -> OSQPFloat;
    #[link_name = "honest_osqp_OSQPVectorf_ew_prod"]
    fn OSQPVectorf_ew_prod(c: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_bound_vec"]
    fn OSQPVectorf_ew_bound_vec(
        x: *mut OSQPVectorf,
        z: *const OSQPVectorf,
        l: *const OSQPVectorf,
        u: *const OSQPVectorf,
    );
    #[link_name = "honest_osqp_OSQPVectorf_project_polar_reccone"]
    fn OSQPVectorf_project_polar_reccone(
        y: *mut OSQPVectorf,
        l: *const OSQPVectorf,
        u: *const OSQPVectorf,
        infval: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPVectorf_in_reccone"]
    fn OSQPVectorf_in_reccone(
        y: *const OSQPVectorf,
        l: *const OSQPVectorf,
        u: *const OSQPVectorf,
        infval: OSQPFloat,
        tol: OSQPFloat,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPVectorf_ew_reciprocal"]
    fn OSQPVectorf_ew_reciprocal(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_bounds_type"]
    fn OSQPVectorf_ew_bounds_type(
        iseq: *mut OSQPVectori,
        l: *const OSQPVectorf,
        u: *const OSQPVectorf,
        tol: OSQPFloat,
        infval: OSQPFloat,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPMatrix_Axpy"]
    fn OSQPMatrix_Axpy(
        A: *const OSQPMatrix,
        x: *const OSQPVectorf,
        y: *mut OSQPVectorf,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPMatrix_Atxpy"]
    fn OSQPMatrix_Atxpy(
        A: *const OSQPMatrix,
        x: *const OSQPVectorf,
        y: *mut OSQPVectorf,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_osqp_algebra_linsys_supported"]
    fn osqp_algebra_linsys_supported() -> OSQPInt;
    #[link_name = "honest_osqp_unscale_solution"]
    fn unscale_solution(
        usolx: *mut OSQPVectorf,
        usoly: *mut OSQPVectorf,
        solx: *const OSQPVectorf,
        soly: *const OSQPVectorf,
        work: *mut OSQPWorkspace,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_c_strcpy"]
    fn c_strcpy(dest: *mut ::core::ffi::c_char, source: *const ::core::ffi::c_char);
}
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
pub type osqp_linsys_solver_type = ::core::ffi::c_uint;
pub const OSQP_INDIRECT_SOLVER: osqp_linsys_solver_type = 2;
pub const OSQP_DIRECT_SOLVER: osqp_linsys_solver_type = 1;
pub const OSQP_UNKNOWN_SOLVER: osqp_linsys_solver_type = 0;
pub type osqp_precond_type = ::core::ffi::c_uint;
pub const OSQP_DIAGONAL_PRECONDITIONER: osqp_precond_type = 1;
pub const OSQP_NO_PRECONDITIONER: osqp_precond_type = 0;
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
pub const OSQP_RHO_MIN: ::core::ffi::c_double = 1e-06f64;
pub const OSQP_RHO_TOL: ::core::ffi::c_double = 1e-04f64;
pub const OSQP_RHO_EQ_OVER_RHO_INEQ: ::core::ffi::c_double = 1e03f64;
pub const OSQP_ADAPTIVE_RHO_UPDATE_TIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const _OSQP_ADAPTIVE_RHO_UPDATE_LAST_VALUE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OSQP_NAN: OSQPFloat = 0x7fc00000 as u64 as OSQPFloat;
pub const OSQP_INFTY: OSQPFloat = 1e30f64;
pub const OSQP_DIVISION_TOL: OSQPFloat = 1.0f64 / OSQP_INFTY;
pub const OSQP_MIN_SCALING: ::core::ffi::c_double = 1e-04f64;
pub const OSQP_ZERO_DEADZONE: ::core::ffi::c_double = 1e-15f64;
#[export_name = "honest_osqp_compute_rho_estimate"]
pub unsafe extern "C" fn compute_rho_estimate(mut solver: *const OSQPSolver) -> OSQPFloat {
    let mut prim_res: OSQPFloat = 0.;
    let mut dual_res: OSQPFloat = 0.;
    let mut prim_res_norm: OSQPFloat = 0.;
    let mut dual_res_norm: OSQPFloat = 0.;
    let mut temp_res_norm: OSQPFloat = 0.;
    let mut rho_estimate: OSQPFloat = 0.;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    prim_res = (*work).scaled_prim_res;
    dual_res = (*work).scaled_dual_res;
    prim_res_norm = OSQPVectorf_norm_inf((*work).z);
    temp_res_norm = OSQPVectorf_norm_inf((*work).Ax);
    prim_res_norm = (if prim_res_norm > temp_res_norm {
        prim_res_norm as ::core::ffi::c_double
    } else {
        temp_res_norm as ::core::ffi::c_double
    }) as OSQPFloat;
    prim_res /= (prim_res_norm + OSQP_DIVISION_TOL) as ::core::ffi::c_double;
    dual_res_norm = OSQPVectorf_norm_inf((*(*work).data).q);
    temp_res_norm = OSQPVectorf_norm_inf((*work).Aty);
    dual_res_norm = (if dual_res_norm > temp_res_norm {
        dual_res_norm as ::core::ffi::c_double
    } else {
        temp_res_norm as ::core::ffi::c_double
    }) as OSQPFloat;
    temp_res_norm = OSQPVectorf_norm_inf((*work).Px);
    dual_res_norm = (if dual_res_norm > temp_res_norm {
        dual_res_norm as ::core::ffi::c_double
    } else {
        temp_res_norm as ::core::ffi::c_double
    }) as OSQPFloat;
    dual_res /= (dual_res_norm + OSQP_DIVISION_TOL) as ::core::ffi::c_double;
    rho_estimate = ((*settings).rho as ::core::ffi::c_double
        * sqrt(prim_res as ::core::ffi::c_double / dual_res as ::core::ffi::c_double))
        as OSQPFloat;
    rho_estimate = (if (if rho_estimate > 1e-06f64 {
        rho_estimate as ::core::ffi::c_double
    } else {
        1e-06f64
    }) < 1e06f64
    {
        if rho_estimate > 1e-06f64 {
            rho_estimate as ::core::ffi::c_double
        } else {
            1e-06f64
        }
    } else {
        1e06f64
    }) as OSQPFloat;
    return rho_estimate;
}
#[export_name = "honest_osqp_adapt_rho"]
pub unsafe extern "C" fn adapt_rho(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut exitflag: OSQPInt = 0;
    let mut rho_new: OSQPFloat = 0.;
    let mut info: *mut OSQPInfo = (*solver).info;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    exitflag = 0 as ::core::ffi::c_int as OSQPInt;
    rho_new = compute_rho_estimate(solver);
    (*info).rho_estimate = rho_new;
    if rho_new > (*settings).rho * (*settings).adaptive_rho_tolerance
        || rho_new < (*settings).rho / (*settings).adaptive_rho_tolerance
    {
        exitflag = osqp_update_rho(solver, rho_new);
        (*info).rho_updates += 1 as ::core::ffi::c_int;
        (*(*solver).work).rho_updated = 1 as ::core::ffi::c_int as OSQPInt;
    }
    return exitflag;
}
#[export_name = "honest_osqp_set_rho_vec"]
pub unsafe extern "C" fn set_rho_vec(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut constr_types_changed: OSQPInt = 0 as OSQPInt;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    (*settings).rho = (if (if (*settings).rho > 1e-06f64 {
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
    constr_types_changed = OSQPVectorf_ew_bounds_type(
        (*work).constr_type,
        (*(*work).data).l,
        (*(*work).data).u,
        OSQP_RHO_TOL,
        OSQP_INFTY * OSQP_MIN_SCALING,
    );
    OSQPVectorf_set_scalar_conditional(
        (*work).rho_vec,
        (*work).constr_type,
        OSQP_RHO_MIN,
        (*settings).rho,
        OSQP_RHO_EQ_OVER_RHO_INEQ * (*settings).rho,
    );
    OSQPVectorf_ew_reciprocal((*work).rho_inv_vec, (*work).rho_vec);
    return constr_types_changed;
}
#[export_name = "honest_osqp_update_rho_vec"]
pub unsafe extern "C" fn update_rho_vec(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut constr_type_changed: OSQPInt = 0;
    let mut exitflag: OSQPInt = 0 as OSQPInt;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    constr_type_changed = set_rho_vec(solver);
    if constr_type_changed == 1 as ::core::ffi::c_int {
        exitflag = (*(*work).linsys_solver)
            .update_rho_vec
            .expect("non-null function pointer")(
            (*work).linsys_solver,
            (*work).rho_vec,
            (*(*solver).settings).rho,
        );
    }
    return exitflag;
}
#[export_name = "honest_osqp_swap_vectors"]
pub unsafe extern "C" fn swap_vectors(mut a: *mut *mut OSQPVectorf, mut b: *mut *mut OSQPVectorf) {
    let mut temp: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    temp = *b;
    *b = *a;
    *a = temp;
}
unsafe extern "C" fn compute_rhs(mut solver: *mut OSQPSolver) {
    let mut work: *mut OSQPWorkspace = (*solver).work;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    OSQPVectorf_add_scaled(
        (*work).xtilde_view,
        (*settings).sigma,
        (*work).x_prev,
        -1.0f64,
        (*(*work).data).q,
    );
    if (*settings).rho_is_vec != 0 {
        OSQPVectorf_ew_prod((*work).ztilde_view, (*work).rho_inv_vec, (*work).y);
        OSQPVectorf_add_scaled(
            (*work).ztilde_view,
            -1.0f64,
            (*work).ztilde_view,
            1.0f64,
            (*work).z_prev,
        );
    } else {
        OSQPVectorf_add_scaled(
            (*work).ztilde_view,
            1.0f64,
            (*work).z_prev,
            -(*work).rho_inv,
            (*work).y,
        );
    };
}
#[export_name = "honest_osqp_update_xz_tilde"]
pub unsafe extern "C" fn update_xz_tilde(mut solver: *mut OSQPSolver, mut admm_iter: OSQPInt) {
    let mut work: *mut OSQPWorkspace = (*solver).work;
    compute_rhs(solver);
    (*(*work).linsys_solver)
        .solve
        .expect("non-null function pointer")((*work).linsys_solver, (*work).xz_tilde, admm_iter);
}
#[export_name = "honest_osqp_update_x"]
pub unsafe extern "C" fn update_x(mut solver: *mut OSQPSolver) {
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    OSQPVectorf_add_scaled(
        (*work).x,
        (*settings).alpha,
        (*work).xtilde_view,
        1.0f64 - (*settings).alpha,
        (*work).x_prev,
    );
    OSQPVectorf_minus((*work).delta_x, (*work).x, (*work).x_prev);
}
#[export_name = "honest_osqp_update_z"]
pub unsafe extern "C" fn update_z(mut solver: *mut OSQPSolver) {
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    if (*settings).rho_is_vec != 0 {
        OSQPVectorf_ew_prod((*work).z, (*work).rho_inv_vec, (*work).y);
        OSQPVectorf_add_scaled3(
            (*work).z,
            1.0f64,
            (*work).z,
            (*settings).alpha,
            (*work).ztilde_view,
            1.0f64 - (*settings).alpha,
            (*work).z_prev,
        );
    } else {
        OSQPVectorf_add_scaled3(
            (*work).z,
            (*settings).alpha,
            (*work).ztilde_view,
            1.0f64 - (*settings).alpha,
            (*work).z_prev,
            (*work).rho_inv,
            (*work).y,
        );
    }
    OSQPVectorf_ew_bound_vec((*work).z, (*work).z, (*(*work).data).l, (*(*work).data).u);
}
#[export_name = "honest_osqp_update_y"]
pub unsafe extern "C" fn update_y(mut solver: *mut OSQPSolver) {
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    OSQPVectorf_add_scaled3(
        (*work).delta_y,
        (*settings).alpha,
        (*work).ztilde_view,
        1.0f64 - (*settings).alpha,
        (*work).z_prev,
        -1.0f64,
        (*work).z,
    );
    if (*settings).rho_is_vec != 0 {
        OSQPVectorf_ew_prod((*work).delta_y, (*work).delta_y, (*work).rho_vec);
    } else {
        OSQPVectorf_mult_scalar((*work).delta_y, (*settings).rho);
    }
    OSQPVectorf_plus((*work).y, (*work).y, (*work).delta_y);
}
#[export_name = "honest_osqp_compute_obj_val_dual_gap"]
pub unsafe extern "C" fn compute_obj_val_dual_gap(
    mut solver: *const OSQPSolver,
    mut x: *const OSQPVectorf,
    mut y: *const OSQPVectorf,
    mut prim_obj_val: *mut OSQPFloat,
    mut dual_obj_val: *mut OSQPFloat,
    mut duality_gap: *mut OSQPFloat,
) {
    let mut quad_term: OSQPFloat = 0.0f64;
    let mut lin_term: OSQPFloat = 0.0f64;
    let mut sup_term: OSQPFloat = 0.0f64;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    quad_term = OSQPVectorf_dot_prod((*work).Px, x);
    lin_term = OSQPVectorf_dot_prod((*(*work).data).q, x);
    OSQPVectorf_copy((*work).z_prev, y);
    OSQPVectorf_project_polar_reccone(
        (*work).z_prev,
        (*(*work).data).l,
        (*(*work).data).u,
        OSQP_INFTY * OSQP_MIN_SCALING,
    );
    OSQPVectorf_round_to_zero((*work).z_prev, OSQP_ZERO_DEADZONE);
    sup_term = OSQPVectorf_dot_prod_signed((*(*work).data).u, (*work).z_prev, 1 as OSQPInt);
    sup_term += OSQPVectorf_dot_prod_signed((*(*work).data).l, (*work).z_prev, -(1 as OSQPInt))
        as ::core::ffi::c_double;
    *prim_obj_val = 0.5f64 * quad_term + lin_term;
    *dual_obj_val = -0.5f64 * quad_term - sup_term;
    (*work).scaled_dual_gap = quad_term + lin_term + sup_term;
    if (*(*solver).settings).scaling != 0 {
        *prim_obj_val *= (*(*work).scaling).cinv as ::core::ffi::c_double;
        *dual_obj_val *= (*(*work).scaling).cinv as ::core::ffi::c_double;
        *duality_gap = (*(*work).scaling).cinv * (*work).scaled_dual_gap;
    } else {
        *duality_gap = (*work).scaled_dual_gap;
    }
    (*work).xtPx = quad_term;
    (*work).qtx = lin_term;
    (*work).SC = sup_term;
}
unsafe extern "C" fn compute_duality_gap_tol(
    mut solver: *const OSQPSolver,
    mut eps_abs: OSQPFloat,
    mut eps_rel: OSQPFloat,
) -> OSQPFloat {
    let mut max_rel_eps: OSQPFloat = 0.0f64;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    max_rel_eps = (if (*work).xtPx < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        -((*work).xtPx as ::core::ffi::c_double)
    } else {
        (*work).xtPx as ::core::ffi::c_double
    }) as OSQPFloat;
    max_rel_eps = (if max_rel_eps
        > (if (*work).qtx < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -((*work).qtx as ::core::ffi::c_double)
        } else {
            (*work).qtx as ::core::ffi::c_double
        }) {
        max_rel_eps as ::core::ffi::c_double
    } else if (*work).qtx < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        -((*work).qtx as ::core::ffi::c_double)
    } else {
        (*work).qtx as ::core::ffi::c_double
    }) as OSQPFloat;
    max_rel_eps = (if max_rel_eps
        > (if (*work).SC < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -((*work).SC as ::core::ffi::c_double)
        } else {
            (*work).SC as ::core::ffi::c_double
        }) {
        max_rel_eps as ::core::ffi::c_double
    } else if (*work).SC < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        -((*work).SC as ::core::ffi::c_double)
    } else {
        (*work).SC as ::core::ffi::c_double
    }) as OSQPFloat;
    if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
        max_rel_eps = (*(*work).scaling).cinv * max_rel_eps;
    }
    return eps_abs + eps_rel * max_rel_eps;
}
unsafe extern "C" fn compute_prim_res(
    mut solver: *mut OSQPSolver,
    mut x: *const OSQPVectorf,
    mut z: *const OSQPVectorf,
) -> OSQPFloat {
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    let mut prim_res: OSQPFloat = 0.;
    OSQPMatrix_Axpy((*(*work).data).A, x, (*work).Ax, 1.0f64, 0.0f64);
    OSQPVectorf_minus((*work).z_prev, (*work).Ax, z);
    (*work).scaled_prim_res = OSQPVectorf_norm_inf((*work).z_prev);
    if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
        prim_res = OSQPVectorf_scaled_norm_inf((*(*work).scaling).Einv, (*work).z_prev);
    } else {
        prim_res = (*work).scaled_prim_res;
    }
    return prim_res;
}
unsafe extern "C" fn compute_prim_tol(
    mut solver: *const OSQPSolver,
    mut eps_abs: OSQPFloat,
    mut eps_rel: OSQPFloat,
) -> OSQPFloat {
    let mut max_rel_eps: OSQPFloat = 0.;
    let mut temp_rel_eps: OSQPFloat = 0.;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
        max_rel_eps = OSQPVectorf_scaled_norm_inf((*(*work).scaling).Einv, (*work).z);
        temp_rel_eps = OSQPVectorf_scaled_norm_inf((*(*work).scaling).Einv, (*work).Ax);
        max_rel_eps = (if max_rel_eps > temp_rel_eps {
            max_rel_eps as ::core::ffi::c_double
        } else {
            temp_rel_eps as ::core::ffi::c_double
        }) as OSQPFloat;
    } else {
        max_rel_eps = OSQPVectorf_norm_inf((*work).z);
        temp_rel_eps = OSQPVectorf_norm_inf((*work).Ax);
        max_rel_eps = (if max_rel_eps > temp_rel_eps {
            max_rel_eps as ::core::ffi::c_double
        } else {
            temp_rel_eps as ::core::ffi::c_double
        }) as OSQPFloat;
    }
    return eps_abs + eps_rel * max_rel_eps;
}
unsafe extern "C" fn compute_dual_res(
    mut solver: *mut OSQPSolver,
    mut x: *const OSQPVectorf,
    mut y: *const OSQPVectorf,
) -> OSQPFloat {
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    let mut dual_res: OSQPFloat = 0.;
    OSQPVectorf_copy((*work).x_prev, (*(*work).data).q);
    OSQPMatrix_Axpy((*(*work).data).P, x, (*work).Px, 1.0f64, 0.0f64);
    OSQPVectorf_plus((*work).x_prev, (*work).x_prev, (*work).Px);
    if (*(*work).data).m != 0 {
        OSQPMatrix_Atxpy((*(*work).data).A, y, (*work).Aty, 1.0f64, 0.0f64);
        OSQPVectorf_plus((*work).x_prev, (*work).x_prev, (*work).Aty);
    }
    (*work).scaled_dual_res = OSQPVectorf_norm_inf((*work).x_prev);
    if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
        dual_res = (*(*work).scaling).cinv
            * OSQPVectorf_scaled_norm_inf((*(*work).scaling).Dinv, (*work).x_prev);
    } else {
        dual_res = (*work).scaled_dual_res;
    }
    return dual_res;
}
unsafe extern "C" fn compute_dual_tol(
    mut solver: *const OSQPSolver,
    mut eps_abs: OSQPFloat,
    mut eps_rel: OSQPFloat,
) -> OSQPFloat {
    let mut max_rel_eps: OSQPFloat = 0.;
    let mut temp_rel_eps: OSQPFloat = 0.;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
        max_rel_eps = OSQPVectorf_scaled_norm_inf((*(*work).scaling).Dinv, (*(*work).data).q);
        temp_rel_eps = OSQPVectorf_scaled_norm_inf((*(*work).scaling).Dinv, (*work).Aty);
        max_rel_eps = (if max_rel_eps > temp_rel_eps {
            max_rel_eps as ::core::ffi::c_double
        } else {
            temp_rel_eps as ::core::ffi::c_double
        }) as OSQPFloat;
        temp_rel_eps = OSQPVectorf_scaled_norm_inf((*(*work).scaling).Dinv, (*work).Px);
        max_rel_eps = (if max_rel_eps > temp_rel_eps {
            max_rel_eps as ::core::ffi::c_double
        } else {
            temp_rel_eps as ::core::ffi::c_double
        }) as OSQPFloat;
        max_rel_eps *= (*(*work).scaling).cinv as ::core::ffi::c_double;
    } else {
        max_rel_eps = OSQPVectorf_norm_inf((*(*work).data).q);
        temp_rel_eps = OSQPVectorf_norm_inf((*work).Aty);
        max_rel_eps = (if max_rel_eps > temp_rel_eps {
            max_rel_eps as ::core::ffi::c_double
        } else {
            temp_rel_eps as ::core::ffi::c_double
        }) as OSQPFloat;
        temp_rel_eps = OSQPVectorf_norm_inf((*work).Px);
        max_rel_eps = (if max_rel_eps > temp_rel_eps {
            max_rel_eps as ::core::ffi::c_double
        } else {
            temp_rel_eps as ::core::ffi::c_double
        }) as OSQPFloat;
    }
    return eps_abs + eps_rel * max_rel_eps;
}
#[export_name = "honest_osqp_is_primal_infeasible"]
pub unsafe extern "C" fn is_primal_infeasible(
    mut solver: *mut OSQPSolver,
    mut eps_prim_inf: OSQPFloat,
) -> OSQPInt {
    let mut norm_delta_y: OSQPFloat = 0.;
    let mut ineq_lhs: OSQPFloat = 0.0f64;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    OSQPVectorf_project_polar_reccone(
        (*work).delta_y,
        (*(*work).data).l,
        (*(*work).data).u,
        OSQP_INFTY * OSQP_MIN_SCALING,
    );
    if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
        OSQPVectorf_ew_prod((*work).Adelta_x, (*(*work).scaling).E, (*work).delta_y);
        norm_delta_y = OSQPVectorf_norm_inf((*work).Adelta_x);
    } else {
        norm_delta_y = OSQPVectorf_norm_inf((*work).delta_y);
    }
    if norm_delta_y > OSQP_DIVISION_TOL {
        ineq_lhs = OSQPVectorf_dot_prod_signed((*(*work).data).u, (*work).delta_y, 1 as OSQPInt);
        ineq_lhs += OSQPVectorf_dot_prod_signed((*(*work).data).l, (*work).delta_y, -(1 as OSQPInt))
            as ::core::ffi::c_double;
        if ineq_lhs < 0.0f64 {
            OSQPMatrix_Atxpy(
                (*(*work).data).A,
                (*work).delta_y,
                (*work).Atdelta_y,
                1.0f64,
                0.0f64,
            );
            if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
                OSQPVectorf_ew_prod(
                    (*work).Atdelta_y,
                    (*work).Atdelta_y,
                    (*(*work).scaling).Dinv,
                );
            }
            return (OSQPVectorf_norm_inf((*work).Atdelta_y) < eps_prim_inf * norm_delta_y)
                as ::core::ffi::c_int;
        }
    }
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_is_dual_infeasible"]
pub unsafe extern "C" fn is_dual_infeasible(
    mut solver: *mut OSQPSolver,
    mut eps_dual_inf: OSQPFloat,
) -> OSQPInt {
    let mut norm_delta_x: OSQPFloat = 0.;
    let mut cost_scaling: OSQPFloat = 0.;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
        norm_delta_x = OSQPVectorf_scaled_norm_inf((*(*work).scaling).D, (*work).delta_x);
        cost_scaling = (*(*work).scaling).c;
    } else {
        norm_delta_x = OSQPVectorf_norm_inf((*work).delta_x);
        cost_scaling = 1.0f64 as OSQPFloat;
    }
    if norm_delta_x > OSQP_DIVISION_TOL {
        if OSQPVectorf_dot_prod((*(*work).data).q, (*work).delta_x) < 0.0f64 {
            OSQPMatrix_Axpy(
                (*(*work).data).P,
                (*work).delta_x,
                (*work).Pdelta_x,
                1.0f64,
                0.0f64,
            );
            if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
                OSQPVectorf_ew_prod((*work).Pdelta_x, (*work).Pdelta_x, (*(*work).scaling).Dinv);
            }
            if OSQPVectorf_norm_inf((*work).Pdelta_x) < cost_scaling * eps_dual_inf * norm_delta_x {
                OSQPMatrix_Axpy(
                    (*(*work).data).A,
                    (*work).delta_x,
                    (*work).Adelta_x,
                    1.0f64,
                    0.0f64,
                );
                if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
                    OSQPVectorf_ew_prod(
                        (*work).Adelta_x,
                        (*work).Adelta_x,
                        (*(*work).scaling).Einv,
                    );
                }
                return OSQPVectorf_in_reccone(
                    (*work).Adelta_x,
                    (*(*work).data).l,
                    (*(*work).data).u,
                    OSQP_INFTY * OSQP_MIN_SCALING,
                    eps_dual_inf * norm_delta_x,
                );
            }
        }
    }
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_has_solution"]
pub unsafe extern "C" fn has_solution(mut info: *const OSQPInfo) -> OSQPInt {
    return ((*info).status_val != OSQP_PRIMAL_INFEASIBLE as ::core::ffi::c_int
        && (*info).status_val != OSQP_PRIMAL_INFEASIBLE_INACCURATE as ::core::ffi::c_int
        && (*info).status_val != OSQP_DUAL_INFEASIBLE as ::core::ffi::c_int
        && (*info).status_val != OSQP_DUAL_INFEASIBLE_INACCURATE as ::core::ffi::c_int
        && (*info).status_val != OSQP_NON_CVX as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[export_name = "honest_osqp_store_solution"]
pub unsafe extern "C" fn store_solution(
    mut solver: *mut OSQPSolver,
    mut solution: *mut OSQPSolution,
) {
    let mut norm_vec: OSQPFloat = 0.;
    let mut info: *mut OSQPInfo = (*solver).info;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    if solution.is_null() {
        return;
    }
    if has_solution(info) != 0 {
        if (*settings).scaling != 0 {
            unscale_solution((*work).x_prev, (*work).z_prev, (*work).x, (*work).y, work);
            OSQPVectorf_to_raw((*solution).x, (*work).x_prev);
            OSQPVectorf_to_raw((*solution).y, (*work).z_prev);
        } else {
            OSQPVectorf_to_raw((*solution).x, (*work).x);
            OSQPVectorf_to_raw((*solution).y, (*work).y);
        }
        OSQPVectorf_set_scalar((*work).delta_y, OSQP_NAN);
        OSQPVectorf_set_scalar((*work).delta_x, OSQP_NAN);
        OSQPVectorf_to_raw((*solution).prim_inf_cert, (*work).delta_y);
        OSQPVectorf_to_raw((*solution).dual_inf_cert, (*work).delta_x);
    } else {
        OSQPVectorf_set_scalar((*work).x, OSQP_NAN);
        OSQPVectorf_set_scalar((*work).y, OSQP_NAN);
        OSQPVectorf_to_raw((*solution).x, (*work).x);
        OSQPVectorf_to_raw((*solution).y, (*work).y);
        osqp_cold_start(solver);
        if (*info).status_val == OSQP_PRIMAL_INFEASIBLE as ::core::ffi::c_int
            || (*info).status_val == OSQP_PRIMAL_INFEASIBLE_INACCURATE as ::core::ffi::c_int
        {
            norm_vec = OSQPVectorf_norm_inf((*work).delta_y);
            OSQPVectorf_mult_scalar((*work).delta_y, 1.0f64 / norm_vec);
            OSQPVectorf_to_raw((*solution).prim_inf_cert, (*work).delta_y);
            OSQPVectorf_set_scalar((*work).delta_x, OSQP_NAN);
            OSQPVectorf_to_raw((*solution).dual_inf_cert, (*work).delta_x);
        }
        if (*info).status_val == OSQP_DUAL_INFEASIBLE as ::core::ffi::c_int
            || (*info).status_val == OSQP_DUAL_INFEASIBLE_INACCURATE as ::core::ffi::c_int
        {
            norm_vec = OSQPVectorf_norm_inf((*work).delta_x);
            OSQPVectorf_mult_scalar((*work).delta_x, 1.0f64 / norm_vec);
            OSQPVectorf_to_raw((*solution).dual_inf_cert, (*work).delta_x);
            OSQPVectorf_set_scalar((*work).delta_y, OSQP_NAN);
            OSQPVectorf_to_raw((*solution).prim_inf_cert, (*work).delta_y);
        }
    };
}
#[export_name = "honest_osqp_update_info"]
pub unsafe extern "C" fn update_info(
    mut solver: *mut OSQPSolver,
    mut iter: OSQPInt,
    mut polishing: OSQPInt,
) {
    let mut x: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut z: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut y: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut prim_obj_val: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut dual_obj_val: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut dual_gap: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut prim_res: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut dual_res: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut info: *mut OSQPInfo = (*solver).info;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    if polishing != 0 {
        x = (*(*work).pol).x;
        y = (*(*work).pol).y;
        z = (*(*work).pol).z;
        prim_obj_val = &raw mut (*(*work).pol).obj_val;
        dual_obj_val = &raw mut (*(*work).pol).dual_obj_val;
        dual_gap = &raw mut (*(*work).pol).duality_gap;
        prim_res = &raw mut (*(*work).pol).prim_res;
        dual_res = &raw mut (*(*work).pol).dual_res;
    } else {
        x = (*work).x;
        y = (*work).y;
        z = (*work).z;
        prim_obj_val = &raw mut (*info).obj_val;
        dual_obj_val = &raw mut (*info).dual_obj_val;
        dual_gap = &raw mut (*info).duality_gap;
        prim_res = &raw mut (*info).prim_res;
        dual_res = &raw mut (*info).dual_res;
        (*info).iter = iter;
    }
    if (*(*work).data).m == 0 as ::core::ffi::c_int {
        *prim_res = 0.0f64 as OSQPFloat;
    } else {
        *prim_res = compute_prim_res(solver, x, z);
    }
    *dual_res = compute_dual_res(solver, x, y);
    compute_obj_val_dual_gap(solver, x, y, prim_obj_val, dual_obj_val, dual_gap);
    if polishing == 0 {
        (*info).primdual_int += if *dual_gap < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -*dual_gap
        } else {
            *dual_gap
        };
    }
    (*info).rel_kkt_error = (if (if *dual_res > *prim_res {
        *dual_res
    } else {
        *prim_res
    }) > *dual_gap
    {
        if *dual_res > *prim_res {
            *dual_res
        } else {
            *prim_res
        }
    } else {
        *dual_gap
    }) as OSQPFloat;
}
#[export_name = "honest_osqp_reset_info"]
pub unsafe extern "C" fn reset_info(mut info: *mut OSQPInfo) {
    update_status(info, OSQP_UNSOLVED as ::core::ffi::c_int as OSQPInt);
    (*info).rho_updates = 0 as ::core::ffi::c_int as OSQPInt;
}
#[export_name = "honest_osqp_OSQP_STATUS_MESSAGE"]
pub static mut OSQP_STATUS_MESSAGE: [*const ::core::ffi::c_char; 12] = [
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"solved\0" as *const u8 as *const ::core::ffi::c_char,
    b"solved inaccurate\0" as *const u8 as *const ::core::ffi::c_char,
    b"primal infeasible\0" as *const u8 as *const ::core::ffi::c_char,
    b"primal infeasible inaccurate\0" as *const u8 as *const ::core::ffi::c_char,
    b"dual infeasible\0" as *const u8 as *const ::core::ffi::c_char,
    b"dual infeasible inaccurate\0" as *const u8 as *const ::core::ffi::c_char,
    b"maximum iterations reached\0" as *const u8 as *const ::core::ffi::c_char,
    b"run time limit reached\0" as *const u8 as *const ::core::ffi::c_char,
    b"problem non convex\0" as *const u8 as *const ::core::ffi::c_char,
    b"interrupted\0" as *const u8 as *const ::core::ffi::c_char,
    b"unsolved\0" as *const u8 as *const ::core::ffi::c_char,
];
#[export_name = "honest_osqp_update_status"]
pub unsafe extern "C" fn update_status(mut info: *mut OSQPInfo, mut status_val: OSQPInt) {
    (*info).status_val = status_val;
    c_strcpy(
        &raw mut (*info).status as *mut ::core::ffi::c_char,
        OSQP_STATUS_MESSAGE[status_val as usize],
    );
}
#[export_name = "honest_osqp_check_termination"]
pub unsafe extern "C" fn check_termination(
    mut solver: *mut OSQPSolver,
    mut approximate: OSQPInt,
) -> OSQPInt {
    let mut eps_prim: OSQPFloat = 0.;
    let mut eps_dual: OSQPFloat = 0.;
    let mut eps_duality_gap: OSQPFloat = 0.;
    let mut eps_prim_inf: OSQPFloat = 0.;
    let mut eps_dual_inf: OSQPFloat = 0.;
    let mut exitflag: OSQPInt = 0;
    let mut prim_res_check: OSQPInt = 0;
    let mut dual_res_check: OSQPInt = 0;
    let mut duality_gap_check: OSQPInt = 0;
    let mut prim_inf_check: OSQPInt = 0;
    let mut dual_inf_check: OSQPInt = 0;
    let mut eps_abs: OSQPFloat = 0.;
    let mut eps_rel: OSQPFloat = 0.;
    let mut info: *mut OSQPInfo = (*solver).info;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    exitflag = 0 as ::core::ffi::c_int as OSQPInt;
    prim_res_check = 0 as ::core::ffi::c_int as OSQPInt;
    dual_res_check = 0 as ::core::ffi::c_int as OSQPInt;
    prim_inf_check = 0 as ::core::ffi::c_int as OSQPInt;
    dual_inf_check = 0 as ::core::ffi::c_int as OSQPInt;
    duality_gap_check = 0 as ::core::ffi::c_int as OSQPInt;
    eps_abs = (*settings).eps_abs;
    eps_rel = (*settings).eps_rel;
    eps_prim_inf = (*settings).eps_prim_inf;
    eps_dual_inf = (*settings).eps_dual_inf;
    if (*info).prim_res > OSQP_INFTY || (*info).dual_res > OSQP_INFTY {
        update_status(info, OSQP_NON_CVX as ::core::ffi::c_int as OSQPInt);
        (*info).obj_val = OSQP_NAN;
        return 1 as OSQPInt;
    }
    if approximate != 0 {
        eps_abs *= 10 as ::core::ffi::c_int as ::core::ffi::c_double;
        eps_rel *= 10 as ::core::ffi::c_int as ::core::ffi::c_double;
        eps_prim_inf *= 10 as ::core::ffi::c_int as ::core::ffi::c_double;
        eps_dual_inf *= 10 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if (*(*work).data).m == 0 as ::core::ffi::c_int {
        prim_res_check = 1 as ::core::ffi::c_int as OSQPInt;
    } else {
        eps_prim = compute_prim_tol(solver, eps_abs, eps_rel);
        if (*info).prim_res < eps_prim {
            prim_res_check = 1 as ::core::ffi::c_int as OSQPInt;
        } else {
            prim_inf_check = is_primal_infeasible(solver, eps_prim_inf);
        }
    }
    eps_dual = compute_dual_tol(solver, eps_abs, eps_rel);
    if (*info).dual_res < eps_dual {
        dual_res_check = 1 as ::core::ffi::c_int as OSQPInt;
    } else {
        dual_inf_check = is_dual_infeasible(solver, eps_dual_inf);
    }
    if (*settings).check_dualgap != 0 {
        eps_duality_gap = compute_duality_gap_tol(solver, eps_abs, eps_rel);
        if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
            if (if (*info).duality_gap < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -((*info).duality_gap as ::core::ffi::c_double)
            } else {
                (*info).duality_gap as ::core::ffi::c_double
            }) < eps_duality_gap
            {
                duality_gap_check = 1 as ::core::ffi::c_int as OSQPInt;
            }
        } else if (if (*work).scaled_dual_gap < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -((*work).scaled_dual_gap as ::core::ffi::c_double)
        } else {
            (*work).scaled_dual_gap as ::core::ffi::c_double
        }) < eps_duality_gap
        {
            duality_gap_check = 1 as ::core::ffi::c_int as OSQPInt;
        }
    } else {
        duality_gap_check = 1 as ::core::ffi::c_int as OSQPInt;
    }
    if prim_res_check != 0 && dual_res_check != 0 && duality_gap_check != 0 {
        if approximate != 0 {
            update_status(
                info,
                OSQP_SOLVED_INACCURATE as ::core::ffi::c_int as OSQPInt,
            );
        } else {
            update_status(info, OSQP_SOLVED as ::core::ffi::c_int as OSQPInt);
        }
        exitflag = 1 as ::core::ffi::c_int as OSQPInt;
    } else if prim_inf_check != 0 {
        if approximate != 0 {
            update_status(
                info,
                OSQP_PRIMAL_INFEASIBLE_INACCURATE as ::core::ffi::c_int as OSQPInt,
            );
        } else {
            update_status(
                info,
                OSQP_PRIMAL_INFEASIBLE as ::core::ffi::c_int as OSQPInt,
            );
        }
        if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
            OSQPVectorf_ew_prod((*work).delta_y, (*work).delta_y, (*(*work).scaling).E);
        }
        (*info).obj_val = OSQP_INFTY;
        exitflag = 1 as ::core::ffi::c_int as OSQPInt;
    } else if dual_inf_check != 0 {
        if approximate != 0 {
            update_status(
                info,
                OSQP_DUAL_INFEASIBLE_INACCURATE as ::core::ffi::c_int as OSQPInt,
            );
        } else {
            update_status(info, OSQP_DUAL_INFEASIBLE as ::core::ffi::c_int as OSQPInt);
        }
        if (*settings).scaling != 0 && (*settings).scaled_termination == 0 {
            OSQPVectorf_ew_prod((*work).delta_x, (*work).delta_x, (*(*work).scaling).D);
        }
        (*info).obj_val = -OSQP_INFTY;
        exitflag = 1 as ::core::ffi::c_int as OSQPInt;
    }
    return exitflag;
}
#[export_name = "honest_osqp_validate_data"]
pub unsafe extern "C" fn validate_data(
    mut P: *const OSQPCscMatrix,
    mut q: *const OSQPFloat,
    mut A: *const OSQPCscMatrix,
    mut l: *const OSQPFloat,
    mut u: *const OSQPFloat,
    mut m: OSQPInt,
    mut n: OSQPInt,
) -> OSQPInt {
    let mut j: OSQPInt = 0;
    let mut ptr: OSQPInt = 0;
    if P.is_null() {
        return 1 as OSQPInt;
    }
    if A.is_null() {
        return 1 as OSQPInt;
    }
    if q.is_null() {
        return 1 as OSQPInt;
    }
    if n <= 0 as ::core::ffi::c_int || m < 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if (*P).m != n {
        return 1 as OSQPInt;
    }
    if (*P).m != (*P).n {
        return 1 as OSQPInt;
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        ptr = *(*P).p.offset(j as isize);
        while ptr
            < *(*P)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        {
            if *(*P).i.offset(ptr as isize) > j {
                return 1 as OSQPInt;
            }
            ptr += 1;
        }
        j += 1;
    }
    if (*A).m != m || (*A).n != n {
        return 1 as OSQPInt;
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < m {
        if *l.offset(j as isize) > *u.offset(j as isize) {
            return 1 as OSQPInt;
        }
        j += 1;
    }
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_validate_linsys_solver"]
pub unsafe extern "C" fn validate_linsys_solver(mut linsys_solver: OSQPInt) -> OSQPInt {
    if linsys_solver == OSQP_INDIRECT_SOLVER as ::core::ffi::c_int
        && osqp_algebra_linsys_supported() as ::core::ffi::c_int
            & OSQP_CAPABILITY_INDIRECT_SOLVER as ::core::ffi::c_int
            != 0
    {
        return 0 as OSQPInt;
    }
    if linsys_solver == OSQP_DIRECT_SOLVER as ::core::ffi::c_int
        && osqp_algebra_linsys_supported() as ::core::ffi::c_int
            & OSQP_CAPABILITY_DIRECT_SOLVER as ::core::ffi::c_int
            != 0
    {
        return 0 as OSQPInt;
    }
    return 1 as OSQPInt;
}
#[export_name = "honest_osqp_validate_settings"]
pub unsafe extern "C" fn validate_settings(
    mut settings: *const OSQPSettings,
    mut from_setup: OSQPInt,
) -> OSQPInt {
    if settings.is_null() {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && validate_linsys_solver((*settings).linsys_solver as OSQPInt) != 0 {
        return 1 as OSQPInt;
    }
    if from_setup != 0
        && (*settings).allocate_solution != 0 as ::core::ffi::c_int
        && (*settings).allocate_solution != 1 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if (*settings).verbose != 0 as ::core::ffi::c_int
        && (*settings).verbose != 1 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if (*settings).profiler_level != 0 as ::core::ffi::c_int
        && (*settings).profiler_level != 1 as ::core::ffi::c_int
        && (*settings).profiler_level != 2 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if (*settings).warm_starting != 0 as ::core::ffi::c_int
        && (*settings).warm_starting != 1 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && (*settings).scaling < 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if (*settings).polishing != 0 as ::core::ffi::c_int
        && (*settings).polishing != 1 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && (*settings).rho <= 0.0f64 {
        return 1 as OSQPInt;
    }
    if from_setup != 0
        && (*settings).rho_is_vec != 0 as ::core::ffi::c_int
        && (*settings).rho_is_vec != 1 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && (*settings).sigma <= 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).alpha <= 0.0f64 || (*settings).alpha >= 2.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).cg_max_iter <= 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if (*settings).cg_tol_reduction <= 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if (*settings).cg_tol_fraction <= 0.0f64 || (*settings).cg_tol_fraction >= 1.0f64 {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && (*settings).adaptive_rho < 0 as ::core::ffi::c_int
        || (*settings).adaptive_rho >= _OSQP_ADAPTIVE_RHO_UPDATE_LAST_VALUE
    {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && (*settings).adaptive_rho == OSQP_ADAPTIVE_RHO_UPDATE_TIME {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && (*settings).adaptive_rho_interval < 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if from_setup != 0
        && (*settings).adaptive_rho_fraction <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        return 1 as OSQPInt;
    }
    if from_setup != 0 && (*settings).adaptive_rho_tolerance < 1.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).max_iter <= 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if (*settings).eps_abs < 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).eps_rel < 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).eps_rel == 0.0f64 && (*settings).eps_abs == 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).eps_prim_inf <= 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).eps_dual_inf <= 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).scaled_termination != 0 as ::core::ffi::c_int
        && (*settings).scaled_termination != 1 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if (*settings).check_termination < 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    if (*settings).check_dualgap != 0 as ::core::ffi::c_int
        && (*settings).check_dualgap != 1 as ::core::ffi::c_int
    {
        return 1 as OSQPInt;
    }
    if (*settings).time_limit <= 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).delta <= 0.0f64 {
        return 1 as OSQPInt;
    }
    if (*settings).polish_refine_iter < 0 as ::core::ffi::c_int {
        return 1 as OSQPInt;
    }
    return 0 as OSQPInt;
}
