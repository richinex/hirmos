import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { Metadata } from '@/components/ui/Metadata'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { RunFold } from '@/components/ui/RunFold'
import { RunMeta } from '@/components/ui/RunMeta'
import { Select } from '@/components/ui/Select'
import { CausalHierarchy } from './CausalHierarchy'
import { useId, useMemo, useState } from 'react'
import { useJob } from '@/analysis/JobsProvider'
import { useWorkflow } from '@/components/WorkflowProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import type { RunActivity } from '@/domain/activity'
import { useRunActivity } from '@/lib/useRunActivity'
import { literatureOf, MethodCaveats, RequirementsFold } from '@/components/MethodCaveats'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { LagGraphViews } from '@/components/discovery/LagGraphViews'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { RefusalTile } from '@/components/ui/figures'
import { RadioList } from '@/components/ui/RadioList'
import { Formula } from '@/components/ui/Formula'
import { button, chapterIntro, chip, field, fieldHint, fieldLabel, fieldRow, label, literal, num, panel, prose, sectionTitle, stepsStack, well } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { cn } from '@/lib/utils'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { formatCount } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { DagDocument, DagDocumentId, DagNodeId } from '@/domain/dag'
import { assertNever } from '@/domain/dop'
import { lagGraphFromDag } from '@/domain/lagGraph'
import { BACKDOOR_IDENTIFICATION_METHOD_ID, COUNTERFACTUAL_IDENTIFICATION_METHOD_ID, GRAPHICAL_IDENTIFICATION_METHOD_ID, IDENTIFICATION_METHODS } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import { roleWord, roleDetail, type DagCausalRole } from '@/domain/dagFlow'
import { IdentityRow } from '@/components/ui/IdentityRow'
import {
  backdoorIdentificationCommand,
  describeIdentificationFailure,
  describeStudyDesignProblem,
  estimableIdentification,
  estimandSentence,
  identifiedExpression,
  offersAdjustmentChoice,
  identifiedExpressionTex,
  CONSISTENCY_STATEMENT,
  DESIGN_ASSUMPTIONS,
  NO_INTERFERENCE_STATEMENT,
  QUANTILE_GROUP_CHOICES,
  studyDesignCategory,
  dagBasisOf,
  describeAssignmentKind,
  describeStudyDesignCategory,
  describeEstimand,
  previewStudyBinding,
  type AssignmentMechanism,
  type Estimand,
  identificationFrom,
  newIdentificationId,
  readyStudySpecification,
  variableRoles,
  type AdjustmentSetChoice,
  type BackdoorIdentificationEvidence,
  type IdentificationArtifact,
  type IdentificationFailure,
  type StudyDesignDraft,
  type StudySpecification,
  type StudyVariable,
} from '@/domain/study'


const ledgerLabel = (result: IdentificationArtifact['result']): string => {
  switch (result.kind) {
    case 'cutoff-design': return 'cutoff design recorded'
    case 'identified': return 'back-door identified'
    case 'graphically-identified': return 'ID expression derived'
    case 'counterfactually-identified': return 'IDC* expressions derived'
    case 'instrument-identified': return 'instruments identified'
    case 'backdoor-not-identified': return 'not identified'
    default: return assertNever(result)
  }
}

const ASSIGNMENT_KINDS: readonly AssignmentMechanism['kind'][] = ['randomised', 'policy-change', 'observed-choice']
const ESTIMAND_KINDS: readonly Estimand['kind'][] = ['average-treatment-effect', 'average-treatment-effect-on-treated', 'local-cutoff-effect', 'conditional-average-treatment-effect', 'conditional-average-treatment-effect-per-row']

const estimandLabel = (kind: Estimand['kind']): string => {
  switch (kind) {
    case 'local-cutoff-effect': return 'At an assignment cutoff (sharp RD)'
    case 'average-treatment-effect': return 'All prepared rows (ATE)'
    case 'average-treatment-effect-on-treated': return 'Treated rows (ATT)'
    case 'conditional-average-treatment-effect': return 'Within groups of a variable (CATE)'
    case 'conditional-average-treatment-effect-per-row': return 'Each row, given its covariates (CATE per row)'
    default: return assertNever(kind)
  }
}

const estimandHint = (kind: Estimand['kind']): string => {
  switch (kind) {
    case 'local-cutoff-effect': return 'Estimate the effect at a cutoff where treatment switches from 0 to 1. This local contrast is not a population-wide ATE.'
    case 'average-treatment-effect': return 'Average the treatment contrast over the prepared population.'
    case 'average-treatment-effect-on-treated': return 'Average the treatment contrast among rows with treatment = 1. Current ETT estimators require a binary treatment; eligibility also depends on the identifying strategy.'
    case 'conditional-average-treatment-effect': return 'Average the treatment contrast within each group of an effect modifier, a variable the treatment does not reach. The double machine learning estimators report the group effects.'
    case 'conditional-average-treatment-effect-per-row': return 'This is the ATE conditioned on specific values of covariates, reported for every prepared row (CATE per row) at that row’s own values of the adjustment variables and any effect modifiers you name.'
    default: return assertNever(kind)
  }
}

