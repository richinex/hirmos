use hirmos_causal_core::continuous_did::panel::{self, Adoption, Observation, Period, Row, Unit};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/continuous_did/panels.json")).unwrap()
}

#[test]
fn complete_cck_impact_matches_pinned_contdid() {
    use hirmos_causal_core::continuous_did::{bootstrap::Bands, cck};
    let f = fixture();
    let data = rows(&f["2"]["rows"]);
    let cases: Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/cck-impact.json")).unwrap();
    for (name, c) in cases.as_object().unwrap() {
        let bands = if name == "TRUE" {
            Bands::Simultaneous
        } else {
            Bands::Pointwise
        };
        let r = cck::impact(&data, 99, 0.05, bands, 314159).unwrap();
        let e = &c["result"];
        for (key, a) in [
            ("overall_att", r.overall_level),
            ("overall_att_se", r.overall_level_se),
            ("overall_acrt", r.overall_slope),
            ("overall_acrt_se", r.overall_slope_se),
            ("att.d_crit.val", r.level_critical),
            ("acrt.d_crit.val", r.slope_critical),
        ] {
            let e = e[key].as_f64().unwrap();
            assert!((a - e).abs() < 1e-8 * (1.0 + e.abs()), "{key}: {a} != {e}");
        }
        for (key, a) in [
            ("dose", r.dose),
            ("att.d", r.level),
            ("acrt.d", r.slope),
            ("att.d_se", r.level_se),
            ("acrt.d_se", r.slope_se),
            ("overall_att_inffunc", r.overall_level_influence),
            ("overall_acrt_inffunc", r.overall_slope_influence),
        ] {
            let expected = e[key].as_array().unwrap();
            assert_eq!(a.len(), expected.len(), "{key}");
            for (a, e) in a.iter().zip(expected) {
                let e = e.as_f64().or_else(|| e[0].as_f64()).unwrap();
                assert!((a - e).abs() < 1e-8 * (1.0 + e.abs()), "{key}: {a} != {e}");
            }
        }
        assert_eq!(r.upstream_mixed_pointwise, bands == Bands::Pointwise);
    }
}

#[test]
fn restricted_and_balanced_windows_match_r() {
    use hirmos_causal_core::continuous_did::{
        aggregation::{self, EventWindow, Target},
        bootstrap::Bands,
    };
    let f = fixture();
    let c = &f["4"];
    let fit = hirmos_causal_core::continuous_did::defaults::fit(&rows(&c["rows"]), 3, 0)
        .unwrap()
        .fit;
    let cases: Value = serde_json::from_str(include_str!("fixtures/continuous_did/windows.json")).unwrap();
    for c in cases.as_object().unwrap().values() {
        let w = &c["window"];
        let window = EventWindow::new(
            w["min_e"].as_i64(),
            w["max_e"].as_i64(),
            w["balance_e"].as_u64().map(|v| v as u32),
        )
        .unwrap();
        let target = if c["target"] == "level" {
            Target::Level
        } else {
            Target::Slope
        };
        let bands = if c["simultaneous"].as_bool().unwrap() {
            Bands::Simultaneous
        } else {
            Bands::Pointwise
        };
        let result =
            aggregation::estimate_window(&fit, target, bands, window, 99, 0.05, 314159).unwrap();
        for (key, s, prefix, suffix) in [
            ("group", result.group, "selective", "g"),
            ("event", result.event, "dynamic", "e"),
        ] {
            let e = &c["result"][key];
            compare(
                "window labels",
                &s.labels.iter().map(|v| *v as f64).collect::<Vec<_>>(),
                &e["egt"],
            );
            compare("window effects", &s.estimates, &e["att.egt"]);
            compare("window se", &s.standard_errors, &e["se.egt"]);
            close(
                "window overall",
                s.overall,
                e["overall.att"].as_f64().unwrap(),
            );
            close(
                "window overall se",
                s.overall_standard_error,
                e["overall.se"].as_f64().unwrap(),
            );
            close(
                "window critical",
                s.critical,
                e["crit.val.egt"].as_f64().unwrap(),
            );
            for (i, row) in s.influence.iter().enumerate() {
                compare(
                    "window influence",
                    row,
                    &e["inf.function"][format!("{prefix}.inf.func.{suffix}")][i],
                );
                close(
                    "window overall influence",
                    s.overall_influence[i],
                    e["inf.function"][format!("{prefix}.inf.func")][i][0]
                        .as_f64()
                        .unwrap(),
                );
            }
        }
    }
    assert!(EventWindow::new(Some(2), Some(1), None).is_err());
    for window in [
        EventWindow::new(Some(10), Some(12), None).unwrap(),
        EventWindow::new(None, None, Some(99)).unwrap(),
        EventWindow::new(Some(-2), Some(-1), None).unwrap(),
    ] {
        assert!(matches!(
            aggregation::estimate_window(
                &fit,
                Target::Slope,
                Bands::Simultaneous,
                window,
                99,
                0.05,
                1
            ),
            Err(aggregation::Error::EmptyEventWindow)
        ));
    }
}

