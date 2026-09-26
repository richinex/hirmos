import { assertNever } from './dop'
import { initialPreprocessingDraft, readyPreprocessingRecipe, type PreparedDatasetArtifact, type PreprocessingDraft } from './preprocessing'
import type { DatasetProfileId } from './dataset'
import type { Workflow } from './workflow'

export interface PreprocessingSession {
  readonly profile: DatasetProfileId
  readonly draft: PreprocessingDraft
  readonly savedRecipe: string | null
}

/** Restore choices from the saved preparation, not from its realised row counts. */
export function preparedDraft(prepared: PreparedDatasetArtifact): PreprocessingDraft {
  const base = {
    frequencyOrigin: 'chosen' as const,
    variables: { kind: 'selected' as const, columns: prepared.columns },
    missingness: prepared.missingness,
    diagnosticTransform: { kind: 'levels' as const },
  }
  switch (prepared.kind) {
    case 'prepared-time-series': {
      const resampling = prepared.resampling
      const seasonal = prepared.seasonalAdjustment
      return {
        ...base,
        sampling: prepared.sampling,
        resampling: resampling.kind === 'none' ? resampling : {
          kind: 'daily-downsample', targetFrequency: resampling.targetFrequency,
          incompleteBins: resampling.incompleteBins, aggregations: resampling.aggregations,
        },
        seasonal: seasonal.kind === 'none' ? seasonal : {
          kind: 'stl', columns: seasonal.columns, robust: seasonal.robust,
        },
        seriesTransforms: prepared.seriesTransforms,
      }
    }
    case 'prepared-panel':
    case 'prepared-cross-section':
      return { ...base, sampling: prepared.sampling, resampling: { kind: 'none' }, seasonal: { kind: 'none' }, seriesTransforms: [] }
    default: return assertNever(prepared)
  }
}

/** Called at workflow transitions; ordinary events must not overwrite an edited draft. */
export function retainPreprocessing(current: PreprocessingSession | null, workflow: Workflow): PreprocessingSession | null {
  if (workflow.kind !== 'profiled') return null
  if (current?.profile === workflow.profile.id) return current
  const prepared = workflow.prepared
  const draft = prepared !== null && prepared.sourceProfile === workflow.profile.id
    ? preparedDraft(prepared)
    : initialPreprocessingDraft(workflow.profile)
  const recipe = readyPreprocessingRecipe(draft)
  return { profile: workflow.profile.id, draft, savedRecipe: recipe.ok ? JSON.stringify(recipe.value) : null }
}
