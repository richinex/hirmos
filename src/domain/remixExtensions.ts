import { z } from 'zod'

const finite = z.number().finite()
const period = z.number().int().safe()
const confidence = finite.gt(0).lt(1)
const unit = z
  .string()
  .min(1)
  .refine((v) => v.trim().length > 0, 'A unit label must not be blank.')
const panel = {
  rows: z.number().int().positive(),
  columns: z.number().int().positive(),
  outcome: z.number().int().nonnegative(),
  units: z.tuple([unit]).rest(unit).readonly(),
  times: z.tuple([period]).rest(period).readonly(),
}
type Panel = {
  readonly rows: number
  readonly columns: number
  readonly outcome: number
  readonly units: readonly string[]
  readonly times: readonly number[]
}
function panelIssues(r: Panel, ctx: z.RefinementCtx): void {
  if (r.outcome >= r.columns || r.units.length !== r.rows || r.times.length !== r.rows)
    ctx.addIssue({
      code: 'custom',
      message: 'The outcome and panel keys must describe the same rows.',
    })
  const keys = new Set(r.units.map((u, i) => JSON.stringify([u, r.times[i]])))
  if (keys.size !== r.rows)
    ctx.addIssue({ code: 'custom', message: 'Each unit and period must identify one observation.' })
}
export const sunAbrahamRequestSchema = z
  .object({
    ...panel,
    adoption: z.tuple([period.nullable()]).rest(period.nullable()).readonly(),
    referencePeriods: z.tuple([period]).rest(period).readonly(),
    referenceCohorts: z.array(period).readonly(),
    confidence,
  })
  .strict()
  .superRefine((r, ctx) => {
    panelIssues(r, ctx)
    if (r.adoption.length !== r.rows)
      ctx.addIssue({ code: 'custom', message: 'Record adoption for every panel row.' })
    const periods = new Set(r.times)
    const adoption = new Map<string, number | null>()
    r.units.forEach((u, i) => {
      const g = r.adoption[i]
      if (g === undefined) return
      if (adoption.has(u) && adoption.get(u) !== g)
        ctx.addIssue({
          code: 'custom',
          message: 'A unit must have the same adoption period on every row.',
        })
      adoption.set(u, g)
      if (g !== null && !periods.has(g))
        ctx.addIssue({
          code: 'custom',
          message: 'Each adoption period must occur on the panel time axis.',
        })
    })
    if (
      new Set(r.referencePeriods).size !== r.referencePeriods.length ||
      new Set(r.referenceCohorts).size !== r.referenceCohorts.length
    )
      ctx.addIssue({
        code: 'custom',
        message: 'Reference periods and cohorts must not be repeated.',
      })
  })
  .readonly()
export type SunAbrahamRequest = z.infer<typeof sunAbrahamRequestSchema>

export const ridgeRegularizationSchema = z
  .discriminatedUnion('kind', [
    z.object({ kind: z.literal('fixed'), lambda: finite.positive() }).strict(),
    z
      .object({
        kind: z.literal('crossValidation'),
        maximum: finite.positive().nullable(),
        minimumRatio: finite.gt(0).lt(1),
        steps: z.number().int().positive(),
        holdoutLength: z.number().int().positive(),
        selection: z.enum(['minimumError', 'oneStandardError']),
      })
      .strict(),
  ])
  .readonly()
export const ridgeUncertaintySchema = z
  .discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z.object({ kind: z.literal('jackknifePlus'), confidence }).strict(),
    z.object({ kind: z.literal('conservative'), confidence }).strict(),
  ])
  .readonly()
export const ridgeAugmentedRequestSchema = z
  .object({
    ...panel,
    treated: unit,
    donors: z.tuple([unit, unit]).rest(unit).readonly(),
    prePeriods: z.number().int().min(2),
    regularization: ridgeRegularizationSchema,
    uncertainty: ridgeUncertaintySchema,
  })
  .strict()
  .superRefine((r, ctx) => {
    panelIssues(r, ctx)
    if (r.donors.includes(r.treated) || new Set(r.donors).size !== r.donors.length)
      ctx.addIssue({
        code: 'custom',
        message: 'Choose distinct donors that do not include the treated unit.',
      })
    const axis = r.times.filter((_, i) => r.units[i] === r.treated).sort((a, b) => a - b)
    if (axis.length <= r.prePeriods || axis.some((t, i) => i > 0 && t - axis[i - 1]! !== 1))
      ctx.addIssue({
        code: 'custom',
        message: 'Retain consecutive pre-treatment and post-treatment periods.',
      })
    if (
      r.regularization.kind === 'crossValidation' &&
      r.regularization.holdoutLength >= r.prePeriods
    )
      ctx.addIssue({
        code: 'custom',
        message: 'A held-out block must leave pre-treatment observations for fitting.',
      })
    const keys = new Set(r.units.map((u, i) => JSON.stringify([u, r.times[i]])))
    if (r.donors.some((u) => axis.some((t) => !keys.has(JSON.stringify([u, t])))))
      ctx.addIssue({
        code: 'custom',
        message: 'Every donor must be observed at every selected treated-unit period.',
      })
  })
  .readonly()
