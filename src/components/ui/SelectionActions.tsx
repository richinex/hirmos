import { Icon } from '@/components/Icon'
import { button } from '@/components/ui/recipes'

export function SelectionActions({ selectLabel, clearLabel, onSelectAll, onClear, compact = false }: {
  readonly selectLabel: string
  readonly clearLabel: string
  readonly onSelectAll: () => void
  readonly onClear: () => void
  readonly compact?: boolean
}) {
  return <div className="flex shrink-0 items-center gap-2">
    <button type="button" className={button('quiet')} aria-label={selectLabel} title={selectLabel} onClick={onSelectAll}>
      <Icon name="select_all" size={15} className={compact ? undefined : '@md/card:hidden'} />
      {!compact && <span className="hidden @md/card:inline">Select all</span>}
    </button>
    <button type="button" className={button('quiet')} aria-label={clearLabel} title={clearLabel} onClick={onClear}>
      <Icon name="deselect" size={15} className={compact ? undefined : '@md/card:hidden'} />
      {!compact && <span className="hidden @md/card:inline">Clear</span>}
    </button>
  </div>
}
