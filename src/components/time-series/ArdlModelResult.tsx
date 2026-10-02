import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { Metadata } from '@/components/ui/Metadata'
import { useMemo, useState } from 'react'
import type { TimeSeriesRun } from '@/domain/timeSeries'
import { assertNever } from '@/domain/dop'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { longRunOption } from '@/charts/data/longRun'
import { useChartTheme } from '@/charts/theme'
import { Select } from '@/components/ui/Select'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { field, fieldLabel, resultSurface, resultTitle } from '@/components/ui/recipes'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import { formatPercent, formatStatistic } from '@/lib/format/number'
import { TimeSeriesEquation } from './TimeSeriesEquation'
import type { VisibleWindow } from '@/charts/window'
import { ardlBoundsDecision, type EcmCoefficient } from '@/domain/ardlModel'

type Run = Extract<TimeSeriesRun,{kind:'ardl-model'}>
const number = (n:number) => formatStatistic('raw',n).text
const reasons = {
  noPredictors:'AIC selection retained no predictors, so there is no predictor level relationship to test.',
  zeroOrder:'This error-correction specification requires lagged values for every predictor. To include a zero-order predictor, choose “Specify lags with diagnostics”.',
  tooManyPredictors:'Bounds critical values are unavailable for this number of predictors.',
  undefinedNormalization:'The fitted model did not produce a finite, defined long-run relationship.',
} as const

function coefficientName(term:EcmCoefficient,run:Run):string {
  switch(term.kind) {
    case 'constant':return 'Constant'
    case 'trend':return 'Time trend'
    case 'outcome':return `${run.outcome.name}, lag ${term.lag}`
    case 'predictor':return `${run.predictors[term.column]?.name}, lag ${term.lag}`
    case 'fixed':return `${run.fixed[term.column]?.name} (fixed)`
    case 'outcomeChange':return `Change in ${run.outcome.name}, lag ${term.lag}`
    case 'predictorChange':return `Change in ${run.predictors[term.column]?.name}, lag ${term.lag}`
    default:return assertNever(term)
  }
}

function CoefficientTable({title,help,exportName,run,terms,params,covariance}:{readonly title:string;readonly help?:string;readonly exportName:string;readonly run:Run;readonly terms:readonly EcmCoefficient[];readonly params:readonly number[];readonly covariance:readonly (readonly number[])[]}) {
  type Row={readonly index:number;readonly name:string;readonly coefficient:number;readonly standardError:number}
  const rows:readonly Row[]=terms.map((term,i)=>({index:i,name:coefficientName(term,run),coefficient:params[i]!,standardError:Math.sqrt(covariance[i]![i]!)}))
  return <EvidenceTable frame="none" title={title} help={help} rows={rows} rowKey={row=>String(row.index)} noun="coefficient" empty="The fit reported no coefficients." exportName={exportName} columns={[
    {id:'term',header:'Term',value:row=>row.name},
    figureColumn<Row>('coefficient','Coefficient',row=>row.coefficient,number),
    figureColumn<Row>('standard-error','Standard error',row=>row.standardError,number),
  ]} />
}

function RAnalysis({run}:{readonly run:Run}) {
  const r=run.evidence.rAnalysis
  if(r.kind==='notRequested')return null
  type Test=typeof r.serialCorrelation[number]
  type Candidate=Extract<typeof r.ranking,{readonly rows:unknown}>['rows'][number]
  return <section className="min-w-0 space-y-4" aria-label="Error-correction analysis">
    <CoefficientTable title="Error-correction model" help={`This equation models the change in ${run.outcome.name}. A zero-lag predictor enters at its current level without an additional change term.`} exportName="ardl-error-correction" run={run} terms={r.coefficients} params={r.params} covariance={r.covariance} />
    <details><DisclosureSummary className="cursor-pointer text-body text-ink">Model diagnostics</DisclosureSummary><div className="mt-3 space-y-3">
      <p className="m-0 text-label text-muted">PSS AIC {number(r.aicPss)}; PSS SBC {number(r.sbcPss)}. Higher values are preferred when comparing models fitted to the same observations.</p>
      {run.evidence.longRun.kind!=='rCalibrated'&&<p className="m-0 text-label text-muted">Bounds F statistic {number(r.boundsF)}.{r.boundsT.kind==='recorded'?` Bounds t statistic ${number(r.boundsT.value)}.`:' The t-bounds statistic does not apply to the selected restricted deterministic case.'} These are test statistics, not p-values. Critical bounds for these statistics are not reported.</p>}
      <EvidenceTable frame="none" title="Serial-correlation tests" rows={r.serialCorrelation} rowKey={test=>String(test.order)} noun="test" empty="No serial-correlation test was run." exportName="ardl-serial-correlation" columns={[
        figureColumn<Test>('order','Residual lags',test=>test.order,String),
        figureColumn<Test>('statistic','BG statistic',test=>test.statistic,number),
        figureColumn<Test>('p','p-value',test=>test.pValue,number),
      ]} />
      <p className="m-0 text-label text-muted">Breusch–Godfrey tests assess remaining serial correlation in the residuals. Small p-values indicate that the fitted model may not account for all of the time dependence.</p>
    </div></details>
    {r.ranking.kind!=='notRequested'&&<details open><DisclosureSummary className="cursor-pointer text-body text-ink">Lag-search results</DisclosureSummary>
      <p className="text-label text-muted">{r.ranking.kind==='grid'?`${r.ranking.evaluated} combinations evaluated. `:'Horizontal search from the recorded starting orders. '}Up to 20 retained candidates are shown in decreasing PSS AIC order.</p>
      <EvidenceTable frame="none" title="Lag-search ranking" rows={r.ranking.rows} rowKey={row=>row.order.join('-')} noun="candidate" empty="The search retained no candidates." exportName="ardl-lag-search" columns={[
        ...[run.outcome,...run.predictors].map((c,j)=>figureColumn<Candidate>(`order-${c.id}`,c.name,row=>row.order[j]!,String)),
        figureColumn<Candidate>('aic','PSS AIC',row=>row.aicPss,number),
      ]} />
    </details>}
  </section>
}

