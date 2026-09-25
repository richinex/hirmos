import type { ColumnId } from './dataset'
import { err, ok, type Result } from './dop'
import { designLayoutOf, type EstimatorId } from './estimation'
import type { CovariateEncoding, DesignLayout, StudyVariable } from './study'

/** A continuous column declared categorical would expand without bound, so the count is capped. */
export const MAX_DESIGN_LEVELS = 64

export type DesignExpansionProblem =
  | { readonly kind: 'single-level'; readonly column: number }
  | { readonly kind: 'too-many-levels'; readonly column: number; readonly levels: number }

export interface ExpandedDesign {
  readonly values: Float64Array
  readonly columnCount: number
  /** The columns each source column became, in source order. */
  readonly expanded: readonly (readonly number[])[]
  /** The levels each expanded column stands for, aligned with `expanded`. */
  readonly levels: readonly (readonly number[])[]
}

const distinctAscending = (values: Float64Array, rowCount: number, column: number): number[] => {
  const seen = new Set<number>()
  const start = column * rowCount
  for (let row = 0; row < rowCount; row += 1) seen.add(values[start + row])
  return [...seen].sort((left, right) => left - right)
}

/**
 * Expands every column the layouts mark categorical, keeping source order. Levels ascend, and a
 * treatment contrast drops the first, which is what patsy's `C(x)` and `pd.get_dummies` do.
 */
export const expandDesign = (
  values: Float64Array,
  rowCount: number,
  columnCount: number,
  layouts: readonly DesignLayout[],
): Result<ExpandedDesign, DesignExpansionProblem> => {
  const levels: number[][] = []
  const widths: number[] = []
  for (let column = 0; column < columnCount; column += 1) {
    const layout = layouts[column] ?? { kind: 'numeric' }
    if (layout.kind === 'numeric') {
      levels.push([])
      widths.push(1)
      continue
    }
    const distinct = distinctAscending(values, rowCount, column)
    if (distinct.length < 2) return err({ kind: 'single-level', column })
    if (distinct.length > MAX_DESIGN_LEVELS) {
      return err({ kind: 'too-many-levels', column, levels: distinct.length })
    }
    const kept = layout.kind === 'treatment-contrast' ? distinct.slice(1) : distinct
    levels.push(kept)
    widths.push(kept.length)
  }

  const total = widths.reduce((sum, width) => sum + width, 0)
  const expandedValues = new Float64Array(total * rowCount)
  const expanded: number[][] = []
  let next = 0
  for (let column = 0; column < columnCount; column += 1) {
    const source = column * rowCount
    if (levels[column].length === 0) {
      expandedValues.set(values.subarray(source, source + rowCount), next * rowCount)
      expanded.push([next])
      next += 1
      continue
    }
    const placed: number[] = []
    for (const level of levels[column]) {
      const target = next * rowCount
      for (let row = 0; row < rowCount; row += 1) {
        expandedValues[target + row] = values[source + row] === level ? 1 : 0
      }
      placed.push(next)
      next += 1
    }
    expanded.push(placed)
  }
  return ok({ values: expandedValues, columnCount: total, expanded, levels })
}

export const describeDesignExpansionProblem = (
  problem: DesignExpansionProblem,
  names: readonly string[],
): string => {
  const name = names[problem.column] ?? 'The covariate'
  switch (problem.kind) {
    case 'single-level':
      return `${name} holds one value, so it has no levels to make columns from.`
    case 'too-many-levels':
      return `${name} holds ${problem.levels} distinct values. The limit for a categorical covariate is ${MAX_DESIGN_LEVELS}.`
  }
}

/**
 * One layout per materialised column. Takes the same array that was materialised so the layouts
 * cannot drift from the columns they describe. The first `fixed` columns carry the treatment and
 * outcome, which are always numeric.
 */
export const designLayouts = (
  columns: readonly StudyVariable[],
  fixed: number,
  estimator: EstimatorId,
  encodings: Readonly<Record<ColumnId, CovariateEncoding>>,
): readonly DesignLayout[] =>
  columns.map((column, index) => index < fixed
    ? { kind: 'numeric' }
    : designLayoutOf(estimator, encodings[column.column] ?? { kind: 'numeric' }))

/** Estimators whose run expands a declared encoding. The control appears only where it applies. */
export const expandsDesign = (estimator: EstimatorId): boolean =>
  estimator === 'backdoor-linear-regression'
  || estimator === 'bayesian-gaussian'
  || estimator === 'poisson-glm'
  || estimator === 'negative-binomial-p'
  || estimator === 'dml-plr'
  || estimator === 'dml-irm'
  || estimator === 't-learner'
  || estimator === 'propensity-weighting'
  || estimator === 'propensity-matching'
  || estimator === 'doubly-robust'
  || estimator === 'continuous-gps'
