import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import type { ColumnId } from './dataset'
import type { PanelKeyMatrix } from './panel'

const finite = z.number().finite()
const index = z.number().int().nonnegative()
const periods = z.array(finite).min(1).refine(values => new Set(values).size === values.length, 'Choose each period only once.')
export const predictorSummarySchema = z.enum(['mean', 'median', 'minimum', 'maximum', 'sum', 'variance', 'standard-deviation'])
export const predictorSyntheticRequestSchema = z.object({
  rows: z.number().int().positive(),
  columnNames: z.array(z.string().min(1)).min(1),
  units: z.array(finite).min(1), times: z.array(finite).min(1),
  treated: finite,
  donors: z.array(finite).min(2), outcome: index,
  predictors: z.array(index), summary: predictorSummarySchema,
  special: z.array(z.object({ column: index, periods, summary: predictorSummarySchema }).strict()),
  predictorPeriods: periods, fitPeriods: periods, plotPeriods: periods,
  selection: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('automatic') }).strict(),
    z.object({ kind: z.literal('supplied'), weights: z.array(finite.nonnegative()).min(1).refine(weights => weights.some(w => w > 0), 'Predictor weights need a positive total.') }).strict(),
  ]),
}).strict().superRefine((model, context) => {
  const reject = (message: string) => context.addIssue({ code: 'custom', message })
  if (model.units.length !== model.rows || model.times.length !== model.rows) reject('The panel keys must describe every matrix row.')
  if (new Set(model.columnNames).size !== model.columnNames.length) reject('Column names must be distinct.')
  if (new Set(model.donors).size !== model.donors.length || model.donors.includes(model.treated)) reject('Choose distinct donor units without including the treated unit.')
  if (new Set(model.predictors).size !== model.predictors.length) reject('Choose each ordinary predictor only once.')
  if (model.predictors.length + model.special.length === 0) reject('Choose an ordinary or period-specific predictor.')
  if ([model.outcome, ...model.predictors, ...model.special.map(p => p.column)].some(column => column >= model.columnNames.length)) reject('A selected column is outside the supplied matrix.')
  if (model.selection.kind === 'supplied' && model.selection.weights.length !== model.predictors.length + model.special.length) reject('Supply one weight for every summarized predictor.')
})
export type PredictorSyntheticRequest = z.infer<typeof predictorSyntheticRequestSchema>

