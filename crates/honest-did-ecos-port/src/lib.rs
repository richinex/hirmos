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

pub fn solve(cost: &[f64], g: &Matrix, h: &[f64], linear: usize, cones: &[usize], a: &Matrix, b: &[f64]) -> Result<Solution,&'static str> {
    let n=cost.len();
    if n==0 || !g.valid() || !a.valid() || g.columns!=n || a.columns!=n || h.len()!=g.rows || b.len()!=a.rows
        || cones.iter().try_fold(linear,|sum,q|sum.checked_add(*q))!=Some(g.rows)
        || cones.iter().any(|q| *q<2) || cost.iter().chain(h).chain(b).any(|v| !v.is_finite()) {
        return Err("Invalid conic problem");
    }
    let mut g=g.clone(); let mut a=a.clone();
    let mut cost=cost.to_vec(); let mut h=h.to_vec(); let mut b=b.to_vec();
    let mut cones:Vec<i64>=cones.iter().map(|q| *q as i64).collect();
    unsafe {
        let w=preproc::ECOS_setup(n as i64,g.rows as i64,a.rows as i64,linear as i64,
            cones.len() as i64,cones.as_mut_ptr(),0,g.values.as_mut_ptr(),g.offsets.as_mut_ptr(),g.indices.as_mut_ptr(),
            if a.rows==0 {core::ptr::null_mut()} else {a.values.as_mut_ptr()},
            if a.rows==0 {core::ptr::null_mut()} else {a.offsets.as_mut_ptr()},
            if a.rows==0 {core::ptr::null_mut()} else {a.indices.as_mut_ptr()},
            cost.as_mut_ptr(),h.as_mut_ptr(),b.as_mut_ptr());
        if w.is_null() { return Err("ECOS setup failed"); }
        (*(*w).stgs).verbose=0;
        // C-compatible layouts are identical in the two generated modules.
        let status=ecos::ECOS_solve(w.cast());
        let info=&*(*w).info;
        let result=Solution { x:core::slice::from_raw_parts((*w).x,n).to_vec(),
            dual_inequalities: core::slice::from_raw_parts((*w).z,g.rows).to_vec(),
            objective:info.pcost,status,iterations:info.iter,
            primal_residual:info.pres,dual_residual:info.dres,gap:info.gap };
        preproc::ECOS_cleanup(w,0);
        Ok(result)
    }
}
