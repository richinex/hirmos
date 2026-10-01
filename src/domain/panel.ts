import { z } from 'zod'
import { sourceFingerprint, type ColumnId, type DatasetProfile, type NumericColumnSelection, type SourceFingerprint } from './dataset'
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
  /** Optional grouping metadata, read in the same query as the analysis values. */
  readonly cluster?: { readonly column: ColumnId; readonly labels: readonly string[] }
  readonly covariates?: readonly ColumnId[]
  readonly kind: 'panel-long-matrix'
  readonly sourceFingerprint: SourceFingerprint
  readonly rowCount: number
  readonly units: NonEmptyArray<string>
  /** Dense period code for each source row. */
  readonly periodCodes: NonEmptyArray<number>
  /** Unique source label for every dense period code. */
  readonly periods: NonEmptyArray<PanelPeriod>
  readonly values: Float64Array
}

/** Panel keys in source-row order, independent of whether any analysis value is missing. */
export interface PanelKeyMatrix {
  readonly kind: 'panel-key-matrix'
  readonly sourceFingerprint: SourceFingerprint
  readonly rowCount: number
  readonly units: NonEmptyArray<string>
  readonly periodCodes: NonEmptyArray<number>
  readonly periods: NonEmptyArray<PanelPeriod>
}

/** One panel period in its computational and source-facing representations. */
export interface PanelPeriod {
  /** Zero-based dense code used by the numerical kernel. */
  readonly code: number
  /** Original value from the selected time column, used in records and UI copy. */
  readonly label: string
}

/** Dense panel values ordered dataset-major, then period-major, with columns kept column-major. */
export interface JointPanelMatrix {
  readonly kind: 'joint-panel-matrix'
  readonly rows: number
  readonly datasets: number
  readonly periods: number
  readonly units: NonEmptyArray<string>
  readonly periodCatalog: NonEmptyArray<PanelPeriod>
  readonly columns: NonEmptyArray<NumericColumnSelection>
  readonly values: Float64Array
}

export type JointPanelMatrixProblem =
  | { readonly kind: 'row-count-mismatch'; readonly prepared: number; readonly panel: number }
  | { readonly kind: 'duplicate-panel-cell'; readonly unit: string; readonly period: PanelPeriod }
  | { readonly kind: 'missing-panel-cell'; readonly unit: string; readonly period: PanelPeriod }
  | { readonly kind: 'incomplete-panel-key'; readonly row: number }

export const describeJointPanelMatrixProblem = (problem: JointPanelMatrixProblem): string => {
  switch (problem.kind) {
    case 'row-count-mismatch': return `The prepared matrix has ${problem.prepared} rows but the panel keys describe ${problem.panel}. Recreate the prepared panel version.`
    case 'duplicate-panel-cell': return `${problem.unit} has more than one row at period ${problem.period.label}. J-PCMCI+ requires one.`
    case 'missing-panel-cell': return `${problem.unit} has no row at period ${problem.period.label}. J-PCMCI+ requires a balanced panel.`
    case 'incomplete-panel-key': return `Panel row ${problem.row + 1} has no complete unit-period key.`
    default: return assertNever(problem)
  }
}

