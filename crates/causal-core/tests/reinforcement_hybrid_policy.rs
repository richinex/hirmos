use hirmos_causal_core::reinforcement::decision_process::{bellman_operator, TabularMdp};
use hirmos_causal_core::reinforcement::counterfactual_planning::CounterfactualModel;
use hirmos_causal_core::reinforcement::hybrid_policy::{plan_hybrid, FreshIntentionModel, HybridError, HybridPolicy};

const DISCOUNT: f64 = 0.9;
const INITIAL: [f64; 2] = [0.5, 0.5];

fn bit(value: bool) -> usize {
    usize::from(value)
}

// The inventory of Eq. 5: intention X = S xor U1, so U1 = S xor X is known once the intention is.
fn inventory() -> (CounterfactualModel, Vec<f64>) {
    let intention = vec![0.1, 0.9, 0.9, 0.1];
    let mut rewards = vec![0.0; 8];
    let mut transitions = vec![0.0; 8 * 4];
    for s in 0..2 {
        for x in 0..2 {
            let u1 = s ^ x;
            for realised in 0..2 {
                let cell = (s * 2 + x) * 2 + realised;
                rewards[cell] = if s ^ realised ^ u1 == 1 { 0.9 } else { 0.1 };
                for u2 in 0..2 {
                    let next = (bit(s == 1 || realised == 1)) ^ u1 ^ u2;
                    let p = if u2 == 1 { 0.1 } else { 0.9 };
                    for next_intention in 0..2 {
                        transitions[cell * 4 + next * 2 + next_intention] += p * intention[next * 2 + next_intention];
                    }
                }
            }
        }
    }
    (CounterfactualModel::new(2, 2, rewards, transitions).unwrap(), intention)
}

fn optimal(mdp: &TabularMdp) -> Vec<f64> {
    (0..2000).fold(vec![0.0; mdp.states()], |value, _| bellman_operator(mdp, &value).unwrap().value)
}

/// Discounted value and consulting share of a hybrid policy, by iterating its Bellman equations on (state, intention).
fn evaluate(model: &CounterfactualModel, intention: &[f64], policy: &HybridPolicy) -> (f64, f64) {
    let choose = |pair: usize, realised: usize| {
        let s = pair / 2;
        (1.0 - policy.consult[s]) * policy.autonomous[s * 2 + realised] + policy.consult[s] * policy.assisted[pair * 2 + realised]
    };
    let start: Vec<f64> = (0..4).map(|pair| INITIAL[pair / 2] * intention[pair]).collect();
    let (mut value, mut occupancy) = (vec![0.0; 4], start.clone());
    for _ in 0..2000 {
        value = (0..4)
            .map(|pair| {
                (0..2)
                    .map(|a| {
                        let cell = pair * 2 + a;
                        let ahead: f64 = (0..4).map(|n| model.transitions()[cell * 4 + n] * value[n]).sum();
                        choose(pair, a) * (model.rewards()[cell] + DISCOUNT * ahead)
                    })
                    .sum()
            })
            .collect();
        occupancy = (0..4)
            .map(|next| {
                start[next]
                    + DISCOUNT
                        * (0..8).map(|cell| occupancy[cell / 2] * choose(cell / 2, cell % 2) * model.transitions()[cell * 4 + next]).sum::<f64>()
            })
            .collect();
    }
    let total: f64 = start.iter().zip(&value).map(|(p, v)| p * v).sum();
    let consulted: f64 = (0..4).map(|pair| occupancy[pair] * policy.consult[pair / 2]).sum();
    (total, consulted * (1.0 - DISCOUNT))
}

#[test]
fn no_budget_gives_the_best_policy_that_ignores_intentions_and_a_full_budget_the_best_that_uses_them() {
    let (model, intention) = inventory();
    let fresh = FreshIntentionModel::new(&model, intention.clone()).unwrap();

    let mut transitions = vec![0.0; 2 * 2 * 2];
    let mut rewards = vec![0.0; 4];
    for s in 0..2 {
        for a in 0..2 {
            for x in 0..2 {
                let cell = (s * 2 + x) * 2 + a;
                rewards[s * 2 + a] += intention[s * 2 + x] * model.rewards()[cell];
                for next in 0..2 {
                    transitions[(a * 2 + s) * 2 + next] += intention[s * 2 + x] * (model.transitions()[cell * 4 + next * 2] + model.transitions()[cell * 4 + next * 2 + 1]);
                }
            }
        }
    }
    let alone = optimal(&TabularMdp::new(2, 2, transitions, rewards, DISCOUNT).unwrap());
    let alone_value: f64 = INITIAL.iter().zip(&alone).map(|(p, v)| p * v).sum();
    let helped = optimal(&model.decision_process(DISCOUNT).unwrap());
    let helped_value: f64 = (0..4).map(|pair| INITIAL[pair / 2] * intention[pair] * helped[pair]).sum();

    assert!((plan_hybrid(&fresh, &INITIAL, DISCOUNT, 0.0).unwrap().value - alone_value).abs() < 1e-9);
    assert!((plan_hybrid(&fresh, &INITIAL, DISCOUNT, 1.0).unwrap().value - helped_value).abs() < 1e-9);
    assert!((alone_value - 0.82 / (1.0 - DISCOUNT)).abs() < 1e-9 && (helped_value - 0.9 / (1.0 - DISCOUNT)).abs() < 1e-9);
}

#[test]
fn larger_budgets_never_lower_the_value_and_every_plan_keeps_its_budget_and_its_value() {
    let (model, intention) = inventory();
    let fresh = FreshIntentionModel::new(&model, intention.clone()).unwrap();
    let mut previous = f64::NEG_INFINITY;
    for budget in [0.0, 0.1, 0.3, 0.5, 0.7, 0.9, 1.0] {
        let policy = plan_hybrid(&fresh, &INITIAL, DISCOUNT, budget).unwrap();
        let (value, share) = evaluate(&model, &intention, &policy);
        assert!((value - policy.value).abs() < 1e-9, "budget {budget}: evaluated {value}, planned {}", policy.value);
        assert!((share - policy.consulted_share).abs() < 1e-9 && share <= budget + 1e-9, "budget {budget}: share {share}");
        assert!(policy.value >= previous - 1e-12);
        previous = policy.value;
    }
}

#[test]
fn an_intention_that_remembers_more_than_the_state_is_refused() {
    let (model, _) = inventory();
    let mut transitions = model.transitions().to_vec();
    transitions[..4].copy_from_slice(&[0.5, 0.0, 0.0, 0.5]);
    let skewed = CounterfactualModel::new(2, 2, model.rewards().to_vec(), transitions).unwrap();
    assert_eq!(FreshIntentionModel::new(&skewed, vec![0.1, 0.9, 0.9, 0.1]), Err(HybridError::IntentionNotFresh { cell: 0 }));
}
