import { z } from 'zod'

export const adjustedDidSpecificationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('regression') }).strict(),
  z.object({ kind: z.literal('doublyRobust'), folds: z.number().int().min(2), seed: z.number().int().min(0).max(0xffffffff), trimming: z.number().finite().gt(0).lt(0.5), normalization: z.enum(['in-sample', 'population']) }).strict(),
])
export type AdjustedDidSpecification = z.infer<typeof adjustedDidSpecificationSchema>
export const adjustedDidConfigurationSchema = z.object({ kind: z.literal('panel-intervention'), primary: z.literal('adjusted'), covariates: z.array(z.string().min(1)).refine(values => new Set(values).size === values.length), specification: adjustedDidSpecificationSchema }).strict()
const interval = z.tuple([z.number().finite(), z.number().finite()]).refine(([lo, hi]) => lo <= hi)
export const adjustedDidEvidenceSchema = z.object({
  kind: z.literal('panelAdjusted'), observations: z.number().int().positive(), units: z.array(z.string().min(1)).min(2), times: z.array(z.number().int()).length(2),
  controlUnits: z.number().int().positive(), treatedUnits: z.number().int().positive(), nPre: z.literal(1), nPost: z.literal(1), covariates: z.number().int().nonnegative(),
  specification: adjustedDidSpecificationSchema, estimate: z.number().finite(), standardError: z.number().finite().nonnegative(), interval,
  groupMeans: z.tuple([z.tuple([z.number().finite(), z.number().finite()]), z.tuple([z.number().finite(), z.number().finite()])]),
  inference: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('independentErrors'), degreesOfFreedom: z.number().int().positive(), coefficients: z.array(z.number().finite()).min(4), standardErrors: z.array(z.number().finite().nonnegative()).min(4), intervals: z.array(interval).min(4) }).strict(),
    z.object({ kind: z.literal('crossFitted'), propensity: z.array(z.number().finite().gt(0).lt(1)).min(2), optimizerStatus: z.array(z.enum(['projected-gradient', 'function-tolerance', 'iteration-limit', 'line-search-failed'])).min(2) }).strict(),
  ]),
}).strict().superRefine((e, ctx) => {
  const issue = (message: string) => ctx.addIssue({ code: 'custom', message })
  if (e.observations !== 2 * e.units.length || e.units.length !== e.controlUnits + e.treatedUnits || new Set(e.units).size !== e.units.length || e.times[0]! >= e.times[1]!) issue('DiD requires distinct units with two ordered observations each.')
  switch (e.specification.kind) {
    case 'regression': {
      const i = e.inference
      if (i.kind !== 'independentErrors') { issue('Regression DiD requires classical independent-error inference.'); break }
      const p = 4 + e.covariates
      if (i.coefficients.length !== p || i.standardErrors.length !== p || i.intervals.length !== p || i.degreesOfFreedom !== e.observations-p) issue('Regression DiD inference dimensions disagree with its specification.')
      if (e.estimate !== i.coefficients[3] || e.standardError !== i.standardErrors[3] || e.interval[0] !== i.intervals[3]?.[0] || e.interval[1] !== i.intervals[3]?.[1]) issue('The DiD headline must be the group-by-post interaction.')
      break
    }
    case 'doublyRobust': {
      const i = e.inference
      if (i.kind !== 'crossFitted') { issue('DR DiD requires cross-fitted inference.'); break }
      if (e.covariates < 1 || i.propensity.length !== e.units.length || i.optimizerStatus.length !== e.specification.folds || e.specification.folds > Math.min(e.controlUnits,e.treatedUnits)) issue('DR DiD folds, covariates or propensity dimensions disagree.')
      break
    }
  }
})
export type AdjustedDidEvidence = z.infer<typeof adjustedDidEvidenceSchema>
const savedColumn = z.object({ column: z.string() })
const savedTarget = z.object({ kind: z.literal('average-treatment-effect-on-treated'), scale: z.literal('additive'), treatedValue: z.literal(1) })
const savedRun = z.object({
  study: z.string(), identification: z.string(), preparedDataset: z.string(), timeLabels: z.array(z.string()).length(2), columns: z.array(savedColumn).min(2),
  configuration: adjustedDidConfigurationSchema, evidence: adjustedDidEvidenceSchema,
  estimate: z.object({ estimand: savedTarget, effect: z.object({ kind: z.literal('additive'), value: z.number().finite() }), standardError: z.number().finite(), interval: z.object({ kind:z.literal('confidence'), level:z.literal(0.95), lower:z.number().finite(), upper:z.number().finite() }) }),
})
export function adjustedDidRecordMatches(raw: unknown, rawStudy: unknown): boolean {
  const run = savedRun.safeParse(raw)
  const study = z.object({ id:z.string(), estimand:savedTarget, treatment:savedColumn, outcome:savedColumn }).safeParse(rawStudy)
  if (!run.success || !study.success) return false
  const { configuration:c, evidence:e, columns, estimate } = run.data
  return run.data.study === study.data.id && sameDidSpecification(c.specification,e.specification) && c.covariates.length === e.covariates
    && columns.length === 2+c.covariates.length && columns[0]?.column === study.data.outcome.column && columns[1]?.column === study.data.treatment.column
    && c.covariates.every((id,index) => columns[index+2]?.column === id && id !== study.data.outcome.column && id !== study.data.treatment.column)
    && estimate.effect.value === e.estimate && estimate.standardError === e.standardError && estimate.interval.lower === e.interval[0] && estimate.interval.upper === e.interval[1]
}
export function sameDidSpecification(a: AdjustedDidSpecification, b: AdjustedDidSpecification): boolean {
  if (a.kind === 'regression') return b.kind === 'regression'
  return b.kind === 'doublyRobust' && a.folds === b.folds && a.seed === b.seed && a.trimming === b.trimming && a.normalization === b.normalization
}
