import { assertNever } from './dop'
import type { DiscoveryRunArtifact } from './discovery'
import { discoverySettings } from './discoverySettings'
import { methodDefinition } from './methods'

function namedEndpoints(run: Extract<DiscoveryRunArtifact, { kind: 'pc-stable-run' | 'fci-run' }>) {
  return run.result.graph.flatMap((targets, source) =>
    targets.flatMap((lags, target) =>
      target > source && lags[0]
        ? [
            {
              left: run.variables[source]?.name ?? null,
              marks: lags[0],
              right: run.variables[target]?.name ?? null,
              display: `${run.variables[source]?.name ?? '[unknown]'} ${lags[0]} ${run.variables[target]?.name ?? '[unknown]'}`,
            },
          ]
        : [],
    ),
  )
}

function methodEvidence(run: DiscoveryRunArtifact) {
  switch (run.kind) {
    case 'pc-stable-run':
      return {
        interpretation:
          'The graph represents an equivalence class under the recorded assumptions. An undirected edge does not establish either direction.',
        conditionalIndependenceTests: run.result.ciTests.length,
        separatingSets: run.result.separatingSets.length,
      }
    case 'fci-run':
      return {
        interpretation:
          'The PAG records endpoint constraints across compatible causal graphs, allowing latent confounding. A circle is an unresolved endpoint; an arrowhead is not by itself a direct causal effect.',
        conditionalIndependenceTests: run.result.ciTests.length,
        edgeProperties: run.result.edgeProperties.map(({ left, right, ...properties }) => ({
          left: run.variables[left]?.name ?? null,
          right: run.variables[right]?.name ?? null,
          ...properties,
        })),
      }
    case 'pcmci-plus-run':
      return {
        interpretation:
          'Read lagged links and contemporaneous endpoint marks separately. The recorded tests depend on the chosen lag range and conditional-independence assumptions.',
        maximumLag: run.result.tauMax,
      }
    case 'lpcmci-run':
      return {
        interpretation:
          'The temporal PAG allows latent confounding. Preserve circle and arrowhead marks; do not turn unresolved endpoints into directed causal links.',
        maximumLag: run.result.tauMax,
      }
    case 'jpcmci-plus-run':
      return {
        interpretation:
          'System, context and dummy nodes have different roles. Use the saved node map when interpreting endpoints.',
        datasets: run.result.datasets,
        periods: run.result.periods,
        ambiguousTriples: run.result.ambiguousTriples.length,
        nodes: run.nodes,
      }
    case 'rpcmci-run':
      return {
        interpretation:
          'Each graph belongs to an estimated regime, not to the whole series. Regime allocation and annealing results do not prove that regimes describe real interventions.',
        regimes: run.result.numRegimes,
        errorFreeAnnealings: run.result.errorFreeAnnealings,
        bestIterationChanges: run.result.diffGBest,
      }
    case 'cdnots-run':
    case 'cdnots-plus-run':
      return {
        interpretation:
          'Interpret links with the recorded context variables and missing-data strategy. Context adjustment does not establish that all changes over time have been captured.',
        contextVariables: run.result.contextVariables,
        missing: run.result.missing,
      }
    case 'grace-run':
      return {
        interpretation:
          'Selected gates describe the fitted predictive structure. Gate values are not probabilities that a causal link is true; training loss does not validate causal assumptions.',
        imputedCells: run.result.imputedCells,
        recordedLossValues: run.result.loss.length,
        finalLoss: run.result.loss.at(-1) ?? null,
        finalRmse: run.result.rmse.at(-1) ?? null,
      }
    case 'direct-lingam-run':
      return {
        interpretation:
          'The ordering and weights rely on the linear, non-Gaussian structural model. A returned ordering does not test those assumptions.',
        causalOrder: run.result.causalOrder.map((i) => run.variables[i]?.name ?? null),
      }
    case 'var-lingam-run':
      return {
        interpretation:
          'Separate contemporaneous structural weights from lagged weights. The fitted ordering depends on the VAR and non-Gaussian assumptions.',
        selectedLag: run.result.selectedLag,
        causalOrder: run.result.causalOrder.map((i) => run.variables[i]?.name ?? null),
      }
    case 'dynotears-run':
      return {
        interpretation:
          'These are penalised dynamic structural weights, not edge probabilities or confidence intervals. The saved evidence does not contain an optimisation-convergence certificate.',
        maximumLag: run.result.maxLag,
      }
    case 'ocse-run':
      return {
        interpretation:
          'Selected lagged links report conditional information under the recorded shuffle test. They do not measure intervention effects.',
        selectedLinks: run.result.edges.length,
        shuffles: run.result.nShuffles,
        seed: run.result.seed,
      }
    case 'cmlp-run':
    case 'clstm-run':
      return {
        interpretation:
          'These are neural Granger-predictive links, not intervention effects. Scores are not edge probabilities. Training loss and iteration counts alone do not establish convergence or out-of-sample performance.',
        iterations: run.result.iterations,
        iterationLimit: run.result.maxIter,
        reachedIterationLimit: run.result.iterations >= run.result.maxIter,
        finalLoss: run.result.loss.at(-1) ?? null,
      }
    default:
      return assertNever(run)
  }
}

export function reviewDiscovery(run: DiscoveryRunArtifact) {
  const method = methodDefinition(run.method)
  const settings = discoverySettings(
    run.configuration,
    method.ok ? method.value.name : run.configuration.kind,
    run.variables,
  )
  const evaluations = [
    ...run.eligibility.satisfied,
    ...(run.eligibility.kind === 'caution' ? run.eligibility.unresolved : []),
  ]
  return {
    kind: 'available' as const,
    method: method.ok ? method.value.name : run.configuration.kind,
    observations: run.result.observations,
    variables: run.variables.map((v) => v.name),
    backgroundKnowledge:
      'background' in run.configuration
        ? {
            meaning:
              'These constraints are recorded in the saved run configuration, not inferred from the current controls. They are inputs, not discoveries from the data.',
            settings: settings.filter((s) =>
              [
                'required',
                'forbidden',
                'requiredPatterns',
                'forbiddenPatterns',
                'tiers',
                'withinTiers',
              ].includes(s.key),
            ),
          }
        : null,
    endpoints:
      run.kind === 'pc-stable-run' || run.kind === 'fci-run'
        ? {
            legend:
              'Read each mark from the named left variable to the named right variable: --> is left to right, <-- is right to left, o is an unresolved endpoint, and <-> has two arrowheads. In a PAG, arrowheads constrain ancestry; they do not alone establish a direct effect. --- does not establish a direction.',
            edges: namedEndpoints(run),
          }
        : null,
    findings: methodEvidence(run),
    requirements: evaluations.map((evaluation) => ({
      status: evaluation.kind,
      requirement: evaluation.caveat.requirement,
      evidence: evaluation.kind === 'satisfied' ? evaluation.evidence : evaluation.missingEvidence,
      consequenceIfUnmet: evaluation.caveat.consequenceIfUnmet,
      sources: evaluation.caveat.sources,
    })),
    settings,
    limits:
      'This review reads one saved fit. It does not establish causal identification, edge stability across fits, or an intervention effect. Requirements marked satisfied retain only the scope of their recorded evidence.',
    evidence: { workspace: 'discovery', id: run.id, view: 'evidence' },
  }
}
