use crate::runtime::{malloc};
extern "C" {
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
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "honest_osqp_c_strcpy"]
pub unsafe extern "C" fn c_strcpy(
    mut dest: *mut ::core::ffi::c_char,
    mut source: *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        *dest.offset(i as isize) = *source.offset(i as isize);
        if *dest.offset(i as isize) as ::core::ffi::c_int == '\0' as i32 {
            break;
        }
        i += 1;
    }
}
#[export_name = "honest_osqp_copy_settings"]
pub unsafe extern "C" fn copy_settings(mut settings: *const OSQPSettings) -> *mut OSQPSettings {
    let mut new: *mut OSQPSettings =
        malloc(::core::mem::size_of::<OSQPSettings>() as size_t) as *mut OSQPSettings;
    if new.is_null() {
        return ::core::ptr::null_mut::<OSQPSettings>();
    }
    (*new).device = (*settings).device;
    (*new).linsys_solver = (*settings).linsys_solver;
    (*new).allocate_solution = (*settings).allocate_solution;
    (*new).profiler_level = (*settings).profiler_level;
    (*new).verbose = (*settings).verbose;
    (*new).warm_starting = (*settings).warm_starting;
    (*new).scaling = (*settings).scaling;
    (*new).polishing = (*settings).polishing;
    (*new).rho = (*settings).rho;
    (*new).rho_is_vec = (*settings).rho_is_vec;
    (*new).sigma = (*settings).sigma;
    (*new).alpha = (*settings).alpha;
    (*new).cg_max_iter = (*settings).cg_max_iter;
    (*new).cg_tol_reduction = (*settings).cg_tol_reduction;
    (*new).cg_tol_fraction = (*settings).cg_tol_fraction;
    (*new).cg_precond = (*settings).cg_precond;
    (*new).adaptive_rho = (*settings).adaptive_rho;
    (*new).adaptive_rho_interval = (*settings).adaptive_rho_interval;
    (*new).adaptive_rho_fraction = (*settings).adaptive_rho_fraction;
    (*new).adaptive_rho_tolerance = (*settings).adaptive_rho_tolerance;
    (*new).max_iter = (*settings).max_iter;
    (*new).eps_abs = (*settings).eps_abs;
    (*new).eps_rel = (*settings).eps_rel;
    (*new).eps_prim_inf = (*settings).eps_prim_inf;
    (*new).eps_dual_inf = (*settings).eps_dual_inf;
    (*new).scaled_termination = (*settings).scaled_termination;
    (*new).check_termination = (*settings).check_termination;
    (*new).check_dualgap = (*settings).check_dualgap;
    (*new).time_limit = (*settings).time_limit;
    (*new).delta = (*settings).delta;
    (*new).polish_refine_iter = (*settings).polish_refine_iter;
    return new;
}
