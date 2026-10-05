import { fontFor, fontPx, lineCountAt, lineHeightFor, textWidth } from '@/lib/textMetrics'
import { DEFAULT_CARD_SIZE, type DagCardSize } from './dagCanvasModel'

/** Horizontal room a card gives up to chrome: `px-3` either side, which is 0.75rem, the body tier's own
 *  size, so it scales with the root the way the text does; a one-pixel border each side; and a pixel
 *  of slack for the rounding between a measured advance and a painted box. */
const cardInset = (bodyPx: number): number => 2 * bodyPx + 2 + 1
/** The widest a card grows before a name wraps: twice the resting width. */
const CARD_MAX_WIDTH = 2 * DEFAULT_CARD_SIZE.width
/** The most lines a name takes; past this the name clips, as it did at one line before. */
const MAX_NAME_LINES = 3

/**
 * The one card size a graph's nodes share, fitted to the longest name: every card keeps the resting
 * 164 by 58 until a name needs more, then all of them widen to hold it, up to twice the width, and only
 * past that does the longest name wrap to a second or third line and every card take that height.
 * Uniform on purpose: a DAG is drawn with equal nodes, and a canvas of cards in one size stays calm.
 * Measured with pretext in the card's own face and weight, so the size dagre lays out is the size that
 * paints, with no read of the DOM and no relayout after the first paint.
 */
export function dagCardSize(names: readonly string[]): DagCardSize {
  if (names.length === 0) return DEFAULT_CARD_SIZE
  const font = fontFor('body', 600)
  const inset = cardInset(fontPx(font))
  const widest = Math.max(0, ...names.map((name) => textWidth(name, font)))
  const width = Math.min(
    CARD_MAX_WIDTH,
    Math.max(DEFAULT_CARD_SIZE.width, Math.ceil(widest) + inset),
  )
  const content = width - inset
  const nameLines = Math.min(
    MAX_NAME_LINES,
    Math.max(1, ...names.map((name) => lineCountAt(name, font, content))),
  )
  const height =
    DEFAULT_CARD_SIZE.height + (nameLines - 1) * Math.round(lineHeightFor('body', font))
  return { width, height, nameLines }
}
