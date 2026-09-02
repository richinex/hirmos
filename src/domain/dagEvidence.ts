import {assertNever, brand } from './dop'
import type { ColumnId, NumericColumnSelection } from './dataset'
import type { DiscoveryCandidateId } from './dag'
import type { DiscoveryRunArtifact, DiscoveryRunId } from './discovery'

export interface EvidenceVariable {
  readonly column: ColumnId
  readonly name: string
}

export type EvidenceRelationMatch =
  | {
      readonly kind: 'directed-candidate'
      readonly cause: EvidenceVariable
      readonly effect: EvidenceVariable
      readonly timing: { readonly kind: 'contemporaneous' } | { readonly kind: 'lagged'; readonly lag: number }
    }
  | {
      readonly kind: 'directed-window-candidate'
      readonly cause: EvidenceVariable
      readonly effect: EvidenceVariable
      readonly context: number
    }
  | { readonly kind: 'orientation-unresolved' }

interface CandidateBase {
  readonly id: DiscoveryCandidateId
  readonly run: DiscoveryRunId
  readonly source: EvidenceVariable
  readonly target: EvidenceVariable
  readonly relationMatch: EvidenceRelationMatch
}

export type DiscoveryCandidate =
  | CandidateBase & {
      readonly kind: 'endpoint-marked'
      readonly method: 'PCMCI+' | 'LPCMCI'
      readonly lag: number
      readonly mark: string
      readonly pValue: number
      readonly statistic: number
    }
  | CandidateBase & {
      readonly kind: 'regime-endpoint-marked'
      readonly method: 'RPCMCI'
      readonly regime: number
      readonly lag: number
      readonly mark: string
      readonly pValue: number
      readonly statistic: number
    }
  | CandidateBase & {
      readonly kind: 'weighted-directed'
      readonly method: 'DirectLiNGAM' | 'DYNOTEARS' | 'VAR-LiNGAM'
      readonly lag: number
      readonly weight: number
    }
  | CandidateBase & {
      readonly kind: 'lagged-information'
      readonly method: 'oCSE'
      readonly lag: number
      readonly cmi: number
      readonly pValue: number
    }
  | CandidateBase & {
      readonly kind: 'neural-lagged'
      readonly method: 'cMLP'
      readonly lag: number
      readonly score: number
    }
  | CandidateBase & {
      readonly kind: 'neural-window'
      readonly method: 'cLSTM'
      readonly context: number
      readonly score: number
    }

export interface DiscoveryEvidenceView {
  readonly run: DiscoveryRunArtifact
  readonly method: 'DirectLiNGAM' | 'PCMCI+' | 'LPCMCI' | 'RPCMCI' | 'DYNOTEARS' | 'VAR-LiNGAM' | 'oCSE' | 'cMLP' | 'cLSTM'
  readonly semantics: 'stationary-lag-graph' | 'pag' | 'regime-specific-lag-graphs' | 'weighted-directed-evidence' | 'lagged-information' | 'neural-lagged-granger' | 'neural-window-granger'
  readonly candidates: readonly DiscoveryCandidate[]
}

const candidateId = (run: DiscoveryRunId, key: string): DiscoveryCandidateId =>
  brand<string, 'DiscoveryCandidateId'>(`${run}:${key}`)

const variable = (value: NumericColumnSelection): EvidenceVariable => ({ column: value.id, name: value.name })

const markedRelationMatch = (
  source: EvidenceVariable,
  target: EvidenceVariable,
  lag: number,
  mark: string,
): EvidenceRelationMatch => {
  if (mark === '-->') {
    return {
      kind: 'directed-candidate',
      cause: source,
      effect: target,
      timing: lag === 0 ? { kind: 'contemporaneous' } : { kind: 'lagged', lag },
    }
  }
  if (lag === 0 && mark === '<--') {
    return {
      kind: 'directed-candidate',
      cause: target,
      effect: source,
      timing: { kind: 'contemporaneous' },
    }
  }
  return { kind: 'orientation-unresolved' }
}

const matrixCandidates = (
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' | 'lpcmci-run' }>,
): readonly DiscoveryCandidate[] => {
  const candidates: DiscoveryCandidate[] = []
  for (let sourceIndex = 0; sourceIndex < run.result.variables; sourceIndex += 1) {
    for (let targetIndex = 0; targetIndex < run.result.variables; targetIndex += 1) {
      for (let lag = 0; lag <= run.result.tauMax; lag += 1) {
        const mark = run.result.graph[sourceIndex][targetIndex][lag]
        if (mark.length === 0 || (lag === 0 && sourceIndex > targetIndex)) continue
        const source = variable(run.variables[sourceIndex])
        const target = variable(run.variables[targetIndex])
        candidates.push({
          kind: 'endpoint-marked',
          id: candidateId(run.id, `${sourceIndex}:${targetIndex}:${lag}:${mark}`),
          run: run.id,
          method: run.kind === 'lpcmci-run' ? 'LPCMCI' : 'PCMCI+',
          source,
          target,
          lag,
          mark,
          pValue: run.result.pMatrix[sourceIndex][targetIndex][lag],
          statistic: run.result.valMatrix[sourceIndex][targetIndex][lag],
          relationMatch: markedRelationMatch(source, target, lag, mark),
        })
      }
    }
  }
  return candidates
}

