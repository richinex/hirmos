import type { DagDocument } from './dag'
import type { DiscoveryRunArtifact } from './discovery'
import { assertNever, isNonEmpty, mapNonEmpty, type NonEmptyArray } from './dop'

/**
 * One method-agnostic projection of lag-resolved structure. Discovery evidence of every kind and an
 * editable DAG both map into it, so one pair of layouts (lag grid, summary graph) serves the
 * Discovery Lab and the DAG Workspace's lag expansion.
 */

export type LagEndpoint = 'tail' | 'arrow' | 'circle' | 'conflict' | 'unresolved'

/** What the link statistic means; the renderer picks a ramp per kind instead of assuming [-1, 1]. */
export type LagLinkStrength =
  | { readonly kind: 'signed-unit'; readonly value: number }
  | { readonly kind: 'signed-weight'; readonly value: number }
  | { readonly kind: 'nonnegative'; readonly value: number }
  | { readonly kind: 'structural' }
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

export type LagGraphSemantics =
  | 'stationary-lag-graph'
  | 'joint-stationary-lag-graph'
  | 'nonstationary-lag-graph'
  | 'cpdag'
  | 'pag'
  | 'regime-specific-lag-graph'
  | 'weighted-directed-evidence'
  | 'lagged-information'
  | 'neural-lagged-granger'
  | 'temporal-dag'

export interface LagGraph {
  readonly variables: NonEmptyArray<LagVariable>
  readonly tauMax: number
  readonly links: readonly LagLink[]
  readonly semantics: LagGraphSemantics
}

export type LagGraphWarning = {
  readonly kind: 'unsupported-mark'
  readonly source: number
  readonly target: number
  readonly lag: number
  readonly mark: string
}

export interface LagGraphProjection {
  readonly graph: LagGraph
  readonly warnings: readonly LagGraphWarning[]
}

const endpoint = (character: string): LagEndpoint => {
  switch (character) {
    case '-':
      return 'tail'
    case 'o':
      return 'circle'
    case 'x':
      return 'conflict'
    case '<':
    case '>':
      return 'arrow'
    default:
      return 'unresolved'
  }
}

/** Magnitude of a strength on its own scale; assumptions count as full strength. */
export const strengthMagnitude = (strength: LagLinkStrength): number => {
  switch (strength.kind) {
    case 'signed-unit':
    case 'signed-weight':
    case 'nonnegative':
      return Math.abs(strength.value)
    case 'structural':
    case 'assumption':
      return 1
    default:
      return assertNever(strength)
  }
}

export const describeStrength = (strength: LagLinkStrength): string => {
  switch (strength.kind) {
    case 'signed-unit':
      return `ParCorr ${strength.value.toFixed(3)}`
    case 'signed-weight':
      return `weight ${strength.value.toFixed(3)}`
    case 'nonnegative':
      return `strength ${strength.value.toFixed(3)}`
    case 'structural':
      return 'reported adjacency'
    case 'assumption':
      return 'user assumption'
    default:
      return assertNever(strength)
  }
}

const variablesOf = (
  run: Extract<DiscoveryRunArtifact, { readonly variables: unknown }>,
): NonEmptyArray<LagVariable> =>
  run.variables.map((column) => ({
    id: column.id,
    name: column.name,
    latent: false,
  })) as unknown as NonEmptyArray<LagVariable>

export const lagGraphFromMarkedMatrices = (
  variables: NonEmptyArray<LagVariable>,
  graph: readonly (readonly (readonly string[])[])[],
  values: readonly (readonly (readonly number[])[])[] | null,
  tauMax: number,
  semantics: Extract<
    LagGraphSemantics,
    | 'stationary-lag-graph'
    | 'joint-stationary-lag-graph'
    | 'nonstationary-lag-graph'
    | 'cpdag'
    | 'pag'
    | 'regime-specific-lag-graph'
  >,
): LagGraphProjection => {
  const links: LagLink[] = []
  const warnings: LagGraphWarning[] = []
  const n = variables.length
  for (let source = 0; source < n; source += 1) {
    for (let target = 0; target < n; target += 1) {
      for (let lag = 0; lag <= tauMax; lag += 1) {
        const mark = graph[source][target][lag]
        if (mark.length === 0) continue
        if (lag === 0 && source > target) continue
        const fromEndpoint = mark.length === 3 ? endpoint(mark[0]) : 'unresolved'
        const toEndpoint = mark.length === 3 ? endpoint(mark[2]) : 'unresolved'
        if (fromEndpoint === 'unresolved' || toEndpoint === 'unresolved') {
          warnings.push({ kind: 'unsupported-mark', source, target, lag, mark })
        }
        links.push({
          from: source,
          to: target,
          lag,
          fromEndpoint,
          toEndpoint,
          strength:
            values === null
              ? { kind: 'structural' }
              : { kind: 'signed-unit', value: values[source][target][lag] },
          mark,
        })
      }
    }
  }
  return {
    graph: { variables, tauMax, links, semantics },
    warnings,
  }
}

