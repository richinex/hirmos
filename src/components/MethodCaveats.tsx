import { Icon } from '@/components/Icon'
import { Tooltip } from '@/components/ui/Tooltip'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import { isStageNote, type CaveatEvaluation, type MethodCaveat, type MethodDefinition, type MethodEligibility, type MethodSource } from '@/domain/methods'
import type { Identification } from '@/domain/study'
import { IdentificationRecord } from '@/components/IdentificationRecord'
import { prose } from '@/components/ui/recipes'

interface MethodCaveatsProps {
  readonly methods: NonEmptyArray<MethodDefinition>
  readonly eligibility?: MethodEligibility | null
  readonly identification?: Identification | null
  /** Folds that belong in the same list ahead of the methods. */
  readonly leading?: React.ReactNode
}

/** The works behind a set of sources, each once. Implementation references stay in the method record and the manifest. */
export const literatureOf = (sources: readonly MethodSource[]): readonly string[] =>
  [...new Set(sources.flatMap((source) => (source.kind === 'paper' ? [`${source.title}, ${source.locator}`] : [])))]

const literature = (method: MethodDefinition): readonly string[] => literatureOf(method.caveats.flatMap((caveat) => caveat.sources))

/** One disclosure row in the requirements panel: a name, an optional tally, the body, and the literature line. */
export function RequirementsFold({ name, tally: tallyText = null, open = false, literature: works = [], children }: {
  readonly name: string
  readonly tally?: React.ReactNode
  readonly open?: boolean
  readonly literature?: readonly string[]
  readonly children: React.ReactNode
}) {
  return (
    <details className="group" open={open}>
      <summary className="grid cursor-pointer list-none grid-cols-[auto_minmax(0,1fr)] items-start gap-x-2 gap-y-1 rounded-md py-2 text-body transition-colors hover:bg-well [&::-webkit-details-marker]:hidden">
        <Icon name="expand_more" size={14} className="shrink-0 self-center text-faint transition-transform duration-(--motion-fast) group-open:rotate-180" />
        <span className="min-w-0 break-words font-medium text-ink">{name}</span>
        {tallyText !== null && <span className="col-start-2 text-label text-faint">{tallyText}</span>}
      </summary>
      {children}
      {works.length > 0 && <p className="mb-2 mt-2 text-label text-faint">Literature: {works.join('; ')}</p>}
    </details>
  )
}

const eligibilityEvaluations = (eligibility: MethodEligibility | null | undefined): readonly CaveatEvaluation[] => {
  if (eligibility === null || eligibility === undefined) return []
  switch (eligibility.kind) {
    case 'eligible': return eligibility.satisfied
    case 'caution': return [...eligibility.satisfied, ...eligibility.unresolved]
    case 'refused': return [...eligibility.satisfied, ...eligibility.unresolved, ...eligibility.violations]
    default: return assertNever(eligibility)
  }
}

function Status({ evaluation }: { readonly evaluation: CaveatEvaluation | undefined }) {
  if (evaluation === undefined) return <span className="whitespace-nowrap text-label text-faint">Not checked</span>
  switch (evaluation.kind) {
    case 'satisfied': return <span className="whitespace-nowrap text-label text-ok">Checked</span>
    case 'unresolved':
      return (
        <Tooltip text="There is no recorded evidence for this requirement.">
          <span tabIndex={0} className="cursor-help whitespace-nowrap text-label text-warn underline decoration-dotted underline-offset-2">Not checked</span>
        </Tooltip>
      )
    case 'violated': return <span className="whitespace-nowrap text-label text-danger">Fails</span>
    default: return assertNever(evaluation)
  }
}

const evidenceText = (evaluation: CaveatEvaluation | undefined): string | null => {
  if (evaluation === undefined) return null
  switch (evaluation.kind) {
    case 'satisfied': return evaluation.evidence
    case 'unresolved': return evaluation.missingEvidence
    case 'violated': return evaluation.evidence
    default: return assertNever(evaluation)
  }
}

/** Conditions the project can meet or fail, then the rules for reading the result, which nobody assesses. */
const conditions = (method: MethodDefinition): readonly MethodCaveat[] => method.caveats.filter((caveat) => caveat.category !== 'interpretation')
const readingRules = (method: MethodDefinition): readonly MethodCaveat[] => method.caveats.filter((caveat) => caveat.category === 'interpretation')

