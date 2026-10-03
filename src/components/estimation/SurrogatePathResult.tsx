import {useMemo} from 'react'
import type {SurrogatePathEvidence} from '@/domain/surrogatePath'
import {ExpandableChart} from '@/charts/ExpandableChart'
import {useChartTheme} from '@/charts/theme'
import {axisLabelStyle,baseOption,valueAxis,gridAuto,legend,tooltip,rangeSelection} from '@/charts/grammar'
import {EvidenceTable,figureColumn} from '@/components/table/EvidenceTable'
import {formatCount,formatStatistic} from '@/lib/format/number'
import {fieldHint} from '@/components/ui/recipes'

// Means above, contrasts below, on one period axis and one zoom window.
function ObservedPathChart({labels,treated,control,contrast,cumulative}:{readonly labels:readonly string[];readonly treated:readonly number[];readonly control:readonly number[];readonly contrast:readonly number[];readonly cumulative:readonly number[]}){
  const theme=useChartTheme()
  // The slider takes the bottom 6–24px; axis labels need about 20px above it with an 8px gap.
  const PANEL=200,GAP=44,TOP=40,BOTTOM=56
  const height=TOP+2*PANEL+GAP+BOTTOM
  const title='Observed outcome means, and treated minus control, by period'
  const option=useMemo(()=>{
    const line=(name:string,axis:number,data:readonly number[],color:string,type:'solid'|'dashed')=>({name,type:'line',xAxisIndex:axis,yAxisIndex:axis,data,showSymbol:labels.length<12,symbolSize:5,lineStyle:{color,width:1.5,type},itemStyle:{color}})
    const names=['Treated','Control','Period contrast','Cumulative mean']
    return {...baseOption(theme,title),tooltip:tooltip(theme,'axis'),
      legend:{...legend(theme,names),top:0,bottom:'auto',left:'center',width:'72%'},
      grid:[0,1].map(i=>({left:72,right:18,top:TOP+i*(PANEL+GAP),height:PANEL})),
      axisPointer:{link:[{xAxisIndex:'all'}]},
      xAxis:[0,1].map(i=>({type:'category',gridIndex:i,data:labels,boundaryGap:false,axisLabel:{...axisLabelStyle(theme),show:i===1}})),
      yAxis:['Mean outcome','Treated minus control'].map((name,i)=>({...valueAxis(theme,name),gridIndex:i})),
      ...rangeSelection(theme,[0,1]),
      series:[line('Treated',0,treated,theme.ink,'solid'),line('Control',0,control,theme.info,'dashed'),
        line('Period contrast',1,contrast,theme.categorical[1]!,'dashed'),line('Cumulative mean',1,cumulative,theme.categorical[2]!,'solid')]}
  },[labels,treated,control,contrast,cumulative,theme])
  return <ExpandableChart option={option} label={title} testId="surrogate-observed-path" style={{height}} className=""/>
}

// Windows sit at their surrogate counts, so unequal groups keep their true spacing.
function WindowChart({windows}:{readonly windows:readonly {readonly label:string;readonly surrogateColumns:number;readonly estimate:number}[]}){
  const theme=useChartTheme()
  const title='Long-term treatment effect by surrogate window'
  const option=useMemo(()=>({...baseOption(theme,title),grid:gridAuto({top:38,bottom:48}),legend:{...legend(theme,['Long-term effect']),top:0,bottom:'auto',left:'center',width:'72%'},
    tooltip:{...tooltip(theme,'axis'),formatter:(items:readonly {readonly dataIndex:number}[])=>{const w=windows[items[0]?.dataIndex??0];return w===undefined?'':`${w.label}<br/>Surrogates: ${formatCount(w.surrogateColumns).text}<br/>Long-term effect: ${formatStatistic('raw',w.estimate).text}`}},
    xAxis:{...valueAxis(theme,'Surrogates included'),min:0,max:'dataMax',minInterval:1,splitLine:{show:false}},yAxis:valueAxis(theme),...rangeSelection(theme,[0],{slider:false}),
    series:[{name:'Long-term effect',type:'line',data:windows.map(w=>[w.surrogateColumns,w.estimate]),showSymbol:true,symbolSize:5,lineStyle:{color:theme.ink,width:1.5},itemStyle:{color:theme.ink}}]}),[windows,theme])
  return <ExpandableChart option={option} label={title} testId="surrogate-window-estimates" className="h-[300px]"/>
}

