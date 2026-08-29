//! Back-door identification over the study graph.

use super::*;

pub(crate) fn backdoor_identification(
    nodes: usize,
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
) -> Result<AnalysisResult, String> {
    if nodes < 2 {
        return Err("backdoor identification needs at least two nodes".to_owned());
    }
    if treatment >= nodes || outcome >= nodes {
        return Err(
            "backdoor identification treatment and outcome must name graph nodes".to_owned(),
        );
    }
    if treatment == outcome {
        return Err("backdoor identification needs distinct treatment and outcome".to_owned());
    }
    if edges
        .iter()
        .any(|&(a, b)| a >= nodes || b >= nodes || a == b)
    {
        return Err("backdoor identification edges must join two distinct graph nodes".to_owned());
    }
    if unobserved
        .iter()
        .any(|&u| u >= nodes || u == treatment || u == outcome)
    {
        return Err("backdoor identification unobserved nodes must be graph nodes other than the treatment and outcome".to_owned());
    }
    let dag = Dag::new(nodes, edges);
    let result = dagitty_adjustment_sets(
        &dag,
        treatment,
        outcome,
        unobserved,
        MAX_MINIMAL_ADJUSTMENT_SETS,
    )
    .map_err(|error| format!("backdoor adjustment-set enumeration failed: {error:?}"))?;
    let result = match result {
        AdjustmentSetAnalysis::Identified {
            canonical,
            minimal,
            truncated,
        } => BackdoorAdjustmentSetEvidence::Identified {
            canonical_set: canonical,
            minimal_sets: minimal,
            truncated,
        },
        AdjustmentSetAnalysis::NotIdentified => BackdoorAdjustmentSetEvidence::NotIdentified,
    };
    Ok(AnalysisResult::BackdoorIdentification {
        nodes,
        treatment,
        outcome,
        unobserved: unobserved.to_vec(),
        result,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backdoor_identification_reports_unobserved_refusal_and_observed_sets() {
        let refused = backdoor_identification(3, &[(2, 0), (2, 1), (0, 1)], 0, 1, &[2])
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("latent confounder still produces a result");
        let value: serde_json::Value = serde_json::from_str(&refused).unwrap();
        assert_eq!(value["kind"], "backdoorIdentification");
        assert_eq!(value["result"]["kind"], "notIdentified");

        let identified = backdoor_identification(3, &[(2, 0), (2, 1), (0, 1)], 0, 1, &[]).unwrap();
        let value: serde_json::Value = serde_json::to_value(identified).unwrap();
        assert_eq!(value["result"]["kind"], "identified");
        assert_eq!(value["result"]["canonicalSet"], serde_json::json!([2]));
        assert_eq!(value["result"]["minimalSets"], serde_json::json!([[2]]));

        assert!(backdoor_identification(3, &[(0, 0)], 0, 1, &[]).is_err());
        assert!(backdoor_identification(3, &[], 0, 0, &[]).is_err());
    }
}
