import { Select } from '@/components/ui/Select'
import { useMemo } from 'react'
import { LagGraphViews } from '@/components/discovery/LagGraphViews'
import { label, literal, num } from '@/components/ui/recipes'
import { assertNever } from '@/domain/dop'
import {
  describeEvidenceSemantics,
  discoveryEvidenceView,
  type DiscoveryCandidate,
  type DiscoveryEvidenceView,
} from '@/domain/dagEvidence'
import type { DiscoveryRunArtifact, DiscoveryRunId } from '@/domain/discovery'
import { lagGraphFromRun } from '@/domain/lagGraph'

const methodTitle = (run: DiscoveryRunArtifact): string => {
  switch (run.kind) {
    case 'direct-lingam-run': return 'DirectLiNGAM'
    case 'pcmci-plus-run': return 'PCMCI+'
    case 'lpcmci-run': return 'LPCMCI'
    case 'rpcmci-run': return 'RPCMCI'
    case 'dynotears-run': return 'DYNOTEARS'
    case 'var-lingam-run': return 'VAR-LiNGAM'
    case 'ocse-run': return 'oCSE'
    default: return assertNever(run)
  }
}

const variablesOf = (view: DiscoveryEvidenceView) => {
  switch (view.run.kind) {
    case 'direct-lingam-run':
    case 'pcmci-plus-run':
    case 'lpcmci-run':
    case 'rpcmci-run':
    case 'dynotears-run':
    case 'var-lingam-run':
    case 'ocse-run': return view.run.variables
    default: return assertNever(view.run)
  }
}

const statistic = (value: number): string => Math.abs(value) >= 1_000 || (Math.abs(value) > 0 && Math.abs(value) < 0.001)
  ? value.toExponential(2)
  : value.toFixed(3)

const pValue = (value: number): string => value < 0.001 ? value.toExponential(2) : value.toFixed(3)

const candidateLabel = (candidate: DiscoveryCandidate): string => {
  switch (candidate.kind) {
    case 'endpoint-marked': return `${candidate.mark}${candidate.lag === 0 ? '' : ` · t−${candidate.lag}`}`
    case 'regime-endpoint-marked': return `Regime ${candidate.regime + 1} · ${candidate.mark}${candidate.lag === 0 ? '' : ` · t−${candidate.lag}`}`
    case 'weighted-directed': return `w ${statistic(candidate.weight)}${candidate.lag === 0 ? '' : ` · t−${candidate.lag}`}`
    case 'lagged-information': return `CMI ${statistic(candidate.cmi)} · t−${candidate.lag}`
    default: return assertNever(candidate)
  }
}

const candidateDetail = (candidate: DiscoveryCandidate): string => {
  switch (candidate.kind) {
    case 'endpoint-marked': return `mark ${candidate.mark} · p ${pValue(candidate.pValue)} · ParCorr ${statistic(candidate.statistic)}`
    case 'regime-endpoint-marked': return `regime ${candidate.regime + 1} · mark ${candidate.mark} · p ${pValue(candidate.pValue)} · ParCorr ${statistic(candidate.statistic)}`
    case 'weighted-directed': return `weight ${statistic(candidate.weight)}${candidate.lag === 0 ? ' · contemporaneous' : ` · lag ${candidate.lag}`}`
    case 'lagged-information': return `CMI ${statistic(candidate.cmi)} · p ${pValue(candidate.pValue)} · lag ${candidate.lag}`
    default: return assertNever(candidate)
  }
}

function EvidenceGraph({ view, selected }: {
  readonly view: DiscoveryEvidenceView
  readonly selected: DiscoveryCandidate | null
}) {
  const regime = selected?.kind === 'regime-endpoint-marked' ? selected.regime : 0
  const graph = useMemo(() => lagGraphFromRun(view.run, regime), [regime, view.run])
  if (!graph.ok) return <p className="text-body text-warn">The run reported a link mark Hirmos cannot draw ({graph.error.mark}).</p>
  const highlighted = selected === null ? [] : [selected.source.column, selected.target.column]
  return <LagGraphViews graph={graph.value} label={`${view.method} evidence graph`} highlighted={highlighted} compact />
}

export function EvidenceInspector({
  runs,
  selectedRun,
  selectedCandidate,
  onRunSelected,
  onCandidateSelected,
}: {
  readonly runs: readonly DiscoveryRunArtifact[]
  readonly selectedRun: DiscoveryRunId | null
  readonly selectedCandidate: DiscoveryCandidate | null
  readonly onRunSelected: (run: DiscoveryRunId) => void
  readonly onCandidateSelected: (candidate: DiscoveryCandidate) => void
}) {
  const run = selectedRun === null ? runs.at(-1) : runs.find((candidate) => candidate.id === selectedRun)
  if (run === undefined) {
    return (
      <aside className="border-t border-hair pt-4" aria-labelledby="discovery-evidence-title">
        <h3 id="discovery-evidence-title" className="mb-1 mt-0 text-body font-medium text-ink">No discovery runs</h3>
        <p className="m-0 text-body text-faint">Create a DAG from substantive knowledge or an experimental design without running discovery.</p>
      </aside>
    )
  }
  const view = discoveryEvidenceView(run)
  return (
    <aside className="min-w-0 border-t border-hair pt-4" aria-labelledby="discovery-evidence-title">
      <div className="mb-3 flex items-start justify-between gap-2">
        <div className="min-w-0">
          <h3 id="discovery-evidence-title" className="mb-1 mt-0 text-body font-medium text-ink">{view.method}</h3>
          <p className="m-0 text-body text-faint">{describeEvidenceSemantics(view)}</p>
        </div>
        <Select
          aria-label="Evidence run"
          className="max-w-32 rounded-md border border-hair bg-well px-2 py-1.5 text-body text-ink"
          value={run.id}
          onChange={(event) => {
            const next = runs.find((candidate) => candidate.id === event.target.value)
            if (next !== undefined) onRunSelected(next.id)
          }}
        >
          {runs.map((candidate, index) => <option key={candidate.id} value={candidate.id}>{methodTitle(candidate)} · {index + 1}</option>)}
        </Select>
      </div>
      <EvidenceGraph view={view} selected={selectedCandidate?.run === run.id ? selectedCandidate : null} />
      <div className="mt-3 max-h-52 space-y-1.5 overflow-auto" aria-label="Discovered relations">
        {view.candidates.length === 0 ? (
          <p className="m-0 text-body text-faint">This run returned no marked relation.</p>
        ) : view.candidates.map((candidate) => (
          <button
            key={candidate.id}
            type="button"
            aria-pressed={selectedCandidate?.id === candidate.id}
            onClick={() => onCandidateSelected(candidate)}
            className="block w-full rounded-lg border border-hair bg-well px-2.5 py-2 text-left transition-colors hover:border-edge aria-pressed:border-signal aria-pressed:bg-raised"
          >
            <span className="block text-body font-medium text-ink">{candidate.source.name} {candidate.kind === 'endpoint-marked' || candidate.kind === 'regime-endpoint-marked' ? candidate.mark : '→'} {candidate.target.name}</span>
            <span className={num('mt-0.5 block text-label text-faint')}>{candidateDetail(candidate)}</span>
          </button>
        ))}
      </div>
      {selectedCandidate !== null && selectedCandidate.run === run.id && (
        <p className={literal('mb-0 mt-3 border-t border-hair pt-3 text-label text-muted')}>
          Blue marks the reported source; amber marks the target. Selection does not create, orient, or alter a causal edge.
        </p>
      )}
    </aside>
  )
}
