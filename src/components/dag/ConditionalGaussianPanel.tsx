import { RunActions } from '@/components/ui/RunActions'
import {useMemo,useState} from 'react'
import {RunFold} from '@/components/ui/RunFold'
import {formatTime} from '@/lib/format/date'
import type {DagDocument,DagNodeId} from '@/domain/dag'
import type {DatasetProfile} from '@/domain/dataset'
import type {PreparedDatasetArtifact} from '@/domain/preprocessing'
import type {SelectedSource} from '@/domain/workflow'
import {newInterventionQueryId} from '@/domain/intervention'
import {conditionalGaussianQuerySchema,conditionalGaussianQueryFormula,describeConditionalGaussianQuery,type ConditionalGaussianArtifact} from '@/domain/conditionalGaussianQuery'
import {isNonEmpty,assertNever} from '@/domain/dop'
import {useJob} from '@/analysis/JobsProvider'
import {describeAnalysisWorkerProblem} from '@/workers/analysisProtocol'
import {Select} from '@/components/ui/Select'
import {SegmentedControl} from '@/components/ui/SegmentedControl'
import {DisclosureSummary} from '@/components/ui/DisclosureSummary'
import {Icon} from '@/components/Icon'
import {formatEstimate,formatStatistic} from '@/lib/format/number'

/** Two decimals in the outcome's own units: enough to tell 310.01 from 307.39 at a glance. */
const outcomeValue=(value:number)=>formatEstimate(value,{kind:'additive',unit:''},{precision:{kind:'decimals',places:2}}).text
import {Formula} from '@/components/ui/Formula'
import {FilterField} from '@/components/table/primitives'
import type {PreparedMatrix} from '@/data/prepared'
import {Alert} from '@/components/ui/Alert'
import {JobNotice} from '@/components/ui/JobNotice'
import {button,field,fieldHint,fieldLabel,well} from '@/components/ui/recipes'
import {EChart} from '@/charts/EChart'
import {useChartTheme,seriesColour} from '@/charts/theme'
import {baseOption,gridAuto,legend,valueAxis,tooltip} from '@/charts/grammar'

type Role={readonly kind:'observe';readonly draft:string}|{readonly kind:'intervene';readonly draft:string}
type VariableKind='continuous'|'discrete'
type Setup={readonly kind:'unreviewed'}|{readonly kind:'reviewed';readonly matrix:PreparedMatrix}
const emptyRole:Role={kind:'observe',draft:''}
const numericDraft=(draft:string):number|null=>draft.trim()!==''&&Number.isFinite(Number(draft))?Number(draft):null
const listClass='panel-scroll max-h-[min(24rem,50dvh)] min-w-0 space-y-4 overflow-y-auto p-1'

/** A shared horizontal range for results that read the same outcome, so a shift between them shows as a shift on the page. */
type DensityRange={readonly min:number;readonly max:number}
const densityRange=(records:readonly ConditionalGaussianArtifact[]):DensityRange=>({
  min:Math.min(...records.map(r=>r.result.mean-4*r.result.std)),
  max:Math.max(...records.map(r=>r.result.mean+4*r.result.std)),
})
const outcomeKey=(record:ConditionalGaussianArtifact)=>record.specification.names[record.specification.outcome]??String(record.specification.outcome)