export function orderJointPanelMatrix(
  prepared: {
    readonly rowCount: number
    readonly columns: NonEmptyArray<NumericColumnSelection>
    readonly values: Float64Array
  },
  panel: PanelKeyMatrix | PanelLongMatrix,
): Result<JointPanelMatrix, JointPanelMatrixProblem> {
  if (prepared.rowCount !== panel.rowCount) {
    return err({ kind: 'row-count-mismatch', prepared: prepared.rowCount, panel: panel.rowCount })
  }
  const units = [...new Set(panel.units)].sort((left, right) => left.localeCompare(right))
  const periods = [...panel.periods].sort((left, right) => left.code - right.code)
  if (!isNonEmpty(units) || !isNonEmpty(periods)) return err({ kind: 'incomplete-panel-key', row: 0 })

  const rows = new Map<string, Map<number, number>>()
  for (let row = 0; row < panel.rowCount; row += 1) {
    const unit = panel.units[row]
    const periodCode = panel.periodCodes[row]
    if (unit === undefined || periodCode === undefined) return err({ kind: 'incomplete-panel-key', row })
    const period = periods.find((candidate) => candidate.code === periodCode)
    if (period === undefined) return err({ kind: 'incomplete-panel-key', row })
    const unitRows = rows.get(unit) ?? new Map<number, number>()
    if (unitRows.has(periodCode)) return err({ kind: 'duplicate-panel-cell', unit, period })
    unitRows.set(periodCode, row)
    rows.set(unit, unitRows)
  }

  const orderedRows: number[] = []
  for (const unit of units) {
    for (const period of periods) {
      const row = rows.get(unit)?.get(period.code)
      if (row === undefined) return err({ kind: 'missing-panel-cell', unit, period })
      orderedRows.push(row)
    }
  }
  const ordered = new Float64Array(prepared.values.length)
  for (let column = 0; column < prepared.columns.length; column += 1) {
    for (const [targetRow, sourceRow] of orderedRows.entries()) {
      const value = prepared.values[column * prepared.rowCount + sourceRow]
      if (value === undefined) return err({ kind: 'incomplete-panel-key', row: sourceRow })
      ordered[column * orderedRows.length + targetRow] = value
    }
  }
  return ok({
    kind: 'joint-panel-matrix',
    rows: orderedRows.length,
    datasets: units.length,
    periods: periods.length,
    units,
    periodCatalog: periods,
    columns: prepared.columns,
    values: ordered,
  })
}

export interface PanelInterventionLayout {
  readonly kind: 'panel-intervention-layout'
  readonly controls: NonEmptyArray<string>
  readonly treated: NonEmptyArray<string>
  readonly periods: NonEmptyArray<PanelPeriod>
  readonly prePeriods: number
  readonly postPeriods: number
  readonly adoption: PanelPeriod
  readonly controlPreDifferenceSd: number | null
}

