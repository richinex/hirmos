use serde_json::Value;
use hirmos_causal_core::staggered_did::{inference::*, Summary};

#[test]
fn simultaneous_intervals_use_the_joint_r_critical_value() {
    let cases: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/staggered-did/bootstrap.json"
        ))
        .unwrap(),
    )
    .unwrap();
    for c in cases.as_object().unwrap().values() {
        let clusters: Vec<_> = c["clusters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();
        let summaries: Vec<_> = (0..2)
            .map(|j| Summary {
                att: j as f64 - 0.5,
                se: 0.0,
                influence: c["influence"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|r| r[j].as_f64().unwrap())
                    .collect(),
            })
            .collect();
        let points = [
            Point::Estimated(&summaries[0]),
            Point::Estimated(&summaries[1]),
            Point::Reference,
        ];
        let result = infer(
            &points,
            &clusters,
            Confidence::new(0.95).unwrap(),
            Method::BootstrapSimultaneous {
                iterations: 199,
                seed: 731,
            },
        );
        let critical = c["critical"].as_f64().unwrap();
        if critical < spec_math::cephes64::ndtri(0.975) {
            assert!(matches!(result, Err(Error::BandNarrowerThanPointwise)));
            continue;
        }
        let result = result.unwrap();
        assert!(
            matches!(result.coverage,Coverage::Simultaneous {critical:v,..} if (v-critical).abs()<1e-12)
        );
        for (j, s) in summaries.iter().enumerate() {
            let se = c["se"][j].as_f64().unwrap();
            let Interval::Estimated { lower, upper, .. } = result.intervals[j] else {
                panic!()
            };
            assert!((lower - (s.att - critical * se)).abs() < 1e-12);
            assert!((upper - (s.att + critical * se)).abs() < 1e-12);
        }
        assert_eq!(result.intervals[2], Interval::Reference);
    }
}

#[test]
fn invalid_inference_cannot_masquerade_as_an_interval() {
    let s = Summary {
        att: 2.0,
        se: 0.0,
        influence: vec![0.0; 4],
    };
    let points = [Point::Estimated(&s), Point::Reference];
    let c = Confidence::new(0.95).unwrap();
    let result = infer(&points, &[1, 2, 3, 4], c, Method::Analytical).unwrap();
    assert_eq!(
        result.intervals,
        vec![Interval::Unavailable { att: 2.0 }, Interval::Reference]
    );
    assert!(matches!(
        infer(
            &points,
            &[1, 2, 3, 4],
            c,
            Method::BootstrapSimultaneous {
                iterations: 199,
                seed: 731
            }
        ),
        Err(Error::UnavailableSimultaneousBand)
    ));
    assert!(matches!(
        infer(&points, &[1, 1, 2, 2], c, Method::Analytical),
        Err(Error::AnalyticalClustering)
    ));
    assert!(matches!(
        infer(&points, &[1, 2], c, Method::Analytical),
        Err(Error::Shape)
    ));
    for level in [0.0, 1.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(Confidence::new(level).is_err());
    }
}

#[test]
fn analytical_intervals_recompute_uncertainty_from_influence_not_stale_se() {
    let s = Summary {
        att: 3.0,
        se: 999.0,
        influence: vec![-2.0, 2.0, 2.0, -2.0],
    };
    let result = infer(
        &[Point::Estimated(&s)],
        &[1, 2, 3, 4],
        Confidence::new(0.95).unwrap(),
        Method::Analytical,
    )
    .unwrap();
    let Interval::Estimated {
        se, lower, upper, ..
    } = result.intervals[0]
    else {
        panic!()
    };
    assert_eq!(se, 1.0);
    assert!((lower - 1.040036015459946).abs() < 1e-12);
    assert!((upper - 4.959963984540054).abs() < 1e-12);
}
