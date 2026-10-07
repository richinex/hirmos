use serde::{Deserialize, Serialize};
use hirmos_causal_core::{power::{self, Alternative, PowerDesign}, power_simulation::{self as sim, DependentScenario, TwoArmScenario}};

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Design {
    IndependentT { ratio: f64 },
    OneSampleT,
    PairedT,
    IndependentNormal { ratio: f64 },
}
impl Design {
    fn kernel(&self) -> PowerDesign { match self {
        Self::IndependentT { ratio } => PowerDesign::IndependentT { ratio: *ratio },
        Self::OneSampleT | Self::PairedT => PowerDesign::OneSampleT,
        Self::IndependentNormal { ratio } => PowerDesign::IndependentNormal { ratio: *ratio },
    }}
    fn ratio(&self) -> Option<f64> { match self { Self::IndependentT {ratio} | Self::IndependentNormal {ratio} => Some(*ratio), _=>None }}
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Tail { TwoSided, Larger, Smaller }
impl Tail { fn kernel(&self)->Alternative {match self {Self::TwoSided=>Alternative::TwoSided,Self::Larger=>Alternative::Larger,Self::Smaller=>Alternative::Smaller}} }
#[derive(Deserialize)]
#[serde(tag="kind", rename_all="camelCase", deny_unknown_fields)]
pub(crate) enum Target {
    Power { n: f64, effect: f64 },
    MinimumEffect { n: f64, target: f64 },
    SampleSize { effect: f64, target: f64 },
}
#[derive(Deserialize)]
#[serde(tag="kind", rename_all="camelCase", deny_unknown_fields)]
pub(crate) enum Scenario {
    Independent { treated: usize, controls: usize },
    EqualClusters { treated: usize, controls: usize, members: usize, icc: f64 },
    TwoPeriodDid { treated: usize, controls: usize, correlation: f64, violation: f64 },
}
#[derive(Deserialize)]
#[serde(tag="kind", rename_all="camelCase", deny_unknown_fields)]
pub(crate) enum Request {
    Analytical { design: Design, alternative: Tail, alpha: f64, target: Target },
    Simulation { scenario: Scenario, effect: f64, sd: f64, replications: usize, seed: u32, alpha: f64 },
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Point { n: f64, power: f64 }
#[derive(Serialize)]
#[serde(tag="kind", rename_all="camelCase", rename_all_fields="camelCase")]
pub(crate) enum Evidence {
    Analytical { effect: f64, n: f64, second_group: Option<f64>, power: f64, continuous_n: Option<f64>, curve: Vec<Point> },
    Simulation { replications: usize, rejected: usize, rejection_rate: f64, mcse: f64, coverage: f64, bias: f64, rmse: f64 },
}
fn numerical(e: power::PowerError)->String {format!("The power calculation could not be completed: {e:?}. Check the design, effect direction and requested power.")}
pub(crate) fn calculate(request: Request)->Result<Evidence,String> {
    match request {
        Request::Analytical {design,alternative,alpha,target}=>{
            let a=alternative.kernel(); let d=design.kernel();
            let (n,effect,continuous_n)=match target {
                Target::Power {n,effect}=>(n,effect,None),
                Target::MinimumEffect {n,target}=>(n,power::minimum_detectable_effect_for(d,n,alpha,target,a).map_err(numerical)?,None),
                Target::SampleSize {effect,target}=>{let continuous=power::required_sample_size_for(d,effect,alpha,target,a).map_err(numerical)?;(continuous.ceil(),effect,Some(continuous))}
            };
            let second_group=design.ratio().map(|r| if continuous_n.is_some(){(n*r).ceil()}else{n*r});
            let rounded=match d { PowerDesign::IndependentT{..}=>PowerDesign::IndependentT{ratio:second_group.unwrap()/n}, PowerDesign::IndependentNormal{..}=>PowerDesign::IndependentNormal{ratio:second_group.unwrap()/n}, other=>other };
            let achieved=power::design_power(rounded,effect,n,alpha,a).map_err(numerical)?;
            let mut curve=Vec::new();
            let minimum=design.ratio().map_or(2.,|r|2_f64.max(2./r));
            for i in 0..41 { let size=(n*(0.25+1.75*i as f64/40.)).max(minimum);curve.push(Point {n:size,power:power::design_power(d,effect,size,alpha,a).map_err(numerical)?}); }
            Ok(Evidence::Analytical{effect,n,second_group,power:achieved,continuous_n,curve})
        }
        Request::Simulation{scenario,effect,sd,replications,seed,alpha}=>{
            let (treated,controls,members)=match scenario {Scenario::Independent{treated,controls}|Scenario::TwoPeriodDid{treated,controls,..}=>(treated,controls,1),Scenario::EqualClusters{treated,controls,members,..}=>(treated,controls,members)};
            if (treated as u128+controls as u128)*members as u128>200_000 || replications==0 || replications>10000 || treated<2 || controls<2 || members==0 || (treated as u128+controls as u128)*members as u128*replications as u128>20_000_000 {return Err("Use at least two units per arm and 1–10,000 replications, with at most 200,000 observations per replication and 20 million in total.".into())}
            let result=match scenario {
                Scenario::Independent{treated,controls}=>sim::simulate_two_arm(TwoArmScenario{treated,controls,effect,noise_sd:sd},replications,seed.into(),alpha),
                Scenario::EqualClusters{treated,controls,members,icc}=>sim::simulate_dependent(DependentScenario::EqualClusters{treated_clusters:treated,control_clusters:controls,members,icc,effect,noise_sd:sd},replications,seed.into(),alpha),
                Scenario::TwoPeriodDid{treated,controls,correlation,violation}=>sim::simulate_dependent(DependentScenario::TwoPeriodDiD{treated,controls,correlation,untreated_trend_difference:violation,effect,noise_sd:sd},replications,seed.into(),alpha),
            }.map_err(|e|format!("The simulation could not be completed: {e:?}. No failed fits were omitted."))?;
            let (_,r)=result;
            Ok(Evidence::Simulation{replications:r.replications,rejected:r.rejected,rejection_rate:r.rejection_rate,mcse:r.rejection_mcse,coverage:r.coverage,bias:r.bias,rmse:r.rmse})
        }
    }
}
