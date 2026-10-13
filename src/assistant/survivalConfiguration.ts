import { z } from 'zod'
import type { ColumnSelection } from '@/domain/dataset'
import type { Draft } from '@/domain/survivalDraft'
import { parametricSurvivalFamilySchema, proportionalHazardsFamilySchema } from '@/domain/survival'
import { forestSettingsSchema } from '@/domain/survivalRegression'

export function survivalConfigurationSchema(columns: readonly ColumnSelection[]): z.ZodType<Draft> {
  const column = z
    .string()
    .transform((id, ctx) => {
      const match = columns.find((column) => column.id === id)
      if (match) return match.id
      ctx.addIssue({ code: 'custom', message: 'Choose a column in this dataset.' })
      return z.NEVER
    })
    .nullable()
  const covariates = z.array(column.unwrap())
  const n = z.number()
  const obj = z.strictObject
  const tagged = <K extends string>(kind: K) => obj({ kind: z.literal(kind) })
  const frequency = z.union([
    tagged('one-observation-per-row'),
    obj({ kind: z.literal('frequency-column'), column }),
  ])
  const columnMode = obj({ kind: z.literal('column'), column })
  const transitions = z.array(obj({ from: n, to: n }))
  const right = { duration: column, event: column }
  return z.discriminatedUnion('kind', [
    obj({ kind: z.literal('aalen'), ...right, covariates }),
    obj({
      kind: z.literal('survival-forest'),
      ...right,
      covariates,
      categorical: covariates,
      settings: forestSettingsSchema,
      predictionRow: n,
    }),
    obj({
      kind: z.literal('right-censored'),
      ...right,
      rowFrequency: frequency,
      covariates,
      family: parametricSurvivalFamilySchema,
      horizon: n,
    }),
    obj({
      kind: z.literal('nonparametric'),
      ...right,
      rowFrequency: frequency,
      horizon: n,
      ties: z.enum(['discrete', 'smoothed']),
    }),
    obj({
      kind: z.literal('start-stop'),
      start: column,
      stop: column,
      event: column,
      rowFrequency: frequency,
      covariates,
      family: proportionalHazardsFamilySchema,
      horizon: n,
    }),
    obj({
      kind: z.literal('cox-regression'),
      observation: z.discriminatedUnion('kind', [
        obj({
          kind: z.literal('right-censored'),
          ...right,
          entry: z.union([tagged('not-used'), columnMode]),
          standardErrors: z.union([
            tagged('model-based'),
            tagged('robust'),
            obj({ kind: z.enum(['clustered', 'clustered-breslow']), column }),
          ]),
          frailty: z.union([
            tagged('none'),
            obj({ kind: z.literal('gamma'), column, ties: z.enum(['efron', 'breslow']) }),
          ]),
        }),
        obj({
          kind: z.literal('start-stop'),
          subject: column,
          start: column,
          stop: column,
          event: column,
        }),
      ]),
      weights: z.union([tagged('equal'), columnMode]),
      strata: z.union([tagged('unstratified'), columnMode]),
      covariates,
      penalty: z.union([
        tagged('unpenalized'),
        obj({ kind: z.literal('elastic-net'), strength: n, l1Ratio: n }),
      ]),
      confidenceLevel: n,
    }),
    obj({
      kind: z.literal('penalized-aft'),
      ...right,
      covariates,
      family: z.enum(['weibull', 'logLogistic']),
      penalizer: n,
      confidenceLevel: n,
      horizon: n,
    }),
    obj({
      kind: z.literal('two-group'),
      ...right,
      group: column,
      truncationTime: n,
      permutations: n,
      seed: n,
    }),
    obj({
      kind: z.literal('multi-state'),
      family: proportionalHazardsFamilySchema,
      horizon: n,
      input: z.discriminatedUnion('kind', [
        obj({
          kind: z.literal('prepared-rows'),
          start: column,
          stop: column,
          event: column,
          from: column,
          to: column,
        }),
        obj({
          kind: z.literal('longitudinal-states'),
          subject: column,
          time: column,
          state: column,
          stateCount: n,
          transitions,
        }),
        obj({
          kind: z.literal('wide-events'),
          transitions,
          states: z.array(
            z.union([
              tagged('not-applicable'),
              obj({ kind: z.literal('recorded'), time: column, status: column }),
            ]),
          ),
          entry: z.union([
            obj({ kind: z.literal('shared'), state: n, time: n }),
            obj({ kind: z.literal('columns'), state: column, time: column }),
          ]),
        }),
      ]),
    }),
  ])
}
