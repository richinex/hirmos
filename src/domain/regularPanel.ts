import { z } from 'zod'
import { assertNever, err, ok, type Result } from './dop'
import { orderJointPanelMatrix, type PanelKeyMatrix, type JointPanelMatrixProblem } from './panel'
import type { PreparedMatrix } from '@/data/prepared'
import type { CalendarReport, CalendarRequest } from './calendar'
import type { Frequency } from './preprocessing'
import type { CountRegressionRequest } from './countRegression'

type Schedule = CalendarRequest['schedule']
export type PanelClock =
  | { readonly kind: 'ordinal' }
  | { readonly kind: 'calendar'; readonly schedule: Schedule; readonly report: CalendarReport }

export type PanelProblem =
  | { readonly kind: 'column-missing' }
  | { readonly kind: 'source-mismatch' }
  | { readonly kind: 'read-failed'; readonly detail: string }
  | { readonly kind: 'cancelled' }
  | { readonly kind: 'structure'; readonly problem: JointPanelMatrixProblem }
  | { readonly kind: 'ordinal-gap' }
  | { readonly kind: 'calendar-unavailable' }
  | { readonly kind: 'calendar-gaps'; readonly missing: number }
  | { readonly kind: 'calendar-alignment' }
  | { readonly kind: 'design-mismatch' }
  | { readonly kind: 'predictor-missing' }
  | { readonly kind: 'onset-missing' }
  | { readonly kind: 'cohort-missing' }
  | { readonly kind: 'onset-invalid'; readonly row: number }
  | { readonly kind: 'onset-reversed'; readonly row: number }

const validated: unique symbol = Symbol('RegularPanel')
export type RegularPanel = {
  readonly [validated]: true
  readonly matrix: PreparedMatrix
  readonly keys: readonly (readonly [number, number])[]
  readonly periods: readonly string[]
  readonly clock: PanelClock
}

/** Candidate alignments are checked by Jiff, never inferred from a partial preview. */
export function calendarSchedules(frequency: Frequency): readonly Schedule[] {
  switch (frequency) {
    case 'daily':
      return ['daily']
    case 'weekly':
      return ['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday']
    case 'monthly':
      return ['month-start', 'month-end']
    case 'quarterly':
      return ['quarter-start', 'quarter-end']
    case 'yearly':
      return ['year-start', 'year-end']
    default:
      return assertNever(frequency)
  }
}

/** The only constructor. Dense period codes alone do not establish regularity. */
export function regularPanel(
  raw: PanelKeyMatrix,
  matrix: PreparedMatrix,
  clock: PanelClock,
): Result<RegularPanel, PanelProblem> {
  const structure = orderJointPanelMatrix(matrix, raw)
  if (!structure.ok) return err({ kind: 'structure', problem: structure.error })
  switch (clock.kind) {
    case 'ordinal': {
      const axis = z
        .array(z.coerce.number().int().safe())
        .min(1)
        .safeParse(raw.periods.map((period) => period.label))
      if (
        !axis.success ||
        axis.data.some((value, index) => index > 0 && value - axis.data[index - 1]! !== 1)
      )
        return err({ kind: 'ordinal-gap' })
      break
    }
    case 'calendar':
      if (clock.report.issues.length > 0) return err({ kind: 'calendar-alignment' })
      if (clock.report.missing > 0)
        return err({ kind: 'calendar-gaps', missing: clock.report.missing })
      if (clock.report.units !== structure.value.datasets)
        return err({ kind: 'calendar-unavailable' })
      break
    default:
      return assertNever(clock)
  }
  const units = new Map(structure.value.units.map((unit, index) => [unit, index]))
  const keys: [number, number][] = []
  for (let row = 0; row < raw.rowCount; row++) {
    const unit = units.get(raw.units[row]!)
    const period = raw.periodCodes[row]
    if (unit === undefined || period === undefined)
      return err({ kind: 'structure', problem: { kind: 'incomplete-panel-key', row } })
    keys.push([unit, period])
  }
  return ok({
    [validated]: true,
    matrix,
    keys,
    periods: raw.periods.map((period) => period.label),
    clock,
  })
}

type Design = CountRegressionRequest['design']
export function lagDesign(
  panel: RegularPanel,
  terms: Extract<Design, { kind: 'lags' }>['terms'],
  sum: number[],
): Extract<Design, { kind: 'lags' }> {
  return { kind: 'lags', keys: panel.keys.map(([u, t]) => [u, t]), terms, sum }
}

export function cohortAdoption(
  panel: RegularPanel,
  column: number,
): Result<[number, number | null][], PanelProblem> {
  const ordered = panel.keys
    .map(([unit, period], row) => ({ unit, period, row }))
    .sort((a, b) => a.unit - b.unit || a.period - b.period)
  const adoption = new Map<number, number | null>()
  for (const { unit, period, row } of ordered) {
    const value = panel.matrix.values[column * panel.matrix.rowCount + row]
    if (value !== 0 && value !== 1) return err({ kind: 'onset-invalid', row })
    if (!adoption.has(unit)) adoption.set(unit, null)
    if (value === 1 && adoption.get(unit) === null) adoption.set(unit, period)
    if (value === 0 && adoption.get(unit) !== null) return err({ kind: 'onset-reversed', row })
  }
  return ok([...adoption])
}

export function describePanelProblem(problem: PanelProblem): string {
  switch (problem.kind) {
    case 'column-missing':
      return 'The selected panel key column is no longer in the source profile.'
    case 'source-mismatch':
      return 'The panel keys belong to a different source.'
    case 'read-failed':
      return problem.detail
    case 'cancelled':
      return 'The analysis was cancelled.'
    case 'ordinal-gap':
      return 'Numeric panel periods must be consecutive integers. Missing periods cannot be compressed into adjacent lags.'
    case 'calendar-unavailable':
      return 'Calendar regularity could not be checked for this panel.'
    case 'calendar-gaps':
      return `The panel has ${problem.missing} missing calendar periods. Resolve the calendar grid before constructing lags or event periods.`
    case 'calendar-alignment':
      return 'The dates do not align with the selected calendar frequency. Use consistent week dates or calendar period starts or ends.'
    case 'design-mismatch':
      return 'The regression design does not match the prepared data structure.'
    case 'predictor-missing':
      return 'Choose a predictor from the prepared model columns.'
    case 'onset-missing':
      return 'Choose a treatment indicator from the prepared model columns.'
    case 'cohort-missing':
      return 'Choose an observed adoption period with a pre-treatment baseline.'
    case 'onset-invalid':
      return `The treatment indicator at row ${problem.row + 1} must be 0 or 1.`
    case 'onset-reversed':
      return `The treatment indicator returns to 0 at row ${problem.row + 1}. It must remain 1 after onset.`
    case 'structure': {
      const issue = problem.problem
      switch (issue.kind) {
        case 'row-count-mismatch':
          return `The prepared values have ${issue.prepared} rows but the panel keys have ${issue.panel}. Recreate the prepared panel without deleting individual rows.`
        case 'duplicate-panel-cell':
          return `${issue.unit} has more than one row at ${issue.period.label}.`
        case 'missing-panel-cell':
          return `${issue.unit} has no row at ${issue.period.label}. This regression requires a balanced panel.`
        case 'incomplete-panel-key':
          return `Panel row ${issue.row + 1} has no complete unit-period key.`
        default:
          return assertNever(issue)
      }
    }
    default:
      return assertNever(problem)
  }
}
