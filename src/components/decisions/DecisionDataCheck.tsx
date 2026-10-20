import { useMemo, useState } from 'react'
import { useJob } from '@/analysis/JobsProvider'
import { copyRuleAcrossSeeds } from '@/analysis/imitationPool'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { regretOption } from '@/charts/decisions/regret'
import { useChartTheme } from '@/charts/theme'
import { SurrogateCategoryPicker } from '@/components/estimation/SurrogateCategoryPicker'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'
import { JobNotice } from '@/components/ui/JobNotice'
import { RunActions } from '@/components/ui/RunActions'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { Metadata } from '@/components/ui/Metadata'
import { RunFold } from '@/components/ui/RunFold'
import { PlanComparison } from './PlanComparison'
import { RunMeta } from '@/components/ui/RunMeta'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { button, field, fieldHint, fieldLabel, label, num } from '@/components/ui/recipes'
import type { DagDocument } from '@/domain/dag'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import {
  actionLabel,
  actionsOutsideBounds,
  cellActions,
  comparedPlans,
  copiedPatterns,
  describeLoggedRowsProblem,
  type CopiedPattern,
  shapeLoggedRows,
  summariseSeeds,
  interpretDecisionEvidence,
  listWords,
  planRows,
  decisionColumns,
  decisionEvidenceSchema,
  describeDecisionColumnsProblem,
  type DecisionEvidence,
  type DecisionEvidenceResult,
  type Holdout,
  type LoggedPolicyValueRequest,
} from '@/domain/decisionEvidence'
import type { DecisionFrame } from '@/domain/decisionFrame'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import { categoricalMembershipSchema, type CategoricalMembership } from '@/domain/sampleMembership'
import type { SelectedSource } from '@/domain/workflow'
import { formatTime } from '@/lib/format/date'
import { formatCount, formatPercent, formatStatistic } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Props = {
  readonly frame: DecisionFrame
  readonly document: DagDocument
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  /** Step numbers continue after the framing steps. */
  readonly firstStep: number
  readonly onRecord: (run: DecisionEvidence) => void
}

type Estimator = 'selfNormalized' | 'inverseProbability'

type Draft = {
  readonly sample: string
  readonly randomised: readonly string[]
  readonly logged: readonly string[]
  readonly learning: boolean
  readonly horizon: string
  readonly seeds: string
  readonly propensity: string
  readonly estimator: Estimator
  readonly rounds: string
  readonly copy: boolean
  readonly copySeeds: string
  readonly copyRows: string
  readonly copyEpochs: string
}

const INITIAL: Draft = {
  sample: '',
  randomised: [],
  logged: [],
  learning: false,
  horizon: '10000',
  seeds: '5',
  propensity: '',
  estimator: 'selfNormalized',
  rounds: '200',
  copy: false,
  copySeeds: '4',
  copyRows: '100000',
  copyEpochs: '30',
}

const BOOTSTRAP_SEED = 12345

/** How many numbered steps the data checks add after the framing steps. */
export const DATA_CHECK_STEPS = 4

/** The copy trains on the other rows, and is scored only on these, whenever the run scores rules. */
const HOLDOUT: Holdout = { share: 0.2, seed: 20_261_018 }

const SAMPLE_TITLES = {
  experimental: 'Randomised rows',
  experimentalHelp:
    'Choose the values that mark rows whose action was set at random. Each value can belong to only one group; unselected values are left out and counted.',
  observational: 'Logged rows',
  observationalHelp: 'Choose the values that mark rows whose action the usual policy chose.',
}

const whole = (text: string, min: number, max: number): number | null => {
  const value = Number(text)
  return Number.isInteger(value) && value >= min && value <= max ? value : null
}

const rate = (value: number): string => formatStatistic('mean', value).text

