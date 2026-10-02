import { useState } from 'react'
import type { TimeSeriesRun } from '@/domain/timeSeries'
import { useChartTheme } from '@/charts/theme'
import { longRunOption } from '@/charts/data/longRun'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { Select } from '@/components/ui/Select'
import { field,fieldLabel } from '@/components/ui/recipes'

export function VecmForecastChart({run}:{readonly run:Extract<TimeSeriesRun,{kind:'vecm'}>}) {
  const theme=useChartTheme(),[selected,setSelected]=useState(0)
  const forecast=run.evidence.forecast
  if(forecast===undefined||forecast.kind==='notRequested')return null
  if(forecast.kind==='notFitted')return <p className="text-body text-muted">No forecast was calculated because the rank test selected zero and this analysis did not fit a VECM coefficient model.</p>
  const name=run.variables[selected]?.name
  return <section className="min-w-0 space-y-3" aria-label="VECM forecast"><label><span className={fieldLabel}>Forecast series</span><Select className={field('text','mt-1')} value={selected} onChange={e=>setSelected(Number(e.target.value))}>{run.variables.map((v,i)=><option key={v.id} value={i}>{v.name}</option>)}</Select></label><ExpandableChart label={`Forecast of ${name}`} className="h-80" option={longRunOption({title:`Forecast of ${name}`,axis:{kind:'ordinal',values:forecast.mean.map((_,i)=>i+1)},startRow:0,slider:true,zero:false,series:[{name:'Forecast',values:forecast.mean.map(r=>r[selected]!)},{name:'95% prediction limits',values:forecast.lower.map(r=>r[selected]!)},{name:'95% prediction limits',values:forecast.upper.map(r=>r[selected]!)}]},theme)} /><p className="m-0 text-label text-muted">Period 1 is the next period after the prepared sample. The series are forecast jointly. The pointwise 95% prediction limits account for future innovations conditional on the fitted model; they do not include coefficient-estimation uncertainty.</p></section>
}
