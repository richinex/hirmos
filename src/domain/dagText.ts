import { err, ok, type Result } from './dop'

/**
 * Reads the graph description language dagitty and graphviz share, to the grammar in dagitty's own
 * `GraphDotParser.pegjs`. Hirmos holds a DAG, so the directed and bidirected arrows are read and
 * the partially directed and undirected ones are refused by name rather than dropped in silence.
 */

export type DagTextProblem =
  | { readonly kind: 'empty' }
  | { readonly kind: 'unterminated-string'; readonly at: number }
  | { readonly kind: 'unexpected-character'; readonly at: number; readonly found: string }
  | {
      readonly kind: 'unexpected-token'
      readonly at: number
      readonly found: string
      readonly expected: string
    }
  | { readonly kind: 'reserved-name'; readonly name: string }
  | { readonly kind: 'unsupported-edge'; readonly operator: string }
  | { readonly kind: 'invalid-lag'; readonly value: string }
  | { readonly kind: 'lag-on-common-cause' }

export interface DagTextNode {
  readonly name: string
  readonly kind: 'observed' | 'latent'
  readonly x: number | null
  readonly y: number | null
}

export interface DagTextEdge {
  readonly from: string
  readonly to: string
  /** Steps back in time from the effect to the cause; null is the same period. */
  readonly lag: number | null
}

export interface ParsedDagText {
  readonly nodes: readonly DagTextNode[]
  readonly edges: readonly DagTextEdge[]
  /** Every variable marked as a source, in the order declared; a study needs exactly one. */
  readonly exposures: readonly string[]
  readonly outcomes: readonly string[]
  readonly adjusted: readonly string[]
  /** Bidirected arrows, each already rewritten as an unmeasured common cause. */
  readonly bidirected: number
}

/** Longest first, so `<->` is never read as `<-`. */
const OPERATORS = ['@->', '<-@', '@--', '--@', '<->', '@-@', '->', '<-', '--'] as const
type Operator = (typeof OPERATORS)[number]

const DIRECTED: ReadonlySet<string> = new Set(['->', '<-', '<->'])

const GRAPH_TYPES: ReadonlySet<string> = new Set(['graph', 'digraph', 'dag', 'mag', 'pdag', 'pag'])

/** dagitty refuses these as variable names because the language already uses them. */
const RESERVED: ReadonlySet<string> = new Set(['graph', 'node'])

type Token =
  | { readonly kind: 'id'; readonly text: string; readonly at: number; readonly quoted: boolean }
  | { readonly kind: 'operator'; readonly text: Operator; readonly at: number }
  | {
      readonly kind: 'punctuation'
      readonly text: '{' | '}' | '[' | ']' | '=' | ','
      readonly at: number
    }
  | { readonly kind: 'end'; readonly at: number }

/** `[\n\r\t ;]*` in the grammar: a semicolon separates nothing, it is space. */
const SKIPPED = new Set([' ', '\t', '\n', '\r', ';'])
const BAREWORD = /[0-9a-zA-Z_.]/

const tokenize = (source: string): Result<readonly Token[], DagTextProblem> => {
  const tokens: Token[] = []
  let at = 0
  while (at < source.length) {
    const character = source[at]!
    if (SKIPPED.has(character)) {
      at += 1
      continue
    }
    // A comma separates attributes and nothing else: the grammar's whitespace is `[\n\r\t ;]*`.
    if (character === ',') {
      tokens.push({ kind: 'punctuation', text: ',', at })
      at += 1
      continue
    }
    if (
      character === '{' ||
      character === '}' ||
      character === '[' ||
      character === ']' ||
      character === '='
    ) {
      tokens.push({ kind: 'punctuation', text: character, at })
      at += 1
      continue
    }
    if (character === '"') {
      let text = ''
      let index = at + 1
      for (;;) {
        if (index >= source.length) return err({ kind: 'unterminated-string', at })
        const inner = source[index]!
        if (inner === '"') {
          index += 1
          break
        }
        if (inner === '\\') {
          const escaped = source[index + 1]
          if (escaped === '"') {
            text += '"'
            index += 2
            continue
          }
          if (escaped === '\n' || escaped === '\r') {
            index += 2
            continue
          }
          text += '\\'
          index += 1
          continue
        }
        text += inner
        index += 1
      }
      tokens.push({ kind: 'id', text, at, quoted: true })
      at = index
      continue
    }
    const operator = OPERATORS.find((candidate) => source.startsWith(candidate, at))
    if (operator !== undefined) {
      tokens.push({ kind: 'operator', text: operator, at })
      at += operator.length
      continue
    }
    if (BAREWORD.test(character) || character === '-') {
      const start = at
      if (character === '-') at += 1
      const from = at
      while (at < source.length && BAREWORD.test(source[at]!)) at += 1
      if (at === from) return err({ kind: 'unexpected-character', at: start, found: character })
      tokens.push({ kind: 'id', text: source.slice(start, at), at: start, quoted: false })
      continue
    }
    return err({ kind: 'unexpected-character', at, found: character })
  }
  tokens.push({ kind: 'end', at: source.length })
  return ok(tokens)
}

