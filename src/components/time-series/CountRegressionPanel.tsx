import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { useCohortPeriods } from '@/components/estimation/useCohortPeriods'
import type { CountOutcomeReadiness } from '@/domain/regressionDesigns'
import { RunActions } from '@/components/ui/RunActions'
import type { ReactNode } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import { useTimeSeriesDraft } from './useTimeSeriesDraft'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { Orb } from '@/components/ui/Orb'
import { Alert } from '@/components/ui/Alert'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import {
  actionGap,
  button,
  field,
  fieldHint,
  fieldLabel,
  fieldRow,
  panel,
  stepsStack,
} from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { cn } from '@/lib/utils'
import { isNumericDuckDbType, type ColumnId } from '@/domain/dataset'
import { isNonEmpty } from '@/domain/dop'
import { countRegressionRequestSchema, type CountRegressionDraft } from '@/domain/countRegression'
import { describePanelProblem } from '@/domain/regularPanel'
import { isRegressionDesignRun, newTimeSeriesRunId, parseTimeSeriesRun } from '@/domain/timeSeries'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { useRunActivity } from '@/lib/useRunActivity'
import { TimeSeriesHeading, type TimeSeriesPanelProps } from './TimeSeriesPanel'
import { TimeSeriesHistory } from './TimeSeriesHistory'
import { CountRegressionResult } from './CountRegressionResult'

