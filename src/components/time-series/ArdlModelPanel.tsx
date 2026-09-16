import { useState, type ReactNode } from 'react'
import { z } from 'zod'
import { ardlModelRequestSchema } from '@/domain/ardlModel'
import { ARDL_TERMS, newTimeSeriesRunId, parseTimeSeriesRun, type ArdlTerms } from '@/domain/timeSeries'
import { assertNever, isNonEmpty } from '@/domain/dop'
import { isNumericDuckDbType, type ColumnId } from '@/domain/dataset'
import { TIME_SERIES_METHODS } from '@/domain/methods'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Select } from '@/components/ui/Select'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Orb } from '@/components/ui/Orb'
import { button, field, fieldLabel, fieldHint, panel } from '@/components/ui/recipes'
import { useRunActivity } from '@/lib/useRunActivity'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { TimeSeriesHeading, type TimeSeriesPanelProps } from './TimeSeriesPanel'
import { TimeSeriesHistory } from './TimeSeriesHistory'
import { TimeSeriesRequirements } from './TimeSeriesRequirements'
import { TimeSeriesRunResult } from './TimeSeriesRunResult'

type Role = {readonly kind: 'unused'} | {readonly kind: 'predictor'; readonly lag: string} | {readonly kind: 'fixed'}
type Future = {readonly kind: 'none'} | {readonly kind: 'scenario'; readonly columns: Readonly<Record<string, string>>}
type Job = {readonly kind: 'idle'} | {readonly kind: 'running'} | {readonly kind: 'failed'; readonly detail: string}
const numeric = (text: string) => text.trim() === '' ? NaN : Number(text)
const futureValues = (text: string) => text.trim() === '' ? [] : text.trim().split(/[\s,;]+/).map(numeric)
type Mode = 'fixed' | 'search' | 'rFixed' | 'rHorizontal' | 'rGrid'
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
  const [outcome, setOutcome] = useState<ColumnId | null>(null)
  const [roles, setRoles] = useState<Readonly<Record<string, Role>>>({})
  const [mode, setMode] = useState<Mode>('search')
  const [starting, setStarting] = useState<Readonly<Record<string,string>>>({})
  const [fixedOrders, setFixedOrders] = useState<Readonly<Record<string,string>>>({})
  const [minimum, setMinimum] = useState('1')
  const searching = mode !== 'fixed' && mode !== 'rFixed'
  const constrained = mode === 'rHorizontal' || mode === 'rGrid'
  const [outcomeLag, setOutcomeLag] = useState('2')
  const [holdBack, setHoldBack] = useState('')
  const [terms, setTerms] = useState<ArdlTerms>('constant')
  const [horizon, setHorizon] = useState('12')
  const [future, setFuture] = useState<Future>({kind: 'none'})
  const [job, setJob] = useState<Job>({kind: 'idle'})
  useRunActivity(props.onActivity, job.kind === 'running' ? {label: 'ARDL', progress: null} : null)
  const predictors = columns.filter(c => c.id !== outcome && roles[c.id]?.kind === 'predictor')
  const fixed = columns.filter(c => c.id !== outcome && roles[c.id]?.kind === 'fixed')
  const futureColumns = [...predictors, ...fixed]
  const runs = props.runs.filter(run => run.kind === 'ardl' || run.kind === 'ardl-model')
  const fit = async () => {
    const y = columns.find(c => c.id === outcome)
    if (y === undefined) return
    const orders = predictors.map(c => {const role = roles[c.id]; return role?.kind === 'predictor' ? numeric(role.lag) : NaN})
    const parsed = ardlModelRequestSchema.safeParse({outcome: 0, predictors: predictors.map((_,i) => i+1), fixed: fixed.map((_,i) => i+1+predictors.length), terms,
      orders: ordersFor(mode, numeric(outcomeLag), orders, [y,...predictors].map(c=>numeric(starting[c.id]??'1')), [y,...predictors].map(c=>(fixedOrders[c.id]??'').trim()===''?null:numeric(fixedOrders[c.id]!)), numeric(minimum)),
      holdBack: holdBack.trim() === '' ? null : numeric(holdBack), multiplierHorizon: numeric(horizon),
      future: future.kind === 'none' ? future : {kind: 'scenario', confidence: 0.95, predictors: predictors.map(c => futureValues(future.columns[c.id] ?? '')), fixed: fixed.map(c => futureValues(future.columns[c.id] ?? ''))}})
    if (!parsed.success) {setJob({kind: 'failed', detail: z.prettifyError(parsed.error)}); return}
    const selected = [y, ...predictors, ...fixed].map(c => c.id)
    if (!isNonEmpty(selected)) return
    setJob({kind: 'running'})
    try {
      const [{materialisePrepared, describePreparedMaterialisationProblem}, {runArdlModel}] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const matrix = await materialisePrepared(props.source, props.profile, props.prepared, selected)
      if (!matrix.ok) {setJob({kind:'failed',detail:describePreparedMaterialisationProblem(matrix.error)}); return}
      const {values,rowCount,timeAxis} = matrix.value
      if (timeAxis === null) {setJob({kind:'failed',detail:'The prepared series has no time key.'}); return}
      const evidence = await runArdlModel(values,rowCount,selected.length,parsed.data)
      if (!evidence.ok) {setJob({kind:'failed',detail:describeAnalysisWorkerProblem(evidence.error)}); return}
      const saved = parseTimeSeriesRun({kind:'ardl-model', id:newTimeSeriesRunId(),preparedDataset:props.prepared.id,createdAt:new Date().toISOString(),
        outcome:{id:y.id,name:y.name},predictors:predictors.map(({id,name})=>({id,name})),fixed:fixed.map(({id,name})=>({id,name})),specification:parsed.data,evidence:evidence.value,
        plotTime:{kind:timeAxis.kind,values:Array.from(timeAxis.kind==='calendar'?timeAxis.timestamps:timeAxis.values)}})
      if (!saved.ok) {setJob({kind:'failed',detail:saved.error}); return}
      props.onRun(saved.value); setJob({kind:'idle'})
    } catch (error: unknown) {setJob({kind:'failed',detail:error instanceof Error?error.message:String(error)})}
  }
  return <WorkbenchLayout id="time-series-ardl" bottom={{trigger: { label: 'History', icon: 'history' }, title:`Time-series runs (${runs.length})`,defaultSize:150,body:<TimeSeriesHistory entries={runs} onDelete={run=>props.onDeleteRun(run.id)} />}}
    inspector={{trigger: { label: 'Requirements', icon: 'fact_check' }, title:'Data and method requirements',body:<TimeSeriesRequirements method={TIME_SERIES_METHODS.ardl} prepared={props.prepared} source={props.source.name} />}}
    stage={<section className="@container/panel flex flex-col gap-5"><TimeSeriesHeading /><section className={panel('p-(--panel-space)')} aria-label="Time-series setup">
      {props.selector}<h3 className="mb-1 mt-3 text-body font-medium text-ink">Autoregressive distributed lag model</h3><p className={`${fieldHint} mb-4`}>Estimate how an outcome relates to its earlier values and to current and earlier values of other series.</p>
      <fieldset disabled={job.kind==='running'} className="m-0 min-w-0 space-y-4 border-0 p-0"><legend className="sr-only">ARDL specification</legend>
        <label className="block"><span className={fieldLabel}>Outcome series</span><Select className={field('text','mt-1')} value={outcome??''} onChange={e=>setOutcome(columns.find(c=>c.id===e.target.value)?.id??null)}><option value="">Choose outcome</option>{columns.map(c=><option key={c.id} value={c.id}>{c.name}</option>)}</Select></label>
        <SegmentedControl variant="line" size="sm" ariaLabel="Lag selection" value={mode} onChange={setMode} options={[{value:'search',label:'Select by AIC'},{value:'fixed',label:'Specify lags'},{value:'rFixed',label:'Specify lags with diagnostics'},{value:'rHorizontal',label:'Horizontal search'},{value:'rGrid',label:'Constrained grid'}]} />
        <div className="grid gap-4 @lg/panel:grid-cols-2"><label><span className={fieldLabel}>{searching?'Maximum outcome lag':'Outcome lag'}</span><input className={field('text','mt-1')} type="number" min={0} max={24} value={outcomeLag} onChange={e=>setOutcomeLag(e.target.value)} /></label>
        <label><span className={fieldLabel}>Deterministic terms</span><Select className={field('text','mt-1')} value={terms} onChange={e=>{const value=z.enum(['restricted-constant','constant','restricted-trend','trend']).safeParse(e.target.value);if(value.success)setTerms(value.data)}}>{Object.entries(ARDL_TERMS).map(([key,value])=><option key={key} value={key}>{value.label}</option>)}</Select></label></div>
        <ColumnChecklist title="Predictors and fixed regressors" help="Choose the other series to include. The outcome cannot also be a predictor." columns={columns} selected={futureColumns.map(c=>c.id)} reserved={outcome===null?[]:[outcome]} onChange={ids=>setRoles(Object.fromEntries(columns.map(c=>{const previous=roles[c.id];return [c.id,ids.includes(c.id)?previous?.kind==='fixed'||previous?.kind==='predictor'?previous:{kind:'predictor',lag:'2'}:{kind:'unused'}]})))} />
        <div className="space-y-3">{futureColumns.map(c=>{const role=roles[c.id];return <div key={c.id} className="grid min-w-0 items-end gap-2 @lg/panel:grid-cols-2"><label><span className={fieldLabel}>{c.name}</span><Select aria-label={`Role for ${c.name}`} className={field('text','mt-1')} value={role?.kind??'predictor'} onChange={e=>{const kind=e.target.value;if(kind==='fixed')setRoles({...roles,[c.id]:{kind}});else if(kind==='predictor')setRoles({...roles,[c.id]:{kind,lag:'2'}})}}><option value="predictor">Lagged predictor</option><option value="fixed">Fixed regressor</option></Select></label>{role?.kind==='predictor'&&<label><span className={fieldLabel}>{searching?'Maximum lag':'Lag order'}</span><input aria-label={`Lag for ${c.name}`} className={field('text','mt-1')} type="number" min={0} max={24} value={role.lag} onChange={e=>setRoles({...roles,[c.id]:{kind:'predictor',lag:e.target.value}})} /></label>}</div>})}</div>
        <p className={fieldHint}>Predictors include their current and earlier values. Fixed regressors enter without lags. “Select by AIC” may exclude predictors. “Horizontal search” and “Constrained grid” keep every selected predictor and choose its lag order.</p>
        {constrained && <section className="space-y-3" aria-label="Search constraints">
          <p className={fieldHint}>All candidates use the same estimation sample. A fixed lag order is kept throughout the search; leave it blank to search that series. Horizontal search adjusts lag orders from the specified starting values. The constrained grid evaluates every allowed combination, up to 10,000 models.</p>
          {mode==='rGrid' && <label className="block"><span className={fieldLabel}>Minimum outcome lag</span><input className={field('text','mt-1')} type="number" min={1} max={24} value={minimum} onChange={e=>setMinimum(e.target.value)} /></label>}
          {columns.filter(c=>c.id===outcome||predictors.some(p=>p.id===c.id)).map(c=><div key={c.id} className="grid gap-3 @lg/panel:grid-cols-2">
            {mode==='rHorizontal' && <label><span className={fieldLabel}>Starting lag for {c.name}</span><input className={field('text','mt-1')} type="number" min={c.id===outcome?1:0} max={24} value={starting[c.id]??'1'} onChange={e=>setStarting({...starting,[c.id]:e.target.value})} /></label>}
            {(mode==='rHorizontal'||c.id!==outcome) && <label><span className={fieldLabel}>Fixed lag for {c.name}</span><input className={field('text','mt-1')} type="number" min={c.id===outcome?1:0} max={24} placeholder="Search this series" value={fixedOrders[c.id]??''} onChange={e=>setFixedOrders({...fixedOrders,[c.id]:e.target.value})} /></label>}
          </div>)}
        </section>}
        <div className="grid gap-4 @lg/panel:grid-cols-2"><label><span className={fieldLabel}>Initial observations to exclude</span><input className={field('text','mt-1')} type="number" min={0} placeholder="Use the largest lag" value={holdBack} onChange={e=>setHoldBack(e.target.value)} /></label><label><span className={fieldLabel}>Multiplier horizon</span><input className={field('text','mt-1')} type="number" min={0} max={200} value={horizon} onChange={e=>setHorizon(e.target.value)} /></label></div>
        <label className="flex gap-2 text-body text-ink"><input type="checkbox" checked={future.kind==='scenario'} onChange={e=>setFuture(e.target.checked?{kind:'scenario',columns:{}}:{kind:'none'})} />Forecast with supplied future values</label>
        {future.kind==='scenario'&&<div className="space-y-3"><p className={fieldHint}>Enter one value per future period, separated by spaces or commas. Supply the same number of periods for every included column. No future values are filled in automatically.</p>{futureColumns.map(c=><label key={c.id} className="block"><span className={fieldLabel}>Future {c.name}</span><textarea className={field('text','mt-1')} rows={2} value={future.columns[c.id]??''} onChange={e=>setFuture({kind:'scenario',columns:{...future.columns,[c.id]:e.target.value}})} /></label>)}</div>}
      </fieldset><div className="mt-4 flex items-center gap-3"><button className={button('signal')} disabled={outcome===null||predictors.length===0} aria-busy={job.kind==='running'} onClick={job.kind==='running'?undefined:()=>void fit()}>Fit ARDL</button><span className="inline-flex h-5 w-5">{job.kind==='running'&&<Orb state="solving" aria-label="ARDL running" />}</span></div>
      {job.kind==='failed'&&<p role="alert" className="mt-3 text-body text-danger">{job.detail}</p>}</section>{runs.slice(-1).map(run=><TimeSeriesRunResult key={run.id} run={run} />)}</section>} />
}
