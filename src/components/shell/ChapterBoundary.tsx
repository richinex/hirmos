import { Component, type ErrorInfo, type ReactNode } from 'react'
import { Alert } from '@/components/ui/Alert'

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
      <Alert tone="warn" live={false}>
        <p className="m-0">The {this.props.chapter} chapter could not render: {this.state.error.message}.</p>
        <p className="mb-0 mt-1 text-muted">A recorded run may have been saved by an earlier build. Open another chapter, or delete the run and run it again.</p>
      </Alert>
    )
  }
}
