import type { Identification } from '@/domain/study'

export function adjustmentSummary(identification: Identification | null) {
  if (identification?.kind !== 'identified') return null
  const adjustment = identification.adjustment
  const guarantee = 'guarantee' in adjustment ? adjustment.guarantee : null
  return {
    kind: adjustment.kind,
    variables: adjustment.variables.map((variable) => variable.name),
    guarantee,
    meaning:
      adjustment.kind === 'o-set'
        ? guarantee === 'established'
          ? 'The O-set is graphically optimal: it gives the lowest asymptotic variance among valid adjustment sets for estimators covered by the theorem, under its assumptions. This is not a guarantee of the smallest standard error in this sample or for every estimator.'
          : 'The O-set is a valid recommendation, but graphical optimality is not established.'
        : 'This records the selected adjustment set, not an O-set choice. A larger valid set and an empty minimal set can both be valid; they do not conflict.',
    reference:
      adjustment.kind === 'o-set'
        ? 'Runge (2021), Theorem 3 and the estimator conditions linking adjustment information to asymptotic variance.'
        : null,
  }
}
