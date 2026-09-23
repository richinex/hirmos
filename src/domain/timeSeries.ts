import { z } from 'zod'
import { plotTimeSchema } from './longRun'
import { ardlModelRequestSchema, ardlModelEvidenceSchema } from './ardlModel'
import { ardlEvidenceSchema, vecmEvidenceSchema } from './estimation'
import { continuousErrorsSchema, declaredImpactSchema, describeImpact, interruptedSeasonalSchema, interruptedSeriesEvidenceSchema, rowIndex, sameErrors, sameImpact } from './interruptedSeries'
import { assertNever, brand, err, ok, type Result } from './dop'
import type { ColumnId } from './dataset'
import type { PreparedDatasetVersionId } from './preprocessing'

const column = z.object({
  id: z.string().min(1).transform((value) => brand<string, 'ColumnId'>(value)),
  name: z.string().min(1),
}).strict()
const identity = {
  windowContext: windowContextSchema.optional(),
  plotTime: plotTimeSchema.optional(),
  id: z.string().min(1).transform((value) => brand<string, 'TimeSeriesRunId'>(value)),
  preparedDataset: z.string().min(1).transform((value) => brand<string, 'PreparedDatasetVersionId'>(value)),
  createdAt: z.string().datetime(),
}

/** Deterministic terms and bounds-test case are one choice, not independent switches. */
export const ARDL_TERMS = {
  'restricted-constant': { label: 'Restricted constant', trend: 'c', case: 2 },
  constant: { label: 'Constant', trend: 'c', case: 3 },
  'restricted-trend': { label: 'Constant and restricted trend', trend: 'ct', case: 4 },
  trend: { label: 'Constant and trend', trend: 'ct', case: 5 },
} as const
export type ArdlTerms = keyof typeof ARDL_TERMS

