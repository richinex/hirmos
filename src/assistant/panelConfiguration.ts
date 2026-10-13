import { z } from 'zod'
import type { ColumnId } from '@/domain/dataset'
import type { PanelInterventionConfiguration } from '@/domain/estimation'
import { adjustedDidConfigurationSchema } from '@/domain/adjustedDid'
import { staggeredConfigurationSchema } from '@/domain/staggeredDid'
import { sunAbrahamConfigurationSchema } from '@/domain/sunAbraham'
import { conventionalDidInferenceSchema, panelResamplingSchema } from '@/domain/conventionalDid'
import { err, ok, type Result } from '@/domain/dop'

export const panelConfigurationSchema = z.union([
  adjustedDidConfigurationSchema,
  staggeredConfigurationSchema,
  sunAbrahamConfigurationSchema,
  panelResamplingSchema
    .extend({
      kind: z.literal('panel-intervention'),
      primary: z.literal('did'),
      inference: conventionalDidInferenceSchema,
    })
    .strict(),
  panelResamplingSchema
    .extend({ kind: z.literal('panel-intervention'), primary: z.literal('syntheticDid') })
    .strict(),
])

export function readPanelConfiguration(
  raw: unknown,
  columns: readonly { readonly id: ColumnId }[],
): Result<PanelInterventionConfiguration, 'invalid'> {
  const parsed = panelConfigurationSchema.safeParse(raw)
  if (!parsed.success) return err('invalid')
  const configuration = parsed.data
  if (configuration.primary !== 'adjusted' && configuration.primary !== 'staggered')
    return ok(configuration)
  const covariates: ColumnId[] = []
  for (const id of configuration.covariates) {
    const column = columns.find((column) => column.id === id)
    if (!column) return err('invalid')
    covariates.push(column.id)
  }
  if (
    configuration.primary === 'staggered' &&
    'clustering' in configuration &&
    configuration.clustering.kind === 'column'
  ) {
    const clusterId = configuration.clustering.column
    const cluster = columns.find((column) => column.id === clusterId)
    if (!cluster) return err('invalid')
    return ok<PanelInterventionConfiguration>({
      ...configuration,
      covariates,
      clustering: { kind: 'column' as const, column: cluster.id },
    })
  }
  if (configuration.primary === 'adjusted') return ok({ ...configuration, covariates })
  const normalized: PanelInterventionConfiguration = {
    kind: 'panel-intervention',
    primary: 'staggered',
    covariates,
    specification: configuration.specification,
    headline: configuration.headline,
    ...('clustering' in configuration ? { clustering: { kind: 'unit' as const } } : {}),
  }
  return ok(normalized)
}
