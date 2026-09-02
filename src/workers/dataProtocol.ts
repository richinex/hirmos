import { z } from 'zod'
import {
  parseColumnProfileShape,
  parseDatasetProfile,
  parseNullableNumericMatrix,
  parseTimeOrderedNumericMatrix,
  previewQuerySchema,
  type ColumnProfile,
  type ColumnProfileProblem,
  type DatasetProfile,
  type DatasetProfileProblem,
  type DatasetSummaryProblem,
  type NullableNumericMatrix,
  type TimeOrderedNumericMatrix,
  type NumericMaterializationProblem,
  type TimeSeriesMaterializationProblem,
  type ColumnId,
  type PreviewFilter,
  type PreviewQuery,
  type PreviewSort,
  type PreviewWindowProblem,
} from '@/domain/dataset'
import { err, isNonEmpty, ok, type Result } from '@/domain/dop'
import { importRequestId, type ImportRequestId } from '@/domain/workflow'
import { panelDataProblemSchema, parsePanelDataProblem, type PanelDataProblem } from '@/domain/panel'

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
  | {
      readonly kind: 'materialize-time-series'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly timeColumn: ColumnId
      readonly columnIds: readonly [ColumnId, ...ColumnId[]]
    }
  | {
      readonly kind: 'profile-column'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly columnId: ColumnId
    }
  | {
      readonly kind: 'summarize-columns'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
    }
  | {
      readonly kind: 'preview-window'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly query: PreviewQuery
    }
  | { readonly kind: 'inspect-panel'; readonly request: ImportRequestId; readonly file: File; readonly profile: DatasetProfile; readonly unitColumn: ColumnId; readonly timeColumn: ColumnId }
  | { readonly kind: 'materialize-panel'; readonly request: ImportRequestId; readonly file: File; readonly profile: DatasetProfile; readonly unitColumn: ColumnId; readonly timeColumn: ColumnId; readonly outcomeColumn: ColumnId; readonly treatmentColumn: ColumnId }

export type DataWorkerEvent =
  | { readonly kind: 'profile-succeeded'; readonly request: ImportRequestId; readonly profile: DatasetProfile }
  | { readonly kind: 'profile-failed'; readonly request: ImportRequestId; readonly problem: DatasetProfileProblem }
  | { readonly kind: 'materialization-succeeded'; readonly request: ImportRequestId; readonly matrix: NullableNumericMatrix }
  | { readonly kind: 'time-series-materialization-succeeded'; readonly request: ImportRequestId; readonly matrix: TimeOrderedNumericMatrix }
  | { readonly kind: 'materialization-failed'; readonly request: ImportRequestId; readonly problem: NumericMaterializationProblem | TimeSeriesMaterializationProblem }
  | { readonly kind: 'column-profile-succeeded'; readonly request: ImportRequestId; readonly profile: ColumnProfile }
  | { readonly kind: 'column-profile-failed'; readonly request: ImportRequestId; readonly problem: ColumnProfileProblem }
  | { readonly kind: 'summary-succeeded'; readonly request: ImportRequestId; readonly summary: unknown }
  | { readonly kind: 'summary-failed'; readonly request: ImportRequestId; readonly problem: DatasetSummaryProblem }
  | { readonly kind: 'preview-window-succeeded'; readonly request: ImportRequestId; readonly window: unknown }
  | { readonly kind: 'preview-window-failed'; readonly request: ImportRequestId; readonly problem: PreviewWindowProblem }
  | { readonly kind: 'panel-inspection-succeeded'; readonly request: ImportRequestId; readonly structure: unknown }
  | { readonly kind: 'panel-materialization-succeeded'; readonly request: ImportRequestId; readonly matrix: unknown }
  | { readonly kind: 'panel-data-failed'; readonly request: ImportRequestId; readonly problem: PanelDataProblem }
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
  z.object({
    kind: z.literal('materialize-time-series'),
    request: requestSchema,
    file: z.instanceof(File),
    profile: z.unknown(),
    timeColumn: z.string(),
    columnIds: z.array(z.string()).min(1),
  }).strict(),
  z.object({
    kind: z.literal('profile-column'),
    request: requestSchema,
    file: z.instanceof(File),
    profile: z.unknown(),
    columnId: z.string(),
  }).strict(),
  z.object({
    kind: z.literal('summarize-columns'),
    request: requestSchema,
    file: z.instanceof(File),
    profile: z.unknown(),
  }).strict(),
  z.object({
    kind: z.literal('preview-window'),
    request: requestSchema,
    file: z.instanceof(File),
    profile: z.unknown(),
    query: previewQuerySchema,
  }).strict(),
  z.object({ kind: z.literal('inspect-panel'), request: requestSchema, file: z.instanceof(File), profile: z.unknown(), unitColumn: z.string(), timeColumn: z.string() }).strict(),
  z.object({ kind: z.literal('materialize-panel'), request: requestSchema, file: z.instanceof(File), profile: z.unknown(), unitColumn: z.string(), timeColumn: z.string(), outcomeColumn: z.string(), treatmentColumn: z.string() }).strict(),
])

const columnProfileProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('column-not-found'), id: z.string() }).strict(),
  z.object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() }).strict(),
  z.object({ kind: z.literal('column-profile-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
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
  z.object({ kind: z.literal('time-value-unparseable'), name: z.string(), row: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('duplicate-time-value'), name: z.string(), row: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

const summaryProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() }).strict(),
  z.object({ kind: z.literal('summary-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

const previewWindowProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() }).strict(),
  z.object({ kind: z.literal('preview-failed'), detail: z.string() }).strict(),
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
  z.object({ kind: z.literal('time-series-materialization-succeeded'), request: requestSchema, matrix: z.unknown() }).strict(),
  z.object({
    kind: z.literal('materialization-failed'),
    request: requestSchema,
    problem: materializationProblemSchema,
  }).strict(),
  z.object({ kind: z.literal('protocol-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('column-profile-succeeded'), request: requestSchema, profile: z.unknown() }).strict(),
  z.object({ kind: z.literal('column-profile-failed'), request: requestSchema, problem: columnProfileProblemSchema }).strict(),
  z.object({ kind: z.literal('summary-succeeded'), request: requestSchema, summary: z.unknown() }).strict(),
  z.object({ kind: z.literal('summary-failed'), request: requestSchema, problem: summaryProblemSchema }).strict(),
  z.object({ kind: z.literal('preview-window-succeeded'), request: requestSchema, window: z.unknown() }).strict(),
  z.object({ kind: z.literal('preview-window-failed'), request: requestSchema, problem: previewWindowProblemSchema }).strict(),
  z.object({ kind: z.literal('panel-inspection-succeeded'), request: requestSchema, structure: z.unknown() }).strict(),
  z.object({ kind: z.literal('panel-materialization-succeeded'), request: requestSchema, matrix: z.unknown() }).strict(),
  z.object({ kind: z.literal('panel-data-failed'), request: requestSchema, problem: panelDataProblemSchema }).strict(),
])

export function parseDataWorkerCommand(value: unknown): Result<DataWorkerCommand, DataProtocolProblem> {
  const parsed = commandSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-command', detail: z.prettifyError(parsed.error) })
  const data = parsed.data
  const request = importRequestId(data.request)
  if (!request.ok) return err({ kind: 'invalid-command', detail: 'The import request identity is invalid.' })
  if (data.kind === 'profile-source') {
    return ok({ kind: 'profile-source', request: request.value, file: data.file })
  }

  const profile = parseDatasetProfile(data.profile)
  if (!profile.ok) return err({ kind: 'invalid-command', detail: profile.error.detail })
  if (data.kind === 'materialize-time-series') {
    const known = new Map<string, ColumnId>(profile.value.columns.map((column) => [column.id, column.id]))
    const timeColumn = known.get(data.timeColumn)
    if (timeColumn === undefined) return err({ kind: 'invalid-command', detail: 'The time column is outside the supplied profile.' })
    const columnIds: ColumnId[] = []
    for (const rawId of data.columnIds) {
      const id = known.get(rawId)
      if (id === undefined) return err({ kind: 'invalid-command', detail: `Column identity ${rawId} is not in the supplied profile.` })
      columnIds.push(id)
    }
    if (!isNonEmpty(columnIds)) return err({ kind: 'invalid-command', detail: 'At least one numeric column is required.' })
    return ok({ kind: data.kind, request: request.value, file: data.file, profile: profile.value, timeColumn, columnIds })
  }
  if (data.kind === 'inspect-panel' || data.kind === 'materialize-panel') {
    const known = new Map<string, ColumnId>(profile.value.columns.map((column) => [column.id, column.id]))
    const unitColumn = known.get(data.unitColumn)
    const timeColumn = known.get(data.timeColumn)
    if (unitColumn === undefined || timeColumn === undefined) return err({ kind: 'invalid-command', detail: 'Panel keys are outside the supplied profile.' })
    if (data.kind === 'inspect-panel') return ok({ kind: data.kind, request: request.value, file: data.file, profile: profile.value, unitColumn, timeColumn })
    const outcomeColumn = known.get(data.outcomeColumn)
    const treatmentColumn = known.get(data.treatmentColumn)
    if (outcomeColumn === undefined || treatmentColumn === undefined) return err({ kind: 'invalid-command', detail: 'Panel values are outside the supplied profile.' })
    return ok({ kind: data.kind, request: request.value, file: data.file, profile: profile.value, unitColumn, timeColumn, outcomeColumn, treatmentColumn })
  }
  if (data.kind === 'summarize-columns') {
    return ok({ kind: 'summarize-columns', request: request.value, file: data.file, profile: profile.value })
  }
  if (data.kind === 'preview-window') {
    const known = new Map<string, ColumnId>(profile.value.columns.map((column) => [column.id, column.id]))
    const bind = (column: string): ColumnId | null => known.get(column) ?? null
    const filters: PreviewFilter[] = []
    for (const filter of data.query.filters) {
      const column = bind(filter.column)
      if (column === null) return err({ kind: 'invalid-command', detail: `The preview filter names a column outside the profile: ${filter.column}.` })
      filters.push({ ...filter, column })
    }
    let sort: PreviewSort | null = null
    if (data.query.sort !== null) {
      const column = bind(data.query.sort.column)
      if (column === null) return err({ kind: 'invalid-command', detail: `The preview sort names a column outside the profile: ${data.query.sort.column}.` })
      sort = { column, direction: data.query.sort.direction }
    }
    const query: PreviewQuery = { offset: data.query.offset, limit: data.query.limit, sort, filters, search: data.query.search }
    return ok({ kind: 'preview-window', request: request.value, file: data.file, profile: profile.value, query })
  }
  if (data.kind === 'profile-column') {
    const column = profile.value.columns.find((candidate) => candidate.id === data.columnId)
    if (!column) return err({ kind: 'invalid-command', detail: `Column identity ${data.columnId} is not in the supplied profile.` })
    return ok({ kind: 'profile-column', request: request.value, file: data.file, profile: profile.value, columnId: column.id })
  }
  const byId = new Map<string, ColumnId>(profile.value.columns.map((column) => [column.id, column.id]))
  const columnIds: ColumnId[] = []
  for (const rawId of data.columnIds) {
    const id = byId.get(rawId)
    if (!id) return err({ kind: 'invalid-command', detail: `Column identity ${rawId} is not in the supplied profile.` })
    columnIds.push(id)
  }
  if (!isNonEmpty(columnIds)) return err({ kind: 'invalid-command', detail: 'At least one numeric column is required.' })
  return ok({
    kind: 'materialize-numeric',
    request: request.value,
    file: data.file,
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
  if (parsed.data.kind === 'panel-data-failed') {
    const problem = parsePanelDataProblem(parsed.data.problem)
    return problem.ok
      ? ok({ kind: parsed.data.kind, request: request.value, problem: problem.value })
      : err({ kind: 'invalid-event', detail: problem.error.detail })
  }
  if (parsed.data.kind === 'panel-inspection-succeeded') return ok({ kind: parsed.data.kind, request: request.value, structure: parsed.data.structure })
  if (parsed.data.kind === 'panel-materialization-succeeded') return ok({ kind: parsed.data.kind, request: request.value, matrix: parsed.data.matrix })
  if (parsed.data.kind === 'profile-failed' || parsed.data.kind === 'materialization-failed' || parsed.data.kind === 'column-profile-failed') {
    return ok({ ...parsed.data, request: request.value })
  }
  if (parsed.data.kind === 'summary-failed') {
    return ok({ kind: 'summary-failed', request: request.value, problem: parsed.data.problem as DatasetSummaryProblem })
  }
  if (parsed.data.kind === 'preview-window-failed') {
    return ok({ kind: 'preview-window-failed', request: request.value, problem: parsed.data.problem as PreviewWindowProblem })
  }
  if (parsed.data.kind === 'summary-succeeded' || parsed.data.kind === 'preview-window-succeeded') {
    return ok({ ...parsed.data, request: request.value })
  }
  if (parsed.data.kind === 'column-profile-succeeded') {
    const profile = parseColumnProfileShape(parsed.data.profile)
    return profile.ok
      ? ok({ kind: 'column-profile-succeeded', request: request.value, profile: profile.value })
      : err({ kind: 'invalid-event', detail: profile.error.detail })
  }
  if (parsed.data.kind === 'materialization-succeeded') {
    const matrix = parseNullableNumericMatrix(parsed.data.matrix)
    return matrix.ok
      ? ok({ kind: 'materialization-succeeded', request: request.value, matrix: matrix.value })
      : err({ kind: 'invalid-event', detail: matrix.error.detail })
  }
  if (parsed.data.kind === 'time-series-materialization-succeeded') {
    const matrix = parseTimeOrderedNumericMatrix(parsed.data.matrix)
    return matrix.ok
      ? ok({ kind: parsed.data.kind, request: request.value, matrix: matrix.value })
      : err({ kind: 'invalid-event', detail: matrix.error.detail })
  }

  const profile = parseDatasetProfile(parsed.data.profile)
  return profile.ok
    ? ok({ kind: 'profile-succeeded', request: request.value, profile: profile.value })
    : err({ kind: 'invalid-event', detail: profile.error.detail })
}
