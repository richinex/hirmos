import { z } from 'zod'
import { brand, err, ok, isNonEmpty, type Result } from './dop'
import { isNumericDuckDbType, type DatasetProfile } from './dataset'
import { timeInterpretationSchema } from './timeInterpretation'
import {
  readyPreprocessingRecipe,
  describeReadinessProblem,
  selectedMissingness,
  describeSeriesTransform,
  type PreprocessingDraft,
} from './preprocessing'
import type { ConfigurationSetting } from './estimation'

const column = z
  .string()
  .min(1)
  .describe('Use the column id from workspace_settings.columns, not its display name.')
  .transform((id) => brand<string, 'ColumnId'>(id))
const frequency = z.enum(['daily', 'weekly', 'monthly', 'quarterly', 'yearly'])
const transform = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('levels') }).strict(),
  z.object({ kind: z.literal('difference'), order: z.literal(1) }).strict(),
  z.object({ kind: z.literal('linear-detrend') }).strict(),
])
export const preprocessingSettingsSchema = z
  .object({
    sampling: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('unconfigured') }).strict(),
      z.object({ kind: z.literal('cross-sectional') }).strict(),
      z.object({ kind: z.literal('regular-series-awaiting-time'), frequency }).strict(),
      z
        .object({
          kind: z.literal('regular-series'),
          timeColumn: column,
          frequency,
          interpretation: timeInterpretationSchema.optional(),
        })
        .strict(),
      z
        .object({
          kind: z.literal('regular-panel-awaiting-keys'),
          unitColumn: column.nullable(),
          timeColumn: column.nullable(),
          frequency,
        })
        .strict(),
      z
        .object({
          kind: z.literal('regular-panel'),
          unitColumn: column,
          timeColumn: column,
          frequency,
        })
        .strict(),
    ]),
    columns: z
      .array(column)
      .refine((ids) => new Set(ids).size === ids.length, 'Select each column once.'),
    missingness: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('not-present') }).strict(),
      z.object({ kind: z.literal('unresolved') }).strict(),
      z.object({ kind: z.literal('complete-interval') }).strict(),
      z
        .object({
          kind: z.literal('imputation'),
          method: z.enum(['linearInterior', 'forwardFill', 'structuralZero']),
          maxGap: z.number().int().min(1).max(1000),
        })
        .strict(),
      z
        .object({
          kind: z.literal('lag-aware-exclusion'),
          cutOff: z.enum([
            'method-default',
            '2xtau-max',
            'tau-max',
            'max-lag',
            'max-lag-or-tau-max',
            '2xtau-max-future',
          ]),
          propagateThroughMaxLag: z.boolean(),
          analysisExclusions: z.discriminatedUnion('kind', [
            z.object({ kind: z.literal('ignore') }).strict(),
            z
              .object({
                kind: z.literal('roles'),
                roles: z
                  .tuple([z.enum(['candidate-cause', 'tested-outcome', 'conditioner'])])
                  .rest(z.enum(['candidate-cause', 'tested-outcome', 'conditioner'])),
              })
              .strict(),
          ]),
        })
        .strict(),
    ]),
    resampling: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('none') }).strict(),
      z
        .object({
          kind: z.literal('daily-downsample'),
          targetFrequency: z.enum(['weekly', 'monthly']),
          incompleteBins: z.enum(['keep', 'drop']),
          aggregations: z.array(
            z
              .object({
                column,
                aggregation: z.enum([
                  'mean',
                  'sum',
                  'median',
                  'minimum',
                  'maximum',
                  'first',
                  'last',
                ]),
              })
              .strict(),
          ),
        })
        .strict(),
    ]),
    seasonal: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('none') }).strict(),
      z.object({ kind: z.literal('stl'), columns: z.array(column), robust: z.boolean() }).strict(),
    ]),
    seriesTransforms: z.array(z.object({ column, transform }).strict()),
  })
  .strict()
export type PreprocessingSettings = z.infer<typeof preprocessingSettingsSchema>

