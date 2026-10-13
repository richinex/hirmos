import { z } from 'zod'
import type { ColumnId } from '@/domain/dataset'
import { err, ok, type Result } from '@/domain/dop'
import { outcomeHistorySyntheticConfigurationSchema, type SyntheticControlConfiguration } from '@/domain/estimation'
import { ridgeConfigurationSchema } from '@/domain/ridgeAugmented'
import { predictorSyntheticConfigurationSchema } from '@/domain/predictorSyntheticControl'

export const syntheticConfigurationSchema = z.union([
  outcomeHistorySyntheticConfigurationSchema,
  ridgeConfigurationSchema,
  predictorSyntheticConfigurationSchema,
])

export function readSyntheticConfiguration(
  raw: unknown,
  columns: readonly { readonly id: ColumnId }[],
): Result<SyntheticControlConfiguration, 'invalid'> {
  const parsed = syntheticConfigurationSchema.safeParse(raw)
  if (!parsed.success) return err('invalid')
  const configuration = parsed.data
  const selected = configuration.specification === 'predictors'
    ? [...configuration.predictors, ...configuration.special.map((item) => item.column)]
    : configuration.specification === 'ridge-augmented' ? [] : configuration.donors
  if (selected.some((id) => !columns.some((column) => column.id === id))) return err('invalid')
  return ok(configuration)
}
