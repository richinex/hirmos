import type { ResultInterpretation as ResultInterpretationModel } from '@/domain/resultInterpretation'
import { assertNever, isNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { prose } from '@/components/ui/recipes'

type StatementKind = ResultInterpretationModel['statements'][number]['kind']
type Statement = ResultInterpretationModel['statements'][number]

interface InterpretationSection {
  readonly kind: StatementKind
  readonly statements: NonEmptyArray<Statement>
}

type InterpretationContext = 'result' | 'sensitivity-check' | 'discovery-run'

const SECTION_ORDER: readonly StatementKind[] = ['magnitude', 'comparison', 'uncertainty', 'qualification']

const sectionLabel = (kind: StatementKind): string => {
  switch (kind) {
    case 'magnitude': return 'Bottom line'
    case 'comparison': return 'Checks and comparisons'
    case 'uncertainty': return 'Uncertainty'
    case 'qualification': return 'What must be true'
    default: return assertNever(kind)
  }
}

const sectionClass = (kind: StatementKind): string => {
  switch (kind) {
    case 'magnitude': return 'text-ink'
    case 'uncertainty':
    case 'comparison': return 'text-muted'
    case 'qualification': return 'text-faint'
    default: return assertNever(kind)
  }
}

const interpretationTitle = (context: InterpretationContext): string => {
  switch (context) {
    case 'result': return 'What this result means'
    case 'sensitivity-check': return 'What this check means'
    case 'discovery-run': return 'What this discovery run means'
    default: return assertNever(context)
  }
}

/** The shared prose companion to a numerical result. Every sentence comes from recorded facts. */
export function ResultInterpretation({ interpretation, className = '', context = 'result' }: {
  readonly interpretation: ResultInterpretationModel
  readonly className?: string
  readonly context?: InterpretationContext
}) {
  const sections: readonly InterpretationSection[] = SECTION_ORDER.flatMap((kind) => {
    const statements = interpretation.statements.filter((statement) => statement.kind === kind)
    return isNonEmpty(statements) ? [{ kind, statements }] : []
  })

  return (
    <section className={`py-3 ${className}`.trim()} aria-label="Interpretation">
      <h4 className="m-0 text-faint text-label font-medium">{interpretationTitle(context)}</h4>
      <div className="mt-4 space-y-5">
        {sections.map((section) => (
          <section key={section.kind}>
            <h5 className="m-0 text-label font-medium text-bone">{sectionLabel(section.kind)}</h5>
            <div className="mt-1 space-y-1.5">
              {section.statements.map((statement, statementIndex) => (
                <p key={`${statement.kind}-${statementIndex}`} className={prose(`m-0 ${sectionClass(statement.kind)}`)}>{statement.text}</p>
              ))}
            </div>
          </section>
        ))}
      </div>
    </section>
  )
}
