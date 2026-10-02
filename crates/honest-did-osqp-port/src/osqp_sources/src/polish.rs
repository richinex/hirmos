#[repr(C)] pub struct OSQPVectorf_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPVectori_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPMatrix_ { _opaque: [u8; 0] }
use crate::runtime::{malloc,free};
extern "C" {
    #[link_name = "honest_osqp_OSQPVectorf_malloc"]
    fn OSQPVectorf_malloc(length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_copy_new"]
    fn OSQPVectorf_copy_new(a: *const OSQPVectorf) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_free"]
    fn OSQPVectorf_free(a: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_view"]
    fn OSQPVectorf_view(a: *const OSQPVectorf, head: OSQPInt, length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_view_free"]
    fn OSQPVectorf_view_free(a: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_length"]
    fn OSQPVectorf_length(a: *const OSQPVectorf) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPVectorf_copy"]
    fn OSQPVectorf_copy(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_from_raw"]
    fn OSQPVectorf_from_raw(b: *mut OSQPVectorf, a: *const OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectori_from_raw"]
    fn OSQPVectori_from_raw(b: *mut OSQPVectori, a: *const OSQPInt);
    #[link_name = "honest_osqp_OSQPVectorf_to_raw"]
    fn OSQPVectorf_to_raw(bv: *mut OSQPFloat, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectori_to_raw"]
    fn OSQPVectori_to_raw(bv: *mut OSQPInt, a: *const OSQPVectori);
    #[link_name = "honest_osqp_OSQPVectorf_set_scalar"]
    fn OSQPVectorf_set_scalar(a: *mut OSQPVectorf, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_plus"]
    fn OSQPVectorf_plus(x: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_minus"]
    fn OSQPVectorf_minus(x: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_bound_vec"]
    fn OSQPVectorf_ew_bound_vec(
        x: *mut OSQPVectorf,
        z: *const OSQPVectorf,
        l: *const OSQPVectorf,
        u: *const OSQPVectorf,
    );
    #[link_name = "honest_osqp_OSQPMatrix_get_m"]
    fn OSQPMatrix_get_m(M: *const OSQPMatrix) -> OSQPInt;
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
    #[link_name = "honest_osqp_OSQPMatrix_free"]
    fn OSQPMatrix_free(M: *mut OSQPMatrix);
    #[link_name = "honest_osqp_OSQPMatrix_submatrix_byrows"]
    fn OSQPMatrix_submatrix_byrows(
        A: *const OSQPMatrix,
        rows: *const OSQPVectori,
    ) -> *mut OSQPMatrix;
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
    #[link_name = "honest_osqp_update_info"]
    fn update_info(solver: *mut OSQPSolver, iter: OSQPInt, polishing: OSQPInt);
    #[link_name = "honest_osqp__osqp_error"]
    fn _osqp_error(
        error_code: osqp_error_type,
        function_name: *const ::core::ffi::c_char,
    ) -> OSQPInt;
}
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
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn form_Ared(mut work: *mut OSQPWorkspace) -> OSQPInt {
    let mut j: OSQPInt = 0;
    let mut n_active: OSQPInt = 0;
    let mut m: OSQPInt = (*(*work).data).m;
    let mut active_flags: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut z: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut y: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut u: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut l: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    active_flags = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t))
        as *mut OSQPInt;
    z = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    y = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    l = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    u = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    if active_flags.is_null() || z.is_null() || y.is_null() || l.is_null() || u.is_null() {
        free(active_flags as *mut ::core::ffi::c_void);
        free(z as *mut ::core::ffi::c_void);
        free(y as *mut ::core::ffi::c_void);
        free(l as *mut ::core::ffi::c_void);
        free(u as *mut ::core::ffi::c_void);
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"form_Ared\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    OSQPVectori_to_raw(active_flags, (*(*work).pol).active_flags);
    OSQPVectorf_to_raw(z, (*work).z);
    OSQPVectorf_to_raw(y, (*work).y);
    OSQPVectorf_to_raw(l, (*(*work).data).l);
    OSQPVectorf_to_raw(u, (*(*work).data).u);
    n_active = 0 as ::core::ffi::c_int as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*(*work).data).m {
        if *z.offset(j as isize) - *l.offset(j as isize) < -*y.offset(j as isize)
            || *l.offset(j as isize) == *u.offset(j as isize)
        {
            *active_flags.offset(j as isize) = -(1 as ::core::ffi::c_int) as OSQPInt;
            n_active += 1;
        } else if *u.offset(j as isize) - *z.offset(j as isize) < *y.offset(j as isize) {
            *active_flags.offset(j as isize) = 1 as ::core::ffi::c_int as OSQPInt;
            n_active += 1;
        } else {
            *active_flags.offset(j as isize) = 0 as ::core::ffi::c_int as OSQPInt;
        }
        j += 1;
    }
    OSQPVectori_from_raw((*(*work).pol).active_flags, active_flags);
    (*(*work).pol).n_active = n_active;
    (*(*work).pol).Ared =
        OSQPMatrix_submatrix_byrows((*(*work).data).A, (*(*work).pol).active_flags);
    free(active_flags as *mut ::core::ffi::c_void);
    free(z as *mut ::core::ffi::c_void);
    free(y as *mut ::core::ffi::c_void);
    free(l as *mut ::core::ffi::c_void);
    free(u as *mut ::core::ffi::c_void);
    if (*(*work).pol).Ared.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"form_Ared\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return OSQP_NO_ERROR as ::core::ffi::c_int as OSQPInt;
}
unsafe extern "C" fn form_rhs_red(
    mut work: *mut OSQPWorkspace,
    mut rhs: *mut OSQPVectorf,
) -> OSQPInt {
    let mut j: OSQPInt = 0;
    let mut counter: OSQPInt = 0;
    let mut n: OSQPInt = (*(*work).data).n;
    let mut m: OSQPInt = (*(*work).data).m;
    let mut n_plus_mred: OSQPInt = OSQPVectorf_length(rhs);
    let mut active_flags: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut rhsv: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut q: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut l: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut u: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    active_flags = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t))
        as *mut OSQPInt;
    rhsv =
        malloc((n_plus_mred as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
            as *mut OSQPFloat;
    q = malloc((n as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    l = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    u = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    if active_flags.is_null() || rhsv.is_null() || q.is_null() || l.is_null() || u.is_null() {
        free(active_flags as *mut ::core::ffi::c_void);
        free(rhsv as *mut ::core::ffi::c_void);
        free(q as *mut ::core::ffi::c_void);
        free(l as *mut ::core::ffi::c_void);
        free(u as *mut ::core::ffi::c_void);
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"form_rhs_red\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    OSQPVectori_to_raw(active_flags, (*(*work).pol).active_flags);
    OSQPVectorf_to_raw(rhsv, rhs);
    OSQPVectorf_to_raw(q, (*(*work).data).q);
    OSQPVectorf_to_raw(l, (*(*work).data).l);
    OSQPVectorf_to_raw(u, (*(*work).data).u);
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*(*work).data).n {
        *rhsv.offset(j as isize) = -*q.offset(j as isize);
        j += 1;
    }
    counter = 0 as ::core::ffi::c_int as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*(*work).data).m {
        if *active_flags.offset(j as isize) == -(1 as ::core::ffi::c_int) {
            *rhsv.offset(((*(*work).data).n + counter) as isize) = *l.offset(j as isize);
            counter += 1;
        } else if *active_flags.offset(j as isize) == 1 as ::core::ffi::c_int {
            *rhsv.offset(((*(*work).data).n + counter) as isize) = *u.offset(j as isize);
            counter += 1;
        }
        j += 1;
    }
    OSQPVectorf_from_raw(rhs, rhsv);
    free(active_flags as *mut ::core::ffi::c_void);
    free(rhsv as *mut ::core::ffi::c_void);
    free(q as *mut ::core::ffi::c_void);
    free(l as *mut ::core::ffi::c_void);
    free(u as *mut ::core::ffi::c_void);
    return OSQP_NO_ERROR as ::core::ffi::c_int as OSQPInt;
}
unsafe extern "C" fn iterative_refinement(
    mut solver: *mut OSQPSolver,
    mut p: *mut LinSysSolver,
    mut z: *mut OSQPVectorf,
    mut b: *mut OSQPVectorf,
) -> OSQPInt {
    let mut i: OSQPInt = 0;
    let mut mred: OSQPInt = 0;
    let mut rhs: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut rhs1: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut rhs2: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut z1: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut z2: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    if (*settings).polish_refine_iter > 0 as ::core::ffi::c_int {
        mred = OSQPMatrix_get_m((*(*work).pol).Ared);
        rhs = OSQPVectorf_malloc((*(*work).data).n + mred);
        rhs1 = OSQPVectorf_view(rhs, 0 as OSQPInt, (*(*work).data).n);
        rhs2 = OSQPVectorf_view(rhs, (*(*work).data).n, mred);
        z1 = OSQPVectorf_view(z, 0 as OSQPInt, (*(*work).data).n);
        z2 = OSQPVectorf_view(z, (*(*work).data).n, mred);
        if rhs.is_null() || rhs1.is_null() || rhs2.is_null() || z1.is_null() || z2.is_null() {
            return _osqp_error(
                OSQP_MEM_ALLOC_ERROR,
                b"iterative_refinement\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < (*settings).polish_refine_iter {
            OSQPVectorf_copy(rhs, b);
            OSQPMatrix_Axpy((*(*work).data).P, z1, rhs1, -1.0f64, 1.0f64);
            OSQPMatrix_Atxpy((*(*work).pol).Ared, z2, rhs1, -1.0f64, 1.0f64);
            OSQPMatrix_Axpy((*(*work).pol).Ared, z1, rhs2, -1.0f64, 1.0f64);
            (*p).solve.expect("non-null function pointer")(p, rhs, 1 as OSQPInt);
            OSQPVectorf_plus(z, z, rhs);
            i += 1;
        }
        OSQPVectorf_free(rhs);
        OSQPVectorf_view_free(rhs1);
        OSQPVectorf_view_free(rhs2);
        OSQPVectorf_view_free(z1);
        OSQPVectorf_view_free(z2);
    }
    return 0 as OSQPInt;
}
unsafe extern "C" fn get_ypol_from_yred(
    mut work: *mut OSQPWorkspace,
    mut yred_vf: *mut OSQPVectorf,
) -> OSQPInt {
    let mut j: OSQPInt = 0;
    let mut counter: OSQPInt = 0;
    let mut m: OSQPInt = (*(*work).data).m;
    let mut mred: OSQPInt = OSQPVectorf_length(yred_vf);
    let mut active_flags: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut y: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut yred: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    active_flags = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t))
        as *mut OSQPInt;
    y = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    yred = malloc((mred as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    if active_flags.is_null() || y.is_null() || yred.is_null() {
        free(active_flags as *mut ::core::ffi::c_void);
        free(y as *mut ::core::ffi::c_void);
        free(yred as *mut ::core::ffi::c_void);
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"get_ypol_from_yred\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    OSQPVectori_to_raw(active_flags, (*(*work).pol).active_flags);
    OSQPVectorf_to_raw(y, (*work).y);
    OSQPVectorf_to_raw(yred, yred_vf);
    if (*(*work).pol).n_active == 0 as ::core::ffi::c_int {
        OSQPVectorf_set_scalar((*(*work).pol).y, 0.0f64);
        free(active_flags as *mut ::core::ffi::c_void);
        free(y as *mut ::core::ffi::c_void);
        free(yred as *mut ::core::ffi::c_void);
        return OSQP_NO_ERROR as ::core::ffi::c_int as OSQPInt;
    }
    counter = 0 as ::core::ffi::c_int as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*(*work).data).m {
        if *active_flags.offset(j as isize) == 0 as ::core::ffi::c_int {
            *y.offset(j as isize) = 0 as ::core::ffi::c_int as OSQPFloat;
        } else {
            *y.offset(j as isize) = *yred.offset(counter as isize);
            counter += 1;
        }
        j += 1;
    }
    OSQPVectorf_from_raw((*(*work).pol).y, y);
    free(active_flags as *mut ::core::ffi::c_void);
    free(y as *mut ::core::ffi::c_void);
    free(yred as *mut ::core::ffi::c_void);
    return OSQP_NO_ERROR as ::core::ffi::c_int as OSQPInt;
}
#[export_name = "honest_osqp_polish"]
pub unsafe extern "C" fn polish(mut solver: *mut OSQPSolver) -> OSQPInt {
    let mut polish_successful: OSQPInt = 0 as OSQPInt;
    let mut exitflag: OSQPInt = 0 as OSQPInt;
    let mut plsh: *mut LinSysSolver = ::core::ptr::null_mut::<LinSysSolver>();
    let mut rhs_red: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut pol_sol: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut pol_sol_xview: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut pol_sol_yview: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut info: *mut OSQPInfo = (*solver).info;
    let mut settings: *mut OSQPSettings = (*solver).settings;
    let mut work: *mut OSQPWorkspace = (*solver).work;
    exitflag = form_Ared(work);
    if exitflag != 0 {
        (*info).status_polish = OSQP_POLISH_FAILED as ::core::ffi::c_int as OSQPInt;
        return exitflag;
    } else if (*(*work).pol).n_active == 0 as ::core::ffi::c_int {
        (*info).status_polish = OSQP_POLISH_NO_ACTIVE_SET_FOUND as ::core::ffi::c_int as OSQPInt;
        OSQPMatrix_free((*(*work).pol).Ared);
        return OSQP_NO_ERROR as ::core::ffi::c_int as OSQPInt;
    }
    exitflag = osqp_algebra_init_linsys_solver(
        &raw mut plsh,
        (*(*work).data).P,
        (*(*work).pol).Ared,
        ::core::ptr::null::<OSQPVectorf>(),
        settings,
        ::core::ptr::null_mut::<OSQPFloat>(),
        ::core::ptr::null_mut::<OSQPFloat>(),
        1 as OSQPInt,
    );
    if exitflag != 0 {
        (*info).status_polish = OSQP_POLISH_LINSYS_ERROR as ::core::ffi::c_int as OSQPInt;
        OSQPMatrix_free((*(*work).pol).Ared);
        return exitflag;
    }
    rhs_red = OSQPVectorf_malloc((*(*work).data).n + (*(*work).pol).n_active);
    if rhs_red.is_null() {
        (*info).status_polish = OSQP_POLISH_FAILED as ::core::ffi::c_int as OSQPInt;
        OSQPMatrix_free((*(*work).pol).Ared);
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"polish\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    exitflag = form_rhs_red(work, rhs_red);
    if exitflag != 0 {
        (*info).status_polish = OSQP_POLISH_FAILED as ::core::ffi::c_int as OSQPInt;
        OSQPMatrix_free((*(*work).pol).Ared);
        return exitflag;
    }
    pol_sol = OSQPVectorf_copy_new(rhs_red);
    if pol_sol.is_null() {
        (*info).status_polish = OSQP_POLISH_FAILED as ::core::ffi::c_int as OSQPInt;
        OSQPMatrix_free((*(*work).pol).Ared);
        OSQPVectorf_free(rhs_red);
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"polish\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pol_sol_xview = OSQPVectorf_view(pol_sol, 0 as OSQPInt, (*(*work).data).n);
    pol_sol_yview = OSQPVectorf_view(pol_sol, (*(*work).data).n, (*(*work).pol).n_active);
    if pol_sol_xview.is_null() || pol_sol_yview.is_null() {
        (*info).status_polish = OSQP_POLISH_FAILED as ::core::ffi::c_int as OSQPInt;
        OSQPMatrix_free((*(*work).pol).Ared);
        OSQPVectorf_free(rhs_red);
        OSQPVectorf_free(pol_sol);
        OSQPVectorf_view_free(pol_sol_xview);
        OSQPVectorf_view_free(pol_sol_yview);
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"polish\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*plsh).warm_start.expect("non-null function pointer")(plsh, (*work).x);
    (*plsh).solve.expect("non-null function pointer")(plsh, pol_sol, 1 as OSQPInt);
    exitflag = iterative_refinement(solver, plsh, pol_sol, rhs_red);
    if exitflag != 0 {
        (*info).status_polish = OSQP_POLISH_FAILED as ::core::ffi::c_int as OSQPInt;
        OSQPMatrix_free((*(*work).pol).Ared);
        OSQPVectorf_free(rhs_red);
        OSQPVectorf_free(pol_sol);
        OSQPVectorf_view_free(pol_sol_xview);
        OSQPVectorf_view_free(pol_sol_yview);
        return exitflag;
    }
    OSQPVectorf_copy((*(*work).pol).x, pol_sol_xview);
    OSQPMatrix_Axpy(
        (*(*work).data).A,
        (*(*work).pol).x,
        (*(*work).pol).z,
        1.0f64,
        0.0f64,
    );
    get_ypol_from_yred(work, pol_sol_yview);
    OSQPVectorf_plus((*(*work).pol).y, (*(*work).pol).y, (*(*work).pol).z);
    OSQPVectorf_ew_bound_vec(
        (*(*work).pol).z,
        (*(*work).pol).y,
        (*(*work).data).l,
        (*(*work).data).u,
    );
    OSQPVectorf_minus((*(*work).pol).y, (*(*work).pol).y, (*(*work).pol).z);
    update_info(solver, 0 as OSQPInt, 1 as OSQPInt);
    polish_successful = ((*(*work).pol).prim_res < (*info).prim_res
        && (*(*work).pol).dual_res < (*info).dual_res
        || (*(*work).pol).prim_res < (*info).prim_res && (*info).dual_res < 1e-10f64
        || (*(*work).pol).dual_res < (*info).dual_res && (*info).prim_res < 1e-10f64)
        as ::core::ffi::c_int as OSQPInt;
    if polish_successful != 0 {
        (*info).obj_val = (*(*work).pol).obj_val;
        (*info).dual_obj_val = (*(*work).pol).dual_obj_val;
        (*info).duality_gap = (*(*work).pol).duality_gap;
        (*info).prim_res = (*(*work).pol).prim_res;
        (*info).dual_res = (*(*work).pol).dual_res;
        (*info).status_polish = OSQP_POLISH_SUCCESS as ::core::ffi::c_int as OSQPInt;
        OSQPVectorf_copy((*work).x, (*(*work).pol).x);
        OSQPVectorf_copy((*work).z, (*(*work).pol).z);
        OSQPVectorf_copy((*work).y, (*(*work).pol).y);
    } else {
        (*info).status_polish = OSQP_POLISH_FAILED as ::core::ffi::c_int as OSQPInt;
    }
    (*plsh).free.expect("non-null function pointer")(plsh);
    OSQPMatrix_free((*(*work).pol).Ared);
    OSQPVectorf_free(rhs_red);
    OSQPVectorf_free(pol_sol);
    OSQPVectorf_view_free(pol_sol_xview);
    OSQPVectorf_view_free(pol_sol_yview);
    return OSQP_NO_ERROR as ::core::ffi::c_int as OSQPInt;
}
