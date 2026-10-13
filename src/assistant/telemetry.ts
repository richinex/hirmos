import { z } from 'zod'

export interface BudgetProgress {
  readonly step: number
  readonly maxSteps: number
  readonly requests: number
  readonly maxRequests: number
  readonly elapsedMs: number
  readonly deadlineMs: number
  readonly outputTokenAllowance: number
}

export function budgetWarning(budget: BudgetProgress): string | null {
  const steps = budget.maxSteps - budget.step
  const requests = budget.maxRequests - budget.requests
  const seconds = Math.max(0, Math.floor((budget.deadlineMs - budget.elapsedMs) / 1000))
  if (steps > 0 && requests > 1 && seconds > 45) return null
  return `Execution budget: this is step ${budget.step} of ${budget.maxSteps}; ${steps} later steps and ${requests} shared requests remain after this request, with ${seconds} seconds until the deadline. Finish in this response using FINAL_VAR or finish. Report what is verified, what is incomplete, and what remains unknown. Do not start a new analysis or delegate. A partial answer must be labelled as partial; do not claim unfinished work succeeded.`
}

export interface ProviderUsage {
  readonly generationId: string | null
  readonly kind: 'reported' | 'unavailable'
  readonly reason: 'early-stop' | 'missing-usage' | null
  readonly inputTokens: number | null
  readonly outputTokens: number | null
  readonly reasoningTokens: number | null
  readonly cachedTokens: number | null
  readonly costUsd: number | null
}

const usageSchema = z.object({
  promptTokens: z.number().nonnegative().optional(),
  completionTokens: z.number().nonnegative().optional(),
  completionTokensDetails: z
    .object({ reasoningTokens: z.number().nonnegative().optional() })
    .optional(),
  promptTokensDetails: z.object({ cachedTokens: z.number().nonnegative().optional() }).optional(),
  cost: z.number().nonnegative().optional(),
})
const metadataSchema = z.object({ openrouter: z.object({ usage: usageSchema }) })

export function providerUsage(
  metadata: unknown,
  earlyStop: boolean,
  generationId: string | null = null,
): ProviderUsage {
  const parsed = metadataSchema.safeParse(metadata)
  const usage = parsed.success ? parsed.data.openrouter.usage : null
  const reported = usage !== null && Object.values(usage).some((value) => value !== undefined)
  return {
    generationId,
    kind: reported ? 'reported' : 'unavailable',
    reason: reported ? null : earlyStop ? 'early-stop' : 'missing-usage',
    inputTokens: usage?.promptTokens ?? null,
    outputTokens: usage?.completionTokens ?? null,
    reasoningTokens: usage?.completionTokensDetails?.reasoningTokens ?? null,
    cachedTokens: usage?.promptTokensDetails?.cachedTokens ?? null,
    costUsd: usage?.cost ?? null,
  }
}

export interface ProviderFailure {
  readonly status: number | null
  readonly limitSource: string | null
  readonly resetAt: string | null
  readonly retryAfterSeconds: number | null
  readonly message: string
}
const failureSchema = z.object({
  code: z.number().optional(),
  metadata: z
    .object({ limit_source: z.string().optional(), resets_at: z.string().nullable().optional() })
    .optional(),
})

export function providerFailure(
  status: number | undefined,
  body: unknown,
  retryAfter: string | undefined,
  now = Date.now(),
): ProviderFailure {
  const envelope = z.object({ error: failureSchema }).safeParse(body)
  const direct = failureSchema.safeParse(body)
  const parsed = envelope.success ? envelope.data.error : direct.success ? direct.data : null
  const code = status ?? parsed?.code ?? null
  const source = parsed?.metadata?.limit_source ?? null
  const seconds =
    retryAfter === undefined
      ? NaN
      : /^\d+(\.\d+)?$/.test(retryAfter)
        ? Number(retryAfter)
        : (Date.parse(retryAfter) - now) / 1000
  const retryAfterSeconds = Number.isFinite(seconds) ? Math.max(0, Math.ceil(seconds)) : null
  const message =
    source === 'openrouter_credits'
      ? 'OpenRouter credit is used up.'
      : source === 'openrouter_key_limit'
        ? 'This OpenRouter key has reached its spending limit.'
        : source === 'openrouter_in_flight_budget'
          ? 'OpenRouter’s in-flight spending limit was reached.'
          : source === 'openrouter_guardrail_budget'
            ? 'The OpenRouter guardrail budget was reached.'
            : source === 'openrouter_workspace_budget'
              ? 'The OpenRouter workspace budget was reached.'
              : code === 429
                ? 'OpenRouter rate limited this request.'
                : code === 402
                  ? 'OpenRouter refused the request because of a credit or spending limit.'
                  : code === 401
                    ? 'OpenRouter did not accept this API key.'
                    : code === 403
                      ? 'OpenRouter denied this request.'
                      : 'The model provider could not complete this request.'
  return {
    status: code,
    limitSource: source,
    resetAt: parsed?.metadata?.resets_at ?? null,
    retryAfterSeconds,
    message:
      message + (retryAfterSeconds === null ? '' : ` Retry after ${retryAfterSeconds} seconds.`),
  }
}
