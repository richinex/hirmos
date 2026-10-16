import { useMemo, useState, type ReactNode } from 'react'
import { useJob } from '@/analysis/JobsProvider'
import { useRecordedAnalysis, runReceipt } from '@/assistant/useRecordedAnalysis'
import { useWorkspaceConfiguration } from '@/assistant/workspaceConfiguration'
import { err, ok } from '@/domain/dop'
import {
  pretestMeaning,
  pretestCandidates,
  pretestInput,
  pretestRequest,
  pretestRunSchema,
  pretestSettings,
  pretestSettingsSchema,
  type PretestRun,
  type PretestSettings,
} from '@/domain/pretestCoverage'
import { newSensitivityRunId } from '@/domain/sensitivity'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { EstimationRunArtifact } from '@/domain/estimation'
import type { TimeSeriesRun } from '@/domain/timeSeries'
import type { RunActivity } from '@/domain/activity'
import { useRunActivity } from '@/lib/useRunActivity'
import { formatPercent, formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { Select } from '@/components/ui/Select'
import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { Alert } from '@/components/ui/Alert'
import { RunActions } from '@/components/ui/RunActions'
import { JobNotice } from '@/components/ui/JobNotice'
import { RunFold } from '@/components/ui/RunFold'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { PutOnDesk } from '@/components/results/desk/PutOnDesk'
import {
  actionGap,
  button,
  panel,
  field,
  fieldLabel,
  stepsStack,
  chapterIntroSingle,
  label,
  num,
} from '@/components/ui/recipes'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { RunDetails } from '@/components/ui/RunDetails'
import { RunMeta } from '@/components/ui/RunMeta'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { interpretSensitivityResult } from '@/domain/resultInterpretation'

const period = (t: number) => (t < 0 ? `−${-t}` : String(t))

const scenarioLabel = (settings: PretestSettings) => {
  if (settings.scenario.kind === 'specified')
    return `Specified bias of ${formatStatistic('raw', settings.scenario.bias).text} standard errors`
  const offset = settings.scenario.offset
  return offset === 0
    ? 'Recorded pre-trend t-statistic'
    : `Recorded pre-trend t-statistic ${offset < 0 ? '−' : '+'} ${formatStatistic('raw', Math.abs(offset)).text}`
}

function PretestCoverageRecord({ run }: { readonly run: PretestRun }) {
  const e = run.evidence
  const nominal = formatPercent(run.settings.confidence, { precision: 1 }).text
  const gap = Math.abs(e.conditional - e.unconditional) * 100
  return (
    <>
      <h3 className="mb-1 mt-2 text-title font-medium leading-7 text-ink">Pre-test coverage</h3>
      <p className="m-0 text-body text-muted">
        Coverage of the {nominal} interval for period {period(run.settings.postPeriod)}, with a
        two-sided pre-test of period {period(run.settings.prePeriod)} at the{' '}
        {formatPercent(run.settings.pretestAlpha, { precision: 1 }).text} level.
      </p>
      <ResultInterpretation
        interpretation={interpretSensitivityResult(run)}
        context="sensitivity-check"
        className="mt-3"
      />
      <MetricGrid className="mt-3" label="Coverage">
        <MetricTile
          size="compact"
          frame="cell"
          label="Coverage without the pre-test"
          value={formatPercent(e.unconditional, { precision: 1 })}
          context={`Nominal ${nominal}`}
        />
        <MetricTile
          size="compact"
          frame="cell"
          label="Coverage among samples that pass"
          value={formatPercent(e.conditional, { precision: 1 })}
          context={
            gap < 0.05
              ? 'Equal to the rate without the pre-test'
              : `${gap.toFixed(1)} percentage points ${e.conditional < e.unconditional ? 'below' : 'above'} the rate without the pre-test`
          }
        />
        <MetricTile
          size="compact"
          frame="cell"
          label="Probability of passing the pre-test"
          value={formatPercent(e.passProbability, { precision: 1 })}
          context={`Pre-test level ${formatPercent(run.settings.pretestAlpha, { precision: 1 }).text}`}
        />
      </MetricGrid>
      <RunDetails label="Coverage specification">
        <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body">
          <dt className="text-faint">Pre-treatment period</dt>
          <dd className={num('m-0 text-ink')}>{period(run.settings.prePeriod)}</dd>
          <dt className="text-faint">Post-treatment period</dt>
          <dd className={num('m-0 text-ink')}>{period(run.settings.postPeriod)}</dd>
          {'kind' in run.eventStudy && (
            <>
              <dt className="text-faint">Pre-treatment coefficient</dt>
              <dd className="m-0 text-ink">{run.eventStudy.names[0]}</dd>
              <dt className="text-faint">Post-treatment coefficient</dt>
              <dd className="m-0 text-ink">{run.eventStudy.names[1]}</dd>
              <dt className="text-faint">Omitted reference period</dt>
              <dd className={num('m-0 text-ink')}>{period(run.eventStudy.reference)}</dd>
            </>
          )}
          <dt className="text-faint">Pre-trend bias scenario</dt>
          <dd className="m-0 text-ink">{scenarioLabel(run.settings)}</dd>
          <dt className="text-faint">Assumed pre-trend bias</dt>
          <dd className={num('m-0 text-ink')}>
            {formatStatistic('raw', e.request.pretestBias).text} standard errors
          </dd>
          <dt className="text-faint">Assumed target bias</dt>
          <dd className={num('m-0 text-ink')}>
            {formatStatistic('raw', e.request.targetBias).text} standard errors
          </dd>
          <dt className="text-faint">Correlation between the estimates</dt>
          <dd className={num('m-0 text-ink')}>
            {formatStatistic('raw', e.request.correlation).text}
          </dd>
          <dt className="text-faint">Numerical integration error</dt>
          <dd className={num('m-0 text-ink')}>{e.integrationError.toExponential(2)}</dd>
        </dl>
      </RunDetails>
    </>
  )
}

export function PretestCoverageResult({ run }: { readonly run: PretestRun }) {
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label="Pre-test coverage result">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal-text')}>Current run</span>
        <span className={num('text-micro text-faint')}>
          <RunMeta>{[scenarioLabel(run.settings), formatTime(run.createdAt)]}</RunMeta>
        </span>
      </div>
      <PretestCoverageRecord run={run} />
    </article>
  )
}

