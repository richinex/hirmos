import { z } from 'zod'

export const DEFAULT_LIMITS = { maxIters: 8, maxRequests: 24, maxTokens: 16384 } as const
export const assistantLimits = z
  .object({
    maxIters: z.number().int().min(1).max(24),
    maxRequests: z.number().int().min(1).max(120),
    maxTokens: z.number().int().min(1).max(64000).default(DEFAULT_LIMITS.maxTokens),
  })
  .refine((value) => value.maxRequests >= value.maxIters, {
    message: 'The shared request limit must be at least the maximum steps per agent.',
  })
export type AssistantLimits = z.infer<typeof assistantLimits>

/** How long one question may run, parent and subagents together. */
export const QUESTION_DEADLINE_MS = 240_000
