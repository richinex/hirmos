use crate::honest_did::lpsolve::runtime::{strlen,strcpy,strncpy,strcmp,strncmp};
use crate::honest_did::lpsolve::runtime::{free,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    #[link_name="honest_lpsolve_unscaled_value"]
    fn unscaled_value(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut __stdinp: *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        __size: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fputs(_: *const ::core::ffi::c_char, _: *mut FILE) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_swapREAL"]
    fn swapREAL(item1: *mut ::core::ffi::c_double, item2: *mut ::core::ffi::c_double);
    #[link_name="honest_lpsolve_find_var"]
    fn find_var(
        lp: *mut lprec,
        name: *mut ::core::ffi::c_char,
        verbose: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_find_row"]
    fn find_row(
        lp: *mut lprec,
        name: *mut ::core::ffi::c_char,
        Unconstrained_rows_found: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_append_SOSrec"]
    fn append_SOSrec(
        SOS: *mut SOSrec,
        size: ::core::ffi::c_int,
        variables: *mut ::core::ffi::c_int,
        weights: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_make_lp"]
    fn make_lp(rows: ::core::ffi::c_int, columns: ::core::ffi::c_int) -> *mut lprec;
    #[link_name="honest_lpsolve_delete_lp"]
    fn delete_lp(lp: *mut lprec);
    #[link_name="honest_lpsolve_set_lp_name"]
    fn set_lp_name(lp: *mut lprec, lpname: *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_maxim"]
    fn set_maxim(lp: *mut lprec);
    #[link_name="honest_lpsolve_set_minim"]
    fn set_minim(lp: *mut lprec);
    #[link_name="honest_lpsolve_is_maxim"]
    fn is_maxim(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_str_add_constraint"]
    fn str_add_constraint(
        lp: *mut lprec,
        row_string: *mut ::core::ffi::c_char,
        constr_type: ::core::ffi::c_int,
        rh: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_constr_type"]
    fn set_constr_type(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        con_type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_rh"]
    fn set_rh(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_add_columnex"]
    fn add_columnex(
        lp: *mut lprec,
        count: ::core::ffi::c_int,
        column: *mut ::core::ffi::c_double,
        rowno: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_columnex"]
    fn get_columnex(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        column: *mut ::core::ffi::c_double,
        nzrow: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_upbo"]
    fn set_upbo(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_upbo"]
    fn get_upbo(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_lowbo"]
    fn set_lowbo(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_lowbo"]
    fn get_lowbo(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_bounds"]
    fn set_bounds(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        lower: ::core::ffi::c_double,
        upper: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_unbounded"]
    fn set_unbounded(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_unbounded"]
    fn is_unbounded(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_int"]
    fn set_int(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        must_be_int: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_int"]
    fn is_int(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_binary"]
    fn set_binary(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        must_be_bin: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_binary"]
    fn is_binary(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_semicont"]
    fn set_semicont(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        must_be_sc: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_semicont"]
    fn is_semicont(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_add_SOS"]
    fn add_SOS(
        lp: *mut lprec,
        name: *mut ::core::ffi::c_char,
        sostype: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        count: ::core::ffi::c_int,
        sosvars: *mut ::core::ffi::c_int,
        weights: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_row_name"]
    fn set_row_name(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        new_name: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_col_name"]
    fn set_col_name(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        new_name: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_default_basis"]
    fn default_basis(lp: *mut lprec);
    #[link_name="honest_lpsolve_set_outputstream"]
    fn set_outputstream(lp: *mut lprec, stream: *mut FILE);
    #[link_name="honest_lpsolve_get_nameindex"]
    fn get_nameindex(
        lp: *mut lprec,
        varname: *mut ::core::ffi::c_char,
        isrow: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_chsign"]
    fn is_chsign(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_count"]
    fn SOS_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_splitvar"]
    fn is_splitvar(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
}
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
pub type __darwin_size_t = usize;
pub type __darwin_wchar_t = ::core::ffi::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
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
pub type read_modeldata_func = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type write_modeldata_func =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [::core::ffi::c_char; 8],
    pub __encoding: [::core::ffi::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            __darwin_size_t,
            *mut *const ::core::ffi::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut ::core::ffi::c_char,
            __darwin_size_t,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut ::core::ffi::c_void,
    pub __variable_len: ::core::ffi::c_int,
    pub __ncharclasses: ::core::ffi::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [::core::ffi::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: ::core::ffi::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
pub const _CTYPE_S: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
#[inline]
unsafe extern "C" fn isascii(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __istype(
    mut _c: __darwin_ct_rune_t,
    mut _f: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    return if isascii(_c as ::core::ffi::c_int) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as ::core::ffi::c_ulong & _f != 0)
            as ::core::ffi::c_int
    } else {
        (native_only!(__maskrune,_c, _f) != 0) as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong);
}
#[inline]
unsafe extern "C" fn toupper(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return native_only!(__toupper,_c as __darwin_ct_rune_t) as ::core::ffi::c_int;
}
pub const ROWNAMEMASK: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"R%d\0") };
pub const COLNAMEMASK: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"C%d\0") };
#[export_name="honest_lpsolve_namecpy"]
pub unsafe extern "C" fn namecpy(
    mut into: *mut ::core::ffi::c_char,
    mut from: *mut ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while *from.offset(i as isize) as ::core::ffi::c_int != '\0' as i32
        && *from.offset(i as isize) as ::core::ffi::c_int != '\n' as i32
        && *from.offset(i as isize) as ::core::ffi::c_int != '\r' as i32
        && i < 8 as ::core::ffi::c_int
    {
        *into.offset(i as isize) = *from.offset(i as isize);
        i += 1;
    }
    *into.offset(i as isize) = '\0' as i32 as ::core::ffi::c_char;
    i -= 1;
    while i >= 0 as ::core::ffi::c_int
        && *into.offset(i as isize) as ::core::ffi::c_int == ' ' as i32
    {
        *into.offset(i as isize) = '\0' as i32 as ::core::ffi::c_char;
        i -= 1;
    }
}
#[export_name="honest_lpsolve_scan_lineFIXED"]
pub unsafe extern "C" fn scan_lineFIXED(
    mut lp: *mut lprec,
    mut section: ::core::ffi::c_int,
    mut line: *mut ::core::ffi::c_char,
    mut field1: *mut ::core::ffi::c_char,
    mut field2: *mut ::core::ffi::c_char,
    mut field3: *mut ::core::ffi::c_char,
    mut field4: *mut ::core::ffi::c_double,
    mut field5: *mut ::core::ffi::c_char,
    mut field6: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut items: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line_len: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut ptr1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line_len = strlen(line) as ::core::ffi::c_int;
    while line_len != 0
        && (*line.offset((line_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '\n' as i32
            || *line.offset((line_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '\r' as i32
            || *line.offset((line_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == ' ' as i32)
    {
        line_len -= 1;
    }
    if line_len >= 1 as ::core::ffi::c_int {
        strncpy(&raw mut buf as *mut ::core::ffi::c_char, line, 4 as size_t);
        buf[4 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        native_only!(sscanf,
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            field1,
        );
        items += 1;
    } else {
        *field1.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    line = line.offset(4 as ::core::ffi::c_int as isize);
    if line_len >= 5 as ::core::ffi::c_int {
        if *line.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32 {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"MPS_readfile: invalid data card; column 4 must be blank\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        namecpy(field2, line);
        items += 1;
    } else {
        *field2.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    line = line.offset(10 as ::core::ffi::c_int as isize);
    if line_len >= 14 as ::core::ffi::c_int {
        if *line.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
            || *line.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
        {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"MPS_readfile: invalid data card; columns 13-14 must be blank\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        namecpy(field3, line);
        items += 1;
    } else {
        *field3.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    line = line.offset(10 as ::core::ffi::c_int as isize);
    if line_len >= 25 as ::core::ffi::c_int {
        if *line.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
            || *line.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
        {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"MPS_readfile: invalid data card; columns 23-24 must be blank\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        strncpy(&raw mut buf as *mut ::core::ffi::c_char, line, 15 as size_t);
        buf[15 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        ptr2 = &raw mut buf as *mut ::core::ffi::c_char;
        ptr1 = ptr2;
        loop {
            if isspace(*ptr1 as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
                let fresh2 = ptr2;
                ptr2 = ptr2.offset(1);
                *fresh2 = *ptr1;
                if *fresh2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            ptr1 = ptr1.offset(1);
        }
        *field4 = native_only!(strtod,&raw mut buf as *mut ::core::ffi::c_char, &raw mut ptr1);
        if *ptr1 != 0 {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"MPS_readfile: invalid number in columns 25-36 \n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        items += 1;
    } else {
        *field4 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    line = line.offset(15 as ::core::ffi::c_int as isize);
    if line_len >= 40 as ::core::ffi::c_int {
        if *line.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
            || *line.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
            || *line.offset(-(3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
        {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"MPS_readfile: invalid data card; columns 37-39 must be blank\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        namecpy(field5, line);
        items += 1;
    } else {
        *field5.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    line = line.offset(10 as ::core::ffi::c_int as isize);
    if line_len >= 50 as ::core::ffi::c_int {
        if *line.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
            || *line.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ' ' as i32
        {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"MPS_readfile: invalid data card; columns 48-49 must be blank\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        strncpy(&raw mut buf as *mut ::core::ffi::c_char, line, 15 as size_t);
        buf[15 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        ptr2 = &raw mut buf as *mut ::core::ffi::c_char;
        ptr1 = ptr2;
        loop {
            if isspace(*ptr1 as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
                let fresh3 = ptr2;
                ptr2 = ptr2.offset(1);
                *fresh3 = *ptr1;
                if *fresh3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            ptr1 = ptr1.offset(1);
        }
        *field6 = native_only!(strtod,&raw mut buf as *mut ::core::ffi::c_char, &raw mut ptr1);
        if *ptr1 != 0 {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"MPS_readfile: invalid number in columns 50-61 \n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        items += 1;
    } else {
        *field6 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    return items;
}
#[export_name="honest_lpsolve_spaces"]
pub unsafe extern "C" fn spaces(
    mut line: *mut ::core::ffi::c_char,
    mut line_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = 0;
    let mut line1: *mut ::core::ffi::c_char = line;
    while *line1 as ::core::ffi::c_int == ' ' as i32 {
        line1 = line1.offset(1);
    }
    l = line1.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
    if line_len < l {
        l = line_len;
    }
    return l;
}
#[export_name="honest_lpsolve_lenfield"]
pub unsafe extern "C" fn lenfield(
    mut line: *mut ::core::ffi::c_char,
    mut line_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = 0;
    let mut line1: *mut ::core::ffi::c_char = line;
    while *line1 as ::core::ffi::c_int != 0 && *line1 as ::core::ffi::c_int != ' ' as i32 {
        line1 = line1.offset(1);
    }
    l = line1.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
    if line_len < l {
        l = line_len;
    }
    return l;
}
#[export_name="honest_lpsolve_scan_lineFREE"]
pub unsafe extern "C" fn scan_lineFREE(
    mut lp: *mut lprec,
    mut section: ::core::ffi::c_int,
    mut line: *mut ::core::ffi::c_char,
    mut field1: *mut ::core::ffi::c_char,
    mut field2: *mut ::core::ffi::c_char,
    mut field3: *mut ::core::ffi::c_char,
    mut field4: *mut ::core::ffi::c_double,
    mut field5: *mut ::core::ffi::c_char,
    mut field6: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut items: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line_len: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut ptr1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line_len = strlen(line) as ::core::ffi::c_int;
    while line_len != 0
        && (*line.offset((line_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '\n' as i32
            || *line.offset((line_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '\r' as i32
            || *line.offset((line_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == ' ' as i32)
    {
        line_len -= 1;
    }
    len = spaces(line, line_len);
    line = line.offset(len as isize);
    line_len -= len;
    if section == MPSCOLUMNS || section == MPSRHS || section == MPSRANGES {
        *field1.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        items += 1;
    } else {
        len = lenfield(line, line_len);
        if line_len >= 1 as ::core::ffi::c_int {
            strncpy(
                &raw mut buf as *mut ::core::ffi::c_char,
                line,
                len as size_t,
            );
            buf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
            native_only!(sscanf,
                &raw mut buf as *mut ::core::ffi::c_char,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                field1,
            );
            if section == MPSBOUNDS {
                ptr1 = field1;
                while *ptr1 != 0 {
                    *ptr1 = toupper(*ptr1 as ::core::ffi::c_int) as ::core::ffi::c_char;
                    ptr1 = ptr1.offset(1);
                }
            }
            items += 1;
        } else {
            *field1.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
        line = line.offset(len as isize);
        line_len -= len;
        len = spaces(line, line_len);
        line = line.offset(len as isize);
        line_len -= len;
    }
    len = lenfield(line, line_len);
    if line_len >= 1 as ::core::ffi::c_int {
        strncpy(field2, line, len as size_t);
        *field2.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        items += 1;
    } else {
        *field2.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    line = line.offset(len as isize);
    line_len -= len;
    len = spaces(line, line_len);
    line = line.offset(len as isize);
    line_len -= len;
    len = lenfield(line, line_len);
    if line_len >= 1 as ::core::ffi::c_int {
        strncpy(field3, line, len as size_t);
        *field3.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        items += 1;
    } else {
        *field3.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    line = line.offset(len as isize);
    line_len -= len;
    len = spaces(line, line_len);
    line = line.offset(len as isize);
    line_len -= len;
    if *field3 != 0 {
        if section == MPSCOLUMNS
            && strcmp(
                field3,
                b"'MARKER'\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            *field4 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            items += 1;
            ptr1 = field3;
        } else if !(section == MPSBOUNDS
            && (strcmp(field1, b"FR\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(field1, b"MI\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcmp(field1, b"PL\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcmp(field1, b"BV\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int))
        {
            let mut line1: *mut ::core::ffi::c_char = line;
            let mut line_len1: ::core::ffi::c_int = line_len;
            let mut items1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while line_len1 > 0 as ::core::ffi::c_int {
                len = lenfield(line1, line_len1);
                if len > 0 as ::core::ffi::c_int {
                    line1 = line1.offset(len as isize);
                    line_len1 -= len;
                    items1 += 1;
                }
                len = spaces(line1, line_len1);
                line1 = line1.offset(len as isize);
                line_len1 -= len;
            }
            if items1 % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                *field4 = native_only!(strtod,field3, &raw mut ptr1);
                if *ptr1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    strcpy(field3, field2);
                    if section == MPSROWS || section == MPSBOUNDS {
                        *field2 = 0 as ::core::ffi::c_char;
                    } else {
                        strcpy(field2, field1);
                        *field1 = 0 as ::core::ffi::c_char;
                    }
                    items += 1;
                } else {
                    ptr1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            } else {
                ptr1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
    } else {
        ptr1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if section == MPSBOUNDS
            && (strcmp(field1, b"FR\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(field1, b"MI\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcmp(field1, b"PL\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcmp(field1, b"BV\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int)
        {
            strcpy(field3, field2);
            *field2 = 0 as ::core::ffi::c_char;
            items += 1;
        }
    }
    if ptr1.is_null() {
        len = lenfield(line, line_len);
        if line_len >= 1 as ::core::ffi::c_int {
            strncpy(
                &raw mut buf as *mut ::core::ffi::c_char,
                line,
                len as size_t,
            );
            buf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
            ptr2 = &raw mut buf as *mut ::core::ffi::c_char;
            ptr1 = ptr2;
            loop {
                if isspace(*ptr1 as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
                    let fresh0 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh0 = *ptr1;
                    if *fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        break;
                    }
                }
                ptr1 = ptr1.offset(1);
            }
            *field4 = native_only!(strtod,&raw mut buf as *mut ::core::ffi::c_char, &raw mut ptr1);
            if *ptr1 != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            items += 1;
        } else {
            *field4 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        line = line.offset(len as isize);
        line_len -= len;
        len = spaces(line, line_len);
        line = line.offset(len as isize);
        line_len -= len;
    }
    len = lenfield(line, line_len);
    if line_len >= 1 as ::core::ffi::c_int {
        strncpy(field5, line, len as size_t);
        *field5.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        items += 1;
    } else {
        *field5.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    line = line.offset(len as isize);
    line_len -= len;
    len = spaces(line, line_len);
    line = line.offset(len as isize);
    line_len -= len;
    len = lenfield(line, line_len);
    if line_len >= 1 as ::core::ffi::c_int {
        strncpy(
            &raw mut buf as *mut ::core::ffi::c_char,
            line,
            len as size_t,
        );
        buf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
        ptr2 = &raw mut buf as *mut ::core::ffi::c_char;
        ptr1 = ptr2;
        loop {
            if isspace(*ptr1 as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
                let fresh1 = ptr2;
                ptr2 = ptr2.offset(1);
                *fresh1 = *ptr1;
                if *fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            ptr1 = ptr1.offset(1);
        }
        *field6 = native_only!(strtod,&raw mut buf as *mut ::core::ffi::c_char, &raw mut ptr1);
        if *ptr1 != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        items += 1;
    } else {
        *field6 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if section == MPSSOS && items == 2 as ::core::ffi::c_int {
        strcpy(field3, field2);
        strcpy(field2, field1);
        *field1 = 0 as ::core::ffi::c_char;
    }
    if section != MPSOBJNAME && section != MPSBOUNDS {
        ptr1 = field1;
        while *ptr1 != 0 {
            *ptr1 = toupper(*ptr1 as ::core::ffi::c_int) as ::core::ffi::c_char;
            ptr1 = ptr1.offset(1);
        }
    }
    return items;
}
#[export_name="honest_lpsolve_addmpscolumn"]
pub unsafe extern "C" fn addmpscolumn(
    mut lp: *mut lprec,
    mut Int_section: ::core::ffi::c_uchar,
    mut typeMPS: ::core::ffi::c_int,
    mut Column_ready: *mut ::core::ffi::c_uchar,
    mut count: *mut ::core::ffi::c_int,
    mut Last_column: *mut ::core::ffi::c_double,
    mut Last_columnno: *mut ::core::ffi::c_int,
    mut Last_col_name: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = TRUE;
    if *Column_ready != 0 {
        ok = add_columnex(lp, *count, Last_column, Last_columnno) as ::core::ffi::c_int;
        if ok != 0 {
            ok = set_col_name(lp, (*lp).columns, Last_col_name) as ::core::ffi::c_int;
        }
        if ok != 0 {
            set_int(lp, (*lp).columns, Int_section);
            if Int_section as ::core::ffi::c_int != 0 && typeMPS & MPSIBM != 0 {
                set_bounds(
                    lp,
                    (*lp).columns,
                    10.0f64 / DEF_INFINITE,
                    DEF_INFINITE / 10.0f64,
                );
            }
        }
    }
    *Column_ready = FALSE as ::core::ffi::c_uchar;
    *count = 0 as ::core::ffi::c_int;
    return ok;
}
#[export_name="honest_lpsolve_appendmpsitem"]
pub unsafe extern "C" fn appendmpsitem(
    mut count: *mut ::core::ffi::c_int,
    mut rowIndex: *mut ::core::ffi::c_int,
    mut rowValue: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = *count;
    if *rowIndex.offset(i as isize) < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uchar;
    }
    while i > 0 as ::core::ffi::c_int
        && *rowIndex.offset(i as isize) < *rowIndex.offset((i - 1 as ::core::ffi::c_int) as isize)
    {
        swapINT(
            rowIndex.offset(i as isize),
            rowIndex
                .offset(i as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
        );
        swapREAL(
            rowValue.offset(i as isize),
            rowValue
                .offset(i as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
        );
        i -= 1;
    }
    if i < *count
        && *rowIndex.offset(i as isize) == *rowIndex.offset((i + 1 as ::core::ffi::c_int) as isize)
    {
        let mut ii: ::core::ffi::c_int = i + 1 as ::core::ffi::c_int;
        *rowValue.offset(i as isize) += *rowValue.offset(ii as isize);
        *count -= 1;
        while ii < *count {
            *rowIndex.offset(ii as isize) =
                *rowIndex.offset((ii + 1 as ::core::ffi::c_int) as isize);
            *rowValue.offset(ii as isize) =
                *rowValue.offset((ii + 1 as ::core::ffi::c_int) as isize);
            ii += 1;
        }
    }
    *count += 1;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_MPS_readfile"]
pub unsafe extern "C" fn MPS_readfile(
    mut newlp: *mut *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
    mut typeMPS: ::core::ffi::c_int,
    mut verbose: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut fpin: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fpin = native_only!(fopen,filename, b"r\0" as *const u8 as *const ::core::ffi::c_char);
    if !fpin.is_null() {
        status = MPS_readhandle(newlp, fpin, typeMPS, verbose);
        native_only!(fclose,fpin);
    }
    return status;
}
unsafe extern "C" fn MPS_input(
    mut fpin: *mut ::core::ffi::c_void,
    mut buf: *mut ::core::ffi::c_char,
    mut max_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (native_only!(fgets,buf, max_size, fpin as *mut FILE) != NULL as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_MPS_readhandle"]
pub unsafe extern "C" fn MPS_readhandle(
    mut newlp: *mut *mut lprec,
    mut filehandle: *mut FILE,
    mut typeMPS: ::core::ffi::c_int,
    mut verbose: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return MPS_readex(
        newlp,
        filehandle as *mut ::core::ffi::c_void,
        Some(
            MPS_input
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        typeMPS,
        verbose,
    );
}
#[export_name="honest_lpsolve_MPS_readex"]
pub unsafe extern "C" fn MPS_readex(
    mut newlp: *mut *mut lprec,
    mut userhandle: *mut ::core::ffi::c_void,
    mut read_modeldata: Option<read_modeldata_func>,
    mut typeMPS: ::core::ffi::c_int,
    mut verbose: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut field1: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut field2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut field3: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut field5: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut Last_col_name: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut probname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut OBJNAME: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut items: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut Lineno: ::core::ffi::c_int = 0;
    let mut var: ::core::ffi::c_int = 0;
    let mut section: ::core::ffi::c_int = MPSUNDEF;
    let mut variant: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut NZ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut SOS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Int_section: ::core::ffi::c_uchar = 0;
    let mut Column_ready: ::core::ffi::c_uchar = 0;
    let mut Column_ready1: ::core::ffi::c_uchar = 0;
    let mut Unconstrained_rows_found: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut OF_found: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut CompleteStatus: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut field4: ::core::ffi::c_double = 0.;
    let mut field6: ::core::ffi::c_double = 0.;
    let mut Last_column: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Last_columnno: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut OBJSENSE: ::core::ffi::c_int = ROWTYPE_EMPTY;
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut scan_line: Option<
        unsafe extern "C" fn(
            *mut lprec,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    > = None;
    if newlp.is_null() {
        return CompleteStatus;
    } else if (*newlp).is_null() {
        lp = make_lp(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    } else {
        lp = *newlp;
    }
    if typeMPS & MPSFIXED == MPSFIXED {
        scan_line = Some(
            scan_lineFIXED
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
            >;
    } else if typeMPS & MPSFREE == MPSFREE {
        scan_line = Some(
            scan_lineFREE
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
            >;
    } else {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"MPS_readfile: Unrecognized MPS line type.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if (*newlp).is_null() {
            delete_lp(lp);
        }
        return CompleteStatus;
    }
    if !lp.is_null() {
        (*lp).source_is_file = TRUE as ::core::ffi::c_uchar;
        (*lp).verbose = verbose;
        strcpy(
            &raw mut Last_col_name as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcpy(
            &raw mut OBJNAME as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
        Int_section = FALSE as ::core::ffi::c_uchar;
        Column_ready = FALSE as ::core::ffi::c_uchar;
        Lineno = 0 as ::core::ffi::c_int;
        memset(
            &raw mut line as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            '\0' as i32,
            (1024 as ::core::ffi::c_int as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        let mut current_block_236: u64;
        while read_modeldata.expect("non-null function pointer")(
            userhandle,
            &raw mut line as *mut ::core::ffi::c_char,
            BUFSIZ - 1 as ::core::ffi::c_int,
        ) != 0
        {
            Lineno += 1;
            ptr = &raw mut line as *mut ::core::ffi::c_char;
            while *ptr as ::core::ffi::c_int != 0
                && isspace(*ptr as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
            {
                ptr = ptr.offset(1);
            }
            if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '*' as i32
                || *ptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || *ptr as ::core::ffi::c_int == '\n' as i32
                || *ptr as ::core::ffi::c_int == '\r' as i32
            {
                report(
                    lp,
                    6 as ::core::ffi::c_int,
                    b"Comment on line %d: %s\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                report(
                    lp,
                    6 as ::core::ffi::c_int,
                    b"Line %6d: %s\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != ' ' as i32 {
                    native_only!(sscanf,
                        &raw mut line as *mut ::core::ffi::c_char,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut tmp as *mut ::core::ffi::c_char,
                    );
                    if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"NAME\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        section = MPSNAME;
                        *(&raw mut probname as *mut ::core::ffi::c_char) = 0 as ::core::ffi::c_char;
                        native_only!(sscanf,
                            &raw mut line as *mut ::core::ffi::c_char,
                            b"NAME %s\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut probname as *mut ::core::ffi::c_char,
                        );
                        if set_lp_name(lp, &raw mut probname as *mut ::core::ffi::c_char) == 0 {
                            break;
                        }
                    } else if typeMPS & MPSFREE == MPSFREE
                        && strcmp(
                            &raw mut tmp as *mut ::core::ffi::c_char,
                            b"OBJSENSE\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        section = MPSOBJSENSE;
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to OBJSENSE section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if typeMPS & MPSFREE == MPSFREE
                        && strcmp(
                            &raw mut tmp as *mut ::core::ffi::c_char,
                            b"OBJNAME\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        section = MPSOBJNAME;
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to OBJNAME section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"ROWS\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        section = MPSROWS;
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to ROWS section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"COLUMNS\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        allocREAL(
                            lp,
                            &raw mut Last_column,
                            (*lp).rows + 1 as ::core::ffi::c_int,
                            TRUE as ::core::ffi::c_uchar,
                        );
                        allocINT(
                            lp,
                            &raw mut Last_columnno,
                            (*lp).rows + 1 as ::core::ffi::c_int,
                            TRUE as ::core::ffi::c_uchar,
                        );
                        count = 0 as ::core::ffi::c_int;
                        if Last_column.is_null() || Last_columnno.is_null() {
                            break;
                        }
                        section = MPSCOLUMNS;
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to COLUMNS section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"RHS\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if addmpscolumn(
                            lp,
                            Int_section,
                            typeMPS,
                            &raw mut Column_ready,
                            &raw mut count,
                            Last_column,
                            Last_columnno,
                            &raw mut Last_col_name as *mut ::core::ffi::c_char,
                        ) == 0
                        {
                            break;
                        }
                        section = MPSRHS;
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to RHS section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"BOUNDS\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        section = MPSBOUNDS;
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to BOUNDS section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"RANGES\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        section = MPSRANGES;
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to RANGES section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"SOS\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                        || strcmp(
                            &raw mut tmp as *mut ::core::ffi::c_char,
                            b"SETS\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        section = MPSSOS;
                        if strcmp(
                            &raw mut tmp as *mut ::core::ffi::c_char,
                            b"SOS\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            variant = 0 as ::core::ffi::c_int;
                        } else {
                            variant = 1 as ::core::ffi::c_int;
                        }
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Switching to %s section\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else if strcmp(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"ENDATA\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        report(
                            lp,
                            6 as ::core::ffi::c_int,
                            b"Finished reading MPS file\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        CompleteStatus = TRUE as ::core::ffi::c_uchar;
                        break;
                    } else {
                        report(
                            lp,
                            3 as ::core::ffi::c_int,
                            b"Unrecognized MPS line %d: %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        break;
                    }
                } else {
                    items = scan_line.expect("non-null function pointer")(
                        lp,
                        section,
                        &raw mut line as *mut ::core::ffi::c_char,
                        &raw mut field1 as *mut ::core::ffi::c_char,
                        &raw mut field2 as *mut ::core::ffi::c_char,
                        &raw mut field3 as *mut ::core::ffi::c_char,
                        &raw mut field4,
                        &raw mut field5 as *mut ::core::ffi::c_char,
                        &raw mut field6,
                    );
                    if items < 0 as ::core::ffi::c_int {
                        report(
                            lp,
                            3 as ::core::ffi::c_int,
                            b"Syntax error on line %d: %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        break;
                    } else {
                        match section {
                            MPSNAME => {
                                report(
                                    lp,
                                    3 as ::core::ffi::c_int,
                                    b"Error, extra line under NAME line\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                            MPSOBJSENSE => {
                                if OBJSENSE != ROWTYPE_EMPTY {
                                    report(
                                        lp,
                                        3 as ::core::ffi::c_int,
                                        b"Error, extra line under OBJSENSE line\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                } else if strcmp(
                                    &raw mut field1 as *mut ::core::ffi::c_char,
                                    b"MAXIMIZE\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                    || strcmp(
                                        &raw mut field1 as *mut ::core::ffi::c_char,
                                        b"MAX\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                {
                                    OBJSENSE = ROWTYPE_OFMAX;
                                    set_maxim(lp);
                                    continue;
                                } else if strcmp(
                                    &raw mut field1 as *mut ::core::ffi::c_char,
                                    b"MINIMIZE\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                    || strcmp(
                                        &raw mut field1 as *mut ::core::ffi::c_char,
                                        b"MIN\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                {
                                    OBJSENSE = ROWTYPE_OFMIN;
                                    set_minim(lp);
                                    continue;
                                } else {
                                    report(
                                        lp,
                                        2 as ::core::ffi::c_int,
                                        b"Unknown OBJSENSE direction '%s' on line %d\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                            }
                            MPSOBJNAME => {
                                if *(&raw mut OBJNAME as *mut ::core::ffi::c_char) != 0 {
                                    report(
                                        lp,
                                        3 as ::core::ffi::c_int,
                                        b"Error, extra line under OBJNAME line\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                } else {
                                    strcpy(
                                        &raw mut OBJNAME as *mut ::core::ffi::c_char,
                                        &raw mut field1 as *mut ::core::ffi::c_char,
                                    );
                                    continue;
                                }
                            }
                            MPSROWS => {
                                report(
                                    lp,
                                    6 as ::core::ffi::c_int,
                                    b"Row   %5d: %s %s\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                if strcmp(
                                    &raw mut field1 as *mut ::core::ffi::c_char,
                                    b"N\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    if *(&raw mut OBJNAME as *mut ::core::ffi::c_char)
                                        as ::core::ffi::c_int
                                        != 0
                                        && strcmp(
                                            &raw mut field2 as *mut ::core::ffi::c_char,
                                            &raw mut OBJNAME as *mut ::core::ffi::c_char,
                                        ) != 0
                                    {
                                        continue;
                                    } else if OF_found == 0 {
                                        if !(set_row_name(
                                            lp,
                                            0 as ::core::ffi::c_int,
                                            &raw mut field2 as *mut ::core::ffi::c_char,
                                        ) == 0)
                                        {
                                            OF_found = TRUE as ::core::ffi::c_uchar;
                                            continue;
                                        }
                                    } else {
                                        if Unconstrained_rows_found == 0 {
                                            report(
                                                lp,
                                                3 as ::core::ffi::c_int,
                                                b"Unconstrained row %s ignored\n\0" as *const u8
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                            report(
                                                lp,
                                                3 as ::core::ffi::c_int,
                                                b"Further messages of this kind will be suppressed\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                            Unconstrained_rows_found = TRUE as ::core::ffi::c_uchar;
                                        }
                                        continue;
                                    }
                                } else if strcmp(
                                    &raw mut field1 as *mut ::core::ffi::c_char,
                                    b"L\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    if !(str_add_constraint(
                                        lp,
                                        b"\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        LE,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                                    ) == 0
                                        || set_row_name(
                                            lp,
                                            (*lp).rows,
                                            &raw mut field2 as *mut ::core::ffi::c_char,
                                        ) == 0)
                                    {
                                        continue;
                                    }
                                } else if strcmp(
                                    &raw mut field1 as *mut ::core::ffi::c_char,
                                    b"G\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    if !(str_add_constraint(
                                        lp,
                                        b"\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        GE,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                                    ) == 0
                                        || set_row_name(
                                            lp,
                                            (*lp).rows,
                                            &raw mut field2 as *mut ::core::ffi::c_char,
                                        ) == 0)
                                    {
                                        continue;
                                    }
                                } else if strcmp(
                                    &raw mut field1 as *mut ::core::ffi::c_char,
                                    b"E\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    if !(str_add_constraint(
                                        lp,
                                        b"\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        EQ,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                                    ) == 0
                                        || set_row_name(
                                            lp,
                                            (*lp).rows,
                                            &raw mut field2 as *mut ::core::ffi::c_char,
                                        ) == 0)
                                    {
                                        continue;
                                    }
                                } else {
                                    report(
                                        lp,
                                        2 as ::core::ffi::c_int,
                                        b"Unknown relation code '%s' on line %d\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                            }
                            MPSCOLUMNS => {
                                report(
                                    lp,
                                    6 as ::core::ffi::c_int,
                                    b"Column %4d: %s %s %g %s %g\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                if items == 4 as ::core::ffi::c_int
                                    || items == 5 as ::core::ffi::c_int
                                    || items == 6 as ::core::ffi::c_int
                                {
                                    if NZ == 0 as ::core::ffi::c_int {
                                        strcpy(
                                            &raw mut Last_col_name as *mut ::core::ffi::c_char,
                                            &raw mut field2 as *mut ::core::ffi::c_char,
                                        );
                                        current_block_236 = 13740693533991687037;
                                    } else if *(&raw mut field2 as *mut ::core::ffi::c_char) != 0 {
                                        Column_ready1 = (strcmp(
                                            &raw mut field2 as *mut ::core::ffi::c_char,
                                            &raw mut Last_col_name as *mut ::core::ffi::c_char,
                                        ) != 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uchar;
                                        if Column_ready1 != 0 {
                                            if find_var(
                                                lp,
                                                &raw mut field2 as *mut ::core::ffi::c_char,
                                                FALSE as ::core::ffi::c_uchar,
                                            ) >= 0 as ::core::ffi::c_int
                                            {
                                                report(
                                                    lp,
                                                    2 as ::core::ffi::c_int,
                                                    b"Variable name (%s) is already used!\n\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                                current_block_236 = 717878598772063298;
                                            } else if Column_ready != 0 {
                                                if addmpscolumn(
                                                    lp,
                                                    Int_section,
                                                    typeMPS,
                                                    &raw mut Column_ready,
                                                    &raw mut count,
                                                    Last_column,
                                                    Last_columnno,
                                                    &raw mut Last_col_name
                                                        as *mut ::core::ffi::c_char,
                                                ) != 0
                                                {
                                                    strcpy(
                                                        &raw mut Last_col_name
                                                            as *mut ::core::ffi::c_char,
                                                        &raw mut field2 as *mut ::core::ffi::c_char,
                                                    );
                                                    NZ = 0 as ::core::ffi::c_int;
                                                    current_block_236 = 13740693533991687037;
                                                } else {
                                                    current_block_236 = 717878598772063298;
                                                }
                                            } else {
                                                current_block_236 = 13740693533991687037;
                                            }
                                        } else {
                                            current_block_236 = 13740693533991687037;
                                        }
                                    } else {
                                        current_block_236 = 13740693533991687037;
                                    }
                                    match current_block_236 {
                                        717878598772063298 => {}
                                        _ => {
                                            if items == 5 as ::core::ffi::c_int {
                                                if strcmp(
                                                    &raw mut field3 as *mut ::core::ffi::c_char,
                                                    b"'MARKER'\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                ) != 0 as ::core::ffi::c_int
                                                {
                                                    current_block_236 = 717878598772063298;
                                                } else {
                                                    if strcmp(
                                                        &raw mut field5 as *mut ::core::ffi::c_char,
                                                        b"'INTORG'\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        Int_section = TRUE as ::core::ffi::c_uchar;
                                                        report(
                                                            lp,
                                                            6 as ::core::ffi::c_int,
                                                            b"Switching to integer section\n\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    } else if strcmp(
                                                        &raw mut field5 as *mut ::core::ffi::c_char,
                                                        b"'INTEND'\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        Int_section = FALSE as ::core::ffi::c_uchar;
                                                        report(
                                                            lp,
                                                            6 as ::core::ffi::c_int,
                                                            b"Switching to non-integer section\n\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    } else {
                                                        report(
                                                            lp,
                                                            3 as ::core::ffi::c_int,
                                                            b"Unknown marker (ignored) at line %d: %s\n\0" as *const u8
                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                                                        );
                                                    }
                                                    current_block_236 = 17394276730598727748;
                                                }
                                            } else {
                                                row = find_row(
                                                    lp,
                                                    &raw mut field3 as *mut ::core::ffi::c_char,
                                                    Unconstrained_rows_found,
                                                );
                                                if row >= 0 as ::core::ffi::c_int {
                                                    if row > (*lp).rows {
                                                        report(
                                                            lp,
                                                            1 as ::core::ffi::c_int,
                                                            b"Invalid row %s encountered in the MPS file\n\0"
                                                                as *const u8 as *const ::core::ffi::c_char
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    }
                                                    *Last_columnno.offset(count as isize) = row;
                                                    *Last_column.offset(count as isize) = field4;
                                                    if appendmpsitem(
                                                        &raw mut count,
                                                        Last_columnno as *mut ::core::ffi::c_int,
                                                        Last_column as *mut ::core::ffi::c_double,
                                                    ) != 0
                                                    {
                                                        NZ += 1;
                                                        Column_ready = TRUE as ::core::ffi::c_uchar;
                                                    }
                                                }
                                                current_block_236 = 17394276730598727748;
                                            }
                                        }
                                    }
                                } else {
                                    current_block_236 = 17394276730598727748;
                                }
                                match current_block_236 {
                                    717878598772063298 => {}
                                    _ => {
                                        if items == 6 as ::core::ffi::c_int {
                                            row = find_row(
                                                lp,
                                                &raw mut field5 as *mut ::core::ffi::c_char,
                                                Unconstrained_rows_found,
                                            );
                                            if row >= 0 as ::core::ffi::c_int {
                                                if row > (*lp).rows {
                                                    report(
                                                        lp,
                                                        1 as ::core::ffi::c_int,
                                                        b"Invalid row %s encountered in the MPS file\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                }
                                                *Last_columnno.offset(count as isize) = row;
                                                *Last_column.offset(count as isize) = field6;
                                                if appendmpsitem(
                                                    &raw mut count,
                                                    Last_columnno as *mut ::core::ffi::c_int,
                                                    Last_column as *mut ::core::ffi::c_double,
                                                ) != 0
                                                {
                                                    NZ += 1;
                                                    Column_ready = TRUE as ::core::ffi::c_uchar;
                                                }
                                            }
                                        }
                                        if !(items < 4 as ::core::ffi::c_int
                                            || items > 6 as ::core::ffi::c_int)
                                        {
                                            continue;
                                        }
                                        report(
                                            lp,
                                            1 as ::core::ffi::c_int,
                                            b"Wrong number of items (%d) in COLUMNS section (line %d)\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    }
                                }
                            }
                            MPSRHS => {
                                report(
                                    lp,
                                    6 as ::core::ffi::c_int,
                                    b"RHS line: %s %s %g %s %g\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                if items != 4 as ::core::ffi::c_int
                                    && items != 6 as ::core::ffi::c_int
                                {
                                    report(
                                        lp,
                                        1 as ::core::ffi::c_int,
                                        b"Wrong number of items (%d) in RHS section line %d\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                } else {
                                    row = find_row(
                                        lp,
                                        &raw mut field3 as *mut ::core::ffi::c_char,
                                        Unconstrained_rows_found,
                                    );
                                    if row >= 0 as ::core::ffi::c_int {
                                        if row == 0 as ::core::ffi::c_int
                                            && typeMPS & MPSNEGOBJCONST == MPSNEGOBJCONST
                                        {
                                            field4 = -field4;
                                        }
                                        set_rh(lp, row, field4);
                                    }
                                    if items == 6 as ::core::ffi::c_int {
                                        row = find_row(
                                            lp,
                                            &raw mut field5 as *mut ::core::ffi::c_char,
                                            Unconstrained_rows_found,
                                        );
                                        if row >= 0 as ::core::ffi::c_int {
                                            if row == 0 as ::core::ffi::c_int
                                                && typeMPS & MPSNEGOBJCONST == MPSNEGOBJCONST
                                            {
                                                field6 = -field6;
                                            }
                                            set_rh(lp, row, field6);
                                        }
                                    }
                                    continue;
                                }
                            }
                            MPSBOUNDS => {
                                report(
                                    lp,
                                    6 as ::core::ffi::c_int,
                                    b"BOUNDS line: %s %s %s %g\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                var = find_var(
                                    lp,
                                    &raw mut field3 as *mut ::core::ffi::c_char,
                                    FALSE as ::core::ffi::c_uchar,
                                );
                                if var < 0 as ::core::ffi::c_int {
                                    Column_ready = TRUE as ::core::ffi::c_uchar;
                                    if addmpscolumn(
                                        lp,
                                        FALSE as ::core::ffi::c_uchar,
                                        typeMPS,
                                        &raw mut Column_ready,
                                        &raw mut count,
                                        Last_column,
                                        Last_columnno,
                                        &raw mut field3 as *mut ::core::ffi::c_char,
                                    ) == 0
                                    {
                                        current_block_236 = 717878598772063298;
                                    } else {
                                        Column_ready = TRUE as ::core::ffi::c_uchar;
                                        var = find_var(
                                            lp,
                                            &raw mut field3 as *mut ::core::ffi::c_char,
                                            TRUE as ::core::ffi::c_uchar,
                                        );
                                        current_block_236 = 3297745280902459415;
                                    }
                                } else {
                                    current_block_236 = 3297745280902459415;
                                }
                                match current_block_236 {
                                    717878598772063298 => {}
                                    _ => {
                                        if var < 0 as ::core::ffi::c_int {
                                            continue;
                                        }
                                        if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"UP\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if !(set_upbo(lp, var, field4) == 0) {
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"SC\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if field4
                                                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            {
                                                field4 = (*lp).infinite;
                                            }
                                            if !(set_upbo(lp, var, field4) == 0) {
                                                set_semicont(lp, var, TRUE as ::core::ffi::c_uchar);
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"SI\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if field4
                                                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            {
                                                field4 = (*lp).infinite;
                                            }
                                            if !(set_upbo(lp, var, field4) == 0) {
                                                set_int(lp, var, TRUE as ::core::ffi::c_uchar);
                                                set_semicont(lp, var, TRUE as ::core::ffi::c_uchar);
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"LO\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if !(set_lowbo(lp, var, field4) == 0) {
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"PL\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if !(set_upbo(lp, var, (*lp).infinite) == 0) {
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"MI\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if !(set_lowbo(lp, var, -(*lp).infinite) == 0) {
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"FR\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            set_unbounded(lp, var);
                                            continue;
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"FX\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if !(set_bounds(lp, var, field4, field4) == 0) {
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"BV\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            set_binary(lp, var, TRUE as ::core::ffi::c_uchar);
                                            continue;
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"UI\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if !(set_upbo(lp, var, field4) == 0) {
                                                set_int(lp, var, TRUE as ::core::ffi::c_uchar);
                                                continue;
                                            }
                                        } else if strcmp(
                                            &raw mut field1 as *mut ::core::ffi::c_char,
                                            b"LI\0" as *const u8 as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            if !(set_lowbo(lp, var, field4) == 0) {
                                                set_int(lp, var, TRUE as ::core::ffi::c_uchar);
                                                continue;
                                            }
                                        } else {
                                            report(
                                                lp,
                                                1 as ::core::ffi::c_int,
                                                b"BOUND type %s on line %d is not supported\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                }
                            }
                            MPSRANGES => {
                                report(
                                    lp,
                                    6 as ::core::ffi::c_int,
                                    b"RANGES line: %s %s %g %s %g\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                if items != 4 as ::core::ffi::c_int
                                    && items != 6 as ::core::ffi::c_int
                                {
                                    report(
                                        lp,
                                        1 as ::core::ffi::c_int,
                                        b"Wrong number of items (%d) in RANGES section line %d\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                } else {
                                    row = find_row(
                                        lp,
                                        &raw mut field3 as *mut ::core::ffi::c_char,
                                        Unconstrained_rows_found,
                                    );
                                    if row >= 0 as ::core::ffi::c_int {
                                        if fabs(field4) >= (*lp).infinite {
                                            report(
                                                lp,
                                                3 as ::core::ffi::c_int,
                                                b"Warning, Range for row %s >= infinity (value %g) on line %d, ignored\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        } else if field4
                                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            if *(*lp).orig_upbo.offset(row as isize)
                                                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            {
                                                set_constr_type(lp, row, EQ);
                                            }
                                        } else if is_chsign(lp, row) != 0 {
                                            *(*lp).orig_upbo.offset(row as isize) = fabs(field4);
                                        } else if *(*lp).orig_upbo.offset(row as isize)
                                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            && field4
                                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            set_constr_type(lp, row, GE);
                                            *(*lp).orig_upbo.offset(row as isize) = field4;
                                        } else if *(*lp).orig_upbo.offset(row as isize)
                                            == (*lp).infinite
                                        {
                                            *(*lp).orig_upbo.offset(row as isize) = fabs(field4);
                                        } else if *(*lp).orig_upbo.offset(row as isize)
                                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            && field4
                                                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            set_constr_type(lp, row, LE);
                                            *(*lp).orig_upbo.offset(row as isize) = if fabs(field4)
                                                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            {
                                                0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            } else {
                                                -field4
                                            };
                                        } else {
                                            report(
                                                lp,
                                                3 as ::core::ffi::c_int,
                                                b"Cannot figure out row type, row = %d, is_chsign = %d, upbo = %g on line %d\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                    if items == 6 as ::core::ffi::c_int {
                                        row = find_row(
                                            lp,
                                            &raw mut field5 as *mut ::core::ffi::c_char,
                                            Unconstrained_rows_found,
                                        );
                                        if row >= 0 as ::core::ffi::c_int {
                                            if fabs(field6) >= (*lp).infinite {
                                                report(
                                                    lp,
                                                    3 as ::core::ffi::c_int,
                                                    b"Warning, Range for row %s >= infinity (value %g) on line %d, ignored\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            } else if field6
                                                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            {
                                                if *(*lp).orig_upbo.offset(row as isize)
                                                    != 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                {
                                                    set_constr_type(lp, row, EQ);
                                                }
                                            } else if is_chsign(lp, row) != 0 {
                                                *(*lp).orig_upbo.offset(row as isize) =
                                                    fabs(field6);
                                            } else if *(*lp).orig_upbo.offset(row as isize)
                                                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                                && field6
                                                    >= 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                            {
                                                set_constr_type(lp, row, GE);
                                                *(*lp).orig_upbo.offset(row as isize) = field6;
                                            } else if *(*lp).orig_upbo.offset(row as isize)
                                                == (*lp).infinite
                                            {
                                                *(*lp).orig_upbo.offset(row as isize) =
                                                    fabs(field6);
                                            } else if *(*lp).orig_upbo.offset(row as isize)
                                                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                                && field6
                                                    < 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                            {
                                                set_constr_type(lp, row, LE);
                                                *(*lp).orig_upbo.offset(row as isize) =
                                                    if fabs(field6)
                                                        == 0 as ::core::ffi::c_int
                                                            as ::core::ffi::c_double
                                                    {
                                                        0 as ::core::ffi::c_int
                                                            as ::core::ffi::c_double
                                                    } else {
                                                        -field6
                                                    };
                                            } else {
                                                report(
                                                    lp,
                                                    3 as ::core::ffi::c_int,
                                                    b"Cannot figure out row type, row = %d, is_chsign = %d, upbo = %g on line %d\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            }
                                        }
                                    }
                                    continue;
                                }
                            }
                            MPSSOS => {
                                report(
                                    lp,
                                    6 as ::core::ffi::c_int,
                                    b"SOS line: %s %s %g %s %g\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                if items == 0 as ::core::ffi::c_int
                                    || items > 4 as ::core::ffi::c_int
                                {
                                    report(
                                        lp,
                                        3 as ::core::ffi::c_int,
                                        b"Invalid number of items (%d) in SOS section line %d\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                } else {
                                    if strlen(&raw mut field1 as *mut ::core::ffi::c_char)
                                        == 0 as size_t
                                    {
                                        items -= 1;
                                    }
                                    if items == 1 as ::core::ffi::c_int
                                        || items == 4 as ::core::ffi::c_int
                                    {
                                        row = field1[1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            - '0' as i32;
                                        if row <= 0 as ::core::ffi::c_int
                                            || row > 9 as ::core::ffi::c_int
                                        {
                                            report(
                                                lp,
                                                3 as ::core::ffi::c_int,
                                                b"Error: Invalid SOS type %s line %d\n\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        } else {
                                            field1[0 as ::core::ffi::c_int as usize] =
                                                '\0' as i32 as ::core::ffi::c_char;
                                            if variant == 0 as ::core::ffi::c_int {
                                                if strlen(
                                                    &raw mut field3 as *mut ::core::ffi::c_char,
                                                ) == 0 as size_t
                                                {
                                                    native_only!(snprintf,
                                                        &raw mut field3 as *mut ::core::ffi::c_char,
                                                        ::core::mem::size_of::<
                                                            [::core::ffi::c_char; 1024],
                                                        >(
                                                        )
                                                            as size_t,
                                                        b"SOS_%d\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        SOS_count(lp) + 1 as ::core::ffi::c_int,
                                                    );
                                                }
                                            } else {
                                                strcpy(
                                                    &raw mut field3 as *mut ::core::ffi::c_char,
                                                    &raw mut field1 as *mut ::core::ffi::c_char,
                                                );
                                            }
                                            if items == 4 as ::core::ffi::c_int {
                                                SOS = field4 as ::core::ffi::c_int;
                                            } else {
                                                SOS = 1 as ::core::ffi::c_int;
                                            }
                                            SOS = add_SOS(
                                                lp,
                                                &raw mut field3 as *mut ::core::ffi::c_char,
                                                row,
                                                SOS,
                                                0 as ::core::ffi::c_int,
                                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                                ::core::ptr::null_mut::<::core::ffi::c_double>(),
                                            );
                                            continue;
                                        }
                                    } else {
                                        let mut field: *mut ::core::ffi::c_char =
                                            if items == 3 as ::core::ffi::c_int {
                                                &raw mut field3 as *mut ::core::ffi::c_char
                                            } else {
                                                &raw mut field2 as *mut ::core::ffi::c_char
                                            };
                                        var = find_var(lp, field, FALSE as ::core::ffi::c_uchar);
                                        if var < 0 as ::core::ffi::c_int {
                                            Column_ready = TRUE as ::core::ffi::c_uchar;
                                            if addmpscolumn(
                                                lp,
                                                FALSE as ::core::ffi::c_uchar,
                                                typeMPS,
                                                &raw mut Column_ready,
                                                &raw mut count,
                                                Last_column,
                                                Last_columnno,
                                                field,
                                            ) == 0
                                            {
                                                current_block_236 = 717878598772063298;
                                            } else {
                                                Column_ready = TRUE as ::core::ffi::c_uchar;
                                                var = find_var(
                                                    lp,
                                                    field,
                                                    TRUE as ::core::ffi::c_uchar,
                                                );
                                                current_block_236 = 9602112577253180622;
                                            }
                                        } else {
                                            current_block_236 = 9602112577253180622;
                                        }
                                        match current_block_236 {
                                            717878598772063298 => {}
                                            _ => {
                                                if !(var < 0 as ::core::ffi::c_int
                                                    || SOS < 1 as ::core::ffi::c_int)
                                                {
                                                    append_SOSrec(
                                                        *(*(*lp).SOS).sos_list.offset(
                                                            (SOS - 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        ),
                                                        1 as ::core::ffi::c_int,
                                                        &raw mut var,
                                                        &raw mut field4,
                                                    );
                                                }
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        report(
                            lp,
                            3 as ::core::ffi::c_int,
                            b"Error: Cannot handle line %d\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        break;
                    }
                }
            }
        }
        if *(&raw mut OBJNAME as *mut ::core::ffi::c_char) as ::core::ffi::c_int != 0
            && OF_found == 0
        {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"Error: Objective function specified by OBJNAME card not found\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            CompleteStatus = FALSE as ::core::ffi::c_uchar;
        }
        if CompleteStatus as ::core::ffi::c_int == FALSE {
            if (*newlp).is_null() {
                delete_lp(lp);
            }
        } else {
            if typeMPS & MPSIBM != 0 {
                let mut lower: ::core::ffi::c_double = 0.;
                let mut upper: ::core::ffi::c_double = 0.;
                var = 1 as ::core::ffi::c_int;
                while var <= (*lp).columns {
                    if is_int(lp, var) != 0 {
                        lower = get_lowbo(lp, var);
                        upper = get_upbo(lp, var);
                        if lower == 10.0f64 / DEF_INFINITE && upper == DEF_INFINITE / 10.0f64 {
                            upper = 1.0f64;
                        }
                        if lower == 10.0f64 / DEF_INFINITE {
                            lower = 0.0f64;
                        }
                        if upper == DEF_INFINITE / 10.0f64 {
                            upper = (*lp).infinite;
                        }
                        set_bounds(lp, var, lower, upper);
                    }
                    var += 1;
                }
            }
            *newlp = lp;
        }
        if !Last_column.is_null() {
            if !(Last_column as *mut ::core::ffi::c_void).is_null() {
                free(Last_column as *mut ::core::ffi::c_void);
                Last_column = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
        }
        if !Last_columnno.is_null() {
            if !(Last_columnno as *mut ::core::ffi::c_void).is_null() {
                free(Last_columnno as *mut ::core::ffi::c_void);
                Last_columnno = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
        }
    }
    return CompleteStatus;
}
#[export_name="honest_lpsolve_MPSnameFIXED"]
pub unsafe extern "C" fn MPSnameFIXED(
    mut name0: *mut ::core::ffi::c_char,
    mut name: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    native_only!(snprintf,
        name0,
        9 as size_t,
        b"%-8.8s\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    );
    return name0;
}
#[export_name="honest_lpsolve_MPSnameFREE"]
pub unsafe extern "C" fn MPSnameFREE(
    mut name0: *mut ::core::ffi::c_char,
    mut name: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if strlen(name) < 8 as size_t {
        return MPSnameFIXED(name0, name);
    } else {
        return name;
    };
}
unsafe extern "C" fn write_data(
    mut userhandle: *mut ::core::ffi::c_void,
    mut write_modeldata: Option<write_modeldata_func>,
    mut format: *mut ::core::ffi::c_char,
) {
}
#[export_name="honest_lpsolve_MPS_writefileex"]
pub unsafe extern "C" fn MPS_writefileex(
    mut lp: *mut lprec,
    mut typeMPS: ::core::ffi::c_int,
    mut userhandle: *mut ::core::ffi::c_void,
    mut write_modeldata: Option<write_modeldata_func>,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut marker: ::core::ffi::c_int = 0;
    let mut putheader: ::core::ffi::c_int = 0;
    let mut ChangeSignObj: ::core::ffi::c_int = FALSE;
    let mut idx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut idx1: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut ok: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut names_used: ::core::ffi::c_uchar = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut val: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut val1: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut MPSname: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    > = None;
    let mut numberbuffer: [::core::ffi::c_char; 15] = [0; 15];
    let mut name0: [::core::ffi::c_char; 9] = [0; 9];
    if typeMPS & MPSFIXED == MPSFIXED {
        MPSname = Some(
            MPSnameFIXED
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_char,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_char,
            >;
        ChangeSignObj = is_maxim(lp) as ::core::ffi::c_int;
    } else if typeMPS & MPSFREE == MPSFREE {
        MPSname = Some(
            MPSnameFREE
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_char,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_char,
            >;
    } else {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"MPS_writefile: unrecognized MPS name type.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    names_used = (*lp).names_used;
    if typeMPS & MPSFIXED == MPSFIXED {
        if names_used != 0 {
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns && ok as ::core::ffi::c_int != 0 {
                if !(*(*lp).col_name.offset(i as isize)).is_null()
                    && !(**(*lp).col_name.offset(i as isize)).name.is_null()
                    && is_splitvar(lp, i) == 0
                    && strlen((**(*lp).col_name.offset(i as isize)).name) > 8 as size_t
                {
                    j = 1 as ::core::ffi::c_int;
                    while j < i && ok as ::core::ffi::c_int != 0 {
                        if !(*(*lp).col_name.offset(j as isize)).is_null()
                            && !(**(*lp).col_name.offset(j as isize)).name.is_null()
                            && is_splitvar(lp, j) == 0
                        {
                            if strncmp(
                                (**(*lp).col_name.offset(i as isize)).name,
                                (**(*lp).col_name.offset(j as isize)).name,
                                8 as size_t,
                            ) == 0 as ::core::ffi::c_int
                            {
                                ok = FALSE as ::core::ffi::c_uchar;
                            }
                        }
                        j += 1;
                    }
                }
                i += 1;
            }
        }
    }
    if ok == 0 {
        (*lp).names_used = FALSE as ::core::ffi::c_uchar;
        ok = TRUE as ::core::ffi::c_uchar;
    }
    memset(
        &raw mut numberbuffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t,
    );
    marker = 0 as ::core::ffi::c_int;
    write_data(
        userhandle,
        write_modeldata,
        b"*<meta creator='lp_solve v%d.%d'>\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    write_data(
        userhandle,
        write_modeldata,
        b"*<meta rows=%d>\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    write_data(
        userhandle,
        write_modeldata,
        b"*<meta columns=%d>\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    write_data(
        userhandle,
        write_modeldata,
        b"*<meta equalities=%d>\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    if SOS_count(lp) > 0 as ::core::ffi::c_int {
        write_data(
            userhandle,
            write_modeldata,
            b"*<meta SOS=%d>\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    write_data(
        userhandle,
        write_modeldata,
        b"*<meta integers=%d>\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    if (*lp).sc_vars > 0 as ::core::ffi::c_int {
        write_data(
            userhandle,
            write_modeldata,
            b"*<meta scvars=%d>\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    write_data(
        userhandle,
        write_modeldata,
        b"*<meta origsense='%s'>\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    write_data(
        userhandle,
        write_modeldata,
        b"*\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    write_data(
        userhandle,
        write_modeldata,
        b"NAME          %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    if typeMPS & MPSFREE == MPSFREE && is_maxim(lp) as ::core::ffi::c_int != 0 {
        write_data(
            userhandle,
            write_modeldata,
            b"OBJSENSE\n MAX\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    write_data(
        userhandle,
        write_modeldata,
        b"ROWS\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if i == 0 as ::core::ffi::c_int {
            write_data(
                userhandle,
                write_modeldata,
                b" N  \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if *(*lp).orig_upbo.offset(i as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            if is_chsign(lp, i) != 0 {
                write_data(
                    userhandle,
                    write_modeldata,
                    b" G  \0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                write_data(
                    userhandle,
                    write_modeldata,
                    b" L  \0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        } else {
            write_data(
                userhandle,
                write_modeldata,
                b" E  \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        write_data(
            userhandle,
            write_modeldata,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    allocREAL(
        lp,
        &raw mut val,
        1 as ::core::ffi::c_int + (*lp).rows,
        TRUE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut idx,
        1 as ::core::ffi::c_int + (*lp).rows,
        TRUE as ::core::ffi::c_uchar,
    );
    write_data(
        userhandle,
        write_modeldata,
        b"COLUMNS\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if is_splitvar(lp, i) == 0 {
            if is_int(lp, i) as ::core::ffi::c_int != 0
                && marker % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"    MARK%04d  'MARKER'                 'INTORG'\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                marker += 1;
            }
            if is_int(lp, i) == 0 && marker % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"    MARK%04d  'MARKER'                 'INTEND'\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                marker += 1;
            }
            je = get_columnex(lp, i, val, idx);
            k = 1 as ::core::ffi::c_int;
            val1 = val;
            idx1 = idx;
            jj = 0 as ::core::ffi::c_int;
            while jj < je {
                k = 1 as ::core::ffi::c_int - k;
                let fresh4 = idx1;
                idx1 = idx1.offset(1);
                j = *fresh4;
                let fresh5 = val1;
                val1 = val1.offset(1);
                a = *fresh5;
                if k == 0 as ::core::ffi::c_int {
                    write_data(
                        userhandle,
                        write_modeldata,
                        b"    %s\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    write_data(
                        userhandle,
                        write_modeldata,
                        b"  %s  %s\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    write_data(
                        userhandle,
                        write_modeldata,
                        b"   %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                jj += 1;
            }
            if k == 0 as ::core::ffi::c_int {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    if marker % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        write_data(
            userhandle,
            write_modeldata,
            b"    MARK%04d  'MARKER'                 'INTEND'\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if !(idx as *mut ::core::ffi::c_void).is_null() {
        free(idx as *mut ::core::ffi::c_void);
        idx = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(val as *mut ::core::ffi::c_void).is_null() {
        free(val as *mut ::core::ffi::c_void);
        val = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    write_data(
        userhandle,
        write_modeldata,
        b"RHS\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    k = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        a = *(*lp).orig_rhs.offset(i as isize);
        if a != 0. {
            a = unscaled_value(lp, a, i);
            if i == 0 as ::core::ffi::c_int && typeMPS & MPSNEGOBJCONST == MPSNEGOBJCONST {
                a = -a;
            }
            if i == 0 as ::core::ffi::c_int || is_chsign(lp, i) as ::core::ffi::c_int != 0 {
                a = if fabs(a) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    -a
                };
            }
            k = 1 as ::core::ffi::c_int - k;
            if k == 0 as ::core::ffi::c_int {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"    RHS       %s  %s\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"   %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    if k == 0 as ::core::ffi::c_int {
        write_data(
            userhandle,
            write_modeldata,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    putheader = TRUE;
    k = 1 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        a = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        if *(*lp).orig_upbo.offset(i as isize) < (*lp).infinite
            && *(*lp).orig_upbo.offset(i as isize) != 0.0f64
        {
            a = *(*lp).orig_upbo.offset(i as isize);
        }
        if a != 0. {
            if putheader != 0 {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"RANGES\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                putheader = FALSE;
            }
            a = unscaled_value(lp, a, i);
            k = 1 as ::core::ffi::c_int - k;
            if k == 0 as ::core::ffi::c_int {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"    RGS       %s  %s\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                write_data(
                    userhandle,
                    write_modeldata,
                    b"   %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    if k == 0 as ::core::ffi::c_int {
        write_data(
            userhandle,
            write_modeldata,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    putheader = TRUE;
    i = (*lp).rows + 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        if is_splitvar(lp, i - (*lp).rows) == 0 {
            j = i - (*lp).rows;
            if *(*lp).orig_lowbo.offset(i as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && *(*lp).orig_upbo.offset(i as isize) < (*lp).infinite
                && *(*lp).orig_lowbo.offset(i as isize) == *(*lp).orig_upbo.offset(i as isize)
            {
                a = *(*lp).orig_upbo.offset(i as isize);
                a = unscaled_value(lp, a, i);
                if putheader != 0 {
                    write_data(
                        userhandle,
                        write_modeldata,
                        b"BOUNDS\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    putheader = FALSE;
                }
                write_data(
                    userhandle,
                    write_modeldata,
                    b" FX BND       %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else if is_binary(lp, j) != 0 {
                if putheader != 0 {
                    write_data(
                        userhandle,
                        write_modeldata,
                        b"BOUNDS\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    putheader = FALSE;
                }
                write_data(
                    userhandle,
                    write_modeldata,
                    b" BV BND       %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else if is_unbounded(lp, j) != 0 {
                if putheader != 0 {
                    write_data(
                        userhandle,
                        write_modeldata,
                        b"BOUNDS\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    putheader = FALSE;
                }
                write_data(
                    userhandle,
                    write_modeldata,
                    b" FR BND       %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                if *(*lp).orig_lowbo.offset(i as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    || is_int(lp, j) as ::core::ffi::c_int != 0
                {
                    a = *(*lp).orig_lowbo.offset(i as isize);
                    a = unscaled_value(lp, a, i);
                    if putheader != 0 {
                        write_data(
                            userhandle,
                            write_modeldata,
                            b"BOUNDS\n\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        putheader = FALSE;
                    }
                    if *(*lp).orig_lowbo.offset(i as isize) != -(*lp).infinite {
                        write_data(
                            userhandle,
                            write_modeldata,
                            b" LO BND       %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else {
                        write_data(
                            userhandle,
                            write_modeldata,
                            b" MI BND       %s\n\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
                if *(*lp).orig_upbo.offset(i as isize) < (*lp).infinite
                    || is_semicont(lp, j) as ::core::ffi::c_int != 0
                {
                    a = *(*lp).orig_upbo.offset(i as isize);
                    if a < (*lp).infinite {
                        a = unscaled_value(lp, a, i);
                    }
                    if putheader != 0 {
                        write_data(
                            userhandle,
                            write_modeldata,
                            b"BOUNDS\n\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        putheader = FALSE;
                    }
                    if is_semicont(lp, j) != 0 {
                        if is_int(lp, j) != 0 {
                            write_data(
                                userhandle,
                                write_modeldata,
                                b" SI BND       %s  %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                        } else {
                            write_data(
                                userhandle,
                                write_modeldata,
                                b" SC BND       %s  %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    } else {
                        write_data(
                            userhandle,
                            write_modeldata,
                            b" UP BND       %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
            }
        }
        i += 1;
    }
    putheader = TRUE;
    i = 0 as ::core::ffi::c_int;
    while i < SOS_count(lp) {
        let mut SOS: *mut SOSgroup = (*lp).SOS;
        if putheader != 0 {
            write_data(
                userhandle,
                write_modeldata,
                b"SOS\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            putheader = FALSE;
        }
        write_data(
            userhandle,
            write_modeldata,
            b" S%1d SOS       %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        j = 1 as ::core::ffi::c_int;
        while j <= (**(*SOS).sos_list.offset(i as isize)).size {
            write_data(
                userhandle,
                write_modeldata,
                b"    SOS       %s  %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            j += 1;
        }
        i += 1;
    }
    write_data(
        userhandle,
        write_modeldata,
        b"ENDATA\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    (*lp).names_used = names_used;
    return ok;
}
unsafe extern "C" fn write_lpdata(
    mut userhandle: *mut ::core::ffi::c_void,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    native_only!(fputs,buf, userhandle as *mut FILE);
    return 1 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_MPS_writefile"]
pub unsafe extern "C" fn MPS_writefile(
    mut lp: *mut lprec,
    mut typeMPS: ::core::ffi::c_int,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut output: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut ok: ::core::ffi::c_uchar = 0;
    if !filename.is_null() {
        output = native_only!(fopen,filename, b"w\0" as *const u8 as *const ::core::ffi::c_char);
        ok = (output != NULL as *mut FILE) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if ok == 0 {
            return ok;
        }
    } else {
        output = (*lp).outstream;
    }
    ok = MPS_writefileex(
        lp,
        typeMPS,
        output as *mut ::core::ffi::c_void,
        Some(
            write_lpdata
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if !filename.is_null() {
        native_only!(fclose,output);
    }
    return ok;
}
#[export_name="honest_lpsolve_MPS_writehandle"]
pub unsafe extern "C" fn MPS_writehandle(
    mut lp: *mut lprec,
    mut typeMPS: ::core::ffi::c_int,
    mut output: *mut FILE,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = 0;
    if !output.is_null() {
        set_outputstream(lp, output);
    }
    output = (*lp).outstream;
    ok = MPS_writefileex(
        lp,
        typeMPS,
        output as *mut ::core::ffi::c_void,
        Some(
            write_lpdata
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    );
    return ok;
}
unsafe extern "C" fn MPS_getnameidx(
    mut lp: *mut lprec,
    mut varname: *mut ::core::ffi::c_char,
    mut tryrowfirst: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut in_0: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*lp).names_used != 0 {
        in_0 = get_nameindex(lp, varname, tryrowfirst);
        if in_0 > 0 as ::core::ffi::c_int && tryrowfirst == 0 {
            in_0 += (*lp).rows;
        } else if in_0 < 0 as ::core::ffi::c_int {
            in_0 = get_nameindex(
                lp,
                varname,
                (tryrowfirst == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
            );
            if in_0 > 0 as ::core::ffi::c_int && tryrowfirst as ::core::ffi::c_int != 0 {
                in_0 += (*lp).rows;
            }
        }
    }
    if in_0 == -(1 as ::core::ffi::c_int) {
        if strncmp(
            varname,
            (if tryrowfirst as ::core::ffi::c_int != 0 {
                ROWNAMEMASK.as_ptr()
            } else {
                COLNAMEMASK.as_ptr()
            }),
            1 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            if native_only!(sscanf,
                varname.offset(1 as ::core::ffi::c_int as isize),
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut in_0,
            ) != 1 as ::core::ffi::c_int
                || in_0
                    < (if tryrowfirst as ::core::ffi::c_int != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                || in_0
                    > (if tryrowfirst as ::core::ffi::c_int != 0 {
                        (*lp).rows
                    } else {
                        (*lp).columns
                    })
            {
                in_0 = -(1 as ::core::ffi::c_int);
            }
        } else if strncmp(
            varname,
            (if tryrowfirst == 0 {
                ROWNAMEMASK.as_ptr()
            } else {
                COLNAMEMASK.as_ptr()
            }),
            1 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            if native_only!(sscanf,
                varname.offset(1 as ::core::ffi::c_int as isize),
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut in_0,
            ) != 1 as ::core::ffi::c_int
                || in_0
                    < (if tryrowfirst as ::core::ffi::c_int != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                || in_0
                    > (if tryrowfirst as ::core::ffi::c_int != 0 {
                        (*lp).rows
                    } else {
                        (*lp).columns
                    })
            {
                in_0 = -(1 as ::core::ffi::c_int);
            }
        }
    }
    return in_0;
}
#[export_name="honest_lpsolve_MPS_readBAS"]
pub unsafe extern "C" fn MPS_readBAS(
    mut lp: *mut lprec,
    mut typeMPS: ::core::ffi::c_int,
    mut filename: *mut ::core::ffi::c_char,
    mut info: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut field1: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut field2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut field3: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut field5: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut field4: ::core::ffi::c_double = 0.;
    let mut field6: ::core::ffi::c_double = 0.;
    let mut ib: ::core::ffi::c_int = 0;
    let mut in_0: ::core::ffi::c_int = 0;
    let mut items: ::core::ffi::c_int = 0;
    let mut Lineno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ok: ::core::ffi::c_uchar = 0;
    let mut input: *mut FILE = __stdinp;
    let mut scan_line: Option<
        unsafe extern "C" fn(
            *mut lprec,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    > = None;
    if typeMPS & MPSFIXED == MPSFIXED {
        scan_line = Some(
            scan_lineFIXED
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
            >;
    } else if typeMPS & MPSFREE == MPSFREE {
        scan_line = Some(
            scan_lineFREE
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
            >;
    } else {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"MPS_readBAS: unrecognized MPS line type.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    ok = (!filename.is_null() && {
        input = native_only!(fopen,filename, b"r\0" as *const u8 as *const ::core::ffi::c_char);
        !input.is_null()
    }) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if ok == 0 {
        return ok;
    }
    default_basis(lp);
    memset(
        &raw mut line as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        '\0' as i32,
        (1024 as ::core::ffi::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    );
    ok = FALSE as ::core::ffi::c_uchar;
    while !native_only!(fgets,
        &raw mut line as *mut ::core::ffi::c_char,
        BUFSIZ - 1 as ::core::ffi::c_int,
        input,
    )
    .is_null()
    {
        Lineno += 1;
        ptr = &raw mut line as *mut ::core::ffi::c_char;
        while *ptr as ::core::ffi::c_int != 0
            && isspace(*ptr as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
        {
            ptr = ptr.offset(1);
        }
        if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '*' as i32
            || *ptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *ptr as ::core::ffi::c_int == '\n' as i32
            || *ptr as ::core::ffi::c_int == '\r' as i32
        {
            report(
                lp,
                6 as ::core::ffi::c_int,
                b"Comment on line %d: %s\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            report(
                lp,
                6 as ::core::ffi::c_int,
                b"Line %6d: %s\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != ' ' as i32 {
                native_only!(sscanf,
                    &raw mut line as *mut ::core::ffi::c_char,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                );
                if strcmp(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    b"NAME\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if !info.is_null() {
                        *info = 0 as ::core::ffi::c_char;
                        ptr = (&raw mut line as *mut ::core::ffi::c_char)
                            .offset(4 as ::core::ffi::c_int as isize);
                        while *ptr as ::core::ffi::c_int != 0
                            && isspace(*ptr as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
                        {
                            ptr = ptr.offset(1);
                        }
                        in_0 = strlen(ptr) as ::core::ffi::c_int;
                        while in_0 > 0 as ::core::ffi::c_int
                            && (*ptr.offset((in_0 - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == '\r' as i32
                                || *ptr.offset((in_0 - 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32
                                || isspace(*ptr.offset((in_0 - 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int)
                                    != 0)
                        {
                            in_0 -= 1;
                        }
                        *ptr.offset(in_0 as isize) = 0 as ::core::ffi::c_char;
                        strcpy(info, ptr);
                    }
                } else if strcmp(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    b"ENDATA\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    report(
                        lp,
                        6 as ::core::ffi::c_int,
                        b"Finished reading BAS file\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    ok = TRUE as ::core::ffi::c_uchar;
                    break;
                } else {
                    report(
                        lp,
                        3 as ::core::ffi::c_int,
                        b"Unrecognized BAS line %d: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    break;
                }
            } else {
                items = scan_line.expect("non-null function pointer")(
                    lp,
                    MPSBOUNDS,
                    &raw mut line as *mut ::core::ffi::c_char,
                    &raw mut field1 as *mut ::core::ffi::c_char,
                    &raw mut field2 as *mut ::core::ffi::c_char,
                    &raw mut field3 as *mut ::core::ffi::c_char,
                    &raw mut field4,
                    &raw mut field5 as *mut ::core::ffi::c_char,
                    &raw mut field6,
                );
                if items < 0 as ::core::ffi::c_int {
                    report(
                        lp,
                        3 as ::core::ffi::c_int,
                        b"Syntax error on line %d: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    break;
                } else {
                    in_0 = MPS_getnameidx(
                        lp,
                        &raw mut field2 as *mut ::core::ffi::c_char,
                        FALSE as ::core::ffi::c_uchar,
                    );
                    if in_0 < 0 as ::core::ffi::c_int {
                        break;
                    }
                    if field1[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'X' as i32
                    {
                        ib = in_0;
                        in_0 = MPS_getnameidx(
                            lp,
                            &raw mut field3 as *mut ::core::ffi::c_char,
                            FALSE as ::core::ffi::c_uchar,
                        );
                        if in_0 < 0 as ::core::ffi::c_int {
                            break;
                        }
                        *(*lp).is_lower.offset(in_0 as isize) =
                            (field1[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                == 'L' as i32) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar;
                        *(*lp).is_basic.offset(ib as isize) = TRUE as ::core::ffi::c_uchar;
                    } else {
                        *(*lp).is_lower.offset(in_0 as isize) =
                            (field1[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                == 'L' as i32) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar;
                    }
                    *(*lp).is_basic.offset(in_0 as isize) = FALSE as ::core::ffi::c_uchar;
                }
            }
        }
    }
    ib = 0 as ::core::ffi::c_int;
    items = (*lp).sum;
    in_0 = 1 as ::core::ffi::c_int;
    while in_0 <= items {
        if *(*lp).is_basic.offset(in_0 as isize) != 0 {
            ib += 1;
            *(*lp).var_basic.offset(ib as isize) = in_0;
        }
        in_0 += 1;
    }
    native_only!(fclose,input);
    return ok;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BUFSIZ: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MPSFIXED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MPSFREE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPSIBM: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MPSNEGOBJCONST: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MPSUNDEF: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const MPSNAME: ::core::ffi::c_int = -3;
pub const MPSOBJSENSE: ::core::ffi::c_int = -2;
pub const MPSOBJNAME: ::core::ffi::c_int = -1;
pub const MPSROWS: ::core::ffi::c_int = 0;
pub const MPSCOLUMNS: ::core::ffi::c_int = 1;
pub const MPSRHS: ::core::ffi::c_int = 2;
pub const MPSBOUNDS: ::core::ffi::c_int = 3;
pub const MPSRANGES: ::core::ffi::c_int = 4;
pub const MPSSOS: ::core::ffi::c_int = 5;
pub const ROWTYPE_EMPTY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ROWTYPE_LE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ROWTYPE_GE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ROWTYPE_EQ: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ROWTYPE_OF: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ROWTYPE_OFMAX: ::core::ffi::c_int = ROWTYPE_OF + ROWTYPE_GE;
pub const ROWTYPE_OFMIN: ::core::ffi::c_int = ROWTYPE_OF + ROWTYPE_LE;
pub const LE: ::core::ffi::c_int = ROWTYPE_LE;
pub const GE: ::core::ffi::c_int = ROWTYPE_GE;
pub const EQ: ::core::ffi::c_int = ROWTYPE_EQ;
pub const DEF_INFINITE: ::core::ffi::c_double = 1.0e+30f64;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
