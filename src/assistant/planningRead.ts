import type { Workflow } from '@/domain/workflow'

export function readPlanning(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  id: string,
  view: 'summary' | 'specification' | 'evidence',
) {
  if (id !== 'current' || workflow.powerPlanning === null)
    return {
      kind: 'refused' as const,
      reason:
        'No saved planning calculation matches this request. Planning stores one calculation; use id current when the project lists one.',
    }
  const record = workflow.powerPlanning
  const common = {
    kind: 'saved-calculation' as const,
    workspace: 'planning' as const,
    model: record.kind,
    view,
    details:
      'This is the planning calculation saved in this question’s project snapshot, not a historical run. It uses the specified design and assumptions, not the uploaded observations. A later calculation replaces it.',
  }
  if (view === 'specification') return { ...common, specification: record.specification }
  if (view === 'evidence') return { ...common, evidence: record.result }
  const result =
    record.result.kind === 'analytical'
      ? (({ curve: _curve, ...summary }) => summary)(record.result)
      : record.result
  return { ...common, specification: record.specification, result }
}
