import { useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { aalenCurveOption } from '@/charts/survival/regression'
import { useChartTheme } from '@/charts/theme'
import { Select } from '@/components/ui/Select'
import { button, caption, field, fieldLabel } from '@/components/ui/recipes'

type Curve = readonly (readonly [number, number, number, number])[]
type View = { readonly kind: 'all'; readonly query: string; readonly page: number } | { readonly kind: 'term'; readonly index: number }
const PAGE_SIZE = 12

/** Bound the number of chart instances without imposing a limit on fitted terms. */
export function AalenCoefficients({ names, curves }: { readonly names: readonly string[]; readonly curves: readonly Curve[] }) {
  const [view, setView] = useState<View>({ kind: 'all', query: '', page: 0 })
  const theme = useChartTheme()
  const terms = names.map((name, index) => ({ name, index }))
  const matches = view.kind === 'all' ? terms.filter(({ name }) => name.toLocaleLowerCase().includes(view.query.trim().toLocaleLowerCase())) : terms.filter(({ index }) => index === view.index)
  const page = view.kind === 'all' ? Math.min(view.page, Math.max(0, Math.ceil(matches.length / PAGE_SIZE) - 1)) : 0
  const visible = matches.slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE)

  return <section className="mt-4" aria-label="Cumulative coefficient plots">
    <label className="block"><span className={fieldLabel}>Coefficient to plot</span><Select className={field('text', 'mt-1')} value={view.kind === 'all' ? 'all' : String(view.index)} onChange={(event) => setView(event.target.value === 'all' ? { kind: 'all', query: '', page: 0 } : { kind: 'term', index: Number(event.target.value) })}>
      <option value="all">All coefficients</option>
      {terms.map(({ name, index }) => <option key={index} value={index}>{name}</option>)}
    </Select></label>
    {view.kind === 'all' && <label className="mt-3 block"><span className={fieldLabel}>Filter coefficients</span><input className={field('text', 'mt-1')} type="search" value={view.query} onChange={(event) => setView({ kind: 'all', query: event.target.value, page: 0 })} /></label>}
    <p className={caption('mt-2')}>Each plot uses the same follow-up period. Vertical scales differ by term. Shading and dashed lines show approximate pointwise 95% confidence bounds.</p>
    <div className="mt-3 grid gap-4" style={{ gridTemplateColumns: view.kind === 'all' ? 'repeat(auto-fit, minmax(min(100%, 280px), 1fr))' : 'minmax(0, 1fr)' }} data-testid="aalen-coefficient-grid">
      {visible.map(({ name, index }) => <div key={index} className="min-w-0" data-testid="aalen-coefficient-card">
        <h4 className="mb-1 break-words text-body font-medium">{name}</h4>
        <ExpandableChart className="h-[280px]" label={`${name} cumulative coefficient`} testId="aalen-coefficients" option={aalenCurveOption(name, curves[index]!, theme, index)} />
      </div>)}
    </div>
    {matches.length === 0 && <p className={caption('mt-3')} role="status">No coefficients match this filter.</p>}
    {view.kind === 'all' && matches.length > PAGE_SIZE && <div className="mt-3 flex flex-wrap items-center gap-3" aria-label="Coefficient pages">
      <button type="button" className={button('quiet')} disabled={page === 0} onClick={() => setView({ ...view, page: page - 1 })}>Previous coefficients</button>
      <span className={caption()} role="status">{page * PAGE_SIZE + 1}–{Math.min((page + 1) * PAGE_SIZE, matches.length)} of {matches.length}</span>
      <button type="button" className={button('quiet')} disabled={(page + 1) * PAGE_SIZE >= matches.length} onClick={() => setView({ ...view, page: page + 1 })}>Next coefficients</button>
    </div>}
  </section>
}
