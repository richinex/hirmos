import { z } from 'zod'
import { err, ok } from '@/domain/dop'
import type { ColumnId } from '@/domain/dataset'
import type { CovariateEncoding } from '@/domain/study'
import { useWorkspaceConfiguration } from './workspaceConfiguration'

export const encodingSettingsSchema = z
  .object({ categorical: z.array(z.string()).max(256) })
  .strict()

export function useEncodingSettings(options: {
  readonly binding: string
  readonly available: boolean
  readonly blocked: boolean
  readonly columns: readonly { readonly id: ColumnId; readonly name: string }[]
  readonly encodings: Readonly<Record<string, CovariateEncoding>>
  readonly configure: (selected: readonly string[]) => void
}) {
  useWorkspaceConfiguration(
    'covariate-encoding',
    options.binding,
    options.available
      ? {
          title: 'Categorical covariates',
          current: {
            categorical: options.columns
              .filter((c) => options.encodings[c.id]?.kind === 'categorical')
              .map((c) => String(c.id)),
          },
          schema: encodingSettingsSchema,
          columns: options.columns,
          describe: (value) =>
            options.columns.map((c) => ({
              key: c.id,
              label: c.name,
              value: value.categorical.includes(c.id) ? 'Categorical' : 'Numeric',
            })),
          validate: (value) => {
            if (options.blocked)
              return err('Wait for the analysis to finish before changing covariate encodings.')
            if (
              new Set(value.categorical).size !== value.categorical.length ||
              value.categorical.some((id) => !options.columns.some((c) => c.id === id))
            )
              return err(
                'Choose each categorical covariate once from the current adjustment variables.',
              )
            return ok(value)
          },
          configure: (value) => options.configure(value.categorical),
        }
      : null,
  )
}
