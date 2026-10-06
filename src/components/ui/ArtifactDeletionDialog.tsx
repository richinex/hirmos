import { useJob } from '@/analysis/JobsProvider'
import { useWorkflow } from '@/components/WorkflowProvider'
import { assessArtifactDeletion, type DeletionTarget } from '@/domain/artifactLifecycle'
import { Dialog } from './Dialog'
import { formatTimestamp } from '@/lib/format/date'
import { button } from './recipes'

const NOUNS = {
  dag: 'DAG',
  study: 'study',
  'dag-check': 'graph check',
  'model-check': 'causal model check',
  'swig-graph': 'saved graph',
} as const satisfies Record<DeletionTarget['kind'], string>

const CONFIRM_LABELS = {
  dag: 'Delete DAG',
  study: 'Delete study',
  'dag-check': 'Delete check',
  'model-check': 'Delete check',
  'swig-graph': 'Delete graph',
} as const satisfies Record<DeletionTarget['kind'], string>

export function ArtifactDeletionDialog({
  target,
  onTarget,
  onClose,
}: {
  readonly target: DeletionTarget
  readonly onTarget: (target: DeletionTarget) => void
  readonly onClose: () => void
}) {
  const workflow = useWorkflow((state) => state.workflow)
  const dispatch = useWorkflow((state) => state.dispatch)
  const { blocked: running } = useJob('artifact-deletion')
  const decision = assessArtifactDeletion(workflow, target)
  return (
    <Dialog
      open
      title={
        decision.kind === 'missing'
          ? 'Record no longer available'
          : `Delete this ${NOUNS[target.kind]}?`
      }
      role="alertdialog"
      onClose={onClose}
    >
      {decision.kind !== 'missing' && (
        <p className="m-0 text-body text-ink">
          {decision.name}
          <time className="block text-label text-faint" dateTime={decision.createdAt}>
            {formatTimestamp(decision.createdAt)}
          </time>
        </p>
      )}
      {decision.kind === 'ready' && (
        <p className="text-body text-muted">{decision.consequence} This cannot be undone.</p>
      )}
      {decision.kind === 'missing' && (
        <p className="text-body text-muted">This record is no longer in the project.</p>
      )}
      {decision.kind === 'blocked' && (
        <>
          <p className="text-body text-muted">
            Saved records still use this {NOUNS[target.kind]}. Delete those records individually
            before deleting it.
          </p>
          <ul className="m-0 max-h-64 overflow-y-auto list-none divide-y divide-line p-0">
            {decision.references.map((reference) => (
              <li key={reference.id} className="py-2 text-body">
                <span className="text-ink">{reference.label}</span>
                <time className="block text-label text-faint" dateTime={reference.createdAt}>
                  {formatTimestamp(reference.createdAt)}
                </time>
                <span className="block text-muted">{reference.location}</span>
                {reference.removable !== undefined && (
                  <button
                    type="button"
                    className={button('outline', 'mt-1')}
                    disabled={running}
                    onClick={() => {
                      if (reference.removable !== undefined) onTarget(reference.removable)
                    }}
                  >
                    Review deletion of this {NOUNS[reference.removable.kind]}
                  </button>
                )}
              </li>
            ))}
          </ul>
        </>
      )}
      {running && (
        <p role="status" className="text-body text-muted">
          Wait for the current run to finish, or cancel it, before deleting a record.
        </p>
      )}
      <div className="mt-4 flex justify-end gap-2">
        <button type="button" autoFocus className={button('outline')} onClick={onClose}>
          {decision.kind === 'ready' ? 'Cancel' : 'Close'}
        </button>
        {decision.kind === 'ready' && (
          <button
            type="button"
            disabled={running}
            className={button('danger')}
            onClick={() => {
              if (!running) {
                dispatch({ type: 'artifact-deletion-requested', target })
                onClose()
              }
            }}
          >
            {CONFIRM_LABELS[target.kind]}
          </button>
        )}
      </div>
    </Dialog>
  )
}
