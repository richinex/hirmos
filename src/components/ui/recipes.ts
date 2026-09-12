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

/** `signal` is the one affirmative action on a surface, so a panel never has two (index.css rule 2), and
 * it is the one pill-shaped button: the shape alone says "this is the thing to do next". Every other tone
 * keeps the control radius, including `danger`, which is a destructive action and not the next step.
 * `soft` is the signal wash for a control that is live but not the primary action. `outline` is the
 * ordinary action, `quiet` the dismissive one, and `mono` the small-caps drafting-label register used by
 * the eval and compile chrome. */
export type ButtonTone = 'signal' | 'soft' | 'danger' | 'outline' | 'quiet' | 'mono'

/** The signal wash: a tint and a translucent border, so a live control reads as live without a second fill on the surface. */
const SIGNAL_WASH = 'border-signal/40 bg-signal/10 text-signal hover:bg-signal/15'

/** A filled tone carries the inset top highlight the house uses for elevation, and drops it on press: the
 * button reads as pushed in, with no drop shadow. The press itself is the 1px settle in index.css. */
const FILLED = 'font-medium text-signal-ink shadow-[inset_0_1px_0_var(--color-highlight)] hover:brightness-110 active:shadow-none'

/** Busy is `aria-busy="true"`, set from the run's own state and never from the pointer: the label stays
 * (so the width does) and a bar-live sweep runs along the inside bottom edge (index.css). `disabled`
 * remains "not ready"; a busy button keeps focus so nothing jumps when the run ends. */
const BUTTON_BASE = 'inline-flex items-center justify-center gap-1.5 rounded-full border border-transparent transition-[color,background-color,border-color,box-shadow,transform] duration-(--motion-fast) aria-busy:pointer-events-none disabled:cursor-not-allowed disabled:border-hair disabled:bg-transparent disabled:text-faint disabled:shadow-none disabled:hover:brightness-100 pointer-coarse:min-h-10'

const BUTTON_TONE: Record<ButtonTone, string> = {
  signal: cn('bg-signal px-4', FILLED),
  soft: SIGNAL_WASH,
  danger: cn('bg-danger', FILLED),
  outline: 'border border-hair text-ink hover:border-edge',
  quiet: 'border border-hair text-muted hover:text-ink',
  // Named for its register, not its typeface: the label voice, at button size.
  mono: 'border border-hair text-label font-medium text-muted hover:border-edge hover:text-ink',
}

/** `md` is the 30px field height that matches `field()` and `SegmentedControl`; `sm` the 24px chrome step. */
export type ButtonSize = 'sm' | 'md'

const BUTTON_SIZE: Record<ButtonSize, string> = {
  sm: 'px-2.5 py-1 text-label',
  md: 'px-3 py-1.5 text-body',
}

export const button = (tone: ButtonTone = 'outline', extra?: string, size: ButtonSize = 'md'): string =>
  cn(BUTTON_BASE, BUTTON_SIZE[size], BUTTON_TONE[tone], extra)

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
  signal: SIGNAL_WASH,
  danger: 'border-transparent text-faint hover:bg-danger/[0.08] hover:text-danger',
}

export const iconControl = (tone: ChromeTone = 'quiet', extra?: string): string =>
  cn(
    'grid h-8 w-8 shrink-0 place-items-center rounded-full border pointer-coarse:h-10 pointer-coarse:w-10 transition-colors duration-(--motion-fast) disabled:cursor-not-allowed disabled:text-dim disabled:hover:bg-transparent',
    CHROME_TONE[tone],
    extra,
  )

/** A word-sized sibling of iconControl. It shares height, radius and state treatment so a toolbar reads
 * as one control family even when some actions need labels and others only need a familiar glyph. */
export const chromeAction = (tone: ChromeTone = 'quiet', extra?: string): string =>
  cn(
    'flex min-h-8 items-center gap-1.5 rounded-full border px-2.5 text-label pointer-coarse:min-h-10 transition-colors duration-(--motion-fast) disabled:cursor-not-allowed disabled:text-dim disabled:hover:bg-transparent',
    CHROME_TONE[tone],
    extra,
  )

/** A filter facet. Until something is chosen its border is dashed, which says "nothing set here" without a
 * badge or a colour; chosen is the raised surface with a solid edge, the same selection the rest of the
 * chrome uses. Counts and words inside it stay in the label register. The control radius, like the
 * segmented control: a choice among a few options has one shape, and the pill belongs to the primary
 * action alone. */
export const facet = (active: boolean, extra?: string): string =>
  cn(
    'min-h-8 rounded-md border px-2.5 pointer-coarse:min-h-10 transition-colors',
    label(),
    active ? 'border-solid border-edge bg-raised text-ink' : 'border-dashed border-hair text-faint hover:border-edge hover:text-muted',
    extra,
  )

/** A view switch. Selection is a surface, never a colour, and the shape is the control radius, shared with
 * the facet and the segmented control. */
export const pill = (active: boolean, extra?: string): string =>
  cn(
    'min-h-8 rounded-md border px-2.5 pointer-coarse:min-h-10 transition-colors',
    label(),
    active ? 'border-edge bg-raised text-ink' : 'border-hair text-faint hover:text-muted',
    extra,
  )