type Attribute = readonly [string, string]

interface Builder {
  readonly order: string[]
  readonly latent: Set<string>
  readonly adjusted: Set<string>
  readonly position: Map<string, { x: number; y: number }>
  readonly edges: DagTextEdge[]
  readonly directedSeen: Set<string>
  readonly bidirectedSeen: Set<string>
  readonly exposures: Set<string>
  readonly outcomes: Set<string>
  bidirected: number
}

const POSITION = /\s*(-?[0-9.]+)\s*,\s*(-?[0-9.]+)\s*/

const LATENT = new Set(['latent', 'l', 'unobserved', 'u'])
const EXPOSURE = new Set(['source', 'exposure', 'e'])
const OUTCOME = new Set(['target', 'outcome', 'o'])
const ADJUSTED = new Set(['adjusted', 'a'])

class Reader {
  private index = 0
  constructor(private readonly tokens: readonly Token[]) {}

  peek(): Token {
    return this.tokens[this.index]!
  }
  take(): Token {
    const token = this.tokens[this.index]!
    this.index += 1
    return token
  }
  at(kind: Token['kind'], text?: string): boolean {
    const token = this.peek()
    return token.kind === kind && (text === undefined || ('text' in token && token.text === text))
  }
}

const parse = (reader: Reader, builder: Builder): Result<readonly string[], DagTextProblem> => {
  const declared: string[] = []

  const note = (name: string): Result<string, DagTextProblem> => {
    if (RESERVED.has(name)) return err({ kind: 'reserved-name', name })
    if (!builder.order.includes(name)) builder.order.push(name)
    if (!declared.includes(name)) declared.push(name)
    return ok(name)
  }

  const attributes = (): Result<readonly Attribute[], DagTextProblem> => {
    const found: Attribute[] = []
    reader.take()
    for (;;) {
      if (reader.at('punctuation', ',')) {
        reader.take()
        continue
      }
      if (reader.at('punctuation', ']')) {
        reader.take()
        return ok(found)
      }
      const key = reader.take()
      if (key.kind !== 'id') {
        return err({
          kind: 'unexpected-token',
          at: key.at,
          found: describeToken(key),
          expected: 'an attribute name',
        })
      }
      if (reader.at('punctuation', '=')) {
        reader.take()
        const value = reader.take()
        if (value.kind !== 'id') {
          return err({
            kind: 'unexpected-token',
            at: value.at,
            found: describeToken(value),
            expected: 'an attribute value',
          })
        }
        found.push([key.text, value.text])
        continue
      }
      found.push([key.text, '1'])
    }
  }

  /** A `{ … }` group stands for every node inside it, on either side of an arrow. */
  const group = (): Result<readonly string[], DagTextProblem> => {
    reader.take()
    const inner = parse(reader, builder)
    if (!inner.ok) return inner
    if (!reader.at('punctuation', '}')) {
      const token = reader.peek()
      return err({
        kind: 'unexpected-token',
        at: token.at,
        found: describeToken(token),
        expected: '}',
      })
    }
    reader.take()
    for (const name of inner.value) if (!declared.includes(name)) declared.push(name)
    return inner
  }

  const operand = (): Result<readonly string[], DagTextProblem> => {
    if (reader.at('punctuation', '{')) return group()
    const token = reader.take()
    if (token.kind !== 'id') {
      return err({
        kind: 'unexpected-token',
        at: token.at,
        found: describeToken(token),
        expected: 'a variable name',
      })
    }
    const named = note(token.text)
    return named.ok ? ok([named.value]) : named
  }

  const apply = (
    operator: Operator,
    left: readonly string[],
    right: readonly string[],
    lag: number | null,
  ): DagTextProblem | null => {
    if (!DIRECTED.has(operator)) return { kind: 'unsupported-edge', operator }
    if (operator === '<->' && lag !== null) return { kind: 'lag-on-common-cause' }
    for (const from of left) {
      for (const to of right) {
        if (operator === '<->') {
          // A bidirected arrow is one unmeasured common cause however it is written, so A <-> B and
          // B <-> A are the same arrow, while A -> B and B -> A are two.
          const pair = [from, to].sort().join('\u0000')
          if (builder.bidirectedSeen.has(pair)) continue
          builder.bidirectedSeen.add(pair)
          const common = `U_${from}${to}`
          builder.bidirected += 1
          if (!builder.order.includes(common)) builder.order.push(common)
          builder.latent.add(common)
          builder.edges.push(
            { from: common, to: from, lag: null },
            { from: common, to: to, lag: null },
          )
          continue
        }
        const [tail, head] = operator === '->' ? [from, to] : [to, from]
        const key = `${tail}\u0000${head}\u0000${lag ?? ''}`
        if (builder.directedSeen.has(key)) continue
        builder.directedSeen.add(key)
        builder.edges.push({ from: tail, to: head, lag })
      }
    }
    return null
  }

  /** `a -> b -> c` adds each arrow in turn, every element standing for one node or a whole group. */
  const chain = (head: readonly string[]): DagTextProblem | null => {
    const links: {
      readonly operator: Operator
      readonly left: readonly string[]
      readonly right: readonly string[]
    }[] = []
    let left = head
    for (;;) {
      const token = reader.peek()
      if (token.kind !== 'operator') break
      reader.take()
      const right = operand()
      if (!right.ok) return right.error
      links.push({ operator: token.text, left, right: right.value })
      left = right.value
    }
    let lag: number | null = null
    if (reader.at('punctuation', '[')) {
      const listed = attributes()
      if (!listed.ok) return listed.error
      for (const [key, value] of listed.value) {
        if (key.toLowerCase() !== 'lag') continue
        const steps = Number(value)
        if (!/^\d+$/.test(value) || !Number.isSafeInteger(steps) || steps < 1)
          return { kind: 'invalid-lag', value }
        lag = steps
      }
    }
    for (const link of links) {
      const refusal = apply(link.operator, link.left, link.right, lag)
      if (refusal !== null) return refusal
    }
    return null
  }

  for (;;) {
    if (reader.at('end') || reader.at('punctuation', '}')) return ok(declared)

    // A group may open a statement on its own, or stand as an arrow's left side.
    if (reader.at('punctuation', '{')) {
      const nested = group()
      if (!nested.ok) return nested
      if (!reader.at('operator')) continue
      const chained = chain(nested.value)
      if (chained !== null) return err(chained)
      continue
    }

    const head = reader.take()
    if (head.kind !== 'id') {
      return err({
        kind: 'unexpected-token',
        at: head.at,
        found: describeToken(head),
        expected: 'a variable name',
      })
    }

    // `name = value` at statement level is a graph option, such as the bounding box.
    if (reader.at('punctuation', '=')) {
      reader.take()
      const value = reader.take()
      if (value.kind !== 'id') {
        return err({
          kind: 'unexpected-token',
          at: value.at,
          found: describeToken(value),
          expected: 'an option value',
        })
      }
      continue
    }

    if (reader.at('operator')) {
      const named = note(head.text)
      if (!named.ok) return named
      const chained = chain([named.value])
      if (chained !== null) return err(chained)
      continue
    }

    const listed = reader.at('punctuation', '[') ? attributes() : ok([] as readonly Attribute[])
    if (!listed.ok) return listed

    // `graph [bb="…"]` carries the drawing's bounding box, not a variable.
    if (head.text === 'graph' || head.text === 'node') continue

    const named = note(head.text)
    if (!named.ok) return named
    for (const [key, value] of listed.value) {
      const lowered = key.toLowerCase()
      if (LATENT.has(lowered)) builder.latent.add(named.value)
      else if (EXPOSURE.has(lowered)) builder.exposures.add(named.value)
      else if (OUTCOME.has(lowered)) builder.outcomes.add(named.value)
      else if (ADJUSTED.has(lowered)) builder.adjusted.add(named.value)
      else if (lowered === 'pos') {
        const matched = POSITION.exec(value)
        if (matched !== null)
          builder.position.set(named.value, { x: Number(matched[1]), y: Number(matched[2]) })
      }
    }
  }
}

