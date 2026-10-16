import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import type { ArdlDraft, CountDraft, InterruptedDraft, LongRunDraft } from './timeSeriesDraft'
import { ARDL_TERMS } from './timeSeries'
import { MAX_ARMA_ORDER } from './interruptedSeries'
import { seriesLagSchema } from './timeSeriesEstimatorSettings'
import { describeCountSeriesReadiness, readyCountSeriesSpecification } from './countSeries'
import type { ConfigurationSetting } from './estimation'
import { clusterCovarianceLabels } from './panelRegression'

const column = z
  .string()
  .min(1)
  .transform((id) => brand<string, 'ColumnId'>(id))
const terms = z.enum(['restricted-constant', 'constant', 'restricted-trend', 'trend'])
const numberText = z
  .string()
  .refine((text) => text.trim() !== '' && Number.isFinite(Number(text)), 'Enter a finite number.')
const integerText = (min: number, max = Number.MAX_SAFE_INTEGER) =>
  numberText.refine(
    (text) => Number.isSafeInteger(Number(text)) && Number(text) >= min && Number(text) <= max,
    `Enter a whole number from ${min} to ${max}.`,
  )
const optionalInteger = (min: number, max = Number.MAX_SAFE_INTEGER) =>
  z.union([z.literal(''), integerText(min, max)])
const lags = z
  .tuple([seriesLagSchema], seriesLagSchema)
  .refine(
    (values) => values.every((n, i) => i === 0 || n > values[i - 1]!),
    'Use distinct lags in increasing order.',
  )

export const longRunDraftSchema: z.ZodType<LongRunDraft> = z
  .object({
    selected: z.array(column).refine((ids) => new Set(ids).size === ids.length),
    maxLag: seriesLagSchema,
    terms,
    deterministic: z.enum(['n', 'co', 'ci', 'coli']),
    significance: z.union([z.literal(90), z.literal(95), z.literal(99)]),
    forecastSteps: optionalInteger(1, 200),
  })
  .strict()

export const countDraftSchema: z.ZodType<CountDraft> = z
  .object({
    outcome: column.nullable(),
    link: z.enum(['identity', 'log']),
    pastObservationLags: lags,
    pastMeanLags: lags,
    candidateStart: z.number().int().positive(),
    candidateEnd: z.number().int().positive(),
    delta: z.number().min(0).max(1),
  })
  .strict()

export const ardlDraftSchema: z.ZodType<ArdlDraft> = z
  .object({
    outcome: column.nullable(),
    roles: z.record(
      z.string(),
      z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('unused') }).strict(),
        z.object({ kind: z.literal('fixed') }).strict(),
        z.object({ kind: z.literal('predictor'), lag: integerText(0) }).strict(),
      ]),
    ),
    mode: z.enum(['fixed', 'search', 'rFixed', 'rHorizontal', 'rGrid']),
    starting: z.record(z.string(), integerText(0)),
    fixedOrders: z.record(z.string(), optionalInteger(0)),
    minimum: integerText(0),
    outcomeLag: integerText(0),
    holdBack: optionalInteger(0),
    terms,
    horizon: integerText(0),
    forecastSteps: integerText(1),
    future: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('none') }).strict(),
      z.object({ kind: z.literal('scenario'), columns: z.record(z.string(), z.string()) }).strict(),
    ]),
    omitted: z.array(z.object({ column, lag: z.number().int().nonnegative() }).strict()),
  })
  .strict()

export const interruptedDraftSchema: z.ZodType<InterruptedDraft> = z
  .object({
    outcome: column.nullable(),
    model: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('count'), exposure: column.nullable() }).strict(),
      z
        .object({
          kind: z.literal('continuous'),
          errors: z.discriminatedUnion('kind', [
            z.object({ kind: z.literal('neweyWest'), maxLags: optionalInteger(0) }).strict(),
            z
              .object({
                kind: z.literal('arma'),
                p: integerText(0, MAX_ARMA_ORDER),
                q: integerText(0, MAX_ARMA_ORDER),
                maxIter: integerText(1),
              })
              .strict()
              .refine(
                (value) => Number(value.p) + Number(value.q) > 0,
                'Choose at least one AR or MA term.',
              ),
          ]),
        })
        .strict(),
    ]),
    interventionRow: integerText(2),
    lag: integerText(0),
    harmonicPairs: integerText(0, 12),
    impact: z.discriminatedUnion('kind', [
      z.object({ kind: z.enum(['level', 'levelAndSlope', 'slope']) }).strict(),
      z.object({ kind: z.literal('temporaryLevel'), until: integerText(2) }).strict(),
    ]),
  })
  .strict()

export const timeSeriesAnalysisSchema = z
  .object({ analysis: z.enum(['count', 'ardl', 'vecm', 'interrupted', 'regression']) })
  .strict()
export const timeSeriesAnalysisLabels = {
  count: 'Count models',
  ardl: 'ARDL',
  vecm: 'VECM',
  interrupted: 'Interrupted series',
  regression: 'Count regression',
} as const

export function checkSettingsColumns<T>(
  value: T,
  ids: readonly (string | null)[],
  columns: readonly { readonly id: string; readonly name: string }[],
): Result<T, string> {
  const missing = ids.find((id) => id !== null && !columns.some((c) => c.id === id))
  return missing === undefined
    ? ok(value)
    : err('A selected column is not available in this prepared dataset.')
}

