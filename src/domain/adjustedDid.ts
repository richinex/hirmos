import { z } from 'zod'
import type { ColumnId } from './dataset'
import type { PanelInterventionLayout, PanelLongMatrix } from './panel'
import { assertNever } from './dop'

export type DidCovariateRole =
  'treatment-indicator' | 'group-indicator' | 'baseline-group-indicator' | 'cohort-indicator'
export interface DidCovariateRestriction {
  readonly column: ColumnId
  readonly role: DidCovariateRole
}

/** The covariate's values for every row of the long matrix; covariates follow the outcome and treatment. */
const covariateValues = (matrix: PanelLongMatrix, index: number): Float64Array =>
  matrix.values.subarray((index + 2) * matrix.rowCount, (index + 3) * matrix.rowCount)

const duplicatesTreatment = (matrix: PanelLongMatrix, values: Float64Array): boolean =>
  values.every((value, row) => value === matrix.values[matrix.rowCount + row])

/**
 * True when the column takes one value on every row of one side and a different value on every row of
 * the other: no overlap is left between the two. `side` returns undefined for units in neither, and
 * `period`, when given, limits the test to that period's rows.
 */
function separates(
  matrix: PanelLongMatrix,
  values: Float64Array,
  side: (unit: string) => boolean | undefined,
  period?: number,
): boolean {
  const levels = new Map<boolean, number>()
  for (let row = 0; row < matrix.rowCount; row++) {
    if (period !== undefined && matrix.periodCodes[row] !== period) continue
    const group = side(matrix.units[row]!)
    if (group === undefined) continue
    const value = values[row]
    if (value === undefined || !Number.isFinite(value)) return false
    if (levels.has(group) && levels.get(group) !== value) return false
    levels.set(group, value)
  }
  return levels.size === 2 && levels.get(false) !== levels.get(true)
}

/** Exact identities, not correlation thresholds or column-name heuristics. */
export function didCovariateRestrictions(
  matrix: PanelLongMatrix,
  layout: PanelInterventionLayout,
  specification: AdjustedDidSpecification,
): readonly DidCovariateRestriction[] {
  const treated = new Set(layout.treated)
  const side = (unit: string) => treated.has(unit)
  return (matrix.covariates ?? []).flatMap((column, index): DidCovariateRestriction[] => {
    const values = covariateValues(matrix, index)
    if (values.length !== matrix.rowCount) return []
    if (duplicatesTreatment(matrix, values)) return [{ column, role: 'treatment-indicator' }]
    if (separates(matrix, values, side)) return [{ column, role: 'group-indicator' }]
    if (
      specification.kind === 'doublyRobust' &&
      separates(matrix, values, side, layout.periods[0].code)
    )
      return [{ column, role: 'baseline-group-indicator' }]
    return []
  })
}

/**
 * Staggered adoption compares each cohort with the never-treated units, so a column that separates any
 * cohort from them exactly, such as the first-treatment period itself, leaves that comparison no overlap.
 */
export function staggeredCovariateRestrictions(
  matrix: PanelLongMatrix,
): readonly DidCovariateRestriction[] {
  const adoption = new Map<string, number | null>()
  for (let row = 0; row < matrix.rowCount; row++) {
    const unit = matrix.units[row]!
    const treated = matrix.values[matrix.rowCount + row] === 1
    const known = adoption.get(unit)
    if (treated && (known === undefined || known === null || matrix.periodCodes[row]! < known))
      adoption.set(unit, matrix.periodCodes[row]!)
    else if (known === undefined) adoption.set(unit, null)
  }
  const cohorts = [...new Set([...adoption.values()].filter((g): g is number => g !== null))]
  return (matrix.covariates ?? []).flatMap((column, index): DidCovariateRestriction[] => {
    const values = covariateValues(matrix, index)
    if (values.length !== matrix.rowCount) return []
    if (duplicatesTreatment(matrix, values)) return [{ column, role: 'treatment-indicator' }]
    const separatesCohort = cohorts.some((cohort) =>
      separates(matrix, values, (unit) => {
        const g = adoption.get(unit)
        return g === cohort ? true : g === null ? false : undefined
      }),
    )
    return separatesCohort ? [{ column, role: 'cohort-indicator' }] : []
  })
}