function GaussianResult({record,range,open,onDelete}:{readonly record:ConditionalGaussianArtifact;readonly range:DensityRange;readonly open:boolean;readonly onDelete:(record:ConditionalGaussianArtifact)=>void}){
  const theme=useChartTheme(),description=describeConditionalGaussianQuery(record.specification)
  const {mean,std}=record.result
  const formula=conditionalGaussianQueryFormula(record.specification)
  const option=useMemo(()=>({...baseOption(theme,description),grid:gridAuto({top:30,bottom:8}),
    // Above the plot: the axis name owns the bottom edge.
    legend:{...legend(theme,['fitted density','mean']),bottom:'auto',top:0},
    tooltip:{...tooltip(theme,'axis'),formatter:(raw:unknown)=>{const first=Array.isArray(raw)?raw[0]:raw;const value=first!==null&&typeof first==='object'?Reflect.get(first,'value'):null;if(!Array.isArray(value))return '';return `${record.specification.names[record.specification.outcome]} ${outcomeValue(Number(value[0]))}<br/>density ${formatStatistic('raw',Number(value[1])).text}<br/>mean ${outcomeValue(mean)}`}},
    xAxis:{...valueAxis(theme,record.specification.names[record.specification.outcome]),min:range.min,max:range.max,splitLine:{show:false}},
    yAxis:{...valueAxis(theme,'Density'),splitLine:{show:false}},
    series:[{name:'fitted density',type:'line' as const,showSymbol:false,itemStyle:{color:seriesColour(theme,0)},lineStyle:{color:seriesColour(theme,0),width:2},
      data:Array.from({length:161},(_,i)=>{const x=range.min+(range.max-range.min)*i/160;const z=(x-mean)/std;return [x,Math.exp(-z*z/2)/(std*Math.sqrt(2*Math.PI))]})},
      // The mean as its own series, so the legend can name it.
      {name:'mean',type:'line' as const,showSymbol:false,silent:true,itemStyle:{color:theme.signal},lineStyle:{color:theme.signal,width:1.5},data:[[mean,0],[mean,1/(std*Math.sqrt(2*Math.PI))]]}],
  }),[theme,description,record.specification,mean,std,range])
  return <RunFold title={description} stamp={formatTime(record.createdAt)} defaultOpen={open} onDelete={()=>onDelete(record)} deleteLabel="Delete this query">
    <div className="text-body"><Formula tex={formula.tex} plain={formula.plain}/></div>
    <dl className="my-2 grid grid-cols-[auto_minmax(0,1fr)] gap-x-3 gap-y-1 text-label text-muted" aria-label="Query variables">
      {formula.variables.map(variable=><div key={variable.plain} className="contents"><dt><Formula tex={variable.tex} plain={variable.plain}/></dt><dd className="m-0 [overflow-wrap:anywhere]">{variable.name}</dd></div>)}
    </dl>
    <EChart option={option} label="Fitted conditional Gaussian density" className="h-[200px]"/>
    <dl className="grid grid-cols-2 gap-2 text-label tabular-nums"><dt>Conditional mean</dt><dd className="m-0 text-right" title={String(mean)}>{mean!==0&&Math.abs(mean)<1e-6?mean.toExponential(3):mean.toFixed(6)}</dd><dt>Residual standard deviation</dt><dd className="m-0 text-right" title={String(std)}>{std<1e-6?std.toExponential(3):std.toFixed(6)}</dd></dl>
  </RunFold>
}

interface PanelProps{
  readonly document:DagDocument;readonly source:SelectedSource;readonly profile:DatasetProfile;readonly prepared:PreparedDatasetArtifact
  readonly records:readonly ConditionalGaussianArtifact[];readonly onQuery:(q:ConditionalGaussianArtifact)=>void;readonly onDelete:(q:ConditionalGaussianArtifact)=>void
}

export function ConditionalGaussianPanel(props:PanelProps){
  return <ConditionalGaussianForm key={`${props.document.current.id}:${props.prepared.id}:${props.source.file.lastModified}`} {...props}/>
}

