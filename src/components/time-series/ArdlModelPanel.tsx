import type { ReactNode } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import { useTimeSeriesDraft } from './useTimeSeriesDraft'
import type { ArdlDraft, Mode } from '@/domain/timeSeriesDraft'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { z } from 'zod'
import { ardlModelRequestSchema } from '@/domain/ardlModel'
import { ARDL_TERMS, newTimeSeriesRunId, parseTimeSeriesRun } from '@/domain/timeSeries'
import { assertNever, isNonEmpty } from '@/domain/dop'
import { isNumericDuckDbType } from '@/domain/dataset'
import { TIME_SERIES_METHODS } from '@/domain/methods'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Select } from '@/components/ui/Select'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Orb } from '@/components/ui/Orb'
import { actionGap, button, field, fieldHint, fieldLabel, fieldRow, panel, settingsStack, stepsStack } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { cn } from '@/lib/utils'
import { useRunActivity } from '@/lib/useRunActivity'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { TimeSeriesHeading, type TimeSeriesPanelProps } from './TimeSeriesPanel'
import { TimeSeriesHistory } from './TimeSeriesHistory'
import { TimeSeriesRequirements } from './TimeSeriesRequirements'
import { TimeSeriesRunResult } from './TimeSeriesRunResult'

const numeric = (text: string) => text.trim() === '' ? NaN : Number(text)
const futureValues = (text: string) => text.trim() === '' ? [] : text.trim().split(/[\s,;]+/).map(numeric)
function ordersFor(mode: Mode, p: number, q: number[], starting: number[], fixed: (number | null)[], minimum: number) {
  switch (mode) {
    case 'fixed': case 'rFixed': return { kind: mode, outcomeLag: p, predictorLags: q }
    case 'search': return { kind: mode, maximumLag: p, maximumOrders: q }
    case 'rHorizontal': return { kind: mode, maximum: [p, ...q], starting, fixed }
    case 'rGrid': return { kind: mode, minimumLag: minimum, maximumLag: p, maximumOrders: q, fixedOrders: fixed.slice(1) }
    default: return assertNever(mode)
  }
}

