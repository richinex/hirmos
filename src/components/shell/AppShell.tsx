import type { ReactNode } from 'react'

interface AppShellProps {
  /** Id of the stage element the skip link jumps to. */
  readonly skipTarget: string
  /** 'reading' keeps the centred column; 'full' hands the whole stage to the chapter, which draws its own panes. */
  readonly mode: 'reading' | 'full'
  readonly header: ReactNode
  readonly nav: ReactNode
  readonly stage: ReactNode
  readonly footer?: ReactNode
}

/** Header, chapter nav, and the stage as flex siblings behind hairlines: no overlays or shadows on desktop. The row is the shell container the nav queries. */
export function AppShell({ skipTarget, mode, header, nav, stage, footer }: AppShellProps) {
  return (
    <div className="flex h-full min-h-0 flex-col bg-stage text-ink">
      <a href={`#${skipTarget}`} className="sr-only rounded-md border border-edge bg-panel px-3 py-1.5 text-body text-ink focus:not-sr-only focus:fixed focus:left-3 focus:top-3 focus:z-(--z-overlay)">Skip to workspace</a>
      <header className="flex h-12 shrink-0 items-center justify-between gap-4 border-b border-line bg-panel px-3.5">
        {header}
      </header>

      <div className="@container/shell relative flex min-h-0 flex-1">
        {nav}
        <main id={skipTarget} className="flex min-w-0 flex-1">
          {mode === 'reading' ? (
            <div className="panel-scroll min-w-0 flex-1 overflow-y-auto [container-type:size]">
              <div className="@container/panel mx-auto flex min-h-full w-full max-w-5xl flex-col px-5 py-8 sm:px-8 lg:px-12">{stage}</div>
            </div>
          ) : stage}
        </main>
      </div>
      {footer}
    </div>
  )
}
