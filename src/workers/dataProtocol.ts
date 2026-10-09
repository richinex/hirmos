import { z } from 'zod'
import { categoricalMembershipSchema, type CategoricalMembership } from '@/domain/sampleMembership'
import { columnDeclarationsSchema, type ColumnDeclarations } from '@/domain/fileReading'
import {
  timeInterpretationSchema,
  timePreviewSchema,
  type TimeInterpretation,
  type TimePreview,
} from '@/domain/timeInterpretation'
import { calendarRequestSchema, type CalendarRequest } from '@/domain/calendar'
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
import { assertNever, err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { importRequestId, type ImportRequestId } from '@/domain/workflow'
import {
  panelDataProblemSchema,
  parsePanelDataProblem,
  type PanelDataProblem,
} from '@/domain/panel'

export type DataWorkerCommand =
  | { readonly kind: 'warm-engine' }
  | {
      readonly kind: 'preview-time'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly timeColumn: ColumnId
      readonly interpretation: TimeInterpretation
      readonly calendar?: CalendarRequest
    }
  | {
      readonly kind: 'profile-source'
      readonly request: ImportRequestId
      readonly file: File
      readonly declared: ColumnDeclarations
    }
  | {
      readonly kind: 'materialize-numeric'
      readonly membership?: CategoricalMembership
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly columnIds: readonly [ColumnId, ...ColumnId[]]
    }
  | {
      readonly kind: 'materialize-time-series'
      readonly interpretation?: TimeInterpretation
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
      readonly categoryColumn?: ColumnId
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
  | {
      readonly kind: 'inspect-panel'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly unitColumn: ColumnId
      readonly timeColumn: ColumnId
    }
  | {
      readonly kind: 'materialize-panel-keys'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly unitColumn: ColumnId
      readonly timeColumn: ColumnId
    }
  | {
      readonly kind: 'materialize-panel'
      readonly request: ImportRequestId
      readonly file: File
      readonly profile: DatasetProfile
      readonly unitColumn: ColumnId
      readonly timeColumn: ColumnId
      readonly outcomeColumn: ColumnId
      readonly treatmentColumn: ColumnId
      readonly covariates?: readonly ColumnId[]
      readonly clusterColumn?: ColumnId
    }

export type DataWorkerEvent =
  | {
      readonly kind: 'time-preview-succeeded'
      readonly request: ImportRequestId
      readonly preview: TimePreview
    }
  | {
      readonly kind: 'profile-succeeded'
      readonly request: ImportRequestId
      readonly profile: DatasetProfile
    }
  | {
      readonly kind: 'profile-failed'
      readonly request: ImportRequestId
      readonly problem: DatasetProfileProblem
    }
  | {
      readonly kind: 'materialization-succeeded'
      readonly request: ImportRequestId
      readonly matrix: NullableNumericMatrix
    }
  | {
      readonly kind: 'time-series-materialization-succeeded'
      readonly request: ImportRequestId
      readonly matrix: TimeOrderedNumericMatrix
    }
  | {
      readonly kind: 'materialization-failed'
      readonly request: ImportRequestId
      readonly problem: NumericMaterializationProblem | TimeSeriesMaterializationProblem
    }
  | {
      readonly kind: 'column-profile-succeeded'
      readonly request: ImportRequestId
      readonly profile: ColumnProfile
    }
  | {
      readonly kind: 'column-profile-failed'
      readonly request: ImportRequestId
      readonly problem: ColumnProfileProblem
    }
  | {
      readonly kind: 'summary-succeeded'
      readonly request: ImportRequestId
      readonly summary: unknown
    }
  | {
      readonly kind: 'summary-failed'
      readonly request: ImportRequestId
      readonly problem: DatasetSummaryProblem
    }
  | {
      readonly kind: 'preview-window-succeeded'
      readonly request: ImportRequestId
      readonly window: unknown
    }
  | {
      readonly kind: 'preview-window-failed'
      readonly request: ImportRequestId
      readonly problem: PreviewWindowProblem
    }
  | {
      readonly kind: 'panel-inspection-succeeded'
      readonly request: ImportRequestId
      readonly structure: unknown
    }
  | {
      readonly kind: 'panel-keys-succeeded'
      readonly request: ImportRequestId
      readonly matrix: unknown
    }
  | {
      readonly kind: 'panel-materialization-succeeded'
      readonly request: ImportRequestId
      readonly matrix: unknown
    }
  | {
      readonly kind: 'panel-data-failed'
      readonly request: ImportRequestId
      readonly problem: PanelDataProblem
    }
  | { readonly kind: 'protocol-failed'; readonly detail: string }

export type DataProtocolProblem =
  | { readonly kind: 'invalid-command'; readonly detail: string }
  | { readonly kind: 'invalid-event'; readonly detail: string }

const requestSchema = z.string().uuid()

const commandSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('warm-engine') }).strict(),
  z
    .object({
      kind: z.literal('preview-time'),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      timeColumn: z.string(),
      interpretation: timeInterpretationSchema,
      calendar: calendarRequestSchema.optional(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('profile-source'),
      request: requestSchema,
      file: z.instanceof(File),
      declared: columnDeclarationsSchema,
    })
    .strict(),
  z
    .object({
      kind: z.literal('materialize-numeric'),
      membership: categoricalMembershipSchema.optional(),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      columnIds: z.array(z.string()).min(1),
    })
    .strict(),
  z
    .object({
      kind: z.literal('materialize-time-series'),
      interpretation: timeInterpretationSchema.default({ kind: 'source-type' }),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      timeColumn: z.string(),
      columnIds: z.array(z.string()).min(1),
    })
    .strict(),
  z
    .object({
      kind: z.literal('profile-column'),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      columnId: z.string(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('summarize-columns'),
      categoryColumn: z.string().optional(),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('preview-window'),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      query: previewQuerySchema,
    })
    .strict(),
  z
    .object({
      kind: z.literal('inspect-panel'),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      unitColumn: z.string(),
      timeColumn: z.string(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('materialize-panel-keys'),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      unitColumn: z.string(),
      timeColumn: z.string(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('materialize-panel'),
      request: requestSchema,
      file: z.instanceof(File),
      profile: z.unknown(),
      unitColumn: z.string(),
      timeColumn: z.string(),
      outcomeColumn: z.string(),
      treatmentColumn: z.string(),
      covariates: z.array(z.string()).default([]),
      clusterColumn: z.string().min(1).optional(),
    })
    .strict(),
])

const columnProfileProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('column-not-found'), id: z.string() }).strict(),
  z
    .object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() })
    .strict(),
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
  z
    .object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() })
    .strict(),
  z.object({ kind: z.literal('column-not-found'), id: z.string() }).strict(),
  z.object({ kind: z.literal('duplicate-column'), id: z.string() }).strict(),
  z
    .object({ kind: z.literal('non-numeric-column'), name: z.string(), duckdbType: z.string() })
    .strict(),
  z
    .object({
      kind: z.literal('non-finite-value'),
      name: z.string(),
      row: z.number().int().nonnegative(),
    })
    .strict(),
  z.object({ kind: z.literal('materialization-failed'), detail: z.string() }).strict(),
  z
    .object({
      kind: z.literal('time-value-unparseable'),
      name: z.string(),
      row: z.number().int().nonnegative(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('duplicate-time-value'),
      name: z.string(),
      row: z.number().int().nonnegative(),
    })
    .strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

const summaryProblemSchema = z.discriminatedUnion('kind', [
  z
    .object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() })
    .strict(),
  z.object({ kind: z.literal('summary-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

const previewWindowProblemSchema = z.discriminatedUnion('kind', [
  z
    .object({ kind: z.literal('source-changed'), expected: z.string(), actual: z.string() })
    .strict(),
  z.object({ kind: z.literal('preview-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

const eventSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('time-preview-succeeded'),
      request: requestSchema,
      preview: timePreviewSchema,
    })
    .strict(),
  z
    .object({
      kind: z.literal('profile-succeeded'),
      request: requestSchema,
      profile: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('profile-failed'),
      request: requestSchema,
      problem: profileProblemSchema,
    })
    .strict(),
  z
    .object({
      kind: z.literal('materialization-succeeded'),
      request: requestSchema,
      matrix: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('time-series-materialization-succeeded'),
      request: requestSchema,
      matrix: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('materialization-failed'),
      request: requestSchema,
      problem: materializationProblemSchema,
    })
    .strict(),
  z.object({ kind: z.literal('protocol-failed'), detail: z.string() }).strict(),
  z
    .object({
      kind: z.literal('column-profile-succeeded'),
      request: requestSchema,
      profile: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('column-profile-failed'),
      request: requestSchema,
      problem: columnProfileProblemSchema,
    })
    .strict(),
  z
    .object({ kind: z.literal('summary-succeeded'), request: requestSchema, summary: z.unknown() })
    .strict(),
  z
    .object({
      kind: z.literal('summary-failed'),
      request: requestSchema,
      problem: summaryProblemSchema,
    })
    .strict(),
  z
    .object({
      kind: z.literal('preview-window-succeeded'),
      request: requestSchema,
      window: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('preview-window-failed'),
      request: requestSchema,
      problem: previewWindowProblemSchema,
    })
    .strict(),
  z
    .object({
      kind: z.literal('panel-inspection-succeeded'),
      request: requestSchema,
      structure: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('panel-keys-succeeded'),
      request: requestSchema,
      matrix: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('panel-materialization-succeeded'),
      request: requestSchema,
      matrix: z.unknown(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('panel-data-failed'),
      request: requestSchema,
      problem: panelDataProblemSchema,
    })
    .strict(),
])

function bindColumns(
  known: ReadonlyMap<string, ColumnId>,
  requested: readonly string[],
): Result<NonEmptyArray<ColumnId>, DataProtocolProblem> {
  const columns: ColumnId[] = []
  for (const raw of requested) {
    const column = known.get(raw)
    if (column === undefined)
      return err({
        kind: 'invalid-command',
        detail: `Column identity ${raw} is not in the supplied profile.`,
      })
    columns.push(column)
  }
  return isNonEmpty(columns)
    ? ok(columns)
    : err({ kind: 'invalid-command', detail: 'At least one numeric column is required.' })
}

export function parseDataWorkerCommand(
  value: unknown,
): Result<DataWorkerCommand, DataProtocolProblem> {
  const parsed = commandSchema.safeParse(value)
  if (!parsed.success)
    return err({ kind: 'invalid-command', detail: z.prettifyError(parsed.error) })
  const data = parsed.data
  if (data.kind === 'warm-engine') return ok({ kind: 'warm-engine' })
  const request = importRequestId(data.request)
  if (!request.ok)
    return err({ kind: 'invalid-command', detail: 'The import request identity is invalid.' })
  if (data.kind === 'profile-source') {
    return ok({
      kind: 'profile-source',
      request: request.value,
      file: data.file,
      declared: data.declared,
    })
  }

  const profile = parseDatasetProfile(data.profile)
  if (!profile.ok) return err({ kind: 'invalid-command', detail: profile.error.detail })
  const known = new Map<string, ColumnId>(
    profile.value.columns.map((column) => [column.id, column.id]),
  )
  switch (data.kind) {
    case 'preview-time': {
      if (data.calendar?.unitColumn !== undefined && !known.has(data.calendar.unitColumn))
        return err({
          kind: 'invalid-command',
          detail: 'The unit column is outside the supplied profile.',
        })
      const timeColumn = known.get(data.timeColumn)
      return timeColumn === undefined
        ? err({
            kind: 'invalid-command',
            detail: 'The time column is outside the supplied profile.',
          })
        : ok({ ...data, request: request.value, profile: profile.value, timeColumn })
    }
    case 'materialize-time-series': {
      const timeColumn = known.get(data.timeColumn)
      if (timeColumn === undefined)
        return err({
          kind: 'invalid-command',
          detail: 'The time column is outside the supplied profile.',
        })
      const columns = bindColumns(known, data.columnIds)
      if (!columns.ok) return columns
      return ok({
        kind: data.kind,
        request: request.value,
        file: data.file,
        profile: profile.value,
        timeColumn,
        columnIds: columns.value,
        interpretation: data.interpretation,
      })
    }
    case 'inspect-panel':
    case 'materialize-panel-keys':
    case 'materialize-panel': {
      const unitColumn = known.get(data.unitColumn)
      const timeColumn = known.get(data.timeColumn)
      if (unitColumn === undefined || timeColumn === undefined)
        return err({
          kind: 'invalid-command',
          detail: 'Panel keys are outside the supplied profile.',
        })
      switch (data.kind) {
        case 'inspect-panel':
        case 'materialize-panel-keys':
          return ok({
            kind: data.kind,
            request: request.value,
            file: data.file,
            profile: profile.value,
            unitColumn,
            timeColumn,
          })
        case 'materialize-panel': {
          const outcomeColumn = known.get(data.outcomeColumn)
          const treatmentColumn = known.get(data.treatmentColumn)
          if (outcomeColumn === undefined || treatmentColumn === undefined)
            return err({
              kind: 'invalid-command',
              detail: 'Panel values are outside the supplied profile.',
            })
          const covariates: ColumnId[] = []
          const selected = new Set([unitColumn, timeColumn, outcomeColumn, treatmentColumn])
          for (const raw of data.covariates) {
            const column = known.get(raw)
            if (column === undefined || selected.has(column))
              return err({
                kind: 'invalid-command',
                detail: 'Panel covariates must be distinct from the keys, outcome and treatment.',
              })
            covariates.push(column)
            selected.add(column)
          }
          const clusterColumn =
            data.clusterColumn === undefined ? undefined : known.get(data.clusterColumn)
          if (data.clusterColumn !== undefined && clusterColumn === undefined)
            return err({
              kind: 'invalid-command',
              detail: 'The cluster column is outside the supplied profile.',
            })
          return ok({
            kind: data.kind,
            request: request.value,
            file: data.file,
            profile: profile.value,
            unitColumn,
            timeColumn,
            outcomeColumn,
            treatmentColumn,
            covariates,
            clusterColumn,
          })
        }
        default:
          return assertNever(data)
      }
    }
    case 'summarize-columns': {
      const categoryColumn =
        data.categoryColumn === undefined ? undefined : known.get(data.categoryColumn)
      if (data.categoryColumn !== undefined && categoryColumn === undefined)
        return err({
          kind: 'invalid-command',
          detail: 'The category column is outside the supplied profile.',
        })
      return ok({
        kind: 'summarize-columns',
        request: request.value,
        file: data.file,
        profile: profile.value,
        categoryColumn,
      })
    }
    case 'preview-window': {
      const bind = (column: string): ColumnId | null => known.get(column) ?? null
      const filters: PreviewFilter[] = []
      for (const filter of data.query.filters) {
        const column = bind(filter.column)
        if (column === null)
          return err({
            kind: 'invalid-command',
            detail: `The preview filter names a column outside the profile: ${filter.column}.`,
          })
        filters.push({ ...filter, column })
      }
      let sort: PreviewSort | null = null
      if (data.query.sort !== null) {
        const column = bind(data.query.sort.column)
        if (column === null)
          return err({
            kind: 'invalid-command',
            detail: `The preview sort names a column outside the profile: ${data.query.sort.column}.`,
          })
        sort = { column, direction: data.query.sort.direction }
      }
      const query: PreviewQuery = {
        offset: data.query.offset,
        limit: data.query.limit,
        sort,
        filters,
        search: data.query.search,
      }
      return ok({
        kind: 'preview-window',
        request: request.value,
        file: data.file,
        profile: profile.value,
        query,
      })
    }
    case 'profile-column': {
      const column = profile.value.columns.find((candidate) => candidate.id === data.columnId)
      if (!column)
        return err({
          kind: 'invalid-command',
          detail: `Column identity ${data.columnId} is not in the supplied profile.`,
        })
      return ok({
        kind: 'profile-column',
        request: request.value,
        file: data.file,
        profile: profile.value,
        columnId: column.id,
      })
    }
    case 'materialize-numeric': {
      if (data.membership !== undefined && !data.columnIds.includes(data.membership.column))
        return err({
          kind: 'invalid-command',
          detail: 'The membership column must be included in the materialized columns.',
        })
      const columns = bindColumns(known, data.columnIds)
      if (!columns.ok) return columns
      return ok({
        kind: 'materialize-numeric',
        membership: data.membership,
        request: request.value,
        file: data.file,
        profile: profile.value,
        columnIds: columns.value,
      })
    }
    default:
      return assertNever(data)
  }
}

export function parseDataWorkerEvent(value: unknown): Result<DataWorkerEvent, DataProtocolProblem> {
  const parsed = eventSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-event', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind === 'protocol-failed') return ok(parsed.data)

  const request = importRequestId(parsed.data.request)
  if (!request.ok)
    return err({ kind: 'invalid-event', detail: 'The import request identity is invalid.' })
  switch (parsed.data.kind) {
    case 'time-preview-succeeded':
    case 'panel-inspection-succeeded':
    case 'panel-materialization-succeeded':
    case 'panel-keys-succeeded':
    case 'profile-failed':
    case 'materialization-failed':
    case 'column-profile-failed':
    case 'summary-succeeded':
    case 'preview-window-succeeded':
      return ok({ ...parsed.data, request: request.value })
    case 'summary-failed':
      return ok({
        ...parsed.data,
        request: request.value,
        problem: parsed.data.problem as DatasetSummaryProblem,
      })
    case 'preview-window-failed':
      return ok({
        ...parsed.data,
        request: request.value,
        problem: parsed.data.problem as PreviewWindowProblem,
      })
    case 'panel-data-failed': {
      const problem = parsePanelDataProblem(parsed.data.problem)
      return problem.ok
        ? ok({ kind: parsed.data.kind, request: request.value, problem: problem.value })
        : err({ kind: 'invalid-event', detail: problem.error.detail })
    }
    case 'column-profile-succeeded': {
      const profile = parseColumnProfileShape(parsed.data.profile)
      return profile.ok
        ? ok({ kind: 'column-profile-succeeded', request: request.value, profile: profile.value })
        : err({ kind: 'invalid-event', detail: profile.error.detail })
    }
    case 'materialization-succeeded': {
      const matrix = parseNullableNumericMatrix(parsed.data.matrix)
      return matrix.ok
        ? ok({ kind: 'materialization-succeeded', request: request.value, matrix: matrix.value })
        : err({ kind: 'invalid-event', detail: matrix.error.detail })
    }
    case 'time-series-materialization-succeeded': {
      const matrix = parseTimeOrderedNumericMatrix(parsed.data.matrix)
      return matrix.ok
        ? ok({ kind: parsed.data.kind, request: request.value, matrix: matrix.value })
        : err({ kind: 'invalid-event', detail: matrix.error.detail })
    }

    case 'profile-succeeded': {
      const profile = parseDatasetProfile(parsed.data.profile)
      return profile.ok
        ? ok({ kind: 'profile-succeeded', request: request.value, profile: profile.value })
        : err({ kind: 'invalid-event', detail: profile.error.detail })
    }
    default:
      return assertNever(parsed.data)
  }
}