/** A text field. `mono` for ids, models and Cypher — anything the user must read character by character.
 *  The edge border and dimmed placeholder are what separate "a box you can type in" from the text around
 *  it: on the dark theme the well fill sits four luminance points above the page, so the border carries
 *  the affordance at 3:1, and the placeholder is italic so it never reads as an entered value. */
export const field = (variant: 'text' | 'mono' = 'text', extra?: string): string =>
  cn(
    'w-full rounded-full border border-control bg-well px-3 py-1.5 text-body text-ink placeholder:italic placeholder:text-faint focus:border-signal/60 disabled:cursor-not-allowed disabled:text-faint',
    variant === 'mono' && 'font-mono',
    extra,
  )

/** The label above a field. Ink, not muted: the label is the anchor of the group, and muted sat one
 *  step from the faint hint below it, which made label, hint and placeholder read as one grey. */
export const fieldLabel = 'block text-body font-medium text-ink'

/** The help line below a field. Body size and the token's 1.5 line height, because the old
 *  11px/leading-snug pairing put multi-sentence help below the WCAG line-height floor. Serif, because a
 *  hint is a sentence, and named here because a hint is as often a span as a paragraph. */
export const fieldHint = 'mt-1 font-serif text-body text-faint text-pretty'

/**
 * Which typeface, and why. The model is a journal page (index.css rule 5).
 *
 * Serif for what is read as prose: a paragraph, a caption, a narrative. Sans for what is scanned or
 * compared: headings, labels, buttons, table cells and every number, in the face the paper prescribes for
 * figures, Helvetica or Arial. Mono is a reading aid, not a house style: it buys a fixed pitch and
 * unambiguous shapes, so 1/l and 0/O stay apart in an id you may have to retype, and it costs legibility
 * in running text. So mono only for what is read character by character: code, ids, locators, hashes.
 */

/** Running text: the paragraph under a chapter title, the explanation beside a method, the reading of a
 *  result. The serif at the 13px prose tier on a 65-character measure, which is the paper's column
 *  brought to the screen. Colour is the call site's: muted for a narrative, faint for an aside. A `p`
 *  element already takes the serif from index.css; the face is named here so a span reads the same. */
export const prose = (extra?: string): string => cn('max-w-[65ch] font-serif text-subtitle text-pretty', extra)

/** The paragraph under a chapter title, set the way the page it is modelled on sets its body: two columns
 *  that fill the stage. A column is at least 45 characters wide; the browser fits two when the width
 *  holds two and shares the remaining width between them, one otherwise, and never three, which turns
 *  a short paragraph into a row of fragments (Clarke, "Revisiting CSS Multi-Column Layout", 2025). On
 *  a stage wider than two full measures the type grows with the container instead, so the columns keep
 *  filling the width without the measure passing 75 characters; the line height is a ratio so it grows
 *  with it. Below the tablet step the paragraph keeps the single-column measure. The columns balance,
 *  words are never hyphenated, and a column keeps at least two lines of a sentence on each side of the
 *  break. Only the chapter narrative takes this: a help line beside a field is one thought and stays one
 *  column. */
export const chapterIntro = prose('m-0 text-muted @3xl/panel:max-w-[calc(150ch+2rem)] @3xl/panel:text-[length:clamp(0.8125rem,0.4rem+0.6cqi,1.0625rem)] leading-[1.55] [columns:45ch_2] gap-x-8 hyphens-none [orphans:2] [widows:2]')

/** Figures the reader compares down a column. `tabular-nums` fixes digit advance width, so a counter
 *  ticking 9 to 10 does not nudge what follows; kerning is off because Helvetica and Arial pull the pair
 *  "11" together and that would undo it. The sans is named, not inherited, because a figure line is
 *  often a paragraph element and a paragraph otherwise takes the serif: a figure is sans without exception. */
export const num = (extra?: string): string => cn('font-sans tabular-nums [font-kerning:none]', extra)

/** A literal the reader may have to type or match character by character: a node id, an API key, a hash.
 *  This is the only mono outside actual code. Filenames, model names and timestamps read as words. */
export const literal = (extra?: string): string => cn('font-mono', extra)

/** A chrome label: the name of a slot, above or beside the value that fills it. The 11px label tier
 * remains readable in dense panels; the 10px micro tier is reserved for timings and keycaps. The size,
 * the weight and the muted colour do the "this is a label" work, so the words are set as words. */
export const label = (extra?: string): string => cn('font-sans text-label font-medium', extra)

/** What a figure plots, under or above it.
 *
 * A caption is a phrase, not a slot name, so it is sentence case: `label()` stays for the short fixed
 * nouns that name a field. Same tier and colour, so the two still read as one register. */
export const caption = (extra?: string): string => cn('font-serif text-label text-faint text-pretty', extra)

/** A variable name as a member of a set the reader counts: enclosure marks membership, so it is for sets only, never a name inside a sentence. */
export const chip = (extra?: string): string => cn('inline-block rounded-md border border-hair bg-panel px-1.5 py-0.5 text-ink', extra)