function LongRunTable({run}:{readonly run:Run}) {
  const result = run.evidence.longRun
  if (result.kind === 'unavailable') return null
  const deterministic = run.evidence.coefficients.filter(term => term.kind === 'constant' || term.kind === 'trend')
  const labels = [
    ...deterministic.map(term => term.kind === 'constant' ? 'Constant' : 'Time trend'),
    run.outcome.name,
    ...run.predictors.filter((_, i) => run.evidence.predictorLags[i] !== null).map(column => column.name),
  ]
  // Moving the other terms to the outcome's right-hand side reverses both signs and interval bounds.
  type LongRunRow = { readonly label: string; readonly estimate: number; readonly lower: number; readonly upper: number }
  const rows: readonly LongRunRow[] = labels.flatMap((label, i) => i === deterministic.length ? [] : [{
    label, estimate: -result.normalized[i]!, lower: -result.intervals[i]![1], upper: -result.intervals[i]![0],
  }])
  return <section className="min-w-0 space-y-3" aria-label="Long-run relationship">
    <EvidenceTable frame="none" title="Long-run relationship" help={`Coefficients describe how ${run.outcome.name} relates to each predictor in the long-run relationship, holding the other predictors unchanged.`} rows={rows} rowKey={row => row.label} noun="coefficient" empty="No long-run coefficients." exportName="ardl-long-run" columns={[
      { id: 'term', header: 'Term', value: row => row.label },
      figureColumn<LongRunRow>('coefficient', 'Coefficient', row => row.estimate, number),
      { id: 'interval', header: '95% confidence interval', align: 'right', value: row => row.lower, format: (_, row) => `${number(row.lower)} to ${number(row.upper)}` },
    ]} />
  </section>
}
type RBounds=Extract<Run['evidence']['longRun'],{readonly kind:'rCalibrated'}>
type Critical=RBounds['fBounds'][number]
function rBoundsFinding(result:RBounds):string{
  const f=result.fBounds.find(row=>row.alpha===0.05)!
  const describe=(family:'F'|'t',statistic:number,bounds:Critical)=>{
    const decision=ardlBoundsDecision(statistic,bounds,family)
    return decision==='Inconclusive'?`The ${family}-bounds test is inconclusive at the 5% level.`:
      decision==='Reject the null'?`The ${family}-bounds test rejects the null of no level relationship at the 5% level.`:
      `The ${family}-bounds test does not reject the null of no level relationship at the 5% level.`
  }
  return describe('F',result.boundsStatistic,f)+(result.tBounds.kind==='recorded'?' '+describe('t',result.tBounds.statistic,result.tBounds.critical.find(row=>row.alpha===0.05)!):'')
}
function RBoundsTable({run}:{readonly run:Run}){
  const result=run.evidence.longRun
  if(result.kind!=='rCalibrated')return null
  type Row={readonly family:'F'|'t';readonly statistic:number;readonly bounds:Critical}
  const t=result.tBounds
  const rows:readonly Row[]=[...result.fBounds.map(bounds=>({family:'F' as const,statistic:result.boundsStatistic,bounds})),
    ...(t.kind==='recorded'?t.critical.map(bounds=>({family:'t' as const,statistic:t.statistic,bounds})):[])]
  return <section className="min-w-0 space-y-3" aria-label="Bounds calibration">
    <p className="m-0 text-body text-muted">F statistic {number(result.boundsStatistic)}; p-value {number(result.fPValue)}.{result.tBounds.kind==='recorded'?` t statistic ${number(result.tBounds.statistic)}; p-value ${number(result.tBounds.pValue)}.`:' The t-bounds test does not apply to the selected restricted deterministic case.'}</p>
    <EvidenceTable frame="none" title="Bounds tests" help="The null hypothesis is no level relationship. The I(0) and I(1) critical values give the bounds for each significance level. The p-values use the I(1) distribution. The F and t tests are reported separately." rows={rows} rowKey={row=>`${row.family}-${row.bounds.alpha}`} noun="test" empty="No calibrated bounds." exportName="ardl-bounds-tests" columns={[
      {id:'test',header:'Test',value:row=>row.family},
      {id:'alpha',header:'Significance level',value:row=>row.bounds.alpha,format:value=>formatPercent(Number(value),{precision:Number.isInteger(Number(value)*100)?0:1}).text,align:'right'},
      figureColumn<Row>('i0','I(0) critical value',row=>row.bounds.i0,number),
      figureColumn<Row>('i1','I(1) critical value',row=>row.bounds.i1,number),
      // A decision is a fixed label; on one line every row keeps the same height.
      {id:'decision',header:'Result',value:row=>ardlBoundsDecision(row.statistic,row.bounds,row.family),format:value=><span className="whitespace-nowrap">{String(value)}</span>},
    ]} />
    <p className="m-0 text-label text-muted">These are asymptotic critical values from the reference calibration with 1,000 observations, not critical values simulated for this sample. A level relationship does not establish a causal effect.</p>
  </section>
}

