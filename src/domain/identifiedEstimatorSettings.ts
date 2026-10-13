import { z } from 'zod'
import { totalEffectEstimatorSchema, causalEffectsUncertaintySchema } from './estimation'

export const posteriorSamplingFields = {
  warmup: z.number().int().min(10).max(5000),
  samples: z.number().int().min(10).max(5000),
  seed: z.number().int().nonnegative(),
}

export const identifiedBootstrapFields = {
  simulations: z.number().int().min(20).max(10_000),
  sampleSizeFraction: z.number().gt(0).max(2),
  seed: z.number().int().nonnegative(),
}

const bootstrapSettings = {
  ...identifiedBootstrapFields,
  simulations: identifiedBootstrapFields.simulations.max(5000),
  sampleSizeFraction: z.literal(1),
  level: z.literal(0.95),
}

export const identifiedEstimatorSettingsSchema = z.union([
  z
    .object({
      kind: z.literal('frontdoor-two-stage'),
      interventions: z.tuple([z.number().finite(), z.number().finite()]),
      ...bootstrapSettings,
    })
    .strict(),
  z.object({ kind: z.literal('instrumental-variable'), ...bootstrapSettings }).strict(),
  z
    .object({ kind: z.enum(['negbin-nuts', 'bayesian-gaussian']), ...posteriorSamplingFields })
    .strict(),
  z
    .object({
      kind: z.literal('causal-effects-total'),
      estimator: totalEffectEstimatorSchema,
      treatmentLag: z.number().int().min(0).max(20),
      interventions: z.tuple([z.number().finite(), z.number().finite()]),
      uncertainty: causalEffectsUncertaintySchema,
    })
    .strict(),
])