export function CountRegressionPanel(
  props: Pick<
    TimeSeriesPanelProps,
    'source' | 'profile' | 'runs' | 'onRun' | 'onDeleteRun' | 'onActivity'
  > & {
    readonly prepared: import('@/domain/preprocessing').PreparedDatasetArtifact
    readonly selector?: ReactNode
    readonly scope?: 'temporal' | 'cohorts'
    readonly analysisSelector?: ReactNode
    readonly inspectorLead?: ReactNode
    readonly readiness?: CountOutcomeReadiness
    readonly runLabel?: string
  },
) {
  const cohorts = props.scope === 'cohorts'
  const draft = useTimeSeriesDraft(props.prepared.id, (s) =>
    cohorts ? s.cohortRegression : s.regression,
  )
  const change = useWorkflow((s) => s.changeTimeSeries)
  const set = (next: CountRegressionDraft) =>
    change(props.prepared.id, { type: cohorts ? 'cohort-regression' : 'regression', value: next })
  const session = useJob(cohorts ? 'estimation:cohort-regression' : 'count-regression'),
    { job } = session
  useRunActivity(
    props.onActivity,
    job.kind === 'running' ? { label: 'Count regression', progress: null } : null,
  )
  const columns = props.profile.columns.filter(
    (c) => props.prepared.columns.includes(c.id) && isNumericDuckDbType(c.duckdbType),
  )
  const panelData = props.prepared.kind === 'prepared-panel'
  const runs = props.runs.filter(
    (r): r is Extract<typeof r, { kind: 'count-regression' }> =>
      r.kind === 'count-regression' &&
      (cohorts
        ? r.specification.design.kind === 'events' || r.specification.design.kind === 'summary'
        : r.specification.design.kind === 'lags' || r.specification.design.kind === 'interrupted'),
  )
  const denominator = draft.family.kind === 'binomial' ? draft.family.trials : draft.family.exposure
  const predictor = draft.model.kind === 'lags' ? draft.model.predictor : null
  const onset =
    draft.model.kind === 'events' || draft.model.kind === 'summary' ? draft.model.onset : null
  const cohortPeriods = useCohortPeriods(
    cohorts,
    props.source,
    props.profile,
    props.prepared,
    onset,
  )
  const readinessProblem =
    props.readiness?.kind === 'checking'
      ? 'Checking the prepared outcome values.'
      : props.readiness?.kind === 'refused'
        ? props.readiness.reason
        : null
  const lags =
    draft.model.kind === 'lags' ? draft.model.lags.split(',').map((v) => Number(v.trim())) : []
  const problem =
    readinessProblem ??
    (draft.outcome === null
      ? 'Choose an outcome.'
      : draft.family.kind === 'binomial' && denominator === null
        ? 'Choose the trials column.'
        : denominator === draft.outcome
          ? 'Choose a denominator different from the outcome.'
          : draft.model.kind === 'lags' &&
              (predictor === null ||
                lags.length === 0 ||
                lags.some((l) => !Number.isSafeInteger(l) || l < 0) ||
                new Set(lags).size !== lags.length)
            ? 'Choose a predictor and distinct non-negative lags.'
            : (draft.model.kind === 'events' || draft.model.kind === 'summary') &&
                (onset === null ||
                  cohortPeriods.kind !== 'ready' ||
                  !cohortPeriods.labels.includes(draft.model.cohort))
              ? cohortPeriods.kind === 'refused'
                ? cohortPeriods.reason
                : cohortPeriods.kind === 'checking'
                  ? 'Checking the observed adoption periods.'
                  : 'Choose the treatment indicator and an observed adoption period.'
              : Number(draft.confidence) <= 0 ||
                  Number(draft.confidence) >= 1 ||
                  !Number.isFinite(Number(draft.confidence))
                ? 'Confidence must be between zero and one.'
                : !Number.isSafeInteger(Number(draft.iterations)) || Number(draft.iterations) < 1
                  ? 'Optimizer iterations must be a positive integer.'
                  : !Number.isFinite(Number(draft.tolerance)) || Number(draft.tolerance) <= 0
                    ? 'Optimizer tolerance must be positive.'
                    : draft.model.kind === 'lags' &&
                        (predictor === draft.outcome || predictor === denominator)
                      ? 'Choose a predictor separate from the outcome and denominator.'
                      : draft.covariates.some((id) =>
                            [draft.outcome, denominator, predictor, onset].includes(id),
                          )
                        ? 'Remove outcome, denominator and predictor roles from the covariate selection.'
                        : draft.model.kind === 'interrupted' &&
                            (!Number.isSafeInteger(Number(draft.model.intervention)) ||
                              Number(draft.model.intervention) < 3 ||
                              !Number.isSafeInteger(Number(draft.model.horizon)) ||
                              Number(draft.model.horizon) < 0 ||
                              !Number.isSafeInteger(Number(draft.model.bandwidth)) ||
                              Number(draft.model.bandwidth) < 0)
                          ? 'Choose an intervention row from 3 onward and non-negative integer horizon and bandwidth.'
                          : draft.model.kind === 'events' &&
                              draft.model.window.kind === 'finite' &&
                              (!Number.isSafeInteger(Number(draft.model.window.first)) ||
                                Number(draft.model.window.first) > -1 ||
                                !Number.isSafeInteger(Number(draft.model.window.last)) ||
                                Number(draft.model.window.last) < 0)
                            ? 'The event window must start at or before -1 and end at or after 0.'
                            : null)
  const first = panelData || draft.model.kind === 'lags' ? 1 : 0
  const setCohort = (update: { onset?: ColumnId | null; cohort?: string }) => {
    if (draft.model.kind === 'events' || draft.model.kind === 'summary')
      set({ ...draft, model: { ...draft.model, ...update } })
  }
  const choose = (
    label: string,
    value: ColumnId | null,
    onChange: (v: ColumnId | null) => void,
    optional = false,
  ) => (
    <label className="block">
      <span className={fieldLabel}>{label}</span>
      <Select
        aria-label={label}
        className={field('text', 'mt-1')}
        value={value ?? ''}
        onChange={(e) => onChange(columns.find((c) => c.id === e.target.value)?.id ?? null)}
      >
        <option value="">{optional ? 'None' : `Choose ${label.toLowerCase()}`}</option>
        {columns.map((c) => (
          <option key={c.id} value={c.id}>
            {c.name}
          </option>
        ))}
      </Select>
    </label>
  )
  const input = (label: string, value: string, onChange: (v: string) => void) => (
    <label className="block">
      <span className={fieldLabel}>{label}</span>
      <input
        aria-label={label}
        className={field('text', 'mt-1')}
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
    </label>
  )
  const execute = async () => {
    if (problem !== null || draft.outcome === null) return
    const current = session.start('analysis', 'Preparing count regression')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    try {
      const selected = [
        ...new Set(
          [draft.outcome, denominator, predictor, onset, ...draft.covariates].filter(
            (id): id is ColumnId => id !== null,
          ),
        ),
      ]
      if (!isNonEmpty(selected)) {
        fail('Choose model columns.')
        return
      }
      const { materialisePrepared, describePreparedMaterialisationProblem } =
        await import('@/data/prepared')
      const result = await materialisePrepared(
        props.source,
        props.profile,
        props.prepared,
        selected,
      )
      if (!session.current(current)) return
      if (!result.ok) {
        fail(describePreparedMaterialisationProblem(result.error))
        return
      }
      const matrix = result.value
      const at = (id: ColumnId) => matrix.columns.findIndex((c) => c.id === id)
      const name = (id: ColumnId) => matrix.columns.find((c) => c.id === id)?.name ?? String(id)
      const covariates = draft.covariates.map((id) => ({ column: at(id), lag: 0, name: name(id) }))
      const { prepareCountDesign } = await import('@/data/countRegressionInput')
      const preparedDesign = await prepareCountDesign(
        props.source,
        props.profile,
        props.prepared,
        matrix,
        draft.model,
        covariates,
        () => session.current(current),
      )
      if (!session.current(current)) return
      if (!preparedDesign.ok) {
        fail(describePanelProblem(preparedDesign.error))
        return
      }
      const { design, periods } = preparedDesign.value
      const parsed = countRegressionRequestSchema.safeParse({
        rows: matrix.rowCount,
        columns: matrix.columns.length,
        outcome: at(draft.outcome),
        family:
          draft.family.kind === 'binomial'
            ? { kind: 'binomial', trials: denominator === null ? -1 : at(denominator) }
            : { kind: 'negativeBinomial', exposure: denominator === null ? null : at(denominator) },
        design,
        confidence: Number(draft.confidence),
        iterations: Number(draft.iterations),
        tolerance: Number(draft.tolerance),
      })
      if (!parsed.success) {
        fail(parsed.error.issues.map((i) => i.message).join(' '))
        return
      }
      const { runCountRegression } = await import('@/analysis/client')
      const evidence = await runCountRegression(matrix.values, parsed.data)
      if (!session.current(current)) return
      if (!evidence.ok) {
        fail(describeAnalysisWorkerProblem(evidence.error))
        return
      }
      const saved = parseTimeSeriesRun({
        kind: 'count-regression',
        id: newTimeSeriesRunId(),
        createdAt: new Date().toISOString(),
        preparedDataset: props.prepared.id,
        outcome: { id: draft.outcome, name: name(draft.outcome) },
        variables: matrix.columns.map((c) => ({ id: c.id, name: c.name })),
        periods,
        specification: parsed.data,
        evidence: evidence.value,
      })
      if (!saved.ok) {
        fail(saved.error)
        return
      }
      props.onRun(saved.value)
      session.finish(current)
    } catch (error: unknown) {
      fail(error instanceof Error ? error.message : String(error))
    }
  }
  return (
    <WorkbenchLayout
      id="count-regression"
      bottom={{
        trigger: { label: 'History', icon: 'history' },
        title: cohorts ? 'Regression-design runs' : 'Regression runs',
        defaultSize: 150,
        body: (
          <TimeSeriesHistory
            label={cohorts ? 'Regression-design runs' : 'Time-series runs'}
            entries={cohorts ? props.runs.filter(isRegressionDesignRun) : runs}
            onDelete={(r) => props.onDeleteRun(r.id)}
          />
        ),
      }}
      inspector={{
        trigger: { label: 'Requirements', icon: 'contract' },
        title: 'Model requirements',
        body: (
          <div className="flex flex-col gap-4">
            {props.inspectorLead ??
              (cohorts ? null : (
                <section aria-labelledby="count-regression-title">
                  <h3
                    id="count-regression-title"
                    className="m-0 mt-1 text-body font-medium text-ink"
                  >
                    Count regression
                  </h3>
                  <p className="mb-0 mt-2 text-body text-muted">
                    Negative-binomial and grouped-binomial regression with explicit denominators and
                    dependence-adjusted uncertainty.
                  </p>
                </section>
              ))}
            <div className="space-y-3 text-body text-muted">
              <p>
                NB2 uses non-negative integer counts and a positive exposure. Grouped binomial uses
                integer successes and trials.
              </p>
              <p>
                {panelData
                  ? 'Panel models require a complete regular panel. Lags stay within teams. Team and period fixed effects are included.'
                  : 'Interrupted series uses the prepared values, a level change and a slope change. The slope-change term is 1 at the first post-event observation.'}
              </p>
              <p>
                Clustered and HAC covariance use finite-sample corrections. Confidence intervals use
                the normal distribution.
              </p>
            </div>
          </div>
        ),
      }}
      stage={
        <section className="@container/panel flex flex-col gap-5">
          {cohorts ? <ChapterHeading>Estimation</ChapterHeading> : <TimeSeriesHeading />}
          <section className={panel('p-(--panel-space)')}>
            {props.selector}
            <fieldset
              disabled={job.kind === 'running'}
              className={
                props.selector === undefined
                  ? 'm-0 min-w-0 border-0 p-0'
                  : 'm-0 mt-6 min-w-0 border-0 p-0'
              }
            >
              <legend className="sr-only">Count regression specification</legend>
              <div className={stepsStack}>
                {(panelData || draft.model.kind === 'lags') && (
                  <SettingsStep
                    number={1}
                    title={cohorts ? 'Choose the analysis' : 'Choose the model'}
                  >
                    {cohorts &&
                      (props.analysisSelector ?? (
                        <SegmentedControl
                          className="justify-self-start"
                          ariaLabel="Count regression model"
                          value={draft.model.kind === 'interrupted' ? 'lags' : draft.model.kind}
                          onChange={(kind) =>
                            set({
                              ...draft,
                              family: { kind: 'negativeBinomial', exposure: null },
                              model:
                                kind === 'lags'
                                  ? { kind, predictor: null, lags: '1, 2, 3, 4' }
                                  : kind === 'events'
                                    ? { kind, onset: null, cohort: '', window: { kind: 'all' } }
                                    : { kind, onset: null, cohort: '' },
                            })
                          }
                          options={[
                            { value: 'events', label: 'Cohort event study' },
                            { value: 'summary', label: 'Cohort summary' },
                          ]}
                        />
                      ))}
                    {draft.model.kind === 'lags' && (
                      <SegmentedControl
                        className="justify-self-start"
                        ariaLabel="Regression family"
                        value={draft.family.kind}
                        onChange={(kind) =>
                          set({
                            ...draft,
                            family:
                              kind === 'binomial'
                                ? { kind, trials: null }
                                : { kind, exposure: null },
                          })
                        }
                        options={[
                          { value: 'negativeBinomial', label: 'Negative binomial' },
                          { value: 'binomial', label: 'Grouped binomial' },
                        ]}
                      />
                    )}
                  </SettingsStep>
                )}
                <SettingsStep number={first + 1} title="Choose the columns">
                  <div className={fieldRow.two}>
                    {choose('Outcome', draft.outcome, (outcome) => set({ ...draft, outcome }))}
                    {choose(
                      draft.family.kind === 'binomial' ? 'Trials' : 'Exposure',
                      denominator,
                      (value) =>
                        set({
                          ...draft,
                          family:
                            draft.family.kind === 'binomial'
                              ? { kind: 'binomial', trials: value }
                              : { kind: 'negativeBinomial', exposure: value },
                        }),
                      draft.family.kind === 'negativeBinomial',
                    )}
                  </div>
                  {draft.model.kind === 'lags' && (
                    <div className={fieldRow.two}>
                      {choose('Predictor', draft.model.predictor, (predictor) =>
                        set({
                          ...draft,
                          model: {
                            kind: 'lags',
                            predictor,
                            lags: draft.model.kind === 'lags' ? draft.model.lags : '1',
                          },
                        }),
                      )}
                      {input('Predictor lags', draft.model.lags, (lags) =>
                        set({ ...draft, model: { kind: 'lags', predictor, lags } }),
                      )}
                    </div>
                  )}
                  {(draft.model.kind === 'events' || draft.model.kind === 'summary') && (
                    <div className={fieldRow.two}>
                      {choose('Treatment indicator', onset, (onset) => setCohort({ onset }))}
                      <label>
                        <span className={fieldLabel}>Adoption cohort</span>
                        <Select
                          aria-label="Adoption cohort"
                          className={field('text', 'mt-1')}
                          value={draft.model.cohort}
                          disabled={cohortPeriods.kind !== 'ready'}
                          onChange={(e) => setCohort({ cohort: e.target.value })}
                        >
                          <option value="">Choose adoption period</option>
                          {cohortPeriods.kind === 'ready' &&
                            cohortPeriods.labels.map((label) => (
                              <option key={label} value={label}>
                                {label}
                              </option>
                            ))}
                        </Select>
                      </label>
                    </div>
                  )}
                  {(draft.model.kind === 'events' || draft.model.kind === 'summary') && (
                    <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
                      The treatment indicator is 0 before adoption and 1 from adoption onward.
                      Never-treated units remain 0.
                    </p>
                  )}
                  {draft.model.kind === 'interrupted' && (
                    <div className={fieldRow.three}>
                      {input(
                        'Intervention row (from 1)',
                        draft.model.intervention,
                        (intervention) =>
                          set({
                            ...draft,
                            model: {
                              ...(draft.model as Extract<
                                CountRegressionDraft['model'],
                                { kind: 'interrupted' }
                              >),
                              intervention,
                            },
                          }),
                      )}
                      {input(
                        'Contrast periods after intervention',
                        draft.model.horizon,
                        (horizon) =>
                          set({
                            ...draft,
                            model: {
                              ...(draft.model as Extract<
                                CountRegressionDraft['model'],
                                { kind: 'interrupted' }
                              >),
                              horizon,
                            },
                          }),
                      )}
                      {input('HAC bandwidth', draft.model.bandwidth, (bandwidth) =>
                        set({
                          ...draft,
                          model: {
                            ...(draft.model as Extract<
                              CountRegressionDraft['model'],
                              { kind: 'interrupted' }
                            >),
                            bandwidth,
                          },
                        }),
                      )}
                    </div>
                  )}
                  {draft.model.kind === 'events' && (
                    <>
                      <div>
                        <span className={fieldLabel}>Event window</span>
                        <SegmentedControl
                          className="mt-1"
                          ariaLabel="Event window"
                          value={draft.model.window.kind}
                          options={[
                            { value: 'all', label: 'All observed periods' },
                            { value: 'finite', label: 'Specified window' },
                          ]}
                          onChange={(kind) => {
                            if (draft.model.kind === 'events')
                              set({
                                ...draft,
                                model: {
                                  ...draft.model,
                                  window:
                                    kind === 'all' ? { kind } : { kind, first: '-8', last: '12' },
                                },
                              })
                          }}
                        />
                      </div>
                      {draft.model.window.kind === 'finite' && (
                        <div className={fieldRow.two}>
                          {input('First event period', draft.model.window.first, (first) => {
                            if (
                              draft.model.kind === 'events' &&
                              draft.model.window.kind === 'finite'
                            )
                              set({
                                ...draft,
                                model: { ...draft.model, window: { ...draft.model.window, first } },
                              })
                          })}
                          {input('Last event period', draft.model.window.last, (last) => {
                            if (
                              draft.model.kind === 'events' &&
                              draft.model.window.kind === 'finite'
                            )
                              set({
                                ...draft,
                                model: { ...draft.model, window: { ...draft.model.window, last } },
                              })
                          })}
                        </div>
                      )}
                    </>
                  )}
                  <ColumnChecklist
                    title="Covariates"
                    help="Contemporaneous covariates. Fixed effects are generated from the panel keys."
                    columns={columns}
                    selected={draft.covariates}
                    reserved={[draft.outcome, denominator, predictor, onset].filter(
                      (id): id is ColumnId => id !== null,
                    )}
                    onChange={(covariates) => set({ ...draft, covariates })}
                  />
                </SettingsStep>
                <SettingsStep number={first + 2} title="Fit the model">
                  <div className={fieldRow.three}>
                    {input('Confidence level', draft.confidence, (confidence) =>
                      set({ ...draft, confidence }),
                    )}
                    {input('Optimizer iterations', draft.iterations, (iterations) =>
                      set({ ...draft, iterations }),
                    )}
                    {input('Optimizer tolerance', draft.tolerance, (tolerance) =>
                      set({ ...draft, tolerance }),
                    )}
                  </div>
                </SettingsStep>
              </div>
            </fieldset>
            <div className={cn(actionGap, 'grid gap-3')}>
              {problem !== null && (
                <Alert tone="info" live={false}>
                  {problem}
                </Alert>
              )}
              <RunActions
                running={job.kind === 'running'}
                onCancel={session.cancel}
                orbLabel="Regression running"
              >
                <button
                  type="button"
                  className={button('signal')}
                  disabled={problem !== null || session.blocked || job.kind === 'running'}
                  aria-busy={job.kind === 'running'}
                  onClick={() => void execute()}
                >
                  {props.runLabel ?? 'Fit regression'}
                </button>
              </RunActions>
            </div>
            <JobNotice job={job} />
          </section>
          {runs.slice(-1).map((run) => (
            <CountRegressionResult key={run.id} run={run} />
          ))}
        </section>
      }
    />
  )
}
