import { Icon } from '@/components/Icon'
import { button, iconControl, num, table, td, th, tr } from '@/components/ui/recipes'
import { EXAMPLE_COLLECTIONS, EXAMPLE_QUESTIONS, type ShippedExample } from '@/domain/example'
import type { SavedProjectHeader } from '@/domain/persistence'
import { ExampleGlyph } from './ExampleGlyph'

interface Group {
  readonly key: string
  readonly title: string
  readonly note: string
  readonly items: readonly ShippedExample[]
}

interface Props {
  readonly examples: readonly ShippedExample[]
  readonly saved: readonly SavedProjectHeader[]
  readonly formatSaved: (iso: string) => string
  readonly onOpen: (example: ShippedExample) => void
  readonly onReset: (example: ShippedExample) => void
  readonly onExport: (id: SavedProjectHeader['id']) => void
  readonly onDelete: (header: SavedProjectHeader) => void
}

const groupExamples = (examples: readonly ShippedExample[]): readonly Group[] => [
  ...EXAMPLE_COLLECTIONS.map((collection) => ({
    key: `collection:${collection.id}`, title: collection.title, note: collection.purpose,
    items: examples.filter((example) => example.collection === collection.id),
  })),
  ...EXAMPLE_QUESTIONS.map((question) => ({
    key: `question:${question.id}`, title: question.title, note: question.when,
    items: examples.filter((example) => example.collection === null && example.question === question.id),
  })),
].filter((group) => group.items.length > 0)

/**
 * The shipped examples, grouped by the collection they belong to and then by the question they answer.
 * Wide enough, they read as one ledger: the question as a spanning left column, then the glyph, the
 * name, the design, the data shape and size, and the estimates recorded. Narrower, each group becomes
 * a heading over a list, so a phone shows the name first and never scrolls sideways.
 */
export function ExampleLedger(props: Props) {
  const groups = groupExamples(props.examples)
  return (
    <div className="@container/examples">
      {/* Written out in full so Tailwind finds the classes: from 56rem the ledger's fixed columns fit with at most a small sideways scroll. */}
      <div className="hidden @4xl/examples:block"><Ledger {...props} groups={groups} /></div>
      <div className="@4xl/examples:hidden"><Sections {...props} groups={groups} /></div>
    </div>
  )
}

const stored = (saved: readonly SavedProjectHeader[], example: ShippedExample): SavedProjectHeader | null =>
  saved.find((entry) => entry.id === example.id) ?? null

/** The controls a stored copy carries beyond Open; as placeholders where no copy exists so table columns align. */
function CopyControls({ example, copy, placeholders, onReset, onExport, onDelete }: {
  readonly example: ShippedExample
  readonly copy: SavedProjectHeader | null
  readonly placeholders: boolean
} & Pick<Props, 'onReset' | 'onExport' | 'onDelete'>) {
  if (copy === null) {
    if (!placeholders) return null
    return (
      <>
        <span aria-hidden className={iconControl('quiet', 'invisible')} />
        <span aria-hidden className={iconControl('quiet', 'invisible')} />
        <span aria-hidden className={iconControl('danger', 'invisible')} />
      </>
    )
  }
  return (
    <>
      <button type="button" className={iconControl('quiet')} aria-label={`Reset ${example.name}`} title="Put the example back as shipped, discarding changes to this copy" onClick={() => onReset(example)}><Icon name="restart_alt" size={14} /></button>
      <button type="button" className={iconControl('quiet')} aria-label={`Export ${example.name}`} title="Export this copy as a bundle, without the source file" onClick={() => onExport(example.id)}><Icon name="download" size={14} /></button>
      <button type="button" className={iconControl('danger')} aria-label={`Delete ${example.name}`} title="Delete this copy" onClick={() => onDelete(copy)}><Icon name="delete" size={14} /></button>
    </>
  )
}