export function checkCountDraft(value: CountDraft, rows: number): Result<CountDraft, string> {
  if (value.outcome === null) return err('Choose the count series.')
  const checked = readyCountSeriesSpecification({ ...value, outcome: value.outcome }, rows, null)
  return checked.ok ? ok(value) : err(describeCountSeriesReadiness(checked.error))
}

export function checkInterruptedDraft(
  value: InterruptedDraft,
  rows: number,
): Result<InterruptedDraft, string> {
  if (value.outcome === null) return err('Choose the series.')
  const first = Number(value.interventionRow) + Number(value.lag)
  if (first > rows) return err('The first changed row must be inside the prepared series.')
  if (
    value.impact.kind === 'temporaryLevel' &&
    (Number(value.impact.until) <= first || Number(value.impact.until) > rows + 1)
  )
    return err(
      'The temporary change must end after it starts and no later than one row past the series.',
    )
  if (value.model.kind === 'count' && value.model.exposure === value.outcome)
    return err('The exposure cannot be the outcome series.')
  return ok(value)
}

const labels: Readonly<Record<string, string>> = {
  covariance: 'Clustered covariance',
  absorbed: 'Absorbed fixed effect',
  outcome: 'Outcome',
  selected: 'Series',
  maxLag: 'Maximum lag',
  terms: 'Deterministic terms',
  deterministic: 'Deterministic terms',
  significance: 'Trace significance',
  forecastSteps: 'Forecast periods',
  link: 'Mean link',
  pastObservationLags: 'Past count lags',
  pastMeanLags: 'Past mean lags',
  candidateStart: 'First candidate row',
  candidateEnd: 'Last candidate row',
  delta: 'Intervention shape',
  roles: 'Column roles',
  kind: 'Choice',
  lag: 'Lag',
  mode: 'Lag selection',
  starting: 'Starting orders',
  fixedOrders: 'Fixed orders',
  minimum: 'Minimum lag',
  outcomeLag: 'Outcome lag',
  holdBack: 'Hold-back rows',
  horizon: 'Multiplier horizon',
  future: 'Future values',
  columns: 'Columns',
  omitted: 'Omitted changes',
  column: 'Column',
  model: 'Outcome model',
  exposure: 'Exposure',
  errors: 'Errors',
  maxLags: 'HAC bandwidth',
  p: 'AR order',
  q: 'MA order',
  maxIter: 'Iteration limit',
  interventionRow: 'Intervention row',
  harmonicPairs: 'Harmonic pairs',
  impact: 'Change',
  until: 'End row',
  analysis: 'Analysis',
  cluster: 'Cluster',
  weight: 'Sample weights',
  confidence: 'Confidence level',
  family: 'Outcome model',
  trials: 'Trials',
  onset: 'Treatment indicator',
  cohort: 'Cohort period',
  window: 'Event window',
  first: 'First event period',
  last: 'Last event period',
  reference: 'Reference period',
  tails: 'Window tails',
  covariates: 'Covariates',
  predictors: 'Predictors',
  coding: 'Encoding',
  iterations: 'Iteration limit',
  tolerance: 'Tolerance',
  intervention: 'Intervention row',
  bandwidth: 'HAC bandwidth',
  lags: 'Predictor lags',
}
const choices: Readonly<Record<string, string>> = {
  ...timeSeriesAnalysisLabels,
  ...Object.fromEntries(Object.entries(ARDL_TERMS).map(([key, value]) => [key, value.label])),
  n: 'None',
  co: 'Constant outside',
  ci: 'Constant inside',
  coli: 'Constant and trend',
  identity: 'Additive',
  log: 'Multiplicative',
  unused: 'Unused',
  predictor: 'Predictor',
  fixed: 'Specify lags',
  search: 'Search lags',
  rFixed: 'Specify lags with diagnostics',
  rHorizontal: 'Horizontal search',
  rGrid: 'Grid search',
  none: 'None',
  scenario: 'Supplied scenario',
  continuous: 'Continuous',
  neweyWest: 'Newey–West',
  arma: 'ARMA',
  level: 'Level',
  levelAndSlope: 'Level and slope',
  slope: 'Slope',
  temporaryLevel: 'Temporary level',
  negativeBinomial: 'Negative binomial',
  binomial: 'Grouped binomial',
  events: 'Event study',
  eventStudy: 'Event study',
  summary: 'Cohort summary',
  interactions: 'Interactions / DDD',
  ...clusterCovarianceLabels,
  bacon: 'Bacon decomposition',
  numeric: 'Numeric',
  categorical: 'Categorical',
  bin: 'Bin tails',
  reference: 'Reference',
  all: 'All supported periods',
  finite: 'Selected window',
}
export function seriesDraftSettings(
  value: object,
  columns: readonly { readonly id: string; readonly name: string }[],
): readonly ConfigurationSetting[] {
  const name = (key: string) => columns.find((c) => c.id === key)?.name ?? labels[key] ?? key
  const display = (entry: unknown): string => {
    if (entry === null || entry === '') return 'Not set'
    if (Array.isArray(entry)) return entry.length ? entry.map(display).join(', ') : 'None'
    if (typeof entry === 'object')
      return Object.entries(entry)
        .map(
          ([key, v]) =>
            `${key === 'terms' && Array.isArray(v) ? 'Model terms' : name(key)}: ${key === 'kind' && v === 'fixed' ? 'Fixed regressor' : display(v)}`,
        )
        .join('; ')
    if (typeof entry === 'string')
      return columns.find((c) => c.id === entry)?.name ?? choices[entry] ?? entry
    return String(entry)
  }
  return Object.entries(value).map(([key, v]) => ({ key, label: name(key), value: display(v) }))
}
