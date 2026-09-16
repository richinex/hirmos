import { Orb } from '@/components/ui/Orb'
import { button } from '@/components/ui/recipes'
type Action = 'analysis' | 'checks'
export type Job = { readonly kind: 'idle' } | { readonly kind: 'running'; readonly action: Action; readonly stage: string } | { readonly kind: 'failed'; readonly action: Action; readonly detail: string }

export function ActionRow({ job, action, disabled = false, onRun, onCancel }: { readonly job: Job; readonly action: Action; readonly disabled?: boolean; readonly onRun: () => void; readonly onCancel: () => void }) {
  const active = job.kind !== 'idle' && job.action === action ? job : null
  return <div className="mt-4" data-testid={`${action}-actions`}>
    <div className="flex flex-wrap items-center gap-3">
      <button type="button" className={button('signal')} disabled={disabled || job.kind === 'running'} aria-busy={active?.kind === 'running'} onClick={onRun}>{action === 'checks' ? 'Check fitted model' : 'Run analysis'}</button>
      {active?.kind === 'running' && <>
        <Orb state="solving" aria-label={action === 'checks' ? 'Model checks running' : 'Causal model analysis running'} />
        <button type="button" className={button('quiet')} onClick={onCancel}>{action === 'checks' ? 'Cancel check' : 'Cancel run'}</button>
        <span role="status" className="min-w-0 flex-1 truncate text-label text-muted" title={active.stage}>{active.stage}</span>
      </>}
    </div>
    {active?.kind === 'failed' && <p role="alert" className="mt-2 text-body text-warn">{active.detail}</p>}
  </div>
}
