import { z } from 'zod'
import { assertNever, err, ok, type Result } from './dop'
import type { DagDocument } from './dag'
import type { PreparedDatasetArtifact } from './preprocessing'
import { matchesRootCauseGraph, selectedRootCauseGraph, type RootCauseGraphProblem } from './rootCause'
import { rootCauseRunSchema, rootCauseCheckRecordSchema, type RootCauseWorkspace } from './rootCauseAnalysis'
import { gcmEffectsRunSchema } from './gcmEffects'
import { gcmInfluenceRunSchema } from './gcmInfluence'

const recordSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('run'), record: rootCauseRunSchema }).strict(),
  z.object({ kind: z.literal('effects'), record: gcmEffectsRunSchema }).strict(),
  z.object({ kind: z.literal('influence'), record: gcmInfluenceRunSchema }).strict(),
  z.object({ kind: z.literal('checks'), record: rootCauseCheckRecordSchema }).strict(),
])

const bound: unique symbol = Symbol('BoundRootCauseRecord')
type BoundRecord = z.infer<typeof recordSchema> & { readonly [bound]: true }

export type RootCauseRecordProblem =
  | { readonly kind: 'no-preparation' }
  | { readonly kind: 'invalid-record' }
  | { readonly kind: 'graph'; readonly problem: RootCauseGraphProblem }
  | { readonly kind: 'model-mismatch' }

/** Resolve against the current dataset and retained revisions when the result arrives. */
export function bindRootCauseRecord(
  value: unknown,
  documents: readonly DagDocument[],
  prepared: PreparedDatasetArtifact | null,
): Result<BoundRecord, RootCauseRecordProblem> {
  if (prepared === null) return err({ kind: 'no-preparation' })
  const parsed = recordSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-record' })

  const { record } = parsed.data
  const graph = selectedRootCauseGraph(record.graph, documents, prepared)
  if (!graph.ok) return err({ kind: 'graph', problem: graph.error })
  if (!matchesRootCauseGraph(graph.value, record.model)) return err({ kind: 'model-mismatch' })

  return ok({ ...parsed.data, [bound]: true })
}

/** Only a parsed record bound to the current dataset can enter the history. */
export function appendRootCauseRecord(workspace: RootCauseWorkspace, value: BoundRecord): RootCauseWorkspace {
  switch (value.kind) {
    case 'run': return { ...workspace, runs: [...workspace.runs, value.record] }
    case 'effects': return { ...workspace, effects: [...workspace.effects, value.record] }
    case 'influence': return { ...workspace, influences: [...workspace.influences, value.record] }
    case 'checks': return { ...workspace, checks: [...workspace.checks, value.record] }
    default: return assertNever(value)
  }
}
