import type { DagNodeId } from '@/domain/dag'

export interface ScreenPoint {
  readonly x: number
  readonly y: number
}

export interface ScreenTargetBox {
  readonly node: DagNodeId
  readonly left: number
  readonly top: number
  readonly right: number
  readonly bottom: number
  readonly eligible: boolean
}

export type DagPointerTarget =
  | { readonly kind: 'none' }
  | {
      readonly kind: 'eligible-node'
      readonly node: DagNodeId
      readonly anchor: ScreenPoint
      readonly distance: number
    }
  | {
      readonly kind: 'refused-node'
      readonly node: DagNodeId
      readonly anchor: ScreenPoint
      readonly distance: number
    }

const distanceToBox = (pointer: ScreenPoint, box: ScreenTargetBox): number => {
  const dx = Math.max(box.left - pointer.x, 0, pointer.x - box.right)
  const dy = Math.max(box.top - pointer.y, 0, pointer.y - box.bottom)
  return Math.hypot(dx, dy)
}

/**
 * Resolve a causal connector in screen coordinates.
 *
 * This follows Recursis's pointer-target seam: tolerance is a screen distance,
 * so zoom never makes a target harder to acquire. The nearest card wins and
 * preview and commit both consume this same tagged result.
 */
export function dagPointerTarget(
  pointer: ScreenPoint,
  boxes: readonly ScreenTargetBox[],
  tolerance: number,
): DagPointerTarget {
  let nearest: { readonly box: ScreenTargetBox; readonly distance: number } | null = null
  for (const box of boxes) {
    const distance = distanceToBox(pointer, box)
    if (distance > tolerance) continue
    if (nearest === null || distance < nearest.distance) nearest = { box, distance }
  }
  if (nearest === null) return { kind: 'none' }
  const anchor = {
    x: (nearest.box.left + nearest.box.right) / 2,
    y: (nearest.box.top + nearest.box.bottom) / 2,
  }
  return nearest.box.eligible
    ? { kind: 'eligible-node', node: nearest.box.node, anchor, distance: nearest.distance }
    : { kind: 'refused-node', node: nearest.box.node, anchor, distance: nearest.distance }
}
