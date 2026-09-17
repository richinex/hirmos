import { calendarReportSchema, type CalendarRequest } from '@/domain/calendar'
import initWasm, { inspectCalendar as inspect } from '@/generated/analysis-wasm/hirmos_analysis'

let runtime: ReturnType<typeof initWasm> | undefined

export async function inspectCalendar(schedule: CalendarRequest['schedule'], units: readonly { name: string; times: number[] }[]) {
  runtime ??= initWasm().catch((error: unknown) => { runtime = undefined; throw error })
  await runtime
  return calendarReportSchema.parse(JSON.parse(inspect(JSON.stringify({ schedule, units }))))
}
