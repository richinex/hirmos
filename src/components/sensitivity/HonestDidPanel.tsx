import {useMemo,useState,type ReactNode} from 'react'
import {useJob} from '@/analysis/JobsProvider'
import {useRunActivity} from '@/lib/useRunActivity'
import type {RunActivity} from '@/domain/activity'
import type {PreparedDatasetArtifact} from '@/domain/preprocessing'
import type {EstimationRunArtifact} from '@/domain/estimation'
import type {TimeSeriesRun} from '@/domain/timeSeries'
import {newSensitivityRunId} from '@/domain/sensitivity'
import {defaultHonestConfiguration,honestCandidates,honestEnvelope,honestRequestSchema,honestRunSchema,type HonestEvidence,type HonestRun} from '@/domain/honestDid'
import {sameSpecification} from '@/domain/panelRegression'
import {describeAnalysisWorkerProblem} from '@/workers/analysisProtocol'
import {ChapterHeading} from '@/components/ui/ChapterHeading'
import {WorkbenchLayout} from '@/components/shell/WorkbenchLayout'
import {SettingsStep} from '@/components/ui/SettingsStep'
import {SegmentedControl} from '@/components/ui/SegmentedControl'
import {Select} from '@/components/ui/Select'
import {ParameterLabel} from '@/components/ui/ParameterLabel'
import {RunActions} from '@/components/ui/RunActions'
import {RunFold} from '@/components/ui/RunFold'
import {RunDetails} from '@/components/ui/RunDetails'
import {RunMeta} from '@/components/ui/RunMeta'
import {ConfirmDialog} from '@/components/ui/ConfirmDialog'
import {JobNotice} from '@/components/ui/JobNotice'
import {Alert} from '@/components/ui/Alert'
import {EvidenceTable,figureColumn} from '@/components/table/EvidenceTable'
import {ExpandableChart} from '@/charts/ExpandableChart'
import {useChartTheme} from '@/charts/theme'
import {baseOption,categoryAxis,gridAuto,legend,tooltip,valueAxis} from '@/charts/grammar'
import {actionGap,button,field,fieldLabel,fieldHint,panel,resultSurface,resultTitle,stepsStack,chapterIntroSingle} from '@/components/ui/recipes'
import {formatPercent,formatStatistic} from '@/lib/format/number'
import {formatTime} from '@/lib/format/date'
const number=(v:number)=>formatStatistic('raw',v).text
const numbers=(text:string)=>text.trim()===''?[]:text.trim().split(/[\s,;]+/).map(Number)
const finiteText=(text:string)=>text.trim()===''?NaN:Number(text)
// Bounds print as entered, so 0.5 reads 0.5 rather than 0.500.
const boundText=(bound:number)=>String(bound)
const accuracyText={solved:'solved',residualQualified:'approximately solved, residuals within tolerance',referenceInaccurate:'solution may be inaccurate'} as const