export type RidgeAugmentedRequest = z.infer<typeof ridgeAugmentedRequestSchema>

const interval = z
  .object({
    estimate: finite,
    standardError: finite.nonnegative(),
    pValue: finite.min(0).max(1),
    lower: finite,
    upper: finite,
  })
  .strict()
  .refine((v) => v.lower <= v.upper, 'Interval bounds must be ordered.')
  .readonly()
const summary = z
  .object({ key: period, support: z.number().int().positive(), interval })
  .strict()
  .readonly()
export const sunAbrahamEvidenceSchema = z
  .object({
    kind: z.literal('sunAbraham'),
    times: z.array(period).min(1).readonly(),
    request: sunAbrahamRequestSchema,
    observations: z.number().int().positive(),
    clusters: z.number().int().min(2),
    retainedRows: z.array(z.number().int().nonnegative()).min(1).readonly(),
    removedRows: z.array(z.number().int().nonnegative()).readonly(),
    omitted: z.array(z.tuple([period, period]).readonly()).readonly(),
    cells: z
      .array(
        z
          .object({ cohort: period, event: period, support: z.number().int().positive(), interval })
          .strict()
          .readonly(),
      )
      .min(1)
      .readonly(),
    events: z.array(summary).min(1).readonly(),
    cohorts: z.array(summary).min(1).readonly(),
    overall: interval,
    covariance: z.array(z.array(finite).readonly()).readonly(),
  })
  .strict()
  .superRefine((e, ctx) => {
    const all = [...e.retainedRows, ...e.removedRows]
    const clusters = new Set(e.retainedRows.map((i) => e.request.units[i])).size
    if (
      e.observations !== e.retainedRows.length ||
      all.length !== e.request.rows ||
      new Set(all).size !== all.length ||
      all.some((i) => i >= e.request.rows) ||
      clusters !== e.clusters ||
      e.covariance.length !== e.cells.length ||
      e.covariance.some((r) => r.length !== e.cells.length)
    )
      ctx.addIssue({
        code: 'custom',
        message: 'The retained rows, clusters and covariance must match the fitted design.',
      })
  })
  .readonly()
export type SunAbrahamEvidence = z.infer<typeof sunAbrahamEvidenceSchema>

const ridgeBounds = z
  .discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z
      .object({
        kind: z.literal('jackknife'),
        lower: z.array(finite).readonly(),
        upper: z.array(finite).readonly(),
        averageLower: finite,
        averageUpper: finite,
      })
      .strict(),
  ])
  .readonly()
export const ridgeAugmentedEvidenceSchema = z
  .object({
    request: ridgeAugmentedRequestSchema,
    periods: z.array(period).min(3).readonly(),
    observed: z.array(finite).readonly(),
    synthetic: z.array(finite).readonly(),
    gaps: z.array(finite).readonly(),
    donorWeights: z.array(finite).readonly(),
    baselineWeights: z.array(finite.min(-1e-7)).readonly(),
    lambda: finite.positive(),
    average: finite,
    tuning: z
      .object({
        candidates: z.array(finite.positive()).min(1).readonly(),
        errors: z.array(finite.nonnegative()).readonly(),
        standardErrors: z.array(finite.nonnegative()).readonly(),
      })
      .strict()
      .readonly()
      .nullable(),
    bounds: ridgeBounds,
  })
  .strict()
  .superRefine((e, ctx) => {
    const axis = e.request.times
      .filter((_, i) => e.request.units[i] === e.request.treated)
      .sort((a, b) => a - b)
    const n = e.periods.length
    if (
      axis.length !== n ||
      axis.some((v, i) => v !== e.periods[i]) ||
      [e.observed, e.synthetic, e.gaps].some((a) => a.length !== n) ||
      e.donorWeights.length !== e.request.donors.length ||
      e.baselineWeights.length !== e.request.donors.length
    )
      ctx.addIssue({
        code: 'custom',
        message: 'The fitted paths and weights must match the selected panel.',
      })
    const bounds = e.bounds
    if (
      bounds.kind === 'jackknife' &&
      (bounds.lower.length !== n - e.request.prePeriods ||
        bounds.upper.length !== bounds.lower.length ||
        bounds.lower.some((v, i) => v > bounds.upper[i]!) ||
        bounds.averageLower > bounds.averageUpper)
    )
      ctx.addIssue({
        code: 'custom',
        message: 'The intervals must match the post-treatment periods.',
      })
    if (
      (e.request.uncertainty.kind === 'none') !== (e.bounds.kind === 'none') ||
      (e.request.regularization.kind === 'fixed') !== (e.tuning === null)
    )
      ctx.addIssue({
        code: 'custom',
        message: 'The fitted evidence must match the requested regularization and uncertainty.',
      })
    if (
      e.tuning !== null &&
      (e.tuning.candidates.length !== e.tuning.errors.length ||
        e.tuning.candidates.length !== e.tuning.standardErrors.length)
    )
      ctx.addIssue({
        code: 'custom',
        message: 'Every candidate must have a validation error and standard error.',
      })
  })
  .readonly()
export type RidgeAugmentedEvidence = z.infer<typeof ridgeAugmentedEvidenceSchema>