function MultiplierPlot({run}:{readonly run:Run}) {
  const theme=useChartTheme()
  const [selected,setSelected]=useState(0)
  const [window,setWindow]=useState<VisibleWindow|null>(null)
  const result=run.evidence.multipliers
  const curves=result.kind==='unavailable'?[]:result.curves.filter(curve=>curve.term.kind==='predictor')
  const curve=curves[selected]
  // Both plots share one window, so a zoom re-renders this component; the options must not be rebuilt with it.
  const figures=useMemo(()=>{
    if(result.kind==='unavailable'||curve===undefined)return null
    const name=curve.term.kind==='predictor'?run.predictors[curve.term.column]?.name:'Predictor'
    const axis={kind:'ordinal' as const,values:curve.delay.map((_,i)=>i)}
    return [
      {title:`Response to a one-period increase in ${name}`,series:[{name:'Estimate',values:curve.delay},{name:`${number(result.confidence * 100)}% confidence limits`,values:curve.interval.map(v=>v[0])},{name:`${number(result.confidence * 100)}% confidence limits`,values:curve.interval.map(v=>v[1])}]},
      {title:`Response to a sustained increase in ${name}`,series:[{name:'Cumulative multiplier',values:curve.cumulative}]},
    ].map((figure,i)=>({title:figure.title,option:longRunOption({...figure,axis,startRow:0,slider:i===0,zero:true},theme)}))
  },[result,curve,run.predictors,theme])
  if(result.kind==='unavailable')return <p className="text-body text-muted">Multiplier estimates and intervals could not be calculated from this fitted model.</p>
  if(curve===undefined||figures===null)return null
  return <section className="min-w-0 space-y-3" aria-label="ARDL multipliers"><label><span className={fieldLabel}>Multiplier predictor</span><Select className={field('text','mt-1')} value={selected} onChange={e=>setSelected(Number(e.target.value))}>{curves.map((c,i)=><option key={i} value={i}>{c.term.kind==='predictor'?run.predictors[c.term.column]?.name:'Predictor'}</option>)}</Select></label>
    {figures.map(figure=><div key={figure.title} className="min-w-0 pt-3"><h4 className="m-0 text-label text-ink">{figure.title}</h4><ExpandableChart label={figure.title} option={figure.option} className="h-72" window={window} onWindow={setWindow} /></div>)}
    <p className="m-0 text-label text-muted">The horizontal axis counts periods after a 1-unit increase. Other predictors are held unchanged. The first plot has pointwise {number(result.confidence * 100)}% confidence limits; no interval is calculated for the cumulative response. These are model-based responses, not identified causal effects.</p></section>
}

