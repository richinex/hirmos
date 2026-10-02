//! Estimator facades, split by the estimator families the Estimation chapter offers.

use super::*;

pub(crate) use hirmos_causal_core::causal_effects::numpy_percentile;
pub(crate) use hirmos_causal_core::frontdoor::{
    frontdoor_two_stage_with_progress, FrontdoorInput, FrontdoorOptions,
};
pub(crate) use hirmos_causal_core::arma_regression::{fit as fit_arma_regression, ArmaOrder, ArmaRegressionFit};
pub(crate) use hirmos_causal_core::{
    fit_tlearner, group_effects, instrumental_variable_with_progress, DowhyBootstrap, IvEstimator,
    IvInput, IvOptions,
};

mod adjusted_outcome;
mod propensity_score;
mod identified_functional;
mod graph_adjusted_temporal;
mod dynamic_time_series;
mod intervention_comparison;
mod time_series;

pub(crate) use adjusted_outcome::*;
pub(crate) use propensity_score::*;
pub(crate) use identified_functional::*;
pub(crate) use graph_adjusted_temporal::*;
pub(crate) use dynamic_time_series::*;
pub(crate) use intervention_comparison::*;
pub(crate) use time_series::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// The chapter's weighting estimator through the browser-facing command, on a design the test
    /// builds itself: a confounder shifts both the treatment and the outcome, so weighting has to
    /// recover the planted effect that a raw comparison overstates.
    #[test]
    fn propensity_weighting_recovers_a_planted_effect() {
        // The treatment model is logistic, so the true propensity is logistic too and the fit is
        // correctly specified. The draw comes from the seeded stream, independent of the outcome.
        let rows = 4000usize;
        let mut rng = hirmos_causal_core::nprandom::Mt19937::seeded(7);
        let confounder: Vec<f64> = (0..rows).map(|row| ((row * 7) % 11) as f64 / 10.0).collect();
        let treated: Vec<f64> = (0..rows)
            .map(|row| {
                let raw = -1.0 + 2.5 * confounder[row];
                f64::from(rng.next_f64() < 1.0 / (1.0 + (-raw).exp()))
            })
            .collect();
        let outcome: Vec<f64> = (0..rows)
            .map(|row| 1.5 * treated[row] + 3.0 * confounder[row])
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend_from_slice(&treated);
        values.extend_from_slice(&outcome);
        values.extend_from_slice(&confounder);

        let naive = {
            let mean = |want: f64| {
                let rows: Vec<usize> = (0..rows).filter(|&r| treated[r] == want).collect();
                rows.iter().map(|&r| outcome[r]).sum::<f64>() / rows.len() as f64
            };
            mean(1.0) - mean(0.0)
        };
        assert!(naive > 1.8, "the confounder should bias the raw comparison, got {naive}");

        let result = propensity_weighting(PropensityTarget::Ate, 
            &values, rows, 3, 0, 1, &[2],
            PropensityWeightScale::InverseProbability,
            WeightingFit::Logistic { model: LogisticModel::Newton, bootstrap: None },
        )
        .expect("the design weights");
        let json = serde_json::to_value(&result).expect("the evidence serialises");
        assert_eq!(json["kind"], "propensityWeighting");
        assert_eq!(json["observations"], rows);
        assert_eq!(json["treatmentModel"]["converged"], true);
        let estimate = json["estimate"].as_f64().expect("an estimate");
        assert!((estimate - 1.5).abs() < 0.05, "weighting should recover 1.5, got {estimate}");
        assert_eq!(json["propensity"].as_array().expect("scores").len(), rows);

        // The two treatment models differ only by where each stops, so a design handed to one of
        // them wrongly shows up here. Fitting the Newton model without its intercept column gave
        // 1.356 against 1.500, which is a plausible number rather than an error.
        let sklearn = propensity_weighting(PropensityTarget::Ate, 
            &values, rows, 3, 0, 1, &[2],
            PropensityWeightScale::InverseProbability,
            WeightingFit::Logistic { model: LogisticModel::Lbfgsb { max_iter: 1000 }, bootstrap: None },
        )
        .expect("the design weights");
        let other = serde_json::to_value(&sklearn).expect("the evidence serialises");
        let apart = (estimate - other["estimate"].as_f64().expect("an estimate")).abs();
        assert!(apart < 5e-3, "the two treatment models should agree, {apart:.3e} apart");

        // Stabilizing rescales both arms by the prevalence, which leaves this estimand alone.
        let stabilized = propensity_weighting(PropensityTarget::Ate, 
            &values, rows, 3, 0, 1, &[2],
            PropensityWeightScale::Stabilized,
            WeightingFit::Logistic { model: LogisticModel::Newton, bootstrap: None },
        )
        .expect("the design weights");
        let scaled = serde_json::to_value(&stabilized).expect("the evidence serialises");
        let moved = (estimate - scaled["estimate"].as_f64().expect("an estimate")).abs();
        assert!(moved < 1e-9, "stabilizing should not move this estimand, moved {moved:.3e}");
    }

    /// A propensity fitted by boosted trees chosen on cross-validated ROC AUC. The planted effect
    /// is the same 1.5, so the boosted score is read against the logistic one.
    #[test]
    fn boosted_treatment_model_recovers_the_same_planted_effect() {
        let (values, rows) = confounded_sample();
        let boosted = BoostedTreatmentModel {
            learning_rate: vec![0.1, 0.15],
            max_depth: vec![2, 3],
            n_estimators: vec![50, 100],
            splits: 5,
            min_samples_leaf: 1,
            min_samples_split: 2,
            seed: 7,
            scoring: BoostedScoring::OneModel,
            candidates_searched: None,
        };
        let result = propensity_weighting(PropensityTarget::Ate, 
            &values, rows, 3, 0, 1, &[2],
            PropensityWeightScale::InverseProbability,
            WeightingFit::Boosted { model: boosted.clone() },
        )
        .expect("the boosted design weights");
        let json = serde_json::to_value(&result).expect("the evidence serialises");
        assert_eq!(json["treatmentModel"]["kind"], "boosted");
        assert_eq!(json["treatmentModel"]["candidates"], 8);
        assert_eq!(json["treatmentModel"]["scoring"]["kind"], "oneModel");
        let auc = json["treatmentModel"]["scoring"]["validationAuc"].as_f64().expect("an auc");
        assert!(auc > 0.5, "the search should beat chance on a confounded design, got {auc}");
        let estimate = json["estimate"].as_f64().expect("an estimate");
        assert!((estimate - 1.5).abs() < 0.2, "boosted weighting should recover 1.5, got {estimate}");
        assert!(json["interval"].is_null(), "a boosted fit carries no bootstrap interval");

        // Cross-fitting scores every row from the half that did not contain it, so the scores
        // change but the estimand does not.
        let split = propensity_weighting(PropensityTarget::Ate, 
            &values, rows, 3, 0, 1, &[2],
            PropensityWeightScale::InverseProbability,
            WeightingFit::Boosted { model: BoostedTreatmentModel { scoring: BoostedScoring::CrossFitted, ..boosted } },
        )
        .expect("the cross-fitted design weights");
        let crossed = serde_json::to_value(&split).expect("the evidence serialises");
        assert_eq!(crossed["treatmentModel"]["scoring"]["kind"], "crossFitted");
        assert!(crossed["treatmentModel"]["scoring"]["validationAuc"].is_null(), "cross-fitting reports no search AUC it did not compute");
        assert_eq!(crossed["propensity"].as_array().expect("scores").len(), rows);
        let apart = (estimate - crossed["estimate"].as_f64().expect("an estimate")).abs();
        assert!(apart < 0.4, "cross-fitting should not move the estimand far, {apart:.3e} apart");
    }

    /// The same confounded design as the weighting test, so the three binary-treatment estimators
    /// are read against one planted effect rather than against each other.
    fn confounded_sample() -> (Vec<f64>, usize) {
        let rows = 4000usize;
        let mut rng = hirmos_causal_core::nprandom::Mt19937::seeded(7);
        let confounder: Vec<f64> = (0..rows).map(|row| ((row * 7) % 11) as f64 / 10.0).collect();
        let treated: Vec<f64> = (0..rows)
            .map(|row| {
                let raw = -1.0 + 2.5 * confounder[row];
                f64::from(rng.next_f64() < 1.0 / (1.0 + (-raw).exp()))
            })
            .collect();
        let outcome: Vec<f64> = (0..rows)
            .map(|row| 1.5 * treated[row] + 3.0 * confounder[row])
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend_from_slice(&treated);
        values.extend_from_slice(&outcome);
        values.extend_from_slice(&confounder);
        (values, rows)
    }

    #[test]
    fn propensity_matching_recovers_a_planted_effect() {
        let (values, rows) = confounded_sample();
        let result = propensity_matching(PropensityTarget::Ate, &values, rows, 3, 0, 1, &[2], PropensityModel::Logistic { model: LogisticModel::Newton })
            .expect("the design matches");
        let json = serde_json::to_value(&result).expect("the evidence serialises");
        assert_eq!(json["kind"], "propensityMatching");
        assert_eq!(json["matches"].as_array().expect("matches").len(), rows);
        assert_eq!(
            json["treatedRows"].as_u64().unwrap_or_default()
                + json["controlRows"].as_u64().unwrap_or_default(),
            rows as u64
        );
        let estimate = json["estimate"].as_f64().expect("an estimate");
        assert!((estimate - 1.5).abs() < 0.1, "matching should recover 1.5, got {estimate}");
    }

    #[test]
    fn doubly_robust_recovers_a_planted_effect_and_reports_its_halves() {
        let (values, rows) = confounded_sample();
        let result = doubly_robust_estimate(PropensityTarget::Ate, 
            &values, rows, 3, 0, 1, &[2], LogisticModel::Newton, None,
        )
        .expect("the design fits");
        let json = serde_json::to_value(&result).expect("the evidence serialises");
        assert_eq!(json["kind"], "doublyRobust");
        let estimate = json["estimate"].as_f64().expect("an estimate");
        assert!((estimate - 1.5).abs() < 0.05, "AIPW should recover 1.5, got {estimate}");
        let halves = json["treatedTerm"].as_f64().expect("a treated term")
            - json["controlTerm"].as_f64().expect("a control term");
        assert!((halves - estimate).abs() < 1e-12, "the halves should sum to the estimate");
        assert_eq!(json["interval"], serde_json::Value::Null, "no rounds were asked for");
    }

    /// A continuous treatment with a known slope: the raw regression is pulled upward by the
    /// covariate, and weighting by the conditional density pulls it back.
    #[test]
    fn continuous_gps_recovers_a_planted_slope() {
        let rows = 4000usize;
        let mut rng = hirmos_causal_core::nprandom::Mt19937::seeded(11);
        let covariate: Vec<f64> = (0..rows).map(|_| rng.next_f64()).collect();
        let dose: Vec<f64> = (0..rows)
            .map(|row| 2.0 * covariate[row] + rng.standard_normal(&mut None))
            .collect();
        let outcome: Vec<f64> = (0..rows)
            .map(|row| -0.8 * dose[row] + 3.0 * covariate[row])
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend_from_slice(&dose);
        values.extend_from_slice(&outcome);
        values.extend_from_slice(&covariate);

        let result = continuous_gps(
            &values, rows, 3, 0, 1, &[2], GpsWeightScale::Stabilized, None,
        )
        .expect("the design weights");
        let json = serde_json::to_value(&result).expect("the evidence serialises");
        assert_eq!(json["kind"], "continuousGps");
        assert_eq!(json["density"].as_array().expect("densities").len(), rows);
        assert_eq!(json["weights"].as_array().expect("weights").len(), rows);
        let estimate = json["estimate"].as_f64().expect("an estimate");
        assert!((estimate + 0.8).abs() < 0.1, "GPS should recover -0.8, got {estimate}");
        // Stabilized weights sum to about the sample size, which is how the chapter reads them.
        let sum = json["weightSum"].as_f64().expect("a weight sum");
        assert!((sum - rows as f64).abs() < rows as f64 * 0.05, "weights sum to {sum}");
    }

    #[test]
    fn propensity_weighting_refuses_input_it_cannot_use() {
        let values = vec![0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 0.1, 0.2, 0.3, 0.4];
        let newton = || WeightingFit::Logistic { model: LogisticModel::Newton, bootstrap: None };
        assert!(propensity_weighting(PropensityTarget::Ate, &values, 4, 3, 0, 1, &[],
            PropensityWeightScale::Stabilized, newton()).is_err(), "no adjustment columns");
        assert!(propensity_weighting(PropensityTarget::Ate, &values, 4, 3, 0, 0, &[2],
            PropensityWeightScale::Stabilized, newton()).is_err(), "treatment repeated as outcome");
        assert!(propensity_weighting(PropensityTarget::Ate, &values, 4, 3, 1, 0, &[2],
            PropensityWeightScale::Stabilized, newton()).is_err(), "treatment column is not 0/1");
    }

    /// The browser command against scikit-learn: each arm's candidate chosen once on all of its rows,
    /// refitted per half, every row's effect and the ATE, through the matrix layout and the
    /// reordering back to row order.
    #[test]
    fn cross_fitted_t_learner_command_reproduces_scikit_learn() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../causal-core/oracle/fixtures/sklearn_tlearner_global_selection.json"
        ))
        .unwrap();
        let rows = fixture["rows"].as_u64().unwrap() as usize;
        let width = fixture["columns"].as_u64().unwrap() as usize;
        let confounder = |row: usize, column: usize| ((row * (column + 3) + column * column) % 29) as f64;
        let treatment: Vec<f64> = (0..rows).map(|r| f64::from((confounder(r, 0) + confounder(r, 2)).rem_euclid(7.0) > 3.0)).collect();
        let death: Vec<f64> = (0..rows).map(|r| f64::from((confounder(r, 1) * 2.0 + treatment[r] * 5.0).rem_euclid(11.0) > 4.0)).collect();
        let mut values = treatment.clone();
        values.extend(&death);
        for column in 0..width { values.extend((0..rows).map(|row| confounder(row, column))); }
        let axis = |name: &str| fixture["grid"][name].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect::<Vec<_>>();
        let grid = || hirmos_causal_core::model_selection::Grid {
            learning_rate: axis("learning_rate"),
            max_depth: axis("max_depth").into_iter().map(|v| v as usize).collect(),
            n_estimators: axis("n_estimators").into_iter().map(|v| v as usize).collect(),
        };
        let adjustment: Vec<usize> = (2..2 + width).collect();
        let leaf = fixture["min_samples_leaf"].as_u64().unwrap() as usize;
        let seed = fixture["seed"].as_u64().unwrap() as u32;
        let chosen = |arm: &str| {
            let want = &fixture["selected"][arm];
            ChosenArm {
                learning_rate: want["learning_rate"].as_f64().unwrap(),
                max_depth: want["max_depth"].as_u64().unwrap() as usize,
                n_estimators: want["n_estimators"].as_u64().unwrap() as usize,
                validation_auc: fixture["validation_auc"][arm].as_f64().unwrap(),
            }
        };
        // The kernel's own search, and the same choice handed in from a search run elsewhere.
        let selections = [ArmSelection::Search, ArmSelection::Chosen { treated: chosen("treated"), control: chosen("control") }];
        for selection in selections {
            let result = cross_fitted_t_learner(&values, rows, 2 + width, 0, 1, &adjustment, grid(), 5, leaf, 2, seed, selection).unwrap();
            let AnalysisResult::CrossFittedTLearner { effects, average, selected, candidates, .. } = result else { panic!("wrong result") };
            assert_eq!(candidates, 8);
            assert!((average - fixture["ate"].as_f64().unwrap()).abs() < 1e-12, "ATE {average}");
            let order: Vec<usize> = fixture["order"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect();
            let expected: Vec<f64> = fixture["effects"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            for (at, &row) in order.iter().enumerate() {
                assert!((effects[row] - expected[at]).abs() < 1e-12, "row {row}");
            }
            for (mine, arm) in [(&selected.treated, "treated"), (&selected.control, "control")] {
                let want = &fixture["selected"][arm];
                assert_eq!(mine.learning_rate, want["learning_rate"].as_f64().unwrap(), "{arm}");
                assert_eq!(mine.max_depth, want["max_depth"].as_u64().unwrap() as usize, "{arm}");
                assert_eq!(mine.n_estimators, want["n_estimators"].as_u64().unwrap() as usize, "{arm}");
                assert!((mine.validation_auc - fixture["validation_auc"][arm].as_f64().unwrap()).abs() < 1e-12, "{arm} AUC");
            }
        }
        let outside = ChosenArm { learning_rate: 0.5, ..chosen("treated") };
        assert!(cross_fitted_t_learner(&values, rows, 2 + width, 0, 1, &adjustment, grid(), 5, leaf, 2, seed,
            ArmSelection::Chosen { treated: outside, control: chosen("control") }).is_err(), "a candidate outside the grid is refused");
        assert!(cross_fitted_t_learner(&values, rows, 2 + width, 2, 1, &adjustment[1..], grid(), 5, leaf, 2, seed, ArmSelection::Search).is_err(),
            "a non-binary treatment is refused");
    }

    #[test]
    fn negative_binomial_ingarch_facade_matches_the_tscount_forecast() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../causal-core/oracle/fixtures/ingarch.json"
        ))
        .unwrap();
        let observations: Vec<f64> =
            serde_json::from_value(fixture["observations"].clone()).unwrap();
        let regressors: Vec<Vec<f64>> =
            serde_json::from_value(fixture["regressors"].clone()).unwrap();
        let rows = observations.len();
        let mut values = observations;
        values.extend(regressors.iter().map(|row| row[0]));
        values.extend(regressors.iter().map(|row| row[1]));
        let progress = std::cell::RefCell::new(Vec::new());
        let result = negative_binomial_ingarch(
            &values,
            rows,
            3,
            0,
            IngarchLink::Log,
            &[1, 2],
            &[1],
            &[1],
            &[false, false],
            10,
            &[2.0, 0.15],
            1,
            2.0,
            5.0,
            IngarchInterventionSchedule::Persistent,
            |stage, completed, total| progress.borrow_mut().push((stage, completed, total)),
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["kind"], "negativeBinomialIngarch");
        assert_eq!(encoded["horizon"], 10);
        assert_eq!(encoded["schedule"]["kind"], "persistent");
        let expected = fixture["scenarios"]["persistent"]["mean"]
            .as_array()
            .unwrap();
        for (actual, expected) in encoded["interventionMean"]
            .as_array()
            .unwrap()
            .iter()
            .zip(expected)
        {
            assert!((actual.as_f64().unwrap() - expected.as_f64().unwrap()).abs() < 1e-8);
        }
        assert_eq!(progress.borrow().last(), Some(&("complete", 2, 2)));
    }

    #[test]
    fn identity_ingarch_facade_matches_tscount_default_forecast() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../causal-core/oracle/fixtures/ingarch_identity.json"
        ))
        .unwrap();
        let observations: Vec<f64> =
            serde_json::from_value(fixture["observations"].clone()).unwrap();
        let regressors: Vec<Vec<f64>> =
            serde_json::from_value(fixture["regressors"].clone()).unwrap();
        let rows = observations.len();
        let mut values = observations;
        values.extend(regressors.iter().map(|row| row[0]));
        let result = negative_binomial_ingarch(
            &values,
            rows,
            2,
            0,
            IngarchLink::Identity,
            &[1],
            &[1],
            &[1],
            &[false],
            12,
            &[0.0],
            1,
            0.0,
            1.0,
            IngarchInterventionSchedule::Persistent,
            |_, _, _| {},
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["link"], "identity");
        for (actual, expected) in encoded["interventionMean"]
            .as_array()
            .unwrap()
            .iter()
            .zip(fixture["scenarios"]["treated"]["mean"].as_array().unwrap())
        {
            assert!((actual.as_f64().unwrap() - expected.as_f64().unwrap()).abs() < 1e-7);
        }
    }

    #[test]
    fn multi_lag_count_series_scan_matches_tscount_and_reports_progress() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../causal-core/oracle/fixtures/ingarch_detection.json"
        ))
        .unwrap();
        let observations: Vec<f64> =
            serde_json::from_value(fixture["observations"].clone()).unwrap();
        let candidates: Vec<usize> =
            serde_json::from_value::<Vec<usize>>(fixture["detection"]["candidates"].clone())
                .unwrap()
                .into_iter()
                .map(|tau| tau - 1)
                .collect();
        let rows = observations.len();
        let progress = std::cell::RefCell::new(Vec::new());
        let result = count_series_intervention_scan(
            &observations,
            rows,
            1,
            0,
            IngarchLink::Identity,
            &[1],
            &[7, 13],
            &candidates,
            1.0,
            |stage, completed, total| progress.borrow_mut().push((stage, completed, total)),
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["kind"], "countSeriesInterventionScan");
        assert_eq!(encoded["strongestReferencePoint"].as_u64(), Some(83));
        let scores = encoded["candidates"].as_array().unwrap();
        let oracle = fixture["detection"]["score_statistics"].as_array().unwrap();
        for (actual, expected) in scores.iter().zip(oracle) {
            assert!(
                (actual["scoreStatistic"].as_f64().unwrap() - expected.as_f64().unwrap()).abs()
                    < 1e-7
            );
        }
        assert_eq!(
            progress.borrow().as_slice(),
            &[("fit-and-scan", 0, 1), ("complete", 1, 1)]
        );
    }

    #[test]
    fn backdoor_linear_serializes_both_intervals() {
        let rows = 120;
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let w: Vec<f64> = (0..rows).map(|_| uniform()).collect();
        let t: Vec<f64> = (0..rows).map(|i| 0.8 * w[i] + uniform()).collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 0.5 * t[i] + 0.7 * w[i] + uniform())
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend(&t);
        values.extend(&y);
        values.extend(&w);
        let json = backdoor_linear(&values, rows, 3, 0, 1, &[2], None, 0.95, LinearErrorModel::NeweyWest, None)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("estimate should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "backdoorLinear");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["parameters"], 3);
        assert_eq!(value["hacMaxLags"], 4);
        let estimate = value["estimate"].as_f64().unwrap();
        assert!(value["interval"][0].as_f64().unwrap() < estimate);
        assert!(value["hacInterval"][1].as_f64().unwrap() > estimate);
        assert!(value["hacPValue"].as_f64().unwrap() < 0.05);
        assert!(value["durbinWatson"].as_f64().unwrap() > 1.0);

        assert!(backdoor_linear(&values, rows, 3, 0, 0, &[2], None, 0.95, LinearErrorModel::NeweyWest, None).is_err());
        assert!(backdoor_linear(&values, rows, 3, 0, 1, &[2], Some(rows), 0.95, LinearErrorModel::NeweyWest, None).is_err());
        assert!(backdoor_linear(&values, rows, 3, 0, 1, &[2], None, 0.4, LinearErrorModel::NeweyWest, None).is_err());
    }

    #[test]
    fn frontdoor_two_stage_serializes_stages_interval_and_progress() {
        let rows = 80;
        let treatment: Vec<f64> = (0..rows).map(|row| (row % 7) as f64 - 3.0).collect();
        let mediator: Vec<f64> = treatment
            .iter()
            .enumerate()
            .map(|(row, treatment)| 4.0 + 2.0 * treatment + ((row * 3) % 5) as f64 * 0.01)
            .collect();
        let outcome: Vec<f64> = mediator
            .iter()
            .zip(&treatment)
            .map(|(mediator, treatment)| 8.0 + 3.0 * mediator + 5.0 * treatment)
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend(&treatment);
        values.extend(&mediator);
        values.extend(&outcome);
        let mut progress_events = Vec::new();
        let result = frontdoor_two_stage_evidence(
            &values,
            rows,
            3,
            0,
            1,
            2,
            &[],
            &[0],
            0.0,
            1.0,
            BootstrapUncertainty::Bootstrap {
                simulations: 20,
                sample_size_fraction: 1.0,
                confidence_level: 0.95,
                seed: 0,
            },
            |stage, completed, total| progress_events.push((stage, completed, total)),
        )
        .unwrap();
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(value["kind"], "frontdoorTwoStage");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["secondStageAdjustment"], serde_json::json!([0]));
        assert!((value["estimate"].as_f64().unwrap() - 6.0).abs() < 0.02);
        assert_eq!(value["uncertainty"]["kind"], "bootstrap");
        assert_eq!(value["uncertainty"]["simulations"], 20);
        assert_eq!(
            progress_events.first(),
            Some(&("frontdoor-bootstrap", 0, 20))
        );
        assert_eq!(
            progress_events.last(),
            Some(&("frontdoor-bootstrap", 20, 20))
        );

        assert!(frontdoor_two_stage_evidence(
            &values,
            rows,
            3,
            0,
            0,
            2,
            &[],
            &[0],
            0.0,
            1.0,
            BootstrapUncertainty::None,
            |_, _, _| {},
        )
        .is_err());
    }

    #[test]
    fn count_glm_reports_incidence_rate_ratios_for_both_families() {
        let rows = 150;
        let mut state = 0x1234_5678_9abc_def1u64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let w: Vec<f64> = (0..rows).map(|_| uniform() - 0.5).collect();
        let t: Vec<f64> = (0..rows)
            .map(|i| {
                if uniform() < 0.5 + 0.4 * w[i] {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| (3.0 + 2.0 * t[i] + w[i] + 4.0 * uniform()).floor())
            .collect();
        let mut values = Vec::new();
        values.extend(&t);
        values.extend(&y);
        values.extend(&w);
        for family in [CountFamily::Poisson, CountFamily::NegativeBinomial] {
            let json = count_glm(&values, rows, 3, 0, 1, &[2], family)
                .and_then(|result| {
                    serde_json::to_string(&result).map_err(|error| error.to_string())
                })
                .expect("count model should serialize");
            let value: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(value["kind"], "countGlm");
            assert!(value["incidenceRateRatio"].as_f64().unwrap() > 1.0);
            assert!(
                value["incidenceRateRatioInterval"][0].as_f64().unwrap()
                    < value["incidenceRateRatio"].as_f64().unwrap()
            );
            if matches!(family, CountFamily::Poisson) {
                assert_eq!(value["converged"], true);
            }
        }
        let mut fractional = values.clone();
        fractional[rows] = 2.5;
        assert!(count_glm(&fractional, rows, 3, 0, 1, &[2], CountFamily::Poisson).is_err());
    }

    #[test]
    fn causal_effects_total_identifies_and_predicts_a_linear_chain() {
        let rows = 400;
        let mut state = 0xdead_beef_cafe_f00du64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let mut x = vec![0.0; rows];
        let mut y = vec![0.0; rows];
        let mut z = vec![0.0; rows];
        for i in 1..rows {
            z[i] = 0.5 * z[i - 1] + uniform();
            x[i] = 0.6 * z[i - 1] + uniform();
            y[i] = 0.8 * x[i - 1] + 0.4 * z[i - 1] + uniform();
        }
        let mut values = Vec::new();
        values.extend(&x);
        values.extend(&y);
        values.extend(&z);
        let empty = String::new();
        let mut graph = vec![vec![vec![empty.clone(); 2]; 3]; 3];
        graph[2][2][1] = "-->".to_owned();
        graph[2][0][1] = "-->".to_owned();
        graph[0][1][1] = "-->".to_owned();
        graph[2][1][1] = "-->".to_owned();
        let json = causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[(0, -1)],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear {
                adjustment: CausalEffectsAdjustmentSelection::Optimal,
            },
            [0.0, 1.0],
            CausalEffectsUncertainty::None,
            |_, _, _| {},
        )
        .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
        .expect("total effect should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "causalEffectsTotal");
        assert_eq!(value["identifiable"], true);
        assert_eq!(value["fit"]["selection"]["kind"], "optimal");
        let effect = value["totalEffect"].as_f64().unwrap();
        assert!((effect - 0.8).abs() < 0.15, "total effect {effect}");

        let explicit: Vec<(usize, i32)> = value["fit"]["adjustmentSet"]
            .as_array()
            .unwrap()
            .iter()
            .map(|node| {
                (
                    node[0].as_u64().unwrap() as usize,
                    node[1].as_i64().unwrap() as i32,
                )
            })
            .collect();
        for adjustment in [
            CausalEffectsAdjustmentSelection::MinimizedOptimal,
            CausalEffectsAdjustmentSelection::CollidersMinimizedOptimal,
            CausalEffectsAdjustmentSelection::Explicit { nodes: explicit },
        ] {
            let result = causal_effects_total(
                &values,
                rows,
                3,
                1,
                &graph,
                &[(0, -1)],
                &[(1, 0)],
                &[],
                TotalEffectEstimator::Linear {
                    adjustment: adjustment.clone(),
                },
                [0.0, 1.0],
                CausalEffectsUncertainty::None,
                |_, _, _| {},
            )
            .expect("valid adjustment selection");
            let result = serde_json::to_value(result).unwrap();
            assert_eq!(result["identifiable"], true);
            assert_eq!(
                result["fit"]["selection"]["kind"],
                serde_json::to_value(adjustment).unwrap()["kind"]
            );
        }
        let invalid = causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[(0, -1)],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear {
                adjustment: CausalEffectsAdjustmentSelection::Explicit {
                    nodes: vec![(0, 0)],
                },
            },
            [0.0, 1.0],
            CausalEffectsUncertainty::None,
            |_, _, _| {},
        )
        .expect("an invalid explicit set is returned as structured evidence");
        let invalid = serde_json::to_value(invalid).unwrap();
        assert_eq!(invalid["identifiable"], false);
        assert_eq!(invalid["fit"]["kind"], "invalidAdjustment");
        assert_eq!(
            invalid["fit"]["requested"]["adjustment"]["nodes"][0],
            serde_json::json!([0, 0])
        );
        assert!(invalid["fit"]["problems"]
            .as_array()
            .unwrap()
            .iter()
            .any(|problem| problem["kind"] == "laterTreatmentOccurrence"));
        let progress_events = std::cell::RefCell::new(Vec::new());
        let bootstrap_json = causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[(0, -1)],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear {
                adjustment: CausalEffectsAdjustmentSelection::Optimal,
            },
            [0.0, 1.0],
            CausalEffectsUncertainty::Bootstrap {
                samples: 20,
                block_length: CausalEffectsBlockLength::Fixed { length: 4 },
                confidence_level: 0.9,
                seed: 4,
            },
            |stage, completed, total| progress_events.borrow_mut().push((stage, completed, total)),
        )
        .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
        .expect("bootstrap total effect should serialize");
        let bootstrap: serde_json::Value = serde_json::from_str(&bootstrap_json).unwrap();
        assert_eq!(bootstrap["uncertainty"]["kind"], "bootstrap");
        assert_eq!(bootstrap["uncertainty"]["samples"], 20);
        assert_eq!(bootstrap["uncertainty"]["resolvedBlockLength"], 4);
        assert_eq!(
            bootstrap["uncertainty"]["effectDraws"]
                .as_array()
                .unwrap()
                .len(),
            20
        );
        let events = progress_events.borrow();
        assert_eq!(events.first(), Some(&("causal-effects-bootstrap", 0, 20)));
        assert_eq!(events.last(), Some(&("causal-effects-bootstrap", 20, 20)));
        assert!(causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear {
                adjustment: CausalEffectsAdjustmentSelection::Optimal,
            },
            [0.0, 1.0],
            CausalEffectsUncertainty::None,
            |_, _, _| {},
        )
        .is_err());
    }

    #[test]
    fn causal_effects_wright_reports_path_decomposition_and_bootstrap() {
        let rows = 180;
        let z: Vec<f64> = (0..rows)
            .map(|row| (row as f64 * 0.31).sin() + (row % 7) as f64 * 0.03)
            .collect();
        let x: Vec<f64> = (0..rows)
            .map(|row| 0.6 * z[row] + (row as f64 * 0.73).cos() * 0.4)
            .collect();
        let mediator: Vec<f64> = (0..rows)
            .map(|row| 0.5 * x[row] + 0.3 * z[row] + (row as f64 * 0.47).sin() * 0.2)
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|row| {
                0.1 * x[row] + 0.4 * mediator[row] + 0.2 * z[row] + (row as f64 * 1.13).cos() * 0.25
            })
            .collect();
        let mut values = Vec::with_capacity(rows * 4);
        values.extend(&x);
        values.extend(&mediator);
        values.extend(&y);
        values.extend(&z);
        let mut graph = vec![vec![vec![String::new(); 1]; 4]; 4];
        for (source, target) in [(3, 0), (3, 1), (3, 2), (0, 1), (1, 2), (0, 2)] {
            graph[source][target][0] = "-->".to_owned();
            graph[target][source][0] = "<--".to_owned();
        }
        let progress_events = std::cell::RefCell::new(Vec::new());
        let result = causal_effects_total(
            &values,
            rows,
            4,
            0,
            &graph,
            &[(0, 0)],
            &[(2, 0)],
            &[],
            TotalEffectEstimator::WrightParents,
            [0.0, 1.0],
            CausalEffectsUncertainty::Bootstrap {
                samples: 20,
                block_length: CausalEffectsBlockLength::Fixed { length: 4 },
                confidence_level: 0.9,
                seed: 19,
            },
            |stage, completed, total| progress_events.borrow_mut().push((stage, completed, total)),
        )
        .unwrap();
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(value["fit"]["kind"], "wrightParents");
        assert_eq!(value["fit"]["coefficients"].as_array().unwrap().len(), 5);
        assert_eq!(value["fit"]["paths"].as_array().unwrap().len(), 2);
        let total = value["totalEffect"].as_f64().unwrap();
        let decomposed = value["fit"]["directEffect"].as_f64().unwrap()
            + value["fit"]["indirectEffect"].as_f64().unwrap();
        assert!((total - decomposed).abs() < 1e-12);
        assert_eq!(value["uncertainty"]["kind"], "bootstrap");
        assert_eq!(
            value["uncertainty"]["effectDraws"]
                .as_array()
                .unwrap()
                .len(),
            20
        );
        let events = progress_events.borrow();
        assert_eq!(
            events.first(),
            Some(&("causal-effects-wright-bootstrap", 0, 20))
        );
        assert_eq!(
            events.last(),
            Some(&("causal-effects-wright-bootstrap", 20, 20))
        );
    }

    #[test]
    fn causal_impact_forecasts_the_post_window() {
        let rows = 120;
        let n_pre = 80;
        let control: Vec<f64> = (0..rows)
            .map(|i| (i as f64 / 9.0).sin() + i as f64 * 0.01)
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 10.0 + 2.0 * control[i] + if i >= n_pre { 3.0 } else { 0.0 })
            .collect();
        let mut values = Vec::new();
        values.extend(&y);
        values.extend(&control);
        let json = causal_impact_evidence(&values, rows, 2, 0, &[1], n_pre, rows, 100)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("impact should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "causalImpact");
        assert_eq!(value["nPost"], rows - n_pre);
        assert_eq!(value["postEnd"], rows);
        assert_eq!(value["pointwise"].as_array().unwrap().len(), rows - n_pre);
        let average = value["average"].as_f64().unwrap();
        assert!((average - 3.0).abs() < 0.5, "average effect {average}");
        assert!(causal_impact_evidence(&values, rows, 2, 0, &[1], 4, rows, 100).is_err());

        // The evaluated window narrows what is reported; the forecast behind it does not move.
        let narrowed = serde_json::to_value(
            causal_impact_evidence(&values, rows, 2, 0, &[1], n_pre, n_pre + 1, 100).unwrap(),
        )
        .unwrap();
        assert_eq!(narrowed["nPost"], 1);
        assert_eq!(narrowed["postEnd"], n_pre + 1);
        assert_eq!(narrowed["pointwise"].as_array().unwrap().len(), 1);
        assert_eq!(narrowed["pointwise"][0], value["pointwise"][0]);
        assert_eq!(narrowed["counterfactual"][0], value["counterfactual"][0]);
        assert!(causal_impact_evidence(&values, rows, 2, 0, &[1], n_pre, n_pre, 100).is_err());
        assert!(causal_impact_evidence(&values, rows, 2, 0, &[1], n_pre, rows + 1, 100).is_err());
    }

    /// Two cointegrated random walks: x is a walk and y follows 2x plus stationary noise.
    fn cointegrated_pair(rows: usize) -> (Vec<f64>, Vec<f64>) {
        let mut stream = Mt19937::seeded(11);
        let mut x = Vec::with_capacity(rows);
        let mut level = 0.0;
        for _ in 0..rows {
            level += stream.next_f64() - 0.5;
            x.push(level);
        }
        let y: Vec<f64> = x
            .iter()
            .map(|value| 2.0 * value + 0.3 * (stream.next_f64() - 0.5))
            .collect();
        (x, y)
    }

    #[test]
    fn ardl_reports_the_long_run_effect_and_the_bounds_test() {
        let rows = 200;
        let (x, y) = cointegrated_pair(rows);
        let values: Vec<f64> = x.iter().chain(y.iter()).copied().collect();
        let result =
            serde_json::to_value(ardl_pss(&values, rows, 2, 0, 1, 4, ArdlTrend::Ct, 4).unwrap())
                .unwrap();
        assert_eq!(result["kind"], "ardlPss");
        let effect = result["longRunEffect"].as_f64().unwrap();
        assert!((effect - 2.0).abs() < 0.3, "long-run effect {effect}");
        assert!(
            result["boundsPUpper"].as_f64().unwrap() < 0.05,
            "bounds p upper {}",
            result["boundsPUpper"]
        );
        assert_eq!(result["boundsCritical"].as_array().unwrap().len(), 4);
        assert!(result["arLag"].as_u64().unwrap() >= 1);
        assert!(ardl_pss(&values, rows, 2, 0, 1, 4, ArdlTrend::C, 4).is_err());
        assert!(ardl_pss(&values, rows, 2, 0, 0, 4, ArdlTrend::Ct, 4).is_err());
    }

    #[test]
    fn vecm_selects_a_rank_and_reports_the_normalised_long_run_relation() {
        let rows = 200;
        let (x, y) = cointegrated_pair(rows);
        let values: Vec<f64> = x.iter().chain(y.iter()).copied().collect();
        let result = serde_json::to_value(
            vecm_with_forecast(
                &values,
                rows,
                2,
                &[1, 0],
                4,
                VecmDeterministic::Co,
                1,
                Some(100),
                None,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "vecm");
        assert_eq!(
            result["rank"].as_u64().unwrap(),
            1,
            "rank {}",
            result["rank"]
        );
        let effect = result["longRunEffect"].as_f64().unwrap();
        assert!((effect - 2.0).abs() < 0.3, "long-run effect {effect}");
        assert_eq!(result["beta"].as_array().unwrap().len(), 2);
        assert!(result["chow"].is_array());
        assert!(vecm_with_forecast(&values, rows, 2, &[0], 4, VecmDeterministic::Co, 1, None, None).is_err());
        assert!(vecm_with_forecast(&values, rows, 2, &[0, 1], 4, VecmDeterministic::Co, 3, None, None).is_err());
    }

    #[test]
    fn synthetic_control_weights_the_donors_and_reports_the_post_gap() {
        let rows = 40;
        let n_pre = 25;
        let donor_a: Vec<f64> = (0..rows).map(|i| 10.0 + i as f64 * 0.5).collect();
        let donor_b: Vec<f64> = (0..rows).map(|i| 30.0 - i as f64 * 0.2).collect();
        // The treated unit is 0.6 a + 0.4 b before the intervention and jumps by 5 afterwards.
        let treated: Vec<f64> = (0..rows)
            .map(|i| 0.6 * donor_a[i] + 0.4 * donor_b[i] + if i >= n_pre { 5.0 } else { 0.0 })
            .collect();
        let values: Vec<f64> = treated
            .iter()
            .chain(donor_a.iter())
            .chain(donor_b.iter())
            .copied()
            .collect();
        let result = serde_json::to_value(
            synthetic_control(&values, rows, 3, 0, &[1, 2], n_pre, 3, 0.05).unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "syntheticControl");
        let weights: Vec<f64> = result["weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        assert!(
            (weights[0] - 0.6).abs() < 1e-6 && (weights[1] - 0.4).abs() < 1e-6,
            "weights {weights:?}"
        );
        assert!((result["att"].as_f64().unwrap() - 5.0).abs() < 1e-6);
        assert_eq!(result["synthetic"].as_array().unwrap().len(), rows);
        assert!(synthetic_control(&values, rows, 3, 0, &[0], n_pre, 3, 0.05).is_err());
        assert!(synthetic_control(&values, rows, 3, 0, &[1, 2], rows, 3, 0.05).is_err());
    }

    #[test]
    fn panel_facade_preserves_the_official_california_synthetic_did_result() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../causal-core/oracle/fixtures/panel_synthdid.json"
        ))
        .unwrap();
        let strings = |value: &serde_json::Value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let numbers = |value: &serde_json::Value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        let units = strings(&fixture["input"]["unit"]);
        let times = numbers(&fixture["input"]["time"])
            .into_iter()
            .map(|time| time as i64)
            .collect::<Vec<_>>();
        let outcome = numbers(&fixture["input"]["outcome"]);
        let treatment = numbers(&fixture["input"]["treatment"]);
        let rows = outcome.len();
        let values = outcome.into_iter().chain(treatment).collect::<Vec<_>>();
        let stages = std::cell::RefCell::new(Vec::new());
        let result = panel_intervention(
            &values,
            rows,
            &units,
            &times,
            12,
            0,
            |stage, completed, total| {
                stages.borrow_mut().push((stage, completed, total));
            },
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["kind"], "panelIntervention");
        assert_eq!(encoded["controlUnits"], 38);
        assert_eq!(encoded["treatedUnits"], 1);
        assert_eq!(encoded["nPre"], 19);
        assert_eq!(encoded["nPost"], 12);
        let actual = encoded["syntheticDid"]["estimate"].as_f64().unwrap();
        let expected = fixture["sdid"]["estimate"].as_f64().unwrap();
        assert!(
            (actual - expected).abs() <= 2e-10,
            "{actual} versus {expected}"
        );
        assert_eq!(encoded["syntheticControlPlacebo"]["kind"], "available");
        assert_eq!(encoded["syntheticDidPlacebo"]["kind"], "available");
        assert_eq!(encoded["syntheticControlInTime"]["kind"], "available");
        assert_eq!(encoded["syntheticDidInTime"]["kind"], "available");
        assert_eq!(
            stages.borrow().last(),
            Some(&("synthetic-did-in-time", 9, 9))
        );
    }

    #[test]
    fn negative_binomial_nuts_recovers_a_rate_ratio_above_one() {
        let rows = 120;
        let mut stream = Mt19937::seeded(5);
        let treatment: Vec<f64> = (0..rows).map(|_| stream.next_f64() * 2.0).collect();
        let confounder: Vec<f64> = (0..rows).map(|_| stream.next_f64() - 0.5).collect();
        // Counts whose log mean rises by 0.7 per unit of treatment.
        let outcome: Vec<f64> = (0..rows)
            .map(|i| {
                ((1.0 + 0.7 * treatment[i] + 0.3 * confounder[i]).exp()
                    * (0.75 + 0.5 * stream.next_f64()))
                .round()
            })
            .collect();
        let values: Vec<f64> = treatment
            .iter()
            .chain(outcome.iter())
            .chain(confounder.iter())
            .copied()
            .collect();
        let result =
            serde_json::to_value(negbin_nuts(&values, rows, 3, 0, 1, 2, 150, 300, 9).unwrap())
                .unwrap();
        assert_eq!(result["kind"], "negbinNuts");
        let median = result["irrMedian"].as_f64().unwrap();
        assert!((median.ln() - 0.7).abs() < 0.3, "IRR median {median}");
        assert!(
            result["irrLower"].as_f64().unwrap() < median
                && median < result["irrUpper"].as_f64().unwrap()
        );
        assert!(result["acceptanceRate"].as_f64().unwrap() > 0.5);
        let mut fractional = values.clone();
        fractional[rows] = 0.5;
        assert!(negbin_nuts(&fractional, rows, 3, 0, 1, 2, 150, 300, 9).is_err());
    }

    #[test]
    fn discrete_bn_query_adjusts_for_the_confounder_and_reports_the_do_difference() {
        let rows = 400;
        let mut stream = Mt19937::seeded(21);
        let confounder: Vec<f64> = (0..rows).map(|_| stream.next_f64()).collect();
        let treatment: Vec<f64> = (0..rows)
            .map(|i| confounder[i] + 0.3 * stream.next_f64())
            .collect();
        let outcome: Vec<f64> = (0..rows)
            .map(|i| 2.0 * treatment[i] + 3.0 * confounder[i] + 0.2 * stream.next_f64())
            .collect();
        let values: Vec<f64> = treatment
            .iter()
            .chain(outcome.iter())
            .chain(confounder.iter())
            .copied()
            .collect();
        let names = vec![
            "treatment".to_owned(),
            "outcome".to_owned(),
            "confounder".to_owned(),
        ];
        let edges = vec![(0, 1), (2, 0), (2, 1)];
        let result = serde_json::to_value(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 1, 3, 5.0).unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "discreteBnQuery");
        assert_eq!(result["statePreparations"].as_array().unwrap().len(), 3);
        assert!(result["statePreparations"]
            .as_array()
            .unwrap()
            .iter()
            .all(|entry| entry["strategy"]["kind"] == "quantiles"));
        assert_eq!(result["parentsAdjusted"].as_array().unwrap().len(), 1);
        assert_eq!(result["minimalAdjustmentSet"].as_array().unwrap().len(), 1);
        let effect = result["effect"].as_f64().unwrap();
        let naive = result["expectations"][1].as_f64().unwrap()
            - result["expectations"][0].as_f64().unwrap();
        assert!(
            effect > 0.0 && (effect - naive).abs() < 1e-12,
            "effect {effect}"
        );
        assert_eq!(result["stateCounts"].as_array().unwrap().len(), 3);
        assert!(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 0, 3, 5.0).is_err()
        );
        assert!(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 1, 1, 5.0).is_err()
        );
    }

    fn complete_binary_columns(variable_count: usize) -> (usize, Vec<f64>) {
        let repeats = 8;
        let rows = (1usize << variable_count) * repeats;
        let mut values = Vec::with_capacity(rows * variable_count);
        for variable in 0..variable_count {
            for pattern in 0..(1usize << variable_count) {
                let value = ((pattern >> variable) & 1) as f64;
                values.extend(std::iter::repeat_n(value, repeats));
            }
        }
        (rows, values)
    }

    #[test]
    fn query_discretisation_preserves_imbalanced_binary_and_ordinal_states() {
        let binary = [0.0, 1.0, 1.0, 1.0, f64::NAN];
        let binary_result =
            discretize_for_discrete_bn(&binary, StateBudget::try_from(3).unwrap()).unwrap();
        assert_eq!(binary_result.labels, ["0", "1", "1", "1", "-1"]);
        assert_eq!(binary_result.means.len(), 2);
        assert_eq!(binary_result.means["0"], 0.0);
        assert_eq!(binary_result.means["1"], 1.0);

        let ordinal = [1.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let ordinal_result =
            discretize_for_discrete_bn(&ordinal, StateBudget::try_from(3).unwrap()).unwrap();
        assert_eq!(ordinal_result.labels, ["0", "0", "1", "2", "2", "2"]);
        assert_eq!(ordinal_result.means.len(), 3);
        assert_eq!(ordinal_result.means["2"], 3.0);
    }

    #[test]
    fn discrete_query_does_not_report_zero_for_an_imbalanced_binary_effect() {
        let treatment = [0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let outcome = treatment;
        let values = treatment.into_iter().chain(outcome).collect::<Vec<_>>();
        let names = vec!["treatment".to_owned(), "outcome".to_owned()];
        let result = serde_json::to_value(
            discrete_bn_query(
                &values,
                treatment.len(),
                2,
                &[0, 1],
                &names,
                &[(0, 1)],
                0,
                1,
                3,
                5.0,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(result["stateCounts"], serde_json::json!([2, 2]));
        assert_eq!(result["statePreparations"][0]["name"], "treatment");
        assert_eq!(
            result["statePreparations"][0]["strategy"]["kind"],
            "observedStates"
        );
        assert!(result["effect"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn discrete_query_returns_a_typed_single_state_refusal() {
        let values = vec![0.0, 1.0, 0.0, 1.0, 4.0, 4.0, 4.0, 4.0];
        let names = vec!["treatment".to_owned(), "recovery".to_owned()];
        let result = serde_json::to_value(
            discrete_bn_query(&values, 4, 2, &[0, 1], &names, &[(0, 1)], 0, 1, 3, 5.0).unwrap(),
        )
        .unwrap();

        assert_eq!(result["kind"], "discreteStateRefused");
        assert_eq!(result["query"], "bayesianNetwork");
        assert_eq!(result["node"], 1);
        assert_eq!(result["name"], "recovery");
        assert_eq!(result["problem"]["kind"], "singleObservedState");
        assert_eq!(result["problem"]["value"], 4.0);
        assert_eq!(result["problem"]["observations"], 4);
    }

    #[test]
    fn identified_discrete_query_evaluates_the_frontdoor_id_expression() {
        let (rows, values) = complete_binary_columns(3);
        let names = vec!["U", "X", "M", "Y"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let result = serde_json::to_value(
            identified_discrete_query(
                &values,
                rows,
                3,
                &[0, 1, 2],
                &names,
                &[(0, 1), (0, 3), (1, 2), (2, 3)],
                1,
                3,
                &[0],
                2,
                None,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "identifiedDiscreteQuery");
        assert_eq!(result["statePreparations"].as_array().unwrap().len(), 3);
        assert!(result["statePreparations"]
            .as_array()
            .unwrap()
            .iter()
            .all(|entry| entry["strategy"]["kind"] == "observedStates"));
        assert_eq!(result["query"]["kind"], "unconditional");
        assert_eq!(result["result"]["kind"], "identified");
        assert_eq!(result["result"]["algorithm"], "ID");
        assert!((result["result"]["normalizationLow"].as_f64().unwrap() - 1.0).abs() < 1e-12);
        assert!((result["result"]["normalizationHigh"].as_f64().unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn identified_discrete_query_evaluates_an_idc_condition() {
        let (rows, values) = complete_binary_columns(3);
        let names = vec!["Z", "X", "Y"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let result = serde_json::to_value(
            identified_discrete_query(
                &values,
                rows,
                3,
                &[0, 1, 2],
                &names,
                &[(0, 1), (0, 2), (1, 2)],
                1,
                2,
                &[],
                2,
                Some(IdentifiedDiscreteCondition {
                    variable: 0,
                    state: DiscreteConditionState::Index { state: 1 },
                }),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["query"]["kind"], "conditional");
        assert_eq!(result["query"]["variable"], 0);
        assert_eq!(result["query"]["state"], "1");
        assert_eq!(result["result"]["kind"], "identified");
        assert_eq!(result["result"]["algorithm"], "IDC");
        assert!((result["result"]["normalizationLow"].as_f64().unwrap() - 1.0).abs() < 1e-12);
    }

    /// Setting a sprinkler and observing rain turns the condition into an action; cloudiness then
    /// cannot affect the grass. The evaluator removes the redundant condition by d-separation.
    #[test]
    fn identified_discrete_query_simplifies_a_redundant_condition() {
        let (rows, values) = complete_binary_columns(4);
        let names = vec!["Cloudy", "Sprinkler", "Rain", "Wet_Grass"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let result = serde_json::to_value(
            identified_discrete_query(
                &values,
                rows,
                4,
                &[0, 1, 2, 3],
                &names,
                &[(0, 1), (0, 2), (1, 3), (2, 3)],
                1,
                3,
                &[],
                2,
                Some(IdentifiedDiscreteCondition { variable: 2, state: DiscreteConditionState::Highest }),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["result"]["kind"], "identified");
        assert_eq!(result["result"]["algorithm"], "IDC");
        let expression = result["result"]["expression"].as_str().unwrap();
        assert!(!expression.contains("Cloudy"), "{expression}");
        assert!((result["result"]["normalizationLow"].as_f64().unwrap() - 1.0).abs() < 1e-12);
        assert!((result["result"]["normalizationHigh"].as_f64().unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn identified_discrete_query_matches_pinned_bnlearn_nonuniform_tables() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../causal-core/oracle/fixtures/identified_sprinkler_bnlearn.json")).unwrap();
        let names = fixture["names"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        for case in fixture["cases"].as_array().unwrap() {
            let patterns = case["patterns"].as_array().unwrap();
            let counts = case["counts"].as_array().unwrap();
            let rows: usize = counts.iter().map(|v| v.as_u64().unwrap() as usize).sum();
            let mut values = Vec::new();
            for col in 0..4 {
                for (pattern, count) in patterns.iter().zip(counts) {
                    values.extend(std::iter::repeat_n(pattern[col].as_f64().unwrap(), count.as_u64().unwrap() as usize));
                }
            }
            for budget in [2, 3, 5] {
                for (rain, state) in [(0, DiscreteConditionState::Lowest), (1, DiscreteConditionState::Highest), (1, DiscreteConditionState::Index { state: 1 })] {
                    let result = serde_json::to_value(identified_discrete_query(&values, rows, 4, &[0,1,2,3], &names, &[(0,1),(0,2),(1,3),(2,3)], 1, 3, &[], budget, Some(IdentifiedDiscreteCondition { variable: 2, state })).unwrap()).unwrap();
                    for (s, key) in ["distributionLow", "distributionHigh"].into_iter().enumerate() {
                        let actual = result["result"][key][1][1].as_f64().unwrap();
                        let expected = case["wet_probabilities"][rain * 2 + s].as_f64().unwrap();
                        assert!((actual - expected).abs() < 1e-12, "{} {budget} {rain} {s}: {actual} != {expected}", case["name"]);
                    }
                    assert_eq!(result["query"]["state"], rain.to_string());
                }
            }
            assert!(identified_discrete_query(&values, rows, 4, &[0,1,2,3], &names, &[(0,1),(0,2),(1,3),(2,3)], 1, 3, &[], 5, Some(IdentifiedDiscreteCondition { variable: 2, state: DiscreteConditionState::Index { state: 2 } })).err().unwrap().contains("absent"));
        }
    }

    #[test]
    fn identified_discrete_query_returns_the_source_hedge_witness() {
        let (rows, values) = complete_binary_columns(2);
        let names = vec!["U", "X", "Y"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let result = serde_json::to_value(
            identified_discrete_query(
                &values,
                rows,
                2,
                &[0, 1],
                &names,
                &[(0, 1), (0, 2), (1, 2)],
                1,
                2,
                &[0],
                2,
                None,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["result"]["kind"], "unidentifiable");
        assert!(!result["result"]["hedgeGraph"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(!result["result"]["hedgeSubgraph"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(result["result"].get("effect").is_none());
    }

    #[test]
    fn binary_ett_facade_preserves_the_exact_g_formula_result() {
        let cells = [
            (0.0, 0.0, 0.0, 30),
            (0.0, 0.0, 1.0, 10),
            (0.0, 1.0, 0.0, 5),
            (0.0, 1.0, 1.0, 5),
            (1.0, 0.0, 0.0, 5),
            (1.0, 0.0, 1.0, 5),
            (1.0, 1.0, 0.0, 10),
            (1.0, 1.0, 1.0, 30),
        ];
        let mut z = Vec::new();
        let mut x = Vec::new();
        let mut y = Vec::new();
        for (z_value, x_value, y_value, count) in cells {
            for _ in 0..count {
                z.push(z_value);
                x.push(x_value);
                y.push(y_value);
            }
        }
        let rows = z.len();
        let values = z.into_iter().chain(x).chain(y).collect::<Vec<_>>();
        let names = vec!["Z".to_owned(), "X".to_owned(), "Y".to_owned()];
        let result = serde_json::to_value(
            binary_ett(
                &values,
                rows,
                3,
                &[0, 1, 2],
                &names,
                &[(0, 1), (0, 2), (1, 2)],
                1,
                2,
                &[],
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "binaryEtt");
        assert!((result["treatedPotentialOutcomeMean"].as_f64().unwrap() - 0.70).abs() < 1e-12);
        assert!((result["untreatedPotentialOutcomeMean"].as_f64().unwrap() - 0.45).abs() < 1e-12);
        assert!((result["effectOnTreated"].as_f64().unwrap() - 0.25).abs() < 1e-12);

        let mut invalid = values;
        invalid[0] = 2.0;
        assert!(binary_ett(
            &invalid,
            rows,
            3,
            &[0, 1, 2],
            &names,
            &[(0, 1), (0, 2), (1, 2)],
            1,
            2,
            &[],
        )
        .is_err());
    }
}
