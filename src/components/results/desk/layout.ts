export interface Rect {
  readonly left: number
  readonly top: number
  readonly width: number
  readonly height: number
}

export interface Size {
  readonly width: number
  readonly height: number
}

export type TiledView = 'compare' | 'evidence'
export type DeskView = 'desk' | TiledView | 'focus'

export type Split =
  | { readonly kind: 'leaf'; readonly key: string }
  | {
      readonly kind: 'split'
      readonly id: string
      readonly dir: 'row' | 'col'
      readonly ratio: number
      readonly a: Split
      readonly b: Split
    }

export interface Seam {
  readonly id: string
  readonly dir: 'row' | 'col'
  readonly outer: Rect
  readonly rect: Rect
}

export const DOCK_HEIGHT = 52
export const MARGIN = 12
export const GAP = 12
export const MIN_WIDTH = 280
export const MIN_HEIGHT = 180
export const TILE = 32
export const TILE_GAP = 10

export const workspaceOf = (desk: Size): Rect => ({
  left: 0,
  top: 0,
  width: desk.width,
  height: Math.max(0, desk.height - DOCK_HEIGHT),
})

export function fit(rect: Rect, space: Rect): Rect {
  const width = Math.min(rect.width, space.width)
  const height = Math.min(rect.height, space.height)
  return {
    left: Math.min(Math.max(rect.left, space.left), space.left + space.width - width),
    top: Math.min(Math.max(rect.top, space.top), space.top + space.height - height),
    width,
    height,
  }
}

/** A rectangle placed on a workspace of size `from`, at the same proportions on one of size `to`. */
export const scaled = (rect: Rect, from: Size, to: Size): Rect => {
  const x = to.width / from.width
  const y = to.height / from.height
  return { left: rect.left * x, top: rect.top * y, width: rect.width * x, height: rect.height * y }
}

export const inset = (space: Rect, by: number): Rect => ({
  left: space.left + by,
  top: space.top + by,
  width: Math.max(0, space.width - 2 * by),
  height: Math.max(0, space.height - 2 * by),
})

/** Tiles for a split tree. On a narrow desk every split stacks. */
export function tile(
  tree: Split,
  outer: Rect,
  compact: boolean,
  ratioOf: (id: string, ratio: number) => number = (_, ratio) => ratio,
) {
  const placed = new Map<string, Rect>()
  const seams: Seam[] = []
  const walk = (node: Split, rect: Rect) => {
    if (node.kind === 'leaf') {
      placed.set(node.key, rect)
      return
    }
    const dir = compact ? 'col' : node.dir
    const span = dir === 'row' ? rect.width : rect.height
    const least = Math.min(dir === 'row' ? MIN_WIDTH : MIN_HEIGHT - 40, (span - GAP) / 2)
    const first = Math.min(
      Math.max(span * ratioOf(node.id, node.ratio) - GAP / 2, least),
      span - GAP - least,
    )
    if (dir === 'row') {
      walk(node.a, { ...rect, width: first })
      walk(node.b, { ...rect, left: rect.left + first + GAP, width: rect.width - first - GAP })
      seams.push({
        id: node.id,
        dir,
        outer: rect,
        rect: { ...rect, left: rect.left + first, width: GAP },
      })
    } else {
      walk(node.a, { ...rect, height: first })
      walk(node.b, { ...rect, top: rect.top + first + GAP, height: rect.height - first - GAP })
      seams.push({
        id: node.id,
        dir,
        outer: rect,
        rect: { ...rect, top: rect.top + first, height: GAP },
      })
    }
  }
  walk(tree, outer)
  return { placed, seams }
}

/** The tree with its leaves filled in the stored order, when that order names the same windows. */
export function inOrder(tree: Split, order: readonly string[] | undefined): Split {
  if (order === undefined) return tree
  const leaves: string[] = []
  const collect = (node: Split) => {
    if (node.kind === 'leaf') leaves.push(node.key)
    else {
      collect(node.a)
      collect(node.b)
    }
  }
  collect(tree)
  if (order.length !== leaves.length || leaves.some((key) => !order.includes(key))) return tree
  let next = 0
  const fill = (node: Split): Split =>
    node.kind === 'leaf'
      ? { kind: 'leaf', key: order[next++]! }
      : { ...node, a: fill(node.a), b: fill(node.b) }
  return fill(tree)
}

