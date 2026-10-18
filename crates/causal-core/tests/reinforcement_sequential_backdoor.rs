use hirmos_causal_core::do_calculus::Admg;
use hirmos_causal_core::reinforcement::sequential_backdoor::{sequential_backdoor, DecisionRule};
use std::collections::BTreeSet;

fn names(list: &[&str]) -> BTreeSet<String> {
    list.iter().map(|name| name.to_string()).collect()
}

fn edges(list: &[(&str, &str)]) -> Vec<(String, String)> {
    list.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

fn rule(action: &str, inputs: &[&str]) -> DecisionRule {
    DecisionRule { action: action.to_string(), inputs: names(inputs) }
}

fn admissible(graph: &Admg, rules: &[DecisionRule]) -> Vec<bool> {
    sequential_backdoor(graph, rules, &names(&["Y"])).unwrap().into_iter().map(|stage| stage.blocked).collect()
}

// Two stages with the confounder acting only on states and outcome.
fn measured_confounding() -> Admg {
    Admg::new(
        names(&["S1", "X1", "S2", "X2", "Y"]),
        edges(&[
            ("S1", "X1"), ("S1", "S2"), ("S1", "X2"), ("S1", "Y"), ("X1", "S2"), ("X1", "X2"),
            ("X1", "Y"), ("S2", "X2"), ("S2", "Y"), ("X2", "Y"),
        ]),
        edges(&[("S1", "S2"), ("S1", "Y"), ("S2", "Y")]),
    )
    .unwrap()
}

// S1 <- U1 -> X1, S2 <- U2 -> X2, S2 <- S1, X1, Y <- S1, X1.
fn latent_action_confounding() -> Admg {
    Admg::new(
        names(&["S1", "X1", "S2", "X2", "Y"]),
        edges(&[("S1", "S2"), ("X1", "S2"), ("S1", "Y"), ("X1", "Y")]),
        edges(&[("S1", "X1"), ("S2", "X2")]),
    )
    .unwrap()
}

#[test]
fn rules_reading_the_full_history_are_admissible_when_no_confounder_reaches_the_actions() {
    let rules = [rule("X1", &["S1"]), rule("X2", &["S1", "X1", "S2"])];
    assert_eq!(admissible(&measured_confounding(), &rules), [true, true]);
}

#[test]
fn rules_that_read_nothing_leave_the_first_stage_open() {
    let rules = [rule("X1", &[]), rule("X2", &[])];
    assert_eq!(admissible(&measured_confounding(), &rules)[0], false);
}

#[test]
fn reading_the_first_state_is_admissible_even_with_latent_confounding_of_both_actions() {
    let graph = latent_action_confounding();
    assert_eq!(admissible(&graph, &[rule("X1", &["S1"]), rule("X2", &["S1"])]), [true, true]);
    assert_eq!(admissible(&graph, &[rule("X1", &[]), rule("X2", &[])])[0], false);
}
