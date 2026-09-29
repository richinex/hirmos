//! ACIC18 score comparisons: R stats::t.test and one-way aov summaries.
//! Equal cluster means are the observations, not individual student scores.
use std::collections::BTreeMap;

pub struct Contrast { pub estimate: f64, pub standard_error: f64, pub statistic: f64, pub df: f64, pub p_value: f64, pub lower: f64, pub upper: f64 }
pub struct Omnibus { pub statistic: f64, pub df: usize, pub residual_df: usize, pub p_value: f64 }
pub struct Between { pub means: Vec<f64>, pub scores: Vec<f64>, pub median: f64, pub cutpoints: [f64;2], pub contrast: Result<Contrast, &'static str>, pub omnibus: Result<Omnibus, &'static str> }

fn moments(x: &[f64]) -> Result<(f64,f64), &'static str> {
    if x.len()<2 || x.iter().any(|v| !v.is_finite()) { return Err("At least two finite cluster values are required per comparison group."); }
    let mean = x.iter().sum::<f64>() / x.len() as f64;
    let variance = x.iter().map(|v| (v-mean).powi(2)).sum::<f64>() / (x.len()-1) as f64;
    Ok((mean, variance))
}
fn contrast(estimate: f64, se: f64, df: f64, level: f64) -> Result<Contrast, &'static str> {
    if se <= 0. || !se.is_finite() || df <= 0. || !df.is_finite() || level <= 0. || level >= 1. || !level.is_finite() {
        return Err("The cluster comparison has no finite sampling uncertainty.");
    }
    let statistic = estimate/se;
    let p_value = spec_math::cephes64::incbet(df/2., 0.5, df/(df+statistic*statistic));
    let beta = spec_math::cephes64::incbi(df/2., 0.5, 1.-level);
    let critical = (df*(1./beta-1.)).sqrt();
    Ok(Contrast { estimate, standard_error: se, statistic, df, p_value, lower: estimate-critical*se, upper: estimate+critical*se })
}
pub fn welch(high: &[f64], low: &[f64], level: f64) -> Result<Contrast, &'static str> {
    let (mh,vh)=moments(high)?; let (ml,vl)=moments(low)?;
    let a=vh/high.len() as f64; let b=vl/low.len() as f64;
    let df=(a+b).powi(2)/(a*a/(high.len()-1) as f64+b*b/(low.len()-1) as f64);
    contrast(mh-ml,(a+b).sqrt(),df,level)
}
pub fn paired(differences: &[f64], level: f64) -> Result<Contrast, &'static str> {
    let (mean,variance)=moments(differences)?;
    contrast(mean,(variance/differences.len() as f64).sqrt(),(differences.len()-1) as f64,level)
}
fn groups(scores: &[f64], labels: &[usize], covariate: &[f64]) -> Result<Vec<Vec<usize>>, &'static str> {
    if scores.len()!=labels.len() || scores.len()!=covariate.len() || scores.is_empty()
        || scores.iter().chain(covariate).any(|v| !v.is_finite()) { return Err("Cluster scores and covariates must have matching finite observations."); }
    let mut groups=BTreeMap::<usize,Vec<usize>>::new();
    for (i,&label) in labels.iter().enumerate() { groups.entry(label).or_default().push(i); }
    Ok(groups.into_values().collect())
}
fn quantile(sorted: &[f64], p:f64)->f64 {
    let h=(sorted.len()-1) as f64*p; let i=h.floor() as usize; let f=h-i as f64;
    sorted[i]*(1.-f)+sorted[(i+1).min(sorted.len()-1)]*f
}
pub fn between(scores: &[f64], labels: &[usize], covariate: &[f64], level:f64)->Result<Between,&'static str> {
    let groups=groups(scores,labels,covariate)?;
    if groups.len()<3 { return Err("At least three independent clusters are required."); }
    let means:Vec<_>=groups.iter().map(|g|g.iter().map(|&i|covariate[i]).sum::<f64>()/g.len() as f64).collect();
    let scores:Vec<_>=groups.iter().map(|g|g.iter().map(|&i|scores[i]).sum::<f64>()/g.len() as f64).collect();
    let mut sorted=means.clone(); sorted.sort_by(f64::total_cmp);
    let median=quantile(&sorted,0.5); let cutpoints=[quantile(&sorted,1./3.),quantile(&sorted,2./3.)];
    let high:Vec<_>=scores.iter().zip(&means).filter_map(|(&s,&m)|(m>median).then_some(s)).collect();
    let low:Vec<_>=scores.iter().zip(&means).filter_map(|(&s,&m)|(m<=median).then_some(s)).collect();
    let contrast=welch(&high,&low,level);
    let omnibus=(|| {
        if cutpoints[0]>=cutpoints[1] { return Err("The tertile boundaries are not distinct."); }
        let mut bins=[vec![],vec![],vec![]];
        for (&s,&m) in scores.iter().zip(&means) { bins[if m<=cutpoints[0] {0} else if m<=cutpoints[1] {1} else {2}].push(s); }
        if bins.iter().any(Vec::is_empty) || scores.len()<=3 { return Err("Each of the three cluster groups needs observations and residual degrees of freedom."); }
        let mean=scores.iter().sum::<f64>()/scores.len() as f64;
        let mut between=0.; let mut within=0.;
        for bin in bins { let m=bin.iter().sum::<f64>()/bin.len() as f64; between+=bin.len() as f64*(m-mean).powi(2); within+=bin.iter().map(|v|(v-m).powi(2)).sum::<f64>(); }
        if within<=0. {return Err("The within-group score variance is zero.");}
        let residual_df=scores.len()-3; let statistic=(between/2.)/(within/residual_df as f64);
        let p_value=spec_math::cephes64::incbet(residual_df as f64/2.,1.,residual_df as f64/(residual_df as f64+2.*statistic));
        Ok(Omnibus {statistic,df:2,residual_df,p_value})
    })();
    Ok(Between {means,scores,median,cutpoints,contrast,omnibus})
}
pub fn within(scores:&[f64],labels:&[usize],covariate:&[f64],threshold:f64,level:f64)->Result<Contrast,&'static str> {
    if !threshold.is_finite() {return Err("The within-cluster threshold must be finite.");}
    let groups=groups(scores,labels,covariate)?;
    let differences=groups.iter().map(|g| {
        let mut high=vec![]; let mut low=vec![];
        for &i in g { if covariate[i]>=threshold {high.push(scores[i])} else {low.push(scores[i])} }
        if high.is_empty() || low.is_empty() {return Err("Every cluster needs observations on both sides of the within-cluster threshold. No clusters were silently removed.");}
        Ok(high.iter().sum::<f64>()/high.len() as f64-low.iter().sum::<f64>()/low.len() as f64)
    }).collect::<Result<Vec<_>,_>>()?;
    paired(&differences,level)
}
