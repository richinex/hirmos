//! Identified-function estimators: the front door, instrumental variables, and the discrete queries the ID algorithms identify.

use super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn frontdoor_two_stage_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    mediator: usize,
    outcome: usize,
    first_stage_adjustment: &[usize],
    second_stage_adjustment: &[usize],
    control_value: f64,
    treatment_value: f64,
    uncertainty: BootstrapUncertainty,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    validate_dense_matrix("front-door two-stage estimate", values, rows, columns)?;
    let endpoints = [treatment, mediator, outcome];
    if endpoints.iter().any(|&column| column >= columns)
        || endpoints
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != endpoints.len()
    {
        return Err(
            "front-door estimation needs distinct treatment, mediator, and outcome columns"
                .to_owned(),
        );
    }
    let valid_adjustment = |stage: &str, excluded: &[usize], adjustment: &[usize]| {
        if adjustment.iter().any(|&column| column >= columns) {
            return Err(format!(
                "front-door {stage} adjustment columns must index the numeric matrix"
            ));
        }
        let distinct = adjustment
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if distinct.len() != adjustment.len()
            || excluded.iter().any(|column| distinct.contains(column))
        {
            return Err(format!(
                "front-door {stage} adjustment columns must be distinct and exclude its treatment, response, and outcome columns"
            ));
        }
        Ok(())
    };
    valid_adjustment(
        "first-stage",
        &[treatment, mediator, outcome],
        first_stage_adjustment,
    )?;
    valid_adjustment(
        "second-stage",
        &[mediator, outcome],
        second_stage_adjustment,
    )?;

    let data = DMatrix::from_column_slice(rows, columns, values);
    let column = |index: usize| (0..rows).map(|row| data[(row, index)]).collect::<Vec<_>>();
    let treatment_values = column(treatment);
    let mediator_values = column(mediator);
    let outcome_values = column(outcome);
    let adjustment_matrix = |selected: &[usize]| {
        (!selected.is_empty()).then(|| {
            DMatrix::from_fn(rows, selected.len(), |row, column| {
                data[(row, selected[column])]
            })
        })
    };
    let first_adjustment = adjustment_matrix(first_stage_adjustment);
    let second_adjustment = adjustment_matrix(second_stage_adjustment);
    let bootstrap = bootstrap_request(uncertainty);
    let result = frontdoor_two_stage_with_progress(
        FrontdoorInput {
            treatment: &treatment_values,
            mediator: &mediator_values,
            outcome: &outcome_values,
            first_stage_adjustment: first_adjustment.as_ref(),
            second_stage_adjustment: second_adjustment.as_ref(),
        },
        FrontdoorOptions {
            control_value,
            treatment_value,
            bootstrap,
        },
        |completed, total| progress("frontdoor-bootstrap", completed, total),
    )
    .map_err(|error| error.to_string())?;
    let uncertainty = bootstrap_evidence(uncertainty, result.confidence_interval)?;
    Ok(AnalysisResult::FrontdoorTwoStage {
        observations: rows,
        treatment,
        mediator,
        outcome,
        first_stage_adjustment: first_stage_adjustment.to_vec(),
        second_stage_adjustment: second_stage_adjustment.to_vec(),
        control_value,
        treatment_value,
        first_stage_params: result.first_stage_params,
        second_stage_params: result.second_stage_params,
        first_stage_effect: result.first_stage_effect,
        second_stage_effect: result.second_stage_effect,
        estimate: result.ate,
        uncertainty,
    })
}

fn bootstrap_request(uncertainty: BootstrapUncertainty) -> Option<DowhyBootstrap> {
    match uncertainty {
        BootstrapUncertainty::None => None,
        BootstrapUncertainty::Bootstrap {
            simulations,
            sample_size_fraction,
            confidence_level,
            seed,
        } => Some(DowhyBootstrap {
            simulations,
            sample_size_fraction,
            confidence_level,
            seed,
        }),
    }
}

