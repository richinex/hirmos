use crate::honest_did::lpsolve::runtime::{free,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_unscaled_mat"]
    fn unscaled_mat(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    #[link_name="honest_lpsolve_swapINT"]
    fn swapINT(item1: *mut ::core::ffi::c_int, item2: *mut ::core::ffi::c_int);
    #[link_name="honest_lpsolve_createLink"]
    fn createLink(
        size: ::core::ffi::c_int,
        linkmap: *mut *mut LLrec,
        usedpos: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_freeLink"]
    fn freeLink(linkmap: *mut *mut LLrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_isActiveLink"]
    fn isActiveLink(linkmap: *mut LLrec, itemnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_firstActiveLink"]
    fn firstActiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_appendLink"]
    fn appendLink(linkmap: *mut LLrec, newitem: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_nextActiveLink"]
    fn nextActiveLink(linkmap: *mut LLrec, backitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_removeLink"]
    fn removeLink(linkmap: *mut LLrec, itemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_validate"]
    fn mat_validate(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_nonzeros"]
    fn mat_nonzeros(mat: *mut MATrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_collength"]
    fn mat_collength(mat: *mut MATrec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_get_constr_type"]
    fn get_constr_type(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_get_nonzeros"]
    fn get_nonzeros(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_get_upbo"]
    fn get_upbo(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_lowbo"]
    fn get_lowbo(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_is_unbounded"]
    fn is_unbounded(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_default_basis"]
    fn default_basis(lp: *mut lprec);
    #[link_name="honest_lpsolve_set_basisvar"]
    fn set_basisvar(
        lp: *mut lprec,
        basisPos: ::core::ffi::c_int,
        enteringCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_chsign"]
    fn is_chsign(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_rh_upper"]
    fn get_rh_upper(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_rh_lower"]
    fn get_rh_lower(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_sortByREAL"]
    fn sortByREAL(
        item: *mut ::core::ffi::c_int,
        weight: *mut ::core::ffi::c_double,
        size: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        unique: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LLrec {
    pub size: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub firstitem: ::core::ffi::c_int,
    pub lastitem: ::core::ffi::c_int,
    pub map: *mut ::core::ffi::c_int,
}
pub type LLrec = _LLrec;
pub const CRASH_THRESHOLD: ::core::ffi::c_double = 0.167f64;
pub const CRASH_SPACER: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CRASH_WEIGHT: ::core::ffi::c_double = 0.500f64;
#[export_name="honest_lpsolve_crash_basis"]
pub unsafe extern "C" fn crash_basis(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ok: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if (*lp).basis_valid != 0 {
        *(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) = FALSE;
    } else {
        default_basis(lp);
    }
    if !(*lp).rowblocks.is_null() {
        (*(*lp).rowblocks).blocknow = 1 as ::core::ffi::c_int;
    }
    if !(*lp).colblocks.is_null() {
        (*(*lp).colblocks).blocknow = if (*lp).crashmode == CRASH_NONE
            || (*(*lp).colblocks).blockcount == 1 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
    }
    if (*lp).crashmode == CRASH_MOSTFEASIBLE && mat_validate(mat) as ::core::ffi::c_int != 0 {
        let mut rowLL: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
        let mut colLL: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
        let mut ii: ::core::ffi::c_int = 0;
        let mut rx: ::core::ffi::c_int = 0;
        let mut cx: ::core::ffi::c_int = 0;
        let mut ix: ::core::ffi::c_int = 0;
        let mut nz: ::core::ffi::c_int = 0;
        let mut wx: ::core::ffi::c_double = 0.;
        let mut tx: ::core::ffi::c_double = 0.;
        let mut rowMAX: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut colMAX: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut rowNZ: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut colNZ: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut rowWT: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut colWT: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut value: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"crash_basis: 'Most feasible' basis crashing selected\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        ok = (allocINT(
            lp,
            &raw mut rowNZ,
            (*lp).rows + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0
            && allocINT(
                lp,
                &raw mut colNZ,
                (*lp).columns + 1 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0
            && allocREAL(
                lp,
                &raw mut rowMAX,
                (*lp).rows + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0
            && allocREAL(
                lp,
                &raw mut colMAX,
                (*lp).columns + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if !(ok == 0) {
            nz = mat_nonzeros(mat);
            rownr = (*mat)
                .col_mat_rownr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            colnr = (*mat)
                .col_mat_colnr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            value = (*mat)
                .col_mat_value
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double;
            i = 0 as ::core::ffi::c_int;
            while i < nz {
                rx = *rownr;
                cx = *colnr;
                wx = fabs(*value);
                let ref mut fresh0 = *rowNZ.offset(rx as isize);
                *fresh0 += 1;
                let ref mut fresh1 = *colNZ.offset(cx as isize);
                *fresh1 += 1;
                if i == 0 as ::core::ffi::c_int {
                    *rowMAX.offset(rx as isize) = wx;
                    *colMAX.offset(cx as isize) = wx;
                    *colMAX.offset(0 as ::core::ffi::c_int as isize) = wx;
                } else {
                    if *rowMAX.offset(rx as isize) < wx {
                        *rowMAX.offset(rx as isize) = wx;
                    }
                    if *colMAX.offset(cx as isize) < wx {
                        *colMAX.offset(cx as isize) = wx;
                    }
                    if *colMAX.offset(0 as ::core::ffi::c_int as isize) < wx {
                        *colMAX.offset(0 as ::core::ffi::c_int as isize) = wx;
                    }
                }
                i += 1;
                rownr = rownr.offset(matRowColStep as isize);
                colnr = colnr.offset(matRowColStep as isize);
                value = value.offset(matValueStep as isize);
            }
            rownr = (*mat)
                .col_mat_rownr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            colnr = (*mat)
                .col_mat_colnr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            value = (*mat)
                .col_mat_value
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double;
            i = 0 as ::core::ffi::c_int;
            while i < nz {
                rx = *rownr;
                cx = *colnr;
                wx = fabs(*value);
                if wx < CRASH_THRESHOLD * *colMAX.offset(0 as ::core::ffi::c_int as isize) {
                    let ref mut fresh2 = *rowNZ.offset(rx as isize);
                    *fresh2 -= 1;
                    let ref mut fresh3 = *colNZ.offset(cx as isize);
                    *fresh3 -= 1;
                }
                i += 1;
                rownr = rownr.offset(matRowColStep as isize);
                colnr = colnr.offset(matRowColStep as isize);
                value = value.offset(matValueStep as isize);
            }
            ok = allocINT(
                lp,
                &raw mut rowWT,
                (*lp).rows + 1 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            );
            createLink(
                (*lp).rows,
                &raw mut rowLL,
                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            );
            ok = (ok as ::core::ffi::c_int & (rowLL != NULL as *mut LLrec) as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            if !(ok == 0) {
                i = 1 as ::core::ffi::c_int;
                while i <= (*lp).rows {
                    if get_constr_type(lp, i) == EQ {
                        ii = 3 as ::core::ffi::c_int;
                    } else if *(*lp).upbo.offset(i as isize) < (*lp).infinite {
                        ii = 2 as ::core::ffi::c_int;
                    } else if fabs(*(*lp).rhs.offset(i as isize)) < (*lp).infinite {
                        ii = 1 as ::core::ffi::c_int;
                    } else {
                        ii = 0 as ::core::ffi::c_int;
                    }
                    *rowWT.offset(i as isize) = ii;
                    if ii > 0 as ::core::ffi::c_int {
                        appendLink(rowLL, i);
                    }
                    i += 1;
                }
                ok = allocINT(
                    lp,
                    &raw mut colWT,
                    (*lp).columns + 1 as ::core::ffi::c_int,
                    TRUE as ::core::ffi::c_uchar,
                );
                createLink(
                    (*lp).columns,
                    &raw mut colLL,
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                );
                ok = (ok as ::core::ffi::c_int
                    & (colLL != NULL as *mut LLrec) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
                if !(ok == 0) {
                    i = 1 as ::core::ffi::c_int;
                    while i <= (*lp).columns {
                        ix = (*lp).rows + i;
                        if is_unbounded(lp, i) != 0 {
                            ii = 3 as ::core::ffi::c_int;
                        } else if *(*lp).upbo.offset(ix as isize) >= (*lp).infinite {
                            ii = 2 as ::core::ffi::c_int;
                        } else if fabs(
                            *(*lp).upbo.offset(ix as isize) - *(*lp).lowbo.offset(ix as isize),
                        ) > (*lp).epsmachine
                        {
                            ii = 1 as ::core::ffi::c_int;
                        } else {
                            ii = 0 as ::core::ffi::c_int;
                        }
                        *colWT.offset(i as isize) = ii;
                        if ii > 0 as ::core::ffi::c_int {
                            appendLink(colLL, i);
                        }
                        i += 1;
                    }
                    i = 1 as ::core::ffi::c_int;
                    while i <= (*lp).rows {
                        rx = 0 as ::core::ffi::c_int;
                        wx = -(*lp).infinite;
                        ii = firstActiveLink(rowLL);
                        while ii > 0 as ::core::ffi::c_int {
                            tx = (*rowWT.offset(ii as isize)
                                - CRASH_SPACER * *rowNZ.offset(ii as isize))
                                as ::core::ffi::c_double;
                            if tx > wx {
                                rx = ii;
                                wx = tx;
                            }
                            ii = nextActiveLink(rowLL, ii);
                        }
                        if rx == 0 as ::core::ffi::c_int {
                            break;
                        }
                        removeLink(rowLL, rx);
                        cx = 0 as ::core::ffi::c_int;
                        wx = -(*lp).infinite;
                        ii = *(*mat)
                            .row_end
                            .offset((rx - 1 as ::core::ffi::c_int) as isize);
                        while ii < *(*mat).row_end.offset(rx as isize) {
                            tx = fabs(
                                *(*mat)
                                    .col_mat_value
                                    .offset(*(*mat).row_mat.offset(ii as isize) as isize),
                            );
                            ix = *(*mat)
                                .col_mat_colnr
                                .offset(*(*mat).row_mat.offset(ii as isize) as isize);
                            if tx
                                >= CRASH_THRESHOLD
                                    * *colMAX.offset(0 as ::core::ffi::c_int as isize)
                            {
                                let ref mut fresh4 = *colNZ.offset(ix as isize);
                                *fresh4 -= 1;
                            }
                            if !(isActiveLink(colLL, ix) == 0
                                || tx < CRASH_THRESHOLD * *rowMAX.offset(rx as isize))
                            {
                                tx = ((if *(*lp).orig_obj.offset(ix as isize)
                                    < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    -(1 as ::core::ffi::c_int)
                                } else {
                                    1 as ::core::ffi::c_int
                                }) - (if *(*mat)
                                    .col_mat_value
                                    .offset(*(*mat).row_mat.offset(ii as isize) as isize)
                                    < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    -(1 as ::core::ffi::c_int)
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as ::core::ffi::c_double;
                                tx = *colWT.offset(ix as isize) as ::core::ffi::c_double
                                    + CRASH_WEIGHT * tx
                                    - (CRASH_SPACER * *colNZ.offset(ix as isize))
                                        as ::core::ffi::c_double;
                                if tx > wx {
                                    cx = ix;
                                    wx = tx;
                                }
                            }
                            ii += 1;
                        }
                        if cx == 0 as ::core::ffi::c_int {
                            break;
                        }
                        removeLink(colLL, cx);
                        ii = *(*mat)
                            .col_end
                            .offset((cx - 1 as ::core::ffi::c_int) as isize);
                        rownr = (*mat).col_mat_rownr.offset(ii as isize) as *mut ::core::ffi::c_int;
                        value =
                            (*mat).col_mat_value.offset(ii as isize) as *mut ::core::ffi::c_double;
                        while ii < *(*mat).col_end.offset(cx as isize) {
                            wx = fabs(*value);
                            ix = *rownr;
                            if wx
                                >= CRASH_THRESHOLD
                                    * *colMAX.offset(0 as ::core::ffi::c_int as isize)
                            {
                                let ref mut fresh5 = *rowNZ.offset(ix as isize);
                                *fresh5 -= 1;
                            }
                            ii += 1;
                            rownr = rownr.offset(matRowColStep as isize);
                            value = value.offset(matValueStep as isize);
                        }
                        set_basisvar(lp, rx, (*lp).rows + cx);
                        i += 1;
                    }
                }
            }
        }
        if !(rowNZ as *mut ::core::ffi::c_void).is_null() {
            free(rowNZ as *mut ::core::ffi::c_void);
            rowNZ = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !(colNZ as *mut ::core::ffi::c_void).is_null() {
            free(colNZ as *mut ::core::ffi::c_void);
            colNZ = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !(rowMAX as *mut ::core::ffi::c_void).is_null() {
            free(rowMAX as *mut ::core::ffi::c_void);
            rowMAX = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !(colMAX as *mut ::core::ffi::c_void).is_null() {
            free(colMAX as *mut ::core::ffi::c_void);
            colMAX = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !(rowWT as *mut ::core::ffi::c_void).is_null() {
            free(rowWT as *mut ::core::ffi::c_void);
            rowWT = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !(colWT as *mut ::core::ffi::c_void).is_null() {
            free(colWT as *mut ::core::ffi::c_void);
            colWT = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        freeLink(&raw mut rowLL);
        freeLink(&raw mut colLL);
    } else if (*lp).crashmode == CRASH_LEASTDEGENERATE
        && mat_validate(mat) as ::core::ffi::c_int != 0
    {
        let mut rowLL_0: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
        let mut colLL_0: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
        let mut ii_0: ::core::ffi::c_int = 0;
        let mut rx_0: ::core::ffi::c_int = 0;
        let mut cx_0: ::core::ffi::c_int = 0;
        let mut ix_0: ::core::ffi::c_int = 0;
        let mut nz_0: ::core::ffi::c_int = 0;
        let mut merit: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut value_0: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut wx_0: ::core::ffi::c_double = 0.;
        let mut hold: ::core::ffi::c_double = 0.;
        let mut rhs: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut eta: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut rownr_0: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut colnr_0: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"crash_basis: 'Least degenerate' basis crashing selected\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        ok = (allocINT(
            lp,
            &raw mut merit,
            (*lp).columns + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0
            && allocREAL(
                lp,
                &raw mut eta,
                (*lp).rows + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0
            && allocREAL(
                lp,
                &raw mut rhs,
                (*lp).rows + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        createLink(
            (*lp).columns,
            &raw mut colLL_0,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        createLink(
            (*lp).rows,
            &raw mut rowLL_0,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        ok = (ok as ::core::ffi::c_int
            & (!colLL_0.is_null() && !rowLL_0.is_null()) as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        if !(ok == 0) {
            memcpy(
                rhs as *mut ::core::ffi::c_void,
                (*lp).orig_rhs as *const ::core::ffi::c_void,
                (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                appendLink(colLL_0, i);
                i += 1;
            }
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).rows {
                appendLink(rowLL_0, i);
                i += 1;
            }
            while (*colLL_0).count > 0 as ::core::ffi::c_int {
                nz_0 = mat_nonzeros(mat);
                rownr_0 = (*mat)
                    .col_mat_rownr
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_int;
                colnr_0 = (*mat)
                    .col_mat_colnr
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_int;
                ii_0 = 0 as ::core::ffi::c_int;
                memset(
                    merit as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (((*lp).columns + 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                );
                i = 0 as ::core::ffi::c_int;
                while i < nz_0 {
                    rx_0 = *rownr_0;
                    cx_0 = *colnr_0;
                    if isActiveLink(colLL_0, cx_0) as ::core::ffi::c_int != 0
                        && *rhs.offset(rx_0 as isize)
                            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        let ref mut fresh6 = *merit.offset(cx_0 as isize);
                        *fresh6 += 1;
                        ii_0 += 1;
                    }
                    i += 1;
                    rownr_0 = rownr_0.offset(matRowColStep as isize);
                    colnr_0 = colnr_0.offset(matRowColStep as isize);
                }
                if ii_0 == 0 as ::core::ffi::c_int {
                    break;
                }
                i = firstActiveLink(colLL_0);
                cx_0 = i;
                i = nextActiveLink(colLL_0, i);
                while i != 0 as ::core::ffi::c_int {
                    if *merit.offset(i as isize) >= *merit.offset(cx_0 as isize) {
                        if *merit.offset(i as isize) > *merit.offset(cx_0 as isize)
                            || mat_collength(mat, i) > mat_collength(mat, cx_0)
                        {
                            cx_0 = i;
                        }
                    }
                    i = nextActiveLink(colLL_0, i);
                }
                i = *(*mat)
                    .col_end
                    .offset((cx_0 - 1 as ::core::ffi::c_int) as isize);
                nz_0 = *(*mat).col_end.offset(cx_0 as isize);
                rownr_0 = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
                value_0 = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
                rx_0 = 0 as ::core::ffi::c_int;
                wx_0 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                memset(
                    eta as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
                while i < nz_0 {
                    ix_0 = *rownr_0;
                    hold = *value_0;
                    *eta.offset(ix_0 as isize) = *rhs.offset(ix_0 as isize) / hold;
                    hold = fabs(hold);
                    if isActiveLink(rowLL_0, ix_0) as ::core::ffi::c_int != 0 && hold > wx_0 {
                        wx_0 = hold;
                        rx_0 = ix_0;
                    }
                    i += 1;
                    rownr_0 = rownr_0.offset(matRowColStep as isize);
                    value_0 = value_0.offset(matValueStep as isize);
                }
                if rx_0 > 0 as ::core::ffi::c_int {
                    i = 1 as ::core::ffi::c_int;
                    while i <= (*lp).rows {
                        *rhs.offset(i as isize) -= wx_0 * *eta.offset(i as isize);
                        i += 1;
                    }
                    *rhs.offset(rx_0 as isize) = wx_0;
                    set_basisvar(lp, rx_0, (*lp).rows + cx_0);
                    removeLink(rowLL_0, rx_0);
                }
                removeLink(colLL_0, cx_0);
            }
        }
        if !(merit as *mut ::core::ffi::c_void).is_null() {
            free(merit as *mut ::core::ffi::c_void);
            merit = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !(rhs as *mut ::core::ffi::c_void).is_null() {
            free(rhs as *mut ::core::ffi::c_void);
            rhs = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        freeLink(&raw mut rowLL_0);
        freeLink(&raw mut colLL_0);
    }
    return ok;
}
#[export_name="honest_lpsolve_guess_basis"]
pub unsafe extern "C" fn guess_basis(
    mut lp: *mut lprec,
    mut guessvector: *mut ::core::ffi::c_double,
    mut basisvector: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut isnz: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut values: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut violation: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut eps: ::core::ffi::c_double = (*lp).epsprimal;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut error: ::core::ffi::c_double = 0.;
    let mut upB: ::core::ffi::c_double = 0.;
    let mut loB: ::core::ffi::c_double = 0.;
    let mut sortorder: ::core::ffi::c_double = -1.0f64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut slkpos: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut ncols: ::core::ffi::c_int = (*lp).columns;
    let mut nsum: ::core::ffi::c_int = (*lp).sum;
    let mut basisnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut mat: *mut MATrec = (*lp).matA;
    if mat_validate(mat) == 0 {
        return status;
    }
    if !(allocREAL(
        lp,
        &raw mut values,
        nsum + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
        || allocREAL(
            lp,
            &raw mut violation,
            nsum + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0)
    {
        i = 0 as ::core::ffi::c_int;
        n = get_nonzeros(lp);
        rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
        colnr = (*mat).col_mat_colnr.offset(i as isize) as *mut ::core::ffi::c_int;
        value = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
        while i < n {
            *values.offset(*rownr as isize) += unscaled_mat(
                lp,
                (if is_chsign(lp, *rownr) as ::core::ffi::c_int != 0
                    && *value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -*value
                } else {
                    *value
                }),
                *rownr,
                *colnr,
            ) * *guessvector.offset(*colnr as isize);
            i += 1;
            rownr = rownr.offset(matRowColStep as isize);
            colnr = colnr.offset(matRowColStep as isize);
            value = value.offset(matValueStep as isize);
        }
        memmove(
            values
                .offset(nrows as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            guessvector.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (ncols as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        i = 1 as ::core::ffi::c_int;
        while i <= nsum {
            if i <= nrows {
                loB = get_rh_lower(lp, i);
                upB = get_rh_upper(lp, i);
            } else {
                loB = get_lowbo(lp, i - nrows);
                upB = get_upbo(lp, i - nrows);
            }
            if (fabs(loB) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
                as ::core::ffi::c_int
                != 0
                && (fabs(upB) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
                    as ::core::ffi::c_int
                    != 0
            {
                error = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else if *values.offset(i as isize) + eps < loB {
                error = loB - *values.offset(i as isize);
            } else if *values.offset(i as isize) - eps > upB {
                error = *values.offset(i as isize) - upB;
            } else if (fabs(upB) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
                != 0
            {
                error = if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > *values.offset(i as isize) - loB
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    *values.offset(i as isize) - loB
                };
            } else if (fabs(loB) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
                != 0
            {
                error = if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > upB - *values.offset(i as isize)
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    upB - *values.offset(i as isize)
                };
            } else {
                error = if upB - *values.offset(i as isize) < *values.offset(i as isize) - loB {
                    upB - *values.offset(i as isize)
                } else {
                    *values.offset(i as isize) - loB
                };
            }
            if error != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *violation.offset(i as isize) = sortorder * error;
            }
            *basisvector.offset(i as isize) = i;
            i += 1;
        }
        sortByREAL(
            basisvector,
            violation,
            nsum,
            1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        error = *violation.offset(1 as ::core::ffi::c_int as isize);
        slkpos = violation as *mut ::core::ffi::c_int;
        n = nrows + 1 as ::core::ffi::c_int;
        memset(
            slkpos as *mut ::core::ffi::c_void,
            '\0' as i32,
            (n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        isnz = slkpos
            .offset(n as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar;
        memset(
            isnz as *mut ::core::ffi::c_void,
            '\0' as i32,
            (n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t),
        );
        i = 1 as ::core::ffi::c_int;
        while i <= nrows {
            j = abs(*basisvector.offset(i as isize));
            if j <= nrows {
                *isnz.offset(j as isize) = TRUE as ::core::ffi::c_uchar;
                *slkpos.offset(j as isize) = i;
            } else {
                j -= nrows;
                jj = *(*mat)
                    .col_end
                    .offset((j - 1 as ::core::ffi::c_int) as isize);
                jj = *(*mat).col_mat_rownr.offset(jj as isize);
                *isnz.offset(jj as isize) = TRUE as ::core::ffi::c_uchar;
            }
            i += 1;
        }
        while i <= nsum {
            j = abs(*basisvector.offset(i as isize));
            if j <= nrows {
                *slkpos.offset(j as isize) = i;
            }
            i += 1;
        }
        j = 1 as ::core::ffi::c_int;
        while j <= nrows {
            if *slkpos.offset(j as isize) == 0 as ::core::ffi::c_int {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"guess_basis: Internal error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if *isnz.offset(j as isize) == 0 {
                *isnz.offset(j as isize) = TRUE as ::core::ffi::c_uchar;
                i = *slkpos.offset(j as isize);
                swapINT(
                    basisvector.offset(i as isize) as *mut ::core::ffi::c_int,
                    basisvector.offset(j as isize) as *mut ::core::ffi::c_int,
                );
                *basisvector.offset(j as isize) = abs(*basisvector.offset(j as isize));
            }
            j += 1;
        }
        i = nrows + 1 as ::core::ffi::c_int;
        basisnr = basisvector.offset(i as isize);
        while i <= nsum {
            n = *basisnr;
            if n <= nrows {
                *values.offset(n as isize) -= get_rh_lower(lp, n);
                if *values.offset(n as isize) <= eps {
                    *basisnr = -*basisnr;
                }
            } else if *values.offset(n as isize) - eps <= get_lowbo(lp, n - nrows) {
                *basisnr = -*basisnr;
            }
            i += 1;
            basisnr = basisnr.offset(1);
        }
        i = 1 as ::core::ffi::c_int;
        while i <= nrows {
            *basisvector.offset(i as isize) = -abs(*basisvector.offset(i as isize));
            i += 1;
        }
        status = (error <= eps) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    if !(values as *mut ::core::ffi::c_void).is_null() {
        free(values as *mut ::core::ffi::c_void);
        values = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(violation as *mut ::core::ffi::c_void).is_null() {
        free(violation as *mut ::core::ffi::c_void);
        violation = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return status;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CRASH_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CRASH_MOSTFEASIBLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CRASH_LEASTDEGENERATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ROWTYPE_EQ: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EQ: ::core::ffi::c_int = ROWTYPE_EQ;
pub const matRowColStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const matValueStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
