import { useState } from 'react'
import type { TimeSeriesRun } from '@/domain/timeSeries'
import type { PlotTime } from '@/domain/longRun'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { longRunOption } from '@/charts/data/longRun'
import { useChartTheme } from '@/charts/theme'
import type { VisibleWindow } from '@/charts/window'
import { Select } from '@/components/ui/Select'
import { field, fieldLabel, label } from '@/components/ui/recipes'

type Figure = { readonly title: string; readonly startRow: number; readonly series: readonly { readonly name: string; readonly values: readonly number[] }[]; readonly zero: boolean }

function Figures({ figures, axis }: { readonly figures: readonly Figure[]; readonly axis: PlotTime }) {
  const theme = useChartTheme()
  const [window, setWindow] = useState<VisibleWindow | null>(null)
  return <div className="grid min-w-0 gap-6" data-testid="long-run-charts">{figures.map((figure, i) => <div key={figure.title} className="min-w-0">
    <h4 className={label('m-0 text-faint')}>{figure.title}</h4>
    <ExpandableChart option={longRunOption({ ...figure, axis, slider: i === 0 }, theme)} label={figure.title} className="mt-1 h-72" window={window} onWindow={setWindow} />
  </div>)}</div>
}

function Relationships({ departures, startRow, axis }: { readonly departures: readonly (readonly number[])[]; readonly startRow: number; readonly axis: PlotTime }) {
  const [selected, setSelected] = useState(0)
  const values = departures[selected]!
  return <>
    {departures.length > 1 && <label><span className={fieldLabel}>Long-run relationship</span><Select className={field('text', 'mt-1')} value={String(selected)} onChange={(event) => setSelected(Number(event.target.value))}>{departures.map((_, i) => <option key={i} value={i}>Relationship {i + 1}</option>)}</Select></label>}
    <Figures axis={axis} figures={[{ title: `Departure from long-run relationship ${selected + 1}`, startRow, zero: true, series: [{ name: 'Departure', values }] }]} />
    <p className="m-0 text-label text-muted">Each value combines the series using the estimated equilibrium coefficients, including any constant or trend inside the relationship. Dates refer to the lagged observations used in the fit. The sign depends on how the relationship is normalised; it does not indicate whether an outcome is good or bad.</p>
  </>
}

export function LongRunCharts({ run }: { readonly run: Extract<TimeSeriesRun, { kind: 'ardl' | 'vecm' }> }) {
  if (run.plotTime === undefined || run.evidence.longRun === undefined) return null
  switch (run.kind) {
    case 'ardl': {
      const fit = run.evidence.longRun!
      const level = fit.observed.map((value, i) => value - fit.departures[i]!)
      return <>
        <Figures axis={run.plotTime} figures={[
          { title: 'Observed outcome and estimated long-run level', startRow: 0, zero: false, series: [{ name: run.outcome.name, values: fit.observed }, { name: 'Estimated long-run level', values: level }] },
          { title: 'Departure from the estimated long-run level', startRow: 0, zero: true, series: [{ name: 'Observed minus long-run level', values: fit.departures }] },
        ]} />
        <p className="m-0 text-label text-muted">The long-run level uses the predictor values and any constant or time trend in the fitted relationship. Positive departures mean {run.outcome.name} was above that level; negative departures mean it was below. This is not a forecast or an intervention comparison. Interpreting the relationship also requires support from the bounds test.</p>
      </>
    }
    case 'vecm': {
      const fit = run.evidence.longRun!
      if (fit.kind === 'notFitted' || run.evidence.rank === run.variables.length) return null
      return <Relationships departures={fit.departures} startRow={fit.startRow} axis={run.plotTime} />
    }
  }
}
