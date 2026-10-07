import { Alert } from '@/components/ui/Alert'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { NumberInput, numberValue } from '@/components/ui/NumberInput'
import { RadioList } from '@/components/ui/RadioList'
import { RunDetails } from '@/components/ui/RunDetails'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { RequirementsFold } from '@/components/MethodCaveats'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { formatStatistic, formatSetting, formatCount } from '@/lib/format/number'
import { Fragment, useState } from 'react'
import { calculatePower } from '@/analysis/client'
import {
  DEFAULT_POWER,
  powerRequestSchema,
  powerRecord,
  type PowerRequest,
  type PowerRecord,
} from '@/domain/powerPlanning'
import { useWorkflow } from '@/components/WorkflowProvider'
import { RunActions } from '@/components/ui/RunActions'
import { button, field, fieldLabel, literal, panel, prose } from '@/components/ui/recipes'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { gridAuto, valueAxis, stepAxis } from '@/charts/grammar'

const names = {
  independentT: 'Independent groups, pooled-variance t-test',
  oneSampleT: 'One-sample t-test',
  pairedT: 'Paired t-test',
  independentNormal: 'Independent groups, normal approximation',
  independent: 'Independent two-arm experiment',
  equalClusters: 'Equal-sized cluster experiment',
  twoPeriodDid: 'Two-period difference-in-differences',
} as const
const hints = {
  independentT:
    'This t-test calculation assumes independent observations and a common outcome variance.',
  oneSampleT:
    'This t-test calculation assumes independent observations and a common outcome variance.',
  pairedT: 'Standardize the mean difference by the standard deviation of within-pair differences.',
  independentNormal:
    'Use a standardized normal-test effect; for proportions this is Cohen’s h, not the raw difference.',
  independent: 'Independent Gaussian outcomes have a constant additive treatment effect.',
  equalClusters: 'Equal-sized clusters are analysed through their mean outcomes.',
  twoPeriodDid:
    'Independent units’ before–after changes are compared under the specified untreated trends.',
} as const
const figure = (x: number) => formatStatistic('raw', x)
const fmt = (x: number) => figure(x).text
const formatSize = (x: number) => (Number.isInteger(x) ? formatCount(x).text : fmt(x))
function NumberField({
  label,
  value,
  onChange,
}: {
  readonly label: string
  readonly value: number
  readonly onChange: (n: number) => void
}) {
  return (
    <label className="block">
      <span className={fieldLabel}>{label}</span>
      <NumberInput
        className={field('text', 'mt-1')}
        step="any"
        aria-label={label}
        value={value}
        onChange={(e) => onChange(numberValue(e.target))}
      />
    </label>
  )
}
function planningSettings(s: PowerRequest): readonly (readonly [string, string])[] {
  const n = (v: number) => formatSetting(v).text
  const common: [string, string][] = [['Significance level', n(s.alpha)]]
  if (s.kind === 'simulation')
    return [
      ...common,
      ['Design', names[s.scenario.kind]],
      [
        'Treated ' + (s.scenario.kind === 'equalClusters' ? 'clusters' : 'units'),
        n(s.scenario.treated),
      ],
      [
        'Control ' + (s.scenario.kind === 'equalClusters' ? 'clusters' : 'units'),
        n(s.scenario.controls),
      ],
      ['Treatment effect', n(s.effect)],
      ['Outcome standard deviation', n(s.sd)],
      ['Replications', n(s.replications)],
      ['Seed', n(s.seed)],
      ...(s.scenario.kind === 'equalClusters'
        ? ([
            ['Members per cluster', n(s.scenario.members)],
            ['Intracluster correlation', n(s.scenario.icc)],
          ] as [string, string][])
        : s.scenario.kind === 'twoPeriodDid'
          ? ([
              ['Before–after correlation', n(s.scenario.correlation)],
              ['Untreated trend difference', n(s.scenario.violation)],
            ] as [string, string][])
          : []),
    ]
  return [
    ...common,
    ['Design', names[s.design.kind]],
    [
      'Alternative',
      s.alternative === 'twoSided'
        ? 'Two-sided'
        : s.alternative === 'larger'
          ? 'Positive difference'
          : 'Negative difference',
    ],
    [
      'Solve for',
      s.target.kind === 'minimumEffect'
        ? 'Minimum detectable effect'
        : s.target.kind === 'sampleSize'
          ? 'Sample size'
          : 'Power',
    ],
    ...('ratio' in s.design
      ? ([['Group 2 / group 1 size ratio', n(s.design.ratio)]] as [string, string][])
      : []),
    ...('n' in s.target ? ([['First sample size', n(s.target.n)]] as [string, string][]) : []),
    ...('effect' in s.target
      ? ([['Standardized effect', n(s.target.effect)]] as [string, string][])
      : []),
    ...('target' in s.target ? ([['Target power', n(s.target.target)]] as [string, string][]) : []),
  ]
}
function Result({ record }: { readonly record: PowerRecord }) {
  const theme = useChartTheme()
  if (record.kind === 'simulation') {
    const { result: r, specification: s } = record
    const label =
      s.effect === 0
        ? 'Type-I rejection rate'
        : s.scenario.kind === 'twoPeriodDid' && s.scenario.violation !== 0
          ? 'Rejection rate under the specified trend violation'
          : 'Estimated power'
    return (
      <div className="grid gap-3" data-testid="power-result">
        <MetricGrid label="Simulation result">
          <MetricTile
            label={label}
            value={figure(r.rejectionRate)}
            context={`${formatSize(r.rejected)} rejections in ${formatSize(r.replications)} replications`}
          />
          <MetricTile label="Monte Carlo standard error" value={figure(r.mcse)} />
          <MetricTile label="Coverage" value={figure(r.coverage)} />
          <MetricTile label="Bias" value={figure(r.bias)} />
          <MetricTile label="RMSE" value={figure(r.rmse)} />
        </MetricGrid>
        <p className={prose('m-0 text-faint')}>
          The interval coverage is measured against the specified treatment effect. A zero Monte
          Carlo standard error when all or none of the tests reject does not establish certainty.
        </p>
      </div>
    )
  }
  const { result: r, specification: s } = record
  const heading =
    s.target.kind === 'minimumEffect'
      ? 'Minimum detectable standardized effect'
      : s.target.kind === 'sampleSize'
        ? 'Required first-group size'
        : 'Power'
  const value =
    s.target.kind === 'minimumEffect' ? r.effect : s.target.kind === 'sampleSize' ? r.n : r.power
  const count =
    s.design.kind === 'pairedT'
      ? 'pairs'
      : s.design.kind === 'oneSampleT'
        ? 'observations'
        : 'observations in group 1'
  const size = (x: number) => (Number.isInteger(x) ? formatCount(x) : figure(x))
  return (
    <div className="grid gap-3" data-testid="power-result">
      <MetricGrid label="Planning result">
        <MetricTile label={heading} value={size(value)} />
        <MetricTile
          label={count.charAt(0).toUpperCase() + count.slice(1)}
          value={size(r.n)}
          context={r.secondGroup === null ? undefined : `${formatSize(r.secondGroup)} in group 2`}
        />
        {s.target.kind !== 'power' && <MetricTile label="Power" value={figure(r.power)} />}
        {s.target.kind !== 'minimumEffect' && (
          <MetricTile label="Standardized effect" value={figure(r.effect)} />
        )}
      </MetricGrid>
      {r.continuousN !== null && (
        <p className={prose('m-0 text-faint')}>
          The continuous solution is {fmt(r.continuousN)}. Group counts are rounded upward and power
          is recalculated at those counts.
        </p>
      )}
      <ExpandableChart
        label="Power by sample size"
        testId="power-curve"
        className="h-64"
        option={{
          animation: false,
          grid: gridAuto({ top: 28, bottom: 42 }),
          textStyle: { fontFamily: theme.font, color: theme.ink },
          xAxis: {
            ...stepAxis(theme, 'Sample size (' + count + ')'),
            min: (extent: { readonly min: number }) => Math.floor(extent.min / 10) * 10,
          },
          yAxis: { ...valueAxis(theme, 'Power'), min: 0, max: 1 },
          tooltip: {
            trigger: 'axis',
            backgroundColor: theme.panel,
            textStyle: { color: theme.ink },
            valueFormatter: (power: unknown) => (typeof power === 'number' ? fmt(power) : ''),
          },
          series: [
            {
              type: 'line',
              name: 'Power',
              showSymbol: false,
              lineStyle: { color: theme.signal, width: 2 },
              data: r.curve.map((p) => [p.n, p.power]),
            },
          ],
        }}
      />
      <p className={prose('text-faint')}>
        Power is calculated across a range of sample sizes. The effect size, allocation ratio,
        significance level and test direction remain fixed.
      </p>
    </div>
  )
}
export function PowerPlanningPanel() {
  const record = useWorkflow((s) =>
    s.workflow.kind === 'profiled' ? s.workflow.powerPlanning : null,
  )
  const dispatch = useWorkflow((s) => s.dispatch)
  const [draft, setDraft] = useState<PowerRequest>(() => record?.specification ?? DEFAULT_POWER)
  const session = useJob('power-planning')
  const running = session.job.kind === 'running'
  const [problem, setProblem] = useState<string | null>(null)
  const set = (next: PowerRequest) => {
    setDraft(next)
    setProblem(null)
  }
  async function run() {
    const parsed = powerRequestSchema.safeParse(draft)
    if (!parsed.success) {
      setProblem(
        'Check the entered values. Sample sizes must be at least two, significance and target power must lie between zero and one, and simulation sizes and seeds must be whole numbers.',
      )
      return
    }
    const id = session.start('analysis', 'Sample size and power planning')
    if (id === null) return
    setProblem(null)
    try {
      const result = await calculatePower(parsed.data)
      if (!session.current(id)) return
      if (!result.ok) {
        session.fail(id, describeAnalysisWorkerProblem(result.error))
        return
      }
      const saved = powerRecord(parsed.data, result.value)
      if (saved === null) {
        session.fail(id, 'The result does not match the requested planning design.')
        return
      }
      dispatch({ type: 'power-planning-recorded', record: saved })
      session.finish(id)
    } catch (error) {
      session.fail(
        id,
        error instanceof Error ? error.message : 'The calculation could not be completed.',
      )
    }
  }
  const simulationDefault: PowerRequest = {
    kind: 'simulation',
    scenario: { kind: 'independent', treated: 100, controls: 100 },
    effect: 0.5,
    sd: 1,
    alpha: 0.05,
    replications: 1000,
    seed: 1,
  }
  const chooseDesign = (k: string) => {
    if (draft.kind === 'analytical') {
      if (k === 'oneSampleT' || k === 'pairedT') set({ ...draft, design: { kind: k } })
      else if (k === 'independentT' || k === 'independentNormal')
        set({ ...draft, design: { kind: k, ratio: 1 } })
    } else {
      const base = { treated: 100, controls: 100 }
      if (k === 'independent') set({ ...draft, scenario: { kind: k, ...base } })
      else if (k === 'equalClusters')
        set({ ...draft, scenario: { kind: k, treated: 20, controls: 20, members: 10, icc: 0.1 } })
      else if (k === 'twoPeriodDid')
        set({ ...draft, scenario: { kind: k, ...base, correlation: 0.5, violation: 0 } })
    }
  }
  const designs =
    draft.kind === 'analytical'
      ? (['independentT', 'oneSampleT', 'pairedT', 'independentNormal'] as const)
      : (['independent', 'equalClusters', 'twoPeriodDid'] as const)
  return (
    <details className={panel('px-4 py-3 text-body')} data-testid="power-planning">
      <DisclosureSummary icon="calculate" className="cursor-pointer text-ink">
        Sample size and power planning
      </DisclosureSummary>
      <div className="mt-3 grid gap-4">
        <p className={prose('m-0 text-muted')}>
          Plan sample size, power or a minimum detectable effect for a specified design.
        </p>
        <fieldset
          disabled={running || session.blocked}
          className="m-0 grid min-w-0 gap-4 border-0 p-0"
        >
          <div className="grid gap-1">
            <span className={fieldLabel}>Calculation</span>
            <SegmentedControl
              ariaLabel="Planning calculation"
              value={draft.kind}
              onChange={(kind) => set(kind === 'analytical' ? DEFAULT_POWER : simulationDefault)}
              options={[
                { value: 'analytical', label: 'Analytical' },
                { value: 'simulation', label: 'Simulation' },
              ]}
            />
          </div>
          <RadioList
            legend="Design"
            value={draft.kind === 'analytical' ? draft.design.kind : draft.scenario.kind}
            onChange={chooseDesign}
            options={designs.map((k) => ({ value: k, label: names[k], hint: hints[k] }))}
          />
          {draft.kind === 'analytical' ? (
            <>
              <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                <div className="grid gap-1">
                  <span className={fieldLabel}>Solve for</span>
                  <SegmentedControl
                    ariaLabel="Solve for"
                    value={draft.target.kind}
                    onChange={(k) => {
                      if (k === 'power') set({ ...draft, target: { kind: k, n: 100, effect: 0.5 } })
                      else if (k === 'minimumEffect')
                        set({ ...draft, target: { kind: k, n: 100, target: 0.8 } })
                      else set({ ...draft, target: { kind: k, effect: 0.5, target: 0.8 } })
                    }}
                    options={[
                      { value: 'minimumEffect', label: 'Minimum detectable effect' },
                      { value: 'power', label: 'Power' },
                      { value: 'sampleSize', label: 'Sample size' },
                    ]}
                  />
                </div>
                <div className="grid gap-1">
                  <span className={fieldLabel}>Alternative hypothesis</span>
                  <SegmentedControl
                    ariaLabel="Alternative hypothesis"
                    value={draft.alternative}
                    onChange={(alternative) => set({ ...draft, alternative })}
                    options={[
                      { value: 'twoSided', label: 'Two-sided' },
                      { value: 'larger', label: 'Positive difference' },
                      { value: 'smaller', label: 'Negative difference' },
                    ]}
                  />
                </div>
              </div>
              <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
                <NumberField
                  label="Significance level"
                  value={draft.alpha}
                  onChange={(alpha) => set({ ...draft, alpha })}
                />
                {'ratio' in draft.design && (
                  <NumberField
                    label="Group 2 / group 1 size ratio"
                    value={draft.design.ratio}
                    onChange={(ratio) => {
                      if ('ratio' in draft.design)
                        set({ ...draft, design: { ...draft.design, ratio } })
                    }}
                  />
                )}
                {'n' in draft.target && (
                  <NumberField
                    label={
                      draft.design.kind === 'pairedT'
                        ? 'Number of pairs'
                        : draft.design.kind === 'oneSampleT'
                          ? 'Sample size'
                          : 'Group 1 size'
                    }
                    value={draft.target.n}
                    onChange={(n) => {
                      if ('n' in draft.target) set({ ...draft, target: { ...draft.target, n } })
                    }}
                  />
                )}
                {'effect' in draft.target && (
                  <NumberField
                    label="Standardized effect"
                    value={draft.target.effect}
                    onChange={(effect) => {
                      if ('effect' in draft.target)
                        set({ ...draft, target: { ...draft.target, effect } })
                    }}
                  />
                )}
                {'target' in draft.target && (
                  <NumberField
                    label="Target power"
                    value={draft.target.target}
                    onChange={(target) => {
                      if ('target' in draft.target)
                        set({ ...draft, target: { ...draft.target, target } })
                    }}
                  />
                )}
              </div>
            </>
          ) : (
            <>
              <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
                <NumberField
                  label={
                    draft.scenario.kind === 'equalClusters' ? 'Treated clusters' : 'Treated units'
                  }
                  value={draft.scenario.treated}
                  onChange={(treated) =>
                    set({ ...draft, scenario: { ...draft.scenario, treated } })
                  }
                />
                <NumberField
                  label={
                    draft.scenario.kind === 'equalClusters' ? 'Control clusters' : 'Control units'
                  }
                  value={draft.scenario.controls}
                  onChange={(controls) =>
                    set({ ...draft, scenario: { ...draft.scenario, controls } })
                  }
                />
                <NumberField
                  label="Treatment effect"
                  value={draft.effect}
                  onChange={(effect) => set({ ...draft, effect })}
                />
                <NumberField
                  label="Outcome standard deviation"
                  value={draft.sd}
                  onChange={(sd) => set({ ...draft, sd })}
                />
                <NumberField
                  label="Significance level"
                  value={draft.alpha}
                  onChange={(alpha) => set({ ...draft, alpha })}
                />
                <NumberField
                  label="Replications"
                  value={draft.replications}
                  onChange={(replications) => set({ ...draft, replications })}
                />
                <NumberField
                  label="Seed"
                  value={draft.seed}
                  onChange={(seed) => set({ ...draft, seed })}
                />
                {draft.scenario.kind === 'equalClusters' && (
                  <>
                    <NumberField
                      label="Members per cluster"
                      value={draft.scenario.members}
                      onChange={(members) => {
                        if (draft.scenario.kind === 'equalClusters')
                          set({ ...draft, scenario: { ...draft.scenario, members } })
                      }}
                    />
                    <NumberField
                      label="Intracluster correlation"
                      value={draft.scenario.icc}
                      onChange={(icc) => {
                        if (draft.scenario.kind === 'equalClusters')
                          set({ ...draft, scenario: { ...draft.scenario, icc } })
                      }}
                    />
                  </>
                )}
                {draft.scenario.kind === 'twoPeriodDid' && (
                  <>
                    <NumberField
                      label="Before–after correlation"
                      value={draft.scenario.correlation}
                      onChange={(correlation) => {
                        if (draft.scenario.kind === 'twoPeriodDid')
                          set({ ...draft, scenario: { ...draft.scenario, correlation } })
                      }}
                    />
                    <NumberField
                      label="Untreated trend difference"
                      value={draft.scenario.violation}
                      onChange={(violation) => {
                        if (draft.scenario.kind === 'twoPeriodDid')
                          set({ ...draft, scenario: { ...draft.scenario, violation } })
                      }}
                    />
                  </>
                )}
              </div>
            </>
          )}
        </fieldset>
        <RunActions
          running={running}
          orbLabel="Power calculation running"
          onCancel={session.cancel}
        >
          <button
            type="button"
            className={button('signal')}
            disabled={running || session.blocked}
            onClick={() => void run()}
          >
            Calculate planning result
          </button>
        </RunActions>
        <JobNotice job={session.job} />
        {problem !== null && (
          <Alert tone="danger">
            <p className="m-0">{problem}</p>
          </Alert>
        )}
        {record !== null && (
          <>
            {JSON.stringify(record.specification) !== JSON.stringify(draft) && (
              <Alert tone="info" live={false}>
                <p className="m-0">
                  Settings have changed. The result below still uses the recorded specification.
                </p>
              </Alert>
            )}
            <Result record={record} />
            <RunDetails label="Recorded planning specification">
              <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
                {planningSettings(record.specification).map(([label, value]) => (
                  <Fragment key={label}>
                    <dt>{label}</dt>
                    <dd className={literal('m-0')}>{value}</dd>
                  </Fragment>
                ))}
              </dl>
            </RunDetails>
          </>
        )}
        <RequirementsFold
          name="Planning assumptions"
          literature={[
            'statsmodels, power and sample-size calculations',
            'DeclareDesign, simulation diagnoses',
            'estimatr, reference regression fits',
          ]}
        >
          <div className="grid gap-2">
            <p className="m-0 text-body text-muted">
              Enter an effect worth detecting and justify the assumed outcome variation. Using the
              effect estimated from the same dataset gives retrospective observed power, not
              prospective planning. Planning does not establish causal identification.
            </p>
            <p className="m-0 text-body text-muted">
              Independent-group t-tests assume a common variance. Paired tests use the standard
              deviation of within-pair differences. The normal approximation is neither an exact
              binomial test nor a Welch test; for proportions, its standardized effect is Cohen’s h.
            </p>
            <p className="m-0 text-body text-muted">
              The simulation designs use Gaussian outcomes and classical common-variance inference.
              Cluster experiments require equal-sized clusters. Two-period DiD compares independent
              units’ before–after changes and allows a specified difference in untreated trends.
              Staggered adoption, covariate adjustment, attrition and general clustered panels are
              not represented.
            </p>
          </div>
        </RequirementsFold>
      </div>
    </details>
  )
}
