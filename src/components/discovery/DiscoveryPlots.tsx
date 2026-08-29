import { Select } from '@/components/ui/Select'
import { useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { edgeStrengthBarsOption } from '@/charts/discovery/edgeStrengthBars'
import { matrixHeatmapOption } from '@/charts/discovery/matrixHeatmap'
import { useChartTheme } from '@/charts/theme'
import { field, label } from '@/components/ui/recipes'
import type { DiscoveryRunArtifact } from '@/domain/discovery'
import { lagGraphFromRun } from '@/domain/lagGraph'
import { LagGraphViews } from './LagGraphViews'

type TimeGraphRun = Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' | 'lpcmci-run' }>
type WeightRun = Extract<DiscoveryRunArtifact, { readonly kind: 'dynotears-run' | 'var-lingam-run' }>
type OcseRun = Extract<DiscoveryRunArtifact, { readonly kind: 'ocse-run' }>

function LagSelect({ lags, value, onChange, contemporaneous }: { readonly lags: number; readonly value: number; readonly onChange: (lag: number) => void; readonly contemporaneous: boolean }) {
  const options = Array.from({ length: lags + (contemporaneous ? 1 : 0) }, (_, index) => (contemporaneous ? index : index + 1))
  return (
    <label className="flex items-center gap-2 text-label text-muted">
      Lag
      <Select className={field('text', 'w-auto py-0.5')} value={value} onChange={(event) => onChange(Number(event.target.value))} aria-label="Plotted lag">
        {options.map((lag) => <option key={lag} value={lag}>{lag === 0 ? 't (contemporaneous)' : `t−${lag}`}</option>)}
      </Select>
    </label>
  )
}

/** ParCorr values as a source-by-target heatmap at one lag, with the graph mark drawn in each reported cell. */
export function TimeGraphPlot({ run }: { readonly run: TimeGraphRun }) {
  const theme = useChartTheme()
  const reportedLags = run.result.graph.flatMap((targets) => targets.flatMap((lags) => lags.map((mark, index) => (mark.length > 0 ? index : -1)))).filter((index) => index >= 0)
  const [lag, setLag] = useState(reportedLags.length > 0 ? Math.min(...reportedLags) : 0)
  const names = run.variables.map((variable) => variable.name)
  const option = useMemo(() => matrixHeatmapOption({
    title: `${run.kind === 'lpcmci-run' ? 'LPCMCI' : 'PCMCI+'} partial correlations at lag ${lag}`,
    sources: names,
    targets: names,
    values: run.result.valMatrix.map((targets, source) => targets.map((lags, target) => (run.result.graph[source][target][lag].length > 0 ? lags[lag] : null))),
    scale: 'signed',
    quantity: 'ParCorr',
    cellText: (source, target) => run.result.graph[source][target][lag] || null,
  }, theme), [lag, names, run, theme])
  return (
    <div className="mt-3 rounded-lg border border-hair bg-well p-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className={label('m-0 text-faint')}>Reported links · {run.kind === 'lpcmci-run' ? 'PAG marks' : 'lag-graph marks'}</p>
        <LagSelect lags={run.result.tauMax} value={lag} onChange={setLag} contemporaneous />
      </div>
      <EChart option={option} label={`Partial correlation heatmap at lag ${lag}`} className="h-[clamp(220px,34cqb,320px)]" />
    </div>
  )
}

/** Fitted weights as a source-by-target heatmap at one lag. */
export function WeightPlot({ run }: { readonly run: WeightRun }) {
  const theme = useChartTheme()
  const [lag, setLag] = useState(0)
  const names = run.variables.map((variable) => variable.name)
  const matrices = [run.result.contemporaneousWeights, ...run.result.laggedWeights]
  const option = useMemo(() => matrixHeatmapOption({
    title: `${run.kind === 'var-lingam-run' ? 'VAR-LiNGAM' : 'DYNOTEARS'} weights at lag ${lag}`,
    sources: names,
    targets: names,
    values: (matrices[lag] ?? []).map((targets) => targets.map((weight) => (weight === 0 ? null : weight))),
    scale: 'signed',
    quantity: 'weight',
  }, theme), [lag, matrices, names, run.kind, theme])
  return (
    <div className="mt-3 rounded-lg border border-hair bg-well p-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className={label('m-0 text-faint')}>Nonzero weights · source(t−lag) → target(t)</p>
        <LagSelect lags={run.result.laggedWeights.length} value={lag} onChange={setLag} contemporaneous />
      </div>
      <EChart option={option} label={`Weight heatmap at lag ${lag}`} className="h-[clamp(220px,34cqb,320px)]" />
    </div>
  )
}

/** Selected oCSE edges ranked by conditional mutual information. */
export function OcsePlot({ run }: { readonly run: OcseRun }) {
  const theme = useChartTheme()
  const option = useMemo(() => edgeStrengthBarsOption({
    title: 'oCSE selected relations',
    quantity: 'CMI',
    edges: run.result.edges.map((edge) => ({
      name: `${run.variables[edge.source].name}(t−${edge.lag}) → ${run.variables[edge.target].name}`,
      strength: edge.cmi,
      pValue: edge.pValue,
    })),
  }, theme), [run, theme])
  if (run.result.edges.length === 0) return null
  return (
    <div className="mt-3 rounded-lg border border-hair bg-well p-3">
      <p className={label('m-0 text-faint')}>Selected relations by conditional mutual information</p>
      <EChart option={option} label="Selected oCSE relations ranked by CMI" className="h-[clamp(160px,28cqb,280px)]" />
    </div>
  )
}

/** The run's structure as tigramite's two views: the summary graph and the lag grid. */
export function StructurePlot({ run, label: name }: { readonly run: DiscoveryRunArtifact; readonly label: string }) {
  const graph = useMemo(() => lagGraphFromRun(run), [run])
  if (!graph.ok) return <p className="mt-3 text-body text-warn">The run reported a link mark Hirmos cannot draw ({graph.error.mark}); the table below is complete.</p>
  if (graph.value.links.length === 0) return <p className="mt-3 text-body text-faint">No link to draw: the run reported no relation.</p>
  return (
    <div className="mt-3">
      <p className={label('mb-1 text-faint')}>Structure · {graph.value.semantics.replaceAll('-', ' ')}</p>
      <LagGraphViews graph={graph.value} label={name} />
    </div>
  )
}

