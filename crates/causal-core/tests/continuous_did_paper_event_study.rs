use hirmos_causal_core::staggered_did::{
    event_study, Adoption, Anticipation, Baseline, Controls, EventWindow, Panel, Period, Row, Unit,
};

#[test]
fn paper_event_studies_reuse_existing_kernel() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/window-contrast.json")).unwrap();
    let oracle: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/paper-event-study.json")).unwrap();
    let median = oracle["median"].as_f64().unwrap();
    for case in oracle["cases"].as_array().unwrap() {
        let rows: Vec<_> = data["rows"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| {
                let d = r["d"].as_f64().unwrap();
                d == 0.0
                    || match case["name"].as_str().unwrap() {
                        "above" => d > median,
                        "below" => d <= median,
                        "all" => true,
                        _ => panic!("unknown case"),
                    }
            })
            .map(|r| Row {
                unit: Unit(r["i"].as_u64().unwrap()),
                period: Period(r["t"].as_i64().unwrap()),
                adoption: if r["d"].as_f64().unwrap() == 0.0 {
                    Adoption::Never
                } else {
                    Adoption::At(Period(r["G"].as_i64().unwrap()))
                },
                outcome: r["y"].as_f64().unwrap(),
            })
            .collect();
        let panel = Panel::new(&rows).unwrap();
        assert_eq!(
            panel.units().count(),
            case["units"].as_u64().unwrap() as usize
        );
        let f = event_study(
            &panel,
            Controls::NotYetTreated,
            Baseline::Universal,
            Anticipation(0),
            EventWindow::new(Some(-11), Some(4), None).unwrap(),
        )
        .unwrap();
        let events = case["events"].as_array().unwrap();
        assert_eq!(f.events.len(), events.len());
        for (i, e) in events.iter().enumerate() {
            let event = e.as_i64().unwrap();
            let got = &f.events[&event];
            assert!(
                (got.att - case["att"][i].as_f64().unwrap()).abs() < 1e-10,
                "{} e={event} ATT",
                case["name"]
            );
            if let Some(se) = case["se"][i].as_f64() {
                assert!(
                    (got.se - se).abs() < 1e-10,
                    "{} e={event} SE {} vs {se}",
                    case["name"],
                    got.se
                );
            } else {
                assert_eq!(event, -1);
                assert_eq!(got.se, 0.0);
            }
        }
        assert!((f.overall.att - case["overall"].as_f64().unwrap()).abs() < 1e-10);
        assert!((f.overall.se - case["overall_se"].as_f64().unwrap()).abs() < 1e-10);
        let influence = nalgebra::DMatrix::from_fn(panel.units().count(), events.len(), |i, j| {
            f.events[&events[j].as_i64().unwrap()].influence[i]
        });
        let boot = hirmos_causal_core::staggered_did::bootstrap::run(
            &influence,
            &panel.units().map(|u| u.0).collect::<Vec<_>>(),
            1000,
            0.05,
            731,
        )
        .unwrap();
        let reference = &case["bootstrap"];
        let critical = boot.critical.unwrap();
        assert!((critical - reference["critical"].as_f64().unwrap()).abs() < 1e-9);
        for (j, event) in events.iter().enumerate() {
            if let Some(se) = boot.se[j] {
                assert!((se - reference["se"][j].as_f64().unwrap()).abs() < 1e-10);
                let att = f.events[&event.as_i64().unwrap()].att;
                for (key, bound) in [
                    ("pointwise_lower", att - 1.959963984540054 * se),
                    ("pointwise_upper", att + 1.959963984540054 * se),
                    ("simultaneous_lower", att - critical * se),
                    ("simultaneous_upper", att + critical * se),
                ] {
                    assert!((bound - reference[key][j].as_f64().unwrap()).abs() < 1e-9);
                }
            } else {
                assert_eq!(event.as_i64().unwrap(), -1);
            }
        }
    }
}
