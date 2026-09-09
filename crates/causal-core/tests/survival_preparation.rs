use hirmos_causal_core::data_preparation::{
    prepare_longitudinal_states, prepare_wide_events, EventStatus, LongitudinalObservation,
    LongitudinalStateHistory, NonEmptyVec, PreparedMultiStateData, StateId, StateObservation,
    SubjectId, TransitionMatrix, WideEventHistory, WideSubject,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    sources: Sources,
    msprep: Vec<WideCase>,
    msm2surv: Vec<LongCase>,
}

#[derive(Deserialize)]
struct Sources {
    mstate: String,
    msm: String,
}

#[derive(Deserialize)]
struct WideCase {
    name: String,
    time: Vec<Vec<Option<f64>>>,
    status: Vec<Vec<Option<u8>>>,
    transition_matrix: Vec<Vec<Option<usize>>>,
    ids: Vec<String>,
    start_states: Vec<usize>,
    start_times: Vec<f64>,
    covariates: Vec<Vec<f64>>,
    expected: Expected,
}

#[derive(Deserialize)]
struct LongCase {
    name: String,
    observations: Vec<Observation>,
    allowed: Vec<Vec<u8>>,
    expected: Expected,
}

#[derive(Deserialize)]
struct Observation {
    id: String,
    time: f64,
    state: usize,
    covariates: Vec<f64>,
}

#[derive(Deserialize)]
struct Expected {
    rows: Vec<ExpectedRow>,
    transition_matrix: Vec<Vec<Option<usize>>>,
    notices: Vec<String>,
}

#[derive(Deserialize)]
struct ExpectedRow {
    id: String,
    from: usize,
    to: usize,
    transition: usize,
    start: f64,
    stop: f64,
    duration: f64,
    event: u8,
    covariates: Vec<f64>,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../oracle/fixtures/survival_preparation.json"))
        .expect("source fixture must parse")
}

fn state(value: usize) -> StateId {
    StateId::new(value).expect("fixture state must be valid")
}

fn assert_matches(actual: &PreparedMultiStateData, expected: &Expected) {
    assert_eq!(
        actual.transitions().as_numbered(),
        expected.transition_matrix
    );
    assert_eq!(actual.rows().len(), expected.rows.len());
    for (actual, expected) in actual.rows().iter().zip(&expected.rows) {
        assert_eq!(actual.subject.as_str(), expected.id);
        assert_eq!(actual.from.get(), expected.from);
        assert_eq!(actual.to.get(), expected.to);
        assert_eq!(actual.transition.get(), expected.transition);
        assert_eq!(actual.start, expected.start);
        assert_eq!(actual.stop, expected.stop);
        assert_eq!(actual.duration, expected.duration);
        assert_eq!(actual.status.indicator(), expected.event);
        assert_eq!(actual.covariates, expected.covariates);
    }
    let notices: Vec<String> = actual.notices().iter().map(ToString::to_string).collect();
    assert_eq!(notices, expected.notices);
}

#[test]
fn msprep_cases_match_mstate_0_3_3() {
    let fixture = fixture();
    assert_eq!(fixture.sources.mstate, "0.3.3");

    for case in fixture.msprep {
        let transitions = TransitionMatrix::from_numbered(case.transition_matrix)
            .expect("fixture transition matrix must be valid");
        let mut subjects = Vec::with_capacity(case.ids.len());
        for index in 0..case.ids.len() {
            let states = case.time[index]
                .iter()
                .copied()
                .zip(case.status[index].iter().copied())
                .map(|(time, status)| match (time, status) {
                    (None, None) => StateObservation::NotApplicable,
                    (Some(time), Some(0)) => {
                        StateObservation::recorded(time, EventStatus::Censored).unwrap()
                    }
                    (Some(time), Some(1)) => {
                        StateObservation::recorded(time, EventStatus::Observed).unwrap()
                    }
                    other => panic!("invalid fixture state observation: {other:?}"),
                })
                .collect();
            subjects.push(
                WideSubject::new(
                    SubjectId::new(case.ids[index].clone()).unwrap(),
                    NonEmptyVec::try_from_vec(states).unwrap(),
                    state(case.start_states[index]),
                    case.start_times[index],
                    case.covariates[index].clone(),
                )
                .unwrap(),
            );
        }
        let history =
            WideEventHistory::new(transitions, NonEmptyVec::try_from_vec(subjects).unwrap())
                .unwrap_or_else(|error| panic!("{} input failed: {error}", case.name));
        let actual = prepare_wide_events(history)
            .unwrap_or_else(|error| panic!("{} preparation failed: {error}", case.name));
        assert_matches(&actual, &case.expected);
    }
}

#[test]
fn msm2surv_cases_match_msm_1_8_2() {
    let fixture = fixture();
    assert_eq!(fixture.sources.msm, "1.8.2");

    for case in fixture.msm2surv {
        let transitions = TransitionMatrix::from_allowed(
            case.allowed
                .into_iter()
                .map(|row| row.into_iter().map(|value| value > 0).collect())
                .collect(),
        )
        .expect("fixture transition matrix must be valid");
        let observations = case
            .observations
            .into_iter()
            .map(|observation| {
                LongitudinalObservation::new(
                    SubjectId::new(observation.id).unwrap(),
                    observation.time,
                    state(observation.state),
                    observation.covariates,
                )
                .unwrap()
            })
            .collect();
        let history = LongitudinalStateHistory::new(
            transitions,
            NonEmptyVec::try_from_vec(observations).unwrap(),
        )
        .unwrap_or_else(|error| panic!("{} input failed: {error}", case.name));
        let actual = prepare_longitudinal_states(history)
            .unwrap_or_else(|error| panic!("{} preparation failed: {error}", case.name));
        assert_matches(&actual, &case.expected);
    }
}

#[test]
fn invalid_survival_preparation_states_are_refused_at_construction() {
    assert!(TransitionMatrix::from_numbered(vec![vec![Some(1)]]).is_err());
    assert!(TransitionMatrix::from_numbered(vec![vec![None, Some(2)], vec![None, None]]).is_err());

    let cycle =
        TransitionMatrix::from_numbered(vec![vec![None, Some(1)], vec![Some(2), None]]).unwrap();
    let subject = WideSubject::new(
        SubjectId::new("one").unwrap(),
        NonEmptyVec::try_from_vec(vec![
            StateObservation::recorded(0.0, EventStatus::Observed).unwrap(),
            StateObservation::recorded(1.0, EventStatus::Observed).unwrap(),
        ])
        .unwrap(),
        state(1),
        0.0,
        vec![],
    )
    .unwrap();
    assert!(
        WideEventHistory::new(cycle, NonEmptyVec::try_from_vec(vec![subject]).unwrap()).is_err()
    );
}

#[test]
fn longitudinal_input_rejects_disallowed_observed_transitions() {
    let transitions = TransitionMatrix::from_allowed(vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![false, false, false],
    ])
    .unwrap();
    let observations = vec![
        LongitudinalObservation::new(SubjectId::new("one").unwrap(), 0.0, state(1), vec![])
            .unwrap(),
        LongitudinalObservation::new(SubjectId::new("one").unwrap(), 1.0, state(3), vec![])
            .unwrap(),
    ];
    let history = LongitudinalStateHistory::new(
        transitions,
        NonEmptyVec::try_from_vec(observations).unwrap(),
    )
    .unwrap();
    assert!(prepare_longitudinal_states(history).is_err());
}
