import { z } from 'zod'

export const calendarRequestSchema = z
  .object({
    schedule: z.enum([
      'daily',
      'monday',
      'tuesday',
      'wednesday',
      'thursday',
      'friday',
      'saturday',
      'sunday',
      'month-start',
      'month-end',
      'quarter-start',
      'quarter-end',
      'year-start',
      'year-end',
    ]),
    unitColumn: z.string().optional(),
  })
  .strict()
export type CalendarRequest = z.infer<typeof calendarRequestSchema>
export const calendarReportSchema = z
  .object({
    units: z.number().int().nonnegative(),
    missing: z.number().int().nonnegative(),
    ranges: z.number().int().nonnegative(),
    gaps: z
      .array(
        z
          .object({
            unit: z.string(),
            first: z.string(),
            last: z.string(),
            count: z.number().int().positive(),
          })
          .strict(),
      )
      .max(200),
    issues: z.array(z.object({ unit: z.string(), detail: z.string() }).strict()),
  })
  .strict()
export type CalendarReport = z.infer<typeof calendarReportSchema>
