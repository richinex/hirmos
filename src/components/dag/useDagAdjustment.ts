import { useEffect, useMemo, useState } from 'react'
import { validateAdjustmentSets } from '@/analysis/client'
import type { DagDocument, DagNodeId } from '@/domain/dag'
import type { DagCausalFlow } from '@/domain/dagFlow'
import type { AdjustmentValidation } from '@/domain/adjustmentValidation'

/** Checks belong to this exact graph and pair, never to whichever query finishes last. */
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
    const positions = new Map(nodes.map((node, index) => [node.id, index]))
    const candidates = nodes.filter((node) => flow.roles.get(node.id)?.kind === 'post-treatment')
    if (candidates.length === 0) return null
    const index = (id: DagNodeId) => {
      const position = positions.get(id)
      if (position === undefined) throw new Error('Unknown DAG variable')
      return position
    }
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
  const [completed, setCompleted] = useState<{
    readonly input: NonNullable<typeof input>
    readonly result:
      | { readonly kind: 'checked'; readonly checks: AdjustmentValidation[] }
      | { readonly kind: 'failed'; readonly detail: string }
  } | null>(null)
  useEffect(() => {
    if (input === null) return
    let active = true
    void validateAdjustmentSets(input.design).then((result) => {
      if (!active) return
      setCompleted({
        input,
        result: result.ok
          ? { kind: 'checked', checks: result.value }
          : {
              kind: 'failed',
              detail: 'Adjustment validity could not be checked. No validity claim is shown.',
            },
      })
    })
    return () => {
      active = false
    }
  }, [input])
  return useMemo(() => {
    if (flow === null || input === null || completed?.input !== input)
      return { flow, problem: null }
    if (completed.result.kind === 'failed') return { flow, problem: completed.result.detail }
    const roles = new Map(flow.roles)
    input.ids.forEach((id, i) => {
      const role = roles.get(id)
      const check = completed.result.kind === 'checked' ? completed.result.checks[i] : undefined
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
    return { flow: { ...flow, roles }, problem: null }
  }, [flow, input, completed])
}
