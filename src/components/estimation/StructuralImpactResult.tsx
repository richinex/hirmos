import { useMemo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { baseOption, categoryAxis, gridAuto, tooltip, valueAxis } from '@/charts/grammar'
import { Select } from '@/components/ui/Select'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldLabel } from '@/components/ui/recipes'
import type { CausalImpactEvidence } from '@/domain/estimation'

type Evidence = Extract<CausalImpactEvidence,{kind:'structuralCausalImpact'}>
export function StructuralImpactResult({ evidence,columnNames }: {readonly evidence:Evidence;readonly columnNames:readonly string[]}) {
  const [selected,setSelected] = useState('0')
  const theme = useChartTheme()
  const contribution = evidence.contributions[Number(selected)] ?? evidence.contributions[0]
  const label = (c:Evidence['contributions'][number]) => c.component.kind === 'trend' ? 'Trend' : c.component.kind === 'seasonal' ? 'Seasonality' : columnNames[c.component.column] ?? `Predictor ${c.component.column+1}`
  const name = label(contribution)
  const option = useMemo(() => ({
    ...baseOption(theme,`${name}: posterior mean and pointwise 95% interval. This is a model contribution, not a separate causal effect.`),
    grid:gridAuto({top:20,bottom:45}),tooltip:tooltip(theme,'axis'),
    xAxis:categoryAxis(theme,contribution.mean.map((_,i)=>String(i+1)),'Observation'),
    yAxis:valueAxis(theme,name),
    series:[
      {id:'component-lower',name:'Lower 95% bound',type:'line',data:contribution.lower,symbol:'none',lineStyle:{color:theme.muted,width:1,type:'dashed'}},
      {id:'component-upper',name:'Upper 95% bound',type:'line',data:contribution.upper,symbol:'none',lineStyle:{color:theme.muted,width:1,type:'dashed'}},
      {id:'component-mean',name,type:'line',data:contribution.mean,symbol:'none',lineStyle:{color:theme.signal,width:2},
        markLine:{silent:true,symbol:'none',label:{show:false},data:[{xAxis:String(evidence.nPre+1)}],lineStyle:{color:theme.muted,type:'dashed'}}},
    ],
  }),[theme,name,contribution,evidence.nPre])
  return <div className="grid gap-3">
    <label>
      <ParameterLabel className={fieldLabel} label="Model contribution" help="The components sum to the latent prediction. Predictor contributions use training-centred values; the trend includes the response baseline. Intervals describe each component separately and must not be added. The vertical marker starts the evaluation window." />
      <Select aria-label="Model contribution" className={field('text','mt-1 w-full')} value={selected} onChange={e=>setSelected(e.target.value)}>
        {evidence.contributions.map((c,i)=><option key={i} value={String(i)}>{label(c)}</option>)}
      </Select>
    </label>
    <ExpandableChart option={option} label={`${name} contribution`} className="h-[280px]" testId="structural-impact-components" />
  </div>
}