export function lagGraphFromConstraintRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'pc-stable-run' | 'fci-run' }>,
): LagGraphProjection {
  return lagGraphFromMarkedMatrices(
    variablesOf(run),
    run.result.graph,
    null,
    0,
    run.kind === 'fci-run' ? 'pag' : 'cpdag',
  )
}

/** Tigramite lag-graph marks into links. A contemporaneous pair is reported twice, mirrored; the lower index keeps it. */
export function lagGraphFromTimeGraphRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' | 'lpcmci-run' }>,
): LagGraphProjection {
  return lagGraphFromMarkedMatrices(
    variablesOf(run),
    run.result.graph,
    run.result.valMatrix,
    run.result.tauMax,
    run.kind === 'lpcmci-run' ? 'pag' : 'stationary-lag-graph',
  )
}

export function lagGraphFromJpcmciRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'jpcmci-plus-run' }>,
): LagGraphProjection {
  const variables = mapNonEmpty(run.nodes, (node, index) =>
    node.kind === 'observed'
      ? { id: node.column.id, name: node.column.name, latent: false }
      : { id: `jpcmci:${run.id}:${node.role}:${index}`, name: node.name, latent: false },
  )
  return lagGraphFromMarkedMatrices(
    variables,
    run.result.graph,
    run.result.valMatrix,
    run.result.tauMax,
    'joint-stationary-lag-graph',
  )
}

export function lagGraphFromRpcmciRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'rpcmci-run' }>,
  regime: number,
): LagGraphProjection {
  return lagGraphFromMarkedMatrices(
    variablesOf(run),
    run.result.graphs[regime],
    run.result.valMatrices[regime],
    run.result.tauMax,
    'regime-specific-lag-graph',
  )
}

/** CD-NOTS retains its generated context nodes in the evidence view without treating them as dataset columns. */
export function lagGraphFromCdnotsRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'cdnots-run' | 'cdnots-plus-run' }>,
): LagGraphProjection {
  const variables = [
    ...variablesOf(run),
    ...run.result.contextVariables.map((name) => ({
      id: `context:${run.id}:${name}`,
      name,
      latent: false,
    })),
  ]
  return lagGraphFromMarkedMatrices(
    isNonEmpty(variables) ? variables : [{ id: 'none', name: '—', latent: false }],
    run.result.graph,
    run.result.valMatrix,
    run.result.maxLag,
    'nonstationary-lag-graph',
  )
}

/** Fitted weight matrices into links: every nonzero coefficient is a tail → arrow link at its lag. */
export function lagGraphFromWeightRun(
  run: Extract<
    DiscoveryRunArtifact,
    { readonly kind: 'direct-lingam-run' | 'dynotears-run' | 'var-lingam-run' }
  >,
): LagGraph {
  const matrices =
    run.kind === 'direct-lingam-run'
      ? [run.result.weights]
      : [run.result.contemporaneousWeights, ...run.result.laggedWeights]
  const links: LagLink[] = []
  matrices.forEach((matrix, lag) =>
    matrix.forEach((targets, source) =>
      targets.forEach((weight, target) => {
        if (weight === 0) return
        links.push({
          from: source,
          to: target,
          lag,
          fromEndpoint: 'tail',
          toEndpoint: 'arrow',
          strength: { kind: 'signed-weight', value: weight },
          mark: null,
        })
      }),
    ),
  )
  return {
    variables: variablesOf(run),
    tauMax: matrices.length - 1,
    links,
    semantics: 'weighted-directed-evidence',
  }
}

