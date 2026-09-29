//! Typed optional forest analyses. Computation delegates to the pinned GRF port.
use hirmos_causal_core::grf::{inference, rank, sampling::Clusters};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Weighting { Uniform {}, Column { column: String } }
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum Sampling {
    Independent { weighting: Weighting },
    Clustered { column: String, equalize: bool, weighting: Weighting },
    EqualClusters { column: String },
}
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AverageMethod { Aipw, Tmle }
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Covariance { Hc0, Hc1, Hc2, Hc3 }
impl Covariance {
    pub fn core(self) -> inference::Covariance { match self {
        Self::Hc0 => inference::Covariance::Hc0, Self::Hc1 => inference::Covariance::Hc1,
        Self::Hc2 => inference::Covariance::Hc2, Self::Hc3 => inference::Covariance::Hc3,
    } }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Projection { None {}, Linear { columns: Vec<String>, overlap: bool, covariance: Covariance } }
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RankTarget { Autoc, Qini }
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Ranking {
    None {},
    External { columns: Vec<String>, rationale: String, target: RankTarget, quantiles: Vec<f64>, replications: usize, seed: u32 },
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Specification { pub sampling: Sampling, pub average_method: AverageMethod, pub projection: Projection, pub ranking: Ranking,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<NamedColumn>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderation: Option<Moderation>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub struct Moderation { pub between: Vec<String>, pub within: Vec<Within> }
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Within { pub column: String, pub threshold: f64 }
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedColumn { pub column: String, pub name: String }
impl Default for Specification {
    fn default() -> Self { Self { sampling: Sampling::Independent { weighting: Weighting::Uniform {} }, average_method: AverageMethod::Aipw, projection: Projection::None {}, ranking: Ranking::None {}, labels: None, moderation: None } }
}
pub struct Sample { pub labels: Vec<usize>, pub clusters: Clusters, pub weights: Option<Vec<f64>>, pub normalized: Vec<f64>, pub clustered: bool }
pub fn column<'a>(name: &str, names: &[String], values: &'a [f64], rows: usize) -> Result<&'a [f64], String> {
    let found: Vec<_> = names.iter().enumerate().filter(|(_, n)| n.as_str() == name).map(|(i, _)| i).collect();
    if found.len() != 1 { return Err(format!("The auxiliary column {name} must have one untransformed numeric representation.")); }
    values.get(found[0]*rows..(found[0]+1)*rows).ok_or_else(|| "An auxiliary column is outside the prepared matrix.".into())
}
pub fn sample(spec: &Specification, names: &[String], values: &[f64], rows: usize) -> Result<Sample, String> {
    if let Sampling::EqualClusters { column } = &spec.sampling {
        let mut resolved = spec.clone();
        resolved.sampling = Sampling::Clustered { column: column.clone(), equalize: true, weighting: Weighting::Uniform {} };
        return sample(&resolved, names, values, rows);
    }
    let (labels, equalize, weighting, clustered) = match &spec.sampling {
        Sampling::Independent { weighting } => ((0..rows).collect::<Vec<_>>(), false, weighting, false),
        Sampling::Clustered { column: name, equalize, weighting } => {
            let mut ids = BTreeMap::new();
            let labels = column(name, names, values, rows)?.iter().map(|value| {
                let key = if *value == 0.0 { 0 } else { value.to_bits() };
                let next = ids.len(); *ids.entry(key).or_insert(next)
            }).collect();
            if ids.len() < 2 { return Err("Clustered inference requires at least two independent clusters.".into()); }
            (labels, *equalize, weighting, true)
        }
        Sampling::EqualClusters { .. } => unreachable!(),
    };
    let weights = match weighting { Weighting::Uniform {} => None, Weighting::Column { column: name } => Some(column(name, names, values, rows)?.to_vec()) };
    let normalized = inference::observation_weights(&labels, weights.as_deref(), equalize).map_err(str::to_owned)?;
    let mut sizes = BTreeMap::new();
    for label in &labels { *sizes.entry(*label).or_insert(0usize) += 1; }
    let per_cluster = if equalize { sizes.values().min() } else { sizes.values().max() }.copied().ok_or("The sample is empty.")?;
    let clusters = Clusters::new(&labels, per_cluster).map_err(str::to_owned)?;
    Ok(Sample { labels, clusters, weights, normalized, clustered })
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Linear { pub estimates: Vec<f64>, pub standard_errors: Vec<f64>, pub statistics: Vec<f64>, pub p_values: Vec<f64>, pub degrees_of_freedom: usize }
impl From<inference::LinearSummary> for Linear {
    fn from(v: inference::LinearSummary) -> Self { Self { estimates: v.estimates, standard_errors: v.standard_errors, statistics: v.statistics, p_values: v.p_values, degrees_of_freedom: v.df } }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankRule { pub estimate: f64, pub standard_error: f64, pub toc: Vec<f64>, pub toc_standard_error: Vec<f64> }
impl From<rank::RuleResult> for RankRule {
    fn from(v: rank::RuleResult) -> Self { Self { estimate: v.estimate, standard_error: v.standard_error, toc: v.toc, toc_standard_error: v.toc_standard_error } }
}
#[derive(Serialize)]
pub struct RankEvidence { pub rules: Vec<RankRule>, pub difference: Option<RankRule> }
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Reading<T> { NotRequested {}, Unavailable { reason: String }, Estimated { result: T } }
pub fn reading(result: Result<inference::LinearSummary, &'static str>) -> Reading<Linear> { match result {
    Ok(result) if result.df > 0 && [&result.estimates, &result.standard_errors, &result.statistics, &result.p_values].iter().all(|values| values.iter().all(|v| v.is_finite())) => Reading::Estimated { result: result.into() },
    Ok(_) => Reading::Unavailable { reason: "The projection does not have finite coefficients and uncertainty.".into() },
    Err(reason) => Reading::Unavailable { reason: reason.into() },
} }
#[derive(Serialize)]
pub struct Evidence { pub specification: Specification, pub projection: Reading<Linear>, pub ranking: Reading<RankEvidence>,
    #[serde(skip_serializing_if="Option::is_none")] pub moderation: Option<Vec<ModerationTest>> }
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct ModerationTest { pub column: String, pub method: String, pub result: Reading<Test> }
#[derive(Serialize)]
#[serde(tag="kind", rename_all="kebab-case", rename_all_fields="camelCase")]
pub enum Test {
    Contrast { estimate:f64, standard_error:f64, statistic:f64, degrees_of_freedom:f64, p_value:f64, lower:f64, upper:f64 },
    Omnibus { statistic:f64, degrees_of_freedom:usize, residual_degrees_of_freedom:usize, p_value:f64 },
}
fn contrast(v:Result<hirmos_causal_core::grf::moderation::Contrast,&str>)->Reading<Test> {
    match v { Ok(v)=>Reading::Estimated { result:Test::Contrast {estimate:v.estimate,standard_error:v.standard_error,statistic:v.statistic,degrees_of_freedom:v.df,p_value:v.p_value,lower:v.lower,upper:v.upper} }, Err(e)=>Reading::Unavailable {reason:e.into()} }
}
pub fn unavailable(spec: &Specification, reason: &str) -> Evidence {
    Evidence { specification: spec.clone(),
        projection: match spec.projection { Projection::None {} => Reading::NotRequested {}, _ => Reading::Unavailable { reason: reason.into() } },
        ranking: match spec.ranking { Ranking::None {} => Reading::NotRequested {}, _ => Reading::Unavailable { reason: reason.into() } },
        moderation: spec.moderation.as_ref().map(|m| {
            let mut tests=vec![];
            for column in &m.between { for method in ["Cluster means above versus at or below the median (Welch)","Cluster-mean tertiles (ANOVA)"] {
                tests.push(ModerationTest {column:column.clone(),method:method.into(),result:Reading::Unavailable {reason:reason.into()}});
            } }
            for within in &m.within { tests.push(ModerationTest {column:within.column.clone(),method:format!("Within-cluster values ≥ {} versus < {} (paired)",within.threshold,within.threshold),result:Reading::Unavailable {reason:reason.into()}}); }
            tests
        }),
    }
}
pub fn evaluate(spec: &Specification, data: &inference::Observations<'_>, sample: &Sample, names: &[String], values: &[f64], debiasing: Option<&[f64]>, level: f64) -> Result<Evidence, String> {
    let rows = data.y.len();
    let projection = match &spec.projection {
        Projection::None {} => Reading::NotRequested {},
        Projection::Linear { columns, overlap, covariance } => {
            if columns.is_empty() { return Err("Select at least one covariate for the linear projection.".into()); }
            let covariates = columns.iter().map(|n| column(n, names, values, rows).map(<[f64]>::to_vec)).collect::<Result<Vec<_>, _>>()?;
            let result = if *overlap { inference::overlap_projection(data, &covariates, &sample.normalized, &sample.labels, covariance.core(), debiasing) }
                else { inference::scores(data, debiasing).and_then(|scores| inference::projection(&scores, &covariates, &sample.normalized, &sample.labels, covariance.core())) };
            reading(result)
        }
    };
    let ranking = match &spec.ranking {
        Ranking::None {} => Reading::NotRequested {},
        Ranking::External { columns, rationale, target, quantiles, replications, seed } => {
            if rationale.trim().is_empty() || *replications < 2 || columns.is_empty() || columns.len() > 2 { return Err("RATE requires one or two independently constructed priorities, an independence rationale and at least two bootstrap replications.".into()); }
            let priorities = columns.iter().map(|n| column(n, names, values, rows).map(<[f64]>::to_vec)).collect::<Result<Vec<_>, _>>()?;
            match inference::scores(data, debiasing).and_then(|scores| rank::fit(&scores, &priorities, Some(&sample.normalized), &sample.labels, quantiles, match target { RankTarget::Autoc => rank::Target::Autoc, RankTarget::Qini => rank::Target::Qini }, *replications, *seed)) {
                Ok(result) if result.rules.iter().chain(result.difference.iter()).all(|r| r.estimate.is_finite() && r.standard_error.is_finite() && r.toc.iter().chain(&r.toc_standard_error).all(|v| v.is_finite())) => Reading::Estimated { result: RankEvidence { rules: result.rules.into_iter().map(Into::into).collect(), difference: result.difference.map(Into::into) } },
                Ok(_) => Reading::Unavailable { reason: "The prioritization estimates or bootstrap standard errors are not finite.".into() },
                Err(reason) => Reading::Unavailable { reason: reason.into() },
            }
        }
    };
    let moderation=spec.moderation.as_ref().map(|m| -> Result<Vec<ModerationTest>,String> {
        if !matches!(spec.sampling,Sampling::EqualClusters {..}) || data.w.iter().any(|&w|w!=0. && w!=1.) {return Err("Cluster-score moderation requires binary treatment and equal cluster weights.".into());}
        let scores=inference::scores(data,None).map_err(str::to_owned)?;
        let mut tests=vec![];
        for name in &m.between {
            let c=column(name,names,values,data.y.len())?;
            let compared=hirmos_causal_core::grf::moderation::between(&scores,&sample.labels,c,level);
            let (welch,anova)=match compared {
                Ok(v)=>(contrast(v.contrast),match v.omnibus {Ok(v)=>Reading::Estimated {result:Test::Omnibus {statistic:v.statistic,degrees_of_freedom:v.df,residual_degrees_of_freedom:v.residual_df,p_value:v.p_value}},Err(e)=>Reading::Unavailable {reason:e.into()}}),
                Err(e)=>(Reading::Unavailable {reason:e.into()},Reading::Unavailable {reason:e.into()}),
            };
            tests.push(ModerationTest {column:name.clone(),method:"Cluster means above versus at or below the median (Welch)".into(),result:welch});
            tests.push(ModerationTest {column:name.clone(),method:"Cluster-mean tertiles (ANOVA)".into(),result:anova});
        }
        for within in &m.within {
            let c=column(&within.column,names,values,data.y.len())?;
            tests.push(ModerationTest {column:within.column.clone(),method:format!("Within-cluster values ≥ {} versus < {} (paired)",within.threshold,within.threshold),
                result:contrast(hirmos_causal_core::grf::moderation::within(&scores,&sample.labels,c,within.threshold,level))});
        }
        Ok(tests)
    }).transpose()?;
    Ok(Evidence { specification: spec.clone(), projection, ranking, moderation })
}
