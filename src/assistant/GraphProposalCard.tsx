import { useState } from 'react'
import { useWorkflow, useWorkflowStore } from '@/components/WorkflowProvider'
import { graphBinding, graphProposalEvent, type GraphProposal } from './graphProposal'
import { Checks, ProposalActions, ProposalCard } from './ProposalParts'

export function GraphProposalCard({ proposal }: { readonly proposal: GraphProposal }) {
  const store = useWorkflowStore()
  const binding = useWorkflow((state) => graphBinding(state.workflow))
  const [state, setState] = useState<'pending' | 'applied' | 'dismissed' | 'refused'>('pending')
  const stale = binding !== proposal.binding
  return (
    <ProposalCard
      name="Graph proposal"
      title={`${proposal.target.kind === 'create' ? 'Create' : 'Edit'} ${proposal.document.name}`}
    >
      <p className="m-0">{proposal.summary}</p>
      <ul className="m-0 grid gap-2 pl-5 text-body" aria-label="Proposed graph changes">
        {proposal.changes.map((change, index) => (
          <li key={index}>
            <span>{change.title}</span>
            {change.rationale ? <p className="m-0 text-label text-muted">{change.rationale}</p> : null}
          </li>
        ))}
      </ul>
      <Checks checks={proposal.checks} />
      {state === 'pending' && !stale ? (
        <ProposalActions
          action="Apply graph changes"
          help="Review these proposed assumptions. Applying saves graph revisions, not an identified study or an effect estimate. Existing studies retain their original graph revisions."
          onAction={() => {
            const event = graphProposalEvent(store.getState().workflow, proposal)
            if (!event.ok) {
              setState('refused')
              return
            }
            store.getState().dispatch(event.value)
            const after = store.getState().workflow
            setState(
              after.kind === 'profiled' &&
                after.dagDocuments.some(
                  (item) =>
                    item.id === proposal.document.id &&
                    item.current.id === proposal.document.current.id,
                )
                ? 'applied'
                : 'refused',
            )
          }}
          onDismiss={() => setState('dismissed')}
        />
      ) : null}
      <p role="status" className="m-0 text-label text-muted">
        {state === 'applied'
          ? 'Graph saved. Review its arrows and selections in the DAG workspace.'
          : state === 'dismissed'
            ? 'Proposal dismissed.'
            : stale || state === 'refused'
              ? 'The data or graphs changed. Ask for a new proposal.'
              : ''}
      </p>
    </ProposalCard>
  )
}
