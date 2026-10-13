import { z } from 'zod'
import { didSensitivityRequestSchema } from './didSensitivity'

export const didSensitivitySettingsSchema = didSensitivityRequestSchema
  .pick({
    scenarios: true,
    rho: true,
    level: true,
    null: true,
  })
  .extend({
    sourceRun: z.string().min(1),
    outcomeMaximum: z.number().gt(0).lt(1),
    rieszMaximum: z.number().gt(0).lt(1),
    points: z.number().int().min(2).max(51),
  })
  .strict()
