#[repr(C)] pub struct OSQPVectorf_ { _opaque: [u8; 0] }
#[repr(C)] pub struct OSQPMatrix_ { _opaque: [u8; 0] }
extern "C" {
    #[link_name = "honest_osqp_OSQPVectorf_copy"]
    fn OSQPVectorf_copy(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_set_scalar"]
    fn OSQPVectorf_set_scalar(a: *mut OSQPVectorf, sc: OSQPFloat);
    #[link_name = "honest_osqp_OSQPVectorf_plus"]
    fn OSQPVectorf_plus(x: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_prod"]
    fn OSQPVectorf_ew_prod(c: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_ew_reciprocal"]
    fn OSQPVectorf_ew_reciprocal(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPMatrix_AtDA_extract_diag"]
    fn OSQPMatrix_AtDA_extract_diag(
        A: *const OSQPMatrix,
        D: *const OSQPVectorf,
        d: *mut OSQPVectorf,
    );
    #[link_name = "honest_osqp_OSQPMatrix_extract_diag"]
    fn OSQPMatrix_extract_diag(A: *const OSQPMatrix, d: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPMatrix_Axpy"]
    fn OSQPMatrix_Axpy(
        A: *const OSQPMatrix,
        x: *const OSQPVectorf,
        y: *mut OSQPVectorf,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPMatrix_Atxpy"]
    fn OSQPMatrix_Atxpy(
        A: *const OSQPMatrix,
        x: *const OSQPVectorf,
        y: *mut OSQPVectorf,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
}
pub type OSQPFloat = ::core::ffi::c_double;
pub type OSQPVectorf = OSQPVectorf_;
pub type OSQPMatrix = OSQPMatrix_;
#[export_name = "honest_osqp_reduced_kkt_mv_times"]
pub unsafe extern "C" fn reduced_kkt_mv_times(
    mut P: *const OSQPMatrix,
    mut A: *const OSQPMatrix,
    mut rho_vec: *const OSQPVectorf,
    mut sigma: OSQPFloat,
    mut x: *const OSQPVectorf,
    mut v: *mut OSQPVectorf,
    mut work: *mut OSQPVectorf,
) {
    OSQPMatrix_Axpy(A, x, work, 1.0f64, 0.0f64);
    OSQPVectorf_ew_prod(work, work, rho_vec);
    OSQPVectorf_copy(v, x);
    OSQPMatrix_Axpy(P, x, v, 1.0f64, sigma);
    OSQPMatrix_Atxpy(A, work, v, 1.0f64, 1.0f64);
}
#[export_name = "honest_osqp_reduced_kkt_diagonal"]
pub unsafe extern "C" fn reduced_kkt_diagonal(
    mut P: *const OSQPMatrix,
    mut A: *const OSQPMatrix,
    mut rho_vec: *const OSQPVectorf,
    mut sigma: OSQPFloat,
    mut diag: *mut OSQPVectorf,
    mut diag_inv: *mut OSQPVectorf,
) {
    OSQPVectorf_set_scalar(diag, sigma);
    OSQPMatrix_extract_diag(P, diag_inv);
    OSQPVectorf_plus(diag, diag, diag_inv);
    OSQPMatrix_AtDA_extract_diag(A, rho_vec, diag_inv);
    OSQPVectorf_plus(diag, diag, diag_inv);
    OSQPVectorf_ew_reciprocal(diag_inv, diag);
}
#[export_name = "honest_osqp_reduced_kkt_compute_rhs"]
pub unsafe extern "C" fn reduced_kkt_compute_rhs(
    mut A: *const OSQPMatrix,
    mut rho_vec: *const OSQPVectorf,
    mut b1: *mut OSQPVectorf,
    mut b2: *const OSQPVectorf,
    mut work: *mut OSQPVectorf,
) {
    OSQPVectorf_ew_prod(work, b2, rho_vec);
    OSQPMatrix_Atxpy(A, work, b1, 1.0f64, 1.0f64);
}
