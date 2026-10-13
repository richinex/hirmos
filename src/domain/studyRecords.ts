import type { IdentificationArtifact, StudySpecification } from './study'
import { assertNever } from './dop'

function identificationDecision({ method, result }: IdentificationArtifact) {
  switch (result.kind) {
    case 'identified':
      return { method, kind: result.kind, adjustment: result.adjustment }
    case 'cutoff-design':
    case 'backdoor-not-identified':
      return { method, kind: result.kind }
    case 'instrument-identified':
      return { method, kind: result.kind, instruments: result.instruments }
    case 'graphically-identified':
      return {
        method,
        kind: result.kind,
        expression: result.expression,
        projection: result.projection,
        frontdoor: result.frontdoor,
        instruments: result.instruments,
      }
    case 'counterfactually-identified':
      return {
        method,
        kind: result.kind,
        treatedExpression: result.treatedExpression,
        untreatedExpression: result.untreatedExpression,
        projection: result.projection,
      }
    default:
      return assertNever(result)
  }
}

function recordContent(value: unknown): string {
  return JSON.stringify(value, (_key, item) =>
    item !== null && typeof item === 'object' && !Array.isArray(item)
      ? Object.fromEntries(Object.entries(item).sort(([a], [b]) => a.localeCompare(b)))
      : item,
  )
}

export function sameStudySpecification(a: StudySpecification, b: StudySpecification): boolean {
  const { id: _aId, createdAt: _aTime, ...left } = a
  const { id: _bId, createdAt: _bTime, ...right } = b
  return recordContent(left) === recordContent(right)
}

export function existingStudyRecord(
  studies: readonly StudySpecification[],
  identifications: readonly IdentificationArtifact[],
  study: StudySpecification,
  identification: IdentificationArtifact,
): IdentificationArtifact | undefined {
  return identifications.find((record) => {
    const saved = studies.find((item) => item.id === record.study)
    return (
      saved !== undefined &&
      sameStudySpecification(saved, study) &&
      recordContent(identificationDecision(record)) ===
        recordContent(identificationDecision(identification))
    )
  })
}
