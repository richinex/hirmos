import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { field, fieldLabel } from '@/components/ui/recipes'
import type { ColumnId } from '@/domain/dataset'
import type { StaggeredConfiguration } from '@/domain/staggeredDid'

export function StaggeredDidControls({configuration,candidates,onChange}:{readonly configuration:StaggeredConfiguration;readonly candidates:readonly {readonly id:ColumnId;readonly name:string}[];readonly onChange:(value:StaggeredConfiguration)=>void}) {
  const spec=configuration.specification
  const update=(value:Partial<typeof spec>)=>onChange({...configuration,specification:{...spec,...value}})
  return <div className="grid gap-3" data-testid="staggered-did-controls">
    <div><ParameterLabel className={fieldLabel} label="Comparison group" help="Never-treated units do not adopt in the observed panel. Not-yet-treated comparisons use units still untreated beyond the comparison and anticipation window. When every unit eventually adopts, preparation records any restrictions or recoding." />
      <SegmentedControl className="mt-1" ariaLabel="Staggered comparison group" value={spec.controls} onChange={controls=>update({controls})} options={[{value:'never-treated',label:'Never treated'},{value:'not-yet-treated',label:'Not yet treated'}]} /></div>
    <div><ParameterLabel className={fieldLabel} label="Pre-treatment baseline" help="Varying baselines compare successive pre-treatment periods. Universal baselines normalize the period immediately before the anticipation window to zero. Post-treatment group-time estimates use the same baseline under either choice." />
      <SegmentedControl className="mt-1" ariaLabel="Staggered baseline" value={spec.baseline} onChange={baseline=>update({baseline})} options={[{value:'varying',label:'Varying'},{value:'universal',label:'Universal'}]} /></div>
    <div><div className="flex items-center justify-between gap-2"><ParameterLabel className={fieldLabel} label="Adjustment covariates" help="Without covariates, comparisons use outcome changes. With covariates, the panel doubly robust score uses a logistic propensity model and linear comparison-outcome model. Covariates are taken at the comparison baseline; do not include variables affected by treatment." /><SelectionActions selectLabel="Select all staggered DiD covariates" clearLabel="Clear staggered DiD covariates" onSelectAll={()=>onChange({...configuration,covariates:candidates.map(c=>c.id)})} onClear={()=>onChange({...configuration,covariates:[]})} /></div>
      <div role="group" aria-label="Staggered DiD covariates" className="mt-1 flex flex-wrap gap-2">{candidates.map(c=><label key={c.id} className="flex items-center gap-1.5 text-body text-ink"><input type="checkbox" checked={configuration.covariates.includes(c.id)} onChange={e=>onChange({...configuration,covariates:e.target.checked?[...configuration.covariates,c.id]:configuration.covariates.filter(id=>id!==c.id)})} />{c.name}</label>)}</div></div>
    <div><ParameterLabel className={fieldLabel} label="Uncertainty" help="This choice changes standard errors and intervals, not ATT estimates. Analytical uses pointwise intervals; simultaneous bootstrap covers each plotted family jointly. Overall ATT intervals remain pointwise. Units are the independent resampling clusters." />
      <SegmentedControl className="mt-1" ariaLabel="Staggered uncertainty" value={spec.inference.kind} onChange={kind=>update({inference:kind==='analytical'?{kind}:{kind,iterations:spec.inference.kind==='analytical'?999:spec.inference.iterations,seed:spec.inference.kind==='analytical'?731:spec.inference.seed}})} options={[{value:'analytical',label:'Analytical'},{value:'bootstrapPointwise',label:'Pointwise bootstrap'},{value:'bootstrapSimultaneous',label:'Simultaneous bootstrap'}]} /></div>
    <div className="grid grid-cols-2 items-end gap-3 @md/panel:grid-cols-3">
      <label><ParameterLabel className={fieldLabel} label="Anticipation periods" help="The number of periods before adoption in which treatment may already affect outcomes." /><input aria-label="Anticipation periods" className={field('text','mt-1 w-full')} type="number" min={0} step={1} value={spec.anticipation} onChange={e=>update({anticipation:Number(e.target.value)})} /></label>
      <label><span className={fieldLabel}>Confidence level</span><input aria-label="Staggered confidence level" className={field('text','mt-1 w-full')} type="number" min={0.01} max={0.999} step={0.01} value={spec.confidence} onChange={e=>update({confidence:Number(e.target.value)})} /></label>
      {spec.inference.kind!=='analytical'&&<>
        <label><span className={fieldLabel}>Replications</span><input aria-label="Staggered bootstrap replications" className={field('text','mt-1 w-full')} type="number" min={1} step={1} value={spec.inference.iterations} onChange={e=>{if(spec.inference.kind!=='analytical')update({inference:{...spec.inference,iterations:Number(e.target.value)}})}} /></label>
        <label><span className={fieldLabel}>Seed</span><input aria-label="Staggered bootstrap seed" className={field('text','mt-1 w-full')} type="number" min={0} step={1} value={spec.inference.seed} onChange={e=>{if(spec.inference.kind!=='analytical')update({inference:{...spec.inference,seed:Number(e.target.value)}})}} /></label>
      </>}
    </div>
    <details><DisclosureSummary className="cursor-pointer text-body text-ink">Event window</DisclosureSummary><div className="mt-3 grid grid-cols-2 items-end gap-3 @md/panel:grid-cols-3">
      <label><ParameterLabel className={fieldLabel} label="First event time" help="Periods relative to adoption. Leave blank to retain all supported event times." /><input aria-label="First event time" className={field('text','mt-1 w-full')} type="number" step={1} value={spec.firstEvent??''} onChange={e=>update({firstEvent:e.target.value===''?null:Number(e.target.value)})} /></label>
      <label><span className={fieldLabel}>Last event time</span><input aria-label="Last event time" className={field('text','mt-1 w-full')} type="number" step={1} value={spec.lastEvent??''} onChange={e=>update({lastEvent:e.target.value===''?null:Number(e.target.value)})} /></label>
      <label><ParameterLabel className={fieldLabel} label="Balance through" help="Keep cohorts observed for at least this many post-adoption periods and restrict dynamic effects accordingly. Leave blank for all available cohort support. This changes the dynamic aggregation, not the cohort or calendar summaries." /><input aria-label="Balance event support through" className={field('text','mt-1 w-full')} type="number" min={0} step={1} value={spec.balance??''} onChange={e=>update({balance:e.target.value===''?null:Number(e.target.value)})} /></label>
    </div></details>
  </div>
}
