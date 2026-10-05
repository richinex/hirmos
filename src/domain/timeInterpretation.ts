import { z } from 'zod'
import { calendarReportSchema } from './calendar'

/** Calendar readings shared by preparation and calendar-event pipelines. */
export const calendarTimeInterpretationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('timestamp') }).strict(),
  z.object({ kind: z.literal('iso-week') }).strict(),
  z
    .object({
      kind: z.literal('date-format'),
      format: z.enum(['%d/%m/%Y', '%m/%d/%Y', '%Y-%m-%d', '%Y-%m']),
    })
    .strict(),
])
/** Omitted in older recipes: numeric source types are indices; other types are ISO timestamps. */
export const timeInterpretationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('source-type') }).strict(),
  z.object({ kind: z.literal('ordinal') }).strict(),
  ...calendarTimeInterpretationSchema.options,
])

export type TimeInterpretation = z.infer<typeof timeInterpretationSchema>

/** The most common gap between consecutive distinct times; the source frequency is read from it. */
export const timeSpacingSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('ordinal') }).strict(),
  z.object({ kind: z.literal('single-period') }).strict(),
  z.object({ kind: z.literal('days'), modal: z.number().positive() }).strict(),
])
export type TimeSpacing = z.infer<typeof timeSpacingSchema>

export const timePreviewSchema = z
  .object({
    calendar: calendarReportSchema.optional(),
    kind: z.enum(['ordinal', 'calendar']),
    rows: z
      .array(
        z
          .object({ original: z.string().nullable(), parsed: z.number().finite().nullable() })
          .strict(),
      )
      .max(12),
    spacing: timeSpacingSchema,
  })
  .strict()
export type TimePreview = z.infer<typeof timePreviewSchema>

/** The readings that yield a calendar date; a numeric index has none. */
export type CalendarTimeInterpretation = z.infer<typeof calendarTimeInterpretationSchema>

/**
 * DuckDB owns calendar arithmetic. The SQL for one column read as a timestamp: a cast, a format,
 * or an ISO week whose round trip rejects overflowing week numbers. Invalid text becomes NULL.
 */
export function timestampSql(
  column: string,
  interpretation: CalendarTimeInterpretation,
  sqlString: (value: string) => string,
): string {
  switch (interpretation.kind) {
    case 'timestamp':
      return `TRY_CAST(${column} AS TIMESTAMPTZ)`
    case 'date-format':
      return `try_strptime(CAST(${column} AS VARCHAR), ${sqlString(interpretation.format)})`
    case 'iso-week': {
      const text = `CAST(${column} AS VARCHAR)`
      const parsed = `try_strptime(${text} || '-1', '%G-W%V-%u')`
      return `CASE WHEN strftime(${parsed}, '%G-W%V') = ${text} THEN ${parsed} ELSE NULL END`
    }
  }
}

export const TIME_INTERPRETATIONS: readonly {
  readonly label: string
  readonly value: string
  readonly interpretation: TimeInterpretation
}[] = [
  { label: 'Source type', value: 'source-type', interpretation: { kind: 'source-type' } },
  { label: 'ISO date or timestamp', value: 'timestamp', interpretation: { kind: 'timestamp' } },
  { label: 'ISO week (2024-W02)', value: 'iso-week', interpretation: { kind: 'iso-week' } },
  {
    label: 'Day/month/year (31/12/2024)',
    value: '%d/%m/%Y',
    interpretation: { kind: 'date-format', format: '%d/%m/%Y' },
  },
  {
    label: 'Month/day/year (12/31/2024)',
    value: '%m/%d/%Y',
    interpretation: { kind: 'date-format', format: '%m/%d/%Y' },
  },
  {
    label: 'Year-month-day (2024-12-31)',
    value: '%Y-%m-%d',
    interpretation: { kind: 'date-format', format: '%Y-%m-%d' },
  },
  {
    label: 'Year-month (2024-12), first day of month',
    value: '%Y-%m',
    interpretation: { kind: 'date-format', format: '%Y-%m' },
  },
  { label: 'Numeric time index', value: 'ordinal', interpretation: { kind: 'ordinal' } },
]

/** The same list where only a calendar reading makes sense. */
export const CALENDAR_TIME_INTERPRETATIONS: readonly {
  readonly label: string
  readonly value: string
  readonly interpretation: CalendarTimeInterpretation
}[] = TIME_INTERPRETATIONS.flatMap((entry) =>
  entry.interpretation.kind === 'source-type' || entry.interpretation.kind === 'ordinal'
    ? []
    : [{ ...entry, interpretation: entry.interpretation }],
)

/** What an ISO week is, as the time-interpretation help states it. */
export const ISO_WEEK_NOTE =
  'Weeks start on Monday. The ISO week-year can differ from the calendar year.'
