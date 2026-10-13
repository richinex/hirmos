import { useState } from 'react'
import { useWorkflow, useWorkflowStore } from '@/components/WorkflowProvider'
import { describeAssignmentKind } from '@/domain/study'
import { Checks, ProposalActions, ProposalCard, ProposalSettings } from './ProposalParts'
import { studyBinding, studyProposalEvent, type StudyProposal } from './studyProposal'

export function StudyProposalCard({ proposal }: { readonly proposal: StudyProposal }) {
  const store = useWorkflowStore()
  const binding = useWorkflow((state) => studyBinding(state.workflow))
  const [state, setState] = useState<'pending' | 'applied' | 'dismissed' | 'refused'>('pending')
  const stale = binding !== proposal.binding
  const { specification } = proposal
  return (
    <ProposalCard name="Study proposal" title="Review the causal question">
      <p className="m-0">{proposal.summary}</p>
      <p className="m-0">{proposal.question}</p>
      <ProposalSettings
        settings={[
          { key: 'graph', label: 'Graph', value: proposal.graphName },
          { key: 'treatment', label: 'Treatment', value: specification.treatment },
          { key: 'outcome', label: 'Outcome', value: specification.outcome },
          ...(specification.target.kind === 'conditional-partial-effect-per-row'
            ? [
                {
                  key: 'modifiers',
                  label: 'Effect modifiers',
                  value:
                    specification.target.modifiers.join(', ') ||
                    'None beyond the adjustment variables.',
                },
              ]
            : []),
          {
            key: 'assignment',
            label: 'Assignment',
            value: `${describeAssignmentKind(specification.assignment.kind)}: ${specification.assignment.description}`,
          },
          {
            key: 'consistency',
            label: 'Consistency',
            value: specification.consistencyRationale ?? 'No rationale supplied.',
          },
          {
            key: 'interference',
            label: 'No interference',
            value: specification.noInterferenceRationale ?? 'No rationale supplied.',
          },
        ]}
      />
      <Checks checks={proposal.checks} />
      {state === 'pending' && !stale ? (
        <ProposalActions
          action="Apply study draft"
          help="These are proposed design statements, not facts established by the data. Applying replaces the current study draft. Identification and estimation require separate runs; saved studies are unchanged."
          onAction={() => {
            const event = studyProposalEvent(store.getState().workflow, proposal)
            if (!event.ok) {
              setState('refused')
              return
            }
            store.getState().dispatch(event.value)
            const workflow = store.getState().workflow
            setState(
              workflow.kind === 'profiled' &&
                JSON.stringify(workflow.studyDraft) === JSON.stringify(proposal.draft)
                ? 'applied'
                : 'refused',
            )
          }}
          onDismiss={() => setState('dismissed')}
        />
      ) : null}
      <p role="status" className="m-0 text-label text-muted">
        {state === 'applied'
          ? 'Draft applied. Review it in Study design before identifying the effect.'
          : state === 'dismissed'
            ? 'Proposal dismissed.'
            : stale || state === 'refused'
              ? 'The data, graph or study draft changed. Ask for a new proposal.'
              : ''}
      </p>
    </ProposalCard>
  )
}
