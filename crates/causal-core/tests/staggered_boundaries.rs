use hirmos_causal_core::staggered_did::*;

fn rows() -> Vec<Row> {
    (1..=12)
        .flat_map(|i| {
            (1..=5).map(move |t| Row {
                unit: Unit(i),
                period: Period(t),
                adoption: if i <= 6 {
                    Adoption::At(Period(3))
                } else {
                    Adoption::Never
                },
                outcome: (i as f64).sin() * t as f64,
            })
        })
        .collect()
}

#[test]
fn weights_are_keyed_finite_complete_and_scale_invariant() {
    for invalid in [
        vec![],
        vec![(Unit(1), 1.0); 12],
        (1..=12).map(|i| (Unit(i), 0.0)).collect(),
        (1..=12)
            .map(|i| (Unit(i), if i == 1 { f64::NAN } else { 1.0 }))
            .collect(),
    ] {
        assert!(matches!(
            Panel::new(&rows()).unwrap().with_weights(invalid),
            Err(Error::InvalidWeights)
        ));
    }
    let fit = |scale| {
        let p = Panel::new(&rows())
            .unwrap()
            .with_weights(
                (1..=12)
                    .rev()
                    .map(|i| (Unit(i), (i as f64 + 1.0) * scale))
                    .collect(),
            )
            .unwrap();
        dynamic(&p, Controls::NeverTreated, Baseline::Varying).unwrap()
    };
    let a = fit(1.0);
    let b = fit(100.0);
    assert!((a.overall.att - b.overall.att).abs() < 1e-12);
    assert!((a.overall.se - b.overall.se).abs() < 1e-12);
}

#[test]
fn invalid_windows_and_insufficient_preperiod_are_errors() {
    assert!(EventWindow::new(Some(2), Some(-1), None).is_err());
    let p = Panel::new(&rows()).unwrap();
    assert!(matches!(
        event_study(
            &p,
            Controls::NeverTreated,
            Baseline::Varying,
            Anticipation(0),
            EventWindow::new(Some(20), Some(30), None).unwrap()
        ),
        Err(Error::InvalidWindow)
    ));
    assert!(matches!(
        with_anticipation(
            &p,
            Controls::NeverTreated,
            Baseline::Universal,
            Anticipation(9)
        ),
        Err(Error::NoBaseline)
    ));
}

#[test]
fn unsupported_panel_shapes_cannot_change_estimators() {
    let mut r = rows();
    r.pop();
    assert!(matches!(
        preparation::prepare(&r, Controls::NeverTreated, Anticipation(0)),
        Err(Error::Unbalanced { .. })
    ));
    r = rows();
    r.push(r[0]);
    assert!(matches!(
        preparation::prepare(&r, Controls::NeverTreated, Anticipation(0)),
        Err(Error::Duplicate { .. })
    ));
    r = rows();
    r[0].outcome = f64::NAN;
    assert!(matches!(
        preparation::prepare(&r, Controls::NeverTreated, Anticipation(0)),
        Err(Error::NonFinite { .. })
    ));
}

#[test]
fn small_treated_cohort_is_reported_but_small_never_control_is_rejected() {
    let small_treated: Vec<_> = rows()
        .into_iter()
        .filter(|r| r.unit == Unit(1) || r.unit.0 > 6)
        .collect();
    let prepared =
        preparation::prepare(&small_treated, Controls::NeverTreated, Anticipation(0)).unwrap();
    assert_eq!(prepared.small_cohorts.len(), 1);
    assert_eq!(prepared.small_cohorts[0].count, 1);
    assert!(dynamic(&prepared.panel, Controls::NeverTreated, Baseline::Varying).is_ok());
    let small_control: Vec<_> = rows().into_iter().filter(|r| r.unit.0 <= 7).collect();
    assert!(matches!(
        preparation::prepare(&small_control, Controls::NeverTreated, Anticipation(0)),
        Err(Error::SmallControlGroup {
            count: 1,
            required: 5
        })
    ));
}

#[test]
fn baseline_references_never_gain_invented_intervals() {
    let p = Panel::new(&rows()).unwrap();
    let fit = dynamic(&p, Controls::NeverTreated, Baseline::Universal).unwrap();
    assert_eq!(fit.cells[&(Period(3), Period(2))], Cell::Reference);
    assert_eq!(fit.event_support[&-1].reference_cohorts, vec![Period(3)]);
}
