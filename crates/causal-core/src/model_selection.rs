use crate::gradient_boosting::{GradientBoostingClassifier, GradientBoostingError, Options};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchError {
    EmptySample,
    RowMismatch,
    EmptyGrid,
    TooFewSplits,
    ClassTooSmall { encoded: usize, count: usize },
    Fit(GradientBoostingError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Candidate {
    pub learning_rate: f64,
    pub max_depth: usize,
    pub n_estimators: usize,
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub learning_rate: Vec<f64>,
    pub max_depth: Vec<usize>,
    pub n_estimators: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct CandidateScore {
    pub candidate: Candidate,
    pub fold_scores: Vec<f64>,
    pub mean_score: f64,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub best: Candidate,
    pub best_score: f64,
    pub scores: Vec<CandidateScore>,
    pub fits: usize,
}

/// `StratifiedKFold(shuffle=False)`: classes encoded by order of appearance, then allocated to
/// folds in blocks so the original row order survives within each class.
pub fn stratified_k_fold(outcome: &[f64], splits: usize) -> Result<Vec<Vec<usize>>, SearchError> {
    let n = outcome.len();
    if n == 0 {
        return Err(SearchError::EmptySample);
    }
    if splits < 2 {
        return Err(SearchError::TooFewSplits);
    }
    let appearance: Vec<f64> = {
        let mut seen: Vec<f64> = Vec::new();
        for value in outcome {
            if !seen.iter().any(|held| held == value) {
                seen.push(*value);
            }
        }
        seen
    };
    let encoded: Vec<usize> = outcome
        .iter()
        .map(|value| appearance.iter().position(|held| held == value).unwrap())
        .collect();
    let classes = appearance.len();
    let mut counts = vec![0usize; classes];
    for &class in &encoded {
        counts[class] += 1;
    }
    if let Some(class) = counts.iter().position(|&count| count < splits) {
        return Err(SearchError::ClassTooSmall {
            encoded: class,
            count: counts[class],
        });
    }

    let mut order: Vec<usize> = encoded.clone();
    order.sort_unstable();
    let mut allocation = vec![vec![0usize; classes]; splits];
    for fold in 0..splits {
        for position in (fold..n).step_by(splits) {
            allocation[fold][order[position]] += 1;
        }
    }

    let mut test_fold = vec![0usize; n];
    for class in 0..classes {
        let mut folds_for_class = Vec::with_capacity(counts[class]);
        for fold in 0..splits {
            folds_for_class.extend(std::iter::repeat_n(fold, allocation[fold][class]));
        }
        let mut next = 0;
        for row in 0..n {
            if encoded[row] == class {
                test_fold[row] = folds_for_class[next];
                next += 1;
            }
        }
    }

    Ok((0..splits)
        .map(|fold| (0..n).filter(|&row| test_fold[row] == fold).collect())
        .collect())
}

fn approximate_mode(
    class_counts: &[usize],
    draws: usize,
    rng: &mut crate::nprandom::Mt19937,
) -> Vec<usize> {
    let total: usize = class_counts.iter().sum();
    let continuous: Vec<f64> = class_counts
        .iter()
        .map(|&count| count as f64 / total as f64 * draws as f64)
        .collect();
    let mut floored: Vec<f64> = continuous.iter().map(|value| value.floor()).collect();
    let mut needed = draws as i64 - floored.iter().sum::<f64>() as i64;
    if needed > 0 {
        let remainder: Vec<f64> = continuous
            .iter()
            .zip(&floored)
            .map(|(value, floor)| value - floor)
            .collect();
        let mut values: Vec<f64> = remainder.clone();
        values.sort_by(f64::total_cmp);
        values.dedup();
        values.reverse();
        for value in values {
            let places: Vec<usize> = (0..remainder.len())
                .filter(|&index| remainder[index] == value)
                .collect();
            let add_now = places.len().min(needed as usize);
            let drawn = rng.permutation(places.len());
            for &position in drawn.iter().take(add_now) {
                floored[places[position]] += 1.0;
            }
            needed -= add_now as i64;
            if needed == 0 {
                break;
            }
        }
    }
    floored.into_iter().map(|value| value as usize).collect()
}

/// `train_test_split(test_size=…, stratify=y)`, which is `StratifiedShuffleSplit` with one split.
/// Classes come from `np.unique`, so they are in sorted order here, not order of appearance.
pub fn stratified_shuffle_split(
    outcome: &[f64],
    test_fraction: f64,
    seed: u32,
) -> Result<(Vec<usize>, Vec<usize>), SearchError> {
    let n = outcome.len();
    if n == 0 {
        return Err(SearchError::EmptySample);
    }
    if !(0.0..1.0).contains(&test_fraction) || test_fraction <= 0.0 {
        return Err(SearchError::TooFewSplits);
    }
    let test_total = (test_fraction * n as f64).ceil() as usize;
    let train_total = n - test_total;

    let mut classes: Vec<f64> = outcome.to_vec();
    classes.sort_by(f64::total_cmp);
    classes.dedup();
    let encoded: Vec<usize> = outcome
        .iter()
        .map(|value| classes.iter().position(|held| held == value).unwrap())
        .collect();
    let mut class_counts = vec![0usize; classes.len()];
    for &class in &encoded {
        class_counts[class] += 1;
    }
    if let Some(class) = class_counts.iter().position(|&count| count < 2) {
        return Err(SearchError::ClassTooSmall {
            encoded: class,
            count: class_counts[class],
        });
    }
    let class_indices: Vec<Vec<usize>> = (0..classes.len())
        .map(|class| (0..n).filter(|&row| encoded[row] == class).collect())
        .collect();

    let mut rng = crate::nprandom::Mt19937::seeded(seed);
    let train_per_class = approximate_mode(&class_counts, train_total, &mut rng);
    let remaining: Vec<usize> = class_counts
        .iter()
        .zip(&train_per_class)
        .map(|(count, taken)| count - taken)
        .collect();
    let test_per_class = approximate_mode(&remaining, test_total, &mut rng);

    let mut train = Vec::with_capacity(train_total);
    let mut test = Vec::with_capacity(test_total);
    for class in 0..classes.len() {
        let permutation = rng.permutation(class_counts[class]);
        let drawn: Vec<usize> = permutation
            .iter()
            .map(|&position| class_indices[class][position])
            .collect();
        train.extend(&drawn[..train_per_class[class]]);
        test.extend(
            &drawn[train_per_class[class]..train_per_class[class] + test_per_class[class]],
        );
    }
    let mut shuffled_train = train;
    rng.shuffle(&mut shuffled_train);
    let mut shuffled_test = test;
    rng.shuffle(&mut shuffled_test);
    Ok((shuffled_train, shuffled_test))
}

/// Candidates in `ParameterGrid` order: keys sorted, the last axis varying fastest.
pub fn candidates(grid: &Grid) -> Vec<Candidate> {
    let mut out = Vec::new();
    for &learning_rate in &grid.learning_rate {
        for &max_depth in &grid.max_depth {
            for &n_estimators in &grid.n_estimators {
                out.push(Candidate {
                    learning_rate,
                    max_depth,
                    n_estimators,
                });
            }
        }
    }
    out
}

/// One fit per (learning_rate, max_depth) pair at the largest tree count, scored at every smaller
/// count as a prefix, because a prefix of a boosted fit is the shorter fit.
pub fn grid_search(
    design: &[Vec<f64>],
    outcome: &[f64],
    grid: &Grid,
    splits: usize,
    min_samples_leaf: usize,
    min_samples_split: usize,
    random_state: u32,
) -> Result<SearchResult, SearchError> {
    let n = design.len();
    if n == 0 {
        return Err(SearchError::EmptySample);
    }
    if outcome.len() != n {
        return Err(SearchError::RowMismatch);
    }
    if grid.learning_rate.is_empty() || grid.max_depth.is_empty() || grid.n_estimators.is_empty() {
        return Err(SearchError::EmptyGrid);
    }
    let folds = stratified_k_fold(outcome, splits)?;
    let all = candidates(grid);
    let mut trees: Vec<usize> = grid.n_estimators.clone();
    trees.sort_unstable();
    trees.dedup();
    let largest = *trees.last().unwrap();

    let mut fold_scores: std::collections::HashMap<(usize, usize, usize), Vec<f64>> =
        std::collections::HashMap::new();
    let mut fits = 0;

    for test in &folds {
        let mut held = vec![false; n];
        for &row in test {
            held[row] = true;
        }
        let train: Vec<usize> = (0..n).filter(|&row| !held[row]).collect();
        let train_design: Vec<Vec<f64>> = train.iter().map(|&row| design[row].clone()).collect();
        let train_outcome: Vec<f64> = train.iter().map(|&row| outcome[row]).collect();
        let test_design: Vec<Vec<f64>> = test.iter().map(|&row| design[row].clone()).collect();
        let test_outcome: Vec<u8> = test.iter().map(|&row| outcome[row] as u8).collect();

        for (rate_index, &learning_rate) in grid.learning_rate.iter().enumerate() {
            for (depth_index, &max_depth) in grid.max_depth.iter().enumerate() {
                let model = GradientBoostingClassifier::fit(
                    &train_design,
                    &train_outcome,
                    &Options {
                        n_estimators: largest,
                        learning_rate,
                        max_depth,
                        min_samples_leaf,
                        min_samples_split,
                        random_state,
                    },
                )
                .map_err(SearchError::Fit)?;
                fits += 1;
                let staged = model
                    .staged_decision_function(&test_design, &trees)
                    .map_err(SearchError::Fit)?;
                for (&stage, raw) in trees.iter().zip(&staged) {
                    fold_scores
                        .entry((rate_index, depth_index, stage))
                        .or_default()
                        .push(crate::metrics::roc_auc(&test_outcome, raw));
                }
            }
        }
    }

    let mut scores = Vec::with_capacity(all.len());
    for candidate in &all {
        let rate_index = grid
            .learning_rate
            .iter()
            .position(|value| *value == candidate.learning_rate)
            .unwrap();
        let depth_index = grid
            .max_depth
            .iter()
            .position(|value| *value == candidate.max_depth)
            .unwrap();
        let per_fold = &fold_scores[&(rate_index, depth_index, candidate.n_estimators)];
        scores.push(CandidateScore {
            candidate: *candidate,
            mean_score: crate::numpy_reduce::numpy_mean(per_fold),
            fold_scores: per_fold.clone(),
        });
    }

    let mut best = 0;
    for (index, score) in scores.iter().enumerate() {
        if score.mean_score > scores[best].mean_score {
            best = index;
        }
    }
    Ok(SearchResult {
        best: scores[best].candidate,
        best_score: scores[best].mean_score,
        scores,
        fits,
    })
}
