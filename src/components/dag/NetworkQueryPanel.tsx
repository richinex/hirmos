import {useMemo,useState} from 'react'
import {RunFold} from '@/components/ui/RunFold'
import {formatTime} from '@/lib/format/date'
import {DEFAULT_BDEU_EQUIVALENT_SAMPLE_SIZE} from '@/domain/discreteDefaults'
import type {DagDocument,DagNodeId} from '@/domain/dag'
import type {DatasetProfile} from '@/domain/dataset'
import type {PreparedDatasetArtifact} from '@/domain/preprocessing'
import type {SelectedSource} from '@/domain/workflow'
import {newInterventionQueryId} from '@/domain/intervention'
import {describeNetworkQuery,networkQuerySchema,type NetworkQueryArtifact} from '@/domain/networkQuery'
import {assertNever,isNonEmpty} from '@/domain/dop'
import {useJob} from '@/analysis/JobsProvider'
import {describeAnalysisWorkerProblem} from '@/workers/analysisProtocol'
import {Select} from '@/components/ui/Select'
import {Alert} from '@/components/ui/Alert'
import {JobNotice} from '@/components/ui/JobNotice'
import {button,field,fieldHint,fieldLabel,well} from '@/components/ui/recipes'
import {EChart} from '@/charts/EChart'
import {useChartTheme,seriesColour} from '@/charts/theme'
import {baseOption,categoryAxis,gridAuto,valueAxis,tooltip} from '@/charts/grammar'

type Role={readonly kind:'unused'}|{readonly kind:'outcome'}|{readonly kind:'observe';readonly state:number}|{readonly kind:'intervene';readonly state:number}

function Distribution({record,open,onDelete}:{readonly record:NetworkQueryArtifact;readonly open:boolean;readonly onDelete:(record:NetworkQueryArtifact)=>void}) {
  const theme=useChartTheme()
  const description=describeNetworkQuery(record.specification)
  const option=useMemo(()=>({...baseOption(theme,description),grid:gridAuto({top:16,bottom:8}),tooltip:tooltip(theme,'axis'),
    xAxis:categoryAxis(theme,record.result.distribution.map(([s])=>s.join(', ')),'Outcome states'),yAxis:{...valueAxis(theme,'Probability'),min:0,max:1},
    series:[{type:'bar' as const,data:record.result.distribution.map(([,p])=>p),barMaxWidth:28,itemStyle:{color:seriesColour(theme,0)}}],
  }),[theme,description,record.result])
  return <RunFold title={description} stamp={formatTime(record.createdAt)} defaultOpen={open} onDelete={()=>onDelete(record)} deleteLabel="Delete this query">
    <p className={fieldHint}>{record.specification.interventions.length?'Interventional':'Observational'} distribution. {record.specification.equivalentSampleSize===0?'Maximum-likelihood':`BDeu (equivalent sample size ${record.specification.equivalentSampleSize})`} conditional probability tables; {record.result.observations} rows. No uncertainty interval is reported.</p>
    <EChart option={option} label={description} className="h-[200px]" />
    <div className="overflow-auto"><table className="w-full text-label tabular-nums"><thead><tr>{record.specification.outcomes.map(i=><th key={i} className="text-left font-medium">{record.specification.names[i]}</th>)}<th className="text-right font-medium">Probability</th></tr></thead>
      <tbody>{record.result.distribution.map(([states,p])=><tr key={states.join(':')}>{states.map((s,i)=><td key={i}>{s}</td>)}<td className="text-right">{p.toFixed(6)}</td></tr>)}</tbody></table></div>
    <p className={fieldHint}>State labels refer to the fitted discrete model. Interventions set values; observations condition on evidence. Causal interpretation depends on the graph and its assumptions.</p>
    <details><summary className="text-label text-muted">State definitions</summary>{record.result.states.map((states,i)=><p key={i} className={fieldHint}>{record.specification.names[i]}: {states.map(([s,v])=>`${s} = ${v}`).join('; ')}</p>)}</details>
  </RunFold>
}