const regimeMatrixCandidates = (
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'rpcmci-run' }>,
): readonly DiscoveryCandidate[] => run.result.graphs.flatMap((graph, regime) => {
  const candidates: DiscoveryCandidate[] = []
  for (let sourceIndex = 0; sourceIndex < run.result.variables; sourceIndex += 1) {
    for (let targetIndex = 0; targetIndex < run.result.variables; targetIndex += 1) {
      for (let lag = 0; lag <= run.result.tauMax; lag += 1) {
        const mark = graph[sourceIndex][targetIndex][lag]
        if (mark.length === 0 || (lag === 0 && sourceIndex > targetIndex)) continue
        const source = variable(run.variables[sourceIndex])
        const target = variable(run.variables[targetIndex])
        candidates.push({
          kind: 'regime-endpoint-marked',
          id: candidateId(run.id, `regime:${regime}:${sourceIndex}:${targetIndex}:${lag}:${mark}`),
          run: run.id,
          method: 'RPCMCI',
          regime,
          source,
          target,
          lag,
          mark,
          pValue: run.result.pMatrices[regime][sourceIndex][targetIndex][lag],
          statistic: run.result.valMatrices[regime][sourceIndex][targetIndex][lag],
          relationMatch: markedRelationMatch(source, target, lag, mark),
        })
      }
    }
  }
  return candidates
})

/** Every nonzero fitted weight, strongest first; weights[lag][source][target] with lag 0 contemporaneous. */
const weightedCandidates = (
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'direct-lingam-run' | 'dynotears-run' | 'var-lingam-run' }>,
  method: 'DirectLiNGAM' | 'DYNOTEARS' | 'VAR-LiNGAM',
): readonly DiscoveryCandidate[] => {
  const candidates: Extract<DiscoveryCandidate, { readonly kind: 'weighted-directed' }>[] = []
  const matrices = run.kind === 'direct-lingam-run'
    ? [run.result.weights]
    : [run.result.contemporaneousWeights, ...run.result.laggedWeights]
  for (let lag = 0; lag < matrices.length; lag += 1) {
    for (let sourceIndex = 0; sourceIndex < run.result.variables; sourceIndex += 1) {
      for (let targetIndex = 0; targetIndex < run.result.variables; targetIndex += 1) {
        const weight = matrices[lag][sourceIndex][targetIndex]
        if (weight === 0) continue
        const source = variable(run.variables[sourceIndex])
        const target = variable(run.variables[targetIndex])
        candidates.push({
          kind: 'weighted-directed',
          id: candidateId(run.id, `${sourceIndex}:${targetIndex}:${lag}`),
          run: run.id,
          method,
          source,
          target,
          lag,
          weight,
          relationMatch: {
            kind: 'directed-candidate',
            cause: source,
            effect: target,
            timing: lag === 0 ? { kind: 'contemporaneous' } : { kind: 'lagged', lag },
          },
        })
      }
    }
  }
  candidates.sort((left, right) => Math.abs(right.weight) - Math.abs(left.weight))
  return candidates
}

const cmlpCandidates = (
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'cmlp-run' }>,
): readonly DiscoveryCandidate[] => {
  const candidates: Extract<DiscoveryCandidate, { readonly kind: 'neural-lagged' }>[] = []
  for (let sourceIndex = 0; sourceIndex < run.result.variables; sourceIndex += 1) {
    for (let targetIndex = 0; targetIndex < run.result.variables; targetIndex += 1) {
      run.result.lagOrder.forEach((lag, position) => {
        if (!run.result.lagActive[sourceIndex][targetIndex][position]) return
        const source = variable(run.variables[sourceIndex])
        const target = variable(run.variables[targetIndex])
        candidates.push({
          kind: 'neural-lagged',
          id: candidateId(run.id, `${sourceIndex}:${targetIndex}:${lag}`),
          run: run.id,
          method: 'cMLP',
          source,
          target,
          lag,
          score: run.result.lagScores[sourceIndex][targetIndex][position],
          relationMatch: {
            kind: 'directed-candidate',
            cause: source,
            effect: target,
            timing: { kind: 'lagged', lag },
          },
        })
      })
    }
  }
  candidates.sort((left, right) => right.score - left.score)
  return candidates
}

