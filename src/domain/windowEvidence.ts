import { z } from 'zod'
import { assertNever } from './dop'
import { calendarWindowDays, describeCalendarWindow, type CalendarRowSpan, type CalendarWindow, type PipelineNode } from './pipeline'
import type { SourceRecipe } from './sqlPreparation'
import type { MissingnessResolutionRecord } from './missingness'
import type { CalendarTimeInterpretation, TimeInterpretation } from './timeInterpretation'

const day = z.object({ month: z.number().int().min(1).max(12), day: z.number().int().min(1).max(31) }).strict()
  .refine(d => d.day <= [31,29,31,30,31,30,31,31,30,31,30,31][d.month - 1]!)
const windowSchema = z.discriminatedUnion('kind', [z.object({ kind:z.literal('year-end') }).strict(), z.object({ kind:z.literal('custom'), from:day, to:day }).strict()])
export const calendarEdgeSchema = z.discriminatedUnion('kind', [
  z.object({ kind:z.literal('unavailable'), reason:z.enum(['no-declaration', 'untraced-date', 'non-calendar', 'legacy']) }).strict(),
  z.object({ kind:z.literal('recorded'), lastCoveredDay:z.number().int().finite(), windows:z.array(z.object({ name:z.string().min(1), window:windowSchema, endsInside:z.boolean() }).strict()).nonempty() }).strict(),
]).superRefine((value, ctx) => {
  if (value.kind !== 'recorded') return
  if (!Number.isFinite(new Date(value.lastCoveredDay).getTime()) || value.lastCoveredDay % DAY !== 0) ctx.addIssue({code:'custom',message:'The final covered day must be a UTC calendar day.'})
  for (const item of value.windows) if (item.endsInside !== endsInsideWindow(value.lastCoveredDay, item.window)) ctx.addIssue({code:'custom',message:'Calendar edge evidence does not match the declared window.'})
})
export type CalendarEdge = z.infer<typeof calendarEdgeSchema>
export const windowContextSchema = z.object({
  completeInterval:z.object({kind:z.literal('window'),start:z.number().int().nonnegative(),endExclusive:z.number().int().positive(),sourceRows:z.number().int().positive()}).strict()
    .refine(r => r.start < r.endExclusive && r.endExclusive <= r.sourceRows).nullable(),
  calendar:calendarEdgeSchema,
}).strict()
export type WindowContext = z.infer<typeof windowContextSchema>
export function windowContext(prepared:{readonly resolution:MissingnessResolutionRecord;readonly calendarCoverage?:CalendarEdge}):WindowContext {
  return {completeInterval:prepared.resolution.kind === 'window' ? prepared.resolution : null, calendar:prepared.calendarCoverage ?? {kind:'unavailable',reason:'legacy'}}
}
type Declaration = { readonly name:string; readonly window:CalendarWindow; readonly span:CalendarRowSpan; readonly interpretation:CalendarTimeInterpretation }
type Lineage = { readonly kind:'declared'; readonly declarations:readonly Declaration[] } | { readonly kind:'unavailable'; readonly reason:'no-declaration'|'untraced-date' }

