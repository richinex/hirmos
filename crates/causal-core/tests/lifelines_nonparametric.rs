use hirmos_causal_core::survival::nonparametric::{
    kaplan_meier, nelson_aalen, EventStatus, NelsonAalenTies, WeightedObservation,
};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../oracle/fixtures/lifelines_nonparametric.json"
    ))
    .unwrap()
}

fn close(actual: f64, expected: &Value, label: &str) {
    let expected = expected.as_f64().unwrap();
    assert!(
        (actual - expected).abs() <= 2e-13 * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}"
    );
}

#[test]
fn weighted_nonparametric_estimators_match_lifelines_0303() {
    let root = fixture();
    assert_eq!(root["lifelines"], "0.30.3");

    for case in root["cases"].as_array().unwrap() {
        let durations = case["durations"].as_array().unwrap();
        let events = case["events"].as_array().unwrap();
        let weights = case["weights"].as_array().unwrap();
        let entries = case["entry"].as_array();
        let observations = durations
            .iter()
            .zip(events)
            .zip(weights)
            .enumerate()
            .map(|(index, ((duration, event), weight))| {
                let duration = duration.as_f64().unwrap();
                let entry = entries.map_or(0.0, |values| values[index].as_f64().unwrap());
                WeightedObservation::with_entry(
                    duration,
                    entry,
                    if event.as_i64().unwrap() == 1 {
                        EventStatus::Observed
                    } else {
                        EventStatus::Censored
                    },
                    weight.as_f64().unwrap(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let timeline = case["timeline"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();

        let hazard = nelson_aalen(
            &observations,
            &timeline,
            0.05,
            if case["smoothing"].as_bool().unwrap() {
                NelsonAalenTies::Smoothed
            } else {
                NelsonAalenTies::Discrete
            },
        )
        .unwrap();
        let survival = kaplan_meier(&observations, &timeline, 0.05).unwrap();
        let name = case["name"].as_str().unwrap();

        for (index, estimate) in hazard.iter().enumerate() {
            close(estimate.time, &case["timeline"][index], name);
            close(
                estimate.cumulative_hazard,
                &case["nelson_aalen"]["cumulative_hazard"][index],
                name,
            );
            close(
                estimate.variance,
                &case["nelson_aalen"]["variance"][index],
                name,
            );
            close(estimate.lower, &case["nelson_aalen"]["lower"][index], name);
            close(estimate.upper, &case["nelson_aalen"]["upper"][index], name);
        }

        for (index, estimate) in survival.iter().enumerate() {
            close(estimate.time, &case["timeline"][index], name);
            close(
                estimate.survival,
                &case["kaplan_meier"]["survival"][index],
                name,
            );
            close(
                estimate.cumulative_density,
                &case["kaplan_meier"]["cumulative_density"][index],
                name,
            );
            close(estimate.lower, &case["kaplan_meier"]["lower"][index], name);
            close(estimate.upper, &case["kaplan_meier"]["upper"][index], name);
        }
    }
}

#[test]
fn invalid_nonparametric_rows_are_refused_before_fitting() {
    assert!(WeightedObservation::new(-1.0, EventStatus::Observed, 1.0).is_err());
    assert!(WeightedObservation::with_entry(1.0, 2.0, EventStatus::Observed, 1.0).is_err());
    assert!(WeightedObservation::new(1.0, EventStatus::Observed, -1.0).is_err());
}
