import { useMemo, useState } from 'react'
import type { RunActivity } from '@/domain/activity'
import type { DagDocument } from '@/domain/dag'
import type { DatasetProfile } from '@/domain/dataset'
import {
  actionLabel,
  actionsOutsideBounds,
  listWords,
  type DecisionEvidence,
} from '@/domain/decisionEvidence'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { DATA_CHECK_STEPS, DecisionDataCheck, EvidenceResult } from './DecisionDataCheck'
import { DecisionRecordForm, RecordedDecision } from './DecisionRecord'
import type { DecisionRecord } from '@/domain/decisionRecord'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import { RunMeta } from '@/components/ui/RunMeta'
import { formatDay, formatTime } from '@/lib/format/date'
import { cn } from '@/lib/utils'
import {
  decisionFrameSchema,
  decisionFrameTitle,
  decisionGraph,
  describeDecisionDraftProblem,
  describeDecisionGraphProblem,
  readableInputs,
  readyDecisionChecks,
  type DecisionChecksRequest,
  type DecisionFrame,
} from '@/domain/decisionFrame'
import { useJob } from '@/analysis/JobsProvider'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { useRunActivity } from '@/lib/useRunActivity'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { StudyGraphCanvas } from '@/components/dag/StudyGraphCanvas'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { Select } from '@/components/ui/Select'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { RunActions } from '@/components/ui/RunActions'
import { JobNotice } from '@/components/ui/JobNotice'
import { Alert } from '@/components/ui/Alert'
import { RunFold } from '@/components/ui/RunFold'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { MethodCaveats } from '@/components/MethodCaveats'
import { DECISION_METHODS } from '@/domain/methods'
import { Icon } from '@/components/Icon'
import {
  button,
  caption,
  chapterIntro,
  chip,
  field,
  fieldLabel,
  iconControl,
  sectionTitle,
  stepsStack,
} from '@/components/ui/recipes'

type Props = {
  readonly documents: readonly DagDocument[]
  readonly frames: readonly DecisionFrame[]
  readonly onFrame: (frame: DecisionFrame) => void
  readonly onDeleteFrame: (id: DecisionFrame['id']) => void
  readonly data: {
    readonly source: SelectedSource
    readonly profile: DatasetProfile
    readonly prepared: PreparedDatasetArtifact
  } | null
  readonly evidence: readonly DecisionEvidence[]
  readonly onEvidence: (run: DecisionEvidence) => void
  readonly onDeleteEvidence: (id: DecisionEvidence['id']) => void
  readonly records: readonly DecisionRecord[]
  readonly onRecordDecision: (record: DecisionRecord) => void
  readonly onDeleteRecord: (id: DecisionRecord['id']) => void
  readonly onActivity?: (activity: RunActivity | null) => void
}

type DraftStage = { readonly key: string; readonly node: string; readonly reads: readonly string[] }

const newStage = (): DraftStage => ({ key: crypto.randomUUID(), node: '', reads: [] })

