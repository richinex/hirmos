import { useState } from 'react'
import { CodeHighlight } from '@/components/ui/CodeHighlight'
import { useAssistantBridge, useSqlCapability, type SqlProposal } from './applicationTools'
import { Checks, CompareAction, ProposalActions, ProposalCard } from './ProposalParts'

type State = 'pending' | 'loading' | 'loaded' | 'dismissed' | 'failed'
export function SqlProposalCard({ proposal }: { readonly proposal: SqlProposal }) {
  const bridge = useAssistantBridge()
  const capability = useSqlCapability()
  const [state, setState] = useState<State>('pending')
  const stale = capability?.revision !== proposal.revision
  return (
    <ProposalCard name="SQL proposal" title="Review SQL transformation">
      <p className="m-0">{proposal.summary}</p>
      <CodeHighlight
        code={proposal.statement}
        lang="sql"
        copyable
        copyLabel="Copy SQL"
        className="max-h-64 overflow-auto rounded-md bg-panel p-2.5 font-mono text-label"
      />
      <Checks checks={proposal.checks} />
      {state === 'pending' && !stale ? (
        <ProposalActions
          action="Load SQL for review"
          help="Loading replaces the open SQL draft, not your data. Review the full statement before running it. The result stays local; use the existing source control only after checking it."
          onAction={() => {
            setState('loading')
            void bridge.loadSql(proposal).then(
              (result) => setState(result.ok ? 'loaded' : 'failed'),
              () => setState('failed'),
            )
          }}
          onDismiss={() => setState('dismissed')}
        />
      ) : null}
      {state === 'loaded' ? <CompareAction>Loaded</CompareAction> : null}
      <p role="status" className="m-0 text-label text-muted">
        {state === 'loading'
          ? 'Loading draft…'
          : state === 'dismissed'
            ? 'Proposal dismissed.'
            : state === 'failed' || stale
              ? 'The workspace or draft changed. Ask for a new proposal.'
              : ''}
      </p>
    </ProposalCard>
  )
}