fn bootstrap_evidence(
    uncertainty: BootstrapUncertainty,
    interval: Option<[f64; 2]>,
) -> Result<BootstrapUncertaintyEvidence, String> {
    match (uncertainty, interval) {
        (BootstrapUncertainty::None, None) => Ok(BootstrapUncertaintyEvidence::None),
        (
            BootstrapUncertainty::Bootstrap {
                simulations,
                sample_size_fraction,
                confidence_level,
                seed,
            },
            Some(interval),
        ) => Ok(BootstrapUncertaintyEvidence::Bootstrap {
            simulations,
            sample_size_fraction,
            confidence_level,
            seed,
            interval,
        }),
        _ => Err("bootstrap uncertainty result did not match its request".to_owned()),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn instrumental_variable_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    instruments: &[usize],
    uncertainty: BootstrapUncertainty,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    validate_dense_matrix("instrumental-variable estimate", values, rows, columns)?;
    let mut selected = vec![treatment, outcome];
    selected.extend_from_slice(instruments);
    if selected.iter().any(|&column| column >= columns) {
        return Err("instrumental-variable columns must index the numeric matrix".to_owned());
    }
    if selected
        .iter()
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != selected.len()
    {
        return Err(
            "instrumental-variable estimation needs distinct treatment, outcome, and instrument columns"
                .to_owned(),
        );
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let column_matrix = |selected: &[usize]| {
        DMatrix::from_fn(rows, selected.len(), |row, column| {
            data[(row, selected[column])]
        })
    };
    let treatment_matrix = column_matrix(&[treatment]);
    let instrument_matrix = column_matrix(instruments);
    let outcome_values = (0..rows)
        .map(|row| data[(row, outcome)])
        .collect::<Vec<_>>();
    let result = instrumental_variable_with_progress(
        IvInput {
            treatments: &treatment_matrix,
            instruments: &instrument_matrix,
            outcome: &outcome_values,
        },
        IvOptions {
            bootstrap: bootstrap_request(uncertainty),
        },
        |completed, total| progress("instrumental-variable-bootstrap", completed, total),
    )
    .map_err(|error| error.to_string())?;
    let route = match result.estimator {
        IvEstimator::WaldRatio => InstrumentalVariableRoute::WaldRatio,
        IvEstimator::CovarianceRatio => InstrumentalVariableRoute::CovarianceRatio,
        IvEstimator::TwoStageLeastSquares => InstrumentalVariableRoute::TwoStageLeastSquares,
    };
    Ok(AnalysisResult::InstrumentalVariable {
        observations: rows,
        treatment,
        outcome,
        instruments: instruments.to_vec(),
        route,
        estimate: result.estimate,
        params: result.params,
        standard_error: result.standard_error,
        uncertainty: bootstrap_evidence(uncertainty, result.confidence_interval)?,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn binary_ett(
    values: &[f64],
    rows: usize,
    columns: usize,
    observed_nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("binary ETT", values, rows, columns)?;
    let nodes = names.len();
    if nodes < 2
        || names.iter().any(|name| name.trim().is_empty())
        || names
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != nodes
    {
        return Err("binary ETT needs at least two distinctly named graph nodes".to_owned());
    }
    if treatment >= nodes || outcome >= nodes || treatment == outcome {
        return Err("binary ETT needs distinct observed treatment and outcome nodes".to_owned());
    }
    if edges
        .iter()
        .any(|&(source, target)| source >= nodes || target >= nodes || source == target)
    {
        return Err("binary ETT edges must join two distinct graph nodes".to_owned());
    }
    let observed = observed_nodes
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let hidden = unobserved
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if observed_nodes.len() != columns
        || observed.len() != observed_nodes.len()
        || hidden.len() != unobserved.len()
        || observed
            .iter()
            .chain(hidden.iter())
            .any(|&node| node >= nodes)
        || !observed.is_disjoint(&hidden)
        || observed.len() + hidden.len() != nodes
        || !observed.contains(&treatment)
        || !observed.contains(&outcome)
    {
        return Err("binary ETT needs one matrix column for every observed node and an exact partition of observed and unobserved graph nodes".to_owned());
    }
    let named_edges = edges
        .iter()
        .map(|&(source, target)| (names[source].clone(), names[target].clone()))
        .collect::<Vec<_>>();
    let hidden_names = unobserved
        .iter()
        .map(|&node| names[node].clone())
        .collect::<Vec<_>>();
    let graph = latent_projection(names.to_vec(), named_edges, hidden_names)
        .map_err(|error| format!("binary ETT latent projection failed: {error}"))?;
    let mut table_columns = BTreeMap::new();
    for (column, &node) in observed_nodes.iter().enumerate() {
        let mut states = Vec::with_capacity(rows);
        for row in 0..rows {
            let value = values[column * rows + row];
            if value != 0.0 && value != 1.0 {
                return Err(format!(
                    "binary ETT requires every observed graph variable to contain only 0 and 1; {} has {value}",
                    names[node]
                ));
            }
            states.push(if value == 0.0 { "0" } else { "1" }.to_owned());
        }
        table_columns.insert(names[node].clone(), states);
    }
    let table = DiscreteTable::from_columns(table_columns)
        .map_err(|error| format!("binary ETT table failed: {error}"))?;
    let estimate = estimate_binary_ett(&graph, &table, &names[treatment], &names[outcome])
        .map_err(|error| format!("binary ETT failed: {error}"))?;
    Ok(AnalysisResult::BinaryEtt {
        observations: rows,
        treatment,
        outcome,
        treated_potential_outcome_mean: estimate.treated_potential_outcome_mean,
        untreated_potential_outcome_mean: estimate.untreated_potential_outcome_mean,
        effect_on_treated: estimate.effect_on_treated,
        treated_expression: estimate
            .treated_query
            .identified_to_y0(&estimate.treated_expression),
        untreated_expression: estimate
            .untreated_query
            .identified_to_y0(&estimate.untreated_expression),
    })
}

fn discrete_state_strategy(strategy: CoreDiscreteStateStrategy) -> DiscreteStateStrategyEvidence {
    match strategy {
        CoreDiscreteStateStrategy::ObservedStates { states } => {
            DiscreteStateStrategyEvidence::ObservedStates { states }
        }
        CoreDiscreteStateStrategy::Quantiles {
            requested,
            populated,
        } => DiscreteStateStrategyEvidence::Quantiles {
            requested,
            populated,
        },
    }
}

fn discrete_state_problem(problem: CoreDiscreteStateProblem) -> DiscreteStateProblemEvidence {
    match problem {
        CoreDiscreteStateProblem::NoFiniteObservations { observations } => {
            DiscreteStateProblemEvidence::NoFiniteObservations { observations }
        }
        CoreDiscreteStateProblem::SingleObservedState {
            value,
            observations,
        } => DiscreteStateProblemEvidence::SingleObservedState {
            value,
            observations,
        },
        CoreDiscreteStateProblem::QuantileCollapse {
            distinct_values,
            requested_states,
            populated_states,
        } => DiscreteStateProblemEvidence::QuantileCollapse {
            distinct_values,
            requested_states,
            populated_states,
        },
    }
}

fn discrete_state_refusal(
    query: DiscreteStateQuery,
    node: usize,
    name: String,
    problem: CoreDiscreteStateProblem,
) -> AnalysisResult {
    AnalysisResult::DiscreteStateRefused {
        query,
        node,
        name,
        problem: discrete_state_problem(problem),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn discrete_bn_query(
    values: &[f64],
    rows: usize,
    columns: usize,
    nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    bins: usize,
    equivalent_sample_size: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("discrete BN query", values, rows, columns)?;
    if nodes.len() < 2
        || nodes.len() != names.len()
        || nodes.iter().any(|&column| column >= columns)
    {
        return Err(
            "discrete BN query needs at least two nodes, each with a name and a matrix column"
                .to_owned(),
        );
    }
    if treatment >= nodes.len() || outcome >= nodes.len() || treatment == outcome {
        return Err(
            "discrete BN query treatment and outcome must be distinct node positions".to_owned(),
        );
    }
    if edges
        .iter()
        .any(|&(cause, effect)| cause >= nodes.len() || effect >= nodes.len() || cause == effect)
    {
        return Err("discrete BN query edges must join distinct node positions".to_owned());
    }
    if !(2..=10).contains(&bins) {
        return Err("discrete BN query state budget must be between 2 and 10".to_owned());
    }
    if !(equivalent_sample_size > 0.0) || !equivalent_sample_size.is_finite() {
        return Err("discrete BN query equivalent sample size must be positive".to_owned());
    }
    let state_budget = StateBudget::try_from(bins)
        .map_err(|_| "discrete BN query state budget must be at least 2".to_owned())?;
    let data = DMatrix::from_column_slice(rows, columns, values);
    let mut frame: HashMap<String, Vec<String>> = HashMap::new();
    let mut means: Vec<std::collections::BTreeMap<String, f64>> = Vec::with_capacity(nodes.len());
    let mut state_counts = Vec::with_capacity(nodes.len());
    let mut state_preparations = Vec::with_capacity(nodes.len());
    for (position, &column) in nodes.iter().enumerate() {
        let series: Vec<f64> = (0..rows).map(|row| data[(row, column)]).collect();
        let discretised = match discretize_for_discrete_bn(&series, state_budget) {
            Ok(discretised) => discretised,
            Err(problem) => {
                return Ok(discrete_state_refusal(
                    DiscreteStateQuery::BayesianNetwork,
                    position,
                    names[position].clone(),
                    problem,
                ))
            }
        };
        state_counts.push(discretised.means.len());
        state_preparations.push(DiscreteStatePreparationEvidence {
            node: position,
            name: names[position].clone(),
            strategy: discrete_state_strategy(discretised.strategy),
        });
        frame.insert(names[position].clone(), discretised.labels);
        means.push(discretised.means);
    }
    let named_edges: Vec<(String, String)> = edges
        .iter()
        .map(|&(cause, effect)| (names[cause].clone(), names[effect].clone()))
        .collect();
    let dag = DiscreteDag::new(&named_edges, names);
    let bn = DiscreteBn::fit(dag, &frame, equivalent_sample_size);
    let treatment_name = &names[treatment];
    let outcome_name = &names[outcome];
    let treatment_states = &bn.state_names[treatment_name];
    let low = treatment_states
        .first()
        .cloned()
        .ok_or("discrete BN query treatment has no states")?;
    let high = treatment_states
        .last()
        .cloned()
        .ok_or("discrete BN query treatment has no states")?;
    if low == high {
        return Err(format!(
            "discrete BN query treatment {treatment_name} has a single state after discretisation"
        ));
    }
    let outcome_means = &means[outcome];
    let expectation = |distribution: &HashMap<String, f64>| -> f64 {
        distribution
            .iter()
            .map(|(state, probability)| {
                outcome_means.get(state).copied().unwrap_or(0.0) * probability
            })
            .sum()
    };
    let ordered = |distribution: HashMap<String, f64>| -> Vec<(String, f64)> {
        let mut entries: Vec<(String, f64)> = distribution.into_iter().collect();
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        entries
    };
    let distribution_low = bn.do_query(outcome_name, treatment_name, &low);
    let distribution_high = bn.do_query(outcome_name, treatment_name, &high);
    let expectations = (
        expectation(&distribution_low),
        expectation(&distribution_high),
    );
    let mut parents_adjusted = bn.dag.parents(treatment_name);
    parents_adjusted.sort();
    Ok(AnalysisResult::DiscreteBnQuery {
        observations: rows,
        bins,
        equivalent_sample_size,
        state_counts,
        state_preparations,
        treatment_states: (low, high),
        expectations,
        effect: expectations.1 - expectations.0,
        distribution_low: ordered(distribution_low),
        distribution_high: ordered(distribution_high),
        minimal_adjustment_set: bn
            .minimal_adjustment_set(treatment_name, outcome_name)
            .map(|set| set.into_iter().collect()),
        parents_adjusted,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn identified_discrete_query(
    values: &[f64],
    rows: usize,
    columns: usize,
    observed_nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
    bins: usize,
    condition: Option<IdentifiedDiscreteCondition>,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("identified discrete query", values, rows, columns)?;
    if names.len() < 2 || !(2..=10).contains(&bins) {
        return Err(
            "identified discrete query needs at least two graph nodes and a state budget between 2 and 10"
                .to_owned(),
        );
    }
    let state_budget = StateBudget::try_from(bins)
        .map_err(|_| "identified discrete query state budget must be at least 2".to_owned())?;
    if treatment >= names.len() || outcome >= names.len() || treatment == outcome {
        return Err(
            "identified discrete query treatment and outcome must be distinct graph-node positions"
                .to_owned(),
        );
    }
    if edges
        .iter()
        .any(|&(cause, effect)| cause >= names.len() || effect >= names.len() || cause == effect)
        || unobserved.iter().any(|&node| node >= names.len())
    {
        return Err(
            "identified discrete query contains an invalid graph edge or unobserved node"
                .to_owned(),
        );
    }
    let unobserved_set = unobserved
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if unobserved_set.contains(&treatment) || unobserved_set.contains(&outcome) {
        return Err(
            "identified discrete query can only set and read observed variables".to_owned(),
        );
    }
    let observed_positions = (0..names.len())
        .filter(|position| !unobserved_set.contains(position))
        .collect::<Vec<_>>();
    if observed_nodes.len() != observed_positions.len()
        || observed_nodes.iter().any(|&column| column >= columns)
    {
        return Err(
            "identified discrete query needs one matrix column per observed graph node".to_owned(),
        );
    }
    if let Some(condition) = condition {
        if condition.variable >= names.len()
            || condition.variable == treatment
            || condition.variable == outcome
            || unobserved_set.contains(&condition.variable)
        {
            return Err(
                "identified discrete query condition must be a distinct observed variable"
                    .to_owned(),
            );
        }
    }

    let data = DMatrix::from_column_slice(rows, columns, values);
    let mut columns_by_name = BTreeMap::new();
    let mut means_by_position = HashMap::new();
    let mut state_counts = Vec::with_capacity(observed_nodes.len());
    let mut state_preparations = Vec::with_capacity(observed_nodes.len());
    for (&position, &column) in observed_positions.iter().zip(observed_nodes) {
        let series = (0..rows).map(|row| data[(row, column)]).collect::<Vec<_>>();
        let discretised = match discretize_for_discrete_bn(&series, state_budget) {
            Ok(discretised) => discretised,
            Err(problem) => {
                return Ok(discrete_state_refusal(
                    DiscreteStateQuery::IdentifiedExpression,
                    position,
                    names[position].clone(),
                    problem,
                ))
            }
        };
        state_counts.push(discretised.means.len());
        state_preparations.push(DiscreteStatePreparationEvidence {
            node: position,
            name: names[position].clone(),
            strategy: discrete_state_strategy(discretised.strategy),
        });
        means_by_position.insert(position, discretised.means);
        columns_by_name.insert(names[position].clone(), discretised.labels);
    }
    let table = DiscreteTable::from_columns(columns_by_name).map_err(|error| {
        format!("identified discrete query could not form its observational table: {error}")
    })?;
    let named_edges = edges
        .iter()
        .map(|&(cause, effect)| (names[cause].clone(), names[effect].clone()))
        .collect::<Vec<_>>();
    let latent_names = unobserved
        .iter()
        .map(|&position| names[position].clone())
        .collect::<Vec<_>>();
    let projection = latent_projection(names.iter().cloned(), named_edges, latent_names)
        .map_err(|error| format!("identified discrete query graph is invalid: {error:?}"))?;
    let treatment_name = names[treatment].clone();
    let outcome_name = names[outcome].clone();
    let treatment_states = table.states(&treatment_name).ok_or_else(|| {
        "identified discrete query treatment is absent from the observed table".to_owned()
    })?;
    let low = treatment_states
        .first()
        .cloned()
        .ok_or_else(|| "identified discrete query treatment has no states".to_owned())?;
    let high = treatment_states
        .last()
        .cloned()
        .ok_or_else(|| "identified discrete query treatment has no states".to_owned())?;
    if low == high {
        return Err(format!("identified discrete query treatment {treatment_name} has a single state after discretisation"));
    }

    let (query, condition_name, condition_state) = match condition {
        None => (IdentifiedDiscreteQueryKind::Unconditional, None, None),
        Some(condition) => {
            let name = names[condition.variable].clone();
            let state = condition.state.to_string();
            let means = &means_by_position[&condition.variable];
            let representative_value = means.get(&state).copied().ok_or_else(|| {
                format!("identified discrete query condition state {state} is absent for {name} after discretisation")
            })?;
            (
                IdentifiedDiscreteQueryKind::Conditional {
                    variable: condition.variable,
                    state: state.clone(),
                    representative_value,
                },
                Some(name),
                Some(state),
            )
        }
    };
    let expression = match &condition_name {
        None => identify_outcomes(
            &projection,
            [treatment_name.clone()],
            [outcome_name.clone()],
        ),
        Some(name) => identify_conditional_outcomes(
            &projection,
            [treatment_name.clone()],
            [outcome_name.clone()],
            [name.clone()],
        ),
    };
    let expression = match expression {
        Ok(expression) => expression,
        Err(IdentificationError::Unidentifiable(hedge)) => {
            let index_by_name = names
                .iter()
                .enumerate()
                .map(|(index, name)| (name.as_str(), index))
                .collect::<HashMap<_, _>>();
            return Ok(AnalysisResult::IdentifiedDiscreteQuery {
                observations: rows,
                bins,
                state_counts,
                state_preparations,
                treatment_states: (low, high),
                query,
                result: IdentifiedDiscreteResult::Unidentifiable {
                    hedge_graph: hedge
                        .graph_district
                        .iter()
                        .map(|name| index_by_name[name.as_str()])
                        .collect(),
                    hedge_subgraph: hedge
                        .treatment_removed_district
                        .iter()
                        .map(|name| index_by_name[name.as_str()])
                        .collect(),
                },
            });
        }
        Err(error) => {
            return Err(format!(
                "identified discrete query could not identify its expression: {error}"
            ))
        }
    };
    let fixed = |treatment_state: String| {
        let mut assignment = BTreeMap::from([(treatment_name.clone(), treatment_state)]);
        if let (Some(name), Some(state)) = (&condition_name, &condition_state) {
            assignment.insert(name.clone(), state.clone());
        }
        assignment
    };
    let low_distribution = evaluate_distribution(
        &expression,
        &table,
        [outcome_name.clone()],
        &fixed(low.clone()),
    )
    .map_err(|error| {
        format!("identified discrete query could not evaluate the low intervention: {error}")
    })?;
    let high_distribution = evaluate_distribution(
        &expression,
        &table,
        [outcome_name.clone()],
        &fixed(high.clone()),
    )
    .map_err(|error| {
        format!("identified discrete query could not evaluate the high intervention: {error}")
    })?;
    let ordered =
        |distribution: &hirmos_causal_core::identified_expression::EvaluatedDistribution| {
            distribution
                .probabilities
                .iter()
                .map(|(states, probability)| (states[0].clone(), *probability))
                .collect::<Vec<_>>()
        };
    let outcome_means = &means_by_position[&outcome];
    let expectation =
        |distribution: &hirmos_causal_core::identified_expression::EvaluatedDistribution| {
            distribution
                .probabilities
                .iter()
                .map(|(states, probability)| outcome_means[&states[0]] * probability)
                .sum::<f64>()
        };
    let expectations = (
        expectation(&low_distribution),
        expectation(&high_distribution),
    );
    Ok(AnalysisResult::IdentifiedDiscreteQuery {
        observations: rows,
        bins,
        state_counts,
        state_preparations,
        treatment_states: (low, high),
        query,
        result: IdentifiedDiscreteResult::Identified {
            algorithm: if condition_name.is_some() {
                "IDC"
            } else {
                "ID"
            },
            expression: expression.to_y0(),
            latex: expression.to_latex(),
            expectations,
            effect: expectations.1 - expectations.0,
            distribution_low: ordered(&low_distribution),
            distribution_high: ordered(&high_distribution),
            normalization_low: low_distribution.total,
            normalization_high: high_distribution.total,
        },
    })
}
