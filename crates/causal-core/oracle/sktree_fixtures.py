# Golden fixtures for the sklearn tree/forest port. Run: uv run python oracle/sktree_fixtures.py
import json

import numpy as np
from sklearn.ensemble import RandomForestClassifier, RandomForestRegressor
from sklearn.tree import DecisionTreeClassifier, DecisionTreeRegressor

fixtures = {}
rng = np.random.default_rng(151)
n, p = 300, 5
X = rng.normal(0, 1, (n, p))
y = 1.5 * X[:, 0] - 0.8 * X[:, 1] * X[:, 2] + 0.5 * np.abs(X[:, 3]) + rng.normal(0, 0.5, n)
yc = (X[:, 0] + 0.7 * X[:, 1] + rng.normal(0, 1, n) > 0).astype(float)
Xt = rng.normal(0, 1, (80, p))


def structure(estimator):
    """The complete part of sklearn's tree representation implemented by the port."""
    tree = estimator.tree_
    return {
        "children_left": tree.children_left.tolist(),
        "children_right": tree.children_right.tolist(),
        "feature": tree.feature.tolist(),
        "threshold": tree.threshold.tolist(),
        "value": tree.value[:, 0, :].tolist(),
    }

tree = DecisionTreeRegressor(min_samples_leaf=5, random_state=3)
tree.fit(X, y)
fixtures["tree_reg"] = {"pred": tree.predict(Xt).tolist(), "structure": structure(tree)}
print("tree_reg nodes:", tree.tree_.node_count)

treec = DecisionTreeClassifier(min_samples_leaf=5, random_state=3, max_features="sqrt")
treec.fit(X, yc)
fixtures["tree_clf"] = {"proba": treec.predict_proba(Xt)[:, 1].tolist(), "structure": structure(treec)}
print("tree_clf nodes:", treec.tree_.node_count)

rf = RandomForestRegressor(n_estimators=30, min_samples_leaf=5, random_state=7)
rf.fit(X, y)
fixtures["rf_reg"] = {
    "pred": rf.predict(Xt).tolist(),
    "structures": [structure(tree) for tree in rf.estimators_],
}
rfc = RandomForestClassifier(n_estimators=30, min_samples_leaf=5, random_state=7)
rfc.fit(X, yc)
fixtures["rf_clf"] = {
    "proba": rfc.predict_proba(Xt)[:, 1].tolist(),
    "structures": [structure(tree) for tree in rfc.estimators_],
}
print("forests done")

fixtures["X"] = X.tolist(); fixtures["y"] = y.tolist(); fixtures["yc"] = yc.tolist(); fixtures["Xt"] = Xt.tolist()
with open("oracle/fixtures/sktree.json", "w") as f:
    json.dump(fixtures, f, indent=1)
print("wrote oracle/fixtures/sktree.json")