const start = z.enum(['equal', 'regression'])
const method = z.enum(['nelder-mead', 'bfgs'])
const count = z.number().int().nonnegative()
const selection = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('single-predictor') }).strict(),
  z.object({ kind: z.literal('supplied-weights') }).strict(),
  z.object({ kind: z.literal('optimized'), start, method, evaluations: count, convergenceCode: count }).strict(),
])
export const predictorSyntheticEvidenceSchema = z.object({
  request: predictorSyntheticRequestSchema,
  treatedUnit: finite, donors: z.array(finite).min(2), donorWeights: z.array(finite.nonnegative()).min(2),
  fitPeriods: periods, plotPeriods: periods,
  observed: z.array(finite).min(1), synthetic: z.array(finite.nullable()).min(1), gaps: z.array(finite.nullable()).min(1),
  balance: z.array(z.object({ predictor: z.string().min(1), treated: finite, synthetic: finite, donorMean: finite, weight: finite.nonnegative() }).strict()).min(1),
  predictorLoss: finite.nonnegative(), outcomeMspe: finite.nonnegative(), donorIterations: count,
  selection,
  attempts: z.array(z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('completed'), start, method, evaluations: count, convergenceCode: count, outcomeMspe: finite.nonnegative() }).strict(),
    z.object({ kind: z.literal('failed'), start, method, evaluations: count, detail: z.string().min(1) }).strict(),
  ])),
  preparationNotes: z.array(z.string().min(1)),
}).strict().superRefine((evidence, context) => {
  const reject = (message: string) => context.addIssue({ code: 'custom', message })
  if (evidence.donors.length !== evidence.donorWeights.length) reject('The donor weights do not match the donor axis.')
  if ([evidence.observed, evidence.synthetic, evidence.gaps].some(path => path.length !== evidence.plotPeriods.length)) reject('The paths do not match the plot periods.')
  if (Math.abs(evidence.donorWeights.reduce((sum, weight) => sum + weight, 0) - 1) > 1e-6) reject('Donor weights must sum to one.')
  if (Math.abs(evidence.balance.reduce((sum, row) => sum + row.weight, 0) - 1) > 1e-8) reject('Predictor weights must sum to one.')
  if (evidence.treatedUnit !== evidence.request.treated) reject('The result names a different treated unit.')
  if (evidence.balance.length !== evidence.request.predictors.length + evidence.request.special.length) reject('The predictor balance does not match the requested summaries.')
  evidence.synthetic.forEach((value, i) => {
    const gap = evidence.gaps[i]
    if (value === null ? gap !== null : gap == null || Math.abs(evidence.observed[i]! - value - gap) > 1e-10 * Math.max(1, Math.abs(gap))) reject('A reported gap does not match the observed and synthetic paths.')
  })
  const selected = evidence.selection
  if (evidence.balance.length === 1 ? selected.kind !== 'single-predictor'
    : evidence.request.selection.kind === 'supplied' ? selected.kind !== 'supplied-weights'
    : selected.kind !== 'optimized') reject('The predictor-weight selection does not match the requested fit.')
  if (selected.kind === 'optimized' && !evidence.attempts.some(attempt => attempt.kind === 'completed' && attempt.start === selected.start && attempt.method === selected.method && attempt.evaluations === selected.evaluations && attempt.convergenceCode === selected.convergenceCode)) reject('The selected optimizer is absent from the completed attempts.')
  const sorted = (values: readonly number[]) => [...values].sort((a, b) => a - b)
  const same = (a: readonly number[], b: readonly number[]) => a.length === b.length && a.every((value, i) => value === b[i])
  if (!same(evidence.donors, sorted(evidence.request.donors)) || !same(evidence.plotPeriods, sorted(evidence.request.plotPeriods)) || !same(evidence.fitPeriods, sorted(evidence.request.fitPeriods))) reject('The result axes differ from the requested units or periods.')
})
export type PredictorSyntheticEvidence = z.infer<typeof predictorSyntheticEvidenceSchema>
export const samePredictorSyntheticRequest = (a: PredictorSyntheticRequest, b: PredictorSyntheticRequest): boolean => {
  const same = <T>(left: readonly T[], right: readonly T[]) => left.length === right.length && left.every((value, i) => value === right[i])
  return a.rows === b.rows && a.treated === b.treated && a.outcome === b.outcome && a.summary === b.summary
    && same(a.columnNames, b.columnNames) && same(a.units, b.units) && same(a.times, b.times)
    && same(a.donors, b.donors) && same(a.predictors, b.predictors)
    && same(a.predictorPeriods, b.predictorPeriods) && same(a.fitPeriods, b.fitPeriods) && same(a.plotPeriods, b.plotPeriods)
    && a.special.length === b.special.length && a.special.every((value, i) => {
      const other = b.special[i]!
      return value.column === other.column && value.summary === other.summary && same(value.periods, other.periods)
    })
    && (a.selection.kind === 'automatic' ? b.selection.kind === 'automatic'
      : b.selection.kind === 'supplied' && same(a.selection.weights, b.selection.weights))
}

const column = z.string().min(1).transform(value => brand<string, 'ColumnId'>(value))
export const predictorSyntheticConfigurationSchema = z.object({
  kind: z.literal('synthetic-control'), specification: z.literal('predictors'),
  treatedUnit: z.string().min(1).nullable(), donorUnits: z.array(z.string().min(1)).readonly(),
  predictors: z.array(column).readonly(), summary: predictorSummarySchema,
  special: z.array(z.object({ column, periods: z.array(finite).readonly(), summary: predictorSummarySchema }).strict()).readonly(),
  predictorPeriods: z.array(finite).readonly(), fitPeriods: z.array(finite).readonly(), plotPeriods: z.array(finite).readonly(),
  interventionPeriod: finite.nullable(),
  selection: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('automatic') }).strict(),
    z.object({ kind: z.literal('supplied'), weights: z.array(finite.nonnegative()).readonly() }).strict(),
  ]),
}).strict()
export type PredictorSyntheticConfiguration = z.infer<typeof predictorSyntheticConfigurationSchema>
export const defaultPredictorSyntheticConfiguration = (): PredictorSyntheticConfiguration => ({
  kind: 'synthetic-control', specification: 'predictors', treatedUnit: null, donorUnits: [],
  predictors: [], summary: 'mean', special: [], predictorPeriods: [], fitPeriods: [], plotPeriods: [],
  interventionPeriod: null, selection: { kind: 'automatic' },
})

