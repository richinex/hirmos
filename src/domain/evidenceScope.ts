import { assertNever } from './dop'

/**
 * Which cells of a discovery run's raw evidence to show.
 *
 * A lag graph reports every ordered pair at every lag, so most cells are absences. Reading the
 * table is a question about a subset: the links the method kept, the cells that cleared the run's
 * own alpha, or one variable's row or column. The scope answers that question; the three keys
 * narrow it further and are independent of it.
 */
export type EvidenceScope =
  | { readonly kind: 'all' }
  | { readonly kind: 'discovered' }
  | { readonly kind: 'significant' }

export interface EvidenceSelection {
  readonly scope: EvidenceScope
  readonly source: string | null
  readonly target: string | null
  readonly lag: number | null
}

/** The fields a raw evidence row must carry to be selectable. */
export interface EvidenceCell {
  readonly source: string
  readonly target: string
  readonly lag: number
  /** The graph mark; empty when the method recorded no link for this cell. */
  readonly mark: string
  readonly p: number
}

export const NO_EVIDENCE_SELECTION: EvidenceSelection = { scope: { kind: 'all' }, source: null, target: null, lag: null }

export const EVIDENCE_SCOPES: readonly EvidenceScope[] = [{ kind: 'all' }, { kind: 'discovered' }, { kind: 'significant' }]

export function describeEvidenceScope(scope: EvidenceScope): string {
  switch (scope.kind) {
    case 'all': return 'All cells'
    case 'discovered': return 'Discovered relations'
    case 'significant': return 'Below alpha'
    default: return assertNever(scope)
  }
}

/**
 * Explain what a scope selects, for the control that offers it.
 *
 * Args:
 *     scope: The scope to describe
 *     alpha: The significance threshold the run was executed with
 */
export function explainEvidenceScope(scope: EvidenceScope, alpha: number): string {
  switch (scope.kind) {
    case 'all': return 'Every ordered pair at every lag, including the absences.'
    case 'discovered': return 'Cells the method recorded a mark for.'
    case 'significant': return `Cells whose p is below the run's own alpha of ${alpha}.`
    default: return assertNever(scope)
  }
}

function matchesScope(cell: EvidenceCell, scope: EvidenceScope, alpha: number): boolean {
  switch (scope.kind) {
    case 'all': return true
    case 'discovered': return cell.mark !== ''
    case 'significant': return cell.p < alpha
    default: return assertNever(scope)
  }
}

/**
 * Decide whether one evidence cell survives the current selection.
 *
 * Args:
 *     cell: The raw evidence cell
 *     selection: Scope plus any source, target or lag narrowing
 *     alpha: The significance threshold recorded with the run
 */
export function matchesEvidenceSelection(cell: EvidenceCell, selection: EvidenceSelection, alpha: number): boolean {
  if (!matchesScope(cell, selection.scope, alpha)) return false
  if (selection.source !== null && cell.source !== selection.source) return false
  if (selection.target !== null && cell.target !== selection.target) return false
  return selection.lag === null || cell.lag === selection.lag
}

/** True when the selection would hide nothing, so the table can skip filtering entirely. */
export function selectsEverything(selection: EvidenceSelection): boolean {
  return selection.scope.kind === 'all' && selection.source === null && selection.target === null && selection.lag === null
}

/** Summarise the active narrowing for the count line, or null when nothing is narrowed. */
export function describeEvidenceSelection(selection: EvidenceSelection): string | null {
  const parts: string[] = []
  if (selection.scope.kind !== 'all') parts.push(describeEvidenceScope(selection.scope).toLowerCase())
  if (selection.source !== null) parts.push(`source ${selection.source}`)
  if (selection.target !== null) parts.push(`target ${selection.target}`)
  if (selection.lag !== null) parts.push(`lag ${selection.lag}`)
  return parts.length === 0 ? null : parts.join(' · ')
}
