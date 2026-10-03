import {SurrogateCategoryPicker} from './SurrogateCategoryPicker'
import {SurrogateHorizonControls,surrogateHorizonDraftProblem,emptySurrogateHorizonDraft,restoreSurrogateHorizons,requestedSurrogateHorizons,type SurrogateHorizonDraft} from './SurrogateHorizonControls'
import {surrogateHorizonRequests} from '@/domain/surrogateHorizons'
import type {SurrogatePathEvidence} from '@/domain/surrogatePath'
import type {SurrogateBiasRestriction,SurrogateDiagnosticEvidence} from '@/domain/surrogateDiagnostics'
import {surrogateDiagnosticsFor} from '@/domain/surrogateSamples'
import { useState, type ReactNode } from 'react'
import { ParameterHelp, ParameterLabel } from '@/components/ui/ParameterLabel'
import { RunFold } from '@/components/ui/RunFold'
import { RunMeta } from '@/components/ui/RunMeta'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { useJob } from '@/analysis/JobsProvider'
import { useRunActivity } from '@/lib/useRunActivity'
import type { RunActivity } from '@/domain/activity'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'
import { surrogateSelectionSchema, type SurrogateSelection } from '@/domain/surrogateSamples'
import { surrogateRunSchema, type SurrogateRun, type SurrogateRunId } from '@/domain/surrogateRun'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { RunActions } from '@/components/ui/RunActions'
import { JobNotice } from '@/components/ui/JobNotice'
import { Alert } from '@/components/ui/Alert'
import { actionGap, button, chapterIntro, field, fieldLabel, fieldRow, panel, stepsStack } from '@/components/ui/recipes'
import { SurrogateResult, surrogateLabels } from './SurrogateResult'

interface Draft {
  readonly experimentalCategories:readonly string[];readonly observationalCategories:readonly string[]
  readonly horizons:SurrogateHorizonDraft
  readonly sample: ColumnId | null; readonly experimental: string; readonly observational: string
  readonly treatment: ColumnId | null; readonly outcome: ColumnId | null
  readonly surrogates: readonly ColumnId[]; readonly baseline: readonly ColumnId[]
  readonly estimator: SurrogateSelection['estimator']; readonly uncertainty: 'none' | 'bootstrap'
  readonly validation: 'none' | 'observedOutcome'; readonly bound: 'none' | SurrogateBiasRestriction['kind']; readonly maximum: string
  readonly repetitions: string; readonly seed: string; readonly rationale: string
}
const initial = (): Draft => ({ experimentalCategories:[],observationalCategories:[],horizons:emptySurrogateHorizonDraft(), sample: null, experimental: '1', observational: '0', treatment: null, outcome: null,
  validation:'none', bound:'none', maximum:'0.1', surrogates: [], baseline: [], estimator: 'index', uncertainty: 'none', repetitions: '200', seed: '1', rationale: '' })
function restore(run: SurrogateRun): Draft {
  const s = run.selection
  return { experimentalCategories:s.sample.kind==='categories'?s.sample.experimental:[],observationalCategories:s.sample.kind==='categories'?s.sample.observational:[],horizons:restoreSurrogateHorizons(s.horizons), sample: s.sample.column, experimental: s.sample.kind==='numeric'?String(s.sample.experimental):s.sample.experimental.join('\n'), observational: s.sample.kind==='numeric'?String(s.sample.observational):s.sample.observational.join('\n'),
    treatment: s.treatment, outcome: s.outcome, surrogates: s.surrogates, baseline: s.adjustment.kind === 'none' ? [] : s.adjustment.columns,
    validation:s.checks.validation,bound:s.checks.biasBounds.kind,maximum:'maximum' in s.checks.biasBounds ? String(s.checks.biasBounds.maximum) : '0.1',
    estimator: s.estimator, uncertainty: s.uncertainty.kind, repetitions: s.uncertainty.kind === 'none' ? '200' : String(s.uncertainty.repetitions),
    seed: s.uncertainty.kind === 'none' ? '1' : String(s.uncertainty.seed), rationale: run.rationale }
}
const biasOptions = [
  {value:'none',label:'No bias bounds'},
  {value:'binaryWithoutSurrogacy',label:'Binary outcome: relax surrogacy'},
  {value:'binaryWithoutComparability',label:'Binary outcome: relax comparability'},
  {value:'boundedDirectEffect',label:'Bounded direct effect'},
  {value:'boundedSampleDifference',label:'Bounded difference between samples'},
] as const
const numeric = (value: string) => value.trim() === '' ? NaN : Number(value)

