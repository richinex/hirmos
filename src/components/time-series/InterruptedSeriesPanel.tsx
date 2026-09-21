import type { ReactNode } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import { useTimeSeriesDraft } from './useTimeSeriesDraft'
import type { InterruptedDraft, TimeSeriesEvent } from '@/domain/timeSeriesDraft'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { newTimeSeriesRunId, parseTimeSeriesRun } from '@/domain/timeSeries'
import { impactForKernel, rowIndex, rowNumber, type DeclaredImpact, type InterruptedModel, type InterruptedSeasonal } from '@/domain/interruptedSeries'
import { isNumericDuckDbType, type ColumnId } from '@/domain/dataset'
import { isNonEmpty } from '@/domain/dop'
import { TIME_SERIES_METHODS } from '@/domain/methods'
import { seasonalPeriodOf } from '@/domain/seasonal'
import { effectiveFrequency } from '@/domain/resampling'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Orb } from '@/components/ui/Orb'
import { button, field, fieldLabel, fieldHint, panel } from '@/components/ui/recipes'
import { useRunActivity } from '@/lib/useRunActivity'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { TimeSeriesHeading, type TimeSeriesPanelProps } from './TimeSeriesPanel'
import { TimeSeriesHistory } from './TimeSeriesHistory'
import { TimeSeriesRequirements } from './TimeSeriesRequirements'
import { TimeSeriesRunResult } from './TimeSeriesRunResult'

const integer = (text: string): number | null => {
  const trimmed = text.trim()
  if (trimmed === '') return null
  const value = Number(trimmed)
  return Number.isInteger(value) ? value : null
}