const axisEntry = z.object({ code: finite, label: z.string().min(1) }).strict()
export const predictorSyntheticCatalogSchema = z.object({ units: z.array(axisEntry).min(1), periods: z.array(axisEntry).min(1) }).strict().refine(catalog => [catalog.units, catalog.periods].every(axis => new Set(axis.map(entry => entry.code)).size === axis.length && new Set(axis.map(entry => entry.label)).size === axis.length), 'Panel axis labels and codes must be unique.')
export type PredictorSyntheticCatalog = z.infer<typeof predictorSyntheticCatalogSchema>
export function predictorSyntheticCatalog(keys: PanelKeyMatrix): PredictorSyntheticCatalog {
  const labels = [...new Set(keys.units)].sort((a, b) => a.localeCompare(b, undefined, { numeric: true }))
  const numeric = labels.map(Number)
  const numericUnits = numeric.every(Number.isFinite) && new Set(numeric).size === labels.length
  const numericPeriods = keys.periods.map(period => Number(period.label))
  const useNumericPeriods = numericPeriods.every(Number.isFinite) && new Set(numericPeriods).size === keys.periods.length
  return {
    units: labels.map((label, index) => ({ label, code: numericUnits ? numeric[index]! : index })),
    periods: keys.periods.map((period, index) => ({ label: period.label, code: useNumericPeriods ? numericPeriods[index]! : period.code })),
  }
}

export function predictorSyntheticModel(configuration: PredictorSyntheticConfiguration, catalog: PredictorSyntheticCatalog,
  keys: PanelKeyMatrix, columns: readonly { readonly id: ColumnId; readonly name: string }[], outcome: ColumnId,
): Result<PredictorSyntheticRequest, string> {
  const at = (id: ColumnId) => columns.findIndex(column => column.id === id)
  const unitCode = (label: string) => catalog.units.find(unit => unit.label === label)?.code
  const periodCode = (code: number) => catalog.periods.find(period => period.label === keys.periods.find(p => p.code === code)?.label)?.code
  const parsed = predictorSyntheticRequestSchema.safeParse({
    rows: keys.rowCount, columnNames: columns.map(column => column.name), units: keys.units.map(unitCode), times: keys.periodCodes.map(periodCode),
    treated: configuration.treatedUnit === null ? undefined : unitCode(configuration.treatedUnit),
    donors: configuration.donorUnits.map(unitCode), outcome: at(outcome), predictors: configuration.predictors.map(at), summary: configuration.summary,
    special: configuration.special.map(p => ({ column: at(p.column), periods: p.periods, summary: p.summary })),
    predictorPeriods: configuration.predictorPeriods, fitPeriods: configuration.fitPeriods, plotPeriods: configuration.plotPeriods,
    selection: configuration.selection,
  })
  if (!parsed.success) return err(parsed.error.issues.map(issue => issue.message).join(' '))
  if (configuration.interventionPeriod === null || !catalog.periods.some(period => period.code === configuration.interventionPeriod)) return err('Choose the intervention period.')
  if ([...configuration.fitPeriods, ...configuration.predictorPeriods, ...configuration.special.flatMap(p => p.periods)].some(period => period >= configuration.interventionPeriod!)) return err('Predictor and fitting periods must precede the intervention.')
  if ([...configuration.fitPeriods, ...configuration.predictorPeriods, ...configuration.plotPeriods, ...configuration.special.flatMap(p => p.periods)].some(code => !catalog.periods.some(p => p.code === code))) return err('A selected period is outside the prepared panel.')
  if (configuration.plotPeriods.every(period => period < configuration.interventionPeriod!)) return err('Choose at least one post-intervention plot period.')
  return ok(parsed.data)
}

