// The query engine's states, typed so illegal ones are unrepresentable (see /DOP.md). The loop runs
// think -> write Cypher -> run it -> observe rows, until a cited answer or a failure.
import { assertNever, type Brand, type NonEmptyArray } from '../dop'

// ---- domain values (branded; only smart constructors produce them) ----
export type NodeId = Brand<string, 'NodeId'>
export type Cypher = Brand<string, 'Cypher'>
export type PyCode = Brand<string, 'PyCode'>   // a CodeAct turn's Python block
export type Answer = Brand<string, 'Answer'>

// Smart constructors, the ONE door to each brand (DOP rule 5): no scattered `x as NodeId`, and one place
// to add validation later.
export const asNodeId = (s: string): NodeId => s.trim() as NodeId
export const asPyCode = (s: string): PyCode => s as PyCode
export const asAnswer = (s: string): Answer => s.trim() as Answer

export interface Citation { readonly nodeId: NodeId; readonly label: string }
/** A done answer must cite at least one node — "answered without evidence" is unrepresentable. */
export type Citations = NonEmptyArray<Citation>

// ---- Cypher execution result: Ok XOR Err, never both, never neither ----
export type CypherRow = ReadonlyArray<unknown>
export type CypherResult =
  | { readonly kind: 'ok'; readonly columns: ReadonlyArray<string>; readonly rows: ReadonlyArray<CypherRow> }
  | { readonly kind: 'err'; readonly message: string }

// ---- CodeAct execution result: what running one Python block produced (observed by the model next turn) ----
export interface CodeResult {
  readonly stdout: string
  readonly stderr: string
  readonly exitCode: number
  readonly answerReady: boolean   // did the block set answer['ready']?
  /** name -> short repr of the namespace after the block (feeds the session task record). */
  readonly locals?: Readonly<Record<string, string>>
}

// ---- the live loop state: each variant holds exactly the data valid in that state ----
export type RlmState =
  | { readonly kind: 'idle' }
  | { readonly kind: 'thinking' }
  | { readonly kind: 'wroteCypher'; readonly code: Cypher }            // has code, not yet run
  | { readonly kind: 'executing'; readonly code: Cypher }              // running that code
  | { readonly kind: 'observed'; readonly code: Cypher; readonly result: CypherResult }
  | { readonly kind: 'writingCode'; readonly text: string }            // CodeAct: reply streaming in (live UI)
  | { readonly kind: 'wroteCode'; readonly code: PyCode }              // CodeAct: has a Python block, not yet run
  | { readonly kind: 'ranCode'; readonly code: PyCode; readonly result: CodeResult }   // CodeAct: ran it, observed output
  | { readonly kind: 'mcpCall'; readonly tool: string; readonly input: Record<string, unknown> }   // MCP: about to call a tool
  | { readonly kind: 'mcpResult'; readonly tool: string; readonly ok: boolean; readonly summary: string }   // MCP: tool returned
  | { readonly kind: 'done'; readonly answer: Answer; readonly citations: Citations }
  // Host-verified computations may have no graph-node citation (for example an MCP tool result). Hosted
  // CodeAct does not use this variant: its grounding boundary emits strict `done` with Citations.
  | { readonly kind: 'doneComputed'; readonly answer: Answer; readonly citations: ReadonlyArray<Citation> }
  | { readonly kind: 'notFound'; readonly reason: string }   // the graph honestly doesn't record it
  | { readonly kind: 'error'; readonly message: string }

// ---- events that drive transitions (their own closed union) ----
export type RlmEvent =
  | { readonly type: 'think' }
  | { readonly type: 'emitCypher'; readonly code: Cypher }
  | { readonly type: 'exec' }
  | { readonly type: 'observe'; readonly result: CypherResult }
  | { readonly type: 'finish'; readonly answer: Answer; readonly citations: Citations }
  | { readonly type: 'fail'; readonly message: string }

// ---- a finished iteration: reasoned, ran a Cypher query, or ran a Python block (each carries its data) ----
export type TranscriptEntry =
  | { readonly kind: 'reasoned'; readonly thought: string }
  | { readonly kind: 'ranQuery'; readonly code: Cypher; readonly result: CypherResult }
  | { readonly kind: 'ranCode'; readonly code: PyCode; readonly result: CodeResult }

// ---- the session as a union: mid-run (with a live state) XOR terminated (with an outcome) ----
export type Outcome =
  | { readonly kind: 'answered'; readonly answer: Answer; readonly citations: Citations }
  | { readonly kind: 'computed'; readonly answer: Answer; readonly citations: ReadonlyArray<Citation> }
  | { readonly kind: 'notFound'; readonly reason: string }
  | { readonly kind: 'failed'; readonly message: string }

export type Session =
  | { readonly kind: 'running'; readonly question: string; readonly history: ReadonlyArray<TranscriptEntry>; readonly live: RlmState }
  // `outputTokens` is everything the loop generated, summed across its turns. A CodeAct agent makes
  // several calls per task, and a fan-out of agents is the one extraction path whose cost nothing else can
  // report, because the caller never makes the model calls itself. See EXTRACT_BATCHED's two branches.
  | { readonly kind: 'terminated'; readonly question: string; readonly history: ReadonlyArray<TranscriptEntry>; readonly outcome: Outcome; readonly outputTokens?: number }

// ---- the one transition function; exhaustive over events (assertNever = compile-time completeness) ----
export function step(state: RlmState, event: RlmEvent): RlmState {
  switch (event.type) {
    case 'think': return { kind: 'thinking' }
    case 'emitCypher': return { kind: 'wroteCypher', code: event.code }
    case 'exec': return state.kind === 'wroteCypher' ? { kind: 'executing', code: state.code } : state
    case 'observe': return state.kind === 'executing' ? { kind: 'observed', code: state.code, result: event.result } : state
    case 'finish': return { kind: 'done', answer: event.answer, citations: event.citations }
    case 'fail': return { kind: 'error', message: event.message }
    default: return assertNever(event)
  }
}
