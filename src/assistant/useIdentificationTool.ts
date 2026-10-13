import { useWorkflowStore } from '@/components/WorkflowProvider'
import type { useJob } from '@/analysis/JobsProvider'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { DagDocument } from '@/domain/dag'
import {
  describeStudyDesignProblem,
  describeEstimand,
  readyStudySpecification,
  type StudyDesignDraft,
  type StudySpecification,
} from '@/domain/study'
import { useAnalysisRunTool } from './applicationTools'
import type { AssistantRunOutcome } from './analysisRun'
import { studyAdjustmentDesign } from '@/domain/studyAdjustment'
import { sameStudySpecification } from '@/domain/studyRecords'

export function identificationChoices(
  sets: readonly (readonly number[])[],
  names: readonly string[],
) {
  const noAdjustmentNeeded = sets.some((set) => set.length === 0)
  return {
    minimalSets: sets.map((set) => set.map((index) => names[index] ?? 'Unknown variable')),
    noAdjustmentNeeded,
    ...(noAdjustmentNeeded
      ? {
          meaning:
            'No adjustment is needed under the recorded graph assumptions. A larger valid set may still be recommended for precision.',
        }
      : {}),
  }
}

export function useIdentificationTool(options: {
  readonly prepared: PreparedDatasetArtifact
  readonly documents: readonly DagDocument[]
  readonly draft: StudyDesignDraft
  readonly studies: readonly StudySpecification[]
  readonly session: ReturnType<typeof useJob>
  readonly execute: () => Promise<void>
}) {
  const store = useWorkflowStore()
  const document = options.documents.find((item) => item.id === options.draft.dagDocument)
  const binding = JSON.stringify({
    prepared: options.prepared.id,
    draft: options.draft,
    revision: document?.current.id,
  })
  const ready = readyStudySpecification(
    options.draft,
    options.documents,
    options.prepared,
    options.studies,
  )
  const read = () => store.getState()
  const pendingReview = async (signal: AbortSignal): Promise<AssistantRunOutcome | null> => {
    const decision = read().adjustmentDecision
    if (
      !decision ||
      decision.study.preparedDataset !== options.prepared.id ||
      decision.study.dagDocument !== document?.id ||
      decision.study.dagRevision !== document.current.id
    )
      return null
    const { validateAdjustmentSets } = await import('@/analysis/client')
    const adjustment = await validateAdjustmentSets(studyAdjustmentDesign(decision.study))
    if (signal.aborted) return { kind: 'cancelled' }
    if (read().adjustmentDecision !== decision)
      return {
        kind: 'refused',
        reason:
          'The adjustment decision changed. Review Study design before requesting another analysis.',
      }
    return {
      kind: 'review-required',
      method: 'Effect identification',
      reason:
        'Choose and record an adjustment set in Study design. Review the available sets and the recommendation below; the study has not been saved yet.',
      summary: JSON.stringify({
        treatment: decision.study.treatment,
        outcome: decision.study.outcome,
        estimand: decision.study.estimand,
        nextStep:
          'Choose and record an adjustment set in Study design. The study is not saved yet.',
        saved: false,
        nodeNames: decision.study.graph.nodes.map((node) => node.name),
        adjustmentAnalysis: adjustment.ok
          ? { kind: 'ready', ...adjustment.value }
          : {
              kind: 'unavailable',
              reason:
                'The adjustment recommendation could not be computed. You can still review the identification evidence.',
            },
        adjustmentChoices:
          decision.evidence.result.kind === 'notIdentified'
            ? null
            : identificationChoices(
                decision.evidence.result.minimalSets,
                decision.study.graph.nodes.map((node) => node.name),
              ),
        evidence: decision.evidence,
      }),
    }
  }
  useAnalysisRunTool(
    'identification',
    binding,
    ready.ok
      ? {
          section: 'Study design',
          method: 'Effect identification',
          study: {
            subject: {
              kind: 'study-draft',
              document: ready.value.dagDocument,
              treatment: ready.value.treatment.name,
              outcome: ready.value.outcome.name,
            },
            question: describeEstimand(ready.value),
            settings: [{ label: 'Assignment', value: options.draft.assignment.description }],
          },
          specification: binding,
          sharedResult:
            'The identifying result, adjustment choices and graph revision. A choice may require review before the study is saved.',
        }
      : null,
    async (signal) => {
      if (signal.aborted) return { kind: 'cancelled' }
      const current = readyStudySpecification(
        options.draft,
        options.documents,
        options.prepared,
        options.studies,
      )
      if (!current.ok) return { kind: 'refused', reason: describeStudyDesignProblem(current.error) }
      if (!options.session.canStart())
        return { kind: 'refused', reason: 'Another analysis is running.' }
      const workflow = read().workflow
      const previous = new Set(
        workflow.kind === 'profiled' ? workflow.identifications.map((item) => item.id) : [],
      )
      const work = options.execute()
      const started = options.session.read()
      const cancel = () => {
        if (started.kind === 'running' && options.session.current(started.id))
          options.session.cancel()
      }
      signal.addEventListener('abort', cancel, { once: true })
      try {
        await work
        if (signal.aborted) return { kind: 'cancelled' }
        const after = read().workflow
        if (after.kind !== 'profiled' || after.prepared?.id !== options.prepared.id)
          return { kind: 'refused', reason: 'The prepared data changed.' }
        const saved = after.identifications.find((item) => !previous.has(item.id))
        if (saved)
          return {
            kind: 'completed',
            runId: saved.id,
            method: 'Effect identification',
            summary: JSON.stringify(saved),
          }
        const job = options.session.read()
        if (job.kind === 'failed') return { kind: 'failed', reason: job.detail }
        if (job.kind === 'cancelled') return { kind: 'cancelled' }
        const pending = await pendingReview(signal)
        if (pending) return pending
        const existing = after.identifications.find((record) => {
          const study = after.studies.find((item) => item.id === record.study)
          return study !== undefined && sameStudySpecification(study, current.value)
        })
        if (existing)
          return {
            kind: 'completed',
            runId: existing.id,
            method: 'Effect identification',
            summary: JSON.stringify({
              record: existing,
              reused: true,
              message: 'The identical identification is already saved. No duplicate was created.',
            }),
          }
        return {
          kind: 'refused',
          reason:
            'No identification was saved or offered for review. Check the Study design requirements.',
        }
      } finally {
        signal.removeEventListener('abort', cancel)
      }
    },
  )
}
