import type { DatasetProfile, ColumnId } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { err, ok, isNonEmpty, type Result, type NonEmptyArray } from '@/domain/dop'
import {
  predictorSyntheticCatalog,
  predictorSyntheticModel,
  type PredictorSyntheticConfiguration,
  type PredictorSyntheticCatalog,
  type PredictorSyntheticRequest,
} from '@/domain/predictorSyntheticControl'
import { describePanelDataProblem } from '@/domain/panel'
import { materialisePrepared, describePreparedMaterialisationProblem } from './prepared'
import { materializePanelKeysInWorker } from './client'

type Column = { readonly column: ColumnId; readonly name: string }
type Input = {
  readonly values: Float64Array
  readonly model: PredictorSyntheticRequest
  readonly catalog: PredictorSyntheticCatalog
  readonly columns: NonEmptyArray<Column>
}
export async function preparePredictorSynthetic(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
  configuration: PredictorSyntheticConfiguration,
  outcome: ColumnId,
  treatment: ColumnId,
  current: () => boolean,
): Promise<Result<Input, string>> {
  if (prepared.kind !== 'prepared-panel')
    return err('Prepare a long panel with unit and period keys.')
  const ids = [
    ...new Set([
      outcome,
      treatment,
      ...configuration.predictors,
      ...configuration.special.map((p) => p.column),
    ]),
  ]
  if (!isNonEmpty(ids)) return err('Choose the outcome and predictors.')
  const [read, keys] = await Promise.all([
    materialisePrepared(source, profile, prepared, ids),
    materializePanelKeysInWorker(
      source.file,
      profile,
      prepared.sampling.unitColumn,
      prepared.sampling.timeColumn,
    ),
  ])
  if (!current()) return err('The analysis was cancelled.')
  if (!read.ok) return err(describePreparedMaterialisationProblem(read.error))
  if (!keys.ok) return err(describePanelDataProblem(keys.error))
  const matrix = read.value
  if (
    keys.value.sourceFingerprint !== profile.source.fingerprint ||
    keys.value.rowCount !== matrix.rowCount ||
    matrix.leadingRowsRemoved !== 0
  )
    return err(
      'The panel keys do not match the prepared rows. Prepare the panel without deleting individual rows.',
    )
  const catalog = predictorSyntheticCatalog(keys.value)
  const model = predictorSyntheticModel(configuration, catalog, keys.value, matrix.columns, outcome)
  if (!model.ok) return model
  const treatmentIndex = matrix.columns.findIndex((column) => column.id === treatment)
  const donorCodes = new Set(model.value.donors)
  const periods = new Set([...model.value.fitPeriods, ...model.value.plotPeriods])
  for (let row = 0; row < matrix.rowCount; row++) {
    const unit = model.value.units[row]!,
      time = model.value.times[row]!
    const value = matrix.values[treatmentIndex * matrix.rowCount + row]!
    if (value !== 0 && value !== 1)
      return err(
        'The study treatment must be a binary intervention indicator for this synthetic-control comparison.',
      )
    if (!periods.has(time)) continue
    const expected =
      unit === model.value.treated && time >= configuration.interventionPeriod! ? 1 : 0
    if ((unit === model.value.treated || donorCodes.has(unit) || value === 1) && value !== expected)
      return err(
        'The treatment indicator must identify only the selected treated unit from the intervention period onward, with untreated donors. The comparison must match the recorded ATT study.',
      )
  }
  const columns = matrix.columns.map((column) => ({ column: column.id, name: column.name }))
  if (!isNonEmpty(columns)) return err('No prepared columns were read.')
  return ok({ values: matrix.values, model: model.value, catalog, columns })
}
