import { expect, test } from '@playwright/test'

test('preprocessing proposals derive missingness and cannot invent columns or consent', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { preprocessingDraftFromSettings, preprocessingSettings, preprocessingSettingsSchema } =
      await import(new URL('/src/domain/preprocessingSettings.ts', location.href).href)
    const { stepPreprocessing } = await import(
      new URL('/src/domain/preprocessing.ts', location.href).href
    )
    const profile = {
      columns: [
        { id: 'y', name: 'Outcome', duckdbType: 'DOUBLE', nullCount: 2 },
        { id: 't', name: 'Period', duckdbType: 'INTEGER', nullCount: 0 },
        { id: 'unit', name: 'Unit', duckdbType: 'VARCHAR', nullCount: 0 },
      ],
    }
    const current = {
      sampling: { kind: 'unconfigured' },
      frequencyOrigin: 'assumed',
      variables: { kind: 'selected', columns: ['y'] },
      missingness: { kind: 'unresolved', cells: 2 },
      resampling: { kind: 'none' },
      seasonal: { kind: 'none' },
      seriesTransforms: [],
      diagnosticTransform: { kind: 'levels' },
    }
    const settings = {
      ...preprocessingSettings(current),
      sampling: {
        kind: 'regular-series',
        timeColumn: 't',
        frequency: 'monthly',
        interpretation: { kind: 'ordinal' },
      },
      missingness: { kind: 'imputation', method: 'linearInterior', maxGap: 3 },
    }
    const check = (value: unknown) => preprocessingDraftFromSettings(value, current, profile)
    const valid = check(settings)
    if (!valid.ok) throw new Error(valid.error)
    const next = stepPreprocessing(current, { type: 'settings-applied', draft: valid.value })
    return {
      cells: next.missingness.cells,
      method: next.missingness.method,
      original: current.sampling.kind,
      invented: check({ ...settings, columns: ['invented'] }).ok,
      unknownReason: check({ ...settings, columns: ['invented'] }).error,
      nameReason: check({ ...settings, columns: ['Outcome'] }).error,
      typeReason: check({ ...settings, columns: ['unit'] }).error,
      text: check({ ...settings, columns: ['unit'] }).ok,
      hiddenMissing: check({ ...settings, missingness: { kind: 'not-present' } }).ok,
      zero: check({
        ...settings,
        missingness: { kind: 'imputation', method: 'structuralZero', maxGap: 3 },
      }).ok,
      consentField: preprocessingSettingsSchema.safeParse({
        ...settings,
        missingness: {
          kind: 'imputation',
          method: 'structuralZero',
          maxGap: 3,
          confirmedStructuralZero: true,
        },
      }).success,
      panel: check({
        ...settings,
        sampling: {
          kind: 'regular-panel',
          unitColumn: 'unit',
          timeColumn: 't',
          frequency: 'monthly',
        },
      }).ok,
      resampling: check({
        ...settings,
        resampling: {
          kind: 'daily-downsample',
          targetFrequency: 'monthly',
          incompleteBins: 'keep',
          aggregations: [{ column: 'y', aggregation: 'mean' }],
        },
      }).ok,
    }
  })
  expect(result).toEqual({
    cells: 2,
    method: 'linearInterior',
    original: 'unconfigured',
    invented: false,
    unknownReason:
      'Column reference “invented” was not found. Use the column IDs listed in the workspace settings.',
    nameReason: '“Outcome” is a column name. Use its column ID “y” in the settings.',
    typeReason: '“Unit” has type VARCHAR. Analysis columns must be numeric.',
    text: false,
    hiddenMissing: false,
    zero: false,
    consentField: false,
    panel: false,
    resampling: false,
  })
})