function ConditionalGaussianForm({document,source,profile,prepared,records,onQuery,onDelete}:PanelProps){
  const nodes=document.current.graph.nodes
  const [types,setTypes]=useState<ReadonlyMap<DagNodeId,VariableKind>>(new Map())
  const [roles,setRoles]=useState<ReadonlyMap<DagNodeId,Role>>(new Map())
  const [outcome,setOutcome]=useState<DagNodeId|null>(null)
  const [setup,setSetup]=useState<Setup>({kind:'unreviewed'})
  const [typesOpen,setTypesOpen]=useState(true)
  const [typeSearch,setTypeSearch]=useState('')
  const [outcomeSearch,setOutcomeSearch]=useState('')
  const [parentSearch,setParentSearch]=useState('')
  const session=useJob(`conditional-gaussian:${document.current.id}`)
  const typeOf=(id:DagNodeId)=>types.get(id)??'continuous'
  const problem=nodes.some(n=>n.kind==='latent')?'Conditional-Gaussian fitting requires a fully observed DAG.':document.current.validation.structure.kind==='invalid'?'Resolve the graph’s structural issues before fitting.':null
  const invalidEdge=document.current.graph.edges.find(e=>typeOf(e.cause)==='continuous'&&typeOf(e.effect)==='discrete')
  const typeProblem=invalidEdge===undefined?null:`${nodes.find(n=>n.id===invalidEdge.effect)?.name} is discrete but has a continuous parent. This model does not support that relationship.`
  const parents=nodes.filter(n=>document.current.graph.edges.some(e=>e.effect===outcome&&e.cause===n.id))
  const categories=useMemo(()=>{
    const result=new Map<DagNodeId,readonly number[]>()
    if(setup.kind==='reviewed')nodes.forEach((n,i)=>{
      if(types.get(n.id)==='discrete')result.set(n.id,[...new Set(setup.matrix.values.subarray(i*setup.matrix.rowCount,(i+1)*setup.matrix.rowCount))].sort((a,b)=>a-b))
    })
    return result
  },[setup,nodes,types])
  const missing=parents.filter(n=>{
    const value=numericDraft((roles.get(n.id)??emptyRole).draft)
    return value===null||typeOf(n.id)==='discrete'&&!categories.get(n.id)?.includes(value)
  }).length
  const busy=session.blocked||session.job.kind==='running'
  const review=async()=>{
    if(problem!==null||typeProblem!==null)return
    const current=session.start('analysis','Read Gaussian query values');if(current===null)return
    try{
      const {materialisePrepared,describePreparedMaterialisationProblem}=await import('@/data/prepared')
      const columns=nodes.flatMap(n=>n.kind==='observed'?[n.column]:[])
      if(!isNonEmpty(columns)){session.fail(current,'Select measured variables.');return}
      const matrix=await materialisePrepared(source,profile,prepared,columns)
      if(!session.current(current))return
      if(!matrix.ok){session.fail(current,describePreparedMaterialisationProblem(matrix.error));return}
      setSetup({kind:'reviewed',matrix:matrix.value});setTypesOpen(false);session.finish(current)
    }catch(error){session.fail(current,error instanceof Error?error.message:String(error))}
  }
  const run=async()=>{
    if(problem!==null||typeProblem!==null||outcome===null||setup.kind!=='reviewed'||missing>0)return
    const current=session.start('analysis','Conditional Gaussian query');if(current===null)return
    try{
      const {runConditionalGaussianQuery}=await import('@/analysis/client')
      const observations:{variable:number;value:number}[]=[],interventions:{variable:number;value:number}[]=[]
      for(const n of parents){const role=roles.get(n.id)??emptyRole,value=numericDraft(role.draft);if(value===null){session.fail(current,`Enter a value for ${n.name}.`);return}const entry={variable:nodes.findIndex(node=>node.id===n.id),value};switch(role.kind){case 'observe':observations.push(entry);break;case 'intervene':interventions.push(entry);break;default:assertNever(role)}}
      const specification=conditionalGaussianQuerySchema.safeParse({rows:setup.matrix.rowCount,columns:nodes.length,names:nodes.map(n=>n.name),edges:document.current.graph.edges.map(e=>[nodes.findIndex(n=>n.id===e.cause),nodes.findIndex(n=>n.id===e.effect)]),variables:nodes.map(n=>typeOf(n.id)),outcome:nodes.findIndex(n=>n.id===outcome),observations,interventions})
      if(!specification.success){session.fail(current,specification.error.message);return}
      // Worker transfer must not detach the reviewed matrix used for subsequent queries.
      const result=await runConditionalGaussianQuery(setup.matrix.values.slice(),specification.data)
      if(!session.current(current))return
      if(!result.ok){session.fail(current,describeAnalysisWorkerProblem(result.error));return}
      onQuery({kind:'conditional-gaussian-query',id:newInterventionQueryId(),dagDocument:document.id,dagRevision:document.current.id,preparedDataset:prepared.id,createdAt:new Date().toISOString(),specification:specification.data,result:result.value})
      session.finish(current)
    }catch(error){session.fail(current,error instanceof Error?error.message:String(error))}
  }
  const matches=(name:string,search:string)=>name.toLocaleLowerCase().includes(search.trim().toLocaleLowerCase())
  const shownTypes=nodes.filter(n=>matches(n.name,typeSearch))
  const shownParents=parents.filter(n=>matches(n.name,parentSearch))
  const continuous=nodes.filter(n=>typeOf(n.id)==='continuous')
  return <div className="@container/gaussian mt-4 min-w-0 space-y-4" aria-label="Conditional Gaussian query">
    <p className={fieldHint}>Choose a continuous outcome and supply a value for each of its direct parents. You can observe a value or set it by intervention.</p>
    <details className={well('min-w-0 p-3')} open={typesOpen} onToggle={e=>setTypesOpen(e.currentTarget.open)}>
      <DisclosureSummary><span className="flex min-w-0 flex-wrap items-center gap-x-3 gap-y-1">
        <span>Variable types</span>
        <span className="text-muted">{nodes.length} variables</span>
        {setup.kind==='reviewed'
          ?<span className="ml-auto inline-flex items-center gap-1.5 text-ok"><Icon name="check_circle" size={16}/>Reviewed</span>
          :<span className="ml-auto inline-flex items-center gap-1.5 text-warn"><Icon name="help_outline" size={16}/>Review required</span>}
      </span></DisclosureSummary>
      <div className="mt-3 space-y-3">
        <p className={fieldHint}>Mark categorical variables as discrete, including binary indicators. All variables start as continuous; review their types before continuing.</p>
        {nodes.length>6&&<FilterField label="Find variable types" placeholder="Find a variable…" value={typeSearch} onChange={setTypeSearch}/>}
        <div className={listClass} role="group" aria-label="Variable types">
          {shownTypes.map(n=><label key={n.id} className="grid min-w-0 items-center gap-2 @sm/gaussian:grid-cols-[minmax(0,1fr)_9rem]"><span className={`${fieldLabel} min-w-0 [overflow-wrap:anywhere]`}>{n.name}</span>
            <Select aria-label={`${n.name} variable type`} disabled={busy} className={field('text')} value={typeOf(n.id)} onChange={e=>{const value=e.target.value;if(value==='discrete'||value==='continuous'){setTypes(old=>new Map(old).set(n.id,value));setSetup({kind:'unreviewed'});if(value==='discrete'&&outcome===n.id)setOutcome(null)}}}><option value="continuous">Continuous</option><option value="discrete">Discrete</option></Select>
          </label>)}
          {shownTypes.length===0&&<p className={fieldHint}>No variables match your search.</p>}
        </div>
        {typeProblem!==null&&<Alert tone="danger">{typeProblem}</Alert>}
        <button className={button('quiet','w-full')} disabled={busy||problem!==null||typeProblem!==null} onClick={()=>void review()}>Confirm variable types</button>
      </div>
    </details>
    <div className="space-y-2">
      <span className={fieldLabel}>Continuous outcome</span>
      {continuous.length>6&&<FilterField label="Find Gaussian outcome" placeholder="Find an outcome…" value={outcomeSearch} onChange={setOutcomeSearch}/>}
      <Select aria-label="Gaussian outcome" disabled={busy} className={field('text')} value={outcome??''} onChange={e=>{const node=continuous.find(n=>n.id===e.target.value);if(node){setOutcome(node.id);setParentSearch('')}}}><option value="" disabled>Choose outcome</option>{continuous.filter(n=>n.id===outcome||matches(n.name,outcomeSearch)).map(n=><option key={n.id} value={n.id}>{n.name}</option>)}</Select>
    </div>
    {outcome!==null&&<section aria-label="Parent values" className="space-y-3">
      <div className="flex flex-wrap items-baseline justify-between gap-2"><h4 className="m-0 text-body font-medium">Parent values</h4><span className="text-label text-muted" role="status">{parents.length-missing} of {parents.length} entered</span></div>
      <p className={fieldHint}>{parents.length===0?'This outcome has no direct parents. No values are required.':'Only variables with arrows pointing directly to this outcome appear here. Use the units and numeric category codes in the prepared data.'}</p>
      {parents.length>4&&<FilterField label="Find parent values" placeholder="Find a parent…" value={parentSearch} onChange={setParentSearch}/>}
      <div className={listClass} role="group" aria-label="Gaussian parent inputs">
        {shownParents.map(n=>{const r=roles.get(n.id)??emptyRole;return <fieldset key={n.id} className="min-w-0 space-y-2 border-0 p-0" disabled={busy||setup.kind!=='reviewed'}>
          <legend className={`${fieldLabel} mb-2 [overflow-wrap:anywhere]`}>{n.name}</legend>
          <SegmentedControl ariaLabel={`${n.name} Gaussian role`} disabled={busy||setup.kind!=='reviewed'} value={r.kind} onChange={kind=>setRoles(old=>new Map(old).set(n.id,{kind,draft:r.draft}))} options={[{value:'observe',label:'Observe'},{value:'intervene',label:'Intervene'}]} fill/>
          <label className="block"><span className={fieldLabel}>{typeOf(n.id)==='discrete'?'Category value':'Value'}</span>
            {typeOf(n.id)==='discrete'?<Select aria-label={`${n.name} Gaussian value`} className={field('text')} disabled={busy||setup.kind!=='reviewed'} value={r.draft} onChange={e=>setRoles(old=>new Map(old).set(n.id,{...r,draft:e.target.value}))}><option value="">Choose a category</option>{(categories.get(n.id)??[]).map(value=><option key={value} value={value}>{value}</option>)}</Select>
              :<input type="number" step="any" className={field('text')} aria-label={`${n.name} Gaussian value`} value={r.draft} onChange={e=>setRoles(old=>new Map(old).set(n.id,{...r,draft:e.target.value}))}/>}
          </label>
        </fieldset>})}
        {parents.length>0&&shownParents.length===0&&<p className={fieldHint}>No parents match your search. Clear the search to see the remaining inputs.</p>}
      </div>
    </section>}
    {problem!==null&&<Alert tone="danger">{problem}</Alert>}
    <JobNotice job={session.job}/>
    <div className="space-y-2">
      <p className={fieldHint}>{setup.kind!=='reviewed'?'Confirm the variable types before evaluating a query.':outcome===null?'Choose an outcome.':missing>0?`${missing} parent ${missing===1?'value is':'values are'} still required.`:'All required values are supplied.'}</p>
      <RunActions running={session.job.kind==='running'} onCancel={session.cancel} orbLabel="Gaussian query running"><button className={button('signal','w-full')} disabled={problem!==null||typeProblem!==null||outcome===null||setup.kind!=='reviewed'||missing>0||busy} aria-busy={session.job.kind==='running'} onClick={()=>void run()}>Evaluate Gaussian query</button></RunActions>
    </div>
    <details><DisclosureSummary>About this model</DisclosureSummary><div className="mt-2 space-y-2">
      <p className={fieldHint}>Continuous parents enter a linear regression. Each observed combination of discrete parent values has its own regression and residual standard deviation.</p>
      <p className={fieldHint}>This query requires a value for every direct parent. It cannot use observations from other variables or average over unspecified parents. Discrete variables cannot have continuous parents.</p>
      <p className={fieldHint}>A causal interpretation requires the recorded graph and mechanisms to remain valid under intervention.</p>
    </div></details>
    {records.length>0&&<section aria-label="Gaussian results"><h4 className="m-0 text-body font-medium">Results</h4><p className={fieldHint}>The curves show fitted outcome distributions, not confidence intervals. Their spread is residual variation. Uncertainty from fitting the model is not included.</p>
      <ul aria-label="Gaussian queries" className="m-0 mt-3 list-none divide-y divide-hair p-0 text-body">{[...records].reverse().map((record,index)=><GaussianResult key={record.id} record={record} open={index===0} onDelete={onDelete} range={densityRange(records.filter(other=>outcomeKey(other)===outcomeKey(record)))}/>)}</ul>
    </section>}
  </div>
}
