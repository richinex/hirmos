use crate::honest_did::lpsolve::runtime::{free,sqrt,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_allocINT"]
    fn allocINT(
        lp: *mut lprec,
        ptr: *mut *mut ::core::ffi::c_int,
        size: ::core::ffi::c_int,
        clear: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_allocREAL"]
    fn allocREAL(
        lp: *mut lprec,
        ptr: *mut *mut ::core::ffi::c_double,
        size: ::core::ffi::c_int,
        clear: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_validate"]
    fn mat_validate(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_collength"]
    fn mat_collength(mat: *mut MATrec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_rowlength"]
    fn mat_rowlength(mat: *mut MATrec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_computemax"]
    fn mat_computemax(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_nonzeros"]
    fn get_nonzeros(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_int"]
    fn is_int(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_scalemode"]
    fn is_scalemode(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_scaletype"]
    fn is_scaletype(lp: *mut lprec, scaletype: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_integerscaling"]
    fn is_integerscaling(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_action"]
    fn set_action(actionvar: *mut ::core::ffi::c_int, actionmask: ::core::ffi::c_int);
}
pub type __int64_t = i64;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lprec {
    pub add_column: Option<add_column_func>,
    pub add_columnex: Option<add_columnex_func>,
    pub add_constraint: Option<add_constraint_func>,
    pub add_constraintex: Option<add_constraintex_func>,
    pub add_lag_con: Option<add_lag_con_func>,
    pub add_SOS: Option<add_SOS_func>,
    pub column_in_lp: Option<column_in_lp_func>,
    pub copy_lp: Option<copy_lp_func>,
    pub default_basis: Option<default_basis_func>,
    pub del_column: Option<del_column_func>,
    pub del_constraint: Option<del_constraint_func>,
    pub delete_lp: Option<delete_lp_func>,
    pub dualize_lp: Option<dualize_lp_func>,
    pub free_lp: Option<free_lp_func>,
    pub get_anti_degen: Option<get_anti_degen_func>,
    pub get_basis: Option<get_basis_func>,
    pub get_basiscrash: Option<get_basiscrash_func>,
    pub get_bb_depthlimit: Option<get_bb_depthlimit_func>,
    pub get_bb_floorfirst: Option<get_bb_floorfirst_func>,
    pub get_bb_rule: Option<get_bb_rule_func>,
    pub get_bounds_tighter: Option<get_bounds_tighter_func>,
    pub get_break_at_value: Option<get_break_at_value_func>,
    pub get_col_name: Option<get_col_name_func>,
    pub get_columnex: Option<get_columnex_func>,
    pub get_constr_type: Option<get_constr_type_func>,
    pub get_constr_value: Option<get_constr_value_func>,
    pub get_constraints: Option<get_constraints_func>,
    pub get_dual_solution: Option<get_dual_solution_func>,
    pub get_epsb: Option<get_epsb_func>,
    pub get_epsd: Option<get_epsd_func>,
    pub get_epsel: Option<get_epsel_func>,
    pub get_epsint: Option<get_epsint_func>,
    pub get_epsperturb: Option<get_epsperturb_func>,
    pub get_epspivot: Option<get_epspivot_func>,
    pub get_improve: Option<get_improve_func>,
    pub get_infinite: Option<get_infinite_func>,
    pub get_lambda: Option<get_lambda_func>,
    pub get_lowbo: Option<get_lowbo_func>,
    pub get_lp_index: Option<get_lp_index_func>,
    pub get_lp_name: Option<get_lp_name_func>,
    pub get_Lrows: Option<get_Lrows_func>,
    pub get_mat: Option<get_mat_func>,
    pub get_mat_byindex: Option<get_mat_byindex_func>,
    pub get_max_level: Option<get_max_level_func>,
    pub get_maxpivot: Option<get_maxpivot_func>,
    pub get_mip_gap: Option<get_mip_gap_func>,
    pub get_multiprice: Option<get_multiprice_func>,
    pub get_nameindex: Option<get_nameindex_func>,
    pub get_Ncolumns: Option<get_Ncolumns_func>,
    pub get_negrange: Option<get_negrange_func>,
    pub get_nonzeros: Option<get_nz_func>,
    pub get_Norig_columns: Option<get_Norig_columns_func>,
    pub get_Norig_rows: Option<get_Norig_rows_func>,
    pub get_Nrows: Option<get_Nrows_func>,
    pub get_obj_bound: Option<get_obj_bound_func>,
    pub get_objective: Option<get_objective_func>,
    pub get_orig_index: Option<get_orig_index_func>,
    pub get_origcol_name: Option<get_origcol_name_func>,
    pub get_origrow_name: Option<get_origrow_name_func>,
    pub get_partialprice: Option<get_partialprice_func>,
    pub get_pivoting: Option<get_pivoting_func>,
    pub get_presolve: Option<get_presolve_func>,
    pub get_presolveloops: Option<get_presolveloops_func>,
    pub get_primal_solution: Option<get_primal_solution_func>,
    pub get_print_sol: Option<get_print_sol_func>,
    pub get_pseudocosts: Option<get_pseudocosts_func>,
    pub get_ptr_constraints: Option<get_ptr_constraints_func>,
    pub get_ptr_dual_solution: Option<get_ptr_dual_solution_func>,
    pub get_ptr_lambda: Option<get_ptr_lambda_func>,
    pub get_ptr_primal_solution: Option<get_ptr_primal_solution_func>,
    pub get_ptr_sensitivity_obj: Option<get_ptr_sensitivity_obj_func>,
    pub get_ptr_sensitivity_objex: Option<get_ptr_sensitivity_objex_func>,
    pub get_ptr_sensitivity_rhs: Option<get_ptr_sensitivity_rhs_func>,
    pub get_ptr_variables: Option<get_ptr_variables_func>,
    pub get_rh: Option<get_rh_func>,
    pub get_rh_range: Option<get_rh_range_func>,
    pub get_row: Option<get_row_func>,
    pub get_rowex: Option<get_rowex_func>,
    pub get_row_name: Option<get_row_name_func>,
    pub get_scalelimit: Option<get_scalelimit_func>,
    pub get_scaling: Option<get_scaling_func>,
    pub get_sensitivity_obj: Option<get_sensitivity_obj_func>,
    pub get_sensitivity_objex: Option<get_sensitivity_objex_func>,
    pub get_sensitivity_rhs: Option<get_sensitivity_rhs_func>,
    pub get_simplextype: Option<get_simplextype_func>,
    pub get_solutioncount: Option<get_solutioncount_func>,
    pub get_solutionlimit: Option<get_solutionlimit_func>,
    pub get_status: Option<get_status_func>,
    pub get_statustext: Option<get_statustext_func>,
    pub get_timeout: Option<get_timeout_func>,
    pub get_total_iter: Option<get_total_iter_func>,
    pub get_total_nodes: Option<get_total_nodes_func>,
    pub get_upbo: Option<get_upbo_func>,
    pub get_var_branch: Option<get_var_branch_func>,
    pub get_var_dualresult: Option<get_var_dualresult_func>,
    pub get_var_primalresult: Option<get_var_primalresult_func>,
    pub get_var_priority: Option<get_var_priority_func>,
    pub get_variables: Option<get_variables_func>,
    pub get_verbose: Option<get_verbose_func>,
    pub get_working_objective: Option<get_working_objective_func>,
    pub has_BFP: Option<has_BFP_func>,
    pub has_XLI: Option<has_XLI_func>,
    pub is_add_rowmode: Option<is_add_rowmode_func>,
    pub is_anti_degen: Option<is_anti_degen_func>,
    pub is_binary: Option<is_binary_func>,
    pub is_break_at_first: Option<is_break_at_first_func>,
    pub is_constr_type: Option<is_constr_type_func>,
    pub is_debug: Option<is_debug_func>,
    pub is_feasible: Option<is_feasible_func>,
    pub is_infinite: Option<is_infinite_func>,
    pub is_int: Option<is_int_func>,
    pub is_integerscaling: Option<is_integerscaling_func>,
    pub is_lag_trace: Option<is_lag_trace_func>,
    pub is_maxim: Option<is_maxim_func>,
    pub is_nativeBFP: Option<is_nativeBFP_func>,
    pub is_nativeXLI: Option<is_nativeXLI_func>,
    pub is_negative: Option<is_negative_func>,
    pub is_obj_in_basis: Option<is_obj_in_basis_func>,
    pub is_piv_mode: Option<is_piv_mode_func>,
    pub is_piv_rule: Option<is_piv_rule_func>,
    pub is_presolve: Option<is_presolve_func>,
    pub is_scalemode: Option<is_scalemode_func>,
    pub is_scaletype: Option<is_scaletype_func>,
    pub is_semicont: Option<is_semicont_func>,
    pub is_SOS_var: Option<is_SOS_var_func>,
    pub is_trace: Option<is_trace_func>,
    pub is_unbounded: Option<is_unbounded_func>,
    pub is_use_names: Option<is_use_names_func>,
    pub lp_solve_version: Option<lp_solve_version_func>,
    pub make_lp: Option<make_lp_func>,
    pub print_constraints: Option<print_constraints_func>,
    pub print_duals: Option<print_duals_func>,
    pub print_lp: Option<print_lp_func>,
    pub print_objective: Option<print_objective_func>,
    pub print_scales: Option<print_scales_func>,
    pub print_solution: Option<print_solution_func>,
    pub print_str: Option<print_str_func>,
    pub print_tableau: Option<print_tableau_func>,
    pub put_abortfunc: Option<put_abortfunc_func>,
    pub put_bb_nodefunc: Option<put_bb_nodefunc_func>,
    pub put_bb_branchfunc: Option<put_bb_branchfunc_func>,
    pub put_logfunc: Option<put_logfunc_func>,
    pub put_msgfunc: Option<put_msgfunc_func>,
    pub read_LP: Option<read_LP_func>,
    pub read_MPS: Option<read_MPS_func>,
    pub read_XLI: Option<read_XLI_func>,
    pub read_params: Option<read_params_func>,
    pub read_basis: Option<read_basis_func>,
    pub reset_basis: Option<reset_basis_func>,
    pub reset_params: Option<reset_params_func>,
    pub resize_lp: Option<resize_lp_func>,
    pub set_add_rowmode: Option<set_add_rowmode_func>,
    pub set_anti_degen: Option<set_anti_degen_func>,
    pub set_basisvar: Option<set_basisvar_func>,
    pub set_basis: Option<set_basis_func>,
    pub set_basiscrash: Option<set_basiscrash_func>,
    pub set_bb_depthlimit: Option<set_bb_depthlimit_func>,
    pub set_bb_floorfirst: Option<set_bb_floorfirst_func>,
    pub set_bb_rule: Option<set_bb_rule_func>,
    pub set_BFP: Option<set_BFP_func>,
    pub set_binary: Option<set_binary_func>,
    pub set_bounds: Option<set_bounds_func>,
    pub set_bounds_tighter: Option<set_bounds_tighter_func>,
    pub set_break_at_first: Option<set_break_at_first_func>,
    pub set_break_at_value: Option<set_break_at_value_func>,
    pub set_column: Option<set_column_func>,
    pub set_columnex: Option<set_columnex_func>,
    pub set_col_name: Option<set_col_name_func>,
    pub set_constr_type: Option<set_constr_type_func>,
    pub set_debug: Option<set_debug_func>,
    pub set_epsb: Option<set_epsb_func>,
    pub set_epsd: Option<set_epsd_func>,
    pub set_epsel: Option<set_epsel_func>,
    pub set_epsint: Option<set_epsint_func>,
    pub set_epslevel: Option<set_epslevel_func>,
    pub set_epsperturb: Option<set_epsperturb_func>,
    pub set_epspivot: Option<set_epspivot_func>,
    pub set_unbounded: Option<set_unbounded_func>,
    pub set_improve: Option<set_improve_func>,
    pub set_infinite: Option<set_infinite_func>,
    pub set_int: Option<set_int_func>,
    pub set_lag_trace: Option<set_lag_trace_func>,
    pub set_lowbo: Option<set_lowbo_func>,
    pub set_lp_name: Option<set_lp_name_func>,
    pub set_mat: Option<set_mat_func>,
    pub set_maxim: Option<set_maxim_func>,
    pub set_maxpivot: Option<set_maxpivot_func>,
    pub set_minim: Option<set_minim_func>,
    pub set_mip_gap: Option<set_mip_gap_func>,
    pub set_multiprice: Option<set_multiprice_func>,
    pub set_negrange: Option<set_negrange_func>,
    pub set_obj_bound: Option<set_obj_bound_func>,
    pub set_obj_fn: Option<set_obj_fn_func>,
    pub set_obj_fnex: Option<set_obj_fnex_func>,
    pub set_obj: Option<set_obj_func>,
    pub set_obj_in_basis: Option<set_obj_in_basis_func>,
    pub set_outputfile: Option<set_outputfile_func>,
    pub set_outputstream: Option<set_outputstream_func>,
    pub set_partialprice: Option<set_partialprice_func>,
    pub set_pivoting: Option<set_pivoting_func>,
    pub set_preferdual: Option<set_preferdual_func>,
    pub set_presolve: Option<set_presolve_func>,
    pub set_print_sol: Option<set_print_sol_func>,
    pub set_pseudocosts: Option<set_pseudocosts_func>,
    pub set_rh: Option<set_rh_func>,
    pub set_rh_range: Option<set_rh_range_func>,
    pub set_rh_vec: Option<set_rh_vec_func>,
    pub set_row: Option<set_row_func>,
    pub set_rowex: Option<set_rowex_func>,
    pub set_row_name: Option<set_row_name_func>,
    pub set_scalelimit: Option<set_scalelimit_func>,
    pub set_scaling: Option<set_scaling_func>,
    pub set_semicont: Option<set_semicont_func>,
    pub set_sense: Option<set_sense_func>,
    pub set_simplextype: Option<set_simplextype_func>,
    pub set_solutionlimit: Option<set_solutionlimit_func>,
    pub set_timeout: Option<set_timeout_func>,
    pub set_trace: Option<set_trace_func>,
    pub set_upbo: Option<set_upbo_func>,
    pub set_use_names: Option<set_use_names_func>,
    pub set_var_branch: Option<set_var_branch_func>,
    pub set_var_weights: Option<set_var_weights_func>,
    pub set_verbose: Option<set_verbose_func>,
    pub set_XLI: Option<set_XLI_func>,
    pub solve: Option<solve_func>,
    pub str_add_column: Option<str_add_column_func>,
    pub str_add_constraint: Option<str_add_constraint_func>,
    pub str_add_lag_con: Option<str_add_lag_con_func>,
    pub str_set_obj_fn: Option<str_set_obj_fn_func>,
    pub str_set_rh_vec: Option<str_set_rh_vec_func>,
    pub time_elapsed: Option<time_elapsed_func>,
    pub unscale: Option<unscale_func>,
    pub write_lp: Option<write_lp_func>,
    pub write_LP: Option<write_LP_func>,
    pub write_mps: Option<write_mps_func>,
    pub write_MPS: Option<write_MPS_func>,
    pub write_freemps: Option<write_freemps_func>,
    pub write_freeMPS: Option<write_freeMPS_func>,
    pub write_XLI: Option<write_XLI_func>,
    pub write_params: Option<write_params_func>,
    pub alignmentspacer: *mut ::core::ffi::c_int,
    pub lp_name: *mut ::core::ffi::c_char,
    pub sum: ::core::ffi::c_int,
    pub rows: ::core::ffi::c_int,
    pub columns: ::core::ffi::c_int,
    pub equalities: ::core::ffi::c_int,
    pub boundedvars: ::core::ffi::c_int,
    pub INTfuture1: ::core::ffi::c_int,
    pub sum_alloc: ::core::ffi::c_int,
    pub rows_alloc: ::core::ffi::c_int,
    pub columns_alloc: ::core::ffi::c_int,
    pub source_is_file: ::core::ffi::c_uchar,
    pub model_is_pure: ::core::ffi::c_uchar,
    pub model_is_valid: ::core::ffi::c_uchar,
    pub tighten_on_set: ::core::ffi::c_uchar,
    pub names_used: ::core::ffi::c_uchar,
    pub use_row_names: ::core::ffi::c_uchar,
    pub use_col_names: ::core::ffi::c_uchar,
    pub lag_trace: ::core::ffi::c_uchar,
    pub spx_trace: ::core::ffi::c_uchar,
    pub bb_trace: ::core::ffi::c_uchar,
    pub streamowned: ::core::ffi::c_uchar,
    pub obj_in_basis: ::core::ffi::c_uchar,
    pub spx_status: ::core::ffi::c_int,
    pub lag_status: ::core::ffi::c_int,
    pub solutioncount: ::core::ffi::c_int,
    pub solutionlimit: ::core::ffi::c_int,
    pub real_solution: ::core::ffi::c_double,
    pub solution: *mut ::core::ffi::c_double,
    pub best_solution: *mut ::core::ffi::c_double,
    pub full_solution: *mut ::core::ffi::c_double,
    pub edgeVector: *mut ::core::ffi::c_double,
    pub drow: *mut ::core::ffi::c_double,
    pub nzdrow: *mut ::core::ffi::c_int,
    pub duals: *mut ::core::ffi::c_double,
    pub full_duals: *mut ::core::ffi::c_double,
    pub dualsfrom: *mut ::core::ffi::c_double,
    pub dualstill: *mut ::core::ffi::c_double,
    pub objfrom: *mut ::core::ffi::c_double,
    pub objtill: *mut ::core::ffi::c_double,
    pub objfromvalue: *mut ::core::ffi::c_double,
    pub orig_obj: *mut ::core::ffi::c_double,
    pub obj: *mut ::core::ffi::c_double,
    pub current_iter: ::core::ffi::c_longlong,
    pub total_iter: ::core::ffi::c_longlong,
    pub current_bswap: ::core::ffi::c_longlong,
    pub total_bswap: ::core::ffi::c_longlong,
    pub solvecount: ::core::ffi::c_int,
    pub max_pivots: ::core::ffi::c_int,
    pub simplex_strategy: ::core::ffi::c_int,
    pub simplex_mode: ::core::ffi::c_int,
    pub verbose: ::core::ffi::c_int,
    pub print_sol: ::core::ffi::c_int,
    pub outstream: *mut FILE,
    pub bb_varbranch: *mut ::core::ffi::c_uchar,
    pub piv_strategy: ::core::ffi::c_int,
    pub _piv_rule_: ::core::ffi::c_int,
    pub bb_rule: ::core::ffi::c_int,
    pub bb_floorfirst: ::core::ffi::c_uchar,
    pub bb_breakfirst: ::core::ffi::c_uchar,
    pub _piv_left_: ::core::ffi::c_uchar,
    pub BOOLfuture1: ::core::ffi::c_uchar,
    pub scalelimit: ::core::ffi::c_double,
    pub scalemode: ::core::ffi::c_int,
    pub improve: ::core::ffi::c_int,
    pub anti_degen: ::core::ffi::c_int,
    pub do_presolve: ::core::ffi::c_int,
    pub presolveloops: ::core::ffi::c_int,
    pub perturb_count: ::core::ffi::c_int,
    pub row_name: *mut *mut hashelem,
    pub col_name: *mut *mut hashelem,
    pub rowname_hashtab: *mut hashtable,
    pub colname_hashtab: *mut hashtable,
    pub rowblocks: *mut partialrec,
    pub colblocks: *mut partialrec,
    pub var_type: *mut ::core::ffi::c_uchar,
    pub multivars: *mut multirec,
    pub multiblockdiv: ::core::ffi::c_int,
    pub fixedvars: ::core::ffi::c_int,
    pub int_vars: ::core::ffi::c_int,
    pub sc_vars: ::core::ffi::c_int,
    pub sc_lobound: *mut ::core::ffi::c_double,
    pub var_is_free: *mut ::core::ffi::c_int,
    pub var_priority: *mut ::core::ffi::c_int,
    pub GUB: *mut SOSgroup,
    pub sos_vars: ::core::ffi::c_int,
    pub sos_ints: ::core::ffi::c_int,
    pub SOS: *mut SOSgroup,
    pub sos_priority: *mut ::core::ffi::c_int,
    pub bsolveVal: *mut ::core::ffi::c_double,
    pub bsolveIdx: *mut ::core::ffi::c_int,
    pub orig_rhs: *mut ::core::ffi::c_double,
    pub rhs: *mut ::core::ffi::c_double,
    pub row_type: *mut ::core::ffi::c_int,
    pub longsteps: *mut multirec,
    pub orig_upbo: *mut ::core::ffi::c_double,
    pub upbo: *mut ::core::ffi::c_double,
    pub orig_lowbo: *mut ::core::ffi::c_double,
    pub lowbo: *mut ::core::ffi::c_double,
    pub matA: *mut MATrec,
    pub invB: *mut INVrec,
    pub bb_bounds: *mut BBrec,
    pub rootbounds: *mut BBrec,
    pub bb_basis: *mut basisrec,
    pub rootbasis: *mut basisrec,
    pub monitor: *mut OBJmonrec,
    pub scalars: *mut ::core::ffi::c_double,
    pub scaling_used: ::core::ffi::c_uchar,
    pub columns_scaled: ::core::ffi::c_uchar,
    pub varmap_locked: ::core::ffi::c_uchar,
    pub basis_valid: ::core::ffi::c_uchar,
    pub crashmode: ::core::ffi::c_int,
    pub var_basic: *mut ::core::ffi::c_int,
    pub val_nonbasic: *mut ::core::ffi::c_double,
    pub is_basic: *mut ::core::ffi::c_uchar,
    pub is_lower: *mut ::core::ffi::c_uchar,
    pub rejectpivot: *mut ::core::ffi::c_int,
    pub bb_PseudoCost: *mut BBPSrec,
    pub bb_PseudoUpdates: ::core::ffi::c_int,
    pub bb_strongbranches: ::core::ffi::c_int,
    pub is_strongbranch: ::core::ffi::c_int,
    pub bb_improvements: ::core::ffi::c_int,
    pub rhsmax: ::core::ffi::c_double,
    pub suminfeas: ::core::ffi::c_double,
    pub bigM: ::core::ffi::c_double,
    pub P1extraVal: ::core::ffi::c_double,
    pub P1extraDim: ::core::ffi::c_int,
    pub spx_action: ::core::ffi::c_int,
    pub spx_perturbed: ::core::ffi::c_uchar,
    pub bb_break: ::core::ffi::c_uchar,
    pub wasPreprocessed: ::core::ffi::c_uchar,
    pub wasPresolved: ::core::ffi::c_uchar,
    pub INTfuture2: ::core::ffi::c_int,
    pub matL: *mut MATrec,
    pub lag_rhs: *mut ::core::ffi::c_double,
    pub lag_con_type: *mut ::core::ffi::c_int,
    pub lambda: *mut ::core::ffi::c_double,
    pub lag_bound: ::core::ffi::c_double,
    pub lag_accept: ::core::ffi::c_double,
    pub infinite: ::core::ffi::c_double,
    pub negrange: ::core::ffi::c_double,
    pub epsmachine: ::core::ffi::c_double,
    pub epsvalue: ::core::ffi::c_double,
    pub epsprimal: ::core::ffi::c_double,
    pub epsdual: ::core::ffi::c_double,
    pub epspivot: ::core::ffi::c_double,
    pub epsperturb: ::core::ffi::c_double,
    pub epssolution: ::core::ffi::c_double,
    pub bb_status: ::core::ffi::c_int,
    pub bb_level: ::core::ffi::c_int,
    pub bb_maxlevel: ::core::ffi::c_int,
    pub bb_limitlevel: ::core::ffi::c_int,
    pub bb_totalnodes: ::core::ffi::c_longlong,
    pub bb_solutionlevel: ::core::ffi::c_int,
    pub bb_cutpoolsize: ::core::ffi::c_int,
    pub bb_cutpoolused: ::core::ffi::c_int,
    pub bb_constraintOF: ::core::ffi::c_int,
    pub bb_cuttype: *mut ::core::ffi::c_int,
    pub bb_varactive: *mut ::core::ffi::c_int,
    pub bb_upperchange: *mut DeltaVrec,
    pub bb_lowerchange: *mut DeltaVrec,
    pub bb_deltaOF: ::core::ffi::c_double,
    pub bb_breakOF: ::core::ffi::c_double,
    pub bb_limitOF: ::core::ffi::c_double,
    pub bb_heuristicOF: ::core::ffi::c_double,
    pub bb_parentOF: ::core::ffi::c_double,
    pub bb_workOF: ::core::ffi::c_double,
    pub presolve_undo: *mut presolveundorec,
    pub workarrays: *mut workarraysrec,
    pub epsint: ::core::ffi::c_double,
    pub mip_absgap: ::core::ffi::c_double,
    pub mip_relgap: ::core::ffi::c_double,
    pub timecreate: ::core::ffi::c_double,
    pub timestart: ::core::ffi::c_double,
    pub timeheuristic: ::core::ffi::c_double,
    pub timepresolved: ::core::ffi::c_double,
    pub timeend: ::core::ffi::c_double,
    pub sectimeout: ::core::ffi::c_long,
    pub ex_status: *mut ::core::ffi::c_char,
    pub hBFP: *mut ::core::ffi::c_void,
    pub bfp_name: Option<BFPchar>,
    pub bfp_compatible: Option<BFPbool_lpintintint>,
    pub bfp_init: Option<BFPbool_lpintintchar>,
    pub bfp_free: Option<BFP_lp>,
    pub bfp_resize: Option<BFPbool_lpint>,
    pub bfp_memallocated: Option<BFPint_lp>,
    pub bfp_restart: Option<BFPbool_lp>,
    pub bfp_mustrefactorize: Option<BFPbool_lp>,
    pub bfp_preparefactorization: Option<BFPint_lp>,
    pub bfp_factorize: Option<BFPint_lpintintboolbool>,
    pub bfp_finishfactorization: Option<BFP_lp>,
    pub bfp_updaterefactstats: Option<BFP_lp>,
    pub bfp_prepareupdate: Option<BFPlreal_lpintintreal>,
    pub bfp_pivotRHS: Option<BFPreal_lplrealreal>,
    pub bfp_finishupdate: Option<BFPbool_lpbool>,
    pub bfp_ftran_prepare: Option<BFP_lprealint>,
    pub bfp_ftran_normal: Option<BFP_lprealint>,
    pub bfp_btran_normal: Option<BFP_lprealint>,
    pub bfp_btran_double: Option<BFP_lprealintrealint>,
    pub bfp_status: Option<BFPint_lp>,
    pub bfp_nonzeros: Option<BFPint_lpbool>,
    pub bfp_implicitslack: Option<BFPbool_lp>,
    pub bfp_indexbase: Option<BFPint_lp>,
    pub bfp_rowoffset: Option<BFPint_lp>,
    pub bfp_pivotmax: Option<BFPint_lp>,
    pub bfp_pivotalloc: Option<BFPbool_lpint>,
    pub bfp_colcount: Option<BFPint_lp>,
    pub bfp_canresetbasis: Option<BFPbool_lp>,
    pub bfp_efficiency: Option<BFPreal_lp>,
    pub bfp_pivotvector: Option<BFPrealp_lp>,
    pub bfp_pivotcount: Option<BFPint_lp>,
    pub bfp_refactcount: Option<BFPint_lpint>,
    pub bfp_isSetI: Option<BFPbool_lp>,
    pub bfp_findredundant: Option<BFPint_lpintrealcbintint>,
    pub hXLI: *mut ::core::ffi::c_void,
    pub xli_name: Option<XLIchar>,
    pub xli_compatible: Option<XLIbool_lpintintint>,
    pub xli_readmodel: Option<XLIbool_lpcharcharcharint>,
    pub xli_writemodel: Option<XLIbool_lpcharcharbool>,
    pub userabort: Option<userabortfunc>,
    pub report: Option<reportfunc>,
    pub explain: Option<explainfunc>,
    pub get_lpcolumn: Option<getvectorfunc>,
    pub get_basiscolumn: Option<getpackedfunc>,
    pub get_OF_active: Option<get_OF_activefunc>,
    pub getMDO: Option<getMDOfunc>,
    pub invert: Option<invertfunc>,
    pub set_action: Option<set_actionfunc>,
    pub is_action: Option<is_actionfunc>,
    pub clear_action: Option<clear_actionfunc>,
    pub ctrlc: Option<lphandle_intfunc>,
    pub ctrlchandle: *mut ::core::ffi::c_void,
    pub writelog: Option<lphandlestr_func>,
    pub loghandle: *mut ::core::ffi::c_void,
    pub debuginfo: Option<lphandlestr_func>,
    pub usermessage: Option<lphandleint_func>,
    pub msgmask: ::core::ffi::c_int,
    pub msghandle: *mut ::core::ffi::c_void,
    pub bb_usenode: Option<lphandleint_intfunc>,
    pub bb_nodehandle: *mut ::core::ffi::c_void,
    pub bb_usebranch: Option<lphandleint_intfunc>,
    pub bb_branchhandle: *mut ::core::ffi::c_void,
    pub rowcol_name: *mut ::core::ffi::c_char,
}
pub type lphandleint_intfunc = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_void,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type lprec = _lprec;
pub type lphandleint_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> ();
pub type lphandlestr_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_void, *mut ::core::ffi::c_char) -> ();
pub type lphandle_intfunc =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
pub type clear_actionfunc = unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> ();
pub type is_actionfunc =
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type set_actionfunc = unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> ();
pub type invertfunc = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type getMDOfunc = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_uchar,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_int;
pub type get_OF_activefunc = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_double;
pub type getpackedfunc = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int;
pub type getvectorfunc = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type explainfunc =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
pub type reportfunc =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int, *mut ::core::ffi::c_char) -> ();
pub type userabortfunc =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type XLIbool_lpcharcharbool = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type XLIbool_lpcharcharcharint = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type XLIbool_lpintintint = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type XLIchar = unsafe extern "C" fn() -> *mut ::core::ffi::c_char;
pub type BFPint_lpintrealcbintint = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    Option<getcolumnex_func>,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type getcolumnex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type BFPbool_lp = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type BFPint_lpint = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int;
pub type BFPint_lp = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type BFPrealp_lp = unsafe extern "C" fn(*mut lprec) -> *mut ::core::ffi::c_double;
pub type BFPreal_lp = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type BFPbool_lpint =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type BFPint_lpbool =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_int;
pub type BFP_lprealintrealint = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ();
pub type BFP_lprealint =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double, *mut ::core::ffi::c_int) -> ();
pub type BFPbool_lpbool =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
pub type BFPreal_lplrealreal = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double;
pub type BFPlreal_lpintintreal = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double;
pub type BFP_lp = unsafe extern "C" fn(*mut lprec) -> ();
pub type BFPint_lpintintboolbool = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_int;
pub type BFPbool_lpintintchar = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar;
pub type BFPbool_lpintintint = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type BFPchar = unsafe extern "C" fn() -> *mut ::core::ffi::c_char;
pub type workarraysrec = _workarraysrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _workarraysrec {
    pub lp: *mut lprec,
    pub size: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub vectorarray: *mut *mut ::core::ffi::c_char,
    pub vectorsize: *mut ::core::ffi::c_int,
}
pub type presolveundorec = _presolveundorec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _presolveundorec {
    pub lp: *mut lprec,
    pub orig_rows: ::core::ffi::c_int,
    pub orig_columns: ::core::ffi::c_int,
    pub orig_sum: ::core::ffi::c_int,
    pub var_to_orig: *mut ::core::ffi::c_int,
    pub orig_to_var: *mut ::core::ffi::c_int,
    pub fixed_rhs: *mut ::core::ffi::c_double,
    pub fixed_obj: *mut ::core::ffi::c_double,
    pub deletedA: *mut DeltaVrec,
    pub primalundo: *mut DeltaVrec,
    pub dualundo: *mut DeltaVrec,
    pub OFcolsdeleted: ::core::ffi::c_uchar,
}
pub type DeltaVrec = _DeltaVrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DeltaVrec {
    pub lp: *mut lprec,
    pub activelevel: ::core::ffi::c_int,
    pub tracker: *mut MATrec,
}
pub type MATrec = _MATrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MATrec {
    pub lp: *mut lprec,
    pub rows: ::core::ffi::c_int,
    pub columns: ::core::ffi::c_int,
    pub rows_alloc: ::core::ffi::c_int,
    pub columns_alloc: ::core::ffi::c_int,
    pub mat_alloc: ::core::ffi::c_int,
    pub col_mat_colnr: *mut ::core::ffi::c_int,
    pub col_mat_rownr: *mut ::core::ffi::c_int,
    pub col_mat_value: *mut ::core::ffi::c_double,
    pub col_end: *mut ::core::ffi::c_int,
    pub col_tag: *mut ::core::ffi::c_int,
    pub row_mat: *mut ::core::ffi::c_int,
    pub row_end: *mut ::core::ffi::c_int,
    pub row_tag: *mut ::core::ffi::c_int,
    pub colmax: *mut ::core::ffi::c_double,
    pub rowmax: *mut ::core::ffi::c_double,
    pub epsvalue: ::core::ffi::c_double,
    pub infnorm: ::core::ffi::c_double,
    pub dynrange: ::core::ffi::c_double,
    pub row_end_valid: ::core::ffi::c_uchar,
    pub is_roworder: ::core::ffi::c_uchar,
}
pub type BBPSrec = _BBPSrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BBPSrec {
    pub lp: *mut lprec,
    pub pseodotype: ::core::ffi::c_int,
    pub updatelimit: ::core::ffi::c_int,
    pub updatesfinished: ::core::ffi::c_int,
    pub restartlimit: ::core::ffi::c_double,
    pub UPcost: *mut MATitem,
    pub LOcost: *mut MATitem,
    pub secondary: *mut _BBPSrec,
}
pub type MATitem = _MATitem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MATitem {
    pub rownr: ::core::ffi::c_int,
    pub colnr: ::core::ffi::c_int,
    pub value: ::core::ffi::c_double,
}
pub type OBJmonrec = _OBJmonrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _OBJmonrec {
    pub lp: *mut lprec,
    pub oldpivstrategy: ::core::ffi::c_int,
    pub oldpivrule: ::core::ffi::c_int,
    pub pivrule: ::core::ffi::c_int,
    pub ruleswitches: ::core::ffi::c_int,
    pub limitstall: [::core::ffi::c_int; 2],
    pub limitruleswitches: ::core::ffi::c_int,
    pub idxstep: [::core::ffi::c_int; 5],
    pub countstep: ::core::ffi::c_int,
    pub startstep: ::core::ffi::c_int,
    pub currentstep: ::core::ffi::c_int,
    pub Rcycle: ::core::ffi::c_int,
    pub Ccycle: ::core::ffi::c_int,
    pub Ncycle: ::core::ffi::c_int,
    pub Mcycle: ::core::ffi::c_int,
    pub Icount: ::core::ffi::c_int,
    pub thisobj: ::core::ffi::c_double,
    pub prevobj: ::core::ffi::c_double,
    pub objstep: [::core::ffi::c_double; 5],
    pub thisinfeas: ::core::ffi::c_double,
    pub previnfeas: ::core::ffi::c_double,
    pub epsvalue: ::core::ffi::c_double,
    pub spxfunc: [::core::ffi::c_char; 10],
    pub pivdynamic: ::core::ffi::c_uchar,
    pub isdual: ::core::ffi::c_uchar,
    pub active: ::core::ffi::c_uchar,
}
pub type basisrec = _basisrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _basisrec {
    pub level: ::core::ffi::c_int,
    pub var_basic: *mut ::core::ffi::c_int,
    pub is_basic: *mut ::core::ffi::c_uchar,
    pub is_lower: *mut ::core::ffi::c_uchar,
    pub pivots: ::core::ffi::c_int,
    pub previous: *mut _basisrec,
}
pub type BBrec = _BBrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BBrec {
    pub parent: *mut _BBrec,
    pub child: *mut _BBrec,
    pub lp: *mut lprec,
    pub varno: ::core::ffi::c_int,
    pub vartype: ::core::ffi::c_int,
    pub lastvarcus: ::core::ffi::c_int,
    pub lastrcf: ::core::ffi::c_int,
    pub nodesleft: ::core::ffi::c_int,
    pub nodessolved: ::core::ffi::c_int,
    pub nodestatus: ::core::ffi::c_int,
    pub noderesult: ::core::ffi::c_double,
    pub lastsolution: ::core::ffi::c_double,
    pub sc_bound: ::core::ffi::c_double,
    pub upbo: *mut ::core::ffi::c_double,
    pub lowbo: *mut ::core::ffi::c_double,
    pub UPbound: ::core::ffi::c_double,
    pub LObound: ::core::ffi::c_double,
    pub UBtrack: ::core::ffi::c_int,
    pub LBtrack: ::core::ffi::c_int,
    pub contentmode: ::core::ffi::c_uchar,
    pub sc_canset: ::core::ffi::c_uchar,
    pub isSOS: ::core::ffi::c_uchar,
    pub isGUB: ::core::ffi::c_uchar,
    pub varmanaged: *mut ::core::ffi::c_int,
    pub isfloor: ::core::ffi::c_uchar,
    pub UBzerobased: ::core::ffi::c_uchar,
}
pub type INVrec = _INVrec;
pub type multirec = _multirec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _multirec {
    pub lp: *mut lprec,
    pub size: ::core::ffi::c_int,
    pub used: ::core::ffi::c_int,
    pub limit: ::core::ffi::c_int,
    pub items: *mut pricerec,
    pub freeList: *mut ::core::ffi::c_int,
    pub sortedList: *mut QSORTrec,
    pub stepList: *mut ::core::ffi::c_double,
    pub valueList: *mut ::core::ffi::c_double,
    pub indexSet: *mut ::core::ffi::c_int,
    pub active: ::core::ffi::c_int,
    pub retries: ::core::ffi::c_int,
    pub step_base: ::core::ffi::c_double,
    pub step_last: ::core::ffi::c_double,
    pub obj_base: ::core::ffi::c_double,
    pub obj_last: ::core::ffi::c_double,
    pub epszero: ::core::ffi::c_double,
    pub maxpivot: ::core::ffi::c_double,
    pub maxbound: ::core::ffi::c_double,
    pub sorted: ::core::ffi::c_uchar,
    pub truncinf: ::core::ffi::c_uchar,
    pub objcheck: ::core::ffi::c_uchar,
    pub dirty: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union QSORTrec {
    pub pvoid2: QSORTrec1,
    pub pvoidreal: QSORTrec2,
    pub pvoidint2: QSORTrec3,
    pub realint2: QSORTrec4,
    pub reallong: QSORTrec5,
    pub real2: QSORTrec6,
    pub int4: QSORTrec7,
}
pub type QSORTrec7 = _QSORTrec7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec7 {
    pub intval: ::core::ffi::c_int,
    pub intpar1: ::core::ffi::c_int,
    pub intpar2: ::core::ffi::c_int,
    pub intpar3: ::core::ffi::c_int,
}
pub type QSORTrec6 = _QSORTrec6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec6 {
    pub realval: ::core::ffi::c_double,
    pub realpar1: ::core::ffi::c_double,
}
pub type QSORTrec5 = _QSORTrec5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec5 {
    pub realval: ::core::ffi::c_double,
    pub longval: ::core::ffi::c_long,
}
pub type QSORTrec4 = _QSORTrec4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec4 {
    pub realval: ::core::ffi::c_double,
    pub intval: ::core::ffi::c_int,
    pub intpar1: ::core::ffi::c_int,
}
pub type QSORTrec3 = _QSORTrec3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec3 {
    pub ptr: *mut ::core::ffi::c_void,
    pub intval: ::core::ffi::c_int,
    pub intpar1: ::core::ffi::c_int,
}
pub type QSORTrec2 = _QSORTrec2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec2 {
    pub ptr: *mut ::core::ffi::c_void,
    pub realval: ::core::ffi::c_double,
}
pub type QSORTrec1 = _QSORTrec1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QSORTrec1 {
    pub ptr: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
pub type pricerec = _pricerec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pricerec {
    pub theta: ::core::ffi::c_double,
    pub pivot: ::core::ffi::c_double,
    pub epspivot: ::core::ffi::c_double,
    pub varno: ::core::ffi::c_int,
    pub lp: *mut lprec,
    pub isdual: ::core::ffi::c_uchar,
}
pub type SOSgroup = _SOSgroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SOSgroup {
    pub lp: *mut lprec,
    pub sos_list: *mut *mut SOSrec,
    pub sos_alloc: ::core::ffi::c_int,
    pub sos_count: ::core::ffi::c_int,
    pub maxorder: ::core::ffi::c_int,
    pub sos1_count: ::core::ffi::c_int,
    pub membership: *mut ::core::ffi::c_int,
    pub memberpos: *mut ::core::ffi::c_int,
}
pub type SOSrec = _SOSrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SOSrec {
    pub parent: *mut SOSgroup,
    pub tagorder: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub isGUB: ::core::ffi::c_uchar,
    pub size: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub members: *mut ::core::ffi::c_int,
    pub weights: *mut ::core::ffi::c_double,
    pub membersSorted: *mut ::core::ffi::c_int,
    pub membersMapped: *mut ::core::ffi::c_int,
}
pub type partialrec = _partialrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _partialrec {
    pub lp: *mut lprec,
    pub blockcount: ::core::ffi::c_int,
    pub blocknow: ::core::ffi::c_int,
    pub blockend: *mut ::core::ffi::c_int,
    pub blockpos: *mut ::core::ffi::c_int,
    pub isrow: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable {
    pub table: *mut *mut hashelem,
    pub size: ::core::ffi::c_int,
    pub base: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub first: *mut _hashelem,
    pub last: *mut _hashelem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hashelem {
    pub name: *mut ::core::ffi::c_char,
    pub index: ::core::ffi::c_int,
    pub next: *mut _hashelem,
    pub nextelem: *mut _hashelem,
}
pub type hashelem = _hashelem;
pub type write_params_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar;
pub type write_XLI_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type write_freeMPS_func = unsafe extern "C" fn(*mut lprec, *mut FILE) -> ::core::ffi::c_uchar;
pub type write_freemps_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type write_MPS_func = unsafe extern "C" fn(*mut lprec, *mut FILE) -> ::core::ffi::c_uchar;
pub type write_mps_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type write_LP_func = unsafe extern "C" fn(*mut lprec, *mut FILE) -> ::core::ffi::c_uchar;
pub type write_lp_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type unscale_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type time_elapsed_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type str_set_rh_vec_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type str_set_obj_fn_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type str_add_lag_con_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type str_add_constraint_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type str_add_column_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type solve_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type set_XLI_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type set_verbose_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_var_weights_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type set_var_branch_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type set_use_names_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar, ::core::ffi::c_uchar) -> ();
pub type set_upbo_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_trace_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_timeout_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_long) -> ();
pub type set_solutionlimit_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_simplextype_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_sense_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_semicont_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type set_scaling_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_scalelimit_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_row_name_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar;
pub type set_rowex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type set_row_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_rh_vec_func = unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ();
pub type set_rh_range_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_rh_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_pseudocosts_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type set_print_sol_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_presolve_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int, ::core::ffi::c_int) -> ();
pub type set_preferdual_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_pivoting_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_partialprice_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type set_outputstream_func = unsafe extern "C" fn(*mut lprec, *mut FILE) -> ();
pub type set_outputfile_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type set_obj_in_basis_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_obj_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_obj_fnex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type set_obj_fn_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type set_obj_bound_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_negrange_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_multiprice_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type set_mip_gap_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar, ::core::ffi::c_double) -> ();
pub type set_minim_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type set_maxpivot_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_maxim_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type set_mat_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_lp_name_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type set_lowbo_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_lag_trace_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_int_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type set_infinite_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_improve_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_unbounded_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type set_epspivot_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_epsperturb_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_epslevel_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type set_epsint_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_epsel_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_epsd_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_epsb_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_debug_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_constr_type_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type set_col_name_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar;
pub type set_columnex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type set_column_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_break_at_value_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type set_break_at_first_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_bounds_tighter_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type set_bounds_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type set_binary_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type set_BFP_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
pub type set_bb_rule_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_bb_floorfirst_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_bb_depthlimit_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_basiscrash_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_basis_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type set_basisvar_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int;
pub type set_anti_degen_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type set_add_rowmode_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
pub type resize_lp_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type reset_params_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type reset_basis_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type read_basis_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar;
pub type read_params_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar;
pub type read_XLI_func = unsafe extern "C" fn(
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
) -> *mut lprec;
pub type read_MPS_func =
    unsafe extern "C" fn(*mut ::core::ffi::c_char, ::core::ffi::c_int) -> *mut lprec;
pub type read_LP_func = unsafe extern "C" fn(
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_char,
) -> *mut lprec;
pub type put_msgfunc_func = unsafe extern "C" fn(
    *mut lprec,
    Option<lphandleint_func>,
    *mut ::core::ffi::c_void,
    ::core::ffi::c_int,
) -> ();
pub type put_logfunc_func =
    unsafe extern "C" fn(*mut lprec, Option<lphandlestr_func>, *mut ::core::ffi::c_void) -> ();
pub type put_bb_branchfunc_func =
    unsafe extern "C" fn(*mut lprec, Option<lphandleint_intfunc>, *mut ::core::ffi::c_void) -> ();
pub type put_bb_nodefunc_func =
    unsafe extern "C" fn(*mut lprec, Option<lphandleint_intfunc>, *mut ::core::ffi::c_void) -> ();
pub type put_abortfunc_func =
    unsafe extern "C" fn(*mut lprec, Option<lphandle_intfunc>, *mut ::core::ffi::c_void) -> ();
pub type print_tableau_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type print_str_func = unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ();
pub type print_solution_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type print_scales_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type print_objective_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type print_lp_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type print_duals_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type print_constraints_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
pub type make_lp_func = unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> *mut lprec;
pub type lp_solve_version_func = unsafe extern "C" fn(
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> ();
pub type is_use_names_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
pub type is_unbounded_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_trace_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_SOS_var_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_semicont_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_scaletype_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_scalemode_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_presolve_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_piv_rule_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_piv_mode_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_obj_in_basis_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_negative_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_nativeXLI_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_nativeBFP_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_maxim_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_lag_trace_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_integerscaling_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_int_func = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_infinite_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type is_feasible_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type is_debug_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_constr_type_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type is_break_at_first_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type is_binary_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_anti_degen_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type is_add_rowmode_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type has_XLI_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type has_BFP_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type get_working_objective_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_verbose_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_variables_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_var_priority_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int;
pub type get_var_primalresult_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double;
pub type get_var_dualresult_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double;
pub type get_var_branch_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int;
pub type get_upbo_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double;
pub type get_total_nodes_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_longlong;
pub type get_total_iter_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_longlong;
pub type get_timeout_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_long;
pub type get_statustext_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
pub type get_status_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_solutionlimit_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_solutioncount_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_simplextype_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_sensitivity_rhs_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type get_sensitivity_objex_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type get_sensitivity_obj_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type get_scaling_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_scalelimit_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_row_name_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
pub type get_rowex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type get_row_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type get_rh_range_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double;
pub type get_rh_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double;
pub type get_ptr_variables_func =
    unsafe extern "C" fn(*mut lprec, *mut *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_ptr_sensitivity_rhs_func = unsafe extern "C" fn(
    *mut lprec,
    *mut *mut ::core::ffi::c_double,
    *mut *mut ::core::ffi::c_double,
    *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type get_ptr_sensitivity_objex_func = unsafe extern "C" fn(
    *mut lprec,
    *mut *mut ::core::ffi::c_double,
    *mut *mut ::core::ffi::c_double,
    *mut *mut ::core::ffi::c_double,
    *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type get_ptr_sensitivity_obj_func = unsafe extern "C" fn(
    *mut lprec,
    *mut *mut ::core::ffi::c_double,
    *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type get_ptr_primal_solution_func =
    unsafe extern "C" fn(*mut lprec, *mut *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_ptr_lambda_func =
    unsafe extern "C" fn(*mut lprec, *mut *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_ptr_dual_solution_func =
    unsafe extern "C" fn(*mut lprec, *mut *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_ptr_constraints_func =
    unsafe extern "C" fn(*mut lprec, *mut *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_pseudocosts_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type get_print_sol_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_primal_solution_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_presolveloops_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_presolve_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_pivoting_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_partialprice_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ();
pub type get_origrow_name_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
pub type get_origcol_name_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
pub type get_orig_index_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int;
pub type get_objective_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_obj_bound_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_Nrows_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_Norig_rows_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_Norig_columns_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_nz_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_negrange_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_Ncolumns_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_nameindex_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_int;
pub type get_multiprice_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_int;
pub type get_mip_gap_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_double;
pub type get_maxpivot_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_max_level_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_mat_byindex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_double;
pub type get_mat_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_double;
pub type get_Lrows_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_lp_name_func = unsafe extern "C" fn(*mut lprec) -> *mut ::core::ffi::c_char;
pub type get_lp_index_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int;
pub type get_lowbo_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double;
pub type get_lambda_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_infinite_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_improve_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_epspivot_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_epsperturb_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_epsint_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_epsel_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_epsd_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_epsb_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_dual_solution_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_constraints_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
pub type get_constr_value_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double;
pub type get_constr_type_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int;
pub type get_columnex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type get_col_name_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
pub type get_break_at_value_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type get_bounds_tighter_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type get_bb_rule_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_bb_floorfirst_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_bb_depthlimit_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_basiscrash_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type get_basis_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_int,
    ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar;
pub type get_anti_degen_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
pub type free_lp_func = unsafe extern "C" fn(*mut *mut lprec) -> ();
pub type dualize_lp_func = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type delete_lp_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type del_constraint_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type del_column_func =
    unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar;
pub type default_basis_func = unsafe extern "C" fn(*mut lprec) -> ();
pub type copy_lp_func = unsafe extern "C" fn(*mut lprec) -> *mut lprec;
pub type column_in_lp_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
pub type add_SOS_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int;
pub type add_lag_con_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type add_constraintex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type add_constraint_func = unsafe extern "C" fn(
    *mut lprec,
    *mut ::core::ffi::c_double,
    ::core::ffi::c_int,
    ::core::ffi::c_double,
) -> ::core::ffi::c_uchar;
pub type add_columnex_func = unsafe extern "C" fn(
    *mut lprec,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_double,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar;
pub type add_column_func =
    unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar;
#[export_name="honest_lpsolve_scaled_value"]
pub unsafe extern "C" fn scaled_value(
    mut lp: *mut lprec,
    mut value: ::core::ffi::c_double,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if fabs(value) < (*lp).infinite {
        if (*lp).scaling_used != 0 {
            if index > (*lp).rows {
                value /= *(*lp).scalars.offset(index as isize);
            } else {
                value *= *(*lp).scalars.offset(index as isize);
            }
        }
    } else {
        value = (if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        }) as ::core::ffi::c_double
            * (*lp).infinite;
    }
    return value;
}
#[export_name="honest_lpsolve_unscaled_value"]
pub unsafe extern "C" fn unscaled_value(
    mut lp: *mut lprec,
    mut value: ::core::ffi::c_double,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if fabs(value) < (*lp).infinite {
        if (*lp).scaling_used != 0 {
            if index > (*lp).rows {
                value *= *(*lp).scalars.offset(index as isize);
            } else {
                value /= *(*lp).scalars.offset(index as isize);
            }
        }
    } else {
        value = (if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        }) as ::core::ffi::c_double
            * (*lp).infinite;
    }
    return value;
}
#[export_name="honest_lpsolve_scaled_mat"]
pub unsafe extern "C" fn scaled_mat(
    mut lp: *mut lprec,
    mut value: ::core::ffi::c_double,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if (*lp).scaling_used != 0 {
        value *= *(*lp).scalars.offset(rownr as isize)
            * *(*lp).scalars.offset(((*lp).rows + colnr) as isize);
    }
    return value;
}
#[export_name="honest_lpsolve_unscaled_mat"]
pub unsafe extern "C" fn unscaled_mat(
    mut lp: *mut lprec,
    mut value: ::core::ffi::c_double,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if (*lp).scaling_used != 0 {
        value /= *(*lp).scalars.offset(rownr as isize)
            * *(*lp).scalars.offset(((*lp).rows + colnr) as isize);
    }
    return value;
}
#[export_name="honest_lpsolve_CurtisReidMeasure"]
pub unsafe extern "C" fn CurtisReidMeasure(
    mut lp: *mut lprec,
    mut _Advanced: ::core::ffi::c_uchar,
    mut FRowScale: *mut ::core::ffi::c_double,
    mut FColScale: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut absvalue: ::core::ffi::c_double = 0.;
    let mut logvalue: ::core::ffi::c_double = 0.;
    let mut result: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    result = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        absvalue = fabs(*(*lp).orig_obj.offset(i as isize));
        if absvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            logvalue = log(absvalue);
            if _Advanced != 0 {
                logvalue -= *FRowScale.offset(0 as ::core::ffi::c_int as isize)
                    + *FColScale.offset(i as isize);
            }
            result += logvalue * logvalue;
        }
        i += 1;
    }
    mat_validate(mat);
    value = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    rownr = (*mat)
        .col_mat_rownr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    colnr = (*mat)
        .col_mat_colnr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    nz = get_nonzeros(lp);
    i = 0 as ::core::ffi::c_int;
    while i < nz {
        absvalue = fabs(*value);
        if absvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            logvalue = log(absvalue);
            if _Advanced != 0 {
                logvalue -= *FRowScale.offset(*rownr as isize) + *FColScale.offset(*colnr as isize);
            }
            result += logvalue * logvalue;
        }
        i += 1;
        value = value.offset(matValueStep as isize);
        rownr = rownr.offset(matRowColStep as isize);
        colnr = colnr.offset(matRowColStep as isize);
    }
    return result;
}
#[export_name="honest_lpsolve_CurtisReidScales"]
pub unsafe extern "C" fn CurtisReidScales(
    mut lp: *mut lprec,
    mut _Advanced: ::core::ffi::c_uchar,
    mut FRowScale: *mut ::core::ffi::c_double,
    mut FColScale: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut ent: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut RowScalem2: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut ColScalem2: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut RowSum: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut ColSum: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut residual_even: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut residual_odd: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut sk: ::core::ffi::c_double = 0.;
    let mut qk: ::core::ffi::c_double = 0.;
    let mut ek: ::core::ffi::c_double = 0.;
    let mut skm1: ::core::ffi::c_double = 0.;
    let mut qkm1: ::core::ffi::c_double = 0.;
    let mut ekm1: ::core::ffi::c_double = 0.;
    let mut qkqkm1: ::core::ffi::c_double = 0.;
    let mut ekekm1: ::core::ffi::c_double = 0.;
    let mut absvalue: ::core::ffi::c_double = 0.;
    let mut logvalue: ::core::ffi::c_double = 0.;
    let mut StopTolerance: ::core::ffi::c_double = 0.;
    let mut RowCount: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut ColCount: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colMax: ::core::ffi::c_int = 0;
    let mut Result: ::core::ffi::c_int = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if CurtisReidMeasure(lp, _Advanced, FRowScale, FColScale)
        < 0.1f64 * get_nonzeros(lp) as ::core::ffi::c_double
    {
        return 0 as ::core::ffi::c_int;
    }
    nz = get_nonzeros(lp);
    colMax = (*lp).columns;
    allocREAL(
        lp,
        &raw mut RowSum,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut RowCount,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut residual_odd,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut ColSum,
        colMax + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut ColCount,
        colMax + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut residual_even,
        colMax + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut RowScalem2,
        (*lp).rows + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut ColScalem2,
        colMax + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= colMax {
        absvalue = fabs(*(*lp).orig_obj.offset(i as isize));
        if absvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            logvalue = log(absvalue);
            *ColSum.offset(i as isize) += logvalue;
            *RowSum.offset(0 as ::core::ffi::c_int as isize) += logvalue;
            let ref mut fresh0 = *ColCount.offset(i as isize);
            *fresh0 += 1;
            let ref mut fresh1 = *RowCount.offset(0 as ::core::ffi::c_int as isize);
            *fresh1 += 1;
        }
        i += 1;
    }
    value = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    rownr = (*mat)
        .col_mat_rownr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    colnr = (*mat)
        .col_mat_colnr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nz {
        absvalue = fabs(*value);
        if absvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            logvalue = log(absvalue);
            *ColSum.offset(*colnr as isize) += logvalue;
            *RowSum.offset(*rownr as isize) += logvalue;
            let ref mut fresh2 = *ColCount.offset(*colnr as isize);
            *fresh2 += 1;
            let ref mut fresh3 = *RowCount.offset(*rownr as isize);
            *fresh3 += 1;
        }
        i += 1;
        value = value.offset(matValueStep as isize);
        rownr = rownr.offset(matRowColStep as isize);
        colnr = colnr.offset(matRowColStep as isize);
    }
    row = 0 as ::core::ffi::c_int;
    while row <= (*lp).rows {
        if *RowCount.offset(row as isize) == 0 as ::core::ffi::c_int {
            *RowCount.offset(row as isize) = 1 as ::core::ffi::c_int;
        }
        row += 1;
    }
    col = 1 as ::core::ffi::c_int;
    while col <= colMax {
        if *ColCount.offset(col as isize) == 0 as ::core::ffi::c_int {
            *ColCount.offset(col as isize) = 1 as ::core::ffi::c_int;
        }
        col += 1;
    }
    StopTolerance = if (*lp).scalelimit - floor((*lp).scalelimit) > 1.0e-02f64 {
        (*lp).scalelimit - floor((*lp).scalelimit)
    } else {
        1.0e-02f64
    };
    StopTolerance *= nz as ::core::ffi::c_double;
    row = 0 as ::core::ffi::c_int;
    while row <= (*lp).rows {
        *FRowScale.offset(row as isize) =
            *RowSum.offset(row as isize) / *RowCount.offset(row as isize) as ::core::ffi::c_double;
        *RowScalem2.offset(row as isize) = *FRowScale.offset(row as isize);
        row += 1;
    }
    col = 1 as ::core::ffi::c_int;
    while col <= colMax {
        *FColScale.offset(col as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *ColScalem2.offset(col as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *residual_even.offset(col as isize) = *ColSum.offset(col as isize);
        if *(*lp).orig_obj.offset(col as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *residual_even.offset(col as isize) -= *RowSum.offset(0 as ::core::ffi::c_int as isize)
                / *RowCount.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_double;
        }
        i = *(*mat)
            .col_end
            .offset((col - 1 as ::core::ffi::c_int) as isize);
        rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
        ent = *(*mat).col_end.offset(col as isize);
        while i < ent {
            *residual_even.offset(col as isize) -= *RowSum.offset(*rownr as isize)
                / *RowCount.offset(*rownr as isize) as ::core::ffi::c_double;
            i += 1;
            rownr = rownr.offset(matRowColStep as isize);
        }
        col += 1;
    }
    sk = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    skm1 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    col = 1 as ::core::ffi::c_int;
    while col <= colMax {
        sk += *residual_even.offset(col as isize) * *residual_even.offset(col as isize)
            / *ColCount.offset(col as isize) as ::core::ffi::c_double;
        col += 1;
    }
    Result = 0 as ::core::ffi::c_int;
    qk = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    qkm1 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    ek = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    ekm1 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    while sk > StopTolerance {
        qkqkm1 = qk * qkm1;
        ekekm1 = ek * ekm1;
        if Result % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if Result != 0 as ::core::ffi::c_int {
                row = 0 as ::core::ffi::c_int;
                while row <= (*lp).rows {
                    *RowScalem2.offset(row as isize) = *FRowScale.offset(row as isize);
                    row += 1;
                }
                if qkqkm1 != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    row = 0 as ::core::ffi::c_int;
                    while row <= (*lp).rows {
                        *FRowScale.offset(row as isize) *=
                            1 as ::core::ffi::c_int as ::core::ffi::c_double + ekekm1 / qkqkm1;
                        row += 1;
                    }
                    row = 0 as ::core::ffi::c_int;
                    while row <= (*lp).rows {
                        *FRowScale.offset(row as isize) += *residual_odd.offset(row as isize)
                            / (qkqkm1 * *RowCount.offset(row as isize) as ::core::ffi::c_double)
                            - *RowScalem2.offset(row as isize) * ekekm1 / qkqkm1;
                        row += 1;
                    }
                }
            }
        } else {
            col = 1 as ::core::ffi::c_int;
            while col <= colMax {
                *ColScalem2.offset(col as isize) = *FColScale.offset(col as isize);
                col += 1;
            }
            if qkqkm1 != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                col = 1 as ::core::ffi::c_int;
                while col <= colMax {
                    *FColScale.offset(col as isize) *=
                        1 as ::core::ffi::c_int as ::core::ffi::c_double + ekekm1 / qkqkm1;
                    col += 1;
                }
                col = 1 as ::core::ffi::c_int;
                while col <= colMax {
                    *FColScale.offset(col as isize) += *residual_even.offset(col as isize)
                        / (*ColCount.offset(col as isize) as ::core::ffi::c_double * qkqkm1)
                        - *ColScalem2.offset(col as isize) * ekekm1 / qkqkm1;
                    col += 1;
                }
            }
        }
        if Result % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            row = 0 as ::core::ffi::c_int;
            while row <= (*lp).rows {
                *residual_odd.offset(row as isize) *= ek;
                row += 1;
            }
            i = 1 as ::core::ffi::c_int;
            while i <= colMax {
                if *(*lp).orig_obj.offset(i as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *residual_odd.offset(0 as ::core::ffi::c_int as isize) += *residual_even
                        .offset(i as isize)
                        / *ColCount.offset(i as isize) as ::core::ffi::c_double;
                }
                i += 1;
            }
            rownr = (*mat)
                .col_mat_rownr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            colnr = (*mat)
                .col_mat_colnr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < nz {
                *residual_odd.offset(*rownr as isize) += *residual_even.offset(*colnr as isize)
                    / *ColCount.offset(*colnr as isize) as ::core::ffi::c_double;
                i += 1;
                rownr = rownr.offset(matRowColStep as isize);
                colnr = colnr.offset(matRowColStep as isize);
            }
            row = 0 as ::core::ffi::c_int;
            while row <= (*lp).rows {
                *residual_odd.offset(row as isize) *=
                    -(1 as ::core::ffi::c_int) as ::core::ffi::c_double / qk;
                row += 1;
            }
            skm1 = sk;
            sk = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            row = 0 as ::core::ffi::c_int;
            while row <= (*lp).rows {
                sk += *residual_odd.offset(row as isize) * *residual_odd.offset(row as isize)
                    / *RowCount.offset(row as isize) as ::core::ffi::c_double;
                row += 1;
            }
        } else {
            col = 1 as ::core::ffi::c_int;
            while col <= colMax {
                *residual_even.offset(col as isize) *= ek;
                col += 1;
            }
            i = 1 as ::core::ffi::c_int;
            while i <= colMax {
                if *(*lp).orig_obj.offset(i as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *residual_even.offset(i as isize) += *residual_odd
                        .offset(0 as ::core::ffi::c_int as isize)
                        / *RowCount.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_double;
                }
                i += 1;
            }
            rownr = (*mat)
                .col_mat_rownr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            colnr = (*mat)
                .col_mat_colnr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < nz {
                *residual_even.offset(*colnr as isize) += *residual_odd.offset(*rownr as isize)
                    / *RowCount.offset(*rownr as isize) as ::core::ffi::c_double;
                i += 1;
                rownr = rownr.offset(matRowColStep as isize);
                colnr = colnr.offset(matRowColStep as isize);
            }
            col = 1 as ::core::ffi::c_int;
            while col <= colMax {
                *residual_even.offset(col as isize) *=
                    -(1 as ::core::ffi::c_int) as ::core::ffi::c_double / qk;
                col += 1;
            }
            skm1 = sk;
            sk = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            col = 1 as ::core::ffi::c_int;
            while col <= colMax {
                sk += *residual_even.offset(col as isize) * *residual_even.offset(col as isize)
                    / *ColCount.offset(col as isize) as ::core::ffi::c_double;
                col += 1;
            }
        }
        ekm1 = ek;
        ek = qk * sk / skm1;
        qkm1 = qk;
        qk = 1 as ::core::ffi::c_int as ::core::ffi::c_double - ek;
        Result += 1;
    }
    ekekm1 = ek * ekm1;
    if qkm1 != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if Result % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            row = 0 as ::core::ffi::c_int;
            while row <= (*lp).rows {
                *FRowScale.offset(row as isize) *= 1.0f64 + ekekm1 / qkm1;
                row += 1;
            }
            row = 0 as ::core::ffi::c_int;
            while row <= (*lp).rows {
                *FRowScale.offset(row as isize) += *residual_odd.offset(row as isize)
                    / (qkm1 * *RowCount.offset(row as isize) as ::core::ffi::c_double)
                    - *RowScalem2.offset(row as isize) * ekekm1 / qkm1;
                row += 1;
            }
        } else {
            col = 1 as ::core::ffi::c_int;
            while col <= colMax {
                *FColScale.offset(col as isize) *=
                    1 as ::core::ffi::c_int as ::core::ffi::c_double + ekekm1 / qkm1;
                col += 1;
            }
            col = 1 as ::core::ffi::c_int;
            while col <= colMax {
                *FColScale.offset(col as isize) += *residual_even.offset(col as isize)
                    / (*ColCount.offset(col as isize) as ::core::ffi::c_double * qkm1)
                    - *ColScalem2.offset(col as isize) * ekekm1 / qkm1;
                col += 1;
            }
        }
    }
    if FALSE != 0 && mat_validate(mat) as ::core::ffi::c_int != 0 {
        let mut check: ::core::ffi::c_double = 0.;
        let mut error: ::core::ffi::c_double = 0.;
        error = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        row = 0 as ::core::ffi::c_int;
        while row <= (*lp).rows {
            check = *RowCount.offset(row as isize) as ::core::ffi::c_double
                * *FRowScale.offset(row as isize);
            if row == 0 as ::core::ffi::c_int {
                i = 1 as ::core::ffi::c_int;
                while i <= colMax {
                    if *(*lp).orig_obj.offset(i as isize)
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        check += *FColScale.offset(i as isize);
                    }
                    i += 1;
                }
            } else {
                i = *(*mat)
                    .row_end
                    .offset((row - 1 as ::core::ffi::c_int) as isize);
                ent = *(*mat).row_end.offset(row as isize);
                while i < ent {
                    col = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(i as isize) as isize);
                    check += *FColScale.offset(col as isize);
                    i += 1;
                }
            }
            check -= *RowSum.offset(row as isize);
            error += check * check;
            row += 1;
        }
        error = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        col = 1 as ::core::ffi::c_int;
        while col <= colMax {
            check = *ColCount.offset(col as isize) as ::core::ffi::c_double
                * *FColScale.offset(col as isize);
            if *(*lp).orig_obj.offset(col as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                check += *FRowScale.offset(0 as ::core::ffi::c_int as isize);
            }
            i = *(*mat)
                .col_end
                .offset((col - 1 as ::core::ffi::c_int) as isize);
            ent = *(*mat).col_end.offset(col as isize);
            rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
            while i < ent {
                check += *FRowScale.offset(*rownr as isize);
                i += 1;
                rownr = rownr.offset(matRowColStep as isize);
            }
            check -= *ColSum.offset(col as isize);
            error += check * check;
            col += 1;
        }
    }
    col = 1 as ::core::ffi::c_int;
    while col <= colMax {
        absvalue = exp(-*FColScale.offset(col as isize));
        if absvalue < MIN_SCALAR {
            absvalue = MIN_SCALAR;
        }
        if absvalue > MAX_SCALAR {
            absvalue = MAX_SCALAR;
        }
        if is_int(lp, col) == 0 || is_integerscaling(lp) as ::core::ffi::c_int != 0 {
            *FColScale.offset(col as isize) = absvalue;
        } else {
            *FColScale.offset(col as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        col += 1;
    }
    row = 0 as ::core::ffi::c_int;
    while row <= (*lp).rows {
        absvalue = exp(-*FRowScale.offset(row as isize));
        if absvalue < MIN_SCALAR {
            absvalue = MIN_SCALAR;
        }
        if absvalue > MAX_SCALAR {
            absvalue = MAX_SCALAR;
        }
        *FRowScale.offset(row as isize) = absvalue;
        row += 1;
    }
    if !(RowSum as *mut ::core::ffi::c_void).is_null() {
        free(RowSum as *mut ::core::ffi::c_void);
        RowSum = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(ColSum as *mut ::core::ffi::c_void).is_null() {
        free(ColSum as *mut ::core::ffi::c_void);
        ColSum = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(RowCount as *mut ::core::ffi::c_void).is_null() {
        free(RowCount as *mut ::core::ffi::c_void);
        RowCount = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(ColCount as *mut ::core::ffi::c_void).is_null() {
        free(ColCount as *mut ::core::ffi::c_void);
        ColCount = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(residual_even as *mut ::core::ffi::c_void).is_null() {
        free(residual_even as *mut ::core::ffi::c_void);
        residual_even = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(residual_odd as *mut ::core::ffi::c_void).is_null() {
        free(residual_odd as *mut ::core::ffi::c_void);
        residual_odd = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(RowScalem2 as *mut ::core::ffi::c_void).is_null() {
        free(RowScalem2 as *mut ::core::ffi::c_void);
        RowScalem2 = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(ColScalem2 as *mut ::core::ffi::c_void).is_null() {
        free(ColScalem2 as *mut ::core::ffi::c_void);
        ColScalem2 = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return Result;
}
#[export_name="honest_lpsolve_scaleCR"]
pub unsafe extern "C" fn scaleCR(
    mut lp: *mut lprec,
    mut scaledelta: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut scalechange: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut Result: ::core::ffi::c_int = 0;
    if (*lp).scaling_used == 0 {
        allocREAL(
            lp,
            &raw mut (*lp).scalars,
            (*lp).sum_alloc + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        Result = 0 as ::core::ffi::c_int;
        while Result <= (*lp).sum {
            *(*lp).scalars.offset(Result as isize) =
                1 as ::core::ffi::c_int as ::core::ffi::c_double;
            Result += 1;
        }
        (*lp).scaling_used = TRUE as ::core::ffi::c_uchar;
    }
    if scaledelta.is_null() {
        allocREAL(
            lp,
            &raw mut scalechange,
            (*lp).sum + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
    } else {
        scalechange = scaledelta;
    }
    Result = CurtisReidScales(
        lp,
        FALSE as ::core::ffi::c_uchar,
        scalechange,
        scalechange.offset((*lp).rows as isize) as *mut ::core::ffi::c_double,
    );
    if Result > 0 as ::core::ffi::c_int {
        if scale_updaterows(lp, scalechange, TRUE as ::core::ffi::c_uchar) as ::core::ffi::c_int
            != 0
            || scale_updatecolumns(
                lp,
                scalechange.offset((*lp).rows as isize) as *mut ::core::ffi::c_double,
                TRUE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0
        {
            (*lp).scalemode |= SCALE_CURTISREID;
        }
        set_action(
            &raw mut (*lp).spx_action,
            ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
        );
    }
    if scaledelta.is_null() {
        if !(scalechange as *mut ::core::ffi::c_void).is_null() {
            free(scalechange as *mut ::core::ffi::c_void);
            scalechange = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    }
    return (Result > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_transform_for_scale"]
pub unsafe extern "C" fn transform_for_scale(
    mut lp: *mut lprec,
    mut value: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut Accept: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    *value = fabs(*value);
    if is_scalemode(lp, SCALE_LOGARITHMIC) != 0 {
        *value = log(*value);
    } else if is_scalemode(lp, SCALE_QUADRATIC) != 0 {
        *value *= *value;
    }
    return Accept;
}
#[export_name="honest_lpsolve_accumulate_for_scale"]
pub unsafe extern "C" fn accumulate_for_scale(
    mut lp: *mut lprec,
    mut min: *mut ::core::ffi::c_double,
    mut max: *mut ::core::ffi::c_double,
    mut value: ::core::ffi::c_double,
) {
    if transform_for_scale(lp, &raw mut value) != 0 {
        if is_scaletype(lp, SCALE_MEAN) != 0 {
            *max += value;
            *min += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            if *max < value {
                *max = value;
            }
            if *min > value {
                *min = value;
            }
        }
    }
}
#[export_name="honest_lpsolve_minmax_to_scale"]
pub unsafe extern "C" fn minmax_to_scale(
    mut lp: *mut lprec,
    mut min: ::core::ffi::c_double,
    mut max: ::core::ffi::c_double,
    mut itemcount: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut scale_0: ::core::ffi::c_double = 0.;
    if is_scalemode(lp, SCALE_LOGARITHMIC) != 0 {
        scale_0 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        scale_0 = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if itemcount <= 0 as ::core::ffi::c_int {
        return scale_0;
    }
    if is_scaletype(lp, SCALE_MEAN) != 0 {
        if min > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            scale_0 = max / min;
        }
    } else if is_scaletype(lp, SCALE_RANGE) != 0 {
        scale_0 = (max + min) / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if is_scaletype(lp, SCALE_GEOMETRIC) != 0 {
        scale_0 = sqrt(min * max);
    } else if is_scaletype(lp, SCALE_EXTREME) != 0 {
        scale_0 = max;
    }
    if is_scalemode(lp, SCALE_LOGARITHMIC) != 0 {
        scale_0 = exp(-scale_0);
    } else if is_scalemode(lp, SCALE_QUADRATIC) != 0 {
        if scale_0 == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            scale_0 = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            scale_0 = 1 as ::core::ffi::c_int as ::core::ffi::c_double / sqrt(scale_0);
        }
    } else if scale_0 == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        scale_0 = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        scale_0 = 1 as ::core::ffi::c_int as ::core::ffi::c_double / scale_0;
    }
    if scale_0 < 1.0e-10f64 {
        scale_0 = 1.0e-10f64;
    }
    if scale_0 > 1.0e+10f64 {
        scale_0 = 1.0e+10f64;
    }
    return scale_0;
}
#[export_name="honest_lpsolve_roundPower2"]
pub unsafe extern "C" fn roundPower2(mut scale_0: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut power2: ::core::ffi::c_long = 0;
    let mut isSmall: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if scale_0 == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        return scale_0;
    }
    if scale_0 < 2 as ::core::ffi::c_int as ::core::ffi::c_double {
        scale_0 = 2 as ::core::ffi::c_int as ::core::ffi::c_double / scale_0;
        isSmall = TRUE as ::core::ffi::c_uchar;
    } else {
        scale_0 /= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    scale_0 = log(scale_0) / log(2.0f64);
    power2 = ceil(scale_0 - 0.5f64) as ::core::ffi::c_long;
    scale_0 = ((1 as ::core::ffi::c_int) << power2) as ::core::ffi::c_double;
    if isSmall != 0 {
        scale_0 = 1.0f64 / scale_0;
    }
    return scale_0;
}
#[export_name="honest_lpsolve_scale_updatecolumns"]
pub unsafe extern "C" fn scale_updatecolumns(
    mut lp: *mut lprec,
    mut scalechange: *mut ::core::ffi::c_double,
    mut updateonly: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = (*lp).columns;
    while i > 0 as ::core::ffi::c_int {
        if fabs(*scalechange.offset(i as isize) - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            > (*lp).epsprimal
        {
            break;
        }
        i -= 1;
    }
    if i <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uchar;
    }
    if updateonly != 0 {
        i = 1 as ::core::ffi::c_int;
        j = (*lp).rows + 1 as ::core::ffi::c_int;
        while j <= (*lp).sum {
            *(*lp).scalars.offset(j as isize) *= *scalechange.offset(i as isize);
            i += 1;
            j += 1;
        }
    } else {
        i = 1 as ::core::ffi::c_int;
        j = (*lp).rows + 1 as ::core::ffi::c_int;
        while j <= (*lp).sum {
            *(*lp).scalars.offset(j as isize) = *scalechange.offset(i as isize);
            i += 1;
            j += 1;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_scale_updaterows"]
pub unsafe extern "C" fn scale_updaterows(
    mut lp: *mut lprec,
    mut scalechange: *mut ::core::ffi::c_double,
    mut updateonly: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    i = (*lp).rows;
    while i >= 0 as ::core::ffi::c_int {
        if fabs(*scalechange.offset(i as isize) - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            > (*lp).epsprimal
        {
            break;
        }
        i -= 1;
    }
    if i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uchar;
    }
    if updateonly != 0 {
        i = 0 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            *(*lp).scalars.offset(i as isize) *= *scalechange.offset(i as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            *(*lp).scalars.offset(i as isize) = *scalechange.offset(i as isize);
            i += 1;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_scale_columns"]
pub unsafe extern "C" fn scale_columns(
    mut lp: *mut lprec,
    mut scaledelta: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut scalechange: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut mat: *mut MATrec = (*lp).matA;
    if (*lp).scalemode & SCALE_ROWSONLY != 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_uchar;
    }
    if scaledelta.is_null() {
        scalechange = (*lp).scalars.offset((*lp).rows as isize) as *mut ::core::ffi::c_double;
    } else {
        scalechange = scaledelta.offset((*lp).rows as isize) as *mut ::core::ffi::c_double;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        *(*lp).orig_obj.offset(i as isize) *= *scalechange.offset(i as isize);
        i += 1;
    }
    mat_validate((*lp).matA);
    nz = get_nonzeros(lp);
    value = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    colnr = (*mat)
        .col_mat_colnr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nz {
        *value *= *scalechange.offset(*colnr as isize);
        i += 1;
        value = value.offset(matValueStep as isize);
        colnr = colnr.offset(matRowColStep as isize);
    }
    i = 1 as ::core::ffi::c_int;
    j = (*lp).rows + 1 as ::core::ffi::c_int;
    while j <= (*lp).sum {
        if *(*lp).orig_lowbo.offset(j as isize) > -(*lp).infinite {
            *(*lp).orig_lowbo.offset(j as isize) /= *scalechange.offset(i as isize);
        }
        if *(*lp).orig_upbo.offset(j as isize) < (*lp).infinite {
            *(*lp).orig_upbo.offset(j as isize) /= *scalechange.offset(i as isize);
        }
        if *(*lp).sc_lobound.offset(i as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *(*lp).sc_lobound.offset(i as isize) /= *scalechange.offset(i as isize);
        }
        i += 1;
        j += 1;
    }
    (*lp).columns_scaled = TRUE as ::core::ffi::c_uchar;
    set_action(
        &raw mut (*lp).spx_action,
        ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_scale_rows"]
pub unsafe extern "C" fn scale_rows(
    mut lp: *mut lprec,
    mut scaledelta: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut colMax: ::core::ffi::c_int = 0;
    let mut scalechange: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut mat: *mut MATrec = (*lp).matA;
    if (*lp).scalemode & SCALE_COLSONLY != 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_uchar;
    }
    if scaledelta.is_null() {
        scalechange = (*lp).scalars;
    } else {
        scalechange = scaledelta;
    }
    colMax = (*lp).columns;
    i = 1 as ::core::ffi::c_int;
    while i <= colMax {
        *(*lp).orig_obj.offset(i as isize) *= *scalechange.offset(0 as ::core::ffi::c_int as isize);
        i += 1;
    }
    nz = get_nonzeros(lp);
    value = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    rownr = (*mat)
        .col_mat_rownr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nz {
        *value *= *scalechange.offset(*rownr as isize);
        i += 1;
        value = value.offset(matValueStep as isize);
        rownr = rownr.offset(matRowColStep as isize);
    }
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if fabs(*(*lp).orig_rhs.offset(i as isize)) < (*lp).infinite {
            *(*lp).orig_rhs.offset(i as isize) *= *scalechange.offset(i as isize);
        }
        j = *(*(*lp).presolve_undo).var_to_orig.offset(i as isize);
        if j != 0 as ::core::ffi::c_int {
            *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize) *= *scalechange.offset(i as isize);
        }
        if *(*lp).orig_upbo.offset(i as isize) < (*lp).infinite {
            *(*lp).orig_upbo.offset(i as isize) *= *scalechange.offset(i as isize);
        }
        if *(*lp).orig_lowbo.offset(i as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && fabs(*(*lp).orig_lowbo.offset(i as isize)) < (*lp).infinite
        {
            *(*lp).orig_lowbo.offset(i as isize) *= *scalechange.offset(i as isize);
        }
        i += 1;
    }
    set_action(
        &raw mut (*lp).spx_action,
        ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_scale"]
pub unsafe extern "C" fn scale(
    mut lp: *mut lprec,
    mut scaledelta: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut row_count: ::core::ffi::c_int = 0;
    let mut nzOF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut row_max: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut row_min: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut scalechange: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut absval: ::core::ffi::c_double = 0.;
    let mut col_max: ::core::ffi::c_double = 0.;
    let mut col_min: ::core::ffi::c_double = 0.;
    let mut rowscaled: ::core::ffi::c_uchar = 0;
    let mut colscaled: ::core::ffi::c_uchar = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if is_scaletype(lp, SCALE_NONE) != 0 {
        return 0.0f64;
    }
    if (*lp).scaling_used == 0 {
        allocREAL(
            lp,
            &raw mut (*lp).scalars,
            (*lp).sum_alloc + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        i = 0 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            *(*lp).scalars.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        (*lp).scaling_used = TRUE as ::core::ffi::c_uchar;
    }
    if scaledelta.is_null() {
        allocREAL(
            lp,
            &raw mut scalechange,
            (*lp).sum + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
    } else {
        scalechange = scaledelta;
    }
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        *scalechange.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    row_count = (*lp).rows;
    allocREAL(
        lp,
        &raw mut row_max,
        row_count + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut row_min,
        row_count + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    i = 0 as ::core::ffi::c_int;
    while i <= row_count {
        if is_scaletype(lp, SCALE_MEAN) != 0 {
            *row_min.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *row_min.offset(i as isize) = (*lp).infinite;
        }
        i += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        absval = *(*lp).orig_obj.offset(j as isize);
        if absval != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            absval = scaled_mat(lp, absval, 0 as ::core::ffi::c_int, j);
            accumulate_for_scale(
                lp,
                row_min.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                row_max.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                absval,
            );
            nzOF += 1;
        }
        i = *(*mat)
            .col_end
            .offset((j - 1 as ::core::ffi::c_int) as isize);
        value = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
        rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
        nz = *(*mat).col_end.offset(j as isize);
        while i < nz {
            absval = scaled_mat(lp, *value, *rownr, j);
            accumulate_for_scale(
                lp,
                row_min.offset(*rownr as isize) as *mut ::core::ffi::c_double,
                row_max.offset(*rownr as isize) as *mut ::core::ffi::c_double,
                absval,
            );
            i += 1;
            value = value.offset(matValueStep as isize);
            rownr = rownr.offset(matRowColStep as isize);
        }
        j += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if i == 0 as ::core::ffi::c_int {
            nz = nzOF;
        } else {
            nz = mat_rowlength((*lp).matA, i);
        }
        absval = minmax_to_scale(
            lp,
            *row_min.offset(i as isize),
            *row_max.offset(i as isize),
            nz,
        );
        if absval == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            absval = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        *scalechange.offset(i as isize) = absval;
        i += 1;
    }
    if !(row_max as *mut ::core::ffi::c_void).is_null() {
        free(row_max as *mut ::core::ffi::c_void);
        row_max = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(row_min as *mut ::core::ffi::c_void).is_null() {
        free(row_min as *mut ::core::ffi::c_void);
        row_min = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    rowscaled = scale_updaterows(lp, scalechange, TRUE as ::core::ffi::c_uchar);
    i = 1 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        if is_int(lp, j) as ::core::ffi::c_int != 0 && is_integerscaling(lp) == 0 {
            *scalechange.offset(((*lp).rows + j) as isize) =
                1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            col_max = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            if is_scaletype(lp, SCALE_MEAN) != 0 {
                col_min = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                col_min = (*lp).infinite;
            }
            absval = *(*lp).orig_obj.offset(j as isize);
            if absval != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                absval = scaled_mat(lp, absval, 0 as ::core::ffi::c_int, j);
                accumulate_for_scale(lp, &raw mut col_min, &raw mut col_max, absval);
            }
            i = *(*mat)
                .col_end
                .offset((j - 1 as ::core::ffi::c_int) as isize);
            value = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
            rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
            nz = *(*mat).col_end.offset(j as isize);
            while i < nz {
                absval = scaled_mat(lp, *value, *rownr, j);
                accumulate_for_scale(lp, &raw mut col_min, &raw mut col_max, absval);
                i += 1;
                value = value.offset(matValueStep as isize);
                rownr = rownr.offset(matRowColStep as isize);
            }
            nz = mat_collength((*lp).matA, j);
            if fabs(*(*lp).orig_obj.offset(j as isize))
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                nz += 1;
            }
            *scalechange.offset(((*lp).rows + j) as isize) =
                minmax_to_scale(lp, col_min, col_max, nz);
        }
        j += 1;
    }
    colscaled = scale_updatecolumns(
        lp,
        scalechange.offset((*lp).rows as isize) as *mut ::core::ffi::c_double,
        TRUE as ::core::ffi::c_uchar,
    );
    if rowscaled as ::core::ffi::c_int != 0 || colscaled as ::core::ffi::c_int != 0 {
        col_max = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            col_max += log(*scalechange.offset(((*lp).rows + j) as isize));
            j += 1;
        }
        col_max = exp(col_max / (*lp).columns as ::core::ffi::c_double);
        i = 0 as ::core::ffi::c_int;
        col_min = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        while i <= (*lp).rows {
            col_min += log(*scalechange.offset(i as isize));
            i += 1;
        }
        col_min = exp(col_min / row_count as ::core::ffi::c_double);
    } else {
        col_max = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        col_min = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if scaledelta.is_null() {
        if !(scalechange as *mut ::core::ffi::c_void).is_null() {
            free(scalechange as *mut ::core::ffi::c_void);
            scalechange = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    }
    return 1 as ::core::ffi::c_int as ::core::ffi::c_double - sqrt(col_max * col_min);
}
#[export_name="honest_lpsolve_finalize_scaling"]
pub unsafe extern "C" fn finalize_scaling(
    mut lp: *mut lprec,
    mut scaledelta: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    if is_scalemode(lp, SCALE_EQUILIBRATE) as ::core::ffi::c_int != 0
        && is_scaletype(lp, SCALE_CURTISREID) == 0
    {
        let mut oldmode: ::core::ffi::c_int = 0;
        oldmode = (*lp).scalemode;
        (*lp).scalemode = SCALE_LINEAR + SCALE_EXTREME;
        scale(lp, scaledelta);
        (*lp).scalemode = oldmode;
    }
    if is_scalemode(lp, SCALE_POWER2) != 0 {
        let mut scalars: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        if scaledelta.is_null() {
            scalars = (*lp).scalars;
        } else {
            scalars = scaledelta;
        }
        i = 0 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            *scalars.offset(i as isize) = roundPower2(*scalars.offset(i as isize));
            i += 1;
        }
    }
    return (scale_rows(lp, scaledelta) as ::core::ffi::c_int != 0
        && scale_columns(lp, scaledelta) as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_auto_scale"]
pub unsafe extern "C" fn auto_scale(mut lp: *mut lprec) -> ::core::ffi::c_double {
    let mut n: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut scalingmetric: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut scalenew: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if (*lp).scaling_used as ::core::ffi::c_int != 0
        && ((*lp).scalemode & SCALE_DYNUPDATE == 0 as ::core::ffi::c_int
            || (*lp).bb_level > 0 as ::core::ffi::c_int)
    {
        return scalingmetric;
    }
    if (*lp).scalemode != SCALE_NONE {
        if (*lp).solvecount > 1 as ::core::ffi::c_int
            && (*lp).bb_level < 1 as ::core::ffi::c_int
            && (*lp).scalemode & SCALE_DYNUPDATE != 0 as ::core::ffi::c_int
        {
            allocREAL(
                lp,
                &raw mut scalenew,
                (*lp).sum + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
        }
        if is_scaletype(lp, SCALE_CURTISREID) != 0 {
            scalingmetric = scaleCR(lp, scalenew) as ::core::ffi::c_double;
        } else {
            let mut scalinglimit: ::core::ffi::c_double = 0.;
            let mut scalingdelta: ::core::ffi::c_double = 0.;
            let mut count: ::core::ffi::c_int = 0;
            count = floor((*lp).scalelimit) as ::core::ffi::c_int;
            scalinglimit = (*lp).scalelimit;
            if count == 0 as ::core::ffi::c_int
                || scalinglimit == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                if scalinglimit > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    count = DEF_SCALINGLIMIT;
                } else {
                    count = 1 as ::core::ffi::c_int;
                }
            } else {
                scalinglimit -= count as ::core::ffi::c_double;
            }
            n = 0 as ::core::ffi::c_int;
            scalingdelta = 1.0f64;
            scalingmetric = 1.0f64;
            while n < count && fabs(scalingdelta) > scalinglimit {
                n += 1;
                scalingdelta = scale(lp, scalenew);
                scalingmetric = scalingmetric
                    * (1 as ::core::ffi::c_int as ::core::ffi::c_double + scalingdelta);
            }
            scalingmetric -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    mat_computemax((*lp).matA);
    if (*lp).scaling_used as ::core::ffi::c_int != 0 && fabs(scalingmetric) >= (*lp).epsprimal {
        finalize_scaling(lp, scalenew);
    } else {
        if !(*lp).scalars.is_null() {
            if !((*lp).scalars as *mut ::core::ffi::c_void).is_null() {
                free((*lp).scalars as *mut ::core::ffi::c_void);
                (*lp).scalars = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
        }
        (*lp).scaling_used = FALSE as ::core::ffi::c_uchar;
        (*lp).columns_scaled = FALSE as ::core::ffi::c_uchar;
    }
    if !scalenew.is_null() {
        if !(scalenew as *mut ::core::ffi::c_void).is_null() {
            free(scalenew as *mut ::core::ffi::c_void);
            scalenew = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    }
    return scalingmetric;
}
#[export_name="honest_lpsolve_unscale_columns"]
pub unsafe extern "C" fn unscale_columns(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if (*lp).columns_scaled == 0 {
        return;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        *(*lp).orig_obj.offset(j as isize) = unscaled_mat(
            lp,
            *(*lp).orig_obj.offset(j as isize),
            0 as ::core::ffi::c_int,
            j,
        );
        j += 1;
    }
    mat_validate(mat);
    nz = get_nonzeros(lp);
    value = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    rownr = (*mat)
        .col_mat_rownr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    colnr = (*mat)
        .col_mat_colnr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < nz {
        *value = unscaled_mat(lp, *value, *rownr, *colnr);
        j += 1;
        value = value.offset(matValueStep as isize);
        rownr = rownr.offset(matRowColStep as isize);
        colnr = colnr.offset(matRowColStep as isize);
    }
    i = (*lp).rows + 1 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        *(*lp).orig_lowbo.offset(i as isize) =
            unscaled_value(lp, *(*lp).orig_lowbo.offset(i as isize), i);
        *(*lp).orig_upbo.offset(i as isize) =
            unscaled_value(lp, *(*lp).orig_upbo.offset(i as isize), i);
        *(*lp).sc_lobound.offset(j as isize) =
            unscaled_value(lp, *(*lp).sc_lobound.offset(j as isize), i);
        i += 1;
        j += 1;
    }
    i = (*lp).rows + 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        *(*lp).scalars.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    (*lp).columns_scaled = FALSE as ::core::ffi::c_uchar;
    set_action(
        &raw mut (*lp).spx_action,
        ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
    );
}
#[export_name="honest_lpsolve_undoscale"]
pub unsafe extern "C" fn undoscale(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if (*lp).scaling_used != 0 {
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            *(*lp).orig_obj.offset(j as isize) = unscaled_mat(
                lp,
                *(*lp).orig_obj.offset(j as isize),
                0 as ::core::ffi::c_int,
                j,
            );
            j += 1;
        }
        mat_validate(mat);
        nz = get_nonzeros(lp);
        value = (*mat)
            .col_mat_value
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
        rownr = (*mat)
            .col_mat_rownr
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
        colnr = (*mat)
            .col_mat_colnr
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while j < nz {
            *value = unscaled_mat(lp, *value, *rownr, *colnr);
            j += 1;
            value = value.offset(matValueStep as isize);
            rownr = rownr.offset(matRowColStep as isize);
            colnr = colnr.offset(matRowColStep as isize);
        }
        i = (*lp).rows + 1 as ::core::ffi::c_int;
        j = 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            *(*lp).orig_lowbo.offset(i as isize) =
                unscaled_value(lp, *(*lp).orig_lowbo.offset(i as isize), i);
            *(*lp).orig_upbo.offset(i as isize) =
                unscaled_value(lp, *(*lp).orig_upbo.offset(i as isize), i);
            *(*lp).sc_lobound.offset(j as isize) =
                unscaled_value(lp, *(*lp).sc_lobound.offset(j as isize), i);
            i += 1;
            j += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            *(*lp).orig_rhs.offset(i as isize) =
                unscaled_value(lp, *(*lp).orig_rhs.offset(i as isize), i);
            j = *(*(*lp).presolve_undo).var_to_orig.offset(i as isize);
            if j != 0 as ::core::ffi::c_int {
                *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize) =
                    unscaled_value(lp, *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize), i);
            }
            *(*lp).orig_lowbo.offset(i as isize) =
                unscaled_value(lp, *(*lp).orig_lowbo.offset(i as isize), i);
            *(*lp).orig_upbo.offset(i as isize) =
                unscaled_value(lp, *(*lp).orig_upbo.offset(i as isize), i);
            i += 1;
        }
        if !((*lp).scalars as *mut ::core::ffi::c_void).is_null() {
            free((*lp).scalars as *mut ::core::ffi::c_void);
            (*lp).scalars = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        (*lp).scaling_used = FALSE as ::core::ffi::c_uchar;
        (*lp).columns_scaled = FALSE as ::core::ffi::c_uchar;
        set_action(
            &raw mut (*lp).spx_action,
            ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
        );
    }
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SCALE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SCALE_EXTREME: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SCALE_RANGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SCALE_MEAN: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SCALE_GEOMETRIC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCALE_CURTISREID: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SCALE_LINEAR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SCALE_QUADRATIC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SCALE_LOGARITHMIC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SCALE_POWER2: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SCALE_EQUILIBRATE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SCALE_DYNUPDATE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const SCALE_ROWSONLY: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const SCALE_COLSONLY: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const ACTION_REBASE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ACTION_RECOMPUTE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ACTION_REINVERT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const DEF_SCALINGLIMIT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MIN_SCALAR: ::core::ffi::c_double = 1.0e-10f64;
pub const MAX_SCALAR: ::core::ffi::c_double = 1.0e+10f64;
pub const matRowColStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const matValueStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
