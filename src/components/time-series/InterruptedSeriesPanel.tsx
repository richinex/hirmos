import type { ReactNode } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import { useTimeSeriesDraft } from './useTimeSeriesDraft'
import type { ContinuousErrorsDraft, InterruptedDraft, TimeSeriesEvent } from '@/domain/timeSeriesDraft'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { newTimeSeriesRunId, parseTimeSeriesRun } from '@/domain/timeSeries'
import { DEFAULT_ARMA_ITERATIONS, MAX_ARMA_ORDER, impactForKernel, rowIndex, rowNumber, type ContinuousErrors, type DeclaredImpact, type InterruptedModel, type InterruptedSeasonal } from '@/domain/interruptedSeries'
import { isNumericDuckDbType, type ColumnId } from '@/domain/dataset'
import { assertNever, isNonEmpty } from '@/domain/dop'
import { TIME_SERIES_METHODS } from '@/domain/methods'
import { seasonalPeriodOf } from '@/domain/seasonal'
import { effectiveFrequency } from '@/domain/resampling'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
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

/** The error model as typed, or the reason it cannot be requested yet. */
const continuousErrors = (draft: ContinuousErrorsDraft): { readonly kind: 'ready'; readonly errors: ContinuousErrors } | { readonly kind: 'problem'; readonly detail: string } => {
  switch (draft.kind) {
    case 'neweyWest': {
      const maxLags = integer(draft.maxLags)
      if (draft.maxLags.trim() !== '' && (maxLags === null || maxLags < 0)) return { kind: 'problem', detail: 'The Newey–West bandwidth is a whole number of lags, or blank for the automatic one.' }
      return { kind: 'ready', errors: { kind: 'neweyWest', maxLags } }
    }
    case 'arma': {
      const p = integer(draft.p) ?? 0
      const q = integer(draft.q) ?? 0
      const maxIter = integer(draft.maxIter)
      if (p < 0 || q < 0 || p > MAX_ARMA_ORDER || q > MAX_ARMA_ORDER) return { kind: 'problem', detail: `ARMA orders are whole numbers from 0 to ${MAX_ARMA_ORDER}.` }
      if (p + q === 0) return { kind: 'problem', detail: 'ARMA errors need at least one autoregressive or moving-average term; with none, the fit is the Newey–West one.' }
      if (maxIter === null || maxIter < 1) return { kind: 'problem', detail: 'Give the optimiser a positive number of iterations.' }
      return { kind: 'ready', errors: { kind: 'arma', p, q, maxIter } }
    }
    default: return assertNever(draft)
  }
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
  const errors = model.kind === 'continuous' ? continuousErrors(model.errors) : null
  const armaDraft = model.kind === 'continuous' && model.errors.kind === 'arma' ? model.errors : null
  const problem = draft.outcome === null ? 'Choose the series.'
    : interventionRow === null || firstChanged === null || interventionRow < 2 || firstChanged > rows ? `Give the intervention row: a number from 2 to ${rows - lag}, the first row after the event.`
    : impact === null || (impact.kind === 'temporaryLevel' && (impact.until <= firstChanged || impact.until > rows + 1)) ? 'A temporary level change needs an end row after it starts and at most one past the last row.'
    : lag < 0 ? 'The lag cannot be negative.'
    : pairs < 0 || pairs > 12 ? 'Choose between 0 and 12 harmonic pairs.'
    : model.kind === 'count' && model.exposure === draft.outcome ? 'The exposure cannot be the series itself.'
    : errors !== null && errors.kind === 'problem' ? errors.detail
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
      if (errors !== null && errors.kind === 'problem') return
      const requested: InterruptedModel = errors !== null ? { kind: 'continuous', errors: errors.errors } : { kind: 'count', exposure: exposure === null ? null : 1 }
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
              <ParameterLabel className={fieldLabel} label="Outcome type" help={ESTIMATION_PARAMETER_HELP.interruptedSeries.outcomeType} />
              <SegmentedControl size="sm" ariaLabel="Outcome type" value={model.kind} onChange={(kind) => set('model', kind === 'count' ? { kind, exposure: null } : { kind, errors: { kind: 'neweyWest', maxLags: '' } })} options={[{ value: 'continuous', label: 'Continuous' }, { value: 'count', label: 'Count' }]} />
            </div>
            {model.kind === 'count' && <label className="block"><ParameterLabel className={fieldLabel} label="Exposure" help={ESTIMATION_PARAMETER_HELP.interruptedSeries.exposure} />
              <Select aria-label="Exposure" className={field('text', 'mt-1')} value={model.exposure ?? ''} onChange={(e) => set('model', { kind: 'count', exposure: columns.find((c) => c.id === e.target.value)?.id ?? null })}><option value="">None</option>{columns.filter((c) => c.id !== draft.outcome).map((c) => <option key={c.id} value={c.id}>{c.name}</option>)}</Select>
            </label>}
          </div>
          <div className="grid gap-4 @lg/panel:grid-cols-2">
            <label className="block"><ParameterLabel className={fieldLabel} label="Intervention row" help={ESTIMATION_PARAMETER_HELP.interruptedSeries.interventionRow} />
              <input aria-label="Intervention row" type="number" min={2} max={rows} className={field('text', 'mt-1')} value={draft.interventionRow} onChange={(e) => set('interventionRow', e.target.value)} />
            </label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Lag" help={ESTIMATION_PARAMETER_HELP.interruptedSeries.lag} />
              <input aria-label="Lag" type="number" min={0} className={field('text', 'mt-1')} value={draft.lag} onChange={(e) => set('lag', e.target.value)} />
            </label>
          </div>
          <div className="space-y-1">
            <ParameterLabel className={fieldLabel} label="Impact model" help={ESTIMATION_PARAMETER_HELP.interruptedSeries.impactModel[draft.impact.kind]} />
            <SegmentedControl size="sm" wrap ariaLabel="Impact model" value={draft.impact.kind} onChange={(kind) => set('impact', kind === 'temporaryLevel' ? { kind, until: '' } : { kind })} options={[{ value: 'level', label: 'Level change' }, { value: 'levelAndSlope', label: 'Level and slope change' }, { value: 'slope', label: 'Slope change' }, { value: 'temporaryLevel', label: 'Temporary level change' }]} />
            {draft.impact.kind === 'temporaryLevel' && <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Until row" help={ESTIMATION_PARAMETER_HELP.interruptedSeries.untilRow} />
              <input aria-label="Until row" type="number" min={2} max={rows} className={field('text', 'mt-1')} value={draft.impact.until} onChange={(e) => set('impact', { kind: 'temporaryLevel', until: e.target.value })} />
            </label>}
          </div>
          <div className="grid gap-4 @lg/panel:grid-cols-2">
            <label className="block"><ParameterLabel className={fieldLabel} label="Seasonal terms" help={period === null ? ESTIMATION_PARAMETER_HELP.interruptedSeries.noSeasonalPeriod : ESTIMATION_PARAMETER_HELP.interruptedSeries.seasonalTerms} />
              <input aria-label="Seasonal terms" type="number" min={0} max={12} className={field('text', 'mt-1')} value={period === null ? '0' : draft.harmonicPairs} disabled={period === null} onChange={(e) => set('harmonicPairs', e.target.value)} />
            </label>
          </div>
          {model.kind === 'continuous' && <div className="space-y-2">
            <ParameterLabel className={fieldLabel} label="Error model" help={model.errors.kind === 'neweyWest' ? ESTIMATION_PARAMETER_HELP.interruptedSeries.neweyWest : ESTIMATION_PARAMETER_HELP.interruptedSeries.armaErrors} />
            <SegmentedControl size="sm" ariaLabel="Error model" value={model.errors.kind} onChange={(kind) => set('model', { kind: 'continuous', errors: kind === 'arma' ? { kind, p: '1', q: '0', maxIter: String(DEFAULT_ARMA_ITERATIONS) } : { kind, maxLags: '' } })} options={[{ value: 'neweyWest', label: 'Newey–West' }, { value: 'arma', label: 'ARMA errors' }]} />
            {armaDraft === null ? <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Newey–West bandwidth" help={ESTIMATION_PARAMETER_HELP.interruptedSeries.neweyWestBandwidth} />
              <input aria-label="Newey–West bandwidth" type="number" min={0} className={field('text', 'mt-1')} placeholder="Automatic" value={model.errors.kind === 'neweyWest' ? model.errors.maxLags : ''} onChange={(e) => set('model', { kind: 'continuous', errors: { kind: 'neweyWest', maxLags: e.target.value } })} />
            </label> : <div className="grid gap-4 @lg/panel:grid-cols-3">
              <label className="block"><ParameterLabel className={fieldLabel} label="Autoregressive order" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.autoregressiveOrder} />
                <input aria-label="Autoregressive order" type="number" min={0} max={MAX_ARMA_ORDER} className={field('text', 'mt-1')} value={armaDraft.p} onChange={(e) => set('model', { kind: 'continuous', errors: { ...armaDraft, p: e.target.value } })} />
              </label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Moving-average order" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.movingAverageOrder} />
                <input aria-label="Moving-average order" type="number" min={0} max={MAX_ARMA_ORDER} className={field('text', 'mt-1')} value={armaDraft.q} onChange={(e) => set('model', { kind: 'continuous', errors: { ...armaDraft, q: e.target.value } })} />
              </label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Optimiser iterations" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.armaIterations} />
                <input aria-label="Optimiser iterations" type="number" min={1} className={field('text', 'mt-1')} value={armaDraft.maxIter} onChange={(e) => set('model', { kind: 'continuous', errors: { ...armaDraft, maxIter: e.target.value } })} />
              </label>
            </div>}
          </div>}
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
