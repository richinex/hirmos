import { z } from 'zod'

const resampling = {
  replications: z.number().int().min(2).max(2000),
  seed: z.number().int().min(0).max(0xffff_ffff),
}
export const conventionalDidInferenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('bootstrap'), ...resampling }).strict(),
  z.object({ kind: z.literal('jackknife') }).strict(),
  z.object({ kind: z.literal('placebo'), ...resampling }).strict(),
])
export type ConventionalDidInference = z.infer<typeof conventionalDidInferenceSchema>
export const defaultConventionalDidInference: ConventionalDidInference = {
  kind: 'bootstrap',
  replications: 200,
  seed: 0,
}
export const conventionalDidUncertaintySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notRequested') }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  z
    .object({
      kind: z.literal('available'),
      method: z.enum(['bootstrap', 'jackknife', 'placebo']),
      standardError: z.number().finite().nonnegative(),
    })
    .strict(),
])
export const conventionalDidInferenceLabels = {
  bootstrap: 'Unit bootstrap',
  jackknife: 'Leave-one-unit-out jackknife',
  placebo: 'Placebo',
} as const
export const conventionalDidInferenceHelp = {
  bootstrap:
    'Resample whole units with replacement, keeping every period for each sampled unit. Each resample must include treated and control units. The original panel must contain at least two treated units.',
  jackknife:
    'Leave out one unit at a time and estimate the treatment contrast again. The panel must contain at least two treated and two control units.',
  placebo:
    'Select control units to receive placebo treatment and compare them with the remaining controls. Each placebo group has the same number of units as the original treated group. The calculation assumes comparable error variation in treated and control units.',
} as const

export const conventionalDidPlaceboUnavailable = (layout: {
  readonly treatedUnits: number
  readonly controlUnits: number
}): string | null =>
  layout.controlUnits > layout.treatedUnits
    ? null
    : `Placebo uncertainty requires more control units than treated units. This panel has ${layout.treatedUnits} treated and ${layout.controlUnits} control units.`
