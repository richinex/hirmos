//! Browser boundary for the fitted dynamic linear-SCM counterfactual.

use super::*;
use hirmos_causal_core::dynamic_counterfactual::{
    DynamicLinearScm, DynamicParentSpec, InterventionTiming,
};
use hirmos_causal_core::linear_mediation::{
    LinearMediationBootstrap, LinearMediationBootstrapOptions,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn dynamic_linear_scm_counterfactual(
    values: &[f64],
    rows: usize,
    columns: usize,
    nodes: &[usize],
    stat_lag: usize,
    graph: &[Vec<Vec<String>>],
    treatment: usize,
    outcome: usize,
    timing: DynamicInterventionTiming,
    steps: usize,
    interventions: (f64, f64),
    uncertainty: DynamicCounterfactualUncertainty,
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("dynamic linear SCM counterfactual", values, rows, columns)?;
    let variables = nodes.len();
    if variables < 2 || nodes.iter().any(|&column| column >= columns) {
        return Err(
            "dynamic linear SCM counterfactual needs at least two measured graph nodes".to_owned(),
        );
    }
    if treatment >= variables || outcome >= variables || treatment == outcome {
        return Err(
            "dynamic linear SCM treatment and outcome must be distinct graph nodes".to_owned(),
        );
    }
    if graph.len() != variables
        || graph.iter().any(|targets| {
            targets.len() != variables || targets.iter().any(|lags| lags.len() != stat_lag + 1)
        })
    {
        return Err(
            "dynamic linear SCM graph must be variables by variables by stat_lag + 1 marks"
                .to_owned(),
        );
    }
    if steps == 0 || !interventions.0.is_finite() || !interventions.1.is_finite() {
        return Err(
            "dynamic linear SCM needs positive steps and two finite intervention values".to_owned(),
        );
    }
    let matrix = DMatrix::from_column_slice(rows, columns, values);
    let data: Vec<Vec<f64>> = nodes
        .iter()
        .map(|&column| (0..rows).map(|row| matrix[(row, column)]).collect())
        .collect();
    let mut parents = vec![Vec::new(); variables];
    for (source, targets) in graph.iter().enumerate() {
        for (target, lags) in targets.iter().enumerate() {
            for (lag, edge) in lags.iter().enumerate() {
                if edge == "-->" {
                    parents[target].push(DynamicParentSpec {
                        variable: source,
                        lag,
                    });
                } else if !edge.is_empty() && edge != "<--" {
                    return Err(format!(
                        "dynamic linear SCM supports directed DAG marks; found {edge:?}"
                    ));
                }
            }
        }
    }
    for list in &mut parents {
        list.sort_by_key(|parent| (parent.lag, parent.variable));
        list.dedup();
    }
    let model = DynamicLinearScm::fit(&data, &parents).map_err(|error| error.to_string())?;
    let core_timing = match timing {
        DynamicInterventionTiming::Point { time } => InterventionTiming::Point { time },
        DynamicInterventionTiming::Persistent { start } => InterventionTiming::Persistent { start },
    };
    let path = model
        .counterfactual_pair(
            &data,
            treatment,
            outcome,
            core_timing,
            steps,
            interventions.0,
            interventions.1,
        )
        .map_err(|error| error.to_string())?;
    let uncertainty_evidence = match uncertainty {
        DynamicCounterfactualUncertainty::None => DynamicCounterfactualUncertaintyEvidence::None,
        DynamicCounterfactualUncertainty::BlockBootstrap {
            samples,
            block_length,
            confidence_level,
            seed,
        } => {
            if !(20..=5000).contains(&samples) || !(0.0..1.0).contains(&confidence_level) {
                return Err("dynamic counterfactual bootstrap needs 20 to 5,000 refits and a confidence level strictly between zero and one".to_owned());
            }
            let core_block_length = match block_length {
                CausalEffectsBlockLength::Fixed { length } => BootstrapBlockLength::Fixed(length),
                CausalEffectsBlockLength::CubeRoot => BootstrapBlockLength::CubeRoot,
            };
            progress("dynamic-counterfactual-bootstrap", 0, samples);
            let bootstrap = LinearMediationBootstrap::fit_with_progress(
                &data,
                &parents,
                LinearMediationBootstrapOptions {
                    samples,
                    block_length: core_block_length,
                    seed,
                },
                |completed, total| progress("dynamic-counterfactual-bootstrap", completed, total),
            )
            .map_err(|error| error.to_string())?;
            let intervals = bootstrap
                .counterfactual_uncertainty(
                    treatment,
                    outcome,
                    core_timing,
                    steps,
                    interventions.0,
                    interventions.1,
                    confidence_level,
                )
                .map_err(|error| error.to_string())?;
            DynamicCounterfactualUncertaintyEvidence::BlockBootstrap {
                samples,
                block_length,
                resolved_block_length: bootstrap.resolved_block_length,
                confidence_level,
                seed,
                effect_draws: intervals.effect_draws,
                pointwise_effect_interval: intervals.pointwise_interval,
                average_draws: intervals.average_draws,
                average_interval: intervals.average_interval,
                cumulative_draws: intervals.cumulative_draws,
                cumulative_interval: intervals.cumulative_interval,
            }
        }
    };
    let equations = model
        .equations
        .iter()
        .enumerate()
        .map(|(variable, equation)| DynamicScmEquationEvidence {
            variable,
            intercept: equation.intercept,
            parents: equation
                .parents
                .iter()
                .map(|parent| DynamicScmParentEvidence {
                    variable: parent.variable,
                    lag: parent.lag,
                    coefficient: parent.coefficient,
                })
                .collect(),
            residual_scale: equation.residual_scale,
        })
        .collect();
    Ok(AnalysisResult::DynamicLinearScmCounterfactual {
        observations: rows,
        fitted_observations: model.fitted_observations,
        max_lag: model.max_lag,
        order: model.order,
        equations,
        timing,
        interventions,
        start: path.start,
        factual_outcome: path.factual,
        counterfactual_low: path.low,
        counterfactual_high: path.high,
        effects: path.contrast,
        average_effect: path.average_contrast,
        cumulative_effect: path.cumulative_contrast,
        uncertainty: uncertainty_evidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn facade_keeps_lagged_self_arrows_and_reports_a_path() {
        let rows = 80;
        let mut x = vec![0.0; rows];
        let mut y = vec![0.0; rows];
        for time in 1..rows {
            x[time] = 0.5 * x[time - 1] + (time as f64 * 0.7).sin();
            y[time] = 0.6 * y[time - 1] + 0.8 * x[time - 1] + (time as f64 * 1.1).cos() * 0.1;
        }
        let values: Vec<f64> = x.into_iter().chain(y).collect();
        let mut graph = vec![vec![vec![String::new(); 2]; 2]; 2];
        graph[0][0][1] = "-->".to_owned();
        graph[1][1][1] = "-->".to_owned();
        graph[0][1][1] = "-->".to_owned();
        let result = dynamic_linear_scm_counterfactual(
            &values,
            rows,
            2,
            &[0, 1],
            1,
            &graph,
            0,
            1,
            DynamicInterventionTiming::Point { time: 20 },
            12,
            (0.0, 1.0),
            DynamicCounterfactualUncertainty::None,
            |_, _, _| {},
        )
        .unwrap();
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(value["kind"], "dynamicLinearScmCounterfactual");
        assert_eq!(value["maxLag"], 1);
        assert_eq!(value["effects"].as_array().unwrap().len(), 12);
        assert!(value["effects"][1].as_f64().unwrap().abs() > 0.1);

        let completed = Cell::new(0);
        let result = dynamic_linear_scm_counterfactual(
            &values,
            rows,
            2,
            &[0, 1],
            1,
            &graph,
            0,
            1,
            DynamicInterventionTiming::Persistent { start: 20 },
            12,
            (0.0, 1.0),
            DynamicCounterfactualUncertainty::BlockBootstrap {
                samples: 24,
                block_length: CausalEffectsBlockLength::Fixed { length: 4 },
                confidence_level: 0.9,
                seed: 37,
            },
            |stage, done, total| {
                assert_eq!(stage, "dynamic-counterfactual-bootstrap");
                assert_eq!(total, 24);
                completed.set(done);
            },
        )
        .unwrap();
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(completed.get(), 24);
        assert_eq!(value["uncertainty"]["kind"], "blockBootstrap");
        assert_eq!(
            value["uncertainty"]["effectDraws"]
                .as_array()
                .unwrap()
                .len(),
            24
        );
        assert_eq!(
            value["uncertainty"]["pointwiseEffectInterval"][0]
                .as_array()
                .unwrap()
                .len(),
            12
        );
        assert_eq!(
            value["uncertainty"]["averageDraws"]
                .as_array()
                .unwrap()
                .len(),
            24
        );
    }
}
