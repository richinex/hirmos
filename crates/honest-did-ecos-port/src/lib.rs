#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_mut,
    unused_assignments, unused_unsafe, unused_variables, dead_code, unused_parens)]
//! Mechanical Rust translation of ECOS 2.0.7 shipped by ECOSolveR 0.6.1.
//! GPL-3.0-or-later; original source hashes and adaptations: sources.json.
mod runtime;
#[path="src/ecos.rs"] mod ecos;
#[path="src/kkt.rs"] mod kkt;
#[path="src/cone.rs"] mod cone;
#[path="src/spla.rs"] mod spla;
#[path="src/preproc.rs"] mod preproc;
#[path="src/splamm.rs"] mod splamm;
#[path="src/equil.rs"] mod equil;
#[path="external/ldl/src/ldl.rs"] mod ldl;
#[path="external/amd/src/amd_1.rs"] mod amd_1;
#[path="external/amd/src/amd_2.rs"] mod amd_2;
#[path="external/amd/src/amd_aat.rs"] mod amd_aat;
#[path="external/amd/src/amd_control.rs"] mod amd_control;
#[path="external/amd/src/amd_defaults.rs"] mod amd_defaults;
#[path="external/amd/src/amd_dump.rs"] mod amd_dump;
#[path="external/amd/src/amd_global.rs"] mod amd_global;
#[path="external/amd/src/amd_info.rs"] mod amd_info;
#[path="external/amd/src/amd_order.rs"] mod amd_order;
#[path="external/amd/src/amd_post_tree.rs"] mod amd_post_tree;
#[path="external/amd/src/amd_postorder.rs"] mod amd_postorder;
#[path="external/amd/src/amd_preprocess.rs"] mod amd_preprocess;
#[path="external/amd/src/amd_valid.rs"] mod amd_valid;

#[no_mangle] static R_PosInf: f64 = f64::INFINITY;
#[no_mangle] static R_NaN: f64 = f64::NAN;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub offsets: Vec<i64>,
    pub indices: Vec<i64>,
    pub values: Vec<f64>,
}
impl Matrix {
    fn valid(&self) -> bool {
        self.rows <= i64::MAX as usize && self.columns <= i64::MAX as usize
            && self.offsets.len() == self.columns + 1 && self.offsets.first() == Some(&0)
            && self.offsets.last() == Some(&(self.values.len() as i64))
            && self.indices.len() == self.values.len()
            && self.values.iter().all(|v| v.is_finite())
            && self.offsets.windows(2).all(|w| w[0] >= 0 && w[0] <= w[1] && w[1] <= self.values.len() as i64)
            && self.indices.iter().all(|i| *i >= 0 && *i < self.rows as i64)
            && self.offsets.windows(2).all(|w| self.indices[w[0] as usize..w[1] as usize].windows(2).all(|i|i[0]<i[1]))
    }
    pub fn from_rows(rows: &[Vec<f64>], columns: usize) -> Result<Self, &'static str> {
        if rows.iter().any(|r| r.len()!=columns || r.iter().any(|v| !v.is_finite())) {
            return Err("Invalid conic matrix dimensions or values");
        }
        let mut offsets=vec![0]; let mut indices=Vec::new(); let mut values=Vec::new();
        for j in 0..columns {
            for (i,row) in rows.iter().enumerate() {
                if row[j]!=0.0 { indices.push(i as i64); values.push(row[j]); }
            }
            offsets.push(values.len() as i64);
        }
        Ok(Self {rows:rows.len(),columns,offsets,indices,values})
    }
}
#[derive(Clone, Debug)]
pub struct Solution {
    pub x: Vec<f64>, pub objective: f64, pub status: i64,
    pub dual_inequalities: Vec<f64>,
    pub iterations: i64, pub primal_residual: f64, pub dual_residual: f64,
    pub gap: f64,
}

fn valid_problem(cost: &[f64], g: &Matrix, h: &[f64], linear: usize, cones: &[usize], a: &Matrix, b: &[f64]) -> bool {
    let n=cost.len();
    n>0 && g.valid() && a.valid() && g.columns==n && a.columns==n && h.len()==g.rows && b.len()==a.rows
        && cones.iter().try_fold(linear,|sum,q|sum.checked_add(*q))==Some(g.rows)
        && !cones.iter().any(|q| *q<2) && cost.iter().chain(h).chain(b).all(|v| v.is_finite())
}

