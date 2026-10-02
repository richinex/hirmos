use crate::honest_did::lpsolve::runtime::{modf};
use crate::honest_did::lpsolve::runtime::{calloc,free,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
#[repr(C)] pub struct _INVrec{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_crash_basis"]
    fn crash_basis(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_auto_scale"]
    fn auto_scale(lp: *mut lprec) -> ::core::ffi::c_double;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmod(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_swapINT"]
    fn swapINT(item1: *mut ::core::ffi::c_int, item2: *mut ::core::ffi::c_int);
    #[link_name="honest_lpsolve_swapREAL"]
    fn swapREAL(item1: *mut ::core::ffi::c_double, item2: *mut ::core::ffi::c_double);
    #[link_name="honest_lpsolve_restoreINT"]
    fn restoreINT(
        valREAL: ::core::ffi::c_double,
        epsilon: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_isOrigFixed"]
    fn isOrigFixed(lp: *mut lprec, varno: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_countActiveLink"]
    fn countActiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_countInactiveLink"]
    fn countInactiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_firstActiveLink"]
    fn firstActiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_lastActiveLink"]
    fn lastActiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_appendLink"]
    fn appendLink(linkmap: *mut LLrec, newitem: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_setLink"]
    fn setLink(linkmap: *mut LLrec, newitem: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_fillLink"]
    fn fillLink(linkmap: *mut LLrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_nextActiveLink"]
    fn nextActiveLink(linkmap: *mut LLrec, backitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_prevActiveLink"]
    fn prevActiveLink(linkmap: *mut LLrec, forwitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_firstInactiveLink"]
    fn firstInactiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_lastInactiveLink"]
    fn lastInactiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_nextInactiveLink"]
    fn nextInactiveLink(linkmap: *mut LLrec, backitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_removeLink"]
    fn removeLink(linkmap: *mut LLrec, itemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_cloneLink"]
    fn cloneLink(
        sourcemap: *mut LLrec,
        newsize: ::core::ffi::c_int,
        freesource: ::core::ffi::c_uchar,
    ) -> *mut LLrec;
    #[link_name="honest_lpsolve_mat_memopt"]
    fn mat_memopt(
        mat: *mut MATrec,
        rowextra: ::core::ffi::c_int,
        colextra: ::core::ffi::c_int,
        nzextra: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_mapreplace"]
    fn mat_mapreplace(
        mat: *mut MATrec,
        rowmap: *mut LLrec,
        colmap: *mut LLrec,
        insmat: *mut MATrec,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_zerocompact"]
    fn mat_zerocompact(mat: *mut MATrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_rowcompact"]
    fn mat_rowcompact(mat: *mut MATrec, dozeros: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_colcompact"]
    fn mat_colcompact(
        mat: *mut MATrec,
        prev_rows: ::core::ffi::c_int,
        prev_cols: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_mat_collength"]
    fn mat_collength(mat: *mut MATrec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_rowlength"]
    fn mat_rowlength(mat: *mut MATrec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_multrow"]
    fn mat_multrow(mat: *mut MATrec, row_nr: ::core::ffi::c_int, mult: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_mat_setcol"]
    fn mat_setcol(
        mat: *mut MATrec,
        colno: ::core::ffi::c_int,
        count: ::core::ffi::c_int,
        column: *mut ::core::ffi::c_double,
        rowno: *mut ::core::ffi::c_int,
        doscale: ::core::ffi::c_uchar,
        checkrowmode: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_checkcounts"]
    fn mat_checkcounts(
        mat: *mut MATrec,
        rownum: *mut ::core::ffi::c_int,
        colnum: *mut ::core::ffi::c_int,
        freeonexit: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_expandcolumn"]
    fn mat_expandcolumn(
        mat: *mut MATrec,
        colnr: ::core::ffi::c_int,
        column: *mut ::core::ffi::c_double,
        nzlist: *mut ::core::ffi::c_int,
        signedA: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_computemax"]
    fn mat_computemax(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_createUndoLadder"]
    fn createUndoLadder(
        lp: *mut lprec,
        levelitems: ::core::ffi::c_int,
        maxlevels: ::core::ffi::c_int,
    ) -> *mut DeltaVrec;
    #[link_name="honest_lpsolve_incrementUndoLadder"]
    fn incrementUndoLadder(DV: *mut DeltaVrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_freeUndoLadder"]
    fn freeUndoLadder(DV: *mut *mut DeltaVrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_appendUndoPresolve"]
    fn appendUndoPresolve(
        lp: *mut lprec,
        isprimal: ::core::ffi::c_uchar,
        beta: ::core::ffi::c_double,
        colnrDep: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_addUndoPresolve"]
    fn addUndoPresolve(
        lp: *mut lprec,
        isprimal: ::core::ffi::c_uchar,
        colnrElim: ::core::ffi::c_int,
        alpha: ::core::ffi::c_double,
        beta: ::core::ffi::c_double,
        colnrDep: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_clean_SOSgroup"]
    fn clean_SOSgroup(
        group: *mut SOSgroup,
        forceupdatemap: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_free_SOSgroup"]
    fn free_SOSgroup(group: *mut *mut SOSgroup);
    #[link_name="honest_lpsolve_delete_SOSrec"]
    fn delete_SOSrec(group: *mut SOSgroup, sosindex: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_append_SOSrec"]
    fn append_SOSrec(
        SOS: *mut SOSrec,
        size: ::core::ffi::c_int,
        variables: *mut ::core::ffi::c_int,
        weights: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_make_SOSchain"]
    fn make_SOSchain(lp: *mut lprec, forceresort: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_member_updatemap"]
    fn SOS_member_updatemap(group: *mut SOSgroup) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_member_delete"]
    fn SOS_member_delete(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        member: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_infeasible"]
    fn SOS_infeasible(group: *mut SOSgroup, sosindex: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_member_index"]
    fn SOS_member_index(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        member: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_memberships"]
    fn SOS_memberships(group: *mut SOSgroup, column: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_SOS_set_GUB"]
    fn SOS_set_GUB(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        state: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_maxim"]
    fn is_maxim(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_del_constraintex"]
    fn del_constraintex(lp: *mut lprec, rowmap: *mut LLrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_constr_type"]
    fn set_constr_type(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        con_type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_constr_type"]
    fn get_constr_type(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_constr_type"]
    fn is_constr_type(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        mask: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_rh"]
    fn set_rh(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_rh"]
    fn get_rh(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_rh_range"]
    fn set_rh_range(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        deltavalue: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_rh_range"]
    fn get_rh_range(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_add_columnex"]
    fn add_columnex(
        lp: *mut lprec,
        count: ::core::ffi::c_int,
        column: *mut ::core::ffi::c_double,
        rowno: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_del_columnex"]
    fn del_columnex(lp: *mut lprec, colmap: *mut LLrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_mat"]
    fn get_mat(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_mat_byindex"]
    fn get_mat_byindex(
        lp: *mut lprec,
        matindex: ::core::ffi::c_int,
        isrow: ::core::ffi::c_uchar,
        adjustsign: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_nonzeros"]
    fn get_nonzeros(lp: *mut lprec) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_set_unbounded"]
    fn set_unbounded(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_unbounded"]
    fn is_unbounded(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_int"]
    fn is_int(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_is_negative"]
    fn is_negative(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_is_SOS_var"]
    fn is_SOS_var(lp: *mut lprec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_presolveloops"]
    fn get_presolveloops(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_is_presolve"]
    fn is_presolve(lp: *mut lprec, testmask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_Lrows"]
    fn get_Lrows(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_yieldformessages"]
    fn yieldformessages(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_userabort"]
    fn userabort(lp: *mut lprec, message: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_is_chsign"]
    fn is_chsign(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_get_rh_upper"]
    fn get_rh_upper(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_get_rh_lower"]
    fn get_rh_lower(lp: *mut lprec, rownr: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_set_rh_upper"]
    fn set_rh_upper(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_set_rh_lower"]
    fn set_rh_lower(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_MIP_count"]
    fn MIP_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_count"]
    fn SOS_count(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_identify_GUB"]
    fn identify_GUB(lp: *mut lprec, mark: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_is_bb_mode"]
    fn is_bb_mode(lp: *mut lprec, bb_mask: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_varmap_lock"]
    fn varmap_lock(lp: *mut lprec);
    #[link_name="honest_lpsolve_varmap_canunlock"]
    fn varmap_canunlock(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_varmap_compact"]
    fn varmap_compact(lp: *mut lprec, prev_rows: ::core::ffi::c_int, prev_cols: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_REPORT_constraintinfo"]
    fn REPORT_constraintinfo(lp: *mut lprec, datainfo: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_REPORT_modelinfo"]
    fn REPORT_modelinfo(
        lp: *mut lprec,
        doName: ::core::ffi::c_uchar,
        datainfo: *mut ::core::ffi::c_char,
    );
    #[link_name="honest_lpsolve_gcd"]
    fn gcd(
        a: ::core::ffi::c_longlong,
        b: ::core::ffi::c_longlong,
        c: *mut ::core::ffi::c_int,
        d: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_qsortex"]
    fn qsortex(
        attributes: *mut ::core::ffi::c_void,
        count: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        recsize: ::core::ffi::c_int,
        descending: ::core::ffi::c_uchar,
        findCompare: Option<findCompare_func>,
        tags: *mut ::core::ffi::c_void,
        tagsize: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_compareREAL"]
    fn compareREAL(
        current: *const ::core::ffi::c_void,
        candidate: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_QS_execute"]
    fn QS_execute(
        a: *mut QSORTrec,
        count: ::core::ffi::c_int,
        findCompare: Option<findCompare_func>,
        nswaps: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _psrec {
    pub varmap: *mut LLrec,
    pub next: *mut *mut ::core::ffi::c_int,
    pub empty: *mut ::core::ffi::c_int,
    pub plucount: *mut ::core::ffi::c_int,
    pub negcount: *mut ::core::ffi::c_int,
    pub pluneg: *mut ::core::ffi::c_int,
    pub infcount: *mut ::core::ffi::c_int,
    pub plulower: *mut ::core::ffi::c_double,
    pub neglower: *mut ::core::ffi::c_double,
    pub pluupper: *mut ::core::ffi::c_double,
    pub negupper: *mut ::core::ffi::c_double,
    pub allocsize: ::core::ffi::c_int,
}
pub type psrec = _psrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _presolverec {
    pub rows: *mut psrec,
    pub cols: *mut psrec,
    pub EQmap: *mut LLrec,
    pub LTmap: *mut LLrec,
    pub INTmap: *mut LLrec,
    pub pv_upbo: *mut ::core::ffi::c_double,
    pub pv_lobo: *mut ::core::ffi::c_double,
    pub dv_upbo: *mut ::core::ffi::c_double,
    pub dv_lobo: *mut ::core::ffi::c_double,
    pub lp: *mut lprec,
    pub epsvalue: ::core::ffi::c_double,
    pub epspivot: ::core::ffi::c_double,
    pub innerloops: ::core::ffi::c_int,
    pub middleloops: ::core::ffi::c_int,
    pub outerloops: ::core::ffi::c_int,
    pub nzdeleted: ::core::ffi::c_int,
    pub forceupdate: ::core::ffi::c_uchar,
}
pub type presolverec = _presolverec;
pub const MAX_PSMERGELOOPS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAX_PSBOUNDTIGHTENLOOPS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MIN_SOS1LENGTH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PRESOLVE_EPSPIVOT: ::core::ffi::c_double = 1.0e-3f64;
pub const PRESOLVE_BOUNDSLACK: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[export_name="honest_lpsolve_presolve_rowlength"]
pub unsafe extern "C" fn presolve_rowlength(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut items: *mut ::core::ffi::c_int = *(*(*psdata).rows).next.offset(rownr as isize);
    if items.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return *items.offset(0 as ::core::ffi::c_int as isize);
    };
}
#[export_name="honest_lpsolve_presolve_collength"]
pub unsafe extern "C" fn presolve_collength(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut items: *mut ::core::ffi::c_int = *(*(*psdata).cols).next.offset(colnr as isize);
    if items.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return *items.offset(0 as ::core::ffi::c_int as isize);
    };
}
#[export_name="honest_lpsolve_presolve_setstatusex"]
pub unsafe extern "C" fn presolve_setstatusex(
    mut psdata: *mut presolverec,
    mut status: ::core::ffi::c_int,
    mut lineno: ::core::ffi::c_int,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if status == INFEASIBLE || status == UNBOUNDED {
        report(
            (*psdata).lp,
            5 as ::core::ffi::c_int,
            b"presolve_setstatus: Status set to '%s' on code line %d, file '%s'\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_statuscheck"]
pub unsafe extern "C" fn presolve_statuscheck(
    mut psdata: *mut presolverec,
    mut status: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if *status == RUNNING {
        let mut lp: *mut lprec = (*psdata).lp;
        if mat_validate((*lp).matA) == 0 {
            *status = MATRIXERROR;
        } else if userabort(lp, -(1 as ::core::ffi::c_int)) != 0 {
            *status = (*lp).spx_status;
        }
    }
    return (*status == RUNNING) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_createUndo"]
pub unsafe extern "C" fn presolve_createUndo(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    if !(*lp).presolve_undo.is_null() {
        presolve_freeUndo(lp);
    }
    (*lp).presolve_undo = calloc(
        1 as size_t,
        ::core::mem::size_of::<presolveundorec>() as size_t,
    ) as *mut presolveundorec;
    (*(*lp).presolve_undo).lp = lp;
    if (*lp).presolve_undo.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_inc_presolve_space"]
pub unsafe extern "C" fn inc_presolve_space(
    mut lp: *mut lprec,
    mut delta: ::core::ffi::c_int,
    mut isrows: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut oldrowcolalloc: ::core::ffi::c_int = 0;
    let mut rowcolsum: ::core::ffi::c_int = 0;
    let mut oldrowalloc: ::core::ffi::c_int = 0;
    let mut oldcolalloc: ::core::ffi::c_int = 0;
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    if psundo.is_null() {
        presolve_createUndo(lp);
        psundo = (*lp).presolve_undo;
    }
    oldrowalloc = (*lp).rows_alloc - delta;
    oldcolalloc = (*lp).columns_alloc - delta;
    oldrowcolalloc = (*lp).sum_alloc - delta;
    rowcolsum = (*lp).sum_alloc + 1 as ::core::ffi::c_int;
    if isrows != 0 {
        allocREAL(
            lp,
            &raw mut (*psundo).fixed_rhs,
            (*lp).rows_alloc + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
    } else {
        allocREAL(
            lp,
            &raw mut (*psundo).fixed_obj,
            (*lp).columns_alloc + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
    }
    allocINT(
        lp,
        &raw mut (*psundo).var_to_orig,
        rowcolsum,
        AUTOMATIC as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut (*psundo).orig_to_var,
        rowcolsum,
        AUTOMATIC as ::core::ffi::c_uchar,
    );
    if isrows != 0 {
        ii = oldrowalloc + 1 as ::core::ffi::c_int;
    } else {
        ii = oldcolalloc + 1 as ::core::ffi::c_int;
    }
    i = oldrowcolalloc + 1 as ::core::ffi::c_int;
    while i < rowcolsum {
        *(*psundo).var_to_orig.offset(i as isize) = 0 as ::core::ffi::c_int;
        *(*psundo).orig_to_var.offset(i as isize) = 0 as ::core::ffi::c_int;
        if isrows != 0 {
            *(*psundo).fixed_rhs.offset(ii as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *(*psundo).fixed_obj.offset(ii as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        i += 1;
        ii += 1;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_setOrig"]
pub unsafe extern "C" fn presolve_setOrig(
    mut lp: *mut lprec,
    mut orig_rows: ::core::ffi::c_int,
    mut orig_cols: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    if psundo.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    (*psundo).orig_rows = orig_rows;
    (*psundo).orig_columns = orig_cols;
    (*psundo).orig_sum = orig_rows + orig_cols;
    if (*lp).wasPresolved != 0 {
        presolve_fillUndo(lp, orig_rows, orig_cols, FALSE as ::core::ffi::c_uchar);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_fillUndo"]
pub unsafe extern "C" fn presolve_fillUndo(
    mut lp: *mut lprec,
    mut orig_rows: ::core::ffi::c_int,
    mut orig_cols: ::core::ffi::c_int,
    mut setOrig: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    i = 0 as ::core::ffi::c_int;
    while i <= orig_rows {
        *(*psundo).var_to_orig.offset(i as isize) = i;
        *(*psundo).orig_to_var.offset(i as isize) = i;
        *(*psundo).fixed_rhs.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= orig_cols {
        *(*psundo).var_to_orig.offset((orig_rows + i) as isize) = i;
        *(*psundo).orig_to_var.offset((orig_rows + i) as isize) = i;
        *(*psundo).fixed_obj.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    if setOrig != 0 {
        presolve_setOrig(lp, orig_rows, orig_cols);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_rebuildUndo"]
pub unsafe extern "C" fn presolve_rebuildUndo(
    mut lp: *mut lprec,
    mut isprimal: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut ik: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut colnrDep: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut hold: ::core::ffi::c_double = 0.;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut solution: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut slacks: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut psdata: *mut presolveundorec = (*lp).presolve_undo;
    let mut mat: *mut MATrec = ::core::ptr::null_mut::<MATrec>();
    if isprimal != 0 {
        if !(*psdata).primalundo.is_null() {
            mat = (*(*psdata).primalundo).tracker;
        }
        if mat.is_null() {
            return 0 as ::core::ffi::c_uchar;
        }
        solution = (*lp)
            .full_solution
            .offset((*(*lp).presolve_undo).orig_rows as isize);
        slacks = (*lp).full_solution;
    } else {
        if !(*psdata).dualundo.is_null() {
            mat = (*(*psdata).dualundo).tracker;
        }
        if mat.is_null() {
            return 0 as ::core::ffi::c_uchar;
        }
        solution = (*lp).full_duals;
        slacks = (*lp)
            .full_duals
            .offset((*(*lp).presolve_undo).orig_rows as isize);
    }
    j = *(*mat).col_tag.offset(0 as ::core::ffi::c_int as isize);
    while j > 0 as ::core::ffi::c_int {
        ix = *(*mat).col_tag.offset(j as isize);
        ik = *(*mat)
            .col_end
            .offset((j - 1 as ::core::ffi::c_int) as isize);
        ie = *(*mat).col_end.offset(j as isize);
        colnrDep = (*mat).col_mat_rownr.offset(ik as isize) as *mut ::core::ffi::c_int;
        value = (*mat).col_mat_value.offset(ik as isize) as *mut ::core::ffi::c_double;
        hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        k = 0 as ::core::ffi::c_int;
        while ik < ie {
            if *colnrDep == 0 as ::core::ffi::c_int {
                hold += *value;
            } else if isprimal as ::core::ffi::c_int != 0
                && *colnrDep > (*(*lp).presolve_undo).orig_columns
            {
                k = *colnrDep - (*(*lp).presolve_undo).orig_columns;
                hold -= *value * *slacks.offset(k as isize);
                *slacks.offset(k as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else if isprimal == 0 && *colnrDep > (*(*lp).presolve_undo).orig_rows {
                k = *colnrDep - (*(*lp).presolve_undo).orig_rows;
                hold -= *value * *slacks.offset(k as isize);
                *slacks.offset(k as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                hold -= *value * *solution.offset(*colnrDep as isize);
            }
            *value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            ik += 1;
            colnrDep = colnrDep.offset(matRowColStep as isize);
            value = value.offset(matValueStep as isize);
        }
        if fabs(hold) > (*lp).epsvalue {
            *solution.offset(ix as isize) = hold;
        }
        j -= 1;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_freeUndo"]
pub unsafe extern "C" fn presolve_freeUndo(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    if psundo.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    if !((*psundo).orig_to_var as *mut ::core::ffi::c_void).is_null() {
        free((*psundo).orig_to_var as *mut ::core::ffi::c_void);
        (*psundo).orig_to_var = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((*psundo).var_to_orig as *mut ::core::ffi::c_void).is_null() {
        free((*psundo).var_to_orig as *mut ::core::ffi::c_void);
        (*psundo).var_to_orig = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((*psundo).fixed_rhs as *mut ::core::ffi::c_void).is_null() {
        free((*psundo).fixed_rhs as *mut ::core::ffi::c_void);
        (*psundo).fixed_rhs = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*psundo).fixed_obj as *mut ::core::ffi::c_void).is_null() {
        free((*psundo).fixed_obj as *mut ::core::ffi::c_void);
        (*psundo).fixed_obj = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(*psundo).deletedA.is_null() {
        freeUndoLadder(&raw mut (*psundo).deletedA);
    }
    if !(*psundo).primalundo.is_null() {
        freeUndoLadder(&raw mut (*psundo).primalundo);
    }
    if !(*psundo).dualundo.is_null() {
        freeUndoLadder(&raw mut (*psundo).dualundo);
    }
    if !((*lp).presolve_undo as *mut ::core::ffi::c_void).is_null() {
        free((*lp).presolve_undo as *mut ::core::ffi::c_void);
        (*lp).presolve_undo = ::core::ptr::null_mut::<presolveundorec>();
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_storeDualUndo"]
pub unsafe extern "C" fn presolve_storeDualUndo(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
) {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut firstdone: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iix: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut Aij: ::core::ffi::c_double = get_mat(lp, rownr, colnr);
    let mut mat: *mut MATrec = (*lp).matA;
    if presolve_collength(psdata, colnr) == 0 as ::core::ffi::c_int {
        return;
    }
    item = 0 as ::core::ffi::c_int;
    ix = presolve_nextrow(psdata, colnr, &raw mut item);
    while ix >= 0 as ::core::ffi::c_int {
        iix = *(*mat).col_mat_rownr.offset(ix as isize);
        if !(iix == rownr) {
            if firstdone == 0 {
                firstdone = addUndoPresolve(
                    lp,
                    FALSE as ::core::ffi::c_uchar,
                    rownr,
                    get_mat(lp, 0 as ::core::ffi::c_int, colnr) / Aij,
                    get_mat_byindex(
                        lp,
                        ix,
                        FALSE as ::core::ffi::c_uchar,
                        TRUE as ::core::ffi::c_uchar,
                    ) / Aij,
                    iix,
                );
            } else {
                appendUndoPresolve(
                    lp,
                    FALSE as ::core::ffi::c_uchar,
                    get_mat_byindex(
                        lp,
                        ix,
                        FALSE as ::core::ffi::c_uchar,
                        TRUE as ::core::ffi::c_uchar,
                    ) / Aij,
                    iix,
                );
            }
        }
        ix = presolve_nextrow(psdata, colnr, &raw mut item);
    }
}
#[export_name="honest_lpsolve_presolve_SOScheck"]
pub unsafe extern "C" fn presolve_SOScheck(mut psdata: *mut presolverec) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut nk: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut nSOS: ::core::ffi::c_int = SOS_count(lp);
    let mut nerr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    if nSOS == 0 as ::core::ffi::c_int {
        return status;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= nSOS {
        SOS = *(*(*lp).SOS)
            .sos_list
            .offset((i - 1 as ::core::ffi::c_int) as isize);
        list = (*SOS).members;
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        j = 1 as ::core::ffi::c_int;
        while j <= n {
            colnr = *list.offset(j as isize);
            if colnr < 1 as ::core::ffi::c_int || colnr > (*lp).columns {
                nerr += 1;
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"presolve_SOScheck: A - Column index %d is outside of valid range\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if isActiveLink((*(*psdata).cols).varmap, colnr) == 0 {
                nerr += 1;
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"presolve_SOScheck: B - Column index %d has been marked for deletion\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if SOS_member_index((*lp).SOS, i, colnr) != j {
                nerr += 1;
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"presolve_SOScheck: C - Column index %d not found in fast search array\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            k = *(*(*lp).SOS)
                .memberpos
                .offset((colnr - 1 as ::core::ffi::c_int) as isize);
            nk = *(*(*lp).SOS).memberpos.offset(colnr as isize);
            while k < nk && *(*(*lp).SOS).membership.offset(k as isize) != i {
                k += 1;
            }
            if k >= nk {
                nerr += 1;
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"presolve_SOScheck: D - Column index %d was not found in sparse array\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            j += 1;
        }
        i += 1;
    }
    colnr = 1 as ::core::ffi::c_int;
    while colnr <= (*lp).columns {
        k = *(*(*lp).SOS)
            .memberpos
            .offset((colnr - 1 as ::core::ffi::c_int) as isize);
        nk = *(*(*lp).SOS).memberpos.offset(colnr as isize);
        while k < nk {
            if SOS_is_member(
                (*lp).SOS,
                *(*(*lp).SOS).membership.offset(k as isize),
                colnr,
            ) == 0
            {
                nerr += 1;
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"presolve_SOScheck: E - Sparse array did not indicate column index %d as member of SOS %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            k += 1;
        }
        colnr += 1;
    }
    status = (nerr == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if status == 0 {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"presolve_SOScheck: There were %d errors\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return status;
}
unsafe extern "C" fn presolve_roundrhs(
    mut lp: *mut lprec,
    mut value: ::core::ffi::c_double,
    mut isGE: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut eps: ::core::ffi::c_double =
        0.1f64 * (*lp).epsprimal * 1000 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut testout: ::core::ffi::c_double = restoreINT(value, eps);
    if (if isGE as ::core::ffi::c_int != 0
        && value - testout != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        -(value - testout)
    } else {
        value - testout
    }) < 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        value = testout;
    }
    return value;
}
unsafe extern "C" fn presolve_roundval(
    mut lp: *mut lprec,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    value = restoreINT(value, 0.1f64 * (*lp).epsprimal);
    return value;
}
unsafe extern "C" fn presolve_sumplumin(
    mut lp: *mut lprec,
    mut item: ::core::ffi::c_int,
    mut ps: *mut psrec,
    mut doUpper: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut plu: *mut ::core::ffi::c_double = if doUpper as ::core::ffi::c_int != 0 {
        (*ps).pluupper
    } else {
        (*ps).plulower
    };
    let mut neg: *mut ::core::ffi::c_double = if doUpper as ::core::ffi::c_int != 0 {
        (*ps).negupper
    } else {
        (*ps).neglower
    };
    if fabs(*plu.offset(item as isize)) >= (*lp).infinite {
        return *plu.offset(item as isize);
    } else if fabs(*neg.offset(item as isize)) >= (*lp).infinite {
        return *neg.offset(item as isize);
    } else {
        return *plu.offset(item as isize) + *neg.offset(item as isize);
    };
}
unsafe extern "C" fn presolve_range(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut ps: *mut psrec,
    mut loValue: *mut ::core::ffi::c_double,
    mut hiValue: *mut ::core::ffi::c_double,
) {
    *loValue = presolve_sumplumin(lp, rownr, ps, FALSE as ::core::ffi::c_uchar);
    *hiValue = presolve_sumplumin(lp, rownr, ps, TRUE as ::core::ffi::c_uchar);
}
#[export_name="honest_lpsolve_presolve_rangeorig"]
pub unsafe extern "C" fn presolve_rangeorig(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut ps: *mut psrec,
    mut loValue: *mut ::core::ffi::c_double,
    mut hiValue: *mut ::core::ffi::c_double,
    mut delta: ::core::ffi::c_double,
) {
    delta = if is_chsign(lp, rownr) as ::core::ffi::c_int != 0
        && *(*(*lp).presolve_undo).fixed_rhs.offset(rownr as isize) + delta
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        -(*(*(*lp).presolve_undo).fixed_rhs.offset(rownr as isize) + delta)
    } else {
        *(*(*lp).presolve_undo).fixed_rhs.offset(rownr as isize) + delta
    };
    *loValue = presolve_sumplumin(lp, rownr, ps, FALSE as ::core::ffi::c_uchar) + delta;
    *hiValue = presolve_sumplumin(lp, rownr, ps, TRUE as ::core::ffi::c_uchar) + delta;
}
#[export_name="honest_lpsolve_presolve_rowfeasible"]
pub unsafe extern "C" fn presolve_rowfeasible(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut userowmap: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut contype: ::core::ffi::c_int = 0;
    let mut origrownr: ::core::ffi::c_int = rownr;
    let mut LHS: ::core::ffi::c_double = 0.;
    let mut RHS: ::core::ffi::c_double = 0.;
    let mut value: ::core::ffi::c_double = 0.;
    if userowmap != 0 {
        rownr = firstActiveLink((*(*psdata).rows).varmap);
    }
    while status as ::core::ffi::c_int == TRUE && rownr != 0 as ::core::ffi::c_int {
        value = presolve_sumplumin(lp, rownr, (*psdata).rows, TRUE as ::core::ffi::c_uchar);
        LHS = get_rh_lower(lp, rownr);
        if value < LHS - (*lp).epssolution {
            contype = get_constr_type(lp, rownr);
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"presolve_rowfeasible: Lower bound infeasibility in %s row %s (%g << %g)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            if rownr != origrownr {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"        ...           Input row base used for testing was %s\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            status = FALSE as ::core::ffi::c_uchar;
        }
        value = presolve_sumplumin(lp, rownr, (*psdata).rows, FALSE as ::core::ffi::c_uchar);
        RHS = get_rh_upper(lp, rownr);
        if value > RHS + (*lp).epssolution {
            contype = get_constr_type(lp, rownr);
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"presolve_rowfeasible: Upper bound infeasibility in %s row %s (%g >> %g)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            status = FALSE as ::core::ffi::c_uchar;
        }
        if userowmap != 0 {
            rownr = nextActiveLink((*(*psdata).rows).varmap, rownr);
        } else {
            rownr = 0 as ::core::ffi::c_int;
        }
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_debugmap"]
pub unsafe extern "C" fn presolve_debugmap(
    mut psdata: *mut presolverec,
    mut caption: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut current_block: u64;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut nx: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut cols: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rows: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nz: ::core::ffi::c_int =
        *(*mat).col_end.offset((*lp).columns as isize) - 1 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    colnr = 1 as ::core::ffi::c_int;
    's_12: loop {
        if !(colnr <= (*lp).columns) {
            current_block = 14576567515993809846;
            break;
        }
        rows = *(*(*psdata).cols).next.offset(colnr as isize);
        if isActiveLink((*(*psdata).cols).varmap, colnr) == 0 {
            if !rows.is_null() {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"presolve_debugmap: Inactive column %d is non-empty\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                current_block = 14825183042480097530;
                break;
            }
        } else {
            if rows.is_null() {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"presolve_debugmap: Active column %d is empty\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            je = *rows;
            rows = rows.offset(1);
            jx = 1 as ::core::ffi::c_int;
            while jx <= je {
                if *rows < 0 as ::core::ffi::c_int || *rows > nz {
                    report(
                        lp,
                        2 as ::core::ffi::c_int,
                        b"presolve_debugmap: NZ index %d for column %d out of range (index %d<=%d)\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    current_block = 14825183042480097530;
                    break 's_12;
                } else {
                    cols = *(*(*psdata).rows)
                        .next
                        .offset(*(*mat).col_mat_rownr.offset(*rows as isize) as isize);
                    ie = *cols.offset(0 as ::core::ffi::c_int as isize);
                    ix = 1 as ::core::ffi::c_int;
                    while ix <= ie {
                        nx = *cols.offset(ix as isize);
                        if nx < 0 as ::core::ffi::c_int || nx > nz {
                            report(
                                lp,
                                2 as ::core::ffi::c_int,
                                b"presolve_debugmap: NZ index %d for column %d to row %d out of range\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            current_block = 14825183042480097530;
                            break 's_12;
                        } else {
                            ix += 1;
                        }
                    }
                    jx += 1;
                    rows = rows.offset(1);
                }
            }
        }
        colnr += 1;
    }
    match current_block {
        14576567515993809846 => {
            status = TRUE as ::core::ffi::c_uchar;
        }
        _ => {}
    }
    if status == 0 && !caption.is_null() {
        report(
            lp,
            2 as ::core::ffi::c_int,
            b"...caller was '%s'\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_validate"]
pub unsafe extern "C" fn presolve_validate(
    mut psdata: *mut presolverec,
    mut forceupdate: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut rownr: ::core::ffi::c_int = 0;
    let mut items: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut upbound: ::core::ffi::c_double = 0.;
    let mut lobound: ::core::ffi::c_double = 0.;
    let mut value: ::core::ffi::c_double = 0.;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut status: ::core::ffi::c_uchar = ((*mat).row_end_valid as ::core::ffi::c_int != 0
        && forceupdate == 0) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    if status != 0 {
        return status;
    } else if (*mat).row_end_valid == 0 {
        status = mat_validate(mat);
    } else {
        status = forceupdate;
    }
    if status != 0 {
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            *(*(*psdata).rows).plucount.offset(i as isize) = 0 as ::core::ffi::c_int;
            *(*(*psdata).rows).negcount.offset(i as isize) = 0 as ::core::ffi::c_int;
            *(*(*psdata).rows).pluneg.offset(i as isize) = 0 as ::core::ffi::c_int;
            if isActiveLink((*(*psdata).rows).varmap, i) == 0 {
                if !(*(*(*psdata).rows).next.offset(i as isize) as *mut ::core::ffi::c_void)
                    .is_null()
                {
                    free(*(*(*psdata).rows).next.offset(i as isize) as *mut ::core::ffi::c_void);
                    let ref mut fresh5 = *(*(*psdata).rows).next.offset(i as isize);
                    *fresh5 = ::core::ptr::null_mut::<::core::ffi::c_int>();
                }
            } else {
                k = mat_rowlength(mat, i);
                allocINT(
                    lp,
                    (*(*psdata).rows).next.offset(i as isize) as *mut *mut ::core::ffi::c_int,
                    k + 1 as ::core::ffi::c_int,
                    AUTOMATIC as ::core::ffi::c_uchar,
                );
                items = *(*(*psdata).rows).next.offset(i as isize);
                je = *(*mat).row_end.offset(i as isize);
                k = 0 as ::core::ffi::c_int;
                j = *(*mat)
                    .row_end
                    .offset((i - 1 as ::core::ffi::c_int) as isize);
                while j < je {
                    if isActiveLink(
                        (*(*psdata).cols).varmap,
                        *(*mat)
                            .col_mat_colnr
                            .offset(*(*mat).row_mat.offset(j as isize) as isize),
                    ) != 0
                    {
                        k += 1;
                        *items.offset(k as isize) = j;
                    }
                    j += 1;
                }
                *items.offset(0 as ::core::ffi::c_int as isize) = k;
            }
            i += 1;
        }
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            *(*(*psdata).cols).plucount.offset(j as isize) = 0 as ::core::ffi::c_int;
            *(*(*psdata).cols).negcount.offset(j as isize) = 0 as ::core::ffi::c_int;
            *(*(*psdata).cols).pluneg.offset(j as isize) = 0 as ::core::ffi::c_int;
            if isActiveLink((*(*psdata).cols).varmap, j) == 0 {
                if !(*(*(*psdata).cols).next.offset(j as isize) as *mut ::core::ffi::c_void)
                    .is_null()
                {
                    free(*(*(*psdata).cols).next.offset(j as isize) as *mut ::core::ffi::c_void);
                    let ref mut fresh6 = *(*(*psdata).cols).next.offset(j as isize);
                    *fresh6 = ::core::ptr::null_mut::<::core::ffi::c_int>();
                }
            } else {
                upbound = get_upbo(lp, j);
                lobound = get_lowbo(lp, j);
                if is_semicont(lp, j) as ::core::ffi::c_int != 0 && upbound > lobound {
                    if lobound > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        lobound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    } else if upbound < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        upbound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                }
                k = mat_collength(mat, j);
                allocINT(
                    lp,
                    (*(*psdata).cols).next.offset(j as isize) as *mut *mut ::core::ffi::c_int,
                    k + 1 as ::core::ffi::c_int,
                    AUTOMATIC as ::core::ffi::c_uchar,
                );
                items = *(*(*psdata).cols).next.offset(j as isize);
                ie = *(*mat).col_end.offset(j as isize);
                k = 0 as ::core::ffi::c_int;
                i = *(*mat)
                    .col_end
                    .offset((j - 1 as ::core::ffi::c_int) as isize);
                while i < ie {
                    rownr = *(*mat).col_mat_rownr.offset(i as isize);
                    if isActiveLink((*(*psdata).rows).varmap, rownr) != 0 {
                        k += 1;
                        *items.offset(k as isize) = i;
                        value = *(*mat).col_mat_value.offset(i as isize);
                        if (if is_chsign(lp, rownr) as ::core::ffi::c_int != 0
                            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -value
                        } else {
                            value
                        }) > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            let ref mut fresh7 = *(*(*psdata).rows).plucount.offset(rownr as isize);
                            *fresh7 += 1;
                            let ref mut fresh8 = *(*(*psdata).cols).plucount.offset(j as isize);
                            *fresh8 += 1;
                        } else {
                            let ref mut fresh9 = *(*(*psdata).rows).negcount.offset(rownr as isize);
                            *fresh9 += 1;
                            let ref mut fresh10 = *(*(*psdata).cols).negcount.offset(j as isize);
                            *fresh10 += 1;
                        }
                        if lobound < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            && upbound >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            let ref mut fresh11 = *(*(*psdata).rows).pluneg.offset(rownr as isize);
                            *fresh11 += 1;
                            let ref mut fresh12 = *(*(*psdata).cols).pluneg.offset(j as isize);
                            *fresh12 += 1;
                        }
                    }
                    i += 1;
                }
                *items.offset(0 as ::core::ffi::c_int as isize) = k;
            }
            j += 1;
        }
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_rowtallies"]
pub unsafe extern "C" fn presolve_rowtallies(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut plu: *mut ::core::ffi::c_int,
    mut neg: *mut ::core::ffi::c_int,
    mut pluneg: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut value: ::core::ffi::c_double = 0.;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ix: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut chsign: ::core::ffi::c_uchar = is_chsign(lp, rownr);
    *plu = 0 as ::core::ffi::c_int;
    *neg = 0 as ::core::ffi::c_int;
    *pluneg = 0 as ::core::ffi::c_int;
    ix = presolve_nextcol(psdata, rownr, &raw mut ib);
    while ix >= 0 as ::core::ffi::c_int {
        jx = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(ix as isize) as isize);
        value = *(*mat)
            .col_mat_value
            .offset(*(*mat).row_mat.offset(ix as isize) as isize);
        if (if chsign as ::core::ffi::c_int != 0
            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -value
        } else {
            value
        }) > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *plu += 1;
        } else {
            *neg += 1;
        }
        if get_lowbo(lp, jx) < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && get_upbo(lp, jx) >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *pluneg += 1;
        }
        ix = presolve_nextcol(psdata, rownr, &raw mut ib);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_debugrowtallies"]
pub unsafe extern "C" fn presolve_debugrowtallies(
    mut psdata: *mut presolverec,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut plu: ::core::ffi::c_int = 0;
    let mut neg: ::core::ffi::c_int = 0;
    let mut pluneg: ::core::ffi::c_int = 0;
    let mut nerr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if isActiveLink((*(*psdata).rows).varmap, i) as ::core::ffi::c_int != 0
            && presolve_rowtallies(psdata, i, &raw mut plu, &raw mut neg, &raw mut pluneg)
                as ::core::ffi::c_int
                != 0
        {
            if *(*(*psdata).rows).plucount.offset(i as isize) != plu
                || *(*(*psdata).rows).negcount.offset(i as isize) != neg
                || *(*(*psdata).rows).pluneg.offset(i as isize) != pluneg
            {
                nerr += 1;
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"presolve_debugrowtallies: Detected inconsistent count for row %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    return (nerr == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_debugcheck"]
pub unsafe extern "C" fn presolve_debugcheck(
    mut lp: *mut lprec,
    mut rowmap: *mut LLrec,
    mut colmap: *mut LLrec,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut errc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < (*lp).rows {
        if !(!rowmap.is_null() && isActiveLink(rowmap, i) == 0) {
            if *(*lp).orig_upbo.offset(i as isize)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                errc += 1;
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"presolve_debugcheck: Detected negative range %g for row %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j < (*lp).columns {
        if !(!colmap.is_null() && isActiveLink(colmap, j) == 0) {
            i = (*lp).rows + j;
            if *(*lp).orig_lowbo.offset(i as isize) > *(*lp).orig_upbo.offset(i as isize) {
                errc += 1;
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"presolve_debugcheck: Detected UB < LB for column %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        j += 1;
    }
    return errc;
}
#[export_name="honest_lpsolve_presolve_candeletevar"]
pub unsafe extern "C" fn presolve_candeletevar(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut usecount: ::core::ffi::c_int = SOS_memberships((*lp).SOS, colnr);
    return ((*lp).SOS.is_null()
        || usecount == 0 as ::core::ffi::c_int
        || ((*(*lp).SOS).sos1_count == (*(*lp).SOS).sos_count
            || usecount == SOS_is_member_of_type((*lp).SOS, colnr, SOS1) as ::core::ffi::c_int))
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_rowlengthex"]
pub unsafe extern "C" fn presolve_rowlengthex(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j1: ::core::ffi::c_int = *(*(*psdata).rows).plucount.offset(rownr as isize)
        + *(*(*psdata).rows).negcount.offset(rownr as isize);
    return j1;
}
#[export_name="honest_lpsolve_presolve_rowlengthdebug"]
pub unsafe extern "C" fn presolve_rowlengthdebug(
    mut psdata: *mut presolverec,
) -> ::core::ffi::c_int {
    let mut rownr: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rownr = firstActiveLink((*(*psdata).rows).varmap);
    while rownr != 0 as ::core::ffi::c_int {
        n += presolve_rowlengthex(psdata, rownr);
        rownr = nextActiveLink((*(*psdata).rows).varmap, rownr);
    }
    return n;
}
unsafe extern "C" fn presolve_nextrecord(
    mut ps: *mut psrec,
    mut recnr: ::core::ffi::c_int,
    mut previtem: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nzlist: *mut ::core::ffi::c_int = *(*ps).next.offset(recnr as isize);
    let mut nzcount: ::core::ffi::c_int = *nzlist.offset(0 as ::core::ffi::c_int as isize);
    let mut status: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if previtem.is_null() {
        if !nzlist.is_null() {
            status = *nzlist.offset(*nzlist as isize);
        }
        return status;
    }
    *previtem += 1;
    if *previtem > nzcount {
        *previtem = 0 as ::core::ffi::c_int;
    } else {
        status = *nzlist.offset(*previtem as isize);
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_nextcol"]
pub unsafe extern "C" fn presolve_nextcol(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut previtem: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return presolve_nextrecord((*psdata).rows, rownr, previtem);
}
unsafe extern "C" fn presolve_lastcol(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return presolve_nextrecord(
        (*psdata).rows,
        rownr,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[export_name="honest_lpsolve_presolve_nextrow"]
pub unsafe extern "C" fn presolve_nextrow(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
    mut previtem: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return presolve_nextrecord((*psdata).cols, colnr, previtem);
}
unsafe extern "C" fn presolve_lastrow(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return presolve_nextrecord(
        (*psdata).cols,
        colnr,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
unsafe extern "C" fn presolve_adjustrhs(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut fixdelta: ::core::ffi::c_double,
    mut epsvalue: ::core::ffi::c_double,
) {
    let mut lp: *mut lprec = (*psdata).lp;
    *(*lp).orig_rhs.offset(rownr as isize) -= fixdelta;
    if epsvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if fabs(*(*lp).orig_rhs.offset(rownr as isize)) < epsvalue {
            *(*lp).orig_rhs.offset(rownr as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    *(*(*lp).presolve_undo).fixed_rhs.offset(rownr as isize) += fixdelta;
}
#[export_name="honest_lpsolve_presolve_shrink"]
pub unsafe extern "C" fn presolve_shrink(
    mut psdata: *mut presolverec,
    mut nConRemove: *mut ::core::ffi::c_int,
    mut nVarRemove: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut SOS: *mut SOSgroup = (*(*psdata).lp).SOS;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut countR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut countC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut list: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut fixValue: ::core::ffi::c_double = 0.;
    list = (*(*psdata).rows).empty;
    if !list.is_null() {
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if isActiveLink((*(*psdata).rows).varmap, *list.offset(i as isize)) != 0 {
                presolve_rowremove(
                    psdata,
                    *list.offset(i as isize),
                    FALSE as ::core::ffi::c_uchar,
                );
                countR += 1;
            }
            i += 1;
        }
        if !nConRemove.is_null() {
            *nConRemove += countR;
        }
        *list.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    }
    list = (*(*psdata).cols).empty;
    if !list.is_null() {
        n = *list.offset(0 as ::core::ffi::c_int as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            ix = *list.offset(i as isize);
            if isActiveLink((*(*psdata).cols).varmap, ix) != 0 {
                if presolve_colfixdual(psdata, ix, &raw mut fixValue, &raw mut status) != 0 {
                    if presolve_colfix(
                        psdata,
                        ix,
                        fixValue,
                        TRUE as ::core::ffi::c_uchar,
                        nVarRemove,
                    ) == 0
                    {
                        status = presolve_setstatusex(
                            psdata,
                            2 as ::core::ffi::c_int,
                            839 as ::core::ffi::c_int,
                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        break;
                    } else {
                        presolve_colremove(psdata, ix, FALSE as ::core::ffi::c_uchar);
                        countC += 1;
                    }
                } else if SOS_is_member(SOS, 0 as ::core::ffi::c_int, ix) != 0 {
                    report(
                        (*psdata).lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_shrink: Empty column %d is member of a SOS\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            i += 1;
        }
        *list.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_rowremove"]
pub unsafe extern "C" fn presolve_rowremove(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut allowcoldelete: ::core::ffi::c_uchar,
) {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut nx: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut cols: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rows: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut n: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    cols = *(*(*psdata).rows).next.offset(rownr as isize);
    ie = *cols;
    cols = cols.offset(1);
    ix = 1 as ::core::ffi::c_int;
    while ix <= ie {
        n = 0 as ::core::ffi::c_int;
        colnr = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(*cols as isize) as isize);
        rows = *(*(*psdata).cols).next.offset(colnr as isize);
        je = *rows.offset(0 as ::core::ffi::c_int as isize);
        jx = je / 2 as ::core::ffi::c_int;
        if jx > 5 as ::core::ffi::c_int
            && rownr
                >= *(*mat)
                    .col_mat_rownr
                    .offset(*rows.offset(jx as isize) as isize)
        {
            n = jx - 1 as ::core::ffi::c_int;
        } else {
            jx = 1 as ::core::ffi::c_int;
        }
        while jx <= je {
            nx = *rows.offset(jx as isize);
            if *(*mat).col_mat_rownr.offset(nx as isize) != rownr {
                n += 1;
                *rows.offset(n as isize) = nx;
            }
            jx += 1;
        }
        *rows.offset(0 as ::core::ffi::c_int as isize) = n;
        if n == 0 as ::core::ffi::c_int && allowcoldelete as ::core::ffi::c_int != 0 {
            let mut list: *mut ::core::ffi::c_int = (*(*psdata).cols).empty;
            let ref mut fresh16 = *list.offset(0 as ::core::ffi::c_int as isize);
            *fresh16 += 1;
            n = *fresh16;
            *list.offset(n as isize) = colnr;
        }
        ix += 1;
        cols = cols.offset(1);
    }
    if !(*(*(*psdata).rows).next.offset(rownr as isize) as *mut ::core::ffi::c_void).is_null() {
        free(*(*(*psdata).rows).next.offset(rownr as isize) as *mut ::core::ffi::c_void);
        let ref mut fresh17 = *(*(*psdata).rows).next.offset(rownr as isize);
        *fresh17 = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    removeLink((*(*psdata).rows).varmap, rownr);
    match get_constr_type(lp, rownr) {
        LE => {
            removeLink((*psdata).LTmap, rownr);
        }
        EQ => {
            removeLink((*psdata).EQmap, rownr);
        }
        _ => {}
    }
    if isActiveLink((*psdata).INTmap, rownr) != 0 {
        removeLink((*psdata).INTmap, rownr);
    }
}
#[export_name="honest_lpsolve_presolve_colremove"]
pub unsafe extern "C" fn presolve_colremove(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
    mut allowrowdelete: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut nx: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut cols: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rows: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut n: ::core::ffi::c_int = 0;
    let mut rownr: ::core::ffi::c_int = 0;
    rows = *(*(*psdata).cols).next.offset(colnr as isize);
    je = *rows;
    rows = rows.offset(1);
    jx = 1 as ::core::ffi::c_int;
    while jx <= je {
        n = 0 as ::core::ffi::c_int;
        rownr = *(*mat).col_mat_rownr.offset(*rows as isize);
        cols = *(*(*psdata).rows).next.offset(rownr as isize);
        ie = *cols.offset(0 as ::core::ffi::c_int as isize);
        ix = ie / 2 as ::core::ffi::c_int;
        if ix > 5 as ::core::ffi::c_int
            && colnr
                >= *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(*cols.offset(ix as isize) as isize) as isize)
        {
            n = ix - 1 as ::core::ffi::c_int;
        } else {
            ix = 1 as ::core::ffi::c_int;
        }
        while ix <= ie {
            nx = *cols.offset(ix as isize);
            if *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(nx as isize) as isize)
                != colnr
            {
                n += 1;
                *cols.offset(n as isize) = nx;
            }
            ix += 1;
        }
        *cols.offset(0 as ::core::ffi::c_int as isize) = n;
        if n == 0 as ::core::ffi::c_int && allowrowdelete as ::core::ffi::c_int != 0 {
            let mut list: *mut ::core::ffi::c_int = (*(*psdata).rows).empty;
            let ref mut fresh14 = *list.offset(0 as ::core::ffi::c_int as isize);
            *fresh14 += 1;
            n = *fresh14;
            *list.offset(n as isize) = rownr;
        }
        jx += 1;
        rows = rows.offset(1);
    }
    if !(*(*(*psdata).cols).next.offset(colnr as isize) as *mut ::core::ffi::c_void).is_null() {
        free(*(*(*psdata).cols).next.offset(colnr as isize) as *mut ::core::ffi::c_void);
        let ref mut fresh15 = *(*(*psdata).cols).next.offset(colnr as isize);
        *fresh15 = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, colnr) != 0 {
        if !(*lp).sos_priority.is_null() {
            (*lp).sos_vars -= 1;
            if is_int(lp, colnr) != 0 {
                (*lp).sos_ints -= 1;
            }
        }
        SOS_member_delete((*lp).SOS, 0 as ::core::ffi::c_int, colnr);
        clean_SOSgroup((*lp).SOS, TRUE as ::core::ffi::c_uchar);
        if SOS_count(lp) == 0 as ::core::ffi::c_int {
            free_SOSgroup(&raw mut (*lp).SOS);
        }
    }
    colnr = removeLink((*(*psdata).cols).varmap, colnr);
    return colnr;
}
#[export_name="honest_lpsolve_presolve_redundantSOS"]
pub unsafe extern "C" fn presolve_redundantSOS(
    mut psdata: *mut presolverec,
    mut nb: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut kk: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut fixed: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut iBoundTighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    ii = SOS_count(lp);
    i = ii;
    if ii == 0 as ::core::ffi::c_int {
        return status;
    }
    if allocINT(
        lp,
        &raw mut fixed,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    ) == 0
    {
        return (*lp).spx_status;
    }
    's_24: loop {
        if !(i > 0 as ::core::ffi::c_int) {
            current_block = 14832935472441733737;
            break;
        }
        SOS = *(*(*lp).SOS)
            .sos_list
            .offset((i - 1 as ::core::ffi::c_int) as isize);
        kk = *(*SOS).members.offset(0 as ::core::ffi::c_int as isize);
        *fixed.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
        k = 1 as ::core::ffi::c_int;
        while k <= kk {
            j = *(*SOS).members.offset(k as isize);
            if get_lowbo(lp, j) > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && is_semicont(lp, j) == 0
            {
                let ref mut fresh48 = *fixed.offset(0 as ::core::ffi::c_int as isize);
                *fresh48 += 1;
                *fixed.offset(*fresh48 as isize) = k;
                if *fixed.offset(0 as ::core::ffi::c_int as isize) > (*SOS).type_0 {
                    status = presolve_setstatusex(
                        psdata,
                        2 as ::core::ffi::c_int,
                        1012 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    current_block = 13926791504171663083;
                    break 's_24;
                }
            }
            k += 1;
        }
        if *fixed.offset(0 as ::core::ffi::c_int as isize) == (*SOS).type_0 {
            k = 2 as ::core::ffi::c_int;
            while k <= *fixed.offset(0 as ::core::ffi::c_int as isize) {
                if *fixed.offset(k as isize)
                    != *fixed.offset((k - 1 as ::core::ffi::c_int) as isize)
                        + 1 as ::core::ffi::c_int
                {
                    status = presolve_setstatusex(
                        psdata,
                        2 as ::core::ffi::c_int,
                        1022 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    current_block = 13926791504171663083;
                    break 's_24;
                } else {
                    k += 1;
                }
            }
            k = kk;
            while k > 0 as ::core::ffi::c_int {
                j = *(*SOS).members.offset(k as isize);
                if !(get_lowbo(lp, j) > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && is_semicont(lp, j) == 0)
                {
                    if presolve_colfix(
                        psdata,
                        j,
                        0.0f64,
                        AUTOMATIC as ::core::ffi::c_uchar,
                        &raw mut iBoundTighten,
                    ) == 0
                    {
                        status = presolve_setstatusex(
                            psdata,
                            2 as ::core::ffi::c_int,
                            1032 as ::core::ffi::c_int,
                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        current_block = 13926791504171663083;
                        break 's_24;
                    }
                }
                k -= 1;
            }
            delete_SOSrec((*lp).SOS, i);
        } else if *fixed.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int {
            k = kk;
            while k > 0 as ::core::ffi::c_int {
                if !(k > *fixed.offset(*fixed.offset(0 as ::core::ffi::c_int as isize) as isize)
                    - (*SOS).type_0
                    && k < *fixed.offset(1 as ::core::ffi::c_int as isize) + (*SOS).type_0)
                {
                    j = *(*SOS).members.offset(k as isize);
                    SOS_member_delete((*lp).SOS, i, j);
                    if !(is_fixedvar(lp, nrows + j) != 0) {
                        if presolve_colfix(
                            psdata,
                            j,
                            0.0f64,
                            AUTOMATIC as ::core::ffi::c_uchar,
                            &raw mut iBoundTighten,
                        ) == 0
                        {
                            status = presolve_setstatusex(
                                psdata,
                                2 as ::core::ffi::c_int,
                                1051 as ::core::ffi::c_int,
                                b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            current_block = 13926791504171663083;
                            break 's_24;
                        }
                    }
                }
                k -= 1;
            }
        }
        i -= 1;
    }
    match current_block {
        14832935472441733737 => {
            i = SOS_count(lp);
            if i < ii || iBoundTighten > 0 as ::core::ffi::c_int {
                SOS_member_updatemap((*lp).SOS);
            }
            while i > 0 as ::core::ffi::c_int {
                (**(*(*lp).SOS)
                    .sos_list
                    .offset((i - 1 as ::core::ffi::c_int) as isize))
                .tagorder = i;
                i -= 1;
            }
        }
        _ => {}
    }
    if !(fixed as *mut ::core::ffi::c_void).is_null() {
        free(fixed as *mut ::core::ffi::c_void);
        fixed = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    *nb += iBoundTighten;
    *nSum += iBoundTighten;
    return status;
}
#[export_name="honest_lpsolve_presolve_fixSOS1"]
pub unsafe extern "C" fn presolve_fixSOS1(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
    mut fixvalue: ::core::ffi::c_double,
    mut nr: *mut ::core::ffi::c_int,
    mut nv: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut current_block: u64;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    let mut newvalue: ::core::ffi::c_double = 0.;
    let mut fixed: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if allocMYBOOL(
        lp,
        &raw mut fixed,
        (*lp).columns + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    i = SOS_count(lp);
    's_21: loop {
        if !(i > 0 as ::core::ffi::c_int) {
            current_block = 2668756484064249700;
            break;
        }
        SOS = *(*(*lp).SOS)
            .sos_list
            .offset((i - 1 as ::core::ffi::c_int) as isize);
        if SOS_is_member((*lp).SOS, i, colnr) != 0 {
            k = *(*SOS).members.offset(0 as ::core::ffi::c_int as isize);
            while k > 0 as ::core::ffi::c_int {
                j = *(*SOS).members.offset(k as isize);
                if !(*fixed.offset(j as isize) != 0) {
                    if j == colnr {
                        *fixed.offset(j as isize) = TRUE as ::core::ffi::c_uchar;
                        newvalue = fixvalue;
                    } else {
                        *fixed.offset(j as isize) = AUTOMATIC as ::core::ffi::c_uchar;
                        newvalue = 0.0f64;
                    }
                    if presolve_candeletevar(psdata, j) == 0 {
                        set_bounds(lp, j, newvalue, newvalue);
                        *fixed.offset(j as isize) = (TRUE | AUTOMATIC) as ::core::ffi::c_uchar;
                        (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
                    } else if presolve_colfix(psdata, j, newvalue, TRUE as ::core::ffi::c_uchar, nv)
                        == 0
                    {
                        current_block = 1684408901801682872;
                        break 's_21;
                    }
                }
                k -= 1;
            }
        }
        i -= 1;
    }
    match current_block {
        2668756484064249700 => {
            i = SOS_count(lp);
            k = i;
            while i > 0 as ::core::ffi::c_int {
                SOS = *(*(*lp).SOS)
                    .sos_list
                    .offset((i - 1 as ::core::ffi::c_int) as isize);
                if SOS_is_member((*lp).SOS, i, colnr) != 0 {
                    if (*SOS).type_0 == SOS1 {
                        delete_SOSrec((*lp).SOS, i);
                    } else {
                        j = 1 as ::core::ffi::c_int;
                        while j <= *(*SOS).members.offset(0 as ::core::ffi::c_int as isize) {
                            if *fixed.offset(*(*SOS).members.offset(j as isize) as isize)
                                as ::core::ffi::c_int
                                == AUTOMATIC
                            {
                                SOS_member_delete((*lp).SOS, i, *(*SOS).members.offset(j as isize));
                            }
                            j += 1;
                        }
                        j = *(*SOS).members.offset(0 as ::core::ffi::c_int as isize);
                        while j > 0 as ::core::ffi::c_int {
                            if *fixed.offset(*(*SOS).members.offset(j as isize) as isize)
                                as ::core::ffi::c_int
                                == AUTOMATIC
                            {
                                SOS_member_delete((*lp).SOS, i, *(*SOS).members.offset(j as isize));
                            }
                            j -= 1;
                        }
                    }
                }
                i -= 1;
            }
            i = SOS_count(lp);
            if i < k {
                SOS_member_updatemap((*lp).SOS);
            }
            k = 0 as ::core::ffi::c_int;
            j = (*lp).columns;
            while j > 0 as ::core::ffi::c_int {
                if *fixed.offset(j as isize) as ::core::ffi::c_int == TRUE
                    || *fixed.offset(j as isize) as ::core::ffi::c_int == AUTOMATIC
                {
                    presolve_colremove(psdata, j, TRUE as ::core::ffi::c_uchar);
                    k += 1;
                }
                j -= 1;
            }
            i = SOS_count(lp);
            while i > 0 as ::core::ffi::c_int {
                (**(*(*lp).SOS)
                    .sos_list
                    .offset((i - 1 as ::core::ffi::c_int) as isize))
                .tagorder = i;
                i -= 1;
            }
            status = TRUE as ::core::ffi::c_uchar;
        }
        _ => {}
    }
    if !(fixed as *mut ::core::ffi::c_void).is_null() {
        free(fixed as *mut ::core::ffi::c_void);
        fixed = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_setEQ"]
pub unsafe extern "C" fn presolve_setEQ(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
) {
    let mut lp: *mut lprec = (*psdata).lp;
    if is_constr_type(lp, rownr, LE) != 0 {
        removeLink((*psdata).LTmap, rownr);
    }
    setLink((*psdata).EQmap, rownr);
    set_constr_type(lp, rownr, EQ);
    *(*psdata).dv_lobo.offset(rownr as isize) = -(*lp).infinite;
    *(*psdata).dv_upbo.offset(rownr as isize) = (*lp).infinite;
}
#[export_name="honest_lpsolve_presolve_singletonbounds"]
pub unsafe extern "C" fn presolve_singletonbounds(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
    mut lobound: *mut ::core::ffi::c_double,
    mut upbound: *mut ::core::ffi::c_double,
    mut aval: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut coeff_a: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut isneg: ::core::ffi::c_uchar = 0;
    if is_constr_type(lp, rownr, EQ) as ::core::ffi::c_int != 0 && fabs(*lobound) < epsvalue {
        *upbound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *lobound = *upbound;
    } else {
        if aval.is_null() {
            coeff_a = get_mat(lp, rownr, colnr);
        } else {
            coeff_a = *aval;
        }
        isneg = (coeff_a < 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
            as ::core::ffi::c_uchar;
        if *lobound > -(*lp).infinite {
            *lobound /= coeff_a;
        } else if isneg != 0 {
            *lobound = -*lobound;
        }
        if *upbound < (*lp).infinite {
            *upbound /= coeff_a;
        } else if isneg != 0 {
            *upbound = -*upbound;
        }
        if isneg != 0 {
            swapREAL(lobound, upbound);
        }
    }
    if is_semicont(lp, colnr) != 0 {
        coeff_a = get_lowbo(lp, colnr);
        if coeff_a > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            if *lobound < 0.0f64 {
                *lobound = 0.0f64;
            }
            if *upbound > get_upbo(lp, colnr) {
                *upbound = get_upbo(lp, colnr);
            }
        } else {
            coeff_a = get_upbo(lp, colnr);
            if coeff_a > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                if *lobound < get_lowbo(lp, colnr) {
                    *lobound = get_lowbo(lp, colnr);
                }
                if *upbound > 0.0f64 {
                    *upbound = 0.0f64;
                }
            }
        }
    } else {
        if *lobound < get_lowbo(lp, colnr) {
            *lobound = get_lowbo(lp, colnr);
        }
        if *upbound > get_upbo(lp, colnr) {
            *upbound = get_upbo(lp, colnr);
        }
    }
    isneg = (*upbound >= *lobound - epsvalue) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if isneg == 0 {
        if fabs((*lobound - get_upbo(lp, colnr)) / (1.0f64 + fabs(get_upbo(lp, colnr))))
            < PRESOLVE_BOUNDSLACK as ::core::ffi::c_double * epsvalue
        {
            *lobound = get_upbo(lp, colnr);
        } else if fabs((*upbound - get_lowbo(lp, colnr)) / (1.0f64 + fabs(get_lowbo(lp, colnr))))
            < PRESOLVE_BOUNDSLACK as ::core::ffi::c_double * epsvalue
        {
            *upbound = get_lowbo(lp, colnr);
        }
        isneg = (*upbound >= *lobound - epsvalue) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if isneg == 0 {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"presolve_singletonbounds: Singleton variable %s in row %s infeasibility (%g << %g)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
    return isneg;
}
#[export_name="honest_lpsolve_presolve_altsingletonvalid"]
pub unsafe extern "C" fn presolve_altsingletonvalid(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
    mut reflotest: ::core::ffi::c_double,
    mut refuptest: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut coeff_bl: ::core::ffi::c_double = 0.;
    let mut coeff_bu: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    coeff_bl = get_rh_lower(lp, rownr);
    coeff_bu = get_rh_upper(lp, rownr);
    if reflotest > refuptest + epsvalue
        || presolve_singletonbounds(
            psdata,
            rownr,
            colnr,
            &raw mut coeff_bl,
            &raw mut coeff_bu,
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    epsvalue = (if reflotest - coeff_bu > coeff_bl - refuptest {
        reflotest - coeff_bu
    } else {
        coeff_bl - refuptest
    }) / epsvalue;
    if epsvalue > PRESOLVE_BOUNDSLACK as ::core::ffi::c_double {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"presolve_altsingletonvalid: Singleton variable %s in row %s infeasible (%g)\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    } else {
        return 1 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_presolve_multibounds"]
pub unsafe extern "C" fn presolve_multibounds(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
    mut lobound: *mut ::core::ffi::c_double,
    mut upbound: *mut ::core::ffi::c_double,
    mut aval: *mut ::core::ffi::c_double,
    mut rowbinds: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut rowbindsvar: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut coeff_a: ::core::ffi::c_double = 0.;
    let mut LHS: ::core::ffi::c_double = 0.;
    let mut RHS: ::core::ffi::c_double = 0.;
    let mut netX: ::core::ffi::c_double = 0.;
    let mut Xupper: ::core::ffi::c_double = 0.;
    let mut Xlower: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    LHS = *lobound;
    RHS = *upbound;
    Xlower = get_lowbo(lp, colnr);
    Xupper = get_upbo(lp, colnr);
    if aval.is_null() {
        coeff_a = get_mat(lp, rownr, colnr);
    } else {
        coeff_a = *aval;
    }
    netX = presolve_sumplumin(lp, rownr, (*psdata).rows, TRUE as ::core::ffi::c_uchar);
    if (fabs(LHS) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar == 0
        && (fabs(netX) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar == 0
    {
        if coeff_a > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            LHS -= netX - coeff_a * Xupper;
            LHS /= coeff_a;
            if LHS > Xlower + epsvalue {
                Xlower = presolve_roundrhs(lp, LHS, TRUE as ::core::ffi::c_uchar);
                status = TRUE as ::core::ffi::c_uchar;
            } else if LHS > Xlower - epsvalue {
                rowbindsvar = TRUE as ::core::ffi::c_uchar;
            }
        } else {
            LHS -= netX - coeff_a * Xlower;
            LHS /= coeff_a;
            if LHS < Xupper - epsvalue {
                Xupper = presolve_roundrhs(lp, LHS, FALSE as ::core::ffi::c_uchar);
                status = AUTOMATIC as ::core::ffi::c_uchar;
            } else if LHS < Xupper + epsvalue {
                rowbindsvar = AUTOMATIC as ::core::ffi::c_uchar;
            }
        }
    }
    netX = presolve_sumplumin(lp, rownr, (*psdata).rows, FALSE as ::core::ffi::c_uchar);
    if (fabs(RHS) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar == 0
        && (fabs(netX) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar == 0
    {
        if coeff_a < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            if (fabs(Xupper) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar == 0 {
                RHS -= netX - coeff_a * Xupper;
                RHS /= coeff_a;
                if RHS > Xlower + epsvalue {
                    Xlower = presolve_roundrhs(lp, RHS, TRUE as ::core::ffi::c_uchar);
                    status = (status as ::core::ffi::c_int | TRUE) as ::core::ffi::c_uchar;
                } else if RHS > Xlower - epsvalue {
                    rowbindsvar =
                        (rowbindsvar as ::core::ffi::c_int | TRUE) as ::core::ffi::c_uchar;
                }
            }
        } else if (fabs(Xlower) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
            == 0
        {
            RHS -= netX - coeff_a * Xlower;
            RHS /= coeff_a;
            if RHS < Xupper - epsvalue {
                Xupper = presolve_roundrhs(lp, RHS, FALSE as ::core::ffi::c_uchar);
                status = (status as ::core::ffi::c_int | AUTOMATIC) as ::core::ffi::c_uchar;
            } else if RHS < Xupper + epsvalue {
                rowbindsvar =
                    (rowbindsvar as ::core::ffi::c_int | AUTOMATIC) as ::core::ffi::c_uchar;
            }
        }
    }
    *lobound = Xlower;
    *upbound = Xupper;
    if !rowbinds.is_null() {
        *rowbinds = rowbindsvar;
    }
    return status;
}
#[export_name="honest_lpsolve_isnz_origobj"]
pub unsafe extern "C" fn isnz_origobj(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (*(*lp).orig_obj.offset(colnr as isize)
        != 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_testrow"]
pub unsafe extern "C" fn presolve_testrow(
    mut psdata: *mut presolverec,
    mut lastrow: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if (*psdata).forceupdate != 0 {
        presolve_updatesums(psdata);
        (*psdata).forceupdate = FALSE as ::core::ffi::c_uchar;
    }
    if presolve_rowfeasible(
        psdata,
        0 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    } else {
        return 1 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_presolve_coltighten"]
pub unsafe extern "C" fn presolve_coltighten(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
    mut LOnew: ::core::ffi::c_double,
    mut UPnew: ::core::ffi::c_double,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut elmend: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut oldcount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut newcount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut deltainf: ::core::ffi::c_int = 0;
    let mut LOold: ::core::ffi::c_double = 0.;
    let mut UPold: ::core::ffi::c_double = 0.;
    let mut Value: ::core::ffi::c_double = 0.;
    let mut margin: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    Value = UPnew - LOnew;
    if Value <= -margin && Value > -(*lp).epsprimal {
        if fabs(fmod(UPnew, 1.0f64)) < margin {
            LOnew = UPnew;
        } else {
            UPnew = LOnew;
        }
    }
    LOold = get_lowbo(lp, colnr);
    UPold = get_upbo(lp, colnr);
    if !count.is_null() {
        newcount = *count;
    }
    oldcount = newcount;
    deltainf = 0 as ::core::ffi::c_int;
    if UPold < (*lp).infinite || LOold > -(*lp).infinite {
        deltainf -= 1 as ::core::ffi::c_int;
    }
    if UPnew < (*lp).infinite || LOnew > -(*lp).infinite {
        deltainf += 1 as ::core::ffi::c_int;
    }
    if isnz_origobj(lp, colnr) != 0 {
        *(*(*psdata).rows)
            .infcount
            .offset(0 as ::core::ffi::c_int as isize) += deltainf;
    }
    elmnr = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    elmend = *(*mat).col_end.offset(colnr as isize);
    rownr = (*mat).col_mat_rownr.offset(elmnr as isize) as *mut ::core::ffi::c_int;
    while elmnr < elmend {
        k = *rownr;
        if isActiveLink((*(*psdata).rows).varmap, k) != 0 {
            *(*(*psdata).rows).infcount.offset(k as isize) += deltainf;
        }
        elmnr += 1;
        rownr = rownr.offset(matRowColStep as isize);
    }
    if UPnew < (*lp).infinite && UPnew + margin < UPold {
        if is_int(lp, colnr) != 0 {
            UPnew = floor(UPnew + margin);
        }
        if UPold < (*lp).infinite {
            k = 0 as ::core::ffi::c_int;
            Value = if is_chsign(lp, k) as ::core::ffi::c_int != 0
                && *(*lp).orig_obj.offset(colnr as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -*(*lp).orig_obj.offset(colnr as isize)
            } else {
                *(*lp).orig_obj.offset(colnr as isize)
            };
            if Value > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && *(*(*psdata).rows).pluupper.offset(k as isize) < (*lp).infinite
            {
                *(*(*psdata).rows).pluupper.offset(k as isize) += (UPnew - UPold) * Value;
            } else if Value < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && *(*(*psdata).rows).negupper.offset(k as isize) < (*lp).infinite
            {
                *(*(*psdata).rows).negupper.offset(k as isize) += (LOnew - LOold) * Value;
            }
            *(*(*psdata).rows).infcount.offset(k as isize) += deltainf;
            elmnr = *(*mat)
                .col_end
                .offset((colnr - 1 as ::core::ffi::c_int) as isize);
            elmend = *(*mat).col_end.offset(colnr as isize);
            rownr = (*mat).col_mat_rownr.offset(elmnr as isize) as *mut ::core::ffi::c_int;
            value = (*mat).col_mat_value.offset(elmnr as isize) as *mut ::core::ffi::c_double;
            while elmnr < elmend {
                k = *rownr;
                if !(isActiveLink((*(*psdata).rows).varmap, k) == 0) {
                    Value = if is_chsign(lp, k) as ::core::ffi::c_int != 0
                        && *value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -*value
                    } else {
                        *value
                    };
                    if Value > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && *(*(*psdata).rows).pluupper.offset(k as isize) < (*lp).infinite
                    {
                        *(*(*psdata).rows).pluupper.offset(k as isize) += (UPnew - UPold) * Value;
                    } else if Value < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && *(*(*psdata).rows).negupper.offset(k as isize) < (*lp).infinite
                    {
                        *(*(*psdata).rows).negupper.offset(k as isize) += (LOnew - LOold) * Value;
                    }
                }
                elmnr += 1;
                rownr = rownr.offset(matRowColStep as isize);
                value = value.offset(matValueStep as isize);
            }
        } else {
            (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
        }
        if UPnew < UPold {
            UPold = UPnew;
            newcount += 1;
        }
    }
    if LOnew > -(*lp).infinite && LOnew - margin > LOold {
        if is_int(lp, colnr) != 0 {
            LOnew = ceil(LOnew - margin);
        }
        if LOold > -(*lp).infinite {
            k = 0 as ::core::ffi::c_int;
            Value = if is_chsign(lp, k) as ::core::ffi::c_int != 0
                && *(*lp).orig_obj.offset(colnr as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -*(*lp).orig_obj.offset(colnr as isize)
            } else {
                *(*lp).orig_obj.offset(colnr as isize)
            };
            if Value > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && *(*(*psdata).rows).plulower.offset(k as isize) > -(*lp).infinite
            {
                *(*(*psdata).rows).plulower.offset(k as isize) += (LOnew - LOold) * Value;
            } else if Value < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && *(*(*psdata).rows).neglower.offset(k as isize) > -(*lp).infinite
            {
                *(*(*psdata).rows).neglower.offset(k as isize) += (UPnew - UPold) * Value;
            }
            elmnr = *(*mat)
                .col_end
                .offset((colnr - 1 as ::core::ffi::c_int) as isize);
            elmend = *(*mat).col_end.offset(colnr as isize);
            rownr = (*mat).col_mat_rownr.offset(elmnr as isize) as *mut ::core::ffi::c_int;
            value = (*mat).col_mat_value.offset(elmnr as isize) as *mut ::core::ffi::c_double;
            while elmnr < elmend {
                k = *rownr;
                if !(isActiveLink((*(*psdata).rows).varmap, k) == 0) {
                    Value = if is_chsign(lp, k) as ::core::ffi::c_int != 0
                        && *value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -*value
                    } else {
                        *value
                    };
                    if Value > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && *(*(*psdata).rows).plulower.offset(k as isize) > -(*lp).infinite
                    {
                        *(*(*psdata).rows).plulower.offset(k as isize) += (LOnew - LOold) * Value;
                    } else if Value < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && *(*(*psdata).rows).neglower.offset(k as isize) > -(*lp).infinite
                    {
                        *(*(*psdata).rows).neglower.offset(k as isize) += (UPnew - UPold) * Value;
                    }
                }
                elmnr += 1;
                rownr = rownr.offset(matRowColStep as isize);
                value = value.offset(matValueStep as isize);
            }
        } else {
            (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
        }
        if LOnew > LOold {
            LOold = LOnew;
            newcount += 1;
        }
    }
    if newcount > oldcount {
        UPnew = presolve_roundval(lp, UPnew);
        LOnew = presolve_roundval(lp, LOnew);
        if LOnew > UPnew {
            if LOnew - UPnew < margin {
                LOnew = UPnew;
            } else {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"presolve_coltighten: Found column %s with LB %g > UB %g\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_uchar;
            }
        }
        if (*lp).spx_trace as ::core::ffi::c_int != 0 || (*lp).verbose > DETAILED {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"presolve_coltighten: Replaced bounds on column %s to [%g ... %g]\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        set_bounds(lp, colnr, LOnew, UPnew);
    }
    if !count.is_null() {
        *count = newcount;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_rowtighten"]
pub unsafe extern "C" fn presolve_rowtighten(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut tally: *mut ::core::ffi::c_int,
    mut intsonly: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut rowbinds: ::core::ffi::c_uchar = 0;
    let mut item: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jx: ::core::ffi::c_int = 0;
    let mut jjx: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut idxn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idxbound: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut newbound: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut RHlo: ::core::ffi::c_double = get_rh_lower(lp, rownr);
    let mut RHup: ::core::ffi::c_double = get_rh_upper(lp, rownr);
    let mut VARlo: ::core::ffi::c_double = 0.;
    let mut VARup: ::core::ffi::c_double = 0.;
    let mut Aval: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    jx = presolve_rowlength(psdata, rownr);
    allocREAL(
        lp,
        &raw mut newbound,
        2 as ::core::ffi::c_int * jx,
        TRUE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut idxbound,
        2 as ::core::ffi::c_int * jx,
        TRUE as ::core::ffi::c_uchar,
    );
    jx = presolve_nextcol(psdata, rownr, &raw mut item);
    while jx >= 0 as ::core::ffi::c_int {
        jjx = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(jx as isize) as isize);
        Aval = *(*mat)
            .col_mat_value
            .offset(*(*mat).row_mat.offset(jx as isize) as isize);
        Aval = if rownr != 0 && Aval != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -Aval
        } else {
            Aval
        };
        VARlo = RHlo;
        VARup = RHup;
        presolve_multibounds(
            psdata,
            rownr,
            jjx,
            &raw mut VARlo,
            &raw mut VARup,
            &raw mut Aval,
            &raw mut rowbinds,
        );
        if rowbinds as ::core::ffi::c_int & TRUE != 0 {
            *idxbound.offset(idxn as isize) = -jjx;
            *newbound.offset(idxn as isize) = VARlo;
            idxn += 1;
        }
        if rowbinds as ::core::ffi::c_int & AUTOMATIC != 0 {
            *idxbound.offset(idxn as isize) = jjx;
            *newbound.offset(idxn as isize) = VARup;
            idxn += 1;
        }
        jx = presolve_nextcol(psdata, rownr, &raw mut item);
    }
    ix = 0 as ::core::ffi::c_int;
    while ix < idxn {
        jjx = *idxbound.offset(ix as isize);
        jx = abs(jjx);
        if is_unbounded(lp, jx) as ::core::ffi::c_int != 0
            || intsonly as ::core::ffi::c_int != 0 && is_int(lp, jx) == 0
        {
            continue;
        }
        VARlo = get_lowbo(lp, jx);
        VARup = get_upbo(lp, jx);
        while ix < idxn && {
            jjx = *idxbound.offset(ix as isize);
            jx == abs(jjx)
        } {
            if jjx < 0 as ::core::ffi::c_int {
                VARlo = *newbound.offset(ix as isize);
            } else {
                VARup = *newbound.offset(ix as isize);
            }
            ix += 1;
        }
        if !(presolve_coltighten(psdata, jx, VARlo, VARup, tally) == 0) {
            continue;
        }
        status = presolve_setstatusex(
            psdata,
            2 as ::core::ffi::c_int,
            1588 as ::core::ffi::c_int,
            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        break;
    }
    if !(newbound as *mut ::core::ffi::c_void).is_null() {
        free(newbound as *mut ::core::ffi::c_void);
        newbound = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(idxbound as *mut ::core::ffi::c_void).is_null() {
        free(idxbound as *mut ::core::ffi::c_void);
        idxbound = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return status;
}
#[export_name="honest_lpsolve_set_dv_bounds"]
pub unsafe extern "C" fn set_dv_bounds(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut lowbo: ::core::ffi::c_double,
    mut upbo: ::core::ffi::c_double,
) {
    *(*psdata).dv_lobo.offset(rownr as isize) = lowbo;
    *(*psdata).dv_upbo.offset(rownr as isize) = upbo;
}
#[export_name="honest_lpsolve_get_dv_lower"]
pub unsafe extern "C" fn get_dv_lower(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    return *(*psdata).dv_lobo.offset(rownr as isize);
}
#[export_name="honest_lpsolve_get_dv_upper"]
pub unsafe extern "C" fn get_dv_upper(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    return *(*psdata).dv_upbo.offset(rownr as isize);
}
#[export_name="honest_lpsolve_presolve_rowfix"]
pub unsafe extern "C" fn presolve_rowfix(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut newvalue: ::core::ffi::c_double,
    mut remove: ::core::ffi::c_uchar,
    mut tally: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut isneg: ::core::ffi::c_uchar = 0;
    let mut lofinite: ::core::ffi::c_uchar = 0;
    let mut upfinite: ::core::ffi::c_uchar = 0;
    let mut doupdate: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut chsign: ::core::ffi::c_uchar = is_chsign(lp, rownr);
    let mut lobound: ::core::ffi::c_double = 0.;
    let mut upbound: ::core::ffi::c_double = 0.;
    let mut lovalue: ::core::ffi::c_double = 0.;
    let mut upvalue: ::core::ffi::c_double = 0.;
    let mut Value: ::core::ffi::c_double = 0.;
    let mut fixvalue: ::core::ffi::c_double = 0.;
    let mut fixprod: ::core::ffi::c_double = 0.;
    let mut mult: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ps: *mut psrec = (*psdata).cols;
    upbound = get_dv_upper(psdata, rownr);
    lobound = get_dv_lower(psdata, rownr);
    if remove != 0 {
        if upbound - lobound < (*psdata).epsvalue {
            if newvalue > lobound && newvalue < upbound {
                fixvalue = newvalue;
            } else {
                fixvalue = lobound;
            }
        } else if (fabs(newvalue) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            != 0
            && get_rh(lp, rownr) == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            fixvalue = if lobound <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && upbound >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else if upbound < lobound {
                upbound
            } else {
                lobound
            };
        } else {
            fixvalue = newvalue;
        }
        set_dv_bounds(psdata, rownr, fixvalue, fixvalue);
        if fixvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            addUndoPresolve(
                lp,
                FALSE as ::core::ffi::c_uchar,
                rownr,
                fixvalue,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int,
            );
        }
        mult = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    } else {
        mult = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        fixvalue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    ix = *(*mat)
        .row_end
        .offset((rownr - 1 as ::core::ffi::c_int) as isize);
    ie = *(*mat).row_end.offset(rownr as isize);
    while ix < ie {
        i = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(ix as isize) as isize);
        Value = *(*mat)
            .col_mat_value
            .offset(*(*mat).row_mat.offset(ix as isize) as isize);
        if !(Value == 0 as ::core::ffi::c_int as ::core::ffi::c_double) {
            if remove as ::core::ffi::c_int != 0
                && fixvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                fixprod = Value * fixvalue;
                *(*lp).orig_obj.offset(i as isize) -= fixprod;
                if fabs(*(*lp).orig_obj.offset(i as isize)) < (*psdata).epsvalue {
                    *(*lp).orig_obj.offset(i as isize) =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                *(*(*lp).presolve_undo).fixed_obj.offset(i as isize) += fixprod;
            }
            Value = if chsign as ::core::ffi::c_int != 0
                && Value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -Value
            } else {
                Value
            };
            isneg = (Value < 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
            if !(isActiveLink((*ps).varmap, i) == 0) {
                if remove != 0 {
                    if isneg != 0 {
                        let ref mut fresh49 = *(*ps).negcount.offset(i as isize);
                        *fresh49 -= 1;
                    } else {
                        let ref mut fresh50 = *(*ps).plucount.offset(i as isize);
                        *fresh50 -= 1;
                    }
                    if lobound < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && upbound >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        let ref mut fresh51 = *(*ps).pluneg.offset(i as isize);
                        *fresh51 -= 1;
                    }
                }
                upfinite = (upbound < (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                lofinite =
                    (lobound > -(*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                if upfinite as ::core::ffi::c_int != 0 || lofinite as ::core::ffi::c_int != 0 {
                    if remove != 0 {
                        let ref mut fresh52 = *(*ps).infcount.offset(i as isize);
                        *fresh52 -= 1;
                    } else {
                        let ref mut fresh53 = *(*ps).infcount.offset(i as isize);
                        *fresh53 += 1;
                    }
                }
                upvalue = if upfinite as ::core::ffi::c_int != 0 {
                    Value * upbound
                } else if isneg as ::core::ffi::c_int != 0
                    && (*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -(*lp).infinite
                } else {
                    (*lp).infinite
                };
                lovalue = if lofinite as ::core::ffi::c_int != 0 {
                    Value * lobound
                } else if isneg as ::core::ffi::c_int != 0
                    && -(*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    --(*lp).infinite
                } else {
                    -(*lp).infinite
                };
                if isneg != 0 {
                    if *(*ps).negupper.offset(i as isize) < (*lp).infinite
                        && lofinite as ::core::ffi::c_int != 0
                    {
                        *(*ps).negupper.offset(i as isize) += mult * lovalue;
                        *(*ps).negupper.offset(i as isize) = presolve_roundrhs(
                            lp,
                            *(*ps).negupper.offset(i as isize),
                            FALSE as ::core::ffi::c_uchar,
                        );
                    } else if remove as ::core::ffi::c_int != 0 && lofinite == 0 {
                        doupdate = TRUE as ::core::ffi::c_uchar;
                    } else {
                        *(*ps).negupper.offset(i as isize) = (*lp).infinite;
                    }
                } else if *(*ps).pluupper.offset(i as isize) < (*lp).infinite
                    && upfinite as ::core::ffi::c_int != 0
                {
                    *(*ps).pluupper.offset(i as isize) += mult * upvalue;
                    *(*ps).pluupper.offset(i as isize) = presolve_roundrhs(
                        lp,
                        *(*ps).pluupper.offset(i as isize),
                        FALSE as ::core::ffi::c_uchar,
                    );
                } else if remove as ::core::ffi::c_int != 0 && upfinite == 0 {
                    doupdate = TRUE as ::core::ffi::c_uchar;
                } else {
                    *(*ps).pluupper.offset(i as isize) = (*lp).infinite;
                }
                if isneg != 0 {
                    if *(*ps).neglower.offset(i as isize) > -(*lp).infinite
                        && upfinite as ::core::ffi::c_int != 0
                    {
                        *(*ps).neglower.offset(i as isize) += mult * upvalue;
                        *(*ps).neglower.offset(i as isize) = presolve_roundrhs(
                            lp,
                            *(*ps).neglower.offset(i as isize),
                            TRUE as ::core::ffi::c_uchar,
                        );
                    } else if remove as ::core::ffi::c_int != 0 && upfinite == 0 {
                        doupdate = TRUE as ::core::ffi::c_uchar;
                    } else {
                        *(*ps).neglower.offset(i as isize) = -(*lp).infinite;
                    }
                } else if *(*ps).plulower.offset(i as isize) > -(*lp).infinite
                    && lofinite as ::core::ffi::c_int != 0
                {
                    *(*ps).plulower.offset(i as isize) += mult * lovalue;
                    *(*ps).plulower.offset(i as isize) = presolve_roundrhs(
                        lp,
                        *(*ps).plulower.offset(i as isize),
                        TRUE as ::core::ffi::c_uchar,
                    );
                } else if remove as ::core::ffi::c_int != 0 && lofinite == 0 {
                    doupdate = TRUE as ::core::ffi::c_uchar;
                } else {
                    *(*ps).plulower.offset(i as isize) = -(*lp).infinite;
                }
                if remove as ::core::ffi::c_int != 0
                    && (i == 0 as ::core::ffi::c_int
                        || *(*(*ps).next.offset(i as isize))
                            .offset(0 as ::core::ffi::c_int as isize)
                            == 1 as ::core::ffi::c_int)
                    && (*psdata).forceupdate == 0
                {
                    presolve_range(lp, i, ps, &raw mut lovalue, &raw mut upvalue);
                    Value = get_mat(lp, 0 as ::core::ffi::c_int, i);
                    if upvalue < Value || lovalue > Value {
                        report(
                            lp,
                            3 as ::core::ffi::c_int,
                            b"presolve: Row %s (%g << %g) infeasibility in column %s (OF=%g)\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        return 0 as ::core::ffi::c_uchar;
                    }
                }
            }
        }
        ix += 1;
    }
    if remove != 0 {
        (*psdata).forceupdate = ((*psdata).forceupdate as ::core::ffi::c_int
            | doupdate as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        if !tally.is_null() {
            *tally += 1;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_colsingleton"]
pub unsafe extern "C" fn presolve_colsingleton(
    mut psdata: *mut presolverec,
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut RHlow: ::core::ffi::c_double = 0.;
    let mut RHup: ::core::ffi::c_double = 0.;
    let mut LObound: ::core::ffi::c_double = 0.;
    let mut UPbound: ::core::ffi::c_double = 0.;
    let mut Value: ::core::ffi::c_double = 0.;
    Value = get_mat(lp, i, j);
    if Value == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return 8 as ::core::ffi::c_int;
    }
    LObound = get_lowbo(lp, j);
    UPbound = get_upbo(lp, j);
    if is_semicont(lp, j) as ::core::ffi::c_int != 0 && UPbound > LObound {
        if LObound > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            LObound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else if UPbound < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            UPbound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    RHlow = get_rh_lower(lp, i);
    RHup = get_rh_upper(lp, i);
    if presolve_singletonbounds(psdata, i, j, &raw mut RHlow, &raw mut RHup, &raw mut Value) == 0 {
        return presolve_setstatusex(
            psdata,
            2 as ::core::ffi::c_int,
            1793 as ::core::ffi::c_int,
            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if presolve_coltighten(psdata, j, RHlow, RHup, count) != 0 {
        return 8 as ::core::ffi::c_int;
    } else {
        return presolve_setstatusex(
            psdata,
            2 as ::core::ffi::c_int,
            1798 as ::core::ffi::c_int,
            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    };
}
#[export_name="honest_lpsolve_presolve_colfix"]
pub unsafe extern "C" fn presolve_colfix(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
    mut newvalue: ::core::ffi::c_double,
    mut remove: ::core::ffi::c_uchar,
    mut tally: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut isneg: ::core::ffi::c_uchar = 0;
    let mut lofinite: ::core::ffi::c_uchar = 0;
    let mut upfinite: ::core::ffi::c_uchar = 0;
    let mut doupdate: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut doOF: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut lobound: ::core::ffi::c_double = 0.;
    let mut upbound: ::core::ffi::c_double = 0.;
    let mut lovalue: ::core::ffi::c_double = 0.;
    let mut upvalue: ::core::ffi::c_double = 0.;
    let mut Value: ::core::ffi::c_double = 0.;
    let mut fixvalue: ::core::ffi::c_double = 0.;
    let mut mult: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ps: *mut psrec = (*psdata).rows;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    upbound = get_upbo(lp, colnr);
    lobound = get_lowbo(lp, colnr);
    if remove != 0 {
        if upbound - lobound < (*psdata).epsvalue {
            if newvalue > lobound && newvalue < upbound {
                fixvalue = newvalue;
            } else {
                fixvalue = lobound;
            }
        } else if (fabs(newvalue) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            != 0
            && get_mat(lp, 0 as ::core::ffi::c_int, colnr)
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            fixvalue = if lobound <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && upbound >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else if upbound < lobound {
                upbound
            } else {
                lobound
            };
        } else {
            fixvalue = newvalue;
        }
        set_bounds(lp, colnr, fixvalue, fixvalue);
        if fixvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            addUndoPresolve(
                lp,
                TRUE as ::core::ffi::c_uchar,
                colnr,
                fixvalue,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int,
            );
        }
        mult = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    } else {
        mult = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        fixvalue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if is_semicont(lp, colnr) as ::core::ffi::c_int != 0 && upbound > lobound {
        if lobound > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            lobound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else if upbound < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            upbound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    ix = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    ie = *(*mat).col_end.offset(colnr as isize);
    rownr = (*mat).col_mat_rownr.offset(ix as isize) as *mut ::core::ffi::c_int;
    value = (*mat).col_mat_value.offset(ix as isize) as *mut ::core::ffi::c_double;
    while doOF as ::core::ffi::c_int != 0 || ix < ie {
        loop {
            if doOF != 0 {
                i = 0 as ::core::ffi::c_int;
                Value = *(*lp).orig_obj.offset(colnr as isize);
            } else {
                i = *rownr;
                Value = *value;
                if isActiveLink((*ps).varmap, i) == 0 {
                    break;
                }
            }
            if !(Value == 0 as ::core::ffi::c_int as ::core::ffi::c_double) {
                if remove as ::core::ffi::c_int != 0
                    && fixvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    presolve_adjustrhs(psdata, i, Value * fixvalue, (*psdata).epsvalue);
                }
                Value = if is_chsign(lp, i) as ::core::ffi::c_int != 0
                    && Value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -Value
                } else {
                    Value
                };
                isneg = (Value < 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as ::core::ffi::c_uchar;
                if remove as ::core::ffi::c_int == TRUE {
                    if isneg != 0 {
                        let ref mut fresh0 = *(*ps).negcount.offset(i as isize);
                        *fresh0 -= 1;
                    } else {
                        let ref mut fresh1 = *(*ps).plucount.offset(i as isize);
                        *fresh1 -= 1;
                    }
                    if lobound < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && upbound >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        let ref mut fresh2 = *(*ps).pluneg.offset(i as isize);
                        *fresh2 -= 1;
                    }
                }
                upfinite = (upbound < (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                lofinite =
                    (lobound > -(*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                if upfinite as ::core::ffi::c_int != 0 || lofinite as ::core::ffi::c_int != 0 {
                    if remove != 0 {
                        let ref mut fresh3 = *(*ps).infcount.offset(i as isize);
                        *fresh3 -= 1;
                    } else {
                        let ref mut fresh4 = *(*ps).infcount.offset(i as isize);
                        *fresh4 += 1;
                    }
                }
                upvalue = if upfinite as ::core::ffi::c_int != 0 {
                    Value * upbound
                } else if isneg as ::core::ffi::c_int != 0
                    && (*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -(*lp).infinite
                } else {
                    (*lp).infinite
                };
                lovalue = if lofinite as ::core::ffi::c_int != 0 {
                    Value * lobound
                } else if isneg as ::core::ffi::c_int != 0
                    && -(*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    --(*lp).infinite
                } else {
                    -(*lp).infinite
                };
                if isneg != 0 {
                    if *(*ps).negupper.offset(i as isize) < (*lp).infinite
                        && lofinite as ::core::ffi::c_int != 0
                    {
                        *(*ps).negupper.offset(i as isize) += mult * lovalue;
                        *(*ps).negupper.offset(i as isize) = presolve_roundrhs(
                            lp,
                            *(*ps).negupper.offset(i as isize),
                            FALSE as ::core::ffi::c_uchar,
                        );
                    } else if remove as ::core::ffi::c_int != 0 && lofinite == 0 {
                        doupdate = TRUE as ::core::ffi::c_uchar;
                    } else {
                        *(*ps).negupper.offset(i as isize) = (*lp).infinite;
                    }
                } else if *(*ps).pluupper.offset(i as isize) < (*lp).infinite
                    && upfinite as ::core::ffi::c_int != 0
                {
                    *(*ps).pluupper.offset(i as isize) += mult * upvalue;
                    *(*ps).pluupper.offset(i as isize) = presolve_roundrhs(
                        lp,
                        *(*ps).pluupper.offset(i as isize),
                        FALSE as ::core::ffi::c_uchar,
                    );
                } else if remove as ::core::ffi::c_int != 0 && upfinite == 0 {
                    doupdate = TRUE as ::core::ffi::c_uchar;
                } else {
                    *(*ps).pluupper.offset(i as isize) = (*lp).infinite;
                }
                if isneg != 0 {
                    if *(*ps).neglower.offset(i as isize) > -(*lp).infinite
                        && upfinite as ::core::ffi::c_int != 0
                    {
                        *(*ps).neglower.offset(i as isize) += mult * upvalue;
                        *(*ps).neglower.offset(i as isize) = presolve_roundrhs(
                            lp,
                            *(*ps).neglower.offset(i as isize),
                            TRUE as ::core::ffi::c_uchar,
                        );
                    } else if remove as ::core::ffi::c_int != 0 && upfinite == 0 {
                        doupdate = TRUE as ::core::ffi::c_uchar;
                    } else {
                        *(*ps).neglower.offset(i as isize) = -(*lp).infinite;
                    }
                } else if *(*ps).plulower.offset(i as isize) > -(*lp).infinite
                    && lofinite as ::core::ffi::c_int != 0
                {
                    *(*ps).plulower.offset(i as isize) += mult * lovalue;
                    *(*ps).plulower.offset(i as isize) = presolve_roundrhs(
                        lp,
                        *(*ps).plulower.offset(i as isize),
                        TRUE as ::core::ffi::c_uchar,
                    );
                } else if remove as ::core::ffi::c_int != 0 && lofinite == 0 {
                    doupdate = TRUE as ::core::ffi::c_uchar;
                } else {
                    *(*ps).plulower.offset(i as isize) = -(*lp).infinite;
                }
                if remove as ::core::ffi::c_int != 0
                    && (i == 0 as ::core::ffi::c_int
                        || *(*(*ps).next.offset(i as isize))
                            .offset(0 as ::core::ffi::c_int as isize)
                            == 1 as ::core::ffi::c_int)
                    && (*psdata).forceupdate == 0
                {
                    if i == 0 as ::core::ffi::c_int {
                        lovalue = get_rh_lower(lp, i);
                        upvalue = get_rh_upper(lp, i);
                        report(
                            lp,
                            5 as ::core::ffi::c_int,
                            b"presolve_colfix: Objective determined by presolve as %18g\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else {
                        presolve_range(lp, i, ps, &raw mut lovalue, &raw mut upvalue);
                        Value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        if upvalue < get_rh_lower(lp, i) - Value
                            || lovalue > get_rh_upper(lp, i) + Value
                        {
                            report(
                                lp,
                                4 as ::core::ffi::c_int,
                                b"presolve_colfix: Variable %s (%g << %g) infeasibility in row %s (%g << %g)\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            return 0 as ::core::ffi::c_uchar;
                        }
                    }
                }
            }
            if !(doOF != 0) {
                break;
            }
            doOF = FALSE as ::core::ffi::c_uchar;
            if !(ix < ie) {
                break;
            }
        }
        ix += 1;
        rownr = rownr.offset(matRowColStep as isize);
        value = value.offset(matValueStep as isize);
    }
    if remove != 0 {
        (*psdata).forceupdate = ((*psdata).forceupdate as ::core::ffi::c_int
            | doupdate as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        if !tally.is_null() {
            *tally += 1;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_rowfixzero"]
pub unsafe extern "C" fn presolve_rowfixzero(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut nv: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ix: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = *(*mat)
        .row_end
        .offset((rownr - 1 as ::core::ffi::c_int) as isize);
    ix = *(*mat).row_end.offset(rownr as isize) - 1 as ::core::ffi::c_int;
    while ix >= ib {
        jx = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(ix as isize) as isize);
        if isActiveLink((*(*psdata).cols).varmap, jx) != 0 {
            if presolve_colfix(psdata, jx, 0.0f64, TRUE as ::core::ffi::c_uchar, nv) == 0 {
                return presolve_setstatusex(
                    psdata,
                    2 as ::core::ffi::c_int,
                    2004 as ::core::ffi::c_int,
                    b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if presolve_candeletevar(psdata, jx) != 0 {
                presolve_colremove(psdata, jx, TRUE as ::core::ffi::c_uchar);
            }
        }
        ix -= 1;
    }
    return 8 as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_presolve_colfixdual"]
pub unsafe extern "C" fn presolve_colfixdual(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
    mut fixValue: *mut ::core::ffi::c_double,
    mut status: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut hasOF: ::core::ffi::c_uchar = 0;
    let mut isDualFREE: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut signOF: ::core::ffi::c_int = 0;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut loX: ::core::ffi::c_double = 0.;
    let mut upX: ::core::ffi::c_double = 0.;
    let mut eps: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut mat: *mut MATrec = (*lp).matA;
    loX = get_lowbo(lp, colnr);
    upX = get_upbo(lp, colnr);
    if loX < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && upX > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        || fabs(upX - loX) < (*lp).epsvalue
        || SOS_is_member_of_type((*lp).SOS, colnr, SOSn) as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    ix = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    ie = *(*mat).col_end.offset(colnr as isize);
    rownr = (*mat).col_mat_rownr.offset(ix as isize) as *mut ::core::ffi::c_int;
    value = (*mat).col_mat_value.offset(ix as isize) as *mut ::core::ffi::c_double;
    hasOF = isnz_origobj(lp, colnr);
    if hasOF != 0 {
        signOF = if *(*lp).orig_obj.offset(colnr as isize)
            < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
    } else {
        signOF = 0 as ::core::ffi::c_int;
    }
    while ix < ie && isDualFREE as ::core::ffi::c_int != 0 {
        i = *rownr;
        if !(isActiveLink((*(*psdata).rows).varmap, i) == 0) {
            if presolve_rowlength(psdata, i) == 1 as ::core::ffi::c_int {
                let mut val: ::core::ffi::c_double = if is_chsign(lp, i) as ::core::ffi::c_int != 0
                    && *value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -*value
                } else {
                    *value
                };
                let mut loR: ::core::ffi::c_double = get_rh_lower(lp, i);
                let mut upR: ::core::ffi::c_double = get_rh_upper(lp, i);
                if presolve_singletonbounds(
                    psdata,
                    i,
                    colnr,
                    &raw mut loR,
                    &raw mut upR,
                    &raw mut val,
                ) == 0
                {
                    *status = presolve_setstatusex(
                        psdata,
                        2 as ::core::ffi::c_int,
                        2056 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    return 0 as ::core::ffi::c_uchar;
                }
                if loR > loX + (*psdata).epsvalue {
                    loX = presolve_roundrhs(lp, loR, TRUE as ::core::ffi::c_uchar);
                }
                if upR < upX - (*psdata).epsvalue {
                    upX = presolve_roundrhs(lp, upR, FALSE as ::core::ffi::c_uchar);
                }
            } else {
                isDualFREE = ((fabs(get_rh_range(lp, i)) >= (*lp).infinite) as ::core::ffi::c_int
                    as ::core::ffi::c_uchar as ::core::ffi::c_int
                    != 0
                    || presolve_sumplumin(lp, i, (*psdata).rows, TRUE as ::core::ffi::c_uchar)
                        - eps
                        <= get_rh_upper(lp, i)
                        && presolve_sumplumin(lp, i, (*psdata).rows, FALSE as ::core::ffi::c_uchar)
                            + eps
                            >= get_rh_lower(lp, i))
                    as ::core::ffi::c_int as ::core::ffi::c_uchar;
                if isDualFREE != 0 {
                    if signOF == 0 as ::core::ffi::c_int {
                        signOF = if *value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            1 as ::core::ffi::c_int
                        };
                    } else {
                        isDualFREE = (signOF
                            == (if *value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                -(1 as ::core::ffi::c_int)
                            } else {
                                1 as ::core::ffi::c_int
                            })) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar;
                    }
                }
            }
        }
        ix += 1;
        rownr = rownr.offset(matRowColStep as isize);
        value = value.offset(matValueStep as isize);
    }
    if isDualFREE != 0 {
        if signOF == 0 as ::core::ffi::c_int {
            if loX < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                loX = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            *fixValue = if loX < upX { loX } else { upX };
        } else if signOF > 0 as ::core::ffi::c_int {
            if (fabs(loX) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar != 0 {
                isDualFREE = FALSE as ::core::ffi::c_uchar;
            } else if is_int(lp, colnr) != 0 {
                *fixValue = ceil(loX - 0.1f64 * (*lp).epsprimal);
            } else {
                *fixValue = loX;
            }
        } else if (fabs(upX) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar != 0 {
            isDualFREE = FALSE as ::core::ffi::c_uchar;
        } else if is_int(lp, colnr) as ::core::ffi::c_int != 0
            && upX != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *fixValue = floor(upX + 0.1f64 * (*lp).epsprimal);
        } else {
            *fixValue = upX;
        }
        if *fixValue != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, colnr) != 0
        {
            return 0 as ::core::ffi::c_uchar;
        }
    }
    return isDualFREE;
}
#[export_name="honest_lpsolve_presolve_probefix01"]
pub unsafe extern "C" fn presolve_probefix01(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
    mut fixvalue: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut loLim: ::core::ffi::c_double = 0.;
    let mut upLim: ::core::ffi::c_double = 0.;
    let mut range: ::core::ffi::c_double = 0.;
    let mut absvalue: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut tolgap: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut chsign: ::core::ffi::c_uchar = 0;
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if is_binary(lp, colnr) == 0 {
        return status;
    }
    item = 0 as ::core::ffi::c_int;
    ix = presolve_nextrow(psdata, colnr, &raw mut item);
    while ix >= 0 as ::core::ffi::c_int {
        i = *(*mat).col_mat_rownr.offset(ix as isize);
        *fixvalue = *(*mat).col_mat_value.offset(ix as isize);
        absvalue = fabs(*fixvalue);
        if absvalue > 100 as ::core::ffi::c_int as ::core::ffi::c_double {
            absvalue = 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        tolgap = epsvalue
            * (if 1 as ::core::ffi::c_int as ::core::ffi::c_double > absvalue {
                1 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                absvalue
            });
        chsign = is_chsign(lp, i);
        loLim = presolve_sumplumin(lp, i, (*psdata).rows, FALSE as ::core::ffi::c_uchar);
        upLim = presolve_sumplumin(lp, i, (*psdata).rows, TRUE as ::core::ffi::c_uchar);
        if chsign != 0 {
            loLim = if chsign as ::core::ffi::c_int != 0
                && loLim != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -loLim
            } else {
                loLim
            };
            upLim = if chsign as ::core::ffi::c_int != 0
                && upLim != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -upLim
            } else {
                upLim
            };
            swapREAL(&raw mut loLim, &raw mut upLim);
        }
        if loLim + *fixvalue > *(*lp).orig_rhs.offset(i as isize) + tolgap {
            if *fixvalue < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                presolve_setstatusex(
                    psdata,
                    2 as ::core::ffi::c_int,
                    2194 as ::core::ffi::c_int,
                    b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            *fixvalue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            break;
        } else {
            range = get_rh_range(lp, i);
            if (fabs(range) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar == 0
                && upLim + *fixvalue < *(*lp).orig_rhs.offset(i as isize) - range - tolgap
            {
                if *fixvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    presolve_setstatusex(
                        psdata,
                        2 as ::core::ffi::c_int,
                        2204 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                *fixvalue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                break;
            } else {
                if !(*(*(*psdata).rows).infcount.offset(i as isize) >= 1 as ::core::ffi::c_int) {
                    if *fixvalue < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && upLim + *fixvalue >= loLim - tolgap
                        && upLim > *(*lp).orig_rhs.offset(i as isize) + tolgap
                        || *fixvalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            && loLim + *fixvalue <= upLim + tolgap
                            && loLim < *(*lp).orig_rhs.offset(i as isize) - range - tolgap
                            && (fabs(range) >= (*lp).infinite) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                == 0
                    {
                        *fixvalue = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                        break;
                    }
                }
                ix = presolve_nextrow(psdata, colnr, &raw mut item);
            }
        }
    }
    status = (ix >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    return status;
}
#[export_name="honest_lpsolve_presolve_probetighten01"]
pub unsafe extern "C" fn presolve_probetighten01(
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut chsign: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut upLim: ::core::ffi::c_double = 0.;
    let mut value: ::core::ffi::c_double = 0.;
    let mut absvalue: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut mat: *mut MATrec = (*lp).matA;
    item = 0 as ::core::ffi::c_int;
    ix = presolve_nextrow(psdata, colnr, &raw mut item);
    while ix >= 0 as ::core::ffi::c_int {
        i = *(*mat).col_mat_rownr.offset(ix as isize);
        value = *(*mat).col_mat_value.offset(ix as isize);
        chsign = is_chsign(lp, i);
        upLim = presolve_sumplumin(
            lp,
            i,
            (*psdata).rows,
            (chsign == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
        );
        upLim = if chsign as ::core::ffi::c_int != 0
            && upLim != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -upLim
        } else {
            upLim
        };
        absvalue = fabs(value);
        if upLim - absvalue
            < *(*lp).orig_rhs.offset(i as isize)
                - epsvalue
                    * (if 1 as ::core::ffi::c_int as ::core::ffi::c_double > absvalue {
                        1 as ::core::ffi::c_int as ::core::ffi::c_double
                    } else {
                        absvalue
                    })
        {
            let mut delta: ::core::ffi::c_double = *(*lp).orig_rhs.offset(i as isize) - upLim;
            *(*lp).orig_rhs.offset(i as isize) = upLim;
            upLim = value
                - (if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && delta != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -delta
                } else {
                    delta
                });
            *(*mat).col_mat_value.offset(ix as isize) = upLim;
            if (if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) != (if upLim < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) {
                if chsign != 0 {
                    let ref mut fresh54 = *(*(*psdata).rows).negcount.offset(i as isize);
                    *fresh54 -= 1;
                    let ref mut fresh55 = *(*(*psdata).rows).plucount.offset(i as isize);
                    *fresh55 += 1;
                } else {
                    let ref mut fresh56 = *(*(*psdata).rows).negcount.offset(i as isize);
                    *fresh56 += 1;
                    let ref mut fresh57 = *(*(*psdata).rows).plucount.offset(i as isize);
                    *fresh57 -= 1;
                }
            }
            n += 1;
        }
        ix = presolve_nextrow(psdata, colnr, &raw mut item);
    }
    return n;
}
#[export_name="honest_lpsolve_presolve_mergerows"]
pub unsafe extern "C" fn presolve_mergerows(
    mut psdata: *mut presolverec,
    mut nRows: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut candelete: ::core::ffi::c_uchar = 0;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut item1: ::core::ffi::c_int = 0;
    let mut item2: ::core::ffi::c_int = 0;
    let mut firstix: ::core::ffi::c_int = 0;
    let mut RT1: ::core::ffi::c_int = 0;
    let mut RT2: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jjx: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Value1: ::core::ffi::c_double = 0.;
    let mut Value2: ::core::ffi::c_double = 0.;
    let mut bound: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    i = lastActiveLink((*(*psdata).rows).varmap);
    while i > 0 as ::core::ffi::c_int && status == RUNNING {
        ix = prevActiveLink((*(*psdata).rows).varmap, i);
        if ix == 0 as ::core::ffi::c_int {
            break;
        }
        j = presolve_rowlength(psdata, i);
        if j <= 1 as ::core::ffi::c_int {
            i = ix;
        } else {
            RT2 = 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            firstix = ix;
            RT1 = 0 as ::core::ffi::c_int;
            while ix > 0 as ::core::ffi::c_int && RT1 < RT2 && status == RUNNING {
                candelete = FALSE as ::core::ffi::c_uchar;
                if !(presolve_rowlength(psdata, ix) != j) {
                    item1 = 0 as ::core::ffi::c_int;
                    iix = presolve_nextcol(psdata, ix, &raw mut item1);
                    item2 = 0 as ::core::ffi::c_int;
                    jjx = presolve_nextcol(psdata, i, &raw mut item2);
                    if !(*(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(iix as isize) as isize)
                        != *(*mat)
                            .col_mat_colnr
                            .offset(*(*mat).row_mat.offset(jjx as isize) as isize))
                    {
                        Value1 = get_mat_byindex(
                            lp,
                            iix,
                            TRUE as ::core::ffi::c_uchar,
                            FALSE as ::core::ffi::c_uchar,
                        );
                        Value2 = get_mat_byindex(
                            lp,
                            jjx,
                            TRUE as ::core::ffi::c_uchar,
                            FALSE as ::core::ffi::c_uchar,
                        );
                        bound = Value1 / Value2;
                        Value1 = bound;
                        jjx = presolve_nextcol(psdata, i, &raw mut item2);
                        while jjx >= 0 as ::core::ffi::c_int && Value1 == bound {
                            iix = presolve_nextcol(psdata, ix, &raw mut item1);
                            if *(*mat)
                                .col_mat_colnr
                                .offset(*(*mat).row_mat.offset(iix as isize) as isize)
                                != *(*mat)
                                    .col_mat_colnr
                                    .offset(*(*mat).row_mat.offset(jjx as isize) as isize)
                            {
                                break;
                            }
                            Value1 = get_mat_byindex(
                                lp,
                                iix,
                                TRUE as ::core::ffi::c_uchar,
                                FALSE as ::core::ffi::c_uchar,
                            );
                            Value2 = get_mat_byindex(
                                lp,
                                jjx,
                                TRUE as ::core::ffi::c_uchar,
                                FALSE as ::core::ffi::c_uchar,
                            );
                            Value1 = Value1 / Value2;
                            if bound == (*lp).infinite {
                                bound = Value1;
                            } else if fabs(Value1 - bound) > (*psdata).epsvalue {
                                break;
                            }
                            jjx = presolve_nextcol(psdata, i, &raw mut item2);
                        }
                        if jjx < 0 as ::core::ffi::c_int {
                            Value1 = *(*lp).orig_rhs.offset(ix as isize);
                            Value2 = *(*lp).orig_rhs.offset(i as isize) * bound;
                            if fabs(Value1 - Value2) > (*psdata).epsvalue
                                && (get_constr_type(lp, ix) == EQ && get_constr_type(lp, i) == EQ)
                            {
                                report(
                                    lp,
                                    4 as ::core::ffi::c_int,
                                    b"presolve_mergerows: Inconsistent equalities %d and %d found\n\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                status = presolve_setstatusex(
                                    psdata,
                                    2 as ::core::ffi::c_int,
                                    2357 as ::core::ffi::c_int,
                                    b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            } else {
                                if is_chsign(lp, i) as ::core::ffi::c_int
                                    != is_chsign(lp, ix) as ::core::ffi::c_int
                                {
                                    bound = -bound;
                                }
                                Value1 = get_rh_lower(lp, i);
                                if Value1 <= -(*lp).infinite {
                                    Value1 *= (if bound
                                        < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        -(1 as ::core::ffi::c_int)
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })
                                        as ::core::ffi::c_double;
                                } else {
                                    Value1 *= bound;
                                }
                                if fabs(Value1) < (*lp).epsdual {
                                    Value1 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                }
                                Value2 = get_rh_upper(lp, i);
                                if Value2 >= (*lp).infinite {
                                    Value2 *= (if bound
                                        < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        -(1 as ::core::ffi::c_int)
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })
                                        as ::core::ffi::c_double;
                                } else {
                                    Value2 *= bound;
                                }
                                if fabs(Value2) < (*lp).epsdual {
                                    Value2 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                }
                                if bound < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    swapREAL(&raw mut Value1, &raw mut Value2);
                                }
                                bound = get_rh_lower(lp, ix);
                                if Value1 > bound + (*psdata).epsvalue {
                                    set_rh_lower(lp, ix, Value1);
                                } else {
                                    Value1 = bound;
                                }
                                bound = get_rh_upper(lp, ix);
                                if Value2 < bound - (*psdata).epsvalue {
                                    set_rh_upper(lp, ix, Value2);
                                } else {
                                    Value2 = bound;
                                }
                                if fabs(Value2 - Value1) < (*psdata).epsvalue {
                                    presolve_setEQ(psdata, ix);
                                } else if Value2 < Value1 {
                                    status = presolve_setstatusex(
                                        psdata,
                                        2 as ::core::ffi::c_int,
                                        2398 as ::core::ffi::c_int,
                                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                            as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                                candelete = (status == RUNNING) as ::core::ffi::c_int
                                    as ::core::ffi::c_uchar;
                                if candelete == 0 {
                                    report(
                                        lp,
                                        4 as ::core::ffi::c_int,
                                        b"presolve: Range infeasibility found involving rows %s and %s\n\0"
                                            as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                            }
                        }
                        if candelete != 0 {
                            presolve_rowremove(psdata, i, TRUE as ::core::ffi::c_uchar);
                            n += 1;
                            break;
                        }
                    }
                }
                ix = prevActiveLink((*(*psdata).rows).varmap, ix);
                RT1 += 1;
            }
            i = firstix;
        }
    }
    *nRows += n;
    *nSum += n;
    return status;
}
#[export_name="honest_lpsolve_presolve_reduceGCD"]
pub unsafe extern "C" fn presolve_reduceGCD(
    mut psdata: *mut presolverec,
    mut nn: *mut ::core::ffi::c_int,
    mut nb: *mut ::core::ffi::c_int,
    mut nsum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut in_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ib: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut GCDvalue: ::core::ffi::c_longlong = 0;
    let mut Avalue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut Rvalue: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut mat: *mut MATrec = (*lp).matA;
    i = firstActiveLink((*psdata).INTmap);
    while i != 0 as ::core::ffi::c_int {
        jx = *(*mat)
            .row_end
            .offset((i - 1 as ::core::ffi::c_int) as isize);
        je = *(*mat).row_end.offset(i as isize);
        Rvalue = *(*mat)
            .col_mat_value
            .offset(*(*mat).row_mat.offset(jx as isize) as isize);
        GCDvalue = abs(Rvalue as ::core::ffi::c_int) as ::core::ffi::c_longlong;
        jx += 1;
        if jx < je {
            while jx < je && GCDvalue > 1 as ::core::ffi::c_longlong {
                Rvalue = fabs(
                    *(*mat)
                        .col_mat_value
                        .offset(*(*mat).row_mat.offset(jx as isize) as isize),
                );
                GCDvalue = gcd(
                    Rvalue as ::core::ffi::c_longlong,
                    GCDvalue,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ) as ::core::ffi::c_longlong;
                jx += 1;
            }
        }
        if GCDvalue > 1 as ::core::ffi::c_longlong {
            jx = *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            je = *(*mat).row_end.offset(i as isize);
            while jx < je {
                Avalue = (*mat)
                    .col_mat_value
                    .offset(*(*mat).row_mat.offset(jx as isize) as isize)
                    as *mut ::core::ffi::c_double;
                *Avalue /= GCDvalue as ::core::ffi::c_double;
                in_0 += 1;
                jx += 1;
            }
            Rvalue =
                *(*lp).orig_rhs.offset(i as isize) / GCDvalue as ::core::ffi::c_double + epsvalue;
            *(*lp).orig_rhs.offset(i as isize) = floor(Rvalue);
            Rvalue = fabs(*(*lp).orig_rhs.offset(i as isize) - Rvalue);
            if is_constr_type(lp, i, EQ) as ::core::ffi::c_int != 0 && Rvalue > epsvalue {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"presolve_reduceGCD: Infeasible equality constraint %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                status = FALSE as ::core::ffi::c_uchar;
                break;
            } else {
                if (fabs(*(*lp).orig_upbo.offset(i as isize)) >= (*lp).infinite)
                    as ::core::ffi::c_int as ::core::ffi::c_uchar
                    == 0
                {
                    *(*lp).orig_upbo.offset(i as isize) = floor(
                        *(*lp).orig_upbo.offset(i as isize) / GCDvalue as ::core::ffi::c_double,
                    );
                }
                ib += 1;
            }
        }
        i = nextActiveLink((*psdata).INTmap, i);
    }
    if status as ::core::ffi::c_int != 0 && in_0 > 0 as ::core::ffi::c_int {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"presolve_reduceGCD: Did %d constraint coefficient reductions.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    *nn += in_0;
    *nb += ib;
    *nsum += in_0 + ib;
    return status;
}
#[export_name="honest_lpsolve_presolve_knapsack"]
pub unsafe extern "C" fn presolve_knapsack(
    mut psdata: *mut presolverec,
    mut nn: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut colOF: *mut ::core::ffi::c_double = (*lp).orig_obj;
    let mut value: ::core::ffi::c_double = 0.;
    let mut ratio: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut map: *mut LLrec = (*psdata).EQmap;
    let mut mat: *mut MATrec = (*lp).matA;
    m = *(*mat).row_end.offset(0 as ::core::ffi::c_int as isize);
    if (*map).count == 0 as ::core::ffi::c_int || m < 2 as ::core::ffi::c_int {
        return status;
    }
    allocINT(
        lp,
        &raw mut rownr,
        (*map).count + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut ratio,
        (*map).count + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    *rownr.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    i = firstActiveLink(map);
    while i != 0 as ::core::ffi::c_int {
        if !(get_rh(lp, i) <= 0 as ::core::ffi::c_int as ::core::ffi::c_double) {
            jx = *(*mat).row_end.offset(i as isize);
            n = 0 as ::core::ffi::c_int;
            j = *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            while j < jx {
                colnr = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(j as isize) as isize);
                value = *(*mat)
                    .col_mat_value
                    .offset(*(*mat).row_mat.offset(j as isize) as isize);
                if *colOF.offset(colnr as isize) == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    break;
                }
                if n == 0 as ::core::ffi::c_int {
                    *ratio.offset(0 as ::core::ffi::c_int as isize) =
                        *colOF.offset(colnr as isize) / value;
                } else if fabs(
                    value * *ratio.offset(0 as ::core::ffi::c_int as isize)
                        - *colOF.offset(colnr as isize),
                ) > (*psdata).epsvalue
                {
                    n = -(1 as ::core::ffi::c_int);
                    break;
                }
                j += 1;
                n += 1;
            }
            if n >= 2 as ::core::ffi::c_int {
                let ref mut fresh18 = *rownr.offset(0 as ::core::ffi::c_int as isize);
                *fresh18 += 1;
                ix = *fresh18;
                *rownr.offset(ix as isize) = i;
                *ratio.offset(ix as isize) = *ratio.offset(0 as ::core::ffi::c_int as isize);
            }
        }
        i = nextActiveLink(map, i);
    }
    n = *rownr.offset(0 as ::core::ffi::c_int as isize);
    if !(n == 0 as ::core::ffi::c_int) {
        ix = 1 as ::core::ffi::c_int;
        while ix <= n {
            i = *rownr.offset(ix as isize);
            jx = *(*mat).row_end.offset(i as isize);
            j = *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            while j < jx {
                colnr = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(j as isize) as isize);
                *colOF.offset(colnr as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                j += 1;
            }
            ix += 1;
        }
        j = (*lp).columns;
        (*(*psdata).cols).varmap = cloneLink(
            (*(*psdata).cols).varmap,
            j + n,
            TRUE as ::core::ffi::c_uchar,
        );
        (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
        ix = 1 as ::core::ffi::c_int;
        while ix <= n {
            i = *rownr.offset(ix as isize);
            *rownr.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
            *colOF.offset(0 as ::core::ffi::c_int as isize) = if is_maxim(lp) as ::core::ffi::c_int
                != 0
                && *ratio.offset(ix as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -*ratio.offset(ix as isize)
            } else {
                *ratio.offset(ix as isize)
            };
            *rownr.offset(1 as ::core::ffi::c_int as isize) = i;
            *colOF.offset(1 as ::core::ffi::c_int as isize) =
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
            value = get_rh(lp, i);
            add_columnex(lp, 2 as ::core::ffi::c_int, colOF, rownr);
            set_bounds(lp, (*lp).columns, value, value);
            set_rh(lp, i, 0 as ::core::ffi::c_int as ::core::ffi::c_double);
            appendLink((*(*psdata).cols).varmap, j + ix);
            ix += 1;
        }
        presolve_validate(psdata, TRUE as ::core::ffi::c_uchar);
    }
    if !(rownr as *mut ::core::ffi::c_void).is_null() {
        free(rownr as *mut ::core::ffi::c_void);
        rownr = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(ratio as *mut ::core::ffi::c_void).is_null() {
        free(ratio as *mut ::core::ffi::c_void);
        ratio = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    *nn += n;
    return status;
}
#[export_name="honest_lpsolve_presolve_invalideq2"]
pub unsafe extern "C" fn presolve_invalideq2(
    mut lp: *mut lprec,
    mut psdata: *mut presolverec,
) -> ::core::ffi::c_uchar {
    let mut jx: ::core::ffi::c_int = 0;
    let mut jjx: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: ::core::ffi::c_int = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut error: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    loop {
        if i == 0 as ::core::ffi::c_int {
            i = firstActiveLink((*psdata).EQmap);
        } else {
            i = nextActiveLink((*psdata).EQmap, i);
        }
        if i == 0 as ::core::ffi::c_int {
            return error;
        }
        while i > 0 as ::core::ffi::c_int {
            if presolve_rowlength(psdata, i) == 2 as ::core::ffi::c_int {
                break;
            }
            i = nextActiveLink((*psdata).EQmap, i);
        }
        if i == 0 as ::core::ffi::c_int {
            return error;
        }
        item = 0 as ::core::ffi::c_int;
        jx = presolve_nextcol(psdata, i, &raw mut item);
        if jx < 0 as ::core::ffi::c_int {
            error = TRUE as ::core::ffi::c_uchar;
        }
        jx = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(jx as isize) as isize);
        jjx = presolve_nextcol(psdata, i, &raw mut item);
        if jjx < 0 as ::core::ffi::c_int {
            error = AUTOMATIC as ::core::ffi::c_uchar;
        }
        if !(error == 0) {
            break;
        }
    }
    return error;
}
#[export_name="honest_lpsolve_presolve_getcolumnEQ"]
pub unsafe extern "C" fn presolve_getcolumnEQ(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut nzvalues: *mut ::core::ffi::c_double,
    mut nzrows: *mut ::core::ffi::c_int,
    mut mapin: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mat: *mut MATrec = (*lp).matA;
    ib = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    ie = *(*mat).col_end.offset(colnr as isize);
    while ib < ie {
        i = *(*mat).col_mat_rownr.offset(ib as isize);
        if !(is_constr_type(lp, i, EQ) == 0 || *mapin.offset(i as isize) == 0 as ::core::ffi::c_int)
        {
            if !nzvalues.is_null() {
                *nzrows.offset(nn as isize) = *mapin.offset(i as isize);
                *nzvalues.offset(nn as isize) = *(*mat).col_mat_value.offset(ib as isize);
            }
            nn += 1;
        }
        ib += 1;
    }
    return nn;
}
#[export_name="honest_lpsolve_presolve_singularities"]
pub unsafe extern "C" fn presolve_singularities(
    mut psdata: *mut presolverec,
    mut nn: *mut ::core::ffi::c_int,
    mut nr: *mut ::core::ffi::c_int,
    mut nv: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut rmapin: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rmapout: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut cmapout: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if (*lp).bfp_findredundant.expect("non-null function pointer")(
        lp,
        0 as ::core::ffi::c_int,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    allocINT(
        lp,
        &raw mut rmapin,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut rmapout,
        (*(*psdata).EQmap).count + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut cmapout,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    n = 0 as ::core::ffi::c_int;
    i = firstActiveLink((*psdata).EQmap);
    while i != 0 as ::core::ffi::c_int {
        n += 1;
        *rmapout.offset(n as isize) = i;
        *rmapin.offset(i as isize) = n;
        i = nextActiveLink((*psdata).EQmap, i);
    }
    *rmapout.offset(0 as ::core::ffi::c_int as isize) = n;
    n = 0 as ::core::ffi::c_int;
    i = firstActiveLink((*(*psdata).cols).varmap);
    while i != 0 as ::core::ffi::c_int {
        n += 1;
        *cmapout.offset(n as isize) = i;
        i = nextActiveLink((*(*psdata).cols).varmap, i);
    }
    *cmapout.offset(0 as ::core::ffi::c_int as isize) = n;
    n = (*lp).bfp_findredundant.expect("non-null function pointer")(
        lp,
        (*(*psdata).EQmap).count,
        Some(
            presolve_getcolumnEQ
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        rmapin,
        cmapout,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        j = *rmapin.offset(i as isize);
        j = *rmapout.offset(j as isize);
        presolve_rowremove(psdata, j, TRUE as ::core::ffi::c_uchar);
        i += 1;
    }
    *nn += n;
    *nr += n;
    *nSum += n;
    if !(rmapout as *mut ::core::ffi::c_void).is_null() {
        free(rmapout as *mut ::core::ffi::c_void);
        rmapout = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(rmapin as *mut ::core::ffi::c_void).is_null() {
        free(rmapin as *mut ::core::ffi::c_void);
        rmapin = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(cmapout as *mut ::core::ffi::c_void).is_null() {
        free(cmapout as *mut ::core::ffi::c_void);
        cmapout = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return n;
}
#[export_name="honest_lpsolve_presolve_elimeq2"]
pub unsafe extern "C" fn presolve_elimeq2(
    mut psdata: *mut presolverec,
    mut nn: *mut ::core::ffi::c_int,
    mut nr: *mut ::core::ffi::c_int,
    mut nc: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut jjx: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut plucount: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut negcount: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colplu: ::core::ffi::c_int = 0;
    let mut colneg: ::core::ffi::c_int = 0;
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iRowsRemoved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iVarsFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut colindex: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut freshupdate: ::core::ffi::c_uchar = 0;
    let mut Coeff1: ::core::ffi::c_double = 0.;
    let mut Coeff2: ::core::ffi::c_double = 0.;
    let mut Value1: ::core::ffi::c_double = 0.;
    let mut Value2: ::core::ffi::c_double = 0.;
    let mut lobound: ::core::ffi::c_double = 0.;
    let mut upbound: ::core::ffi::c_double = 0.;
    let mut bound: ::core::ffi::c_double = 0.;
    let mut test: ::core::ffi::c_double = 0.;
    let mut product: ::core::ffi::c_double = 0.;
    let mut colvalue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut delvalue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut colitem: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut mat: *mut MATrec = (*lp).matA;
    let mut rev: *mut MATrec = ::core::ptr::null_mut::<MATrec>();
    let mut DV: *mut DeltaVrec = ::core::ptr::null_mut::<DeltaVrec>();
    let mut EQ2: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
    if (*(*psdata).EQmap).count == 0 as ::core::ffi::c_int {
        *nSum = 0 as ::core::ffi::c_int;
        return status;
    }
    createLink(
        (*lp).rows,
        &raw mut EQ2,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    if !(EQ2.is_null()
        || allocREAL(
            lp,
            &raw mut colvalue,
            nrows + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut delvalue,
            nrows + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) == 0)
    {
        i = firstActiveLink((*psdata).EQmap);
        while i > 0 as ::core::ffi::c_int {
            if presolve_rowlength(psdata, i) == 2 as ::core::ffi::c_int {
                appendLink(EQ2, i);
            }
            i = nextActiveLink((*psdata).EQmap, i);
        }
        if !((*EQ2).count == 0 as ::core::ffi::c_int) {
            n = 0 as ::core::ffi::c_int;
            i = firstActiveLink(EQ2);
            while i > 0 as ::core::ffi::c_int {
                if !(presolve_rowlength(psdata, i) != 2 as ::core::ffi::c_int) {
                    item = 0 as ::core::ffi::c_int;
                    jx = presolve_nextcol(psdata, i, &raw mut item);
                    Coeff2 = *(*mat)
                        .col_mat_value
                        .offset(*(*mat).row_mat.offset(jx as isize) as isize);
                    jx = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(jx as isize) as isize);
                    jjx = presolve_nextcol(psdata, i, &raw mut item);
                    Coeff1 = *(*mat)
                        .col_mat_value
                        .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
                    jjx = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
                    if !(fabs(Coeff1) < (*psdata).epspivot * *(*mat).colmax.offset(jx as isize)
                        && (fabs(Coeff1) != 1 as ::core::ffi::c_int as ::core::ffi::c_double
                            && fabs(Coeff2) != 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                        && fabs(Coeff2) < (*psdata).epspivot * *(*mat).colmax.offset(jjx as isize))
                    {
                        if !(is_semicont(lp, jx) as ::core::ffi::c_int != 0
                            && is_semicont(lp, jjx) as ::core::ffi::c_int != 0
                            || SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, jx) != 0
                                && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, jjx) != 0)
                        {
                            k = 0 as ::core::ffi::c_int;
                            if is_int(lp, jx) == 0 && is_int(lp, jjx) as ::core::ffi::c_int != 0 {
                                k += 1 as ::core::ffi::c_int;
                            } else if is_semicont(lp, jx) == 0
                                && is_semicont(lp, jjx) as ::core::ffi::c_int != 0
                            {
                                k += 2 as ::core::ffi::c_int;
                            } else if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, jx) == 0
                                && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, jjx) != 0
                            {
                                k += 4 as ::core::ffi::c_int;
                            }
                            if k == 0 as ::core::ffi::c_int {
                                if is_int(lp, jx) as ::core::ffi::c_int != 0 && is_int(lp, jjx) == 0
                                {
                                    k += 8 as ::core::ffi::c_int;
                                } else if is_semicont(lp, jx) as ::core::ffi::c_int != 0
                                    && is_semicont(lp, jjx) == 0
                                {
                                    k += 16 as ::core::ffi::c_int;
                                } else if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, jx) != 0
                                    && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, jjx) == 0
                                {
                                    k += 32 as ::core::ffi::c_int;
                                }
                                if k == 0 as ::core::ffi::c_int {
                                    if fabs(Coeff2)
                                        < (*psdata).epspivot * *(*mat).colmax.offset(jjx as isize)
                                        && fabs(Coeff1)
                                            > (*psdata).epspivot
                                                * *(*mat).colmax.offset(jx as isize)
                                    {
                                        k += 64 as ::core::ffi::c_int;
                                    } else if presolve_collength(psdata, jx)
                                        > presolve_collength(psdata, jjx)
                                    {
                                        k += 128 as ::core::ffi::c_int;
                                    }
                                }
                                if k == 0 as ::core::ffi::c_int {
                                    Value2 = Coeff1 / Coeff2;
                                    if fabs(modf(Coeff2, &raw mut Value2)) >= (*lp).epsvalue
                                        && fabs(modf(Coeff1, &raw mut Value2)) < (*lp).epsvalue
                                    {
                                        k += 512 as ::core::ffi::c_int;
                                    } else if fabs(
                                        fabs(Coeff2)
                                            - 1 as ::core::ffi::c_int as ::core::ffi::c_double,
                                    ) >= (*lp).epsvalue
                                        && fabs(
                                            fabs(Coeff1)
                                                - 1 as ::core::ffi::c_int as ::core::ffi::c_double,
                                        ) < (*lp).epsvalue
                                    {
                                        k += 1024 as ::core::ffi::c_int;
                                    }
                                }
                            } else {
                                k = 0 as ::core::ffi::c_int;
                            }
                            if k != 0 as ::core::ffi::c_int {
                                swapINT(&raw mut jx, &raw mut jjx);
                                swapREAL(&raw mut Coeff1, &raw mut Coeff2);
                            }
                            Value1 = *(*lp).orig_rhs.offset(i as isize) / Coeff2;
                            Value2 = Coeff1 / Coeff2;
                            upbound = *(*lp).orig_upbo.offset(((*lp).rows + jx) as isize);
                            lobound = *(*lp).orig_lowbo.offset(((*lp).rows + jx) as isize);
                            if (*lp).spx_trace != 0 {
                                report(
                                    lp,
                                    5 as ::core::ffi::c_int,
                                    b"Row %3d : Elim %g %s - %d\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                report(
                                    lp,
                                    5 as ::core::ffi::c_int,
                                    b"          Keep %g %s - %d\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                            freshupdate = (colindex.is_null()
                                || *colindex.offset(jjx as isize) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uchar;
                            if freshupdate != 0 {
                                mat_expandcolumn(
                                    mat,
                                    jjx,
                                    colvalue,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                    TRUE as ::core::ffi::c_uchar,
                                );
                            } else {
                                mat_expandcolumn(
                                    rev,
                                    *colindex.offset(jjx as isize),
                                    colvalue,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                    FALSE as ::core::ffi::c_uchar,
                                );
                            }
                            if colindex.is_null()
                                || *colindex.offset(jx as isize) == 0 as ::core::ffi::c_int
                            {
                                mat_expandcolumn(
                                    mat,
                                    jx,
                                    delvalue,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                    TRUE as ::core::ffi::c_uchar,
                                );
                            } else {
                                mat_expandcolumn(
                                    rev,
                                    *colindex.offset(jx as isize),
                                    delvalue,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                    FALSE as ::core::ffi::c_uchar,
                                );
                            }
                            addUndoPresolve(
                                lp,
                                TRUE as ::core::ffi::c_uchar,
                                jx,
                                Value1,
                                Value2,
                                jjx,
                            );
                            bound = lobound;
                            k = nrows + jjx;
                            if bound > -(*lp).infinite {
                                bound =
                                    (*(*lp).orig_rhs.offset(i as isize) - Coeff2 * bound) / Coeff1;
                                if Value2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    test = *(*lp).orig_upbo.offset(k as isize);
                                    if bound < test - (*psdata).epsvalue {
                                        if is_int(lp, jjx) != 0 {
                                            *(*lp).orig_upbo.offset(k as isize) =
                                                floor(bound + (*lp).epsint);
                                        } else {
                                            *(*lp).orig_upbo.offset(k as isize) = presolve_roundrhs(
                                                lp,
                                                bound,
                                                FALSE as ::core::ffi::c_uchar,
                                            );
                                        }
                                    }
                                } else {
                                    test = *(*lp).orig_lowbo.offset(k as isize);
                                    if bound > test + (*psdata).epsvalue {
                                        if is_int(lp, jjx) != 0 {
                                            *(*lp).orig_lowbo.offset(k as isize) =
                                                ceil(bound - (*lp).epsint);
                                        } else {
                                            *(*lp).orig_lowbo.offset(k as isize) =
                                                presolve_roundrhs(
                                                    lp,
                                                    bound,
                                                    TRUE as ::core::ffi::c_uchar,
                                                );
                                        }
                                    }
                                }
                            }
                            bound = upbound;
                            if bound < (*lp).infinite {
                                bound =
                                    (*(*lp).orig_rhs.offset(i as isize) - Coeff2 * bound) / Coeff1;
                                if Value2 < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    test = *(*lp).orig_upbo.offset(k as isize);
                                    if bound < test - (*psdata).epsvalue {
                                        if is_int(lp, jjx) != 0 {
                                            *(*lp).orig_upbo.offset(k as isize) =
                                                floor(bound + (*lp).epsint);
                                        } else {
                                            *(*lp).orig_upbo.offset(k as isize) = presolve_roundrhs(
                                                lp,
                                                bound,
                                                FALSE as ::core::ffi::c_uchar,
                                            );
                                        }
                                    }
                                } else {
                                    test = *(*lp).orig_lowbo.offset(k as isize);
                                    if bound > test + (*psdata).epsvalue {
                                        if is_int(lp, jjx) != 0 {
                                            *(*lp).orig_lowbo.offset(k as isize) =
                                                ceil(bound - (*lp).epsint);
                                        } else {
                                            *(*lp).orig_lowbo.offset(k as isize) =
                                                presolve_roundrhs(
                                                    lp,
                                                    bound,
                                                    TRUE as ::core::ffi::c_uchar,
                                                );
                                        }
                                    }
                                }
                            }
                            test =
                                2 as ::core::ffi::c_int as ::core::ffi::c_double * (*lp).epsvalue;
                            if fabs(
                                (*(*lp).orig_upbo.offset(k as isize)
                                    - *(*lp).orig_lowbo.offset(k as isize))
                                    / (1.0f64 + fabs(*(*lp).orig_lowbo.offset(k as isize))),
                            ) < test
                            {
                                if fabs(*(*lp).orig_lowbo.offset(k as isize)) < test {
                                    *(*lp).orig_lowbo.offset(k as isize) =
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                }
                                *(*lp).orig_upbo.offset(k as isize) =
                                    *(*lp).orig_lowbo.offset(k as isize);
                            } else {
                                if fabs(*(*lp).orig_upbo.offset(k as isize)) < test {
                                    *(*lp).orig_upbo.offset(k as isize) =
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                }
                                if fabs(*(*lp).orig_lowbo.offset(k as isize)) < test {
                                    *(*lp).orig_lowbo.offset(k as isize) =
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                }
                            }
                            if fabs((upbound - lobound) / (1.0f64 + fabs(lobound))) < test {
                                if fabs(lobound) < test {
                                    lobound = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                }
                                *(*lp).orig_upbo.offset((nrows + jx) as isize) = lobound;
                                upbound = lobound;
                            }
                            colitem = colvalue;
                            plucount = (*(*psdata).rows).plucount;
                            negcount = (*(*psdata).rows).negcount;
                            colplu = 0 as ::core::ffi::c_int;
                            colneg = 0 as ::core::ffi::c_int;
                            item = presolve_collength(psdata, jjx) - 1 as ::core::ffi::c_int;
                            if isnz_origobj(lp, jjx) != 0 {
                                item += 1;
                            }
                            k = 0 as ::core::ffi::c_int;
                            while k <= nrows {
                                bound = *delvalue.offset(k as isize);
                                if !(k == i
                                    || bound == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    || k > 0 as ::core::ffi::c_int
                                        && isActiveLink((*(*psdata).rows).varmap, k) == 0)
                                {
                                    product = bound * Value1;
                                    presolve_adjustrhs(
                                        psdata,
                                        k,
                                        if is_chsign(lp, k) as ::core::ffi::c_int != 0
                                            && product
                                                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            -product
                                        } else {
                                            product
                                        },
                                        test,
                                    );
                                    if *colitem != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        if *colitem
                                            > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            colplu -= 1;
                                            let ref mut fresh29 = *plucount.offset(k as isize);
                                            *fresh29 -= 1;
                                        } else {
                                            colneg -= 1;
                                            let ref mut fresh30 = *negcount.offset(k as isize);
                                            *fresh30 -= 1;
                                        }
                                        if lobound
                                            < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            && upbound
                                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            let ref mut fresh31 =
                                                *(*(*psdata).cols).pluneg.offset(jjx as isize);
                                            *fresh31 -= 1;
                                            let ref mut fresh32 =
                                                *(*(*psdata).rows).pluneg.offset(k as isize);
                                            *fresh32 -= 1;
                                        }
                                        item -= 1;
                                    }
                                    *colitem -= bound * Value2;
                                    iCoeffChanged += 1;
                                    if fabs(*colitem) >= (*mat).epsvalue {
                                        if *colitem
                                            > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            colplu += 1;
                                            let ref mut fresh33 = *plucount.offset(k as isize);
                                            *fresh33 += 1;
                                        } else {
                                            colneg += 1;
                                            let ref mut fresh34 = *negcount.offset(k as isize);
                                            *fresh34 += 1;
                                        }
                                        if lobound
                                            < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            && upbound
                                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            let ref mut fresh35 =
                                                *(*(*psdata).cols).pluneg.offset(jjx as isize);
                                            *fresh35 += 1;
                                            let ref mut fresh36 =
                                                *(*(*psdata).rows).pluneg.offset(k as isize);
                                            *fresh36 += 1;
                                        }
                                        item += 1;
                                    } else {
                                        *colitem = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                    }
                                    if bound > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        let ref mut fresh37 = *plucount.offset(k as isize);
                                        *fresh37 -= 1;
                                    } else {
                                        let ref mut fresh38 = *negcount.offset(k as isize);
                                        *fresh38 -= 1;
                                    }
                                }
                                k += 1;
                                colitem = colitem.offset(1);
                            }
                            *(*(*psdata).cols).plucount.offset(jjx as isize) += colplu;
                            *(*(*psdata).cols).negcount.offset(jjx as isize) += colneg;
                            if rev.is_null() {
                                DV = createUndoLadder(lp, nrows, (*lp).columns / RESIZEFACTOR);
                                rev = (*DV).tracker;
                                (*rev).epsvalue = (*mat).epsvalue;
                                allocINT(
                                    lp,
                                    &raw mut (*rev).col_tag,
                                    (*mat).columns_alloc + 1 as ::core::ffi::c_int,
                                    FALSE as ::core::ffi::c_uchar,
                                );
                                allocINT(
                                    lp,
                                    &raw mut colindex,
                                    (*lp).columns + 1 as ::core::ffi::c_int,
                                    TRUE as ::core::ffi::c_uchar,
                                );
                                *(*rev).col_tag.offset(0 as ::core::ffi::c_int as isize) =
                                    0 as ::core::ffi::c_int;
                            }
                            let ref mut fresh39 =
                                *(*rev).col_tag.offset(0 as ::core::ffi::c_int as isize);
                            *fresh39 = incrementUndoLadder(DV);
                            n = *fresh39;
                            mat_setcol(
                                rev,
                                n,
                                0 as ::core::ffi::c_int,
                                colvalue,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                FALSE as ::core::ffi::c_uchar,
                                FALSE as ::core::ffi::c_uchar,
                            );
                            *(*rev).col_tag.offset(n as isize) = jjx;
                            if freshupdate == 0 {
                                *(*rev)
                                    .col_tag
                                    .offset(*colindex.offset(jjx as isize) as isize) *=
                                    -(1 as ::core::ffi::c_int);
                            }
                            *colindex.offset(jjx as isize) = n;
                            jx = presolve_colremove(psdata, jx, FALSE as ::core::ffi::c_uchar);
                            iVarsFixed += 1;
                            if item == 0 as ::core::ffi::c_int {
                                if presolve_colfix(
                                    psdata,
                                    jjx,
                                    0.0f64,
                                    TRUE as ::core::ffi::c_uchar,
                                    nc,
                                ) != 0
                                {
                                    jjx = presolve_colremove(
                                        psdata,
                                        jjx,
                                        FALSE as ::core::ffi::c_uchar,
                                    );
                                }
                            }
                            presolve_rowremove(psdata, i, FALSE as ::core::ffi::c_uchar);
                            iRowsRemoved += 1;
                        }
                    }
                }
                i = nextActiveLink(EQ2, i);
            }
            if n > 0 as ::core::ffi::c_int {
                mat_mapreplace(mat, (*(*psdata).rows).varmap, (*(*psdata).cols).varmap, rev);
                presolve_validate(psdata, TRUE as ::core::ffi::c_uchar);
                (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
            }
        }
    }
    if !DV.is_null() {
        freeUndoLadder(&raw mut DV);
    }
    freeLink(&raw mut EQ2);
    if !(colvalue as *mut ::core::ffi::c_void).is_null() {
        free(colvalue as *mut ::core::ffi::c_void);
        colvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(delvalue as *mut ::core::ffi::c_void).is_null() {
        free(delvalue as *mut ::core::ffi::c_void);
        delvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(colindex as *mut ::core::ffi::c_void).is_null() {
        free(colindex as *mut ::core::ffi::c_void);
        colindex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    *nn += iCoeffChanged;
    *nr += iRowsRemoved;
    *nc += iVarsFixed;
    *nSum += iCoeffChanged + iRowsRemoved + iVarsFixed;
    return status;
}
#[export_name="honest_lpsolve_presolve_impliedfree"]
pub unsafe extern "C" fn presolve_impliedfree(
    mut lp: *mut lprec,
    mut psdata: *mut presolverec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut Tlower: ::core::ffi::c_double = 0.;
    let mut Tupper: ::core::ffi::c_double = 0.;
    let mut status: ::core::ffi::c_uchar = 0;
    let mut rowbinds: ::core::ffi::c_uchar = 0;
    let mut isfree: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut mat: *mut MATrec = (*lp).matA;
    if (fabs(get_lowbo(lp, colnr)) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
        as ::core::ffi::c_int
        != 0
        && (fabs(get_upbo(lp, colnr)) >= (*lp).infinite) as ::core::ffi::c_int
            as ::core::ffi::c_uchar as ::core::ffi::c_int
            != 0
    {
        return 1 as ::core::ffi::c_uchar;
    }
    ie = *(*mat).col_end.offset(colnr as isize);
    ix = *(*mat)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    while isfree as ::core::ffi::c_int != TRUE | AUTOMATIC && ix < ie {
        i = *(*mat).col_mat_rownr.offset(ix as isize);
        if !(isActiveLink((*(*psdata).rows).varmap, i) == 0) {
            Tlower = get_rh_lower(lp, i);
            Tupper = get_rh_upper(lp, i);
            status = presolve_multibounds(
                psdata,
                i,
                colnr,
                &raw mut Tlower,
                &raw mut Tupper,
                ::core::ptr::null_mut::<::core::ffi::c_double>(),
                &raw mut rowbinds,
            );
            isfree = (isfree as ::core::ffi::c_int
                | status as ::core::ffi::c_int
                | rowbinds as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        ix += 1;
    }
    return (isfree as ::core::ffi::c_int == TRUE | AUTOMATIC) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_impliedcolfix"]
pub unsafe extern "C" fn presolve_impliedcolfix(
    mut psdata: *mut presolverec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
    mut isfree: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut signflip: ::core::ffi::c_uchar = 0;
    let mut undoadded: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut jx: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = *(*mat).row_end.offset(rownr as isize);
    let mut varLo: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut varHi: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut varRange: ::core::ffi::c_double = 0.;
    let mut conRange: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut matValue: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut dual: ::core::ffi::c_double = 0.;
    let mut RHS: ::core::ffi::c_double = *(*lp).orig_rhs.offset(rownr as isize);
    let mut pivot: ::core::ffi::c_double = 0.;
    let mut matAij: ::core::ffi::c_double = mat_getitem(mat, rownr, colnr);
    let mut vecOF: *mut ::core::ffi::c_double = (*lp).orig_obj;
    if is_semicont(lp, colnr) as ::core::ffi::c_int != 0
        || is_SOS_var(lp, colnr) as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    if is_int(lp, colnr) != 0 {
        if isActiveLink((*psdata).INTmap, rownr) == 0 || is_presolve(lp, PRESOLVE_KNAPSACK) == 0 {
            return 0 as ::core::ffi::c_uchar;
        }
        varRange = (*lp).infinite;
        i = 0 as ::core::ffi::c_int;
        pivot = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        ib = presolve_nextcol(psdata, rownr, &raw mut i);
        while i != 0 as ::core::ffi::c_int {
            jx = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(ib as isize) as isize);
            dual = fabs(
                *(*mat)
                    .col_mat_value
                    .offset(*(*mat).row_mat.offset(ib as isize) as isize),
            );
            if jx == colnr {
                if fabs(dual - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    < (*psdata).epsvalue
                {
                    break;
                }
                pivot = dual;
            } else if pivot > dual + (*psdata).epsvalue
                || pivot > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && fabs(fmod(dual, pivot)) > (*psdata).epsvalue
            {
                return 0 as ::core::ffi::c_uchar;
            }
            ib = presolve_nextcol(psdata, rownr, &raw mut i);
        }
    }
    pivot = matAij;
    if fabs(pivot) < (*psdata).epspivot * *(*mat).colmax.offset(colnr as isize) {
        return 0 as ::core::ffi::c_uchar;
    }
    if SOS_count(lp) > 0 as ::core::ffi::c_int {
        ib = *(*mat)
            .row_end
            .offset((rownr - 1 as ::core::ffi::c_int) as isize);
        while ib < ie {
            if SOS_is_member(
                (*lp).SOS,
                0 as ::core::ffi::c_int,
                *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(ib as isize) as isize),
            ) != 0
            {
                return 0 as ::core::ffi::c_uchar;
            }
            ib += 1;
        }
    }
    dual = *vecOF.offset(colnr as isize) / pivot;
    if isfree as ::core::ffi::c_int != 0 && is_constr_type(lp, rownr, EQ) as ::core::ffi::c_int != 0
    {
        matValue = RHS / pivot;
        if matValue != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            undoadded = addUndoPresolve(
                lp,
                TRUE as ::core::ffi::c_uchar,
                colnr,
                matValue,
                0.0f64,
                0 as ::core::ffi::c_int,
            );
        }
    } else {
        if isfree != 0 {
            if RHS > presolve_sumplumin(lp, rownr, (*psdata).rows, 1 as ::core::ffi::c_uchar) {
                RHS = presolve_sumplumin(lp, rownr, (*psdata).rows, 1 as ::core::ffi::c_uchar);
            }
            matValue = presolve_sumplumin(lp, rownr, (*psdata).rows, FALSE as ::core::ffi::c_uchar);
            conRange = get_rh_lower(lp, rownr);
            conRange = RHS
                - (if matValue > conRange {
                    matValue
                } else {
                    conRange
                });
            signflip = (dual > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (fabs(conRange) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
                    == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        } else {
            varLo = get_lowbo(lp, colnr);
            varLo *= if (fabs(varLo) >= (*lp).infinite) as ::core::ffi::c_int
                as ::core::ffi::c_uchar as ::core::ffi::c_int
                != 0
            {
                (if pivot < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_double
            } else {
                pivot
            };
            varHi = get_upbo(lp, colnr);
            varHi *= if (fabs(varHi) >= (*lp).infinite) as ::core::ffi::c_int
                as ::core::ffi::c_uchar as ::core::ffi::c_int
                != 0
            {
                (if pivot < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_double
            } else {
                pivot
            };
            if pivot < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                swapREAL(&raw mut varHi, &raw mut varLo);
            }
            signflip =
                (fabs(varLo) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        }
        if signflip != 0 {
            mat_multrow(
                mat,
                rownr,
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            RHS -= conRange;
            RHS = -RHS;
            *(*lp).orig_rhs.offset(rownr as isize) = RHS;
            pivot = -pivot;
            dual = -dual;
            if isfree == 0 {
                varLo = -varLo;
                varHi = -varHi;
                swapREAL(&raw mut varHi, &raw mut varLo);
            }
        }
        matValue = RHS / pivot;
        if isfree != 0 {
            if matValue != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                undoadded = addUndoPresolve(
                    lp,
                    TRUE as ::core::ffi::c_uchar,
                    colnr,
                    matValue,
                    0.0f64,
                    0 as ::core::ffi::c_int,
                );
            }
            if dual != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                addUndoPresolve(
                    lp,
                    FALSE as ::core::ffi::c_uchar,
                    rownr,
                    dual,
                    0.0f64,
                    0 as ::core::ffi::c_int,
                );
            }
        } else {
            if (fabs(varHi) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar != 0 {
                varRange = (*lp).infinite;
            } else {
                varRange = restoreINT(fabs(varHi - varLo) + (*lp).epsvalue, (*psdata).epsvalue);
            }
            presolve_adjustrhs(psdata, rownr, varLo, (*psdata).epsvalue);
            if is_constr_type(lp, rownr, EQ) != 0 {
                if varRange > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    set_constr_type(lp, rownr, LE);
                    if (fabs(varRange) >= (*lp).infinite) as ::core::ffi::c_int
                        as ::core::ffi::c_uchar
                        == 0
                    {
                        *(*lp).orig_upbo.offset(rownr as isize) = varRange;
                    }
                    setLink((*psdata).LTmap, rownr);
                    removeLink((*psdata).EQmap, rownr);
                }
            } else if (fabs(*(*lp).orig_upbo.offset(rownr as isize)) >= (*lp).infinite)
                as ::core::ffi::c_int as ::core::ffi::c_uchar
                == 0
            {
                if (fabs(varRange) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
                    != 0
                {
                    *(*lp).orig_upbo.offset(rownr as isize) = (*lp).infinite;
                } else {
                    *(*lp).orig_upbo.offset(rownr as isize) += varHi - varLo;
                }
            }
            if matAij > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                let ref mut fresh40 = *(*(*psdata).rows).plucount.offset(rownr as isize);
                *fresh40 -= 1;
            } else {
                let ref mut fresh41 = *(*(*psdata).rows).negcount.offset(rownr as isize);
                *fresh41 -= 1;
            }
            if (if varLo < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) != (if varHi < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) {
                let ref mut fresh42 = *(*(*psdata).rows).pluneg.offset(rownr as isize);
                *fresh42 -= 1;
            }
            if RHS != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                undoadded = addUndoPresolve(
                    lp,
                    TRUE as ::core::ffi::c_uchar,
                    colnr,
                    RHS / pivot,
                    0.0f64,
                    0 as ::core::ffi::c_int,
                );
            }
        }
    }
    if dual != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        presolve_adjustrhs(
            psdata,
            0 as ::core::ffi::c_int,
            dual * RHS,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        *vecOF.offset(colnr as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    i = 0 as ::core::ffi::c_int;
    ib = presolve_nextcol(psdata, rownr, &raw mut i);
    while ib >= 0 as ::core::ffi::c_int {
        jx = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(ib as isize) as isize);
        if !(jx == colnr) {
            matValue = *(*mat)
                .col_mat_value
                .offset(*(*mat).row_mat.offset(ib as isize) as isize);
            if dual != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *vecOF.offset(jx as isize) -= dual * matValue;
            }
            if undoadded == 0 {
                undoadded = addUndoPresolve(
                    lp,
                    TRUE as ::core::ffi::c_uchar,
                    colnr,
                    0.0f64,
                    matValue / pivot,
                    jx,
                );
            } else {
                appendUndoPresolve(lp, TRUE as ::core::ffi::c_uchar, matValue / pivot, jx);
            }
        }
        ib = presolve_nextcol(psdata, rownr, &raw mut i);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_initpsrec"]
pub unsafe extern "C" fn presolve_initpsrec(
    mut lp: *mut lprec,
    mut size: ::core::ffi::c_int,
) -> *mut psrec {
    let mut ps: *mut psrec =
        calloc(1 as size_t, ::core::mem::size_of::<psrec>() as size_t) as *mut psrec;
    createLink(
        size,
        &raw mut (*ps).varmap,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    fillLink((*ps).varmap);
    size += 1;
    allocINT(
        lp,
        &raw mut (*ps).empty,
        size,
        FALSE as ::core::ffi::c_uchar,
    );
    *(*ps).empty.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    allocREAL(
        lp,
        &raw mut (*ps).pluupper,
        size,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut (*ps).negupper,
        size,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut (*ps).plulower,
        size,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut (*ps).neglower,
        size,
        FALSE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut (*ps).infcount,
        size,
        FALSE as ::core::ffi::c_uchar,
    );
    (*ps).next = calloc(
        size as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t,
    ) as *mut *mut ::core::ffi::c_int;
    allocINT(
        lp,
        &raw mut (*ps).plucount,
        size,
        TRUE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut (*ps).negcount,
        size,
        TRUE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut (*ps).pluneg,
        size,
        TRUE as ::core::ffi::c_uchar,
    );
    (*ps).allocsize = size;
    return ps;
}
#[export_name="honest_lpsolve_presolve_freepsrec"]
pub unsafe extern "C" fn presolve_freepsrec(mut ps: *mut *mut psrec) {
    if !((**ps).plucount as *mut ::core::ffi::c_void).is_null() {
        free((**ps).plucount as *mut ::core::ffi::c_void);
        (**ps).plucount = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**ps).negcount as *mut ::core::ffi::c_void).is_null() {
        free((**ps).negcount as *mut ::core::ffi::c_void);
        (**ps).negcount = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**ps).pluneg as *mut ::core::ffi::c_void).is_null() {
        free((**ps).pluneg as *mut ::core::ffi::c_void);
        (**ps).pluneg = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((**ps).infcount as *mut ::core::ffi::c_void).is_null() {
        free((**ps).infcount as *mut ::core::ffi::c_void);
        (**ps).infcount = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(**ps).next.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = (**ps).allocsize;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            if !(*(**ps).next.offset(i as isize) as *mut ::core::ffi::c_void).is_null() {
                free(*(**ps).next.offset(i as isize) as *mut ::core::ffi::c_void);
                let ref mut fresh13 = *(**ps).next.offset(i as isize);
                *fresh13 = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
            i += 1;
        }
        if !((**ps).next as *mut ::core::ffi::c_void).is_null() {
            free((**ps).next as *mut ::core::ffi::c_void);
            (**ps).next = ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
        }
    }
    if !((**ps).plulower as *mut ::core::ffi::c_void).is_null() {
        free((**ps).plulower as *mut ::core::ffi::c_void);
        (**ps).plulower = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**ps).neglower as *mut ::core::ffi::c_void).is_null() {
        free((**ps).neglower as *mut ::core::ffi::c_void);
        (**ps).neglower = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**ps).pluupper as *mut ::core::ffi::c_void).is_null() {
        free((**ps).pluupper as *mut ::core::ffi::c_void);
        (**ps).pluupper = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**ps).negupper as *mut ::core::ffi::c_void).is_null() {
        free((**ps).negupper as *mut ::core::ffi::c_void);
        (**ps).negupper = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**ps).empty as *mut ::core::ffi::c_void).is_null() {
        free((**ps).empty as *mut ::core::ffi::c_void);
        (**ps).empty = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    freeLink(&raw mut (**ps).varmap);
    if !(*ps as *mut ::core::ffi::c_void).is_null() {
        free(*ps as *mut ::core::ffi::c_void);
        *ps = ::core::ptr::null_mut::<psrec>();
    }
}
#[export_name="honest_lpsolve_presolve_init"]
pub unsafe extern "C" fn presolve_init(mut lp: *mut lprec) -> *mut presolverec {
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ixx: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut ncols: ::core::ffi::c_int = (*lp).columns;
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut hold: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut psdata: *mut presolverec = ::core::ptr::null_mut::<presolverec>();
    ix = get_nonzeros(lp);
    ixx = (*(*lp).matA).mat_alloc;
    if ixx - ix > MAT_START_SIZE && (ixx - ix) * 20 as ::core::ffi::c_int > ixx {
        mat_memopt(
            (*lp).matA,
            nrows / 20 as ::core::ffi::c_int,
            ncols / 20 as ::core::ffi::c_int,
            ix / 20 as ::core::ffi::c_int,
        );
    }
    psdata =
        calloc(1 as size_t, ::core::mem::size_of::<presolverec>() as size_t) as *mut presolverec;
    (*psdata).lp = lp;
    (*psdata).rows = presolve_initpsrec(lp, nrows);
    (*psdata).cols = presolve_initpsrec(lp, ncols);
    (*psdata).epsvalue = 0.1f64 * (*lp).epsprimal;
    (*psdata).epspivot = PRESOLVE_EPSPIVOT;
    (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
    k = (*lp).sum + 1 as ::core::ffi::c_int;
    allocREAL(
        lp,
        &raw mut (*psdata).pv_lobo,
        k,
        FALSE as ::core::ffi::c_uchar,
    );
    memcpy(
        (*psdata).pv_lobo as *mut ::core::ffi::c_void,
        (*lp).orig_lowbo as *const ::core::ffi::c_void,
        (k as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    allocREAL(
        lp,
        &raw mut (*psdata).pv_upbo,
        k,
        FALSE as ::core::ffi::c_uchar,
    );
    memcpy(
        (*psdata).pv_upbo as *mut ::core::ffi::c_void,
        (*lp).orig_upbo as *const ::core::ffi::c_void,
        (k as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    allocREAL(
        lp,
        &raw mut (*psdata).dv_lobo,
        k,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut (*psdata).dv_upbo,
        k,
        FALSE as ::core::ffi::c_uchar,
    );
    i = 0 as ::core::ffi::c_int;
    while i <= nrows {
        *(*psdata).dv_lobo.offset(i as isize) =
            if is_constr_type(lp, i, EQ) as ::core::ffi::c_int != 0 {
                -(*lp).infinite
            } else {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            };
        *(*psdata).dv_upbo.offset(i as isize) = (*lp).infinite;
        i += 1;
    }
    k -= 1;
    while i <= k {
        *(*psdata).dv_lobo.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*psdata).dv_upbo.offset(i as isize) = (*lp).infinite;
        i += 1;
    }
    createLink(
        nrows,
        &raw mut (*psdata).EQmap,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    createLink(
        nrows,
        &raw mut (*psdata).LTmap,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    createLink(
        nrows,
        &raw mut (*psdata).INTmap,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    i = 1 as ::core::ffi::c_int;
    while i <= nrows {
        match get_constr_type(lp, i) {
            LE => {
                appendLink((*psdata).LTmap, i);
            }
            EQ => {
                appendLink((*psdata).EQmap, i);
            }
            _ => {}
        }
        k = mat_rowlength(mat, i);
        if (*lp).int_vars > 0 as ::core::ffi::c_int && k > 0 as ::core::ffi::c_int {
            appendLink((*psdata).INTmap, i);
        }
        i += 1;
    }
    if (*(*psdata).INTmap).count > 0 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= nrows {
            if !(isActiveLink((*psdata).INTmap, i) == 0) {
                ix = *(*mat)
                    .row_end
                    .offset((i - 1 as ::core::ffi::c_int) as isize);
                ixx = *(*mat).row_end.offset(i as isize);
                colnr = 0 as ::core::ffi::c_int;
                while ix < ixx {
                    if is_int(
                        lp,
                        *(*mat)
                            .col_mat_colnr
                            .offset(*(*mat).row_mat.offset(ix as isize) as isize),
                    ) == 0
                    {
                        removeLink((*psdata).INTmap, i);
                        break;
                    } else {
                        hold = fabs(
                            *(*mat)
                                .col_mat_value
                                .offset(*(*mat).row_mat.offset(ix as isize) as isize),
                        );
                        hold = fmod(hold, 1 as ::core::ffi::c_int as ::core::ffi::c_double);
                        k = 0 as ::core::ffi::c_int;
                        while k <= MAX_FRACSCALE
                            && hold + (*psdata).epsvalue
                                < 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            hold *= 10 as ::core::ffi::c_int as ::core::ffi::c_double;
                            k += 1;
                        }
                        if k > MAX_FRACSCALE {
                            removeLink((*psdata).INTmap, i);
                            break;
                        } else {
                            if colnr < k {
                                colnr = k;
                            }
                            ix += 1;
                        }
                    }
                }
                if !(isActiveLink((*psdata).INTmap, i) == 0) {
                    hold = pow(10.0f64, colnr as ::core::ffi::c_double);
                    if fabs(fmod(
                        *(*lp).orig_rhs.offset(i as isize) * hold,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                    )) > (*psdata).epsvalue
                    {
                        removeLink((*psdata).INTmap, i);
                    } else if k > 0 as ::core::ffi::c_int {
                        ix = *(*mat)
                            .row_end
                            .offset((i - 1 as ::core::ffi::c_int) as isize);
                        while ix < ixx {
                            *(*mat)
                                .col_mat_value
                                .offset(*(*mat).row_mat.offset(ix as isize) as isize) *= hold;
                            ix += 1;
                        }
                        *(*lp).orig_rhs.offset(i as isize) *= hold;
                        if (fabs(*(*lp).orig_upbo.offset(i as isize)) >= (*lp).infinite)
                            as ::core::ffi::c_int as ::core::ffi::c_uchar
                            == 0
                        {
                            *(*lp).orig_upbo.offset(i as isize) *= hold;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    presolve_validate(psdata, TRUE as ::core::ffi::c_uchar);
    return psdata;
}
#[export_name="honest_lpsolve_presolve_free"]
pub unsafe extern "C" fn presolve_free(mut psdata: *mut *mut presolverec) {
    presolve_freepsrec(&raw mut (**psdata).rows);
    presolve_freepsrec(&raw mut (**psdata).cols);
    if !((**psdata).dv_lobo as *mut ::core::ffi::c_void).is_null() {
        free((**psdata).dv_lobo as *mut ::core::ffi::c_void);
        (**psdata).dv_lobo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**psdata).dv_upbo as *mut ::core::ffi::c_void).is_null() {
        free((**psdata).dv_upbo as *mut ::core::ffi::c_void);
        (**psdata).dv_upbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**psdata).pv_lobo as *mut ::core::ffi::c_void).is_null() {
        free((**psdata).pv_lobo as *mut ::core::ffi::c_void);
        (**psdata).pv_lobo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((**psdata).pv_upbo as *mut ::core::ffi::c_void).is_null() {
        free((**psdata).pv_upbo as *mut ::core::ffi::c_void);
        (**psdata).pv_upbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    freeLink(&raw mut (**psdata).EQmap);
    freeLink(&raw mut (**psdata).LTmap);
    freeLink(&raw mut (**psdata).INTmap);
    if !(*psdata as *mut ::core::ffi::c_void).is_null() {
        free(*psdata as *mut ::core::ffi::c_void);
        *psdata = ::core::ptr::null_mut::<presolverec>();
    }
}
#[export_name="honest_lpsolve_presolve_makefree"]
pub unsafe extern "C" fn presolve_makefree(mut psdata: *mut presolverec) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Xlower: ::core::ffi::c_double = 0.;
    let mut Xupper: ::core::ffi::c_double = 0.;
    let mut losum: ::core::ffi::c_double = 0.;
    let mut upsum: ::core::ffi::c_double = 0.;
    let mut lorhs: ::core::ffi::c_double = 0.;
    let mut uprhs: ::core::ffi::c_double = 0.;
    let mut freeinf: ::core::ffi::c_double =
        (*lp).infinite / 10 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut colLL: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
    i = firstActiveLink((*(*psdata).rows).varmap);
    while i != 0 as ::core::ffi::c_int {
        if !(is_constr_type(lp, i, EQ) != 0) {
            presolve_range(lp, i, (*psdata).rows, &raw mut losum, &raw mut upsum);
            lorhs = get_rh_lower(lp, i);
            uprhs = get_rh_upper(lp, i);
            if presolve_rowlength(psdata, i) > 1 as ::core::ffi::c_int {
                if is_constr_type(lp, i, GE) as ::core::ffi::c_int != 0 && upsum <= uprhs
                    || is_constr_type(lp, i, LE) as ::core::ffi::c_int != 0 && losum >= lorhs
                {
                    set_rh_range(lp, i, (*lp).infinite);
                }
            }
        }
        i = nextActiveLink((*(*psdata).rows).varmap, i);
    }
    createLink(
        (*lp).columns,
        &raw mut colLL,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    j = firstActiveLink((*(*psdata).cols).varmap);
    while j != 0 as ::core::ffi::c_int {
        if presolve_impliedfree(lp, psdata, j) != 0 {
            appendLink(colLL, j);
        }
        j = nextActiveLink((*(*psdata).cols).varmap, j);
    }
    if (*colLL).count > 0 as ::core::ffi::c_int {
        let mut rowLL: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
        let mut canfree: ::core::ffi::c_uchar = 0;
        createLink(
            (*lp).rows,
            &raw mut rowLL,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        fillLink(rowLL);
        j = firstActiveLink(colLL);
        while j > 0 as ::core::ffi::c_int && (*rowLL).count > 0 as ::core::ffi::c_int {
            canfree = TRUE as ::core::ffi::c_uchar;
            ix = *(*mat)
                .col_end
                .offset((j - 1 as ::core::ffi::c_int) as isize);
            while canfree as ::core::ffi::c_int != 0 && ix < *(*mat).col_end.offset(j as isize) {
                canfree = isActiveLink(rowLL, *(*mat).col_mat_rownr.offset(ix as isize));
                ix += 1;
            }
            if canfree != 0 {
                nn += 1;
                Xlower = get_lowbo(lp, j);
                Xupper = get_upbo(lp, j);
                if Xlower >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    set_bounds(
                        lp,
                        j,
                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                        freeinf,
                    );
                } else if Xupper <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    set_bounds(
                        lp,
                        j,
                        -freeinf,
                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
                } else {
                    set_unbounded(lp, j);
                }
                ix = *(*mat)
                    .col_end
                    .offset((j - 1 as ::core::ffi::c_int) as isize);
                while ix < *(*mat).col_end.offset(j as isize) {
                    removeLink(rowLL, *(*mat).col_mat_rownr.offset(ix as isize));
                    ix += 1;
                }
            }
            j = nextActiveLink(colLL, j);
        }
        freeLink(&raw mut rowLL);
    }
    freeLink(&raw mut colLL);
    return nn;
}
#[export_name="honest_lpsolve_presolve_updatesums"]
pub unsafe extern "C" fn presolve_updatesums(mut psdata: *mut presolverec) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut j: ::core::ffi::c_int = 0;
    memset(
        (*(*psdata).rows).pluupper as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    memset(
        (*(*psdata).rows).negupper as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    memset(
        (*(*psdata).rows).plulower as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    memset(
        (*(*psdata).rows).neglower as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    memset(
        (*(*psdata).rows).infcount as *mut ::core::ffi::c_void,
        '\0' as i32,
        (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    j = firstActiveLink((*(*psdata).cols).varmap);
    while j != 0 as ::core::ffi::c_int {
        presolve_colfix(
            psdata,
            j,
            (*lp).infinite,
            FALSE as ::core::ffi::c_uchar,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        j = nextActiveLink((*(*psdata).cols).varmap, j);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_presolve_finalize"]
pub unsafe extern "C" fn presolve_finalize(mut psdata: *mut presolverec) -> ::core::ffi::c_uchar {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut compactvars: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut ke: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    (*(*lp).presolve_undo).OFcolsdeleted = FALSE as ::core::ffi::c_uchar;
    n = firstInactiveLink((*(*psdata).cols).varmap);
    while n != 0 as ::core::ffi::c_int && (*(*lp).presolve_undo).OFcolsdeleted == 0 {
        (*(*lp).presolve_undo).OFcolsdeleted = (*(*lp).orig_obj.offset(n as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int
            as ::core::ffi::c_uchar;
        n = nextInactiveLink((*(*psdata).cols).varmap, n);
    }
    ke = lastInactiveLink((*(*psdata).cols).varmap);
    n = countInactiveLink((*(*psdata).cols).varmap);
    if n > 0 as ::core::ffi::c_int && ke > 0 as ::core::ffi::c_int {
        del_columnex(lp, (*(*psdata).cols).varmap);
        mat_colcompact(
            (*lp).matA,
            (*(*lp).presolve_undo).orig_rows,
            (*(*lp).presolve_undo).orig_columns,
        );
        compactvars = TRUE as ::core::ffi::c_uchar;
    }
    ke = lastInactiveLink((*(*psdata).rows).varmap);
    n = countInactiveLink((*(*psdata).rows).varmap);
    if n > 0 as ::core::ffi::c_int && ke > 0 as ::core::ffi::c_int {
        del_constraintex(lp, (*(*psdata).rows).varmap);
        mat_rowcompact((*lp).matA, TRUE as ::core::ffi::c_uchar);
        compactvars = TRUE as ::core::ffi::c_uchar;
    } else if (*psdata).nzdeleted > 0 as ::core::ffi::c_int {
        mat_zerocompact((*lp).matA);
    }
    if compactvars != 0 {
        varmap_compact(
            lp,
            (*(*lp).presolve_undo).orig_rows,
            (*(*lp).presolve_undo).orig_columns,
        );
    }
    if !(*(*lp).presolve_undo).primalundo.is_null() {
        mat_memopt(
            (*(*(*lp).presolve_undo).primalundo).tracker,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    if !(*(*lp).presolve_undo).dualundo.is_null() {
        mat_memopt(
            (*(*(*lp).presolve_undo).dualundo).tracker,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    ke = (*lp).columns;
    n = 1 as ::core::ffi::c_int;
    while n <= ke {
        if fabs(*(*lp).orig_obj.offset(n as isize)) < (*lp).epsvalue {
            *(*lp).orig_obj.offset(n as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        n += 1;
    }
    ke = (*lp).rows;
    n = 1 as ::core::ffi::c_int;
    while n <= ke {
        if fabs(*(*lp).orig_rhs.offset(n as isize)) < (*lp).epsvalue {
            *(*lp).orig_rhs.offset(n as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        n += 1;
    }
    if SOS_count(lp) > 0 as ::core::ffi::c_int {
        SOS_member_updatemap((*lp).SOS);
    }
    return mat_validate((*lp).matA);
}
#[export_name="honest_lpsolve_compRedundant"]
pub unsafe extern "C" fn compRedundant(
    mut current: *const QSORTrec,
    mut candidate: *const QSORTrec,
) -> ::core::ffi::c_int {
    let mut start1: ::core::ffi::c_int = (*current).int4.intpar1;
    let mut start2: ::core::ffi::c_int = (*candidate).int4.intpar1;
    let mut result: ::core::ffi::c_int = if start1 < start2 {
        -(1 as ::core::ffi::c_int)
    } else if start1 > start2 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if result == 0 as ::core::ffi::c_int {
        start1 = (*current).int4.intpar2;
        start2 = (*candidate).int4.intpar2;
        result = -if start1 < start2 {
            -(1 as ::core::ffi::c_int)
        } else if start1 > start2 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    return result;
}
#[export_name="honest_lpsolve_compSparsity"]
pub unsafe extern "C" fn compSparsity(
    mut current: *const QSORTrec,
    mut candidate: *const QSORTrec,
) -> ::core::ffi::c_int {
    let mut start1: ::core::ffi::c_int = (*current).int4.intpar1;
    let mut start2: ::core::ffi::c_int = (*candidate).int4.intpar1;
    let mut result: ::core::ffi::c_int = if start1 < start2 {
        -(1 as ::core::ffi::c_int)
    } else if start1 > start2 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if result == 0 as ::core::ffi::c_int {
        start1 = (*current).int4.intpar2;
        start2 = (*candidate).int4.intpar2;
        result = -if start1 < start2 {
            -(1 as ::core::ffi::c_int)
        } else if start1 > start2 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if result == 0 as ::core::ffi::c_int {
        start1 = (*current).int4.intval;
        start2 = (*candidate).int4.intval;
        result = if start1 < start2 {
            -(1 as ::core::ffi::c_int)
        } else if start1 > start2 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    return result;
}
#[export_name="honest_lpsolve_compAggregate"]
pub unsafe extern "C" fn compAggregate(
    mut current: *const QSORTrec,
    mut candidate: *const QSORTrec,
) -> ::core::ffi::c_int {
    let mut index1: ::core::ffi::c_int = (*current).pvoidint2.intval;
    let mut index2: ::core::ffi::c_int = (*candidate).pvoidint2.intval;
    let mut lp: *mut lprec = (*current).pvoidint2.ptr as *mut lprec;
    let mut value1: ::core::ffi::c_double = *(*lp).orig_obj.offset(index1 as isize);
    let mut value2: ::core::ffi::c_double = *(*lp).orig_obj.offset(index2 as isize);
    let mut result: ::core::ffi::c_int = if value1 < value2 {
        -(1 as ::core::ffi::c_int)
    } else if value1 > value2 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if result == 0 as ::core::ffi::c_int {
        index1 += (*lp).rows;
        index2 += (*lp).rows;
        value1 = *(*lp).orig_lowbo.offset(index1 as isize);
        value2 = *(*lp).orig_lowbo.offset(index2 as isize);
        result = if value1 < value2 {
            -(1 as ::core::ffi::c_int)
        } else if value1 > value2 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if result == 0 as ::core::ffi::c_int {
        value1 = *(*lp).orig_upbo.offset(index1 as isize);
        value2 = *(*lp).orig_upbo.offset(index2 as isize);
        result = -if value1 < value2 {
            -(1 as ::core::ffi::c_int)
        } else if value1 > value2 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    return result;
}
#[export_name="honest_lpsolve_presolve_rowdominance"]
pub unsafe extern "C" fn presolve_rowdominance(
    mut psdata: *mut presolverec,
    mut nCoeffChanged: *mut ::core::ffi::c_int,
    mut nRowsRemoved: *mut ::core::ffi::c_int,
    mut nVarsFixed: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut coldel: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut item: ::core::ffi::c_int = 0;
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iRowRemoved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ratio: ::core::ffi::c_double = 0.;
    let mut rowvalues: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut QS: *mut QSORTrec = calloc(
        ((*lp).rows + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<QSORTrec>() as size_t,
    ) as *mut QSORTrec;
    if QS.is_null() {
        return status;
    }
    n = 0 as ::core::ffi::c_int;
    i = firstActiveLink((*psdata).EQmap);
    while i != 0 as ::core::ffi::c_int {
        je = 0 as ::core::ffi::c_int;
        jb = je;
        if SOS_count(lp) > 0 as ::core::ffi::c_int || (*lp).sc_vars > 0 as ::core::ffi::c_int {
            item = 0 as ::core::ffi::c_int;
            jb = presolve_nextcol(psdata, i, &raw mut item);
            while jb >= 0 as ::core::ffi::c_int {
                jx = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, jx) != 0
                    || is_semicont(lp, jx) as ::core::ffi::c_int != 0
                {
                    break;
                }
                jb = presolve_nextcol(psdata, i, &raw mut item);
            }
        }
        if jb < 0 as ::core::ffi::c_int {
            (*QS.offset(n as isize)).int4.intval = i;
            item = 0 as ::core::ffi::c_int;
            ii = presolve_nextcol(psdata, i, &raw mut item);
            (*QS.offset(n as isize)).int4.intpar1 = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(ii as isize) as isize);
            (*QS.offset(n as isize)).int4.intpar2 = presolve_rowlength(psdata, i);
            n += 1;
        }
        i = nextActiveLink((*psdata).EQmap, i);
    }
    if !(n <= 1 as ::core::ffi::c_int) {
        QS_execute(
            QS as *mut QSORTrec,
            n,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int,
                >,
                Option<findCompare_func>,
            >(Some(
                compRedundant
                    as unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int,
            )),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !(allocREAL(
            lp,
            &raw mut rowvalues,
            (*lp).columns + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0
            || allocINT(
                lp,
                &raw mut coldel,
                (*lp).columns + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) == 0)
        {
            ib = 0 as ::core::ffi::c_int;
            's_114: while ib < n {
                i = (*QS.offset(ib as isize)).int4.intval;
                if !(i < 0 as ::core::ffi::c_int) {
                    item = 0 as ::core::ffi::c_int;
                    jb = presolve_nextcol(psdata, i, &raw mut item);
                    while jb >= 0 as ::core::ffi::c_int {
                        jx = *(*mat)
                            .col_mat_colnr
                            .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                        *rowvalues.offset(jx as isize) = *(*mat)
                            .col_mat_value
                            .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                        jb = presolve_nextcol(psdata, i, &raw mut item);
                    }
                    ie = ib + 1 as ::core::ffi::c_int;
                    while ie < n {
                        ii = (*QS.offset(ie as isize)).int4.intval;
                        if !(ii < 0 as ::core::ffi::c_int) {
                            if *(*lp).orig_rhs.offset(i as isize)
                                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                && *(*lp).orig_rhs.offset(ii as isize)
                                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                ratio = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                current_block = 6417057564578538666;
                            } else if *(*lp).orig_rhs.offset(i as isize)
                                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                && *(*lp).orig_rhs.offset(ii as isize)
                                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                ratio = *(*lp).orig_rhs.offset(i as isize)
                                    / *(*lp).orig_rhs.offset(ii as isize);
                                current_block = 6417057564578538666;
                            } else {
                                current_block = 2569451025026770673;
                            }
                            match current_block {
                                2569451025026770673 => {}
                                _ => {
                                    item = 0 as ::core::ffi::c_int;
                                    jb = presolve_nextcol(psdata, ii, &raw mut item);
                                    while jb >= 0 as ::core::ffi::c_int {
                                        jx = *(*mat)
                                            .col_mat_colnr
                                            .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                                        if *rowvalues.offset(jx as isize)
                                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            break;
                                        }
                                        if ratio == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            ratio = *rowvalues.offset(jx as isize)
                                                / *(*mat)
                                                    .col_mat_value
                                                    .offset(*(*mat).row_mat.offset(jb as isize)
                                                        as isize);
                                        } else if fabs(
                                            *rowvalues.offset(jx as isize)
                                                - ratio
                                                    * *(*mat).col_mat_value.offset(
                                                        *(*mat).row_mat.offset(jb as isize)
                                                            as isize,
                                                    ),
                                        ) > (*psdata).epsvalue
                                        {
                                            break;
                                        }
                                        jb = presolve_nextcol(psdata, ii, &raw mut item);
                                    }
                                    if jb < 0 as ::core::ffi::c_int {
                                        let mut sign_1: ::core::ffi::c_int =
                                            0 as ::core::ffi::c_int;
                                        let mut sign_j: ::core::ffi::c_int =
                                            0 as ::core::ffi::c_int;
                                        *coldel.offset(0 as ::core::ffi::c_int as isize) =
                                            0 as ::core::ffi::c_int;
                                        item = 0 as ::core::ffi::c_int;
                                        jb = presolve_nextcol(psdata, i, &raw mut item);
                                        while jb >= 0 as ::core::ffi::c_int {
                                            jx =
                                                *(*mat)
                                                    .col_mat_colnr
                                                    .offset(*(*mat).row_mat.offset(jb as isize)
                                                        as isize);
                                            if mat_findelm(mat, ii, jx) <= 0 as ::core::ffi::c_int {
                                                if *(*lp)
                                                    .orig_lowbo
                                                    .offset(((*lp).rows + jx) as isize)
                                                    < 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                    && *(*lp)
                                                        .orig_upbo
                                                        .offset(((*lp).rows + jx) as isize)
                                                        > 0 as ::core::ffi::c_int
                                                            as ::core::ffi::c_double
                                                {
                                                    *coldel
                                                        .offset(0 as ::core::ffi::c_int as isize) =
                                                        -(1 as ::core::ffi::c_int);
                                                    break;
                                                } else if *(*lp)
                                                    .orig_lowbo
                                                    .offset(((*lp).rows + jx) as isize)
                                                    > 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                    || *(*lp)
                                                        .orig_upbo
                                                        .offset(((*lp).rows + jx) as isize)
                                                        < 0 as ::core::ffi::c_int
                                                            as ::core::ffi::c_double
                                                {
                                                    report(
                                                        lp,
                                                        5 as ::core::ffi::c_int,
                                                        b"presolve_rowdominate: Column %s is infeasible due to conflict in rows %s and %s\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    *coldel
                                                        .offset(0 as ::core::ffi::c_int as isize) =
                                                        -(1 as ::core::ffi::c_int);
                                                    break;
                                                } else {
                                                    sign_j = if *(*mat)
                                                        .col_mat_value
                                                        .offset(*(*mat).row_mat.offset(jb as isize)
                                                            as isize)
                                                        < 0 as ::core::ffi::c_int
                                                            as ::core::ffi::c_double
                                                    {
                                                        -(1 as ::core::ffi::c_int)
                                                    } else {
                                                        1 as ::core::ffi::c_int
                                                    };
                                                    sign_j = if is_negative(lp, jx)
                                                        as ::core::ffi::c_int
                                                        != 0
                                                        && sign_j != 0 as ::core::ffi::c_int
                                                    {
                                                        -sign_j
                                                    } else {
                                                        sign_j
                                                    };
                                                    if *coldel
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        == 0 as ::core::ffi::c_int
                                                    {
                                                        sign_1 = sign_j;
                                                        let ref mut fresh46 = *coldel.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        );
                                                        *fresh46 += 1;
                                                        *coldel.offset(*fresh46 as isize) = jx;
                                                    } else if sign_j == sign_1 {
                                                        let ref mut fresh47 = *coldel.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        );
                                                        *fresh47 += 1;
                                                        *coldel.offset(*fresh47 as isize) = jx;
                                                    } else {
                                                        *coldel.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        ) = -(1 as ::core::ffi::c_int);
                                                        break;
                                                    }
                                                }
                                            }
                                            jb = presolve_nextcol(psdata, i, &raw mut item);
                                        }
                                        if !(*coldel.offset(0 as ::core::ffi::c_int as isize)
                                            < 0 as ::core::ffi::c_int)
                                        {
                                            jb = 1 as ::core::ffi::c_int;
                                            while jb
                                                <= *coldel.offset(0 as ::core::ffi::c_int as isize)
                                            {
                                                jx = *coldel.offset(jb as isize);
                                                if presolve_colfix(
                                                    psdata,
                                                    jx,
                                                    0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double,
                                                    TRUE as ::core::ffi::c_uchar,
                                                    &raw mut iVarFixed,
                                                ) == 0
                                                {
                                                    status = presolve_setstatusex(
                                                        psdata,
                                                        2 as ::core::ffi::c_int,
                                                        3846 as ::core::ffi::c_int,
                                                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                                            as *const u8 as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    break 's_114;
                                                } else {
                                                    presolve_colremove(
                                                        psdata,
                                                        jx,
                                                        TRUE as ::core::ffi::c_uchar,
                                                    );
                                                    *rowvalues.offset(jx as isize) = 0
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_double;
                                                    jb += 1;
                                                }
                                            }
                                            presolve_rowremove(
                                                psdata,
                                                ii,
                                                TRUE as ::core::ffi::c_uchar,
                                            );
                                            iRowRemoved += 1;
                                            (*QS.offset(ie as isize)).int4.intval = -ii;
                                        }
                                    }
                                }
                            }
                        }
                        ie += 1;
                    }
                    ie = *(*mat)
                        .row_end
                        .offset((i - 1 as ::core::ffi::c_int) as isize);
                    ii = *(*mat).row_end.offset(i as isize);
                    while ie < ii {
                        *rowvalues.offset(
                            *(*mat)
                                .col_mat_colnr
                                .offset(*(*mat).row_mat.offset(ie as isize) as isize)
                                as isize,
                        ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        ie += 1;
                    }
                }
                ib += 1;
            }
        }
    }
    if !(QS as *mut ::core::ffi::c_void).is_null() {
        free(QS as *mut ::core::ffi::c_void);
        QS = ::core::ptr::null_mut::<QSORTrec>();
    }
    if !(rowvalues as *mut ::core::ffi::c_void).is_null() {
        free(rowvalues as *mut ::core::ffi::c_void);
        rowvalues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(coldel as *mut ::core::ffi::c_void).is_null() {
        free(coldel as *mut ::core::ffi::c_void);
        coldel = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    *nCoeffChanged += iCoeffChanged;
    *nRowsRemoved += iRowRemoved;
    *nVarsFixed += iVarFixed;
    *nSum += iCoeffChanged + iRowRemoved + iVarFixed;
    return status;
}
#[export_name="honest_lpsolve_presolve_coldominance01"]
pub unsafe extern "C" fn presolve_coldominance01(
    mut psdata: *mut presolverec,
    mut nConRemoved: *mut ::core::ffi::c_int,
    mut nVarsFixed: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut i: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut item2: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = (*lp).int_vars;
    let mut iVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nrows: ::core::ffi::c_int = (*lp).rows;
    let mut coldel: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut jb: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut rhsval: ::core::ffi::c_double = 0.0f64;
    let mut colvalues: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut colobj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut sets: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
    let mut QS: *mut QSORTrec = calloc(
        (n + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<QSORTrec>() as size_t,
    ) as *mut QSORTrec;
    if QS.is_null() {
        return status;
    }
    if !(n == 0 as ::core::ffi::c_int) {
        createLink(
            nrows,
            &raw mut sets,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        i = firstActiveLink((*(*psdata).rows).varmap);
        while i != 0 as ::core::ffi::c_int {
            if !(*(*lp).orig_rhs.offset(i as isize)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                || *(*(*psdata).rows).negcount.offset(i as isize) > 0 as ::core::ffi::c_int)
            {
                item = 0 as ::core::ffi::c_int;
                jb = presolve_nextcol(psdata, i, &raw mut item);
                while jb >= 0 as ::core::ffi::c_int {
                    jx = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                    if is_binary(lp, jx) == 0 {
                        break;
                    }
                    rhsval = *(*mat)
                        .col_mat_value
                        .offset(*(*mat).row_mat.offset(jb as isize) as isize)
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    if fabs(rhsval) > (*lp).epsvalue {
                        break;
                    }
                    jb = presolve_nextcol(psdata, i, &raw mut item);
                }
                if jb < 0 as ::core::ffi::c_int {
                    setLink(sets, i);
                }
            }
            i = nextActiveLink((*(*psdata).rows).varmap, i);
        }
        if !(countActiveLink(sets) == 0 as ::core::ffi::c_int) {
            n = 0 as ::core::ffi::c_int;
            i = firstActiveLink((*(*psdata).cols).varmap);
            while i != 0 as ::core::ffi::c_int {
                if is_binary(lp, i) as ::core::ffi::c_int != 0
                    && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, i) == 0
                {
                    item = 0 as ::core::ffi::c_int;
                    jb = presolve_nextrow(psdata, i, &raw mut item);
                    while jb >= 0 as ::core::ffi::c_int {
                        jx = *(*mat).col_mat_rownr.offset(jb as isize);
                        if isActiveLink(sets, jx) != 0 {
                            break;
                        }
                        jb = presolve_nextrow(psdata, i, &raw mut item);
                    }
                    if jb >= 0 as ::core::ffi::c_int {
                        (*QS.offset(n as isize)).int4.intval = i;
                        item = 0 as ::core::ffi::c_int;
                        ii = presolve_nextrow(psdata, i, &raw mut item);
                        (*QS.offset(n as isize)).int4.intpar1 =
                            *(*mat).col_mat_rownr.offset(ii as isize);
                        ii = presolve_collength(psdata, i);
                        (*QS.offset(n as isize)).int4.intpar2 = ii;
                        n += 1;
                    }
                }
                i = nextActiveLink((*(*psdata).cols).varmap, i);
            }
            if n <= 1 as ::core::ffi::c_int {
                if !(QS as *mut ::core::ffi::c_void).is_null() {
                    free(QS as *mut ::core::ffi::c_void);
                    QS = ::core::ptr::null_mut::<QSORTrec>();
                }
                return status;
            }
            QS_execute(
                QS as *mut QSORTrec,
                n,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *const QSORTrec,
                            *const QSORTrec,
                        ) -> ::core::ffi::c_int,
                    >,
                    Option<findCompare_func>,
                >(Some(
                    compRedundant
                        as unsafe extern "C" fn(
                            *const QSORTrec,
                            *const QSORTrec,
                        ) -> ::core::ffi::c_int,
                )),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if !(allocREAL(
                lp,
                &raw mut colvalues,
                nrows + 1 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            ) == 0
                || allocREAL(
                    lp,
                    &raw mut colobj,
                    n + 1 as ::core::ffi::c_int,
                    FALSE as ::core::ffi::c_uchar,
                ) == 0
                || allocINT(
                    lp,
                    &raw mut coldel,
                    n + 1 as ::core::ffi::c_int,
                    FALSE as ::core::ffi::c_uchar,
                ) == 0)
            {
                ib = 0 as ::core::ffi::c_int;
                's_210: while ib < n {
                    i = (*QS.offset(ib as isize)).int4.intval;
                    if !(isActiveLink((*(*psdata).cols).varmap, i) == 0) {
                        item = 0 as ::core::ffi::c_int;
                        jb = presolve_nextrow(psdata, i, &raw mut item);
                        while jb >= 0 as ::core::ffi::c_int {
                            jx = *(*mat).col_mat_rownr.offset(jb as isize);
                            *colvalues.offset(jx as isize) =
                                *(*mat).col_mat_value.offset(jb as isize);
                            jb = presolve_nextrow(psdata, i, &raw mut item);
                        }
                        *coldel.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
                        *coldel.offset(1 as ::core::ffi::c_int as isize) = i;
                        *colobj.offset(1 as ::core::ffi::c_int as isize) =
                            *(*lp).orig_obj.offset(i as isize);
                        ie = ib + 1 as ::core::ffi::c_int;
                        while ie < n {
                            ii = (*QS.offset(ie as isize)).int4.intval;
                            if !(isActiveLink((*(*psdata).cols).varmap, ii) == 0) {
                                ii = (*QS.offset(ib as isize)).int4.intpar2
                                    - (*QS.offset(ie as isize)).int4.intpar2;
                                if ii != 0 as ::core::ffi::c_int {
                                    break;
                                }
                                ii = (*QS.offset(ib as isize)).int4.intpar1
                                    - (*QS.offset(ie as isize)).int4.intpar1;
                                if ii != 0 as ::core::ffi::c_int {
                                    break;
                                }
                                ii = (*QS.offset(ie as isize)).int4.intval;
                                rhsval = (*lp).infinite;
                                item = 0 as ::core::ffi::c_int;
                                item2 = 0 as ::core::ffi::c_int;
                                jb = presolve_nextrow(psdata, ii, &raw mut item);
                                jj = presolve_nextrow(psdata, i, &raw mut item2);
                                while jb >= 0 as ::core::ffi::c_int {
                                    jx = *(*mat).col_mat_rownr.offset(jb as isize);
                                    if jx != *(*mat).col_mat_rownr.offset(jj as isize) {
                                        break;
                                    }
                                    if isActiveLink(sets, jx) != 0 {
                                        if rhsval > *(*lp).orig_rhs.offset(jx as isize) {
                                            rhsval = *(*lp).orig_rhs.offset(jx as isize);
                                        }
                                    }
                                    jb = presolve_nextrow(psdata, ii, &raw mut item);
                                    jj = presolve_nextrow(psdata, i, &raw mut item2);
                                }
                                if jb < 0 as ::core::ffi::c_int {
                                    let ref mut fresh45 =
                                        *coldel.offset(0 as ::core::ffi::c_int as isize);
                                    *fresh45 += 1;
                                    *coldel.offset(*fresh45 as isize) = ii;
                                    *colobj
                                        .offset(*coldel.offset(0 as ::core::ffi::c_int as isize)
                                            as isize) = *(*lp).orig_obj.offset(ii as isize);
                                }
                            }
                            ie += 1;
                        }
                        if *coldel.offset(0 as ::core::ffi::c_int as isize)
                            > 1 as ::core::ffi::c_int
                        {
                            qsortex(
                                colobj.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_void,
                                *coldel.offset(0 as ::core::ffi::c_int as isize),
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<::core::ffi::c_double>()
                                    as ::core::ffi::c_int,
                                FALSE as ::core::ffi::c_uchar,
                                Some(
                                    compareREAL
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_void,
                                            *const ::core::ffi::c_void,
                                        )
                                            -> ::core::ffi::c_int,
                                ),
                                coldel.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
                            );
                            jb = (rhsval + (*lp).epsvalue) as ::core::ffi::c_int;
                            jb += 1;
                            while jb <= *coldel.offset(0 as ::core::ffi::c_int as isize) {
                                jx = *coldel.offset(jb as isize);
                                if presolve_colfix(
                                    psdata,
                                    jx,
                                    *(*lp).orig_lowbo.offset((nrows + jx) as isize),
                                    TRUE as ::core::ffi::c_uchar,
                                    &raw mut iVarFixed,
                                ) == 0
                                {
                                    status = presolve_setstatusex(
                                        psdata,
                                        2 as ::core::ffi::c_int,
                                        4224 as ::core::ffi::c_int,
                                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                            as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                    break 's_210;
                                } else {
                                    presolve_colremove(psdata, jx, TRUE as ::core::ffi::c_uchar);
                                    jb += 1;
                                }
                            }
                        }
                        if (ib + 1 as ::core::ffi::c_int) < n {
                            ie = *(*mat)
                                .col_end
                                .offset((i - 1 as ::core::ffi::c_int) as isize);
                            ii = *(*mat).col_end.offset(i as isize);
                            while ie < ii {
                                *colvalues
                                    .offset(*(*mat).col_mat_rownr.offset(ie as isize) as isize) =
                                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                ie += 1;
                            }
                        }
                    }
                    ib += 1;
                }
            }
        }
    }
    freeLink(&raw mut sets);
    if !(QS as *mut ::core::ffi::c_void).is_null() {
        free(QS as *mut ::core::ffi::c_void);
        QS = ::core::ptr::null_mut::<QSORTrec>();
    }
    if !(colvalues as *mut ::core::ffi::c_void).is_null() {
        free(colvalues as *mut ::core::ffi::c_void);
        colvalues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(coldel as *mut ::core::ffi::c_void).is_null() {
        free(coldel as *mut ::core::ffi::c_void);
        coldel = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(colobj as *mut ::core::ffi::c_void).is_null() {
        free(colobj as *mut ::core::ffi::c_void);
        colobj = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    *nVarsFixed += iVarFixed;
    *nSum += iVarFixed;
    return status;
}
#[export_name="honest_lpsolve_presolve_aggregate"]
pub unsafe extern "C" fn presolve_aggregate(
    mut psdata: *mut presolverec,
    mut nConRemoved: *mut ::core::ffi::c_int,
    mut nVarsFixed: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut first: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut item2: ::core::ffi::c_int = 0;
    let mut coldel: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut iVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut scale: ::core::ffi::c_double = 0.;
    let mut colvalues: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut QScand: *mut QSORTrec = calloc(
        ((*lp).columns + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<QSORTrec>() as size_t,
    ) as *mut QSORTrec;
    if QScand.is_null() {
        return status;
    }
    n = 0 as ::core::ffi::c_int;
    i = firstActiveLink((*(*psdata).cols).varmap);
    while i != 0 as ::core::ffi::c_int {
        if is_semicont(lp, i) == 0 && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, i) == 0 {
            (*QScand.offset(n as isize)).int4.intval = i;
            item = 0 as ::core::ffi::c_int;
            ii = presolve_nextrow(psdata, i, &raw mut item);
            (*QScand.offset(n as isize)).int4.intpar1 = *(*mat).col_mat_rownr.offset(ii as isize);
            ii = presolve_collength(psdata, i);
            (*QScand.offset(n as isize)).int4.intpar2 = ii;
            n += 1;
        }
        i = nextActiveLink((*(*psdata).cols).varmap, i);
    }
    if n <= 1 as ::core::ffi::c_int {
        if !(QScand as *mut ::core::ffi::c_void).is_null() {
            free(QScand as *mut ::core::ffi::c_void);
            QScand = ::core::ptr::null_mut::<QSORTrec>();
        }
        return status;
    }
    QS_execute(
        QScand as *mut QSORTrec,
        n,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int>,
            Option<findCompare_func>,
        >(Some(
            compRedundant
                as unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int,
        )),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if !(allocREAL(
        lp,
        &raw mut colvalues,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
        || allocINT(
            lp,
            &raw mut coldel,
            (*lp).columns + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) == 0)
    {
        ib = 0 as ::core::ffi::c_int;
        while ib < n {
            i = (*QScand.offset(ib as isize)).int4.intval;
            if !(i < 0 as ::core::ffi::c_int) {
                item = 0 as ::core::ffi::c_int;
                jb = presolve_nextrow(psdata, i, &raw mut item);
                while jb >= 0 as ::core::ffi::c_int {
                    jx = *(*mat).col_mat_rownr.offset(jb as isize);
                    *colvalues.offset(jx as isize) = *(*mat).col_mat_value.offset(jb as isize);
                    jb = presolve_nextrow(psdata, i, &raw mut item);
                }
                *coldel.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
                ie = ib + 1 as ::core::ffi::c_int;
                while ie < n {
                    ii = (*QScand.offset(ib as isize)).int4.intpar2
                        - (*QScand.offset(ie as isize)).int4.intpar2;
                    if ii != 0 as ::core::ffi::c_int {
                        break;
                    }
                    ii = (*QScand.offset(ib as isize)).int4.intpar1
                        - (*QScand.offset(ie as isize)).int4.intpar1;
                    if ii != 0 as ::core::ffi::c_int {
                        break;
                    }
                    ii = (*QScand.offset(ie as isize)).int4.intval;
                    if !(ii < 0 as ::core::ffi::c_int) {
                        first = TRUE as ::core::ffi::c_uchar;
                        item = 0 as ::core::ffi::c_int;
                        item2 = 0 as ::core::ffi::c_int;
                        scale = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                        jb = presolve_nextrow(psdata, ii, &raw mut item);
                        jj = presolve_nextrow(psdata, i, &raw mut item2);
                        while jb >= 0 as ::core::ffi::c_int {
                            jx = *(*mat).col_mat_rownr.offset(jb as isize);
                            if jx != *(*mat).col_mat_rownr.offset(jj as isize) {
                                break;
                            }
                            if first != 0 {
                                first = (first == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                                scale = *colvalues.offset(jx as isize)
                                    / *(*mat).col_mat_value.offset(jb as isize);
                            } else if fabs(
                                *colvalues.offset(jx as isize)
                                    - scale * *(*mat).col_mat_value.offset(jb as isize),
                            ) > (*psdata).epsvalue
                            {
                                break;
                            }
                            jb = presolve_nextrow(psdata, ii, &raw mut item);
                            jj = presolve_nextrow(psdata, i, &raw mut item2);
                        }
                        if jb < 0 as ::core::ffi::c_int {
                            let ref mut fresh43 = *coldel.offset(0 as ::core::ffi::c_int as isize);
                            *fresh43 += 1;
                            *coldel.offset(*fresh43 as isize) = ii;
                            (*QScand.offset(ie as isize)).int4.intval = -ii;
                        }
                    }
                    ie += 1;
                }
                if *coldel.offset(0 as ::core::ffi::c_int as isize) > 1 as ::core::ffi::c_int {
                    let mut of: ::core::ffi::c_double = 0.;
                    let mut ofelim: ::core::ffi::c_double = 0.;
                    let mut fixvalue: ::core::ffi::c_double = 0.;
                    let mut isint: ::core::ffi::c_uchar = 0;
                    let mut QSagg: *mut QSORTrec = calloc(
                        *coldel.offset(0 as ::core::ffi::c_int as isize) as size_t,
                        ::core::mem::size_of::<QSORTrec>() as size_t,
                    ) as *mut QSORTrec;
                    jb = 1 as ::core::ffi::c_int;
                    while jb <= *coldel.offset(0 as ::core::ffi::c_int as isize) {
                        ii = jb - 1 as ::core::ffi::c_int;
                        (*QSagg.offset(ii as isize)).pvoidint2.intval = *coldel.offset(jb as isize);
                        let ref mut fresh44 = (*QSagg.offset(ii as isize)).pvoidint2.ptr;
                        *fresh44 = lp as *mut ::core::ffi::c_void;
                        jb += 1;
                    }
                    QS_execute(
                        QSagg as *mut QSORTrec,
                        *coldel.offset(0 as ::core::ffi::c_int as isize),
                        ::core::mem::transmute::<
                            Option<
                                unsafe extern "C" fn(
                                    *const QSORTrec,
                                    *const QSORTrec,
                                )
                                    -> ::core::ffi::c_int,
                            >,
                            Option<findCompare_func>,
                        >(Some(
                            compAggregate
                                as unsafe extern "C" fn(
                                    *const QSORTrec,
                                    *const QSORTrec,
                                )
                                    -> ::core::ffi::c_int,
                        )),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                    jb = 0 as ::core::ffi::c_int;
                    while status == RUNNING && jb < *coldel.offset(0 as ::core::ffi::c_int as isize)
                    {
                        ii = (*QSagg.offset(jb as isize)).pvoidint2.intval;
                        of = *(*lp).orig_obj.offset(ii as isize);
                        isint = is_int(lp, ii);
                        je = jb + 1 as ::core::ffi::c_int;
                        while status == RUNNING
                            && je < *coldel.offset(0 as ::core::ffi::c_int as isize)
                            && {
                                ix = (*QSagg.offset(je as isize)).pvoidint2.intval;
                                fabs(*(*lp).orig_obj.offset(ix as isize) - of) < (*psdata).epsvalue
                            }
                        {
                            if is_int(lp, ix) as ::core::ffi::c_int == isint as ::core::ffi::c_int {
                                ofelim = *(*lp).orig_obj.offset(ix as isize);
                                if of == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    scale = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                                } else {
                                    scale = ofelim / of;
                                }
                                if (fabs(*(*lp).orig_upbo.offset(((*lp).rows + ii) as isize))
                                    >= (*lp).infinite)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uchar
                                    != 0
                                {
                                    if is_unbounded(lp, ix) != 0 {
                                        fixvalue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                    } else if ofelim
                                        < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        fixvalue =
                                            *(*lp).orig_upbo.offset(((*lp).rows + ix) as isize);
                                    } else {
                                        fixvalue =
                                            *(*lp).orig_lowbo.offset(((*lp).rows + ix) as isize);
                                    }
                                    if (fabs(fixvalue) >= (*lp).infinite) as ::core::ffi::c_int
                                        as ::core::ffi::c_uchar
                                        != 0
                                    {
                                        status = presolve_setstatusex(
                                            psdata,
                                            3 as ::core::ffi::c_int,
                                            4408 as ::core::ffi::c_int,
                                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    } else if presolve_colfix(
                                        psdata,
                                        ix,
                                        fixvalue,
                                        TRUE as ::core::ffi::c_uchar,
                                        &raw mut iVarFixed,
                                    ) == 0
                                    {
                                        status = presolve_setstatusex(
                                            psdata,
                                            2 as ::core::ffi::c_int,
                                            4410 as ::core::ffi::c_int,
                                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    } else {
                                        presolve_colremove(
                                            psdata,
                                            ix,
                                            TRUE as ::core::ffi::c_uchar,
                                        );
                                    }
                                } else if !((fabs(
                                    *(*lp).orig_lowbo.offset(((*lp).rows + ii) as isize),
                                ) >= (*lp).infinite)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uchar
                                    != 0)
                                {
                                    if ofelim >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        fixvalue =
                                            *(*lp).orig_lowbo.offset(((*lp).rows + ix) as isize);
                                        *(*lp).orig_upbo.offset(((*lp).rows + ii) as isize) += scale
                                            * (*(*lp).orig_upbo.offset(((*lp).rows + ix) as isize)
                                                - fixvalue);
                                    } else {
                                        fixvalue =
                                            *(*lp).orig_upbo.offset(((*lp).rows + ix) as isize);
                                        *(*lp).orig_upbo.offset(((*lp).rows + ii) as isize) -= scale
                                            * (fixvalue
                                                - *(*lp)
                                                    .orig_lowbo
                                                    .offset(((*lp).rows + ix) as isize));
                                    }
                                    if (fabs(fixvalue) >= (*lp).infinite) as ::core::ffi::c_int
                                        as ::core::ffi::c_uchar
                                        != 0
                                    {
                                        status = presolve_setstatusex(
                                            psdata,
                                            3 as ::core::ffi::c_int,
                                            4432 as ::core::ffi::c_int,
                                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    } else if presolve_colfix(
                                        psdata,
                                        ix,
                                        fixvalue,
                                        TRUE as ::core::ffi::c_uchar,
                                        &raw mut iVarFixed,
                                    ) == 0
                                    {
                                        status = presolve_setstatusex(
                                            psdata,
                                            2 as ::core::ffi::c_int,
                                            4434 as ::core::ffi::c_int,
                                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                        );
                                    } else {
                                        presolve_colremove(
                                            psdata,
                                            ix,
                                            TRUE as ::core::ffi::c_uchar,
                                        );
                                    }
                                    (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
                                }
                            }
                            je += 1;
                        }
                        jb = je;
                    }
                    if !(QSagg as *mut ::core::ffi::c_void).is_null() {
                        free(QSagg as *mut ::core::ffi::c_void);
                        QSagg = ::core::ptr::null_mut::<QSORTrec>();
                    }
                }
                if (ib + 1 as ::core::ffi::c_int) < n {
                    ie = *(*mat)
                        .col_end
                        .offset((i - 1 as ::core::ffi::c_int) as isize);
                    ii = *(*mat).col_end.offset(i as isize);
                    while ie < ii {
                        *colvalues.offset(*(*mat).col_mat_rownr.offset(ie as isize) as isize) =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        ie += 1;
                    }
                }
            }
            ib += 1;
        }
    }
    if !(QScand as *mut ::core::ffi::c_void).is_null() {
        free(QScand as *mut ::core::ffi::c_void);
        QScand = ::core::ptr::null_mut::<QSORTrec>();
    }
    if !(colvalues as *mut ::core::ffi::c_void).is_null() {
        free(colvalues as *mut ::core::ffi::c_void);
        colvalues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(coldel as *mut ::core::ffi::c_void).is_null() {
        free(coldel as *mut ::core::ffi::c_void);
        coldel = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    *nVarsFixed += iVarFixed;
    *nSum += iVarFixed;
    return status;
}
#[export_name="honest_lpsolve_presolve_makesparser"]
pub unsafe extern "C" fn presolve_makesparser(
    mut psdata: *mut presolverec,
    mut nCoeffChanged: *mut ::core::ffi::c_int,
    mut nConRemove: *mut ::core::ffi::c_int,
    mut nVarFixed: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut chsign: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut jl: ::core::ffi::c_int = 0;
    let mut jjb: ::core::ffi::c_int = 0;
    let mut jje: ::core::ffi::c_int = 0;
    let mut jjl: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut jjx: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut itemEQ: ::core::ffi::c_int = 0;
    let mut nzidx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut iObjChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iConRemove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut test: ::core::ffi::c_double = 0.;
    let mut ratio: ::core::ffi::c_double = 0.;
    let mut value: ::core::ffi::c_double = 0.;
    let mut valueEQ: ::core::ffi::c_double = 0.;
    let mut valptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut EQlist: *mut LLrec = ::core::ptr::null_mut::<LLrec>();
    let mut QS: *mut QSORTrec = calloc(
        (*lp).rows as size_t,
        ::core::mem::size_of::<QSORTrec>() as size_t,
    ) as *mut QSORTrec;
    if QS.is_null()
        || (*(*(*psdata).rows).varmap).count == 0 as ::core::ffi::c_int
        || (*(*psdata).EQmap).count == 0 as ::core::ffi::c_int
    {
        return status;
    }
    n = 0 as ::core::ffi::c_int;
    i = firstActiveLink((*(*psdata).rows).varmap);
    while i != 0 as ::core::ffi::c_int {
        k = presolve_rowlength(psdata, i);
        if k >= 2 as ::core::ffi::c_int {
            item = 0 as ::core::ffi::c_int;
            ii = presolve_nextcol(psdata, i, &raw mut item);
            (*QS.offset(n as isize)).int4.intval =
                if is_constr_type(lp, i, 3 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                    && i != 0 as ::core::ffi::c_int
                {
                    -i
                } else {
                    i
                };
            (*QS.offset(n as isize)).int4.intpar1 = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(ii as isize) as isize);
            (*QS.offset(n as isize)).int4.intpar2 = k;
            n += 1;
        }
        i = nextActiveLink((*(*psdata).rows).varmap, i);
    }
    if n <= 1 as ::core::ffi::c_int {
        if !(QS as *mut ::core::ffi::c_void).is_null() {
            free(QS as *mut ::core::ffi::c_void);
            QS = ::core::ptr::null_mut::<QSORTrec>();
        }
        return status;
    }
    QS_execute(
        QS as *mut QSORTrec,
        n,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int>,
            Option<findCompare_func>,
        >(Some(
            compSparsity
                as unsafe extern "C" fn(*const QSORTrec, *const QSORTrec) -> ::core::ffi::c_int,
        )),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    allocINT(
        lp,
        &raw mut nzidx,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    createLink(
        (*lp).rows,
        &raw mut EQlist,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    );
    ib = 0 as ::core::ffi::c_int;
    while ib < n {
        i = (*QS.offset(ib as isize)).int4.intval;
        if i < 0 as ::core::ffi::c_int {
            appendLink(EQlist, ib + 1 as ::core::ffi::c_int);
        }
        ib += 1;
    }
    ix = firstActiveLink(EQlist);
    's_134: while ix != 0 as ::core::ffi::c_int {
        ii = abs((*QS.offset((ix - 1 as ::core::ffi::c_int) as isize))
            .int4
            .intval);
        jjb = (*QS.offset((ix - 1 as ::core::ffi::c_int) as isize))
            .int4
            .intpar1;
        jje = presolve_lastcol(psdata, ii);
        jje = *(*mat)
            .col_mat_colnr
            .offset(*(*mat).row_mat.offset(jje as isize) as isize);
        jjl = (*QS.offset((ix - 1 as ::core::ffi::c_int) as isize))
            .int4
            .intpar2;
        i = 0 as ::core::ffi::c_int;
        chsign = is_chsign(lp, i);
        ratio = 0.0f64;
        test = ratio;
        itemEQ = 0 as ::core::ffi::c_int;
        *nzidx.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
        loop {
            jjx = presolve_nextcol(psdata, ii, &raw mut itemEQ);
            if !(jjx >= 0 as ::core::ffi::c_int && fabs(test - ratio) < (*psdata).epsvalue) {
                break;
            }
            valueEQ = *(*mat)
                .col_mat_value
                .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
            if valueEQ == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                continue;
            }
            k = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
            value = *(*lp).orig_obj.offset(k as isize);
            if fabs(value) < (*psdata).epsvalue {
                break;
            }
            if ratio == 0.0f64 {
                ratio = value / valueEQ;
                test = ratio;
            } else {
                test = value / valueEQ;
            }
            let ref mut fresh19 = *nzidx.offset(0 as ::core::ffi::c_int as isize);
            *fresh19 += 1;
            *nzidx.offset(*fresh19 as isize) = k;
        }
        if itemEQ == 0 as ::core::ffi::c_int
            && *nzidx.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int
            && fabs(test - ratio) < (*psdata).epsvalue
        {
            k = 1 as ::core::ffi::c_int;
            while k <= *nzidx.offset(0 as ::core::ffi::c_int as isize) {
                jx = *nzidx.offset(k as isize);
                value = *(*lp).orig_obj.offset(jx as isize);
                *(*lp).orig_obj.offset(jx as isize) = 0.0f64;
                value = if chsign as ::core::ffi::c_int != 0
                    && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -value
                } else {
                    value
                };
                if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    let ref mut fresh20 = *(*(*psdata).rows).negcount.offset(i as isize);
                    *fresh20 -= 1;
                    let ref mut fresh21 = *(*(*psdata).cols).negcount.offset(jx as isize);
                    *fresh21 -= 1;
                } else {
                    let ref mut fresh22 = *(*(*psdata).rows).plucount.offset(i as isize);
                    *fresh22 -= 1;
                    let ref mut fresh23 = *(*(*psdata).cols).plucount.offset(jx as isize);
                    *fresh23 -= 1;
                }
                iObjChanged += 1;
                k += 1;
            }
            value = ratio * *(*lp).orig_rhs.offset(ii as isize);
            presolve_adjustrhs(psdata, i, value, (*psdata).epsvalue);
        }
        ib = 1 as ::core::ffi::c_int;
        's_291: while ib < ix {
            i = abs((*QS.offset((ib - 1 as ::core::ffi::c_int) as isize))
                .int4
                .intval);
            jb = (*QS.offset((ib - 1 as ::core::ffi::c_int) as isize))
                .int4
                .intpar1;
            je = presolve_lastcol(psdata, i);
            je = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(je as isize) as isize);
            jl = (*QS.offset((ib - 1 as ::core::ffi::c_int) as isize))
                .int4
                .intpar2;
            if jjb < jb || jje > je || jjl > jl {
                break;
            }
            chsign = is_chsign(lp, i);
            ratio = 0.0f64;
            test = ratio;
            itemEQ = 0 as ::core::ffi::c_int;
            item = 0 as ::core::ffi::c_int;
            *nzidx.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
            loop {
                jjx = presolve_nextcol(psdata, ii, &raw mut itemEQ);
                if !(jjx >= 0 as ::core::ffi::c_int && fabs(test - ratio) < (*psdata).epsvalue) {
                    break;
                }
                valueEQ = *(*mat)
                    .col_mat_value
                    .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
                if valueEQ == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    continue;
                }
                jx = 0 as ::core::ffi::c_int;
                jjx = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
                k = presolve_nextcol(psdata, i, &raw mut item);
                while jx < jjx && item > 0 as ::core::ffi::c_int {
                    jx = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(k as isize) as isize);
                    if jx == jjx {
                        value = *(*mat)
                            .col_mat_value
                            .offset(*(*mat).row_mat.offset(k as isize) as isize);
                        if value == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            break 's_291;
                        }
                        if ratio == 0.0f64 {
                            ratio = value / valueEQ;
                            test = ratio;
                        } else {
                            test = value / valueEQ;
                        }
                        let ref mut fresh24 = *nzidx.offset(0 as ::core::ffi::c_int as isize);
                        *fresh24 += 1;
                        *nzidx.offset(*fresh24 as isize) = k;
                        break;
                    } else {
                        if jx > jjx {
                            break 's_291;
                        }
                        k = presolve_nextcol(psdata, i, &raw mut item);
                    }
                }
            }
            if itemEQ == 0 as ::core::ffi::c_int
                && *nzidx.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int
                && fabs(test - ratio) < (*psdata).epsvalue
            {
                if presolve_rowlength(psdata, i) == presolve_rowlength(psdata, ii) {
                    value = *(*lp).orig_rhs.offset(i as isize);
                    valueEQ = *(*lp).orig_rhs.offset(ii as isize);
                    if is_constr_type(lp, i, EQ) != 0 {
                        if fabs(valueEQ) < (*psdata).epsvalue {
                            if fabs(value) < (*psdata).epsvalue {
                                test = ratio;
                            } else {
                                test = (*lp).infinite;
                            }
                        } else {
                            test = value / valueEQ;
                        }
                        if fabs(test - ratio) > (*psdata).epsvalue {
                            report(
                                lp,
                                4 as ::core::ffi::c_int,
                                b"presolve_sparser: Infeasibility of relatively equal constraints %d and %d\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            status = presolve_setstatusex(
                                psdata,
                                2 as ::core::ffi::c_int,
                                4663 as ::core::ffi::c_int,
                                b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            break 's_134;
                        } else {
                            removeLink(EQlist, i);
                            presolve_rowremove(psdata, i, TRUE as ::core::ffi::c_uchar);
                            memcpy(
                                QS.offset((ib - 1 as ::core::ffi::c_int) as isize) as *mut QSORTrec
                                    as *mut ::core::ffi::c_void,
                                QS.offset(ib as isize) as *mut QSORTrec
                                    as *const ::core::ffi::c_void,
                                ((n - ib) as size_t)
                                    .wrapping_mul(::core::mem::size_of::<QSORTrec>() as size_t),
                            );
                            n -= 1;
                            iConRemove += 1;
                        }
                    } else if value + (*psdata).epsvalue < valueEQ
                        || value - get_rh_range(lp, i) - (*psdata).epsvalue > valueEQ
                    {
                        report(
                            lp,
                            4 as ::core::ffi::c_int,
                            b"presolve_sparser: Infeasibility of relatively equal RHS values for %d and %d\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        status = presolve_setstatusex(
                            psdata,
                            2 as ::core::ffi::c_int,
                            4682 as ::core::ffi::c_int,
                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        break 's_134;
                    } else {
                        presolve_rowremove(psdata, i, TRUE as ::core::ffi::c_uchar);
                        memcpy(
                            QS.offset((ib - 1 as ::core::ffi::c_int) as isize) as *mut QSORTrec
                                as *mut ::core::ffi::c_void,
                            QS.offset(ib as isize) as *mut QSORTrec as *const ::core::ffi::c_void,
                            ((n - ib) as size_t)
                                .wrapping_mul(::core::mem::size_of::<QSORTrec>() as size_t),
                        );
                        n -= 1;
                        iConRemove += 1;
                    }
                } else {
                    k = 1 as ::core::ffi::c_int;
                    while k <= *nzidx.offset(0 as ::core::ffi::c_int as isize) {
                        jjx = *nzidx.offset(k as isize);
                        jx = *(*mat)
                            .col_mat_colnr
                            .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
                        valptr = (*mat)
                            .col_mat_value
                            .offset(*(*mat).row_mat.offset(jjx as isize) as isize)
                            as *mut ::core::ffi::c_double;
                        value = *valptr;
                        *valptr = 0.0f64;
                        value = if chsign as ::core::ffi::c_int != 0
                            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -value
                        } else {
                            value
                        };
                        if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            let ref mut fresh25 = *(*(*psdata).rows).negcount.offset(i as isize);
                            *fresh25 -= 1;
                            let ref mut fresh26 = *(*(*psdata).cols).negcount.offset(jx as isize);
                            *fresh26 -= 1;
                        } else {
                            let ref mut fresh27 = *(*(*psdata).rows).plucount.offset(i as isize);
                            *fresh27 -= 1;
                            let ref mut fresh28 = *(*(*psdata).cols).plucount.offset(jx as isize);
                            *fresh28 -= 1;
                        }
                        iCoeffChanged += 1;
                        k += 1;
                    }
                    value = ratio * *(*lp).orig_rhs.offset(ii as isize);
                    presolve_adjustrhs(psdata, i, value, (*psdata).epsvalue);
                }
            }
            ib += 1;
        }
        ix = nextActiveLink(EQlist, ix);
    }
    if !(QS as *mut ::core::ffi::c_void).is_null() {
        free(QS as *mut ::core::ffi::c_void);
        QS = ::core::ptr::null_mut::<QSORTrec>();
    }
    freeLink(&raw mut EQlist);
    if !(nzidx as *mut ::core::ffi::c_void).is_null() {
        free(nzidx as *mut ::core::ffi::c_void);
        nzidx = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if iCoeffChanged > 0 as ::core::ffi::c_int {
        (*mat).row_end_valid = FALSE as ::core::ffi::c_uchar;
        mat_zerocompact(mat);
        presolve_validate(psdata, TRUE as ::core::ffi::c_uchar);
        (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
    }
    *nConRemove += iConRemove;
    *nCoeffChanged += iCoeffChanged + iObjChanged;
    *nSum += iCoeffChanged + iObjChanged + iConRemove;
    return status;
}
#[export_name="honest_lpsolve_presolve_SOS1"]
pub unsafe extern "C" fn presolve_SOS1(
    mut psdata: *mut presolverec,
    mut nCoeffChanged: *mut ::core::ffi::c_int,
    mut nConRemove: *mut ::core::ffi::c_int,
    mut nVarFixed: *mut ::core::ffi::c_int,
    mut nSOS: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut candelete: ::core::ffi::c_uchar = 0;
    let mut SOS_GUBactive: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iConRemove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iSOS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut jjx: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut Value1: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    i = lastActiveLink((*(*psdata).rows).varmap);
    while i > 0 as ::core::ffi::c_int {
        candelete = FALSE as ::core::ffi::c_uchar;
        Value1 = get_rh(lp, i);
        jx = get_constr_type(lp, i);
        if Value1 == 1 as ::core::ffi::c_int as ::core::ffi::c_double
            && presolve_rowlength(psdata, i) >= MIN_SOS1LENGTH
            && (SOS_GUBactive as ::core::ffi::c_int != 0 && jx != GE
                || SOS_GUBactive == 0 && jx == LE)
        {
            jjx = *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            iix = *(*mat).row_end.offset(i as isize);
            while jjx < iix {
                j = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
                if !(isActiveLink((*(*psdata).cols).varmap, j) == 0) {
                    if is_binary(lp, j) == 0
                        || *(*mat)
                            .col_mat_value
                            .offset(*(*mat).row_mat.offset(jjx as isize) as isize)
                            != 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        break;
                    }
                }
                jjx += 1;
            }
            if jjx >= iix {
                let mut SOSname: [::core::ffi::c_char; 16] = [0; 16];
                ix = SOS_count(lp) + 1 as ::core::ffi::c_int;
                native_only!(snprintf,
                    &raw mut SOSname as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                    b"SOS_%d\0" as *const u8 as *const ::core::ffi::c_char,
                    ix,
                );
                ix = add_SOS(
                    lp,
                    &raw mut SOSname as *mut ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    ix,
                    0 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_double>(),
                );
                if jx == EQ {
                    SOS_set_GUB((*lp).SOS, ix, TRUE as ::core::ffi::c_uchar);
                }
                Value1 = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                jjx = *(*mat)
                    .row_end
                    .offset((i - 1 as ::core::ffi::c_int) as isize);
                while jjx < iix {
                    j = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(jjx as isize) as isize);
                    if !(isActiveLink((*(*psdata).cols).varmap, j) == 0) {
                        Value1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                        append_SOSrec(
                            *(*(*lp).SOS)
                                .sos_list
                                .offset((ix - 1 as ::core::ffi::c_int) as isize),
                            1 as ::core::ffi::c_int,
                            &raw mut j,
                            &raw mut Value1,
                        );
                    }
                    jjx += 1;
                }
                candelete = TRUE as ::core::ffi::c_uchar;
                iSOS += 1;
            }
        }
        ix = i;
        i = prevActiveLink((*(*psdata).rows).varmap, i);
        if candelete != 0 {
            presolve_rowremove(psdata, ix, TRUE as ::core::ffi::c_uchar);
            iConRemove += 1;
        }
    }
    if iSOS != 0 {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"presolve_SOS1: Converted %5d constraints to SOS1.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    clean_SOSgroup(
        (*lp).SOS,
        (iSOS > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar,
    );
    *nCoeffChanged += iCoeffChanged;
    *nConRemove += iConRemove;
    *nSOS += iSOS;
    *nSum += iCoeffChanged + iConRemove + iSOS;
    return status;
}
#[export_name="honest_lpsolve_presolve_boundconflict"]
pub unsafe extern "C" fn presolve_boundconflict(
    mut psdata: *mut presolverec,
    mut baserowno: ::core::ffi::c_int,
    mut colno: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut Value1: ::core::ffi::c_double = 0.;
    let mut Value2: ::core::ffi::c_double = 0.;
    let mut lp: *mut lprec = (*psdata).lp;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut ix: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = RUNNING;
    if baserowno <= 0 as ::core::ffi::c_int {
        loop {
            ix = presolve_nextrow(psdata, colno, &raw mut item);
            if ix < 0 as ::core::ffi::c_int {
                return status;
            }
            baserowno = *(*mat).col_mat_rownr.offset(ix as isize);
            if !(presolve_rowlength(psdata, baserowno) != 1 as ::core::ffi::c_int) {
                break;
            }
        }
    }
    Value1 = get_rh_upper(lp, baserowno);
    Value2 = get_rh_lower(lp, baserowno);
    if presolve_singletonbounds(
        psdata,
        baserowno,
        colno,
        &raw mut Value2,
        &raw mut Value1,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    ) != 0
    {
        let mut iix: ::core::ffi::c_int = 0;
        item = 0 as ::core::ffi::c_int;
        ix = presolve_nextrow(psdata, colno, &raw mut item);
        while ix >= 0 as ::core::ffi::c_int {
            iix = *(*mat).col_mat_rownr.offset(ix as isize);
            if iix != baserowno
                && presolve_rowlength(psdata, iix) == 1 as ::core::ffi::c_int
                && presolve_altsingletonvalid(psdata, iix, colno, Value2, Value1) == 0
            {
                status = presolve_setstatusex(
                    psdata,
                    2 as ::core::ffi::c_int,
                    4840 as ::core::ffi::c_int,
                    b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                break;
            } else {
                ix = presolve_nextrow(psdata, colno, &raw mut item);
            }
        }
    } else {
        status = presolve_setstatusex(
            psdata,
            2 as ::core::ffi::c_int,
            4846 as ::core::ffi::c_int,
            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return status;
}
#[export_name="honest_lpsolve_presolve_columns"]
pub unsafe extern "C" fn presolve_columns(
    mut psdata: *mut presolverec,
    mut nCoeffChanged: *mut ::core::ffi::c_int,
    mut nConRemove: *mut ::core::ffi::c_int,
    mut nVarFixed: *mut ::core::ffi::c_int,
    mut nBoundTighten: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut candelete: ::core::ffi::c_uchar = 0;
    let mut isOFNZ: ::core::ffi::c_uchar = 0;
    let mut probefix: ::core::ffi::c_uchar = is_presolve(lp, PRESOLVE_PROBEFIX);
    let mut colfixdual: ::core::ffi::c_uchar = is_presolve(lp, PRESOLVE_COLFIXDUAL);
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iConRemove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iBoundTighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut ix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut countNZ: ::core::ffi::c_int = 0;
    let mut Value1: ::core::ffi::c_double = 0.;
    j = firstActiveLink((*(*psdata).cols).varmap);
    while j != 0 as ::core::ffi::c_int && status == RUNNING {
        if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, j) != 0 {
            j = nextActiveLink((*(*psdata).cols).varmap, j);
        } else {
            countNZ = presolve_collength(psdata, j);
            isOFNZ = isnz_origobj(lp, j);
            Value1 = get_lowbo(lp, j);
            if (*lp).sc_vars > 0 as ::core::ffi::c_int
                && Value1 == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && is_semicont(lp, j) as ::core::ffi::c_int != 0
            {
                set_semicont(lp, j, FALSE as ::core::ffi::c_uchar);
            }
            candelete = FALSE as ::core::ffi::c_uchar;
            ix = (*lp).rows + j;
            if countNZ == 0 as ::core::ffi::c_int && isOFNZ == 0 {
                if Value1 != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_columns: Eliminated unused variable %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                candelete = TRUE as ::core::ffi::c_uchar;
            } else if countNZ == 0 as ::core::ffi::c_int && isOFNZ as ::core::ffi::c_int != 0 {
                if *(*lp).orig_obj.offset(j as isize)
                    < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    Value1 = get_upbo(lp, j);
                }
                if fabs(Value1) >= (*lp).infinite {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_columns: Unbounded variable %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    status = presolve_setstatusex(
                        psdata,
                        3 as ::core::ffi::c_int,
                        4900 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_columns: Eliminated trivial variable %s fixed at %g\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    candelete = TRUE as ::core::ffi::c_uchar;
                }
            } else if isOrigFixed(lp, ix) != 0 {
                if countNZ > 0 as ::core::ffi::c_int {
                    status = presolve_boundconflict(psdata, -(1 as ::core::ffi::c_int), j);
                    if status != RUNNING {
                        break;
                    }
                }
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"presolve_columns: Eliminated variable %s fixed at %g\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                candelete = TRUE as ::core::ffi::c_uchar;
            } else if colfixdual as ::core::ffi::c_int != 0
                && presolve_colfixdual(psdata, j, &raw mut Value1, &raw mut status)
                    as ::core::ffi::c_int
                    != 0
            {
                if (fabs(Value1) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar
                    != 0
                {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_columns: Unbounded variable %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    status = presolve_setstatusex(
                        psdata,
                        3 as ::core::ffi::c_int,
                        4958 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_columns: Eliminated dual-zero variable %s fixed at %g\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    candelete = TRUE as ::core::ffi::c_uchar;
                }
            } else if probefix as ::core::ffi::c_int != 0
                && is_binary(lp, j) as ::core::ffi::c_int != 0
                && presolve_probefix01(psdata, j, &raw mut Value1) as ::core::ffi::c_int != 0
            {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"presolve_columns: Fixed binary variable %s at %g\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                candelete = TRUE as ::core::ffi::c_uchar;
            }
            if candelete != 0 {
                if Value1 != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, j) != 0
                {
                    ix = iVarFixed;
                    if presolve_fixSOS1(psdata, j, Value1, &raw mut iConRemove, &raw mut iVarFixed)
                        == 0
                    {
                        status = presolve_setstatusex(
                            psdata,
                            2 as ::core::ffi::c_int,
                            4994 as ::core::ffi::c_int,
                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    if iVarFixed > ix {
                        (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
                    }
                    break;
                } else if presolve_colfix(
                    psdata,
                    j,
                    Value1,
                    TRUE as ::core::ffi::c_uchar,
                    &raw mut iVarFixed,
                ) == 0
                {
                    status = presolve_setstatusex(
                        psdata,
                        2 as ::core::ffi::c_int,
                        5001 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    break;
                } else {
                    j = presolve_colremove(psdata, j, TRUE as ::core::ffi::c_uchar);
                }
            } else {
                j = nextActiveLink((*(*psdata).cols).varmap, j);
            }
        }
    }
    if status == RUNNING {
        status = presolve_shrink(psdata, &raw mut iConRemove, &raw mut iVarFixed);
    }
    *nCoeffChanged += iCoeffChanged;
    *nConRemove += iConRemove;
    *nVarFixed += iVarFixed;
    *nBoundTighten += iBoundTighten;
    *nSum += iCoeffChanged + iConRemove + iVarFixed + iBoundTighten;
    return status;
}
#[export_name="honest_lpsolve_presolve_freeandslacks"]
pub unsafe extern "C" fn presolve_freeandslacks(
    mut psdata: *mut presolverec,
    mut nCoeffChanged: *mut ::core::ffi::c_int,
    mut nConRemove: *mut ::core::ffi::c_int,
    mut nVarFixed: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut isOFNZ: ::core::ffi::c_uchar = 0;
    let mut unbounded: ::core::ffi::c_uchar = 0;
    let mut impliedfree: ::core::ffi::c_uchar = is_presolve(lp, PRESOLVE_IMPLIEDFREE);
    let mut impliedslack: ::core::ffi::c_uchar = is_presolve(lp, PRESOLVE_IMPLIEDSLK);
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iConRemove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut countNZ: ::core::ffi::c_int = 0;
    let mut coeff_bl: ::core::ffi::c_double = 0.;
    let mut coeff_bu: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    if impliedfree as ::core::ffi::c_int != 0 || impliedslack as ::core::ffi::c_int != 0 {
        j = firstActiveLink((*(*psdata).cols).varmap);
        while j != 0 as ::core::ffi::c_int {
            if presolve_collength(psdata, j) != 1 as ::core::ffi::c_int
                || is_int(lp, j) as ::core::ffi::c_int != 0
                || is_semicont(lp, j) as ::core::ffi::c_int != 0
                || presolve_candeletevar(psdata, j) == 0
            {
                j = nextActiveLink((*(*psdata).cols).varmap, j);
            } else {
                ix = 0 as ::core::ffi::c_int;
                i = *(*mat)
                    .col_mat_rownr
                    .offset(presolve_nextrow(psdata, j, &raw mut ix) as isize);
                isOFNZ = isnz_origobj(lp, j);
                countNZ = presolve_rowlength(psdata, i);
                coeff_bu = get_upbo(lp, j);
                coeff_bl = get_lowbo(lp, j);
                unbounded = ((fabs(coeff_bl) >= (*lp).infinite) as ::core::ffi::c_int
                    as ::core::ffi::c_uchar as ::core::ffi::c_int
                    != 0
                    && (fabs(coeff_bu) >= (*lp).infinite) as ::core::ffi::c_int
                        as ::core::ffi::c_uchar as ::core::ffi::c_int
                        != 0) as ::core::ffi::c_int
                    as ::core::ffi::c_uchar;
                ix = (*lp).rows + j;
                if impliedfree as ::core::ffi::c_int != 0
                    && unbounded as ::core::ffi::c_int != 0
                    && presolve_impliedcolfix(psdata, i, j, TRUE as ::core::ffi::c_uchar)
                        as ::core::ffi::c_int
                        != 0
                {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_freeandslacks: Eliminated free variable %s and row %s\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    presolve_rowremove(psdata, i, TRUE as ::core::ffi::c_uchar);
                    iConRemove += 1;
                    j = presolve_colremove(psdata, j, TRUE as ::core::ffi::c_uchar);
                    iVarFixed += 1;
                } else if impliedslack as ::core::ffi::c_int != 0
                    && countNZ > 1 as ::core::ffi::c_int
                    && is_constr_type(lp, i, EQ) as ::core::ffi::c_int != 0
                    && presolve_impliedcolfix(psdata, i, j, FALSE as ::core::ffi::c_uchar)
                        as ::core::ffi::c_int
                        != 0
                {
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_freeandslacks: Eliminated implied slack variable %s via row %s\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
                    j = presolve_colremove(psdata, j, TRUE as ::core::ffi::c_uchar);
                    iVarFixed += 1;
                } else if impliedslack as ::core::ffi::c_int != 0
                    && isOFNZ == 0
                    && (fabs(coeff_bu) >= (*lp).infinite) as ::core::ffi::c_int
                        as ::core::ffi::c_uchar as ::core::ffi::c_int
                        != 0
                    && (fabs(coeff_bl) >= (*lp).infinite) as ::core::ffi::c_int
                        as ::core::ffi::c_uchar
                        == 0
                    && countNZ > 1 as ::core::ffi::c_int
                    && is_constr_type(lp, i, EQ) == 0
                {
                    let mut target: *mut ::core::ffi::c_double =
                        ::core::ptr::null_mut::<::core::ffi::c_double>();
                    let mut ValueA: ::core::ffi::c_double = *(*mat)
                        .col_mat_value
                        .offset(presolve_lastrow(psdata, j) as isize);
                    if coeff_bl != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && (fabs(coeff_bl) >= (*lp).infinite) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar
                            == 0
                        && (fabs(coeff_bu) >= (*lp).infinite) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar
                            == 0
                    {
                        coeff_bu -= coeff_bl;
                    }
                    if ValueA > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        target = (*lp).orig_upbo.offset(i as isize) as *mut ::core::ffi::c_double;
                        if (fabs(*target) >= (*lp).infinite) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar
                            == 0
                        {
                            if (fabs(coeff_bu) >= (*lp).infinite) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                != 0
                            {
                                *target = (*lp).infinite;
                                (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
                            } else {
                                *target += ValueA * coeff_bu;
                                *target =
                                    presolve_roundrhs(lp, *target, FALSE as ::core::ffi::c_uchar);
                            }
                        }
                    } else {
                        target = (*lp).orig_rhs.offset(i as isize) as *mut ::core::ffi::c_double;
                        if (fabs(coeff_bu) >= (*lp).infinite) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar as ::core::ffi::c_int
                            != 0
                            || (fabs(*target) >= (*lp).infinite) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                != 0
                        {
                            if (fabs(*(*lp).orig_upbo.offset(i as isize)) >= (*lp).infinite)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                != 0
                            {
                                presolve_rowremove(psdata, i, TRUE as ::core::ffi::c_uchar);
                                iConRemove += 1;
                            } else {
                                *target -= *(*lp).orig_upbo.offset(i as isize);
                                *target = -*target;
                                mat_multrow(
                                    mat,
                                    i,
                                    -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                                );
                                *(*lp).orig_upbo.offset(i as isize) = (*lp).infinite;
                                (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
                            }
                        } else {
                            *target -= ValueA * coeff_bu;
                            *target = presolve_roundrhs(lp, *target, FALSE as ::core::ffi::c_uchar);
                        }
                    }
                    presolve_colfix(
                        psdata,
                        j,
                        coeff_bl,
                        TRUE as ::core::ffi::c_uchar,
                        &raw mut iVarFixed,
                    );
                    report(
                        lp,
                        5 as ::core::ffi::c_int,
                        b"presolve_freeandslacks: Eliminated duplicate slack variable %s via row %s\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    j = presolve_colremove(psdata, j, TRUE as ::core::ffi::c_uchar);
                } else {
                    j = nextActiveLink((*(*psdata).cols).varmap, j);
                }
            }
        }
    }
    *nCoeffChanged += iCoeffChanged;
    *nConRemove += iConRemove;
    *nVarFixed += iVarFixed;
    *nSum += iCoeffChanged + iConRemove + iVarFixed;
    return status;
}
#[export_name="honest_lpsolve_presolve_preparerows"]
pub unsafe extern "C" fn presolve_preparerows(
    mut psdata: *mut presolverec,
    mut nBoundTighten: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut impliedfree: ::core::ffi::c_uchar = is_presolve(lp, PRESOLVE_IMPLIEDFREE);
    let mut tightenbounds: ::core::ffi::c_uchar = is_presolve(lp, PRESOLVE_BOUNDS);
    let mut iRangeTighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iBoundTighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut losum: ::core::ffi::c_double = 0.;
    let mut upsum: ::core::ffi::c_double = 0.;
    let mut lorhs: ::core::ffi::c_double = 0.;
    let mut uprhs: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut mat: *mut MATrec = (*lp).matA;
    i = lastActiveLink((*(*psdata).rows).varmap);
    while i > 0 as ::core::ffi::c_int {
        j = presolve_rowlengthex(psdata, i);
        if j > 1 as ::core::ffi::c_int
            && (*psdata).forceupdate == 0
            && presolve_rowfeasible(psdata, i, FALSE as ::core::ffi::c_uchar) == 0
        {
            status = presolve_setstatusex(
                psdata,
                2 as ::core::ffi::c_int,
                5174 as ::core::ffi::c_int,
                b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            break;
        } else {
            if impliedfree as ::core::ffi::c_int != 0
                && j > 1 as ::core::ffi::c_int
                && mat_validate(mat) as ::core::ffi::c_int != 0
            {
                presolve_range(lp, i, (*psdata).rows, &raw mut losum, &raw mut upsum);
                lorhs = get_rh_lower(lp, i);
                uprhs = get_rh_upper(lp, i);
                if losum > (if upsum < uprhs { upsum } else { uprhs }) + epsvalue
                    || upsum < (if losum > lorhs { losum } else { lorhs }) - epsvalue
                {
                    report(
                        lp,
                        4 as ::core::ffi::c_int,
                        b"presolve_preparerows: Variable bound / constraint value infeasibility in row %s.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    status = presolve_setstatusex(
                        psdata,
                        2 as ::core::ffi::c_int,
                        5190 as ::core::ffi::c_int,
                        b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    break;
                } else {
                    if losum > lorhs + epsvalue {
                        set_rh_lower(
                            lp,
                            i,
                            presolve_roundrhs(lp, losum, TRUE as ::core::ffi::c_uchar),
                        );
                        iRangeTighten += 1;
                    }
                    if upsum < uprhs - epsvalue {
                        set_rh_upper(
                            lp,
                            i,
                            presolve_roundrhs(lp, upsum, FALSE as ::core::ffi::c_uchar),
                        );
                        iRangeTighten += 1;
                    }
                }
            }
            if tightenbounds as ::core::ffi::c_int != 0
                && mat_validate(mat) as ::core::ffi::c_int != 0
            {
                if j > 1 as ::core::ffi::c_int {
                    status = presolve_rowtighten(
                        psdata,
                        i,
                        &raw mut iBoundTighten,
                        FALSE as ::core::ffi::c_uchar,
                    );
                }
            }
            if is_constr_type(lp, i, EQ) == 0 && get_rh_range(lp, i) < epsvalue {
                presolve_setEQ(psdata, i);
                iRangeTighten += 1;
            }
            i = prevActiveLink((*(*psdata).rows).varmap, i);
        }
    }
    (*psdata).forceupdate = ((*psdata).forceupdate as ::core::ffi::c_int
        | (iBoundTighten > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar
            as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    *nBoundTighten += iBoundTighten + iRangeTighten;
    *nSum += iBoundTighten + iRangeTighten;
    return status;
}
#[export_name="honest_lpsolve_presolve_rows"]
pub unsafe extern "C" fn presolve_rows(
    mut psdata: *mut presolverec,
    mut nCoeffChanged: *mut ::core::ffi::c_int,
    mut nConRemove: *mut ::core::ffi::c_int,
    mut nVarFixed: *mut ::core::ffi::c_int,
    mut nBoundTighten: *mut ::core::ffi::c_int,
    mut nSum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut lprec = (*psdata).lp;
    let mut candelete: ::core::ffi::c_uchar = 0;
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iConRemove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iBoundTighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jx: ::core::ffi::c_int = 0;
    let mut item: ::core::ffi::c_int = 0;
    let mut Value1: ::core::ffi::c_double = 0.;
    let mut Value2: ::core::ffi::c_double = 0.;
    let mut losum: ::core::ffi::c_double = 0.;
    let mut upsum: ::core::ffi::c_double = 0.;
    let mut lorhs: ::core::ffi::c_double = 0.;
    let mut uprhs: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*psdata).epsvalue;
    let mut mat: *mut MATrec = (*lp).matA;
    i = lastActiveLink((*(*psdata).rows).varmap);
    while i > 0 as ::core::ffi::c_int && status == RUNNING {
        candelete = FALSE as ::core::ffi::c_uchar;
        j = presolve_rowlengthex(psdata, i);
        if j > 1 as ::core::ffi::c_int
            && (*psdata).forceupdate == 0
            && presolve_rowfeasible(psdata, i, FALSE as ::core::ffi::c_uchar) == 0
        {
            status = presolve_setstatusex(
                psdata,
                2 as ::core::ffi::c_int,
                5248 as ::core::ffi::c_int,
                b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            break;
        } else {
            presolve_range(lp, i, (*psdata).rows, &raw mut losum, &raw mut upsum);
            lorhs = get_rh_lower(lp, i);
            uprhs = get_rh_upper(lp, i);
            if j == 0 as ::core::ffi::c_int {
                candelete = TRUE as ::core::ffi::c_uchar;
            } else if j == 1 as ::core::ffi::c_int && uprhs - lorhs >= -epsvalue {
                item = 0 as ::core::ffi::c_int;
                jx = presolve_nextcol(psdata, i, &raw mut item);
                j = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(jx as isize) as isize);
                Value1 = (*lp).infinite;
                Value2 = -Value1;
                if presolve_collength(psdata, j) > 1 as ::core::ffi::c_int {
                    status = presolve_boundconflict(psdata, i, j);
                } else if is_constr_type(lp, i, EQ) != 0 {
                    Value2 = *(*mat)
                        .col_mat_value
                        .offset(*(*mat).row_mat.offset(jx as isize) as isize);
                    Value1 = *(*lp).orig_rhs.offset(i as isize) / Value2;
                    if Value2 < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        swapREAL(&raw mut losum, &raw mut upsum);
                    }
                    if Value1
                        < losum
                            / (if (fabs(losum) >= (*lp).infinite) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                != 0
                            {
                                (if Value2 < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    -(1 as ::core::ffi::c_int)
                                } else {
                                    1 as ::core::ffi::c_int
                                }) as ::core::ffi::c_double
                            } else {
                                Value2
                            })
                            - epsvalue
                        || Value1
                            > upsum
                                / (if (fabs(upsum) >= (*lp).infinite) as ::core::ffi::c_int
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    != 0
                                {
                                    (if Value2 < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        -(1 as ::core::ffi::c_int)
                                    } else {
                                        1 as ::core::ffi::c_int
                                    }) as ::core::ffi::c_double
                                } else {
                                    Value2
                                })
                                + epsvalue
                    {
                        status = presolve_setstatusex(
                            psdata,
                            2 as ::core::ffi::c_int,
                            5288 as ::core::ffi::c_int,
                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    Value2 = Value1;
                }
                if status == RUNNING {
                    if fabs(Value2 - Value1) < epsvalue && fabs(Value2) > epsvalue {
                        let mut isSOS: ::core::ffi::c_uchar =
                            (SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, j) != FALSE)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uchar;
                        let mut deleteSOS: ::core::ffi::c_uchar = (isSOS as ::core::ffi::c_int != 0
                            && presolve_candeletevar(psdata, j) as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_uchar;
                        if Value1 != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            && deleteSOS as ::core::ffi::c_int != 0
                        {
                            if presolve_fixSOS1(
                                psdata,
                                j,
                                Value1,
                                &raw mut iConRemove,
                                &raw mut iVarFixed,
                            ) == 0
                            {
                                status = presolve_setstatusex(
                                    psdata,
                                    2 as ::core::ffi::c_int,
                                    5299 as ::core::ffi::c_int,
                                    b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                            (*psdata).forceupdate = TRUE as ::core::ffi::c_uchar;
                        } else if presolve_colfix(
                            psdata,
                            j,
                            Value1,
                            (isSOS == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        ) == 0
                        {
                            status = presolve_setstatusex(
                                psdata,
                                2 as ::core::ffi::c_int,
                                5305 as ::core::ffi::c_int,
                                b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                        } else if isSOS as ::core::ffi::c_int != 0 && deleteSOS == 0 {
                            iBoundTighten += 1;
                        } else {
                            presolve_colremove(psdata, j, TRUE as ::core::ffi::c_uchar);
                            iVarFixed += 1;
                        }
                    } else {
                        status = presolve_colsingleton(psdata, i, j, &raw mut iBoundTighten);
                    }
                }
                if status == INFEASIBLE {
                    break;
                }
                if (*psdata).forceupdate as ::core::ffi::c_int != AUTOMATIC {
                    presolve_storeDualUndo(psdata, i, j);
                    candelete = TRUE as ::core::ffi::c_uchar;
                }
            } else if j > 0 as ::core::ffi::c_int
                && fabs(*(*lp).orig_rhs.offset(i as isize)) < epsvalue
                && (*(*(*psdata).rows).plucount.offset(i as isize) == 0 as ::core::ffi::c_int
                    || *(*(*psdata).rows).negcount.offset(i as isize) == 0 as ::core::ffi::c_int)
                && *(*(*psdata).rows).pluneg.offset(i as isize) == 0 as ::core::ffi::c_int
                && (is_constr_type(lp, i, EQ) as ::core::ffi::c_int != 0
                    || fabs(lorhs - upsum) < epsvalue
                    || fabs(uprhs - losum) < epsvalue)
            {
                status = presolve_rowfixzero(psdata, i, &raw mut iVarFixed);
                if status == RUNNING {
                    candelete = TRUE as ::core::ffi::c_uchar;
                }
            } else if losum >= lorhs - epsvalue && upsum <= uprhs + epsvalue {
                if fabs(losum - upsum) < epsvalue {
                    item = 0 as ::core::ffi::c_int;
                    jx = presolve_nextcol(psdata, i, &raw mut item);
                    while status == RUNNING && jx >= 0 as ::core::ffi::c_int {
                        j = *(*mat)
                            .col_mat_colnr
                            .offset(*(*mat).row_mat.offset(jx as isize) as isize);
                        Value1 = get_lowbo(lp, j);
                        if presolve_colfix(
                            psdata,
                            j,
                            Value1,
                            TRUE as ::core::ffi::c_uchar,
                            &raw mut iVarFixed,
                        ) != 0
                        {
                            presolve_colremove(psdata, j, TRUE as ::core::ffi::c_uchar);
                            iVarFixed += 1;
                            jx = presolve_nextcol(psdata, i, &raw mut item);
                        } else {
                            status = presolve_setstatusex(
                                psdata,
                                2 as ::core::ffi::c_int,
                                5368 as ::core::ffi::c_int,
                                b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                }
                candelete = TRUE as ::core::ffi::c_uchar;
            }
            ix = i;
            i = prevActiveLink((*(*psdata).rows).varmap, i);
            if candelete != 0 {
                presolve_rowremove(psdata, ix, TRUE as ::core::ffi::c_uchar);
                iConRemove += 1;
            }
        }
    }
    if status == RUNNING {
        status = presolve_shrink(psdata, &raw mut iConRemove, &raw mut iVarFixed);
    }
    *nCoeffChanged += iCoeffChanged;
    *nConRemove += iConRemove;
    *nVarFixed += iVarFixed;
    *nBoundTighten += iBoundTighten;
    *nSum += iCoeffChanged + iConRemove + iVarFixed + iBoundTighten;
    return status;
}
#[export_name="honest_lpsolve_presolve"]
pub unsafe extern "C" fn presolve(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut status: ::core::ffi::c_int = RUNNING;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jjx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut oSum: ::core::ffi::c_int = 0;
    let mut iCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iConRemove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iBoundTighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iSOS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iSum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nCoeffChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nConRemove: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nVarFixed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nBoundTighten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nSOS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nSum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Value1: ::core::ffi::c_double = 0.;
    let mut Value2: ::core::ffi::c_double = 0.;
    let mut initrhs0: ::core::ffi::c_double =
        *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize);
    let mut psdata: *mut presolverec = ::core::ptr::null_mut::<presolverec>();
    let mut mat: *mut MATrec = (*lp).matA;
    if (*lp).varmap_locked == 0 {
        varmap_lock(lp);
    }
    mat_validate(mat);
    if (*lp).wasPresolved != 0 {
        if SOS_count(lp) > 0 as ::core::ffi::c_int {
            SOS_member_updatemap((*lp).SOS);
            make_SOSchain(
                lp,
                ((*lp).do_presolve & PRESOLVE_LASTMASKMODE != PRESOLVE_NONE) as ::core::ffi::c_int
                    as ::core::ffi::c_uchar,
            );
        }
        if (*lp).solvecount > 1 as ::core::ffi::c_int
            && (*lp).bb_level < 1 as ::core::ffi::c_int
            && (*lp).scalemode & SCALE_DYNUPDATE != 0 as ::core::ffi::c_int
        {
            auto_scale(lp);
        }
        if (*lp).basis_valid == 0 {
            crash_basis(lp);
            report(
                lp,
                5 as ::core::ffi::c_int,
                b"presolve: Had to repair broken basis.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        (*lp).timepresolved = timeNow();
        return status;
    }
    i = SOS_count(lp);
    if i > 0 as ::core::ffi::c_int {
        SOS_member_updatemap((*lp).SOS);
        (*lp).sos_vars = SOS_memberships((*lp).SOS, 0 as ::core::ffi::c_int);
    }
    REPORT_modelinfo(
        lp,
        TRUE as ::core::ffi::c_uchar,
        b"SUBMITTED\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if i > 0 as ::core::ffi::c_int {
        (*lp).sos_vars = 0 as ::core::ffi::c_int;
    }
    if (*lp).basis_valid == 0 {
        *(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) = AUTOMATIC;
    }
    mat_computemax(mat);
    yieldformessages(lp);
    if (*lp).do_presolve & PRESOLVE_LASTMASKMODE == PRESOLVE_NONE {
        mat_checkcounts(
            mat,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            TRUE as ::core::ffi::c_uchar,
        );
        i = 0 as ::core::ffi::c_int;
        current_block = 1228639923084383292;
    } else {
        if (*lp).full_solution.is_null() {
            allocREAL(
                lp,
                &raw mut (*lp).full_solution,
                (*lp).sum_alloc + 1 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            );
        }
        j = 0 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i <= SOS_count(lp) {
            k = SOS_infeasible((*lp).SOS, i);
            if k > 0 as ::core::ffi::c_int {
                let mut psdata_0: presolverec = presolverec {
                    rows: ::core::ptr::null_mut::<psrec>(),
                    cols: ::core::ptr::null_mut::<psrec>(),
                    EQmap: ::core::ptr::null_mut::<LLrec>(),
                    LTmap: ::core::ptr::null_mut::<LLrec>(),
                    INTmap: ::core::ptr::null_mut::<LLrec>(),
                    pv_upbo: ::core::ptr::null_mut::<::core::ffi::c_double>(),
                    pv_lobo: ::core::ptr::null_mut::<::core::ffi::c_double>(),
                    dv_upbo: ::core::ptr::null_mut::<::core::ffi::c_double>(),
                    dv_lobo: ::core::ptr::null_mut::<::core::ffi::c_double>(),
                    lp: ::core::ptr::null_mut::<lprec>(),
                    epsvalue: 0.,
                    epspivot: 0.,
                    innerloops: 0,
                    middleloops: 0,
                    outerloops: 0,
                    nzdeleted: 0,
                    forceupdate: 0,
                };
                psdata_0.lp = lp;
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"presolve: Found SOS %d (type %d) to be range-infeasible on variable %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                status = presolve_setstatusex(
                    &raw mut psdata_0,
                    2 as ::core::ffi::c_int,
                    5503 as ::core::ffi::c_int,
                    b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                j += 1;
            }
            i += 1;
        }
        if j > 0 as ::core::ffi::c_int {
            current_block = 15758502876150501180;
        } else {
            psdata = presolve_init(lp);
            (*psdata).outerloops = 0 as ::core::ffi::c_int;
            loop {
                (*psdata).outerloops += 1;
                iCoeffChanged = 0 as ::core::ffi::c_int;
                iConRemove = 0 as ::core::ffi::c_int;
                iVarFixed = 0 as ::core::ffi::c_int;
                iBoundTighten = 0 as ::core::ffi::c_int;
                iSOS = 0 as ::core::ffi::c_int;
                oSum = nSum;
                loop {
                    (*psdata).middleloops += 1;
                    nSum += iSum;
                    iSum = 0 as ::core::ffi::c_int;
                    j = 0 as ::core::ffi::c_int;
                    while presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                        && (*psdata).forceupdate as ::core::ffi::c_int != 0
                    {
                        (*psdata).forceupdate = FALSE as ::core::ffi::c_uchar;
                        if presolve_updatesums(psdata) as ::core::ffi::c_int != 0
                            && j < MAX_PSBOUNDTIGHTENLOOPS
                        {
                            if (*psdata).outerloops == 1 as ::core::ffi::c_int
                                && (*psdata).middleloops == 1 as ::core::ffi::c_int
                            {
                                status = presolve_preparerows(
                                    psdata,
                                    &raw mut iBoundTighten,
                                    &raw mut iSum,
                                );
                            }
                            nBoundTighten += iBoundTighten;
                            iBoundTighten = 0 as ::core::ffi::c_int;
                            nSum += iSum;
                            iSum = 0 as ::core::ffi::c_int;
                            j += 1;
                            if status != RUNNING {
                                report(
                                    lp,
                                    4 as ::core::ffi::c_int,
                                    b"presolve: Break after bound tightening iteration %d.\n\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                        }
                    }
                    if status != RUNNING {
                        break;
                    }
                    loop {
                        (*psdata).innerloops += 1;
                        nSum += iSum;
                        iSum = 0 as ::core::ffi::c_int;
                        if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                            && is_presolve(lp, PRESOLVE_ROWS) as ::core::ffi::c_int != 0
                        {
                            status = presolve_rows(
                                psdata,
                                &raw mut iCoeffChanged,
                                &raw mut iConRemove,
                                &raw mut iVarFixed,
                                &raw mut iBoundTighten,
                                &raw mut iSum,
                            );
                        }
                        if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                            && is_presolve(lp, PRESOLVE_COLS) as ::core::ffi::c_int != 0
                        {
                            status = presolve_columns(
                                psdata,
                                &raw mut iCoeffChanged,
                                &raw mut iConRemove,
                                &raw mut iVarFixed,
                                &raw mut iBoundTighten,
                                &raw mut iSum,
                            );
                        }
                        if presolve_statuscheck(psdata, &raw mut status) != 0 {
                            status = presolve_redundantSOS(
                                psdata,
                                &raw mut iBoundTighten,
                                &raw mut iSum,
                            );
                        }
                        if !(status == RUNNING && iSum > 0 as ::core::ffi::c_int) {
                            break;
                        }
                    }
                    if status != RUNNING {
                        break;
                    }
                    if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                        && (*psdata).outerloops == 1 as ::core::ffi::c_int
                        && (*psdata).middleloops <= MAX_PSMERGELOOPS
                        && is_presolve(lp, PRESOLVE_MERGEROWS) as ::core::ffi::c_int != 0
                    {
                        status = presolve_mergerows(psdata, &raw mut iConRemove, &raw mut iSum);
                    }
                    if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                        && is_presolve(lp, PRESOLVE_ROWDOMINATE) as ::core::ffi::c_int != 0
                    {
                        presolve_rowdominance(
                            psdata,
                            &raw mut iCoeffChanged,
                            &raw mut iConRemove,
                            &raw mut iVarFixed,
                            &raw mut iSum,
                        );
                    }
                    if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                        && MIP_count(lp) > 0 as ::core::ffi::c_int
                        && is_presolve(lp, PRESOLVE_SOS) as ::core::ffi::c_int != 0
                    {
                        status = presolve_SOS1(
                            psdata,
                            &raw mut iCoeffChanged,
                            &raw mut iConRemove,
                            &raw mut iVarFixed,
                            &raw mut iSOS,
                            &raw mut iSum,
                        );
                    }
                    if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                        && (*lp).int_vars > 1 as ::core::ffi::c_int
                        && is_presolve(lp, PRESOLVE_COLDOMINATE) as ::core::ffi::c_int != 0
                    {
                        presolve_coldominance01(
                            psdata,
                            &raw mut iConRemove,
                            &raw mut iVarFixed,
                            &raw mut iSum,
                        );
                    }
                    if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                        && is_presolve(lp, PRESOLVE_AGGREGATE) as ::core::ffi::c_int != 0
                    {
                        presolve_aggregate(
                            psdata,
                            &raw mut iConRemove,
                            &raw mut iVarFixed,
                            &raw mut iSum,
                        );
                    }
                    if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                        && is_presolve(lp, PRESOLVE_IMPLIEDSLK | PRESOLVE_IMPLIEDFREE)
                            as ::core::ffi::c_int
                            != 0
                    {
                        status = presolve_freeandslacks(
                            psdata,
                            &raw mut iCoeffChanged,
                            &raw mut iConRemove,
                            &raw mut iVarFixed,
                            &raw mut iSum,
                        );
                    }
                    if !(status == RUNNING && iSum > 0 as ::core::ffi::c_int) {
                        break;
                    }
                }
                if status != RUNNING {
                    break;
                }
                if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                    && (*(*psdata).EQmap).count > 1 as ::core::ffi::c_int
                    && is_presolve(lp, PRESOLVE_LINDEP) as ::core::ffi::c_int != 0
                {
                    presolve_singularities(
                        psdata,
                        &raw mut iCoeffChanged,
                        &raw mut iConRemove,
                        &raw mut iVarFixed,
                        &raw mut iSum,
                    );
                }
                if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                    && is_presolve(lp, PRESOLVE_ELIMEQ2) as ::core::ffi::c_int != 0
                {
                    jjx = 0 as ::core::ffi::c_int;
                    loop {
                        jjx += iSum;
                        status = presolve_elimeq2(
                            psdata,
                            &raw mut iCoeffChanged,
                            &raw mut iConRemove,
                            &raw mut iVarFixed,
                            &raw mut iSum,
                        );
                        if !(status == RUNNING && iSum > jjx) {
                            break;
                        }
                    }
                    iSum = jjx;
                }
                if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                    && (*(*psdata).EQmap).count > 0 as ::core::ffi::c_int
                    && is_presolve(lp, PRESOLVE_SPARSER) as ::core::ffi::c_int != 0
                {
                    status = presolve_makesparser(
                        psdata,
                        &raw mut iCoeffChanged,
                        &raw mut iConRemove,
                        &raw mut iVarFixed,
                        &raw mut iSum,
                    );
                }
                if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                    && (*(*psdata).INTmap).count > 0 as ::core::ffi::c_int
                    && is_presolve(lp, PRESOLVE_REDUCEGCD) as ::core::ffi::c_int != 0
                {
                    if presolve_reduceGCD(
                        psdata,
                        &raw mut iCoeffChanged,
                        &raw mut iBoundTighten,
                        &raw mut iSum,
                    ) == 0
                    {
                        status = presolve_setstatusex(
                            psdata,
                            2 as ::core::ffi::c_int,
                            5649 as ::core::ffi::c_int,
                            b"/private/var/folders/lg/d5j598mx5q39sbdp9cdxzyy00000gn/T/honest-lpsolve-LrANs4/src/lp_solve/lp_presolve.c\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
                if presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                    && is_presolve(lp, PRESOLVE_KNAPSACK) as ::core::ffi::c_int != 0
                {
                    i = iCoeffChanged;
                    status = presolve_knapsack(psdata, &raw mut iCoeffChanged);
                }
                if status == RUNNING {
                    status = presolve_shrink(psdata, &raw mut iConRemove, &raw mut iVarFixed);
                }
                nCoeffChanged += iCoeffChanged;
                nConRemove += iConRemove;
                nVarFixed += iVarFixed;
                nBoundTighten += iBoundTighten;
                nSOS += iSOS;
                nSum += iSum;
                iSum = iConRemove + iVarFixed + iBoundTighten + iCoeffChanged;
                if iSum > 0 as ::core::ffi::c_int {
                    report(
                        lp,
                        4 as ::core::ffi::c_int,
                        b"Presolve O:%d -> Reduced rows:%5d, cols:%5d --- changed bnds:%5d, Ab:%5d.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                if !(presolve_statuscheck(psdata, &raw mut status) as ::core::ffi::c_int != 0
                    && ((*psdata).forceupdate as ::core::ffi::c_int != 0 || oSum < nSum)
                    && (*psdata).outerloops < get_presolveloops(lp)
                    && (*(*(*psdata).rows).varmap).count + (*(*(*psdata).cols).varmap).count
                        > 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if status == RUNNING && is_presolve(lp, PRESOLVE_IMPLIEDFREE) == 0 {
                jjx = presolve_makefree(psdata);
            } else {
                jjx = 0 as ::core::ffi::c_int;
            }
            if presolve_finalize(psdata) == 0 {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"presolve: Unable to construct internal data representation\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            i = NORMAL;
            iVarFixed = (*(*lp).presolve_undo).orig_columns - (*(*(*psdata).cols).varmap).count;
            iConRemove = (*(*lp).presolve_undo).orig_rows - (*(*(*psdata).rows).varmap).count;
            if nSum > 0 as ::core::ffi::c_int {
                report(
                    lp,
                    i,
                    b"PRESOLVE             Elimination loops performed.......... O%d:M%d:I%d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if nVarFixed != 0 {
                report(
                    lp,
                    i,
                    b"            %8d empty or fixed variables............. %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if nConRemove != 0 {
                report(
                    lp,
                    i,
                    b"            %8d empty or redundant constraints....... %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if nBoundTighten != 0 {
                report(
                    lp,
                    i,
                    b"            %8d bounds............................... %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if nCoeffChanged != 0 {
                report(
                    lp,
                    i,
                    b"            %8d matrix coefficients.................. %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if jjx > 0 as ::core::ffi::c_int {
                report(
                    lp,
                    i,
                    b"            %8d variables' final bounds.............. %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if nSOS != 0 {
                report(
                    lp,
                    i,
                    b"            %8d constraints detected as SOS1......... %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if status == UNBOUNDED {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"%20s Solution status detected............. %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else if status == INFEASIBLE {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"%20s Solution status detected............. %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                if (*(*(*psdata).cols).varmap).count == 0 as ::core::ffi::c_int {
                    Value2 = *(*(*lp).presolve_undo)
                        .fixed_rhs
                        .offset(0 as ::core::ffi::c_int as isize)
                        - initrhs0;
                    Value1 = Value2;
                } else {
                    presolve_rangeorig(
                        lp,
                        0 as ::core::ffi::c_int,
                        (*psdata).rows,
                        &raw mut Value1,
                        &raw mut Value2,
                        -initrhs0,
                    );
                }
                if fabs(Value1 - Value2) < (*psdata).epsvalue
                    || fabs((Value1 - Value2) / (1.0f64 + fabs(Value2))) < (*psdata).epsvalue
                {
                    if (*lp).rows == 0 as ::core::ffi::c_int
                        && (*lp).columns == 0 as ::core::ffi::c_int
                    {
                        status = PRESOLVED;
                        Value1 = if is_maxim(lp) as ::core::ffi::c_int != 0
                            && Value1 != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -Value1
                        } else {
                            Value1
                        };
                        *(*lp).solution.offset(0 as ::core::ffi::c_int as isize) = Value1;
                        *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize) = Value1;
                        *(*lp).full_solution.offset(0 as ::core::ffi::c_int as isize) = Value1;
                    }
                    report(
                        lp,
                        4 as ::core::ffi::c_int,
                        b"%20s OPTIMAL solution found............... %-g\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                } else if status == RUNNING && i >= NORMAL {
                    let mut lonum: [::core::ffi::c_char; 20] = [0; 20];
                    let mut upnum: [::core::ffi::c_char; 20] = [0; 20];
                    if (fabs(Value1) >= (*lp).infinite) as ::core::ffi::c_int
                        as ::core::ffi::c_uchar
                        != 0
                    {
                        native_only!(snprintf,
                            &raw mut lonum as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
                            b"%13s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"-Inf\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        native_only!(snprintf,
                            &raw mut lonum as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
                            b"%+12g\0" as *const u8 as *const ::core::ffi::c_char,
                            Value1,
                        );
                    }
                    if (fabs(Value2) >= (*lp).infinite) as ::core::ffi::c_int
                        as ::core::ffi::c_uchar
                        != 0
                    {
                        native_only!(snprintf,
                            &raw mut upnum as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
                            b"%-13s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"Inf\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        native_only!(snprintf,
                            &raw mut upnum as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
                            b"%+-12g\0" as *const u8 as *const ::core::ffi::c_char,
                            Value2,
                        );
                    }
                    report(
                        lp,
                        i,
                        b"%20s [ %s < Z < %s ]\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                if MIP_count(lp) > 0 as ::core::ffi::c_int
                    || get_Lrows(lp) > 0 as ::core::ffi::c_int
                {
                    if is_maxim(lp) != 0 {
                        if (*lp).bb_heuristicOF < Value1 {
                            (*lp).bb_heuristicOF = Value1;
                        }
                        if (*lp).bb_limitOF > Value2 {
                            (*lp).bb_limitOF = Value2;
                        }
                    } else {
                        if (*lp).bb_heuristicOF > Value2 {
                            (*lp).bb_heuristicOF = Value2;
                        }
                        if (*lp).bb_limitOF < Value1 {
                            (*lp).bb_limitOF = Value1;
                        }
                    }
                }
            }
            report(
                lp,
                4 as ::core::ffi::c_int,
                b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            j = (*(*psdata).LTmap).count;
            jx = (*(*psdata).EQmap).count;
            jjx = (*lp).rows - j - jx;
            presolve_free(&raw mut psdata);
            current_block = 1228639923084383292;
        }
    }
    match current_block {
        1228639923084383292 => {
            if (*lp).usermessage.is_some()
                && (*lp).do_presolve & PRESOLVE_LASTMASKMODE != 0 as ::core::ffi::c_int
                && (*lp).msgmask & MSG_PRESOLVE != 0
            {
                (*lp).usermessage.expect("non-null function pointer")(
                    lp,
                    (*lp).msghandle,
                    MSG_PRESOLVE,
                );
            }
            if SOS_count(lp) > 0 as ::core::ffi::c_int {
                make_SOSchain(
                    lp,
                    ((*lp).do_presolve & PRESOLVE_LASTMASKMODE != PRESOLVE_NONE)
                        as ::core::ffi::c_int as ::core::ffi::c_uchar,
                );
            }
            if status == RUNNING {
                if is_bb_mode(lp, NODE_GUBMODE) != 0 {
                    identify_GUB(lp, TRUE as ::core::ffi::c_uchar);
                }
                auto_scale(lp);
                crash_basis(lp);
                if nConRemove + nVarFixed + nBoundTighten + nVarFixed + nCoeffChanged
                    > 0 as ::core::ffi::c_int
                {
                    REPORT_modelinfo(
                        lp,
                        FALSE as ::core::ffi::c_uchar,
                        b"REDUCED\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    if nSum > 0 as ::core::ffi::c_int {
                        report(
                            lp,
                            4 as ::core::ffi::c_int,
                            b"Row-types:   %7d LE,          %7d GE,             %7d EQ.\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        report(
                            lp,
                            4 as ::core::ffi::c_int,
                            b" \n\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
            }
            if (*lp).verbose > NORMAL {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
                REPORT_constraintinfo(
                    lp,
                    b"CONSTRAINT CLASSES\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    (*lp).wasPresolved = TRUE as ::core::ffi::c_uchar;
    (*lp).timepresolved = timeNow();
    return status;
}
#[export_name="honest_lpsolve_postsolve"]
pub unsafe extern "C" fn postsolve(
    mut lp: *mut lprec,
    mut status: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if (*lp).lag_status != RUNNING {
        let mut itemp: ::core::ffi::c_int = 0;
        if status == PRESOLVED {
            status = OPTIMAL;
        }
        if status == OPTIMAL || status == SUBOPTIMAL {
            itemp = check_solution(
                lp,
                (*lp).columns,
                (*lp).best_solution,
                (*lp).orig_upbo,
                (*lp).orig_lowbo,
                (*lp).epssolution,
            );
            if itemp != OPTIMAL && (*lp).spx_status == OPTIMAL {
                (*lp).spx_status = itemp;
            } else if itemp == OPTIMAL && (status == SUBOPTIMAL || (*lp).spx_status == PRESOLVED) {
                (*lp).spx_status = status;
            }
        } else if status != PRESOLVED {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"lp_solve unsuccessful after %.0f iter and a last best value of %g\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            if (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"lp_solve explored %.0f nodes before termination\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        } else {
            (*lp).spx_status = OPTIMAL;
        }
        presolve_rebuildUndo(lp, TRUE as ::core::ffi::c_uchar);
    }
    if varmap_canunlock(lp) != 0 {
        (*lp).varmap_locked = FALSE as ::core::ffi::c_uchar;
    }
    return 1 as ::core::ffi::c_uchar;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAXINT32: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PRESOLVE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PRESOLVE_ROWS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PRESOLVE_COLS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PRESOLVE_LINDEP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PRESOLVE_AGGREGATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PRESOLVE_SPARSER: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PRESOLVE_SOS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PRESOLVE_KNAPSACK: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PRESOLVE_ELIMEQ2: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const PRESOLVE_IMPLIEDFREE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const PRESOLVE_REDUCEGCD: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const PRESOLVE_PROBEFIX: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const PRESOLVE_ROWDOMINATE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const PRESOLVE_COLDOMINATE: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const PRESOLVE_MERGEROWS: ::core::ffi::c_int = 32768 as ::core::ffi::c_int;
pub const PRESOLVE_IMPLIEDSLK: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const PRESOLVE_COLFIXDUAL: ::core::ffi::c_int = 131072 as ::core::ffi::c_int;
pub const PRESOLVE_BOUNDS: ::core::ffi::c_int = 262144 as ::core::ffi::c_int;
pub const PRESOLVE_LASTMASKMODE: ::core::ffi::c_int = PRESOLVE_DUALS - 1 as ::core::ffi::c_int;
pub const PRESOLVE_DUALS: ::core::ffi::c_int = 524288 as ::core::ffi::c_int;
pub const NORMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DETAILED: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MSG_PRESOLVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ROWTYPE_LE: ::core::ffi::c_int = 1;
pub const ROWTYPE_GE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ROWTYPE_EQ: ::core::ffi::c_int = 3;
pub const LE: ::core::ffi::c_int = ROWTYPE_LE;
pub const GE: ::core::ffi::c_int = ROWTYPE_GE;
pub const EQ: ::core::ffi::c_int = ROWTYPE_EQ;
pub const SCALE_DYNUPDATE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NODE_GUBMODE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const OPTIMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SUBOPTIMAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INFEASIBLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UNBOUNDED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const RUNNING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PRESOLVED: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MATRIXERROR: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const MAT_START_SIZE: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const RESIZEFACTOR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_FRACSCALE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const matRowColStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const matValueStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOS1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOSn: ::core::ffi::c_int = MAXINT32;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
