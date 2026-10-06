import type { DatasetProfile, ColumnId } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { err, ok, isNonEmpty, type Result } from '@/domain/dop'
import {
  baconRequestSchema,
  panelRegressionRequestSchema,
  type PanelRegressionDraft,
  type PanelRegressionRequest,
  type BaconRequest,
} from '@/domain/panelRegression'
import { cohortAdoption, describePanelProblem } from '@/domain/regularPanel'
import { describePanelDataProblem } from '@/domain/panel'
import { materialisePrepared, describePreparedMaterialisationProblem } from './prepared'
import { prepareRegularPanel } from './regularPanelInput'
import { materializePanelKeysInWorker } from './client'

type Variable = { readonly id: ColumnId; readonly name: string }
type Input =
  | {
      readonly kind: 'regression'
      readonly values: Float64Array
      readonly request: PanelRegressionRequest
      readonly variables: Variable[]
    }
  | {
      readonly kind: 'bacon'
      readonly values: Float64Array
      readonly request: BaconRequest
      readonly variables: Variable[]
      readonly periods: string[]
    }

export async function preparePanelRegression(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
  draft: PanelRegressionDraft,
  current: () => boolean,
): Promise<Result<Input, string>> {
  if (draft.outcome === null) return err('Choose an outcome.')
  const model = draft.model
  const roles =
    model.kind === 'interactions'
      ? model.predictors.map((p) => p.column)
      : [model.onset, ...model.covariates]
  const ids = [
    ...new Set(
      [
        draft.outcome,
        ...roles,
        ...(model.kind === 'bacon' ? [] : [draft.weight, draft.cluster]),
      ].filter((v): v is ColumnId => v !== null),
    ),
  ]
  if (!isNonEmpty(ids)) return err('Choose model columns.')
  const read = await materialisePrepared(source, profile, prepared, ids)
  if (!current()) return err('The analysis was cancelled.')
  if (!read.ok) return err(describePreparedMaterialisationProblem(read.error))
  const matrix = read.value,
    variables = matrix.columns.map((c) => ({ id: c.id, name: c.name }))
  const at = (id: ColumnId | null) => (id === null ? -1 : variables.findIndex((c) => c.id === id))
  if (model.kind !== 'interactions') {
    if (prepared.kind !== 'prepared-panel')
      return err('Prepare a panel with unit and period keys for this analysis.')
    if (model.onset === null) return err('Choose the absorbing treatment indicator.')
    const regular = await prepareRegularPanel(
      source,
      profile,
      prepared,
      matrix,
      current,
      model.kind === 'bacon' ? 'evenly-spaced' : 'consecutive',
    )
    if (!current()) return err('The analysis was cancelled.')
    if (!regular.ok) return err(describePanelProblem(regular.error))
    const panel = regular.value
    if (model.kind === 'bacon') {
      const parsed = baconRequestSchema.safeParse({
        rows: matrix.rowCount,
        columns: variables.length,
        units: panel.keys.map(([u]) => String(u)),
        times: panel.keys.map(([, t]) => t),
        outcome: at(draft.outcome),
        treatment: at(model.onset),
        specification:
          model.covariates.length === 0
            ? { kind: 'unadjusted' }
            : { kind: 'adjusted', controls: model.covariates.map(at) },
      })
      if (!parsed.success) return err(parsed.error.message)
      // Bacon's existing Rust boundary takes row-major values; prepared matrices are column-major.
      const values = Float64Array.from(
        { length: matrix.values.length },
        (_, i) =>
          matrix.values[
            (i % variables.length) * matrix.rowCount + Math.floor(i / variables.length)
          ]!,
      )
      return ok({
        kind: 'bacon',
        values,
        request: parsed.data,
        variables,
        periods: [...panel.periods],
      })
    }
    const adoption = cohortAdoption(panel, at(model.onset))
    if (!adoption.ok) return err(describePanelProblem(adoption.error))
    return regression(
      {
        kind: 'eventStudy',
        keys: panel.keys.map(([u, t]) => [u, t]),
        adoption: adoption.value,
        covariates: model.covariates.map(at),
        window: {
          first: Number(model.first),
          last: Number(model.last),
          reference: Number(model.reference),
          tails: model.tails,
        },
      },
      panel.keys.map(([u]) => u),
    )
  }
  let units: number[] | null = null
  if (draft.cluster === null) {
    if (prepared.kind !== 'prepared-panel')
      return err(
        'Choose a cluster column. Rows within a cluster may have correlated errors; clusters are treated as independent.',
      )
    const keys = await materializePanelKeysInWorker(
      source.file,
      profile,
      prepared.sampling.unitColumn,
      prepared.sampling.timeColumn,
    )
    if (!current()) return err('The analysis was cancelled.')
    if (!keys.ok) return err(describePanelDataProblem(keys.error))
    if (
      keys.value.sourceFingerprint !== profile.source.fingerprint ||
      keys.value.rowCount !== matrix.rowCount
    )
      return err('The cluster keys do not match the prepared rows.')
    const codes = new Map<string, number>()
    units = keys.value.units.map((u) => {
      if (!codes.has(u)) codes.set(u, codes.size)
      return codes.get(u)!
    })
  }
  return regression(
    {
      kind: 'interactions',
      predictors: model.predictors.map((p) => ({
        column: at(p.column),
        name: variables[at(p.column)]?.name ?? '',
        coding: p.coding,
      })),
      terms: model.terms.map((t) =>
        t.map((id) => model.predictors.findIndex((p) => p.column === id)),
      ),
    },
    units,
  )

  function regression(
    specification: PanelRegressionRequest['specification'],
    units: readonly number[] | null,
  ): Result<Input, string> {
    let values = matrix.values
    const names = variables.map((v) => v.name)
    let cluster = at(draft.cluster)
    if (draft.cluster === null) {
      if (units === null) return err('Choose a cluster column.')
      cluster = names.length
      let label = 'Panel unit (cluster)'
      while (names.includes(label)) label += ' '
      names.push(label)
      values = new Float64Array(matrix.values.length + matrix.rowCount)
      values.set(matrix.values)
      values.set(units, matrix.values.length)
    }
    const parsed = panelRegressionRequestSchema.safeParse({
      rows: matrix.rowCount,
      columns: names.length,
      names,
      outcome: at(draft.outcome),
      weights: draft.weight === null ? null : at(draft.weight),
      cluster,
      confidence: Number(draft.confidence),
      specification,
    })
    return parsed.success
      ? ok({ kind: 'regression', values, request: parsed.data, variables })
      : err(parsed.error.issues.map((i) => i.message).join(' '))
  }
}
