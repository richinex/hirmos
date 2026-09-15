import { literatureOf, RequirementsFold } from '@/components/MethodCaveats'
import { fieldHint, num } from '@/components/ui/recipes'
import type { MethodDefinition } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'

export function TimeSeriesRequirements({ method, prepared, source }: {
  readonly method: MethodDefinition
  readonly prepared: Extract<PreparedDatasetArtifact, { kind: 'prepared-time-series' }>
  readonly source: string
}) {
  return <div className="space-y-6">
    <section><h3 className="mb-2 mt-1 text-body font-medium text-ink">{method.name}</h3><p className={`${fieldHint} mt-0`}>{method.summary}</p></section>
    <section aria-label="Method requirements">
      <RequirementsFold name="Requirements" open literature={literatureOf(method.caveats.flatMap((caveat) => caveat.sources))}>
        <ul className="m-0 mt-2 list-none space-y-3 p-0">{method.caveats.map((caveat) => <li key={caveat.id} className="text-body"><p className="m-0 text-ink">{caveat.requirement}</p><p className="m-0 font-serif text-faint">{caveat.consequenceIfUnmet}</p></li>)}</ul>
      </RequirementsFold>
    </section>
    <section><h3 className="m-0 text-body font-medium text-ink">Prepared data</h3><dl className="m-0 mt-2 grid grid-cols-[auto_minmax(0,1fr)] gap-x-3 gap-y-1 text-body"><dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{prepared.observations}</dd><dt className="text-faint">Columns</dt><dd className={num('m-0 text-ink')}>{prepared.columns.length}</dd><dt className="text-faint">Source</dt><dd className="m-0 break-words text-ink">{source}</dd></dl></section>
  </div>
}
