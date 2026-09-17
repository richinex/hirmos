import { z } from 'zod'
import { calendarReportSchema } from './calendar'

/** Omitted in older recipes: numeric source types are indices; other types are ISO timestamps. */
export const timeInterpretationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('source-type') }).strict(),
  z.object({ kind: z.literal('timestamp') }).strict(),
  z.object({ kind: z.literal('ordinal') }).strict(),
  z.object({ kind: z.literal('iso-week') }).strict(),
  z.object({ kind: z.literal('date-format'), format: z.enum(['%d/%m/%Y', '%m/%d/%Y', '%Y-%m-%d']) }).strict(),
])

export type TimeInterpretation = z.infer<typeof timeInterpretationSchema>

export const timePreviewSchema = z.object({
  calendar: calendarReportSchema.optional(),
  kind: z.enum(['ordinal', 'calendar']),
  rows: z.array(z.object({ original: z.string().nullable(), parsed: z.number().finite().nullable() }).strict()).max(12),
}).strict()
export type TimePreview = z.infer<typeof timePreviewSchema>

export const TIME_INTERPRETATIONS: readonly { readonly label: string; readonly value: string; readonly interpretation: TimeInterpretation }[] = [
  { label: 'Source type', value: 'source-type', interpretation: { kind: 'source-type' } },
  { label: 'ISO date or timestamp', value: 'timestamp', interpretation: { kind: 'timestamp' } },
  { label: 'ISO week (2024-W02)', value: 'iso-week', interpretation: { kind: 'iso-week' } },
  { label: 'Day/month/year (31/12/2024)', value: '%d/%m/%Y', interpretation: { kind: 'date-format', format: '%d/%m/%Y' } },
  { label: 'Month/day/year (12/31/2024)', value: '%m/%d/%Y', interpretation: { kind: 'date-format', format: '%m/%d/%Y' } },
  { label: 'Year-month-day (2024-12-31)', value: '%Y-%m-%d', interpretation: { kind: 'date-format', format: '%Y-%m-%d' } },
  { label: 'Numeric time index', value: 'ordinal', interpretation: { kind: 'ordinal' } },
]
