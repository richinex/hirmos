use crate::gradient_boosting::{GradientBoostingClassifier, Options};
use crate::model_selection::{grid_search, stratified_shuffle_split, Candidate, Grid, SearchError};

#[derive(Clone, Debug)]
pub struct CrossFitted {
    /// Rows in the order the halves are concatenated, not the source order.
    pub rows: Vec<usize>,
    pub scores: Vec<f64>,
    pub selected: Vec<Candidate>,
}

#[derive(Clone, Debug)]
pub struct TreatmentEffect {
    pub ate: f64,
    pub rows: Vec<usize>,
    pub effects: Vec<f64>,
    pub selected: Vec<Candidate>,
}

fn take(design: &[Vec<f64>], rows: &[usize]) -> Vec<Vec<f64>> {
    rows.iter().map(|&row| design[row].clone()).collect()
}

fn take_values(values: &[f64], rows: &[usize]) -> Vec<f64> {
    rows.iter().map(|&row| values[row]).collect()
}

fn fit_selected(
    design: &[Vec<f64>],
    outcome: &[f64],
    grid: &Grid,
    splits: usize,
    min_samples_leaf: usize,
    min_samples_split: usize,
    seed: u32,
) -> Result<(GradientBoostingClassifier, Candidate), SearchError> {
    let search = grid_search(
        design,
        outcome,
        grid,
        splits,
        min_samples_leaf,
        min_samples_split,
        seed,
    )?;
    let model = GradientBoostingClassifier::fit(
        design,
        outcome,
        &Options {
            n_estimators: search.best.n_estimators,
            learning_rate: search.best.learning_rate,
            max_depth: search.best.max_depth,
            min_samples_leaf,
            min_samples_split,
            random_state: seed,
        },
    )
    .map_err(SearchError::Fit)?;
    Ok((model, search.best))
}

/// Two-fold cross-fitting: a model selected and fitted on one half scores the other, so no row is
/// scored by a model that saw it.
pub fn cross_fitted_propensity(
    design: &[Vec<f64>],
    treatment: &[f64],
    grid: &Grid,
    splits: usize,
    min_samples_leaf: usize,
    min_samples_split: usize,
    seed: u32,
) -> Result<CrossFitted, SearchError> {
    let n = design.len();
    if n == 0 {
        return Err(SearchError::EmptySample);
    }
    if treatment.len() != n {
        return Err(SearchError::RowMismatch);
    }
    let (first, second) = stratified_shuffle_split(treatment, 0.5, seed)?;

    let (first_model, first_choice) = fit_selected(
        &take(design, &first), &take_values(treatment, &first),
        grid, splits, min_samples_leaf, min_samples_split, seed,
    )?;
    let (second_model, second_choice) = fit_selected(
        &take(design, &second), &take_values(treatment, &second),
        grid, splits, min_samples_leaf, min_samples_split, seed,
    )?;

    let mut scores = second_model
        .predict_probability(&take(design, &first))
        .map_err(SearchError::Fit)?;
    scores.extend(
        first_model
            .predict_probability(&take(design, &second))
            .map_err(SearchError::Fit)?,
    );
    let mut rows = first;
    rows.extend(second);
    Ok(CrossFitted {
        rows,
        scores,
        selected: vec![first_choice, second_choice],
    })
}

/// A T-learner over the same two halves: within each half one model per arm, and the opposite
/// half's pair supplies every row's predicted difference.
pub fn cross_fitted_tlearner(
    design: &[Vec<f64>],
    treatment: &[f64],
    outcome: &[f64],
    grid: &Grid,
    splits: usize,
    min_samples_leaf: usize,
    min_samples_split: usize,
    seed: u32,
) -> Result<TreatmentEffect, SearchError> {
    let n = design.len();
    if n == 0 {
        return Err(SearchError::EmptySample);
    }
    if treatment.len() != n || outcome.len() != n {
        return Err(SearchError::RowMismatch);
    }
    let (first, second) = stratified_shuffle_split(outcome, 0.5, seed)?;

    let arm_models = |half: &[usize]| -> Result<
        (GradientBoostingClassifier, GradientBoostingClassifier, Vec<Candidate>),
        SearchError,
    > {
        let treated: Vec<usize> = half.iter().copied().filter(|&row| treatment[row] == 1.0).collect();
        let control: Vec<usize> = half.iter().copied().filter(|&row| treatment[row] == 0.0).collect();
        if treated.is_empty() || control.is_empty() {
            return Err(SearchError::ClassTooSmall {
                encoded: usize::from(treated.is_empty()),
                count: 0,
            });
        }
        let (treated_model, treated_choice) = fit_selected(
            &take(design, &treated), &take_values(outcome, &treated),
            grid, splits, min_samples_leaf, min_samples_split, seed,
        )?;
        let (control_model, control_choice) = fit_selected(
            &take(design, &control), &take_values(outcome, &control),
            grid, splits, min_samples_leaf, min_samples_split, seed,
        )?;
        Ok((treated_model, control_model, vec![treated_choice, control_choice]))
    };

    let (first_treated, first_control, first_choices) = arm_models(&first)?;
    let (second_treated, second_control, second_choices) = arm_models(&second)?;

    let difference = |treated: &GradientBoostingClassifier,
                      control: &GradientBoostingClassifier,
                      rows: &[usize]|
     -> Result<Vec<f64>, SearchError> {
        let held = take(design, rows);
        let with = treated.predict_probability(&held).map_err(SearchError::Fit)?;
        let without = control.predict_probability(&held).map_err(SearchError::Fit)?;
        Ok(with.iter().zip(&without).map(|(a, b)| a - b).collect())
    };

    let mut effects = difference(&second_treated, &second_control, &first)?;
    effects.extend(difference(&first_treated, &first_control, &second)?);
    let mut rows = first;
    rows.extend(second);

    let mut selected = first_choices;
    selected.extend(second_choices);
    Ok(TreatmentEffect {
        ate: crate::numpy_reduce::numpy_mean(&effects),
        rows,
        effects,
        selected,
    })
}
