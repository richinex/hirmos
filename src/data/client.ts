import { assertNever, err, type Result } from '@/domain/dop'
import type { NonEmptyArray } from '@/domain/dop'
import {
  type ColumnId,
  type ColumnProfile,
  type ColumnProfileProblem,
  type DatasetProfile,
  type DatasetProfileProblem,
  type DatasetSummary,
  type DatasetSummaryProblem,
  type NullableNumericMatrix,
  type NumericMaterializationProblem,
  type TimeOrderedNumericMatrix,
  type TimeSeriesMaterializationProblem,
  type PreviewQuery,
  type PreviewWindow,
  type PreviewWindowProblem,
  parseColumnProfile,
  parseDatasetSummary,
  parsePreviewWindow,
  validateNumericMatrixAgainstProfile,
  validateTimeOrderedMatrixAgainstProfile,
} from '@/domain/dataset'
import type { ImportRequestId } from '@/domain/workflow'
import { newImportRequestId } from '@/domain/workflow'
import { parseDataWorkerEvent, type DataWorkerCommand } from '@/workers/dataProtocol'
import { parsePanelLongMatrix, parsePanelStructure, type PanelDataProblem, type PanelLongMatrix, type PanelStructureEvidence } from '@/domain/panel'

type ProfileOutcome = Result<DatasetProfile, DatasetProfileProblem>
type MaterializationOutcome = Result<NullableNumericMatrix, NumericMaterializationProblem>
type TimeSeriesMaterializationOutcome = Result<TimeOrderedNumericMatrix, TimeSeriesMaterializationProblem>
type ColumnProfileOutcome = Result<ColumnProfile, ColumnProfileProblem>
type SummaryOutcome = Result<DatasetSummary, DatasetSummaryProblem>
type PreviewWindowOutcome = Result<PreviewWindow, PreviewWindowProblem>
type PanelStructureOutcome = Result<PanelStructureEvidence, PanelDataProblem>
type PanelMaterializationOutcome = Result<PanelLongMatrix, PanelDataProblem>
type PendingRequest =
  | { readonly kind: 'profile'; readonly resolve: (outcome: ProfileOutcome) => void }
  | { readonly kind: 'summary'; readonly profile: DatasetProfile; readonly resolve: (outcome: SummaryOutcome) => void }
  | { readonly kind: 'preview-window'; readonly profile: DatasetProfile; readonly resolve: (outcome: PreviewWindowOutcome) => void }
  | {
      readonly kind: 'column-profile'
      readonly profile: DatasetProfile
      readonly resolve: (outcome: ColumnProfileOutcome) => void
    }
  | {
      readonly kind: 'materialization'
      readonly profile: DatasetProfile
      readonly resolve: (outcome: MaterializationOutcome) => void
    }
  | { readonly kind: 'time-series-materialization'; readonly profile: DatasetProfile; readonly resolve: (outcome: TimeSeriesMaterializationOutcome) => void }
  | { readonly kind: 'panel-inspection'; readonly profile: DatasetProfile; readonly resolve: (outcome: PanelStructureOutcome) => void }
  | { readonly kind: 'panel-materialization'; readonly profile: DatasetProfile; readonly resolve: (outcome: PanelMaterializationOutcome) => void }

let worker: Worker | null = null
const pending = new Map<ImportRequestId, PendingRequest>()

type SharedWorkerProblem =
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }

const stopAll = (problem: SharedWorkerProblem) => {
  for (const request of pending.values()) {
    switch (request.kind) {
      case 'profile': request.resolve(err(problem)); break
      case 'summary': request.resolve(err(problem)); break
      case 'preview-window': request.resolve(err(problem)); break
      case 'column-profile': request.resolve(err(problem)); break
      case 'materialization': request.resolve(err(problem)); break
      case 'time-series-materialization': request.resolve(err(problem)); break
      case 'panel-inspection': request.resolve(err(problem)); break
      case 'panel-materialization': request.resolve(err(problem)); break
      default: assertNever(request)
    }
  }
  pending.clear()
  worker?.terminate()
  worker = null
}

const failAll = (detail: string) => stopAll({ kind: 'worker-protocol-failed', detail })

