import {useEffect,useState} from 'react'
import type {ColumnId,DatasetProfile} from '@/domain/dataset'
import type {PreparedDatasetArtifact} from '@/domain/preprocessing'
import type {SelectedSource} from '@/domain/workflow'
import {cohortAdoption,describePanelProblem} from '@/domain/regularPanel'

export type CohortPeriods=
  | {readonly kind:'unselected'}
  | {readonly kind:'checking'}
  | {readonly kind:'ready';readonly labels:readonly string[]}
  | {readonly kind:'refused';readonly reason:string}

/** Read the prepared panel once and expose observed adoption labels, never row positions. */
export function useCohortPeriods(enabled:boolean,source:SelectedSource,profile:DatasetProfile,prepared:PreparedDatasetArtifact,onset:ColumnId|null):CohortPeriods{
  const key=`${prepared.id}:${profile.source.fingerprint}:${onset??''}`
  const [state,setState]=useState<{readonly key:string;readonly value:CohortPeriods}|null>(null)
  useEffect(()=>{
    if(!enabled||onset===null||prepared.kind!=='prepared-panel')return
    let active=true
    setState({key,value:{kind:'checking'}})
    void(async()=>{
      const refuse=(reason:string)=>{if(active)setState({key,value:{kind:'refused',reason}})}
      try{
        const {materialisePrepared,describePreparedMaterialisationProblem}=await import('@/data/prepared')
        const matrix=await materialisePrepared(source,profile,prepared,[onset])
        if(!active)return
        if(!matrix.ok){refuse(describePreparedMaterialisationProblem(matrix.error));return}
        const {prepareRegularPanel}=await import('@/data/regularPanelInput')
        const panel=await prepareRegularPanel(source,profile,prepared,matrix.value,()=>active)
        if(!active)return
        if(!panel.ok){refuse(describePanelProblem(panel.error));return}
        const adoption=cohortAdoption(panel.value,0)
        if(!adoption.ok){refuse(describePanelProblem(adoption.error));return}
        const codes=[...new Set(adoption.value.flatMap(([,period])=>period!==null&&period>0?[period]:[]))].sort((a,b)=>a-b)
        const labels=codes.map(code=>panel.value.periods[code]!)
        if(labels.length===0){refuse('No observed adoption cohort has a pre-treatment period.');return}
        setState({key,value:{kind:'ready',labels}})
      }catch(error:unknown){refuse(error instanceof Error?error.message:String(error))}
    })()
    return ()=>{active=false}
  },[enabled,source,profile,prepared,onset,key])
  if(!enabled||onset===null)return {kind:'unselected'}
  if(prepared.kind!=='prepared-panel')return {kind:'refused',reason:'Prepare panel data to select an adoption cohort.'}
  return state?.key===key?state.value:{kind:'checking'}
}