export function predictorSyntheticRecordMatches(configuration: PredictorSyntheticConfiguration, evidence: PredictorSyntheticEvidence,
  catalog: PredictorSyntheticCatalog, columns: readonly { readonly column: ColumnId; readonly name: string }[], outcome: ColumnId,
): boolean {
  // Rebuild only the declared roles and axes. Source-row keys are already recorded in the worker request.
  const at = (id: ColumnId) => columns.findIndex(column => column.column === id)
  const code = (label: string | null) => catalog.units.find(unit => unit.label === label)?.code
  const r = evidence.request
  const same = (left: readonly number[], right: readonly number[]) => left.length === right.length && left.every((v, i) => v === right[i])
  return configuration.interventionPeriod !== null && catalog.periods.some(period => period.code === configuration.interventionPeriod)
    && configuration.fitPeriods.length >= 2 && [...configuration.fitPeriods, ...configuration.predictorPeriods, ...configuration.special.flatMap(p => p.periods)].every(p => p < configuration.interventionPeriod!)
    && configuration.plotPeriods.some(p => p >= configuration.interventionPeriod!)
    && r.outcome === at(outcome) && r.treated === code(configuration.treatedUnit) && same(r.donors, configuration.donorUnits.map(label => code(label) ?? NaN))
    && same(r.predictors, configuration.predictors.map(at)) && r.summary === configuration.summary
    && same(r.predictorPeriods, configuration.predictorPeriods) && same(r.fitPeriods, configuration.fitPeriods) && same(r.plotPeriods, configuration.plotPeriods)
    && configuration.special.length === r.special.length && configuration.special.every((p, i) => p.column === columns[r.special[i]!.column]?.column && p.summary === r.special[i]!.summary && same(p.periods, r.special[i]!.periods))
    && r.columnNames.length === columns.length && r.columnNames.every((name, i) => name === columns[i]?.name)
    && r.units.every(unit => catalog.units.some(entry => entry.code === unit)) && r.times.every(time => catalog.periods.some(period => period.code === time))
    && (configuration.selection.kind === 'automatic' ? r.selection.kind === 'automatic' : r.selection.kind === 'supplied' && same(configuration.selection.weights, r.selection.weights))
}

/** A saved headline and path must describe the recorded fit, not a separately edited result. */
export function predictorSyntheticEstimateMatches(value: unknown, configuration: PredictorSyntheticConfiguration, evidence: PredictorSyntheticEvidence): boolean {
  const point = z.object({ step: finite, actual: finite, counterfactual: finite, lower: finite, upper: finite, effect: finite })
  const parsed = z.object({
    kind: z.literal('causal-estimate'), estimand: z.object({ kind: z.literal('average-treatment-effect-on-treated') }),
    effect: z.object({ kind: z.literal('path'), values: z.array(point).min(1), aggregate: z.object({ cumulative: finite, average: finite }) }),
    interval: z.object({ kind: z.literal('none') }), standardError: z.null(), adjustment: z.object({ kind: z.literal('none') }),
    sample: z.object({ observations: z.number().int(), parameters: z.number().int(), degreesOfFreedom: z.null() }),
  }).safeParse(value)
  if (!parsed.success || configuration.interventionPeriod === null) return false
  const post = evidence.plotPeriods.flatMap((step, index) => step >= configuration.interventionPeriod! ? [{ step, index }] : [])
  const result = parsed.data
  if (post.length !== result.effect.values.length || result.sample.observations !== evidence.plotPeriods.length || result.sample.parameters !== evidence.donors.length) return false
  if (!post.every(({ step, index }, i) => {
    const point = result.effect.values[i]!
    return point.step === step && point.actual === evidence.observed[index] && point.counterfactual === evidence.synthetic[index]
      && point.lower === point.counterfactual && point.upper === point.counterfactual && point.effect === evidence.gaps[index]
  })) return false
  const cumulative = result.effect.values.reduce((sum, point) => sum + point.effect, 0)
  return result.effect.aggregate.cumulative === cumulative && result.effect.aggregate.average === cumulative / post.length
}
