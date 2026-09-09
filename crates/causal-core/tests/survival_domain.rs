use hirmos_causal_core::survival::flexsurv::distribution::{
    DistributionError, FlexSurvDistribution,
};
use hirmos_causal_core::survival::flexsurv::likelihood::{LikelihoodError, LikelihoodRow};
use hirmos_causal_core::survival::flexsurv::observation::{ObservationError, SurvivalObservation};
use hirmos_causal_core::survival::flexsurv::r_optim::{ROptimControl, ROptimError};

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_distribution_parameters_never_form_a_distribution() {
    assert_eq!(
        FlexSurvDistribution::exponential(-1.0),
        Err(DistributionError::NonPositiveParameter("rate"))
    );
    assert_eq!(
        FlexSurvDistribution::weibull(1.0, 0.0),
        Err(DistributionError::NonPositiveParameter("scale"))
    );
    assert_eq!(
        FlexSurvDistribution::generalized_f(0.0, 1.0, 0.0, -0.1),
        Err(DistributionError::NegativeParameter("p"))
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn contradictory_censoring_bounds_never_form_an_observation() {
    assert_eq!(
        SurvivalObservation::interval_censored(4.0, 3.0),
        Err(ObservationError::IntervalNotOrdered)
    );
    assert_eq!(
        SurvivalObservation::delayed_event(5.0, 5.0),
        Err(ObservationError::DelayedEntryNotOrdered)
    );
    assert_eq!(
        SurvivalObservation::delayed_right_censored(5.0, 4.0),
        Err(ObservationError::DelayedEntryNotOrdered)
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_sampling_and_optimization_settings_stop_at_the_boundary() {
    let observation = SurvivalObservation::exact(2.0).unwrap();
    let distribution = FlexSurvDistribution::exponential(0.5).unwrap();
    assert_eq!(
        LikelihoodRow::new(observation, distribution, -1.0),
        Err(LikelihoodError::InvalidWeight)
    );

    let row = LikelihoodRow::new(observation, distribution, 1.0).unwrap();
    assert_eq!(
        row.with_right_truncation(1.0),
        Err(LikelihoodError::RightTruncationBeforeObservation)
    );

    assert_eq!(
        ROptimControl::new(
            f64::NEG_INFINITY,
            f64::EPSILON.sqrt(),
            100,
            vec![1e-3, 1e-3],
            vec![1.0],
            1.0,
        ),
        Err(ROptimError::ControlLength {
            field: "parameter_scales",
            expected: 2,
            actual: 1,
        })
    );
}
