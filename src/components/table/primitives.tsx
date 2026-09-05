import { useEffect, useId, useRef, useState, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { Icon } from '@/components/Icon'
import { escapeFor, pushLayer } from '@/lib/dismissal'
import { useShellLayout } from '@/components/shell/useShellLayout'
import { literal, num, panel, pill, rowPadding, segment, tableFoot, th } from '@/components/ui/recipes'
import type { HistogramBins } from '@/domain/dataset'
import type { TableDensity } from '@/domain/shellLayout'
import { formatCount } from '@/lib/format/number'
import { cn } from '@/lib/utils'

/**
 * The pieces every data table shares: the panel shell with its toolbar and live count line, the
 * sortable header cell, the filter field, facet pills, the density switch, and the per-column action
 * menu. Styling comes from the recipes; nothing here knows what the rows are.
 */

/** Row pitch per density, hairline included; the recipes' `rowPadding` is derived to hit it exactly. */
export const ROW_HEIGHT: Record<TableDensity, number> = { compact: 24, comfortable: 32 }

/** Cell padding that lands the row on the density's height with 12px body text and its 1px hairline. */
export const cellPadding = (density: TableDensity): string => rowPadding[density]

export function useTableDensity(): readonly [TableDensity, (density: TableDensity) => void] {
  const shell = useShellLayout()
  return [shell.tableDensity, shell.setTableDensity]
}

export function DensityToggle({ density, onChange }: { readonly density: TableDensity; readonly onChange: (density: TableDensity) => void }) {
  return (
    <div className="flex gap-0.5 rounded-md border border-hair bg-panel p-0.5" role="group" aria-label="Row density">
      <button type="button" className={segment(density === 'comfortable', 'h-[18px] px-1.5 py-0')} aria-pressed={density === 'comfortable'} title="Comfortable rows" aria-label="Comfortable rows" onClick={() => onChange('comfortable')}><Icon name="density_medium" size={13} /></button>
      <button type="button" className={segment(density === 'compact', 'h-[18px] px-1.5 py-0')} aria-pressed={density === 'compact'} title="Compact rows" aria-label="Compact rows" onClick={() => onChange('compact')}><Icon name="density_small" size={13} /></button>
    </div>
  )
}

export function TableShell({ title, titleId, toolbar, lead, count, foot, children, className, scrollRef, collapsible = false, maxHeight = 'max-h-[clamp(240px,52cqb,560px)]', frame = 'panel' }: {
  readonly title: string
  readonly titleId: string
  /** Search, facets, chips: the row under the title. */
  readonly toolbar?: ReactNode
  readonly lead?: ReactNode
  /** The live row-count line under the table. */
  readonly count: ReactNode
  readonly foot?: ReactNode
  readonly children: ReactNode
  readonly className?: string
  readonly scrollRef?: React.RefObject<HTMLDivElement | null>
  /** Let the title fold the body away, for a table read once and then kept out of the way. */
  readonly collapsible?: boolean
  readonly maxHeight?: string
  /**
   * `panel` gives the table its own card; `none` leaves it on the surface it sits on.
   *
   * A table dropped straight into a section of the same fill and the same radius draws a second
   * border around the first for no gain. Its own header rule already separates the head from the
   * body, which is the division a reader actually uses.
   */
  readonly frame?: 'panel' | 'none'
}) {
  const [open, setOpen] = useState(true)
  const folded = collapsible && !open
  return (
    <section className={cn('flex min-h-0 min-w-0 flex-col overflow-hidden', frame === 'panel' && panel(), className)} aria-labelledby={titleId}>
      <div className="flex flex-wrap items-center justify-between gap-x-3 gap-y-2 border-b border-hair px-3.5 py-2">
        {collapsible ? (
          <button
            type="button"
            onClick={() => setOpen((value) => !value)}
            aria-expanded={open}
            aria-controls={`${titleId}-body`}
            className="-mx-1 flex items-center gap-1.5 rounded px-1 text-title font-medium text-ink transition-colors hover:text-muted"
          >
            <Icon name="expand_more" size={14} className={cn('shrink-0 transition-transform duration-150', open && 'rotate-180')} />
            <span id={titleId}>{title}</span>
          </button>
        ) : (
          <h3 id={titleId} className="m-0 text-title font-medium text-ink">{title}</h3>
        )}
        {toolbar && !folded && <div className="flex min-w-0 flex-wrap items-center gap-2">{toolbar}</div>}
      </div>
      {!folded && lead}
      <div id={`${titleId}-body`} hidden={folded} ref={scrollRef} className={cn('figure-strip panel-scroll min-h-0 overflow-auto', maxHeight)}>
        {children}
      </div>
      {!folded && foot}
      {!folded && <p aria-live="polite" className={cn(tableFoot, 'm-0')}>{count}</p>}
    </section>
  )
}

export type SortState = 'asc' | 'desc' | false

/** A header cell whose whole surface toggles the sort; the arrow is decoration, `aria-sort` is the fact. */
export function SortHeader({ sorted, canSort = true, onToggle, align = 'left', children, className, title }: {
  readonly sorted: SortState
  readonly canSort?: boolean
  readonly onToggle: () => void
  readonly align?: 'left' | 'right'
  readonly children: ReactNode
  readonly className?: string
  readonly title?: string
}) {
  return (
    <th
      scope="col"
      aria-sort={sorted === 'asc' ? 'ascending' : sorted === 'desc' ? 'descending' : undefined}
      className={th(cn('p-0', align === 'right' && 'text-right', className))}
    >
      {canSort ? (
        <button
          type="button"
          onClick={onToggle}
          title={title ?? (sorted === 'asc' ? 'Sorted ascending. Click to sort descending.' : sorted === 'desc' ? 'Sorted descending. Click to clear the sort.' : 'Click to sort ascending.')}
          className={cn('flex w-full items-center gap-1 whitespace-nowrap px-3.5 py-[7px] text-label font-medium transition-colors hover:text-ink', align === 'right' ? 'justify-end text-right' : 'text-left')}
        >
          <span>{children}</span>
          {sorted !== false && <Icon name={sorted === 'asc' ? 'arrow_upward' : 'arrow_downward'} size={11} className="shrink-0" />}
        </button>
      ) : (
        <span className="block whitespace-nowrap px-3.5 py-[7px]">{children}</span>
      )}
    </th>
  )
}

/** The Octopus filter field: a search glyph, the input, and a clear control that hands focus back. */
export function FilterField({ value, onChange, placeholder, label: fieldLabel, className, autoFocus }: {
  readonly value: string
  readonly onChange: (value: string) => void
  readonly placeholder: string
  readonly label: string
  readonly className?: string
  readonly autoFocus?: boolean
}) {
  const input = useRef<HTMLInputElement>(null)
  return (
    <div className={cn('flex min-w-0 items-center gap-2 rounded-md border border-hair bg-well px-2 py-[2px] focus-within:border-signal/60', className)}>
      <Icon name="search" size={13} className="shrink-0 text-faint" />
      <input
        ref={input}
        type="search"
        value={value}
        autoFocus={autoFocus}
        aria-label={fieldLabel}
        placeholder={placeholder}
        onChange={(event) => onChange(event.target.value)}
        onKeyDown={(event) => { if (event.key === 'Escape' && value.length > 0) { event.preventDefault(); onChange('') } }}
        className="min-w-0 flex-1 bg-transparent text-body text-ink outline-none placeholder:text-faint [&::-webkit-search-cancel-button]:hidden"
      />
      {value.length > 0 && (
        <button type="button" aria-label={`Clear ${fieldLabel.toLowerCase()}`} className="grid h-5 w-5 shrink-0 place-items-center text-faint hover:text-ink" onClick={() => { onChange(''); input.current?.focus() }}>
          <Icon name="close" size={12} />
        </button>
      )}
    </div>
  )
}

export interface Facet {
  readonly id: string
  readonly text: string
  readonly count: number
  readonly active: boolean
}

/** Facet pills with counts, so a reader knows whether a filter is worth applying before clicking it. */
export function FacetPills({ facets, onToggle, label: groupLabel }: { readonly facets: readonly Facet[]; readonly onToggle: (id: string) => void; readonly label: string }) {
  return (
    <div className="flex flex-wrap gap-1" role="group" aria-label={groupLabel}>
      {facets.map((facet) => (
        <button
          key={facet.id}
          type="button"
          aria-pressed={facet.active}
          disabled={facet.count === 0 && !facet.active}
          onClick={() => onToggle(facet.id)}
          className={pill(facet.active, 'min-h-6 px-2 py-0 disabled:cursor-not-allowed disabled:opacity-50 pointer-coarse:min-h-8')}
        >
          {facet.text} <span className={num('text-faint')}>{facet.count}</span>
        </button>
      ))}
    </div>
  )
}

/** A removable filter chip in a toolbar. */
export function Chip({ children, onRemove, removeLabel }: { readonly children: ReactNode; readonly onRemove: () => void; readonly removeLabel: string }) {
  return (
    <span className="inline-flex max-w-full items-center gap-1 rounded-md border border-edge bg-raised px-1.5 py-0.5 text-body text-ink">
      <span className="truncate">{children}</span>
      <button type="button" aria-label={removeLabel} className="grid h-4 w-4 shrink-0 place-items-center text-muted hover:text-ink" onClick={onRemove}><Icon name="close" size={11} /></button>
    </span>
  )
}

/** A 48×12 bar chart of the bins; decoration beside the numbers that already say the same thing. */
export function MiniHistogram({ bins, width = 48, height = 12, className }: { readonly bins: HistogramBins; readonly width?: number; readonly height?: number; readonly className?: string }) {
  const max = Math.max(1, ...bins.counts)
  const n = Math.max(1, bins.counts.length)
  const gap = n > 24 ? 0 : 1
  // Integer pitch and bar widths so every edge lands on a CSS pixel instead of an anti-aliased smear.
  const pitch = Math.max(1, Math.floor((width + gap) / n))
  const bar = Math.max(1, pitch - gap)
  const drawn = n * pitch - gap
  return (
    <svg aria-hidden viewBox={`0 0 ${drawn} ${height}`} width={drawn} height={height} shapeRendering="crispEdges" className={cn('block shrink-0', className)}>
      {bins.counts.map((count, index) => {
        const barHeight = count === 0 ? 0 : Math.max(1, Math.round((count / max) * height))
        return <rect key={index} x={index * pitch} y={height - barHeight} width={bar} height={barHeight} fill="var(--color-bone)" opacity={0.75} />
      })}
    </svg>
  )
}

/** Proportions of a low-cardinality column as one stacked bar, most frequent first. */
export function CategoryBar({ categories, total, width = 48, height = 12, className }: {
  readonly categories: readonly { readonly value: string; readonly count: number }[]
  readonly total: number
  readonly width?: number
  readonly height?: number
  readonly className?: string
}) {
  // Integer 1px gaps are reserved first; every category keeps at least one pixel so none vanishes.
  const usable = Math.max(1, width - Math.max(0, categories.length - 1))
  let x = 0
  const shares = categories.map((category) => {
    const share = total === 0 ? 0 : Math.max(1, Math.round((category.count / total) * usable))
    const start = x
    x += share + 1
    return { value: category.value, start, share }
  })
  return (
    <svg aria-hidden viewBox={`0 0 ${width} ${height}`} width={width} height={height} shapeRendering="crispEdges" className={cn('block shrink-0', className)}>
      {shares.map((entry, index) => (
        <rect key={entry.value} x={entry.start} y={height - 8} width={entry.share} height={8} fill="var(--color-bone)" opacity={index === 0 ? 0.8 : Math.max(0.2, 0.6 - index * 0.08)} />
      ))}
    </svg>
  )
}

export interface MenuItem {
  readonly id: string
  readonly text: string
  readonly icon?: string
  readonly disabled?: boolean
  readonly onSelect: () => void
}

/** The per-column cog: a menu of actions, closed by Escape, a choice, or a click elsewhere. */
export function HeaderMenu({ label: menuLabel, items, className }: { readonly label: string; readonly items: readonly MenuItem[]; readonly className?: string }) {
  const [open, setOpen] = useState(false)
  const host = useRef<HTMLDivElement>(null)
  const menu = useRef<HTMLUListElement>(null)
  // The list is portalled to the body and fixed at the button, so the table scroller and the stage
  // pane cannot clip it; any scroll or resize closes it rather than leaving it stranded.
  const [anchor, setAnchor] = useState<{ readonly top: number; readonly right: number } | null>(null)
  const id = useId()
  const menuLayer = `header-menu-${id.replaceAll(':', '')}`
  useEffect(() => (open ? pushLayer(menuLayer) : undefined), [menuLayer, open])
  useEffect(() => {
    if (!open) return
    const place = () => {
      const rect = host.current?.getBoundingClientRect()
      if (rect) setAnchor({ top: rect.bottom + 4, right: Math.max(8, window.innerWidth - rect.right) })
    }
    place()
    const onPointer = (event: PointerEvent) => {
      const target = event.target as Node
      if (host.current?.contains(target) === true || menu.current?.contains(target) === true) return
      setOpen(false)
    }
    const onKey = (event: KeyboardEvent) => { if (event.key === 'Escape' && escapeFor(menuLayer, event)) setOpen(false) }
    const onScroll = () => setOpen(false)
    document.addEventListener('pointerdown', onPointer)
    document.addEventListener('keydown', onKey)
    document.addEventListener('scroll', onScroll, true)
    window.addEventListener('resize', onScroll)
    return () => {
      document.removeEventListener('pointerdown', onPointer)
      document.removeEventListener('keydown', onKey)
      document.removeEventListener('scroll', onScroll, true)
      window.removeEventListener('resize', onScroll)
    }
  }, [open])
  return (
    <div ref={host} className={cn('relative', className)}>
      <button
        type="button"
        aria-label={menuLabel}
        aria-haspopup="menu"
        aria-expanded={open}
        aria-controls={id}
        title={menuLabel}
        onClick={() => setOpen((current) => !current)}
        className={cn('grid h-5 w-5 place-items-center rounded text-faint transition-opacity hover:text-ink focus-visible:opacity-100', open ? 'text-ink' : 'opacity-0 group-hover/th:opacity-100 group-focus-within/th:opacity-100')}
      >
        <Icon name="more_vert" size={14} />
      </button>
      {open && anchor !== null && createPortal(
        <ul ref={menu} id={id} role="menu" aria-label={menuLabel} style={{ top: anchor.top, right: anchor.right }} className="float fixed z-(--z-popover) m-0 min-w-44 list-none rounded-lg border border-edge bg-panel p-1 text-left">
          {items.map((item) => (
            <li key={item.id} role="none">
              <button
                type="button"
                role="menuitem"
                disabled={item.disabled}
                onClick={() => { setOpen(false); item.onSelect() }}
                className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-body normal-case tracking-normal text-ink hover:bg-well disabled:cursor-not-allowed disabled:text-dim"
              >
                {item.icon && <Icon name={item.icon} size={14} className="text-muted" />}
                {item.text}
              </button>
            </li>
          ))}
        </ul>,
        document.body,
      )}
    </div>
  )
}

/** "n of m rows" with an optional trailing clause. */
export function countLine(shown: number, total: number, noun: string, extra?: string): string {
  const all = shown === total
  const base = all ? `${formatCount(total).text} ${noun}${total === 1 ? '' : 's'}` : `${formatCount(shown).text} of ${formatCount(total).text} ${noun}s`
  return extra ? `${base} · ${extra}` : base
}

/** Mono type text for a header line. */
export const typeText = (duckdbType: string): ReactNode => <span className={literal('text-micro text-faint')}>{duckdbType}</span>

/** One proportion as a bar on a track, the same ink as the schema table's micro-figures; decoration beside a count that already says it. */
export function ShareBar({ share, height = 4, className }: { readonly share: number; readonly height?: number; readonly className?: string }) {
  const width = Math.max(0, Math.min(1, share)) * 100
  return (
    <svg aria-hidden viewBox="0 0 100 1" preserveAspectRatio="none" width="100%" height={height} className={cn('block rounded-full', className)}>
      <rect x={0} y={0} width={100} height={1} fill="var(--color-hair)" />
      <rect x={0} y={0} width={width} height={1} fill="var(--color-bone)" />
    </svg>
  )
}
