import { z } from 'zod'
import { sourceFingerprint, type ColumnId, type DatasetProfile, type SourceFingerprint } from './dataset'
import { assertNever, err, isNonEmpty, ok, type NonEmptyArray, type Result } from './dop'

export interface PanelStructureEvidence {
  readonly kind: 'panel-structure'
  readonly sourceFingerprint: SourceFingerprint
  readonly unitColumn: ColumnId
  readonly timeColumn: ColumnId
  readonly observations: number
  readonly units: number
  readonly periods: number
  readonly duplicateKeys: number
  readonly missingUnitKeys: number
  readonly missingTimeKeys: number
  readonly balanced: boolean
}

/** Long-form panel values in source-row order. Values are outcome then treatment, column-major. */
export interface PanelLongMatrix {
  readonly kind: 'panel-long-matrix'
  readonly sourceFingerprint: SourceFingerprint
  readonly rowCount: number
  readonly units: NonEmptyArray<string>
  readonly times: NonEmptyArray<number>
  readonly timeLabels: NonEmptyArray<string>
  readonly values: Float64Array
}

export interface PanelInterventionLayout {
  readonly kind: 'panel-intervention-layout'
  readonly controls: NonEmptyArray<string>
  readonly treated: NonEmptyArray<string>
  readonly periods: NonEmptyArray<number>
  readonly periodLabels: NonEmptyArray<string>
  readonly prePeriods: number
  readonly postPeriods: number
  readonly adoptionPeriod: number
  readonly adoptionLabel: string
  readonly controlPreDifferenceSd: number
}

export type PanelInterventionLayoutProblem =
  | { readonly kind: 'treatment-not-binary'; readonly row: number; readonly value: number }
  | { readonly kind: 'no-treatment-variation' }
  | { readonly kind: 'no-pre-period' }
  | { readonly kind: 'no-post-period' }
  | { readonly kind: 'no-control-unit' }
  | { readonly kind: 'no-treated-unit' }
  | { readonly kind: 'non-simultaneous-adoption'; readonly unit: string; readonly period: number }
  | { readonly kind: 'duplicate-panel-cell'; readonly unit: string; readonly period: number }
  | { readonly kind: 'missing-panel-cell'; readonly unit: string; readonly period: number }
  | { readonly kind: 'degenerate-control-pre-period' }

export type PanelInterventionPreflight =
  | { readonly kind: 'not-applicable' }
  | { readonly kind: 'pending' }
  | { readonly kind: 'ready'; readonly layout: PanelInterventionLayout }
  | {
      readonly kind: 'refused'
      readonly problem:
        | { readonly kind: 'panel-data'; readonly problem: PanelDataProblem }
        | { readonly kind: 'panel-layout'; readonly problem: PanelInterventionLayoutProblem }
    }

export type PanelDataProblem =
  | { readonly kind: 'source-changed'; readonly expected: SourceFingerprint; readonly actual: SourceFingerprint }
  | { readonly kind: 'column-not-found'; readonly id: string }
  | { readonly kind: 'duplicate-column'; readonly id: string }
  | { readonly kind: 'non-numeric-column'; readonly name: string; readonly duckdbType: string }
  | { readonly kind: 'missing-key'; readonly name: string; readonly row: number }
  | { readonly kind: 'missing-value'; readonly name: string; readonly row: number }
  | { readonly kind: 'non-finite-value'; readonly name: string; readonly row: number }
  | { readonly kind: 'panel-data-failed'; readonly detail: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }

export type PanelBoundaryProblem = { readonly kind: 'invalid-panel-boundary'; readonly detail: string }

