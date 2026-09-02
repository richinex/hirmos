import { Select } from '@/components/ui/Select'
import { useMemo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { EChart } from '@/charts/EChart'
import { edgeStrengthBarsOption } from '@/charts/discovery/edgeStrengthBars'
import { matrixHeatmapOption } from '@/charts/discovery/matrixHeatmap'
import { annealingObjectiveOption, regimeMembershipOption } from '@/charts/discovery/regimeMembership'
import { useChartTheme } from '@/charts/theme'
import { caption, field, well } from '@/components/ui/recipes'
import type { DiscoveryRunArtifact } from '@/domain/discovery'
import { lagGraphFromRun } from '@/domain/lagGraph'
import { LagGraphViews } from './LagGraphViews'

type TimeGraphRun = Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' | 'lpcmci-run' }>
type RpcmciRun = Extract<DiscoveryRunArtifact, { readonly kind: 'rpcmci-run' }>
type WeightRun = Extract<DiscoveryRunArtifact, { readonly kind: 'direct-lingam-run' | 'dynotears-run' | 'var-lingam-run' }>
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

function MarkedMatrixPlot({ graph, values, names, tauMax, title, caption: text }: {
  readonly graph: readonly (readonly (readonly string[])[])[]
  readonly values: readonly (readonly (readonly number[])[])[]
  readonly names: readonly string[]
  readonly tauMax: number
  readonly title: string
  readonly caption: string
}) {
  const theme = useChartTheme()
  const reportedLags = graph.flatMap((targets) => targets.flatMap((lags) => lags.map((mark, index) => (mark.length > 0 ? index : -1)))).filter((index) => index >= 0)
  const [lag, setLag] = useState(reportedLags.length > 0 ? Math.min(...reportedLags) : 0)
  const option = useMemo(() => matrixHeatmapOption({
    title: `${title} at lag ${lag}`,
    sources: names,
    targets: names,
    values: values.map((targets, source) => targets.map((lags, target) => (graph[source][target][lag].length > 0 ? lags[lag] : null))),
    scale: 'signed',
    quantity: 'ParCorr',
    cellText: (source, target) => graph[source][target][lag] || null,
  }, theme), [graph, lag, names, theme, title, values])
  return (
    <div className={well('mt-3 p-3')}>
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className={caption('m-0')}>{text}</p>
        <LagSelect lags={tauMax} value={lag} onChange={setLag} contemporaneous />
      </div>
      <EChart option={option} label={`Partial correlation heatmap at lag ${lag}`} className="h-[clamp(220px,34cqb,320px)]" />
    </div>
  )
}

/** ParCorr values as a source-by-target heatmap at one lag, with the graph mark drawn in each reported cell. */
export function TimeGraphPlot({ run }: { readonly run: TimeGraphRun }) {
  return <MarkedMatrixPlot
    graph={run.result.graph}
    values={run.result.valMatrix}
    names={run.variables.map((variable) => variable.name)}
    tauMax={run.result.tauMax}
    title={`${run.kind === 'lpcmci-run' ? 'LPCMCI' : 'PCMCI+'} partial correlations`}
    caption={`Reported links · ${run.kind === 'lpcmci-run' ? 'PAG marks' : 'lag-graph marks'}`}
  />
}

export function RpcmciTimeGraphPlot({ run, regime }: { readonly run: RpcmciRun; readonly regime: number }) {
  return <MarkedMatrixPlot
    graph={run.result.graphs[regime]}
    values={run.result.valMatrices[regime]}
    names={run.variables.map((variable) => variable.name)}
    tauMax={run.result.tauMax}
    title={`RPCMCI regime ${regime + 1} partial correlations`}
    caption={`Regime ${regime + 1} · reported lag-graph marks`}
  />
}

export function RpcmciMembershipPlot({ run }: { readonly run: RpcmciRun }) {
  const theme = useChartTheme()
  const membership = useMemo(() => regimeMembershipOption({ memberships: run.result.regimes }, theme), [run.result.regimes, theme])
  const objective = useMemo(() => annealingObjectiveOption({ best: run.result.diffGBest }, theme), [run.result.diffGBest, theme])
  return (
    <div className="mt-3 grid gap-3">
      <div className={well('p-3')}>
        <p className={caption('m-0')}>Regime membership by observation</p>
        <ExpandableChart option={membership} label="RPCMCI regime membership by observation" className="h-[clamp(200px,30cqb,300px)]" testId="rpcmci-membership" />
      </div>
      <div className={well('p-3')}>
        <p className={caption('m-0')}>Best annealing objective</p>
        <ExpandableChart option={objective} label="RPCMCI best annealing objective by iteration" className="h-[clamp(200px,30cqb,300px)]" testId="rpcmci-objective" />
      </div>
    </div>
  )
}

/** Fitted weights as a source-by-target heatmap at one lag. */
export function WeightPlot({ run }: { readonly run: WeightRun }) {
  const theme = useChartTheme()
  const [lag, setLag] = useState(0)
  const names = run.variables.map((variable) => variable.name)
  const matrices = run.kind === 'direct-lingam-run'
    ? [run.result.weights]
    : [run.result.contemporaneousWeights, ...run.result.laggedWeights]
  const method = run.kind === 'direct-lingam-run' ? 'DirectLiNGAM' : run.kind === 'var-lingam-run' ? 'VAR-LiNGAM' : 'DYNOTEARS'
  const option = useMemo(() => matrixHeatmapOption({
    title: run.kind === 'direct-lingam-run' ? 'DirectLiNGAM weights' : `${method} weights at lag ${lag}`,
    sources: names,
    targets: names,
    values: (matrices[lag] ?? []).map((targets) => targets.map((weight) => (weight === 0 ? null : weight))),
    scale: 'signed',
    quantity: 'weight',
  }, theme), [lag, matrices, method, names, run.kind, theme])
  return (
    <div className={well('mt-3 p-3')}>
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className={caption('m-0')}>{run.kind === 'direct-lingam-run' ? 'Nonzero weights · source → target' : 'Nonzero weights · source(t−lag) → target(t)'}</p>
        {run.kind !== 'direct-lingam-run' && <LagSelect lags={run.result.laggedWeights.length} value={lag} onChange={setLag} contemporaneous />}
      </div>
      <EChart
        option={option}
        label={run.kind === 'direct-lingam-run' ? 'DirectLiNGAM weight heatmap' : `Weight heatmap at lag ${lag}`}
        className="h-[clamp(220px,34cqb,320px)]"
      />
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
    <div className={well('mt-3 p-3')}>
      <p className={caption('m-0')}>Selected relations by conditional mutual information</p>
      <EChart option={option} label="Selected oCSE relations ranked by CMI" className="h-[clamp(160px,28cqb,280px)]" />
    </div>
  )
}

/** The run's structure as tigramite's two views: the summary graph and the lag grid. */
export function StructurePlot({ run, label: name, regime = 0 }: { readonly run: DiscoveryRunArtifact; readonly label: string; readonly regime?: number }) {
  const graph = useMemo(() => lagGraphFromRun(run, regime), [regime, run])
  if (!graph.ok) return <p className="mt-3 text-body text-warn">The run reported a link mark Hirmos cannot draw ({graph.error.mark}); the table below is complete.</p>
  if (graph.value.links.length === 0) return <p className="mt-3 text-body text-faint">No link to draw: the run reported no relation.</p>
  return (
    <div className="mt-3">
      <p className={caption('mb-1')}>Structure · {graph.value.semantics.replaceAll('-', ' ')}</p>
      <LagGraphViews graph={graph.value} label={name} />
    </div>
  )
}
