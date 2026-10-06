import { Icon } from '@/components/Icon'
import { useClosePane } from '@/components/shell/WorkbenchLayout'
import { formatTime } from '@/lib/format/date'
import { caption, iconControl } from './recipes'

export interface RunPickerEntry<Id extends string> {
  readonly id: Id
  readonly title: string
  readonly createdAt: string
  /** Required when the picker offers deletion. */
  readonly deleteLabel?: string
}

/**
 * A bottom-pane list whose rows choose what the stage shows, newest first. On a phone, choosing
 * an entry closes the sheet so the stage is visible.
 */
export function RunPicker<Id extends string>({
  label,
  empty,
  runs,
  selected,
  onSelect,
  onDelete,
}: {
  readonly label: string
  readonly empty: string
  /** Oldest first, as recorded. */
  readonly runs: readonly RunPickerEntry<Id>[]
  readonly selected: Id | undefined
  readonly onSelect: (id: Id) => void
  /** Omitted when the entries cannot be deleted. */
  readonly onDelete?: (id: Id) => void
}) {
  const close = useClosePane()
  return (
    <ul className="m-0 list-none p-1 text-body" aria-label={label}>
      {runs.length === 0 && <li className="px-3 py-2 text-faint">{empty}</li>}
      {[...runs].reverse().map((run) => (
        <li
          key={run.id}
          className={`flex min-w-0 items-center gap-2 rounded-md px-2 ${selected === run.id ? 'bg-well' : ''}`}
        >
          <button
            type="button"
            className="flex min-h-10 min-w-0 flex-1 items-center gap-3 rounded-md text-left focus-visible:outline-2 focus-visible:outline-signal"
            aria-label={`${run.title}, ${formatTime(run.createdAt)}`}
            aria-pressed={selected === run.id}
            title={run.title}
            onClick={() => {
              onSelect(run.id)
              close()
            }}
          >
            <span className={caption('min-w-0 flex-1 truncate text-ink')}>{run.title}</span>
            <time className="shrink-0 text-label tabular-nums text-faint" dateTime={run.createdAt}>
              {formatTime(run.createdAt)}
            </time>
          </button>
          {onDelete !== undefined && (
            <button
              type="button"
              className={iconControl('danger')}
              aria-label={run.deleteLabel}
              title={run.deleteLabel}
              onClick={() => {
                close()
                onDelete(run.id)
              }}
            >
              <Icon name="delete" size={16} />
            </button>
          )}
        </li>
      ))}
    </ul>
  )
}
