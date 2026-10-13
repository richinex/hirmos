import { NumberInput } from '@/components/ui/NumberInput'
import { z } from 'zod'
import { err, ok } from '@/domain/dop'
import { useWorkspaceConfiguration } from '@/assistant/workspaceConfiguration'
import { useRecordedAnalysis } from '@/assistant/useRecordedAnalysis'
import {
  swigSettingsSchema,
  prepareSwigSettings,
  swigSettingsDescription,
} from '@/domain/swigSettings'
import { useMemo, useState } from 'react'
import type { DagDocument } from '@/domain/dag'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import {
  readySwigGraph,
  describeSwigGraphProblem,
  swigAnalysisSchema,
  swigNodeName,
  swigDifferences,
  describeSeparation,
  distinctSeparationChecks,
  separationKey,
  separationNotation,
  groupSwigAnalyses,
  swigGraphTitle,
  type SwigAnalysis,
  type SwigSpecification,
} from '@/domain/swig'
import { useJob } from '@/analysis/JobsProvider'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { initialSwigDidDraft } from '@/domain/swigDidDraft'
import { SwigDidForm } from './SwigDidForm'
import { SwigDidSpecification } from './SwigDidSpecification'
import { SwigDidResult } from './SwigDidResult'
import { swigProjectionSchema, type SwigProjection } from '@/domain/swigProjection'
import { SwigProjectionForm } from './SwigProjectionForm'
import { SwigCanvas } from './SwigCanvas'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { RunActions } from '@/components/ui/RunActions'
import { JobNotice } from '@/components/ui/JobNotice'
import { Alert } from '@/components/ui/Alert'
import { RunPicker } from '@/components/ui/RunPicker'
import { Icon } from '@/components/Icon'
import { button, field, fieldHint, fieldLabel, iconControl } from '@/components/ui/recipes'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'

type Term = {
  readonly key: string
  readonly identity: string
  readonly arguments: readonly string[]
}
type Equation = { readonly outcome: string; readonly terms: readonly Term[] }
type Construction =
  | { readonly kind: 'swig' }
  | { readonly kind: 'difference'; readonly earlier: Equation; readonly later: Equation }
const emptyEquation = (): Equation => ({ outcome: '', terms: [] })
const listClass = 'panel-scroll max-h-64 overflow-y-auto p-1'
const separationSettingsSchema = z
  .object({ left: z.string(), right: z.string(), given: z.array(z.string()) })
  .strict()
type Props = {
  readonly document: DagDocument
  readonly prepared: PreparedDatasetArtifact
  readonly records: readonly SwigAnalysis[]
  readonly onRecord: (record: SwigAnalysis) => void
  /** Removes one record directly, as when a repeated query replaces it. */
  readonly onDelete: (id: SwigAnalysis['id']) => void
  /** Asks to delete a saved graph and its checks through the shared deletion dialog. */
  readonly onDeleteGraph: (id: SwigAnalysis['id']) => void
  readonly onBack: () => void
}

