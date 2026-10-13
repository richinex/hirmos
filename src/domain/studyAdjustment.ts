import { backdoorIdentificationCommand, type StudySpecification } from './study'
import type { AdjustmentValidationDesign } from './adjustmentValidation'

export function studyAdjustmentDesign(study: StudySpecification): AdjustmentValidationDesign {
  const command = backdoorIdentificationCommand(study)
  return {
    nodes: command.nodes,
    edges: command.edges.map(([cause, effect]) => [cause, effect]),
    treatment: command.treatment,
    outcome: command.outcome,
    unobserved: [...command.unobserved],
    sets: [],
  }
}
