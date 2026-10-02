import { useMemo } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { baseOption,gridAuto,tooltip,valueAxis,categoryAxis } from '@/charts/grammar'
import { useChartTheme } from '@/charts/theme'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'
import { MetricGrid,MetricTile } from '@/components/ui/figures'
import { formatStatistic,formatWords } from '@/lib/format/number'
import { resultSurface,resultTitle } from '@/components/ui/recipes'
import type { TimeSeriesRun } from '@/domain/timeSeries'

export function CountRegressionResult({run}:{readonly run:Extract<TimeSeriesRun,{kind:'count-regression'}>}) {
  const e=run.evidence,theme=useChartTheme(),events=e.request.design.kind==='events'
  const points=useMemo(()=>events?e.eventPeriods.map(([index,time])=>({...e.terms[index]!,time})):e.contrasts.map((t,time)=>({...t,time})),[e,events])
  const option=useMemo(()=>{
    const intervals:(number[]|null)[]=points.flatMap(t=>[[t.time,t.lower],[t.time,t.upper],null])
    return {...baseOption(theme,'Log-scale regression coefficients with pointwise normal-Wald confidence intervals.'),grid:gridAuto({top:24,bottom:30}),tooltip:tooltip(theme,'item'),
      xAxis:events?{...valueAxis(theme,'Periods since onset'),minInterval:1}:categoryAxis(theme,points.map(t=>t.name)),yAxis:valueAxis(theme,'Log-scale coefficient'),
      series:[{id:'intervals',name:'Pointwise confidence interval',type:'line',data:intervals,symbol:'none',connectNulls:false,lineStyle:{color:theme.signal,width:1.5}},
        {id:'estimates',name:'Coefficient',type:'scatter',symbolSize:7,data:points.map(t=>[t.time,t.estimate]),itemStyle:{color:theme.signal},markLine:{silent:true,symbol:'none',label:{show:false},data:[{yAxis:0}]}},
        ...(events?[{id:'reference',name:'Omitted reference',type:'scatter',symbol:'emptyCircle',symbolSize:7,data:[[-1,0]],itemStyle:{color:theme.muted}}]:[])],
    }
  },[theme,points,events])
  const fittedOption=useMemo(()=>({
    ...baseOption(theme,'Observed counts and fitted mean counts, including the intervention terms.'),
    grid:gridAuto({top:24,bottom:30}),tooltip:tooltip(theme,'axis'),xAxis:valueAxis(theme,'Observation'),yAxis:valueAxis(theme,'Count'),
    series:[{id:'observed',name:'Observed',type:'scatter',symbolSize:5,data:e.observed.map((v,i)=>[e.retained[i]!+1,v]),itemStyle:{color:theme.muted}},
      {id:'fitted',name:'Fitted mean',type:'line',showSymbol:false,data:e.fitted.map((v,i)=>[e.retained[i]!+1,v]),lineStyle:{color:theme.signal,width:2},
        ...(e.request.design.kind==='interrupted'?{markLine:{silent:true,symbol:'none',data:[{xAxis:e.request.design.intervention+1,name:'Intervention'}]}}:{})}],
  }),[e,theme])
  const number=(value:number|string)=>typeof value==='number'?formatStatistic('raw',value).text:value
  const ratio=e.request.family.kind==='binomial'?'Odds ratio':'Rate ratio'
  return <section className={resultSurface()} aria-label="Count regression result">
    <h3 className={resultTitle}>{e.request.design.kind==='events'?'Event study, negative-binomial model':e.request.design.kind==='summary'?'Cohort summary, negative-binomial model':e.request.family.kind==='binomial'?'Grouped-binomial regression':'Negative-binomial regression'} for {run.outcome.name}</h3>
    {e.status.kind!=='converged'&&<Alert tone="warn" live={false}>{e.status.kind==='lineSearchFailed'?'The optimizer line search failed.':'The optimizer reached its iteration limit.'} Review the fit before interpreting its coefficients or intervals.</Alert>}
    <MetricGrid label="Count regression summary"><MetricTile label="Retained observations" value={formatWords(String(e.observations))}/><MetricTile label="Omitted observations" value={formatWords(String(e.omitted.length))}/><MetricTile label="Parameters" value={formatWords(String(e.parameters))}/></MetricGrid>
    <p className="m-0 text-body text-muted">{e.request.design.kind==='interrupted'?'NB2 with a log link, level and slope changes, and Bartlett HAC covariance.':'Team and period fixed effects with team-clustered covariance and a finite-sample correction.'} Intervals are pointwise normal-Wald intervals, not simultaneous bands.</p>
    {(e.request.design.kind==='events'||e.request.design.kind==='summary')&&<p className="m-0 text-body text-muted">{e.request.design.kind==='summary'?'The post-adoption contrast compares the selected cohort with never-treated units.':e.request.design.window.kind==='all'?'Each coefficient is relative to the period before adoption.':'Rows outside the specified event window remain in the fit and share the omitted category with the period before adoption.'} Other adopting cohorts are excluded; never-adopting teams form the comparison group. These are log rate contrasts, not Callaway–Sant’Anna ATT estimates.</p>}
    <p className="m-0 text-body text-muted">The regression describes the specified association. A causal interpretation also requires the study’s identification assumptions.</p>
    {points.length>0&&<ExpandableChart option={option} label={events?'Cohort event study':'Regression contrast'} className="h-[300px]"/>}
    {e.request.design.kind==='interrupted'&&<ExpandableChart option={fittedOption} label="Observed and fitted counts" className="h-[300px]"/>}
    {e.contrasts.length>0&&<EvidenceTable title="Joint contrast" rows={e.contrasts} rowKey={r=>r.name} noun="contrast" empty="No contrast" columns={[
      {id:'name',header:'Contrast',value:r=>r.name},{id:'estimate',header:'Log coefficient',value:r=>r.estimate,format:number,align:'right'},
      {id:'ratio',header:ratio,value:r=>r.ratio,format:number,align:'right'},{id:'lower',header:'Lower',value:r=>r.ratioLower,format:number,align:'right'},{id:'upper',header:'Upper',value:r=>r.ratioUpper,format:number,align:'right'},
    ]}/>}
    <EvidenceTable title="Coefficients" rows={e.terms} rowKey={r=>r.name} noun="coefficient" empty="No coefficients" exportName="count-regression-coefficients" columns={[
      {id:'term',header:'Term',value:r=>r.name},{id:'estimate',header:'Log coefficient',value:r=>r.estimate,format:number,align:'right'},
      {id:'se',header:'Standard error',value:r=>r.standardError,format:number,align:'right'},
      {id:'lower',header:'Lower',value:r=>r.lower,format:number,align:'right'},{id:'upper',header:'Upper',value:r=>r.upper,format:number,align:'right'},
    ]}/>
  </section>
}
