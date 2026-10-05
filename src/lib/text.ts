/** Names that keep their capital mid-sentence; the method catalogue's labels start with several of them. */
const PROPER_NAMES = [
  'Bayesian',
  'Poisson',
  'Gaussian',
  'Granger',
  'Johansen',
  'Newey',
  'Bartlett',
  'Ljung',
  'Durbin',
  'Shapiro',
  'Causal impact',
]

/** Lowercases a leading capital only when a lowercase letter follows and the word is not a name, so "Refuters" becomes "refuters" while "DML" and "Bayesian" keep their capital after "Run". */
export const lowerFirst = (text: string): string =>
  PROPER_NAMES.some((name) => text.startsWith(name))
    ? text
    : text.replace(/^[A-Z](?=[a-z])/, (initial) => initial.toLowerCase())
