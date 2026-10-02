#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_mut,
    unused_assignments, unused_unsafe, unused_variables, dead_code, unused_parens,
    unused_must_use, clashing_extern_declarations)]
//! OSQP 1.0.0 shipped by the pinned R oracle. Generated solver and QDLDL
//! calculations retain their source control flow; see sources.json.
#[path="../../honest-did-ecos-port/src/runtime.rs"] mod runtime;
mod generated;
use generated::osqp_api as api;

#[derive(Clone, Debug)]
pub struct Solution {
    pub x: Vec<f64>, pub objective: f64, pub status: i32, pub iterations: i32,
    pub primal_residual: f64, pub dual_residual: f64,
}

/// Solve a dense convex QP with the CVXR 1.9.2 default OSQP settings.
/// P is the full symmetric Hessian in 1/2 x'Px + q'x; constraints are l <= Ax <= u.
pub fn solve(p: &[Vec<f64>], q: &[f64], a: &[Vec<f64>], lower: &[f64], upper: &[f64]) -> Result<Solution, &'static str> {
    solve_with(p,q,a,lower,upper,Settings::Cvxr)
}

/// Source-specific settings, not one universal uncertainty/optimization preset.
#[derive(Clone,Copy,Debug)]
pub enum Settings { Cvxr, Augsynth }
pub fn solve_with(p: &[Vec<f64>], q: &[f64], a: &[Vec<f64>], lower: &[f64], upper: &[f64], source:Settings) -> Result<Solution, &'static str> {
    let n=q.len(); let m=a.len();
    if n==0 || n>i32::MAX as usize || m>i32::MAX as usize || p.len()!=n
        || p.iter().any(|r|r.len()!=n || r.iter().any(|v|!v.is_finite()))
        || a.iter().any(|r|r.len()!=n || r.iter().any(|v|!v.is_finite()))
        || lower.len()!=m || upper.len()!=m || q.iter().any(|v|!v.is_finite())
        || lower.iter().zip(upper).any(|(l,u)|l.is_nan() || u.is_nan() || l>u) {
        return Err("Invalid quadratic problem");
    }
    let csc=|rows:&[Vec<f64>],triangle:bool| {
        let mut offsets=vec![0i32];let mut indices=Vec::new();let mut values=Vec::new();
        for j in 0..n {for (i,row) in rows.iter().enumerate(){
            if (!triangle || i<=j) && row[j]!=0.0 {indices.push(i as i32);values.push(row[j]);}
        } offsets.push(values.len() as i32);}
        (offsets,indices,values)
    };
    let (mut pp,mut pi,mut px)=csc(p,true);let (mut ap,mut ai,mut ax)=csc(a,false);
    let matrix=|rows,offsets:&mut Vec<i32>,indices:&mut Vec<i32>,values:&mut Vec<f64>|api::OSQPCscMatrix {
        m:rows as i32,n:n as i32,p:offsets.as_mut_ptr(),i:indices.as_mut_ptr(),x:values.as_mut_ptr(),
        nzmax:values.len() as i32,nz:-1,owned:0,
    };
    let pm=matrix(n,&mut pp,&mut pi,&mut px);let am=matrix(m,&mut ap,&mut ai,&mut ax);
    let bounds=|v:&[f64]| v.iter().map(|x|x.clamp(-1e30,1e30)).collect::<Vec<_>>();
    let lower=bounds(lower);let upper=bounds(upper);
    unsafe {
        let mut settings:api::OSQPSettings=core::mem::zeroed();
        api::osqp_set_default_settings(&mut settings);
        settings.verbose=0;settings.adaptive_rho_interval=50;
        match source {
            Settings::Cvxr=>{settings.eps_abs=1e-5;settings.eps_rel=1e-5;settings.max_iter=10000;settings.polishing=1;},
            Settings::Augsynth=>{settings.eps_abs=1e-8;settings.eps_rel=1e-8;settings.max_iter=4000;settings.polishing=0;settings.check_dualgap=1;settings.time_limit=1e10;},
        }
        let mut solver=core::ptr::null_mut();
        let setup=api::osqp_setup(&mut solver,&pm,q.as_ptr(),&am,lower.as_ptr(),upper.as_ptr(),m as i32,n as i32,&settings);
        if setup!=0 || solver.is_null(){if !solver.is_null(){api::osqp_cleanup(solver);}return Err("OSQP setup failed");}
        let exit=api::osqp_solve(solver);
        if exit!=0{api::osqp_cleanup(solver);return Err("OSQP execution failed");}
        let info=&*(*solver).info;let solution=&*(*solver).solution;
        let result=Solution{x:core::slice::from_raw_parts(solution.x,n).to_vec(),objective:info.obj_val,
            status:info.status_val,iterations:info.iter,primal_residual:info.prim_res,dual_residual:info.dual_res};
        api::osqp_cleanup(solver);
        Ok(result)
    }
}
