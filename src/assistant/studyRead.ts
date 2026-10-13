import { z } from 'zod'
import type { Workflow } from '@/domain/workflow'
import { adjustmentSummary } from './adjustmentSummary'

const studyReadArgs = z
  .object({
    id: z.string().min(1).max(200),
    view: z.enum(['summary', 'assumptions', 'full']).default('summary'),
  })
  .strict()

export function readStudy(workflow: Workflow, raw: unknown) {
  const parsed = studyReadArgs.safeParse(raw)
  if (!parsed.success)
    return {
      kind: 'refused' as const,
      reason: 'Supply one exact study ID from the project summary.',
    }
  if (workflow.kind !== 'profiled')
    return { kind: 'refused' as const, reason: 'No prepared project is open.' }
  const study = workflow.studies.find((item) => item.id === parsed.data.id)
  if (!study)
    return { kind: 'refused' as const, reason: 'That study is not saved in this project.' }
  const document = workflow.dagDocuments.find((item) => item.id === study.dagDocument)
  const identifications = workflow.identifications.filter((item) => item.study === study.id)
  const assumptions = identifications.flatMap((record) =>
    'basis' in record.result
      ? record.result.basis
          .filter(
            (entry) => entry.kind === 'graph-assumption' || entry.kind === 'design-assumption',
          )
          .map((entry) => ({ ...entry, study: study.id, identification: record.id }))
      : [],
  )
  const common = {
    kind: 'study-record' as const,
    study: study.id,
    assumptions,
  }
  if (parsed.data.view === 'assumptions') return { ...common, view: 'assumptions' as const }
  if (parsed.data.view === 'summary')
    return {
      ...common,
      view: 'summary' as const,
      specification: {
        treatment: study.treatment.name,
        outcome: study.outcome.name,
        estimand: study.estimand,
        preparedDataset: study.preparedDataset,
        dagRevision: study.dagRevision,
      },
      identifications: identifications.map((record) => ({
        id: record.id,
        kind: record.result.kind,
        adjustment: adjustmentSummary(record.result),
      })),
      details:
        'Use view="assumptions" or view="full" for a focused or complete record. Save the result in a Python variable and print only needed fields; printed feedback is limited to 3,000 characters. Assumptions are recorded premises, not verified facts.',
    }
  return {
    ...common,
    view: 'full' as const,
    specification: study,
    identifications: identifications.map((record) => ({
      record,
      adjustment: adjustmentSummary(record.result),
    })),
    currentGraphRevision: document?.current.id ?? null,
    interpretation:
      'All records here belong to this study ID and its saved specification. The assumptions list contains the graph-assumption and design-assumption entries from the identification basis, unchanged and labelled by source. An empty list means no such basis entries are recorded, not that the analysis requires no assumptions. Assumptions are recorded premises, not verified facts. The saved graph revision is not replaced by the current graph.',
  }
}

export type StudyRead = ReturnType<typeof readStudy>
