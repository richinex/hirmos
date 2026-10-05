import type { DatasetProfile } from '@/domain/dataset'
import { assertNever, err, ok, type Result } from '@/domain/dop'
import type { CountRegressionDraft, CountRegressionRequest } from '@/domain/countRegression'
import { cohortAdoption, lagDesign, type PanelProblem } from '@/domain/regularPanel'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import type { PreparedMatrix } from './prepared'
import { prepareRegularPanel } from './regularPanelInput'

type Design = CountRegressionRequest['design']
type Term = Extract<Design, { kind: 'lags' }>['terms'][number]

export async function prepareCountDesign(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
  matrix: PreparedMatrix,
  model: CountRegressionDraft['model'],
  covariates: Term[],
  current: () => boolean,
): Promise<Result<{ readonly design: Design; readonly periods: string[] }, PanelProblem>> {
  if (model.kind === 'interrupted') {
    if (prepared.kind !== 'prepared-time-series') return err({ kind: 'design-mismatch' })
    return ok({
      design: {
        kind: 'interrupted',
        intervention: Number(model.intervention) - 1,
        horizon: Number(model.horizon),
        bandwidth: Number(model.bandwidth),
        covariates,
      },
      periods: [],
    })
  }
  if (prepared.kind !== 'prepared-panel') return err({ kind: 'design-mismatch' })
  const result = await prepareRegularPanel(source, profile, prepared, matrix, current)
  if (!result.ok) return result
  const panel = result.value
  const periods = [...panel.periods]
  switch (model.kind) {
    case 'lags': {
      const column = matrix.columns.findIndex((candidate) => candidate.id === model.predictor)
      const predictor = matrix.columns[column]
      if (predictor === undefined) return err({ kind: 'predictor-missing' })
      const lags = model.lags.split(',').map((value) => Number(value.trim()))
      const terms = [
        ...lags.map((lag) => ({ column, lag, name: `${predictor.name} (lag ${lag})` })),
        ...covariates,
      ]
      return ok({
        design: lagDesign(
          panel,
          terms,
          lags.map((_, index) => index),
        ),
        periods,
      })
    }
    case 'events':
    case 'summary': {
      const column = matrix.columns.findIndex((candidate) => candidate.id === model.onset)
      if (column < 0) return err({ kind: 'onset-missing' })
      const adoption = cohortAdoption(panel, column)
      if (!adoption.ok) return adoption
      const cohort = panel.periods.indexOf(model.cohort)
      if (cohort < 1 || !adoption.value.some(([, period]) => period === cohort))
        return err({ kind: 'cohort-missing' })
      const common = {
        keys: panel.keys.map(([u, t]): [number, number] => [u, t]),
        adoption: adoption.value,
        cohort,
        covariates,
      }
      const design: Design =
        model.kind === 'summary'
          ? { kind: 'summary', ...common }
          : {
              kind: 'events',
              ...common,
              window:
                model.window.kind === 'all'
                  ? { kind: 'all' }
                  : {
                      kind: 'finite',
                      first: Number(model.window.first),
                      last: Number(model.window.last),
                    },
            }
      return ok({ design, periods })
    }
    default:
      return assertNever(model)
  }
}
