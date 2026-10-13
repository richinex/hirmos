// Port of recursis' extract_first_code_block (rlm_agent/parsing.py): pull the FIRST executable Python
// block out of the model's text, leniently. Each rule was earned by a real provider failure:
//   - the opener may be ```python, ```py or a bare ```, but a fence naming another language (```json,
//     ```bash) is SKIPPED whole, never executed as Python;
//   - an unclosed fence is tolerated, and everything after the opener is the code;
//   - a ``` inside an unterminated triple-quoted string does NOT close the block;
//   - blocks after the first are counted so the loop can warn the model;
//   - with no fence at all, an unfenced emit_graph(...) or FINAL_VAR(...) written in prose is found by
//     balanced-paren scan, saving the nudge round trip.
export interface ExtractedCode {
  /** The first code block's body, or null if the turn contained no executable block. */
  readonly code: string | null
  /** The model's full text (kept for transcript/UI). */
  readonly text: string
  /** Fenced blocks after the first — not executed; the loop should TELL the model. */
  readonly discarded: number
}

const PY_LANGS = new Set(['', 'python', 'py'])

/** The language tag on a fence line, lowercased ('' for a bare fence), or null if not a fence line. */
function fenceLang(line: string): string | null {
  const stripped = line.trimStart()
  if (!stripped.startsWith('```')) return null
  return stripped.replace(/^`+/, '').trim().toLowerCase()
}

/** True when the accumulated code holds an odd number of ''' or """, so a ``` line is part of a
 *  docstring rather than a closer. recursis uses tokenize; counting is the JS analogue. */
function insideTripleQuote(code: string): boolean {
  const count = (s: string, needle: string) => s.split(needle).length - 1
  return count(code, "'''") % 2 === 1 || count(code, '"""') % 2 === 1
}

export function extractFirstCodeBlock(response: string): ExtractedCode {
  const lines = response.split('\n')
  const codeLines: string[] = []
  let started = false        // inside the first executable block
  let closed = false         // first block complete
  let skippingOther = false  // inside a named non-python fence (```json …) before the first block
  let discarded = 0          // fence-pairs after the first block (counted, never executed)
  let inLater = false
  for (const line of lines) {
    const lang = fenceLang(line)
    if (!started) {
      if (lang === null) continue
      if (skippingOther) { skippingOther = false; continue }   // the skipped block's closing fence
      if (PY_LANGS.has(lang)) { started = true; continue }     // our opener
      skippingOther = true                                     // ```json / ```bash … — skip it whole
      continue
    }
    if (!closed) {
      if (lang !== null && !insideTripleQuote(codeLines.join('\n'))) { closed = true; continue }
      codeLines.push(line)
      continue
    }
    if (lang !== null) { if (!inLater) { discarded++; inLater = true } else inLater = false }
  }
  if (!started) {
    // No fence. Check for a TOOL-CALL ENVELOPE first: models in the Qwen/Hermes lineage, MiMo among them,
    // emit code as a pseudo-XML function call (`<tool_call><function=code_interpreter><parameter=code>`)
    // with a whole program inside. Without this the fallback below matched `emit_graph(` in the MIDDLE of
    // that program and ran only that call, so every name it referenced was undefined and the block died
    // with NameError. Four turns in a row on mimo-v2.5, on a program that was perfectly good.
    const envelope = extractToolCallCode(response)
    if (envelope) return { code: envelope, text: response, discarded: 0 }
    const call = extractUnfencedSpecialCall(response)
    return { code: call, text: response, discarded: 0 }
  }
  const code = codeLines.join('\n').trim()
  return { code: code.length > 0 ? code : null, text: response, discarded }
}

// The code parameter of a pseudo-XML tool call. Tolerant about the closing tag, because the reply is often
// cut at the token cap mid-envelope and the code so far is still what the model meant to run.
const TOOLCALL_CODE_RE = /<parameter\s*=\s*["']?code["']?\s*>\r?\n?([\s\S]*?)(?:<\/parameter>|<\/function>|<\/tool_call>|$)/i

function extractToolCallCode(text: string): string | null {
  const m = TOOLCALL_CODE_RE.exec(text)
  if (!m) return null
  const code = (m[1] ?? '').trim()
  return code.length > 0 ? code : null
}

const SPECIAL_CALL_RE = /(?:emit_graph|FINAL_VAR)\s*\(/

/**
 * True when this call is the TAIL OF A PROGRAM rather than a standalone statement, meaning the same text
 * assigns to a name the call passes.
 *
 * recursis only plucked unfenced SUBMIT_PLAN / ASK_QUESTION out of prose, and their arguments are
 * literals. Our port widened the pattern to emit_graph, whose arguments are VARIABLES, so a program
 * ending in emit_graph(nodes, edges) had that last line extracted and run alone: four turns in a row of
 * `NameError: name 'deduped_nodes' is not defined` on mimo-v2.5, on a program that was perfectly good.
 */
function isProgramTail(text: string, call: string): boolean {
  const args = call.slice(call.indexOf('(') + 1, call.lastIndexOf(')'))
  const names = args.match(/[A-Za-z_][A-Za-z0-9_]*/g) ?? []
  return names.some((n) => new RegExp(`^\\s*${n}\\s*(?:[+*-]?=[^=]|,)`, 'm').test(text))
}

function extractUnfencedSpecialCall(text: string): string | null {
  const m = SPECIAL_CALL_RE.exec(text)
  if (!m) return null
  const start = m.index
  let i = start + m[0].length
  let depth = 1
  let quote: string | null = null   // ' or " while inside a string; \ escapes inside it
  while (i < text.length && depth > 0) {
    const c = text[i]
    if (quote) {
      if (c === '\\') i++            // skip the escaped char
      else if (c === quote) quote = null
    } else if (c === '"' || c === "'") quote = c
    else if (c === '(' || c === '[' || c === '{') depth++
    else if (c === ')' || c === ']' || c === '}') depth--
    i++
  }
  if (depth !== 0) return null       // never balanced — not a usable call
  const call = text.slice(start, i)
  // Running the tail of a program without its body is not a saved round trip. It is a guaranteed
  // NameError that teaches the model nothing.
  return isProgramTail(text, call) ? null : call
}

/**
 * The code channel, as a TOOL rather than a formatting convention.
 *
 * Everything above is recovery, and that list only grows: a model trained to call tools answers with a
 * tool call, whatever the prompt asks for. MiMo v2.5, asked for a fence, emitted
 * `<｜DSML｜invoke name="execute_python">`, which the envelope reader did not match, and the compile fell
 * to 132 nodes where the same document had produced 736.
 *
 * This is NOT a tool we execute through a tool-runner. It is the tool channel used to receive one typed
 * value, as extraction already does with `submit_extraction`, and the code still runs in the same Pyodide
 * REPL. `codeFromReply` falls back to the text parser for providers that ignore the tool.
 */
export const EXECUTE_PYTHON_TOOL = {
  name: 'execute_python',
  description:
    'Run one block of Python in the persistent notebook session and return its stdout, stderr and any error. '
    + 'Variables defined here survive into later calls. This is the ONLY way to run code: never write code as '
    + 'prose or in a markdown fence, always call this tool.',
  input_schema: {
    type: 'object',
    properties: {
      code: {
        type: 'string',
        description: 'The Python source to run. A complete block, not a fragment; no markdown fences around it.',
      },
    },
    required: ['code'],
  },
} as const

/**
 * The code the model wants run this turn: the tool call if it made one, otherwise whatever the text parser
 * recovers. Tool first, because when both are present the tool call is the one the model chose.
 */
export function codeFromReply(toolInput: unknown, text: string): ExtractedCode {
  const viaTool = toolInput !== null && typeof toolInput === 'object'
    ? (toolInput as Record<string, unknown>).code
    : undefined
  if (typeof viaTool === 'string' && viaTool.trim()) {
    return { code: viaTool.trim(), text, discarded: 0 }
  }
  return extractFirstCodeBlock(text)
}
