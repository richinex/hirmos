import {useMemo,useState,type ReactNode} from 'react'
import {useJob} from '@/analysis/JobsProvider'
import {useRunActivity} from '@/lib/useRunActivity'
import type {RunActivity} from '@/domain/activity'
import type {SelectedSource} from '@/domain/workflow'
import type {DatasetProfile,ColumnId} from '@/domain/dataset'
import type {PreparedDatasetArtifact} from '@/domain/preprocessing'
import type {EstimationRunArtifact} from '@/domain/estimation'
import {didSensitivityRequestSchema,didSensitivityRunSchema,didSensitivitySource,didSensitivityRunMatches,type DidSensitivityRun} from '@/domain/didSensitivity'
import {newSensitivityRunId} from '@/domain/sensitivity'
import {sameSpecification} from '@/domain/panelRegression'
import {describePanelDataProblem} from '@/domain/panel'
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
import {ConfirmDialog} from '@/components/ui/ConfirmDialog'
import {JobNotice} from '@/components/ui/JobNotice'
import {Alert} from '@/components/ui/Alert'
import {EvidenceTable,figureColumn} from '@/components/table/EvidenceTable'
import {ExpandableChart} from '@/charts/ExpandableChart'
import {useChartTheme} from '@/charts/theme'
import {didBoundsOption,didContourOption} from '@/charts/didSensitivity'
import {actionGap,button,field,fieldLabel,fieldHint,label,num,panel,stepsStack,chapterIntroSingle} from '@/components/ui/recipes'
import {MetricGrid,MetricTile} from '@/components/ui/figures'
import {RunMeta} from '@/components/ui/RunMeta'
import {didNormalizationLabels} from '@/domain/adjustedDid'
import {formatCount,formatPercent,formatSetPercent,formatSetting,formatStatistic,percentAsSet} from '@/lib/format/number'
import {formatTime} from '@/lib/format/date'
const number=(v:number)=>formatStatistic('raw',v).text
const numeric=(s:string)=>s.trim()===''?NaN:Number(s)
// Settings are typed in percent, as the results show them, and kept as proportions.
const percents=(s:string)=>s.trim()===''?[]:s.trim().split(/[\s,;]+/).map(v=>numeric(v)/100)
const asSet=(proportions:readonly number[])=>proportions.map(percentAsSet).join(', ')
const contains=(interval:readonly number[],value:number)=>interval[0]!<=value&&interval[1]!>=value

/** The source fit as the run picker names it. */
const sourceName=(run:EstimationRunArtifact)=>`Two-period DR DiD, ${formatTime(run.createdAt)}`