const tallyStyle = {
  satisfied: { icon: 'check_circle', label: 'checked', tone: 'text-ok' },
  unresolved: { icon: 'help_outline', label: 'to review', tone: 'text-warn' },
  violated: { icon: 'error', label: 'failed', tone: 'text-danger' },
} as const satisfies Record<CaveatEvaluation['kind'], { readonly icon: string; readonly label: string; readonly tone: string }>

const tally = (method: MethodDefinition, evaluations: ReadonlyMap<string, CaveatEvaluation>): React.ReactNode => {
  const counts = { satisfied: 0, unresolved: 0, violated: 0 }
  for (const caveat of conditions(method)) {
    const evaluation = evaluations.get(caveat.id)
    if (evaluation !== undefined) counts[evaluation.kind] += 1
  }
  const statuses = (['satisfied', 'unresolved', 'violated'] as const).filter((kind) => counts[kind] > 0)
  const total = conditions(method).length
  if (statuses.length === 0) return `${total} condition${total === 1 ? '' : 's'}`
  return <span className="inline-flex flex-wrap items-center gap-x-4 gap-y-1" data-testid="requirement-tally">
    {statuses.map((kind) => <span key={kind} className={`inline-flex items-center gap-1.5 whitespace-nowrap ${tallyStyle[kind].tone}`}>
      <Icon name={tallyStyle[kind].icon} size={15} />
      <span><span className="tabular-nums">{counts[kind]}</span> {tallyStyle[kind].label}</span>
    </span>)}
  </span>
}

export function MethodCaveats({ methods, eligibility = null, identification = null, leading = null }: MethodCaveatsProps) {
  const evaluations = new Map(eligibilityEvaluations(eligibility).map((evaluation) => [evaluation.caveat.id, evaluation]))
  // Without an evaluator (diagnostics, identification methods) a status column is noise: the method reads as prose.
  const evaluated = eligibility !== null && eligibility !== undefined
  return (
    <section className="mt-6" aria-label="Method requirements">
      {identification !== null && <IdentificationRecord identification={identification} />}
      <div className="space-y-4">
        {leading}
        {methods.map((method) => {
          return (
            <RequirementsFold key={method.id} name={method.name} tally={evaluated ? tally(method, evaluations) : null} open={methods.length === 1} literature={literature(method)}>
              {!evaluated && (
                <p className={prose('mb-2 mt-0 text-muted')}>
                  {[...(method.summary.length > 0 ? [method.summary] : []), ...method.caveats.flatMap((caveat) => (caveat.category === 'interpretation' ? [caveat.requirement] : [caveat.requirement, `If this is not met: ${caveat.consequenceIfUnmet}`]))].join(' ')}
                </p>
              )}
              {evaluated && method.summary.length > 0 && <p className={prose('mb-2 mt-0 text-muted')}>{method.summary}</p>}
              {evaluated && <ol className="m-0 list-none space-y-2 p-0">
                {[...conditions(method), ...readingRules(method)].map((caveat) => {
                  const reading = caveat.category === 'interpretation'
                  const evaluation = reading ? undefined : evaluations.get(caveat.id)
                  const evidence = evaluation !== undefined && isStageNote(evaluation) ? null : evidenceText(evaluation)
                  return (
                    <li key={caveat.id} className="grid grid-cols-[minmax(0,1fr)_auto] gap-x-3 gap-y-0.5 py-2 text-body">
                      <p className="m-0 text-ink">{caveat.requirement}</p>
                      {reading ? <span className="whitespace-nowrap text-label text-faint">Interpretation</span> : <Status evaluation={evaluation} />}
                      {evidence !== null && evidence.length > 0 && <p className="col-span-2 m-0 text-muted">{evidence}</p>}
                      {!reading && evaluation?.kind !== 'satisfied' && <p className="col-span-2 m-0 text-faint">If this is not met: {caveat.consequenceIfUnmet}</p>}
                    </li>
                  )
                })}
              </ol>}
            </RequirementsFold>
          )
        })}
      </div>
    </section>
  )
}