export function describePanelDataProblem(problem: PanelDataProblem): string {
  switch (problem.kind) {
    case 'source-changed': return 'The selected source changed after it was profiled. Import it again before estimating.'
    case 'column-not-found': return `The panel column ${problem.id} is no longer present in the source.`
    case 'duplicate-column': return `The panel column ${problem.id} was selected more than once.`
    case 'non-numeric-column': return `${problem.name} has type ${problem.duckdbType}; the outcome and treatment must be numeric.`
    case 'missing-key': return `${problem.name} has a missing panel key at row ${problem.row + 1}.`
    case 'missing-value': return `${problem.name} has a missing analysis value at row ${problem.row + 1}.`
    case 'non-finite-value': return `${problem.name} has a non-finite analysis value at row ${problem.row + 1}.`
    case 'panel-data-failed':
    case 'worker-unavailable':
    case 'worker-protocol-failed': return problem.detail
    default: return assertNever(problem)
  }
}

export function describePanelInterventionLayoutProblem(problem: PanelInterventionLayoutProblem): string {
  switch (problem.kind) {
    case 'treatment-not-binary': return `The treatment is ${problem.value} at row ${problem.row + 1}; panel intervention requires exactly 0 or 1.`
    case 'no-treatment-variation': return 'The treatment column must contain both untreated and treated observations.'
    case 'no-pre-period': return 'Treatment begins in the first period, leaving no pre-intervention observations.'
    case 'no-post-period': return 'The panel has no post-intervention period.'
    case 'no-control-unit': return 'Every panel unit is treated; at least one never-treated control unit is required.'
    case 'no-treated-unit': return 'No panel unit receives treatment.'
    case 'non-simultaneous-adoption': return `${problem.unit} does not follow the required simultaneous absorbing adoption pattern at period ${problem.period}.`
    case 'duplicate-panel-cell': return `${problem.unit} has more than one observation at period ${problem.period}.`
    case 'missing-panel-cell': return `${problem.unit} has no observation at period ${problem.period}.`
    case 'degenerate-control-pre-period': return 'The control pre-period does not contain enough non-constant first differences to fit synthetic DID weights.'
    default: return assertNever(problem)
  }
}

export function describePanelInterventionPreflight(preflight: Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>): string {
  switch (preflight.problem.kind) {
    case 'panel-data': return describePanelDataProblem(preflight.problem.problem)
    case 'panel-layout': return describePanelInterventionLayoutProblem(preflight.problem.problem)
    default: return assertNever(preflight.problem)
  }
}

