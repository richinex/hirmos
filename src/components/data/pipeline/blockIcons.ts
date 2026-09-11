import type { PipelineBlockKind } from '@/domain/pipeline'

/** The Material Symbol for each block kind, shared by the palette and the cards so a kind looks the same in both. */
export const blockIcon = (kind: PipelineBlockKind): string => {
  switch (kind) {
    case 'input': return 'csv'
    case 'filter-rows': return 'filter_alt'
    case 'select-columns': return 'view_column'
    case 'derive-columns': return 'function'
    case 'join': return 'join_inner'
    case 'union': return 'stacks'
    case 'aggregate': return 'functions'
    case 'sort-limit': return 'swap_vert'
    case 'script': return 'code'
    case 'output': return 'output'
    default: { const exhaustive: never = kind; return exhaustive }
  }
}

export type PaletteKind = Exclude<PipelineBlockKind, 'output'>

/** The palette in groups: files, then what a block does to rows, to columns, to several tables, and code. */
export const PALETTE_GROUPS: readonly { readonly label: string; readonly kinds: readonly PaletteKind[] }[] = [
  { label: 'Files', kinds: ['input'] },
  { label: 'Rows', kinds: ['filter-rows', 'sort-limit'] },
  { label: 'Columns', kinds: ['select-columns', 'derive-columns'] },
  { label: 'Tables', kinds: ['join', 'union', 'aggregate'] },
  { label: 'Code', kinds: ['script'] },
]

export const BLOCK_DRAG_TYPE = 'application/x-hirmos-block'
