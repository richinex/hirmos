use crate::honest_did::lpsolve::runtime::{strlen,strcpy,strcmp,strncmp,strcat,strrchr,modf};
use crate::honest_did::lpsolve::runtime::{malloc,calloc,free,realloc,fabs};
#[repr(C)] pub struct __sFILEX{_opaque:[u8;0]}
extern "C" {
    #[link_name="honest_lpsolve_MPS_readfile"]
    fn MPS_readfile(
        newlp: *mut *mut lprec,
        filename: *mut ::core::ffi::c_char,
        typeMPS: ::core::ffi::c_int,
        verbose: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_MPS_readhandle"]
    fn MPS_readhandle(
        newlp: *mut *mut lprec,
        filehandle: *mut FILE,
        typeMPS: ::core::ffi::c_int,
        verbose: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_MPS_writefile"]
    fn MPS_writefile(
        lp: *mut lprec,
        typeMPS: ::core::ffi::c_int,
        filename: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_MPS_writehandle"]
    fn MPS_writehandle(
        lp: *mut lprec,
        typeMPS: ::core::ffi::c_int,
        output: *mut FILE,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_MPS_readBAS"]
    fn MPS_readBAS(
        lp: *mut lprec,
        typeMPS: ::core::ffi::c_int,
        filename: *mut ::core::ffi::c_char,
        info: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_LP_writefile"]
    fn LP_writefile(lp: *mut lprec, filename: *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_LP_writehandle"]
    fn LP_writehandle(lp: *mut lprec, output: *mut FILE) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_presolve_createUndo"]
    fn presolve_createUndo(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_presolve_rebuildUndo"]
    fn presolve_rebuildUndo(lp: *mut lprec, isprimal: ::core::ffi::c_uchar)
        -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_inc_presolve_space"]
    fn inc_presolve_space(
        lp: *mut lprec,
        delta: ::core::ffi::c_int,
        isrows: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_presolve_setOrig"]
    fn presolve_setOrig(
        lp: *mut lprec,
        orig_rows: ::core::ffi::c_int,
        orig_cols: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_presolve_fillUndo"]
    fn presolve_fillUndo(
        lp: *mut lprec,
        orig_rows: ::core::ffi::c_int,
        orig_cols: ::core::ffi::c_int,
        setOrig: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_presolve_freeUndo"]
    fn presolve_freeUndo(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_unscale_columns"]
    fn unscale_columns(lp: *mut lprec);
    #[link_name="honest_lpsolve_scaled_mat"]
    fn scaled_mat(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_unscaled_mat"]
    fn unscaled_mat(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_scaled_value"]
    fn scaled_value(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_unscaled_value"]
    fn unscaled_value(
        lp: *mut lprec,
        value: ::core::ffi::c_double,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_undoscale"]
    fn undoscale(lp: *mut lprec);
    #[link_name="honest_lpsolve_lin_solve"]
    fn lin_solve(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_getMDO"]
    fn getMDO(
        lp: *mut lprec,
        usedpos: *mut ::core::ffi::c_uchar,
        colorder: *mut ::core::ffi::c_int,
        size: *mut ::core::ffi::c_int,
        symmetric: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_name"]
    fn bfp_name() -> *mut ::core::ffi::c_char;
    #[link_name="honest_lpsolve_bfp_free"]
    fn bfp_free(lp: *mut lprec);
    #[link_name="honest_lpsolve_bfp_resize"]
    fn bfp_resize(lp: *mut lprec, newsize: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_nonzeros"]
    fn bfp_nonzeros(lp: *mut lprec, maximum: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_memallocated"]
    fn bfp_memallocated(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_preparefactorization"]
    fn bfp_preparefactorization(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_factorize"]
    fn bfp_factorize(
        lp: *mut lprec,
        uservars: ::core::ffi::c_int,
        Bsize: ::core::ffi::c_int,
        usedpos: *mut ::core::ffi::c_uchar,
        final_0: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_finishupdate"]
    fn bfp_finishupdate(lp: *mut lprec, changesign: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_ftran_normal"]
    fn bfp_ftran_normal(
        lp: *mut lprec,
        pcol: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_bfp_ftran_prepare"]
    fn bfp_ftran_prepare(
        lp: *mut lprec,
        pcol: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_bfp_btran_normal"]
    fn bfp_btran_normal(
        lp: *mut lprec,
        prow: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_bfp_status"]
    fn bfp_status(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_findredundant"]
    fn bfp_findredundant(
        lp: *mut lprec,
        items: ::core::ffi::c_int,
        cb: Option<getcolumnex_func>,
        maprow: *mut ::core::ffi::c_int,
        mapcol: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_compatible"]
    fn bfp_compatible(
        lp: *mut lprec,
        bfpversion: ::core::ffi::c_int,
        lpversion: ::core::ffi::c_int,
        sizeofvar: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_indexbase"]
    fn bfp_indexbase(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_rowoffset"]
    fn bfp_rowoffset(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_pivotmax"]
    fn bfp_pivotmax(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_efficiency"]
    fn bfp_efficiency(lp: *mut lprec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_bfp_pivotvector"]
    fn bfp_pivotvector(lp: *mut lprec) -> *mut ::core::ffi::c_double;
    #[link_name="honest_lpsolve_bfp_pivotcount"]
    fn bfp_pivotcount(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_mustrefactorize"]
    fn bfp_mustrefactorize(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_refactcount"]
    fn bfp_refactcount(lp: *mut lprec, kind: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_isSetI"]
    fn bfp_isSetI(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_updaterefactstats"]
    fn bfp_updaterefactstats(lp: *mut lprec);
    #[link_name="honest_lpsolve_bfp_init"]
    fn bfp_init(
        lp: *mut lprec,
        size: ::core::ffi::c_int,
        deltasize: ::core::ffi::c_int,
        options: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_restart"]
    fn bfp_restart(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_implicitslack"]
    fn bfp_implicitslack(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_pivotalloc"]
    fn bfp_pivotalloc(lp: *mut lprec, newsize: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_colcount"]
    fn bfp_colcount(lp: *mut lprec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_bfp_canresetbasis"]
    fn bfp_canresetbasis(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_bfp_finishfactorization"]
    fn bfp_finishfactorization(lp: *mut lprec);
    #[link_name="honest_lpsolve_bfp_prepareupdate"]
    fn bfp_prepareupdate(
        lp: *mut lprec,
        row_nr: ::core::ffi::c_int,
        col_nr: ::core::ffi::c_int,
        pcol: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_bfp_pivotRHS"]
    fn bfp_pivotRHS(
        lp: *mut lprec,
        theta: ::core::ffi::c_double,
        pcol: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_bfp_btran_double"]
    fn bfp_btran_double(
        lp: *mut lprec,
        prow: *mut ::core::ffi::c_double,
        pnzidx: *mut ::core::ffi::c_int,
        drow: *mut ::core::ffi::c_double,
        dnzidx: *mut ::core::ffi::c_int,
    );
    #[link_name="honest_lpsolve_init_BLAS"]
    fn init_BLAS();
    #[link_name="honest_lpsolve_is_nativeBLAS"]
    fn is_nativeBLAS() -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_load_BLAS"]
    fn load_BLAS(libname: *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_unload_BLAS"]
    fn unload_BLAS() -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_lps_idamax"]
    fn lps_idamax(
        n: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_double,
        is: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_partial_createBlocks"]
    fn partial_createBlocks(lp: *mut lprec, isrow: ::core::ffi::c_uchar) -> *mut partialrec;
    #[link_name="honest_lpsolve_partial_countBlocks"]
    fn partial_countBlocks(lp: *mut lprec, isrow: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_partial_freeBlocks"]
    fn partial_freeBlocks(blockdata: *mut *mut partialrec);
    #[link_name="honest_lpsolve_partial_findBlocks"]
    fn partial_findBlocks(
        lp: *mut lprec,
        autodefine: ::core::ffi::c_uchar,
        isrow: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_multi_enteringtheta"]
    fn multi_enteringtheta(multi: *mut multirec) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_multi_free"]
    fn multi_free(multi: *mut *mut multirec);
    #[link_name="honest_lpsolve_initPricer"]
    fn initPricer(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_freePricer"]
    fn freePricer(lp: *mut lprec);
    #[link_name="honest_lpsolve_resizePricer"]
    fn resizePricer(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_updatePricer"]
    fn updatePricer(
        lp: *mut lprec,
        rownr: ::core::ffi::c_int,
        colnr: ::core::ffi::c_int,
        pcol: *mut ::core::ffi::c_double,
        prow: *mut ::core::ffi::c_double,
        nzprow: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
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
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmod(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    #[link_name="honest_lpsolve_allocLREAL"]
    fn allocLREAL(
        lp: *mut lprec,
        ptr: *mut *mut ::core::ffi::c_double,
        size: ::core::ffi::c_int,
        clear: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mempool_create"]
    fn mempool_create(lp: *mut lprec) -> *mut workarraysrec;
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
    #[link_name="honest_lpsolve_mempool_free"]
    fn mempool_free(mempool: *mut *mut workarraysrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_roundVector"]
    fn roundVector(
        myvector: *mut ::core::ffi::c_double,
        endpos: ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
    );
    #[link_name="honest_lpsolve_swapINT"]
    fn swapINT(item1: *mut ::core::ffi::c_int, item2: *mut ::core::ffi::c_int);
    #[link_name="honest_lpsolve_swapREAL"]
    fn swapREAL(item1: *mut ::core::ffi::c_double, item2: *mut ::core::ffi::c_double);
    #[link_name="honest_lpsolve_roundToPrecision"]
    fn roundToPrecision(
        value: ::core::ffi::c_double,
        precision: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_isINT"]
    fn isINT(lp: *mut lprec, value: ::core::ffi::c_double) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_rand_uniform"]
    fn rand_uniform(lp: *mut lprec, range: ::core::ffi::c_double) -> ::core::ffi::c_double;
    #[link_name="honest_lpsolve_isActiveLink"]
    fn isActiveLink(linkmap: *mut LLrec, itemnr: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_firstActiveLink"]
    fn firstActiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_nextActiveLink"]
    fn nextActiveLink(linkmap: *mut LLrec, backitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_firstInactiveLink"]
    fn firstInactiveLink(linkmap: *mut LLrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_nextInactiveLink"]
    fn nextInactiveLink(linkmap: *mut LLrec, backitemnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dlclose(__handle: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn dlopen(
        __path: *const ::core::ffi::c_char,
        __mode: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn dlsym(
        __handle: *mut ::core::ffi::c_void,
        __symbol: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    #[link_name="honest_lpsolve_create_hash_table"]
    fn create_hash_table(size: ::core::ffi::c_int, base: ::core::ffi::c_int) -> *mut hashtable;
    #[link_name="honest_lpsolve_free_hash_table"]
    fn free_hash_table(ht: *mut hashtable);
    #[link_name="honest_lpsolve_puthash"]
    fn puthash(
        name: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        list: *mut *mut hashelem,
        ht: *mut hashtable,
    ) -> *mut hashelem;
    #[link_name="honest_lpsolve_drophash"]
    fn drophash(name: *const ::core::ffi::c_char, list: *mut *mut hashelem, ht: *mut hashtable);
    #[link_name="honest_lpsolve_copy_hash_table"]
    fn copy_hash_table(
        ht: *mut hashtable,
        list: *mut *mut hashelem,
        newsize: ::core::ffi::c_int,
    ) -> *mut hashtable;
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
    #[link_name="honest_lpsolve_mat_create"]
    fn mat_create(
        lp: *mut lprec,
        rows: ::core::ffi::c_int,
        columns: ::core::ffi::c_int,
        epsvalue: ::core::ffi::c_double,
    ) -> *mut MATrec;
    #[link_name="honest_lpsolve_mat_memopt"]
    fn mat_memopt(
        mat: *mut MATrec,
        rowextra: ::core::ffi::c_int,
        colextra: ::core::ffi::c_int,
        nzextra: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_free"]
    fn mat_free(matrix: *mut *mut MATrec);
    #[link_name="honest_lpsolve_inc_matrow_space"]
    fn inc_matrow_space(mat: *mut MATrec, deltarows: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_inc_matcol_space"]
    fn inc_matcol_space(mat: *mut MATrec, deltacols: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_shiftrows"]
    fn mat_shiftrows(
        mat: *mut MATrec,
        bbase: *mut ::core::ffi::c_int,
        delta: ::core::ffi::c_int,
        varmap: *mut LLrec,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_shiftcols"]
    fn mat_shiftcols(
        mat: *mut MATrec,
        bbase: *mut ::core::ffi::c_int,
        delta: ::core::ffi::c_int,
        varmap: *mut LLrec,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_appendrow"]
    fn mat_appendrow(
        mat: *mut MATrec,
        count: ::core::ffi::c_int,
        row: *mut ::core::ffi::c_double,
        colno: *mut ::core::ffi::c_int,
        mult: ::core::ffi::c_double,
        checkrowmode: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_appendcol"]
    fn mat_appendcol(
        mat: *mut MATrec,
        count: ::core::ffi::c_int,
        column: *mut ::core::ffi::c_double,
        rowno: *mut ::core::ffi::c_int,
        mult: ::core::ffi::c_double,
        checkrowmode: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_get_data"]
    fn mat_get_data(
        lp: *mut lprec,
        matindex: ::core::ffi::c_int,
        isrow: ::core::ffi::c_uchar,
        rownr: *mut *mut ::core::ffi::c_int,
        colnr: *mut *mut ::core::ffi::c_int,
        value: *mut *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_validate"]
    fn mat_validate(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_findelm"]
    fn mat_findelm(
        mat: *mut MATrec,
        row: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_multcol"]
    fn mat_multcol(
        mat: *mut MATrec,
        col_nr: ::core::ffi::c_int,
        mult: ::core::ffi::c_double,
        DoObj: ::core::ffi::c_uchar,
    );
    #[link_name="honest_lpsolve_mat_setvalue"]
    fn mat_setvalue(
        mat: *mut MATrec,
        Row: ::core::ffi::c_int,
        Column: ::core::ffi::c_int,
        Value: ::core::ffi::c_double,
        doscale: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_mat_nonzeros"]
    fn mat_nonzeros(mat: *mut MATrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_collength"]
    fn mat_collength(mat: *mut MATrec, colnr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_mat_multrow"]
    fn mat_multrow(mat: *mut MATrec, row_nr: ::core::ffi::c_int, mult: ::core::ffi::c_double);
    #[link_name="honest_lpsolve_mat_multadd"]
    fn mat_multadd(
        mat: *mut MATrec,
        lhsvector: *mut ::core::ffi::c_double,
        varnr: ::core::ffi::c_int,
        mult: ::core::ffi::c_double,
    );
    #[link_name="honest_lpsolve_mat_setrow"]
    fn mat_setrow(
        mat: *mut MATrec,
        rowno: ::core::ffi::c_int,
        count: ::core::ffi::c_int,
        row: *mut ::core::ffi::c_double,
        colno: *mut ::core::ffi::c_int,
        doscale: ::core::ffi::c_uchar,
        checkrowmode: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_mat_transpose"]
    fn mat_transpose(mat: *mut MATrec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_invert"]
    fn invert(
        lp: *mut lprec,
        shiftbounds: ::core::ffi::c_uchar,
        final_0: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
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
    #[link_name="honest_lpsolve_bsolve"]
    fn bsolve(
        lp: *mut lprec,
        row_nr: ::core::ffi::c_int,
        rhsvector: *mut ::core::ffi::c_double,
        nzidx: *mut ::core::ffi::c_int,
        roundzero: ::core::ffi::c_double,
        ofscalar: ::core::ffi::c_double,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_strongbranch_BB"]
    fn strongbranch_BB(
        lp: *mut lprec,
        BB: *mut BBrec,
        varno: ::core::ffi::c_int,
        vartype: ::core::ffi::c_int,
        varcus: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_freecuts_BB"]
    fn freecuts_BB(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_rcfbound_BB"]
    fn rcfbound_BB(
        BB: *mut BBrec,
        varno: ::core::ffi::c_int,
        isINT_0: ::core::ffi::c_uchar,
        newbound: *mut ::core::ffi::c_double,
        isfeasible: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_pop_BB"]
    fn pop_BB(BB: *mut BBrec) -> *mut BBrec;
    #[link_name="honest_lpsolve_create_SOSgroup"]
    fn create_SOSgroup(lp: *mut lprec) -> *mut SOSgroup;
    #[link_name="honest_lpsolve_append_SOSgroup"]
    fn append_SOSgroup(group: *mut SOSgroup, SOS: *mut SOSrec) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_free_SOSgroup"]
    fn free_SOSgroup(group: *mut *mut SOSgroup);
    #[link_name="honest_lpsolve_create_SOSrec"]
    fn create_SOSrec(
        group: *mut SOSgroup,
        name: *mut ::core::ffi::c_char,
        type_0: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        size: ::core::ffi::c_int,
        variables: *mut ::core::ffi::c_int,
        weights: *mut ::core::ffi::c_double,
    ) -> *mut SOSrec;
    #[link_name="honest_lpsolve_SOS_shift_col"]
    fn SOS_shift_col(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
        delta: ::core::ffi::c_int,
        usedmap: *mut LLrec,
        forceresort: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_is_member"]
    fn SOS_is_member(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_SOS_is_marked"]
    fn SOS_is_marked(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_is_full"]
    fn SOS_is_full(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        column: ::core::ffi::c_int,
        activeonly: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_SOS_is_satisfied"]
    fn SOS_is_satisfied(
        group: *mut SOSgroup,
        sosindex: ::core::ffi::c_int,
        solution: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name="honest_lpsolve_write_params"]
    fn write_params(
        lp: *mut lprec,
        filename: *mut ::core::ffi::c_char,
        options: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_read_params"]
    fn read_params(
        lp: *mut lprec,
        filename: *mut ::core::ffi::c_char,
        options: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_MPS_readex"]
    fn MPS_readex(
        newlp: *mut *mut lprec,
        userhandle: *mut ::core::ffi::c_void,
        read_modeldata: Option<read_modeldata_func>,
        typeMPS: ::core::ffi::c_int,
        options: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_explain"]
    fn explain(lp: *mut lprec, format: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name="honest_lpsolve_report"]
    fn report(lp: *mut lprec, level: ::core::ffi::c_int, format: *mut ::core::ffi::c_char);
    #[link_name="honest_lpsolve_REPORT_objective"]
    fn REPORT_objective(lp: *mut lprec);
    #[link_name="honest_lpsolve_REPORT_solution"]
    fn REPORT_solution(lp: *mut lprec, columns: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_REPORT_constraints"]
    fn REPORT_constraints(lp: *mut lprec, columns: ::core::ffi::c_int);
    #[link_name="honest_lpsolve_REPORT_duals"]
    fn REPORT_duals(lp: *mut lprec);
    #[link_name="honest_lpsolve_REPORT_extended"]
    fn REPORT_extended(lp: *mut lprec);
    #[link_name="honest_lpsolve_REPORT_lp"]
    fn REPORT_lp(lp: *mut lprec);
    #[link_name="honest_lpsolve_REPORT_tableau"]
    fn REPORT_tableau(lp: *mut lprec) -> ::core::ffi::c_uchar;
    #[link_name="honest_lpsolve_REPORT_scales"]
    fn REPORT_scales(lp: *mut lprec);
    #[link_name="honest_lpsolve_gcd"]
    fn gcd(
        a: ::core::ffi::c_longlong,
        b: ::core::ffi::c_longlong,
        c: *mut ::core::ffi::c_int,
        d: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub type read_modeldata_func = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type findCompare_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub const ROWNAMEMASK: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"R%d\0") };
pub const ROWNAMEMASK2: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"r%d\0") };
pub const COLNAMEMASK: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"C%d\0") };
pub const COLNAMEMASK2: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"c%d\0") };
pub const BFP_STAT_REFACT_TOTAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const sensrejvar: ::core::ffi::c_int = TRUE;
#[export_name="honest_lpsolve_lp_solve_version"]
pub unsafe extern "C" fn lp_solve_version(
    mut majorversion: *mut ::core::ffi::c_int,
    mut minorversion: *mut ::core::ffi::c_int,
    mut release: *mut ::core::ffi::c_int,
    mut build: *mut ::core::ffi::c_int,
) {
    if !majorversion.is_null() {
        *majorversion = MAJORVERSION;
    }
    if !minorversion.is_null() {
        *minorversion = MINORVERSION;
    }
    if !release.is_null() {
        *release = RELEASE;
    }
    if !build.is_null() {
        *build = BUILD;
    }
}
unsafe extern "C" fn set_biton(
    mut bitarray: *mut ::core::ffi::c_uchar,
    mut item: ::core::ffi::c_int,
) {
    let ref mut fresh51 = *bitarray.offset((item / 8 as ::core::ffi::c_int) as isize);
    *fresh51 = (*fresh51 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << item % 8 as ::core::ffi::c_int)
        as ::core::ffi::c_uchar;
}
unsafe extern "C" fn is_biton(
    mut bitarray: *mut ::core::ffi::c_uchar,
    mut item: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (*bitarray.offset((item / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << item % 8 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_userabort"]
pub unsafe extern "C" fn userabort(
    mut lp: *mut lprec,
    mut message: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut abort: ::core::ffi::c_uchar = 0;
    let mut spx_save: ::core::ffi::c_int = 0;
    spx_save = (*lp).spx_status;
    (*lp).spx_status = RUNNING;
    if yieldformessages(lp) != 0 as ::core::ffi::c_int {
        (*lp).spx_status = USERABORT;
        if (*lp).bb_level > 0 as ::core::ffi::c_int {
            (*lp).bb_break = TRUE as ::core::ffi::c_uchar;
        }
    }
    if message > 0 as ::core::ffi::c_int
        && (*lp).usermessage.is_some()
        && (*lp).msgmask & message != 0
    {
        (*lp).usermessage.expect("non-null function pointer")(lp, (*lp).msghandle, message);
    }
    abort = ((*lp).spx_status != RUNNING) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if abort == 0 {
        (*lp).spx_status = spx_save;
    }
    return abort;
}
#[export_name="honest_lpsolve_yieldformessages"]
pub unsafe extern "C" fn yieldformessages(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).sectimeout > 0 as ::core::ffi::c_long
        && timeNow() - (*lp).timestart - (*lp).sectimeout as ::core::ffi::c_double
            > 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        (*lp).spx_status = TIMEOUT;
    }
    if (*lp).ctrlc.is_some() {
        let mut retcode: ::core::ffi::c_int =
            (*lp).ctrlc.expect("non-null function pointer")(lp, (*lp).ctrlchandle);
        if retcode == ACTION_RESTART && (*lp).bb_level > 1 as ::core::ffi::c_int {
            (*lp).bb_break = AUTOMATIC as ::core::ffi::c_uchar;
            retcode = 0 as ::core::ffi::c_int;
        }
        return retcode;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_set_outputstream"]
pub unsafe extern "C" fn set_outputstream(mut lp: *mut lprec, mut stream: *mut FILE) {
    if !(*lp).outstream.is_null() {
        if (*lp).streamowned != 0 {
            native_only!(fclose,(*lp).outstream);
        } else {
            native_only!(fflush,(*lp).outstream);
        }
    }
    (*lp).outstream = stream;
    (*lp).streamowned = FALSE as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_outputfile"]
pub unsafe extern "C" fn set_outputfile(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = 0;
    let mut output: *mut FILE = ::core::ptr::null_mut::<FILE>();
    ok = (filename.is_null() || *filename as ::core::ffi::c_int == 0 as ::core::ffi::c_int || {
        output = native_only!(fopen,filename, b"w\0" as *const u8 as *const ::core::ffi::c_char);
        !output.is_null()
    }) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if ok != 0 {
        set_outputstream(lp, output);
        (*lp).streamowned = (!filename.is_null()
            && *filename as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if !filename.is_null() && *filename as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*lp).outstream = ::core::ptr::null_mut::<FILE>();
        }
    }
    return ok;
}
#[export_name="honest_lpsolve_time_elapsed"]
pub unsafe extern "C" fn time_elapsed(mut lp: *mut lprec) -> ::core::ffi::c_double {
    if (*lp).timeend > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return (*lp).timeend - (*lp).timestart;
    } else {
        return timeNow() - (*lp).timestart;
    };
}
#[export_name="honest_lpsolve_put_bb_nodefunc"]
pub unsafe extern "C" fn put_bb_nodefunc(
    mut lp: *mut lprec,
    mut newnode: Option<lphandleint_intfunc>,
    mut bbnodehandle: *mut ::core::ffi::c_void,
) {
    (*lp).bb_usenode = newnode as Option<lphandleint_intfunc>;
    (*lp).bb_nodehandle = bbnodehandle;
}
#[export_name="honest_lpsolve_put_bb_branchfunc"]
pub unsafe extern "C" fn put_bb_branchfunc(
    mut lp: *mut lprec,
    mut newbranch: Option<lphandleint_intfunc>,
    mut bbbranchhandle: *mut ::core::ffi::c_void,
) {
    (*lp).bb_usebranch = newbranch as Option<lphandleint_intfunc>;
    (*lp).bb_branchhandle = bbbranchhandle;
}
#[export_name="honest_lpsolve_put_abortfunc"]
pub unsafe extern "C" fn put_abortfunc(
    mut lp: *mut lprec,
    mut newctrlc: Option<lphandle_intfunc>,
    mut ctrlchandle: *mut ::core::ffi::c_void,
) {
    (*lp).ctrlc = newctrlc as Option<lphandle_intfunc>;
    (*lp).ctrlchandle = ctrlchandle;
}
#[export_name="honest_lpsolve_put_logfunc"]
pub unsafe extern "C" fn put_logfunc(
    mut lp: *mut lprec,
    mut newlog: Option<lphandlestr_func>,
    mut loghandle: *mut ::core::ffi::c_void,
) {
    (*lp).writelog = newlog as Option<lphandlestr_func>;
    (*lp).loghandle = loghandle;
}
#[export_name="honest_lpsolve_put_msgfunc"]
pub unsafe extern "C" fn put_msgfunc(
    mut lp: *mut lprec,
    mut newmsg: Option<lphandleint_func>,
    mut msghandle: *mut ::core::ffi::c_void,
    mut mask: ::core::ffi::c_int,
) {
    (*lp).usermessage = newmsg as Option<lphandleint_func>;
    (*lp).msghandle = msghandle;
    (*lp).msgmask = mask;
}
#[export_name="honest_lpsolve_read_MPS"]
pub unsafe extern "C" fn read_MPS(
    mut filename: *mut ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut typeMPS: ::core::ffi::c_int = 0;
    typeMPS = (options & !(0x7 as ::core::ffi::c_int)) >> 2 as ::core::ffi::c_int;
    if typeMPS & (MPSFIXED | MPSFREE) == 0 as ::core::ffi::c_int {
        typeMPS |= MPSFIXED;
    }
    if MPS_readfile(
        &raw mut lp,
        filename,
        typeMPS,
        options & 0x7 as ::core::ffi::c_int,
    ) != 0
    {
        return lp;
    } else {
        return ::core::ptr::null_mut::<lprec>();
    };
}
#[export_name="honest_lpsolve_read_mps"]
pub unsafe extern "C" fn read_mps(
    mut filename: *mut FILE,
    mut options: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut typeMPS: ::core::ffi::c_int = 0;
    typeMPS = (options & !(0x7 as ::core::ffi::c_int)) >> 2 as ::core::ffi::c_int;
    if typeMPS & (MPSFIXED | MPSFREE) == 0 as ::core::ffi::c_int {
        typeMPS |= MPSFIXED;
    }
    if MPS_readhandle(
        &raw mut lp,
        filename,
        typeMPS,
        options & 0x7 as ::core::ffi::c_int,
    ) != 0
    {
        return lp;
    } else {
        return ::core::ptr::null_mut::<lprec>();
    };
}
#[export_name="honest_lpsolve_read_mpsex"]
pub unsafe extern "C" fn read_mpsex(
    mut userhandle: *mut ::core::ffi::c_void,
    mut read_modeldata: Option<read_modeldata_func>,
    mut options: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut typeMPS: ::core::ffi::c_int = 0;
    typeMPS = (options & !(0x7 as ::core::ffi::c_int)) >> 2 as ::core::ffi::c_int;
    if typeMPS & (MPSFIXED | MPSFREE) == 0 as ::core::ffi::c_int {
        typeMPS |= MPSFIXED;
    }
    if MPS_readex(
        &raw mut lp,
        userhandle,
        read_modeldata,
        typeMPS,
        options & 0x7 as ::core::ffi::c_int,
    ) != 0
    {
        return lp;
    } else {
        return ::core::ptr::null_mut::<lprec>();
    };
}
#[export_name="honest_lpsolve_read_freeMPS"]
pub unsafe extern "C" fn read_freeMPS(
    mut filename: *mut ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut typeMPS: ::core::ffi::c_int = 0;
    typeMPS = (options & !(0x7 as ::core::ffi::c_int)) >> 2 as ::core::ffi::c_int;
    typeMPS &= !MPSFIXED;
    typeMPS |= MPSFREE;
    if MPS_readfile(
        &raw mut lp,
        filename,
        typeMPS,
        options & 0x7 as ::core::ffi::c_int,
    ) != 0
    {
        return lp;
    } else {
        return ::core::ptr::null_mut::<lprec>();
    };
}
#[export_name="honest_lpsolve_read_freemps"]
pub unsafe extern "C" fn read_freemps(
    mut filename: *mut FILE,
    mut options: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut typeMPS: ::core::ffi::c_int = 0;
    typeMPS = (options & !(0x7 as ::core::ffi::c_int)) >> 2 as ::core::ffi::c_int;
    typeMPS &= !MPSFIXED;
    typeMPS |= MPSFREE;
    if MPS_readhandle(
        &raw mut lp,
        filename,
        typeMPS,
        options & 0x7 as ::core::ffi::c_int,
    ) != 0
    {
        return lp;
    } else {
        return ::core::ptr::null_mut::<lprec>();
    };
}
#[export_name="honest_lpsolve_read_freempsex"]
pub unsafe extern "C" fn read_freempsex(
    mut userhandle: *mut ::core::ffi::c_void,
    mut read_modeldata: Option<read_modeldata_func>,
    mut options: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut typeMPS: ::core::ffi::c_int = 0;
    typeMPS = (options & !(0x7 as ::core::ffi::c_int)) >> 2 as ::core::ffi::c_int;
    typeMPS &= !MPSFIXED;
    typeMPS |= MPSFREE;
    if MPS_readex(
        &raw mut lp,
        userhandle,
        read_modeldata,
        typeMPS,
        options & 0x7 as ::core::ffi::c_int,
    ) != 0
    {
        return lp;
    } else {
        return ::core::ptr::null_mut::<lprec>();
    };
}
#[export_name="honest_lpsolve_write_mps"]
pub unsafe extern "C" fn write_mps(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    return MPS_writefile(lp, MPSFIXED, filename);
}
#[export_name="honest_lpsolve_write_MPS"]
pub unsafe extern "C" fn write_MPS(
    mut lp: *mut lprec,
    mut output: *mut FILE,
) -> ::core::ffi::c_uchar {
    return MPS_writehandle(lp, MPSFIXED, output);
}
#[export_name="honest_lpsolve_write_freemps"]
pub unsafe extern "C" fn write_freemps(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    return MPS_writefile(lp, MPSFREE, filename);
}
#[export_name="honest_lpsolve_write_freeMPS"]
pub unsafe extern "C" fn write_freeMPS(
    mut lp: *mut lprec,
    mut output: *mut FILE,
) -> ::core::ffi::c_uchar {
    return MPS_writehandle(lp, MPSFREE, output);
}
#[export_name="honest_lpsolve_write_lp"]
pub unsafe extern "C" fn write_lp(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    return LP_writefile(lp, filename);
}
#[export_name="honest_lpsolve_write_LP"]
pub unsafe extern "C" fn write_LP(
    mut lp: *mut lprec,
    mut output: *mut FILE,
) -> ::core::ffi::c_uchar {
    return LP_writehandle(lp, output);
}
#[export_name="honest_lpsolve_LP_readhandle"]
pub unsafe extern "C" fn LP_readhandle(
    mut lp: *mut *mut lprec,
    mut filename: *mut FILE,
    mut verbose: ::core::ffi::c_int,
    mut lp_name: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_read_lp"]
pub unsafe extern "C" fn read_lp(
    mut filename: *mut FILE,
    mut verbose: ::core::ffi::c_int,
    mut lp_name: *mut ::core::ffi::c_char,
) -> *mut lprec {
    return ::core::ptr::null_mut::<lprec>();
}
#[export_name="honest_lpsolve_read_LP"]
pub unsafe extern "C" fn read_LP(
    mut filename: *mut ::core::ffi::c_char,
    mut verbose: ::core::ffi::c_int,
    mut lp_name: *mut ::core::ffi::c_char,
) -> *mut lprec {
    return ::core::ptr::null_mut::<lprec>();
}
#[export_name="honest_lpsolve_read_basis"]
pub unsafe extern "C" fn read_basis(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
    mut info: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut typeMPS: ::core::ffi::c_int = MPSFIXED;
    typeMPS = MPS_readBAS(lp, typeMPS, filename, info) as ::core::ffi::c_int;
    if typeMPS != 0 {
        set_action(
            &raw mut (*lp).spx_action,
            ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
        );
        (*lp).basis_valid = TRUE as ::core::ffi::c_uchar;
        *(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) = FALSE;
    }
    return typeMPS as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_reset_params"]
pub unsafe extern "C" fn reset_params(mut lp: *mut lprec) {
    let mut mode: ::core::ffi::c_int = 0;
    (*lp).epsmachine = DEF_EPSMACHINE;
    (*lp).epsperturb = DEF_PERTURB;
    (*lp).lag_accept = DEF_LAGACCEPT;
    set_epslevel(lp, EPS_DEFAULT);
    (*lp).tighten_on_set = FALSE as ::core::ffi::c_uchar;
    (*lp).negrange = DEF_NEGRANGE;
    (*lp).do_presolve = PRESOLVE_NONE;
    (*lp).presolveloops = DEF_MAXPRESOLVELOOPS;
    (*lp).scalelimit = DEF_SCALINGLIMIT as ::core::ffi::c_double;
    (*lp).scalemode = SCALE_INTEGERS | SCALE_LINEAR | SCALE_GEOMETRIC | SCALE_EQUILIBRATE;
    (*lp).crashmode = CRASH_NONE;
    (*lp).max_pivots = 0 as ::core::ffi::c_int;
    (*lp).simplex_strategy = SIMPLEX_DUAL_PRIMAL;
    mode = PRICER_DEVEX;
    mode |= PRICE_ADAPTIVE;
    set_pivoting(lp, mode);
    (*lp).improve = IMPROVE_DEFAULT;
    (*lp).anti_degen = ANTIDEGEN_DEFAULT;
    (*lp).bb_floorfirst = BRANCH_AUTOMATIC as ::core::ffi::c_uchar;
    (*lp).bb_rule = NODE_DYNAMICMODE
        | NODE_GREEDYMODE
        | NODE_GAPSELECT
        | NODE_PSEUDOCOSTSELECT
        | NODE_RCOSTFIXING;
    (*lp).bb_limitlevel = DEF_BB_LIMITLEVEL;
    (*lp).bb_PseudoUpdates = DEF_PSEUDOCOSTUPDATES;
    (*lp).bb_heuristicOF = if is_maxim(lp) as ::core::ffi::c_int != 0
        && (if 1.0e+30f64 > (*lp).infinite {
            1.0e+30f64
        } else {
            (*lp).infinite
        }) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        -if 1.0e+30f64 > (*lp).infinite {
            1.0e+30f64
        } else {
            (*lp).infinite
        }
    } else if 1.0e+30f64 > (*lp).infinite {
        1.0e+30f64
    } else {
        (*lp).infinite
    };
    (*lp).bb_breakOF = -(*lp).bb_heuristicOF;
    (*lp).sectimeout = 0 as ::core::ffi::c_long;
    (*lp).solutionlimit = 1 as ::core::ffi::c_int;
    set_outputstream(lp, ::core::ptr::null_mut::<FILE>());
    (*lp).verbose = NORMAL;
    (*lp).print_sol = FALSE;
    (*lp).spx_trace = FALSE as ::core::ffi::c_uchar;
    (*lp).lag_trace = FALSE as ::core::ffi::c_uchar;
    (*lp).bb_trace = FALSE as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_unscale"]
pub unsafe extern "C" fn unscale(mut lp: *mut lprec) {
    undoscale(lp);
}
#[export_name="honest_lpsolve_solve"]
pub unsafe extern "C" fn solve(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if has_BFP(lp) != 0 {
        (*lp).solvecount += 1;
        if is_add_rowmode(lp) != 0 {
            set_add_rowmode(lp, FALSE as ::core::ffi::c_uchar);
        }
        return lin_solve(lp);
    } else {
        return -(3 as ::core::ffi::c_int);
    };
}
#[export_name="honest_lpsolve_print_lp"]
pub unsafe extern "C" fn print_lp(mut lp: *mut lprec) {
    REPORT_lp(lp);
}
#[export_name="honest_lpsolve_print_tableau"]
pub unsafe extern "C" fn print_tableau(mut lp: *mut lprec) {
    REPORT_tableau(lp);
}
#[export_name="honest_lpsolve_print_objective"]
pub unsafe extern "C" fn print_objective(mut lp: *mut lprec) {
    REPORT_objective(lp);
}
#[export_name="honest_lpsolve_print_solution"]
pub unsafe extern "C" fn print_solution(mut lp: *mut lprec, mut columns: ::core::ffi::c_int) {
    REPORT_solution(lp, columns);
}
#[export_name="honest_lpsolve_print_constraints"]
pub unsafe extern "C" fn print_constraints(mut lp: *mut lprec, mut columns: ::core::ffi::c_int) {
    REPORT_constraints(lp, columns);
}
#[export_name="honest_lpsolve_print_duals"]
pub unsafe extern "C" fn print_duals(mut lp: *mut lprec) {
    REPORT_duals(lp);
}
#[export_name="honest_lpsolve_print_scales"]
pub unsafe extern "C" fn print_scales(mut lp: *mut lprec) {
    REPORT_scales(lp);
}
#[export_name="honest_lpsolve_print_str"]
pub unsafe extern "C" fn print_str(mut lp: *mut lprec, mut str: *mut ::core::ffi::c_char) {
    report(
        lp,
        (*lp).verbose,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
}
#[export_name="honest_lpsolve_set_timeout"]
pub unsafe extern "C" fn set_timeout(mut lp: *mut lprec, mut sectimeout: ::core::ffi::c_long) {
    (*lp).sectimeout = sectimeout;
}
#[export_name="honest_lpsolve_get_timeout"]
pub unsafe extern "C" fn get_timeout(mut lp: *mut lprec) -> ::core::ffi::c_long {
    return (*lp).sectimeout;
}
#[export_name="honest_lpsolve_set_verbose"]
pub unsafe extern "C" fn set_verbose(mut lp: *mut lprec, mut verbose: ::core::ffi::c_int) {
    (*lp).verbose = verbose;
}
#[export_name="honest_lpsolve_get_verbose"]
pub unsafe extern "C" fn get_verbose(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).verbose;
}
#[export_name="honest_lpsolve_set_print_sol"]
pub unsafe extern "C" fn set_print_sol(mut lp: *mut lprec, mut print_sol: ::core::ffi::c_int) {
    (*lp).print_sol = print_sol;
}
#[export_name="honest_lpsolve_get_print_sol"]
pub unsafe extern "C" fn get_print_sol(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).print_sol;
}
#[export_name="honest_lpsolve_set_debug"]
pub unsafe extern "C" fn set_debug(mut lp: *mut lprec, mut debug: ::core::ffi::c_uchar) {
    (*lp).bb_trace = debug;
}
#[export_name="honest_lpsolve_is_debug"]
pub unsafe extern "C" fn is_debug(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*lp).bb_trace;
}
#[export_name="honest_lpsolve_set_trace"]
pub unsafe extern "C" fn set_trace(mut lp: *mut lprec, mut trace: ::core::ffi::c_uchar) {
    (*lp).spx_trace = trace;
}
#[export_name="honest_lpsolve_is_trace"]
pub unsafe extern "C" fn is_trace(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*lp).spx_trace;
}
#[export_name="honest_lpsolve_set_anti_degen"]
pub unsafe extern "C" fn set_anti_degen(mut lp: *mut lprec, mut anti_degen: ::core::ffi::c_int) {
    (*lp).anti_degen = anti_degen;
}
#[export_name="honest_lpsolve_get_anti_degen"]
pub unsafe extern "C" fn get_anti_degen(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).anti_degen;
}
#[export_name="honest_lpsolve_is_anti_degen"]
pub unsafe extern "C" fn is_anti_degen(
    mut lp: *mut lprec,
    mut testmask: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return ((*lp).anti_degen == testmask || (*lp).anti_degen & testmask != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_presolve"]
pub unsafe extern "C" fn set_presolve(
    mut lp: *mut lprec,
    mut presolvemode: ::core::ffi::c_int,
    mut maxloops: ::core::ffi::c_int,
) {
    presolvemode &= !PRESOLVE_REDUCEMIP;
    (*lp).do_presolve = presolvemode;
    (*lp).presolveloops = maxloops;
}
#[export_name="honest_lpsolve_get_presolve"]
pub unsafe extern "C" fn get_presolve(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).do_presolve;
}
#[export_name="honest_lpsolve_get_presolveloops"]
pub unsafe extern "C" fn get_presolveloops(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).presolveloops < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    } else if (*lp).presolveloops == 0 as ::core::ffi::c_int {
        return 2147483647 as ::core::ffi::c_int;
    } else {
        return (*lp).presolveloops;
    };
}
#[export_name="honest_lpsolve_is_presolve"]
pub unsafe extern "C" fn is_presolve(
    mut lp: *mut lprec,
    mut testmask: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return ((*lp).do_presolve == testmask
        || (*lp).do_presolve & testmask != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_maxpivot"]
pub unsafe extern "C" fn set_maxpivot(mut lp: *mut lprec, mut maxpivot: ::core::ffi::c_int) {
    (*lp).max_pivots = maxpivot;
}
#[export_name="honest_lpsolve_get_maxpivot"]
pub unsafe extern "C" fn get_maxpivot(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).bfp_pivotmax.expect("non-null function pointer")(lp);
}
#[export_name="honest_lpsolve_set_bb_rule"]
pub unsafe extern "C" fn set_bb_rule(mut lp: *mut lprec, mut bb_rule: ::core::ffi::c_int) {
    (*lp).bb_rule = bb_rule;
}
#[export_name="honest_lpsolve_get_bb_rule"]
pub unsafe extern "C" fn get_bb_rule(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).bb_rule;
}
#[export_name="honest_lpsolve_is_bb_rule"]
pub unsafe extern "C" fn is_bb_rule(
    mut lp: *mut lprec,
    mut bb_rule: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return ((*lp).bb_rule & NODE_STRATEGYMASK == bb_rule) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_bb_mode"]
pub unsafe extern "C" fn is_bb_mode(
    mut lp: *mut lprec,
    mut bb_mask: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return ((*lp).bb_rule & bb_mask > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_action"]
pub unsafe extern "C" fn set_action(
    mut actionvar: *mut ::core::ffi::c_int,
    mut actionmask: ::core::ffi::c_int,
) {
    *actionvar |= actionmask;
}
#[export_name="honest_lpsolve_clear_action"]
pub unsafe extern "C" fn clear_action(
    mut actionvar: *mut ::core::ffi::c_int,
    mut actionmask: ::core::ffi::c_int,
) {
    *actionvar &= !actionmask;
}
#[export_name="honest_lpsolve_is_action"]
pub unsafe extern "C" fn is_action(
    mut actionvar: ::core::ffi::c_int,
    mut testmask: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (actionvar & testmask != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_bb_depthlimit"]
pub unsafe extern "C" fn set_bb_depthlimit(
    mut lp: *mut lprec,
    mut bb_maxlevel: ::core::ffi::c_int,
) {
    (*lp).bb_limitlevel = bb_maxlevel;
}
#[export_name="honest_lpsolve_get_bb_depthlimit"]
pub unsafe extern "C" fn get_bb_depthlimit(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).bb_limitlevel;
}
#[export_name="honest_lpsolve_set_obj_bound"]
pub unsafe extern "C" fn set_obj_bound(
    mut lp: *mut lprec,
    mut bb_heuristicOF: ::core::ffi::c_double,
) {
    (*lp).bb_heuristicOF = bb_heuristicOF;
}
#[export_name="honest_lpsolve_get_obj_bound"]
pub unsafe extern "C" fn get_obj_bound(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).bb_heuristicOF;
}
#[export_name="honest_lpsolve_set_mip_gap"]
pub unsafe extern "C" fn set_mip_gap(
    mut lp: *mut lprec,
    mut absolute: ::core::ffi::c_uchar,
    mut mip_gap: ::core::ffi::c_double,
) {
    if absolute != 0 {
        (*lp).mip_absgap = mip_gap;
    } else {
        (*lp).mip_relgap = mip_gap;
    };
}
#[export_name="honest_lpsolve_get_mip_gap"]
pub unsafe extern "C" fn get_mip_gap(
    mut lp: *mut lprec,
    mut absolute: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    if absolute != 0 {
        return (*lp).mip_absgap;
    } else {
        return (*lp).mip_relgap;
    };
}
#[export_name="honest_lpsolve_set_var_branch"]
pub unsafe extern "C" fn set_var_branch(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut branch_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_var_branch: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if (*lp).bb_varbranch.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        if branch_mode == BRANCH_DEFAULT {
            return 1 as ::core::ffi::c_uchar;
        }
        allocMYBOOL(
            lp,
            &raw mut (*lp).bb_varbranch,
            (*lp).columns_alloc,
            FALSE as ::core::ffi::c_uchar,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*lp).columns {
            *(*lp).bb_varbranch.offset(i as isize) = BRANCH_DEFAULT as ::core::ffi::c_uchar;
            i += 1;
        }
    }
    *(*lp)
        .bb_varbranch
        .offset((colnr - 1 as ::core::ffi::c_int) as isize) = branch_mode as ::core::ffi::c_uchar;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_var_branch"]
pub unsafe extern "C" fn get_var_branch(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_var_branch: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return (*lp).bb_floorfirst as ::core::ffi::c_int;
    }
    if (*lp).bb_varbranch.is_null() {
        return (*lp).bb_floorfirst as ::core::ffi::c_int;
    }
    if *(*lp)
        .bb_varbranch
        .offset((colnr - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        == BRANCH_DEFAULT
    {
        return (*lp).bb_floorfirst as ::core::ffi::c_int;
    } else {
        return *(*lp)
            .bb_varbranch
            .offset((colnr - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn set_infiniteex(
    mut lp: *mut lprec,
    mut infinite: ::core::ffi::c_double,
    mut init: ::core::ffi::c_uchar,
) {
    let mut i: ::core::ffi::c_int = 0;
    infinite = fabs(infinite);
    if init as ::core::ffi::c_int != 0
        || is_infinite(lp, (*lp).bb_heuristicOF) as ::core::ffi::c_int != 0
    {
        (*lp).bb_heuristicOF = if is_maxim(lp) as ::core::ffi::c_int != 0
            && infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -infinite
        } else {
            infinite
        };
    }
    if init as ::core::ffi::c_int != 0
        || is_infinite(lp, (*lp).bb_breakOF) as ::core::ffi::c_int != 0
    {
        (*lp).bb_breakOF = if is_maxim(lp) as ::core::ffi::c_int != 0
            && -infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            --infinite
        } else {
            -infinite
        };
    }
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        if init == 0
            && is_infinite(lp, *(*lp).orig_lowbo.offset(i as isize)) as ::core::ffi::c_int != 0
        {
            *(*lp).orig_lowbo.offset(i as isize) = -infinite;
        }
        if init as ::core::ffi::c_int != 0
            || is_infinite(lp, *(*lp).orig_upbo.offset(i as isize)) as ::core::ffi::c_int != 0
        {
            *(*lp).orig_upbo.offset(i as isize) = infinite;
        }
        i += 1;
    }
    (*lp).infinite = infinite;
}
#[export_name="honest_lpsolve_is_infinite"]
pub unsafe extern "C" fn is_infinite(
    mut lp: *mut lprec,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return (fabs(value) >= (*lp).infinite) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_infinite"]
pub unsafe extern "C" fn set_infinite(mut lp: *mut lprec, mut infinite: ::core::ffi::c_double) {
    set_infiniteex(lp, infinite, FALSE as ::core::ffi::c_uchar);
}
#[export_name="honest_lpsolve_get_infinite"]
pub unsafe extern "C" fn get_infinite(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).infinite;
}
#[export_name="honest_lpsolve_set_epsperturb"]
pub unsafe extern "C" fn set_epsperturb(mut lp: *mut lprec, mut epsperturb: ::core::ffi::c_double) {
    (*lp).epsperturb = epsperturb;
}
#[export_name="honest_lpsolve_get_epsperturb"]
pub unsafe extern "C" fn get_epsperturb(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).epsperturb;
}
#[export_name="honest_lpsolve_set_epspivot"]
pub unsafe extern "C" fn set_epspivot(mut lp: *mut lprec, mut epspivot: ::core::ffi::c_double) {
    (*lp).epspivot = epspivot;
}
#[export_name="honest_lpsolve_get_epspivot"]
pub unsafe extern "C" fn get_epspivot(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).epspivot;
}
#[export_name="honest_lpsolve_set_epsint"]
pub unsafe extern "C" fn set_epsint(mut lp: *mut lprec, mut epsint: ::core::ffi::c_double) {
    (*lp).epsint = epsint;
}
#[export_name="honest_lpsolve_get_epsint"]
pub unsafe extern "C" fn get_epsint(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).epsint;
}
#[export_name="honest_lpsolve_set_epsb"]
pub unsafe extern "C" fn set_epsb(mut lp: *mut lprec, mut epsb: ::core::ffi::c_double) {
    (*lp).epsprimal = if epsb > (*lp).epsmachine {
        epsb
    } else {
        (*lp).epsmachine
    };
}
#[export_name="honest_lpsolve_get_epsb"]
pub unsafe extern "C" fn get_epsb(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).epsprimal;
}
#[export_name="honest_lpsolve_set_epsd"]
pub unsafe extern "C" fn set_epsd(mut lp: *mut lprec, mut epsd: ::core::ffi::c_double) {
    (*lp).epsdual = if epsd > (*lp).epsmachine {
        epsd
    } else {
        (*lp).epsmachine
    };
}
#[export_name="honest_lpsolve_get_epsd"]
pub unsafe extern "C" fn get_epsd(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).epsdual;
}
#[export_name="honest_lpsolve_set_epsel"]
pub unsafe extern "C" fn set_epsel(mut lp: *mut lprec, mut epsel: ::core::ffi::c_double) {
    (*lp).epsvalue = if epsel > (*lp).epsmachine {
        epsel
    } else {
        (*lp).epsmachine
    };
}
#[export_name="honest_lpsolve_get_epsel"]
pub unsafe extern "C" fn get_epsel(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).epsvalue;
}
#[export_name="honest_lpsolve_set_epslevel"]
pub unsafe extern "C" fn set_epslevel(
    mut lp: *mut lprec,
    mut epslevel: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut SPX_RELAX: ::core::ffi::c_double = 0.;
    let mut MIP_RELAX: ::core::ffi::c_double = 0.;
    match epslevel {
        EPS_TIGHT => {
            SPX_RELAX = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            MIP_RELAX = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        EPS_MEDIUM => {
            SPX_RELAX = 10 as ::core::ffi::c_int as ::core::ffi::c_double;
            MIP_RELAX = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        EPS_LOOSE => {
            SPX_RELAX = 100 as ::core::ffi::c_int as ::core::ffi::c_double;
            MIP_RELAX = 10 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        EPS_BAGGY => {
            SPX_RELAX = 1000 as ::core::ffi::c_int as ::core::ffi::c_double;
            MIP_RELAX = 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        _ => return 0 as ::core::ffi::c_uchar,
    }
    (*lp).epsvalue = SPX_RELAX * DEF_EPSVALUE;
    (*lp).epsprimal = SPX_RELAX * DEF_EPSPRIMAL;
    (*lp).epsdual = SPX_RELAX * DEF_EPSDUAL;
    (*lp).epspivot = SPX_RELAX * DEF_EPSPIVOT;
    (*lp).epssolution = MIP_RELAX * DEF_EPSSOLUTION;
    (*lp).epsint = MIP_RELAX * DEF_EPSINT;
    (*lp).mip_absgap = MIP_RELAX * DEF_MIP_GAP;
    (*lp).mip_relgap = MIP_RELAX * DEF_MIP_GAP;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_scaling"]
pub unsafe extern "C" fn set_scaling(mut lp: *mut lprec, mut scalemode: ::core::ffi::c_int) {
    (*lp).scalemode = scalemode;
}
#[export_name="honest_lpsolve_get_scaling"]
pub unsafe extern "C" fn get_scaling(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).scalemode;
}
#[export_name="honest_lpsolve_is_scalemode"]
pub unsafe extern "C" fn is_scalemode(
    mut lp: *mut lprec,
    mut testmask: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return ((*lp).scalemode & testmask != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_scaletype"]
pub unsafe extern "C" fn is_scaletype(
    mut lp: *mut lprec,
    mut scaletype: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut testtype: ::core::ffi::c_int = 0;
    testtype = (*lp).scalemode & SCALE_MAXTYPE;
    return (scaletype == testtype) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_scalelimit"]
pub unsafe extern "C" fn set_scalelimit(mut lp: *mut lprec, mut scalelimit: ::core::ffi::c_double) {
    (*lp).scalelimit = fabs(scalelimit);
}
#[export_name="honest_lpsolve_get_scalelimit"]
pub unsafe extern "C" fn get_scalelimit(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).scalelimit;
}
#[export_name="honest_lpsolve_is_integerscaling"]
pub unsafe extern "C" fn is_integerscaling(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return is_scalemode(lp, SCALE_INTEGERS);
}
#[export_name="honest_lpsolve_set_improve"]
pub unsafe extern "C" fn set_improve(mut lp: *mut lprec, mut improve: ::core::ffi::c_int) {
    (*lp).improve = improve;
}
#[export_name="honest_lpsolve_get_improve"]
pub unsafe extern "C" fn get_improve(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).improve;
}
#[export_name="honest_lpsolve_set_lag_trace"]
pub unsafe extern "C" fn set_lag_trace(mut lp: *mut lprec, mut lag_trace: ::core::ffi::c_uchar) {
    (*lp).lag_trace = lag_trace;
}
#[export_name="honest_lpsolve_is_lag_trace"]
pub unsafe extern "C" fn is_lag_trace(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*lp).lag_trace;
}
#[export_name="honest_lpsolve_set_pivoting"]
pub unsafe extern "C" fn set_pivoting(mut lp: *mut lprec, mut pivoting: ::core::ffi::c_int) {
    (*lp).piv_strategy = pivoting;
    report(
        lp,
        5 as ::core::ffi::c_int,
        b"set_pivoting: Pricing strategy set to '%s'\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
}
#[export_name="honest_lpsolve_get_pivoting"]
pub unsafe extern "C" fn get_pivoting(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).piv_strategy;
}
#[export_name="honest_lpsolve_get_piv_rule"]
pub unsafe extern "C" fn get_piv_rule(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return ((*lp).piv_strategy | PRICE_STRATEGYMASK) ^ PRICE_STRATEGYMASK;
}
#[export_name="honest_lpsolve_get_str_piv_rule"]
pub unsafe extern "C" fn get_str_piv_rule(
    mut rule: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    static mut pivotText: [*mut ::core::ffi::c_char; 4] = [
        b"Bland first index\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"Dantzig\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"Devex\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"Steepest Edge\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    return pivotText[rule as usize];
}
#[export_name="honest_lpsolve_is_piv_rule"]
pub unsafe extern "C" fn is_piv_rule(
    mut lp: *mut lprec,
    mut rule: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (get_piv_rule(lp) == rule) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_piv_mode"]
pub unsafe extern "C" fn is_piv_mode(
    mut lp: *mut lprec,
    mut testmask: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (testmask & PRICE_STRATEGYMASK != 0 as ::core::ffi::c_int
        && (*lp).piv_strategy & testmask != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_break_at_first"]
pub unsafe extern "C" fn set_break_at_first(
    mut lp: *mut lprec,
    mut break_at_first: ::core::ffi::c_uchar,
) {
    (*lp).bb_breakfirst = break_at_first;
}
#[export_name="honest_lpsolve_is_break_at_first"]
pub unsafe extern "C" fn is_break_at_first(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*lp).bb_breakfirst;
}
#[export_name="honest_lpsolve_set_bb_floorfirst"]
pub unsafe extern "C" fn set_bb_floorfirst(
    mut lp: *mut lprec,
    mut bb_floorfirst: ::core::ffi::c_int,
) {
    (*lp).bb_floorfirst = bb_floorfirst as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_bb_floorfirst"]
pub unsafe extern "C" fn get_bb_floorfirst(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).bb_floorfirst as ::core::ffi::c_int;
}
#[export_name="honest_lpsolve_set_break_at_value"]
pub unsafe extern "C" fn set_break_at_value(
    mut lp: *mut lprec,
    mut break_at_value: ::core::ffi::c_double,
) {
    (*lp).bb_breakOF = break_at_value;
}
#[export_name="honest_lpsolve_get_break_at_value"]
pub unsafe extern "C" fn get_break_at_value(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).bb_breakOF;
}
#[export_name="honest_lpsolve_set_negrange"]
pub unsafe extern "C" fn set_negrange(mut lp: *mut lprec, mut negrange: ::core::ffi::c_double) {
    if negrange <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        (*lp).negrange = negrange;
    } else {
        (*lp).negrange = 0.0f64;
    };
}
#[export_name="honest_lpsolve_get_negrange"]
pub unsafe extern "C" fn get_negrange(mut lp: *mut lprec) -> ::core::ffi::c_double {
    return (*lp).negrange;
}
#[export_name="honest_lpsolve_get_max_level"]
pub unsafe extern "C" fn get_max_level(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).bb_maxlevel;
}
#[export_name="honest_lpsolve_get_total_nodes"]
pub unsafe extern "C" fn get_total_nodes(mut lp: *mut lprec) -> ::core::ffi::c_longlong {
    return (*lp).bb_totalnodes;
}
#[export_name="honest_lpsolve_get_total_iter"]
pub unsafe extern "C" fn get_total_iter(mut lp: *mut lprec) -> ::core::ffi::c_longlong {
    return (*lp).total_iter + (*lp).current_iter;
}
#[export_name="honest_lpsolve_get_objective"]
pub unsafe extern "C" fn get_objective(mut lp: *mut lprec) -> ::core::ffi::c_double {
    if !((*lp).spx_status == OPTIMAL) {
        if (*lp).basis_valid == 0 {
            report(
                lp,
                1 as ::core::ffi::c_int,
                b"get_objective: Not a valid basis\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return 0.0f64;
        }
    }
    return *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize);
}
#[export_name="honest_lpsolve_get_nonzeros"]
pub unsafe extern "C" fn get_nonzeros(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return mat_nonzeros((*lp).matA);
}
#[export_name="honest_lpsolve_set_mat"]
pub unsafe extern "C" fn set_mat(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_mat: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if colnr < 1 as ::core::ffi::c_int || colnr > (*lp).columns {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_mat: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if rownr == 0 as ::core::ffi::c_int {
        value = roundToPrecision(value, (*(*lp).matA).epsvalue);
    }
    value = scaled_mat(lp, value, rownr, colnr);
    if rownr == 0 as ::core::ffi::c_int {
        *(*lp).orig_obj.offset(colnr as isize) = if is_chsign(lp, rownr) as ::core::ffi::c_int != 0
            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -value
        } else {
            value
        };
        return 1 as ::core::ffi::c_uchar;
    } else {
        return mat_setvalue(
            (*lp).matA,
            rownr,
            colnr,
            value,
            FALSE as ::core::ffi::c_uchar,
        );
    };
}
#[export_name="honest_lpsolve_get_working_objective"]
pub unsafe extern "C" fn get_working_objective(mut lp: *mut lprec) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.0f64;
    if (*lp).basis_valid == 0 {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_working_objective: Not a valid basis\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else if (*lp).spx_status == RUNNING && (*lp).solutioncount == 0 as ::core::ffi::c_int {
        value = if is_maxim(lp) == 0
            && *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -*(*lp).rhs.offset(0 as ::core::ffi::c_int as isize)
        } else {
            *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize)
        };
    } else {
        value = *(*lp).solution.offset(0 as ::core::ffi::c_int as isize);
    }
    return value;
}
#[export_name="honest_lpsolve_get_var_primalresult"]
pub unsafe extern "C" fn get_var_primalresult(
    mut lp: *mut lprec,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if index < 0 as ::core::ffi::c_int || index > (*(*lp).presolve_undo).orig_sum {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_var_primalresult: Index %d out of range\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0.0f64;
    }
    if (*lp).do_presolve & PRESOLVE_LASTMASKMODE != PRESOLVE_NONE {
        return *(*lp).full_solution.offset(index as isize);
    } else {
        return *(*lp).best_solution.offset(index as isize);
    };
}
#[export_name="honest_lpsolve_get_var_dualresult"]
pub unsafe extern "C" fn get_var_dualresult(
    mut lp: *mut lprec,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut duals: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if index < 0 as ::core::ffi::c_int || index > (*(*lp).presolve_undo).orig_sum {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_var_dualresult: Index %d out of range\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0.0f64;
    }
    if index == 0 as ::core::ffi::c_int {
        return *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize);
    }
    if get_ptr_sensitivity_rhs(
        lp,
        &raw mut duals,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
    ) == 0
    {
        return 0.0f64;
    } else {
        duals = if (*lp).full_duals.is_null() {
            (*lp).duals
        } else {
            (*lp).full_duals
        };
    }
    return *duals.offset(index as isize);
}
#[export_name="honest_lpsolve_get_variables"]
pub unsafe extern "C" fn get_variables(
    mut lp: *mut lprec,
    mut var: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if !((*lp).spx_status == OPTIMAL) {
        if (*lp).basis_valid == 0 {
            report(
                lp,
                1 as ::core::ffi::c_int,
                b"get_variables: Not a valid basis\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
    }
    memcpy(
        var as *mut ::core::ffi::c_void,
        (*lp)
            .best_solution
            .offset((1 as ::core::ffi::c_int + (*lp).rows) as isize)
            as *const ::core::ffi::c_void,
        ((*lp).columns as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_ptr_variables"]
pub unsafe extern "C" fn get_ptr_variables(
    mut lp: *mut lprec,
    mut var: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if !((*lp).spx_status == OPTIMAL) {
        if (*lp).basis_valid == 0 {
            report(
                lp,
                1 as ::core::ffi::c_int,
                b"get_ptr_variables: Not a valid basis\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
    }
    if !var.is_null() {
        *var = (*lp)
            .best_solution
            .offset((1 as ::core::ffi::c_int + (*lp).rows) as isize);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_constraints"]
pub unsafe extern "C" fn get_constraints(
    mut lp: *mut lprec,
    mut constr: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if !((*lp).spx_status == OPTIMAL) {
        if (*lp).basis_valid == 0 {
            report(
                lp,
                1 as ::core::ffi::c_int,
                b"get_constraints: Not a valid basis\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
    }
    memcpy(
        constr as *mut ::core::ffi::c_void,
        (*lp).best_solution.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ((*lp).rows as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_ptr_constraints"]
pub unsafe extern "C" fn get_ptr_constraints(
    mut lp: *mut lprec,
    mut constr: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if !((*lp).spx_status == OPTIMAL) {
        if (*lp).basis_valid == 0 {
            report(
                lp,
                1 as ::core::ffi::c_int,
                b"get_ptr_constraints: Not a valid basis\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
    }
    if !constr.is_null() {
        *constr = (*lp).best_solution.offset(1 as ::core::ffi::c_int as isize);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_sensitivity_rhs"]
pub unsafe extern "C" fn get_sensitivity_rhs(
    mut lp: *mut lprec,
    mut duals: *mut ::core::ffi::c_double,
    mut dualsfrom: *mut ::core::ffi::c_double,
    mut dualstill: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut duals0: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut dualsfrom0: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut dualstill0: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    if (*lp).basis_valid == 0 {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_sensitivity_rhs: Not a valid basis\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if get_ptr_sensitivity_rhs(
        lp,
        if !duals.is_null() {
            &raw mut duals0
        } else {
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>()
        },
        if !dualsfrom.is_null() {
            &raw mut dualsfrom0
        } else {
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>()
        },
        if !dualstill.is_null() {
            &raw mut dualstill0
        } else {
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>()
        },
    ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    if !duals.is_null() {
        memcpy(
            duals as *mut ::core::ffi::c_void,
            duals0 as *const ::core::ffi::c_void,
            ((*lp).sum as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    if !dualsfrom.is_null() {
        memcpy(
            dualsfrom as *mut ::core::ffi::c_void,
            dualsfrom0 as *const ::core::ffi::c_void,
            ((*lp).sum as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    if !dualstill.is_null() {
        memcpy(
            dualstill as *mut ::core::ffi::c_void,
            dualstill0 as *const ::core::ffi::c_void,
            ((*lp).sum as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_ptr_sensitivity_rhs"]
pub unsafe extern "C" fn get_ptr_sensitivity_rhs(
    mut lp: *mut lprec,
    mut duals: *mut *mut ::core::ffi::c_double,
    mut dualsfrom: *mut *mut ::core::ffi::c_double,
    mut dualstill: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if (*lp).basis_valid == 0 {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_ptr_sensitivity_rhs: Not a valid basis\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if !duals.is_null() {
        if (*lp).duals.is_null() {
            if MIP_count(lp) > 0 as ::core::ffi::c_int
                && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
            {
                report(
                    lp,
                    1 as ::core::ffi::c_int,
                    b"get_ptr_sensitivity_rhs: Sensitivity unknown\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_uchar;
            }
            if construct_duals(lp) == 0 {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        *duals = (*lp).duals.offset(1 as ::core::ffi::c_int as isize);
    }
    if !dualsfrom.is_null() || !dualstill.is_null() {
        if (*lp).dualsfrom.is_null() || (*lp).dualstill.is_null() {
            if MIP_count(lp) > 0 as ::core::ffi::c_int
                && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
            {
                report(
                    lp,
                    1 as ::core::ffi::c_int,
                    b"get_ptr_sensitivity_rhs: Sensitivity unknown\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_uchar;
            }
            construct_sensitivity_duals(lp);
            if (*lp).dualsfrom.is_null() || (*lp).dualstill.is_null() {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        if !dualsfrom.is_null() {
            *dualsfrom = (*lp).dualsfrom.offset(1 as ::core::ffi::c_int as isize);
        }
        if !dualstill.is_null() {
            *dualstill = (*lp).dualstill.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_sensitivity_objex"]
pub unsafe extern "C" fn get_sensitivity_objex(
    mut lp: *mut lprec,
    mut objfrom: *mut ::core::ffi::c_double,
    mut objtill: *mut ::core::ffi::c_double,
    mut objfromvalue: *mut ::core::ffi::c_double,
    mut objtillvalue: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut objfrom0: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objtill0: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objfromvalue0: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut objtillvalue0: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    if (*lp).basis_valid == 0 {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_sensitivity_objex: Not a valid basis\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if get_ptr_sensitivity_objex(
        lp,
        if !objfrom.is_null() {
            &raw mut objfrom0
        } else {
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>()
        },
        if !objtill.is_null() {
            &raw mut objtill0
        } else {
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>()
        },
        if !objfromvalue.is_null() {
            &raw mut objfromvalue0
        } else {
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>()
        },
        if !objtillvalue.is_null() {
            &raw mut objtillvalue0
        } else {
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>()
        },
    ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    if !objfrom.is_null() && !objfrom0.is_null() {
        memcpy(
            objfrom as *mut ::core::ffi::c_void,
            objfrom0 as *const ::core::ffi::c_void,
            ((*lp).columns as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    if !objtill.is_null() && !objtill0.is_null() {
        memcpy(
            objtill as *mut ::core::ffi::c_void,
            objtill0 as *const ::core::ffi::c_void,
            ((*lp).columns as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    if !objfromvalue.is_null() && !objfromvalue0.is_null() {
        memcpy(
            objfromvalue as *mut ::core::ffi::c_void,
            objfromvalue0 as *const ::core::ffi::c_void,
            ((*lp).columns as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    if !objtillvalue.is_null() && !objtillvalue0.is_null() {
        memcpy(
            objtillvalue as *mut ::core::ffi::c_void,
            objtillvalue0 as *const ::core::ffi::c_void,
            ((*lp).columns as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_sensitivity_obj"]
pub unsafe extern "C" fn get_sensitivity_obj(
    mut lp: *mut lprec,
    mut objfrom: *mut ::core::ffi::c_double,
    mut objtill: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return get_sensitivity_objex(
        lp,
        objfrom,
        objtill,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[export_name="honest_lpsolve_get_ptr_sensitivity_objex"]
pub unsafe extern "C" fn get_ptr_sensitivity_objex(
    mut lp: *mut lprec,
    mut objfrom: *mut *mut ::core::ffi::c_double,
    mut objtill: *mut *mut ::core::ffi::c_double,
    mut objfromvalue: *mut *mut ::core::ffi::c_double,
    mut objtillvalue: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if (*lp).basis_valid == 0 {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_ptr_sensitivity_objex: Not a valid basis\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if !objfrom.is_null() || !objtill.is_null() {
        if (*lp).objfrom.is_null() || (*lp).objtill.is_null() {
            if MIP_count(lp) > 0 as ::core::ffi::c_int
                && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
            {
                report(
                    lp,
                    1 as ::core::ffi::c_int,
                    b"get_ptr_sensitivity_objex: Sensitivity unknown\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_uchar;
            }
            construct_sensitivity_obj(lp);
            if (*lp).objfrom.is_null() || (*lp).objtill.is_null() {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        if !objfrom.is_null() {
            *objfrom = (*lp).objfrom.offset(1 as ::core::ffi::c_int as isize);
        }
        if !objtill.is_null() {
            *objtill = (*lp).objtill.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if !objfromvalue.is_null() {
        if (*lp).objfromvalue.is_null() {
            if MIP_count(lp) > 0 as ::core::ffi::c_int
                && (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong
            {
                report(
                    lp,
                    1 as ::core::ffi::c_int,
                    b"get_ptr_sensitivity_objex: Sensitivity unknown\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_uchar;
            }
            construct_sensitivity_duals(lp);
            if (*lp).objfromvalue.is_null() {
                return 0 as ::core::ffi::c_uchar;
            }
        }
    }
    if !objfromvalue.is_null() {
        *objfromvalue = (*lp).objfromvalue.offset(1 as ::core::ffi::c_int as isize);
    }
    if !objtillvalue.is_null() {
        *objtillvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_ptr_sensitivity_obj"]
pub unsafe extern "C" fn get_ptr_sensitivity_obj(
    mut lp: *mut lprec,
    mut objfrom: *mut *mut ::core::ffi::c_double,
    mut objtill: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return get_ptr_sensitivity_objex(
        lp,
        objfrom,
        objtill,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
    );
}
#[export_name="honest_lpsolve_set_solutionlimit"]
pub unsafe extern "C" fn set_solutionlimit(mut lp: *mut lprec, mut limit: ::core::ffi::c_int) {
    (*lp).solutionlimit = limit;
}
#[export_name="honest_lpsolve_get_solutionlimit"]
pub unsafe extern "C" fn get_solutionlimit(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).solutionlimit;
}
#[export_name="honest_lpsolve_get_solutioncount"]
pub unsafe extern "C" fn get_solutioncount(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).solutioncount;
}
#[export_name="honest_lpsolve_get_Nrows"]
pub unsafe extern "C" fn get_Nrows(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).rows;
}
#[export_name="honest_lpsolve_get_Norig_rows"]
pub unsafe extern "C" fn get_Norig_rows(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).varmap_locked != 0 {
        return (*(*lp).presolve_undo).orig_rows;
    } else {
        return (*lp).rows;
    };
}
#[export_name="honest_lpsolve_get_Lrows"]
pub unsafe extern "C" fn get_Lrows(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).matL.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return (*(*lp).matL).rows;
    };
}
#[export_name="honest_lpsolve_get_Ncolumns"]
pub unsafe extern "C" fn get_Ncolumns(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).columns;
}
#[export_name="honest_lpsolve_get_Norig_columns"]
pub unsafe extern "C" fn get_Norig_columns(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).varmap_locked != 0 {
        return (*(*lp).presolve_undo).orig_columns;
    } else {
        return (*lp).columns;
    };
}
#[export_name="honest_lpsolve_get_status"]
pub unsafe extern "C" fn get_status(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).spx_status;
}
#[export_name="honest_lpsolve_get_statustext"]
pub unsafe extern "C" fn get_statustext(
    mut lp: *mut lprec,
    mut statuscode: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if statuscode == NOBFP {
        return b"No basis factorization package\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == DATAIGNORED {
        return b"Invalid input data provided\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == NOMEMORY {
        return b"Not enough memory available\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == NOTRUN {
        return b"Model has not been optimized\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == OPTIMAL {
        return b"OPTIMAL solution\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == SUBOPTIMAL {
        return b"SUB-OPTIMAL solution\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == INFEASIBLE {
        return b"Model is primal INFEASIBLE\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == UNBOUNDED {
        return b"Model is primal UNBOUNDED\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == RUNNING {
        return b"lp_solve is currently running\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == NUMFAILURE {
        return b"NUMERIC FAILURE encountered\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == DEGENERATE {
        return b"DEGENERATE situation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == USERABORT {
        return b"User-requested termination\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == TIMEOUT {
        return b"Termination due to timeout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == PRESOLVED {
        return b"Model solved by presolve\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == PROCFAIL {
        return b"B&B routine failed\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == PROCBREAK {
        return b"B&B routine terminated\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == FEASFOUND {
        return b"Feasible B&B solution found\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == NOFEASFOUND {
        return b"No feasible B&B solution found\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else if statuscode == FATHOMED {
        return b"Fathomed/pruned branch\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else {
        return b"Undefined internal error\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    };
}
#[export_name="honest_lpsolve_is_obj_in_basis"]
pub unsafe extern "C" fn is_obj_in_basis(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*lp).obj_in_basis;
}
#[export_name="honest_lpsolve_set_obj_in_basis"]
pub unsafe extern "C" fn set_obj_in_basis(
    mut lp: *mut lprec,
    mut obj_in_basis: ::core::ffi::c_uchar,
) {
    (*lp).obj_in_basis =
        (obj_in_basis as ::core::ffi::c_int == TRUE) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_make_lp"]
pub unsafe extern "C" fn make_lp(
    mut rows: ::core::ffi::c_int,
    mut columns: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    if rows < 0 as ::core::ffi::c_int || columns < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<lprec>();
    }
    lp = calloc(1 as size_t, ::core::mem::size_of::<lprec>() as size_t) as *mut lprec;
    if lp.is_null() {
        return ::core::ptr::null_mut::<lprec>();
    }
    set_lp_name(lp, ::core::ptr::null_mut::<::core::ffi::c_char>());
    (*lp).names_used = FALSE as ::core::ffi::c_uchar;
    (*lp).use_row_names = TRUE as ::core::ffi::c_uchar;
    (*lp).use_col_names = TRUE as ::core::ffi::c_uchar;
    (*lp).rowcol_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*lp).obj_in_basis = DEF_OBJINBASIS as ::core::ffi::c_uchar;
    (*lp).verbose = NORMAL;
    set_callbacks(lp);
    set_BFP(lp, ::core::ptr::null_mut::<::core::ffi::c_char>());
    set_XLI(lp, ::core::ptr::null_mut::<::core::ffi::c_char>());
    init_BLAS();
    is_nativeBLAS() as ::core::ffi::c_int != 0
        && load_BLAS(libnameBLAS.as_ptr() as *mut ::core::ffi::c_char) == 0;
    reset_params(lp);
    (*lp).source_is_file = FALSE as ::core::ffi::c_uchar;
    (*lp).model_is_pure = TRUE as ::core::ffi::c_uchar;
    (*lp).model_is_valid = FALSE as ::core::ffi::c_uchar;
    (*lp).spx_status = NOTRUN;
    (*lp).lag_status = NOTRUN;
    (*lp).workarrays = mempool_create(lp);
    (*lp).wasPreprocessed = FALSE as ::core::ffi::c_uchar;
    (*lp).wasPresolved = FALSE as ::core::ffi::c_uchar;
    presolve_createUndo(lp);
    (*lp).bb_varactive = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*lp).bb_varbranch = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*lp).var_priority = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*lp).rhsmax = 0.0f64;
    (*lp).bigM = 0.0f64;
    (*lp).bb_deltaOF = 0.0f64;
    (*lp).equalities = 0 as ::core::ffi::c_int;
    (*lp).fixedvars = 0 as ::core::ffi::c_int;
    (*lp).int_vars = 0 as ::core::ffi::c_int;
    (*lp).sc_vars = 0 as ::core::ffi::c_int;
    (*lp).sos_ints = 0 as ::core::ffi::c_int;
    (*lp).sos_vars = 0 as ::core::ffi::c_int;
    (*lp).sos_priority = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*lp).rows_alloc = 0 as ::core::ffi::c_int;
    (*lp).columns_alloc = 0 as ::core::ffi::c_int;
    (*lp).sum_alloc = 0 as ::core::ffi::c_int;
    (*lp).rows = rows;
    (*lp).columns = columns;
    (*lp).sum = rows + columns;
    varmap_clear(lp);
    (*lp).matA = mat_create(lp, rows, columns, (*lp).epsvalue);
    (*lp).matL = ::core::ptr::null_mut::<MATrec>();
    (*lp).invB = ::core::ptr::null_mut::<INVrec>();
    (*lp).duals = ::core::ptr::null_mut::<::core::ffi::c_double>();
    (*lp).dualsfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    (*lp).dualstill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    (*lp).objfromvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    (*lp).objfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    (*lp).objtill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    inc_col_space(lp, columns + 1 as ::core::ffi::c_int);
    inc_row_space(lp, rows + 1 as ::core::ffi::c_int);
    *(*lp).orig_lowbo.offset(0 as ::core::ffi::c_int as isize) =
        0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*lp).rootbounds = ::core::ptr::null_mut::<BBrec>();
    (*lp).bb_bounds = ::core::ptr::null_mut::<BBrec>();
    (*lp).bb_basis = ::core::ptr::null_mut::<basisrec>();
    (*lp).basis_valid = FALSE as ::core::ffi::c_uchar;
    (*lp).simplex_mode = SIMPLEX_DYNAMIC;
    (*lp).scaling_used = FALSE as ::core::ffi::c_uchar;
    (*lp).columns_scaled = FALSE as ::core::ffi::c_uchar;
    (*lp).P1extraDim = 0 as ::core::ffi::c_int;
    (*lp).P1extraVal = 0.0f64;
    (*lp).bb_strongbranches = 0 as ::core::ffi::c_int;
    (*lp).current_iter = 0 as ::core::ffi::c_longlong;
    (*lp).total_iter = 0 as ::core::ffi::c_longlong;
    (*lp).current_bswap = 0 as ::core::ffi::c_longlong;
    (*lp).total_bswap = 0 as ::core::ffi::c_longlong;
    (*lp).solutioncount = 0 as ::core::ffi::c_int;
    (*lp).solvecount = 0 as ::core::ffi::c_int;
    allocINT(
        lp,
        &raw mut (*lp).rejectpivot,
        DEF_MAXPIVOTRETRY + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    set_minim(lp);
    set_infiniteex(lp, DEF_INFINITE, TRUE as ::core::ffi::c_uchar);
    initPricer(lp);
    (*lp).ctrlc = None;
    (*lp).ctrlchandle = NULL;
    (*lp).writelog = None;
    (*lp).loghandle = NULL;
    (*lp).debuginfo = None;
    (*lp).usermessage = None;
    (*lp).msgmask = MSG_NONE;
    (*lp).msghandle = NULL;
    (*lp).timecreate = timeNow();
    return lp;
}
#[export_name="honest_lpsolve_resize_lp"]
pub unsafe extern "C" fn resize_lp(
    mut lp: *mut lprec,
    mut rows: ::core::ffi::c_int,
    mut columns: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if columns > (*lp).columns {
        status = inc_col_space(lp, columns - (*lp).columns);
    } else {
        while status as ::core::ffi::c_int != 0 && (*lp).columns > columns {
            status = del_column(lp, (*lp).columns);
        }
    }
    if status as ::core::ffi::c_int != 0 && rows > (*lp).rows {
        status = inc_row_space(lp, rows - (*lp).rows);
    } else {
        while status as ::core::ffi::c_int != 0 && (*lp).rows > rows {
            status = del_constraint(lp, (*lp).rows);
        }
    }
    return status;
}
#[export_name="honest_lpsolve_free_lp"]
pub unsafe extern "C" fn free_lp(mut plp: *mut *mut lprec) {
    if !plp.is_null() {
        let mut lp: *mut lprec = *plp;
        if !lp.is_null() {
            delete_lp(lp);
        }
        *plp = ::core::ptr::null_mut::<lprec>();
    }
}
#[export_name="honest_lpsolve_delete_lp"]
pub unsafe extern "C" fn delete_lp(mut lp: *mut lprec) {
    if lp.is_null() {
        return;
    }
    if !((*lp).rowcol_name as *mut ::core::ffi::c_void).is_null() {
        free((*lp).rowcol_name as *mut ::core::ffi::c_void);
        (*lp).rowcol_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !((*lp).lp_name as *mut ::core::ffi::c_void).is_null() {
        free((*lp).lp_name as *mut ::core::ffi::c_void);
        (*lp).lp_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !((*lp).ex_status as *mut ::core::ffi::c_void).is_null() {
        free((*lp).ex_status as *mut ::core::ffi::c_void);
        (*lp).ex_status = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*lp).names_used != 0 {
        if !((*lp).row_name as *mut ::core::ffi::c_void).is_null() {
            free((*lp).row_name as *mut ::core::ffi::c_void);
            (*lp).row_name = ::core::ptr::null_mut::<*mut hashelem>();
        }
        if !((*lp).col_name as *mut ::core::ffi::c_void).is_null() {
            free((*lp).col_name as *mut ::core::ffi::c_void);
            (*lp).col_name = ::core::ptr::null_mut::<*mut hashelem>();
        }
        free_hash_table((*lp).rowname_hashtab);
        free_hash_table((*lp).colname_hashtab);
    }
    mat_free(&raw mut (*lp).matA);
    (*lp).bfp_free.expect("non-null function pointer")(lp);
    if !(*lp).hBFP.is_null() {
        set_BFP(lp, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    if !(*lp).hXLI.is_null() {
        set_XLI(lp, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    unset_OF_p1extra(lp);
    if !((*lp).orig_obj as *mut ::core::ffi::c_void).is_null() {
        free((*lp).orig_obj as *mut ::core::ffi::c_void);
        (*lp).orig_obj = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).orig_rhs as *mut ::core::ffi::c_void).is_null() {
        free((*lp).orig_rhs as *mut ::core::ffi::c_void);
        (*lp).orig_rhs = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).rhs as *mut ::core::ffi::c_void).is_null() {
        free((*lp).rhs as *mut ::core::ffi::c_void);
        (*lp).rhs = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).var_type as *mut ::core::ffi::c_void).is_null() {
        free((*lp).var_type as *mut ::core::ffi::c_void);
        (*lp).var_type = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    set_var_weights(lp, ::core::ptr::null_mut::<::core::ffi::c_double>());
    if !((*lp).bb_varbranch as *mut ::core::ffi::c_void).is_null() {
        free((*lp).bb_varbranch as *mut ::core::ffi::c_void);
        (*lp).bb_varbranch = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !((*lp).sc_lobound as *mut ::core::ffi::c_void).is_null() {
        free((*lp).sc_lobound as *mut ::core::ffi::c_void);
        (*lp).sc_lobound = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).var_is_free as *mut ::core::ffi::c_void).is_null() {
        free((*lp).var_is_free as *mut ::core::ffi::c_void);
        (*lp).var_is_free = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((*lp).orig_upbo as *mut ::core::ffi::c_void).is_null() {
        free((*lp).orig_upbo as *mut ::core::ffi::c_void);
        (*lp).orig_upbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).orig_lowbo as *mut ::core::ffi::c_void).is_null() {
        free((*lp).orig_lowbo as *mut ::core::ffi::c_void);
        (*lp).orig_lowbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).upbo as *mut ::core::ffi::c_void).is_null() {
        free((*lp).upbo as *mut ::core::ffi::c_void);
        (*lp).upbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).lowbo as *mut ::core::ffi::c_void).is_null() {
        free((*lp).lowbo as *mut ::core::ffi::c_void);
        (*lp).lowbo = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).var_basic as *mut ::core::ffi::c_void).is_null() {
        free((*lp).var_basic as *mut ::core::ffi::c_void);
        (*lp).var_basic = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((*lp).is_basic as *mut ::core::ffi::c_void).is_null() {
        free((*lp).is_basic as *mut ::core::ffi::c_void);
        (*lp).is_basic = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !((*lp).is_lower as *mut ::core::ffi::c_void).is_null() {
        free((*lp).is_lower as *mut ::core::ffi::c_void);
        (*lp).is_lower = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !(*lp).bb_PseudoCost.is_null() {
        free_pseudocost(lp);
    }
    if !(*lp).bb_bounds.is_null() {
        report(
            lp,
            2 as ::core::ffi::c_int,
            b"delete_lp: The stack of B&B levels was not empty (failed at %.0f nodes)\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        unload_BB(lp);
    }
    if !(*lp).bb_basis.is_null() {
        unload_basis(lp, FALSE as ::core::ffi::c_uchar);
    }
    if !((*lp).rejectpivot as *mut ::core::ffi::c_void).is_null() {
        free((*lp).rejectpivot as *mut ::core::ffi::c_void);
        (*lp).rejectpivot = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    partial_freeBlocks(&raw mut (*lp).rowblocks);
    partial_freeBlocks(&raw mut (*lp).colblocks);
    multi_free(&raw mut (*lp).multivars);
    multi_free(&raw mut (*lp).longsteps);
    if !((*lp).solution as *mut ::core::ffi::c_void).is_null() {
        free((*lp).solution as *mut ::core::ffi::c_void);
        (*lp).solution = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).best_solution as *mut ::core::ffi::c_void).is_null() {
        free((*lp).best_solution as *mut ::core::ffi::c_void);
        (*lp).best_solution = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).full_solution as *mut ::core::ffi::c_void).is_null() {
        free((*lp).full_solution as *mut ::core::ffi::c_void);
        (*lp).full_solution = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    presolve_freeUndo(lp);
    mempool_free(&raw mut (*lp).workarrays);
    freePricer(lp);
    if !((*lp).drow as *mut ::core::ffi::c_void).is_null() {
        free((*lp).drow as *mut ::core::ffi::c_void);
        (*lp).drow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).nzdrow as *mut ::core::ffi::c_void).is_null() {
        free((*lp).nzdrow as *mut ::core::ffi::c_void);
        (*lp).nzdrow = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !((*lp).duals as *mut ::core::ffi::c_void).is_null() {
        free((*lp).duals as *mut ::core::ffi::c_void);
        (*lp).duals = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).full_duals as *mut ::core::ffi::c_void).is_null() {
        free((*lp).full_duals as *mut ::core::ffi::c_void);
        (*lp).full_duals = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).dualsfrom as *mut ::core::ffi::c_void).is_null() {
        free((*lp).dualsfrom as *mut ::core::ffi::c_void);
        (*lp).dualsfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).dualstill as *mut ::core::ffi::c_void).is_null() {
        free((*lp).dualstill as *mut ::core::ffi::c_void);
        (*lp).dualstill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).objfromvalue as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objfromvalue as *mut ::core::ffi::c_void);
        (*lp).objfromvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).objfrom as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objfrom as *mut ::core::ffi::c_void);
        (*lp).objfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).objtill as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objtill as *mut ::core::ffi::c_void);
        (*lp).objtill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).row_type as *mut ::core::ffi::c_void).is_null() {
        free((*lp).row_type as *mut ::core::ffi::c_void);
        (*lp).row_type = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if (*lp).sos_vars > 0 as ::core::ffi::c_int {
        if !((*lp).sos_priority as *mut ::core::ffi::c_void).is_null() {
            free((*lp).sos_priority as *mut ::core::ffi::c_void);
            (*lp).sos_priority = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    free_SOSgroup(&raw mut (*lp).SOS);
    free_SOSgroup(&raw mut (*lp).GUB);
    freecuts_BB(lp);
    if (*lp).scaling_used != 0 {
        if !((*lp).scalars as *mut ::core::ffi::c_void).is_null() {
            free((*lp).scalars as *mut ::core::ffi::c_void);
            (*lp).scalars = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    }
    if !(*lp).matL.is_null() {
        if !((*lp).lag_rhs as *mut ::core::ffi::c_void).is_null() {
            free((*lp).lag_rhs as *mut ::core::ffi::c_void);
            (*lp).lag_rhs = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !((*lp).lambda as *mut ::core::ffi::c_void).is_null() {
            free((*lp).lambda as *mut ::core::ffi::c_void);
            (*lp).lambda = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !((*lp).lag_con_type as *mut ::core::ffi::c_void).is_null() {
            free((*lp).lag_con_type as *mut ::core::ffi::c_void);
            (*lp).lag_con_type = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        mat_free(&raw mut (*lp).matL);
    }
    if (*lp).streamowned != 0 {
        set_outputstream(lp, ::core::ptr::null_mut::<FILE>());
    }
    if is_nativeBLAS() == 0 {
        unload_BLAS();
    }
    if !(lp as *mut ::core::ffi::c_void).is_null() {
        free(lp as *mut ::core::ffi::c_void);
        lp = ::core::ptr::null_mut::<lprec>();
    }
}
unsafe extern "C" fn get_SOS(
    mut lp: *mut lprec,
    mut index: ::core::ffi::c_int,
    mut name: *mut ::core::ffi::c_char,
    mut sostype: *mut ::core::ffi::c_int,
    mut priority: *mut ::core::ffi::c_int,
    mut count: *mut ::core::ffi::c_int,
    mut sosvars: *mut ::core::ffi::c_int,
    mut weights: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    if index < 1 as ::core::ffi::c_int || index > SOS_count(lp) {
        return 0 as ::core::ffi::c_uchar;
    }
    SOS = *(*(*lp).SOS)
        .sos_list
        .offset((index - 1 as ::core::ffi::c_int) as isize);
    if !name.is_null() {
        strcpy(name, (*SOS).name);
    }
    if !sostype.is_null() {
        *sostype = (*SOS).type_0;
    }
    if !priority.is_null() {
        *priority = (*SOS).priority;
    }
    if !count.is_null() {
        *count = (*SOS).size;
        if !sosvars.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 1 as ::core::ffi::c_int;
            while i <= *count {
                *sosvars.offset((i - 1 as ::core::ffi::c_int) as isize) =
                    *(*SOS).members.offset(i as isize);
                if !weights.is_null() {
                    *weights.offset((i - 1 as ::core::ffi::c_int) as isize) =
                        *(*SOS).weights.offset(i as isize);
                }
                i += 1;
            }
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_copy_lp"]
pub unsafe extern "C" fn copy_lp(mut lp: *mut lprec) -> *mut lprec {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut idx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut hold: ::core::ffi::c_double = 0.;
    let mut val: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut infinite: ::core::ffi::c_double = 0.;
    let mut newlp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut ok: ::core::ffi::c_char = FALSE as ::core::ffi::c_char;
    let mut sostype: ::core::ffi::c_int = 0;
    let mut priority: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut sosvars: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rows: ::core::ffi::c_int = 0;
    let mut columns: ::core::ffi::c_int = 0;
    let mut weights: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    rows = get_Nrows(lp);
    columns = get_Ncolumns(lp);
    if !(allocINT(
        lp,
        &raw mut idx,
        rows + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    ) == 0
        || allocREAL(
            lp,
            &raw mut val,
            rows + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) == 0)
    {
        newlp = make_lp(rows, 0 as ::core::ffi::c_int);
        if !newlp.is_null() {
            if !(resize_lp(newlp, rows, columns) == 0) {
                set_sense(newlp, is_maxim(lp));
                set_use_names(
                    newlp,
                    FALSE as ::core::ffi::c_uchar,
                    is_use_names(lp, FALSE as ::core::ffi::c_uchar),
                );
                set_use_names(
                    newlp,
                    TRUE as ::core::ffi::c_uchar,
                    is_use_names(lp, TRUE as ::core::ffi::c_uchar),
                );
                if !(set_lp_name(newlp, get_lp_name(lp)) == 0) {
                    set_verbose(newlp, get_verbose(lp));
                    set_epspivot(newlp, get_epspivot(lp));
                    set_epsel(newlp, get_epsel(lp));
                    set_epsb(newlp, get_epsb(lp));
                    set_epsd(newlp, get_epsd(lp));
                    set_pivoting(newlp, get_pivoting(lp));
                    set_negrange(newlp, (*lp).negrange);
                    set_infinite(newlp, get_infinite(lp));
                    set_presolve(newlp, get_presolve(lp), get_presolveloops(lp));
                    set_scaling(newlp, get_scaling(lp));
                    set_scalelimit(newlp, get_scalelimit(lp));
                    set_simplextype(newlp, get_simplextype(lp));
                    set_epsperturb(newlp, get_epsperturb(lp));
                    set_anti_degen(newlp, get_anti_degen(lp));
                    set_improve(newlp, get_improve(lp));
                    set_basiscrash(newlp, get_basiscrash(lp));
                    set_maxpivot(newlp, get_maxpivot(lp));
                    set_timeout(newlp, get_timeout(lp));
                    set_epsint(newlp, get_epsint(lp));
                    set_bb_rule(newlp, get_bb_rule(lp));
                    set_bb_depthlimit(newlp, get_bb_depthlimit(lp));
                    set_bb_floorfirst(newlp, get_bb_floorfirst(lp));
                    set_mip_gap(
                        newlp,
                        TRUE as ::core::ffi::c_uchar,
                        get_mip_gap(lp, TRUE as ::core::ffi::c_uchar),
                    );
                    set_mip_gap(
                        newlp,
                        FALSE as ::core::ffi::c_uchar,
                        get_mip_gap(lp, FALSE as ::core::ffi::c_uchar),
                    );
                    set_break_at_first(newlp, is_break_at_first(lp));
                    set_break_at_value(newlp, get_break_at_value(lp));
                    infinite = get_infinite(lp);
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i <= rows) {
                            current_block = 2873832966593178012;
                            break;
                        }
                        if i > 0 as ::core::ffi::c_int {
                            if set_constr_type(newlp, i, get_constr_type(lp, i)) == 0 {
                                current_block = 8775961366080407122;
                                break;
                            }
                        }
                        if set_rh(newlp, i, get_rh(lp, i)) == 0 {
                            current_block = 8775961366080407122;
                            break;
                        }
                        if i > 0 as ::core::ffi::c_int && {
                            hold = get_rh_range(lp, i);
                            hold < infinite
                        } {
                            if set_rh_range(newlp, i, hold) == 0 {
                                current_block = 8775961366080407122;
                                break;
                            }
                        }
                        if (*lp).names_used as ::core::ffi::c_int != 0
                            && (*lp).use_row_names as ::core::ffi::c_int != 0
                            && !(*(*lp).row_name.offset(i as isize)).is_null()
                            && !(**(*lp).row_name.offset(i as isize)).name.is_null()
                        {
                            if set_row_name(newlp, i, get_row_name(lp, i)) == 0 {
                                current_block = 8775961366080407122;
                                break;
                            }
                        }
                        i += 1;
                    }
                    match current_block {
                        8775961366080407122 => {}
                        _ => {
                            i = 1 as ::core::ffi::c_int;
                            loop {
                                if !(i <= columns) {
                                    current_block = 10095721787123848864;
                                    break;
                                }
                                n = get_columnex(lp, i, val, idx);
                                if n < 0 as ::core::ffi::c_int
                                    || add_columnex(newlp, n, val, idx) == 0
                                {
                                    current_block = 8775961366080407122;
                                    break;
                                }
                                if is_binary(lp, i) != 0 {
                                    if set_binary(newlp, i, TRUE as ::core::ffi::c_uchar) == 0 {
                                        current_block = 8775961366080407122;
                                        break;
                                    }
                                } else {
                                    if is_int(lp, i) != 0 {
                                        if set_int(newlp, i, TRUE as ::core::ffi::c_uchar) == 0 {
                                            current_block = 8775961366080407122;
                                            break;
                                        }
                                    }
                                    hold = get_lowbo(lp, i);
                                    if hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        if set_lowbo(newlp, i, hold) == 0 {
                                            current_block = 8775961366080407122;
                                            break;
                                        }
                                    }
                                    hold = get_upbo(lp, i);
                                    if hold < infinite {
                                        if set_upbo(newlp, i, hold) == 0 {
                                            current_block = 8775961366080407122;
                                            break;
                                        }
                                    }
                                }
                                if is_semicont(lp, i) != 0 {
                                    if set_semicont(newlp, i, TRUE as ::core::ffi::c_uchar) == 0 {
                                        current_block = 8775961366080407122;
                                        break;
                                    }
                                }
                                if (*lp).names_used as ::core::ffi::c_int != 0
                                    && (*lp).use_col_names as ::core::ffi::c_int != 0
                                    && !(*(*lp).col_name.offset(i as isize)).is_null()
                                    && !(**(*lp).col_name.offset(i as isize)).name.is_null()
                                {
                                    if set_col_name(newlp, i, get_col_name(lp, i)) == 0 {
                                        current_block = 8775961366080407122;
                                        break;
                                    }
                                }
                                i += 1;
                            }
                            match current_block {
                                8775961366080407122 => {}
                                _ => {
                                    i = 1 as ::core::ffi::c_int;
                                    loop {
                                        if !(get_SOS(
                                            lp,
                                            i,
                                            &raw mut buf as *mut ::core::ffi::c_char,
                                            &raw mut sostype,
                                            &raw mut priority,
                                            &raw mut count,
                                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                            ::core::ptr::null_mut::<::core::ffi::c_double>(),
                                        ) != 0)
                                        {
                                            current_block = 15970011996474399071;
                                            break;
                                        }
                                        if count != 0 {
                                            if allocINT(
                                                lp,
                                                &raw mut sosvars,
                                                count,
                                                FALSE as ::core::ffi::c_uchar,
                                            ) == 0
                                                || allocREAL(
                                                    lp,
                                                    &raw mut weights,
                                                    count,
                                                    FALSE as ::core::ffi::c_uchar,
                                                ) == 0
                                            {
                                                n = 0 as ::core::ffi::c_int;
                                            } else {
                                                get_SOS(
                                                    lp,
                                                    i,
                                                    &raw mut buf as *mut ::core::ffi::c_char,
                                                    &raw mut sostype,
                                                    &raw mut priority,
                                                    &raw mut count,
                                                    sosvars,
                                                    weights,
                                                );
                                                n = add_SOS(
                                                    newlp,
                                                    &raw mut buf as *mut ::core::ffi::c_char,
                                                    sostype,
                                                    priority,
                                                    count,
                                                    sosvars,
                                                    weights,
                                                );
                                            }
                                            if !(weights as *mut ::core::ffi::c_void).is_null() {
                                                free(weights as *mut ::core::ffi::c_void);
                                                weights =
                                                    ::core::ptr::null_mut::<::core::ffi::c_double>(
                                                    );
                                            }
                                            if !(sosvars as *mut ::core::ffi::c_void).is_null() {
                                                free(sosvars as *mut ::core::ffi::c_void);
                                                sosvars =
                                                    ::core::ptr::null_mut::<::core::ffi::c_int>();
                                            }
                                            if n == 0 as ::core::ffi::c_int {
                                                current_block = 8775961366080407122;
                                                break;
                                            }
                                        }
                                        i += 1;
                                    }
                                    match current_block {
                                        8775961366080407122 => {}
                                        _ => {
                                            ok = TRUE as ::core::ffi::c_char;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ok == 0 {
        free_lp(&raw mut newlp);
    }
    if !(val as *mut ::core::ffi::c_void).is_null() {
        free(val as *mut ::core::ffi::c_void);
        val = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(idx as *mut ::core::ffi::c_void).is_null() {
        free(idx as *mut ::core::ffi::c_void);
        idx = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return newlp;
}
#[export_name="honest_lpsolve_dualize_lp"]
pub unsafe extern "C" fn dualize_lp(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut item: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if MIP_count(lp) > 0 as ::core::ffi::c_int || (*lp).solvecount > 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uchar;
    }
    set_sense(
        lp,
        (is_maxim(lp) == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
    );
    n = mat_nonzeros(mat);
    mat_transpose(mat);
    item = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *item *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        i += 1;
        item = item.offset(matValueStep as isize);
    }
    swapINT(&raw mut (*lp).rows, &raw mut (*lp).columns);
    swapINT(&raw mut (*lp).rows_alloc, &raw mut (*lp).columns_alloc);
    swapREAL((*lp).orig_rhs, (*lp).orig_obj);
    if !(*lp).rhs.is_null() && !(*lp).obj.is_null() {
        swapREAL((*lp).rhs, (*lp).obj);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_memopt_lp"]
pub unsafe extern "C" fn memopt_lp(
    mut lp: *mut lprec,
    mut rowextra: ::core::ffi::c_int,
    mut colextra: ::core::ffi::c_int,
    mut nzextra: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if lp.is_null() {
        return status;
    }
    status = (mat_memopt((*lp).matA, rowextra, colextra, nzextra) as ::core::ffi::c_int != 0
        && {
            rowextra += 1;
            rowextra > 0 as ::core::ffi::c_int
        }
        && {
            colextra += 1;
            colextra > 0 as ::core::ffi::c_int
        }
        && {
            nzextra += 1;
            nzextra > 0 as ::core::ffi::c_int
        }) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    return status;
}
#[export_name="honest_lpsolve_varmap_lock"]
pub unsafe extern "C" fn varmap_lock(mut lp: *mut lprec) {
    presolve_fillUndo(lp, (*lp).rows, (*lp).columns, TRUE as ::core::ffi::c_uchar);
    (*lp).varmap_locked = TRUE as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_varmap_clear"]
pub unsafe extern "C" fn varmap_clear(mut lp: *mut lprec) {
    presolve_setOrig(lp, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    (*lp).varmap_locked = FALSE as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_varmap_canunlock"]
pub unsafe extern "C" fn varmap_canunlock(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    if (*lp).varmap_locked != 0 {
        let mut i: ::core::ffi::c_int = 0;
        let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
        if (*psundo).orig_columns > (*lp).columns || (*psundo).orig_rows > (*lp).rows {
            return 0 as ::core::ffi::c_uchar;
        }
        i = (*psundo).orig_rows + (*psundo).orig_columns;
        while i > 0 as ::core::ffi::c_int {
            if *(*psundo).orig_to_var.offset(i as isize) == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_uchar;
            }
            i -= 1;
        }
        i = (*lp).sum;
        while i > 0 as ::core::ffi::c_int {
            if *(*psundo).var_to_orig.offset(i as isize) == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_uchar;
            }
            i -= 1;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_varmap_add"]
pub unsafe extern "C" fn varmap_add(
    mut lp: *mut lprec,
    mut base: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    if (*lp).varmap_locked == 0 {
        return;
    }
    i = (*lp).sum;
    while i >= base {
        ii = i + delta;
        *(*psundo).var_to_orig.offset(ii as isize) = *(*psundo).var_to_orig.offset(i as isize);
        i -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < delta {
        ii = base + i;
        *(*psundo).var_to_orig.offset(ii as isize) = 0 as ::core::ffi::c_int;
        i += 1;
    }
}
#[export_name="honest_lpsolve_varmap_delete"]
pub unsafe extern "C" fn varmap_delete(
    mut lp: *mut lprec,
    mut base: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut varmap: *mut LLrec,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut preparecompact: ::core::ffi::c_uchar =
        (varmap != NULL as *mut LLrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    (*lp).model_is_pure = ((*lp).model_is_pure as ::core::ffi::c_int
        & ((*lp).solutioncount == 0 as ::core::ffi::c_int && preparecompact == 0)
            as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int)
        as ::core::ffi::c_uchar;
    if (*lp).varmap_locked == 0 {
        if (*lp).model_is_pure == 0 && (*lp).names_used as ::core::ffi::c_int != 0 {
            varmap_lock(lp);
        }
    }
    preparecompact = (varmap != NULL as *mut LLrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if preparecompact != 0 {
        preparecompact = (base > (*lp).rows) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        j = firstInactiveLink(varmap);
        while j != 0 as ::core::ffi::c_int {
            i = j;
            if preparecompact != 0 {
                i += (*lp).rows;
            }
            ii = *(*psundo).var_to_orig.offset(i as isize);
            if ii > 0 as ::core::ffi::c_int {
                *(*psundo).var_to_orig.offset(i as isize) = -ii;
            } else {
                *(*psundo).var_to_orig.offset(i as isize) =
                    -((*psundo).orig_rows + (*psundo).orig_columns + i);
            }
            j = nextInactiveLink(varmap, j);
        }
        return;
    }
    preparecompact = (base < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if preparecompact != 0 {
        base = -base;
        if base > (*lp).rows {
            base += (*psundo).orig_rows - (*lp).rows;
        }
        i = base;
        while i < base - delta {
            ii = *(*psundo).var_to_orig.offset(i as isize);
            if ii > 0 as ::core::ffi::c_int {
                *(*psundo).var_to_orig.offset(i as isize) = -ii;
            } else {
                *(*psundo).var_to_orig.offset(i as isize) =
                    -((*psundo).orig_rows + (*psundo).orig_columns + i);
            }
            i += 1;
        }
        return;
    }
    if varmap_canunlock(lp) != 0 {
        (*lp).varmap_locked = FALSE as ::core::ffi::c_uchar;
    }
    i = base;
    while i < base - delta {
        ii = *(*psundo).var_to_orig.offset(i as isize);
        if ii > 0 as ::core::ffi::c_int {
            *(*psundo).orig_to_var.offset(ii as isize) = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    i = base;
    while i <= (*lp).sum + delta {
        ii = i - delta;
        *(*psundo).var_to_orig.offset(i as isize) = *(*psundo).var_to_orig.offset(ii as isize);
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    j = (*psundo).orig_rows;
    if base > (*lp).rows {
        i += j;
        j += (*psundo).orig_columns;
    }
    ii = base - delta;
    while i <= j {
        if *(*psundo).orig_to_var.offset(i as isize) >= ii {
            *(*psundo).orig_to_var.offset(i as isize) += delta;
        }
        i += 1;
    }
}
#[export_name="honest_lpsolve_varmap_validate"]
pub unsafe extern "C" fn varmap_validate(
    mut lp: *mut lprec,
    mut varno: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut success: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut n_rows: ::core::ffi::c_int = (*lp).rows;
    let mut orig_sum: ::core::ffi::c_int = (*(*lp).presolve_undo).orig_sum;
    let mut orig_rows: ::core::ffi::c_int = (*(*lp).presolve_undo).orig_rows;
    if varno <= 0 as ::core::ffi::c_int {
        varno = 1 as ::core::ffi::c_int;
        ie = orig_sum;
    } else {
        ie = varno;
    }
    i = varno;
    while success as ::core::ffi::c_int != 0 && i <= ie {
        ix = *(*(*lp).presolve_undo).orig_to_var.offset(i as isize);
        if ix > 0 as ::core::ffi::c_int && i > orig_rows {
            ix += n_rows;
        }
        success = (ix <= orig_sum) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if success == 0 {
            report(
                lp,
                2 as ::core::ffi::c_int,
                b"varmap_validate: Invalid new mapping found for variable %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if ix != 0 as ::core::ffi::c_int {
            ii = *(*(*lp).presolve_undo).var_to_orig.offset(ix as isize);
            if ix > n_rows {
                ii += orig_rows;
            }
            success = (ii == i) as ::core::ffi::c_int as ::core::ffi::c_uchar;
            if success == 0 {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"varmap_validate: Invalid old mapping found for variable %d (%d)\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    return success;
}
#[export_name="honest_lpsolve_varmap_compact"]
pub unsafe extern "C" fn varmap_compact(
    mut lp: *mut lprec,
    mut prev_rows: ::core::ffi::c_int,
    mut prev_cols: ::core::ffi::c_int,
) {
    let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut n_sum: ::core::ffi::c_int = 0;
    let mut n_rows: ::core::ffi::c_int = 0;
    let mut orig_rows: ::core::ffi::c_int = (*psundo).orig_rows;
    let mut prev_sum: ::core::ffi::c_int = prev_rows + prev_cols;
    if (*lp).model_is_pure as ::core::ffi::c_int != 0 || (*lp).varmap_locked == 0 {
        return;
    }
    n_sum = 0 as ::core::ffi::c_int;
    n_rows = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= prev_sum {
        ii = *(*psundo).var_to_orig.offset(i as isize);
        if ii < 0 as ::core::ffi::c_int {
            ii = -ii;
            if i <= prev_rows {
                *(*psundo).orig_to_var.offset(ii as isize) = 0 as ::core::ffi::c_int;
            } else {
                *(*psundo).orig_to_var.offset((orig_rows + ii) as isize) = 0 as ::core::ffi::c_int;
            }
        } else {
            n_sum += 1;
            if n_sum < i {
                *(*psundo).var_to_orig.offset(n_sum as isize) = ii;
            }
            if ii > 0 as ::core::ffi::c_int {
                if i <= prev_rows {
                    *(*psundo).orig_to_var.offset(ii as isize) = n_sum;
                    n_rows = n_sum;
                } else {
                    *(*psundo).orig_to_var.offset((orig_rows + ii) as isize) = n_sum - n_rows;
                }
            }
        }
        i += 1;
    }
}
#[export_name="honest_lpsolve_shift_rowcoldata"]
pub unsafe extern "C" fn shift_rowcoldata(
    mut lp: *mut lprec,
    mut base: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut usedmap: *mut LLrec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut lodefault: ::core::ffi::c_double = 0.;
    if delta > 0 as ::core::ffi::c_int {
        let mut easyout: ::core::ffi::c_uchar = ((*lp).solvecount == 0 as ::core::ffi::c_int
            && base > (*lp).rows)
            as ::core::ffi::c_int
            as ::core::ffi::c_uchar;
        memmove(
            (*lp).orig_upbo.offset(base as isize).offset(delta as isize)
                as *mut ::core::ffi::c_void,
            (*lp).orig_upbo.offset(base as isize) as *const ::core::ffi::c_void,
            (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        memmove(
            (*lp)
                .orig_lowbo
                .offset(base as isize)
                .offset(delta as isize) as *mut ::core::ffi::c_void,
            (*lp).orig_lowbo.offset(base as isize) as *const ::core::ffi::c_void,
            (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        if easyout == 0 {
            memmove(
                (*lp).upbo.offset(base as isize).offset(delta as isize) as *mut ::core::ffi::c_void,
                (*lp).upbo.offset(base as isize) as *const ::core::ffi::c_void,
                (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            memmove(
                (*lp).lowbo.offset(base as isize).offset(delta as isize)
                    as *mut ::core::ffi::c_void,
                (*lp).lowbo.offset(base as isize) as *const ::core::ffi::c_void,
                (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            if (*lp).model_is_valid != 0 {
                memmove(
                    (*lp).solution.offset(base as isize).offset(delta as isize)
                        as *mut ::core::ffi::c_void,
                    (*lp).solution.offset(base as isize) as *const ::core::ffi::c_void,
                    (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
                memmove(
                    (*lp)
                        .best_solution
                        .offset(base as isize)
                        .offset(delta as isize) as *mut ::core::ffi::c_void,
                    (*lp).best_solution.offset(base as isize) as *const ::core::ffi::c_void,
                    (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
            }
            memmove(
                (*lp).is_lower.offset(base as isize).offset(delta as isize)
                    as *mut ::core::ffi::c_void,
                (*lp).is_lower.offset(base as isize) as *const ::core::ffi::c_void,
                (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t),
            );
        }
        if !(*lp).scalars.is_null() {
            if easyout == 0 {
                ii = (*lp).sum;
                while ii >= base {
                    i = ii + delta;
                    *(*lp).scalars.offset(i as isize) = *(*lp).scalars.offset(ii as isize);
                    ii -= 1;
                }
            }
            ii = base;
            while ii < base + delta {
                *(*lp).scalars.offset(ii as isize) =
                    1 as ::core::ffi::c_int as ::core::ffi::c_double;
                ii += 1;
            }
        }
        lodefault = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i = 0 as ::core::ffi::c_int;
        while i < delta {
            ii = base + i;
            *(*lp).orig_upbo.offset(ii as isize) = (*lp).infinite;
            *(*lp).orig_lowbo.offset(ii as isize) = lodefault;
            if easyout == 0 {
                *(*lp).upbo.offset(ii as isize) = *(*lp).orig_upbo.offset(ii as isize);
                *(*lp).lowbo.offset(ii as isize) = *(*lp).orig_lowbo.offset(ii as isize);
                *(*lp).is_lower.offset(ii as isize) = TRUE as ::core::ffi::c_uchar;
            }
            i += 1;
        }
    } else if !usedmap.is_null() {
        let mut k: ::core::ffi::c_int = 0;
        let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if isrow == 0 {
            offset += (*lp).rows;
        }
        i = offset + 1 as ::core::ffi::c_int;
        k = firstActiveLink(usedmap);
        while k != 0 as ::core::ffi::c_int {
            ii = k + offset;
            if !(ii == i) {
                *(*lp).upbo.offset(i as isize) = *(*lp).upbo.offset(ii as isize);
                *(*lp).orig_upbo.offset(i as isize) = *(*lp).orig_upbo.offset(ii as isize);
                *(*lp).lowbo.offset(i as isize) = *(*lp).lowbo.offset(ii as isize);
                *(*lp).orig_lowbo.offset(i as isize) = *(*lp).orig_lowbo.offset(ii as isize);
                *(*lp).solution.offset(i as isize) = *(*lp).solution.offset(ii as isize);
                *(*lp).best_solution.offset(i as isize) = *(*lp).best_solution.offset(ii as isize);
                *(*lp).is_lower.offset(i as isize) = *(*lp).is_lower.offset(ii as isize);
                if !(*lp).scalars.is_null() {
                    *(*lp).scalars.offset(i as isize) = *(*lp).scalars.offset(ii as isize);
                }
            }
            i += 1;
            k = nextActiveLink(usedmap, k);
        }
        if isrow != 0 {
            base = (*lp).rows + 1 as ::core::ffi::c_int;
            memmove(
                (*lp).upbo.offset(i as isize) as *mut ::core::ffi::c_void,
                (*lp).upbo.offset(base as isize) as *const ::core::ffi::c_void,
                ((*lp).columns as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            memmove(
                (*lp).orig_upbo.offset(i as isize) as *mut ::core::ffi::c_void,
                (*lp).orig_upbo.offset(base as isize) as *const ::core::ffi::c_void,
                ((*lp).columns as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            memmove(
                (*lp).lowbo.offset(i as isize) as *mut ::core::ffi::c_void,
                (*lp).lowbo.offset(base as isize) as *const ::core::ffi::c_void,
                ((*lp).columns as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            memmove(
                (*lp).orig_lowbo.offset(i as isize) as *mut ::core::ffi::c_void,
                (*lp).orig_lowbo.offset(base as isize) as *const ::core::ffi::c_void,
                ((*lp).columns as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
            if (*lp).model_is_valid != 0 {
                memmove(
                    (*lp).solution.offset(i as isize) as *mut ::core::ffi::c_void,
                    (*lp).solution.offset(base as isize) as *const ::core::ffi::c_void,
                    ((*lp).columns as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
                memmove(
                    (*lp).best_solution.offset(i as isize) as *mut ::core::ffi::c_void,
                    (*lp).best_solution.offset(base as isize) as *const ::core::ffi::c_void,
                    ((*lp).columns as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
            }
            memmove(
                (*lp).is_lower.offset(i as isize) as *mut ::core::ffi::c_void,
                (*lp).is_lower.offset(base as isize) as *const ::core::ffi::c_void,
                ((*lp).columns as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t),
            );
            if !(*lp).scalars.is_null() {
                memmove(
                    (*lp).scalars.offset(i as isize) as *mut ::core::ffi::c_void,
                    (*lp).scalars.offset(base as isize) as *const ::core::ffi::c_void,
                    ((*lp).columns as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
                );
            }
        }
    } else if delta < 0 as ::core::ffi::c_int {
        if base - delta - 1 as ::core::ffi::c_int > (*lp).sum {
            delta = base - (*lp).sum - 1 as ::core::ffi::c_int;
        }
        i = base;
        while i <= (*lp).sum + delta {
            ii = i - delta;
            *(*lp).upbo.offset(i as isize) = *(*lp).upbo.offset(ii as isize);
            *(*lp).orig_upbo.offset(i as isize) = *(*lp).orig_upbo.offset(ii as isize);
            *(*lp).lowbo.offset(i as isize) = *(*lp).lowbo.offset(ii as isize);
            *(*lp).orig_lowbo.offset(i as isize) = *(*lp).orig_lowbo.offset(ii as isize);
            *(*lp).solution.offset(i as isize) = *(*lp).solution.offset(ii as isize);
            *(*lp).best_solution.offset(i as isize) = *(*lp).best_solution.offset(ii as isize);
            *(*lp).is_lower.offset(i as isize) = *(*lp).is_lower.offset(ii as isize);
            if !(*lp).scalars.is_null() {
                *(*lp).scalars.offset(i as isize) = *(*lp).scalars.offset(ii as isize);
            }
            i += 1;
        }
    }
    (*lp).sum += delta;
    (*(*lp).matA).row_end_valid = FALSE as ::core::ffi::c_uchar;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_shift_basis"]
pub unsafe extern "C" fn shift_basis(
    mut lp: *mut lprec,
    mut base: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut usedmap: *mut LLrec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut Ok_0: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if is_BasisReady(lp) == 0 {
        return Ok_0;
    }
    if delta > 0 as ::core::ffi::c_int {
        if isrow != 0 {
            set_action(&raw mut (*lp).spx_action, ACTION_REBASE | ACTION_REINVERT);
        }
        if base <= (*lp).sum {
            memmove(
                (*lp).is_basic.offset(base as isize).offset(delta as isize)
                    as *mut ::core::ffi::c_void,
                (*lp).is_basic.offset(base as isize) as *const ::core::ffi::c_void,
                (((*lp).sum - base + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t),
            );
        }
        if (*lp).model_is_pure == 0 || (*lp).solvecount > 0 as ::core::ffi::c_int {
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).rows {
                ii = *(*lp).var_basic.offset(i as isize);
                if ii >= base {
                    *(*lp).var_basic.offset(i as isize) += delta;
                }
                i += 1;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < delta {
            ii = base + i;
            *(*lp).is_basic.offset(ii as isize) = isrow;
            if isrow != 0 {
                *(*lp)
                    .var_basic
                    .offset(((*lp).rows + 1 as ::core::ffi::c_int + i) as isize) = ii;
            }
            i += 1;
        }
    } else {
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        k = 0 as ::core::ffi::c_int;
        let mut current_block_28: u64;
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            ii = *(*lp).var_basic.offset(i as isize);
            *(*lp).is_basic.offset(ii as isize) = FALSE as ::core::ffi::c_uchar;
            if ii >= base {
                if ii < base - delta {
                    set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
                    current_block_28 = 11584701595673473500;
                } else {
                    ii += delta;
                    current_block_28 = 11057878835866523405;
                }
            } else {
                current_block_28 = 11057878835866523405;
            }
            match current_block_28 {
                11057878835866523405 => {
                    k += 1;
                    *(*lp).var_basic.offset(k as isize) = ii;
                }
                _ => {}
            }
            i += 1;
        }
        i = k;
        if isrow != 0 {
            i = if k < (*lp).rows + delta {
                k
            } else {
                (*lp).rows + delta
            };
        }
        while i > 0 as ::core::ffi::c_int {
            j = *(*lp).var_basic.offset(i as isize);
            *(*lp).is_basic.offset(j as isize) = TRUE as ::core::ffi::c_uchar;
            i -= 1;
        }
        if isrow == 0 && k < (*lp).rows {
            j = 0 as ::core::ffi::c_int;
            while j <= 1 as ::core::ffi::c_int {
                i = 1 as ::core::ffi::c_int;
                while i <= (*lp).rows && k < (*lp).rows {
                    if *(*lp).is_basic.offset(i as isize) == 0 {
                        if is_constr_type(lp, i, EQ) == 0 || j == 1 as ::core::ffi::c_int {
                            k += 1;
                            *(*lp).var_basic.offset(k as isize) = i;
                            *(*lp).is_basic.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
                        }
                    }
                    i += 1;
                }
                j += 1;
            }
            k = 0 as ::core::ffi::c_int;
        }
        if k + delta < 0 as ::core::ffi::c_int {
            Ok_0 = FALSE as ::core::ffi::c_uchar;
        }
        if isrow as ::core::ffi::c_int != 0 || k != (*lp).rows {
            set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
        }
    }
    return Ok_0;
}
#[export_name="honest_lpsolve_shift_rowdata"]
pub unsafe extern "C" fn shift_rowdata(
    mut lp: *mut lprec,
    mut base: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut usedmap: *mut LLrec,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    if (*(*lp).matA).is_roworder != 0 {
        mat_shiftcols((*lp).matA, &raw mut base, delta, usedmap);
    } else {
        mat_shiftrows((*lp).matA, &raw mut base, delta, usedmap);
    }
    if delta > 0 as ::core::ffi::c_int {
        ii = (*lp).rows;
        while ii >= base {
            i = ii + delta;
            *(*lp).orig_rhs.offset(i as isize) = *(*lp).orig_rhs.offset(ii as isize);
            *(*lp).rhs.offset(i as isize) = *(*lp).rhs.offset(ii as isize);
            *(*lp).row_type.offset(i as isize) = *(*lp).row_type.offset(ii as isize);
            ii -= 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < delta {
            ii = base + i;
            *(*lp).orig_rhs.offset(ii as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *(*lp).rhs.offset(ii as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *(*lp).row_type.offset(ii as isize) = ROWTYPE_EMPTY;
            i += 1;
        }
    } else if !usedmap.is_null() {
        i = 1 as ::core::ffi::c_int;
        ii = firstActiveLink(usedmap);
        while ii != 0 as ::core::ffi::c_int {
            if !(i == ii) {
                *(*lp).orig_rhs.offset(i as isize) = *(*lp).orig_rhs.offset(ii as isize);
                *(*lp).rhs.offset(i as isize) = *(*lp).rhs.offset(ii as isize);
                *(*lp).row_type.offset(i as isize) = *(*lp).row_type.offset(ii as isize);
            }
            i += 1;
            ii = nextActiveLink(usedmap, ii);
        }
        delta = i - (*lp).rows - 1 as ::core::ffi::c_int;
    } else if delta < 0 as ::core::ffi::c_int {
        if base - delta - 1 as ::core::ffi::c_int > (*lp).rows {
            delta = base - (*lp).rows - 1 as ::core::ffi::c_int;
        }
        i = base;
        while i <= (*lp).rows + delta {
            ii = i - delta;
            *(*lp).orig_rhs.offset(i as isize) = *(*lp).orig_rhs.offset(ii as isize);
            *(*lp).rhs.offset(i as isize) = *(*lp).rhs.offset(ii as isize);
            *(*lp).row_type.offset(i as isize) = *(*lp).row_type.offset(ii as isize);
            i += 1;
        }
    }
    shift_basis(lp, base, delta, usedmap, TRUE as ::core::ffi::c_uchar);
    shift_rowcoldata(lp, base, delta, usedmap, TRUE as ::core::ffi::c_uchar);
    inc_rows(lp, delta);
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_shift_coldata"]
pub unsafe extern "C" fn shift_coldata(
    mut lp: *mut lprec,
    mut base: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut usedmap: *mut LLrec,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong {
        free_duals(lp);
    }
    if (*(*lp).matA).is_roworder != 0 {
        mat_shiftrows((*lp).matA, &raw mut base, delta, usedmap);
    } else {
        mat_shiftcols((*lp).matA, &raw mut base, delta, usedmap);
    }
    if delta > 0 as ::core::ffi::c_int {
        if !(*lp).var_priority.is_null() && base <= (*lp).columns {
            i = 0 as ::core::ffi::c_int;
            while i < (*lp).columns {
                if *(*lp).var_priority.offset(i as isize) >= base {
                    *(*lp).var_priority.offset(i as isize) += delta;
                }
                i += 1;
            }
        }
        if !(*lp).sos_priority.is_null() && base <= (*lp).columns {
            i = 0 as ::core::ffi::c_int;
            while i < (*lp).sos_vars {
                if *(*lp).sos_priority.offset(i as isize) >= base {
                    *(*lp).sos_priority.offset(i as isize) += delta;
                }
                i += 1;
            }
        }
        if !(*lp).var_is_free.is_null() && base <= (*lp).columns {
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                if abs(*(*lp).var_is_free.offset(i as isize)) >= base {
                    *(*lp).var_is_free.offset(i as isize) +=
                        if *(*lp).var_is_free.offset(i as isize) < 0 as ::core::ffi::c_int
                            && delta != 0 as ::core::ffi::c_int
                        {
                            -delta
                        } else {
                            delta
                        };
                }
                i += 1;
            }
        }
        ii = (*lp).columns;
        while ii >= base {
            i = ii + delta;
            *(*lp).var_type.offset(i as isize) = *(*lp).var_type.offset(ii as isize);
            *(*lp).sc_lobound.offset(i as isize) = *(*lp).sc_lobound.offset(ii as isize);
            *(*lp).orig_obj.offset(i as isize) = *(*lp).orig_obj.offset(ii as isize);
            if !(*lp).obj.is_null() {
                *(*lp).obj.offset(i as isize) = *(*lp).obj.offset(ii as isize);
            }
            if !(*lp).var_priority.is_null() {
                *(*lp)
                    .var_priority
                    .offset((i - 1 as ::core::ffi::c_int) as isize) = *(*lp)
                    .var_priority
                    .offset((ii - 1 as ::core::ffi::c_int) as isize);
            }
            if !(*lp).bb_varbranch.is_null() {
                *(*lp)
                    .bb_varbranch
                    .offset((i - 1 as ::core::ffi::c_int) as isize) = *(*lp)
                    .bb_varbranch
                    .offset((ii - 1 as ::core::ffi::c_int) as isize);
            }
            if !(*lp).var_is_free.is_null() {
                *(*lp).var_is_free.offset(i as isize) = *(*lp).var_is_free.offset(ii as isize);
            }
            if !(*lp).best_solution.is_null() {
                *(*lp).best_solution.offset(((*lp).rows + i) as isize) =
                    *(*lp).best_solution.offset(((*lp).rows + ii) as isize);
            }
            ii -= 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < delta {
            ii = base + i;
            *(*lp).var_type.offset(ii as isize) = ISREAL as ::core::ffi::c_uchar;
            *(*lp).sc_lobound.offset(ii as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *(*lp).orig_obj.offset(ii as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            if !(*lp).obj.is_null() {
                *(*lp).obj.offset(ii as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if !(*lp).var_priority.is_null() {
                *(*lp)
                    .var_priority
                    .offset((ii - 1 as ::core::ffi::c_int) as isize) = ii;
            }
            if !(*lp).bb_varbranch.is_null() {
                *(*lp)
                    .bb_varbranch
                    .offset((ii - 1 as ::core::ffi::c_int) as isize) =
                    BRANCH_DEFAULT as ::core::ffi::c_uchar;
            }
            if !(*lp).var_is_free.is_null() {
                *(*lp).var_is_free.offset(ii as isize) = 0 as ::core::ffi::c_int;
            }
            if !(*lp).best_solution.is_null() {
                *(*lp).best_solution.offset(((*lp).rows + ii) as isize) =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            i += 1;
        }
    } else if !usedmap.is_null() {
        if (*lp).int_vars + (*lp).sc_vars > 0 as ::core::ffi::c_int {
            ii = firstInactiveLink(usedmap);
            while ii != 0 as ::core::ffi::c_int {
                if is_int(lp, ii) != 0 {
                    (*lp).int_vars -= 1;
                    if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, ii) != 0 {
                        (*lp).sos_ints -= 1;
                    }
                }
                if is_semicont(lp, ii) != 0 {
                    (*lp).sc_vars -= 1;
                }
                ii = nextInactiveLink(usedmap, ii);
            }
        }
        i = 1 as ::core::ffi::c_int;
        ii = firstActiveLink(usedmap);
        while ii != 0 as ::core::ffi::c_int {
            if !(i == ii) {
                *(*lp).var_type.offset(i as isize) = *(*lp).var_type.offset(ii as isize);
                *(*lp).sc_lobound.offset(i as isize) = *(*lp).sc_lobound.offset(ii as isize);
                *(*lp).orig_obj.offset(i as isize) = *(*lp).orig_obj.offset(ii as isize);
                if !(*lp).obj.is_null() {
                    *(*lp).obj.offset(i as isize) = *(*lp).obj.offset(ii as isize);
                }
                if !(*lp).bb_varbranch.is_null() {
                    *(*lp)
                        .bb_varbranch
                        .offset((i - 1 as ::core::ffi::c_int) as isize) = *(*lp)
                        .bb_varbranch
                        .offset((ii - 1 as ::core::ffi::c_int) as isize);
                }
                if !(*lp).var_is_free.is_null() {
                    *(*lp).var_is_free.offset(i as isize) = *(*lp).var_is_free.offset(ii as isize);
                }
                if !(*lp).best_solution.is_null() {
                    *(*lp).best_solution.offset(((*lp).rows + i) as isize) =
                        *(*lp).best_solution.offset(((*lp).rows + ii) as isize);
                }
            }
            i += 1;
            ii = nextActiveLink(usedmap, ii);
        }
        if !(*lp).var_priority.is_null() || !(*lp).sos_priority.is_null() {
            let mut colmap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
            let mut k: ::core::ffi::c_int = 0;
            allocINT(
                lp,
                &raw mut colmap,
                (*lp).columns + 1 as ::core::ffi::c_int,
                TRUE as ::core::ffi::c_uchar,
            );
            i = 1 as ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                if isActiveLink(usedmap, i) != 0 {
                    ii += 1;
                    *colmap.offset(i as isize) = ii;
                }
                i += 1;
            }
            if !(*lp).var_priority.is_null() {
                i = 0 as ::core::ffi::c_int;
                ii = 0 as ::core::ffi::c_int;
                while i < (*lp).columns {
                    k = *colmap.offset(*(*lp).var_priority.offset(i as isize) as isize);
                    if k > 0 as ::core::ffi::c_int {
                        *(*lp).var_priority.offset(ii as isize) = k;
                        ii += 1;
                    }
                    i += 1;
                }
            }
            if !(*lp).sos_priority.is_null() {
                i = 0 as ::core::ffi::c_int;
                ii = 0 as ::core::ffi::c_int;
                while i < (*lp).sos_vars {
                    k = *colmap.offset(*(*lp).sos_priority.offset(i as isize) as isize);
                    if k > 0 as ::core::ffi::c_int {
                        *(*lp).sos_priority.offset(ii as isize) = k;
                        ii += 1;
                    }
                    i += 1;
                }
                (*lp).sos_vars = ii;
            }
            if !(colmap as *mut ::core::ffi::c_void).is_null() {
                free(colmap as *mut ::core::ffi::c_void);
                colmap = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
        }
        delta = i - (*lp).columns - 1 as ::core::ffi::c_int;
    } else if delta < 0 as ::core::ffi::c_int {
        if !(*lp).var_is_free.is_null() {
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                if abs(*(*lp).var_is_free.offset(i as isize)) >= base {
                    *(*lp).var_is_free.offset(i as isize) -=
                        if *(*lp).var_is_free.offset(i as isize) < 0 as ::core::ffi::c_int
                            && delta != 0 as ::core::ffi::c_int
                        {
                            -delta
                        } else {
                            delta
                        };
                }
                i += 1;
            }
        }
        i = base;
        while i < base - delta {
            if is_int(lp, i) != 0 {
                (*lp).int_vars -= 1;
                if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, i) != 0 {
                    (*lp).sos_ints -= 1;
                }
            }
            if is_semicont(lp, i) != 0 {
                (*lp).sc_vars -= 1;
            }
            i += 1;
        }
        i = base;
        while i <= (*lp).columns + delta {
            ii = i - delta;
            *(*lp).var_type.offset(i as isize) = *(*lp).var_type.offset(ii as isize);
            *(*lp).sc_lobound.offset(i as isize) = *(*lp).sc_lobound.offset(ii as isize);
            *(*lp).orig_obj.offset(i as isize) = *(*lp).orig_obj.offset(ii as isize);
            if !(*lp).obj.is_null() {
                *(*lp).obj.offset(i as isize) = *(*lp).obj.offset(ii as isize);
            }
            if !(*lp).var_priority.is_null() {
                *(*lp)
                    .var_priority
                    .offset((i - 1 as ::core::ffi::c_int) as isize) = *(*lp)
                    .var_priority
                    .offset((ii - 1 as ::core::ffi::c_int) as isize);
            }
            if !(*lp).bb_varbranch.is_null() {
                *(*lp)
                    .bb_varbranch
                    .offset((i - 1 as ::core::ffi::c_int) as isize) = *(*lp)
                    .bb_varbranch
                    .offset((ii - 1 as ::core::ffi::c_int) as isize);
            }
            if !(*lp).var_is_free.is_null() {
                *(*lp).var_is_free.offset(i as isize) = *(*lp).var_is_free.offset(ii as isize);
            }
            if !(*lp).best_solution.is_null() {
                *(*lp).best_solution.offset(((*lp).rows + i) as isize) =
                    *(*lp).best_solution.offset(((*lp).rows + ii) as isize);
            }
            i += 1;
        }
        if !(*lp).var_priority.is_null() {
            i = 0 as ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            while i < (*lp).columns {
                if *(*lp).var_priority.offset(i as isize) > base - delta {
                    let fresh41 = ii;
                    ii = ii + 1;
                    *(*lp).var_priority.offset(fresh41 as isize) =
                        *(*lp).var_priority.offset(i as isize) + delta;
                } else if *(*lp).var_priority.offset(i as isize) < base {
                    let fresh42 = ii;
                    ii = ii + 1;
                    *(*lp).var_priority.offset(fresh42 as isize) =
                        *(*lp).var_priority.offset(i as isize);
                }
                i += 1;
            }
        }
        if !(*lp).sos_priority.is_null() {
            i = 0 as ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            while i < (*lp).sos_vars {
                if *(*lp).sos_priority.offset(i as isize) > base - delta {
                    let fresh43 = ii;
                    ii = ii + 1;
                    *(*lp).sos_priority.offset(fresh43 as isize) =
                        *(*lp).sos_priority.offset(i as isize) + delta;
                } else if *(*lp).sos_priority.offset(i as isize) < base {
                    let fresh44 = ii;
                    ii = ii + 1;
                    *(*lp).sos_priority.offset(fresh44 as isize) =
                        *(*lp).sos_priority.offset(i as isize);
                }
                i += 1;
            }
            (*lp).sos_vars = ii;
        }
    }
    shift_basis(
        lp,
        (*lp).rows + base,
        delta,
        usedmap,
        FALSE as ::core::ffi::c_uchar,
    );
    if SOS_count(lp) > 0 as ::core::ffi::c_int {
        SOS_shift_col(
            (*lp).SOS,
            0 as ::core::ffi::c_int,
            base,
            delta,
            usedmap,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    shift_rowcoldata(
        lp,
        (*lp).rows + base,
        delta,
        usedmap,
        FALSE as ::core::ffi::c_uchar,
    );
    inc_columns(lp, delta);
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_inc_rows"]
pub unsafe extern "C" fn inc_rows(mut lp: *mut lprec, mut delta: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).names_used as ::core::ffi::c_int != 0 && !(*lp).row_name.is_null() {
        i = (*lp).rows + delta;
        while i > (*lp).rows {
            let ref mut fresh40 = *(*lp).row_name.offset(i as isize);
            *fresh40 = ::core::ptr::null_mut::<hashelem>();
            i -= 1;
        }
    }
    (*lp).rows += delta;
    if (*(*lp).matA).is_roworder != 0 {
        (*(*lp).matA).columns += delta;
    } else {
        (*(*lp).matA).rows += delta;
    };
}
#[export_name="honest_lpsolve_inc_columns"]
pub unsafe extern "C" fn inc_columns(mut lp: *mut lprec, mut delta: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).names_used as ::core::ffi::c_int != 0 && !(*lp).col_name.is_null() {
        i = (*lp).columns + delta;
        while i > (*lp).columns {
            let ref mut fresh45 = *(*lp).col_name.offset(i as isize);
            *fresh45 = ::core::ptr::null_mut::<hashelem>();
            i -= 1;
        }
    }
    (*lp).columns += delta;
    if (*(*lp).matA).is_roworder != 0 {
        (*(*lp).matA).rows += delta;
    } else {
        (*(*lp).matA).columns += delta;
    }
    if get_Lrows(lp) > 0 as ::core::ffi::c_int {
        (*(*lp).matL).columns += delta;
    }
}
#[export_name="honest_lpsolve_inc_rowcol_space"]
pub unsafe extern "C" fn inc_rowcol_space(
    mut lp: *mut lprec,
    mut delta: ::core::ffi::c_int,
    mut isrows: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut oldrowcolalloc: ::core::ffi::c_int = 0;
    let mut rowcolsum: ::core::ffi::c_int = 0;
    if (*lp).solvecount > 0 as ::core::ffi::c_int {
        free_duals(lp);
    }
    oldrowcolalloc = (*lp).sum_alloc;
    (*lp).sum_alloc += delta;
    rowcolsum = (*lp).sum_alloc + 1 as ::core::ffi::c_int;
    if allocREAL(
        lp,
        &raw mut (*lp).upbo,
        rowcolsum,
        AUTOMATIC as ::core::ffi::c_uchar,
    ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).orig_upbo,
            rowcolsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).lowbo,
            rowcolsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).orig_lowbo,
            rowcolsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).solution,
            rowcolsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).best_solution,
            rowcolsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocMYBOOL(
            lp,
            &raw mut (*lp).is_basic,
            rowcolsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocMYBOOL(
            lp,
            &raw mut (*lp).is_lower,
            rowcolsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || !(*lp).scalars.is_null()
            && allocREAL(
                lp,
                &raw mut (*lp).scalars,
                rowcolsum,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
    i = oldrowcolalloc + 1 as ::core::ffi::c_int;
    while i < rowcolsum {
        *(*lp).upbo.offset(i as isize) = (*lp).infinite;
        *(*lp).orig_upbo.offset(i as isize) = *(*lp).upbo.offset(i as isize);
        *(*lp).lowbo.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).orig_lowbo.offset(i as isize) = *(*lp).lowbo.offset(i as isize);
        *(*lp).is_basic.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        i += 1;
    }
    if !(*lp).scalars.is_null() {
        i = oldrowcolalloc + 1 as ::core::ffi::c_int;
        while i < rowcolsum {
            *(*lp).scalars.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        if oldrowcolalloc == 0 as ::core::ffi::c_int {
            *(*lp).scalars.offset(0 as ::core::ffi::c_int as isize) =
                1 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    return (inc_presolve_space(lp, delta, isrows) as ::core::ffi::c_int != 0
        && resizePricer(lp) as ::core::ffi::c_int != 0) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_inc_lag_space"]
pub unsafe extern "C" fn inc_lag_space(
    mut lp: *mut lprec,
    mut deltarows: ::core::ffi::c_int,
    mut ignoreMAT: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut newsize: ::core::ffi::c_int = 0;
    if deltarows > 0 as ::core::ffi::c_int {
        newsize = get_Lrows(lp) + deltarows;
        if allocREAL(
            lp,
            &raw mut (*lp).lag_rhs,
            newsize + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
            || allocREAL(
                lp,
                &raw mut (*lp).lambda,
                newsize + 1 as ::core::ffi::c_int,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
            || allocINT(
                lp,
                &raw mut (*lp).lag_con_type,
                newsize + 1 as ::core::ffi::c_int,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
        {
            return 0 as ::core::ffi::c_uchar;
        }
        if ignoreMAT == 0 {
            if (*lp).matL.is_null() {
                (*lp).matL = mat_create(lp, newsize, (*lp).columns, (*lp).epsvalue);
            } else {
                inc_matrow_space((*lp).matL, deltarows);
            }
        }
        (*(*lp).matL).rows += deltarows;
    } else if ignoreMAT == 0 {
        inc_matcol_space(
            (*lp).matL,
            (*lp).columns_alloc - (*(*lp).matL).columns_alloc + 1 as ::core::ffi::c_int,
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_inc_row_space"]
pub unsafe extern "C" fn inc_row_space(
    mut lp: *mut lprec,
    mut deltarows: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut rowsum: ::core::ffi::c_int = 0;
    let mut oldrowsalloc: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    i = (*lp).rows_alloc + deltarows;
    if (*(*lp).matA).is_roworder != 0 {
        i -= (*(*lp).matA).columns_alloc;
        if i > deltarows {
            i = deltarows;
        }
        if i > 0 as ::core::ffi::c_int {
            inc_matcol_space((*lp).matA, i);
        }
        rowsum = (*(*lp).matA).columns_alloc;
    } else {
        i -= (*(*lp).matA).rows_alloc;
        if i > deltarows {
            i = deltarows;
        }
        if i > 0 as ::core::ffi::c_int {
            inc_matrow_space((*lp).matA, i);
        }
        rowsum = (*(*lp).matA).rows_alloc;
    }
    if (*lp).rows + deltarows > (*lp).rows_alloc {
        rowsum += 1;
        oldrowsalloc = (*lp).rows_alloc;
        (*lp).rows_alloc = rowsum;
        deltarows = rowsum - oldrowsalloc;
        rowsum += 1;
        if allocREAL(
            lp,
            &raw mut (*lp).orig_rhs,
            rowsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
            || allocLREAL(
                lp,
                &raw mut (*lp).rhs,
                rowsum,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
            || allocINT(
                lp,
                &raw mut (*lp).row_type,
                rowsum,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
            || allocINT(
                lp,
                &raw mut (*lp).var_basic,
                rowsum,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
        {
            return 0 as ::core::ffi::c_uchar;
        }
        if oldrowsalloc == 0 as ::core::ffi::c_int {
            *(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) = AUTOMATIC;
            *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *(*lp).row_type.offset(0 as ::core::ffi::c_int as isize) = ROWTYPE_OFMIN;
        }
        i = oldrowsalloc + 1 as ::core::ffi::c_int;
        while i < rowsum {
            *(*lp).orig_rhs.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *(*lp).rhs.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *(*lp).row_type.offset(i as isize) = ROWTYPE_EMPTY;
            *(*lp).var_basic.offset(i as isize) = i;
            i += 1;
        }
        if (*lp).names_used as ::core::ffi::c_int != 0 && !(*lp).row_name.is_null() {
            if (*(*lp).rowname_hashtab).size < (*lp).rows_alloc {
                let mut ht: *mut hashtable = ::core::ptr::null_mut::<hashtable>();
                ht = copy_hash_table(
                    (*lp).rowname_hashtab,
                    (*lp).row_name,
                    (*lp).rows_alloc + 1 as ::core::ffi::c_int,
                );
                if ht.is_null() {
                    (*lp).spx_status = NOMEMORY;
                    return 0 as ::core::ffi::c_uchar;
                }
                free_hash_table((*lp).rowname_hashtab);
                (*lp).rowname_hashtab = ht;
            }
            (*lp).row_name = realloc(
                (*lp).row_name as *mut ::core::ffi::c_void,
                (rowsum as size_t).wrapping_mul(::core::mem::size_of::<*mut hashelem>() as size_t),
            ) as *mut *mut hashelem;
            if (*lp).row_name.is_null() {
                (*lp).spx_status = NOMEMORY;
                return 0 as ::core::ffi::c_uchar;
            }
            i = oldrowsalloc + 1 as ::core::ffi::c_int;
            while i < rowsum {
                let ref mut fresh0 = *(*lp).row_name.offset(i as isize);
                *fresh0 = ::core::ptr::null_mut::<hashelem>();
                i += 1;
            }
        }
        ok = inc_rowcol_space(lp, deltarows, TRUE as ::core::ffi::c_uchar);
    }
    return ok;
}
#[export_name="honest_lpsolve_inc_col_space"]
pub unsafe extern "C" fn inc_col_space(
    mut lp: *mut lprec,
    mut deltacols: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut colsum: ::core::ffi::c_int = 0;
    let mut oldcolsalloc: ::core::ffi::c_int = 0;
    i = (*lp).columns_alloc + deltacols;
    if (*(*lp).matA).is_roworder != 0 {
        i -= (*(*lp).matA).rows_alloc;
        if i > deltacols {
            i = deltacols;
        }
        if i > 0 as ::core::ffi::c_int {
            inc_matrow_space((*lp).matA, i);
        }
        colsum = (*(*lp).matA).rows_alloc;
    } else {
        i -= (*(*lp).matA).columns_alloc;
        if i > deltacols {
            i = deltacols;
        }
        if i > 0 as ::core::ffi::c_int {
            inc_matcol_space((*lp).matA, i);
        }
        colsum = (*(*lp).matA).columns_alloc;
    }
    if (*lp).columns + deltacols >= (*lp).columns_alloc {
        colsum += 1;
        oldcolsalloc = (*lp).columns_alloc;
        (*lp).columns_alloc = colsum;
        deltacols = colsum - oldcolsalloc;
        colsum += 1;
        if (*lp).names_used as ::core::ffi::c_int != 0 && !(*lp).col_name.is_null() {
            if (*(*lp).colname_hashtab).size < (*lp).columns_alloc {
                let mut ht: *mut hashtable = ::core::ptr::null_mut::<hashtable>();
                ht = copy_hash_table(
                    (*lp).colname_hashtab,
                    (*lp).col_name,
                    (*lp).columns_alloc + 1 as ::core::ffi::c_int,
                );
                if !ht.is_null() {
                    free_hash_table((*lp).colname_hashtab);
                    (*lp).colname_hashtab = ht;
                }
            }
            (*lp).col_name = realloc(
                (*lp).col_name as *mut ::core::ffi::c_void,
                (colsum as size_t).wrapping_mul(::core::mem::size_of::<*mut hashelem>() as size_t),
            ) as *mut *mut hashelem;
            i = oldcolsalloc + 1 as ::core::ffi::c_int;
            while i < colsum {
                let ref mut fresh1 = *(*lp).col_name.offset(i as isize);
                *fresh1 = ::core::ptr::null_mut::<hashelem>();
                i += 1;
            }
        }
        if allocREAL(
            lp,
            &raw mut (*lp).orig_obj,
            colsum,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
            || allocMYBOOL(
                lp,
                &raw mut (*lp).var_type,
                colsum,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
            || allocREAL(
                lp,
                &raw mut (*lp).sc_lobound,
                colsum,
                AUTOMATIC as ::core::ffi::c_uchar,
            ) == 0
            || !(*lp).obj.is_null()
                && allocREAL(
                    lp,
                    &raw mut (*lp).obj,
                    colsum,
                    AUTOMATIC as ::core::ffi::c_uchar,
                ) == 0
            || !(*lp).var_priority.is_null()
                && allocINT(
                    lp,
                    &raw mut (*lp).var_priority,
                    colsum - 1 as ::core::ffi::c_int,
                    AUTOMATIC as ::core::ffi::c_uchar,
                ) == 0
            || !(*lp).var_is_free.is_null()
                && allocINT(
                    lp,
                    &raw mut (*lp).var_is_free,
                    colsum,
                    AUTOMATIC as ::core::ffi::c_uchar,
                ) == 0
            || !(*lp).bb_varbranch.is_null()
                && allocMYBOOL(
                    lp,
                    &raw mut (*lp).bb_varbranch,
                    colsum - 1 as ::core::ffi::c_int,
                    AUTOMATIC as ::core::ffi::c_uchar,
                ) == 0
        {
            return 0 as ::core::ffi::c_uchar;
        }
        if get_Lrows(lp) > 0 as ::core::ffi::c_int {
            inc_lag_space(lp, 0 as ::core::ffi::c_int, FALSE as ::core::ffi::c_uchar);
        }
        i = (if oldcolsalloc < (*lp).columns {
            oldcolsalloc
        } else {
            (*lp).columns
        }) + 1 as ::core::ffi::c_int;
        while i < colsum {
            *(*lp).orig_obj.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            if !(*lp).obj.is_null() {
                *(*lp).obj.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            *(*lp).var_type.offset(i as isize) = ISREAL as ::core::ffi::c_uchar;
            *(*lp).sc_lobound.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            if !(*lp).var_priority.is_null() {
                *(*lp)
                    .var_priority
                    .offset((i - 1 as ::core::ffi::c_int) as isize) = i;
            }
            i += 1;
        }
        if !(*lp).var_is_free.is_null() {
            i = oldcolsalloc + 1 as ::core::ffi::c_int;
            while i < colsum {
                *(*lp).var_is_free.offset(i as isize) = 0 as ::core::ffi::c_int;
                i += 1;
            }
        }
        if !(*lp).bb_varbranch.is_null() {
            i = oldcolsalloc;
            while i < colsum - 1 as ::core::ffi::c_int {
                *(*lp).bb_varbranch.offset(i as isize) = BRANCH_DEFAULT as ::core::ffi::c_uchar;
                i += 1;
            }
        }
        inc_rowcol_space(lp, deltacols, FALSE as ::core::ffi::c_uchar);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_obj"]
pub unsafe extern "C" fn set_obj(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if colnr <= 0 as ::core::ffi::c_int {
        colnr = set_rh(lp, 0 as ::core::ffi::c_int, value) as ::core::ffi::c_int;
    } else {
        colnr = set_mat(lp, 0 as ::core::ffi::c_int, colnr, value) as ::core::ffi::c_int;
    }
    return colnr as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_obj_fnex"]
pub unsafe extern "C" fn set_obj_fnex(
    mut lp: *mut lprec,
    mut count: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut chsgn: ::core::ffi::c_uchar = is_maxim(lp);
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0.;
    if row.is_null() {
        return 0 as ::core::ffi::c_uchar;
    } else if colno.is_null() {
        if count <= 0 as ::core::ffi::c_int {
            count = (*lp).columns;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= count {
            value = *row.offset(i as isize);
            value = roundToPrecision(value, (*(*lp).matA).epsvalue);
            *(*lp).orig_obj.offset(i as isize) = if chsgn as ::core::ffi::c_int != 0
                && scaled_mat(lp, value, 0 as ::core::ffi::c_int, i)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -scaled_mat(lp, value, 0 as ::core::ffi::c_int, i)
            } else {
                scaled_mat(lp, value, 0 as ::core::ffi::c_int, i)
            };
            i += 1;
        }
    } else {
        memset(
            (*lp).orig_obj as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*lp).columns + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        i = 0 as ::core::ffi::c_int;
        while i < count {
            ix = *colno.offset(i as isize);
            value = *row.offset(i as isize);
            value = roundToPrecision(value, (*(*lp).matA).epsvalue);
            *(*lp).orig_obj.offset(ix as isize) = if chsgn as ::core::ffi::c_int != 0
                && scaled_mat(lp, value, 0 as ::core::ffi::c_int, ix)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -scaled_mat(lp, value, 0 as ::core::ffi::c_int, ix)
            } else {
                scaled_mat(lp, value, 0 as ::core::ffi::c_int, ix)
            };
            i += 1;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_obj_fn"]
pub unsafe extern "C" fn set_obj_fn(
    mut lp: *mut lprec,
    mut row: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return set_obj_fnex(
        lp,
        0 as ::core::ffi::c_int,
        row,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[export_name="honest_lpsolve_str_set_obj_fn"]
pub unsafe extern "C" fn str_set_obj_fn(
    mut lp: *mut lprec,
    mut row_string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut arow: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut newp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    allocREAL(
        lp,
        &raw mut arow,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    p = row_string;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        *arow.offset(i as isize) = native_only!(strtod,p, &raw mut newp);
        if p == newp {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"str_set_obj_fn: Bad string %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            (*lp).spx_status = DATAIGNORED;
            ret = FALSE as ::core::ffi::c_uchar;
            break;
        } else {
            p = newp;
            i += 1;
        }
    }
    if (*lp).spx_status != DATAIGNORED {
        ret = set_obj_fn(lp, arow);
    }
    if !(arow as *mut ::core::ffi::c_void).is_null() {
        free(arow as *mut ::core::ffi::c_void);
        arow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return ret;
}
#[export_name="honest_lpsolve_append_columns"]
pub unsafe extern "C" fn append_columns(
    mut lp: *mut lprec,
    mut deltacolumns: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if inc_col_space(lp, deltacolumns) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    varmap_add(lp, (*lp).sum + 1 as ::core::ffi::c_int, deltacolumns);
    shift_coldata(
        lp,
        (*lp).columns + 1 as ::core::ffi::c_int,
        deltacolumns,
        ::core::ptr::null_mut::<LLrec>(),
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_append_rows"]
pub unsafe extern "C" fn append_rows(
    mut lp: *mut lprec,
    mut deltarows: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if inc_row_space(lp, deltarows) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    varmap_add(lp, (*lp).rows + 1 as ::core::ffi::c_int, deltarows);
    shift_rowdata(
        lp,
        (*lp).rows + 1 as ::core::ffi::c_int,
        deltarows,
        ::core::ptr::null_mut::<LLrec>(),
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_add_rowmode"]
pub unsafe extern "C" fn set_add_rowmode(
    mut lp: *mut lprec,
    mut turnon: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    if (*lp).solvecount == 0 as ::core::ffi::c_int
        && turnon as ::core::ffi::c_int ^ (*(*lp).matA).is_roworder as ::core::ffi::c_int != 0
    {
        return mat_transpose((*lp).matA);
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_is_add_rowmode"]
pub unsafe extern "C" fn is_add_rowmode(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*(*lp).matA).is_roworder;
}
#[export_name="honest_lpsolve_set_row"]
pub unsafe extern "C" fn set_row(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_row: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if rownr == 0 as ::core::ffi::c_int {
        return set_obj_fn(lp, row);
    } else {
        return mat_setrow(
            (*lp).matA,
            rownr,
            (*lp).columns,
            row,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            TRUE as ::core::ffi::c_uchar,
            TRUE as ::core::ffi::c_uchar,
        );
    };
}
#[export_name="honest_lpsolve_set_rowex"]
pub unsafe extern "C" fn set_rowex(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_rowex: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if rownr == 0 as ::core::ffi::c_int {
        return set_obj_fnex(lp, count, row, colno);
    } else {
        return mat_setrow(
            (*lp).matA,
            rownr,
            count,
            row,
            colno,
            TRUE as ::core::ffi::c_uchar,
            TRUE as ::core::ffi::c_uchar,
        );
    };
}
#[export_name="honest_lpsolve_add_constraintex"]
pub unsafe extern "C" fn add_constraintex(
    mut lp: *mut lprec,
    mut count: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
    mut constr_type: ::core::ffi::c_int,
    mut rh: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut n: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if !(constr_type == LE || constr_type == GE || constr_type == EQ) {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"add_constraintex: Invalid %d constraint type\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return status;
    }
    if append_rows(lp, 1 as ::core::ffi::c_int) == 0 {
        return status;
    }
    if constr_type & ROWTYPE_CONSTRAINT == EQ {
        (*lp).equalities += 1;
        *(*lp).orig_upbo.offset((*lp).rows as isize) =
            0 as ::core::ffi::c_int as ::core::ffi::c_double;
        *(*lp).upbo.offset((*lp).rows as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    *(*lp).row_type.offset((*lp).rows as isize) = constr_type;
    if is_chsign(lp, (*lp).rows) as ::core::ffi::c_int != 0
        && rh != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        *(*lp).orig_rhs.offset((*lp).rows as isize) = -rh;
    } else {
        *(*lp).orig_rhs.offset((*lp).rows as isize) = rh;
    }
    if colno.is_null() && !row.is_null() {
        n = (*lp).columns;
    } else {
        n = count;
    }
    mat_appendrow(
        (*lp).matA,
        n,
        row,
        colno,
        if is_chsign(lp, (*lp).rows) as ::core::ffi::c_int != 0
            && 1.0f64 != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -1.0f64
        } else {
            1.0f64
        },
        TRUE as ::core::ffi::c_uchar,
    );
    if (*lp).varmap_locked == 0 {
        presolve_setOrig(lp, (*lp).rows, (*lp).columns);
    }
    status = TRUE as ::core::ffi::c_uchar;
    return status;
}
#[export_name="honest_lpsolve_add_constraint"]
pub unsafe extern "C" fn add_constraint(
    mut lp: *mut lprec,
    mut row: *mut ::core::ffi::c_double,
    mut constr_type: ::core::ffi::c_int,
    mut rh: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return add_constraintex(
        lp,
        0 as ::core::ffi::c_int,
        row,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        constr_type,
        rh,
    );
}
#[export_name="honest_lpsolve_str_add_constraint"]
pub unsafe extern "C" fn str_add_constraint(
    mut lp: *mut lprec,
    mut row_string: *mut ::core::ffi::c_char,
    mut constr_type: ::core::ffi::c_int,
    mut rh: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut newp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aRow: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    allocREAL(
        lp,
        &raw mut aRow,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    p = row_string;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        *aRow.offset(i as isize) = native_only!(strtod,p, &raw mut newp);
        if p == newp {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"str_add_constraint: Bad string '%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            (*lp).spx_status = DATAIGNORED;
            break;
        } else {
            p = newp;
            i += 1;
        }
    }
    if (*lp).spx_status != DATAIGNORED {
        status = add_constraint(lp, aRow, constr_type, rh);
    }
    if !(aRow as *mut ::core::ffi::c_void).is_null() {
        free(aRow as *mut ::core::ffi::c_void);
        aRow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return status;
}
#[export_name="honest_lpsolve_del_constraintex"]
pub unsafe extern "C" fn del_constraintex(
    mut lp: *mut lprec,
    mut rowmap: *mut LLrec,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).equalities > 0 as ::core::ffi::c_int {
        i = firstInactiveLink(rowmap);
        while i != 0 as ::core::ffi::c_int {
            if is_constr_type(lp, i, EQ) != 0 {
                (*lp).equalities -= 1;
            }
            i = nextInactiveLink(rowmap, i);
        }
    }
    varmap_delete(
        lp,
        1 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        rowmap,
    );
    shift_rowdata(
        lp,
        1 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        rowmap,
    );
    if (*lp).varmap_locked == 0 {
        presolve_setOrig(lp, (*lp).rows, (*lp).columns);
        if (*lp).names_used != 0 {
            del_varnameex(
                lp,
                (*lp).row_name,
                (*lp).rows,
                (*lp).rowname_hashtab,
                0 as ::core::ffi::c_int,
                rowmap,
            );
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_del_constraint"]
pub unsafe extern "C" fn del_constraint(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut preparecompact: ::core::ffi::c_uchar =
        (rownr < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if preparecompact != 0 {
        rownr = -rownr;
    }
    if rownr < 1 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"del_constraint: Attempt to delete non-existing constraint %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if is_constr_type(lp, rownr, EQ) as ::core::ffi::c_int != 0
        && (*lp).equalities > 0 as ::core::ffi::c_int
    {
        (*lp).equalities -= 1;
    }
    varmap_delete(
        lp,
        if preparecompact as ::core::ffi::c_int != 0 && rownr != 0 as ::core::ffi::c_int {
            -rownr
        } else {
            rownr
        },
        -(1 as ::core::ffi::c_int),
        ::core::ptr::null_mut::<LLrec>(),
    );
    shift_rowdata(
        lp,
        if preparecompact as ::core::ffi::c_int != 0 && rownr != 0 as ::core::ffi::c_int {
            -rownr
        } else {
            rownr
        },
        -(1 as ::core::ffi::c_int),
        ::core::ptr::null_mut::<LLrec>(),
    );
    if (*lp).varmap_locked == 0 {
        presolve_setOrig(lp, (*lp).rows, (*lp).columns);
        if (*lp).names_used != 0 {
            del_varnameex(
                lp,
                (*lp).row_name,
                (*lp).rows,
                (*lp).rowname_hashtab,
                rownr,
                ::core::ptr::null_mut::<LLrec>(),
            );
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_add_lag_con"]
pub unsafe extern "C" fn add_lag_con(
    mut lp: *mut lprec,
    mut row: *mut ::core::ffi::c_double,
    mut con_type: ::core::ffi::c_int,
    mut rhs: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut k: ::core::ffi::c_int = 0;
    let mut sign: ::core::ffi::c_double = 0.;
    if con_type == LE || con_type == EQ {
        sign = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if con_type == GE {
        sign = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    } else {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"add_lag_con: Constraint type %d not implemented\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    inc_lag_space(lp, 1 as ::core::ffi::c_int, FALSE as ::core::ffi::c_uchar);
    k = get_Lrows(lp);
    *(*lp).lag_rhs.offset(k as isize) = rhs * sign;
    mat_appendrow(
        (*lp).matL,
        (*lp).columns,
        row,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        sign,
        TRUE as ::core::ffi::c_uchar,
    );
    *(*lp).lambda.offset(k as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    *(*lp).lag_con_type.offset(k as isize) = con_type;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_str_add_lag_con"]
pub unsafe extern "C" fn str_add_lag_con(
    mut lp: *mut lprec,
    mut row_string: *mut ::core::ffi::c_char,
    mut con_type: ::core::ffi::c_int,
    mut rhs: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut a_row: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    allocREAL(
        lp,
        &raw mut a_row,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    p = row_string;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        *a_row.offset(i as isize) = native_only!(strtod,p, &raw mut new_p);
        if p == new_p {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"str_add_lag_con: Bad string '%s'\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            (*lp).spx_status = DATAIGNORED;
            ret = FALSE as ::core::ffi::c_uchar;
            break;
        } else {
            p = new_p;
            i += 1;
        }
    }
    if (*lp).spx_status != DATAIGNORED {
        ret = add_lag_con(lp, a_row, con_type, rhs);
    }
    if !(a_row as *mut ::core::ffi::c_void).is_null() {
        free(a_row as *mut ::core::ffi::c_void);
        a_row = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return ret;
}
#[export_name="honest_lpsolve_is_splitvar"]
pub unsafe extern "C" fn is_splitvar(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (!(*lp).var_is_free.is_null()
        && *(*lp).var_is_free.offset(colnr as isize) < 0 as ::core::ffi::c_int
        && -*(*lp).var_is_free.offset(colnr as isize) != colnr) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_del_splitvars"]
pub unsafe extern "C" fn del_splitvars(mut lp: *mut lprec) {
    let mut j: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if !(*lp).var_is_free.is_null() {
        j = (*lp).columns;
        while j >= 1 as ::core::ffi::c_int {
            if is_splitvar(lp, j) != 0 {
                jj = (*lp).rows + abs(*(*lp).var_is_free.offset(j as isize));
                i = (*lp).rows + j;
                if *(*lp).is_basic.offset(i as isize) as ::core::ffi::c_int != 0
                    && *(*lp).is_basic.offset(jj as isize) == 0
                {
                    i = findBasisPos(lp, i, ::core::ptr::null_mut::<::core::ffi::c_int>());
                    set_basisvar(lp, i, jj);
                }
                del_column(lp, j);
            }
            j -= 1;
        }
        if !((*lp).var_is_free as *mut ::core::ffi::c_void).is_null() {
            free((*lp).var_is_free as *mut ::core::ffi::c_void);
            (*lp).var_is_free = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
}
#[export_name="honest_lpsolve_set_column"]
pub unsafe extern "C" fn set_column(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return mat_setcol(
        (*lp).matA,
        colnr,
        (*lp).rows,
        column,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        TRUE as ::core::ffi::c_uchar,
        TRUE as ::core::ffi::c_uchar,
    );
}
#[export_name="honest_lpsolve_set_columnex"]
pub unsafe extern "C" fn set_columnex(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut rowno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return mat_setcol(
        (*lp).matA,
        colnr,
        count,
        column,
        rowno,
        TRUE as ::core::ffi::c_uchar,
        TRUE as ::core::ffi::c_uchar,
    );
}
#[export_name="honest_lpsolve_add_columnex"]
pub unsafe extern "C" fn add_columnex(
    mut lp: *mut lprec,
    mut count: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut rowno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if append_columns(lp, 1 as ::core::ffi::c_int) == 0 {
        return status;
    }
    if mat_appendcol(
        (*lp).matA,
        count,
        column,
        rowno,
        1.0f64,
        TRUE as ::core::ffi::c_uchar,
    ) < 0 as ::core::ffi::c_int
    {
        report(
            lp,
            2 as ::core::ffi::c_int,
            b"add_columnex: Data column %d supplied in non-ascending row index order.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        status = TRUE as ::core::ffi::c_uchar;
    }
    if (*lp).varmap_locked == 0 {
        presolve_setOrig(lp, (*lp).rows, (*lp).columns);
    }
    return status;
}
#[export_name="honest_lpsolve_add_column"]
pub unsafe extern "C" fn add_column(
    mut lp: *mut lprec,
    mut column: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    del_splitvars(lp);
    return add_columnex(
        lp,
        (*lp).rows,
        column,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[export_name="honest_lpsolve_str_add_column"]
pub unsafe extern "C" fn str_add_column(
    mut lp: *mut lprec,
    mut col_string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut aCol: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut newp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    allocREAL(
        lp,
        &raw mut aCol,
        (*lp).rows + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    p = col_string;
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        *aCol.offset(i as isize) = native_only!(strtod,p, &raw mut newp);
        if p == newp {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"str_add_column: Bad string '%s'\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            (*lp).spx_status = DATAIGNORED;
            ret = FALSE as ::core::ffi::c_uchar;
            break;
        } else {
            p = newp;
            i += 1;
        }
    }
    if (*lp).spx_status != DATAIGNORED {
        ret = add_column(lp, aCol);
    }
    if !(aCol as *mut ::core::ffi::c_void).is_null() {
        free(aCol as *mut ::core::ffi::c_void);
        aCol = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return ret;
}
#[export_name="honest_lpsolve_del_varnameex"]
pub unsafe extern "C" fn del_varnameex(
    mut lp: *mut lprec,
    mut namelist: *mut *mut hashelem,
    mut items: ::core::ffi::c_int,
    mut ht: *mut hashtable,
    mut varnr: ::core::ffi::c_int,
    mut varmap: *mut LLrec,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if !varmap.is_null() {
        i = firstInactiveLink(varmap);
    } else {
        i = varnr;
    }
    while i > 0 as ::core::ffi::c_int {
        if !(*namelist.offset(i as isize)).is_null() {
            if !(**namelist.offset(i as isize)).name.is_null() {
                drophash((**namelist.offset(i as isize)).name, namelist, ht);
            }
        }
        if !varmap.is_null() {
            i = nextInactiveLink(varmap, i);
        } else {
            i = 0 as ::core::ffi::c_int;
        }
    }
    if !varmap.is_null() {
        i = firstInactiveLink(varmap);
        n = nextActiveLink(varmap, i);
        varnr = i;
    } else {
        i = varnr;
        n = i + 1 as ::core::ffi::c_int;
    }
    while n != 0 as ::core::ffi::c_int {
        let ref mut fresh46 = *namelist.offset(i as isize);
        *fresh46 = *namelist.offset(n as isize);
        if !(*namelist.offset(i as isize)).is_null()
            && (**namelist.offset(i as isize)).index > varnr
        {
            (**namelist.offset(i as isize)).index -= n - i;
        }
        i += 1;
        if !varmap.is_null() {
            n = nextActiveLink(varmap, i);
        } else if n <= items {
            n += 1;
        } else {
            n = 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_del_columnex"]
pub unsafe extern "C" fn del_columnex(
    mut lp: *mut lprec,
    mut colmap: *mut LLrec,
) -> ::core::ffi::c_uchar {
    varmap_delete(
        lp,
        (*lp).rows + 1 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        colmap,
    );
    shift_coldata(
        lp,
        1 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        colmap,
    );
    if (*lp).varmap_locked == 0 {
        presolve_setOrig(lp, (*lp).rows, (*lp).columns);
        if (*lp).names_used != 0 {
            del_varnameex(
                lp,
                (*lp).col_name,
                (*lp).columns,
                (*lp).colname_hashtab,
                0 as ::core::ffi::c_int,
                colmap,
            );
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_del_column"]
pub unsafe extern "C" fn del_column(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut preparecompact: ::core::ffi::c_uchar =
        (colnr < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if preparecompact != 0 {
        colnr = -colnr;
    }
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"del_column: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if !(*lp).var_is_free.is_null()
        && *(*lp).var_is_free.offset(colnr as isize) > 0 as ::core::ffi::c_int
    {
        del_column(lp, *(*lp).var_is_free.offset(colnr as isize));
    }
    varmap_delete(
        lp,
        if preparecompact as ::core::ffi::c_int != 0
            && (*lp).rows + colnr != 0 as ::core::ffi::c_int
        {
            -((*lp).rows + colnr)
        } else {
            (*lp).rows + colnr
        },
        -(1 as ::core::ffi::c_int),
        ::core::ptr::null_mut::<LLrec>(),
    );
    shift_coldata(
        lp,
        if preparecompact as ::core::ffi::c_int != 0 && colnr != 0 as ::core::ffi::c_int {
            -colnr
        } else {
            colnr
        },
        -(1 as ::core::ffi::c_int),
        ::core::ptr::null_mut::<LLrec>(),
    );
    if (*lp).varmap_locked == 0 {
        presolve_setOrig(lp, (*lp).rows, (*lp).columns);
        if (*lp).names_used != 0 {
            del_varnameex(
                lp,
                (*lp).col_name,
                (*lp).columns,
                (*lp).colname_hashtab,
                colnr,
                ::core::ptr::null_mut::<LLrec>(),
            );
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_simplextype"]
pub unsafe extern "C" fn set_simplextype(mut lp: *mut lprec, mut simplextype: ::core::ffi::c_int) {
    (*lp).simplex_strategy = simplextype;
}
#[export_name="honest_lpsolve_get_simplextype"]
pub unsafe extern "C" fn get_simplextype(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).simplex_strategy;
}
#[export_name="honest_lpsolve_set_preferdual"]
pub unsafe extern "C" fn set_preferdual(mut lp: *mut lprec, mut dodual: ::core::ffi::c_uchar) {
    if dodual as ::core::ffi::c_int & TRUE != 0 {
        (*lp).simplex_strategy = SIMPLEX_DUAL_DUAL;
    } else {
        (*lp).simplex_strategy = SIMPLEX_PRIMAL_PRIMAL;
    };
}
#[export_name="honest_lpsolve_set_bounds_tighter"]
pub unsafe extern "C" fn set_bounds_tighter(mut lp: *mut lprec, mut tighten: ::core::ffi::c_uchar) {
    (*lp).tighten_on_set = tighten;
}
#[export_name="honest_lpsolve_get_bounds_tighter"]
pub unsafe extern "C" fn get_bounds_tighter(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*lp).tighten_on_set;
}
#[export_name="honest_lpsolve_set_upbo"]
pub unsafe extern "C" fn set_upbo(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_upbo: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if fabs(value) < (*lp).infinite {
        value = if fabs(value) < (*(*lp).matA).epsvalue {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            value
        };
    }
    value = scaled_value(lp, value, (*lp).rows + colnr);
    if (*lp).tighten_on_set != 0 {
        if value < *(*lp).orig_lowbo.offset(((*lp).rows + colnr) as isize) {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"set_upbo: Upperbound must be >= lowerbound\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
        if value < *(*lp).orig_upbo.offset(((*lp).rows + colnr) as isize) {
            set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
            *(*lp).orig_upbo.offset(((*lp).rows + colnr) as isize) = value;
        }
    } else {
        set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
        if value > (*lp).infinite {
            value = (*lp).infinite;
        }
        *(*lp).orig_upbo.offset(((*lp).rows + colnr) as isize) = value;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_upbo"]
pub unsafe extern "C" fn get_upbo(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_upbo: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    value = *(*lp).orig_upbo.offset(((*lp).rows + colnr) as isize);
    value = unscaled_value(lp, value, (*lp).rows + colnr);
    return value;
}
#[export_name="honest_lpsolve_set_lowbo"]
pub unsafe extern "C" fn set_lowbo(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_lowbo: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if fabs(value) < (*lp).infinite {
        value = if fabs(value) < (*(*lp).matA).epsvalue {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            value
        };
    }
    value = scaled_value(lp, value, (*lp).rows + colnr);
    if (*lp).tighten_on_set != 0 {
        if value > *(*lp).orig_upbo.offset(((*lp).rows + colnr) as isize) {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"set_lowbo: Upper bound must be >= lower bound\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
        if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || value > *(*lp).orig_lowbo.offset(((*lp).rows + colnr) as isize)
        {
            set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
            *(*lp).orig_lowbo.offset(((*lp).rows + colnr) as isize) = value;
        }
    } else {
        set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
        if value < -(*lp).infinite {
            value = -(*lp).infinite;
        }
        *(*lp).orig_lowbo.offset(((*lp).rows + colnr) as isize) = value;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_lowbo"]
pub unsafe extern "C" fn get_lowbo(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_lowbo: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    value = *(*lp).orig_lowbo.offset(((*lp).rows + colnr) as isize);
    value = unscaled_value(lp, value, (*lp).rows + colnr);
    return value;
}
#[export_name="honest_lpsolve_set_bounds"]
pub unsafe extern "C" fn set_bounds(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut lower: ::core::ffi::c_double,
    mut upper: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_bounds: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if fabs(upper - lower) < (*lp).epsvalue {
        if lower < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            lower = upper;
        } else {
            upper = lower;
        }
    } else if lower > upper {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_bounds: Column %d upper bound must be >= lower bound\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    colnr += (*lp).rows;
    if lower < -(*lp).infinite {
        lower = -(*lp).infinite;
    } else if (*lp).scaling_used != 0 {
        lower = scaled_value(lp, lower, colnr);
        lower = if fabs(lower) < (*(*lp).matA).epsvalue {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            lower
        };
    }
    if upper > (*lp).infinite {
        upper = (*lp).infinite;
    } else if (*lp).scaling_used != 0 {
        upper = scaled_value(lp, upper, colnr);
        upper = if fabs(upper) < (*(*lp).matA).epsvalue {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            upper
        };
    }
    *(*lp).orig_lowbo.offset(colnr as isize) = lower;
    *(*lp).orig_upbo.offset(colnr as isize) = upper;
    set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_bounds"]
pub unsafe extern "C" fn get_bounds(
    mut lp: *mut lprec,
    mut column: ::core::ffi::c_int,
    mut lower: *mut ::core::ffi::c_double,
    mut upper: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if column > (*lp).columns || column < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_bounds: Column %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if !lower.is_null() {
        *lower = get_lowbo(lp, column);
    }
    if !upper.is_null() {
        *upper = get_upbo(lp, column);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_int"]
pub unsafe extern "C" fn set_int(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut var_type: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_int: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if *(*lp).var_type.offset(colnr as isize) as ::core::ffi::c_int & ISINTEGER
        != 0 as ::core::ffi::c_int
    {
        (*lp).int_vars -= 1;
        let ref mut fresh49 = *(*lp).var_type.offset(colnr as isize);
        *fresh49 = (*fresh49 as ::core::ffi::c_int & !ISINTEGER) as ::core::ffi::c_uchar;
    }
    if var_type != 0 {
        let ref mut fresh50 = *(*lp).var_type.offset(colnr as isize);
        *fresh50 = (*fresh50 as ::core::ffi::c_int | ISINTEGER) as ::core::ffi::c_uchar;
        (*lp).int_vars += 1;
        if (*lp).columns_scaled as ::core::ffi::c_int != 0 && is_integerscaling(lp) == 0 {
            unscale_columns(lp);
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_int"]
pub unsafe extern "C" fn is_int(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"is_int: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    return (*(*lp).var_type.offset(colnr as isize) as ::core::ffi::c_int & ISINTEGER
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_SOS_var"]
pub unsafe extern "C" fn is_SOS_var(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"is_SOS_var: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    return (*(*lp).var_type.offset(colnr as isize) as ::core::ffi::c_int & ISSOS
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_add_SOS"]
pub unsafe extern "C" fn add_SOS(
    mut lp: *mut lprec,
    mut name: *mut ::core::ffi::c_char,
    mut sostype: ::core::ffi::c_int,
    mut priority: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut sosvars: *mut ::core::ffi::c_int,
    mut weights: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut SOS: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    let mut k: ::core::ffi::c_int = 0;
    if sostype < 1 as ::core::ffi::c_int || count < 0 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"add_SOS: Invalid SOS type definition %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if sostype > 2 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        k = 0 as ::core::ffi::c_int;
        while k < count {
            j = *sosvars.offset(k as isize);
            if is_int(lp, j) == 0 || is_semicont(lp, j) == 0 {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"add_SOS: SOS3+ members all have to be integer or semi-continuous.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_int;
            }
            k += 1;
        }
    }
    if (*lp).SOS.is_null() {
        (*lp).SOS = create_SOSgroup(lp);
    }
    SOS = create_SOSrec((*lp).SOS, name, sostype, priority, count, sosvars, weights);
    k = append_SOSgroup((*lp).SOS, SOS);
    return k;
}
#[export_name="honest_lpsolve_add_GUB"]
pub unsafe extern "C" fn add_GUB(
    mut lp: *mut lprec,
    mut name: *mut ::core::ffi::c_char,
    mut priority: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut gubvars: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut GUB: *mut SOSrec = ::core::ptr::null_mut::<SOSrec>();
    let mut k: ::core::ffi::c_int = 0;
    if (*lp).GUB.is_null() {
        (*lp).GUB = create_SOSgroup(lp);
    }
    GUB = create_SOSrec(
        (*lp).GUB,
        name,
        1 as ::core::ffi::c_int,
        priority,
        count,
        gubvars,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
    (*GUB).isGUB = TRUE as ::core::ffi::c_uchar;
    k = append_SOSgroup((*lp).GUB, GUB);
    return k;
}
#[export_name="honest_lpsolve_set_binary"]
pub unsafe extern "C" fn set_binary(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut must_be_bin: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_binary: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return status;
    }
    status = set_int(lp, colnr, must_be_bin);
    if status as ::core::ffi::c_int != 0 && must_be_bin as ::core::ffi::c_int != 0 {
        status = set_bounds(
            lp,
            colnr,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
    }
    return status;
}
#[export_name="honest_lpsolve_is_binary"]
pub unsafe extern "C" fn is_binary(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"is_binary: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    return (*(*lp).var_type.offset(colnr as isize) as ::core::ffi::c_int & ISINTEGER
        != 0 as ::core::ffi::c_int
        && get_lowbo(lp, colnr) == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && fabs(get_upbo(lp, colnr) - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            < (*lp).epsprimal) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_unbounded"]
pub unsafe extern "C" fn set_unbounded(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_unbounded: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    return set_bounds(lp, colnr, -(*lp).infinite, (*lp).infinite);
}
#[export_name="honest_lpsolve_is_unbounded"]
pub unsafe extern "C" fn is_unbounded(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut test: ::core::ffi::c_uchar = 0;
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"is_unbounded: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    test = is_splitvar(lp, colnr);
    if test == 0 {
        colnr += (*lp).rows;
        test = (*(*lp).orig_lowbo.offset(colnr as isize) <= -(*lp).infinite
            && *(*lp).orig_upbo.offset(colnr as isize) >= (*lp).infinite)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    return test;
}
#[export_name="honest_lpsolve_is_negative"]
pub unsafe extern "C" fn is_negative(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"is_negative: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    colnr += (*lp).rows;
    return (*(*lp).orig_upbo.offset(colnr as isize)
        <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && *(*lp).orig_lowbo.offset(colnr as isize)
            < 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_var_weights"]
pub unsafe extern "C" fn set_var_weights(
    mut lp: *mut lprec,
    mut weights: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if !(*lp).var_priority.is_null() {
        if !((*lp).var_priority as *mut ::core::ffi::c_void).is_null() {
            free((*lp).var_priority as *mut ::core::ffi::c_void);
            (*lp).var_priority = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    if !weights.is_null() {
        let mut n: ::core::ffi::c_int = 0;
        allocINT(
            lp,
            &raw mut (*lp).var_priority,
            (*lp).columns_alloc,
            FALSE as ::core::ffi::c_uchar,
        );
        n = 0 as ::core::ffi::c_int;
        while n < (*lp).columns {
            *(*lp).var_priority.offset(n as isize) = n + 1 as ::core::ffi::c_int;
            n += 1;
        }
        n = sortByREAL(
            (*lp).var_priority,
            weights,
            (*lp).columns,
            0 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_var_priority"]
pub unsafe extern "C" fn set_var_priority(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut status: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    if is_bb_mode(lp, NODE_AUTOORDER) as ::core::ffi::c_int != 0
        && (*lp).var_priority.is_null()
        && SOS_count(lp) == 0 as ::core::ffi::c_int
    {
        let mut rcost: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut colorder: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        allocINT(
            lp,
            &raw mut colorder,
            (*lp).columns + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        *colorder.offset(0 as ::core::ffi::c_int as isize) = (*lp).columns;
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            *colorder.offset(j as isize) = (*lp).rows + j;
            j += 1;
        }
        i = getMDO(
            lp,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            colorder,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            FALSE as ::core::ffi::c_uchar,
        );
        allocREAL(
            lp,
            &raw mut rcost,
            (*lp).columns + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        j = (*lp).columns;
        while j > 0 as ::core::ffi::c_int {
            i = *colorder.offset(j as isize) - (*lp).rows;
            *rcost.offset(i as isize) = -j as ::core::ffi::c_double;
            j -= 1;
        }
        set_var_weights(lp, rcost.offset(1 as ::core::ffi::c_int as isize));
        if !(rcost as *mut ::core::ffi::c_void).is_null() {
            free(rcost as *mut ::core::ffi::c_void);
            rcost = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !(colorder as *mut ::core::ffi::c_void).is_null() {
            free(colorder as *mut ::core::ffi::c_void);
            colorder = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        status = TRUE as ::core::ffi::c_uchar;
    }
    return status;
}
#[export_name="honest_lpsolve_get_var_priority"]
pub unsafe extern "C" fn get_var_priority(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_var_priority: Column %d out of range\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*lp).var_priority.is_null() {
        return colnr;
    } else {
        return *(*lp)
            .var_priority
            .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    };
}
#[export_name="honest_lpsolve_set_semicont"]
pub unsafe extern "C" fn set_semicont(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut must_be_sc: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_semicont: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if *(*lp).sc_lobound.offset(colnr as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        (*lp).sc_vars -= 1;
        let ref mut fresh47 = *(*lp).var_type.offset(colnr as isize);
        *fresh47 = (*fresh47 as ::core::ffi::c_int & !ISSEMI) as ::core::ffi::c_uchar;
    }
    *(*lp).sc_lobound.offset(colnr as isize) = must_be_sc as ::core::ffi::c_double;
    if must_be_sc != 0 {
        let ref mut fresh48 = *(*lp).var_type.offset(colnr as isize);
        *fresh48 = (*fresh48 as ::core::ffi::c_int | ISSEMI) as ::core::ffi::c_uchar;
        (*lp).sc_vars += 1;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_semicont"]
pub unsafe extern "C" fn is_semicont(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"is_semicont: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    return (*(*lp).var_type.offset(colnr as isize) as ::core::ffi::c_int & ISSEMI
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_rh"]
pub unsafe extern "C" fn set_rh(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if rownr > (*lp).rows || rownr < 0 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_rh: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if rownr == 0 as ::core::ffi::c_int && is_maxim(lp) == 0
        || rownr > 0 as ::core::ffi::c_int && is_chsign(lp, rownr) as ::core::ffi::c_int != 0
    {
        value = if fabs(value) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            -value
        };
    }
    if fabs(value) > (*lp).infinite {
        if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            value = -(*lp).infinite;
        } else {
            value = (*lp).infinite;
        }
    } else {
        value = if fabs(value) < (*(*lp).matA).epsvalue {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            value
        };
    }
    value = scaled_value(lp, value, rownr);
    *(*lp).orig_rhs.offset(rownr as isize) = value;
    set_action(&raw mut (*lp).spx_action, ACTION_RECOMPUTE);
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_rh"]
pub unsafe extern "C" fn get_rh(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    if rownr > (*lp).rows || rownr < 0 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_rh: Row %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0.0f64;
    }
    value = *(*lp).orig_rhs.offset(rownr as isize);
    if rownr == 0 as ::core::ffi::c_int && is_maxim(lp) == 0
        || rownr > 0 as ::core::ffi::c_int && is_chsign(lp, rownr) as ::core::ffi::c_int != 0
    {
        value = if fabs(value) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            -value
        };
    }
    value = unscaled_value(lp, value, rownr);
    return value;
}
#[export_name="honest_lpsolve_get_rh_upper"]
pub unsafe extern "C" fn get_rh_upper(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    let mut valueR: ::core::ffi::c_double = 0.;
    value = *(*lp).orig_rhs.offset(rownr as isize);
    if is_chsign(lp, rownr) != 0 {
        valueR = *(*lp).orig_upbo.offset(rownr as isize);
        if is_infinite(lp, valueR) != 0 {
            return (*lp).infinite;
        }
        value = if fabs(value) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            -value
        };
        value += valueR;
    }
    value = unscaled_value(lp, value, rownr);
    return value;
}
#[export_name="honest_lpsolve_get_rh_lower"]
pub unsafe extern "C" fn get_rh_lower(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    let mut valueR: ::core::ffi::c_double = 0.;
    value = *(*lp).orig_rhs.offset(rownr as isize);
    if is_chsign(lp, rownr) != 0 {
        value = if fabs(value) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            -value
        };
    } else {
        valueR = *(*lp).orig_upbo.offset(rownr as isize);
        if is_infinite(lp, valueR) != 0 {
            return -(*lp).infinite;
        }
        value -= valueR;
    }
    value = unscaled_value(lp, value, rownr);
    return value;
}
#[export_name="honest_lpsolve_set_rh_upper"]
pub unsafe extern "C" fn set_rh_upper(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if rownr > (*lp).rows || rownr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_rh_upper: Row %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    value = scaled_value(lp, value, rownr);
    if is_chsign(lp, rownr) != 0 {
        if is_infinite(lp, value) != 0 {
            *(*lp).orig_upbo.offset(rownr as isize) = (*lp).infinite;
        } else {
            *(*lp).orig_upbo.offset(rownr as isize) =
                if fabs(value + *(*lp).orig_rhs.offset(rownr as isize)) < (*lp).epsvalue {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    value + *(*lp).orig_rhs.offset(rownr as isize)
                };
        }
    } else {
        if is_infinite(lp, *(*lp).orig_upbo.offset(rownr as isize)) == 0 {
            *(*lp).orig_upbo.offset(rownr as isize) -=
                *(*lp).orig_rhs.offset(rownr as isize) - value;
            if fabs(*(*lp).orig_upbo.offset(rownr as isize)) < (*lp).epsvalue {
                *(*lp).orig_upbo.offset(rownr as isize) =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if *(*lp).orig_upbo.offset(rownr as isize)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"set_rh_upper: Negative bound set for constraint %d made 0\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                *(*lp).orig_upbo.offset(rownr as isize) =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        *(*lp).orig_rhs.offset(rownr as isize) = value;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_rh_lower"]
pub unsafe extern "C" fn set_rh_lower(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if rownr > (*lp).rows || rownr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_rh_lower: Row %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    value = scaled_value(lp, value, rownr);
    if is_chsign(lp, rownr) == 0 {
        if is_infinite(lp, value) != 0 {
            *(*lp).orig_upbo.offset(rownr as isize) = (*lp).infinite;
        } else {
            *(*lp).orig_upbo.offset(rownr as isize) =
                if fabs(*(*lp).orig_rhs.offset(rownr as isize) - value) < (*lp).epsvalue {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    *(*lp).orig_rhs.offset(rownr as isize) - value
                };
        }
    } else {
        value = if fabs(value) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            -value
        };
        if is_infinite(lp, *(*lp).orig_upbo.offset(rownr as isize)) == 0 {
            *(*lp).orig_upbo.offset(rownr as isize) -=
                *(*lp).orig_rhs.offset(rownr as isize) - value;
            if fabs(*(*lp).orig_upbo.offset(rownr as isize)) < (*lp).epsvalue {
                *(*lp).orig_upbo.offset(rownr as isize) =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if *(*lp).orig_upbo.offset(rownr as isize)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"set_rh_lower: Negative bound set for constraint %d made 0\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                *(*lp).orig_upbo.offset(rownr as isize) =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        *(*lp).orig_rhs.offset(rownr as isize) = value;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_rh_range"]
pub unsafe extern "C" fn set_rh_range(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut deltavalue: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if rownr > (*lp).rows || rownr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_rh_range: Row %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    deltavalue = scaled_value(lp, deltavalue, rownr);
    if deltavalue > (*lp).infinite {
        deltavalue = (*lp).infinite;
    } else if deltavalue < -(*lp).infinite {
        deltavalue = -(*lp).infinite;
    } else {
        deltavalue = if fabs(deltavalue) < (*(*lp).matA).epsvalue {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            deltavalue
        };
    }
    if fabs(deltavalue) < (*lp).epsprimal {
        set_constr_type(lp, rownr, EQ);
    } else if is_constr_type(lp, rownr, EQ) != 0 {
        if deltavalue > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            set_constr_type(lp, rownr, GE);
        } else {
            set_constr_type(lp, rownr, LE);
        }
        *(*lp).orig_upbo.offset(rownr as isize) = fabs(deltavalue);
    } else {
        *(*lp).orig_upbo.offset(rownr as isize) = fabs(deltavalue);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_rh_range"]
pub unsafe extern "C" fn get_rh_range(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if rownr > (*lp).rows || rownr < 0 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_rh_range: row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if *(*lp).orig_upbo.offset(rownr as isize) >= (*lp).infinite {
        return *(*lp).orig_upbo.offset(rownr as isize);
    } else {
        return unscaled_value(lp, *(*lp).orig_upbo.offset(rownr as isize), rownr);
    };
}
#[export_name="honest_lpsolve_set_rh_vec"]
pub unsafe extern "C" fn set_rh_vec(mut lp: *mut lprec, mut rh: *mut ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut rhi: ::core::ffi::c_double = 0.;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        rhi = *rh.offset(i as isize);
        rhi = if fabs(rhi) < (*(*lp).matA).epsvalue {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            rhi
        };
        *(*lp).orig_rhs.offset(i as isize) = if is_chsign(lp, i) as ::core::ffi::c_int != 0
            && scaled_value(lp, rhi, i) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -scaled_value(lp, rhi, i)
        } else {
            scaled_value(lp, rhi, i)
        };
        i += 1;
    }
    set_action(&raw mut (*lp).spx_action, ACTION_RECOMPUTE);
}
#[export_name="honest_lpsolve_str_set_rh_vec"]
pub unsafe extern "C" fn str_set_rh_vec(
    mut lp: *mut lprec,
    mut rh_string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut newrh: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut newp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    allocREAL(
        lp,
        &raw mut newrh,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    p = rh_string;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        *newrh.offset(i as isize) = native_only!(strtod,p, &raw mut newp);
        if p == newp {
            report(
                lp,
                3 as ::core::ffi::c_int,
                b"str_set_rh_vec: Bad string %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            (*lp).spx_status = DATAIGNORED;
            ret = FALSE as ::core::ffi::c_uchar;
            break;
        } else {
            p = newp;
            i += 1;
        }
    }
    if !((*lp).spx_status == DATAIGNORED) {
        set_rh_vec(lp, newrh);
    }
    if !(newrh as *mut ::core::ffi::c_void).is_null() {
        free(newrh as *mut ::core::ffi::c_void);
        newrh = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return ret;
}
#[export_name="honest_lpsolve_set_sense"]
pub unsafe extern "C" fn set_sense(mut lp: *mut lprec, mut maximize: ::core::ffi::c_uchar) {
    maximize =
        (maximize as ::core::ffi::c_int != FALSE) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if is_maxim(lp) as ::core::ffi::c_int != maximize as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        if is_infinite(lp, (*lp).bb_heuristicOF) != 0 {
            (*lp).bb_heuristicOF = if maximize as ::core::ffi::c_int != 0
                && (*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -(*lp).infinite
            } else {
                (*lp).infinite
            };
        }
        if is_infinite(lp, (*lp).bb_breakOF) != 0 {
            (*lp).bb_breakOF = if maximize as ::core::ffi::c_int != 0
                && -(*lp).infinite != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                --(*lp).infinite
            } else {
                -(*lp).infinite
            };
        }
        *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize) =
            if fabs(*(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize))
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                -*(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize)
            };
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).columns {
            *(*lp).orig_obj.offset(i as isize) = if fabs(*(*lp).orig_obj.offset(i as isize))
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                -*(*lp).orig_obj.offset(i as isize)
            };
            i += 1;
        }
        set_action(
            &raw mut (*lp).spx_action,
            ACTION_REINVERT | ACTION_RECOMPUTE,
        );
    }
    if maximize != 0 {
        *(*lp).row_type.offset(0 as ::core::ffi::c_int as isize) = ROWTYPE_OFMAX;
    } else {
        *(*lp).row_type.offset(0 as ::core::ffi::c_int as isize) = ROWTYPE_OFMIN;
    };
}
#[export_name="honest_lpsolve_set_maxim"]
pub unsafe extern "C" fn set_maxim(mut lp: *mut lprec) {
    set_sense(lp, TRUE as ::core::ffi::c_uchar);
}
#[export_name="honest_lpsolve_set_minim"]
pub unsafe extern "C" fn set_minim(mut lp: *mut lprec) {
    set_sense(lp, FALSE as ::core::ffi::c_uchar);
}
#[export_name="honest_lpsolve_is_maxim"]
pub unsafe extern "C" fn is_maxim(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (!(*lp).row_type.is_null()
        && *(*lp).row_type.offset(0 as ::core::ffi::c_int as isize) & ROWTYPE_CHSIGN == ROWTYPE_GE)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_constr_type"]
pub unsafe extern "C" fn set_constr_type(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut con_type: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut oldchsign: ::core::ffi::c_uchar = 0;
    if rownr > (*lp).rows + 1 as ::core::ffi::c_int || rownr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_constr_type: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if rownr > (*lp).rows && append_rows(lp, rownr - (*lp).rows) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if is_constr_type(lp, rownr, EQ) != 0 {
        (*lp).equalities -= 1;
    }
    if con_type & ROWTYPE_CONSTRAINT == EQ {
        (*lp).equalities += 1;
        *(*lp).orig_upbo.offset(rownr as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if con_type & LE > 0 as ::core::ffi::c_int
        || con_type & GE > 0 as ::core::ffi::c_int
        || con_type == FR
    {
        *(*lp).orig_upbo.offset(rownr as isize) = (*lp).infinite;
    } else {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_constr_type: Constraint type %d not implemented (row %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    oldchsign = is_chsign(lp, rownr);
    if con_type == FR {
        *(*lp).row_type.offset(rownr as isize) = LE;
    } else {
        *(*lp).row_type.offset(rownr as isize) = con_type;
    }
    if oldchsign as ::core::ffi::c_int != is_chsign(lp, rownr) as ::core::ffi::c_int {
        let mut mat: *mut MATrec = (*lp).matA;
        if (*mat).is_roworder != 0 {
            mat_multcol(
                mat,
                rownr,
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                FALSE as ::core::ffi::c_uchar,
            );
        } else {
            mat_multrow(
                mat,
                rownr,
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
        }
        if *(*lp).orig_rhs.offset(rownr as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *(*lp).orig_rhs.offset(rownr as isize) *=
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        }
        set_action(&raw mut (*lp).spx_action, ACTION_RECOMPUTE);
    }
    if con_type == FR {
        *(*lp).orig_rhs.offset(rownr as isize) = (*lp).infinite;
    }
    set_action(&raw mut (*lp).spx_action, ACTION_REINVERT);
    (*lp).basis_valid = FALSE as ::core::ffi::c_uchar;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_chsign"]
pub unsafe extern "C" fn is_chsign(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (*(*lp).row_type.offset(rownr as isize) & ROWTYPE_CONSTRAINT == ROWTYPE_CHSIGN)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_constr_type"]
pub unsafe extern "C" fn is_constr_type(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"is_constr_type: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    return (*(*lp).row_type.offset(rownr as isize) & ROWTYPE_CONSTRAINT == mask)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_constr_type"]
pub unsafe extern "C" fn get_constr_type(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_constr_type: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return *(*lp).row_type.offset(rownr as isize);
}
#[export_name="honest_lpsolve_get_constr_value"]
pub unsafe extern "C" fn get_constr_value(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
    mut primsolution: *mut ::core::ffi::c_double,
    mut nzindex: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0.0f64;
    let mut mat: *mut MATrec = (*lp).matA;
    if rownr < 0 as ::core::ffi::c_int || rownr > get_Nrows(lp) {
        return value;
    }
    if mat_validate(mat) == 0
        || primsolution.is_null() && (*lp).solvecount == 0 as ::core::ffi::c_int
    {
        return value;
    }
    i = get_Ncolumns(lp);
    if !primsolution.is_null()
        && nzindex.is_null()
        && (count <= 0 as ::core::ffi::c_int || count > i)
    {
        count = i;
    }
    if primsolution.is_null() {
        get_ptr_variables(lp, &raw mut primsolution);
        primsolution = primsolution.offset(-1);
        nzindex = ::core::ptr::null_mut::<::core::ffi::c_int>();
        count = i;
    }
    if rownr == 0 as ::core::ffi::c_int {
        value += get_rh(lp, 0 as ::core::ffi::c_int);
        if !nzindex.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < count {
                value += get_mat(lp, 0 as ::core::ffi::c_int, *nzindex.offset(i as isize))
                    * *primsolution.offset(i as isize);
                i += 1;
            }
        } else {
            i = 1 as ::core::ffi::c_int;
            while i <= count {
                value += get_mat(lp, 0 as ::core::ffi::c_int, i) * *primsolution.offset(i as isize);
                i += 1;
            }
        }
    } else if !nzindex.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < count {
            value +=
                get_mat(lp, rownr, *nzindex.offset(i as isize)) * *primsolution.offset(i as isize);
            i += 1;
        }
    } else {
        let mut j: ::core::ffi::c_int = 0;
        i = *(*mat)
            .row_end
            .offset((rownr - 1 as ::core::ffi::c_int) as isize);
        while i < *(*mat).row_end.offset(rownr as isize) {
            j = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(i as isize) as isize);
            value += unscaled_mat(
                lp,
                *(*mat)
                    .col_mat_value
                    .offset(*(*mat).row_mat.offset(i as isize) as isize),
                rownr,
                j,
            ) * *primsolution.offset(j as isize);
            i += 1;
        }
        value = if is_chsign(lp, rownr) as ::core::ffi::c_int != 0
            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -value
        } else {
            value
        };
    }
    return value;
}
#[export_name="honest_lpsolve_get_str_constr_class"]
pub unsafe extern "C" fn get_str_constr_class(
    mut lp: *mut lprec,
    mut con_class: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    match con_class {
        ROWCLASS_Unknown => {
            return b"Unknown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_Objective => {
            return b"Objective\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_GeneralREAL => {
            return b"General LPSREAL\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_GeneralMIP => {
            return b"General MIP\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_GeneralINT => {
            return b"General INT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_GeneralBIN => {
            return b"General BIN\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_KnapsackINT => {
            return b"Knapsack INT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_KnapsackBIN => {
            return b"Knapsack BIN\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_SetPacking => {
            return b"Set packing\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_SetCover => {
            return b"Set cover\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        ROWCLASS_GUB => {
            return b"GUB\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Error\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
#[export_name="honest_lpsolve_get_str_constr_type"]
pub unsafe extern "C" fn get_str_constr_type(
    mut lp: *mut lprec,
    mut con_type: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    match con_type {
        FR => {
            return b"FR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        LE => {
            return b"LE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        GE => {
            return b"GE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        EQ => {
            return b"EQ\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Error\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
#[export_name="honest_lpsolve_get_constr_class"]
pub unsafe extern "C" fn get_constr_class(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut aBIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aINT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aREAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut xBIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut xINT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut xREAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut elmend: ::core::ffi::c_int = 0;
    let mut nelm: ::core::ffi::c_int = 0;
    let mut chsign: ::core::ffi::c_uchar = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    if rownr < 1 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_constr_class: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    mat_validate(mat);
    if rownr == 0 as ::core::ffi::c_int {
        elmnr = 1 as ::core::ffi::c_int;
        elmend = (*lp).columns;
        nelm = 0 as ::core::ffi::c_int;
    } else {
        elmnr = *(*mat)
            .row_end
            .offset((rownr - 1 as ::core::ffi::c_int) as isize);
        elmend = *(*mat).row_end.offset(rownr as isize);
        nelm = elmend - elmnr;
    }
    chsign = is_chsign(lp, rownr);
    let mut current_block_30: u64;
    while elmnr < elmend {
        if rownr == 0 as ::core::ffi::c_int {
            a = *(*lp).orig_obj.offset(elmnr as isize);
            if a == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                current_block_30 = 2979737022853876585;
            } else {
                j = elmnr;
                current_block_30 = 2370887241019905314;
            }
        } else {
            j = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(elmnr as isize) as isize);
            a = *(*mat)
                .col_mat_value
                .offset(*(*mat).row_mat.offset(elmnr as isize) as isize);
            current_block_30 = 2370887241019905314;
        }
        match current_block_30 {
            2370887241019905314 => {
                a = unscaled_mat(
                    lp,
                    if chsign as ::core::ffi::c_int != 0
                        && a != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -a
                    } else {
                        a
                    },
                    rownr,
                    j,
                );
                if is_binary(lp, j) != 0 {
                    xBIN += 1;
                } else if get_lowbo(lp, j) >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && is_int(lp, j) as ::core::ffi::c_int != 0
                {
                    xINT += 1;
                } else {
                    xREAL += 1;
                }
                if fabs(a - 1.0f64) < (*lp).epsvalue {
                    aBIN += 1;
                } else if a > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && fabs(floor(a + (*lp).epsvalue) - a) < (*lp).epsvalue
                {
                    aINT += 1;
                } else {
                    aREAL += 1;
                }
            }
            _ => {}
        }
        elmnr += 1;
    }
    if rownr == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    j = get_constr_type(lp, rownr);
    a = get_rh(lp, rownr);
    if aBIN == nelm && xBIN == nelm && a >= 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        if a > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            j = ROWCLASS_KnapsackBIN;
        } else if j == EQ {
            j = ROWCLASS_GUB;
        } else if j == LE {
            j = ROWCLASS_SetCover;
        } else {
            j = ROWCLASS_SetPacking;
        }
    } else if aINT == nelm && xINT == nelm && a >= 1 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        j = ROWCLASS_KnapsackINT;
    } else if xBIN == nelm {
        j = ROWCLASS_GeneralBIN;
    } else if xINT == nelm {
        j = ROWCLASS_GeneralINT;
    } else if xREAL > 0 as ::core::ffi::c_int && xINT + xBIN > 0 as ::core::ffi::c_int {
        j = ROWCLASS_GeneralMIP;
    } else {
        j = ROWCLASS_GeneralREAL;
    }
    return j;
}
#[export_name="honest_lpsolve_get_mat"]
pub unsafe extern "C" fn get_mat(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut colnr1: ::core::ffi::c_int = colnr;
    let mut rownr1: ::core::ffi::c_int = rownr;
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_mat: Row %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if colnr < 1 as ::core::ffi::c_int || colnr > (*lp).columns {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_mat: Column %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if rownr == 0 as ::core::ffi::c_int {
        value = *(*lp).orig_obj.offset(colnr as isize);
        value = if is_chsign(lp, rownr) as ::core::ffi::c_int != 0
            && value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -value
        } else {
            value
        };
        value = unscaled_mat(lp, value, rownr, colnr);
    } else {
        if (*(*lp).matA).is_roworder != 0 {
            swapINT(&raw mut colnr1, &raw mut rownr1);
        }
        elmnr = mat_findelm((*lp).matA, rownr1, colnr1);
        if elmnr >= 0 as ::core::ffi::c_int {
            let mut mat: *mut MATrec = (*lp).matA;
            value = if is_chsign(lp, rownr) as ::core::ffi::c_int != 0
                && *(*mat).col_mat_value.offset(elmnr as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -*(*mat).col_mat_value.offset(elmnr as isize)
            } else {
                *(*mat).col_mat_value.offset(elmnr as isize)
            };
            value = unscaled_mat(lp, value, rownr, colnr);
        } else {
            value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    return value;
}
#[export_name="honest_lpsolve_get_mat_byindex"]
pub unsafe extern "C" fn get_mat_byindex(
    mut lp: *mut lprec,
    mut matindex: ::core::ffi::c_int,
    mut isrow: ::core::ffi::c_uchar,
    mut adjustsign: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut result: ::core::ffi::c_double = 0.;
    mat_get_data(
        lp,
        matindex,
        isrow,
        &raw mut rownr,
        &raw mut colnr,
        &raw mut value,
    );
    if adjustsign != 0 {
        result = *value
            * (if is_chsign(lp, *rownr) as ::core::ffi::c_int != 0 {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_double;
    } else {
        result = *value;
    }
    if (*lp).scaling_used != 0 {
        return unscaled_mat(lp, result, *rownr, *colnr);
    } else {
        return result;
    };
}
unsafe extern "C" fn mat_getrow(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut isnz: ::core::ffi::c_uchar = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut countnz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut a: ::core::ffi::c_double = 0.;
    if rownr == 0 as ::core::ffi::c_int || mat_validate((*lp).matA) == 0 {
        j = 1 as ::core::ffi::c_int;
        while j <= (*lp).columns {
            a = get_mat(lp, rownr, j);
            isnz = (a != 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
            if colno.is_null() {
                *row.offset(j as isize) = a;
            } else if isnz != 0 {
                *row.offset(countnz as isize) = a;
                *colno.offset(countnz as isize) = j;
            }
            if isnz != 0 {
                countnz += 1;
            }
            j += 1;
        }
    } else {
        let mut chsign: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
        let mut ie: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut mat: *mut MATrec = (*lp).matA;
        if colno.is_null() {
            memset(
                row as *mut ::core::ffi::c_void,
                '\0' as i32,
                (((*lp).columns + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
        if (*mat).is_roworder != 0 {
            a = get_mat(lp, 0 as ::core::ffi::c_int, rownr);
            if colno.is_null() {
                *row.offset(countnz as isize) = a;
                if a != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    countnz += 1;
                }
            } else if a != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *row.offset(countnz as isize) = a;
                *colno.offset(countnz as isize) = 0 as ::core::ffi::c_int;
                countnz += 1;
            }
        }
        i = *(*mat)
            .row_end
            .offset((rownr - 1 as ::core::ffi::c_int) as isize);
        ie = *(*mat).row_end.offset(rownr as isize);
        if (*(*lp).matA).is_roworder == 0 {
            chsign = is_chsign(lp, rownr);
        }
        while i < ie {
            j = *(*mat)
                .col_mat_colnr
                .offset(*(*mat).row_mat.offset(i as isize) as isize);
            a = get_mat_byindex(
                lp,
                i,
                TRUE as ::core::ffi::c_uchar,
                FALSE as ::core::ffi::c_uchar,
            );
            if (*(*lp).matA).is_roworder != 0 {
                chsign = is_chsign(lp, j);
            }
            a = if chsign as ::core::ffi::c_int != 0
                && a != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -a
            } else {
                a
            };
            if colno.is_null() {
                *row.offset(j as isize) = a;
            } else {
                *row.offset(countnz as isize) = a;
                *colno.offset(countnz as isize) = j;
            }
            countnz += 1;
            i += 1;
        }
    }
    return countnz;
}
unsafe extern "C" fn mat_getcolumn(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut nzrow: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut hold: ::core::ffi::c_double = 0.;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut mat: *mut MATrec = (*lp).matA;
    if nzrow.is_null() {
        memset(
            column as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    if (*mat).is_roworder == 0 {
        hold = get_mat(lp, 0 as ::core::ffi::c_int, colnr);
        if nzrow.is_null() {
            *column.offset(n as isize) = hold;
            if hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                n += 1;
            }
        } else if hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *column.offset(n as isize) = hold;
            *nzrow.offset(n as isize) = 0 as ::core::ffi::c_int;
            n += 1;
        }
    }
    i = *(*(*lp).matA)
        .col_end
        .offset((colnr - 1 as ::core::ffi::c_int) as isize);
    ie = *(*(*lp).matA).col_end.offset(colnr as isize);
    if nzrow.is_null() {
        n += ie - i;
    }
    rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
    value = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
    while i < ie {
        ii = *rownr;
        hold = if is_chsign(
            lp,
            (if (*mat).is_roworder as ::core::ffi::c_int != 0 {
                colnr
            } else {
                ii
            }),
        ) as ::core::ffi::c_int
            != 0
            && *value != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -*value
        } else {
            *value
        };
        hold = unscaled_mat(lp, hold, ii, colnr);
        if nzrow.is_null() {
            *column.offset(ii as isize) = hold;
        } else if hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *column.offset(n as isize) = hold;
            *nzrow.offset(n as isize) = ii;
            n += 1;
        }
        i += 1;
        rownr = rownr.offset(matRowColStep as isize);
        value = value.offset(matValueStep as isize);
    }
    return n;
}
#[export_name="honest_lpsolve_get_columnex"]
pub unsafe extern "C" fn get_columnex(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut nzrow: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if colnr > (*lp).columns || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_columnex: Column %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*lp).matA).is_roworder != 0 {
        return mat_getrow(lp, colnr, column, nzrow);
    } else {
        return mat_getcolumn(lp, colnr, column, nzrow);
    };
}
#[export_name="honest_lpsolve_get_column"]
pub unsafe extern "C" fn get_column(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return (get_columnex(
        lp,
        colnr,
        column,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_rowex"]
pub unsafe extern "C" fn get_rowex(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_rowex: Row %d out of range\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if rownr != 0 as ::core::ffi::c_int && (*(*lp).matA).is_roworder as ::core::ffi::c_int != 0 {
        return mat_getcolumn(lp, rownr, row, colno);
    } else {
        return mat_getrow(lp, rownr, row, colno);
    };
}
#[export_name="honest_lpsolve_get_row"]
pub unsafe extern "C" fn get_row(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut row: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    return (get_rowex(
        lp,
        rownr,
        row,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_OF_override"]
pub unsafe extern "C" fn set_OF_override(
    mut lp: *mut lprec,
    mut ofVector: *mut ::core::ffi::c_double,
) {
    (*lp).obj = ofVector;
}
#[export_name="honest_lpsolve_modifyOF1"]
pub unsafe extern "C" fn modifyOF1(
    mut lp: *mut lprec,
    mut index: ::core::ffi::c_int,
    mut ofValue: *mut ::core::ffi::c_double,
    mut mult: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut accept: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if (*lp).simplex_mode & SIMPLEX_Phase1_PRIMAL != 0 as ::core::ffi::c_int
        && abs((*lp).P1extraDim) > 0 as ::core::ffi::c_int
    {
        if index <= (*lp).sum - (*lp).P1extraDim
            || mult == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            if mult == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                || (*lp).bigM == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                accept = FALSE as ::core::ffi::c_uchar;
            } else {
                *ofValue /= (*lp).bigM;
            }
        }
    } else if (*lp).simplex_mode & SIMPLEX_Phase1_DUAL != 0 as ::core::ffi::c_int
        && index > (*lp).rows
    {
        if (*lp).P1extraVal != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && *(*lp).orig_obj.offset((index - (*lp).rows) as isize)
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *ofValue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *ofValue -= (*lp).P1extraVal;
        }
    }
    if accept != 0 {
        *ofValue *= mult;
        if fabs(*ofValue) < (*lp).epsmachine {
            *ofValue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            accept = FALSE as ::core::ffi::c_uchar;
        }
    } else {
        *ofValue = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    return accept;
}
#[export_name="honest_lpsolve_set_OF_p1extra"]
pub unsafe extern "C" fn set_OF_p1extra(mut lp: *mut lprec, mut p1extra: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if (*lp).spx_trace != 0 {
        report(
            lp,
            5 as ::core::ffi::c_int,
            b"set_OF_p1extra: Set dual objective offset to %g at iter %.0f.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    (*lp).P1extraVal = p1extra;
    if (*lp).obj.is_null() {
        allocREAL(
            lp,
            &raw mut (*lp).obj,
            (*lp).columns_alloc + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    i = 1 as ::core::ffi::c_int;
    value = (*lp).obj.offset(1 as ::core::ffi::c_int as isize);
    while i <= (*lp).columns {
        *value = *(*lp).orig_obj.offset(i as isize);
        modifyOF1(lp, (*lp).rows + i, value, 1.0f64);
        i += 1;
        value = value.offset(1);
    }
}
#[export_name="honest_lpsolve_unset_OF_p1extra"]
pub unsafe extern "C" fn unset_OF_p1extra(mut lp: *mut lprec) {
    (*lp).P1extraVal = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if !((*lp).obj as *mut ::core::ffi::c_void).is_null() {
        free((*lp).obj as *mut ::core::ffi::c_void);
        (*lp).obj = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
}
#[export_name="honest_lpsolve_get_OF_active"]
pub unsafe extern "C" fn get_OF_active(
    mut lp: *mut lprec,
    mut varnr: ::core::ffi::c_int,
    mut mult: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut colnr: ::core::ffi::c_int = varnr - (*lp).rows;
    let mut holdOF: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if (*lp).obj.is_null() {
        if colnr > 0 as ::core::ffi::c_int {
            holdOF = *(*lp).orig_obj.offset(colnr as isize);
        }
        modifyOF1(lp, varnr, &raw mut holdOF, mult);
    } else if colnr > 0 as ::core::ffi::c_int {
        holdOF = *(*lp).obj.offset(colnr as isize) * mult;
    }
    return holdOF;
}
#[export_name="honest_lpsolve_is_OF_nz"]
pub unsafe extern "C" fn is_OF_nz(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    return (*(*lp).orig_obj.offset(colnr as isize)
        != 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_singleton_column"]
pub unsafe extern "C" fn singleton_column(
    mut lp: *mut lprec,
    mut row_nr: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut nzlist: *mut ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
    mut maxabs: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nz: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if nzlist.is_null() {
        memset(
            column as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        *column.offset(row_nr as isize) = value;
    } else {
        *column.offset(nz as isize) = value;
        *nzlist.offset(nz as isize) = row_nr;
    }
    if !maxabs.is_null() {
        *maxabs = row_nr;
    }
    return nz;
}
#[export_name="honest_lpsolve_expand_column"]
pub unsafe extern "C" fn expand_column(
    mut lp: *mut lprec,
    mut col_nr: ::core::ffi::c_int,
    mut column: *mut ::core::ffi::c_double,
    mut nzlist: *mut ::core::ffi::c_int,
    mut mult: ::core::ffi::c_double,
    mut maxabs: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut maxidx: ::core::ffi::c_int = 0;
    let mut nzcount: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0.;
    let mut maxval: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    maxval = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    maxidx = -(1 as ::core::ffi::c_int);
    if nzlist.is_null() {
        memset(
            column as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
        i = *(*mat)
            .col_end
            .offset((col_nr - 1 as ::core::ffi::c_int) as isize);
        ie = *(*mat).col_end.offset(col_nr as isize);
        matRownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
        matValue = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
        nzcount = i;
        while i < ie {
            j = *matRownr;
            value = *matValue;
            if j > 0 as ::core::ffi::c_int {
                value *= mult;
                if fabs(value) > maxval {
                    maxval = fabs(value);
                    maxidx = j;
                }
            }
            *column.offset(j as isize) = value;
            i += 1;
            matRownr = matRownr.offset(matRowColStep as isize);
            matValue = matValue.offset(matValueStep as isize);
        }
        nzcount = i - nzcount;
        if (*lp).obj_in_basis != 0 {
            *column.offset(0 as ::core::ffi::c_int as isize) =
                get_OF_active(lp, (*lp).rows + col_nr, mult);
            if *column.offset(0 as ::core::ffi::c_int as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                nzcount += 1;
            }
        }
    } else {
        nzcount = 0 as ::core::ffi::c_int;
        if (*lp).obj_in_basis != 0 {
            value = get_OF_active(lp, (*lp).rows + col_nr, mult);
            if value != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                nzcount += 1;
                *nzlist.offset(nzcount as isize) = 0 as ::core::ffi::c_int;
                *column.offset(nzcount as isize) = value;
            }
        }
        i = *(*mat)
            .col_end
            .offset((col_nr - 1 as ::core::ffi::c_int) as isize);
        ie = *(*mat).col_end.offset(col_nr as isize);
        matRownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
        matValue = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
        while i < ie {
            j = *matRownr;
            value = *matValue * mult;
            nzcount += 1;
            *nzlist.offset(nzcount as isize) = j;
            *column.offset(nzcount as isize) = value;
            if fabs(value) > maxval {
                maxval = fabs(value);
                maxidx = nzcount;
            }
            i += 1;
            matRownr = matRownr.offset(matRowColStep as isize);
            matValue = matValue.offset(matValueStep as isize);
        }
    }
    if !maxabs.is_null() {
        *maxabs = maxidx;
    }
    return nzcount;
}
#[export_name="honest_lpsolve_obtain_column"]
pub unsafe extern "C" fn obtain_column(
    mut lp: *mut lprec,
    mut varin: ::core::ffi::c_int,
    mut pcol: *mut ::core::ffi::c_double,
    mut nzlist: *mut ::core::ffi::c_int,
    mut maxabs: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_double =
        (if *(*lp).is_lower.offset(varin as isize) as ::core::ffi::c_int != 0
            && -(1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
        {
            --(1 as ::core::ffi::c_int)
        } else {
            -(1 as ::core::ffi::c_int)
        }) as ::core::ffi::c_double;
    if varin > (*lp).rows {
        varin -= (*lp).rows;
        varin = expand_column(lp, varin, pcol, nzlist, value, maxabs);
    } else if (*lp).obj_in_basis as ::core::ffi::c_int != 0 || varin > 0 as ::core::ffi::c_int {
        varin = singleton_column(lp, varin, pcol, nzlist, value, maxabs);
    } else {
        varin = get_basisOF(
            lp,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            pcol as *mut ::core::ffi::c_double,
            nzlist as *mut ::core::ffi::c_int,
        );
    }
    return varin;
}
#[export_name="honest_lpsolve_set_callbacks"]
pub unsafe extern "C" fn set_callbacks(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    (*lp).add_column = Some(
        add_column
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<add_column_func>;
    (*lp).add_columnex = Some(
        add_columnex
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<add_columnex_func>;
    (*lp).add_constraint = Some(
        add_constraint
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<add_constraint_func>;
    (*lp).add_constraintex = Some(
        add_constraintex
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<add_constraintex_func>;
    (*lp).add_lag_con = Some(
        add_lag_con
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<add_lag_con_func>;
    (*lp).add_SOS = Some(
        add_SOS
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_int,
    ) as Option<add_SOS_func>;
    (*lp).column_in_lp = Some(
        column_in_lp
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_int,
    ) as Option<column_in_lp_func>;
    (*lp).copy_lp =
        Some(copy_lp as unsafe extern "C" fn(*mut lprec) -> *mut lprec) as Option<copy_lp_func>;
    (*lp).default_basis =
        Some(default_basis as unsafe extern "C" fn(*mut lprec) -> ()) as Option<default_basis_func>;
    (*lp).del_column = Some(
        del_column as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<del_column_func>;
    (*lp).del_constraint = Some(
        del_constraint
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<del_constraint_func>;
    (*lp).delete_lp =
        Some(delete_lp as unsafe extern "C" fn(*mut lprec) -> ()) as Option<delete_lp_func>;
    (*lp).dualize_lp = Some(dualize_lp as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
        as Option<dualize_lp_func>;
    (*lp).free_lp =
        Some(free_lp as unsafe extern "C" fn(*mut *mut lprec) -> ()) as Option<free_lp_func>;
    (*lp).get_anti_degen =
        Some(get_anti_degen as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_anti_degen_func>;
    (*lp).get_basis = Some(
        get_basis
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_basis_func>;
    (*lp).get_basiscrash =
        Some(get_basiscrash as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_basiscrash_func>;
    (*lp).get_bb_depthlimit =
        Some(get_bb_depthlimit as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_bb_depthlimit_func>;
    (*lp).get_bb_floorfirst =
        Some(get_bb_floorfirst as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_bb_floorfirst_func>;
    (*lp).get_bb_rule = Some(get_bb_rule as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
        as Option<get_bb_rule_func>;
    (*lp).get_bounds_tighter =
        Some(get_bounds_tighter as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<get_bounds_tighter_func>;
    (*lp).get_break_at_value =
        Some(get_break_at_value as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_break_at_value_func>;
    (*lp).get_col_name = Some(
        get_col_name
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char,
    ) as Option<get_col_name_func>;
    (*lp).get_columnex = Some(
        get_columnex
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as Option<get_columnex_func>;
    (*lp).get_constr_type = Some(
        get_constr_type
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int,
    ) as Option<get_constr_type_func>;
    (*lp).get_constr_value = Some(
        get_constr_value
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_double,
    ) as Option<get_constr_value_func>;
    (*lp).get_constraints = Some(
        get_constraints
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<get_constraints_func>;
    (*lp).get_dual_solution = Some(
        get_dual_solution
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<get_dual_solution_func>;
    (*lp).get_epsb = Some(get_epsb as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
        as Option<get_epsb_func>;
    (*lp).get_epsd = Some(get_epsd as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
        as Option<get_epsd_func>;
    (*lp).get_epsel = Some(get_epsel as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
        as Option<get_epsel_func>;
    (*lp).get_epsint = Some(get_epsint as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
        as Option<get_epsint_func>;
    (*lp).get_epsperturb =
        Some(get_epsperturb as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_epsperturb_func>;
    (*lp).get_epspivot =
        Some(get_epspivot as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_epspivot_func>;
    (*lp).get_improve = Some(get_improve as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
        as Option<get_improve_func>;
    (*lp).get_infinite =
        Some(get_infinite as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_infinite_func>;
    (*lp).get_lambda = Some(
        get_lambda
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<get_lambda_func>;
    (*lp).get_lowbo = Some(
        get_lowbo as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double,
    ) as Option<get_lowbo_func>;
    (*lp).get_lp_index = Some(
        get_lp_index as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int,
    ) as Option<get_lp_index_func>;
    (*lp).get_lp_name =
        Some(get_lp_name as unsafe extern "C" fn(*mut lprec) -> *mut ::core::ffi::c_char)
            as Option<get_lp_name_func>;
    (*lp).get_Lrows = Some(get_Lrows as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
        as Option<get_Lrows_func>;
    (*lp).get_mat = Some(
        get_mat
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_double,
    ) as Option<get_mat_func>;
    (*lp).get_mat_byindex = Some(
        get_mat_byindex
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_uchar,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_double,
    ) as Option<get_mat_byindex_func>;
    (*lp).get_max_level =
        Some(get_max_level as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_max_level_func>;
    (*lp).get_maxpivot =
        Some(get_maxpivot as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_maxpivot_func>;
    (*lp).get_mip_gap = Some(
        get_mip_gap
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_double,
    ) as Option<get_mip_gap_func>;
    (*lp).get_multiprice = Some(
        get_multiprice
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_int,
    ) as Option<get_multiprice_func>;
    (*lp).get_nameindex = Some(
        get_nameindex
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_int,
    ) as Option<get_nameindex_func>;
    (*lp).get_Ncolumns =
        Some(get_Ncolumns as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_Ncolumns_func>;
    (*lp).get_negrange =
        Some(get_negrange as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_negrange_func>;
    (*lp).get_nonzeros =
        Some(get_nonzeros as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_nz_func>;
    (*lp).get_Norig_columns =
        Some(get_Norig_columns as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_Norig_columns_func>;
    (*lp).get_Norig_rows =
        Some(get_Norig_rows as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_Norig_rows_func>;
    (*lp).get_Nrows = Some(get_Nrows as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
        as Option<get_Nrows_func>;
    (*lp).get_obj_bound =
        Some(get_obj_bound as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_obj_bound_func>;
    (*lp).get_objective =
        Some(get_objective as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_objective_func>;
    (*lp).get_orig_index = Some(
        get_orig_index
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int,
    ) as Option<get_orig_index_func>;
    (*lp).get_origcol_name = Some(
        get_origcol_name
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char,
    ) as Option<get_origcol_name_func>;
    (*lp).get_origrow_name = Some(
        get_origrow_name
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char,
    ) as Option<get_origrow_name_func>;
    (*lp).get_partialprice = Some(
        get_partialprice
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> (),
    ) as Option<get_partialprice_func>;
    (*lp).get_pivoting =
        Some(get_pivoting as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_pivoting_func>;
    (*lp).get_presolve =
        Some(get_presolve as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_presolve_func>;
    (*lp).get_presolveloops =
        Some(get_presolveloops as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_presolveloops_func>;
    (*lp).get_primal_solution = Some(
        get_primal_solution
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<get_primal_solution_func>;
    (*lp).get_print_sol =
        Some(get_print_sol as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_print_sol_func>;
    (*lp).get_pseudocosts = Some(
        get_pseudocosts
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_pseudocosts_func>;
    (*lp).get_ptr_constraints = Some(
        get_ptr_constraints
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_constraints_func>;
    (*lp).get_ptr_dual_solution = Some(
        get_ptr_dual_solution
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_dual_solution_func>;
    (*lp).get_ptr_lambda = Some(
        get_ptr_lambda
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_lambda_func>;
    (*lp).get_ptr_primal_solution = Some(
        get_ptr_primal_solution
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_primal_solution_func>;
    (*lp).get_ptr_sensitivity_obj = Some(
        get_ptr_sensitivity_obj
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_sensitivity_obj_func>;
    (*lp).get_ptr_sensitivity_objex = Some(
        get_ptr_sensitivity_objex
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
                *mut *mut ::core::ffi::c_double,
                *mut *mut ::core::ffi::c_double,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_sensitivity_objex_func>;
    (*lp).get_ptr_sensitivity_rhs = Some(
        get_ptr_sensitivity_rhs
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
                *mut *mut ::core::ffi::c_double,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_sensitivity_rhs_func>;
    (*lp).get_ptr_variables = Some(
        get_ptr_variables
            as unsafe extern "C" fn(
                *mut lprec,
                *mut *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_ptr_variables_func>;
    (*lp).get_rh = Some(
        get_rh as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double,
    ) as Option<get_rh_func>;
    (*lp).get_rh_range = Some(
        get_rh_range
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double,
    ) as Option<get_rh_range_func>;
    (*lp).get_row = Some(
        get_row
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_row_func>;
    (*lp).get_rowex = Some(
        get_rowex
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as Option<get_rowex_func>;
    (*lp).get_row_name = Some(
        get_row_name
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char,
    ) as Option<get_row_name_func>;
    (*lp).get_scalelimit =
        Some(get_scalelimit as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_scalelimit_func>;
    (*lp).get_scaling = Some(get_scaling as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
        as Option<get_scaling_func>;
    (*lp).get_sensitivity_obj = Some(
        get_sensitivity_obj
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_sensitivity_obj_func>;
    (*lp).get_sensitivity_objex = Some(
        get_sensitivity_objex
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_sensitivity_objex_func>;
    (*lp).get_sensitivity_rhs = Some(
        get_sensitivity_rhs
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<get_sensitivity_rhs_func>;
    (*lp).get_simplextype =
        Some(get_simplextype as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_simplextype_func>;
    (*lp).get_solutioncount =
        Some(get_solutioncount as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_solutioncount_func>;
    (*lp).get_solutionlimit =
        Some(get_solutionlimit as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
            as Option<get_solutionlimit_func>;
    (*lp).get_status = Some(get_status as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
        as Option<get_status_func>;
    (*lp).get_statustext = Some(
        get_statustext
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> *mut ::core::ffi::c_char,
    ) as Option<get_statustext_func>;
    (*lp).get_timeout = Some(get_timeout as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_long)
        as Option<get_timeout_func>;
    (*lp).get_total_iter =
        Some(get_total_iter as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_longlong)
            as Option<get_total_iter_func>;
    (*lp).get_total_nodes =
        Some(get_total_nodes as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_longlong)
            as Option<get_total_nodes_func>;
    (*lp).get_upbo = Some(
        get_upbo as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double,
    ) as Option<get_upbo_func>;
    (*lp).get_var_branch = Some(
        get_var_branch
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int,
    ) as Option<get_var_branch_func>;
    (*lp).get_var_dualresult = Some(
        get_var_dualresult
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double,
    ) as Option<get_var_dualresult_func>;
    (*lp).get_var_primalresult = Some(
        get_var_primalresult
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_double,
    ) as Option<get_var_primalresult_func>;
    (*lp).get_var_priority = Some(
        get_var_priority
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int,
    ) as Option<get_var_priority_func>;
    (*lp).get_variables = Some(
        get_variables
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<get_variables_func>;
    (*lp).get_verbose = Some(get_verbose as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
        as Option<get_verbose_func>;
    (*lp).get_working_objective =
        Some(get_working_objective as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<get_working_objective_func>;
    (*lp).has_BFP = Some(has_BFP as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
        as Option<has_BFP_func>;
    (*lp).has_XLI = Some(has_XLI as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
        as Option<has_XLI_func>;
    (*lp).is_add_rowmode =
        Some(is_add_rowmode as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<is_add_rowmode_func>;
    (*lp).is_anti_degen = Some(
        is_anti_degen
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_anti_degen_func>;
    (*lp).is_binary = Some(
        is_binary as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_binary_func>;
    (*lp).is_break_at_first =
        Some(is_break_at_first as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<is_break_at_first_func>;
    (*lp).is_constr_type = Some(
        is_constr_type
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<is_constr_type_func>;
    (*lp).is_debug = Some(is_debug as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
        as Option<is_debug_func>;
    (*lp).is_feasible = Some(
        is_feasible
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<is_feasible_func>;
    (*lp).is_unbounded = Some(
        is_unbounded
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_unbounded_func>;
    (*lp).is_infinite = Some(
        is_infinite
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<is_infinite_func>;
    (*lp).is_int = Some(
        is_int as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_int_func>;
    (*lp).is_integerscaling =
        Some(is_integerscaling as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<is_integerscaling_func>;
    (*lp).is_lag_trace =
        Some(is_lag_trace as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<is_lag_trace_func>;
    (*lp).is_maxim = Some(is_maxim as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
        as Option<is_maxim_func>;
    (*lp).is_nativeBFP =
        Some(is_nativeBFP as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<is_nativeBFP_func>;
    (*lp).is_nativeXLI =
        Some(is_nativeXLI as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<is_nativeXLI_func>;
    (*lp).is_negative = Some(
        is_negative as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_negative_func>;
    (*lp).is_obj_in_basis =
        Some(is_obj_in_basis as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
            as Option<is_obj_in_basis_func>;
    (*lp).is_piv_mode = Some(
        is_piv_mode as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_piv_mode_func>;
    (*lp).is_piv_rule = Some(
        is_piv_rule as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_piv_rule_func>;
    (*lp).is_presolve = Some(
        is_presolve as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_presolve_func>;
    (*lp).is_scalemode = Some(
        is_scalemode
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_scalemode_func>;
    (*lp).is_scaletype = Some(
        is_scaletype
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_scaletype_func>;
    (*lp).is_semicont = Some(
        is_semicont as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_semicont_func>;
    (*lp).is_SOS_var = Some(
        is_SOS_var as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_SOS_var_func>;
    (*lp).is_trace = Some(is_trace as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
        as Option<is_trace_func>;
    (*lp).lp_solve_version = Some(
        lp_solve_version
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as Option<lp_solve_version_func>;
    (*lp).make_lp =
        Some(make_lp as unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> *mut lprec)
            as Option<make_lp_func>;
    (*lp).print_constraints =
        Some(print_constraints as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<print_constraints_func>;
    (*lp).print_duals =
        Some(print_duals as unsafe extern "C" fn(*mut lprec) -> ()) as Option<print_duals_func>;
    (*lp).print_lp =
        Some(print_lp as unsafe extern "C" fn(*mut lprec) -> ()) as Option<print_lp_func>;
    (*lp).print_objective = Some(print_objective as unsafe extern "C" fn(*mut lprec) -> ())
        as Option<print_objective_func>;
    (*lp).print_scales =
        Some(print_scales as unsafe extern "C" fn(*mut lprec) -> ()) as Option<print_scales_func>;
    (*lp).print_solution =
        Some(print_solution as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<print_solution_func>;
    (*lp).print_str =
        Some(print_str as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ())
            as Option<print_str_func>;
    (*lp).print_tableau =
        Some(print_tableau as unsafe extern "C" fn(*mut lprec) -> ()) as Option<print_tableau_func>;
    (*lp).put_abortfunc = Some(
        put_abortfunc
            as unsafe extern "C" fn(
                *mut lprec,
                Option<lphandle_intfunc>,
                *mut ::core::ffi::c_void,
            ) -> (),
    ) as Option<put_abortfunc_func>;
    (*lp).put_bb_nodefunc = Some(
        put_bb_nodefunc
            as unsafe extern "C" fn(
                *mut lprec,
                Option<lphandleint_intfunc>,
                *mut ::core::ffi::c_void,
            ) -> (),
    ) as Option<put_bb_nodefunc_func>;
    (*lp).put_bb_branchfunc = Some(
        put_bb_branchfunc
            as unsafe extern "C" fn(
                *mut lprec,
                Option<lphandleint_intfunc>,
                *mut ::core::ffi::c_void,
            ) -> (),
    ) as Option<put_bb_branchfunc_func>;
    (*lp).put_logfunc = Some(
        put_logfunc
            as unsafe extern "C" fn(
                *mut lprec,
                Option<lphandlestr_func>,
                *mut ::core::ffi::c_void,
            ) -> (),
    ) as Option<put_logfunc_func>;
    (*lp).put_msgfunc = Some(
        put_msgfunc
            as unsafe extern "C" fn(
                *mut lprec,
                Option<lphandleint_func>,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> (),
    ) as Option<put_msgfunc_func>;
    (*lp).read_LP = Some(
        read_LP
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
            ) -> *mut lprec,
    ) as Option<read_LP_func>;
    (*lp).read_MPS = Some(
        read_MPS
            as unsafe extern "C" fn(*mut ::core::ffi::c_char, ::core::ffi::c_int) -> *mut lprec,
    ) as Option<read_MPS_func>;
    (*lp).read_XLI = Some(
        read_XLI
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> *mut lprec,
    ) as Option<read_XLI_func>;
    (*lp).read_basis = Some(
        read_basis
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_uchar,
    ) as Option<read_basis_func>;
    (*lp).reset_basis =
        Some(reset_basis as unsafe extern "C" fn(*mut lprec) -> ()) as Option<reset_basis_func>;
    (*lp).read_params = Some(
        read_params
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_uchar,
    ) as Option<read_params_func>;
    (*lp).reset_params =
        Some(reset_params as unsafe extern "C" fn(*mut lprec) -> ()) as Option<reset_params_func>;
    (*lp).resize_lp = Some(
        resize_lp
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<resize_lp_func>;
    (*lp).set_action =
        Some(set_action as unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> ())
            as Option<set_actionfunc>;
    (*lp).set_add_rowmode = Some(
        set_add_rowmode
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_uchar,
    ) as Option<set_add_rowmode_func>;
    (*lp).set_anti_degen =
        Some(set_anti_degen as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_anti_degen_func>;
    (*lp).set_basisvar = Some(
        set_basisvar
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as Option<set_basisvar_func>;
    (*lp).set_basis = Some(
        set_basis
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_basis_func>;
    (*lp).set_basiscrash =
        Some(set_basiscrash as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_basiscrash_func>;
    (*lp).set_bb_depthlimit =
        Some(set_bb_depthlimit as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_bb_depthlimit_func>;
    (*lp).set_bb_floorfirst =
        Some(set_bb_floorfirst as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_bb_floorfirst_func>;
    (*lp).set_bb_rule =
        Some(set_bb_rule as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_bb_rule_func>;
    (*lp).set_BFP = Some(
        set_BFP
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<set_BFP_func>;
    (*lp).set_binary = Some(
        set_binary
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_binary_func>;
    (*lp).set_bounds = Some(
        set_bounds
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_bounds_func>;
    (*lp).set_bounds_tighter =
        Some(set_bounds_tighter as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_bounds_tighter_func>;
    (*lp).set_break_at_first =
        Some(set_break_at_first as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_break_at_first_func>;
    (*lp).set_break_at_value =
        Some(set_break_at_value as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_break_at_value_func>;
    (*lp).set_col_name = Some(
        set_col_name
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_col_name_func>;
    (*lp).set_constr_type = Some(
        set_constr_type
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_constr_type_func>;
    (*lp).set_debug =
        Some(set_debug as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_debug_func>;
    (*lp).set_epsb = Some(set_epsb as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
        as Option<set_epsb_func>;
    (*lp).set_epsd = Some(set_epsd as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
        as Option<set_epsd_func>;
    (*lp).set_epsel =
        Some(set_epsel as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_epsel_func>;
    (*lp).set_epsint =
        Some(set_epsint as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_epsint_func>;
    (*lp).set_epslevel = Some(
        set_epslevel
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<set_epslevel_func>;
    (*lp).set_epsperturb =
        Some(set_epsperturb as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_epsperturb_func>;
    (*lp).set_epspivot =
        Some(set_epspivot as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_epspivot_func>;
    (*lp).set_unbounded = Some(
        set_unbounded
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<set_unbounded_func>;
    (*lp).set_improve =
        Some(set_improve as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_improve_func>;
    (*lp).set_infinite =
        Some(set_infinite as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_infinite_func>;
    (*lp).set_int = Some(
        set_int
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_int_func>;
    (*lp).set_lag_trace =
        Some(set_lag_trace as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_lag_trace_func>;
    (*lp).set_lowbo = Some(
        set_lowbo
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_lowbo_func>;
    (*lp).set_lp_name = Some(
        set_lp_name
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<set_lp_name_func>;
    (*lp).set_mat = Some(
        set_mat
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_mat_func>;
    (*lp).set_maxim =
        Some(set_maxim as unsafe extern "C" fn(*mut lprec) -> ()) as Option<set_maxim_func>;
    (*lp).set_maxpivot =
        Some(set_maxpivot as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_maxpivot_func>;
    (*lp).set_minim =
        Some(set_minim as unsafe extern "C" fn(*mut lprec) -> ()) as Option<set_minim_func>;
    (*lp).set_mip_gap = Some(
        set_mip_gap
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar, ::core::ffi::c_double) -> (),
    ) as Option<set_mip_gap_func>;
    (*lp).set_multiprice = Some(
        set_multiprice
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<set_multiprice_func>;
    (*lp).set_negrange =
        Some(set_negrange as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_negrange_func>;
    (*lp).set_obj = Some(
        set_obj
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_obj_func>;
    (*lp).set_obj_bound =
        Some(set_obj_bound as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_obj_bound_func>;
    (*lp).set_obj_fn = Some(
        set_obj_fn
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<set_obj_fn_func>;
    (*lp).set_obj_fnex = Some(
        set_obj_fnex
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_obj_fnex_func>;
    (*lp).set_obj_in_basis =
        Some(set_obj_in_basis as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_obj_in_basis_func>;
    (*lp).set_outputfile = Some(
        set_outputfile
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<set_outputfile_func>;
    (*lp).set_outputstream =
        Some(set_outputstream as unsafe extern "C" fn(*mut lprec, *mut FILE) -> ())
            as Option<set_outputstream_func>;
    (*lp).set_partialprice = Some(
        set_partialprice
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_partialprice_func>;
    (*lp).set_pivoting =
        Some(set_pivoting as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_pivoting_func>;
    (*lp).set_preferdual =
        Some(set_preferdual as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_preferdual_func>;
    (*lp).set_presolve = Some(
        set_presolve
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    ) as Option<set_presolve_func>;
    (*lp).set_print_sol =
        Some(set_print_sol as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_print_sol_func>;
    (*lp).set_pseudocosts = Some(
        set_pseudocosts
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_pseudocosts_func>;
    (*lp).set_rh = Some(
        set_rh
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_rh_func>;
    (*lp).set_rh_range = Some(
        set_rh_range
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_rh_range_func>;
    (*lp).set_rh_vec =
        Some(set_rh_vec as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ())
            as Option<set_rh_vec_func>;
    (*lp).set_row = Some(
        set_row
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_row_func>;
    (*lp).set_rowex = Some(
        set_rowex
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_rowex_func>;
    (*lp).set_row_name = Some(
        set_row_name
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_row_name_func>;
    (*lp).set_scalelimit =
        Some(set_scalelimit as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_double) -> ())
            as Option<set_scalelimit_func>;
    (*lp).set_scaling =
        Some(set_scaling as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_scaling_func>;
    (*lp).set_semicont = Some(
        set_semicont
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_semicont_func>;
    (*lp).set_sense =
        Some(set_sense as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_sense_func>;
    (*lp).set_simplextype =
        Some(set_simplextype as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_simplextype_func>;
    (*lp).set_solutionlimit =
        Some(set_solutionlimit as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_solutionlimit_func>;
    (*lp).set_timeout =
        Some(set_timeout as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_long) -> ())
            as Option<set_timeout_func>;
    (*lp).set_trace =
        Some(set_trace as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ())
            as Option<set_trace_func>;
    (*lp).set_upbo = Some(
        set_upbo
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_upbo_func>;
    (*lp).set_var_branch = Some(
        set_var_branch
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_uchar,
    ) as Option<set_var_branch_func>;
    (*lp).set_var_weights = Some(
        set_var_weights
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_double) -> ::core::ffi::c_uchar,
    ) as Option<set_var_weights_func>;
    (*lp).set_verbose =
        Some(set_verbose as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ())
            as Option<set_verbose_func>;
    (*lp).set_XLI = Some(
        set_XLI
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<set_XLI_func>;
    (*lp).solve =
        Some(solve as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int) as Option<solve_func>;
    (*lp).str_add_column = Some(
        str_add_column
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<str_add_column_func>;
    (*lp).str_add_constraint = Some(
        str_add_constraint
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<str_add_constraint_func>;
    (*lp).str_add_lag_con = Some(
        str_add_lag_con
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_uchar,
    ) as Option<str_add_lag_con_func>;
    (*lp).str_set_obj_fn = Some(
        str_set_obj_fn
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<str_set_obj_fn_func>;
    (*lp).str_set_rh_vec = Some(
        str_set_rh_vec
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<str_set_rh_vec_func>;
    (*lp).time_elapsed =
        Some(time_elapsed as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
            as Option<time_elapsed_func>;
    (*lp).unscale = Some(unscale as unsafe extern "C" fn(*mut lprec) -> ()) as Option<unscale_func>;
    (*lp).write_lp = Some(
        write_lp
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<write_lp_func>;
    (*lp).write_LP =
        Some(write_LP as unsafe extern "C" fn(*mut lprec, *mut FILE) -> ::core::ffi::c_uchar)
            as Option<write_LP_func>;
    (*lp).write_mps = Some(
        write_mps
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<write_mps_func>;
    (*lp).write_freemps = Some(
        write_freemps
            as unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar,
    ) as Option<write_freemps_func>;
    (*lp).write_MPS =
        Some(write_MPS as unsafe extern "C" fn(*mut lprec, *mut FILE) -> ::core::ffi::c_uchar)
            as Option<write_MPS_func>;
    (*lp).write_freeMPS =
        Some(write_freeMPS as unsafe extern "C" fn(*mut lprec, *mut FILE) -> ::core::ffi::c_uchar)
            as Option<write_freeMPS_func>;
    (*lp).write_XLI = Some(
        write_XLI
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<write_XLI_func>;
    (*lp).write_params = Some(
        write_params
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_uchar,
    ) as Option<write_params_func>;
    (*lp).userabort = Some(
        userabort as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<userabortfunc>;
    (*lp).report = Some(
        report
            as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int, *mut ::core::ffi::c_char) -> (),
    ) as Option<reportfunc>;
    (*lp).explain = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lprec, *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
        >,
        Option<explainfunc>,
    >(Some(
        explain
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_char,
    ));
    (*lp).set_basisvar = Some(
        set_basisvar
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as Option<set_basisvar_func>;
    (*lp).get_lpcolumn = Some(
        obtain_column
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as Option<getvectorfunc>;
    (*lp).get_basiscolumn = Some(
        get_basiscolumn
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_int,
    ) as Option<getpackedfunc>;
    (*lp).get_OF_active = Some(
        get_OF_active
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_double,
    ) as Option<get_OF_activefunc>;
    (*lp).getMDO = Some(
        getMDO
            as unsafe extern "C" fn(
                *mut lprec,
                *mut ::core::ffi::c_uchar,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_int,
    ) as Option<getMDOfunc>;
    (*lp).invert = Some(
        invert
            as unsafe extern "C" fn(
                *mut lprec,
                ::core::ffi::c_uchar,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_uchar,
    ) as Option<invertfunc>;
    (*lp).set_action =
        Some(set_action as unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> ())
            as Option<set_actionfunc>;
    (*lp).clear_action = Some(
        clear_action as unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    ) as Option<clear_actionfunc>;
    (*lp).is_action = Some(
        is_action
            as unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
    ) as Option<is_actionfunc>;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_has_BFP"]
pub unsafe extern "C" fn has_BFP(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (is_nativeBFP(lp) as ::core::ffi::c_int != 0
        || ((*lp).hBFP != NULL) as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_nativeBFP"]
pub unsafe extern "C" fn is_nativeBFP(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return ((*lp).hBFP == NULL) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_BFP"]
pub unsafe extern "C" fn set_BFP(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut result: ::core::ffi::c_int = LIB_LOADED;
    if !(*lp).invB.is_null() {
        bfp_free(lp);
    }
    if !(*lp).hBFP.is_null() {
        native_only!(dlclose,(*lp).hBFP);
        (*lp).hBFP = NULL;
    }
    if filename.is_null() {
        if is_nativeBFP(lp) == 0 {
            return 0 as ::core::ffi::c_uchar;
        }
        (*lp).bfp_name =
            Some(bfp_name as unsafe extern "C" fn() -> *mut ::core::ffi::c_char) as Option<BFPchar>;
        (*lp).bfp_compatible = Some(
            bfp_compatible
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_uchar,
        ) as Option<BFPbool_lpintintint>;
        (*lp).bfp_free = Some(bfp_free as unsafe extern "C" fn(*mut lprec) -> ()) as Option<BFP_lp>;
        (*lp).bfp_resize = Some(
            bfp_resize
                as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
        ) as Option<BFPbool_lpint>;
        (*lp).bfp_nonzeros = Some(
            bfp_nonzeros
                as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_int,
        ) as Option<BFPint_lpbool>;
        (*lp).bfp_memallocated =
            Some(bfp_memallocated as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
                as Option<BFPint_lp>;
        (*lp).bfp_restart =
            Some(bfp_restart as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
                as Option<BFPbool_lp>;
        (*lp).bfp_mustrefactorize =
            Some(bfp_mustrefactorize as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
                as Option<BFPbool_lp>;
        (*lp).bfp_preparefactorization = Some(
            bfp_preparefactorization as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int,
        ) as Option<BFPint_lp>;
        (*lp).bfp_factorize = Some(
            bfp_factorize
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_uchar,
                    ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ) as Option<BFPint_lpintintboolbool>;
        (*lp).bfp_finishupdate = Some(
            bfp_finishupdate
                as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_uchar) -> ::core::ffi::c_uchar,
        ) as Option<BFPbool_lpbool>;
        (*lp).bfp_ftran_normal = Some(
            bfp_ftran_normal
                as unsafe extern "C" fn(
                    *mut lprec,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> (),
        ) as Option<BFP_lprealint>;
        (*lp).bfp_ftran_prepare = Some(
            bfp_ftran_prepare
                as unsafe extern "C" fn(
                    *mut lprec,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> (),
        ) as Option<BFP_lprealint>;
        (*lp).bfp_btran_normal = Some(
            bfp_btran_normal
                as unsafe extern "C" fn(
                    *mut lprec,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> (),
        ) as Option<BFP_lprealint>;
        (*lp).bfp_status =
            Some(bfp_status as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
                as Option<BFPint_lp>;
        (*lp).bfp_implicitslack =
            Some(bfp_implicitslack as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
                as Option<BFPbool_lp>;
        (*lp).bfp_indexbase =
            Some(bfp_indexbase as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
                as Option<BFPint_lp>;
        (*lp).bfp_rowoffset =
            Some(bfp_rowoffset as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
                as Option<BFPint_lp>;
        (*lp).bfp_pivotmax =
            Some(bfp_pivotmax as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
                as Option<BFPint_lp>;
        (*lp).bfp_init = Some(
            bfp_init
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_uchar,
        ) as Option<BFPbool_lpintintchar>;
        (*lp).bfp_pivotalloc = Some(
            bfp_pivotalloc
                as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_uchar,
        ) as Option<BFPbool_lpint>;
        (*lp).bfp_colcount =
            Some(bfp_colcount as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
                as Option<BFPint_lp>;
        (*lp).bfp_canresetbasis =
            Some(bfp_canresetbasis as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
                as Option<BFPbool_lp>;
        (*lp).bfp_finishfactorization =
            Some(bfp_finishfactorization as unsafe extern "C" fn(*mut lprec) -> ())
                as Option<BFP_lp>;
        (*lp).bfp_updaterefactstats =
            Some(bfp_updaterefactstats as unsafe extern "C" fn(*mut lprec) -> ()) as Option<BFP_lp>;
        (*lp).bfp_prepareupdate = Some(
            bfp_prepareupdate
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
        ) as Option<BFPlreal_lpintintreal>;
        (*lp).bfp_pivotRHS = Some(
            bfp_pivotRHS
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
        ) as Option<BFPreal_lplrealreal>;
        (*lp).bfp_btran_double = Some(
            bfp_btran_double
                as unsafe extern "C" fn(
                    *mut lprec,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> (),
        ) as Option<BFP_lprealintrealint>;
        (*lp).bfp_efficiency =
            Some(bfp_efficiency as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_double)
                as Option<BFPreal_lp>;
        (*lp).bfp_pivotvector =
            Some(bfp_pivotvector as unsafe extern "C" fn(*mut lprec) -> *mut ::core::ffi::c_double)
                as Option<BFPrealp_lp>;
        (*lp).bfp_pivotcount =
            Some(bfp_pivotcount as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_int)
                as Option<BFPint_lp>;
        (*lp).bfp_refactcount = Some(
            bfp_refactcount
                as unsafe extern "C" fn(*mut lprec, ::core::ffi::c_int) -> ::core::ffi::c_int,
        ) as Option<BFPint_lpint>;
        (*lp).bfp_isSetI =
            Some(bfp_isSetI as unsafe extern "C" fn(*mut lprec) -> ::core::ffi::c_uchar)
                as Option<BFPbool_lp>;
        (*lp).bfp_findredundant = Some(
            bfp_findredundant
                as unsafe extern "C" fn(
                    *mut lprec,
                    ::core::ffi::c_int,
                    Option<getcolumnex_func>,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ) as Option<BFPint_lpintrealcbintint>;
    } else {
        let mut bfpname: [::core::ffi::c_char; 260] = [0; 260];
        let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        strcpy(&raw mut bfpname as *mut ::core::ffi::c_char, filename);
        ptr = strrchr(filename, '/' as i32);
        if ptr.is_null() {
            ptr = filename;
        } else {
            ptr = ptr.offset(1);
        }
        bfpname[ptr.offset_from(filename) as ::core::ffi::c_long as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_char;
        if strncmp(
            ptr,
            b"lib\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) != 0
        {
            strcat(
                &raw mut bfpname as *mut ::core::ffi::c_char,
                b"lib\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        strcat(&raw mut bfpname as *mut ::core::ffi::c_char, ptr);
        if strcmp(
            (&raw mut bfpname as *mut ::core::ffi::c_char)
                .offset(strlen(&raw mut bfpname as *mut ::core::ffi::c_char) as isize)
                .offset(-(3 as ::core::ffi::c_int as isize)),
            b".so\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            strcat(
                &raw mut bfpname as *mut ::core::ffi::c_char,
                b".so\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*lp).hBFP = native_only!(dlopen,&raw mut bfpname as *mut ::core::ffi::c_char, RTLD_LAZY);
        if !(*lp).hBFP.is_null() {
            let ref mut fresh6 = *(&raw mut (*lp).bfp_compatible as *mut *mut ::core::ffi::c_void);
            *fresh6 = native_only!(dlsym,
                (*lp).hBFP,
                b"bfp_compatible\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if (*lp).bfp_compatible.is_none() {
                result = LIB_NOINFO;
            } else if (*lp).bfp_compatible.expect("non-null function pointer")(
                lp,
                BFPVERSION,
                MAJORVERSION,
                ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
            ) != 0
            {
                let ref mut fresh7 = *(&raw mut (*lp).bfp_name as *mut *mut ::core::ffi::c_void);
                *fresh7 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_name\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh8 = *(&raw mut (*lp).bfp_free as *mut *mut ::core::ffi::c_void);
                *fresh8 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_free\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh9 = *(&raw mut (*lp).bfp_resize as *mut *mut ::core::ffi::c_void);
                *fresh9 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_resize\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh10 =
                    *(&raw mut (*lp).bfp_nonzeros as *mut *mut ::core::ffi::c_void);
                *fresh10 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_nonzeros\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh11 =
                    *(&raw mut (*lp).bfp_memallocated as *mut *mut ::core::ffi::c_void);
                *fresh11 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_memallocated\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh12 =
                    *(&raw mut (*lp).bfp_restart as *mut *mut ::core::ffi::c_void);
                *fresh12 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_restart\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh13 =
                    *(&raw mut (*lp).bfp_mustrefactorize as *mut *mut ::core::ffi::c_void);
                *fresh13 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_mustrefactorize\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh14 =
                    *(&raw mut (*lp).bfp_preparefactorization as *mut *mut ::core::ffi::c_void);
                *fresh14 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_preparefactorization\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh15 =
                    *(&raw mut (*lp).bfp_factorize as *mut *mut ::core::ffi::c_void);
                *fresh15 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_factorize\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh16 =
                    *(&raw mut (*lp).bfp_finishupdate as *mut *mut ::core::ffi::c_void);
                *fresh16 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_finishupdate\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh17 =
                    *(&raw mut (*lp).bfp_ftran_normal as *mut *mut ::core::ffi::c_void);
                *fresh17 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_ftran_normal\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh18 =
                    *(&raw mut (*lp).bfp_ftran_prepare as *mut *mut ::core::ffi::c_void);
                *fresh18 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_ftran_prepare\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh19 =
                    *(&raw mut (*lp).bfp_btran_normal as *mut *mut ::core::ffi::c_void);
                *fresh19 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_btran_normal\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh20 = *(&raw mut (*lp).bfp_status as *mut *mut ::core::ffi::c_void);
                *fresh20 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_status\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh21 =
                    *(&raw mut (*lp).bfp_implicitslack as *mut *mut ::core::ffi::c_void);
                *fresh21 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_implicitslack\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh22 =
                    *(&raw mut (*lp).bfp_indexbase as *mut *mut ::core::ffi::c_void);
                *fresh22 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_indexbase\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh23 =
                    *(&raw mut (*lp).bfp_rowoffset as *mut *mut ::core::ffi::c_void);
                *fresh23 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_rowoffset\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh24 =
                    *(&raw mut (*lp).bfp_pivotmax as *mut *mut ::core::ffi::c_void);
                *fresh24 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_pivotmax\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh25 = *(&raw mut (*lp).bfp_init as *mut *mut ::core::ffi::c_void);
                *fresh25 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_init\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh26 =
                    *(&raw mut (*lp).bfp_pivotalloc as *mut *mut ::core::ffi::c_void);
                *fresh26 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_pivotalloc\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh27 =
                    *(&raw mut (*lp).bfp_colcount as *mut *mut ::core::ffi::c_void);
                *fresh27 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_colcount\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh28 =
                    *(&raw mut (*lp).bfp_canresetbasis as *mut *mut ::core::ffi::c_void);
                *fresh28 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_canresetbasis\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh29 =
                    *(&raw mut (*lp).bfp_finishfactorization as *mut *mut ::core::ffi::c_void);
                *fresh29 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_finishfactorization\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh30 =
                    *(&raw mut (*lp).bfp_updaterefactstats as *mut *mut ::core::ffi::c_void);
                *fresh30 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_updaterefactstats\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh31 =
                    *(&raw mut (*lp).bfp_prepareupdate as *mut *mut ::core::ffi::c_void);
                *fresh31 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_prepareupdate\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh32 =
                    *(&raw mut (*lp).bfp_pivotRHS as *mut *mut ::core::ffi::c_void);
                *fresh32 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_pivotRHS\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh33 =
                    *(&raw mut (*lp).bfp_btran_double as *mut *mut ::core::ffi::c_void);
                *fresh33 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_btran_double\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh34 =
                    *(&raw mut (*lp).bfp_efficiency as *mut *mut ::core::ffi::c_void);
                *fresh34 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_efficiency\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh35 =
                    *(&raw mut (*lp).bfp_pivotvector as *mut *mut ::core::ffi::c_void);
                *fresh35 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_pivotvector\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh36 =
                    *(&raw mut (*lp).bfp_pivotcount as *mut *mut ::core::ffi::c_void);
                *fresh36 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_pivotcount\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh37 =
                    *(&raw mut (*lp).bfp_refactcount as *mut *mut ::core::ffi::c_void);
                *fresh37 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_refactcount\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh38 = *(&raw mut (*lp).bfp_isSetI as *mut *mut ::core::ffi::c_void);
                *fresh38 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_isSetI\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh39 =
                    *(&raw mut (*lp).bfp_findredundant as *mut *mut ::core::ffi::c_void);
                *fresh39 = native_only!(dlsym,
                    (*lp).hBFP,
                    b"bfp_findredundant\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                result = LIB_VERINVALID;
            }
        } else {
            result = LIB_NOTFOUND;
        }
        if result != LIB_LOADED
            || ((*lp).bfp_name.is_none()
                || (*lp).bfp_compatible.is_none()
                || (*lp).bfp_free.is_none()
                || (*lp).bfp_resize.is_none()
                || (*lp).bfp_nonzeros.is_none()
                || (*lp).bfp_memallocated.is_none()
                || (*lp).bfp_restart.is_none()
                || (*lp).bfp_mustrefactorize.is_none()
                || (*lp).bfp_preparefactorization.is_none()
                || (*lp).bfp_factorize.is_none()
                || (*lp).bfp_finishupdate.is_none()
                || (*lp).bfp_ftran_normal.is_none()
                || (*lp).bfp_ftran_prepare.is_none()
                || (*lp).bfp_btran_normal.is_none()
                || (*lp).bfp_status.is_none()
                || (*lp).bfp_implicitslack.is_none()
                || (*lp).bfp_indexbase.is_none()
                || (*lp).bfp_rowoffset.is_none()
                || (*lp).bfp_pivotmax.is_none()
                || (*lp).bfp_init.is_none()
                || (*lp).bfp_pivotalloc.is_none()
                || (*lp).bfp_colcount.is_none()
                || (*lp).bfp_canresetbasis.is_none()
                || (*lp).bfp_finishfactorization.is_none()
                || (*lp).bfp_updaterefactstats.is_none()
                || (*lp).bfp_prepareupdate.is_none()
                || (*lp).bfp_pivotRHS.is_none()
                || (*lp).bfp_btran_double.is_none()
                || (*lp).bfp_efficiency.is_none()
                || (*lp).bfp_pivotvector.is_none()
                || (*lp).bfp_pivotcount.is_none()
                || (*lp).bfp_refactcount.is_none()
                || (*lp).bfp_isSetI.is_none()
                || (*lp).bfp_findredundant.is_none())
        {
            set_BFP(lp, ::core::ptr::null_mut::<::core::ffi::c_char>());
            if result == LIB_LOADED {
                result = LIB_NOFUNCTION;
            }
        }
    }
    if !filename.is_null() {
        let mut info: [::core::ffi::c_char; 24] = [0; 24];
        match result {
            LIB_NOTFOUND => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_NOTFOUND.as_ptr(),
                );
            }
            LIB_NOINFO => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_NOINFO.as_ptr(),
                );
            }
            LIB_NOFUNCTION => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_NOFUNCTION.as_ptr(),
                );
            }
            LIB_VERINVALID => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_VERINVALID.as_ptr(),
                );
            }
            _ => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_LOADED.as_ptr(),
                );
            }
        }
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_BFP: %s '%s'\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    return (result == LIB_LOADED) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_read_XLI"]
pub unsafe extern "C" fn read_XLI(
    mut xliname: *mut ::core::ffi::c_char,
    mut modelname: *mut ::core::ffi::c_char,
    mut dataname: *mut ::core::ffi::c_char,
    mut options: *mut ::core::ffi::c_char,
    mut verbose: ::core::ffi::c_int,
) -> *mut lprec {
    let mut lp: *mut lprec = ::core::ptr::null_mut::<lprec>();
    lp = make_lp(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    if !lp.is_null() {
        (*lp).source_is_file = TRUE as ::core::ffi::c_uchar;
        (*lp).verbose = verbose;
        if set_XLI(lp, xliname) == 0 {
            free_lp(&raw mut lp);
        } else if (*lp).xli_readmodel.expect("non-null function pointer")(
            lp,
            modelname,
            if !dataname.is_null() && *dataname as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                dataname
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            options,
            verbose,
        ) == 0
        {
            free_lp(&raw mut lp);
        }
    }
    return lp;
}
#[export_name="honest_lpsolve_write_XLI"]
pub unsafe extern "C" fn write_XLI(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
    mut options: *mut ::core::ffi::c_char,
    mut results: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    return (has_XLI(lp) as ::core::ffi::c_int != 0
        && mat_validate((*lp).matA) as ::core::ffi::c_int != 0
        && (*lp).xli_writemodel.expect("non-null function pointer")(lp, filename, options, results)
            as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_has_XLI"]
pub unsafe extern "C" fn has_XLI(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (is_nativeXLI(lp) as ::core::ffi::c_int != 0
        || ((*lp).hXLI != NULL) as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_nativeXLI"]
pub unsafe extern "C" fn is_nativeXLI(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return 0 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_XLI"]
pub unsafe extern "C" fn set_XLI(
    mut lp: *mut lprec,
    mut filename: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut result: ::core::ffi::c_int = LIB_LOADED;
    if !(*lp).hXLI.is_null() {
        native_only!(dlclose,(*lp).hXLI);
        (*lp).hXLI = NULL;
    }
    if filename.is_null() {
        if is_nativeXLI(lp) == 0 {
            return 0 as ::core::ffi::c_uchar;
        }
    } else {
        let mut xliname: [::core::ffi::c_char; 260] = [0; 260];
        let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        strcpy(&raw mut xliname as *mut ::core::ffi::c_char, filename);
        ptr = strrchr(filename, '/' as i32);
        if ptr.is_null() {
            ptr = filename;
        } else {
            ptr = ptr.offset(1);
        }
        xliname[ptr.offset_from(filename) as ::core::ffi::c_long as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_char;
        if strncmp(
            ptr,
            b"lib\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) != 0
        {
            strcat(
                &raw mut xliname as *mut ::core::ffi::c_char,
                b"lib\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        strcat(&raw mut xliname as *mut ::core::ffi::c_char, ptr);
        if strcmp(
            (&raw mut xliname as *mut ::core::ffi::c_char)
                .offset(strlen(&raw mut xliname as *mut ::core::ffi::c_char) as isize)
                .offset(-(3 as ::core::ffi::c_int as isize)),
            b".so\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            strcat(
                &raw mut xliname as *mut ::core::ffi::c_char,
                b".so\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*lp).hXLI = native_only!(dlopen,&raw mut xliname as *mut ::core::ffi::c_char, RTLD_LAZY);
        if !(*lp).hXLI.is_null() {
            let ref mut fresh2 = *(&raw mut (*lp).xli_compatible as *mut *mut ::core::ffi::c_void);
            *fresh2 = native_only!(dlsym,
                (*lp).hXLI,
                b"xli_compatible\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if (*lp).xli_compatible.is_none() {
                result = LIB_NOINFO;
            } else if (*lp).xli_compatible.expect("non-null function pointer")(
                lp,
                XLIVERSION,
                MAJORVERSION,
                ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
            ) != 0
            {
                let ref mut fresh3 = *(&raw mut (*lp).xli_name as *mut *mut ::core::ffi::c_void);
                *fresh3 = native_only!(dlsym,
                    (*lp).hXLI,
                    b"xli_name\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh4 =
                    *(&raw mut (*lp).xli_readmodel as *mut *mut ::core::ffi::c_void);
                *fresh4 = native_only!(dlsym,
                    (*lp).hXLI,
                    b"xli_readmodel\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let ref mut fresh5 =
                    *(&raw mut (*lp).xli_writemodel as *mut *mut ::core::ffi::c_void);
                *fresh5 = native_only!(dlsym,
                    (*lp).hXLI,
                    b"xli_writemodel\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                result = LIB_VERINVALID;
            }
        } else {
            result = LIB_NOTFOUND;
        }
        if result != LIB_LOADED
            || ((*lp).xli_name.is_none()
                || (*lp).xli_compatible.is_none()
                || (*lp).xli_readmodel.is_none()
                || (*lp).xli_writemodel.is_none())
        {
            set_XLI(lp, ::core::ptr::null_mut::<::core::ffi::c_char>());
            if result == LIB_LOADED {
                result = LIB_NOFUNCTION;
            }
        }
    }
    if !filename.is_null() {
        let mut info: [::core::ffi::c_char; 24] = [0; 24];
        match result {
            LIB_NOTFOUND => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_NOTFOUND.as_ptr(),
                );
            }
            LIB_NOINFO => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_NOINFO.as_ptr(),
                );
            }
            LIB_NOFUNCTION => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_NOFUNCTION.as_ptr(),
                );
            }
            LIB_VERINVALID => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_VERINVALID.as_ptr(),
                );
            }
            _ => {
                strcpy(
                    &raw mut info as *mut ::core::ffi::c_char,
                    LIB_STR_LOADED.as_ptr(),
                );
            }
        }
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_XLI: %s '%s'\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    return (result == LIB_LOADED) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_basisOF"]
pub unsafe extern "C" fn get_basisOF(
    mut lp: *mut lprec,
    mut coltarget: *mut ::core::ffi::c_int,
    mut crow: *mut ::core::ffi::c_double,
    mut colno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = (*lp).rows;
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut obj: *mut ::core::ffi::c_double = (*lp).obj;
    let mut epsvalue: ::core::ffi::c_double = (*lp).epsvalue;
    if !coltarget.is_null() {
        let mut ix: ::core::ffi::c_int = 0;
        let mut m: ::core::ffi::c_int = *coltarget.offset(0 as ::core::ffi::c_int as isize);
        let mut value: ::core::ffi::c_double = 0.;
        i = 1 as ::core::ffi::c_int;
        coltarget = coltarget.offset(1);
        while i <= m {
            ix = *coltarget;
            value = *crow.offset(ix as isize);
            if ix > n {
                value += *obj.offset((ix - n) as isize);
            }
            if fabs(value) > epsvalue {
                nz += 1;
                if !colno.is_null() {
                    *colno.offset(nz as isize) = ix;
                }
            } else {
                value = 0.0f64;
            }
            *crow.offset(ix as isize) = value;
            i += 1;
            coltarget = coltarget.offset(1);
        }
    } else {
        let mut basvar: *mut ::core::ffi::c_int = (*lp).var_basic;
        i = 1 as ::core::ffi::c_int;
        crow = crow.offset(1);
        basvar = basvar.offset(1);
        while i <= n {
            if *basvar <= n {
                *crow = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                *crow = -*obj.offset((*basvar - n) as isize);
            }
            if *crow != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                nz += 1;
                if !colno.is_null() {
                    *colno.offset(nz as isize) = i;
                }
            }
            i += 1;
            crow = crow.offset(1);
            basvar = basvar.offset(1);
        }
    }
    if !colno.is_null() {
        *colno.offset(0 as ::core::ffi::c_int as isize) = nz;
    }
    return nz;
}
#[export_name="honest_lpsolve_get_basiscolumn"]
pub unsafe extern "C" fn get_basiscolumn(
    mut lp: *mut lprec,
    mut j: ::core::ffi::c_int,
    mut rn: *mut ::core::ffi::c_int,
    mut bj: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = (*lp).bfp_rowoffset.expect("non-null function pointer")(lp);
    let mut matbase: ::core::ffi::c_int =
        (*lp).bfp_indexbase.expect("non-null function pointer")(lp);
    if matbase > 0 as ::core::ffi::c_int {
        matbase += k - 1 as ::core::ffi::c_int;
    }
    j -= k;
    if j > 0 as ::core::ffi::c_int && (*lp).bfp_isSetI.expect("non-null function pointer")(lp) == 0
    {
        j = *(*lp).var_basic.offset(j as isize);
    }
    if j <= (*lp).rows {
        *rn.offset(1 as ::core::ffi::c_int as isize) = j + matbase;
        *bj.offset(1 as ::core::ffi::c_int as isize) = 1.0f64;
        k = 1 as ::core::ffi::c_int;
    } else {
        k = obtain_column(
            lp,
            j,
            bj as *mut ::core::ffi::c_double,
            rn as *mut ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if matbase != 0 as ::core::ffi::c_int {
            j = 1 as ::core::ffi::c_int;
            while j <= k {
                *rn.offset(j as isize) += matbase;
                j += 1;
            }
        }
    }
    return k;
}
#[export_name="honest_lpsolve_get_primal_solution"]
pub unsafe extern "C" fn get_primal_solution(
    mut lp: *mut lprec,
    mut pv: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if !((*lp).spx_status == OPTIMAL) {
        if (*lp).basis_valid == 0 {
            report(
                lp,
                1 as ::core::ffi::c_int,
                b"get_primal_solution: Not a valid basis\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
    }
    memcpy(
        pv as *mut ::core::ffi::c_void,
        (*lp).best_solution as *const ::core::ffi::c_void,
        (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_ptr_primal_solution"]
pub unsafe extern "C" fn get_ptr_primal_solution(
    mut lp: *mut lprec,
    mut pv: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    *pv = (*lp).best_solution;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_dual_solution"]
pub unsafe extern "C" fn get_dual_solution(
    mut lp: *mut lprec,
    mut rc: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut duals: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut ret: ::core::ffi::c_uchar = 0;
    if (*lp).basis_valid == 0 {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_dual_solution: Not a valid basis\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    ret = get_ptr_sensitivity_rhs(
        lp,
        &raw mut duals,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
    );
    if ret != 0 {
        memcpy(
            rc as *mut ::core::ffi::c_void,
            duals.offset(-(1 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
            (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    return ret;
}
#[export_name="honest_lpsolve_get_ptr_dual_solution"]
pub unsafe extern "C" fn get_ptr_dual_solution(
    mut lp: *mut lprec,
    mut rc: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut ret: ::core::ffi::c_uchar = (*lp).basis_valid;
    if rc.is_null() {
        return (ret as ::core::ffi::c_int != 0
            && (MIP_count(lp) == 0 as ::core::ffi::c_int
                || (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong))
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    if ret == 0 {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_ptr_dual_solution: Not a valid basis\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return ret;
    }
    ret = get_ptr_sensitivity_rhs(
        lp,
        rc,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
    );
    if ret != 0 {
        *rc = (*rc).offset(-1);
    }
    return ret;
}
#[export_name="honest_lpsolve_get_lambda"]
pub unsafe extern "C" fn get_lambda(
    mut lp: *mut lprec,
    mut lambda: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    if (*lp).basis_valid == 0 || get_Lrows(lp) == 0 as ::core::ffi::c_int {
        report(
            lp,
            1 as ::core::ffi::c_int,
            b"get_lambda: Not a valid basis\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    memcpy(
        lambda as *mut ::core::ffi::c_void,
        (*lp).lambda.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        (get_Lrows(lp) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_ptr_lambda"]
pub unsafe extern "C" fn get_ptr_lambda(
    mut lp: *mut lprec,
    mut lambda: *mut *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    *lambda = (*lp).lambda;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_orig_index"]
pub unsafe extern "C" fn get_orig_index(
    mut lp: *mut lprec,
    mut lp_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*lp).varmap_locked != 0 {
        return *(*(*lp).presolve_undo).var_to_orig.offset(lp_index as isize);
    } else if lp_index <= (*(*lp).presolve_undo).orig_rows {
        return lp_index;
    } else {
        return lp_index - (*(*lp).presolve_undo).orig_rows;
    };
}
#[export_name="honest_lpsolve_get_lp_index"]
pub unsafe extern "C" fn get_lp_index(
    mut lp: *mut lprec,
    mut orig_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*lp).varmap_locked != 0 {
        return *(*(*lp).presolve_undo)
            .orig_to_var
            .offset(orig_index as isize);
    } else if orig_index <= (*(*lp).presolve_undo).orig_rows {
        return orig_index;
    } else {
        return orig_index - (*(*lp).presolve_undo).orig_rows;
    };
}
#[export_name="honest_lpsolve_is_feasible"]
pub unsafe extern "C" fn is_feasible(
    mut lp: *mut lprec,
    mut values: *mut ::core::ffi::c_double,
    mut threshold: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut elmnr: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut this_rhs: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut dist: ::core::ffi::c_double = 0.;
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut mat: *mut MATrec = (*lp).matA;
    i = (*lp).rows + 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        if *values.offset((i - (*lp).rows) as isize)
            < unscaled_value(lp, *(*lp).orig_lowbo.offset(i as isize), i)
            || *values.offset((i - (*lp).rows) as isize)
                > unscaled_value(lp, *(*lp).orig_upbo.offset(i as isize), i)
        {
            if !(*(*lp).sc_lobound.offset((i - (*lp).rows) as isize)
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && *values.offset((i - (*lp).rows) as isize)
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            {
                return 0 as ::core::ffi::c_uchar;
            }
        }
        i += 1;
    }
    this_rhs = mempool_obtainVector(
        (*lp).workarrays,
        (*lp).rows + 1 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int,
    ) as *mut ::core::ffi::c_double;
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        elmnr = *(*mat)
            .col_end
            .offset((j - 1 as ::core::ffi::c_int) as isize);
        ie = *(*mat).col_end.offset(j as isize);
        rownr = (*mat).col_mat_rownr.offset(elmnr as isize) as *mut ::core::ffi::c_int;
        value = (*mat).col_mat_value.offset(elmnr as isize) as *mut ::core::ffi::c_double;
        while elmnr < ie {
            *this_rhs.offset(*rownr as isize) += unscaled_mat(lp, *value, *rownr, j);
            elmnr += 1;
            rownr = rownr.offset(matRowColStep as isize);
            value = value.offset(matValueStep as isize);
        }
        j += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        dist = *(*lp).orig_rhs.offset(i as isize) - *this_rhs.offset(i as isize);
        if fabs(dist) < threshold {
            dist = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if *(*lp).orig_upbo.offset(i as isize) == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && dist != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || dist < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            if !(this_rhs as *mut ::core::ffi::c_void).is_null() {
                free(this_rhs as *mut ::core::ffi::c_void);
                this_rhs = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            return 0 as ::core::ffi::c_uchar;
        }
        i += 1;
    }
    mempool_releaseVector(
        (*lp).workarrays,
        this_rhs as *mut ::core::ffi::c_char,
        FALSE as ::core::ffi::c_uchar,
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_column_in_lp"]
pub unsafe extern "C" fn column_in_lp(
    mut lp: *mut lprec,
    mut testcolumn: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut colnr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nz: ::core::ffi::c_int = 0;
    let mut ident: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut mat: *mut MATrec = (*lp).matA;
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut value: ::core::ffi::c_double = 0.;
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    nz = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if fabs(*testcolumn.offset(i as isize)) > (*lp).epsvalue {
            nz += 1;
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns && ident != 0 {
        ident = nz;
        value = fabs(
            get_mat(lp, 0 as ::core::ffi::c_int, i)
                - *testcolumn.offset(0 as ::core::ffi::c_int as isize),
        );
        if !(value > (*lp).epsvalue) {
            j = *(*mat)
                .col_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            je = *(*mat).col_end.offset(i as isize);
            matRownr = (*mat).col_mat_rownr.offset(j as isize) as *mut ::core::ffi::c_int;
            matValue = (*mat).col_mat_value.offset(j as isize) as *mut ::core::ffi::c_double;
            while j < je && ident >= 0 as ::core::ffi::c_int {
                value = *matValue;
                if is_chsign(lp, *matRownr) != 0 {
                    value = if fabs(value) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        0 as ::core::ffi::c_int as ::core::ffi::c_double
                    } else {
                        -value
                    };
                }
                value = unscaled_mat(lp, value, *matRownr, i);
                value -= *testcolumn.offset(*matRownr as isize);
                if fabs(value) > (*lp).epsvalue {
                    break;
                }
                j += 1;
                ident -= 1;
                matRownr = matRownr.offset(matRowColStep as isize);
                matValue = matValue.offset(matValueStep as isize);
            }
            if ident == 0 as ::core::ffi::c_int {
                colnr = i;
            }
        }
        i += 1;
    }
    return colnr;
}
#[export_name="honest_lpsolve_set_lp_name"]
pub unsafe extern "C" fn set_lp_name(
    mut lp: *mut lprec,
    mut name: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    if name.is_null() {
        if !((*lp).lp_name as *mut ::core::ffi::c_void).is_null() {
            free((*lp).lp_name as *mut ::core::ffi::c_void);
            (*lp).lp_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*lp).lp_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        allocCHAR(
            lp,
            &raw mut (*lp).lp_name,
            strlen(name).wrapping_add(1 as size_t) as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        strcpy((*lp).lp_name, name);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_lp_name"]
pub unsafe extern "C" fn get_lp_name(mut lp: *mut lprec) -> *mut ::core::ffi::c_char {
    return if !(*lp).lp_name.is_null() {
        (*lp).lp_name
    } else {
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
    };
}
#[export_name="honest_lpsolve_init_rowcol_names"]
pub unsafe extern "C" fn init_rowcol_names(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    if (*lp).names_used == 0 {
        (*lp).row_name = calloc(
            ((*lp).rows_alloc + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<*mut hashelem>() as size_t,
        ) as *mut *mut hashelem;
        (*lp).col_name = calloc(
            ((*lp).columns_alloc + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<*mut hashelem>() as size_t,
        ) as *mut *mut hashelem;
        (*lp).rowname_hashtab = create_hash_table(
            (*lp).rows_alloc + 1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        (*lp).colname_hashtab = create_hash_table(
            (*lp).columns_alloc + 1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        (*lp).names_used = TRUE as ::core::ffi::c_uchar;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_rename_var"]
pub unsafe extern "C" fn rename_var(
    mut lp: *mut lprec,
    mut varindex: ::core::ffi::c_int,
    mut new_name: *mut ::core::ffi::c_char,
    mut list: *mut *mut hashelem,
    mut ht: *mut *mut hashtable,
) -> ::core::ffi::c_uchar {
    let mut hp: *mut hashelem = ::core::ptr::null_mut::<hashelem>();
    let mut newitem: ::core::ffi::c_uchar = 0;
    hp = *list.offset(varindex as isize);
    newitem = (hp == NULL as *mut hashelem) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if newitem != 0 {
        hp = puthash(new_name, varindex, list, *ht);
    } else if strlen((*hp).name) != strlen(new_name)
        || strcmp((*hp).name, new_name) != 0 as ::core::ffi::c_int
    {
        let mut newht: *mut hashtable = ::core::ptr::null_mut::<hashtable>();
        let mut oldht: *mut hashtable = ::core::ptr::null_mut::<hashtable>();
        allocCHAR(
            lp,
            &raw mut (*hp).name,
            strlen(new_name).wrapping_add(1 as size_t) as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        strcpy((*hp).name, new_name);
        oldht = *ht;
        newht = copy_hash_table(oldht, list, (*oldht).size);
        *ht = newht;
        free_hash_table(oldht);
    }
    return newitem;
}
#[export_name="honest_lpsolve_is_use_names"]
pub unsafe extern "C" fn is_use_names(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    if isrow != 0 {
        return (*lp).use_row_names;
    } else {
        return (*lp).use_col_names;
    };
}
#[export_name="honest_lpsolve_set_use_names"]
pub unsafe extern "C" fn set_use_names(
    mut lp: *mut lprec,
    mut isrow: ::core::ffi::c_uchar,
    mut use_names: ::core::ffi::c_uchar,
) {
    if isrow != 0 {
        (*lp).use_row_names = use_names;
    } else {
        (*lp).use_col_names = use_names;
    };
}
#[export_name="honest_lpsolve_get_nameindex"]
pub unsafe extern "C" fn get_nameindex(
    mut lp: *mut lprec,
    mut varname: *mut ::core::ffi::c_char,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if isrow != 0 {
        return find_row(lp, varname, FALSE as ::core::ffi::c_uchar);
    } else {
        return find_var(lp, varname, FALSE as ::core::ffi::c_uchar);
    };
}
#[export_name="honest_lpsolve_set_row_name"]
pub unsafe extern "C" fn set_row_name(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut new_name: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows + 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_row_name: Row %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uchar;
    }
    if rownr > (*lp).rows && append_rows(lp, rownr - (*lp).rows) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if (*lp).names_used == 0 {
        if init_rowcol_names(lp) == 0 {
            return 0 as ::core::ffi::c_uchar;
        }
    }
    rename_var(
        lp,
        rownr,
        new_name,
        (*lp).row_name,
        &raw mut (*lp).rowname_hashtab,
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_row_name"]
pub unsafe extern "C" fn get_row_name(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if rownr < 0 as ::core::ffi::c_int || rownr > (*lp).rows + 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_row_name: Row %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*(*lp).presolve_undo).var_to_orig.is_null()
        && (*lp).wasPresolved as ::core::ffi::c_int != 0
    {
        if *(*(*lp).presolve_undo).var_to_orig.offset(rownr as isize) == 0 as ::core::ffi::c_int {
            rownr = -rownr;
        } else {
            rownr = *(*(*lp).presolve_undo).var_to_orig.offset(rownr as isize);
        }
    }
    return get_origrow_name(lp, rownr);
}
#[export_name="honest_lpsolve_get_origrow_name"]
pub unsafe extern "C" fn get_origrow_name(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut newrow: ::core::ffi::c_uchar = 0;
    static mut name: [::core::ffi::c_char; 50] = [0; 50];
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    newrow = (rownr < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    rownr = abs(rownr);
    if (*lp).names_used as ::core::ffi::c_int != 0
        && (*lp).use_row_names as ::core::ffi::c_int != 0
        && !(*(*lp).row_name.offset(rownr as isize)).is_null()
        && !(**(*lp).row_name.offset(rownr as isize)).name.is_null()
    {
        ptr = (**(*lp).row_name.offset(rownr as isize)).name;
    } else {
        if (*lp).rowcol_name.is_null() {
            if allocCHAR(
                lp,
                &raw mut (*lp).rowcol_name,
                20 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) == 0
            {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
        ptr = (*lp).rowcol_name;
        if newrow != 0 {
            native_only!(snprintf,
                &raw mut name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as size_t,
                ROWNAMEMASK2.as_ptr(),
                rownr,
            );
        } else {
            native_only!(snprintf,
                &raw mut name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as size_t,
                ROWNAMEMASK.as_ptr(),
                rownr,
            );
        }
        ptr = &raw mut name as *mut ::core::ffi::c_char;
    }
    return ptr;
}
#[export_name="honest_lpsolve_set_col_name"]
pub unsafe extern "C" fn set_col_name(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut new_name: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    if colnr > (*lp).columns + 1 as ::core::ffi::c_int || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"set_col_name: Column %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    if colnr > (*lp).columns && append_columns(lp, colnr - (*lp).columns) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    if (*lp).names_used == 0 {
        init_rowcol_names(lp);
    }
    rename_var(
        lp,
        colnr,
        new_name,
        (*lp).col_name,
        &raw mut (*lp).colname_hashtab,
    );
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_col_name"]
pub unsafe extern "C" fn get_col_name(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if colnr > (*lp).columns + 1 as ::core::ffi::c_int || colnr < 1 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"get_col_name: Column %d out of range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*(*lp).presolve_undo).var_to_orig.is_null()
        && (*lp).wasPresolved as ::core::ffi::c_int != 0
    {
        if *(*(*lp).presolve_undo)
            .var_to_orig
            .offset(((*lp).rows + colnr) as isize)
            == 0 as ::core::ffi::c_int
        {
            colnr = -colnr;
        } else {
            colnr = *(*(*lp).presolve_undo)
                .var_to_orig
                .offset(((*lp).rows + colnr) as isize);
        }
    }
    return get_origcol_name(lp, colnr);
}
#[export_name="honest_lpsolve_get_origcol_name"]
pub unsafe extern "C" fn get_origcol_name(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut newcol: ::core::ffi::c_uchar = 0;
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut name: [::core::ffi::c_char; 50] = [0; 50];
    newcol = (colnr < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    colnr = abs(colnr);
    if (*lp).names_used as ::core::ffi::c_int != 0
        && (*lp).use_col_names as ::core::ffi::c_int != 0
        && !(*(*lp).col_name.offset(colnr as isize)).is_null()
        && !(**(*lp).col_name.offset(colnr as isize)).name.is_null()
    {
        ptr = (**(*lp).col_name.offset(colnr as isize)).name;
    } else {
        if (*lp).rowcol_name.is_null() {
            if allocCHAR(
                lp,
                &raw mut (*lp).rowcol_name,
                20 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            ) == 0
            {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
        ptr = (*lp).rowcol_name;
        if newcol != 0 {
            native_only!(snprintf,
                &raw mut name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as size_t,
                COLNAMEMASK2.as_ptr(),
                colnr,
            );
        } else {
            native_only!(snprintf,
                &raw mut name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as size_t,
                COLNAMEMASK.as_ptr(),
                colnr,
            );
        }
        ptr = &raw mut name as *mut ::core::ffi::c_char;
    }
    return ptr;
}
#[export_name="honest_lpsolve_MIP_count"]
pub unsafe extern "C" fn MIP_count(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).int_vars + (*lp).sc_vars + SOS_count(lp);
}
#[export_name="honest_lpsolve_bin_count"]
pub unsafe extern "C" fn bin_count(
    mut lp: *mut lprec,
    mut working: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if working != 0 {
        i = (*lp).rows + 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            if fabs(
                unscaled_value(lp, *(*lp).upbo.offset(i as isize), i)
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) < (*lp).epsvalue
            {
                n += 1;
            }
            i += 1;
        }
    } else {
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).columns {
            if fabs(get_upbo(lp, i) - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                < (*lp).epsvalue
                && fabs(get_lowbo(lp, i) - 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    < (*lp).epsvalue
            {
                n += 1;
            }
            i += 1;
        }
    }
    return n;
}
#[export_name="honest_lpsolve_SOS_count"]
pub unsafe extern "C" fn SOS_count(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).SOS.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return (*(*lp).SOS).sos_count;
    };
}
#[export_name="honest_lpsolve_GUB_count"]
pub unsafe extern "C" fn GUB_count(mut lp: *mut lprec) -> ::core::ffi::c_int {
    if (*lp).GUB.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return (*(*lp).GUB).sos_count;
    };
}
#[export_name="honest_lpsolve_compute_violation"]
pub unsafe extern "C" fn compute_violation(
    mut lp: *mut lprec,
    mut row_nr: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    let mut test: ::core::ffi::c_double = 0.;
    value = *(*lp).rhs.offset(row_nr as isize);
    row_nr = *(*lp).var_basic.offset(row_nr as isize);
    test = value
        - (if 0 as ::core::ffi::c_int != 0 {
            *(*lp).lowbo.offset(row_nr as isize)
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        });
    if fabs(test) < (*lp).epsprimal {
        test = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if test > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        test = value - *(*lp).upbo.offset(row_nr as isize);
        if fabs(test) < (*lp).epsprimal {
            test = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if test < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            test = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    return test;
}
#[export_name="honest_lpsolve_feasibilityOffset"]
pub unsafe extern "C" fn feasibilityOffset(
    mut lp: *mut lprec,
    mut isdual: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut Extra: ::core::ffi::c_double = 0.;
    Extra = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if isdual != 0 {
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).columns {
            f = *(*lp).orig_obj.offset(i as isize);
            if f < Extra {
                Extra = f;
            }
            i += 1;
        }
    } else {
        Extra = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        j = 0 as ::core::ffi::c_int;
        Extra = (*lp).infinite;
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            f = *(*lp).rhs.offset(i as isize);
            if f < Extra {
                Extra = f;
                j = i;
            }
            i += 1;
        }
        Extra = j as ::core::ffi::c_double;
    }
    return Extra;
}
#[export_name="honest_lpsolve_compute_dualslacks"]
pub unsafe extern "C" fn compute_dualslacks(
    mut lp: *mut lprec,
    mut target: ::core::ffi::c_int,
    mut dvalues: *mut *mut ::core::ffi::c_double,
    mut nzdvalues: *mut *mut ::core::ffi::c_int,
    mut dosum: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut varnr: ::core::ffi::c_int = 0;
    let mut coltarget: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nzduals: *mut *mut ::core::ffi::c_int =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
    let mut nzvtemp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut d: ::core::ffi::c_double = 0.;
    let mut g: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut duals: *mut *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    let mut vtemp: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut localREAL: ::core::ffi::c_uchar = (dvalues == NULL as *mut *mut ::core::ffi::c_double)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    let mut localINT: ::core::ffi::c_uchar = (nzdvalues == NULL as *mut *mut ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if is_action((*lp).spx_action, ACTION_REBASE) as ::core::ffi::c_int != 0
        || is_action((*lp).spx_action, ACTION_REINVERT) as ::core::ffi::c_int != 0
        || (*lp).basis_valid == 0
    {
        return g;
    }
    if localREAL == 0 {
        duals = dvalues;
        nzduals = nzdvalues;
    } else {
        duals = &raw mut vtemp;
        nzduals = &raw mut nzvtemp;
    }
    if localINT as ::core::ffi::c_int != 0 || (*nzduals).is_null() {
        allocINT(
            lp,
            nzduals,
            (*lp).columns + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
    }
    if localREAL as ::core::ffi::c_int != 0 || (*duals).is_null() {
        allocREAL(
            lp,
            duals,
            (*lp).sum + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
    }
    if target == 0 as ::core::ffi::c_int {
        target = SCAN_ALLVARS + USE_NONBASICVARS;
    }
    coltarget = mempool_obtainVector(
        (*lp).workarrays,
        (*lp).columns + 1 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
    ) as *mut ::core::ffi::c_int;
    if get_colIndexA(lp, target, coltarget, FALSE as ::core::ffi::c_uchar) == 0 {
        mempool_releaseVector(
            (*lp).workarrays,
            coltarget as *mut ::core::ffi::c_char,
            FALSE as ::core::ffi::c_uchar,
        );
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    bsolve(
        lp,
        0 as ::core::ffi::c_int,
        *duals,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        (*lp).epsmachine * DOUBLEROUND,
        1.0f64,
    );
    prod_xA(
        lp,
        coltarget,
        *duals,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        (*lp).epsmachine,
        1.0f64,
        *duals,
        *nzduals,
        MAT_ROUNDDEFAULT | MAT_ROUNDRC,
    );
    mempool_releaseVector(
        (*lp).workarrays,
        coltarget as *mut ::core::ffi::c_char,
        FALSE as ::core::ffi::c_uchar,
    );
    i = 1 as ::core::ffi::c_int;
    while i <= *(*nzduals).offset(0 as ::core::ffi::c_int as isize) {
        varnr = *(*nzduals).offset(i as isize);
        d = if *(*lp).is_lower.offset(varnr as isize) == 0
            && *(*duals).offset(varnr as isize) != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -*(*duals).offset(varnr as isize)
        } else {
            *(*duals).offset(varnr as isize)
        };
        if d < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            if dosum != 0 {
                g += -d;
            } else if g > d {
                g = d;
            }
        }
        i += 1;
    }
    if localREAL != 0 {
        if !(*duals as *mut ::core::ffi::c_void).is_null() {
            free(*duals as *mut ::core::ffi::c_void);
            *duals = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    }
    if localINT != 0 {
        if !(*nzduals as *mut ::core::ffi::c_void).is_null() {
            free(*nzduals as *mut ::core::ffi::c_void);
            *nzduals = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    return g;
}
#[export_name="honest_lpsolve_compute_feasibilitygap"]
pub unsafe extern "C" fn compute_feasibilitygap(
    mut lp: *mut lprec,
    mut isdual: ::core::ffi::c_uchar,
    mut dosum: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut f: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if isdual != 0 {
        let mut i: ::core::ffi::c_int = 0;
        let mut g: ::core::ffi::c_double = 0.;
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            if *(*lp).rhs.offset(i as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                g = *(*lp).rhs.offset(i as isize);
            } else if *(*lp).rhs.offset(i as isize)
                > *(*lp)
                    .upbo
                    .offset(*(*lp).var_basic.offset(i as isize) as isize)
            {
                g = *(*lp).rhs.offset(i as isize)
                    - *(*lp)
                        .upbo
                        .offset(*(*lp).var_basic.offset(i as isize) as isize);
            } else {
                g = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if dosum != 0 {
                f += g;
            } else if f < g {
                f = g;
            }
            i += 1;
        }
    } else {
        f = compute_dualslacks(
            lp,
            SCAN_USERVARS + USE_ALLVARS,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
            dosum,
        );
    }
    return f;
}
#[export_name="honest_lpsolve_row_decimals"]
pub unsafe extern "C" fn row_decimals(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut intsonly: ::core::ffi::c_uchar,
    mut intscalar: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut basi: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ncols: ::core::ffi::c_int = (*lp).columns;
    let mut f: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*lp).epsprimal;
    basi = 0 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    while j <= ncols {
        if intsonly as ::core::ffi::c_int != 0 && is_int(lp, j) == 0 {
            if intsonly as ::core::ffi::c_int == TRUE {
                break;
            }
        } else {
            f = fabs(get_mat(lp, rownr, j));
            f -= floor(f + epsvalue);
            i = 0 as ::core::ffi::c_int;
            while i <= MAX_FRACSCALE && f > epsvalue {
                f *= 10 as ::core::ffi::c_int as ::core::ffi::c_double;
                f -= floor(f + epsvalue);
                i += 1;
            }
            if i > MAX_FRACSCALE {
                break;
            }
            if basi < i {
                basi = i;
            }
        }
        j += 1;
    }
    if j > ncols {
        *intscalar = pow(10.0f64, basi as ::core::ffi::c_double);
    } else {
        basi = -(1 as ::core::ffi::c_int);
        *intscalar = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    return basi;
}
#[export_name="honest_lpsolve_row_intstats"]
pub unsafe extern "C" fn row_intstats(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut pivcolnr: ::core::ffi::c_int,
    mut maxndec: *mut ::core::ffi::c_int,
    mut plucount: *mut ::core::ffi::c_int,
    mut intcount: *mut ::core::ffi::c_int,
    mut intval: *mut ::core::ffi::c_int,
    mut valGCD: *mut ::core::ffi::c_double,
    mut pivcolval: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut jb: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    let mut nn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut multA: ::core::ffi::c_int = 0;
    let mut multB: ::core::ffi::c_int = 0;
    let mut intGCD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rowval: ::core::ffi::c_double = 0.;
    let mut inthold: ::core::ffi::c_double = 0.;
    let mut intfrac: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    if mat_validate(mat) != 0 {
        *maxndec = row_decimals(
            lp,
            rownr,
            AUTOMATIC as ::core::ffi::c_uchar,
            &raw mut intfrac,
        );
        if rownr == 0 as ::core::ffi::c_int {
            jb = 1 as ::core::ffi::c_int;
            je = (*lp).columns + 1 as ::core::ffi::c_int;
        } else {
            jb = *(*mat)
                .row_end
                .offset((rownr - 1 as ::core::ffi::c_int) as isize);
            je = *(*mat).row_end.offset(rownr as isize);
        }
        nn = je - jb;
        *pivcolval = 1.0f64;
        *plucount = 0 as ::core::ffi::c_int;
        *intcount = 0 as ::core::ffi::c_int;
        *intval = 0 as ::core::ffi::c_int;
        let mut current_block_33: u64;
        while jb < je {
            if rownr == 0 as ::core::ffi::c_int {
                if *(*lp).orig_obj.offset(jb as isize)
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    nn -= 1;
                    current_block_33 = 17965632435239708295;
                } else {
                    jj = jb;
                    current_block_33 = 12039483399334584727;
                }
            } else {
                jj = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                current_block_33 = 12039483399334584727;
            }
            match current_block_33 {
                12039483399334584727 => {
                    if jj == pivcolnr {
                        if rownr == 0 as ::core::ffi::c_int {
                            *pivcolval = unscaled_mat(
                                lp,
                                *(*lp).orig_obj.offset(jb as isize),
                                0 as ::core::ffi::c_int,
                                jb,
                            );
                        } else {
                            *pivcolval = get_mat_byindex(
                                lp,
                                jb,
                                TRUE as ::core::ffi::c_uchar,
                                FALSE as ::core::ffi::c_uchar,
                            );
                        }
                    } else if !(is_int(lp, jj) == 0) {
                        *intcount += 1;
                        if rownr == 0 as ::core::ffi::c_int {
                            rowval = unscaled_mat(
                                lp,
                                *(*lp).orig_obj.offset(jb as isize),
                                0 as ::core::ffi::c_int,
                                jb,
                            );
                        } else {
                            rowval = get_mat_byindex(
                                lp,
                                jb,
                                TRUE as ::core::ffi::c_uchar,
                                FALSE as ::core::ffi::c_uchar,
                            );
                        }
                        if rowval > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            *plucount += 1;
                        }
                        rowval = fabs(rowval) * intfrac;
                        rowval += rowval * (*lp).epsmachine;
                        rowval = modf(rowval, &raw mut inthold);
                        if rowval < (*lp).epsprimal {
                            *intval += 1;
                            if *intval == 1 as ::core::ffi::c_int {
                                intGCD = inthold as ::core::ffi::c_int;
                            } else {
                                intGCD = gcd(
                                    intGCD as ::core::ffi::c_longlong,
                                    inthold as ::core::ffi::c_longlong,
                                    &raw mut multA,
                                    &raw mut multB,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
            jb += 1;
        }
        *valGCD = intGCD as ::core::ffi::c_double;
        *valGCD /= intfrac;
    }
    return nn;
}
#[export_name="honest_lpsolve_MIP_stepOF"]
pub unsafe extern "C" fn MIP_stepOF(mut lp: *mut lprec) -> ::core::ffi::c_double {
    let mut OFgcd: ::core::ffi::c_uchar = 0;
    let mut colnr: ::core::ffi::c_int = 0;
    let mut rownr: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ib: ::core::ffi::c_int = 0;
    let mut ie: ::core::ffi::c_int = 0;
    let mut pluscount: ::core::ffi::c_int = 0;
    let mut intcount: ::core::ffi::c_int = 0;
    let mut intval: ::core::ffi::c_int = 0;
    let mut maxndec: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut valOF: ::core::ffi::c_double = 0.;
    let mut divOF: ::core::ffi::c_double = 0.;
    let mut valGCD: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    if (*lp).int_vars > 0 as ::core::ffi::c_int
        && (*lp).solutionlimit == 1 as ::core::ffi::c_int
        && mat_validate(mat) as ::core::ffi::c_int != 0
    {
        n = row_intstats(
            lp,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut maxndec,
            &raw mut pluscount,
            &raw mut intcount,
            &raw mut intval,
            &raw mut valGCD,
            &raw mut divOF,
        );
        if n == 0 as ::core::ffi::c_int || maxndec < 0 as ::core::ffi::c_int {
            return value;
        }
        OFgcd = (intval > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        if OFgcd != 0 {
            value = valGCD;
        }
        if n - intcount > 0 as ::core::ffi::c_int {
            let mut nrv: ::core::ffi::c_int = n - intcount;
            let mut niv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nrows: ::core::ffi::c_int = (*lp).rows;
            ib = 1 as ::core::ffi::c_int;
            while ib <= nrows {
                if is_constr_type(lp, ib, EQ) != 0 {
                    break;
                }
                ib += 1;
            }
            if ib < nrows {
                colnr = 1 as ::core::ffi::c_int;
                while colnr <= (*lp).columns {
                    if !(*(*lp).orig_obj.offset(colnr as isize)
                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        || is_int(lp, colnr) as ::core::ffi::c_int != 0)
                    {
                        ib = *(*mat)
                            .col_end
                            .offset((colnr - 1 as ::core::ffi::c_int) as isize);
                        ie = *(*mat).col_end.offset(colnr as isize);
                        while ib < ie {
                            rownr = *(*mat).col_mat_rownr.offset(ib as isize);
                            if is_constr_type(lp, rownr, EQ) != 0 {
                                n = row_intstats(
                                    lp,
                                    rownr,
                                    colnr,
                                    &raw mut maxndec,
                                    &raw mut pluscount,
                                    &raw mut intcount,
                                    &raw mut intval,
                                    &raw mut valGCD,
                                    &raw mut divOF,
                                );
                                if intval < n - 1 as ::core::ffi::c_int
                                    || maxndec < 0 as ::core::ffi::c_int
                                {
                                    value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                    break;
                                } else {
                                    niv += 1;
                                    valOF = unscaled_mat(
                                        lp,
                                        *(*lp).orig_obj.offset(colnr as isize),
                                        0 as ::core::ffi::c_int,
                                        colnr,
                                    );
                                    valOF = fabs(valOF * (valGCD / divOF));
                                    if OFgcd != 0 {
                                        if value > valOF {
                                            value = valOF;
                                        }
                                    } else {
                                        OFgcd = TRUE as ::core::ffi::c_uchar;
                                        value = valOF;
                                    }
                                }
                            }
                            ib += 1;
                        }
                        if value == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            break;
                        }
                    }
                    colnr += 1;
                }
            }
            if nrv > niv {
                value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
    }
    return value;
}
#[export_name="honest_lpsolve_isPrimalSimplex"]
pub unsafe extern "C" fn isPrimalSimplex(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return ((*lp).simplex_mode & SIMPLEX_Phase1_PRIMAL != 0 as ::core::ffi::c_int
        || (*lp).simplex_mode & SIMPLEX_Phase2_PRIMAL != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_isPhase1"]
pub unsafe extern "C" fn isPhase1(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return ((*lp).simplex_mode & SIMPLEX_Phase1_PRIMAL != 0 as ::core::ffi::c_int
        || (*lp).simplex_mode & SIMPLEX_Phase1_DUAL != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_isP1extra"]
pub unsafe extern "C" fn isP1extra(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return ((*lp).P1extraDim > 0 as ::core::ffi::c_int
        || (*lp).P1extraVal != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_feasiblePhase1"]
pub unsafe extern "C" fn feasiblePhase1(
    mut lp: *mut lprec,
    mut epsvalue: ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut gap: ::core::ffi::c_double = 0.;
    let mut test: ::core::ffi::c_uchar = 0;
    gap = fabs(
        *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize)
            - *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize),
    );
    test = (gap < epsvalue) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    return test;
}
#[export_name="honest_lpsolve_isDegenerateBasis"]
pub unsafe extern "C" fn isDegenerateBasis(
    mut lp: *mut lprec,
    mut basisvar: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut varindex: ::core::ffi::c_int = 0;
    varindex = *(*lp).var_basic.offset(basisvar as isize);
    if fabs(*(*lp).rhs.offset(basisvar as isize)) < (*lp).epsprimal
        || fabs(*(*lp).upbo.offset(varindex as isize) - *(*lp).rhs.offset(basisvar as isize))
            < (*lp).epsprimal
    {
        return 1 as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_findBasicFixedvar"]
pub unsafe extern "C" fn findBasicFixedvar(
    mut lp: *mut lprec,
    mut afternr: ::core::ffi::c_int,
    mut slacksonly: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut varnr: ::core::ffi::c_int = 0;
    let mut delta: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if afternr < 0 as ::core::ffi::c_int {
        delta = -(1 as ::core::ffi::c_int);
        afternr = -afternr;
    }
    afternr += delta;
    if afternr < 1 as ::core::ffi::c_int || afternr > (*lp).rows {
        return 0 as ::core::ffi::c_int;
    }
    while afternr > 0 as ::core::ffi::c_int && afternr <= (*lp).rows {
        varnr = *(*lp).var_basic.offset(afternr as isize);
        if varnr <= (*lp).rows && is_constr_type(lp, varnr, EQ) as ::core::ffi::c_int != 0
            || slacksonly == 0
                && varnr > (*lp).rows
                && is_fixedvar(lp, varnr) as ::core::ffi::c_int != 0
        {
            break;
        }
        afternr += delta;
    }
    if afternr > (*lp).rows {
        afternr = 0 as ::core::ffi::c_int;
    }
    return afternr;
}
#[export_name="honest_lpsolve_isBasisVarFeasible"]
pub unsafe extern "C" fn isBasisVarFeasible(
    mut lp: *mut lprec,
    mut tol: ::core::ffi::c_double,
    mut basis_row: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut col: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut Ok_0: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut doSC: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    col = *(*lp).var_basic.offset(basis_row as isize);
    x = *(*lp).rhs.offset(basis_row as isize);
    if x < -tol || x > *(*lp).upbo.offset(col as isize) + tol {
        Ok_0 = FALSE as ::core::ffi::c_uchar;
    } else if doSC as ::core::ffi::c_int != 0
        && col > (*lp).rows
        && fabs(*(*lp).sc_lobound.offset((col - (*lp).rows) as isize))
            > 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if x > tol && x < fabs(*(*lp).sc_lobound.offset((col - (*lp).rows) as isize)) - tol {
            Ok_0 = FALSE as ::core::ffi::c_uchar;
        }
    }
    return Ok_0;
}
#[export_name="honest_lpsolve_isPrimalFeasible"]
pub unsafe extern "C" fn isPrimalFeasible(
    mut lp: *mut lprec,
    mut tol: ::core::ffi::c_double,
    mut infeasibles: *mut ::core::ffi::c_int,
    mut feasibilitygap: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut feasible: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    let mut rhsptr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut idxptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if !infeasibles.is_null() {
        *infeasibles.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    rhsptr = (*lp).rhs.offset(1 as ::core::ffi::c_int as isize);
    idxptr = (*lp).var_basic.offset(1 as ::core::ffi::c_int as isize);
    while i <= (*lp).rows {
        feasible = TRUE as ::core::ffi::c_uchar;
        if *rhsptr < -tol || *rhsptr > *(*lp).upbo.offset(*idxptr as isize) + tol {
            feasible = FALSE as ::core::ffi::c_uchar;
        }
        if feasible == 0 {
            if infeasibles.is_null() {
                break;
            }
            let ref mut fresh52 = *infeasibles.offset(0 as ::core::ffi::c_int as isize);
            *fresh52 += 1;
            *infeasibles.offset(*infeasibles.offset(0 as ::core::ffi::c_int as isize) as isize) = i;
        }
        i += 1;
        rhsptr = rhsptr.offset(1);
        idxptr = idxptr.offset(1);
    }
    if !feasibilitygap.is_null() {
        if feasible != 0 {
            *feasibilitygap = 0.0f64;
        } else {
            *feasibilitygap = feasibilityOffset(lp, FALSE as ::core::ffi::c_uchar);
        }
    }
    return feasible;
}
#[export_name="honest_lpsolve_isDualFeasible"]
pub unsafe extern "C" fn isDualFeasible(
    mut lp: *mut lprec,
    mut tol: ::core::ffi::c_double,
    mut boundflipcount: *mut ::core::ffi::c_int,
    mut infeasibles: *mut ::core::ffi::c_int,
    mut feasibilitygap: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut varnr: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut target: ::core::ffi::c_int = SCAN_ALLVARS + USE_NONBASICVARS;
    let mut f: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut feasible: ::core::ffi::c_uchar = 0;
    let mut islower: ::core::ffi::c_uchar = 0;
    if !infeasibles.is_null() || !boundflipcount.is_null() {
        let mut nzdcol: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut d: ::core::ffi::c_double = 0.;
        let mut dcol: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
        f = compute_dualslacks(
            lp,
            target,
            &raw mut dcol,
            &raw mut nzdcol,
            FALSE as ::core::ffi::c_uchar,
        );
        if !nzdcol.is_null() {
            i = 1 as ::core::ffi::c_int;
            while i <= *nzdcol.offset(0 as ::core::ffi::c_int as isize) {
                varnr = *nzdcol.offset(i as isize);
                islower = *(*lp).is_lower.offset(varnr as isize);
                d = if islower == 0
                    && *dcol.offset(varnr as isize)
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -*dcol.offset(varnr as isize)
                } else {
                    *dcol.offset(varnr as isize)
                };
                if !(d > -tol
                    || *(*lp).upbo.offset(varnr as isize) >= (*lp).infinite
                        && *(*lp).lowbo.offset(varnr as isize) <= -(*lp).infinite
                    || is_fixedvar(lp, varnr) as ::core::ffi::c_int != 0)
                {
                    if boundflipcount.is_null()
                        || (*lp).bb_level <= 1 as ::core::ffi::c_int
                            && *(*lp).upbo.offset(varnr as isize) > fabs((*lp).negrange)
                        || islower as ::core::ffi::c_int != 0
                            && (fabs(*(*lp).upbo.offset(varnr as isize)) >= (*lp).infinite)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                != 0
                        || islower == 0
                            && (fabs(0.0f64) >= (*lp).infinite) as ::core::ffi::c_int
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                != 0
                    {
                        m += 1;
                        if !infeasibles.is_null() {
                            *infeasibles.offset(m as isize) = varnr;
                        }
                    } else {
                        *(*lp).is_lower.offset(varnr as isize) =
                            (islower == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        n += 1;
                    }
                }
                i += 1;
            }
        }
        if !infeasibles.is_null() {
            *infeasibles.offset(0 as ::core::ffi::c_int as isize) = m;
        }
        if !(dcol as *mut ::core::ffi::c_void).is_null() {
            free(dcol as *mut ::core::ffi::c_void);
            dcol = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !(nzdcol as *mut ::core::ffi::c_void).is_null() {
            free(nzdcol as *mut ::core::ffi::c_void);
            nzdcol = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        if n > 0 as ::core::ffi::c_int {
            set_action(&raw mut (*lp).spx_action, ACTION_RECOMPUTE);
            if m == 0 as ::core::ffi::c_int {
                f = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
    } else {
        f = compute_dualslacks(
            lp,
            target,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_double>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
            FALSE as ::core::ffi::c_uchar,
        );
    }
    varnr = (*lp).rows + 1 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if mat_collength((*lp).matA, i) == 0 as ::core::ffi::c_int {
            islower = *(*lp).is_lower.offset(varnr as isize);
            if (if islower as ::core::ffi::c_int != 0
                && *(*lp).orig_obj.offset(i as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -*(*lp).orig_obj.offset(i as isize)
            } else {
                *(*lp).orig_obj.offset(i as isize)
            }) > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, i) == 0
            {
                *(*lp).is_lower.offset(varnr as isize) =
                    (islower == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
                if islower as ::core::ffi::c_int != 0
                    && (fabs(*(*lp).upbo.offset(varnr as isize)) >= (*lp).infinite)
                        as ::core::ffi::c_int as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        != 0
                    || islower == 0
                        && (fabs(0.0f64) >= (*lp).infinite) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar as ::core::ffi::c_int
                            != 0
                {
                    (*lp).spx_status = UNBOUNDED;
                    break;
                } else {
                    n += 1;
                }
            }
        }
        i += 1;
        varnr += 1;
    }
    if !boundflipcount.is_null() {
        *boundflipcount = n;
    }
    if !feasibilitygap.is_null() {
        if fabs(f) < tol {
            f = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        *feasibilitygap = f;
    }
    feasible = (f == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && m == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
    return feasible;
}
#[export_name="honest_lpsolve_default_basis"]
pub unsafe extern "C" fn default_basis(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        *(*lp).var_basic.offset(i as isize) = i;
        *(*lp).is_basic.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        i += 1;
    }
    *(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) = TRUE;
    while i <= (*lp).sum {
        *(*lp).is_basic.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        i += 1;
    }
    *(*lp).is_lower.offset(0 as ::core::ffi::c_int as isize) = TRUE as ::core::ffi::c_uchar;
    set_action(
        &raw mut (*lp).spx_action,
        ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
    );
    (*lp).basis_valid = TRUE as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_basiscrash"]
pub unsafe extern "C" fn get_basiscrash(mut lp: *mut lprec) -> ::core::ffi::c_int {
    return (*lp).crashmode;
}
#[export_name="honest_lpsolve_set_basiscrash"]
pub unsafe extern "C" fn set_basiscrash(mut lp: *mut lprec, mut mode: ::core::ffi::c_int) {
    (*lp).crashmode = mode;
}
#[export_name="honest_lpsolve_set_basis"]
pub unsafe extern "C" fn set_basis(
    mut lp: *mut lprec,
    mut bascolumn: *mut ::core::ffi::c_int,
    mut nonbasic: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if (*lp).wasPresolved as ::core::ffi::c_int != 0
        && ((*lp).rows != (*(*lp).presolve_undo).orig_rows
            || (*lp).columns != (*(*lp).presolve_undo).orig_columns)
    {
        return 0 as ::core::ffi::c_uchar;
    }
    *(*lp).is_lower.offset(0 as ::core::ffi::c_int as isize) = TRUE as ::core::ffi::c_uchar;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        *(*lp).is_lower.offset(i as isize) = TRUE as ::core::ffi::c_uchar;
        *(*lp).is_basic.offset(i as isize) = FALSE as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        *(*lp).var_basic.offset(i as isize) = FALSE;
        i += 1;
    }
    if nonbasic != 0 {
        n = (*lp).sum;
    } else {
        n = (*lp).rows;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        s = *bascolumn.offset(i as isize);
        k = abs(s);
        if k <= 0 as ::core::ffi::c_int || k > (*lp).sum {
            return 0 as ::core::ffi::c_uchar;
        }
        if i <= (*lp).rows {
            *(*lp).var_basic.offset(i as isize) = k;
            *(*lp).is_basic.offset(k as isize) = TRUE as ::core::ffi::c_uchar;
        } else if s > 0 as ::core::ffi::c_int {
            *(*lp).is_lower.offset(k as isize) = FALSE as ::core::ffi::c_uchar;
        }
        i += 1;
    }
    if verify_basis(lp) == 0 {
        return 0 as ::core::ffi::c_uchar;
    }
    set_action(
        &raw mut (*lp).spx_action,
        ACTION_REBASE | ACTION_REINVERT | ACTION_RECOMPUTE,
    );
    (*lp).basis_valid = TRUE as ::core::ffi::c_uchar;
    *(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) = FALSE;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_reset_basis"]
pub unsafe extern "C" fn reset_basis(mut lp: *mut lprec) {
    (*lp).basis_valid = FALSE as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_basis"]
pub unsafe extern "C" fn get_basis(
    mut lp: *mut lprec,
    mut bascolumn: *mut ::core::ffi::c_int,
    mut nonbasic: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).basis_valid == 0
        || (*lp).rows != (*(*lp).presolve_undo).orig_rows
        || (*lp).columns != (*(*lp).presolve_undo).orig_columns
    {
        return 0 as ::core::ffi::c_uchar;
    }
    *bascolumn = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        k = *(*lp).var_basic.offset(i as isize);
        *bascolumn.offset(i as isize) = if *(*lp).is_lower.offset(k as isize) as ::core::ffi::c_int
            != 0
            && k != 0 as ::core::ffi::c_int
        {
            -k
        } else {
            k
        };
        i += 1;
    }
    if nonbasic != 0 {
        k = 1 as ::core::ffi::c_int;
        while k <= (*lp).sum && i <= (*lp).sum {
            if !(*(*lp).is_basic.offset(k as isize) != 0) {
                *bascolumn.offset(i as isize) =
                    if *(*lp).is_lower.offset(k as isize) as ::core::ffi::c_int != 0
                        && k != 0 as ::core::ffi::c_int
                    {
                        -k
                    } else {
                        k
                    };
                i += 1;
            }
            k += 1;
        }
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_BasisReady"]
pub unsafe extern "C" fn is_BasisReady(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return (*(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) != AUTOMATIC)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_is_slackbasis"]
pub unsafe extern "C" fn is_slackbasis(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*lp).basis_valid != 0 {
        let mut i: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut used: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        allocMYBOOL(
            lp,
            &raw mut used,
            (*lp).rows + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            k = *(*lp).var_basic.offset(i as isize);
            if k <= (*lp).rows {
                if *used.offset(k as isize) != 0 {
                    err += 1;
                } else {
                    *used.offset(k as isize) = TRUE as ::core::ffi::c_uchar;
                }
                n += 1;
            }
            i += 1;
        }
        if !(used as *mut ::core::ffi::c_void).is_null() {
            free(used as *mut ::core::ffi::c_void);
            used = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if err > 0 as ::core::ffi::c_int {
            report(
                lp,
                2 as ::core::ffi::c_int,
                b"is_slackbasis: %d inconsistencies found in slack basis\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    return (n == (*lp).rows) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_verify_basis"]
pub unsafe extern "C" fn verify_basis(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    i = 1 as ::core::ffi::c_int;
    loop {
        if !(i <= (*lp).rows) {
            current_block = 6873731126896040597;
            break;
        }
        ii = *(*lp).var_basic.offset(i as isize);
        if ii < 1 as ::core::ffi::c_int
            || ii > (*lp).sum
            || *(*lp).is_basic.offset(ii as isize) == 0
        {
            ii = 0 as ::core::ffi::c_int;
            current_block = 4129979245367713660;
            break;
        } else {
            i += 1;
        }
    }
    match current_block {
        6873731126896040597 => {
            ii = (*lp).rows;
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).sum {
                if *(*lp).is_basic.offset(i as isize) != 0 {
                    ii -= 1;
                }
                i += 1;
            }
            result = (ii == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        }
        _ => {}
    }
    return result;
}
#[export_name="honest_lpsolve_set_basisvar"]
pub unsafe extern "C" fn set_basisvar(
    mut lp: *mut lprec,
    mut basisPos: ::core::ffi::c_int,
    mut enteringCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut leavingCol: ::core::ffi::c_int = 0;
    leavingCol = *(*lp).var_basic.offset(basisPos as isize);
    *(*lp).var_basic.offset(0 as ::core::ffi::c_int as isize) = FALSE;
    *(*lp).var_basic.offset(basisPos as isize) = enteringCol;
    *(*lp).is_basic.offset(leavingCol as isize) = FALSE as ::core::ffi::c_uchar;
    *(*lp).is_basic.offset(enteringCol as isize) = TRUE as ::core::ffi::c_uchar;
    if !(*lp).bb_basis.is_null() {
        (*(*lp).bb_basis).pivots += 1;
    }
    return leavingCol;
}
#[export_name="honest_lpsolve_perturb_bounds"]
pub unsafe extern "C" fn perturb_bounds(
    mut lp: *mut lprec,
    mut perturbed: *mut BBrec,
    mut doRows: ::core::ffi::c_uchar,
    mut doCols: ::core::ffi::c_uchar,
    mut includeFIXED: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut new_lb: ::core::ffi::c_double = 0.;
    let mut new_ub: ::core::ffi::c_double = 0.;
    let mut upbo: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut lowbo: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if perturbed.is_null() {
        return n;
    }
    upbo = (*perturbed).upbo;
    lowbo = (*perturbed).lowbo;
    i = 1 as ::core::ffi::c_int;
    ii = (*lp).rows;
    if doRows == 0 {
        i += ii;
    }
    if doCols == 0 {
        ii = (*lp).sum;
    }
    while i <= ii {
        if !(i <= (*lp).rows
            && *lowbo.offset(i as isize) == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && *upbo.offset(i as isize) >= (*lp).infinite)
        {
            new_lb = *lowbo.offset(i as isize);
            new_ub = *upbo.offset(i as isize);
            if !(includeFIXED == 0 && new_ub == new_lb) {
                if i > (*lp).rows && new_lb < (*lp).infinite {
                    new_lb = rand_uniform(lp, RANDSCALE as ::core::ffi::c_double)
                        + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    new_lb *= (*lp).epsperturb;
                    *lowbo.offset(i as isize) -= new_lb;
                    n += 1;
                }
                if new_ub < (*lp).infinite {
                    new_ub = rand_uniform(lp, RANDSCALE as ::core::ffi::c_double)
                        + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    new_ub *= (*lp).epsperturb;
                    *upbo.offset(i as isize) += new_ub;
                    n += 1;
                }
            }
        }
        i += 1;
    }
    set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
    return n;
}
#[export_name="honest_lpsolve_impose_bounds"]
pub unsafe extern "C" fn impose_bounds(
    mut lp: *mut lprec,
    mut upbo: *mut ::core::ffi::c_double,
    mut lowbo: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = 0;
    ok = (!upbo.is_null() || !lowbo.is_null()) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if ok != 0 {
        if !upbo.is_null() && upbo != (*lp).upbo {
            memcpy(
                (*lp).upbo as *mut ::core::ffi::c_void,
                upbo as *const ::core::ffi::c_void,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
        if !lowbo.is_null() && lowbo != (*lp).lowbo {
            memcpy(
                (*lp).lowbo as *mut ::core::ffi::c_void,
                lowbo as *const ::core::ffi::c_void,
                (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            );
        }
        if !(*lp).bb_bounds.is_null() {
            (*(*lp).bb_bounds).UBzerobased = FALSE as ::core::ffi::c_uchar;
        }
        set_action(&raw mut (*lp).spx_action, ACTION_REBASE);
    }
    set_action(&raw mut (*lp).spx_action, ACTION_RECOMPUTE);
    return ok;
}
#[export_name="honest_lpsolve_validate_bounds"]
pub unsafe extern "C" fn validate_bounds(
    mut lp: *mut lprec,
    mut upbo: *mut ::core::ffi::c_double,
    mut lowbo: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    ok = (!upbo.is_null() || !lowbo.is_null()) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if ok != 0 {
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            if *lowbo.offset(i as isize) > *upbo.offset(i as isize)
                || *lowbo.offset(i as isize) < *(*lp).orig_lowbo.offset(i as isize)
                || *upbo.offset(i as isize) > *(*lp).orig_upbo.offset(i as isize)
            {
                break;
            }
            i += 1;
        }
        ok = (i > (*lp).sum) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    return ok;
}
#[export_name="honest_lpsolve_unload_BB"]
pub unsafe extern "C" fn unload_BB(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut levelsunloaded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*lp).bb_bounds.is_null() {
        while !pop_BB((*lp).bb_bounds).is_null() {
            levelsunloaded += 1;
        }
    }
    return levelsunloaded;
}
#[export_name="honest_lpsolve_push_basis"]
pub unsafe extern "C" fn push_basis(
    mut lp: *mut lprec,
    mut basisvar: *mut ::core::ffi::c_int,
    mut isbasic: *mut ::core::ffi::c_uchar,
    mut islower: *mut ::core::ffi::c_uchar,
) -> *mut basisrec {
    let mut sum: ::core::ffi::c_int = (*lp).sum + 1 as ::core::ffi::c_int;
    let mut newbasis: *mut basisrec = ::core::ptr::null_mut::<basisrec>();
    newbasis = calloc(::core::mem::size_of::<basisrec>() as size_t, 1 as size_t) as *mut basisrec;
    if !newbasis.is_null()
        && allocMYBOOL(
            lp,
            &raw mut (*newbasis).is_lower,
            (sum + 8 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0
        && allocINT(
            lp,
            &raw mut (*newbasis).var_basic,
            (*lp).rows + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0
    {
        if islower.is_null() {
            islower = (*lp).is_lower;
        }
        if isbasic.is_null() {
            isbasic = (*lp).is_basic;
        }
        if basisvar.is_null() {
            basisvar = (*lp).var_basic;
        }
        sum = 1 as ::core::ffi::c_int;
        while sum <= (*lp).sum {
            if *islower.offset(sum as isize) != 0 {
                set_biton((*newbasis).is_lower, sum);
            }
            sum += 1;
        }
        memcpy(
            (*newbasis).var_basic as *mut ::core::ffi::c_void,
            basisvar as *const ::core::ffi::c_void,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        (*newbasis).previous = (*lp).bb_basis as *mut _basisrec;
        if (*lp).bb_basis.is_null() {
            (*newbasis).level = 0 as ::core::ffi::c_int;
        } else {
            (*newbasis).level = (*(*lp).bb_basis).level + 1 as ::core::ffi::c_int;
        }
        (*newbasis).pivots = 0 as ::core::ffi::c_int;
        (*lp).bb_basis = newbasis;
    }
    return newbasis;
}
#[export_name="honest_lpsolve_compare_basis"]
pub unsafe extern "C" fn compare_basis(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut same_basis: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    if (*lp).bb_basis.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    i = 1 as ::core::ffi::c_int;
    while same_basis as ::core::ffi::c_int != 0 && i <= (*lp).rows {
        j = 1 as ::core::ffi::c_int;
        while same_basis as ::core::ffi::c_int != 0 && j <= (*lp).rows {
            same_basis = (*(*(*lp).bb_basis).var_basic.offset(i as isize)
                != *(*lp).var_basic.offset(j as isize))
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
            j += 1;
        }
        same_basis = (same_basis == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while same_basis as ::core::ffi::c_int != 0 && i <= (*lp).sum {
        same_basis = (*(*(*lp).bb_basis).is_lower.offset(i as isize) as ::core::ffi::c_int != 0
            && *(*lp).is_lower.offset(i as isize) as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    return same_basis;
}
#[export_name="honest_lpsolve_restore_basis"]
pub unsafe extern "C" fn restore_basis(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    ok = ((*lp).bb_basis != NULL as *mut basisrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if ok != 0 {
        memcpy(
            (*lp).var_basic as *mut ::core::ffi::c_void,
            (*(*lp).bb_basis).var_basic as *const ::core::ffi::c_void,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*lp).is_basic as *mut ::core::ffi::c_void,
            '\0' as i32,
            (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t),
        );
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            *(*lp)
                .is_basic
                .offset(*(*lp).var_basic.offset(i as isize) as isize) =
                TRUE as ::core::ffi::c_uchar;
            i += 1;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).sum {
            *(*lp).is_lower.offset(i as isize) = is_biton((*(*lp).bb_basis).is_lower, i);
            i += 1;
        }
        set_action(&raw mut (*lp).spx_action, ACTION_REBASE | ACTION_REINVERT);
    }
    return ok;
}
#[export_name="honest_lpsolve_pop_basis"]
pub unsafe extern "C" fn pop_basis(
    mut lp: *mut lprec,
    mut restore: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut ok: ::core::ffi::c_uchar = 0;
    let mut oldbasis: *mut basisrec = ::core::ptr::null_mut::<basisrec>();
    ok = ((*lp).bb_basis != NULL as *mut basisrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if ok != 0 {
        oldbasis = (*lp).bb_basis;
        if !oldbasis.is_null() {
            (*lp).bb_basis = (*oldbasis).previous as *mut basisrec;
            if !((*oldbasis).var_basic as *mut ::core::ffi::c_void).is_null() {
                free((*oldbasis).var_basic as *mut ::core::ffi::c_void);
                (*oldbasis).var_basic = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
            if !((*oldbasis).is_lower as *mut ::core::ffi::c_void).is_null() {
                free((*oldbasis).is_lower as *mut ::core::ffi::c_void);
                (*oldbasis).is_lower = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            if !(oldbasis as *mut ::core::ffi::c_void).is_null() {
                free(oldbasis as *mut ::core::ffi::c_void);
                oldbasis = ::core::ptr::null_mut::<basisrec>();
            }
        }
        if restore as ::core::ffi::c_int != 0 && !(*lp).bb_basis.is_null() {
            restore_basis(lp);
        }
    }
    return ok;
}
#[export_name="honest_lpsolve_unload_basis"]
pub unsafe extern "C" fn unload_basis(
    mut lp: *mut lprec,
    mut restorelast: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut levelsunloaded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*lp).bb_basis.is_null() {
        while pop_basis(lp, restorelast) != 0 {
            levelsunloaded += 1;
        }
    }
    return levelsunloaded;
}
#[export_name="honest_lpsolve_scaled_floor"]
pub unsafe extern "C" fn scaled_floor(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
    mut epsscale: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    value = floor(value);
    if value != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if (*lp).columns_scaled as ::core::ffi::c_int != 0
            && is_integerscaling(lp) as ::core::ffi::c_int != 0
        {
            value = scaled_value(lp, value, colnr);
            if epsscale != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                value += epsscale * (*lp).epsmachine;
            }
        }
    }
    return value;
}
#[export_name="honest_lpsolve_scaled_ceil"]
pub unsafe extern "C" fn scaled_ceil(
    mut lp: *mut lprec,
    mut colnr: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
    mut epsscale: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    value = ceil(value);
    if value != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if (*lp).columns_scaled as ::core::ffi::c_int != 0
            && is_integerscaling(lp) as ::core::ffi::c_int != 0
        {
            value = scaled_value(lp, value, colnr);
            if epsscale != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                value -= epsscale * (*lp).epsmachine;
            }
        }
    }
    return value;
}
#[export_name="honest_lpsolve_is_sc_violated"]
pub unsafe extern "C" fn is_sc_violated(
    mut lp: *mut lprec,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut varno: ::core::ffi::c_int = 0;
    let mut tmpreal: ::core::ffi::c_double = 0.;
    varno = (*lp).rows + column;
    tmpreal = unscaled_value(lp, *(*lp).sc_lobound.offset(column as isize), varno);
    return (tmpreal > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && *(*lp).solution.offset(varno as isize) < tmpreal
        && *(*lp).solution.offset(varno as isize)
            > 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_find_sc_bbvar"]
pub unsafe extern "C" fn find_sc_bbvar(
    mut lp: *mut lprec,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut bestvar: ::core::ffi::c_int = 0;
    let mut firstsc: ::core::ffi::c_int = 0;
    let mut lastsc: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_double = 0.;
    let mut holdINT: ::core::ffi::c_double = 0.;
    let mut bestval: ::core::ffi::c_double = 0.;
    let mut OFval: ::core::ffi::c_double = 0.;
    let mut randval: ::core::ffi::c_double = 0.;
    let mut scval: ::core::ffi::c_double = 0.;
    let mut reversemode: ::core::ffi::c_uchar = 0;
    let mut greedymode: ::core::ffi::c_uchar = 0;
    let mut randomizemode: ::core::ffi::c_uchar = 0;
    let mut pseudocostmode: ::core::ffi::c_uchar = 0;
    let mut pseudocostsel: ::core::ffi::c_uchar = 0;
    bestvar = 0 as ::core::ffi::c_int;
    if (*lp).sc_vars == 0 as ::core::ffi::c_int || *count > 0 as ::core::ffi::c_int {
        return bestvar;
    }
    reversemode = is_bb_mode(lp, NODE_WEIGHTREVERSEMODE);
    greedymode = is_bb_mode(lp, NODE_GREEDYMODE);
    randomizemode = is_bb_mode(lp, NODE_RANDOMIZEMODE);
    pseudocostmode = is_bb_mode(lp, NODE_PSEUDOCOSTMODE);
    pseudocostsel = (is_bb_rule(lp, NODE_PSEUDOCOSTSELECT) as ::core::ffi::c_int != 0
        || is_bb_rule(lp, NODE_PSEUDONONINTSELECT) as ::core::ffi::c_int != 0
        || is_bb_rule(lp, NODE_PSEUDORATIOSELECT) as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    bestvar = 0 as ::core::ffi::c_int;
    bestval = -(*lp).infinite;
    hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    randval = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    firstsc = 0 as ::core::ffi::c_int;
    lastsc = (*lp).columns;
    let mut current_block_52: u64;
    n = 1 as ::core::ffi::c_int;
    while n <= (*lp).columns {
        ii = get_var_priority(lp, n);
        i = (*lp).rows + ii;
        if *(*lp).bb_varactive.offset(ii as isize) == 0
            && is_sc_violated(lp, ii) as ::core::ffi::c_int != 0
            && SOS_is_marked((*lp).SOS, 0 as ::core::ffi::c_int, ii) == 0
        {
            *count += 1;
            lastsc = i;
            if firstsc <= 0 as ::core::ffi::c_int {
                firstsc = i;
            }
            scval = get_pseudorange((*lp).bb_PseudoCost, ii, BB_SC);
            if pseudocostmode != 0 {
                OFval = get_pseudonodecost(
                    (*lp).bb_PseudoCost,
                    ii,
                    BB_SC,
                    *(*lp).solution.offset(i as isize),
                );
            } else {
                OFval = if is_maxim(lp) as ::core::ffi::c_int != 0
                    && get_mat(lp, 0 as ::core::ffi::c_int, ii)
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -get_mat(lp, 0 as ::core::ffi::c_int, ii)
                } else {
                    get_mat(lp, 0 as ::core::ffi::c_int, ii)
                };
            }
            if randomizemode != 0 {
                randval = exp(rand_uniform(lp, 1.0f64));
            }
            if pseudocostsel != 0 {
                if pseudocostmode != 0 {
                    hold = OFval;
                } else {
                    hold = get_pseudonodecost(
                        (*lp).bb_PseudoCost,
                        ii,
                        BB_SC,
                        *(*lp).solution.offset(i as isize),
                    );
                }
                hold *= randval;
                if greedymode != 0 {
                    if pseudocostmode != 0 {
                        OFval = if is_maxim(lp) as ::core::ffi::c_int != 0
                            && get_mat(lp, 0 as ::core::ffi::c_int, ii)
                                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            -get_mat(lp, 0 as ::core::ffi::c_int, ii)
                        } else {
                            get_mat(lp, 0 as ::core::ffi::c_int, ii)
                        };
                    }
                    hold *= OFval;
                }
                hold = if reversemode as ::core::ffi::c_int != 0
                    && hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -hold
                } else {
                    hold
                };
                current_block_52 = 6450597802325118133;
            } else if is_bb_rule(lp, NODE_FRACTIONSELECT) != 0 {
                hold = modf(*(*lp).solution.offset(i as isize) / scval, &raw mut holdINT);
                holdINT = hold - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                if fabs(holdINT) > hold {
                    hold = holdINT;
                }
                if greedymode != 0 {
                    hold *= OFval;
                }
                hold = (if reversemode as ::core::ffi::c_int != 0
                    && hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -hold
                } else {
                    hold
                }) * scval
                    * randval;
                current_block_52 = 6450597802325118133;
            } else if reversemode != 0 {
                current_block_52 = 11650488183268122163;
            } else {
                bestvar = i;
                break;
            }
            match current_block_52 {
                11650488183268122163 => {}
                _ => {
                    if hold > bestval {
                        if bestvar == 0 as ::core::ffi::c_int
                            || hold > bestval + (*lp).epsprimal
                            || fabs(
                                modf(*(*lp).solution.offset(i as isize) / scval, &raw mut holdINT)
                                    - 0.5f64,
                            ) < fabs(
                                modf(
                                    *(*lp).solution.offset(bestvar as isize)
                                        / get_pseudorange(
                                            (*lp).bb_PseudoCost,
                                            bestvar - (*lp).rows,
                                            BB_SC,
                                        ),
                                    &raw mut holdINT,
                                ) - 0.5f64,
                            )
                        {
                            bestval = hold;
                            bestvar = i;
                        }
                    }
                }
            }
        }
        n += 1;
    }
    if is_bb_rule(lp, NODE_FIRSTSELECT) as ::core::ffi::c_int != 0
        && reversemode as ::core::ffi::c_int != 0
    {
        bestvar = lastsc;
    }
    return bestvar;
}
#[export_name="honest_lpsolve_find_sos_bbvar"]
pub unsafe extern "C" fn find_sos_bbvar(
    mut lp: *mut lprec,
    mut count: *mut ::core::ffi::c_int,
    mut intsos: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut var: ::core::ffi::c_int = 0;
    var = 0 as ::core::ffi::c_int;
    if (*lp).SOS.is_null() || *count > 0 as ::core::ffi::c_int {
        return var;
    }
    i = SOS_is_satisfied((*lp).SOS, 0 as ::core::ffi::c_int, (*lp).solution);
    if i == SOS_COMPLETE || i == SOS_INCOMPLETE {
        return -(1 as ::core::ffi::c_int);
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*lp).sos_vars {
        i = *(*lp).sos_priority.offset(k as isize);
        j = (*lp).rows + i;
        if SOS_is_marked((*lp).SOS, 0 as ::core::ffi::c_int, i) == 0
            && SOS_is_full(
                (*lp).SOS,
                0 as ::core::ffi::c_int,
                i,
                FALSE as ::core::ffi::c_uchar,
            ) == 0
        {
            if intsos == 0 || is_int(lp, i) as ::core::ffi::c_int != 0 {
                *count += 1;
                if var == 0 as ::core::ffi::c_int {
                    var = j;
                    break;
                }
            }
        }
        k += 1;
    }
    return var;
}
#[export_name="honest_lpsolve_find_int_bbvar"]
pub unsafe extern "C" fn find_int_bbvar(
    mut lp: *mut lprec,
    mut count: *mut ::core::ffi::c_int,
    mut BB: *mut BBrec,
    mut isfeasible: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut bestvar: ::core::ffi::c_int = 0;
    let mut depthmax: ::core::ffi::c_int = 0;
    let mut nonint: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut hold: ::core::ffi::c_double = 0.;
    let mut holdINT: ::core::ffi::c_double = 0.;
    let mut bestval: ::core::ffi::c_double = 0.;
    let mut OFval: ::core::ffi::c_double = 0.;
    let mut randval: ::core::ffi::c_double = 0.;
    let mut lowbo: *mut ::core::ffi::c_double = (*BB).lowbo;
    let mut upbo: *mut ::core::ffi::c_double = (*BB).upbo;
    let mut reversemode: ::core::ffi::c_uchar = 0;
    let mut greedymode: ::core::ffi::c_uchar = 0;
    let mut depthfirstmode: ::core::ffi::c_uchar = 0;
    let mut breadthfirstmode: ::core::ffi::c_uchar = 0;
    let mut randomizemode: ::core::ffi::c_uchar = 0;
    let mut rcostmode: ::core::ffi::c_uchar = 0;
    let mut pseudocostmode: ::core::ffi::c_uchar = 0;
    let mut pseudocostsel: ::core::ffi::c_uchar = 0;
    let mut pseudostrong: ::core::ffi::c_uchar = 0;
    let mut isINT_0: ::core::ffi::c_uchar = 0;
    let mut valINT: ::core::ffi::c_uchar = 0;
    if (*lp).int_vars == 0 as ::core::ffi::c_int || *count > 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*lp).bb_usenode.is_some() {
        i = (*lp).bb_usenode.expect("non-null function pointer")(lp, (*lp).bb_nodehandle, BB_INT);
        if i >= 0 as ::core::ffi::c_int {
            if i > 0 as ::core::ffi::c_int {
                *count += 1;
            }
            return i;
        }
    }
    reversemode = is_bb_mode(lp, NODE_WEIGHTREVERSEMODE);
    greedymode = is_bb_mode(lp, NODE_GREEDYMODE);
    randomizemode = is_bb_mode(lp, NODE_RANDOMIZEMODE);
    depthfirstmode = is_bb_mode(lp, NODE_DEPTHFIRSTMODE);
    breadthfirstmode = (is_bb_mode(lp, NODE_BREADTHFIRSTMODE) as ::core::ffi::c_int != 0
        && ((*lp).bb_level <= (*lp).int_vars) as ::core::ffi::c_int as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    rcostmode = (((*(*BB).lp).solutioncount > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_uchar as ::core::ffi::c_int
        != 0
        && is_bb_mode(lp, NODE_RCOSTFIXING) as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    pseudocostmode = is_bb_mode(lp, NODE_PSEUDOCOSTMODE);
    pseudocostsel = (is_bb_rule(lp, NODE_PSEUDOCOSTSELECT) as ::core::ffi::c_int != 0
        || is_bb_rule(lp, NODE_PSEUDONONINTSELECT) as ::core::ffi::c_int != 0
        || is_bb_rule(lp, NODE_PSEUDORATIOSELECT) as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    pseudostrong = (FALSE != 0
        && pseudocostsel as ::core::ffi::c_int != 0
        && rcostmode == 0
        && is_bb_mode(lp, NODE_STRONGINIT) as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    allocINT(
        lp,
        &raw mut nonint,
        (*lp).columns + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    n = 0 as ::core::ffi::c_int;
    depthmax = -(1 as ::core::ffi::c_int);
    if !isfeasible.is_null() {
        *isfeasible = TRUE as ::core::ffi::c_uchar;
    }
    (*BB).lastrcf = 0 as ::core::ffi::c_int;
    k = 1 as ::core::ffi::c_int;
    while k <= (*lp).columns {
        ii = get_var_priority(lp, k);
        isINT_0 = is_int(lp, ii);
        i = (*lp).rows + ii;
        if isINT_0 == 0 {
            if rcostmode != 0 {
                bestvar = rcfbound_BB(
                    BB,
                    i,
                    isINT_0,
                    ::core::ptr::null_mut::<::core::ffi::c_double>(),
                    isfeasible,
                );
                if bestvar != FR {
                    (*BB).lastrcf += 1;
                }
            }
        } else {
            valINT = solution_is_int(lp, i, FALSE as ::core::ffi::c_uchar);
            if !(*lowbo.offset(i as isize) == *upbo.offset(i as isize)) {
                if rcostmode != 0 {
                    bestvar = rcfbound_BB(
                        BB,
                        i,
                        isINT_0,
                        ::core::ptr::null_mut::<::core::ffi::c_double>(),
                        isfeasible,
                    );
                    if bestvar != FR {
                        (*BB).lastrcf += 1;
                    }
                } else {
                    bestvar = FR;
                }
                if valINT == 0 && bestvar >= FR {
                    n += 1;
                    *nonint.offset(n as isize) = ii;
                    if depthmax < *(*lp).bb_varactive.offset(ii as isize) {
                        depthmax = *(*lp).bb_varactive.offset(ii as isize);
                    }
                }
            }
        }
        k += 1;
    }
    *nonint.offset(0 as ::core::ffi::c_int as isize) = n;
    *count = n;
    bestvar = 0 as ::core::ffi::c_int;
    if !(n == 0 as ::core::ffi::c_int) {
        bestval = -(*lp).infinite;
        hold = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        randval = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        if (*lp).bb_level > 1 as ::core::ffi::c_int
            && depthmax > 0 as ::core::ffi::c_int
            && (depthfirstmode as ::core::ffi::c_int != 0
                || breadthfirstmode as ::core::ffi::c_int != 0)
        {
            let mut depths: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
            allocINT(
                lp,
                &raw mut depths,
                n + 1 as ::core::ffi::c_int,
                FALSE as ::core::ffi::c_uchar,
            );
            i = 1 as ::core::ffi::c_int;
            while i <= n {
                *depths.offset(i as isize) = (if depthfirstmode as ::core::ffi::c_int != 0 {
                    n + 1 as ::core::ffi::c_int - i
                } else {
                    i
                }) + (n + 1 as ::core::ffi::c_int)
                    * *(*lp)
                        .bb_varactive
                        .offset(*nonint.offset(i as isize) as isize);
                i += 1;
            }
            hpsortex(
                depths as *mut ::core::ffi::c_void,
                n,
                1 as ::core::ffi::c_int,
                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
                depthfirstmode,
                Some(
                    compareINT
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                nonint,
            );
            if !(depths as *mut ::core::ffi::c_void).is_null() {
                free(depths as *mut ::core::ffi::c_void);
                depths = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
        }
        if is_bb_rule(lp, NODE_FIRSTSELECT) != 0 {
            if reversemode != 0 {
                bestvar = (*lp).rows
                    + *nonint.offset(*nonint.offset(0 as ::core::ffi::c_int as isize) as isize);
            } else {
                bestvar = (*lp).rows + *nonint.offset(1 as ::core::ffi::c_int as isize);
            }
        } else {
            n = 1 as ::core::ffi::c_int;
            while n <= *nonint.offset(0 as ::core::ffi::c_int as isize) {
                ii = *nonint.offset(n as isize);
                i = (*lp).rows + ii;
                if n == 1 as ::core::ffi::c_int {
                    bestvar = i;
                }
                if pseudostrong as ::core::ffi::c_int != 0
                    && (if (*(*(*lp).bb_PseudoCost).LOcost.offset(ii as isize)).rownr
                        > (*(*(*lp).bb_PseudoCost).UPcost.offset(ii as isize)).rownr
                    {
                        (*(*(*lp).bb_PseudoCost).LOcost.offset(ii as isize)).rownr
                    } else {
                        (*(*(*lp).bb_PseudoCost).UPcost.offset(ii as isize)).rownr
                    }) < (*(*lp).bb_PseudoCost).updatelimit
                    && (if (*(*(*lp).bb_PseudoCost).LOcost.offset(ii as isize)).colnr
                        > (*(*(*lp).bb_PseudoCost).UPcost.offset(ii as isize)).colnr
                    {
                        (*(*(*lp).bb_PseudoCost).LOcost.offset(ii as isize)).colnr
                    } else {
                        (*(*(*lp).bb_PseudoCost).UPcost.offset(ii as isize)).colnr
                    }) < 5 as ::core::ffi::c_int * (*(*lp).bb_PseudoCost).updatelimit
                {
                    strongbranch_BB(
                        lp,
                        BB,
                        ii,
                        BB_INT,
                        *nonint.offset(0 as ::core::ffi::c_int as isize),
                    );
                }
                if pseudocostmode != 0 {
                    OFval = get_pseudonodecost(
                        (*lp).bb_PseudoCost,
                        ii,
                        BB_INT,
                        *(*lp).solution.offset(i as isize),
                    );
                } else {
                    OFval = if is_maxim(lp) as ::core::ffi::c_int != 0
                        && get_mat(lp, 0 as ::core::ffi::c_int, ii)
                            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -get_mat(lp, 0 as ::core::ffi::c_int, ii)
                    } else {
                        get_mat(lp, 0 as ::core::ffi::c_int, ii)
                    };
                }
                if randomizemode != 0 {
                    randval = exp(rand_uniform(lp, 1.0f64));
                }
                if pseudocostsel != 0 {
                    if pseudocostmode != 0 {
                        hold = OFval;
                    } else {
                        hold = get_pseudonodecost(
                            (*lp).bb_PseudoCost,
                            ii,
                            BB_INT,
                            *(*lp).solution.offset(i as isize),
                        );
                    }
                    hold *= randval;
                    if greedymode != 0 {
                        if pseudocostmode != 0 {
                            OFval = if is_maxim(lp) as ::core::ffi::c_int != 0
                                && get_mat(lp, 0 as ::core::ffi::c_int, ii)
                                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                -get_mat(lp, 0 as ::core::ffi::c_int, ii)
                            } else {
                                get_mat(lp, 0 as ::core::ffi::c_int, ii)
                            };
                        }
                        hold *= OFval;
                    }
                    hold = if reversemode as ::core::ffi::c_int != 0
                        && hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -hold
                    } else {
                        hold
                    };
                } else if is_bb_rule(lp, NODE_GAPSELECT) != 0 {
                    hold = *(*lp).solution.offset(i as isize);
                    holdINT = hold - unscaled_value(lp, *upbo.offset(i as isize), i);
                    hold -= unscaled_value(lp, *lowbo.offset(i as isize), i);
                    if fabs(holdINT) > hold {
                        hold = holdINT;
                    }
                    if greedymode != 0 {
                        hold *= OFval;
                    }
                    hold = (if reversemode as ::core::ffi::c_int != 0
                        && hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -hold
                    } else {
                        hold
                    }) * randval;
                } else if is_bb_rule(lp, NODE_FRACTIONSELECT) != 0 {
                    hold = modf(*(*lp).solution.offset(i as isize), &raw mut holdINT);
                    holdINT = hold - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    if fabs(holdINT) > hold {
                        hold = holdINT;
                    }
                    if greedymode != 0 {
                        hold *= OFval;
                    }
                    hold = (if reversemode as ::core::ffi::c_int != 0
                        && hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -hold
                    } else {
                        hold
                    }) * randval;
                } else if is_bb_rule(lp, NODE_RANGESELECT) != 0 {
                    hold =
                        unscaled_value(lp, *upbo.offset(i as isize) - *lowbo.offset(i as isize), i);
                    if greedymode != 0 {
                        hold *= OFval;
                    }
                    hold = (if reversemode as ::core::ffi::c_int != 0
                        && hold != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -hold
                    } else {
                        hold
                    }) * randval;
                }
                if hold > bestval {
                    if hold > bestval + (*lp).epsprimal
                        || fabs(modf(*(*lp).solution.offset(i as isize), &raw mut holdINT) - 0.5f64)
                            < fabs(
                                modf(*(*lp).solution.offset(bestvar as isize), &raw mut holdINT)
                                    - 0.5f64,
                            )
                    {
                        bestval = hold;
                        bestvar = i;
                    }
                }
                n += 1;
            }
        }
    }
    if !(nonint as *mut ::core::ffi::c_void).is_null() {
        free(nonint as *mut ::core::ffi::c_void);
        nonint = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return bestvar;
}
#[export_name="honest_lpsolve_init_pseudocost"]
pub unsafe extern "C" fn init_pseudocost(
    mut lp: *mut lprec,
    mut pseudotype: ::core::ffi::c_int,
) -> *mut BBPSrec {
    let mut i: ::core::ffi::c_int = 0;
    let mut PSinitUP: ::core::ffi::c_double = 0.;
    let mut PSinitLO: ::core::ffi::c_double = 0.;
    let mut newitem: *mut BBPSrec = ::core::ptr::null_mut::<BBPSrec>();
    let mut isPSCount: ::core::ffi::c_uchar = 0;
    newitem = malloc(::core::mem::size_of::<BBPSrec>() as size_t) as *mut BBPSrec;
    (*newitem).lp = lp;
    (*newitem).LOcost = malloc(
        (((*lp).columns + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<MATitem>() as size_t),
    ) as *mut MATitem;
    (*newitem).UPcost = malloc(
        (((*lp).columns + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<MATitem>() as size_t),
    ) as *mut MATitem;
    (*newitem).secondary = ::core::ptr::null_mut::<_BBPSrec>();
    (*newitem).pseodotype = pseudotype & NODE_STRATEGYMASK;
    isPSCount = (pseudotype & NODE_PSEUDONONINTSELECT != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        (*(*newitem).LOcost.offset(i as isize)).rownr = 1 as ::core::ffi::c_int;
        (*(*newitem).LOcost.offset(i as isize)).colnr = 1 as ::core::ffi::c_int;
        (*(*newitem).UPcost.offset(i as isize)).rownr = 1 as ::core::ffi::c_int;
        (*(*newitem).UPcost.offset(i as isize)).colnr = 1 as ::core::ffi::c_int;
        PSinitUP = if is_maxim(lp) as ::core::ffi::c_int != 0
            && get_mat(lp, 0 as ::core::ffi::c_int, i)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -get_mat(lp, 0 as ::core::ffi::c_int, i)
        } else {
            get_mat(lp, 0 as ::core::ffi::c_int, i)
        };
        PSinitLO = -PSinitUP;
        if isPSCount != 0 {
            PSinitUP = 0.1f64 * 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            PSinitLO = PSinitUP;
        }
        (*(*newitem).UPcost.offset(i as isize)).value = PSinitUP;
        (*(*newitem).LOcost.offset(i as isize)).value = PSinitLO;
        i += 1;
    }
    (*newitem).updatelimit = (*lp).bb_PseudoUpdates;
    (*newitem).updatesfinished = 0 as ::core::ffi::c_int;
    (*newitem).restartlimit = DEF_PSEUDOCOSTRESTART;
    if userabort(lp, MSG_INITPSEUDOCOST) != 0 {
        (*lp).spx_status = USERABORT;
    }
    return newitem;
}
#[export_name="honest_lpsolve_free_pseudoclass"]
pub unsafe extern "C" fn free_pseudoclass(
    mut PseudoClass: *mut *mut BBPSrec,
) -> ::core::ffi::c_uchar {
    let mut target: *mut BBPSrec = *PseudoClass;
    if !((*target).LOcost as *mut ::core::ffi::c_void).is_null() {
        free((*target).LOcost as *mut ::core::ffi::c_void);
        (*target).LOcost = ::core::ptr::null_mut::<MATitem>();
    }
    if !((*target).UPcost as *mut ::core::ffi::c_void).is_null() {
        free((*target).UPcost as *mut ::core::ffi::c_void);
        (*target).UPcost = ::core::ptr::null_mut::<MATitem>();
    }
    target = (*target).secondary as *mut BBPSrec;
    if !(*PseudoClass as *mut ::core::ffi::c_void).is_null() {
        free(*PseudoClass as *mut ::core::ffi::c_void);
        *PseudoClass = ::core::ptr::null_mut::<BBPSrec>();
    }
    *PseudoClass = target;
    return (target != NULL as *mut BBPSrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_free_pseudocost"]
pub unsafe extern "C" fn free_pseudocost(mut lp: *mut lprec) {
    if !lp.is_null() && !(*lp).bb_PseudoCost.is_null() {
        while free_pseudoclass(&raw mut (*lp).bb_PseudoCost) != 0 {}
    }
}
#[export_name="honest_lpsolve_set_pseudocosts"]
pub unsafe extern "C" fn set_pseudocosts(
    mut lp: *mut lprec,
    mut clower: *mut ::core::ffi::c_double,
    mut cupper: *mut ::core::ffi::c_double,
    mut updatelimit: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).bb_PseudoCost.is_null() || clower.is_null() && cupper.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if !clower.is_null() {
            (*(*(*lp).bb_PseudoCost).LOcost.offset(i as isize)).value = *clower.offset(i as isize);
        }
        if !cupper.is_null() {
            (*(*(*lp).bb_PseudoCost).UPcost.offset(i as isize)).value = *cupper.offset(i as isize);
        }
        i += 1;
    }
    if !updatelimit.is_null() {
        (*(*lp).bb_PseudoCost).updatelimit = *updatelimit;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_pseudocosts"]
pub unsafe extern "C" fn get_pseudocosts(
    mut lp: *mut lprec,
    mut clower: *mut ::core::ffi::c_double,
    mut cupper: *mut ::core::ffi::c_double,
    mut updatelimit: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    if (*lp).bb_PseudoCost.is_null() || clower.is_null() && cupper.is_null() {
        return 0 as ::core::ffi::c_uchar;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).columns {
        if !clower.is_null() {
            *clower.offset(i as isize) = (*(*(*lp).bb_PseudoCost).LOcost.offset(i as isize)).value;
        }
        if !cupper.is_null() {
            *cupper.offset(i as isize) = (*(*(*lp).bb_PseudoCost).UPcost.offset(i as isize)).value;
        }
        i += 1;
    }
    if !updatelimit.is_null() {
        *updatelimit = (*(*lp).bb_PseudoCost).updatelimit;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_pseudorange"]
pub unsafe extern "C" fn get_pseudorange(
    mut pc: *mut BBPSrec,
    mut mipvar: ::core::ffi::c_int,
    mut varcode: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if varcode == BB_SC {
        return unscaled_value(
            (*pc).lp,
            *(*(*pc).lp).sc_lobound.offset(mipvar as isize),
            (*(*pc).lp).rows + mipvar,
        );
    } else {
        return 1.0f64;
    };
}
#[export_name="honest_lpsolve_update_pseudocost"]
pub unsafe extern "C" fn update_pseudocost(
    mut pc: *mut BBPSrec,
    mut mipvar: ::core::ffi::c_int,
    mut varcode: ::core::ffi::c_int,
    mut capupper: ::core::ffi::c_uchar,
    mut varsol: ::core::ffi::c_double,
) {
    let mut OFsol: ::core::ffi::c_double = 0.;
    let mut uplim: ::core::ffi::c_double = 0.;
    let mut PS: *mut MATitem = ::core::ptr::null_mut::<MATitem>();
    let mut nonIntSelect: ::core::ffi::c_uchar = is_bb_rule((*pc).lp, NODE_PSEUDONONINTSELECT);
    uplim = get_pseudorange(pc, mipvar, varcode);
    varsol = modf(varsol / uplim, &raw mut OFsol);
    if nonIntSelect != 0 {
        OFsol = (*(*(*pc).lp).bb_bounds).lastvarcus as ::core::ffi::c_double;
    } else {
        OFsol = *(*(*pc).lp)
            .solution
            .offset(0 as ::core::ffi::c_int as isize);
    }
    if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_float>() as usize
    {
        __inline_isnanf(varsol as ::core::ffi::c_float)
    } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
    {
        __inline_isnand(varsol)
    } else {
        __inline_isnanl(crate::honest_did::lpsolve::extended::Extended::new(varsol))
    } != 0
    {
        (*(*pc).lp).bb_parentOF = OFsol;
        return;
    }
    if capupper != 0 {
        PS = (*pc).LOcost.offset(mipvar as isize) as *mut MATitem;
    } else {
        PS = (*pc).UPcost.offset(mipvar as isize) as *mut MATitem;
        varsol = 1 as ::core::ffi::c_int as ::core::ffi::c_double - varsol;
    }
    (*PS).colnr += 1;
    if is_bb_rule((*pc).lp, NODE_PSEUDORATIOSELECT) != 0 {
        varsol *= capupper as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    mipvar = (*pc).updatelimit;
    if (mipvar <= 0 as ::core::ffi::c_int || (*PS).rownr < mipvar)
        && fabs(varsol) > (*(*pc).lp).epspivot
    {
        (*PS).value = (*PS).value * (*PS).rownr as ::core::ffi::c_double
            + ((*(*pc).lp).bb_parentOF - OFsol) / (varsol * uplim);
        (*PS).rownr += 1;
        (*PS).value /= (*PS).rownr as ::core::ffi::c_double;
        if (*PS).rownr == mipvar {
            (*pc).updatesfinished += 1;
            if is_bb_mode((*pc).lp, NODE_RESTARTMODE) as ::core::ffi::c_int != 0
                && (*pc).updatesfinished as ::core::ffi::c_double
                    / (2.0f64 * (*(*pc).lp).int_vars as ::core::ffi::c_double)
                    > (*pc).restartlimit
            {
                (*(*pc).lp).bb_break = AUTOMATIC as ::core::ffi::c_uchar;
                (*pc).restartlimit *= 2.681f64;
                if (*pc).restartlimit > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                    (*(*pc).lp).bb_rule -= NODE_RESTARTMODE;
                }
                report(
                    (*pc).lp,
                    4 as ::core::ffi::c_int,
                    b"update_pseudocost: Restarting with updated pseudocosts\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    (*(*pc).lp).bb_parentOF = OFsol;
}
#[export_name="honest_lpsolve_get_pseudobranchcost"]
pub unsafe extern "C" fn get_pseudobranchcost(
    mut pc: *mut BBPSrec,
    mut mipvar: ::core::ffi::c_int,
    mut dofloor: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    if dofloor != 0 {
        return (*(*pc).LOcost.offset(mipvar as isize)).value;
    } else {
        return (*(*pc).UPcost.offset(mipvar as isize)).value;
    };
}
#[export_name="honest_lpsolve_get_pseudonodecost"]
pub unsafe extern "C" fn get_pseudonodecost(
    mut pc: *mut BBPSrec,
    mut mipvar: ::core::ffi::c_int,
    mut vartype: ::core::ffi::c_int,
    mut varsol: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut hold: ::core::ffi::c_double = 0.;
    let mut uplim: ::core::ffi::c_double = 0.;
    uplim = get_pseudorange(pc, mipvar, vartype);
    varsol = modf(varsol / uplim, &raw mut hold);
    if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_float>() as usize
    {
        __inline_isnanf(varsol as ::core::ffi::c_float)
    } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
    {
        __inline_isnand(varsol)
    } else {
        __inline_isnanl(crate::honest_did::lpsolve::extended::Extended::new(varsol))
    } != 0
    {
        varsol = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    hold = (*(*pc).LOcost.offset(mipvar as isize)).value * varsol
        + (*(*pc).UPcost.offset(mipvar as isize)).value
            * (1 as ::core::ffi::c_int as ::core::ffi::c_double - varsol);
    return hold * uplim;
}
#[export_name="honest_lpsolve_compute_theta"]
pub unsafe extern "C" fn compute_theta(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut theta: *mut ::core::ffi::c_double,
    mut isupbound: ::core::ffi::c_int,
    mut HarrisScalar: ::core::ffi::c_double,
    mut primal: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut colnr: ::core::ffi::c_int = *(*lp).var_basic.offset(rownr as isize);
    let mut x: ::core::ffi::c_double = *(*lp).rhs.offset(rownr as isize);
    let mut lb: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut ub: ::core::ffi::c_double = *(*lp).upbo.offset(colnr as isize);
    let mut eps: ::core::ffi::c_double = (*lp).epsprimal;
    HarrisScalar *= eps;
    if primal != 0 {
        if *theta > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            x -= lb - HarrisScalar;
        } else if ub < (*lp).infinite {
            x -= ub + HarrisScalar;
        } else {
            *theta = -(*lp).infinite;
            return colnr;
        }
    } else {
        if isupbound != 0 {
            *theta = -*theta;
        }
        if x < lb + eps {
            x -= lb - HarrisScalar;
        } else if x > ub - eps {
            if ub >= (*lp).infinite {
                *theta = (*lp).infinite
                    * (if *theta < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) as ::core::ffi::c_double;
                return colnr;
            } else {
                x -= ub + HarrisScalar;
            }
        }
    }
    if fabs(x) < (*lp).epsmachine {
        x = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    *theta = x / *theta;
    return colnr;
}
#[export_name="honest_lpsolve_check_degeneracy"]
pub unsafe extern "C" fn check_degeneracy(
    mut lp: *mut lprec,
    mut pcol: *mut ::core::ffi::c_double,
    mut degencount: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ndegen: ::core::ffi::c_int = 0;
    let mut rhs: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut sdegen: ::core::ffi::c_double = 0.;
    let mut epsmargin: ::core::ffi::c_double = (*lp).epsprimal;
    sdegen = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    ndegen = 0 as ::core::ffi::c_int;
    rhs = (*lp).rhs;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        rhs = rhs.offset(1);
        pcol = pcol.offset(1);
        if fabs(*rhs) < epsmargin {
            sdegen += *pcol;
            ndegen += 1;
        } else if fabs(
            *rhs - *(*lp)
                .upbo
                .offset(*(*lp).var_basic.offset(i as isize) as isize),
        ) < epsmargin
        {
            sdegen -= *pcol;
            ndegen += 1;
        }
        i += 1;
    }
    if !degencount.is_null() {
        *degencount = ndegen;
    }
    return (sdegen <= 0 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_performiteration"]
pub unsafe extern "C" fn performiteration(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut varin: ::core::ffi::c_int,
    mut theta: ::core::ffi::c_double,
    mut primal: ::core::ffi::c_uchar,
    mut allowminit: ::core::ffi::c_uchar,
    mut prow: *mut ::core::ffi::c_double,
    mut nzprow: *mut ::core::ffi::c_int,
    mut pcol: *mut ::core::ffi::c_double,
    mut nzpcol: *mut ::core::ffi::c_int,
    mut boundswaps: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut varout: ::core::ffi::c_int = 0;
    let mut pivot: ::core::ffi::c_double = 0.;
    let mut epsmargin: ::core::ffi::c_double = 0.;
    let mut leavingValue: ::core::ffi::c_double = 0.;
    let mut leavingUB: ::core::ffi::c_double = 0.;
    let mut enteringUB: ::core::ffi::c_double = 0.;
    let mut leavingToUB: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut enteringFromUB: ::core::ffi::c_uchar = 0;
    let mut enteringIsFixed: ::core::ffi::c_uchar = 0;
    let mut leavingIsFixed: ::core::ffi::c_uchar = 0;
    let mut islower: *mut ::core::ffi::c_uchar =
        (*lp).is_lower.offset(varin as isize) as *mut ::core::ffi::c_uchar;
    let mut minitNow: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
    let mut minitStatus: ::core::ffi::c_uchar = ITERATE_MAJORMAJOR as ::core::ffi::c_uchar;
    let mut deltatheta: ::core::ffi::c_double = theta;
    if userabort(lp, MSG_ITERATION) != 0 {
        return minitNow;
    }
    varout = *(*lp).var_basic.offset(rownr as isize);
    (*lp).current_iter += 1;
    epsmargin = (*lp).epsprimal;
    enteringFromUB = (*islower == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    enteringUB = *(*lp).upbo.offset(varin as isize);
    leavingUB = *(*lp).upbo.offset(varout as isize);
    enteringIsFixed = (fabs(enteringUB) < epsmargin) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    leavingIsFixed = (fabs(leavingUB) < epsmargin) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if !boundswaps.is_null()
        && *boundswaps.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut boundvar: ::core::ffi::c_int = 0;
        let mut hold: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
        allocREAL(
            lp,
            &raw mut hold,
            (*lp).rows + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= *boundswaps.offset(0 as ::core::ffi::c_int as isize) {
            boundvar = *boundswaps.offset(i as isize);
            deltatheta = if *(*lp).is_lower.offset(boundvar as isize) == 0
                && *(*lp).upbo.offset(boundvar as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -*(*lp).upbo.offset(boundvar as isize)
            } else {
                *(*lp).upbo.offset(boundvar as isize)
            };
            mat_multadd((*lp).matA, hold, boundvar, deltatheta);
            *(*lp).is_lower.offset(boundvar as isize) = (*(*lp).is_lower.offset(boundvar as isize)
                == 0) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
            i += 1;
        }
        (*lp).current_bswap +=
            *boundswaps.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_longlong;
        (*lp).current_iter +=
            *boundswaps.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_longlong;
        ftran(
            lp,
            hold,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            (*lp).epsmachine,
        );
        if (*lp).obj_in_basis == 0 {
            *hold.offset(0 as ::core::ffi::c_int as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        pivot = (*lp).bfp_pivotRHS.expect("non-null function pointer")(
            lp,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            hold,
        );
        deltatheta = multi_enteringtheta((*lp).longsteps);
        theta = deltatheta;
        if !(hold as *mut ::core::ffi::c_void).is_null() {
            free(hold as *mut ::core::ffi::c_void);
            hold = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    } else if allowminit as ::core::ffi::c_int != 0 && enteringIsFixed == 0 {
        pivot = (*lp).epsdual;
        if enteringUB - theta < -pivot {
            if fabs(enteringUB - theta) < pivot {
                minitStatus = ITERATE_MINORMAJOR as ::core::ffi::c_uchar;
            } else {
                minitStatus = ITERATE_MINORRETRY as ::core::ffi::c_uchar;
            }
            minitNow = (minitStatus as ::core::ffi::c_int != ITERATE_MAJORMAJOR)
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
        }
    }
    if minitNow != 0 {
        theta = if fabs(theta) < enteringUB {
            fabs(theta)
        } else {
            enteringUB
        };
        pivot = (*lp).bfp_pivotRHS.expect("non-null function pointer")(
            lp,
            theta,
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
        );
        *islower = (*islower == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        (*lp).current_bswap += 1;
    } else {
        updatePricer(
            lp,
            rownr,
            varin,
            (*lp).bfp_pivotvector.expect("non-null function pointer")(lp),
            prow,
            nzprow,
        );
        pivot = (*lp).bfp_pivotRHS.expect("non-null function pointer")(
            lp,
            theta,
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
        );
        leavingValue = *(*lp).rhs.offset(rownr as isize);
        leavingToUB =
            (leavingValue > 0.5f64 * leavingUB) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        *(*lp).is_lower.offset(varout as isize) =
            (leavingIsFixed as ::core::ffi::c_int != 0 || leavingToUB == 0) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
        if enteringFromUB != 0 {
            *(*lp).rhs.offset(rownr as isize) = enteringUB - deltatheta;
            *islower = TRUE as ::core::ffi::c_uchar;
        } else {
            *(*lp).rhs.offset(rownr as isize) = deltatheta;
        }
        if fabs(*(*lp).rhs.offset(rownr as isize)) < epsmargin {
            *(*lp).rhs.offset(rownr as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        varout = set_basisvar(lp, rownr, varin);
        (*lp).bfp_finishupdate.expect("non-null function pointer")(lp, enteringFromUB);
    }
    if (*lp).verbose > NORMAL
        && MIP_count(lp) == 0 as ::core::ffi::c_int
        && (*lp).current_iter
            % (if 2 as ::core::ffi::c_int > (*lp).rows / 10 as ::core::ffi::c_int {
                2 as ::core::ffi::c_int
            } else {
                (*lp).rows / 10 as ::core::ffi::c_int
            }) as ::core::ffi::c_longlong
            == 0 as ::core::ffi::c_longlong
    {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"Objective value %18.12g at iter %10.0f.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if (*lp).spx_trace != 0 {
        if minitNow != 0 {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"I:%5.0f - minor - %5d ignored,          %5d flips  from %s with THETA=%g and OBJ=%g\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"I:%5.0f - MAJOR - %5d leaves to %s,  %5d enters from %s with THETA=%g and OBJ=%g\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        if minitNow != 0 {
            if *(*lp).is_lower.offset(varin as isize) == 0 {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"performiteration: Variable %d changed to its lower bound at iter %.0f (from %g)\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                report(
                    lp,
                    5 as ::core::ffi::c_int,
                    b"performiteration: Variable %d changed to its upper bound at iter %.0f (to %g)\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        } else {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"performiteration: Variable %d entered basis at iter %.0f at %18.12g\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        if primal == 0 {
            pivot = compute_feasibilitygap(
                lp,
                (primal == 0) as ::core::ffi::c_int as ::core::ffi::c_uchar,
                TRUE as ::core::ffi::c_uchar,
            );
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"performiteration: Feasibility gap at iter %.0f is %18.12g\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"performiteration: Current objective function value at iter %.0f is %18.12g\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
    return minitStatus;
}
#[export_name="honest_lpsolve_get_refactfrequency"]
pub unsafe extern "C" fn get_refactfrequency(
    mut lp: *mut lprec,
    mut final_0: ::core::ffi::c_uchar,
) -> ::core::ffi::c_double {
    let mut iters: ::core::ffi::c_longlong = 0;
    let mut refacts: ::core::ffi::c_int = 0;
    iters = (*lp).total_iter + (*lp).current_iter - ((*lp).total_bswap + (*lp).current_bswap);
    refacts = (*lp).bfp_refactcount.expect("non-null function pointer")(lp, BFP_STAT_REFACT_TOTAL);
    if final_0 != 0 {
        return iters as ::core::ffi::c_double
            / (if 1 as ::core::ffi::c_int > refacts {
                1 as ::core::ffi::c_int
            } else {
                refacts
            }) as ::core::ffi::c_double;
    } else if (*lp).bb_totalnodes > 0 as ::core::ffi::c_longlong {
        return (*lp).bfp_pivotmax.expect("non-null function pointer")(lp) as ::core::ffi::c_double;
    } else {
        return ((*lp).bfp_pivotmax.expect("non-null function pointer")(lp)
            as ::core::ffi::c_longlong
            + iters) as ::core::ffi::c_double
            / (1 as ::core::ffi::c_int + refacts) as ::core::ffi::c_double;
    };
}
#[export_name="honest_lpsolve_is_fixedvar"]
pub unsafe extern "C" fn is_fixedvar(
    mut lp: *mut lprec,
    mut varnr: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if (*lp).bb_bounds.is_null() {
        if varnr <= (*lp).rows {
            return (*(*lp).orig_upbo.offset(varnr as isize) < (*lp).epsmachine)
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
        } else {
            return (*(*lp).orig_upbo.offset(varnr as isize)
                - *(*lp).orig_lowbo.offset(varnr as isize)
                < (*lp).epsmachine) as ::core::ffi::c_int
                as ::core::ffi::c_uchar;
        }
    } else if varnr <= (*lp).rows || (*(*lp).bb_bounds).UBzerobased as ::core::ffi::c_int == TRUE {
        return (*(*lp).upbo.offset(varnr as isize) < (*lp).epsvalue) as ::core::ffi::c_int
            as ::core::ffi::c_uchar;
    } else {
        return (*(*lp).upbo.offset(varnr as isize) - *(*lp).lowbo.offset(varnr as isize)
            < (*lp).epsvalue) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_solution_is_int"]
pub unsafe extern "C" fn solution_is_int(
    mut lp: *mut lprec,
    mut index: ::core::ffi::c_int,
    mut checkfixed: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    return (isINT(lp, *(*lp).solution.offset(index as isize)) as ::core::ffi::c_int != 0
        && (checkfixed == 0 || is_fixedvar(lp, index) as ::core::ffi::c_int != 0))
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_set_multiprice"]
pub unsafe extern "C" fn set_multiprice(
    mut lp: *mut lprec,
    mut multiblockdiv: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if multiblockdiv != (*lp).multiblockdiv {
        if multiblockdiv < 1 as ::core::ffi::c_int {
            multiblockdiv = 1 as ::core::ffi::c_int;
        }
        (*lp).multiblockdiv = multiblockdiv;
        multi_free(&raw mut (*lp).multivars);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_multiprice"]
pub unsafe extern "C" fn get_multiprice(
    mut lp: *mut lprec,
    mut getabssize: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if (*lp).multivars.is_null() || (*(*lp).multivars).used == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if getabssize != 0 {
        return (*(*lp).multivars).size;
    } else {
        return (*lp).multiblockdiv;
    };
}
#[export_name="honest_lpsolve_set_partialprice"]
pub unsafe extern "C" fn set_partialprice(
    mut lp: *mut lprec,
    mut blockcount: ::core::ffi::c_int,
    mut blockstart: *mut ::core::ffi::c_int,
    mut isrow: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut ne: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut items: ::core::ffi::c_int = 0;
    let mut blockdata: *mut *mut partialrec = ::core::ptr::null_mut::<*mut partialrec>();
    if isrow != 0 {
        blockdata = &raw mut (*lp).rowblocks;
    } else {
        blockdata = &raw mut (*lp).colblocks;
    }
    ne = 0 as ::core::ffi::c_int;
    items = if isrow as ::core::ffi::c_int != 0 {
        (*lp).rows
    } else {
        (*lp).columns
    };
    if blockcount == 1 as ::core::ffi::c_int {
        partial_freeBlocks(blockdata);
    } else if blockcount <= 0 as ::core::ffi::c_int {
        blockstart = ::core::ptr::null_mut::<::core::ffi::c_int>();
        if items < DEF_PARTIALBLOCKS * DEF_PARTIALBLOCKS {
            blockcount = items / DEF_PARTIALBLOCKS + 1 as ::core::ffi::c_int;
        } else {
            blockcount = DEF_PARTIALBLOCKS;
        }
        ne = items / blockcount;
        if ne * blockcount < items {
            ne += 1;
        }
    }
    if blockcount > 1 as ::core::ffi::c_int {
        let mut isNew: ::core::ffi::c_uchar =
            (*blockdata == NULL as *mut partialrec) as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i = 0 as ::core::ffi::c_int;
        if isrow == 0 {
            i += 1;
        }
        if isNew != 0 {
            *blockdata = partial_createBlocks(lp, isrow);
        }
        allocINT(
            lp,
            &raw mut (**blockdata).blockend,
            blockcount + i + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        allocINT(
            lp,
            &raw mut (**blockdata).blockpos,
            blockcount + i + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        );
        if !blockstart.is_null() {
            memcpy(
                (**blockdata).blockend.offset(i as isize) as *mut ::core::ffi::c_void,
                blockstart as *const ::core::ffi::c_void,
                ((blockcount + i + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            );
            if isrow == 0 {
                blockcount += 1;
                *(**blockdata)
                    .blockend
                    .offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
                i = 1 as ::core::ffi::c_int;
                while i < blockcount {
                    *(**blockdata).blockend.offset(i as isize) += (*lp).rows;
                    i += 1;
                }
            }
        } else {
            *(**blockdata)
                .blockend
                .offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
            *(**blockdata)
                .blockpos
                .offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
            if ne == 0 as ::core::ffi::c_int {
                ne = items / blockcount;
                while ne * blockcount < items {
                    ne += 1;
                }
            }
            i = 1 as ::core::ffi::c_int;
            if isrow == 0 {
                *(**blockdata).blockend.offset(i as isize) = *(**blockdata)
                    .blockend
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
                    + (*lp).rows;
                blockcount += 1;
                i += 1;
                items += (*lp).rows;
            }
            while i < blockcount {
                *(**blockdata).blockend.offset(i as isize) = *(**blockdata)
                    .blockend
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
                    + ne;
                i += 1;
            }
            *(**blockdata).blockend.offset(blockcount as isize) = items + 1 as ::core::ffi::c_int;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= blockcount {
            *(**blockdata).blockpos.offset(i as isize) = *(**blockdata)
                .blockend
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    }
    (**blockdata).blockcount = blockcount;
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_get_partialprice"]
pub unsafe extern "C" fn get_partialprice(
    mut lp: *mut lprec,
    mut blockcount: *mut ::core::ffi::c_int,
    mut blockstart: *mut ::core::ffi::c_int,
    mut isrow: ::core::ffi::c_uchar,
) {
    let mut blockdata: *mut partialrec = ::core::ptr::null_mut::<partialrec>();
    if isrow != 0 {
        blockdata = (*lp).rowblocks;
    } else {
        blockdata = (*lp).colblocks;
    }
    *blockcount = partial_countBlocks(lp, isrow);
    if !blockdata.is_null() && !blockstart.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut k: ::core::ffi::c_int = *blockcount;
        if isrow == 0 {
            i += 1;
        }
        memcpy(
            blockstart as *mut ::core::ffi::c_void,
            (*blockdata).blockend.offset(i as isize) as *const ::core::ffi::c_void,
            ((k - i) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        if isrow == 0 {
            k -= i;
            i = 0 as ::core::ffi::c_int;
            while i < k {
                *blockstart.offset(i as isize) -= (*lp).rows;
                i += 1;
            }
        }
    }
}
#[export_name="honest_lpsolve_bb_better"]
pub unsafe extern "C" fn bb_better(
    mut lp: *mut lprec,
    mut target: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut epsvalue: ::core::ffi::c_double = 0.;
    let mut offset: ::core::ffi::c_double = (*lp).epsprimal;
    let mut refvalue: ::core::ffi::c_double = (*lp).infinite;
    let mut testvalue: ::core::ffi::c_double =
        *(*lp).solution.offset(0 as ::core::ffi::c_int as isize);
    let mut ismax: ::core::ffi::c_uchar = is_maxim(lp);
    let mut relgap: ::core::ffi::c_uchar = is_action(mode, OF_TEST_RELGAP);
    let mut fcast: ::core::ffi::c_uchar = is_action(target, OF_PROJECTED);
    let mut delta: ::core::ffi::c_uchar = is_action(target, OF_DELTA);
    if relgap != 0 {
        epsvalue = (*lp).mip_relgap;
        clear_action(&raw mut mode, OF_TEST_RELGAP);
    } else {
        epsvalue = (*lp).mip_absgap;
    }
    if delta != 0 {
        clear_action(&raw mut target, OF_DELTA);
    }
    if fcast != 0 {
        clear_action(&raw mut target, OF_PROJECTED);
    }
    match target {
        OF_RELAXED => {
            refvalue = (*lp).real_solution;
        }
        OF_INCUMBENT => {
            refvalue = *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize);
        }
        OF_WORKING => {
            refvalue = if ismax == 0
                && (*lp).bb_workOF != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                -(*lp).bb_workOF
            } else {
                (*lp).bb_workOF
            };
            if fcast != 0 {
                testvalue = (if ismax == 0
                    && (*(*lp).longsteps).obj_last
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -(*(*lp).longsteps).obj_last
                } else {
                    (*(*lp).longsteps).obj_last
                }) - epsvalue;
            } else {
                testvalue = if ismax == 0
                    && *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize)
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -*(*lp).rhs.offset(0 as ::core::ffi::c_int as isize)
                } else {
                    *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize)
                };
            }
        }
        OF_USERBREAK => {
            refvalue = (*lp).bb_breakOF;
        }
        OF_HEURISTIC => {
            refvalue = (*lp).bb_heuristicOF;
        }
        OF_DUALLIMIT => {
            refvalue = (*lp).bb_limitOF;
        }
        _ => {
            report(
                lp,
                2 as ::core::ffi::c_int,
                b"bb_better: Passed invalid test target '%d'\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_uchar;
        }
    }
    if delta != 0 {
        if epsvalue < (*lp).bb_deltaOF - epsvalue {
            epsvalue = (*lp).bb_deltaOF - epsvalue;
        }
    } else {
        epsvalue = if target >= 3 as ::core::ffi::c_int
            && epsvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -epsvalue
        } else {
            epsvalue
        };
    }
    testvalue += if ismax as ::core::ffi::c_int != 0
        && epsvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        -epsvalue
    } else {
        epsvalue
    };
    if relgap != 0 {
        testvalue = (testvalue - refvalue) / (1.0f64 + fabs(refvalue));
    } else {
        testvalue -= refvalue;
    }
    if mode == OF_TEST_NE {
        relgap = (fabs(testvalue) >= offset) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        testvalue = if mode > 3 as ::core::ffi::c_int
            && testvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -testvalue
        } else {
            testvalue
        };
        testvalue = if ismax as ::core::ffi::c_int != 0
            && testvalue != 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -testvalue
        } else {
            testvalue
        };
        relgap = (testvalue < offset) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    return relgap;
}
#[export_name="honest_lpsolve_construct_solution"]
pub unsafe extern "C" fn construct_solution(
    mut lp: *mut lprec,
    mut target: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut basi: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = (*lp).epsprimal;
    let mut solution: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut value: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut rownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut mat: *mut MATrec = (*lp).matA;
    if target.is_null() {
        solution = (*lp).solution;
    } else {
        solution = target;
    }
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if i == 0 as ::core::ffi::c_int {
            f = unscaled_value(lp, -*(*lp).orig_rhs.offset(i as isize), i);
        } else {
            j = *(*(*lp).presolve_undo).var_to_orig.offset(i as isize);
            if j > 0 as ::core::ffi::c_int {
                f = *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize);
                f = unscaled_value(lp, f, i);
            } else {
                f = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        *solution.offset(i as isize) = f;
        i += 1;
    }
    i = (*lp).rows + 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        *solution.offset(i as isize) = *(*lp).lowbo.offset(i as isize);
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        basi = *(*lp).var_basic.offset(i as isize);
        if basi > (*lp).rows {
            *solution.offset(basi as isize) += *(*lp).rhs.offset(i as isize);
        }
        i += 1;
    }
    i = (*lp).rows + 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        if *(*lp).is_basic.offset(i as isize) == 0 && *(*lp).is_lower.offset(i as isize) == 0 {
            *solution.offset(i as isize) += *(*lp).upbo.offset(i as isize);
        }
        *solution.offset(i as isize) = unscaled_value(lp, *solution.offset(i as isize), i);
        i += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        f = *solution.offset(((*lp).rows + j) as isize);
        if f != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *solution.offset(0 as ::core::ffi::c_int as isize) += f * unscaled_mat(
                lp,
                *(*lp).orig_obj.offset(j as isize),
                0 as ::core::ffi::c_int,
                j,
            );
            i = *(*mat)
                .col_end
                .offset((j - 1 as ::core::ffi::c_int) as isize);
            basi = *(*mat).col_end.offset(j as isize);
            rownr = (*mat).col_mat_rownr.offset(i as isize) as *mut ::core::ffi::c_int;
            value = (*mat).col_mat_value.offset(i as isize) as *mut ::core::ffi::c_double;
            while i < basi {
                *solution.offset(*rownr as isize) += f * unscaled_mat(lp, *value, *rownr, j);
                i += 1;
                rownr = rownr.offset(matRowColStep as isize);
                value = value.offset(matValueStep as isize);
            }
        }
        j += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if fabs(*solution.offset(i as isize)) < epsvalue {
            *solution.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if is_chsign(lp, i) != 0 {
            *solution.offset(i as isize) = if fabs(*solution.offset(i as isize))
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                -*solution.offset(i as isize)
            };
        }
        i += 1;
    }
    if target.is_null() {
        if is_infinite(lp, (*lp).real_solution) != 0 {
            (*lp).bb_workOF = *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize);
            (*lp).real_solution = *solution.offset(0 as ::core::ffi::c_int as isize);
            if is_infinite(lp, (*lp).bb_limitOF) != 0 {
                (*lp).bb_limitOF = (*lp).real_solution;
            } else if is_maxim(lp) != 0 {
                if (*lp).bb_limitOF > (*lp).real_solution {
                    (*lp).bb_limitOF = (*lp).real_solution;
                }
            } else if (*lp).bb_limitOF < (*lp).real_solution {
                (*lp).bb_limitOF = (*lp).real_solution;
            }
            if (*lp).int_vars > 0 as ::core::ffi::c_int
                && mat_validate((*lp).matA) as ::core::ffi::c_int != 0
            {
                let mut fixedOF: ::core::ffi::c_double = unscaled_value(
                    lp,
                    *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                );
                basi = (*lp).columns;
                j = 1 as ::core::ffi::c_int;
                while j <= basi {
                    f = fabs(get_mat(lp, 0 as ::core::ffi::c_int, j))
                        + (*lp).epsint / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                    if f > (*lp).epsint {
                        if is_int(lp, j) == 0
                            || fmod(f, 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                                > (*lp).epsint
                        {
                            break;
                        }
                    }
                    j += 1;
                }
                if j > basi {
                    f = (if is_maxim(lp) as ::core::ffi::c_int != 0
                        && (*lp).real_solution != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -(*lp).real_solution
                    } else {
                        (*lp).real_solution
                    }) + fixedOF;
                    f = floor(f + (1 as ::core::ffi::c_int as ::core::ffi::c_double - epsvalue));
                    f = if is_maxim(lp) as ::core::ffi::c_int != 0
                        && f - fixedOF != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        -(f - fixedOF)
                    } else {
                        f - fixedOF
                    };
                    if is_infinite(lp, (*lp).bb_limitOF) != 0 {
                        (*lp).bb_limitOF = f;
                    } else if is_maxim(lp) != 0 {
                        if (*lp).bb_limitOF > f {
                            (*lp).bb_limitOF = f;
                        }
                    } else if (*lp).bb_limitOF < f {
                        (*lp).bb_limitOF = f;
                    }
                }
            }
            if (*lp).int_vars > 0 as ::core::ffi::c_int
                && (if is_maxim(lp) as ::core::ffi::c_int != 0
                    && (*(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize)
                        - (*lp).bb_limitOF)
                        / (1.0f64 + fabs((*lp).bb_limitOF))
                        != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -((*(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize)
                        - (*lp).bb_limitOF)
                        / (1.0f64 + fabs((*lp).bb_limitOF)))
                } else {
                    (*(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize)
                        - (*lp).bb_limitOF)
                        / (1.0f64 + fabs((*lp).bb_limitOF))
                }) < -epsvalue
            {
                (*lp).spx_status = INFEASIBLE;
                (*lp).bb_break = TRUE as ::core::ffi::c_uchar;
            }
        }
    }
}
#[export_name="honest_lpsolve_check_solution"]
pub unsafe extern "C" fn check_solution(
    mut lp: *mut lprec,
    mut lastcolumn: ::core::ffi::c_int,
    mut solution: *mut ::core::ffi::c_double,
    mut upbo: *mut ::core::ffi::c_double,
    mut lowbo: *mut ::core::ffi::c_double,
    mut tolerance: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut isSC: ::core::ffi::c_uchar = 0;
    let mut test: ::core::ffi::c_double = 0.;
    let mut value: ::core::ffi::c_double = 0.;
    let mut hold: ::core::ffi::c_double = 0.;
    let mut diff: ::core::ffi::c_double = 0.;
    let mut maxdiff: ::core::ffi::c_double = 0.0f64;
    let mut maxerr: ::core::ffi::c_double = 0.0f64;
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut plusum: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut negsum: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut errlevel: ::core::ffi::c_int = IMPORTANT;
    let mut errlimit: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut matColnr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut mat: *mut MATrec = (*lp).matA;
    report(
        lp,
        4 as ::core::ffi::c_int,
        b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if MIP_count(lp) > 0 as ::core::ffi::c_int {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"%s solution  %18.12g after %10.0f iter, %9.0f nodes (gap %.1f%%).\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"Optimal solution  %18.12g after %10.0f iter.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    allocREAL(
        lp,
        &raw mut plusum,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut negsum,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    );
    n = get_nonzeros(lp);
    matRownr = (*mat)
        .col_mat_rownr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    matColnr = (*mat)
        .col_mat_colnr
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    matValue = (*mat)
        .col_mat_value
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        test = unscaled_mat(lp, *matValue, *matRownr, *matColnr);
        test *= *solution.offset(((*lp).rows + *matColnr) as isize);
        if test > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *plusum.offset(*matRownr as isize) += test;
        } else {
            *negsum.offset(*matRownr as isize) += test;
        }
        i += 1;
        matRownr = matRownr.offset(matRowColStep as isize);
        matColnr = matColnr.offset(matRowColStep as isize);
        matValue = matValue.offset(matValueStep as isize);
    }
    n = 0 as ::core::ffi::c_int;
    i = (*lp).rows + 1 as ::core::ffi::c_int;
    while i <= (*lp).rows + lastcolumn {
        value = *solution.offset(i as isize);
        if lowbo.is_null() {
            test = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            test = unscaled_value(lp, *lowbo.offset(i as isize), i);
        }
        isSC = is_semicont(lp, i - (*lp).rows);
        diff = (value - test) / (1.0f64 + fabs(test));
        if diff < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            if isSC as ::core::ffi::c_int != 0
                && value < test / 2 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                test = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if maxerr < fabs(value - test) {
                maxerr = fabs(value - test);
            }
            if maxdiff < fabs(diff) {
                maxdiff = fabs(diff);
            }
        }
        if diff < -tolerance && isSC == 0 {
            if n < errlimit {
                report(
                    lp,
                    errlevel,
                    b"check_solution: Variable   %s = %18.12g is below its lower bound %18.12g\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            n += 1;
        }
        test = unscaled_value(lp, *upbo.offset(i as isize), i);
        diff = (value - test) / (1.0f64 + fabs(test));
        if diff > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            if maxerr < fabs(value - test) {
                maxerr = fabs(value - test);
            }
            if maxdiff < fabs(diff) {
                maxdiff = fabs(diff);
            }
        }
        if diff > tolerance {
            if n < errlimit {
                report(
                    lp,
                    errlevel,
                    b"check_solution: Variable   %s = %18.12g is above its upper bound %18.12g\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            n += 1;
        }
        i += 1;
    }
    let mut current_block_100: u64;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        test = *(*lp).orig_rhs.offset(i as isize);
        if !(is_infinite(lp, test) != 0) {
            j = *(*(*lp).presolve_undo).var_to_orig.offset(i as isize);
            if j != 0 as ::core::ffi::c_int {
                if is_infinite(lp, *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize)) != 0 {
                    current_block_100 = 1847472278776910194;
                } else {
                    test += *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize);
                    current_block_100 = 1423531122933789233;
                }
            } else {
                current_block_100 = 1423531122933789233;
            }
            match current_block_100 {
                1847472278776910194 => {}
                _ => {
                    if is_chsign(lp, i) != 0 {
                        test = if fabs(test) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
                        } else {
                            -test
                        };
                        test += fabs(*upbo.offset(i as isize));
                    }
                    value = *solution.offset(i as isize);
                    test = unscaled_value(lp, test, i);
                    hold = *plusum.offset(i as isize) - *negsum.offset(i as isize);
                    if hold < (*lp).epsvalue {
                        hold = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                    diff = ((value + 1 as ::core::ffi::c_int as ::core::ffi::c_double) / hold
                        - (test + 1 as ::core::ffi::c_int as ::core::ffi::c_double) / hold)
                        / (1.0f64
                            + fabs(
                                (test + 1 as ::core::ffi::c_int as ::core::ffi::c_double) / hold,
                            ));
                    if diff > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        if maxerr < fabs(value - test) {
                            maxerr = fabs(value - test);
                        }
                        if maxdiff < fabs(diff) {
                            maxdiff = fabs(diff);
                        }
                    }
                    if diff > tolerance {
                        if n < errlimit {
                            report(
                                lp,
                                errlevel,
                                b"check_solution: Constraint %s = %18.12g is above its %s %18.12g\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                        n += 1;
                    }
                    test = *(*lp).orig_rhs.offset(i as isize);
                    j = *(*(*lp).presolve_undo).var_to_orig.offset(i as isize);
                    if j != 0 as ::core::ffi::c_int {
                        if is_infinite(lp, *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize))
                            != 0
                        {
                            current_block_100 = 1847472278776910194;
                        } else {
                            test += *(*(*lp).presolve_undo).fixed_rhs.offset(j as isize);
                            current_block_100 = 7858101417678297991;
                        }
                    } else {
                        current_block_100 = 7858101417678297991;
                    }
                    match current_block_100 {
                        1847472278776910194 => {}
                        _ => {
                            value = *solution.offset(i as isize);
                            if is_chsign(lp, i) != 0 {
                                test = if fabs(test)
                                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                                } else {
                                    -test
                                };
                                current_block_100 = 8834769789432328951;
                            } else if is_infinite(lp, *upbo.offset(i as isize)) != 0 {
                                current_block_100 = 1847472278776910194;
                            } else {
                                test -= fabs(*upbo.offset(i as isize));
                                current_block_100 = 8834769789432328951;
                            }
                            match current_block_100 {
                                1847472278776910194 => {}
                                _ => {
                                    test = unscaled_value(lp, test, i);
                                    hold = *plusum.offset(i as isize) - *negsum.offset(i as isize);
                                    if hold < (*lp).epsvalue {
                                        hold = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                                    }
                                    diff = ((value
                                        + 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                                        / hold
                                        - (test
                                            + 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                                            / hold)
                                        / (1.0f64
                                            + fabs(
                                                (test
                                                    + 1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double)
                                                    / hold,
                                            ));
                                    if diff < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        if maxerr < fabs(value - test) {
                                            maxerr = fabs(value - test);
                                        }
                                        if maxdiff < fabs(diff) {
                                            maxdiff = fabs(diff);
                                        }
                                    }
                                    if diff < -tolerance {
                                        if n < errlimit {
                                            report(
                                                lp,
                                                errlevel,
                                                b"check_solution: Constraint %s = %18.12g is below its %s %18.12g\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char,
                                            );
                                        }
                                        n += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if !(plusum as *mut ::core::ffi::c_void).is_null() {
        free(plusum as *mut ::core::ffi::c_void);
        plusum = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(negsum as *mut ::core::ffi::c_void).is_null() {
        free(negsum as *mut ::core::ffi::c_void);
        negsum = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if n > 0 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"\nSeriously low accuracy found ||*|| = %g (rel. error %g)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 5 as ::core::ffi::c_int;
    } else {
        if maxerr > 1.0e-7f64 {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"\nMarginal numeric accuracy ||*|| = %g (rel. error %g)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if maxerr > 1.0e-9f64 {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"\nReasonable numeric accuracy ||*|| = %g (rel. error %g)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if maxerr > 1.0e11f64 {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"\nVery good numeric accuracy ||*|| = %g\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"\nExcellent numeric accuracy ||*|| = %g\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_int;
    };
}
#[export_name="honest_lpsolve_transfer_solution_var"]
pub unsafe extern "C" fn transfer_solution_var(
    mut lp: *mut lprec,
    mut uservar: ::core::ffi::c_int,
) {
    if (*lp).varmap_locked as ::core::ffi::c_int != 0
        && ((*lp).do_presolve & PRESOLVE_LASTMASKMODE != PRESOLVE_NONE) as ::core::ffi::c_int
            as ::core::ffi::c_uchar as ::core::ffi::c_int
            != 0
    {
        uservar += (*lp).rows;
        *(*lp).full_solution.offset(
            ((*(*lp).presolve_undo).orig_rows
                + *(*(*lp).presolve_undo).var_to_orig.offset(uservar as isize))
                as isize,
        ) = *(*lp).best_solution.offset(uservar as isize);
    }
}
#[export_name="honest_lpsolve_transfer_solution"]
pub unsafe extern "C" fn transfer_solution(mut lp: *mut lprec, mut dofinal: ::core::ffi::c_uchar) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    memcpy(
        (*lp).best_solution as *mut ::core::ffi::c_void,
        (*lp).solution as *const ::core::ffi::c_void,
        (((*lp).sum + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    if is_integerscaling(lp) as ::core::ffi::c_int != 0 && (*lp).int_vars > 0 as ::core::ffi::c_int
    {
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).columns {
            if is_int(lp, i) != 0 {
                ii = (*lp).rows + i;
                *(*lp).best_solution.offset(ii as isize) =
                    floor(*(*lp).best_solution.offset(ii as isize) + 0.5f64);
            }
            i += 1;
        }
    }
    if dofinal as ::core::ffi::c_int != 0
        && (*lp).varmap_locked as ::core::ffi::c_int != 0
        && ((*lp).do_presolve & PRESOLVE_LASTMASKMODE != PRESOLVE_NONE) as ::core::ffi::c_int
            as ::core::ffi::c_uchar as ::core::ffi::c_int
            != 0
    {
        let mut psundo: *mut presolveundorec = (*lp).presolve_undo;
        *(*lp).full_solution.offset(0 as ::core::ffi::c_int as isize) =
            *(*lp).best_solution.offset(0 as ::core::ffi::c_int as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            ii = *(*psundo).var_to_orig.offset(i as isize);
            *(*lp).full_solution.offset(ii as isize) = *(*lp).best_solution.offset(i as isize);
            i += 1;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).columns {
            ii = *(*psundo).var_to_orig.offset(((*lp).rows + i) as isize);
            *(*lp)
                .full_solution
                .offset(((*psundo).orig_rows + ii) as isize) =
                *(*lp).best_solution.offset(((*lp).rows + i) as isize);
            i += 1;
        }
    }
}
#[export_name="honest_lpsolve_construct_duals"]
pub unsafe extern "C" fn construct_duals(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut coltarget: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut scale0: ::core::ffi::c_double = 0.;
    let mut value: ::core::ffi::c_double = 0.;
    let mut dualOF: ::core::ffi::c_double = 0.;
    if !(*lp).duals.is_null() {
        free_duals(lp);
    }
    if is_action((*lp).spx_action, ACTION_REBASE) as ::core::ffi::c_int != 0
        || is_action((*lp).spx_action, ACTION_REINVERT) as ::core::ffi::c_int != 0
        || (*lp).basis_valid == 0
        || allocREAL(
            lp,
            &raw mut (*lp).duals,
            (*lp).sum + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
    {
        return 0 as ::core::ffi::c_uchar;
    }
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
    bsolve(
        lp,
        0 as ::core::ffi::c_int,
        (*lp).duals,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        (*lp).epsmachine * DOUBLEROUND,
        1.0f64,
    );
    prod_xA(
        lp,
        coltarget,
        (*lp).duals,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        (*lp).epsmachine,
        1.0f64,
        (*lp).duals,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        MAT_ROUNDDEFAULT | MAT_ROUNDRC,
    );
    mempool_releaseVector(
        (*lp).workarrays,
        coltarget as *mut ::core::ffi::c_char,
        FALSE as ::core::ffi::c_uchar,
    );
    n = (*lp).rows;
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        if *(*lp).is_basic.offset(i as isize) != 0 {
            *(*lp).duals.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else if is_chsign(lp, 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            == is_chsign(lp, i) as ::core::ffi::c_int
            && *(*lp).duals.offset(i as isize) != 0.
        {
            *(*lp).duals.offset(i as isize) = if fabs(*(*lp).duals.offset(i as isize))
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                -*(*lp).duals.offset(i as isize)
            };
        }
        i += 1;
    }
    if is_maxim(lp) != 0 {
        n = (*lp).sum;
        i = (*lp).rows + 1 as ::core::ffi::c_int;
        while i <= n {
            *(*lp).duals.offset(i as isize) = if fabs(*(*lp).duals.offset(i as isize))
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                -*(*lp).duals.offset(i as isize)
            };
            i += 1;
        }
    }
    n = (*(*lp).presolve_undo).orig_sum;
    if (*lp).do_presolve & PRESOLVE_LASTMASKMODE != PRESOLVE_NONE
        && allocREAL(
            lp,
            &raw mut (*lp).full_duals,
            n + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) as ::core::ffi::c_int
            != 0
    {
        let mut ix: ::core::ffi::c_int = 0;
        let mut ii: ::core::ffi::c_int = (*(*lp).presolve_undo).orig_rows;
        n = (*lp).sum;
        ix = 1 as ::core::ffi::c_int;
        while ix <= n {
            i = *(*(*lp).presolve_undo).var_to_orig.offset(ix as isize);
            if ix > (*lp).rows {
                i += ii;
            }
            *(*lp).full_duals.offset(i as isize) = *(*lp).duals.offset(ix as isize);
            ix += 1;
        }
        presolve_rebuildUndo(lp, FALSE as ::core::ffi::c_uchar);
    }
    if (*lp).scaling_used != 0 {
        scale0 = *(*lp).scalars.offset(0 as ::core::ffi::c_int as isize);
    } else {
        scale0 = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    dualOF = (if is_maxim(lp) as ::core::ffi::c_int != 0
        && *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        -*(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize)
    } else {
        *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize)
    }) / scale0;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        value = scaled_value(lp, *(*lp).duals.offset(i as isize) / scale0, i);
        if fabs(value) < (*lp).epsprimal {
            value = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        *(*lp).duals.offset(i as isize) = value;
        if i <= (*lp).rows {
            dualOF += value * *(*lp).solution.offset(i as isize);
        }
        i += 1;
    }
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_construct_sensitivity_duals"]
pub unsafe extern "C" fn construct_sensitivity_duals(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut k: ::core::ffi::c_int = 0;
    let mut varnr: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = TRUE;
    let mut workINT: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut pcol: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut a: ::core::ffi::c_double = 0.;
    let mut infinite: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = 0.;
    let mut from: ::core::ffi::c_double = 0.;
    let mut till: ::core::ffi::c_double = 0.;
    let mut objfromvalue: ::core::ffi::c_double = 0.;
    if !((*lp).objfromvalue as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objfromvalue as *mut ::core::ffi::c_void);
        (*lp).objfromvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).dualsfrom as *mut ::core::ffi::c_void).is_null() {
        free((*lp).dualsfrom as *mut ::core::ffi::c_void);
        (*lp).dualsfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).dualstill as *mut ::core::ffi::c_void).is_null() {
        free((*lp).dualstill as *mut ::core::ffi::c_void);
        (*lp).dualstill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if allocREAL(
        lp,
        &raw mut pcol,
        (*lp).rows + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).objfromvalue,
            (*lp).columns + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).dualsfrom,
            (*lp).sum + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).dualstill,
            (*lp).sum + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
    {
        if !(pcol as *mut ::core::ffi::c_void).is_null() {
            free(pcol as *mut ::core::ffi::c_void);
            pcol = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !((*lp).objfromvalue as *mut ::core::ffi::c_void).is_null() {
            free((*lp).objfromvalue as *mut ::core::ffi::c_void);
            (*lp).objfromvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !((*lp).dualsfrom as *mut ::core::ffi::c_void).is_null() {
            free((*lp).dualsfrom as *mut ::core::ffi::c_void);
            (*lp).dualsfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        if !((*lp).dualstill as *mut ::core::ffi::c_void).is_null() {
            free((*lp).dualstill as *mut ::core::ffi::c_void);
            (*lp).dualstill = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        ok = FALSE;
    } else {
        infinite = (*lp).infinite;
        epsvalue = (*lp).epsmachine;
        varnr = 1 as ::core::ffi::c_int;
        while varnr <= (*lp).sum {
            from = infinite;
            till = infinite;
            objfromvalue = infinite;
            if *(*lp).is_basic.offset(varnr as isize) == 0 {
                if fsolve(
                    lp,
                    varnr,
                    pcol,
                    workINT,
                    epsvalue,
                    1.0f64,
                    FALSE as ::core::ffi::c_uchar,
                ) == 0
                {
                    ok = FALSE;
                    break;
                } else {
                    k = 1 as ::core::ffi::c_int;
                    while k <= (*lp).rows {
                        if fabs(*pcol.offset(k as isize)) > epsvalue {
                            a = *(*lp).rhs.offset(k as isize) / *pcol.offset(k as isize);
                            if varnr > (*lp).rows
                                && fabs(*(*lp).solution.offset(varnr as isize)) <= epsvalue
                                && a < objfromvalue
                                && a >= *(*lp).lowbo.offset(varnr as isize)
                            {
                                objfromvalue = a;
                            }
                            if a <= 0.0f64 && *pcol.offset(k as isize) < 0.0f64 && -a < from {
                                from = if fabs(a)
                                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                                } else {
                                    -a
                                };
                            }
                            if a >= 0.0f64 && *pcol.offset(k as isize) > 0.0f64 && a < till {
                                till = a;
                            }
                            if *(*lp)
                                .upbo
                                .offset(*(*lp).var_basic.offset(k as isize) as isize)
                                < infinite
                            {
                                a = (*(*lp).rhs.offset(k as isize)
                                    - *(*lp)
                                        .upbo
                                        .offset(*(*lp).var_basic.offset(k as isize) as isize))
                                    / *pcol.offset(k as isize);
                                if varnr > (*lp).rows
                                    && fabs(*(*lp).solution.offset(varnr as isize)) <= epsvalue
                                    && a < objfromvalue
                                    && a >= *(*lp).lowbo.offset(varnr as isize)
                                {
                                    objfromvalue = a;
                                }
                                if a <= 0.0f64 && *pcol.offset(k as isize) > 0.0f64 && -a < from {
                                    from = if fabs(a)
                                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    } else {
                                        -a
                                    };
                                }
                                if a >= 0.0f64 && *pcol.offset(k as isize) < 0.0f64 && a < till {
                                    till = a;
                                }
                            }
                        }
                        k += 1;
                    }
                    if *(*lp).is_lower.offset(varnr as isize) == 0 {
                        a = from;
                        from = till;
                        till = a;
                    }
                    if varnr <= (*lp).rows && is_chsign(lp, varnr) == 0 {
                        a = from;
                        from = till;
                        till = a;
                    }
                }
            }
            if from != infinite {
                *(*lp).dualsfrom.offset(varnr as isize) =
                    *(*lp).solution.offset(varnr as isize) - unscaled_value(lp, from, varnr);
            } else {
                *(*lp).dualsfrom.offset(varnr as isize) = -infinite;
            }
            if till != infinite {
                *(*lp).dualstill.offset(varnr as isize) =
                    *(*lp).solution.offset(varnr as isize) + unscaled_value(lp, till, varnr);
            } else {
                *(*lp).dualstill.offset(varnr as isize) = infinite;
            }
            if varnr > (*lp).rows {
                if objfromvalue != infinite {
                    if sensrejvar == 0 || *(*lp).upbo.offset(varnr as isize) != 0.0f64 {
                        if *(*lp).is_lower.offset(varnr as isize) == 0 {
                            objfromvalue = *(*lp).upbo.offset(varnr as isize) - objfromvalue;
                        }
                        if *(*lp).upbo.offset(varnr as isize) < infinite
                            && objfromvalue > *(*lp).upbo.offset(varnr as isize)
                        {
                            objfromvalue = *(*lp).upbo.offset(varnr as isize);
                        }
                    }
                    objfromvalue += *(*lp).lowbo.offset(varnr as isize);
                    objfromvalue = unscaled_value(lp, objfromvalue, varnr);
                } else {
                    objfromvalue = -infinite;
                }
                *(*lp).objfromvalue.offset((varnr - (*lp).rows) as isize) = objfromvalue;
            }
            varnr += 1;
        }
        if !(pcol as *mut ::core::ffi::c_void).is_null() {
            free(pcol as *mut ::core::ffi::c_void);
            pcol = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
    }
    return ok as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_construct_sensitivity_obj"]
pub unsafe extern "C" fn construct_sensitivity_obj(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut varnr: ::core::ffi::c_int = 0;
    let mut row_nr: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = TRUE;
    let mut OrigObj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut drow: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut prow: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut sign: ::core::ffi::c_double = 0.;
    let mut a: ::core::ffi::c_double = 0.;
    let mut min1: ::core::ffi::c_double = 0.;
    let mut min2: ::core::ffi::c_double = 0.;
    let mut infinite: ::core::ffi::c_double = 0.;
    let mut epsvalue: ::core::ffi::c_double = 0.;
    let mut from: ::core::ffi::c_double = 0.;
    let mut till: ::core::ffi::c_double = 0.;
    if !((*lp).objfrom as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objfrom as *mut ::core::ffi::c_void);
        (*lp).objfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).objtill as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objtill as *mut ::core::ffi::c_void);
        (*lp).objtill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    let mut current_block_105: u64;
    if allocREAL(
        lp,
        &raw mut drow,
        (*lp).sum + 1 as ::core::ffi::c_int,
        TRUE as ::core::ffi::c_uchar,
    ) == 0
        || allocREAL(
            lp,
            &raw mut OrigObj,
            (*lp).columns + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut prow,
            (*lp).sum + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).objfrom,
            (*lp).columns + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
        || allocREAL(
            lp,
            &raw mut (*lp).objtill,
            (*lp).columns + 1 as ::core::ffi::c_int,
            AUTOMATIC as ::core::ffi::c_uchar,
        ) == 0
    {
        current_block_105 = 17958792315934771242;
    } else {
        let mut coltarget: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        infinite = (*lp).infinite;
        epsvalue = (*lp).epsmachine;
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
            current_block_105 = 17958792315934771242;
        } else {
            bsolve(
                lp,
                0 as ::core::ffi::c_int,
                drow,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                epsvalue * DOUBLEROUND,
                1.0f64,
            );
            prod_xA(
                lp,
                coltarget,
                drow,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                epsvalue,
                1.0f64,
                drow,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                MAT_ROUNDDEFAULT | MAT_ROUNDRC,
            );
            get_row(lp, 0 as ::core::ffi::c_int, OrigObj);
            i = 1 as ::core::ffi::c_int;
            while i <= (*lp).columns {
                from = -infinite;
                till = infinite;
                varnr = (*lp).rows + i;
                if *(*lp).is_basic.offset(varnr as isize) == 0 {
                    a = unscaled_mat(lp, *drow.offset(varnr as isize), 0 as ::core::ffi::c_int, i);
                    if is_maxim(lp) != 0 {
                        a = -a;
                    }
                    if !(sensrejvar == 0 && *(*lp).upbo.offset(varnr as isize) == 0.0f64) {
                        if (*(*lp).is_lower.offset(varnr as isize) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            == (is_maxim(lp) as ::core::ffi::c_int == FALSE) as ::core::ffi::c_int
                            && a > -epsvalue
                        {
                            from = *OrigObj.offset(i as isize) - a;
                        } else {
                            till = *OrigObj.offset(i as isize) - a;
                        }
                    }
                } else {
                    row_nr = 1 as ::core::ffi::c_int;
                    while row_nr <= (*lp).rows && *(*lp).var_basic.offset(row_nr as isize) != varnr
                    {
                        row_nr += 1;
                    }
                    if row_nr <= (*lp).rows {
                        bsolve(
                            lp,
                            row_nr,
                            prow,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            epsvalue * DOUBLEROUND,
                            1.0f64,
                        );
                        prod_xA(
                            lp,
                            coltarget,
                            prow,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            epsvalue,
                            1.0f64,
                            prow,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            MAT_ROUNDDEFAULT,
                        );
                        sign = (if *(*lp).is_lower.offset(row_nr as isize) as ::core::ffi::c_int
                            != 0
                            && -(1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
                        {
                            --(1 as ::core::ffi::c_int)
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }) as ::core::ffi::c_double;
                        min1 = infinite;
                        min2 = infinite;
                        l = 1 as ::core::ffi::c_int;
                        while l <= (*lp).sum {
                            if *(*lp).is_basic.offset(l as isize) == 0
                                && *(*lp).upbo.offset(l as isize) > 0.0f64
                                && fabs(*prow.offset(l as isize)) > epsvalue
                                && (*drow.offset(l as isize)
                                    * (if *(*lp).is_lower.offset(l as isize) as ::core::ffi::c_int
                                        != 0
                                    {
                                        -(1 as ::core::ffi::c_int)
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })
                                        as ::core::ffi::c_double)
                                    < epsvalue
                            {
                                a = unscaled_mat(
                                    lp,
                                    fabs(*drow.offset(l as isize) / *prow.offset(l as isize)),
                                    0 as ::core::ffi::c_int,
                                    i,
                                );
                                if (*prow.offset(l as isize)
                                    * sign
                                    * (if *(*lp).is_lower.offset(l as isize) as ::core::ffi::c_int
                                        != 0
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        -(1 as ::core::ffi::c_int)
                                    })
                                        as ::core::ffi::c_double)
                                    < 0.0f64
                                {
                                    if a < min1 {
                                        min1 = a;
                                    }
                                } else if a < min2 {
                                    min2 = a;
                                }
                            }
                            l += 1;
                        }
                        if (*(*lp).is_lower.offset(varnr as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            == (is_maxim(lp) as ::core::ffi::c_int == FALSE) as ::core::ffi::c_int
                        {
                            a = min1;
                            min1 = min2;
                            min2 = a;
                        }
                        if min1 < infinite {
                            from = *OrigObj.offset(i as isize) - min1;
                        }
                        if min2 < infinite {
                            till = *OrigObj.offset(i as isize) + min2;
                        }
                        a = *(*lp).solution.offset(varnr as isize);
                        if is_maxim(lp) != 0 {
                            if a - *(*lp).lowbo.offset(varnr as isize) < epsvalue {
                                from = -infinite;
                            } else if (sensrejvar == 0
                                || *(*lp).upbo.offset(varnr as isize) != 0.0f64)
                                && *(*lp).lowbo.offset(varnr as isize)
                                    + *(*lp).upbo.offset(varnr as isize)
                                    - a
                                    < epsvalue
                            {
                                till = infinite;
                            }
                        } else if a - *(*lp).lowbo.offset(varnr as isize) < epsvalue {
                            till = infinite;
                        } else if (sensrejvar == 0 || *(*lp).upbo.offset(varnr as isize) != 0.0f64)
                            && *(*lp).lowbo.offset(varnr as isize)
                                + *(*lp).upbo.offset(varnr as isize)
                                - a
                                < epsvalue
                        {
                            from = -infinite;
                        }
                    }
                }
                *(*lp).objfrom.offset(i as isize) = from;
                *(*lp).objtill.offset(i as isize) = till;
                i += 1;
            }
            mempool_releaseVector(
                (*lp).workarrays,
                coltarget as *mut ::core::ffi::c_char,
                FALSE as ::core::ffi::c_uchar,
            );
            current_block_105 = 15237655884915618618;
        }
    }
    match current_block_105 {
        17958792315934771242 => {
            if !(drow as *mut ::core::ffi::c_void).is_null() {
                free(drow as *mut ::core::ffi::c_void);
                drow = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(OrigObj as *mut ::core::ffi::c_void).is_null() {
                free(OrigObj as *mut ::core::ffi::c_void);
                OrigObj = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(prow as *mut ::core::ffi::c_void).is_null() {
                free(prow as *mut ::core::ffi::c_void);
                prow = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !((*lp).objfrom as *mut ::core::ffi::c_void).is_null() {
                free((*lp).objfrom as *mut ::core::ffi::c_void);
                (*lp).objfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !((*lp).objtill as *mut ::core::ffi::c_void).is_null() {
                free((*lp).objtill as *mut ::core::ffi::c_void);
                (*lp).objtill = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            ok = FALSE;
        }
        _ => {}
    }
    if !(prow as *mut ::core::ffi::c_void).is_null() {
        free(prow as *mut ::core::ffi::c_void);
        prow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(OrigObj as *mut ::core::ffi::c_void).is_null() {
        free(OrigObj as *mut ::core::ffi::c_void);
        OrigObj = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(drow as *mut ::core::ffi::c_void).is_null() {
        free(drow as *mut ::core::ffi::c_void);
        drow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    return ok as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_refactRecent"]
pub unsafe extern "C" fn refactRecent(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    let mut pivcount: ::core::ffi::c_int =
        (*lp).bfp_pivotcount.expect("non-null function pointer")(lp);
    if pivcount == 0 as ::core::ffi::c_int {
        return 2 as ::core::ffi::c_uchar;
    } else if pivcount < 2 as ::core::ffi::c_int * DEF_MAXPIVOTRETRY {
        return 1 as ::core::ffi::c_uchar;
    } else {
        return 0 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_check_if_less"]
pub unsafe extern "C" fn check_if_less(
    mut lp: *mut lprec,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut variable: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    if y < x - scaled_value(lp, (*lp).epsint, variable) {
        if (*lp).bb_trace != 0 {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"check_if_less: Invalid new bound %g should be < %g for %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_uchar;
    } else {
        return 1 as ::core::ffi::c_uchar;
    };
}
#[export_name="honest_lpsolve_findNonBasicSlack"]
pub unsafe extern "C" fn findNonBasicSlack(
    mut lp: *mut lprec,
    mut is_basic: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = (*lp).rows;
    while i > 0 as ::core::ffi::c_int {
        if *is_basic.offset(i as isize) == 0 {
            break;
        }
        i -= 1;
    }
    return i;
}
#[export_name="honest_lpsolve_findBasisPos"]
pub unsafe extern "C" fn findBasisPos(
    mut lp: *mut lprec,
    mut notint: ::core::ffi::c_int,
    mut var_basic: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if var_basic.is_null() {
        var_basic = (*lp).var_basic;
    }
    i = (*lp).rows;
    while i > 0 as ::core::ffi::c_int {
        if *var_basic.offset(i as isize) == notint {
            break;
        }
        i -= 1;
    }
    return i;
}
#[export_name="honest_lpsolve_replaceBasisVar"]
pub unsafe extern "C" fn replaceBasisVar(
    mut lp: *mut lprec,
    mut rownr: ::core::ffi::c_int,
    mut var: ::core::ffi::c_int,
    mut var_basic: *mut ::core::ffi::c_int,
    mut is_basic: *mut ::core::ffi::c_uchar,
) {
    let mut out: ::core::ffi::c_int = 0;
    out = *var_basic.offset(rownr as isize);
    *var_basic.offset(rownr as isize) = var;
    *is_basic.offset(out as isize) = FALSE as ::core::ffi::c_uchar;
    *is_basic.offset(var as isize) = TRUE as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_free_duals"]
pub unsafe extern "C" fn free_duals(mut lp: *mut lprec) {
    if !((*lp).duals as *mut ::core::ffi::c_void).is_null() {
        free((*lp).duals as *mut ::core::ffi::c_void);
        (*lp).duals = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).full_duals as *mut ::core::ffi::c_void).is_null() {
        free((*lp).full_duals as *mut ::core::ffi::c_void);
        (*lp).full_duals = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).dualsfrom as *mut ::core::ffi::c_void).is_null() {
        free((*lp).dualsfrom as *mut ::core::ffi::c_void);
        (*lp).dualsfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).dualstill as *mut ::core::ffi::c_void).is_null() {
        free((*lp).dualstill as *mut ::core::ffi::c_void);
        (*lp).dualstill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).objfromvalue as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objfromvalue as *mut ::core::ffi::c_void);
        (*lp).objfromvalue = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).objfrom as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objfrom as *mut ::core::ffi::c_void);
        (*lp).objfrom = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !((*lp).objtill as *mut ::core::ffi::c_void).is_null() {
        free((*lp).objtill as *mut ::core::ffi::c_void);
        (*lp).objtill = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
}
#[export_name="honest_lpsolve_initialize_solution"]
pub unsafe extern "C" fn initialize_solution(
    mut lp: *mut lprec,
    mut shiftbounds: ::core::ffi::c_uchar,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k1: ::core::ffi::c_int = 0;
    let mut k2: ::core::ffi::c_int = 0;
    let mut matRownr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut colnr: ::core::ffi::c_int = 0;
    let mut theta: ::core::ffi::c_double = 0.;
    let mut value: ::core::ffi::c_double = 0.;
    let mut matValue: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut loB: ::core::ffi::c_double = 0.;
    let mut upB: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    if !(*lp).bb_bounds.is_null() {
        if shiftbounds as ::core::ffi::c_int == INITSOL_SHIFTZERO {
            if (*(*lp).bb_bounds).UBzerobased != 0 {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"initialize_solution: The upper bounds are already zero-based at refactorization %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            (*(*lp).bb_bounds).UBzerobased = TRUE as ::core::ffi::c_uchar;
        } else if (*(*lp).bb_bounds).UBzerobased == 0 {
            report(
                lp,
                2 as ::core::ffi::c_int,
                b"initialize_solution: The upper bounds are not zero-based at refactorization %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
    i = (is_action((*lp).anti_degen, ANTIDEGEN_RHSPERTURB) as ::core::ffi::c_int != 0
        && !(*lp).monitor.is_null()
        && (*(*lp).monitor).active as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
        && i == 0
    {
        memcpy(
            (*lp).rhs as *mut ::core::ffi::c_void,
            (*lp).orig_rhs as *const ::core::ffi::c_void,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    } else if i != 0 {
        *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) =
            *(*lp).orig_rhs.offset(0 as ::core::ffi::c_int as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            if is_constr_type(lp, i, EQ) != 0 {
                theta = rand_uniform(lp, (*lp).epsvalue);
            } else {
                theta = rand_uniform(lp, (*lp).epsperturb);
            }
            *(*lp).rhs.offset(i as isize) = *(*lp).orig_rhs.offset(i as isize) + theta;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i <= (*lp).rows {
            *(*lp).rhs.offset(i as isize) = *(*lp).orig_rhs.offset(i as isize);
            i += 1;
        }
    }
    let mut current_block_59: u64;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).sum {
        upB = *(*lp).upbo.offset(i as isize);
        loB = *(*lp).lowbo.offset(i as isize);
        if shiftbounds as ::core::ffi::c_int == INITSOL_SHIFTZERO {
            if loB > -(*lp).infinite && upB < (*lp).infinite {
                *(*lp).upbo.offset(i as isize) -= loB;
            }
            if *(*lp).upbo.offset(i as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                report(
                    lp,
                    2 as ::core::ffi::c_int,
                    b"initialize_solution: Invalid rebounding; variable %d at refact %d, iter %.0f\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            current_block_59 = 10891380440665537214;
        } else if shiftbounds as ::core::ffi::c_int == INITSOL_USEZERO {
            if loB > -(*lp).infinite && upB < (*lp).infinite {
                upB += loB;
            }
            current_block_59 = 10891380440665537214;
        } else if shiftbounds as ::core::ffi::c_int == INITSOL_ORIGINAL {
            if loB > -(*lp).infinite && upB < (*lp).infinite {
                *(*lp).upbo.offset(i as isize) += loB;
                upB += loB;
            }
            current_block_59 = 9828876828309294594;
        } else {
            report(
                lp,
                2 as ::core::ffi::c_int,
                b"initialize_solution: Invalid option value '%d'\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            current_block_59 = 10891380440665537214;
        }
        match current_block_59 {
            10891380440665537214 => {
                if *(*lp).is_lower.offset(i as isize) != 0 {
                    theta = loB;
                } else {
                    theta = upB;
                }
                if !(theta == 0 as ::core::ffi::c_int as ::core::ffi::c_double) {
                    if i > (*lp).rows {
                        colnr = i - (*lp).rows;
                        k1 = *(*mat)
                            .col_end
                            .offset((colnr - 1 as ::core::ffi::c_int) as isize);
                        k2 = *(*mat).col_end.offset(colnr as isize);
                        matRownr =
                            (*mat).col_mat_rownr.offset(k1 as isize) as *mut ::core::ffi::c_int;
                        matValue =
                            (*mat).col_mat_value.offset(k1 as isize) as *mut ::core::ffi::c_double;
                        value = get_OF_active(lp, i, theta);
                        *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) -= value;
                        while k1 < k2 {
                            *(*lp).rhs.offset(*matRownr as isize) -= theta * *matValue;
                            k1 += 1;
                            matRownr = matRownr.offset(matRowColStep as isize);
                            matValue = matValue.offset(matValueStep as isize);
                        }
                    } else {
                        *(*lp).rhs.offset(i as isize) -= theta;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    i = lps_idamax((*lp).rows, (*lp).rhs, 1 as ::core::ffi::c_int);
    (*lp).rhsmax = fabs(*(*lp).rhs.offset(i as isize));
    if shiftbounds as ::core::ffi::c_int == INITSOL_SHIFTZERO {
        clear_action(&raw mut (*lp).spx_action, ACTION_REBASE);
    }
}
#[export_name="honest_lpsolve_recompute_solution"]
pub unsafe extern "C" fn recompute_solution(
    mut lp: *mut lprec,
    mut shiftbounds: ::core::ffi::c_uchar,
) {
    initialize_solution(lp, shiftbounds);
    (*lp).bfp_ftran_normal.expect("non-null function pointer")(
        lp,
        (*lp).rhs,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if (*lp).obj_in_basis == 0 {
        let mut i: ::core::ffi::c_int = 0;
        let mut ib: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = (*lp).rows;
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            ib = *(*lp).var_basic.offset(i as isize);
            if ib > n {
                *(*lp).rhs.offset(0 as ::core::ffi::c_int as isize) -=
                    get_OF_active(lp, ib, *(*lp).rhs.offset(i as isize));
            }
            i += 1;
        }
    }
    roundVector((*lp).rhs, (*lp).rows, (*lp).epsvalue);
    clear_action(&raw mut (*lp).spx_action, ACTION_RECOMPUTE);
}
#[export_name="honest_lpsolve_verify_solution"]
pub unsafe extern "C" fn verify_solution(
    mut lp: *mut lprec,
    mut reinvert: ::core::ffi::c_uchar,
    mut info: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut oldmap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut newmap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut refmap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut oldrhs: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut err: ::core::ffi::c_double = 0.;
    let mut errmax: ::core::ffi::c_double = 0.;
    allocINT(
        lp,
        &raw mut oldmap,
        (*lp).rows + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    allocINT(
        lp,
        &raw mut newmap,
        (*lp).rows + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    allocREAL(
        lp,
        &raw mut oldrhs,
        (*lp).rows + 1 as ::core::ffi::c_int,
        FALSE as ::core::ffi::c_uchar,
    );
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        *oldmap.offset(i as isize) = i;
        i += 1;
    }
    if reinvert != 0 {
        allocINT(
            lp,
            &raw mut refmap,
            (*lp).rows + 1 as ::core::ffi::c_int,
            FALSE as ::core::ffi::c_uchar,
        );
        memcpy(
            refmap as *mut ::core::ffi::c_void,
            (*lp).var_basic as *const ::core::ffi::c_void,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        sortByINT(
            oldmap,
            refmap,
            (*lp).rows,
            1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    memcpy(
        oldrhs as *mut ::core::ffi::c_void,
        (*lp).rhs as *const ::core::ffi::c_void,
        (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    if reinvert != 0 {
        invert(
            lp,
            INITSOL_USEZERO as ::core::ffi::c_uchar,
            FALSE as ::core::ffi::c_uchar,
        );
    } else {
        recompute_solution(lp, INITSOL_USEZERO as ::core::ffi::c_uchar);
    }
    i = 0 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        *newmap.offset(i as isize) = i;
        i += 1;
    }
    if reinvert != 0 {
        memcpy(
            refmap as *mut ::core::ffi::c_void,
            (*lp).var_basic as *const ::core::ffi::c_void,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        sortByINT(
            newmap,
            refmap,
            (*lp).rows,
            1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        );
    }
    errmax = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    ii = -(1 as ::core::ffi::c_int);
    n = 0 as ::core::ffi::c_int;
    i = (*lp).rows;
    while i > 0 as ::core::ffi::c_int {
        err = fabs(
            (*oldrhs.offset(*oldmap.offset(i as isize) as isize)
                - *(*lp).rhs.offset(*newmap.offset(i as isize) as isize))
                / (1.0f64 + fabs(*(*lp).rhs.offset(*newmap.offset(i as isize) as isize))),
        );
        if err > (*lp).epsprimal {
            n += 1;
            if err > errmax {
                ii = i;
                errmax = err;
            }
        }
        i -= 1;
    }
    err = fabs(
        (*oldrhs.offset(i as isize) - *(*lp).rhs.offset(i as isize))
            / (1.0f64 + fabs(*(*lp).rhs.offset(i as isize))),
    );
    if err < (*lp).epspivot {
        i -= 1;
        err = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        n += 1;
        if ii < 0 as ::core::ffi::c_int {
            ii = 0 as ::core::ffi::c_int;
            errmax = err;
        }
    }
    if n > 0 as ::core::ffi::c_int {
        report(
            lp,
            3 as ::core::ffi::c_int,
            b"verify_solution: Iter %.0f %s - %d errors; OF %g, Max @row %d %g\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if reinvert == 0 {
        memcpy(
            (*lp).rhs as *mut ::core::ffi::c_void,
            oldrhs as *const ::core::ffi::c_void,
            (((*lp).rows + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        );
    }
    if !(oldmap as *mut ::core::ffi::c_void).is_null() {
        free(oldmap as *mut ::core::ffi::c_void);
        oldmap = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(newmap as *mut ::core::ffi::c_void).is_null() {
        free(newmap as *mut ::core::ffi::c_void);
        newmap = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(oldrhs as *mut ::core::ffi::c_void).is_null() {
        free(oldrhs as *mut ::core::ffi::c_void);
        oldrhs = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if reinvert != 0 {
        if !(refmap as *mut ::core::ffi::c_void).is_null() {
            free(refmap as *mut ::core::ffi::c_void);
            refmap = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    return ii;
}
#[export_name="honest_lpsolve_identify_GUB"]
pub unsafe extern "C" fn identify_GUB(
    mut lp: *mut lprec,
    mut mark: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut knint: ::core::ffi::c_int = 0;
    let mut srh: ::core::ffi::c_int = 0;
    let mut rh: ::core::ffi::c_double = 0.;
    let mut mv: ::core::ffi::c_double = 0.;
    let mut tv: ::core::ffi::c_double = 0.;
    let mut bv: ::core::ffi::c_double = 0.;
    let mut mat: *mut MATrec = (*lp).matA;
    if (*lp).equalities == 0 as ::core::ffi::c_int || mat_validate(mat) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    k = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if !(is_constr_type(lp, i, EQ) == 0) {
            rh = get_rh(lp, i);
            srh = if rh < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
            knint = 0 as ::core::ffi::c_int;
            je = *(*mat).row_end.offset(i as isize);
            jb = *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            while jb < je {
                j = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                if is_int(lp, j) == 0 {
                    knint += 1;
                }
                if knint > 1 as ::core::ffi::c_int {
                    break;
                }
                mv = get_mat_byindex(
                    lp,
                    jb,
                    TRUE as ::core::ffi::c_uchar,
                    FALSE as ::core::ffi::c_uchar,
                );
                if fabs((mv - rh) / (1.0f64 + fabs(rh))) > (*lp).epsprimal {
                    break;
                }
                tv = mv * get_upbo(lp, j);
                bv = get_lowbo(lp, j);
                if srh as ::core::ffi::c_double * (tv - rh) < -(*lp).epsprimal
                    || bv != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    break;
                }
                jb += 1;
            }
            if jb == je {
                k += 1;
                if mark as ::core::ffi::c_int == TRUE {
                    *(*lp).row_type.offset(i as isize) |= ROWTYPE_GUB;
                } else if mark as ::core::ffi::c_int == AUTOMATIC {
                    break;
                }
            }
        }
        i += 1;
    }
    return k;
}
#[export_name="honest_lpsolve_prepare_GUB"]
pub unsafe extern "C" fn prepare_GUB(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jb: ::core::ffi::c_int = 0;
    let mut je: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut members: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut rh: ::core::ffi::c_double = 0.;
    let mut GUBname: [::core::ffi::c_char; 16] = [0; 16];
    let mut mat: *mut MATrec = (*lp).matA;
    if (*lp).equalities == 0 as ::core::ffi::c_int
        || allocINT(
            lp,
            &raw mut members,
            (*lp).columns + 1 as ::core::ffi::c_int,
            TRUE as ::core::ffi::c_uchar,
        ) == 0
        || mat_validate(mat) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*lp).rows {
        if !(*(*lp).row_type.offset(i as isize) & ROWTYPE_GUB == 0) {
            k = 0 as ::core::ffi::c_int;
            je = *(*mat).row_end.offset(i as isize);
            jb = *(*mat)
                .row_end
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            k = 0 as ::core::ffi::c_int;
            while jb < je {
                *members.offset(k as isize) = *(*mat)
                    .col_mat_colnr
                    .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                k += 1;
                jb += 1;
            }
            j = GUB_count(lp) + 1 as ::core::ffi::c_int;
            native_only!(snprintf,
                &raw mut GUBname as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                b"GUB_%d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
            add_GUB(
                lp,
                &raw mut GUBname as *mut ::core::ffi::c_char,
                j,
                k,
                members,
            );
            clear_action(
                (*lp).row_type.offset(i as isize) as *mut ::core::ffi::c_int,
                ROWTYPE_GUB,
            );
            rh = get_rh(lp, i);
            if fabs(
                (rh - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                    / (1.0f64 + fabs(1 as ::core::ffi::c_int as ::core::ffi::c_double)),
            ) > (*lp).epsprimal
            {
                set_rh(lp, i, 1 as ::core::ffi::c_int as ::core::ffi::c_double);
                jb = *(*mat)
                    .row_end
                    .offset((i - 1 as ::core::ffi::c_int) as isize);
                while jb < je {
                    j = *(*mat)
                        .col_mat_colnr
                        .offset(*(*mat).row_mat.offset(jb as isize) as isize);
                    set_mat(lp, i, j, 1 as ::core::ffi::c_int as ::core::ffi::c_double);
                    jb += 1;
                }
            }
        }
        i += 1;
    }
    if !(members as *mut ::core::ffi::c_void).is_null() {
        free(members as *mut ::core::ffi::c_void);
        members = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    return GUB_count(lp);
}
#[export_name="honest_lpsolve_pre_MIPOBJ"]
pub unsafe extern "C" fn pre_MIPOBJ(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    (*lp).bb_deltaOF = MIP_stepOF(lp);
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_post_MIPOBJ"]
pub unsafe extern "C" fn post_MIPOBJ(mut lp: *mut lprec) -> ::core::ffi::c_uchar {
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_preprocess"]
pub unsafe extern "C" fn preprocess(mut lp: *mut lprec) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = TRUE;
    let mut new_index: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut hold: ::core::ffi::c_double = 0.;
    let mut new_column: *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut scaled: ::core::ffi::c_uchar = 0;
    let mut primal1: ::core::ffi::c_uchar = 0;
    let mut primal2: ::core::ffi::c_uchar = 0;
    if (*lp).wasPreprocessed != 0 {
        return ok;
    }
    if (*lp).lag_status != RUNNING {
        let mut doPP: ::core::ffi::c_uchar = 0;
        primal1 = ((*lp).simplex_strategy & SIMPLEX_Phase1_PRIMAL) as ::core::ffi::c_uchar;
        primal2 = ((*lp).simplex_strategy & SIMPLEX_Phase2_PRIMAL) as ::core::ffi::c_uchar;
        doPP = is_piv_mode(lp, PRICE_PARTIAL | PRICE_AUTOPARTIAL);
        if doPP != 0 {
            i = partial_findBlocks(
                lp,
                FALSE as ::core::ffi::c_uchar,
                FALSE as ::core::ffi::c_uchar,
            );
            if i < 4 as ::core::ffi::c_int {
                i = (5 as ::core::ffi::c_int as ::core::ffi::c_double
                    * log((*lp).columns as ::core::ffi::c_double
                        / (*lp).rows as ::core::ffi::c_double))
                    as ::core::ffi::c_int;
            }
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"The model is %s to have %d column blocks/stages.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            set_partialprice(
                lp,
                i,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                FALSE as ::core::ffi::c_uchar,
            );
        }
        if doPP != 0 {
            i = partial_findBlocks(
                lp,
                FALSE as ::core::ffi::c_uchar,
                TRUE as ::core::ffi::c_uchar,
            );
            if i < 4 as ::core::ffi::c_int {
                i = (5 as ::core::ffi::c_int as ::core::ffi::c_double
                    * log((*lp).rows as ::core::ffi::c_double
                        / (*lp).columns as ::core::ffi::c_double))
                    as ::core::ffi::c_int;
            }
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"The model is %s to have %d row blocks/stages.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            set_partialprice(
                lp,
                i,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                TRUE as ::core::ffi::c_uchar,
            );
        }
        if doPP == 0 && is_piv_mode(lp, PRICE_PARTIAL) as ::core::ffi::c_int != 0 {
            if (*lp).rowblocks.is_null() || (*lp).colblocks.is_null() {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"Ignoring partial pricing, since block structures are not defined.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                clear_action(&raw mut (*lp).piv_strategy, PRICE_PARTIAL);
            }
        }
        if is_piv_mode(lp, PRICE_MULTIPLE) as ::core::ffi::c_int != 0
            && (primal1 as ::core::ffi::c_int != 0 || primal2 as ::core::ffi::c_int != 0)
        {
            doPP = is_piv_mode(lp, PRICE_AUTOMULTIPLE);
            if doPP != 0 {
                i = (2.5f64 * log((*lp).sum as ::core::ffi::c_double)) as ::core::ffi::c_int;
                if i < 1 as ::core::ffi::c_int {
                    i = 1 as ::core::ffi::c_int;
                }
                set_multiprice(lp, i);
            }
            if (*lp).multiblockdiv > 1 as ::core::ffi::c_int {
                report(
                    lp,
                    4 as ::core::ffi::c_int,
                    b"Using %d-candidate primal simplex multiple pricing block.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        } else {
            set_multiprice(lp, 1 as ::core::ffi::c_int);
        }
        report(
            lp,
            4 as ::core::ffi::c_int,
            b"Using %s simplex for phase 1 and %s simplex for phase 2.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        i = get_piv_rule(lp);
        if i == PRICER_STEEPESTEDGE
            && is_piv_mode(lp, PRICE_PRIMALFALLBACK) as ::core::ffi::c_int != 0
        {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"The pricing strategy is set to '%s' for the dual and '%s' for the primal.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            report(
                lp,
                4 as ::core::ffi::c_int,
                b"The primal and dual simplex pricing strategy set to '%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        report(
            lp,
            4 as ::core::ffi::c_int,
            b" \n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    pre_MIPOBJ(lp);
    let mut current_block_91: u64;
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        i = (*lp).rows + j;
        hold = *(*lp).orig_upbo.offset(i as isize);
        if hold < (*lp).infinite
            && (fabs(*(*lp).orig_lowbo.offset(i as isize)) >= (*lp).infinite) as ::core::ffi::c_int
                as ::core::ffi::c_uchar as ::core::ffi::c_int
                != 0
            || fullybounded == 0
                && (fabs((*lp).negrange) >= (*lp).infinite) as ::core::ffi::c_int
                    as ::core::ffi::c_uchar
                    == 0
                && hold < -(*lp).negrange
                && *(*lp).orig_lowbo.offset(i as isize) <= (*lp).negrange
        {
            if !(*lp).var_is_free.is_null()
                && *(*lp).var_is_free.offset(j as isize) > 0 as ::core::ffi::c_int
            {
                del_column(lp, *(*lp).var_is_free.offset(j as isize));
            }
            mat_multcol(
                (*lp).matA,
                j,
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                TRUE as ::core::ffi::c_uchar,
            );
            if (*lp).var_is_free.is_null() {
                if allocINT(
                    lp,
                    &raw mut (*lp).var_is_free,
                    (if (*lp).columns > (*lp).columns_alloc {
                        (*lp).columns
                    } else {
                        (*lp).columns_alloc
                    }) + 1 as ::core::ffi::c_int,
                    TRUE as ::core::ffi::c_uchar,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            *(*lp).var_is_free.offset(j as isize) = -j;
            *(*lp).orig_upbo.offset(i as isize) = if fabs(*(*lp).orig_lowbo.offset(i as isize))
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                -*(*lp).orig_lowbo.offset(i as isize)
            };
            *(*lp).orig_lowbo.offset(i as isize) =
                if fabs(hold) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    -hold
                };
            if *(*lp).sc_lobound.offset(j as isize)
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                *(*lp).sc_lobound.offset(j as isize) = *(*lp).orig_lowbo.offset(i as isize);
                *(*lp).orig_lowbo.offset(i as isize) =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            current_block_91 = 16231175055492490595;
        } else if *(*lp).orig_lowbo.offset(i as isize) <= (*lp).negrange && hold >= -(*lp).negrange
        {
            if (*lp).var_is_free.is_null() {
                if allocINT(
                    lp,
                    &raw mut (*lp).var_is_free,
                    (if (*lp).columns > (*lp).columns_alloc {
                        (*lp).columns
                    } else {
                        (*lp).columns_alloc
                    }) + 1 as ::core::ffi::c_int,
                    TRUE as ::core::ffi::c_uchar,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if *(*lp).var_is_free.offset(j as isize) <= 0 as ::core::ffi::c_int {
                if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, i - (*lp).rows) != 0 {
                    report(
                        lp,
                        3 as ::core::ffi::c_int,
                        b"preprocess: Converted negative bound for SOS variable %d to zero\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    *(*lp).orig_lowbo.offset(i as isize) =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    current_block_91 = 6450636197030046351;
                } else {
                    if new_column.is_null() {
                        if allocREAL(
                            lp,
                            &raw mut new_column,
                            (*lp).rows + 1 as ::core::ffi::c_int,
                            FALSE as ::core::ffi::c_uchar,
                        ) == 0
                            || allocINT(
                                lp,
                                &raw mut new_index,
                                (*lp).rows + 1 as ::core::ffi::c_int,
                                FALSE as ::core::ffi::c_uchar,
                            ) == 0
                        {
                            ok = FALSE;
                            break;
                        }
                    }
                    scaled = (*lp).scaling_used;
                    (*lp).scaling_used = FALSE as ::core::ffi::c_uchar;
                    k = get_columnex(lp, j, new_column, new_index);
                    if add_columnex(lp, k, new_column, new_index) == 0 {
                        ok = FALSE;
                        break;
                    } else {
                        mat_multcol(
                            (*lp).matA,
                            (*lp).columns,
                            -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                            TRUE as ::core::ffi::c_uchar,
                        );
                        if scaled != 0 {
                            *(*lp).scalars.offset(((*lp).rows + (*lp).columns) as isize) =
                                *(*lp).scalars.offset(i as isize);
                        }
                        (*lp).scaling_used = scaled;
                        if (*lp).names_used as ::core::ffi::c_int != 0
                            && (*(*lp).col_name.offset(j as isize)).is_null()
                        {
                            let mut fieldn: [::core::ffi::c_char; 50] = [0; 50];
                            native_only!(snprintf,
                                &raw mut fieldn as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as size_t,
                                b"__AntiBodyOf(%d)__\0" as *const u8 as *const ::core::ffi::c_char,
                                j,
                            );
                            if set_col_name(
                                lp,
                                (*lp).columns,
                                &raw mut fieldn as *mut ::core::ffi::c_char,
                            ) == 0
                            {
                                ok = FALSE;
                                break;
                            }
                        }
                        *(*lp).var_is_free.offset(j as isize) = (*lp).columns;
                    }
                    current_block_91 = 2723324002591448311;
                }
            } else {
                current_block_91 = 2723324002591448311;
            }
            match current_block_91 {
                6450636197030046351 => {}
                _ => {
                    *(*lp)
                        .orig_upbo
                        .offset(((*lp).rows + *(*lp).var_is_free.offset(j as isize)) as isize) =
                        if fabs(*(*lp).orig_lowbo.offset(i as isize))
                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
                        } else {
                            -*(*lp).orig_lowbo.offset(i as isize)
                        };
                    *(*lp).orig_lowbo.offset(i as isize) =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    *(*lp)
                        .var_is_free
                        .offset(*(*lp).var_is_free.offset(j as isize) as isize) = -j;
                    *(*lp)
                        .var_type
                        .offset(*(*lp).var_is_free.offset(j as isize) as isize) =
                        *(*lp).var_type.offset(j as isize);
                    current_block_91 = 16231175055492490595;
                }
            }
        } else {
            if *(*lp).sc_lobound.offset(j as isize)
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                *(*lp).sc_lobound.offset(j as isize) = *(*lp).orig_lowbo.offset(i as isize);
                *(*lp).orig_lowbo.offset(i as isize) =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            current_block_91 = 16231175055492490595;
        }
        match current_block_91 {
            16231175055492490595 => {
                if SOS_is_member((*lp).SOS, 0 as ::core::ffi::c_int, j) != 0
                    && is_int(lp, j) as ::core::ffi::c_int != 0
                {
                    (*lp).sos_ints += 1;
                }
            }
            _ => {}
        }
        j += 1;
    }
    if !(new_column as *mut ::core::ffi::c_void).is_null() {
        free(new_column as *mut ::core::ffi::c_void);
        new_column = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !(new_index as *mut ::core::ffi::c_void).is_null() {
        free(new_index as *mut ::core::ffi::c_void);
        new_index = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if MIP_count(lp) > 0 as ::core::ffi::c_int
        && is_bb_mode(lp, NODE_GUBMODE) as ::core::ffi::c_int != 0
        && identify_GUB(lp, AUTOMATIC as ::core::ffi::c_uchar) > 0 as ::core::ffi::c_int
    {
        prepare_GUB(lp);
    }
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
    if ok != 0 {
        *(*lp).nzdrow.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    }
    memopt_lp(
        lp,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    (*lp).wasPreprocessed = TRUE as ::core::ffi::c_uchar;
    return ok;
}
pub const fullybounded: ::core::ffi::c_int = FALSE;
#[export_name="honest_lpsolve_postprocess"]
pub unsafe extern "C" fn postprocess(mut lp: *mut lprec) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_double = 0.;
    if (*lp).wasPreprocessed == 0 {
        return;
    }
    if (*lp).bb_totalnodes == 0 as ::core::ffi::c_longlong && (*lp).var_is_free.is_null() {
        if is_presolve(lp, PRESOLVE_DUALS) != 0 {
            construct_duals(lp);
        }
        if is_presolve(lp, PRESOLVE_SENSDUALS) != 0 {
            if construct_sensitivity_duals(lp) == 0 || construct_sensitivity_obj(lp) == 0 {
                report(
                    lp,
                    3 as ::core::ffi::c_int,
                    b"postprocess: Unable to allocate working memory for duals.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    j = 1 as ::core::ffi::c_int;
    while j <= (*lp).columns {
        i = (*lp).rows + j;
        if !(*lp).var_is_free.is_null()
            && *(*lp).var_is_free.offset(j as isize) < 0 as ::core::ffi::c_int
        {
            if -*(*lp).var_is_free.offset(j as isize) == j {
                mat_multcol(
                    (*lp).matA,
                    j,
                    -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                    TRUE as ::core::ffi::c_uchar,
                );
                hold = *(*lp).orig_upbo.offset(i as isize);
                *(*lp).orig_upbo.offset(i as isize) = if fabs(*(*lp).orig_lowbo.offset(i as isize))
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    -*(*lp).orig_lowbo.offset(i as isize)
                };
                *(*lp).orig_lowbo.offset(i as isize) =
                    if fabs(hold) == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        0 as ::core::ffi::c_int as ::core::ffi::c_double
                    } else {
                        -hold
                    };
                *(*lp).best_solution.offset(i as isize) =
                    if fabs(*(*lp).best_solution.offset(i as isize))
                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        0 as ::core::ffi::c_int as ::core::ffi::c_double
                    } else {
                        -*(*lp).best_solution.offset(i as isize)
                    };
                transfer_solution_var(lp, j);
                *(*lp).var_is_free.offset(j as isize) = 0 as ::core::ffi::c_int;
                if *(*lp).sc_lobound.offset(j as isize)
                    > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    *(*lp).orig_lowbo.offset(((*lp).rows + j) as isize) =
                        -*(*lp).sc_lobound.offset(j as isize);
                }
            }
        } else if !(*lp).var_is_free.is_null()
            && *(*lp).var_is_free.offset(j as isize) > 0 as ::core::ffi::c_int
        {
            ii = *(*lp).var_is_free.offset(j as isize);
            ii += (*lp).rows;
            *(*lp).best_solution.offset(i as isize) -= *(*lp).best_solution.offset(ii as isize);
            transfer_solution_var(lp, j);
            *(*lp).best_solution.offset(ii as isize) =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *(*lp).orig_lowbo.offset(i as isize) = if fabs(*(*lp).orig_upbo.offset(ii as isize))
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                -*(*lp).orig_upbo.offset(ii as isize)
            };
        } else if *(*lp).sc_lobound.offset(j as isize)
            > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *(*lp).orig_lowbo.offset(i as isize) = *(*lp).sc_lobound.offset(j as isize);
        }
        j += 1;
    }
    del_splitvars(lp);
    post_MIPOBJ(lp);
    if (*lp).verbose > NORMAL {
        REPORT_extended(lp);
    }
    (*lp).wasPreprocessed = FALSE as ::core::ffi::c_uchar;
}
pub const FULLYBOUNDEDSIMPLEX: ::core::ffi::c_int = FALSE;
pub const libnameBLAS: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"myBLAS\0") };
pub const DEF_OBJINBASIS: ::core::ffi::c_int = TRUE;
pub const MAJORVERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MINORVERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const RELEASE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BUILD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BFPVERSION: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const XLIVERSION: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
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
pub const LIB_LOADED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIB_NOTFOUND: ::core::ffi::c_int = 1;
pub const LIB_NOINFO: ::core::ffi::c_int = 2;
pub const LIB_NOFUNCTION: ::core::ffi::c_int = 3;
pub const LIB_VERINVALID: ::core::ffi::c_int = 4;
pub const LIB_STR_LOADED: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"Successfully loaded\0")
};
pub const LIB_STR_NOTFOUND: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"File not found\0") };
pub const LIB_STR_NOINFO: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"No version data\0") };
pub const LIB_STR_NOFUNCTION: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"Missing function header\0")
};
pub const LIB_STR_VERINVALID: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"Incompatible version\0")
};
pub const RTLD_LAZY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SIMPLEX_Phase1_PRIMAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIMPLEX_Phase1_DUAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIMPLEX_Phase2_PRIMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIMPLEX_Phase2_DUAL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIMPLEX_DYNAMIC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SIMPLEX_PRIMAL_PRIMAL: ::core::ffi::c_int = SIMPLEX_Phase1_PRIMAL + SIMPLEX_Phase2_PRIMAL;
pub const SIMPLEX_DUAL_PRIMAL: ::core::ffi::c_int = SIMPLEX_Phase1_DUAL + SIMPLEX_Phase2_PRIMAL;
pub const SIMPLEX_DUAL_DUAL: ::core::ffi::c_int = SIMPLEX_Phase1_DUAL + SIMPLEX_Phase2_DUAL;
pub const ISREAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ISINTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ISSEMI: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ISSOS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PRESOLVE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PRESOLVE_REDUCEMIP: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const PRESOLVE_LASTMASKMODE: ::core::ffi::c_int = PRESOLVE_DUALS - 1 as ::core::ffi::c_int;
pub const PRESOLVE_DUALS: ::core::ffi::c_int = 524288 as ::core::ffi::c_int;
pub const PRESOLVE_SENSDUALS: ::core::ffi::c_int = 1048576 as ::core::ffi::c_int;
pub const CRASH_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INITSOL_SHIFTZERO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INITSOL_USEZERO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INITSOL_ORIGINAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ANTIDEGEN_FIXEDVARS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTIDEGEN_STALLING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTIDEGEN_RHSPERTURB: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const ANTIDEGEN_DEFAULT: ::core::ffi::c_int = ANTIDEGEN_FIXEDVARS | ANTIDEGEN_STALLING;
pub const IMPORTANT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NORMAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MSG_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MSG_ITERATION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MSG_INITPSEUDOCOST: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const MPSFIXED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MPSFREE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ROWTYPE_EMPTY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ROWTYPE_LE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ROWTYPE_GE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ROWTYPE_EQ: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ROWTYPE_CONSTRAINT: ::core::ffi::c_int = ROWTYPE_EQ;
pub const ROWTYPE_OF: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ROWTYPE_GUB: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const ROWTYPE_OFMAX: ::core::ffi::c_int = ROWTYPE_OF + ROWTYPE_GE;
pub const ROWTYPE_OFMIN: ::core::ffi::c_int = ROWTYPE_OF + ROWTYPE_LE;
pub const ROWTYPE_CHSIGN: ::core::ffi::c_int = ROWTYPE_GE;
pub const FR: ::core::ffi::c_int = ROWTYPE_EMPTY;
pub const LE: ::core::ffi::c_int = ROWTYPE_LE;
pub const GE: ::core::ffi::c_int = ROWTYPE_GE;
pub const EQ: ::core::ffi::c_int = ROWTYPE_EQ;
pub const ROWCLASS_Unknown: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ROWCLASS_Objective: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ROWCLASS_GeneralREAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ROWCLASS_GeneralMIP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ROWCLASS_GeneralINT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ROWCLASS_GeneralBIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ROWCLASS_KnapsackINT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ROWCLASS_KnapsackBIN: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const ROWCLASS_SetPacking: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ROWCLASS_SetCover: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const ROWCLASS_GUB: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SCAN_USERVARS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SCAN_SLACKVARS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SCAN_ARTIFICIALVARS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const USE_BASICVARS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const USE_NONBASICVARS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SCAN_ALLVARS: ::core::ffi::c_int = SCAN_SLACKVARS + SCAN_USERVARS + SCAN_ARTIFICIALVARS;
pub const USE_ALLVARS: ::core::ffi::c_int = USE_BASICVARS + USE_NONBASICVARS;
pub const IMPROVE_DUALFEAS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IMPROVE_THETAGAP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const IMPROVE_DEFAULT: ::core::ffi::c_int = IMPROVE_DUALFEAS + IMPROVE_THETAGAP;
pub const SCALE_GEOMETRIC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCALE_LINEAR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SCALE_QUADRATIC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SCALE_MAXTYPE: ::core::ffi::c_int = SCALE_QUADRATIC - 1 as ::core::ffi::c_int;
pub const SCALE_EQUILIBRATE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SCALE_INTEGERS: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const ITERATE_MAJORMAJOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ITERATE_MINORMAJOR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ITERATE_MINORRETRY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PRICER_DEVEX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PRICER_STEEPESTEDGE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PRICE_PRIMALFALLBACK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PRICE_MULTIPLE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PRICE_PARTIAL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PRICE_ADAPTIVE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PRICE_HYBRID: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const PRICE_RANDOMIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PRICE_AUTOPARTIAL: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const PRICE_AUTOMULTIPLE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const PRICE_LOOPLEFT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const PRICE_LOOPALTERNATE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const PRICE_HARRISTWOPASS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const PRICE_FORCEFULL: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const PRICE_TRUENORMINIT: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const PRICE_STRATEGYMASK: ::core::ffi::c_int = PRICE_PRIMALFALLBACK
    + PRICE_MULTIPLE
    + PRICE_PARTIAL
    + PRICE_ADAPTIVE
    + PRICE_HYBRID
    + PRICE_RANDOMIZE
    + PRICE_AUTOPARTIAL
    + PRICE_AUTOMULTIPLE
    + PRICE_LOOPLEFT
    + PRICE_LOOPALTERNATE
    + PRICE_HARRISTWOPASS
    + PRICE_FORCEFULL
    + PRICE_TRUENORMINIT;
pub const BB_INT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BB_SC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NODE_FIRSTSELECT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NODE_GAPSELECT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NODE_RANGESELECT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NODE_FRACTIONSELECT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NODE_PSEUDOCOSTSELECT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NODE_PSEUDONONINTSELECT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NODE_PSEUDORATIOSELECT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const NODE_STRATEGYMASK: ::core::ffi::c_int = NODE_WEIGHTREVERSEMODE - 1 as ::core::ffi::c_int;
pub const NODE_WEIGHTREVERSEMODE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NODE_GREEDYMODE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NODE_PSEUDOCOSTMODE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NODE_DEPTHFIRSTMODE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const NODE_RANDOMIZEMODE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NODE_GUBMODE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const NODE_DYNAMICMODE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NODE_RESTARTMODE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const NODE_BREADTHFIRSTMODE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const NODE_AUTOORDER: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const NODE_RCOSTFIXING: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const NODE_STRONGINIT: ::core::ffi::c_int = 32768 as ::core::ffi::c_int;
pub const BRANCH_AUTOMATIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BRANCH_DEFAULT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ACTION_REBASE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ACTION_RECOMPUTE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ACTION_REINVERT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ACTION_RESTART: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const DATAIGNORED: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const NOBFP: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
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
pub const PROCBREAK: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FEASFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const NOFEASFOUND: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FATHOMED: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const OF_RELAXED: ::core::ffi::c_int = 0;
pub const OF_INCUMBENT: ::core::ffi::c_int = 1;
pub const OF_WORKING: ::core::ffi::c_int = 2;
pub const OF_USERBREAK: ::core::ffi::c_int = 3;
pub const OF_HEURISTIC: ::core::ffi::c_int = 4;
pub const OF_DUALLIMIT: ::core::ffi::c_int = 5;
pub const OF_DELTA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OF_PROJECTED: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OF_TEST_NE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OF_TEST_RELGAP: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DEF_PARTIALBLOCKS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const DEF_MAXPIVOTRETRY: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const DEF_SCALINGLIMIT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DEF_NEGRANGE: ::core::ffi::c_double = -1.0e+06f64;
pub const DEF_BB_LIMITLEVEL: ::core::ffi::c_int = -(50 as ::core::ffi::c_int);
pub const MAX_FRACSCALE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const RANDSCALE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const DOUBLEROUND: ::core::ffi::c_double = 0.0e-02f64;
pub const DEF_EPSMACHINE: ::core::ffi::c_double = 2.22e-16f64;
pub const EPS_TIGHT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EPS_MEDIUM: ::core::ffi::c_int = 1;
pub const EPS_LOOSE: ::core::ffi::c_int = 2;
pub const EPS_BAGGY: ::core::ffi::c_int = 3;
pub const EPS_DEFAULT: ::core::ffi::c_int = EPS_TIGHT;
pub const DEF_INFINITE: ::core::ffi::c_double = 1.0e+30f64;
pub const DEF_EPSVALUE: ::core::ffi::c_double = 1.0e-12f64;
pub const DEF_EPSPRIMAL: ::core::ffi::c_double = 1.0e-10f64;
pub const DEF_EPSDUAL: ::core::ffi::c_double = 1.0e-09f64;
pub const DEF_EPSPIVOT: ::core::ffi::c_double = 2.0e-07f64;
pub const DEF_PERTURB: ::core::ffi::c_double = 1.0e-05f64;
pub const DEF_EPSSOLUTION: ::core::ffi::c_double = 1.0e-05f64;
pub const DEF_EPSINT: ::core::ffi::c_double = 1.0e-07f64;
pub const DEF_MIP_GAP: ::core::ffi::c_double = 1.0e-11f64;
pub const DEF_LAGACCEPT: ::core::ffi::c_double = 1.0e-03f64;
pub const DEF_PSEUDOCOSTUPDATES: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const DEF_PSEUDOCOSTRESTART: ::core::ffi::c_double = 0.15f64;
pub const DEF_MAXPRESOLVELOOPS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAT_ROUNDREL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAT_ROUNDRC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAT_ROUNDDEFAULT: ::core::ffi::c_int = MAT_ROUNDREL;
pub const matRowColStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const matValueStep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOS_INCOMPLETE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SOS_COMPLETE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