const clstmCandidates = (
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'clstm-run' }>,
): readonly DiscoveryCandidate[] => {
  const candidates: Extract<DiscoveryCandidate, { readonly kind: 'neural-window' }>[] = []
  for (let sourceIndex = 0; sourceIndex < run.result.variables; sourceIndex += 1) {
    for (let targetIndex = 0; targetIndex < run.result.variables; targetIndex += 1) {
      if (!run.result.summaryActive[sourceIndex][targetIndex]) continue
      const source = variable(run.variables[sourceIndex])
      const target = variable(run.variables[targetIndex])
      candidates.push({
        kind: 'neural-window',
        id: candidateId(run.id, `${sourceIndex}:${targetIndex}:window`),
        run: run.id,
        method: 'cLSTM',
        source,
        target,
        context: run.result.context,
        score: run.result.summaryScores[sourceIndex][targetIndex],
        relationMatch: { kind: 'directed-window-candidate', cause: source, effect: target, context: run.result.context },
      })
    }
  }
  candidates.sort((left, right) => right.score - left.score)
  return candidates
}

export function discoveryEvidenceView(run: DiscoveryRunArtifact): DiscoveryEvidenceView {
  switch (run.kind) {
    case 'direct-lingam-run': return {
      run,
      method: 'DirectLiNGAM',
      semantics: 'weighted-directed-evidence',
      candidates: weightedCandidates(run, 'DirectLiNGAM'),
    }
    case 'pcmci-plus-run': return {
      run,
      method: 'PCMCI+',
      semantics: 'stationary-lag-graph',
      candidates: matrixCandidates(run),
    }
    case 'lpcmci-run': return {
      run,
      method: 'LPCMCI',
      semantics: 'pag',
      candidates: matrixCandidates(run),
    }
    case 'rpcmci-run': return {
      run,
      method: 'RPCMCI',
      semantics: 'regime-specific-lag-graphs',
      candidates: regimeMatrixCandidates(run),
    }
    case 'dynotears-run': return {
      run,
      method: 'DYNOTEARS',
      semantics: 'weighted-directed-evidence',
      candidates: weightedCandidates(run, 'DYNOTEARS'),
    }
    case 'var-lingam-run': return {
      run,
      method: 'VAR-LiNGAM',
      semantics: 'weighted-directed-evidence',
      candidates: weightedCandidates(run, 'VAR-LiNGAM'),
    }
    case 'ocse-run': return {
      run,
      method: 'oCSE',
      semantics: 'lagged-information',
      candidates: run.result.edges.map((edge, index) => {
        const source = variable(run.variables[edge.source])
        const target = variable(run.variables[edge.target])
        return {
          kind: 'lagged-information' as const,
          id: candidateId(run.id, `${edge.source}:${edge.target}:${edge.lag}:${index}`),
          run: run.id,
          method: 'oCSE' as const,
          source,
          target,
          lag: edge.lag,
          cmi: edge.cmi,
          pValue: edge.pValue,
          relationMatch: {
            kind: 'directed-candidate' as const,
            cause: source,
            effect: target,
            timing: { kind: 'lagged' as const, lag: edge.lag },
          },
        }
      }),
    }
    case 'cmlp-run': return {
      run,
      method: 'cMLP',
      semantics: 'neural-lagged-granger',
      candidates: cmlpCandidates(run),
    }
    case 'clstm-run': return {
      run,
      method: 'cLSTM',
      semantics: 'neural-window-granger',
      candidates: clstmCandidates(run),
    }
    default: return assertNever(run)
  }
}

export function describeEvidenceSemantics(view: DiscoveryEvidenceView): string {
  switch (view.semantics) {
    case 'stationary-lag-graph': return 'Conditional-dependence marks over lagged variables'
    case 'pag': return 'Partial ancestral graph marks; circles and bidirected endpoints remain unresolved'
    case 'regime-specific-lag-graphs': return 'A separately estimated lag graph for each inferred regime'
    case 'weighted-directed-evidence': return 'Fitted directed structural weights'
    case 'lagged-information': return 'Selected lagged conditional-information relations'
    case 'neural-lagged-granger': return 'Lag-resolved predictive relations selected by structured neural sparsity'
    case 'neural-window-granger': return 'Directed predictive relations over the fitted history window; individual lags are not identified'
    default: return assertNever(view.semantics)
  }
}

export function discoveryEvidenceReference(candidate: DiscoveryCandidate) {
  switch (candidate.kind) {
    case 'endpoint-marked': return { kind: 'discovery' as const, run: candidate.run, candidate: candidate.id, semantics: 'endpoint-marked' as const }
    case 'regime-endpoint-marked': return { kind: 'discovery' as const, run: candidate.run, candidate: candidate.id, semantics: 'endpoint-marked' as const }
    case 'weighted-directed': return { kind: 'discovery' as const, run: candidate.run, candidate: candidate.id, semantics: 'weighted-directed' as const }
    case 'lagged-information': return { kind: 'discovery' as const, run: candidate.run, candidate: candidate.id, semantics: 'lagged-information' as const }
    case 'neural-lagged': return { kind: 'discovery' as const, run: candidate.run, candidate: candidate.id, semantics: 'weighted-directed' as const }
    case 'neural-window': return { kind: 'discovery' as const, run: candidate.run, candidate: candidate.id, semantics: 'weighted-directed' as const }
    default: return assertNever(candidate)
  }
}