function Ledger({ groups, saved, formatSaved, onOpen, onReset, onExport, onDelete }: Props & { readonly groups: readonly Group[] }) {
  return (
    <div className="overflow-x-auto rounded-lg border border-hair">
      {/* Fixed columns that sum to the reading width: auto layout gave the longest wrapping text the least room. */}
      <table className={`${table} table-fixed`} aria-label="Examples">
        <thead>
          <tr>
            <th className={th('w-[160px] px-2.5')}>Question</th>
            <th className={th('w-14 px-2.5')}><span className="sr-only">Kind</span></th>
            <th className={th('px-2.5')}>Example</th>
            <th className={th('w-[160px] px-2.5')}>Approach</th>
            <th className={th('w-[124px] px-2.5')}>Data</th>
            <th className={th('w-[96px] whitespace-normal px-2.5 text-right')}>Estimation runs</th>
            <th className={th('w-[200px] px-2.5')}><span className="sr-only">Actions</span></th>
          </tr>
        </thead>
        <tbody>
          {groups.map((group) => group.items.map((example, index) => {
            const copy = stored(saved, example)
            return (
              <tr key={example.id} className={tr('static', 'border-t border-line')}>
                {index === 0 && (
                  <td rowSpan={group.items.length} className={td('whitespace-normal border-r border-line px-2.5 text-muted')}>
                    <span className="block font-medium text-ink">{group.title}</span>
                    <span className="mt-0.5 block text-label text-faint">{group.note}</span>
                  </td>
                )}
                <td className={td('w-14 px-2.5')}><ExampleGlyph kind={example.glyph} /></td>
                <td className={td('px-2.5')}>
                  <span className="block whitespace-normal text-ink">{example.name}</span>
                  <span className={num('block whitespace-normal break-all text-label text-faint')}>
                    {copy === null ? example.sourceName : `saved ${formatSaved(copy.savedAt)}`}
                  </span>
                </td>
                <td className={td('max-w-[28ch] whitespace-normal px-2.5 text-muted')}>{example.approach}</td>
                <td className={td(num('px-2.5 text-faint'))}>
                  <span className="block truncate">{example.shape}</span>
                  <span className="block whitespace-normal">{example.size}</span>
                </td>
                <td className={td(num('px-2.5 text-right text-faint'))}>{copy === null ? example.estimationRuns : copy.estimationRuns}</td>
                <td className={td('px-2.5')}>
                  <span className="flex items-center gap-1.5">
                    <button type="button" className={button('outline')} onClick={() => onOpen(example)}>Open</button>
                    <CopyControls example={example} copy={copy} placeholders onReset={onReset} onExport={onExport} onDelete={onDelete} />
                  </span>
                </td>
              </tr>
            )
          }))}
        </tbody>
      </table>
    </div>
  )
}

function Sections({ groups, saved, formatSaved, onOpen, onReset, onExport, onDelete }: Props & { readonly groups: readonly Group[] }) {
  const estimationRuns = (count: number) => `${count} estimation ${count === 1 ? 'run' : 'runs'}`
  return (
    <div className="flex flex-col gap-5">
      {groups.map((group) => (
        <section key={group.key} aria-label={group.title} className="flex flex-col gap-2">
          <div>
            <h4 className="m-0 text-body font-medium text-ink">{group.title}</h4>
            <p className="m-0 mt-0.5 text-label text-faint">{group.note}</p>
          </div>
          <ul className="m-0 list-none divide-y divide-line rounded-lg border border-hair bg-panel p-0">
            {group.items.map((example) => {
              const copy = stored(saved, example)
              const count = copy === null ? example.estimationRuns : copy.estimationRuns
              return (
                <li key={example.id} className="grid grid-cols-[56px_minmax(0,1fr)_auto] items-center gap-x-3 gap-y-1.5 px-3 py-2">
                  <ExampleGlyph kind={example.glyph} />
                  <div className="min-w-0">
                    <span className="block text-ink">{example.name}</span>
                    <span className="block text-label text-muted">{example.approach}</span>
                    <span className={num('block text-label text-faint')}>{example.shape} · {example.size}{count > 0 ? ` · ${estimationRuns(count)}` : ''}</span>
                  </div>
                  <button type="button" className={button('outline')} onClick={() => onOpen(example)}>Open</button>
                  {copy !== null && (
                    <span className="col-start-2 col-end-4 flex items-center gap-1.5">
                      <span className={num('mr-auto text-label text-faint')}>saved {formatSaved(copy.savedAt)}</span>
                      <CopyControls example={example} copy={copy} placeholders={false} onReset={onReset} onExport={onExport} onDelete={onDelete} />
                    </span>
                  )}
                </li>
              )
            })}
          </ul>
        </section>
      ))}
    </div>
  )
}