export function preprocessingSettings(draft: PreprocessingDraft): PreprocessingSettings {
  const { missingness: m } = draft
  const missingness: PreprocessingSettings['missingness'] =
    m.kind === 'imputation'
      ? { kind: m.kind, method: m.method, maxGap: m.maxGap }
      : m.kind === 'lag-aware-exclusion'
        ? {
            kind: m.kind,
            cutOff: m.cutOff,
            propagateThroughMaxLag: m.propagateThroughMaxLag,
            analysisExclusions:
              m.analysisExclusions.kind === 'ignore'
                ? m.analysisExclusions
                : { kind: 'roles', roles: [...m.analysisExclusions.roles] },
          }
        : { kind: m.kind }
  return {
    sampling: draft.sampling,
    columns: draft.variables.kind === 'selected' ? [...draft.variables.columns] : [],
    missingness,
    resampling:
      draft.resampling.kind === 'none'
        ? draft.resampling
        : { ...draft.resampling, aggregations: [...draft.resampling.aggregations] },
    seasonal:
      draft.seasonal.kind === 'none'
        ? draft.seasonal
        : { ...draft.seasonal, columns: [...draft.seasonal.columns] },
    seriesTransforms: [...draft.seriesTransforms],
  }
}

export function preprocessingDraftFromSettings(
  value: PreprocessingSettings,
  current: PreprocessingDraft,
  profile: DatasetProfile,
): Result<PreprocessingDraft, string> {
  const parsed = preprocessingSettingsSchema.safeParse(value)
  if (!parsed.success) return err(parsed.error.issues.map((issue) => issue.message).join('; '))
  const s = value.sampling
  const keys =
    s.kind === 'regular-series'
      ? [s.timeColumn]
      : s.kind === 'regular-panel'
        ? [s.unitColumn, s.timeColumn]
        : []
  if (keys.some((id) => !profile.columns.some((c) => c.id === id)))
    return err('A panel or time key is not in the current source.')
  if (s.kind === 'regular-panel' && s.unitColumn === s.timeColumn)
    return err('Choose different unit and time columns.')
  if (!isNonEmpty(value.columns)) return err('Choose at least one analysis column.')
  for (const id of value.columns) {
    const selected = profile.columns.find((c) => c.id === id)
    if (!selected) {
      const named = profile.columns.filter((c) => c.name === id)
      return err(
        named.length === 1
          ? `“${id}” is a column name. Use its column ID “${named[0]!.id}” in the settings.`
          : `Column reference “${id}” was not found. Use the column IDs listed in the workspace settings.`,
      )
    }
    if (!isNumericDuckDbType(selected.duckdbType))
      return err(
        `“${selected.name}” has type ${selected.duckdbType}. Analysis columns must be numeric.`,
      )
  }
  const transforms = value.seriesTransforms.map((t) => t.column)
  const seasonal = value.seasonal.kind === 'stl' ? value.seasonal.columns : []
  const aggregations =
    value.resampling.kind === 'daily-downsample'
      ? value.resampling.aggregations.map((a) => a.column)
      : []
  if (
    [transforms, seasonal, aggregations].some(
      (ids) => new Set(ids).size !== ids.length || ids.some((id) => !value.columns.includes(id)),
    )
  )
    return err('Transformations must name distinct selected analysis columns.')
  if (
    s.kind !== 'regular-series' &&
    (value.resampling.kind !== 'none' ||
      value.seasonal.kind !== 'none' ||
      value.seriesTransforms.some((t) => t.transform.kind !== 'levels'))
  )
    return err(
      'Resampling, seasonal adjustment and series transformations require a regular time series.',
    )
  if (value.resampling.kind !== 'none' && s.kind === 'regular-series' && s.frequency !== 'daily')
    return err('Calendar downsampling requires a daily source series.')
  const base: PreprocessingDraft = {
    ...current,
    sampling: value.sampling,
    resampling: value.resampling,
    seasonal: value.seasonal,
    seriesTransforms: value.seriesTransforms,
    variables: { kind: 'selected', columns: value.columns },
    frequencyOrigin: 'chosen',
  }
  const observed = selectedMissingness(base, profile)
  const cells = observed.kind === 'not-present' ? 0 : observed.cells
  const m = value.missingness
  if (cells > 0 && m.kind === 'not-present')
    return err('The selected columns contain missing cells. Choose a missing-data policy.')
  const confirmed =
    current.missingness.kind === 'imputation' &&
    current.missingness.method === 'structuralZero' &&
    current.missingness.confirmedStructuralZero &&
    current.variables.kind === 'selected' &&
    current.variables.columns.length === value.columns.length &&
    current.variables.columns.every((id) => value.columns.includes(id))
  if (m.kind === 'imputation' && m.method === 'structuralZero' && !confirmed)
    return err(
      'Confirm structural zeros in the preprocessing controls before proposing this policy.',
    )
  const missingness: PreprocessingDraft['missingness'] =
    cells === 0 || m.kind === 'not-present'
      ? { kind: 'not-present' }
      : m.kind === 'imputation'
        ? { ...m, cells, confirmedStructuralZero: confirmed }
        : { ...m, cells }
  const next = { ...base, missingness }
  const ready = readyPreprocessingRecipe(next)
  return ready.ok ? ok(next) : err(describeReadinessProblem(ready.error))
}