export function NetworkQueryPanel({document,source,profile,prepared,records,onQuery,onDelete}:{
  readonly document:DagDocument;readonly source:SelectedSource;readonly profile:DatasetProfile;readonly prepared:PreparedDatasetArtifact
  readonly records:readonly NetworkQueryArtifact[];readonly onQuery:(q:NetworkQueryArtifact)=>void;readonly onDelete:(q:NetworkQueryArtifact)=>void
}) {
  const [roles,setRoles]=useState<ReadonlyMap<DagNodeId,Role>>(new Map())
  const [bins,setBins]=useState(3)
  const [ess,setEss]=useState(0)
  const session=useJob(`network-query:${document.current.id}`)
  const nodes=document.current.graph.nodes
  const problem=nodes.some(n=>n.kind==='latent')?'Distribution queries currently require a fully observed DAG. Use the contrast view for supported ID/IDC queries with unmeasured variables.':document.current.validation.structure.kind==='invalid'?'Resolve the graph’s structural issues before running a query.':null
  const change=(id:DagNodeId,role:Role)=>setRoles(old=>new Map(old).set(id,role))
  const outcomes:number[]=[],observations:{variable:number;state:number}[]=[],interventions:{variable:number;state:number}[]=[]
  nodes.forEach((n,i)=>{const r=roles.get(n.id)??{kind:'unused'};switch(r.kind){
    case 'unused':break
    case 'outcome':outcomes.push(i);break
    case 'observe':observations.push({variable:i,state:r.state});break
    case 'intervene':interventions.push({variable:i,state:r.state});break
    default:assertNever(r)
  }})
  const run=async()=>{
    if(problem!==null||outcomes.length===0)return
    const current=session.start('analysis','Probability query');if(current===null)return
    try{
      const [{materialisePrepared,describePreparedMaterialisationProblem},{runNetworkQuery}]=await Promise.all([import('@/data/prepared'),import('@/analysis/client')])
      const columns=nodes.flatMap(n=>n.kind==='observed'?[n.column]:[])
      if(!isNonEmpty(columns)){session.fail(current,'Select at least one measured variable.');return}
      const matrix=await materialisePrepared(source,profile,prepared,columns)
      if(!session.current(current))return
      if(!matrix.ok){session.fail(current,describePreparedMaterialisationProblem(matrix.error));return}
      const specification=networkQuerySchema.safeParse({rows:matrix.value.rowCount,columns:nodes.length,names:nodes.map(n=>n.name),edges:document.current.graph.edges.map(e=>[nodes.findIndex(n=>n.id===e.cause),nodes.findIndex(n=>n.id===e.effect)]),outcomes,observations,interventions,bins,equivalentSampleSize:ess})
      if(!specification.success){session.fail(current,specification.error.message);return}
      const result=await runNetworkQuery(matrix.value.values,specification.data)
      if(!session.current(current))return
      if(!result.ok){session.fail(current,describeAnalysisWorkerProblem(result.error));return}
      onQuery({kind:'network-query',id:newInterventionQueryId(),dagDocument:document.id,dagRevision:document.current.id,preparedDataset:prepared.id,createdAt:new Date().toISOString(),specification:specification.data,result:result.value})
      session.finish(current)
    }catch(error){session.fail(current,error instanceof Error?error.message:String(error))}
  }
  return <div className="mt-4">
    <p className={fieldHint}>Choose one or more outcomes. Observe values to condition on them, or intervene to set them. Leave interventions empty for an observational query. Each variable has one role.</p>
    {nodes.map(n=>{const r=roles.get(n.id)??{kind:'unused'};return <div key={n.id} className="mb-3 grid grid-cols-2 gap-2">
      <label className="min-w-0"><span className={fieldLabel}>{n.name}</span><Select className={field('text')} aria-label={`${n.name} query role`} value={r.kind} onChange={e=>{
        const k=e.target.value;if(k==='unused'||k==='outcome')change(n.id,{kind:k});else if(k==='observe'||k==='intervene')change(n.id,{kind:k,state:1})
      }}><option value="unused">Marginalise</option><option value="outcome">Read outcome</option><option value="observe">Observe</option><option value="intervene">Intervene</option></Select></label>
      {(r.kind==='observe'||r.kind==='intervene')&&<label><span className={fieldLabel}>State index</span><input className={field('text')} type="number" min={0} step={1} aria-label={`${n.name} state index`} value={r.state} onChange={e=>{const state=e.target.valueAsNumber;if(Number.isSafeInteger(state)&&state>=0)change(n.id,{...r,state})}} /></label>}
    </div>})}
    <p className={fieldHint}>Indices start at 0 in ascending state order. Low-cardinality numeric values are preserved; other values are grouped at quantiles. A state index must exist after preparation.</p>
    <div className="grid grid-cols-2 gap-3"><label><span className={fieldLabel}>State budget</span><Select aria-label="Query state budget" className={field('text')} value={bins} onChange={e=>setBins(Number(e.target.value))}>{[2,3,4,5].map(n=><option key={n}>{n}</option>)}</Select></label>
      <label><span className={fieldLabel}>Parameter estimation</span><Select aria-label="Parameter estimation" className={field('text')} value={ess===0?'mle':'bdeu'} onChange={e=>setEss(e.target.value==='mle'?0:DEFAULT_BDEU_EQUIVALENT_SAMPLE_SIZE)}><option value="mle">Maximum likelihood</option><option value="bdeu">BDeu</option></Select></label></div>
    {ess>0&&<label><span className={fieldLabel}>Equivalent sample size</span><input className={field('text')} aria-label="Query equivalent sample size" type="number" min={1} value={ess} onChange={e=>{if(Number.isFinite(e.target.valueAsNumber)&&e.target.valueAsNumber>0)setEss(e.target.valueAsNumber)}} /></label>}
    {problem!==null&&<Alert tone="danger">{problem}</Alert>}<JobNotice job={session.job}/>
    <div className="mt-4 flex gap-2"><button className={button('signal')} disabled={problem!==null||outcomes.length===0||session.blocked||session.job.kind==='running'} onClick={()=>void run()}>Evaluate probability</button>{session.job.kind==='running'&&<button className={button('quiet')} onClick={session.cancel}>Cancel run</button>}</div>
    <ul aria-label="Probability queries" className="m-0 mt-4 list-none divide-y divide-hair p-0 text-body">{[...records].reverse().map((r,index)=><Distribution key={r.id} record={r} open={index===0} onDelete={onDelete}/>)}</ul>
  </div>
}