export const timeSeriesRunSchema = z.discriminatedUnion('kind', [
  z.object({
    ...identity, kind: z.literal('ardl-model'), outcome: column,
    predictors: z.tuple([column]).rest(column), fixed: z.array(column),
    specification: ardlModelRequestSchema, evidence: ardlModelEvidenceSchema,
  }).strict(),
  z.object({
    ...identity,
    kind: z.literal('ardl'),
    outcome: column,
    predictor: column,
    specification: z.object({ maxLag: z.number().int().min(1).max(24), terms: z.enum(['restricted-constant', 'constant', 'restricted-trend', 'trend']) }).strict(),
    evidence: ardlEvidenceSchema,
  }).strict(),
  z.object({
    ...identity,
    kind: z.literal('vecm'),
    variables: z.tuple([column, column]).rest(column),
    specification: z.object({ maxLags: z.number().int().min(1).max(24), deterministic: z.enum(['n', 'co', 'ci', 'coli']), significance: z.union([z.literal(90), z.literal(95), z.literal(99)]), forecastSteps: z.number().int().min(1).max(200).nullable().optional() }).strict(),
    evidence: vecmEvidenceSchema,
  }).strict(),
  z.object({
    ...identity,
    kind: z.literal('interrupted-series'),
    outcome: column,
    specification: z.object({
      /** The fit, with the column a count model's exposure came from. */
      model: z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('continuous'), errors: continuousErrorsSchema }).strict(),
        z.object({ kind: z.literal('count'), exposure: column.nullable() }).strict(),
      ]),
      /** The first screen row after the event, numbered from 1. */
      interventionRow: z.number().int().min(2).transform((value) => brand<number, 'RowNumber'>(value)),
      lag: z.number().int().nonnegative(),
      impact: declaredImpactSchema,
      seasonal: interruptedSeasonalSchema,
      ljungBoxLags: z.number().int().positive(),
    }).strict(),
    evidence: interruptedSeriesEvidenceSchema,
  }).strict(),
]).superRefine((run, ctx) => {
  const fail = (message: string) => ctx.addIssue({ code: 'custom', message })
  if (run.plotTime !== undefined && run.plotTime.values.length !== run.evidence.observations) fail('Chart dates must match the prepared observations.')
  switch (run.kind) {
    case 'ardl-model': {
      const { specification: s, evidence: e } = run
      const columns = [run.outcome, ...run.predictors, ...run.fixed]
      if (new Set(columns.map(c => c.id)).size !== columns.length) fail('Choose distinct model columns.')
      if (s.outcome !== 0 || s.predictors.length !== run.predictors.length || s.fixed.length !== run.fixed.length ||
        s.predictors.some((index, i) => index !== i + 1) || s.fixed.some((index, i) => index !== i + 1 + run.predictors.length)) fail('The specification must refer to the saved model columns in their recorded order.')
      if (e.predictorLags.length !== run.predictors.length) fail('The fitted orders must match the selected predictors.')
      if ((s.orders.kind === 'fixed' || s.orders.kind === 'rFixed') && (s.orders.outcomeLag !== e.outcomeLag || s.orders.predictorLags.some((q, i) => q !== e.predictorLags[i]))) fail('The fitted orders differ from the requested fixed orders.')
      if (s.orders.kind === 'search' && (e.outcomeLag > s.orders.maximumLag || e.predictorLags.some((q, i) => q !== null && q > (s.orders.kind === 'search' ? s.orders.maximumOrders[i] ?? -1 : -1)))) fail('The selected orders exceed the search limits.')
      const orders = s.orders
      const isR = orders.kind === 'rFixed' || orders.kind === 'rHorizontal' || orders.kind === 'rGrid'
      if (isR !== (e.rAnalysis.kind === 'recorded')) fail('R error-correction evidence must match the recorded fitting method.')
      if ((e.longRun.kind === 'uncalibrated' && !isR) || (e.longRun.kind === 'recorded' && isR)) fail('Bounds calibration must match the recorded fitting method.')
      if (orders.kind === 'rHorizontal') {
        const actual = [e.outcomeLag, ...e.predictorLags]
        if (actual.some((q,i)=>q===null||q>orders.maximum[i]!||(orders.fixed[i]!==null&&q!==orders.fixed[i]))) fail('The selected orders violate horizontal-search constraints.')
      }
      if (orders.kind === 'rGrid') {
        if (e.outcomeLag < orders.minimumLag || e.outcomeLag > orders.maximumLag || e.predictorLags.some((q,i)=>q===null||q>orders.maximumOrders[i]!||(orders.fixedOrders[i]!==null&&q!==orders.fixedOrders[i]))) fail('The selected orders violate grid-search constraints.')
      }
      if (e.rAnalysis.kind === 'recorded') {
        const r = e.rAnalysis
        const expected = orders.kind === 'rHorizontal' ? 'horizontal' : orders.kind === 'rGrid' ? 'grid' : 'notRequested'
        if (r.ranking.kind !== expected) fail('The search ranking must match the requested search method.')
        const unrestricted = s.terms === 'constant' || s.terms === 'trend'
        if (unrestricted !== (r.boundsT.kind === 'recorded')) fail('The t-bounds statistic applies only to unrestricted deterministic cases.')
        if (r.coefficients.some(term=>(term.kind==='predictor'||term.kind==='predictorChange')&&term.column>=run.predictors.length||term.kind==='fixed'&&term.column>=run.fixed.length)) fail('An error-correction coefficient refers to an unknown column.')
      }
      if (e.coefficients.some(term => term.kind === 'fixed' && term.column >= run.fixed.length)) fail('A fitted coefficient refers to an unknown fixed regressor.')
      switch (s.future.kind) {
        case 'none': if (e.forecast.kind !== 'notRequested') fail('A forecast requires a recorded future scenario.'); break
        case 'scenario': if (e.forecast.kind !== 'recorded' || e.forecast.mean.length !== s.future.predictors[0]?.length || e.forecast.confidence !== s.future.confidence) fail('The forecast must match its recorded scenario and confidence level.'); break
        default: assertNever(s.future)
      }
      return
    }
    case 'ardl': {
      if (run.evidence.longRun !== undefined && run.evidence.longRun.observed.length !== run.evidence.observations) fail('ARDL chart values must cover every prepared observation.')
      if (run.outcome.id === run.predictor.id) fail('Choose different outcome and predictor series.')
      const terms = ARDL_TERMS[run.specification.terms]
      if (run.evidence.trend !== terms.trend || run.evidence.case !== terms.case) fail('The ARDL result does not match its deterministic terms.')
      if (run.evidence.interval[0] > run.evidence.interval[1]) fail('The ARDL interval limits are reversed.')
      return
    }
    case 'vecm': {
      const { evidence, variables, specification } = run
      const forecast=evidence.forecast
      if(forecast?.kind==='recorded'&&(forecast.mean.length!==specification.forecastSteps||forecast.mean.some(row=>row.length!==variables.length)||evidence.rank===0))fail('The forecast must match the selected series, horizon and fitted rank.')
      if(specification.forecastSteps!=null&&(forecast===undefined||forecast.kind==='notRequested'))fail('The requested forecast is missing from the result.')
      if(forecast?.kind==='notFitted'&&(evidence.rank!==0||specification.forecastSteps==null))fail('An unfitted forecast requires a rank-zero model and a requested horizon.')
      const longRun = evidence.longRun
      if (longRun?.kind === 'notFitted' && evidence.rank !== 0) fail('A positive-rank VECM must have a fitted coefficient model.')
      if (longRun?.kind === 'recorded') {
        if (evidence.rank === 0 || longRun.departures.length !== evidence.rank) fail('Long-run chart dimensions must match the fitted rank.')
        if (longRun.startRow !== evidence.kArDiff || longRun.departures.some((row) => row.length !== evidence.observations - evidence.kArDiff - 1)) fail('VECM chart dates must align with the lagged levels used in estimation.')
      }
      if (new Set(variables.map((value) => value.id)).size !== variables.length) fail('Choose distinct VECM series.')
      if (evidence.deterministic !== specification.deterministic || [90, 95, 99][evidence.significance] !== specification.significance) fail('The VECM result does not match its specification.')
      if (evidence.rank > variables.length) fail('The cointegration rank exceeds the number of series.')
      for (const matrix of [evidence.alpha, evidence.beta, evidence.gamma, evidence.pvaluesAlpha]) {
        if (matrix.some((row) => row.some((value) => !Number.isFinite(value)))) fail('The VECM result contains non-finite coefficients.')
      }
      const expectedRows = evidence.rank === 0 ? 0 : variables.length
      for (const matrix of [evidence.alpha, evidence.beta, evidence.pvaluesAlpha]) {
        if (matrix.length !== expectedRows || matrix.some((row) => row.length !== evidence.rank)) fail('The VECM coefficient dimensions do not match the series and rank.')
      }
      return
    }
    case 'interrupted-series': {
      const { specification: s, evidence: e } = run
      if (s.model.kind !== e.model.kind || rowIndex(s.interventionRow) !== e.interventionRow || s.lag !== e.lag || !sameImpact(s.impact, e.impact)) fail('The interrupted-series result does not match its specification.')
      if (s.seasonal.kind !== e.seasonal.kind || (s.seasonal.kind === 'harmonic' && e.seasonal.kind === 'harmonic' && (s.seasonal.pairs !== e.seasonal.pairs || s.seasonal.period !== e.seasonal.period))) fail('The seasonal terms differ from the requested ones.')
      if (s.model.kind === 'continuous' && e.model.kind === 'continuous' && !sameErrors(s.model.errors, e.model.errors)) fail('The fitted error model differs from the requested one.')
      if (s.model.kind === 'count' && e.model.kind === 'count' && (s.model.exposure === null) !== (e.model.exposure === null)) fail('An exposure column must be recorded with the count model that used it.')
      return
    }
    default: return assertNever(run)
  }
})

