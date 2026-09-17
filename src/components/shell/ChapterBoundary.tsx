import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { Component, type ErrorInfo, type ReactNode } from 'react'
import { Alert } from '@/components/ui/Alert'
import { button, literal } from '@/components/ui/recipes'

/**
 * Keeps one failing chapter from blanking the workbench. A chapter renders recorded artifacts, and an
 * artifact written by an earlier build can lack a field the current renderer reads; the boundary shows
 * what failed and where, and clears when the reader moves to another chapter (the host keys it by chapter).
 */
export class ChapterBoundary extends Component<{ readonly chapter: string; readonly children: ReactNode }, { readonly error: Error | null }> {
  override state: { readonly error: Error | null } = { error: null }

  static getDerivedStateFromError(error: Error): { readonly error: Error } {
    return { error }
  }

  override componentDidCatch(error: Error, info: ErrorInfo): void {
    console.error(`Chapter "${this.props.chapter}" failed to render`, error, info.componentStack)
  }

  override render(): ReactNode {
    if (this.state.error === null) return this.props.children
    return (
      <Alert tone="warn" live={false} className="rise my-auto max-w-2xl">
        <p className="m-0">The {this.props.chapter} chapter could not be displayed.</p>
        <p className="mb-0 mt-1 text-muted">
          A run recorded by an earlier build of Hirmos is usually the cause: it does not carry a field this version reads.
          Delete that run from another chapter that still opens, or reset the example project in Projects, then run it again.
        </p>
        <div className="mt-3 flex flex-wrap items-center gap-2">
          <button type="button" className={button('quiet')} onClick={() => this.setState({ error: null })}>Try again</button>
        </div>
        <details className="mt-3">
          <DisclosureSummary className="cursor-pointer text-label text-muted">Technical detail</DisclosureSummary>
          <p className={literal('mb-0 mt-2 break-words text-body text-faint')}>{this.state.error.message}</p>
        </details>
      </Alert>
    )
  }
}