/** Whether a variable may define the groups: anything measured that the treatment does not reach. */
const modifierAllowed = (role: DagCausalRole | null): boolean => role === null || !(role.kind === 'mediator' || role.kind === 'collider' || role.kind === 'post-treatment')

const isValidated = (document: DagDocument): boolean => document.current.validation.structure.kind === 'sound'


/** One line on what each assignment mechanism means for the reader choosing it. */
const assignmentHint = (kind: AssignmentMechanism['kind']): string => {
  switch (kind) {
    case 'randomised': return 'An experiment assigned the treatment by chance.'
    case 'policy-change': return 'A rule, law or programme set the treatment at a known time.'
    case 'observed-choice': return 'Units chose the treatment, or circumstances set it.'
    default: return assertNever(kind)
  }
}

function StudyRecord({ study, identification }: { readonly study: StudySpecification; readonly identification: IdentificationArtifact | null }) {
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <DisclosureSummary className="cursor-pointer text-ink">Study details</DisclosureSummary>
      <RecordList className="mt-2 text-label">
        <RecordRow term="Study"><span className={literal('text-muted')} title={study.id}>{study.id.slice(0, 8)}</span></RecordRow>
        <RecordRow term="Identification">{identification === null ? '—' : <span className={literal('text-muted')} title={identification.id}>{identification.id.slice(0, 8)}</span>}</RecordRow>
        <RecordRow term="Graph">{study.dagName}, revision <span className={literal('text-muted')} title={study.dagRevision}>{study.dagRevision.slice(0, 8)}</span></RecordRow>
        <RecordRow term="Prepared dataset"><span className={literal('text-muted')} title={study.preparedDataset}>{study.preparedDataset.slice(0, 8)}</span></RecordRow>
        <RecordRow term="Created">{formatTimestamp(study.createdAt)}</RecordRow>
        <RecordRow term="Method">{identification?.method ?? '—'}</RecordRow>
      </RecordList>
    </details>
  )
}

const variableChips = (variables: readonly StudyVariable[]): React.ReactNode =>
  variables.map((variable, index) => <span key={variable.node}>{index > 0 ? ', ' : ''}<span className={chip()}>{variable.name}</span></span>)

const failureList = (reasons: readonly IdentificationFailure[]): React.ReactNode => (
  <ul className="mb-0 mt-2 list-disc pl-4">{reasons.map((reason) => <li key={`${reason.kind}-${describeIdentificationFailure(reason)}`}>{describeIdentificationFailure(reason)}</li>)}</ul>
)