/** Browser preflight equivalent of the structural checks in causal-core `panel_matrices`. */
export function assessPanelInterventionLayout(matrix: PanelLongMatrix): Result<PanelInterventionLayout, PanelInterventionLayoutProblem> {
  const units = [...new Set(matrix.units)].sort((left, right) => left.localeCompare(right))
  const periods = [...new Set(matrix.times)].sort((left, right) => left - right)
  const labels = new Map<number, string>()
  const cells = new Map<string, Map<number, { readonly outcome: number; readonly treatment: number }>>()
  let sawZero = false
  let sawOne = false

  for (let row = 0; row < matrix.rowCount; row += 1) {
    const unit = matrix.units[row]
    const period = matrix.times[row]
    const periodLabel = matrix.timeLabels[row]
    if (unit === undefined || period === undefined || periodLabel === undefined) return err({ kind: 'missing-panel-cell', unit: unit ?? 'unknown unit', period: period ?? -1 })
    const treatment = matrix.values[matrix.rowCount + row]
    const outcome = matrix.values[row]
    if (treatment === undefined || outcome === undefined) return err({ kind: 'missing-panel-cell', unit, period })
    if (treatment !== 0 && treatment !== 1) return err({ kind: 'treatment-not-binary', row, value: treatment })
    sawZero ||= treatment === 0
    sawOne ||= treatment === 1
    labels.set(period, periodLabel)
    const unitCells = cells.get(unit) ?? new Map<number, { readonly outcome: number; readonly treatment: number }>()
    if (unitCells.has(period)) return err({ kind: 'duplicate-panel-cell', unit, period })
    unitCells.set(period, { outcome, treatment })
    cells.set(unit, unitCells)
  }
  if (!sawZero || !sawOne) return err({ kind: 'no-treatment-variation' })

  for (const unit of units) {
    const unitCells = cells.get(unit)
    for (const period of periods) {
      if (unitCells?.has(period) !== true) return err({ kind: 'missing-panel-cell', unit, period })
    }
  }

  const firstTreated = periods.findIndex((period) => units.some((unit) => cells.get(unit)?.get(period)?.treatment === 1))
  if (firstTreated < 0) return err({ kind: 'no-treated-unit' })
  if (firstTreated === 0) return err({ kind: 'no-pre-period' })
  if (firstTreated >= periods.length) return err({ kind: 'no-post-period' })

  const controls = units.filter((unit) => periods.every((period) => cells.get(unit)?.get(period)?.treatment === 0))
  const treated = units.filter((unit) => periods.some((period) => cells.get(unit)?.get(period)?.treatment === 1))
  if (!isNonEmpty(controls)) return err({ kind: 'no-control-unit' })
  if (!isNonEmpty(treated)) return err({ kind: 'no-treated-unit' })

  for (const unit of treated) {
    for (const [periodIndex, period] of periods.entries()) {
      const expected = periodIndex < firstTreated ? 0 : 1
      if (cells.get(unit)?.get(period)?.treatment !== expected) return err({ kind: 'non-simultaneous-adoption', unit, period })
    }
  }

  const differences: number[] = []
  for (const unit of controls) {
    for (let periodIndex = 1; periodIndex < firstTreated; periodIndex += 1) {
      const currentPeriod = periods[periodIndex]
      const previousPeriod = periods[periodIndex - 1]
      if (currentPeriod === undefined || previousPeriod === undefined) return err({ kind: 'degenerate-control-pre-period' })
      const current = cells.get(unit)?.get(currentPeriod)?.outcome
      const previous = cells.get(unit)?.get(previousPeriod)?.outcome
      if (current === undefined || previous === undefined) return err({ kind: 'missing-panel-cell', unit, period: currentPeriod })
      differences.push(current - previous)
    }
  }
  if (differences.length < 2) return err({ kind: 'degenerate-control-pre-period' })
  const mean = differences.reduce((sum, value) => sum + value, 0) / differences.length
  const variance = differences.reduce((sum, value) => sum + (value - mean) ** 2, 0) / (differences.length - 1)
  const controlPreDifferenceSd = Math.sqrt(variance)
  if (!Number.isFinite(controlPreDifferenceSd) || controlPreDifferenceSd === 0) return err({ kind: 'degenerate-control-pre-period' })

  if (!isNonEmpty(periods)) return err({ kind: 'no-post-period' })
  const periodLabels = periods.map((period) => labels.get(period) ?? String(period))
  if (!isNonEmpty(periodLabels)) return err({ kind: 'no-post-period' })
  const adoptionPeriod = periods[firstTreated]
  const adoptionLabel = periodLabels[firstTreated]
  if (adoptionPeriod === undefined || adoptionLabel === undefined) return err({ kind: 'no-post-period' })
  return ok({
    kind: 'panel-intervention-layout',
    controls,
    treated,
    periods,
    periodLabels,
    prePeriods: firstTreated,
    postPeriods: periods.length - firstTreated,
    adoptionPeriod,
    adoptionLabel,
    controlPreDifferenceSd,
  })
}

const structureSchema = z.object({
  kind: z.literal('panel-structure'),
  sourceFingerprint: z.string(),
  unitColumn: z.string(),
  timeColumn: z.string(),
  observations: z.number().int().positive(),
  units: z.number().int().positive(),
  periods: z.number().int().positive(),
  duplicateKeys: z.number().int().nonnegative(),
  missingUnitKeys: z.number().int().nonnegative(),
  missingTimeKeys: z.number().int().nonnegative(),
  balanced: z.boolean(),
}).strict()

const longMatrixSchema = z.object({
  kind: z.literal('panel-long-matrix'),
  sourceFingerprint: z.string(),
  rowCount: z.number().int().positive(),
  units: z.array(z.string().min(1)).min(1),
  times: z.array(z.number().int().nonnegative()).min(1),
  timeLabels: z.array(z.string().min(1)).min(1),
  values: z.instanceof(Float64Array),
}).strict()

