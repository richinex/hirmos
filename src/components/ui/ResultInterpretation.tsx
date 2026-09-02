import type { ResultInterpretation as ResultInterpretationModel } from '@/domain/resultInterpretation'
import { assertNever } from '@/domain/dop'
import { well } from './recipes'

const statementClass = (kind: ResultInterpretationModel['statements'][number]['kind']): string => {
  switch (kind) {
    case 'magnitude': return 'text-ink'
    case 'uncertainty':
    case 'comparison': return 'text-muted'
    case 'qualification': return 'text-faint'
    default: return assertNever(kind)
  }
}

/** The shared prose companion to a numerical result. Every sentence comes from recorded facts. */
export function ResultInterpretation({ interpretation, className = '' }: {
  readonly interpretation: ResultInterpretationModel
  readonly className?: string
}) {
  return (
    <section className={well(`px-3 py-3 ${className}`.trim())} aria-label="Interpretation">
      <h4 className="m-0 text-faint text-label font-medium">What this result means</h4>
      <div className="mt-2 space-y-1.5">
        {interpretation.statements.map((statement, index) => (
          <p key={`${statement.kind}-${index}`} className={`m-0 max-w-[75ch] text-body ${statementClass(statement.kind)}`}>{statement.text}</p>
        ))}
      </div>
    </section>
  )
}
