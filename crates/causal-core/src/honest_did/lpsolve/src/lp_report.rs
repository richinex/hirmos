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
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn fflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_allocREAL"]
    fn allocREAL(
        lp: *mut lprec,
        ptr: *mut *mut ::core::ffi::c_double,
        size: ::core::ffi::c_int,
        clear: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mempool_obtainVector"]
    fn mempool_obtainVector(
        mempool: *mut workarraysrec,
        count: ::core::ffi::c_int,
        unitsize: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name="honest_lpsolve_mempool_releaseVector"]
    fn mempool_releaseVector(
        mempool: *mut workarraysrec,
        memvector: *mut ::core::ffi::c_char,
        forcefree: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_validate"]
    fn mat_validate(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_colIndexA"]
    fn get_colIndexA(
        lp: *mut lprec,
        varset: ::core::ffi::c_int,
        colindex: *mut ::core::ffi::c_int,
        append: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_prod_xA"]
    fn prod_xA(
        lp: *mut lprec,
        coltarget: *mut ::core::ffi::c_int,
        input: *mut ::core::ffi::c_double,
        nzinput: *mut ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
        ofscalar: ::core::ffi::c_double,
        output: *mut ::core::ffi::c_double,
        nzoutput: *mut ::core::ffi::c_int,
        roundmode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bsolve"]
    fn bsolve(
        lp: *mut lprec,
        row_nr: ::core::ffi::c_int,
        rhsvector: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
        ofscalar: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_lp_name"]
    fn get_lp_name(lp: *mut lprec) -> *mut ::core::ffi::c_char;
    #[link_name="honest_lpsolve_has_BFP"]
    fn has_BFP(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_maxim"]
    fn is_maxim(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_constr_type"]
    fn is_constr_type(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        mask: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_constr_class"]
    fn get_constr_class(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_get_rh"]
    fn get_rh(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_mat"]
    fn get_mat(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_upbo"]
    fn get_upbo(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_lowbo"]
    fn get_lowbo(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_is_int"]
    fn is_int(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_row_name"]
    fn get_row_name(lp: *mut lprec, rownr: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name="honest_lpsolve_get_col_name"]
    fn get_col_name(lp: *mut lprec, colnr: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name="honest_lpsolve_get_origcol_name"]
    fn get_origcol_name(lp: *mut lprec, colnr: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name="honest_lpsolve_get_total_iter"]
    fn get_total_iter(lp: *mut lprec) -> ::core::ffi::c_longlong;
    #[link_name="honest_lpsolve_get_var_primalresult"]
    fn get_var_primalresult(lp: *mut lprec, index: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_ptr_sensitivity_rhs"]
    fn get_ptr_sensitivity_rhs(
        lp: *mut lprec,
        duals: *mut *mut ::core::ffi::c_double,
        dualsfrom: *mut *mut ::core::ffi::c_double,
        dualstill: *mut *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_ptr_sensitivity_obj"]
    fn get_ptr_sensitivity_obj(
        lp: *mut lprec,
        objfrom: *mut *mut ::core::ffi::c_double,
        objtill: *mut *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_ptr_sensitivity_objex"]
    fn get_ptr_sensitivity_objex(
        lp: *mut lprec,
        objfrom: *mut *mut ::core::ffi::c_double,
        objtill: *mut *mut ::core::ffi::c_double,
        objfromvalue: *mut *mut ::core::ffi::c_double,
        objtillvalue: *mut *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_chsign"]
    fn is_chsign(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_rh_upper"]
    fn get_rh_upper(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_rh_lower"]
    fn get_rh_lower(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_SOS_count"]
    fn SOS_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_GUB_count"]
    fn GUB_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_splitvar"]
    fn is_splitvar(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_modifyOF1"]
    fn modifyOF1(
        lp: *mut lprec,
        index: ::core::ffi::c_int,
        ofValue: *mut ::core::ffi::c_double,
        mult: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
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
#[export_name="honest_lpsolve_explain"]
pub unsafe extern "C" fn explain(
    mut lp: *mut lprec,
    mut format: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return format;
}
#[export_name="honest_lpsolve_report"]
pub unsafe extern "C" fn report(
    mut lp: *mut lprec,
    mut level: ::core::ffi::c_int,
    mut format: *mut ::core::ffi::c_char,
) {
}
#[export_name="honest_lpsolve_print_indent"]
pub unsafe extern "C" fn print_indent(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    report(
        lp,
        0 as ::core::ffi::c_int,
        b"%2d\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if (*lp).bb_level < 50 as ::core::ffi::c_int {
        i = (*lp).bb_level;
        while i > 0 as ::core::ffi::c_int {
            report(
                lp,
                0 as ::core::ffi::c_int,
                b"--\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            i -= 1;
        }
    } else {
        report(
            lp,
            0 as ::core::ffi::c_int,
            b" *** too deep ***\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    report(
        lp,
        0 as ::core::ffi::c_int,
        b"> \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
}
#[export_name="honest_lpsolve_debug_print"]
pub unsafe extern "C" fn debug_print(mut lp: *mut lprec, mut format: *mut ::core::ffi::c_char) {}
#[export_name="honest_lpsolve_debug_print_solution"]
pub unsafe extern "C" fn debug_print_solution(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).bb_trace != 0 {
        i = (*lp).rows + 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            print_indent(lp);
            report(
                lp,
                0 as ::core::ffi::c_int,
                b"%s %18.12g\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            i += 1;
        }
    }
}
#[export_name="honest_lpsolve_debug_print_bounds"]
pub unsafe extern "C" fn debug_print_bounds(
    mut lp: *mut lprec,
    mut upbo: *mut ::core::ffi::c_double,
    mut lowbo: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).bb_trace != 0 {
        i = (*lp).rows + 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            if *lowbo.offset(i as isize) == *upbo.offset(i as isize) {
                print_indent(lp);
                report(
                    lp,
                    0 as ::core::ffi::c_int,
                    b"%s = %18.12g\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                if *lowbo.offset(i as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    print_indent(lp);
                    report(
                        lp,
                        0 as ::core::ffi::c_int,
                        b"%s > %18.12g\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                if *upbo.offset(i as isize) != (*lp).infinite {
                    print_indent(lp);
                    report(
                        lp,
                        0 as ::core::ffi::c_int,
                        b"%s < %18.12g\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            i += 1;
        }
    }
}
#[export_name="honest_lpsolve_blockWriteLREAL"]
pub unsafe extern "C" fn blockWriteLREAL(
    mut output: *mut FILE,
    mut label: *mut ::core::ffi::c_char,
    mut vector: *mut ::core::ffi::c_double,
    mut first: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    native_only!(fprintf,
        output,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        label,
    );
    native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = first;
    while i <= last {
        native_only!(fprintf,
            output,
            b" %18g\0" as *const u8 as *const ::core::ffi::c_char,
            *vector.offset(i as isize),
        );
        k += 1;
        if k % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            k = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if k % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[export_name="honest_lpsolve_blockWriteAMAT"]
pub unsafe extern "C" fn blockWriteAMAT(
    mut output: *mut FILE,
    mut label: *const ::core::ffi::c_char,
    mut lp: *mut lprec,
    mut first: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nzb: ::core::ffi::c_int = 0;
    let mut nze: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    if mat_validate(mat) == 0 {
        return;
    }
    if first < 0 as ::core::ffi::c_int {
        first = 0 as ::core::ffi::c_int;
    }
    if last < 0 as ::core::ffi::c_int {
        last = (*lp).rows;
    }
    native_only!(fprintf,
        output,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        label,
    );
    native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    if first == 0 as ::core::ffi::c_int {
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            hold = get_mat(lp, 0 as ::core::ffi::c_int, j);
            native_only!(fprintf,
                output,
                b" %18g\0" as *const u8 as *const ::core::ffi::c_char,
                hold,
            );
            k += 1;
            if k % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                k = 0 as ::core::ffi::c_int;
            }
            j += 1;
        }
        if k % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            k = 0 as ::core::ffi::c_int;
        }
        first += 1;
    }
    nze = *(*mat)
        .row_end
        .offset((first - 1 as ::core::ffi::c_int) as isize);
    i = first;
    while i <= last {
        nzb = nze;
        nze = *(*mat).row_end.offset(i as isize);
        if nzb >= nze {
            jb = (*lp).columns + 1 as ::core::ffi::c_int;
        } else {
            jb = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(nzb as isize) as isize);
        }
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            if j < jb {
                hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                hold = get_mat(lp, i, j);
                nzb += 1;
                if nzb < nze {
                    jb = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(nzb as isize) as isize);
                } else {
                    jb = (*lp).columns + 1 as ::core::ffi::c_int;
                }
            }
            native_only!(fprintf,
                output,
                b" %18g\0" as *const u8 as *const ::core::ffi::c_char,
                hold,
            );
            k += 1;
            if k % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                k = 0 as ::core::ffi::c_int;
            }
            j += 1;
        }
        if k % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            k = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if k % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[export_name="honest_lpsolve_blockWriteBMAT"]
pub unsafe extern "C" fn blockWriteBMAT(
    mut output: *mut FILE,
    mut label: *const ::core::ffi::c_char,
    mut lp: *mut lprec,
    mut first: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hold: ::core::ffi::c_double = 0.;
    if first < 0 as ::core::ffi::c_int {
        first = 0 as ::core::ffi::c_int;
    }
    if last < 0 as ::core::ffi::c_int {
        last = (*lp).rows;
    }
    native_only!(fprintf,
        output,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        label,
    );
    native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = first;
    while i <= last {
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).rows {
            jb = *(*lp).var_basic.offset(j as isize);
            if jb <= (*lp).rows {
                if jb == i {
                    hold = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                hold = get_mat(lp, i, j);
            }
            if i == 0 as ::core::ffi::c_int {
                modifyOF1(
                    lp,
                    jb,
                    &raw mut hold,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                );
            }
            hold = unscaled_mat(lp, hold, i, jb);
            native_only!(fprintf,
                output,
                b" %18g\0" as *const u8 as *const ::core::ffi::c_char,
                hold,
            );
            k += 1;
            if k % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                k = 0 as ::core::ffi::c_int;
            }
            j += 1;
        }
        if k % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            k = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if k % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        native_only!(fprintf,output, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[export_name="honest_lpsolve_REPORT_objective"]
pub unsafe extern "C" fn REPORT_objective(mut lp: *mut lprec) {
    if (*lp).outstream.is_null() {
        return;
    }
    if fabs(*(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize)) < 1e-5f64 {
        native_only!(fprintf,
            (*lp).outstream,
            b"\nValue of objective function: %g\n\0" as *const u8 as *const ::core::ffi::c_char,
            *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize),
        );
    } else {
        native_only!(fprintf,
            (*lp).outstream,
            b"\nValue of objective function: %.8f\n\0" as *const u8 as *const ::core::ffi::c_char,
            *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    native_only!(fflush,(*lp).outstream);
}
#[export_name="honest_lpsolve_REPORT_solution"]
pub unsafe extern "C" fn REPORT_solution(mut lp: *mut lprec, mut columns: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0.;
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    let mut NZonly: ::core::ffi::c_uchar = ((*lp).print_sol & AUTOMATIC > 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if (*lp).outstream.is_null() {
        return;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"\nActual values of the variables:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if columns <= 0 as ::core::ffi::c_int {
        columns = 2 as ::core::ffi::c_int;
    }
    n = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*psundo).orig_columns {
        j = (*psundo).orig_rows + i;
        value = get_var_primalresult(lp, j);
        if !(NZonly as ::core::ffi::c_int != 0 && fabs(value) < (*lp).epsprimal) {
            n = (n + 1 as ::core::ffi::c_int) % columns;
            native_only!(fprintf,
                (*lp).outstream,
                b"%-20s %12g\0" as *const u8 as *const ::core::ffi::c_char,
                get_origcol_name(lp, i),
                value,
            );
            if n == 0 as ::core::ffi::c_int {
                native_only!(fprintf,
                    (*lp).outstream,
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                native_only!(fprintf,
                    (*lp).outstream,
                    b"       \0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    native_only!(fflush,(*lp).outstream);
}
#[export_name="honest_lpsolve_REPORT_constraints"]
pub unsafe extern "C" fn REPORT_constraints(mut lp: *mut lprec, mut columns: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0.;
    let mut NZonly: ::core::ffi::c_uchar = ((*lp).print_sol & AUTOMATIC > 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if (*lp).outstream.is_null() {
        return;
    }
    if columns <= 0 as ::core::ffi::c_int {
        columns = 2 as ::core::ffi::c_int;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"\nActual values of the constraints:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    n = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        value = *(*lp).best_solution.offset(i as isize);
        if !(NZonly as ::core::ffi::c_int != 0 && fabs(value) < (*lp).epsprimal) {
            n = (n + 1 as ::core::ffi::c_int) % columns;
            native_only!(fprintf,
                (*lp).outstream,
                b"%-20s %12g\0" as *const u8 as *const ::core::ffi::c_char,
                get_row_name(lp, i),
                value,
            );
            if n == 0 as ::core::ffi::c_int {
                native_only!(fprintf,
                    (*lp).outstream,
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                native_only!(fprintf,
                    (*lp).outstream,
                    b"       \0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    native_only!(fflush,(*lp).outstream);
}
#[export_name="honest_lpsolve_REPORT_duals"]
pub unsafe extern "C" fn REPORT_duals(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut duals: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut dualsfrom: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut dualstill: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objfrom: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objtill: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objfromvalue: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut ret: ::core::ffi::c_uchar = 0;
    if (*lp).outstream.is_null() {
        return;
    }
    ret = get_ptr_sensitivity_objex(
        lp,
        &raw mut objfrom,
        &raw mut objtill,
        &raw mut objfromvalue,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
    );
    if ret != 0 {
        native_only!(fprintf,
            (*lp).outstream,
            b"\nObjective function limits:\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        native_only!(fprintf,
            (*lp).outstream,
            b"                                 From            Till       FromValue\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).columns {
            if is_splitvar(lp, i) == 0 {
                native_only!(fprintf,
                    (*lp).outstream,
                    b"%-20s  %15.7g %15.7g %15.7g\n\0" as *const u8 as *const ::core::ffi::c_char,
                    get_col_name(lp, i),
                    *objfrom.offset((i - 1 as ::core::ffi::c_int) as isize),
                    *objtill.offset((i - 1 as ::core::ffi::c_int) as isize),
                    *objfromvalue.offset((i - 1 as ::core::ffi::c_int) as isize),
                );
            }
            i += 1;
        }
    }
    ret = get_ptr_sensitivity_rhs(lp, &raw mut duals, &raw mut dualsfrom, &raw mut dualstill);
    if ret != 0 {
        native_only!(fprintf,
            (*lp).outstream,
            b"\nDual values with from - till limits:\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        native_only!(fprintf,
            (*lp).outstream,
            b"                           Dual value            From            Till\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            native_only!(fprintf,
                (*lp).outstream,
                b"%-20s  %15.7g %15.7g %15.7g\n\0" as *const u8 as *const ::core::ffi::c_char,
                if i <= (*lp).rows {
                    get_row_name(lp, i)
                } else {
                    get_col_name(lp, i - (*lp).rows)
                },
                *duals.offset((i - 1 as ::core::ffi::c_int) as isize),
                *dualsfrom.offset((i - 1 as ::core::ffi::c_int) as isize),
                *dualstill.offset((i - 1 as ::core::ffi::c_int) as isize),
            );
            i += 1;
        }
        native_only!(fflush,(*lp).outstream);
    }
}
#[export_name="honest_lpsolve_REPORT_extended"]
pub unsafe extern "C" fn REPORT_extended(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_double = 0.;
    let mut duals: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut dualsfrom: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut dualstill: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objfrom: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objtill: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut ret: ::core::ffi::c_uchar = 0;
    ret = get_ptr_sensitivity_obj(lp, &raw mut objfrom, &raw mut objtill);
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"Primal objective:\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"  Column name                      Value   Objective         Min         Max\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"  --------------------------------------------------------------------------\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        hold = get_mat(lp, 0 as ::core::ffi::c_int, j);
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"  %-25s %12g%12g%12g%12g\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        j += 1;
    }
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    ret = get_ptr_sensitivity_rhs(lp, &raw mut duals, &raw mut dualsfrom, &raw mut dualstill);
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"Primal variables:\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"  Column name                      Value       Slack         Min         Max\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"  --------------------------------------------------------------------------\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"  %-25s %12g%12g%12g%12g\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        j += 1;
    }
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"Dual variables:\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"  Row name                         Value       Slack         Min         Max\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"  --------------------------------------------------------------------------\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"  %-25s %12g%12g%12g%12g\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
}
#[export_name="honest_lpsolve_REPORT_lp"]
pub unsafe extern "C" fn REPORT_lp(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if (*lp).outstream.is_null() {
        return;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"Model name: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        get_lp_name(lp),
    );
    native_only!(fprintf,
        (*lp).outstream,
        b"          \0" as *const u8 as *const ::core::ffi::c_char,
    );
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        native_only!(fprintf,
            (*lp).outstream,
            b"%8s \0" as *const u8 as *const ::core::ffi::c_char,
            get_col_name(lp, j),
        );
        j += 1;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"\n%simize  \0" as *const u8 as *const ::core::ffi::c_char,
        if is_maxim(lp) as ::core::ffi::c_int != 0 {
            b"Max\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"Min\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        native_only!(fprintf,
            (*lp).outstream,
            b"%8g \0" as *const u8 as *const ::core::ffi::c_char,
            get_mat(lp, 0 as ::core::ffi::c_int, j),
        );
        j += 1;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        native_only!(fprintf,
            (*lp).outstream,
            b"%-9s \0" as *const u8 as *const ::core::ffi::c_char,
            get_row_name(lp, i),
        );
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            native_only!(fprintf,
                (*lp).outstream,
                b"%8g \0" as *const u8 as *const ::core::ffi::c_char,
                get_mat(lp, i, j),
            );
            j += 1;
        }
        if is_constr_type(lp, i, GE) != 0 {
            native_only!(fprintf,
                (*lp).outstream,
                b">= \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if is_constr_type(lp, i, LE) != 0 {
            native_only!(fprintf,
                (*lp).outstream,
                b"<= \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            native_only!(fprintf,
                (*lp).outstream,
                b" = \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        native_only!(fprintf,
            (*lp).outstream,
            b"%8g\0" as *const u8 as *const ::core::ffi::c_char,
            get_rh(lp, i),
        );
        if is_constr_type(lp, i, GE) != 0 {
            if get_rh_upper(lp, i) < (*lp).infinite {
                native_only!(fprintf,
                    (*lp).outstream,
                    b"  %s = %8g\0" as *const u8 as *const ::core::ffi::c_char,
                    b"upbo\0" as *const u8 as *const ::core::ffi::c_char,
                    get_rh_upper(lp, i),
                );
            }
        } else if is_constr_type(lp, i, LE) != 0 {
            if get_rh_lower(lp, i) > -(*lp).infinite {
                native_only!(fprintf,
                    (*lp).outstream,
                    b"  %s = %8g\0" as *const u8 as *const ::core::ffi::c_char,
                    b"lowbo\0" as *const u8 as *const ::core::ffi::c_char,
                    get_rh_lower(lp, i),
                );
            }
        }
        native_only!(fprintf,
            (*lp).outstream,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i += 1;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"Type      \0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if is_int(lp, i) != 0 {
            native_only!(fprintf,
                (*lp).outstream,
                b"     Int \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            native_only!(fprintf,
                (*lp).outstream,
                b"    Real \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        i += 1;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"\nupbo      \0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if get_upbo(lp, i) >= (*lp).infinite {
            native_only!(fprintf,
                (*lp).outstream,
                b"     Inf \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            native_only!(fprintf,
                (*lp).outstream,
                b"%8g \0" as *const u8 as *const ::core::ffi::c_char,
                get_upbo(lp, i),
            );
        }
        i += 1;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"\nlowbo     \0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if get_lowbo(lp, i) <= -(*lp).infinite {
            native_only!(fprintf,
                (*lp).outstream,
                b"    -Inf \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            native_only!(fprintf,
                (*lp).outstream,
                b"%8g \0" as *const u8 as *const ::core::ffi::c_char,
                get_lowbo(lp, i),
            );
        }
        i += 1;
    }
    native_only!(fprintf,
        (*lp).outstream,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    native_only!(fflush,(*lp).outstream);
}
#[export_name="honest_lpsolve_REPORT_scales"]
pub unsafe extern "C" fn REPORT_scales(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut colMax: ::core::ffi::c_int = 0;
    colMax = (*lp).columns;
    if (*lp).outstream.is_null() {
        return;
    }
    if (*lp).scaling_used != 0 {
        native_only!(fprintf,
            (*lp).outstream,
            b"\nScale factors:\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i <= (*lp).rows + colMax {
            native_only!(fprintf,
                (*lp).outstream,
                b"%-20s scaled at %g\n\0" as *const u8 as *const ::core::ffi::c_char,
                if i <= (*lp).rows {
                    get_row_name(lp, i)
                } else {
                    get_col_name(lp, i - (*lp).rows)
                },
                *(*lp).scalars.offset(i as isize),
            );
            i += 1;
        }
    }
    native_only!(fflush,(*lp).outstream);
}
#[export_name="honest_lpsolve_REPORT_tableau"]
pub unsafe extern "C" fn REPORT_tableau(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut j: ::core::ffi::c_int = 0;
    let mut row_nr: ::core::ffi::c_int = 0;
    let mut coltarget: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut prow: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut stream: *mut FILE = (*lp).outstream;
    if (*lp).outstream.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    if (*lp).model_is_valid == 0
        || has_BFP(lp) == 0
        || get_total_iter(lp) == 0 as ::core::ffi::c_longlong
        || (*lp).spx_status == NOTRUN
    {
        (*lp).spx_status = NOTRUN;
        return 0 as ::core::ffi::c_uchar;
    }
    if allocREAL(
        lp,
        &raw mut prow,
        (*lp).sum + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
    {
        (*lp).spx_status = NOMEMORY;
        return 0 as ::core::ffi::c_uchar;
    }
    native_only!(fprintf,stream, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    native_only!(fprintf,
        stream,
        b"Tableau at iter %.0f:\n\0" as *const u8 as *const ::core::ffi::c_char,
        get_total_iter(lp) as ::core::ffi::c_double,
    );
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).sum {
        if *(*lp).is_basic.offset(j as isize) == 0 {
            native_only!(fprintf,
                stream,
                b"%15d\0" as *const u8 as *const ::core::ffi::c_char,
                (if j <= (*lp).rows {
                    (j + (*lp).columns)
                        * (if *(*lp).orig_upbo.offset(j as isize)
                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            || is_chsign(lp, j) as ::core::ffi::c_int != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        })
                } else {
                    j - (*lp).rows
                }) * (if *(*lp).is_lower.offset(j as isize) as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                }),
            );
        }
        j += 1;
    }
    native_only!(fprintf,stream, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    coltarget = mempool_obtainVector(
        (*lp).workarrays,
        (*lp).columns + 1 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
    ) as *mut ::core::ffi::c_int;
    if get_colIndexA(
        lp,
        SCAN_USERVARS + USE_NONBASICVARS,
        coltarget,
        FALSE as ::core::ffi::c_uchar,
    ) == 0
    {
        mempool_releaseVector(
            (*lp).workarrays,
            coltarget as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    row_nr = 1 as ::core::ffi::c_int;
    while row_nr <= (*lp).rows + 1 as ::core::ffi::c_int {
        if row_nr <= (*lp).rows {
            native_only!(fprintf,
                stream,
                b"%3d\0" as *const u8 as *const ::core::ffi::c_char,
                (if *(*lp).var_basic.offset(row_nr as isize) <= (*lp).rows {
                    (*(*lp).var_basic.offset(row_nr as isize) + (*lp).columns)
                        * (if *(*lp)
                            .orig_upbo
                            .offset(*(*lp).var_basic.offset(row_nr as isize) as isize)
                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            || is_chsign(lp, *(*lp).var_basic.offset(row_nr as isize))
                                as ::core::ffi::c_int
                                != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        })
                } else {
                    *(*lp).var_basic.offset(row_nr as isize) - (*lp).rows
                }) * (if *(*lp)
                    .is_lower
                    .offset(*(*lp).var_basic.offset(row_nr as isize) as isize)
                    as ::core::ffi::c_int
                    != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                }),
            );
        } else {
            native_only!(fprintf,stream, b"   \0" as *const u8 as *const ::core::ffi::c_char);
        }
        bsolve(
            lp,
            if row_nr <= (*lp).rows {
                row_nr
            } else {
                0 as ::core::ffi::c_int
            },
            prow,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            (*lp).epsmachine * DOUBLEROUND,
            1.0f64,
        );
        prod_xA(
            lp,
            coltarget,
            prow,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            (*lp).epsmachine,
            1.0f64,
            prow,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            MAT_ROUNDDEFAULT,
        );
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).rows + (*lp).columns {
            if *(*lp).is_basic.offset(j as isize) == 0 {
                native_only!(fprintf,
                    stream,
                    b"%15.7f\0" as *const u8 as *const ::core::ffi::c_char,
                    *prow.offset(j as isize)
                        * (if *(*lp).is_lower.offset(j as isize) as ::core::ffi::c_int != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }) as ::core::ffi::c_double
                        * (if row_nr <= (*lp).rows {
                            1 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }) as ::core::ffi::c_double,
                );
            }
            j += 1;
        }
        native_only!(fprintf,
            stream,
            b"%15.7f\0" as *const u8 as *const ::core::ffi::c_char,
            *(*lp).rhs.offset(
                (if row_nr <= (*lp).rows {
                    row_nr
                } else {
                    0 as ::core::ffi::c_int
                }) as isize,
            ) * (if row_nr <= (*lp).rows || is_maxim(lp) as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }) as ::core::ffi::c_double,
        );
        native_only!(fprintf,stream, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        row_nr += 1;
    }
    native_only!(fflush,stream);
    mempool_releaseVector(
        (*lp).workarrays,
        coltarget as *mut ::core::ffi::c_char,
        FALSE as ::core::ffi::c_uchar,
    );
    if !(prow as *mut ::core::ffi::c_void).is_null() {
        free(prow as *mut ::core::ffi::c_void);
        prow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_REPORT_constraintinfo"]
pub unsafe extern "C" fn REPORT_constraintinfo(
    mut lp: *mut lprec,
    mut datainfo: *mut ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut tally: [::core::ffi::c_int; 11] = [0; 11];
    memset(
        &raw mut tally as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        '\0' as i32,
        ((10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        tally[get_constr_class(lp, i) as usize] += 1;
        i += 1;
    }
    if !datainfo.is_null() {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i <= ROWCLASS_MAX {
        if tally[i as usize] > 0 as ::core::ffi::c_int {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"%-15s %4d\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        i += 1;
    }
}
#[export_name="honest_lpsolve_REPORT_modelinfo"]
pub unsafe extern "C" fn REPORT_modelinfo(
    mut lp: *mut lprec,
    mut doName: ::core::ffi::c_uchar,
    mut datainfo: *mut ::core::ffi::c_char,
) {
    if doName != 0 {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"\nModel name:  '%s' - run #%-5d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"Objective:   %simize(%s)\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if !datainfo.is_null() {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"Model size:  %7d constraints, %7d variables, %12d non-zeros.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if GUB_count(lp) + SOS_count(lp) > 0 as ::core::ffi::c_int {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"Var-types:   %7d integer,     %7d semi-cont.,     %7d SOS.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    report(
        lp,
        4 as ::core::ffi::c_int,
        b"Sets:                             %7d GUB,            %7d SOS.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ROWTYPE_LE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ROWTYPE_GE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LE: ::core::ffi::c_int = ROWTYPE_LE;
pub const GE: ::core::ffi::c_int = ROWTYPE_GE;
pub const ROWCLASS_GUB: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const ROWCLASS_MAX: ::core::ffi::c_int = ROWCLASS_GUB;
pub const SCAN_USERVARS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const USE_NONBASICVARS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NOMEMORY: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const NOTRUN: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const DOUBLEROUND: ::core::ffi::c_double = 0.0e-02f64;
pub const MAT_ROUNDREL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAT_ROUNDDEFAULT: ::core::ffi::c_int = MAT_ROUNDREL;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
