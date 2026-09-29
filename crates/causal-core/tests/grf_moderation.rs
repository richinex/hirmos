use hirmos_causal_core::grf::moderation::{self, Contrast};
use serde_json::Value;
fn close(a:f64,b:&Value) {assert!((a-b.as_f64().unwrap()).abs()<1e-11,"{a} != {b}");}
fn check(a:Contrast,b:&Value) {
    for (key,value) in [("estimate",a.estimate),("se",a.standard_error),("statistic",a.statistic),("df",a.df),("p",a.p_value),("lower",a.lower),("upper",a.upper)] {close(value,&b[key]);}
}
#[test]
fn cluster_comparisons_match_r_stats() {
    let v:Value=serde_json::from_str(include_str!("../oracle/grf/fixtures/moderation.json")).unwrap();
    let array=|name:&str|v[name].as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect::<Vec<_>>();
    let scores=array("scores"); let labels=array("labels").into_iter().map(|v|v as usize).collect::<Vec<_>>();
    let result=moderation::between(&scores,&labels,&array("between"),0.95).unwrap();
    check(result.contrast.unwrap(),&v["welch"]);
    let a=result.omnibus.unwrap(); close(a.statistic,&v["anova"]["statistic"]); close(a.p_value,&v["anova"]["p"]);
    assert_eq!(a.df,2); assert_eq!(a.residual_df,9);
    check(moderation::within(&scores,&labels,&array("within"),2.,0.95).unwrap(),&v["paired"]);
}
#[test]
fn unavailable_groups_are_not_silently_dropped() {
    assert!(moderation::welch(&[1.],&[2.,3.],0.95).is_err());
    assert!(moderation::paired(&[1.,1.,1.],0.95).is_err());
    assert!(moderation::within(&[1.,2.,3.,4.],&[0,0,1,1],&[0.,1.,0.,0.],1.,0.95).is_err());
    assert!(moderation::between(&[1.,2.,3.,4.],&[0,1,2,3],&[1.,1.,1.,1.],0.95).unwrap().omnibus.is_err());
    assert!(moderation::paired(&[1.,2.,3.],0.).is_err());
}
