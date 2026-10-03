import {SurrogatePanel} from './SurrogatePanel'
import type {SurrogateRun,SurrogateRunId} from '@/domain/surrogateRun'
import {useCallback,useEffect,useState,type ComponentProps} from 'react'
import {EstimationPanel} from './EstimationPanel'
import {RegressionDesignControls,RegressionDesignSummary} from './RegressionDesignControls'
import {CountRegressionPanel} from '@/components/time-series/CountRegressionPanel'
import {PanelRegressionPanel} from '@/components/time-series/PanelRegressionPanel'
import {SegmentedControl} from '@/components/ui/SegmentedControl'
import {useWorkflow} from '@/components/WorkflowProvider'
import {useTimeSeriesDraft} from '@/components/time-series/useTimeSeriesDraft'
import type {PanelRegressionDraft} from '@/domain/panelRegression'
import {assessCountOutcome,regressionDesignInfo,transitionRegressionDesign,type RegressionDesign,type CountOutcomeReadiness} from '@/domain/regressionDesigns'
import {assertNever} from '@/domain/dop'
import type {ColumnId} from '@/domain/dataset'
import {identificationAllowsEstimation} from '@/domain/study'
import type {RunActivity} from '@/domain/activity'
import type {TimeSeriesRun,TimeSeriesRunId} from '@/domain/timeSeries'

type Props = ComponentProps<typeof EstimationPanel> & {
  readonly surrogateRuns: readonly SurrogateRun[]
  readonly onSurrogateRun: (run:SurrogateRun)=>void
  readonly onDeleteSurrogateRun: (id:SurrogateRunId)=>void
  readonly designRuns: readonly TimeSeriesRun[]
  readonly onDesignRun: (run: TimeSeriesRun) => void
  readonly onDeleteDesignRun: (id: TimeSeriesRunId) => void
}

/** Selection is a supported specification; the numerical routes keep their existing contracts. */
export function EstimationWorkspace({designRuns,onDesignRun,onDeleteDesignRun,surrogateRuns,onSurrogateRun,onDeleteSurrogateRun,...props}: Props) {
  const [view,setView] = useState<'effects'|'designs'|'surrogates'>(() =>
    surrogateRuns.length > 0 && props.runs.length === 0 && designRuns.length === 0 ? 'surrogates'
      : props.identifications.some(record => identificationAllowsEstimation(record.result.kind)) ? 'effects' : 'designs')
  const [busy,setBusy] = useState(false)
  const design=useTimeSeriesDraft(props.prepared.id,state=>state.regressionDesign)
  const draft=useTimeSeriesDraft(props.prepared.id,state=>state.panelRegression)
  const cohortDraft=useTimeSeriesDraft(props.prepared.id,state=>state.cohortRegression)
  const change=useWorkflow(state=>state.changeTimeSeries)
  const info=regressionDesignInfo(design)
  const outcome=info.model==='count'?cohortDraft.outcome:draft.outcome
  const checkKey=`${props.prepared.id}:${props.profile.source.fingerprint}:${outcome??''}`
  const [checked,setChecked]=useState<{readonly key:string;readonly readiness:CountOutcomeReadiness}|null>(null)
  const countReadiness:CountOutcomeReadiness=outcome===null?{kind:'unselected'}:checked?.key===checkKey?checked.readiness:{kind:'checking'}
  useEffect(()=>{
    if(view!=='designs'||info.model===null||outcome===null)return
    let active=true
    setChecked({key:checkKey,readiness:{kind:'checking'}})
    void (async()=>{
      try{
        const {materialisePrepared,describePreparedMaterialisationProblem}=await import('@/data/prepared')
        const result=await materialisePrepared(props.source,props.profile,props.prepared,[outcome])
        if(!active)return
        setChecked({key:checkKey,readiness:result.ok?assessCountOutcome(result.value.values):{kind:'refused',reason:describePreparedMaterialisationProblem(result.error)}})
      }catch(error:unknown){if(active)setChecked({key:checkKey,readiness:{kind:'refused',reason:error instanceof Error?error.message:String(error)}})}
    })()
    return ()=>{active=false}
  },[view,info.model,outcome,checkKey,props.source,props.profile,props.prepared])
  const selectDesign=(next:RegressionDesign)=>{
    const updated=transitionRegressionDesign({selection:design,linear:draft,cohort:cohortDraft},next)
    if(updated.linear!==draft)change(props.prepared.id,{type:'panel-regression',value:updated.linear})
    if(updated.cohort!==cohortDraft)change(props.prepared.id,{type:'cohort-regression',value:updated.cohort})
    change(props.prepared.id,{type:'regression-design',value:updated.selection})
  }
  const openBacon=(outcome:ColumnId,treatment:ColumnId)=>{
    change(props.prepared.id,{type:'panel-regression',value:{...draft,outcome,model:{kind:'bacon',onset:treatment,covariates:[]}}})
    change(props.prepared.id,{type:'regression-design',value:{kind:'bacon'}})
    setView('designs')
  }
  const restoreDesign=(controls:PanelRegressionDraft)=>{
    const kind=controls.model.kind==='eventStudy'?'linear-event-study':controls.model.kind
    change(props.prepared.id,{type:'regression-design',value:{kind}})
  }
  const report=props.onActivity
  const onActivity=useCallback((activity:RunActivity|null)=>{setBusy(activity!==null);report?.(activity)},[report])
  const selector=<SegmentedControl variant="line" size="sm" ariaLabel="Estimation workspace" value={view} disabled={busy} onChange={setView}
    options={[{value:'effects',label:'Effect estimation'},{value:'designs',label:'Regression designs'},{value:'surrogates',label:'Two-sample surrogates'}]}/>
  const analysisSelector=<RegressionDesignControls design={design} panel={props.prepared.kind==='prepared-panel'} busy={busy} countReadiness={countReadiness} onChange={selectDesign}/>
  if(view==='surrogates')return <SurrogatePanel source={props.source} profile={props.profile} selector={selector} runs={surrogateRuns} onRun={onSurrogateRun} onDeleteRun={onDeleteSurrogateRun} onActivity={onActivity}/>
  if(view==='effects')return <EstimationPanel {...props} selector={selector} onActivity={onActivity} onOpenBacon={openBacon}/>
  switch(design.kind){
    case 'count-event-study':
    case 'count-cohort-summary':
      return <CountRegressionPanel scope="cohorts" source={props.source} profile={props.profile} prepared={props.prepared}
        runs={designRuns} onRun={onDesignRun} onDeleteRun={onDeleteDesignRun} selector={selector} analysisSelector={analysisSelector}
        readiness={countReadiness} runLabel={info.runLabel} inspectorLead={<RegressionDesignSummary design={design}/>} onActivity={onActivity}/>
    case 'linear-event-study':
    case 'interactions':
    case 'bacon':
      return <PanelRegressionPanel source={props.source} profile={props.profile} prepared={props.prepared}
        runs={designRuns} onRun={onDesignRun} onDeleteRun={onDeleteDesignRun} selector={selector} analysisSelector={analysisSelector}
        runLabel={info.runLabel} onRestore={restoreDesign} inspectorLead={<RegressionDesignSummary design={design}/>} onActivity={onActivity}/>
    default:return assertNever(design)
  }
}
