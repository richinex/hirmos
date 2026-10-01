import type { ColumnId } from '@/domain/dataset'
import { assertNever, err, ok, type Result } from '@/domain/dop'
import type { BackdoorLinearConfiguration } from '@/domain/estimation'
import type { AdjustedRegressionErrorModel, AdjustedRegressionFixedEffects } from '@/workers/analysisProtocol'

type Plan = { readonly errorModel: AdjustedRegressionErrorModel; readonly fixedEffects: AdjustedRegressionFixedEffects | null }
type Problem = { readonly kind: 'missing-grouping-column'; readonly column: ColumnId }

/** Resolve validated column identities at the worker boundary, without substituting another model. */
export function adjustedRegressionPlan(configuration: BackdoorLinearConfiguration, columns: ReadonlyMap<ColumnId, number>): Result<Plan, Problem> {
  let fixedEffects: AdjustedRegressionFixedEffects | null
  const effects = configuration.fixedEffects
  switch (effects.kind) {
    case 'none': fixedEffects = null; break
    case 'unit':
    case 'time': {
      const column = columns.get(effects.column)
      if (column === undefined) return err({ kind: 'missing-grouping-column', column: effects.column })
      fixedEffects = { kind: effects.kind, column }
      break
    }
    case 'unit-and-time': {
      const unit = columns.get(effects.column)
      const time = columns.get(effects.timeColumn)
      if (unit === undefined) return err({ kind: 'missing-grouping-column', column: effects.column })
      if (time === undefined) return err({ kind: 'missing-grouping-column', column: effects.timeColumn })
      fixedEffects = { kind: 'unitAndTime', unit, time }
      break
    }
    default: return assertNever(effects)
  }
  const errors = configuration.errors
  switch (errors.kind) {
    case 'classical':
    case 'hac': return ok({ fixedEffects, errorModel: { kind: 'neweyWest' } })
    case 'hc1': return ok({ fixedEffects, errorModel: { kind: 'hc1' } })
    case 'arma': return ok({ fixedEffects, errorModel: errors })
    case 'cluster': {
      const column = columns.get(errors.column)
      return column === undefined ? err({ kind: 'missing-grouping-column', column: errors.column }) : ok({ fixedEffects, errorModel: { kind: 'cluster', column } })
    }
    default: return assertNever(errors)
  }
}
