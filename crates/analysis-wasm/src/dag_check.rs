//! Browser façade for the parity-tested KCI and DAG falsification kernels.

use super::*;
use hirmos_causal_core::graph_falsification::{
    falsify_graph_with_progress, test_implications, uniformity_test, DagImplication,
    ImplicationDecision,
};

pub(crate) fn dag_check(
    values: &[f64],
    rows: usize,
    columns: usize,
    node_columns: &[usize],
    edges: &[(usize, usize)],
    implications: &[DagImplicationCommand],
    maximum_observations: usize,
    permutations: usize,
    significance_level: f64,
    run_falsification: bool,
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("DAG check", values, rows, columns)?;
    if node_columns.is_empty() {
        return Err("DAG check needs at least one observed node".to_owned());
    }
    if maximum_observations < 4 {
        return Err("DAG check observation cap must be at least 4".to_owned());
    }
    if node_columns.iter().any(|&column| column >= columns) {
        return Err("DAG check node columns must index the numeric matrix".to_owned());
    }
    let mut unique_columns = node_columns.to_vec();
    unique_columns.sort_unstable();
    unique_columns.dedup();
    if unique_columns.len() != node_columns.len() {
        return Err("DAG check node columns must be distinct".to_owned());
    }
    if edges
        .iter()
        .any(|&(cause, effect)| cause >= node_columns.len() || effect >= node_columns.len())
    {
        return Err("DAG check edges must use node positions".to_owned());
    }
    if topological_order(node_columns.len(), edges).is_none() {
        return Err("DAG check requires an acyclic directed graph".to_owned());
    }
    let implication_values: Vec<DagImplication> = implications
        .iter()
        .map(|implication| DagImplication {
            x: implication.x,
            y: implication.y,
            given: implication.given.clone(),
        })
        .collect();
    if implication_values.iter().any(|implication| {
        implication.x >= node_columns.len()
            || implication.y >= node_columns.len()
            || implication
                .given
                .iter()
                .any(|&node| node >= node_columns.len())
    }) {
        return Err("DAG implications must use node positions".to_owned());
    }
    let data = DMatrix::from_fn(rows, node_columns.len(), |row, node| {
        values[node_columns[node] * rows + row]
    });
    let dag = Dag::new(node_columns.len(), edges);

    progress("dag-implications", 0, implication_values.len());
    let tested = test_implications(
        &data,
        &implication_values,
        significance_level,
        maximum_observations,
    )
    .map_err(|error| format!("DAG implication tests were refused: {error:?}"))?;
    progress(
        "dag-implications",
        implication_values.len(),
        implication_values.len(),
    );
    let raw_p_values: Vec<f64> = tested.iter().map(|test| test.p_value).collect();
    let uniformity = uniformity_test(&raw_p_values)
        .ok_or_else(|| "DAG implication p-values could not be tested for uniformity".to_owned())?;

    let falsification = if run_falsification {
        let falsification_data = if rows <= maximum_observations {
            data
        } else {
            let mut rng = Mt19937::seeded(7);
            let selected = rng.permutation(rows);
            DMatrix::from_fn(maximum_observations, node_columns.len(), |row, column| {
                data[(selected[row], column)]
            })
        };
        let result = falsify_graph_with_progress(
            &dag,
            &falsification_data,
            permutations,
            significance_level,
            significance_level,
            11,
            |completed, total| progress("dag-permutations", completed, total),
        )
        .map_err(|error| format!("DAG falsification was refused: {error:?}"))?;
        DagFalsificationEvidence::Completed {
            permutations: result.permutations,
            given_lmc_violations: result.given_lmc_violations,
            given_lmc_tests: result.given_lmc_tests,
            given_lmc_violation_fraction: result.given_lmc_violation_fraction,
            permutation_lmc_violation_fractions: result.permutation_lmc_violation_fractions,
            permutation_tpa_violation_fractions: result.permutation_tpa_violation_fractions,
            p_value_lmc: result.p_value_lmc,
            p_value_tpa: result.p_value_tpa,
            permutations_in_markov_equivalence_class: result
                .permutations_in_markov_equivalence_class,
            falsifiable: result.falsifiable,
            falsified: result.falsified,
        }
    } else {
        progress("dag-permutations", 0, 0);
        DagFalsificationEvidence::Skipped {
            reason: "The DAG contains an unmeasured variable; relabeling only its observed projection would test a different graph.",
        }
    };

    Ok(AnalysisResult::DagCheck {
        observations: rows.min(maximum_observations),
        significance_level,
        correction: "holm",
        implications: tested
            .into_iter()
            .map(|test| DagImplicationEvidence {
                x: test.implication.x,
                y: test.implication.y,
                given: test.implication.given,
                p_value: test.p_value,
                adjusted_p_value: test.adjusted_p_value,
                observations: test.observations,
                decision: match test.decision {
                    ImplicationDecision::Contradicted => DagImplicationDecision::Contradicted,
                    ImplicationDecision::NotRefuted => DagImplicationDecision::NotRefuted,
                },
            })
            .collect(),
        uniformity: DagUniformityEvidence {
            statistic: uniformity.statistic,
            p_value: uniformity.p_value,
            tests: uniformity.tests,
        },
        falsification,
    })
}
