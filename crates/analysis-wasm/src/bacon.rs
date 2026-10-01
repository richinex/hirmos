//! Validated browser boundary for a TWFE decomposition, not an ATT estimator.
use hirmos_causal_core::panel_design::{bacon as core, Panel};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Specification {
    Unadjusted {},
    Adjusted { controls: Vec<usize> },
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    rows: usize,
    columns: usize,
    units: Vec<String>,
    times: Vec<i64>,
    outcome: usize,
    treatment: usize,
    specification: Specification,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum Comparison {
    TreatedVsNever { adoption: i64 },
    EarlierVsLater { earlier: i64, later: i64 },
    LaterVsEarlier { earlier: i64, later: i64 },
    LaterVsAlways { adoption: i64 },
    BothTreated { earlier: i64, later: i64 },
}
#[derive(Serialize)]
struct Component {
    comparison: Comparison,
    estimate: f64,
    weight: f64,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum Decomposition {
    Unadjusted {
        components: Vec<Component>,
    },
    Adjusted {
        within_estimate: f64,
        within_weight: f64,
        between: Vec<Component>,
    },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    version: u8,
    specification: Specification,
    observations: usize,
    units: usize,
    twfe: f64,
    reconstructed: f64,
    decomposition: Decomposition,
}
fn describe(error: core::Error) -> String {
    match error {
        core::Error::InvalidColumns=>"Choose distinct outcome, treatment and adjustment columns; adjusted decomposition requires at least one covariate.".into(),
        core::Error::NonBinaryTreatment{row}=>format!("Treatment must be 0 or 1. Source row {} has another value.",row+1),
        core::Error::TreatmentReversal{unit,period}=>format!("Treatment reverses for unit index {unit} at period {period}. This decomposition requires absorbing treatment."),
        core::Error::NoComparisons=>"No eligible treatment-timing comparisons remain.".into(),
        core::Error::Regression(error)=>error.to_string(),
        core::Error::Numerical=>"The decomposition could not produce finite estimates and weights.".into(),
        core::Error::RankDeficientProjection=>"An adjustment projection is rank deficient. This decomposition does not support redundant or fixed-effect-absorbed covariates.".into(),
        core::Error::DegenerateWithin=>"The adjusted decomposition has insufficient within-cohort variation to estimate its within component.".into(),
    }
}
pub(crate) fn run(values: &[f64], r: Request) -> Result<Evidence, String> {
    if r.rows == 0
        || r.columns < 2
        || r.rows.checked_mul(r.columns) != Some(values.len())
        || r.units.len() != r.rows
        || r.times.len() != r.rows
        || r.units.iter().any(|u| u.trim().is_empty())
    {
        return Err(
            "The decomposition matrix and panel keys must have matching nonzero dimensions.".into(),
        );
    }
    let unique: BTreeMap<_, _> = r
        .units
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, u)| (u, i as u64))
        .collect();
    let keys: Vec<_> = r
        .units
        .iter()
        .zip(&r.times)
        .map(|(u, t)| (unique[u], *t))
        .collect();
    let panel = Panel::new(&keys, DMatrix::from_row_slice(r.rows, r.columns, values)).map_err(
        |e| match e {
            hirmos_causal_core::panel_design::Error::DuplicateKey => {
                "Each unit-period key must be unique.".into()
            }
            hirmos_causal_core::panel_design::Error::Gap => {
                "Panel periods must be consecutive; missing periods cannot be compressed.".into()
            }
            hirmos_causal_core::panel_design::Error::Unbalanced => {
                "This decomposition requires the same observed periods for every unit.".into()
            }
            _ => "The decomposition requires a nonempty, finite, balanced panel.".to_string(),
        },
    )?;
    let (twfe, reconstructed, decomposition) = match &r.specification {
        Specification::Unadjusted {} => {
            let f = panel
                .bacon_unadjusted(r.outcome, r.treatment)
                .map_err(describe)?;
            let components = f
                .components
                .into_iter()
                .map(|c| Component {
                    estimate: c.estimate,
                    weight: c.weight,
                    comparison: match c.comparison {
                        core::Comparison::TreatedVsNever { adoption } => {
                            Comparison::TreatedVsNever { adoption }
                        }
                        core::Comparison::EarlierVsLater { earlier, later } => {
                            Comparison::EarlierVsLater { earlier, later }
                        }
                        core::Comparison::LaterVsEarlier { earlier, later } => {
                            Comparison::LaterVsEarlier { earlier, later }
                        }
                        core::Comparison::LaterVsAlways { adoption } => {
                            Comparison::LaterVsAlways { adoption }
                        }
                    },
                })
                .collect();
            (
                f.twfe,
                f.reconstructed,
                Decomposition::Unadjusted { components },
            )
        }
        Specification::Adjusted { controls } => {
            let f = panel
                .bacon_adjusted(r.outcome, r.treatment, controls)
                .map_err(describe)?;
            let between = f
                .between
                .into_iter()
                .map(|c| Component {
                    estimate: c.estimate,
                    weight: c.weight,
                    comparison: match c.comparison {
                        core::adjusted::Comparison::TreatedVsNever { adoption } => {
                            Comparison::TreatedVsNever { adoption }
                        }
                        core::adjusted::Comparison::BothTreated { earlier, later } => {
                            Comparison::BothTreated { earlier, later }
                        }
                        core::adjusted::Comparison::LaterVsAlways { adoption } => {
                            Comparison::LaterVsAlways { adoption }
                        }
                    },
                })
                .collect();
            (
                f.twfe,
                f.reconstructed,
                Decomposition::Adjusted {
                    within_estimate: f.within_estimate,
                    within_weight: f.within_weight,
                    between,
                },
            )
        }
    };
    Ok(Evidence {
        version: 1,
        specification: r.specification,
        observations: r.rows,
        units: unique.len(),
        twfe,
        reconstructed,
        decomposition,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn schema_refuses_unknown_options_and_bad_shapes() {
        let request = serde_json::json!({"rows":4,"columns":2,"units":["a","a","b","b"],"times":[0,1,0,1],"outcome":0,"treatment":1,"specification":{"kind":"unadjusted"}});
        let mut bad = request.clone();
        bad["specification"]["controls"] = serde_json::json!([2]);
        assert!(serde_json::from_value::<Request>(bad).is_err());
        assert!(run(&[0.; 7], serde_json::from_value(request).unwrap()).is_err());
    }
    #[test]
    fn browser_contract_preserves_castle_reference() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../causal-core/oracle/fixtures/bacon.json"))
                .unwrap();
        let case = &fixture["cases"]["castle"];
        let data = case["data"].as_array().unwrap();
        let values: Vec<f64> = data
            .iter()
            .flat_map(|r| [r["y"].as_f64().unwrap(), r["treated"].as_f64().unwrap()])
            .collect();
        let request = Request {
            rows: data.len(),
            columns: 2,
            units: data.iter().map(|r| r["id"].to_string()).collect(),
            times: data.iter().map(|r| r["time"].as_i64().unwrap()).collect(),
            outcome: 0,
            treatment: 1,
            specification: Specification::Unadjusted {},
        };
        let result = run(&values, request).unwrap();
        assert!((result.twfe - case["twfe"].as_f64().unwrap()).abs() < 1e-8);
        let json = serde_json::to_value(result).unwrap();
        assert_eq!(json["decomposition"]["kind"], "unadjusted");
        assert_eq!(
            json["decomposition"]["components"]
                .as_array()
                .unwrap()
                .len(),
            25
        );
    }
}