export function DecisionWorkspace({
  documents,
  frames,
  onFrame,
  onDeleteFrame,
  data,
  evidence,
  onEvidence,
  onDeleteEvidence,
  records,
  onRecordDecision,
  onDeleteRecord,
  onActivity,
}: Props) {
  const [documentId, setDocumentId] = useState(documents.at(-1)?.id ?? '')
  const document = documents.find((candidate) => candidate.id === documentId) ?? null
  const graph = useMemo(() => (document === null ? null : decisionGraph(document)), [document])
  const [stages, setStages] = useState<readonly DraftStage[]>([newStage()])
  const [objective, setObjective] = useState('')
  const [rationale, setRationale] = useState('')
  const [problem, setProblem] = useState<string | null>(null)
  const [selected, setSelected] = useState<DecisionFrame['id'] | null>(null)
  const [inspectorView, setInspectorView] = useState<'decision' | 'method'>('decision')

  const session = useJob(`decision:${documentId}`)
  const running = session.job.kind === 'running'
  useRunActivity(onActivity, running ? { label: 'Decision checks', progress: null } : null)

  const ownFrames = frames.filter((frame) => frame.dagDocument === documentId)
  const frame =
    (selected === null ? null : ownFrames.find((candidate) => candidate.id === selected)) ??
    ownFrames.at(-1) ??
    null

  const frameEvidence = frame === null ? [] : evidence.filter((run) => run.frame === frame.id)
  const currentEvidence = frameEvidence.at(-1) ?? null
  const latestRecord =
    frame === null ? null : (records.filter((record) => record.frame === frame.id).at(-1) ?? null)
  const ledgerRuns = [
    ...ownFrames.map((item) => ({ kind: 'frame' as const, frame: item, at: item.createdAt })),
    ...evidence
      .filter((run) => ownFrames.some((item) => item.id === run.frame))
      .map((run) => ({ kind: 'evidence' as const, run, at: run.createdAt })),
    ...records
      .filter((record) => ownFrames.some((item) => item.id === record.frame))
      .map((record) => ({ kind: 'record' as const, record, at: record.createdAt })),
  ].sort((a, b) => b.at.localeCompare(a.at))

  const names = graph?.ok ? graph.value.names : []
  const hidden = new Set(graph?.ok ? graph.value.unobserved : [])
  const observed = names.flatMap((name, i) => (hidden.has(i) ? [] : [{ id: String(i), name }]))
  const chosen = new Set(stages.map((stage) => stage.node).filter((node) => node !== ''))
  const goal = objective === '' ? null : Number(objective)
  const readable = (node: string): readonly { readonly id: string; readonly name: string }[] =>
    graph?.ok && node !== ''
      ? readableInputs(graph.value, Number(node), goal).map((i) => ({
          id: String(i),
          name: names[i]!,
        }))
      : []

  function chooseDocument(id: string) {
    setDocumentId(id)
    setStages([newStage()])
    setObjective('')
    setProblem(null)
    setSelected(null)
  }

  const updateStage = (key: string, change: Partial<DraftStage>) =>
    setStages((current) =>
      current.map((stage) => (stage.key === key ? { ...stage, ...change } : stage)),
    )

  async function check() {
    if (document === null || graph === null || !graph.ok) return
    const ready = readyDecisionChecks(graph.value, {
      decisions: stages
        .filter((stage) => stage.node !== '')
        .map((stage) => {
          const allowed = new Set(readable(stage.node).map((input) => input.id))
          return {
            node: Number(stage.node),
            reads: stage.reads.filter((id) => allowed.has(id)).map(Number),
          }
        }),
      objective: objective === '' ? null : Number(objective),
    })
    if (!ready.ok) {
      setProblem(describeDecisionDraftProblem(ready.error, names))
      return
    }
    setProblem(null)
    const current = session.start('checks', 'Check decision')
    if (current === null) return
    try {
      const { runDecisionChecks } = await import('@/analysis/client')
      const answer = await runDecisionChecks(ready.value)
      if (!session.current(current)) return
      if (!answer.ok) {
        session.fail(current, describeAnalysisWorkerProblem(answer.error))
        return
      }
      const parsed = decisionFrameSchema.safeParse({
        kind: 'decision-frame',
        id: crypto.randomUUID(),
        dagDocument: document.id,
        dagRevision: document.current.id,
        createdAt: new Date().toISOString(),
        rationale: rationale.trim(),
        request: ready.value,
        result: answer.value,
      })
      if (!parsed.success) {
        session.fail(
          current,
          'The checks do not match the recorded decisions. No result was saved.',
        )
        return
      }
      onFrame(parsed.data)
      setSelected(parsed.data.id)
      session.finish(current)
    } catch (cause) {
      if (session.current(current))
        session.fail(current, cause instanceof Error ? cause.message : String(cause))
    }
  }

  const stage = (
    <section aria-labelledby="decisions-title" className="@container/panel flex flex-col gap-5">
      <div>
        <div className="mb-2 flex items-center gap-1.5">
          <ChapterHeading id="decisions-title" className="mb-0">
            Decision lab
          </ChapterHeading>
          <ParameterHelp
            label="Decision lab"
            help="Frame a decision on a causal graph, check whether recorded data identify its value, and test it against the data."
          />
        </div>
        <p className={chapterIntro}>
          A policy sets the action to take at each decision, using only what is known when the
          decision is made. In this section, you mark the decisions on a causal graph in the order
          they are made, choose what each decision may read, and choose the objective. The checks
          report whether recorded data identify the value of every such policy. You can then compare
          what the log says about each action with its natural bounds and with randomised rows.
        </p>
      </div>
      <section aria-label="Frame the decision">
        {documents.length === 0 ? (
          <p className="m-0 text-body text-faint">Create a DAG in the DAG workspace first.</p>
        ) : (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Choose the graph">
              <label className="block max-w-2xl">
                <span className={fieldLabel}>DAG</span>
                <Select
                  className={field('text', 'mt-1')}
                  value={documentId}
                  onChange={(event) => chooseDocument(event.target.value)}
                >
                  {documents.map((candidate) => (
                    <option key={candidate.id} value={candidate.id}>
                      {candidate.name}
                    </option>
                  ))}
                </Select>
              </label>
              {graph !== null && !graph.ok && (
                <Alert tone="warn" className="mt-3">
                  {describeDecisionGraphProblem(graph.error)}
                </Alert>
              )}
            </SettingsStep>
            {graph?.ok && (
              <>
                <SettingsStep
                  number={2}
                  title="Mark the decisions in order"
                  help="For each decision, choose the variables its rule may read when the decision is made."
                >
                  <ol className="m-0 flex list-none flex-col gap-4 p-0">
                    {stages.map((item, index) => (
                      <li key={item.key} className="flex flex-col gap-3">
                        <div className="flex items-end gap-2">
                          <label className="min-w-0 max-w-md flex-1">
                            <span className={fieldLabel}>Decision {index + 1}</span>
                            <Select
                              className={field('text', 'mt-1')}
                              value={item.node}
                              onChange={(event) =>
                                updateStage(item.key, { node: event.target.value, reads: [] })
                              }
                            >
                              <option value="">Choose a variable</option>
                              {observed
                                .filter(
                                  (candidate) =>
                                    candidate.id === item.node ||
                                    (!chosen.has(candidate.id) && candidate.id !== objective),
                                )
                                .map((candidate) => (
                                  <option key={candidate.id} value={candidate.id}>
                                    {candidate.name}
                                  </option>
                                ))}
                            </Select>
                          </label>
                          {stages.length > 1 && (
                            <button
                              type="button"
                              className={iconControl('danger')}
                              aria-label={`Remove decision ${index + 1}`}
                              onClick={() =>
                                setStages((current) => current.filter((s) => s.key !== item.key))
                              }
                            >
                              <Icon name="delete" />
                            </button>
                          )}
                        </div>
                        {item.node !== '' && (
                          <ColumnChecklist
                            width="max-w-md"
                            title="Variables this decision may read"
                            help="Only recorded variables settled before this decision are listed."
                            columns={readable(item.node)}
                            selected={item.reads}
                            onChange={(reads) => updateStage(item.key, { reads })}
                          />
                        )}
                      </li>
                    ))}
                  </ol>
                  <button
                    type="button"
                    className={button('quiet', 'mt-3 w-fit')}
                    onClick={() => setStages((current) => [...current, newStage()])}
                  >
                    Add a later decision
                  </button>
                </SettingsStep>
                <SettingsStep number={3} title="Choose the objective">
                  <label className="block max-w-md">
                    <span className={fieldLabel}>Objective</span>
                    <Select
                      className={field('text', 'mt-1')}
                      value={objective}
                      onChange={(event) => setObjective(event.target.value)}
                    >
                      <option value="">Choose a variable</option>
                      {observed
                        .filter((candidate) => !chosen.has(candidate.id))
                        .map((candidate) => (
                          <option key={candidate.id} value={candidate.id}>
                            {candidate.name}
                          </option>
                        ))}
                    </Select>
                  </label>
                  <label className="mt-4 block max-w-2xl">
                    <span className={fieldLabel}>Rationale</span>
                    <textarea
                      className={field('text', 'mt-1')}
                      rows={3}
                      value={rationale}
                      onChange={(event) => setRationale(event.target.value)}
                    />
                  </label>
                </SettingsStep>
                {problem !== null && <Alert tone="warn">{problem}</Alert>}
                <RunActions
                  running={running}
                  onCancel={session.cancel}
                  orbLabel="Decision checks running"
                >
                  <button
                    type="button"
                    className={button('signal')}
                    disabled={running || session.blocked}
                    onClick={() => void check()}
                  >
                    Check decision
                  </button>
                </RunActions>
                <JobNotice job={session.job} />
                {frame !== null && document !== null && data !== null && (
                  <DecisionDataCheck
                    key={frame.id}
                    frame={frame}
                    document={document}
                    source={data.source}
                    profile={data.profile}
                    prepared={data.prepared}
                    firstStep={4}
                    onRecord={onEvidence}
                  />
                )}
              </>
            )}
          </div>
        )}
      </section>
      {frame !== null && (
        <section aria-labelledby="decision-results-title" className="grid grid-cols-1 gap-4">
          <h2 id="decision-results-title" className={cn(sectionTitle, 'm-0')}>
            Results
          </h2>
          {currentEvidence === null ? (
            <p className="m-0 text-body text-muted">
              Run natural bounds to compare what the log says about each action with randomised
              rows.
            </p>
          ) : (
            <EvidenceResult run={currentEvidence} current />
          )}
        </section>
      )}
      {frame !== null && currentEvidence !== null && (
        <section aria-labelledby="decision-decide-title" className="flex flex-col gap-4">
          <h2 id="decision-decide-title" className={cn(sectionTitle, 'm-0')}>
            Decide and record
          </h2>
          <DecisionRecordForm
            key={currentEvidence.id}
            run={currentEvidence}
            frame={frame}
            firstStep={4 + DATA_CHECK_STEPS}
            onRecord={onRecordDecision}
          />
        </section>
      )}
    </section>
  )

  const inspector = (
    <div className="flex flex-col gap-4">
      <h3 id="decision-inspector-title" className="m-0 mt-1 text-body font-medium text-ink">
        {frame === null ? 'No decision framed' : decisionFrameTitle(frame)}
      </h3>
      <SegmentedControl
        size="sm"
        fill
        ariaLabel="Decision inspector"
        value={inspectorView}
        onChange={setInspectorView}
        options={[
          {
            value: 'decision',
            label: (
              <span className="flex items-center gap-1.5">
                <Icon name="route" size={14} />
                Decision
              </span>
            ),
          },
          {
            value: 'method',
            label: (
              <span className="flex items-center gap-1.5">
                <Icon name="function" size={14} />
                Method
              </span>
            ),
          },
        ]}
      />
      {inspectorView === 'decision' && (
        <section aria-labelledby="decision-inspector-title" className="flex flex-col gap-4">
          {frame !== null && <DecisionChecks frame={frame} evidence={currentEvidence} />}
          {latestRecord !== null && (
            <section aria-label="Recorded decision" className="flex flex-col gap-3">
              <h3 className="m-0 text-body font-medium text-ink">Recorded decision</h3>
              <RecordedDecision record={latestRecord} />
            </section>
          )}
          {frame !== null && <FrameRecord frame={frame} />}
          {document !== null && (
            <StudyGraphCanvas document={document} treatment={null} outcome={null} />
          )}
        </section>
      )}
      {inspectorView === 'method' && <MethodCaveats methods={DECISION_METHODS} />}
    </div>
  )

  return (
    <WorkbenchLayout
      id="decisions"
      stage={stage}
      inspector={{
        trigger: { label: 'Requirements', icon: 'contract' },
        title: 'Decision and method requirements',
        body: inspector,
      }}
      bottom={{
        trigger: { label: 'History', icon: 'history' },
        title: `Runs (${ledgerRuns.length})`,
        defaultSize: 150,
        body: (
          <ul
            className="m-0 list-none divide-y divide-hair p-0 text-body"
            aria-label="Decision ledger"
          >
            {ledgerRuns.length === 0 && (
              <li className="px-3 py-2 text-faint">Check a decision to record it here.</li>
            )}
            {ledgerRuns.map((entry) =>
              entry.kind === 'frame' ? (
                <RunFold
                  key={entry.frame.id}
                  title={decisionFrameTitle(entry.frame)}
                  figure={entry.frame.result.admissible ? 'Identified' : 'Not identified'}
                  stamp={
                    <RunMeta>
                      {['Sequential backdoor check', formatTime(entry.frame.createdAt)]}
                    </RunMeta>
                  }
                  actions={
                    entry.frame.id === frame?.id ? undefined : (
                      <button
                        type="button"
                        className={iconControl()}
                        aria-label={`Show ${decisionFrameTitle(entry.frame)} on the stage`}
                        title="Show on the stage"
                        onClick={(event) => {
                          event.preventDefault()
                          setSelected(entry.frame.id)
                        }}
                      >
                        <Icon name="visibility" size={15} />
                      </button>
                    )
                  }
                  onDelete={() => onDeleteFrame(entry.frame.id)}
                  deleteLabel={`Delete decision: ${decisionFrameTitle(entry.frame)}, ${formatTime(entry.frame.createdAt)}`}
                >
                  <div className="px-3 pb-3">
                    <FrameRecord frame={entry.frame} />
                  </div>
                </RunFold>
              ) : entry.kind === 'evidence' ? (
                <EvidenceResult
                  key={entry.run.id}
                  run={entry.run}
                  current={false}
                  onDelete={() => onDeleteEvidence(entry.run.id)}
                />
              ) : (
                <RunFold
                  key={entry.record.id}
                  title={entry.record.plan}
                  figure={`Review ${formatDay(`${entry.record.reviewOn}T00:00`)}`}
                  stamp={
                    <RunMeta>{['Recorded decision', formatTime(entry.record.createdAt)]}</RunMeta>
                  }
                  onDelete={() => onDeleteRecord(entry.record.id)}
                  deleteLabel={`Delete recorded decision: ${entry.record.plan}, ${formatTime(entry.record.createdAt)}`}
                >
                  <div className="px-3 pb-3">
                    <RecordedDecision record={entry.record} />
                  </div>
                </RunFold>
              ),
            )}
          </ul>
        ),
      }}
    />
  )
}

