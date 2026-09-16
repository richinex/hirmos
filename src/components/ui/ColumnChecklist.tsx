import { fieldHint, fieldLabel } from './recipes'
import { SelectionActions } from './SelectionActions'

/** Shared multi-column selection; reserved roles remain visible but cannot be selected. */
export function ColumnChecklist<Id extends string>({ title, help, columns, selected, reserved = [], onChange }: {
  readonly title: string
  readonly help: string
  readonly columns: readonly {readonly id: Id; readonly name: string}[]
  readonly selected: readonly Id[]
  readonly reserved?: readonly Id[]
  readonly onChange: (selected: readonly Id[]) => void
}) {
  return <div className="min-w-0"><div className="flex flex-wrap items-center justify-between gap-2"><span className={fieldLabel}>{title}</span><SelectionActions selectLabel={`Select all ${title.toLowerCase()}`} clearLabel={`Clear selected ${title.toLowerCase()}`} onSelectAll={()=>onChange(columns.filter(c=>!reserved.includes(c.id)).map(c=>c.id))} onClear={()=>onChange([])} /></div>
    <p className={`${fieldHint} mb-2 max-w-[65ch]`}>{help}</p><div className="flex flex-wrap gap-x-4 gap-y-1.5" role="group" aria-label={title}>{columns.map(c=><label key={c.id} className={`flex min-w-0 max-w-full items-start gap-2 text-body ${reserved.includes(c.id)?'text-faint':'text-ink'}`}><input className="mt-1 shrink-0" type="checkbox" disabled={reserved.includes(c.id)} checked={selected.includes(c.id)&&!reserved.includes(c.id)} onChange={e=>onChange(e.target.checked?[...selected,c.id]:selected.filter(id=>id!==c.id))} /><span className="[overflow-wrap:anywhere]">{c.name}</span></label>)}</div>
  </div>
}
