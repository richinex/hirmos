import { Icon } from '@/components/Icon'
import { cn } from '@/lib/utils'
import { iconControl } from './recipes'

/**
 * The arrow at a clipped end of a horizontal scroller, which pages it: a bare chevron over the fade, which
 * takes a pale surface under the pointer and moves two pixels toward the content out of view. It takes no
 * Tab stop: the keyboard reaches an item out of view by moving to it, and the scroller follows.
 */
export function ScrollArrow({
  end,
  size = 'md',
  label,
  title,
  onClick,
  className,
}: {
  readonly end: 'start' | 'end'
  /** `md` in a toolbar's 40px row, `sm` in a view switch's row of labels. */
  readonly size?: 'sm' | 'md'
  readonly label: string
  readonly title: string
  readonly onClick: () => void
  readonly className?: string
}) {
  return (
    <button
      type="button"
      className={iconControl(
        'quiet',
        cn('group text-muted', size === 'sm' ? 'h-5 w-5 rounded-md' : 'h-7 w-7', className),
      )}
      aria-label={label}
      title={title}
      tabIndex={-1}
      onClick={onClick}
    >
      <Icon
        name={end === 'end' ? 'chevron_right' : 'chevron_left'}
        size={size === 'sm' ? 16 : 20}
        className={cn(
          'transition-transform duration-(--motion-fast)',
          end === 'end' ? 'group-hover:translate-x-0.5' : 'group-hover:-translate-x-0.5',
        )}
      />
    </button>
  )
}
