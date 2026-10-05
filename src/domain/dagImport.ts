import {
  describeDagEditProblem,
  describeDagVariableEditProblem,
  type DagDocument,
  type DagImport,
  type DagImportProblem,
} from './dag'
import {
  describeDagTextProblem,
  parseDagText,
  type DagTextEdge,
  type DagTextProblem,
} from './dagText'
import { err, isNonEmpty, ok, type NonEmptyArray, type Result } from './dop'

/**
 * A pasted graph, resolved against the document it will join: which names are variables already
 * there, which become unmeasured variables, and which roles the text declares. The plan is the
 * decision; `reviseDagWithImportedGraph` is the action.
 */
export interface DagImportPlan {
  readonly known: readonly string[]
  readonly latent: readonly string[]
  readonly arrows: readonly DagTextEdge[]
  /** The one declared exposure or outcome; null when the text declares none or more than one. */
  readonly exposure: string | null
  readonly outcome: string | null
  readonly bidirected: number
}

export type DagImportPlanProblem =
  | { readonly kind: 'text'; readonly problem: DagTextProblem }
  | { readonly kind: 'no-arrows' }
  | { readonly kind: 'unmarked-variables'; readonly names: NonEmptyArray<string> }

const only = (names: readonly string[]): string | null => (names.length === 1 ? names[0]! : null)

/**
 * An unmarked variable must be a column of the prepared data or a variable already drawn; one
 * that is neither is refused by name rather than guessed at, since marking it `[latent]` in the
 * text is what says it is unmeasured.
 */
export function planDagImport(
  document: DagDocument,
  text: string,
): Result<DagImportPlan, DagImportPlanProblem> {
  const parsed = parseDagText(text)
  if (!parsed.ok) return err({ kind: 'text', problem: parsed.error })
  if (parsed.value.edges.length === 0) return err({ kind: 'no-arrows' })

  const present = new Set(document.current.graph.nodes.map((node) => node.name))
  const known: string[] = []
  const latent: string[] = []
  const unmarked: string[] = []
  for (const node of parsed.value.nodes) {
    if (present.has(node.name)) known.push(node.name)
    else if (node.kind === 'latent') latent.push(node.name)
    else unmarked.push(node.name)
  }
  if (isNonEmpty(unmarked)) return err({ kind: 'unmarked-variables', names: unmarked })

  return ok({
    known,
    latent,
    arrows: parsed.value.edges,
    exposure: only(parsed.value.exposures),
    outcome: only(parsed.value.outcomes),
    bidirected: parsed.value.bidirected,
  })
}

export const importOf = (plan: DagImportPlan): DagImport => ({
  latent: plan.latent,
  arrows: plan.arrows,
})

export const describeDagImportProblem = (
  problem: DagImportPlanProblem | DagImportProblem,
): string => {
  switch (problem.kind) {
    case 'text':
      return describeDagTextProblem(problem.problem)
    case 'no-arrows':
      return 'The text declares no arrows, so there is nothing to draw.'
    case 'unmarked-variables':
      return problem.names.length === 1
        ? `${problem.names[0]} is not a column of the prepared data. Mark it [latent] in the text if it is unmeasured, or rename it to match a column.`
        : `${problem.names.join(', ')} are not columns of the prepared data. Mark each [latent] in the text if it is unmeasured, or rename it to match a column.`
    case 'unknown-variable':
      return `${problem.name} is not a variable in this graph.`
    case 'variable-refused':
      return `${problem.name}: ${describeDagVariableEditProblem(problem.problem)}`
    case 'arrow-refused':
      return `${problem.from} → ${problem.to}: ${describeDagEditProblem(problem.problem)}`
  }
}
