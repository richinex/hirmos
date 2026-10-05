import { Icon } from '@/components/Icon'
import { button } from '@/components/ui/recipes'

/** Select-all and clear buttons. Their text labels show only when the enclosing pane is wide enough. */
export function SelectionActions({
  selectLabel,
  clearLabel,
  onSelectAll,
  onClear,
}: {
  readonly selectLabel: string
  readonly clearLabel: string
  readonly onSelectAll: () => void
  readonly onClear: () => void
}) {
  return (
    <div className="flex shrink-0 items-center gap-2">
      <button
        type="button"
        className={button('quiet')}
        aria-label={selectLabel}
        title={selectLabel}
        onClick={onSelectAll}
      >
        <Icon name="select_all" size={15} />
        <span className="hidden @sm:inline">Select all</span>
      </button>
      <button
        type="button"
        className={button('quiet')}
        aria-label={clearLabel}
        title={clearLabel}
        onClick={onClear}
      >
        <Icon name="deselect" size={15} />
        <span className="hidden @sm:inline">Clear</span>
      </button>
    </div>
  )
}
