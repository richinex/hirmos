use hirmos_causal_core::estimation::inference::{joint, JointTest, Reference};
use hirmos_causal_core::estimation::{weighted_within_convention, WithinConvention, WithinErrors};
use hirmos_causal_core::panel_design::{EventTails, EventWindow, Panel};
use nalgebra::DMatrix;
use serde_json::Value;
use std::collections::BTreeMap;

#[test]
fn castle_matches_lfe() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/mixtape_lfe.json")).unwrap();
    for name in [
        "castle_static",
        "castle_simple",
        "synthetic_nested",
        "synthetic_crossed",
        "synthetic_time_cluster",
        "castle_events",
        "synthetic_events_reference",
        "synthetic_events_bin",
    ] {
        let f = &root[name];
        let y: Vec<f64> = serde_json::from_value(f["y"].clone()).unwrap();
        let weights: Vec<f64> = serde_json::from_value(f["weights"].clone()).unwrap();
        let units: Vec<u64> = serde_json::from_value(f["units"].clone()).unwrap();
        let times: Vec<u64> = serde_json::from_value(f["times"].clone()).unwrap();
        let clusters: Vec<u64> = serde_json::from_value(f["cluster_ids"].clone()).unwrap();
        let rows: Vec<Vec<f64>> = serde_json::from_value(f["x"].clone()).unwrap();
        let mut x = DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j]);
        if !f["window"].is_null() {
            let labels: Vec<String> = serde_json::from_value(f["terms"].clone()).unwrap();
            let covariates: Vec<String> = serde_json::from_value(f["covariates"].clone()).unwrap();
            let selected: Vec<usize> = covariates
                .iter()
                .map(|c| labels.iter().position(|l| l == c).unwrap())
                .collect();
            let adoption: BTreeMap<u64, Option<i64>> = f["adoption"]
                .as_array()
                .unwrap()
                .iter()
                .map(|a| (a["unit"].as_u64().unwrap(), a["time"].as_i64()))
                .collect();
            // Deliberately reverse source rows: row identities must survive sorting.
            let keys: Vec<_> = units
                .iter()
                .zip(&times)
                .rev()
                .map(|(u, t)| (*u, *t as i64))
                .collect();
            let source = DMatrix::from_fn(x.nrows(), selected.len(), |i, j| {
                x[(x.nrows() - 1 - i, selected[j])]
            });
            let panel = Panel::new(&keys, source).unwrap();
            let w = &f["window"];
            let design = panel
                .pooled_events(
                    &adoption,
                    &(0..selected.len()).collect::<Vec<_>>(),
                    EventWindow {
                        first: w["first"].as_i64().unwrap(),
                        last: w["last"].as_i64().unwrap(),
                        reference: w["reference"].as_i64().unwrap(),
                        tails: if w["tails"] == "bin" {
                            EventTails::Bin
                        } else {
                            EventTails::Reference
                        },
                    },
                )
                .unwrap();
            let mut design_labels = covariates;
            design_labels.extend(design.event_periods.iter().map(|e| {
                if *e < 0 {
                    format!("lead{}", -e)
                } else {
                    format!("lag{e}")
                }
            }));
            let mut actual = DMatrix::zeros(x.nrows(), x.ncols());
            for (i, original) in design.original_rows.iter().enumerate() {
                let target = x.nrows() - 1 - original;
                for (j, label) in labels.iter().enumerate() {
                    actual[(target, j)] =
                        design.matrix[(i, design_labels.iter().position(|l| l == label).unwrap())];
                }
            }
            assert_eq!(
                actual, x,
                "{name}: generated design must equal the R design"
            );
            assert!(design.support.iter().all(|n| *n > 0));
            x = actual;
        }
        let fit = weighted_within_convention(
            &y,
            &x,
            &units,
            Some(&times),
            &weights,
            WithinErrors::ClusterBy(&clusters),
            WithinConvention::Lfe,
        )
        .unwrap();
        assert_eq!(fit.inference_df as u64, f["df"].as_u64().unwrap());
        assert_eq!(
            fit.kept.len(),
            x.ncols(),
            "all reference terms are estimable"
        );
        assert_eq!(
            fit.df_resid as u64,
            f["n"].as_u64().unwrap() - f["parameters"].as_u64().unwrap()
        );
        for (i, &source) in fit.kept.iter().enumerate() {
            for (actual, key) in [
                (fit.params[i], "params"),
                (fit.bse[i], "se"),
                (fit.pvalues[i], "pvalues"),
            ] {
                let expected = f[key][source].as_f64().unwrap();
                assert!(
                    (actual - expected).abs() <= 1e-8 * expected.abs().max(1.0),
                    "{name} {key} term {source}: {actual} != {expected}"
                );
            }
            for (actual, col) in [(fit.conf_int[i].0, 0), (fit.conf_int[i].1, 1)] {
                let expected = f["interval"][source][col].as_f64().unwrap();
                assert!(
                    (actual - expected).abs() <= 1e-8 * expected.abs().max(1.0),
                    "{name} interval {source}: {actual} != {expected}"
                );
            }
            for (j, &other) in fit.kept.iter().enumerate() {
                let expected = f["covariance"][source][other].as_f64().unwrap();
                assert!(
                    (fit.covariance[(i, j)] - expected).abs() <= 1e-8 * expected.abs().max(1.0),
                    "{name} covariance {source},{other}"
                );
            }
        }
        println!("{name}: kept {} of {} terms", fit.kept.len(), x.ncols());
        if !f["joint"].is_null() {
            let indices: Vec<usize> =
                serde_json::from_value(f["joint"]["indices"].clone()).unwrap();
            let r = DMatrix::from_fn(indices.len(), fit.params.len(), |i, j| {
                f64::from(indices[i] == j)
            });
            let test = joint(
                &fit.params,
                &fit.covariance,
                &r,
                &vec![0.0; indices.len()],
                Reference::Student {
                    degrees_of_freedom: fit.inference_df as f64,
                },
            )
            .unwrap();
            let JointTest::F {
                statistic, p_value, ..
            } = test
            else {
                panic!("wrong reference")
            };
            for (actual, key) in [(statistic, "statistic"), (p_value, "p")] {
                let expected = f["joint"][key].as_f64().unwrap();
                assert!(
                    (actual - expected).abs() <= 1e-8 * expected.abs().max(1.0),
                    "{name} joint {key}: {actual} != {expected}"
                );
            }
            for i in 0..fit.params.len() {
                for (actual, col) in [
                    (fit.params[i] - 1.96 * fit.bse[i], 0),
                    (fit.params[i] + 1.96 * fit.bse[i], 1),
                ] {
                    let expected = f["plot_interval"][i][col].as_f64().unwrap();
                    assert!((actual - expected).abs() <= 1e-8 * expected.abs().max(1.0));
                }
            }
        }
    }
}