export function DecisionDataCheck({
  frame,
  document,
  source,
  profile,
  prepared,
  firstStep,
  onRecord,
}: Props) {
  const [draft, setDraft] = useState<Draft>(INITIAL)
  const set = (part: Partial<Draft>) => setDraft((current) => ({ ...current, ...part }))
  const [problem, setProblem] = useState<string | null>(null)
  const session = useJob(`decision:data:${frame.id}`)
  const running = session.job.kind === 'running'
  const columns = decisionColumns(frame, document)
  const numeric = profile.columns.filter((column) => isNumericDuckDbType(column.duckdbType))
  const usable = prepared.kind === 'prepared-cross-section' && prepared.resolution.kind === 'none'

  async function check() {
    if (!columns.ok) return
    const { action, outcome, inputs } = columns.value
    const membership: CategoricalMembership | undefined =
      draft.sample === ''
        ? undefined
        : (() => {
            const parsed = categoricalMembershipSchema.safeParse({
              kind: 'categories',
              column: draft.sample,
              experimental: draft.randomised,
              observational: draft.logged,
            })
            return parsed.success ? parsed.data : undefined
          })()
    if (draft.sample !== '' && membership === undefined) {
      setProblem('Choose at least one value for the randomised rows and one for the logged rows.')
      return
    }
    const horizon = whole(draft.horizon, 1, 1_000_000)
    const seeds = whole(draft.seeds, 1, 20)
    const rounds = whole(draft.rounds, 1, 10_000)
    const copySeeds = whole(draft.copySeeds, 1, 16)
    const copyRows = whole(draft.copyRows, 100, 2_000_000)
    const copyEpochs = whole(draft.copyEpochs, 1, 500)
    if (draft.copy && (copySeeds === null || copyRows === null || copyEpochs === null)) {
      setProblem(
        'Enter from 1 to 16 seeds, from 100 to 2,000,000 training rows and from 1 to 500 epochs.',
      )
      return
    }
    if (draft.learning && (horizon === null || seeds === null)) {
      setProblem('Enter a horizon from 1 to 1,000,000 steps and from 1 to 20 seeds.')
      return
    }
    if (draft.learning && membership === undefined) {
      setProblem('The learning replay draws from the randomised rows, so mark them first.')
      return
    }
    if (draft.propensity !== '' && rounds === null) {
      setProblem('Enter from 1 to 10,000 bootstrap rounds.')
      return
    }
    setProblem(null)
    const current = session.start('analysis', 'Count the rows')
    if (current === null) return
    try {
      const data = await import('@/data/client')
      const analysis = await import('@/analysis/client')
      const cells = await data.countDecisionCellsInWorker(
        source.file,
        profile,
        action.column,
        outcome.column,
        membership,
      )
      if (!session.current(current)) return
      if (!cells.ok) {
        session.fail(
          current,
          'The rows could not be counted. Check that the action and outcome are numeric.',
        )
        return
      }
      const actions = cellActions(cells.value)
      if (actions.length < 2 || actions.length > 256) {
        session.fail(
          current,
          `The logged rows hold ${actions.length} distinct actions; the checks need from 2 to 256.`,
        )
        return
      }
      const request = {
        actions: [...actions],
        learning: draft.learning
          ? { horizon: horizon!, seeds: Array.from({ length: seeds! }, (_, i) => i + 1) }
          : null,
      }
      session.progress(
        current,
        draft.learning ? 'Bound the actions and replay the learners' : 'Bound the actions',
      )
      const evidence = await analysis.runDecisionEvidence(cells.value, request)
      if (!session.current(current)) return
      if (!evidence.ok) {
        session.fail(current, describeAnalysisWorkerProblem(evidence.error))
        return
      }
      let scoring = null
      let imitation = null
      if (draft.propensity !== '' || draft.copy) {
        const propensity =
          draft.propensity === '' ? null : numeric.find((column) => column.id === draft.propensity)!
        session.progress(current, 'Read the logged rows')
        const wanted: [ColumnId, ...ColumnId[]] = [
          action.column,
          outcome.column,
          ...(propensity === null ? [] : [propensity.id]),
          ...inputs.map((input) => input.column),
          ...(membership === undefined ? [] : [membership.column]),
        ]
        const offset = propensity === null ? 2 : 3
        const matrix = await data.materializeNumericColumnsInWorker(
          source.file,
          profile,
          wanted,
          membership,
        )
        if (!session.current(current)) return
        if (!matrix.ok) {
          session.fail(current, 'The logged rows could not be read.')
          return
        }
        const logged = shapeLoggedRows(
          {
            values: matrix.value.values,
            validity: matrix.value.validity,
            rowCount: matrix.value.rowCount,
            columns: wanted.length,
          },
          {
            action: 0,
            outcome: 1,
            propensity: propensity === null ? null : 2,
            inputs: inputs.map((_, i) => offset + i),
            membership: membership === undefined ? null : offset + inputs.length,
          },
          actions,
        )
        if (!logged.ok) {
          session.fail(current, describeLoggedRowsProblem(logged.error))
          return
        }
        const holdout = propensity === null ? null : HOLDOUT
        if (draft.copy) {
          session.progress(current, 'Copy the demonstrator', { completed: 0, total: copySeeds! })
          const heldOutRows =
            holdout === null ? null : Math.round(logged.value.rows * holdout.share)
          const available = logged.value.rows - (heldOutRows ?? 0)
          const sampled = Math.min(available, copyRows!)
          const copied = await copyRuleAcrossSeeds(
            logged.value.inputs,
            {
              width: inputs.length,
              actions: {
                kind: 'discrete',
                count: actions.length,
                chosen: [...logged.value.chosen],
              },
              probes: logged.value.patterns.map((pattern) => [...pattern]),
              settings: {
                hidden: 64,
                learningRate: 1e-3,
                epochs: copyEpochs!,
                batchSize: 256,
                validationShare: 0.2,
                patience: 5,
                seed: 0,
              },
              sample: sampled < available ? { rows: sampled, seed: 0 } : null,
              holdout,
            },
            Array.from({ length: copySeeds! }, (_, i) => i + 1),
            (completed, total) =>
              session.progress(current, 'Copy the demonstrator', { completed, total }),
          )
          if (!session.current(current)) return
          if (!copied.ok) {
            session.fail(current, describeAnalysisWorkerProblem(copied.error))
            return
          }
          imitation = {
            inputs: inputs.map((input) => ({ column: input.column, name: input.name })),
            seeds: copySeeds!,
            trainingRows: sampled,
            heldOutRows,
            patterns: logged.value.patterns.map((pattern) => [...pattern]),
            demonstrated: logged.value.demonstrated.map((counts) => [...counts]),
            learned: summariseSeeds(copied.value),
          }
        }
        if (propensity !== null && logged.value.scoring !== null) {
          const model: LoggedPolicyValueRequest = {
            actions: [...actions],
            plans: comparedPlans(actions.length, imitation?.learned.mean ?? null),
            estimator:
              draft.estimator === 'selfNormalized'
                ? { kind: 'selfNormalized' }
                : { kind: 'inverseProbability', maxWeight: null },
            bootstrap: { rounds: rounds!, seed: BOOTSTRAP_SEED, alpha: 0.05 },
            holdout: imitation === null ? null : holdout,
          }
          session.progress(current, 'Score the rules from the logged propensities')
          const scored = await analysis.runLoggedPolicyValue(logged.value.scoring, model)
          if (!session.current(current)) return
          if (!scored.ok) {
            session.fail(current, describeAnalysisWorkerProblem(scored.error))
            return
          }
          scoring = {
            propensity: { column: propensity.id, name: propensity.name },
            request: model,
            result: scored.value,
          }
        }
      }
      const parsed = decisionEvidenceSchema.safeParse({
        kind: 'decision-evidence',
        id: crypto.randomUUID(),
        frame: frame.id,
        prepared: prepared.id,
        createdAt: new Date().toISOString(),
        action,
        outcome,
        randomised: membership ?? null,
        request,
        result: evidence.value,
        scoring,
        imitation,
      })
      if (!parsed.success) {
        session.fail(current, 'The data checks do not match the request. No result was saved.')
        return
      }
      onRecord(parsed.data)
      session.finish(current)
    } catch (cause) {
      if (session.current(current))
        session.fail(current, cause instanceof Error ? cause.message : String(cause))
    }
  }

  return (
    <>
      {!columns.ok ? (
        <Alert tone="warn">{describeDecisionColumnsProblem(columns.error)}</Alert>
      ) : !usable ? (
        <Alert tone="warn">
          These methods read every row of a cross-sectional dataset with no missing values. Prepare
          the data that way to run them.
        </Alert>
      ) : (
        <>
          <SettingsStep
            number={firstStep}
            title="Mark the randomised rows"
            help={`The action is ${columns.value.action.name} and the outcome is ${columns.value.outcome.name}, which must lie between 0 and 1. Rows whose action was set at random give each action's average outcome under intervention. Without them, the results show only the log's view and its natural bounds.`}
          >
            <label className="block max-w-md">
              <span className={fieldLabel}>Column that marks the rows</span>
              <Select
                className={field('text', 'mt-1')}
                value={draft.sample}
                onChange={(event) =>
                  set({ sample: event.target.value, randomised: [], logged: [], learning: false })
                }
              >
                <option value="">No randomised rows</option>
                {profile.columns.map((column) => (
                  <option key={column.id} value={column.id}>
                    {column.name}
                  </option>
                ))}
              </Select>
            </label>
            {draft.sample !== '' && (
              <div className="mt-3">
                <SurrogateCategoryPicker
                  file={source.file}
                  profile={profile}
                  column={draft.sample as ColumnId}
                  experimental={draft.randomised}
                  observational={draft.logged}
                  titles={SAMPLE_TITLES}
                  onChange={(randomised, logged) => set({ randomised, logged })}
                />
              </div>
            )}
          </SettingsStep>
          <SettingsStep
            number={firstStep + 1}
            title="Replay learning on the randomised rows"
            help="An optimistic learner chooses one action per step and receives an outcome drawn from that action's randomised rows. It runs twice, once from scratch and once with each action's index kept inside its natural bounds."
          >
            <SegmentedControl
              ariaLabel="Replay learning"
              className="w-fit"
              value={draft.learning ? 'on' : 'off'}
              onChange={(value) => set({ learning: value === 'on' })}
              options={[
                { value: 'off', label: 'Off' },
                { value: 'on', label: 'On' },
              ]}
            />
            {draft.learning && (
              <div className="mt-3 flex flex-wrap gap-4">
                <label className="block w-40">
                  <span className={fieldLabel}>Steps</span>
                  <input
                    className={field('text', 'mt-1')}
                    inputMode="numeric"
                    value={draft.horizon}
                    onChange={(event) => set({ horizon: event.target.value })}
                  />
                </label>
                <label className="block w-40">
                  <span className={fieldLabel}>Seeds</span>
                  <input
                    className={field('text', 'mt-1')}
                    inputMode="numeric"
                    value={draft.seeds}
                    onChange={(event) => set({ seeds: event.target.value })}
                  />
                </label>
              </div>
            )}
          </SettingsStep>
          <SettingsStep
            number={firstStep + 2}
            title="Copy the demonstrator"
            help="Behaviour cloning learns from the logged rows how the logging policy chose the action, given the variables the decision may read. Several seeds train in parallel, and their spread shows how stable the copy is."
          >
            <SegmentedControl
              ariaLabel="Copy the demonstrator"
              className="w-fit"
              value={draft.copy ? 'on' : 'off'}
              onChange={(value) => set({ copy: value === 'on' })}
              options={[
                { value: 'off', label: 'Off' },
                { value: 'on', label: 'On' },
              ]}
            />
            {draft.copy && (
              <>
                <p className={fieldHint}>
                  {columns.value.inputs.length === 0
                    ? 'The decision reads no variables, so the copy is the logged action frequencies.'
                    : `The copy reads ${listWords(columns.value.inputs.map((input) => input.name))}, as chosen in step 2.`}
                </p>
                <div className="mt-3 flex flex-wrap gap-4">
                  <label className="block w-40">
                    <span className={fieldLabel}>Seeds</span>
                    <input
                      className={field('text', 'mt-1')}
                      inputMode="numeric"
                      value={draft.copySeeds}
                      onChange={(event) => set({ copySeeds: event.target.value })}
                    />
                  </label>
                  <label className="block w-40">
                    <span className={fieldLabel}>Training rows</span>
                    <input
                      className={field('text', 'mt-1')}
                      inputMode="numeric"
                      value={draft.copyRows}
                      onChange={(event) => set({ copyRows: event.target.value })}
                    />
                  </label>
                  <label className="block w-40">
                    <span className={fieldLabel}>Epochs</span>
                    <input
                      className={field('text', 'mt-1')}
                      inputMode="numeric"
                      value={draft.copyEpochs}
                      onChange={(event) => set({ copyEpochs: event.target.value })}
                    />
                  </label>
                </div>
              </>
            )}
          </SettingsStep>
          <SettingsStep
            number={firstStep + 3}
            title="Score plans from logged propensities"
            help="When the log records the probability with which the logging policy chose each action, weighting the logged outcomes estimates what another plan would have achieved. When the demonstrator is copied, a fifth of the logged rows is held out from training and the copy is scored only on those rows."
          >
            <label className="block max-w-md">
              <span className={fieldLabel}>Propensity column</span>
              <Select
                className={field('text', 'mt-1')}
                value={draft.propensity}
                onChange={(event) => set({ propensity: event.target.value })}
              >
                <option value="">No logged propensities</option>
                {numeric.map((column) => (
                  <option key={column.id} value={column.id}>
                    {column.name}
                  </option>
                ))}
              </Select>
            </label>
            {draft.propensity !== '' && (
              <div className="mt-3 flex flex-wrap items-end gap-4">
                <SegmentedControl
                  ariaLabel="Estimator"
                  className="w-fit"
                  value={draft.estimator}
                  onChange={(estimator) => set({ estimator })}
                  options={[
                    { value: 'selfNormalized', label: 'Self-normalised weighting' },
                    { value: 'inverseProbability', label: 'Inverse probability weighting' },
                  ]}
                />
                <label className="block w-40">
                  <span className={fieldLabel}>Bootstrap rounds</span>
                  <input
                    className={field('text', 'mt-1')}
                    inputMode="numeric"
                    value={draft.rounds}
                    onChange={(event) => set({ rounds: event.target.value })}
                  />
                </label>
              </div>
            )}
          </SettingsStep>
          {problem !== null && <Alert tone="warn">{problem}</Alert>}
          <RunActions
            running={running}
            onCancel={session.cancel}
            orbLabel="Policy evaluation running"
          >
            <button
              type="button"
              className={button('signal')}
              disabled={running || session.blocked}
              onClick={() => void check()}
            >
              Evaluate policies
            </button>
          </RunActions>
          <JobNotice job={session.job} />
        </>
      )}
    </>
  )
}

