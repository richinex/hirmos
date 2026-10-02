use crate::honest_did::lpsolve::runtime::{calloc,free,realloc,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_getPricer"]
    fn getPricer(
        lp: *mut lprec,
        item: ::core::ffi::c_int,
        isdual: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_double;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn log(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log10(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
    #[link_name="honest_lpsolve_swapINT"]
    fn swapINT(item1: *mut ::core::ffi::c_int, item2: *mut ::core::ffi::c_int);
    #[link_name="honest_lpsolve_rand_uniform"]
    fn rand_uniform(lp: *mut lprec, range: ::core::ffi::c_double) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_mat_validate"]
    fn mat_validate(mat: *mut MATrec) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_fsolve"]
    fn fsolve(
        lp: *mut lprec,
        varin: ::core::ffi::c_int,
        pcol: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
        ofscalar: ::core::ffi::c_double,
        prepareupdate: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bsolve"]
    fn bsolve(
        lp: *mut lprec,
        row_nr: ::core::ffi::c_int,
        rhsvector: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
        ofscalar: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bsolve_xA2"]
    fn bsolve_xA2(
        lp: *mut lprec,
        coltarget: *mut ::core::ffi::c_int,
        row_nr1: ::core::ffi::c_int,
        vector1: *mut ::core::ffi::c_double,
        roundzero1: ::core::ffi::c_double,
        nzvector1: *mut ::core::ffi::c_int,
        row_nr2: ::core::ffi::c_int,
        vector2: *mut ::core::ffi::c_double,
        roundzero2: ::core::ffi::c_double,
        nzvector2: *mut ::core::ffi::c_int,
        roundmode: ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_is_infinite"]
    fn is_infinite(lp: *mut lprec, value: ::core::ffi::c_double) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_partialprice"]
    fn set_partialprice(
        lp: *mut lprec,
        blockcount: ::core::ffi::c_int,
        blockstart: *mut ::core::ffi::c_int,
        isrow: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_piv_mode"]
    fn is_piv_mode(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_refactRecent"]
    fn refactRecent(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_fixedvar"]
    fn is_fixedvar(lp: *mut lprec, variable: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_action"]
    fn set_action(actionvar: *mut ::core::ffi::c_int, actionmask: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_clear_action"]
    fn clear_action(actionvar: *mut ::core::ffi::c_int, actionmask: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_is_action"]
    fn is_action(
        actionvar: ::core::ffi::c_int,
        testmask: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_piv_rule"]
    fn get_piv_rule(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bb_better"]
    fn bb_better(
        lp: *mut lprec,
        target: ::core::ffi::c_int,
        mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_OF_active"]
    fn get_OF_active(
        lp: *mut lprec,
        varnr: ::core::ffi::c_int,
        mult: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_compute_theta"]
    fn compute_theta(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        theta: *mut ::core::ffi::c_double,
        isupbound: ::core::ffi::c_int,
        HarrisScalar: ::core::ffi::c_double,
        primal: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_findIndexEx"]
    fn findIndexEx(
        target: *mut ::core::ffi::c_void,
        attributes: *mut ::core::ffi::c_void,
        count: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        recsize: ::core::ffi::c_int,
        findCompare: Option<findCompare_func>,
        ascending: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_compareREAL"]
    fn compareREAL(
        current: *const ::core::ffi::c_void,
        candidate: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_QS_append"]
    fn QS_append(
        a: *mut QSORTrec,
        ipos: ::core::ffi::c_int,
        mydata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_QS_insert"]
    fn QS_insert(
        a: *mut QSORTrec,
        ipos: ::core::ffi::c_int,
        mydata: *mut ::core::ffi::c_void,
        epos: ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_QS_execute"]
    fn QS_execute(
        a: *mut QSORTrec,
        count: ::core::ffi::c_int,
        findCompare: Option<findCompare_func>,
        nswaps: *mut ::core::ffi::c_int,
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
pub type findCompare_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;
#[export_name="honest_lpsolve_compareImprovementVar"]
pub unsafe extern "C" fn compareImprovementVar(
    mut current: *const pricerec,
    mut candidate: *const pricerec,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut result: ::core::ffi::c_int = COMP_PREFERNONE;
    let mut lp: *mut lprec = (*current).lp;
    let mut testvalue: ::core::ffi::c_double = 0.;
    let mut margin: ::core::ffi::c_double = (*lp).epsdual;
    let mut currentvarno: ::core::ffi::c_int = (*current).varno;
    let mut candidatevarno: ::core::ffi::c_int = (*candidate).varno;
    let mut isdual: ::core::ffi::c_uchar = (*candidate).isdual;
    if isdual != 0 {
        candidatevarno = *(*lp).var_basic.offset(candidatevarno as isize);
        currentvarno = *(*lp).var_basic.offset(currentvarno as isize);
    }
    if (*lp)._piv_rule_ != PRICER_FIRSTINDEX {
        let mut candbetter: ::core::ffi::c_uchar = 0;
        testvalue = (*candidate).pivot;
        if fabs(testvalue) < LIMIT_ABS_REL {
            testvalue -= (*current).pivot;
        } else {
            testvalue = (testvalue - (*current).pivot) / (1.0f64 + fabs((*current).pivot));
        }
        if isdual != 0 {
            testvalue = -testvalue;
        }
        candbetter = (testvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if candbetter != 0 {
            if testvalue > margin {
                result = COMP_PREFERCANDIDATE;
            }
        } else if testvalue < -(*lp).epsvalue {
            result = COMP_PREFERINCUMBENT;
        }
        if result == COMP_PREFERNONE && candbetter as ::core::ffi::c_int != 0 {
            result = COMP_PREFERCANDIDATE;
            current_block = 1688860142876511044;
        } else {
            current_block = 2370887241019905314;
        }
    } else {
        current_block = 2370887241019905314;
    }
    match current_block {
        2370887241019905314 => {
            if result == COMP_PREFERNONE {
                if (*lp).piv_strategy & PRICE_RANDOMIZE != 0 {
                    result = if 0.1f64 - rand_uniform(lp, 1.0f64)
                        < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    };
                    if candidatevarno < currentvarno {
                        result = -result;
                    }
                }
                if result == COMP_PREFERNONE {
                    if candidatevarno < currentvarno {
                        result = COMP_PREFERCANDIDATE;
                    } else {
                        result = COMP_PREFERINCUMBENT;
                    }
                    if (*lp)._piv_left_ != 0 {
                        result = -result;
                    }
                }
            }
        }
        _ => {}
    }
    return result;
}
#[export_name="honest_lpsolve_compareSubstitutionVar"]
pub unsafe extern "C" fn compareSubstitutionVar(
    mut current: *const pricerec,
    mut candidate: *const pricerec,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = COMP_PREFERNONE;
    let mut lp: *mut lprec = (*current).lp;
    let mut testvalue: ::core::ffi::c_double = (*candidate).theta;
    let mut margin: ::core::ffi::c_double = (*current).theta;
    let mut isdual: ::core::ffi::c_uchar = (*candidate).isdual;
    let mut candbetter: ::core::ffi::c_uchar = 0;
    let mut currentvarno: ::core::ffi::c_int = (*current).varno;
    let mut candidatevarno: ::core::ffi::c_int = (*candidate).varno;
    if isdual == 0 {
        candidatevarno = *(*lp).var_basic.offset(candidatevarno as isize);
        currentvarno = *(*lp).var_basic.offset(currentvarno as isize);
    }
    if isdual != 0 {
        testvalue = fabs(testvalue);
        margin = fabs(margin);
    }
    if fabs(testvalue) < LIMIT_ABS_REL {
        testvalue -= margin;
    } else {
        testvalue = (testvalue - margin) / (1.0f64 + fabs(margin));
    }
    margin = (*lp).epsprimal;
    candbetter = (testvalue < 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if candbetter != 0 {
        if testvalue < -margin {
            result = COMP_PREFERCANDIDATE;
        }
    } else if testvalue > margin {
        result = COMP_PREFERINCUMBENT;
    }
    if result == COMP_PREFERNONE {
        let mut currentpivot: ::core::ffi::c_double = fabs((*current).pivot);
        let mut candidatepivot: ::core::ffi::c_double = fabs((*candidate).pivot);
        if (*lp)._piv_rule_ == PRICER_FIRSTINDEX {
            margin = (*candidate).epspivot;
            if candidatepivot >= margin && currentpivot < margin {
                result = COMP_PREFERCANDIDATE;
            }
        } else {
            testvalue = candidatepivot - currentpivot;
            if testvalue > margin {
                result = COMP_PREFERCANDIDATE;
            } else if testvalue < -margin {
                result = COMP_PREFERINCUMBENT;
            }
        }
    }
    if result == COMP_PREFERNONE && candbetter as ::core::ffi::c_int != 0 {
        result = COMP_PREFERCANDIDATE;
    } else if result == COMP_PREFERNONE {
        if (*lp).piv_strategy & PRICE_RANDOMIZE != 0 {
            result = if 0.1f64 - rand_uniform(lp, 1.0f64)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
            if candidatevarno < currentvarno {
                result = -result;
            }
        }
        if result == COMP_PREFERNONE {
            if candidatevarno < currentvarno {
                result = COMP_PREFERCANDIDATE;
            } else {
                result = COMP_PREFERINCUMBENT;
            }
            if (*lp)._piv_left_ != 0 {
                result = -result;
            }
        }
    }
    return result;
}
#[export_name="honest_lpsolve_compareBoundFlipVar"]
pub unsafe extern "C" fn compareBoundFlipVar(
    mut current: *const pricerec,
    mut candidate: *const pricerec,
) -> ::core::ffi::c_int {
    let mut testvalue: ::core::ffi::c_double = 0.;
    let mut margin: ::core::ffi::c_double = 0.;
    let mut result: ::core::ffi::c_int = COMP_PREFERNONE;
    let mut lp: *mut lprec = (*current).lp;
    let mut candbetter: ::core::ffi::c_uchar = 0;
    let mut currentvarno: ::core::ffi::c_int = (*current).varno;
    let mut candidatevarno: ::core::ffi::c_int = (*candidate).varno;
    if (*current).isdual == 0 {
        candidatevarno = *(*lp).var_basic.offset(candidatevarno as isize);
        currentvarno = *(*lp).var_basic.offset(currentvarno as isize);
    }
    testvalue = (*candidate).theta;
    margin = (*current).theta;
    if (*candidate).isdual != 0 {
        testvalue = fabs(testvalue);
        margin = fabs(margin);
    }
    if fabs(margin) < LIMIT_ABS_REL {
        testvalue -= margin;
    } else {
        testvalue = (testvalue - margin) / (1.0f64 + fabs(margin));
    }
    margin = (*lp).epsprimal;
    candbetter = (testvalue < 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if candbetter != 0 {
        if testvalue < -margin {
            result = COMP_PREFERCANDIDATE;
        }
    } else if testvalue > margin {
        result = COMP_PREFERINCUMBENT;
    }
    if result == COMP_PREFERNONE {
        if result == COMP_PREFERNONE {
            let mut currentpivot: ::core::ffi::c_double = fabs((*current).pivot);
            let mut candidatepivot: ::core::ffi::c_double = fabs((*candidate).pivot);
            if candidatepivot > currentpivot + margin {
                result = COMP_PREFERCANDIDATE;
            } else if candidatepivot < currentpivot - margin {
                result = COMP_PREFERINCUMBENT;
            }
        }
        if result == COMP_PREFERNONE {
            result = compareREAL(
                (*lp).upbo.offset(currentvarno as isize) as *mut ::core::ffi::c_double
                    as *const ::core::ffi::c_void,
                (*lp).upbo.offset(candidatevarno as isize) as *mut ::core::ffi::c_double
                    as *const ::core::ffi::c_void,
            );
        }
    }
    if result == COMP_PREFERNONE && candbetter as ::core::ffi::c_int != 0 {
        result = COMP_PREFERCANDIDATE;
    } else if result == COMP_PREFERNONE {
        if candidatevarno < currentvarno {
            result = COMP_PREFERCANDIDATE;
        } else {
            result = COMP_PREFERINCUMBENT;
        }
        if (*lp)._piv_left_ != 0 {
            result = -result;
        }
    }
    return result;
}
#[export_name="honest_lpsolve_validImprovementVar"]
pub unsafe extern "C" fn validImprovementVar(mut candidate: *mut pricerec) -> ::core::ffi::c_uchar {
    let mut candidatepivot: ::core::ffi::c_double = fabs((*candidate).pivot);
    return (candidatepivot > (*(*candidate).lp).epsvalue) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_validSubstitutionVar"]
pub unsafe extern "C" fn validSubstitutionVar(
    mut candidate: *mut pricerec,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*candidate).lp;
    let mut theta: ::core::ffi::c_double = if (*candidate).isdual as ::core::ffi::c_int != 0 {
        fabs((*candidate).theta)
    } else {
        (*candidate).theta
    };
    if fabs((*candidate).pivot) >= (*lp).infinite {
        return (theta < (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        return (theta < (*lp).infinite && fabs((*candidate).pivot) >= (*candidate).epspivot)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_compareImprovementQS"]
pub unsafe extern "C" fn compareImprovementQS(
    mut current: *const QSORTrec,
    mut candidate: *const QSORTrec,
) -> ::core::ffi::c_int {
    return compareImprovementVar(
        (*current).pvoidint2.ptr as *mut pricerec,
        (*candidate).pvoidint2.ptr as *mut pricerec,
    );
}
#[export_name="honest_lpsolve_compareSubstitutionQS"]
pub unsafe extern "C" fn compareSubstitutionQS(
    mut current: *const QSORTrec,
    mut candidate: *const QSORTrec,
) -> ::core::ffi::c_int {
    return compareBoundFlipVar(
        (*current).pvoidint2.ptr as *mut pricerec,
        (*candidate).pvoidint2.ptr as *mut pricerec,
    );
}
#[export_name="honest_lpsolve_addCandidateVar"]
pub unsafe extern "C" fn addCandidateVar(
    mut candidate: *mut pricerec,
    mut multi: *mut multirec,
    mut findCompare: Option<findCompare_func>,
    mut allowSortedExpand: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut insertpos: ::core::ffi::c_int = 0;
    let mut delta: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut targetrec: *mut pricerec = ::core::ptr::null_mut::<pricerec>();
    if *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        || (*multi).sorted as ::core::ffi::c_int != 0
            && allowSortedExpand as ::core::ffi::c_int != 0
        || (*candidate).isdual as ::core::ffi::c_int != 0
            && (*multi).used == 1 as ::core::ffi::c_int
            && ((*multi).step_last >= (*multi).epszero
                || multi_truncatingvar(
                    multi,
                    (*((*(*multi).sortedList.offset(0 as ::core::ffi::c_int as isize))
                        .pvoidreal
                        .ptr as *mut pricerec))
                        .varno,
                ) as ::core::ffi::c_int
                    != 0)
    {
        let mut searchTarget: QSORTrec = QSORTrec {
            pvoid2: QSORTrec1 {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ptr2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        };
        if *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && (*multi).sorted == 0
        {
            (*multi).sorted = QS_execute(
                (*multi).sortedList as *mut QSORTrec,
                (*multi).used,
                findCompare,
                &raw mut insertpos,
            );
            (*multi).dirty =
                (insertpos > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        }
        searchTarget.pvoidint2.ptr = candidate as *mut ::core::ffi::c_void;
        insertpos = ::core::mem::size_of::<QSORTrec>() as ::core::ffi::c_int;
        insertpos = findIndexEx(
            &raw mut searchTarget as *mut ::core::ffi::c_void,
            (*multi).sortedList.offset(-(delta as isize)) as *mut ::core::ffi::c_void,
            (*multi).used,
            delta,
            insertpos,
            findCompare,
            TRUE as ::core::ffi::c_uchar,
        );
        if insertpos > 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        insertpos = -insertpos - delta;
        if insertpos >= (*multi).size
            && *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
            || insertpos == (*multi).used
                && (allowSortedExpand == 0 || (*multi).step_last >= (*multi).epszero)
        {
            return -(1 as ::core::ffi::c_int);
        }
        if *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
            targetrec = (*(*multi)
                .sortedList
                .offset(((*multi).used - 1 as ::core::ffi::c_int) as isize))
            .pvoidreal
            .ptr as *mut pricerec;
        } else {
            let ref mut fresh0 = *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize);
            let fresh1 = *fresh0;
            *fresh0 = *fresh0 - 1;
            delta = fresh1;
            delta = *(*multi).freeList.offset(delta as isize);
            targetrec = (*multi).items.offset(delta as isize) as *mut pricerec;
        }
    } else {
        let ref mut fresh2 = *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize);
        let fresh3 = *fresh2;
        *fresh2 = *fresh2 - 1;
        delta = fresh3;
        delta = *(*multi).freeList.offset(delta as isize);
        targetrec = (*multi).items.offset(delta as isize) as *mut pricerec;
        insertpos = (*multi).used;
    }
    memcpy(
        targetrec as *mut ::core::ffi::c_void,
        candidate as *const ::core::ffi::c_void,
        (1 as ::core::ffi::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<pricerec>() as size_t),
    );
    if (*multi).used < (*multi).size && insertpos >= (*multi).used {
        QS_append(
            (*multi).sortedList as *mut QSORTrec,
            insertpos,
            targetrec as *mut ::core::ffi::c_void,
        );
        (*multi).used += 1;
    } else if (*multi).used == (*multi).size {
        QS_insert(
            (*multi).sortedList as *mut QSORTrec,
            insertpos,
            targetrec as *mut ::core::ffi::c_void,
            (*multi).size - 1 as ::core::ffi::c_int,
        );
    } else {
        QS_insert(
            (*multi).sortedList as *mut QSORTrec,
            insertpos,
            targetrec as *mut ::core::ffi::c_void,
            (*multi).used,
        );
        (*multi).used += 1;
    }
    (*multi).active = insertpos;
    return insertpos;
}
#[export_name="honest_lpsolve_findImprovementVar"]
pub unsafe extern "C" fn findImprovementVar(
    mut current: *mut pricerec,
    mut candidate: *mut pricerec,
    mut collectMP: ::core::ffi::c_uchar,
    mut candidatecount: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut Action: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut Accept: ::core::ffi::c_uchar = validImprovementVar(candidate);
    if Accept != 0 {
        if !candidatecount.is_null() {
            *candidatecount += 1;
        }
        if collectMP != 0 {
            if addCandidateVar(
                candidate,
                (*(*current).lp).multivars,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *const QSORTrec,
                            *const QSORTrec,
                        ) -> ::core::ffi::c_int,
                    >,
                    Option<findCompare_func>,
                >(Some(
                    compareImprovementQS
                        as unsafe extern "C" fn(
                            *const QSORTrec,
                            *const QSORTrec,
                        ) -> ::core::ffi::c_int,
                )),
                FALSE as ::core::ffi::c_uchar,
            ) < 0 as ::core::ffi::c_int
            {
                return Action;
            }
        }
        if (*current).varno > 0 as ::core::ffi::c_int {
            Accept = (compareImprovementVar(current, candidate) > 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
        }
    }
    if Accept != 0 {
        *current = *candidate;
        if (*candidate).isdual == 0 {
            Action = ((*(*candidate).lp)._piv_rule_ == PRICER_FIRSTINDEX) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
        }
    }
    return Action;
}
#[export_name="honest_lpsolve_collectMinorVar"]
pub unsafe extern "C" fn collectMinorVar(
    mut candidate: *mut pricerec,
    mut longsteps: *mut multirec,
    mut isphase2: ::core::ffi::c_uchar,
    mut isbatch: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut inspos: ::core::ffi::c_int = 0;
    if validSubstitutionVar(candidate) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if isbatch == 0
        && (*longsteps).sorted == 0
        && (*longsteps).used > 1 as ::core::ffi::c_int
        && (*(*longsteps)
            .freeList
            .offset(0 as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
            || multi_truncatingvar(longsteps, (*candidate).varno) as ::core::ffi::c_int != 0
            || (*longsteps).step_last >= (*longsteps).epszero)
    {
        (*longsteps).sorted = QS_execute(
            (*longsteps).sortedList as *mut QSORTrec,
            (*longsteps).used,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int,
                >,
                Option<findCompare_func>,
            >(Some(
                compareSubstitutionQS
                    as unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int,
            )),
            &raw mut inspos,
        );
        (*longsteps).dirty =
            (inspos > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if (*longsteps).dirty != 0 {
            multi_recompute(
                longsteps,
                0 as ::core::ffi::c_int,
                isphase2,
                TRUE as ::core::ffi::c_uchar,
            );
        }
    }
    inspos = addCandidateVar(
        candidate,
        longsteps,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int>,
            Option<findCompare_func>,
        >(Some(
            compareSubstitutionQS
                as unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int,
        )),
        TRUE as ::core::ffi::c_uchar,
    );
    return ((inspos >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar
        as ::core::ffi::c_int
        != 0
        && (isbatch as ::core::ffi::c_int == TRUE
            || multi_recompute(longsteps, inspos, isphase2, TRUE as ::core::ffi::c_uchar)
                as ::core::ffi::c_int
                != 0)) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_findSubstitutionVar"]
pub unsafe extern "C" fn findSubstitutionVar(
    mut current: *mut pricerec,
    mut candidate: *mut pricerec,
    mut candidatecount: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut Action: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut Accept: ::core::ffi::c_uchar = validSubstitutionVar(candidate);
    if Accept != 0 {
        if !candidatecount.is_null() {
            *candidatecount += 1;
        }
        if (*current).varno != 0 as ::core::ffi::c_int {
            Accept = (compareSubstitutionVar(current, candidate) > 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
        }
    }
    if Accept != 0 {
        *current = *candidate;
    }
    return Action;
}
#[export_name="honest_lpsolve_partial_createBlocks"]
pub unsafe extern "C" fn partial_createBlocks(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
) -> *mut partialrec {
    let mut blockdata: *mut partialrec = ::core::ptr::null_mut::<partialrec>();
    blockdata =
        calloc(1 as size_t, ::core::mem::size_of::<partialrec>() as size_t) as *mut partialrec;
    (*blockdata).lp = lp;
    (*blockdata).blockcount = 1 as ::core::ffi::c_int;
    (*blockdata).blocknow = 1 as ::core::ffi::c_int;
    (*blockdata).isrow = isrow;
    return blockdata;
}
#[export_name="honest_lpsolve_partial_countBlocks"]
pub unsafe extern "C" fn partial_countBlocks(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut blockdata: *mut partialrec = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rowblocks
    } else {
        (*lp).colblocks
    };
    if blockdata.is_null() {
        return 1 as ::core::ffi::c_int;
    } else {
        return (*blockdata).blockcount;
    };
}
#[export_name="honest_lpsolve_partial_activeBlocks"]
pub unsafe extern "C" fn partial_activeBlocks(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut blockdata: *mut partialrec = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rowblocks
    } else {
        (*lp).colblocks
    };
    if blockdata.is_null() {
        return 1 as ::core::ffi::c_int;
    } else {
        return (*blockdata).blocknow;
    };
}
#[export_name="honest_lpsolve_partial_freeBlocks"]
pub unsafe extern "C" fn partial_freeBlocks(mut blockdata: *mut *mut partialrec) {
    if blockdata.is_null() || (*blockdata).is_null() {
        return;
    }
    if !((**blockdata).blockend as *mut ::core::ffi::c_void).is_null() {
        free((**blockdata).blockend as *mut ::core::ffi::c_void);
        (**blockdata).blockend = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**blockdata).blockpos as *mut ::core::ffi::c_void).is_null() {
        free((**blockdata).blockpos as *mut ::core::ffi::c_void);
        (**blockdata).blockpos = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(*blockdata as *mut ::core::ffi::c_void).is_null() {
        free(*blockdata as *mut ::core::ffi::c_void);
        *blockdata = ::core::ptr::null_mut::<partialrec>();
    }
}
#[export_name="honest_lpsolve_makePriceLoop"]
pub unsafe extern "C" fn makePriceLoop(
    mut lp: *mut lprec,
    mut start: *mut ::core::ffi::c_int,
    mut end: *mut ::core::ffi::c_int,
    mut delta: *mut ::core::ffi::c_int,
) {
    let mut offset: ::core::ffi::c_int = is_piv_mode(lp, PRICE_LOOPLEFT) as ::core::ffi::c_int;
    if offset != 0
        || ((*lp).total_iter + offset as ::core::ffi::c_longlong) % 2 as ::core::ffi::c_longlong
            == 0 as ::core::ffi::c_longlong
            && is_piv_mode(lp, PRICE_LOOPALTERNATE) as ::core::ffi::c_int != 0
    {
        *delta = -(1 as ::core::ffi::c_int);
        swapINT(start, end);
        (*lp)._piv_left_ = TRUE as ::core::ffi::c_uchar;
    } else {
        *delta = 1 as ::core::ffi::c_int;
        (*lp)._piv_left_ = FALSE as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_serious_facterror"]
pub unsafe extern "C" fn serious_facterror(
    mut lp: *mut lprec,
    mut bvector: *mut ::core::ffi::c_double,
    mut maxcols: ::core::ffi::c_int,
    mut tolerance: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut nc: ::core::ffi::c_int = 0;
    let mut sum: ::core::ffi::c_double = 0.;
    let mut tsum: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut err: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut mat: *mut MATrec = (*lp).matA;
    if bvector.is_null() {
        bvector = (*lp).bsolveVal;
    }
    nc = 0 as ::core::ffi::c_int;
    nz = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows && nc <= maxcols {
        j = *(*lp).var_basic.offset(i as isize) - (*lp).rows;
        if !(j <= 0 as ::core::ffi::c_int) {
            nc += 1;
            ib = *(*mat)
                .col_end
                .offset((j - 1 as ::core::ffi::c_int) as isize);
            ie = *(*mat).col_end.offset(j as isize);
            nz += ie - ib;
            sum = get_OF_active(
                lp,
                j + (*lp).rows,
                *bvector.offset(0 as ::core::ffi::c_int as isize),
            );
            while ib < ie {
                sum += *(*mat).col_mat_value.offset(ib as isize)
                    * *bvector.offset(*(*mat).col_mat_rownr.offset(ib as isize) as isize);
                ib += 1;
            }
            tsum += sum;
            if err < fabs(sum) {
                err = fabs(sum);
            }
            if tsum / nc as ::core::ffi::c_double
                > tolerance / 100 as ::core::ffi::c_int as ::core::ffi::c_double
                && err < tolerance / 100 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                break;
            }
        }
        i += 1;
    }
    err /= (*mat).infnorm;
    return (err >= tolerance) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_update_reducedcosts"]
pub unsafe extern "C" fn update_reducedcosts(
    mut lp: *mut lprec,
    mut isdual: ::core::ffi::c_uchar,
    mut leave_nr: ::core::ffi::c_int,
    mut enter_nr: ::core::ffi::c_int,
    mut prow: *mut ::core::ffi::c_double,
    mut drow: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_double = 0.;
    if isdual != 0 {
        hold = -*drow.offset(enter_nr as isize) / *prow.offset(enter_nr as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            if *(*lp).is_basic.offset(i as isize) == 0 {
                if i == leave_nr {
                    *drow.offset(i as isize) = hold;
                } else {
                    *drow.offset(i as isize) += hold * *prow.offset(i as isize);
                    if fabs(*drow.offset(i as isize)) < (*lp).epsmachine {
                        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                }
            }
            i += 1;
        }
    } else {
        report(
            lp,
            2 as ::core::ffi::c_int,
            b"update_reducedcosts: Cannot update primal reduced costs!\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    };
}
#[export_name="honest_lpsolve_compute_reducedcosts"]
pub unsafe extern "C" fn compute_reducedcosts(
    mut lp: *mut lprec,
    mut isdual: ::core::ffi::c_uchar,
    mut row_nr: ::core::ffi::c_int,
    mut coltarget: *mut ::core::ffi::c_int,
    mut dosolve: ::core::ffi::c_uchar,
    mut prow: *mut ::core::ffi::c_double,
    mut nzprow: *mut ::core::ffi::c_int,
    mut drow: *mut ::core::ffi::c_double,
    mut nzdrow: *mut ::core::ffi::c_int,
    mut roundmode: ::core::ffi::c_int,
) {
    let mut epsvalue: ::core::ffi::c_double = (*lp).epsvalue;
    roundmode |= MAT_ROUNDRC;
    if isdual != 0 {
        bsolve_xA2(
            lp,
            coltarget,
            row_nr,
            prow,
            epsvalue,
            nzprow,
            0 as ::core::ffi::c_int,
            drow,
            epsvalue,
            nzdrow,
            roundmode,
        );
    } else {
        let mut bVector: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        if (*lp).multivars.is_null() && (*lp).P1extraDim == 0 as ::core::ffi::c_int {
            bVector = drow;
        } else {
            bVector = (*lp).bsolveVal;
        }
        if dosolve != 0 {
            bsolve(
                lp,
                0 as ::core::ffi::c_int,
                bVector,
                (*lp).bsolveIdx,
                epsvalue * DOUBLEROUND,
                1.0f64,
            );
            if isdual == 0
                && row_nr == 0 as ::core::ffi::c_int
                && (*lp).improve & IMPROVE_SOLUTION != 0
                && refactRecent(lp) == 0
                && serious_facterror(lp, bVector, (*lp).rows, (*lp).epsvalue) as ::core::ffi::c_int
                    != 0
            {
                set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
            }
        }
        prod_xA(
            lp,
            coltarget,
            bVector,
            (*lp).bsolveIdx,
            epsvalue,
            1.0f64,
            drow,
            nzdrow,
            roundmode,
        );
    };
}
#[export_name="honest_lpsolve_verify_stability"]
pub unsafe extern "C" fn verify_stability(
    mut lp: *mut lprec,
    mut isprimal: ::core::ffi::c_uchar,
    mut xfeas: ::core::ffi::c_double,
    mut sfeas: ::core::ffi::c_double,
    mut nfeas: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut testOK: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    return testOK;
}
#[export_name="honest_lpsolve_find_rowReplacement"]
pub unsafe extern "C" fn find_rowReplacement(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut prow: *mut ::core::ffi::c_double,
    mut nzprow: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut bestindex: ::core::ffi::c_int = 0;
    let mut bestvalue: ::core::ffi::c_double = 0.;
    set_action(&raw mut (*lp).piv_strategy, PRICE_FORCEFULL);
    compute_reducedcosts(
        lp,
        TRUE as ::core::ffi::c_uchar,
        rownr,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        TRUE as ::core::ffi::c_uchar,
        prow,
        nzprow,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        MAT_ROUNDDEFAULT,
    );
    clear_action(&raw mut (*lp).piv_strategy, PRICE_FORCEFULL);
    bestindex = 0 as ::core::ffi::c_int;
    bestvalue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).sum - abs((*lp).P1extraDim) {
        if *(*lp).is_basic.offset(i as isize) == 0
            && is_fixedvar(lp, i) == 0
            && fabs(*prow.offset(i as isize)) > bestvalue
        {
            bestindex = i;
            bestvalue = fabs(*prow.offset(i as isize));
        }
        i += 1;
    }
    if i > (*lp).sum - abs((*lp).P1extraDim) {
        bestindex = 0 as ::core::ffi::c_int;
    } else {
        fsolve(
            lp,
            bestindex,
            prow,
            nzprow,
            (*lp).epsmachine,
            1.0f64,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    return bestindex;
}
#[export_name="honest_lpsolve_colprim"]
pub unsafe extern "C" fn colprim(
    mut lp: *mut lprec,
    mut drow: *mut ::core::ffi::c_double,
    mut nzdrow: *mut ::core::ffi::c_int,
    mut skipupdate: ::core::ffi::c_uchar,
    mut partialloop: ::core::ffi::c_int,
    mut candidatecount: *mut ::core::ffi::c_int,
    mut updateinfeas: ::core::ffi::c_uchar,
    mut xviol: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut ninfeas: ::core::ffi::c_int = 0;
    let mut nloop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut f: ::core::ffi::c_double = 0.;
    let mut sinfeas: ::core::ffi::c_double = 0.;
    let mut xinfeas: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*lp).epsdual;
    let mut current: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut candidate: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut collectMP: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut coltarget: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    current.pivot = (*lp).epsprimal;
    current.varno = 0 as ::core::ffi::c_int;
    current.lp = lp;
    current.isdual = FALSE as ::core::ffi::c_uchar;
    candidate.lp = lp;
    candidate.isdual = FALSE as ::core::ffi::c_uchar;
    *candidatecount = 0 as ::core::ffi::c_int;
    (*lp)._piv_rule_ = get_piv_rule(lp);
    loop {
        nloop += 1;
        if !(*lp).multivars.is_null()
            && (*lp).simplex_mode & SIMPLEX_PRIMAL_PRIMAL != 0 as ::core::ffi::c_int
        {
            collectMP = multi_mustupdate((*lp).multivars);
            if collectMP != 0 {
                multi_restart((*lp).multivars);
                coltarget = ::core::ptr::null_mut::<::core::ffi::c_int>();
            } else {
                coltarget = multi_indexSet((*lp).multivars, FALSE as ::core::ffi::c_uchar);
            }
        }
        if skipupdate == 0 {
            compute_reducedcosts(
                lp,
                FALSE as ::core::ffi::c_uchar,
                0 as ::core::ffi::c_int,
                coltarget,
                (nloop <= 1 as ::core::ffi::c_int || partialloop > 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as ::core::ffi::c_uchar,
                ::core::ptr::null_mut::<::core::ffi::c_double>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                drow,
                nzdrow,
                MAT_ROUNDDEFAULT,
            );
        }
        ix = 1 as ::core::ffi::c_int;
        iy = *nzdrow.offset(0 as ::core::ffi::c_int as isize);
        ninfeas = 0 as ::core::ffi::c_int;
        xinfeas = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        sinfeas = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        makePriceLoop(lp, &raw mut ix, &raw mut iy, &raw mut iz);
        iy *= iz;
        let mut current_block_37: u64;
        while ix * iz <= iy {
            i = *nzdrow.offset(ix as isize);
            if *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int
            {
                let mut kk: ::core::ffi::c_int = 0;
                kk = 1 as ::core::ffi::c_int;
                while kk <= *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize)
                    && i != *(*lp).rejectpivot.offset(kk as isize)
                {
                    kk += 1;
                }
                if kk <= *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) {
                    current_block_37 = 5601891728916014340;
                } else {
                    current_block_37 = 13550086250199790493;
                }
            } else {
                current_block_37 = 13550086250199790493;
            }
            match current_block_37 {
                13550086250199790493 => {
                    f = if *(*lp).is_lower.offset(i as isize) as ::core::ffi::c_int != 0
                        && *drow.offset(i as isize)
                            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -*drow.offset(i as isize)
                    } else {
                        *drow.offset(i as isize)
                    };
                    if !(f <= epsvalue) {
                        ninfeas += 1;
                        if xinfeas < f {
                            xinfeas = f;
                        }
                        sinfeas += f;
                        candidate.pivot = normalizeEdge(lp, i, f, FALSE as ::core::ffi::c_uchar);
                        candidate.varno = i;
                        if findImprovementVar(
                            &raw mut current,
                            &raw mut candidate,
                            collectMP,
                            candidatecount,
                        ) != 0
                        {
                            break;
                        }
                    }
                }
                _ => {}
            }
            ix += iz;
        }
        if (*lp).multivars.is_null() {
            current_block = 3160140712158701372;
            break;
        }
        if collectMP != 0 {
            if (*(*lp).multivars).sorted == 0 {
                (*(*lp).multivars).sorted = QS_execute(
                    (*(*lp).multivars).sortedList as *mut QSORTrec,
                    (*(*lp).multivars).used,
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *const QSORTrec,
                                *const QSORTrec,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<findCompare_func>,
                    >(Some(
                        compareImprovementQS
                            as unsafe extern "C" fn(
                                *const QSORTrec,
                                *const QSORTrec,
                            )
                                -> ::core::ffi::c_int,
                    )),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
            coltarget = multi_indexSet((*lp).multivars, TRUE as ::core::ffi::c_uchar);
            current_block = 721385680381463314;
            break;
        } else {
            if !(current.varno == 0 as ::core::ffi::c_int
                && (*(*lp).multivars).retries == 0 as ::core::ffi::c_int)
            {
                current_block = 721385680381463314;
                break;
            }
            ix = partial_blockStart(lp, FALSE as ::core::ffi::c_uchar);
            iy = partial_blockEnd(lp, FALSE as ::core::ffi::c_uchar);
            (*(*lp).multivars).used = 0 as ::core::ffi::c_int;
            (*(*lp).multivars).retries += 1;
        }
    }
    match current_block {
        721385680381463314 => {
            (*(*lp).multivars).retries = 0 as ::core::ffi::c_int;
            if current.varno != 0 as ::core::ffi::c_int {
                multi_removevar((*lp).multivars, current.varno);
            }
        }
        _ => {}
    }
    if !xviol.is_null() {
        *xviol = xinfeas;
    }
    if updateinfeas != 0 {
        (*lp).suminfeas = fabs(sinfeas);
    }
    if (*lp).multivars.is_null()
        && current.varno > 0 as ::core::ffi::c_int
        && verify_stability(lp, TRUE as ::core::ffi::c_uchar, xinfeas, sinfeas, ninfeas) == 0
    {
        current.varno = 0 as ::core::ffi::c_int;
    }
    if (*lp).spx_trace != 0 {
        if current.varno > 0 as ::core::ffi::c_int {
            report(
                lp,
                5 as ::core::ffi::c_int,
                b"colprim: Column %d reduced cost = %18.12g\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            report(
                lp,
                5 as ::core::ffi::c_int,
                b"colprim: No positive reduced costs found, optimality!\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    return current.varno;
}
#[export_name="honest_lpsolve_rowprim"]
pub unsafe extern "C" fn rowprim(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut theta: *mut ::core::ffi::c_double,
    mut pcol: *mut ::core::ffi::c_double,
    mut nzpcol: *mut ::core::ffi::c_int,
    mut forceoutEQ: ::core::ffi::c_uchar,
    mut xviol: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut Hpass: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut nzlist: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut f: ::core::ffi::c_double = 0.;
    let mut savef: ::core::ffi::c_double = 0.;
    let mut Heps: ::core::ffi::c_double = 0.;
    let mut Htheta: ::core::ffi::c_double = 0.;
    let mut Hlimit: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = 0.;
    let mut epspivot: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.0f64;
    let mut current: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut candidate: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut isupper: ::core::ffi::c_uchar =
        (*(*lp).is_lower.offset(colnr as isize) == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    let mut HarrisTwoPass: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    (*lp)._piv_rule_ = get_piv_rule(lp);
    if nzpcol.is_null() {
        nzlist = mempool_obtainVector(
            (*lp).workarrays,
            (*lp).rows + 1 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
    } else {
        nzlist = nzpcol;
    }
    epspivot = (*lp).epspivot;
    epsvalue = (*lp).epsvalue;
    Hlimit = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    Htheta = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    k = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        p = fabs(*pcol.offset(i as isize));
        if p > Hlimit {
            Hlimit = p;
        }
        if p > epsvalue {
            k += 1;
            *nzlist.offset(k as isize) = i;
            if Htheta < p {
                Htheta = p;
            }
        }
        i += 1;
    }
    if !xviol.is_null() {
        *xviol = Htheta;
    }
    Htheta = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    *nzlist.offset(0 as ::core::ffi::c_int as isize) = k;
    k = 0 as ::core::ffi::c_int;
    loop {
        k += 1;
        HarrisTwoPass = is_piv_mode(lp, PRICE_HARRISTWOPASS);
        if HarrisTwoPass != 0 {
            Hpass = 1 as ::core::ffi::c_int;
        } else {
            Hpass = 2 as ::core::ffi::c_int;
        }
        current.theta = (*lp).infinite;
        current.pivot = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        current.varno = 0 as ::core::ffi::c_int;
        current.isdual = FALSE as ::core::ffi::c_uchar;
        current.epspivot = epspivot;
        current.lp = lp;
        candidate.epspivot = epspivot;
        candidate.isdual = FALSE as ::core::ffi::c_uchar;
        candidate.lp = lp;
        savef = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        while Hpass <= 2 as ::core::ffi::c_int {
            Htheta = (*lp).infinite;
            if Hpass == 1 as ::core::ffi::c_int {
                Hlimit = (*lp).infinite;
                Heps = epspivot / (*lp).epsprimal;
            } else {
                Hlimit = current.theta;
                Heps = 0.0f64;
            }
            current.theta = (*lp).infinite;
            current.pivot = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            current.varno = 0 as ::core::ffi::c_int;
            savef = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            ii = 1 as ::core::ffi::c_int;
            iy = *nzlist.offset(0 as ::core::ffi::c_int as isize);
            makePriceLoop(lp, &raw mut ii, &raw mut iy, &raw mut iz);
            iy *= iz;
            while ii * iz <= iy {
                i = *nzlist.offset(ii as isize);
                f = *pcol.offset(i as isize);
                candidate.theta = f;
                candidate.pivot = f;
                candidate.varno = i;
                compute_theta(
                    lp,
                    i,
                    &raw mut candidate.theta,
                    isupper as ::core::ffi::c_int,
                    if *(*lp)
                        .upbo
                        .offset(*(*lp).var_basic.offset(i as isize) as isize)
                        < (*lp).epsprimal
                    {
                        Heps / 10 as ::core::ffi::c_int as ::core::ffi::c_double
                    } else {
                        Heps
                    },
                    TRUE as ::core::ffi::c_uchar,
                );
                if fabs(candidate.theta) >= (*lp).infinite {
                    savef = f;
                    candidate.theta =
                        2 as ::core::ffi::c_int as ::core::ffi::c_double * (*lp).infinite;
                } else if !(Hpass == 2 as ::core::ffi::c_int && candidate.theta > Hlimit) {
                    if forceoutEQ != 0 {
                        p = candidate.pivot;
                        if *(*lp)
                            .upbo
                            .offset(*(*lp).var_basic.offset(i as isize) as isize)
                            < (*lp).epsprimal
                        {
                            if forceoutEQ as ::core::ffi::c_int == AUTOMATIC {
                                candidate.pivot *= 1.0f64 + (*lp).epspivot;
                            } else {
                                candidate.pivot *= 10.0f64;
                            }
                        }
                    }
                    if HarrisTwoPass != 0 {
                        f = candidate.theta;
                        if Hpass == 2 as ::core::ffi::c_int {
                            candidate.theta = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                        }
                        if findSubstitutionVar(
                            &raw mut current,
                            &raw mut candidate,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        ) != 0
                        {
                            break;
                        }
                        if Hpass == 2 as ::core::ffi::c_int && current.varno == candidate.varno {
                            Htheta = f;
                        }
                    } else if findSubstitutionVar(
                        &raw mut current,
                        &raw mut candidate,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ) != 0
                    {
                        break;
                    }
                    if forceoutEQ as ::core::ffi::c_int != 0 && current.varno == candidate.varno {
                        current.pivot = p;
                    }
                }
                ii += iz;
            }
            Hpass += 1;
        }
        if HarrisTwoPass != 0 {
            current.theta = Htheta;
        }
        if current.varno == 0 as ::core::ffi::c_int {
            if *(*lp).upbo.offset(colnr as isize) >= (*lp).infinite {
                if !(k < 2 as ::core::ffi::c_int) {
                    break;
                }
                epspivot = epspivot / 10 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                i = 1 as ::core::ffi::c_int;
                while *pcol.offset(i as isize) >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && i <= (*lp).rows
                {
                    i += 1;
                }
                if i > (*lp).rows {
                    *(*lp).is_lower.offset(colnr as isize) =
                        (*(*lp).is_lower.offset(colnr as isize) == 0) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar;
                    *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) +=
                        *(*lp).upbo.offset(colnr as isize)
                            * *pcol.offset(0 as ::core::ffi::c_int as isize);
                } else {
                    current.varno = i;
                }
                break;
            }
        } else {
            if current.theta >= (*lp).infinite {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"rowprim: Numeric instability pcol[%d] = %g, rhs[%d] = %g, upbo = %g\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            break;
        }
    }
    if nzpcol.is_null() {
        mempool_releaseVector(
            (*lp).workarrays,
            nzlist as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    if (*lp).spx_trace != 0 {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"row_prim: %d, pivot size = %18.12g\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    *theta = fabs(current.theta);
    return current.varno;
}
#[export_name="honest_lpsolve_rowdual"]
pub unsafe extern "C" fn rowdual(
    mut lp: *mut lprec,
    mut rhvec: *mut ::core::ffi::c_double,
    mut forceoutEQ: ::core::ffi::c_uchar,
    mut updateinfeas: ::core::ffi::c_uchar,
    mut xviol: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut ninfeas: ::core::ffi::c_int = 0;
    let mut rh: ::core::ffi::c_double = 0.;
    let mut up: ::core::ffi::c_double = 0.;
    let mut lo: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut epsvalue: ::core::ffi::c_double = 0.;
    let mut sinfeas: ::core::ffi::c_double = 0.;
    let mut xinfeas: ::core::ffi::c_double = 0.;
    let mut current: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut candidate: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut collectMP: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if rhvec.is_null() {
        rhvec = (*lp).rhs;
    }
    epsvalue = (*lp).epsdual;
    current.pivot = -epsvalue;
    current.theta = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    current.varno = 0 as ::core::ffi::c_int;
    current.isdual = TRUE as ::core::ffi::c_uchar;
    current.lp = lp;
    candidate.isdual = TRUE as ::core::ffi::c_uchar;
    candidate.lp = lp;
    if is_action((*lp).piv_strategy, PRICE_FORCEFULL) != 0 {
        k = 1 as ::core::ffi::c_int;
        iy = (*lp).rows;
    } else {
        k = partial_blockStart(lp, TRUE as ::core::ffi::c_uchar);
        iy = partial_blockEnd(lp, TRUE as ::core::ffi::c_uchar);
    }
    ninfeas = 0 as ::core::ffi::c_int;
    xinfeas = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    sinfeas = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    makePriceLoop(lp, &raw mut k, &raw mut iy, &raw mut iz);
    iy *= iz;
    let mut current_block_44: u64;
    while k * iz <= iy {
        i = k;
        if *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int {
            let mut kk: ::core::ffi::c_int = 0;
            kk = 1 as ::core::ffi::c_int;
            while kk <= *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize)
                && i != *(*lp).rejectpivot.offset(kk as isize)
            {
                kk += 1;
            }
            if kk <= *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) {
                current_block_44 = 4956146061682418353;
            } else {
                current_block_44 = 10652014663920648156;
            }
        } else {
            current_block_44 = 10652014663920648156;
        }
        match current_block_44 {
            10652014663920648156 => {
                ii = *(*lp).var_basic.offset(i as isize);
                up = *(*lp).upbo.offset(ii as isize);
                lo = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                rh = *rhvec.offset(i as isize);
                if rh > up {
                    rh = up - rh;
                } else {
                    rh -= lo;
                }
                up -= lo;
                if rh < -epsvalue || forceoutEQ as ::core::ffi::c_int == TRUE && up < epsvalue {
                    ninfeas += 1;
                    if xinfeas > rh {
                        xinfeas = rh;
                    }
                    sinfeas += rh;
                    if up < epsvalue {
                        if forceoutEQ as ::core::ffi::c_int == TRUE {
                            current.varno = i;
                            current.pivot = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                            break;
                        } else if forceoutEQ as ::core::ffi::c_int == AUTOMATIC {
                            rh *= 10.0f64;
                        } else {
                            rh *= 1.0f64 + (*lp).epspivot;
                        }
                    }
                    candidate.pivot = normalizeEdge(lp, i, rh, TRUE as ::core::ffi::c_uchar);
                    candidate.varno = i;
                    if findImprovementVar(
                        &raw mut current,
                        &raw mut candidate,
                        collectMP,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ) != 0
                    {
                        break;
                    }
                }
            }
            _ => {}
        }
        k += iz;
    }
    if updateinfeas != 0 {
        (*lp).suminfeas = fabs(sinfeas);
    }
    if ninfeas > 1 as ::core::ffi::c_int
        && verify_stability(lp, FALSE as ::core::ffi::c_uchar, xinfeas, sinfeas, ninfeas) == 0
    {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"rowdual: Check for reduced accuracy and tolerance settings.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        current.varno = 0 as ::core::ffi::c_int;
    }
    if (*lp).spx_trace != 0 {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"rowdual: Infeasibility sum %18.12g in %7d constraints.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if current.varno > 0 as ::core::ffi::c_int {
            report(
                lp,
                5 as ::core::ffi::c_int,
                b"rowdual: rhs[%d] = %18.12g\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            report(
                lp,
                6 as ::core::ffi::c_int,
                b"rowdual: Optimality - No primal infeasibilities found\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    if !xviol.is_null() {
        *xviol = fabs(xinfeas);
    }
    return current.varno;
}
#[export_name="honest_lpsolve_longdual_testset"]
pub unsafe extern "C" fn longdual_testset(
    mut lp: *mut lprec,
    mut which: ::core::ffi::c_int,
    mut rownr: ::core::ffi::c_int,
    mut prow: *mut ::core::ffi::c_double,
    mut nzprow: *mut ::core::ffi::c_int,
    mut drow: *mut ::core::ffi::c_double,
    mut nzdrow: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut F: ::core::ffi::c_double = (*lp).infinite;
    if which == 0 as ::core::ffi::c_int {
        j = 1 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        j = 2 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 3 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 4 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 3 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = -(6 as ::core::ffi::c_int) as ::core::ffi::c_double;
        j = 5 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(4 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        j = 6 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 7 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 8 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 9 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 4 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 10 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = F;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 10 as ::core::ffi::c_int as ::core::ffi::c_double;
        *nzprow.offset(0 as ::core::ffi::c_int as isize) = i - (*lp).rows;
        *(*lp).rhs.offset(rownr as isize) = -(11 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *(*lp)
            .upbo
            .offset(*(*lp).var_basic.offset(rownr as isize) as isize) = F;
        *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) =
            1 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if which == 1 as ::core::ffi::c_int {
        j = 1 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        j = 2 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 3 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(4 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        j = 4 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 5 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 6 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 7 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 8 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 3 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = -(6 as ::core::ffi::c_int) as ::core::ffi::c_double;
        j = 9 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 4 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 10 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = F;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 10 as ::core::ffi::c_int as ::core::ffi::c_double;
        *nzprow.offset(0 as ::core::ffi::c_int as isize) = i - (*lp).rows;
        *(*lp).rhs.offset(rownr as isize) = -(11 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *(*lp)
            .upbo
            .offset(*(*lp).var_basic.offset(rownr as isize) as isize) = F;
        *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) =
            1 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if which == 10 as ::core::ffi::c_int {
        j = 1 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 2 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 3 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = 3 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 3 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 4 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        *drow.offset(i as isize) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double;
        j = 5 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 6 as ::core::ffi::c_int;
        i = (*lp).rows + j;
        *(*lp).upbo.offset(i as isize) = F;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *nzprow.offset(j as isize) = i;
        *prow.offset(i as isize) = 3 as ::core::ffi::c_int as ::core::ffi::c_double;
        *drow.offset(i as isize) = 9 as ::core::ffi::c_int as ::core::ffi::c_double;
        *nzprow.offset(0 as ::core::ffi::c_int as isize) = i - (*lp).rows;
        *(*lp).rhs.offset(rownr as isize) = 14 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp)
            .upbo
            .offset(*(*lp).var_basic.offset(rownr as isize) as isize) =
            2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) =
            6 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
}
#[export_name="honest_lpsolve_coldual"]
pub unsafe extern "C" fn coldual(
    mut lp: *mut lprec,
    mut row_nr: ::core::ffi::c_int,
    mut prow: *mut ::core::ffi::c_double,
    mut nzprow: *mut ::core::ffi::c_int,
    mut drow: *mut ::core::ffi::c_double,
    mut nzdrow: *mut ::core::ffi::c_int,
    mut dualphase1: ::core::ffi::c_uchar,
    mut skipupdate: ::core::ffi::c_uchar,
    mut candidatecount: *mut ::core::ffi::c_int,
    mut xviol: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut nbound: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_double = 0.;
    let mut g: ::core::ffi::c_double = 0.;
    let mut quot: ::core::ffi::c_double = 0.;
    let mut viol: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.;
    let mut epspivot: ::core::ffi::c_double = (*lp).epspivot;
    let mut epsvalue: ::core::ffi::c_double = (*lp).epsvalue;
    let mut current: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut candidate: pricerec = pricerec {
        theta: 0.,
        pivot: 0.,
        epspivot: 0.,
        varno: 0,
        lp: ::core::ptr::null_mut::<lprec>(),
        isdual: 0,
    };
    let mut isbatch: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut dolongsteps: ::core::ffi::c_uchar =
        ((*lp).longsteps != NULL as *mut multirec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if !xviol.is_null() {
        *xviol = (*lp).infinite;
    }
    if dolongsteps as ::core::ffi::c_int != 0 && dualphase1 == 0 {
        dolongsteps = AUTOMATIC as ::core::ffi::c_uchar;
    }
    current.theta = (*lp).infinite;
    current.pivot = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    current.varno = 0 as ::core::ffi::c_int;
    current.epspivot = epspivot;
    current.isdual = TRUE as ::core::ffi::c_uchar;
    current.lp = lp;
    candidate.epspivot = epspivot;
    candidate.isdual = TRUE as ::core::ffi::c_uchar;
    candidate.lp = lp;
    *candidatecount = 0 as ::core::ffi::c_int;
    if skipupdate == 0 {
        compute_reducedcosts(
            lp,
            TRUE as ::core::ffi::c_uchar,
            row_nr,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            TRUE as ::core::ffi::c_uchar,
            prow,
            nzprow,
            drow,
            nzdrow,
            MAT_ROUNDDEFAULT,
        );
    }
    g = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    viol = *(*lp).rhs.offset(row_nr as isize);
    if viol > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        p = *(*lp)
            .upbo
            .offset(*(*lp).var_basic.offset(row_nr as isize) as isize);
        if p < (*lp).infinite {
            viol -= p;
            if fabs(viol) < epsvalue {
                viol = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if viol > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                g = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
            }
        }
        if g == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            if viol >= (*lp).infinite {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"coldual: Large basic solution value %g at iter %.0f indicates numerical instability\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                (*lp).spx_status = NUMFAILURE;
                return 0 as ::core::ffi::c_int;
            }
            if skipupdate != 0 {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"coldual: Inaccurate bound-flip accuracy at iter %.0f\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"coldual: Leaving variable %d does not violate bounds at iter %.0f\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*lp)._piv_rule_ = get_piv_rule(lp);
    p = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    k = 0 as ::core::ffi::c_int;
    nbound = 0 as ::core::ffi::c_int;
    ix = 1 as ::core::ffi::c_int;
    iy = *nzprow.offset(0 as ::core::ffi::c_int as isize);
    ix = 1 as ::core::ffi::c_int;
    while ix <= iy {
        i = *nzprow.offset(ix as isize);
        w = *prow.offset(i as isize) * g;
        w = if *(*lp).is_lower.offset(i as isize) == 0
            && w != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -w
        } else {
            w
        };
        if w < -epsvalue {
            if *(*lp).upbo.offset(i as isize) < (*lp).infinite {
                nbound += 1;
            }
            k += 1;
            *nzprow.offset(k as isize) = i;
            if p < -w {
                p = -w;
            }
        }
        ix += 1;
    }
    *nzprow.offset(0 as ::core::ffi::c_int as isize) = k;
    if !xviol.is_null() {
        *xviol = p;
    }
    current.epspivot = epspivot;
    candidate.epspivot = epspivot;
    if dolongsteps != 0 {
        if *nzprow.offset(0 as ::core::ffi::c_int as isize) <= 1 as ::core::ffi::c_int
            || nbound == 0 as ::core::ffi::c_int
        {
            dolongsteps = FALSE as ::core::ffi::c_uchar;
            *(*(*lp).longsteps)
                .indexSet
                .offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
        } else {
            multi_restart((*lp).longsteps);
            multi_valueInit(
                (*lp).longsteps,
                g * viol,
                *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize),
            );
        }
    }
    ix = 1 as ::core::ffi::c_int;
    iy = *nzprow.offset(0 as ::core::ffi::c_int as isize);
    makePriceLoop(lp, &raw mut ix, &raw mut iy, &raw mut iz);
    iy *= iz;
    while ix * iz <= iy {
        i = *nzprow.offset(ix as isize);
        w = *prow.offset(i as isize) * g;
        quot = -*drow.offset(i as isize) / w;
        candidate.theta = quot;
        candidate.pivot = w;
        candidate.varno = i;
        if dolongsteps != 0 {
            if isbatch as ::core::ffi::c_int != 0 && ix == iy {
                isbatch = AUTOMATIC as ::core::ffi::c_uchar;
            }
            if collectMinorVar(
                &raw mut candidate,
                (*lp).longsteps,
                (dolongsteps as ::core::ffi::c_int == AUTOMATIC) as ::core::ffi::c_int
                    as ::core::ffi::c_uchar,
                isbatch,
            ) as ::core::ffi::c_int
                != 0
                && (*lp).spx_trace as ::core::ffi::c_int != 0
            {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"coldual: Long-dual break point with %d bound-flip variables\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if (*lp).spx_status == FATHOMED {
                return 0 as ::core::ffi::c_int;
            }
        } else if findSubstitutionVar(&raw mut current, &raw mut candidate, candidatecount) != 0 {
            break;
        }
        ix += iz;
    }
    if dolongsteps != 0 {
        *candidatecount = (*(*lp).longsteps).used;
        i = multi_enteringvar(
            (*lp).longsteps,
            ::core::ptr::null_mut::<pricerec>(),
            3 as ::core::ffi::c_int,
        );
    } else {
        i = current.varno;
    }
    if (*lp).spx_trace != 0 {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"coldual: Entering column %d, reduced cost %g, pivot value %g, bound swaps %d\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return i;
}
#[export_name="honest_lpsolve_normalizeEdge"]
pub unsafe extern "C" fn normalizeEdge(
    mut lp: *mut lprec,
    mut item: ::core::ffi::c_int,
    mut edge: ::core::ffi::c_double,
    mut isdual: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    if fabs(edge) > (*lp).epssolution {
        edge /= getPricer(lp, item, isdual);
    }
    if (*lp).piv_strategy & PRICE_RANDOMIZE != 0 as ::core::ffi::c_int {
        edge *= 1.0f64 - PRICER_RANDFACT + PRICER_RANDFACT * rand_uniform(lp, 1.0f64);
    }
    return edge;
}
#[export_name="honest_lpsolve_partial_findBlocks"]
pub unsafe extern "C" fn partial_findBlocks(
    mut lp: *mut lprec,
    mut autodefine: ::core::ffi::c_uchar,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nb: ::core::ffi::c_int = 0;
    let mut ne: ::core::ffi::c_int = 0;
    let mut items: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_double = 0.;
    let mut biggest: ::core::ffi::c_double = 0.;
    let mut sum: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut mat: *mut MATrec = (*lp).matA;
    if mat_validate(mat) == 0 {
        return 1 as ::core::ffi::c_int;
    }
    items = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rows
    } else {
        (*lp).columns
    };
    allocREAL(
        lp,
        &raw mut sum,
        items + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    *sum.offset(0 as ::core::ffi::c_int as isize) =
        0 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 1 as ::core::ffi::c_int;
    while i <= items {
        n = 0 as ::core::ffi::c_int;
        if isrow != 0 {
            nb = *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            ne = *(*mat).row_end.offset(i as isize);
        } else {
            nb = *(*mat)
                .col_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            ne = *(*mat).col_end.offset(i as isize);
        }
        n = ne - nb;
        *sum.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        if n > 0 as ::core::ffi::c_int {
            if isrow != 0 {
                jj = nb;
                while jj < ne {
                    *sum.offset(i as isize) += *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(jj as isize) as isize)
                        as ::core::ffi::c_double;
                    jj += 1;
                }
            } else {
                jj = nb;
                while jj < ne {
                    *sum.offset(i as isize) +=
                        *(*mat).col_mat_rownr.offset(jj as isize) as ::core::ffi::c_double;
                    jj += 1;
                }
            }
            *sum.offset(i as isize) /= n as ::core::ffi::c_double;
        } else {
            *sum.offset(i as isize) = *sum.offset((i - 1 as ::core::ffi::c_int) as isize);
        }
        i += 1;
    }
    hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    biggest = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 2 as ::core::ffi::c_int;
    while i <= items {
        hold = *sum.offset(i as isize) - *sum.offset((i - 1 as ::core::ffi::c_int) as isize);
        if hold > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            if hold > biggest {
                biggest = hold;
            }
        } else {
            hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        *sum.offset((i - 1 as ::core::ffi::c_int) as isize) = hold;
        i += 1;
    }
    biggest = if 1 as ::core::ffi::c_int as ::core::ffi::c_double > 0.9f64 * biggest {
        1 as ::core::ffi::c_int as ::core::ffi::c_double
    } else {
        0.9f64 * biggest
    };
    n = 0 as ::core::ffi::c_int;
    nb = 0 as ::core::ffi::c_int;
    ne = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < items {
        if *sum.offset(i as isize) > biggest {
            ne += i - nb;
            nb = i;
            n += 1;
        }
        i += 1;
    }
    if !(sum as *mut ::core::ffi::c_void).is_null() {
        free(sum as *mut ::core::ffi::c_void);
        sum = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if n > 0 as ::core::ffi::c_int {
        ne /= n;
        i = if isrow as ::core::ffi::c_int != 0 {
            (*lp).columns
        } else {
            (*lp).rows
        };
        nb = i / ne;
        if abs(nb - n) > 2 as ::core::ffi::c_int {
            n = 1 as ::core::ffi::c_int;
        } else if autodefine != 0 {
            set_partialprice(lp, nb, ::core::ptr::null_mut::<::core::ffi::c_int>(), isrow);
        }
    } else {
        n = 1 as ::core::ffi::c_int;
    }
    return n;
}
#[export_name="honest_lpsolve_partial_blockStart"]
pub unsafe extern "C" fn partial_blockStart(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut blockdata: *mut partialrec = ::core::ptr::null_mut::<partialrec>();
    blockdata = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rowblocks
    } else {
        (*lp).colblocks
    };
    if blockdata.is_null() {
        return 1 as ::core::ffi::c_int;
    } else {
        if (*blockdata).blocknow < 1 as ::core::ffi::c_int
            || (*blockdata).blocknow > (*blockdata).blockcount
        {
            (*blockdata).blocknow = 1 as ::core::ffi::c_int;
        }
        return *(*blockdata)
            .blockend
            .offset(((*blockdata).blocknow - 1 as ::core::ffi::c_int) as isize);
    };
}
#[export_name="honest_lpsolve_partial_blockEnd"]
pub unsafe extern "C" fn partial_blockEnd(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut blockdata: *mut partialrec = ::core::ptr::null_mut::<partialrec>();
    blockdata = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rowblocks
    } else {
        (*lp).colblocks
    };
    if blockdata.is_null() {
        return if isrow as ::core::ffi::c_int != 0 {
            (*lp).rows
        } else {
            (*lp).sum
        };
    } else {
        if (*blockdata).blocknow < 1 as ::core::ffi::c_int
            || (*blockdata).blocknow > (*blockdata).blockcount
        {
            (*blockdata).blocknow = 1 as ::core::ffi::c_int;
        }
        return *(*blockdata).blockend.offset((*blockdata).blocknow as isize)
            - 1 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_partial_blockNextPos"]
pub unsafe extern "C" fn partial_blockNextPos(
    mut lp: *mut lprec,
    mut block: ::core::ffi::c_int,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut blockdata: *mut partialrec = ::core::ptr::null_mut::<partialrec>();
    blockdata = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rowblocks
    } else {
        (*lp).colblocks
    };
    block -= 1;
    if *(*blockdata).blockpos.offset(block as isize)
        == *(*blockdata)
            .blockend
            .offset((block + 1 as ::core::ffi::c_int) as isize)
    {
        *(*blockdata).blockpos.offset(block as isize) =
            *(*blockdata).blockend.offset(block as isize);
    } else {
        let ref mut fresh6 = *(*blockdata).blockpos.offset(block as isize);
        *fresh6 += 1;
    }
    return *(*blockdata).blockpos.offset(block as isize);
}
#[export_name="honest_lpsolve_partial_blockStep"]
pub unsafe extern "C" fn partial_blockStep(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut blockdata: *mut partialrec = ::core::ptr::null_mut::<partialrec>();
    blockdata = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rowblocks
    } else {
        (*lp).colblocks
    };
    if blockdata.is_null() {
        return 0 as ::core::ffi::c_uchar;
    } else if (*blockdata).blocknow < (*blockdata).blockcount {
        (*blockdata).blocknow += 1;
        return 1 as ::core::ffi::c_uchar;
    } else {
        (*blockdata).blocknow = 1 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_partial_isVarActive"]
pub unsafe extern "C" fn partial_isVarActive(
    mut lp: *mut lprec,
    mut varno: ::core::ffi::c_int,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut blockdata: *mut partialrec = ::core::ptr::null_mut::<partialrec>();
    blockdata = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rowblocks
    } else {
        (*lp).colblocks
    };
    if blockdata.is_null() {
        return 1 as ::core::ffi::c_uchar;
    } else {
        return (varno
            >= *(*blockdata)
                .blockend
                .offset(((*blockdata).blocknow - 1 as ::core::ffi::c_int) as isize)
            && varno < *(*blockdata).blockend.offset((*blockdata).blocknow as isize))
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_multi_create"]
pub unsafe extern "C" fn multi_create(
    mut lp: *mut lprec,
    mut truncinf: ::core::ffi::c_uchar,
) -> *mut multirec {
    let mut multi: *mut multirec = ::core::ptr::null_mut::<multirec>();
    multi = calloc(1 as size_t, ::core::mem::size_of::<multirec>() as size_t) as *mut multirec;
    if !multi.is_null() {
        (*multi).active = 1 as ::core::ffi::c_int;
        (*multi).lp = lp;
        (*multi).epszero = (*lp).epsprimal;
        (*multi).truncinf = truncinf;
    }
    return multi;
}
#[export_name="honest_lpsolve_multi_free"]
pub unsafe extern "C" fn multi_free(mut multi: *mut *mut multirec) {
    if multi.is_null() || (*multi).is_null() {
        return;
    }
    if !((**multi).items as *mut ::core::ffi::c_void).is_null() {
        free((**multi).items as *mut ::core::ffi::c_void);
        (**multi).items = ::core::ptr::null_mut::<pricerec>();
    }
    if !((**multi).valueList as *mut ::core::ffi::c_void).is_null() {
        free((**multi).valueList as *mut ::core::ffi::c_void);
        (**multi).valueList = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**multi).indexSet as *mut ::core::ffi::c_void).is_null() {
        free((**multi).indexSet as *mut ::core::ffi::c_void);
        (**multi).indexSet = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**multi).freeList as *mut ::core::ffi::c_void).is_null() {
        free((**multi).freeList as *mut ::core::ffi::c_void);
        (**multi).freeList = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**multi).sortedList as *mut ::core::ffi::c_void).is_null() {
        free((**multi).sortedList as *mut ::core::ffi::c_void);
        (**multi).sortedList = ::core::ptr::null_mut::<QSORTrec>();
    }
    if !(*multi as *mut ::core::ffi::c_void).is_null() {
        free(*multi as *mut ::core::ffi::c_void);
        *multi = ::core::ptr::null_mut::<multirec>();
    }
}
#[export_name="honest_lpsolve_multi_mustupdate"]
pub unsafe extern "C" fn multi_mustupdate(mut multi: *mut multirec) -> ::core::ffi::c_uchar {
    return (!multi.is_null() && (*multi).used < (*multi).limit) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_multi_resize"]
pub unsafe extern "C" fn multi_resize(
    mut multi: *mut multirec,
    mut blocksize: ::core::ffi::c_int,
    mut blockdiv: ::core::ffi::c_int,
    mut doVlist: ::core::ffi::c_uchar,
    mut doIset: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut current_block_47: u64;
    if blocksize > 1 as ::core::ffi::c_int && blockdiv > 0 as ::core::ffi::c_int {
        let mut oldsize: ::core::ffi::c_int = (*multi).size;
        (*multi).size = blocksize;
        if blockdiv > 1 as ::core::ffi::c_int {
            (*multi).limit += ((*multi).size - oldsize) / blockdiv;
        }
        (*multi).items = realloc(
            (*multi).items as *mut ::core::ffi::c_void,
            (((*multi).size + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<pricerec>() as size_t),
        ) as *mut pricerec;
        (*multi).sortedList = realloc(
            (*multi).sortedList as *mut ::core::ffi::c_void,
            (((*multi).size + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<QSORTrec>() as size_t),
        ) as *mut QSORTrec;
        ok = (!(*multi).items.is_null()
            && !(*multi).sortedList.is_null()
            && allocINT(
                (*multi).lp,
                &raw mut (*multi).freeList,
                (*multi).size + 1 as ::core::ffi::c_int,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if ok != 0 {
            let mut i: ::core::ffi::c_int = 0;
            let mut n: ::core::ffi::c_int = 0;
            if oldsize == 0 as ::core::ffi::c_int {
                i = 0 as ::core::ffi::c_int;
            } else {
                i = *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize);
            }
            *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize) =
                i + ((*multi).size - oldsize);
            n = (*multi).size - 1 as ::core::ffi::c_int;
            i += 1;
            while i <= *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize) {
                *(*multi).freeList.offset(i as isize) = n;
                i += 1;
                n -= 1;
            }
        }
        if doVlist != 0 {
            ok = (ok as ::core::ffi::c_int
                & allocREAL(
                    (*multi).lp,
                    &raw mut (*multi).valueList,
                    (*multi).size + 1 as ::core::ffi::c_int,
                    AUTOMATIC as ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        if doIset != 0 {
            ok = (ok as ::core::ffi::c_int
                & allocINT(
                    (*multi).lp,
                    &raw mut (*multi).indexSet,
                    (*multi).size + 1 as ::core::ffi::c_int,
                    AUTOMATIC as ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            if ok as ::core::ffi::c_int != 0 && oldsize == 0 as ::core::ffi::c_int {
                *(*multi).indexSet.offset(0 as ::core::ffi::c_int as isize) =
                    0 as ::core::ffi::c_int;
            }
        }
        if ok == 0 {
            current_block_47 = 12565474829857245305;
        } else {
            current_block_47 = 6450597802325118133;
        }
    } else {
        current_block_47 = 12565474829857245305;
    }
    match current_block_47 {
        12565474829857245305 => {
            (*multi).size = 0 as ::core::ffi::c_int;
            if !((*multi).items as *mut ::core::ffi::c_void).is_null() {
                free((*multi).items as *mut ::core::ffi::c_void);
                (*multi).items = ::core::ptr::null_mut::<pricerec>();
            }
            if !((*multi).valueList as *mut ::core::ffi::c_void).is_null() {
                free((*multi).valueList as *mut ::core::ffi::c_void);
                (*multi).valueList = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !((*multi).indexSet as *mut ::core::ffi::c_void).is_null() {
                free((*multi).indexSet as *mut ::core::ffi::c_void);
                (*multi).indexSet = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
            if !((*multi).freeList as *mut ::core::ffi::c_void).is_null() {
                free((*multi).freeList as *mut ::core::ffi::c_void);
                (*multi).freeList = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
            if !((*multi).sortedList as *mut ::core::ffi::c_void).is_null() {
                free((*multi).sortedList as *mut ::core::ffi::c_void);
                (*multi).sortedList = ::core::ptr::null_mut::<QSORTrec>();
            }
        }
        _ => {}
    }
    (*multi).active = 1 as ::core::ffi::c_int;
    return ok;
}
#[export_name="honest_lpsolve_multi_size"]
pub unsafe extern "C" fn multi_size(mut multi: *mut multirec) -> ::core::ffi::c_int {
    if multi.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return (*multi).size;
    };
}
#[export_name="honest_lpsolve_multi_used"]
pub unsafe extern "C" fn multi_used(mut multi: *mut multirec) -> ::core::ffi::c_int {
    if multi.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return (*multi).used;
    };
}
#[export_name="honest_lpsolve_multi_restart"]
pub unsafe extern "C" fn multi_restart(mut multi: *mut multirec) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = (*multi).used;
    (*multi).used = 0 as ::core::ffi::c_int;
    (*multi).sorted = FALSE as ::core::ffi::c_uchar;
    (*multi).dirty = FALSE as ::core::ffi::c_uchar;
    if !(*multi).freeList.is_null() {
        i = 1 as ::core::ffi::c_int;
        while i <= (*multi).size {
            *(*multi).freeList.offset(i as isize) = (*multi).size - i;
            i += 1;
        }
        *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize) = (*multi).size;
    }
    return n;
}
#[export_name="honest_lpsolve_multi_valueInit"]
pub unsafe extern "C" fn multi_valueInit(
    mut multi: *mut multirec,
    mut step_base: ::core::ffi::c_double,
    mut obj_base: ::core::ffi::c_double,
) {
    (*multi).step_last = step_base;
    (*multi).step_base = (*multi).step_last;
    (*multi).obj_last = obj_base;
    (*multi).obj_base = (*multi).obj_last;
}
#[export_name="honest_lpsolve_multi_valueList"]
pub unsafe extern "C" fn multi_valueList(mut multi: *mut multirec) -> *mut ::core::ffi::c_double {
    return (*multi).valueList;
}
#[export_name="honest_lpsolve_multi_indexSet"]
pub unsafe extern "C" fn multi_indexSet(
    mut multi: *mut multirec,
    mut regenerate: ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_int {
    if regenerate != 0 {
        multi_populateSet(
            multi,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
            -(1 as ::core::ffi::c_int),
        );
    }
    return (*multi).indexSet;
}
#[export_name="honest_lpsolve_multi_getvar"]
pub unsafe extern "C" fn multi_getvar(
    mut multi: *mut multirec,
    mut item: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*(&raw mut (*(*multi).sortedList.offset(item as isize)).pvoidreal.ptr
        as *mut pricerec))
        .varno;
}
#[export_name="honest_lpsolve_multi_recompute"]
pub unsafe extern "C" fn multi_recompute(
    mut multi: *mut multirec,
    mut index: ::core::ffi::c_int,
    mut isphase2: ::core::ffi::c_uchar,
    mut fullupdate: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut lB: ::core::ffi::c_double = 0.;
    let mut uB: ::core::ffi::c_double = 0.;
    let mut Alpha: ::core::ffi::c_double = 0.;
    let mut this_theta: ::core::ffi::c_double = 0.;
    let mut prev_theta: ::core::ffi::c_double = 0.;
    let mut lp: *mut lprec = (*multi).lp;
    let mut thisprice: *mut pricerec = ::core::ptr::null_mut::<pricerec>();
    if (*multi).dirty != 0 {
        index = 0 as ::core::ffi::c_int;
        n = (*multi).used - 1 as ::core::ffi::c_int;
    } else if fullupdate != 0 {
        n = (*multi).used - 1 as ::core::ffi::c_int;
    } else {
        n = index;
    }
    if index == 0 as ::core::ffi::c_int {
        (*multi).maxpivot = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*multi).maxbound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*multi).step_last = (*multi).step_base;
        (*multi).obj_last = (*multi).obj_base;
        thisprice = ::core::ptr::null_mut::<pricerec>();
        this_theta = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        (*multi).obj_last = *(*multi)
            .valueList
            .offset((index - 1 as ::core::ffi::c_int) as isize);
        (*multi).step_last = (*(*multi)
            .sortedList
            .offset((index - 1 as ::core::ffi::c_int) as isize))
        .pvoidreal
        .realval;
        thisprice = (*(*multi)
            .sortedList
            .offset((index - 1 as ::core::ffi::c_int) as isize))
        .pvoidreal
        .ptr as *mut pricerec;
        this_theta = (*thisprice).theta;
    }
    while index <= n && (*multi).step_last < (*multi).epszero {
        prev_theta = this_theta;
        thisprice = (*(*multi).sortedList.offset(index as isize)).pvoidreal.ptr as *mut pricerec;
        this_theta = (*thisprice).theta;
        Alpha = fabs((*thisprice).pivot);
        uB = *(*lp).upbo.offset((*thisprice).varno as isize);
        lB = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        if (*multi).maxpivot < Alpha {
            (*multi).maxpivot = Alpha;
        }
        if (*multi).maxbound < uB {
            (*multi).maxbound = uB;
        }
        if isphase2 != 0 {
            (*multi).obj_last += (this_theta - prev_theta) * (*multi).step_last;
            if uB >= (*lp).infinite {
                (*multi).step_last = (*lp).infinite;
            } else {
                (*multi).step_last += Alpha * (uB - lB);
            }
        } else {
            (*multi).obj_last += (this_theta - prev_theta) * (*multi).step_last;
            (*multi).step_last += Alpha;
        }
        (*(*multi).sortedList.offset(index as isize))
            .pvoidreal
            .realval = (*multi).step_last;
        *(*multi).valueList.offset(index as isize) = (*multi).obj_last;
        index += 1;
    }
    n = index;
    while n < (*multi).used {
        let ref mut fresh4 = *(*multi).freeList.offset(0 as ::core::ffi::c_int as isize);
        *fresh4 += 1;
        i = *fresh4;
        *(*multi).freeList.offset(i as isize) =
            ((*(*multi).sortedList.offset(n as isize)).pvoidreal.ptr as *mut pricerec)
                .offset_from((*multi).items) as ::core::ffi::c_long
                as ::core::ffi::c_int;
        n += 1;
    }
    (*multi).used = index;
    if (*multi).sorted as ::core::ffi::c_int != 0 && index == 1 as ::core::ffi::c_int {
        (*multi).sorted = FALSE as ::core::ffi::c_uchar;
    }
    (*multi).dirty = FALSE as ::core::ffi::c_uchar;
    return ((*multi).step_last >= (*multi).epszero) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_multi_truncatingvar"]
pub unsafe extern "C" fn multi_truncatingvar(
    mut multi: *mut multirec,
    mut varnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return ((*multi).truncinf as ::core::ffi::c_int != 0
        && is_infinite((*multi).lp, *(*(*multi).lp).upbo.offset(varnr as isize))
            as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_multi_removevar"]
pub unsafe extern "C" fn multi_removevar(
    mut multi: *mut multirec,
    mut varnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut coltarget: *mut ::core::ffi::c_int = (*multi).indexSet;
    if coltarget.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    while i <= (*multi).used && *coltarget.offset(i as isize) != varnr {
        i += 1;
    }
    if i > (*multi).used {
        return 0 as ::core::ffi::c_uchar;
    }
    while i < (*multi).used {
        *coltarget.offset(i as isize) = *coltarget.offset((i + 1 as ::core::ffi::c_int) as isize);
        i += 1;
    }
    let ref mut fresh5 = *coltarget.offset(0 as ::core::ffi::c_int as isize);
    *fresh5 -= 1;
    (*multi).used -= 1;
    (*multi).dirty = TRUE as ::core::ffi::c_uchar;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_multi_enteringvar"]
pub unsafe extern "C" fn multi_enteringvar(
    mut multi: *mut multirec,
    mut current: *mut pricerec,
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*multi).lp;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bestindex: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut bound: ::core::ffi::c_double = 0.;
    let mut score: ::core::ffi::c_double = 0.;
    let mut bestscore: ::core::ffi::c_double = -(*lp).infinite;
    let mut b1: ::core::ffi::c_double = 0.;
    let mut b2: ::core::ffi::c_double = 0.;
    let mut b3: ::core::ffi::c_double = 0.;
    let mut candidate: *mut pricerec = ::core::ptr::null_mut::<pricerec>();
    let mut bestcand: *mut pricerec = ::core::ptr::null_mut::<pricerec>();
    bestindex = 0 as ::core::ffi::c_int;
    (*multi).active = bestindex;
    if multi.is_null() || (*multi).used == 0 as ::core::ffi::c_int {
        return bestindex;
    }
    if (*multi).objcheck as ::core::ffi::c_int != 0
        && (*lp).solutioncount > 0 as ::core::ffi::c_int
        && bb_better(lp, OF_WORKING | OF_PROJECTED, OF_TEST_WE) as ::core::ffi::c_int != 0
    {
        (*lp).spx_status = FATHOMED;
        return bestindex;
    }
    if (*multi).used == 1 as ::core::ffi::c_int {
        bestcand = (*(*multi).sortedList.offset(bestindex as isize))
            .pvoidreal
            .ptr as *mut pricerec;
    } else {
        loop {
            match priority {
                0 => {
                    b1 = 0.0f64;
                    b2 = 0.0f64;
                    b3 = 1.0f64;
                    bestindex = (*multi).used - 2 as ::core::ffi::c_int;
                }
                1 => {
                    b1 = 0.2f64;
                    b2 = 0.3f64;
                    b3 = 0.5f64;
                }
                2 => {
                    b1 = 0.3f64;
                    b2 = 0.5f64;
                    b3 = 0.2f64;
                }
                3 => {
                    b1 = 0.6f64;
                    b2 = 0.2f64;
                    b3 = 0.2f64;
                }
                4 => {
                    b1 = 1.0f64;
                    b2 = 0.0f64;
                    b3 = 0.0f64;
                }
                _ => {
                    b1 = 0.4f64;
                    b2 = 0.2f64;
                    b3 = 0.4f64;
                }
            }
            bestcand = (*(*multi).sortedList.offset(bestindex as isize))
                .pvoidreal
                .ptr as *mut pricerec;
            i = (*multi).used - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                candidate =
                    (*(*multi).sortedList.offset(i as isize)).pvoidreal.ptr as *mut pricerec;
                colnr = (*candidate).varno;
                bound = *(*lp).upbo.offset(colnr as isize);
                score = fabs((*candidate).pivot) / (*multi).maxpivot;
                score = pow(1.0f64 + score, b1)
                    * pow(
                        1.0f64
                            + log(bound / (*multi).maxbound
                                + 1 as ::core::ffi::c_int as ::core::ffi::c_double),
                        b2,
                    )
                    * pow(
                        1.0f64
                            + i as ::core::ffi::c_double / (*multi).used as ::core::ffi::c_double,
                        b3,
                    );
                if score > bestscore {
                    bestscore = score;
                    bestindex = i;
                    bestcand = candidate;
                }
                i -= 1;
            }
            if !(priority < 4 as ::core::ffi::c_int && fabs((*bestcand).pivot) < (*lp).epssolution)
            {
                break;
            }
            bestindex = 0 as ::core::ffi::c_int;
            priority += 1;
        }
    }
    colnr = (*bestcand).varno;
    (*multi).active = colnr;
    if bestindex < (*multi).used - 1 as ::core::ffi::c_int {
        (*multi).used = i + 1 as ::core::ffi::c_int;
    }
    multi_populateSet(
        multi,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
        (*multi).active,
    );
    score = if (*multi).used == 1 as ::core::ffi::c_int {
        (*multi).step_base
    } else {
        (*(*multi)
            .sortedList
            .offset(((*multi).used - 2 as ::core::ffi::c_int) as isize))
        .pvoidreal
        .realval
    };
    score /= (*bestcand).pivot;
    score = if *(*lp).is_lower.offset((*multi).active as isize) == 0
        && score != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        -score
    } else {
        score
    };
    if (*lp).spx_trace as ::core::ffi::c_int != 0
        && fabs(score) > 1 as ::core::ffi::c_int as ::core::ffi::c_double / (*lp).epsprimal
    {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"multi_enteringvar: A very large Theta %g was generated (pivot %g)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    (*multi).step_base = score;
    if !current.is_null() {
        *current = *bestcand;
    }
    return (*multi).active;
}
#[export_name="honest_lpsolve_multi_enteringtheta"]
pub unsafe extern "C" fn multi_enteringtheta(mut multi: *mut multirec) -> ::core::ffi::c_double {
    return (*multi).step_base;
}
#[export_name="honest_lpsolve_multi_populateSet"]
pub unsafe extern "C" fn multi_populateSet(
    mut multi: *mut multirec,
    mut list: *mut *mut ::core::ffi::c_int,
    mut excludenr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if list.is_null() {
        list = &raw mut (*multi).indexSet;
    }
    if (*multi).used > 0 as ::core::ffi::c_int
        && (!(*list).is_null()
            || allocINT(
                (*multi).lp,
                list,
                (*multi).size + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut colnr: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*multi).used {
            colnr =
                (*((*(*multi).sortedList.offset(i as isize)).pvoidreal.ptr as *mut pricerec)).varno;
            if colnr != excludenr
                && (excludenr > 0 as ::core::ffi::c_int
                    && *(*(*multi).lp).upbo.offset(colnr as isize) < (*(*multi).lp).infinite)
            {
                n += 1;
                *(*list).offset(n as isize) = colnr;
            }
            i += 1;
        }
        *(*list).offset(0 as ::core::ffi::c_int as isize) = n;
    }
    return n;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const COMP_PREFERCANDIDATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COMP_PREFERNONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COMP_PREFERINCUMBENT: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SIMPLEX_Phase1_PRIMAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIMPLEX_Phase2_PRIMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIMPLEX_PRIMAL_PRIMAL: ::core::ffi::c_int = SIMPLEX_Phase1_PRIMAL + SIMPLEX_Phase2_PRIMAL;
pub const IMPROVE_SOLUTION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PRICER_FIRSTINDEX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PRICER_RANDFACT: ::core::ffi::c_double = 0.1f64;
pub const PRICE_RANDOMIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PRICE_LOOPLEFT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const PRICE_LOOPALTERNATE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const PRICE_HARRISTWOPASS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const PRICE_FORCEFULL: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const ACTION_REINVERT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NUMFAILURE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FATHOMED: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const OF_WORKING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OF_PROJECTED: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OF_TEST_WE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DOUBLEROUND: ::core::ffi::c_double = 0.0e-02f64;
pub const LIMIT_ABS_REL: ::core::ffi::c_double = 10.0f64;
pub const MAT_ROUNDREL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAT_ROUNDRC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAT_ROUNDDEFAULT: ::core::ffi::c_int = MAT_ROUNDREL;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
