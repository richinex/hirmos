use hirmos_causal_core::data_preparation::{
    prepare_cohort_events, CohortEntry, CohortEvent, CohortEventHistory, CohortId, EventStatus,
    NonEmptyVec, PositiveCount, StratumId,
};

fn cohort(value: &str) -> CohortId {
    CohortId::new(value).unwrap()
}

fn stratum(value: &str) -> StratumId {
    StratumId::new(value).unwrap()
}

fn count(value: usize) -> PositiveCount {
    PositiveCount::new(value).unwrap()
}

#[test]
fn one_history_produces_survival_rows_and_calendar_flows() {
    let cohorts = NonEmptyVec::try_from_vec(vec![
        CohortEntry::new(cohort("jan-a"), stratum("A"), 1.0, 6.0, count(10)).unwrap(),
        CohortEntry::new(cohort("mar-a"), stratum("A"), 3.0, 6.0, count(5)).unwrap(),
        CohortEntry::new(cohort("jan-b"), stratum("B"), 1.0, 6.0, count(8)).unwrap(),
    ])
    .unwrap();
    let events = vec![
        CohortEvent::new(cohort("jan-a"), 3.0, count(2)).unwrap(),
        CohortEvent::new(cohort("jan-a"), 5.0, count(1)).unwrap(),
        CohortEvent::new(cohort("mar-a"), 5.0, count(2)).unwrap(),
        CohortEvent::new(cohort("jan-b"), 4.0, count(3)).unwrap(),
    ];
    let prepared =
        prepare_cohort_events(CohortEventHistory::new(cohorts, events).unwrap()).unwrap();

    assert_eq!(prepared.survival_rows().len(), 7);
    assert_eq!(
        prepared
            .survival_rows()
            .iter()
            .filter(|row| row.status == EventStatus::Observed)
            .map(|row| row.frequency)
            .sum::<usize>(),
        8
    );
    assert_eq!(
        prepared
            .survival_rows()
            .iter()
            .filter(|row| row.status == EventStatus::Censored)
            .map(|row| row.frequency)
            .sum::<usize>(),
        15
    );

    let a = prepared
        .calendar_rows()
        .iter()
        .filter(|row| row.stratum.as_str() == "A")
        .collect::<Vec<_>>();
    assert_eq!(a.len(), 4);
    assert_eq!(
        (a[0].time, a[0].entrants, a[0].at_risk_after),
        (1.0, 10, 10)
    );
    assert_eq!(
        (a[1].time, a[1].entrants, a[1].events, a[1].at_risk_after),
        (3.0, 5, 2, 13)
    );
    assert_eq!((a[2].time, a[2].events, a[2].at_risk_after), (5.0, 3, 10));
    assert_eq!((a[3].time, a[3].censored, a[3].at_risk_after), (6.0, 10, 0));
}

#[test]
fn invalid_cohort_histories_are_unrepresentable_or_refused() {
    assert!(CohortId::new(" ").is_err());
    assert!(PositiveCount::new(0).is_err());
    assert!(CohortEntry::new(cohort("bad"), stratum("A"), 2.0, 2.0, count(1)).is_err());

    let cohorts = NonEmptyVec::try_from_vec(vec![CohortEntry::new(
        cohort("one"),
        stratum("A"),
        0.0,
        4.0,
        count(2),
    )
    .unwrap()])
    .unwrap();
    let too_many = vec![CohortEvent::new(cohort("one"), 1.0, count(3)).unwrap()];
    assert!(CohortEventHistory::new(cohorts.clone(), too_many).is_err());

    let unknown = vec![CohortEvent::new(cohort("other"), 1.0, count(1)).unwrap()];
    assert!(CohortEventHistory::new(cohorts, unknown).is_err());
}