/** One block per identification outcome; the switch is exhaustive, so a new record kind cannot fall into another kind's copy. */
function IdentificationOutcome({ study, identification, onOpenDag }: {
  readonly study: StudySpecification
  readonly identification: IdentificationArtifact
  readonly onOpenDag: () => void
}) {
  const result = identification.result
  switch (result.kind) {
    case 'cutoff-design': return <Alert tone="info" live={false} className="mt-3"><p className="m-0">Sharp RD design recorded</p><p className="mb-0 mt-1">The local effect relies on continuity at the cutoff and no precise manipulation of the running variable. The graph alone cannot verify these assumptions.</p><Formula tex={identifiedExpressionTex(study, [])} plain={identifiedExpression(study, [])} /></Alert>
    case 'identified': {
      const selectedLabel = result.adjustment.kind === 'canonical' ? 'Canonical adjustment set' : `Minimal adjustment set ${result.adjustment.ordinal + 1}`
      return (
        <Alert tone="ok" icon="function" live={false} className="mt-3">
          <p className="m-0">Identified by back-door adjustment</p>
          <p className="mb-0 mt-1 text-muted">
            {result.adjustment.variables.length === 0
              ? 'No adjustment is needed: no back-door path is open.'
              : <>{selectedLabel} {variableChips(result.adjustment.variables)}</>}
          </p>
          {result.adjustment.kind === 'minimal' && <p className="mb-0 mt-1 text-faint">Canonical set: {result.canonicalAdjustmentSet.map((variable) => variable.name).join(', ') || 'none'}.</p>}
          <details className="mt-2 text-muted">
            <DisclosureSummary className="cursor-pointer text-body text-ink"><Metadata><span>Minimal valid sets</span><span>{result.minimalAdjustmentSets.sets.length}</span></Metadata></DisclosureSummary>
            <ol className="mb-0 mt-1 pl-5">
              {result.minimalAdjustmentSets.sets.map((set, index) => (
                <li key={set.map((variable) => variable.node).join('|') || 'empty'}>
                  {set.length === 0 ? 'No adjustment' : set.map((variable) => variable.name).join(', ')}
                  <span className="text-faint"><Metadata className="ml-3"><span>set {index + 1}</span></Metadata></span>
                </li>
              ))}
            </ol>
            {result.minimalAdjustmentSets.kind === 'truncated' && <p className="mb-0 mt-1 text-warn">The result limit was reached; additional minimal sets may exist.</p>}
          </details>
          <figure className="mb-0 mt-2">
            <figcaption className={label('text-faint')}>Identified expression</figcaption>
            <div className="mt-1">
              <Formula tex={identifiedExpressionTex(study, result.adjustment.variables)} plain={identifiedExpression(study, result.adjustment.variables)} />
            </div>
          </figure>
          {result.instruments.kind === 'identified' && <p className="mb-0 mt-2 text-muted">{variableChips(result.instruments.instruments)} also {result.instruments.instruments.length === 1 ? 'meets' : 'meet'} the two definitional requirements for a valid instrument, so an instrumental variable estimand is available as well.</p>}
        </Alert>
      )
    }
    case 'graphically-identified':
      return (
        <Alert tone="ok" icon="function" live={false} className="mt-3">
          <p className="m-0">Identified by the general ID algorithm</p>
          <p className="mb-0 mt-1 text-muted">The graph has no measured back-door adjustment set, but the interventional distribution can be written using observed probabilities.</p>
          <figure className="mb-0 mt-2">
            <figcaption className={label('text-faint')}>Identified expression</figcaption>
            <div className="mt-1"><Formula tex={result.latex} plain={result.expression} /></div>
          </figure>
          <p className="mb-0 mt-2 text-muted">{result.frontdoor.kind === 'identified' && result.frontdoor.mediators.length === 1
            ? 'The linear two-stage front-door estimator can evaluate this expression under its recorded stage-model assumptions.'
            : result.instruments.kind === 'identified'
              ? 'No available estimator evaluates this expression, but the graph names an instrument, so the instrumental variable estimand is available under its linearity assumption.'
              : 'No available estimator evaluates this expression. Back-door estimators target a different functional.'}</p>
        </Alert>
      )
    case 'counterfactually-identified':
      return (
        <Alert tone="ok" icon="function" live={false} className="mt-3">
          <p className="m-0">Identified by IDC*</p>
          <p className="mb-0 mt-1 text-muted">The effect on the treated is identified through two conditional counterfactual distributions.</p>
          <div className="mt-2 grid gap-2">
            <figure className="m-0">
              <figcaption className={label('text-faint')}>Treated potential outcome</figcaption>
              <code className={literal('figure-strip mt-1 block overflow-x-auto whitespace-nowrap text-body text-ink')}>{result.treatedExpression}</code>
            </figure>
            <figure className="m-0">
              <figcaption className={label('text-faint')}>Untreated potential outcome</figcaption>
              <code className={literal('figure-strip mt-1 block overflow-x-auto whitespace-nowrap text-body text-ink')}>{result.untreatedExpression}</code>
            </figure>
          </div>
          <p className="mb-0 mt-2 text-muted">The available evaluator requires all observed graph variables to be binary and reports a plug-in estimate without a sampling interval.</p>
        </Alert>
      )
    case 'instrument-identified':
      return (
        <Alert tone="ok" icon="function" live={false} className="mt-3">
          <p className="m-0">Instrumental variable estimand</p>
          <p className="mb-0 mt-1 text-muted">
            No measured back-door adjustment set and no observational expression were found, but the graph names {result.instruments.length === 1 ? 'an instrument' : 'instruments'}: {variableChips(result.instruments)}.
          </p>
          <p className="mb-0 mt-2 text-muted">The instrumental variable estimand does not rely on adjusting for all common causes. The level 2 graphical assumptions are not sufficient for instrumental variable identification; additional parametric assumptions are needed, and the estimator makes a linearity assumption.</p>
          <details className="mt-2 text-muted">
            <DisclosureSummary className="cursor-pointer text-body text-ink"><Metadata><span>Why no observational expression</span><span>{result.reasons.length}</span></Metadata></DisclosureSummary>
            {failureList(result.reasons)}
          </details>
        </Alert>
      )
    case 'backdoor-not-identified':
      return (
        <div className="mt-3">
          <RefusalTile
            label={estimandSentence(study)}
            headline="No identifying expression was found"
            reason={<><p className="m-0">This record includes measured adjustment-set enumeration, the level-2 ID algorithm, and the instrument search.</p>{failureList(result.reasons)}<p className="mb-0 mt-2">Revise the graph only when its causal assumptions are incorrect. Otherwise identification requires additional measurements, study-design information, experimental distributions, or stronger assumptions.</p></>}
            rule={`${identification.method}, ${study.dagName} r${study.dagRevision.slice(0, 8)}`}
            actions={<button type="button" className={button('outline')} onClick={onOpenDag}>Review the graph</button>}
          />
        </div>
      )
    default:
      return assertNever(result)
  }
}

