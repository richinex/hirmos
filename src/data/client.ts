import { err, type Result } from '@/domain/dop'
import type { NonEmptyArray } from '@/domain/dop'
import {
  validateNumericMatrixAgainstProfile,
  type ColumnId,
  type DatasetProfile,
  type DatasetProfileProblem,
  type NullableNumericMatrix,
  type NumericMaterializationProblem,
} from '@/domain/dataset'
import type { ImportRequestId } from '@/domain/workflow'
import { newImportRequestId } from '@/domain/workflow'
import { parseDataWorkerEvent, type DataWorkerCommand } from '@/workers/dataProtocol'

type ProfileOutcome = Result<DatasetProfile, DatasetProfileProblem>
type MaterializationOutcome = Result<NullableNumericMatrix, NumericMaterializationProblem>
type PendingRequest =
  | { readonly kind: 'profile'; readonly resolve: (outcome: ProfileOutcome) => void }
  | {
      readonly kind: 'materialization'
      readonly profile: DatasetProfile
      readonly resolve: (outcome: MaterializationOutcome) => void
    }

let worker: Worker | null = null
const pending = new Map<ImportRequestId, PendingRequest>()

const failAll = (detail: string) => {
  for (const request of pending.values()) {
    request.resolve(err({ kind: 'worker-protocol-failed', detail }))
  }
  pending.clear()
  worker?.terminate()
  worker = null
}

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
    if (parsed.value.kind !== 'materialization-succeeded' && parsed.value.kind !== 'materialization-failed') {
      failAll('The data worker returned a profile response for a materialization request.')
      return
    }
    pending.delete(parsed.value.request)
    if (parsed.value.kind === 'materialization-failed') {
      waiting.resolve({ ok: false, error: parsed.value.problem })
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
    for (const request of pending.values()) request.resolve(err({ kind: 'worker-unavailable', detail }))
    pending.clear()
    worker?.terminate()
    worker = null
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