export type TimeSeriesRun = z.infer<typeof timeSeriesRunSchema>
export type TimeSeriesRunId = TimeSeriesRun['id']
export const newTimeSeriesRunId = (): TimeSeriesRunId => brand(crypto.randomUUID())

export function parseTimeSeriesRun(value: unknown): Result<TimeSeriesRun, string> {
  const parsed = timeSeriesRunSchema.safeParse(value)
  return parsed.success ? ok(parsed.data) : err(parsed.error.issues.map((issue) => issue.message).join(' '))
}

export const timeSeriesRunLabel = (run: TimeSeriesRun): string => {
  switch (run.kind) {
    case 'ardl-model': return `ARDL for ${run.outcome.name}`
    case 'ardl': return `ARDL for ${run.outcome.name} and ${run.predictor.name}`
    case 'vecm': return `VECM for ${run.variables.map((variable) => variable.name).join(', ')}`
    case 'interrupted-series': return `Interrupted series for ${run.outcome.name}, ${describeImpact(run.specification.impact)}`
    default: return assertNever(run)
  }
}

/** A run can only enter the history of the prepared series it actually used. */
export const timeSeriesRunMatches = (run: TimeSeriesRun, prepared: { readonly id: PreparedDatasetVersionId; readonly columns: readonly ColumnId[] }): boolean => {
  const variables = run.kind === 'ardl-model' ? [run.outcome, ...run.predictors, ...run.fixed] : run.kind === 'ardl' ? [run.outcome, run.predictor] : run.kind === 'interrupted-series' ? [run.outcome, ...(run.specification.model.kind === 'count' && run.specification.model.exposure !== null ? [run.specification.model.exposure] : [])] : run.variables
  return run.preparedDataset === prepared.id && variables.every((variable) => prepared.columns.includes(variable.id))
}
import { windowContextSchema } from './windowEvidence'
