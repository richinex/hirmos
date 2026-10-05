import { z } from 'zod'
import {
  ridgeRegularizationSchema,
  ridgeUncertaintySchema,
  ridgeAugmentedRequestSchema,
  ridgeAugmentedEvidenceSchema,
  type RidgeAugmentedRequest,
  type RidgeAugmentedEvidence,
} from './remixExtensions'
import type { PanelLongMatrix, PanelInterventionLayout } from './panel'
import type { MethodDefinition, MethodCaveat } from './methods'
import { mapNonEmpty } from './dop'
import { err, ok, type Result } from './dop'
export const ridgeConfigurationSchema = z
  .object({
    kind: z.literal('synthetic-control'),
    specification: z.literal('ridge-augmented'),
    treatedUnit: z.string().min(1).nullable(),
    donorUnits: z.array(z.string().min(1)).readonly(),
    interventionPeriod: z.number().int().nonnegative().nullable(),
    regularization: ridgeRegularizationSchema,
    uncertainty: ridgeUncertaintySchema,
  })
  .strict()
  .readonly()
export type RidgeConfiguration = z.infer<typeof ridgeConfigurationSchema>
export const defaultRidgeConfiguration: RidgeConfiguration = {
  kind: 'synthetic-control',
  specification: 'ridge-augmented',
  treatedUnit: null,
  donorUnits: [],
  interventionPeriod: null,
  regularization: {
    kind: 'crossValidation',
    maximum: null,
    minimumRatio: 1e-8,
    steps: 20,
    holdoutLength: 1,
    selection: 'oneStandardError',
  },
  uncertainty: { kind: 'jackknifePlus', confidence: 0.95 },
}
/** Seed a new specification only; never apply this to an edited or restored configuration. */
export function ridgeConfigurationForPanel(
  layout: PanelInterventionLayout | null,
): RidgeConfiguration {
  if (layout === null || layout.treated.length !== 1) return defaultRidgeConfiguration
  return {
    ...defaultRidgeConfiguration,
    treatedUnit: layout.treated[0],
    interventionPeriod: layout.adoption.code,
    donorUnits: [...layout.controls],
  }
}

/** Keep the shared requirement identities, with wording for the selected specification. */
export function ridgeMethodDefinition(method: MethodDefinition): MethodDefinition {
  const descriptions: Readonly<
    Record<string, Pick<MethodCaveat, 'requirement' | 'consequenceIfUnmet'>>
  > = {
    'synthetic-panel-layout': {
      requirement:
        'One treated unit and at least two untreated donor units are observed over common periods in a long panel.',
      consequenceIfUnmet: 'The selected units cannot be compared on the same outcome history.',
    },
    'synthetic-pre-period': {
      requirement:
        'The intervention period is known. At least two pre-treatment periods are required, with enough observations for the selected cross-validation blocks.',
      consequenceIfUnmet:
        'The pre-treatment fit or the selected cross-validation scheme cannot be estimated.',
    },
    'synthetic-convex-hull': {
      requirement:
        'Inspect pre-treatment fit and augmented donor weights. Ridge augmentation corrects imbalance using an outcome model and can assign negative weights.',
      consequenceIfUnmet:
        'Poor pre-treatment fit or substantial extrapolation makes the comparison more dependent on the outcome model.',
    },
    'synthetic-no-interval': {
      requirement:
        'When requested, jackknife intervals hold the initially selected lambda fixed during refits. Pointwise intervals describe individual post-treatment gaps; the headline interval describes their average.',
      consequenceIfUnmet:
        'Pointwise intervals may be mistaken for simultaneous bands or an interval for the average effect.',
    },
  }
  return {
    ...method,
    name: 'Ridge-augmented synthetic control',
    summary:
      'Ridge augmentation uses an outcome model to correct pre-treatment imbalance in the synthetic control. Augmented donor weights can be negative. The reported effect is the average post-treatment observed-minus-synthetic gap.',
    caveats: mapNonEmpty(method.caveats, (caveat) => {
      const description = descriptions[caveat.id]
      return description === undefined
        ? caveat
        : {
            ...caveat,
            ...description,
            sources: [
              {
                kind: 'hirmos-constraint',
                locator:
                  'src/domain/ridgeAugmented.ts; numerical reference: augsynth 0.2.0, single_augsynth and time_jackknife_plus',
              },
            ],
          }
    }),
  }
}