function EquationEditor({
  title,
  equation,
  names,
  edges,
  intervened,
  onChange,
}: {
  readonly title: string
  readonly equation: Equation
  readonly names: readonly string[]
  readonly edges: readonly (readonly [number, number])[]
  readonly intervened: readonly string[]
  readonly onChange: (value: Equation) => void
}) {
  const parents = edges
    .filter(
      ([cause, effect]) =>
        String(effect) === equation.outcome && !intervened.includes(String(cause)),
    )
    .map(([cause]) => ({ id: String(cause), name: names[cause]! }))
  return (
    <fieldset className="min-w-0 space-y-3 border-0 p-0">
      <legend className={fieldLabel}>{title}</legend>
      <Select
        aria-label={title}
        className={field('text', 'mt-1')}
        value={equation.outcome}
        onChange={(event) => onChange({ outcome: event.target.value, terms: [] })}
      >
        <option value="">Choose an explicit period node</option>
        {names.map((name, index) => (
          <option key={index} value={index}>
            {name}
          </option>
        ))}
      </Select>
      {equation.terms.map((term, index) => (
        <div key={term.key} className="space-y-2 rounded border border-hair p-2">
          <div className="flex items-center gap-2">
            <label className="min-w-0 flex-1">
              <span className={fieldLabel}>Term name</span>
              <input
                aria-label={`${title} term ${index + 1}`}
                className={field('text', 'mt-1')}
                value={term.identity}
                onChange={(event) =>
                  onChange({
                    ...equation,
                    terms: equation.terms.map((t) =>
                      t.key === term.key ? { ...t, identity: event.target.value } : t,
                    ),
                  })
                }
              />
            </label>
            <button
              type="button"
              className={iconControl()}
              aria-label={`Remove ${title.toLowerCase()} term ${index + 1}`}
              onClick={() =>
                onChange({ ...equation, terms: equation.terms.filter((t) => t.key !== term.key) })
              }
            >
              <Icon name="close" size={16} />
            </button>
          </div>
          <div className={listClass}>
            <ColumnChecklist
              title="Arguments"
              help="Choose the random parents used by this function. An empty selection represents a constant."
              columns={parents}
              selected={term.arguments}
              onChange={(arguments_) =>
                onChange({
                  ...equation,
                  terms: equation.terms.map((t) =>
                    t.key === term.key ? { ...t, arguments: arguments_ } : t,
                  ),
                })
              }
            />
          </div>
        </div>
      ))}
      <button
        type="button"
        className={button('quiet')}
        disabled={equation.outcome === '' || equation.terms.length >= 256}
        onClick={() =>
          onChange({
            ...equation,
            terms: [...equation.terms, { key: crypto.randomUUID(), identity: '', arguments: [] }],
          })
        }
      >
        Add additive term
      </button>
    </fieldset>
  )
}