/** The framed decision as a record: its roles, the backdoor check by decision, and the sets worth intervening on. */
function FrameRecord({ frame }: { readonly frame: DecisionFrame }) {
  const { request, result } = frame
  const names = request.names
  return (
    <section aria-label="Framed decision" className="flex flex-col gap-3">
      <h3 className="m-0 text-body font-medium text-ink">Framed decision</h3>
      <RecordList>
        {request.decisions.map((stage, index) => {
          const check = result.stages[index]
          return (
            <RecordRow key={stage.node} term={`Decision ${index + 1}`}>
              <span className={chip()}>{names[stage.node]}</span>
              {stage.reads.length === 0 ? (
                <p className={caption('mb-0 mt-1')}>Reads no variables</p>
              ) : (
                <>
                  <p className={caption('mb-1 mt-1')}>Reads</p>
                  <NameChips names={stage.reads.map((i) => names[i]!)} />
                </>
              )}
              {check !== undefined && (
                <p
                  className={cn(
                    'mb-0 mt-1 flex items-center gap-1.5 text-body',
                    check.blocked ? 'text-ok' : 'text-warn',
                  )}
                >
                  <Icon name={check.blocked ? 'check_circle' : 'warning'} size={15} />
                  {check.blocked ? 'Back-door paths blocked' : 'Back-door path open'}
                </p>
              )}
            </RecordRow>
          )
        })}
        <RecordRow term="Objective">
          <span className={chip()}>{names[request.objective]}</span>
        </RecordRow>
        {request.unobserved.length > 0 && (
          <RecordRow term="Unobserved">
            <NameChips names={request.unobserved.map((i) => names[i]!)} />
          </RecordRow>
        )}
        <RecordRow term="Worth intervening on">
          <span className="flex flex-wrap items-center gap-1">
            {result.possiblyOptimal.map((set) => (
              <span key={set.join(',')} className={chip()}>
                {set.length === 0 ? 'No intervention' : listNames(set, names)}
              </span>
            ))}
            <ParameterHelp
              label="Worth intervening on"
              help="Each set is optimal in at least one model compatible with the graph. A set that is not listed is never better than one of these."
            />
          </span>
        </RecordRow>
        {frame.rationale !== '' && (
          <RecordRow term="Rationale">
            <p className="m-0 text-body text-muted">{frame.rationale}</p>
          </RecordRow>
        )}
      </RecordList>
      <p className={caption('m-0')}>
        <RunMeta>{['Sequential backdoor check', formatTime(frame.createdAt)]}</RunMeta>
      </p>
    </section>
  )
}

