import { z } from 'zod'
import {
  parseDatasetProfile,
  parseNullableNumericMatrix,
  type DatasetProfile,
  type DatasetProfileProblem,
  type NullableNumericMatrix,
  type NumericMaterializationProblem,
  type ColumnId,
} from '@/domain/dataset'
import { err, isNonEmpty, ok, type Result } from '@/domain/dop'
import { importRequestId, type ImportRequestId } from '@/domain/workflow'

export type DataWorkerCommand =
  | {
      readonly kind: 'profile-source'
      readonly request: ImportRequestId
      readonly file: File
    }
  | {
      readonly kind: 'materialize-numeric'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly columnIds: readonly [ColumnId, ...ColumnId[]]
    }

export type DataWorkerEvent =
  | { readonly kind: 'profile-succeeded'; readonly request: ImportRequestId; readonly profile: DatasetProfile }
  | { readonly kind: 'profile-failed'; readonly request: ImportRequestId; readonly problem: DatasetProfileProblem }
  | { readonly kind: 'materialization-succeeded'; readonly request: ImportRequestId; readonly matrix: NullableNumericMatrix }
  | { readonly kind: 'materialization-failed'; readonly request: ImportRequestId; readonly problem: NumericMaterializationProblem }
  | { readonly kind: 'protocol-failed'; readonly detail: string }

export type DataProtocolProblem =
  | { readonly kind: 'invalid-command'; readonly detail: string }
  | { readonly kind: 'invalid-event'; readonly detail: string }

const requestSchema = z.string().uuid()

const commandSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('profile-source'),
    request: requestSchema,
    file: z.instanceof(File),
  }).strict(),
  z.object({
    kind: z.literal('materialize-numeric'),
    request: requestSchema,
    file: z.instanceof(File),
    profile: z.unknown(),
    columnIds: z.array(z.string()).min(1),
  }).strict(),
])

const profileProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('fingerprint-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('engine-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('registration-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('parse-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('cleanup-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('empty-dataset') }).strict(),
  z.object({ kind: z.literal('no-columns') }).strict(),
  z.object({ kind: z.literal('unsafe-row-count'), value: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

const materializationProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() }).strict(),
  z.object({ kind: z.literal('column-not-found'), id: z.string() }).strict(),
  z.object({ kind: z.literal('duplicate-column'), id: z.string() }).strict(),
  z.object({ kind: z.literal('non-numeric-column'), name: z.string(), duckdbType: z.string() }).strict(),
  z.object({ kind: z.literal('non-finite-value'), name: z.string(), row: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('materialization-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

const eventSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('profile-succeeded'),
    request: requestSchema,
    profile: z.unknown(),
  }).strict(),
  z.object({
    kind: z.literal('profile-failed'),
    request: requestSchema,
    problem: profileProblemSchema,
  }).strict(),
  z.object({
    kind: z.literal('materialization-succeeded'),
    request: requestSchema,
    matrix: z.unknown(),
  }).strict(),
  z.object({
    kind: z.literal('materialization-failed'),
    request: requestSchema,
    problem: materializationProblemSchema,
  }).strict(),
  z.object({ kind: z.literal('protocol-failed'), detail: z.string() }).strict(),
])

export function parseDataWorkerCommand(value: unknown): Result<DataWorkerCommand, DataProtocolProblem> {
  const parsed = commandSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-command', detail: z.prettifyError(parsed.error) })
  const request = importRequestId(parsed.data.request)
  if (!request.ok) return err({ kind: 'invalid-command', detail: 'The import request identity is invalid.' })
  if (parsed.data.kind === 'profile-source') {
    return ok({ kind: 'profile-source', request: request.value, file: parsed.data.file })
  }

  const profile = parseDatasetProfile(parsed.data.profile)
  if (!profile.ok) return err({ kind: 'invalid-command', detail: profile.error.detail })
  const byId = new Map<string, ColumnId>(profile.value.columns.map((column) => [column.id, column.id]))
  const columnIds: ColumnId[] = []
  for (const rawId of parsed.data.columnIds) {
    const id = byId.get(rawId)
    if (!id) return err({ kind: 'invalid-command', detail: `Column identity ${rawId} is not in the supplied profile.` })
    columnIds.push(id)
  }
  if (!isNonEmpty(columnIds)) return err({ kind: 'invalid-command', detail: 'At least one numeric column is required.' })
  return ok({
    kind: 'materialize-numeric',
    request: request.value,
    file: parsed.data.file,
    profile: profile.value,
    columnIds,
  })
}

export function parseDataWorkerEvent(value: unknown): Result<DataWorkerEvent, DataProtocolProblem> {
  const parsed = eventSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-event', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind === 'protocol-failed') return ok(parsed.data)

  const request = importRequestId(parsed.data.request)
  if (!request.ok) return err({ kind: 'invalid-event', detail: 'The import request identity is invalid.' })
  if (parsed.data.kind === 'profile-failed' || parsed.data.kind === 'materialization-failed') {
    return ok({ ...parsed.data, request: request.value })
  }
  if (parsed.data.kind === 'materialization-succeeded') {
    const matrix = parseNullableNumericMatrix(parsed.data.matrix)
    return matrix.ok
      ? ok({ kind: 'materialization-succeeded', request: request.value, matrix: matrix.value })
      : err({ kind: 'invalid-event', detail: matrix.error.detail })
  }

  const profile = parseDatasetProfile(parsed.data.profile)
  return profile.ok
    ? ok({ kind: 'profile-succeeded', request: request.value, profile: profile.value })
    : err({ kind: 'invalid-event', detail: profile.error.detail })
}
