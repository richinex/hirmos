import type { Job } from '@/analysis/jobs'
import { assertNever } from '@/domain/dop'
import { Alert } from './Alert'

export function JobNotice({ job }: { readonly job: Job }) {
  switch (job.kind) {
    case 'idle':
    case 'running':
      return null
    case 'failed':
      return (
        <Alert tone="danger" className="mt-2">
          <p className="m-0">{job.detail}</p>
        </Alert>
      )
    case 'cancelled':
      return (
        <Alert tone="info" className="mt-2">
          <p className="m-0">
            {job.action === 'checks'
              ? 'The model check was cancelled.'
              : 'The analysis was cancelled.'}
          </p>
        </Alert>
      )
    default:
      return assertNever(job)
  }
}