type ActionRow = DecisionEvidenceResult['actions'][number] & { readonly weighted: number | null }

const COPY_TABLE_HELP =
  'For each combination of the inputs the copy reads, the action the demonstrator chose most often and its share of those rows, beside the copy’s probability of that action across seeds. The largest gap compares every action, not only the most frequent.'

const COPY_COLUMNS = (action: string): EvidenceColumn<CopiedPattern>[] => [
  { id: 'inputs', header: 'Inputs', value: (row) => row.label },
  {
    id: 'rows',
    header: 'Rows',
    align: 'right',
    value: (row) => row.rows,
    format: (v) => formatCount(Number(v)).text,
  },
  { id: 'action', header: `Most logged ${action}`, value: (row) => actionLabel(row.action) },
  {
    id: 'logged',
    header: 'Logged share',
    align: 'right',
    value: (row) => row.loggedShare,
    format: (v) => rate(Number(v)),
  },
  {
    id: 'copied',
    header: 'Copied probability',
    align: 'right',
    value: (row) => row.copied,
    format: (_, row) =>
      row.copiedLow === row.copiedHigh
        ? rate(row.copied)
        : `${rate(row.copied)} (${rate(row.copiedLow)} to ${rate(row.copiedHigh)})`,
  },
  {
    id: 'gap',
    header: 'Largest gap',
    align: 'right',
    value: (row) => row.gap,
    format: (v) => rate(Number(v)),
  },
]

