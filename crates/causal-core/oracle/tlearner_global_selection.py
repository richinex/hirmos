"""A cross-fitted T-learner whose arm models are chosen once, on all of each arm's rows.

The grid search for each arm runs on every row of that arm, as EconML's `_crossfit` selects a
nuisance model before refitting it per fold and as DoubleML tunes when `tune_on_folds=False`. The
rows are then split in half, stratified on the outcome; each half fits the two chosen candidates on
its own arm rows and predicts the other half. The data is the generated sample of
sklearn_crossfit_kernel.json, so the two fixtures share their rows.
"""
import json, sys
import numpy as np, sklearn
from sklearn.base import clone
from sklearn.ensemble import GradientBoostingClassifier
from sklearn.model_selection import GridSearchCV, train_test_split

SEED, ROWS, COLUMNS = 1234, 400, 5
LEAF = int(sys.argv[1]) if len(sys.argv) > 1 else 5
grid = {'learning_rate': [0.05, 0.15], 'max_depth': [1, 2], 'n_estimators': [5, 10]}

confounders = np.array([[float((r * (c + 3) + c * c) % 29) for c in range(COLUMNS)] for r in range(ROWS)])
treatment = np.array([float(((confounders[r, 0] + confounders[r, 2]) % 7) > 3) for r in range(ROWS)])
death = np.array([float(((confounders[r, 1] * 2 + treatment[r] * 5) % 11) > 4) for r in range(ROWS)])


def search(rows):
    return GridSearchCV(GradientBoostingClassifier(random_state=SEED, min_samples_leaf=LEAF),
                        grid, scoring='roc_auc', cv=5, n_jobs=1).fit(confounders[rows], death[rows])


def params(fitted):
    p = fitted.best_params_
    return {'learning_rate': float(p['learning_rate']), 'max_depth': int(p['max_depth']), 'n_estimators': int(p['n_estimators'])}


treated_search = search(np.flatnonzero(treatment == 1.0))
control_search = search(np.flatnonzero(treatment == 0.0))

rows = np.arange(ROWS)
first, second, _, _ = train_test_split(rows, death, test_size=0.5, stratify=death, random_state=SEED)


def arm_models(half):
    treated = half[treatment[half] == 1.0]
    control = half[treatment[half] == 0.0]
    return (clone(treated_search.best_estimator_).fit(confounders[treated], death[treated]),
            clone(control_search.best_estimator_).fit(confounders[control], death[control]))


first_treated, first_control = arm_models(first)
second_treated, second_control = arm_models(second)
effects = np.concatenate([
    second_treated.predict_proba(confounders[first])[:, 1] - second_control.predict_proba(confounders[first])[:, 1],
    first_treated.predict_proba(confounders[second])[:, 1] - first_control.predict_proba(confounders[second])[:, 1]])

print(json.dumps({
    'versions': {'sklearn': sklearn.__version__, 'numpy': np.__version__},
    'rows': ROWS, 'columns': COLUMNS, 'seed': SEED, 'min_samples_leaf': LEAF, 'grid': grid,
    'selected': {'treated': params(treated_search), 'control': params(control_search)},
    'validation_auc': {'treated': float(treated_search.best_score_), 'control': float(control_search.best_score_)},
    'order': [int(v) for v in np.concatenate([first, second])],
    'effects': [float(v) for v in effects],
    'ate': float(effects.mean()),
}, indent=1))
