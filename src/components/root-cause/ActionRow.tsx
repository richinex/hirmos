import { Orb } from '@/components/ui/Orb'
import { JobNotice } from '@/components/ui/JobNotice'
import { button } from '@/components/ui/recipes'
import type { Action, Job } from '@/analysis/jobs'
export type { Job } from '@/analysis/jobs'

export function ActionRow({ job, action, progress = 'visible', disabled = false, onRun, onCancel }: { readonly job: Job; readonly action: Action; readonly progress?: 'visible' | 'accessible'; readonly disabled?: boolean; readonly onRun: () => void; readonly onCancel: () => void }) {
  const active = job.kind !== 'idle' && job.action === action ? job : null
  return <div className="mt-8" data-testid={`${action}-actions`}>
    <div className="flex flex-wrap items-center gap-3">
      <button type="button" className={button('signal')} disabled={disabled || job.kind === 'running'} aria-busy={active?.kind === 'running'} onClick={onRun}>{action === 'checks' ? 'Check fitted model' : 'Run analysis'}</button>
      {active?.kind === 'running' && <>
        <Orb state="solving" aria-label={action === 'checks' ? 'Model checks running' : 'Causal model analysis running'} />
        <button type="button" className={button('quiet')} onClick={onCancel}>{action === 'checks' ? 'Cancel check' : 'Cancel run'}</button>
        <span role="status" className={progress === 'accessible' ? 'sr-only' : 'min-w-0 flex-1 truncate text-label text-muted'}>{active.stage}</span>
      </>}
    </div>
    {active !== null && <JobNotice job={active} />}
  </div>
}