/**
 * A section of the workbench: the outermost surface a reader sees inside a chapter.
 *
 * Its padding is one variable, `--panel-space`, that a call site spends as `p-(--panel-space)`: 16px on
 * a panel, 12px on a well, and one step less each when the chapter column is narrower than a tablet.
 * Density is then a property of the surface, retuned in one place, not a padding chosen per call site.
 *
 * One recipe so the app has one panel, and so a surface can be dropped in one place rather than in
 * fifteen. A panel holds content, never another panel: two of these nested draw the same border
 * twice around the same thing, and the inner one stops meaning anything. Where a component would
 * land its own surface inside this one, reach for its `frame` escape hatch instead.
 */
export const panel = (extra?: string): string => cn('rounded-xl border border-hair bg-panel [--panel-space:--spacing(4)] @max-md/panel:[--panel-space:--spacing(3)]', extra)

/**
 * A recessed area inside a panel: a control group, a figure, a quoted reading.
 *
 * One step in from `panel()`, and the innermost surface that should carry a border. A well inside a
 * well reads as a mistake, and the give-away is a call site passing `bg-panel` back to cancel it.
 */
export const well = (extra?: string): string => cn('rounded-lg border border-hair bg-well [--panel-space:--spacing(3)] @max-md/panel:[--panel-space:--spacing(2)]', extra)

/** A hairline-joined grid of figure cells: the 1px gaps draw the rules, so the cells carry no borders of their own. */
export const figureGrid = (extra?: string): string => cn('grid gap-px overflow-hidden rounded-lg border border-hair bg-hair', extra)

/** Text colour for a machine-state verdict, from the status ramp; muted for a state that is neither good nor bad. */
export const statusText: Record<'ok' | 'warn' | 'danger' | 'muted', string> = { ok: 'text-ok', warn: 'text-warn', danger: 'text-danger', muted: 'text-muted' }

/** A section title inside a chapter: "Examples", "Discovery methods", "Diagnostics". The paper's section
 *  head, bold sans in the muted grey with a square marker before it, in sentence case. The marker is
 *  what makes it a section head rather than a name, so a heading that names a thing (a column, a run, a
 *  document) does not take it. The chapter title above stays in ink and takes no marker. */
export const sectionTitle = "text-title font-medium text-muted text-balance before:mr-2 before:inline-block before:align-[0.15em] before:text-[0.6em] before:content-['■']"

/** The title of a floating surface: panel, drawer or sheet.
 *
 * A heading, so it is set in sentence case on the title tier, the ramp step named for a surface's
 * subject. Capitals are for a label naming a slot, not for a heading naming a surface. */
export const panelTitle = 'text-title font-medium text-ink text-balance'

/** Row padding per density: 24px compact and 32px comfortable rows with 12px body text, including the 1px hairline under the row. */
export const rowPadding = { compact: 'pt-[3px] pb-[2px]', comfortable: 'pt-[7px] pb-[6px]' } as const

/** A data table: the caps tier for headers, hairline rows, comfortable 32px body rows unless a density is applied; figure cells add `text-right`. */
export const table = 'w-full border-collapse text-left text-body'

/** A header cell. Sticky, on the well, set in sentence case and carried by weight rather than capitals,
 *  because a header may be a phrase ("Zivot-Andrews p, constant and trend"). `p-0` when a sort button fills it. */
export const th = (extra?: string): string =>
  cn('sticky top-0 z-(--z-sticky) whitespace-nowrap border-b border-hair bg-well px-3.5 py-2 text-left text-label font-medium text-muted', extra)

/** A body row. `action` rows fill on hover and focus; `selected` is a surface, never a colour or a weight.
 *  A row whose menu or expander is open stays filled, so the reader keeps which row they are acting on. */
export const tr = (state: 'static' | 'action' | 'selected' = 'static', extra?: string): string =>
  cn(
    'border-b border-line transition-colors has-[[aria-expanded=true]]:bg-well',
    state !== 'static' && 'cursor-pointer hover:bg-well has-[button:focus-visible]:bg-well',
    state === 'selected' && 'bg-raised',
    extra,
  )

/** A body cell. Text cells truncate with the full value in `title`; figure cells add `text-right`. */
export const td = (extra?: string): string => cn('max-w-[300px] truncate px-3.5 align-top', rowPadding.comfortable, extra)

/** A text cell that takes the lines it needs: a rationale, a conditioning set, an expression. `td` clips
 *  at one line so a row keeps the density's pitch; this cell wraps, and a windowed table predicts the
 *  height it will take before the row exists (lib/textMetrics.ts), so the window stays exact without
 *  measuring rows as they scroll. Words break only where the browser would, which is what the
 *  prediction models. */
export const tdText = (extra?: string): string => cn('whitespace-normal px-3.5 align-top [overflow-wrap:break-word]', rowPadding.comfortable, extra)

/** The row-count line under a table; `aria-live="polite"` so a sort or filter is announced.
 *  It reports a count, so it is sentence case: capitals are for a label naming a slot. */
export const tableFoot = 'border-t border-hair px-3.5 py-1.5 font-sans text-label text-faint'
