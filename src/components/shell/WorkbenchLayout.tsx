import { createContext, useContext, useEffect, useRef, useState, type ReactNode } from 'react'
import { Group, Panel, Separator, useDefaultLayout, usePanelRef } from 'react-resizable-panels'
import { Icon } from '@/components/Icon'
import { Sheet } from '@/components/ui/Sheet'
import { button, iconControl, label, panelTitle } from '@/components/ui/recipes'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'
import { useShellLayout } from './useShellLayout'

export interface WorkbenchPane {
  readonly title: string
  /** Short mobile action label; the sheet and accessible name retain the full title. */
  readonly trigger?: { readonly label: string; readonly icon: string }
  readonly body: ReactNode
  /** Rendered in the pane header beside the title: tabs, counts, a close control. */
  readonly controls?: ReactNode
  /** Initial height in pixels for the bottom pane; ignored for the inspector. */
  readonly defaultSize?: number
  /** Start with only the bottom header visible. */
  readonly defaultCollapsed?: boolean
}

function PaneHeader({ id, title, controls, collapse }: {
  readonly id?: string
  readonly title: string
  readonly controls?: ReactNode
  /** Collapse or expand the pane; the header stays visible when collapsed so the pane can be reopened. */
  readonly collapse?: { readonly collapsed: boolean; readonly onToggle: () => void; readonly icon: { readonly open: string; readonly closed: string } }
}) {
  return (
    <div className="flex h-9 shrink-0 select-none items-center justify-between gap-2 border-b border-hair px-3">
      {/* One line, whatever the pane width: the title clips rather than wrapping into the 36px header.
          `text-nowrap` replaces the recipe's balanced wrapping, which would otherwise win over the clip,
          because `text-wrap: balance` also sets the wrap mode back to wrapping. The real text stays in the
          heading, so its accessible name and a test's text query both see it. */}
      <h2 id={id} className={label('m-0 min-w-0 truncate text-nowrap text-ink')} title={title}>{title}</h2>
      <div className="flex items-center gap-1.5">
        {!collapse?.collapsed && controls}
        {collapse && (
          <button
            type="button"
            className={iconControl('quiet', 'h-7 w-7 text-faint')}
            aria-expanded={!collapse.collapsed}
            aria-label={`${collapse.collapsed ? 'Expand' : 'Collapse'} ${title}`}
            title={`${collapse.collapsed ? 'Expand' : 'Collapse'} ${title}`}
            onClick={collapse.onToggle}
          >
            <Icon name={collapse.collapsed ? collapse.icon.closed : collapse.icon.open} size={15} />
          </button>
        )}
      </div>
    </div>
  )
}

const PANE_HEADER_HEIGHT = 36

type PhonePane = 'inspector' | 'bottom'

/**
 * Lets content inside a pane surface another pane on phones, where panes are sheets: selecting an
 * arrow in the ledger opens the inspector that edits it. On desktop every pane is already visible,
 * so the default request is a no-op.
 */
const PaneRequestContext = createContext<(pane: PhonePane) => void>(() => {})
const PaneCloseContext = createContext<() => void>(() => {})

export const useOpenPane = (): ((pane: PhonePane) => void) => useContext(PaneRequestContext)
export const useClosePane = (): (() => void) => useContext(PaneCloseContext)

/** Independent actions open contextual sheets; they do not select a workbench mode. */
function PaneOpener({ pane, icon, open, onOpen }: { readonly pane: WorkbenchPane; readonly icon: string; readonly open: boolean; readonly onOpen: () => void }) {
  return (
    <button
      type="button"
      className={button('outline', 'sheet-opener min-w-0 w-full whitespace-normal text-center')}
      aria-label={pane.title}
      aria-haspopup="dialog"
      aria-expanded={open}
      onClick={onOpen}
    >
      <Icon name={pane.trigger?.icon ?? icon} size={18} />
      <span className="min-w-0 [overflow-wrap:anywhere]">{pane.trigger?.label ?? pane.title}</span>
    </button>
  )
}

