import { useMemo, useState } from 'react'
import type { SunAbrahamEvidence } from '@/domain/remixExtensions'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { baseOption, valueAxis, gridAuto, tooltip } from '@/charts/grammar'
import { formatStatistic } from '@/lib/format/number'
import { periodLabel } from '@/domain/periodLabels'

export function SunAbrahamResult({
  evidence,
  sourcePeriods,
}: {
  readonly evidence: SunAbrahamEvidence
  readonly sourcePeriods: readonly { readonly code: number; readonly label: string }[]
}) {
  const [view, setView] = useState<'event' | 'cohort' | 'cells'>('event')
  const theme = useChartTheme()
  const labels = useMemo(
    () => new Map(sourcePeriods.map((p) => [p.code, p.label])),
    [sourcePeriods],
  )
  const option = useMemo(() => {
    const effects = evidence.events.map((e) => ({
      key: e.key,
      estimate: e.interval.estimate,
      lower: e.interval.lower,
      upper: e.interval.upper,
      reference: false,
    }))
    for (const key of evidence.request.referencePeriods)
      effects.push({ key, estimate: 0, lower: 0, upper: 0, reference: true })
    effects.sort((a, b) => a.key - b.key)
    const bars: (number[] | null)[] = effects.flatMap((e) =>
      e.reference ? [] : [[e.key, e.lower], [e.key, e.upper], null],
    )
    return {
      ...baseOption(
        theme,
        'Sun–Abraham event-time ATT with pointwise intervals. Open circles mark normalized reference periods, not estimated zero effects.',
      ),
      grid: gridAuto({ top: 26, bottom: 28 }),
      tooltip: tooltip(theme, 'item'),
      xAxis: { ...valueAxis(theme, 'Periods since adoption'), minInterval: 1 },
      yAxis: valueAxis(theme, 'ATT'),
      series: [
        {
          id: 'intervals',
          type: 'line',
          data: bars,
          symbol: 'none',
          connectNulls: false,
          lineStyle: { color: theme.signal, width: 1.5 },
          silent: true,
        },
        {
          id: 'effects',
          name: 'ATT',
          type: 'scatter',
          symbolSize: 7,
          data: effects.map((e) => ({
            name: e.reference ? 'Normalized reference' : 'ATT',
            value: [e.key, e.estimate],
            symbol: e.reference ? 'emptyCircle' : 'circle',
            itemStyle: { color: e.reference ? theme.muted : theme.signal },
          })),
          markLine: {
            silent: true,
            symbol: 'none',
            label: { show: false },
            lineStyle: { color: theme.muted, type: 'dashed' },
            data: [{ yAxis: 0 }, { xAxis: 0 }],
          },
        },
      ],
    }
  }, [evidence, theme])
  const rows =
    view === 'cells'
      ? evidence.cells.map((c) => ({
          key: `${c.cohort}:${c.event}`,
          label: `${periodLabel(c.cohort, labels)} / ${c.event}`,
          support: c.support,
          interval: c.interval,
        }))
      : (view === 'event' ? evidence.events : evidence.cohorts).map((c) => ({
          key: String(c.key),
          label: view === 'event' ? String(c.key) : periodLabel(c.key, labels),
          support: c.support,
          interval: c.interval,
        }))
  const number = (v: string | number | null) =>
    typeof v === 'number' ? formatStatistic('raw', v).text : v
  return (
    <div className="mt-3 grid grid-cols-[minmax(0,1fr)] gap-3" data-testid="sun-abraham-result">
      <p className="m-0 text-body text-muted">
        {evidence.observations} retained observations; {evidence.clusters} independent unit
        clusters. {Math.round(evidence.request.confidence * 100)}% pointwise intervals.
      </p>
      <p className="m-0 text-body text-muted">
        {evidence.request.referenceCohorts.length === 0
          ? 'Reference group: never-treated units.'
          : 'Reference adoption cohorts: ' +
            evidence.request.referenceCohorts.map((c) => periodLabel(c, labels)).join(', ') +
            '. Never-treated units, when present, also remain references. Reference cohorts stay in the fit and are excluded from the reported ATT average.'}
      </p>
      <ExpandableChart
        option={option}
        label="Sun–Abraham event study"
        testId="sun-abraham-events"
        className="h-[300px]"
      />
      <SegmentedControl
        variant="line"
        ariaLabel="Sun–Abraham results"
        value={view}
        onChange={setView}
        options={[
          { value: 'event', label: 'Event study' },
          { value: 'cohort', label: 'Cohorts' },
          { value: 'cells', label: 'Cohort × event time' },
        ]}
      />
      <EvidenceTable
        title="Treatment effects"
        rows={rows}
        rowKey={(r) => r.key}
        noun="effect"
        empty="No supported effects."
        exportName="sun-abraham-effects"
        columns={[
          {
            id: 'key',
            header:
              view === 'event'
                ? 'Event time'
                : view === 'cohort'
                  ? 'Cohort'
                  : 'Cohort / event time',
            value: (r) => r.label,
          },
          {
            id: 'estimate',
            header: 'ATT',
            align: 'right',
            value: (r) => r.interval.estimate,
            format: number,
          },
          {
            id: 'se',
            header: 'Standard error',
            align: 'right',
            value: (r) => r.interval.standardError,
            format: number,
          },
          {
            id: 'lower',
            header: 'Lower',
            align: 'right',
            value: (r) => r.interval.lower,
            format: number,
          },
          {
            id: 'upper',
            header: 'Upper',
            align: 'right',
            value: (r) => r.interval.upper,
            format: number,
          },
          {
            id: 'support',
            header: 'Treated observations',
            align: 'right',
            value: (r) => r.support,
          },
        ]}
      />
      {evidence.removedRows.length > 0 && (
        <p className="m-0 text-body text-muted">
          {evidence.removedRows.length} observations excluded from units treated throughout their
          observed window.
        </p>
      )}
      {evidence.omitted.length > 0 && (
        <EvidenceTable
          title="Absorbed interactions"
          rows={evidence.omitted.map(([cohort, event]) => ({
            key: `${cohort}:${event}`,
            cohort,
            event,
          }))}
          rowKey={(r) => r.key}
          noun="interaction"
          empty="No absorbed interactions."
          columns={[
            { id: 'cohort', header: 'Cohort', value: (r) => periodLabel(r.cohort, labels) },
            { id: 'event', header: 'Event time', value: (r) => r.event },
          ]}
        />
      )}
    </div>
  )
}
