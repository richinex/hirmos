import {
  isNumericDuckDbType,
  type PhysicalColumnProfile,
  type PreviewCell,
  type PreviewRow,
} from '@/domain/dataset'
import { formatAbsent, formatCount } from '@/lib/format/number'
import { num } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { columnDecimals, isNumericCell, previewCellText } from './previewCell'

export interface PreviewBodyView {
  readonly total: number
  readonly rowHeight: number
  readonly padding: string
  readonly columns: readonly PhysicalColumnProfile[]
  readonly cellIndex: readonly number[]
  readonly selected: PhysicalColumnProfile['id'] | null
  readonly rowAt: (index: number) => PreviewRow | null
  readonly onRange: (first: number, last: number) => void
  readonly describe: (first: number, last: number) => string
}

interface Slot {
  readonly node: HTMLTableRowElement
  readonly header: HTMLTableCellElement
  readonly cells: HTMLTableCellElement[]
  row: number
  data: PreviewRow | null | undefined
  readonly texts: string[]
  readonly classes: string[]
}

const OVERSCAN = 6
const absent = formatAbsent('unavailable', 'missing value')

const missingValue = (document: Document): HTMLSpanElement => {
  const outer = document.createElement('span')
  outer.className = 'text-muted'
  outer.title = 'Missing value'
  const spoken = outer.appendChild(document.createElement('span'))
  spoken.className = 'sr-only'
  spoken.textContent = absent.srText
  const shown = outer.appendChild(document.createElement('span'))
  shown.setAttribute('aria-hidden', 'true')
  shown.textContent = absent.text
  return outer
}

const notLoaded = (document: Document): HTMLSpanElement => {
  const span = document.createElement('span')
  span.className = 'text-faint'
  span.textContent = '…'
  return span
}