function DidSensitivityRecord({run,source}:{readonly run:DidSensitivityRun;readonly source:EstimationRunArtifact|undefined}){
  const e=run.evidence,theme=useChartTheme()
  const [view,setView]=useState<'bounds'|'contour'>('bounds')
  const [quantity,setQuantity]=useState<'effect'|'interval'>('effect')
  const option=useMemo(()=>view==='bounds'?didBoundsOption(e,theme):didContourOption(e,theme,quantity),[e,theme,view,quantity])
  const rows=useMemo(()=>[{shares:[0,0],...e.baseline},...e.scenarios.map((b,i)=>({shares:e.request.scenarios[i]!,...b}))],[e])
  const failed=e.optimizerStatus.filter(s=>s==='iteration-limit'||s==='line-search-failed').length
  const level=formatSetPercent(e.request.level).text
  return <>
    <h3 className="mb-1 mt-2 text-title font-medium leading-7 text-ink">DiD omitted-variable sensitivity</h3>
    <p className="m-0 text-body text-muted">ATT <span className={num('text-ink')}>{number(e.estimate)}</span>, standard error <span className={num('text-ink')}>{number(e.standardError)}</span>. {formatCount(e.units.length).text} paired units.</p>
    {failed>0&&<Alert tone="warn" live={false} className="mt-3"><p className="m-0">{formatCount(failed).text} propensity fits did not converge. These bounds describe the returned fits; examine covariate scale and overlap before interpreting them.</p></Alert>}
    {e.rieszEstimate==='nonOrthogonalReferenceRecovery'&&<Alert tone="info" live={false} className="mt-3"><p className="m-0">The orthogonal estimate of the Riesz norm was nonpositive. The calculation uses the non-orthogonal estimate, following the reference procedure.</p></Alert>}
    <MetricGrid className="mt-3" label="Robustness values">
      <MetricTile size="compact" frame="cell" label="Equal-share effect robustness value" value={formatPercent(e.robustnessValue,{precision:1})} context={e.estimate===e.request.null?'The estimate already equals the null.':`Null effect ${formatSetting(e.request.null).text}`}/>
      <MetricTile size="compact" frame="cell" label="Equal-share interval robustness value" value={formatPercent(e.robustnessValueCi,{precision:1})} context={contains(e.baseline.interval,e.request.null)?'The zero-confounding confidence bounds already include the null.':`${level} one-sided level`}/>
    </MetricGrid>
    <div className="mt-4 flex flex-wrap items-center gap-2">
      <SegmentedControl ariaLabel="DiD sensitivity plot" value={view} onChange={setView} options={[{value:'bounds',label:'Scenario bounds'},{value:'contour',label:'Contour'}]} />
      {view==='contour'&&<SegmentedControl ariaLabel="Contour quantity" value={quantity} onChange={setQuantity} options={[{value:'effect',label:'Effect bound'},{value:'interval',label:'Confidence bound'}]} />}
    </div>
    <div className="mt-3">
      <ExpandableChart label={view==='bounds'?'DiD effect and confidence bounds':'DiD confounding-share contours'} option={option} className="h-[340px]" />
      <p className={`${fieldHint} max-w-[75ch]`}>{view==='bounds'?'Thick lines show effect bounds; thin lines also account for sampling uncertainty.':'Contours show the bound on the side of the estimate facing the null. Values are interpolated between the computed grid points, not extrapolated beyond them. If no contour is drawn, no displayed level crosses the grid.'} Each confidence bound uses a {level} one-sided level. At zero confounding, the displayed bounds form a {formatSetPercent(2*e.request.level-1).text} central interval, not the usual {level} two-sided interval.</p>
    </div>
    <div className="mt-4">
      <EvidenceTable frame="none" title="Confounding scenarios" rows={rows} rowKey={(_,i)=>String(i)} noun="scenario" empty="No scenarios." exportName="did-sensitivity-bounds" columns={[
        {id:'outcome',header:'Outcome share',value:r=>r.shares[0]!,format:v=>formatSetPercent(Number(v)).text},
        {id:'riesz',header:'Riesz share',value:r=>r.shares[1]!,format:v=>formatSetPercent(Number(v)).text},
        {id:'effect',header:'Effect bounds',align:'right',value:r=>r.effect.join(', '),format:(_,r)=>r.effect[0]===r.effect[1]?number(r.effect[0]):r.effect.map(number).join(' to ')},
        {id:'interval',header:'Confidence bounds',align:'right',value:r=>r.interval.join(', '),format:(_,r)=>r.interval.map(number).join(' to ')},
      ]} />
    </div>
    <RunDetails label="Sensitivity specification"><dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body">
      <dt className="text-faint">Source run</dt><dd className="m-0 text-ink">{source===undefined?'No longer saved':sourceName(source)}</dd>
      <dt className="text-faint">Fold seed</dt><dd className={num('m-0 text-ink')}>{formatSetting(e.request.seed).text}</dd>
      <dt className="text-faint">Folds</dt><dd className={num('m-0 text-ink')}>{e.request.folds}</dd>
      <dt className="text-faint">Normalization</dt><dd className="m-0 text-ink">{didNormalizationLabels[e.request.normalization]}</dd>
      <dt className="text-faint">Trimming</dt><dd className={num('m-0 text-ink')}>{formatSetting(e.request.trimming).text}</dd>
      <dt className="text-faint">Correlation</dt><dd className={num('m-0 text-ink')}>{formatSetting(e.request.rho).text}</dd>
      <dt className="text-faint">Null effect</dt><dd className={num('m-0 text-ink')}>{formatSetting(e.request.null).text}</dd>
      <dt className="text-faint">One-sided confidence level</dt><dd className={num('m-0 text-ink')}>{level}</dd>
      <dt className="text-faint">Contour grid</dt><dd className={num('m-0 text-ink')}>{formatCount(e.request.outcomeShares.length).text} × {formatCount(e.request.rieszShares.length).text} points, to {formatSetPercent(e.request.outcomeShares.at(-1)!).text} outcome and {formatSetPercent(e.request.rieszShares.at(-1)!).text} Riesz share</dd>
    </dl></RunDetails>
  </>
}

