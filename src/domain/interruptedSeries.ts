import { z } from 'zod'
import { assertNever, brand, err, ok, type Brand, type Result } from './dop'

/**
 * The interrupted time series of Lopez Bernal, Cummins and Gasparrini (IJE 2017): a segmented
 * regression of one series on time, the declared impact of an event, and harmonic seasonal terms.
 * Each choice is a variant carrying only what that choice needs; the shapes are the worker's,
 * checked at the boundary.
 */

/**
 * A row as the screen numbers it, from 1. The kernel counts from 0; the two never share a type,
 * and the one conversion is `rowIndex`, at the worker boundary.
 */
export type RowNumber = Brand<number, 'RowNumber'>
export const rowNumber = (value: number): RowNumber => brand<number, 'RowNumber'>(value)
const rowNumberSchema = z.number().int().positive().transform(rowNumber)
/** The kernel's 0-based index of a screen row. */
export const rowIndex = (row: RowNumber): number => row - 1

/** The impact model as the kernel takes it, rows 0-based. */
export const interruptedImpactSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('level') }).strict(),
  z.object({ kind: z.literal('levelAndSlope') }).strict(),
  z.object({ kind: z.literal('slope') }).strict(),
  /** A level change that ends at `until`, an exclusive 0-based row. */
  z.object({ kind: z.literal('temporaryLevel'), until: z.number().int().positive() }).strict(),
])
export type InterruptedImpact = z.infer<typeof interruptedImpactSchema>

/** The impact model as declared: `until` is the first screen row back at the old level. */
export const declaredImpactSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('level') }).strict(),
  z.object({ kind: z.literal('levelAndSlope') }).strict(),
  z.object({ kind: z.literal('slope') }).strict(),
  z.object({ kind: z.literal('temporaryLevel'), until: rowNumberSchema }).strict(),
])
export type DeclaredImpact = z.infer<typeof declaredImpactSchema>

/** The declaration the kernel receives: the first row back at the old level is the exclusive end index. */
export const impactForKernel = (impact: DeclaredImpact): InterruptedImpact =>
  impact.kind === 'temporaryLevel' ? { kind: 'temporaryLevel', until: rowIndex(impact.until) } : impact

/** The declared row numbers and the kernel's indexes name the same rows. */
export const sameImpact = (declared: DeclaredImpact, fitted: InterruptedImpact): boolean =>
  declared.kind === fitted.kind && (declared.kind !== 'temporaryLevel' || fitted.kind !== 'temporaryLevel' || rowIndex(declared.until) === fitted.until)

/** The largest ARMA order the kernel fits; a state of 13 is already past what a monthly or weekly series supports. */
export const MAX_ARMA_ORDER = 12
/** statsmodels' `maxiter` for the state-space fit; the result records whether the optimiser converged inside it. */
export const DEFAULT_ARMA_ITERATIONS = 50

/**
 * How a continuous series' terms are fitted: least squares with Newey–West errors (the kernel's
 * bandwidth when `maxLags` is null), or maximum likelihood with an ARMA(p, q) error process,
 * the error model of Hyndman and Athanasopoulos's dynamic harmonic regression.
 */
export const continuousErrorsSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('neweyWest'), maxLags: z.number().int().nonnegative().nullable() }).strict(),
  z.object({ kind: z.literal('arma'), p: z.number().int().min(0).max(MAX_ARMA_ORDER), q: z.number().int().min(0).max(MAX_ARMA_ORDER), maxIter: z.number().int().positive() }).strict()
    .refine((errors) => errors.p + errors.q > 0, { message: 'ARMA errors need at least one autoregressive or moving-average term.' }),
])
export type ContinuousErrors = z.infer<typeof continuousErrorsSchema>

/** The error process an adjusted regression is asked for beside its Newey–West reading. */
export const linearErrorModelSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('neweyWest') }).strict(),
  z.object({ kind: z.literal('arma'), p: z.number().int().min(0).max(MAX_ARMA_ORDER), q: z.number().int().min(0).max(MAX_ARMA_ORDER), maxIter: z.number().int().positive() }).strict()
    .refine((errors) => errors.p + errors.q > 0, { message: 'ARMA errors need at least one autoregressive or moving-average term.' }),
])
export type LinearErrorModel = z.infer<typeof linearErrorModelSchema>