#[test]
fn pointwise_and_simultaneous_options_match_r_across_seeds() {
    use hirmos_causal_core::continuous_did::{aggregation, bootstrap::Bands, dose};
    let fixtures = fixture();
    let options: Value = serde_json::from_str(include_str!("fixtures/continuous_did/options.json")).unwrap();
    for c in options.as_object().unwrap().values() {
        let data = &fixtures[c["periods"].as_u64().unwrap().to_string()];
        let fit = panel::fit(&rows(&data["rows"]), &vector(&data["grid"]), 3, vec![]).unwrap();
        let bands = if c["simultaneous"].as_bool().unwrap() {
            Bands::Simultaneous
        } else {
            Bands::Pointwise
        };
        let iterations = c["iterations"].as_u64().unwrap() as usize;
        let alpha = c["alpha"].as_f64().unwrap();
        let seed = c["seed"].as_u64().unwrap() as u32;
        let result = dose::infer_with_bands(&fit, bands, iterations, alpha, seed).unwrap();
        let e = &c["result"]["dose"];
        assert_eq!(result.bands, bands);
        close(
            "overall level se",
            result.overall_level_standard_error,
            e["overall_att_se"].as_f64().unwrap(),
        );
        close(
            "overall slope se",
            result.overall_slope_standard_error,
            e["overall_acrt_se"].as_f64().unwrap(),
        );
        compare(
            "level curve se",
            &result.level_curve.standard_errors,
            &e["att.d_se"],
        );
        compare(
            "slope curve se",
            &result.slope_curve.standard_errors,
            &e["acrt.d_se"],
        );
        close(
            "level critical",
            result.level_critical,
            e["att.d_crit.val"].as_f64().unwrap(),
        );
        close(
            "slope critical",
            result.slope_critical,
            e["acrt.d_crit.val"].as_f64().unwrap(),
        );
        let result = aggregation::estimate(
            &fit,
            aggregation::Target::Slope,
            bands,
            iterations,
            alpha,
            seed,
        )
        .unwrap();
        for (key, result) in [("group", result.group), ("event", result.event)] {
            let e = &c["result"][key];
            compare("aggregate se", &result.standard_errors, &e["se.egt"]);
            close(
                "aggregate overall se",
                result.overall_standard_error,
                e["overall.se"].as_f64().unwrap(),
            );
            close(
                "aggregate critical",
                result.critical,
                e["crit.val.egt"].as_f64().unwrap(),
            );
        }
    }
}
fn vector(v: &Value) -> Vec<f64> {
    if let Some(value) = v.as_f64() {
        return vec![value];
    }
    v.as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_f64().unwrap())
        .collect()
}

#[test]
fn event_and_cohort_level_and_slope_aggregation_and_seeded_inference_match_r() {
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/aggregation.json")).unwrap();
    for (name, case) in fixture().as_object().unwrap() {
        let fit = panel::fit(&rows(&case["rows"]), &vector(&case["grid"]), 3, vec![]).unwrap();
        for (reference, result) in [
            (
                name.clone(),
                hirmos_causal_core::continuous_did::aggregation::slope(&fit, 99, 0.05, 314159).unwrap(),
            ),
            (
                format!("{name}-level"),
                hirmos_causal_core::continuous_did::aggregation::level(&fit, 99, 0.05, 314159).unwrap(),
            ),
        ] {
            for (key, s, prefix) in [
                ("group", &result.group, "selective"),
                ("dynamic", &result.event, "dynamic"),
            ] {
                let e = &expected[&reference]["result"][key];
                compare(
                    "labels",
                    &s.labels.iter().map(|v| *v as f64).collect::<Vec<_>>(),
                    &e["egt"],
                );
                compare("estimates", &s.estimates, &e["att.egt"]);
                compare("standard errors", &s.standard_errors, &e["se.egt"]);
                close("critical", s.critical, e["crit.val.egt"].as_f64().unwrap());
                close("overall", s.overall, e["overall.att"].as_f64().unwrap());
                close(
                    "overall standard error",
                    s.overall_standard_error,
                    e["overall.se"].as_f64().unwrap(),
                );
                let component = if key == "group" { "g" } else { "e" };
                for (i, row) in s.influence.iter().enumerate() {
                    compare(
                        "aggregate influence",
                        row,
                        &e["inf.function"][format!("{prefix}.inf.func.{component}")][i],
                    );
                    close(
                        "overall influence",
                        s.overall_influence[i],
                        e["inf.function"][format!("{prefix}.inf.func")][i][0]
                            .as_f64()
                            .unwrap(),
                    );
                }
            }
        }
    }
}
fn rows(v: &Value) -> Vec<Row> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|r| Row {
            observation: Observation {
                unit: Unit(r["id"].as_u64().unwrap()),
                period: Period(r["time_period"].as_i64().unwrap()),
                adoption: match r["G"].as_i64().unwrap() {
                    0 => Adoption::Never,
                    g => Adoption::At(Period(g)),
                },
                outcome: r["Y"].as_f64().unwrap(),
            },
            dose: r["D"].as_f64().unwrap(),
        })
        .collect()
}
fn close(label: &str, a: f64, e: f64) {
    assert!(
        (a - e).abs() <= 1e-10 * (1.0 + e.abs()),
        "{label}: {a} != {e}"
    );
}
fn compare(label: &str, a: &[f64], e: &Value) {
    let e = vector(e);
    assert_eq!(a.len(), e.len(), "{label}");
    for (a, e) in a.iter().zip(e) {
        close(label, *a, e);
    }
}

