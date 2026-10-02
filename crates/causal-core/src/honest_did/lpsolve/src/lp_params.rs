use crate::honest_did::lpsolve::runtime::{strlen,strcpy,strcmp,strcat,strdup,strchr,strrchr};
use crate::honest_did::lpsolve::runtime::{malloc,calloc,free};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    fn __error() -> *mut ::core::ffi::c_int;
    #[link_name="honest_lpsolve_ini_create"]
    fn ini_create(filename: *mut ::core::ffi::c_char) -> *mut FILE;
    #[link_name="honest_lpsolve_ini_open"]
    fn ini_open(filename: *mut ::core::ffi::c_char) -> *mut FILE;
    #[link_name="honest_lpsolve_ini_writecomment"]
    fn ini_writecomment(fp: *mut FILE, comment: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_ini_writeheader"]
    fn ini_writeheader(
        fp: *mut FILE,
        header: *mut ::core::ffi::c_char,
        addnewline: ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_ini_writedata"]
    fn ini_writedata(fp: *mut FILE, name: *mut ::core::ffi::c_char, data: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_ini_readdata"]
    fn ini_readdata(
        fp: *mut FILE,
        data: *mut ::core::ffi::c_char,
        szdata: ::core::ffi::c_int,
        withcomment: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_ini_close"]
    fn ini_close(fp: *mut FILE);
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
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
    fn remove(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_create_hash_table"]
    fn create_hash_table(size: ::core::ffi::c_int, base: ::core::ffi::c_int) -> *mut hashtable;
    #[link_name="honest_lpsolve_free_hash_table"]
    fn free_hash_table(ht: *mut hashtable);
    #[link_name="honest_lpsolve_findhash"]
    fn findhash(name: *const ::core::ffi::c_char, ht: *mut hashtable) -> *mut hashelem;
    #[link_name="honest_lpsolve_puthash"]
    fn puthash(
        name: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        list: *mut *mut hashelem,
        ht: *mut hashtable,
    ) -> *mut hashelem;
    #[link_name="honest_lpsolve_lp_solve_version"]
    fn lp_solve_version(
        majorversion: *mut ::core::ffi::c_int,
        minorversion: *mut ::core::ffi::c_int,
        release: *mut ::core::ffi::c_int,
        build: *mut ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_is_obj_in_basis"]
    fn is_obj_in_basis(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_obj_in_basis"]
    fn set_obj_in_basis(lp: *mut lprec, obj_in_basis: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_set_simplextype"]
    fn set_simplextype(lp: *mut lprec, simplextype_0: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_simplextype"]
    fn get_simplextype(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_basiscrash"]
    fn set_basiscrash(lp: *mut lprec, mode: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_basiscrash"]
    fn get_basiscrash(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_verbose"]
    fn set_verbose(lp: *mut lprec, verbose_0: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_verbose"]
    fn get_verbose(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_timeout"]
    fn set_timeout(lp: *mut lprec, sectimeout: ::core::ffi::c_long);
    #[link_name="honest_lpsolve_get_timeout"]
    fn get_timeout(lp: *mut lprec) -> ::core::ffi::c_long;
    #[link_name="honest_lpsolve_set_print_sol"]
    fn set_print_sol(lp: *mut lprec, print_sol_0: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_print_sol"]
    fn get_print_sol(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_debug"]
    fn set_debug(lp: *mut lprec, debug: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_is_debug"]
    fn is_debug(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_trace"]
    fn set_trace(lp: *mut lprec, trace: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_is_trace"]
    fn is_trace(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_anti_degen"]
    fn set_anti_degen(lp: *mut lprec, anti_degen_0: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_anti_degen"]
    fn get_anti_degen(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_presolve"]
    fn set_presolve(lp: *mut lprec, presolvemode: ::core::ffi::c_int, maxloops: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_presolve"]
    fn get_presolve(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_get_presolveloops"]
    fn get_presolveloops(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_maxpivot"]
    fn set_maxpivot(lp: *mut lprec, max_num_inv: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_maxpivot"]
    fn get_maxpivot(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_obj_bound"]
    fn set_obj_bound(lp: *mut lprec, obj_bound: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_obj_bound"]
    fn get_obj_bound(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_mip_gap"]
    fn set_mip_gap(lp: *mut lprec, absolute: ::core::ffi::c_uchar, mip_gap: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_mip_gap"]
    fn get_mip_gap(lp: *mut lprec, absolute: ::core::ffi::c_uchar) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_bb_rule"]
    fn set_bb_rule(lp: *mut lprec, bb_rule_0: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_bb_rule"]
    fn get_bb_rule(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_infinite"]
    fn set_infinite(lp: *mut lprec, infinite: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_infinite"]
    fn get_infinite(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_epsint"]
    fn set_epsint(lp: *mut lprec, epsint: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_epsint"]
    fn get_epsint(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_epsb"]
    fn set_epsb(lp: *mut lprec, epsb: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_epsb"]
    fn get_epsb(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_epsd"]
    fn set_epsd(lp: *mut lprec, epsd: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_epsd"]
    fn get_epsd(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_epsel"]
    fn set_epsel(lp: *mut lprec, epsel: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_epsel"]
    fn get_epsel(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_scaling"]
    fn set_scaling(lp: *mut lprec, scalemode: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_scaling"]
    fn get_scaling(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_scalelimit"]
    fn set_scalelimit(lp: *mut lprec, scalelimit: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_scalelimit"]
    fn get_scalelimit(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_improve"]
    fn set_improve(lp: *mut lprec, improve_0: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_improve"]
    fn get_improve(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_pivoting"]
    fn set_pivoting(lp: *mut lprec, piv_rule: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_pivoting"]
    fn get_pivoting(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_break_at_first"]
    fn set_break_at_first(lp: *mut lprec, break_at_first: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_is_break_at_first"]
    fn is_break_at_first(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_bb_floorfirst"]
    fn set_bb_floorfirst(lp: *mut lprec, bb_floorfirst_0: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_bb_floorfirst"]
    fn get_bb_floorfirst(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_bb_depthlimit"]
    fn set_bb_depthlimit(lp: *mut lprec, bb_maxlevel: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_get_bb_depthlimit"]
    fn get_bb_depthlimit(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_break_at_value"]
    fn set_break_at_value(lp: *mut lprec, break_at_value: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_break_at_value"]
    fn get_break_at_value(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_negrange"]
    fn set_negrange(lp: *mut lprec, negrange: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_negrange"]
    fn get_negrange(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_epsperturb"]
    fn set_epsperturb(lp: *mut lprec, epsperturb: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_epsperturb"]
    fn get_epsperturb(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_epspivot"]
    fn set_epspivot(lp: *mut lprec, epspivot: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_get_epspivot"]
    fn get_epspivot(lp: *mut lprec) -> ::core::ffi::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _functions {
    pub par: *mut ::core::ffi::c_char,
    pub get_function: C2RustUnnamed_0,
    pub set_function: C2RustUnnamed,
    pub type_0: ::core::ffi::c_int,
    pub values: *mut _values,
    pub elements: ::core::ffi::c_int,
    pub basemask: ::core::ffi::c_uint,
    pub mask: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _values {
    pub value: ::core::ffi::c_int,
    pub svalue: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub int_set_function: Option<fn_int_set_function>,
    pub long_set_function: Option<fn_long_set_function>,
    pub MYBOOL_set_function: Option<fn_MYBOOL_set_function>,
    pub REAL_set_function: Option<fn_REAL_set_function>,
}
pub type fn_REAL_set_function = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ();
pub type fn_MYBOOL_set_function = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ();
pub type fn_long_set_function = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_long) -> ();
pub type fn_int_set_function = unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub int_get_function: Option<fn_int_get_function>,
    pub long_get_function: Option<fn_long_get_function>,
    pub MYBOOL_get_function: Option<fn_MYBOOL_get_function>,
    pub REAL_get_function: Option<fn_REAL_get_function>,
}
pub type fn_REAL_get_function = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double;
pub type fn_MYBOOL_get_function = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar;
pub type fn_long_get_function = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_long;
pub type fn_int_get_function = unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int;
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
unsafe extern "C" fn tolower(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return native_only!(__tolower,_c as __darwin_ct_rune_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toupper(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return native_only!(__toupper,_c as __darwin_ct_rune_t) as ::core::ffi::c_int;
}
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EACCES: ::core::ffi::c_int = 13;
pub const intfunction: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const longfunction: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MYBOOLfunction: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const REALfunction: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const WRITE_COMMENTED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRITE_ACTIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut anti_degen: [_values; 11] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_NONE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_FIXEDVARS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_COLUMNCHECK\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_STALLING\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_NUMFAILURE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_LOSTFEAS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 32 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_INFEASIBLE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 64 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_DYNAMIC\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 128 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_DURINGBB\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 256 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_RHSPERTURB\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 512 as ::core::ffi::c_int,
        svalue: b"ANTIDEGEN_BOUNDFLIP\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut basiscrash: [_values; 3] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"CRASH_NONE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"CRASH_MOSTFEASIBLE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 3 as ::core::ffi::c_int,
        svalue: b"CRASH_LEASTDEGENERATE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut bb_floorfirst: [_values; 3] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"BRANCH_CEILING\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"BRANCH_FLOOR\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"BRANCH_AUTOMATIC\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut bb_rule: [_values; 21] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"NODE_FIRSTSELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"NODE_GAPSELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"NODE_RANGESELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 3 as ::core::ffi::c_int,
        svalue: b"NODE_FRACTIONSELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4 as ::core::ffi::c_int,
        svalue: b"NODE_PSEUDOCOSTSELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 5 as ::core::ffi::c_int,
        svalue: b"NODE_PSEUDONONINTSELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 6 as ::core::ffi::c_int,
        svalue: b"NODE_PSEUDORATIOSELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 7 as ::core::ffi::c_int,
        svalue: b"NODE_USERSELECT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8 as ::core::ffi::c_int,
        svalue: b"NODE_WEIGHTREVERSEMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16 as ::core::ffi::c_int,
        svalue: b"NODE_BRANCHREVERSEMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 32 as ::core::ffi::c_int,
        svalue: b"NODE_GREEDYMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 64 as ::core::ffi::c_int,
        svalue: b"NODE_PSEUDOCOSTMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 128 as ::core::ffi::c_int,
        svalue: b"NODE_DEPTHFIRSTMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 256 as ::core::ffi::c_int,
        svalue: b"NODE_RANDOMIZEMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 512 as ::core::ffi::c_int,
        svalue: b"NODE_GUBMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1024 as ::core::ffi::c_int,
        svalue: b"NODE_DYNAMICMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2048 as ::core::ffi::c_int,
        svalue: b"NODE_RESTARTMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4096 as ::core::ffi::c_int,
        svalue: b"NODE_BREADTHFIRSTMODE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8192 as ::core::ffi::c_int,
        svalue: b"NODE_AUTOORDER\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16384 as ::core::ffi::c_int,
        svalue: b"NODE_RCOSTFIXING\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 32768 as ::core::ffi::c_int,
        svalue: b"NODE_STRONGINIT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut improve: [_values; 5] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"IMPROVE_NONE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"IMPROVE_SOLUTION\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"IMPROVE_DUALFEAS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4 as ::core::ffi::c_int,
        svalue: b"IMPROVE_THETAGAP\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8 as ::core::ffi::c_int,
        svalue: b"IMPROVE_BBSIMPLEX\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
unsafe extern "C" fn get_mip_gap_abs(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return get_mip_gap(lp, TRUE as ::core::ffi::c_uchar);
}
unsafe extern "C" fn get_mip_gap_rel(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return get_mip_gap(lp, FALSE as ::core::ffi::c_uchar);
}
unsafe extern "C" fn set_mip_gap_abs(mut lp: *mut lprec, mut mip_gap: ::core::ffi::c_double) {
    set_mip_gap(lp, TRUE as ::core::ffi::c_uchar, mip_gap);
}
unsafe extern "C" fn set_mip_gap_rel(mut lp: *mut lprec, mut mip_gap: ::core::ffi::c_double) {
    set_mip_gap(lp, FALSE as ::core::ffi::c_uchar, mip_gap);
}
static mut pivoting: [_values; 14] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"PRICER_FIRSTINDEX\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"PRICER_DANTZIG\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"PRICER_DEVEX\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 3 as ::core::ffi::c_int,
        svalue: b"PRICER_STEEPESTEDGE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4 as ::core::ffi::c_int,
        svalue: b"PRICE_PRIMALFALLBACK\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8 as ::core::ffi::c_int,
        svalue: b"PRICE_MULTIPLE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16 as ::core::ffi::c_int,
        svalue: b"PRICE_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 32 as ::core::ffi::c_int,
        svalue: b"PRICE_ADAPTIVE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 128 as ::core::ffi::c_int,
        svalue: b"PRICE_RANDOMIZE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 256 as ::core::ffi::c_int,
        svalue: b"PRICE_AUTOPARTIAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1024 as ::core::ffi::c_int,
        svalue: b"PRICE_LOOPLEFT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2048 as ::core::ffi::c_int,
        svalue: b"PRICE_LOOPALTERNATE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4096 as ::core::ffi::c_int,
        svalue: b"PRICE_HARRISTWOPASS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16384 as ::core::ffi::c_int,
        svalue: b"PRICE_TRUENORMINIT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut presolving: [_values; 22] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_NONE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_ROWS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_COLS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_LINDEP\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_AGGREGATE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_SPARSER\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 32 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_SOS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 64 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_REDUCEMIP\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 128 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_KNAPSACK\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 256 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_ELIMEQ2\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 512 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_IMPLIEDFREE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1024 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_REDUCEGCD\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2048 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_PROBEFIX\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4096 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_PROBEREDUCE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8192 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_ROWDOMINATE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16384 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_COLDOMINATE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 32768 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_MERGEROWS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 65536 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_IMPLIEDSLK\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 131072 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_COLFIXDUAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 262144 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_BOUNDS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 524288 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_DUALS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1048576 as ::core::ffi::c_int,
        svalue: b"PRESOLVE_SENSDUALS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
unsafe extern "C" fn STRLWR(mut str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ptr = str;
    while *ptr != 0 {
        *ptr = tolower(*ptr as ::core::ffi::c_uchar as ::core::ffi::c_int) as ::core::ffi::c_char;
        ptr = ptr.offset(1);
    }
    return str;
}
unsafe extern "C" fn STRUPR(mut str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ptr = str;
    while *ptr != 0 {
        *ptr = toupper(*ptr as ::core::ffi::c_uchar as ::core::ffi::c_int) as ::core::ffi::c_char;
        ptr = ptr.offset(1);
    }
    return str;
}
unsafe extern "C" fn set_presolve1(mut lp: *mut lprec, mut do_presolve: ::core::ffi::c_int) {
    set_presolve(lp, do_presolve, get_presolveloops(lp));
}
unsafe extern "C" fn set_presolve2(mut lp: *mut lprec, mut maxloops: ::core::ffi::c_int) {
    set_presolve(lp, get_presolve(lp), maxloops);
}
static mut print_sol: [_values; 3] = [
    _values {
        value: FALSE,
        svalue: b"0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    _values {
        value: TRUE,
        svalue: b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"AUTOMATIC\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut scaling: [_values; 15] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"SCALE_NONE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"SCALE_EXTREME\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"SCALE_RANGE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 3 as ::core::ffi::c_int,
        svalue: b"SCALE_MEAN\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4 as ::core::ffi::c_int,
        svalue: b"SCALE_GEOMETRIC\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 7 as ::core::ffi::c_int,
        svalue: b"SCALE_CURTISREID\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 8 as ::core::ffi::c_int,
        svalue: b"SCALE_QUADRATIC\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 16 as ::core::ffi::c_int,
        svalue: b"SCALE_LOGARITHMIC\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 31 as ::core::ffi::c_int,
        svalue: b"SCALE_USERWEIGHT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 32 as ::core::ffi::c_int,
        svalue: b"SCALE_POWER2\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 64 as ::core::ffi::c_int,
        svalue: b"SCALE_EQUILIBRATE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 128 as ::core::ffi::c_int,
        svalue: b"SCALE_INTEGERS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 256 as ::core::ffi::c_int,
        svalue: b"SCALE_DYNUPDATE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 512 as ::core::ffi::c_int,
        svalue: b"SCALE_ROWSONLY\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1024 as ::core::ffi::c_int,
        svalue: b"SCALE_COLSONLY\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut simplextype: [_values; 4] = [
    _values {
        value: 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
        svalue: b"SIMPLEX_PRIMAL_PRIMAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
        svalue: b"SIMPLEX_DUAL_PRIMAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int,
        svalue: b"SIMPLEX_PRIMAL_DUAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int + 8 as ::core::ffi::c_int,
        svalue: b"SIMPLEX_DUAL_DUAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
static mut verbose: [_values; 7] = [
    _values {
        value: 0 as ::core::ffi::c_int,
        svalue: b"NEUTRAL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    _values {
        value: 1 as ::core::ffi::c_int,
        svalue: b"CRITICAL\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 2 as ::core::ffi::c_int,
        svalue: b"SEVERE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    _values {
        value: 3 as ::core::ffi::c_int,
        svalue: b"IMPORTANT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 4 as ::core::ffi::c_int,
        svalue: b"NORMAL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    _values {
        value: 5 as ::core::ffi::c_int,
        svalue: b"DETAILED\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    _values {
        value: 6 as ::core::ffi::c_int,
        svalue: b"FULL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
];
static mut functions: [_functions; 32] = [_functions {
    par: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    get_function: C2RustUnnamed_0 {
        int_get_function: None,
    },
    set_function: C2RustUnnamed {
        int_set_function: None,
    },
    type_0: 0,
    values: ::core::ptr::null_mut::<_values>(),
    elements: 0,
    basemask: 0,
    mask: 0,
}; 32];
unsafe extern "C" fn write_params1(
    mut lp: *mut lprec,
    mut fp: *mut FILE,
    mut header: *mut ::core::ffi::c_char,
    mut newline: ::core::ffi::c_int,
) {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret2: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_int = 0;
    let mut value2: ::core::ffi::c_int = 0;
    let mut elements: ::core::ffi::c_int = 0;
    let mut majorversion: ::core::ffi::c_int = 0;
    let mut minorversion: ::core::ffi::c_int = 0;
    let mut release: ::core::ffi::c_int = 0;
    let mut build: ::core::ffi::c_int = 0;
    let mut basemask: ::core::ffi::c_uint = 0;
    let mut a: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut par: [::core::ffi::c_char; 20] = [0; 20];
    ini_writeheader(fp, header, newline);
    lp_solve_version(
        &raw mut majorversion,
        &raw mut minorversion,
        &raw mut release,
        &raw mut build,
    );
    native_only!(snprintf,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        b"lp_solve version %d.%d settings\n\0" as *const u8 as *const ::core::ffi::c_char,
        majorversion,
        minorversion,
    );
    ini_writecomment(fp, &raw mut buf as *mut ::core::ffi::c_char);
    let mut current_block_46: u64;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[_functions; 32]>() as usize)
            .wrapping_div(::core::mem::size_of::<_functions>() as usize)
    {
        match functions[i as usize].type_0 {
            intfunction => {
                if functions[i as usize]
                    .get_function
                    .int_get_function
                    .is_none()
                {
                    current_block_46 = 735147466149431745;
                } else {
                    ret = functions[i as usize]
                        .get_function
                        .int_get_function
                        .expect("non-null function pointer")(lp);
                    current_block_46 = 8831408221741692167;
                }
            }
            longfunction => {
                if functions[i as usize]
                    .get_function
                    .long_get_function
                    .is_none()
                {
                    current_block_46 = 735147466149431745;
                } else {
                    ret = functions[i as usize]
                        .get_function
                        .long_get_function
                        .expect("non-null function pointer")(lp)
                        as ::core::ffi::c_int;
                    current_block_46 = 8831408221741692167;
                }
            }
            MYBOOLfunction => {
                if functions[i as usize]
                    .get_function
                    .MYBOOL_get_function
                    .is_none()
                {
                    current_block_46 = 735147466149431745;
                } else {
                    ret = functions[i as usize]
                        .get_function
                        .MYBOOL_get_function
                        .expect("non-null function pointer")(lp)
                        as ::core::ffi::c_int;
                    current_block_46 = 8831408221741692167;
                }
            }
            REALfunction => {
                if functions[i as usize]
                    .get_function
                    .REAL_get_function
                    .is_none()
                {
                    current_block_46 = 735147466149431745;
                } else {
                    a = functions[i as usize]
                        .get_function
                        .REAL_get_function
                        .expect("non-null function pointer")(lp);
                    current_block_46 = 8831408221741692167;
                }
            }
            _ => {
                current_block_46 = 8831408221741692167;
            }
        }
        match current_block_46 {
            8831408221741692167 => {
                buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                if functions[i as usize].values.is_null() {
                    match functions[i as usize].type_0 {
                        intfunction | longfunction | MYBOOLfunction => {
                            native_only!(snprintf,
                                &raw mut buf as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
                                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                                ret,
                            );
                        }
                        REALfunction => {
                            native_only!(snprintf,
                                &raw mut buf as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
                                b"%g\0" as *const u8 as *const ::core::ffi::c_char,
                                a,
                            );
                        }
                        _ => {}
                    }
                } else {
                    elements = functions[i as usize].elements;
                    basemask = functions[i as usize].basemask;
                    j = 0 as ::core::ffi::c_int;
                    while j < elements {
                        value = (*functions[i as usize].values.offset(j as isize)).value;
                        ret2 = ret;
                        if (value as ::core::ffi::c_uint) < basemask {
                            ret2 = (ret2 as ::core::ffi::c_uint & basemask) as ::core::ffi::c_int;
                        }
                        if value == 0 as ::core::ffi::c_int {
                            if ret2 == 0 as ::core::ffi::c_int {
                                if *(&raw mut buf as *mut ::core::ffi::c_char) != 0 {
                                    strcat(
                                        &raw mut buf as *mut ::core::ffi::c_char,
                                        b" + \0" as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                                strcat(
                                    &raw mut buf as *mut ::core::ffi::c_char,
                                    (*functions[i as usize].values.offset(j as isize)).svalue,
                                );
                            }
                        } else if ret2 & value == value {
                            k = 0 as ::core::ffi::c_int;
                            while k < elements {
                                value2 = (*functions[i as usize].values.offset(k as isize)).value;
                                if k != j
                                    && value2 > value
                                    && value2 & value == value
                                    && ret2 & value2 == value2
                                {
                                    break;
                                }
                                k += 1;
                            }
                            if k == elements {
                                if *(&raw mut buf as *mut ::core::ffi::c_char) != 0 {
                                    strcat(
                                        &raw mut buf as *mut ::core::ffi::c_char,
                                        b" + \0" as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                                strcat(
                                    &raw mut buf as *mut ::core::ffi::c_char,
                                    (*functions[i as usize].values.offset(j as isize)).svalue,
                                );
                            }
                        }
                        j += 1;
                    }
                }
                if functions[i as usize].mask & WRITE_ACTIVE != 0 {
                    par[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                } else {
                    strcpy(
                        &raw mut par as *mut ::core::ffi::c_char,
                        b";\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                strcat(
                    &raw mut par as *mut ::core::ffi::c_char,
                    functions[i as usize].par,
                );
                ini_writedata(
                    fp,
                    STRLWR(&raw mut par as *mut ::core::ffi::c_char),
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn readoptions(
    mut options: *mut ::core::ffi::c_char,
    mut header: *mut *mut ::core::ffi::c_char,
) {
    let mut ptr1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !options.is_null() {
        ptr1 = options;
        while *ptr1 != 0 {
            ptr2 = strchr(ptr1, '-' as i32);
            if ptr2.is_null() {
                break;
            }
            ptr2 = ptr2.offset(1);
            if tolower(*ptr2 as ::core::ffi::c_uchar as ::core::ffi::c_int) == 'h' as i32 {
                ptr2 = ptr2.offset(1);
                while *ptr2 as ::core::ffi::c_int != 0 && isspace(*ptr2 as ::core::ffi::c_int) != 0
                {
                    ptr2 = ptr2.offset(1);
                }
                ptr1 = ptr2;
                while *ptr1 as ::core::ffi::c_int != 0 && isspace(*ptr1 as ::core::ffi::c_int) == 0
                {
                    ptr1 = ptr1.offset(1);
                }
                *header = calloc(
                    (1 as ::core::ffi::c_int
                        + ptr1.offset_from(ptr2) as ::core::ffi::c_long as ::core::ffi::c_int)
                        as size_t,
                    1 as size_t,
                ) as *mut ::core::ffi::c_char;
                memcpy(
                    *header as *mut ::core::ffi::c_void,
                    ptr2 as *const ::core::ffi::c_void,
                    ptr1.offset_from(ptr2) as ::core::ffi::c_long as ::core::ffi::c_int as size_t,
                );
            }
        }
    }
    if (*header).is_null() {
        *header = strdup(b"Default\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[export_name="honest_lpsolve_write_params"]
pub unsafe extern "C" fn write_params(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
    mut options: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut k: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut params_written: ::core::ffi::c_int = 0;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fp0: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut state: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut looping: ::core::ffi::c_int = 0;
    let mut newline: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut filename0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut header: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    readoptions(options, &raw mut header);
    k = strlen(filename) as ::core::ffi::c_int;
    filename0 = malloc((k + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
        as *mut ::core::ffi::c_char;
    strcpy(filename0, filename);
    ptr1 = strrchr(filename0, '.' as i32);
    ptr2 = strrchr(filename0, '\\' as i32);
    if ptr1.is_null() || !ptr2.is_null() && ptr1 < ptr2 {
        ptr1 = filename0.offset(k as isize);
    }
    memmove(
        ptr1.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        ptr1 as *const ::core::ffi::c_void,
        (k + 1 as ::core::ffi::c_int
            - ptr1.offset_from(filename0) as ::core::ffi::c_long as ::core::ffi::c_int)
            as size_t,
    );
    *ptr1.offset(0 as ::core::ffi::c_int as isize) = '_' as i32 as ::core::ffi::c_char;
    if native_only!(rename,filename, filename0) != 0 {
        match *native_only!(__error,) {
            ENOENT => {
                if !(filename0 as *mut ::core::ffi::c_void).is_null() {
                    free(filename0 as *mut ::core::ffi::c_void);
                    filename0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                filename0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            EACCES => {
                if !(filename0 as *mut ::core::ffi::c_void).is_null() {
                    free(filename0 as *mut ::core::ffi::c_void);
                    filename0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if !(header as *mut ::core::ffi::c_void).is_null() {
                    free(header as *mut ::core::ffi::c_void);
                    header = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                return 0 as ::core::ffi::c_uchar;
            }
            _ => {}
        }
    }
    fp = ini_create(filename);
    if fp.is_null() {
        ret = FALSE;
    } else {
        params_written = FALSE;
        newline = TRUE;
        if !filename0.is_null() {
            fp0 = ini_open(filename0);
            if fp0.is_null() {
                native_only!(rename,filename0, filename);
                if !(filename0 as *mut ::core::ffi::c_void).is_null() {
                    free(filename0 as *mut ::core::ffi::c_void);
                    filename0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if !(header as *mut ::core::ffi::c_void).is_null() {
                    free(header as *mut ::core::ffi::c_void);
                    header = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                return 0 as ::core::ffi::c_uchar;
            }
            looping = TRUE;
            while looping != 0 {
                match ini_readdata(
                    fp0,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
                    TRUE,
                ) {
                    0 => {
                        looping = FALSE;
                    }
                    1 => {
                        ptr1 = strdup(&raw mut buf as *mut ::core::ffi::c_char);
                        STRUPR(&raw mut buf as *mut ::core::ffi::c_char);
                        ptr2 = strdup(header);
                        STRUPR(ptr2);
                        if strcmp(&raw mut buf as *mut ::core::ffi::c_char, ptr2)
                            == 0 as ::core::ffi::c_int
                        {
                            write_params1(lp, fp, ptr1, newline);
                            params_written = TRUE;
                            newline = TRUE;
                            state = 1 as ::core::ffi::c_int;
                        } else {
                            state = 0 as ::core::ffi::c_int;
                            ini_writeheader(fp, ptr1, newline);
                            newline = TRUE;
                        }
                        if !(ptr2 as *mut ::core::ffi::c_void).is_null() {
                            free(ptr2 as *mut ::core::ffi::c_void);
                            ptr2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        if !(ptr1 as *mut ::core::ffi::c_void).is_null() {
                            free(ptr1 as *mut ::core::ffi::c_void);
                            ptr1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                    }
                    2 => {
                        if state == 0 as ::core::ffi::c_int {
                            ini_writedata(
                                fp,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                &raw mut buf as *mut ::core::ffi::c_char,
                            );
                            newline = (*(&raw mut buf as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int;
                        }
                    }
                    _ => {}
                }
            }
            ini_close(fp0);
        }
        if params_written == 0 {
            write_params1(lp, fp, header, newline);
        }
        ini_close(fp);
        ret = TRUE;
    }
    if !filename0.is_null() {
        native_only!(remove,filename0);
        if !(filename0 as *mut ::core::ffi::c_void).is_null() {
            free(filename0 as *mut ::core::ffi::c_void);
            filename0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if !(header as *mut ::core::ffi::c_void).is_null() {
        free(header as *mut ::core::ffi::c_void);
        header = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return ret as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_read_params"]
pub unsafe extern "C" fn read_params(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
    mut options: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut ret: ::core::ffi::c_int = 0;
    let mut looping: ::core::ffi::c_int = 0;
    let mut line: ::core::ffi::c_int = 0;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut hashfunctions: *mut hashtable = ::core::ptr::null_mut::<hashtable>();
    let mut hashparameters: *mut hashtable = ::core::ptr::null_mut::<hashtable>();
    let mut hp: *mut hashelem = ::core::ptr::null_mut::<hashelem>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut elements: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut intvalue: ::core::ffi::c_int = 0;
    let mut state: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut REALvalue: ::core::ffi::c_double = 0.;
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut header: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    fp = ini_open(filename);
    if fp.is_null() {
        ret = FALSE;
    } else {
        hashfunctions = create_hash_table(
            (::core::mem::size_of::<[_functions; 32]>() as usize)
                .wrapping_div(::core::mem::size_of::<_functions>() as usize)
                as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        n = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[_functions; 32]>() as usize)
                .wrapping_div(::core::mem::size_of::<_functions>() as usize)
                as ::core::ffi::c_int
        {
            puthash(
                functions[i as usize].par,
                i,
                ::core::ptr::null_mut::<*mut hashelem>(),
                hashfunctions,
            );
            if !functions[i as usize].values.is_null() {
                n += functions[i as usize].elements;
            }
            i += 1;
        }
        hashparameters = create_hash_table(n, 0 as ::core::ffi::c_int);
        n = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[_functions; 32]>() as usize)
                .wrapping_div(::core::mem::size_of::<_functions>() as usize)
                as ::core::ffi::c_int
        {
            if !functions[i as usize].values.is_null() {
                elements = functions[i as usize].elements;
                j = 0 as ::core::ffi::c_int;
                while j < elements {
                    if strcmp(
                        (*functions[i as usize].values.offset(j as isize)).svalue,
                        b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                        && strcmp(
                            (*functions[i as usize].values.offset(j as isize)).svalue,
                            b"1\0" as *const u8 as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                    {
                        puthash(
                            (*functions[i as usize].values.offset(j as isize)).svalue,
                            j,
                            ::core::ptr::null_mut::<*mut hashelem>(),
                            hashparameters,
                        );
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        readoptions(options, &raw mut header);
        STRUPR(header);
        looping = TRUE;
        ret = looping;
        line = 0 as ::core::ffi::c_int;
        while ret != 0 && looping != 0 {
            line += 1;
            match ini_readdata(
                fp,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
                FALSE,
            ) {
                0 => {
                    looping = FALSE;
                }
                1 => match state {
                    0 => {
                        STRUPR(&raw mut buf as *mut ::core::ffi::c_char);
                        if strcmp(&raw mut buf as *mut ::core::ffi::c_char, header)
                            == 0 as ::core::ffi::c_int
                        {
                            state = 1 as ::core::ffi::c_int;
                        }
                    }
                    1 => {
                        looping = FALSE;
                    }
                    _ => {}
                },
                2 => {
                    if state == 1 as ::core::ffi::c_int {
                        ptr = &raw mut buf as *mut ::core::ffi::c_char;
                        while *ptr as ::core::ffi::c_int != 0
                            && isspace(*ptr as ::core::ffi::c_int) != 0
                        {
                            ptr = ptr.offset(1);
                        }
                    } else {
                        ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if !ptr.is_null() && *ptr as ::core::ffi::c_int != 0 {
                        STRUPR(&raw mut buf as *mut ::core::ffi::c_char);
                        ptr = strchr(&raw mut buf as *mut ::core::ffi::c_char, '=' as i32);
                        if ptr.is_null() {
                            report(
                                lp,
                                3 as ::core::ffi::c_int,
                                b"read_params: No equal sign on line %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            ret = FALSE;
                        } else {
                            *ptr = 0 as ::core::ffi::c_char;
                            ptr1 = &raw mut buf as *mut ::core::ffi::c_char;
                            while isspace(*ptr1 as ::core::ffi::c_int) != 0 {
                                ptr1 = ptr1.offset(1);
                            }
                            ptr2 = ptr.offset(-(1 as ::core::ffi::c_int as isize));
                            while ptr2 >= ptr1 && isspace(*ptr2 as ::core::ffi::c_int) != 0 {
                                ptr2 = ptr2.offset(-1);
                            }
                            if ptr2 <= ptr1 {
                                report(
                                    lp,
                                    3 as ::core::ffi::c_int,
                                    b"read_params: No parameter name before equal sign on line %d\n\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                ret = FALSE;
                            } else {
                                *ptr2.offset(1 as ::core::ffi::c_int as isize) =
                                    0 as ::core::ffi::c_char;
                                hp = findhash(ptr1, hashfunctions);
                                if hp.is_null() {
                                    report(
                                        lp,
                                        3 as ::core::ffi::c_int,
                                        b"read_params: Unknown parameter name (%s) before equal sign on line %d\n\0"
                                            as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                    ret = FALSE;
                                } else {
                                    i = (*hp).index;
                                    ptr = ptr.offset(1);
                                    ptr1 = ptr;
                                    intvalue = 0 as ::core::ffi::c_int;
                                    REALvalue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                    if functions[i as usize].values.is_null() {
                                        match functions[i as usize].type_0 {
                                            intfunction | longfunction | MYBOOLfunction => {
                                                intvalue = native_only!(strtol,
                                                    ptr1,
                                                    &raw mut ptr2,
                                                    10 as ::core::ffi::c_int,
                                                )
                                                    as ::core::ffi::c_int;
                                                while *ptr2 as ::core::ffi::c_int != 0
                                                    && isspace(*ptr2 as ::core::ffi::c_int) != 0
                                                {
                                                    ptr2 = ptr2.offset(1);
                                                }
                                                if *ptr2 != 0 {
                                                    report(
                                                        lp,
                                                        3 as ::core::ffi::c_int,
                                                        b"read_params: Invalid integer value on line %d\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    ret = FALSE;
                                                }
                                            }
                                            REALfunction => {
                                                REALvalue = native_only!(strtod,ptr1, &raw mut ptr2);
                                                while *ptr2 as ::core::ffi::c_int != 0
                                                    && isspace(*ptr2 as ::core::ffi::c_int) != 0
                                                {
                                                    ptr2 = ptr2.offset(1);
                                                }
                                                if *ptr2 != 0 {
                                                    report(
                                                        lp,
                                                        3 as ::core::ffi::c_int,
                                                        b"read_params: Invalid real value on line %d\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    ret = FALSE;
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        while ret != 0 {
                                            ptr = strchr(ptr1, '+' as i32);
                                            if ptr.is_null() {
                                                ptr = ptr1.offset(strlen(ptr1) as isize);
                                            }
                                            while isspace(*ptr1 as ::core::ffi::c_int) != 0 {
                                                ptr1 = ptr1.offset(1);
                                            }
                                            ptr2 = ptr.offset(-(1 as ::core::ffi::c_int as isize));
                                            while ptr2 >= ptr1
                                                && isspace(*ptr2 as ::core::ffi::c_int) != 0
                                            {
                                                ptr2 = ptr2.offset(-1);
                                            }
                                            if ptr2 <= ptr1 {
                                                break;
                                            }
                                            *ptr2.offset(1 as ::core::ffi::c_int as isize) =
                                                0 as ::core::ffi::c_char;
                                            hp = findhash(ptr1, hashparameters);
                                            if hp.is_null() {
                                                report(
                                                    lp,
                                                    3 as ::core::ffi::c_int,
                                                    b"read_params: Invalid parameter name (%s) on line %d\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                                ret = FALSE;
                                            } else {
                                                j = (*hp).index;
                                                if j >= functions[i as usize].elements
                                                    || strcmp(
                                                        (*functions[i as usize]
                                                            .values
                                                            .offset(j as isize))
                                                        .svalue,
                                                        ptr1,
                                                    ) != 0
                                                {
                                                    report(
                                                        lp,
                                                        3 as ::core::ffi::c_int,
                                                        b"read_params: Inappropriate parameter name (%s) on line %d\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    ret = FALSE;
                                                } else {
                                                    intvalue += (*functions[i as usize]
                                                        .values
                                                        .offset(j as isize))
                                                    .value;
                                                }
                                            }
                                            ptr1 = ptr.offset(1 as ::core::ffi::c_int as isize);
                                        }
                                    }
                                    if ret != 0 {
                                        match functions[i as usize].type_0 {
                                            intfunction => {
                                                functions[i as usize]
                                                    .set_function
                                                    .int_set_function
                                                    .expect("non-null function pointer")(
                                                    lp, intvalue,
                                                );
                                            }
                                            longfunction => {
                                                functions[i as usize]
                                                    .set_function
                                                    .long_set_function
                                                    .expect("non-null function pointer")(
                                                    lp,
                                                    intvalue as ::core::ffi::c_long,
                                                );
                                            }
                                            MYBOOLfunction => {
                                                functions[i as usize]
                                                    .set_function
                                                    .MYBOOL_set_function
                                                    .expect("non-null function pointer")(
                                                    lp,
                                                    intvalue as ::core::ffi::c_uchar,
                                                );
                                            }
                                            REALfunction => {
                                                functions[i as usize]
                                                    .set_function
                                                    .REAL_set_function
                                                    .expect("non-null function pointer")(
                                                    lp, REALvalue,
                                                );
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if !(header as *mut ::core::ffi::c_void).is_null() {
            free(header as *mut ::core::ffi::c_void);
            header = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        free_hash_table(hashfunctions);
        free_hash_table(hashparameters);
        ini_close(fp);
    }
    return ret as ::core::ffi::c_uchar;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
unsafe extern "C" fn run_static_initializers() {
    functions = [
        _functions {
            par: b"ANTI_DEGEN\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_anti_degen as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_anti_degen as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut anti_degen as *mut _values,
            elements: (::core::mem::size_of::<[_values; 11]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"BASISCRASH\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_basiscrash as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_basiscrash as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut basiscrash as *mut _values,
            elements: (::core::mem::size_of::<[_values; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"IMPROVE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_improve as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_improve as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut improve as *mut _values,
            elements: (::core::mem::size_of::<[_values; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"MAXPIVOT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_maxpivot as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_maxpivot as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"NEGRANGE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_negrange as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_negrange as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"PIVOTING\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_pivoting as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_pivoting as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut pivoting as *mut _values,
            elements: (::core::mem::size_of::<[_values; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: 3 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"PRESOLVE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_presolve as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_presolve1 as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut presolving as *mut _values,
            elements: (::core::mem::size_of::<[_values; 22]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"PRESOLVELOOPS\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_presolveloops as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_presolve2 as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"SCALELIMIT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_scalelimit as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_scalelimit as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"SCALING\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_scaling as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_scaling as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut scaling as *mut _values,
            elements: (::core::mem::size_of::<[_values; 15]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: 7 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"SIMPLEXTYPE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_simplextype as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_simplextype as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut simplextype as *mut _values,
            elements: (::core::mem::size_of::<[_values; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"OBJ_IN_BASIS\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar>,
                    Option<fn_int_get_function>,
                >(Some(
                    is_obj_in_basis as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_obj_in_basis
                        as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> (),
                )),
            },
            type_0: MYBOOLfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"BB_DEPTHLIMIT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_bb_depthlimit as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_bb_depthlimit as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"BB_FLOORFIRST\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_bb_floorfirst as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_bb_floorfirst as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut bb_floorfirst as *mut _values,
            elements: (::core::mem::size_of::<[_values; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"BB_RULE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_bb_rule as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_bb_rule as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut bb_rule as *mut _values,
            elements: (::core::mem::size_of::<[_values; 21]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"BREAK_AT_FIRST\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar>,
                    Option<fn_int_get_function>,
                >(Some(
                    is_break_at_first as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_break_at_first
                        as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> (),
                )),
            },
            type_0: MYBOOLfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"BREAK_AT_VALUE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_break_at_value as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_break_at_value
                        as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"MIP_GAP_ABS\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_mip_gap_abs as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_mip_gap_abs
                        as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"MIP_GAP_REL\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_mip_gap_rel as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_mip_gap_rel
                        as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"EPSINT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_epsint as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_epsint as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"EPSB\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_epsb as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_epsb as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"EPSD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_epsd as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_epsd as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"EPSEL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_epsel as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_epsel as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"EPSPERTURB\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_epsperturb as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_epsperturb as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"EPSPIVOT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_epspivot as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_epspivot as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"INFINITE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_infinite as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_infinite as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_ACTIVE,
        },
        _functions {
            par: b"DEBUG\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar>,
                    Option<fn_int_get_function>,
                >(Some(
                    is_debug as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_debug as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> (),
                )),
            },
            type_0: MYBOOLfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"OBJ_BOUND\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_obj_bound as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_obj_bound as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> (),
                )),
            },
            type_0: REALfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"PRINT_SOL\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_print_sol as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_print_sol as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut print_sol as *mut _values,
            elements: (::core::mem::size_of::<[_values; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"TIMEOUT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_long>,
                    Option<fn_int_get_function>,
                >(Some(
                    get_timeout as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_long,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_long) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_timeout as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_long) -> (),
                )),
            },
            type_0: longfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"TRACE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar>,
                    Option<fn_int_get_function>,
                >(Some(
                    is_trace as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar,
                )),
            },
            set_function: C2RustUnnamed {
                int_set_function: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ()>,
                    Option<fn_int_set_function>,
                >(Some(
                    set_trace as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> (),
                )),
            },
            type_0: MYBOOLfunction,
            values: ::core::ptr::null_mut::<_values>(),
            elements: 0 as ::core::ffi::c_int,
            basemask: 0 as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
        _functions {
            par: b"VERBOSE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            get_function: C2RustUnnamed_0 {
                int_get_function: Some(
                    get_verbose as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
                ),
            },
            set_function: C2RustUnnamed {
                int_set_function: Some(
                    set_verbose as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> (),
                ),
            },
            type_0: intfunction,
            values: &raw mut verbose as *mut _values,
            elements: (::core::mem::size_of::<[_values; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<_values>() as usize)
                as ::core::ffi::c_int,
            basemask: !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            mask: WRITE_COMMENTED,
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
