import { useMemo, useState } from 'react'
import { periodLabel } from '@/domain/periodLabels'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import {
  baseOption,
  categoryAxis,
  gridAuto,
  rangeSelection,
  tooltip,
  valueAxis,
} from '@/charts/grammar'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Alert } from '@/components/ui/Alert'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { formatStatistic } from '@/lib/format/number'
import {
  recordedStaggeredAdjustment,
  staggeredAdjustmentDescriptions,
  type StaggeredEvidence,
  type StaggeredFamily,
  type StaggeredInterval,
} from '@/domain/staggeredDid'

const number = (value: number) => formatStatistic('raw', value).text
function Effects({
  family,
  title,
  mode,
  labels,
  confidence,
}: {
  readonly family: StaggeredFamily
  readonly title: string
  readonly mode: 'event' | 'calendar' | 'cohort'
  readonly labels: ReadonlyMap<number, string>
  readonly confidence: number
}) {
  const theme = useChartTheme()
  const coverage = `${Math.round(confidence * 100)}% ${family.coverage.kind === 'simultaneous' ? 'simultaneous bands' : 'pointwise intervals'}`
  const option = useMemo(() => {
    const horizontal = mode === 'cohort'
    const bars: (number[] | null)[] = [],
      points: { name: string; value: number[]; symbol: string; itemStyle: { color: string } }[] = []
    family.keys.forEach((key, index) => {
      const interval = family.intervals[index]!
      const estimate = interval.kind === 'reference' ? 0 : interval.estimate
      const x = horizontal ? index : key
      points.push({
        name:
          interval.kind === 'reference'
            ? 'Normalized reference'
            : interval.kind === 'unavailable'
              ? 'ATT (uncertainty unavailable)'
              : 'ATT',
        value: horizontal ? [estimate, x] : [x, estimate],
        symbol: interval.kind === 'reference' ? 'emptyCircle' : 'circle',
        itemStyle: { color: interval.kind === 'reference' ? theme.muted : theme.signal },
      })
      if (interval.kind === 'estimated')
        bars.push(
          horizontal ? [interval.lower, x] : [x, interval.lower],
          horizontal ? [interval.upper, x] : [x, interval.upper],
          null,
        )
    })
    return {
      ...baseOption(
        theme,
        `${title}. ${coverage}. Open circles are normalized reference periods, not estimated zero effects.`,
      ),
      grid: gridAuto({ top: 26, bottom: 28 }),
      tooltip: tooltip(theme, 'item'),
      xAxis: horizontal
        ? valueAxis(theme, 'ATT')
        : {
            ...valueAxis(theme, mode === 'event' ? 'Periods since adoption' : 'Period'),
            min: Math.min(...family.keys) - 0.5,
            max: Math.max(...family.keys) + 0.5,
            minInterval: 1,
            axisLabel: {
              ...valueAxis(theme).axisLabel,
              formatter: (v: number) =>
                !Number.isInteger(v) ? '' : mode === 'calendar' ? (labels.get(v) ?? '') : String(v),
            },
          },
      yAxis: horizontal
        ? categoryAxis(
            theme,
            family.keys.map((k) => periodLabel(k, labels)),
          )
        : valueAxis(theme, 'ATT'),
      series: [
        {
          id: 'intervals',
          name: coverage,
          type: 'line',
          data: bars,
          connectNulls: false,
          symbol: 'none',
          lineStyle: { color: theme.signal, width: 1.5 },
          silent: true,
        },
        {
          id: 'effects',
          name: 'ATT',
          type: 'scatter',
          data: points,
          symbolSize: 7,
          markLine: {
            silent: true,
            symbol: 'none',
            label: { show: false },
            lineStyle: { color: theme.muted, type: 'dashed' },
            data: [
              horizontal ? { xAxis: 0 } : { yAxis: 0 },
              ...(mode === 'event' ? [{ xAxis: 0 }] : []),
            ],
          },
        },
      ],
    }
  }, [family, title, mode, labels, theme, coverage])
  return (
    <div className="grid grid-cols-[minmax(0,1fr)] gap-2">
      <p className="m-0 text-body text-muted">{coverage}</p>
      <ExpandableChart
        option={option}
        label={title}
        testId={`staggered-${mode}`}
        className="h-[300px]"
      />
      {family.coverage.kind === 'simultaneous' && family.coverage.largeCritical && (
        <Alert tone="warn" live={false}>
          The simultaneous critical value is at least 7. Review the comparison support and overlap.
        </Alert>
      )}
    </div>
  )
}