/** A column of windows that each take a share of the height, as one split tree. */
export function stack(keys: readonly string[], id: string): Split | null {
  if (keys.length === 0) return null
  if (keys.length === 1) return { kind: 'leaf', key: keys[0]! }
  const rest = stack(keys.slice(1), `${id}.${keys.length - 1}`)!
  return {
    kind: 'split',
    id,
    dir: 'col',
    ratio: 1 / keys.length,
    a: { kind: 'leaf', key: keys[0]! },
    b: rest,
  }
}

/**
 * The open windows side by side with no overlap, in the order given: 3, 2 or 1 columns by width, and more
 * columns while the rows would be shorter than a window can be.
 */
export function grid(keys: readonly string[], space: Rect): Map<string, Rect> {
  const placed = new Map<string, Rect>()
  if (keys.length === 0) return placed
  const outer = inset(space, MARGIN)
  const rowHeight = (columns: number) => {
    const rows = Math.ceil(keys.length / columns)
    return (outer.height - (rows - 1) * GAP) / rows
  }
  const widest = Math.max(1, Math.floor((outer.width + GAP) / (MIN_WIDTH + GAP)))
  let columns = Math.min(keys.length, outer.width >= 1000 ? 3 : outer.width >= 620 ? 2 : 1)
  while (columns < Math.min(keys.length, widest) && rowHeight(columns) < MIN_HEIGHT) columns++
  const rows = Math.ceil(keys.length / columns)
  const height = (outer.height - (rows - 1) * GAP) / rows
  keys.forEach((key, index) => {
    const row = Math.floor(index / columns)
    const inRow = Math.min(columns, keys.length - row * columns)
    const width = (outer.width - (inRow - 1) * GAP) / inRow
    const column = index % columns
    placed.set(key, {
      left: outer.left + column * (width + GAP),
      top: outer.top + row * (height + GAP),
      width,
      height,
    })
  })
  return placed
}

/**
 * The first rectangles on a free desk: a group at sensible sizes, centred, where a later window
 * overlaps the lower part of an earlier one; a cascade on a narrow desk.
 */
export function tidy(keys: readonly string[], space: Rect): Map<string, Rect> {
  const placed = new Map<string, Rect>()
  if (keys.length === 0) return placed
  const outer = inset(space, MARGIN)
  if (outer.width < 620) {
    const front = Math.max(MIN_HEIGHT, Math.min(420, outer.height * 0.5))
    const slice = keys.length > 1 ? Math.max(44, (outer.height - front) / (keys.length - 1)) : 0
    keys.forEach((key, index) => {
      const top = outer.top + index * slice
      placed.set(key, {
        left: outer.left,
        top,
        width: outer.width,
        height: outer.top + outer.height - top,
      })
    })
    return placed
  }
  const width = Math.min(560, Math.max(MIN_WIDTH, outer.width * 0.42))
  const height = Math.min(460, Math.max(MIN_HEIGHT, outer.height * 0.62))
  const stepX =
    keys.length > 1 ? Math.min(width * 0.62, (outer.width - width) / (keys.length - 1)) : 0
  const stepY = keys.length > 1 ? Math.min(72, (outer.height - height) / (keys.length - 1)) : 0
  const groupWidth = width + stepX * (keys.length - 1)
  const groupHeight = height + stepY * (keys.length - 1)
  const left = outer.left + (outer.width - groupWidth) / 2
  const top = outer.top + (outer.height - groupHeight) / 2
  keys.forEach((key, index) => {
    placed.set(key, { left: left + index * stepX, top: top + index * stepY, width, height })
  })
  return placed
}

export function dockTile(index: number, count: number, desk: Size): Rect {
  const size = desk.width < 400 ? 30 : TILE
  const gap = desk.width < 400 ? 8 : TILE_GAP
  const row = count * size + (count - 1) * gap
  return {
    left: desk.width / 2 - row / 2 + index * (size + gap),
    top: desk.height - 14 - size,
    width: size,
    height: size,
  }
}

export const overlaps = (a: Rect, b: Rect): boolean =>
  a.left < b.left + b.width - 1 &&
  b.left < a.left + a.width - 1 &&
  a.top < b.top + b.height - 1 &&
  b.top < a.top + a.height - 1
