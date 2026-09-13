//! R-compatible error-correction diagnostics and constrained lag searches.
use super::{problem, Coefficient, Orders};
use hirmos_causal_core::ardl::{
    bounds_statistic,
    multivariate::{
        fit_r_uecm,
        horizontal::HorizontalSearch,
        search::{Criterion, Search},
        Input, Specification, Term,
    },
    Trend,
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RankedOrder {
    order: Vec<usize>,
    aic_pss: f64,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum Ranking {
    NotRequested,
    Horizontal {
        rows: Vec<RankedOrder>,
    },
    Grid {
        evaluated: usize,
        rows: Vec<RankedOrder>,
    },
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum TStatistic {
    NotApplicable,
    Recorded { value: f64 },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SerialCorrelation {
    order: usize,
    statistic: f64,
    p_value: f64,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum Evidence {
    NotRequested,
    Recorded {
        coefficients: Vec<Coefficient>,
        params: Vec<f64>,
        covariance: Vec<Vec<f64>>,
        residuals: Vec<f64>,
        aic_pss: f64,
        sbc_pss: f64,
        serial_correlation: Vec<SerialCorrelation>,
        bounds_f: f64,
        bounds_t: TStatistic,
        ranking: Ranking,
    },
}

pub(super) fn select(
    orders: Orders,
    input: &Input<'_>,
    trend: Trend,
    hold: Option<usize>,
) -> Result<(Specification, Ranking), String> {
    match orders {
        Orders::RFixed {
            outcome_lag,
            predictor_lags,
        } => Ok((
            Specification::new(
                outcome_lag,
                predictor_lags.into_iter().map(Some).collect(),
                trend,
                hold,
            )
            .map_err(problem)?,
            Ranking::NotRequested,
        )),
        Orders::RHorizontal {
            maximum,
            fixed,
            starting,
        } => {
            let hold =
                hold.ok_or("Specify initial observations to exclude for a common search sample.")?;
            let ranked = HorizontalSearch::new(maximum, fixed, starting, trend, hold, 10000)
                .map_err(problem)?
                .run(input, |_| true)
                .map_err(problem)?;
            let best = ranked
                .first()
                .ok_or("The search found no candidate model.")?;
            let spec = Specification::new(
                best.order[0],
                best.order[1..].iter().copied().map(Some).collect(),
                trend,
                Some(hold),
            )
            .map_err(problem)?;
            Ok((
                spec,
                Ranking::Horizontal {
                    rows: ranked
                        .into_iter()
                        .map(|r| RankedOrder {
                            order: r.order,
                            aic_pss: r.aic_pss,
                        })
                        .collect(),
                },
            ))
        }
        Orders::RGrid {
            minimum_lag,
            maximum_lag,
            maximum_orders,
            fixed_orders,
        } => {
            let hold =
                hold.ok_or("Specify initial observations to exclude for a common search sample.")?;
            let result = Search::restricted(
                minimum_lag,
                maximum_lag,
                maximum_orders,
                fixed_orders,
                trend,
                Some(hold),
                Criterion::Aic,
                10000,
            )
            .map_err(problem)?
            .run(input, |_, _| true)
            .map_err(problem)?;
            let mut candidates = result.candidates;
            let evaluated = candidates.len();
            candidates.sort_by(|a, b| a.aic.total_cmp(&b.aic));
            let rows = candidates
                .into_iter()
                .take(20)
                .map(|c| RankedOrder {
                    order: std::iter::once(c.outcome_lag)
                        .chain(c.predictor_lags.into_iter().flatten())
                        .collect(),
                    aic_pss: 1.0 - c.aic / 2.0,
                })
                .collect();
            Ok((result.specification, Ranking::Grid { evaluated, rows }))
        }
        Orders::Fixed { .. } | Orders::Search { .. } => {
            Err("This request does not use R ARDL search conventions.".into())
        }
    }
}

pub(super) fn fit(
    input: &Input<'_>,
    spec: &Specification,
    case: usize,
    ranking: Ranking,
    coefficient: impl Fn(&Term) -> Coefficient,
) -> Result<Evidence, String> {
    let model = fit_r_uecm(input, spec).map_err(problem)?;
    let (aic_pss, sbc_pss) = model.pss_information_criteria().map_err(problem)?;
    let max_order = 5.min(model.fit.nobs.saturating_sub(model.fit.params.len() + 1));
    let serial_correlation = (1..=max_order)
        .map(|k| {
            model
                .serial_correlation(k)
                .map(|r| SerialCorrelation {
                    order: r.order,
                    statistic: r.statistic,
                    p_value: r.p_value,
                })
                .map_err(problem)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let bounds_f = bounds_statistic(&model, case);
    let bounds_t = match case {
        3 | 5 => TStatistic::Recorded {
            value: model.fit.params[model.n_det]
                / model.fit.cov_params[(model.n_det, model.n_det)].sqrt(),
        },
        _ => TStatistic::NotApplicable,
    };
    Ok(Evidence::Recorded {
        coefficients: model.terms.iter().map(coefficient).collect(),
        params: model.fit.params.iter().copied().collect(),
        covariance: (0..model.fit.params.len())
            .map(|i| model.fit.cov_params.row(i).iter().copied().collect())
            .collect(),
        residuals: model.fit.resid.iter().copied().collect(),
        aic_pss,
        sbc_pss,
        serial_correlation,
        bounds_f,
        bounds_t,
        ranking,
    })
}
