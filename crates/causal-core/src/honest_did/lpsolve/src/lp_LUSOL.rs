use crate::honest_did::lpsolve::runtime::{strlen,strcpy};
use crate::honest_did::lpsolve::runtime::{malloc,calloc,free,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
    #[link_name="honest_lpsolve_createLink"]
    fn createLink(
        size: ::core::ffi::c_int,
        linkmap: *mut *mut LLrec,
        usedpos: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_firstActiveLink"]
    fn firstActiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_nextActiveLink"]
    fn nextActiveLink(linkmap: *mut LLrec, backitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_removeLink"]
    fn removeLink(linkmap: *mut LLrec, itemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_sortByINT"]
    fn sortByINT(
        item: *mut ::core::ffi::c_int,
        weight: *mut ::core::ffi::c_int,
        size: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        unique: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_timeNow"]
    fn timeNow() -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_LUSOL_create"]
    fn LUSOL_create(
        outstream: *mut FILE,
        msgfil: ::core::ffi::c_int,
        pivotmodel: ::core::ffi::c_int,
        updatelimit: ::core::ffi::c_int,
    ) -> *mut LUSOLrec;
    #[link_name="honest_lpsolve_LUSOL_sizeto"]
    fn LUSOL_sizeto(
        LUSOL: *mut LUSOLrec,
        init_r: ::core::ffi::c_int,
        init_c: ::core::ffi::c_int,
        init_a: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_LUSOL_clear"]
    fn LUSOL_clear(LUSOL: *mut LUSOLrec, nzonly: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_LUSOL_free"]
    fn LUSOL_free(LUSOL: *mut LUSOLrec);
    #[link_name="honest_lpsolve_LUSOL_loadColumn"]
    fn LUSOL_loadColumn(
        LUSOL: *mut LUSOLrec,
        iA: *mut ::core::ffi::c_int,
        jA: ::core::ffi::c_int,
        Aij: *mut ::core::ffi::c_double,
        nzcount: ::core::ffi::c_int,
        offset1: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_LUSOL_setpivotmodel"]
    fn LUSOL_setpivotmodel(
        LUSOL: *mut LUSOLrec,
        pivotmodel: ::core::ffi::c_int,
        initlevel: ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_LUSOL_factorize"]
    fn LUSOL_factorize(LUSOL: *mut LUSOLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_LUSOL_replaceColumn"]
    fn LUSOL_replaceColumn(
        LUSOL: *mut LUSOLrec,
        jcol: ::core::ffi::c_int,
        v: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_LUSOL_tightenpivot"]
    fn LUSOL_tightenpivot(LUSOL: *mut LUSOLrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_LUSOL_getSingularity"]
    fn LUSOL_getSingularity(
        LUSOL: *mut LUSOLrec,
        singitem: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_LUSOL_ftran"]
    fn LUSOL_ftran(
        LUSOL: *mut LUSOLrec,
        b: *mut ::core::ffi::c_double,
        NZidx: *mut ::core::ffi::c_int,
        prepareupdate: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_LUSOL_btran"]
    fn LUSOL_btran(
        LUSOL: *mut LUSOLrec,
        b: *mut ::core::ffi::c_double,
        NZidx: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_LU8RPC"]
    fn LU8RPC(
        LUSOL: *mut LUSOLrec,
        MODE1: ::core::ffi::c_int,
        MODE2: ::core::ffi::c_int,
        JREP: ::core::ffi::c_int,
        V: *mut ::core::ffi::c_double,
        W: *mut ::core::ffi::c_double,
        INFORM: *mut ::core::ffi::c_int,
        DIAG: *mut ::core::ffi::c_double,
        VNORM: *mut ::core::ffi::c_double,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _INVrec {
    pub status: ::core::ffi::c_int,
    pub dimcount: ::core::ffi::c_int,
    pub dimalloc: ::core::ffi::c_int,
    pub user_colcount: ::core::ffi::c_int,
    pub LUSOL: *mut LUSOLrec,
    pub col_enter: ::core::ffi::c_int,
    pub col_leave: ::core::ffi::c_int,
    pub col_pos: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_double,
    pub pcol: *mut ::core::ffi::c_double,
    pub theta_enter: ::core::ffi::c_double,
    pub max_Bsize: ::core::ffi::c_int,
    pub max_colcount: ::core::ffi::c_int,
    pub max_LUsize: ::core::ffi::c_int,
    pub num_refact: ::core::ffi::c_int,
    pub num_timed_refact: ::core::ffi::c_int,
    pub num_dense_refact: ::core::ffi::c_int,
    pub time_refactstart: ::core::ffi::c_double,
    pub time_refactnext: ::core::ffi::c_double,
    pub num_pivots: ::core::ffi::c_int,
    pub num_singular: ::core::ffi::c_int,
    pub opts: *mut ::core::ffi::c_char,
    pub is_dirty: ::core::ffi::c_uchar,
    pub force_refact: ::core::ffi::c_uchar,
    pub timed_refact: ::core::ffi::c_uchar,
    pub set_Bidentity: ::core::ffi::c_uchar,
}
pub type LUSOLrec = _LUSOLrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LUSOLrec {
    pub outstream: *mut FILE,
    pub writelog: Option<LUSOLlogfunc>,
    pub loghandle: *mut ::core::ffi::c_void,
    pub debuginfo: Option<LUSOLlogfunc>,
    pub luparm: [::core::ffi::c_int; 33],
    pub parmlu: [::core::ffi::c_double; 21],
    pub lena: ::core::ffi::c_int,
    pub nelem: ::core::ffi::c_int,
    pub indc: *mut ::core::ffi::c_int,
    pub indr: *mut ::core::ffi::c_int,
    pub a: *mut ::core::ffi::c_double,
    pub maxm: ::core::ffi::c_int,
    pub m: ::core::ffi::c_int,
    pub lenr: *mut ::core::ffi::c_int,
    pub ip: *mut ::core::ffi::c_int,
    pub iqloc: *mut ::core::ffi::c_int,
    pub ipinv: *mut ::core::ffi::c_int,
    pub locr: *mut ::core::ffi::c_int,
    pub maxn: ::core::ffi::c_int,
    pub n: ::core::ffi::c_int,
    pub lenc: *mut ::core::ffi::c_int,
    pub iq: *mut ::core::ffi::c_int,
    pub iploc: *mut ::core::ffi::c_int,
    pub iqinv: *mut ::core::ffi::c_int,
    pub locc: *mut ::core::ffi::c_int,
    pub w: *mut ::core::ffi::c_double,
    pub vLU6L: *mut ::core::ffi::c_double,
    pub isingular: *mut ::core::ffi::c_int,
    pub Ha: *mut ::core::ffi::c_double,
    pub diagU: *mut ::core::ffi::c_double,
    pub Hj: *mut ::core::ffi::c_int,
    pub Hk: *mut ::core::ffi::c_int,
    pub amaxr: *mut ::core::ffi::c_double,
    pub L0: *mut LUSOLmat,
    pub U: *mut LUSOLmat,
    pub expanded_a: ::core::ffi::c_int,
    pub replaced_c: ::core::ffi::c_int,
    pub replaced_r: ::core::ffi::c_int,
}
pub type LUSOLmat = _LUSOLmat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LUSOLmat {
    pub a: *mut ::core::ffi::c_double,
    pub lenx: *mut ::core::ffi::c_int,
    pub indr: *mut ::core::ffi::c_int,
    pub indc: *mut ::core::ffi::c_int,
    pub indx: *mut ::core::ffi::c_int,
}
pub type LUSOLlogfunc = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_char,
) -> ();
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
pub const MATINDEXBASE: ::core::ffi::c_int = LUSOL_ARRAYOFFSET;
pub const DEF_MAXPIVOT: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const MAX_DELTAFILLIN: ::core::ffi::c_double = 2.0f64;
pub const TIGHTENAFTER: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const BFP_STATUS_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BFP_STATUS_ERROR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const BFP_STAT_ERROR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const BFP_STAT_REFACT_TOTAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BFP_STAT_REFACT_TIMED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BFP_STAT_REFACT_DENSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[export_name="honest_lpsolve_bfp_compatible"]
pub unsafe extern "C" fn bfp_compatible(
    mut lp: *mut lprec,
    mut bfpversion: ::core::ffi::c_int,
    mut lpversion: ::core::ffi::c_int,
    mut sizeofvar: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if !lp.is_null()
        && bfpversion == BFPVERSION
        && ::core::mem::size_of::<::core::ffi::c_double>() as usize == sizeofvar as usize
    {
        status = TRUE as ::core::ffi::c_uchar;
    }
    return status;
}
#[export_name="honest_lpsolve_bfp_status"]
pub unsafe extern "C" fn bfp_status(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*(*lp).invB).status;
}
#[export_name="honest_lpsolve_bfp_indexbase"]
pub unsafe extern "C" fn bfp_indexbase(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_bfp_rowoffset"]
pub unsafe extern "C" fn bfp_rowoffset(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).obj_in_basis != 0 {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_bfp_pivotmax"]
pub unsafe extern "C" fn bfp_pivotmax(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).max_pivots > 0 as ::core::ffi::c_int {
        return (*lp).max_pivots;
    } else {
        return 250 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_bfp_pivotvector"]
pub unsafe extern "C" fn bfp_pivotvector(mut lp: *mut lprec) -> *mut ::core::ffi::c_double {
    return (*(*lp).invB).pcol;
}
#[export_name="honest_lpsolve_bfp_efficiency"]
pub unsafe extern "C" fn bfp_efficiency(mut lp: *mut lprec) -> ::core::ffi::c_double {
    let mut hold: ::core::ffi::c_double = 0.;
    hold = (*lp).bfp_nonzeros.expect("non-null function pointer")(
        lp,
        AUTOMATIC as ::core::ffi::c_uchar,
    ) as ::core::ffi::c_double;
    if hold == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        hold = (1 as ::core::ffi::c_int + (*lp).rows) as ::core::ffi::c_double;
    }
    hold = (*lp).bfp_nonzeros.expect("non-null function pointer")(lp, TRUE as ::core::ffi::c_uchar)
        as ::core::ffi::c_double
        / hold;
    return hold;
}
#[export_name="honest_lpsolve_bfp_pivotcount"]
pub unsafe extern "C" fn bfp_pivotcount(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*(*lp).invB).num_pivots;
}
#[export_name="honest_lpsolve_bfp_refactcount"]
pub unsafe extern "C" fn bfp_refactcount(
    mut lp: *mut lprec,
    mut kind: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if kind == BFP_STAT_REFACT_TOTAL {
        return (*(*lp).invB).num_refact;
    } else if kind == BFP_STAT_REFACT_TIMED {
        return (*(*lp).invB).num_timed_refact;
    } else if kind == BFP_STAT_REFACT_DENSE {
        return (*(*lp).invB).num_dense_refact;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
#[export_name="honest_lpsolve_bfp_mustrefactorize"]
pub unsafe extern "C" fn bfp_mustrefactorize(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut test: ::core::ffi::c_uchar = (*lp).is_action.expect("non-null function pointer")(
        (*lp).spx_action,
        ACTION_REINVERT | ACTION_TIMEDREINVERT,
    );
    if test == 0 {
        let mut f: ::core::ffi::c_double = 0.;
        let mut lu: *mut INVrec = (*lp).invB;
        if (*lu).num_pivots > 0 as ::core::ffi::c_int {
            f = (timeNow() - (*lu).time_refactstart) / (*lu).num_pivots as ::core::ffi::c_double;
        } else {
            f = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if (*lu).force_refact as ::core::ffi::c_int != 0
            || (*lu).num_pivots >= (*lp).bfp_pivotmax.expect("non-null function pointer")(lp)
        {
            (*lp).set_action.expect("non-null function pointer")(
                &raw mut (*lp).spx_action,
                ACTION_REINVERT,
            );
        } else if (*lu).timed_refact as ::core::ffi::c_int != 0
            && (*lu).num_pivots > 1 as ::core::ffi::c_int
            && f > MIN_TIMEPIVOT
            && f > (*lu).time_refactnext
        {
            if (*lu).timed_refact as ::core::ffi::c_int == AUTOMATIC
                && ((*lu).num_pivots as ::core::ffi::c_double)
                    < 0.4f64
                        * (*lp).bfp_pivotmax.expect("non-null function pointer")(lp)
                            as ::core::ffi::c_double
            {
                (*lu).time_refactnext = f;
            } else {
                (*lp).set_action.expect("non-null function pointer")(
                    &raw mut (*lp).spx_action,
                    ACTION_TIMEDREINVERT,
                );
            }
        } else {
            (*lu).time_refactnext = f;
        }
    }
    test = (*lp).is_action.expect("non-null function pointer")(
        (*lp).spx_action,
        ACTION_REINVERT | ACTION_TIMEDREINVERT,
    );
    return test;
}
#[export_name="honest_lpsolve_bfp_isSetI"]
pub unsafe extern "C" fn bfp_isSetI(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*(*lp).invB).set_Bidentity;
}
#[export_name="honest_lpsolve_bfp_createMDO"]
pub unsafe extern "C" fn bfp_createMDO(
    mut lp: *mut lprec,
    mut usedpos: *mut ::core::ffi::c_uchar,
    mut count: ::core::ffi::c_int,
    mut doMDO: ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_int {
    let mut mdo: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut kk: ::core::ffi::c_int = 0;
    mdo = malloc(
        ((count + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    kk = 0 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        i = (*lp).rows + j;
        if *usedpos.offset(i as isize) as ::core::ffi::c_int == TRUE {
            kk += 1;
            *mdo.offset(kk as isize) = i;
        }
        j += 1;
    }
    *mdo.offset(0 as ::core::ffi::c_int as isize) = kk;
    if !(kk == 0 as ::core::ffi::c_int) {
        if doMDO != 0 {
            i = (*lp).getMDO.expect("non-null function pointer")(
                lp,
                usedpos,
                mdo,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                FALSE as ::core::ffi::c_uchar,
            );
            if i != 0 as ::core::ffi::c_int {
                (*lp).report.expect("non-null function pointer")(
                    lp,
                    1 as ::core::ffi::c_int,
                    b"bfp_createMDO: Internal error %d in minimum degree ordering routine\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                if !(mdo as *mut ::core::ffi::c_void).is_null() {
                    free(mdo as *mut ::core::ffi::c_void);
                    mdo = ::core::ptr::null_mut::<::core::ffi::c_int>();
                }
            }
        }
    }
    return mdo;
}
#[export_name="honest_lpsolve_bfp_updaterefactstats"]
pub unsafe extern "C" fn bfp_updaterefactstats(mut lp: *mut lprec) {
    let mut lu: *mut INVrec = (*lp).invB;
    (*lu).is_dirty = AUTOMATIC as ::core::ffi::c_uchar;
    (*lu).time_refactstart = timeNow();
    (*lu).time_refactnext = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*lu).user_colcount = 0 as ::core::ffi::c_int;
    if (*lu).force_refact != 0 {
        (*lu).num_dense_refact += 1;
    } else if (*lu).timed_refact as ::core::ffi::c_int != 0
        && (*lp).is_action.expect("non-null function pointer")(
            (*lp).spx_action,
            ACTION_TIMEDREINVERT,
        ) as ::core::ffi::c_int
            != 0
    {
        (*lu).num_timed_refact += 1;
    }
    (*lu).num_refact += 1;
}
#[export_name="honest_lpsolve_bfp_rowextra"]
pub unsafe extern "C" fn bfp_rowextra(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).is_obj_in_basis.expect("non-null function pointer")(lp) != 0 {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_bfp_init"]
pub unsafe extern "C" fn bfp_init(
    mut lp: *mut lprec,
    mut size: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut options: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    (*lp).invB = calloc(1 as size_t, ::core::mem::size_of::<INVrec>() as size_t) as *mut INVrec;
    lu = (*lp).invB;
    if lu.is_null()
        || (*lp).bfp_resize.expect("non-null function pointer")(lp, size) == 0
        || (*lp).bfp_restart.expect("non-null function pointer")(lp) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    if !options.is_null() {
        let mut len: size_t = strlen(options);
        (*lu).opts = malloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        strcpy((*lu).opts, options);
    }
    (*lp)
        .bfp_preparefactorization
        .expect("non-null function pointer")(lp);
    (*lu).num_refact = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bfp_restart"]
pub unsafe extern "C" fn bfp_restart(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    if lu.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    (*lu).status = BFP_STATUS_SUCCESS;
    (*lu).max_Bsize = 0 as ::core::ffi::c_int;
    (*lu).max_colcount = 0 as ::core::ffi::c_int;
    (*lu).max_LUsize = 0 as ::core::ffi::c_int;
    (*lu).num_refact = 0 as ::core::ffi::c_int;
    (*lu).num_timed_refact = 0 as ::core::ffi::c_int;
    (*lu).num_dense_refact = 0 as ::core::ffi::c_int;
    (*lu).num_pivots = 0 as ::core::ffi::c_int;
    (*lu).pcol = ::core::ptr::null_mut::<::core::ffi::c_double>();
    (*lu).set_Bidentity = FALSE as ::core::ffi::c_uchar;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bfp_implicitslack"]
pub unsafe extern "C" fn bfp_implicitslack(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bfp_colcount"]
pub unsafe extern "C" fn bfp_colcount(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*(*lp).invB).user_colcount;
}
#[export_name="honest_lpsolve_bfp_canresetbasis"]
pub unsafe extern "C" fn bfp_canresetbasis(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bfp_pivotalloc"]
pub unsafe extern "C" fn bfp_pivotalloc(
    mut lp: *mut lprec,
    mut newsize: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bfp_finishfactorization"]
pub unsafe extern "C" fn bfp_finishfactorization(mut lp: *mut lprec) {
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    if (*lu).max_colcount < (*lp).bfp_colcount.expect("non-null function pointer")(lp) {
        (*lu).max_colcount = (*lp).bfp_colcount.expect("non-null function pointer")(lp);
    }
    if (*lu).max_LUsize
        < (*lp).bfp_nonzeros.expect("non-null function pointer")(lp, 0 as ::core::ffi::c_uchar)
    {
        (*lu).max_LUsize =
            (*lp).bfp_nonzeros.expect("non-null function pointer")(lp, 0 as ::core::ffi::c_uchar);
    }
    (*lu).is_dirty = FALSE as ::core::ffi::c_uchar;
    (*lp).clear_action.expect("non-null function pointer")(
        &raw mut (*lp).spx_action,
        ACTION_REINVERT | ACTION_TIMEDREINVERT,
    );
    (*lu).force_refact = FALSE as ::core::ffi::c_uchar;
    (*lu).num_pivots = 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_bfp_prepareupdate"]
pub unsafe extern "C" fn bfp_prepareupdate(
    mut lp: *mut lprec,
    mut row_nr: ::core::ffi::c_int,
    mut col_nr: ::core::ffi::c_int,
    mut pcol: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut pivValue: ::core::ffi::c_double = 0.;
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    (*lu).col_enter = col_nr;
    (*lu).col_pos = row_nr;
    (*lu).col_leave = *(*lp).var_basic.offset(row_nr as isize);
    if pcol.is_null() {
        pivValue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        pivValue = *pcol.offset(row_nr as isize);
    }
    (*lu).theta_enter = pivValue;
    (*lu).pcol = pcol;
    if (*lu).is_dirty as ::core::ffi::c_int != AUTOMATIC {
        (*lu).is_dirty = TRUE as ::core::ffi::c_uchar;
    }
    return pivValue;
}
#[export_name="honest_lpsolve_bfp_pivotRHS"]
pub unsafe extern "C" fn bfp_pivotRHS(
    mut lp: *mut lprec,
    mut theta: ::core::ffi::c_double,
    mut pcol: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    if pcol.is_null() {
        pcol = (*lu).pcol;
    }
    if theta != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = (*lp).rows;
        let mut roundzero: ::core::ffi::c_double = (*lp).epsvalue;
        let mut rhs: *mut ::core::ffi::c_double = (*lp).rhs;
        let mut rhsmax: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i = 0 as ::core::ffi::c_int;
        while i <= n {
            *rhs -= theta * *pcol;
            if fabs(*rhs) < roundzero {
                *rhs = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if rhsmax < fabs(*rhs) {
                rhsmax = fabs(*rhs);
            }
            i += 1;
            rhs = rhs.offset(1);
            pcol = pcol.offset(1);
        }
        (*lp).rhsmax = rhsmax;
    }
    if pcol == (*lu).pcol {
        return (*lu).theta_enter;
    } else {
        return 0.0f64;
    };
}
#[export_name="honest_lpsolve_bfp_btran_double"]
pub unsafe extern "C" fn bfp_btran_double(
    mut lp: *mut lprec,
    mut prow: *mut ::core::ffi::c_double,
    mut pnzidx: *mut ::core::ffi::c_int,
    mut drow: *mut ::core::ffi::c_double,
    mut dnzidx: *mut ::core::ffi::c_int,
) {
    if !prow.is_null() {
        (*lp).bfp_btran_normal.expect("non-null function pointer")(lp, prow, pnzidx);
    }
    if !drow.is_null() {
        (*lp).bfp_btran_normal.expect("non-null function pointer")(lp, drow, dnzidx);
    }
}
#[export_name="honest_lpsolve_bfp_name"]
pub unsafe extern "C" fn bfp_name() -> *mut ::core::ffi::c_char {
    return b"LUSOL v2.2.1.0\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
}
#[export_name="honest_lpsolve_bfp_resize"]
pub unsafe extern "C" fn bfp_resize(
    mut lp: *mut lprec,
    mut newsize: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    newsize = newsize + bfp_rowoffset(lp);
    (*lu).dimalloc = newsize;
    if allocREAL(
        lp,
        &raw mut (*lu).value,
        newsize + MATINDEXBASE,
        AUTOMATIC as ::core::ffi::c_uchar,
    ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    if !(*lu).LUSOL.is_null() {
        if newsize > 0 as ::core::ffi::c_int || 1 as ::core::ffi::c_int != 0 {
            LUSOL_sizeto((*lu).LUSOL, newsize, newsize, 0 as ::core::ffi::c_int);
        } else {
            LUSOL_free((*lu).LUSOL);
            (*lu).LUSOL = ::core::ptr::null_mut::<LUSOLrec>();
        }
    } else if newsize > 0 as ::core::ffi::c_int || 1 as ::core::ffi::c_int != 0 {
        let mut asize: ::core::ffi::c_int = 0;
        let mut bsize: ::core::ffi::c_double = 0.;
        (*lu).LUSOL = LUSOL_create(
            ::core::ptr::null_mut::<FILE>(),
            0 as ::core::ffi::c_int,
            LUSOL_PIVMOD_TPP,
            bfp_pivotmax(lp) * 0 as ::core::ffi::c_int,
        );
        (*(*lu).LUSOL).luparm[LUSOL_IP_ACCELERATION as usize] = LUSOL_AUTOORDER;
        (*(*lu).LUSOL).parmlu[LUSOL_RP_SMARTRATIO as usize] = 0.50f64;
        (*lu).timed_refact = FALSE as ::core::ffi::c_uchar;
        LUSOL_setpivotmodel((*lu).LUSOL, LUSOL_PIVMOD_NOCHANGE, LUSOL_PIVTOL_SLIM);
        bsize = (*lp).get_nonzeros.expect("non-null function pointer")(lp) as ::core::ffi::c_double;
        if newsize > (*lp).columns {
            bsize += newsize as ::core::ffi::c_double;
        } else {
            bsize =
                bsize / (*lp).columns as ::core::ffi::c_double * newsize as ::core::ffi::c_double;
        }
        asize = (bsize * MAX_DELTAFILLIN * 1.3333f64) as ::core::ffi::c_int;
        if LUSOL_sizeto((*lu).LUSOL, newsize, newsize, asize) == 0 {
            return 0 as ::core::ffi::c_uchar;
        }
    }
    (*lu).dimcount = newsize;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bfp_free"]
pub unsafe extern "C" fn bfp_free(mut lp: *mut lprec) {
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    if lu.is_null() {
        return;
    }
    if !((*lu).opts as *mut ::core::ffi::c_void).is_null() {
        free((*lu).opts as *mut ::core::ffi::c_void);
        (*lu).opts = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !((*lu).value as *mut ::core::ffi::c_void).is_null() {
        free((*lu).value as *mut ::core::ffi::c_void);
        (*lu).value = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    LUSOL_free((*lu).LUSOL);
    if !(lu as *mut ::core::ffi::c_void).is_null() {
        free(lu as *mut ::core::ffi::c_void);
        lu = ::core::ptr::null_mut::<INVrec>();
    }
    (*lp).invB = ::core::ptr::null_mut::<INVrec>();
}
#[export_name="honest_lpsolve_bfp_nonzeros"]
pub unsafe extern "C" fn bfp_nonzeros(
    mut lp: *mut lprec,
    mut maximum: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    if maximum as ::core::ffi::c_int == TRUE {
        return (*lu).max_LUsize;
    } else if maximum as ::core::ffi::c_int == AUTOMATIC {
        return (*lu).max_Bsize;
    } else {
        return (*(*lu).LUSOL).luparm[LUSOL_IP_NONZEROS_L0 as usize]
            + (*(*lu).LUSOL).luparm[LUSOL_IP_NONZEROS_U0 as usize];
    };
}
#[export_name="honest_lpsolve_bfp_memallocated"]
pub unsafe extern "C" fn bfp_memallocated(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut mem: ::core::ffi::c_int = 0;
    let mut LUSOL: *mut LUSOLrec = (*(*lp).invB).LUSOL;
    mem = (::core::mem::size_of::<::core::ffi::c_double>() as usize)
        .wrapping_mul(((*LUSOL).lena + (*LUSOL).maxm + LUSOL_RP_LASTITEM) as usize)
        as ::core::ffi::c_int;
    mem = (mem as ::core::ffi::c_ulong).wrapping_add(
        (::core::mem::size_of::<::core::ffi::c_int>() as usize).wrapping_mul(
            (2 as ::core::ffi::c_int * (*LUSOL).lena
                + 5 as ::core::ffi::c_int * (*LUSOL).maxm
                + 5 as ::core::ffi::c_int * (*LUSOL).maxn
                + LUSOL_IP_LASTITEM) as usize,
        ) as ::core::ffi::c_ulong,
    ) as ::core::ffi::c_int as ::core::ffi::c_int;
    if (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] == LUSOL_PIVMOD_TCP {
        mem = (mem as ::core::ffi::c_ulong).wrapping_add(
            (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                .wrapping_mul((*LUSOL).maxn as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
                        .wrapping_mul((*LUSOL).maxn as usize),
                ) as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    } else if (*LUSOL).luparm[LUSOL_IP_PIVOTTYPE as usize] == LUSOL_PIVMOD_TRP {
        mem = (mem as ::core::ffi::c_ulong).wrapping_add(
            (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                .wrapping_mul((*LUSOL).maxn as usize) as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if (*LUSOL).luparm[LUSOL_IP_KEEPLU as usize] == 0 {
        mem = (mem as ::core::ffi::c_ulong).wrapping_add(
            (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                .wrapping_mul((*LUSOL).maxn as usize) as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    return mem;
}
#[export_name="honest_lpsolve_bfp_preparefactorization"]
pub unsafe extern "C" fn bfp_preparefactorization(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut lu: *mut INVrec = (*lp).invB;
    if (*lu).is_dirty as ::core::ffi::c_int == AUTOMATIC {
        (*lp)
            .bfp_finishfactorization
            .expect("non-null function pointer")(lp);
    }
    LUSOL_clear((*lu).LUSOL, TRUE as ::core::ffi::c_uchar);
    if (*lu).dimcount != (*lp).rows + bfp_rowoffset(lp) {
        (*lp).bfp_resize.expect("non-null function pointer")(lp, (*lp).rows);
    }
    (*lp)
        .bfp_updaterefactstats
        .expect("non-null function pointer")(lp);
    (*lu).col_pos = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_bfp_LUSOLsetcolumn"]
pub unsafe extern "C" fn bfp_LUSOLsetcolumn(
    mut lp: *mut lprec,
    mut posnr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut inform: ::core::ffi::c_int = 0;
    inform = LUSOL_replaceColumn(
        (*(*lp).invB).LUSOL,
        posnr,
        (*(*(*lp).invB).LUSOL).w as *mut ::core::ffi::c_double,
    );
    return inform;
}
#[export_name="honest_lpsolve_bfp_LUSOLidentity"]
pub unsafe extern "C" fn bfp_LUSOLidentity(
    mut lp: *mut lprec,
    mut rownum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut invB: *mut INVrec = (*lp).invB;
    LUSOL_clear((*invB).LUSOL, TRUE as ::core::ffi::c_uchar);
    (*(*lp).invB).set_Bidentity = TRUE as ::core::ffi::c_uchar;
    i = 1 as ::core::ffi::c_int;
    while i <= (*invB).dimcount {
        nz = (*lp).get_basiscolumn.expect("non-null function pointer")(
            lp,
            i,
            rownum as *mut ::core::ffi::c_int,
            (*invB).value as *mut ::core::ffi::c_double,
        );
        LUSOL_loadColumn(
            (*invB).LUSOL,
            rownum as *mut ::core::ffi::c_int,
            i,
            (*invB).value as *mut ::core::ffi::c_double,
            nz,
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    (*(*lp).invB).set_Bidentity = FALSE as ::core::ffi::c_uchar;
    i = LUSOL_factorize((*invB).LUSOL);
    return i;
}
#[export_name="honest_lpsolve_bfp_LUSOLfactorize"]
pub unsafe extern "C" fn bfp_LUSOLfactorize(
    mut lp: *mut lprec,
    mut usedpos: *mut ::core::ffi::c_uchar,
    mut rownum: *mut ::core::ffi::c_int,
    mut singular: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut deltarows: ::core::ffi::c_int = bfp_rowoffset(lp);
    let mut invB: *mut INVrec = (*lp).invB;
    if singular.is_null() {
        LUSOL_clear((*invB).LUSOL, TRUE as ::core::ffi::c_uchar);
        i = 1 as ::core::ffi::c_int;
        while i <= (*invB).dimcount {
            nz = (*lp).get_basiscolumn.expect("non-null function pointer")(
                lp,
                i,
                rownum as *mut ::core::ffi::c_int,
                (*invB).value as *mut ::core::ffi::c_double,
            );
            LUSOL_loadColumn(
                (*invB).LUSOL,
                rownum as *mut ::core::ffi::c_int,
                i,
                (*invB).value as *mut ::core::ffi::c_double,
                nz,
                0 as ::core::ffi::c_int,
            );
            if i > deltarows && *(*lp).var_basic.offset((i - deltarows) as isize) > (*lp).rows {
                (*(*lp).invB).user_colcount += 1;
            }
            i += 1;
        }
        i = LUSOL_factorize((*invB).LUSOL);
    } else {
        let mut map: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
        i = bfp_LUSOLidentity(lp, rownum);
        nz = createLink(
            (*lp).rows,
            &raw mut map,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            if *(*lp).var_basic.offset(i as isize) <= (*lp).rows {
                removeLink(map, i);
            }
            i += 1;
        }
        j = firstActiveLink(map);
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            if !(*(*lp).var_basic.offset(i as isize) <= (*lp).rows) {
                nz = bfp_LUSOLsetcolumn(lp, j + deltarows, *(*lp).var_basic.offset(i as isize));
                if nz == LUSOL_INFORM_LUSUCCESS {
                    (*(*lp).invB).user_colcount += 1;
                } else {
                    nz = bfp_LUSOLsetcolumn(lp, j + deltarows, i);
                    (*lp).set_basisvar.expect("non-null function pointer")(lp, i, i);
                }
                j = nextActiveLink(map, j);
            }
            i += 1;
        }
        memcpy(
            rownum as *mut ::core::ffi::c_void,
            (*lp).var_basic as *const ::core::ffi::c_void,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        sortByINT(
            (*lp).var_basic,
            rownum,
            (*lp).rows,
            1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    return i;
}
#[export_name="honest_lpsolve_bfp_LUSOLtighten"]
pub unsafe extern "C" fn bfp_LUSOLtighten(mut lp: *mut lprec) {
    let mut infolevel: ::core::ffi::c_int = DETAILED;
    match LUSOL_tightenpivot((*(*lp).invB).LUSOL) as ::core::ffi::c_int {
        FALSE => {
            (*lp)
                .report
                .expect(
                    "non-null function pointer",
                )(
                lp,
                infolevel,
                b"bfp_factorize: Very hard numerics, but cannot tighten LUSOL thresholds further.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        TRUE => {
            (*lp)
                .report
                .expect(
                    "non-null function pointer",
                )(
                lp,
                infolevel,
                b"bfp_factorize: Frequent refact pivot count %d at iter %.0f; tightened thresholds.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        _ => {
            (*lp).report.expect("non-null function pointer")(
                lp,
                infolevel,
                b"bfp_factorize: LUSOL switched to %s pivoting model to enhance stability.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn is_fixedvar_(
    mut lp: *mut lprec,
    mut variable: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if !(*lp).bb_bounds.is_null() && (*(*lp).bb_bounds).UBzerobased as ::core::ffi::c_int != 0
        || variable <= (*lp).rows
    {
        return (*(*lp).upbo.offset(variable as isize) < (*lp).epsprimal) as ::core::ffi::c_int
            as ::core::ffi::c_uchar;
    } else {
        return (*(*lp).upbo.offset(variable as isize) - *(*lp).lowbo.offset(variable as isize)
            < (*lp).epsprimal) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_bfp_factorize"]
pub unsafe extern "C" fn bfp_factorize(
    mut lp: *mut lprec,
    mut uservars: ::core::ffi::c_int,
    mut Bsize: ::core::ffi::c_int,
    mut usedpos: *mut ::core::ffi::c_uchar,
    mut final_0: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut kcol: ::core::ffi::c_int = 0;
    let mut inform: ::core::ffi::c_int = 0;
    let mut rownum: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut singularities: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dimsize: ::core::ffi::c_int = (*(*lp).invB).dimcount;
    let mut LUSOL: *mut LUSOLrec = (*(*lp).invB).LUSOL;
    if (*(*lp).invB).max_Bsize < Bsize + (1 as ::core::ffi::c_int + (*lp).rows - uservars) {
        (*(*lp).invB).max_Bsize = Bsize + (1 as ::core::ffi::c_int + (*lp).rows - uservars);
    }
    kcol = (*(*lp).invB).dimcount;
    (*LUSOL).m = kcol;
    (*LUSOL).n = kcol;
    allocINT(
        lp,
        &raw mut rownum,
        kcol + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    inform = (*lp).bfp_pivotcount.expect("non-null function pointer")(lp);
    if final_0 == 0
        && (*(*lp).invB).force_refact == 0
        && (*lp).is_action.expect("non-null function pointer")(
            (*lp).spx_action,
            ACTION_TIMEDREINVERT,
        ) == 0
        && inform > 5 as ::core::ffi::c_int
        && (inform as ::core::ffi::c_double)
            < 0.25f64
                * (*lp).bfp_pivotmax.expect("non-null function pointer")(lp)
                    as ::core::ffi::c_double
    {
        bfp_LUSOLtighten(lp);
    }
    inform = bfp_LUSOLfactorize(
        lp,
        usedpos,
        rownum,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if inform != LUSOL_INFORM_LUSUCCESS {
        let mut singularcols: ::core::ffi::c_int = 0;
        let mut replacedcols: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut hold: ::core::ffi::c_double = 0.;
        if ((*(*lp).invB).num_singular + 1 as ::core::ffi::c_int) % TIGHTENAFTER
            == 0 as ::core::ffi::c_int
        {
            bfp_LUSOLtighten(lp);
        }
        while inform == LUSOL_INFORM_LUSINGULAR && replacedcols < dimsize {
            let mut iLeave: ::core::ffi::c_int = 0;
            let mut jLeave: ::core::ffi::c_int = 0;
            let mut iEnter: ::core::ffi::c_int = 0;
            let mut isfixed: ::core::ffi::c_uchar = 0;
            singularities += 1;
            singularcols = (*LUSOL).luparm[LUSOL_IP_SINGULARITIES as usize];
            hold = (*lp).get_total_iter.expect("non-null function pointer")(lp)
                as ::core::ffi::c_double;
            (*lp).report.expect("non-null function pointer")(
                lp,
                4 as ::core::ffi::c_int,
                b"bfp_factorize: Resolving %d singularit%s at refact %d, iter %.0f\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            kcol = 1 as ::core::ffi::c_int;
            while kcol <= singularcols {
                iLeave = LUSOL_getSingularity(LUSOL, kcol);
                iEnter = iLeave;
                iEnter = *(*LUSOL).iqinv.offset(iEnter as isize);
                iEnter = *(*LUSOL).ip.offset(iEnter as isize);
                iLeave -= bfp_rowextra(lp);
                jLeave = *(*lp).var_basic.offset(iLeave as isize);
                iEnter -= bfp_rowextra(lp);
                if *(*lp).is_basic.offset(iEnter as isize) != 0 {
                    (*lp).report.expect("non-null function pointer")(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"bfp_factorize: Replacement slack %d is already basic!\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    iEnter = 0 as ::core::ffi::c_int;
                    inform = 1 as ::core::ffi::c_int;
                    while inform <= (*lp).rows {
                        if *(*lp).is_basic.offset(inform as isize) == 0 {
                            if iEnter == 0 as ::core::ffi::c_int
                                || *(*lp).upbo.offset(inform as isize)
                                    > *(*lp).upbo.offset(iEnter as isize)
                            {
                                iEnter = inform;
                                if (fabs(*(*lp).upbo.offset(iEnter as isize)) >= (*lp).infinite)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uchar
                                    != 0
                                {
                                    break;
                                }
                            }
                        }
                        inform += 1;
                    }
                    if iEnter == 0 as ::core::ffi::c_int {
                        (*lp).report.expect("non-null function pointer")(
                            lp,
                            2 as ::core::ffi::c_int,
                            b"bfp_factorize: Could not find replacement slack variable!\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        break;
                    }
                }
                isfixed = is_fixedvar_(lp, iEnter);
                if isfixed != 0 {
                    (*lp).fixedvars += 1;
                }
                hold = *(*lp).upbo.offset(jLeave as isize);
                *(*lp).is_lower.offset(jLeave as isize) = (isfixed as ::core::ffi::c_int != 0
                    || fabs(hold) >= (*lp).infinite
                    || *(*lp).rhs.offset(iLeave as isize) < hold)
                    as ::core::ffi::c_int
                    as ::core::ffi::c_uchar;
                *(*lp).is_lower.offset(iEnter as isize) = TRUE as ::core::ffi::c_uchar;
                (*lp).set_basisvar.expect("non-null function pointer")(lp, iLeave, iEnter);
                kcol += 1;
            }
            inform = bfp_LUSOLfactorize(
                lp,
                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                rownum,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            replacedcols += singularcols;
        }
        if singularities >= dimsize {
            (*lp).report.expect("non-null function pointer")(
                lp,
                3 as ::core::ffi::c_int,
                b"bfp_factorize: LUSOL was unable to recover from a singular basis\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            (*lp).spx_status = NUMFAILURE;
        }
    }
    if !(rownum as *mut ::core::ffi::c_void).is_null() {
        free(rownum as *mut ::core::ffi::c_void);
        rownum = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    (*(*lp).invB).num_singular += singularities;
    return singularities;
}
#[export_name="honest_lpsolve_bfp_finishupdate"]
pub unsafe extern "C" fn bfp_finishupdate(
    mut lp: *mut lprec,
    mut changesign: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut deltarows: ::core::ffi::c_int = bfp_rowoffset(lp);
    let mut DIAG: ::core::ffi::c_double = 0.;
    let mut VNORM: ::core::ffi::c_double = 0.;
    let mut lu: *mut INVrec = (*lp).invB;
    let mut LUSOL: *mut LUSOLrec = (*lu).LUSOL;
    if (*lu).is_dirty == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if (*lu).is_dirty as ::core::ffi::c_int != AUTOMATIC {
        (*lu).is_dirty = FALSE as ::core::ffi::c_uchar;
    }
    k = (*lu).col_pos + deltarows;
    (*lu).num_pivots += 1;
    if (*lu).col_leave > (*lu).dimcount - deltarows {
        (*lu).user_colcount -= 1;
    }
    if (*lu).col_enter > (*lu).dimcount - deltarows {
        (*lu).user_colcount += 1;
    }
    (*lu).col_pos = 0 as ::core::ffi::c_int;
    if TRUE != 0 || changesign == 0 {
        if changesign != 0 {
            let mut temp: *mut ::core::ffi::c_double = (*LUSOL).vLU6L;
            i = 1 as ::core::ffi::c_int;
            temp = temp.offset(1);
            while i <= (*lp).rows + deltarows {
                if *temp != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    *temp = -*temp;
                }
                i += 1;
                temp = temp.offset(1);
            }
        }
        LU8RPC(
            LUSOL,
            LUSOL_UPDATE_OLDNONEMPTY,
            LUSOL_UPDATE_USEPREPARED,
            k,
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            &raw mut i,
            &raw mut DIAG,
            &raw mut VNORM,
        );
    } else {
        i = (*lp).get_lpcolumn.expect("non-null function pointer")(
            lp,
            (*lu).col_enter,
            (*lu).value.offset(deltarows as isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        *(*lu).value.offset(0 as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int as ::core::ffi::c_double;
        LU8RPC(
            LUSOL,
            LUSOL_UPDATE_OLDNONEMPTY,
            LUSOL_UPDATE_NEWNONEMPTY,
            k,
            (*lu).value as *mut ::core::ffi::c_double,
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            &raw mut i,
            &raw mut DIAG,
            &raw mut VNORM,
        );
    }
    if i == LUSOL_INFORM_LUSUCCESS {
        DIAG = ((*LUSOL).luparm[LUSOL_IP_NONZEROS_L as usize]
            + (*LUSOL).luparm[LUSOL_IP_NONZEROS_U as usize])
            as ::core::ffi::c_double;
        VNORM = ((*LUSOL).luparm[LUSOL_IP_NONZEROS_L0 as usize]
            + (*LUSOL).luparm[LUSOL_IP_NONZEROS_U0 as usize])
            as ::core::ffi::c_double;
        VNORM *= pow(
            MAX_DELTAFILLIN,
            pow(
                0.5f64 * (*LUSOL).nelem as ::core::ffi::c_double / VNORM,
                0.25f64,
            ),
        );
        (*lu).force_refact = (DIAG > VNORM && (*lu).num_pivots > 20 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        let mut infolevel: ::core::ffi::c_int = DETAILED;
        (*lp).report.expect("non-null function pointer")(
            lp,
            infolevel,
            b"bfp_finishupdate: Failed at iter %.0f, pivot %d;\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if i == LUSOL_INFORM_ANEEDMEM {
            (*lp).invert.expect("non-null function pointer")(
                lp,
                INITSOL_USEZERO as ::core::ffi::c_uchar,
                FALSE as ::core::ffi::c_uchar,
            );
            if i != LUSOL_INFORM_LUSUCCESS {
                (*lp).report.expect("non-null function pointer")(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"bfp_finishupdate: Insufficient memory at iter %.0f;\n%s\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        } else if i == LUSOL_INFORM_RANKLOSS {
            (*lp).invert.expect("non-null function pointer")(
                lp,
                INITSOL_USEZERO as ::core::ffi::c_uchar,
                FALSE as ::core::ffi::c_uchar,
            );
            i = (*LUSOL).luparm[LUSOL_IP_INFORM as usize];
            if i != LUSOL_INFORM_LUSUCCESS {
                (*lp).report.expect("non-null function pointer")(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"bfp_finishupdate: Recovery attempt unsuccessful at iter %.0f;\n%s\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                (*lp).report.expect("non-null function pointer")(
                    lp,
                    infolevel,
                    b"bfp_finishupdate: Correction or recovery was successful.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    return (i == LUSOL_INFORM_LUSUCCESS) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bfp_ftran_normal"]
pub unsafe extern "C" fn bfp_ftran_normal(
    mut lp: *mut lprec,
    mut pcol: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    i = LUSOL_ftran(
        (*lu).LUSOL,
        pcol.offset(-(bfp_rowoffset(lp) as isize)),
        nzidx as *mut ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    if i != LUSOL_INFORM_LUSUCCESS {
        (*lu).status = BFP_STATUS_ERROR;
        (*lp).report.expect("non-null function pointer")(
            lp,
            4 as ::core::ffi::c_int,
            b"bfp_ftran_normal: Failed at iter %.0f, pivot %d;\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
}
#[export_name="honest_lpsolve_bfp_ftran_prepare"]
pub unsafe extern "C" fn bfp_ftran_prepare(
    mut lp: *mut lprec,
    mut pcol: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    i = LUSOL_ftran(
        (*lu).LUSOL,
        pcol.offset(-(bfp_rowoffset(lp) as isize)),
        nzidx as *mut ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    if i != LUSOL_INFORM_LUSUCCESS {
        (*lu).status = BFP_STATUS_ERROR;
        (*lp).report.expect("non-null function pointer")(
            lp,
            4 as ::core::ffi::c_int,
            b"bfp_ftran_prepare: Failed at iter %.0f, pivot %d;\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
}
#[export_name="honest_lpsolve_bfp_btran_normal"]
pub unsafe extern "C" fn bfp_btran_normal(
    mut lp: *mut lprec,
    mut prow: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut lu: *mut INVrec = ::core::ptr::null_mut::<INVrec>();
    lu = (*lp).invB;
    i = LUSOL_btran(
        (*lu).LUSOL,
        prow.offset(-(bfp_rowoffset(lp) as isize)),
        nzidx as *mut ::core::ffi::c_int,
    );
    if i != LUSOL_INFORM_LUSUCCESS {
        (*lu).status = BFP_STATUS_ERROR;
        (*lp).report.expect("non-null function pointer")(
            lp,
            4 as ::core::ffi::c_int,
            b"bfp_btran_normal: Failed at iter %.0f, pivot %d;\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
}
#[export_name="honest_lpsolve_bfp_findredundant"]
pub unsafe extern "C" fn bfp_findredundant(
    mut lp: *mut lprec,
    mut items: ::core::ffi::c_int,
    mut cb: Option<getcolumnex_func>,
    mut maprow: *mut ::core::ffi::c_int,
    mut mapcol: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nzrows: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nzvalues: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut arraymax: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut LUSOL: *mut LUSOLrec = ::core::ptr::null_mut::<LUSOLrec>();
    if maprow.is_null() && mapcol.is_null() {
        return n;
    }
    if allocINT(lp, &raw mut nzrows, items, FALSE as ::core::ffi::c_uchar) == 0
        || allocREAL(lp, &raw mut nzvalues, items, FALSE as ::core::ffi::c_uchar) == 0
    {
        return n;
    }
    m = 0 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    while j <= *mapcol.offset(0 as ::core::ffi::c_int as isize) {
        n = cb.expect("non-null function pointer")(
            lp,
            *mapcol.offset(j as isize),
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            maprow,
        );
        if n > 0 as ::core::ffi::c_int {
            m += 1;
            *mapcol.offset(m as isize) = *mapcol.offset(j as isize);
            nz += n;
        }
        j += 1;
    }
    *mapcol.offset(0 as ::core::ffi::c_int as isize) = m;
    LUSOL = LUSOL_create(
        ::core::ptr::null_mut::<FILE>(),
        0 as ::core::ffi::c_int,
        LUSOL_PIVMOD_TRP,
        0 as ::core::ffi::c_int,
    );
    if !(LUSOL.is_null() || LUSOL_sizeto(LUSOL, items, m, nz * LUSOL_MULT_nz_a) == 0) {
        (*LUSOL).m = items;
        (*LUSOL).n = m;
        j = 1 as ::core::ffi::c_int;
        loop {
            if !(j <= m) {
                current_block = 4495394744059808450;
                break;
            }
            n = cb.expect("non-null function pointer")(
                lp,
                *mapcol.offset(j as isize),
                nzvalues,
                nzrows,
                maprow,
            );
            i = LUSOL_loadColumn(
                LUSOL,
                nzrows as *mut ::core::ffi::c_int,
                j,
                nzvalues as *mut ::core::ffi::c_double,
                n,
                -(1 as ::core::ffi::c_int),
            );
            if n != i {
                (*lp).report.expect("non-null function pointer")(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"bfp_findredundant: Error %d while loading column %d with %d nz\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                n = 0 as ::core::ffi::c_int;
                current_block = 396889919800514278;
                break;
            } else {
                j += 1;
            }
        }
        match current_block {
            396889919800514278 => {}
            _ => {
                if (*lp).scalemode != SCALE_NONE
                    && allocREAL(
                        lp,
                        &raw mut arraymax,
                        items + 1 as ::core::ffi::c_int,
                        TRUE as ::core::ffi::c_uchar,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    i = 1 as ::core::ffi::c_int;
                    while i <= nz {
                        if *arraymax.offset(*(*LUSOL).indc.offset(i as isize) as isize)
                            < fabs(*(*LUSOL).a.offset(i as isize))
                        {
                            *arraymax.offset(*(*LUSOL).indc.offset(i as isize) as isize) =
                                fabs(*(*LUSOL).a.offset(i as isize));
                        }
                        i += 1;
                    }
                    i = 1 as ::core::ffi::c_int;
                    while i <= nz {
                        *(*LUSOL).a.offset(i as isize) /=
                            *arraymax.offset(*(*LUSOL).indc.offset(i as isize) as isize);
                        i += 1;
                    }
                    if !(arraymax as *mut ::core::ffi::c_void).is_null() {
                        free(arraymax as *mut ::core::ffi::c_void);
                        arraymax = ::core::ptr::null_mut::<::core::ffi::c_double>();
                    }
                }
                n = 0 as ::core::ffi::c_int;
                i = LUSOL_factorize(LUSOL);
                if !(i == LUSOL_INFORM_LUSUCCESS || i != LUSOL_INFORM_LUSINGULAR) {
                    i = (*LUSOL).luparm[LUSOL_IP_RANK_U as usize] + 1 as ::core::ffi::c_int;
                    while i <= items {
                        n += 1;
                        *maprow.offset(n as isize) = *(*LUSOL).ip.offset(i as isize);
                        i += 1;
                    }
                    *maprow.offset(0 as ::core::ffi::c_int as isize) = n;
                }
            }
        }
    }
    LUSOL_free(LUSOL);
    if !(nzrows as *mut ::core::ffi::c_void).is_null() {
        free(nzrows as *mut ::core::ffi::c_void);
        nzrows = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(nzvalues as *mut ::core::ffi::c_void).is_null() {
        free(nzvalues as *mut ::core::ffi::c_void);
        nzvalues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return n;
}
pub const BFPVERSION: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const INITSOL_USEZERO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DETAILED: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SCALE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ACTION_REINVERT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ACTION_TIMEDREINVERT: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NUMFAILURE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MIN_TIMEPIVOT: ::core::ffi::c_double = 5.0e-02f64;
pub const LUSOL_ARRAYOFFSET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_MULT_nz_a: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_RP_SMARTRATIO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_RP_RESIDUAL_U: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const LUSOL_RP_LASTITEM: ::core::ffi::c_int = LUSOL_RP_RESIDUAL_U;
pub const LUSOL_IP_PIVOTTYPE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LUSOL_IP_ACCELERATION: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LUSOL_IP_KEEPLU: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LUSOL_IP_INFORM: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LUSOL_IP_SINGULARITIES: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const LUSOL_IP_RANK_U: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_L0: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_U0: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_L: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const LUSOL_IP_NONZEROS_U: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const LUSOL_IP_ROWCOUNT_L0: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const LUSOL_IP_LASTITEM: ::core::ffi::c_int = LUSOL_IP_ROWCOUNT_L0;
pub const LUSOL_AUTOORDER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_NOCHANGE: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const LUSOL_PIVMOD_TPP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_TRP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_PIVMOD_TCP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_PIVTOL_SLIM: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LUSOL_UPDATE_OLDNONEMPTY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_UPDATE_NEWNONEMPTY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_UPDATE_USEPREPARED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUSOL_INFORM_RANKLOSS: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const LUSOL_INFORM_LUSUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LUSOL_INFORM_LUSINGULAR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LUSOL_INFORM_ANEEDMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
