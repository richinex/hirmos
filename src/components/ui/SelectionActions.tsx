import { Icon } from '@/components/Icon'
import { button } from '@/components/ui/recipes'

/** Select-all and clear buttons. Their text labels show only when the enclosing pane is wide enough. */
export function SelectionActions({
  selectLabel,
  clearLabel,
  onSelectAll,
  onClear,
  size = 'md',
}: {
  readonly selectLabel: string
  readonly clearLabel: string
  readonly onSelectAll: () => void
  readonly onClear: () => void
  readonly size?: 'sm' | 'md'
}) {
  return (
    <div className={`flex shrink-0 items-center ${size === 'sm' ? 'gap-1' : 'gap-2'}`}>
      <button
        type="button"
        className={button('quiet', undefined, size)}
        aria-label={selectLabel}
        title={selectLabel}
        onClick={onSelectAll}
      >
        <Icon name="select_all" size={size === 'sm' ? 13 : 15} />
        <span className="hidden @sm:inline">Select all</span>
      </button>
      <button
        type="button"
        className={button('quiet', undefined, size)}
        aria-label={clearLabel}
        title={clearLabel}
        onClick={onClear}
      >
        <Icon name="deselect" size={size === 'sm' ? 13 : 15} />
        <span className="hidden @sm:inline">Clear</span>
      </button>
    </div>
  )
}
