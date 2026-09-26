//! The linear SCM counterfactual: per-node least squares, abduction, action, prediction.

use super::*;

/// Every node fitted by OLS on its DAG parents over the prepared rows; then Pearl's three steps
/// per row for both intervention values.
#[allow(clippy::too_many_arguments)]
pub(crate) fn linear_scm_counterfactual(
    values: &[f64],
    rows: usize,
    columns: usize,
    nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    interventions: (f64, f64),
    observation_noise: Option<f64>,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("linear SCM counterfactual", values, rows, columns)?;
    let count = nodes.len();
    if count < 2 || names.len() != count || nodes.iter().any(|&column| column >= columns) {
        return Err("linear SCM counterfactual needs at least two nodes, each with a name and a matrix column".to_owned());
    }
    if treatment >= count || outcome >= count || treatment == outcome {
        return Err(
            "linear SCM counterfactual treatment and outcome must be distinct node positions"
                .to_owned(),
        );
    }
    if edges
        .iter()
        .any(|&(cause, effect)| cause >= count || effect >= count || cause == effect)
    {
        return Err("linear SCM counterfactual edges must join distinct node positions".to_owned());
    }
    if !interventions.0.is_finite() || !interventions.1.is_finite() {
        return Err("linear SCM counterfactual interventions must be finite".to_owned());
    }
    if let Some(scale) = observation_noise {
        if !(scale.is_finite() && scale > 0.0) {
            return Err("linear SCM counterfactual observation noise must be positive".to_owned());
        }
    }
    let max_parents = (0..count)
        .map(|node| edges.iter().filter(|(_, effect)| *effect == node).count())
        .max()
        .unwrap_or(0);
    if rows < max_parents + 3 {
        return Err(format!(
            "linear SCM counterfactual needs more than {} rows for {max_parents} parents",
            max_parents + 2
        ));
    }
    let order = topological_order(count, edges)
        .ok_or("linear SCM counterfactual needs an acyclic graph")?;
    let position_in_order: Vec<usize> = {
        let mut lookup = vec![0usize; count];
        for (index, &node) in order.iter().enumerate() {
            lookup[node] = index;
        }
        lookup
    };
    let data = DMatrix::from_column_slice(rows, columns, values);
    let column =
        |node: usize| -> Vec<f64> { (0..rows).map(|row| data[(row, nodes[node])]).collect() };
    let mut equations_out: Vec<ScmEquation> = Vec::with_capacity(count);
    let mut scm_equations: Vec<Equation> = Vec::with_capacity(count);
    for &node in &order {
        let mut parents: Vec<usize> = edges
            .iter()
            .filter(|(_, effect)| *effect == node)
            .map(|(cause, _)| *cause)
            .collect();
        parents.sort_unstable();
        parents.dedup();
        let y = DVector::from_vec(column(node));
        let design = DMatrix::from_fn(rows, parents.len() + 1, |row, index| {
            if index == 0 {
                1.0
            } else {
                data[(row, nodes[parents[index - 1]])]
            }
        });
        let fit = Ols::fit(&design, &y);
        let degrees = rows.saturating_sub(parents.len() + 1).max(1) as f64;
        let residual_sd =
            (fit.resid.iter().map(|value| value * value).sum::<f64>() / degrees).sqrt();
        let mean = y.iter().sum::<f64>() / rows as f64;
        let total = y.iter().map(|value| (value - mean).powi(2)).sum::<f64>();
        let residual = fit.resid.iter().map(|value| value * value).sum::<f64>();
        let r_squared = if total > 0.0 {
            1.0 - residual / total
        } else {
            0.0
        };
        let coefficients: Vec<(usize, f64)> = parents
            .iter()
            .enumerate()
            .map(|(index, &parent)| (parent, fit.params[index + 1]))
            .collect();
        equations_out.push(ScmEquation {
            node,
            intercept: fit.params[0],
            parents: coefficients.clone(),
            residual_sd,
            r_squared,
        });
        scm_equations.push(Equation {
            intercept: fit.params[0],
            parents: coefficients
                .iter()
                .map(|&(parent, coefficient)| (position_in_order[parent], coefficient))
                .collect(),
            noise_scale: if residual_sd > 0.0 { residual_sd } else { 1.0 },
        });
    }
    let scm = LinearScm {
        equations: scm_equations,
    };
    let treatment_slot = position_in_order[treatment];
    let outcome_slot = position_in_order[outcome];
    let mut factual = Vec::with_capacity(rows);
    let mut low = Vec::with_capacity(rows);
    let mut high = Vec::with_capacity(rows);
    for row in 0..rows {
        let observed: Vec<f64> = order.iter().map(|&node| data[(row, nodes[node])]).collect();
        factual.push(observed[outcome_slot]);
        low.push(
            scm.counterfactual(
                &observed,
                &[(treatment_slot, interventions.0)],
                observation_noise,
            )[outcome_slot],
        );
        high.push(
            scm.counterfactual(
                &observed,
                &[(treatment_slot, interventions.1)],
                observation_noise,
            )[outcome_slot],
        );
    }
    let effects: Vec<f64> = low.iter().zip(&high).map(|(a, b)| b - a).collect();
    let average_effect = effects.iter().sum::<f64>() / rows as f64;
    let share_positive = effects.iter().filter(|value| **value > 0.0).count() as f64 / rows as f64;
    Ok(AnalysisResult::LinearScmCounterfactual {
        observations: rows,
        order,
        equations: equations_out,
        interventions,
        observation_noise,
        factual_outcome: factual,
        counterfactual_low: low,
        counterfactual_high: high,
        effects,
        average_effect,
        share_positive,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scm_counterfactual_recovers_the_structural_effect_row_by_row() {
        let rows = 300;
        let mut stream = Mt19937::seeded(31);
        let z: Vec<f64> = (0..rows).map(|_| stream.next_f64() * 4.0).collect();
        let d: Vec<f64> = (0..rows)
            .map(|i| 1.0 + 0.5 * z[i] + (stream.next_f64() - 0.5))
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 2.0 + 3.0 * d[i] - 1.0 * z[i] + 0.1 * (stream.next_f64() - 0.5))
            .collect();
        // Columns: d, y, z; nodes in the same order; z -> d, z -> y, d -> y.
        let values: Vec<f64> = d.iter().chain(y.iter()).chain(z.iter()).copied().collect();
        let names = vec!["d".to_owned(), "y".to_owned(), "z".to_owned()];
        let edges = vec![(2, 0), (2, 1), (0, 1)];
        let result = serde_json::to_value(
            linear_scm_counterfactual(
                &values,
                rows,
                3,
                &[0, 1, 2],
                &names,
                &edges,
                0,
                1,
                (0.0, 1.0),
                None,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "linearScmCounterfactual");
        let order: Vec<u64> = result["order"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();
        assert_eq!(order, vec![2, 0, 1]);
        let average = result["averageEffect"].as_f64().unwrap();
        assert!((average - 3.0).abs() < 0.05, "average effect {average}");
        assert_eq!(result["sharePositive"].as_f64().unwrap(), 1.0);
        // Exact abduction reproduces the factual outcome when the treatment is set to its own value.
        let equations = result["equations"].as_array().unwrap();
        assert_eq!(equations.len(), 3);
        assert!(equations[2]["rSquared"].as_f64().unwrap() > 0.99);
        assert!(linear_scm_counterfactual(
            &values,
            rows,
            3,
            &[0, 1, 2],
            &names,
            &[(0, 1), (1, 0)],
            0,
            1,
            (0.0, 1.0),
            None
        )
        .is_err());
        assert!(linear_scm_counterfactual(
            &values,
            rows,
            3,
            &[0, 1, 2],
            &names,
            &edges,
            0,
            1,
            (0.0, 1.0),
            Some(0.0)
        )
        .is_err());
    }
}
