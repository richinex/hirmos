import { z } from 'zod'
import { absorbingAdoption } from './absorbingAdoption'
import type { PanelLongMatrix, PanelPeriod } from './panel'
import { err, ok, type Result } from './dop'
import {
  sunAbrahamRequestSchema,
  sunAbrahamEvidenceSchema,
  type SunAbrahamRequest,
} from './remixExtensions'

export const sunAbrahamConfigurationSchema = z
  .object({
    kind: z.literal('panel-intervention'),
    primary: z.literal('sunAbraham'),
    referencePeriods: z.tuple([z.number().int().safe()]).rest(z.number().int().safe()).readonly(),
    referenceCohorts: z.array(z.number().int().safe()).readonly(),
    confidence: z.number().finite().gt(0).lt(1),
  })
  .strict()
  .refine(
    (c) => new Set(c.referencePeriods).size === c.referencePeriods.length,
    'Reference periods must not be repeated.',
  )
  .refine(
    (c) => new Set(c.referenceCohorts).size === c.referenceCohorts.length,
    'Reference cohorts must not be repeated.',
  )
  .readonly()
export type SunAbrahamConfiguration = z.infer<typeof sunAbrahamConfigurationSchema>
export const defaultSunAbraham: SunAbrahamConfiguration = {
  kind: 'panel-intervention',
  primary: 'sunAbraham',
  referencePeriods: [-1],
  referenceCohorts: [],
  confidence: 0.95,
}
export interface SunAbrahamCatalog {
  readonly cohorts: readonly (PanelPeriod & { readonly units: number })[]
  readonly neverTreated: number
  readonly eventPeriods: readonly number[]
}

export function sunAbrahamCatalog(matrix: PanelLongMatrix): Result<SunAbrahamCatalog, string> {
  const adoption = absorbingAdoption(matrix, 'not-required')
  if (!adoption.ok) return adoption
  const ranges = new Map<number, { min: number; units: number }>()
  let neverTreated = 0
  for (const cohort of adoption.value.values()) {
    if (cohort === null) {
      neverTreated++
      continue
    }
    const range = ranges.get(cohort) ?? { min: Infinity, units: 0 }
    range.units++
    ranges.set(cohort, range)
  }
  const eventPeriods = new Set<number>()
  matrix.units.forEach((unit, i) => {
    const cohort = adoption.value.get(unit),
      time = matrix.periodCodes[i]
    if (cohort === undefined || cohort === null || time === undefined) return
    const range = ranges.get(cohort)
    if (range !== undefined) range.min = Math.min(range.min, time - cohort)
    eventPeriods.add(time - cohort)
  })
  const cohorts = matrix.periods.flatMap((period) => {
    const range = ranges.get(period.code)
    return range !== undefined && range.min < 0 ? [{ ...period, units: range.units }] : []
  })
  if (cohorts.length === 0) return err('No adoption cohort has an observed pre-treatment period.')
  return ok({ cohorts, neverTreated, eventPeriods: [...eventPeriods].sort((a, b) => a - b) })
}

export function sunAbrahamComparison(
  catalog: SunAbrahamCatalog,
  configuration: SunAbrahamConfiguration,
): Result<null, string> {
  if (!sunAbrahamConfigurationSchema.safeParse(configuration).success)
    return err('Choose valid reference cohorts, event periods and a confidence level.')
  if (configuration.referenceCohorts.some((code) => !catalog.cohorts.some((c) => c.code === code)))
    return err(
      'A selected reference cohort is absent or has no observed pre-treatment period. Review the cohort selection.',
    )
  if (catalog.neverTreated === 0 && configuration.referenceCohorts.length === 0)
    return err(
      'This panel has no never-treated units. Select an adoption cohort as a reference, such as the last-treated cohort, to define the comparison.',
    )
  if (catalog.cohorts.every((c) => configuration.referenceCohorts.includes(c.code)))
    return err(
      'Leave at least one adoption cohort outside the reference group so its treatment effects can be estimated.',
    )
  if (configuration.referencePeriods.some((p) => !catalog.eventPeriods.includes(p)))
    return err('A selected reference event period is not observed in this panel.')
  return ok(null)
}

