import { z } from 'zod'
import { brand } from './dop'
import { describeProbe, type KappaRange } from './sensitivity'
import type { ConfigurationSetting } from './estimation'

const seed = z.number().int().min(0).max(4294967295)
export const refutationFields = {
  simulations: z.number().int().min(1).max(2000),
  subsetFraction: z.number().gt(0.1).lt(1),
  seed,
  ljungBoxLags: z.number().int().min(1).max(200),
}
const range = (value: z.ZodNumber) =>
  z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('inferred') }).strict(),
    z
      .object({
        kind: z.literal('range'),
        from: value,
        to: value,
        steps: z.number().int().min(1).max(200),
      })
      .strict(),
  ])
export const sensitivityConfigurationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('linear-refutation'), ...refutationFields }).strict(),
  z
    .object({
      kind: z.literal('unobserved-confounding'),
      seed,
      kappaT: range(z.number().min(0).max(1)),
      kappaY: range(z.number().finite()),
    })
    .strict(),
  z.object({ kind: z.literal('dml-refutation'), seed }).strict(),
])
export const sensitivitySettingsSchema = z
  .object({
    estimationRun: z
      .string()
      .min(1)
      .transform((id) => brand<string, 'EstimationRunId'>(id))
      .nullable(),
    configuration: sensitivityConfigurationSchema,
  })
  .strict()
export type SensitivitySettings = z.infer<typeof sensitivitySettingsSchema>

export function sensitivitySettings(value: SensitivitySettings): readonly ConfigurationSetting[] {
  const c = value.configuration
  const strength = (v: KappaRange) =>
    v.kind === 'inferred'
      ? 'Infer from observed common causes'
      : `${v.from} to ${v.to}, ${v.steps} values`
  return [
    { key: 'source', label: 'Source run', value: value.estimationRun ?? 'Not selected' },
    { key: 'probe', label: 'Probe', value: describeProbe(c.kind) },
    {
      key: 'seed',
      label: c.kind === 'dml-refutation' ? 'Fold seed' : 'Seed',
      value: String(c.seed),
    },
    ...(c.kind === 'linear-refutation'
      ? [
          { key: 'simulations', label: 'Simulations', value: String(c.simulations) },
          { key: 'subsetFraction', label: 'Subset fraction', value: String(c.subsetFraction) },
          { key: 'ljungBoxLags', label: 'Ljung–Box lags', value: String(c.ljungBoxLags) },
        ]
      : c.kind === 'unobserved-confounding'
        ? [
            { key: 'kappaT', label: 'Treatment strength', value: strength(c.kappaT) },
            { key: 'kappaY', label: 'Outcome strength', value: strength(c.kappaY) },
          ]
        : []),
  ]
}