#[test]
fn panel_samples_and_dose_aggregation_match_r() {
    for (name, case) in fixture().as_object().unwrap() {
        let automatic = hirmos_causal_core::continuous_did::defaults::fit(&rows(&case["rows"]), 3, 0).unwrap();
        compare("automatic dose grid", &automatic.design.grid, &case["grid"]);
        compare(
            "automatic level curve",
            &automatic.fit.level,
            &case["result"]["att.d"],
        );
        compare(
            "automatic slope curve",
            &automatic.fit.slope,
            &case["result"]["acrt.d"],
        );
        let fit = panel::fit(
            &rows(&case["rows"]),
            &vector(&case["grid"]),
            3,
            vector(&case["knots"]),
        )
        .unwrap();
        let expected = case["cells"].as_array().unwrap();
        assert_eq!(fit.cells.len(), expected.len());
        for (sample, e) in fit.cells.iter().zip(expected) {
            assert_eq!(sample.group.0, e["group"].as_i64().unwrap());
            assert_eq!(sample.time.0, e["time"].as_i64().unwrap());
            assert_eq!(sample.units.len(), e["n1"].as_u64().unwrap() as usize);
            let ids: Vec<_> = e["unit"]
                .as_array()
                .unwrap()
                .iter()
                .map(|id| Unit(id.as_u64().unwrap()))
                .collect();
            assert_eq!(sample.units, ids);
            compare("cell dose", &sample.dose, &e["dose"]);
            compare("cell change", &sample.change, &e["change"]);
        }
        for (i, row) in fit.raw_influence.iter().enumerate() {
            compare("raw influence", row, &case["raw_influence"][i]);
        }
        let e = &case["result"];
        let inference = hirmos_causal_core::continuous_did::dose::infer(&fit, 99, 0.05, 314159).unwrap();
        close(
            "overall level inference estimate",
            inference.overall_level,
            e["overall_att"].as_f64().unwrap(),
        );
        close(
            "overall level se",
            inference.overall_level_standard_error,
            e["overall_att_se"].as_f64().unwrap(),
        );
        close(
            "overall slope se",
            inference.overall_slope_standard_error,
            e["overall_acrt_se"].as_f64().unwrap(),
        );
        for (j, v) in inference.overall_level_influence.iter().enumerate() {
            close(
                "overall level influence",
                *v,
                e["overall_att_inffunc"][j][0].as_f64().unwrap(),
            );
        }
        compare(
            "level curve se",
            &inference.level_curve.standard_errors,
            &e["att.d_se"],
        );
        compare(
            "slope curve se",
            &inference.slope_curve.standard_errors,
            &e["acrt.d_se"],
        );
        close(
            "level curve critical",
            inference.level_curve.critical,
            e["att.d_crit.val"].as_f64().unwrap(),
        );
        close(
            "slope curve critical",
            inference.slope_curve.critical,
            e["acrt.d_crit.val"].as_f64().unwrap(),
        );
        close(name, fit.average_level, e["overall_att"].as_f64().unwrap());
        close(name, fit.average_slope, e["overall_acrt"].as_f64().unwrap());
        compare("level curve", &fit.level, &e["att.d"]);
        compare("slope curve", &fit.slope, &e["acrt.d"]);
        for (i, row) in fit.level_influence.iter().enumerate() {
            compare("level curve influence", row, &e["att.d_inffunc"][i]);
        }
        for (i, row) in fit.slope_influence.iter().enumerate() {
            compare("slope curve influence", row, &e["acrt.d_inffunc"][i]);
        }
        for (i, actual) in fit.average_slope_influence.iter().enumerate() {
            close(
                "overall slope influence",
                *actual,
                e["overall_acrt_inffunc"][i][0].as_f64().unwrap(),
            );
        }
    }
}

#[test]
fn keyed_preparation_is_order_independent_and_rejects_changing_doses() {
    let f = fixture();
    let case = &f["4"];
    let mut data = rows(&case["rows"]);
    let grid = vector(&case["grid"]);
    let a = panel::fit(&data, &grid, 3, vec![]).unwrap();
    data.reverse();
    let b = panel::fit(&data, &grid, 3, vec![]).unwrap();
    assert_eq!(a.level, b.level);
    assert_eq!(a.raw_influence, b.raw_influence);
    let treated = data.iter_mut().find(|r| r.dose > 0.0).unwrap();
    treated.dose *= 0.5;
    assert!(matches!(
        panel::fit(&data, &grid, 3, vec![]),
        Err(panel::Error::DoseChanged(_))
    ));
    let mut data = rows(&case["rows"]);
    data.pop();
    assert!(matches!(
        panel::fit(&data, &grid, 3, vec![]),
        Err(panel::Error::Panel(
            hirmos_causal_core::staggered_did::Error::Unbalanced { .. }
        ))
    ));
}
