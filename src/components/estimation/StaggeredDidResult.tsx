import { useMemo,useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { baseOption,categoryAxis,gridAuto,tooltip,valueAxis } from '@/charts/grammar'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Alert } from '@/components/ui/Alert'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { formatStatistic } from '@/lib/format/number'
import type { StaggeredEvidence,StaggeredFamily,StaggeredInterval } from '@/domain/staggeredDid'

const number=(value:number)=>formatStatistic('raw',value).text
function Effects({family,title,mode,labels,confidence}:{readonly family:StaggeredFamily;readonly title:string;readonly mode:'event'|'calendar'|'cohort';readonly labels:ReadonlyMap<number,string>;readonly confidence:number}) {
  const theme=useChartTheme()
  const coverage=`${Math.round(confidence*100)}% ${family.coverage.kind==='simultaneous'?'simultaneous bands':'pointwise intervals'}`
  const option=useMemo(()=>{
    const horizontal=mode==='cohort'
    const bars:(number[]|null)[]=[],points:{name:string;value:number[];symbol:string;itemStyle:{color:string}}[]=[]
    family.keys.forEach((key,index)=>{
      const interval=family.intervals[index]!
      const estimate=interval.kind==='reference'?0:interval.estimate
      const x=horizontal?index:key
      points.push({name:interval.kind==='reference'?'Normalized reference':interval.kind==='unavailable'?'ATT (uncertainty unavailable)':'ATT',value:horizontal?[estimate,x]:[x,estimate],symbol:interval.kind==='reference'?'emptyCircle':'circle',itemStyle:{color:interval.kind==='reference'?theme.muted:theme.signal}})
      if(interval.kind==='estimated') bars.push(horizontal?[interval.lower,x]:[x,interval.lower],horizontal?[interval.upper,x]:[x,interval.upper],null)
    })
    return {...baseOption(theme,`${title}. ${coverage}. Open circles are normalized reference periods, not estimated zero effects.`),grid:gridAuto({top:26,bottom:28}),tooltip:tooltip(theme,'item'),
      xAxis:horizontal?valueAxis(theme,'ATT'):{...valueAxis(theme,mode==='event'?'Periods since adoption':'Period'),min:Math.min(...family.keys)-0.5,max:Math.max(...family.keys)+0.5,minInterval:1,axisLabel:{...valueAxis(theme).axisLabel,formatter:(v:number)=>!Number.isInteger(v)?'':mode==='calendar'?(labels.get(v)??String(v)):String(v)}},
      yAxis:horizontal?categoryAxis(theme,family.keys.map(k=>labels.get(k)??String(k))):valueAxis(theme,'ATT'),
      series:[{id:'intervals',name:coverage,type:'line',data:bars,connectNulls:false,symbol:'none',lineStyle:{color:theme.signal,width:1.5},silent:true},
        {id:'effects',name:'ATT',type:'scatter',data:points,symbolSize:7,markLine:{silent:true,symbol:'none',label:{show:false},lineStyle:{color:theme.muted,type:'dashed'},data:[horizontal?{xAxis:0}:{yAxis:0},...(mode==='event'?[{xAxis:0}]:[])]}}],
    }
  },[family,title,mode,labels,theme,coverage])
  return <div className="grid gap-2"><p className="m-0 text-body text-muted">{coverage}</p><ExpandableChart option={option} label={title} testId={`staggered-${mode}`} className="h-[300px]" />{family.coverage.kind==='simultaneous'&&family.coverage.largeCritical&&<Alert tone="warn" live={false}>The simultaneous critical value is at least 7. Review the comparison support and overlap.</Alert>}</div>
}

