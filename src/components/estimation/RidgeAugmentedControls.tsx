import { SettingsStep } from '@/components/ui/SettingsStep'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { field, fieldHint, fieldLabel, fieldRow, stepsStack } from '@/components/ui/recipes'
import { defaultRidgeConfiguration, type RidgeConfiguration } from '@/domain/ridgeAugmented'
import type { DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { usePanelCatalog } from './usePanelCatalog'
export function RidgeAugmentedControls({configuration:c,onChange,source,profile,prepared}:{readonly configuration:RidgeConfiguration;readonly onChange:(c:RidgeConfiguration)=>void;readonly source:SelectedSource;readonly profile:DatasetProfile;readonly prepared:PreparedDatasetArtifact}){
  const job=usePanelCatalog(source,profile,prepared)
  if(job.kind!=='ready')return <p className={fieldHint}>{job.kind==='loading'?'Reading panel units and periods…':job.detail}</p>
  const catalog=job.catalog,r=c.regularization
  return <div className={stepsStack}>
    <SettingsStep number={1} title="Define the comparison" className="[column-span:all]">
      <div className={fieldRow.two}>
        <label><ParameterLabel className={fieldLabel} label="Treated unit" help="Compare this unit with untreated donors."/><Select className={field('text','mt-1 w-full')} aria-label="Augmented treated unit" value={c.treatedUnit??''} onChange={e=>onChange({...c,treatedUnit:e.target.value||null,donorUnits:c.donorUnits.filter(u=>u!==e.target.value)})}><option value="">Choose a unit</option>{catalog.units.map(u=><option key={u.label} value={u.label}>{u.label}</option>)}</Select></label>
        <label><ParameterLabel className={fieldLabel} label="Intervention period" help="Earlier periods fit the model. At least two pre-treatment periods and one post-treatment period are required."/><Select className={field('text','mt-1 w-full')} aria-label="Augmented intervention period" value={c.interventionPeriod??''} onChange={e=>{const p=job.panelPeriods.find(p=>String(p.code)===e.target.value);if(p!==undefined)onChange({...c,interventionPeriod:p.code})}}><option value="">Choose a period</option>{job.panelPeriods.map(p=><option key={p.code} value={p.code}>{p.label}</option>)}</Select></label>
      </div>
      <div className="max-h-56 overflow-y-auto rounded-md bg-well p-3"><ColumnChecklist title="Donor units" help="Choose at least two untreated donors. Ridge augmentation can produce negative donor weights." columns={catalog.units.map(u=>({id:u.label,name:u.label}))} selected={c.donorUnits} reserved={c.treatedUnit===null?[]:[c.treatedUnit]} onChange={donorUnits=>onChange({...c,donorUnits})}/></div>
    </SettingsStep>
    <SettingsStep number={2} title="Regularize the outcome model">
      <div><ParameterLabel className={fieldLabel} label="Lambda" help="Ridge augmentation uses an outcome model to correct imbalance in the synthetic control’s pre-treatment outcomes."/>
        <SegmentedControl className="mt-1" ariaLabel="Ridge regularization" value={r.kind} onChange={kind=>onChange({...c,regularization:kind==='fixed'?{kind:'fixed',lambda:1}:defaultRidgeConfiguration.regularization})} options={[{value:'crossValidation',label:'Cross-validation'},{value:'fixed',label:'Specified lambda'}]}/></div>
      {r.kind==='fixed'?<label><span className={fieldLabel}>Lambda</span><input className={field('text','mt-1 w-full')} aria-label="Ridge lambda" type="number" min={0.00000001} step="any" value={r.lambda} onChange={e=>onChange({...c,regularization:{...r,lambda:Number(e.target.value)}})}/></label>:<>
        <div><span className={fieldLabel}>Selection rule</span>
          <SegmentedControl className="mt-1" ariaLabel="Ridge selection rule" value={r.selection} onChange={selection=>onChange({...c,regularization:{...r,selection}})} options={[{value:'oneStandardError',label:'One standard error'},{value:'minimumError',label:'Minimum error'}]}/></div>
        <div className={fieldRow.two}><label><ParameterLabel className={fieldLabel} label="Held-out block length" help="Cross-validation leaves out consecutive pre-treatment periods."/><input className={field('text','mt-1 w-full')} aria-label="Ridge held-out block length" type="number" min={1} value={r.holdoutLength} onChange={e=>onChange({...c,regularization:{...r,holdoutLength:Number(e.target.value)}})}/></label>
        <label><ParameterLabel className={fieldLabel} label="Candidate grid steps" help="The grid contains this number of steps plus its starting candidate."/><input className={field('text','mt-1 w-full')} aria-label="Ridge grid steps" type="number" min={1} value={r.steps} onChange={e=>onChange({...c,regularization:{...r,steps:Number(e.target.value)}})}/></label></div>
      </>}
    </SettingsStep>
    <SettingsStep number={3} title="Report uncertainty">
      <div><ParameterLabel className={fieldLabel} label="Uncertainty" help="Jackknife intervals refit with held-out pre-treatment periods. The initially selected lambda stays fixed during these refits. This route does not compute conformal intervals."/>
      <SegmentedControl className="mt-1" ariaLabel="Augmented uncertainty" value={c.uncertainty.kind} onChange={kind=>onChange({...c,uncertainty:kind==='none'?{kind}:{kind,confidence:0.95}})} options={[{value:'jackknifePlus',label:'Jackknife+'},{value:'conservative',label:'Conservative'},{value:'none',label:'No interval'}]}/></div>
      {c.uncertainty.kind!=='none'&&<div className={fieldRow.two}><label><span className={fieldLabel}>Confidence level</span><Select className={field('text','mt-1 w-full')} aria-label="Augmented confidence level" value={c.uncertainty.confidence} onChange={e=>{const confidence=[0.9,0.95,0.99].find(v=>String(v)===e.target.value);if(confidence!==undefined&&c.uncertainty.kind!=='none')onChange({...c,uncertainty:{...c.uncertainty,confidence}})}}>{[0.9,0.95,0.99].map(v=><option key={v} value={v}>{Math.round(v*100)}%</option>)}</Select></label></div>}
    </SettingsStep>
  </div>
}