/** Group-time ATT as one figure: a panel per cohort, stacked on a shared calendar axis with one set of range controls, like the other stacked estimate figures. */
function GroupTimeEffects({
  facets,
  labels,
  confidence,
}: {
  readonly facets: readonly { readonly cohort: number; readonly family: StaggeredFamily }[]
  readonly labels: ReadonlyMap<number, string>
  readonly confidence: number
}) {
  const theme = useChartTheme()
  const first = facets[0]?.family
  const coverage =
    first === undefined
      ? ''
      : `${Math.round(confidence * 100)}% ${first.coverage.kind === 'simultaneous' ? 'simultaneous bands' : 'pointwise intervals'}`
  const PANEL = 150,
    GAP = 36,
    TOP = 24,
    BOTTOM = 100
  const height = TOP + facets.length * (PANEL + GAP) - GAP + BOTTOM
  const option = useMemo(() => {
    const keys = facets.flatMap((f) => f.family.keys)
    const min = Math.min(...keys) - 0.5,
      max = Math.max(...keys) + 0.5
    const last = facets.length - 1
    const panels = facets.map((f) => `Cohort ${periodLabel(f.cohort, labels)}`)
    return {
      ...baseOption(
        theme,
        `Group-time ATT. ${coverage}. One panel per adoption cohort on a shared calendar axis. Open circles are normalized reference periods, not estimated zero effects.`,
      ),
      grid: facets.map((_, i) => ({
        left: 72,
        right: 18,
        top: TOP + i * (PANEL + GAP),
        height: PANEL,
      })),
      axisPointer: { link: [{ xAxisIndex: 'all' }] },
      tooltip: tooltip(theme, 'item'),
      xAxis: facets.map((_, i) => ({
        ...valueAxis(theme, i === last ? 'Period' : ''),
        gridIndex: i,
        min,
        max,
        minInterval: 1,
        axisLabel: {
          ...valueAxis(theme).axisLabel,
          show: i === last,
          formatter: (v: number) => (Number.isInteger(v) ? (labels.get(v) ?? '') : ''),
        },
      })),
      yAxis: panels.map((name, i) => ({ ...valueAxis(theme, name), gridIndex: i })),
      ...rangeSelection(
        theme,
        facets.map((_, i) => i),
      ),
      series: facets.flatMap((f, i) => {
        const bars: (number[] | null)[] = [],
          points: {
            name: string
            value: number[]
            symbol: string
            itemStyle: { color: string }
          }[] = []
        f.family.keys.forEach((key, index) => {
          const interval = f.family.intervals[index]!
          const estimate = interval.kind === 'reference' ? 0 : interval.estimate
          points.push({
            name:
              interval.kind === 'reference'
                ? 'Normalized reference'
                : interval.kind === 'unavailable'
                  ? 'ATT (uncertainty unavailable)'
                  : 'ATT',
            value: [key, estimate],
            symbol: interval.kind === 'reference' ? 'emptyCircle' : 'circle',
            itemStyle: { color: interval.kind === 'reference' ? theme.muted : theme.signal },
          })
          if (interval.kind === 'estimated')
            bars.push([key, interval.lower], [key, interval.upper], null)
        })
        return [
          {
            id: `intervals-${f.cohort}`,
            name: coverage,
            type: 'line',
            xAxisIndex: i,
            yAxisIndex: i,
            data: bars,
            connectNulls: false,
            symbol: 'none',
            lineStyle: { color: theme.signal, width: 1.5 },
            silent: true,
          },
          {
            id: `effects-${f.cohort}`,
            name: panels[i],
            type: 'scatter',
            xAxisIndex: i,
            yAxisIndex: i,
            data: points,
            symbolSize: 7,
            markLine: {
              silent: true,
              symbol: 'none',
              label: { show: false },
              lineStyle: { color: theme.muted, type: 'dashed' },
              data: [{ yAxis: 0 }],
            },
          },
        ]
      }),
    }
  }, [facets, labels, theme, coverage])
  return (
    <div className="grid grid-cols-[minmax(0,1fr)] gap-2">
      <p className="m-0 text-body text-muted">{coverage}</p>
      <ExpandableChart
        option={option}
        label="Group-time ATT"
        testId="staggered-cells"
        style={{ height }}
        className=""
      />
      {facets.some(
        (f) => f.family.coverage.kind === 'simultaneous' && f.family.coverage.largeCritical,
      ) && (
        <Alert tone="warn" live={false}>
          The simultaneous critical value is at least 7. Review the comparison support and overlap.
        </Alert>
      )}
    </div>
  )
}

