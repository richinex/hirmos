import { useMemo } from 'react'
import type { RidgeAugmentedEvidence } from '@/domain/remixExtensions'
import { EvidenceTable,figureColumn } from '@/components/table/EvidenceTable'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { axisLabelStyle,baseOption,legend,rangeSelection,tooltip,valueAxis } from '@/charts/grammar'
import { periodLabel } from '@/domain/periodLabels'
import { fieldHint } from '@/components/ui/recipes'
export function RidgeAugmentedResult({evidence:e,sourcePeriods,outcome}:{readonly evidence:RidgeAugmentedEvidence;readonly sourcePeriods:readonly {readonly code:number;readonly label:string}[];readonly outcome:string}){
  const theme=useChartTheme()
  const labels=useMemo(()=>new Map(sourcePeriods.map(p=>[p.code,p.label])),[sourcePeriods])
  // Two panels on one period axis, as in the other stacked plots: linked pointers and one range slider.
  const PANEL=200,GAP=44,TOP=40,BOTTOM=100
  const height=TOP+2*PANEL+GAP+BOTTOM
  const chart=useMemo(()=>{
    const periods=e.periods.map(t=>periodLabel(t,labels)),event=periods[e.request.prePeriods]
    const markLine={silent:true,symbol:'none',label:{show:false},lineStyle:{color:theme.muted,type:'dashed'},data:[{xAxis:event}]}
    const intervals:(number[]|null)[]=[]
    const bounds=e.bounds
    if(bounds.kind==='jackknife')bounds.lower.forEach((v,i)=>intervals.push([e.request.prePeriods+i,v],[e.request.prePeriods+i,bounds.upper[i]!],null))
    const names=['Observed','Synthetic','Observed minus synthetic',...(intervals.length>0?['Pointwise interval']:[])]
    return {...baseOption(theme,`Ridge-augmented synthetic control for ${e.request.treated}. The vertical line marks intervention.`),
      legend:{...legend(theme,names),top:0,bottom:'auto'},
      grid:[0,1].map(i=>({left:72,right:18,top:TOP+i*(PANEL+GAP),height:PANEL})),
      axisPointer:{link:[{xAxisIndex:'all'}]},
      tooltip:tooltip(theme,'axis'),
      xAxis:[0,1].map(i=>({type:'category',gridIndex:i,data:periods,boundaryGap:false,axisLabel:{...axisLabelStyle(theme),show:i===1}})),
      yAxis:['Observed and synthetic','Observed minus synthetic'].map((name,i)=>({...valueAxis(theme,name),gridIndex:i})),
      ...rangeSelection(theme,[0,1]),
      series:[
        {name:'Observed',type:'line',xAxisIndex:0,yAxisIndex:0,data:e.observed,showSymbol:false,lineStyle:{color:theme.ink,width:1.5},itemStyle:{color:theme.ink},markLine},
        {name:'Synthetic',type:'line',xAxisIndex:0,yAxisIndex:0,data:e.synthetic,showSymbol:false,lineStyle:{color:theme.info,width:1.5,type:'dashed'},itemStyle:{color:theme.info}},
        {name:'Observed minus synthetic',type:'line',xAxisIndex:1,yAxisIndex:1,data:e.gaps,showSymbol:false,lineStyle:{color:theme.info,width:1.5},itemStyle:{color:theme.info},markLine:{...markLine,data:[{xAxis:event},{yAxis:0}]}},
        {name:'Pointwise interval',type:'line',xAxisIndex:1,yAxisIndex:1,data:intervals,symbol:'none',connectNulls:false,lineStyle:{color:theme.signal,width:1.5},itemStyle:{color:theme.signal},silent:true},
      ]}
  },[e,labels,theme])
  const donors=e.request.donors.map((unit,i)=>({unit,weight:e.donorWeights[i]!,baseline:e.baselineWeights[i]!}))
  const tuning=e.tuning
  const candidates=tuning===null?[]:tuning.candidates.map((lambda,i)=>({lambda,error:tuning.errors[i]!,se:tuning.standardErrors[i]!}))
  return <div className="grid min-w-0 grid-cols-[minmax(0,1fr)] gap-6" data-testid="ridge-augmented-result">
    <p className={fieldHint}>Treated unit: {e.request.treated}. The headline is the average observed-minus-synthetic gap over the post-treatment periods. Augmented weights can be negative.</p>
    <ExpandableChart option={chart} label={`Observed and synthetic ${outcome}, and their difference`} testId="ridge-augmented-path" style={{height}} className=""/>
    <p className={fieldHint}>{e.request.uncertainty.kind==='none'?'No interval was requested.':`${Math.round(e.request.uncertainty.confidence*100)}% pointwise ${e.request.uncertainty.kind==='jackknifePlus'?'jackknife+':'conservative jackknife'} intervals. The headline interval applies to the average gap, not its cumulative sum.`}</p>
    <EvidenceTable title="Donor weights" rows={donors} rowKey={r=>r.unit} noun="donor" empty="No donor weights." columns={[{id:'unit',header:'Donor unit',value:r=>r.unit},figureColumn<typeof donors[number]>('weight','Augmented weight',r=>r.weight),figureColumn<typeof donors[number]>('baseline','Baseline weight',r=>r.baseline)]}/>
    {candidates.length>0&&<EvidenceTable title="Cross-validation" rows={candidates} rowKey={r=>String(r.lambda)} noun="candidate" empty="No cross-validation." columns={[figureColumn<typeof candidates[number]>('lambda','Lambda',r=>r.lambda),figureColumn<typeof candidates[number]>('error','Validation error',r=>r.error),figureColumn<typeof candidates[number]>('se','Standard error',r=>r.se)]}/>}
  </div>
}
