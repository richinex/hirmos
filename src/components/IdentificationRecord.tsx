import { Icon } from '@/components/Icon'
import { label, num } from '@/components/ui/recipes'
import { assertNever } from '@/domain/dop'
import { describeIdentificationFailure, type Identification, type IdentificationBasisEntry } from '@/domain/study'
import { cn } from '@/lib/utils'

/**
 * The identification record as the inspector shows it: three summary lines a reader can check without
 * expanding anything, then the full record behind one disclosure, split into what the user assumed and
 * what the graph derived. Every entry stays verbatim in the artifact and the Results manifest; this
 * view only groups and counts.
 */

type PathEntry = Extract<IdentificationBasisEntry, { kind: 'backdoor-path' }>

const ROW_LABEL: Record<string, string> = {
  'assignment-mechanism': 'Assignment',
  'graph-theory': 'Graph',
  'no-unmeasured-confounding': 'No unmeasured confounding',
  consistency: 'Consistency',
  'no-interference': 'No interference',
  'backdoor-criterion': 'Back-door criterion',
  'lags-collapsed': 'Lagged arrows',
}

const rowLabel = (entry: IdentificationBasisEntry): string => ROW_LABEL[entry.id] ?? (entry.id.startsWith('kept-out-') ? 'Left out' : entry.kind.replace('-', ' '))

const isPath = (entry: IdentificationBasisEntry): entry is PathEntry => entry.kind === 'backdoor-path'

const isOwned = (entry: IdentificationBasisEntry): boolean => {
  switch (entry.kind) {
    case 'graph-assumption':
    case 'design-record':
    case 'design-assumption':
      return true
    case 'graph-result':
    case 'backdoor-path':
    case 'qualification':
      return false
    default:
      return assertNever(entry)
  }
}

const PATHS_SHOWN = 8

function PathRow({ entry }: { readonly entry: PathEntry }) {
  return (
    <li title={entry.statement} className="grid grid-cols-[minmax(0,1fr)_auto] items-baseline gap-x-3 px-2 py-1">
      <span className="min-w-0 text-muted">
        {entry.nodes.map((node, index) => (
          <span key={`${node}-${index}`}>
            {index > 0 && <span className="text-faint"> {entry.arrows[index - 1] ?? '–'} </span>}
            <span className={entry.blockedAt.includes(node) ? 'text-ink' : undefined}>{node}</span>
          </span>
        ))}
      </span>
      <span className={num('whitespace-nowrap text-right text-ink')}>
        <span className="sr-only">{entry.closure === 'collider' ? 'blocked at collider ' : 'blocked at '}</span>{entry.blockedAt.join(', ')}
      </span>
    </li>
  )
}

function PathTable({ paths }: { readonly paths: readonly PathEntry[] }) {
  const shown = paths.slice(0, PATHS_SHOWN)
  const rest = paths.slice(PATHS_SHOWN)
  return (
    <div className="border-y border-hair">
      <div className="flex items-baseline justify-between gap-2 border-b border-hair px-2 py-1">
        <span className={label('text-faint')}>Back-door paths</span>
        <span className={num('text-label text-faint')}>{paths.length}</span>
      </div>
      <ol className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Back-door paths">
        {shown.map((entry) => <PathRow key={entry.id} entry={entry} />)}
      </ol>
      {rest.length > 0 && (
        <details className="group border-t border-hair">
          <summary className="flex cursor-pointer list-none items-center gap-1.5 px-2 py-1 text-body text-muted hover:text-ink [&::-webkit-details-marker]:hidden">
            <Icon name="expand_more" size={14} className="transition-transform duration-150 group-open:rotate-180" />
            Show all {paths.length} paths
          </summary>
          <ol className="m-0 list-none divide-y divide-hair border-t border-hair p-0 text-body">
            {rest.map((entry) => <PathRow key={entry.id} entry={entry} />)}
          </ol>
        </details>
      )}
    </div>
  )
}

function OwnedRow({ entry }: { readonly entry: IdentificationBasisEntry }) {
  const rationale = entry.kind === 'design-assumption' ? entry.rationale : undefined
  return (
    <li className="border-l border-edge pl-2.5">
      <span className={label('text-faint')}>{rowLabel(entry)}</span>
      <p className="mb-0 mt-0.5 text-body text-muted">{entry.statement}</p>
      {rationale === undefined ? null : rationale === null
        ? <p className="mb-0 mt-1 flex items-center gap-1.5 text-body text-warn"><Icon name="warning" size={14} /> Rationale not recorded</p>
        : <p className="mb-0 mt-1 text-body text-muted"><span className="text-faint">Rationale</span> {rationale}</p>}
    </li>
  )
}