export function StaggeredDidResult({
  evidence,
  labels,
  sourcePeriods,
}: {
  readonly evidence: StaggeredEvidence
  readonly labels: readonly string[]
  readonly sourcePeriods: readonly { readonly code: number; readonly label: string }[]
}) {
  const [view, setView] = useState<'event' | 'cohort' | 'calendar' | 'cells'>('event')
  const periods = useMemo(
    () => new Map(sourcePeriods.map((p) => [p.code, p.label] as const)),
    [evidence.times, labels, sourcePeriods],
  )
  const family =
    view === 'cohort' ? evidence.cohorts : view === 'calendar' ? evidence.calendar : evidence.events
  const name =
    view === 'cohort'
      ? 'Cohort-average effects'
      : view === 'calendar'
        ? 'Calendar-average effects'
        : 'Event-study effects'
  const facets = useMemo(
    () =>
      [...new Set(evidence.cells.keys.map(([g]) => g))].map((cohort) => {
        const indices = evidence.cells.keys.flatMap(([g], i) => (g === cohort ? [i] : []))
        return {
          cohort,
          family: {
            keys: indices.map((i) => evidence.cells.keys[i]![1]),
            intervals: indices.map((i) => evidence.cells.intervals[i]!),
            coverage: evidence.cells.coverage,
            analyticalCovariance: indices.map((i) =>
              indices.map((j) => evidence.cells.analyticalCovariance[i]![j]!),
            ),
          },
        }
      }),
    [evidence.cells],
  )
  const rows =
    view === 'cells'
      ? evidence.cells.keys.map(([g, t], i) => ({
          key: `${g}:${t}`,
          label: `${periodLabel(g, periods)} / ${periodLabel(t, periods)}`,
          interval: evidence.cells.intervals[i]!,
          cohorts: 0,
          units: 0,
        }))
      : family.keys.map((key, i) => ({
          key: String(key),
          label: view === 'event' ? String(key) : periodLabel(key, periods),
          interval: family.intervals[i]!,
          cohorts: evidence.support[i]?.cohorts.length ?? 0,
          units: evidence.support[i]?.treatedUnits ?? 0,
        }))
  const field = (
    interval: StaggeredInterval,
    key: 'estimate' | 'lower' | 'upper' | 'standardError',
  ) =>
    interval.kind === 'reference'
      ? key === 'estimate'
        ? 'Reference'
        : 'Not estimated'
      : interval.kind === 'unavailable'
        ? key === 'estimate'
          ? interval.estimate
          : 'Unavailable'
        : interval[key]
  const print = (value: string | number) => (typeof value === 'number' ? number(value) : value)
  const summaryColumns = [
    {
      id: 'estimate',
      header: 'ATT',
      align: 'right' as const,
      value: (r: { readonly interval: StaggeredInterval }) => field(r.interval, 'estimate'),
      format: print,
    },
    {
      id: 'se',
      header: 'Standard error',
      align: 'right' as const,
      value: (r: { readonly interval: StaggeredInterval }) => field(r.interval, 'standardError'),
      format: print,
    },
    {
      id: 'lower',
      header: 'Lower',
      align: 'right' as const,
      value: (r: { readonly interval: StaggeredInterval }) => field(r.interval, 'lower'),
      format: print,
    },
    {
      id: 'upper',
      header: 'Upper',
      align: 'right' as const,
      value: (r: { readonly interval: StaggeredInterval }) => field(r.interval, 'upper'),
      format: print,
    },
  ]
  const aggregate =
    view === 'cohort'
      ? {
          title: 'Overall cohort ATT',
          interval: evidence.overall.group,
          description: `Average of supported cohort-average ATT estimates, weighted by ${evidence.weighted ? 'cohort observation weights' : 'cohort sizes'}.`,
        }
      : view === 'calendar'
        ? {
            title: 'Overall calendar ATT',
            interval: evidence.overall.calendar,
            description: 'Average of supported calendar-period ATT estimates.',
          }
        : null
  return (
    <div className="mt-3 grid grid-cols-[minmax(0,1fr)] gap-3" data-testid="staggered-did-result">
      <p className="m-0 text-body text-muted" data-testid="staggered-adjustment-result">
        {
          staggeredAdjustmentDescriptions[recordedStaggeredAdjustment(evidence.specification).kind]
            .label
        }
        .{' '}
        {evidence.covariates === 0
          ? 'No adjustment covariates; comparisons use outcome changes.'
          : staggeredAdjustmentDescriptions[
              recordedStaggeredAdjustment(evidence.specification).kind
            ].description}
      </p>
      <p className="m-0 text-body text-muted">
        {evidence.clusterCount} independent clusters across {evidence.units.length} retained panel
        units.
      </p>
      <section className="grid gap-2" aria-label="Simple ATT summary">
        <p className="m-0 text-body text-muted">
          Weighted average of supported post-treatment group-time ATT estimates, using{' '}
          {evidence.weighted ? 'cohort observation weights' : 'cohort sizes'}. The headline instead
          averages supported event-time ATT estimates. Bounds are{' '}
          {Math.round(evidence.specification.confidence * 100)}% pointwise confidence intervals.
        </p>
        <EvidenceTable
          title="Simple ATT"
          rows={[{ key: 'simple', interval: evidence.overall.simple }]}
          rowKey={(r) => r.key}
          noun="estimate"
          empty="No supported estimate."
          exportName="staggered-did-simple-att"
          columns={summaryColumns}
        />
      </section>
      <SegmentedControl
        variant="line"
        ariaLabel="Staggered DiD results"
        value={view}
        onChange={setView}
        options={[
          { value: 'event', label: 'Event study' },
          { value: 'cohort', label: 'Cohorts' },
          { value: 'calendar', label: 'Calendar' },
          { value: 'cells', label: 'Group-time' },
        ]}
      />
      {aggregate !== null && (
        <section className="grid gap-2" aria-label={aggregate.title}>
          <p className="m-0 text-body text-muted">
            {aggregate.description} Bounds are {Math.round(evidence.specification.confidence * 100)}
            % pointwise confidence intervals.
          </p>
          <EvidenceTable
            title={aggregate.title}
            rows={[{ key: view, interval: aggregate.interval }]}
            rowKey={(r) => r.key}
            noun="estimate"
            empty="No supported estimate."
            exportName={`staggered-did-overall-${view}-att`}
            columns={summaryColumns}
          />
        </section>
      )}
      {view === 'cells' ? (
        <GroupTimeEffects
          facets={facets}
          labels={periods}
          confidence={evidence.specification.confidence}
        />
      ) : (
        <Effects
          family={family}
          title={name}
          mode={view}
          labels={periods}
          confidence={evidence.specification.confidence}
        />
      )}
      <EvidenceTable
        title={view === 'cells' ? 'Group-time ATT' : name}
        rows={rows}
        rowKey={(r) => r.key}
        noun="effect"
        empty="No supported effects."
        exportName="staggered-did-effects"
        columns={[
          {
            id: 'key',
            header:
              view === 'cells'
                ? 'Cohort / period'
                : view === 'event'
                  ? 'Event time'
                  : view === 'cohort'
                    ? 'Cohort'
                    : 'Period',
            value: (r) => r.label,
          },
          {
            id: 'estimate',
            header: 'ATT',
            align: 'right',
            value: (r) => field(r.interval, 'estimate'),
            format: print,
          },
          {
            id: 'se',
            header: 'Standard error',
            align: 'right',
            value: (r) => field(r.interval, 'standardError'),
            format: print,
          },
          {
            id: 'lower',
            header: 'Lower',
            align: 'right',
            value: (r) => field(r.interval, 'lower'),
            format: print,
          },
          {
            id: 'upper',
            header: 'Upper',
            align: 'right',
            value: (r) => field(r.interval, 'upper'),
            format: print,
          },
          ...(view === 'event'
            ? [
                {
                  id: 'cohorts',
                  header: 'Cohorts',
                  align: 'right' as const,
                  value: (r: (typeof rows)[number]) => r.cohorts,
                },
                {
                  id: 'units',
                  header: 'Treated units',
                  align: 'right' as const,
                  value: (r: (typeof rows)[number]) => r.units,
                },
              ]
            : []),
        ]}
      />
      {evidence.changes.length > 0 && (
        <EvidenceTable
          title="Panel preparation"
          rows={evidence.changes.map((c, i) => ({ key: String(i), ...c }))}
          rowKey={(r) => r.key}
          noun="change"
          empty="No changes."
          columns={[
            {
              id: 'record',
              header: 'Unit or period',
              value: (r) => ('unit' in r ? r.unit : periodLabel(r.period, periods)),
            },
            {
              id: 'change',
              header: 'Change',
              value: (r) =>
                ({
                  beyondObservedWindow: 'Adoption beyond observed window; used as comparison',
                  latestCohortAsComparison: 'Latest cohort used as comparison',
                  noUntreatedComparison: 'Period removed: no untreated comparison',
                  noPreTreatment: 'Unit removed: no untreated baseline',
                })[r.kind],
            },
          ]}
        />
      )}
      {evidence.smallCohorts.length > 0 && (
        <Alert tone="warn" live={false}>
          At least one cohort has fewer than the reference requirement of covariate count plus five
          units. Review the cohort support.
        </Alert>
      )}
      {evidence.fits.some((f) => f.status.kind === 'iterationLimit') && (
        <Alert tone="warn" live={false}>
          At least one propensity fit reached its iteration limit. Review the covariates and
          treatment overlap.
        </Alert>
      )}
      {evidence.fits.length > 0 && (
        <details>
          <DisclosureSummary className="cursor-pointer text-body text-ink">
            Propensity fits
          </DisclosureSummary>
          <EvidenceTable
            title="Propensity fits"
            rows={evidence.fits}
            rowKey={(r) => `${r.cohort}:${r.period}`}
            noun="fit"
            empty="No adjustment models."
            columns={[
              { id: 'cohort', header: 'Cohort', value: (r) => periodLabel(r.cohort, periods) },
              { id: 'period', header: 'Period', value: (r) => periodLabel(r.period, periods) },
              {
                id: 'status',
                header: 'Termination',
                value: (r) =>
                  r.status.kind === 'converged'
                    ? `Converged in ${r.status.iterations} iterations`
                    : 'Iteration limit',
              },
            ]}
          />
        </details>
      )}
    </div>
  )
}
