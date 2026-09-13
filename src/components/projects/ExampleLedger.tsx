import { useState } from 'react'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { button, field, iconControl, num, table, td, th } from '@/components/ui/recipes'
import { EXAMPLE_COLLECTIONS, EXAMPLE_QUESTIONS, type ShippedExample } from '@/domain/example'
import { browseExamples, type ExampleSort } from '@/domain/exampleBrowser'
import type { SavedProjectHeader } from '@/domain/persistence'
import { OpenControl } from './OpenControl'

interface Props {
  readonly examples: readonly ShippedExample[]
  readonly saved: readonly SavedProjectHeader[]
  readonly formatSaved: (iso: string) => string
  readonly onOpen: (example: ShippedExample) => void
  readonly onReset: (example: ShippedExample) => void
  readonly onExport: (id: SavedProjectHeader['id']) => void
  readonly onDelete: (header: SavedProjectHeader) => void
}

/** A compact property table on desktop and a text-first list at narrow widths. */
export function ExampleLedger(props: Props) {
  const [query, setQuery] = useState('')
  const [shape, setShape] = useState<string | null>(null)
  const [sort, setSort] = useState<ExampleSort>('catalog')
  const shapes = [...new Set(props.examples.map(example => example.shape))]
  const examples = browseExamples(props.examples, { query, shape, sort })
  const groups = [
    ...EXAMPLE_COLLECTIONS.map(group => ({ key: group.id, title: group.title, note: group.purpose, items: examples.filter(example => example.collection === group.id) })),
    ...EXAMPLE_QUESTIONS.map(group => ({ key: group.id, title: group.title, note: group.when, items: examples.filter(example => example.collection === null && example.question === group.id) })),
  ].filter(group => group.items.length > 0)
  const clear = () => { setQuery(''); setShape(null); setSort('catalog') }

  return (
    <div className="example-browser @container/examples space-y-3" data-testid="example-browser">
      <div className="flex flex-wrap items-end gap-3">
        <label className="min-w-0 flex-1 basis-60">
          <span className="sr-only">Search examples</span>
          <input type="search" className={field()} placeholder="Search examples" value={query} onChange={event => setQuery(event.target.value)} />
        </label>
        <label className="min-w-0 flex-1 basis-44 @lg/examples:flex-none">
          <span className="sr-only">Data structure</span>
          <Select className={field()} value={shape ?? ''} onChange={event => {
            const value = event.target.value
            if (value === '' || shapes.includes(value)) setShape(value === '' ? null : value)
          }}>
            <option value="">All data structures</option>
            {shapes.map(value => <option key={value} value={value}>{value}</option>)}
          </Select>
        </label>
        <label className="min-w-0 flex-1 basis-40 @lg/examples:flex-none">
          <span className="sr-only">Sort examples</span>
          <Select className={field()} value={sort} onChange={event => {
            const value = event.target.value
            if (value === 'catalog' || value === 'name') setSort(value)
          }}>
            <option value="catalog">Catalog order</option>
            <option value="name">Name A–Z</option>
          </Select>
        </label>
      </div>
      {examples.length === 0 ? (
        <div className="py-6 text-body text-muted">
          <p className="m-0">No examples match these filters.</p>
          <button type="button" className={button('outline', 'mt-3')} onClick={clear}>Clear filters</button>
        </div>
      ) : (
        <>
          <div className="hidden overflow-x-auto @4xl/examples:block [--table-surface:var(--color-stage)]">
            <table className={table + ' example-table'} aria-label="Examples">
              <thead><tr>
                <th scope="col" className={th('w-[30%] px-2.5')}>Example</th>
                <th scope="col" className={th('w-[29%] px-2.5')}>Approach</th>
                <th scope="col" className={th('px-2.5')}>Data</th>
                <th scope="col" className={th('w-24 whitespace-normal px-2.5 text-right')}>Estimation runs</th>
                <th scope="col" className={th('px-2.5')}>Actions</th>
              </tr></thead>
              <tbody>{examples.map(example => {
                const copy = props.saved.find(entry => entry.id === example.id) ?? null
                return <tr key={example.id} className="border-b border-line hover:bg-well">
                  <td className={td('px-2.5 py-2.5')}>
                    <span className="block whitespace-normal font-medium text-ink">{example.name}</span>
                    <span className={num('mt-1 block whitespace-normal break-all text-label text-faint')}>{copy === null ? example.sourceName : `saved ${props.formatSaved(copy.savedAt)}`}</span>
                  </td>
                  <td className={td('whitespace-normal px-2.5 py-2.5 text-muted')}>{example.approach}</td>
                  <td className={td('px-2.5 py-2.5')}><ShapeLabel shape={example.shape} /><span className={num('mt-1 block whitespace-normal text-label text-faint')}>{example.size}</span></td>
                  <td className={td(num('px-2.5 py-2.5 text-right text-muted'))}>{copy === null ? example.estimationRuns : copy.estimationRuns}</td>
                  <td className={td('px-2.5 py-2.5')}><div className="flex items-center gap-1">
                    <OpenControl name={example.name} onOpen={() => props.onOpen(example)} />
                    <CopyControls {...props} example={example} copy={copy} />
                  </div></td>
                </tr>
              })}</tbody>
            </table>
          </div>
          <div className="space-y-5 @4xl/examples:hidden">
            {groups.map(group => <section key={group.key} aria-label={group.title}>
              <h4 className="mb-1 mt-0 text-body font-medium text-ink">{group.title}</h4>
              <p className="mb-2 mt-0 text-label text-faint">{group.note}</p>
              <ul className="m-0 list-none divide-y divide-line border-y border-line p-0">
                {group.items.map(example => {
                  const copy = props.saved.find(entry => entry.id === example.id) ?? null
                  const count = copy === null ? example.estimationRuns : copy.estimationRuns
                  return <li key={example.id} className="py-3">
                    <div className="flex items-start gap-3">
                      <div className="min-w-0 flex-1"><span className="block text-body font-medium text-ink">{example.name}</span><span className="mt-1 block text-label text-muted">{example.approach}</span></div>
                      <OpenControl name={example.name} onOpen={() => props.onOpen(example)} />
                    </div>
                    <div className="mt-2 flex flex-wrap items-center gap-x-2 gap-y-1"><ShapeLabel shape={example.shape} /><span className={num('text-label text-faint')}>{example.size} · {count} estimation {count === 1 ? 'run' : 'runs'}</span></div>
                    <div className="mt-1 flex flex-wrap items-center gap-2"><span className={num('min-w-0 flex-1 break-all text-label text-faint')}>{copy === null ? example.sourceName : `saved ${props.formatSaved(copy.savedAt)}`}</span><CopyControls {...props} example={example} copy={copy} /></div>
                  </li>
                })}
              </ul>
            </section>)}
          </div>
        </>
      )}
      <p role="status" className={num('m-0 text-label text-faint')}>{examples.length} of {props.examples.length} examples</p>
    </div>
  )
}

function ShapeLabel({ shape }: { readonly shape: string }) {
  return <span className="example-shape" data-shape={shape}>{shape}</span>
}

function CopyControls({ example, copy, onReset, onExport, onDelete }: {
  readonly example: ShippedExample
  readonly copy: SavedProjectHeader | null
} & Pick<Props, 'onReset' | 'onExport' | 'onDelete'>) {
  if (copy === null) return null
  return <>
    <button type="button" className={iconControl('quiet')} aria-label={`Reset ${example.name}`} title="Put the example back as shipped, discarding changes to this copy" onClick={() => onReset(example)}><Icon name="restart_alt" size={14} /></button>
    <button type="button" className={iconControl('quiet')} aria-label={`Export ${example.name}`} title="Export this copy as a bundle, without the source file" onClick={() => onExport(example.id)}><Icon name="download" size={14} /></button>
    <button type="button" className={iconControl('danger')} aria-label={`Delete ${example.name}`} title="Delete this copy" onClick={() => onDelete(copy)}><Icon name="delete" size={14} /></button>
  </>
}
