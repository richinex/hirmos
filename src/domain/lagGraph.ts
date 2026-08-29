import type { DagDocument } from './dag'
import type { DiscoveryRunArtifact } from './discovery'
import { assertNever, err, isNonEmpty, ok, type NonEmptyArray, type Result } from './dop'

/**
 * One method-agnostic projection of lag-resolved structure. Discovery evidence of every kind and an
 * editable DAG both map into it, so one pair of layouts (lag grid, summary graph) serves the
 * Discovery Lab and the DAG Workspace's lag expansion.
 */

export type LagEndpoint = 'tail' | 'arrow' | 'circle'

/** What the link statistic means; the renderer picks a ramp per kind instead of assuming [-1, 1]. */
export type LagLinkStrength =
  | { readonly kind: 'signed-unit'; readonly value: number }
  | { readonly kind: 'signed-weight'; readonly value: number }
  | { readonly kind: 'nonnegative'; readonly value: number }
  | { readonly kind: 'assumption' }

export interface LagVariable {
  readonly id: string
  readonly name: string
  readonly latent: boolean
}

export interface LagLink {
  readonly from: number
  readonly to: number
  /** 0 is contemporaneous. */
  readonly lag: number
  readonly fromEndpoint: LagEndpoint
  readonly toEndpoint: LagEndpoint
  readonly strength: LagLinkStrength
  /** The original mark string for endpoint-marked evidence, kept for labels and tooltips. */
  readonly mark: string | null
}

export type LagGraphSemantics = 'stationary-lag-graph' | 'pag' | 'weighted-directed-evidence' | 'lagged-information' | 'temporal-dag'

export interface LagGraph {
  readonly variables: NonEmptyArray<LagVariable>
  readonly tauMax: number
  readonly links: readonly LagLink[]
  readonly semantics: LagGraphSemantics
}

export type LagGraphProblem =
  | { readonly kind: 'unknown-mark'; readonly source: number; readonly target: number; readonly lag: number; readonly mark: string }

const endpoint = (character: string): LagEndpoint | null => {
  switch (character) {
    case '-': return 'tail'
    case 'o': return 'circle'
    case '<':
    case '>': return 'arrow'
    default: return null
  }
}

/** Magnitude of a strength on its own scale; assumptions count as full strength. */
export const strengthMagnitude = (strength: LagLinkStrength): number => {
  switch (strength.kind) {
    case 'signed-unit':
    case 'signed-weight':
    case 'nonnegative': return Math.abs(strength.value)
    case 'assumption': return 1
    default: return assertNever(strength)
  }
}

export const describeStrength = (strength: LagLinkStrength): string => {
  switch (strength.kind) {
    case 'signed-unit': return `ParCorr ${strength.value.toFixed(3)}`
    case 'signed-weight': return `weight ${strength.value.toFixed(3)}`
    case 'nonnegative': return `strength ${strength.value.toFixed(3)}`
    case 'assumption': return 'user assumption'
    default: return assertNever(strength)
  }
}

const variablesOf = (run: Extract<DiscoveryRunArtifact, { readonly variables: unknown }>): NonEmptyArray<LagVariable> =>
  run.variables.map((column) => ({ id: column.id, name: column.name, latent: false })) as unknown as NonEmptyArray<LagVariable>

/** Tigramite lag-graph marks into links. A contemporaneous pair is reported twice, mirrored; the lower index keeps it. */
export function lagGraphFromTimeGraphRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' | 'lpcmci-run' }>,
): Result<LagGraph, LagGraphProblem> {
  const links: LagLink[] = []
  const n = run.result.variables
  for (let source = 0; source < n; source += 1) {
    for (let target = 0; target < n; target += 1) {
      for (let lag = 0; lag <= run.result.tauMax; lag += 1) {
        const mark = run.result.graph[source][target][lag]
        if (mark.length === 0) continue
        if (lag === 0 && source > target) continue
        const fromEndpoint = mark.length === 3 ? endpoint(mark[0]) : null
        const toEndpoint = mark.length === 3 ? endpoint(mark[2]) : null
        if (fromEndpoint === null || toEndpoint === null) return err({ kind: 'unknown-mark', source, target, lag, mark })
        links.push({
          from: source,
          to: target,
          lag,
          fromEndpoint,
          toEndpoint,
          strength: { kind: 'signed-unit', value: run.result.valMatrix[source][target][lag] },
          mark,
        })
      }
    }
  }
  return ok({
    variables: variablesOf(run),
    tauMax: run.result.tauMax,
    links,
    semantics: run.kind === 'lpcmci-run' ? 'pag' : 'stationary-lag-graph',
  })
}

