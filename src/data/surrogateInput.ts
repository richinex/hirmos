import type { DatasetProfile } from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'
import { err, type Result } from '@/domain/dop'
import { selectSurrogateSamples, surrogateSelectionColumns, surrogateSelectionSchema, type SurrogateSamples, type SurrogateSampleProblem, type SurrogateSelection } from '@/domain/surrogateSamples'

export type SurrogatePreparationProblem = SurrogateSampleProblem | { readonly kind: 'materialization-failed'; readonly detail: string }

/** Uses the pipeline's current source, not a globally complete-case prepared matrix. */
export async function prepareSurrogateSamples(source: SelectedSource, profile: DatasetProfile, selection: SurrogateSelection): Promise<Result<SurrogateSamples, SurrogatePreparationProblem>> {
  const parsed = surrogateSelectionSchema.safeParse(selection)
  if (!parsed.success) return err({ kind: 'invalid-selection', detail: parsed.error.issues.map(i => i.message).join(' ') })
  const { materializeNumericColumnsInWorker } = await import('./client')
  const matrix = await materializeNumericColumnsInWorker(source.file, profile, surrogateSelectionColumns(parsed.data),parsed.data.sample.kind==='categories'?parsed.data.sample:undefined)
  if (!matrix.ok) return err({ kind: 'materialization-failed', detail: 'detail' in matrix.error ? matrix.error.detail : `The source could not be read (${matrix.error.kind}).` })
  return selectSurrogateSamples(matrix.value, parsed.data, profile)
}
