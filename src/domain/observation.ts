import { err, ok, type Result } from './dop'
import type { RootCauseGraph } from './rootCause'

/** Blank entries are missing observations, never zero. */
export function readObservation(
  graph: RootCauseGraph,
  draft: Readonly<Record<string, string>>,
): Result<{ readonly values: readonly number[]; readonly csv: string }, string> {
  const values: number[] = []
  for (const node of graph.nodes) {
    const text = draft[node.id]?.trim() ?? ''
    if (text === '') return err(`Enter the observed value for ${node.name}.`)
    if (!/^[+-]?(?:\d+\.?\d*|\.\d+)(?:e[+-]?\d+)?$/i.test(text) || !Number.isFinite(Number(text)))
      return err(`Enter a finite number for ${node.name}.`)
    values.push(Number(text))
  }
  const header = graph.nodes.map((node) => `"${node.name.replaceAll('"', '""')}"`).join(',')
  return ok({ values, csv: `${header}\n${values.join(',')}\n` })
}