export function sunAbrahamInput(
  matrix: PanelLongMatrix,
  configuration: SunAbrahamConfiguration,
): Result<{ readonly values: Float64Array; readonly model: SunAbrahamRequest }, string> {
  const catalog = sunAbrahamCatalog(matrix)
  if (!catalog.ok) return catalog
  const comparison = sunAbrahamComparison(catalog.value, configuration)
  if (!comparison.ok) return comparison
  const cohorts = absorbingAdoption(matrix, 'not-required')
  if (!cohorts.ok) return cohorts
  const adoption: (number | null)[] = []
  for (const unit of matrix.units) {
    const g = cohorts.value.get(unit)
    if (g === undefined) return err('Every unit needs an adoption record.')
    adoption.push(g)
  }
  const values = matrix.values.slice(0, matrix.rowCount)
  if (values.some((v) => !Number.isFinite(v)))
    return err('Every retained outcome must be observed.')
  const parsed = sunAbrahamRequestSchema.safeParse({
    rows: matrix.rowCount,
    columns: 1,
    outcome: 0,
    units: matrix.units,
    times: matrix.periodCodes,
    adoption,
    referencePeriods: configuration.referencePeriods,
    referenceCohorts: configuration.referenceCohorts,
    confidence: configuration.confidence,
  })
  return parsed.success ? ok({ values, model: parsed.data }) : err(z.prettifyError(parsed.error))
}
export function sameSunAbraham(
  configuration: SunAbrahamConfiguration,
  request: SunAbrahamRequest,
): boolean {
  return (
    request.confidence === configuration.confidence &&
    request.referenceCohorts.length === configuration.referenceCohorts.length &&
    request.referenceCohorts.every((c, i) => c === configuration.referenceCohorts[i]) &&
    request.referencePeriods.length === configuration.referencePeriods.length &&
    request.referencePeriods.every((p, i) => p === configuration.referencePeriods[i])
  )
}
export function sunAbrahamRecordMatches(raw: unknown, rawStudy: unknown): boolean {
  const run = z
    .object({
      study: z.string(),
      configuration: sunAbrahamConfigurationSchema,
      evidence: sunAbrahamEvidenceSchema,
      columns: z.array(z.object({ column: z.string() })),
      timeLabels: z.array(z.string()),
      sourcePeriods: z.array(z.object({ code: z.number(), label: z.string().min(1) })),
      estimate: z.object({
        estimand: z.object({
          kind: z.literal('average-treatment-effect-on-treated'),
          scale: z.literal('additive'),
          treatedValue: z.literal(1),
        }),
        effect: z.object({ kind: z.literal('additive'), value: z.number() }),
        standardError: z.number(),
        interval: z.object({
          kind: z.literal('confidence'),
          level: z.number(),
          lower: z.number(),
          upper: z.number(),
        }),
      }),
    })
    .safeParse(raw)
  const study = z
    .object({
      id: z.string(),
      estimand: z.object({
        kind: z.literal('average-treatment-effect-on-treated'),
        scale: z.literal('additive'),
        treatedValue: z.literal(1),
      }),
      outcome: z.object({ column: z.string() }),
      treatment: z.object({ column: z.string() }),
    })
    .safeParse(rawStudy)
  if (!run.success || !study.success) return false
  const r = run.data,
    e = r.evidence,
    point = e.overall
  return (
    r.study === study.data.id &&
    sameSunAbraham(r.configuration, e.request) &&
    r.columns.length === 2 &&
    r.columns[0]?.column === study.data.outcome.column &&
    r.columns[1]?.column === study.data.treatment.column &&
    r.estimate.effect.value === point.estimate &&
    r.estimate.standardError === point.standardError &&
    r.estimate.interval.level === r.configuration.confidence &&
    r.estimate.interval.lower === point.lower &&
    r.estimate.interval.upper === point.upper &&
    e.times.length === r.timeLabels.length &&
    e.times.every((t, i) =>
      r.sourcePeriods.some((p) => p.code === t && p.label === r.timeLabels[i]),
    )
  )
}