export function PretestCoveragePanel({
  prepared,
  estimates,
  designs,
  runs,
  onRun,
  onDelete,
  onActivity,
  selector,
}: {
  readonly prepared: PreparedDatasetArtifact
  readonly estimates: readonly EstimationRunArtifact[]
  readonly designs: readonly TimeSeriesRun[]
  readonly runs: readonly PretestRun[]
  readonly onRun: (run: PretestRun) => void
  readonly onDelete: (id: PretestRun['id']) => void
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly selector: ReactNode
}) {
  const candidates = useMemo(
    () => pretestCandidates(estimates, designs).filter((c) => c.preparedDataset === prepared.id),
    [estimates, designs, prepared.id],
  )
  const latest = runs.at(-1)
  const [settings, setSettings] = useState<PretestSettings>(
    () =>
      latest?.settings ?? {
        sourceRun: candidates.at(-1)?.source.run ?? '',
        prePeriod: -2,
        postPeriod: 0,
        scenario: { kind: 'calibrated', offset: 0 },
        confidence: 0.95,
        pretestAlpha: 0.05,
      },
  )
  const [pendingDelete, setPendingDelete] = useState<PretestRun | null>(null)
  const session = useJob('sensitivity:pretest-coverage'),
    job = session.job
  const candidate = candidates.find((c) => c.source.run === settings.sourceRun)
  const check = (value: PretestSettings) => {
    if (value.sourceRun === '')
      return err('Choose a saved regression or event study from this prepared dataset.')
    const parsed = pretestSettingsSchema.safeParse(value)
    if (!parsed.success)
      return err(
        'Choose an event-study run, two supported periods, finite scenario values and levels strictly between zero and one.',
      )
    const source = candidates.find((c) => c.source.run === value.sourceRun)
    if (!source) return err('Choose a saved regression or event study from this prepared dataset.')
    const input = pretestInput(source, parsed.data)
    if (!input.ok) return input
    return pretestRequest(input.value, parsed.data)
  }
  const request = check(settings),
    readiness = request.ok ? null : request.error
  useWorkspaceConfiguration(
    'pretest-coverage',
    prepared.id,
    job.kind === 'running'
      ? null
      : {
          title: 'Pre-test coverage',
          current: settings,
          schema: pretestSettingsSchema,
          columns: [],
          describe: (value) => pretestSettings(value, candidates),
          validate: (value) => {
            const result = check(value)
            return result.ok ? ok(value) : err(result.error)
          },
          configure: setSettings,
        },
  )
  useRunActivity(
    onActivity,
    job.kind === 'running' ? { label: 'Pre-test coverage', progress: null } : null,
  )
  const execute = async () => {
    if (!request.ok || !candidate) return
    const input = pretestInput(candidate, settings)
    if (!input.ok) return
    const current = session.start('analysis', 'Calculating pre-test coverage')
    if (current === null) return
    try {
      const { runPretestCoverage } = await import('@/analysis/client')
      const result = await runPretestCoverage(request.value)
      if (!session.current(current)) return
      if (!result.ok) {
        session.fail(current, describeAnalysisWorkerProblem(result.error))
        return
      }
      const saved = pretestRunSchema.safeParse({
        kind: 'pretest-coverage-run',
        id: newSensitivityRunId(),
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        source: candidate.source,
        eventStudy: input.value,
        settings,
        evidence: result.value,
      })
      if (!saved.success) {
        session.fail(current, 'The coverage result does not match the requested scenario.')
        return
      }
      onRun(saved.data)
      session.finish(current)
    } catch (error) {
      if (session.current(current))
        session.fail(current, error instanceof Error ? error.message : String(error))
    }
  }
  useRecordedAnalysis({
    key: 'pretest-coverage',
    prepared: prepared.id,
    unavailable: readiness,
    binding: JSON.stringify({ prepared: prepared.id, settings, source: candidate }),
    inspection: {
      section: 'Sensitivity',
      method: 'Pre-test coverage',
      specification: JSON.stringify({
        settings: pretestSettings(settings, candidates),
        scenario: request.ok ? request.value : null,
        readiness,
        assumptions: pretestMeaning,
      }),
      sharedResult:
        'Saved-run receipt. Read the sensitivity run for the coverage probabilities and scenario.',
    },
    session,
    execute,
    records: (workflow) =>
      workflow.sensitivityRuns.filter((r) => r.kind === 'pretest-coverage-run'),
    summarise: runReceipt,
  })
  const change = (patch: Partial<PretestSettings>) =>
    setSettings((previous) => ({ ...previous, ...patch }))
  const scenarioKind =
    settings.scenario.kind === 'specified'
      ? 'specified'
      : settings.scenario.offset === 0
        ? 'observed'
        : settings.scenario.offset === -1.96
          ? 'lower'
          : settings.scenario.offset === 1.96
            ? 'upper'
            : 'offset'
  return (
    <WorkbenchLayout
      id="sensitivity-pretest-coverage"
      stage={
        <section className="@container/panel flex flex-col gap-5">
          <div>
            <ChapterHeading>Sensitivity</ChapterHeading>
            <p className={chapterIntroSingle}>
              Compare interval coverage before and after conditioning on a pre-test under a
              specified linear-trend scenario.
            </p>
          </div>
          {selector}
          <section className={panel('p-(--panel-space)')}>
            <fieldset disabled={job.kind === 'running'} className="m-0 min-w-0 border-0 p-0">
              <div className={stepsStack}>
                <SettingsStep number={1} title="Choose the source run">
                  <label>
                    <span className={fieldLabel}>Source run</span>
                    <Select
                      className={field('text', 'mt-1')}
                      value={settings.sourceRun}
                      onChange={(e) =>
                        change({ sourceRun: e.target.value, coefficients: undefined })
                      }
                    >
                      <option value="">Choose run</option>
                      {candidates.map((c) => (
                        <option key={c.source.run} value={c.source.run}>
                          {c.label}, {formatTime(c.createdAt)}
                        </option>
                      ))}
                    </Select>
                  </label>
                  {candidate?.custom && (
                    <div className="grid gap-4 @md/panel:grid-cols-2">
                      {(['pre', 'post'] as const).map((key) => (
                        <label key={key} className="block">
                          <ParameterLabel
                            className={fieldLabel}
                            label={
                              key === 'pre'
                                ? 'Pre-treatment coefficient'
                                : 'Post-treatment coefficient'
                            }
                            help="The coefficient on the indicator for one event period. Do not choose a tail bin, a pooled effect or an unrelated predictor."
                          />
                          <Select
                            className={field('text', 'mt-1')}
                            aria-label={
                              key === 'pre'
                                ? 'Pre-treatment coefficient'
                                : 'Post-treatment coefficient'
                            }
                            value={settings.coefficients?.[key] ?? -1}
                            onChange={(e) =>
                              change({
                                coefficients: {
                                  pre: -1,
                                  post: -1,
                                  reference: -1,
                                  ...settings.coefficients,
                                  [key]: Number(e.target.value),
                                },
                              })
                            }
                          >
                            <option value={-1}>Choose coefficient</option>
                            {candidate.custom!.evidence.terms.map((t) => (
                              <option key={t.index} value={t.index}>
                                {t.name}
                              </option>
                            ))}
                          </Select>
                        </label>
                      ))}
                    </div>
                  )}
                  <div className="grid gap-4 @md/panel:grid-cols-2">
                    {(['prePeriod', 'postPeriod'] as const).map((key) => (
                      <label key={key}>
                        <ParameterLabel
                          className={fieldLabel}
                          label={
                            key === 'prePeriod' ? 'Pre-treatment period' : 'Post-treatment period'
                          }
                          help={
                            key === 'prePeriod'
                              ? 'The event time of the pre-treatment coefficient. The linear trend runs from the reference period through this period.'
                              : 'The event time of the post-treatment coefficient. The linear trend is extended to this period to set the target bias.'
                          }
                        />
                        {candidate?.custom ? (
                          <NumberInput
                            aria-label={
                              key === 'prePeriod' ? 'Pre-treatment period' : 'Post-treatment period'
                            }
                            className={field('text', 'mt-1')}
                            value={settings[key]}
                            onChange={(e) =>
                              change({
                                [key]: e.target.value.trim() === '' ? NaN : Number(e.target.value),
                              })
                            }
                          />
                        ) : (
                          <Select
                            className={field('text', 'mt-1')}
                            value={String(settings[key])}
                            onChange={(e) => change({ [key]: Number(e.target.value) })}
                          >
                            {candidate?.input.ok ? (
                              candidate.input.value.events
                                .filter((t) => (key === 'prePeriod' ? t < -1 : t >= 0))
                                .map((t) => (
                                  <option key={t} value={t}>
                                    {period(t)}
                                  </option>
                                ))
                            ) : (
                              <option value={settings[key]}>{period(settings[key])}</option>
                            )}
                          </Select>
                        )}
                      </label>
                    ))}
                  </div>
                  {candidate?.custom && (
                    <div className="grid gap-4 @md/panel:grid-cols-2">
                      <label className="block">
                        <ParameterLabel
                          className={fieldLabel}
                          label="Omitted reference period"
                          help="The event period whose indicator is left out of the regression. Both coefficients are measured relative to it."
                        />
                        <NumberInput
                          aria-label="Omitted reference period"
                          className={field('text', 'mt-1')}
                          value={settings.coefficients?.reference ?? -1}
                          onChange={(e) =>
                            change({
                              coefficients: {
                                pre: -1,
                                post: -1,
                                ...settings.coefficients,
                                reference:
                                  e.target.value.trim() === '' ? NaN : Number(e.target.value),
                              },
                            })
                          }
                        />
                      </label>
                    </div>
                  )}
                </SettingsStep>
                <SettingsStep number={2} title="Specify the trend scenario">
                  <div>
                    <ParameterLabel
                      className={fieldLabel}
                      label="Pre-trend bias scenario"
                      help="Calibration treats the recorded t-statistic as a hypothetical standardised bias. It does not estimate the true bias. Under linear drift, the target bias follows from the period distances and standard-error ratio."
                    />
                    <SegmentedControl
                      className="mt-1"
                      ariaLabel="Pre-trend bias scenario"
                      value={scenarioKind}
                      onChange={(next) =>
                        next === 'offset'
                          ? undefined
                          : change({
                              scenario:
                                next === 'specified'
                                  ? { kind: 'specified', bias: 0 }
                                  : {
                                      kind: 'calibrated',
                                      offset:
                                        next === 'lower' ? -1.96 : next === 'upper' ? 1.96 : 0,
                                    },
                            })
                      }
                      options={[
                        { value: 'observed', label: 'Recorded t-statistic' },
                        { value: 'lower', label: 'Recorded − 1.96' },
                        { value: 'upper', label: 'Recorded + 1.96' },
                        { value: 'specified', label: 'Specified bias' },
                        ...(scenarioKind === 'offset'
                          ? [{ value: 'offset' as const, label: 'Custom offset' }]
                          : []),
                      ]}
                    />
                  </div>
                  {(settings.scenario.kind === 'specified' || scenarioKind === 'offset') && (
                    <label className="block">
                      <ParameterLabel
                        className={fieldLabel}
                        label={
                          settings.scenario.kind === 'specified'
                            ? 'Bias in standard errors'
                            : 'Calibration offset'
                        }
                        help={
                          settings.scenario.kind === 'specified'
                            ? 'The assumed pre-trend bias, in standard errors of the pre-treatment estimate. The target bias follows from the linear trend.'
                            : 'Added to the recorded pre-trend t-statistic to give the assumed pre-trend bias.'
                        }
                      />
                      <NumberInput
                        aria-label={
                          settings.scenario.kind === 'specified'
                            ? 'Bias in standard errors'
                            : 'Calibration offset'
                        }
                        className={field('text', 'mt-1')}
                        step="any"
                        value={
                          settings.scenario.kind === 'specified'
                            ? settings.scenario.bias
                            : settings.scenario.offset
                        }
                        onChange={(e) =>
                          change({
                            scenario:
                              settings.scenario.kind === 'specified'
                                ? {
                                    kind: 'specified',
                                    bias:
                                      e.target.value.trim() === '' ? NaN : Number(e.target.value),
                                  }
                                : {
                                    kind: 'calibrated',
                                    offset:
                                      e.target.value.trim() === '' ? NaN : Number(e.target.value),
                                  },
                          })
                        }
                      />
                    </label>
                  )}
                </SettingsStep>
                <SettingsStep number={3} title="Set the interval and pre-test levels">
                  <div className="grid gap-4 @md/panel:grid-cols-2">
                    {(['confidence', 'pretestAlpha'] as const).map((key) => (
                      <label key={key} className="block">
                        <ParameterLabel
                          className={fieldLabel}
                          label={
                            key === 'confidence'
                              ? 'Confidence level'
                              : 'Pre-test significance level'
                          }
                          help="Enter a proportion strictly between zero and one. Both the interval and pre-test are two-sided normal calculations."
                        />
                        <NumberInput
                          aria-label={
                            key === 'confidence'
                              ? 'Confidence level'
                              : 'Pre-test significance level'
                          }
                          className={field('text', 'mt-1')}
                          step="any"
                          value={settings[key]}
                          onChange={(e) =>
                            change({
                              [key]: e.target.value.trim() === '' ? NaN : Number(e.target.value),
                            })
                          }
                        />
                      </label>
                    ))}
                  </div>
                </SettingsStep>
              </div>
            </fieldset>
            {readiness !== null && (
              <Alert tone="info" live={false} className="mt-4">
                <p className="m-0">{readiness}</p>
              </Alert>
            )}
            <RunActions
              className={actionGap}
              running={job.kind === 'running'}
              onCancel={session.cancel}
              orbLabel="Calculating coverage"
            >
              <button
                className={button('signal')}
                disabled={readiness !== null || session.blocked || job.kind === 'running'}
                onClick={() => void execute()}
              >
                Calculate coverage
              </button>
            </RunActions>
            <JobNotice job={job} />
          </section>
          {latest && <PretestCoverageResult run={latest} />}
        </section>
      }
      inspector={{
        trigger: { label: 'Requirements', icon: 'contract' },
        title: 'Pre-test coverage requirements',
        body: (
          <div className="space-y-3 text-body text-muted">
            <p>{pretestMeaning}</p>
            <p>
              The selected estimates must share a reference period and a sampling covariance. This
              event-study route uses period −1 as the reference. Custom regressions require an
              explicit shared reference. Binned regressions are supported, but the selected periods
              cannot be tail bins. Staggered estimates require zero anticipation and a universal
              baseline.
            </p>
            <p>
              The calculation assumes joint normal sampling errors and a specified differential
              linear trend. The ±1.96 offsets reproduce the paper’s calibration scenarios,
              regardless of the selected interval level. They are not bounds on actual coverage.
            </p>
            <p className="mb-2 mt-2 text-label text-faint">
              Literature: Is Inference Conditional on Not Rejecting a Pre-test Less Reliable than
              Unconditional Inference? (de Chaisemartin and D’Haultfœuille), SSRN Working Paper
              4975578, Section 3.3
            </p>
          </div>
        ),
      }}
      bottom={{
        trigger: { label: 'History', icon: 'history' },
        title: `Sensitivity runs (${runs.length})`,
        defaultSize: 150,
        body: (
          <>
            {[...runs].reverse().map((run) => (
              <RunFold
                key={run.id}
                title="Pre-test coverage"
                figure={
                  <RunMeta>
                    {[
                      `Without ${formatPercent(run.evidence.unconditional, { precision: 1 }).text}`,
                      `Passing ${formatPercent(run.evidence.conditional, { precision: 1 }).text}`,
                    ]}
                  </RunMeta>
                }
                stamp={
                  <RunMeta>{[scenarioLabel(run.settings), formatTime(run.createdAt)]}</RunMeta>
                }
                onDelete={() => setPendingDelete(run)}
                deleteLabel="Delete this coverage run"
                actions={
                  <PutOnDesk iconSize={13} deskRef={{ kind: 'sensitivity', id: String(run.id) }} />
                }
              >
                <PretestCoverageRecord run={run} />
              </RunFold>
            ))}
            <ConfirmDialog
              open={pendingDelete !== null}
              title="Delete this coverage run?"
              message="This removes the coverage calculation, not its source event study."
              confirmLabel="Delete run"
              danger
              onConfirm={() => {
                if (pendingDelete !== null) onDelete(pendingDelete.id)
              }}
              onClose={() => setPendingDelete(null)}
            />
          </>
        ),
      }}
    />
  )
}