export function StaggeredDidResult({evidence,labels}:{readonly evidence:StaggeredEvidence;readonly labels:readonly string[]}) {
  const [view,setView]=useState<'event'|'cohort'|'calendar'|'cells'>('event')
  const periods=useMemo(()=>new Map(evidence.times.map((t,i)=>[t,labels[i]??String(t)])),[evidence.times,labels])
  const family=view==='cohort'?evidence.cohorts:view==='calendar'?evidence.calendar:evidence.events
  const name=view==='cohort'?'Cohort-average effects':view==='calendar'?'Calendar-average effects':'Event-study effects'
  const facets=useMemo(()=>[...new Set(evidence.cells.keys.map(([g])=>g))].map(cohort=>{
    const indices=evidence.cells.keys.flatMap(([g],i)=>g===cohort?[i]:[])
    return {cohort,family:{keys:indices.map(i=>evidence.cells.keys[i]![1]),intervals:indices.map(i=>evidence.cells.intervals[i]!),coverage:evidence.cells.coverage,analyticalCovariance:indices.map(i=>indices.map(j=>evidence.cells.analyticalCovariance[i]![j]!))}}
  }),[evidence.cells])
  const rows=view==='cells'?evidence.cells.keys.map(([g,t],i)=>({key:`${g}:${t}`,label:`${periods.get(g)??g} / ${periods.get(t)??t}`,interval:evidence.cells.intervals[i]!,cohorts:0,units:0}))
    :family.keys.map((key,i)=>({key:String(key),label:view==='event'?String(key):(periods.get(key)??String(key)),interval:family.intervals[i]!,cohorts:evidence.support[i]?.cohorts.length??0,units:evidence.support[i]?.treatedUnits??0}))
  const field=(interval:StaggeredInterval,key:'estimate'|'lower'|'upper'|'standardError')=>interval.kind==='reference'?(key==='estimate'?'Reference':'Not estimated'):interval.kind==='unavailable'?(key==='estimate'?interval.estimate:'Unavailable'):interval[key]
  const print=(value:string|number)=>typeof value==='number'?number(value):value
  return <div className="mt-3 grid gap-3" data-testid="staggered-did-result">
    <SegmentedControl variant="line" ariaLabel="Staggered DiD results" value={view} onChange={setView} options={[{value:'event',label:'Event study'},{value:'cohort',label:'Cohorts'},{value:'calendar',label:'Calendar'},{value:'cells',label:'Group-time'}]} />
    {view==='cells'?<div className="grid gap-4 @3xl/panel:grid-cols-2">{facets.map(f=><section key={f.cohort}><h4 className="mb-2">Cohort {periods.get(f.cohort)??f.cohort}</h4><Effects family={f.family} title={`Cohort ${periods.get(f.cohort)??f.cohort} group-time ATT`} mode="calendar" labels={periods} confidence={evidence.specification.confidence} /></section>)}</div>:<Effects family={family} title={name} mode={view} labels={periods} confidence={evidence.specification.confidence} />}
    <EvidenceTable title={view==='cells'?'Group-time ATT':name} rows={rows} rowKey={r=>r.key} noun="effect" empty="No supported effects." exportName="staggered-did-effects" columns={[
      {id:'key',header:view==='cells'?'Cohort / period':view==='event'?'Event time':view==='cohort'?'Cohort':'Period',value:r=>r.label},
      {id:'estimate',header:'ATT',align:'right',value:r=>field(r.interval,'estimate'),format:print},{id:'se',header:'Standard error',align:'right',value:r=>field(r.interval,'standardError'),format:print},
      {id:'lower',header:'Lower',align:'right',value:r=>field(r.interval,'lower'),format:print},{id:'upper',header:'Upper',align:'right',value:r=>field(r.interval,'upper'),format:print},
      ...(view==='event'?[{id:'cohorts',header:'Cohorts',align:'right' as const,value:(r:typeof rows[number])=>r.cohorts},{id:'units',header:'Treated units',align:'right' as const,value:(r:typeof rows[number])=>r.units}]:[]),
    ]} />
    {evidence.changes.length>0&&<EvidenceTable title="Panel preparation" rows={evidence.changes.map((c,i)=>({key:String(i),...c}))} rowKey={r=>r.key} noun="change" empty="No changes." columns={[{id:'record',header:'Unit or period',value:r=>'unit' in r?r.unit:(periods.get(r.period)??String(r.period))},{id:'change',header:'Change',value:r=>({beyondObservedWindow:'Adoption beyond observed window; used as comparison',latestCohortAsComparison:'Latest cohort used as comparison',noUntreatedComparison:'Period removed: no untreated comparison',noPreTreatment:'Unit removed: no untreated baseline'})[r.kind]}]} />}
    {evidence.smallCohorts.length>0&&<Alert tone="warn" live={false}>At least one cohort has fewer than the reference requirement of covariate count plus five units. Review the cohort support.</Alert>}
    {evidence.fits.some(f=>f.status.kind==='iterationLimit')&&<Alert tone="warn" live={false}>At least one propensity fit reached its iteration limit. Review the covariates and treatment overlap.</Alert>}
    {evidence.fits.length>0&&<details><DisclosureSummary className="cursor-pointer text-body text-ink">Propensity fits</DisclosureSummary><EvidenceTable title="Propensity fits" rows={evidence.fits} rowKey={r=>`${r.cohort}:${r.period}`} noun="fit" empty="No adjustment models." columns={[{id:'cohort',header:'Cohort',value:r=>periods.get(r.cohort)??String(r.cohort)},{id:'period',header:'Period',value:r=>periods.get(r.period)??String(r.period)},{id:'status',header:'Termination',value:r=>r.status.kind==='converged'?`Converged in ${r.status.iterations} iterations`:'Iteration limit'}]} /></details>}
  </div>
}