// Vectors own stable heap allocations for every pointer retained by ECOS.
// The raw workspace is private, not Send/Sync, and freed before its inputs.
struct Workspace {
    w: *mut preproc::pwork,
    g: Matrix,
    a: Matrix,
    cost: Vec<f64>,
    h: Vec<f64>,
    b: Vec<f64>,
    cones: Vec<i64>,
}

impl Workspace {
    fn new(cost: &[f64], g: &Matrix, h: &[f64], linear: usize, cones: &[usize], a: &Matrix, b: &[f64]) -> Result<Self,&'static str> {
        if !valid_problem(cost,g,h,linear,cones,a,b) { return Err("Invalid conic problem"); }
        let mut owned=Self { w:core::ptr::null_mut(),g:g.clone(),a:a.clone(),cost:cost.to_vec(),h:h.to_vec(),b:b.to_vec(),cones:cones.iter().map(|q| *q as i64).collect() };
        unsafe {
            owned.w=preproc::ECOS_setup(cost.len() as i64,g.rows as i64,a.rows as i64,linear as i64,
                owned.cones.len() as i64,owned.cones.as_mut_ptr(),0,owned.g.values.as_mut_ptr(),owned.g.offsets.as_mut_ptr(),owned.g.indices.as_mut_ptr(),
                if a.rows==0 {core::ptr::null_mut()} else {owned.a.values.as_mut_ptr()},
                if a.rows==0 {core::ptr::null_mut()} else {owned.a.offsets.as_mut_ptr()},
                if a.rows==0 {core::ptr::null_mut()} else {owned.a.indices.as_mut_ptr()},
                owned.cost.as_mut_ptr(),owned.h.as_mut_ptr(),owned.b.as_mut_ptr());
            if owned.w.is_null() { return Err("ECOS setup failed"); }
            (*(*owned.w).stgs).verbose=0;
        }
        Ok(owned)
    }

    fn run(&mut self) -> Solution {
        let w=self.w;
        unsafe {
        // C-compatible layouts are identical in the two generated modules.
        // ECOS_solve invokes init: no primal/dual iterate is warm-started.
        let status=ecos::ECOS_solve(w.cast());
        let info=&*(*w).info;
        Solution { x:core::slice::from_raw_parts((*w).x,self.cost.len()).to_vec(),
            dual_inequalities: core::slice::from_raw_parts((*w).z,self.g.rows).to_vec(),
            objective:info.pcost,status,iterations:info.iter,
            primal_residual:info.pres,dual_residual:info.dres,gap:info.gap }
        }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        if !self.w.is_null() { unsafe { preproc::ECOS_cleanup(self.w,0); } }
    }
}

/// A fixed linear constraint system with changing objectives. Symbolic ordering,
/// allocation and equilibration are reused; numerical factorization and the
/// reference cold-start initialization are performed on every solve.
pub struct LinearObjectiveWorkspace(Workspace);

impl LinearObjectiveWorkspace {
    pub fn new(g: &Matrix, h: &[f64], a: &Matrix, b: &[f64]) -> Result<Self,&'static str> {
        if !g.valid() || !a.valid() || g.columns!=a.columns { return Err("Invalid conic problem"); }
        let cost=vec![0.;g.columns];
        Workspace::new(&cost,g,h,g.rows,&[],a,b).map(Self)
    }

    pub fn solve(&mut self, cost: &[f64]) -> Result<Solution,&'static str> {
        if cost.len()!=self.0.cost.len() || cost.iter().any(|v| !v.is_finite()) { return Err("Invalid conic objective"); }
        // ECOS backscales its objective on exit. Replace it on its original scale
        // before the next cold start, without re-equilibrating fixed constraints.
        self.0.cost.copy_from_slice(cost);
        Ok(self.0.run())
    }
}

pub fn solve(cost: &[f64], g: &Matrix, h: &[f64], linear: usize, cones: &[usize], a: &Matrix, b: &[f64]) -> Result<Solution,&'static str> {
    Ok(Workspace::new(cost,g,h,linear,cones,a,b)?.run())
}
