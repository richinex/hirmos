//! Back-door identification over the study graph.

use super::*;

pub(crate) fn backdoor_identification(
    nodes: usize,
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
    estimand: IdentificationEstimand,
) -> Result<AnalysisResult, String> {
    if nodes < 2 {
        return Err("backdoor identification needs at least two nodes".to_owned());
    }
    if names.len() != nodes
        || names.iter().any(|name| name.trim().is_empty())
        || names
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != nodes
    {
        return Err(
            "graphical identification needs one distinct, non-empty name per node".to_owned(),
        );
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
    let frontdoor = identify_frontdoor_set(&dag, treatment, outcome, unobserved)
        .map_err(|error| format!("front-door identification failed: {error}"))?
        .map_or(FrontdoorSetEvidence::NotIdentified, |mediators| {
            FrontdoorSetEvidence::Identified { mediators }
        });
    let named_edges = edges
        .iter()
        .map(|&(source, target)| (names[source].clone(), names[target].clone()))
        .collect::<Vec<_>>();
    let latent_names = unobserved
        .iter()
        .map(|&index| names[index].clone())
        .collect::<Vec<_>>();
    let projection = latent_projection(names.to_vec(), named_edges, latent_names)
        .map_err(|error| format!("latent projection failed: {error}"))?;
    let name_to_index = names
        .iter()
        .enumerate()
        .map(|(index, name)| (name.as_str(), index))
        .collect::<std::collections::BTreeMap<_, _>>();
    let projection_evidence = || {
        let mut directed_edges = projection
            .directed_edges()
            .iter()
            .map(|(source, target)| {
                (
                    name_to_index[source.as_str()],
                    name_to_index[target.as_str()],
                )
            })
            .collect::<Vec<_>>();
        let mut bidirected_edges = projection
            .bidirected_edges()
            .iter()
            .map(|(left, right)| (name_to_index[left.as_str()], name_to_index[right.as_str()]))
            .collect::<Vec<_>>();
        directed_edges.sort_unstable();
        bidirected_edges.sort_unstable();
        LatentProjectionEvidence {
            directed_edges,
            bidirected_edges,
        }
    };
    let graphical_identification = match identify_outcomes(
        &projection,
        [names[treatment].clone()],
        [names[outcome].clone()],
    ) {
        Ok(expression) => GraphicalIdentificationEvidence::Identified {
            latex: expression.to_latex(),
            expression: expression.to_y0(),
            projection: projection_evidence(),
        },
        Err(IdentificationError::Unidentifiable(hedge)) => {
            GraphicalIdentificationEvidence::Unidentifiable {
                hedge_graph: hedge
                    .graph_district
                    .iter()
                    .map(|name| name_to_index[name.as_str()])
                    .collect(),
                hedge_subgraph: hedge
                    .treatment_removed_district
                    .iter()
                    .map(|name| name_to_index[name.as_str()])
                    .collect(),
                projection: projection_evidence(),
            }
        }
        Err(error) => return Err(format!("graphical identification failed: {error}")),
    };
    let counterfactual_identification = match estimand {
        IdentificationEstimand::Ate => CounterfactualIdentificationEvidence::NotApplicable,
        IdentificationEstimand::Att => {
            match identify_binary_ett(&projection, &names[treatment], &names[outcome]) {
                Ok(identified) => CounterfactualIdentificationEvidence::Identified {
                    treated_expression: identified
                        .treated_query
                        .identified_to_y0(&identified.treated_expression),
                    untreated_expression: identified
                        .untreated_query
                        .identified_to_y0(&identified.untreated_expression),
                },
                Err(error) => CounterfactualIdentificationEvidence::Unidentifiable {
                    reason: error.to_string(),
                },
            }
        }
    };
    Ok(AnalysisResult::BackdoorIdentification {
        nodes,
        treatment,
        outcome,
        unobserved: unobserved.to_vec(),
        result,
        frontdoor,
        graphical_identification,
        counterfactual_identification,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backdoor_identification_reports_unobserved_refusal_and_observed_sets() {
        let names = vec!["X".to_owned(), "Y".to_owned(), "U".to_owned()];
        let refused = backdoor_identification(
            3,
            &names,
            &[(2, 0), (2, 1), (0, 1)],
            0,
            1,
            &[2],
            IdentificationEstimand::Ate,
        )
        .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
        .expect("latent confounder still produces a result");
        let value: serde_json::Value = serde_json::from_str(&refused).unwrap();
        assert_eq!(value["kind"], "backdoorIdentification");
        assert_eq!(value["result"]["kind"], "notIdentified");
        assert_eq!(value["frontdoor"]["kind"], "notIdentified");
        assert_eq!(value["graphicalIdentification"]["kind"], "unidentifiable");

        let identified = backdoor_identification(
            3,
            &names,
            &[(2, 0), (2, 1), (0, 1)],
            0,
            1,
            &[],
            IdentificationEstimand::Ate,
        )
        .unwrap();
        let value: serde_json::Value = serde_json::to_value(identified).unwrap();
        assert_eq!(value["result"]["kind"], "identified");
        assert_eq!(value["result"]["canonicalSet"], serde_json::json!([2]));
        assert_eq!(value["result"]["minimalSets"], serde_json::json!([[2]]));
        assert_eq!(value["graphicalIdentification"]["kind"], "identified");
        assert_eq!(value["frontdoor"]["kind"], "notIdentified");

        assert!(backdoor_identification(
            3,
            &names,
            &[(0, 0)],
            0,
            1,
            &[],
            IdentificationEstimand::Ate,
        )
        .is_err());
        assert!(
            backdoor_identification(3, &names, &[], 0, 0, &[], IdentificationEstimand::Ate,)
                .is_err()
        );
    }

    #[test]
    fn frontdoor_is_identified_after_latent_projection() {
        let names = vec![
            "X".to_owned(),
            "M".to_owned(),
            "Y".to_owned(),
            "U".to_owned(),
        ];
        let value = serde_json::to_value(
            backdoor_identification(
                4,
                &names,
                &[(3, 0), (3, 2), (0, 1), (1, 2)],
                0,
                2,
                &[3],
                IdentificationEstimand::Ate,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(value["result"]["kind"], "notIdentified");
        assert_eq!(value["frontdoor"]["kind"], "identified");
        assert_eq!(value["frontdoor"]["mediators"], serde_json::json!([1]));
        assert_eq!(value["graphicalIdentification"]["kind"], "identified");
        assert_eq!(
            value["graphicalIdentification"]["expression"],
            "Sum[M](P(M | X) * Sum[X](P(X) * P(Y | M, X)))"
        );
        assert_eq!(
            value["graphicalIdentification"]["projection"]["bidirectedEdges"],
            serde_json::json!([[0, 2]])
        );

        let att = serde_json::to_value(
            backdoor_identification(
                4,
                &names,
                &[(3, 0), (3, 2), (0, 1), (1, 2)],
                0,
                2,
                &[3],
                IdentificationEstimand::Att,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(att["counterfactualIdentification"]["kind"], "identified");
    }

    #[test]
    fn att_identification_records_both_idc_star_expressions() {
        let names = vec!["Z".to_owned(), "X".to_owned(), "Y".to_owned()];
        let value = serde_json::to_value(
            backdoor_identification(
                3,
                &names,
                &[(0, 1), (0, 2), (1, 2)],
                1,
                2,
                &[],
                IdentificationEstimand::Att,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(value["counterfactualIdentification"]["kind"], "identified");
        assert!(value["counterfactualIdentification"]["treatedExpression"]
            .as_str()
            .is_some_and(|expression| !expression.is_empty()));
        assert!(value["counterfactualIdentification"]["untreatedExpression"]
            .as_str()
            .is_some_and(|expression| !expression.is_empty()));
        assert_ne!(
            value["counterfactualIdentification"]["treatedExpression"],
            value["counterfactualIdentification"]["untreatedExpression"]
        );
    }
}
