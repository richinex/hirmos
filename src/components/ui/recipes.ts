import { cn } from '@/lib/utils'

/**
 * The repeated control recipes, in one place.
 *
 * These are class strings, not components, so a call site keeps its own element, handlers and extra
 * classes. What they buy is one definition: the signal button had been written five ways, the hairline
 * button five, and the settings input eleven times with two paddings and two sizes.
 *
 * This is not a design system. Radius, elevation and spacing stay in index.css. Focus is handled there
 * too, so nothing here sets an outline; the `focus:border-*` tints complement that ring.
 */

/** `signal` is the one affirmative action on a surface, so a panel never has two (index.css rule 2).
 * `outline` is the ordinary action, `quiet` the dismissive one, and `mono` the small-caps drafting-label
 * register used by the eval and compile chrome. */
export type ButtonTone = 'signal' | 'danger' | 'outline' | 'quiet' | 'mono'

const BUTTON_BASE = 'rounded-md px-3 py-1.5 text-body transition-[color,background-color,border-color,transform] duration-150 active:scale-[0.99] disabled:cursor-not-allowed disabled:border-hair disabled:bg-transparent disabled:text-faint disabled:hover:brightness-100 pointer-coarse:min-h-10'

const BUTTON_TONE: Record<ButtonTone, string> = {
  signal: 'bg-signal font-medium text-signal-ink hover:brightness-110',
  danger: 'bg-danger font-medium text-signal-ink hover:brightness-110',
  outline: 'border border-hair text-ink hover:border-edge',
  quiet: 'border border-hair text-muted hover:text-ink',
  // Named for its register, not its typeface. It carried font-mono until the typography pass, which only
  // made the words wider.
  mono: 'border border-hair text-label uppercase tracking-[0.1em] text-muted hover:border-edge hover:text-ink',
}

export const button = (tone: ButtonTone = 'outline', extra?: string): string =>
  cn(BUTTON_BASE, BUTTON_TONE[tone], extra)

/** Segmented-control segment. Selected is a surface, never a signal fill. */
export const segment = (active: boolean, extra?: string): string =>
  cn(
    'rounded-md border px-3 py-1.5 text-body transition-colors',
    active ? 'border-edge bg-raised text-ink' : 'border-transparent text-muted hover:text-ink',
    extra,
  )

/** Compact actions inside headers, answer footers and toolbars. These deliberately keep a quiet visual
 * footprint while giving the glyph a 32px target, promoted to 40px on coarse pointers by construction.
 * A transparent border is always present so selecting an action never changes its geometry. */
export type ChromeTone = 'quiet' | 'selected' | 'signal' | 'danger'

const CHROME_TONE: Record<ChromeTone, string> = {
  quiet: 'border-transparent text-faint hover:bg-well hover:text-ink',
  selected: 'border-edge bg-raised text-ink',
  signal: 'border-signal/40 bg-signal/10 text-signal hover:bg-signal/15',
  danger: 'border-transparent text-faint hover:bg-danger/[0.08] hover:text-danger',
}

export const iconControl = (tone: ChromeTone = 'quiet', extra?: string): string =>
  cn(
    'grid h-8 w-8 shrink-0 place-items-center rounded-lg border pointer-coarse:h-10 pointer-coarse:w-10 transition-colors duration-150 disabled:cursor-not-allowed disabled:text-dim disabled:hover:bg-transparent',
    CHROME_TONE[tone],
    extra,
  )

/** A word-sized sibling of iconControl. It shares height, radius and state treatment so a toolbar reads
 * as one control family even when some actions need labels and others only need a familiar glyph. */
export const chromeAction = (tone: ChromeTone = 'quiet', extra?: string): string =>
  cn(
    'flex min-h-8 items-center gap-1.5 rounded-lg border px-2 text-label pointer-coarse:min-h-10 transition-colors duration-150 disabled:cursor-not-allowed disabled:text-dim disabled:hover:bg-transparent',
    CHROME_TONE[tone],
    extra,
  )

/** A pill-shaped view switch. Selection is a surface, never a colour. */
export const pill = (active: boolean, extra?: string): string =>
  cn(
    'min-h-8 rounded-full border px-2.5 pointer-coarse:min-h-10 transition-colors',
    label(),
    active ? 'border-edge bg-raised text-ink' : 'border-hair text-faint hover:text-muted',
    extra,
  )

/** A text field. `mono` for ids, models and Cypher — anything the user must read character by character.
 *  The edge border and dimmed placeholder are what separate "a box you can type in" from the text around
 *  it: on the dark theme the well fill sits four luminance points above the page, so the border carries
 *  the affordance, and a placeholder at full faint reads as an entered value. */
export const field = (variant: 'text' | 'mono' = 'text', extra?: string): string =>
  cn(
    'w-full rounded-md border border-edge bg-well px-2 py-1.5 text-body text-ink placeholder:text-faint/60 focus:border-signal/60 disabled:cursor-not-allowed disabled:text-faint',
    variant === 'mono' && 'font-mono',
    extra,
  )

/** The label above a field. Ink, not muted: the label is the anchor of the group, and muted sat one
 *  step from the faint hint below it, which made label, hint and placeholder read as one grey. */
export const fieldLabel = 'block text-body font-medium text-ink'

/** The help line below a field. Body size and the token's 1.5 line height, because the old
 *  11px/leading-snug pairing put multi-sentence help below the WCAG line-height floor. */
export const fieldHint = 'mt-1 text-body text-faint'

/**
 * Which typeface, and why.
 *
 * Mono is a reading aid, not a house style. It buys a fixed pitch, so digits line up down a column, and
 * unambiguous shapes, so 1/l and 0/O stay apart in an id you may have to retype. It costs legibility in
 * running text, where every glyph takes the same width whether it needs it or not.
 *
 * So mono for what is read character by character: numbers, code, ids, locators, hashes. Sans for what is
 * read as words: labels, headings, buttons, status, narration, help text.
 */

/** Figures the reader compares down a column. `tabular-nums` fixes digit advance width, so a counter
 *  ticking 9 to 10 does not nudge what follows. Sans, because Geist's tabular figures align without the
 *  typeface change. */
export const num = (extra?: string): string => cn('tabular-nums', extra)

/** A literal the reader may have to type or match character by character: a node id, an API key, a hash.
 *  This is the only mono outside actual code. Filenames, model names and timestamps read as words. */
export const literal = (extra?: string): string => cn('font-mono', extra)

/** A small-caps chrome label. The 11px label tier remains readable in dense panels; the 10px micro tier
 * is reserved for timings and keycaps. Ships the app's one label tracking (0.1em); call sites had
 * invented twelve values for that one job. */
export const label = (extra?: string): string => cn('text-label uppercase tracking-[0.1em]', extra)

/** The title of a floating surface: panel, drawer or sheet. A step above `label()` in size, same voice.
 *  The app once had four registers for this, from 9.5px to 13px across three trackings. */
export const panelTitle = 'text-label uppercase tracking-[0.1em] text-muted'
