import { useMemo } from 'react'
import { useAdjustmentAnalysis } from '@/analysis/useAdjustmentAnalysis'
import type { DagDocument } from '@/domain/dag'
import {
  analyseDagCausalFlow,
  type DagCausalFlow,
  type DagAdjustmentAnalysis,
} from '@/domain/dagFlow'

export function useDagAdjustment(
  document: DagDocument | undefined,
  flow: DagCausalFlow | null,
): { readonly flow: DagCausalFlow | null; readonly problem: string | null } {
  const input = useMemo(() => {
    if (
      document === undefined ||
      flow === null ||
      flow.laggedArrows > 0 ||
      document.current.validation.structure.kind === 'invalid'
    )
      return null
    const nodes = document.current.graph.nodes
    const index = (id: string) => nodes.findIndex((node) => node.id === id)
    const candidates = nodes.filter((node) => flow.roles.get(node.id)?.kind === 'post-treatment')
    return {
      ids: candidates.map((node) => node.id),
      design: {
        nodes: nodes.length,
        edges: document.current.graph.edges
          .filter((edge) => edge.timing.kind === 'contemporaneous')
          .map((edge) => [index(edge.cause), index(edge.effect)] as [number, number]),
        treatment: index(flow.treatment),
        outcome: index(flow.outcome),
        unobserved: nodes.flatMap((node, i) => (node.kind === 'latent' ? [i] : [])),
        sets: candidates.map((node) => [index(node.id)]),
      },
    }
  }, [document?.current, flow])
  const state = useAdjustmentAnalysis(input?.design ?? null)
  return useMemo(() => {
    if (flow === null || document === undefined) return { flow, problem: null }
    let adjustment: DagAdjustmentAnalysis
    switch (state.kind) {
      case 'pending':
        adjustment = { kind: 'pending' }
        break
      case 'failed':
        adjustment = { kind: 'failed', detail: state.detail }
        break
      case 'unavailable':
        adjustment = { kind: 'unsupported' }
        break
      case 'ready': {
        const result = state.value.analysis
        adjustment =
          result.kind === 'notIdentified'
            ? { kind: 'none' }
            : result.emptyValid
              ? { kind: 'unnecessary' }
              : {
                  kind: 'sufficient',
                  variables: result.canonicalSet.map((i) => document.current.graph.nodes[i]!.id),
                }
        break
      }
    }
    const resolved = analyseDagCausalFlow(
      document.current.graph,
      flow.treatment,
      flow.outcome,
      adjustment,
    )
    if (state.kind === 'ready' && input !== null) {
      const roles = new Map(resolved.roles)
      input.ids.forEach((id, i) => {
        const role = roles.get(id),
          check = state.value.checks[i]
        if (
          role?.kind === 'post-treatment' &&
          role.relationship.kind === 'other' &&
          check !== undefined
        )
          roles.set(id, {
            kind: 'post-treatment',
            relationship: {
              kind: 'other',
              adjustment: check.kind === 'valid' ? 'sufficient' : 'insufficient',
            },
          })
      })
      return { flow: { ...resolved, roles }, problem: null }
    }
    return { flow: resolved, problem: null }
  }, [document?.current, flow, input, state])
}
