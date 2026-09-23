use nalgebra::DMatrix;
use serde_json::Value;
use hirmos_causal_core::staggered_did::bootstrap::run;

#[test]
fn seeded_draws_and_bands_match_r() {
    let cases:Value=serde_json::from_str(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"),"/fixtures/staggered-did/bootstrap.json")).unwrap()).unwrap();
    for (_,c) in cases.as_object().unwrap() {
        let n=c["influence"].as_array().unwrap().len();
        let input=DMatrix::from_fn(n,3,|i,j| c["influence"][i][j].as_f64().unwrap());
        let clusters:Vec<_>=c["clusters"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
        let output=run(&input,&clusters,199,0.05,731).unwrap();
        for i in 0..199 {for j in 0..2 {assert!((output.draws[(i,j)]-c["draws"][i][j].as_f64().unwrap()).abs()<1e-12);}}
        for j in 0..2 {assert!((output.se[j].unwrap()-c["se"][j].as_f64().unwrap()).abs()<1e-12);}
        assert_eq!(output.se[2],None);
        assert!((output.critical.unwrap()-c["critical"].as_f64().unwrap()).abs()<1e-12);
    }
}
