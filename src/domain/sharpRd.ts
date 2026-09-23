import { z } from 'zod'
import { err, ok, type Result } from './dop'

export const sharpRdConfigurationSchema = z.object({ kind: z.literal('sharp-rd') }).strict()
export type SharpRdConfiguration = z.infer<typeof sharpRdConfigurationSchema>

const interval = z.tuple([z.number().finite(), z.number().finite()]).refine(([lower, upper]) => lower <= upper)
const estimate = z.object({ value: z.number().finite(), standardError: z.number().finite().nonnegative(), interval }).strict()
const counts = z.tuple([z.number().int().positive(), z.number().int().positive()])
export const sharpRdEvidenceSchema = z.object({
  kind: z.literal('sharpRd'), cutoff: z.number().finite(),
  target: z.literal('local-at-cutoff'), assignment: z.literal('at-or-above'),
  kernel: z.literal('triangular'), bandwidthSelection: z.literal('mserd'),
  polynomialOrder: z.literal(1), biasOrder: z.literal(2), nearestNeighbors: z.literal(3),
  bandwidth: z.number().finite().positive(), biasBandwidth: z.number().finite().positive(),
  conventional: estimate, biasCorrected: estimate, robust: estimate,
  observations: counts, effectiveObservations: counts,
  leftCoefficients: z.tuple([z.number().finite(), z.number().finite()]),
  rightCoefficients: z.tuple([z.number().finite(), z.number().finite()]),
  points: z.array(z.tuple([z.number().finite(), z.number().finite()])).nonempty(),
}).strict().superRefine((value, context) => {
  if (value.points.length !== value.observations[0] + value.observations[1]) context.addIssue({ code: 'custom', message: 'RD observations do not match the saved plot rows.' })
  const left = value.points.filter(([x]) => x < value.cutoff).length
  if (left !== value.observations[0]) context.addIssue({ code: 'custom', message: 'RD side counts do not match the cutoff.' })
  for (const side of [0, 1] as const) {
    if (value.effectiveObservations[side] > value.observations[side]) context.addIssue({ code: 'custom', message: 'RD effective sample exceeds its available observations.' })
  }
})
export type SharpRdEvidence = z.infer<typeof sharpRdEvidenceSchema>

const variable = z.object({ column: z.string().min(1) })
const target = z.object({ kind: z.literal('local-cutoff-effect'), scale: z.literal('additive'), running: variable, cutoff: z.number().finite(), assignment: z.literal('at-or-above') })
const savedStudy = z.object({ id: z.string(), estimand: target, outcome: variable, treatment: variable })
const savedIdentification = z.object({ study: z.string(), result: z.object({ kind: z.literal('cutoff-design') }) })
const savedRun = z.object({
  columns: z.array(variable).length(3),
  estimate: z.object({ estimand: target, effect: z.object({ kind: z.literal('additive'), value: z.number().finite() }),
    interval: z.object({ kind: z.literal('confidence'), level: z.literal(0.95), lower: z.number().finite(), upper: z.number().finite() }) }),
})

/** Decode unknown records before checking that the result belongs to this design. */
export function sharpRdRecordMatches(rawRun: unknown, rawStudy: unknown, rawIdentification: unknown, evidence: SharpRdEvidence): boolean {
  const run = savedRun.safeParse(rawRun)
  const study = savedStudy.safeParse(rawStudy)
  const identification = savedIdentification.safeParse(rawIdentification)
  if (!run.success || !study.success || !identification.success) return false
  const { estimand, outcome, treatment } = study.data
  const { estimate, columns } = run.data
  return identification.data.study === study.data.id && estimand.cutoff === evidence.cutoff
    && columns[0]?.column === estimand.running.column && columns[1]?.column === outcome.column && columns[2]?.column === treatment.column
    && estimate.estimand.cutoff === estimand.cutoff && estimate.estimand.running.column === estimand.running.column
    && estimate.effect.value === evidence.robust.value
    && estimate.interval.lower === evidence.robust.interval[0] && estimate.interval.upper === evidence.robust.interval[1]
}
export function parseSharpRdEvidence(value: unknown): Result<SharpRdEvidence, { readonly kind: 'invalid-estimation-evidence'; readonly detail: string }> {
  const parsed = sharpRdEvidenceSchema.safeParse(value)
  return parsed.success ? ok(parsed.data) : err({ kind: 'invalid-estimation-evidence', detail: z.prettifyError(parsed.error) })
}
