import { APICallError, streamText, type ModelMessage } from 'ai'
import {
  providerFailure,
  providerUsage,
  type ProviderFailure,
  type ProviderUsage,
} from './telemetry'
import { createOpenRouter } from '@openrouter/ai-sdk-provider'
import { z } from 'zod'
import type { CodeActLoopOptions, CodeActMessage } from './octopus/rlm/codeactPorts'
import { extractFirstCodeBlock } from './octopus/rlm/parseCode'

const contentBlock = z.discriminatedUnion('type', [
  z.object({ type: z.literal('text'), text: z.string() }),
  z.object({ type: z.literal('reasoning'), text: z.string() }),
])

// From Octopus's packages/core/src/anthropic.ts; keep the same fence and Gemini rules.
const FENCE_OPEN = /```(?:python|py)[ \t]*\n/i
const FENCE_CLOSE = /\n```[ \t]*(?:\n|$)/
export function firstCodeFenceClosed(text: string): boolean {
  const open = FENCE_OPEN.exec(text)
  return open !== null && FENCE_CLOSE.test(text.slice(open.index + open[0].length))
}

function modelMessages(messages: readonly CodeActMessage[]): ModelMessage[] {
  return messages.map((message): ModelMessage => {
    if (typeof message.content === 'string') return { role: message.role, content: message.content }
    if (message.role === 'assistant')
      return { role: 'assistant', content: z.array(contentBlock).parse(message.content) }
    throw new Error('A CodeAct user message must contain text.')
  })
}

export function openRouterModel(
  key: string,
  model: string,
  claimRequest: () => void,
  reportUnusedReply?: (reason: string) => void,
  reporting?: {
    readonly usage: (usage: ProviderUsage) => void
    readonly failure: (failure: ProviderFailure) => void
  },
): CodeActLoopOptions['callModel'] {
  const provider = createOpenRouter({ apiKey: key })
  return async (request) => {
    request.signal?.throwIfAborted()
    claimRequest()
    const controller = new AbortController()
    const signal = request.signal
      ? AbortSignal.any([request.signal, controller.signal])
      : controller.signal
    let text = ''
    let reasoning = ''
    let cut = false
    let streamError: unknown = null
    let generationId: string | null = null
    const failed = (error: unknown) => {
      let body: unknown = error
      if (APICallError.isInstance(error) && error.responseBody) {
        try {
          body = JSON.parse(error.responseBody)
        } catch {
          body = null
        }
      }
      const failure = providerFailure(
        APICallError.isInstance(error) ? error.statusCode : undefined,
        body,
        APICallError.isInstance(error) ? error.responseHeaders?.['retry-after'] : undefined,
      )
      reporting?.failure(failure)
      return new Error(failure.message)
    }
    const response = streamText({
      model: provider(model, /^anthropic\//.test(model) ? {} : { reasoning: { effort: 'none' } }),
      system: request.system,
      messages: modelMessages(request.messages),
      maxOutputTokens: request.maxTokens,
      includeRawChunks: true,
      ...(!/gemini/i.test(model) ? { stopSequences: ['\n```\n'] } : {}),
      maxRetries: 0,
      timeout: 120_000,
      abortSignal: signal,
      onError: ({ error }) => {
        if (!cut && !request.signal?.aborted) streamError = error
      },
    })
    try {
      for await (const part of response.fullStream) {
        if (part.type === 'raw') {
          const chunk = z.object({ id: z.string().min(1) }).safeParse(part.rawValue)
          if (chunk.success) generationId = chunk.data.id
          continue
        }
        if (part.type === 'error') throw part.error
        if (part.type === 'reasoning-delta') {
          reasoning += part.text
          continue
        }
        if (part.type !== 'text-delta') continue
        text += part.text
        request.onText(text)
        if (!/gemini/i.test(model) && firstCodeFenceClosed(text)) {
          cut = true
          controller.abort(
            new DOMException('CodeAct received its first complete Python block.', 'AbortError'),
          )
          void Promise.resolve(response.consumeStream({ onError: () => undefined })).catch(
            () => undefined,
          )
          break
        }
      }
    } catch (error) {
      request.signal?.throwIfAborted()
      if (!cut) throw failed(error)
    }
    request.signal?.throwIfAborted()
    if (streamError && !cut) throw failed(streamError)
    const finish = cut ? 'stop' : await response.finishReason
    const usage = cut ? undefined : await response.usage
    reporting?.usage(
      providerUsage(cut ? undefined : await response.providerMetadata, cut, generationId),
    )
    if (finish === 'length')
      reportUnusedReply?.(
        `Reply reached the output-token limit; no code was executed from this reply. Received ${text.length} visible characters and ${reasoning.length} reasoning characters.`,
      )
    else if (!extractFirstCodeBlock(text.trim()).code)
      reportUnusedReply?.(
        text.trim() === ''
          ? 'The model returned no visible text; no code was executed.'
          : 'The reply contained no Python block; no code was executed.',
      )
    return {
      text,
      content: [
        ...(reasoning ? [{ type: 'reasoning', text: reasoning }] : []),
        { type: 'text', text },
      ],
      stop_reason: finish === 'length' ? 'max_tokens' : 'end_turn',
      ...(usage?.outputTokens !== undefined
        ? { usage: { output_tokens: usage.outputTokens } }
        : {}),
    }
  }
}