export function ArdlModelResult({run}:{readonly run:Run}) {
  const e=run.evidence,theme=useChartTheme()
  const [window,setWindow]=useState<VisibleWindow|null>(null)
  const longRun=e.longRun
  const bounds=longRun.kind==='recorded'?longRun.boundsCritical[1]:undefined
  const finding=longRun.kind==='unavailable'?reasons[longRun.reason]:longRun.kind==='rCalibrated'?rBoundsFinding(longRun):longRun.kind==='uncalibrated'?'The level relationship is estimated, but no bounds-test conclusion is reported without critical bounds.':bounds===undefined?'Bounds critical values are unavailable.':longRun.boundsStatistic>bounds[1]?'The bounds test supports a long-run level relationship at the 5% level.':longRun.boundsStatistic<bounds[0]?'The bounds test does not support a long-run level relationship at the 5% level.':'The bounds test is inconclusive at the 5% level.'
  // Both level plots share one window, so a zoom re-renders this component; the options must not be rebuilt with it.
  const plotTime=run.plotTime
  const levelFigures=useMemo(()=>{
    if(longRun.kind==='unavailable'||plotTime===undefined)return null
    return [
      {title:'Observed outcome and estimated long-run level',zero:false,series:[{name:run.outcome.name,values:e.observed},{name:'Estimated long-run level',values:e.observed.map((y,i)=>y-longRun.departures[i]!)}]},
      {title:'Departure from the estimated long-run level',zero:true,series:[{name:'Departure',values:longRun.departures}]},
    ].map((figure,i)=>({title:figure.title,option:longRunOption({...figure,axis:plotTime,startRow:0,slider:i===0},theme)}))
  },[longRun,plotTime,run.outcome.name,e.observed,theme])
  const forecastOption=useMemo(()=>e.forecast.kind!=='recorded'?null:longRunOption({title:'ARDL forecast',axis:{kind:'ordinal',values:e.forecast.mean.map((_,i)=>i+1)},startRow:0,slider:true,zero:false,series:[{name:'Forecast',values:e.forecast.mean},{name:`${number(e.forecast.confidence * 100)}% prediction limits`,values:e.forecast.interval.map(v=>v[0])},{name:`${number(e.forecast.confidence * 100)}% prediction limits`,values:e.forecast.interval.map(v=>v[1])}]},theme),[e.forecast,theme])
  return <section className={resultSurface()} aria-label="Time-series result"><h3 className={`${resultTitle} m-0`}><Metadata><span>ARDL</span><span>{run.outcome.name}</span></Metadata></h3>
    <ResultInterpretation interpretation={{kind:'result-interpretation',statements:[{kind:'magnitude',text:`The model uses ${e.outcomeLag} earlier outcome values and ${e.predictorLags.filter(q=>q!==null).length} retained predictors. ${finding}`}]}} />
    <p className="m-0 text-body text-muted">{e.fittedRows} fitted observations from {e.observations} prepared observations. {run.specification.orders.kind==='fixed'||run.specification.orders.kind==='rFixed'||run.specification.orders.kind==='rRestricted'?'Lag orders were specified before fitting.':'Lag orders were selected by the recorded search.'}</p>
    {run.specification.orders.kind==='rRestricted'&&<p className="m-0 text-body text-muted">Restricted error-correction model. Omitted terms: {run.specification.orders.omitted.map(change=>coefficientName(change.kind==='outcome'?{kind:'outcomeChange',lag:change.lag}:{kind:'predictorChange',column:change.column,lag:change.lag},run)).join('; ')}. The levels coefficients, forecasts and multipliers represent this restricted fit, not an unrestricted refit.</p>}
    <CoefficientTable title="ARDL coefficients" exportName="ardl-coefficients" run={run} terms={e.coefficients} params={e.params} covariance={e.covariance} />
    {longRun.kind==='recorded'&&<p className="m-0 text-body text-muted">Bounds statistic {number(longRun.boundsStatistic)}; 5% bounds {bounds?.map(number).join(' to ')}.</p>}
    <LongRunTable run={run} />
    <RBoundsTable run={run} />
    <RAnalysis run={run} />
    {levelFigures!==null&&<div className="space-y-3">{levelFigures.map(figure=><div key={figure.title} className="min-w-0 pt-3"><h4 className="m-0 text-label text-ink">{figure.title}</h4><ExpandableChart label={figure.title} option={figure.option} className="h-72" window={window} onWindow={setWindow} /></div>)}</div>}
    <MultiplierPlot run={run} />
    {e.forecast.kind==='recorded'&&forecastOption!==null&&<section className="min-w-0 space-y-3" aria-label="ARDL forecast"><h4 className="m-0 text-body text-ink">Forecast with supplied future values</h4><ExpandableChart label="ARDL forecast" className="h-80" option={forecastOption} /><p className="m-0 text-label text-muted">Period 1 is the next period after the prepared sample. These {number(e.forecast.confidence * 100)}% prediction limits account for future innovations conditional on the fitted coefficients and supplied predictor values. They do not include uncertainty in those future values or in the estimated coefficients.</p></section>}
    <TimeSeriesEquation run={run} />
  </section>
}