export type PanelInterventionLayoutProblem =
  | { readonly kind: 'treatment-not-binary'; readonly row: number; readonly value: number }
  | { readonly kind: 'no-treatment-variation' }
  | { readonly kind: 'no-pre-period' }
  | { readonly kind: 'no-post-period' }
  | { readonly kind: 'no-control-unit' }
  | { readonly kind: 'no-treated-unit' }
  | { readonly kind: 'incomplete-panel-row'; readonly row: number }
  | { readonly kind: 'inconsistent-period-label'; readonly code: number; readonly first: string; readonly next: string }
  | { readonly kind: 'missing-period-label'; readonly code: number }
  | { readonly kind: 'non-simultaneous-adoption'; readonly unit: string; readonly period: PanelPeriod }
  | { readonly kind: 'duplicate-panel-cell'; readonly unit: string; readonly period: PanelPeriod }
  | { readonly kind: 'missing-panel-cell'; readonly unit: string; readonly period: PanelPeriod }
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
    case 'incomplete-panel-row': return `The panel matrix is incomplete at row ${problem.row + 1}.`
    case 'inconsistent-period-label': return `Panel period code ${problem.code} is associated with both ${problem.first} and ${problem.next}.`
    case 'missing-period-label': return `Panel period code ${problem.code} has no source value.`
    case 'non-simultaneous-adoption': return `${problem.unit} does not follow the required simultaneous absorbing adoption pattern at period ${problem.period.label}.`
    case 'duplicate-panel-cell': return `${problem.unit} has more than one observation at period ${problem.period.label}.`
    case 'missing-panel-cell': return `${problem.unit} has no observation at period ${problem.period.label}.`
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
  const labels = new Map<number, string>()
  for (const period of matrix.periods) {
    const recorded = labels.get(period.code)
    if (recorded !== undefined && recorded !== period.label) {
      return err({ kind: 'inconsistent-period-label', code: period.code, first: recorded, next: period.label })
    }
    labels.set(period.code, period.label)
  }
  const periodCodes = [...new Set(matrix.periodCodes)].sort((left, right) => left - right)
  const cells = new Map<string, Map<number, { readonly outcome: number; readonly treatment: number }>>()
  let sawZero = false
  let sawOne = false

  for (let row = 0; row < matrix.rowCount; row += 1) {
    const unit = matrix.units[row]
    const periodCode = matrix.periodCodes[row]
    if (unit === undefined || periodCode === undefined) return err({ kind: 'incomplete-panel-row', row })
    const periodLabel = labels.get(periodCode)
    if (periodLabel === undefined) return err({ kind: 'missing-period-label', code: periodCode })
    const treatment = matrix.values[matrix.rowCount + row]
    const outcome = matrix.values[row]
    if (treatment === undefined || outcome === undefined) return err({ kind: 'incomplete-panel-row', row })
    if (treatment !== 0 && treatment !== 1) return err({ kind: 'treatment-not-binary', row, value: treatment })
    sawZero ||= treatment === 0
    sawOne ||= treatment === 1
    const unitCells = cells.get(unit) ?? new Map<number, { readonly outcome: number; readonly treatment: number }>()
    if (unitCells.has(periodCode)) return err({ kind: 'duplicate-panel-cell', unit, period: { code: periodCode, label: periodLabel } })
    unitCells.set(periodCode, { outcome, treatment })
    cells.set(unit, unitCells)
  }
  if (!sawZero || !sawOne) return err({ kind: 'no-treatment-variation' })

  const periods: PanelPeriod[] = []
  for (const code of periodCodes) {
    const label = labels.get(code)
    if (label === undefined) return err({ kind: 'missing-period-label', code })
    periods.push({ code, label })
  }
  if (!isNonEmpty(periods)) return err({ kind: 'no-post-period' })

  for (const unit of units) {
    const unitCells = cells.get(unit)
    for (const period of periods) {
      if (unitCells?.has(period.code) !== true) return err({ kind: 'missing-panel-cell', unit, period })
    }
  }

  const firstTreated = periods.findIndex((period) => units.some((unit) => cells.get(unit)?.get(period.code)?.treatment === 1))
  if (firstTreated < 0) return err({ kind: 'no-treated-unit' })
  if (firstTreated === 0) return err({ kind: 'no-pre-period' })
  if (firstTreated >= periods.length) return err({ kind: 'no-post-period' })

  const controls = units.filter((unit) => periods.every((period) => cells.get(unit)?.get(period.code)?.treatment === 0))
  const treated = units.filter((unit) => periods.some((period) => cells.get(unit)?.get(period.code)?.treatment === 1))
  if (!isNonEmpty(controls)) return err({ kind: 'no-control-unit' })
  if (!isNonEmpty(treated)) return err({ kind: 'no-treated-unit' })

  for (const unit of treated) {
    for (const [periodIndex, period] of periods.entries()) {
      const expected = periodIndex < firstTreated ? 0 : 1
      if (cells.get(unit)?.get(period.code)?.treatment !== expected) return err({ kind: 'non-simultaneous-adoption', unit, period })
    }
  }

  const differences: number[] = []
  for (const unit of controls) {
    for (let periodIndex = 1; periodIndex < firstTreated; periodIndex += 1) {
      const currentPeriod = periods[periodIndex]
      const previousPeriod = periods[periodIndex - 1]
      if (currentPeriod === undefined || previousPeriod === undefined) return err({ kind: 'degenerate-control-pre-period' })
      const current = cells.get(unit)?.get(currentPeriod.code)?.outcome
      const previous = cells.get(unit)?.get(previousPeriod.code)?.outcome
      if (current === undefined || previous === undefined) return err({ kind: 'missing-panel-cell', unit, period: currentPeriod })
      differences.push(current - previous)
    }
  }
  const mean = differences.length === 0 ? 0 : differences.reduce((sum, value) => sum + value, 0) / differences.length
  const variance = differences.length < 2 ? 0 : differences.reduce((sum, value) => sum + (value - mean) ** 2, 0) / (differences.length - 1)
  const controlPreDifferenceSd = variance > 0 && Number.isFinite(variance) ? Math.sqrt(variance) : null

  const adoption = periods[firstTreated]
  if (adoption === undefined) return err({ kind: 'no-post-period' })
  return ok({
    kind: 'panel-intervention-layout',
    controls,
    treated,
    periods,
    prePeriods: firstTreated,
    postPeriods: periods.length - firstTreated,
    adoption,
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
  cluster: z.object({ column: z.string().min(1), labels: z.array(z.string().min(1)) }).strict().optional(),
  covariates: z.array(z.string()).default([]),
  kind: z.literal('panel-long-matrix'),
  sourceFingerprint: z.string(),
  rowCount: z.number().int().positive(),
  units: z.array(z.string().min(1)).min(1),
  periodCodes: z.array(z.number().int().nonnegative()).min(1),
  periods: z.array(z.object({ code: z.number().int().nonnegative(), label: z.string().min(1) }).strict()).min(1),
  values: z.instanceof(Float64Array),
}).strict()

