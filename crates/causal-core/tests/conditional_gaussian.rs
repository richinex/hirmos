use hirmos_causal_core::conditional_gaussian::{Model, Variable, Error};
use nalgebra::DMatrix;

#[test]
fn pinned_bnlearn_local_fits_and_queries() {
    let f:serde_json::Value=serde_json::from_str(include_str!("../oracle/fixtures/conditional_gaussian_bnlearn.json")).unwrap();
    for c in f["cases"].as_array().unwrap(){
        let n=c["rows"].as_u64().unwrap() as usize;
        let kinds=c["kinds"].as_array().unwrap().iter().map(|v|if v=="discrete"{Variable::Discrete}else{Variable::Continuous}).collect::<Vec<_>>();
        let edges:Vec<(usize,usize)>=serde_json::from_value(c["edges"].clone()).unwrap();
        let values:Vec<f64>=serde_json::from_value(c["values"].clone()).unwrap();
        let m=Model::fit(&DMatrix::from_column_slice(n,kinds.len(),&values),&kinds,&edges).unwrap();
        let close=|x:f64,y:f64|assert!((x-y).abs()<1e-9,"{}: {x} != {y}",c["name"]);
        for l in c["locals"].as_array().unwrap(){
            let local=m.locals.iter().find(|v|v.node==l["node"].as_u64().unwrap() as usize).unwrap();
            assert_eq!(local.configurations.len(),l["configurations"].as_array().unwrap().len());
            for (actual,expected) in local.configurations.iter().zip(l["configurations"].as_array().unwrap()){
                assert_eq!(actual.rows,expected["n"].as_u64().unwrap() as usize);
                let states:Vec<f64>=serde_json::from_value(expected["states"].clone()).unwrap();assert_eq!(actual.states,states);
                for (a,b) in actual.coefficients.iter().zip(expected["beta"].as_array().unwrap()){close(*a,b.as_f64().unwrap())}close(actual.std,expected["std"].as_f64().unwrap());
            }
        }
        for q in c["queries"].as_array().unwrap(){
            let obs:Vec<(usize,f64)>=serde_json::from_value(q["observations"].clone()).unwrap();
            let actions:Vec<(usize,f64)>=serde_json::from_value(q["interventions"].clone()).unwrap();
            let r=m.query(&[q["outcome"].as_u64().unwrap() as usize],&obs,&actions).unwrap();
            close(r[0].mean,q["mean"].as_f64().unwrap());close(r[0].std,q["std"].as_f64().unwrap());
        }
    }
}

#[test]
fn refuses_unsupported_or_ambiguous_queries() {
    let data=DMatrix::from_row_slice(4,4,&[0.,0.,1.,2., 0.,0.,2.,4., 1.,1.,3.,5., 1.,1.,4.,7.]);
    let kinds=[Variable::Discrete,Variable::Discrete,Variable::Continuous,Variable::Continuous];
    let m=Model::fit(&data,&kinds,&[(0,3),(1,3),(2,3)]).unwrap();
    assert!(matches!(m.query(&[3],&[(0,0.),(1,1.),(2,2.)],&[]),Err(Error::UnseenConfiguration{..})));
    assert!(matches!(m.query(&[3],&[(0,0.),(1,0.)],&[]),Err(Error::MissingParent{..})));
    assert!(matches!(m.query(&[2],&[(0,0.)],&[]),Err(Error::UnsupportedEvidence{..})));
    assert!(matches!(m.query(&[3],&[(0,2.),(1,0.),(2,2.)],&[]),Err(Error::InvalidState{..})));
    assert!(m.query(&[3],&[(0,0.)],&[(0,0.)]).is_err());
    assert!(m.query(&[0],&[],&[]).is_err());
    assert!(Model::fit(&data,&kinds,&[(2,0)]).is_err());
    assert!(Model::fit(&data,&kinds,&[(2,3),(3,2)]).is_err());
}

#[test]
fn signed_zero_categories_and_nonfinite_inputs() {
    let data=DMatrix::from_row_slice(3,2,&[-0.,2.,0.,3.,1.,4.]);
    let kinds=[Variable::Discrete,Variable::Continuous];
    let model=Model::fit(&data,&kinds,&[(0,1)]).unwrap();
    let result=model.query(&[1],&[(0,0.)],&[]).unwrap();
    assert_eq!(result[0].rows,2);assert_eq!(result[0].mean,2.5);
    assert!(model.query(&[1],&[(0,f64::NAN)],&[]).is_err());
    assert!(Model::fit(&DMatrix::from_row_slice(1,2,&[0.,f64::INFINITY]),&kinds,&[(0,1)]).is_err());
}
