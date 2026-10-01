use hirmos_causal_core::panel_design::{Error, EventTails, EventWindow, Panel};
use nalgebra::DMatrix;
use std::collections::BTreeMap;

fn window(tails: EventTails) -> EventWindow {
    EventWindow {
        first: -1,
        last: 1,
        reference: 0,
        tails,
    }
}

#[test]
fn tails_and_reference_are_explicit_and_controls_remain_zero() {
    let keys: Vec<_> = (0..2).flat_map(|u| (0..5).map(move |t| (u, t))).collect();
    let panel = Panel::new(&keys, DMatrix::zeros(10, 0)).unwrap();
    let adoption = BTreeMap::from([(0, Some(2)), (1, None)]);
    let reference = panel
        .pooled_events(&adoption, &[], window(EventTails::Reference))
        .unwrap();
    let binned = panel
        .pooled_events(&adoption, &[], window(EventTails::Bin))
        .unwrap();
    assert_eq!(reference.event_periods, vec![-1, 1]);
    assert_eq!(reference.support, vec![1, 1]);
    assert_eq!(binned.support, vec![2, 2]);
    for design in [&reference, &binned] {
        assert!(design.matrix.row(2).iter().all(|x| *x == 0.0));
        for i in 5..10 {
            assert!(design.matrix.row(i).iter().all(|x| *x == 0.0));
        }
        assert_eq!(design.original_rows, (0..10).collect::<Vec<_>>());
    }
}

#[test]
fn invalid_or_unsupported_event_requests_are_named_errors() {
    let keys: Vec<_> = (0..2).flat_map(|u| (0..5).map(move |t| (u, t))).collect();
    let panel = Panel::new(&keys, DMatrix::zeros(10, 1)).unwrap();
    let adoption = BTreeMap::from([(0, Some(2)), (1, None)]);
    assert_eq!(
        panel
            .pooled_events(&BTreeMap::new(), &[], window(EventTails::Reference))
            .err(),
        Some(Error::InvalidEvent)
    );
    assert_eq!(
        panel
            .pooled_events(&adoption, &[0, 0], window(EventTails::Reference))
            .err(),
        Some(Error::InvalidEvent)
    );
    assert_eq!(
        panel
            .pooled_events(&adoption, &[1], window(EventTails::Reference))
            .err(),
        Some(Error::InvalidEvent)
    );
    assert_eq!(
        panel
            .pooled_events(
                &adoption,
                &[],
                EventWindow {
                    first: -5,
                    last: 5,
                    reference: 0,
                    tails: EventTails::Reference
                }
            )
            .err(),
        Some(Error::UnsupportedEvent)
    );
    assert_eq!(
        panel
            .pooled_events(
                &adoption,
                &[],
                EventWindow {
                    first: i64::MIN,
                    last: i64::MAX,
                    reference: 0,
                    tails: EventTails::Reference
                }
            )
            .err(),
        Some(Error::InvalidEvent)
    );
    assert_eq!(
        panel
            .pooled_events(
                &adoption,
                &[],
                EventWindow {
                    first: -1,
                    last: 1,
                    reference: -1,
                    tails: EventTails::Bin
                }
            )
            .err(),
        Some(Error::InvalidEvent)
    );
    assert_eq!(
        panel
            .pooled_events(
                &BTreeMap::from([(0, Some(i64::MIN)), (1, None)]),
                &[],
                window(EventTails::Reference)
            )
            .err(),
        Some(Error::InvalidEvent)
    );
}