export function SurrogatePathResult({path}:{readonly path:SurrogatePathEvidence}){
  if(path.kind==='observedOutcomes'){
    const uncertainty=path.cumulativeUncertainty
    const rows=path.periods.map((p,i)=>({...p,bootstrapSe:uncertainty.kind==='bootstrapStandardErrors'?uncertainty.standardErrors[i]!:null}))
    return <section aria-label="Observed experimental outcome paths" className="grid min-w-0 grid-cols-[minmax(0,1fr)] gap-4">
      <div><h4 className="m-0 text-body font-medium text-ink">Observed experimental outcomes</h4><p className={fieldHint}>{formatCount(path.treatedRows).text} treated and {formatCount(path.controlRows).text} control participants at every period.</p></div>
      <ObservedPathChart labels={rows.map(p=>p.label)} treated={rows.map(p=>p.treatedMean)} control={rows.map(p=>p.controlMean)} contrast={rows.map(p=>p.contrast)} cumulative={rows.map(p=>p.cumulativeMeanContrast)}/>
      <EvidenceTable frame="none" title="Observed outcome periods" help={`The cumulative mean is the average treated-minus-control difference through each period.${uncertainty.kind==='bootstrapStandardErrors'?` Its standard errors use ${formatCount(uncertainty.repetitions).text} bootstrap repetitions that resample whole participant paths within each treatment arm.`:''}`} rows={rows} rowKey={r=>r.label} noun="period" empty="No observed periods." columns={[{id:'label',header:'Period',value:r=>r.label},figureColumn<typeof rows[number]>('treated','Treated mean',r=>r.treatedMean),figureColumn<typeof rows[number]>('control','Control mean',r=>r.controlMean),figureColumn<typeof rows[number]>('contrast','Period contrast',r=>r.contrast),figureColumn<typeof rows[number]>('cumulative','Cumulative mean contrast',r=>r.cumulativeMeanContrast),...(uncertainty.kind==='none'?[]:[{id:'cumulative-se',header:'Cumulative mean bootstrap SE',align:'right' as const,value:(r:typeof rows[number])=>r.bootstrapSe??'Not recorded',format:(v:unknown)=>typeof v==='number'?formatStatistic('raw',v).text:'Not recorded'}])]}/>
    </section>
  }
  const rows=path.windows
  const bootstrap=rows[0]!.evidence.uncertainty.kind!=='none'
  return <section aria-label="Surrogate window estimates" className="grid min-w-0 grid-cols-[minmax(0,1fr)] gap-4">
    <h4 className="m-0 text-body font-medium text-ink">Long-term estimates by surrogate window</h4>
    <WindowChart windows={rows.map(w=>({label:w.label,surrogateColumns:w.evidence.surrogateColumns,estimate:w.evidence.estimate}))}/>
    <EvidenceTable frame="none" title="Surrogate windows" help="Each estimate uses the surrogates available through that window for the same long-term outcome. It is not the effect on an outcome measured at that time." rows={rows} rowKey={r=>r.label} noun="window" empty="No surrogate windows." columns={[{id:'label',header:'Window',value:r=>r.label},{id:'columns',header:'Surrogates',align:'right',value:r=>r.evidence.surrogateColumns,format:v=>formatCount(Number(v)).text},figureColumn<typeof rows[number]>('effect','Long-term effect',r=>r.evidence.estimate),...(bootstrap?[{id:'se',header:'Bootstrap SE',align:'right' as const,value:(r:typeof rows[number])=>r.evidence.uncertainty.kind==='none'?0:r.evidence.uncertainty.standardError,format:(v:unknown)=>formatStatistic('raw',Number(v)).text}]:[])]}/>
  </section>
}
