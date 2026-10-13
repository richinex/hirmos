import type { TimeSeriesRun } from '@/domain/timeSeries'
import type { SurvivalRunArtifact } from '@/domain/survival'

export const TIME_SERIES_SUMMARY =
  'Run ID, model, observation count, and supported aggregate model statistics. No observed series, fitted values or residuals.'
export const SURVIVAL_SUMMARY =
  'Run ID and model; aggregate coefficients and fit statistics for Cox and parametric survival. Other models return a completion receipt only. No subject-level values.'

export function timeSeriesSummary(run: TimeSeriesRun): string {
  const identity = { kind: run.kind, observations: run.evidence.observations }
  switch (run.kind) {
    case 'ardl-model':
      return JSON.stringify({
        ...identity,
        outcomeLag: run.evidence.outcomeLag,
        predictorLags: run.evidence.predictorLags,
        terms: run.evidence.coefficients,
        coefficients: run.evidence.params,
        longRun: run.evidence.longRun,
      })
    case 'vecm':
      return JSON.stringify({
        ...identity,
        rank: run.evidence.rank,
        differencedLags: run.evidence.kArDiff,
        alpha: run.evidence.alpha,
        beta: run.evidence.beta,
        note: 'Cointegration and predictive relationships do not independently establish causality.',
      })
    default:
      return JSON.stringify({
        ...identity,
        note: 'Numerical result details remain in the result panel; this summary does not include them.',
      })
  }
}

export function survivalSummary(run: SurvivalRunArtifact): string {
  switch (run.kind) {
    case 'cox-regression-run':
      return JSON.stringify({
        kind: run.kind,
        observations: run.evidence.observations,
        events: run.evidence.events,
        coefficients: run.evidence.coefficients,
        logLikelihood: run.evidence.logLikelihood,
        partialAic: run.evidence.partialAic,
        standardErrors: run.evidence.standardErrors,
        fitting: run.evidence.fitting.kind,
        note: 'Hazard ratios are conditional model associations unless the causal design supports an intervention interpretation.',
      })
    case 'right-censored-survival-run':
    case 'start-stop-survival-run':
      return JSON.stringify({
        kind: run.kind,
        observations: run.evidence.observations,
        events: run.evidence.events,
        family: run.evidence.family,
        coefficients: run.evidence.coefficients,
        parameterIntervals: run.evidence.parameterIntervals,
        logLikelihood: run.evidence.logLikelihood,
        aic: run.evidence.aic,
        bic: run.evidence.bic,
      })
    default:
      return JSON.stringify({
        kind: run.kind,
        note: 'The run was saved. Numerical details remain in the Survival result panel and are not included in this receipt.',
      })
  }
}
