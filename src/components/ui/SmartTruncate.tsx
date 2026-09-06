import { cn } from '@/lib/utils'

/**
 * A one-line label that fades at its right edge when the whole text would not fit, instead of
 * wrapping or cutting to an ellipsis. The text is laid out twice: a clipped single-line copy is what
 * the reader sees, and the real text wraps freely inside a zero-width sensor column whose height is
 * a size container. When that height reaches a second line, the container query in index.css
 * (`smart-truncate`) turns the fade on. No script measures anything; the layout answers the question.
 * The fade takes its colour from `--smart-truncate-surface`, so a row sets it to its own background.
 */
export function SmartTruncate({ text, className, textClassName, title }: {
  readonly text: string
  readonly className?: string
  readonly textClassName?: string
  readonly title?: string
}) {
  return (
    <span className={cn('smart-truncate', className)} title={title ?? text}>
      <span className="smart-truncate__frame">
        <span className="smart-truncate__grid">
          <span className={cn('smart-truncate__layers', textClassName)} data-text={text}>
            <span className="smart-truncate__text">{text}</span>
          </span>
          <span className="smart-truncate__sensor" aria-hidden="true">
            <span className="smart-truncate__fade" />
          </span>
        </span>
      </span>
    </span>
  )
}