export function SurrogatePanel(props: {
  readonly source: SelectedSource; readonly profile: DatasetProfile; readonly selector: ReactNode
  readonly runs: readonly SurrogateRun[]; readonly onRun: (run: SurrogateRun) => void
  readonly onDeleteRun: (id: SurrogateRunId) => void; readonly onActivity?: (activity: RunActivity | null) => void
}) {
  const [draft, setDraft] = useState<Draft>(() => props.runs.length === 0 ? initial() : restore(props.runs[props.runs.length - 1]!))
  const set = (part: Partial<Draft>) => setDraft(current => ({ ...current, ...part }))
  const session = useJob('estimation:surrogate'), busy = session.job.kind === 'running'
  const [pendingDelete, setPendingDelete] = useState<SurrogateRun | null>(null)
  useRunActivity(props.onActivity, busy ? { label: 'Surrogate estimation', progress: null } : null)
  const columns = props.profile.columns.filter(c => isNumericDuckDbType(c.duckdbType))
  const membershipColumn=props.profile.columns.find(c=>c.id===draft.sample)
  const categorical=membershipColumn!==undefined && !isNumericDuckDbType(membershipColumn.duckdbType)
  const roles = [draft.sample, draft.treatment, draft.outcome].filter((id): id is ColumnId => id !== null)
  const specification = surrogateSelectionSchema.safeParse({ sample: categorical?{kind:'categories',column:draft.sample,experimental:draft.experimentalCategories,observational:draft.observationalCategories}:{kind:'numeric',column:draft.sample,experimental:numeric(draft.experimental),observational:numeric(draft.observational)},
    checks:{validation:draft.validation,biasBounds:draft.bound==='boundedDirectEffect'||draft.bound==='boundedSampleDifference'?{kind:draft.bound,maximum:numeric(draft.maximum)}:{kind:draft.bound}},
    horizons:requestedSurrogateHorizons(draft.horizons,columns.filter(c=>draft.surrogates.includes(c.id))),
    treatment: draft.treatment, outcome: draft.outcome, surrogates: draft.surrogates,
    adjustment: draft.baseline.length === 0 ? { kind: 'none' } : { kind: 'baseline', columns: draft.baseline }, estimator: draft.estimator,
    uncertainty: draft.uncertainty === 'none' ? { kind: 'none' } : { kind: 'bootstrap', repetitions: numeric(draft.repetitions), seed: numeric(draft.seed) } })
  const problem = roles.length !== 3 ? 'Choose the sample, treatment and long-term outcome columns.' : draft.surrogates.length === 0 ? 'Choose at least one short-term surrogate.' :
    surrogateHorizonDraftProblem(draft.horizons,columns.filter(c=>draft.surrogates.includes(c.id))) ?? (!specification.success ? specification.error.issues.map(i => i.code==='custom'?i.message:'Check the sample values, bootstrap settings and maximum deviation.').filter((v,i,a)=>a.indexOf(v)===i).join(' ') : draft.rationale.trim() === '' ? 'Record the rationale for the identifying assumptions.' : null)
  const heading = (label: string, help?: string) => help === undefined ? <span className={fieldLabel}>{label}</span> : <ParameterLabel className={fieldLabel} label={label} help={help}/>
  const choose = (label: string, value: ColumnId | null, change: (id: ColumnId | null) => void, choices:readonly DatasetProfile['columns'][number][]=columns, help?: string) => <label>{heading(label, help)}<Select aria-label={label} className={field('text', 'mt-1')} value={value ?? ''} onChange={e => change(choices.find(c => c.id === e.target.value)?.id ?? null)}><option value="">Choose a column</option>{choices.map(c => <option key={c.id} value={c.id}>{c.name}</option>)}</Select></label>
  const input = (label: string, value: string, change: (value: string) => void, help?: string) => <label>{heading(label, help)}<input className={field('text', 'mt-1')} aria-label={label} value={value} onChange={e => change(e.target.value)}/></label>
  const execute = async () => {
    if (!specification.success || problem !== null) return
    const current = session.start('analysis', 'Preparing the two samples')
    if (current === null) return
    try {
      const [{ prepareSurrogateSamples }, { runSurrogate, runSurrogateDiagnostics, runSurrogatePath }] = await Promise.all([import('@/data/surrogateInput'), import('@/analysis/client')])
      const samples = await prepareSurrogateSamples(props.source, props.profile, specification.data)
      if (!session.current(current)) return
      if (!samples.ok) {
        const e = samples.error
        session.fail(current, e.kind === 'missing-value' ? `Source row ${e.row + 1} requires an observed ${e.name} value for ${e.sample === 'membership' ? 'sample membership' : `the ${e.sample} sample`}. No rows were dropped.` : 'detail' in e ? e.detail : 'The selected source and columns no longer match this specification.')
        return
      }
      session.progress(current, 'Estimating the long-term effect')
      const fitted = await runSurrogate(samples.value.request)
      if (!session.current(current)) return
      if (!fitted.ok) { session.fail(current, describeAnalysisWorkerProblem(fitted.error)); return }
      const diagnostics: SurrogateDiagnosticEvidence[] = []
      for (const request of surrogateDiagnosticsFor(samples.value, specification.data)) {
        session.progress(current, request.analysis.kind === 'validation' ? 'Fitting validation regressions' : 'Calculating bias bounds')
        const result = await runSurrogateDiagnostics(request)
        if (!session.current(current)) return
        if (!result.ok) { session.fail(current,describeAnalysisWorkerProblem(result.error)); return }
        diagnostics.push(result.value)
      }
      const paths:SurrogatePathEvidence[]=[]
      for(const request of surrogateHorizonRequests(samples.value,specification.data)){
        session.progress(current,request.kind==='observedOutcomes'?'Calculating observed outcome paths':'Estimating effects across surrogate windows')
        const result=await runSurrogatePath(request)
        if(!session.current(current))return
        if(!result.ok){session.fail(current,describeAnalysisWorkerProblem(result.error));return}
        paths.push(result.value)
      }
      const saved = surrogateRunSchema.safeParse({ kind: 'surrogate-run', id: crypto.randomUUID(), createdAt: new Date().toISOString(),
        sourceFingerprint: props.profile.source.fingerprint, sourceRows: props.profile.rowCount, selection: specification.data,
        rationale: draft.rationale, diagnostics, paths:{specification:specification.data.horizons,results:paths}, rows: samples.value.rows, evidence: fitted.value })
      if (!saved.success) { session.fail(current, saved.error.message); return }
      props.onRun(saved.data); session.finish(current)
    } catch (e: unknown) { session.fail(current, e instanceof Error ? e.message : String(e)) }
  }
  const latest = props.runs.at(-1)
  const restoreRun = (run: SurrogateRun) => setDraft(restore(run))
  const result = (run: SurrogateRun, current = false) => <SurrogateResult run={run} profile={props.profile} current={current} onRestore={busy ? undefined : () => restoreRun(run)}/>
  return <WorkbenchLayout id="surrogate-estimation"
    inspector={{ trigger: { label: 'Requirements', icon: 'contract' }, title: 'Two-sample identifying assumptions', body: <div className="space-y-3 text-body text-muted">
      <p>The experimental sample contains treatment and short-term surrogates. The observational sample contains the same surrogates and the long-term outcome.</p>
      <p>Surrogacy requires the long-term outcome to be independent of treatment conditional on the surrogates and baseline covariates in the experimental population.</p>
      <p>Comparability requires the conditional distribution of the long-term outcome, given surrogates and baseline covariates, to be the same in both populations.</p>
      <p>A causal interpretation also requires treatment assignment to be unconfounded given the baseline covariates, with sufficient treatment and sample overlap. These assumptions are not established by fitting the model.</p>
      <p>The index uses linear outcome regression. The score and influence-function estimators also use logistic probability models. Baseline covariates must precede treatment.</p>
      <p className="mb-2 mt-2 text-label text-faint">Literature: The Surrogate Index: Combining Short-Term Proxies to Estimate Long-Term Treatment Effects More Rapidly and Precisely (Athey, Chetty, Imbens and Kang, 2019), NBER Working Paper 26463</p>
    </div> }}
    bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Surrogate runs (${props.runs.length})`, defaultSize: 150, body: <>
      <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Surrogate runs">
        {props.runs.length === 0 && <li className="px-3 py-2 text-faint">Run a surrogate estimate to record it here.</li>}
        {[...props.runs].reverse().map(run => <RunFold key={run.id} title={surrogateLabels[run.evidence.estimator]} figure={formatStatistic('raw', run.evidence.estimate).text} stamp={formatTime(run.createdAt)} onDelete={() => setPendingDelete(run)} deleteLabel="Delete this surrogate run">{result(run)}</RunFold>)}
      </ul>
      <ConfirmDialog open={pendingDelete !== null} title="Delete this surrogate run?" message="Recorded results cannot be restored." confirmLabel="Delete run" danger onConfirm={() => { if (pendingDelete !== null) props.onDeleteRun(pendingDelete.id) }} onClose={() => setPendingDelete(null)}/>
    </> }}
    stage={<section className="@container/panel flex flex-col gap-5"><div><ChapterHeading className="mb-2">Estimation</ChapterHeading><p className={chapterIntro}>Estimate a long-term treatment effect by combining short-term experimental outcomes with an observational sample containing the long-term outcome.</p></div>
    <section className={panel('p-(--panel-space)')} aria-label="Surrogate setup"><div className="mb-6">{props.selector}</div><fieldset disabled={busy} className="m-0 min-w-0 border-0 p-0"><legend className="sr-only">Surrogate specification</legend><div className={stepsStack}>
      <SettingsStep number={1} title="Define the two samples" help="Use the current source before preprocessing, with one row per independent unit. Combine files in the pipeline and choose a sample-membership column. For named categories, specify the values belonging to each sample. Other membership values are excluded and counted. No missing values are imputed and no incomplete rows are silently removed.">
        <div className={fieldRow.two}>{choose('Sample membership', draft.sample, sample => {const numeric=props.profile.columns.some(c=>c.id===sample&&isNumericDuckDbType(c.duckdbType));set({ sample,experimental:numeric?'1':'',observational:numeric?'0':'',experimentalCategories:[],observationalCategories:[] })},props.profile.columns)}</div>
        {categorical&&draft.sample!==null?<SurrogateCategoryPicker key={draft.sample+props.profile.source.fingerprint} file={props.source.file} profile={props.profile} column={draft.sample} experimental={draft.experimentalCategories} observational={draft.observationalCategories} onChange={(experimentalCategories,observationalCategories)=>set({experimentalCategories,observationalCategories})}/>:<div className={fieldRow.two}>{input('Experimental sample value', draft.experimental, experimental => set({ experimental }))}{input('Observational sample value', draft.observational, observational => set({ observational }))}</div>}
        <div className={fieldRow.two}>{choose('Treatment in experimental sample', draft.treatment, treatment => set({ treatment }), columns, 'Treatment must be 0 or 1 in the experimental sample. The long-term outcome may be missing there; treatment may be missing in the observational sample.')}{choose('Long-term outcome in observational sample', draft.outcome, outcome => set({ outcome }))}</div>
      </SettingsStep>
      <SettingsStep number={2} title="Choose surrogates and baseline covariates"><ColumnChecklist title="Short-term surrogates" help="Observed in both samples, measured on the same scale." columns={columns} selected={draft.surrogates} reserved={[...roles, ...draft.baseline]} onChange={surrogates => set({ surrogates })}/><ColumnChecklist title="Baseline covariates" help="Optional pre-treatment covariates observed in both samples." columns={columns} selected={draft.baseline} reserved={[...roles, ...draft.surrogates]} onChange={baseline => set({ baseline })}/></SettingsStep>
      <SettingsStep number={3} title="Choose estimation and uncertainty">
        <SegmentedControl className="w-fit max-w-full" ariaLabel="Surrogate estimator" value={draft.estimator} onChange={estimator => set({ estimator })} options={[{ value: 'index', label: 'Index' }, { value: 'score', label: 'Score' }, { value: 'influenceFunction', label: 'Influence function' }]}/>
        <SegmentedControl className="w-fit max-w-full" ariaLabel="Surrogate uncertainty" value={draft.uncertainty} onChange={uncertainty => set({ uncertainty })} options={[{ value: 'none', label: 'Point estimate' }, { value: 'bootstrap', label: 'Bootstrap standard error' }]}/>
        {draft.uncertainty === 'bootstrap' ? <div className={fieldRow.two}>{input('Bootstrap repetitions', draft.repetitions, repetitions => set({ repetitions }))}{input('Bootstrap seed', draft.seed, seed => set({ seed }))}</div> : null}
        <label>{heading('Identifying-assumption rationale', 'Explain treatment assignment, surrogacy, comparability and overlap using the study context. Recording a rationale does not verify these assumptions.')}<textarea aria-label="Identifying-assumption rationale" className={field('text', 'mt-1 min-h-24')} value={draft.rationale} onChange={e => set({ rationale: e.target.value })}/></label>
      </SettingsStep>
      <SettingsStep number={4} title="Validation and sensitivity">
        <div className="flex items-center gap-1.5"><SegmentedControl className="w-fit max-w-full" ariaLabel="Surrogate validation" value={draft.validation} onChange={validation=>set({validation})} options={[{value:'none',label:'No validation outcome'},{value:'observedOutcome',label:'Outcome observed in both samples'}]}/><ParameterHelp label="Surrogate validation" help="Validation fits linear regressions of the long-term outcome on the selected surrogates and baseline covariates, adding treatment in the experimental sample and sample membership in the pooled sample. It requires the observed outcome in both samples. Small coefficients or non-rejection do not establish surrogacy or comparability."/></div>
        <div className={fieldRow.two}><label>{heading('Bias-bound restriction', "Bounds describe the true effect minus the surrogate estimand, not sampling uncertainty. Each restriction relaxes one identifying assumption while retaining the others. Binary-outcome bounds require values and fitted index predictions between 0 and 1; predictions are not clipped. A bounded deviation is specified in the outcome's recorded units.")}<Select aria-label="Bias-bound restriction" className={field('text','mt-1')} value={draft.bound} onChange={e=>{const kind=biasOptions.find(o=>o.value===e.target.value)?.value;if(kind!==undefined)set({bound:kind})}}>{biasOptions.map(o=><option key={o.value} value={o.value}>{o.label}</option>)}</Select></label>
        {draft.bound==='boundedDirectEffect'||draft.bound==='boundedSampleDifference'?input('Maximum deviation (outcome units)',draft.maximum,maximum=>set({maximum})):null}</div>
      </SettingsStep>
      <SettingsStep number={5} title="Outcome paths and surrogate horizons"><SurrogateHorizonControls draft={draft.horizons} onChange={horizons=>set({horizons})} columns={columns} surrogates={draft.surrogates} reserved={[...draft.baseline,...[draft.sample,draft.treatment].filter((id):id is ColumnId=>id!==null)]}/></SettingsStep>
    </div></fieldset>
    {problem !== null ? <Alert tone="info" live={false} className="mt-6"><p className="m-0">{problem}</p></Alert> : null}
    <RunActions className={actionGap} running={busy} onCancel={session.cancel} orbLabel="Surrogate estimation running"><button className={button('signal')} disabled={busy || session.blocked || problem !== null} aria-busy={busy} onClick={() => void execute()}>Estimate long-term effect</button></RunActions><JobNotice job={session.job}/></section>
    {latest !== undefined && result(latest, true)}
  </section>}/>
}