export function ArdlModelPanel(props: TimeSeriesPanelProps & {readonly selector: ReactNode}) {
  const columns = props.profile.columns.filter(c => props.prepared.columns.includes(c.id) && isNumericDuckDbType(c.duckdbType))
  const { outcome, roles, mode, starting, fixedOrders, minimum, outcomeLag, holdBack, terms, horizon, future } = useTimeSeriesDraft(props.prepared.id, state => state.ardl)
  const change = useWorkflow(state => state.changeTimeSeries)
  const setOutcome = (value: ArdlDraft['outcome']) => change(props.prepared.id, { type: 'ardl', field: 'outcome', value })
  const setRoles = (value: ArdlDraft['roles']) => change(props.prepared.id, { type: 'ardl', field: 'roles', value })
  const setMode = (value: ArdlDraft['mode']) => change(props.prepared.id, { type: 'ardl', field: 'mode', value })
  const setStarting = (value: ArdlDraft['starting']) => change(props.prepared.id, { type: 'ardl', field: 'starting', value })
  const setFixedOrders = (value: ArdlDraft['fixedOrders']) => change(props.prepared.id, { type: 'ardl', field: 'fixedOrders', value })
  const setMinimum = (value: ArdlDraft['minimum']) => change(props.prepared.id, { type: 'ardl', field: 'minimum', value })
  const setOutcomeLag = (value: ArdlDraft['outcomeLag']) => change(props.prepared.id, { type: 'ardl', field: 'outcomeLag', value })
  const setHoldBack = (value: ArdlDraft['holdBack']) => change(props.prepared.id, { type: 'ardl', field: 'holdBack', value })
  const setTerms = (value: ArdlDraft['terms']) => change(props.prepared.id, { type: 'ardl', field: 'terms', value })
  const setHorizon = (value: ArdlDraft['horizon']) => change(props.prepared.id, { type: 'ardl', field: 'horizon', value })
  const setFuture = (value: ArdlDraft['future']) => change(props.prepared.id, { type: 'ardl', field: 'future', value })
  const searching = mode !== 'fixed' && mode !== 'rFixed'
  const constrained = mode === 'rHorizontal' || mode === 'rGrid'
  const session = useJob('time-series:ardl-model')
  const { job } = session
  useRunActivity(props.onActivity, job.kind === 'running' ? {label: 'ARDL', progress: null} : null)
  const predictors = columns.filter(c => c.id !== outcome && roles[c.id]?.kind === 'predictor')
  const fixed = columns.filter(c => c.id !== outcome && roles[c.id]?.kind === 'fixed')
  const futureColumns = [...predictors, ...fixed]
  const runs = props.runs.filter(run => run.kind === 'ardl' || run.kind === 'ardl-model')
  const fit = async () => {
    const y = columns.find(c => c.id === outcome)
    if (y === undefined) return
    const current = session.start('analysis', 'Preparing ARDL')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    const orders = predictors.map(c => {const role = roles[c.id]; return role?.kind === 'predictor' ? numeric(role.lag) : NaN})
    const parsed = ardlModelRequestSchema.safeParse({outcome: 0, predictors: predictors.map((_,i) => i+1), fixed: fixed.map((_,i) => i+1+predictors.length), terms,
      orders: ordersFor(mode, numeric(outcomeLag), orders, [y,...predictors].map(c=>numeric(starting[c.id]??'1')), [y,...predictors].map(c=>(fixedOrders[c.id]??'').trim()===''?null:numeric(fixedOrders[c.id]!)), numeric(minimum)),
      holdBack: holdBack.trim() === '' ? null : numeric(holdBack), multiplierHorizon: numeric(horizon),
      future: future.kind === 'none' ? future : {kind: 'scenario', confidence: 0.95, predictors: predictors.map(c => futureValues(future.columns[c.id] ?? '')), fixed: fixed.map(c => futureValues(future.columns[c.id] ?? ''))}})
    if (!parsed.success) {fail(z.prettifyError(parsed.error)); return}
    const selected = [y, ...predictors, ...fixed].map(c => c.id)
    if (!isNonEmpty(selected)) { fail('Choose an outcome series.'); return }
    try {
      const [{materialisePrepared, describePreparedMaterialisationProblem}, {runArdlModel}] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const matrix = await materialisePrepared(props.source, props.profile, props.prepared, selected)
      if (!session.current(current)) return
      if (!matrix.ok) {fail(describePreparedMaterialisationProblem(matrix.error)); return}
      const {values,rowCount,timeAxis} = matrix.value
      if (timeAxis === null) {fail('The prepared series has no time key.'); return}
      const evidence = await runArdlModel(values,rowCount,selected.length,parsed.data)
      if (!session.current(current)) return
      if (!evidence.ok) {fail(describeAnalysisWorkerProblem(evidence.error)); return}
      const saved = parseTimeSeriesRun({kind:'ardl-model', id:newTimeSeriesRunId(),preparedDataset:props.prepared.id,createdAt:new Date().toISOString(),
        outcome:{id:y.id,name:y.name},predictors:predictors.map(({id,name})=>({id,name})),fixed:fixed.map(({id,name})=>({id,name})),specification:parsed.data,evidence:evidence.value,
        plotTime:{kind:timeAxis.kind,values:Array.from(timeAxis.kind==='calendar'?timeAxis.timestamps:timeAxis.values)}})
      if (!saved.ok) {fail(saved.error); return}
      props.onRun(saved.value); session.finish(current)
    } catch (error: unknown) {fail(error instanceof Error?error.message:String(error))}
  }
  return <WorkbenchLayout id="time-series-ardl" bottom={{trigger: { label: 'History', icon: 'history' }, title:`Time-series runs (${runs.length})`,defaultSize:150,body:<TimeSeriesHistory entries={runs} onDelete={run=>props.onDeleteRun(run.id)} />}}
    inspector={{trigger: { label: 'Requirements', icon: 'contract' }, title:'Data and method requirements',body:<TimeSeriesRequirements method={TIME_SERIES_METHODS.ardl} prepared={props.prepared} source={props.source.name} />}}
    stage={<section className="@container/panel flex flex-col gap-5"><TimeSeriesHeading /><section className={panel('p-(--panel-space)')} aria-label="Time-series setup">
      {props.selector}<h3 className="mb-1 mt-3 text-body font-medium text-ink">Autoregressive distributed lag model</h3><p className={`${fieldHint} m-0 max-w-[65ch]`}>Estimate how an outcome relates to its earlier values and to current and earlier values of other series.</p>
      <fieldset disabled={job.kind==='running'} className="m-0 mt-8 min-w-0 border-0 p-0"><legend className="sr-only">ARDL specification</legend>
        <div className={stepsStack}>
          <SettingsStep number={1} title="Choose the series">
            <label className="block max-w-md"><span className={fieldLabel}>Outcome series</span><Select className={field('text','mt-1')} value={outcome??''} onChange={e=>setOutcome(columns.find(c=>c.id===e.target.value)?.id??null)}><option value="">Choose outcome</option>{columns.map(c=><option key={c.id} value={c.id}>{c.name}</option>)}</Select></label>
            <ColumnChecklist title="Predictors and fixed regressors" help="Choose the other series to include. The outcome cannot also be a predictor." columns={columns} selected={futureColumns.map(c=>c.id)} reserved={outcome===null?[]:[outcome]} onChange={ids=>setRoles(Object.fromEntries(columns.map(c=>{const previous=roles[c.id];return [c.id,ids.includes(c.id)?previous?.kind==='fixed'||previous?.kind==='predictor'?previous:{kind:'predictor',lag:'2'}:{kind:'unused'}]})))} />
            <div className="grid gap-4">{futureColumns.map(c=>{const role=roles[c.id];return <div key={c.id} className={fieldRow.two}><label><span className={fieldLabel}>{c.name}</span><Select aria-label={`Role for ${c.name}`} className={field('text','mt-1')} value={role?.kind??'predictor'} onChange={e=>{const kind=e.target.value;if(kind==='fixed')setRoles({...roles,[c.id]:{kind}});else if(kind==='predictor')setRoles({...roles,[c.id]:{kind,lag:'2'}})}}><option value="predictor">Lagged predictor</option><option value="fixed">Fixed regressor</option></Select></label>{role?.kind==='predictor'&&<label><span className={fieldLabel}>{searching?'Maximum lag':'Lag order'}</span><input aria-label={`Lag for ${c.name}`} className={field('text','mt-1')} type="number" min={0} max={24} value={role.lag} onChange={e=>setRoles({...roles,[c.id]:{kind:'predictor',lag:e.target.value}})} /></label>}</div>})}</div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Predictors include their current and earlier values. Fixed regressors enter without lags. “Select by AIC” may exclude predictors. “Horizontal search” and “Constrained grid” keep every selected predictor and choose its lag order.</p>
          </SettingsStep>
          <SettingsStep number={2} title="Choose the lag orders">
            <SegmentedControl className="justify-self-start" wrap ariaLabel="Lag selection" value={mode} onChange={setMode} options={[{value:'search',label:'Select by AIC'},{value:'fixed',label:'Specify lags'},{value:'rFixed',label:'Specify lags with diagnostics'},{value:'rHorizontal',label:'Horizontal search'},{value:'rGrid',label:'Constrained grid'}]} />
            <div className={fieldRow.two}><label><span className={fieldLabel}>{searching?'Maximum outcome lag':'Outcome lag'}</span><input className={field('text','mt-1')} type="number" min={0} max={24} value={outcomeLag} onChange={e=>setOutcomeLag(e.target.value)} /></label>
              <label><span className={fieldLabel}>Deterministic terms</span><Select className={field('text','mt-1')} value={terms} onChange={e=>{const value=z.enum(['restricted-constant','constant','restricted-trend','trend']).safeParse(e.target.value);if(value.success)setTerms(value.data)}}>{Object.entries(ARDL_TERMS).map(([key,value])=><option key={key} value={key}>{value.label}</option>)}</Select></label></div>
            {constrained && <section className={settingsStack} aria-label="Search constraints">
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>All candidates use the same estimation sample. A fixed lag order is kept throughout the search; leave it blank to search that series. Horizontal search adjusts lag orders from the specified starting values. The constrained grid evaluates every allowed combination, up to 10,000 models.</p>
              {mode==='rGrid' && <label className="block max-w-xs"><span className={fieldLabel}>Minimum outcome lag</span><input className={field('text','mt-1')} type="number" min={1} max={24} value={minimum} onChange={e=>setMinimum(e.target.value)} /></label>}
              {columns.filter(c=>(c.id===outcome&&mode==='rHorizontal')||predictors.some(p=>p.id===c.id)).map(c=><div key={c.id} className={fieldRow.two}>
                {mode==='rHorizontal' && <label><span className={fieldLabel}>Starting lag for {c.name}</span><input className={field('text','mt-1')} type="number" min={c.id===outcome?1:0} max={24} value={starting[c.id]??'1'} onChange={e=>setStarting({...starting,[c.id]:e.target.value})} /></label>}
                {(mode==='rHorizontal'||c.id!==outcome) && <label><span className={fieldLabel}>Fixed lag for {c.name}</span><input className={field('text','mt-1')} type="number" min={c.id===outcome?1:0} max={24} placeholder="Search this series" value={fixedOrders[c.id]??''} onChange={e=>setFixedOrders({...fixedOrders,[c.id]:e.target.value})} /></label>}
              </div>)}
            </section>}
          </SettingsStep>
          <SettingsStep number={3} title="Set the sample and multipliers">
            <div className={fieldRow.two}><label><span className={fieldLabel}>Initial observations to exclude</span><input className={field('text','mt-1')} type="number" min={0} placeholder="Use the largest lag" value={holdBack} onChange={e=>setHoldBack(e.target.value)} /></label><label><span className={fieldLabel}>Multiplier horizon</span><input className={field('text','mt-1')} type="number" min={0} max={200} value={horizon} onChange={e=>setHorizon(e.target.value)} /></label></div>
          </SettingsStep>
          <SettingsStep number={4} title="Forecast">
            <label className="flex gap-2 text-body text-ink"><input type="checkbox" checked={future.kind==='scenario'} onChange={e=>setFuture(e.target.checked?{kind:'scenario',columns:{}}:{kind:'none'})} />Forecast with supplied future values</label>
            {future.kind==='scenario'&&<div className={settingsStack}><p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Enter one value per future period, separated by spaces or commas. Supply the same number of periods for every included column. No future values are filled in automatically.</p>{futureColumns.map(c=><label key={c.id} className="block"><span className={fieldLabel}>Future {c.name}</span><textarea className={field('text','mt-1')} rows={2} value={future.columns[c.id]??''} onChange={e=>setFuture({kind:'scenario',columns:{...future.columns,[c.id]:e.target.value}})} /></label>)}</div>}
          </SettingsStep>
        </div>
      </fieldset><div className={cn(actionGap, 'flex flex-wrap items-center gap-3')}><button className={button('signal')} disabled={session.blocked||outcome===null||predictors.length===0} aria-busy={job.kind==='running'} onClick={job.kind==='running'?undefined:()=>void fit()}>Fit ARDL</button><span className="inline-flex h-5 w-5">{job.kind==='running'&&<Orb state="solving" aria-label="ARDL running" />}</span>{job.kind==='running'&&<button type="button" className={button('quiet')} onClick={session.cancel}>Cancel run</button>}</div>
      <JobNotice job={job} /></section>{runs.slice(-1).map(run=><TimeSeriesRunResult key={run.id} run={run} />)}</section>} />
}