const dataWorker = (): Worker => {
  if (worker) return worker
  const created = new Worker(new URL('../workers/data.worker.ts', import.meta.url), {
    type: 'module',
    name: 'hirmos-data',
  })
  created.onmessage = (message: MessageEvent<unknown>) => {
    const parsed = parseDataWorkerEvent(message.data)
    if (!parsed.ok) {
      failAll(parsed.error.detail)
      return
    }
    if (parsed.value.kind === 'protocol-failed') {
      failAll(parsed.value.detail)
      return
    }
    const waiting = pending.get(parsed.value.request)
    if (!waiting) return
    if (waiting.kind === 'profile') {
      if (parsed.value.kind !== 'profile-succeeded' && parsed.value.kind !== 'profile-failed') {
        failAll('The data worker returned a materialization response for a profile request.')
        return
      }
      pending.delete(parsed.value.request)
      waiting.resolve(parsed.value.kind === 'profile-succeeded'
        ? { ok: true, value: parsed.value.profile }
        : { ok: false, error: parsed.value.problem })
      return
    }
    if (waiting.kind === 'summary') {
      if (parsed.value.kind !== 'summary-succeeded' && parsed.value.kind !== 'summary-failed') {
        failAll('The data worker returned another response for a summary request.')
        return
      }
      pending.delete(parsed.value.request)
      if (parsed.value.kind === 'summary-failed') {
        waiting.resolve({ ok: false, error: parsed.value.problem })
        return
      }
      const bound = parseDatasetSummary(parsed.value.summary, waiting.profile)
      waiting.resolve(bound.ok ? { ok: true, value: bound.value } : { ok: false, error: { kind: 'worker-protocol-failed', detail: bound.error.detail } })
      return
    }
    if (waiting.kind === 'preview-window') {
      if (parsed.value.kind !== 'preview-window-succeeded' && parsed.value.kind !== 'preview-window-failed') {
        failAll('The data worker returned another response for a preview request.')
        return
      }
      pending.delete(parsed.value.request)
      if (parsed.value.kind === 'preview-window-failed') {
        waiting.resolve({ ok: false, error: parsed.value.problem })
        return
      }
      const bound = parsePreviewWindow(parsed.value.window, waiting.profile)
      waiting.resolve(bound.ok ? { ok: true, value: bound.value } : { ok: false, error: { kind: 'worker-protocol-failed', detail: bound.error.detail } })
      return
    }
    if (waiting.kind === 'column-profile') {
      if (parsed.value.kind !== 'column-profile-succeeded' && parsed.value.kind !== 'column-profile-failed') {
        failAll('The data worker returned another response for a column profile request.')
        return
      }
      pending.delete(parsed.value.request)
      if (parsed.value.kind === 'column-profile-failed') {
        waiting.resolve({ ok: false, error: parsed.value.problem })
        return
      }
      const bound = parseColumnProfile(parsed.value.profile, waiting.profile)
      waiting.resolve(bound.ok
        ? { ok: true, value: bound.value }
        : { ok: false, error: { kind: 'worker-protocol-failed', detail: bound.error.detail } })
      return
    }
    if (waiting.kind === 'panel-inspection') {
      if (parsed.value.kind !== 'panel-inspection-succeeded' && parsed.value.kind !== 'panel-data-failed') { failAll('The data worker returned another response for a panel inspection.'); return }
      pending.delete(parsed.value.request)
      if (parsed.value.kind === 'panel-data-failed') { waiting.resolve({ ok: false, error: parsed.value.problem }); return }
      const checked = parsePanelStructure(parsed.value.structure, waiting.profile)
      waiting.resolve(checked.ok ? checked : { ok: false, error: { kind: 'worker-protocol-failed', detail: checked.error.detail } })
      return
    }
    if (waiting.kind === 'panel-materialization') {
      if (parsed.value.kind !== 'panel-materialization-succeeded' && parsed.value.kind !== 'panel-data-failed') { failAll('The data worker returned another response for a panel materialization.'); return }
      pending.delete(parsed.value.request)
      if (parsed.value.kind === 'panel-data-failed') { waiting.resolve({ ok: false, error: parsed.value.problem }); return }
      const checked = parsePanelLongMatrix(parsed.value.matrix, waiting.profile)
      waiting.resolve(checked.ok ? checked : { ok: false, error: { kind: 'worker-protocol-failed', detail: checked.error.detail } })
      return
    }
    if (waiting.kind === 'time-series-materialization') {
      if (parsed.value.kind !== 'time-series-materialization-succeeded' && parsed.value.kind !== 'materialization-failed') {
        failAll('The data worker returned another response for a time-series materialization request.')
        return
      }
      pending.delete(parsed.value.request)
      if (parsed.value.kind === 'materialization-failed') { waiting.resolve({ ok: false, error: parsed.value.problem }); return }
      const checked = validateTimeOrderedMatrixAgainstProfile(parsed.value.matrix, waiting.profile)
      waiting.resolve(checked.ok ? checked : { ok: false, error: { kind: 'worker-protocol-failed', detail: checked.error.detail } })
      return
    }
    if (parsed.value.kind !== 'materialization-succeeded' && parsed.value.kind !== 'materialization-failed') {
      failAll('The data worker returned a profile response for a materialization request.')
      return
    }
    pending.delete(parsed.value.request)
    if (parsed.value.kind === 'materialization-failed') {
      waiting.resolve(parsed.value.problem.kind === 'time-value-unparseable' || parsed.value.problem.kind === 'duplicate-time-value'
        ? { ok: false, error: { kind: 'worker-protocol-failed', detail: 'The data worker returned a time-axis failure for a numeric materialization.' } }
        : { ok: false, error: parsed.value.problem })
      return
    }
    const checked = validateNumericMatrixAgainstProfile(parsed.value.matrix, waiting.profile)
    waiting.resolve(checked.ok
      ? { ok: true, value: checked.value }
      : { ok: false, error: { kind: 'worker-protocol-failed', detail: checked.error.detail } })
  }
  created.onerror = (event) => {
    event.preventDefault()
    const detail = event.message || 'The data worker stopped unexpectedly.'
    stopAll({ kind: 'worker-unavailable', detail })
  }
  worker = created
  return created
}