/** Trace only operations that preserve the meaning of the selected date column. */
export function calendarDeclarations(recipe: SourceRecipe, timeColumn:string): Lineage {
  if (recipe.kind !== 'pipeline-derived') return { kind:'unavailable', reason:'no-declaration' }
  const outputs = recipe.graph.nodes.filter(n => n.block.kind === 'output')
  if (outputs.length !== 1) return { kind:'unavailable', reason:'untraced-date' }
  let node:PipelineNode|undefined = outputs[0]
  let column = timeColumn
  const seen = new Set<string>()
  const declarations:Declaration[] = []
  const finish = (reason:'no-declaration'|'untraced-date'):Lineage => declarations.length > 0 ? {kind:'declared',declarations} : {kind:'unavailable',reason}
  while (node !== undefined) {
    if (seen.has(node.id)) return finish('untraced-date')
    seen.add(node.id)
    const block = node.block
    switch (block.kind) {
      case 'input': return finish('no-declaration')
      case 'script': case 'join': case 'union': return finish('untraced-date')
      case 'calendar-events':
        if (block.name === column) return finish('untraced-date')
        if (block.column === column) declarations.push({name:block.name,window:block.window,span:block.span,interpretation:block.interpretation})
        break
      case 'select-columns': {
        const rename = block.renames.find(r => r.to === column)
        const original = rename?.from ?? column
        if ((block.mode === 'keep' && !block.columns.includes(original)) || (block.mode === 'drop' && block.columns.includes(original))) return finish('untraced-date')
        column = original
        break
      }
      case 'derive-columns': if (block.columns.some(c => c.name === column)) return finish('untraced-date'); break
      case 'aggregate': if (!block.groupBy.includes(column)) return finish('untraced-date'); break
      case 'filter-rows': case 'sort-limit': case 'output': break
      default: return assertNever(block)
    }
    const incoming = recipe.graph.edges.filter(e => e.to === node!.id)
    if (incoming.length !== 1) return finish('untraced-date')
    node = recipe.graph.nodes.find(n => n.id === incoming[0]!.from)
  }
  return finish('untraced-date')
}
const DAY = 86_400_000
export function utcDay(timestamp:number):number { const d = new Date(timestamp); return Date.UTC(d.getUTCFullYear(),d.getUTCMonth(),d.getUTCDate()) }
/** Same inclusive month/day predicate as the Calendar events SQL. */
function inside(timestamp:number, window:CalendarWindow):boolean {
  const d = new Date(timestamp), key = (d.getUTCMonth()+1)*100+d.getUTCDate()
  const {from,to} = calendarWindowDays(window), lo=from.month*100+from.day, hi=to.month*100+to.day
  return lo <= hi ? key >= lo && key <= hi : key >= lo || key <= hi
}
function endsInsideWindow(lastDay:number, window:CalendarWindow):boolean {
  const d=new Date(lastDay), {to}=calendarWindowDays(window)
  return inside(lastDay,window) && inside(lastDay+DAY,window) && !(d.getUTCMonth()+1 === to.month && d.getUTCDate() === to.day)
}
/** End-exclusive coverage of a row, matching DuckDB's calendar month addition. */
export function rowCoverageEnd(timestamp:number, span:CalendarRowSpan):number {
  const start=utcDay(timestamp)
  if (span === 'day') return start+DAY
  if (span === 'week') return start+7*DAY
  const d=new Date(start), y=d.getUTCFullYear(), m=d.getUTCMonth()
  const last=new Date(Date.UTC(y,m+2,0)).getUTCDate()
  return Date.UTC(y,m+1,Math.min(d.getUTCDate(),last))
}
export function calendarEdgeFor(recipe:SourceRecipe, timeColumn:string, lastTimestamp:number|null, span:CalendarRowSpan|null, coverageCap:number|null = null, interpretation:TimeInterpretation = {kind:'source-type'}):CalendarEdge {
  if (lastTimestamp === null || span === null) return {kind:'unavailable',reason:'non-calendar'}
  const lineage=calendarDeclarations(recipe,timeColumn)
  if (lineage.kind === 'unavailable') return lineage
  // A different span describes different covered days, not merely a different label.
  const parsingMatches = (declared:CalendarTimeInterpretation):boolean => {
    switch (interpretation.kind) {
      case 'source-type': case 'timestamp': return declared.kind === 'timestamp'
      case 'ordinal': return false
      case 'iso-week': return declared.kind === 'iso-week'
      case 'date-format': return declared.kind === 'date-format' && declared.format === interpretation.format
      default: return assertNever(interpretation)
    }
  }
  if (lineage.declarations.some(d => d.span !== span || !parsingMatches(d.interpretation))) return {kind:'unavailable',reason:'untraced-date'}
  const end=Math.min(rowCoverageEnd(lastTimestamp,span),coverageCap ?? Infinity)
  const lastCoveredDay=utcDay(end-1)
  return {kind:'recorded',lastCoveredDay,windows:lineage.declarations.map(d => ({name:d.name,window:d.window,endsInside:endsInsideWindow(lastCoveredDay,d.window)}))}
}
export function trimmingFact(record:MissingnessResolutionRecord):string|null {
  return record.kind === 'window' ? `The complete-interval rule kept ${record.endExclusive-record.start} of ${record.sourceRows} source rows. It removed ${record.start} before and ${record.sourceRows-record.endExclusive} after this interval.` : null
}
export function seasonalFact(rows:number, period:number):string {
  const cycles=((rows-1)/period).toFixed(2)
  return `${rows} observations at a seasonal period of ${period} rows. The first-to-last observation spans ${cycles} cycles.`
}
export function calendarFacts(evidence:CalendarEdge):readonly string[] {
  if (evidence.kind === 'unavailable') {
    switch (evidence.reason) {
      case 'no-declaration': return []
      case 'legacy': return ['Calendar coverage was not recorded for this prepared version.']
      case 'non-calendar': return []
      case 'untraced-date': return ['Calendar coverage is unavailable because the date lineage or row span could not be confirmed.']
      default: return assertNever(evidence.reason)
    }
  }
  const date=new Intl.DateTimeFormat('en-GB',{day:'numeric',month:'long',year:'numeric',timeZone:'UTC'}).format(evidence.lastCoveredDay)
  return evidence.windows.map(w => `Coverage ends on ${date}. ${w.endsInside ? 'This is before the end of' : 'It does not end partway through'} the declared ${describeCalendarWindow(w.window)} window (${w.name}).`)
}