/** Below `md` the panes become bottom sheets opened from a bar under the stage, as Octopus does for its drawer. */
function PhoneWorkbench({ stage, inspector, bottom, stagePadding }: {
  readonly stage: ReactNode
  readonly inspector?: WorkbenchPane
  readonly bottom?: WorkbenchPane
  readonly stagePadding: boolean
}) {
  const [open, setOpen] = useState<PhonePane | null>(null)
  return (
    <PaneRequestContext.Provider value={setOpen}>
    <PaneCloseContext.Provider value={() => setOpen(null)}>
    <div className="flex min-w-0 flex-1 flex-col">
      <div className="panel-scroll min-h-0 flex-1 overflow-y-auto">
        <div className={cn('flex min-h-full w-full flex-col', stagePadding && 'px-4 py-5')}>{stage}</div>
      </div>
      {(inspector || bottom) && (
        <div className="shrink-0 bg-panel px-3 pb-[max(0.5rem,env(safe-area-inset-bottom))] pt-2">
          <div className={cn('grid items-stretch gap-2', inspector && bottom ? 'grid-cols-2' : 'grid-cols-1')} role="group" aria-label="Panes">
            {inspector && <PaneOpener pane={inspector} icon="tune" open={open === 'inspector'} onOpen={() => setOpen('inspector')} />}
            {bottom && <PaneOpener pane={bottom} icon="bottom_panel_open" open={open === 'bottom'} onOpen={() => setOpen('bottom')} />}
          </div>
        </div>
      )}
      {inspector && (
        <Sheet open={open === 'inspector'} onClose={() => setOpen(null)} title={inspector.title}>
          {inspector.controls && <div className="mb-2 flex justify-end">{inspector.controls}</div>}
          <div className="inspector-content">{inspector.body}</div>
        </Sheet>
      )}
      {bottom && (
        <Sheet open={open === 'bottom'} onClose={() => setOpen(null)} title={bottom.title}>
          {bottom.controls && <div className="mb-2 flex justify-end">{bottom.controls}</div>}
          {bottom.body}
        </Sheet>
      )}
    </div>
    </PaneCloseContext.Provider>
    </PaneRequestContext.Provider>
  )
}

/**
 * A chapter's full-bleed layout: the stage, an optional right inspector, and an optional bottom panel,
 * each a flex sibling behind a hairline seam. Pane sizes persist per chapter through the shell store.
 */
export function WorkbenchLayout(props: {
  readonly id: string
  readonly stage: ReactNode
  readonly inspector?: WorkbenchPane
  readonly bottom?: WorkbenchPane
  readonly stagePadding?: boolean
  /** False when the stage owns its height (a canvas that fills the pane) instead of scrolling. */
  readonly stageScroll?: boolean
}) {
  const mobile = useIsMobile()
  return mobile
    ? <PhoneWorkbench stage={props.stage} inspector={props.inspector} bottom={props.bottom} stagePadding={props.stagePadding ?? true} />
    : <DesktopWorkbench {...props} />
}