/** The checks the results rest on, as the DAG workspace shows its live validation: a status line per check, then what it means. */
function DecisionChecks({
  frame,
  evidence,
}: {
  readonly frame: DecisionFrame
  readonly evidence: DecisionEvidence | null
}) {
  const names = frame.request.names
  const objective = names[frame.request.objective]
  const open = frame.result.stages.filter((stage) => !stage.blocked)
  const tested =
    evidence !== null && evidence.result.actions.some((action) => action.randomised !== null)
  const outside = evidence === null ? [] : actionsOutsideBounds(evidence.result)
  const checks = [
    open.length === 0
      ? {
          ok: true,
          heading: 'Policies identified',
          detail: 'Recorded data identify the value of every policy in this space.',
        }
      : {
          ok: false,
          heading: 'Policies not identified',
          detail: `A back-door path from ${listWords(open.map((stage) => names[stage.decision]!))} to ${objective} stays open, so only bounds are available without an experiment.`,
        },
    ...(frame.result.imitation === undefined
      ? []
      : frame.result.imitation === null
        ? [
            {
              ok: false,
              heading: 'Copying is not safe',
              detail: `No set of recorded variables lets a copier keep the demonstrator’s value: the demonstrator acts on something the log does not record that also affects ${objective}.`,
            },
          ]
        : [
            {
              ok: true,
              heading: 'Copying is safe',
              detail: `A copier keeps the demonstrator’s value if ${listWords(
                frame.request.decisions.map((stage, i) => {
                  const reads = frame.result.imitation![i] ?? []
                  return `${names[stage.node]} reads ${reads.length === 0 ? 'nothing' : listWords(reads.map((r) => names[r]!))}`
                }),
              )}.`,
            },
          ]),
    ...(!tested
      ? []
      : outside.length === 0
        ? [
            {
              ok: true,
              heading: 'Bounds hold',
              detail: 'Every randomised estimate lies inside its natural bounds.',
            },
          ]
        : [
            {
              ok: false,
              heading: 'Bounds fail',
              detail: `The randomised estimate for ${evidence!.action.name} ${listWords(outside.map((action) => actionLabel(action.value)))} lies outside its natural bounds, so the randomised rows come from a different population or measure the outcome differently, unless sampling error explains it.`,
            },
          ]),
  ]
  return (
    <aside aria-labelledby="decision-checks-title">
      <h3 id="decision-checks-title" className="mb-1 mt-0 text-body font-medium text-ink">
        Checks
      </h3>
      <ul className="m-0 list-none space-y-3 p-0">
        {checks.map((check) => (
          <li key={check.heading}>
            <p
              className={cn(
                'm-0 flex items-center gap-1.5 text-body',
                check.ok ? 'text-ok' : 'text-warn',
              )}
            >
              <Icon name={check.ok ? 'check_circle' : 'error'} size={16} />
              {check.heading}
            </p>
            <p className="mb-0 mt-1 text-body text-muted">{check.detail}</p>
          </li>
        ))}
      </ul>
    </aside>
  )
}

const listNames = (indices: readonly number[], names: readonly string[]): string =>
  indices.length === 0 ? 'Nothing' : indices.map((i) => names[i]).join(', ')

function NameChips({ names }: { readonly names: readonly string[] }) {
  return (
    <span className="flex flex-wrap gap-1">
      {names.map((name) => (
        <span key={name} className={chip()}>
          {name}
        </span>
      ))}
    </span>
  )
}
