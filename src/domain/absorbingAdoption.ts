import type { PanelLongMatrix } from './panel'
import { err, ok, type Result } from './dop'

/** Shared assignment check. Balance is a property of a method, not adoption. */
export function absorbingAdoption(
  matrix: PanelLongMatrix,
  balance: 'required' | 'not-required',
): Result<ReadonlyMap<string, number | null>, string> {
  const rows = matrix.rowCount
  if (
    rows < 1 ||
    matrix.units.length !== rows ||
    matrix.periodCodes.length !== rows ||
    matrix.values.length < 2 * rows
  )
    return err('The panel values and keys are not aligned.')
  const byUnit = new Map<string, Map<number, number>>()
  for (let i = 0; i < rows; i++) {
    const unit = matrix.units[i],
      time = matrix.periodCodes[i],
      treatment = matrix.values[rows + i]
    if (unit === undefined || time === undefined)
      return err('Every row needs a unit and period key.')
    if (treatment !== 0 && treatment !== 1)
      return err('DiD requires a binary treatment indicator in each period.')
    const periods = byUnit.get(unit) ?? new Map<number, number>()
    if (periods.has(time)) return err('The panel contains a repeated unit-period key.')
    periods.set(time, treatment)
    byUnit.set(unit, periods)
  }
  const adoption = new Map<string, number | null>()
  for (const [unit, periods] of byUnit) {
    if (
      balance === 'required' &&
      (periods.size !== matrix.periods.length || matrix.periods.some((p) => !periods.has(p.code)))
    )
      return err('This DiD specification requires a balanced, complete panel.')
    let first: number | null = null
    for (const [time, treatment] of [...periods].sort(([a], [b]) => a - b)) {
      if (treatment === 1 && first === null) first = time
      if (treatment === 0 && first !== null)
        return err(
          `Treatment switches off for ${unit}. This method requires treatment to remain on after adoption.`,
        )
    }
    adoption.set(unit, first)
  }
  if ([...adoption.values()].every((g) => g === null))
    return err('No unit adopts treatment in the observed panel.')
  return ok(adoption)
}
