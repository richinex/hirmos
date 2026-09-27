import { formatCount } from './number'

/** A list this long or shorter is named in full; a longer one is counted, so a figure or a sentence stays readable. */
export const NAMED_IN_FULL = 5

const SHOWN_BEFORE_COUNT = 3

/** Names as a figure: in full when short, otherwise a count, with the first few names as a preview beneath it. */
export const namesFigure = (names: readonly string[], noun: string): { readonly value: string; readonly preview: string | null } =>
  names.length <= NAMED_IN_FULL
    ? { value: names.join(', '), preview: null }
    : {
      value: `${formatCount(names.length).text} ${noun}`,
      preview: `${names.slice(0, SHOWN_BEFORE_COUNT).join(', ')} and ${formatCount(names.length - SHOWN_BEFORE_COUNT).text} more`,
    }

/** Names inside a sentence: in full when short, otherwise the phrase `counted` makes from their count. */
export const namesInProse = (names: readonly string[], counted: (count: string) => string): string =>
  names.length <= NAMED_IN_FULL ? names.join(', ') : counted(formatCount(names.length).text)
