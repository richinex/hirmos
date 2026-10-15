import { z } from 'zod'
import type { DagDocument } from './dag'

export const dagInspectorLabels = {
  selection: 'Selection',
  evidence: 'Evidence',
  intervene: 'Intervene',
  swig: 'SWIG',
} as const

export const dagWorkspaceSettingsSchema = z
  .object({
    document: z
      .string()
      .min(1)
      .describe('The exact source DAG document ID, not its name or a saved SWIG run ID.'),
    inspector: z.enum(['selection', 'evidence', 'intervene', 'swig']),
  })
  .strict()

export function dagWorkspaceChoices(documents: readonly DagDocument[]) {
  return documents.map((document, index) => ({
    id: document.id,
    revision: document.current.id,
    label:
      documents.filter((other) => other.name === document.name).length > 1
        ? `${document.name} (DAG ${index + 1})`
        : document.name,
  }))
}

export interface GraphTarget {
  readonly document: string
  readonly revision: string
  readonly name: string
}
