use crate::honest_did::lpsolve::runtime::{strcpy};
use crate::honest_did::lpsolve::runtime::{calloc,free,sqrt,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_presolve"]
    fn presolve(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_postsolve"]
    fn postsolve(lp: *mut lprec, status: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_find_rowReplacement"]
    fn find_rowReplacement(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        prow: *mut ::core::ffi::c_double,
        nzprow: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_colprim"]
    fn colprim(
        lp: *mut lprec,
        drow: *mut ::core::ffi::c_double,
        nzdrow: *mut ::core::ffi::c_int,
        skipupdate: ::core::ffi::c_uchar,
        partialloop: ::core::ffi::c_int,
        candidatecount: *mut ::core::ffi::c_int,
        updateinfeas: ::core::ffi::c_uchar,
        xviol: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_rowprim"]
    fn rowprim(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        theta: *mut ::core::ffi::c_double,
        pcol: *mut ::core::ffi::c_double,
        nzpcol: *mut ::core::ffi::c_int,
        forceoutEQ: ::core::ffi::c_uchar,
        xviol: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_rowdual"]
    fn rowdual(
        lp: *mut lprec,
        rhvec: *mut ::core::ffi::c_double,
        forceoutEQ: ::core::ffi::c_uchar,
        updateinfeas: ::core::ffi::c_uchar,
        xviol: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_coldual"]
    fn coldual(
        lp: *mut lprec,
        row_nr: ::core::ffi::c_int,
        prow: *mut ::core::ffi::c_double,
        nzprow: *mut ::core::ffi::c_int,
        drow: *mut ::core::ffi::c_double,
        nzdrow: *mut ::core::ffi::c_int,
        dualphase1: ::core::ffi::c_uchar,
        skipupdate: ::core::ffi::c_uchar,
        candidatecount: *mut ::core::ffi::c_int,
        xviol: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_partial_countBlocks"]
    fn partial_countBlocks(lp: *mut lprec, isrow: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_partial_blockStep"]
    fn partial_blockStep(lp: *mut lprec, isrow: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_multi_create"]
    fn multi_create(lp: *mut lprec, truncinf: ::core::ffi::c_uchar) -> *mut multirec;
    #[link_name="honest_lpsolve_multi_resize"]
    fn multi_resize(
        multi: *mut multirec,
        blocksize: ::core::ffi::c_int,
        blockdiv: ::core::ffi::c_int,
        doVlist: ::core::ffi::c_uchar,
        doIset: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_multi_indexSet"]
    fn multi_indexSet(
        multi: *mut multirec,
        regenerate: ::core::ffi::c_uchar,
    ) -> *mut ::core::ffi::c_int;
    #[link_name="honest_lpsolve_multi_free"]
    fn multi_free(multi: *mut *mut multirec);
    #[link_name="honest_lpsolve_simplexPricer"]
    fn simplexPricer(lp: *mut lprec, isdual: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_restartPricer"]
    fn restartPricer(lp: *mut lprec, isdual: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_mat_validate"]
    fn mat_validate(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_findelm"]
    fn mat_findelm(
        mat: *mut MATrec,
        row: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_getitem"]
    fn mat_getitem(
        mat: *mut MATrec,
        row: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_mat_multadd"]
    fn mat_multadd(
        mat: *mut MATrec,
        lhsvector: *mut ::core::ffi::c_double,
        varnr: ::core::ffi::c_int,
        mult: ::core::ffi::c_double,
    );
    #[link_name="honest_lpsolve_invert"]
    fn invert(
        lp: *mut lprec,
        shiftbounds: ::core::ffi::c_uchar,
        final_0: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_ftran"]
    fn ftran(
        lp: *mut lprec,
        rhsvector: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
    );
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
    #[link_name="honest_lpsolve_freecuts_BB"]
    fn freecuts_BB(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_run_BB"]
    fn run_BB(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_make_lp"]
    fn make_lp(rows: ::core::ffi::c_int, columns: ::core::ffi::c_int) -> *mut lprec;
    #[link_name="honest_lpsolve_delete_lp"]
    fn delete_lp(lp: *mut lprec);
    #[link_name="honest_lpsolve_set_sense"]
    fn set_sense(lp: *mut lprec, maximize: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_is_maxim"]
    fn is_maxim(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_row"]
    fn get_row(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        row: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_del_constraint"]
    fn del_constraint(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_constr_type"]
    fn get_constr_type(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_get_rh"]
    fn get_rh(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_add_columnex"]
    fn add_columnex(
        lp: *mut lprec,
        count: ::core::ffi::c_int,
        column: *mut ::core::ffi::c_double,
        rowno: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_del_column"]
    fn del_column(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_mat"]
    fn set_mat(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_set_bounds"]
    fn set_bounds(
        lp: *mut lprec,
        colnr: ::core::ffi::c_int,
        lower: ::core::ffi::c_double,
        upper: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_default_basis"]
    fn default_basis(lp: *mut lprec);
    #[link_name="honest_lpsolve_set_basisvar"]
    fn set_basisvar(
        lp: *mut lprec,
        basisPos: ::core::ffi::c_int,
        enteringCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_solve"]
    fn solve(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_print_lp"]
    fn print_lp(lp: *mut lprec);
    #[link_name="honest_lpsolve_print_solution"]
    fn print_solution(lp: *mut lprec, columns: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_is_anti_degen"]
    fn is_anti_degen(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_presolve"]
    fn is_presolve(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_piv_mode"]
    fn is_piv_mode(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_ptr_sensitivity_rhs"]
    fn get_ptr_sensitivity_rhs(
        lp: *mut lprec,
        duals: *mut *mut ::core::ffi::c_double,
        dualsfrom: *mut *mut ::core::ffi::c_double,
        dualstill: *mut *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_Lrows"]
    fn get_Lrows(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_userabort"]
    fn userabort(lp: *mut lprec, message: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_chsign"]
    fn is_chsign(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_inc_lag_space"]
    fn inc_lag_space(
        lp: *mut lprec,
        deltarows: ::core::ffi::c_int,
        ignoreMAT: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bin_count"]
    fn bin_count(lp: *mut lprec, working: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_MIP_count"]
    fn MIP_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_GUB_count"]
    fn GUB_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_refactRecent"]
    fn refactRecent(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_feasiblePhase1"]
    fn feasiblePhase1(lp: *mut lprec, epsvalue: ::core::ffi::c_double) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_free_duals"]
    fn free_duals(lp: *mut lprec);
    #[link_name="honest_lpsolve_recompute_solution"]
    fn recompute_solution(lp: *mut lprec, shiftbounds: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_check_solution"]
    fn check_solution(
        lp: *mut lprec,
        lastcolumn: ::core::ffi::c_int,
        solution: *mut ::core::ffi::c_double,
        upbo: *mut ::core::ffi::c_double,
        lowbo: *mut ::core::ffi::c_double,
        tolerance: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_push_basis"]
    fn push_basis(
        lp: *mut lprec,
        basisvar: *mut ::core::ffi::c_int,
        isbasic: *mut ::core::ffi::c_uchar,
        islower: *mut ::core::ffi::c_uchar,
    ) -> *mut basisrec;
    #[link_name="honest_lpsolve_compare_basis"]
    fn compare_basis(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_restore_basis"]
    fn restore_basis(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_pop_basis"]
    fn pop_basis(lp: *mut lprec, restore: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_isP1extra"]
    fn isP1extra(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_findBasicFixedvar"]
    fn findBasicFixedvar(
        lp: *mut lprec,
        afternr: ::core::ffi::c_int,
        slacksonly: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_isBasisVarFeasible"]
    fn isBasisVarFeasible(
        lp: *mut lprec,
        tol: ::core::ffi::c_double,
        basis_row: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_isPrimalFeasible"]
    fn isPrimalFeasible(
        lp: *mut lprec,
        tol: ::core::ffi::c_double,
        infeasibles: *mut ::core::ffi::c_int,
        feasibilitygap: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_isDualFeasible"]
    fn isDualFeasible(
        lp: *mut lprec,
        tol: ::core::ffi::c_double,
        boundflips: *mut ::core::ffi::c_int,
        infeasibles: *mut ::core::ffi::c_int,
        feasibilitygap: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_preprocess"]
    fn preprocess(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_postprocess"]
    fn postprocess(lp: *mut lprec);
    #[link_name="honest_lpsolve_performiteration"]
    fn performiteration(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        varin: ::core::ffi::c_int,
        theta: ::core::ffi::c_double,
        primal: ::core::ffi::c_uchar,
        allowminit: ::core::ffi::c_uchar,
        prow: *mut ::core::ffi::c_double,
        nzprow: *mut ::core::ffi::c_int,
        pcol: *mut ::core::ffi::c_double,
        nzpcol: *mut ::core::ffi::c_int,
        boundswaps: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_transfer_solution"]
    fn transfer_solution(lp: *mut lprec, dofinal: ::core::ffi::c_uchar);
    #[link_name="honest_lpsolve_set_OF_p1extra"]
    fn set_OF_p1extra(lp: *mut lprec, p1extra: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_unset_OF_p1extra"]
    fn unset_OF_p1extra(lp: *mut lprec);
    #[link_name="honest_lpsolve_obtain_column"]
    fn obtain_column(
        lp: *mut lprec,
        varin: ::core::ffi::c_int,
        pcol: *mut ::core::ffi::c_double,
        nzlist: *mut ::core::ffi::c_int,
        maxabs: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_compute_theta"]
    fn compute_theta(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        theta: *mut ::core::ffi::c_double,
        isupbound: ::core::ffi::c_int,
        HarrisScalar: ::core::ffi::c_double,
        primal: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_check_degeneracy"]
    fn check_degeneracy(
        lp: *mut lprec,
        pcol: *mut ::core::ffi::c_double,
        degencount: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
    #[link_name = "mod"]
    fn mod_0(n: ::core::ffi::c_int, d: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_timeNow"]
    fn timeNow() -> ::core::ffi::c_double;
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
#[export_name="honest_lpsolve_stallMonitor_update"]
pub unsafe extern "C" fn stallMonitor_update(mut lp: *mut lprec, mut newOF: ::core::ffi::c_double) {
    let mut newpos: ::core::ffi::c_int = 0;
    let mut monitor: *mut OBJmonrec = (*lp).monitor;
    if (*monitor).countstep < OBJ_STEPS {
        (*monitor).countstep += 1;
    } else {
        (*monitor).startstep = mod_0((*monitor).startstep + 1 as ::core::ffi::c_int, OBJ_STEPS);
    }
    newpos = mod_0(
        (*monitor).startstep + (*monitor).countstep - 1 as ::core::ffi::c_int,
        OBJ_STEPS,
    );
    (*monitor).objstep[newpos as usize] = newOF;
    (*monitor).idxstep[newpos as usize] = (*monitor).Icount;
    (*monitor).currentstep = newpos;
}
#[export_name="honest_lpsolve_stallMonitor_creepingObj"]
pub unsafe extern "C" fn stallMonitor_creepingObj(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut monitor: *mut OBJmonrec = (*lp).monitor;
    if (*monitor).countstep > 1 as ::core::ffi::c_int {
        let mut deltaOF: ::core::ffi::c_double = ((*monitor).objstep
            [(*monitor).currentstep as usize]
            - (*monitor).objstep[(*monitor).startstep as usize])
            / (*monitor).countstep as ::core::ffi::c_double;
        deltaOF /= (if 1 as ::core::ffi::c_int
            > (*monitor).idxstep[(*monitor).currentstep as usize]
                - (*monitor).idxstep[(*monitor).startstep as usize]
        {
            1 as ::core::ffi::c_int
        } else {
            (*monitor).idxstep[(*monitor).currentstep as usize]
                - (*monitor).idxstep[(*monitor).startstep as usize]
        }) as ::core::ffi::c_double;
        deltaOF = if (*monitor).isdual as ::core::ffi::c_int != 0
            && deltaOF != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -deltaOF
        } else {
            deltaOF
        };
        return (deltaOF < (*monitor).epsvalue) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_stallMonitor_shortSteps"]
pub unsafe extern "C" fn stallMonitor_shortSteps(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut monitor: *mut OBJmonrec = (*lp).monitor;
    if (*monitor).countstep == OBJ_STEPS {
        let mut deltaOF: ::core::ffi::c_double = ((if 1 as ::core::ffi::c_int
            > (*monitor).idxstep[(*monitor).currentstep as usize]
                - (*monitor).idxstep[(*monitor).startstep as usize]
        {
            1 as ::core::ffi::c_int
        } else {
            (*monitor).idxstep[(*monitor).currentstep as usize]
                - (*monitor).idxstep[(*monitor).startstep as usize]
        }) / (*monitor).countstep)
            as ::core::ffi::c_double;
        deltaOF = pow(deltaOF * OBJ_STEPS as ::core::ffi::c_double, 0.66f64);
        return (deltaOF > (*monitor).limitstall[TRUE as usize] as ::core::ffi::c_double)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_stallMonitor_reset"]
pub unsafe extern "C" fn stallMonitor_reset(mut lp: *mut lprec) {
    let mut monitor: *mut OBJmonrec = (*lp).monitor;
    (*monitor).ruleswitches = 0 as ::core::ffi::c_int;
    (*monitor).Ncycle = 0 as ::core::ffi::c_int;
    (*monitor).Mcycle = 0 as ::core::ffi::c_int;
    (*monitor).Icount = 0 as ::core::ffi::c_int;
    (*monitor).startstep = 0 as ::core::ffi::c_int;
    (*monitor).objstep[(*monitor).startstep as usize] = (*lp).infinite;
    (*monitor).idxstep[(*monitor).startstep as usize] = (*monitor).Icount;
    (*monitor).prevobj = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*monitor).countstep = 1 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_stallMonitor_create"]
pub unsafe extern "C" fn stallMonitor_create(
    mut lp: *mut lprec,
    mut isdual: ::core::ffi::c_uchar,
    mut funcname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut monitor: *mut OBJmonrec = ::core::ptr::null_mut::<OBJmonrec>();
    if !(*lp).monitor.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    monitor = calloc(::core::mem::size_of::<OBJmonrec>() as size_t, 1 as size_t) as *mut OBJmonrec;
    if monitor.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    (*monitor).lp = lp;
    strcpy(
        &raw mut (*monitor).spxfunc as *mut ::core::ffi::c_char,
        funcname,
    );
    (*monitor).isdual = isdual;
    (*monitor).pivdynamic = is_piv_mode(lp, PRICE_ADAPTIVE);
    (*monitor).oldpivstrategy = (*lp).piv_strategy;
    (*monitor).oldpivrule = get_piv_rule(lp);
    if MAX_STALLCOUNT <= 1 as ::core::ffi::c_int {
        (*monitor).limitstall[FALSE as usize] = 0 as ::core::ffi::c_int;
    } else {
        (*monitor).limitstall[FALSE as usize] = if 12 as ::core::ffi::c_int
            > pow(
                ((*lp).rows + (*lp).columns) as ::core::ffi::c_double
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double,
                0.667f64,
            ) as ::core::ffi::c_int
        {
            12 as ::core::ffi::c_int
        } else {
            pow(
                ((*lp).rows + (*lp).columns) as ::core::ffi::c_double
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double,
                0.667f64,
            ) as ::core::ffi::c_int
        };
    }
    (*monitor).limitstall[FALSE as usize] *= 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
    (*monitor).limitstall[TRUE as usize] = (*monitor).limitstall[FALSE as usize];
    if (*monitor).oldpivrule == PRICER_DEVEX {
        (*monitor).limitstall[TRUE as usize] *= 2 as ::core::ffi::c_int;
    }
    if MAX_RULESWITCH <= 0 as ::core::ffi::c_int {
        (*monitor).limitruleswitches = MAXINT32;
    } else {
        (*monitor).limitruleswitches =
            if 5 as ::core::ffi::c_int > (*lp).rows / 5 as ::core::ffi::c_int {
                5 as ::core::ffi::c_int
            } else {
                (*lp).rows / 5 as ::core::ffi::c_int
            };
    }
    (*monitor).epsvalue = (*lp).epsprimal;
    (*lp).monitor = monitor;
    stallMonitor_reset(lp);
    (*lp).suminfeas = (*lp).infinite;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_stallMonitor_check"]
pub unsafe extern "C" fn stallMonitor_check(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
    mut lastnr: ::core::ffi::c_int,
    mut minit: ::core::ffi::c_uchar,
    mut approved: ::core::ffi::c_uchar,
    mut forceoutEQ: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut monitor: *mut OBJmonrec = (*lp).monitor;
    let mut isStalled: ::core::ffi::c_uchar = 0;
    let mut isCreeping: ::core::ffi::c_uchar = 0;
    let mut acceptance: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut altrule: ::core::ffi::c_int = 0;
    let mut msglevel: ::core::ffi::c_int = DETAILED;
    let mut deltaobj: ::core::ffi::c_double = (*lp).suminfeas;
    (*monitor).active = FALSE as ::core::ffi::c_uchar;
    if (*monitor).Icount <= 1 as ::core::ffi::c_int {
        if (*monitor).Icount == 1 as ::core::ffi::c_int {
            (*monitor).prevobj = *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize);
            (*monitor).previnfeas = deltaobj;
        }
        (*monitor).Icount += 1;
        return acceptance;
    }
    (*monitor).thisobj = *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize);
    (*monitor).thisinfeas = deltaobj;
    if (*lp).spx_trace as ::core::ffi::c_int != 0 && lastnr > 0 as ::core::ffi::c_int {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"%s: Objective at iter %10.0f is %18.12g (%4d: %4d %s- %4d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    (*monitor).pivrule = get_piv_rule(lp);
    deltaobj = ((*monitor).thisobj - (*monitor).prevobj) / (1.0f64 + fabs((*monitor).prevobj));
    deltaobj = fabs(deltaobj);
    isStalled = (deltaobj < (*monitor).epsvalue) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if isStalled != 0 {
        let mut testvalue: ::core::ffi::c_double = 0.;
        let mut refvalue: ::core::ffi::c_double = (*monitor).epsvalue;
        if (*monitor).isdual != 0 {
            refvalue *= 1000 as ::core::ffi::c_int as ::core::ffi::c_double
                * log10(9.0f64 + (*lp).rows as ::core::ffi::c_double);
        } else {
            refvalue *= 1000 as ::core::ffi::c_int as ::core::ffi::c_double
                * log10(9.0f64 + (*lp).columns as ::core::ffi::c_double);
        }
        testvalue = ((*monitor).thisinfeas - (*monitor).previnfeas)
            / (1.0f64 + fabs((*monitor).previnfeas));
        isStalled = (isStalled as ::core::ffi::c_int
            & (fabs(testvalue) < refvalue) as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        if isStalled == 0
            && testvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && is_action((*lp).anti_degen, ANTIDEGEN_BOUNDFLIP) as ::core::ffi::c_int != 0
        {
            acceptance = AUTOMATIC as ::core::ffi::c_uchar;
        }
    }
    isCreeping = FALSE as ::core::ffi::c_uchar;
    if isStalled as ::core::ffi::c_int != 0 || isCreeping as ::core::ffi::c_int != 0 {
        if minit as ::core::ffi::c_int != ITERATE_MAJORMAJOR {
            (*monitor).Mcycle += 1;
            if (*monitor).Mcycle > 2 as ::core::ffi::c_int {
                (*monitor).Mcycle = 0 as ::core::ffi::c_int;
                (*monitor).Ncycle += 1;
            }
        } else {
            (*monitor).Ncycle += 1;
        }
        if (*monitor).Ncycle <= 1 as ::core::ffi::c_int {
            (*monitor).Ccycle = colnr;
            (*monitor).Rcycle = rownr;
        } else if isCreeping as ::core::ffi::c_int != 0
            || (*monitor).Ncycle > (*monitor).limitstall[(*monitor).isdual as usize]
            || (*monitor).Ccycle == rownr && (*monitor).Rcycle == colnr
        {
            (*monitor).active = TRUE as ::core::ffi::c_uchar;
            if (*lp).fixedvars > 0 as ::core::ffi::c_int
                && *forceoutEQ as ::core::ffi::c_int != TRUE
            {
                *forceoutEQ = TRUE as ::core::ffi::c_uchar;
            } else {
                approved = (approved as ::core::ffi::c_int
                    & ((*monitor).pivdynamic as ::core::ffi::c_int != 0
                        && (*monitor).ruleswitches < (*monitor).limitruleswitches)
                        as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                if approved == 0 && is_anti_degen(lp, ANTIDEGEN_STALLING) == 0 {
                    (*lp).spx_status = DEGENERATE;
                    report(
                        lp,
                        msglevel,
                        b"%s: Stalling at iter %10.0f; no alternative strategy left.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    acceptance = FALSE as ::core::ffi::c_uchar;
                    return acceptance;
                }
                match (*monitor).oldpivrule {
                    PRICER_FIRSTINDEX => {
                        altrule = PRICER_DEVEX;
                    }
                    PRICER_DANTZIG => {
                        altrule = PRICER_DEVEX;
                    }
                    PRICER_DEVEX => {
                        altrule = PRICER_STEEPESTEDGE;
                    }
                    PRICER_STEEPESTEDGE => {
                        altrule = PRICER_DEVEX;
                    }
                    _ => {
                        altrule = PRICER_FIRSTINDEX;
                    }
                }
                if approved as ::core::ffi::c_int != 0
                    && (*monitor).pivrule != altrule
                    && (*monitor).pivrule == (*monitor).oldpivrule
                {
                    (*monitor).ruleswitches += 1;
                    (*lp).piv_strategy = altrule;
                    (*monitor).Ccycle = 0 as ::core::ffi::c_int;
                    (*monitor).Rcycle = 0 as ::core::ffi::c_int;
                    (*monitor).Ncycle = 0 as ::core::ffi::c_int;
                    (*monitor).Mcycle = 0 as ::core::ffi::c_int;
                    report(
                        lp,
                        msglevel,
                        b"%s: Stalling at iter %10.0f; changed to '%s' rule.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    if altrule == PRICER_DEVEX || altrule == PRICER_STEEPESTEDGE {
                        restartPricer(lp, AUTOMATIC as ::core::ffi::c_uchar);
                    }
                } else {
                    report(
                        lp,
                        msglevel,
                        b"%s: Stalling at iter %10.0f; proceed to bound relaxation.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    acceptance = FALSE as ::core::ffi::c_uchar;
                    (*lp).spx_status = DEGENERATE;
                    return acceptance;
                }
            }
        }
    } else {
        if (*monitor).pivrule != (*monitor).oldpivrule {
            (*lp).piv_strategy = (*monitor).oldpivstrategy;
            altrule = (*monitor).oldpivrule;
            if altrule == PRICER_DEVEX || altrule == PRICER_STEEPESTEDGE {
                restartPricer(lp, AUTOMATIC as ::core::ffi::c_uchar);
            }
            report(
                lp,
                msglevel,
                b"...returned to original pivot selection rule at iter %.0f.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        stallMonitor_update(lp, (*monitor).thisobj);
        (*monitor).Ccycle = 0 as ::core::ffi::c_int;
        (*monitor).Rcycle = 0 as ::core::ffi::c_int;
        (*monitor).Ncycle = 0 as ::core::ffi::c_int;
        (*monitor).Mcycle = 0 as ::core::ffi::c_int;
    }
    (*monitor).Icount += 1;
    if deltaobj >= (*monitor).epsvalue {
        (*monitor).prevobj = (*monitor).thisobj;
    }
    (*monitor).previnfeas = (*monitor).thisinfeas;
    return acceptance;
}
#[export_name="honest_lpsolve_stallMonitor_finish"]
pub unsafe extern "C" fn stallMonitor_finish(mut lp: *mut lprec) {
    let mut monitor: *mut OBJmonrec = (*lp).monitor;
    if monitor.is_null() {
        return;
    }
    if (*lp).piv_strategy != (*monitor).oldpivstrategy {
        (*lp).piv_strategy = (*monitor).oldpivstrategy;
    }
    if !(monitor as *mut ::core::ffi::c_void).is_null() {
        free(monitor as *mut ::core::ffi::c_void);
        monitor = ::core::ptr::null_mut::<OBJmonrec>();
    }
    (*lp).monitor = ::core::ptr::null_mut::<OBJmonrec>();
}
#[export_name="honest_lpsolve_add_artificial"]
pub unsafe extern "C" fn add_artificial(
    mut lp: *mut lprec,
    mut forrownr: ::core::ffi::c_int,
    mut nzarray: *mut ::core::ffi::c_double,
    mut idxarray: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut add: ::core::ffi::c_uchar = 0;
    add = (isBasisVarFeasible(lp, (*lp).epspivot, forrownr) == 0) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    if add != 0 {
        let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut i: ::core::ffi::c_int = 0;
        let mut bvar: ::core::ffi::c_int = 0;
        let mut ii: ::core::ffi::c_int = 0;
        let mut avalue: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut rhscoef: ::core::ffi::c_double = 0.;
        let mut acoef: ::core::ffi::c_double = 0.;
        let mut mat: *mut MATrec = (*lp).matA;
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            if *(*lp).var_basic.offset(i as isize) == forrownr {
                break;
            }
            i += 1;
        }
        acoef = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        if i > (*lp).rows {
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).rows {
                ii = *(*lp).var_basic.offset(i as isize) - (*lp).rows;
                if !(ii <= 0 as ::core::ffi::c_int || ii > (*lp).columns - (*lp).P1extraDim) {
                    ii = mat_findelm(mat, forrownr, ii);
                    if ii >= 0 as ::core::ffi::c_int {
                        acoef = *(*mat).col_mat_value.offset(ii as isize);
                        break;
                    }
                }
                i += 1;
            }
        }
        bvar = i;
        add = (bvar <= (*lp).rows) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if add != 0 {
            rhscoef = *(*lp).rhs.offset(forrownr as isize);
            if nzarray.is_null() {
                allocREAL(
                    lp,
                    &raw mut avalue,
                    2 as ::core::ffi::c_int,
                    FALSE as ::core::ffi::c_uchar,
                );
            } else {
                avalue = nzarray;
            }
            if idxarray.is_null() {
                allocINT(
                    lp,
                    &raw mut rownr,
                    2 as ::core::ffi::c_int,
                    FALSE as ::core::ffi::c_uchar,
                );
            } else {
                rownr = idxarray;
            }
            *rownr.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
            *avalue.offset(0 as ::core::ffi::c_int as isize) =
                (if is_chsign(lp, 0 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                    && 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_double;
            *rownr.offset(1 as ::core::ffi::c_int as isize) = forrownr;
            *avalue.offset(1 as ::core::ffi::c_int as isize) =
                (if is_chsign(lp, forrownr) as ::core::ffi::c_int != 0
                    && (if rhscoef / acoef < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) != 0 as ::core::ffi::c_int
                {
                    -if rhscoef / acoef < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }
                } else if rhscoef / acoef < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_double;
            add_columnex(lp, 2 as ::core::ffi::c_int, avalue, rownr);
            if idxarray.is_null() {
                if !(rownr as *mut ::core::ffi::c_void).is_null() {
                    free(rownr as *mut ::core::ffi::c_void);
                    rownr = ::core::ptr::null_mut::<::core::ffi::c_int>();
                }
            }
            if nzarray.is_null() {
                if !(avalue as *mut ::core::ffi::c_void).is_null() {
                    free(avalue as *mut ::core::ffi::c_void);
                    avalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
                }
            }
            set_basisvar(lp, bvar, (*lp).sum);
            (*lp).P1extraDim += 1;
        } else {
            report(
                lp,
                1 as ::core::ffi::c_int,
                b"add_artificial: Could not find replacement basis variable for row %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            (*lp).basis_valid = FALSE as ::core::ffi::c_uchar;
        }
    }
    return add;
}
#[export_name="honest_lpsolve_get_artificialRow"]
pub unsafe extern "C" fn get_artificialRow(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mat: *mut MATrec = (*lp).matA;
    colnr = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    colnr = *(*mat).col_mat_rownr.offset(colnr as isize);
    return colnr;
}
#[export_name="honest_lpsolve_findAnti_artificial"]
pub unsafe extern "C" fn findAnti_artificial(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut rownr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut P1extraDim: ::core::ffi::c_int = abs((*lp).P1extraDim);
    if P1extraDim == 0 as ::core::ffi::c_int
        || colnr > (*lp).rows
        || *(*lp).is_basic.offset(colnr as isize) == 0
    {
        return rownr;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        k = *(*lp).var_basic.offset(i as isize);
        if k > (*lp).sum - P1extraDim
            && *(*lp).rhs.offset(i as isize) == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            rownr = get_artificialRow(lp, k - (*lp).rows);
            if rownr == colnr {
                break;
            }
            rownr = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return rownr;
}
#[export_name="honest_lpsolve_findBasicArtificial"]
pub unsafe extern "C" fn findBasicArtificial(
    mut lp: *mut lprec,
    mut before: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut P1extraDim: ::core::ffi::c_int = abs((*lp).P1extraDim);
    if P1extraDim > 0 as ::core::ffi::c_int {
        if before > (*lp).rows || before <= 1 as ::core::ffi::c_int {
            i = (*lp).rows;
        } else {
            i = before;
        }
        while i > 0 as ::core::ffi::c_int
            && *(*lp).var_basic.offset(i as isize) <= (*lp).sum - P1extraDim
        {
            i -= 1;
        }
    }
    return i;
}
#[export_name="honest_lpsolve_eliminate_artificials"]
pub unsafe extern "C" fn eliminate_artificials(
    mut lp: *mut lprec,
    mut prow: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut rownr: ::core::ffi::c_int = 0;
    let mut P1extraDim: ::core::ffi::c_int = abs((*lp).P1extraDim);
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows && P1extraDim > 0 as ::core::ffi::c_int {
        j = *(*lp).var_basic.offset(i as isize);
        if !(j <= (*lp).sum - P1extraDim) {
            j -= (*lp).rows;
            rownr = get_artificialRow(lp, j);
            colnr = find_rowReplacement(
                lp,
                rownr,
                prow,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            set_basisvar(lp, rownr, colnr);
            del_column(lp, j);
            P1extraDim -= 1;
        }
        i += 1;
    }
    (*lp).P1extraDim = 0 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_clear_artificials"]
pub unsafe extern "C" fn clear_artificials(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut P1extraDim: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    P1extraDim = abs((*lp).P1extraDim);
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows && n < P1extraDim {
        j = *(*lp).var_basic.offset(i as isize);
        if !(j <= (*lp).sum - P1extraDim) {
            j = get_artificialRow(lp, j - (*lp).rows);
            set_basisvar(lp, i, j);
            n += 1;
        }
        i += 1;
    }
    while P1extraDim > 0 as ::core::ffi::c_int {
        i = (*lp).sum - (*lp).rows;
        del_column(lp, i);
        P1extraDim -= 1;
    }
    (*lp).P1extraDim = 0 as ::core::ffi::c_int;
    if n > 0 as ::core::ffi::c_int {
        set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
        (*lp).basis_valid = TRUE as ::core::ffi::c_uchar;
    }
}
#[export_name="honest_lpsolve_primloop"]
pub unsafe extern "C" fn primloop(
    mut lp: *mut lprec,
    mut primalfeasible: ::core::ffi::c_uchar,
    mut primaloffset: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut primal: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut bfpfinal: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut changedphase: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut forceoutEQ: ::core::ffi::c_uchar = AUTOMATIC as ::core::ffi::c_uchar;
    let mut primalphase1: ::core::ffi::c_uchar = 0;
    let mut pricerCanChange: ::core::ffi::c_uchar = 0;
    let mut minit: ::core::ffi::c_uchar = 0;
    let mut stallaccept: ::core::ffi::c_uchar = 0;
    let mut pendingunbounded: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rownr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lastnr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut candidatecount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut minitcount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ok: ::core::ffi::c_int = TRUE;
    let mut theta: ::core::ffi::c_double = 0.0f64;
    let mut epsvalue: ::core::ffi::c_double = 0.;
    let mut xviolated: ::core::ffi::c_double = 0.0f64;
    let mut cviolated: ::core::ffi::c_double = 0.0f64;
    let mut prow: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut pcol: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut drow: *mut ::core::ffi::c_double = (*lp).drow;
    let mut workINT: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nzdrow: *mut ::core::ffi::c_int = (*lp).nzdrow;
    if (*lp).spx_trace != 0 {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"Entered primal simplex algorithm with feasibility %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    (*lp).P1extraDim = 0 as ::core::ffi::c_int;
    if primalfeasible == 0 {
        (*lp).simplex_mode = SIMPLEX_Phase1_PRIMAL;
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            add_artificial(
                lp,
                i,
                ::core::ptr::null_mut::<::core::ffi::c_double>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            i += 1;
        }
        if (*lp).P1extraDim > 0 as ::core::ffi::c_int {
            ok = (allocREAL(
                lp,
                &raw mut (*lp).drow,
                (*lp).sum + 1 as ::core::ffi::c_int,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0
                && allocINT(
                    lp,
                    &raw mut (*lp).nzdrow,
                    (*lp).sum + 1 as ::core::ffi::c_int,
                    AUTOMATIC as ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int
                    != 0) as ::core::ffi::c_int;
            if ok == 0 {
                current_block = 3790274334279233252;
            } else {
                *(*lp).nzdrow.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
                drow = (*lp).drow;
                nzdrow = (*lp).nzdrow;
                mat_validate((*lp).matA);
                set_OF_p1extra(lp, 0.0f64);
                current_block = 17860125682698302841;
            }
        } else {
            current_block = 17860125682698302841;
        }
        match current_block {
            3790274334279233252 => {}
            _ => {
                if (*lp).spx_trace != 0 {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"P1extraDim count = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                simplexPricer(
                    lp,
                    (primal == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
                );
                invert(
                    lp,
                    INITSOL_USEZERO as ::core::ffi::c_uchar,
                    TRUE as ::core::ffi::c_uchar,
                );
                current_block = 11584701595673473500;
            }
        }
    } else {
        (*lp).simplex_mode = SIMPLEX_Phase2_PRIMAL;
        restartPricer(
            lp,
            (primal == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
        );
        current_block = 11584701595673473500;
    }
    match current_block {
        11584701595673473500 => {
            ok = (allocREAL(
                lp,
                &raw mut (*lp).bsolveVal,
                (*lp).rows + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) as ::core::ffi::c_int
                != 0
                && allocREAL(
                    lp,
                    &raw mut prow,
                    (*lp).sum + 1 as ::core::ffi::c_int,
                    TRUE as ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int
                    != 0
                && allocREAL(
                    lp,
                    &raw mut pcol,
                    (*lp).rows + 1 as ::core::ffi::c_int,
                    TRUE as ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int
                    != 0) as ::core::ffi::c_int;
            if is_piv_mode(lp, PRICE_MULTIPLE) as ::core::ffi::c_int != 0
                && (*lp).multiblockdiv > 1 as ::core::ffi::c_int
            {
                (*lp).multivars = multi_create(lp, FALSE as ::core::ffi::c_uchar);
                ok &= (!(*lp).multivars.is_null()
                    && multi_resize(
                        (*lp).multivars,
                        (*lp).sum / (*lp).multiblockdiv,
                        2 as ::core::ffi::c_int,
                        FALSE as ::core::ffi::c_uchar,
                        TRUE as ::core::ffi::c_uchar,
                    ) as ::core::ffi::c_int
                        != 0) as ::core::ffi::c_int;
            }
            if !(ok == 0) {
                (*lp).spx_status = RUNNING;
                minit = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
                epsvalue = (*lp).epspivot;
                pendingunbounded = FALSE as ::core::ffi::c_uchar;
                ok = stallMonitor_create(
                    lp,
                    FALSE as ::core::ffi::c_uchar,
                    b"primloop\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_int;
                if !(ok == 0) {
                    *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) =
                        0 as ::core::ffi::c_int;
                    's_151: while (*lp).spx_status == RUNNING
                        && userabort(lp, -(1 as ::core::ffi::c_int)) == 0
                    {
                        primalphase1 = ((*lp).P1extraDim > 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_uchar;
                        clear_action(&raw mut (*lp).spx_action, ACTION_REINVERT | ACTION_ITERATE);
                        pricerCanChange =
                            (primalphase1 == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        stallaccept = stallMonitor_check(
                            lp,
                            rownr,
                            colnr,
                            lastnr,
                            minit,
                            pricerCanChange,
                            &raw mut forceoutEQ,
                        );
                        if stallaccept == 0 {
                            break;
                        }
                        loop {
                            if changedphase == 0 {
                                i = 0 as ::core::ffi::c_int;
                                loop {
                                    i += 1;
                                    colnr = colprim(
                                        lp,
                                        drow,
                                        nzdrow,
                                        (minit as ::core::ffi::c_int == ITERATE_MINORRETRY)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uchar,
                                        i,
                                        &raw mut candidatecount,
                                        TRUE as ::core::ffi::c_uchar,
                                        &raw mut xviolated,
                                    );
                                    if !(colnr == 0 as ::core::ffi::c_int
                                        && i < partial_countBlocks(
                                            lp,
                                            (primal == 0) as ::core::ffi::c_int
                                                as ::core::ffi::c_uchar,
                                        )
                                        && partial_blockStep(
                                            lp,
                                            (primal == 0) as ::core::ffi::c_int
                                                as ::core::ffi::c_uchar,
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        break;
                                    }
                                }
                                if colnr == 0 as ::core::ffi::c_int {
                                    (*lp).spx_status = OPTIMAL;
                                }
                                if *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize)
                                    > 0 as ::core::ffi::c_int
                                {
                                    minit = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
                                }
                                if is_action((*lp).spx_action, ACTION_REINVERT) != 0 {
                                    bfpfinal = TRUE as ::core::ffi::c_uchar;
                                }
                            }
                            if colnr == 0 as ::core::ffi::c_int
                                && *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize)
                                    > 0 as ::core::ffi::c_int
                            {
                                (*lp).spx_status = UNBOUNDED;
                                if (*lp).spx_trace as ::core::ffi::c_int != 0
                                    && (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                    || (*lp).bb_trace as ::core::ffi::c_int != 0
                                        && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
                                {
                                    report(
                                        lp,
                                        5 as ::core::ffi::c_int,
                                        b"The model is primal unbounded.\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                                colnr = *(*lp).rejectpivot.offset(1 as ::core::ffi::c_int as isize);
                                rownr = 0 as ::core::ffi::c_int;
                                *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) =
                                    0 as ::core::ffi::c_int;
                                ok = FALSE;
                                break 's_151;
                            } else {
                                if !(colnr > 0 as ::core::ffi::c_int) {
                                    current_block = 15012059527946116900;
                                    break;
                                }
                                changedphase = FALSE as ::core::ffi::c_uchar;
                                fsolve(
                                    lp,
                                    colnr,
                                    pcol,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                    (*lp).epsmachine,
                                    1.0f64,
                                    TRUE as ::core::ffi::c_uchar,
                                );
                                if is_anti_degen(lp, ANTIDEGEN_COLUMNCHECK) as ::core::ffi::c_int
                                    != 0
                                    && check_degeneracy(
                                        lp,
                                        pcol,
                                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                    ) == 0
                                {
                                    if *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize)
                                        < DEF_MAXPIVOTRETRY / 3 as ::core::ffi::c_int
                                    {
                                        let ref mut fresh0 = *(*lp)
                                            .rejectpivot
                                            .offset(0 as ::core::ffi::c_int as isize);
                                        *fresh0 += 1;
                                        i = *fresh0;
                                        *(*lp).rejectpivot.offset(i as isize) = colnr;
                                        report(
                                            lp,
                                            5 as ::core::ffi::c_int,
                                            b"Entering column %d found to be non-improving due to degeneracy.\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                        minit = ITERATE_MINORRETRY as ::core::ffi::c_uchar;
                                        continue;
                                    } else {
                                        *(*lp)
                                            .rejectpivot
                                            .offset(0 as ::core::ffi::c_int as isize) =
                                            0 as ::core::ffi::c_int;
                                        report(
                                            lp,
                                            5 as ::core::ffi::c_int,
                                            b"Gave up trying to find a strictly improving entering column.\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    }
                                }
                                theta = *drow.offset(colnr as isize);
                                rownr = rowprim(
                                    lp,
                                    colnr,
                                    &raw mut theta,
                                    pcol,
                                    workINT,
                                    forceoutEQ,
                                    &raw mut cviolated,
                                );
                                if rownr > 0 as ::core::ffi::c_int
                                    && xviolated + cviolated < (*lp).epspivot
                                {
                                    if (*lp).bb_trace as ::core::ffi::c_int != 0
                                        || (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                    {
                                        report(
                                            lp,
                                            5 as ::core::ffi::c_int,
                                            b"primloop: Assuming convergence with reduced accuracy %g.\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    }
                                    rownr = 0 as ::core::ffi::c_int;
                                    colnr = 0 as ::core::ffi::c_int;
                                    current_block = 15012059527946116900;
                                    break;
                                } else {
                                    if (*lp).P1extraDim != 0 as ::core::ffi::c_int
                                        && rownr == 0 as ::core::ffi::c_int
                                        && colnr <= (*lp).rows
                                    {
                                        rownr = findAnti_artificial(lp, colnr);
                                    }
                                    if rownr > 0 as ::core::ffi::c_int {
                                        pendingunbounded = FALSE as ::core::ffi::c_uchar;
                                        *(*lp)
                                            .rejectpivot
                                            .offset(0 as ::core::ffi::c_int as isize) =
                                            0 as ::core::ffi::c_int;
                                        set_action(&raw mut (*lp).spx_action, ACTION_ITERATE);
                                        if (*lp).obj_in_basis == 0 {
                                            *pcol.offset(0 as ::core::ffi::c_int as isize) =
                                                if *(*lp).is_lower.offset(colnr as isize) == 0
                                                    && *drow.offset(colnr as isize)
                                                        != 0 as ::core::ffi::c_int
                                                            as ::core::ffi::c_double
                                                {
                                                    -*drow.offset(colnr as isize)
                                                } else {
                                                    *drow.offset(colnr as isize)
                                                };
                                        }
                                        (*lp).bfp_prepareupdate.expect("non-null function pointer")(
                                            lp, rownr, colnr, pcol,
                                        );
                                        current_block = 16910810822589621899;
                                        break;
                                    } else if *(*lp)
                                        .rejectpivot
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        < DEF_MAXPIVOTRETRY
                                    {
                                        (*lp).spx_status = RUNNING;
                                        let ref mut fresh1 = *(*lp)
                                            .rejectpivot
                                            .offset(0 as ::core::ffi::c_int as isize);
                                        *fresh1 += 1;
                                        *(*lp).rejectpivot.offset(
                                            *(*lp)
                                                .rejectpivot
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as isize,
                                        ) = colnr;
                                        report(
                                            lp,
                                            5 as ::core::ffi::c_int,
                                            b"...trying to recover via another pivot column.\n\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                        minit = ITERATE_MINORRETRY as ::core::ffi::c_uchar;
                                    } else if refactRecent(lp) == 0 && pendingunbounded == 0 {
                                        current_block = 5793491756164225964;
                                        break;
                                    } else {
                                        current_block = 993425571616822999;
                                        break;
                                    }
                                }
                            }
                        }
                        match current_block {
                            15012059527946116900 => {
                                if primalfeasible == 0 || isP1extra(lp) as ::core::ffi::c_int != 0 {
                                    if feasiblePhase1(lp, epsvalue) != 0 {
                                        (*lp).spx_status = RUNNING;
                                        if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong {
                                            report(
                                                lp,
                                                4 as ::core::ffi::c_int,
                                                b"Found feasibility by primal simplex after  %10.0f iter.\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                            if (*lp).usermessage.is_some()
                                                && (*lp).msgmask & MSG_LPFEASIBLE != 0
                                            {
                                                (*lp)
                                                    .usermessage
                                                    .expect("non-null function pointer")(
                                                    lp,
                                                    (*lp).msghandle,
                                                    MSG_LPFEASIBLE,
                                                );
                                            }
                                        }
                                        changedphase = FALSE as ::core::ffi::c_uchar;
                                        primalfeasible = TRUE as ::core::ffi::c_uchar;
                                        (*lp).simplex_mode = SIMPLEX_Phase2_PRIMAL;
                                        set_OF_p1extra(lp, 0.0f64);
                                        if (*lp).P1extraDim > 0 as ::core::ffi::c_int {
                                            if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                                && MIP_count(lp) == 0 as ::core::ffi::c_int
                                            {
                                                while (*lp).P1extraDim > 0 as ::core::ffi::c_int {
                                                    i = (*lp).rows;
                                                    while i > 0 as ::core::ffi::c_int
                                                        && *(*lp).var_basic.offset(i as isize)
                                                            <= (*lp).sum - (*lp).P1extraDim
                                                    {
                                                        i -= 1;
                                                    }
                                                    j = *(*lp).var_basic.offset(i as isize)
                                                        - (*lp).rows;
                                                    k = get_artificialRow(lp, j);
                                                    if *(*lp).is_basic.offset(k as isize) != 0 {
                                                        *(*lp)
                                                            .is_basic
                                                            .offset(((*lp).rows + j) as isize) =
                                                            FALSE as ::core::ffi::c_uchar;
                                                        del_constraint(lp, k);
                                                    } else {
                                                        set_basisvar(lp, i, k);
                                                    }
                                                    del_column(lp, j);
                                                    (*lp).P1extraDim -= 1;
                                                }
                                                (*lp).basis_valid = TRUE as ::core::ffi::c_uchar;
                                            } else {
                                                eliminate_artificials(lp, prow);
                                            }
                                        }
                                        set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
                                        bfpfinal = TRUE as ::core::ffi::c_uchar;
                                    } else {
                                        (*lp).spx_status = INFEASIBLE;
                                        minit = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
                                        if (*lp).spx_trace != 0 {
                                            report(
                                                lp,
                                                4 as ::core::ffi::c_int,
                                                b"Model infeasible by primal simplex at iter   %10.0f.\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                }
                                if (*lp).bb_level <= 1 as ::core::ffi::c_int
                                    || (*lp).improve & IMPROVE_BBSIMPLEX != 0
                                {
                                    set_action(&raw mut (*lp).piv_strategy, PRICE_FORCEFULL);
                                    i = rowdual(
                                        lp,
                                        (*lp).rhs,
                                        FALSE as ::core::ffi::c_uchar,
                                        FALSE as ::core::ffi::c_uchar,
                                        ::core::ptr::null_mut::<::core::ffi::c_double>(),
                                    );
                                    clear_action(&raw mut (*lp).piv_strategy, PRICE_FORCEFULL);
                                    if i > 0 as ::core::ffi::c_int {
                                        (*lp).spx_status = LOSTFEAS;
                                        if (*lp).total_iter == 0 as ::core::ffi::c_longlong {
                                            report(
                                                lp,
                                                5 as ::core::ffi::c_int,
                                                b"primloop: Lost primal feasibility at iter  %10.0f: will try to recover.\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                }
                            }
                            993425571616822999 => {
                                (*lp).spx_status = UNBOUNDED;
                                report(
                                    lp,
                                    5 as ::core::ffi::c_int,
                                    b"The model is primal unbounded.\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                break;
                            }
                            5793491756164225964 => {
                                bfpfinal = TRUE as ::core::ffi::c_uchar;
                                pendingunbounded = TRUE as ::core::ffi::c_uchar;
                                set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
                            }
                            _ => {}
                        }
                        if is_action((*lp).spx_action, ACTION_ITERATE) != 0 {
                            lastnr = *(*lp).var_basic.offset(rownr as isize);
                            if refactRecent(lp) as ::core::ffi::c_int == AUTOMATIC {
                                minitcount = 0 as ::core::ffi::c_int;
                            } else if minitcount > MAX_MINITUPDATES {
                                recompute_solution(lp, INITSOL_USEZERO as ::core::ffi::c_uchar);
                                minitcount = 0 as ::core::ffi::c_int;
                            }
                            minit = performiteration(
                                lp,
                                rownr,
                                colnr,
                                theta,
                                primal,
                                (stallaccept as ::core::ffi::c_int != AUTOMATIC)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uchar,
                                ::core::ptr::null_mut::<::core::ffi::c_double>(),
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                pcol,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            );
                            if minit as ::core::ffi::c_int != ITERATE_MAJORMAJOR {
                                minitcount += 1;
                            }
                            if (*lp).spx_status == USERABORT || (*lp).spx_status == TIMEOUT {
                                break;
                            }
                            if minit as ::core::ffi::c_int == ITERATE_MINORMAJOR {
                                continue;
                            }
                            if minit as ::core::ffi::c_int == ITERATE_MAJORMAJOR
                                && lastnr > (*lp).sum - abs((*lp).P1extraDim)
                            {
                                del_column(lp, lastnr - (*lp).rows);
                                if (*lp).P1extraDim > 0 as ::core::ffi::c_int {
                                    (*lp).P1extraDim -= 1;
                                } else {
                                    (*lp).P1extraDim += 1;
                                }
                                if (*lp).P1extraDim == 0 as ::core::ffi::c_int {
                                    colnr = 0 as ::core::ffi::c_int;
                                    changedphase = TRUE as ::core::ffi::c_uchar;
                                    stallMonitor_reset(lp);
                                }
                            }
                        }
                        if !((*lp).spx_status == SWITCH_TO_DUAL) {
                            if changedphase == 0
                                && (*lp)
                                    .bfp_mustrefactorize
                                    .expect("non-null function pointer")(
                                    lp
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                minit = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
                                if invert(lp, INITSOL_USEZERO as ::core::ffi::c_uchar, bfpfinal)
                                    == 0
                                {
                                    (*lp).spx_status = SINGULAR_BASIS;
                                }
                                bfpfinal = FALSE as ::core::ffi::c_uchar;
                            }
                        }
                    }
                    (*lp).P1extraDim = abs((*lp).P1extraDim);
                    if (*lp).P1extraDim > 0 as ::core::ffi::c_int {
                        clear_artificials(lp);
                        if (*lp).spx_status != OPTIMAL {
                            restore_basis(lp);
                        }
                        i = invert(
                            lp,
                            INITSOL_USEZERO as ::core::ffi::c_uchar,
                            TRUE as ::core::ffi::c_uchar,
                        ) as ::core::ffi::c_int;
                    }
                    if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                        && MIP_count(lp) > 0 as ::core::ffi::c_int
                        && (*lp).simplex_strategy & SIMPLEX_Phase1_DUAL == 0 as ::core::ffi::c_int
                    {
                        (*lp).simplex_strategy &= !SIMPLEX_Phase1_PRIMAL;
                        (*lp).simplex_strategy += SIMPLEX_Phase1_DUAL;
                    }
                }
            }
        }
        _ => {}
    }
    stallMonitor_finish(lp);
    multi_free(&raw mut (*lp).multivars);
    if !(prow as *mut ::core::ffi::c_void).is_null() {
        free(prow as *mut ::core::ffi::c_void);
        prow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(pcol as *mut ::core::ffi::c_void).is_null() {
        free(pcol as *mut ::core::ffi::c_void);
        pcol = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).bsolveVal as *mut ::core::ffi::c_void).is_null() {
        free((*lp).bsolveVal as *mut ::core::ffi::c_void);
        (*lp).bsolveVal = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return ok;
}
#[export_name="honest_lpsolve_dualloop"]
pub unsafe extern "C" fn dualloop(
    mut lp: *mut lprec,
    mut dualfeasible: ::core::ffi::c_uchar,
    mut dualinfeasibles: *mut ::core::ffi::c_int,
    mut dualoffset: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut primal: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut inP1extra: ::core::ffi::c_uchar = 0;
    let mut dualphase1: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut changedphase: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut pricerCanChange: ::core::ffi::c_uchar = 0;
    let mut minit: ::core::ffi::c_uchar = 0;
    let mut stallaccept: ::core::ffi::c_uchar = 0;
    let mut longsteps: ::core::ffi::c_uchar = 0;
    let mut forceoutEQ: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut bfpfinal: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rownr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lastnr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut candidatecount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut minitcount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ok: ::core::ffi::c_int = TRUE;
    let mut boundswaps: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut theta: ::core::ffi::c_double = 0.0f64;
    let mut xviolated: ::core::ffi::c_double = 0.;
    let mut cviolated: ::core::ffi::c_double = 0.;
    let mut prow: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut pcol: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut drow: *mut ::core::ffi::c_double = (*lp).drow;
    let mut nzprow: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut workINT: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nzdrow: *mut ::core::ffi::c_int = (*lp).nzdrow;
    if (*lp).spx_trace != 0 {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"Entered dual simplex algorithm with feasibility %s.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    ok = (allocREAL(
        lp,
        &raw mut prow,
        (*lp).sum + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) as ::core::ffi::c_int
        != 0
        && allocINT(
            lp,
            &raw mut nzprow,
            (*lp).sum + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0
        && allocREAL(
            lp,
            &raw mut pcol,
            (*lp).rows + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int;
    if !(ok == 0) {
        inP1extra = (dualoffset != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if inP1extra != 0 {
            set_OF_p1extra(lp, dualoffset);
            simplexPricer(
                lp,
                (primal == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
            );
            invert(
                lp,
                INITSOL_USEZERO as ::core::ffi::c_uchar,
                TRUE as ::core::ffi::c_uchar,
            );
        } else {
            restartPricer(
                lp,
                (primal == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
            );
        }
        longsteps = FALSE as ::core::ffi::c_uchar;
        if longsteps != 0 {
            (*lp).longsteps = multi_create(lp, TRUE as ::core::ffi::c_uchar);
            ok = (!(*lp).longsteps.is_null()
                && multi_resize(
                    (*lp).longsteps,
                    (if ((*lp).boundedvars + 2 as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
                        (*lp).boundedvars + 2 as ::core::ffi::c_int
                    } else {
                        11 as ::core::ffi::c_int
                    }),
                    1 as ::core::ffi::c_int,
                    TRUE as ::core::ffi::c_uchar,
                    TRUE as ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int
                    != 0) as ::core::ffi::c_int;
            if ok == 0 {
                current_block = 4712587579434878807;
            } else {
                boundswaps = multi_indexSet((*lp).longsteps, FALSE as ::core::ffi::c_uchar);
                current_block = 7976072742316086414;
            }
        } else {
            current_block = 7976072742316086414;
        }
        match current_block {
            4712587579434878807 => {}
            _ => {
                (*lp).spx_status = RUNNING;
                minit = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
                ok = stallMonitor_create(
                    lp,
                    TRUE as ::core::ffi::c_uchar,
                    b"dualloop\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_int;
                if !(ok == 0) {
                    *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) =
                        0 as ::core::ffi::c_int;
                    if dualfeasible != 0 {
                        (*lp).simplex_mode = SIMPLEX_Phase2_DUAL;
                    } else {
                        (*lp).simplex_mode = SIMPLEX_Phase1_DUAL;
                    }
                    if dualphase1 as ::core::ffi::c_int != 0
                        || inP1extra as ::core::ffi::c_int != 0
                        || (*lp).fixedvars > 0 as ::core::ffi::c_int
                            && is_anti_degen(lp, ANTIDEGEN_FIXEDVARS) as ::core::ffi::c_int != 0
                    {
                        forceoutEQ = AUTOMATIC as ::core::ffi::c_uchar;
                    }
                    if is_anti_degen(lp, ANTIDEGEN_DYNAMIC) as ::core::ffi::c_int != 0
                        && bin_count(lp, TRUE as ::core::ffi::c_uchar) * 2 as ::core::ffi::c_int
                            > (*lp).columns
                    {
                        match forceoutEQ as ::core::ffi::c_int {
                            FALSE => {
                                forceoutEQ = AUTOMATIC as ::core::ffi::c_uchar;
                            }
                            _ => {}
                        }
                    }
                    let mut current_block_200: u64;
                    's_141: while (*lp).spx_status == RUNNING
                        && userabort(lp, -(1 as ::core::ffi::c_int)) == 0
                    {
                        pricerCanChange = (dualphase1 == 0 && inP1extra == 0) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar;
                        stallaccept = stallMonitor_check(
                            lp,
                            rownr,
                            colnr,
                            lastnr,
                            minit,
                            pricerCanChange,
                            &raw mut forceoutEQ,
                        );
                        if stallaccept == 0 {
                            break;
                        }
                        changedphase = FALSE as ::core::ffi::c_uchar;
                        dualphase1 = (dualphase1 as ::core::ffi::c_int
                            & ((*lp).simplex_mode == SIMPLEX_Phase1_DUAL) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        if longsteps as ::core::ffi::c_int != 0
                            && dualphase1 as ::core::ffi::c_int != 0
                            && inP1extra == 0
                        {
                            obtain_column(
                                lp,
                                *dualinfeasibles.offset(1 as ::core::ffi::c_int as isize),
                                pcol,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            );
                            i = 2 as ::core::ffi::c_int;
                            i = 2 as ::core::ffi::c_int;
                            while i <= *dualinfeasibles.offset(0 as ::core::ffi::c_int as isize) {
                                mat_multadd(
                                    (*lp).matA,
                                    pcol,
                                    *dualinfeasibles.offset(i as isize),
                                    1.0f64,
                                );
                                i += 1;
                            }
                            ftran(
                                lp,
                                pcol,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                (*lp).epsmachine,
                            );
                        }
                        loop {
                            if minit as ::core::ffi::c_int != ITERATE_MINORRETRY {
                                i = 0 as ::core::ffi::c_int;
                                loop {
                                    i += 1;
                                    rownr = rowdual(
                                        lp,
                                        if dualphase1 as ::core::ffi::c_int != 0 {
                                            pcol
                                        } else {
                                            ::core::ptr::null_mut::<::core::ffi::c_double>()
                                        },
                                        forceoutEQ,
                                        TRUE as ::core::ffi::c_uchar,
                                        &raw mut xviolated,
                                    );
                                    if !(rownr == 0 as ::core::ffi::c_int
                                        && i < partial_countBlocks(
                                            lp,
                                            (primal == 0) as ::core::ffi::c_int
                                                as ::core::ffi::c_uchar,
                                        )
                                        && partial_blockStep(
                                            lp,
                                            (primal == 0) as ::core::ffi::c_int
                                                as ::core::ffi::c_uchar,
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        break;
                                    }
                                }
                            }
                            if rownr == 0 as ::core::ffi::c_int
                                && *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize)
                                    > 0 as ::core::ffi::c_int
                            {
                                (*lp).spx_status = INFEASIBLE;
                                if (*lp).spx_trace as ::core::ffi::c_int != 0
                                    && (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                    || (*lp).bb_trace as ::core::ffi::c_int != 0
                                        && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
                                {
                                    report(
                                        lp,
                                        5 as ::core::ffi::c_int,
                                        b"The model is primal infeasible.\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                                rownr = *(*lp).rejectpivot.offset(1 as ::core::ffi::c_int as isize);
                                colnr = 0 as ::core::ffi::c_int;
                                *(*lp).rejectpivot.offset(0 as ::core::ffi::c_int as isize) =
                                    0 as ::core::ffi::c_int;
                                ok = FALSE;
                                break 's_141;
                            } else {
                                clear_action(&raw mut (*lp).spx_action, ACTION_ITERATE);
                                if rownr > 0 as ::core::ffi::c_int {
                                    colnr = coldual(
                                        lp,
                                        rownr,
                                        prow,
                                        nzprow,
                                        drow,
                                        nzdrow,
                                        (dualphase1 as ::core::ffi::c_int != 0 && inP1extra == 0)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uchar,
                                        (minit as ::core::ffi::c_int == ITERATE_MINORRETRY)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uchar,
                                        &raw mut candidatecount,
                                        &raw mut cviolated,
                                    );
                                    if colnr < 0 as ::core::ffi::c_int {
                                        minit = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
                                        continue 's_141;
                                    } else {
                                        if xviolated + cviolated < (*lp).epspivot {
                                            if (*lp).bb_trace as ::core::ffi::c_int != 0
                                                || (*lp).bb_totalnodes
                                                    == 0 as ::core::ffi::c_longlong
                                            {
                                                report(
                                                    lp,
                                                    5 as ::core::ffi::c_int,
                                                    b"dualloop: Assuming convergence with reduced accuracy %g.\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            }
                                            rownr = 0 as ::core::ffi::c_int;
                                            colnr = 0 as ::core::ffi::c_int;
                                        }
                                        if (*lp).spx_status == FATHOMED {
                                            break 's_141;
                                        }
                                    }
                                } else {
                                    colnr = 0 as ::core::ffi::c_int;
                                }
                                if rownr > 0 as ::core::ffi::c_int {
                                    if colnr > 0 as ::core::ffi::c_int {
                                        fsolve(
                                            lp,
                                            colnr,
                                            pcol,
                                            workINT,
                                            (*lp).epsmachine,
                                            1.0f64,
                                            TRUE as ::core::ffi::c_uchar,
                                        );
                                        if *pcol.offset(rownr as isize)
                                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            if (*lp).spx_trace != 0 {
                                                report(
                                                    lp,
                                                    5 as ::core::ffi::c_int,
                                                    b"dualloop: Attempt to divide by zero (pcol[%d])\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            }
                                            if refactRecent(lp) == 0 {
                                                report(
                                                    lp,
                                                    5 as ::core::ffi::c_int,
                                                    b"...trying to recover by refactorizing basis.\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                                set_action(
                                                    &raw mut (*lp).spx_action,
                                                    ACTION_REINVERT,
                                                );
                                                bfpfinal = FALSE as ::core::ffi::c_uchar;
                                            } else {
                                                if (*lp).bb_totalnodes
                                                    == 0 as ::core::ffi::c_longlong
                                                {
                                                    report(
                                                        lp,
                                                        5 as ::core::ffi::c_int,
                                                        b"...cannot recover by refactorizing basis.\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                }
                                                (*lp).spx_status = NUMFAILURE;
                                                ok = FALSE;
                                            }
                                        } else {
                                            set_action(&raw mut (*lp).spx_action, ACTION_ITERATE);
                                            *(*lp)
                                                .rejectpivot
                                                .offset(0 as ::core::ffi::c_int as isize) =
                                                0 as ::core::ffi::c_int;
                                            if (*lp).obj_in_basis == 0 {
                                                *pcol.offset(0 as ::core::ffi::c_int as isize) =
                                                    if *(*lp).is_lower.offset(colnr as isize) == 0
                                                        && *drow.offset(colnr as isize)
                                                            != 0 as ::core::ffi::c_int
                                                                as ::core::ffi::c_double
                                                    {
                                                        -*drow.offset(colnr as isize)
                                                    } else {
                                                        *drow.offset(colnr as isize)
                                                    };
                                            }
                                            theta = (*lp)
                                                .bfp_prepareupdate
                                                .expect("non-null function pointer")(
                                                lp, rownr, colnr, pcol,
                                            );
                                            if (*lp).improve & IMPROVE_THETAGAP != 0
                                                && refactRecent(lp) == 0
                                                && (fabs(theta)
                                                    - fabs(*prow.offset(colnr as isize)))
                                                    / (1.0f64
                                                        + fabs(fabs(*prow.offset(colnr as isize))))
                                                    > (*lp).epspivot
                                                        * 10.0f64
                                                        * log(2.0f64
                                                            + 50.0f64
                                                                * (*lp).rows
                                                                    as ::core::ffi::c_double)
                                            {
                                                set_action(
                                                    &raw mut (*lp).spx_action,
                                                    ACTION_REINVERT,
                                                );
                                                bfpfinal = TRUE as ::core::ffi::c_uchar;
                                                report(
                                                    lp,
                                                    5 as ::core::ffi::c_int,
                                                    b"dualloop: Refactorizing at iter %.0f due to loss of accuracy.\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            }
                                            theta = *prow.offset(colnr as isize);
                                            compute_theta(
                                                lp,
                                                rownr,
                                                &raw mut theta,
                                                (*(*lp).is_lower.offset(colnr as isize) == 0)
                                                    as ::core::ffi::c_int,
                                                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                                                primal,
                                            );
                                        }
                                        current_block_200 = 8968043056769084000;
                                        break;
                                    } else if refactRecent(lp) == 0 {
                                        set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
                                        bfpfinal = TRUE as ::core::ffi::c_uchar;
                                        current_block_200 = 8968043056769084000;
                                        break;
                                    } else if *(*lp)
                                        .rejectpivot
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        < DEF_MAXPIVOTRETRY
                                    {
                                        (*lp).spx_status = RUNNING;
                                        let ref mut fresh2 = *(*lp)
                                            .rejectpivot
                                            .offset(0 as ::core::ffi::c_int as isize);
                                        *fresh2 += 1;
                                        *(*lp).rejectpivot.offset(
                                            *(*lp)
                                                .rejectpivot
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as isize,
                                        ) = rownr;
                                        if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong {
                                            report(
                                                lp,
                                                5 as ::core::ffi::c_int,
                                                b"...trying to find another pivot row!\n\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    } else if dualphase1 as ::core::ffi::c_int != 0
                                        && dualoffset
                                            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        current_block_200 = 2616667235040759262;
                                        break;
                                    } else {
                                        current_block_200 = 18425699056680496821;
                                        break;
                                    }
                                } else if inP1extra as ::core::ffi::c_int != 0
                                    && refactRecent(lp) == 0
                                    && is_action((*lp).improve, IMPROVE_INVERSE)
                                        as ::core::ffi::c_int
                                        != 0
                                {
                                    current_block_200 = 11865390570819897086;
                                    break;
                                } else {
                                    current_block_200 = 12065775993741208975;
                                    break;
                                }
                            }
                        }
                        match current_block_200 {
                            12065775993741208975 => {
                                bfpfinal = TRUE as ::core::ffi::c_uchar;
                                if inP1extra as ::core::ffi::c_int != 0
                                    && colnr == 0 as ::core::ffi::c_int
                                    && (*lp).fixedvars > 0 as ::core::ffi::c_int
                                    && is_anti_degen(lp, ANTIDEGEN_FIXEDVARS) as ::core::ffi::c_int
                                        != 0
                                {
                                    report(
                                        lp,
                                        5 as ::core::ffi::c_int,
                                        b"dualloop: Trying to pivot out %d fixed basic variables at iter %.0f\n\0"
                                            as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                    rownr = 0 as ::core::ffi::c_int;
                                    while (*lp).fixedvars > 0 as ::core::ffi::c_int {
                                        rownr = findBasicFixedvar(
                                            lp,
                                            rownr,
                                            TRUE as ::core::ffi::c_uchar,
                                        );
                                        if rownr == 0 as ::core::ffi::c_int {
                                            colnr = 0 as ::core::ffi::c_int;
                                            break;
                                        } else {
                                            colnr = find_rowReplacement(lp, rownr, prow, nzprow);
                                            if colnr > 0 as ::core::ffi::c_int {
                                                theta = 0 as ::core::ffi::c_int
                                                    as ::core::ffi::c_double;
                                                performiteration(
                                                    lp,
                                                    rownr,
                                                    colnr,
                                                    theta,
                                                    TRUE as ::core::ffi::c_uchar,
                                                    FALSE as ::core::ffi::c_uchar,
                                                    prow,
                                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                                    ::core::ptr::null_mut::<::core::ffi::c_double>(
                                                    ),
                                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                                );
                                                (*lp).fixedvars -= 1;
                                            }
                                        }
                                    }
                                }
                                if inP1extra as ::core::ffi::c_int != 0
                                    && colnr < 0 as ::core::ffi::c_int
                                    && isPrimalFeasible(
                                        lp,
                                        (*lp).epsprimal,
                                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                        ::core::ptr::null_mut::<::core::ffi::c_double>(),
                                    ) == 0
                                {
                                    if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong {
                                        if dualfeasible != 0 {
                                            report(
                                                lp,
                                                5 as ::core::ffi::c_int,
                                                b"The model is primal infeasible and dual feasible.\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        } else {
                                            report(
                                                lp,
                                                5 as ::core::ffi::c_int,
                                                b"The model is primal infeasible and dual unbounded.\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                    set_OF_p1extra(
                                        lp,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                                    );
                                    inP1extra = FALSE as ::core::ffi::c_uchar;
                                    set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
                                    (*lp).spx_status = INFEASIBLE;
                                    (*lp).simplex_mode = SIMPLEX_UNDEFINED;
                                    ok = FALSE;
                                } else if inP1extra != 0 {
                                    if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong {
                                        report(
                                            lp,
                                            4 as ::core::ffi::c_int,
                                            b"Found feasibility by dual simplex after    %10.0f iter.\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                        if (*lp).usermessage.is_some()
                                            && (*lp).msgmask & MSG_LPFEASIBLE != 0
                                        {
                                            (*lp).usermessage.expect("non-null function pointer")(
                                                lp,
                                                (*lp).msghandle,
                                                MSG_LPFEASIBLE,
                                            );
                                        }
                                    }
                                    set_OF_p1extra(
                                        lp,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                                    );
                                    inP1extra = FALSE as ::core::ffi::c_uchar;
                                    set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
                                    if (*lp).simplex_strategy & SIMPLEX_DUAL_PRIMAL != 0
                                        && (*lp).fixedvars == 0 as ::core::ffi::c_int
                                    {
                                        (*lp).spx_status = SWITCH_TO_PRIMAL;
                                    }
                                    changedphase = TRUE as ::core::ffi::c_uchar;
                                } else {
                                    (*lp).simplex_mode = SIMPLEX_Phase2_DUAL;
                                    if (*lp).fixedvars > 0 as ::core::ffi::c_int
                                        && (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                    {
                                        report(
                                            lp,
                                            5 as ::core::ffi::c_int,
                                            b"Found dual solution with %d fixed slack variables left basic.\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    }
                                    colnr = 0 as ::core::ffi::c_int;
                                    if dualoffset
                                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        || (*lp).bb_level <= 1 as ::core::ffi::c_int
                                        || (*lp).improve & IMPROVE_BBSIMPLEX != 0
                                        || (*lp).bb_rule & NODE_RCOSTFIXING != 0
                                    {
                                        set_action(&raw mut (*lp).piv_strategy, PRICE_FORCEFULL);
                                        colnr = colprim(
                                            lp,
                                            drow,
                                            nzdrow,
                                            FALSE as ::core::ffi::c_uchar,
                                            1 as ::core::ffi::c_int,
                                            &raw mut candidatecount,
                                            FALSE as ::core::ffi::c_uchar,
                                            ::core::ptr::null_mut::<::core::ffi::c_double>(),
                                        );
                                        clear_action(&raw mut (*lp).piv_strategy, PRICE_FORCEFULL);
                                        if dualoffset
                                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            && colnr > 0 as ::core::ffi::c_int
                                        {
                                            (*lp).spx_status = LOSTFEAS;
                                            if (*lp).total_iter == 0 as ::core::ffi::c_longlong {
                                                report(
                                                    lp,
                                                    5 as ::core::ffi::c_int,
                                                    b"Recovering lost dual feasibility at iter %10.0f.\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            }
                                            break;
                                        }
                                    }
                                    if colnr == 0 as ::core::ffi::c_int {
                                        (*lp).spx_status = OPTIMAL;
                                    } else {
                                        (*lp).spx_status = SWITCH_TO_PRIMAL;
                                        if (*lp).total_iter == 0 as ::core::ffi::c_longlong {
                                            report(
                                                lp,
                                                5 as ::core::ffi::c_int,
                                                b"Use primal simplex for finalization at iter  %10.0f.\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                    if (*lp).total_iter == 0 as ::core::ffi::c_longlong
                                        && (*lp).spx_status == OPTIMAL
                                    {
                                        report(
                                            lp,
                                            5 as ::core::ffi::c_int,
                                            b"Optimal solution with dual simplex at iter   %10.0f.\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    }
                                }
                                if changedphase == 0 {
                                    break;
                                }
                            }
                            18425699056680496821 => {
                                if (*lp).spx_status == RUNNING {
                                    if xviolated < (*lp).epspivot {
                                        if (*lp).bb_trace as ::core::ffi::c_int != 0
                                            || (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                        {
                                            report(
                                                lp,
                                                4 as ::core::ffi::c_int,
                                                b"The model is primal optimal, but marginally infeasible.\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                        (*lp).spx_status = OPTIMAL;
                                        break;
                                    } else {
                                        (*lp).spx_status = INFEASIBLE;
                                        if (*lp).spx_trace as ::core::ffi::c_int != 0
                                            && (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                            || (*lp).bb_trace as ::core::ffi::c_int != 0
                                                && (*lp).bb_totalnodes
                                                    > 0 as ::core::ffi::c_longlong
                                        {
                                            report(
                                                lp,
                                                5 as ::core::ffi::c_int,
                                                b"The model is primal infeasible.\n\0" as *const u8
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                    }
                                }
                                ok = FALSE;
                                break;
                            }
                            2616667235040759262 => {
                                (*lp).spx_status = LOSTFEAS;
                                if (*lp).spx_trace as ::core::ffi::c_int != 0
                                    && (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong
                                    || (*lp).bb_trace as ::core::ffi::c_int != 0
                                        && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
                                {
                                    report(
                                        lp,
                                        5 as ::core::ffi::c_int,
                                        b"dualloop: Model lost dual feasibility.\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                                ok = FALSE;
                                break;
                            }
                            11865390570819897086 => {
                                set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
                                bfpfinal = TRUE as ::core::ffi::c_uchar;
                            }
                            _ => {}
                        }
                        if is_action((*lp).spx_action, ACTION_ITERATE) != 0 {
                            lastnr = *(*lp).var_basic.offset(rownr as isize);
                            if refactRecent(lp) as ::core::ffi::c_int == AUTOMATIC {
                                minitcount = 0 as ::core::ffi::c_int;
                            } else if minitcount > MAX_MINITUPDATES {
                                recompute_solution(lp, INITSOL_USEZERO as ::core::ffi::c_uchar);
                                minitcount = 0 as ::core::ffi::c_int;
                            }
                            minit = performiteration(
                                lp,
                                rownr,
                                colnr,
                                theta,
                                primal,
                                (stallaccept as ::core::ffi::c_int != AUTOMATIC)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uchar,
                                prow,
                                nzprow,
                                pcol,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                boundswaps,
                            );
                            if (*lp).is_strongbranch == 0
                                && (*lp).solutioncount >= 1 as ::core::ffi::c_int
                                && (*lp).spx_perturbed == 0
                                && inP1extra == 0
                                && bb_better(lp, OF_WORKING, OF_TEST_WE) as ::core::ffi::c_int != 0
                            {
                                (*lp).spx_status = FATHOMED;
                                ok = FALSE;
                                break;
                            } else {
                                if minit as ::core::ffi::c_int != ITERATE_MAJORMAJOR {
                                    minitcount += 1;
                                }
                                if longsteps as ::core::ffi::c_int != 0
                                    && dualphase1 as ::core::ffi::c_int != 0
                                    && inP1extra == 0
                                {
                                    dualfeasible = isDualFeasible(
                                        lp,
                                        (*lp).epsprimal,
                                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                        dualinfeasibles,
                                        ::core::ptr::null_mut::<::core::ffi::c_double>(),
                                    );
                                    if dualfeasible != 0 {
                                        dualphase1 = FALSE as ::core::ffi::c_uchar;
                                        changedphase = TRUE as ::core::ffi::c_uchar;
                                        (*lp).simplex_mode = SIMPLEX_Phase2_DUAL;
                                    }
                                }
                                if minit as ::core::ffi::c_int == ITERATE_MAJORMAJOR
                                    && lastnr <= (*lp).rows
                                    && is_fixedvar(lp, lastnr) as ::core::ffi::c_int != 0
                                {
                                    (*lp).fixedvars -= 1;
                                }
                            }
                        }
                        if (*lp)
                            .bfp_mustrefactorize
                            .expect("non-null function pointer")(lp)
                            != 0
                        {
                            if invert(lp, INITSOL_USEZERO as ::core::ffi::c_uchar, bfpfinal) != 0 {
                                bfpfinal = FALSE as ::core::ffi::c_uchar;
                                minit = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
                            } else {
                                (*lp).spx_status = SINGULAR_BASIS;
                            }
                        }
                    }
                }
            }
        }
    }
    stallMonitor_finish(lp);
    multi_free(&raw mut (*lp).longsteps);
    if !(prow as *mut ::core::ffi::c_void).is_null() {
        free(prow as *mut ::core::ffi::c_void);
        prow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(nzprow as *mut ::core::ffi::c_void).is_null() {
        free(nzprow as *mut ::core::ffi::c_void);
        nzprow = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(pcol as *mut ::core::ffi::c_void).is_null() {
        free(pcol as *mut ::core::ffi::c_void);
        pcol = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return ok;
}
#[export_name="honest_lpsolve_spx_run"]
pub unsafe extern "C" fn spx_run(
    mut lp: *mut lprec,
    mut validInvB: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut singular_count: ::core::ffi::c_int = 0;
    let mut lost_feas_count: ::core::ffi::c_int = 0;
    let mut infeasibles: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut boundflip_count: *mut ::core::ffi::c_int =
        ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut primalfeasible: ::core::ffi::c_uchar = 0;
    let mut dualfeasible: ::core::ffi::c_uchar = 0;
    let mut lost_feas_state: ::core::ffi::c_uchar = 0;
    let mut isbb: ::core::ffi::c_uchar = 0;
    let mut primaloffset: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut dualoffset: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*lp).current_iter = 0 as ::core::ffi::c_longlong;
    (*lp).current_bswap = 0 as ::core::ffi::c_longlong;
    (*lp).spx_status = RUNNING;
    (*lp).bb_status = (*lp).spx_status;
    (*lp).P1extraDim = 0 as ::core::ffi::c_int;
    set_OF_p1extra(lp, 0 as ::core::ffi::c_int as ::core::ffi::c_double);
    singular_count = 0 as ::core::ffi::c_int;
    lost_feas_count = 0 as ::core::ffi::c_int;
    lost_feas_state = FALSE as ::core::ffi::c_uchar;
    (*lp).simplex_mode = SIMPLEX_DYNAMIC;
    (*lp).fixedvars = 0 as ::core::ffi::c_int;
    (*lp).boundedvars = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        j = *(*lp).var_basic.offset(i as isize);
        if j <= (*lp).rows && is_fixedvar(lp, j) as ::core::ffi::c_int != 0 {
            (*lp).fixedvars += 1;
        }
        if *(*lp).upbo.offset(i as isize) < (*lp).infinite
            && *(*lp).upbo.offset(i as isize) > (*lp).epsprimal
        {
            (*lp).boundedvars += 1;
        }
        i += 1;
    }
    while i <= (*lp).sum {
        if *(*lp).upbo.offset(i as isize) < (*lp).infinite
            && *(*lp).upbo.offset(i as isize) > (*lp).epsprimal
        {
            (*lp).boundedvars += 1;
        }
        i += 1;
    }
    isbb = (MIP_count(lp) > 0 as ::core::ffi::c_int && (*lp).bb_level > 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if is_action((*lp).spx_action, ACTION_REINVERT) != 0 {
        if isbb as ::core::ffi::c_int != 0
            && (*(*lp).bb_bounds).nodessolved == 0 as ::core::ffi::c_int
        {
            recompute_solution(lp, INITSOL_SHIFTZERO as ::core::ffi::c_uchar);
        } else {
            i = if is_action((*lp).spx_action, 2 as ::core::ffi::c_int) as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            invert(lp, i as ::core::ffi::c_uchar, TRUE as ::core::ffi::c_uchar);
        }
    } else if is_action((*lp).spx_action, ACTION_REBASE) != 0 {
        recompute_solution(lp, INITSOL_SHIFTZERO as ::core::ffi::c_uchar);
    }
    if is_action((*lp).improve, IMPROVE_DUALFEAS) as ::core::ffi::c_int != 0
        || (*lp).rows == 0 as ::core::ffi::c_int
    {
        boundflip_count = &raw mut i;
    } else {
        boundflip_count = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    while (*lp).spx_status == RUNNING {
        dualfeasible = (isbb as ::core::ffi::c_int != 0
            || isDualFeasible(
                lp,
                (*lp).epsprimal,
                boundflip_count,
                infeasibles as *mut ::core::ffi::c_int,
                &raw mut dualoffset,
            ) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if is_action((*lp).spx_action, ACTION_RECOMPUTE) != 0 {
            recompute_solution(lp, INITSOL_USEZERO as ::core::ffi::c_uchar);
        }
        primalfeasible = isPrimalFeasible(
            lp,
            (*lp).epsprimal,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut primaloffset,
        );
        if userabort(lp, -(1 as ::core::ffi::c_int)) != 0 {
            break;
        }
        if (*lp).spx_trace != 0 {
            if primalfeasible != 0 {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"Start at primal feasible basis\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else if dualfeasible != 0 {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"Start at dual feasible basis\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else if lost_feas_count > 0 as ::core::ffi::c_int {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"Continuing at infeasible basis\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"Start at infeasible basis\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        if (*lp).simplex_strategy & SIMPLEX_Phase1_DUAL == 0 as ::core::ffi::c_int
            || MIP_count(lp) > 0 as ::core::ffi::c_int
                && (*lp).total_iter == 0 as ::core::ffi::c_longlong
                && is_presolve(lp, PRESOLVE_REDUCEMIP) as ::core::ffi::c_int != 0
        {
            if lost_feas_state == 0
                && primalfeasible as ::core::ffi::c_int != 0
                && (*lp).simplex_strategy & SIMPLEX_Phase2_DUAL > 0 as ::core::ffi::c_int
            {
                (*lp).spx_status = SWITCH_TO_DUAL;
            } else {
                primloop(lp, primalfeasible, 0.0f64);
            }
            if (*lp).spx_status == SWITCH_TO_DUAL {
                dualloop(
                    lp,
                    TRUE as ::core::ffi::c_uchar,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    0.0f64,
                );
            }
        } else {
            if lost_feas_state == 0
                && primalfeasible as ::core::ffi::c_int != 0
                && (*lp).simplex_strategy & SIMPLEX_Phase2_PRIMAL > 0 as ::core::ffi::c_int
            {
                (*lp).spx_status = SWITCH_TO_PRIMAL;
            } else {
                dualloop(
                    lp,
                    dualfeasible,
                    infeasibles as *mut ::core::ffi::c_int,
                    dualoffset,
                );
            }
            if (*lp).spx_status == SWITCH_TO_PRIMAL {
                primloop(lp, TRUE as ::core::ffi::c_uchar, 0.0f64);
            }
        }
        i = (*lp).spx_status;
        primalfeasible = (i == OPTIMAL) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if primalfeasible as ::core::ffi::c_int != 0 || i == UNBOUNDED {
            break;
        }
        if i == INFEASIBLE && is_anti_degen(lp, ANTIDEGEN_INFEASIBLE) as ::core::ffi::c_int != 0
            || i == LOSTFEAS && is_anti_degen(lp, ANTIDEGEN_LOSTFEAS) as ::core::ffi::c_int != 0
            || i == NUMFAILURE && is_anti_degen(lp, ANTIDEGEN_NUMFAILURE) as ::core::ffi::c_int != 0
            || i == DEGENERATE && is_anti_degen(lp, ANTIDEGEN_STALLING) as ::core::ffi::c_int != 0
        {
            if (*lp).bb_level <= 1 as ::core::ffi::c_int
                || is_anti_degen(lp, ANTIDEGEN_DURINGBB) as ::core::ffi::c_int != 0
            {
                break;
            }
            if (*lp).bb_level > 1 as ::core::ffi::c_int && i == INFEASIBLE {
                break;
            }
        }
        if (*lp).spx_status == SINGULAR_BASIS {
            lost_feas_state = FALSE as ::core::ffi::c_uchar;
            singular_count += 1;
            if singular_count >= DEF_MAXSINGULARITIES {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"spx_run: Failure due to too many singular bases.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                (*lp).spx_status = NUMFAILURE;
                break;
            } else {
                if (*lp).spx_trace as ::core::ffi::c_int != 0 || (*lp).verbose > DETAILED {
                    report(
                        lp,
                        4 as ::core::ffi::c_int,
                        b"spx_run: Singular basis; attempting to recover.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                (*lp).spx_status = RUNNING;
            }
        } else {
            lost_feas_state =
                ((*lp).spx_status == LOSTFEAS) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            if lost_feas_state != 0 {
                lost_feas_count += 1;
                if lost_feas_count < DEF_MAXSINGULARITIES {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"spx_run: Recover lost feasibility at iter  %10.0f.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    (*lp).spx_status = RUNNING;
                } else {
                    report(
                        lp,
                        3 as ::core::ffi::c_int,
                        b"spx_run: Lost feasibility %d times - iter %10.0f and %9.0f nodes.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    (*lp).spx_status = NUMFAILURE;
                }
            }
        }
    }
    (*lp).total_iter += (*lp).current_iter;
    (*lp).current_iter = 0 as ::core::ffi::c_longlong;
    (*lp).total_bswap += (*lp).current_bswap;
    (*lp).current_bswap = 0 as ::core::ffi::c_longlong;
    if !(infeasibles as *mut ::core::ffi::c_void).is_null() {
        free(infeasibles as *mut ::core::ffi::c_void);
        infeasibles = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return (*lp).spx_status;
}
#[export_name="honest_lpsolve_make_lag"]
pub unsafe extern "C" fn make_lag(mut lpserver: *mut lprec) -> *mut lprec {
    let mut i: ::core::ffi::c_int = 0;
    let mut hlp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut ret: ::core::ffi::c_uchar = 0;
    let mut duals: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    hlp = make_lp(0 as ::core::ffi::c_int, (*lpserver).columns);
    if !hlp.is_null() {
        set_sense(hlp, is_maxim(lpserver));
        (*hlp).lag_bound = (*lpserver).bb_limitOF;
        i = 1 as ::core::ffi::c_int;
        while i <= (*lpserver).columns {
            set_mat(
                hlp,
                0 as ::core::ffi::c_int,
                i,
                get_mat(lpserver, 0 as ::core::ffi::c_int, i),
            );
            if is_binary(lpserver, i) != 0 {
                set_binary(hlp, i, TRUE as ::core::ffi::c_uchar);
            } else {
                set_int(hlp, i, is_int(lpserver, i));
                set_bounds(hlp, i, get_lowbo(lpserver, i), get_upbo(lpserver, i));
            }
            i += 1;
        }
        (*hlp).matL = (*lpserver).matA;
        inc_lag_space(hlp, (*lpserver).rows, TRUE as ::core::ffi::c_uchar);
        ret = get_ptr_sensitivity_rhs(
            hlp,
            &raw mut duals,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
        );
        i = 1 as ::core::ffi::c_int;
        while i <= (*lpserver).rows {
            *(*hlp).lag_con_type.offset(i as isize) = get_constr_type(lpserver, i);
            *(*hlp).lag_rhs.offset(i as isize) = *(*lpserver).orig_rhs.offset(i as isize);
            *(*hlp).lambda.offset(i as isize) = if ret as ::core::ffi::c_int != 0 {
                *duals.offset((i - 1 as ::core::ffi::c_int) as isize)
            } else {
                0.0f64
            };
            i += 1;
        }
    }
    return hlp;
}
#[export_name="honest_lpsolve_heuristics"]
pub unsafe extern "C" fn heuristics(
    mut lp: *mut lprec,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hlp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut status: ::core::ffi::c_int = PROCFAIL;
    if (*lp).bb_level > 1 as ::core::ffi::c_int {
        return status;
    }
    status = RUNNING;
    (*lp).bb_limitOF = if is_maxim(lp) as ::core::ffi::c_int != 0
        && -(*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        --(*lp).infinite
    } else {
        -(*lp).infinite
    };
    if FALSE != 0 && (*lp).int_vars > 0 as ::core::ffi::c_int {
        hlp = make_lag(lp);
        status = solve(hlp);
        (*lp).bb_heuristicOF = *(*hlp)
            .best_solution
            .offset(0 as ::core::ffi::c_int as isize);
        (*hlp).matL = ::core::ptr::null_mut::<MATrec>();
        delete_lp(hlp);
    }
    (*lp).timeheuristic = timeNow();
    return status;
}
#[export_name="honest_lpsolve_lag_solve"]
pub unsafe extern "C" fn lag_solve(
    mut lp: *mut lprec,
    mut start_bound: ::core::ffi::c_double,
    mut num_iter: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut citer: ::core::ffi::c_int = 0;
    let mut nochange: ::core::ffi::c_int = 0;
    let mut oldpresolve: ::core::ffi::c_int = 0;
    let mut LagFeas: ::core::ffi::c_uchar = 0;
    let mut AnyFeas: ::core::ffi::c_uchar = 0;
    let mut Converged: ::core::ffi::c_uchar = 0;
    let mut same_basis: ::core::ffi::c_uchar = 0;
    let mut OrigObj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut ModObj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut SubGrad: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut BestFeasSol: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut Zub: ::core::ffi::c_double = 0.;
    let mut Zlb: ::core::ffi::c_double = 0.;
    let mut Znow: ::core::ffi::c_double = 0.;
    let mut Zprev: ::core::ffi::c_double = 0.;
    let mut Zbest: ::core::ffi::c_double = 0.;
    let mut rhsmod: ::core::ffi::c_double = 0.;
    let mut hold: ::core::ffi::c_double = 0.;
    let mut Phi: ::core::ffi::c_double = 0.;
    let mut StepSize: ::core::ffi::c_double = 0.0f64;
    let mut SqrsumSubGrad: ::core::ffi::c_double = 0.;
    if (*lp).spx_status != OPTIMAL {
        (*lp).lag_status = NOTRUN;
        return (*lp).lag_status;
    }
    if allocREAL(
        lp,
        &raw mut OrigObj,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    ) == 0
        || allocREAL(
            lp,
            &raw mut ModObj,
            (*lp).columns + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut SubGrad,
            get_Lrows(lp) + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut BestFeasSol,
            (*lp).sum + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0
    {
        (*lp).lag_status = NOMEMORY;
        return (*lp).lag_status;
    }
    (*lp).lag_status = RUNNING;
    oldpresolve = (*lp).do_presolve;
    (*lp).do_presolve = PRESOLVE_NONE;
    push_basis(
        lp,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    Zlb = *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize);
    Zub = start_bound;
    Zbest = Zub;
    Znow = Zlb;
    Zprev = (*lp).infinite;
    rhsmod = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    Phi = DEF_LAGCONTRACT;
    LagFeas = FALSE as ::core::ffi::c_uchar;
    Converged = FALSE as ::core::ffi::c_uchar;
    AnyFeas = FALSE as ::core::ffi::c_uchar;
    citer = 0 as ::core::ffi::c_int;
    nochange = 0 as ::core::ffi::c_int;
    get_row(lp, 0 as ::core::ffi::c_int, OrigObj);
    *OrigObj.offset(0 as ::core::ffi::c_int as isize) = get_rh(lp, 0 as ::core::ffi::c_int);
    i = 1 as ::core::ffi::c_int;
    while i <= get_Lrows(lp) {
        *(*lp).lambda.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    loop {
        if !((*lp).lag_status == RUNNING && citer < num_iter) {
            current_block = 7739940392431776979;
            break;
        }
        citer += 1;
        LagFeas = TRUE as ::core::ffi::c_uchar;
        Converged = TRUE as ::core::ffi::c_uchar;
        SqrsumSubGrad = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i = 1 as ::core::ffi::c_int;
        while i <= get_Lrows(lp) {
            hold = *(*lp).lag_rhs.offset(i as isize);
            j = 1 as ::core::ffi::c_int;
            while j <= (*lp).columns {
                hold -= mat_getitem((*lp).matL, i, j)
                    * *(*lp).best_solution.offset(((*lp).rows + j) as isize);
                j += 1;
            }
            if LagFeas != 0 {
                if *(*lp).lag_con_type.offset(i as isize) == EQ {
                    if fabs(hold) > (*lp).epsprimal {
                        LagFeas = FALSE as ::core::ffi::c_uchar;
                    }
                } else if hold < -(*lp).epsprimal {
                    LagFeas = FALSE as ::core::ffi::c_uchar;
                }
            }
            if Converged as ::core::ffi::c_int != 0
                && fabs(
                    (hold - *SubGrad.offset(i as isize))
                        / (1.0f64 + fabs(*SubGrad.offset(i as isize))),
                ) > (*lp).lag_accept
            {
                Converged = FALSE as ::core::ffi::c_uchar;
            }
            *SubGrad.offset(i as isize) = hold;
            SqrsumSubGrad += hold * hold;
            i += 1;
        }
        SqrsumSubGrad = sqrt(SqrsumSubGrad);
        Converged = (Converged as ::core::ffi::c_int & LagFeas as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        if Converged != 0 {
            current_block = 7739940392431776979;
            break;
        }
        Znow = *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize) - rhsmod;
        if Znow > Zub {
            Phi *= DEF_LAGCONTRACT;
            StepSize *= (Zub - Zlb) / (Znow - Zlb);
        } else {
            StepSize = Phi
                * (2 as ::core::ffi::c_int as ::core::ffi::c_double - DEF_LAGCONTRACT)
                * (Zub - Znow)
                / SqrsumSubGrad;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= get_Lrows(lp) {
            *(*lp).lambda.offset(i as isize) += StepSize * *SubGrad.offset(i as isize);
            if *(*lp).lag_con_type.offset(i as isize) != EQ
                && *(*lp).lambda.offset(i as isize)
                    > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                if Znow < Zub {
                    *(*lp).lambda.offset(i as isize) =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            }
            i += 1;
        }
        if LagFeas as ::core::ffi::c_int != 0 && Znow < Zbest {
            memcpy(
                BestFeasSol as *mut ::core::ffi::c_void,
                (*lp).best_solution as *const ::core::ffi::c_void,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            hold = *OrigObj.offset(0 as ::core::ffi::c_int as isize);
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                hold += *(*lp).best_solution.offset(((*lp).rows + i) as isize)
                    * *OrigObj.offset(i as isize);
                i += 1;
            }
            *BestFeasSol.offset(0 as ::core::ffi::c_int as isize) = hold;
            if (*lp).lag_trace != 0 {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"lag_solve: Improved feasible solution at iteration %d of %g\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            Zbest = Znow;
            AnyFeas = TRUE as ::core::ffi::c_uchar;
            nochange = 0 as ::core::ffi::c_int;
        } else if Znow == Zprev {
            nochange += 1;
            if nochange > LAG_SINGULARLIMIT {
                Phi *= 0.5f64;
                nochange = 0 as ::core::ffi::c_int;
            }
        }
        Zprev = Znow;
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i = 1 as ::core::ffi::c_int;
            while i <= get_Lrows(lp) {
                hold += *(*lp).lambda.offset(i as isize) * mat_getitem((*lp).matL, i, j);
                i += 1;
            }
            *ModObj.offset(j as isize) = *OrigObj.offset(j as isize)
                - (if is_maxim(lp) as ::core::ffi::c_int != 0
                    && hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -hold
                } else {
                    hold
                });
            set_mat(lp, 0 as ::core::ffi::c_int, j, *ModObj.offset(j as isize));
            j += 1;
        }
        rhsmod = if is_maxim(lp) as ::core::ffi::c_int != 0
            && get_rh(lp, 0 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -get_rh(lp, 0 as ::core::ffi::c_int)
        } else {
            get_rh(lp, 0 as ::core::ffi::c_int)
        };
        i = 1 as ::core::ffi::c_int;
        while i <= get_Lrows(lp) {
            rhsmod += *(*lp).lambda.offset(i as isize) * *(*lp).lag_rhs.offset(i as isize);
            i += 1;
        }
        if (*lp).lag_trace != 0 {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"Zub: %10g Zlb: %10g Stepsize: %10g Phi: %10g Feas %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            i = 1 as ::core::ffi::c_int;
            while i <= get_Lrows(lp) {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"%3d SubGrad %10g lambda %10g\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                i += 1;
            }
            if (*lp).sum < 20 as ::core::ffi::c_int {
                print_lp(lp);
            }
        }
        i = spx_solve(lp);
        if (*lp).spx_status == UNBOUNDED {
            if (*lp).lag_trace != 0 {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"lag_solve: Unbounded solution encountered with this OF:\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                i = 1 as ::core::ffi::c_int;
                while i <= (*lp).columns {
                    report(
                        lp,
                        4 as ::core::ffi::c_int,
                        b"%18.12g \0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    i += 1;
                }
            }
            current_block = 6542917025277487033;
            break;
        } else {
            if (*lp).spx_status == NUMFAILURE
                || (*lp).spx_status == PROCFAIL
                || (*lp).spx_status == USERABORT
                || (*lp).spx_status == TIMEOUT
                || (*lp).spx_status == INFEASIBLE
            {
                (*lp).lag_status = (*lp).spx_status;
            }
            same_basis = compare_basis(lp);
            if LagFeas as ::core::ffi::c_int != 0 && same_basis == 0 {
                pop_basis(lp, FALSE as ::core::ffi::c_uchar);
                push_basis(
                    lp,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                );
                Phi *= DEF_LAGCONTRACT;
            }
            if (*lp).lag_trace != 0 {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"lag_solve: Simplex status code %d, same basis %s\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                print_solution(lp, 1 as ::core::ffi::c_int);
            }
        }
    }
    match current_block {
        7739940392431776979 => {
            if AnyFeas != 0 {
                (*lp).lag_bound = if is_maxim(lp) as ::core::ffi::c_int != 0
                    && Zbest != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -Zbest
                } else {
                    Zbest
                };
                i = 0 as ::core::ffi::c_int;
                while i <= (*lp).sum {
                    *(*lp).solution.offset(i as isize) = *BestFeasSol.offset(i as isize);
                    i += 1;
                }
                transfer_solution(lp, TRUE as ::core::ffi::c_uchar);
                if is_maxim(lp) == 0 {
                    i = 1 as ::core::ffi::c_int;
                    while i <= get_Lrows(lp) {
                        *(*lp).lambda.offset(i as isize) = if fabs(*(*lp).lambda.offset(i as isize))
                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
                        } else {
                            -*(*lp).lambda.offset(i as isize)
                        };
                        i += 1;
                    }
                }
            }
        }
        _ => {}
    }
    if citer >= num_iter {
        if AnyFeas != 0 {
            (*lp).lag_status = FEASFOUND;
        } else {
            (*lp).lag_status = NOFEASFOUND;
        }
    } else {
        (*lp).lag_status = (*lp).spx_status;
    }
    if (*lp).lag_status == OPTIMAL {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"\nLagrangean convergence achieved in %d iterations\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        i = check_solution(
            lp,
            (*lp).columns,
            (*lp).best_solution,
            (*lp).orig_upbo,
            (*lp).orig_lowbo,
            (*lp).epssolution,
        );
    } else {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"\nUnsatisfactory convergence achieved over %d Lagrangean iterations.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if AnyFeas != 0 {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"The best feasible Lagrangean objective function value was %g\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        set_mat(lp, 0 as ::core::ffi::c_int, i, *OrigObj.offset(i as isize));
        i += 1;
    }
    if !(BestFeasSol as *mut ::core::ffi::c_void).is_null() {
        free(BestFeasSol as *mut ::core::ffi::c_void);
        BestFeasSol = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(SubGrad as *mut ::core::ffi::c_void).is_null() {
        free(SubGrad as *mut ::core::ffi::c_void);
        SubGrad = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(OrigObj as *mut ::core::ffi::c_void).is_null() {
        free(OrigObj as *mut ::core::ffi::c_void);
        OrigObj = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(ModObj as *mut ::core::ffi::c_void).is_null() {
        free(ModObj as *mut ::core::ffi::c_void);
        ModObj = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    pop_basis(lp, FALSE as ::core::ffi::c_uchar);
    (*lp).do_presolve = oldpresolve;
    return (*lp).lag_status;
}
#[export_name="honest_lpsolve_spx_solve"]
pub unsafe extern "C" fn spx_solve(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut status: ::core::ffi::c_int = 0;
    let mut iprocessed: ::core::ffi::c_uchar = 0;
    (*lp).total_iter = 0 as ::core::ffi::c_longlong;
    (*lp).total_bswap = 0 as ::core::ffi::c_longlong;
    (*lp).perturb_count = 0 as ::core::ffi::c_int;
    (*lp).bb_maxlevel = 1 as ::core::ffi::c_int;
    (*lp).bb_totalnodes = 0 as ::core::ffi::c_longlong;
    (*lp).bb_improvements = 0 as ::core::ffi::c_int;
    (*lp).bb_strongbranches = 0 as ::core::ffi::c_int;
    (*lp).is_strongbranch = FALSE;
    (*lp).bb_level = 0 as ::core::ffi::c_int;
    (*lp).bb_solutionlevel = 0 as ::core::ffi::c_int;
    *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize) =
        if is_maxim(lp) as ::core::ffi::c_int != 0
            && (*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -(*lp).infinite
        } else {
            (*lp).infinite
        };
    if !(*lp).invB.is_null() {
        (*lp).bfp_restart.expect("non-null function pointer")(lp);
    }
    (*lp).spx_status = presolve(lp);
    if (*lp).spx_status == PRESOLVED {
        status = (*lp).spx_status;
        current_block = 15081087785955092192;
    } else if (*lp).spx_status != RUNNING {
        current_block = 3665245624154001931;
    } else {
        iprocessed = ((*lp).wasPreprocessed == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if preprocess(lp) == 0
            || userabort(lp, -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int != 0
        {
            current_block = 3665245624154001931;
        } else if mat_validate((*lp).matA) != 0 {
            (*lp).solutioncount = 0 as ::core::ffi::c_int;
            (*lp).real_solution = (*lp).infinite;
            set_action(&raw mut (*lp).spx_action, ACTION_REBASE | ACTION_REINVERT);
            (*lp).bb_break = FALSE as ::core::ffi::c_uchar;
            status = run_BB(lp);
            if iprocessed != 0 {
                postprocess(lp);
            }
            current_block = 15081087785955092192;
        } else {
            if (*lp).bb_trace as ::core::ffi::c_int != 0
                || (*lp).spx_trace as ::core::ffi::c_int != 0
            {
                report(
                    lp,
                    1 as ::core::ffi::c_int,
                    b"spx_solve: The current LP seems to be invalid\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            (*lp).spx_status = NUMFAILURE;
            current_block = 3665245624154001931;
        }
    }
    match current_block {
        15081087785955092192 => {
            if postsolve(lp, status) == 0 {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"spx_solve: Failure during postsolve.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    (*lp).timeend = timeNow();
    if (*lp).lag_status != RUNNING && !(*lp).invB.is_null() {
        let mut itemp: ::core::ffi::c_int = 0;
        let mut test: ::core::ffi::c_double = 0.;
        itemp = (*lp).bfp_nonzeros.expect("non-null function pointer")(
            lp,
            TRUE as ::core::ffi::c_uchar,
        );
        test = 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        if (*lp).total_iter > 0 as ::core::ffi::c_longlong {
            test *= (*lp).total_bswap as ::core::ffi::c_double
                / (*lp).total_iter as ::core::ffi::c_double;
        }
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"\n \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"MEMO: lp_solve version %d.%d.%d.%d for %d bit OS, with %d bit LPSREAL variables.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"      In the total iteration count %.0f, %.0f (%.1f%%) were bound flips.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"      There were %d refactorizations, %d triggered by time and %d by density.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"       ... on average %.1f major pivots per refactorization.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"      The largest [%s] fact(B) had %d NZ entries, %.1fx largest basis.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if (*lp).perturb_count > 0 as ::core::ffi::c_int {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"      The bounds were relaxed via perturbations %d times.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if MIP_count(lp) > 0 as ::core::ffi::c_int {
            if (*lp).bb_solutionlevel > 0 as ::core::ffi::c_int {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"      The maximum B&B level was %d, %.1fx MIP order, %d at the optimal solution.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"      The maximum B&B level was %d, %.1fx MIP order, with %.0f nodes explored.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if GUB_count(lp) > 0 as ::core::ffi::c_int {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"      %d general upper-bounded (GUB) structures were employed during B&B.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"      The constraint matrix inf-norm is %g, with a dynamic range of %g.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"      Time to load data was %.3f seconds, presolve used %.3f seconds,\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"       ... %.3f seconds in simplex solver, in total %.3f seconds.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return (*lp).spx_status;
}
#[export_name="honest_lpsolve_lin_solve"]
pub unsafe extern "C" fn lin_solve(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = NOTRUN;
    (*lp).lag_status = NOTRUN;
    if (*lp).columns == 0 as ::core::ffi::c_int {
        default_basis(lp);
        (*lp).spx_status = NOTRUN;
        return (*lp).spx_status;
    }
    unset_OF_p1extra(lp);
    free_duals(lp);
    if !((*lp).drow as *mut ::core::ffi::c_void).is_null() {
        free((*lp).drow as *mut ::core::ffi::c_void);
        (*lp).drow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).nzdrow as *mut ::core::ffi::c_void).is_null() {
        free((*lp).nzdrow as *mut ::core::ffi::c_void);
        (*lp).nzdrow = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(*lp).bb_cuttype.is_null() {
        freecuts_BB(lp);
    }
    (*lp).timestart = timeNow();
    (*lp).timeheuristic = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*lp).timepresolved = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*lp).timeend = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if heuristics(lp, AUTOMATIC) != RUNNING {
        return 2 as ::core::ffi::c_int;
    }
    status = spx_solve(lp);
    if get_Lrows(lp) > 0 as ::core::ffi::c_int && (*lp).lag_status == NOTRUN {
        if status == OPTIMAL {
            status = lag_solve(lp, (*lp).bb_heuristicOF, DEF_LAGMAXITERATIONS);
        } else {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"\nCannot do Lagrangean optimization since root model was not solved.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
    (*lp).bb_heuristicOF = if is_maxim(lp) as ::core::ffi::c_int != 0
        && (*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        -(*lp).infinite
    } else {
        (*lp).infinite
    };
    if (*lp).spx_status == OPTIMAL && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong {
        if (*lp).bb_break as ::core::ffi::c_int != 0 && bb_better(lp, OF_DUALLIMIT, OF_TEST_BE) == 0
        {
            (*lp).spx_status = SUBOPTIMAL;
            status = (*lp).spx_status;
        }
    }
    return status;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAXINT32: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OBJ_STEPS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SIMPLEX_UNDEFINED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIMPLEX_Phase1_PRIMAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIMPLEX_Phase1_DUAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIMPLEX_Phase2_PRIMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIMPLEX_Phase2_DUAL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIMPLEX_DYNAMIC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SIMPLEX_DUAL_PRIMAL: ::core::ffi::c_int = SIMPLEX_Phase1_DUAL + SIMPLEX_Phase2_PRIMAL;
pub const PRESOLVE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PRESOLVE_REDUCEMIP: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const INITSOL_SHIFTZERO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INITSOL_USEZERO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTIDEGEN_FIXEDVARS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTIDEGEN_COLUMNCHECK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ANTIDEGEN_STALLING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTIDEGEN_NUMFAILURE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ANTIDEGEN_LOSTFEAS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ANTIDEGEN_INFEASIBLE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const ANTIDEGEN_DYNAMIC: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ANTIDEGEN_DURINGBB: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const ANTIDEGEN_BOUNDFLIP: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const DETAILED: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MSG_LPFEASIBLE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ROWTYPE_EQ: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EQ: ::core::ffi::c_int = ROWTYPE_EQ;
pub const IMPROVE_SOLUTION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IMPROVE_DUALFEAS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IMPROVE_THETAGAP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const IMPROVE_BBSIMPLEX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const IMPROVE_INVERSE: ::core::ffi::c_int = IMPROVE_SOLUTION + IMPROVE_THETAGAP;
pub const ITERATE_MAJORMAJOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ITERATE_MINORMAJOR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ITERATE_MINORRETRY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PRICER_FIRSTINDEX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PRICER_DANTZIG: ::core::ffi::c_int = 1;
pub const PRICER_DEVEX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PRICER_STEEPESTEDGE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PRICE_MULTIPLE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PRICE_ADAPTIVE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PRICE_FORCEFULL: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const NODE_RCOSTFIXING: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const ACTION_REBASE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ACTION_RECOMPUTE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ACTION_REINVERT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ACTION_ITERATE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NOMEMORY: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const NOTRUN: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const OPTIMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SUBOPTIMAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INFEASIBLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UNBOUNDED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DEGENERATE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUMFAILURE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const USERABORT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TIMEOUT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const RUNNING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PRESOLVED: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PROCFAIL: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FEASFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const NOFEASFOUND: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FATHOMED: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SWITCH_TO_PRIMAL: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SWITCH_TO_DUAL: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SINGULAR_BASIS: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const LOSTFEAS: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const OF_WORKING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OF_DUALLIMIT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OF_TEST_BE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OF_TEST_WE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DEF_MAXPIVOTRETRY: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const DEF_MAXSINGULARITIES: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAX_MINITUPDATES: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const LAG_SINGULARLIMIT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MAX_STALLCOUNT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MAX_RULESWITCH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DEF_LAGCONTRACT: ::core::ffi::c_double = 0.90f64;
pub const DEF_LAGMAXITERATIONS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
