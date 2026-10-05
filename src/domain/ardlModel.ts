import { z } from 'zod'
import { assertNever } from './dop'

const finite = z.number().finite()
const index = z.number().int().nonnegative()
const lag = z.number().int().min(0).max(24)
const interval = z
  .tuple([finite, finite])
  .refine(([lower, upper]) => lower <= upper, 'Interval limits are reversed.')
const level = z.number().gt(0).lt(1)
const values = z.array(finite)
const series = z.tuple([finite]).rest(finite)
export const omittedEcmChangeSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('outcome'), lag: lag.min(1) }).strict(),
  z.object({ kind: z.literal('predictor'), column: index, lag }).strict(),
])

export const ardlOrdersSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('fixed'),
      outcomeLag: lag,
      predictorLags: z.array(lag.nullable()).min(1),
    })
    .strict(),
  z
    .object({ kind: z.literal('search'), maximumLag: lag, maximumOrders: z.array(lag).min(1) })
    .strict(),
  z
    .object({
      kind: z.literal('rFixed'),
      outcomeLag: lag.min(1),
      predictorLags: z.array(lag).min(1),
    })
    .strict(),
  z
    .object({
      kind: z.literal('rRestricted'),
      outcomeLag: lag.min(1),
      predictorLags: z.array(lag).min(1),
      omitted: z.array(omittedEcmChangeSchema).min(1),
    })
    .strict(),
  z
    .object({
      kind: z.literal('rHorizontal'),
      maximum: z.array(lag).min(2),
      fixed: z.array(lag.nullable()).min(2),
      starting: z.array(lag).min(2),
    })
    .strict(),
  z
    .object({
      kind: z.literal('rGrid'),
      minimumLag: lag.min(1),
      maximumLag: lag.min(1),
      maximumOrders: z.array(lag).min(1),
      fixedOrders: z.array(lag.nullable()).min(1),
    })
    .strict(),
])
export const ardlFutureSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z
    .object({
      kind: z.literal('scenario'),
      predictors: z.array(series).min(1),
      fixed: z.array(series),
      confidence: level,
    })
    .strict(),
])
export const ardlModelRequestSchema = z
  .object({
    outcome: index,
    predictors: z.tuple([index]).rest(index),
    fixed: z.array(index),
    terms: z.enum(['restricted-constant', 'constant', 'restricted-trend', 'trend']),
    orders: ardlOrdersSchema,
    holdBack: index.nullable(),
    multiplierHorizon: z.number().int().min(0).max(200),
    future: ardlFutureSchema,
  })
  .strict()
  .superRefine((request, ctx) => {
    const columns = [request.outcome, ...request.predictors, ...request.fixed]
    const selection = request.orders
    const orders =
      selection.kind === 'rHorizontal'
        ? selection.maximum.slice(1)
        : selection.kind === 'fixed' ||
            selection.kind === 'rFixed' ||
            selection.kind === 'rRestricted'
          ? selection.predictorLags
          : selection.maximumOrders
    const maximum = Math.max(
      selection.kind === 'rHorizontal'
        ? selection.maximum[0]!
        : selection.kind === 'fixed' ||
            selection.kind === 'rFixed' ||
            selection.kind === 'rRestricted'
          ? selection.outcomeLag
          : selection.maximumLag,
      ...orders.map((q) => q ?? 0),
    )
    if (
      selection.kind === 'rRestricted' &&
      (new Set(selection.omitted.map((change) => JSON.stringify(change))).size !==
        selection.omitted.length ||
        selection.omitted.some((change) =>
          change.kind === 'outcome'
            ? change.lag >= selection.outcomeLag
            : change.column >= selection.predictorLags.length ||
              change.lag >= selection.predictorLags[change.column]!,
        ))
    )
      ctx.addIssue({
        code: 'custom',
        message: 'Omit only distinct short-run change terms included in the specified ECM.',
      })
    const checks = [
      {
        valid: new Set(columns).size === columns.length,
        message: 'Choose distinct outcome, predictor and fixed columns.',
      },
      {
        valid: orders.length === request.predictors.length,
        message: 'Specify one lag order for each predictor.',
      },
      {
        valid: request.holdBack === null || request.holdBack >= maximum,
        message: 'The sample must start after every included lag.',
      },
    ]
    for (const check of checks)
      if (!check.valid) ctx.addIssue({ code: 'custom', message: check.message })
    if (selection.kind === 'rHorizontal') {
      const size = request.predictors.length + 1
      const valid =
        selection.maximum.length === size &&
        selection.starting.length === size &&
        selection.fixed.length === size &&
        selection.starting[0]! > 0 &&
        selection.fixed[0] !== 0 &&
        selection.starting.every((q, i) => q <= selection.maximum[i]!) &&
        selection.fixed.every((q, i) => q === null || q <= selection.maximum[i]!)
      if (!valid)
        ctx.addIssue({
          code: 'custom',
          message:
            'Each series needs valid maximum, starting and optional fixed orders. The outcome must have at least one lag.',
        })
    }
    if (selection.kind === 'rGrid') {
      const valid =
        selection.minimumLag <= selection.maximumLag &&
        selection.fixedOrders.length === orders.length &&
        selection.fixedOrders.every((q, i) => q === null || q <= selection.maximumOrders[i]!)
      if (!valid)
        ctx.addIssue({
          code: 'custom',
          message:
            'Fixed orders must fit within the grid limits, and the minimum outcome lag cannot exceed the maximum.',
        })
    }
    if (
      (selection.kind === 'rHorizontal' || selection.kind === 'rGrid') &&
      request.holdBack === null
    )
      ctx.addIssue({
        code: 'custom',
        message:
          'Specify initial observations to exclude so all search candidates use the same sample.',
      })
    switch (request.future.kind) {
      case 'none':
        return
      case 'scenario': {
        const future = request.future
        const horizon = future.predictors[0]?.length ?? 0
        const valid =
          horizon > 0 &&
          horizon <= 200 &&
          future.predictors.length === request.predictors.length &&
          future.fixed.length === request.fixed.length &&
          [...future.predictors, ...future.fixed].every((column) => column.length === horizon)
        if (!valid)
          ctx.addIssue({
            code: 'custom',
            message:
              'Supply the same number of future periods for every predictor and fixed column.',
          })
        return
      }
      default:
        return assertNever(request.future)
    }
  })
  .brand<'ArdlModelRequest'>()