/** Rows are 0-based source indices; messages print them 1-based, as the preview table and the Rust façade do. */
export const panelDataProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() }).strict(),
  z.object({ kind: z.literal('column-not-found'), id: z.string() }).strict(),
  z.object({ kind: z.literal('duplicate-column'), id: z.string() }).strict(),
  z.object({ kind: z.literal('non-numeric-column'), name: z.string(), duckdbType: z.string() }).strict(),
  z.object({ kind: z.literal('missing-key'), name: z.string(), row: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('missing-value'), name: z.string(), row: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('non-finite-value'), name: z.string(), row: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('panel-data-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

export function parsePanelDataProblem(value: unknown): Result<PanelDataProblem, PanelBoundaryProblem> {
  const parsed = panelDataProblemSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-panel-boundary', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind !== 'source-changed') return ok(parsed.data)
  const expected = sourceFingerprint(parsed.data.expected)
  const actual = sourceFingerprint(parsed.data.actual)
  if (!expected.ok || !actual.ok) return err({ kind: 'invalid-panel-boundary', detail: 'A panel source-change response contains an invalid fingerprint.' })
  return ok({ kind: 'source-changed', expected: expected.value, actual: actual.value })
}

const bindColumn = (profile: DatasetProfile, raw: string): ColumnId | null =>
  profile.columns.find((column) => column.id === raw)?.id ?? null

export function parsePanelStructure(value: unknown, profile: DatasetProfile): Result<PanelStructureEvidence, PanelBoundaryProblem> {
  const parsed = structureSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-panel-boundary', detail: z.prettifyError(parsed.error) })
  if (parsed.data.sourceFingerprint !== profile.source.fingerprint || parsed.data.observations !== profile.rowCount) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The panel structure belongs to another source or row count.' })
  }
  const unitColumn = bindColumn(profile, parsed.data.unitColumn)
  const timeColumn = bindColumn(profile, parsed.data.timeColumn)
  if (unitColumn === null || timeColumn === null || unitColumn === timeColumn) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The panel structure has invalid unit or time columns.' })
  }
  const completeCells = parsed.data.units * parsed.data.periods
  const balanced = parsed.data.duplicateKeys === 0
    && parsed.data.missingUnitKeys === 0
    && parsed.data.missingTimeKeys === 0
    && parsed.data.observations === completeCells
  if (parsed.data.balanced !== balanced) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The panel balance flag disagrees with its key counts.' })
  }
  return ok({ ...parsed.data, sourceFingerprint: profile.source.fingerprint, unitColumn, timeColumn })
}

export function parsePanelLongMatrix(value: unknown, profile: DatasetProfile): Result<PanelLongMatrix, PanelBoundaryProblem> {
  const parsed = longMatrixSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-panel-boundary', detail: z.prettifyError(parsed.error) })
  if (parsed.data.sourceFingerprint !== profile.source.fingerprint || parsed.data.rowCount !== profile.rowCount) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long panel belongs to another source or row count.' })
  }
  if (parsed.data.units.length !== parsed.data.rowCount || parsed.data.times.length !== parsed.data.rowCount || parsed.data.timeLabels.length !== parsed.data.rowCount || parsed.data.values.length !== parsed.data.rowCount * 2) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long-panel buffers do not match the row count.' })
  }
  if (parsed.data.values.some((value) => !Number.isFinite(value))) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long-panel numeric buffer contains a non-finite value.' })
  }
  if (!isNonEmpty(parsed.data.units) || !isNonEmpty(parsed.data.times) || !isNonEmpty(parsed.data.timeLabels)) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long-panel buffers must contain at least one row.' })
  }
  return ok({
    kind: parsed.data.kind,
    sourceFingerprint: profile.source.fingerprint,
    rowCount: parsed.data.rowCount,
    units: parsed.data.units,
    times: parsed.data.times,
    timeLabels: parsed.data.timeLabels,
    values: parsed.data.values,
  })
}