/** The fit requested: a continuous series with its error model, or the paper's quasi-Poisson model. */
export const interruptedModelSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('continuous'), errors: continuousErrorsSchema }).strict(),
  z.object({ kind: z.literal('count'), exposure: z.number().int().nonnegative().nullable() }).strict(),
])
export type InterruptedModel = z.infer<typeof interruptedModelSchema>

export const interruptedSeasonalSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({ kind: z.literal('harmonic'), pairs: z.number().int().positive().max(12), period: z.number().finite().positive() }).strict(),
])
export type InterruptedSeasonal = z.infer<typeof interruptedSeasonalSchema>

export const interruptedTermSchema = z.object({
  name: z.string().min(1),
  coefficient: z.number().finite(),
  standardError: z.number().finite().nonnegative(),
  pValue: z.number().finite().min(0).max(1),
  interval: z.tuple([z.number().finite(), z.number().finite()]),
}).strict()
export type InterruptedTermEvidence = z.infer<typeof interruptedTermSchema>

const armaErrorShape = {
  p: z.number().int().min(0).max(MAX_ARMA_ORDER),
  q: z.number().int().min(0).max(MAX_ARMA_ORDER),
  ar: z.array(interruptedTermSchema),
  ma: z.array(interruptedTermSchema),
  sigma2: z.number().finite().positive(),
  logLikelihood: z.number().finite(),
  aic: z.number().finite(),
  bic: z.number().finite(),
  iterations: z.number().int().nonnegative(),
  converged: z.boolean(),
}
const wholeOrder = (e: { readonly p: number; readonly q: number; readonly ar: readonly unknown[]; readonly ma: readonly unknown[] }): boolean => e.ar.length === e.p && e.ma.length === e.q
const ORDER_MESSAGE = { message: 'The fitted error terms must match the declared order.' }

/** An ARMA(p, q) error process as fitted with the regression: `SARIMAX(y, exog, order=(p, 0, q))`. */
export const armaErrorFieldsSchema = z.object(armaErrorShape).strict().refine(wholeOrder, ORDER_MESSAGE)
export type ArmaErrorEvidence = z.infer<typeof armaErrorFieldsSchema>
/** The same process as a variant of a continuous series' error model. */
export const armaErrorEvidenceSchema = z.object({ kind: z.literal('arma'), ...armaErrorShape }).strict()

export const continuousErrorEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('neweyWest'), maxLags: z.number().int().nonnegative() }).strict(),
  armaErrorEvidenceSchema,
]).refine((errors) => errors.kind === 'neweyWest' || wholeOrder(errors), ORDER_MESSAGE)
export type ContinuousErrorEvidence = z.infer<typeof continuousErrorEvidenceSchema>

const interruptedModelEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('continuous'), errors: continuousErrorEvidenceSchema }).strict(),
  z.object({ kind: z.literal('count'), exposure: z.number().int().nonnegative().nullable(), dispersion: z.number().finite().positive() }).strict(),
])

const interruptedSeasonalEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('harmonic'),
    pairs: z.number().int().positive(),
    period: z.number().finite().positive(),
    /** The fit and counterfactual with the seasonal terms at one fixed phase, per row. */
    deseasonalised: z.array(z.object({ fitted: z.number().finite(), counterfactual: z.number().finite() }).strict()),
  }).strict(),
])

export const interruptedSeriesEvidenceSchema = z.object({
  kind: z.literal('interruptedSeries'),
  observations: z.number().int().positive(),
  outcome: z.number().int().nonnegative(),
  model: interruptedModelEvidenceSchema,
  interventionRow: z.number().int().positive(),
  lag: z.number().int().nonnegative(),
  impact: interruptedImpactSchema,
  seasonal: interruptedSeasonalEvidenceSchema,
  terms: z.array(interruptedTermSchema).min(2),
  /** One row per observation, on the plotted scale: the series itself, or a count standardised to the mean exposure. */
  path: z.array(z.object({ observed: z.number().finite(), fitted: z.number().finite(), counterfactual: z.number().finite(), residual: z.number().finite() }).strict()),
  ljungBox: z.array(z.object({ statistic: z.number().finite(), pValue: z.number().finite().min(0).max(1) }).strict()),
  residualAcf: z.array(z.object({ value: z.number().finite(), limit: z.number().finite() }).strict()),
  residualPacf: z.array(z.object({ value: z.number().finite(), limit: z.number().finite() }).strict()),
  converged: z.boolean(),
}).strict()
export type InterruptedSeriesEvidence = z.infer<typeof interruptedSeriesEvidenceSchema>

