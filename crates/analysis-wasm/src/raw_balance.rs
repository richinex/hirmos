use crate::protocol::AnalysisResult;
use serde::Serialize;
use hirmos_causal_core::covariate_balance::{balance, CovariateKind, Denominator, Scale};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Row { pub column: usize, pub difference: f64, pub denominator: f64, pub used_full_sample_spread: bool }

pub fn calculate(values: &[f64], rows: usize, columns: usize, treated_reference: bool) -> Result<AnalysisResult,String> {
    if rows == 0 || columns < 2 || rows.checked_mul(columns) != Some(values.len()) || values.iter().any(|v| !v.is_finite()) {
        return Err("Balance requires a finite treatment column and at least one covariate.".into());
    }
    if values[..rows].iter().any(|&v| v != 0.0 && v != 1.0) { return Err("Raw balance requires treatment values of 0 and 1.".into()); }
    let treated: Vec<_> = values[..rows].iter().map(|&v| v == 1.0).collect();
    let reference = if treated_reference { Denominator::Treated } else { Denominator::Pooled };
    let mut result = Vec::new();
    for column in 1..columns {
        let x=&values[column*rows..(column+1)*rows];
        let kind=if x.iter().all(|&v| v == 0.0 || v == 1.0) { CovariateKind::Binary } else { CovariateKind::Continuous };
        let r=balance(x,&treated,None,kind,Scale::Standardized(reference)).map_err(|_| "Balance could not be calculated. Both treatment groups must be present and the covariate values must permit finite calculations.".to_owned())?;
        result.push(Row {column:column-1,difference:r.before,denominator:r.denominator,used_full_sample_spread:r.used_full_sample_spread});
    }
    Ok(AnalysisResult::RawBalance { rows:result })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_balance_uses_only_treatment_and_covariates() {
        let values=[0.,0.,1.,1., 1.,3.,5.,7., 0.,1.,1.,1.];
        let AnalysisResult::RawBalance {rows}=calculate(&values,4,3,false).unwrap() else {panic!("wrong result")};
        assert_eq!(rows.len(),2);
        assert!((rows[0].difference-4.0/2.0_f64.sqrt()).abs()<1e-12);
        assert!((rows[1].difference-0.5/(0.125_f64.sqrt())).abs()<1e-12);
    }
    #[test]
    fn invalid_treatment_and_matrix_are_refused() {
        assert!(calculate(&[0.,2.,1.,3.],2,2,false).is_err());
        assert!(calculate(&[0.,1.,2.],2,2,false).is_err());
        assert!(calculate(&[0.,0.,1.,3.],2,2,false).is_err());
    }
}