export function SwigWorkspace({
  document,
  prepared,
  records,
  onRecord,
  onDelete,
  onDeleteGraph,
  onBack,
}: Props) {
  const [projection, setProjection] = useState<SwigProjection>({ kind: 'explicit' })
  const [historyConfirmed, setHistoryConfirmed] = useState(false)
  const [purpose, setPurpose] = useState<'graph' | 'did'>('graph')
  const [pane, setPane] = useState<'new' | 'saved'>('saved')
  const [additionalPairs, setAdditionalPairs] = useState<
    readonly { key: string; earlier: Equation; later: Equation }[]
  >([])
  const graph = readySwigGraph(document, projection)
  const [didDraft, setDidDraft] = useState(() =>
    initialSwigDidDraft(graph.ok ? graph.value.names.length : 0),
  )
  const boundaryReady = projection.kind === 'explicit' || historyConfirmed
  const session = useJob(`swig:${document.current.id}`)
  const [interventions, setInterventions] = useState<
    readonly { readonly key: string; readonly variable: string; readonly value: string }[]
  >([{ key: 'initial', variable: '', value: '' }])
  const [exogenous, setExogenous] = useState<readonly string[]>([])
  const [construction, setConstruction] = useState<Construction>({ kind: 'swig' })
  const [rationale, setRationale] = useState('')
  const [selected, setSelected] = useState<string | null>(null)
  const [problem, setProblem] = useState<string | null>(null)
  const [left, setLeft] = useState(''),
    [right, setRight] = useState('')
  const [given, setGiven] = useState<readonly string[]>([])
  // Grouping serialises each specification, so it runs when the records change, not on every keystroke.
  const entries = useMemo(
    () =>
      groupSwigAnalyses(
        records.filter(
          (record) => record.dagDocument === document.id && record.preparedDataset === prepared.id,
        ),
      ),
    [records, document.id, prepared.id],
  )
  const entry =
    selected === null
      ? (entries.at(-1) ?? null)
      : (entries.find((item) => item.recordIds.some((id) => id === selected)) ?? null)
  const record = entry?.graph ?? null
  const running = session.job.kind === 'running'
  const locked = running || session.blocked
  const show = (id: string) => {
    setSelected(id)
    setLeft('')
    setRight('')
    setGiven([])
  }

  const settings =
    purpose === 'did'
      ? { purpose, draft: didDraft }
      : { purpose, interventions, exogenous, construction, additionalPairs, rationale }
  const ready = graph.ok
    ? prepareSwigSettings(settings, graph.value)
    : err(describeSwigGraphProblem(graph.error))
  const binding = JSON.stringify({
    document: document.id,
    revision: document.current.id,
    prepared: prepared.id,
    projection,
    historyConfirmed,
  })
  useWorkspaceConfiguration('swig-projection', binding, {
    title: 'Intervention graph time expansion',
    current: projection,
    schema: swigProjectionSchema,
    columns: document.current.graph.nodes.map((n) => ({ id: n.id, name: n.name })),
    describe: (value) => [
      {
        key: 'kind',
        label: 'Graph',
        value:
          value.kind === 'explicit'
            ? 'Explicit period nodes'
            : `Periods ${value.start} to ${value.end}`,
      },
      ...(value.kind === 'temporal'
        ? [
            {
              key: 'invariant',
              label: 'Time-invariant variables',
              value:
                value.invariant
                  .map((id) => document.current.graph.nodes.find((n) => n.id === id)?.name ?? id)
                  .join(', ') || 'None',
            },
            {
              key: 'boundary',
              label: 'History boundary',
              value: 'Confirm the closed-history assumption on the page before running.',
            },
          ]
        : []),
    ],
    validate: (value) => {
      if (locked) return err('Wait for the current analysis to finish.')
      const next = readySwigGraph(document, value)
      return next.ok ? ok(value) : err(describeSwigGraphProblem(next.error))
    },
    configure: (value) => {
      const next = readySwigGraph(document, value)
      setProjection(value)
      setHistoryConfirmed(false)
      setDidDraft(initialSwigDidDraft(next.ok ? next.value.names.length : 0))
      setInterventions([{ key: crypto.randomUUID(), variable: '', value: '' }])
      setExogenous([])
      setConstruction({ kind: 'swig' })
      setAdditionalPairs([])
      setPane('new')
    },
  })
  useWorkspaceConfiguration<unknown>(
    'swig-settings',
    binding,
    graph.ok
      ? {
          title: 'Intervention graph',
          current: settings,
          schema: swigSettingsSchema,
          columns: graph.value.names.map((name, i) => ({ id: String(i), name })),
          describe: (value) => {
            const parsed = swigSettingsSchema.safeParse(value)
            return parsed.success
              ? swigSettingsDescription(parsed.data, graph.value.names)
              : [
                  {
                    key: 'draft',
                    label: 'Draft',
                    value: 'Complete the intervention graph settings.',
                  },
                ]
          },
          validate: (value) => {
            if (locked) return err('Wait for the current analysis to finish.')
            const next = prepareSwigSettings(value, graph.value)
            return next.ok ? ok(value) : next
          },
          configure: (value) => {
            const next = swigSettingsSchema.parse(value)
            setPurpose(next.purpose)
            setPane('new')
            if (next.purpose === 'did') setDidDraft(next.draft)
            else {
              setInterventions(next.interventions)
              setExogenous(next.exogenous)
              setConstruction(next.construction)
              setAdditionalPairs(next.additionalPairs)
              setRationale(next.rationale)
            }
          },
        }
      : null,
  )
  useRecordedAnalysis({
    key: 'swig',
    prepared: prepared.id,
    binding: JSON.stringify({ binding, settings }),
    session,
    inspection: {
      section: 'DAG workspace → Intervention graphs',
      method: purpose === 'did' ? 'DiD adjustment assessment' : 'Intervention graph',
      specification: JSON.stringify({
        settings,
        projection,
        names: graph.ok ? graph.value.names : [],
      }),
      sharedResult:
        'Saved graph and graphical conclusions. This is not an estimated treatment effect.',
    },
    unavailable: !boundaryReady
      ? 'Confirm the closed-history assumption in Intervention graphs before running.'
      : ready.ok
        ? null
        : ready.error,
    execute: async () => {
      if (ready.ok && graph.ok && boundaryReady)
        await run(
          ready.value,
          graph.value.names,
          purpose === 'did' ? didDraft.rationale.trim() : rationale.trim(),
        )
    },
    records: (workflow) => workflow.swigAnalyses,
    summarise: (value) =>
      JSON.stringify({ id: value.id, names: value.names, conclusion: value.result.conclusion }),
  })
  const separationProblem = (value: {
    readonly left: string
    readonly right: string
    readonly given: readonly string[]
  }) => {
    if (!record || record.result.conclusion.kind === 'did')
      return 'Choose a saved intervention graph for a separation check.'
    if (record.dagRevision !== document.current.id)
      return 'The saved graph belongs to an earlier revision. Construct a graph from the current revision first.'
    const ids = [value.left, value.right, ...value.given]
    if (
      new Set(ids).size !== ids.length ||
      ids.some(
        (id) =>
          !/^(0|[1-9]\d*)$/.test(id) ||
          !record.result.nodes[Number(id)] ||
          record.result.nodes[Number(id)]?.kind === 'fixed',
      )
    )
      return 'Choose two distinct random variables and separate conditioning variables from the saved graph.'
    return null
  }
  useWorkspaceConfiguration(
    'swig-separation',
    JSON.stringify({ binding, record: record?.id }),
    record && record.result.conclusion.kind !== 'did'
      ? {
          title: 'Separation check',
          current: { left, right, given },
          schema: separationSettingsSchema,
          columns: record.result.nodes.flatMap((n, i) =>
            n.kind === 'fixed' ? [] : [{ id: String(i), name: swigNodeName(record, i) }],
          ),
          describe: (value) => [
            {
              key: 'left',
              label: 'First variable',
              value: value.left === '' ? 'Not selected' : swigNodeName(record, Number(value.left)),
            },
            {
              key: 'right',
              label: 'Second variable',
              value:
                value.right === '' ? 'Not selected' : swigNodeName(record, Number(value.right)),
            },
            {
              key: 'given',
              label: 'Conditioning variables',
              value: value.given.map((id) => swigNodeName(record, Number(id))).join(', ') || 'None',
            },
          ],
          validate: (value) => {
            const problem = separationProblem(value)
            return locked
              ? err('Wait for the current analysis to finish.')
              : problem
                ? err(problem)
                : ok(value)
          },
          configure: (value) => {
            setLeft(value.left)
            setRight(value.right)
            setGiven(value.given)
            setPane('saved')
          },
        }
      : null,
  )
  useRecordedAnalysis({
    key: 'swig-separation',
    prepared: prepared.id,
    binding: JSON.stringify({ binding, record: record?.id, left, right, given }),
    session,
    inspection: {
      section: 'DAG workspace → Intervention graphs',
      method: 'Separation check',
      specification: JSON.stringify({
        record: record?.id,
        left,
        right,
        given,
        names: record?.names,
      }),
      sharedResult:
        'Graphical separation, conditional on the recorded graph and assumptions; not a test against data.',
    },
    unavailable: separationProblem({ left, right, given }),
    execute: async () => {
      if (record && !separationProblem({ left, right, given }))
        await run(
          {
            ...record.specification,
            query: {
              kind: 'separation',
              left: Number(left),
              right: Number(right),
              given: given.map(Number),
            },
          },
          record.names,
          record.rationale,
          record.projection,
        )
    },
    records: (workflow) => workflow.swigAnalyses,
    summarise: (value) => JSON.stringify({ id: value.id, conclusion: value.result.conclusion }),
  })

  async function run(
    specification: SwigSpecification,
    names: readonly string[],
    assumptions: string,
    recordedProjection: SwigProjection = projection,
  ) {
    const current = session.start(
      'analysis',
      specification.query.kind === 'graph' ? 'Construct intervention graph' : 'Check separation',
    )
    if (current === null) return
    setProblem(null)
    try {
      const { runSwigAnalysis } = await import('@/analysis/client')
      const answer = await runSwigAnalysis(specification, names)
      if (!session.current(current)) return
      if (!answer.ok) {
        session.fail(current, describeAnalysisWorkerProblem(answer.error))
        return
      }
      const parsed = swigAnalysisSchema.safeParse({
        kind: 'swig-analysis',
        id: crypto.randomUUID(),
        dagDocument: document.id,
        dagRevision: document.current.id,
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        names,
        projection: recordedProjection,
        rationale: assumptions,
        specification,
        result: answer.value,
      })
      if (!parsed.success) {
        session.fail(
          current,
          'The derived graph does not match the recorded specification. No result was saved.',
        )
        return
      }
      // Separation is deterministic: a repeated query replaces the earlier record rather than adding one.
      const key = separationKey(parsed.data.specification.query)
      if (key !== null)
        for (const check of entry?.checks ?? [])
          if (separationKey(check.specification.query) === key) onDelete(check.id)
      onRecord(parsed.data)
      // A separation check joins the graph it ran on, so the query stays in place for the next check.
      if (specification.query.kind === 'graph') show(parsed.data.id)
      setPane('saved')
      session.finish(current)
    } catch (error) {
      if (session.current(current))
        session.fail(current, error instanceof Error ? error.message : String(error))
    }
  }
  function build() {
    if (!graph.ok || !boundaryReady) return
    if (!ready.ok) {
      setProblem(ready.error)
      return
    }
    void run(ready.value, graph.value.names, rationale.trim())
  }
  const newAnalysis = (
    <div className="space-y-4">
      <SwigProjectionForm
        document={document}
        value={projection}
        confirmed={historyConfirmed}
        onConfirm={setHistoryConfirmed}
        disabled={locked}
        boundaryArrows={graph.ok ? graph.value.boundaryArrows : null}
        onChange={(value) => {
          const next = readySwigGraph(document, value)
          setDidDraft(initialSwigDidDraft(next.ok ? next.value.names.length : 0))
          setProjection(value)
          setHistoryConfirmed(false)
          setInterventions([{ key: crypto.randomUUID(), variable: '', value: '' }])
          setExogenous([])
          setConstruction({ kind: 'swig' })
          setAdditionalPairs([])
        }}
      />
      <SegmentedControl
        size="sm"
        fill
        ariaLabel="Intervention graph analysis"
        value={purpose}
        onChange={setPurpose}
        options={[
          { value: 'graph', label: 'Graph', disabled: locked },
          { value: 'did', label: 'DiD adjustment', disabled: locked },
        ]}
      />
      {!graph.ok ? (
        <Alert tone="warn">{describeSwigGraphProblem(graph.error)}</Alert>
      ) : purpose === 'did' ? (
        <SwigDidForm
          key={JSON.stringify(projection)}
          graph={graph.value}
          draft={didDraft}
          onChange={setDidDraft}
          blocked={locked || !boundaryReady}
          running={running}
          onCancel={session.cancel}
          onRun={(spec, reason) => void run(spec, graph.value.names, reason)}
        />
      ) : (
        <fieldset disabled={locked} className="min-w-0 space-y-4 border-0 p-0">
          <SegmentedControl
            size="sm"
            fill
            ariaLabel="Graph construction"
            value={construction.kind}
            onChange={(kind) =>
              setConstruction(
                kind === 'swig'
                  ? { kind }
                  : { kind, earlier: emptyEquation(), later: emptyEquation() },
              )
            }
            options={[
              { value: 'swig', label: 'SWIG' },
              { value: 'difference', label: 'Δ-SWIG' },
            ]}
          />
          {interventions.map((row, index) => (
            <div key={row.key} className="grid min-w-0 grid-cols-[1fr_5rem_auto] items-end gap-2">
              <label className="min-w-0">
                <span className={fieldLabel}>Set variable</span>
                <Select
                  aria-label={`Intervention variable ${index + 1}`}
                  value={row.variable}
                  className={field('text', 'mt-1')}
                  onChange={(event) =>
                    setInterventions((rows) =>
                      rows.map((r) =>
                        r.key === row.key ? { ...r, variable: event.target.value } : r,
                      ),
                    )
                  }
                >
                  <option value="">Choose</option>
                  {graph.value.names.map((name, i) => (
                    <option key={i} value={i} disabled={exogenous.includes(String(i))}>
                      {name}
                    </option>
                  ))}
                </Select>
              </label>
              <label>
                <span className={fieldLabel}>Value</span>
                <NumberInput
                  aria-label={`Intervention value ${index + 1}`}
                  className={field('text', 'mt-1')}
                  step="any"
                  value={row.value}
                  onChange={(event) =>
                    setInterventions((rows) =>
                      rows.map((r) =>
                        r.key === row.key ? { ...r, value: event.target.value } : r,
                      ),
                    )
                  }
                />
              </label>
              <button
                type="button"
                aria-label={`Remove intervention ${index + 1}`}
                disabled={interventions.length === 1}
                className={iconControl()}
                onClick={() => setInterventions((rows) => rows.filter((r) => r.key !== row.key))}
              >
                <Icon name="close" size={16} />
              </button>
            </div>
          ))}
          <button
            type="button"
            className={button('quiet')}
            disabled={interventions.length >= graph.value.names.length}
            onClick={() =>
              setInterventions((rows) => [
                ...rows,
                { key: crypto.randomUUID(), variable: '', value: '' },
              ])
            }
          >
            Add intervention
          </button>
          <details>
            <DisclosureSummary className="cursor-pointer text-body text-ink">
              Exogenous variables
            </DisclosureSummary>
            <div className={listClass}>
              <ColumnChecklist
                title="Exogenous variables"
                help="Declare root variables determined outside the model. Unmeasured does not necessarily mean exogenous. Dependence between disturbances must be represented in the source graph."
                columns={graph.value.names
                  .map((name, i) => ({ id: String(i), name }))
                  .filter((n) => !graph.value.edges.some(([, effect]) => String(effect) === n.id))}
                selected={exogenous}
                reserved={interventions.map((row) => row.variable)}
                onChange={setExogenous}
              />
            </div>
          </details>
          {construction.kind === 'difference' && (
            <>
              <p className={fieldHint}>
                Use separate source nodes for the earlier and later outcomes, including their
                disturbances. Each equation is a sum of named functions. Reusing a term name asserts
                the same function, coefficient and ordered arguments in both equations. Only those
                terms cancel; sharing a parent is not sufficient.
              </p>
              <p className={fieldHint}>
                Arguments must cover all random parents of each outcome. Fixed intervention values
                are held constant within this world. Record any additional restrictions in the
                rationale.
              </p>
              <EquationEditor
                title="Earlier outcome"
                equation={construction.earlier}
                names={graph.value.names}
                edges={graph.value.edges}
                intervened={interventions.map((r) => r.variable)}
                onChange={(earlier) =>
                  setConstruction((current) =>
                    current.kind === 'difference' ? { ...current, earlier } : current,
                  )
                }
              />
              <EquationEditor
                title="Later outcome"
                equation={construction.later}
                names={graph.value.names}
                edges={graph.value.edges}
                intervened={interventions.map((r) => r.variable)}
                onChange={(later) =>
                  setConstruction((current) =>
                    current.kind === 'difference' ? { ...current, later } : current,
                  )
                }
              />
              {additionalPairs.map((pair, index) => (
                <div key={pair.key} className="space-y-3 pt-3">
                  <EquationEditor
                    title={`Earlier outcome ${index + 2}`}
                    equation={pair.earlier}
                    names={graph.value.names}
                    edges={graph.value.edges}
                    intervened={interventions.map((r) => r.variable)}
                    onChange={(earlier) =>
                      setAdditionalPairs((pairs) =>
                        pairs.map((p) => (p.key === pair.key ? { ...p, earlier } : p)),
                      )
                    }
                  />
                  <EquationEditor
                    title={`Later outcome ${index + 2}`}
                    equation={pair.later}
                    names={graph.value.names}
                    edges={graph.value.edges}
                    intervened={interventions.map((r) => r.variable)}
                    onChange={(later) =>
                      setAdditionalPairs((pairs) =>
                        pairs.map((p) => (p.key === pair.key ? { ...p, later } : p)),
                      )
                    }
                  />
                  <button
                    type="button"
                    className={button('quiet')}
                    onClick={() =>
                      setAdditionalPairs((pairs) => pairs.filter((p) => p.key !== pair.key))
                    }
                  >
                    Remove difference {index + 2}
                  </button>
                </div>
              ))}
              <button
                type="button"
                className={button('quiet')}
                disabled={additionalPairs.length >= 31}
                onClick={() =>
                  setAdditionalPairs((pairs) => [
                    ...pairs,
                    { key: crypto.randomUUID(), earlier: emptyEquation(), later: emptyEquation() },
                  ])
                }
              >
                Add difference
              </button>
            </>
          )}
          <label className="block">
            <span className={fieldLabel}>Assumptions and rationale</span>
            <textarea
              aria-label="SWIG assumptions and rationale"
              className={field('text', 'mt-1')}
              rows={4}
              value={rationale}
              onChange={(event) => setRationale(event.target.value)}
            />
          </label>
        </fieldset>
      )}
      {problem !== null && <Alert tone="warn">{problem}</Alert>}
      {purpose === 'graph' && (
        <RunActions
          running={
            session.job.kind === 'running' && session.job.stage === 'Construct intervention graph'
          }
          onCancel={session.cancel}
          orbLabel="Constructing intervention graph"
        >
          <button
            type="button"
            className={button('signal')}
            disabled={locked || !graph.ok || !boundaryReady}
            onClick={build}
          >
            Construct graph
          </button>
        </RunActions>
      )}
      <JobNotice job={session.job} />
    </div>
  )

  const checks = entry === null ? [] : distinctSeparationChecks(entry.checks)
  const latest = checks[0] ?? null
  const savedGraph =
    record === null ? null : (
      <div className="flex flex-col gap-6">
        {record.result.nodes.some((node) => node.kind === 'difference') && (
          <div className="space-y-1 text-body text-ink">
            {record.result.nodes.flatMap((node, index) =>
              node.kind === 'difference' ? (
                <p key={index} className="m-0">
                  Cancelled additive terms:{' '}
                  {node.cancelled.length === 0 ? 'none' : node.cancelled.join(', ')}.
                </p>
              ) : (
                []
              ),
            )}
          </div>
        )}
        {record.result.conclusion.kind === 'did' ? (
          <SwigDidResult assessment={record.result.conclusion.assessment} names={record.names} />
        ) : (
          <section className="space-y-3" aria-labelledby="swig-separation-title">
            <div>
              <h3 id="swig-separation-title" className="m-0 text-body font-medium text-ink">
                Separation checks
              </h3>
              <p className="mb-0 mt-1 text-label text-faint">
                Test whether two nodes of this graph are d-separated, given the conditioning
                variables you choose.
              </p>
            </div>
            {(['left', 'right'] as const).map((side) => (
              <label key={side} className="block">
                <span className={fieldLabel}>
                  {side === 'left' ? 'First variable' : 'Second variable'}
                </span>
                <Select
                  aria-label={
                    side === 'left' ? 'First separation variable' : 'Second separation variable'
                  }
                  value={side === 'left' ? left : right}
                  className={field('text', 'mt-1')}
                  disabled={locked}
                  onChange={(event) => {
                    ;(side === 'left' ? setLeft : setRight)(event.target.value)
                    setGiven([])
                  }}
                >
                  <option value="">Choose</option>
                  {record.result.nodes.map((node, index) =>
                    node.kind === 'fixed' ? null : (
                      <option key={index} value={index}>
                        {swigNodeName(record, index)}
                      </option>
                    ),
                  )}
                </Select>
              </label>
            ))}
            <div className={listClass}>
              <ColumnChecklist
                title="Conditioning variables"
                help="Leave empty for an unconditional separation query."
                columns={record.result.nodes.flatMap((node, index) =>
                  node.kind === 'fixed'
                    ? []
                    : [{ id: String(index), name: swigNodeName(record, index) }],
                )}
                selected={given}
                reserved={[left, right]}
                onChange={setGiven}
              />
            </div>
            {record.dagRevision !== document.current.id && (
              <p className={fieldHint}>
                This result belongs to an earlier revision. Return to that revision or construct a
                new graph before running another query.
              </p>
            )}
            <RunActions
              running={session.job.kind === 'running' && session.job.stage === 'Check separation'}
              onCancel={session.cancel}
              orbLabel="Checking graphical separation"
            >
              <button
                type="button"
                className={button('outline', 'w-full')}
                disabled={
                  locked ||
                  left === '' ||
                  right === '' ||
                  left === right ||
                  record.dagRevision !== document.current.id
                }
                onClick={() =>
                  void run(
                    {
                      ...record.specification,
                      query: {
                        kind: 'separation',
                        left: Number(left),
                        right: Number(right),
                        given: given.map(Number),
                      },
                    },
                    record.names,
                    record.rationale,
                    record.projection,
                  )
                }
              >
                Check separation
              </button>
            </RunActions>
            {latest !== null && (
              <div aria-live="polite">
                <p className="m-0 text-body text-ink" data-testid="swig-conclusion">
                  {describeSeparation(latest)}
                </p>
                <p className="mb-0 mt-1 text-label text-faint">
                  Separation follows from the recorded graph and assumptions. It is not a test
                  against the data.
                </p>
              </div>
            )}
            {checks.length > 1 && (
              <details>
                <DisclosureSummary className="cursor-pointer text-body text-ink">
                  All checks ({checks.length})
                </DisclosureSummary>
                <table className="mt-2 w-full border-collapse text-left text-label">
                  <thead>
                    <tr className="border-b border-line text-faint">
                      <th className="py-1 pr-2 font-medium">Query</th>
                      <th className="py-1 pl-2 font-medium">Decision</th>
                    </tr>
                  </thead>
                  <tbody>
                    {checks.map((check) => (
                      <tr key={check.id} className="border-b border-hair">
                        <td className="py-1.5 pr-2 text-ink">{separationNotation(check)}</td>
                        <td className="py-1.5 pl-2 text-muted">
                          {check.result.conclusion.kind === 'separated' ? 'Separated' : 'Connected'}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </details>
            )}
          </section>
        )}
        <details>
          <DisclosureSummary className="cursor-pointer text-body text-ink">
            Recorded specification
          </DisclosureSummary>
          <div className="mt-2 space-y-2 text-label text-muted">
            <p className="m-0 whitespace-pre-wrap text-ink">{record.rationale}</p>
            <p className="m-0">
              {record.dagRevision === document.current.id
                ? 'Constructed from the current revision of this DAG.'
                : 'Constructed from an earlier revision of this DAG.'}
            </p>
            {record.specification.construction.kind === 'did' && (
              <SwigDidSpecification
                design={record.specification.construction.design}
                names={record.names}
              />
            )}
            {record.projection.kind === 'temporal' && (
              <p className="m-0">
                Time expansion: periods {record.projection.start} to {record.projection.end}.
                Initial-history closure was explicitly assumed.
              </p>
            )}
            {swigDifferences(record.specification.construction)
              .flatMap((pair) => [pair.earlier, pair.later])
              .map((equation, index) => (
                <p key={index} className="m-0 text-ink">
                  {record.names[equation.outcome]} ={' '}
                  {equation.terms
                    .map(
                      (term) =>
                        `${term.identity}(${term.arguments.map((i) => record.names[i]).join(', ')})`,
                    )
                    .join(' + ') || '0'}
                </p>
              ))}
          </div>
        </details>
      </div>
    )

  const inspector = (
    <div className="space-y-4">
      <h3 className="m-0 text-body font-medium">Intervention graph</h3>
      <p className={fieldHint}>
        In a single-world intervention graph (SWIG), each intervened variable is split into its
        natural value and the value set by intervention. The source DAG remains unchanged.
        Conclusions rest on the graph and the recorded assumptions, not on a statistical fit.
      </p>
      <SegmentedControl
        size="sm"
        fill
        ariaLabel="Intervention graph inspector"
        value={savedGraph === null ? 'new' : pane}
        onChange={setPane}
        options={[
          { value: 'new', label: 'New analysis' },
          { value: 'saved', label: 'Saved graph', disabled: savedGraph === null },
        ]}
      />
      {pane === 'saved' && savedGraph !== null ? savedGraph : newAnalysis}
      <details>
        <DisclosureSummary className="cursor-pointer text-body text-ink">
          References
        </DisclosureSummary>
        <p className={fieldHint}>
          Richardson and Robins, Single World Intervention Graphs (2013). Knaus and Pfleiderer,
          Causal Graphs for Conditional Parallel Trends.
        </p>
      </details>
    </div>
  )

  const stage = (
    <section className="@container/panel min-w-0 space-y-4" aria-label="SWIG analysis">
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div className="min-w-0">
          <h2 className="m-0 text-title">Intervention graphs</h2>
          <p className={fieldHint}>{document.name}</p>
        </div>
        <button type="button" className={button('quiet')} onClick={onBack}>
          Return to source DAG
        </button>
      </div>
      {record !== null && <SwigCanvas key={record.id} record={record} />}
    </section>
  )
  return (
    <>
      <WorkbenchLayout
        id="swig"
        stage={stage}
        stageScroll
        inspector={{ title: 'Inspector', body: inspector }}
        bottom={{
          trigger: { label: 'History', icon: 'history' },
          title: `Saved graphs (${entries.length})`,
          defaultSize: 150,
          body: (
            <RunPicker
              label="Saved intervention graphs"
              empty="Construct a graph to record it here."
              runs={entries.map((entry) => {
                const item = entry.graph
                return {
                  id: item.id,
                  title: swigGraphTitle(entry),
                  createdAt: item.createdAt,
                  deleteLabel: 'Delete this saved graph',
                }
              })}
              selected={record?.id}
              onSelect={show}
              onDelete={onDeleteGraph}
            />
          ),
        }}
      />
    </>
  )
}
