import { calendarFacts, seasonalFact, trimmingFact, type WindowContext } from '@/domain/windowEvidence'

export function WindowEvidence({ context, seasonal }: {
  readonly context?:WindowContext
  readonly seasonal?:{readonly rows:number;readonly period:number;readonly before:number}
}) {
  const trimming=context?.completeInterval === null || context?.completeInterval === undefined ? null : trimmingFact(context.completeInterval)
  const facts=[...(trimming === null ? [] : [trimming]), ...(context === undefined ? [] : calendarFacts(context.calendar))]
  if (facts.length === 0 && seasonal === undefined) return null
  return <section aria-label="Analysis window" className="space-y-2 text-body text-muted">
    <h4 className="m-0 text-body font-medium text-ink">Analysis window</h4>
    {facts.map(fact => <p key={fact} className="m-0">{fact}</p>)}
    {seasonal !== undefined && <>
      <p className="m-0">{seasonalFact(seasonal.rows,seasonal.period)}</p>
      <p className="m-0 text-label text-faint">Before the event: {seasonal.before} observations. From the event onwards: {seasonal.rows-seasonal.before} observations.</p>
    </>}
  </section>
}
