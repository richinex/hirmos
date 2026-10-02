use crate::honest_did::lpsolve::runtime::{malloc,calloc,free,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_scaled_mat"]
    fn scaled_mat(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_partial_blockStart"]
    fn partial_blockStart(lp: *mut lprec, isrow: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_partial_blockEnd"]
    fn partial_blockEnd(lp: *mut lprec, isrow: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_restartPricer"]
    fn restartPricer(lp: *mut lprec, isdual: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
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
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
    #[link_name="honest_lpsolve_roundVector"]
    fn roundVector(
        myvector: *mut ::core::ffi::c_double,
        endpos: ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
    );
    #[link_name="honest_lpsolve_swapINT"]
    fn swapINT(item1: *mut ::core::ffi::c_int, item2: *mut ::core::ffi::c_int);
    #[link_name="honest_lpsolve_swapPTR"]
    fn swapPTR(item1: *mut *mut ::core::ffi::c_void, item2: *mut *mut ::core::ffi::c_void);
    #[link_name="honest_lpsolve_roundToPrecision"]
    fn roundToPrecision(
        value: ::core::ffi::c_double,
        precision: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_isActiveLink"]
    fn isActiveLink(linkmap: *mut LLrec, itemnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_maxim"]
    fn is_maxim(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_mat_byindex"]
    fn get_mat_byindex(
        lp: *mut lprec,
        matindex: ::core::ffi::c_int,
        isrow: ::core::ffi::c_uchar,
        adjustsign: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_is_piv_mode"]
    fn is_piv_mode(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_Lrows"]
    fn get_Lrows(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_userabort"]
    fn userabort(lp: *mut lprec, message: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_inc_col_space"]
    fn inc_col_space(lp: *mut lprec, deltacols: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_shift_coldata"]
    fn shift_coldata(
        lp: *mut lprec,
        base: ::core::ffi::c_int,
        delta: ::core::ffi::c_int,
        usedmap: *mut LLrec,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_chsign"]
    fn is_chsign(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_recompute_solution"]
    fn recompute_solution(lp: *mut lprec, shiftbounds: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_set_action"]
    fn set_action(actionvar: *mut ::core::ffi::c_int, actionmask: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_refactfrequency"]
    fn get_refactfrequency(lp: *mut lprec, final_0: ::core::ffi::c_uchar) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_OF_active"]
    fn get_OF_active(
        lp: *mut lprec,
        varnr: ::core::ffi::c_int,
        mult: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_is_OF_nz"]
    fn is_OF_nz(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_basisOF"]
    fn get_basisOF(
        lp: *mut lprec,
        coltarget: *mut ::core::ffi::c_int,
        crow: *mut ::core::ffi::c_double,
        colno: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_obtain_column"]
    fn obtain_column(
        lp: *mut lprec,
        varin: ::core::ffi::c_int,
        pcol: *mut ::core::ffi::c_double,
        nzlist: *mut ::core::ffi::c_int,
        maxabs: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_compareINT"]
    fn compareINT(
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
    #[link_name="honest_lpsolve_sortREALByINT"]
    fn sortREALByINT(
        item: *mut ::core::ffi::c_double,
        weight: *mut ::core::ffi::c_int,
        size: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        unique: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_double;
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
#[export_name="honest_lpsolve_mat_create"]
pub unsafe extern "C" fn mat_create(
    mut lp: *mut lprec,
    mut rows: ::core::ffi::c_int,
    mut columns: ::core::ffi::c_int,
    mut epsvalue: ::core::ffi::c_double,
) -> *mut MATrec {
    let mut newmat: *mut MATrec = ::core::ptr::null_mut::<MATrec>();
    newmat = calloc(1 as size_t, ::core::mem::size_of::<MATrec>() as size_t) as *mut MATrec;
    (*newmat).lp = lp;
    (*newmat).rows_alloc = 0 as ::core::ffi::c_int;
    (*newmat).columns_alloc = 0 as ::core::ffi::c_int;
    (*newmat).mat_alloc = 0 as ::core::ffi::c_int;
    inc_matrow_space(newmat, rows);
    (*newmat).rows = rows;
    inc_matcol_space(newmat, columns);
    (*newmat).columns = columns;
    inc_mat_space(newmat, 0 as ::core::ffi::c_int);
    (*newmat).epsvalue = epsvalue;
    return newmat;
}
#[export_name="honest_lpsolve_mat_free"]
pub unsafe extern "C" fn mat_free(mut matrix: *mut *mut MATrec) {
    if matrix.is_null() || (*matrix).is_null() {
        return;
    }
    if !((**matrix).col_mat_colnr as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).col_mat_colnr as *mut ::core::ffi::c_void);
        (**matrix).col_mat_colnr = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**matrix).col_mat_rownr as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).col_mat_rownr as *mut ::core::ffi::c_void);
        (**matrix).col_mat_rownr = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**matrix).col_mat_value as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).col_mat_value as *mut ::core::ffi::c_void);
        (**matrix).col_mat_value = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**matrix).col_end as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).col_end as *mut ::core::ffi::c_void);
        (**matrix).col_end = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**matrix).col_tag as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).col_tag as *mut ::core::ffi::c_void);
        (**matrix).col_tag = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**matrix).row_mat as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).row_mat as *mut ::core::ffi::c_void);
        (**matrix).row_mat = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**matrix).row_end as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).row_end as *mut ::core::ffi::c_void);
        (**matrix).row_end = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**matrix).row_tag as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).row_tag as *mut ::core::ffi::c_void);
        (**matrix).row_tag = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**matrix).colmax as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).colmax as *mut ::core::ffi::c_void);
        (**matrix).colmax = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**matrix).rowmax as *mut ::core::ffi::c_void).is_null() {
        free((**matrix).rowmax as *mut ::core::ffi::c_void);
        (**matrix).rowmax = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(*matrix as *mut ::core::ffi::c_void).is_null() {
        free(*matrix as *mut ::core::ffi::c_void);
        *matrix = ::core::ptr::null_mut::<MATrec>();
    }
}
#[export_name="honest_lpsolve_mat_memopt"]
pub unsafe extern "C" fn mat_memopt(
    mut mat: *mut MATrec,
    mut rowextra: ::core::ffi::c_int,
    mut colextra: ::core::ffi::c_int,
    mut nzextra: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut matalloc: ::core::ffi::c_int = 0;
    let mut colalloc: ::core::ffi::c_int = 0;
    let mut rowalloc: ::core::ffi::c_int = 0;
    if mat.is_null()
        || rowextra < 0 as ::core::ffi::c_int
        || colextra < 0 as ::core::ffi::c_int
        || nzextra < 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_uchar;
    }
    (*mat).rows_alloc = if (*mat).rows_alloc < (*mat).rows + rowextra {
        (*mat).rows_alloc
    } else {
        (*mat).rows + rowextra
    };
    (*mat).columns_alloc = if (*mat).columns_alloc < (*mat).columns + colextra {
        (*mat).columns_alloc
    } else {
        (*mat).columns + colextra
    };
    (*mat).mat_alloc =
        if (*mat).mat_alloc < *(*mat).col_end.offset((*mat).columns as isize) + nzextra {
            (*mat).mat_alloc
        } else {
            *(*mat).col_end.offset((*mat).columns as isize) + nzextra
        };
    rowalloc = (*mat).rows_alloc + 1 as ::core::ffi::c_int;
    colalloc = (*mat).columns_alloc + 1 as ::core::ffi::c_int;
    matalloc = (*mat).mat_alloc + 1 as ::core::ffi::c_int;
    status = (status as ::core::ffi::c_int
        & (allocINT(
            (*mat).lp,
            &raw mut (*mat).col_mat_colnr,
            matalloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0
            && allocINT(
                (*mat).lp,
                &raw mut (*mat).col_mat_rownr,
                matalloc,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0
            && allocREAL(
                (*mat).lp,
                &raw mut (*mat).col_mat_value,
                matalloc,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    status = (status as ::core::ffi::c_int
        & allocINT(
            (*mat).lp,
            &raw mut (*mat).col_end,
            colalloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    if !(*mat).col_tag.is_null() {
        status = (status as ::core::ffi::c_int
            & allocINT(
                (*mat).lp,
                &raw mut (*mat).col_tag,
                colalloc,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
    status = (status as ::core::ffi::c_int
        & allocINT(
            (*mat).lp,
            &raw mut (*mat).row_mat,
            matalloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    status = (status as ::core::ffi::c_int
        & allocINT(
            (*mat).lp,
            &raw mut (*mat).row_end,
            rowalloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    if !(*mat).row_tag.is_null() {
        status = (status as ::core::ffi::c_int
            & allocINT(
                (*mat).lp,
                &raw mut (*mat).row_tag,
                rowalloc,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
    if !(*mat).colmax.is_null() {
        status = (status as ::core::ffi::c_int
            & allocREAL(
                (*mat).lp,
                &raw mut (*mat).colmax,
                colalloc,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
    if !(*mat).rowmax.is_null() {
        status = (status as ::core::ffi::c_int
            & allocREAL(
                (*mat).lp,
                &raw mut (*mat).rowmax,
                rowalloc,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
    return status;
}
#[export_name="honest_lpsolve_inc_mat_space"]
pub unsafe extern "C" fn inc_mat_space(
    mut mat: *mut MATrec,
    mut mindelta: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut spaceneeded: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = mat_nonzeros(mat);
    if mindelta <= 0 as ::core::ffi::c_int {
        mindelta = (if (*mat).rows > (*mat).columns {
            (*mat).rows
        } else {
            (*mat).columns
        }) + 1 as ::core::ffi::c_int;
    }
    spaceneeded = (mindelta as ::core::ffi::c_double
        * (if 1.33f64
            < pow(
                1.5f64,
                fabs(mindelta as ::core::ffi::c_double)
                    / (nz + mindelta + 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            )
        {
            1.33f64
        } else {
            pow(
                1.5f64,
                fabs(mindelta as ::core::ffi::c_double)
                    / (nz + mindelta + 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            )
        })) as ::core::ffi::c_int;
    if mindelta < spaceneeded {
        mindelta = spaceneeded;
    }
    if (*mat).mat_alloc == 0 as ::core::ffi::c_int {
        spaceneeded = mindelta;
    } else {
        spaceneeded = nz + mindelta;
    }
    if spaceneeded >= (*mat).mat_alloc {
        if (*mat).mat_alloc < MAT_START_SIZE {
            (*mat).mat_alloc = MAT_START_SIZE;
        }
        while spaceneeded >= (*mat).mat_alloc {
            (*mat).mat_alloc += (*mat).mat_alloc / RESIZEFACTOR;
        }
        allocINT(
            (*mat).lp,
            &raw mut (*mat).col_mat_colnr,
            (*mat).mat_alloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        allocINT(
            (*mat).lp,
            &raw mut (*mat).col_mat_rownr,
            (*mat).mat_alloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        allocREAL(
            (*mat).lp,
            &raw mut (*mat).col_mat_value,
            (*mat).mat_alloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        allocINT(
            (*mat).lp,
            &raw mut (*mat).row_mat,
            (*mat).mat_alloc,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_inc_matrow_space"]
pub unsafe extern "C" fn inc_matrow_space(
    mut mat: *mut MATrec,
    mut deltarows: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut rowsum: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if (*mat).rows + deltarows >= (*mat).rows_alloc {
        deltarows = (deltarows as ::core::ffi::c_double
            * (if 1.33f64
                < pow(
                    1.5f64,
                    fabs(deltarows as ::core::ffi::c_double)
                        / ((*mat).rows + deltarows + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_double,
                )
            {
                1.33f64
            } else {
                pow(
                    1.5f64,
                    fabs(deltarows as ::core::ffi::c_double)
                        / ((*mat).rows + deltarows + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_double,
                )
            })) as ::core::ffi::c_int;
        if deltarows < 100 as ::core::ffi::c_int {
            deltarows = 100 as ::core::ffi::c_int;
        }
        (*mat).rows_alloc += deltarows;
        rowsum = (*mat).rows_alloc + 1 as ::core::ffi::c_int;
        status = allocINT(
            (*mat).lp,
            &raw mut (*mat).row_end,
            rowsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    }
    return status;
}
#[export_name="honest_lpsolve_inc_matcol_space"]
pub unsafe extern "C" fn inc_matcol_space(
    mut mat: *mut MATrec,
    mut deltacols: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut colsum: ::core::ffi::c_int = 0;
    let mut oldcolsalloc: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if (*mat).columns + deltacols >= (*mat).columns_alloc {
        oldcolsalloc = (*mat).columns_alloc;
        deltacols = (deltacols as ::core::ffi::c_double
            * (if 1.33f64
                < pow(
                    1.5f64,
                    fabs(deltacols as ::core::ffi::c_double)
                        / ((*mat).columns + deltacols + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_double,
                )
            {
                1.33f64
            } else {
                pow(
                    1.5f64,
                    fabs(deltacols as ::core::ffi::c_double)
                        / ((*mat).columns + deltacols + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_double,
                )
            })) as ::core::ffi::c_int;
        if deltacols < 100 as ::core::ffi::c_int {
            deltacols = 100 as ::core::ffi::c_int;
        }
        (*mat).columns_alloc += deltacols;
        colsum = (*mat).columns_alloc + 1 as ::core::ffi::c_int;
        status = allocINT(
            (*mat).lp,
            &raw mut (*mat).col_end,
            colsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        if oldcolsalloc == 0 as ::core::ffi::c_int {
            *(*mat).col_end.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
        }
        i = (if oldcolsalloc < (*mat).columns {
            oldcolsalloc
        } else {
            (*mat).columns
        }) + 1 as ::core::ffi::c_int;
        while i < colsum {
            *(*mat).col_end.offset(i as isize) = *(*mat)
                .col_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    }
    return status;
}
#[export_name="honest_lpsolve_mat_collength"]
pub unsafe extern "C" fn mat_collength(
    mut mat: *mut MATrec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return *(*mat).col_end.offset(colnr as isize)
        - *(*mat)
            .col_end
            .offset((colnr - 1 as ::core::ffi::c_int) as isize);
}
#[export_name="honest_lpsolve_mat_rowlength"]
pub unsafe extern "C" fn mat_rowlength(
    mut mat: *mut MATrec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if mat_validate(mat) != 0 {
        if rownr <= 0 as ::core::ffi::c_int {
            return *(*mat).row_end.offset(0 as ::core::ffi::c_int as isize);
        } else {
            return *(*mat).row_end.offset(rownr as isize)
                - *(*mat)
                    .row_end
                    .offset((rownr - 1 as ::core::ffi::c_int) as isize);
        }
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_mat_nonzeros"]
pub unsafe extern "C" fn mat_nonzeros(mut mat: *mut MATrec) -> ::core::ffi::c_int {
    return *(*mat).col_end.offset((*mat).columns as isize);
}
#[export_name="honest_lpsolve_mat_indexrange"]
pub unsafe extern "C" fn mat_indexrange(
    mut mat: *mut MATrec,
    mut index: ::core::ffi::c_int,
    mut isrow: ::core::ffi::c_uchar,
    mut startpos: *mut ::core::ffi::c_int,
    mut endpos: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if isrow as ::core::ffi::c_int != 0 && mat_validate(mat) as ::core::ffi::c_int != 0 {
        if index == 0 as ::core::ffi::c_int {
            *startpos = 0 as ::core::ffi::c_int;
        } else {
            *startpos = *(*mat)
                .row_end
                .offset((index - 1 as ::core::ffi::c_int) as isize);
        }
        *endpos = *(*mat).row_end.offset(index as isize);
    } else {
        *startpos = *(*mat)
            .col_end
            .offset((index - 1 as ::core::ffi::c_int) as isize);
        *endpos = *(*mat).col_end.offset(index as isize);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_shiftrows"]
pub unsafe extern "C" fn mat_shiftrows(
    mut mat: *mut MATrec,
    mut bbase: *mut ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut varmap: *mut LLrec,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut thisrow: ::core::ffi::c_int = 0;
    let mut colend: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut base: ::core::ffi::c_int = 0;
    let mut preparecompact: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if delta == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    base = abs(*bbase);
    if delta > 0 as ::core::ffi::c_int {
        if base <= (*mat).rows {
            k = mat_nonzeros(mat);
            rownr = (*mat)
                .col_mat_rownr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            while ii < k {
                if *rownr >= base {
                    *rownr += delta;
                }
                ii += 1;
                rownr = rownr.offset(matRowColStep as isize);
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < delta {
            ii = base + i;
            *(*mat).row_end.offset(ii as isize) = 0 as ::core::ffi::c_int;
            i += 1;
        }
    } else if base <= (*mat).rows {
        preparecompact =
            (varmap != NULL as *mut LLrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if preparecompact != 0 {
            let mut newrowidx: *mut ::core::ffi::c_int =
                ::core::ptr::null_mut::<::core::ffi::c_int>();
            allocINT(
                (*mat).lp,
                &raw mut newrowidx,
                (*mat).rows + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            *newrowidx.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
            delta = 0 as ::core::ffi::c_int;
            j = 1 as ::core::ffi::c_int;
            while j <= (*mat).rows {
                if isActiveLink(varmap, j) != 0 {
                    delta += 1;
                    *newrowidx.offset(j as isize) = delta;
                } else {
                    *newrowidx.offset(j as isize) = -(1 as ::core::ffi::c_int);
                }
                j += 1;
            }
            k = 0 as ::core::ffi::c_int;
            delta = 0 as ::core::ffi::c_int;
            base = mat_nonzeros(mat);
            rownr = (*mat)
                .col_mat_rownr
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < base {
                thisrow = *newrowidx.offset(*rownr as isize);
                if thisrow < 0 as ::core::ffi::c_int {
                    *rownr = -(1 as ::core::ffi::c_int);
                    delta += 1;
                } else {
                    *rownr = thisrow;
                }
                i += 1;
                rownr = rownr.offset(matRowColStep as isize);
            }
            if !(newrowidx as *mut ::core::ffi::c_void).is_null() {
                free(newrowidx as *mut ::core::ffi::c_void);
                newrowidx = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
            return delta;
        }
        preparecompact =
            (*bbase < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if preparecompact != 0 {
            *bbase = if fabs(*bbase as ::core::ffi::c_double)
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int
            } else {
                -*bbase
            };
        }
        if base - delta - 1 as ::core::ffi::c_int > (*mat).rows {
            delta = base - (*mat).rows - 1 as ::core::ffi::c_int;
        }
        if preparecompact != 0 {
            k = 0 as ::core::ffi::c_int;
            j = 1 as ::core::ffi::c_int;
            colend = (*mat).col_end.offset(1 as ::core::ffi::c_int as isize);
            while j <= (*mat).columns {
                i = k;
                k = *colend;
                rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
                while i < k {
                    thisrow = *rownr;
                    if !(thisrow < base) {
                        if thisrow >= base - delta {
                            *rownr += delta;
                        } else {
                            *rownr = -(1 as ::core::ffi::c_int);
                        }
                    }
                    i += 1;
                    rownr = rownr.offset(matRowColStep as isize);
                }
                j += 1;
                colend = colend.offset(1);
            }
        } else {
            k = 0 as ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            j = 1 as ::core::ffi::c_int;
            colend = (*mat).col_end.offset(1 as ::core::ffi::c_int as isize);
            while j <= (*mat).columns {
                i = k;
                k = *colend;
                rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
                let mut current_block_82: u64;
                while i < k {
                    thisrow = *rownr;
                    if thisrow >= base {
                        if thisrow >= base - delta {
                            *rownr += delta;
                            current_block_82 = 4216521074440650966;
                        } else {
                            current_block_82 = 15594839951440953787;
                        }
                    } else {
                        current_block_82 = 4216521074440650966;
                    }
                    match current_block_82 {
                        4216521074440650966 => {
                            if ii != i {
                                *(*mat).col_mat_colnr.offset(ii as isize) =
                                    *(*mat).col_mat_colnr.offset(i as isize);
                                *(*mat).col_mat_rownr.offset(ii as isize) =
                                    *(*mat).col_mat_rownr.offset(i as isize);
                                *(*mat).col_mat_value.offset(ii as isize) =
                                    *(*mat).col_mat_value.offset(i as isize);
                            }
                            ii += 1;
                        }
                        _ => {}
                    }
                    i += 1;
                    rownr = rownr.offset(matRowColStep as isize);
                }
                *colend = ii;
                j += 1;
                colend = colend.offset(1);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_mat_mapreplace"]
pub unsafe extern "C" fn mat_mapreplace(
    mut mat: *mut MATrec,
    mut rowmap: *mut LLrec,
    mut colmap: *mut LLrec,
    mut mat2: *mut MATrec,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*mat).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut colend: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rownr2: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut indirect: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut value2: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if !mat2.is_null()
        && ((*mat2).col_tag.is_null()
            || *(*mat2).col_tag.offset(0 as ::core::ffi::c_int as isize) <= 0 as ::core::ffi::c_int
            || mat_nonzeros(mat2) == 0 as ::core::ffi::c_int)
    {
        return 0 as ::core::ffi::c_int;
    }
    if !mat2.is_null() {
        jj = *(*mat2).col_tag.offset(0 as ::core::ffi::c_int as isize);
        allocINT(
            lp,
            &raw mut indirect,
            jj + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        *indirect.offset(0 as ::core::ffi::c_int as isize) = jj;
        i = 1 as ::core::ffi::c_int;
        while i <= jj {
            *indirect.offset(i as isize) = i;
            i += 1;
        }
        hpsortex(
            (*mat2).col_tag as *mut ::core::ffi::c_void,
            jj,
            1 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
            Some(
                compareINT
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            indirect,
        );
    }
    (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    nz = *(*mat).col_end.offset((*mat).columns as isize);
    ie = 0 as ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    if mat2.is_null()
        || *indirect.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        je = (*mat).columns + 1 as ::core::ffi::c_int;
        jj = 1 as ::core::ffi::c_int;
        jb = 0 as ::core::ffi::c_int;
    } else {
        je = *indirect.offset(0 as ::core::ffi::c_int as isize);
        jj = 0 as ::core::ffi::c_int;
        loop {
            jj += 1;
            jb = *(*mat2).col_tag.offset(jj as isize);
            if !(jb <= 0 as ::core::ffi::c_int) {
                break;
            }
        }
    }
    j = 1 as ::core::ffi::c_int;
    colend = (*mat).col_end.offset(1 as ::core::ffi::c_int as isize);
    while j <= (*mat).columns {
        ib = ie;
        ie = *colend;
        if j == jb {
            jj += 1;
            if jj <= je {
                jb = *(*mat2).col_tag.offset(jj as isize);
            } else {
                jb = (*mat).columns + 1 as ::core::ffi::c_int;
            }
        } else if isActiveLink(colmap, j) != 0 {
            rownr = (*mat).col_mat_rownr.offset(ib as isize) as *mut ::core::ffi::c_int;
            while ib < ie {
                if isActiveLink(rowmap, *rownr) != 0 {
                    if ii != ib {
                        *(*mat).col_mat_colnr.offset(ii as isize) =
                            *(*mat).col_mat_colnr.offset(ib as isize);
                        *(*mat).col_mat_rownr.offset(ii as isize) =
                            *(*mat).col_mat_rownr.offset(ib as isize);
                        *(*mat).col_mat_value.offset(ii as isize) =
                            *(*mat).col_mat_value.offset(ib as isize);
                    }
                    ii += 1;
                }
                ib += 1;
                rownr = rownr.offset(matRowColStep as isize);
            }
        }
        *colend = ii;
        j += 1;
        colend = colend.offset(1);
    }
    if !mat2.is_null() {
        i = 0 as ::core::ffi::c_int;
        j = 1 as ::core::ffi::c_int;
        while j <= *(*mat2).col_tag.offset(0 as ::core::ffi::c_int as isize) {
            jj = *(*mat2).col_tag.offset(j as isize);
            if jj > 0 as ::core::ffi::c_int && isActiveLink(colmap, jj) as ::core::ffi::c_int != 0 {
                jj = *indirect.offset(j as isize);
                je = *(*mat2).col_end.offset(jj as isize);
                jb = *(*mat2)
                    .col_end
                    .offset((jj - 1 as ::core::ffi::c_int) as isize);
                rownr2 = (*mat2).col_mat_rownr.offset(jb as isize) as *mut ::core::ffi::c_int;
                while jb < je {
                    if *rownr2 > 0 as ::core::ffi::c_int
                        && isActiveLink(rowmap, *rownr2) as ::core::ffi::c_int != 0
                    {
                        i += 1;
                    }
                    jb += 1;
                    rownr2 = rownr2.offset(matRowColStep as isize);
                }
            }
            j += 1;
        }
        ii = *(*mat).col_end.offset((*mat).columns as isize) + i;
        if (*mat).mat_alloc <= ii {
            inc_mat_space(mat, i);
        }
        jj = *indirect.offset(0 as ::core::ffi::c_int as isize);
        jj = *(*mat2).col_tag.offset(jj as isize);
        j = (*mat).columns;
        colend = (*mat).col_end.offset((*mat).columns as isize);
        ib = *colend;
        while j > 0 as ::core::ffi::c_int {
            ie = ib;
            *colend = ii;
            colend = colend.offset(-1);
            ib = *colend;
            if j == jj {
                if isActiveLink(colmap, j) != 0 {
                    jj = *indirect.offset(0 as ::core::ffi::c_int as isize);
                    jj = *indirect.offset(jj as isize);
                    rownr = (*mat)
                        .col_mat_rownr
                        .offset((ii - 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_int;
                    value = (*mat)
                        .col_mat_value
                        .offset((ii - 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_double;
                    jb = *(*mat2)
                        .col_end
                        .offset((jj - 1 as ::core::ffi::c_int) as isize);
                    je = *(*mat2).col_end.offset(jj as isize) - 1 as ::core::ffi::c_int;
                    rownr2 = (*mat2).col_mat_rownr.offset(je as isize) as *mut ::core::ffi::c_int;
                    value2 =
                        (*mat2).col_mat_value.offset(je as isize) as *mut ::core::ffi::c_double;
                    while je >= jb {
                        i = *rownr2;
                        if i == 0 as ::core::ffi::c_int {
                            i = -(1 as ::core::ffi::c_int);
                            break;
                        } else {
                            if isActiveLink(rowmap, i) != 0 {
                                ii -= 1;
                                *rownr = i;
                                rownr = rownr.offset(-(matRowColStep as isize));
                                *value = if is_chsign(lp, i) as ::core::ffi::c_int != 0
                                    && *value2 != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    -*value2
                                } else {
                                    *value2
                                };
                                value = value.offset(-(matValueStep as isize));
                            }
                            je -= 1;
                            rownr2 = rownr2.offset(-(matRowColStep as isize));
                            value2 = value2.offset(-(matValueStep as isize));
                        }
                    }
                    if i == -(1 as ::core::ffi::c_int) {
                        *(*lp).orig_obj.offset(j as isize) = if is_maxim(lp) as ::core::ffi::c_int
                            != 0
                            && *value2 != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -*value2
                        } else {
                            *value2
                        };
                        rownr2 = rownr2.offset(-(matRowColStep as isize));
                        value2 = value2.offset(-(matValueStep as isize));
                    } else {
                        *(*lp).orig_obj.offset(j as isize) =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                }
                let ref mut fresh0 = *indirect.offset(0 as ::core::ffi::c_int as isize);
                *fresh0 -= 1;
                jj = *fresh0;
                if jj == 0 as ::core::ffi::c_int {
                    break;
                }
                jj = *(*mat2).col_tag.offset(jj as isize);
                if jj <= 0 as ::core::ffi::c_int {
                    break;
                }
            } else if isActiveLink(colmap, j) != 0 {
                while ie > ib {
                    ii -= 1;
                    ie -= 1;
                    if ie != ii {
                        *(*mat).col_mat_colnr.offset(ii as isize) =
                            *(*mat).col_mat_colnr.offset(ie as isize);
                        *(*mat).col_mat_rownr.offset(ii as isize) =
                            *(*mat).col_mat_rownr.offset(ie as isize);
                        *(*mat).col_mat_value.offset(ii as isize) =
                            *(*mat).col_mat_value.offset(ie as isize);
                    }
                }
            }
            j -= 1;
        }
    }
    nz -= *(*mat).col_end.offset((*mat).columns as isize);
    if !(indirect as *mut ::core::ffi::c_void).is_null() {
        free(indirect as *mut ::core::ffi::c_void);
        indirect = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return nz;
}
#[export_name="honest_lpsolve_mat_zerocompact"]
pub unsafe extern "C" fn mat_zerocompact(mut mat: *mut MATrec) -> ::core::ffi::c_int {
    return mat_rowcompact(mat, TRUE as ::core::ffi::c_uchar);
}
#[export_name="honest_lpsolve_mat_rowcompact"]
pub unsafe extern "C" fn mat_rowcompact(
    mut mat: *mut MATrec,
    mut dozeros: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0;
    let mut colend: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    nn = 0 as ::core::ffi::c_int;
    ie = 0 as ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    colend = (*mat).col_end.offset(1 as ::core::ffi::c_int as isize);
    while j <= (*mat).columns {
        i = ie;
        ie = *colend;
        rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
        value = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
        while i < ie {
            if *rownr < 0 as ::core::ffi::c_int
                || dozeros as ::core::ffi::c_int != 0 && fabs(*value) < (*mat).epsvalue
            {
                nn += 1;
            } else {
                if ii != i {
                    *(*mat).col_mat_colnr.offset(ii as isize) =
                        *(*mat).col_mat_colnr.offset(i as isize);
                    *(*mat).col_mat_rownr.offset(ii as isize) =
                        *(*mat).col_mat_rownr.offset(i as isize);
                    *(*mat).col_mat_value.offset(ii as isize) =
                        *(*mat).col_mat_value.offset(i as isize);
                }
                ii += 1;
            }
            i += 1;
            rownr = rownr.offset(matRowColStep as isize);
            value = value.offset(matValueStep as isize);
        }
        *colend = ii;
        j += 1;
        colend = colend.offset(1);
    }
    return nn;
}
#[export_name="honest_lpsolve_mat_colcompact"]
pub unsafe extern "C" fn mat_colcompact(
    mut mat: *mut MATrec,
    mut prev_rows: ::core::ffi::c_int,
    mut prev_cols: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n_del: ::core::ffi::c_int = 0;
    let mut n_sum: ::core::ffi::c_int = 0;
    let mut colend: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut newcolend: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut newcolnr: ::core::ffi::c_int = 0;
    let mut deleted: ::core::ffi::c_uchar = 0;
    let mut lp: *mut lprec = (*mat).lp;
    let mut lpundo: *mut presolveundorec = (*lp).presolve_undo;
    n_sum = 0 as ::core::ffi::c_int;
    k = 0 as ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    newcolnr = 1 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    newcolend = (*mat).col_end.offset(1 as ::core::ffi::c_int as isize);
    colend = newcolend;
    while j <= prev_cols {
        n_del = 0 as ::core::ffi::c_int;
        i = k;
        k = *colend;
        colnr = (*mat).col_mat_colnr.offset(i as isize) as *mut ::core::ffi::c_int;
        while i < k {
            if *colnr < 0 as ::core::ffi::c_int {
                n_del += 1;
                n_sum += 1;
            } else {
                if ii < i {
                    *(*mat).col_mat_colnr.offset(ii as isize) =
                        *(*mat).col_mat_colnr.offset(i as isize);
                    *(*mat).col_mat_rownr.offset(ii as isize) =
                        *(*mat).col_mat_rownr.offset(i as isize);
                    *(*mat).col_mat_value.offset(ii as isize) =
                        *(*mat).col_mat_value.offset(i as isize);
                }
                if newcolnr < j {
                    *(*mat).col_mat_colnr.offset(ii as isize) = newcolnr;
                }
                ii += 1;
            }
            i += 1;
            colnr = colnr.offset(matRowColStep as isize);
        }
        *newcolend = ii;
        deleted = (n_del > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        deleted = (deleted as ::core::ffi::c_int
            | ((*lp).wasPresolved == 0
                && *(*lpundo).var_to_orig.offset((prev_rows + j) as isize)
                    < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uchar as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        if deleted == 0 {
            newcolend = newcolend.offset(1);
            newcolnr += 1;
        }
        j += 1;
        colend = colend.offset(1);
    }
    return n_sum;
}
#[export_name="honest_lpsolve_mat_shiftcols"]
pub unsafe extern "C" fn mat_shiftcols(
    mut mat: *mut MATrec,
    mut bbase: *mut ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut varmap: *mut LLrec,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut base: ::core::ffi::c_int = 0;
    k = 0 as ::core::ffi::c_int;
    if delta == 0 as ::core::ffi::c_int {
        return k;
    }
    base = abs(*bbase);
    if delta > 0 as ::core::ffi::c_int {
        ii = (*mat).columns;
        while ii > base {
            i = ii + delta;
            *(*mat).col_end.offset(i as isize) = *(*mat).col_end.offset(ii as isize);
            ii -= 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < delta {
            ii = base + i;
            *(*mat).col_end.offset(ii as isize) = *(*mat)
                .col_end
                .offset((ii - 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    } else {
        let mut preparecompact: ::core::ffi::c_uchar =
            (varmap != NULL as *mut LLrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if preparecompact != 0 {
            let mut j: ::core::ffi::c_int = 0;
            let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
            let mut colend: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
            n = 0 as ::core::ffi::c_int;
            k = 0 as ::core::ffi::c_int;
            base = 0 as ::core::ffi::c_int;
            j = 1 as ::core::ffi::c_int;
            colend = (*mat).col_end.offset(1 as ::core::ffi::c_int as isize);
            while j <= (*mat).columns {
                i = k;
                k = *colend;
                if isActiveLink(varmap, j) != 0 {
                    base += 1;
                    ii = base;
                } else {
                    ii = -(1 as ::core::ffi::c_int);
                }
                if ii < 0 as ::core::ffi::c_int {
                    n += k - i;
                }
                colnr = (*mat).col_mat_colnr.offset(i as isize) as *mut ::core::ffi::c_int;
                while i < k {
                    *colnr = ii;
                    i += 1;
                    colnr = colnr.offset(matRowColStep as isize);
                }
                j += 1;
                colend = colend.offset(1);
            }
            return n;
        }
        preparecompact =
            (*bbase < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if preparecompact != 0 {
            *bbase = if fabs(*bbase as ::core::ffi::c_double)
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int
            } else {
                -*bbase
            };
        }
        if base - delta - 1 as ::core::ffi::c_int > (*mat).columns {
            delta = base - (*mat).columns - 1 as ::core::ffi::c_int;
        }
        if preparecompact != 0 {
            let mut colnr_0: *mut ::core::ffi::c_int =
                ::core::ptr::null_mut::<::core::ffi::c_int>();
            n = 0 as ::core::ffi::c_int;
            i = *(*mat)
                .col_end
                .offset((base - 1 as ::core::ffi::c_int) as isize);
            k = *(*mat)
                .col_end
                .offset((base - delta - 1 as ::core::ffi::c_int) as isize);
            colnr_0 = (*mat).col_mat_colnr.offset(i as isize) as *mut ::core::ffi::c_int;
            while i < k {
                n += 1;
                *colnr_0 = -(1 as ::core::ffi::c_int);
                i += 1;
                colnr_0 = colnr_0.offset(matRowColStep as isize);
            }
            k = n;
        } else if base <= (*mat).columns {
            i = *(*mat)
                .col_end
                .offset((base - 1 as ::core::ffi::c_int) as isize);
            ii = *(*mat)
                .col_end
                .offset((base - delta - 1 as ::core::ffi::c_int) as isize);
            n = mat_nonzeros(mat);
            k = ii - i;
            if k > 0 as ::core::ffi::c_int && n > i {
                n -= ii;
                memmove(
                    (*mat).col_mat_colnr.offset(i as isize) as *mut ::core::ffi::c_int
                        as *mut ::core::ffi::c_void,
                    (*mat).col_mat_colnr.offset(ii as isize) as *mut ::core::ffi::c_int
                        as *const ::core::ffi::c_void,
                    (n as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                );
                memmove(
                    (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int
                        as *mut ::core::ffi::c_void,
                    (*mat).col_mat_rownr.offset(ii as isize) as *mut ::core::ffi::c_int
                        as *const ::core::ffi::c_void,
                    (n as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                );
                memmove(
                    (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_void,
                    (*mat).col_mat_value.offset(ii as isize) as *mut ::core::ffi::c_double
                        as *const ::core::ffi::c_void,
                    (n as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
            }
            i = base;
            while i <= (*mat).columns + delta {
                ii = i - delta;
                *(*mat).col_end.offset(i as isize) = *(*mat).col_end.offset(ii as isize) - k;
                i += 1;
            }
        }
    }
    return k;
}
#[export_name="honest_lpsolve_mat_extractmat"]
pub unsafe extern "C" fn mat_extractmat(
    mut mat: *mut MATrec,
    mut rowmap: *mut LLrec,
    mut colmap: *mut LLrec,
    mut negated: ::core::ffi::c_uchar,
) -> *mut MATrec {
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut xa: ::core::ffi::c_int = 0;
    let mut na: ::core::ffi::c_int = 0;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut newmat: *mut MATrec =
        mat_create((*mat).lp, (*mat).rows, (*mat).columns, (*mat).epsvalue);
    na = mat_nonzeros(mat);
    rownr = (*mat)
        .col_mat_rownr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    colnr = (*mat)
        .col_mat_colnr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    value = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    xa = 0 as ::core::ffi::c_int;
    while xa < na {
        if isActiveLink(colmap, *colnr) as ::core::ffi::c_int ^ negated as ::core::ffi::c_int != 0
            && isActiveLink(rowmap, *rownr) as ::core::ffi::c_int ^ negated as ::core::ffi::c_int
                != 0
        {
            mat_setvalue(
                newmat,
                *rownr,
                *colnr,
                *value,
                FALSE as ::core::ffi::c_uchar,
            );
        }
        xa += 1;
        rownr = rownr.offset(matRowColStep as isize);
        colnr = colnr.offset(matRowColStep as isize);
        value = value.offset(matValueStep as isize);
    }
    return newmat;
}
#[export_name="honest_lpsolve_mat_setcol"]
pub unsafe extern "C" fn mat_setcol(
    mut mat: *mut MATrec,
    mut colno: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut rowno: *mut ::core::ffi::c_int,
    mut doscale: ::core::ffi::c_uchar,
    mut checkrowmode: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut orignr: ::core::ffi::c_int = 0;
    let mut newnr: ::core::ffi::c_int = 0;
    let mut firstrow: ::core::ffi::c_int = 0;
    let mut addto: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut isA: ::core::ffi::c_uchar = 0;
    let mut isNZ: ::core::ffi::c_uchar = 0;
    let mut value: ::core::ffi::c_double = 0.;
    let mut saved: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut lp: *mut lprec = (*mat).lp;
    if checkrowmode as ::core::ffi::c_int != 0 && (*mat).is_roworder as ::core::ffi::c_int != 0 {
        return mat_setrow(
            mat,
            colno,
            count,
            column,
            rowno,
            doscale,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    isA = (mat == (*(*mat).lp).matA) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    isNZ = (rowno != NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if isNZ == 0 {
        count = (*(*mat).lp).rows;
    } else if count < 0 as ::core::ffi::c_int
        || count
            > (*mat).rows
                + (if (*mat).is_roworder as ::core::ffi::c_int != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                })
    {
        return 0 as ::core::ffi::c_uchar;
    }
    if isNZ as ::core::ffi::c_int != 0 && count > 0 as ::core::ffi::c_int {
        if count > 1 as ::core::ffi::c_int {
            sortREALByINT(
                column,
                rowno,
                count,
                0 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            );
        }
        if *rowno.offset(0 as ::core::ffi::c_int as isize) < 0 as ::core::ffi::c_int
            || *rowno.offset((count - 1 as ::core::ffi::c_int) as isize) > (*mat).rows
        {
            return 0 as ::core::ffi::c_uchar;
        }
    }
    if isA as ::core::ffi::c_int != 0 && (*mat).is_roworder == 0 {
        if isNZ as ::core::ffi::c_int != 0
            && count > 0 as ::core::ffi::c_int
            && *rowno.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            value = *column.offset(0 as ::core::ffi::c_int as isize);
            value = roundToPrecision(value, (*mat).epsvalue);
            if doscale != 0 {
                value = scaled_mat(lp, value, 0 as ::core::ffi::c_int, colno);
            }
            value = if is_maxim(lp) as ::core::ffi::c_int != 0
                && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -value
            } else {
                value
            };
            *(*lp).orig_obj.offset(colno as isize) = value;
            count -= 1;
            column = column.offset(1);
            rowno = rowno.offset(1);
        } else if isNZ == 0
            && *column.offset(0 as ::core::ffi::c_int as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            saved = *column.offset(0 as ::core::ffi::c_int as isize);
            value = saved;
            value = roundToPrecision(value, (*mat).epsvalue);
            if doscale != 0 {
                value = scaled_mat(lp, value, 0 as ::core::ffi::c_int, colno);
            }
            value = if is_maxim(lp) as ::core::ffi::c_int != 0
                && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -value
            } else {
                value
            };
            *(*lp).orig_obj.offset(colno as isize) = value;
            *column.offset(0 as ::core::ffi::c_int as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *(*lp).orig_obj.offset(colno as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    firstrow = (*mat).rows + 1 as ::core::ffi::c_int;
    if isNZ != 0 {
        newnr = count;
        if newnr != 0 {
            firstrow = *rowno.offset(0 as ::core::ffi::c_int as isize);
            jj = *rowno.offset((newnr - 1 as ::core::ffi::c_int) as isize);
        }
    } else {
        newnr = 0 as ::core::ffi::c_int;
        if allocMYBOOL(
            lp,
            &raw mut addto,
            (*mat).rows + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0
        {
            return 0 as ::core::ffi::c_uchar;
        }
        i = (*mat).rows;
        while i >= 0 as ::core::ffi::c_int {
            if fabs(*column.offset(i as isize)) > (*mat).epsvalue {
                *addto.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
                firstrow = i;
                newnr += 1;
            }
            i -= 1;
        }
    }
    if inc_mat_space(mat, newnr) == 0 {
        newnr = 0 as ::core::ffi::c_int;
    } else {
        orignr = mat_collength(mat, colno);
        elmnr = newnr - orignr;
        i = mat_nonzeros(mat) - *(*mat).col_end.offset(colno as isize);
        if elmnr != 0 as ::core::ffi::c_int && i > 0 as ::core::ffi::c_int {
            memmove(
                (*mat)
                    .col_mat_colnr
                    .offset((*(*mat).col_end.offset(colno as isize) + elmnr) as isize)
                    as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                (*mat)
                    .col_mat_colnr
                    .offset(*(*mat).col_end.offset(colno as isize) as isize)
                    as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
                (i as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            memmove(
                (*mat)
                    .col_mat_rownr
                    .offset((*(*mat).col_end.offset(colno as isize) + elmnr) as isize)
                    as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                (*mat)
                    .col_mat_rownr
                    .offset(*(*mat).col_end.offset(colno as isize) as isize)
                    as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
                (i as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            memmove(
                (*mat)
                    .col_mat_value
                    .offset((*(*mat).col_end.offset(colno as isize) + elmnr) as isize)
                    as *mut ::core::ffi::c_double as *mut ::core::ffi::c_void,
                (*mat)
                    .col_mat_value
                    .offset(*(*mat).col_end.offset(colno as isize) as isize)
                    as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                (i as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
        if elmnr != 0 as ::core::ffi::c_int {
            i = colno;
            while i <= (*mat).columns {
                *(*mat).col_end.offset(i as isize) += elmnr;
                i += 1;
            }
        }
        jj = *(*mat)
            .col_end
            .offset((colno - 1 as ::core::ffi::c_int) as isize);
        if isNZ != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < count {
                value = *column.offset(i as isize);
                value = roundToPrecision(value, (*mat).epsvalue);
                if (*mat).is_roworder != 0 {
                    if isA as ::core::ffi::c_int != 0 && doscale as ::core::ffi::c_int != 0 {
                        value = scaled_mat(lp, value, colno, *rowno.offset(i as isize));
                    }
                    if isA != 0 {
                        value = if is_chsign(lp, colno) as ::core::ffi::c_int != 0
                            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -value
                        } else {
                            value
                        };
                    }
                } else {
                    if isA as ::core::ffi::c_int != 0 && doscale as ::core::ffi::c_int != 0 {
                        value = scaled_mat(lp, value, *rowno.offset(i as isize), colno);
                    }
                    if isA != 0 {
                        value = if is_chsign(lp, *rowno.offset(i as isize)) as ::core::ffi::c_int
                            != 0
                            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -value
                        } else {
                            value
                        };
                    }
                }
                *(*mat).col_mat_rownr.offset(jj as isize) = *rowno.offset(i as isize);
                *(*mat).col_mat_colnr.offset(jj as isize) = colno;
                *(*mat).col_mat_value.offset(jj as isize) = value;
                jj += 1;
                i += 1;
            }
        } else {
            i = firstrow;
            while i <= (*mat).rows {
                if !(*addto.offset(i as isize) == 0) {
                    value = *column.offset(i as isize);
                    value = roundToPrecision(value, (*mat).epsvalue);
                    if (*mat).is_roworder != 0 {
                        if isA as ::core::ffi::c_int != 0 && doscale as ::core::ffi::c_int != 0 {
                            value = scaled_mat(lp, value, colno, i);
                        }
                        if isA != 0 {
                            value = if is_chsign(lp, colno) as ::core::ffi::c_int != 0
                                && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                -value
                            } else {
                                value
                            };
                        }
                    } else {
                        if isA as ::core::ffi::c_int != 0 && doscale as ::core::ffi::c_int != 0 {
                            value = scaled_mat(lp, value, i, colno);
                        }
                        if isA != 0 {
                            value = if is_chsign(lp, i) as ::core::ffi::c_int != 0
                                && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                -value
                            } else {
                                value
                            };
                        }
                    }
                    *(*mat).col_mat_rownr.offset(jj as isize) = i;
                    *(*mat).col_mat_colnr.offset(jj as isize) = colno;
                    *(*mat).col_mat_value.offset(jj as isize) = value;
                    jj += 1;
                }
                i += 1;
            }
        }
        (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    }
    if saved != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *column.offset(0 as ::core::ffi::c_int as isize) = saved;
    }
    if !(addto as *mut ::core::ffi::c_void).is_null() {
        free(addto as *mut ::core::ffi::c_void);
        addto = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_mergemat"]
pub unsafe extern "C" fn mat_mergemat(
    mut target: *mut MATrec,
    mut source: *mut MATrec,
    mut usecolmap: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*target).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut colmap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colvalue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if (*target).rows < (*source).rows
        || allocREAL(
            lp,
            &raw mut colvalue,
            (*target).rows + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    if usecolmap != 0 {
        n = *(*source).col_tag.offset(0 as ::core::ffi::c_int as isize);
        allocINT(
            lp,
            &raw mut colmap,
            n + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            *colmap.offset(i as isize) = i;
            i += 1;
        }
        hpsortex(
            (*source).col_tag as *mut ::core::ffi::c_void,
            n,
            1 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
            Some(
                compareINT
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            colmap,
        );
    } else {
        n = (*source).columns;
    }
    let mut current_block_16: u64;
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        if !(usecolmap == 0 && mat_collength(source, i) == 0 as ::core::ffi::c_int) {
            if usecolmap != 0 {
                ix = *colmap.offset(i as isize);
                if ix <= 0 as ::core::ffi::c_int {
                    current_block_16 = 13109137661213826276;
                } else {
                    iy = *(*source).col_tag.offset(i as isize);
                    if iy <= 0 as ::core::ffi::c_int {
                        current_block_16 = 13109137661213826276;
                    } else {
                        current_block_16 = 8457315219000651999;
                    }
                }
            } else {
                iy = i;
                ix = iy;
                current_block_16 = 8457315219000651999;
            }
            match current_block_16 {
                13109137661213826276 => {}
                _ => {
                    mat_expandcolumn(
                        source,
                        ix,
                        colvalue,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        FALSE as ::core::ffi::c_uchar,
                    );
                    mat_setcol(
                        target,
                        iy,
                        0 as ::core::ffi::c_int,
                        colvalue,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        FALSE as ::core::ffi::c_uchar,
                        FALSE as ::core::ffi::c_uchar,
                    );
                }
            }
        }
        i += 1;
    }
    if !(colvalue as *mut ::core::ffi::c_void).is_null() {
        free(colvalue as *mut ::core::ffi::c_void);
        colvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(colmap as *mut ::core::ffi::c_void).is_null() {
        free(colmap as *mut ::core::ffi::c_void);
        colmap = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_nz_unused"]
pub unsafe extern "C" fn mat_nz_unused(mut mat: *mut MATrec) -> ::core::ffi::c_int {
    return (*mat).mat_alloc - *(*mat).col_end.offset((*mat).columns as isize);
}
#[export_name="honest_lpsolve_mat_setrow"]
pub unsafe extern "C" fn mat_setrow(
    mut mat: *mut MATrec,
    mut rowno: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
    mut doscale: ::core::ffi::c_uchar,
    mut checkrowmode: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*mat).lp;
    let mut delta: ::core::ffi::c_int = 0;
    let mut delta1: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jj_j: ::core::ffi::c_int = 0;
    let mut lendense: ::core::ffi::c_int = 0;
    let mut origidx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut newidx: ::core::ffi::c_int = 0;
    let mut orignz: ::core::ffi::c_int = 0;
    let mut newnz: ::core::ffi::c_int = 0;
    let mut rownr: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut colnr1: ::core::ffi::c_int = 0;
    let mut isA: ::core::ffi::c_uchar = 0;
    let mut isNZ: ::core::ffi::c_uchar = 0;
    let mut value: ::core::ffi::c_double = 0.0f64;
    if checkrowmode as ::core::ffi::c_int != 0 && (*mat).is_roworder as ::core::ffi::c_int != 0 {
        return mat_setcol(
            mat,
            rowno,
            count,
            row,
            colno,
            doscale,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    if mat_validate(mat) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    isA = (mat == (*lp).matA) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if doscale as ::core::ffi::c_int != 0
        && isA as ::core::ffi::c_int != 0
        && (*lp).scaling_used == 0
    {
        doscale = FALSE as ::core::ffi::c_uchar;
    }
    isNZ = (colno != NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    lendense = if (*mat).is_roworder as ::core::ffi::c_int != 0 {
        (*lp).rows
    } else {
        (*lp).columns
    };
    if count < 0 as ::core::ffi::c_int || count > lendense {
        return 0 as ::core::ffi::c_uchar;
    }
    colnr1 = lendense + 1 as ::core::ffi::c_int;
    if isA as ::core::ffi::c_int != 0 && (*mat).is_roworder as ::core::ffi::c_int != 0 {
        *(*lp).orig_obj.offset(rowno as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        if count > 0 as ::core::ffi::c_int
            && *colno.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            value = *row.offset(0 as ::core::ffi::c_int as isize);
            if doscale != 0 {
                value = scaled_mat(lp, value, 0 as ::core::ffi::c_int, rowno);
            }
            value = if is_maxim(lp) as ::core::ffi::c_int != 0
                && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -value
            } else {
                value
            };
            value = roundToPrecision(value, (*mat).epsvalue);
            *(*lp).orig_obj.offset(rowno as isize) = value;
            if isNZ != 0 {
                colno = colno.offset(1);
                row = row.offset(1);
                count -= 1;
            }
        } else {
            *(*lp).orig_obj.offset(rowno as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
            value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    if isNZ == 0 {
        let mut tmprow: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        if allocINT(
            lp,
            &raw mut colno,
            lendense + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) == 0
        {
            return 0 as ::core::ffi::c_uchar;
        }
        newnz = 0 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i <= lendense {
            value = *row.offset(i as isize);
            if value != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                if tmprow.is_null()
                    && allocREAL(
                        lp,
                        &raw mut tmprow,
                        lendense - i + 1 as ::core::ffi::c_int,
                        FALSE as ::core::ffi::c_uchar,
                    ) == 0
                {
                    if !(colno as *mut ::core::ffi::c_void).is_null() {
                        free(colno as *mut ::core::ffi::c_void);
                        colno = ::core::ptr::null_mut::<::core::ffi::c_int>();
                    }
                    return 0 as ::core::ffi::c_uchar;
                }
                *tmprow.offset(newnz as isize) = value;
                let fresh5 = newnz;
                newnz = newnz + 1;
                *colno.offset(fresh5 as isize) = i;
            }
            i += 1;
        }
        count = newnz;
        row = tmprow;
    } else {
        let mut tmpcolno: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        if allocINT(
            lp,
            &raw mut tmpcolno,
            lendense,
            FALSE as ::core::ffi::c_uchar,
        ) == 0
        {
            return 0 as ::core::ffi::c_uchar;
        }
        newnz = count;
        memcpy(
            tmpcolno as *mut ::core::ffi::c_void,
            colno as *const ::core::ffi::c_void,
            (newnz as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        colno = tmpcolno;
        if newnz > 1 as ::core::ffi::c_int {
            sortREALByINT(
                row,
                colno,
                newnz,
                0 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            );
        }
        if newnz > 0 as ::core::ffi::c_int
            && (*colno.offset(0 as ::core::ffi::c_int as isize) < 0 as ::core::ffi::c_int
                || *colno.offset((newnz - 1 as ::core::ffi::c_int) as isize) > lendense)
        {
            if !(colno as *mut ::core::ffi::c_void).is_null() {
                free(colno as *mut ::core::ffi::c_void);
                colno = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
            newnz = 0 as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_uchar;
        }
    }
    i = *(*mat)
        .row_end
        .offset((rowno - 1 as ::core::ffi::c_int) as isize);
    ii = *(*mat).row_end.offset(rowno as isize);
    delta = count - (ii - i);
    delta1 = delta;
    colnr1 = if newnz > 0 as ::core::ffi::c_int {
        *colno.offset(0 as ::core::ffi::c_int as isize)
    } else {
        lendense + 1 as ::core::ffi::c_int
    };
    orignz = mat_nonzeros(mat);
    j = if i >= orignz {
        colnr1
    } else {
        *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(i as isize) as isize)
    };
    origidx = *(*mat)
        .col_end
        .offset((colnr1 - 1 as ::core::ffi::c_int) as isize);
    colnr = if origidx >= orignz {
        colnr1
    } else {
        *(*mat).col_mat_colnr.offset(origidx as isize)
    };
    if j < colnr {
        newidx = *(*mat)
            .col_end
            .offset((j - 1 as ::core::ffi::c_int) as isize);
        origidx = newidx;
        while j < colnr {
            jj_j = *(*mat).col_end.offset(j as isize);
            while origidx < jj_j {
                if *(*mat).col_mat_rownr.offset(origidx as isize) != rowno {
                    if newidx != origidx {
                        *(*mat).col_mat_colnr.offset(newidx as isize) =
                            *(*mat).col_mat_colnr.offset(origidx as isize);
                        *(*mat).col_mat_rownr.offset(newidx as isize) =
                            *(*mat).col_mat_rownr.offset(origidx as isize);
                        *(*mat).col_mat_value.offset(newidx as isize) =
                            *(*mat).col_mat_value.offset(origidx as isize);
                    }
                    newidx += 1;
                }
                origidx += 1;
            }
            *(*mat).col_end.offset(j as isize) = newidx;
            j += 1;
        }
        delta = newidx - origidx;
    } else {
        delta = 0 as ::core::ffi::c_int;
        newidx = origidx;
    }
    jj_j = if 0 as ::core::ffi::c_int > newnz + delta {
        0 as ::core::ffi::c_int
    } else {
        newnz + delta
    };
    j = (!(orignz == lendense && newnz == orignz && delta1 == 0 as ::core::ffi::c_int)
        && jj_j > 0 as ::core::ffi::c_int
        && orignz > origidx) as ::core::ffi::c_int;
    if j != 0 && jj_j > delta1 {
        delta1 = jj_j;
    }
    if delta1 > 0 as ::core::ffi::c_int
        && mat_nz_unused(mat) <= delta1
        && inc_mat_space(mat, delta1) == 0
    {
        newnz = 0 as ::core::ffi::c_int;
    } else {
        if j != 0 {
            memmove(
                (*mat).col_mat_colnr.offset((origidx + jj_j) as isize) as *mut ::core::ffi::c_int
                    as *mut ::core::ffi::c_void,
                (*mat).col_mat_colnr.offset(origidx as isize) as *mut ::core::ffi::c_int
                    as *const ::core::ffi::c_void,
                ((orignz - origidx) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            memmove(
                (*mat).col_mat_rownr.offset((origidx + jj_j) as isize) as *mut ::core::ffi::c_int
                    as *mut ::core::ffi::c_void,
                (*mat).col_mat_rownr.offset(origidx as isize) as *mut ::core::ffi::c_int
                    as *const ::core::ffi::c_void,
                ((orignz - origidx) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            memmove(
                (*mat).col_mat_value.offset((origidx + jj_j) as isize) as *mut ::core::ffi::c_double
                    as *mut ::core::ffi::c_void,
                (*mat).col_mat_value.offset(origidx as isize) as *mut ::core::ffi::c_double
                    as *const ::core::ffi::c_void,
                ((orignz - origidx) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            origidx += jj_j;
            orignz += jj_j;
        }
        newnz = 0 as ::core::ffi::c_int;
        j = origidx
            - *(*mat)
                .col_end
                .offset((colnr1 - 1 as ::core::ffi::c_int) as isize);
        k = colnr1;
        let mut current_block_151: u64;
        while colnr1 <= lendense || origidx < orignz {
            if newnz < count {
                colnr1 = *colno.offset(newnz as isize);
            } else {
                colnr1 = lendense + 1 as ::core::ffi::c_int;
            }
            if origidx < orignz {
                rownr = *(*mat).col_mat_rownr.offset(origidx as isize);
                colnr = *(*mat).col_mat_colnr.offset(origidx as isize);
            } else {
                if colnr1 > lendense {
                    break;
                }
                rownr = rowno;
                colnr = lendense + 1 as ::core::ffi::c_int;
            }
            jj_j = origidx - j;
            i = if colnr < colnr1 { colnr } else { colnr1 };
            while k < i {
                *(*mat).col_end.offset(k as isize) = jj_j;
                k += 1;
            }
            if colnr1 > colnr {
                if rownr == rowno {
                    current_block_151 = 14743764238938021888;
                } else {
                    current_block_151 = 14652688882591975137;
                }
            } else if colnr > colnr1 || colnr == colnr1 && rownr >= rowno {
                value = *row.offset(newnz as isize);
                newnz += 1;
                if isA as ::core::ffi::c_int != 0 && doscale as ::core::ffi::c_int != 0 {
                    value = scaled_mat(lp, value, rowno, colnr1);
                }
                if isA != 0 {
                    value = if is_chsign(lp, rowno) as ::core::ffi::c_int != 0
                        && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -value
                    } else {
                        value
                    };
                }
                value = roundToPrecision(value, (*mat).epsvalue);
                if value == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    if colnr > colnr1 || rownr > rowno {
                        current_block_151 = 10109057886293123569;
                    } else {
                        current_block_151 = 14743764238938021888;
                    }
                } else {
                    current_block_151 = 10109057886293123569;
                }
                match current_block_151 {
                    14743764238938021888 => {}
                    _ => {
                        *(*mat).col_mat_rownr.offset(jj_j as isize) = rowno;
                        *(*mat).col_mat_colnr.offset(jj_j as isize) = colnr1;
                        *(*mat).col_mat_value.offset(jj_j as isize) = value;
                        if colnr > colnr1 || rownr > rowno {
                            j -= 1;
                            origidx -= 1;
                            jj_j += 1;
                            delta += 1;
                        }
                        origidx += 1;
                        continue;
                    }
                }
            } else {
                current_block_151 = 14652688882591975137;
            }
            match current_block_151 {
                14652688882591975137 => {
                    if jj_j != origidx {
                        *(*mat).col_mat_colnr.offset(jj_j as isize) =
                            *(*mat).col_mat_colnr.offset(origidx as isize);
                        *(*mat).col_mat_rownr.offset(jj_j as isize) =
                            *(*mat).col_mat_rownr.offset(origidx as isize);
                        *(*mat).col_mat_value.offset(jj_j as isize) =
                            *(*mat).col_mat_value.offset(origidx as isize);
                    }
                    origidx += 1;
                }
                _ => {
                    j += 1;
                    delta -= 1;
                    origidx += 1;
                }
            }
        }
        jj_j = origidx - j;
        while k <= lendense {
            *(*mat).col_end.offset(k as isize) = jj_j;
            k += 1;
        }
        (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    }
    if isNZ == 0 {
        if !(row as *mut ::core::ffi::c_void).is_null() {
            free(row as *mut ::core::ffi::c_void);
            row = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    }
    if !(colno as *mut ::core::ffi::c_void).is_null() {
        free(colno as *mut ::core::ffi::c_void);
        colno = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return (newnz > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_appendrow"]
pub unsafe extern "C" fn mat_appendrow(
    mut mat: *mut MATrec,
    mut count: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
    mut mult: ::core::ffi::c_double,
    mut checkrowmode: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stcol: ::core::ffi::c_int = 0;
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut orignr: ::core::ffi::c_int = 0;
    let mut newnr: ::core::ffi::c_int = 0;
    let mut firstcol: ::core::ffi::c_int = 0;
    let mut addto: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut isA: ::core::ffi::c_uchar = 0;
    let mut isNZ: ::core::ffi::c_uchar = 0;
    let mut value: ::core::ffi::c_double = 0.;
    let mut saved: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut lp: *mut lprec = (*mat).lp;
    if checkrowmode as ::core::ffi::c_int != 0 && (*mat).is_roworder as ::core::ffi::c_int != 0 {
        return mat_appendcol(mat, count, row, colno, mult, FALSE as ::core::ffi::c_uchar);
    }
    isA = (mat == (*lp).matA) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    isNZ = (colno != NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if isNZ as ::core::ffi::c_int != 0 && count > 0 as ::core::ffi::c_int {
        if count > 1 as ::core::ffi::c_int {
            sortREALByINT(
                row,
                colno,
                count,
                0 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            );
        }
        if *colno.offset(0 as ::core::ffi::c_int as isize) < 1 as ::core::ffi::c_int
            || *colno.offset((count - 1 as ::core::ffi::c_int) as isize) > (*mat).columns
        {
            return 0 as ::core::ffi::c_int;
        }
    } else if isNZ == 0 && !row.is_null() && (*mat).is_roworder == 0 {
        *row.offset(0 as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if isA as ::core::ffi::c_int != 0 && (*mat).is_roworder as ::core::ffi::c_int != 0 {
        if isNZ as ::core::ffi::c_int != 0
            && *colno.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            value = *row.offset(0 as ::core::ffi::c_int as isize);
            value = roundToPrecision(value, (*mat).epsvalue);
            value = scaled_mat(lp, value, 0 as ::core::ffi::c_int, (*lp).columns);
            value = if is_maxim(lp) as ::core::ffi::c_int != 0
                && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -value
            } else {
                value
            };
            *(*lp).orig_obj.offset((*lp).columns as isize) = value;
            count -= 1;
            row = row.offset(1);
            colno = colno.offset(1);
        } else if isNZ == 0
            && !row.is_null()
            && *row.offset(0 as ::core::ffi::c_int as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            saved = *row.offset(0 as ::core::ffi::c_int as isize);
            value = saved;
            value = roundToPrecision(value, (*mat).epsvalue);
            value = scaled_mat(lp, value, 0 as ::core::ffi::c_int, (*lp).columns);
            value = if is_maxim(lp) as ::core::ffi::c_int != 0
                && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -value
            } else {
                value
            };
            *(*lp).orig_obj.offset((*lp).columns as isize) = value;
            *row.offset(0 as ::core::ffi::c_int as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *(*lp).orig_obj.offset((*lp).columns as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    firstcol = (*mat).columns + 1 as ::core::ffi::c_int;
    if isNZ != 0 {
        newnr = count;
        if newnr != 0 {
            firstcol = *colno.offset(0 as ::core::ffi::c_int as isize);
            jj = *colno.offset((newnr - 1 as ::core::ffi::c_int) as isize);
        }
    } else {
        newnr = 0 as ::core::ffi::c_int;
        if !row.is_null() {
            if allocMYBOOL(
                lp,
                &raw mut addto,
                (*mat).columns + 1 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            ) == 0
            {
                return newnr;
            }
            i = (*mat).columns;
            while i >= 1 as ::core::ffi::c_int {
                if fabs(*row.offset(i as isize)) > (*mat).epsvalue {
                    *addto.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
                    firstcol = i;
                    newnr += 1;
                }
                i -= 1;
            }
        }
    }
    if inc_mat_space(mat, newnr) == 0 {
        newnr = 0 as ::core::ffi::c_int;
    } else {
        orignr = mat_nonzeros(mat) - 1 as ::core::ffi::c_int;
        elmnr = orignr + newnr;
        j = (*mat).columns;
        while j >= firstcol {
            stcol = *(*mat).col_end.offset(j as isize) - 1 as ::core::ffi::c_int;
            *(*mat).col_end.offset(j as isize) = elmnr + 1 as ::core::ffi::c_int;
            if isNZ as ::core::ffi::c_int != 0 && j == jj
                || !addto.is_null() && *addto.offset(j as isize) as ::core::ffi::c_int != 0
            {
                newnr -= 1;
                if isNZ != 0 {
                    value = *row.offset(newnr as isize);
                    if newnr != 0 {
                        jj = *colno.offset((newnr - 1 as ::core::ffi::c_int) as isize);
                    } else {
                        jj = 0 as ::core::ffi::c_int;
                    }
                } else {
                    value = *row.offset(j as isize);
                }
                value = roundToPrecision(value, (*mat).epsvalue);
                value *= mult;
                if isA != 0 {
                    if (*mat).is_roworder != 0 {
                        value = if is_chsign(lp, j) as ::core::ffi::c_int != 0
                            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -value
                        } else {
                            value
                        };
                    }
                    value = scaled_mat(lp, value, (*mat).rows, j);
                }
                *(*mat).col_mat_rownr.offset(elmnr as isize) = (*mat).rows;
                *(*mat).col_mat_colnr.offset(elmnr as isize) = j;
                *(*mat).col_mat_value.offset(elmnr as isize) = value;
                elmnr -= 1;
            }
            i = stcol
                - *(*mat)
                    .col_end
                    .offset((j - 1 as ::core::ffi::c_int) as isize)
                + 1 as ::core::ffi::c_int;
            if i > 0 as ::core::ffi::c_int {
                orignr -= i;
                elmnr -= i;
                memmove(
                    (*mat)
                        .col_mat_colnr
                        .offset((elmnr + 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                    (*mat)
                        .col_mat_colnr
                        .offset((orignr + 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_int
                        as *const ::core::ffi::c_void,
                    (i as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                );
                memmove(
                    (*mat)
                        .col_mat_rownr
                        .offset((elmnr + 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                    (*mat)
                        .col_mat_rownr
                        .offset((orignr + 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_int
                        as *const ::core::ffi::c_void,
                    (i as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                );
                memmove(
                    (*mat)
                        .col_mat_value
                        .offset((elmnr + 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_void,
                    (*mat)
                        .col_mat_value
                        .offset((orignr + 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_double
                        as *const ::core::ffi::c_void,
                    (i as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
            }
            j -= 1;
        }
    }
    if saved != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *row.offset(0 as ::core::ffi::c_int as isize) = saved;
    }
    if !(addto as *mut ::core::ffi::c_void).is_null() {
        free(addto as *mut ::core::ffi::c_void);
        addto = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return newnr;
}
#[export_name="honest_lpsolve_mat_appendcol"]
pub unsafe extern "C" fn mat_appendcol(
    mut mat: *mut MATrec,
    mut count: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut rowno: *mut ::core::ffi::c_int,
    mut mult: ::core::ffi::c_double,
    mut checkrowmode: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut lastnr: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0.;
    let mut isA: ::core::ffi::c_uchar = 0;
    let mut isNZ: ::core::ffi::c_uchar = 0;
    let mut lp: *mut lprec = (*mat).lp;
    if checkrowmode as ::core::ffi::c_int != 0 && (*mat).is_roworder as ::core::ffi::c_int != 0 {
        return mat_appendrow(
            mat,
            count,
            column,
            rowno,
            mult,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    if column.is_null() {
        i = 0 as ::core::ffi::c_int;
    } else if !rowno.is_null() {
        i = count;
    } else {
        let mut nrows: ::core::ffi::c_int = (*mat).rows;
        elmnr = 0 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i <= nrows {
            if *column.offset(i as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                elmnr += 1;
            }
            i += 1;
        }
        i = elmnr;
    }
    if mat_nz_unused(mat) <= i && inc_mat_space(mat, i) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    isA = (mat == (*lp).matA) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    isNZ = (column.is_null() || !rowno.is_null()) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if isNZ as ::core::ffi::c_int != 0 && count > 0 as ::core::ffi::c_int {
        if count > 1 as ::core::ffi::c_int {
            sortREALByINT(
                column,
                rowno,
                count,
                0 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            );
        }
        if *rowno.offset(0 as ::core::ffi::c_int as isize) < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if !rowno.is_null() {
        count -= 1;
    }
    elmnr = *(*mat)
        .col_end
        .offset(((*mat).columns - 1 as ::core::ffi::c_int) as isize);
    if !column.is_null() {
        row = -(1 as ::core::ffi::c_int);
        let mut current_block_42: u64;
        i = if isNZ as ::core::ffi::c_int != 0 || (*mat).is_roworder == 0 {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        while i <= count {
            value = *column.offset(i as isize);
            if fabs(value) > (*mat).epsvalue {
                if isNZ != 0 {
                    lastnr = row;
                    row = *rowno.offset(i as isize);
                    if row > (*mat).rows {
                        break;
                    }
                    if row <= lastnr {
                        return -(1 as ::core::ffi::c_int);
                    }
                } else {
                    row = i;
                }
                value = roundToPrecision(value, (*mat).epsvalue);
                if (*mat).is_roworder != 0 {
                    value *= mult;
                    current_block_42 = 14072441030219150333;
                } else if isA != 0 {
                    value = if is_chsign(lp, row) as ::core::ffi::c_int != 0
                        && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -value
                    } else {
                        value
                    };
                    value = scaled_mat(lp, value, row, (*mat).columns);
                    if (*mat).is_roworder == 0 && row == 0 as ::core::ffi::c_int {
                        *(*lp).orig_obj.offset((*mat).columns as isize) = value;
                        current_block_42 = 10043043949733653460;
                    } else {
                        current_block_42 = 14072441030219150333;
                    }
                } else {
                    current_block_42 = 14072441030219150333;
                }
                match current_block_42 {
                    10043043949733653460 => {}
                    _ => {
                        *(*mat).col_mat_rownr.offset(elmnr as isize) = row;
                        *(*mat).col_mat_colnr.offset(elmnr as isize) = (*mat).columns;
                        *(*mat).col_mat_value.offset(elmnr as isize) = value;
                        elmnr += 1;
                    }
                }
            }
            i += 1;
        }
        if get_Lrows(lp) > 0 as ::core::ffi::c_int {
            mat_appendcol(
                (*lp).matL,
                get_Lrows(lp),
                column.offset((*mat).rows as isize),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                mult,
                checkrowmode,
            );
        }
    }
    *(*mat).col_end.offset((*mat).columns as isize) = elmnr;
    return *(*mat).col_end.offset((*mat).columns as isize)
        - *(*mat)
            .col_end
            .offset(((*mat).columns - 1 as ::core::ffi::c_int) as isize);
}
#[export_name="honest_lpsolve_mat_checkcounts"]
pub unsafe extern "C" fn mat_checkcounts(
    mut mat: *mut MATrec,
    mut rownum: *mut ::core::ffi::c_int,
    mut colnum: *mut ::core::ffi::c_int,
    mut freeonexit: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if rownum.is_null() {
        allocINT(
            (*mat).lp,
            &raw mut rownum,
            (*mat).rows + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    if colnum.is_null() {
        allocINT(
            (*mat).lp,
            &raw mut colnum,
            (*mat).columns + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*mat).columns {
        j = *(*mat)
            .col_end
            .offset((i - 1 as ::core::ffi::c_int) as isize);
        n = *(*mat).col_end.offset(i as isize);
        rownr = (*mat).col_mat_rownr.offset(j as isize) as *mut ::core::ffi::c_int;
        while j < n {
            let ref mut fresh6 = *colnum.offset(i as isize);
            *fresh6 += 1;
            let ref mut fresh7 = *rownum.offset(*rownr as isize);
            *fresh7 += 1;
            j += 1;
            rownr = rownr.offset(matRowColStep as isize);
        }
        i += 1;
    }
    n = 0 as ::core::ffi::c_int;
    if (*(*mat).lp).do_presolve != PRESOLVE_NONE
        && ((*(*mat).lp).spx_trace as ::core::ffi::c_int != 0 || (*(*mat).lp).verbose > NORMAL)
    {
        j = 1 as ::core::ffi::c_int;
        while j <= (*mat).columns {
            if *colnum.offset(j as isize) == 0 as ::core::ffi::c_int {
                n += 1;
                report(
                    (*mat).lp,
                    6 as ::core::ffi::c_int,
                    b"mat_checkcounts: Variable %s is not used in any constraints\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            j += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i <= (*mat).rows {
            if *rownum.offset(i as isize) == 0 as ::core::ffi::c_int {
                n += 1;
                report(
                    (*mat).lp,
                    6 as ::core::ffi::c_int,
                    b"mat_checkcounts: Constraint %s empty\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            i += 1;
        }
    }
    if freeonexit != 0 {
        if !(rownum as *mut ::core::ffi::c_void).is_null() {
            free(rownum as *mut ::core::ffi::c_void);
            rownum = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !(colnum as *mut ::core::ffi::c_void).is_null() {
            free(colnum as *mut ::core::ffi::c_void);
            colnum = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    return n;
}
#[export_name="honest_lpsolve_mat_validate"]
pub unsafe extern "C" fn mat_validate(mut mat: *mut MATrec) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut rownum: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if (*mat).row_end_valid == 0 {
        memset(
            (*mat).row_end as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*mat).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        allocINT(
            (*mat).lp,
            &raw mut rownum,
            (*mat).rows + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
        j = mat_nonzeros(mat);
        rownr = (*mat)
            .col_mat_rownr
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < j {
            let ref mut fresh3 = *(*mat).row_end.offset(*rownr as isize);
            *fresh3 += 1;
            i += 1;
            rownr = rownr.offset(matRowColStep as isize);
        }
        i = 1 as ::core::ffi::c_int;
        while i <= (*mat).rows {
            *(*mat).row_end.offset(i as isize) += *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= (*mat).columns {
            j = *(*mat)
                .col_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            je = *(*mat).col_end.offset(i as isize);
            rownr = (*mat).col_mat_rownr.offset(j as isize) as *mut ::core::ffi::c_int;
            colnr = (*mat).col_mat_colnr.offset(j as isize) as *mut ::core::ffi::c_int;
            while j < je {
                *colnr = i;
                if *rownr == 0 as ::core::ffi::c_int {
                    mat_set_rowmap(mat, *rownum.offset(*rownr as isize), *rownr, i, j);
                } else {
                    mat_set_rowmap(
                        mat,
                        *(*mat)
                            .row_end
                            .offset((*rownr - 1 as ::core::ffi::c_int) as isize)
                            + *rownum.offset(*rownr as isize),
                        *rownr,
                        i,
                        j,
                    );
                }
                let ref mut fresh4 = *rownum.offset(*rownr as isize);
                *fresh4 += 1;
                j += 1;
                rownr = rownr.offset(matRowColStep as isize);
                colnr = colnr.offset(matRowColStep as isize);
            }
            i += 1;
        }
        if !(rownum as *mut ::core::ffi::c_void).is_null() {
            free(rownum as *mut ::core::ffi::c_void);
            rownum = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        (*mat).row_end_valid = TRUE as ::core::ffi::c_uchar;
    }
    if mat == (*(*mat).lp).matA {
        (*(*mat).lp).model_is_valid = TRUE as ::core::ffi::c_uchar;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_get_data"]
pub unsafe extern "C" fn mat_get_data(
    mut lp: *mut lprec,
    mut matindex: ::core::ffi::c_int,
    mut isrow: ::core::ffi::c_uchar,
    mut rownr: *mut *mut ::core::ffi::c_int,
    mut colnr: *mut *mut ::core::ffi::c_int,
    mut value: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut mat: *mut MATrec = (*lp).matA;
    if isrow != 0 {
        matindex = *(*mat).row_mat.offset(matindex as isize);
    }
    if !rownr.is_null() {
        *rownr = (*mat).col_mat_rownr.offset(matindex as isize) as *mut ::core::ffi::c_int;
    }
    if !colnr.is_null() {
        *colnr = (*mat).col_mat_colnr.offset(matindex as isize) as *mut ::core::ffi::c_int;
    }
    if !value.is_null() {
        *value = (*mat).col_mat_value.offset(matindex as isize) as *mut ::core::ffi::c_double;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_set_rowmap"]
pub unsafe extern "C" fn mat_set_rowmap(
    mut mat: *mut MATrec,
    mut row_mat_index: ::core::ffi::c_int,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
    mut col_mat_index: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    *(*mat).row_mat.offset(row_mat_index as isize) = col_mat_index;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_findelm"]
pub unsafe extern "C" fn mat_findelm(
    mut mat: *mut MATrec,
    mut row: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut low: ::core::ffi::c_int = 0;
    let mut high: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    if column < 1 as ::core::ffi::c_int || column > (*mat).columns {
        report(
            (*mat).lp,
            3 as ::core::ffi::c_int,
            b"mat_findelm: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if row < 0 as ::core::ffi::c_int || row > (*mat).rows {
        report(
            (*mat).lp,
            3 as ::core::ffi::c_int,
            b"mat_findelm: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    low = *(*mat)
        .col_end
        .offset((column - 1 as ::core::ffi::c_int) as isize);
    high = *(*mat).col_end.offset(column as isize) - 1 as ::core::ffi::c_int;
    if low > high {
        return -(2 as ::core::ffi::c_int);
    }
    mid = (low + high) / 2 as ::core::ffi::c_int;
    item = *(*mat).col_mat_rownr.offset(mid as isize);
    while high - low > LINEARSEARCH {
        if item < row {
            low = mid + 1 as ::core::ffi::c_int;
            mid = (low + high) / 2 as ::core::ffi::c_int;
            item = *(*mat).col_mat_rownr.offset(mid as isize);
        } else if item > row {
            high = mid - 1 as ::core::ffi::c_int;
            mid = (low + high) / 2 as ::core::ffi::c_int;
            item = *(*mat).col_mat_rownr.offset(mid as isize);
        } else {
            low = mid;
            high = mid;
        }
    }
    if high > low && high - low <= LINEARSEARCH {
        item = *(*mat).col_mat_rownr.offset(low as isize);
        while low < high && item < row {
            low += 1;
            item = *(*mat).col_mat_rownr.offset(low as isize);
        }
        if item == row {
            high = low;
        }
    }
    if low == high && row == item {
        return low;
    } else {
        return -(2 as ::core::ffi::c_int);
    };
}
#[export_name="honest_lpsolve_mat_findins"]
pub unsafe extern "C" fn mat_findins(
    mut mat: *mut MATrec,
    mut row: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
    mut insertpos: *mut ::core::ffi::c_int,
    mut validate: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut low: ::core::ffi::c_int = 0;
    let mut high: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut exitvalue: ::core::ffi::c_int = 0;
    let mut insvalue: ::core::ffi::c_int = 0;
    insvalue = -(1 as ::core::ffi::c_int);
    if column < 1 as ::core::ffi::c_int || column > (*mat).columns {
        if column > 0 as ::core::ffi::c_int && validate == 0 {
            insvalue = *(*mat).col_end.offset((*mat).columns as isize);
            exitvalue = -(2 as ::core::ffi::c_int);
        } else {
            report(
                (*mat).lp,
                3 as ::core::ffi::c_int,
                b"mat_findins: Column %d out of range\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            exitvalue = -(1 as ::core::ffi::c_int);
        }
    } else if row < 0 as ::core::ffi::c_int || row > (*mat).rows {
        if row >= 0 as ::core::ffi::c_int && validate == 0 {
            insvalue = *(*mat).col_end.offset(column as isize);
            exitvalue = -(2 as ::core::ffi::c_int);
        } else {
            report(
                (*mat).lp,
                3 as ::core::ffi::c_int,
                b"mat_findins: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            exitvalue = -(1 as ::core::ffi::c_int);
        }
    } else {
        low = *(*mat)
            .col_end
            .offset((column - 1 as ::core::ffi::c_int) as isize);
        insvalue = low;
        high = *(*mat).col_end.offset(column as isize) - 1 as ::core::ffi::c_int;
        if low > high {
            exitvalue = -(2 as ::core::ffi::c_int);
        } else {
            mid = (low + high) / 2 as ::core::ffi::c_int;
            item = *(*mat).col_mat_rownr.offset(mid as isize);
            while high - low > LINEARSEARCH {
                if item < row {
                    low = mid + 1 as ::core::ffi::c_int;
                    mid = (low + high) / 2 as ::core::ffi::c_int;
                    item = *(*mat).col_mat_rownr.offset(mid as isize);
                } else if item > row {
                    high = mid - 1 as ::core::ffi::c_int;
                    mid = (low + high) / 2 as ::core::ffi::c_int;
                    item = *(*mat).col_mat_rownr.offset(mid as isize);
                } else {
                    low = mid;
                    high = mid;
                }
            }
            if high > low && high - low <= LINEARSEARCH {
                item = *(*mat).col_mat_rownr.offset(low as isize);
                while low < high && item < row {
                    low += 1;
                    item = *(*mat).col_mat_rownr.offset(low as isize);
                }
                if item == row {
                    high = low;
                }
            }
            insvalue = low;
            if low == high && row == item {
                exitvalue = low;
            } else {
                if low < *(*mat).col_end.offset(column as isize)
                    && *(*mat).col_mat_rownr.offset(low as isize) < row
                {
                    insvalue += 1;
                }
                exitvalue = -(2 as ::core::ffi::c_int);
            }
        }
    }
    if !insertpos.is_null() {
        *insertpos = insvalue;
    }
    return exitvalue;
}
#[export_name="honest_lpsolve_mat_getitem"]
pub unsafe extern "C" fn mat_getitem(
    mut mat: *mut MATrec,
    mut row: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut elmnr: ::core::ffi::c_int = 0;
    elmnr = mat_findelm(mat, row, column);
    if elmnr >= 0 as ::core::ffi::c_int {
        return *(*mat).col_mat_value.offset(elmnr as isize);
    } else {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    };
}
#[export_name="honest_lpsolve_mat_additem"]
pub unsafe extern "C" fn mat_additem(
    mut mat: *mut MATrec,
    mut row: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut elmnr: ::core::ffi::c_int = 0;
    elmnr = mat_findelm(mat, row, column);
    if elmnr >= 0 as ::core::ffi::c_int {
        *(*mat).col_mat_value.offset(elmnr as isize) += delta;
        return 1 as ::core::ffi::c_uchar;
    } else {
        mat_setitem(mat, row, column, delta);
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_mat_setitem"]
pub unsafe extern "C" fn mat_setitem(
    mut mat: *mut MATrec,
    mut row: ::core::ffi::c_int,
    mut column: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return mat_setvalue(mat, row, column, value, FALSE as ::core::ffi::c_uchar);
}
#[export_name="honest_lpsolve_mat_multrow"]
pub unsafe extern "C" fn mat_multrow(
    mut mat: *mut MATrec,
    mut row_nr: ::core::ffi::c_int,
    mut mult: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k1: ::core::ffi::c_int = 0;
    let mut k2: ::core::ffi::c_int = 0;
    if mat_validate(mat) != 0 {
        if row_nr == 0 as ::core::ffi::c_int {
            k1 = 0 as ::core::ffi::c_int;
        } else {
            k1 = *(*mat)
                .row_end
                .offset((row_nr - 1 as ::core::ffi::c_int) as isize);
        }
        k2 = *(*mat).row_end.offset(row_nr as isize);
        i = k1;
        while i < k2 {
            *(*mat)
                .col_mat_value
                .offset(*(*mat).row_mat.offset(i as isize) as isize) *= mult;
            i += 1;
        }
    }
}
#[export_name="honest_lpsolve_mat_multcol"]
pub unsafe extern "C" fn mat_multcol(
    mut mat: *mut MATrec,
    mut col_nr: ::core::ffi::c_int,
    mut mult: ::core::ffi::c_double,
    mut DoObj: ::core::ffi::c_uchar,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut isA: ::core::ffi::c_uchar = 0;
    if mult == 1.0f64 {
        return;
    }
    isA = (mat == (*(*mat).lp).matA) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    ie = *(*mat).col_end.offset(col_nr as isize);
    i = *(*mat)
        .col_end
        .offset((col_nr - 1 as ::core::ffi::c_int) as isize);
    while i < ie {
        *(*mat).col_mat_value.offset(i as isize) *= mult;
        i += 1;
    }
    if isA != 0 {
        if DoObj != 0 {
            *(*(*mat).lp).orig_obj.offset(col_nr as isize) *= mult;
        }
        if get_Lrows((*mat).lp) > 0 as ::core::ffi::c_int {
            mat_multcol((*(*mat).lp).matL, col_nr, mult, DoObj);
        }
    }
}
#[export_name="honest_lpsolve_mat_multadd"]
pub unsafe extern "C" fn mat_multadd(
    mut mat: *mut MATrec,
    mut lhsvector: *mut ::core::ffi::c_double,
    mut varnr: ::core::ffi::c_int,
    mut mult: ::core::ffi::c_double,
) {
    let mut colnr: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if varnr <= (*(*mat).lp).rows {
        *lhsvector.offset(varnr as isize) += mult;
        return;
    }
    if (*(*mat).lp).matA == mat {
        *lhsvector.offset(0 as ::core::ffi::c_int as isize) +=
            get_OF_active((*mat).lp, varnr, mult);
    }
    colnr = varnr - (*(*mat).lp).rows;
    ib = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    ie = *(*mat).col_end.offset(colnr as isize);
    if ib < ie {
        matRownr = (*mat).col_mat_rownr.offset(ib as isize) as *mut ::core::ffi::c_int;
        matValue = (*mat).col_mat_value.offset(ib as isize) as *mut ::core::ffi::c_double;
        while ib < ie {
            *lhsvector.offset(*matRownr as isize) += mult * *matValue;
            ib += 1;
            matValue = matValue.offset(matValueStep as isize);
            matRownr = matRownr.offset(matRowColStep as isize);
        }
    }
}
#[export_name="honest_lpsolve_mat_setvalue"]
pub unsafe extern "C" fn mat_setvalue(
    mut mat: *mut MATrec,
    mut Row: ::core::ffi::c_int,
    mut Column: ::core::ffi::c_int,
    mut Value: ::core::ffi::c_double,
    mut doscale: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut lastelm: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut RowA: ::core::ffi::c_int = Row;
    let mut ColumnA: ::core::ffi::c_int = Column;
    let mut isA: ::core::ffi::c_uchar = 0;
    isA = (mat == (*(*mat).lp).matA) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if (*mat).is_roworder != 0 {
        swapINT(&raw mut Row, &raw mut Column);
    }
    if fabs(Value) < (*mat).epsvalue {
        Value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        Value = roundToPrecision(Value, (*mat).epsvalue);
    }
    if Column > (*mat).columns {
        if isA != 0 {
            inc_col_space((*mat).lp, ColumnA - (*mat).columns);
        } else {
            inc_matcol_space(mat, Column - (*mat).columns);
        }
    }
    i = mat_findins(
        mat,
        Row,
        Column,
        &raw mut elmnr,
        FALSE as ::core::ffi::c_uchar,
    );
    if i == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_uchar;
    }
    if isA != 0 {
        set_action(
            &raw mut (*(*mat).lp).spx_action,
            ACTION_REBASE | ACTION_RECOMPUTE | ACTION_REINVERT,
        );
    }
    if i >= 0 as ::core::ffi::c_int {
        if fabs(Value) > (*mat).epsvalue {
            if isA != 0 {
                Value = if is_chsign((*mat).lp, RowA) as ::core::ffi::c_int != 0
                    && Value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -Value
                } else {
                    Value
                };
                if doscale as ::core::ffi::c_int != 0
                    && (*(*mat).lp).scaling_used as ::core::ffi::c_int != 0
                {
                    Value = scaled_mat((*mat).lp, Value, RowA, ColumnA);
                }
            }
            *(*mat).col_mat_value.offset(elmnr as isize) = Value;
        } else {
            lastelm = mat_nonzeros(mat);
            lastelm -= elmnr;
            memmove(
                (*mat).col_mat_colnr.offset(elmnr as isize) as *mut ::core::ffi::c_int
                    as *mut ::core::ffi::c_void,
                (*mat)
                    .col_mat_colnr
                    .offset((elmnr + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
                (lastelm as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            memmove(
                (*mat).col_mat_rownr.offset(elmnr as isize) as *mut ::core::ffi::c_int
                    as *mut ::core::ffi::c_void,
                (*mat)
                    .col_mat_rownr
                    .offset((elmnr + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
                (lastelm as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            memmove(
                (*mat).col_mat_value.offset(elmnr as isize) as *mut ::core::ffi::c_double
                    as *mut ::core::ffi::c_void,
                (*mat)
                    .col_mat_value
                    .offset((elmnr + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                (lastelm as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            i = Column;
            while i <= (*mat).columns {
                let ref mut fresh1 = *(*mat).col_end.offset(i as isize);
                *fresh1 -= 1;
                i += 1;
            }
            (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
        }
    } else if fabs(Value) > (*mat).epsvalue {
        if inc_mat_space(mat, 1 as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_uchar;
        }
        if Column > (*mat).columns {
            i = (*mat).columns + 1 as ::core::ffi::c_int;
            if isA != 0 {
                shift_coldata(
                    (*mat).lp,
                    i,
                    ColumnA - (*mat).columns,
                    ::core::ptr::null_mut::<LLrec>(),
                );
            } else {
                mat_shiftcols(
                    mat,
                    &raw mut i,
                    Column - (*mat).columns,
                    ::core::ptr::null_mut::<LLrec>(),
                );
            }
        }
        lastelm = mat_nonzeros(mat);
        i = lastelm;
        while i > elmnr {
            *(*mat).col_mat_colnr.offset(i as isize) = *(*mat)
                .col_mat_colnr
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            *(*mat).col_mat_rownr.offset(i as isize) = *(*mat)
                .col_mat_rownr
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            *(*mat).col_mat_value.offset(i as isize) = *(*mat)
                .col_mat_value
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            i -= 1;
        }
        if isA != 0 {
            Value = if is_chsign((*mat).lp, RowA) as ::core::ffi::c_int != 0
                && Value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -Value
            } else {
                Value
            };
            if doscale != 0 {
                Value = scaled_mat((*mat).lp, Value, RowA, ColumnA);
            }
        }
        *(*mat).col_mat_rownr.offset(elmnr as isize) = Row;
        *(*mat).col_mat_colnr.offset(elmnr as isize) = Column;
        *(*mat).col_mat_value.offset(elmnr as isize) = Value;
        i = Column;
        while i <= (*mat).columns {
            let ref mut fresh2 = *(*mat).col_end.offset(i as isize);
            *fresh2 += 1;
            i += 1;
        }
        (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    }
    if isA as ::core::ffi::c_int != 0
        && !(*(*mat).lp).var_is_free.is_null()
        && *(*(*mat).lp).var_is_free.offset(ColumnA as isize) > 0 as ::core::ffi::c_int
    {
        return mat_setvalue(
            mat,
            RowA,
            *(*(*mat).lp).var_is_free.offset(ColumnA as isize),
            -Value,
            doscale,
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_appendvalue"]
pub unsafe extern "C" fn mat_appendvalue(
    mut mat: *mut MATrec,
    mut Row: ::core::ffi::c_int,
    mut Value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut elmnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Column: ::core::ffi::c_int = (*mat).columns;
    if fabs(Value) < (*mat).epsvalue {
        Value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        Value = roundToPrecision(Value, (*mat).epsvalue);
    }
    if inc_mat_space(mat, 1 as ::core::ffi::c_int) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    elmnr = (*mat).col_end.offset(Column as isize);
    *(*mat).col_mat_rownr.offset(*elmnr as isize) = Row;
    *(*mat).col_mat_colnr.offset(*elmnr as isize) = Column;
    *(*mat).col_mat_value.offset(*elmnr as isize) = Value;
    *elmnr += 1;
    (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_equalRows"]
pub unsafe extern "C" fn mat_equalRows(
    mut mat: *mut MATrec,
    mut baserow: ::core::ffi::c_int,
    mut comprow: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if mat_validate(mat) != 0 {
        let mut bj1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ej1: ::core::ffi::c_int = 0;
        let mut bj2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ej2: ::core::ffi::c_int = 0;
        if baserow >= 0 as ::core::ffi::c_int {
            bj1 = *(*mat)
                .row_end
                .offset((baserow - 1 as ::core::ffi::c_int) as isize);
        }
        ej1 = *(*mat).row_end.offset(baserow as isize);
        if comprow >= 0 as ::core::ffi::c_int {
            bj2 = *(*mat)
                .row_end
                .offset((comprow - 1 as ::core::ffi::c_int) as isize);
        }
        ej2 = *(*mat).row_end.offset(comprow as isize);
        if ej1 - bj1 != ej2 - bj2 {
            return status;
        }
        while bj1 < ej1 {
            if *(*mat).col_mat_colnr.offset(bj1 as isize)
                != *(*mat).col_mat_colnr.offset(bj2 as isize)
            {
                break;
            }
            if fabs(
                get_mat_byindex(
                    (*mat).lp,
                    bj1,
                    TRUE as ::core::ffi::c_uchar,
                    FALSE as ::core::ffi::c_uchar,
                ) - get_mat_byindex(
                    (*mat).lp,
                    bj2,
                    TRUE as ::core::ffi::c_uchar,
                    FALSE as ::core::ffi::c_uchar,
                ),
            ) > (*(*mat).lp).epsprimal
            {
                break;
            }
            bj1 += 1;
            bj2 += 1;
        }
        status = (bj1 == ej1) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    return status;
}
#[export_name="honest_lpsolve_mat_findcolumn"]
pub unsafe extern "C" fn mat_findcolumn(
    mut mat: *mut MATrec,
    mut matindex: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    j = 1 as ::core::ffi::c_int;
    while j <= (*mat).columns {
        if matindex < *(*mat).col_end.offset(j as isize) {
            break;
        }
        j += 1;
    }
    return j;
}
#[export_name="honest_lpsolve_mat_expandcolumn"]
pub unsafe extern "C" fn mat_expandcolumn(
    mut mat: *mut MATrec,
    mut colnr: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut nzlist: *mut ::core::ffi::c_int,
    mut signedA: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut isA: ::core::ffi::c_uchar =
        ((*(*mat).lp).matA == mat) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nzcount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    signedA = (signedA as ::core::ffi::c_int & isA as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    memset(
        column as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*mat).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    if isA != 0 {
        *column.offset(0 as ::core::ffi::c_int as isize) =
            *(*(*mat).lp).orig_obj.offset(colnr as isize);
        if signedA as ::core::ffi::c_int != 0
            && is_chsign((*mat).lp, 0 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        {
            *column.offset(0 as ::core::ffi::c_int as isize) =
                -*column.offset(0 as ::core::ffi::c_int as isize);
        }
    }
    i = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    ie = *(*mat).col_end.offset(colnr as isize);
    matRownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
    matValue = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
    while i < ie {
        j = *matRownr;
        *column.offset(j as isize) = *matValue;
        if signedA as ::core::ffi::c_int != 0 && is_chsign((*mat).lp, j) as ::core::ffi::c_int != 0
        {
            *column.offset(j as isize) = -*column.offset(j as isize);
        }
        nzcount += 1;
        if !nzlist.is_null() {
            *nzlist.offset(nzcount as isize) = j;
        }
        i += 1;
        matRownr = matRownr.offset(matRowColStep as isize);
        matValue = matValue.offset(matValueStep as isize);
    }
    if !nzlist.is_null() {
        *nzlist.offset(0 as ::core::ffi::c_int as isize) = nzcount;
    }
    return nzcount;
}
#[export_name="honest_lpsolve_mat_computemax"]
pub unsafe extern "C" fn mat_computemax(mut mat: *mut MATrec) -> ::core::ffi::c_uchar {
    let mut rownr: *mut ::core::ffi::c_int = (*mat)
        .col_mat_rownr
        .offset(0 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_int;
    let mut colnr: *mut ::core::ffi::c_int = (*mat)
        .col_mat_colnr
        .offset(0 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ie: ::core::ffi::c_int = *(*mat).col_end.offset((*mat).columns as isize);
    let mut ez: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut value: *mut ::core::ffi::c_double = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_double;
    let mut epsmachine: ::core::ffi::c_double = (*(*mat).lp).epsmachine;
    let mut absvalue: ::core::ffi::c_double = 0.;
    if allocREAL(
        (*mat).lp,
        &raw mut (*mat).colmax,
        (*mat).columns_alloc + 1 as ::core::ffi::c_int,
        AUTOMATIC as ::core::ffi::c_uchar,
    ) == 0
        || allocREAL(
            (*mat).lp,
            &raw mut (*mat).rowmax,
            (*mat).rows_alloc + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    memset(
        (*mat).colmax as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*mat).columns + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    memset(
        (*mat).rowmax as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*mat).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    (*mat).dynrange = (*(*mat).lp).infinite;
    while i < ie {
        absvalue = fabs(*value);
        if *(*mat).colmax.offset(*colnr as isize) < absvalue {
            *(*mat).colmax.offset(*colnr as isize) = absvalue;
        }
        if *(*mat).rowmax.offset(*rownr as isize) < absvalue {
            *(*mat).rowmax.offset(*rownr as isize) = absvalue;
        }
        if (*mat).dynrange > absvalue {
            (*mat).dynrange = absvalue;
        }
        if absvalue < epsmachine {
            ez += 1;
        }
        i += 1;
        rownr = rownr.offset(matRowColStep as isize);
        colnr = colnr.offset(matRowColStep as isize);
        value = value.offset(matValueStep as isize);
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*mat).rows {
        if *(*mat).rowmax.offset(0 as ::core::ffi::c_int as isize)
            < *(*mat).rowmax.offset(i as isize)
        {
            *(*mat).rowmax.offset(0 as ::core::ffi::c_int as isize) =
                *(*mat).rowmax.offset(i as isize);
        }
        i += 1;
    }
    let ref mut fresh8 = *(*mat).colmax.offset(0 as ::core::ffi::c_int as isize);
    *fresh8 = *(*mat).rowmax.offset(0 as ::core::ffi::c_int as isize);
    (*mat).infnorm = *fresh8;
    if (*mat).dynrange == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        report(
            (*mat).lp,
            2 as ::core::ffi::c_int,
            b"%d matrix contains zero-valued coefficients.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        (*mat).dynrange = (*(*mat).lp).infinite;
    } else {
        (*mat).dynrange = (*mat).infnorm / (*mat).dynrange;
        if ez > 0 as ::core::ffi::c_int {
            report(
                (*mat).lp,
                3 as ::core::ffi::c_int,
                b"%d matrix coefficients below machine precision were found.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_mat_transpose"]
pub unsafe extern "C" fn mat_transpose(mut mat: *mut MATrec) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_uchar = 0;
    status = mat_validate(mat);
    if status != 0 {
        nz = mat_nonzeros(mat);
        if nz > 0 as ::core::ffi::c_int {
            let mut newValue: *mut ::core::ffi::c_double =
                ::core::ptr::null_mut::<::core::ffi::c_double>();
            let mut newRownr: *mut ::core::ffi::c_int =
                ::core::ptr::null_mut::<::core::ffi::c_int>();
            allocREAL(
                (*mat).lp,
                &raw mut newValue,
                (*mat).mat_alloc,
                FALSE as ::core::ffi::c_uchar,
            );
            allocINT(
                (*mat).lp,
                &raw mut newRownr,
                (*mat).mat_alloc,
                FALSE as ::core::ffi::c_uchar,
            );
            j = *(*mat).row_end.offset(0 as ::core::ffi::c_int as isize);
            i = nz - 1 as ::core::ffi::c_int;
            while i >= j {
                k = i - j;
                *newValue.offset(k as isize) = *(*mat)
                    .col_mat_value
                    .offset(*(*mat).row_mat.offset(i as isize) as isize);
                *newRownr.offset(k as isize) = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(i as isize) as isize);
                i -= 1;
            }
            i = j - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                k = nz - j + i;
                *newValue.offset(k as isize) = *(*mat)
                    .col_mat_value
                    .offset(*(*mat).row_mat.offset(i as isize) as isize);
                *newRownr.offset(k as isize) = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(i as isize) as isize);
                i -= 1;
            }
            swapPTR(
                &raw mut (*mat).col_mat_rownr as *mut *mut ::core::ffi::c_void,
                &raw mut newRownr as *mut *mut ::core::ffi::c_void,
            );
            swapPTR(
                &raw mut (*mat).col_mat_value as *mut *mut ::core::ffi::c_void,
                &raw mut newValue as *mut *mut ::core::ffi::c_void,
            );
            if !(newValue as *mut ::core::ffi::c_void).is_null() {
                free(newValue as *mut ::core::ffi::c_void);
                newValue = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(newRownr as *mut ::core::ffi::c_void).is_null() {
                free(newRownr as *mut ::core::ffi::c_void);
                newRownr = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
        }
        if (*mat).rows == (*mat).rows_alloc {
            inc_matcol_space(mat, 1 as ::core::ffi::c_int);
        }
        j = *(*mat).row_end.offset(0 as ::core::ffi::c_int as isize);
        i = (*mat).rows;
        while i >= 1 as ::core::ffi::c_int {
            *(*mat).row_end.offset(i as isize) -= j;
            i -= 1;
        }
        *(*mat).row_end.offset((*mat).rows as isize) = nz;
        swapPTR(
            &raw mut (*mat).row_end as *mut *mut ::core::ffi::c_void,
            &raw mut (*mat).col_end as *mut *mut ::core::ffi::c_void,
        );
        swapPTR(
            &raw mut (*mat).rowmax as *mut *mut ::core::ffi::c_void,
            &raw mut (*mat).colmax as *mut *mut ::core::ffi::c_void,
        );
        swapINT(&raw mut (*mat).rows, &raw mut (*mat).columns);
        swapINT(&raw mut (*mat).rows_alloc, &raw mut (*mat).columns_alloc);
        (*mat).is_roworder =
            ((*mat).is_roworder == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
    }
    return status;
}
#[export_name="honest_lpsolve_createUndoLadder"]
pub unsafe extern "C" fn createUndoLadder(
    mut lp: *mut lprec,
    mut levelitems: ::core::ffi::c_int,
    mut maxlevels: ::core::ffi::c_int,
) -> *mut DeltaVrec {
    let mut hold: *mut DeltaVrec = ::core::ptr::null_mut::<DeltaVrec>();
    hold = malloc(::core::mem::size_of::<DeltaVrec>() as size_t) as *mut DeltaVrec;
    (*hold).lp = lp;
    (*hold).activelevel = 0 as ::core::ffi::c_int;
    (*hold).tracker = mat_create(lp, levelitems, 0 as ::core::ffi::c_int, 0.0f64);
    inc_matcol_space((*hold).tracker, maxlevels);
    return hold;
}
#[export_name="honest_lpsolve_incrementUndoLadder"]
pub unsafe extern "C" fn incrementUndoLadder(mut DV: *mut DeltaVrec) -> ::core::ffi::c_int {
    (*DV).activelevel += 1;
    inc_matcol_space((*DV).tracker, 1 as ::core::ffi::c_int);
    mat_shiftcols(
        (*DV).tracker,
        &raw mut (*DV).activelevel,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<LLrec>(),
    );
    (*(*DV).tracker).columns += 1;
    return (*DV).activelevel;
}
#[export_name="honest_lpsolve_modifyUndoLadder"]
pub unsafe extern "C" fn modifyUndoLadder(
    mut DV: *mut DeltaVrec,
    mut itemno: ::core::ffi::c_int,
    mut target: *mut ::core::ffi::c_double,
    mut newvalue: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = 0;
    let mut varindex: ::core::ffi::c_int = itemno;
    let mut oldvalue: ::core::ffi::c_double = *target.offset(itemno as isize);
    varindex -= (*(*DV).lp).rows;
    status = mat_appendvalue((*DV).tracker, varindex, oldvalue);
    *target.offset(itemno as isize) = newvalue;
    return status;
}
#[export_name="honest_lpsolve_countsUndoLadder"]
pub unsafe extern "C" fn countsUndoLadder(mut DV: *mut DeltaVrec) -> ::core::ffi::c_int {
    if (*DV).activelevel > 0 as ::core::ffi::c_int {
        return mat_collength((*DV).tracker, (*DV).activelevel);
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_restoreUndoLadder"]
pub unsafe extern "C" fn restoreUndoLadder(
    mut DV: *mut DeltaVrec,
    mut target: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut iD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*DV).activelevel > 0 as ::core::ffi::c_int {
        let mut mat: *mut MATrec = (*DV).tracker;
        let mut iB: ::core::ffi::c_int = *(*mat)
            .col_end
            .offset(((*DV).activelevel - 1 as ::core::ffi::c_int) as isize);
        let mut iE: ::core::ffi::c_int = *(*mat).col_end.offset((*DV).activelevel as isize);
        let mut matRownr: *mut ::core::ffi::c_int =
            (*mat).col_mat_rownr.offset(iB as isize) as *mut ::core::ffi::c_int;
        let mut matValue: *mut ::core::ffi::c_double =
            (*mat).col_mat_value.offset(iB as isize) as *mut ::core::ffi::c_double;
        let mut oldvalue: ::core::ffi::c_double = 0.;
        iD = iE - iB;
        while iB < iE {
            oldvalue = *matValue;
            *target.offset(((*(*DV).lp).rows + *matRownr) as isize) = oldvalue;
            iB += 1;
            matValue = matValue.offset(matValueStep as isize);
            matRownr = matRownr.offset(matRowColStep as isize);
        }
        mat_shiftcols(
            (*DV).tracker,
            &raw mut (*DV).activelevel,
            -(1 as ::core::ffi::c_int),
            ::core::ptr::null_mut::<LLrec>(),
        );
    }
    return iD;
}
#[export_name="honest_lpsolve_decrementUndoLadder"]
pub unsafe extern "C" fn decrementUndoLadder(mut DV: *mut DeltaVrec) -> ::core::ffi::c_int {
    let mut deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*DV).activelevel > 0 as ::core::ffi::c_int {
        deleted = mat_shiftcols(
            (*DV).tracker,
            &raw mut (*DV).activelevel,
            -(1 as ::core::ffi::c_int),
            ::core::ptr::null_mut::<LLrec>(),
        );
        (*DV).activelevel -= 1;
        (*(*DV).tracker).columns -= 1;
    }
    return deleted;
}
#[export_name="honest_lpsolve_freeUndoLadder"]
pub unsafe extern "C" fn freeUndoLadder(mut DV: *mut *mut DeltaVrec) -> ::core::ffi::c_uchar {
    if DV.is_null() || (*DV).is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    mat_free(&raw mut (**DV).tracker);
    if !(*DV as *mut ::core::ffi::c_void).is_null() {
        free(*DV as *mut ::core::ffi::c_void);
        *DV = ::core::ptr::null_mut::<DeltaVrec>();
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_appendUndoPresolve"]
pub unsafe extern "C" fn appendUndoPresolve(
    mut lp: *mut lprec,
    mut isprimal: ::core::ffi::c_uchar,
    mut beta: ::core::ffi::c_double,
    mut colnrDep: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut mat: *mut MATrec = ::core::ptr::null_mut::<MATrec>();
    if isprimal != 0 {
        mat = (*(*(*lp).presolve_undo).primalundo).tracker;
    } else {
        mat = (*(*(*lp).presolve_undo).dualundo).tracker;
    }
    if colnrDep > 0 as ::core::ffi::c_int
        && beta != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && !mat.is_null()
        && *(*mat).col_tag.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int
    {
        let mut ix: ::core::ffi::c_int = *(*mat).col_tag.offset(0 as ::core::ffi::c_int as isize);
        if colnrDep <= (*lp).columns {
            mat_setvalue(mat, colnrDep, ix, beta, FALSE as ::core::ffi::c_uchar);
        } else {
            let mut ipos: ::core::ffi::c_int = 0;
            let mut jx: ::core::ffi::c_int = *(*mat).col_tag.offset(ix as isize);
            mat_setvalue(mat, jx, ix, beta, FALSE as ::core::ffi::c_uchar);
            jx = mat_findins(mat, jx, ix, &raw mut ipos, FALSE as ::core::ffi::c_uchar);
            *(*mat).col_mat_rownr.offset(ipos as isize) = colnrDep;
        }
        return 1 as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_addUndoPresolve"]
pub unsafe extern "C" fn addUndoPresolve(
    mut lp: *mut lprec,
    mut isprimal: ::core::ffi::c_uchar,
    mut colnrElim: ::core::ffi::c_int,
    mut alpha: ::core::ffi::c_double,
    mut beta: ::core::ffi::c_double,
    mut colnrDep: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut ix: ::core::ffi::c_int = 0;
    let mut DV: *mut *mut DeltaVrec = ::core::ptr::null_mut::<*mut DeltaVrec>();
    let mut mat: *mut MATrec = ::core::ptr::null_mut::<MATrec>();
    let mut psdata: *mut presolveundorec = (*lp).presolve_undo;
    if isprimal != 0 {
        DV = &raw mut (*psdata).primalundo;
        if (*DV).is_null() {
            *DV = createUndoLadder(lp, (*lp).columns + 1 as ::core::ffi::c_int, (*lp).columns);
            mat = (**DV).tracker;
            (*mat).epsvalue = (*(*lp).matA).epsvalue;
            allocINT(
                lp,
                &raw mut (*mat).col_tag,
                (*lp).columns + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            *(*mat).col_tag.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
        }
    } else {
        DV = &raw mut (*psdata).dualundo;
        if (*DV).is_null() {
            *DV = createUndoLadder(lp, (*lp).rows + 1 as ::core::ffi::c_int, (*lp).rows);
            mat = (**DV).tracker;
            (*mat).epsvalue = (*(*lp).matA).epsvalue;
            allocINT(
                lp,
                &raw mut (*mat).col_tag,
                (*lp).rows + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            *(*mat).col_tag.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
        }
    }
    mat = (**DV).tracker;
    let ref mut fresh9 = *(*mat).col_tag.offset(0 as ::core::ffi::c_int as isize);
    *fresh9 = incrementUndoLadder(*DV);
    ix = *fresh9;
    *(*mat).col_tag.offset(ix as isize) = colnrElim;
    if alpha != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        mat_setvalue(
            mat,
            0 as ::core::ffi::c_int,
            ix,
            alpha,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    if colnrDep > 0 as ::core::ffi::c_int
        && beta != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if colnrDep > (*lp).columns {
            return appendUndoPresolve(lp, isprimal, beta, colnrDep);
        } else {
            mat_setvalue(mat, colnrDep, ix, beta, FALSE as ::core::ffi::c_uchar);
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_invert"]
pub unsafe extern "C" fn invert(
    mut lp: *mut lprec,
    mut shiftbounds: ::core::ffi::c_uchar,
    mut final_0: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut usedpos: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut resetbasis: ::core::ffi::c_uchar = 0;
    let mut test: ::core::ffi::c_double = 0.;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut singularities: ::core::ffi::c_int = 0;
    let mut usercolB: ::core::ffi::c_int = 0;
    if mat_validate((*lp).matA) == 0 {
        (*lp).spx_status = INFEASIBLE;
        return 0 as ::core::ffi::c_uchar;
    }
    if (*lp).invB.is_null() {
        (*lp).bfp_init.expect("non-null function pointer")(
            lp,
            (*lp).rows,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    } else {
        (*lp)
            .bfp_preparefactorization
            .expect("non-null function pointer")(lp);
    }
    singularities = 0 as ::core::ffi::c_int;
    if userabort(lp, MSG_INVERT) != 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if allocMYBOOL(
        lp,
        &raw mut usedpos,
        (*lp).sum + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
    {
        (*lp).bb_break = TRUE as ::core::ffi::c_uchar;
        return 0 as ::core::ffi::c_uchar;
    }
    *usedpos.offset(0 as ::core::ffi::c_int as isize) = TRUE as ::core::ffi::c_uchar;
    usercolB = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        k = *(*lp).var_basic.offset(i as isize);
        if k > (*lp).rows {
            usercolB += 1;
        }
        *usedpos.offset(k as isize) = TRUE as ::core::ffi::c_uchar;
        i += 1;
    }
    resetbasis = (usercolB > 0 as ::core::ffi::c_int
        && (*lp).bfp_canresetbasis.expect("non-null function pointer")(lp) as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    k = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if *(*lp).var_basic.offset(i as isize) > (*lp).rows {
            k += mat_collength((*lp).matA, *(*lp).var_basic.offset(i as isize) - (*lp).rows)
                + (if is_OF_nz(lp, *(*lp).var_basic.offset(i as isize) - (*lp).rows)
                    as ::core::ffi::c_int
                    != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                });
        }
        if resetbasis != 0 {
            j = *(*lp).var_basic.offset(i as isize);
            if j > (*lp).rows {
                *(*lp).is_basic.offset(j as isize) = FALSE as ::core::ffi::c_uchar;
            }
            *(*lp).var_basic.offset(i as isize) = i;
            *(*lp).is_basic.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        }
        i += 1;
    }
    singularities =
        (*lp).bfp_factorize.expect("non-null function pointer")(lp, usercolB, k, usedpos, final_0);
    if !(userabort(lp, MSG_INVERT) != 0) {
        (*lp)
            .bfp_finishfactorization
            .expect("non-null function pointer")(lp);
        recompute_solution(lp, shiftbounds);
        restartPricer(lp, AUTOMATIC as ::core::ffi::c_uchar);
    }
    test = get_refactfrequency(lp, FALSE as ::core::ffi::c_uchar);
    if test < MIN_REFACTFREQUENCY as ::core::ffi::c_double {
        test = get_refactfrequency(lp, TRUE as ::core::ffi::c_uchar);
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"invert: Refactorization frequency %.1g indicates numeric instability.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        (*lp).spx_status = NUMFAILURE;
    }
    if !(usedpos as *mut ::core::ffi::c_void).is_null() {
        free(usedpos as *mut ::core::ffi::c_void);
        usedpos = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return (singularities <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_fimprove"]
pub unsafe extern "C" fn fimprove(
    mut lp: *mut lprec,
    mut pcol: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut errors: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut sdp: ::core::ffi::c_double = 0.;
    let mut j: ::core::ffi::c_int = 0;
    let mut Ok_0: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    allocREAL(
        lp,
        &raw mut errors,
        (*lp).rows + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    if errors.is_null() {
        Ok_0 = FALSE as ::core::ffi::c_uchar;
        return Ok_0;
    }
    memcpy(
        errors as *mut ::core::ffi::c_void,
        pcol as *const ::core::ffi::c_void,
        (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    (*lp).bfp_ftran_normal.expect("non-null function pointer")(lp, pcol, nzidx);
    prod_Ax(
        lp,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        pcol,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0.0f64,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
        errors,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        MAT_ROUNDDEFAULT,
    );
    (*lp).bfp_ftran_normal.expect("non-null function pointer")(
        lp,
        errors,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    sdp = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).rows {
        if fabs(*errors.offset(j as isize)) > sdp {
            sdp = fabs(*errors.offset(j as isize));
        }
        j += 1;
    }
    if sdp > (*lp).epsmachine {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"Iterative FTRAN correction metric %g\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).rows {
            *pcol.offset(j as isize) += *errors.offset(j as isize);
            if fabs(*pcol.offset(j as isize)) < roundzero {
                *pcol.offset(j as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            j += 1;
        }
    }
    if !(errors as *mut ::core::ffi::c_void).is_null() {
        free(errors as *mut ::core::ffi::c_void);
        errors = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return Ok_0;
}
#[export_name="honest_lpsolve_bimprove"]
pub unsafe extern "C" fn bimprove(
    mut lp: *mut lprec,
    mut rhsvector: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut j: ::core::ffi::c_int = 0;
    let mut errors: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut err: ::core::ffi::c_double = 0.;
    let mut maxerr: ::core::ffi::c_double = 0.;
    let mut Ok_0: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    allocREAL(
        lp,
        &raw mut errors,
        (*lp).sum + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    if errors.is_null() {
        Ok_0 = FALSE as ::core::ffi::c_uchar;
        return Ok_0;
    }
    memcpy(
        errors as *mut ::core::ffi::c_void,
        rhsvector as *const ::core::ffi::c_void,
        (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    (*lp).bfp_btran_normal.expect("non-null function pointer")(lp, errors, nzidx);
    prod_xA(
        lp,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        errors,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0.0f64,
        1.0f64,
        errors,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        MAT_ROUNDDEFAULT,
    );
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).rows {
        *errors.offset(j as isize) = *errors
            .offset(((*lp).rows + *(*lp).var_basic.offset(j as isize)) as isize)
            - *rhsvector.offset(j as isize);
        j += 1;
    }
    j = (*lp).rows;
    while j <= (*lp).sum {
        *errors.offset(j as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j += 1;
    }
    (*lp).bfp_btran_normal.expect("non-null function pointer")(
        lp,
        errors,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    maxerr = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).rows {
        if !(*(*lp).var_basic.offset(j as isize) <= (*lp).rows) {
            err = *errors.offset(((*lp).rows + *(*lp).var_basic.offset(j as isize)) as isize);
            if fabs(err) > maxerr {
                maxerr = fabs(err);
            }
        }
        j += 1;
    }
    if maxerr > (*lp).epsmachine {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"Iterative BTRAN correction metric %g\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).rows {
            if !(*(*lp).var_basic.offset(j as isize) <= (*lp).rows) {
                *rhsvector.offset(j as isize) +=
                    *errors.offset(((*lp).rows + *(*lp).var_basic.offset(j as isize)) as isize);
                if fabs(*rhsvector.offset(j as isize)) < roundzero {
                    *rhsvector.offset(j as isize) =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            }
            j += 1;
        }
    }
    if !(errors as *mut ::core::ffi::c_void).is_null() {
        free(errors as *mut ::core::ffi::c_void);
        errors = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return Ok_0;
}
#[export_name="honest_lpsolve_ftran"]
pub unsafe extern "C" fn ftran(
    mut lp: *mut lprec,
    mut rhsvector: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
) {
    (*lp).bfp_ftran_normal.expect("non-null function pointer")(lp, rhsvector, nzidx);
}
#[export_name="honest_lpsolve_btran"]
pub unsafe extern "C" fn btran(
    mut lp: *mut lprec,
    mut rhsvector: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
) {
    (*lp).bfp_btran_normal.expect("non-null function pointer")(lp, rhsvector, nzidx);
}
#[export_name="honest_lpsolve_fsolve"]
pub unsafe extern "C" fn fsolve(
    mut lp: *mut lprec,
    mut varin: ::core::ffi::c_int,
    mut pcol: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
    mut ofscalar: ::core::ffi::c_double,
    mut prepareupdate: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if varin > 0 as ::core::ffi::c_int {
        obtain_column(
            lp,
            varin,
            pcol,
            nzidx,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    }
    *pcol.offset(0 as ::core::ffi::c_int as isize) *= ofscalar;
    if prepareupdate != 0 {
        (*lp).bfp_ftran_prepare.expect("non-null function pointer")(lp, pcol, nzidx);
    } else {
        ftran(lp, pcol, nzidx, roundzero);
    }
    return ok;
}
#[export_name="honest_lpsolve_bsolve"]
pub unsafe extern "C" fn bsolve(
    mut lp: *mut lprec,
    mut row_nr: ::core::ffi::c_int,
    mut rhsvector: *mut ::core::ffi::c_double,
    mut nzidx: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
    mut ofscalar: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if row_nr >= 0 as ::core::ffi::c_int {
        row_nr = obtain_column(
            lp,
            row_nr,
            rhsvector,
            nzidx,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    }
    *rhsvector.offset(0 as ::core::ffi::c_int as isize) *= ofscalar;
    btran(lp, rhsvector, nzidx, roundzero);
    return ok;
}
#[export_name="honest_lpsolve_vec_compress"]
pub unsafe extern "C" fn vec_compress(
    mut densevector: *mut ::core::ffi::c_double,
    mut startpos: ::core::ffi::c_int,
    mut endpos: ::core::ffi::c_int,
    mut epsilon: ::core::ffi::c_double,
    mut nzvector: *mut ::core::ffi::c_double,
    mut nzindex: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut n: ::core::ffi::c_int = 0;
    if densevector.is_null() || nzindex.is_null() || startpos > endpos {
        return 0 as ::core::ffi::c_uchar;
    }
    n = 0 as ::core::ffi::c_int;
    densevector = densevector.offset(startpos as isize);
    while startpos <= endpos {
        if fabs(*densevector) > epsilon {
            if !nzvector.is_null() {
                *nzvector.offset(n as isize) = *densevector;
            }
            n += 1;
            *nzindex.offset(n as isize) = startpos;
        }
        startpos += 1;
        densevector = densevector.offset(1);
    }
    *nzindex.offset(0 as ::core::ffi::c_int as isize) = n;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_vec_expand"]
pub unsafe extern "C" fn vec_expand(
    mut nzvector: *mut ::core::ffi::c_double,
    mut nzindex: *mut ::core::ffi::c_int,
    mut densevector: *mut ::core::ffi::c_double,
    mut startpos: ::core::ffi::c_int,
    mut endpos: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    n = *nzindex.offset(0 as ::core::ffi::c_int as isize);
    i = *nzindex.offset(n as isize);
    densevector = densevector.offset(endpos as isize);
    while endpos >= startpos {
        if endpos == i {
            n -= 1;
            *densevector = *nzvector.offset(n as isize);
            i = *nzindex.offset(n as isize);
        } else {
            *densevector = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        endpos -= 1;
        densevector = densevector.offset(-1);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_colIndexA"]
pub unsafe extern "C" fn get_colIndexA(
    mut lp: *mut lprec,
    mut varset: ::core::ffi::c_int,
    mut colindex: *mut ::core::ffi::c_int,
    mut append: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut varnr: ::core::ffi::c_int = 0;
    let mut P1extraDim: ::core::ffi::c_int = 0;
    let mut vb: ::core::ffi::c_int = 0;
    let mut ve: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut nsum: ::core::ffi::c_int = (*lp).sum;
    let mut omitfixed: ::core::ffi::c_uchar = 0;
    let mut omitnonfixed: ::core::ffi::c_uchar = 0;
    let mut v: ::core::ffi::c_double = 0.;
    P1extraDim = abs((*lp).P1extraDim);
    vb = nrows + 1 as ::core::ffi::c_int;
    if varset & SCAN_ARTIFICIALVARS != 0 {
        vb = nsum - P1extraDim + 1 as ::core::ffi::c_int;
    }
    if varset & SCAN_USERVARS != 0 {
        vb = nrows + 1 as ::core::ffi::c_int;
    }
    if varset & SCAN_SLACKVARS != 0 {
        vb = 1 as ::core::ffi::c_int;
    }
    ve = nsum;
    if varset & SCAN_SLACKVARS != 0 {
        ve = nrows;
    }
    if varset & SCAN_USERVARS != 0 {
        ve = nsum - P1extraDim;
    }
    if varset & SCAN_ARTIFICIALVARS != 0 {
        ve = nsum;
    }
    if varset & SCAN_PARTIALBLOCK != 0 {
        if vb < partial_blockStart(lp, 0 as ::core::ffi::c_uchar) {
            vb = partial_blockStart(lp, 0 as ::core::ffi::c_uchar);
        }
        if ve > partial_blockEnd(lp, 0 as ::core::ffi::c_uchar) {
            ve = partial_blockEnd(lp, 0 as ::core::ffi::c_uchar);
        }
    }
    omitfixed = (varset & OMIT_FIXED != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    omitnonfixed = (varset & OMIT_NONFIXED != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    if omitfixed as ::core::ffi::c_int != 0 && omitnonfixed as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if append != 0 {
        n = *colindex.offset(0 as ::core::ffi::c_int as isize);
    } else {
        n = 0 as ::core::ffi::c_int;
    }
    let mut current_block_34: u64;
    varnr = vb;
    while varnr <= ve {
        if varnr > nrows {
            if varnr <= nsum - P1extraDim && varset & SCAN_USERVARS == 0 {
                current_block_34 = 5634871135123216486;
            } else if mat_collength((*lp).matA, varnr - nrows) == 0 as ::core::ffi::c_int {
                current_block_34 = 5634871135123216486;
            } else {
                current_block_34 = 15925075030174552612;
            }
        } else {
            current_block_34 = 15925075030174552612;
        }
        match current_block_34 {
            15925075030174552612 => {
                i = *(*lp).is_basic.offset(varnr as isize) as ::core::ffi::c_int;
                if varset & USE_BASICVARS > 0 as ::core::ffi::c_int && i != 0 {
                    current_block_34 = 15897653523371991391;
                } else if varset & USE_NONBASICVARS > 0 as ::core::ffi::c_int && i == 0 {
                    current_block_34 = 15897653523371991391;
                } else {
                    current_block_34 = 5634871135123216486;
                }
                match current_block_34 {
                    5634871135123216486 => {}
                    _ => {
                        v = *(*lp).upbo.offset(varnr as isize);
                        if !(omitfixed as ::core::ffi::c_int != 0
                            && v == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            || omitnonfixed as ::core::ffi::c_int != 0
                                && v != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                        {
                            n += 1;
                            *colindex.offset(n as isize) = varnr;
                        }
                    }
                }
            }
            _ => {}
        }
        varnr += 1;
    }
    *colindex.offset(0 as ::core::ffi::c_int as isize) = n;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_prod_Ax"]
pub unsafe extern "C" fn prod_Ax(
    mut lp: *mut lprec,
    mut coltarget: *mut ::core::ffi::c_int,
    mut input: *mut ::core::ffi::c_double,
    mut nzinput: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
    mut ofscalar: ::core::ffi::c_double,
    mut output: *mut ::core::ffi::c_double,
    mut nzoutput: *mut ::core::ffi::c_int,
    mut roundmode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut vb: ::core::ffi::c_int = 0;
    let mut localset: ::core::ffi::c_uchar = 0;
    let mut localnz: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut isRC: ::core::ffi::c_uchar = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut sdp: ::core::ffi::c_double = 0.;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    isRC = (roundmode & MAT_ROUNDRC != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    localset = (coltarget == NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    if localset != 0 {
        let mut varset: ::core::ffi::c_int =
            SCAN_SLACKVARS | SCAN_USERVARS | USE_BASICVARS | OMIT_FIXED;
        if isRC as ::core::ffi::c_int != 0
            && is_piv_mode(lp, PRICE_PARTIAL) as ::core::ffi::c_int != 0
            && is_piv_mode(lp, PRICE_FORCEFULL) == 0
        {
            varset |= SCAN_PARTIALBLOCK;
        }
        coltarget = mempool_obtainVector(
            (*lp).workarrays,
            (*lp).sum + 1 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        if get_colIndexA(lp, varset, coltarget, FALSE as ::core::ffi::c_uchar) == 0 {
            mempool_releaseVector(
                (*lp).workarrays,
                coltarget as *mut ::core::ffi::c_char,
                FALSE as ::core::ffi::c_uchar,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    localnz =
        (nzinput == NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if localnz != 0 {
        nzinput = mempool_obtainVector(
            (*lp).workarrays,
            (*lp).rows + 1 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        vec_compress(
            input,
            0 as ::core::ffi::c_int,
            (*lp).rows,
            (*(*lp).matA).epsvalue,
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            nzinput,
        );
    }
    vb = 1 as ::core::ffi::c_int;
    vb = 1 as ::core::ffi::c_int;
    while vb <= *coltarget.offset(0 as ::core::ffi::c_int as isize) {
        colnr = *coltarget.offset(vb as isize);
        j = *(*lp).is_basic.offset(colnr as isize) as ::core::ffi::c_int;
        sdp = ofscalar * *input.offset(j as isize);
        if colnr <= (*lp).rows {
            *output.offset(colnr as isize) += sdp;
        } else {
            colnr -= (*lp).rows;
            ib = *(*mat)
                .col_end
                .offset((colnr - 1 as ::core::ffi::c_int) as isize);
            ie = *(*mat).col_end.offset(colnr as isize);
            rownr = (*mat).col_mat_rownr.offset(ib as isize) as *mut ::core::ffi::c_int;
            value = (*mat).col_mat_value.offset(ib as isize) as *mut ::core::ffi::c_double;
            while ib < ie {
                *output.offset(*rownr as isize) += *value * sdp;
                ib += 1;
                rownr = rownr.offset(matRowColStep as isize);
                value = value.offset(matValueStep as isize);
            }
        }
        vb += 1;
    }
    roundVector(
        output.offset(1 as ::core::ffi::c_int as isize),
        (*lp).rows - 1 as ::core::ffi::c_int,
        roundzero,
    );
    if localset != 0 {
        mempool_releaseVector(
            (*lp).workarrays,
            coltarget as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    if localnz != 0 {
        mempool_releaseVector(
            (*lp).workarrays,
            nzinput as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    return 1 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_prod_xA"]
pub unsafe extern "C" fn prod_xA(
    mut lp: *mut lprec,
    mut coltarget: *mut ::core::ffi::c_int,
    mut input: *mut ::core::ffi::c_double,
    mut nzinput: *mut ::core::ffi::c_int,
    mut roundzero: ::core::ffi::c_double,
    mut ofscalar: ::core::ffi::c_double,
    mut output: *mut ::core::ffi::c_double,
    mut nzoutput: *mut ::core::ffi::c_int,
    mut roundmode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut colnr: ::core::ffi::c_int = 0;
    let mut rownr: ::core::ffi::c_int = 0;
    let mut varnr: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut vb: ::core::ffi::c_int = 0;
    let mut ve: ::core::ffi::c_int = 0;
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut localset: ::core::ffi::c_uchar = 0;
    let mut localnz: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut includeOF: ::core::ffi::c_uchar = 0;
    let mut isRC: ::core::ffi::c_uchar = 0;
    let mut vmax: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
    let mut v: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
    let mut inz: ::core::ffi::c_int = 0;
    let mut rowin: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut countNZ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    isRC = (roundmode & MAT_ROUNDRC != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    if nzoutput.is_null() {
        if input == output {
            memset(
                output
                    .offset(nrows as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                '\0' as i32,
                ((*lp).columns as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        } else {
            memset(
                output as *mut ::core::ffi::c_void,
                '\0' as i32,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
    }
    localset = (coltarget == NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    if localset != 0 {
        let mut varset: ::core::ffi::c_int =
            SCAN_SLACKVARS | SCAN_USERVARS | USE_NONBASICVARS | OMIT_FIXED;
        if isRC as ::core::ffi::c_int != 0
            && is_piv_mode(lp, PRICE_PARTIAL) as ::core::ffi::c_int != 0
            && is_piv_mode(lp, PRICE_FORCEFULL) == 0
        {
            varset |= SCAN_PARTIALBLOCK;
        }
        coltarget = mempool_obtainVector(
            (*lp).workarrays,
            (*lp).sum + 1 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        if get_colIndexA(lp, varset, coltarget, FALSE as ::core::ffi::c_uchar) == 0 {
            mempool_releaseVector(
                (*lp).workarrays,
                coltarget as *mut ::core::ffi::c_char,
                FALSE as ::core::ffi::c_uchar,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    includeOF = ((nzinput.is_null()
        || *nzinput.offset(1 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int)
        && *input.offset(0 as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*lp).obj_in_basis as ::core::ffi::c_int != 0) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    vmax = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
    ve = *coltarget.offset(0 as ::core::ffi::c_int as isize);
    vb = 1 as ::core::ffi::c_int;
    while vb <= ve {
        varnr = *coltarget.offset(vb as isize);
        if varnr <= nrows {
            v = crate::honest_did::lpsolve::extended::Extended::new(*input.offset(varnr as isize));
        } else {
            colnr = varnr - nrows;
            v = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
            ib = *(*mat)
                .col_end
                .offset((colnr - 1 as ::core::ffi::c_int) as isize);
            ie = *(*mat).col_end.offset(colnr as isize);
            if ib < ie {
                if nzinput.is_null() {
                    if includeOF != 0 {
                        v += crate::honest_did::lpsolve::extended::Extended::new(
                            *input.offset(0 as ::core::ffi::c_int as isize)
                                * *(*lp).obj.offset(colnr as isize)
                                * ofscalar,
                        );
                    }
                    matRownr = (*mat).col_mat_rownr.offset(ib as isize) as *mut ::core::ffi::c_int;
                    matValue =
                        (*mat).col_mat_value.offset(ib as isize) as *mut ::core::ffi::c_double;
                    while ib < ie {
                        v += crate::honest_did::lpsolve::extended::Extended::new(*input.offset(*matRownr as isize) * *matValue);
                        matValue = matValue.offset(matValueStep as isize);
                        matRownr = matRownr.offset(matRowColStep as isize);
                        ib += 1;
                    }
                } else {
                    if includeOF != 0 {
                        v += crate::honest_did::lpsolve::extended::Extended::new(
                            *input.offset(0 as ::core::ffi::c_int as isize)
                                * *(*lp).obj.offset(colnr as isize)
                                * ofscalar,
                        );
                    }
                    inz = 1 as ::core::ffi::c_int;
                    rowin = nzinput.offset(inz as isize);
                    matRownr = (*mat).col_mat_rownr.offset(ib as isize) as *mut ::core::ffi::c_int;
                    matValue =
                        (*mat).col_mat_value.offset(ib as isize) as *mut ::core::ffi::c_double;
                    ie -= 1;
                    while inz <= *nzinput && ib <= ie {
                        while *rowin > *matRownr && ib < ie {
                            ib += 1;
                            matValue = matValue.offset(matValueStep as isize);
                            matRownr = matRownr.offset(matRowColStep as isize);
                        }
                        while *rowin < *matRownr && inz < *nzinput {
                            inz += 1;
                            rowin = rowin.offset(1);
                        }
                        if *rowin == *matRownr {
                            v += crate::honest_did::lpsolve::extended::Extended::new(*input.offset(*rowin as isize) * *matValue);
                            inz += 1;
                            rowin = rowin.offset(1);
                        }
                    }
                }
            }
            if roundmode & MAT_ROUNDABS != 0 as ::core::ffi::c_int {
                if fabs(v.to_f64().unwrap()) < roundzero {
                    v = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
                }
            }
        }
        if isRC == 0
            || (if *(*lp).is_lower.offset(varnr as isize) as ::core::ffi::c_int != 0
                && v != crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int)
            {
                -v
            } else {
                v
            }) < crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int)
        {
            if vmax < crate::honest_did::lpsolve::extended::Extended::new(fabs(v.to_f64().unwrap())) {
                vmax = crate::honest_did::lpsolve::extended::Extended::new(fabs(v.to_f64().unwrap()));
            }
        }
        if v != crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int) {
            countNZ += 1;
            if !nzoutput.is_null() {
                *nzoutput.offset(countNZ as isize) = varnr;
            }
        }
        *output.offset(varnr as isize) = v.to_f64().unwrap();
        vb += 1;
    }
    if isRC as ::core::ffi::c_int != 0 && (*lp).obj_in_basis == 0 {
        countNZ = get_basisOF(
            lp,
            coltarget as *mut ::core::ffi::c_int,
            output as *mut ::core::ffi::c_double,
            nzoutput as *mut ::core::ffi::c_int,
        );
    }
    if roundmode & MAT_ROUNDREL != 0 as ::core::ffi::c_int {
        if roundzero > 0 as ::core::ffi::c_int as ::core::ffi::c_double && !nzoutput.is_null() {
            ie = 0 as ::core::ffi::c_int;
            if isRC != 0 {
                if vmax < crate::honest_did::lpsolve::extended::Extended::new(1.0) {
                    vmax = crate::honest_did::lpsolve::extended::Extended::new(1.0);
                }
            }
            vmax *= crate::honest_did::lpsolve::extended::Extended::new(roundzero);
            ib = 1 as ::core::ffi::c_int;
            while ib <= countNZ {
                rownr = *nzoutput.offset(ib as isize);
                if crate::honest_did::lpsolve::extended::Extended::new(fabs(*output.offset(rownr as isize))) < vmax {
                    *output.offset(rownr as isize) =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ie += 1;
                    *nzoutput.offset(ie as isize) = rownr;
                }
                ib += 1;
            }
            countNZ = ie;
        }
    }
    if localset != 0 {
        mempool_releaseVector(
            (*lp).workarrays,
            coltarget as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    if localnz != 0 {
        mempool_releaseVector(
            (*lp).workarrays,
            nzinput as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    if !nzoutput.is_null() {
        *nzoutput = countNZ;
    }
    return countNZ;
}
#[export_name="honest_lpsolve_prod_xA2"]
pub unsafe extern "C" fn prod_xA2(
    mut lp: *mut lprec,
    mut coltarget: *mut ::core::ffi::c_int,
    mut prow: *mut ::core::ffi::c_double,
    mut proundzero: ::core::ffi::c_double,
    mut nzprow: *mut ::core::ffi::c_int,
    mut drow: *mut ::core::ffi::c_double,
    mut droundzero: ::core::ffi::c_double,
    mut nzdrow: *mut ::core::ffi::c_int,
    mut ofscalar: ::core::ffi::c_double,
    mut roundmode: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut varnr: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut vb: ::core::ffi::c_int = 0;
    let mut ve: ::core::ffi::c_int = 0;
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut includeOF: ::core::ffi::c_uchar = 0;
    let mut isRC: ::core::ffi::c_uchar = 0;
    let mut dmax: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
    let mut pmax: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
    let mut d: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
    let mut p: crate::honest_did::lpsolve::extended::Extended = crate::honest_did::lpsolve::extended::Extended::ZERO;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut value: ::core::ffi::c_double = 0.;
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut localset: ::core::ffi::c_uchar = 0;
    localset = (coltarget == NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    if localset != 0 {
        let mut varset: ::core::ffi::c_int =
            SCAN_SLACKVARS + SCAN_USERVARS + USE_NONBASICVARS + OMIT_FIXED;
        coltarget = mempool_obtainVector(
            (*lp).workarrays,
            (*lp).sum + 1 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        if get_colIndexA(lp, varset, coltarget, FALSE as ::core::ffi::c_uchar) == 0 {
            mempool_releaseVector(
                (*lp).workarrays,
                coltarget as *mut ::core::ffi::c_char,
                FALSE as ::core::ffi::c_uchar,
            );
            return 0 as ::core::ffi::c_uchar;
        }
    }
    isRC = (roundmode & MAT_ROUNDRC != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    pmax = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
    dmax = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
    if !nzprow.is_null() {
        *nzprow = 0 as ::core::ffi::c_int;
    }
    if !nzdrow.is_null() {
        *nzdrow = 0 as ::core::ffi::c_int;
    }
    includeOF = ((*prow.offset(0 as ::core::ffi::c_int as isize)
        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        || *drow.offset(0 as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        && (*lp).obj_in_basis as ::core::ffi::c_int != 0) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    ve = *coltarget.offset(0 as ::core::ffi::c_int as isize);
    vb = 1 as ::core::ffi::c_int;
    while vb <= ve {
        varnr = *coltarget.offset(vb as isize);
        if varnr <= nrows {
            p = crate::honest_did::lpsolve::extended::Extended::new(*prow.offset(varnr as isize));
            d = crate::honest_did::lpsolve::extended::Extended::new(*drow.offset(varnr as isize));
        } else {
            colnr = varnr - nrows;
            p = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
            d = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
            ib = *(*mat)
                .col_end
                .offset((colnr - 1 as ::core::ffi::c_int) as isize);
            ie = *(*mat).col_end.offset(colnr as isize);
            if ib < ie {
                if includeOF != 0 {
                    value = *(*lp).obj.offset(colnr as isize) * ofscalar;
                    p += crate::honest_did::lpsolve::extended::Extended::new(*prow.offset(0 as ::core::ffi::c_int as isize) * value);
                    d += crate::honest_did::lpsolve::extended::Extended::new(*drow.offset(0 as ::core::ffi::c_int as isize) * value);
                }
                matRownr = (*mat).col_mat_rownr.offset(ib as isize) as *mut ::core::ffi::c_int;
                matValue = (*mat).col_mat_value.offset(ib as isize) as *mut ::core::ffi::c_double;
                while ib < ie {
                    p += crate::honest_did::lpsolve::extended::Extended::new(*prow.offset(*matRownr as isize) * *matValue);
                    d += crate::honest_did::lpsolve::extended::Extended::new(*drow.offset(*matRownr as isize) * *matValue);
                    matValue = matValue.offset(matValueStep as isize);
                    matRownr = matRownr.offset(matRowColStep as isize);
                    ib += 1;
                }
            }
            if roundmode & MAT_ROUNDABS != 0 as ::core::ffi::c_int {
                if fabs(p.to_f64().unwrap()) < proundzero {
                    p = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
                }
                if fabs(d.to_f64().unwrap()) < droundzero {
                    d = crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int);
                }
            }
        }
        if pmax < crate::honest_did::lpsolve::extended::Extended::new(fabs(p.to_f64().unwrap())) {
            pmax = crate::honest_did::lpsolve::extended::Extended::new(fabs(p.to_f64().unwrap()));
        }
        *prow.offset(varnr as isize) = p.to_f64().unwrap();
        if !nzprow.is_null() && p != crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int) {
            *nzprow += 1;
            *nzprow.offset(*nzprow as isize) = varnr;
        }
        if isRC == 0
            || (if *(*lp).is_lower.offset(varnr as isize) as ::core::ffi::c_int != 0
                && d != crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int)
            {
                -d
            } else {
                d
            }) < crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int)
        {
            if dmax < crate::honest_did::lpsolve::extended::Extended::new(fabs(d.to_f64().unwrap())) {
                dmax = crate::honest_did::lpsolve::extended::Extended::new(fabs(d.to_f64().unwrap()));
            }
        }
        *drow.offset(varnr as isize) = d.to_f64().unwrap();
        if !nzdrow.is_null() && d != crate::honest_did::lpsolve::extended::Extended::new(0 as ::core::ffi::c_int) {
            *nzdrow += 1;
            *nzdrow.offset(*nzdrow as isize) = varnr;
        }
        vb += 1;
    }
    if !drow.is_null() && (*lp).obj_in_basis == 0 {
        get_basisOF(
            lp,
            coltarget as *mut ::core::ffi::c_int,
            drow as *mut ::core::ffi::c_double,
            nzdrow as *mut ::core::ffi::c_int,
        );
    }
    if roundmode & MAT_ROUNDREL != 0 as ::core::ffi::c_int {
        if proundzero > 0 as ::core::ffi::c_int as ::core::ffi::c_double && !nzprow.is_null() {
            ie = 0 as ::core::ffi::c_int;
            pmax *= crate::honest_did::lpsolve::extended::Extended::new(proundzero);
            ib = 1 as ::core::ffi::c_int;
            while ib <= *nzprow {
                varnr = *nzprow.offset(ib as isize);
                if crate::honest_did::lpsolve::extended::Extended::new(fabs(*prow.offset(varnr as isize))) < pmax {
                    *prow.offset(varnr as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ie += 1;
                    *nzprow.offset(ie as isize) = varnr;
                }
                ib += 1;
            }
            *nzprow = ie;
        }
        if droundzero > 0 as ::core::ffi::c_int as ::core::ffi::c_double && !nzdrow.is_null() {
            ie = 0 as ::core::ffi::c_int;
            if isRC != 0 {
                if dmax < crate::honest_did::lpsolve::extended::Extended::new(1.0) {
                    dmax = crate::honest_did::lpsolve::extended::Extended::new(1.0);
                }
            }
            dmax *= crate::honest_did::lpsolve::extended::Extended::new(droundzero);
            ib = 1 as ::core::ffi::c_int;
            while ib <= *nzdrow {
                varnr = *nzdrow.offset(ib as isize);
                if crate::honest_did::lpsolve::extended::Extended::new(fabs(*drow.offset(varnr as isize))) < dmax {
                    *drow.offset(varnr as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ie += 1;
                    *nzdrow.offset(ie as isize) = varnr;
                }
                ib += 1;
            }
            *nzdrow = ie;
        }
    }
    if localset != 0 {
        mempool_releaseVector(
            (*lp).workarrays,
            coltarget as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_bsolve_xA2"]
pub unsafe extern "C" fn bsolve_xA2(
    mut lp: *mut lprec,
    mut coltarget: *mut ::core::ffi::c_int,
    mut row_nr1: ::core::ffi::c_int,
    mut vector1: *mut ::core::ffi::c_double,
    mut roundzero1: ::core::ffi::c_double,
    mut nzvector1: *mut ::core::ffi::c_int,
    mut row_nr2: ::core::ffi::c_int,
    mut vector2: *mut ::core::ffi::c_double,
    mut roundzero2: ::core::ffi::c_double,
    mut nzvector2: *mut ::core::ffi::c_int,
    mut roundmode: ::core::ffi::c_int,
) {
    let mut ofscalar: ::core::ffi::c_double = 1.0f64;
    if nzvector1.is_null() {
        memset(
            vector1 as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    } else {
        memset(
            vector1 as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    *vector1.offset(row_nr1 as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    if vector2.is_null() {
        (*lp).bfp_btran_normal.expect("non-null function pointer")(
            lp,
            vector1,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        prod_xA(
            lp,
            coltarget,
            vector1,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            roundzero1,
            ofscalar * 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            vector1,
            nzvector1,
            roundmode,
        );
    } else {
        if nzvector2.is_null() {
            memset(
                vector2 as *mut ::core::ffi::c_void,
                '\0' as i32,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        } else {
            memset(
                vector2 as *mut ::core::ffi::c_void,
                '\0' as i32,
                (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
        if (*lp).obj_in_basis as ::core::ffi::c_int != 0 || row_nr2 > 0 as ::core::ffi::c_int {
            *vector2.offset(row_nr2 as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            get_basisOF(
                lp,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                vector2 as *mut ::core::ffi::c_double,
                nzvector2 as *mut ::core::ffi::c_int,
            );
        }
        (*lp).bfp_btran_double.expect("non-null function pointer")(
            lp,
            vector1,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            vector2,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        prod_xA2(
            lp, coltarget, vector1, roundzero1, nzvector1, vector2, roundzero2, nzvector2,
            ofscalar, roundmode,
        );
    };
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PRESOLVE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NORMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MSG_INVERT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCAN_USERVARS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SCAN_SLACKVARS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SCAN_ARTIFICIALVARS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCAN_PARTIALBLOCK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const USE_BASICVARS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const USE_NONBASICVARS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OMIT_FIXED: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const OMIT_NONFIXED: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PRICE_PARTIAL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PRICE_FORCEFULL: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const ACTION_REBASE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ACTION_RECOMPUTE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ACTION_REINVERT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const INFEASIBLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NUMFAILURE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MAT_START_SIZE: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const RESIZEFACTOR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MIN_REFACTFREQUENCY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MAT_ROUNDABS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAT_ROUNDREL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAT_ROUNDRC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAT_ROUNDDEFAULT: ::core::ffi::c_int = MAT_ROUNDREL;
pub const matRowColStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const matValueStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LINEARSEARCH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