/** Selected oCSE relations into lagged tail → arrow links carrying their conditional mutual information. */
export function lagGraphFromOcseRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'ocse-run' }>,
): LagGraph {
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

/** cMLP's active lag groups as directed predictive links; source lag is preserved exactly. */
export function lagGraphFromCmlpRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'cmlp-run' }>,
): LagGraph {
  const links: LagLink[] = []
  for (let source = 0; source < run.result.variables; source += 1) {
    for (let target = 0; target < run.result.variables; target += 1) {
      run.result.lagOrder.forEach((lag, position) => {
        if (!run.result.lagActive[source][target][position]) return
        links.push({
          from: source,
          to: target,
          lag,
          fromEndpoint: 'tail',
          toEndpoint: 'arrow',
          strength: { kind: 'nonnegative', value: run.result.lagScores[source][target][position] },
          mark: null,
        })
      })
    }
  }
  return {
    variables: variablesOf(run),
    tauMax: run.result.lag,
    links,
    semantics: 'neural-lagged-granger',
  }
}

/** GRACE gate values are nonnegative edge-selection scores at explicit lags. */
export function lagGraphFromGraceRun(
  run: Extract<DiscoveryRunArtifact, { readonly kind: 'grace-run' }>,
): LagGraph {
  const links: LagLink[] = []
  for (let source = 0; source < run.result.variables; source += 1) {
    for (let target = 0; target < run.result.variables; target += 1) {
      for (let lag = 0; lag <= run.result.maxLag; lag += 1) {
        if (!run.result.graph[source][target][lag]) continue
        links.push({
          from: source,
          to: target,
          lag,
          fromEndpoint: 'tail',
          toEndpoint: 'arrow',
          strength: { kind: 'nonnegative', value: run.result.gateValues[source][target][lag] },
          mark: null,
        })
      }
    }
  }
  return {
    variables: variablesOf(run),
    tauMax: run.result.maxLag,
    links,
    semantics: 'neural-lagged-granger',
  }
}

export type LagResolvedDiscoveryRun = Exclude<DiscoveryRunArtifact, { readonly kind: 'clstm-run' }>

export function lagGraphFromRun(run: LagResolvedDiscoveryRun, regime = 0): LagGraphProjection {
  switch (run.kind) {
    case 'direct-lingam-run':
      return { graph: lagGraphFromWeightRun(run), warnings: [] }
    case 'pc-stable-run':
    case 'fci-run':
      return lagGraphFromConstraintRun(run)
    case 'pcmci-plus-run':
    case 'lpcmci-run':
      return lagGraphFromTimeGraphRun(run)
    case 'jpcmci-plus-run':
      return lagGraphFromJpcmciRun(run)
    case 'rpcmci-run':
      return lagGraphFromRpcmciRun(run, regime)
    case 'cdnots-run':
    case 'cdnots-plus-run':
      return lagGraphFromCdnotsRun(run)
    case 'grace-run':
      return { graph: lagGraphFromGraceRun(run), warnings: [] }
    case 'dynotears-run':
    case 'var-lingam-run':
      return { graph: lagGraphFromWeightRun(run), warnings: [] }
    case 'ocse-run':
      return { graph: lagGraphFromOcseRun(run), warnings: [] }
    case 'cmlp-run':
      return { graph: lagGraphFromCmlpRun(run), warnings: [] }
    default:
      return assertNever(run)
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
  const variables = nodes.map((node) => ({
    id: node.id,
    name: node.name,
    latent: node.kind === 'latent',
  }))
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
      const strongest = matching.reduce((best, link) =>
        strengthMagnitude(link.strength) >= strengthMagnitude(best.strength) ? link : best,
      )
      links.push({
        from,
        to,
        fromEndpoint: strongest.fromEndpoint,
        toEndpoint: strongest.toEndpoint,
        strength: strongest.strength,
        mark: strongest.mark,
        lags: matching
          .map((link) => link.lag)
          .filter((lag) => lag > 0)
          .sort((left, right) => left - right),
        contemporaneous: matching.some((link) => link.lag === 0),
      })
    }
  }
  const autos = graph.variables.map((_, variable) => {
    const own = graph.links.filter((link) => link.from === variable && link.to === variable)
    if (own.length === 0) return null
    return own.reduce((best, link) =>
      strengthMagnitude(link.strength) > strengthMagnitude(best.strength) ? link : best,
    ).strength
  })
  return { variables: graph.variables, links, autos, semantics: graph.semantics }
}