export type InterruptedSeriesBoundaryProblem = { readonly kind: 'invalid-interrupted-series-result'; readonly detail: string }

/** The design columns the kernel names, in order, for an impact model and seasonal choice. */
export const interruptedTermNames = (impact: InterruptedImpact, seasonal: { readonly kind: 'none' } | { readonly kind: 'harmonic'; readonly pairs: number }): readonly string[] => {
  const pairs = seasonal.kind === 'harmonic' ? seasonal.pairs : 0
  return [
    'const', 'time',
    ...(impact.kind === 'slope' ? [] : ['step']),
    ...(impact.kind === 'levelAndSlope' || impact.kind === 'slope' ? ['slope_change'] : []),
    ...Array.from({ length: pairs }, (_, k) => `sin${k + 1}`),
    ...Array.from({ length: pairs }, (_, k) => `cos${k + 1}`),
  ]
}

export function parseInterruptedSeriesEvidence(value: unknown): Result<InterruptedSeriesEvidence, InterruptedSeriesBoundaryProblem> {
  const parsed = interruptedSeriesEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-interrupted-series-result', detail: z.prettifyError(parsed.error) })
  const e = parsed.data
  if (e.path.length !== e.observations) return err({ kind: 'invalid-interrupted-series-result', detail: 'The fitted path must have one row per observation.' })
  if (e.seasonal.kind === 'harmonic' && e.seasonal.deseasonalised.length !== e.observations) return err({ kind: 'invalid-interrupted-series-result', detail: 'The deseasonalised trend must have one row per observation.' })
  if (e.residualAcf.length !== e.residualPacf.length) return err({ kind: 'invalid-interrupted-series-result', detail: 'The autocorrelation and partial autocorrelation must cover the same lags.' })
  if (e.interventionRow + e.lag >= e.observations) return err({ kind: 'invalid-interrupted-series-result', detail: 'The change must start inside the series.' })
  if (e.terms.map((term) => term.name).join(',') !== interruptedTermNames(e.impact, e.seasonal).join(',')) return err({ kind: 'invalid-interrupted-series-result', detail: 'The fitted terms do not match the declared impact model and seasonal terms.' })
  return ok(e)
}

/** The term that carries the event's effect on the level, when the impact model has one. */
export const levelTerm = (e: InterruptedSeriesEvidence): InterruptedTermEvidence | null => e.terms.find((term) => term.name === 'step') ?? null
export const slopeTerm = (e: InterruptedSeriesEvidence): InterruptedTermEvidence | null => e.terms.find((term) => term.name === 'slope_change') ?? null
export const trendTerm = (e: InterruptedSeriesEvidence): InterruptedTermEvidence => e.terms.find((term) => term.name === 'time')!

/** `exp(coefficient)` with its limits: the rate ratio a count model's term reads as, `ci.lin(model, Exp = TRUE)`. */
export const rateRatioOf = (term: InterruptedTermEvidence): { readonly ratio: number; readonly interval: readonly [number, number] } =>
  ({ ratio: Math.exp(term.coefficient), interval: [Math.exp(term.interval[0]), Math.exp(term.interval[1])] })

/** The requested error model and the fitted one name the same process. */
export const sameErrors = (requested: ContinuousErrors, fitted: ContinuousErrorEvidence): boolean => {
  switch (requested.kind) {
    case 'neweyWest': return fitted.kind === 'neweyWest' && (requested.maxLags === null || requested.maxLags === fitted.maxLags)
    case 'arma': return fitted.kind === 'arma' && requested.p === fitted.p && requested.q === fitted.q
    default: return assertNever(requested)
  }
}

export function describeErrors(errors: ContinuousErrors | ContinuousErrorEvidence): string {
  switch (errors.kind) {
    case 'neweyWest': return errors.maxLags === null ? 'Newey–West errors' : `Newey–West errors, bandwidth ${errors.maxLags}`
    case 'arma': return `ARMA(${errors.p}, ${errors.q}) errors`
    default: return assertNever(errors)
  }
}

export function describeImpact(impact: DeclaredImpact): string {
  switch (impact.kind) {
    case 'level': return 'level change'
    case 'levelAndSlope': return 'level and slope change'
    case 'slope': return 'slope change'
    case 'temporaryLevel': return `temporary level change until row ${impact.until}`
    default: return assertNever(impact)
  }
}