export function InterruptedSeriesPanel(props: TimeSeriesPanelProps & { readonly selector: ReactNode }) {
  const columns = props.profile.columns.filter((c) => props.prepared.columns.includes(c.id) && isNumericDuckDbType(c.duckdbType))
  const draft = useTimeSeriesDraft(props.prepared.id, (state) => state.interrupted)
  const change = useWorkflow((state) => state.changeTimeSeries)
  const set = <K extends keyof InterruptedDraft>(field: K, value: InterruptedDraft[K]) => change(props.prepared.id, { type: 'interrupted', field, value } as TimeSeriesEvent)
  const session = useJob('time-series:interrupted')
  const { job } = session
  useRunActivity(props.onActivity, job.kind === 'running' ? { label: 'Interrupted series', progress: null } : null)
  const runs = props.runs.filter((run) => run.kind === 'interrupted-series')
  const rows = props.prepared.observations
  const period = seasonalPeriodOf(effectiveFrequency(props.prepared.sampling.frequency, props.prepared.resampling))
  const typedRow = integer(draft.interventionRow)
  const lag = integer(draft.lag) ?? 0
  const pairs = period === null ? 0 : integer(draft.harmonicPairs) ?? 0
  const model = draft.model
  const count = model.kind === 'count'
  // Every row here is a screen row, numbered from 1; the kernel's index is taken once, at the call.
  const interventionRow = typedRow === null ? null : rowNumber(typedRow)
  const until = draft.impact.kind === 'temporaryLevel' ? integer(draft.impact.until) : null
  const impact: DeclaredImpact | null = draft.impact.kind === 'temporaryLevel' ? (until === null ? null : { kind: 'temporaryLevel', until: rowNumber(until) }) : draft.impact
  const seasonal: InterruptedSeasonal = pairs > 0 && period !== null ? { kind: 'harmonic', pairs, period } : { kind: 'none' }
  const firstChanged = interventionRow === null ? null : interventionRow + lag
  const problem = draft.outcome === null ? 'Choose the series.'
    : interventionRow === null || firstChanged === null || interventionRow < 2 || firstChanged > rows ? `Give the intervention row: a number from 2 to ${rows - lag}, the first row after the event.`
    : impact === null || (impact.kind === 'temporaryLevel' && (impact.until <= firstChanged || impact.until > rows + 1)) ? 'A temporary level change needs an end row after it starts and at most one past the last row.'
    : lag < 0 ? 'The lag cannot be negative.'
    : pairs < 0 || pairs > 12 ? 'Choose between 0 and 12 harmonic pairs.'
    : model.kind === 'count' && model.exposure === draft.outcome ? 'The exposure cannot be the series itself.'
    : null
  const ready = problem === null && job.kind !== 'running'

  const fit = async () => {
    if (!ready || draft.outcome === null || interventionRow === null || impact === null) return
    const current = session.start('analysis', 'Preparing the interrupted series')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    const exposure = model.kind === 'count' ? model.exposure : null
    const selected: ColumnId[] = [draft.outcome, ...(exposure === null ? [] : [exposure])]
    if (!isNonEmpty(selected)) return
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runInterruptedSeries }] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const matrix = await materialisePrepared(props.source, props.profile, props.prepared, selected)
      if (!session.current(current)) return
      if (!matrix.ok) { fail(describePreparedMaterialisationProblem(matrix.error)); return }
      const { values, rowCount, columns: used, timeAxis } = matrix.value
      if (timeAxis === null) { fail('The prepared series has no time key.'); return }
      const requested: InterruptedModel = model.kind === 'continuous' ? { kind: 'continuous', hacMaxLags: integer(model.hacMaxLags) } : { kind: 'count', exposure: exposure === null ? null : 1 }
      const ljungBoxLags = Math.max(1, Math.min(period ?? 12, Math.floor(rowCount / 4)))
      const evidence = await runInterruptedSeries(values, rowCount, used.length, { outcome: 0, model: requested, interventionRow: rowIndex(interventionRow), lag, impact: impactForKernel(impact), seasonal, ljungBoxLags })
      if (!session.current(current)) return
      if (!evidence.ok) { fail(describeAnalysisWorkerProblem(evidence.error)); return }
      const outcome = used[0]!
      const exposureColumn = used[1]
      const saved = parseTimeSeriesRun({
        kind: 'interrupted-series', id: newTimeSeriesRunId(), preparedDataset: props.prepared.id, createdAt: new Date().toISOString(),
        outcome: { id: outcome.id, name: outcome.name },
        specification: {
          model: requested.kind === 'continuous' ? requested : { kind: 'count', exposure: exposureColumn === undefined ? null : { id: exposureColumn.id, name: exposureColumn.name } },
          interventionRow, lag, impact, seasonal, ljungBoxLags,
        },
        evidence: evidence.value,
        plotTime: { kind: timeAxis.kind, values: Array.from(timeAxis.kind === 'calendar' ? timeAxis.timestamps : timeAxis.values) },
      })
      if (!saved.ok) { fail(saved.error); return }
      props.onRun(saved.value)
      session.finish(current)
    } catch (error: unknown) { fail(error instanceof Error ? error.message : String(error)) }
  }

  return <WorkbenchLayout id="time-series-interrupted"
    bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Time-series runs (${runs.length})`, defaultSize: 150, body: <TimeSeriesHistory entries={runs} onDelete={(run) => props.onDeleteRun(run.id)} /> }}
    inspector={{ trigger: { label: 'Requirements', icon: 'fact_check' }, title: 'Data and method requirements', body: <TimeSeriesRequirements method={TIME_SERIES_METHODS.interrupted} prepared={props.prepared} source={props.source.name} /> }}
    stage={<section className="@container/panel flex flex-col gap-5">
      <TimeSeriesHeading />
      <section className={panel('p-(--panel-space)')} aria-label="Time-series setup">
        {props.selector}
        <h3 className="mb-1 mt-3 text-body font-medium text-ink">Interrupted series</h3>
        <p className={`${fieldHint} mb-4 max-w-[65ch]`}>{TIME_SERIES_METHODS.interrupted.summary}</p>
        <fieldset disabled={job.kind === 'running'} className="m-0 min-w-0 space-y-4 border-0 p-0">
          <legend className="sr-only">Interrupted series specification</legend>
          <div className="grid gap-4 @lg/panel:grid-cols-2">
            <label className="block"><span className={fieldLabel}>Series</span>
              <Select className={field('text', 'mt-1')} value={draft.outcome ?? ''} onChange={(e) => set('outcome', columns.find((c) => c.id === e.target.value)?.id ?? null)}><option value="">Choose the series</option>{columns.map((c) => <option key={c.id} value={c.id}>{c.name}</option>)}</Select>
            </label>
            <div className="space-y-1">
              <span className={fieldLabel}>Outcome type</span>
              <SegmentedControl size="sm" ariaLabel="Outcome type" value={model.kind} onChange={(kind) => set('model', kind === 'count' ? { kind, exposure: null } : { kind, hacMaxLags: '' })} options={[{ value: 'continuous', label: 'Continuous' }, { value: 'count', label: 'Count' }]} />
              <p className={`${fieldHint} m-0`}>{count ? 'A quasi-Poisson model, as the reference tutorial fits a count.' : 'Least squares with Newey–West errors.'}</p>
            </div>
            {model.kind === 'count' && <label className="block"><span className={fieldLabel}>Exposure</span>
              <Select className={field('text', 'mt-1')} value={model.exposure ?? ''} onChange={(e) => set('model', { kind: 'count', exposure: columns.find((c) => c.id === e.target.value)?.id ?? null })}><option value="">None</option>{columns.filter((c) => c.id !== draft.outcome).map((c) => <option key={c.id} value={c.id}>{c.name}</option>)}</Select>
              <span className={fieldHint}>The population or denominator the count is a rate of; its log enters as an offset.</span>
            </label>}
          </div>
          <div className="grid gap-4 @lg/panel:grid-cols-2">
            <label className="block"><span className={fieldLabel}>Intervention row</span>
              <input aria-label="Intervention row" type="number" min={2} max={rows} className={field('text', 'mt-1')} value={draft.interventionRow} onChange={(e) => set('interventionRow', e.target.value)} />
              <span className={fieldHint}>The first row after the event. Rows before it are the pre-period.</span>
            </label>
            <label className="block"><span className={fieldLabel}>Lag</span>
              <input aria-label="Lag" type="number" min={0} className={field('text', 'mt-1')} value={draft.lag} onChange={(e) => set('lag', e.target.value)} />
              <span className={fieldHint}>Rows after the intervention row before the change is assumed to start.</span>
            </label>
          </div>
          <div className="space-y-1">
            <span className={fieldLabel}>Impact model</span>
            <SegmentedControl size="sm" wrap ariaLabel="Impact model" value={draft.impact.kind} onChange={(kind) => set('impact', kind === 'temporaryLevel' ? { kind, until: '' } : { kind })} options={[{ value: 'level', label: 'Level change' }, { value: 'levelAndSlope', label: 'Level and slope change' }, { value: 'slope', label: 'Slope change' }, { value: 'temporaryLevel', label: 'Temporary level change' }]} />
            <p className={`${fieldHint} m-0`}>Choose from what is known about the event, not from the data. Sensitivity to other shapes is reported separately.</p>
            {draft.impact.kind === 'temporaryLevel' && <label className="block max-w-xs"><span className={fieldLabel}>Until row</span>
              <input aria-label="Until row" type="number" min={2} max={rows} className={field('text', 'mt-1')} value={draft.impact.until} onChange={(e) => set('impact', { kind: 'temporaryLevel', until: e.target.value })} />
              <span className={fieldHint}>The first row on which the level is back to normal.</span>
            </label>}
          </div>
          <div className="grid gap-4 @lg/panel:grid-cols-2">
            <label className="block"><span className={fieldLabel}>Seasonal terms</span>
              <input aria-label="Seasonal terms" type="number" min={0} max={12} className={field('text', 'mt-1')} value={period === null ? '0' : draft.harmonicPairs} disabled={period === null} onChange={(e) => set('harmonicPairs', e.target.value)} />
              <span className={fieldHint}>{period === null ? 'Yearly rows have no seasonal period.' : `Sine and cosine pairs at the sampling period of ${period}, as the reference tutorial fits them; 0 for none.`}</span>
            </label>
            {model.kind === 'continuous' && <label className="block"><span className={fieldLabel}>Newey–West bandwidth</span>
              <input aria-label="Newey–West bandwidth" type="number" min={0} className={field('text', 'mt-1')} placeholder="Automatic" value={model.hacMaxLags} onChange={(e) => set('model', { kind: 'continuous', hacMaxLags: e.target.value })} />
              <span className={fieldHint}>Robust to serial correlation in the residuals; not a fix for a misspecified trend. Blank uses floor(4(n/100)^(2/9)).</span>
            </label>}
          </div>
          {problem !== null && draft.outcome !== null && <p role="status" className="m-0 text-body text-muted">{problem}</p>}
        </fieldset>
        <div className="mt-4 flex flex-wrap items-center gap-3">
          <button type="button" className={button('signal')} disabled={!ready || session.blocked} aria-busy={job.kind === 'running'} onClick={job.kind === 'running' ? undefined : () => void fit()}>Fit interrupted series</button>
          <span className="inline-flex h-5 w-5 items-center">{job.kind === 'running' && <Orb state="solving" aria-label="Interrupted series running" />}</span>
          {job.kind === 'running' && <button type="button" className={button('quiet')} onClick={session.cancel}>Cancel run</button>}
        </div>
        <JobNotice job={job} />
      </section>
      {runs.length === 0 && <p className="text-body text-muted">Choose the series, the intervention row and the impact model to begin.</p>}
      {runs.slice(-1).map((saved) => <TimeSeriesRunResult key={saved.id} run={saved} />)}
    </section>} />
}
