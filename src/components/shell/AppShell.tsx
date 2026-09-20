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

/** Header, chapter rail, and the stage as flex siblings. The header and the rail are the band: one blue shape along the top and the left, textured the same way; the row is the shell container the rail queries. */
export function AppShell({ skipTarget, mode, header, nav, stage, footer }: AppShellProps) {
  return (
    // Vaul scales this element back while a bottom sheet is open, which is how the sheet reads as a
    // layer on the dark themes, where a scrim over a near-black page dims almost nothing.
    <div data-vaul-drawer-wrapper className="dashboard flex h-full min-h-0 bg-stage text-ink">
      <a href={`#${skipTarget}`} className="sr-only rounded-md border border-edge bg-panel px-3 py-1.5 text-body text-ink no-underline focus:not-sr-only focus:fixed focus:left-3 focus:top-3 focus:z-(--z-overlay)">Skip to workspace</a>
      <div className="@container/shell relative flex min-h-0 flex-1">
        {nav}
        <div className="dashboard-workspace flex min-w-0 flex-1 flex-col">
      <header className="dashboard-header band-texture flex shrink-0 items-center justify-between gap-4 px-5">
        {header}
      </header>

        <main id={skipTarget} className="flex min-h-0 min-w-0 flex-1">
          {mode === 'reading' ? (
            <div className="panel-scroll min-w-0 flex-1 overflow-y-auto [container-type:size]">
              <div className="dashboard-reading @container/panel mx-auto flex min-h-full w-full max-w-[1440px] flex-col px-5 py-8 sm:px-8 lg:px-10">{stage}</div>
            </div>
          ) : stage}
        </main>
      {footer}
        </div>
      </div>
    </div>
  )
}
