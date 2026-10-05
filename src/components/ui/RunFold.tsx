import { type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { caption, iconControl, num } from './recipes'

/**
 * One recorded run in a chapter's history: a single line that folds open to the full record.
 * Rows sit in a hairline-divided list; the delete control lives on the line, revealed on hover and
 * always visible to a coarse pointer, and never toggles the fold.
 */
export function RunFold({
  title,
  figure,
  stamp,
  onDelete,
  deleteLabel,
  defaultOpen = false,
  children,
}: {
  readonly title: ReactNode
  readonly figure?: ReactNode
  readonly stamp: ReactNode
  readonly onDelete?: () => void
  readonly deleteLabel?: string
  readonly defaultOpen?: boolean
  readonly children: ReactNode
}) {
  return (
    <li>
      <details className="group/fold" open={defaultOpen}>
        <summary className="flex cursor-pointer list-none items-start gap-x-3 gap-y-1 px-3 py-1.5 transition-colors hover:bg-well [&::-webkit-details-marker]:hidden">
          <Icon
            name="expand_more"
            size={14}
            className="shrink-0 text-faint transition-transform duration-(--motion-fast) group-open/fold:rotate-180"
          />
          <span className={caption('min-w-0 flex-1 text-ink')}>{title}</span>
          {figure !== undefined && <span className={num('text-label text-ink')}>{figure}</span>}
          <span className={num('ml-auto shrink-0 text-label text-faint')}>{stamp}</span>
          {onDelete !== undefined && (
            <button
              type="button"
              className={iconControl(
                'danger',
                'h-7 w-7 opacity-0 transition-opacity group-hover/fold:opacity-100 focus-visible:opacity-100 pointer-coarse:opacity-100',
              )}
              aria-label={deleteLabel}
              title={deleteLabel}
              onClick={(event) => {
                event.preventDefault()
                event.stopPropagation()
                onDelete()
              }}
            >
              <Icon name="delete" size={13} />
            </button>
          )}
        </summary>
        <div className="px-3 pb-3">{children}</div>
      </details>
    </li>
  )
}