/** The run's refusal: each restricted column by name, with its reason. */
export function describeDidCovariateRestrictions(
  restrictions: readonly DidCovariateRestriction[],
  nameOf: (column: ColumnId) => string,
): string {
  return restrictions
    .map((item) => `${nameOf(item.column)}: ${describeDidCovariateRole(item.role)}`)
    .join(' ')
}

export function describeDidCovariateRole(role: DidCovariateRole): string {
  switch (role) {
    case 'treatment-indicator':
      return 'Duplicates the treatment indicator; it cannot also be an adjustment covariate.'
    case 'group-indicator':
      return 'Identifies the treated and control groups exactly; it is not an adjustment covariate.'
    case 'baseline-group-indicator':
      return 'Baseline values identify the treated and control groups exactly, leaving no treatment overlap.'
    case 'cohort-indicator':
      return 'Identifies an adoption cohort exactly against the never-treated units; it is not an adjustment covariate.'
    default:
      return assertNever(role)
  }
}

export const adjustedDidSpecificationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('regression') }).strict(),
  z
    .object({
      kind: z.literal('doublyRobust'),
      folds: z.number().int().min(2),
      seed: z.number().int().min(0).max(0xffffffff),
      trimming: z.number().finite().gt(0).lt(0.5),
      normalization: z.enum(['in-sample', 'population']),
    })
    .strict(),
])
export type AdjustedDidSpecification = z.infer<typeof adjustedDidSpecificationSchema>
export const didPropensityFitSchema = z.literal('standardized-logistic-v1')
export const adjustedDidConfigurationSchema = z
  .object({
    kind: z.literal('panel-intervention'),
    primary: z.literal('adjusted'),
    covariates: z
      .array(z.string().min(1))
      .refine((values) => new Set(values).size === values.length),
    specification: adjustedDidSpecificationSchema,
  })
  .strict()
const interval = z.tuple([z.number().finite(), z.number().finite()]).refine(([lo, hi]) => lo <= hi)
export const adjustedDidEvidenceSchema = z
  .object({
    kind: z.literal('panelAdjusted'),
    observations: z.number().int().positive(),
    units: z.array(z.string().min(1)).min(2),
    times: z.array(z.number().int()).length(2),
    controlUnits: z.number().int().positive(),
    treatedUnits: z.number().int().positive(),
    nPre: z.literal(1),
    nPost: z.literal(1),
    covariates: z.number().int().nonnegative(),
    specification: adjustedDidSpecificationSchema,
    estimate: z.number().finite(),
    standardError: z.number().finite().nonnegative(),
    interval,
    groupMeans: z.tuple([
      z.tuple([z.number().finite(), z.number().finite()]),
      z.tuple([z.number().finite(), z.number().finite()]),
    ]),
    inference: z.discriminatedUnion('kind', [
      z
        .object({
          kind: z.literal('independentErrors'),
          degreesOfFreedom: z.number().int().positive(),
          coefficients: z.array(z.number().finite()).min(4),
          standardErrors: z.array(z.number().finite().nonnegative()).min(4),
          intervals: z.array(interval).min(4),
        })
        .strict(),
      z
        .object({
          kind: z.literal('crossFitted'),
          propensityFit: didPropensityFitSchema.optional(),
          propensity: z.array(z.number().finite().gt(0).lt(1)).min(2),
          optimizerStatus: z
            .array(
              z.enum([
                'projected-gradient',
                'function-tolerance',
                'iteration-limit',
                'line-search-failed',
              ]),
            )
            .min(2),
        })
        .strict(),
    ]),
  })
  .strict()
  .superRefine((e, ctx) => {
    const issue = (message: string) => ctx.addIssue({ code: 'custom', message })
    if (
      e.observations !== 2 * e.units.length ||
      e.units.length !== e.controlUnits + e.treatedUnits ||
      new Set(e.units).size !== e.units.length ||
      e.times[0]! >= e.times[1]!
    )
      issue('DiD requires distinct units with two ordered observations each.')
    switch (e.specification.kind) {
      case 'regression': {
        const i = e.inference
        if (i.kind !== 'independentErrors') {
          issue('Regression DiD requires classical independent-error inference.')
          break
        }
        const p = 4 + e.covariates
        if (
          i.coefficients.length !== p ||
          i.standardErrors.length !== p ||
          i.intervals.length !== p ||
          i.degreesOfFreedom !== e.observations - p
        )
          issue('Regression DiD inference dimensions disagree with its specification.')
        if (
          e.estimate !== i.coefficients[3] ||
          e.standardError !== i.standardErrors[3] ||
          e.interval[0] !== i.intervals[3]?.[0] ||
          e.interval[1] !== i.intervals[3]?.[1]
        )
          issue('The DiD headline must be the group-by-post interaction.')
        break
      }
      case 'doublyRobust': {
        const i = e.inference
        if (i.kind !== 'crossFitted') {
          issue('DR DiD requires cross-fitted inference.')
          break
        }
        // Missing fit metadata describes a historical record, never a solver fallback.
        if (
          i.propensityFit !== undefined &&
          i.optimizerStatus.some((s) => s === 'iteration-limit' || s === 'line-search-failed')
        )
          issue('A standardized propensity fit must converge in every fold.')
        if (
          e.covariates < 1 ||
          i.propensity.length !== e.units.length ||
          i.optimizerStatus.length !== e.specification.folds ||
          e.specification.folds > Math.min(e.controlUnits, e.treatedUnits)
        )
          issue('DR DiD folds, covariates or propensity dimensions disagree.')
        break
      }
    }
  })
