import { assertNever } from '../dop'
import { extractFirstCodeBlock } from './parseCode'
import { asAnswer, asPyCode, type CodeResult, type RlmState, type Session, type TranscriptEntry } from './types'
import type { CodeActExecutor, CodeActLoopOptions, CodeActMessage, CodeExecResult } from './codeactPorts'

// What the model "observes" after running a block: its output, any error, and the variables now in scope.
function formatFeedback(exec: CodeExecResult, finish: string): string {
  const parts: string[] = []
  const out = (exec.stdout ?? '').slice(0, 3000)
  const err = (exec.stderr ?? '').slice(0, 2000)
  if (out.trim()) parts.push(`Output:\n${out}`)
  if (err.trim()) parts.push(`Error:\n${err}`)
  const vars = Object.keys(exec.locals ?? {})
  if (vars.length) parts.push(`Variables in scope: ${vars.join(', ')}`)
  if (!parts.length) parts.push('(the block produced no output)')
  parts.push(finish)
  return parts.join('\n\n')
}

const fail = (question: string, history: TranscriptEntry[], message: string, emit: (s: RlmState) => void): Session => {
  emit({ kind: 'error', message })
  return { kind: 'terminated', question, history, outcome: { kind: 'failed', message } }
}

/** Run one question to a final answer (or failure) via the CodeAct loop, emitting live state for the UI. */
export async function runCodeActLoop(question: string, opts: CodeActLoopOptions): Promise<Session> {
  const emit = (s: RlmState) => opts.onState?.(s)
  const history: TranscriptEntry[] = []
  const system = opts.system
  const messages: CodeActMessage[] = [{ role: 'user', content: question }]
  const maxIters = opts.maxIters ?? 12
  const turnBudget = opts.maxTokens ?? 4000
  // Every output token this loop generates, summed across its turns. A truncation retry counts too: it is
  // a full generation that was paid for.
  let outputTokens = 0

  emit({ kind: 'thinking' })
  let repl: CodeActExecutor
  try {
    // Warm reuse, namespace reset and boot-failure teardown all live behind acquireExecutor (the host).
    repl = await opts.acquireExecutor()
  } catch (e) {
    return fail(question, history, `Pyodide failed to start: ${(e as Error).message}`, emit)
  }

  try {
    for (let i = 0; i < maxIters; i++) {
      opts.signal?.throwIfAborted()
      let r
      let truncated = false
      try {
        // Headroom for reasoning models (GPT-5, deepseek-reasoner): hidden reasoning eats output tokens
        // before the Python block, so too small a budget returns empty and wastes an iteration.
        // Generation stops at the first closed ```python fence, since only that block ever runs.
        let lastEmit = 0
        const call = (maxTokens: number) => opts.callModel({
          system, messages, maxTokens, signal: opts.signal,
          onText: (full) => {
            const now = Date.now()
            if (now - lastEmit > 150) { lastEmit = now; emit({ kind: 'writingCode', text: full }) }
          },
        })
        r = await call(turnBudget)
        outputTokens += r.usage?.output_tokens ?? 0
        // A completion cut off at the output cap is NOT a program: its last string, dict or fence is still
        // open. Running it reports "SyntaxError: unterminated string literal", so the model rewrites the
        // same over-long block and truncates in the same place. Climb the budget once, as extract.ts does,
        // then say what happened instead of executing a fragment.
        if (r.stop_reason === 'max_tokens') {
          r = await call(Math.min(turnBudget * 4, 64_000))
          outputTokens += r.usage?.output_tokens ?? 0
          truncated = r.stop_reason === 'max_tokens'
        }
      } catch (e) {
        return fail(question, history, (e as Error).message, emit)
      }
      const text = r.text.trim()
      // Some providers reject an empty assistant turn, which a reasoning model can produce by spending its
      // whole budget thinking.
      messages.push({ role: 'assistant', content: r.content.length ? r.content : [{ type: 'text', text: '(no output)' }] })

      // Still cut off at four times the budget, so the block is a fragment and is not executed. Truncation
      // and a syntax error need opposite recoveries, emit less versus fix and resend, so the model is told
      // which one it hit.
      if (truncated) {
        messages.push({ role: 'user', content:
          'Your block was CUT OFF at the output token limit before it finished, so it was NOT run. The code '
          + 'was not wrong, it was too long. Do not resend it whole. Emit it across several smaller blocks: '
          + 'start a list in one block (nodes = []), append a few items per block (nodes += [...]), and call '
          + 'FINAL_VAR only in the last one.' })
        emit({ kind: 'thinking' })
        continue
      }

      const { code, discarded } = extractFirstCodeBlock(text)
      if (!code) {
        // No block — nudge (the model may have answered in prose without calling FINAL_VAR).
        messages.push({ role: 'user', content: 'No python code block found. Write ONE ```python block. ' + opts.guidance.finish })
        emit({ kind: 'thinking' })
        continue
      }

      const pycode = asPyCode(code)
      emit({ kind: 'wroteCode', code: pycode })

      let exec: CodeExecResult
      try {
        exec = await repl.execute(code)
      } catch (e) {
        return fail(question, history, (e as Error).message, emit)
      }

      opts.signal?.throwIfAborted()
      const result: CodeResult = {
        stdout: exec.stdout, stderr: exec.stderr, exitCode: exec.exit_code, answerReady: !!exec.answer?.ready,
        locals: Object.fromEntries(Object.entries(exec.locals ?? {}).map(([k, v]) => [k, typeof v === 'string' ? v.slice(0, 200) : JSON.stringify(v)?.slice(0, 200) ?? ''])),
      }
      emit({ kind: 'ranCode', code: pycode, result })
      history.push({ kind: 'ranCode', code: pycode, result })

      const hasErrors = exec.exit_code !== 0
      // An error always forces another turn, even if the block also set answer (recourse invariant).
      if (exec.answer?.ready && !hasErrors) {
        const raw = exec.answer.content
        const answerStr = typeof raw === 'string' ? raw : JSON.stringify(raw)

        if (answerStr.startsWith('NOT_RECORDED')) {
          const reason = answerStr.replace(/^NOT_RECORDED:?\s*/, '').trim() || 'the graph does not record this'
          emit({ kind: 'notFound', reason })
          return { kind: 'terminated', question, history, outcome: { kind: 'notFound', reason }, outputTokens }
        }

        // GROUNDING GATE: a model that lost its variables mid-run invents plausible ids ("risk_1",
        // "measurement_1"), observed in the wild, and unvalidated they make the answer look grounded.
        // Missing, partially invalid and wholly invalid claims are all distinct from a grounded answer.
        const grounding = await opts.validateCitations(exec.answer.citations)
        switch (grounding.kind) {
          case 'uncited':
            repl.resetAnswer()
            emit({ kind: 'mcpResult', tool: 'FINAL_VAR', ok: false, summary: 'rejected: no fact citations supplied' })
            messages.push({ role: 'user', content:
              opts.guidance.uncited })
            emit({ kind: 'thinking' })
            continue
          case 'rejected':
            repl.resetAnswer()
            emit({ kind: 'mcpResult', tool: 'FINAL_VAR', ok: false,
              summary: `rejected unusable citations: ${grounding.invalid.slice(0, 4).map((citation) => citation.nodeId).join(', ')}` })
            messages.push({ role: 'user', content:
              opts.guidance.rejected })
            emit({ kind: 'thinking' })
            continue
          case 'validationFailed':
            return fail(question, history, `could not validate answer citations against the graph: ${grounding.message}`, emit)
          case 'grounded': {
            const answer = asAnswer(answerStr)
            // The strict terminal variant requires Citations (NonEmptyArray). CodeAct cannot construct a
            // settled answer from an uncited FINAL_VAR any more; only this grounded branch has the type.
            emit({ kind: 'done', answer, citations: grounding.citations })
            return { kind: 'terminated', question, history, outcome: { kind: 'answered', answer, citations: grounding.citations }, outputTokens }
          }
          default:
            return assertNever(grounding)
        }
      }

      if (exec.answer?.ready && hasErrors) repl.resetAnswer()   // stale answer from an errored block — drop it

      const discardNote = discarded > 0
        ? `\n\nWARNING: you wrote ${discarded} additional code block(s) that were NOT executed. Only the FIRST block runs per turn.`
        : ''
      messages.push({ role: 'user', content: formatFeedback(exec, opts.guidance.finish) + discardNote })
      emit({ kind: 'thinking' })
    }
    return fail(question, history, 'reached max iterations without a final answer', emit)
  } finally {
    // keep the executor warm for the next run — do NOT terminate (the host owns its lifecycle)
  }
}