function DerivedRow({ entry }: { readonly entry: IdentificationBasisEntry }) {
  const caution = entry.kind === 'qualification'
  return (
    <li>
      <span className={label(caution ? 'text-warn' : 'text-faint')}>{rowLabel(entry)}</span>
      <p className="mb-0 mt-0.5 text-body text-muted">{entry.statement}</p>
    </li>
  )
}

const plural = (count: number, noun: string): string => `${count} ${noun}${count === 1 ? '' : 's'}`

export function IdentificationRecord({ identification }: { readonly identification: Identification }) {
  if (identification.kind === 'backdoor-not-identified') {
    return (
      <section aria-label="Identification record" className="mb-3 border-t border-hair pt-3">
        <span className="block text-body font-medium text-ink">Identification record</span>
        <p className="mb-1 mt-1 flex items-center gap-2 text-body text-ink"><Icon name="block" size={16} className="text-muted" /> No measured back-door adjustment set</p>
        <p className="mb-1 mt-1 text-body text-muted">Front-door, instrumental-variable and other identification strategies were not assessed.</p>
        <ul className="m-0 list-disc pl-4 text-body text-muted">
          {identification.reasons.map((reason) => <li key={reason.kind + describeIdentificationFailure(reason)}>{describeIdentificationFailure(reason)}</li>)}
        </ul>
      </section>
    )
  }
  const { basis, adjustment } = identification
  const adjustmentSet = adjustment.variables
  const paths = basis.filter(isPath)
  const owned = basis.filter(isOwned)
  const derived = basis.filter((entry) => !isOwned(entry) && !isPath(entry) && entry.kind !== 'qualification')
  const qualification = basis.find((entry) => entry.kind === 'qualification')
  const assumptions = basis.filter((entry) => entry.kind === 'graph-assumption' || entry.kind === 'design-assumption')
  const missing = basis.filter((entry) => entry.kind === 'design-assumption' && entry.rationale === null).length
  const setNames = adjustmentSet.map((variable) => variable.name)
  const closers = [...new Set(paths.filter((entry) => entry.closure === 'adjustment').flatMap((entry) => entry.blockedAt))]
  const closersAreTheSet = closers.length === setNames.length && closers.every((name) => setNames.includes(name))
  const pathSummary = paths.length === 0
    ? 'No open back-door path · no adjustment needed'
    : `${plural(paths.length, 'back-door path')} · ${closersAreTheSet ? `blocked at ${closers.join(', ')}` : 'all blocked'}`

  return (
    <section aria-label="Identification record" className="mb-3 border-t border-hair pt-3">
      <span className="block text-body font-medium text-ink">Identification record</span>
      <ul className="m-0 mt-1 list-none space-y-0.5 p-0 text-body">
        <li className="flex items-center gap-2 text-ink"><Icon name="check_circle" size={16} className="text-ok" /> {pathSummary}</li>
        <li className={cn('flex items-center gap-2', missing > 0 ? 'text-warn' : 'text-muted')}>
          {missing > 0 && <Icon name="warning" size={16} />}
          {plural(assumptions.length, 'assumption')} recorded · {missing > 0 ? `${missing} without rationale` : 'all with rationale'}
        </li>
        {qualification !== undefined && <li className="flex items-center gap-2 text-warn"><Icon name="warning" size={16} /> {qualification.statement.split(';')[0]}</li>}
      </ul>
      <details className="group mt-2">
        <summary className="flex cursor-pointer list-none items-center gap-1.5 text-body text-muted hover:text-ink [&::-webkit-details-marker]:hidden">
          <Icon name="expand_more" size={14} className="transition-transform duration-150 group-open:rotate-180" />
          Show the full record · {plural(basis.length, 'entry').replace('entrys', 'entries')}
        </summary>
        <div className="mt-2 space-y-3">
          <div>
            <span className={label('block text-muted')}>Assumed by you</span>
            <ul className="m-0 mt-1 list-none space-y-2 p-0">{owned.map((entry) => <OwnedRow key={entry.id} entry={entry} />)}</ul>
          </div>
          <div>
            <span className={label('block text-muted')}>Derived from the graph</span>
            <ul className="m-0 mt-1 list-none space-y-2 p-0">{derived.map((entry) => <DerivedRow key={entry.id} entry={entry} />)}</ul>
            {paths.length > 0 && <div className="mt-2"><PathTable paths={paths} /></div>}
            {qualification !== undefined && <ul className="m-0 mt-2 list-none p-0"><DerivedRow entry={qualification} /></ul>}
          </div>
        </div>
      </details>
    </section>
  )
}