export type AdjustedDidEvidence = z.infer<typeof adjustedDidEvidenceSchema>
const savedColumn = z.object({ column: z.string() })
const savedTarget = z.object({
  kind: z.literal('average-treatment-effect-on-treated'),
  scale: z.literal('additive'),
  treatedValue: z.literal(1),
})
const savedRun = z.object({
  study: z.string(),
  identification: z.string(),
  preparedDataset: z.string(),
  timeLabels: z.array(z.string()).length(2),
  columns: z.array(savedColumn).min(2),
  configuration: adjustedDidConfigurationSchema,
  evidence: adjustedDidEvidenceSchema,
  estimate: z.object({
    estimand: savedTarget,
    effect: z.object({ kind: z.literal('additive'), value: z.number().finite() }),
    standardError: z.number().finite(),
    interval: z.object({
      kind: z.literal('confidence'),
      level: z.literal(0.95),
      lower: z.number().finite(),
      upper: z.number().finite(),
    }),
  }),
})
export function adjustedDidRecordMatches(raw: unknown, rawStudy: unknown): boolean {
  const run = savedRun.safeParse(raw)
  const study = z
    .object({ id: z.string(), estimand: savedTarget, treatment: savedColumn, outcome: savedColumn })
    .safeParse(rawStudy)
  if (!run.success || !study.success) return false
  const { configuration: c, evidence: e, columns, estimate } = run.data
  return (
    run.data.study === study.data.id &&
    sameDidSpecification(c.specification, e.specification) &&
    c.covariates.length === e.covariates &&
    columns.length === 2 + c.covariates.length &&
    columns[0]?.column === study.data.outcome.column &&
    columns[1]?.column === study.data.treatment.column &&
    c.covariates.every(
      (id, index) =>
        columns[index + 2]?.column === id &&
        id !== study.data.outcome.column &&
        id !== study.data.treatment.column,
    ) &&
    estimate.effect.value === e.estimate &&
    estimate.standardError === e.standardError &&
    estimate.interval.lower === e.interval[0] &&
    estimate.interval.upper === e.interval[1]
  )
}
export function sameDidSpecification(
  a: AdjustedDidSpecification,
  b: AdjustedDidSpecification,
): boolean {
  if (a.kind === 'regression') return b.kind === 'regression'
  return (
    b.kind === 'doublyRobust' &&
    a.folds === b.folds &&
    a.seed === b.seed &&
    a.trimming === b.trimming &&
    a.normalization === b.normalization
  )
}

export const didNormalizationLabels = {
  'in-sample': 'In-sample',
  population: 'Population',
} as const