const ACTION_TABLE_HELP =
  'Logged share and observed average describe the logged rows. The natural bounds contain each action’s average outcome under intervention, assuming an outcome between 0 and 1; they are computed from a sample. The randomised estimate, with its row count, comes from rows whose action was set at random. The weighted estimate scores the plan that always chooses the action.'

/** One evidence run: the current run keeps the stage, and older runs fold into History. */
export function EvidenceResult({
  run,
  current,
  onDelete,
}: {
  readonly run: DecisionEvidence
  readonly current: boolean
  readonly onDelete?: () => void
}) {
  const theme = useChartTheme()
  const { result, scoring } = run
  const plans = useMemo(() => planRows(run), [run])
  const rows: ActionRow[] = result.actions.map((action, i) => ({
    ...action,
    weighted: plans[i + 1]?.scored.value.value ?? null,
  }))
  const randomised = rows.some((row) => row.randomised !== null)
  const outside = actionsOutsideBounds(result)
  const option = useMemo(
    () =>
      result.learning === null
        ? null
        : regretOption(
            {
              steps: result.learning.steps,
              seeds: result.learning.scratch.finals.length,
              curves: [
                { name: 'From scratch', ...result.learning.scratch },
                { name: 'Within the natural bounds', ...result.learning.clipped },
              ],
            },
            theme,
          ),
    [result.learning, theme],
  )
  const columns: EvidenceColumn<ActionRow>[] = [
    { id: 'action', header: run.action.name, value: (row) => actionLabel(row.value) },
    {
      id: 'share',
      header: 'Logged share',
      align: 'right',
      value: (row) => row.share,
      format: (v) => formatPercent(Number(v)).text,
    },
    {
      id: 'log',
      header: 'Observed average',
      align: 'right',
      value: (row) => row.logged.mean,
      format: (v) => rate(Number(v)),
    },
    {
      id: 'bounds',
      header: 'Natural bounds',
      align: 'right',
      value: (row) => row.lower,
      format: (_, row) => `${rate(row.lower)} to ${rate(row.upper)}`,
    },
    ...(randomised
      ? [
          {
            id: 'randomised',
            header: 'Randomised estimate',
            align: 'right' as const,
            value: (row: ActionRow) => row.randomised?.mean ?? -1,
            format: (_: unknown, row: ActionRow) =>
              row.randomised === null
                ? 'Not evaluated'
                : `${rate(row.randomised.mean)} (${formatCount(row.randomised.rows).text} rows)`,
          },
        ]
      : []),
    ...(scoring === null
      ? []
      : [
          {
            id: 'weighted',
            header: 'Weighted estimate',
            align: 'right' as const,
            value: (row: ActionRow) => row.weighted ?? -1,
            format: (_: unknown, row: ActionRow) =>
              row.weighted === null ? 'Not evaluated' : rate(row.weighted),
          },
        ]),
  ]
  const copied =
    run.imitation === null
      ? null
      : copiedPatterns(
          run.imitation,
          result.actions.map((action) => action.value),
        )
  const title = `${run.action.name} for ${run.outcome.name}`
  const stamp = <RunMeta>{['Policy evaluation', formatTime(run.createdAt)]}</RunMeta>
  const counts = (
    <p className={num('mb-0 mt-1 text-body text-bone')}>
      <Metadata>
        <span>{formatCount(result.loggedRows).text} logged rows</span>
        <span>{formatCount(result.randomisedRows).text} randomised rows</span>
        <span>{formatCount(result.actions.length).text} actions</span>
      </Metadata>
    </p>
  )
  const body = (
    <>
      {!current && outside.length > 0 && (
        <Alert
          tone="warn"
          live={false}
          title="Some randomised estimates fall outside the natural bounds"
          className="mt-3"
        >
          The randomised rows either come from a different population or measure the outcome
          differently, or sampling error has moved a mean across a bound.
        </Alert>
      )}
      <ResultInterpretation interpretation={interpretDecisionEvidence(run)} className="mt-3" />
      <div className="-mx-3.5 mt-4">
        <EvidenceTable
          title="Each action by source"
          help={ACTION_TABLE_HELP}
          frame="none"
          rows={rows}
          rowKey={(row) => String(row.value)}
          noun="action"
          empty="No actions were read."
          columns={columns}
        />
      </div>
      {plans.length > 0 && <PlanComparison run={run} plans={plans} />}
      {copied !== null && (
        <div className="-mx-3.5 mt-4">
          <EvidenceTable
            title="Copied rule by input pattern"
            help={COPY_TABLE_HELP}
            frame="none"
            rows={copied}
            rowKey={(row) => row.label}
            noun="pattern"
            empty="No input patterns were copied."
            columns={COPY_COLUMNS(run.action.name)}
          />
        </div>
      )}
      {option !== null && (
        <section aria-labelledby={`regret-${run.id}`} className="mt-4">
          <div className="flex items-center gap-1.5">
            <h4 id={`regret-${run.id}`} className="m-0 text-body font-medium text-ink">
              Learning replay
            </h4>
            <ParameterHelp
              label="Learning replay"
              help="Regret is the outcome given up against always choosing the action with the best randomised estimate. Each band shows the range across seeds."
            />
          </div>
          <ExpandableChart
            option={option}
            label="Cumulative regret of the two learners"
            className="mt-2 h-[260px]"
          />
        </section>
      )}
    </>
  )
  if (!current)
    return (
      <RunFold
        title={title}
        stamp={stamp}
        onDelete={onDelete}
        deleteLabel="Delete this evidence run"
      >
        {counts}
        {body}
      </RunFold>
    )
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label={`${title} evidence`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal-text')}>Current evidence</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      <h3 className="mb-0 mt-1 text-title font-normal leading-7 text-ink">{title}</h3>
      {counts}
      {body}
    </article>
  )
}