export function profileSourceInWorker(
  request: ImportRequestId,
  file: File,
): Promise<ProfileOutcome> {
  if (pending.has(request)) {
    return Promise.resolve(err({
      kind: 'worker-protocol-failed',
      detail: 'The import request identity is already active.',
    }))
  }

  return new Promise((resolve) => {
    pending.set(request, { kind: 'profile', resolve })
    const command: DataWorkerCommand = { kind: 'profile-source', request, file }
    try {
      dataWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({
        kind: 'worker-unavailable',
        detail: cause instanceof Error ? cause.message : String(cause),
      }))
    }
  })
}

export function materializeNumericColumnsInWorker(
  file: File,
  profile: DatasetProfile,
  columnIds: NonEmptyArray<ColumnId>,
): Promise<MaterializationOutcome> {
  const request = newImportRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'materialization', profile, resolve })
    const command: DataWorkerCommand = {
      kind: 'materialize-numeric',
      request,
      file,
      profile,
      columnIds,
    }
    try {
      dataWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({
        kind: 'worker-unavailable',
        detail: cause instanceof Error ? cause.message : String(cause),
      }))
    }
  })
}

export function materializeTimeSeriesColumnsInWorker(
  file: File,
  profile: DatasetProfile,
  timeColumn: ColumnId,
  columnIds: NonEmptyArray<ColumnId>,
): Promise<TimeSeriesMaterializationOutcome> {
  const request = newImportRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'time-series-materialization', profile, resolve })
    const command: DataWorkerCommand = { kind: 'materialize-time-series', request, file, profile, timeColumn, columnIds }
    try {
      dataWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function profileColumnInWorker(
  file: File,
  profile: DatasetProfile,
  columnId: ColumnId,
): Promise<ColumnProfileOutcome> {
  const request = newImportRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'column-profile', profile, resolve })
    const command: DataWorkerCommand = { kind: 'profile-column', request, file, profile, columnId }
    try {
      dataWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function summarizeColumnsInWorker(file: File, profile: DatasetProfile): Promise<SummaryOutcome> {
  const request = newImportRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'summary', profile, resolve })
    const command: DataWorkerCommand = { kind: 'summarize-columns', request, file, profile }
    try {
      dataWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function previewWindowInWorker(file: File, profile: DatasetProfile, query: PreviewQuery): Promise<PreviewWindowOutcome> {
  const request = newImportRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'preview-window', profile, resolve })
    const command: DataWorkerCommand = { kind: 'preview-window', request, file, profile, query }
    try {
      dataWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function inspectPanelInWorker(file: File, profile: DatasetProfile, unitColumn: ColumnId, timeColumn: ColumnId): Promise<PanelStructureOutcome> {
  const request = newImportRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'panel-inspection', profile, resolve })
    const command: DataWorkerCommand = { kind: 'inspect-panel', request, file, profile, unitColumn, timeColumn }
    try { dataWorker().postMessage(command) } catch (cause: unknown) {
      pending.delete(request); resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function materializePanelInWorker(file: File, profile: DatasetProfile, columns: { readonly unit: ColumnId; readonly time: ColumnId; readonly outcome: ColumnId; readonly treatment: ColumnId }): Promise<PanelMaterializationOutcome> {
  const request = newImportRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'panel-materialization', profile, resolve })
    const command: DataWorkerCommand = { kind: 'materialize-panel', request, file, profile, unitColumn: columns.unit, timeColumn: columns.time, outcomeColumn: columns.outcome, treatmentColumn: columns.treatment }
    try { dataWorker().postMessage(command) } catch (cause: unknown) {
      pending.delete(request); resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}
