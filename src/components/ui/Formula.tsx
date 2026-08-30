import { lazy, Suspense } from 'react'
import { literal } from './recipes'

const Typeset = lazy(() => import('./KatexFormula'))

export interface FormulaProps {
  readonly tex: string
  /** The same expression as text: shown until the typesetter loads, read by assistive technology, and the record's own field. */
  readonly plain: string
}

/** The plain expression, wrapping wherever it must so it never widens a phone panel. */
export function FormulaText({ plain }: Pick<FormulaProps, 'plain'>) {
  return <p className={literal('m-0 min-w-0 text-body text-muted [overflow-wrap:anywhere]')}>{plain}</p>
}

/** A typeset expression: KaTeX in its own chunk, the plain text until it arrives or if the TeX fails. */
export function Formula(props: FormulaProps) {
  return (
    <Suspense fallback={<FormulaText plain={props.plain} />}>
      <Typeset {...props} />
    </Suspense>
  )
}
