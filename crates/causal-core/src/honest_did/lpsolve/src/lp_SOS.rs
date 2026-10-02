use crate::honest_did::lpsolve::runtime::{strlen,strcpy};
use crate::honest_did::lpsolve::runtime::{malloc,calloc,free,realloc};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name="honest_lpsolve_allocCHAR"]
    fn allocCHAR(
        lp: *mut lprec,
        ptr: *mut *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        clear: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_allocMYBOOL"]
    fn allocMYBOOL(
        lp: *mut lprec,
        ptr: *mut *mut ::core::ffi::c_uchar,
        size: ::core::ffi::c_int,
        clear: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_searchFor"]
    fn searchFor(
        target: ::core::ffi::c_int,
        attributes: *mut ::core::ffi::c_int,
        size: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        absolute: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_isActiveLink"]
    fn isActiveLink(linkmap: *mut LLrec, itemnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_firstActiveLink"]
    fn firstActiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_nextActiveLink"]
    fn nextActiveLink(linkmap: *mut LLrec, backitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_modifyUndoLadder"]
    fn modifyUndoLadder(
        DV: *mut DeltaVrec,
        itemno: ::core::ffi::c_int,
        target: *mut ::core::ffi::c_double,
        newvalue: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_int"]
    fn set_int(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        must_be_int: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_int"]
    fn is_int(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_semicont"]
    fn is_semicont(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_count"]
    fn SOS_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_compareREAL"]
    fn compareREAL(
        current: *const ::core::ffi::c_void,
        candidate: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_hpsortex"]
    fn hpsortex(
        attributes: *mut ::core::ffi::c_void,
        count: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        recsize: ::core::ffi::c_int,
        descending: ::core::ffi::c_uchar,
        findCompare: Option<findCompare_func>,
        tags: *mut ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_sortByREAL"]
    fn sortByREAL(
        item: *mut ::core::ffi::c_int,
        weight: *mut ::core::ffi::c_double,
        size: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        unique: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_sortByINT"]
    fn sortByINT(
        item: *mut ::core::ffi::c_int,
        weight: *mut ::core::ffi::c_int,
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
pub type findCompare_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;
#[export_name="honest_lpsolve_create_SOSgroup"]
pub unsafe extern "C" fn create_SOSgroup(mut lp: *mut lprec) -> *mut SOSgroup {
    let mut group: *mut SOSgroup = ::core::ptr::null_mut::<SOSgroup>();
    group = calloc(1 as size_t, ::core::mem::size_of::<SOSgroup>() as size_t) as *mut SOSgroup;
    (*group).lp = lp;
    (*group).sos_alloc = SOS_START_SIZE;
    (*group).sos_list = malloc(
        ((*group).sos_alloc as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut SOSrec>() as size_t),
    ) as *mut *mut SOSrec;
    return group;
}
#[export_name="honest_lpsolve_resize_SOSgroup"]
pub unsafe extern "C" fn resize_SOSgroup(mut group: *mut SOSgroup) {
    if (*group).sos_count == (*group).sos_alloc {
        (*group).sos_alloc = ((*group).sos_alloc as ::core::ffi::c_double
            * RESIZEFACTOR as ::core::ffi::c_double)
            as ::core::ffi::c_int;
        (*group).sos_list = realloc(
            (*group).sos_list as *mut ::core::ffi::c_void,
            ((*group).sos_alloc as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut SOSrec>() as size_t),
        ) as *mut *mut SOSrec;
    }
}
#[export_name="honest_lpsolve_append_SOSgroup"]
pub unsafe extern "C" fn append_SOSgroup(
    mut group: *mut SOSgroup,
    mut SOS: *mut SOSrec,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut SOSHold: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    resize_SOSgroup(group);
    let ref mut fresh0 = *(*group).sos_list.offset((*group).sos_count as isize);
    *fresh0 = SOS;
    (*group).sos_count += 1;
    i = abs((*SOS).type_0);
    if (*group).maxorder < i {
        (*group).maxorder = i;
    }
    if i == 1 as ::core::ffi::c_int {
        (*group).sos1_count += 1;
    }
    k = (*group).sos_count;
    (*SOS).tagorder = k;
    i = (*group).sos_count - 1 as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        if !((**(*group).sos_list.offset(i as isize)).priority
            < (**(*group)
                .sos_list
                .offset((i - 1 as ::core::ffi::c_int) as isize))
            .priority)
        {
            break;
        }
        SOSHold = *(*group).sos_list.offset(i as isize);
        let ref mut fresh1 = *(*group).sos_list.offset(i as isize);
        *fresh1 = *(*group)
            .sos_list
            .offset((i - 1 as ::core::ffi::c_int) as isize);
        let ref mut fresh2 = *(*group)
            .sos_list
            .offset((i - 1 as ::core::ffi::c_int) as isize);
        *fresh2 = SOSHold;
        if SOSHold == SOS {
            k = i;
        }
        i -= 1;
    }
    return k;
}
#[export_name="honest_lpsolve_clean_SOSgroup"]
pub unsafe extern "C" fn clean_SOSgroup(
    mut group: *mut SOSgroup,
    mut forceupdatemap: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    if group.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    n = 0 as ::core::ffi::c_int;
    if (*group).sos_alloc > 0 as ::core::ffi::c_int {
        (*group).maxorder = 0 as ::core::ffi::c_int;
        i = (*group).sos_count;
        while i > 0 as ::core::ffi::c_int {
            SOS = *(*group)
                .sos_list
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            k = *(*SOS).members.offset(0 as ::core::ffi::c_int as isize);
            if k == 0 as ::core::ffi::c_int
                || k == abs((*SOS).type_0) && k <= 2 as ::core::ffi::c_int
            {
                delete_SOSrec(group, i);
                n += 1;
            } else if (*group).maxorder < abs((*SOS).type_0) {
                (*group).maxorder = abs((*SOS).type_0);
            }
            i -= 1;
        }
        if n > 0 as ::core::ffi::c_int || forceupdatemap as ::core::ffi::c_int != 0 {
            SOS_member_updatemap(group);
        }
    }
    return n;
}
#[export_name="honest_lpsolve_free_SOSgroup"]
pub unsafe extern "C" fn free_SOSgroup(mut group: *mut *mut SOSgroup) {
    let mut i: ::core::ffi::c_int = 0;
    if group.is_null() || (*group).is_null() {
        return;
    }
    if (**group).sos_alloc > 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < (**group).sos_count {
            free_SOSrec(*(**group).sos_list.offset(i as isize));
            i += 1;
        }
        if !((**group).sos_list as *mut ::core::ffi::c_void).is_null() {
            free((**group).sos_list as *mut ::core::ffi::c_void);
            (**group).sos_list = ::core::ptr::null_mut::<*mut SOSrec>();
        }
        if !((**group).membership as *mut ::core::ffi::c_void).is_null() {
            free((**group).membership as *mut ::core::ffi::c_void);
            (**group).membership = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !((**group).memberpos as *mut ::core::ffi::c_void).is_null() {
            free((**group).memberpos as *mut ::core::ffi::c_void);
            (**group).memberpos = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    if !(*group as *mut ::core::ffi::c_void).is_null() {
        free(*group as *mut ::core::ffi::c_void);
        *group = ::core::ptr::null_mut::<SOSgroup>();
    }
}
#[export_name="honest_lpsolve_create_SOSrec"]
pub unsafe extern "C" fn create_SOSrec(
    mut group: *mut SOSgroup,
    mut name: *mut ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_int,
    mut priority: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut variables: *mut ::core::ffi::c_int,
    mut weights: *mut ::core::ffi::c_double,
) -> *mut SOSrec {
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    SOS = calloc(1 as size_t, ::core::mem::size_of::<SOSrec>() as size_t) as *mut SOSrec;
    (*SOS).parent = group;
    (*SOS).type_0 = type_0;
    if name.is_null() {
        (*SOS).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        allocCHAR(
            (*group).lp,
            &raw mut (*SOS).name,
            strlen(name).wrapping_add(1 as size_t) as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        strcpy((*SOS).name, name);
    }
    if type_0 < 0 as ::core::ffi::c_int {
        type_0 = abs(type_0);
    }
    (*SOS).tagorder = 0 as ::core::ffi::c_int;
    (*SOS).size = 0 as ::core::ffi::c_int;
    (*SOS).priority = priority;
    (*SOS).members = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*SOS).weights = ::core::ptr::null_mut::<::core::ffi::c_double>();
    (*SOS).membersSorted = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*SOS).membersMapped = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if size > 0 as ::core::ffi::c_int {
        size = append_SOSrec(SOS, size, variables, weights);
    }
    return SOS;
}
#[export_name="honest_lpsolve_append_SOSrec"]
pub unsafe extern "C" fn append_SOSrec(
    mut SOS: *mut SOSrec,
    mut size: ::core::ffi::c_int,
    mut variables: *mut ::core::ffi::c_int,
    mut weights: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut oldsize: ::core::ffi::c_int = 0;
    let mut newsize: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut lp: *mut lprec = (*(*SOS).parent).lp;
    oldsize = (*SOS).size;
    newsize = oldsize + size;
    nn = abs((*SOS).type_0);
    if (*SOS).members.is_null() {
        allocINT(
            lp,
            &raw mut (*SOS).members,
            1 as ::core::ffi::c_int + newsize + 1 as ::core::ffi::c_int + nn,
            TRUE as ::core::ffi::c_uchar,
        );
    } else {
        allocINT(
            lp,
            &raw mut (*SOS).members,
            1 as ::core::ffi::c_int + newsize + 1 as ::core::ffi::c_int + nn,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        i = newsize + 1 as ::core::ffi::c_int + nn;
        while i > newsize + 1 as ::core::ffi::c_int {
            *(*SOS).members.offset(i as isize) = *(*SOS).members.offset((i - size) as isize);
            i -= 1;
        }
    }
    *(*SOS).members.offset(0 as ::core::ffi::c_int as isize) = newsize;
    *(*SOS)
        .members
        .offset((newsize + 1 as ::core::ffi::c_int) as isize) = nn;
    if (*SOS).weights.is_null() {
        allocREAL(
            lp,
            &raw mut (*SOS).weights,
            1 as ::core::ffi::c_int + newsize,
            TRUE as ::core::ffi::c_uchar,
        );
    } else {
        allocREAL(
            lp,
            &raw mut (*SOS).weights,
            1 as ::core::ffi::c_int + newsize,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
    }
    i = oldsize + 1 as ::core::ffi::c_int;
    while i <= newsize {
        *(*SOS).members.offset(i as isize) =
            *variables.offset((i - oldsize - 1 as ::core::ffi::c_int) as isize);
        if *(*SOS).members.offset(i as isize) < 1 as ::core::ffi::c_int
            || *(*SOS).members.offset(i as isize) > (*lp).columns
        {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"append_SOS_rec: Invalid SOS variable definition for index %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if (*SOS).isGUB != 0 {
            let ref mut fresh7 = *(*lp)
                .var_type
                .offset(*(*SOS).members.offset(i as isize) as isize);
            *fresh7 = (*fresh7 as ::core::ffi::c_int | ISGUB) as ::core::ffi::c_uchar;
        } else {
            let ref mut fresh8 = *(*lp)
                .var_type
                .offset(*(*SOS).members.offset(i as isize) as isize);
            *fresh8 = (*fresh8 as ::core::ffi::c_int | ISSOS) as ::core::ffi::c_uchar;
        }
        if weights.is_null() {
            *(*SOS).weights.offset(i as isize) = i as ::core::ffi::c_double;
        } else {
            *(*SOS).weights.offset(i as isize) =
                *weights.offset((i - oldsize - 1 as ::core::ffi::c_int) as isize);
        }
        *(*SOS).weights.offset(0 as ::core::ffi::c_int as isize) +=
            *(*SOS).weights.offset(i as isize);
        i += 1;
    }
    i = sortByREAL(
        (*SOS).members,
        (*SOS).weights,
        newsize,
        1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    if i > 0 as ::core::ffi::c_int {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"append_SOS_rec: Non-unique SOS variable weight for index %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    allocINT(
        lp,
        &raw mut (*SOS).membersSorted,
        newsize,
        AUTOMATIC as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut (*SOS).membersMapped,
        newsize,
        AUTOMATIC as ::core::ffi::c_uchar,
    );
    i = oldsize + 1 as ::core::ffi::c_int;
    while i <= newsize {
        *(*SOS)
            .membersSorted
            .offset((i - 1 as ::core::ffi::c_int) as isize) = *(*SOS).members.offset(i as isize);
        *(*SOS)
            .membersMapped
            .offset((i - 1 as ::core::ffi::c_int) as isize) = i;
        i += 1;
    }
    sortByINT(
        (*SOS).membersMapped,
        (*SOS).membersSorted,
        newsize,
        0 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    (*SOS).size = newsize;
    return newsize;
}
#[export_name="honest_lpsolve_make_SOSchain"]
pub unsafe extern "C" fn make_SOSchain(
    mut lp: *mut lprec,
    mut forceresort: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut hold: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut order: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut sum: ::core::ffi::c_double = 0.;
    let mut weight: ::core::ffi::c_double = 0.;
    let mut group: *mut SOSgroup = (*lp).SOS;
    if forceresort != 0 {
        SOS_member_sortlist(group, 0 as ::core::ffi::c_int);
    }
    n = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*group).sos_count {
        n += (**(*group).sos_list.offset(i as isize)).size;
        i += 1;
    }
    (*lp).sos_vars = n;
    if (*lp).sos_vars > 0 as ::core::ffi::c_int {
        if !((*lp).sos_priority as *mut ::core::ffi::c_void).is_null() {
            free((*lp).sos_priority as *mut ::core::ffi::c_void);
            (*lp).sos_priority = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    allocINT(
        lp,
        &raw mut (*lp).sos_priority,
        n,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(lp, &raw mut order, n, FALSE as ::core::ffi::c_uchar);
    n = 0 as ::core::ffi::c_int;
    sum = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while i < (*group).sos_count {
        j = 1 as ::core::ffi::c_int;
        while j <= (**(*group).sos_list.offset(i as isize)).size {
            *(*lp).sos_priority.offset(n as isize) = *(**(*group).sos_list.offset(i as isize))
                .members
                .offset(j as isize);
            weight = *(**(*group).sos_list.offset(i as isize))
                .weights
                .offset(j as isize);
            sum += weight;
            *order.offset(n as isize) = sum;
            n += 1;
            j += 1;
        }
        i += 1;
    }
    hpsortex(
        order as *mut ::core::ffi::c_void,
        n,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
        Some(
            compareREAL
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        (*lp).sos_priority,
    );
    if !(order as *mut ::core::ffi::c_void).is_null() {
        free(order as *mut ::core::ffi::c_void);
        order = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    allocMYBOOL(
        lp,
        &raw mut hold,
        (*lp).columns + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    k = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        j = *(*lp).sos_priority.offset(i as isize);
        if *hold.offset(j as isize) == 0 {
            *hold.offset(j as isize) = TRUE as ::core::ffi::c_uchar;
            if k < i {
                *(*lp).sos_priority.offset(k as isize) = j;
            }
            k += 1;
        }
        i += 1;
    }
    if !(hold as *mut ::core::ffi::c_void).is_null() {
        free(hold as *mut ::core::ffi::c_void);
        hold = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if k < (*lp).sos_vars {
        allocINT(
            lp,
            &raw mut (*lp).sos_priority,
            k,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        (*lp).sos_vars = k;
    }
    return k;
}
#[export_name="honest_lpsolve_delete_SOSrec"]
pub unsafe extern "C" fn delete_SOSrec(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if abs(SOS_get_type(group, sosindex)) == 1 as ::core::ffi::c_int {
        (*group).sos1_count -= 1;
    }
    free_SOSrec(
        *(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize),
    );
    while sosindex < (*group).sos_count {
        let ref mut fresh6 = *(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize);
        *fresh6 = *(*group).sos_list.offset(sosindex as isize);
        sosindex += 1;
    }
    (*group).sos_count -= 1;
    (*group).maxorder = 0 as ::core::ffi::c_int;
    sosindex = 0 as ::core::ffi::c_int;
    while sosindex < (*group).sos_count {
        if (*group).maxorder < abs((**(*group).sos_list.offset(sosindex as isize)).type_0) {
            (*group).maxorder = abs((**(*group).sos_list.offset(sosindex as isize)).type_0);
        }
        sosindex += 1;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_free_SOSrec"]
pub unsafe extern "C" fn free_SOSrec(mut SOS: *mut SOSrec) {
    if !(*SOS).name.is_null() {
        if !((*SOS).name as *mut ::core::ffi::c_void).is_null() {
            free((*SOS).name as *mut ::core::ffi::c_void);
            (*SOS).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if (*SOS).size > 0 as ::core::ffi::c_int {
        if !((*SOS).members as *mut ::core::ffi::c_void).is_null() {
            free((*SOS).members as *mut ::core::ffi::c_void);
            (*SOS).members = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !((*SOS).weights as *mut ::core::ffi::c_void).is_null() {
            free((*SOS).weights as *mut ::core::ffi::c_void);
            (*SOS).weights = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !((*SOS).membersSorted as *mut ::core::ffi::c_void).is_null() {
            free((*SOS).membersSorted as *mut ::core::ffi::c_void);
            (*SOS).membersSorted = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !((*SOS).membersMapped as *mut ::core::ffi::c_void).is_null() {
            free((*SOS).membersMapped as *mut ::core::ffi::c_void);
            (*SOS).membersMapped = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    if !(SOS as *mut ::core::ffi::c_void).is_null() {
        free(SOS as *mut ::core::ffi::c_void);
        SOS = ::core::ptr::null_mut::<SOSrec>();
    }
}
#[export_name="honest_lpsolve_SOS_member_sortlist"]
pub unsafe extern "C" fn SOS_member_sortlist(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = (*group).lp;
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    if sosindex == 0 as ::core::ffi::c_int && (*group).sos_count == 1 as ::core::ffi::c_int {
        sosindex = 1 as ::core::ffi::c_int;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= (*group).sos_count {
            if SOS_member_sortlist(group, i) == 0 {
                return 0 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    } else {
        SOS = *(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize);
        list = (*SOS).members;
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        if n != (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .size
        {
            allocINT(
                lp,
                &raw mut (*SOS).membersSorted,
                n,
                AUTOMATIC as ::core::ffi::c_uchar,
            );
            allocINT(
                lp,
                &raw mut (*SOS).membersMapped,
                n,
                AUTOMATIC as ::core::ffi::c_uchar,
            );
            (**(*group)
                .sos_list
                .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
            .size = n;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            *(*SOS)
                .membersSorted
                .offset((i - 1 as ::core::ffi::c_int) as isize) = *list.offset(i as isize);
            *(*SOS)
                .membersMapped
                .offset((i - 1 as ::core::ffi::c_int) as isize) = i;
            i += 1;
        }
        sortByINT(
            (*SOS).membersMapped,
            (*SOS).membersSorted,
            n,
            0 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_member_updatemap"]
pub unsafe extern "C" fn SOS_member_updatemap(mut group: *mut SOSgroup) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nvars: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut tally: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rec: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    let mut lp: *mut lprec = (*group).lp;
    allocINT(
        lp,
        &raw mut (*group).memberpos,
        (*lp).columns + 1 as ::core::ffi::c_int,
        AUTOMATIC as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut tally,
        (*lp).columns + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*group).sos_count {
        rec = *(*group).sos_list.offset(i as isize);
        n = (*rec).size;
        list = (*rec).members;
        j = 1 as ::core::ffi::c_int;
        while j <= n {
            k = *list.offset(j as isize);
            let ref mut fresh3 = *tally.offset(k as isize);
            *fresh3 += 1;
            j += 1;
        }
        i += 1;
    }
    *(*group).memberpos.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        n = *tally.offset(i as isize);
        if n > 0 as ::core::ffi::c_int {
            nvars += 1;
        }
        *(*group).memberpos.offset(i as isize) = *(*group)
            .memberpos
            .offset((i - 1 as ::core::ffi::c_int) as isize)
            + n;
        i += 1;
    }
    n = *(*group).memberpos.offset((*lp).columns as isize);
    memcpy(
        tally.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        (*group).memberpos as *const ::core::ffi::c_void,
        ((*lp).columns as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    allocINT(
        lp,
        &raw mut (*group).membership,
        n + 1 as ::core::ffi::c_int,
        AUTOMATIC as ::core::ffi::c_uchar,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*group).sos_count {
        rec = *(*group).sos_list.offset(i as isize);
        n = (*rec).size;
        list = (*rec).members;
        j = 1 as ::core::ffi::c_int;
        while j <= n {
            let ref mut fresh4 = *tally.offset(*list.offset(j as isize) as isize);
            let fresh5 = *fresh4;
            *fresh4 = *fresh4 + 1;
            k = fresh5;
            *(*group).membership.offset(k as isize) = i + 1 as ::core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
    if !(tally as *mut ::core::ffi::c_void).is_null() {
        free(tally as *mut ::core::ffi::c_void);
        tally = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return nvars;
}
#[export_name="honest_lpsolve_SOS_shift_col"]
pub unsafe extern "C" fn SOS_shift_col(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut usedmap: *mut LLrec,
    mut forceresort: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut nr: ::core::ffi::c_int = 0;
    let mut changed: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut weights: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if sosindex == 0 as ::core::ffi::c_int && (*group).sos_count == 1 as ::core::ffi::c_int {
        sosindex = 1 as ::core::ffi::c_int;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= (*group).sos_count {
            if SOS_shift_col(group, i, column, delta, usedmap, forceresort) == 0 {
                return 0 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        weights = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .weights;
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        nn = *list.offset((n + 1 as ::core::ffi::c_int) as isize);
        if delta > 0 as ::core::ffi::c_int {
            i = 1 as ::core::ffi::c_int;
            while i <= n {
                if *list.offset(i as isize) >= column {
                    *list.offset(i as isize) += delta;
                }
                i += 1;
            }
        } else {
            changed = 0 as ::core::ffi::c_int;
            if !usedmap.is_null() {
                let mut newidx: *mut ::core::ffi::c_int =
                    ::core::ptr::null_mut::<::core::ffi::c_int>();
                if newidx.is_null() {
                    allocINT(
                        (*group).lp,
                        &raw mut newidx,
                        (*(*group).lp).columns + 1 as ::core::ffi::c_int,
                        TRUE as ::core::ffi::c_uchar,
                    );
                    i = firstActiveLink(usedmap);
                    ii = 1 as ::core::ffi::c_int;
                    while i != 0 as ::core::ffi::c_int {
                        *newidx.offset(i as isize) = ii;
                        i = nextActiveLink(usedmap, i);
                        ii += 1;
                    }
                }
                i = 1 as ::core::ffi::c_int;
                ii = 0 as ::core::ffi::c_int;
                while i <= n {
                    nr = *list.offset(i as isize);
                    if !(isActiveLink(usedmap, nr) == 0) {
                        changed += 1;
                        ii += 1;
                        *list.offset(ii as isize) = *newidx.offset(nr as isize);
                        *weights.offset(ii as isize) = *weights.offset(i as isize);
                    }
                    i += 1;
                }
                if !(newidx as *mut ::core::ffi::c_void).is_null() {
                    free(newidx as *mut ::core::ffi::c_void);
                    newidx = ::core::ptr::null_mut::<::core::ffi::c_int>();
                }
            } else {
                i = 1 as ::core::ffi::c_int;
                ii = 0 as ::core::ffi::c_int;
                while i <= n {
                    nr = *list.offset(i as isize);
                    if !(nr >= column && nr < column - delta) {
                        if nr > column {
                            changed += 1;
                            nr += delta;
                        }
                        ii += 1;
                        *list.offset(ii as isize) = nr;
                        *weights.offset(ii as isize) = *weights.offset(i as isize);
                    }
                    i += 1;
                }
            }
            if ii < n {
                *list.offset(0 as ::core::ffi::c_int as isize) = ii;
                *list.offset((ii + 1 as ::core::ffi::c_int) as isize) = nn;
            }
            if forceresort as ::core::ffi::c_int != 0
                && (ii < n || changed > 0 as ::core::ffi::c_int)
            {
                SOS_member_sortlist(group, sosindex);
            }
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_member_count"]
pub unsafe extern "C" fn SOS_member_count(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    SOS = *(*group)
        .sos_list
        .offset((sosindex - 1 as ::core::ffi::c_int) as isize);
    return *(*SOS).members.offset(0 as ::core::ffi::c_int as isize);
}
#[export_name="honest_lpsolve_SOS_member_delete"]
pub unsafe extern "C" fn SOS_member_delete(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut member: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut i: ::core::ffi::c_int = 0;
    let mut i2: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    let mut lp: *mut lprec = (*group).lp;
    if sosindex == 0 as ::core::ffi::c_int {
        i = *(*group)
            .memberpos
            .offset((member - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(member as isize) {
            k = *(*group).membership.offset(i as isize);
            n = SOS_member_delete(group, k, member);
            if n >= 0 as ::core::ffi::c_int {
                nn += n;
            } else {
                return n;
            }
            i += 1;
        }
        k = *(*group).memberpos.offset(member as isize);
        i = *(*group)
            .memberpos
            .offset((member - 1 as ::core::ffi::c_int) as isize);
        n = *(*group).memberpos.offset((*lp).columns as isize) - k;
        if n > 0 as ::core::ffi::c_int {
            memcpy(
                (*group).membership.offset(i as isize) as *mut ::core::ffi::c_void,
                (*group).membership.offset(k as isize) as *const ::core::ffi::c_void,
                (n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
        }
        i = member;
        while i <= (*lp).columns {
            *(*group).memberpos.offset(i as isize) = *(*group)
                .memberpos
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    } else {
        SOS = *(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize);
        list = (*SOS).members;
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= n && abs(*list.offset(i as isize)) != member {
            i += 1;
        }
        if i > n {
            return -(1 as ::core::ffi::c_int);
        }
        nn += 1;
        while i <= n {
            *list.offset(i as isize) = *list.offset((i + 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        let ref mut fresh9 = *list.offset(0 as ::core::ffi::c_int as isize);
        *fresh9 -= 1;
        (*SOS).size -= 1;
        i = n + 1 as ::core::ffi::c_int;
        i2 = i + *list.offset(n as isize);
        k = i + 1 as ::core::ffi::c_int;
        while i < i2 {
            if abs(*list.offset(k as isize)) == member {
                k += 1;
            }
            *list.offset(i as isize) = *list.offset(k as isize);
            i += 1;
            k += 1;
        }
    }
    return nn;
}
#[export_name="honest_lpsolve_SOS_get_type"]
pub unsafe extern "C" fn SOS_get_type(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (**(*group)
        .sos_list
        .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
    .type_0;
}
#[export_name="honest_lpsolve_SOS_infeasible"]
pub unsafe extern "C" fn SOS_infeasible(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut varnr: ::core::ffi::c_int = 0;
    let mut failindex: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = (*group).lp;
    if sosindex == 0 as ::core::ffi::c_int && (*group).sos_count == 1 as ::core::ffi::c_int {
        sosindex = 1 as ::core::ffi::c_int;
    }
    failindex = 0 as ::core::ffi::c_int;
    if sosindex == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= (*group).sos_count {
            failindex = SOS_infeasible(group, i);
            if failindex > 0 as ::core::ffi::c_int {
                break;
            }
            i += 1;
        }
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        nn = *list.offset((n + 1 as ::core::ffi::c_int) as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            varnr = abs(*list.offset(i as isize));
            if *(*lp).orig_lowbo.offset(((*lp).rows + varnr) as isize)
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && !((*lp).sc_vars > 0 as ::core::ffi::c_int
                    && is_semicont(lp, varnr) as ::core::ffi::c_int != 0)
            {
                break;
            }
            i += 1;
        }
        i = i + nn;
        while i <= n {
            varnr = abs(*list.offset(i as isize));
            if *(*lp).orig_lowbo.offset(((*lp).rows + varnr) as isize)
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && !((*lp).sc_vars > 0 as ::core::ffi::c_int
                    && is_semicont(lp, varnr) as ::core::ffi::c_int != 0)
            {
                break;
            }
            i += 1;
        }
        if i <= n {
            failindex = abs(*list.offset(i as isize));
        }
    }
    return failindex;
}
#[export_name="honest_lpsolve_SOS_member_index"]
pub unsafe extern "C" fn SOS_member_index(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut member: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    SOS = *(*group)
        .sos_list
        .offset((sosindex - 1 as ::core::ffi::c_int) as isize);
    n = *(*SOS).members.offset(0 as ::core::ffi::c_int as isize);
    n = searchFor(
        member,
        (*SOS).membersSorted,
        n,
        0 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    if n >= 0 as ::core::ffi::c_int {
        n = *(*SOS).membersMapped.offset(n as isize);
    }
    return n;
}
#[export_name="honest_lpsolve_SOS_memberships"]
pub unsafe extern "C" fn SOS_memberships(
    mut group: *mut SOSgroup,
    mut varnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    if group.is_null() || {
        lp = (*group).lp;
        SOS_count(lp) == 0 as ::core::ffi::c_int
    } {
        return n;
    }
    if varnr == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).columns {
            if *(*group).memberpos.offset(i as isize)
                > *(*group)
                    .memberpos
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
            {
                n += 1;
            }
            i += 1;
        }
    } else {
        n = *(*group).memberpos.offset(varnr as isize)
            - *(*group)
                .memberpos
                .offset((varnr - 1 as ::core::ffi::c_int) as isize);
    }
    return n;
}
#[export_name="honest_lpsolve_SOS_is_member"]
pub unsafe extern "C" fn SOS_is_member(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = FALSE;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    if group.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    lp = (*group).lp;
    if sosindex == 0 as ::core::ffi::c_int {
        if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) != 0 {
            n = (SOS_memberships(group, column) > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uchar as ::core::ffi::c_int;
        }
    } else if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) != 0 {
        i = SOS_member_index(group, sosindex, column);
        if i > 0 as ::core::ffi::c_int {
            list = (**(*group)
                .sos_list
                .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
            .members;
            if *list.offset(i as isize) < 0 as ::core::ffi::c_int {
                n = -TRUE;
            } else {
                n = TRUE;
            }
        }
    }
    return n;
}
#[export_name="honest_lpsolve_SOS_is_member_of_type"]
pub unsafe extern "C" fn SOS_is_member_of_type(
    mut group: *mut SOSgroup,
    mut column: ::core::ffi::c_int,
    mut sostype: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if !group.is_null() {
        i = *(*group)
            .memberpos
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(column as isize) {
            k = *(*group).membership.offset(i as isize);
            n = SOS_get_type(group, k);
            if (n == sostype || sostype == SOSn && n > 2 as ::core::ffi::c_int)
                && SOS_is_member(group, k, column) != 0
            {
                return 1 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_set_GUB"]
pub unsafe extern "C" fn SOS_set_GUB(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut state: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    if sosindex == 0 as ::core::ffi::c_int && (*group).sos_count == 1 as ::core::ffi::c_int {
        sosindex = 1 as ::core::ffi::c_int;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= (*group).sos_count {
            SOS_set_GUB(group, i, state);
            i += 1;
        }
    } else {
        (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .isGUB = state;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_is_GUB"]
pub unsafe extern "C" fn SOS_is_GUB(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    if sosindex == 0 as ::core::ffi::c_int && (*group).sos_count == 1 as ::core::ffi::c_int {
        sosindex = 1 as ::core::ffi::c_int;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= (*group).sos_count {
            if SOS_is_GUB(group, i) != 0 {
                return 1 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_uchar;
    } else {
        return (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .isGUB;
    };
}
#[export_name="honest_lpsolve_SOS_is_marked"]
pub unsafe extern "C" fn SOS_is_marked(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    if group.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    lp = (*group).lp;
    if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = *(*group)
            .memberpos
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(column as isize) {
            k = *(*group).membership.offset(i as isize);
            n = SOS_is_marked(group, k, column) as ::core::ffi::c_int;
            if n != 0 {
                return 1 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        column = -column;
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if *list.offset(i as isize) == column {
                return 1 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_is_active"]
pub unsafe extern "C" fn SOS_is_active(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = (*group).lp;
    if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = *(*group)
            .memberpos
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(column as isize) {
            nn = *(*group).membership.offset(i as isize);
            n = SOS_is_active(group, nn, column) as ::core::ffi::c_int;
            if n != 0 {
                return 1 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= nn && *list.offset((n + i) as isize) != 0 as ::core::ffi::c_int {
            if *list.offset((n + i) as isize) == column {
                return 1 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_is_full"]
pub unsafe extern "C" fn SOS_is_full(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
    mut activeonly: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = (*group).lp;
    if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = *(*group)
            .memberpos
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(column as isize) {
            nn = *(*group).membership.offset(i as isize);
            if SOS_is_full(group, nn, column, activeonly) != 0 {
                return 1 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    } else if SOS_is_member(group, sosindex, column) != 0 {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        if *list.offset((n + nn) as isize) != 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_uchar;
        }
        if activeonly == 0 {
            i = nn - 1 as ::core::ffi::c_int;
            while i > 0 as ::core::ffi::c_int
                && *list.offset((n + i) as isize) == 0 as ::core::ffi::c_int
            {
                i -= 1;
            }
            if i > 0 as ::core::ffi::c_int {
                nn -= i;
                i = SOS_member_index(group, sosindex, *list.offset((n + i) as isize));
                while nn > 0 as ::core::ffi::c_int
                    && *list.offset(i as isize) < 0 as ::core::ffi::c_int
                {
                    i += 1;
                    nn -= 1;
                }
                if nn == 0 as ::core::ffi::c_int {
                    return 1 as ::core::ffi::c_uchar;
                }
            }
        }
    }
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_can_activate"]
pub unsafe extern "C" fn SOS_can_activate(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    if group.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    lp = (*group).lp;
    if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = *(*group)
            .memberpos
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(column as isize) {
            nn = *(*group).membership.offset(i as isize);
            n = SOS_can_activate(group, nn, column) as ::core::ffi::c_int;
            if n == FALSE {
                return 0 as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    } else if SOS_is_member(group, sosindex, column) != 0 {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        if *list.offset((n + nn) as isize) != 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_uchar;
        }
        nz = 0 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i < n {
            if *(*(*lp).bb_bounds)
                .lowbo
                .offset(((*lp).rows + abs(*list.offset(i as isize))) as isize)
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                nz += 1;
                if *list.offset(i as isize) == column {
                    return 0 as ::core::ffi::c_uchar;
                }
            }
            i += 1;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= nn {
            if *list.offset((n + i) as isize) == 0 as ::core::ffi::c_int {
                break;
            }
            if *(*(*lp).bb_bounds)
                .lowbo
                .offset(((*lp).rows + *list.offset((n + i) as isize)) as isize)
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                nz += 1;
            }
            i += 1;
        }
        if nz == nn {
            return 0 as ::core::ffi::c_uchar;
        }
        if *list.offset((n + 1 as ::core::ffi::c_int) as isize) == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_uchar;
        }
        if nn > 1 as ::core::ffi::c_int {
            i = 1 as ::core::ffi::c_int;
            while i <= nn {
                if *list.offset((n + i) as isize) == 0 as ::core::ffi::c_int {
                    break;
                }
                if *list.offset((n + i) as isize) == column {
                    return 0 as ::core::ffi::c_uchar;
                }
                i += 1;
            }
            i -= 1;
            nn = *list.offset((n + i) as isize);
            n = *list.offset(0 as ::core::ffi::c_int as isize);
            i = 1 as ::core::ffi::c_int;
            while i <= n {
                if abs(*list.offset(i as isize)) == nn {
                    break;
                }
                i += 1;
            }
            if i > n {
                report(
                    lp,
                    1 as ::core::ffi::c_int,
                    b"SOS_can_activate: Internal index error at SOS %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_uchar;
            }
            if i > 1 as ::core::ffi::c_int
                && *list.offset((i - 1 as ::core::ffi::c_int) as isize) == column
            {
                return 1 as ::core::ffi::c_uchar;
            }
            if i < n && *list.offset((i + 1 as ::core::ffi::c_int) as isize) == column {
                return 1 as ::core::ffi::c_uchar;
            }
            return 0 as ::core::ffi::c_uchar;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_SOS_set_marked"]
pub unsafe extern "C" fn SOS_set_marked(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
    mut asactive: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = (*group).lp;
    if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        if asactive as ::core::ffi::c_int != 0
            && is_int(lp, column) == 0
            && SOS_is_member_of_type(group, column, SOS3) as ::core::ffi::c_int != 0
        {
            let ref mut fresh11 = *(*lp).var_type.offset(column as isize);
            *fresh11 = (*fresh11 as ::core::ffi::c_int | ISSOSTEMPINT) as ::core::ffi::c_uchar;
            set_int(lp, column, TRUE as ::core::ffi::c_uchar);
        }
        nn = 0 as ::core::ffi::c_int;
        i = *(*group)
            .memberpos
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(column as isize) {
            n = *(*group).membership.offset(i as isize);
            if SOS_set_marked(group, n, column, asactive) != 0 {
                nn += 1;
            }
            i += 1;
        }
        return (nn == (*group).sos_count) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        i = SOS_member_index(group, sosindex, column);
        if i > 0 as ::core::ffi::c_int && *list.offset(i as isize) > 0 as ::core::ffi::c_int {
            *list.offset(i as isize) *= -(1 as ::core::ffi::c_int);
        } else {
            return 1 as ::core::ffi::c_uchar;
        }
        if asactive != 0 {
            i = 1 as ::core::ffi::c_int;
            while i <= nn {
                if *list.offset((n + i) as isize) == column {
                    return 0 as ::core::ffi::c_uchar;
                } else if *list.offset((n + i) as isize) == 0 as ::core::ffi::c_int {
                    *list.offset((n + i) as isize) = column;
                    return 0 as ::core::ffi::c_uchar;
                }
                i += 1;
            }
        }
        return 1 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_SOS_unmark"]
pub unsafe extern "C" fn SOS_unmark(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut isactive: ::core::ffi::c_uchar = 0;
    let mut lp: *mut lprec = (*group).lp;
    if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & (ISSOS | ISGUB) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        if *(*lp).var_type.offset(column as isize) as ::core::ffi::c_int & ISSOSTEMPINT != 0 {
            let ref mut fresh12 = *(*lp).var_type.offset(column as isize);
            *fresh12 = (*fresh12 as ::core::ffi::c_int & (ISSOSTEMPINT == 0) as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            set_int(lp, column, FALSE as ::core::ffi::c_uchar);
        }
        nn = 0 as ::core::ffi::c_int;
        i = *(*group)
            .memberpos
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(column as isize) {
            n = *(*group).membership.offset(i as isize);
            if SOS_unmark(group, n, column) != 0 {
                nn += 1;
            }
            i += 1;
        }
        return (nn == (*group).sos_count) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        i = SOS_member_index(group, sosindex, column);
        if i > 0 as ::core::ffi::c_int && *list.offset(i as isize) < 0 as ::core::ffi::c_int {
            *list.offset(i as isize) *= -(1 as ::core::ffi::c_int);
        } else {
            return 1 as ::core::ffi::c_uchar;
        }
        isactive = SOS_is_active(group, sosindex, column);
        if isactive != 0 {
            i = 1 as ::core::ffi::c_int;
            while i <= nn {
                if *list.offset((n + i) as isize) == column {
                    break;
                }
                i += 1;
            }
            if i <= nn {
                while i < nn {
                    *list.offset((n + i) as isize) =
                        *list.offset((n + i + 1 as ::core::ffi::c_int) as isize);
                    i += 1;
                }
                *list.offset((n + nn) as isize) = 0 as ::core::ffi::c_int;
                return 1 as ::core::ffi::c_uchar;
            }
            return 0 as ::core::ffi::c_uchar;
        } else {
            return 1 as ::core::ffi::c_uchar;
        }
    };
}
#[export_name="honest_lpsolve_SOS_fix_unmarked"]
pub unsafe extern "C" fn SOS_fix_unmarked(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut variable: ::core::ffi::c_int,
    mut bound: *mut ::core::ffi::c_double,
    mut value: ::core::ffi::c_double,
    mut isupper: ::core::ffi::c_uchar,
    mut diffcount: *mut ::core::ffi::c_int,
    mut changelog: *mut DeltaVrec,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut nLeft: ::core::ffi::c_int = 0;
    let mut nRight: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = (*group).lp;
    count = 0 as ::core::ffi::c_int;
    if sosindex == 0 as ::core::ffi::c_int {
        i = *(*group)
            .memberpos
            .offset((variable - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(variable as isize) {
            n = *(*group).membership.offset(i as isize);
            count += SOS_fix_unmarked(
                group, n, variable, bound, value, isupper, diffcount, changelog,
            );
            i += 1;
        }
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= nn {
            if *list.offset((n + i) as isize) == 0 as ::core::ffi::c_int {
                break;
            }
            i += 1;
        }
        i -= 1;
        i = nn - i;
        if i == nn {
            nLeft = 0 as ::core::ffi::c_int;
            nRight = SOS_member_index(group, sosindex, variable);
        } else {
            nLeft = SOS_member_index(
                group,
                sosindex,
                *list.offset((n + 1 as ::core::ffi::c_int) as isize),
            );
            if variable == *list.offset((n + 1 as ::core::ffi::c_int) as isize) {
                nRight = nLeft;
            } else {
                nRight = SOS_member_index(group, sosindex, variable);
            }
        }
        nRight += i;
        i = 1 as ::core::ffi::c_int;
        while i < n {
            if !(i >= nLeft && i <= nRight) {
                ii = *list.offset(i as isize);
                if ii > 0 as ::core::ffi::c_int {
                    ii += (*lp).rows;
                    if *bound.offset(ii as isize) != value {
                        if isupper as ::core::ffi::c_int != 0
                            && value < *(*lp).orig_lowbo.offset(ii as isize)
                        {
                            return -ii;
                        } else if isupper == 0 && value > *(*lp).orig_upbo.offset(ii as isize) {
                            return -ii;
                        }
                        count += 1;
                        if changelog.is_null() {
                            *bound.offset(ii as isize) = value;
                        } else {
                            modifyUndoLadder(
                                changelog,
                                ii,
                                bound as *mut ::core::ffi::c_double,
                                value,
                            );
                        }
                    }
                    if !diffcount.is_null() && *(*lp).solution.offset(ii as isize) != value {
                        *diffcount += 1;
                    }
                }
            }
            i += 1;
        }
    }
    return count;
}
#[export_name="honest_lpsolve_SOS_get_candidates"]
pub unsafe extern "C" fn SOS_get_candidates(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
    mut excludetarget: ::core::ffi::c_uchar,
    mut upbound: *mut ::core::ffi::c_double,
    mut lobound: *mut ::core::ffi::c_double,
) -> *mut ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut candidates: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lp: *mut lprec = (*group).lp;
    if group.is_null() {
        return candidates;
    }
    if sosindex <= 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        ii = (*group).sos_count;
    } else {
        i = sosindex - 1 as ::core::ffi::c_int;
        ii = sosindex;
    }
    allocINT(
        lp,
        &raw mut candidates,
        (*lp).columns + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    's_42: loop {
        if !(i < ii) {
            current_block = 2668756484064249700;
            break;
        }
        if !(SOS_is_member(group, i + 1 as ::core::ffi::c_int, column) == 0) {
            list = (**(*group).sos_list.offset(i as isize)).members;
            n = *list.offset(0 as ::core::ffi::c_int as isize);
            while n > 0 as ::core::ffi::c_int {
                j = *list.offset(n as isize);
                if j > 0 as ::core::ffi::c_int
                    && *upbound.offset(((*lp).rows + j) as isize)
                        > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    if *lobound.offset(((*lp).rows + j) as isize)
                        > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        report(
                            lp,
                            3 as ::core::ffi::c_int,
                            b"SOS_get_candidates: Invalid non-zero lower bound setting\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        n = 0 as ::core::ffi::c_int;
                        current_block = 14639573743508277779;
                        break 's_42;
                    } else {
                        if *candidates.offset(j as isize) == 0 as ::core::ffi::c_int {
                            nn += 1;
                        }
                        let ref mut fresh10 = *candidates.offset(j as isize);
                        *fresh10 += 1;
                    }
                }
                n -= 1;
            }
            if sosindex < 0 as ::core::ffi::c_int && nn > 1 as ::core::ffi::c_int {
                current_block = 2668756484064249700;
                break;
            }
        }
        i += 1;
    }
    match current_block {
        2668756484064249700 => {
            n = 0 as ::core::ffi::c_int;
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                if *candidates.offset(i as isize) > 0 as ::core::ffi::c_int
                    && (excludetarget == 0 || i != column)
                {
                    n += 1;
                    *candidates.offset(n as isize) = i;
                }
                i += 1;
            }
        }
        _ => {}
    }
    *candidates.offset(0 as ::core::ffi::c_int as isize) = n;
    if n == 0 as ::core::ffi::c_int {
        if !(candidates as *mut ::core::ffi::c_void).is_null() {
            free(candidates as *mut ::core::ffi::c_void);
            candidates = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    return candidates;
}
#[export_name="honest_lpsolve_SOS_fix_list"]
pub unsafe extern "C" fn SOS_fix_list(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut variable: ::core::ffi::c_int,
    mut bound: *mut ::core::ffi::c_double,
    mut varlist: *mut ::core::ffi::c_int,
    mut isleft: ::core::ffi::c_uchar,
    mut changelog: *mut DeltaVrec,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut value: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut lp: *mut lprec = (*group).lp;
    if sosindex == 0 as ::core::ffi::c_int {
        i = *(*group)
            .memberpos
            .offset((variable - 1 as ::core::ffi::c_int) as isize);
        while i < *(*group).memberpos.offset(variable as isize) {
            ii = *(*group).membership.offset(i as isize);
            count += SOS_fix_list(group, ii, variable, bound, varlist, isleft, changelog);
            i += 1;
        }
    } else {
        ii = *varlist.offset(0 as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int;
        if isleft != 0 {
            i = 1 as ::core::ffi::c_int;
            if isleft as ::core::ffi::c_int == AUTOMATIC {
                ii = *varlist.offset(0 as ::core::ffi::c_int as isize);
            }
        } else {
            i = ii + 1 as ::core::ffi::c_int;
            ii = *varlist.offset(0 as ::core::ffi::c_int as isize);
        }
        while i <= ii {
            if SOS_is_member(group, sosindex, *varlist.offset(i as isize)) != 0 {
                jj = (*lp).rows + *varlist.offset(i as isize);
                if value < *(*lp).orig_lowbo.offset(jj as isize) {
                    return -jj;
                }
                count += 1;
                if changelog.is_null() {
                    *bound.offset(jj as isize) = value;
                } else {
                    modifyUndoLadder(changelog, jj, bound as *mut ::core::ffi::c_double, value);
                }
            }
            i += 1;
        }
    }
    return count;
}
#[export_name="honest_lpsolve_SOS_is_satisfied"]
pub unsafe extern "C" fn SOS_is_satisfied(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut solution: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut type_0: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lp: *mut lprec = (*group).lp;
    if sosindex == 0 as ::core::ffi::c_int && (*group).sos_count == 1 as ::core::ffi::c_int {
        sosindex = 1 as ::core::ffi::c_int;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= (*group).sos_count {
            status = SOS_is_satisfied(group, i, solution);
            if status != SOS_COMPLETE && status != SOS_INCOMPLETE {
                break;
            }
            i += 1;
        }
    } else {
        type_0 = SOS_get_type(group, sosindex);
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= nn {
            if *list.offset((n + i) as isize) == 0 as ::core::ffi::c_int {
                break;
            }
            i += 1;
        }
        count = i - 1 as ::core::ffi::c_int;
        if count == nn {
            status = SOS_COMPLETE;
        } else {
            status = SOS_INCOMPLETE;
        }
        if count > 0 as ::core::ffi::c_int {
            nn = *list.offset((n + 1 as ::core::ffi::c_int) as isize);
            i = 1 as ::core::ffi::c_int;
            while i < n {
                if abs(*list.offset(i as isize)) == nn
                    || *solution.offset(((*lp).rows + abs(*list.offset(i as isize))) as isize)
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    break;
                }
                i += 1;
            }
            if abs(*list.offset(i as isize)) != nn {
                status = SOS_INTERNALERROR;
            } else {
                while count > 0 as ::core::ffi::c_int {
                    if *solution.offset(((*lp).rows + abs(*list.offset(i as isize))) as isize)
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        break;
                    }
                    i += 1;
                    count -= 1;
                }
                while count > 0 as ::core::ffi::c_int {
                    if *solution.offset(((*lp).rows + abs(*list.offset(i as isize))) as isize)
                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        break;
                    }
                    i += 1;
                    count -= 1;
                }
                if count > 0 as ::core::ffi::c_int {
                    status = SOS_INTERNALERROR;
                }
            }
        } else {
            i = 1 as ::core::ffi::c_int;
            while i < n
                && *solution.offset(((*lp).rows + abs(*list.offset(i as isize))) as isize)
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                i += 1;
            }
            count = 0 as ::core::ffi::c_int;
            while i < n
                && count <= nn
                && *solution.offset(((*lp).rows + abs(*list.offset(i as isize))) as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                count += 1;
                i += 1;
            }
            if count > nn {
                status = SOS_INFEASIBLE;
            }
        }
        if status <= 0 as ::core::ffi::c_int {
            n -= 1;
            while i <= n {
                if *solution.offset(((*lp).rows + abs(*list.offset(i as isize))) as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    break;
                }
                i += 1;
            }
            if i <= n {
                status = SOS_INFEASIBLE;
            } else if status == -(1 as ::core::ffi::c_int) && type_0 <= SOS3 {
                status = SOS3_INCOMPLETE;
            }
        }
    }
    return status;
}
#[export_name="honest_lpsolve_SOS_is_feasible"]
pub unsafe extern "C" fn SOS_is_feasible(
    mut group: *mut SOSgroup,
    mut sosindex: ::core::ffi::c_int,
    mut solution: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut lp: *mut lprec = (*group).lp;
    if sosindex == 0 as ::core::ffi::c_int && (*group).sos_count == 1 as ::core::ffi::c_int {
        sosindex = 1 as ::core::ffi::c_int;
    }
    if sosindex == 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while status as ::core::ffi::c_int != 0 && i <= (*group).sos_count {
            status = SOS_is_feasible(group, i, solution);
            i += 1;
        }
    } else {
        list = (**(*group)
            .sos_list
            .offset((sosindex - 1 as ::core::ffi::c_int) as isize))
        .members;
        n = *list.offset(0 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
        nn = *list.offset(n as isize);
        if nn <= 2 as ::core::ffi::c_int {
            return status;
        }
        i = 1 as ::core::ffi::c_int;
        sosindex = 0 as ::core::ffi::c_int;
        while i <= nn && *list.offset((n + i) as isize) != 0 as ::core::ffi::c_int {
            while i <= nn
                && *list.offset((n + i) as isize) != 0 as ::core::ffi::c_int
                && *solution.offset(((*lp).rows + *list.offset((n + i) as isize)) as isize)
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                i += 1;
            }
            if i <= nn && *list.offset((n + i) as isize) != 0 as ::core::ffi::c_int {
                i += 1;
                while i <= nn
                    && *list.offset((n + i) as isize) != 0 as ::core::ffi::c_int
                    && *solution.offset(((*lp).rows + *list.offset((n + i) as isize)) as isize)
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    i += 1;
                }
                sosindex += 1;
            }
            i += 1;
        }
        status =
            (sosindex <= 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    return status;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAXINT32: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ISSOS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ISSOSTEMPINT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ISGUB: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const RESIZEFACTOR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SOS3: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SOSn: ::core::ffi::c_int = MAXINT32;
pub const SOS_START_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SOS3_INCOMPLETE: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const SOS_INCOMPLETE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SOS_COMPLETE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SOS_INFEASIBLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOS_INTERNALERROR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