/** Fitted weight matrices into links: every nonzero coefficient is a tail → arrow link at its lag. */
export function lagGraphFromWeightRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'dynotears-run' | 'var-lingam-run' }>,
): LagGraph {
  const matrices = [run.result.contemporaneousWeights, ...run.result.laggedWeights]
  const links: LagLink[] = []
  matrices.forEach((matrix, lag) => matrix.forEach((targets, source) => targets.forEach((weight, target) => {
    if (weight === 0) return
    links.push({ from: source, to: target, lag, fromEndpoint: 'tail', toEndpoint: 'arrow', strength: { kind: 'signed-weight', value: weight }, mark: null })
  })))
  return { variables: variablesOf(run), tauMax: matrices.length - 1, links, semantics: 'weighted-directed-evidence' }
}

/** Selected oCSE relations into lagged tail → arrow links carrying their conditional mutual information. */
export function lagGraphFromOcseRun(run: Extract<DiscoveryRunArtifact, { readonly kind: 'ocse-run' }>): LagGraph {
  return {
    variables: variablesOf(run),
    tauMax: run.result.maxLag,
    links: run.result.edges.map((edge) => ({
      from: edge.source,
      to: edge.target,
      lag: edge.lag,
      fromEndpoint: 'tail',
      toEndpoint: 'arrow',
      strength: { kind: 'nonnegative', value: edge.cmi },
      mark: null,
    })),
    semantics: 'lagged-information',
  }
}

export function lagGraphFromRun(run: DiscoveryRunArtifact): Result<LagGraph, LagGraphProblem> {
  switch (run.kind) {
    case 'pcmci-plus-run':
    case 'lpcmci-run': return lagGraphFromTimeGraphRun(run)
    case 'dynotears-run':
    case 'var-lingam-run': return ok(lagGraphFromWeightRun(run))
    case 'ocse-run': return ok(lagGraphFromOcseRun(run))
    default: return assertNever(run)
  }
}

/** The editable DAG as a lag graph: the "lag expansion on demand" hook. */
export function lagGraphFromDag(document: DagDocument): LagGraph {
  const nodes = document.current.graph.nodes
  const index = new Map(nodes.map((node, position) => [node.id, position]))
  const links: LagLink[] = []
  for (const edge of document.current.graph.edges) {
    const from = index.get(edge.cause)
    const to = index.get(edge.effect)
    if (from === undefined || to === undefined) continue
    links.push({
      from,
      to,
      lag: edge.timing.kind === 'contemporaneous' ? 0 : edge.timing.lag,
      fromEndpoint: 'tail',
      toEndpoint: 'arrow',
      strength: { kind: 'assumption' },
      mark: null,
    })
  }
  const variables = nodes.map((node) => ({ id: node.id, name: node.name, latent: node.kind === 'latent' }))
  return {
    variables: isNonEmpty(variables) ? variables : [{ id: 'none', name: '—', latent: false }],
    tauMax: Math.max(0, ...links.map((link) => link.lag)),
    links,
    semantics: 'temporal-dag',
  }
}

export interface SummaryLink {
  readonly from: number
  readonly to: number
  readonly fromEndpoint: LagEndpoint
  readonly toEndpoint: LagEndpoint
  /** The strongest link across lags. */
  readonly strength: LagLinkStrength
  readonly mark: string | null
  /** Positive lags carrying a link, ascending; empty means contemporaneous only. */
  readonly lags: readonly number[]
  readonly contemporaneous: boolean
}

export interface SummaryGraph {
  readonly variables: NonEmptyArray<LagVariable>
  readonly links: readonly SummaryLink[]
  /** Strongest self-dependency per variable, or null. */
  readonly autos: readonly (LagLinkStrength | null)[]
  readonly semantics: LagGraphSemantics
}

/** Collapse the lag axis: one link per ordered pair, keeping the strongest lag's endpoints; tigramite's process graph. */
export function summarizeLagGraph(graph: LagGraph): SummaryGraph {
  const n = graph.variables.length
  const links: SummaryLink[] = []
  for (let from = 0; from < n; from += 1) {
    for (let to = 0; to < n; to += 1) {
      if (from === to) continue
      const matching = graph.links.filter((link) => link.from === from && link.to === to)
      if (matching.length === 0) continue
      const strongest = matching.reduce((best, link) => (strengthMagnitude(link.strength) >= strengthMagnitude(best.strength) ? link : best))
      links.push({
        from,
        to,
        fromEndpoint: strongest.fromEndpoint,
        toEndpoint: strongest.toEndpoint,
        strength: strongest.strength,
        mark: strongest.mark,
        lags: matching.map((link) => link.lag).filter((lag) => lag > 0).sort((left, right) => left - right),
        contemporaneous: matching.some((link) => link.lag === 0),
      })
    }
  }
  const autos = graph.variables.map((_, variable) => {
    const own = graph.links.filter((link) => link.from === variable && link.to === variable)
    if (own.length === 0) return null
    return own.reduce((best, link) => (strengthMagnitude(link.strength) > strengthMagnitude(best.strength) ? link : best)).strength
  })
  return { variables: graph.variables, links, autos, semantics: graph.semantics }
}
