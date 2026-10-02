#[repr(C)] pub struct OSQPVectorf_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPVectori_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPMatrix_ { _opaque: [u8; 0] }
extern "C" {
    #[link_name = "honest_osqp_OSQPVectorf_set_scalar"]
    fn OSQPVectorf_set_scalar(a: *mut OSQPVectorf, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_mult_scalar"]
    fn OSQPVectorf_mult_scalar(a: *mut OSQPVectorf, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_norm_inf"]
    fn OSQPVectorf_norm_inf(v: *const OSQPVectorf) -> OSQPFloat;
    #[link_name = "honest_osqp_OSQPVectorf_norm_1"]
    fn OSQPVectorf_norm_1(a: *const OSQPVectorf) -> OSQPFloat;
    #[link_name = "honest_osqp_OSQPVectorf_ew_prod"]
    fn OSQPVectorf_ew_prod(c: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_reciprocal"]
    fn OSQPVectorf_ew_reciprocal(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_sqrt"]
    fn OSQPVectorf_ew_sqrt(a: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_max_vec"]
    fn OSQPVectorf_ew_max_vec(c: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_set_scalar_if_lt"]
    fn OSQPVectorf_set_scalar_if_lt(
        x: *mut OSQPVectorf,
        z: *const OSQPVectorf,
        testval: OSQPFloat,
        newval: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPVectorf_set_scalar_if_gt"]
    fn OSQPVectorf_set_scalar_if_gt(
        x: *mut OSQPVectorf,
        z: *const OSQPVectorf,
        testval: OSQPFloat,
        newval: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPMatrix_mult_scalar"]
    fn OSQPMatrix_mult_scalar(A: *mut OSQPMatrix, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPMatrix_lmult_diag"]
    fn OSQPMatrix_lmult_diag(A: *mut OSQPMatrix, L: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPMatrix_rmult_diag"]
    fn OSQPMatrix_rmult_diag(A: *mut OSQPMatrix, R: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPMatrix_col_norm_inf"]
    fn OSQPMatrix_col_norm_inf(M: *const OSQPMatrix, E: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPMatrix_row_norm_inf"]
    fn OSQPMatrix_row_norm_inf(M: *const OSQPMatrix, E: *mut OSQPVectorf);
}
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
pub const OSQP_MIN_SCALING: ::core::ffi::c_double = 1e-04f64;
pub const OSQP_MAX_SCALING: ::core::ffi::c_double = 1e+04f64;
#[export_name = "honest_osqp_limit_scaling_scalar"]
pub unsafe extern "C" fn limit_scaling_scalar(mut v: OSQPFloat) -> OSQPFloat {
    v = (if v < OSQP_MIN_SCALING {
        1.0f64
    } else {
        v as ::core::ffi::c_double
    }) as OSQPFloat;
    v = (if v > OSQP_MAX_SCALING {
        OSQP_MAX_SCALING
    } else {
        v as ::core::ffi::c_double
    }) as OSQPFloat;
    return v;
}
#[export_name = "honest_osqp_limit_scaling_vector"]
pub unsafe extern "C" fn limit_scaling_vector(mut v: *mut OSQPVectorf) {
    OSQPVectorf_set_scalar_if_lt(v, v, OSQP_MIN_SCALING, 1.0f64);
    OSQPVectorf_set_scalar_if_gt(v, v, OSQP_MAX_SCALING, OSQP_MAX_SCALING);
}
#[export_name = "honest_osqp_compute_inf_norm_cols_KKT"]
pub unsafe extern "C" fn compute_inf_norm_cols_KKT(
    mut P: *const OSQPMatrix,
    mut A: *const OSQPMatrix,
    mut D: *mut OSQPVectorf,
    mut D_temp_A: *mut OSQPVectorf,
    mut E: *mut OSQPVectorf,
) {
    OSQPMatrix_col_norm_inf(P, D);
    OSQPMatrix_col_norm_inf(A, D_temp_A);
    OSQPVectorf_ew_max_vec(D, D_temp_A, D);
    OSQPMatrix_row_norm_inf(A, E);
}
#[export_name = "honest_osqp_scale_data"]
pub unsafe extern "C" fn scale_data(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut i: OSQPInt = 0;
    let mut n: OSQPInt = 0;
    let mut c_temp: OSQPFloat = 0.;
    let mut inf_norm_q: OSQPFloat = 0.;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    n = (*(*work).data).n;
    (*(*work).scaling).c = 1.0f64 as OSQPFloat;
    OSQPVectorf_set_scalar((*(*work).scaling).D, 1.0f64);
    OSQPVectorf_set_scalar((*(*work).scaling).Dinv, 1.0f64);
    OSQPVectorf_set_scalar((*(*work).scaling).E, 1.0f64);
    OSQPVectorf_set_scalar((*(*work).scaling).Einv, 1.0f64);
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < (*settings).scaling {
        compute_inf_norm_cols_KKT(
            (*(*work).data).P,
            (*(*work).data).A,
            (*work).D_temp,
            (*work).D_temp_A,
            (*work).E_temp,
        );
        limit_scaling_vector((*work).D_temp);
        limit_scaling_vector((*work).E_temp);
        OSQPVectorf_ew_sqrt((*work).D_temp);
        OSQPVectorf_ew_sqrt((*work).E_temp);
        OSQPVectorf_ew_reciprocal((*work).D_temp, (*work).D_temp);
        OSQPVectorf_ew_reciprocal((*work).E_temp, (*work).E_temp);
        OSQPMatrix_lmult_diag((*(*work).data).P, (*work).D_temp);
        OSQPMatrix_rmult_diag((*(*work).data).P, (*work).D_temp);
        OSQPMatrix_lmult_diag((*(*work).data).A, (*work).E_temp);
        OSQPMatrix_rmult_diag((*(*work).data).A, (*work).D_temp);
        OSQPVectorf_ew_prod((*(*work).data).q, (*(*work).data).q, (*work).D_temp);
        OSQPVectorf_ew_prod((*(*work).scaling).D, (*(*work).scaling).D, (*work).D_temp);
        OSQPVectorf_ew_prod((*(*work).scaling).E, (*(*work).scaling).E, (*work).E_temp);
        OSQPMatrix_col_norm_inf((*(*work).data).P, (*work).D_temp);
        c_temp = OSQPVectorf_norm_1((*work).D_temp);
        c_temp = (c_temp as ::core::ffi::c_double / n as ::core::ffi::c_double) as OSQPFloat;
        inf_norm_q = OSQPVectorf_norm_inf((*(*work).data).q);
        inf_norm_q = limit_scaling_scalar(inf_norm_q);
        c_temp = (if c_temp > inf_norm_q {
            c_temp as ::core::ffi::c_double
        } else {
            inf_norm_q as ::core::ffi::c_double
        }) as OSQPFloat;
        c_temp = limit_scaling_scalar(c_temp);
        c_temp = 1.0f64 / c_temp;
        OSQPMatrix_mult_scalar((*(*work).data).P, c_temp);
        OSQPVectorf_mult_scalar((*(*work).data).q, c_temp);
        (*(*work).scaling).c *= c_temp as ::core::ffi::c_double;
        i += 1;
    }
    (*(*work).scaling).cinv = 1.0f64 / (*(*work).scaling).c;
    OSQPVectorf_ew_reciprocal((*(*work).scaling).Dinv, (*(*work).scaling).D);
    OSQPVectorf_ew_reciprocal((*(*work).scaling).Einv, (*(*work).scaling).E);
    OSQPVectorf_ew_prod((*(*work).data).l, (*(*work).data).l, (*(*work).scaling).E);
    OSQPVectorf_ew_prod((*(*work).data).u, (*(*work).data).u, (*(*work).scaling).E);
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_unscale_data"]
pub unsafe extern "C" fn unscale_data(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut work: *mut OSQPWorkspace = (*solver).work;
    OSQPMatrix_mult_scalar((*(*work).data).P, (*(*work).scaling).cinv);
    OSQPMatrix_lmult_diag((*(*work).data).P, (*(*work).scaling).Dinv);
    OSQPMatrix_rmult_diag((*(*work).data).P, (*(*work).scaling).Dinv);
    OSQPVectorf_mult_scalar((*(*work).data).q, (*(*work).scaling).cinv);
    OSQPVectorf_ew_prod(
        (*(*work).data).q,
        (*(*work).data).q,
        (*(*work).scaling).Dinv,
    );
    OSQPMatrix_lmult_diag((*(*work).data).A, (*(*work).scaling).Einv);
    OSQPMatrix_rmult_diag((*(*work).data).A, (*(*work).scaling).Dinv);
    OSQPVectorf_ew_prod(
        (*(*work).data).l,
        (*(*work).data).l,
        (*(*work).scaling).Einv,
    );
    OSQPVectorf_ew_prod(
        (*(*work).data).u,
        (*(*work).data).u,
        (*(*work).scaling).Einv,
    );
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_unscale_solution"]
pub unsafe extern "C" fn unscale_solution(
    mut usolx: *mut OSQPVectorf,
    mut usoly: *mut OSQPVectorf,
    mut solx: *const OSQPVectorf,
    mut soly: *const OSQPVectorf,
    mut work: *mut OSQPWorkspace,
) -> OSQPInt {
    OSQPVectorf_ew_prod(usolx, solx, (*(*work).scaling).D);
    OSQPVectorf_ew_prod(usoly, soly, (*(*work).scaling).E);
    OSQPVectorf_mult_scalar(usoly, (*(*work).scaling).cinv);
    return 0 as OSQPInt;
}