export function HonestDidResult({run}:{readonly run:HonestRun}){
  const e=run.evidence,theme=useChartTheme()
  const rows=useMemo(()=>e.results.map(row=>({...row,envelope:honestEnvelope(row.interval)})),[e])
  const confidence=e.request.configuration.confidence
  const level=formatPercent(confidence,{precision:Number.isInteger(confidence*100)?0:1}).text
  // One vertical interval per bound, beside the conventional interval, as in the reference sensitivity plot.
  const option=useMemo(()=>{
    const categories=['Conventional',...rows.map(row=>boundText(row.bound))]
    return {...baseOption(theme,'Parallel-trends sensitivity'),grid:gridAuto({top:42,bottom:42}),legend:{...legend(theme,['Conventional interval','Confidence limits']),bottom:'auto',top:0},tooltip:tooltip(theme,'item'),
      xAxis:categoryAxis(theme,categories,e.request.configuration.restriction.kind==='smoothness'?'Smoothness bound M':'Relative magnitude bound M̄'),yAxis:valueAxis(theme,'Effect'),series:[
        {id:'conventional',name:'Conventional interval',type:'line' as const,symbol:'circle',symbolSize:6,data:[[categories[0],e.conventional[0]],[categories[0],e.conventional[1]]],lineStyle:{color:theme.info,width:2},itemStyle:{color:theme.info},markLine:{silent:true,symbol:'none',label:{show:false},lineStyle:{color:theme.muted,type:'dashed' as const},data:[{yAxis:0}]}},
        {id:'limits',name:'Confidence limits',type:'line' as const,symbol:'circle',symbolSize:6,connectNulls:false,data:rows.flatMap((row,i)=>row.envelope===null?[]:[[categories[i+1],row.envelope.lower],[categories[i+1],row.envelope.upper],null]),lineStyle:{color:theme.signal,width:2},itemStyle:{color:theme.signal}},
      ]}
  },[e,rows,theme])
  const breakdown=rows.find(row=>row.envelope!==null&&row.envelope.lower<=0&&row.envelope.upper>=0)
  return <section className={resultSurface('min-w-0 space-y-4')} aria-label="Parallel-trends sensitivity result"><h3 className={`${resultTitle} m-0`}>Parallel-trends sensitivity</h3>
    <p className="m-0 text-body text-ink">Target estimate {number(e.estimate)}. Conventional {level} interval: {number(e.conventional[0])} to {number(e.conventional[1])}.</p>
    <ExpandableChart label="Confidence limits across parallel-trends restrictions" option={option} className="h-[320px]" />
    {e.request.configuration.restriction.kind==='relativeMagnitude'&&<p className={fieldHint}>The lines show the envelope of accepted grid points, not a claim that the confidence set is connected. An accepted endpoint indicates that the set may extend beyond the evaluated grid.</p>}
    <EvidenceTable frame="none" title="Sensitivity bounds" help="Each row uses the recorded event estimates and their full covariance. The restriction bound is an assumption about violations of parallel trends, not an estimate of the violation." rows={rows} rowKey={r=>String(r.bound)} noun="bound" empty="No sensitivity bounds." exportName="honest-did-bounds" columns={[
      figureColumn<(typeof rows)[number]>('bound','Bound',r=>r.bound,boundText),
      {id:'lower',header:'Lower limit',value:r=>r.envelope?.lower??'No accepted grid points',format:(_,r)=>r.envelope===null?'No accepted grid points':`${r.interval.kind==='confidenceGrid'&&r.interval.openLower?'At or below ':''}${number(r.envelope.lower)}`},
      {id:'upper',header:'Upper limit',value:r=>r.envelope?.upper??'No accepted grid points',format:(_,r)=>r.envelope===null?'No accepted grid points':`${r.interval.kind==='confidenceGrid'&&r.interval.openUpper?'At or above ':''}${number(r.envelope.upper)}`},
      {id:'method',header:'Calculation',value:r=>r.interval.kind==='fixedLength'?`Fixed-length interval, ${accuracyText[r.interval.accuracy]}`:'Conditional confidence grid'},
    ]} />
    <p className={fieldHint}>{breakdown===undefined?'No evaluated confidence-set envelope is shown to include zero.':`The first evaluated bound whose confidence-set envelope includes zero is ${boundText(breakdown.bound)}. No interpolation between bounds is reported.`}</p>
    <RunDetails label="Sensitivity specification and numerical evidence"><dl className="grid grid-cols-[auto_1fr] gap-x-3 text-body"><dt>Source</dt><dd className="m-0 break-all">{run.source.run}</dd><dt>Restriction</dt><dd className="m-0">{e.request.configuration.restriction.kind==='smoothness'?'Smoothness':'Relative magnitude'}</dd><dt>Target weights</dt><dd className="m-0">{e.request.eventStudy.contrast.map(number).join(', ')}</dd><dt>Seed</dt><dd className="m-0">{e.request.configuration.seed}</dd></dl>
      {e.results.filter(r=>r.interval.kind==='confidenceGrid').map(row=>row.interval.kind==='confidenceGrid'?<EvidenceTable key={row.bound} title={`Evaluated grid at bound ${boundText(row.bound)}`} rows={row.interval.grid.map((value,i)=>({value,accepted:row.interval.kind==='confidenceGrid'&&row.interval.accepted[i]}))} rowKey={r=>String(r.value)} noun="point" empty="No grid points." exportName={`honest-did-grid-${boundText(row.bound)}`} columns={[figureColumn<{value:number;accepted:boolean|undefined}>('value','Target value',r=>r.value,number),{id:'accepted',header:'Accepted',value:r=>r.accepted?'Yes':'No'}]} />:null)}
    </RunDetails>
  </section>
}