export function ridgeInput(
  matrix: PanelLongMatrix,
  c: RidgeConfiguration,
): Result<{ readonly values: Float64Array; readonly model: RidgeAugmentedRequest }, string> {
  if (c.treatedUnit === null || c.interventionPeriod === null)
    return err('Choose the treated unit and intervention period.')
  const selected = new Set([c.treatedUnit, ...c.donorUnits])
  for (let row = 0; row < matrix.rowCount; row++) {
    const unit = matrix.units[row],
      time = matrix.periodCodes[row]
    if (unit === undefined || time === undefined)
      return err('Every panel row needs unit and period keys.')
    if (!selected.has(unit)) continue
    const treatment = matrix.values[matrix.rowCount + row]
    const expected = unit === c.treatedUnit && time >= c.interventionPeriod ? 1 : 0
    if (treatment !== expected)
      return err(
        'The selected unit must adopt at the intervention period and remain treated. Donors must remain untreated.',
      )
  }
  const axis = [
    ...new Set(matrix.periodCodes.filter((_, i) => matrix.units[i] === c.treatedUnit)),
  ].sort((a, b) => a - b)
  if (!axis.includes(c.interventionPeriod))
    return err('The selected intervention period must be observed for the treated unit.')
  const values = matrix.values.slice(0, matrix.rowCount)
  if (values.some((v) => !Number.isFinite(v))) return err('Every outcome must be observed.')
  const parsed = ridgeAugmentedRequestSchema.safeParse({
    rows: matrix.rowCount,
    columns: 1,
    outcome: 0,
    units: matrix.units,
    times: matrix.periodCodes,
    treated: c.treatedUnit,
    donors: c.donorUnits,
    prePeriods: axis.filter((t) => t < c.interventionPeriod!).length,
    regularization: c.regularization,
    uncertainty: c.uncertainty,
  })
  return parsed.success ? ok({ values, model: parsed.data }) : err(z.prettifyError(parsed.error))
}
export function ridgeMatches(c: RidgeConfiguration, e: RidgeAugmentedEvidence): boolean {
  return (
    c.treatedUnit === e.request.treated &&
    c.interventionPeriod === e.periods[e.request.prePeriods] &&
    c.donorUnits.length === e.request.donors.length &&
    c.donorUnits.every((u, i) => u === e.request.donors[i]) &&
    JSON.stringify(c.regularization) === JSON.stringify(e.request.regularization) &&
    JSON.stringify(c.uncertainty) === JSON.stringify(e.request.uncertainty)
  )
}
export function ridgeRecordMatches(raw: unknown, rawStudy: unknown): boolean {
  const run = z
    .object({
      study: z.string(),
      configuration: ridgeConfigurationSchema,
      evidence: ridgeAugmentedEvidenceSchema,
      columns: z.array(z.object({ column: z.string() })),
      sourcePeriods: z.array(z.object({ code: z.number(), label: z.string().min(1) })),
      estimate: z.object({
        estimand: z.object({ kind: z.literal('average-treatment-effect-on-treated') }),
        effect: z.object({ kind: z.literal('additive'), value: z.number() }),
        interval: z.discriminatedUnion('kind', [
          z.object({ kind: z.literal('none'), reason: z.string() }),
          z.object({
            kind: z.literal('confidence'),
            level: z.number(),
            lower: z.number(),
            upper: z.number(),
          }),
        ]),
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
    i = r.estimate.interval
  return (
    r.study === study.data.id &&
    ridgeMatches(r.configuration, e) &&
    r.columns.length === 2 &&
    r.columns[0]?.column === study.data.outcome.column &&
    r.columns[1]?.column === study.data.treatment.column &&
    r.estimate.effect.value === e.average &&
    e.periods.every((t) => r.sourcePeriods.some((p) => p.code === t)) &&
    (e.bounds.kind === 'none'
      ? i.kind === 'none'
      : i.kind === 'confidence' &&
        e.request.uncertainty.kind !== 'none' &&
        i.level === e.request.uncertainty.confidence &&
        i.lower === e.bounds.averageLower &&
        i.upper === e.bounds.averageUpper)
  )
}
