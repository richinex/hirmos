import { z } from 'zod'
import { useMemo } from 'react'
import type { Workflow } from '@/domain/workflow'
import { err, ok } from '@/domain/dop'
import { useWorkspaceConfiguration } from './workspaceConfiguration'

const schema = z.object({ editor: z.enum(['sql', 'pipeline']) }).strict()
type Editor = z.infer<typeof schema>['editor']

export function useDataEditor(workflow: Workflow, closed: boolean, open: (editor: Editor) => void) {
  const source =
    workflow.kind === 'profiled' || workflow.kind === 'source-selected' ? workflow.source : null
  const recipe = source?.recipe
  const revision = useMemo(() => crypto.randomUUID(), [source])
  const editor: Editor = recipe?.kind === 'pipeline-derived' ? 'pipeline' : 'sql'
  const available =
    closed && source !== null && (workflow.kind === 'profiled' || recipe?.kind !== 'uploaded-file')
  useWorkspaceConfiguration<z.infer<typeof schema>>(
    'data-editor',
    JSON.stringify([
      workflow.kind,
      revision,
      source?.name,
      source?.lastModified,
      source?.bytes,
      workflow.kind === 'profiled' ? workflow.profile.id : recipe,
    ]),
    available
      ? {
          title: 'Data editor',
          action: 'open-editor',
          current: { editor },
          schema,
          columns: [],
          describe: (value) => [
            { key: 'editor', label: 'Editor', value: value.editor === 'sql' ? 'SQL' : 'Pipeline' },
          ],
          validate: (value) =>
            recipe?.kind !== 'uploaded-file' && value.editor !== editor
              ? err(
                  `This source was made with ${editor === 'sql' ? 'SQL' : 'Pipeline'}. Reopen that editor to preserve its recipe.`,
                )
              : ok(value),
          configure: (value) => open(value.editor),
        }
      : null,
  )
}
