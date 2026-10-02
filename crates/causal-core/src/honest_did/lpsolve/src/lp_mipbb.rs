use crate::honest_did::lpsolve::runtime::{modf};
use crate::honest_did::lpsolve::runtime::{calloc,free,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_unscaled_value"]
    fn unscaled_value(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_spx_run"]
    fn spx_run(lp: *mut lprec, validInvB: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    #[link_name="honest_lpsolve_mat_getitem"]
    fn mat_getitem(
        mat: *mut MATrec,
        row: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_createUndoLadder"]
    fn createUndoLadder(
        lp: *mut lprec,
        levelitems: ::core::ffi::c_int,
        maxlevels: ::core::ffi::c_int,
    ) -> *mut DeltaVrec;
    #[link_name="honest_lpsolve_incrementUndoLadder"]
    fn incrementUndoLadder(DV: *mut DeltaVrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_modifyUndoLadder"]
    fn modifyUndoLadder(
        DV: *mut DeltaVrec,
        itemno: ::core::ffi::c_int,
        target: *mut ::core::ffi::c_double,
        newvalue: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_countsUndoLadder"]
    fn countsUndoLadder(DV: *mut DeltaVrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_restoreUndoLadder"]
    fn restoreUndoLadder(
        DV: *mut DeltaVrec,
        target: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_decrementUndoLadder"]
    fn decrementUndoLadder(DV: *mut DeltaVrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_freeUndoLadder"]
    fn freeUndoLadder(DV: *mut *mut DeltaVrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_get_candidates"]
    fn SOS_get_candidates(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
        excludetarget: ::core::ffi::c_uchar,
        upbound: *mut ::core::ffi::c_double,
        lobound: *mut ::core::ffi::c_double,
    ) -> *mut ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_is_member"]
    fn SOS_is_member(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_is_member_of_type"]
    fn SOS_is_member_of_type(
        group: *mut SOSgroup,
        column: ::core::ffi::c_int,
        sostype: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_can_activate"]
    fn SOS_can_activate(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_set_marked"]
    fn SOS_set_marked(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
        asactive: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_unmark"]
    fn SOS_unmark(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_fix_unmarked"]
    fn SOS_fix_unmarked(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        variable: ::core::ffi::c_int,
        bound: *mut ::core::ffi::c_double,
        value: ::core::ffi::c_double,
        isupper: ::core::ffi::c_uchar,
        diffcount: *mut ::core::ffi::c_int,
        changelog: *mut DeltaVrec,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_fix_list"]
    fn SOS_fix_list(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        variable: ::core::ffi::c_int,
        bound: *mut ::core::ffi::c_double,
        varlist: *mut ::core::ffi::c_int,
        isleft: ::core::ffi::c_uchar,
        changelog: *mut DeltaVrec,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_is_feasible"]
    fn SOS_is_feasible(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        solution: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_maxim"]
    fn is_maxim(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_int"]
    fn is_int(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_print_objective"]
    fn print_objective(lp: *mut lprec);
    #[link_name="honest_lpsolve_print_solution"]
    fn print_solution(lp: *mut lprec, columns: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_is_anti_degen"]
    fn is_anti_degen(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_presolve"]
    fn is_presolve(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_bb_rule"]
    fn get_bb_rule(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_get_var_branch"]
    fn get_var_branch(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_infinite"]
    fn is_infinite(lp: *mut lprec, value: ::core::ffi::c_double) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_userabort"]
    fn userabort(lp: *mut lprec, message: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_MIP_count"]
    fn MIP_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_count"]
    fn SOS_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_set_action"]
    fn set_action(actionvar: *mut ::core::ffi::c_int, actionmask: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_is_bb_mode"]
    fn is_bb_mode(lp: *mut lprec, bb_mask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_var_priority"]
    fn set_var_priority(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_find_sc_bbvar"]
    fn find_sc_bbvar(lp: *mut lprec, count: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_find_sos_bbvar"]
    fn find_sos_bbvar(
        lp: *mut lprec,
        count: *mut ::core::ffi::c_int,
        intsos: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_find_int_bbvar"]
    fn find_int_bbvar(
        lp: *mut lprec,
        count: *mut ::core::ffi::c_int,
        BB: *mut BBrec,
        isfeasible: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_solution_is_int"]
    fn solution_is_int(
        lp: *mut lprec,
        index: ::core::ffi::c_int,
        checkfixed: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bb_better"]
    fn bb_better(
        lp: *mut lprec,
        target: ::core::ffi::c_int,
        mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_construct_solution"]
    fn construct_solution(lp: *mut lprec, target: *mut ::core::ffi::c_double);
    #[link_name="honest_lpsolve_construct_duals"]
    fn construct_duals(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_construct_sensitivity_duals"]
    fn construct_sensitivity_duals(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_construct_sensitivity_obj"]
    fn construct_sensitivity_obj(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_push_basis"]
    fn push_basis(
        lp: *mut lprec,
        basisvar: *mut ::core::ffi::c_int,
        isbasic: *mut ::core::ffi::c_uchar,
        islower: *mut ::core::ffi::c_uchar,
    ) -> *mut basisrec;
    #[link_name="honest_lpsolve_restore_basis"]
    fn restore_basis(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_pop_basis"]
    fn pop_basis(lp: *mut lprec, restore: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_perturb_bounds"]
    fn perturb_bounds(
        lp: *mut lprec,
        perturbed: *mut BBrec,
        doRows: ::core::ffi::c_uchar,
        doCols: ::core::ffi::c_uchar,
        includeFIXED: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_impose_bounds"]
    fn impose_bounds(
        lp: *mut lprec,
        upbo: *mut ::core::ffi::c_double,
        lowbo: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_transfer_solution"]
    fn transfer_solution(lp: *mut lprec, dofinal: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_scaled_floor"]
    fn scaled_floor(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
        epsscale: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_scaled_ceil"]
    fn scaled_ceil(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
        epsscale: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_init_pseudocost"]
    fn init_pseudocost(lp: *mut lprec, pseudotype: ::core::ffi::c_int) -> *mut BBPSrec;
    #[link_name="honest_lpsolve_free_pseudocost"]
    fn free_pseudocost(lp: *mut lprec);
    #[link_name="honest_lpsolve_get_pseudorange"]
    fn get_pseudorange(
        pc: *mut BBPSrec,
        mipvar: ::core::ffi::c_int,
        varcode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_update_pseudocost"]
    fn update_pseudocost(
        pc: *mut BBPSrec,
        mipvar: ::core::ffi::c_int,
        varcode: ::core::ffi::c_int,
        capupper: ::core::ffi::c_uchar,
        varsol: ::core::ffi::c_double,
    );
    #[link_name="honest_lpsolve_get_pseudobranchcost"]
    fn get_pseudobranchcost(
        pc: *mut BBPSrec,
        mipvar: ::core::ffi::c_int,
        dofloor: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_pseudonodecost"]
    fn get_pseudonodecost(
        pc: *mut BBPSrec,
        mipvar: ::core::ffi::c_int,
        vartype: ::core::ffi::c_int,
        varsol: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
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
#[export_name="honest_lpsolve_create_BB"]
pub unsafe extern "C" fn create_BB(
    mut lp: *mut lprec,
    mut parentBB: *mut BBrec,
    mut dofullcopy: ::core::ffi::c_uchar,
) -> *mut BBrec {
    let mut newBB: *mut BBrec = ::core::ptr::null_mut::<BBrec>();
    newBB = calloc(1 as size_t, ::core::mem::size_of::<BBrec>() as size_t) as *mut BBrec;
    if !newBB.is_null() {
        if parentBB.is_null() {
            allocREAL(
                lp,
                &raw mut (*newBB).upbo,
                (*lp).sum + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            allocREAL(
                lp,
                &raw mut (*newBB).lowbo,
                (*lp).sum + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            memcpy(
                (*newBB).upbo as *mut ::core::ffi::c_void,
                (*lp).orig_upbo as *const ::core::ffi::c_void,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            memcpy(
                (*newBB).lowbo as *mut ::core::ffi::c_void,
                (*lp).orig_lowbo as *const ::core::ffi::c_void,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        } else if dofullcopy != 0 {
            allocREAL(
                lp,
                &raw mut (*newBB).upbo,
                (*lp).sum + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            allocREAL(
                lp,
                &raw mut (*newBB).lowbo,
                (*lp).sum + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            memcpy(
                (*newBB).upbo as *mut ::core::ffi::c_void,
                (*parentBB).upbo as *const ::core::ffi::c_void,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            memcpy(
                (*newBB).lowbo as *mut ::core::ffi::c_void,
                (*parentBB).lowbo as *const ::core::ffi::c_void,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        } else {
            (*newBB).upbo = (*parentBB).upbo;
            (*newBB).lowbo = (*parentBB).lowbo;
        }
        (*newBB).contentmode = dofullcopy;
        (*newBB).lp = lp;
        (*newBB).parent = parentBB as *mut _BBrec;
    }
    return newBB;
}
#[export_name="honest_lpsolve_push_BB"]
pub unsafe extern "C" fn push_BB(
    mut lp: *mut lprec,
    mut parentBB: *mut BBrec,
    mut varno: ::core::ffi::c_int,
    mut vartype: ::core::ffi::c_int,
    mut varcus: ::core::ffi::c_int,
) -> *mut BBrec {
    let mut newBB: *mut BBrec = ::core::ptr::null_mut::<BBrec>();
    if parentBB.is_null() {
        parentBB = (*lp).bb_bounds;
    }
    newBB = create_BB(lp, parentBB, FALSE as ::core::ffi::c_uchar);
    if !newBB.is_null() {
        (*newBB).varno = varno;
        (*newBB).vartype = vartype;
        (*newBB).lastvarcus = varcus;
        incrementUndoLadder((*lp).bb_lowerchange);
        (*newBB).LBtrack += 1;
        incrementUndoLadder((*lp).bb_upperchange);
        (*newBB).UBtrack += 1;
        if !parentBB.is_null() && (*parentBB).lastrcf > 0 as ::core::ffi::c_int {
            let mut isINT: ::core::ffi::c_uchar = 0;
            let mut k: ::core::ffi::c_int = 0;
            let mut ii: ::core::ffi::c_int = 0;
            let mut nfixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut ntighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut deltaUL: ::core::ffi::c_double = 0.;
            let mut current_block_25: u64;
            k = 1 as ::core::ffi::c_int;
            while k <= *(*lp).nzdrow.offset(0 as ::core::ffi::c_int as isize) {
                ii = *(*lp).nzdrow.offset(k as isize);
                if !(ii <= (*lp).rows) {
                    isINT = is_int(lp, ii - (*lp).rows);
                    match abs(rcfbound_BB(
                        newBB,
                        ii,
                        isINT,
                        &raw mut deltaUL,
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                    )) {
                        LE => {
                            current_block_25 = 8232355699563859837;
                            match current_block_25 {
                                10673023933983460535 => {
                                    if deltaUL < *(*newBB).lowbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).lowbo.offset(ii as isize);
                                    }
                                    if deltaUL > *(*newBB).upbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).upbo.offset(ii as isize);
                                    }
                                    modifyUndoLadder(
                                        (*lp).bb_lowerchange,
                                        ii,
                                        (*newBB).lowbo as *mut ::core::ffi::c_double,
                                        deltaUL,
                                    );
                                }
                                _ => {
                                    if deltaUL > *(*newBB).upbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).upbo.offset(ii as isize);
                                    }
                                    if deltaUL < *(*newBB).lowbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).lowbo.offset(ii as isize);
                                    }
                                    modifyUndoLadder(
                                        (*lp).bb_upperchange,
                                        ii,
                                        (*newBB).upbo as *mut ::core::ffi::c_double,
                                        deltaUL,
                                    );
                                }
                            }
                            if *(*newBB).upbo.offset(ii as isize)
                                == *(*newBB).lowbo.offset(ii as isize)
                            {
                                nfixed += 1;
                            } else {
                                ntighten += 1;
                            }
                        }
                        GE => {
                            current_block_25 = 10673023933983460535;
                            match current_block_25 {
                                10673023933983460535 => {
                                    if deltaUL < *(*newBB).lowbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).lowbo.offset(ii as isize);
                                    }
                                    if deltaUL > *(*newBB).upbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).upbo.offset(ii as isize);
                                    }
                                    modifyUndoLadder(
                                        (*lp).bb_lowerchange,
                                        ii,
                                        (*newBB).lowbo as *mut ::core::ffi::c_double,
                                        deltaUL,
                                    );
                                }
                                _ => {
                                    if deltaUL > *(*newBB).upbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).upbo.offset(ii as isize);
                                    }
                                    if deltaUL < *(*newBB).lowbo.offset(ii as isize) {
                                        deltaUL = *(*newBB).lowbo.offset(ii as isize);
                                    }
                                    modifyUndoLadder(
                                        (*lp).bb_upperchange,
                                        ii,
                                        (*newBB).upbo as *mut ::core::ffi::c_double,
                                        deltaUL,
                                    );
                                }
                            }
                            if *(*newBB).upbo.offset(ii as isize)
                                == *(*newBB).lowbo.offset(ii as isize)
                            {
                                nfixed += 1;
                            } else {
                                ntighten += 1;
                            }
                        }
                        _ => {}
                    }
                }
                k += 1;
            }
            if (*lp).bb_trace != 0 {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"push_BB: Used reduced cost to fix %d variables and tighten %d bounds\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        if parentBB == (*lp).bb_bounds {
            (*lp).bb_bounds = newBB;
        } else {
            (*newBB).child = (*parentBB).child;
        }
        if !parentBB.is_null() {
            (*parentBB).child = newBB as *mut _BBrec;
        }
        (*lp).bb_level += 1;
        if (*lp).bb_level > (*lp).bb_maxlevel {
            (*lp).bb_maxlevel = (*lp).bb_level;
        }
        if initbranches_BB(newBB) == 0 {
            newBB = pop_BB(newBB);
        } else if MIP_count(lp) > 0 as ::core::ffi::c_int {
            if (*lp).bb_level <= 1 as ::core::ffi::c_int
                && (*lp).bb_varactive.is_null()
                && (allocINT(
                    lp,
                    &raw mut (*lp).bb_varactive,
                    (*lp).columns + 1 as ::core::ffi::c_int,
                    TRUE as ::core::ffi::c_uchar,
                ) == 0
                    || initcuts_BB(lp) == 0)
            {
                newBB = pop_BB(newBB);
            }
            if varno > 0 as ::core::ffi::c_int {
                let ref mut fresh0 = *(*lp).bb_varactive.offset((varno - (*lp).rows) as isize);
                *fresh0 += 1;
            }
        }
    }
    return newBB;
}
#[export_name="honest_lpsolve_free_BB"]
pub unsafe extern "C" fn free_BB(mut BB: *mut *mut BBrec) -> ::core::ffi::c_uchar {
    let mut parentreturned: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if !BB.is_null() && !(*BB).is_null() {
        let mut parent: *mut BBrec = (**BB).parent as *mut BBrec;
        if parent.is_null() || (**BB).contentmode as ::core::ffi::c_int != 0 {
            if !((**BB).upbo as *mut ::core::ffi::c_void).is_null() {
                free((**BB).upbo as *mut ::core::ffi::c_void);
                (**BB).upbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !((**BB).lowbo as *mut ::core::ffi::c_void).is_null() {
                free((**BB).lowbo as *mut ::core::ffi::c_void);
                (**BB).lowbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
        }
        if !((**BB).varmanaged as *mut ::core::ffi::c_void).is_null() {
            free((**BB).varmanaged as *mut ::core::ffi::c_void);
            (**BB).varmanaged = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if !(*BB as *mut ::core::ffi::c_void).is_null() {
            free(*BB as *mut ::core::ffi::c_void);
            *BB = ::core::ptr::null_mut::<BBrec>();
        }
        parentreturned =
            (parent != NULL as *mut BBrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if parentreturned != 0 {
            *BB = parent;
        }
    }
    return parentreturned;
}
#[export_name="honest_lpsolve_pop_BB"]
pub unsafe extern "C" fn pop_BB(mut BB: *mut BBrec) -> *mut BBrec {
    let mut k: ::core::ffi::c_int = 0;
    let mut parentBB: *mut BBrec = ::core::ptr::null_mut::<BBrec>();
    let mut lp: *mut lprec = (*BB).lp;
    if BB.is_null() {
        return BB;
    }
    parentBB = (*BB).parent as *mut BBrec;
    if BB == (*lp).bb_bounds {
        (*lp).bb_bounds = parentBB;
        if !parentBB.is_null() {
            (*parentBB).child = ::core::ptr::null_mut::<_BBrec>();
        }
    } else {
        if !parentBB.is_null() {
            (*parentBB).child = (*BB).child;
        }
        if !(*BB).child.is_null() {
            (*(*BB).child).parent = parentBB as *mut _BBrec;
        }
    }
    if !(*lp).bb_upperchange.is_null() {
        restoreUndoLadder(
            (*lp).bb_upperchange,
            (*BB).upbo as *mut ::core::ffi::c_double,
        );
        while (*BB).UBtrack > 0 as ::core::ffi::c_int {
            decrementUndoLadder((*lp).bb_upperchange);
            restoreUndoLadder(
                (*lp).bb_upperchange,
                (*BB).upbo as *mut ::core::ffi::c_double,
            );
            (*BB).UBtrack -= 1;
        }
    }
    if !(*lp).bb_lowerchange.is_null() {
        restoreUndoLadder(
            (*lp).bb_lowerchange,
            (*BB).lowbo as *mut ::core::ffi::c_double,
        );
        while (*BB).LBtrack > 0 as ::core::ffi::c_int {
            decrementUndoLadder((*lp).bb_lowerchange);
            restoreUndoLadder(
                (*lp).bb_lowerchange,
                (*BB).lowbo as *mut ::core::ffi::c_double,
            );
            (*BB).LBtrack -= 1;
        }
    }
    (*lp).bb_level -= 1;
    k = (*BB).varno - (*lp).rows;
    if (*lp).bb_level == 0 as ::core::ffi::c_int {
        if !(*lp).bb_varactive.is_null() {
            if !((*lp).bb_varactive as *mut ::core::ffi::c_void).is_null() {
                free((*lp).bb_varactive as *mut ::core::ffi::c_void);
                (*lp).bb_varactive = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
            freecuts_BB(lp);
        }
        if (*lp).int_vars + (*lp).sc_vars > 0 as ::core::ffi::c_int {
            free_pseudocost(lp);
        }
        pop_basis(lp, FALSE as ::core::ffi::c_uchar);
        (*lp).rootbounds = ::core::ptr::null_mut::<BBrec>();
    } else {
        let ref mut fresh1 = *(*lp).bb_varactive.offset(k as isize);
        *fresh1 -= 1;
    }
    if (*BB).isSOS as ::core::ffi::c_int != 0 && (*BB).vartype != BB_INT {
        SOS_unmark((*lp).SOS, 0 as ::core::ffi::c_int, k);
    } else if (*BB).isGUB != 0 {
        SOS_unmark((*lp).GUB, 0 as ::core::ffi::c_int, k);
    }
    if (*BB).sc_canset != 0 {
        *(*lp).sc_lobound.offset(k as isize) *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    }
    pop_basis(lp, FALSE as ::core::ffi::c_uchar);
    free_BB(&raw mut BB);
    return parentBB;
}
#[export_name="honest_lpsolve_probe_BB"]
pub unsafe extern "C" fn probe_BB(mut BB: *mut BBrec) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut coefOF: ::core::ffi::c_double = 0.;
    let mut sum: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut lp: *mut lprec = (*BB).lp;
    if (*lp).solutioncount == 0 as ::core::ffi::c_int {
        return (*lp).infinite;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if !(is_int(lp, i) == 0) {
            ii = (*lp).rows + i;
            coefOF = *(*lp).obj.offset(i as isize);
            if coefOF < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                if is_infinite(lp, *(*BB).lowbo.offset(ii as isize)) != 0 {
                    return (*lp).infinite;
                }
                sum += coefOF
                    * (*(*lp).solution.offset(ii as isize) - *(*BB).lowbo.offset(ii as isize));
            } else {
                if is_infinite(lp, *(*BB).upbo.offset(ii as isize)) != 0 {
                    return (*lp).infinite;
                }
                sum += coefOF
                    * (*(*BB).upbo.offset(ii as isize) - *(*lp).solution.offset(ii as isize));
            }
        }
        i += 1;
    }
    return sum;
}
#[export_name="honest_lpsolve_presolve_BB"]
pub unsafe extern "C" fn presolve_BB(mut BB: *mut BBrec) -> ::core::ffi::c_double {
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[export_name="honest_lpsolve_initbranches_BB"]
pub unsafe extern "C" fn initbranches_BB(mut BB: *mut BBrec) -> ::core::ffi::c_uchar {
    let mut new_bound: ::core::ffi::c_double = 0.;
    let mut temp: ::core::ffi::c_double = 0.;
    let mut k: ::core::ffi::c_int = 0;
    let mut lp: *mut lprec = (*BB).lp;
    (*BB).nodestatus = NOTRUN;
    (*BB).noderesult = (*lp).infinite;
    push_basis(
        lp,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    if (*BB).vartype == BB_REAL {
        (*BB).nodesleft = 1 as ::core::ffi::c_int;
    } else {
        (*BB).nodesleft = 2 as ::core::ffi::c_int;
        k = (*BB).varno - (*lp).rows;
        (*BB).lastsolution = *(*lp).solution.offset((*BB).varno as isize);
        (*BB).isSOS = ((*BB).vartype == BB_SOS
            || SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, k) != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
        (*BB).isGUB = ((*BB).vartype == BB_INT
            && SOS_can_activate((*lp).GUB, 0 as ::core::ffi::c_int, k) as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if (*BB).isGUB != 0 {
            (*BB).varmanaged = SOS_get_candidates(
                (*lp).GUB,
                -(1 as ::core::ffi::c_int),
                k,
                TRUE as ::core::ffi::c_uchar,
                (*BB).upbo,
                (*BB).lowbo,
            );
            (*BB).nodesleft += 1;
        }
        if (*BB).vartype == BB_SOS {
            if SOS_can_activate((*lp).SOS, 0 as ::core::ffi::c_int, k) == 0 {
                (*BB).nodesleft -= 1;
                (*BB).isfloor = TRUE as ::core::ffi::c_uchar;
            } else {
                (*BB).isfloor = ((*BB).lastsolution
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
        } else if (*lp).bb_usebranch.is_some() {
            (*BB).isfloor = (*lp).bb_usebranch.expect("non-null function pointer")(
                lp,
                (*lp).bb_branchhandle,
                k,
            ) as ::core::ffi::c_uchar;
        } else if get_var_branch(lp, k) == BRANCH_AUTOMATIC {
            new_bound = modf(
                (*BB).lastsolution / get_pseudorange((*lp).bb_PseudoCost, k, (*BB).vartype),
                &raw mut temp,
            );
            if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                == ::core::mem::size_of::<::core::ffi::c_float>() as usize
            {
                __inline_isnanf(new_bound as ::core::ffi::c_float)
            } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                == ::core::mem::size_of::<::core::ffi::c_double>() as usize
            {
                __inline_isnand(new_bound)
            } else {
                __inline_isnanl(crate::honest_did::lpsolve::extended::Extended::new(new_bound))
            } != 0
            {
                new_bound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else if new_bound < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                new_bound += 1.0f64;
            }
            (*BB).isfloor = (new_bound <= 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            if is_bb_mode(lp, NODE_GREEDYMODE) != 0 {
                if is_bb_mode(lp, NODE_PSEUDOCOSTMODE) != 0 {
                    (*BB).sc_bound = get_pseudonodecost(
                        (*lp).bb_PseudoCost,
                        k,
                        (*BB).vartype,
                        (*BB).lastsolution,
                    );
                } else {
                    (*BB).sc_bound = mat_getitem((*lp).matA, 0 as ::core::ffi::c_int, k);
                }
                new_bound -= 0.5f64;
                (*BB).sc_bound *= new_bound;
                (*BB).isfloor = ((*BB).sc_bound > 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as ::core::ffi::c_uchar;
            } else if is_bb_mode(lp, NODE_PSEUDOCOSTMODE) != 0 {
                (*BB).isfloor =
                    (get_pseudobranchcost((*lp).bb_PseudoCost, k, TRUE as ::core::ffi::c_uchar)
                        > get_pseudobranchcost(
                            (*lp).bb_PseudoCost,
                            k,
                            FALSE as ::core::ffi::c_uchar,
                        )) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                if is_maxim(lp) != 0 {
                    (*BB).isfloor =
                        ((*BB).isfloor == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                }
            }
            if is_bb_mode(lp, NODE_BRANCHREVERSEMODE) != 0 {
                (*BB).isfloor = ((*BB).isfloor == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
        } else {
            (*BB).isfloor = (get_var_branch(lp, k) == BRANCH_FLOOR) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
        }
        new_bound = fabs(*(*lp).sc_lobound.offset(k as isize));
        (*BB).sc_bound = new_bound;
        (*BB).sc_canset = (new_bound != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
        new_bound = unscaled_value(lp, new_bound, (*BB).varno);
        if is_int(lp, k) as ::core::ffi::c_int != 0
            && (new_bound > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*BB).lastsolution > floor(new_bound))
        {
            if (*BB).lastsolution < ceil(new_bound) {
                (*BB).lastsolution += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            modifyUndoLadder(
                (*lp).bb_lowerchange,
                (*BB).varno,
                (*BB).lowbo as *mut ::core::ffi::c_double,
                scaled_floor(
                    lp,
                    (*BB).varno,
                    (*BB).lastsolution,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                ),
            );
        }
    }
    return fillbranches_BB(BB);
}
#[export_name="honest_lpsolve_fillbranches_BB"]
pub unsafe extern "C" fn fillbranches_BB(mut BB: *mut BBrec) -> ::core::ffi::c_uchar {
    let mut current_block: u64;
    let mut K: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ult_upbo: ::core::ffi::c_double = 0.;
    let mut ult_lowbo: ::core::ffi::c_double = 0.;
    let mut new_bound: ::core::ffi::c_double = 0.;
    let mut SC_bound: ::core::ffi::c_double = 0.;
    let mut intmargin: ::core::ffi::c_double = (*(*BB).lp).epsprimal;
    let mut lp: *mut lprec = (*BB).lp;
    let mut OKstatus: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if (*lp).bb_break as ::core::ffi::c_int != 0
        || userabort(lp, MSG_MILPSTRATEGY) as ::core::ffi::c_int != 0
    {
        return OKstatus;
    }
    K = (*BB).varno;
    if K > 0 as ::core::ffi::c_int {
        k = (*BB).varno - (*lp).rows;
        ult_upbo = *(*lp).orig_upbo.offset(K as isize);
        ult_lowbo = *(*lp).orig_lowbo.offset(K as isize);
        SC_bound = unscaled_value(lp, (*BB).sc_bound, K);
        (*BB).UPbound = (*lp).infinite;
        if SC_bound > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && fabs((*BB).lastsolution) < SC_bound - intmargin
        {
            new_bound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            current_block = 2370887241019905314;
        } else if (*BB).vartype == BB_INT {
            if ult_lowbo >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && floor((*BB).lastsolution)
                    < unscaled_value(
                        lp,
                        (if ult_lowbo > fabs(*(*lp).sc_lobound.offset(k as isize)) {
                            ult_lowbo
                        } else {
                            fabs(*(*lp).sc_lobound.offset(k as isize))
                        }),
                        K,
                    ) - intmargin
                || ult_upbo <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && floor((*BB).lastsolution)
                        > unscaled_value(
                            lp,
                            (if ult_upbo < -fabs(*(*lp).sc_lobound.offset(k as isize)) {
                                ult_upbo
                            } else {
                                -fabs(*(*lp).sc_lobound.offset(k as isize))
                            }),
                            K,
                        ) - intmargin
            {
                (*BB).nodesleft -= 1;
                current_block = 18428752719097972994;
            } else {
                new_bound = scaled_floor(
                    lp,
                    K,
                    (*BB).lastsolution,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                );
                current_block = 2370887241019905314;
            }
        } else {
            if (*BB).isSOS != 0 {
                new_bound = ult_lowbo;
                if is_int(lp, k) != 0 {
                    new_bound = scaled_ceil(
                        lp,
                        K,
                        unscaled_value(lp, new_bound, K),
                        -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                    );
                }
            } else {
                new_bound = (*BB).sc_bound;
            }
            current_block = 2370887241019905314;
        }
        match current_block {
            2370887241019905314 => {
                if new_bound < *(*BB).lowbo.offset(K as isize) {
                    new_bound = *(*BB).lowbo.offset(K as isize)
                        - (if fabs(new_bound - *(*BB).lowbo.offset(K as isize)) < intmargin {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
                        } else {
                            new_bound - *(*BB).lowbo.offset(K as isize)
                        });
                }
                if new_bound < *(*BB).lowbo.offset(K as isize) {
                    (*BB).nodesleft -= 1;
                } else {
                    if fabs(new_bound - *(*BB).lowbo.offset(K as isize))
                        < intmargin * SCALEDINTFIXRANGE
                    {
                        new_bound = *(*BB).lowbo.offset(K as isize);
                    }
                    (*BB).UPbound = new_bound;
                }
            }
            _ => {}
        }
        (*BB).LObound = -(*lp).infinite;
        if SC_bound > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && fabs((*BB).lastsolution) < SC_bound
        {
            if is_int(lp, k) != 0 {
                new_bound = scaled_ceil(
                    lp,
                    K,
                    SC_bound,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                );
            } else {
                new_bound = (*BB).sc_bound;
            }
            current_block = 9441801433784995173;
        } else if (*BB).vartype == BB_INT {
            if ceil((*BB).lastsolution) == (*BB).lastsolution
                || ceil((*BB).lastsolution) > unscaled_value(lp, ult_upbo, K) + intmargin
                || (*BB).isSOS as ::core::ffi::c_int != 0
                    && (*BB).lastsolution == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                (*BB).nodesleft -= 1;
                current_block = 10553789205651965589;
            } else {
                new_bound = scaled_ceil(
                    lp,
                    K,
                    (*BB).lastsolution,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                );
                current_block = 9441801433784995173;
            }
        } else {
            if (*BB).isSOS != 0 {
                if SOS_is_member_of_type((*lp).SOS, k, SOS3) != 0 {
                    new_bound = scaled_floor(
                        lp,
                        K,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
                } else {
                    new_bound = ult_lowbo;
                    if is_int(lp, k) != 0 {
                        new_bound = scaled_floor(
                            lp,
                            K,
                            unscaled_value(lp, new_bound, K),
                            1 as ::core::ffi::c_int as ::core::ffi::c_double,
                        );
                    }
                    if (*(*lp).SOS).maxorder > 2 as ::core::ffi::c_int
                        && (*BB).lastsolution == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && SOS_is_member_of_type((*lp).SOS, k, SOSn) as ::core::ffi::c_int != 0
                    {
                        (*BB).isSOS = AUTOMATIC as ::core::ffi::c_uchar;
                    }
                }
            } else {
                new_bound = (*BB).sc_bound;
            }
            current_block = 9441801433784995173;
        }
        match current_block {
            9441801433784995173 => {
                if new_bound > *(*BB).upbo.offset(K as isize) {
                    new_bound = *(*BB).upbo.offset(K as isize)
                        + (if fabs(new_bound - *(*BB).upbo.offset(K as isize)) < intmargin {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
                        } else {
                            new_bound - *(*BB).upbo.offset(K as isize)
                        });
                }
                if new_bound > *(*BB).upbo.offset(K as isize) {
                    (*BB).nodesleft -= 1;
                } else {
                    if fabs(*(*BB).upbo.offset(K as isize) - new_bound)
                        < intmargin * SCALEDINTFIXRANGE
                    {
                        new_bound = *(*BB).upbo.offset(K as isize);
                    }
                    (*BB).LObound = new_bound;
                }
            }
            _ => {}
        }
        if (*BB).nodesleft > 0 as ::core::ffi::c_int {
            if countsUndoLadder((*lp).bb_upperchange) > 0 as ::core::ffi::c_int {
                incrementUndoLadder((*lp).bb_upperchange);
                (*BB).UBtrack += 1;
            }
            if countsUndoLadder((*lp).bb_lowerchange) > 0 as ::core::ffi::c_int {
                incrementUndoLadder((*lp).bb_lowerchange);
                (*BB).LBtrack += 1;
            }
            if (*BB).vartype != BB_SOS && fabs((*BB).LObound - (*BB).UPbound) < intmargin {
                (*BB).nodesleft -= 1;
                if fabs(*(*BB).lowbo.offset(K as isize) - (*BB).LObound) < intmargin {
                    (*BB).isfloor = FALSE as ::core::ffi::c_uchar;
                } else if fabs(*(*BB).upbo.offset(K as isize) - (*BB).UPbound) < intmargin {
                    (*BB).isfloor = TRUE as ::core::ffi::c_uchar;
                } else {
                    report(
                        (*BB).lp,
                        3 as ::core::ffi::c_int,
                        b"fillbranches_BB: Inconsistent equal-valued bounds for %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            if (*BB).nodesleft == 1 as ::core::ffi::c_int
                && ((*BB).isfloor as ::core::ffi::c_int != 0 && (*BB).UPbound >= (*lp).infinite
                    || (*BB).isfloor == 0 && (*BB).LObound <= -(*lp).infinite)
            {
                (*BB).isfloor = ((*BB).isfloor == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            (*BB).isfloor = ((*BB).isfloor == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            while OKstatus == 0
                && (*lp).spx_status != TIMEOUT
                && (*lp).bb_break == 0
                && (*BB).nodesleft > 0 as ::core::ffi::c_int
            {
                OKstatus = nextbranch_BB(BB);
            }
        }
        if (*BB).sc_canset != 0 {
            *(*lp).sc_lobound.offset(k as isize) *=
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        }
    } else {
        (*BB).nodesleft -= 1;
        OKstatus = TRUE as ::core::ffi::c_uchar;
    }
    return OKstatus;
}
#[export_name="honest_lpsolve_nextbranch_BB"]
pub unsafe extern "C" fn nextbranch_BB(mut BB: *mut BBrec) -> ::core::ffi::c_uchar {
    let mut k: ::core::ffi::c_int = 0;
    let mut lp: *mut lprec = (*BB).lp;
    let mut OKstatus: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if (*BB).nodessolved > 0 as ::core::ffi::c_int {
        restoreUndoLadder(
            (*lp).bb_upperchange,
            (*BB).upbo as *mut ::core::ffi::c_double,
        );
        restoreUndoLadder(
            (*lp).bb_lowerchange,
            (*BB).lowbo as *mut ::core::ffi::c_double,
        );
    }
    if (*lp).bb_break as ::core::ffi::c_int != 0
        || userabort(lp, MSG_MILPSTRATEGY) as ::core::ffi::c_int != 0
    {
        if (*lp).bb_level == 1 as ::core::ffi::c_int
            && (*lp).bb_break as ::core::ffi::c_int == AUTOMATIC
        {
            (*lp).bb_break = FALSE as ::core::ffi::c_uchar;
            OKstatus = TRUE as ::core::ffi::c_uchar;
        }
        return OKstatus;
    }
    if (*BB).nodesleft > 0 as ::core::ffi::c_int {
        k = (*BB).varno - (*lp).rows;
        (*BB).isfloor = ((*BB).isfloor == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        (*BB).nodesleft -= 1;
        if (*BB).isSOS as ::core::ffi::c_int != 0 && (*BB).vartype != BB_INT {
            if (*BB).nodessolved > 0 as ::core::ffi::c_int
                || (*BB).nodessolved == 0 as ::core::ffi::c_int
                    && (*BB).nodesleft == 0 as ::core::ffi::c_int
            {
                if (*BB).isfloor != 0 {
                    if (*BB).nodesleft == 0 as ::core::ffi::c_int
                        && *(*lp).orig_lowbo.offset((*BB).varno as isize)
                            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        return OKstatus;
                    }
                }
                SOS_unmark((*lp).SOS, 0 as ::core::ffi::c_int, k);
            }
            if (*BB).isfloor != 0 {
                SOS_set_marked(
                    (*lp).SOS,
                    0 as ::core::ffi::c_int,
                    k,
                    ((*BB).UPbound != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                        as ::core::ffi::c_int as ::core::ffi::c_uchar,
                );
                (*BB).isSOS as ::core::ffi::c_int == AUTOMATIC;
            } else {
                SOS_set_marked(
                    (*lp).SOS,
                    0 as ::core::ffi::c_int,
                    k,
                    TRUE as ::core::ffi::c_uchar,
                );
                if SOS_fix_unmarked(
                    (*lp).SOS,
                    0 as ::core::ffi::c_int,
                    k,
                    (*BB).upbo,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    TRUE as ::core::ffi::c_uchar,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    (*lp).bb_upperchange,
                ) < 0 as ::core::ffi::c_int
                {
                    return OKstatus;
                }
            }
        } else if (*BB).isGUB != 0 {
            if (*BB).nodessolved > 0 as ::core::ffi::c_int {
                SOS_unmark((*lp).GUB, 0 as ::core::ffi::c_int, k);
            }
            if (*BB).nodesleft == 0 as ::core::ffi::c_int && (*BB).isfloor == 0 {
                (*BB).isfloor = ((*BB).isfloor == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            SOS_set_marked(
                (*lp).GUB,
                0 as ::core::ffi::c_int,
                k,
                ((*BB).isfloor == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
            );
            if (*BB).isfloor != 0 {
                if SOS_fix_list(
                    (*lp).GUB,
                    0 as ::core::ffi::c_int,
                    k,
                    (*BB).upbo,
                    (*BB).varmanaged,
                    ((*BB).nodesleft > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uchar,
                    (*lp).bb_upperchange,
                ) < 0 as ::core::ffi::c_int
                {
                    return OKstatus;
                }
            } else if SOS_fix_unmarked(
                (*lp).GUB,
                0 as ::core::ffi::c_int,
                k,
                (*BB).upbo,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                TRUE as ::core::ffi::c_uchar,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                (*lp).bb_upperchange,
            ) < 0 as ::core::ffi::c_int
            {
                return OKstatus;
            }
        }
        OKstatus = TRUE as ::core::ffi::c_uchar;
    }
    if OKstatus != 0 {
        (*lp).bb_totalnodes += 1;
        (*BB).nodestatus = NOTRUN;
        (*BB).noderesult = (*lp).infinite;
    }
    return OKstatus;
}
#[export_name="honest_lpsolve_initcuts_BB"]
pub unsafe extern "C" fn initcuts_BB(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_updatecuts_BB"]
pub unsafe extern "C" fn updatecuts_BB(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_freecuts_BB"]
pub unsafe extern "C" fn freecuts_BB(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    if !(*lp).bb_cuttype.is_null() {
        if !((*lp).bb_cuttype as *mut ::core::ffi::c_void).is_null() {
            free((*lp).bb_cuttype as *mut ::core::ffi::c_void);
            (*lp).bb_cuttype = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_solve_LP"]
pub unsafe extern "C" fn solve_LP(mut lp: *mut lprec, mut BB: *mut BBrec) -> ::core::ffi::c_int {
    let mut tilted: ::core::ffi::c_int = 0;
    let mut restored: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut testOF: ::core::ffi::c_double = 0.;
    let mut upbo: *mut ::core::ffi::c_double = (*BB).upbo;
    let mut lowbo: *mut ::core::ffi::c_double = (*BB).lowbo;
    let mut perturbed: *mut BBrec = ::core::ptr::null_mut::<BBrec>();
    if (*lp).bb_break != 0 {
        return 11 as ::core::ffi::c_int;
    }
    impose_bounds(lp, upbo, lowbo);
    if (*BB).nodessolved > 1 as ::core::ffi::c_int {
        restore_basis(lp);
    }
    status = RUNNING;
    tilted = 0 as ::core::ffi::c_int;
    restored = 0 as ::core::ffi::c_int;
    while status == RUNNING {
        status = spx_run(
            lp,
            (tilted + restored > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uchar,
        );
        (*lp).bb_status = status;
        (*lp).spx_perturbed = FALSE as ::core::ffi::c_uchar;
        if tilted < 0 as ::core::ffi::c_int {
            break;
        }
        if status == OPTIMAL && tilted > 0 as ::core::ffi::c_int {
            if (*lp).spx_trace != 0 {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"solve_LP: Restoring relaxed bounds at level %d.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            free_BB(&raw mut perturbed);
            if perturbed.is_null() || perturbed == BB {
                perturbed = ::core::ptr::null_mut::<BBrec>();
                impose_bounds(lp, upbo, lowbo);
            } else {
                impose_bounds(lp, (*perturbed).upbo, (*perturbed).lowbo);
            }
            set_action(&raw mut (*lp).spx_action, ACTION_REBASE | ACTION_RECOMPUTE);
            (*BB).UBzerobased = FALSE as ::core::ffi::c_uchar;
            if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong {
                (*lp).real_solution = (*lp).infinite;
            }
            status = RUNNING;
            tilted -= 1;
            restored += 1;
            (*lp).spx_perturbed = TRUE as ::core::ffi::c_uchar;
        } else if ((*lp).bb_level <= 1 as ::core::ffi::c_int
            || is_anti_degen(lp, ANTIDEGEN_DURINGBB) as ::core::ffi::c_int != 0)
            && (status == LOSTFEAS
                && is_anti_degen(lp, ANTIDEGEN_LOSTFEAS) as ::core::ffi::c_int != 0
                || status == INFEASIBLE
                    && is_anti_degen(lp, ANTIDEGEN_INFEASIBLE) as ::core::ffi::c_int != 0
                || status == NUMFAILURE
                    && is_anti_degen(lp, ANTIDEGEN_NUMFAILURE) as ::core::ffi::c_int != 0
                || status == DEGENERATE
                    && is_anti_degen(lp, ANTIDEGEN_STALLING) as ::core::ffi::c_int != 0)
        {
            if tilted <= DEF_MAXRELAX
                && !(tilted == 0 as ::core::ffi::c_int && restored > DEF_MAXRELAX)
            {
                if tilted == 0 as ::core::ffi::c_int {
                    perturbed = BB;
                }
                perturbed = create_BB(lp, perturbed, TRUE as ::core::ffi::c_uchar);
                perturb_bounds(
                    lp,
                    perturbed,
                    TRUE as ::core::ffi::c_uchar,
                    TRUE as ::core::ffi::c_uchar,
                    TRUE as ::core::ffi::c_uchar,
                );
                impose_bounds(lp, (*perturbed).upbo, (*perturbed).lowbo);
                set_action(&raw mut (*lp).spx_action, ACTION_REBASE | ACTION_RECOMPUTE);
                (*BB).UBzerobased = FALSE as ::core::ffi::c_uchar;
                status = RUNNING;
                tilted += 1;
                (*lp).perturb_count += 1;
                (*lp).spx_perturbed = TRUE as ::core::ffi::c_uchar;
                if (*lp).spx_trace != 0 {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"solve_LP: Starting bound relaxation #%d ('%s')\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            } else {
                if (*lp).spx_trace != 0 {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"solve_LP: Relaxation limit exceeded in resolving infeasibility\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                while !perturbed.is_null() && perturbed != BB {
                    free_BB(&raw mut perturbed);
                }
                perturbed = ::core::ptr::null_mut::<BBrec>();
            }
        }
    }
    if status != OPTIMAL {
        if (*lp).bb_level <= 1 as ::core::ffi::c_int {
            (*lp).bb_parentOF = (*lp).infinite;
        }
        if status == USERABORT || status == TIMEOUT {
            if (*lp).solutioncount == 0 as ::core::ffi::c_int
                && MIP_count(lp) == 0 as ::core::ffi::c_int
                && (*lp).simplex_mode & (SIMPLEX_Phase2_PRIMAL | SIMPLEX_Phase2_DUAL)
                    > 0 as ::core::ffi::c_int
            {
                (*lp).solutioncount += 1;
                construct_solution(lp, ::core::ptr::null_mut::<::core::ffi::c_double>());
                transfer_solution(lp, TRUE as ::core::ffi::c_uchar);
            }
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"\nlp_solve optimization was stopped %s.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if (*BB).varno == 0 as ::core::ffi::c_int {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"The model %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else if status == FATHOMED {
            (*lp).spx_status = INFEASIBLE;
        }
    } else {
        construct_solution(lp, ::core::ptr::null_mut::<::core::ffi::c_double>());
        if (*lp).bb_level <= 1 as ::core::ffi::c_int && restored > 0 as ::core::ffi::c_int {
            report(
                lp,
                5 as ::core::ffi::c_int,
                b"%s numerics encountered; validate accuracy\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if (*lp).spx_status != OPTIMAL {
            status = (*lp).spx_status;
        } else if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
            && MIP_count(lp) > 0 as ::core::ffi::c_int
        {
            if (*lp).lag_status != RUNNING {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"\nRelaxed solution  %18.12g after %10.0f iter is B&B base.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            if (*lp).usermessage.is_some() && (*lp).msgmask & MSG_LPOPTIMAL != 0 {
                let mut best_solution: *mut ::core::ffi::c_double = (*lp).best_solution;
                (*lp).best_solution = (*lp).solution;
                (*lp).usermessage.expect("non-null function pointer")(
                    lp,
                    (*lp).msghandle,
                    MSG_LPOPTIMAL,
                );
                (*lp).best_solution = best_solution;
            }
            set_var_priority(lp);
        }
        testOF = if is_maxim(lp) as ::core::ffi::c_int != 0
            && (*(*lp).solution.offset(0 as ::core::ffi::c_int as isize) - (*lp).real_solution)
                / (1.0f64 + fabs((*lp).real_solution))
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -((*(*lp).solution.offset(0 as ::core::ffi::c_int as isize) - (*lp).real_solution)
                / (1.0f64 + fabs((*lp).real_solution)))
        } else {
            (*(*lp).solution.offset(0 as ::core::ffi::c_int as isize) - (*lp).real_solution)
                / (1.0f64 + fabs((*lp).real_solution))
        };
        if testOF < -(*lp).epsprimal {
            report(
                lp,
                5 as ::core::ffi::c_int,
                b"solve_LP: A MIP subproblem returned a value better than the base.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            status = INFEASIBLE;
            (*lp).spx_status = status;
            set_action(
                &raw mut (*lp).spx_action,
                ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
            );
        } else if testOF < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *(*lp).solution.offset(0 as ::core::ffi::c_int as isize) = (*lp).real_solution;
        }
    }
    return status;
}
#[export_name="honest_lpsolve_findself_BB"]
pub unsafe extern "C" fn findself_BB(mut BB: *mut BBrec) -> *mut BBrec {
    let mut varno: ::core::ffi::c_int = (*BB).varno;
    let mut vartype: ::core::ffi::c_int = (*BB).vartype;
    BB = (*BB).parent as *mut BBrec;
    while !BB.is_null() && (*BB).vartype != vartype && (*BB).varno != varno {
        BB = (*BB).parent as *mut BBrec;
    }
    return BB;
}
#[export_name="honest_lpsolve_rcfbound_BB"]
pub unsafe extern "C" fn rcfbound_BB(
    mut BB: *mut BBrec,
    mut varno: ::core::ffi::c_int,
    mut isINT: ::core::ffi::c_uchar,
    mut newbound: *mut ::core::ffi::c_double,
    mut isfeasible: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = FR;
    let mut lp: *mut lprec = (*BB).lp;
    let mut deltaRC: ::core::ffi::c_double = 0.;
    let mut rangeLU: ::core::ffi::c_double = 0.;
    let mut deltaOF: ::core::ffi::c_double = 0.;
    let mut lowbo: ::core::ffi::c_double = 0.;
    let mut upbo: ::core::ffi::c_double = 0.;
    if *(*lp).is_basic.offset(varno as isize) != 0 {
        return i;
    }
    lowbo = *(*BB).lowbo.offset(varno as isize);
    upbo = *(*BB).upbo.offset(varno as isize);
    rangeLU = upbo - lowbo;
    if rangeLU > (*lp).epsprimal {
        deltaOF = *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) - (*lp).bb_workOF;
        deltaRC = if *(*lp).is_lower.offset(varno as isize) == 0
            && *(*lp).drow.offset(varno as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -*(*lp).drow.offset(varno as isize)
        } else {
            *(*lp).drow.offset(varno as isize)
        };
        if deltaRC < (*lp).epspivot {
            return i;
        }
        deltaRC = deltaOF / deltaRC;
        if deltaRC < rangeLU + (*lp).epsint {
            if *(*lp).is_lower.offset(varno as isize) != 0 {
                if isINT != 0 {
                    deltaRC = scaled_floor(
                        lp,
                        varno,
                        unscaled_value(lp, deltaRC, varno) + (*lp).epsprimal,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
                }
                upbo = lowbo + deltaRC;
                deltaRC = upbo;
                i = LE;
            } else {
                if isINT != 0 {
                    deltaRC = scaled_ceil(
                        lp,
                        varno,
                        unscaled_value(lp, deltaRC, varno) + (*lp).epsprimal,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
                }
                lowbo = upbo - deltaRC;
                deltaRC = lowbo;
                i = GE;
            }
            if !isfeasible.is_null() && upbo - lowbo < -(*lp).epsprimal {
                *isfeasible = FALSE as ::core::ffi::c_uchar;
            } else if fabs(upbo - lowbo) < (*lp).epsprimal {
                i = -i;
            }
            if !newbound.is_null() {
                if fabs(deltaRC) < (*lp).epsprimal {
                    deltaRC = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                *newbound = deltaRC;
            }
        }
    }
    return i;
}
#[export_name="honest_lpsolve_findnode_BB"]
pub unsafe extern "C" fn findnode_BB(
    mut BB: *mut BBrec,
    mut varno: *mut ::core::ffi::c_int,
    mut vartype: *mut ::core::ffi::c_int,
    mut varcus: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut countsossc: ::core::ffi::c_int = 0;
    let mut countnint: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut reasonmsg: ::core::ffi::c_int = MSG_NONE;
    let mut varsol: ::core::ffi::c_double = 0.;
    let mut is_better: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut is_equal: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut is_feasible: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut lp: *mut lprec = (*BB).lp;
    *varno = 0 as ::core::ffi::c_int;
    *vartype = BB_REAL;
    *varcus = 0 as ::core::ffi::c_int;
    countnint = 0 as ::core::ffi::c_int;
    (*BB).nodestatus = (*lp).spx_status;
    (*BB).noderesult = *(*lp).solution.offset(0 as ::core::ffi::c_int as isize);
    if (*lp).bb_limitlevel != 1 as ::core::ffi::c_int && MIP_count(lp) > 0 as ::core::ffi::c_int {
        countsossc = (*lp).sos_vars + (*lp).sc_vars;
        if (*lp).bb_limitlevel > 0 as ::core::ffi::c_int
            && (*lp).bb_level > (*lp).bb_limitlevel + countsossc
        {
            return 0 as ::core::ffi::c_uchar;
        } else if (*lp).bb_limitlevel < 0 as ::core::ffi::c_int
            && (*lp).bb_level
                > 2 as ::core::ffi::c_int * ((*lp).int_vars + countsossc) * abs((*lp).bb_limitlevel)
        {
            if (*lp).bb_limitlevel == DEF_BB_LIMITLEVEL {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"findnode_BB: Default B&B limit reached at %d; optionally change strategy or limit.\n\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            return 0 as ::core::ffi::c_uchar;
        }
        if (*BB).varno == 0 as ::core::ffi::c_int {
            varsol = (*lp).infinite;
            if (*lp).int_vars + (*lp).sc_vars > 0 as ::core::ffi::c_int
                && (*lp).bb_PseudoCost.is_null()
            {
                (*lp).bb_PseudoCost = init_pseudocost(lp, get_bb_rule(lp));
            }
        } else {
            varsol = *(*lp).solution.offset((*BB).varno as isize);
            if (*lp).int_vars > 0 as ::core::ffi::c_int && (*BB).vartype == BB_INT
                || (*lp).sc_vars > 0 as ::core::ffi::c_int
                    && (*BB).vartype == BB_SC
                    && is_int(lp, (*BB).varno - (*lp).rows) == 0
            {
                update_pseudocost(
                    (*lp).bb_PseudoCost,
                    (*BB).varno - (*lp).rows,
                    (*BB).vartype,
                    (*BB).isfloor,
                    varsol,
                );
            }
        }
        if (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
            && bb_better(lp, OF_RELAXED, OF_TEST_WE) == 0
        {
            if (*lp).bb_trace != 0 {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"findnode_BB: Simplex failure due to loss of numeric accuracy\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            (*lp).spx_status = NUMFAILURE;
            return 0 as ::core::ffi::c_uchar;
        }
        if (*lp).solutioncount == 0 as ::core::ffi::c_int
            && bb_better(lp, OF_HEURISTIC, OF_TEST_BE) == 0
            || (*lp).solutioncount > 0 as ::core::ffi::c_int
                && (bb_better(lp, OF_INCUMBENT | OF_DELTA, OF_TEST_BE | OF_TEST_RELGAP) == 0
                    || bb_better(lp, OF_INCUMBENT | OF_DELTA, OF_TEST_BE) == 0)
        {
            return 0 as ::core::ffi::c_uchar;
        }
        if (*lp).sc_vars > 0 as ::core::ffi::c_int {
            *varno = find_sc_bbvar(lp, &raw mut countnint);
            if *varno > 0 as ::core::ffi::c_int {
                *vartype = BB_SC;
            }
        }
        if SOS_count(lp) > 0 as ::core::ffi::c_int && *varno == 0 as ::core::ffi::c_int {
            *varno = find_sos_bbvar(lp, &raw mut countnint, FALSE as ::core::ffi::c_uchar);
            if *varno < 0 as ::core::ffi::c_int {
                *varno = 0 as ::core::ffi::c_int;
            } else if *varno > 0 as ::core::ffi::c_int {
                *vartype = BB_SOS;
            }
        }
        if (*lp).int_vars > 0 as ::core::ffi::c_int && *varno == 0 as ::core::ffi::c_int {
            *varno = find_int_bbvar(lp, &raw mut countnint, BB, &raw mut is_feasible);
            if *varno > 0 as ::core::ffi::c_int {
                *vartype = BB_INT;
                if countnint == 1 as ::core::ffi::c_int && is_feasible == 0 {
                    (*BB).lastrcf = 0 as ::core::ffi::c_int;
                    return 0 as ::core::ffi::c_uchar;
                }
            }
        }
        k = *varno - (*lp).rows;
        if *varno > 0 as ::core::ffi::c_int
            && (*lp).bb_limitlevel != 0 as ::core::ffi::c_int
            && *(*lp).bb_varactive.offset(k as isize) >= abs((*lp).bb_limitlevel)
        {
            return 0 as ::core::ffi::c_uchar;
        }
        if *varno == 0 as ::core::ffi::c_int {
            is_better = (((*lp).solutioncount == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uchar as ::core::ffi::c_int
                != 0
                || bb_better(lp, OF_INCUMBENT | OF_DELTA, OF_TEST_BT) as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
            is_better = (is_better as ::core::ffi::c_int
                & bb_better(lp, OF_INCUMBENT | OF_DELTA, OF_TEST_BT | OF_TEST_RELGAP)
                    as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            is_equal = (is_better == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            if is_equal != 0 {
                if (*lp).solutionlimit <= 0 as ::core::ffi::c_int
                    || (*lp).solutioncount < (*lp).solutionlimit
                {
                    (*lp).solutioncount += 1;
                    if (*lp).bb_solutionlevel > (*lp).bb_level {
                        (*lp).bb_solutionlevel = (*lp).bb_level;
                    }
                    reasonmsg = MSG_MILPEQUAL;
                }
            } else if is_better != 0 {
                if !(*lp).bb_varactive.is_null() {
                    let ref mut fresh2 =
                        *(*lp).bb_varactive.offset(0 as ::core::ffi::c_int as isize);
                    *fresh2 += 1;
                    if *(*lp).bb_varactive.offset(0 as ::core::ffi::c_int as isize)
                        == 1 as ::core::ffi::c_int
                        && is_bb_mode(lp, NODE_DEPTHFIRSTMODE) as ::core::ffi::c_int != 0
                        && is_bb_mode(lp, NODE_DYNAMICMODE) as ::core::ffi::c_int != 0
                    {
                        (*lp).bb_rule &= (NODE_DEPTHFIRSTMODE == 0) as ::core::ffi::c_int;
                    }
                }
                if (*lp).bb_trace as ::core::ffi::c_int != 0
                    || (*lp).verbose >= NORMAL
                        && (*lp).print_sol == FALSE
                        && (*lp).lag_status != RUNNING
                {
                    report(
                        lp,
                        3 as ::core::ffi::c_int,
                        b"%s solution %18.12g after %10.0f iter, %9.0f nodes (gap %.1f%%)\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                if MIP_count(lp) > 0 as ::core::ffi::c_int {
                    if (*lp).bb_improvements == 0 as ::core::ffi::c_int {
                        reasonmsg = MSG_MILPFEASIBLE;
                    } else {
                        reasonmsg = MSG_MILPBETTER;
                    }
                }
                (*lp).bb_status = FEASFOUND;
                (*lp).bb_solutionlevel = (*lp).bb_level;
                (*lp).solutioncount = 1 as ::core::ffi::c_int;
                (*lp).bb_improvements += 1;
                (*lp).bb_workOF = *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize);
                if (*lp).bb_breakfirst as ::core::ffi::c_int != 0
                    || is_infinite(lp, (*lp).bb_breakOF) == 0
                        && bb_better(lp, OF_USERBREAK, OF_TEST_BE) as ::core::ffi::c_int != 0
                {
                    (*lp).bb_break = TRUE as ::core::ffi::c_uchar;
                }
            }
        }
    } else {
        is_better = TRUE as ::core::ffi::c_uchar;
        (*lp).solutioncount = 1 as ::core::ffi::c_int;
    }
    if is_better as ::core::ffi::c_int != 0 || is_equal as ::core::ffi::c_int != 0 {
        transfer_solution(
            lp,
            ((*lp).do_presolve & PRESOLVE_LASTMASKMODE != PRESOLVE_NONE) as ::core::ffi::c_int
                as ::core::ffi::c_uchar,
        );
        if MIP_count(lp) > 0 as ::core::ffi::c_int
            && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
        {
            construct_duals(lp) == 0
                || is_presolve(lp, PRESOLVE_SENSDUALS) as ::core::ffi::c_int != 0
                    && (construct_sensitivity_duals(lp) == 0 || construct_sensitivity_obj(lp) == 0);
        }
        if reasonmsg != MSG_NONE && (*lp).msgmask & reasonmsg != 0 && (*lp).usermessage.is_some() {
            (*lp).usermessage.expect("non-null function pointer")(lp, (*lp).msghandle, reasonmsg);
        }
        if (*lp).print_sol != FALSE {
            print_objective(lp);
            print_solution(lp, 1 as ::core::ffi::c_int);
        }
    }
    *varcus = countnint;
    if MIP_count(lp) > 0 as ::core::ffi::c_int {
        if countnint == 0 as ::core::ffi::c_int
            && (*lp).solutioncount == 1 as ::core::ffi::c_int
            && (*lp).solutionlimit == 1 as ::core::ffi::c_int
            && (bb_better(lp, OF_DUALLIMIT, OF_TEST_BE) as ::core::ffi::c_int != 0
                || bb_better(lp, OF_USERBREAK, OF_TEST_BE | OF_TEST_RELGAP) as ::core::ffi::c_int
                    != 0)
        {
            (*lp).bb_break = (countnint == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
            return 0 as ::core::ffi::c_uchar;
        } else if (*lp).bb_level > 0 as ::core::ffi::c_int {
            if (*lp).spx_trace != 0 {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"B&B level %5d OPT %16s value %18.12g\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        return (*varno > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_solve_BB"]
pub unsafe extern "C" fn solve_BB(mut BB: *mut BBrec) -> ::core::ffi::c_int {
    let mut K: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut lp: *mut lprec = (*BB).lp;
    status = PROCFAIL;
    K = (*BB).varno;
    if K > 0 as ::core::ffi::c_int {
        updatecuts_BB(lp);
        if (*BB).isfloor != 0 {
            modifyUndoLadder(
                (*lp).bb_upperchange,
                K,
                (*BB).upbo as *mut ::core::ffi::c_double,
                (*BB).UPbound,
            );
        } else {
            modifyUndoLadder(
                (*lp).bb_lowerchange,
                K,
                (*BB).lowbo as *mut ::core::ffi::c_double,
                (*BB).LObound,
            );
        }
        (*BB).nodessolved += 1;
    }
    status = solve_LP(lp, BB);
    if status == OPTIMAL
        && (*BB).vartype == BB_SOS
        && SOS_is_feasible((*lp).SOS, 0 as ::core::ffi::c_int, (*lp).solution) == 0
    {
        status = INFEASIBLE;
    }
    return status;
}
#[export_name="honest_lpsolve_strongbranch_BB"]
pub unsafe extern "C" fn strongbranch_BB(
    mut lp: *mut lprec,
    mut BB: *mut BBrec,
    mut varno: ::core::ffi::c_int,
    mut vartype: ::core::ffi::c_int,
    mut varcus: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut success: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    let mut strongBB: *mut BBrec = ::core::ptr::null_mut::<BBrec>();
    (*lp).is_strongbranch = TRUE;
    push_basis(lp, (*lp).var_basic, (*lp).is_basic, (*lp).is_lower);
    strongBB = push_BB(lp, BB, (*lp).rows + varno, vartype, varcus);
    if strongBB == BB {
        return success;
    }
    loop {
        (*lp).bb_strongbranches += 1;
        if solve_BB(strongBB) == OPTIMAL {
            success = (success as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << (*strongBB).isfloor as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            (*strongBB).lastvarcus = 0 as ::core::ffi::c_int;
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                if is_int(lp, i) as ::core::ffi::c_int != 0
                    && solution_is_int(lp, (*lp).rows + i, FALSE as ::core::ffi::c_uchar) == 0
                {
                    (*strongBB).lastvarcus += 1;
                }
                i += 1;
            }
            update_pseudocost(
                (*lp).bb_PseudoCost,
                varno,
                (*strongBB).vartype,
                (*strongBB).isfloor,
                *(*lp).solution.offset((*strongBB).varno as isize),
            );
        }
        if !(nextbranch_BB(strongBB) != 0) {
            break;
        }
    }
    strongBB = pop_BB(strongBB);
    if strongBB != BB {
        report(
            lp,
            2 as ::core::ffi::c_int,
            b"strongbranch_BB: Invalid bound settings restored for variable %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    pop_basis(lp, TRUE as ::core::ffi::c_uchar);
    set_action(
        &raw mut (*lp).spx_action,
        ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
    );
    (*lp).is_strongbranch = FALSE;
    return success;
}
#[export_name="honest_lpsolve_pre_BB"]
pub unsafe extern "C" fn pre_BB(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_post_BB"]
pub unsafe extern "C" fn post_BB(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_run_BB"]
pub unsafe extern "C" fn run_BB(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut currentBB: *mut BBrec = ::core::ptr::null_mut::<BBrec>();
    let mut varno: ::core::ffi::c_int = 0;
    let mut vartype: ::core::ffi::c_int = 0;
    let mut varcus: ::core::ffi::c_int = 0;
    let mut prevsolutions: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = NOTRUN;
    pre_BB(lp);
    prevsolutions = (*lp).solutioncount;
    varno = (*lp).columns;
    (*lp).bb_upperchange = createUndoLadder(lp, varno, 2 as ::core::ffi::c_int * MIP_count(lp));
    (*lp).bb_lowerchange = createUndoLadder(lp, varno, 2 as ::core::ffi::c_int * MIP_count(lp));
    currentBB = push_BB(
        lp,
        ::core::ptr::null_mut::<BBrec>(),
        0 as ::core::ffi::c_int,
        BB_REAL,
        0 as ::core::ffi::c_int,
    );
    (*lp).rootbounds = currentBB;
    while (*lp).bb_level > 0 as ::core::ffi::c_int {
        status = solve_BB(currentBB);
        if status == OPTIMAL
            && findnode_BB(currentBB, &raw mut varno, &raw mut vartype, &raw mut varcus)
                as ::core::ffi::c_int
                != 0
        {
            currentBB = push_BB(lp, currentBB, varno, vartype, varcus);
        } else {
            while (*lp).bb_level > 0 as ::core::ffi::c_int && nextbranch_BB(currentBB) == 0 {
                currentBB = pop_BB(currentBB);
            }
        }
    }
    freeUndoLadder(&raw mut (*lp).bb_upperchange);
    freeUndoLadder(&raw mut (*lp).bb_lowerchange);
    if (*lp).solutioncount > prevsolutions {
        if status == PROCBREAK
            || status == USERABORT
            || status == TIMEOUT
            || userabort(lp, -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int != 0
        {
            status = SUBOPTIMAL;
        } else {
            status = OPTIMAL;
        }
        if (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong {
            (*lp).spx_status = OPTIMAL;
        }
    }
    post_BB(lp);
    return status;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn __inline_isnanf(mut __x: ::core::ffi::c_float) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnand(mut __x: ::core::ffi::c_double) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnanl(mut __x: crate::honest_did::lpsolve::extended::Extended) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
pub const MAXINT32: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIMPLEX_Phase2_PRIMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIMPLEX_Phase2_DUAL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PRESOLVE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PRESOLVE_LASTMASKMODE: ::core::ffi::c_int = PRESOLVE_DUALS - 1 as ::core::ffi::c_int;
pub const PRESOLVE_DUALS: ::core::ffi::c_int = 524288 as ::core::ffi::c_int;
pub const PRESOLVE_SENSDUALS: ::core::ffi::c_int = 1048576 as ::core::ffi::c_int;
pub const ANTIDEGEN_STALLING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTIDEGEN_NUMFAILURE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ANTIDEGEN_LOSTFEAS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ANTIDEGEN_INFEASIBLE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const ANTIDEGEN_DURINGBB: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const NORMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MSG_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MSG_LPOPTIMAL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MSG_MILPFEASIBLE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MSG_MILPEQUAL: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MSG_MILPBETTER: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MSG_MILPSTRATEGY: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const ROWTYPE_EMPTY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ROWTYPE_LE: ::core::ffi::c_int = 1;
pub const ROWTYPE_GE: ::core::ffi::c_int = 2;
pub const FR: ::core::ffi::c_int = ROWTYPE_EMPTY;
pub const LE: ::core::ffi::c_int = ROWTYPE_LE;
pub const GE: ::core::ffi::c_int = ROWTYPE_GE;
pub const BB_REAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BB_INT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BB_SC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BB_SOS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NODE_BRANCHREVERSEMODE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NODE_GREEDYMODE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NODE_PSEUDOCOSTMODE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NODE_DEPTHFIRSTMODE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const NODE_DYNAMICMODE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const BRANCH_FLOOR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BRANCH_AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ACTION_REBASE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ACTION_RECOMPUTE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ACTION_REINVERT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NOTRUN: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const OPTIMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SUBOPTIMAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INFEASIBLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DEGENERATE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUMFAILURE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const USERABORT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TIMEOUT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const RUNNING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PROCFAIL: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PROCBREAK: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FEASFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const FATHOMED: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const LOSTFEAS: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const OF_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OF_INCUMBENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OF_USERBREAK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OF_HEURISTIC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OF_DUALLIMIT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OF_DELTA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OF_TEST_BT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OF_TEST_BE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OF_TEST_WE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OF_TEST_RELGAP: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DEF_MAXRELAX: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const DEF_BB_LIMITLEVEL: ::core::ffi::c_int = -(50 as ::core::ffi::c_int);
pub const SCALEDINTFIXRANGE: ::core::ffi::c_double = 1.6f64;
pub const SOS3: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SOSn: ::core::ffi::c_int = MAXINT32;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