export function HonestDidPanel({prepared,estimates,designs,runs,onRun,onDelete,onActivity,selector}:{readonly prepared:PreparedDatasetArtifact;readonly estimates:readonly EstimationRunArtifact[];readonly designs:readonly TimeSeriesRun[];readonly runs:readonly HonestRun[];readonly onRun:(run:HonestRun)=>void;readonly onDelete:(id:HonestRun['id'])=>void;readonly onActivity?: (activity:RunActivity|null)=>void;readonly selector:ReactNode}){
  const candidates=useMemo(()=>honestCandidates(estimates,designs).filter(c=>c.preparedDataset===prepared.id),[estimates,designs,prepared.id])
  const latest=runs.at(-1),config=latest?.evidence.request.configuration??defaultHonestConfiguration,restriction=config.restriction
  const [selected,setSelected]=useState(latest?.source.run??candidates.at(-1)?.source.run??'')
  const [kind,setKind]=useState(restriction.kind)
  const [method,setMethod]=useState<'conditional'|'leastFavorableHybrid'>(restriction.kind==='relativeMagnitude'?restriction.method.kind:'conditional')
  const [moments,setMoments]=useState<'postTreatment'|'allPeriods'>(restriction.kind==='relativeMagnitude'?restriction.moments:'postTreatment')
  const [boundText,setBoundText]=useState(config.bounds.join(', ')),[confidence,setConfidence]=useState(String(config.confidence)),[seed,setSeed]=useState(String(config.seed))
  const [points,setPoints]=useState(String(restriction.kind==='smoothness'?restriction.points:restriction.grid.points))
  const [kappa,setKappa]=useState(String(restriction.kind==='relativeMagnitude'&&restriction.method.kind==='leastFavorableHybrid'?restriction.method.kappa:0.005))
  const [gridKind,setGridKind]=useState<'referenceDefault'|'explicit'>(restriction.kind==='relativeMagnitude'?restriction.grid.kind:'referenceDefault')
  const [lower,setLower]=useState(String(restriction.kind==='relativeMagnitude'&&restriction.grid.kind==='explicit'?restriction.grid.lower:-1)),[upper,setUpper]=useState(String(restriction.kind==='relativeMagnitude'&&restriction.grid.kind==='explicit'?restriction.grid.upper:1))
  const [target,setTarget]=useState<'first'|'average'|'specified'>(latest===undefined?'first':'specified'),[weights,setWeights]=useState(latest?.evidence.request.eventStudy.contrast.join(', ')??'')
  const [pendingDelete,setPendingDelete]=useState<HonestRun|null>(null)
  const candidate=candidates.find(c=>c.source.run===selected)
  const input=candidate?.input
  const contrast=input?.ok?(target==='first'?input.value.contrast:target==='average'?Array.from({length:input.value.post},()=>1/input.value.post):numbers(weights)):[]
  const request=input?.ok?honestRequestSchema.safeParse({
    eventStudy:{...input.value,contrast},
    configuration:{
      confidence:finiteText(confidence),bounds:numbers(boundText),seed:finiteText(seed),
      restriction:kind==='smoothness'?{kind,points:finiteText(points)}:{
        kind,method:method==='conditional'?{kind:method}:{kind:method,kappa:finiteText(kappa)},moments,
        grid:gridKind==='referenceDefault'?{kind:gridKind,points:finiteText(points)}:{kind:gridKind,lower:finiteText(lower),upper:finiteText(upper),points:finiteText(points)},
      },
    },
  }):null
  const readiness=candidate===undefined?'Choose an event-study run.':!candidate.input.ok?candidate.input.error:request===null||!request.success?'Complete the restriction, target weights and numerical settings.':null
  const session=useJob('sensitivity:honest-did'),{job}=session
  useRunActivity(onActivity,job.kind==='running'?{label:'Parallel-trends sensitivity',progress:null}:null)
  const execute=async()=>{
    if(candidate===undefined||request===null||!request.success)return
    const current=session.start('analysis','Computing robust confidence sets');if(current===null)return
    try{
      const {runHonestDid}=await import('@/analysis/client'),result=await runHonestDid(request.data)
      if(!session.current(current))return
      if(!result.ok){session.fail(current,describeAnalysisWorkerProblem(result.error));return}
      if(!sameSpecification(result.value.request,request.data)){session.fail(current,'The sensitivity result does not match the requested event study and restriction.');return}
      const saved=honestRunSchema.safeParse({kind:'honest-did-run',id:newSensitivityRunId(),preparedDataset:prepared.id,createdAt:new Date().toISOString(),source:candidate.source,evidence:result.value})
      if(!saved.success){session.fail(current,saved.error.message);return}
      onRun(saved.data);session.finish(current)
    }catch(error){if(session.current(current))session.fail(current,error instanceof Error?error.message:String(error))}
  }
  const numeric=(name:string,value:string,change:(v:string)=>void,help?:string)=><label className="block"><ParameterLabel className={fieldLabel} label={name} help={help??name} /><input aria-label={name} className={field('text','mt-1')} type="number" step="any" value={value} onChange={e=>change(e.target.value)} /></label>
  const result=(run:HonestRun)=><HonestDidResult run={run} />
  return <WorkbenchLayout id="sensitivity-honest-did" stage={<section className="@container/panel flex flex-col gap-5"><div><ChapterHeading>Sensitivity</ChapterHeading><p className={chapterIntroSingle}>Examine how event-study confidence sets change when parallel trends may be violated within a specified restriction.</p></div>{selector}<section className={panel('p-(--panel-space)')}><fieldset disabled={job.kind==='running'} className="m-0 min-w-0 border-0 p-0"><div className={stepsStack}>
    <SettingsStep number={1} title="Choose the event study"><label className="block"><span className={fieldLabel}>Event-study run</span><Select value={selected} onChange={e=>setSelected(e.target.value)}><option value="">Choose run</option>{candidates.map(c=><option key={c.source.run} value={c.source.run}>{c.label}, {formatTime(c.createdAt)}</option>)}</Select></label>{input?.ok&&<p className={fieldHint}>{input.value.pre} pre-treatment estimates and {input.value.post} post-treatment estimates, with period -1 omitted as the zero reference. The full covariance is retained.</p>}</SettingsStep>
    <SettingsStep number={2} title="Choose the target"><div><SegmentedControl ariaLabel="Sensitivity target" value={target} onChange={setTarget} options={[{value:'first',label:'First post-period'},{value:'average',label:'Average post-periods'},{value:'specified',label:'Specified weights'}]} /></div>{target==='specified'&&<label className="block"><ParameterLabel className={fieldLabel} label="Post-treatment target weights" help="One weight per post-treatment estimate, in event-time order." /><input aria-label="Post-treatment target weights" className={field('text','mt-1')} value={weights} onChange={e=>setWeights(e.target.value)} /></label>}</SettingsStep>
    <SettingsStep number={3} title="Specify the restriction"><div><ParameterLabel className={fieldLabel} label="Parallel-trends restriction" help={kind==='smoothness'?'The slope of the counterfactual difference in trends may change by no more than M across consecutive periods. M = 0 restricts the difference to a linear trend.':'Post-treatment violations of parallel trends may be no more than M̄ times the largest pre-treatment violation between consecutive periods. M̄ = 1 uses the worst pre-treatment violation as the bound.'} /><SegmentedControl className="mt-1" ariaLabel="Parallel-trends restriction" value={kind} onChange={setKind} options={[{value:'relativeMagnitude',label:'Relative magnitude'},{value:'smoothness',label:'Smoothness'}]} /></div><label className="block"><ParameterLabel className={fieldLabel} label={kind==='smoothness'?'Smoothness bounds M':'Relative magnitude bounds M̄'} help="Enter increasing nonnegative bounds, separated by commas." /><input aria-label={kind==='smoothness'?'Smoothness bounds M':'Relative magnitude bounds M̄'} className={field('text','mt-1')} value={boundText} onChange={e=>setBoundText(e.target.value)} /></label>{kind==='relativeMagnitude'&&<><div><span className={fieldLabel}>Conditional inference</span><SegmentedControl className="mt-1" ariaLabel="Conditional inference" value={method} onChange={setMethod} options={[{value:'conditional',label:'Conditional'},{value:'leastFavorableHybrid',label:'Least-favorable hybrid'}]} /></div><div><span className={fieldLabel}>Moment inequalities</span><SegmentedControl className="mt-1" ariaLabel="Moment inequalities" value={moments} onChange={setMoments} options={[{value:'postTreatment',label:'Post-treatment'},{value:'allPeriods',label:'All periods'}]} /></div>{method==='leastFavorableHybrid'&&numeric('Hybrid size',kappa,setKappa,'The hybrid size must be positive and smaller than one minus the confidence level.')}</>}</SettingsStep>
    <SettingsStep number={4} title="Set uncertainty and numerical precision"><div className="grid gap-4 @md/panel:grid-cols-2">{numeric('Confidence level',confidence,setConfidence,'Enter a proportion between zero and one.')}{numeric('Seed',seed,setSeed)}{numeric(kind==='smoothness'?'Search points':'Grid points',points,setPoints,'Use 2 to 10,001 points. More points increase numerical resolution and computation time.')}</div>{kind==='relativeMagnitude'&&<><div><ParameterLabel className={fieldLabel} label="Confidence-set grid" help="The reference grid extends from -20 to 20 times the target standard error. Accepted endpoints require a wider grid before reporting finite limits." /><SegmentedControl className="mt-1" ariaLabel="Confidence-set grid" value={gridKind} onChange={setGridKind} options={[{value:'referenceDefault',label:'Reference grid'},{value:'explicit',label:'Specified limits'}]} /></div>{gridKind==='explicit'&&<div className="grid gap-4 @md/panel:grid-cols-2">{numeric('Grid lower limit',lower,setLower)}{numeric('Grid upper limit',upper,setUpper)}</div>}</>}</SettingsStep>
  </div></fieldset>{readiness!==null&&<Alert tone="info" live={false}><p className="m-0">{readiness}</p></Alert>}<RunActions className={actionGap} running={job.kind==='running'} onCancel={session.cancel} orbLabel="Sensitivity running"><button className={button('signal')} disabled={readiness!==null||session.blocked||job.kind==='running'} onClick={()=>void execute()}>Compute sensitivity sets</button></RunActions><JobNotice job={job} /></section>{latest!==undefined&&result(latest)}</section>}
    inspector={{trigger:{label:'Requirements',icon:'contract'},title:'Parallel-trends sensitivity requirements',body:<div className="space-y-3 text-body text-muted"><p>Use consecutive event periods with period -1 normalized to zero, estimated pre-treatment and post-treatment coefficients, and their full sampling covariance.</p><p>The confidence sets are valid under the specified restriction on violations of parallel trends and the sampling approximation for the event-study estimates. They do not establish that the restriction holds.</p><p>These sets account for estimation error in the pre-treatment and post-treatment estimates under the recorded restriction. Failure to reject pre-treatment differences does not establish parallel trends.</p><p>Empty grids or accepted endpoints do not establish exclusion outside the evaluated grid.</p><p className="mb-2 mt-2 text-label text-faint">Literature: A More Credible Approach to Parallel Trends (Rambachan and Roth, 2023), Review of Economic Studies 90(5), 2555–2591</p></div>}}
    bottom={{trigger:{label:'History',icon:'history'},title:`Sensitivity runs (${runs.length})`,defaultSize:150,body:<>{[...runs].reverse().map(run=><RunFold key={run.id} title="Parallel-trends sensitivity" figure={<RunMeta>{[run.evidence.request.configuration.restriction.kind==='smoothness'?'Smoothness':'Relative magnitude',`${run.evidence.results.length} bounds`]}</RunMeta>} stamp={formatTime(run.createdAt)} onDelete={()=>setPendingDelete(run)} deleteLabel="Delete this sensitivity run">{result(run)}</RunFold>)}<ConfirmDialog open={pendingDelete!==null} title="Delete this sensitivity run?" message="This removes the sensitivity record, not its source event study." confirmLabel="Delete run" danger onConfirm={()=>{if(pendingDelete!==null)onDelete(pendingDelete.id)}} onClose={()=>setPendingDelete(null)} /></>}} />
}
