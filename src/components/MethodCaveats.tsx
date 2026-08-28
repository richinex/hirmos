import { useId } from 'react'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import type { MethodDefinition, MethodSource } from '@/domain/methods'
import { label } from '@/components/ui/recipes'

interface MethodCaveatsProps {
  readonly methods: NonEmptyArray<MethodDefinition>
}

function sourceLabel(source: MethodSource): string {
  switch (source.kind) {
    case 'reference-implementation': return `${source.repository}@${source.revision.slice(0, 8)} · ${source.locator}`
    case 'paper': return `${source.title} · ${source.locator}`
    case 'hirmos-constraint': return `Hirmos boundary · ${source.locator}`
    default: return assertNever(source)
  }
}

export function MethodCaveats({ methods }: MethodCaveatsProps) {
  const titleId = useId()
  return (
    <section className="mt-4 rounded-lg border border-hair bg-well p-3" aria-labelledby={titleId}>
      <span className={label('text-faint')}>Assumptions and limits</span>
      <h4 id={titleId} className="mb-1 mt-1 text-body font-medium text-ink">Review before running</h4>
      <p className="mb-3 mt-0 text-body text-faint">Each method answers a narrower question than a causal conclusion. Sources are pinned to the reference revision used by the port.</p>
      <div className="space-y-2">
        {methods.map((method) => (
          <details key={method.id} className="rounded-md border border-hair bg-panel px-3 py-2">
            <summary className="cursor-pointer text-body font-medium text-ink">
              {method.name} · {method.caveats.length} caveats
            </summary>
            <p className="mb-2 mt-2 text-body text-muted">{method.summary}</p>
            <ul className="m-0 space-y-3 pl-4 text-body text-muted">
              {method.caveats.map((caveat) => (
                <li key={caveat.id}>
                  <p className="m-0 text-ink">{caveat.requirement}</p>
                  <p className="mb-1 mt-0.5 text-faint">If unmet: {caveat.consequenceIfUnmet}</p>
                  <p className="m-0 text-micro text-faint">Source: {caveat.sources.map(sourceLabel).join('; ')}</p>
                </li>
              ))}
            </ul>
          </details>
        ))}
      </div>
    </section>
  )
}
