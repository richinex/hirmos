#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPProfilerItemInfo {
    pub name: *const ::core::ffi::c_char,
    pub desc: *const ::core::ffi::c_char,
    pub level: ::core::ffi::c_int,
}
#[export_name = "honest_osqp_osqp_profiler_sections"]
pub static mut osqp_profiler_sections: [OSQPProfilerItemInfo; 14] = [
    OSQPProfilerItemInfo {
        name: b"prob_setup\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Problem setup\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"prob_scale\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Problem data scaling\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"solve_opt_prob\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Solving optimization problem\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"admm_iter\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"ADMM iteration\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"admm_kkt_solve\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"KKT system solve in ADMM iteration\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"admm_vec_update\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Vector updates in ADMM iteration\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"admm_project\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Projection in ADMM iteration\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"sol_polish\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Solution polishing\0" as *const u8 as *const ::core::ffi::c_char,
        level: 1 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"linsys_init\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Initialize linear system solver\0" as *const u8 as *const ::core::ffi::c_char,
        level: 2 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"linsys_solve\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Solve the linear system\0" as *const u8 as *const ::core::ffi::c_char,
        level: 2 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"linsys_sym_fac\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Symbolic factorization in direct solver\0" as *const u8
            as *const ::core::ffi::c_char,
        level: 2 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"linsys_num_fac\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Numeric factorization in direct solver\0" as *const u8
            as *const ::core::ffi::c_char,
        level: 2 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"linsys_backsolve\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Backsolve in direct solver\0" as *const u8 as *const ::core::ffi::c_char,
        level: 2 as ::core::ffi::c_int,
    },
    OSQPProfilerItemInfo {
        name: b"linsys_mvm\0" as *const u8 as *const ::core::ffi::c_char,
        desc: b"Matrix-vector multiplication\0" as *const u8 as *const ::core::ffi::c_char,
        level: 2 as ::core::ffi::c_int,
    },
];
#[export_name = "honest_osqp_osqp_profiler_events"]
pub static mut osqp_profiler_events: [OSQPProfilerItemInfo; 1] = [OSQPProfilerItemInfo {
    name: b"rho_update\0" as *const u8 as *const ::core::ffi::c_char,
    desc: b"Rho update\0" as *const u8 as *const ::core::ffi::c_char,
    level: 1 as ::core::ffi::c_int,
}];
