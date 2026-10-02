#[repr(C)] pub struct OSQPVectorf_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPMatrix_ { _opaque: [u8; 0] }
extern "C" {
    #[link_name = "honest_osqp_init_linsys_solver_qdldl"]
    fn init_linsys_solver_qdldl(
        sp: *mut *mut qdldl_solver,
        P: *const OSQPMatrix,
        A: *const OSQPMatrix,
        rho_vec: *const OSQPVectorf,
        settings: *const OSQPSettings,
        polishing: OSQPInt,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_adjoint_derivative_qdldl"]
    fn adjoint_derivative_qdldl(
        s: *mut *mut qdldl_solver,
        P: *const OSQPMatrix,
        G: *const OSQPMatrix,
        A_eq: *const OSQPMatrix,
        GDiagLambda: *const OSQPMatrix,
        slacks: *const OSQPVectorf,
        rhs: *mut OSQPVectorf,
    ) -> OSQPInt;
}
pub type osqp_capabilities_type = ::core::ffi::c_uint;
pub const OSQP_CAPABILITY_DERIVATIVES: osqp_capabilities_type = 16;
pub const OSQP_CAPABILITY_UPDATE_MATRICES: osqp_capabilities_type = 8;
pub const OSQP_CAPABILITY_CODEGEN: osqp_capabilities_type = 4;
pub const OSQP_CAPABILITY_INDIRECT_SOLVER: osqp_capabilities_type = 2;
pub const OSQP_CAPABILITY_DIRECT_SOLVER: osqp_capabilities_type = 1;
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
pub type OSQPVectorf = OSQPVectorf_;
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
pub type QDLDL_int = ::core::ffi::c_int;
pub type QDLDL_float = ::core::ffi::c_double;
pub type QDLDL_bool = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qdldl {
    pub type_0: osqp_linsys_solver_type,
    pub name: Option<unsafe extern "C" fn(*mut qdldl) -> *const ::core::ffi::c_char>,
    pub solve: Option<unsafe extern "C" fn(*mut qdldl, *mut OSQPVectorf, OSQPInt) -> OSQPInt>,
    pub update_settings: Option<unsafe extern "C" fn(*mut qdldl, *const OSQPSettings) -> ()>,
    pub warm_start: Option<unsafe extern "C" fn(*mut qdldl, *const OSQPVectorf) -> ()>,
    pub adjoint_derivative: Option<
        unsafe extern "C" fn(
            *mut *mut qdldl_solver,
            *const OSQPMatrix,
            *const OSQPMatrix,
            *const OSQPMatrix,
            *const OSQPMatrix,
            *const OSQPVectorf,
            *mut OSQPVectorf,
        ) -> OSQPInt,
    >,
    pub free: Option<unsafe extern "C" fn(*mut qdldl) -> ()>,
    pub update_matrices: Option<
        unsafe extern "C" fn(
            *mut qdldl,
            *const OSQPMatrix,
            *const OSQPInt,
            OSQPInt,
            *const OSQPMatrix,
            *const OSQPInt,
            OSQPInt,
        ) -> OSQPInt,
    >,
    pub update_rho_vec:
        Option<unsafe extern "C" fn(*mut qdldl, *const OSQPVectorf, OSQPFloat) -> OSQPInt>,
    pub nthreads: OSQPInt,
    pub L: *mut OSQPCscMatrix,
    pub Dinv: *mut OSQPFloat,
    pub P: *mut OSQPInt,
    pub bp: *mut OSQPFloat,
    pub sol: *mut OSQPFloat,
    pub rho_inv_vec: *mut OSQPFloat,
    pub sigma: OSQPFloat,
    pub rho_inv: OSQPFloat,
    pub polishing: OSQPInt,
    pub n: OSQPInt,
    pub m: OSQPInt,
    pub KKT: *mut OSQPCscMatrix,
    pub PtoKKT: *mut OSQPInt,
    pub AtoKKT: *mut OSQPInt,
    pub rhotoKKT: *mut OSQPInt,
    pub D: *mut QDLDL_float,
    pub etree: *mut QDLDL_int,
    pub Lnz: *mut QDLDL_int,
    pub iwork: *mut QDLDL_int,
    pub bwork: *mut QDLDL_bool,
    pub fwork: *mut QDLDL_float,
    pub adj: *mut OSQPCscMatrix,
}
pub type qdldl_solver = qdldl;
#[export_name = "honest_osqp_osqp_algebra_linsys_supported"]
pub unsafe extern "C" fn osqp_algebra_linsys_supported() -> OSQPInt {
    return OSQP_CAPABILITY_DIRECT_SOLVER as ::core::ffi::c_int as OSQPInt;
}
#[export_name = "honest_osqp_osqp_algebra_default_linsys"]
pub unsafe extern "C" fn osqp_algebra_default_linsys() -> osqp_linsys_solver_type {
    return OSQP_DIRECT_SOLVER;
}
#[export_name = "honest_osqp_osqp_algebra_init_libs"]
pub unsafe extern "C" fn osqp_algebra_init_libs(mut device: OSQPInt) -> OSQPInt {
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_osqp_algebra_free_libs"]
pub unsafe extern "C" fn osqp_algebra_free_libs() {}
#[export_name = "honest_osqp_osqp_algebra_name"]
pub unsafe extern "C" fn osqp_algebra_name(
    mut name: *mut ::core::ffi::c_char,
    mut nameLen: OSQPInt,
) -> OSQPInt {
    *name.offset(0 as ::core::ffi::c_int as isize) = 'B' as i32 as ::core::ffi::c_char;
    *name.offset(1 as ::core::ffi::c_int as isize) = 'u' as i32 as ::core::ffi::c_char;
    *name.offset(2 as ::core::ffi::c_int as isize) = 'i' as i32 as ::core::ffi::c_char;
    *name.offset(3 as ::core::ffi::c_int as isize) = 'l' as i32 as ::core::ffi::c_char;
    *name.offset(4 as ::core::ffi::c_int as isize) = 't' as i32 as ::core::ffi::c_char;
    *name.offset(5 as ::core::ffi::c_int as isize) = '-' as i32 as ::core::ffi::c_char;
    *name.offset(6 as ::core::ffi::c_int as isize) = 'i' as i32 as ::core::ffi::c_char;
    *name.offset(7 as ::core::ffi::c_int as isize) = 'n' as i32 as ::core::ffi::c_char;
    *name.offset(8 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    return 9 as OSQPInt;
}
#[export_name = "honest_osqp_osqp_algebra_device_name"]
pub unsafe extern "C" fn osqp_algebra_device_name(
    mut name: *mut ::core::ffi::c_char,
    mut nameLen: OSQPInt,
) -> OSQPInt {
    *name.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_osqp_algebra_init_linsys_solver"]
pub unsafe extern "C" fn osqp_algebra_init_linsys_solver(
    mut s: *mut *mut LinSysSolver,
    mut P: *const OSQPMatrix,
    mut A: *const OSQPMatrix,
    mut rho_vec: *const OSQPVectorf,
    mut settings: *const OSQPSettings,
    mut scaled_prim_res: *mut OSQPFloat,
    mut scaled_dual_res: *mut OSQPFloat,
    mut polishing: OSQPInt,
) -> OSQPInt {
    let mut retval: OSQPInt = 0 as OSQPInt;
    match (*settings).linsys_solver as ::core::ffi::c_uint {
        1 | _ => {}
    }
    retval = init_linsys_solver_qdldl(
        s as *mut *mut qdldl_solver,
        P,
        A,
        rho_vec,
        settings,
        polishing,
    );
    return retval;
}
#[export_name = "honest_osqp_adjoint_derivative_linsys_solver"]
pub unsafe extern "C" fn adjoint_derivative_linsys_solver(
    mut s: *mut *mut LinSysSolver,
    mut settings: *const OSQPSettings,
    mut P: *const OSQPMatrix,
    mut G: *const OSQPMatrix,
    mut A_eq: *const OSQPMatrix,
    mut GDiagLambda: *mut OSQPMatrix,
    mut slacks: *mut OSQPVectorf,
    mut rhs: *mut OSQPVectorf,
) -> OSQPInt {
    return adjoint_derivative_qdldl(
        s as *mut *mut qdldl_solver,
        P,
        G,
        A_eq,
        GDiagLambda,
        slacks,
        rhs,
    );
}
