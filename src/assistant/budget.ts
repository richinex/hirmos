import { z } from 'zod'
import type { StampedTrace } from './traceCells'

/** The limits a question started with, shown until the agent's first budget snapshot arrives. */
export interface TurnLimits {
  readonly maxSteps: number
  readonly maxRequests: number
  readonly deadlineMs: number
}

/**
 * What the provider reported for the question's requests. A request that ended without a reported cost,
 * such as a failed or stopped one, is missing rather than zero.
 */
export type TurnCost =
  | { readonly kind: 'exact'; readonly usd: number }
  | {
      readonly kind: 'partial'
      readonly usd: number
      readonly found: number
      readonly requests: number
    }
  | { readonly kind: 'unavailable' }

/** The key's own spending limit, in OpenRouter credits. It is not the account balance, which a normal key cannot read. */
export type CreditStatus =
  | { readonly kind: 'unchecked' }
  | { readonly kind: 'checking' }
  | {
      readonly kind: 'known'
      /** Null when the key has no limit of its own. */
      readonly keyLimit: number | null
      readonly keyRemaining: number | null
    }
  | { readonly kind: 'failed'; readonly message: string }

const PARENT = 'Assistant'

type BudgetEvent = Extract<StampedTrace, { readonly kind: 'budget' }>

const parentBudget = (budgets: readonly BudgetEvent[]): BudgetEvent | undefined =>
  budgets.filter((event) => event.agent === PARENT).at(-1)

/** The parent's step limit from its latest snapshot, or the limit the question started with. */
export function stepLimit(trace: readonly StampedTrace[], fallback: number): number {
  const budgets = trace.filter((event): event is BudgetEvent => event.kind === 'budget')
  return parentBudget(budgets)?.budget.maxSteps ?? fallback
}

export function turnCost(trace: readonly StampedTrace[]): TurnCost | null {
  const requests = new Set<number>()
  const costs = new Map<number, number>()
  for (const event of trace) {
    if (event.kind === 'budget') requests.add(event.requestId)
    if (event.kind === 'usage' && event.usage.costUsd !== null)
      costs.set(event.requestId, event.usage.costUsd)
  }
  if (requests.size === 0) return null
  if (costs.size === 0) return { kind: 'unavailable' }
  const usd = [...costs.values()].reduce((sum, cost) => sum + cost, 0)
  return costs.size === requests.size
    ? { kind: 'exact', usd }
    : { kind: 'partial', usd, found: costs.size, requests: requests.size }
}

const keyResponse = z.object({
  data: z.object({ limit: z.number().nullable(), limit_remaining: z.number().nullable() }),
})

/** Reads the key's own limit from OpenRouter. The key goes only to OpenRouter, as it does for model requests. */
export async function readKeyCredit(apiKey: string, signal: AbortSignal): Promise<CreditStatus> {
  try {
    const response = await fetch('https://openrouter.ai/api/v1/key', {
      headers: { Authorization: `Bearer ${apiKey}` },
      signal,
      cache: 'no-store',
    })
    if (!response.ok)
      return {
        kind: 'failed',
        message:
          response.status === 401
            ? 'OpenRouter did not accept this API key.'
            : `OpenRouter returned ${response.status} for the key check.`,
      }
    const parsed = keyResponse.safeParse(await response.json())
    if (!parsed.success)
      return { kind: 'failed', message: 'OpenRouter returned an unexpected key record.' }
    return {
      kind: 'known',
      keyLimit: parsed.data.data.limit,
      keyRemaining: parsed.data.data.limit_remaining,
    }
  } catch {
    return { kind: 'failed', message: 'The key check could not reach OpenRouter.' }
  }
}

/** The share of a budget at which the meter turns to the warning colour. */
export const WARN_AT = 0.75

export const usd = (value: number): string =>
  value < 1 ? `$${value.toFixed(4)}` : `$${value.toFixed(2)}`

export const credits = (value: number): string => value.toFixed(2)

export const clock = (ms: number): string => {
  const total = Math.max(0, Math.round(ms / 1000))
  return `${Math.floor(total / 60)}:${String(total % 60).padStart(2, '0')}`
}

export function costText(cost: TurnCost): string {
  switch (cost.kind) {
    case 'exact':
      return usd(cost.usd)
    case 'partial':
      return `${usd(cost.usd)} for ${cost.found} of ${cost.requests} requests`
    case 'unavailable':
      return 'cost unavailable'
  }
}