export function createPreviewBody(
  body: HTMLTableSectionElement,
  scroller: HTMLElement,
  count: HTMLElement,
  read: () => PreviewBodyView,
) {
  const document = body.ownerDocument
  const spacer = () => {
    const row = document.createElement('tr')
    row.setAttribute('role', 'presentation')
    row.setAttribute('aria-hidden', 'true')
    const cell = row.appendChild(document.createElement('td'))
    cell.setAttribute('role', 'presentation')
    cell.style.padding = '0'
    cell.style.border = '0'
    return { row, cell }
  }
  const top = spacer()
  const bottom = spacer()
  body.replaceChildren(top.row, bottom.row)
  let slots: Slot[] = []
  let start = 0
  let shape = ''
  let decimals: number[] = []
  let frame = 0

  const createSlot = (view: PreviewBodyView): Slot => {
    const node = document.createElement('tr')
    node.className = 'border-b border-line'
    const header = node.appendChild(document.createElement('th'))
    header.scope = 'row'
    const cells = view.columns.map(() => node.appendChild(document.createElement('td')))
    bottom.row.before(node)
    return {
      node,
      header,
      cells,
      row: -1,
      data: undefined,
      texts: view.columns.map(() => ''),
      classes: view.columns.map(() => ''),
    }
  }

  const writeSlot = (slot: Slot, row: number, view: PreviewBodyView, force: boolean) => {
    const data = view.rowAt(row)
    if (!force && slot.row === row && slot.data === data) return
    if (slot.row !== row) slot.node.setAttribute('aria-rowindex', String(row + 2))
    slot.row = row
    slot.data = data
    slot.header.textContent = data === null ? '' : formatCount(data.index).text
    view.columns.forEach((column, position) => {
      const cell = data?.cells[view.cellIndex[position]!]
      const right =
        cell === undefined || cell.kind === 'null'
          ? isNumericDuckDbType(column.duckdbType)
          : isNumericCell(cell)
      const className = cn(
        'truncate whitespace-nowrap px-3.5',
        view.padding,
        right ? num('text-right') : 'text-left',
        cell?.kind === 'null' ? 'text-muted' : 'text-ink',
        column.id === view.selected && 'bg-raised',
      )
      const element = slot.cells[position]!
      if (slot.classes[position] !== className) {
        element.className = className
        slot.classes[position] = className
      }
      const text =
        cell === undefined
          ? '\u0000loading'
          : cell.kind === 'null'
            ? '\u0000missing'
            : previewCellText(cell, decimals[position] ?? 0)
      if (slot.texts[position] === text) return
      slot.texts[position] = text
      if (cell === undefined) element.replaceChildren(notLoaded(document))
      else if (cell.kind === 'null') element.replaceChildren(missingValue(document))
      else element.textContent = text
      if (cell !== undefined && cell.kind !== 'null') element.title = String(cell.value)
      else element.removeAttribute('title')
    })
  }

  const draw = () => {
    frame = 0
    const view = read()
    const nextShape = `${view.columns.map((column) => column.id).join(',')}|${view.rowHeight}|${view.padding}|${view.selected}`
    if (nextShape !== shape) {
      for (const slot of slots) slot.node.remove()
      slots = []
      shape = nextShape
    }
    const headerClass = num(
      cn(
        'sticky left-0 z-10 border-r border-hair bg-panel px-3.5 text-right font-normal text-faint',
        view.padding,
      ),
    )
    const height = view.rowHeight
    top.cell.colSpan = view.columns.length + 1
    bottom.cell.colSpan = view.columns.length + 1

    const offset = scroller.scrollTop - body.offsetTop
    const firstVisible = Math.max(0, Math.min(view.total - 1, Math.floor(offset / height)))
    const lastVisible = Math.max(
      0,
      Math.min(view.total - 1, Math.floor((offset + scroller.clientHeight - 1) / height)),
    )
    const wanted = Math.ceil(scroller.clientHeight / height) + 2 * OVERSCAN
    while (slots.length < wanted) {
      const slot = createSlot(view)
      slot.header.className = headerClass
      slot.node.style.height = `${height}px`
      slots.push(slot)
    }
    const next = Math.max(0, Math.min(firstVisible - OVERSCAN, view.total - slots.length))

    const delta = next - start
    if (delta > 0 && delta < slots.length) {
      const moved = slots.slice(0, delta)
      bottom.row.before(...moved.map((slot) => slot.node))
      slots = [...slots.slice(delta), ...moved]
    } else if (delta < 0 && -delta < slots.length) {
      const moved = slots.slice(slots.length + delta)
      top.row.after(...moved.map((slot) => slot.node))
      slots = [...moved, ...slots.slice(0, slots.length + delta)]
    }
    start = next

    const shown = Math.max(0, Math.min(slots.length, view.total - start))
    const onScreen: (readonly PreviewCell[] | null)[] = []
    for (let i = 0; i < shown; i++) onScreen.push(view.rowAt(start + i)?.cells ?? null)
    const nextDecimals = view.cellIndex.map((index) => columnDecimals(onScreen, index))
    const decimalsChanged = nextDecimals.some((value, position) => value !== decimals[position])
    decimals = nextDecimals

    top.row.style.display = start > 0 ? '' : 'none'
    top.cell.style.height = `${start * height}px`
    const below = view.total - start - shown
    bottom.row.style.display = below > 0 ? '' : 'none'
    bottom.cell.style.height = `${below * height}px`
    slots.forEach((slot, index) => {
      const present = index < shown
      slot.node.style.display = present ? '' : 'none'
      if (present) writeSlot(slot, start + index, view, decimalsChanged)
    })

    const line = view.total === 0 ? view.describe(-1, -1) : view.describe(firstVisible, lastVisible)
    if (count.textContent !== line) count.textContent = line
    if (shown > 0) view.onRange(start, start + shown - 1)
  }

  const schedule = () => {
    if (frame === 0) frame = requestAnimationFrame(draw)
  }
  scroller.addEventListener('scroll', schedule, { passive: true })
  const resize = new ResizeObserver(schedule)
  resize.observe(scroller)

  return {
    draw,
    dispose() {
      cancelAnimationFrame(frame)
      scroller.removeEventListener('scroll', schedule)
      resize.disconnect()
      body.replaceChildren()
    },
  }
}
