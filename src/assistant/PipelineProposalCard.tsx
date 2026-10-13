import { useState } from 'react'
import { Icon } from '@/components/Icon'
import { CodeHighlight } from '@/components/ui/CodeHighlight'
import { label } from '@/components/ui/recipes'
import { blockIcon } from '@/components/data/pipeline/blockIcons'
import { summariseBlock } from '@/components/data/pipeline/blockSummary'
import { blockLabel, type PipelineNode } from '@/domain/pipeline'
import { cn } from '@/lib/utils'
import { useAssistantBridge, usePreparationCapability } from './applicationTools'
import { proposalFlow, type BlockChange, type FlowStep } from './pipelineDiff'
import type { PipelineProposal } from './preparation'
import { Checks, CompareAction, ProposalActions, ProposalCard } from './ProposalParts'

const CHANGE: Record<BlockChange | 'removed', { readonly word: string; readonly tone: string }> = {
  added: { word: 'added', tone: 'text-ok' },
  changed: { word: 'changed', tone: 'text-warn' },
  kept: { word: 'kept', tone: 'text-faint' },
  removed: { word: 'removed', tone: 'text-danger' },
}

/** The block's setting: derived columns and scripts as code, everything else in the canvas card's own words. */
function Setting({ node }: { readonly node: PipelineNode }) {
  const block = node.block
  if (block.kind === 'derive-columns' && block.columns.length > 0)
    return (
      <div className="flex flex-col gap-1">
        {block.columns.map((column) => (
          <div key={column.name} className="min-w-0 rounded-md bg-panel px-2 py-1">
            <span className="font-mono text-label text-ink">{column.name} =</span>
            <CodeHighlight code={column.expression} lang="sql" className="font-mono text-label" />
          </div>
        ))}
      </div>
    )
  if (block.kind === 'script')
    return (
      <CodeHighlight
        code={block.code}
        copyable
        className="max-h-40 overflow-auto rounded-md bg-panel px-2 py-1 font-mono text-label"
      />
    )
  return <span className="min-w-0 truncate text-label text-muted">{summariseBlock(block)}</span>
}

function Step({ step, last }: { readonly step: FlowStep; readonly last: boolean }) {
  const kind = step.node.block.kind
  return (
    <li className="relative grid grid-cols-[22px_minmax(0,1fr)] gap-x-2.5 pb-2.5 last:pb-0">
      {last ? null : (
        <span aria-hidden className="absolute bottom-0 left-[10.5px] top-6 w-px bg-signal/30" />
      )}
      <span
        className={cn(
          'grid size-[22px] place-items-center rounded-md border bg-panel',
          step.change === 'added' ? 'border-ok/50 text-ok' : 'border-hair text-muted',
        )}
      >
        <Icon name={blockIcon(kind)} size={14} />
      </span>
      <div className="flex min-w-0 flex-col gap-1 pt-px">
        <div className="flex items-baseline gap-2">
          <span className="text-body font-medium text-ink">{blockLabel(kind)}</span>
          {step.change === null ? null : (
            <span className={label(CHANGE[step.change].tone)}>{CHANGE[step.change].word}</span>
          )}
        </div>
        {step.inputs.length > 1 ? (
          <span className="text-label text-faint">
            from {step.inputs.map((input) => blockLabel(input.block.kind)).join(', ')}
          </span>
        ) : null}
        <Setting node={step.node} />
      </div>
    </li>
  )
}

export function PipelineProposalCard({ proposal }: { readonly proposal: PipelineProposal }) {
  const bridge = useAssistantBridge()
  const capability = usePreparationCapability()
  const [state, setState] = useState<'pending' | 'applied' | 'dismissed' | 'refused'>('pending')
  // Compared once, when the card appears: after Apply the open pipeline is the proposal itself.
  const [flow] = useState(() =>
    proposalFlow(
      proposal.graph,
      capability?.kind === 'pipeline' ? capability.inspection.graph : null,
    ),
  )
  const stale = capability?.revision !== proposal.revision
  return (
    <ProposalCard name="Pipeline proposal" title="Review pipeline transformation">
      <p className="m-0">{proposal.summary}</p>
      <ol aria-label="Proposed blocks and connections" className="m-0 flex list-none flex-col p-0">
        {flow.steps.map((step, i) => (
          <Step key={step.node.id} step={step} last={i === flow.steps.length - 1} />
        ))}
      </ol>
      {flow.removed.length === 0 ? null : (
        <ul className="m-0 flex list-none flex-col gap-1 p-0">
          {flow.removed.map((node) => (
            <li key={node.id} className="flex items-center gap-2 text-label">
              <Icon name={blockIcon(node.block.kind)} size={14} className="text-faint" />
              <span className="text-faint line-through">{blockLabel(node.block.kind)}</span>
              <span className={label(CHANGE.removed.tone)}>{CHANGE.removed.word}</span>
            </li>
          ))}
        </ul>
      )}
      <Checks checks={proposal.checks} />
      {state === 'pending' && !stale ? (
        <ProposalActions
          action="Apply and evaluate pipeline"
          help="Applying replaces the open pipeline and evaluates its blocks locally, including any SQL expressions or Python scripts. Review the proposed blocks before applying. The prepared data source is not replaced until you choose to use the output."
          onAction={() => setState(bridge.applyPipeline(proposal).ok ? 'applied' : 'refused')}
          onDismiss={() => setState('dismissed')}
        />
      ) : null}
      {state === 'applied' ? <CompareAction>Applied</CompareAction> : null}
      <p role="status" className="m-0 text-label text-muted">
        {state === 'dismissed'
          ? 'Proposal dismissed.'
          : state === 'refused' || (state === 'pending' && stale)
            ? 'The workspace changed. Ask for a new proposal.'
            : ''}
      </p>
    </ProposalCard>
  )
}