export function describePreprocessingSettings(
  value: PreprocessingSettings,
  profile: DatasetProfile,
): readonly ConfigurationSetting[] {
  const name = (id: string) => profile.columns.find((c) => c.id === id)?.name ?? id
  const s = value.sampling
  const m = value.missingness
  const interpretation = s.kind === 'regular-series' ? s.interpretation : undefined
  const missing =
    m.kind === 'not-present'
      ? 'No missing cells'
      : m.kind === 'unresolved'
        ? 'Not resolved'
        : m.kind === 'complete-interval'
          ? 'Complete interval'
          : m.kind === 'imputation'
            ? `${{ linearInterior: 'Interpolate interior gaps', forwardFill: 'Forward fill', structuralZero: 'Structural zeros' }[m.method]}; maximum gap ${m.maxGap}`
            : `Lag-aware exclusion; cutoff ${m.cutOff}; propagate through maximum lag: ${m.propagateThroughMaxLag ? 'yes' : 'no'}; roles: ${m.analysisExclusions.kind === 'ignore' ? 'ignore' : m.analysisExclusions.roles.join(', ')}`
  return [
    ...(interpretation
      ? [
          {
            key: 'interpretation',
            label: 'Time values',
            value:
              interpretation.kind === 'date-format'
                ? interpretation.format
                : {
                    'source-type': 'Use source type',
                    ordinal: 'Ordered periods',
                    timestamp: 'Timestamps',
                    'iso-week': 'ISO weeks',
                  }[interpretation.kind],
          },
        ]
      : []),
    {
      key: 'sampling',
      label: 'Rows represent',
      value:
        s.kind === 'cross-sectional'
          ? 'Independent observations'
          : s.kind === 'regular-panel'
            ? 'Panel data'
            : s.kind === 'regular-series'
              ? 'Regular time series'
              : 'Not configured',
    },
    ...('timeColumn' in s && s.timeColumn
      ? [{ key: 'time', label: 'Time column', value: name(s.timeColumn) }]
      : []),
    ...('unitColumn' in s && s.unitColumn
      ? [{ key: 'unit', label: 'Unit column', value: name(s.unitColumn) }]
      : []),
    ...('frequency' in s ? [{ key: 'frequency', label: 'Frequency', value: s.frequency }] : []),
    { key: 'columns', label: 'Analysis columns', value: value.columns.map(name).join(', ') },
    { key: 'missingness', label: 'Missing data', value: missing },
    {
      key: 'resampling',
      label: 'Calendar aggregation',
      value:
        value.resampling.kind === 'none'
          ? 'None'
          : `${value.resampling.targetFrequency}; ${value.resampling.incompleteBins} incomplete bins; ${value.resampling.aggregations.map((a) => `${name(a.column)}: ${a.aggregation}`).join(', ')}`,
    },
    {
      key: 'seasonal',
      label: 'Seasonal adjustment',
      value:
        value.seasonal.kind === 'none'
          ? 'None'
          : `STL: ${value.seasonal.columns.map(name).join(', ')}; robust: ${value.seasonal.robust}`,
    },
    {
      key: 'transforms',
      label: 'Series transformations',
      value:
        value.seriesTransforms
          .map((t) => `${name(t.column)}: ${describeSeriesTransform(t.transform)}`)
          .join('; ') || 'Levels',
    },
  ]
}