/** The latest run on the stage, as the other probe cards show theirs. */
export function DidSensitivityResult({run,source}:{readonly run:DidSensitivityRun;readonly source:EstimationRunArtifact|undefined}){
  return <article className="rounded-xl bg-panel lift p-4" aria-label="DiD omitted-variable sensitivity result">
    <div className="flex flex-wrap items-baseline justify-between gap-2">
      <span className={label('text-signal-text')}>Current run</span>
      <span className={num('text-micro text-faint')}><RunMeta>{[...(source===undefined?[]:[sourceName(source)]),`Seed ${run.evidence.request.seed}`,formatTime(run.createdAt)]}</RunMeta></span>
    </div>
    <DidSensitivityRecord run={run} source={source}/>
  </article>
}

export function DidSensitivityPanel({source,profile,prepared,estimates,runs,onRun,onDelete,onActivity,selector}:{
 readonly source:SelectedSource;readonly profile:DatasetProfile;readonly prepared:PreparedDatasetArtifact;
 readonly estimates:readonly EstimationRunArtifact[];readonly runs:readonly DidSensitivityRun[];
 readonly onRun:(run:DidSensitivityRun)=>void;readonly onDelete:(id:DidSensitivityRun['id'])=>void;
 readonly onActivity?:(a:RunActivity|null)=>void;readonly selector:ReactNode;
}){
  const candidates=useMemo(()=>estimates.filter(r=>r.preparedDataset===prepared.id).flatMap(r=>{const c=didSensitivitySource(r);return c===null?[]:[c]}),[estimates,prepared.id])
  const latest=runs.at(-1),previous=latest?.evidence.request
  const [selected,setSelected]=useState<string>(latest?.estimationRun??candidates.at(-1)?.run.id??'')
  const [y,setY]=useState(previous===undefined?'2, 5, 10':asSet(previous.scenarios.map(s=>s[0])))
  const [d,setD]=useState(previous===undefined?'2, 5, 10':asSet(previous.scenarios.map(s=>s[1])))
  const [rho,setRho]=useState(String(previous?.rho??1)),[level,setLevel]=useState(String(percentAsSet(previous?.level??0.95))),[nullValue,setNullValue]=useState(String(previous?.null??0))
  const [yMax,setYMax]=useState(String(percentAsSet(previous?.outcomeShares.at(-1)??0.15))),[dMax,setDMax]=useState(String(percentAsSet(previous?.rieszShares.at(-1)??0.15)))
  const [points,setPoints]=useState(String(previous?.outcomeShares.length??41))
  const [pendingDelete,setPendingDelete]=useState<DidSensitivityRun|null>(null)
  const candidate=candidates.find(c=>c.run.id===selected)
  const ys=percents(y),ds=percents(d),count=numeric(points)
  const axis=(maximum:string)=>Number.isInteger(count)&&count>=2&&count<=51?Array.from({length:count},(_,i)=>numeric(maximum)/100*i/(count-1)):[]
  const specification=candidate?.specification
  const request=specification===undefined?null:didSensitivityRequestSchema.safeParse({
    propensityFit:'standardized-logistic-v1',
    folds:specification.folds,seed:specification.seed,trimming:specification.trimming,normalization:specification.normalization,
    scenarios:ys.length===ds.length?ys.map((v,i)=>[v,ds[i]]):[],rho:numeric(rho),level:numeric(level)/100,null:numeric(nullValue),
    outcomeShares:axis(yMax),rieszShares:axis(dMax),
  })
  const readiness=prepared.kind!=='prepared-panel'?'This analysis requires a prepared panel.':candidate===undefined?'Choose a saved two-period doubly robust DiD run.':candidate.evidence.inference.kind!=='crossFitted'||candidate.evidence.inference.propensityFit===undefined?'Refit this DiD analysis before computing sensitivity. Its saved result predates training-fold standardization and the stricter convergence requirement.':request===null||!request.success?'Enter matching lists of outcome and Riesz shares from 0% to less than 100%, a correlation from −1 to 1, and valid confidence and grid settings.':null
  const session=useJob('sensitivity:did-omitted-variables'),{job}=session
  useRunActivity(onActivity,job.kind==='running'?{label:'DiD omitted-variable sensitivity',progress:null}:null)
  const execute=async()=>{
    if(readiness!==null||prepared.kind!=='prepared-panel'||candidate===undefined||request===null||!request.success)return
    const current=session.start('analysis','Computing DiD sensitivity');if(current===null)return
    try{
      const [{materializePanelInWorker},{runDidSensitivity}]=await Promise.all([import('@/data/client'),import('@/analysis/client')])
      if(!session.current(current))return
      const columns=candidate.run.columns
      const matrix=await materializePanelInWorker(source.file,profile,{unit:prepared.sampling.unitColumn,time:prepared.sampling.timeColumn,outcome:columns[0].column,treatment:columns[1]!.column,covariates:candidate.covariates as ColumnId[]})
      if(!session.current(current))return
      if(!matrix.ok){session.fail(current,describePanelDataProblem(matrix.error));return}
      const m=matrix.value
      const result=await runDidSensitivity(m.values,m.rowCount,2+candidate.covariates.length,m.units,m.periodCodes,request.data)
      if(!session.current(current))return
      if(!result.ok){session.fail(current,describeAnalysisWorkerProblem(result.error));return}
      if(!sameSpecification(result.value.request,request.data)){session.fail(current,'The sensitivity result does not match the requested scenarios.');return}
      const saved=didSensitivityRunSchema.safeParse({kind:'did-sensitivity-run',id:newSensitivityRunId(),estimationRun:candidate.run.id,preparedDataset:prepared.id,createdAt:new Date().toISOString(),evidence:result.value})
      if(!saved.success||!didSensitivityRunMatches(saved.data,estimates)){session.fail(current,'The sensitivity result does not match its saved DiD fit. Refit the source analysis before continuing.');return}
      onRun(saved.data);session.finish(current)
    }catch(error){if(session.current(current))session.fail(current,error instanceof Error?error.message:String(error))}
  }
  const input=(label:string,value:string,change:(v:string)=>void,help:string,type='number')=><label className="block"><ParameterLabel className={fieldLabel} label={label} help={help}/><input aria-label={label} className={field('text','mt-1')} type={type} step="any" value={value} onChange={e=>change(e.target.value)}/></label>
  return <WorkbenchLayout id="sensitivity-did" stage={<section className="@container/panel flex flex-col gap-5"><div><ChapterHeading>Sensitivity</ChapterHeading><p className={chapterIntroSingle}>Examine how omitted variables could change a two-period DiD effect under specified limits on their contribution to outcome differences and treatment selection.</p></div>{selector}
    <section className={panel('p-(--panel-space)')}><fieldset disabled={job.kind==='running'} className="m-0 min-w-0 border-0 p-0"><div className={stepsStack}>
      <SettingsStep number={1} title="Choose the estimate"><label className="block"><span className={fieldLabel}>DiD estimation run</span><Select className={field('text','mt-1')} value={selected} onChange={e=>setSelected(e.target.value)}><option value="">Choose run</option>{candidates.map(c=><option key={c.run.id} value={c.run.id}>{sourceName(c.run)}</option>)}</Select></label>
        {candidate!==undefined&&<p className={fieldHint}>This analysis uses the baseline covariates and estimation settings from the selected DiD run. It examines how omitted variables could change the estimated effect and its uncertainty.</p>}</SettingsStep>
      <SettingsStep number={2} title="Specify confounding scenarios"><div className="grid gap-4 @md/panel:grid-cols-2">
        {input('Outcome shares (%)',y,setY,'Enter percentages separated by commas. Each value is paired with the Riesz share in the same position.','text')}
        {input('Riesz shares (%)',d,setD,'Enter the same number of percentages as outcome shares. This is not a share of treatment variance.','text')}
        {input('Correlation',rho,setRho,'The correlation between the omitted parts of the outcome regression and Riesz representer. Bounds use its absolute value.')}
        {input('Null effect',nullValue,setNullValue,'The effect value used for robustness values and the reference line.')}
        {input('One-sided confidence level (%)',level,setLevel,'Confidence for each bound, above 50% and below 100%. A value of 95% gives a 90% central interval at zero confounding.')}
      </div></SettingsStep>
      <SettingsStep number={3} title="Set the contour grid"><div className="grid gap-4 @md/panel:grid-cols-2">
        {input('Maximum outcome share (%)',yMax,setYMax,'A positive percentage below 100. The grid starts at zero.')}
        {input('Maximum Riesz share (%)',dMax,setDMax,'A positive percentage below 100. The grid starts at zero.')}
        {input('Grid points per axis',points,setPoints,'Use 2 to 51 points per axis. More points increase resolution and computation time.')}
      </div></SettingsStep>
    </div></fieldset>{readiness!==null&&<Alert tone="info" live={false}><p className="m-0">{readiness}</p></Alert>}
      <RunActions className={actionGap} running={job.kind==='running'} onCancel={session.cancel} orbLabel="DiD sensitivity running"><button className={button('signal')} disabled={readiness!==null||session.blocked||job.kind==='running'} onClick={()=>void execute()}>Compute DiD sensitivity</button></RunActions><JobNotice job={job}/>
    </section>{latest!==undefined&&<DidSensitivityResult run={latest} source={estimates.find(r=>r.id===latest.estimationRun)}/>}</section>}
    inspector={{trigger:{label:'Requirements',icon:'contract'},title:'DiD sensitivity requirements',body:<div className="space-y-3 text-body text-muted">
      <p>Use a saved two-period doubly robust DiD fit with baseline covariates. Staggered group-time effects and repeated cross-sections are not supported by this route.</p>
      <dl><dt>Baseline covariates</dt><dd className="m-0">{candidate?.run.columns.slice(2).map(c=>c.name).join(', ')||'Choose a source fit.'}</dd></dl>
      <p>The outcome share describes the fraction of residual variation in outcome differences explained by omitted variables. The Riesz share describes their contribution to the squared norm of the Riesz representer, which determines the comparison weights.</p>
      <p>These limits and their correlation are sensitivity assumptions. They do not estimate how much confounding is present or establish conditional parallel trends.</p>
      <p>Robustness values vary both shares equally until a bound reaches the chosen null. Unequal-share scenarios and contours remain available.</p>
      <p>Reference: <a href="https://arxiv.org/abs/2510.09064" target="_blank" rel="noreferrer">Bach, Klaassen, Kueck, Mattes and Spindler, Sensitivity Analysis for Treatment Effects in Difference-in-Differences Models using Riesz Representation.</a></p>
    </div>}}
    bottom={{trigger:{label:'History',icon:'history'},title:`Sensitivity runs (${runs.length})`,defaultSize:150,body:<>{[...runs].reverse().map(run=><RunFold key={run.id} title="DiD omitted-variable sensitivity" figure={<RunMeta>{[`ATT ${number(run.evidence.estimate)}`,`Robustness ${formatPercent(run.evidence.robustnessValue,{precision:1}).text}`]}</RunMeta>} stamp={<RunMeta>{[`Seed ${run.evidence.request.seed}`,formatTime(run.createdAt)]}</RunMeta>} onDelete={()=>setPendingDelete(run)} deleteLabel="Delete this sensitivity run"><DidSensitivityRecord run={run} source={estimates.find(r=>r.id===run.estimationRun)}/></RunFold>)}<ConfirmDialog open={pendingDelete!==null} title="Delete this sensitivity run?" message="This removes the sensitivity record, not its source DiD fit." confirmLabel="Delete run" danger onConfirm={()=>{if(pendingDelete!==null)onDelete(pendingDelete.id)}} onClose={()=>setPendingDelete(null)}/></>}}/>
}