function DesktopWorkbench({ id, stage, inspector, bottom, stagePadding = true, stageScroll = true }: {
  readonly id: string
  readonly stage: ReactNode
  readonly inspector?: WorkbenchPane
  readonly bottom?: WorkbenchPane
  readonly stagePadding?: boolean
  readonly stageScroll?: boolean
}) {
  const shell = useShellLayout()
  const bottomRef = usePanelRef()
  const inspectorRef = usePanelRef()
  const [bottomCollapsed, setBottomCollapsed] = useState(bottom?.defaultCollapsed ?? false)
  const [inspectorCollapsed, setInspectorCollapsed] = useState(false)
  const toggle = (ref: ReturnType<typeof usePanelRef>, collapsed: boolean) => {
    const panel = ref.current
    if (panel === null) return
    if (collapsed) panel.expand()
    else panel.collapse()
  }
  const { defaultLayout, onLayoutChanged } = useDefaultLayout({
    id: `hirmos.shell.${id}`,
    storage: shell.storage,
    onlySaveAfterUserInteractions: true,
  })
  // The panel group resolves pixel constraints against its own width at mount; mounted inside a box
  // that has not been laid out yet, it measures zero and collapses the collapsible panes for good.
  // The group therefore waits for the host to report a width.
  const host = useRef<HTMLDivElement>(null)
  const [measured, setMeasured] = useState(false)
  useEffect(() => {
    const element = host.current
    if (element === null) return
    if (element.getBoundingClientRect().width > 0) {
      setMeasured(true)
      return
    }
    const observer = new ResizeObserver((entries) => {
      if (entries.some((entry) => entry.contentRect.width > 0)) setMeasured(true)
    })
    observer.observe(element)
    return () => observer.disconnect()
  }, [])
  return (
    <div ref={host} className="flex min-w-0 flex-1">
      {measured && (
    <Group orientation="horizontal" defaultLayout={defaultLayout} onLayoutChanged={onLayoutChanged} className="min-w-0 flex-1">
      <Panel id="stage" minSize={440} className="flex min-w-0 flex-col">
        <Group orientation="vertical">
          <Panel id="canvas" minSize={200} className={cn('flex min-h-0 flex-col [container-type:size] [container-name:layout]', stageScroll ? 'panel-scroll overflow-y-auto' : 'overflow-hidden')}>
            <div className={cn('flex w-full flex-col', stageScroll ? 'min-h-full' : 'h-full min-h-0', stagePadding && 'px-5 py-6')}>{stage}</div>
          </Panel>
          {bottom && (
            <>
              <Separator className="seam seam-h" aria-label={`Resize ${bottom.title}`} />
              <Panel
                id="bottom"
                panelRef={bottomRef}
                collapsible
                collapsedSize={PANE_HEADER_HEIGHT + 1}
                minSize={120}
                maxSize="50%"
                defaultSize={bottom.defaultCollapsed ? PANE_HEADER_HEIGHT + 1 : bottom.defaultSize ?? 240}
                onResize={(size) => { if (size.inPixels > 0) setBottomCollapsed(size.inPixels <= PANE_HEADER_HEIGHT + 2) }}
                className="flex flex-col border-t border-line bg-column [container-type:size] [container-name:layout_bottom]"
              >
                <PaneHeader title={bottom.title} controls={bottom.controls} collapse={{ collapsed: bottomCollapsed, onToggle: () => toggle(bottomRef, bottomCollapsed), icon: { open: 'keyboard_arrow_down', closed: 'keyboard_arrow_up' } }} />
                {!bottomCollapsed && <div className="panel-scroll min-h-0 flex-1 overflow-y-auto">{bottom.body}</div>}
              </Panel>
            </>
          )}
        </Group>
      </Panel>
      {inspector && (
        <>
          <Separator className="seam seam-v" aria-label={`Resize ${inspector.title}`} />
          <Panel
            id="inspector"
            panelRef={inspectorRef}
            collapsible
            collapsedSize={PANE_HEADER_HEIGHT + 4}
            minSize={260}
            maxSize={520}
            defaultSize={320}
            onResize={(size) => { if (size.inPixels > 0) setInspectorCollapsed(size.inPixels <= PANE_HEADER_HEIGHT + 5) }}
            className="flex flex-col border-l border-line bg-panel [container-type:size] [container-name:layout_inspector]"
          >
            <aside aria-labelledby={`${id}-inspector-title`} className="flex min-h-0 flex-1 flex-col">
              {inspectorCollapsed ? (
                <button type="button" className="flex h-full w-full flex-col items-center gap-2 pt-2 text-faint hover:text-ink" aria-expanded={false} aria-label={`Expand ${inspector.title}`} title={`Expand ${inspector.title}`} onClick={() => toggle(inspectorRef, true)}>
                  <Icon name="left_panel_open" size={16} />
                  <span id={`${id}-inspector-title`} className={cn(panelTitle, '[writing-mode:vertical-rl]')}>{inspector.title}</span>
                </button>
              ) : (
                <>
                  <PaneHeader id={`${id}-inspector-title`} title={inspector.title} controls={inspector.controls} collapse={{ collapsed: false, onToggle: () => toggle(inspectorRef, false), icon: { open: 'right_panel_close', closed: 'right_panel_open' } }} />
                  <div className="inspector-content panel-scroll min-h-0 flex-1 overflow-y-auto p-3">{inspector.body}</div>
                </>
              )}
            </aside>
          </Panel>
        </>
      )}
    </Group>
      )}
    </div>
  )
}