export type ArdlModelRequest = z.infer<typeof ardlModelRequestSchema>

export const ardlCoefficientSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('constant') }).strict(),
  z.object({ kind: z.literal('trend') }).strict(),
  z.object({ kind: z.literal('outcome'), lag: lag.min(1) }).strict(),
  z.object({ kind: z.literal('predictor'), column: index, lag }).strict(),
  z.object({ kind: z.literal('fixed'), column: index }).strict(),
])
const ecmCoefficientSchema = z.discriminatedUnion('kind', [
  ...ardlCoefficientSchema.options,
  z.object({ kind: z.literal('outcomeChange'), lag: lag.min(1) }).strict(),
  z.object({ kind: z.literal('predictorChange'), column: index, lag }).strict(),
])
export type EcmCoefficient = z.infer<typeof ecmCoefficientSchema>
const multiplierVariable = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('constant') }).strict(),
  z.object({ kind: z.literal('trend') }).strict(),
  z.object({ kind: z.literal('predictor'), column: index }).strict(),
])
const longRun = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('unavailable'),
      reason: z.enum(['noPredictors', 'zeroOrder', 'tooManyPredictors', 'undefinedNormalization']),
    })
    .strict(),
  z
    .object({
      kind: z.literal('recorded'),
      departures: series,
      normalized: series,
      intervals: z.array(interval),
      boundsStatistic: finite.nonnegative(),
      boundsCritical: z.array(interval).length(4),
      pLower: finite.min(0).max(1),
      pUpper: finite.min(0).max(1),
    })
    .strict(),
  z
    .object({
      kind: z.literal('uncalibrated'),
      departures: series,
      normalized: series,
      intervals: z.array(interval),
      boundsStatistic: finite.nonnegative(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('rCalibrated'),
      departures: series,
      normalized: series,
      intervals: z.array(interval),
      boundsStatistic: finite.nonnegative(),
      fBounds: z.array(z.object({ alpha: level, i0: finite, i1: finite }).strict()).length(8),
      fPValue: finite.min(0).max(1),
      tBounds: z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('notApplicable') }).strict(),
        z
          .object({
            kind: z.literal('recorded'),
            statistic: finite,
            critical: z
              .array(z.object({ alpha: level, i0: finite, i1: finite }).strict())
              .length(8),
            pValue: finite.min(0).max(1),
          })
          .strict(),
      ]),
    })
    .strict(),
])
const rankedOrder = z.object({ order: z.array(lag).min(2), aicPss: finite }).strict()
const ranking = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notRequested') }).strict(),
  z.object({ kind: z.literal('horizontal'), rows: z.array(rankedOrder).min(1).max(20) }).strict(),
  z
    .object({
      kind: z.literal('grid'),
      evaluated: index.positive(),
      rows: z.array(rankedOrder).min(1).max(20),
    })
    .strict(),
])
const rAnalysis = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notRequested') }).strict(),
  z
    .object({
      kind: z.literal('recorded'),
      coefficients: z.array(ecmCoefficientSchema).min(1),
      params: series,
      covariance: z.array(series).min(1),
      residuals: series,
      aicPss: finite,
      sbcPss: finite,
      boundsF: finite.nonnegative(),
      boundsT: z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('notApplicable') }).strict(),
        z.object({ kind: z.literal('recorded'), value: finite }).strict(),
      ]),
      serialCorrelation: z
        .array(
          z
            .object({
              order: index.positive(),
              statistic: finite.nonnegative(),
              pValue: finite.min(0).max(1),
            })
            .strict(),
        )
        .max(5),
      ranking,
    })
    .strict(),
])
const multipliers = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('unavailable'), reason: z.literal('numericalFailure') }).strict(),
  z
    .object({
      kind: z.literal('recorded'),
      confidence: level,
      curves: z
        .array(
          z
            .object({
              term: multiplierVariable,
              shortRun: finite,
              longRun: finite,
              longRunSe: finite.nonnegative(),
              delay: series,
              standardError: z.array(finite.nonnegative()).min(1),
              interval: z.array(interval).min(1),
              cumulative: series,
            })
            .strict()
            .superRefine((curve, ctx) => {
              const size = curve.delay.length
              if (
                [curve.standardError, curve.interval, curve.cumulative].some(
                  (values) => values.length !== size,
                )
              ) {
                ctx.addIssue({
                  code: 'custom',
                  message: 'Multiplier values and intervals must cover the same periods.',
                })
                return
              }
              let total = 0
              curve.delay.forEach((value, i) => {
                total += value
                if (Math.abs(total - curve.cumulative[i]!) > 1e-8 * Math.max(1, Math.abs(total)))
                  ctx.addIssue({
                    code: 'custom',
                    message: 'Cumulative multipliers must equal the sum of the delay multipliers.',
                  })
                if (curve.interval[i]![0] > value || curve.interval[i]![1] < value)
                  ctx.addIssue({
                    code: 'custom',
                    message: 'A multiplier interval must contain its estimate.',
                  })
              })
            }),
        )
        .min(1),
    })
    .strict(),
])
const forecast = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notRequested') }).strict(),
  z
    .object({
      kind: z.literal('recorded'),
      confidence: level,
      mean: series,
      variance: z.array(finite.nonnegative()).min(1),
      interval: z.array(interval).min(1),
    })
    .strict()
    .superRefine((result, ctx) => {
      if (
        result.mean.length !== result.variance.length ||
        result.mean.length !== result.interval.length
      )
        ctx.addIssue({
          code: 'custom',
          message: 'Forecast values and intervals must cover the same periods.',
        })
      result.interval.forEach(([lower, upper], i) => {
        const mean = result.mean[i]
        if (mean !== undefined && (lower > mean || upper < mean))
          ctx.addIssue({
            code: 'custom',
            message: 'A forecast interval must contain its estimate.',
          })
      })
    }),
])
export const ardlModelEvidenceSchema = z
  .object({
    observations: index.positive(),
    startRow: index,
    fittedRows: index.positive(),
    outcomeLag: lag,
    predictorLags: z.array(lag.nullable()).min(1),
    coefficients: z.array(ardlCoefficientSchema).min(1),
    params: series,
    covariance: z.array(series).min(1),
    observed: series,
    fitted: series,
    longRun,
    multipliers,
    forecast,
    rAnalysis: rAnalysis.default({ kind: 'notRequested' }),
  })
  .strict()
  .superRefine((result, ctx) => {
    const count = result.params.length
    const checks = [
      {
        valid: result.observed.length === result.observations,
        message: 'Observed values must cover the prepared sample.',
      },
      {
        valid:
          result.startRow + result.fittedRows === result.observations &&
          result.fitted.length === result.fittedRows,
        message: 'Fitted values must match the recorded estimation sample.',
      },
      {
        valid:
          result.coefficients.length === count &&
          result.covariance.length === count &&
          result.covariance.every((row) => row.length === count),
        message: 'Coefficient and covariance dimensions must agree.',
      },
      {
        valid: result.coefficients.every(
          (term) => term.kind !== 'predictor' || term.column < result.predictorLags.length,
        ),
        message: 'A coefficient refers to an unknown predictor.',
      },
    ]
    for (const check of checks)
      if (!check.valid) ctx.addIssue({ code: 'custom', message: check.message })
    const r = result.rAnalysis
    if (r.kind === 'recorded') {
      const size = r.params.length
      if (
        r.coefficients.length !== size ||
        r.covariance.length !== size ||
        r.covariance.some((row) => row.length !== size) ||
        r.residuals.length !== result.fittedRows
      )
        ctx.addIssue({
          code: 'custom',
          message:
            'Error-correction coefficients, covariance and residuals must match their estimation sample.',
        })
      if (r.covariance.some((row, i) => (row[i] ?? -1) < 0))
        ctx.addIssue({
          code: 'custom',
          message: 'Error-correction coefficient variances cannot be negative.',
        })
      if (
        (result.longRun.kind === 'uncalibrated' || result.longRun.kind === 'rCalibrated') &&
        Math.abs(result.longRun.boundsStatistic - r.boundsF) >
          1e-8 * Math.max(1, Math.abs(r.boundsF))
      )
        ctx.addIssue({ code: 'custom', message: 'The reported bounds statistics must agree.' })
      if (r.serialCorrelation.some((row, i) => row.order !== i + 1))
        ctx.addIssue({
          code: 'custom',
          message: 'Serial-correlation tests must cover consecutive orders.',
        })
      if (r.ranking.kind !== 'notRequested') {
        const rows = r.ranking.rows
        if (
          rows.some(
            (row, i) =>
              row.order.length !== result.predictorLags.length + 1 ||
              (i > 0 && row.aicPss > rows[i - 1]!.aicPss),
          )
        )
          ctx.addIssue({
            code: 'custom',
            message: 'Search rows must be ordered by decreasing PSS AIC and cover every series.',
          })
        if (rows[0]!.order.some((q, i) => q !== [result.outcomeLag, ...result.predictorLags][i]))
          ctx.addIssue({
            code: 'custom',
            message: 'The fitted orders must match the best search row.',
          })
      }
    }
    if (
      (result.longRun.kind === 'uncalibrated' || result.longRun.kind === 'rCalibrated') &&
      r.kind !== 'recorded'
    )
      ctx.addIssue({
        code: 'custom',
        message: 'R bounds evidence requires its error-correction evidence.',
      })
    if (result.longRun.kind === 'rCalibrated') {
      const b = result.longRun
      const levels = [0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.15, 0.2]
      if (b.fBounds.some((row, i) => row.alpha !== levels[i] || row.i0 > row.i1))
        ctx.addIssue({
          code: 'custom',
          message: 'F critical bounds must cover the recorded significance levels in order.',
        })
      if (
        b.tBounds.kind === 'recorded' &&
        (b.tBounds.critical.some((row, i) => row.alpha !== levels[i] || row.i1 > row.i0) ||
          r.kind !== 'recorded' ||
          r.boundsT.kind !== 'recorded' ||
          Math.abs(b.tBounds.statistic - r.boundsT.value) >
            1e-8 * Math.max(1, Math.abs(r.boundsT.value)))
      )
        ctx.addIssue({
          code: 'custom',
          message: 'The t critical bounds and statistic must match the error-correction evidence.',
        })
      if (
        r.kind === 'recorded' &&
        (b.tBounds.kind === 'recorded') !== (r.boundsT.kind === 'recorded')
      )
        ctx.addIssue({
          code: 'custom',
          message:
            'The applicability of the t-bounds test must match the error-correction evidence.',
        })
    }
    switch (result.longRun.kind) {
      case 'unavailable':
        return
      case 'recorded':
      case 'rCalibrated':
      case 'uncalibrated': {
        const active = result.predictorLags.filter((q) => q !== null)
        const terms = result.coefficients.filter(
          (term) => term.kind === 'constant' || term.kind === 'trend',
        ).length
        const size = active.length + terms + 1
        if (
          active.length === 0 ||
          (result.longRun.kind === 'recorded' && active.includes(0)) ||
          result.longRun.normalized.length !== size ||
          result.longRun.intervals.length !== size ||
          result.longRun.departures.length !== result.observations
        ) {
          ctx.addIssue({
            code: 'custom',
            message:
              'Long-run evidence must match the included lagged predictors and prepared sample.',
          })
        }
        return
      }
      default:
        return assertNever(result.longRun)
    }
  })
  .brand<'ArdlModelEvidence'>()
export type ArdlModelEvidence = z.infer<typeof ardlModelEvidenceSchema>

/** Compare each statistic with its own calibrated rejection bounds. */
export function ardlBoundsDecision(
  statistic: number,
  bounds: { readonly i0: number; readonly i1: number },
  family: 'F' | 't',
): 'Reject the null' | 'Do not reject' | 'Inconclusive' {
  if (family === 'F')
    return statistic > bounds.i1
      ? 'Reject the null'
      : statistic < bounds.i0
        ? 'Do not reject'
        : 'Inconclusive'
  return statistic < bounds.i1
    ? 'Reject the null'
    : statistic > bounds.i0
      ? 'Do not reject'
      : 'Inconclusive'
}

/** Parse the transport envelope before exposing its validated numerical evidence. */
export const ardlModelResponseSchema = z
  .object({
    kind: z.literal('ardlModel'),
    evidence: ardlModelEvidenceSchema,
  })
  .strict()
