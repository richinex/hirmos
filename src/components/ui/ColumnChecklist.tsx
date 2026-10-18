import { fieldHint, fieldLabel } from './recipes'
import { SelectionActions } from './SelectionActions'

/** Shared multi-column selection; reserved roles remain visible but cannot be selected. */
export function ColumnChecklist<Id extends string>({
  title,
  help,
  columns,
  selected,
  reserved = [],
  onChange,
  width = 'max-w-3xl',
}: {
  readonly title: string
  readonly help: string
  readonly columns: readonly { readonly id: Id; readonly name: string }[]
  readonly selected: readonly Id[]
  readonly reserved?: readonly Id[]
  readonly onChange: (selected: readonly Id[]) => void
  /** Match the controls above it, so the actions end where they end. */
  readonly width?: 'max-w-md' | 'max-w-2xl' | 'max-w-3xl'
}) {
  const chosen = new Set(selected)
  const locked = new Set(reserved)
  return (
    <div className={`min-w-0 ${width}`}>
      <div className="mb-1 flex flex-wrap items-center justify-between gap-x-3 gap-y-1">
        <span className={fieldLabel}>{title}</span>
        <SelectionActions
          selectLabel={`Select all ${title.toLowerCase()}`}
          clearLabel={`Clear selected ${title.toLowerCase()}`}
          onSelectAll={() => onChange(columns.filter((c) => !locked.has(c.id)).map((c) => c.id))}
          onClear={() => onChange([])}
          size="sm"
        />
      </div>
      <p className={`${fieldHint} mb-2 max-w-[65ch]`}>{help}</p>
      <div
        className="panel-scroll flex max-h-64 flex-wrap content-start gap-x-4 gap-y-1.5 overflow-y-auto p-0.5"
        role="group"
        aria-label={title}
      >
        {columns.map((c) => (
          <label
            key={c.id}
            className={`flex min-w-0 max-w-full items-start gap-2 text-body ${locked.has(c.id) ? 'text-faint' : 'text-ink'}`}
          >
            <input
              className="mt-1 shrink-0"
              type="checkbox"
              disabled={locked.has(c.id)}
              checked={chosen.has(c.id) && !locked.has(c.id)}
              onChange={(e) =>
                onChange(
                  e.target.checked ? [...selected, c.id] : selected.filter((id) => id !== c.id),
                )
              }
            />
            <span className="[overflow-wrap:anywhere]">{c.name}</span>
          </label>
        ))}
      </div>
    </div>
  )
}
