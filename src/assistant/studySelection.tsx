import { useLayoutEffect, useRef, useState } from 'react'
import { useStore } from 'zustand'
import { z } from 'zod'
import type { IdentificationArtifact, StudySpecification } from '@/domain/study'
import { err, ok, type Result } from '@/domain/dop'
import { ProposalActions, ProposalCard, ProposalSettings } from './ProposalParts'
import { useAssistantBridge } from './applicationTools'

export interface StudyChoice {
  readonly identification: IdentificationArtifact['id']
  readonly study: StudySpecification['id']
  readonly question: string
  readonly treatment: string
  readonly outcome: string
  readonly adjustment: string
}
export interface StudySelectionInspection {
  readonly selected: IdentificationArtifact['id'] | null
  readonly choices: readonly StudyChoice[]
}
export interface StudySelectionCapability {
  readonly revision: string
  readonly inspection: StudySelectionInspection
  readonly select: (id: IdentificationArtifact['id']) => boolean
}
export interface StudySelectionProposal {
  readonly revision: string
  readonly choice: StudyChoice
  readonly reason: string
}
const schema = z
  .object({ identification: z.string().min(1), reason: z.string().trim().min(1).max(2000) })
  .strict()

export function proposeStudySelection(
  capability: StudySelectionCapability,
  raw: unknown,
): Result<StudySelectionProposal, string> {
  const parsed = schema.safeParse(raw)
  if (!parsed.success)
    return err('Supply an identification ID from the available studies and a reason.')
  const choice = capability.inspection.choices.find(
    (item) => item.identification === parsed.data.identification,
  )
  if (!choice) return err('This identified study is not available in Estimation.')
  if (choice.identification === capability.inspection.selected)
    return err(
      'This study is already selected. Inspect its current settings before requesting a run.',
    )
  return ok({ revision: capability.revision, choice, reason: parsed.data.reason })
}

export function useStudySelection() {
  const bridge = useAssistantBridge()
  return useStore(bridge.store, (state) => state.studySelection)
}

export function applyStudySelection(
  capability: StudySelectionCapability | null,
  proposal: StudySelectionProposal,
): Result<void, string> {
  if (!capability || capability.revision !== proposal.revision)
    return err('The available studies or settings changed.')
  const checked = proposeStudySelection(capability, {
    identification: proposal.choice.identification,
    reason: proposal.reason,
  })
  if (!checked.ok) return checked
  return capability.select(checked.value.choice.identification)
    ? ok(undefined)
    : err('The selection was not applied.')
}

export function useStudySelectionTool(
  binding: string,
  inspection: StudySelectionInspection | null,
  select: StudySelectionCapability['select'],
) {
  const bridge = useAssistantBridge()
  const latest = useRef({ inspection, select })
  useLayoutEffect(() => {
    latest.current = { inspection, select }
  })
  const description = JSON.stringify(inspection)
  useLayoutEffect(() => {
    const value = latest.current.inspection
    if (!value) return
    const capability = {
      revision: crypto.randomUUID(),
      inspection: value,
      select: (id: IdentificationArtifact['id']) => latest.current.select(id),
    }
    bridge.store.setState({ studySelection: capability })
    return () => {
      if (bridge.store.getState().studySelection === capability)
        bridge.store.setState({ studySelection: null })
    }
  }, [bridge, binding, description])
}

export function StudySelectionCard({ proposal }: { readonly proposal: StudySelectionProposal }) {
  const bridge = useAssistantBridge()
  const capability = useStudySelection()
  const [status, setStatus] = useState<'pending' | 'applied' | 'dismissed' | 'refused'>('pending')
  const stale = capability?.revision !== proposal.revision
  return (
    <ProposalCard name="Study selection proposal" title="Select study in Estimation">
      <p className="m-0">{proposal.choice.question}</p>
      <ProposalSettings
        settings={[
          { key: 'adjustment', label: 'Recorded adjustment', value: proposal.choice.adjustment },
        ]}
      />
      <p className="m-0">{proposal.reason}</p>
      {status === 'pending' && !stale ? (
        <ProposalActions
          action="Select study"
          help="This selects the saved study using Estimation’s controls. Review the resulting method and settings before approving a run."
          onAction={() => {
            const current = bridge.store.getState().studySelection
            setStatus(applyStudySelection(current, proposal).ok ? 'applied' : 'refused')
          }}
          onDismiss={() => setStatus('dismissed')}
        />
      ) : null}
      <p role="status" className="m-0 text-label text-muted">
        {status === 'applied'
          ? 'Study selected. No analysis was run.'
          : status === 'dismissed'
            ? 'Proposal dismissed.'
            : stale || status === 'refused'
              ? 'The available studies or settings changed. Ask for a new proposal.'
              : ''}
      </p>
    </ProposalCard>
  )
}