const keyMatrixSchema = longMatrixSchema.omit({ values: true, covariates: true, cluster: true }).extend({ kind: z.literal('panel-key-matrix') }).strict()

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
  const clusterColumn = parsed.data.cluster === undefined ? null : bindColumn(profile, parsed.data.cluster.column)
  if (parsed.data.cluster !== undefined && (clusterColumn === null || parsed.data.cluster.labels.length !== parsed.data.rowCount)) {
    return err({ kind: 'invalid-panel-boundary', detail: 'Panel cluster labels must match the source columns and row count.' })
  }
  if (parsed.data.sourceFingerprint !== profile.source.fingerprint || parsed.data.rowCount !== profile.rowCount) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long panel belongs to another source or row count.' })
  }
  if (parsed.data.units.length !== parsed.data.rowCount || parsed.data.periodCodes.length !== parsed.data.rowCount || parsed.data.values.length !== parsed.data.rowCount * (2 + parsed.data.covariates.length)) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long-panel buffers do not match the row count.' })
  }
  if (parsed.data.values.some((value) => !Number.isFinite(value))) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long-panel numeric buffer contains a non-finite value.' })
  }
  if (!isNonEmpty(parsed.data.units) || !isNonEmpty(parsed.data.periodCodes) || !isNonEmpty(parsed.data.periods)) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long-panel buffers must contain at least one row.' })
  }
  const covariates: ColumnId[] = []
  for (const raw of parsed.data.covariates) {
    const column = bindColumn(profile, raw)
    if (column === null || covariates.includes(column)) return err({ kind: 'invalid-panel-boundary', detail: 'Panel covariates must be distinct columns in this source.' })
    covariates.push(column)
  }
  const units = parsed.data.units
  const periodCodes = parsed.data.periodCodes
  const periods = parsed.data.periods
  const labels = new Map<number, string>()
  for (const period of parsed.data.periods) {
    if (labels.has(period.code)) return err({ kind: 'invalid-panel-boundary', detail: `The long panel repeats period code ${period.code}.` })
    labels.set(period.code, period.label)
  }
  if (parsed.data.periodCodes.some((code) => !labels.has(code)) || labels.size !== new Set(parsed.data.periodCodes).size) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The long-panel period catalog does not match its row codes.' })
  }
  return ok({
    kind: parsed.data.kind,
    sourceFingerprint: profile.source.fingerprint,
    rowCount: parsed.data.rowCount,
    units,
    periodCodes,
    periods,
    values: parsed.data.values,
    covariates,
    ...(parsed.data.cluster !== undefined && clusterColumn !== null ? { cluster: { column: clusterColumn, labels: parsed.data.cluster.labels } } : {}),
  })
}

export function parsePanelKeyMatrix(value: unknown, profile: DatasetProfile): Result<PanelKeyMatrix, PanelBoundaryProblem> {
  const parsed = keyMatrixSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-panel-boundary', detail: z.prettifyError(parsed.error) })
  if (parsed.data.sourceFingerprint !== profile.source.fingerprint || parsed.data.rowCount !== profile.rowCount) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The panel keys belong to another source or row count.' })
  }
  if (parsed.data.units.length !== parsed.data.rowCount || parsed.data.periodCodes.length !== parsed.data.rowCount) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The panel-key buffers do not match the row count.' })
  }
  if (!isNonEmpty(parsed.data.units) || !isNonEmpty(parsed.data.periodCodes) || !isNonEmpty(parsed.data.periods)) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The panel-key buffers must contain at least one row.' })
  }
  const units = parsed.data.units
  const periodCodes = parsed.data.periodCodes
  const periods = parsed.data.periods
  const labels = new Map<number, string>()
  for (const period of parsed.data.periods) {
    if (labels.has(period.code)) return err({ kind: 'invalid-panel-boundary', detail: `The panel keys repeat period code ${period.code}.` })
    labels.set(period.code, period.label)
  }
  if (parsed.data.periodCodes.some((code) => !labels.has(code)) || labels.size !== new Set(parsed.data.periodCodes).size) {
    return err({ kind: 'invalid-panel-boundary', detail: 'The panel-key period catalog does not match its row codes.' })
  }
  return ok({ kind: 'panel-key-matrix', sourceFingerprint: profile.source.fingerprint, rowCount: parsed.data.rowCount, units, periodCodes, periods })
}