function IdentificationCard({ study, identification, current, onContinue, onOpenDag }: {
  readonly study: StudySpecification
  readonly identification: IdentificationArtifact
  readonly current: boolean
  readonly onContinue: () => void
  readonly onOpenDag: () => void
}) {
  const result = identification.result
  const title = estimandSentence(study)
  const body = (
    <>
      <p className="mb-0 mt-2 text-body text-muted" aria-label="Assignment and credibility"><Metadata><span><span className="text-ink">{describeAssignmentKind(study.assignment.kind)} treatment</span></span><span>{study.assignment.description} {describeStudyDesignCategory(studyDesignCategory(study))}</span></Metadata></p>
      <IdentificationOutcome study={study} identification={identification} onOpenDag={onOpenDag} />
      {current && estimableIdentification(result) && (
        <button type="button" className={button('signal', 'mt-4')} onClick={onContinue}>Continue to estimation</button>
      )}
      <StudyRecord study={study} identification={identification} />
    </>
  )
  if (!current) {
    // History rows fold to one line in the studies drawer; only the newest record keeps the stage.
    return (
      <RunFold title={title} figure={ledgerLabel(result)} stamp={<RunMeta>{[study.dagName, formatTime(study.createdAt)]}</RunMeta>}>
        {body}
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label={`${title} identification`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <div>
          <span className={label('text-signal-text')}>Current study</span>
          <h3 className="mb-0 mt-1 text-title font-medium text-ink">{title}</h3>
        </div>
        <span className={num('text-micro text-faint')}><Metadata><span>{formatCount(study.population.observations).text} rows</span><span>{study.dagName}</span></Metadata></span>
      </div>
      {body}
    </article>
  )
}

function AdjustmentSetChoicePanel({ study, evidence, onChoose }: {
  readonly study: StudySpecification
  readonly evidence: BackdoorIdentificationEvidence
  readonly onChoose: (choice: AdjustmentSetChoice) => void
}) {
  if (evidence.result.kind === 'notIdentified') return null
  return (
    <section className="mt-4 border-t border-hair pt-4" aria-labelledby="adjustment-set-choice-title">
      <h4 id="adjustment-set-choice-title" className="m-0 text-body font-medium text-ink">Choose a valid adjustment set</h4>
      <p className={prose('mb-0 mt-1 text-muted')}>{evidence.result.kind === 'identified' && evidence.result.minimalSets.length > 1 ? 'The graph has several minimal valid sets.' : 'The minimal set closes every back-door path. The canonical set adds predictors of the outcome: they close no path, and can narrow the interval.'} Choose using measurement quality, observed support and the planned model—not the estimate, which has not been run.</p>
      <div className="mt-3 grid gap-2">
        {evidence.result.minimalSets.map((set, ordinal) => (
          <button key={set.join('|')} type="button" className={button('outline', 'justify-start text-left')} onClick={() => onChoose({ kind: 'minimal', ordinal })}>
            Minimal set {ordinal + 1}, {set.map((index) => study.graph.nodes[index]?.name ?? String(index)).join(', ') || 'no adjustment'}
          </button>
        ))}
        <button type="button" className={button('quiet', 'justify-start text-left')} onClick={() => onChoose({ kind: 'canonical' })}>
          Canonical set, {evidence.result.canonicalSet.map((index) => study.graph.nodes[index]?.name ?? String(index)).join(', ') || 'no adjustment'}
        </button>
      </div>
      {evidence.result.truncated && <p className="mb-0 mt-2 text-body text-warn">The result limit was reached; additional minimal sets may exist.</p>}
    </section>
  )
}

export function StudyDesignPanel({ prepared, documents, draft, onDraftChanged, studies, identifications, onIdentified, onContinue, onOpenDag, onActivity }: {
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly prepared: PreparedDatasetArtifact
  readonly documents: readonly DagDocument[]
  readonly draft: StudyDesignDraft
  readonly onDraftChanged: (draft: StudyDesignDraft) => void
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly onIdentified: (study: StudySpecification, identification: IdentificationArtifact) => void
  readonly onContinue: () => void
  readonly onOpenDag: () => void
}) {
  const session = useJob('identification')
  const { job } = session
  const adjustmentChoice = useWorkflow(state => state.adjustmentDecision)
  const offerAdjustment = useWorkflow(state => state.offerAdjustment)
  const clearAdjustment = useWorkflow(state => state.clearAdjustment)
  const [choiceProblem, setChoiceProblem] = useState<string | null>(null)
  useRunActivity(onActivity, job.kind === 'running' ? { label: 'Identifying the effect', progress: null } : null)
  const rationaleId = useId()
  const state = { draft, job }
  const chooseDag = (documentId: DagDocumentId | null) => onDraftChanged({ ...draft, dagDocument: documentId, treatment: null, outcome: null })
  const chooseTreatment = (node: DagNodeId | null) => onDraftChanged({ ...draft, treatment: node })
  const chooseOutcome = (node: DagNodeId | null) => onDraftChanged({ ...draft, outcome: node })
  const document = documents.find((candidate) => candidate.id === state.draft.dagDocument) ?? null
  const dagBasis = document === null ? null : dagBasisOf(document)
  const observedNodes = document?.current.graph.nodes.filter((node) => node.kind === 'observed') ?? []
  const readiness = useMemo(() => readyStudySpecification(state.draft, documents, prepared), [documents, prepared, state.draft])
  const preview = useMemo(() => previewStudyBinding(state.draft, documents, prepared), [documents, prepared, state.draft])
  const modifierCandidates = useMemo(() => {
    const roles = preview === null ? null : variableRoles(preview)
    return observedNodes
      .filter((node) => node.id !== state.draft.treatment && node.id !== state.draft.outcome)
      .map((node) => {
        const role = roles?.find((entry) => entry.node.node === node.id)?.role ?? null
        return { node, role, allowed: modifierAllowed(role) }
      })
  }, [observedNodes, preview, state.draft.treatment, state.draft.outcome])
  const evidenceGraph = useMemo(() => (document === null ? null : lagGraphFromDag(document)), [document])
  const latestIdentified = [...identifications].reverse().find((identification) => identification.result.kind !== 'backdoor-not-identified') ?? null
  const newestRecorded = [...studies].reverse().flatMap((study) => {
    const identification = identifications.find((candidate) => candidate.study === study.id)
    return identification === undefined ? [] : [{ study, identification }]
  })[0] ?? null

  const recordIdentification = (study: StudySpecification, evidence: BackdoorIdentificationEvidence, choice: AdjustmentSetChoice) => {
    const result = identificationFrom(study, evidence, choice)
    if (!result.ok) {
      setChoiceProblem(`Adjustment set ${result.error.ordinal + 1} is not available; ${result.error.available} minimal sets were returned.`)
      return
    }
    const identification: IdentificationArtifact = {
      kind: 'identification',
      id: newIdentificationId(),
      study: study.id,
      createdAt: new Date().toISOString(),
      method: result.value.kind === 'cutoff-design' ? RD_DESIGN_METHOD_ID : result.value.kind === 'graphically-identified'
        ? GRAPHICAL_IDENTIFICATION_METHOD_ID
        : result.value.kind === 'counterfactually-identified'
          ? COUNTERFACTUAL_IDENTIFICATION_METHOD_ID
          : BACKDOOR_IDENTIFICATION_METHOD_ID,
      evidence,
      result: result.value,
    }
    onIdentified(study, identification)
    clearAdjustment(study.id)
    setChoiceProblem(null)
  }

  const execute = async () => {
    const ready = readyStudySpecification(state.draft, documents, prepared)
    if (!ready.ok || state.job.kind === 'running') return
    const id = session.start('analysis', 'Identifying the effect')
    if (id === null) return
    setChoiceProblem(null)
    try {
      const analysis = await import('@/analysis/client')
      if (!session.current(id)) return
      const outcome = await analysis.identifyBackdoor(backdoorIdentificationCommand(ready.value))
      if (!session.current(id)) return
      if (!outcome.ok) {
        session.fail(id, describeAnalysisWorkerProblem(outcome.error))
        return
      }
      // The choice is put to the person whenever the graph leaves one: several minimal sets, or a
      // canonical set that adds outcome predictors to the only minimal set. A study is recorded
      // with its identification once, so the set is chosen before, never switched after.
      if (ready.value.estimand.kind !== 'local-cutoff-effect' && outcome.value.result.kind === 'identified' && offersAdjustmentChoice(outcome.value.result)) {
        offerAdjustment({ study: ready.value, evidence: outcome.value })
        session.finish(id)
        return
      }
      recordIdentification(
        ready.value,
        outcome.value,
        outcome.value.result.kind === 'identified' ? { kind: 'minimal', ordinal: 0 } : { kind: 'canonical' },
      )
      session.finish(id)
    } catch (cause: unknown) {
      session.fail(id, cause instanceof Error ? cause.message : String(cause))
    }
  }

  const stage = (
    <section aria-labelledby="study-title" className="@container/panel flex flex-col gap-5">
      <div>
        <ChapterHeading id="study-title" className="mb-2">Study design</ChapterHeading>
        <p className={chapterIntro}>A causal question specifies the treatment, outcome, intervention contrast, effect measure, and target population. Bind the question to a DAG, enumerate measured back-door adjustment sets, and run the ID algorithm to determine whether the interventional distribution can be written using observed probabilities.</p>
      </div>

      <CausalHierarchy />

      <section className={panel('p-(--panel-space)')} aria-labelledby="study-form-title">
        <h3 id="study-form-title" className={cn(sectionTitle, 'mb-6 mt-0')}>Define the estimand and select a graph</h3>
        {documents.length === 0 && (
          <Alert tone="info" live={false}>
            <p className="m-0">Draw and validate a causal graph in the DAG workspace first.</p>
          </Alert>
        )}
        <div className={stepsStack}>
          <SettingsStep number={1} title="Choose the graph and variables">
            <div className={fieldRow.three}>
              <label className="block">
                <span className={fieldLabel}>Causal graph</span>
                <Select
                  className={field('text', 'mt-1')}
                  value={state.draft.dagDocument ?? ''}
                  onChange={(event) => chooseDag(event.target.value === '' ? null : (event.target.value as DagDocumentId))}
                >
                  <option value="">Choose a graph</option>
                  {documents.map((candidate) => (
                    <option key={candidate.id} value={candidate.id} disabled={!isValidated(candidate)}>
                      {candidate.name}{isValidated(candidate) ? '' : ' (needs validation)'}
                    </option>
                  ))}
                </Select>
              </label>
              <label className="block">
                <span className={fieldLabel}>Treatment</span>
                <Select
                  className={field('text', 'mt-1')}
                  value={state.draft.treatment ?? ''}
                  disabled={document === null}
                  onChange={(event) => chooseTreatment(event.target.value === '' ? null : (event.target.value as DagNodeId))}
                >
                  <option value="">Choose a variable</option>
                  {observedNodes.map((node) => <option key={node.id} value={node.id} disabled={node.id === state.draft.outcome}>{node.name}</option>)}
                </Select>
              </label>
              <label className="block">
                <span className={fieldLabel}>Outcome</span>
                <Select
                  className={field('text', 'mt-1')}
                  value={state.draft.outcome ?? ''}
                  disabled={document === null}
                  onChange={(event) => chooseOutcome(event.target.value === '' ? null : (event.target.value as DagNodeId))}
                >
                  <option value="">Choose a variable</option>
                  {observedNodes.map((node) => <option key={node.id} value={node.id} disabled={node.id === state.draft.treatment}>{node.name}</option>)}
                </Select>
              </label>
            </div>
          </SettingsStep>
          <SettingsStep number={2} title="Target population">
            <RadioList
              legend="Target population"
              legendHidden
              className="max-w-3xl"
              value={state.draft.estimand}
              onChange={(estimand) => onDraftChanged({ ...draft, estimand })}
              options={ESTIMAND_KINDS.map((kind) => ({ value: kind, label: estimandLabel(kind), hint: estimandHint(kind) }))}
            />
            {state.draft.estimand === 'local-cutoff-effect' && <div className={fieldRow.two}>
              <label className="block"><span className={fieldLabel}>Running variable</span><Select aria-label="Running variable" className={field('text', 'mt-1')} value={state.draft.cutoff?.variable ?? ''} onChange={event => onDraftChanged({ ...draft, cutoff: { variable: event.target.value === '' ? null : event.target.value as DagNodeId, value: draft.cutoff?.value ?? '0' } })}>
                <option value="">Choose a variable</option>{modifierCandidates.map(({ node, allowed }) => <option key={node.id} value={node.id} disabled={!allowed}>{node.name}</option>)}
              </Select></label>
              <label className="block"><span className={fieldLabel}>Assignment cutoff</span><input aria-label="Assignment cutoff" type="number" step="any" className={field('text', 'mt-1')} value={state.draft.cutoff?.value ?? '0'} onChange={event => onDraftChanged({ ...draft, cutoff: { variable: draft.cutoff?.variable ?? null, value: event.target.value } })} /></label>
            </div>}
            {state.draft.estimand === 'conditional-average-treatment-effect' && (
              <div className={fieldRow.two}>
                <label className="block">
                  <span className={fieldLabel}>Effect modifier</span>
                  <Select
                    className={field('text', 'mt-1')}
                    value={state.draft.modifier ?? ''}
                    disabled={document === null}
                    onChange={(event) => onDraftChanged({ ...draft, modifier: event.target.value === '' ? null : (event.target.value as DagNodeId) })}
                  >
                    <option value="">Choose a variable</option>
                    {modifierCandidates.map(({ node, role, allowed }) => (
                      <option key={node.id} value={node.id} disabled={!allowed}>{node.name}{role === null ? '' : ` — ${roleWord(role)}`}</option>
                    ))}
                  </Select>
                </label>
                <label className="block">
                  <span className={fieldLabel}>Groups</span>
                  <Select
                    className={field('text', 'mt-1')}
                    value={state.draft.grouping.kind === 'levels' ? 'levels' : String(state.draft.grouping.bins)}
                    onChange={(event) => onDraftChanged({ ...draft, grouping: event.target.value === 'levels' ? { kind: 'levels' } : { kind: 'quantiles', bins: Number(event.target.value) } })}
                  >
                    <option value="levels">Each value of the modifier</option>
                    {QUANTILE_GROUP_CHOICES.map((bins) => <option key={bins} value={String(bins)}>{bins} quantile groups</option>)}
                  </Select>
                </label>
              </div>
            )}
            {state.draft.estimand === 'conditional-average-treatment-effect-per-row' && (
              <fieldset className="m-0 min-w-0 border-0 p-0" aria-label="Effect modifiers">
                <legend className={fieldLabel}>Effect modifiers</legend>
                <p className={cn(fieldHint, 'mt-1')}>Variables the effect may vary with, joined to the adjustment set as what each row’s effect is conditioned on.</p>
                <div className="mt-2 grid gap-1.5 @lg/panel:grid-cols-2">
                  {modifierCandidates.map(({ node, role, allowed }) => (
                    <label key={node.id} className={cn('flex items-center gap-2 text-body', allowed ? 'text-ink' : 'text-faint')}>
                      <input
                        type="checkbox"
                        disabled={!allowed || document === null}
                        checked={state.draft.modifiers.includes(node.id)}
                        onChange={(event) => onDraftChanged({ ...draft, modifiers: event.target.checked ? [...draft.modifiers, node.id] : draft.modifiers.filter((id) => id !== node.id) })}
                      />
                      <span>{node.name}{role === null ? '' : <span className="text-faint"><Metadata className="ml-3"><span>{roleWord(role)}</span></Metadata></span>}</span>
                    </label>
                  ))}
                  {modifierCandidates.length === 0 && <p className={cn(fieldHint, 'm-0')}>Choose a graph, treatment and outcome first.</p>}
                </div>
              </fieldset>
            )}
          </SettingsStep>
          <SettingsStep number={3} title="Why the treatment varied">
            <RadioList
              legend="Why the treatment varied"
              legendHidden
              className="max-w-3xl"
              value={state.draft.assignment.kind}
              onChange={(kind) => onDraftChanged({ ...draft, assignment: { ...draft.assignment, kind } })}
              options={ASSIGNMENT_KINDS.map((kind) => {
                const withheld = kind === 'randomised' && dagBasis !== 'experimental-design'
                return { value: kind, label: describeAssignmentKind(kind), disabled: withheld, hint: withheld ? 'Only for a graph built from an experimental design.' : assignmentHint(kind) }
              })}
            />
            <label className="flex min-w-0 max-w-3xl flex-col">
              <span className={fieldLabel}>Assignment sentence</span>
              <textarea
                className={field('text', 'mt-1 min-h-24 flex-1 resize-y rounded-lg')}
                placeholder="One sentence: who or what set the treatment, and when"
                value={state.draft.assignment.description}
                onChange={(event) => onDraftChanged({ ...draft, assignment: { ...draft.assignment, description: event.target.value } })}
              />
            </label>
            <div className={fieldRow.two}>
              <div className="grid gap-y-1">
                <ParameterLabel className={fieldLabel} htmlFor={`${rationaleId}-consistency`} label="Consistency rationale" help={CONSISTENCY_STATEMENT} />
                <input id={`${rationaleId}-consistency`} type="text" className={field('text', 'self-start')} placeholder="Why this holds here (optional)" value={state.draft.consistencyRationale} onChange={(event) => onDraftChanged({ ...draft, consistencyRationale: event.target.value })} />
              </div>
              <div className="grid gap-y-1">
                <ParameterLabel className={fieldLabel} htmlFor={`${rationaleId}-interference`} label="No-interference rationale" help={NO_INTERFERENCE_STATEMENT} />
                <input id={`${rationaleId}-interference`} type="text" className={field('text', 'self-start')} placeholder="Why this holds here (optional)" value={state.draft.noInterferenceRationale} onChange={(event) => onDraftChanged({ ...draft, noInterferenceRationale: event.target.value })} />
              </div>
            </div>
          </SettingsStep>
        </div>
        <dl className="mb-0 mt-10 grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-body" aria-label="Estimand and population">
          <dt className="text-faint">Estimand</dt>
          <dd className="m-0 text-ink">{preview === null ? <Metadata><span>Average treatment effect</span><span>additive scale</span><span>total effect, mediators included</span></Metadata> : describeEstimand(preview)}</dd>
          <dt className="text-faint">Population</dt>
          <dd className={num('m-0 text-ink')}>All {formatCount(prepared.observations).text} rows</dd>
          <dt className="text-faint">Graph revision</dt>
          <dd className="m-0 text-ink">{document === null ? '—' : <><Metadata><span><span className={literal()}>{document.current.id.slice(0, 8)}</span></span><span>{document.current.graph.edges.length} arrows{preview !== null && preview.graph.laggedArrows > 0 ? `, ${preview.graph.laggedArrows} lagged` : ''}</span></Metadata></>}</dd>
        </dl>
        {!readiness.ok && <Alert tone="danger" className="mt-3">{describeStudyDesignProblem(readiness.error)}</Alert>}
        <JobNotice job={job} />
        {choiceProblem !== null && <Alert tone="danger" className="mt-3">{choiceProblem}</Alert>}
        {adjustmentChoice !== null && <AdjustmentSetChoicePanel study={adjustmentChoice.study} evidence={adjustmentChoice.evidence} onChoose={(choice) => recordIdentification(adjustmentChoice.study, adjustmentChoice.evidence, choice)} />}
        <button
          type="button"
          className={button('signal', 'mt-4')}
          disabled={!readiness.ok || adjustmentChoice !== null || session.blocked || job.kind === 'running'}
          aria-busy={state.job.kind === 'running'}
          onClick={state.job.kind === 'running' ? undefined : () => void execute()}
        >
          Identify the effect
        </button>
        {job.kind === 'running' && <button type="button" className={button('quiet', 'mt-4')} onClick={session.cancel}>Cancel identification</button>}
      </section>

      {newestRecorded !== null && (
        <section aria-labelledby="study-results-title" className="grid gap-4">
          <div>
            <h2 id="study-results-title" className={cn(sectionTitle, 'm-0')}>Identified studies</h2>
          </div>
          <IdentificationCard
            key={newestRecorded.study.id}
            study={newestRecorded.study}
            identification={newestRecorded.identification}
            current
            onContinue={onContinue}
            onOpenDag={onOpenDag}
          />
        </section>
      )}
    </section>
  )

  const roles = preview === null ? null : variableRoles(preview)
  const inspector = (
    <div className="space-y-4">
      <section aria-labelledby="study-graph-title">
        <h3 id="study-graph-title" className="mb-2 mt-0 text-body font-medium text-ink">{document === null ? 'No graph chosen' : document.name}</h3>
        {evidenceGraph !== null && (
          <LagGraphViews
            graph={evidenceGraph}
            label={`${document?.name ?? 'Graph'} summary`}
            highlighted={[state.draft.treatment, state.draft.outcome].filter((node): node is DagNodeId => node !== null)}
            compact
          />
        )}
      </section>
      <section className="border-t border-hair pt-4" aria-labelledby="study-roles-title">
        <h3 id="study-roles-title" className="mb-1 mt-0 text-body font-medium text-ink">Variable roles</h3>
        {roles === null
          ? <p className="m-0 text-body text-faint">Choose a graph, a treatment, and an outcome to see which variables may be adjusted for.</p>
          : (
            <ul className="m-0 list-none divide-y divide-line border-t border-line p-0 text-body" aria-label="Variable roles">
              {roles.map(({ node, role }) => (
                <li key={node.node} className="flex flex-col py-1.5">
                  <IdentityRow name={<span className="text-ink">{node.name}</span>}><span>{roleWord(role)}</span></IdentityRow>
                  {roleDetail(role) !== null && <span className="mt-1 text-label text-faint">{roleDetail(role)}</span>}
                </li>
              ))}
            </ul>
          )}
      </section>
      <MethodCaveats
        methods={IDENTIFICATION_METHODS}
        identification={latestIdentified?.result ?? null}
        leading={(
          <RequirementsFold name="Design assumptions" literature={literatureOf(DESIGN_ASSUMPTIONS.flatMap((assumption) => assumption.sources))}>
            <ol className="m-0 list-none divide-y divide-line p-0">
              {DESIGN_ASSUMPTIONS.map((assumption) => (
                <li key={assumption.id} className="py-2 text-body">
                  <p className={prose('m-0 text-muted')}>{assumption.statement}</p>
                </li>
              ))}
            </ol>
          </RequirementsFold>
        )}
      />
    </div>
  )

  const ledger = (
    <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Study ledger">
      {studies.length === 0 && <li className="px-3 py-2 text-faint">Choose a graph, treatment and outcome, then run identification.</li>}
      {[...studies].reverse().map((study) => {
        const identification = identifications.find((candidate) => candidate.study === study.id) ?? null
        return identification === null
          ? (
            <li key={study.id} className="flex flex-wrap items-baseline justify-between gap-2 px-3 py-1.5">
              <span className="text-ink">{estimandSentence(study)}</span>
              <span className={num('text-label text-faint')}><RunMeta>{['Pending', study.dagName, formatTime(study.createdAt)]}</RunMeta></span>
            </li>
          )
          : (
            <IdentificationCard
              key={study.id}
              study={study}
              identification={identification}
              current={false}
              onContinue={onContinue}
              onOpenDag={onOpenDag}
            />
          )
      })}
    </ul>
  )

  return (
    <WorkbenchLayout
      id="study"
      stage={stage}
      inspector={{ trigger: { label: 'Requirements', icon: 'fact_check' }, title: 'Graph, roles, and method requirements', body: inspector }}
      bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Studies (${studies.length})`, body: ledger, defaultSize: 150 }}
    />
  )
}
import { RD_DESIGN_METHOD_ID } from '@/domain/methods'
