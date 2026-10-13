import type { ColumnId } from '@/domain/dataset'
import { z } from 'zod'
import { causalForestConfigurationSchema } from '@/domain/causalForest'
import { forestAnalysisColumns } from '@/domain/causalForestAnalysis'
import { readPanelConfiguration } from './panelConfiguration'
import { readSyntheticConfiguration } from './syntheticConfiguration'
import { identifiedEstimatorSettingsSchema } from '@/domain/identifiedEstimatorSettings'
import { timeSeriesEstimatorSettingsSchema } from '@/domain/timeSeriesEstimatorSettings'
import { err, ok, type Result } from '@/domain/dop'
import {
  backdoorLinearConfigurationSchema,
  doubleMlConfigurationSchema,
  propensityConfigurationSchema,
  tLearnerConfigurationSchema,
  type EstimatorConfiguration,
} from '@/domain/estimation'

export type EditableEstimatorConfiguration = Extract<
  EstimatorConfiguration,
  {
    kind:
      | 'backdoor-linear-regression'
      | 'propensity-weighting'
      | 'propensity-matching'
      | 'doubly-robust'
      | 'continuous-gps'
      | 'panel-intervention'
      | 'causal-forest'
      | 't-learner'
      | 'dml-plr'
      | 'dml-irm'
      | 'synthetic-control'
      | 'frontdoor-two-stage'
      | 'instrumental-variable'
      | 'negbin-nuts'
      | 'bayesian-gaussian'
      | 'causal-effects-total'
      | 'ardl-pss'
      | 'vecm'
      | 'negative-binomial-ingarch'
  }
>

export function canConfigureEstimator(
  configuration: EstimatorConfiguration,
): configuration is EditableEstimatorConfiguration {
  return (
    configuration.kind === 'backdoor-linear-regression' ||
    configuration.kind === 'propensity-weighting' ||
    configuration.kind === 'propensity-matching' ||
    configuration.kind === 'doubly-robust' ||
    configuration.kind === 'continuous-gps' ||
    configuration.kind === 'panel-intervention' ||
    configuration.kind === 'causal-forest' ||
    configuration.kind === 'dml-plr' ||
    configuration.kind === 'dml-irm' ||
    configuration.kind === 'synthetic-control' ||
    configuration.kind === 'frontdoor-two-stage' ||
    configuration.kind === 'instrumental-variable' ||
    configuration.kind === 'negbin-nuts' ||
    configuration.kind === 'bayesian-gaussian' ||
    configuration.kind === 'causal-effects-total' ||
    configuration.kind === 'ardl-pss' ||
    configuration.kind === 'vecm' ||
    configuration.kind === 'negative-binomial-ingarch' ||
    configuration.kind === 't-learner'
  )
}

export function readEstimatorConfiguration(
  raw: unknown,
  columns: readonly { readonly id: ColumnId; readonly name: string }[],
): Result<EditableEstimatorConfiguration, 'invalid'> {
  const series = timeSeriesEstimatorSettingsSchema.safeParse(raw)
  if (series.success) return ok(series.data)
  const identified = identifiedEstimatorSettingsSchema.safeParse(raw)
  if (identified.success) return ok(identified.data)
  const synthetic = readSyntheticConfiguration(raw, columns)
  if (synthetic.ok) return synthetic
  const forest = causalForestConfigurationSchema.safeParse(raw)
  if (forest.success) {
    const analysis = forest.data.analysis
    if (
      analysis &&
      (forestAnalysisColumns(analysis).some((id) => !columns.some((column) => column.id === id)) ||
        analysis.labels?.some(
          (label) =>
            !columns.some((column) => column.id === label.column && column.name === label.name),
        ))
    )
      return err('invalid')
    return ok(forest.data)
  }
  const learner = tLearnerConfigurationSchema.safeParse(raw)
  if (learner.success) return ok(learner.data)
  const dml = doubleMlConfigurationSchema.safeParse(raw)
  if (dml.success) return ok(dml.data)
  const panel = readPanelConfiguration(raw, columns)
  if (panel.ok) return panel
  const propensity = propensityConfigurationSchema.safeParse(raw)
  if (propensity.success) return ok(propensity.data)
  const parsed = backdoorLinearConfigurationSchema.safeParse(raw)
  if (!parsed.success) return err('invalid')
  const { errors, fixedEffects, kind, level } = parsed.data
  const column = (id: string, name: string) =>
    columns.find((item) => item.id === id && item.name === name)
  const cluster = errors.kind === 'cluster' ? column(errors.column, errors.name) : null
  if (errors.kind === 'cluster' && !cluster) return err('invalid')
  const covariance =
    errors.kind === 'cluster'
      ? { kind: 'cluster' as const, column: cluster!.id, name: cluster!.name }
      : errors
  if (fixedEffects.kind === 'none') return ok({ kind, level, errors: covariance, fixedEffects })
  const first = column(fixedEffects.column, fixedEffects.name)
  if (!first) return err('invalid')
  if (fixedEffects.kind !== 'unit-and-time')
    return ok({
      kind,
      level,
      errors: covariance,
      fixedEffects: { ...fixedEffects, column: first.id },
    })
  const time = column(fixedEffects.timeColumn, fixedEffects.timeName)
  if (!time) return err('invalid')
  return ok({
    kind,
    level,
    errors: covariance,
    fixedEffects: { ...fixedEffects, column: first.id, timeColumn: time.id },
  })
}

export function learnerConfigurationSchema() {
  return z.toJSONSchema(z.union([causalForestConfigurationSchema, tLearnerConfigurationSchema, doubleMlConfigurationSchema]), {
    io: 'input',
  })
}
