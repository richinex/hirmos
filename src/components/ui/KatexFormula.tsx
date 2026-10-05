import { useMemo } from 'react'
import katex from 'katex'
import 'katex/dist/katex.min.css'
import { FormulaText, type FormulaProps } from './Formula'

/** The TeX comes from the domain, never from a user, so its HTML can be set directly. Inline mode, not
 * display: inline maths breaks lines after top-level relations and operators, so on a phone the
 * expression wraps like a sentence instead of panning. */
export default function KatexFormula({ tex, plain }: FormulaProps) {
  const html = useMemo(() => {
    try {
      return katex.renderToString(tex, { displayMode: false, throwOnError: false, strict: 'warn' })
    } catch {
      return null
    }
  }, [tex])
  if (html === null) return <FormulaText plain={plain} />
  return (
    <p
      className="formula m-0 min-w-0"
      aria-label={plain}
      dangerouslySetInnerHTML={{ __html: html }}
    />
  )
}