const describeToken = (token: Token): string => {
  switch (token.kind) {
    case 'end':
      return 'the end of the text'
    case 'id':
      return token.quoted ? `"${token.text}"` : token.text
    default:
      return token.text
  }
}

/** Strips the optional `strict dag name { … }` wrapper, the way dagitty's own reader does. */
const unwrap = (tokens: readonly Token[]): readonly Token[] => {
  let index = 0
  const first = tokens[index]
  if (first?.kind === 'id' && !first.quoted && first.text.toLowerCase() === 'strict') index += 1
  const type = tokens[index]
  if (type?.kind !== 'id' || type.quoted || !GRAPH_TYPES.has(type.text.toLowerCase())) return tokens
  index += 1
  const next = tokens[index]
  if (next?.kind === 'id') index += 1
  if (tokens[index]?.kind !== 'punctuation' || (tokens[index] as { text: string }).text !== '{')
    return tokens
  const last = tokens[tokens.length - 2]
  if (last?.kind !== 'punctuation' || last.text !== '}') return tokens
  return [...tokens.slice(index + 1, tokens.length - 2), tokens[tokens.length - 1]!]
}

export function parseDagText(source: string): Result<ParsedDagText, DagTextProblem> {
  if (source.trim().length === 0) return err({ kind: 'empty' })
  const tokens = tokenize(source)
  if (!tokens.ok) return tokens
  const builder: Builder = {
    order: [],
    latent: new Set(),
    adjusted: new Set(),
    position: new Map(),
    edges: [],
    directedSeen: new Set(),
    bidirectedSeen: new Set(),
    exposures: new Set(),
    outcomes: new Set(),
    bidirected: 0,
  }
  const parsed = parse(new Reader(unwrap(tokens.value)), builder)
  if (!parsed.ok) return parsed
  const nodes = builder.order.map((name) => {
    const position = builder.position.get(name)
    return {
      name,
      kind: builder.latent.has(name) ? ('latent' as const) : ('observed' as const),
      x: position?.x ?? null,
      y: position?.y ?? null,
    }
  })
  return ok({
    nodes,
    edges: builder.edges,
    exposures: [...builder.exposures],
    outcomes: [...builder.outcomes],
    adjusted: [...builder.adjusted],
    bidirected: builder.bidirected,
  })
}

export const describeDagTextProblem = (problem: DagTextProblem): string => {
  switch (problem.kind) {
    case 'empty':
      return 'Paste a graph description to read.'
    case 'unterminated-string':
      return `A quoted name starting at character ${problem.at + 1} is never closed.`
    case 'unexpected-character':
      return `Character ${problem.at + 1} is ${problem.found}, which the graph language does not use.`
    case 'unexpected-token':
      return `Character ${problem.at + 1} is ${problem.found} where ${problem.expected} was expected.`
    case 'reserved-name':
      return `${problem.name} names part of the language, so it cannot name a variable. Give the variable another name and use a label.`
    case 'unsupported-edge':
      return `${problem.operator} is not a directed or bidirected arrow, so it does not describe a DAG. Use ->, <- or <->.`
    case 'invalid-lag':
      return `lag=${problem.value} is not a whole number of steps of 1 or more.`
    case 'lag-on-common-cause':
      return 'A lag applies to a directed arrow; an unmeasured common cause has no lag.'
  }
}
